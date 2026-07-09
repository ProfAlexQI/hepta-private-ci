use super::*;

#[test]
fn store_snapshot_context_plane_status_report_is_payload_light() {
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

    let report = snapshot.context_plane_status_report(&request);

    assert!(report.has_status_integrity());
    assert_eq!(report.sections.len(), 22);
    assert_eq!(report.ready_section_count(), 8);
    assert_eq!(report.shadow_section_count(), 13);
    assert_eq!(report.disabled_section_count(), 1);
    assert_eq!(report.blocker_count(), 0);
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::SourceRegistry),
        Some(ContextPlaneStatusKind::Ready)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::AdaptiveBudgetAllocation),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::RecallQualityGate),
        Some(ContextPlaneStatusKind::Ready)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryRankedRecallShadowEval),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryNamespacePolicy),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryWriteChainReadiness),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryProviderBoundary),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryProviderV2Boundary),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryShadowCanaryReadiness),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowEval),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowStore),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::SourceAwareFrontDoor),
        Some(ContextPlaneStatusKind::Disabled)
    );
    let recall_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist");
    assert_eq!(recall_entry.recall_quality_blocking_reason_count, 0);
    assert!(recall_entry.recall_quality_blocking_reasons.is_empty());
    let ranked_recall_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval status row should exist");
    assert_eq!(
        ranked_recall_entry.ranked_recall_hybrid_signal_required_count,
        5
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_hybrid_signal_pass_count,
        5
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_positive_hybrid_signal_pass_count,
        15
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_hybrid_regression_blocked_count,
        1
    );
    let temporal_graph_store_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryTemporalGraphShadowStore)
        .expect("temporal graph shadow store status row should exist");
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_node_count,
        5
    );
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_edge_count,
        10
    );
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_stage_projected_count,
        6
    );
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_digest_count,
        1
    );
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_recorded_receipt_count,
        0
    );
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_graph_write_count,
        0
    );
    let temporal_graph_replay_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay)
        .expect("temporal graph shadow replay status row should exist");
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_node_count,
        5
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_edge_count,
        10
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_provenance_count,
        5
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_bitemporal_validity_count,
        5
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_stage_projected_count,
        6
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_digest_count,
        6
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_freshness_pass_count,
        6
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_guard_pass_count,
        6
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_stale_replay_rejected_count,
        6
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_recorded_receipt_count,
        0
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_persisted_receipt_count,
        0
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_production_write_count,
        0
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_graph_write_count,
        0
    );
    let promotion_entry = report
        .sections
        .iter()
        .find(|entry| {
            entry.section == ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness
        })
        .expect("memory shadow canary promotion readiness status row should exist");
    assert_eq!(promotion_entry.canary_promotion_checklist_required_count, 4);
    assert_eq!(promotion_entry.canary_promotion_checklist_pass_count, 4);
    assert!(promotion_entry.canary_promotion_readiness_check_pass);
    assert!(promotion_entry.canary_promotion_negative_rehearsal_check_pass);
    assert!(promotion_entry.canary_promotion_audit_digest_check_pass);
    assert!(promotion_entry.canary_promotion_audit_freshness_check_pass);
    let provider_v2_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryProviderV2Boundary)
        .expect("memory provider v2 boundary status row should exist");
    assert_eq!(
        provider_v2_entry.memory_provider_v2_lifecycle_required_count,
        6
    );
    assert_eq!(provider_v2_entry.memory_provider_v2_lifecycle_pass_count, 6);
    assert!(provider_v2_entry.memory_provider_v2_query_check_pass);
    assert!(provider_v2_entry.memory_provider_v2_update_context_check_pass);
    assert!(provider_v2_entry.memory_provider_v2_propose_write_check_pass);
    assert!(provider_v2_entry.memory_provider_v2_add_check_pass);
    assert!(provider_v2_entry.memory_provider_v2_clear_check_pass);
    assert!(provider_v2_entry.memory_provider_v2_close_check_pass);
    let namespace_policy_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryNamespacePolicy)
        .expect("memory namespace policy status row should exist");
    assert_eq!(
        namespace_policy_entry.memory_namespace_policy_namespace_count,
        6
    );
    assert_eq!(
        namespace_policy_entry.memory_namespace_policy_operator_approval_required_count,
        6
    );
    assert_eq!(
        namespace_policy_entry.memory_namespace_policy_shadow_wal_required_count,
        6
    );
    assert_eq!(
        namespace_policy_entry.memory_namespace_policy_production_write_count,
        0
    );
    let write_chain_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness status row should exist");
    assert_eq!(write_chain_entry.memory_write_chain_namespace_count, 6);
    assert_eq!(write_chain_entry.memory_write_chain_stage_required_count, 6);
    assert_eq!(write_chain_entry.memory_write_chain_stage_pass_count, 6);
    assert_eq!(write_chain_entry.memory_write_chain_readback_ready_count, 6);
    assert_eq!(write_chain_entry.memory_write_chain_canary_ready_count, 6);
    assert_eq!(
        write_chain_entry.memory_write_chain_production_write_count,
        0
    );
    assert_eq!(write_chain_entry.memory_write_chain_graph_write_count, 0);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.adaptive_allocator_runtime_activation);
    assert!(!report.source_aware_runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let json = serde_json::to_string(&report).expect("context plane status should serialize");
    assert!(json.contains("source_registry"));
    assert!(json.contains("adaptive_budget_allocation"));
    assert!(json.contains("memory_taxonomy"));
    assert!(json.contains("memory_formation_receipts"));
    assert!(json.contains("memory_formation_queue"));
    assert!(json.contains("memory_namespace_policy"));
    assert!(json.contains("memory_write_chain_readiness"));
    assert!(json.contains("memory_write_chain_stage_pass_count"));
    assert!(json.contains("memory_write_chain_readback_ready_count"));
    assert!(json.contains("memory_write_chain_canary_ready_count"));
    assert!(json.contains("memory_temporal_facts"));
    assert!(json.contains("memory_temporal_fact_graph"));
    assert!(json.contains("memory_temporal_graph_shadow_eval"));
    assert!(json.contains("memory_temporal_graph_shadow_replay"));
    assert!(json.contains("memory_temporal_graph_shadow_replay_stage_projected_count"));
    assert!(json.contains("memory_temporal_graph_shadow_replay_stale_replay_rejected_count"));
    assert!(json.contains("eval_harness_seed"));
    assert!(json.contains("adaptive_allocator_eval_shadow"));
    assert!(json.contains("recall_quality_gate"));
    assert!(json.contains("memory_ranked_recall_shadow_eval"));
    assert!(json.contains("ranked_recall_hybrid_signal_pass_count"));
    assert!(json.contains("ranked_recall_positive_hybrid_signal_pass_count"));
    assert!(json.contains("ranked_recall_hybrid_regression_blocked_count"));
    assert!(json.contains("memory_provider_boundary"));
    assert!(json.contains("memory_provider_v2_boundary"));
    assert!(json.contains("memory_provider_v2_lifecycle_pass_count"));
    assert!(json.contains("memory_provider_v2_propose_write_check_pass"));
    assert!(json.contains("memory_provider_v2_close_check_pass"));
    assert!(json.contains("memory_namespace_policy_namespace_count"));
    assert!(json.contains("memory_namespace_policy_shadow_wal_required_count"));
    assert!(json.contains("memory_namespace_policy_operator_approval_required_count"));
    assert!(json.contains("memory_shadow_canary_readiness"));
    assert!(json.contains("memory_shadow_canary_promotion_readiness"));
    assert!(json.contains("canary_promotion_required_stable_window_count"));
    assert!(json.contains("canary_promotion_checklist_pass_count"));
    assert!(json.contains("canary_promotion_negative_rehearsal_check_pass"));
    assert!(json.contains("canary_promotion_audit_digest_check_pass"));
    assert!(json.contains("canary_promotion_audit_freshness_check_pass"));
    assert!(json.contains("canary_promotion_rollback_rehearsal_pass_count"));
    assert!(json.contains("canary_promotion_kill_switch_rehearsal_pass_count"));
    assert!(json.contains("canary_promotion_soak_readback_pass_count"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("source_aware_front_door"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[tokio::test]
async fn store_context_plane_status_report_matches_snapshot_helper() {
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
        .context_plane_status_report(request.clone())
        .expect("context plane status should succeed");

    assert_eq!(from_store, snapshot.context_plane_status_report(&request));
    assert!(from_store.has_status_integrity());
    assert_eq!(from_store.sections.len(), 22);
    assert_eq!(from_store.blocker_count(), 0);
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::RecallQualityGate),
        Some(ContextPlaneStatusKind::Ready)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryRankedRecallShadowEval),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryProviderBoundary),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryProviderV2Boundary),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryShadowCanaryReadiness),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowEval),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowStore),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::SourceAwareFrontDoor),
        Some(ContextPlaneStatusKind::Disabled)
    );
    let recall_entry = from_store
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist");
    assert_eq!(recall_entry.recall_quality_blocking_reason_count, 0);
    assert!(recall_entry.recall_quality_blocking_reasons.is_empty());
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.adaptive_allocator_runtime_activation);
    assert!(!from_store.source_aware_runtime_activation);
    assert!(!from_store.prompt_assembly_change);
    assert!(!from_store.operator_activation_allowed);
}
