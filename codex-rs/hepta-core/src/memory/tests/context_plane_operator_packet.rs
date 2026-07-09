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
    let eval_seed = ContextMemoryEvalHarnessReport::seeded();
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::from_seed(&eval_seed);
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
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
        &recall_quality_gate,
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
    let status = ContextPlaneStatusReport::from_reports(ContextPlaneStatusReportInput {
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
        eval_seed: &eval_seed,
        allocator_shadow: &allocator_shadow,
        recall_quality_gate: &recall_quality_gate,
        ranked_recall: &ranked_recall,
        provider_report: &provider_report,
        provider_v2_audit: &provider_v2_audit,
        shadow_quality_trend_snapshot: &shadow_quality_trend_snapshot,
        shadow_canary_promotion_readiness: &shadow_canary_promotion_readiness,
    });
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);

    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);

    assert!(packet.has_packet_integrity());
    assert!(packet.dry_run_only);
    assert!(packet.approval_required);
    assert!(!packet.activation_command_present);
    assert_eq!(packet.matrix_row_count, 25);
    assert_eq!(packet.threshold_satisfied_count, 9);
    assert_eq!(packet.blocker_count, 16);
    assert_eq!(packet.threshold_snapshot.total_row_count, 25);
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
    assert_eq!(packet.memory_namespace_policy_readback_required_count, 6);
    assert_eq!(packet.memory_namespace_policy_canary_required_count, 6);
    assert_eq!(packet.memory_namespace_policy_rollback_supported_count, 6);
    assert_eq!(packet.memory_namespace_policy_production_write_count, 0);
    assert_eq!(packet.memory_namespace_policy_graph_write_count, 0);
    assert_eq!(packet.memory_write_chain_namespace_count, 6);
    assert_eq!(packet.memory_write_chain_stage_required_count, 6);
    assert_eq!(packet.memory_write_chain_stage_pass_count, 6);
    assert_eq!(packet.memory_write_chain_propose_write_ready_count, 6);
    assert_eq!(packet.memory_write_chain_policy_approval_ready_count, 6);
    assert_eq!(packet.memory_write_chain_operator_approval_ready_count, 6);
    assert_eq!(packet.memory_write_chain_shadow_wal_ready_count, 6);
    assert_eq!(packet.memory_write_chain_readback_ready_count, 6);
    assert_eq!(packet.memory_write_chain_canary_ready_count, 6);
    assert_eq!(packet.memory_write_chain_rollback_ready_count, 6);
    assert_eq!(packet.memory_write_chain_production_write_count, 0);
    assert_eq!(packet.memory_write_chain_graph_write_count, 0);
    assert_eq!(packet.memory_temporal_graph_shadow_store_node_count, 1);
    assert_eq!(packet.memory_temporal_graph_shadow_store_edge_count, 2);
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_provenance_edge_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_validity_window_edge_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_stage_required_count,
        6
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_stage_projected_count,
        6
    );
    assert_eq!(packet.memory_temporal_graph_shadow_store_digest_count, 1);
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_freshness_pass_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_replay_guard_pass_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_stale_replay_rejected_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_operator_approval_required_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_operator_approval_recorded_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_recorded_receipt_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_persisted_receipt_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_production_write_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_store_graph_write_count,
        0
    );
    assert_eq!(packet.memory_temporal_graph_shadow_replay_node_count, 1);
    assert_eq!(packet.memory_temporal_graph_shadow_replay_edge_count, 2);
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_provenance_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_bitemporal_validity_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_stage_required_count,
        6
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_stage_projected_count,
        6
    );
    assert_eq!(packet.memory_temporal_graph_shadow_replay_digest_count, 6);
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_freshness_pass_count,
        6
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_guard_pass_count,
        6
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_stale_replay_rejected_count,
        6
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_operator_approval_required_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_operator_approval_recorded_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_recorded_receipt_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_persisted_receipt_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_production_write_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_replay_graph_write_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_production_selection_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_lexical_bm25_candidate_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_semantic_candidate_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count,
        2
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_hybrid_candidate_count,
        2
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_overlap_candidate_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_win_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_loss_count,
        0
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_cost_count,
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_stage_required_count,
        5
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
        packet.memory_temporal_graph_shadow_traversal_diff_freshness_pass_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_replay_guard_pass_count,
        5
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_diff_stale_replay_rejected_count,
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
        1
    );
    assert_eq!(
        packet.memory_temporal_graph_shadow_traversal_quality_token_saved_estimate,
        256
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
    assert!(packet.ranked_recall_lexical_bm25_check_pass);
    assert!(packet.ranked_recall_recency_check_pass);
    assert!(packet.ranked_recall_source_authority_check_pass);
    assert!(packet.ranked_recall_temporal_validity_check_pass);
    assert!(packet.ranked_recall_feedback_check_pass);
    assert_eq!(packet.ranked_recall_positive_hybrid_signal_pass_count, 15);
    assert_eq!(packet.ranked_recall_hybrid_regression_blocked_count, 1);
    assert_eq!(packet.ranked_recall_hybrid_signal_min_basis_points, 6000);
    assert_eq!(
        packet.ranked_recall_min_positive_hybrid_score_basis_points,
        7800
    );
    assert_eq!(packet.ranked_recall_routing_diff_fixture_count, 4);
    assert_eq!(packet.ranked_recall_routing_diff_shadow_only_count, 4);
    assert_eq!(packet.ranked_recall_routing_diff_win_count, 3);
    assert_eq!(packet.ranked_recall_routing_diff_loss_count, 1);
    assert_eq!(
        packet.ranked_recall_routing_diff_delta_min_basis_points,
        400
    );
    assert_eq!(
        packet.ranked_recall_min_positive_routing_diff_delta_basis_points,
        640
    );
    assert_eq!(packet.ranked_recall_routing_diff_latency_delta_max_ms, 20);
    assert_eq!(
        packet.ranked_recall_max_positive_routing_diff_latency_delta_ms,
        10
    );
    assert_eq!(
        packet.ranked_recall_routing_diff_token_tradeoff_min_basis_points,
        1_000
    );
    assert_eq!(
        packet.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points,
        3_000
    );
    assert_eq!(packet.ranked_recall_real_workload_trace_fixture_count, 4);
    assert_eq!(
        packet.ranked_recall_real_workload_trace_shadow_only_count,
        4
    );
    assert_eq!(packet.ranked_recall_real_workload_trace_slo_pass_count, 3);
    assert_eq!(packet.ranked_recall_real_workload_trace_win_count, 3);
    assert_eq!(packet.ranked_recall_real_workload_trace_loss_count, 1);
    assert_eq!(
        packet.ranked_recall_real_workload_trace_operator_review_required_count,
        4
    );
    assert_eq!(packet.ranked_recall_real_workload_trace_total_leak_count, 0);
    assert_eq!(
        packet.ranked_recall_real_workload_trace_max_leak_rate_basis_points,
        0
    );
    assert_eq!(
        packet.ranked_recall_min_positive_real_workload_trace_coverage_basis_points,
        8_000
    );
    assert_eq!(
        packet.ranked_recall_min_positive_real_workload_trace_precision_basis_points,
        8_000
    );
    assert_eq!(
        packet.ranked_recall_total_positive_real_workload_trace_token_saved,
        2_140
    );
    assert_eq!(
        packet.ranked_recall_max_positive_real_workload_trace_latency_ms,
        55
    );
    assert_eq!(
        packet.ranked_recall_real_workload_trace_regression_loss_count,
        1
    );
    assert_eq!(packet.ranked_recall_canary_precondition_fixture_count, 4);
    assert_eq!(
        packet.ranked_recall_canary_precondition_shadow_only_count,
        4
    );
    assert_eq!(packet.ranked_recall_canary_precondition_pass_count, 4);
    assert_eq!(packet.ranked_recall_canary_feature_flag_registered_count, 4);
    assert_eq!(packet.ranked_recall_canary_feature_flag_disabled_count, 4);
    assert_eq!(packet.ranked_recall_canary_kill_switch_enabled_count, 4);
    assert_eq!(
        packet.ranked_recall_canary_precondition_route_opened_count,
        0
    );
    assert_eq!(
        packet.ranked_recall_canary_precondition_rollback_write_count,
        0
    );
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
    assert!(json.contains("memory_temporal_graph_shadow_traversal_diff_llm_rerank_count"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_slo_pass_count"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_coverage_basis_points"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_projected_latency_ms"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_token_saved_estimate"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_llm_rerank_count"));
    assert!(json.contains("memory_ranked_recall_shadow_eval_shadow_only"));
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
    assert!(json.contains("ranked_recall_canary_precondition_pass_count"));
    assert!(json.contains("ranked_recall_canary_feature_flag_disabled_count"));
    assert!(json.contains("ranked_recall_canary_kill_switch_enabled_count"));
    assert!(json.contains("ranked_recall_canary_precondition_route_opened_count"));
    assert!(json.contains("memory_provider_boundary_shadow_only"));
    assert!(json.contains("memory_provider_v2_boundary_shadow_only"));
    assert!(json.contains("memory_provider_v2_lifecycle_pass_count"));
    assert!(json.contains("memory_provider_v2_propose_write_check_pass"));
    assert!(json.contains("memory_provider_v2_close_check_pass"));
    assert!(json.contains("memory_namespace_policy_shadow_only"));
    assert!(json.contains("memory_write_chain_readiness_shadow_only"));
    assert!(json.contains("memory_write_chain_receipt_freshness_shadow_only"));
    assert!(json.contains("memory_namespace_policy_namespace_count"));
    assert!(json.contains("memory_namespace_policy_shadow_wal_required_count"));
    assert!(json.contains("memory_namespace_policy_operator_approval_required_count"));
    assert!(json.contains("memory_write_chain_stage_pass_count"));
    assert!(json.contains("memory_write_chain_readback_ready_count"));
    assert!(json.contains("memory_write_chain_canary_ready_count"));
    assert!(json.contains("memory_write_chain_receipt_projected_count"));
    assert!(json.contains("memory_write_chain_receipt_digest_count"));
    assert!(json.contains("memory_write_chain_receipt_freshness_pass_count"));
    assert!(json.contains("memory_write_chain_receipt_stale_replay_rejected_count"));
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
fn context_plane_operator_approval_packet_rejects_ranked_recall_hybrid_false_green() {
    let status = super::context_plane_activation::context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);
    assert!(packet.has_packet_integrity());

    let mut partial_signal = packet.clone();
    partial_signal.ranked_recall_feedback_check_pass = false;
    assert!(!partial_signal.has_packet_integrity());

    let mut inflated_pass_count = packet.clone();
    inflated_pass_count.ranked_recall_hybrid_signal_pass_count = 6;
    assert!(!inflated_pass_count.has_packet_integrity());

    let mut regression_false_green = packet.clone();
    regression_false_green.ranked_recall_hybrid_regression_blocked_count = 0;
    assert!(!regression_false_green.has_packet_integrity());

    let mut low_score_false_green = packet.clone();
    low_score_false_green.ranked_recall_min_positive_hybrid_score_basis_points = 5999;
    assert!(!low_score_false_green.has_packet_integrity());

    let mut routing_diff_false_green = packet.clone();
    routing_diff_false_green.ranked_recall_routing_diff_shadow_only_count = 3;
    assert!(!routing_diff_false_green.has_packet_integrity());

    let mut slo_false_green = packet.clone();
    slo_false_green.ranked_recall_real_workload_trace_total_leak_count = 1;
    assert!(!slo_false_green.has_packet_integrity());

    let mut canary_precondition_false_green = packet.clone();
    canary_precondition_false_green.ranked_recall_canary_feature_flag_disabled_count = 3;
    assert!(!canary_precondition_false_green.has_packet_integrity());

    let mut canary_route_false_green = packet.clone();
    canary_route_false_green.ranked_recall_canary_precondition_route_opened_count = 1;
    assert!(!canary_route_false_green.has_packet_integrity());
}

#[test]
fn context_plane_operator_approval_packet_rejects_canary_promotion_checklist_false_green() {
    let status = super::context_plane_activation::context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);
    assert!(packet.has_packet_integrity());

    let mut partial_checklist = packet.clone();
    partial_checklist.canary_promotion_audit_freshness_check_pass = false;
    partial_checklist.canary_promotion_checklist_pass_count = 3;
    assert!(!partial_checklist.has_packet_integrity());

    let mut partial_rehearsal = packet.clone();
    partial_rehearsal.canary_promotion_rollback_rehearsal_pass_count = 2;
    assert!(!partial_rehearsal.has_packet_integrity());

    let mut blocker_false_green = packet.clone();
    blocker_false_green.canary_promotion_blocker_count = 1;
    assert!(!blocker_false_green.has_packet_integrity());
}

#[test]
fn context_plane_operator_approval_packet_rejects_memory_provider_v2_lifecycle_false_green() {
    let status = super::context_plane_activation::context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);
    assert!(packet.has_packet_integrity());

    let mut partial_lifecycle = packet.clone();
    partial_lifecycle.memory_provider_v2_close_check_pass = false;
    assert!(!partial_lifecycle.has_packet_integrity());

    let mut inflated_pass_count = packet.clone();
    inflated_pass_count.memory_provider_v2_lifecycle_pass_count = 7;
    assert!(!inflated_pass_count.has_packet_integrity());
}

