impl IntelligenceMutationState {
    pub(crate) fn new(
        binding: IntelligenceMutationBinding,
    ) -> Result<Self, IntelligenceMutationStateError> {
        binding.validate()?;
        let state = Self {
            last_published_generation: binding.starting_projection_generation,
            binding,
            phase: IntelligenceMutationPhase::Planned,
            next_sequence: 1,
            history: Vec::new(),
            intent_disposition: IntelligenceMutationIntentDisposition::None,
            memory_write_count: 0,
            projection_publish_count: 0,
            outbox_settled: false,
            indeterminate_from: None,
            last_recovery_origin: None,
            reconciliation_count: 0,
        };
        state.validate()?;
        Ok(state)
    }

    pub(crate) const fn phase(&self) -> IntelligenceMutationPhase {
        self.phase
    }

    pub(crate) const fn next_sequence(&self) -> u64 {
        self.next_sequence
    }

    pub(crate) fn causal_parent_sha256(&self) -> Sha256Digest {
        self.history
            .last()
            .map_or_else(
                || self.binding.causal_root_sha256.clone(),
                |entry| entry.receipt.transition_sha256.clone(),
            )
    }

    pub(crate) fn binding(&self) -> &IntelligenceMutationBinding {
        &self.binding
    }

    pub(crate) fn history_len(&self) -> usize {
        self.history.len()
    }

    pub(crate) fn apply(
        &mut self,
        request: IntelligenceMutationTransitionRequest,
    ) -> Result<IntelligenceMutationApplyResult, IntelligenceMutationStateError> {
        request.binding.validate()?;
        request.action.validate()?;
        if request.binding != self.binding {
            return Err(IntelligenceMutationStateError::BindingDrift);
        }
        if request.sequence == 0 {
            return Err(IntelligenceMutationStateError::SequenceMismatch {
                expected: self.next_sequence,
                received: 0,
            });
        }

        let request_sha256 = request_digest(&request);
        if request.sequence < self.next_sequence {
            let history_index = request
                .sequence
                .checked_sub(1)
                .and_then(|value| usize::try_from(value).ok())
                .ok_or(IntelligenceMutationStateError::ReplayConflict)?;
            let entry = self
                .history
                .get(history_index)
                .ok_or(IntelligenceMutationStateError::ReplayConflict)?;
            if entry.request_sha256 != request_sha256 {
                return Err(IntelligenceMutationStateError::ReplayConflict);
            }
            verify_transition_receipt(&entry.receipt)?;
            return Ok(IntelligenceMutationApplyResult {
                disposition: IntelligenceMutationApplyDisposition::Replay,
                receipt: entry.receipt.clone(),
            });
        }
        if request.sequence != self.next_sequence {
            return Err(IntelligenceMutationStateError::SequenceMismatch {
                expected: self.next_sequence,
                received: request.sequence,
            });
        }
        if request.causal_parent_sha256 != self.causal_parent_sha256() {
            return Err(IntelligenceMutationStateError::CausalParentMismatch);
        }
        if self.phase.is_terminal() {
            return Err(IntelligenceMutationStateError::TerminalState(self.phase));
        }
        if self.history.len() >= MAX_TRANSITIONS {
            return Err(IntelligenceMutationStateError::TransitionLimit {
                maximum: MAX_TRANSITIONS,
            });
        }
        let next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(IntelligenceMutationStateError::SequenceOverflow)?;

        // Apply to a clone first. Any error leaves the caller-visible state
        // byte-for-byte unchanged.
        let mut candidate = self.clone();
        let from_phase = candidate.phase;
        let to_phase = candidate.apply_action(&request.action)?;
        candidate.phase = to_phase;

        let mut receipt = IntelligenceMutationTransitionReceipt {
            schema_version: INTELLIGENCE_MUTATION_STATE_SCHEMA_VERSION,
            namespace: INTELLIGENCE_MUTATION_STATE_NAMESPACE.to_string(),
            operation_id: candidate.binding.operation_id.clone(),
            sequence: request.sequence,
            from_phase,
            to_phase,
            action: request.action.kind().to_string(),
            request_sha256: request_sha256.clone(),
            causal_parent_sha256: request.causal_parent_sha256,
            transition_sha256: Sha256Digest::for_bytes(b"pending-transition-digest"),
            intent_disposition: candidate.intent_disposition,
            memory_write_count: candidate.memory_write_count,
            projection_publish_count: candidate.projection_publish_count,
            outbox_settled: candidate.outbox_settled,
            last_published_generation: candidate.last_published_generation,
            indeterminate_from: candidate.indeterminate_from,
            last_recovery_origin: candidate.last_recovery_origin,
            reconciliation_count: candidate.reconciliation_count,
            runtime_wired: INTELLIGENCE_MUTATION_STATE_RUNTIME_WIRED,
            qualified: INTELLIGENCE_MUTATION_STATE_QUALIFIED,
            sqlite_persistence: INTELLIGENCE_MUTATION_STATE_SQLITE_PERSISTENCE,
            external_effects: INTELLIGENCE_MUTATION_STATE_EXTERNAL_EFFECTS,
            production_authority: INTELLIGENCE_MUTATION_STATE_PRODUCTION_AUTHORITY,
            operator_acceptance: INTELLIGENCE_MUTATION_STATE_OPERATOR_ACCEPTANCE,
            promotion: INTELLIGENCE_MUTATION_STATE_PROMOTION,
            callers_ratchet: INTELLIGENCE_MUTATION_STATE_CALLERS_RATCHET,
        };
        receipt.transition_sha256 = transition_digest(&receipt);
        verify_transition_receipt(&receipt)?;

        candidate.history.push(IntelligenceMutationHistoryEntry {
            request_sha256,
            receipt: receipt.clone(),
        });
        candidate.next_sequence = next_sequence;
        candidate.validate()?;
        *self = candidate;

        Ok(IntelligenceMutationApplyResult {
            disposition: IntelligenceMutationApplyDisposition::Applied,
            receipt,
        })
    }

