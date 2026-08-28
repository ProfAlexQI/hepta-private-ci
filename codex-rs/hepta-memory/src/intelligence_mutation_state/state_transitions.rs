impl IntelligenceMutationState {
    fn apply_action(
        &mut self,
        action: &IntelligenceMutationAction,
    ) -> Result<IntelligenceMutationPhase, IntelligenceMutationStateError> {
        use IntelligenceMutationAction as Action;
        use IntelligenceMutationIntentDisposition as Disposition;
        use IntelligenceMutationPhase as Phase;

        match (self.phase, action) {
            (Phase::Planned, Action::WitnessSource { .. }) => Ok(Phase::SourceWitnessed),
            (Phase::SourceWitnessed, Action::ValidateGrounding { .. }) => {
                Ok(Phase::GroundingValidated)
            }
            (
                Phase::Planned | Phase::SourceWitnessed | Phase::GroundingValidated,
                Action::RejectPreCommit { .. },
            ) => Ok(Phase::RejectedPreCommit),
            (
                Phase::Planned | Phase::SourceWitnessed | Phase::GroundingValidated,
                Action::CancelPreCommit { .. },
            ) => Ok(Phase::CancelledPreCommit),
            (Phase::GroundingValidated, Action::AppendDurableIntent { .. }) => {
                self.intent_disposition = Disposition::Pending;
                Ok(Phase::DurableIntentAppended)
            }
            (Phase::DurableIntentAppended, Action::CommitMemoryFacts { .. }) => {
                self.memory_write_count = self
                    .memory_write_count
                    .checked_add(1)
                    .ok_or(IntelligenceMutationStateError::DoubleWrite)?;
                if self.memory_write_count != 1 {
                    return Err(IntelligenceMutationStateError::DoubleWrite);
                }
                Ok(Phase::MemoryFactsCommitted)
            }
            (
                Phase::MemoryFactsCommitted,
                Action::PublishProjection {
                    expected_previous_generation,
                    new_generation,
                    ..
                },
            ) => {
                self.publish_projection(
                    *expected_previous_generation,
                    *new_generation,
                )?;
                Ok(Phase::ProjectionPublished)
            }
            (Phase::ProjectionPublished, Action::SettleOutbox { .. }) => {
                self.outbox_settled = true;
                self.intent_disposition = Disposition::SettledApplied;
                Ok(Phase::OutboxSettled)
            }
            (Phase::OutboxSettled, Action::Terminalize) => Ok(Phase::Terminal),
            (
                origin @ (Phase::DurableIntentAppended
                | Phase::MemoryFactsCommitted
                | Phase::ProjectionPublished
                | Phase::OutboxSettled),
                Action::MarkIndeterminate { .. },
            ) => {
                self.indeterminate_from = Some(origin);
                Ok(Phase::Indeterminate)
            }
            (Phase::Indeterminate, Action::Reconcile { observation }) => {
                self.apply_reconciliation(observation)
            }
            (Phase::Indeterminate, Action::Quarantine { .. }) => {
                let origin = self.indeterminate_from.ok_or_else(|| {
                    IntelligenceMutationStateError::Corrupt(
                        "quarantine has no indeterminate origin".to_string(),
                    )
                })?;
                self.last_recovery_origin = Some(origin);
                self.indeterminate_from = None;
                self.intent_disposition = Disposition::Quarantined;
                self.reconciliation_count = self
                    .reconciliation_count
                    .checked_add(1)
                    .ok_or(IntelligenceMutationStateError::ReconciliationOverflow)?;
                Ok(Phase::Quarantined)
            }
            (phase, action) => Err(IntelligenceMutationStateError::InvalidTransition {
                phase,
                action: action.kind().to_string(),
            }),
        }
    }

    fn apply_reconciliation(
        &mut self,
        observation: &IntelligenceMutationReconciliationObservation,
    ) -> Result<IntelligenceMutationPhase, IntelligenceMutationStateError> {
        use IntelligenceMutationIntentDisposition as Disposition;
        use IntelligenceMutationPhase as Phase;
        use IntelligenceMutationReconciliationObservation as Observation;

        let origin = self.indeterminate_from.ok_or_else(|| {
            IntelligenceMutationStateError::Corrupt(
                "reconciliation has no indeterminate origin".to_string(),
            )
        })?;
        let origin_rank = origin.durable_rank().ok_or_else(|| {
            IntelligenceMutationStateError::Corrupt(
                "reconciliation origin is not durable".to_string(),
            )
        })?;
        if let Some(observed_rank) = observation.observed_rank() {
            if observed_rank < origin_rank {
                return Err(IntelligenceMutationStateError::ReconciliationRegression {
                    origin,
                    observation: observation.kind(),
                });
            }
        }

        let next_phase = match observation {
            Observation::NotApplied { .. } => {
                if origin != Phase::DurableIntentAppended
                    || self.memory_write_count != 0
                    || self.projection_publish_count != 0
                    || self.outbox_settled
                {
                    return Err(IntelligenceMutationStateError::InvalidReconciliation(
                        "not-applied is valid only when no memory write was observed"
                            .to_string(),
                    ));
                }
                self.intent_disposition = Disposition::SettledNotApplied;
                Phase::ReconciledNotApplied
            }
            Observation::MemoryFactsCommitted { .. } => {
                if self.projection_publish_count != 0 || self.outbox_settled {
                    return Err(IntelligenceMutationStateError::ReconciliationRegression {
                        origin,
                        observation: observation.kind(),
                    });
                }
                self.memory_write_count = 1;
                self.intent_disposition = Disposition::Pending;
                Phase::MemoryFactsCommitted
            }
            Observation::ProjectionPublished {
                expected_previous_generation,
                new_generation,
                ..
            } => {
                self.memory_write_count = 1;
                self.reconcile_projection(
                    *expected_previous_generation,
                    *new_generation,
                )?;
                self.outbox_settled = false;
                self.intent_disposition = Disposition::Pending;
                Phase::ProjectionPublished
            }
            Observation::OutboxSettled {
                expected_previous_generation,
                new_generation,
                ..
            } => {
                self.memory_write_count = 1;
                self.reconcile_projection(
                    *expected_previous_generation,
                    *new_generation,
                )?;
                self.outbox_settled = true;
                self.intent_disposition = Disposition::SettledApplied;
                Phase::OutboxSettled
            }
        };
        self.last_recovery_origin = Some(origin);
        self.indeterminate_from = None;
        self.reconciliation_count = self
            .reconciliation_count
            .checked_add(1)
            .ok_or(IntelligenceMutationStateError::ReconciliationOverflow)?;
        Ok(next_phase)
    }

    fn publish_projection(
        &mut self,
        expected_previous_generation: u64,
        new_generation: u64,
    ) -> Result<(), IntelligenceMutationStateError> {
        if self.projection_publish_count != 0 {
            return Err(IntelligenceMutationStateError::DoubleProjectionPublish);
        }
        if expected_previous_generation != self.last_published_generation {
            return Err(IntelligenceMutationStateError::StaleProjectionGeneration {
                expected: self.last_published_generation,
                received: expected_previous_generation,
            });
        }
        validate_projection_step(expected_previous_generation, new_generation)?;
        self.projection_publish_count = 1;
        self.last_published_generation = new_generation;
        Ok(())
    }

    fn reconcile_projection(
        &mut self,
        expected_previous_generation: u64,
        new_generation: u64,
    ) -> Result<(), IntelligenceMutationStateError> {
        let starting_generation = self.binding.starting_projection_generation;
        if expected_previous_generation != starting_generation {
            return Err(IntelligenceMutationStateError::StaleProjectionGeneration {
                expected: starting_generation,
                received: expected_previous_generation,
            });
        }
        validate_projection_step(expected_previous_generation, new_generation)?;
        let expected_new = starting_generation
            .checked_add(1)
            .ok_or(IntelligenceMutationStateError::ProjectionGenerationOverflow)?;
        if new_generation != expected_new {
            return Err(IntelligenceMutationStateError::StaleProjectionGeneration {
                expected: expected_new,
                received: new_generation,
            });
        }
        if self.projection_publish_count == 1
            && self.last_published_generation != new_generation
        {
            return Err(IntelligenceMutationStateError::ReconciliationConflict(
                "observed projection generation conflicts with prior state".to_string(),
            ));
        }
        self.projection_publish_count = 1;
        self.last_published_generation = new_generation;
        Ok(())
    }
}
