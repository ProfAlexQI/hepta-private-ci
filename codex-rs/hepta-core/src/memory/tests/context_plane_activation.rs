use super::*;

pub(super) fn context_plane_activation_status_fixture() -> ContextPlaneStatusReport {
    ContextPlaneStatusReport {
        sections: vec![
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::SourceRegistry, 1),
            ContextPlaneStatusEntry::shadow(ContextPlaneStatusSection::AdaptiveBudgetAllocation, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryTaxonomy, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryFormationReceipts, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryFormationQueue, 1),
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryNamespacePolicy,
                    6,
                );
                entry.memory_namespace_policy_namespace_count = 6;
                entry.memory_namespace_policy_operator_approval_required_count = 6;
                entry.memory_namespace_policy_shadow_wal_required_count = 6;
                entry.memory_namespace_policy_readback_required_count = 6;
                entry.memory_namespace_policy_canary_required_count = 6;
                entry.memory_namespace_policy_rollback_supported_count = 6;
                entry
            },
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryWriteChainReadiness,
                    6,
                );
                entry.memory_write_chain_namespace_count = 6;
                entry.memory_write_chain_stage_required_count = 6;
                entry.memory_write_chain_stage_pass_count = 6;
                entry.memory_write_chain_propose_write_ready_count = 6;
                entry.memory_write_chain_policy_approval_ready_count = 6;
                entry.memory_write_chain_operator_approval_ready_count = 6;
                entry.memory_write_chain_shadow_wal_ready_count = 6;
                entry.memory_write_chain_readback_ready_count = 6;
                entry.memory_write_chain_canary_ready_count = 6;
                entry.memory_write_chain_rollback_ready_count = 6;
                entry
            },
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness,
                    6,
                );
                entry.memory_write_chain_receipt_namespace_count = 6;
                entry.memory_write_chain_receipt_required_count = 18;
                entry.memory_write_chain_receipt_projected_count = 18;
                entry.memory_write_chain_receipt_digest_count = 6;
                entry.memory_write_chain_receipt_freshness_pass_count = 6;
                entry.memory_write_chain_receipt_replay_guard_pass_count = 6;
                entry.memory_write_chain_receipt_stale_replay_rejected_count = 6;
                entry
            },
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryTemporalFacts, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryTemporalFactGraph, 1),
            ContextPlaneStatusEntry::shadow(
                ContextPlaneStatusSection::MemoryTemporalGraphShadowEval,
                4,
            ),
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryTemporalGraphShadowStore,
                    5,
                );
                entry.memory_temporal_graph_shadow_store_node_count = 5;
                entry.memory_temporal_graph_shadow_store_edge_count = 10;
                entry.memory_temporal_graph_shadow_store_provenance_edge_count = 5;
                entry.memory_temporal_graph_shadow_store_validity_window_edge_count = 5;
                entry.memory_temporal_graph_shadow_store_stage_required_count = 6;
                entry.memory_temporal_graph_shadow_store_stage_projected_count = 6;
                entry.memory_temporal_graph_shadow_store_digest_count = 1;
                entry.memory_temporal_graph_shadow_store_freshness_pass_count = 1;
                entry.memory_temporal_graph_shadow_store_replay_guard_pass_count = 1;
                entry.memory_temporal_graph_shadow_store_stale_replay_rejected_count = 1;
                entry.memory_temporal_graph_shadow_store_operator_approval_required_count = 1;
                entry
            },
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay,
                    5,
                );
                entry.memory_temporal_graph_shadow_replay_node_count = 5;
                entry.memory_temporal_graph_shadow_replay_edge_count = 10;
                entry.memory_temporal_graph_shadow_replay_provenance_count = 5;
                entry.memory_temporal_graph_shadow_replay_bitemporal_validity_count = 5;
                entry.memory_temporal_graph_shadow_replay_stage_required_count = 6;
                entry.memory_temporal_graph_shadow_replay_stage_projected_count = 6;
                entry.memory_temporal_graph_shadow_replay_digest_count = 6;
                entry.memory_temporal_graph_shadow_replay_freshness_pass_count = 6;
                entry.memory_temporal_graph_shadow_replay_guard_pass_count = 6;
                entry.memory_temporal_graph_shadow_replay_stale_replay_rejected_count = 6;
                entry.memory_temporal_graph_shadow_replay_operator_approval_required_count = 1;
                entry
            },
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalDiff,
                    5,
                );
                entry.memory_temporal_graph_shadow_traversal_diff_production_selection_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_lexical_bm25_candidate_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_semantic_candidate_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count =
                    10;
                entry.memory_temporal_graph_shadow_traversal_diff_hybrid_candidate_count = 10;
                entry.memory_temporal_graph_shadow_traversal_diff_overlap_candidate_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count =
                    5;
                entry.memory_temporal_graph_shadow_traversal_diff_win_count = 1;
                entry.memory_temporal_graph_shadow_traversal_diff_loss_count = 0;
                entry.memory_temporal_graph_shadow_traversal_diff_cost_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_stage_required_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_stage_projected_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_digest_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_freshness_pass_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_replay_guard_pass_count = 5;
                entry.memory_temporal_graph_shadow_traversal_diff_stale_replay_rejected_count = 5;
                entry
            },
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality,
                    5,
                );
                entry.memory_temporal_graph_shadow_traversal_quality_fixture_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_slo_required_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_slo_pass_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_coverage_basis_points = 10_000;
                entry.memory_temporal_graph_shadow_traversal_quality_precision_basis_points =
                    10_000;
                entry.memory_temporal_graph_shadow_traversal_quality_leak_rate_basis_points = 0;
                entry.memory_temporal_graph_shadow_traversal_quality_latency_budget_ms = 20;
                entry.memory_temporal_graph_shadow_traversal_quality_projected_latency_ms = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_token_saved_estimate = 768;
                entry
                    .memory_temporal_graph_shadow_traversal_quality_operator_review_required_count =
                    5;
                entry.memory_temporal_graph_shadow_traversal_quality_win_count = 1;
                entry.memory_temporal_graph_shadow_traversal_quality_loss_count = 0;
                entry.memory_temporal_graph_shadow_traversal_quality_cost_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_stage_required_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_stage_projected_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_digest_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_freshness_pass_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_replay_guard_pass_count = 5;
                entry.memory_temporal_graph_shadow_traversal_quality_stale_replay_rejected_count =
                    5;
                entry
            },
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard,
                    5,
                );
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_stage_projected_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_quality_slo_pass_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_required_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_recorded_count = 0;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_enabled_count = 0;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_registered_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_ready_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_required_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_pass_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_activation_denial_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count = 0;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_digest_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_freshness_pass_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_replay_guard_pass_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_canary_guard_stale_replay_rejected_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_llm_rerank_count = 0;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_graph_persistence_count =
                    0;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_production_route_count =
                    0;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_production_write_count =
                    0;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_graph_write_count = 0;
                entry.memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count = 0;
                entry
            },
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch,
                    5,
                );
                entry.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count =
                    6;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_projected_count =
                    6;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_guard_pass_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_required_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_recorded_count =
                    0;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_registered_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_enabled_count =
                    0;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_registered_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_readback_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_pass_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_required_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_readback_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_pass_count =
                    5;
                entry.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_denial_count =
                    5;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_route_opened_count =
                    0;
                entry.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_digest_count = 6;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_freshness_pass_count =
                    6;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_replay_guard_pass_count =
                    6;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stale_replay_rejected_count =
                    6;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_llm_rerank_count =
                    0;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_persistence_count =
                    0;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_route_count =
                    0;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_write_count =
                    0;
                entry.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_write_count =
                    0;
                entry
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_count =
                    0;
                entry
            },
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalPromotionReadiness,
                    5,
                );
                entry.memory_temporal_graph_shadow_retrieval_promotion_readiness_fixture_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_stage_required_count = 7;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_stage_projected_count = 7;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_rollback_kill_switch_pass_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_required_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_shadow_only_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_slo_pass_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_win_count = 1;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_operator_review_required_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_coverage_basis_points = 10_000;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_precision_basis_points = 10_000;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_token_saved_estimate = 768;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_real_workload_trace_latency_ms = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_operator_approval_required_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_feature_flag_registered_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_kill_switch_pass_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_rollback_rehearsal_pass_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_route_denial_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_rollback_write_denial_count = 5;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_ready_shadow_only_count = 5;
                entry.memory_temporal_graph_shadow_retrieval_promotion_readiness_digest_count = 7;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_freshness_pass_count = 7;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_replay_guard_pass_count = 7;
                entry
                    .memory_temporal_graph_shadow_retrieval_promotion_readiness_stale_replay_rejected_count = 7;
                entry
            },
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::EvalHarnessSeed, 2),
            ContextPlaneStatusEntry::shadow(
                ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow,
                4,
            ),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::RecallQualityGate, 2),
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryRankedRecallShadowEval,
                    4,
                );
                entry.ranked_recall_hybrid_signal_required_count = 5;
                entry.ranked_recall_hybrid_signal_pass_count = 5;
                entry.ranked_recall_lexical_bm25_check_pass = true;
                entry.ranked_recall_recency_check_pass = true;
                entry.ranked_recall_source_authority_check_pass = true;
                entry.ranked_recall_temporal_validity_check_pass = true;
                entry.ranked_recall_feedback_check_pass = true;
                entry.ranked_recall_positive_hybrid_signal_required_count = 15;
                entry.ranked_recall_positive_hybrid_signal_pass_count = 15;
                entry.ranked_recall_hybrid_regression_blocked_count = 1;
                entry.ranked_recall_hybrid_signal_min_basis_points = 6000;
                entry.ranked_recall_min_positive_hybrid_score_basis_points = 7800;
                entry.ranked_recall_routing_diff_fixture_count = 4;
                entry.ranked_recall_routing_diff_shadow_only_count = 4;
                entry.ranked_recall_routing_diff_win_count = 3;
                entry.ranked_recall_routing_diff_loss_count = 1;
                entry.ranked_recall_routing_diff_regression_blocked_count = 1;
                entry.ranked_recall_routing_diff_delta_min_basis_points = 400;
                entry.ranked_recall_min_positive_routing_diff_delta_basis_points = 640;
                entry.ranked_recall_routing_diff_latency_delta_max_ms = 20;
                entry.ranked_recall_max_positive_routing_diff_latency_delta_ms = 10;
                entry.ranked_recall_routing_diff_token_tradeoff_min_basis_points = 1_000;
                entry.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points = 3_000;
                entry.ranked_recall_real_workload_trace_fixture_count = 4;
                entry.ranked_recall_real_workload_trace_shadow_only_count = 4;
                entry.ranked_recall_real_workload_trace_slo_pass_count = 3;
                entry.ranked_recall_real_workload_trace_win_count = 3;
                entry.ranked_recall_real_workload_trace_loss_count = 1;
                entry.ranked_recall_real_workload_trace_operator_review_required_count = 4;
                entry.ranked_recall_real_workload_trace_total_leak_count = 0;
                entry.ranked_recall_real_workload_trace_max_leak_rate_basis_points = 0;
                entry.ranked_recall_min_positive_real_workload_trace_coverage_basis_points = 8_000;
                entry.ranked_recall_min_positive_real_workload_trace_precision_basis_points = 8_000;
                entry.ranked_recall_total_positive_real_workload_trace_token_saved = 2_140;
                entry.ranked_recall_max_positive_real_workload_trace_latency_ms = 55;
                entry.ranked_recall_real_workload_trace_regression_loss_count = 1;
                entry.ranked_recall_canary_precondition_fixture_count = 4;
                entry.ranked_recall_canary_precondition_shadow_only_count = 4;
                entry.ranked_recall_canary_precondition_pass_count = 4;
                entry.ranked_recall_canary_feature_flag_registered_count = 4;
                entry.ranked_recall_canary_feature_flag_disabled_count = 4;
                entry.ranked_recall_canary_kill_switch_registered_count = 4;
                entry.ranked_recall_canary_kill_switch_enabled_count = 4;
                entry.ranked_recall_canary_rollback_rehearsal_covered_count = 4;
                entry.ranked_recall_canary_activation_denial_covered_count = 4;
                entry.ranked_recall_canary_precondition_operator_review_required_count = 4;
                entry.ranked_recall_canary_precondition_route_opened_count = 0;
                entry.ranked_recall_canary_precondition_rollback_write_count = 0;
                entry
            },
            ContextPlaneStatusEntry::shadow(ContextPlaneStatusSection::MemoryProviderBoundary, 1),
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryProviderV2Boundary,
                    6,
                );
                entry.memory_provider_v2_lifecycle_required_count = 6;
                entry.memory_provider_v2_lifecycle_pass_count = 6;
                entry.memory_provider_v2_query_check_pass = true;
                entry.memory_provider_v2_update_context_check_pass = true;
                entry.memory_provider_v2_propose_write_check_pass = true;
                entry.memory_provider_v2_add_check_pass = true;
                entry.memory_provider_v2_clear_check_pass = true;
                entry.memory_provider_v2_close_check_pass = true;
                entry.memory_provider_v2_candidate_count = 1;
                entry.memory_provider_v2_operator_review_required_count = 1;
                entry
            },
            ContextPlaneStatusEntry::shadow(
                ContextPlaneStatusSection::MemoryShadowCanaryReadiness,
                3,
            ),
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness,
                    9,
                );
                entry.canary_promotion_required_stable_window_count = 1;
                entry.canary_promotion_observed_stable_window_count = 1;
                entry.canary_promotion_required_pass_streak = 3;
                entry.canary_promotion_observed_pass_streak = 3;
                entry.canary_promotion_checklist_required_count = 4;
                entry.canary_promotion_checklist_pass_count = 4;
                entry.canary_promotion_readiness_check_pass = true;
                entry.canary_promotion_negative_rehearsal_check_pass = true;
                entry.canary_promotion_audit_digest_check_pass = true;
                entry.canary_promotion_audit_freshness_check_pass = true;
                entry.canary_promotion_rollback_rehearsal_count = 3;
                entry.canary_promotion_rollback_rehearsal_pass_count = 3;
                entry.canary_promotion_kill_switch_rehearsal_count = 3;
                entry.canary_promotion_kill_switch_rehearsal_pass_count = 3;
                entry.canary_promotion_soak_readback_window_count = 3;
                entry.canary_promotion_soak_readback_pass_count = 3;
                entry
            },
            ContextPlaneStatusEntry::disabled(ContextPlaneStatusSection::SourceAwareFrontDoor),
        ],
        ..ContextPlaneStatusReport::default()
    }
}

