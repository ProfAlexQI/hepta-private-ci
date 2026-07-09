use serde::Deserialize;
use serde::Serialize;

use super::section::ContextPlaneStatusKind;
use super::section::ContextPlaneStatusSection;
use crate::memory::ContextMemoryNamespacePolicyReport;
use crate::memory::ContextMemoryRankedRecallShadowEvalReport;
use crate::memory::ContextMemoryRankedRecallShadowHybridSignal;
use crate::memory::ContextMemoryRecallQualityGateBlockerReason;
use crate::memory::ContextMemoryRecallQualityGateReport;
use crate::memory::ContextMemoryShadowCanaryPromotionReadinessReport;
use crate::memory::ContextMemoryShadowQualityTrendSnapshotReport;
use crate::memory::ContextMemoryTemporalGraphShadowEvalReport;
use crate::memory::ContextMemoryTemporalGraphShadowReplayReport;
use crate::memory::ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport;
use crate::memory::ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport;
use crate::memory::ContextMemoryTemporalGraphShadowStoreReport;
use crate::memory::ContextMemoryTemporalGraphShadowTraversalDiffReport;
use crate::memory::ContextMemoryTemporalGraphShadowTraversalQualityReport;
use crate::memory::ContextMemoryWriteChainReadinessReport;
use crate::memory::ContextMemoryWriteChainReceiptFreshnessReport;
use crate::memory::MemoryProviderReport;
use crate::memory::MemoryProviderV2AuditReport;

const CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT: usize = 4;
const MEMORY_NAMESPACE_POLICY_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_NAMESPACE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_STAGE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_RECEIPT_NAMESPACE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_RECEIPT_REQUIRED_COUNT: usize = 18;
const MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_STAGE_REQUIRED_COUNT: usize = 6;
const MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_STAGE_REQUIRED_COUNT: usize = 6;
const MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_STAGE_REQUIRED_COUNT: usize = 5;
const MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_STAGE_REQUIRED_COUNT: usize = 5;
const MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_STAGE_REQUIRED_COUNT: usize = 5;
const MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_STAGE_REQUIRED_COUNT: usize = 6;
const MEMORY_PROVIDER_V2_LIFECYCLE_REQUIRED_COUNT: usize = 6;
const RANKED_RECALL_HYBRID_SIGNAL_REQUIRED_COUNT: usize = 5;
const RANKED_RECALL_POSITIVE_HYBRID_SIGNAL_REQUIRED_COUNT: usize = 15;
const RANKED_RECALL_HYBRID_REGRESSION_BLOCKED_REQUIRED_COUNT: usize = 1;
const RANKED_RECALL_HYBRID_SIGNAL_MIN_BASIS_POINTS: u32 = 6_000;
const RANKED_RECALL_MIN_POSITIVE_HYBRID_SCORE_BASIS_POINTS: u32 = 7_800;
const RANKED_RECALL_ROUTING_DIFF_FIXTURE_REQUIRED_COUNT: usize = 4;
const RANKED_RECALL_ROUTING_DIFF_WIN_REQUIRED_COUNT: usize = 3;
const RANKED_RECALL_ROUTING_DIFF_LOSS_REQUIRED_COUNT: usize = 1;
const RANKED_RECALL_ROUTING_DIFF_REGRESSION_BLOCKED_REQUIRED_COUNT: usize = 1;
const RANKED_RECALL_ROUTING_DIFF_DELTA_MIN_BASIS_POINTS: i32 = 400;
const RANKED_RECALL_MIN_POSITIVE_ROUTING_DIFF_DELTA_BASIS_POINTS: i32 = 640;
const RANKED_RECALL_ROUTING_DIFF_LATENCY_DELTA_MAX_MS: i32 = 20;
const RANKED_RECALL_MAX_POSITIVE_ROUTING_DIFF_LATENCY_DELTA_MS: i32 = 10;
const RANKED_RECALL_ROUTING_DIFF_TOKEN_TRADEOFF_MIN_BASIS_POINTS: u32 = 1_000;
const RANKED_RECALL_MIN_POSITIVE_ROUTING_DIFF_TOKEN_TRADEOFF_BASIS_POINTS: u32 = 3_000;
const RANKED_RECALL_REAL_WORKLOAD_TRACE_FIXTURE_REQUIRED_COUNT: usize = 4;
const RANKED_RECALL_REAL_WORKLOAD_TRACE_SLO_PASS_REQUIRED_COUNT: usize = 3;
const RANKED_RECALL_REAL_WORKLOAD_TRACE_WIN_REQUIRED_COUNT: usize = 3;
const RANKED_RECALL_REAL_WORKLOAD_TRACE_LOSS_REQUIRED_COUNT: usize = 1;
const RANKED_RECALL_REAL_WORKLOAD_TRACE_OPERATOR_REVIEW_REQUIRED_COUNT: usize = 4;
const RANKED_RECALL_REAL_WORKLOAD_TRACE_LEAK_RATE_MAX_BASIS_POINTS: u32 = 0;
const RANKED_RECALL_MIN_POSITIVE_REAL_WORKLOAD_TRACE_COVERAGE_BASIS_POINTS: u32 = 8_000;
const RANKED_RECALL_MIN_POSITIVE_REAL_WORKLOAD_TRACE_PRECISION_BASIS_POINTS: u32 = 8_000;
const RANKED_RECALL_TOTAL_POSITIVE_REAL_WORKLOAD_TRACE_TOKEN_SAVED_MIN: usize = 2_140;
const RANKED_RECALL_MAX_POSITIVE_REAL_WORKLOAD_TRACE_LATENCY_MS: u32 = 55;
const RANKED_RECALL_REAL_WORKLOAD_TRACE_REGRESSION_LOSS_REQUIRED_COUNT: usize = 1;
const RANKED_RECALL_CANARY_PRECONDITION_REQUIRED_COUNT: usize = 4;