    pub(crate) fn replay_verified(
        binding: IntelligenceMutationBinding,
        events: &[(
            IntelligenceMutationTransitionRequest,
            IntelligenceMutationTransitionReceipt,
        )],
    ) -> Result<Self, IntelligenceMutationStateError> {
        let mut state = Self::new(binding)?;
        for (request, supplied_receipt) in events {
            verify_transition_receipt(supplied_receipt)?;
            let result = state.apply(request.clone())?;
            if result.disposition != IntelligenceMutationApplyDisposition::Applied
                || &result.receipt != supplied_receipt
            {
                return Err(IntelligenceMutationStateError::ReceiptMismatch {
                    sequence: supplied_receipt.sequence,
                });
            }
        }
        state.validate()?;
        Ok(state)
    }

    pub(crate) fn validate(&self) -> Result<(), IntelligenceMutationStateError> {
        self.binding.validate()?;
        if self.history.len() > MAX_TRANSITIONS {
            return Err(IntelligenceMutationStateError::Corrupt(
                "transition history exceeds the source-only bound".to_string(),
            ));
        }
        let expected_next = u64::try_from(self.history.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or(IntelligenceMutationStateError::SequenceOverflow)?;
        if self.next_sequence != expected_next {
            return Err(IntelligenceMutationStateError::Corrupt(
                "next transition sequence does not match history length".to_string(),
            ));
        }
        self.validate_history()?;
        self.validate_business_invariants()
    }

    fn validate_history(&self) -> Result<(), IntelligenceMutationStateError> {
        let mut expected_parent = self.binding.causal_root_sha256.clone();
        let mut expected_from_phase = IntelligenceMutationPhase::Planned;
        for (index, entry) in self.history.iter().enumerate() {
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(IntelligenceMutationStateError::SequenceOverflow)?;
            let receipt = &entry.receipt;
            verify_transition_receipt(receipt)?;
            if receipt.operation_id != self.binding.operation_id
                || receipt.sequence != expected_sequence
                || receipt.request_sha256 != entry.request_sha256
                || receipt.causal_parent_sha256 != expected_parent
                || receipt.from_phase != expected_from_phase
            {
                return Err(IntelligenceMutationStateError::Corrupt(
                    "transition history binding, sequence, request, parent, or phase drifted"
                        .to_string(),
                ));
            }
            expected_parent = receipt.transition_sha256.clone();
            expected_from_phase = receipt.to_phase;
        }
        match self.history.last() {
            Some(entry) => {
                let receipt = &entry.receipt;
                if receipt.to_phase != self.phase
                    || receipt.intent_disposition != self.intent_disposition
                    || receipt.memory_write_count != self.memory_write_count
                    || receipt.projection_publish_count != self.projection_publish_count
                    || receipt.outbox_settled != self.outbox_settled
                    || receipt.last_published_generation != self.last_published_generation
                    || receipt.indeterminate_from != self.indeterminate_from
                    || receipt.last_recovery_origin != self.last_recovery_origin
                    || receipt.reconciliation_count != self.reconciliation_count
                {
                    return Err(IntelligenceMutationStateError::Corrupt(
                        "current state does not match its latest transition receipt"
                            .to_string(),
                    ));
                }
            }
            None if self.phase != IntelligenceMutationPhase::Planned => {
                return Err(IntelligenceMutationStateError::Corrupt(
                    "non-planned state has no transition history".to_string(),
                ));
            }
            None => {}
        }
        Ok(())
    }

}