#[test]
fn context_plane_operator_approval_packet_rejects_namespace_policy_false_green() {
    let status = super::context_plane_activation::context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);
    assert!(packet.has_packet_integrity());

    let mut partial_policy = packet.clone();
    partial_policy.memory_namespace_policy_shadow_wal_required_count = 5;
    assert!(!partial_policy.has_packet_integrity());

    let mut write_false_green = packet.clone();
    write_false_green.memory_namespace_policy_production_write_count = 1;
    assert!(!write_false_green.has_packet_integrity());
}

#[test]
fn context_plane_operator_approval_packet_rejects_memory_write_chain_false_green() {
    let status = super::context_plane_activation::context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);
    assert!(packet.has_packet_integrity());

    let mut partial_readback = packet.clone();
    partial_readback.memory_write_chain_readback_ready_count = 5;
    assert!(!partial_readback.has_packet_integrity());

    let mut partial_canary = packet.clone();
    partial_canary.memory_write_chain_canary_ready_count = 5;
    assert!(!partial_canary.has_packet_integrity());

    let mut write_false_green = packet.clone();
    write_false_green.memory_write_chain_production_write_count = 1;
    assert!(!write_false_green.has_packet_integrity());

    let mut stale_receipt = packet.clone();
    stale_receipt.memory_write_chain_receipt_freshness_pass_count = 5;
    assert!(!stale_receipt.has_packet_integrity());

    let mut receipt_digest_drift = packet.clone();
    receipt_digest_drift.memory_write_chain_receipt_digest_count = 5;
    assert!(!receipt_digest_drift.has_packet_integrity());

    let mut receipt_persistence_drift = packet.clone();
    receipt_persistence_drift.memory_write_chain_receipt_persisted_count = 1;
    assert!(!receipt_persistence_drift.has_packet_integrity());
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
    assert_eq!(packet.matrix_row_count, 25);
    assert_eq!(packet.threshold_satisfied_count, 8);
    assert_eq!(packet.blocker_count, 17);
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
    assert_eq!(packet.canary_promotion_blocker_count, 0);
    assert_eq!(packet.canary_promotion_checklist_required_count, 4);
    assert_eq!(packet.canary_promotion_checklist_pass_count, 4);
    assert_eq!(packet.canary_promotion_rollback_rehearsal_pass_count, 3);
    assert_eq!(packet.canary_promotion_kill_switch_rehearsal_pass_count, 3);
    assert_eq!(packet.canary_promotion_soak_readback_pass_count, 3);
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
        matrix_row_count: 25,
        threshold_satisfied_count: 9,
        blocker_count: 16,
        threshold_snapshot: ContextPlaneOperatorApprovalThresholdSnapshot {
            total_row_count: 25,
            threshold_satisfied_count: 9,
            blocker_count: 16,
            required_ready_count: 24,
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
                reason: ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::TemporalGraphShadowStoreShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::TemporalGraphShadowReplayShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason:
                    ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalDiffShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason:
                    ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalQualityShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason:
                    ContextPlaneActivationBlockerReason::MemoryWriteChainReceiptFreshnessShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason:
                    ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly,
                count: 1,
            },
            ContextPlaneOperatorApprovalBlockerReasonCount {
                reason: ContextPlaneActivationBlockerReason::OperatorApprovalMissing,
                count: 1,
            },
        ],
        canary_promotion_required_stable_window_count: 1,
        canary_promotion_observed_stable_window_count: 1,
        canary_promotion_required_pass_streak: 3,
        canary_promotion_observed_pass_streak: 3,
        canary_promotion_blocker_count: 0,
        canary_promotion_checklist_required_count: 4,
        canary_promotion_checklist_pass_count: 4,
        canary_promotion_readiness_check_pass: true,
        canary_promotion_negative_rehearsal_check_pass: true,
        canary_promotion_audit_digest_check_pass: true,
        canary_promotion_audit_freshness_check_pass: true,
        canary_promotion_rollback_rehearsal_count: 3,
        canary_promotion_rollback_rehearsal_pass_count: 3,
        canary_promotion_kill_switch_rehearsal_count: 3,
        canary_promotion_kill_switch_rehearsal_pass_count: 3,
        canary_promotion_soak_readback_window_count: 3,
        canary_promotion_soak_readback_pass_count: 3,
        memory_provider_v2_lifecycle_required_count: 6,
        memory_provider_v2_lifecycle_pass_count: 6,
        memory_provider_v2_query_check_pass: true,
        memory_provider_v2_update_context_check_pass: true,
        memory_provider_v2_propose_write_check_pass: true,
        memory_provider_v2_add_check_pass: true,
        memory_provider_v2_clear_check_pass: true,
        memory_provider_v2_close_check_pass: true,
        memory_provider_v2_candidate_count: 1,
        memory_provider_v2_operator_review_required_count: 1,
        memory_namespace_policy_namespace_count: 6,
        memory_namespace_policy_operator_approval_required_count: 6,
        memory_namespace_policy_shadow_wal_required_count: 6,
        memory_namespace_policy_readback_required_count: 6,
        memory_namespace_policy_canary_required_count: 6,
        memory_namespace_policy_rollback_supported_count: 6,
        memory_namespace_policy_production_write_count: 0,
        memory_namespace_policy_graph_write_count: 0,
        memory_write_chain_namespace_count: 6,
        memory_write_chain_stage_required_count: 6,
        memory_write_chain_stage_pass_count: 6,
        memory_write_chain_propose_write_ready_count: 6,
        memory_write_chain_policy_approval_ready_count: 6,
        memory_write_chain_operator_approval_ready_count: 6,
        memory_write_chain_shadow_wal_ready_count: 6,
        memory_write_chain_readback_ready_count: 6,
        memory_write_chain_canary_ready_count: 6,
        memory_write_chain_rollback_ready_count: 6,
        memory_write_chain_production_write_count: 0,
        memory_write_chain_graph_write_count: 0,
        memory_write_chain_receipt_namespace_count: 6,
        memory_write_chain_receipt_required_count: 18,
        memory_write_chain_receipt_projected_count: 18,
        memory_write_chain_receipt_digest_count: 6,
        memory_write_chain_receipt_freshness_pass_count: 6,
        memory_write_chain_receipt_replay_guard_pass_count: 6,
        memory_write_chain_receipt_stale_replay_rejected_count: 6,
        memory_write_chain_receipt_recorded_count: 0,
        memory_write_chain_receipt_persisted_count: 0,
        memory_write_chain_receipt_production_write_count: 0,
        memory_write_chain_receipt_graph_write_count: 0,
        memory_temporal_graph_shadow_store_node_count: 1,
        memory_temporal_graph_shadow_store_edge_count: 2,
        memory_temporal_graph_shadow_store_provenance_edge_count: 1,
        memory_temporal_graph_shadow_store_validity_window_edge_count: 1,
        memory_temporal_graph_shadow_store_stage_required_count: 6,
        memory_temporal_graph_shadow_store_stage_projected_count: 6,
        memory_temporal_graph_shadow_store_digest_count: 1,
        memory_temporal_graph_shadow_store_freshness_pass_count: 1,
        memory_temporal_graph_shadow_store_replay_guard_pass_count: 1,
        memory_temporal_graph_shadow_store_stale_replay_rejected_count: 1,
        memory_temporal_graph_shadow_store_operator_approval_required_count: 1,
        memory_temporal_graph_shadow_store_operator_approval_recorded_count: 0,
        memory_temporal_graph_shadow_store_recorded_receipt_count: 0,
        memory_temporal_graph_shadow_store_persisted_receipt_count: 0,
        memory_temporal_graph_shadow_store_production_write_count: 0,
        memory_temporal_graph_shadow_store_graph_write_count: 0,
        memory_temporal_graph_shadow_replay_node_count: 1,
        memory_temporal_graph_shadow_replay_edge_count: 2,
        memory_temporal_graph_shadow_replay_provenance_count: 1,
        memory_temporal_graph_shadow_replay_bitemporal_validity_count: 1,
        memory_temporal_graph_shadow_replay_fact_invalidation_count: 0,
        memory_temporal_graph_shadow_replay_supersede_tombstone_count: 0,
        memory_temporal_graph_shadow_replay_stage_required_count: 6,
        memory_temporal_graph_shadow_replay_stage_projected_count: 6,
        memory_temporal_graph_shadow_replay_digest_count: 6,
        memory_temporal_graph_shadow_replay_freshness_pass_count: 6,
        memory_temporal_graph_shadow_replay_guard_pass_count: 6,
        memory_temporal_graph_shadow_replay_stale_replay_rejected_count: 6,
        memory_temporal_graph_shadow_replay_operator_approval_required_count: 1,
        memory_temporal_graph_shadow_replay_operator_approval_recorded_count: 0,
        memory_temporal_graph_shadow_replay_recorded_receipt_count: 0,
        memory_temporal_graph_shadow_replay_persisted_receipt_count: 0,
        memory_temporal_graph_shadow_replay_production_write_count: 0,
        memory_temporal_graph_shadow_replay_graph_write_count: 0,
        memory_temporal_graph_shadow_traversal_diff_production_selection_count: 1,
        memory_temporal_graph_shadow_traversal_diff_lexical_bm25_candidate_count: 1,
        memory_temporal_graph_shadow_traversal_diff_semantic_candidate_count: 1,
        memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count: 2,
        memory_temporal_graph_shadow_traversal_diff_hybrid_candidate_count: 2,
        memory_temporal_graph_shadow_traversal_diff_overlap_candidate_count: 1,
        memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count: 1,
        memory_temporal_graph_shadow_traversal_diff_win_count: 1,
        memory_temporal_graph_shadow_traversal_diff_loss_count: 0,
        memory_temporal_graph_shadow_traversal_diff_cost_count: 1,
        memory_temporal_graph_shadow_traversal_diff_stage_required_count: 5,
        memory_temporal_graph_shadow_traversal_diff_stage_projected_count: 5,
        memory_temporal_graph_shadow_traversal_diff_digest_count: 5,
        memory_temporal_graph_shadow_traversal_diff_freshness_pass_count: 5,
        memory_temporal_graph_shadow_traversal_diff_replay_guard_pass_count: 5,
        memory_temporal_graph_shadow_traversal_diff_stale_replay_rejected_count: 5,
        memory_temporal_graph_shadow_traversal_diff_llm_rerank_count: 0,
        memory_temporal_graph_shadow_traversal_diff_graph_persistence_count: 0,
        memory_temporal_graph_shadow_traversal_diff_production_route_count: 0,
        memory_temporal_graph_shadow_traversal_diff_production_write_count: 0,
        memory_temporal_graph_shadow_traversal_diff_graph_write_count: 0,
        memory_temporal_graph_shadow_traversal_quality_fixture_count: 5,
        memory_temporal_graph_shadow_traversal_quality_slo_required_count: 5,
        memory_temporal_graph_shadow_traversal_quality_slo_pass_count: 5,
        memory_temporal_graph_shadow_traversal_quality_coverage_basis_points: 10_000,
        memory_temporal_graph_shadow_traversal_quality_precision_basis_points: 10_000,
        memory_temporal_graph_shadow_traversal_quality_leak_rate_basis_points: 0,
        memory_temporal_graph_shadow_traversal_quality_latency_budget_ms: 20,
        memory_temporal_graph_shadow_traversal_quality_projected_latency_ms: 1,
        memory_temporal_graph_shadow_traversal_quality_token_saved_estimate: 256,
        memory_temporal_graph_shadow_traversal_quality_operator_review_required_count: 5,
        memory_temporal_graph_shadow_traversal_quality_win_count: 1,
        memory_temporal_graph_shadow_traversal_quality_loss_count: 0,
        memory_temporal_graph_shadow_traversal_quality_cost_count: 1,
        memory_temporal_graph_shadow_traversal_quality_stage_required_count: 5,
        memory_temporal_graph_shadow_traversal_quality_stage_projected_count: 5,
        memory_temporal_graph_shadow_traversal_quality_digest_count: 5,
        memory_temporal_graph_shadow_traversal_quality_freshness_pass_count: 5,
        memory_temporal_graph_shadow_traversal_quality_replay_guard_pass_count: 5,
        memory_temporal_graph_shadow_traversal_quality_stale_replay_rejected_count: 5,
        memory_temporal_graph_shadow_traversal_quality_llm_rerank_count: 0,
        memory_temporal_graph_shadow_traversal_quality_graph_persistence_count: 0,
        memory_temporal_graph_shadow_traversal_quality_production_route_count: 0,
        memory_temporal_graph_shadow_traversal_quality_production_write_count: 0,
        memory_temporal_graph_shadow_traversal_quality_graph_write_count: 0,
        ranked_recall_hybrid_signal_required_count: 5,
        ranked_recall_hybrid_signal_pass_count: 5,
        ranked_recall_lexical_bm25_check_pass: true,
        ranked_recall_recency_check_pass: true,
        ranked_recall_source_authority_check_pass: true,
        ranked_recall_temporal_validity_check_pass: true,
        ranked_recall_feedback_check_pass: true,
        ranked_recall_positive_hybrid_signal_required_count: 15,
        ranked_recall_positive_hybrid_signal_pass_count: 15,
        ranked_recall_hybrid_regression_blocked_count: 1,
        ranked_recall_hybrid_signal_min_basis_points: 6000,
        ranked_recall_min_positive_hybrid_score_basis_points: 7800,
        ranked_recall_routing_diff_fixture_count: 4,
        ranked_recall_routing_diff_shadow_only_count: 4,
        ranked_recall_routing_diff_win_count: 3,
        ranked_recall_routing_diff_loss_count: 1,
        ranked_recall_routing_diff_regression_blocked_count: 1,
        ranked_recall_routing_diff_delta_min_basis_points: 400,
        ranked_recall_min_positive_routing_diff_delta_basis_points: 640,
        ranked_recall_routing_diff_latency_delta_max_ms: 20,
        ranked_recall_max_positive_routing_diff_latency_delta_ms: 10,
        ranked_recall_routing_diff_token_tradeoff_min_basis_points: 1_000,
        ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: 3_000,
        ranked_recall_real_workload_trace_fixture_count: 4,
        ranked_recall_real_workload_trace_shadow_only_count: 4,
        ranked_recall_real_workload_trace_slo_pass_count: 3,
        ranked_recall_real_workload_trace_win_count: 3,
        ranked_recall_real_workload_trace_loss_count: 1,
        ranked_recall_real_workload_trace_operator_review_required_count: 4,
        ranked_recall_real_workload_trace_total_leak_count: 0,
        ranked_recall_real_workload_trace_max_leak_rate_basis_points: 0,
        ranked_recall_min_positive_real_workload_trace_coverage_basis_points: 8_000,
        ranked_recall_min_positive_real_workload_trace_precision_basis_points: 8_000,
        ranked_recall_total_positive_real_workload_trace_token_saved: 2_140,
        ranked_recall_max_positive_real_workload_trace_latency_ms: 55,
        ranked_recall_real_workload_trace_regression_loss_count: 1,
        ranked_recall_canary_precondition_fixture_count: 4,
        ranked_recall_canary_precondition_shadow_only_count: 4,
        ranked_recall_canary_precondition_pass_count: 4,
        ranked_recall_canary_feature_flag_registered_count: 4,
        ranked_recall_canary_feature_flag_disabled_count: 4,
        ranked_recall_canary_kill_switch_registered_count: 4,
        ranked_recall_canary_kill_switch_enabled_count: 4,
        ranked_recall_canary_rollback_rehearsal_covered_count: 4,
        ranked_recall_canary_activation_denial_covered_count: 4,
        ranked_recall_canary_precondition_operator_review_required_count: 4,
        ranked_recall_canary_precondition_route_opened_count: 0,
        ranked_recall_canary_precondition_rollback_write_count: 0,
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
