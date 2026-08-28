    fn digest(value: &str) -> Sha256Digest {
        Sha256Digest::for_bytes(value.as_bytes())
    }

    fn binding() -> IntelligenceMutationBinding {
        IntelligenceMutationBinding::derive(
            "agent:test",
            "workspace:test",
            "remember",
            "turn:7:memory:aurora",
            "lease:1",
            7,
            Some(3),
            11,
        )
        .expect("binding")
    }

    fn request(
        state: &IntelligenceMutationState,
        action: IntelligenceMutationAction,
    ) -> IntelligenceMutationTransitionRequest {
        IntelligenceMutationTransitionRequest {
            binding: state.binding().clone(),
            sequence: state.next_sequence(),
            causal_parent_sha256: state.causal_parent_sha256(),
            action,
        }
    }

    fn apply(
        state: &mut IntelligenceMutationState,
        action: IntelligenceMutationAction,
    ) -> IntelligenceMutationTransitionReceipt {
        let request = request(state, action);
        state.apply(request).expect("transition").receipt
    }

    fn witness_and_ground(state: &mut IntelligenceMutationState) {
        apply(
            state,
            IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("source"),
            },
        );
        apply(
            state,
            IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
        );
    }

    fn advance_to_intent(state: &mut IntelligenceMutationState) {
        witness_and_ground(state);
        apply(
            state,
            IntelligenceMutationAction::AppendDurableIntent {
                intent_sha256: digest("intent"),
            },
        );
    }

    fn commit_memory(state: &mut IntelligenceMutationState) {
        apply(
            state,
            IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: digest("write"),
            },
        );
    }

    fn publish_projection(state: &mut IntelligenceMutationState) {
        apply(
            state,
            IntelligenceMutationAction::PublishProjection {
                expected_previous_generation: 11,
                new_generation: 12,
                projection_receipt_sha256: digest("projection"),
            },
        );
    }

    fn settle_outbox(state: &mut IntelligenceMutationState) {
        apply(
            state,
            IntelligenceMutationAction::SettleOutbox {
                outcome_sha256: digest("settled"),
            },
        );
    }

    fn happy_path_events() -> Vec<(
        IntelligenceMutationTransitionRequest,
        IntelligenceMutationTransitionReceipt,
    )> {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        let actions = [
            IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("source"),
            },
            IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
            IntelligenceMutationAction::AppendDurableIntent {
                intent_sha256: digest("intent"),
            },
            IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: digest("write"),
            },
            IntelligenceMutationAction::PublishProjection {
                expected_previous_generation: 11,
                new_generation: 12,
                projection_receipt_sha256: digest("projection"),
            },
            IntelligenceMutationAction::SettleOutbox {
                outcome_sha256: digest("settled"),
            },
            IntelligenceMutationAction::Terminalize,
        ];
        actions
            .into_iter()
            .map(|action| {
                let request = request(&state, action);
                let receipt = state.apply(request.clone()).expect("apply").receipt;
                (request, receipt)
            })
            .collect()
    }
