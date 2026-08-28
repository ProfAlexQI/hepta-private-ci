    #[test]
    fn changed_replay_reorder_parent_and_binding_drift_fail_closed() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        let first = request(
            &state,
            IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("source"),
            },
        );
        state.apply(first).expect("first");

        let changed = IntelligenceMutationTransitionRequest {
            binding: state.binding().clone(),
            sequence: 1,
            causal_parent_sha256: state.binding().causal_root_sha256.clone(),
            action: IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("other-source"),
            },
        };
        let before = state.clone();
        assert_eq!(
            state.apply(changed),
            Err(IntelligenceMutationStateError::ReplayConflict)
        );
        assert_eq!(state, before);

        let reordered = IntelligenceMutationTransitionRequest {
            binding: state.binding().clone(),
            sequence: state.next_sequence() + 1,
            causal_parent_sha256: state.causal_parent_sha256(),
            action: IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
        };
        assert!(matches!(
            state.apply(reordered),
            Err(IntelligenceMutationStateError::SequenceMismatch { .. })
        ));
        assert_eq!(state, before);

        let wrong_parent = IntelligenceMutationTransitionRequest {
            binding: state.binding().clone(),
            sequence: state.next_sequence(),
            causal_parent_sha256: digest("wrong-parent"),
            action: IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
        };
        assert_eq!(
            state.apply(wrong_parent),
            Err(IntelligenceMutationStateError::CausalParentMismatch)
        );
        assert_eq!(state, before);

        let mut drifted_binding = state.binding().clone();
        drifted_binding.lease_epoch += 1;
        let drifted = IntelligenceMutationTransitionRequest {
            binding: drifted_binding,
            sequence: state.next_sequence(),
            causal_parent_sha256: state.causal_parent_sha256(),
            action: IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
        };
        assert_eq!(
            state.apply(drifted),
            Err(IntelligenceMutationStateError::BindingDrift)
        );
        assert_eq!(state, before);
    }

    #[test]
    fn stale_or_overflowing_projection_generation_is_zero_mutation() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        commit_memory(&mut state);
        let before = state.clone();
        let stale = request(
            &state,
            IntelligenceMutationAction::PublishProjection {
                expected_previous_generation: 10,
                new_generation: 11,
                projection_receipt_sha256: digest("projection"),
            },
        );
        assert!(matches!(
            state.apply(stale),
            Err(IntelligenceMutationStateError::StaleProjectionGeneration { .. })
        ));
        assert_eq!(state, before);

        let overflow_binding = IntelligenceMutationBinding::derive(
            "agent:test",
            "workspace:test",
            "remember",
            "overflow",
            "lease:overflow",
            1,
            None,
            u64::MAX,
        )
        .expect("binding");
        let mut overflow = IntelligenceMutationState::new(overflow_binding).expect("state");
        advance_to_intent(&mut overflow);
        commit_memory(&mut overflow);
        let before = overflow.clone();
        let request = request(
            &overflow,
            IntelligenceMutationAction::PublishProjection {
                expected_previous_generation: u64::MAX,
                new_generation: u64::MAX,
                projection_receipt_sha256: digest("projection"),
            },
        );
        assert_eq!(
            overflow.apply(request),
            Err(IntelligenceMutationStateError::ProjectionGenerationOverflow)
        );
        assert_eq!(overflow, before);
    }

    #[test]
    fn crash_before_write_can_only_reconcile_not_applied() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("crash-after-intent"),
            },
        );
        let receipt = apply(
            &mut state,
            IntelligenceMutationAction::Reconcile {
                observation: IntelligenceMutationReconciliationObservation::NotApplied {
                    outcome_sha256: digest("not-applied"),
                },
            },
        );
        assert_eq!(
            state.phase(),
            IntelligenceMutationPhase::ReconciledNotApplied
        );
        assert_eq!(
            receipt.intent_disposition,
            IntelligenceMutationIntentDisposition::SettledNotApplied
        );
        assert_eq!(receipt.memory_write_count, 0);
        assert_eq!(receipt.projection_publish_count, 0);
    }

    #[test]
    fn crash_after_write_resumes_at_memory_and_must_finish_projection_and_outbox() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        commit_memory(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("crash-after-write"),
            },
        );
        apply(
            &mut state,
            IntelligenceMutationAction::Reconcile {
                observation:
                    IntelligenceMutationReconciliationObservation::MemoryFactsCommitted {
                        write_receipt_sha256: digest("observed-write"),
                    },
            },
        );
        assert_eq!(
            state.phase(),
            IntelligenceMutationPhase::MemoryFactsCommitted
        );
        publish_projection(&mut state);
        settle_outbox(&mut state);
        apply(&mut state, IntelligenceMutationAction::Terminalize);
        assert_eq!(state.phase(), IntelligenceMutationPhase::Terminal);
    }
