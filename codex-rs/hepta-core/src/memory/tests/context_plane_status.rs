use super::*;

fn context_plane_status_report_fixture(
    allocator_shadow: &ContextMemoryAdaptiveAllocatorEvalShadowReport,
    recall_quality_gate: &ContextMemoryRecallQualityGateReport,
) -> ContextPlaneStatusReport {
    let taxonomy = ContextMemoryTaxonomyReport {
        buckets: vec![
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Semantic,
                source_count: 1,
                returned_count: 1,
                available_count: 2,
                omitted_count: 1,
                provenance_span_count: 0,
            },
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Transcript,
                source_count: 1,
                returned_count: 1,
                available_count: 1,
                omitted_count: 0,
                provenance_span_count: 1,
            },
        ],
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
                "status-test",
                "1",
                "1",
            ]),
            privacy_class: "user_private".into(),
            queued_for_background: true,
            production_write: false,
        }],
    };
    let formation_queue = ContextMemoryFormationQueueReport::from_receipts(&formation_receipts);
    let namespace_policy = ContextMemoryNamespacePolicyReport::seeded();
    let write_chain_readiness =
        ContextMemoryWriteChainReadinessReport::from_namespace_policy(&namespace_policy);
    let write_chain_receipt_freshness =
        ContextMemoryWriteChainReceiptFreshnessReport::from_readiness(&write_chain_readiness);
    let temporal_facts = ContextMemoryTemporalFactReport {
        facts: vec![ContextMemoryTemporalFact {
            fact_type: ContextMemoryTemporalFactType::Attribute,
            entity_hash: stable_receipt_hash(&[
                "memory_temporal_fact_entity",
                "attribute",
                "status-test",
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
    let temporal_graph_shadow_eval = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let temporal_graph_shadow_store =
        ContextMemoryTemporalGraphShadowStoreReport::from_fact_graph(&temporal_fact_graph);
    let temporal_graph_shadow_replay =
        ContextMemoryTemporalGraphShadowReplayReport::from_shadow_store(
            &temporal_graph_shadow_store,
        );
    let temporal_graph_shadow_traversal_diff =
        ContextMemoryTemporalGraphShadowTraversalDiffReport::from_shadow_replay(
            &temporal_graph_shadow_replay,
        );
    let temporal_graph_shadow_traversal_quality =
        ContextMemoryTemporalGraphShadowTraversalQualityReport::from_traversal_diff(
            &temporal_graph_shadow_traversal_diff,
        );
    let temporal_graph_shadow_retrieval_canary_guard =
        ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport::from_traversal_quality(
            &temporal_graph_shadow_traversal_quality,
        );
    let temporal_graph_shadow_retrieval_rollback_kill_switch =
        ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport::from_retrieval_canary_guard(
            &temporal_graph_shadow_retrieval_canary_guard,
        );
    let eval_seed = ContextMemoryEvalHarnessReport::seeded();
    let provider_report = MemoryProviderReport::from_update(
        MemoryProviderDescriptor::builtin(),
        MemoryProviderContextUpdateEnvelope {
            provider_id: "builtin".into(),
            mode: MemoryProviderContextUpdateMode::ShadowOnly,
            source_counts: ContextRecallSourceCounts::default(),
            limit_pressure: ContextRecallLimitPressure::default(),
            ranked_item_count: 2,
            selected_item_count: 1,
            estimated_token_count: 256,
            payload_light: true,
            operator_approval_required: true,
            prompt_payload_exported: false,
            query_payload_exported: false,
            ranked_payload_exported: false,
            write_performed: false,
            runtime_activation: false,
        },
    );
    let provider_v2_write_proposal =
        MemoryProviderWriteProposalReport::from_formation_queue("builtin", &formation_queue);
    let provider_v2_audit = MemoryProviderV2AuditReport::from_parts(
        provider_report.descriptor.clone(),
        provider_report.update_context.clone(),
        provider_v2_write_proposal.clone(),
        MemoryProviderAddReport::blocked(&provider_v2_write_proposal),
        MemoryProviderClearReport::blocked("builtin", MemoryProviderClearScope::All),
        MemoryProviderCloseReport::shadow_noop("builtin"),
    );
    let ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let dashboard = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph_shadow_eval,
        recall_quality_gate,
        &provider_report,
    );
    let shadow_quality_summary =
        ContextMemoryShadowQualitySummaryReport::from_dashboard(&dashboard);
    let shadow_quality_trend_snapshot =
        ContextMemoryShadowQualityTrendSnapshotReport::from_summary(&shadow_quality_summary);
    let shadow_canary_promotion_readiness =
        ContextMemoryShadowCanaryPromotionReadinessReport::from_trend_snapshot(
            &shadow_quality_trend_snapshot,
        );

    ContextPlaneStatusReport::from_reports(ContextPlaneStatusReportInput {
        taxonomy: &taxonomy,
        formation_receipts: &formation_receipts,
        formation_queue: &formation_queue,
        namespace_policy: &namespace_policy,
        write_chain_readiness: &write_chain_readiness,
        write_chain_receipt_freshness: &write_chain_receipt_freshness,
        temporal_facts: &temporal_facts,
        temporal_fact_graph: &temporal_fact_graph,
        temporal_graph_shadow_eval: &temporal_graph_shadow_eval,
        temporal_graph_shadow_store: &temporal_graph_shadow_store,
        temporal_graph_shadow_replay: &temporal_graph_shadow_replay,
        temporal_graph_shadow_traversal_diff: &temporal_graph_shadow_traversal_diff,
        temporal_graph_shadow_traversal_quality: &temporal_graph_shadow_traversal_quality,
        temporal_graph_shadow_retrieval_canary_guard: &temporal_graph_shadow_retrieval_canary_guard,
        temporal_graph_shadow_retrieval_rollback_kill_switch:
            &temporal_graph_shadow_retrieval_rollback_kill_switch,
        eval_seed: &eval_seed,
        allocator_shadow,
        recall_quality_gate,
        ranked_recall: &ranked_recall,
        provider_report: &provider_report,
        provider_v2_audit: &provider_v2_audit,
        shadow_quality_trend_snapshot: &shadow_quality_trend_snapshot,
        shadow_canary_promotion_readiness: &shadow_canary_promotion_readiness,
    })
}

#[test]
fn context_plane_status_report_unifies_readiness_without_payloads_or_activation() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);

    assert!(report.has_status_integrity());
    assert_eq!(report.sections.len(), 26);
    assert_eq!(report.ready_section_count(), 8);
    assert_eq!(report.shadow_section_count(), 17);
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
        report.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalDiff),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(
            ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard
        ),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(
            ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch
        ),
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
    let temporal_graph_store_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryTemporalGraphShadowStore)
        .expect("temporal graph shadow store status row should exist");
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_node_count,
        1
    );
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_edge_count,
        2
    );
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_stage_required_count,
        6
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
        temporal_graph_store_entry
            .memory_temporal_graph_shadow_store_operator_approval_required_count,
        1
    );
    assert_eq!(
        temporal_graph_store_entry
            .memory_temporal_graph_shadow_store_operator_approval_recorded_count,
        0
    );
    assert_eq!(
        temporal_graph_store_entry.memory_temporal_graph_shadow_store_production_write_count,
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
        1
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_edge_count,
        2
    );
    assert_eq!(
        temporal_graph_replay_entry.memory_temporal_graph_shadow_replay_stage_required_count,
        6
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
        temporal_graph_replay_entry
            .memory_temporal_graph_shadow_replay_operator_approval_required_count,
        1
    );
    assert_eq!(
        temporal_graph_replay_entry
            .memory_temporal_graph_shadow_replay_operator_approval_recorded_count,
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
    let temporal_graph_traversal_diff_entry = report
        .sections
        .iter()
        .find(|entry| {
            entry.section == ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalDiff
        })
        .expect("temporal graph shadow traversal diff status row should exist");
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_production_selection_count,
        1
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count,
        2
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_stage_required_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_stage_projected_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_digest_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_llm_rerank_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_graph_persistence_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_production_route_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_production_write_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_diff_entry
            .memory_temporal_graph_shadow_traversal_diff_graph_write_count,
        0
    );
    let temporal_graph_traversal_quality_entry = report
        .sections
        .iter()
        .find(|entry| {
            entry.section == ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality
        })
        .expect("temporal graph shadow traversal quality status row should exist");
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_fixture_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_slo_pass_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_coverage_basis_points,
        10_000
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_precision_basis_points,
        10_000
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_leak_rate_basis_points,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_projected_latency_ms,
        1
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_token_saved_estimate,
        256
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_stage_projected_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_digest_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_llm_rerank_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_graph_persistence_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_production_route_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_production_write_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_entry
            .memory_temporal_graph_shadow_traversal_quality_graph_write_count,
        0
    );
    let temporal_graph_retrieval_canary_guard_entry = report
        .sections
        .iter()
        .find(|entry| {
            entry.section
                == ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard
        })
        .expect("temporal graph shadow retrieval canary guard status row should exist");
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_stage_projected_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_quality_slo_pass_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_required_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_recorded_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_enabled_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_ready_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_pass_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_activation_denial_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_digest_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_llm_rerank_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_production_route_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_entry
            .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count,
        0
    );
    let temporal_graph_retrieval_rollback_kill_switch_entry = report
        .sections
        .iter()
        .find(|entry| {
            entry.section
                == ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch
        })
        .expect("temporal graph shadow retrieval rollback/kill-switch status row should exist");
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count,
        6
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_projected_count,
        6
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_guard_pass_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_readback_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_readback_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_denial_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_route_opened_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_digest_count,
        6
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_llm_rerank_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_route_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_entry
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_count,
        0
    );
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
    assert!(ranked_recall_entry.ranked_recall_lexical_bm25_check_pass);
    assert!(ranked_recall_entry.ranked_recall_recency_check_pass);
    assert!(ranked_recall_entry.ranked_recall_source_authority_check_pass);
    assert!(ranked_recall_entry.ranked_recall_temporal_validity_check_pass);
    assert!(ranked_recall_entry.ranked_recall_feedback_check_pass);
    assert_eq!(
        ranked_recall_entry.ranked_recall_positive_hybrid_signal_pass_count,
        15
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_hybrid_regression_blocked_count,
        1
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_hybrid_signal_min_basis_points,
        6000
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_min_positive_hybrid_score_basis_points,
        7800
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_routing_diff_fixture_count,
        4
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_routing_diff_shadow_only_count,
        4
    );
    assert_eq!(ranked_recall_entry.ranked_recall_routing_diff_win_count, 3);
    assert_eq!(ranked_recall_entry.ranked_recall_routing_diff_loss_count, 1);
    assert_eq!(
        ranked_recall_entry.ranked_recall_min_positive_routing_diff_delta_basis_points,
        640
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_max_positive_routing_diff_latency_delta_ms,
        10
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points,
        3_000
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_fixture_count,
        4
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_shadow_only_count,
        4
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_slo_pass_count,
        3
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_win_count,
        3
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_loss_count,
        1
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_operator_review_required_count,
        4
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_total_leak_count,
        0
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_max_leak_rate_basis_points,
        0
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_min_positive_real_workload_trace_coverage_basis_points,
        8_000
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_min_positive_real_workload_trace_precision_basis_points,
        8_000
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_total_positive_real_workload_trace_token_saved,
        2_140
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_max_positive_real_workload_trace_latency_ms,
        55
    );
    assert_eq!(
        ranked_recall_entry.ranked_recall_real_workload_trace_regression_loss_count,
        1
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
        namespace_policy_entry.memory_namespace_policy_readback_required_count,
        6
    );
    assert_eq!(
        namespace_policy_entry.memory_namespace_policy_canary_required_count,
        6
    );
    assert_eq!(
        namespace_policy_entry.memory_namespace_policy_rollback_supported_count,
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
    assert_eq!(
        write_chain_entry.memory_write_chain_propose_write_ready_count,
        6
    );
    assert_eq!(
        write_chain_entry.memory_write_chain_policy_approval_ready_count,
        6
    );
    assert_eq!(
        write_chain_entry.memory_write_chain_operator_approval_ready_count,
        6
    );
    assert_eq!(
        write_chain_entry.memory_write_chain_shadow_wal_ready_count,
        6
    );
    assert_eq!(write_chain_entry.memory_write_chain_readback_ready_count, 6);
    assert_eq!(write_chain_entry.memory_write_chain_canary_ready_count, 6);
    assert_eq!(write_chain_entry.memory_write_chain_rollback_ready_count, 6);
    assert_eq!(
        write_chain_entry.memory_write_chain_production_write_count,
        0
    );
    assert_eq!(write_chain_entry.memory_write_chain_graph_write_count, 0);
    let receipt_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness)
        .expect("memory write-chain receipt freshness status row should exist");
    assert_eq!(receipt_entry.memory_write_chain_receipt_namespace_count, 6);
    assert_eq!(receipt_entry.memory_write_chain_receipt_required_count, 18);
    assert_eq!(receipt_entry.memory_write_chain_receipt_projected_count, 18);
    assert_eq!(receipt_entry.memory_write_chain_receipt_digest_count, 6);
    assert_eq!(
        receipt_entry.memory_write_chain_receipt_freshness_pass_count,
        6
    );
    assert_eq!(
        receipt_entry.memory_write_chain_receipt_replay_guard_pass_count,
        6
    );
    assert_eq!(
        receipt_entry.memory_write_chain_receipt_stale_replay_rejected_count,
        6
    );
    assert_eq!(receipt_entry.memory_write_chain_receipt_recorded_count, 0);
    assert_eq!(receipt_entry.memory_write_chain_receipt_persisted_count, 0);
    assert_eq!(
        receipt_entry.memory_write_chain_receipt_production_write_count,
        0
    );
    assert_eq!(
        receipt_entry.memory_write_chain_receipt_graph_write_count,
        0
    );
    assert_eq!(
        namespace_policy_entry.memory_namespace_policy_graph_write_count,
        0
    );
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
    assert!(json.contains("memory_write_chain_receipt_freshness"));
    assert!(json.contains("memory_write_chain_receipt_projected_count"));
    assert!(json.contains("memory_write_chain_receipt_digest_count"));
    assert!(json.contains("memory_write_chain_receipt_freshness_pass_count"));
    assert!(json.contains("memory_write_chain_receipt_stale_replay_rejected_count"));
    assert!(json.contains("memory_temporal_facts"));
    assert!(json.contains("memory_temporal_fact_graph"));
    assert!(json.contains("memory_temporal_graph_shadow_eval"));
    assert!(json.contains("memory_temporal_graph_shadow_retrieval_canary_guard"));
    assert!(json.contains(
        "memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count"
    ));
    assert!(
        json.contains(
            "memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count"
        )
    );
    assert!(json.contains("eval_harness_seed"));
    assert!(json.contains("adaptive_allocator_eval_shadow"));
    assert!(json.contains("recall_quality_gate"));
    assert!(json.contains("memory_ranked_recall_shadow_eval"));
    assert!(json.contains("ranked_recall_hybrid_signal_pass_count"));
    assert!(json.contains("ranked_recall_lexical_bm25_check_pass"));
    assert!(json.contains("ranked_recall_temporal_validity_check_pass"));
    assert!(json.contains("ranked_recall_positive_hybrid_signal_pass_count"));
    assert!(json.contains("ranked_recall_hybrid_regression_blocked_count"));
    assert!(json.contains("ranked_recall_routing_diff_shadow_only_count"));
    assert!(json.contains("ranked_recall_min_positive_routing_diff_delta_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points"));
    assert!(json.contains("ranked_recall_real_workload_trace_slo_pass_count"));
    assert!(json.contains("ranked_recall_real_workload_trace_total_leak_count"));
    assert!(json.contains("ranked_recall_min_positive_real_workload_trace_coverage_basis_points"));
    assert!(json.contains("ranked_recall_real_workload_trace_operator_review_required_count"));
    assert!(json.contains("memory_provider_boundary"));
    assert!(json.contains("memory_provider_v2_boundary"));
    assert!(json.contains("memory_provider_v2_lifecycle_pass_count"));
    assert!(json.contains("memory_provider_v2_propose_write_check_pass"));
    assert!(json.contains("memory_provider_v2_close_check_pass"));
    assert!(json.contains("memory_namespace_policy_namespace_count"));
    assert!(json.contains("memory_namespace_policy_shadow_wal_required_count"));
    assert!(json.contains("memory_namespace_policy_operator_approval_required_count"));
    assert!(json.contains("memory_namespace_policy_production_write_count"));
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
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("status-test"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_plane_status_report_rejects_ranked_recall_hybrid_false_green() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);
    assert!(report.has_status_integrity());

    let mut partial_signal = report.clone();
    partial_signal
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval status row should exist")
        .ranked_recall_feedback_check_pass = false;
    assert!(!partial_signal.has_status_integrity());

    let mut inflated_pass_count = report.clone();
    inflated_pass_count
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval status row should exist")
        .ranked_recall_hybrid_signal_pass_count = 6;
    assert!(!inflated_pass_count.has_status_integrity());

    let mut low_score_false_green = report.clone();
    low_score_false_green
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval status row should exist")
        .ranked_recall_min_positive_hybrid_score_basis_points = 5999;
    assert!(!low_score_false_green.has_status_integrity());

    let mut routing_diff_replay = report.clone();
    routing_diff_replay
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval status row should exist")
        .ranked_recall_routing_diff_shadow_only_count = 3;
    assert!(!routing_diff_replay.has_status_integrity());

    let mut slo_false_green = report.clone();
    slo_false_green
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval status row should exist")
        .ranked_recall_real_workload_trace_total_leak_count = 1;
    assert!(!slo_false_green.has_status_integrity());

    let mut non_ranked_leak = report.clone();
    non_ranked_leak
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist")
        .ranked_recall_hybrid_signal_pass_count = 1;
    assert!(!non_ranked_leak.has_status_integrity());
}

