use super::*;

impl RuntimeKernel {
    pub(crate) fn ensure_outcome_dispatch_open(&self) -> Result<(), HeptaError> {
        let local_reason = self
            .execution_outcome_state
            .lock()
            .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))?
            .breaker
            .reason();
        if let Some(reason) = local_reason {
            return Err(HeptaError(format!(
                "outcome receipt breaker is open: {reason}"
            )));
        }
        if let Some(reason) = self.durable_pending_outcome_reason()? {
            return Err(HeptaError(format!(
                "outcome receipt breaker is open: {reason}"
            )));
        }
        Ok(())
    }

    pub(super) fn reserve_execution_attempt(
        &self,
    ) -> Result<ExecutionAttemptReservation, HeptaError> {
        self.ensure_outcome_dispatch_open()?;
        for _ in 0..ATTEMPT_ID_RETRY_LIMIT {
            let attempt_id = Uuid::new_v4().to_string();
            {
                let mut state = self
                    .execution_outcome_state
                    .lock()
                    .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))?;
                if let Some(reason) = state.breaker.reason() {
                    return Err(HeptaError(format!(
                        "outcome receipt breaker is open: {reason}"
                    )));
                }
                if !state.active_attempts.insert(attempt_id.clone()) {
                    continue;
                }
            }

            match self.outcome_sink.read_by_attempt(&attempt_id) {
                Ok(None) => {
                    return Ok(ExecutionAttemptReservation {
                        state: Arc::clone(&self.execution_outcome_state),
                        attempt_id,
                        active: true,
                    });
                }
                Ok(Some(_)) => {
                    self.release_attempt_id(&attempt_id);
                    continue;
                }
                Err(error) => {
                    let reason = format!("outcome store read failed before dispatch: {error}");
                    self.release_attempt_id(&attempt_id);
                    self.trip_outcome_breaker(reason.clone());
                    return Err(HeptaError(reason));
                }
            }
        }

        let reason = "could not allocate a unique execution attempt identity".to_string();
        self.trip_outcome_breaker(reason.clone());
        Err(HeptaError(reason))
    }

    fn release_attempt_id(&self, attempt_id: &str) {
        match self.execution_outcome_state.lock() {
            Ok(mut state) => {
                state.active_attempts.remove(attempt_id);
            }
            Err(poisoned) => {
                poisoned.into_inner().active_attempts.remove(attempt_id);
            }
        }
    }

    pub fn terminal_receipt_recorded(&self, attempt_id: &str) -> Result<bool, HeptaError> {
        let attempt_id = attempt_id.trim();
        if attempt_id.is_empty() {
            return Err(HeptaError(
                "terminal receipt attempt id must not be empty".into(),
            ));
        }
        self.outcome_sink
            .read_by_attempt(attempt_id)
            .map(|record| record.is_some())
            .map_err(|error| HeptaError(format!("terminal receipt readback failed: {error}")))
    }

    pub fn execution_receipt_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<crate::RuntimeExecutionReceipt>, HeptaError> {
        let attempt_id = attempt_id.trim();
        if attempt_id.is_empty() {
            return Err(HeptaError(
                "execution receipt attempt id must not be empty".into(),
            ));
        }
        let Some(terminal) = self
            .outcome_sink
            .read_by_attempt(attempt_id)
            .map_err(|error| HeptaError(format!("execution receipt readback failed: {error}")))?
        else {
            return Ok(None);
        };
        let effect_ack = self
            .outcome_sink
            .execution_effect_ack(attempt_id)
            .map_err(|error| {
                HeptaError(format!(
                    "execution effect ACK readback failed for {attempt_id}: {error}"
                ))
            })?;
        let terminal_status = match terminal.receipt().status() {
            hepta_contracts::OutcomeStatus::Succeeded => "succeeded".to_string(),
            hepta_contracts::OutcomeStatus::Failed { error_code } => {
                format!("failed:{error_code}")
            }
            hepta_contracts::OutcomeStatus::Cancelled { reason_code } => {
                format!("cancelled:{reason_code}")
            }
            _ => "unknown".to_string(),
        };
        Ok(Some(crate::RuntimeExecutionReceipt {
            attempt_id: attempt_id.to_string(),
            durable_intent_recorded: true,
            effect_plan_recorded: effect_ack.is_some(),
            effect_plan_hash: effect_ack
                .as_ref()
                .map(hepta_memory::ExecutionEffectAck::effect_plan_hash)
                .map(ToString::to_string),
            provider_effect_ack_hash: effect_ack
                .as_ref()
                .map(hepta_memory::ExecutionEffectAck::ack_hash)
                .map(ToString::to_string),
            terminal_receipt_id: terminal.receipt().id().to_string(),
            terminal_receipt_hash: terminal.receipt().receipt_hash().to_string(),
            terminal_outcome_hash: terminal.receipt().outcome_hash().to_string(),
            terminal_evidence_hash: terminal.canonical_evidence_hash().to_string(),
            terminal_status,
        }))
    }

    #[cfg(test)]
    pub(crate) fn outcome_record_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, HeptaError> {
        self.outcome_sink
            .read_by_attempt(attempt_id)
            .map_err(|error| HeptaError(format!("outcome sink read failed: {error}")))
    }

    #[cfg(test)]
    pub(crate) fn outcome_record_count(&self) -> Result<u64, HeptaError> {
        self.execution_outcome_state
            .lock()
            .map(|state| state.finalized_attempts)
            .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))
    }

    pub(crate) fn trip_outcome_breaker(&self, reason: impl Into<String>) {
        let reason = reason.into();
        match self.execution_outcome_state.lock() {
            Ok(mut state) => state.breaker.trip_fatal(reason),
            Err(poisoned) => poisoned.into_inner().breaker.trip_fatal(reason),
        }
    }
}
