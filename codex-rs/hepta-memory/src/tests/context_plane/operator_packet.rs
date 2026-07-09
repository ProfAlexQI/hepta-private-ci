use super::*;

#[test]
fn store_snapshot_context_plane_operator_approval_packet_is_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
            memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ),
        ],
    };
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };

    let packet = snapshot.context_plane_operator_approval_packet(&request);

    assert!(packet.has_packet_integrity());
    assert!(packet.dry_run_only);
    assert!(packet.approval_required);
    assert!(!packet.activation_command_present);
    assert_eq!(packet.matrix_row_count, 25);
    assert_eq!(packet.threshold_satisfied_count, 9);
    assert_eq!(packet.blocker_count, 16);
    assert_eq!(packet.threshold_snapshot.required_ready_count, 24);
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
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryWriteChainReceiptFreshnessShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowStoreShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowReplayShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalDiffShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalQualityShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(ContextPlaneActivationBlockerReason::OperatorApprovalMissing),
        Some(1)
    );
    assert_eq!(packet.recall_quality_blocking_reason_count, 0);
    assert_eq!(packet.recall_quality_blocking_reason_count_total(), 0);
    assert_eq!(packet.canary_promotion_blocker_count, 0);
    assert_eq!(packet.canary_promotion_checklist_required_count, 4);
    assert_eq!(packet.canary_promotion_checklist_pass_count, 4);
    assert!(packet.canary_promotion_readiness_check_pass);
    assert!(packet.canary_promotion_negative_rehearsal_check_pass);
    assert!(packet.canary_promotion_audit_digest_check_pass);
    assert!(packet.canary_promotion_audit_freshness_check_pass);
    assert_eq!(packet.canary_promotion_rollback_rehearsal_pass_count, 3);
    assert_eq!(packet.canary_promotion_kill_switch_rehearsal_pass_count, 3);
    assert_eq!(packet.canary_promotion_soak_readback_pass_count, 3);
    assert_eq!(packet.memory_provider_v2_lifecycle_required_count, 6);
    assert_eq!(packet.memory_provider_v2_lifecycle_pass_count, 6);
    assert!(packet.memory_provider_v2_query_check_pass);
    assert!(packet.memory_provider_v2_update_context_check_pass);
    assert!(packet.memory_provider_v2_propose_write_check_pass);
    assert!(packet.memory_provider_v2_add_check_pass);
    assert!(packet.memory_provider_v2_clear_check_pass);
    assert!(packet.memory_provider_v2_close_check_pass);
    assert_eq!(packet.memory_namespace_policy_namespace_count, 6);
    assert_eq!(
        packet.memory_namespace_policy_operator_approval_required_count,
        6
    );
    assert_eq!(packet.memory_namespace_policy_shadow_wal_required_count, 6);
    assert_eq!(packet.memory_namespace_policy_production_write_count, 0);
    assert_eq!(packet.memory_write_chain_namespace_count, 6);
    assert_eq!(packet.memory_write_chain_stage_required_count, 6);
    assert_eq!(packet.memory_write_chain_stage_pass_count, 6);
    assert_eq!(packet.memory_write_chain_readback_ready_count, 6);
    assert_eq!(packet.memory_write_chain_canary_ready_count, 6);
    assert_eq!(packet.memory_write_chain_production_write_count, 0);
    assert_eq!(packet.memory_write_chain_graph_write_count, 0);
    assert_eq!(packet.memory_temporal_graph_shadow_store_node_count, 5);
    assert_eq!(packet.memory_temporal_graph_shadow_store_edge_count, 10);
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_stage_projected_count,
        6
    );
    assert_eq!(packet.memory_temporal_graph_shadow_store_digest_count, 1);
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_recorded_receipt_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_graph_write_count,
        0
    );
    assert_eq!(packet.memory_temporal_graph_shadow_replay_node_count, 5);
    assert_eq!(packet.memory_temporal_graph_shadow_replay_edge_count, 10);
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_provenance_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_bitemporal_validity_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_stage_projected_count,
        6
    );
    assert_eq!(packet.memory_temporal_graph_shadow_replay_digest_count, 6);
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_stale_replay_rejected_count,
        6
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_recorded_receipt_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_graph_write_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_production_selection_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count,
        10
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_stage_projected_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_digest_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_llm_rerank_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_graph_persistence_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_production_route_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_production_write_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_graph_write_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_fixture_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_slo_pass_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_coverage_basis_points,
        10_000
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_precision_basis_points,
        10_000
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_leak_rate_basis_points,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_projected_latency_ms,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_token_saved_estimate,
        768
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_operator_review_required_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_stage_projected_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_digest_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_llm_rerank_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_graph_persistence_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_production_route_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_production_write_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_graph_write_count,
        0
    );
    assert_eq!(packet.ranked_recall_hybrid_signal_required_count, 5);
    assert_eq!(packet.ranked_recall_hybrid_signal_pass_count, 5);
    assert_eq!(packet.ranked_recall_positive_hybrid_signal_pass_count, 15);
    assert_eq!(packet.ranked_recall_hybrid_regression_blocked_count, 1);

    let json = serde_json::to_string(&packet).expect("operator approval packet should serialize");
    assert!(json.contains("adaptive_budget_allocation_runtime"));
    assert!(json.contains("source_aware_runtime_activation"));
    assert!(json.contains("operator_activation"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reason_counts"));
    assert!(json.contains("adaptive_budget_allocation_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_eval_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_store_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_replay_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_traversal_diff_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_traversal_quality_shadow_only"));
    assert!(json.contains("memory_temporal_graph_shadow_store_node_count"));
    assert!(json.contains("memory_temporal_graph_shadow_store_stage_projected_count"));
    assert!(json.contains("memory_temporal_graph_shadow_store_stale_replay_rejected_count"));
    assert!(json.contains("memory_temporal_graph_shadow_replay_node_count"));
    assert!(json.contains("memory_temporal_graph_shadow_replay_stage_projected_count"));
    assert!(json.contains("memory_temporal_graph_shadow_replay_stale_replay_rejected_count"));
    assert!(
        json.contains(
            "memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count"
        )
    );
    assert!(json.contains("memory_temporal_graph_shadow_traversal_diff_stage_projected_count"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_slo_pass_count"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_token_saved_estimate"));
    assert!(json.contains("memory_ranked_recall_shadow_eval_shadow_only"));
    assert!(json.contains("ranked_recall_hybrid_signal_pass_count"));
    assert!(json.contains("ranked_recall_positive_hybrid_signal_pass_count"));
    assert!(json.contains("ranked_recall_hybrid_regression_blocked_count"));
    assert!(json.contains("memory_provider_boundary_shadow_only"));
    assert!(json.contains("memory_provider_v2_boundary_shadow_only"));
    assert!(json.contains("memory_provider_v2_lifecycle_pass_count"));
    assert!(json.contains("memory_provider_v2_propose_write_check_pass"));
    assert!(json.contains("memory_provider_v2_close_check_pass"));
    assert!(json.contains("memory_namespace_policy_shadow_only"));
    assert!(json.contains("memory_write_chain_readiness_shadow_only"));
    assert!(json.contains("memory_namespace_policy_namespace_count"));
    assert!(json.contains("memory_namespace_policy_shadow_wal_required_count"));
    assert!(json.contains("memory_write_chain_stage_pass_count"));
    assert!(json.contains("memory_write_chain_readback_ready_count"));
    assert!(json.contains("memory_write_chain_canary_ready_count"));
    assert!(json.contains("memory_shadow_canary_readiness_shadow_only"));
    assert!(json.contains("memory_shadow_canary_promotion_readiness_shadow_only"));
    assert!(json.contains("canary_promotion_checklist_pass_count"));
    assert!(json.contains("canary_promotion_negative_rehearsal_check_pass"));
    assert!(json.contains("canary_promotion_audit_digest_check_pass"));
    assert!(json.contains("canary_promotion_audit_freshness_check_pass"));
    assert!(json.contains("canary_promotion_rollback_rehearsal_pass_count"));
    assert!(json.contains("canary_promotion_kill_switch_rehearsal_pass_count"));
    assert!(json.contains("canary_promotion_soak_readback_pass_count"));
    assert!(json.contains("source_aware_front_door_disabled"));
    assert!(json.contains("operator_approval_missing"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"activation_command_present\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[tokio::test]
async fn store_context_plane_operator_approval_packet_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        ))
        .await
        .expect("put should succeed");
    store
        .put(memory_record(
            "memory-2",
            MemoryScope::Session,
            "session timeout summary",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "timeout surfaced during tool run",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-1",
            2,
            TranscriptEntryKind::Summary,
            "timeout retried successfully",
        ))
        .await
        .expect("append should succeed");

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };
    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_plane_operator_approval_packet(request.clone())
        .expect("context plane operator approval packet should succeed");

    assert_eq!(
        from_store,
        snapshot.context_plane_operator_approval_packet(&request)
    );
    assert!(from_store.has_packet_integrity());
    assert!(from_store.dry_run_only);
    assert!(from_store.approval_required);
    assert!(!from_store.activation_command_present);
    assert_eq!(from_store.matrix_row_count, 25);
    assert_eq!(from_store.threshold_satisfied_count, 9);
    assert_eq!(from_store.blocker_count, 16);
    assert_eq!(from_store.threshold_snapshot.required_ready_count, 24);
    assert_eq!(from_store.threshold_snapshot.required_shadow_count, 1);
    assert_eq!(from_store.required_scope_count(), 6);
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store
            .blocker_reason_count(ContextPlaneActivationBlockerReason::OperatorApprovalMissing),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowStoreShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowReplayShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalDiffShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalQualityShadowOnly
        ),
        Some(1)
    );
    assert_eq!(from_store.recall_quality_blocking_reason_count, 0);
    assert_eq!(from_store.recall_quality_blocking_reason_count_total(), 0);
    assert_eq!(from_store.canary_promotion_blocker_count, 0);
    assert_eq!(from_store.canary_promotion_checklist_required_count, 4);
    assert_eq!(from_store.canary_promotion_checklist_pass_count, 4);
    assert!(from_store.canary_promotion_readiness_check_pass);
    assert!(from_store.canary_promotion_negative_rehearsal_check_pass);
    assert!(from_store.canary_promotion_audit_digest_check_pass);
    assert!(from_store.canary_promotion_audit_freshness_check_pass);
    assert_eq!(from_store.canary_promotion_rollback_rehearsal_pass_count, 3);
    assert_eq!(
        from_store.canary_promotion_kill_switch_rehearsal_pass_count,
        3
    );
    assert_eq!(from_store.canary_promotion_soak_readback_pass_count, 3);
    assert_eq!(from_store.memory_provider_v2_lifecycle_required_count, 6);
    assert_eq!(from_store.memory_provider_v2_lifecycle_pass_count, 6);
    assert!(from_store.memory_provider_v2_query_check_pass);
    assert!(from_store.memory_provider_v2_update_context_check_pass);
    assert!(from_store.memory_provider_v2_propose_write_check_pass);
    assert!(from_store.memory_provider_v2_add_check_pass);
    assert!(from_store.memory_provider_v2_clear_check_pass);
    assert!(from_store.memory_provider_v2_close_check_pass);
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.adaptive_allocator_runtime_activation);
    assert!(!from_store.source_aware_runtime_activation);
    assert!(!from_store.prompt_assembly_change);
    assert!(!from_store.operator_activation_allowed);
}

