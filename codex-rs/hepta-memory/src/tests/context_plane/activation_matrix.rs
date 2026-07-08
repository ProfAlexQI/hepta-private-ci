use super::*;

#[test]
fn store_snapshot_context_plane_activation_blocker_matrix_is_payload_light() {
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

    let matrix = snapshot.context_plane_activation_blocker_matrix(&request);

    assert!(matrix.has_matrix_integrity());
    assert_eq!(matrix.rows.len(), 17);
    assert_eq!(matrix.satisfied_count(), 9);
    assert_eq!(matrix.blocker_count, 8);
    assert!(!matrix.activation_allowed);
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::AdaptiveBudgetAllocation),
        Some(ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::SourceAwareFrontDoor),
        Some(ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryProviderBoundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryProviderV2Boundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryShadowCanaryReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::OperatorApproval),
        Some(ContextPlaneActivationBlockerReason::OperatorApprovalMissing)
    );
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::EvalHarnessSeed),
        Some(true)
    );
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow),
        Some(true)
    );
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::RecallQualityGate),
        Some(true)
    );
    let recall_quality_row = matrix
        .row_for_target(ContextPlaneActivationTarget::RecallQualityGate)
        .expect("recall quality activation row should exist");
    assert_eq!(recall_quality_row.recall_quality_blocking_reason_count, 0);
    assert!(
        recall_quality_row
            .recall_quality_blocking_reasons
            .is_empty()
    );
    let promotion_row = matrix
        .row_for_target(ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness)
        .expect("memory shadow canary promotion readiness row should exist");
    assert_eq!(promotion_row.canary_promotion_checklist_required_count, 4);
    assert_eq!(promotion_row.canary_promotion_checklist_pass_count, 4);
    assert!(promotion_row.canary_promotion_readiness_check_pass);
    assert!(promotion_row.canary_promotion_negative_rehearsal_check_pass);
    assert!(promotion_row.canary_promotion_audit_digest_check_pass);
    assert!(promotion_row.canary_promotion_audit_freshness_check_pass);
    let provider_v2_row = matrix
        .row_for_target(ContextPlaneActivationTarget::MemoryProviderV2Boundary)
        .expect("memory provider v2 activation row should exist");
    assert_eq!(
        provider_v2_row.memory_provider_v2_lifecycle_required_count,
        6
    );
    assert_eq!(provider_v2_row.memory_provider_v2_lifecycle_pass_count, 6);
    assert!(provider_v2_row.memory_provider_v2_query_check_pass);
    assert!(provider_v2_row.memory_provider_v2_update_context_check_pass);
    assert!(provider_v2_row.memory_provider_v2_propose_write_check_pass);
    assert!(provider_v2_row.memory_provider_v2_add_check_pass);
    assert!(provider_v2_row.memory_provider_v2_clear_check_pass);
    assert!(provider_v2_row.memory_provider_v2_close_check_pass);

    let json = serde_json::to_string(&matrix).expect("activation blocker matrix should serialize");
    assert!(json.contains("recall_quality_gate"));
    assert!(json.contains("memory_temporal_graph_shadow_eval"));
    assert!(json.contains("memory_provider_boundary"));
    assert!(json.contains("memory_provider_v2_boundary"));
    assert!(json.contains("memory_provider_v2_lifecycle_pass_count"));
    assert!(json.contains("memory_provider_v2_propose_write_check_pass"));
    assert!(json.contains("memory_provider_v2_close_check_pass"));
    assert!(json.contains("memory_shadow_canary_readiness"));
    assert!(json.contains("memory_shadow_canary_promotion_readiness"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("adaptive_budget_allocation_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_eval_shadow_only"));
    assert!(json.contains("memory_provider_boundary_shadow_only"));
    assert!(json.contains("memory_provider_v2_boundary_shadow_only"));
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
    assert!(!json.contains("\"activation_allowed\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[tokio::test]
async fn store_context_plane_activation_blocker_matrix_matches_snapshot_helper() {
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
        .context_plane_activation_blocker_matrix(request.clone())
        .expect("context plane activation blocker matrix should succeed");

    assert_eq!(
        from_store,
        snapshot.context_plane_activation_blocker_matrix(&request)
    );
    assert!(from_store.has_matrix_integrity());
    assert_eq!(from_store.rows.len(), 17);
    assert_eq!(from_store.satisfied_count(), 9);
    assert_eq!(from_store.blocker_count, 8);
    assert_eq!(
        from_store.threshold_satisfied(ContextPlaneActivationTarget::RecallQualityGate),
        Some(true)
    );
    let recall_quality_row = from_store
        .row_for_target(ContextPlaneActivationTarget::RecallQualityGate)
        .expect("recall quality activation row should exist");
    assert_eq!(recall_quality_row.recall_quality_blocking_reason_count, 0);
    assert!(
        recall_quality_row
            .recall_quality_blocking_reasons
            .is_empty()
    );
    assert_eq!(
        from_store.blocker_reason(ContextPlaneActivationTarget::AdaptiveBudgetAllocation),
        Some(ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly)
    );
    assert_eq!(
        from_store.blocker_reason(ContextPlaneActivationTarget::SourceAwareFrontDoor),
        Some(ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled)
    );
    assert_eq!(
        from_store.blocker_reason(ContextPlaneActivationTarget::MemoryProviderBoundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly)
    );
    assert_eq!(
        from_store.blocker_reason(ContextPlaneActivationTarget::MemoryProviderV2Boundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly)
    );
    assert_eq!(
        from_store.blocker_reason(ContextPlaneActivationTarget::MemoryShadowCanaryReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly)
    );
    assert_eq!(
        from_store
            .blocker_reason(ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly)
    );
    assert_eq!(
        from_store.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly)
    );
    assert_eq!(
        from_store.blocker_reason(ContextPlaneActivationTarget::OperatorApproval),
        Some(ContextPlaneActivationBlockerReason::OperatorApprovalMissing)
    );
    assert!(!from_store.activation_allowed);
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.adaptive_allocator_runtime_activation);
    assert!(!from_store.source_aware_runtime_activation);
    assert!(!from_store.prompt_assembly_change);
    assert!(!from_store.operator_activation_allowed);
}