#[test]
fn context_plane_activation_blocker_matrix_explains_disabled_runtime_activation() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);

    assert!(matrix.has_matrix_integrity());
    assert_eq!(matrix.rows.len(), 28);
    assert_eq!(matrix.satisfied_count(), 9);
    assert_eq!(matrix.blocker_count, 19);
    assert!(!matrix.activation_allowed);
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::SourceRegistry),
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
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::AdaptiveBudgetAllocation),
        Some(ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryProviderBoundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryRankedRecallShadowEval),
        Some(ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryNamespacePolicy),
        Some(ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryWriteChainReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness),
        Some(ContextPlaneActivationBlockerReason::MemoryWriteChainReceiptFreshnessShadowOnly)
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
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowStore),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowStoreShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowReplay),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowReplayShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalDiff),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalDiffShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalQuality
        ),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalQualityShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard
        ),
        Some(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowRetrievalCanaryGuardShadowOnly
        )
    );
    assert_eq!(
        matrix.blocker_reason(
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch
        ),
        Some(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowRetrievalRollbackKillSwitchShadowOnly
        )
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::SourceAwareFrontDoor),
        Some(ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::OperatorApproval),
        Some(ContextPlaneActivationBlockerReason::OperatorApprovalMissing)
    );
    let temporal_graph_traversal_diff_row = matrix
        .row_for_target(ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalDiff)
        .expect("memory temporal graph shadow traversal diff activation row should exist");
    assert_eq!(
        temporal_graph_traversal_diff_row
            .memory_temporal_graph_shadow_traversal_diff_production_selection_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_diff_row
            .memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count,
        10
    );
    assert_eq!(
        temporal_graph_traversal_diff_row
            .memory_temporal_graph_shadow_traversal_diff_stage_projected_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_diff_row.memory_temporal_graph_shadow_traversal_diff_digest_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_diff_row
            .memory_temporal_graph_shadow_traversal_diff_llm_rerank_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_diff_row
            .memory_temporal_graph_shadow_traversal_diff_graph_persistence_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_diff_row
            .memory_temporal_graph_shadow_traversal_diff_production_route_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_diff_row
            .memory_temporal_graph_shadow_traversal_diff_production_write_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_diff_row
            .memory_temporal_graph_shadow_traversal_diff_graph_write_count,
        0
    );
    let temporal_graph_traversal_quality_row = matrix
        .row_for_target(ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalQuality)
        .expect("memory temporal graph shadow traversal quality activation row should exist");
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_fixture_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_slo_pass_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_coverage_basis_points,
        10_000
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_projected_latency_ms,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_token_saved_estimate,
        768
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_stage_projected_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_digest_count,
        5
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_llm_rerank_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_graph_persistence_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_production_route_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_production_write_count,
        0
    );
    assert_eq!(
        temporal_graph_traversal_quality_row
            .memory_temporal_graph_shadow_traversal_quality_graph_write_count,
        0
    );
    let temporal_graph_retrieval_canary_guard_row = matrix
        .row_for_target(ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard)
        .expect("memory temporal graph shadow retrieval canary guard activation row should exist");
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_stage_projected_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_enabled_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_ready_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_pass_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_digest_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_production_route_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_canary_guard_row
            .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count,
        0
    );
    let temporal_graph_retrieval_rollback_kill_switch_row = matrix
        .row_for_target(
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch,
        )
        .expect(
            "memory temporal graph shadow retrieval rollback/kill-switch activation row should exist",
        );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_projected_count,
        6
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_readback_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_readback_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_denial_count,
        5
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_route_opened_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_digest_count,
        6
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_route_count,
        0
    );
    assert_eq!(
        temporal_graph_retrieval_rollback_kill_switch_row
            .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_count,
        0
    );
    assert!(!matrix.production_write);
    assert!(!matrix.graph_write);
    assert!(!matrix.runtime_activation);
    assert!(!matrix.adaptive_allocator_runtime_activation);
    assert!(!matrix.source_aware_runtime_activation);
    assert!(!matrix.prompt_assembly_change);
    assert!(!matrix.operator_activation_allowed);

    let json = serde_json::to_string(&matrix).expect("activation matrix should serialize");
    assert!(json.contains("source_registry"));
    assert!(json.contains("adaptive_budget_allocation"));
    assert!(json.contains("memory_formation_queue"));
    assert!(json.contains("memory_namespace_policy"));
    assert!(json.contains("memory_write_chain_readiness"));
    assert!(json.contains("memory_write_chain_readiness_shadow_only"));
    assert!(json.contains("memory_write_chain_stage_pass_count"));
    assert!(json.contains("memory_write_chain_readback_ready_count"));
    assert!(json.contains("memory_write_chain_canary_ready_count"));
    assert!(json.contains("memory_temporal_graph_shadow_eval"));
    assert!(json.contains("memory_temporal_graph_shadow_replay"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_diff"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality"));
    assert!(json.contains("memory_temporal_graph_shadow_retrieval_canary_guard"));
    assert!(json.contains("memory_temporal_graph_shadow_retrieval_rollback_kill_switch"));
    assert!(json.contains("temporal_graph_shadow_traversal_diff_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_traversal_quality_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_retrieval_canary_guard_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_retrieval_rollback_kill_switch_shadow_only"));
    assert!(
        json.contains(
            "memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count"
        )
    );
    assert!(json.contains("memory_temporal_graph_shadow_traversal_diff_stage_projected_count"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_slo_pass_count"));
    assert!(json.contains("memory_temporal_graph_shadow_traversal_quality_token_saved_estimate"));
    assert!(json.contains(
        "memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count"
    ));
    assert!(
        json.contains(
            "memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count"
        )
    );
    assert!(json.contains(
        "memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count"
    ));
    assert!(json.contains(
        "memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_denial_count"
    ));
    assert!(json.contains("recall_quality_gate"));
    assert!(json.contains("memory_ranked_recall_shadow_eval"));
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
    assert!(json.contains("memory_provider_boundary"));
    assert!(json.contains("memory_provider_v2_boundary"));
    assert!(json.contains("memory_provider_v2_lifecycle_pass_count"));
    assert!(json.contains("memory_provider_v2_propose_write_check_pass"));
    assert!(json.contains("memory_provider_v2_close_check_pass"));
    assert!(json.contains("memory_namespace_policy_shadow_only"));
    assert!(json.contains("memory_namespace_policy_namespace_count"));
    assert!(json.contains("memory_namespace_policy_shadow_wal_required_count"));
    assert!(json.contains("memory_namespace_policy_operator_approval_required_count"));
    assert!(json.contains("memory_shadow_canary_readiness"));
    assert!(json.contains("memory_shadow_canary_promotion_readiness"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("adaptive_budget_allocation_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_eval_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_replay_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_traversal_diff_shadow_only"));
    assert!(json.contains("memory_ranked_recall_shadow_eval_shadow_only"));
    assert!(json.contains("memory_provider_boundary_shadow_only"));
    assert!(json.contains("memory_provider_v2_boundary_shadow_only"));
    assert!(json.contains("memory_namespace_policy_shadow_only"));
    assert!(json.contains("memory_write_chain_readiness_shadow_only"));
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
    assert!(!json.contains("activation-test"));
    assert!(!json.contains("\"activation_allowed\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_ranked_recall_hybrid_false_green() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_signal = matrix.clone();
    partial_signal
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval activation row should exist")
        .ranked_recall_feedback_check_pass = false;
    assert!(!partial_signal.has_matrix_integrity());

    let mut inflated_pass_count = matrix.clone();
    inflated_pass_count
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval activation row should exist")
        .ranked_recall_hybrid_signal_pass_count = 6;
    assert!(!inflated_pass_count.has_matrix_integrity());

    let mut routing_diff_replay = matrix.clone();
    routing_diff_replay
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval activation row should exist")
        .ranked_recall_routing_diff_shadow_only_count = 3;
    assert!(!routing_diff_replay.has_matrix_integrity());

    let mut slo_false_green = matrix.clone();
    slo_false_green
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval activation row should exist")
        .ranked_recall_real_workload_trace_total_leak_count = 1;
    assert!(!slo_false_green.has_matrix_integrity());

    let mut canary_route_false_green = matrix.clone();
    canary_route_false_green
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryRankedRecallShadowEval)
        .expect("ranked recall shadow eval activation row should exist")
        .ranked_recall_canary_precondition_route_opened_count = 1;
    assert!(!canary_route_false_green.has_matrix_integrity());

    let mut non_ranked_leak = matrix.clone();
    non_ranked_leak
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryProviderBoundary)
        .expect("memory provider boundary activation row should exist")
        .ranked_recall_hybrid_signal_pass_count = 1;
    assert!(!non_ranked_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_temporal_graph_retrieval_canary_guard_false_green()
 {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_feature_flag = matrix.clone();
    partial_feature_flag
        .rows
        .iter_mut()
        .find(|row| {
            row.target
                == ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard
        })
        .expect("retrieval canary guard activation row should exist")
        .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count = 4;
    assert!(!partial_feature_flag.has_matrix_integrity());

    let mut canary_route_opened = matrix.clone();
    canary_route_opened
        .rows
        .iter_mut()
        .find(|row| {
            row.target
                == ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard
        })
        .expect("retrieval canary guard activation row should exist")
        .memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count = 1;
    assert!(!canary_route_opened.has_matrix_integrity());

    let mut rollback_write = matrix.clone();
    rollback_write
        .rows
        .iter_mut()
        .find(|row| {
            row.target
                == ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard
        })
        .expect("retrieval canary guard activation row should exist")
        .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count = 1;
    assert!(!rollback_write.has_matrix_integrity());

    let mut non_guard_leak = matrix.clone();
    non_guard_leak
        .rows
        .iter_mut()
        .find(|row| {
            row.target == ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalQuality
        })
        .expect("traversal quality activation row should exist")
        .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count = 5;
    assert!(!non_guard_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_temporal_graph_retrieval_rollback_kill_switch_false_green()
 {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_kill_switch_readback = matrix.clone();
    partial_kill_switch_readback
        .rows
        .iter_mut()
        .find(|row| {
            row.target
                == ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch
        })
        .expect("retrieval rollback/kill-switch activation row should exist")
        .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_readback_count = 4;
    assert!(!partial_kill_switch_readback.has_matrix_integrity());

    let mut route_opened = matrix.clone();
    route_opened
        .rows
        .iter_mut()
        .find(|row| {
            row.target
                == ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch
        })
        .expect("retrieval rollback/kill-switch activation row should exist")
        .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_route_opened_count = 1;
    assert!(!route_opened.has_matrix_integrity());

    let mut rollback_write = matrix.clone();
    rollback_write
        .rows
        .iter_mut()
        .find(|row| {
            row.target
                == ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch
        })
        .expect("retrieval rollback/kill-switch activation row should exist")
        .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_count = 1;
    assert!(!rollback_write.has_matrix_integrity());

    let mut non_rollback_kill_switch_leak = matrix.clone();
    non_rollback_kill_switch_leak
        .rows
        .iter_mut()
        .find(|row| {
            row.target
                == ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard
        })
        .expect("retrieval canary guard activation row should exist")
        .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count = 5;
    assert!(!non_rollback_kill_switch_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_canary_promotion_checklist_false_green() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_rehearsal = matrix.clone();
    partial_rehearsal
        .rows
        .iter_mut()
        .find(|row| {
            row.target == ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness
        })
        .expect("memory shadow canary promotion readiness activation row should exist")
        .canary_promotion_rollback_rehearsal_pass_count = 2;
    assert!(!partial_rehearsal.has_matrix_integrity());

    let mut blocker_false_green = matrix.clone();
    blocker_false_green
        .rows
        .iter_mut()
        .find(|row| {
            row.target == ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness
        })
        .expect("memory shadow canary promotion readiness activation row should exist")
        .canary_promotion_blocker_count = 1;
    assert!(!blocker_false_green.has_matrix_integrity());

    let mut non_promotion_leak = matrix.clone();
    non_promotion_leak
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::SourceRegistry)
        .expect("source registry activation row should exist")
        .canary_promotion_checklist_pass_count = 1;
    assert!(!non_promotion_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_memory_provider_v2_lifecycle_false_green() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_lifecycle = matrix.clone();
    partial_lifecycle
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryProviderV2Boundary)
        .expect("memory provider v2 activation row should exist")
        .memory_provider_v2_close_check_pass = false;
    assert!(!partial_lifecycle.has_matrix_integrity());

    let mut inflated_pass_count = matrix.clone();
    inflated_pass_count
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryProviderV2Boundary)
        .expect("memory provider v2 activation row should exist")
        .memory_provider_v2_lifecycle_pass_count = 7;
    assert!(!inflated_pass_count.has_matrix_integrity());

    let mut non_provider_v2_leak = matrix.clone();
    non_provider_v2_leak
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryProviderBoundary)
        .expect("memory provider boundary activation row should exist")
        .memory_provider_v2_lifecycle_pass_count = 1;
    assert!(!non_provider_v2_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_blocks_side_effect_flags_without_activation() {
    let mut status = context_plane_activation_status_fixture();
    let source_registry = status
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::SourceRegistry)
        .expect("source registry status row should exist");
    source_registry.production_write = true;

    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);

    assert!(matrix.has_matrix_integrity());
    assert_eq!(matrix.rows.len(), 28);
    assert_eq!(matrix.satisfied_count(), 8);
    assert_eq!(matrix.blocker_count, 20);
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::SourceRegistry),
        Some(false)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::SourceRegistry),
        Some(ContextPlaneActivationBlockerReason::SideEffectFlagEnabled)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::AdaptiveBudgetAllocation),
        Some(ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryProviderBoundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryRankedRecallShadowEval),
        Some(ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryNamespacePolicy),
        Some(ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly)
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
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowReplay),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowReplayShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalDiff),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalDiffShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalQuality
        ),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalQualityShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard
        ),
        Some(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowRetrievalCanaryGuardShadowOnly
        )
    );
    assert_eq!(
        matrix.blocker_reason(
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch
        ),
        Some(
            ContextPlaneActivationBlockerReason::TemporalGraphShadowRetrievalRollbackKillSwitchShadowOnly
        )
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::SourceAwareFrontDoor),
        Some(ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::OperatorApproval),
        Some(ContextPlaneActivationBlockerReason::OperatorApprovalMissing)
    );
    assert!(!matrix.activation_allowed);
    assert!(!matrix.production_write);
    assert!(!matrix.graph_write);
    assert!(!matrix.runtime_activation);
    assert!(!matrix.adaptive_allocator_runtime_activation);
    assert!(!matrix.source_aware_runtime_activation);
    assert!(!matrix.prompt_assembly_change);
    assert!(!matrix.operator_activation_allowed);

    let json = serde_json::to_string(&matrix).expect("side-effect blocker matrix should serialize");
    assert!(json.contains("side_effect_flag_enabled"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("\"activation_allowed\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));

    let mut report_side_effect_status = context_plane_activation_status_fixture();
    report_side_effect_status.runtime_activation = true;
    let report_side_effect_matrix =
        ContextPlaneActivationBlockerMatrix::from_status(&report_side_effect_status);

    assert!(report_side_effect_matrix.has_matrix_integrity());
    assert_eq!(report_side_effect_matrix.satisfied_count(), 0);
    assert_eq!(report_side_effect_matrix.blocker_count, 28);
    assert_eq!(
        report_side_effect_matrix.blocker_reason(ContextPlaneActivationTarget::RecallQualityGate),
        Some(ContextPlaneActivationBlockerReason::SideEffectFlagEnabled)
    );
    assert_eq!(
        report_side_effect_matrix.blocker_reason(ContextPlaneActivationTarget::OperatorApproval),
        Some(ContextPlaneActivationBlockerReason::OperatorApprovalMissing)
    );
    assert!(!report_side_effect_matrix.runtime_activation);
    assert!(!report_side_effect_matrix.activation_allowed);
}

#[test]
fn context_plane_activation_blocker_matrix_rolls_up_recall_quality_blockers_without_payloads() {
    let mut status = context_plane_activation_status_fixture();
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

    assert!(matrix.has_matrix_integrity());
    assert_eq!(matrix.rows.len(), 28);
    assert_eq!(matrix.satisfied_count(), 8);
    assert_eq!(matrix.blocker_count, 20);
    let recall_quality_row = matrix
        .row_for_target(ContextPlaneActivationTarget::RecallQualityGate)
        .expect("recall quality activation row should exist");
    assert_eq!(
        recall_quality_row.blocker_reason,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled
    );
    assert_eq!(recall_quality_row.recall_quality_blocking_reason_count, 2);
    assert_eq!(
        recall_quality_row.recall_quality_blocking_reasons,
        vec![
            ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression,
            ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled,
        ]
    );
    assert!(
        matrix
            .rows
            .iter()
            .filter(|row| row.target != ContextPlaneActivationTarget::RecallQualityGate)
            .all(|row| row.recall_quality_blocking_reasons.is_empty())
    );
    assert!(!matrix.activation_allowed);
    assert!(!matrix.production_write);
    assert!(!matrix.graph_write);
    assert!(!matrix.runtime_activation);
    assert!(!matrix.adaptive_allocator_runtime_activation);
    assert!(!matrix.source_aware_runtime_activation);
    assert!(!matrix.prompt_assembly_change);
    assert!(!matrix.operator_activation_allowed);

    let json =
        serde_json::to_string(&matrix).expect("recall-quality blocker matrix should serialize");
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
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
    assert!(!json.contains("activation-test"));
    assert!(!json.contains("\"activation_allowed\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_namespace_policy_false_green() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_policy = matrix.clone();
    partial_policy
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryNamespacePolicy)
        .expect("memory namespace policy activation row should exist")
        .memory_namespace_policy_shadow_wal_required_count = 5;
    assert!(!partial_policy.has_matrix_integrity());

    let mut write_false_green = matrix.clone();
    write_false_green
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryNamespacePolicy)
        .expect("memory namespace policy activation row should exist")
        .memory_namespace_policy_production_write_count = 1;
    assert!(!write_false_green.has_matrix_integrity());

    let mut non_policy_leak = matrix.clone();
    non_policy_leak
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryProviderBoundary)
        .expect("memory provider boundary activation row should exist")
        .memory_namespace_policy_namespace_count = 6;
    assert!(!non_policy_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_memory_write_chain_false_green() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_readback = matrix.clone();
    partial_readback
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness activation row should exist")
        .memory_write_chain_readback_ready_count = 5;
    assert!(!partial_readback.has_matrix_integrity());

    let mut partial_canary = matrix.clone();
    partial_canary
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness activation row should exist")
        .memory_write_chain_canary_ready_count = 5;
    assert!(!partial_canary.has_matrix_integrity());

    let mut write_false_green = matrix.clone();
    write_false_green
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness activation row should exist")
        .memory_write_chain_production_write_count = 1;
    assert!(!write_false_green.has_matrix_integrity());

    let mut non_write_chain_leak = matrix.clone();
    non_write_chain_leak
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryNamespacePolicy)
        .expect("memory namespace policy activation row should exist")
        .memory_write_chain_stage_pass_count = 6;
    assert!(!non_write_chain_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_memory_write_chain_receipt_false_green() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut stale_receipt = matrix.clone();
    stale_receipt
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness)
        .expect("memory write-chain receipt freshness activation row should exist")
        .memory_write_chain_receipt_freshness_pass_count = 5;
    assert!(!stale_receipt.has_matrix_integrity());

    let mut digest_drift = matrix.clone();
    digest_drift
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness)
        .expect("memory write-chain receipt freshness activation row should exist")
        .memory_write_chain_receipt_digest_count = 5;
    assert!(!digest_drift.has_matrix_integrity());

    let mut persistence_drift = matrix.clone();
    persistence_drift
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness)
        .expect("memory write-chain receipt freshness activation row should exist")
        .memory_write_chain_receipt_persisted_count = 1;
    assert!(!persistence_drift.has_matrix_integrity());

    let mut non_receipt_leak = matrix.clone();
    non_receipt_leak
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryWriteChainReadiness)
        .expect("memory write-chain readiness activation row should exist")
        .memory_write_chain_receipt_projected_count = 18;
    assert!(!non_receipt_leak.has_matrix_integrity());
}
