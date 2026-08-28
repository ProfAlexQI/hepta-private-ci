    #[test]
    fn crash_after_projection_resumes_at_projection_without_second_publish() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        commit_memory(&mut state);
        publish_projection(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("crash-after-projection"),
            },
        );
        apply(
            &mut state,
            IntelligenceMutationAction::Reconcile {
                observation:
                    IntelligenceMutationReconciliationObservation::ProjectionPublished {
                        write_receipt_sha256: digest("observed-write"),
                        expected_previous_generation: 11,
                        new_generation: 12,
                        projection_receipt_sha256: digest("observed-projection"),
                    },
            },
        );
        assert_eq!(
            state.phase(),
            IntelligenceMutationPhase::ProjectionPublished
        );
        assert_eq!(state.projection_publish_count, 1);
        settle_outbox(&mut state);
        apply(&mut state, IntelligenceMutationAction::Terminalize);
    }

    #[test]
    fn crash_after_outbox_resumes_at_settled_and_terminalizes() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        commit_memory(&mut state);
        publish_projection(&mut state);
        settle_outbox(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("crash-before-terminal"),
            },
        );
        apply(
            &mut state,
            IntelligenceMutationAction::Reconcile {
                observation: IntelligenceMutationReconciliationObservation::OutboxSettled {
                    write_receipt_sha256: digest("observed-write"),
                    expected_previous_generation: 11,
                    new_generation: 12,
                    projection_receipt_sha256: digest("observed-projection"),
                    outcome_sha256: digest("observed-outbox"),
                },
            },
        );
        assert_eq!(state.phase(), IntelligenceMutationPhase::OutboxSettled);
        apply(&mut state, IntelligenceMutationAction::Terminalize);
        assert_eq!(state.phase(), IntelligenceMutationPhase::Terminal);
    }

    #[test]
    fn reconciliation_cannot_regress_or_skip_required_projection_work() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        commit_memory(&mut state);
        publish_projection(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("crash"),
            },
        );
        let before = state.clone();
        let regress = request(
            &state,
            IntelligenceMutationAction::Reconcile {
                observation:
                    IntelligenceMutationReconciliationObservation::MemoryFactsCommitted {
                        write_receipt_sha256: digest("write"),
                    },
            },
        );
        assert!(matches!(
            state.apply(regress),
            Err(IntelligenceMutationStateError::ReconciliationRegression { .. })
        ));
        assert_eq!(state, before);

        let not_applied = request(
            &state,
            IntelligenceMutationAction::Reconcile {
                observation: IntelligenceMutationReconciliationObservation::NotApplied {
                    outcome_sha256: digest("not-applied"),
                },
            },
        );
        assert!(matches!(
            state.apply(not_applied),
            Err(IntelligenceMutationStateError::InvalidReconciliation(_))
        ));
        assert_eq!(state, before);
    }

    #[test]
    fn quarantine_is_explicit_and_does_not_masquerade_as_applied() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        commit_memory(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("ambiguous"),
            },
        );
        let receipt = apply(
            &mut state,
            IntelligenceMutationAction::Quarantine {
                reason_sha256: digest("operator-review-required"),
            },
        );
        assert_eq!(state.phase(), IntelligenceMutationPhase::Quarantined);
        assert_eq!(
            receipt.intent_disposition,
            IntelligenceMutationIntentDisposition::Quarantined
        );
        assert!(!receipt.outbox_settled);
    }

    #[test]
    fn verified_replay_rejects_digest_and_authority_tamper() {
        let events = happy_path_events();
        let replayed = IntelligenceMutationState::replay_verified(binding(), &events)
            .expect("verified replay");
        assert_eq!(replayed.phase(), IntelligenceMutationPhase::Terminal);

        let mut digest_tamper = events.clone();
        digest_tamper[2].1.transition_sha256 = digest("tampered-transition");
        assert_eq!(
            IntelligenceMutationState::replay_verified(binding(), &digest_tamper),
            Err(IntelligenceMutationStateError::ReceiptDigestMismatch)
        );

        let mut authority_tamper = events.clone();
        authority_tamper[0].1.production_authority = true;
        assert_eq!(
            IntelligenceMutationState::replay_verified(binding(), &authority_tamper),
            Err(IntelligenceMutationStateError::AuthorityEscalation)
        );

        let mut ratchet_tamper = events.clone();
        ratchet_tamper[0].1.callers_ratchet = true;
        assert_eq!(
            IntelligenceMutationState::replay_verified(binding(), &ratchet_tamper),
            Err(IntelligenceMutationStateError::AuthorityEscalation)
        );

        let mut schema_tamper = events;
        schema_tamper[0].1.schema_version += 1;
        assert_eq!(
            IntelligenceMutationState::replay_verified(binding(), &schema_tamper),
            Err(IntelligenceMutationStateError::ReceiptSchemaMismatch)
        );
    }