#[tokio::test]
async fn store_context_plane_operator_approval_packet_rejects_activation_shaped_rehydration() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "timeout surfaced during tool run",
        ))
        .await
        .expect("append should succeed");

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };
    let packet = store
        .context_plane_operator_approval_packet(request)
        .expect("context plane operator approval packet should succeed");
    assert!(packet.has_packet_integrity());

    let mut command_shaped =
        serde_json::to_value(&packet).expect("packet should convert to json value");
    command_shaped
        .as_object_mut()
        .expect("packet json should be an object")
        .insert(
            "activation_command".into(),
            serde_json::Value::String("hepta context activate --now".into()),
        );
    assert!(serde_json::from_value::<ContextPlaneOperatorApprovalPacket>(command_shaped).is_err());

    let mut payload_shaped =
        serde_json::to_value(&packet).expect("packet should convert to json value");
    payload_shaped["threshold_snapshot"]
        .as_object_mut()
        .expect("threshold snapshot should be an object")
        .insert("tool_args".into(), serde_json::json!({"cmd": "activate"}));
    assert!(serde_json::from_value::<ContextPlaneOperatorApprovalPacket>(payload_shaped).is_err());

    let mut enabled_flag =
        serde_json::to_value(&packet).expect("packet should convert to json value");
    enabled_flag
        .as_object_mut()
        .expect("packet json should be an object")
        .insert("runtime_activation".into(), serde_json::Value::Bool(true));
    let parsed: ContextPlaneOperatorApprovalPacket =
        serde_json::from_value(enabled_flag).expect("known side-effect flag should deserialize");
    assert!(!parsed.has_packet_integrity());
}
