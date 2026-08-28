    #[test]
    fn normal_path_requires_complete_projection_and_outbox_before_terminal() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        commit_memory(&mut state);
        publish_projection(&mut state);
        let before = state.clone();
        let terminal_request = request(&state, IntelligenceMutationAction::Terminalize);
        assert!(matches!(
            state.apply(terminal_request),
            Err(IntelligenceMutationStateError::InvalidTransition { .. })
        ));
        assert_eq!(state, before);

        settle_outbox(&mut state);
        let terminal = apply(&mut state, IntelligenceMutationAction::Terminalize);
        assert_eq!(state.phase(), IntelligenceMutationPhase::Terminal);
        assert_eq!(
            terminal.intent_disposition,
            IntelligenceMutationIntentDisposition::SettledApplied
        );
        assert_eq!(terminal.memory_write_count, 1);
        assert_eq!(terminal.projection_publish_count, 1);
        assert!(terminal.outbox_settled);
        assert!(!terminal.runtime_wired);
        assert!(!terminal.qualified);
        assert!(!terminal.sqlite_persistence);
        assert!(!terminal.production_authority);
        assert!(!terminal.callers_ratchet);
        state.validate().expect("valid terminal state");
    }

    #[test]
    fn precommit_rejection_and_cancel_are_terminal_without_durable_intent() {
        for action in [
            IntelligenceMutationAction::RejectPreCommit {
                reason_sha256: digest("rejected"),
            },
            IntelligenceMutationAction::CancelPreCommit {
                reason_sha256: digest("cancelled"),
            },
        ] {
            let mut state = IntelligenceMutationState::new(binding()).expect("state");
            witness_and_ground(&mut state);
            let receipt = apply(&mut state, action);
            assert!(matches!(
                state.phase(),
                IntelligenceMutationPhase::RejectedPreCommit
                    | IntelligenceMutationPhase::CancelledPreCommit
            ));
            assert_eq!(
                receipt.intent_disposition,
                IntelligenceMutationIntentDisposition::None
            );
            assert_eq!(receipt.memory_write_count, 0);
            assert_eq!(receipt.projection_publish_count, 0);
            assert!(!receipt.outbox_settled);
        }
    }

    #[test]
    fn exact_duplicate_of_any_historical_sequence_replays_without_mutation() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        let first = request(
            &state,
            IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("source"),
            },
        );
        let first_receipt = state.apply(first.clone()).expect("first").receipt;
        apply(
            &mut state,
            IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
        );
        apply(
            &mut state,
            IntelligenceMutationAction::AppendDurableIntent {
                intent_sha256: digest("intent"),
            },
        );
        let before = state.clone();
        let replay = state.apply(first).expect("historical replay");
        assert_eq!(
            replay.disposition,
            IntelligenceMutationApplyDisposition::Replay
        );
        assert_eq!(replay.receipt, first_receipt);
        assert_eq!(state, before);
    }
