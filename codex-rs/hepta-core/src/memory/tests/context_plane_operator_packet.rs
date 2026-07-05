use super::*;

#[test]
fn context_plane_operator_approval_packet_is_payload_light_dry_run() {
    let taxonomy = ContextMemoryTaxonomyReport {
        buckets: vec![ContextMemoryTaxonomyBucket {
            class: ContextMemoryTaxonomyClass::Semantic,
            source_count: 1,
            returned_count: 1,
            available_count: 1,
            omitted_count: 0,
            provenance_span_count: 0,
        }],
    };
    let formation_receipts = ContextMemoryFormationReceiptReport {
        receipts: vec![ContextMemoryFormationReceipt {
            candidate_type: ContextMemoryFormationCandidateType::Fact,
            transcript_span_count: 1,
            provenance_span_count: 1,
            confidence_basis_points: 6400,
            idempotency_key_hash: stable_receipt_hash(&[
                "memory_formation",
                "fact",
                "approval-test",
                "1",
                "1",
            ]),
            privacy_class: "user_private".into(),
            queued_for_background: true,
            production_write: false,
        }],
    };
    let formation_queue = ContextMemoryFormationQueueReport::from_receipts(&formation_receipts);
    let temporal_facts = ContextMemoryTemporalFactReport {
        facts: vec![ContextMemoryTemporalFact {
            fact_type: ContextMemoryTemporalFactType::Attribute,
            entity_hash: stable_receipt_hash(&[
                "memory_temporal_fact_entity",
                "attribute",
                "approval-test",
                "1",
                "1",
            ]),
            provenance_span_count: 1,
            valid_from_sequence: 1,
            invalid_at_sequence: None,
            confidence_basis_points: 6200,
            supersedes_fact_hash: None,
            privacy_class: "user_private".into(),
            dry_run_only: true,
            production_write: false,
        }],
    };
    let temporal_fact_graph =
        ContextMemoryTemporalFactGraphReport::from_temporal_facts(&temporal_facts);
    let eval_seed = ContextMemoryEvalHarnessReport::seeded();
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::from_seed(&eval_seed);
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let status = ContextPlaneStatusReport::from_reports(
        &taxonomy,
        &formation_receipts,
        &formation_queue,
        &temporal_facts,
        &temporal_fact_graph,
        &eval_seed,
        &allocator_shadow,
        &recall_quality_gate,
    );
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);

    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);

    assert!(packet.has_packet_integrity());
    assert!(packet.dry_run_only);
    assert!(packet.approval_required);
    assert!(!packet.activation_command_present);
    assert_eq!(packet.matrix_row_count, 12);
    assert_eq!(packet.threshold_satisfied_count, 9);
    assert_eq!(packet.blocker_count, 3);
    assert_eq!(packet.threshold_snapshot.total_row_count, 12);
    assert_eq!(packet.threshold_snapshot.required_ready_count, 11);
    assert_eq!(packet.threshold_snapshot.required_shadow_count, 1);
    assert_eq!(packet.required_scope_count(), 6);
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(ContextPlaneActivationBlockerReason::OperatorApprovalMissing),
        Some(1)
    );
    assert_eq!(packet.recall_quality_blocking_reason_count, 0);
    assert_eq!(packet.recall_quality_blocking_reason_count_total(), 0);
    assert!(!packet.production_write);
    assert!(!packet.graph_write);
    assert!(!packet.runtime_activation);
    assert!(!packet.adaptive_allocator_runtime_activation);
    assert!(!packet.source_aware_runtime_activation);
    assert!(!packet.prompt_assembly_change);
    assert!(!packet.operator_activation_allowed);

    let json = serde_json::to_string(&packet).expect("operator approval packet should serialize");
    assert!(json.contains("adaptive_budget_allocation_runtime"));
    assert!(json.contains("source_aware_runtime_activation"));
    assert!(json.contains("operator_activation"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reason_counts"));
    assert!(json.contains("adaptive_budget_allocation_shadow_only"));
    assert!(json.contains("source_aware_front_door_disabled"));
    assert!(json.contains("operator_approval_missing"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("approval-test"));
    assert!(!json.contains("\"activation_command_present\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_plane_operator_approval_packet_rolls_up_recall_quality_blockers_without_payloads() {
    let mut status = super::context_plane_activation::context_plane_activation_status_fixture();
    let recall_quality_entry = status
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist");
    recall_quality_entry.status = ContextPlaneStatusKind::Blocked;
    recall_quality_entry.blocker_count = 2;
    recall_quality_entry.recall_quality_blocking_reason_count = 2;
    recall_quality_entry.recall_quality_blocking_reasons = vec![
        ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression,
        ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled,
    ];
    recall_quality_entry.prompt_assembly_change = true;

    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);

    assert!(packet.has_packet_integrity());
    assert!(packet.dry_run_only);
    assert!(packet.approval_required);
    assert!(!packet.activation_command_present);
    assert_eq!(packet.matrix_row_count, 12);
    assert_eq!(packet.threshold_satisfied_count, 8);
    assert_eq!(packet.blocker_count, 4);
    assert_eq!(
        packet.blocker_reason_count(ContextPlaneActivationBlockerReason::SideEffectFlagEnabled),
        Some(1)
    );
    assert_eq!(packet.recall_quality_blocking_reason_count, 2);
    assert_eq!(packet.recall_quality_blocking_reason_count_total(), 2);
    assert_eq!(
        packet.recall_quality_blocking_reason_count_for(
            ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression,
        ),
        Some(1)
    );
    assert_eq!(
        packet.recall_quality_blocking_reason_count_for(
            ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled,
        ),
        Some(1)
    );
    assert!(!packet.production_write);
    assert!(!packet.graph_write);
    assert!(!packet.runtime_activation);
    assert!(!packet.adaptive_allocator_runtime_activation);
    assert!(!packet.source_aware_runtime_activation);
    assert!(!packet.prompt_assembly_change);
    assert!(!packet.operator_activation_allowed);

    let json =
        serde_json::to_string(&packet).expect("recall-quality approval packet should serialize");
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reason_counts"));
    assert!(json.contains("answer_quality_regression"));
    assert!(json.contains("side_effect_flag_enabled"));
    assert!(!json.contains("fixture_id_hash"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("approval-test"));
    assert!(!json.contains("\"activation_command_present\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_plane_operator_approval_packet_rejects_activation_shaped_input() {
    let packet = ContextPlaneOperatorApprovalPacket {
        matrix_row_count: 12,
        threshold_satisfied_count: 9,
        blocker_count: 3,
        threshold_snapshot: ContextPlaneOperatorApprovalThresholdSnapshot {
            total_row_count: 12,
            threshold_satisfied_count: 9,
            blocker_count: 3,
            required_ready_count: 11,
            required_shadow_count: 1,
        },
        blocker_reason_counts: vec![
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::OperatorApprovalMissing,
                count: 1,
            },
        ],
        required_approval_scopes: required_operator_approval_scopes(),
        ..ContextPlaneOperatorApprovalPacket::default()
    };
    assert!(packet.has_packet_integrity());

    for field in [
        "activation_command_present",
        "production_write",
        "graph_write",
        "runtime_activation",
        "adaptive_allocator_runtime_activation",
        "source_aware_runtime_activation",
        "prompt_assembly_change",
        "operator_activation_allowed",
    ] {
        let mut activation_shaped =
            serde_json::to_value(&packet).expect("packet should convert to json value");
        activation_shaped
            .as_object_mut()
            .expect("packet json should be an object")
            .insert(field.into(), serde_json::Value::Bool(true));

        let parsed: ContextPlaneOperatorApprovalPacket = serde_json::from_value(activation_shaped)
            .expect("known side-effect field should deserialize");
        assert!(
            !parsed.has_packet_integrity(),
            "{field} must fail operator approval packet integrity"
        );
    }

    for (field, value) in [
        (
            "activation_command",
            serde_json::Value::String("hepta context activate --now".into()),
        ),
        (
            "tool_args",
            serde_json::json!({"command": "hepta context activate --now"}),
        ),
        (
            "raw_payload",
            serde_json::Value::String("prompt_text transcript_text memory_text".into()),
        ),
        (
            "operator_email",
            serde_json::Value::String("operator@example.com".into()),
        ),
        (
            "session_id",
            serde_json::Value::String("session-private".into()),
        ),
    ] {
        let mut malformed =
            serde_json::to_value(&packet).expect("packet should convert to json value");
        malformed
            .as_object_mut()
            .expect("packet json should be an object")
            .insert(field.into(), value);

        assert!(
            serde_json::from_value::<ContextPlaneOperatorApprovalPacket>(malformed).is_err(),
            "{field} must be rejected, not silently dropped"
        );
    }

    let mut nested_threshold =
        serde_json::to_value(&packet).expect("packet should convert to json value");
    nested_threshold["threshold_snapshot"]
        .as_object_mut()
        .expect("threshold snapshot should be an object")
        .insert(
            "activation_command".into(),
            serde_json::Value::String("hepta context activate --now".into()),
        );
    assert!(
        serde_json::from_value::<ContextPlaneOperatorApprovalPacket>(nested_threshold).is_err()
    );

    let mut nested_blocker =
        serde_json::to_value(&packet).expect("packet should convert to json value");
    nested_blocker["blocker_reason_counts"][0]
        .as_object_mut()
        .expect("blocker reason count should be an object")
        .insert(
            "raw_payload".into(),
            serde_json::Value::String("memory_text".into()),
        );
    assert!(serde_json::from_value::<ContextPlaneOperatorApprovalPacket>(nested_blocker).is_err());
}