#[test]
fn context_plane_status_report_rejects_temporal_graph_retrieval_canary_guard_false_green() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);
    assert!(report.has_status_integrity());

    let mut partial_feature_flag = report.clone();
    partial_feature_flag
        .sections
        .iter_mut()
        .find(|entry| {
            entry.section
                == ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard
        })
        .expect("retrieval canary guard status row should exist")
        .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count = 4;
    assert!(!partial_feature_flag.has_status_integrity());

    let mut canary_route_opened = report.clone();
    canary_route_opened
        .sections
        .iter_mut()
        .find(|entry| {
            entry.section
                == ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard
        })
        .expect("retrieval canary guard status row should exist")
        .memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count = 1;
    assert!(!canary_route_opened.has_status_integrity());

    let mut rollback_write = report.clone();
    rollback_write
        .sections
        .iter_mut()
        .find(|entry| {
            entry.section
                == ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard
        })
        .expect("retrieval canary guard status row should exist")
        .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count = 1;
    assert!(!rollback_write.has_status_integrity());

    let mut non_guard_leak = report.clone();
    non_guard_leak
        .sections
        .iter_mut()
        .find(|entry| {
            entry.section == ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality
        })
        .expect("traversal quality status row should exist")
        .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count = 5;
    assert!(!non_guard_leak.has_status_integrity());
}

