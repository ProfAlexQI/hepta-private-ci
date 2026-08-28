impl IntelligenceMutationState {
    fn validate_business_invariants(&self) -> Result<(), IntelligenceMutationStateError> {
        if self.memory_write_count > 1 {
            return Err(IntelligenceMutationStateError::Corrupt(
                "one mutation wrote memory facts more than once".to_string(),
            ));
        }
        if self.projection_publish_count > 1 {
            return Err(IntelligenceMutationStateError::Corrupt(
                "one mutation published a projection more than once".to_string(),
            ));
        }
        match self.projection_publish_count {
            0 if self.last_published_generation
                != self.binding.starting_projection_generation =>
            {
                return Err(IntelligenceMutationStateError::Corrupt(
                    "projection generation changed without publication".to_string(),
                ));
            }
            1 => {
                let expected = self
                    .binding
                    .starting_projection_generation
                    .checked_add(1)
                    .ok_or(IntelligenceMutationStateError::ProjectionGenerationOverflow)?;
                if self.last_published_generation != expected {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "projection publication did not advance exactly once".to_string(),
                    ));
                }
            }
            _ => {}
        }

        if self.reconciliation_count == 0 && self.last_recovery_origin.is_some() {
            return Err(IntelligenceMutationStateError::Corrupt(
                "recovery origin exists without a reconciliation".to_string(),
            ));
        }
        if self.reconciliation_count > 0 && self.last_recovery_origin.is_none() {
            return Err(IntelligenceMutationStateError::Corrupt(
                "reconciliation count has no recovery origin".to_string(),
            ));
        }

        use IntelligenceMutationIntentDisposition as Disposition;
        use IntelligenceMutationPhase as Phase;
        match self.phase {
            Phase::Planned | Phase::SourceWitnessed | Phase::GroundingValidated => {
                self.require_summary(Disposition::None, 0, 0, false)?;
                if self.indeterminate_from.is_some() {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "pre-intent phase retains an indeterminate origin".to_string(),
                    ));
                }
            }
            Phase::RejectedPreCommit | Phase::CancelledPreCommit => {
                self.require_summary(Disposition::None, 0, 0, false)?;
                if self.indeterminate_from.is_some() {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "pre-commit terminal phase retains an indeterminate origin"
                            .to_string(),
                    ));
                }
            }
            Phase::DurableIntentAppended => {
                self.require_summary(Disposition::Pending, 0, 0, false)?;
                if self.indeterminate_from.is_some() {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "resumed durable-intent phase still marked indeterminate".to_string(),
                    ));
                }
            }
            Phase::MemoryFactsCommitted => {
                self.require_summary(Disposition::Pending, 1, 0, false)?;
                if self.indeterminate_from.is_some() {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "resumed memory phase still marked indeterminate".to_string(),
                    ));
                }
            }
            Phase::ProjectionPublished => {
                self.require_summary(Disposition::Pending, 1, 1, false)?;
                if self.indeterminate_from.is_some() {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "resumed projection phase still marked indeterminate".to_string(),
                    ));
                }
            }
            Phase::OutboxSettled | Phase::Terminal => {
                self.require_summary(Disposition::SettledApplied, 1, 1, true)?;
                if self.indeterminate_from.is_some() {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "settled phase still marked indeterminate".to_string(),
                    ));
                }
            }
            Phase::ReconciledNotApplied => {
                self.require_summary(Disposition::SettledNotApplied, 0, 0, false)?;
                if self.indeterminate_from.is_some() {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "not-applied terminal phase still marked indeterminate".to_string(),
                    ));
                }
            }
            Phase::Quarantined => {
                if self.intent_disposition != Disposition::Quarantined
                    || self.indeterminate_from.is_some()
                    || self.last_recovery_origin.and_then(Phase::durable_rank).is_none()
                    || self.reconciliation_count == 0
                {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "quarantine disposition or origin is inconsistent".to_string(),
                    ));
                }
            }
            Phase::Indeterminate => self.validate_indeterminate_summary()?,
        }
        Ok(())
    }

    fn validate_indeterminate_summary(&self) -> Result<(), IntelligenceMutationStateError> {
        use IntelligenceMutationIntentDisposition as Disposition;
        use IntelligenceMutationPhase as Phase;
        let origin = self.indeterminate_from.ok_or_else(|| {
            IntelligenceMutationStateError::Corrupt(
                "indeterminate state has no durable origin".to_string(),
            )
        })?;
        match origin {
            Phase::DurableIntentAppended => {
                self.require_summary(Disposition::Pending, 0, 0, false)
            }
            Phase::MemoryFactsCommitted => {
                self.require_summary(Disposition::Pending, 1, 0, false)
            }
            Phase::ProjectionPublished => {
                self.require_summary(Disposition::Pending, 1, 1, false)
            }
            Phase::OutboxSettled => {
                self.require_summary(Disposition::SettledApplied, 1, 1, true)
            }
            _ => Err(IntelligenceMutationStateError::Corrupt(
                "indeterminate origin is not a durable mutation phase".to_string(),
            )),
        }
    }

    fn require_summary(
        &self,
        disposition: IntelligenceMutationIntentDisposition,
        memory_write_count: u8,
        projection_publish_count: u8,
        outbox_settled: bool,
    ) -> Result<(), IntelligenceMutationStateError> {
        if self.intent_disposition != disposition
            || self.memory_write_count != memory_write_count
            || self.projection_publish_count != projection_publish_count
            || self.outbox_settled != outbox_settled
        {
            return Err(IntelligenceMutationStateError::Corrupt(format!(
                "phase summary mismatch: expected disposition={}, writes={memory_write_count}, \
                 projections={projection_publish_count}, outbox_settled={outbox_settled}",
                disposition.as_str()
            )));
        }
        Ok(())
    }

}