/// One payload-light context-plane status row.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextPlaneStatusEntry {
    pub section: ContextPlaneStatusSection,
    pub status: ContextPlaneStatusKind,
    pub observed_count: usize,
    pub omitted_count: usize,
    pub blocker_count: usize,
    pub recall_quality_blocking_reason_count: usize,
    pub recall_quality_blocking_reasons: Vec<ContextMemoryRecallQualityGateBlockerReason>,
    pub canary_promotion_required_stable_window_count: usize,
    pub canary_promotion_observed_stable_window_count: usize,
    pub canary_promotion_required_pass_streak: usize,
    pub canary_promotion_observed_pass_streak: usize,
    pub canary_promotion_blocker_count: usize,
    pub canary_promotion_checklist_required_count: usize,
    pub canary_promotion_checklist_pass_count: usize,
    pub canary_promotion_readiness_check_pass: bool,
    pub canary_promotion_negative_rehearsal_check_pass: bool,
    pub canary_promotion_audit_digest_check_pass: bool,
    pub canary_promotion_audit_freshness_check_pass: bool,
    pub canary_promotion_rollback_rehearsal_count: usize,
    pub canary_promotion_rollback_rehearsal_pass_count: usize,
    pub canary_promotion_kill_switch_rehearsal_count: usize,
    pub canary_promotion_kill_switch_rehearsal_pass_count: usize,
    pub canary_promotion_soak_readback_window_count: usize,
    pub canary_promotion_soak_readback_pass_count: usize,
    pub memory_provider_v2_lifecycle_required_count: usize,
    pub memory_provider_v2_lifecycle_pass_count: usize,
    pub memory_provider_v2_query_check_pass: bool,
    pub memory_provider_v2_update_context_check_pass: bool,
    pub memory_provider_v2_propose_write_check_pass: bool,
    pub memory_provider_v2_add_check_pass: bool,
    pub memory_provider_v2_clear_check_pass: bool,
    pub memory_provider_v2_close_check_pass: bool,
    pub memory_provider_v2_candidate_count: usize,
    pub memory_provider_v2_operator_review_required_count: usize,
    pub memory_namespace_policy_namespace_count: usize,
    pub memory_namespace_policy_operator_approval_required_count: usize,
    pub memory_namespace_policy_shadow_wal_required_count: usize,
    pub memory_namespace_policy_readback_required_count: usize,
    pub memory_namespace_policy_canary_required_count: usize,
    pub memory_namespace_policy_rollback_supported_count: usize,
    pub memory_namespace_policy_production_write_count: usize,
    pub memory_namespace_policy_graph_write_count: usize,
    pub memory_write_chain_namespace_count: usize,
    pub memory_write_chain_stage_required_count: usize,
    pub memory_write_chain_stage_pass_count: usize,
    pub memory_write_chain_propose_write_ready_count: usize,
    pub memory_write_chain_policy_approval_ready_count: usize,
    pub memory_write_chain_operator_approval_ready_count: usize,
    pub memory_write_chain_shadow_wal_ready_count: usize,
    pub memory_write_chain_readback_ready_count: usize,
    pub memory_write_chain_canary_ready_count: usize,
    pub memory_write_chain_rollback_ready_count: usize,
    pub memory_write_chain_production_write_count: usize,
    pub memory_write_chain_graph_write_count: usize,
    pub memory_write_chain_receipt_namespace_count: usize,
    pub memory_write_chain_receipt_required_count: usize,
    pub memory_write_chain_receipt_projected_count: usize,
    pub memory_write_chain_receipt_digest_count: usize,
    pub memory_write_chain_receipt_freshness_pass_count: usize,
    pub memory_write_chain_receipt_replay_guard_pass_count: usize,
    pub memory_write_chain_receipt_stale_replay_rejected_count: usize,
    pub memory_write_chain_receipt_recorded_count: usize,
    pub memory_write_chain_receipt_persisted_count: usize,
    pub memory_write_chain_receipt_production_write_count: usize,
    pub memory_write_chain_receipt_graph_write_count: usize,
    pub memory_temporal_graph_shadow_store_node_count: usize,
    pub memory_temporal_graph_shadow_store_edge_count: usize,
    pub memory_temporal_graph_shadow_store_provenance_edge_count: usize,
    pub memory_temporal_graph_shadow_store_validity_window_edge_count: usize,
    pub memory_temporal_graph_shadow_store_supersedes_edge_count: usize,
    pub memory_temporal_graph_shadow_store_invalidated_node_count: usize,
    pub memory_temporal_graph_shadow_store_stage_required_count: usize,
    pub memory_temporal_graph_shadow_store_stage_projected_count: usize,
    pub memory_temporal_graph_shadow_store_digest_count: usize,
    pub memory_temporal_graph_shadow_store_freshness_pass_count: usize,
    pub memory_temporal_graph_shadow_store_replay_guard_pass_count: usize,
    pub memory_temporal_graph_shadow_store_stale_replay_rejected_count: usize,
    pub memory_temporal_graph_shadow_store_operator_approval_required_count: usize,
    pub memory_temporal_graph_shadow_store_operator_approval_recorded_count: usize,
    pub memory_temporal_graph_shadow_store_recorded_receipt_count: usize,
    pub memory_temporal_graph_shadow_store_persisted_receipt_count: usize,
    pub memory_temporal_graph_shadow_store_production_write_count: usize,
    pub memory_temporal_graph_shadow_store_graph_write_count: usize,
    pub memory_temporal_graph_shadow_replay_node_count: usize,
    pub memory_temporal_graph_shadow_replay_edge_count: usize,
    pub memory_temporal_graph_shadow_replay_provenance_count: usize,
    pub memory_temporal_graph_shadow_replay_bitemporal_validity_count: usize,
    pub memory_temporal_graph_shadow_replay_fact_invalidation_count: usize,
    pub memory_temporal_graph_shadow_replay_supersede_tombstone_count: usize,
    pub memory_temporal_graph_shadow_replay_stage_required_count: usize,
    pub memory_temporal_graph_shadow_replay_stage_projected_count: usize,
    pub memory_temporal_graph_shadow_replay_digest_count: usize,
    pub memory_temporal_graph_shadow_replay_freshness_pass_count: usize,
    pub memory_temporal_graph_shadow_replay_guard_pass_count: usize,
    pub memory_temporal_graph_shadow_replay_stale_replay_rejected_count: usize,
    pub memory_temporal_graph_shadow_replay_operator_approval_required_count: usize,
    pub memory_temporal_graph_shadow_replay_operator_approval_recorded_count: usize,
    pub memory_temporal_graph_shadow_replay_recorded_receipt_count: usize,
    pub memory_temporal_graph_shadow_replay_persisted_receipt_count: usize,
    pub memory_temporal_graph_shadow_replay_production_write_count: usize,
    pub memory_temporal_graph_shadow_replay_graph_write_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_production_selection_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_lexical_bm25_candidate_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_semantic_candidate_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_hybrid_candidate_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_overlap_candidate_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_win_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_loss_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_cost_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_stage_required_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_stage_projected_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_digest_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_freshness_pass_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_replay_guard_pass_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_stale_replay_rejected_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_llm_rerank_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_graph_persistence_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_production_route_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_production_write_count: usize,
    pub memory_temporal_graph_shadow_traversal_diff_graph_write_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_fixture_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_slo_required_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_slo_pass_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_coverage_basis_points: u32,
    pub memory_temporal_graph_shadow_traversal_quality_precision_basis_points: u32,
    pub memory_temporal_graph_shadow_traversal_quality_leak_rate_basis_points: u32,
    pub memory_temporal_graph_shadow_traversal_quality_latency_budget_ms: u32,
    pub memory_temporal_graph_shadow_traversal_quality_projected_latency_ms: u32,
    pub memory_temporal_graph_shadow_traversal_quality_token_saved_estimate: usize,
    pub memory_temporal_graph_shadow_traversal_quality_operator_review_required_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_win_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_loss_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_cost_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_stage_required_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_stage_projected_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_digest_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_freshness_pass_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_replay_guard_pass_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_stale_replay_rejected_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_llm_rerank_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_graph_persistence_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_production_route_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_production_write_count: usize,
    pub memory_temporal_graph_shadow_traversal_quality_graph_write_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_stage_projected_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_quality_slo_pass_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_required_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_recorded_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_enabled_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_registered_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_ready_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_required_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_pass_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_activation_denial_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_digest_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_freshness_pass_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_replay_guard_pass_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_stale_replay_rejected_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_llm_rerank_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_graph_persistence_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_production_route_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_production_write_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_graph_write_count: usize,
    pub memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_projected_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_guard_pass_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_required_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_recorded_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_registered_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_enabled_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_registered_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_readback_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_pass_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_required_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_readback_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_pass_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_denial_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_route_opened_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_digest_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_freshness_pass_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_replay_guard_pass_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stale_replay_rejected_count:
        usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_llm_rerank_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_persistence_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_route_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_write_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_write_count: usize,
    pub memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_count: usize,
    pub ranked_recall_hybrid_signal_required_count: usize,
    pub ranked_recall_hybrid_signal_pass_count: usize,
    pub ranked_recall_lexical_bm25_check_pass: bool,
    pub ranked_recall_recency_check_pass: bool,
    pub ranked_recall_source_authority_check_pass: bool,
    pub ranked_recall_temporal_validity_check_pass: bool,
    pub ranked_recall_feedback_check_pass: bool,
    pub ranked_recall_positive_hybrid_signal_required_count: usize,
    pub ranked_recall_positive_hybrid_signal_pass_count: usize,
    pub ranked_recall_hybrid_regression_blocked_count: usize,
    pub ranked_recall_hybrid_signal_min_basis_points: u32,
    pub ranked_recall_min_positive_hybrid_score_basis_points: u32,
    pub ranked_recall_routing_diff_fixture_count: usize,
    pub ranked_recall_routing_diff_shadow_only_count: usize,
    pub ranked_recall_routing_diff_win_count: usize,
    pub ranked_recall_routing_diff_loss_count: usize,
    pub ranked_recall_routing_diff_regression_blocked_count: usize,
    pub ranked_recall_routing_diff_delta_min_basis_points: i32,
    pub ranked_recall_min_positive_routing_diff_delta_basis_points: i32,
    pub ranked_recall_routing_diff_latency_delta_max_ms: i32,
    pub ranked_recall_max_positive_routing_diff_latency_delta_ms: i32,
    pub ranked_recall_routing_diff_token_tradeoff_min_basis_points: u32,
    pub ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: u32,
    pub ranked_recall_real_workload_trace_fixture_count: usize,
    pub ranked_recall_real_workload_trace_shadow_only_count: usize,
    pub ranked_recall_real_workload_trace_slo_pass_count: usize,
    pub ranked_recall_real_workload_trace_win_count: usize,
    pub ranked_recall_real_workload_trace_loss_count: usize,
    pub ranked_recall_real_workload_trace_operator_review_required_count: usize,
    pub ranked_recall_real_workload_trace_total_leak_count: usize,
    pub ranked_recall_real_workload_trace_max_leak_rate_basis_points: u32,
    pub ranked_recall_min_positive_real_workload_trace_coverage_basis_points: u32,
    pub ranked_recall_min_positive_real_workload_trace_precision_basis_points: u32,
    pub ranked_recall_total_positive_real_workload_trace_token_saved: usize,
    pub ranked_recall_max_positive_real_workload_trace_latency_ms: u32,
    pub ranked_recall_real_workload_trace_regression_loss_count: usize,
    pub ranked_recall_canary_precondition_fixture_count: usize,
    pub ranked_recall_canary_precondition_shadow_only_count: usize,
    pub ranked_recall_canary_precondition_pass_count: usize,
    pub ranked_recall_canary_feature_flag_registered_count: usize,
    pub ranked_recall_canary_feature_flag_disabled_count: usize,
    pub ranked_recall_canary_kill_switch_registered_count: usize,
    pub ranked_recall_canary_kill_switch_enabled_count: usize,
    pub ranked_recall_canary_rollback_rehearsal_covered_count: usize,
    pub ranked_recall_canary_activation_denial_covered_count: usize,
    pub ranked_recall_canary_precondition_operator_review_required_count: usize,
    pub ranked_recall_canary_precondition_route_opened_count: usize,
    pub ranked_recall_canary_precondition_rollback_write_count: usize,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl ContextPlaneStatusEntry {
    pub(in crate::memory) fn ready(
        section: ContextPlaneStatusSection,
        observed_count: usize,
    ) -> Self {
        Self {
            section,
            status: ContextPlaneStatusKind::Ready,
            observed_count,
            ..Self::default()
        }
    }

    pub(in crate::memory) fn shadow(
        section: ContextPlaneStatusSection,
        observed_count: usize,
    ) -> Self {
        Self {
            section,
            status: ContextPlaneStatusKind::Shadow,
            observed_count,
            ..Self::default()
        }
    }

    pub(in crate::memory) fn disabled(section: ContextPlaneStatusSection) -> Self {
        Self {
            section,
            status: ContextPlaneStatusKind::Disabled,
            observed_count: 1,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_integrity(
        section: ContextPlaneStatusSection,
        integrity: bool,
        observed_count: usize,
        omitted_count: usize,
    ) -> Self {
        Self {
            section,
            status: if integrity {
                ContextPlaneStatusKind::Ready
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count,
            omitted_count,
            blocker_count: usize::from(!integrity),
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn shadow_from_integrity(
        section: ContextPlaneStatusSection,
        integrity: bool,
        observed_count: usize,
        omitted_count: usize,
    ) -> Self {
        Self {
            section,
            status: if integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count,
            omitted_count,
            blocker_count: usize::from(!integrity),
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_recall_quality_gate(
        recall_quality_gate: &ContextMemoryRecallQualityGateReport,
    ) -> Self {
        let recall_quality_blocking_reasons =
            recall_quality_status_blocking_reasons(recall_quality_gate);
        let status = if recall_quality_gate.has_quality_gate_integrity() {
            ContextPlaneStatusKind::Ready
        } else {
            ContextPlaneStatusKind::Blocked
        };
        let blocker_count = match status {
            ContextPlaneStatusKind::Blocked => recall_quality_blocking_reasons.len().max(1),
            ContextPlaneStatusKind::Ready
            | ContextPlaneStatusKind::Shadow
            | ContextPlaneStatusKind::Disabled
            | ContextPlaneStatusKind::Unknown => 0,
        };

        Self {
            section: ContextPlaneStatusSection::RecallQualityGate,
            status,
            observed_count: recall_quality_gate.fixture_count,
            omitted_count: recall_quality_gate.missing_critical_fact_count,
            blocker_count,
            recall_quality_blocking_reason_count: recall_quality_blocking_reasons.len(),
            recall_quality_blocking_reasons,
            production_write: recall_quality_gate.production_write,
            graph_write: recall_quality_gate.graph_write,
            runtime_activation: recall_quality_gate.runtime_activation,
            prompt_assembly_change: recall_quality_gate.prompt_assembly_change,
            operator_activation_allowed: recall_quality_gate.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_temporal_graph_shadow_eval(
        temporal_graph_shadow_eval: &ContextMemoryTemporalGraphShadowEvalReport,
    ) -> Self {
        let has_integrity = temporal_graph_shadow_eval.has_temporal_graph_shadow_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryTemporalGraphShadowEval,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: temporal_graph_shadow_eval.fixture_count(),
            omitted_count: temporal_graph_shadow_eval
                .fixture_count()
                .saturating_sub(temporal_graph_shadow_eval.fixture_pass_count()),
            blocker_count: usize::from(!has_integrity),
            production_write: temporal_graph_shadow_eval.production_write,
            graph_write: temporal_graph_shadow_eval.graph_write,
            runtime_activation: temporal_graph_shadow_eval.runtime_activation,
            prompt_assembly_change: temporal_graph_shadow_eval.prompt_assembly_change,
            operator_activation_allowed: temporal_graph_shadow_eval.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_temporal_graph_shadow_store(
        temporal_graph_shadow_store: &ContextMemoryTemporalGraphShadowStoreReport,
    ) -> Self {
        let has_integrity = temporal_graph_shadow_store.has_shadow_store_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryTemporalGraphShadowStore,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: temporal_graph_shadow_store.node_count,
            omitted_count: temporal_graph_shadow_store.receipt_recorded_count()
                + temporal_graph_shadow_store.receipt_persisted_count()
                + temporal_graph_shadow_store.production_write_count()
                + temporal_graph_shadow_store.graph_write_count(),
            blocker_count: usize::from(!has_integrity),
            memory_temporal_graph_shadow_store_node_count: temporal_graph_shadow_store.node_count,
            memory_temporal_graph_shadow_store_edge_count: temporal_graph_shadow_store.edge_count,
            memory_temporal_graph_shadow_store_provenance_edge_count: temporal_graph_shadow_store
                .provenance_edge_count,
            memory_temporal_graph_shadow_store_validity_window_edge_count:
                temporal_graph_shadow_store.validity_window_edge_count,
            memory_temporal_graph_shadow_store_supersedes_edge_count: temporal_graph_shadow_store
                .supersedes_edge_count,
            memory_temporal_graph_shadow_store_invalidated_node_count: temporal_graph_shadow_store
                .invalidated_node_count,
            memory_temporal_graph_shadow_store_stage_required_count: temporal_graph_shadow_store
                .readiness_stage_required_count(),
            memory_temporal_graph_shadow_store_stage_projected_count: temporal_graph_shadow_store
                .readiness_stage_projected_count(),
            memory_temporal_graph_shadow_store_digest_count: usize::from(
                !temporal_graph_shadow_store.store_digest.is_empty(),
            ),
            memory_temporal_graph_shadow_store_freshness_pass_count: usize::from(
                temporal_graph_shadow_store.freshness_check_pass,
            ),
            memory_temporal_graph_shadow_store_replay_guard_pass_count: usize::from(
                temporal_graph_shadow_store.replay_guard_pass,
            ),
            memory_temporal_graph_shadow_store_stale_replay_rejected_count: usize::from(
                temporal_graph_shadow_store.stale_replay_rejected,
            ),
            memory_temporal_graph_shadow_store_operator_approval_required_count: usize::from(
                temporal_graph_shadow_store.operator_approval_required,
            ),
            memory_temporal_graph_shadow_store_operator_approval_recorded_count: usize::from(
                temporal_graph_shadow_store.operator_approval_recorded,
            ),
            memory_temporal_graph_shadow_store_recorded_receipt_count: temporal_graph_shadow_store
                .receipt_recorded_count(),
            memory_temporal_graph_shadow_store_persisted_receipt_count: temporal_graph_shadow_store
                .receipt_persisted_count(),
            memory_temporal_graph_shadow_store_production_write_count: temporal_graph_shadow_store
                .production_write_count(),
            memory_temporal_graph_shadow_store_graph_write_count: temporal_graph_shadow_store
                .graph_write_count(),
            production_write: temporal_graph_shadow_store.production_write
                || temporal_graph_shadow_store.production_route,
            graph_write: temporal_graph_shadow_store.graph_write,
            runtime_activation: temporal_graph_shadow_store.runtime_activation,
            prompt_assembly_change: temporal_graph_shadow_store.prompt_assembly_change
                || temporal_graph_shadow_store.hot_path_write,
            operator_activation_allowed: temporal_graph_shadow_store.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_temporal_graph_shadow_replay(
        temporal_graph_shadow_replay: &ContextMemoryTemporalGraphShadowReplayReport,
    ) -> Self {
        let has_integrity = temporal_graph_shadow_replay.has_shadow_replay_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: temporal_graph_shadow_replay.node_count,
            omitted_count: temporal_graph_shadow_replay.receipt_recorded_count()
                + temporal_graph_shadow_replay.receipt_persisted_count()
                + temporal_graph_shadow_replay.production_write_count()
                + temporal_graph_shadow_replay.graph_write_count(),
            blocker_count: usize::from(!has_integrity),
            memory_temporal_graph_shadow_replay_node_count: temporal_graph_shadow_replay.node_count,
            memory_temporal_graph_shadow_replay_edge_count: temporal_graph_shadow_replay.edge_count,
            memory_temporal_graph_shadow_replay_provenance_count: temporal_graph_shadow_replay
                .provenance_replay_count,
            memory_temporal_graph_shadow_replay_bitemporal_validity_count:
                temporal_graph_shadow_replay.bitemporal_validity_replay_count,
            memory_temporal_graph_shadow_replay_fact_invalidation_count:
                temporal_graph_shadow_replay.fact_invalidation_replay_count,
            memory_temporal_graph_shadow_replay_supersede_tombstone_count:
                temporal_graph_shadow_replay.supersede_tombstone_replay_count,
            memory_temporal_graph_shadow_replay_stage_required_count: temporal_graph_shadow_replay
                .replay_stage_required_count(),
            memory_temporal_graph_shadow_replay_stage_projected_count: temporal_graph_shadow_replay
                .replay_stage_projected_count(),
            memory_temporal_graph_shadow_replay_digest_count: temporal_graph_shadow_replay
                .replay_digest_count(),
            memory_temporal_graph_shadow_replay_freshness_pass_count: temporal_graph_shadow_replay
                .freshness_pass_count(),
            memory_temporal_graph_shadow_replay_guard_pass_count: temporal_graph_shadow_replay
                .replay_guard_pass_count(),
            memory_temporal_graph_shadow_replay_stale_replay_rejected_count:
                temporal_graph_shadow_replay.stale_replay_rejected_count(),
            memory_temporal_graph_shadow_replay_operator_approval_required_count: usize::from(
                temporal_graph_shadow_replay.operator_approval_required,
            ),
            memory_temporal_graph_shadow_replay_operator_approval_recorded_count: usize::from(
                temporal_graph_shadow_replay.operator_approval_recorded,
            ),
            memory_temporal_graph_shadow_replay_recorded_receipt_count:
                temporal_graph_shadow_replay.receipt_recorded_count(),
            memory_temporal_graph_shadow_replay_persisted_receipt_count:
                temporal_graph_shadow_replay.receipt_persisted_count(),
            memory_temporal_graph_shadow_replay_production_write_count:
                temporal_graph_shadow_replay.production_write_count(),
            memory_temporal_graph_shadow_replay_graph_write_count: temporal_graph_shadow_replay
                .graph_write_count(),
            production_write: temporal_graph_shadow_replay.production_write
                || temporal_graph_shadow_replay.production_route,
            graph_write: temporal_graph_shadow_replay.graph_write,
            runtime_activation: temporal_graph_shadow_replay.runtime_activation,
            prompt_assembly_change: temporal_graph_shadow_replay.prompt_assembly_change
                || temporal_graph_shadow_replay.hot_path_write,
            operator_activation_allowed: temporal_graph_shadow_replay.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_temporal_graph_shadow_traversal_diff(
        traversal_diff: &ContextMemoryTemporalGraphShadowTraversalDiffReport,
    ) -> Self {
        let has_integrity = traversal_diff.has_traversal_diff_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalDiff,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: traversal_diff.production_selection_count,
            omitted_count: traversal_diff.llm_rerank_count()
                + traversal_diff.graph_persistence_count()
                + traversal_diff.production_route_count()
                + traversal_diff.production_write_count()
                + traversal_diff.graph_write_count(),
            blocker_count: usize::from(!has_integrity),
            memory_temporal_graph_shadow_traversal_diff_production_selection_count: traversal_diff
                .production_selection_count,
            memory_temporal_graph_shadow_traversal_diff_lexical_bm25_candidate_count:
                traversal_diff.lexical_bm25_candidate_count,
            memory_temporal_graph_shadow_traversal_diff_semantic_candidate_count: traversal_diff
                .semantic_candidate_count,
            memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count:
                traversal_diff.graph_traversal_candidate_count,
            memory_temporal_graph_shadow_traversal_diff_hybrid_candidate_count: traversal_diff
                .hybrid_candidate_count,
            memory_temporal_graph_shadow_traversal_diff_overlap_candidate_count: traversal_diff
                .overlap_candidate_count,
            memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count:
                traversal_diff.graph_expansion_candidate_count,
            memory_temporal_graph_shadow_traversal_diff_win_count: traversal_diff
                .traversal_diff_win_count,
            memory_temporal_graph_shadow_traversal_diff_loss_count: traversal_diff
                .traversal_diff_loss_count,
            memory_temporal_graph_shadow_traversal_diff_cost_count: traversal_diff
                .traversal_diff_cost_count,
            memory_temporal_graph_shadow_traversal_diff_stage_required_count: traversal_diff
                .traversal_stage_required_count(),
            memory_temporal_graph_shadow_traversal_diff_stage_projected_count: traversal_diff
                .traversal_stage_projected_count(),
            memory_temporal_graph_shadow_traversal_diff_digest_count: traversal_diff
                .traversal_digest_count(),
            memory_temporal_graph_shadow_traversal_diff_freshness_pass_count: traversal_diff
                .freshness_pass_count(),
            memory_temporal_graph_shadow_traversal_diff_replay_guard_pass_count: traversal_diff
                .replay_guard_pass_count(),
            memory_temporal_graph_shadow_traversal_diff_stale_replay_rejected_count: traversal_diff
                .stale_replay_rejected_count(),
            memory_temporal_graph_shadow_traversal_diff_llm_rerank_count: traversal_diff
                .llm_rerank_count(),
            memory_temporal_graph_shadow_traversal_diff_graph_persistence_count: traversal_diff
                .graph_persistence_count(),
            memory_temporal_graph_shadow_traversal_diff_production_route_count: traversal_diff
                .production_route_count(),
            memory_temporal_graph_shadow_traversal_diff_production_write_count: traversal_diff
                .production_write_count(),
            memory_temporal_graph_shadow_traversal_diff_graph_write_count: traversal_diff
                .graph_write_count(),
            production_write: traversal_diff.production_write || traversal_diff.production_route,
            graph_write: traversal_diff.graph_write,
            runtime_activation: traversal_diff.runtime_activation,
            prompt_assembly_change: traversal_diff.prompt_assembly_change
                || traversal_diff.hot_path_write,
            operator_activation_allowed: traversal_diff.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_temporal_graph_shadow_traversal_quality(
        traversal_quality: &ContextMemoryTemporalGraphShadowTraversalQualityReport,
    ) -> Self {
        let has_integrity = traversal_quality.has_traversal_quality_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: traversal_quality.quality_fixture_count,
            omitted_count: traversal_quality.llm_rerank_count()
                + traversal_quality.graph_persistence_count()
                + traversal_quality.production_route_count()
                + traversal_quality.production_write_count()
                + traversal_quality.graph_write_count(),
            blocker_count: usize::from(!has_integrity),
            memory_temporal_graph_shadow_traversal_quality_fixture_count: traversal_quality
                .quality_fixture_count,
            memory_temporal_graph_shadow_traversal_quality_slo_required_count: traversal_quality
                .quality_slo_required_count,
            memory_temporal_graph_shadow_traversal_quality_slo_pass_count: traversal_quality
                .quality_slo_pass_count,
            memory_temporal_graph_shadow_traversal_quality_coverage_basis_points: traversal_quality
                .coverage_basis_points,
            memory_temporal_graph_shadow_traversal_quality_precision_basis_points:
                traversal_quality.precision_basis_points,
            memory_temporal_graph_shadow_traversal_quality_leak_rate_basis_points:
                traversal_quality.leak_rate_basis_points,
            memory_temporal_graph_shadow_traversal_quality_latency_budget_ms: traversal_quality
                .latency_budget_ms,
            memory_temporal_graph_shadow_traversal_quality_projected_latency_ms: traversal_quality
                .projected_latency_ms,
            memory_temporal_graph_shadow_traversal_quality_token_saved_estimate: traversal_quality
                .token_saved_estimate,
            memory_temporal_graph_shadow_traversal_quality_operator_review_required_count:
                traversal_quality.operator_review_required_count,
            memory_temporal_graph_shadow_traversal_quality_win_count: traversal_quality
                .traversal_win_count,
            memory_temporal_graph_shadow_traversal_quality_loss_count: traversal_quality
                .traversal_loss_count,
            memory_temporal_graph_shadow_traversal_quality_cost_count: traversal_quality
                .traversal_cost_count,
            memory_temporal_graph_shadow_traversal_quality_stage_required_count: traversal_quality
                .traversal_quality_stage_required_count(),
            memory_temporal_graph_shadow_traversal_quality_stage_projected_count: traversal_quality
                .traversal_quality_stage_projected_count(),
            memory_temporal_graph_shadow_traversal_quality_digest_count: traversal_quality
                .traversal_quality_digest_count(),
            memory_temporal_graph_shadow_traversal_quality_freshness_pass_count: traversal_quality
                .freshness_pass_count(),
            memory_temporal_graph_shadow_traversal_quality_replay_guard_pass_count:
                traversal_quality.replay_guard_pass_count(),
            memory_temporal_graph_shadow_traversal_quality_stale_replay_rejected_count:
                traversal_quality.stale_replay_rejected_count(),
            memory_temporal_graph_shadow_traversal_quality_llm_rerank_count: traversal_quality
                .llm_rerank_count(),
            memory_temporal_graph_shadow_traversal_quality_graph_persistence_count:
                traversal_quality.graph_persistence_count(),
            memory_temporal_graph_shadow_traversal_quality_production_route_count:
                traversal_quality.production_route_count(),
            memory_temporal_graph_shadow_traversal_quality_production_write_count:
                traversal_quality.production_write_count(),
            memory_temporal_graph_shadow_traversal_quality_graph_write_count: traversal_quality
                .graph_write_count(),
            production_write: traversal_quality.production_write
                || traversal_quality.production_route,
            graph_write: traversal_quality.graph_write,
            runtime_activation: traversal_quality.runtime_activation,
            prompt_assembly_change: traversal_quality.prompt_assembly_change
                || traversal_quality.hot_path_write,
            operator_activation_allowed: traversal_quality.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_temporal_graph_shadow_retrieval_canary_guard(
        retrieval_canary_guard: &ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport,
    ) -> Self {
        let has_integrity = retrieval_canary_guard.has_retrieval_canary_guard_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: retrieval_canary_guard.guard_fixture_count,
            omitted_count: retrieval_canary_guard.llm_rerank_count()
                + retrieval_canary_guard.graph_persistence_count()
                + retrieval_canary_guard.production_route_count()
                + retrieval_canary_guard.production_write_count()
                + retrieval_canary_guard.graph_write_count()
                + retrieval_canary_guard.rollback_write_count()
                + retrieval_canary_guard.canary_route_opened_count,
            blocker_count: usize::from(!has_integrity),
            memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count:
                retrieval_canary_guard.guard_fixture_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count:
                retrieval_canary_guard.guard_stage_required_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_stage_projected_count:
                retrieval_canary_guard.guard_stage_projected_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_quality_slo_pass_count:
                retrieval_canary_guard.quality_slo_pass_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_required_count:
                retrieval_canary_guard.operator_approval_required_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_recorded_count:
                retrieval_canary_guard.operator_approval_recorded_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count:
                retrieval_canary_guard.feature_flag_registered_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_enabled_count:
                retrieval_canary_guard.feature_flag_enabled_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_registered_count:
                retrieval_canary_guard.kill_switch_registered_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_ready_count:
                retrieval_canary_guard.kill_switch_ready_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_required_count:
                retrieval_canary_guard.rollback_rehearsal_required_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_pass_count:
                retrieval_canary_guard.rollback_rehearsal_pass_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_activation_denial_count:
                retrieval_canary_guard.activation_denial_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count:
                retrieval_canary_guard.canary_route_opened_count,
            memory_temporal_graph_shadow_retrieval_canary_guard_digest_count:
                retrieval_canary_guard.retrieval_canary_guard_digest_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_freshness_pass_count:
                retrieval_canary_guard.freshness_pass_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_replay_guard_pass_count:
                retrieval_canary_guard.replay_guard_pass_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_stale_replay_rejected_count:
                retrieval_canary_guard.stale_replay_rejected_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_llm_rerank_count:
                retrieval_canary_guard.llm_rerank_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_graph_persistence_count:
                retrieval_canary_guard.graph_persistence_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_production_route_count:
                retrieval_canary_guard.production_route_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_production_write_count:
                retrieval_canary_guard.production_write_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_graph_write_count:
                retrieval_canary_guard.graph_write_count(),
            memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count:
                retrieval_canary_guard.rollback_write_count(),
            production_write: retrieval_canary_guard.production_write
                || retrieval_canary_guard.production_route
                || retrieval_canary_guard.canary_route_opened_count > 0,
            graph_write: retrieval_canary_guard.graph_write,
            runtime_activation: retrieval_canary_guard.runtime_activation,
            prompt_assembly_change: retrieval_canary_guard.prompt_assembly_change
                || retrieval_canary_guard.hot_path_write,
            operator_activation_allowed: retrieval_canary_guard.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_temporal_graph_shadow_retrieval_rollback_kill_switch(
        retrieval_rollback_kill_switch:
            &ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport,
    ) -> Self {
        let has_integrity =
            retrieval_rollback_kill_switch.has_retrieval_rollback_kill_switch_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: retrieval_rollback_kill_switch.evidence_fixture_count,
            omitted_count: retrieval_rollback_kill_switch.llm_rerank_count()
                + retrieval_rollback_kill_switch.graph_persistence_count()
                + retrieval_rollback_kill_switch.production_route_count()
                + retrieval_rollback_kill_switch.production_write_count()
                + retrieval_rollback_kill_switch.graph_write_count()
                + retrieval_rollback_kill_switch.rollback_write_count()
                + retrieval_rollback_kill_switch.canary_route_opened_count,
            blocker_count: usize::from(!has_integrity),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count:
                retrieval_rollback_kill_switch.evidence_fixture_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count:
                retrieval_rollback_kill_switch.evidence_stage_required_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_projected_count:
                retrieval_rollback_kill_switch.evidence_stage_projected_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_guard_pass_count:
                retrieval_rollback_kill_switch.canary_guard_pass_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_required_count:
                retrieval_rollback_kill_switch.operator_approval_required_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_recorded_count:
                retrieval_rollback_kill_switch.operator_approval_recorded_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_registered_count:
                retrieval_rollback_kill_switch.feature_flag_registered_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_enabled_count:
                retrieval_rollback_kill_switch.feature_flag_enabled_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_registered_count:
                retrieval_rollback_kill_switch.kill_switch_registered_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_readback_count:
                retrieval_rollback_kill_switch.kill_switch_readback_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_pass_count:
                retrieval_rollback_kill_switch.kill_switch_pass_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_required_count:
                retrieval_rollback_kill_switch.rollback_rehearsal_required_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_readback_count:
                retrieval_rollback_kill_switch.rollback_rehearsal_readback_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_pass_count:
                retrieval_rollback_kill_switch.rollback_rehearsal_pass_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count:
                retrieval_rollback_kill_switch.route_denial_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_denial_count:
                retrieval_rollback_kill_switch.rollback_write_denial_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_route_opened_count:
                retrieval_rollback_kill_switch.canary_route_opened_count,
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_digest_count:
                retrieval_rollback_kill_switch.retrieval_rollback_kill_switch_digest_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_freshness_pass_count:
                retrieval_rollback_kill_switch.freshness_pass_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_replay_guard_pass_count:
                retrieval_rollback_kill_switch.replay_guard_pass_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stale_replay_rejected_count:
                retrieval_rollback_kill_switch.stale_replay_rejected_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_llm_rerank_count:
                retrieval_rollback_kill_switch.llm_rerank_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_persistence_count:
                retrieval_rollback_kill_switch.graph_persistence_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_route_count:
                retrieval_rollback_kill_switch.production_route_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_write_count:
                retrieval_rollback_kill_switch.production_write_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_write_count:
                retrieval_rollback_kill_switch.graph_write_count(),
            memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_count:
                retrieval_rollback_kill_switch.rollback_write_count(),
            production_write: retrieval_rollback_kill_switch.production_write
                || retrieval_rollback_kill_switch.production_route
                || retrieval_rollback_kill_switch.canary_route_opened_count > 0,
            graph_write: retrieval_rollback_kill_switch.graph_write,
            runtime_activation: retrieval_rollback_kill_switch.runtime_activation,
            prompt_assembly_change: retrieval_rollback_kill_switch.prompt_assembly_change
                || retrieval_rollback_kill_switch.hot_path_write,
            operator_activation_allowed: retrieval_rollback_kill_switch.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_provider_report(
        provider_report: &MemoryProviderReport,
    ) -> Self {
        let has_integrity = provider_report.has_provider_boundary_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryProviderBoundary,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: 1,
            blocker_count: usize::from(!has_integrity),
            production_write: provider_report.update_context.write_performed,
            runtime_activation: provider_report.update_context.runtime_activation,
            prompt_assembly_change: provider_report.update_context.prompt_payload_exported
                || provider_report.update_context.ranked_payload_exported,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_namespace_policy(
        namespace_policy: &ContextMemoryNamespacePolicyReport,
    ) -> Self {
        let has_integrity = namespace_policy.has_policy_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryNamespacePolicy,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: namespace_policy.namespace_count(),
            omitted_count: namespace_policy.production_write_count()
                + namespace_policy.graph_write_count(),
            blocker_count: usize::from(!has_integrity),
            memory_namespace_policy_namespace_count: namespace_policy.namespace_count(),
            memory_namespace_policy_operator_approval_required_count: namespace_policy
                .operator_approval_required_count(),
            memory_namespace_policy_shadow_wal_required_count: namespace_policy
                .shadow_wal_required_count(),
            memory_namespace_policy_readback_required_count: namespace_policy
                .readback_required_count(),
            memory_namespace_policy_canary_required_count: namespace_policy.canary_required_count(),
            memory_namespace_policy_rollback_supported_count: namespace_policy
                .rollback_supported_count(),
            memory_namespace_policy_production_write_count: namespace_policy
                .production_write_count(),
            memory_namespace_policy_graph_write_count: namespace_policy.graph_write_count(),
            production_write: namespace_policy.production_write,
            graph_write: namespace_policy.graph_write,
            runtime_activation: namespace_policy.runtime_activation,
            prompt_assembly_change: namespace_policy.prompt_assembly_change
                || namespace_policy.hot_path_write,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_write_chain_readiness(
        write_chain: &ContextMemoryWriteChainReadinessReport,
    ) -> Self {
        let has_integrity = write_chain.has_readiness_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryWriteChainReadiness,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: write_chain.namespace_count(),
            omitted_count: write_chain.production_write_count() + write_chain.graph_write_count(),
            blocker_count: usize::from(!has_integrity),
            memory_write_chain_namespace_count: write_chain.namespace_count(),
            memory_write_chain_stage_required_count: write_chain.stage_required_count(),
            memory_write_chain_stage_pass_count: write_chain.stage_pass_count(),
            memory_write_chain_propose_write_ready_count: write_chain.propose_write_ready_count(),
            memory_write_chain_policy_approval_ready_count: write_chain
                .policy_approval_ready_count(),
            memory_write_chain_operator_approval_ready_count: write_chain
                .operator_approval_ready_count(),
            memory_write_chain_shadow_wal_ready_count: write_chain.shadow_wal_ready_count(),
            memory_write_chain_readback_ready_count: write_chain.readback_ready_count(),
            memory_write_chain_canary_ready_count: write_chain.canary_ready_count(),
            memory_write_chain_rollback_ready_count: write_chain.rollback_ready_count(),
            memory_write_chain_production_write_count: write_chain.production_write_count(),
            memory_write_chain_graph_write_count: write_chain.graph_write_count(),
            production_write: write_chain.production_write,
            graph_write: write_chain.graph_write,
            runtime_activation: write_chain.runtime_activation,
            prompt_assembly_change: write_chain.prompt_assembly_change
                || write_chain.hot_path_write,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_write_chain_receipt_freshness(
        receipts: &ContextMemoryWriteChainReceiptFreshnessReport,
    ) -> Self {
        let has_integrity = receipts.has_receipt_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: receipts.namespace_count(),
            omitted_count: receipts.recorded_receipt_count()
                + receipts.persisted_receipt_count()
                + receipts.production_write_count()
                + receipts.graph_write_count(),
            blocker_count: usize::from(!has_integrity),
            memory_write_chain_receipt_namespace_count: receipts.namespace_count(),
            memory_write_chain_receipt_required_count: receipts.receipt_required_count(),
            memory_write_chain_receipt_projected_count: receipts.receipt_projected_count(),
            memory_write_chain_receipt_digest_count: receipts.receipt_digest_count(),
            memory_write_chain_receipt_freshness_pass_count: receipts.freshness_pass_count(),
            memory_write_chain_receipt_replay_guard_pass_count: receipts.replay_guard_pass_count(),
            memory_write_chain_receipt_stale_replay_rejected_count: receipts
                .stale_replay_rejected_count(),
            memory_write_chain_receipt_recorded_count: receipts.recorded_receipt_count(),
            memory_write_chain_receipt_persisted_count: receipts.persisted_receipt_count(),
            memory_write_chain_receipt_production_write_count: receipts.production_write_count(),
            memory_write_chain_receipt_graph_write_count: receipts.graph_write_count(),
            production_write: receipts.production_write,
            graph_write: receipts.graph_write,
            runtime_activation: receipts.runtime_activation,
            prompt_assembly_change: receipts.prompt_assembly_change || receipts.hot_path_write,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_ranked_recall_shadow_eval(
        ranked_recall: &ContextMemoryRankedRecallShadowEvalReport,
    ) -> Self {
        let lexical_bm25_check_pass = ranked_recall
            .hybrid_signals
            .contains(&ContextMemoryRankedRecallShadowHybridSignal::LexicalBm25);
        let recency_check_pass = ranked_recall
            .hybrid_signals
            .contains(&ContextMemoryRankedRecallShadowHybridSignal::Recency);
        let source_authority_check_pass = ranked_recall
            .hybrid_signals
            .contains(&ContextMemoryRankedRecallShadowHybridSignal::SourceAuthority);
        let temporal_validity_check_pass = ranked_recall
            .hybrid_signals
            .contains(&ContextMemoryRankedRecallShadowHybridSignal::TemporalValidity);
        let feedback_check_pass = ranked_recall
            .hybrid_signals
            .contains(&ContextMemoryRankedRecallShadowHybridSignal::Feedback);
        let hybrid_signal_pass_count = [
            lexical_bm25_check_pass,
            recency_check_pass,
            source_authority_check_pass,
            temporal_validity_check_pass,
            feedback_check_pass,
        ]
        .iter()
        .filter(|check| **check)
        .count();
        let has_integrity = ranked_recall.has_ranked_recall_shadow_integrity()
            && hybrid_signal_pass_count == RANKED_RECALL_HYBRID_SIGNAL_REQUIRED_COUNT;

        Self {
            section: ContextPlaneStatusSection::MemoryRankedRecallShadowEval,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: ranked_recall.fixture_count(),
            omitted_count: ranked_recall
                .fixture_count()
                .saturating_sub(ranked_recall.fixture_pass_count()),
            blocker_count: usize::from(!has_integrity),
            ranked_recall_hybrid_signal_required_count: RANKED_RECALL_HYBRID_SIGNAL_REQUIRED_COUNT,
            ranked_recall_hybrid_signal_pass_count: hybrid_signal_pass_count,
            ranked_recall_lexical_bm25_check_pass: lexical_bm25_check_pass,
            ranked_recall_recency_check_pass: recency_check_pass,
            ranked_recall_source_authority_check_pass: source_authority_check_pass,
            ranked_recall_temporal_validity_check_pass: temporal_validity_check_pass,
            ranked_recall_feedback_check_pass: feedback_check_pass,
            ranked_recall_positive_hybrid_signal_required_count:
                RANKED_RECALL_POSITIVE_HYBRID_SIGNAL_REQUIRED_COUNT,
            ranked_recall_positive_hybrid_signal_pass_count: ranked_recall
                .positive_hybrid_signal_pass_count(),
            ranked_recall_hybrid_regression_blocked_count: ranked_recall
                .hybrid_regression_blocked_count(),
            ranked_recall_hybrid_signal_min_basis_points: ranked_recall
                .hybrid_signal_min_basis_points,
            ranked_recall_min_positive_hybrid_score_basis_points: ranked_recall
                .min_positive_hybrid_score_basis_points(),
            ranked_recall_routing_diff_fixture_count: ranked_recall.routing_diff_fixture_count(),
            ranked_recall_routing_diff_shadow_only_count: ranked_recall
                .routing_diff_shadow_only_count(),
            ranked_recall_routing_diff_win_count: ranked_recall.routing_diff_win_count(),
            ranked_recall_routing_diff_loss_count: ranked_recall.routing_diff_loss_count(),
            ranked_recall_routing_diff_regression_blocked_count: ranked_recall
                .routing_diff_regression_blocked_count(),
            ranked_recall_routing_diff_delta_min_basis_points: ranked_recall
                .routing_diff_delta_min_basis_points,
            ranked_recall_min_positive_routing_diff_delta_basis_points: ranked_recall
                .min_positive_routing_diff_delta_basis_points(),
            ranked_recall_routing_diff_latency_delta_max_ms: ranked_recall
                .routing_diff_latency_delta_max_ms,
            ranked_recall_max_positive_routing_diff_latency_delta_ms: ranked_recall
                .max_positive_routing_diff_latency_delta_ms(),
            ranked_recall_routing_diff_token_tradeoff_min_basis_points: ranked_recall
                .routing_diff_token_tradeoff_min_basis_points,
            ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: ranked_recall
                .min_positive_routing_diff_token_tradeoff_basis_points(),
            ranked_recall_real_workload_trace_fixture_count: ranked_recall
                .real_workload_trace_fixture_count(),
            ranked_recall_real_workload_trace_shadow_only_count: ranked_recall
                .real_workload_trace_shadow_only_count(),
            ranked_recall_real_workload_trace_slo_pass_count: ranked_recall
                .real_workload_trace_slo_pass_count(),
            ranked_recall_real_workload_trace_win_count: ranked_recall
                .real_workload_trace_win_count(),
            ranked_recall_real_workload_trace_loss_count: ranked_recall
                .real_workload_trace_loss_count(),
            ranked_recall_real_workload_trace_operator_review_required_count: ranked_recall
                .real_workload_trace_operator_review_required_count(),
            ranked_recall_real_workload_trace_total_leak_count: ranked_recall
                .real_workload_trace_total_leak_count(),
            ranked_recall_real_workload_trace_max_leak_rate_basis_points: ranked_recall
                .real_workload_trace_max_leak_rate_basis_points(),
            ranked_recall_min_positive_real_workload_trace_coverage_basis_points: ranked_recall
                .min_positive_real_workload_trace_coverage_basis_points(),
            ranked_recall_min_positive_real_workload_trace_precision_basis_points: ranked_recall
                .min_positive_real_workload_trace_precision_basis_points(),
            ranked_recall_total_positive_real_workload_trace_token_saved: ranked_recall
                .total_positive_real_workload_trace_token_saved(),
            ranked_recall_max_positive_real_workload_trace_latency_ms: ranked_recall
                .max_positive_real_workload_trace_latency_ms(),
            ranked_recall_real_workload_trace_regression_loss_count: ranked_recall
                .real_workload_trace_regression_loss_count(),
            ranked_recall_canary_precondition_fixture_count: ranked_recall
                .canary_precondition_fixture_count(),
            ranked_recall_canary_precondition_shadow_only_count: ranked_recall
                .canary_precondition_shadow_only_count(),
            ranked_recall_canary_precondition_pass_count: ranked_recall
                .canary_precondition_pass_count(),
            ranked_recall_canary_feature_flag_registered_count: ranked_recall
                .canary_feature_flag_registered_count(),
            ranked_recall_canary_feature_flag_disabled_count: ranked_recall
                .canary_feature_flag_disabled_count(),
            ranked_recall_canary_kill_switch_registered_count: ranked_recall
                .canary_kill_switch_registered_count(),
            ranked_recall_canary_kill_switch_enabled_count: ranked_recall
                .canary_kill_switch_enabled_count(),
            ranked_recall_canary_rollback_rehearsal_covered_count: ranked_recall
                .canary_rollback_rehearsal_covered_count(),
            ranked_recall_canary_activation_denial_covered_count: ranked_recall
                .canary_activation_denial_covered_count(),
            ranked_recall_canary_precondition_operator_review_required_count: ranked_recall
                .canary_precondition_operator_review_required_count(),
            ranked_recall_canary_precondition_route_opened_count: ranked_recall
                .canary_precondition_route_opened_count(),
            ranked_recall_canary_precondition_rollback_write_count: ranked_recall
                .canary_precondition_rollback_write_count(),
            production_write: ranked_recall.production_write || ranked_recall.production_route,
            graph_write: ranked_recall.graph_write,
            runtime_activation: ranked_recall.runtime_activation,
            prompt_assembly_change: ranked_recall.prompt_assembly_change,
            operator_activation_allowed: ranked_recall.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_provider_v2_audit(
        provider_v2_audit: &MemoryProviderV2AuditReport,
    ) -> Self {
        let query_check_pass = provider_v2_audit.descriptor.context_fencing_required
            && provider_v2_audit.descriptor.provenance_required;
        let update_context_check_pass = provider_v2_audit
            .update_context
            .has_payload_light_boundary();
        let propose_write_check_pass = provider_v2_audit
            .write_proposal
            .has_shadow_boundary_integrity();
        let add_check_pass = provider_v2_audit.add.has_no_side_effects();
        let clear_check_pass = provider_v2_audit.clear.has_no_side_effects();
        let close_check_pass = provider_v2_audit.close.has_no_side_effects();
        let lifecycle_pass_count = [
            query_check_pass,
            update_context_check_pass,
            propose_write_check_pass,
            add_check_pass,
            clear_check_pass,
            close_check_pass,
        ]
        .iter()
        .filter(|check| **check)
        .count();
        let has_integrity = provider_v2_audit.has_shadow_boundary_integrity()
            && lifecycle_pass_count == MEMORY_PROVIDER_V2_LIFECYCLE_REQUIRED_COUNT;

        Self {
            section: ContextPlaneStatusSection::MemoryProviderV2Boundary,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: MEMORY_PROVIDER_V2_LIFECYCLE_REQUIRED_COUNT,
            blocker_count: usize::from(!has_integrity),
            memory_provider_v2_lifecycle_required_count:
                MEMORY_PROVIDER_V2_LIFECYCLE_REQUIRED_COUNT,
            memory_provider_v2_lifecycle_pass_count: lifecycle_pass_count,
            memory_provider_v2_query_check_pass: query_check_pass,
            memory_provider_v2_update_context_check_pass: update_context_check_pass,
            memory_provider_v2_propose_write_check_pass: propose_write_check_pass,
            memory_provider_v2_add_check_pass: add_check_pass,
            memory_provider_v2_clear_check_pass: clear_check_pass,
            memory_provider_v2_close_check_pass: close_check_pass,
            memory_provider_v2_candidate_count: provider_v2_audit.write_proposal.candidate_count,
            memory_provider_v2_operator_review_required_count: provider_v2_audit
                .write_proposal
                .operator_review_required_count,
            production_write: provider_v2_audit.update_context.write_performed
                || provider_v2_audit.write_proposal.write_performed
                || provider_v2_audit.add.write_performed
                || provider_v2_audit.clear.write_performed
                || provider_v2_audit.close.write_performed,
            graph_write: provider_v2_audit.write_proposal.graph_write_performed
                || provider_v2_audit.add.graph_write_performed,
            runtime_activation: provider_v2_audit.update_context.runtime_activation
                || provider_v2_audit.write_proposal.runtime_activation
                || provider_v2_audit.add.runtime_activation
                || provider_v2_audit.clear.runtime_activation
                || provider_v2_audit.close.runtime_activation,
            prompt_assembly_change: provider_v2_audit.update_context.prompt_payload_exported
                || provider_v2_audit.update_context.query_payload_exported
                || provider_v2_audit.update_context.ranked_payload_exported
                || provider_v2_audit.write_proposal.prompt_payload_exported
                || provider_v2_audit.write_proposal.query_payload_exported
                || provider_v2_audit.write_proposal.candidate_payload_exported
                || provider_v2_audit.write_proposal.source_payload_exported
                || provider_v2_audit.add.prompt_payload_exported
                || provider_v2_audit.add.candidate_payload_exported
                || provider_v2_audit.clear.prompt_payload_exported
                || provider_v2_audit.close.prompt_payload_exported,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_shadow_canary_readiness(
        trend_snapshot: &ContextMemoryShadowQualityTrendSnapshotReport,
    ) -> Self {
        let has_integrity = trend_snapshot.has_shadow_quality_trend_snapshot_integrity();
        let blocker_count = if has_integrity {
            0
        } else {
            trend_snapshot.regression_window_blocking_count.max(1)
        };

        Self {
            section: ContextPlaneStatusSection::MemoryShadowCanaryReadiness,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: trend_snapshot.window_observation_count,
            omitted_count: trend_snapshot.regression_window_blocking_count,
            blocker_count,
            production_write: trend_snapshot.production_write || trend_snapshot.production_route,
            graph_write: trend_snapshot.graph_write,
            runtime_activation: trend_snapshot.runtime_activation,
            prompt_assembly_change: trend_snapshot.prompt_assembly_change,
            operator_activation_allowed: trend_snapshot.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_shadow_canary_promotion_readiness(
        promotion_readiness: &ContextMemoryShadowCanaryPromotionReadinessReport,
    ) -> Self {
        let has_integrity = promotion_readiness.has_shadow_canary_promotion_readiness_integrity();
        let canary_promotion_checklist_pass_count = if has_integrity {
            CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT
        } else {
            0
        };
        let blocker_count = if has_integrity {
            0
        } else {
            promotion_readiness.promotion_blocker_count.max(1)
        };

        Self {
            section: ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: promotion_readiness.rollback_rehearsal_count
                + promotion_readiness.kill_switch_rehearsal_count
                + promotion_readiness.soak_readback_window_count,
            omitted_count: promotion_readiness.promotion_blocker_count,
            blocker_count,
            canary_promotion_required_stable_window_count: promotion_readiness
                .required_stable_window_count,
            canary_promotion_observed_stable_window_count: promotion_readiness
                .observed_stable_window_count,
            canary_promotion_required_pass_streak: promotion_readiness.required_pass_streak,
            canary_promotion_observed_pass_streak: promotion_readiness.observed_pass_streak,
            canary_promotion_blocker_count: promotion_readiness.promotion_blocker_count,
            canary_promotion_checklist_required_count: CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT,
            canary_promotion_checklist_pass_count,
            canary_promotion_readiness_check_pass: has_integrity,
            canary_promotion_negative_rehearsal_check_pass: has_integrity,
            canary_promotion_audit_digest_check_pass: has_integrity,
            canary_promotion_audit_freshness_check_pass: has_integrity,
            canary_promotion_rollback_rehearsal_count: promotion_readiness.rollback_rehearsal_count,
            canary_promotion_rollback_rehearsal_pass_count: promotion_readiness
                .rollback_rehearsal_pass_count,
            canary_promotion_kill_switch_rehearsal_count: promotion_readiness
                .kill_switch_rehearsal_count,
            canary_promotion_kill_switch_rehearsal_pass_count: promotion_readiness
                .kill_switch_rehearsal_pass_count,
            canary_promotion_soak_readback_window_count: promotion_readiness
                .soak_readback_window_count,
            canary_promotion_soak_readback_pass_count: promotion_readiness.soak_readback_pass_count,
            production_write: promotion_readiness.production_write
                || promotion_readiness.production_route
                || promotion_readiness.history_persistence_write
                || promotion_readiness.canary_promotion_route_opened
                || promotion_readiness.rollback_write,
            graph_write: promotion_readiness.graph_write,
            runtime_activation: promotion_readiness.runtime_activation,
            prompt_assembly_change: promotion_readiness.prompt_assembly_change,
            operator_activation_allowed: promotion_readiness.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn has_status_integrity(&self) -> bool {
        !self.section.is_unknown()
            && !self.status.is_unknown()
            && (self.status == ContextPlaneStatusKind::Blocked) == (self.blocker_count > 0)
            && self.has_recall_quality_blocker_integrity()
            && self.has_ranked_recall_hybrid_integrity()
            && self.has_canary_promotion_checklist_integrity()
            && self.has_memory_namespace_policy_integrity()
            && self.has_memory_write_chain_readiness_integrity()
            && self.has_memory_write_chain_receipt_freshness_integrity()
            && self.has_memory_temporal_graph_shadow_store_integrity()
            && self.has_memory_temporal_graph_shadow_replay_integrity()
            && self.has_memory_temporal_graph_shadow_traversal_diff_integrity()
            && self.has_memory_temporal_graph_shadow_traversal_quality_integrity()
            && self.has_memory_temporal_graph_shadow_retrieval_canary_guard_integrity()
            && self.has_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_integrity()
            && self.has_memory_provider_v2_lifecycle_integrity()
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    fn has_recall_quality_blocker_integrity(&self) -> bool {
        if self.section != ContextPlaneStatusSection::RecallQualityGate {
            return self.recall_quality_blocking_reason_count == 0
                && self.recall_quality_blocking_reasons.is_empty();
        }

        let reasons_are_unique = self
            .recall_quality_blocking_reasons
            .iter()
            .enumerate()
            .all(|(index, reason)| !self.recall_quality_blocking_reasons[..index].contains(reason));

        self.recall_quality_blocking_reason_count == self.recall_quality_blocking_reasons.len()
            && reasons_are_unique
            && (self.status == ContextPlaneStatusKind::Ready)
                == self.recall_quality_blocking_reasons.is_empty()
    }

    fn has_canary_promotion_checklist_integrity(&self) -> bool {
        let counts = [
            self.canary_promotion_required_stable_window_count,
            self.canary_promotion_observed_stable_window_count,
            self.canary_promotion_required_pass_streak,
            self.canary_promotion_observed_pass_streak,
            self.canary_promotion_blocker_count,
            self.canary_promotion_checklist_required_count,
            self.canary_promotion_checklist_pass_count,
            self.canary_promotion_rollback_rehearsal_count,
            self.canary_promotion_rollback_rehearsal_pass_count,
            self.canary_promotion_kill_switch_rehearsal_count,
            self.canary_promotion_kill_switch_rehearsal_pass_count,
            self.canary_promotion_soak_readback_window_count,
            self.canary_promotion_soak_readback_pass_count,
        ];
        let checks = [
            self.canary_promotion_readiness_check_pass,
            self.canary_promotion_negative_rehearsal_check_pass,
            self.canary_promotion_audit_digest_check_pass,
            self.canary_promotion_audit_freshness_check_pass,
        ];

        if self.section != ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness {
            return counts.iter().all(|count| *count == 0) && checks.iter().all(|check| !check);
        }

        let checklist_pass_count = checks.iter().filter(|check| **check).count();
        let no_promotion_blockers = self.canary_promotion_blocker_count == 0;
        let checklist_complete = self.canary_promotion_checklist_pass_count
            == self.canary_promotion_checklist_required_count;
        let stable_window_complete = self.canary_promotion_observed_stable_window_count
            == self.canary_promotion_required_stable_window_count;
        let pass_streak_complete = self.canary_promotion_observed_pass_streak
            == self.canary_promotion_required_pass_streak;
        let rollback_rehearsal_complete = self.canary_promotion_rollback_rehearsal_pass_count
            == self.canary_promotion_rollback_rehearsal_count;
        let kill_switch_rehearsal_complete = self.canary_promotion_kill_switch_rehearsal_pass_count
            == self.canary_promotion_kill_switch_rehearsal_count;
        let soak_readback_complete = self.canary_promotion_soak_readback_pass_count
            == self.canary_promotion_soak_readback_window_count;

        self.canary_promotion_required_stable_window_count > 0
            && self.canary_promotion_observed_stable_window_count
                <= self.canary_promotion_required_stable_window_count
            && self.canary_promotion_required_pass_streak > 0
            && self.canary_promotion_observed_pass_streak
                <= self.canary_promotion_required_pass_streak
            && self.canary_promotion_rollback_rehearsal_count > 0
            && self.canary_promotion_rollback_rehearsal_pass_count
                <= self.canary_promotion_rollback_rehearsal_count
            && self.canary_promotion_kill_switch_rehearsal_count > 0
            && self.canary_promotion_kill_switch_rehearsal_pass_count
                <= self.canary_promotion_kill_switch_rehearsal_count
            && self.canary_promotion_soak_readback_window_count > 0
            && self.canary_promotion_soak_readback_pass_count
                <= self.canary_promotion_soak_readback_window_count
            && self.canary_promotion_checklist_required_count
                == CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT
            && self.canary_promotion_checklist_pass_count == checklist_pass_count
            && self.canary_promotion_checklist_pass_count
                <= self.canary_promotion_checklist_required_count
            && self.canary_promotion_blocker_count == self.blocker_count
            && no_promotion_blockers == checklist_complete
            && (!no_promotion_blockers
                || (stable_window_complete
                    && pass_streak_complete
                    && rollback_rehearsal_complete
                    && kill_switch_rehearsal_complete
                    && soak_readback_complete))
            && (self.status == ContextPlaneStatusKind::Shadow)
                == (no_promotion_blockers && checklist_complete)
    }

    fn has_ranked_recall_hybrid_integrity(&self) -> bool {
        let counts = [
            self.ranked_recall_hybrid_signal_required_count,
            self.ranked_recall_hybrid_signal_pass_count,
            self.ranked_recall_positive_hybrid_signal_required_count,
            self.ranked_recall_positive_hybrid_signal_pass_count,
            self.ranked_recall_hybrid_regression_blocked_count,
            self.ranked_recall_routing_diff_fixture_count,
            self.ranked_recall_routing_diff_shadow_only_count,
            self.ranked_recall_routing_diff_win_count,
            self.ranked_recall_routing_diff_loss_count,
            self.ranked_recall_routing_diff_regression_blocked_count,
            self.ranked_recall_real_workload_trace_fixture_count,
            self.ranked_recall_real_workload_trace_shadow_only_count,
            self.ranked_recall_real_workload_trace_slo_pass_count,
            self.ranked_recall_real_workload_trace_win_count,
            self.ranked_recall_real_workload_trace_loss_count,
            self.ranked_recall_real_workload_trace_operator_review_required_count,
            self.ranked_recall_real_workload_trace_total_leak_count,
            self.ranked_recall_real_workload_trace_regression_loss_count,
            self.ranked_recall_canary_precondition_fixture_count,
            self.ranked_recall_canary_precondition_shadow_only_count,
            self.ranked_recall_canary_precondition_pass_count,
            self.ranked_recall_canary_feature_flag_registered_count,
            self.ranked_recall_canary_feature_flag_disabled_count,
            self.ranked_recall_canary_kill_switch_registered_count,
            self.ranked_recall_canary_kill_switch_enabled_count,
            self.ranked_recall_canary_rollback_rehearsal_covered_count,
            self.ranked_recall_canary_activation_denial_covered_count,
            self.ranked_recall_canary_precondition_operator_review_required_count,
            self.ranked_recall_canary_precondition_route_opened_count,
            self.ranked_recall_canary_precondition_rollback_write_count,
        ];
        let thresholds = [
            self.ranked_recall_hybrid_signal_min_basis_points,
            self.ranked_recall_min_positive_hybrid_score_basis_points,
            self.ranked_recall_real_workload_trace_max_leak_rate_basis_points,
            self.ranked_recall_min_positive_real_workload_trace_coverage_basis_points,
            self.ranked_recall_min_positive_real_workload_trace_precision_basis_points,
            self.ranked_recall_max_positive_real_workload_trace_latency_ms,
        ];
        let workload_thresholds_usize =
            [self.ranked_recall_total_positive_real_workload_trace_token_saved];
        let routing_thresholds_i32 = [
            self.ranked_recall_routing_diff_delta_min_basis_points,
            self.ranked_recall_min_positive_routing_diff_delta_basis_points,
            self.ranked_recall_routing_diff_latency_delta_max_ms,
            self.ranked_recall_max_positive_routing_diff_latency_delta_ms,
        ];
        let routing_thresholds_u32 = [
            self.ranked_recall_routing_diff_token_tradeoff_min_basis_points,
            self.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points,
        ];
        let checks = [
            self.ranked_recall_lexical_bm25_check_pass,
            self.ranked_recall_recency_check_pass,
            self.ranked_recall_source_authority_check_pass,
            self.ranked_recall_temporal_validity_check_pass,
            self.ranked_recall_feedback_check_pass,
        ];

        if self.section != ContextPlaneStatusSection::MemoryRankedRecallShadowEval {
            return counts.iter().all(|count| *count == 0)
                && thresholds.iter().all(|threshold| *threshold == 0)
                && workload_thresholds_usize
                    .iter()
                    .all(|threshold| *threshold == 0)
                && routing_thresholds_i32
                    .iter()
                    .all(|threshold| *threshold == 0)
                && routing_thresholds_u32
                    .iter()
                    .all(|threshold| *threshold == 0)
                && checks.iter().all(|check| !check);
        }

        let hybrid_signal_pass_count = checks.iter().filter(|check| **check).count();
        self.ranked_recall_hybrid_signal_required_count
            == RANKED_RECALL_HYBRID_SIGNAL_REQUIRED_COUNT
            && self.ranked_recall_hybrid_signal_pass_count == hybrid_signal_pass_count
            && self.ranked_recall_hybrid_signal_pass_count
                == self.ranked_recall_hybrid_signal_required_count
            && self.ranked_recall_positive_hybrid_signal_required_count
                == RANKED_RECALL_POSITIVE_HYBRID_SIGNAL_REQUIRED_COUNT
            && self.ranked_recall_positive_hybrid_signal_pass_count
                == self.ranked_recall_positive_hybrid_signal_required_count
            && self.ranked_recall_hybrid_regression_blocked_count
                == RANKED_RECALL_HYBRID_REGRESSION_BLOCKED_REQUIRED_COUNT
            && self.ranked_recall_hybrid_signal_min_basis_points
                == RANKED_RECALL_HYBRID_SIGNAL_MIN_BASIS_POINTS
            && self.ranked_recall_min_positive_hybrid_score_basis_points
                >= RANKED_RECALL_MIN_POSITIVE_HYBRID_SCORE_BASIS_POINTS
            && self.ranked_recall_routing_diff_fixture_count
                == RANKED_RECALL_ROUTING_DIFF_FIXTURE_REQUIRED_COUNT
            && self.ranked_recall_routing_diff_shadow_only_count
                == self.ranked_recall_routing_diff_fixture_count
            && self.ranked_recall_routing_diff_win_count
                == RANKED_RECALL_ROUTING_DIFF_WIN_REQUIRED_COUNT
            && self.ranked_recall_routing_diff_loss_count
                == RANKED_RECALL_ROUTING_DIFF_LOSS_REQUIRED_COUNT
            && self.ranked_recall_routing_diff_regression_blocked_count
                == RANKED_RECALL_ROUTING_DIFF_REGRESSION_BLOCKED_REQUIRED_COUNT
            && self.ranked_recall_routing_diff_delta_min_basis_points
                == RANKED_RECALL_ROUTING_DIFF_DELTA_MIN_BASIS_POINTS
            && self.ranked_recall_min_positive_routing_diff_delta_basis_points
                >= RANKED_RECALL_MIN_POSITIVE_ROUTING_DIFF_DELTA_BASIS_POINTS
            && self.ranked_recall_routing_diff_latency_delta_max_ms
                == RANKED_RECALL_ROUTING_DIFF_LATENCY_DELTA_MAX_MS
            && self.ranked_recall_max_positive_routing_diff_latency_delta_ms
                <= RANKED_RECALL_MAX_POSITIVE_ROUTING_DIFF_LATENCY_DELTA_MS
            && self.ranked_recall_routing_diff_token_tradeoff_min_basis_points
                == RANKED_RECALL_ROUTING_DIFF_TOKEN_TRADEOFF_MIN_BASIS_POINTS
            && self.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points
                >= RANKED_RECALL_MIN_POSITIVE_ROUTING_DIFF_TOKEN_TRADEOFF_BASIS_POINTS
            && self.ranked_recall_real_workload_trace_fixture_count
                == RANKED_RECALL_REAL_WORKLOAD_TRACE_FIXTURE_REQUIRED_COUNT
            && self.ranked_recall_real_workload_trace_shadow_only_count
                == self.ranked_recall_real_workload_trace_fixture_count
            && self.ranked_recall_real_workload_trace_slo_pass_count
                == RANKED_RECALL_REAL_WORKLOAD_TRACE_SLO_PASS_REQUIRED_COUNT
            && self.ranked_recall_real_workload_trace_win_count
                == RANKED_RECALL_REAL_WORKLOAD_TRACE_WIN_REQUIRED_COUNT
            && self.ranked_recall_real_workload_trace_loss_count
                == RANKED_RECALL_REAL_WORKLOAD_TRACE_LOSS_REQUIRED_COUNT
            && self.ranked_recall_real_workload_trace_operator_review_required_count
                == RANKED_RECALL_REAL_WORKLOAD_TRACE_OPERATOR_REVIEW_REQUIRED_COUNT
            && self.ranked_recall_real_workload_trace_total_leak_count == 0
            && self.ranked_recall_real_workload_trace_max_leak_rate_basis_points
                == RANKED_RECALL_REAL_WORKLOAD_TRACE_LEAK_RATE_MAX_BASIS_POINTS
            && self.ranked_recall_min_positive_real_workload_trace_coverage_basis_points
                >= RANKED_RECALL_MIN_POSITIVE_REAL_WORKLOAD_TRACE_COVERAGE_BASIS_POINTS
            && self.ranked_recall_min_positive_real_workload_trace_precision_basis_points
                >= RANKED_RECALL_MIN_POSITIVE_REAL_WORKLOAD_TRACE_PRECISION_BASIS_POINTS
            && self.ranked_recall_total_positive_real_workload_trace_token_saved
                >= RANKED_RECALL_TOTAL_POSITIVE_REAL_WORKLOAD_TRACE_TOKEN_SAVED_MIN
            && self.ranked_recall_max_positive_real_workload_trace_latency_ms
                <= RANKED_RECALL_MAX_POSITIVE_REAL_WORKLOAD_TRACE_LATENCY_MS
            && self.ranked_recall_real_workload_trace_regression_loss_count
                == RANKED_RECALL_REAL_WORKLOAD_TRACE_REGRESSION_LOSS_REQUIRED_COUNT
            && self.ranked_recall_canary_precondition_fixture_count
                == RANKED_RECALL_CANARY_PRECONDITION_REQUIRED_COUNT
            && self.ranked_recall_canary_precondition_shadow_only_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_precondition_pass_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_feature_flag_registered_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_feature_flag_disabled_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_kill_switch_registered_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_kill_switch_enabled_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_rollback_rehearsal_covered_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_activation_denial_covered_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_precondition_operator_review_required_count
                == self.ranked_recall_canary_precondition_fixture_count
            && self.ranked_recall_canary_precondition_route_opened_count == 0
            && self.ranked_recall_canary_precondition_rollback_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_provider_v2_lifecycle_integrity(&self) -> bool {
        let counts = [
            self.memory_provider_v2_lifecycle_required_count,
            self.memory_provider_v2_lifecycle_pass_count,
            self.memory_provider_v2_candidate_count,
            self.memory_provider_v2_operator_review_required_count,
        ];
        let checks = [
            self.memory_provider_v2_query_check_pass,
            self.memory_provider_v2_update_context_check_pass,
            self.memory_provider_v2_propose_write_check_pass,
            self.memory_provider_v2_add_check_pass,
            self.memory_provider_v2_clear_check_pass,
            self.memory_provider_v2_close_check_pass,
        ];

        if self.section != ContextPlaneStatusSection::MemoryProviderV2Boundary {
            return counts.iter().all(|count| *count == 0) && checks.iter().all(|check| !check);
        }

        let lifecycle_pass_count = checks.iter().filter(|check| **check).count();
        self.memory_provider_v2_lifecycle_required_count
            == MEMORY_PROVIDER_V2_LIFECYCLE_REQUIRED_COUNT
            && self.memory_provider_v2_lifecycle_pass_count == lifecycle_pass_count
            && self.memory_provider_v2_lifecycle_pass_count
                <= self.memory_provider_v2_lifecycle_required_count
            && self.memory_provider_v2_operator_review_required_count
                <= self.memory_provider_v2_candidate_count
            && (self.status == ContextPlaneStatusKind::Shadow)
                == (self.memory_provider_v2_lifecycle_pass_count
                    == self.memory_provider_v2_lifecycle_required_count
                    && self.blocker_count == 0)
    }

    fn has_memory_namespace_policy_integrity(&self) -> bool {
        let counts = [
            self.memory_namespace_policy_namespace_count,
            self.memory_namespace_policy_operator_approval_required_count,
            self.memory_namespace_policy_shadow_wal_required_count,
            self.memory_namespace_policy_readback_required_count,
            self.memory_namespace_policy_canary_required_count,
            self.memory_namespace_policy_rollback_supported_count,
            self.memory_namespace_policy_production_write_count,
            self.memory_namespace_policy_graph_write_count,
        ];

        if self.section != ContextPlaneStatusSection::MemoryNamespacePolicy {
            return counts.iter().all(|count| *count == 0);
        }

        self.memory_namespace_policy_namespace_count == MEMORY_NAMESPACE_POLICY_REQUIRED_COUNT
            && self.memory_namespace_policy_operator_approval_required_count
                == self.memory_namespace_policy_namespace_count
            && self.memory_namespace_policy_shadow_wal_required_count
                == self.memory_namespace_policy_namespace_count
            && self.memory_namespace_policy_readback_required_count
                == self.memory_namespace_policy_namespace_count
            && self.memory_namespace_policy_canary_required_count
                == self.memory_namespace_policy_namespace_count
            && self.memory_namespace_policy_rollback_supported_count
                == self.memory_namespace_policy_namespace_count
            && self.memory_namespace_policy_production_write_count == 0
            && self.memory_namespace_policy_graph_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_write_chain_readiness_integrity(&self) -> bool {
        let counts = [
            self.memory_write_chain_namespace_count,
            self.memory_write_chain_stage_required_count,
            self.memory_write_chain_stage_pass_count,
            self.memory_write_chain_propose_write_ready_count,
            self.memory_write_chain_policy_approval_ready_count,
            self.memory_write_chain_operator_approval_ready_count,
            self.memory_write_chain_shadow_wal_ready_count,
            self.memory_write_chain_readback_ready_count,
            self.memory_write_chain_canary_ready_count,
            self.memory_write_chain_rollback_ready_count,
            self.memory_write_chain_production_write_count,
            self.memory_write_chain_graph_write_count,
        ];

        if self.section != ContextPlaneStatusSection::MemoryWriteChainReadiness {
            return counts.iter().all(|count| *count == 0);
        }

        self.memory_write_chain_namespace_count == MEMORY_WRITE_CHAIN_NAMESPACE_REQUIRED_COUNT
            && self.memory_write_chain_stage_required_count
                == MEMORY_WRITE_CHAIN_STAGE_REQUIRED_COUNT
            && self.memory_write_chain_stage_pass_count
                == self.memory_write_chain_stage_required_count
            && self.memory_write_chain_propose_write_ready_count
                == self.memory_write_chain_namespace_count
            && self.memory_write_chain_policy_approval_ready_count
                == self.memory_write_chain_namespace_count
            && self.memory_write_chain_operator_approval_ready_count
                == self.memory_write_chain_namespace_count
            && self.memory_write_chain_shadow_wal_ready_count
                == self.memory_write_chain_namespace_count
            && self.memory_write_chain_readback_ready_count
                == self.memory_write_chain_namespace_count
            && self.memory_write_chain_canary_ready_count == self.memory_write_chain_namespace_count
            && self.memory_write_chain_rollback_ready_count
                == self.memory_write_chain_namespace_count
            && self.memory_write_chain_production_write_count == 0
            && self.memory_write_chain_graph_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_write_chain_receipt_freshness_integrity(&self) -> bool {
        let counts = [
            self.memory_write_chain_receipt_namespace_count,
            self.memory_write_chain_receipt_required_count,
            self.memory_write_chain_receipt_projected_count,
            self.memory_write_chain_receipt_digest_count,
            self.memory_write_chain_receipt_freshness_pass_count,
            self.memory_write_chain_receipt_replay_guard_pass_count,
            self.memory_write_chain_receipt_stale_replay_rejected_count,
            self.memory_write_chain_receipt_recorded_count,
            self.memory_write_chain_receipt_persisted_count,
            self.memory_write_chain_receipt_production_write_count,
            self.memory_write_chain_receipt_graph_write_count,
        ];

        if self.section != ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness {
            return counts.iter().all(|count| *count == 0);
        }

        self.memory_write_chain_receipt_namespace_count
            == MEMORY_WRITE_CHAIN_RECEIPT_NAMESPACE_REQUIRED_COUNT
            && self.memory_write_chain_receipt_required_count
                == MEMORY_WRITE_CHAIN_RECEIPT_REQUIRED_COUNT
            && self.memory_write_chain_receipt_projected_count
                == self.memory_write_chain_receipt_required_count
            && self.memory_write_chain_receipt_digest_count
                == self.memory_write_chain_receipt_namespace_count
            && self.memory_write_chain_receipt_freshness_pass_count
                == self.memory_write_chain_receipt_namespace_count
            && self.memory_write_chain_receipt_replay_guard_pass_count
                == self.memory_write_chain_receipt_namespace_count
            && self.memory_write_chain_receipt_stale_replay_rejected_count
                == self.memory_write_chain_receipt_namespace_count
            && self.memory_write_chain_receipt_recorded_count == 0
            && self.memory_write_chain_receipt_persisted_count == 0
            && self.memory_write_chain_receipt_production_write_count == 0
            && self.memory_write_chain_receipt_graph_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_temporal_graph_shadow_store_integrity(&self) -> bool {
        let counts = [
            self.memory_temporal_graph_shadow_store_node_count,
            self.memory_temporal_graph_shadow_store_edge_count,
            self.memory_temporal_graph_shadow_store_provenance_edge_count,
            self.memory_temporal_graph_shadow_store_validity_window_edge_count,
            self.memory_temporal_graph_shadow_store_supersedes_edge_count,
            self.memory_temporal_graph_shadow_store_invalidated_node_count,
            self.memory_temporal_graph_shadow_store_stage_required_count,
            self.memory_temporal_graph_shadow_store_stage_projected_count,
            self.memory_temporal_graph_shadow_store_digest_count,
            self.memory_temporal_graph_shadow_store_freshness_pass_count,
            self.memory_temporal_graph_shadow_store_replay_guard_pass_count,
            self.memory_temporal_graph_shadow_store_stale_replay_rejected_count,
            self.memory_temporal_graph_shadow_store_operator_approval_required_count,
            self.memory_temporal_graph_shadow_store_operator_approval_recorded_count,
            self.memory_temporal_graph_shadow_store_recorded_receipt_count,
            self.memory_temporal_graph_shadow_store_persisted_receipt_count,
            self.memory_temporal_graph_shadow_store_production_write_count,
            self.memory_temporal_graph_shadow_store_graph_write_count,
        ];

        if self.section != ContextPlaneStatusSection::MemoryTemporalGraphShadowStore {
            return counts.iter().all(|count| *count == 0);
        }

        self.memory_temporal_graph_shadow_store_node_count > 0
            && self.memory_temporal_graph_shadow_store_edge_count
                >= self.memory_temporal_graph_shadow_store_node_count
            && self.memory_temporal_graph_shadow_store_provenance_edge_count
                == self.memory_temporal_graph_shadow_store_node_count
            && self.memory_temporal_graph_shadow_store_validity_window_edge_count
                == self.memory_temporal_graph_shadow_store_node_count
            && self.memory_temporal_graph_shadow_store_supersedes_edge_count
                <= self.memory_temporal_graph_shadow_store_edge_count
            && self.memory_temporal_graph_shadow_store_invalidated_node_count
                <= self.memory_temporal_graph_shadow_store_node_count
            && self.memory_temporal_graph_shadow_store_stage_required_count
                == MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_STAGE_REQUIRED_COUNT
            && self.memory_temporal_graph_shadow_store_stage_projected_count
                == self.memory_temporal_graph_shadow_store_stage_required_count
            && self.memory_temporal_graph_shadow_store_digest_count == 1
            && self.memory_temporal_graph_shadow_store_freshness_pass_count == 1
            && self.memory_temporal_graph_shadow_store_replay_guard_pass_count == 1
            && self.memory_temporal_graph_shadow_store_stale_replay_rejected_count == 1
            && self.memory_temporal_graph_shadow_store_operator_approval_required_count == 1
            && self.memory_temporal_graph_shadow_store_operator_approval_recorded_count == 0
            && self.memory_temporal_graph_shadow_store_recorded_receipt_count == 0
            && self.memory_temporal_graph_shadow_store_persisted_receipt_count == 0
            && self.memory_temporal_graph_shadow_store_production_write_count == 0
            && self.memory_temporal_graph_shadow_store_graph_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_temporal_graph_shadow_replay_integrity(&self) -> bool {
        let counts = [
            self.memory_temporal_graph_shadow_replay_node_count,
            self.memory_temporal_graph_shadow_replay_edge_count,
            self.memory_temporal_graph_shadow_replay_provenance_count,
            self.memory_temporal_graph_shadow_replay_bitemporal_validity_count,
            self.memory_temporal_graph_shadow_replay_fact_invalidation_count,
            self.memory_temporal_graph_shadow_replay_supersede_tombstone_count,
            self.memory_temporal_graph_shadow_replay_stage_required_count,
            self.memory_temporal_graph_shadow_replay_stage_projected_count,
            self.memory_temporal_graph_shadow_replay_digest_count,
            self.memory_temporal_graph_shadow_replay_freshness_pass_count,
            self.memory_temporal_graph_shadow_replay_guard_pass_count,
            self.memory_temporal_graph_shadow_replay_stale_replay_rejected_count,
            self.memory_temporal_graph_shadow_replay_operator_approval_required_count,
            self.memory_temporal_graph_shadow_replay_operator_approval_recorded_count,
            self.memory_temporal_graph_shadow_replay_recorded_receipt_count,
            self.memory_temporal_graph_shadow_replay_persisted_receipt_count,
            self.memory_temporal_graph_shadow_replay_production_write_count,
            self.memory_temporal_graph_shadow_replay_graph_write_count,
        ];

        if self.section != ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay {
            return counts.iter().all(|count| *count == 0);
        }

        self.memory_temporal_graph_shadow_replay_node_count > 0
            && self.memory_temporal_graph_shadow_replay_edge_count
                >= self.memory_temporal_graph_shadow_replay_node_count
            && self.memory_temporal_graph_shadow_replay_provenance_count
                == self.memory_temporal_graph_shadow_replay_node_count
            && self.memory_temporal_graph_shadow_replay_bitemporal_validity_count
                == self.memory_temporal_graph_shadow_replay_node_count
            && self.memory_temporal_graph_shadow_replay_fact_invalidation_count
                <= self.memory_temporal_graph_shadow_replay_node_count
            && self.memory_temporal_graph_shadow_replay_supersede_tombstone_count
                <= self.memory_temporal_graph_shadow_replay_edge_count
                    + self.memory_temporal_graph_shadow_replay_node_count
            && self.memory_temporal_graph_shadow_replay_stage_required_count
                == MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_STAGE_REQUIRED_COUNT
            && self.memory_temporal_graph_shadow_replay_stage_projected_count
                == self.memory_temporal_graph_shadow_replay_stage_required_count
            && self.memory_temporal_graph_shadow_replay_digest_count
                == self.memory_temporal_graph_shadow_replay_stage_required_count
            && self.memory_temporal_graph_shadow_replay_freshness_pass_count
                == self.memory_temporal_graph_shadow_replay_stage_required_count
            && self.memory_temporal_graph_shadow_replay_guard_pass_count
                == self.memory_temporal_graph_shadow_replay_stage_required_count
            && self.memory_temporal_graph_shadow_replay_stale_replay_rejected_count
                == self.memory_temporal_graph_shadow_replay_stage_required_count
            && self.memory_temporal_graph_shadow_replay_operator_approval_required_count == 1
            && self.memory_temporal_graph_shadow_replay_operator_approval_recorded_count == 0
            && self.memory_temporal_graph_shadow_replay_recorded_receipt_count == 0
            && self.memory_temporal_graph_shadow_replay_persisted_receipt_count == 0
            && self.memory_temporal_graph_shadow_replay_production_write_count == 0
            && self.memory_temporal_graph_shadow_replay_graph_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_temporal_graph_shadow_traversal_diff_integrity(&self) -> bool {
        let counts = [
            self.memory_temporal_graph_shadow_traversal_diff_production_selection_count,
            self.memory_temporal_graph_shadow_traversal_diff_lexical_bm25_candidate_count,
            self.memory_temporal_graph_shadow_traversal_diff_semantic_candidate_count,
            self.memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count,
            self.memory_temporal_graph_shadow_traversal_diff_hybrid_candidate_count,
            self.memory_temporal_graph_shadow_traversal_diff_overlap_candidate_count,
            self.memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count,
            self.memory_temporal_graph_shadow_traversal_diff_win_count,
            self.memory_temporal_graph_shadow_traversal_diff_loss_count,
            self.memory_temporal_graph_shadow_traversal_diff_cost_count,
            self.memory_temporal_graph_shadow_traversal_diff_stage_required_count,
            self.memory_temporal_graph_shadow_traversal_diff_stage_projected_count,
            self.memory_temporal_graph_shadow_traversal_diff_digest_count,
            self.memory_temporal_graph_shadow_traversal_diff_freshness_pass_count,
            self.memory_temporal_graph_shadow_traversal_diff_replay_guard_pass_count,
            self.memory_temporal_graph_shadow_traversal_diff_stale_replay_rejected_count,
            self.memory_temporal_graph_shadow_traversal_diff_llm_rerank_count,
            self.memory_temporal_graph_shadow_traversal_diff_graph_persistence_count,
            self.memory_temporal_graph_shadow_traversal_diff_production_route_count,
            self.memory_temporal_graph_shadow_traversal_diff_production_write_count,
            self.memory_temporal_graph_shadow_traversal_diff_graph_write_count,
        ];

        if self.section != ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalDiff {
            return counts.iter().all(|count| *count == 0);
        }

        self.memory_temporal_graph_shadow_traversal_diff_production_selection_count > 0
            && self.memory_temporal_graph_shadow_traversal_diff_lexical_bm25_candidate_count
                == self.memory_temporal_graph_shadow_traversal_diff_production_selection_count
            && self.memory_temporal_graph_shadow_traversal_diff_semantic_candidate_count
                == self.memory_temporal_graph_shadow_traversal_diff_production_selection_count
            && self.memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count
                >= self.memory_temporal_graph_shadow_traversal_diff_production_selection_count
            && self.memory_temporal_graph_shadow_traversal_diff_hybrid_candidate_count
                >= self.memory_temporal_graph_shadow_traversal_diff_production_selection_count
            && self.memory_temporal_graph_shadow_traversal_diff_overlap_candidate_count
                <= self.memory_temporal_graph_shadow_traversal_diff_production_selection_count
            && self.memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count
                == self
                    .memory_temporal_graph_shadow_traversal_diff_graph_traversal_candidate_count
                    .saturating_sub(
                        self.memory_temporal_graph_shadow_traversal_diff_overlap_candidate_count,
                    )
            && self.memory_temporal_graph_shadow_traversal_diff_win_count
                == usize::from(
                    self.memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count
                        > 0,
                )
            && self.memory_temporal_graph_shadow_traversal_diff_loss_count
                <= self.memory_temporal_graph_shadow_traversal_diff_production_selection_count
            && self.memory_temporal_graph_shadow_traversal_diff_cost_count
                == self.memory_temporal_graph_shadow_traversal_diff_graph_expansion_candidate_count
                    + self.memory_temporal_graph_shadow_traversal_diff_loss_count
            && self.memory_temporal_graph_shadow_traversal_diff_stage_required_count
                == MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_STAGE_REQUIRED_COUNT
            && self.memory_temporal_graph_shadow_traversal_diff_stage_projected_count
                == self.memory_temporal_graph_shadow_traversal_diff_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_diff_digest_count
                == self.memory_temporal_graph_shadow_traversal_diff_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_diff_freshness_pass_count
                == self.memory_temporal_graph_shadow_traversal_diff_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_diff_replay_guard_pass_count
                == self.memory_temporal_graph_shadow_traversal_diff_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_diff_stale_replay_rejected_count
                == self.memory_temporal_graph_shadow_traversal_diff_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_diff_llm_rerank_count == 0
            && self.memory_temporal_graph_shadow_traversal_diff_graph_persistence_count == 0
            && self.memory_temporal_graph_shadow_traversal_diff_production_route_count == 0
            && self.memory_temporal_graph_shadow_traversal_diff_production_write_count == 0
            && self.memory_temporal_graph_shadow_traversal_diff_graph_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_temporal_graph_shadow_traversal_quality_integrity(&self) -> bool {
        let counts = [
            self.memory_temporal_graph_shadow_traversal_quality_fixture_count,
            self.memory_temporal_graph_shadow_traversal_quality_slo_required_count,
            self.memory_temporal_graph_shadow_traversal_quality_slo_pass_count,
            self.memory_temporal_graph_shadow_traversal_quality_token_saved_estimate,
            self.memory_temporal_graph_shadow_traversal_quality_operator_review_required_count,
            self.memory_temporal_graph_shadow_traversal_quality_win_count,
            self.memory_temporal_graph_shadow_traversal_quality_loss_count,
            self.memory_temporal_graph_shadow_traversal_quality_cost_count,
            self.memory_temporal_graph_shadow_traversal_quality_stage_required_count,
            self.memory_temporal_graph_shadow_traversal_quality_stage_projected_count,
            self.memory_temporal_graph_shadow_traversal_quality_digest_count,
            self.memory_temporal_graph_shadow_traversal_quality_freshness_pass_count,
            self.memory_temporal_graph_shadow_traversal_quality_replay_guard_pass_count,
            self.memory_temporal_graph_shadow_traversal_quality_stale_replay_rejected_count,
            self.memory_temporal_graph_shadow_traversal_quality_llm_rerank_count,
            self.memory_temporal_graph_shadow_traversal_quality_graph_persistence_count,
            self.memory_temporal_graph_shadow_traversal_quality_production_route_count,
            self.memory_temporal_graph_shadow_traversal_quality_production_write_count,
            self.memory_temporal_graph_shadow_traversal_quality_graph_write_count,
        ];
        let basis_counts = [
            self.memory_temporal_graph_shadow_traversal_quality_coverage_basis_points,
            self.memory_temporal_graph_shadow_traversal_quality_precision_basis_points,
            self.memory_temporal_graph_shadow_traversal_quality_leak_rate_basis_points,
            self.memory_temporal_graph_shadow_traversal_quality_latency_budget_ms,
            self.memory_temporal_graph_shadow_traversal_quality_projected_latency_ms,
        ];

        if self.section != ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality {
            return counts.iter().all(|count| *count == 0)
                && basis_counts.iter().all(|count| *count == 0);
        }

        self.memory_temporal_graph_shadow_traversal_quality_fixture_count
            == MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_STAGE_REQUIRED_COUNT
            && self.memory_temporal_graph_shadow_traversal_quality_slo_required_count
                == MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_STAGE_REQUIRED_COUNT
            && self.memory_temporal_graph_shadow_traversal_quality_slo_pass_count
                == self.memory_temporal_graph_shadow_traversal_quality_slo_required_count
            && self.memory_temporal_graph_shadow_traversal_quality_coverage_basis_points >= 8_000
            && self.memory_temporal_graph_shadow_traversal_quality_precision_basis_points >= 8_000
            && self.memory_temporal_graph_shadow_traversal_quality_leak_rate_basis_points == 0
            && self.memory_temporal_graph_shadow_traversal_quality_latency_budget_ms == 20
            && self.memory_temporal_graph_shadow_traversal_quality_projected_latency_ms
                <= self.memory_temporal_graph_shadow_traversal_quality_latency_budget_ms
            && self.memory_temporal_graph_shadow_traversal_quality_token_saved_estimate > 0
            && self.memory_temporal_graph_shadow_traversal_quality_operator_review_required_count
                == self.memory_temporal_graph_shadow_traversal_quality_fixture_count
            && self.memory_temporal_graph_shadow_traversal_quality_win_count > 0
            && self.memory_temporal_graph_shadow_traversal_quality_loss_count == 0
            && self.memory_temporal_graph_shadow_traversal_quality_cost_count > 0
            && self.memory_temporal_graph_shadow_traversal_quality_stage_required_count
                == MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_STAGE_REQUIRED_COUNT
            && self.memory_temporal_graph_shadow_traversal_quality_stage_projected_count
                == self.memory_temporal_graph_shadow_traversal_quality_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_quality_digest_count
                == self.memory_temporal_graph_shadow_traversal_quality_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_quality_freshness_pass_count
                == self.memory_temporal_graph_shadow_traversal_quality_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_quality_replay_guard_pass_count
                == self.memory_temporal_graph_shadow_traversal_quality_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_quality_stale_replay_rejected_count
                == self.memory_temporal_graph_shadow_traversal_quality_stage_required_count
            && self.memory_temporal_graph_shadow_traversal_quality_llm_rerank_count == 0
            && self.memory_temporal_graph_shadow_traversal_quality_graph_persistence_count == 0
            && self.memory_temporal_graph_shadow_traversal_quality_production_route_count == 0
            && self.memory_temporal_graph_shadow_traversal_quality_production_write_count == 0
            && self.memory_temporal_graph_shadow_traversal_quality_graph_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_temporal_graph_shadow_retrieval_canary_guard_integrity(&self) -> bool {
        let counts = [
            self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_stage_projected_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_quality_slo_pass_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_required_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_recorded_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_enabled_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_registered_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_ready_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_required_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_pass_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_activation_denial_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_digest_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_freshness_pass_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_replay_guard_pass_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_stale_replay_rejected_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_llm_rerank_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_graph_persistence_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_production_route_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_production_write_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_graph_write_count,
            self.memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count,
        ];

        if self.section != ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard
        {
            return counts.iter().all(|count| *count == 0);
        }

        self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count
            == MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_STAGE_REQUIRED_COUNT
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count
                == MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_STAGE_REQUIRED_COUNT
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_stage_projected_count
                == self
                    .memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_quality_slo_pass_count
                == self
                    .memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count
            && self
                .memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_required_count
                == self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_canary_guard_operator_approval_recorded_count
                == 0
            && self
                .memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_registered_count
                == self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_feature_flag_enabled_count
                == 0
            && self
                .memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_registered_count
                == self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_kill_switch_ready_count
                == self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_required_count
                == self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_canary_guard_rollback_rehearsal_pass_count
                == self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_activation_denial_count
                == self.memory_temporal_graph_shadow_retrieval_canary_guard_fixture_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_canary_route_opened_count
                == 0
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_digest_count
                == self
                    .memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_freshness_pass_count
                == self
                    .memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_replay_guard_pass_count
                == self
                    .memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_stale_replay_rejected_count
                == self
                    .memory_temporal_graph_shadow_retrieval_canary_guard_stage_required_count
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_llm_rerank_count == 0
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_graph_persistence_count
                == 0
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_production_route_count == 0
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_production_write_count == 0
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_graph_write_count == 0
            && self.memory_temporal_graph_shadow_retrieval_canary_guard_rollback_write_count == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }

    fn has_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_integrity(&self) -> bool {
        let counts = [
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_projected_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_guard_pass_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_required_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_recorded_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_registered_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_enabled_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_registered_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_readback_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_pass_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_required_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_readback_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_pass_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_denial_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_route_opened_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_digest_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_freshness_pass_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_replay_guard_pass_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stale_replay_rejected_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_llm_rerank_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_persistence_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_route_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_write_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_write_count,
            self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_count,
        ];

        if self.section
            != ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch
        {
            return counts.iter().all(|count| *count == 0);
        }

        self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            == MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_STAGE_REQUIRED_COUNT
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count
                == MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_STAGE_REQUIRED_COUNT
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_projected_count
                == self
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_guard_pass_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_required_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_operator_approval_recorded_count
                == 0
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_registered_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_feature_flag_enabled_count
                == 0
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_registered_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_readback_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_kill_switch_pass_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_required_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_readback_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_rehearsal_pass_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_denial_count
                == self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_fixture_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_canary_route_opened_count
                == 0
            && self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_digest_count
                == self
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_freshness_pass_count
                == self
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_replay_guard_pass_count
                == self
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stale_replay_rejected_count
                == self
                    .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_stage_required_count
            && self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_llm_rerank_count
                == 0
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_persistence_count
                == 0
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_route_count
                == 0
            && self
                .memory_temporal_graph_shadow_retrieval_rollback_kill_switch_production_write_count
                == 0
            && self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_graph_write_count
                == 0
            && self.memory_temporal_graph_shadow_retrieval_rollback_kill_switch_rollback_write_count
                == 0
            && (self.status == ContextPlaneStatusKind::Shadow) == (self.blocker_count == 0)
    }
}

pub(in crate::memory::context_plane) fn context_plane_status_entry_has_side_effect_flag(
    entry: &ContextPlaneStatusEntry,
) -> bool {
    entry.production_write
        || entry.graph_write
        || entry.runtime_activation
        || entry.prompt_assembly_change
        || entry.operator_activation_allowed
}

fn recall_quality_status_blocking_reasons(
    recall_quality_gate: &ContextMemoryRecallQualityGateReport,
) -> Vec<ContextMemoryRecallQualityGateBlockerReason> {
    let mut reasons = Vec::new();
    for fixture in &recall_quality_gate.fixture_matrix {
        for reason in &fixture.blocking_reasons {
            if !reasons.contains(reason) {
                reasons.push(*reason);
            }
        }
    }
    reasons
}