#[test]
fn context_plane_status_report_rejects_canary_promotion_checklist_false_green() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);
    assert!(report.has_status_integrity());

    let mut partial_rehearsal = report.clone();
    partial_rehearsal
        .sections
        .iter_mut()
        .find(|entry| {
            entry.section == ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness
        })
        .expect("memory shadow canary promotion readiness status row should exist")
        .canary_promotion_rollback_rehearsal_pass_count = 2;
    assert!(!partial_rehearsal.has_status_integrity());

    let mut blocker_false_green = report.clone();
    let promotion_entry = blocker_false_green
        .sections
        .iter_mut()
        .find(|entry| {
            entry.section == ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness
        })
        .expect("memory shadow canary promotion readiness status row should exist");
    promotion_entry.status = ContextPlaneStatusKind::Blocked;
    promotion_entry.blocker_count = 1;
    promotion_entry.canary_promotion_blocker_count = 1;
    assert!(!blocker_false_green.has_status_integrity());

    let mut non_promotion_leak = report.clone();
    non_promotion_leak
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::SourceRegistry)
        .expect("source registry status row should exist")
        .canary_promotion_checklist_pass_count = 1;
    assert!(!non_promotion_leak.has_status_integrity());
}

#[test]
fn context_plane_status_report_rejects_memory_provider_v2_lifecycle_false_green() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);
    assert!(report.has_status_integrity());

    let mut partial_lifecycle = report.clone();
    partial_lifecycle
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryProviderV2Boundary)
        .expect("memory provider v2 boundary status row should exist")
        .memory_provider_v2_close_check_pass = false;
    assert!(!partial_lifecycle.has_status_integrity());

    let mut inflated_pass_count = report.clone();
    inflated_pass_count
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryProviderV2Boundary)
        .expect("memory provider v2 boundary status row should exist")
        .memory_provider_v2_lifecycle_pass_count = 7;
    assert!(!inflated_pass_count.has_status_integrity());

    let mut non_provider_v2_leak = report.clone();
    non_provider_v2_leak
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryProviderBoundary)
        .expect("memory provider boundary status row should exist")
        .memory_provider_v2_lifecycle_pass_count = 1;
    assert!(!non_provider_v2_leak.has_status_integrity());
}

