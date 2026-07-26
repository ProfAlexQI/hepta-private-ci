use super::*;

impl RuntimeKernel {
    pub fn reconcile_pending_outcome(
        &self,
        attempt_id: &str,
    ) -> Result<OutcomeRecordResult, HeptaError> {
        let attempt_id = attempt_id.trim();
        if attempt_id.is_empty() {
            return Err(HeptaError(
                "pending outcome attempt id must not be empty".into(),
            ));
        }
        let local_pending = {
            let mut state = self
                .execution_outcome_state
                .lock()
                .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))?;
            state.breaker.begin_reconciliation(attempt_id)?
        };
        let execution_intent = self
            .outcome_sink
            .pending_execution_intent(attempt_id)
            .map_err(|error| {
                HeptaError(format!(
                    "failed to inspect execution intent {attempt_id}: {error}"
                ))
            })?;
        let (exact, _kind) = match local_pending {
            Some(pending) => pending,
            None => {
                let pending = self
                    .outcome_sink
                    .pending_intent(attempt_id)
                    .map_err(|error| {
                        HeptaError(format!(
                            "failed to recover pending outcome intent {attempt_id}: {error}"
                        ))
                    })?;
                if let Some(pending) = pending {
                    (pending.exact, pending.kind)
                } else if execution_intent.is_some() {
                    let record = self
                        .outcome_sink
                        .read_by_attempt(attempt_id)
                        .map_err(|error| {
                            HeptaError(format!(
                                "failed to recover committed outcome {attempt_id}: {error}"
                            ))
                        })?
                        .ok_or_else(|| {
                            HeptaError(format!(
                                "execution attempt {attempt_id} is in doubt without exact terminal material"
                            ))
                        })?;
                    (
                        ExactOutcomeRecord::from_record(&record),
                        PendingOutcomeKind::CommitAmbiguous,
                    )
                } else {
                    return Err(HeptaError(format!(
                        "no retryable outcome is pending for attempt {attempt_id}"
                    )));
                }
            }
        };
        let replay = match execution_intent.as_ref() {
            Some(intent) => self
                .outcome_sink
                .record_and_resolve_execution(&exact, intent),
            None => self.outcome_sink.record(&exact),
        };
        let result = match replay {
            Ok(OutcomeRecordResult::Recorded) => OutcomeRecordResult::Recorded,
            Ok(OutcomeRecordResult::AlreadyRecorded) => OutcomeRecordResult::AlreadyRecorded,
            Err(error) if error.pending_kind().is_some() => {
                let mut state = self
                    .execution_outcome_state
                    .lock()
                    .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))?;
                if let Err(reason) = state.breaker.retain_pending(exact, error.clone()) {
                    state.breaker.trip_fatal(reason);
                }
                return Err(HeptaError(format!(
                    "outcome reconciliation remains pending: {error}"
                )));
            }
            Err(error) => {
                let reason = format!("outcome reconciliation failed: {error}");
                match self.execution_outcome_state.lock() {
                    Ok(mut state) => {
                        state.breaker.finish_nonretryable_failure(&exact);
                        state.breaker.trip_fatal(reason.clone());
                    }
                    Err(poisoned) => {
                        let mut state = poisoned.into_inner();
                        state.breaker.finish_nonretryable_failure(&exact);
                        state.breaker.trip_fatal(reason.clone());
                    }
                }
                return Err(HeptaError(reason));
            }
        };
        let mut state = self
            .execution_outcome_state
            .lock()
            .map_err(|_| HeptaError("execution outcome state mutex poisoned".into()))?;
        if state.breaker.resolve(&exact) {
            state.active_attempts.remove(attempt_id);
            state.finalized_attempts = state.finalized_attempts.saturating_add(1);
        }
        Ok(result)
    }

    pub(in crate::runtime_kernel) fn durable_pending_outcome_reason(
        &self,
    ) -> Result<Option<String>, HeptaError> {
        let execution_intent = self
            .outcome_sink
            .pending_execution_intents()
            .map_err(|error| {
                HeptaError(format!(
                    "durable execution-intent inspection failed closed: {error}"
                ))
            })?
            .into_iter()
            .next();
        if let Some(intent) = execution_intent {
            return Ok(Some(format!(
                "durable execution attempt {} is in doubt before terminal resolution",
                intent.attempt_id()
            )));
        }
        self.outcome_sink
            .first_pending_intent()
            .map(|pending| {
                pending.map(|pending| {
                    format!(
                        "durable outcome attempt {} requires exact {} from the producer-intent journal",
                        pending.exact.attempt_id(),
                        match pending.kind {
                            PendingOutcomeKind::SafeRetry => "retry",
                            PendingOutcomeKind::CommitAmbiguous => "reconciliation",
                        }
                    )
                })
            })
            .map_err(|error| {
                HeptaError(format!(
                    "durable outcome intent inspection failed closed: {error}"
                ))
            })
    }

    pub fn pending_execution_intents(&self) -> Result<Vec<ExecutionIntent>, HeptaError> {
        self.outcome_sink
            .pending_execution_intents()
            .map_err(|error| {
                HeptaError(format!(
                    "failed to enumerate pending execution intents: {error}"
                ))
            })
    }

    pub fn pending_execution_effect_inspections(
        &self,
    ) -> Result<Vec<crate::PendingExecutionEffectInspection>, HeptaError> {
        self.outcome_sink
            .pending_execution_intents()
            .map_err(|error| {
                HeptaError(format!(
                    "failed to enumerate pending execution intents: {error}"
                ))
            })?
            .into_iter()
            .map(|intent| {
                let ack = self
                    .outcome_sink
                    .execution_effect_ack(intent.attempt_id())
                    .map_err(|error| {
                        HeptaError(format!(
                            "failed to inspect provider effect ACK {}: {error}",
                            intent.attempt_id()
                        ))
                    })?;
                super::super::provider_effect::inspect_pending_effect(&intent, ack.as_ref())
            })
            .collect()
    }

    pub fn reconcile_ambiguous_outcome(
        &self,
        attempt_id: &str,
    ) -> Result<OutcomeRecordResult, HeptaError> {
        self.reconcile_pending_outcome(attempt_id)
    }
}