#[test]
fn context_plane_status_report_rejects_namespace_policy_false_green() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);
    assert!(report.has_status_integrity());

    let mut partial_policy = report.clone();
    partial_policy
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryNamespacePolicy)
        .expect("memory namespace policy status row should exist")
        .memory_namespace_policy_shadow_wal_required_count = 5;
    assert!(!partial_policy.has_status_integrity());

    let mut write_false_green = report.clone();
    write_false_green
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryNamespacePolicy)
        .expect("memory namespace policy status row should exist")
        .memory_namespace_policy_production_write_count = 1;
    assert!(!write_false_green.has_status_integrity());

    let mut non_policy_leak = report.clone();
    non_policy_leak
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryProviderBoundary)
        .expect("memory provider boundary status row should exist")
        .memory_namespace_policy_namespace_count = 6;
    assert!(!non_policy_leak.has_status_integrity());
}

#[test]
fn context_plane_status_report_rejects_memory_write_chain_false_green() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);
    assert!(report.has_status_integrity());

    let mut partial_readback = report.clone();
    partial_readback
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness status row should exist")
        .memory_write_chain_readback_ready_count = 5;
    assert!(!partial_readback.has_status_integrity());

    let mut partial_canary = report.clone();
    partial_canary
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness status row should exist")
        .memory_write_chain_canary_ready_count = 5;
    assert!(!partial_canary.has_status_integrity());

    let mut write_false_green = report.clone();
    write_false_green
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness status row should exist")
        .memory_write_chain_production_write_count = 1;
    assert!(!write_false_green.has_status_integrity());

    let mut non_write_chain_leak = report.clone();
    non_write_chain_leak
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryNamespacePolicy)
        .expect("memory namespace policy status row should exist")
        .memory_write_chain_stage_pass_count = 6;
    assert!(!non_write_chain_leak.has_status_integrity());
}

#[test]
fn context_plane_status_report_rejects_memory_write_chain_receipt_false_green() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);

    let mut stale_receipt = report.clone();
    stale_receipt
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness)
        .expect("memory write-chain receipt freshness status row should exist")
        .memory_write_chain_receipt_freshness_pass_count = 5;
    assert!(!stale_receipt.has_status_integrity());

    let mut digest_drift = report.clone();
    digest_drift
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness)
        .expect("memory write-chain receipt freshness status row should exist")
        .memory_write_chain_receipt_digest_count = 5;
    assert!(!digest_drift.has_status_integrity());

    let mut persistence_drift = report.clone();
    persistence_drift
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness)
        .expect("memory write-chain receipt freshness status row should exist")
        .memory_write_chain_receipt_persisted_count = 1;
    assert!(!persistence_drift.has_status_integrity());

    let mut non_receipt_leak = report.clone();
    non_receipt_leak
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness status row should exist")
        .memory_write_chain_receipt_projected_count = 18;
    assert!(!non_receipt_leak.has_status_integrity());
}

#[test]
fn context_plane_status_report_rolls_up_recall_quality_blockers_without_payloads() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let mut recall_quality_shadow = allocator_shadow.clone();
    let proposed_redacted = recall_quality_shadow
        .shadow_results
        .iter_mut()
        .find(|result| {
            result.arm == ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive
                && result.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace
        })
        .expect("proposed redacted trace fixture should exist");
    proposed_redacted.answer_quality_regression_count = 2;
    proposed_redacted.prompt_assembly_change = true;

    let recall_quality_gate =
        ContextMemoryRecallQualityGateReport::from_shadow(&recall_quality_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);

    assert!(!report.has_status_integrity());
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::RecallQualityGate),
        Some(ContextPlaneStatusKind::Blocked)
    );
    assert_eq!(report.blocker_count(), 40);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.adaptive_allocator_runtime_activation);
    assert!(!report.source_aware_runtime_activation);
    assert!(report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let recall_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist");
    assert_eq!(recall_entry.status, ContextPlaneStatusKind::Blocked);
    assert_eq!(recall_entry.blocker_count, 2);
    assert_eq!(recall_entry.recall_quality_blocking_reason_count, 2);
    assert_eq!(
        recall_entry.recall_quality_blocking_reasons,
        vec![
            ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression,
            ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled,
        ]
    );
    assert!(!recall_entry.production_write);
    assert!(!recall_entry.graph_write);
    assert!(!recall_entry.runtime_activation);
    assert!(recall_entry.prompt_assembly_change);
    assert!(!recall_entry.operator_activation_allowed);
    let canary_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryShadowCanaryReadiness)
        .expect("memory shadow canary readiness status row should exist");
    assert_eq!(canary_entry.status, ContextPlaneStatusKind::Blocked);
    assert_eq!(canary_entry.blocker_count, 19);
    assert_eq!(canary_entry.omitted_count, 19);
    let promotion_entry = report
        .sections
        .iter()
        .find(|entry| {
            entry.section == ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness
        })
        .expect("memory shadow canary promotion readiness status row should exist");
    assert_eq!(promotion_entry.status, ContextPlaneStatusKind::Blocked);
    assert_eq!(promotion_entry.blocker_count, 19);
    assert_eq!(promotion_entry.canary_promotion_blocker_count, 19);

    let json = serde_json::to_string(&report).expect("context plane status should serialize");
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("answer_quality_regression"));
    assert!(json.contains("side_effect_flag_enabled"));
    assert!(json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("fixture_id_hash"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("status-test"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}
