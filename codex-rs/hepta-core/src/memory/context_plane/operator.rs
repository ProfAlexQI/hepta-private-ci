use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_PLANE_OPERATOR_APPROVAL_PACKET_SCHEMA_VERSION;
use super::super::ContextMemoryRecallQualityGateBlockerReason;
use super::activation::ContextPlaneActivationBlockerMatrix;
use super::activation::ContextPlaneActivationBlockerReason;
use super::activation::ContextPlaneActivationTarget;
use super::activation::activation_blocker_reason_order;
use super::status::ContextPlaneStatusKind;

const CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT: usize = 4;
const MEMORY_NAMESPACE_POLICY_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_NAMESPACE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_STAGE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_RECEIPT_NAMESPACE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_RECEIPT_REQUIRED_COUNT: usize = 18;
const MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_STAGE_REQUIRED_COUNT: usize = 6;
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

/// Approval scope that must be covered before any context-plane promotion.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextPlaneOperatorApprovalScope {
    AdaptiveBudgetAllocationRuntime,
    SourceAwareRuntimeActivation,
    ProductionMemoryWrite,
    GraphWrite,
    PromptAssemblyChange,
    OperatorActivation,
    #[default]
    Unknown,
}

impl ContextPlaneOperatorApprovalScope {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Count of a blocker reason in a payload-light operator approval packet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ContextPlaneOperatorApprovalBlockerReasonCount {
    pub reason: ContextPlaneActivationBlockerReason,
    pub count: usize,
}

impl ContextPlaneOperatorApprovalBlockerReasonCount {
    pub fn has_count_integrity(&self) -> bool {
        !self.reason.is_unknown() && self.reason.is_blocking() && self.count > 0
    }
}

/// Count of one recall-quality blocker reason in an operator approval dry-run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount {
    pub reason: ContextMemoryRecallQualityGateBlockerReason,
    pub count: usize,
}

impl ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount {
    pub fn has_count_integrity(&self) -> bool {
        self.count > 0
    }
}

/// Aggregated readiness thresholds captured for an operator approval dry-run.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ContextPlaneOperatorApprovalThresholdSnapshot {
    pub total_row_count: usize,
    pub threshold_satisfied_count: usize,
    pub blocker_count: usize,
    pub required_ready_count: usize,
    pub required_shadow_count: usize,
}

impl ContextPlaneOperatorApprovalThresholdSnapshot {
    pub fn from_matrix(matrix: &ContextPlaneActivationBlockerMatrix) -> Self {
        Self {
            total_row_count: matrix.rows.len(),
            threshold_satisfied_count: matrix.satisfied_count(),
            blocker_count: matrix.blocker_count,
            required_ready_count: matrix
                .rows
                .iter()
                .filter(|row| row.required_status == ContextPlaneStatusKind::Ready)
                .count(),
            required_shadow_count: matrix
                .rows
                .iter()
                .filter(|row| row.required_status == ContextPlaneStatusKind::Shadow)
                .count(),
        }
    }

    pub fn has_snapshot_integrity(&self) -> bool {
        self.total_row_count == 22
            && self.threshold_satisfied_count + self.blocker_count == self.total_row_count
            && self.required_ready_count + self.required_shadow_count == self.total_row_count
    }
}

/// Payload-light dry-run packet describing what approval would be required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ContextPlaneOperatorApprovalPacket {
    pub schema_version: u32,
    pub dry_run_only: bool,
    pub approval_required: bool,
    pub activation_command_present: bool,
    pub matrix_row_count: usize,
    pub threshold_satisfied_count: usize,
    pub blocker_count: usize,
    pub threshold_snapshot: ContextPlaneOperatorApprovalThresholdSnapshot,
    pub blocker_reason_counts: Vec<ContextPlaneOperatorApprovalBlockerReasonCount>,
    pub recall_quality_blocking_reason_count: usize,
    pub recall_quality_blocking_reason_counts:
        Vec<ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount>,
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
    pub required_approval_scopes: Vec<ContextPlaneOperatorApprovalScope>,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub adaptive_allocator_runtime_activation: bool,
    pub source_aware_runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextPlaneOperatorApprovalPacket {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_PLANE_OPERATOR_APPROVAL_PACKET_SCHEMA_VERSION,
            dry_run_only: true,
            approval_required: true,
            activation_command_present: false,
            matrix_row_count: 0,
            threshold_satisfied_count: 0,
            blocker_count: 0,
            threshold_snapshot: ContextPlaneOperatorApprovalThresholdSnapshot::default(),
            blocker_reason_counts: Vec::new(),
            recall_quality_blocking_reason_count: 0,
            recall_quality_blocking_reason_counts: Vec::new(),
            canary_promotion_required_stable_window_count: 0,
            canary_promotion_observed_stable_window_count: 0,
            canary_promotion_required_pass_streak: 0,
            canary_promotion_observed_pass_streak: 0,
            canary_promotion_blocker_count: 0,
            canary_promotion_checklist_required_count: 0,
            canary_promotion_checklist_pass_count: 0,
            canary_promotion_readiness_check_pass: false,
            canary_promotion_negative_rehearsal_check_pass: false,
            canary_promotion_audit_digest_check_pass: false,
            canary_promotion_audit_freshness_check_pass: false,
            canary_promotion_rollback_rehearsal_count: 0,
            canary_promotion_rollback_rehearsal_pass_count: 0,
            canary_promotion_kill_switch_rehearsal_count: 0,
            canary_promotion_kill_switch_rehearsal_pass_count: 0,
            canary_promotion_soak_readback_window_count: 0,
            canary_promotion_soak_readback_pass_count: 0,
            memory_provider_v2_lifecycle_required_count: 0,
            memory_provider_v2_lifecycle_pass_count: 0,
            memory_provider_v2_query_check_pass: false,
            memory_provider_v2_update_context_check_pass: false,
            memory_provider_v2_propose_write_check_pass: false,
            memory_provider_v2_add_check_pass: false,
            memory_provider_v2_clear_check_pass: false,
            memory_provider_v2_close_check_pass: false,
            memory_provider_v2_candidate_count: 0,
            memory_provider_v2_operator_review_required_count: 0,
            memory_namespace_policy_namespace_count: 0,
            memory_namespace_policy_operator_approval_required_count: 0,
            memory_namespace_policy_shadow_wal_required_count: 0,
            memory_namespace_policy_readback_required_count: 0,
            memory_namespace_policy_canary_required_count: 0,
            memory_namespace_policy_rollback_supported_count: 0,
            memory_namespace_policy_production_write_count: 0,
            memory_namespace_policy_graph_write_count: 0,
            memory_write_chain_namespace_count: 0,
            memory_write_chain_stage_required_count: 0,
            memory_write_chain_stage_pass_count: 0,
            memory_write_chain_propose_write_ready_count: 0,
            memory_write_chain_policy_approval_ready_count: 0,
            memory_write_chain_operator_approval_ready_count: 0,
            memory_write_chain_shadow_wal_ready_count: 0,
            memory_write_chain_readback_ready_count: 0,
            memory_write_chain_canary_ready_count: 0,
            memory_write_chain_rollback_ready_count: 0,
            memory_write_chain_production_write_count: 0,
            memory_write_chain_graph_write_count: 0,
            memory_write_chain_receipt_namespace_count: 0,
            memory_write_chain_receipt_required_count: 0,
            memory_write_chain_receipt_projected_count: 0,
            memory_write_chain_receipt_digest_count: 0,
            memory_write_chain_receipt_freshness_pass_count: 0,
            memory_write_chain_receipt_replay_guard_pass_count: 0,
            memory_write_chain_receipt_stale_replay_rejected_count: 0,
            memory_write_chain_receipt_recorded_count: 0,
            memory_write_chain_receipt_persisted_count: 0,
            memory_write_chain_receipt_production_write_count: 0,
            memory_write_chain_receipt_graph_write_count: 0,
            memory_temporal_graph_shadow_store_node_count: 0,
            memory_temporal_graph_shadow_store_edge_count: 0,
            memory_temporal_graph_shadow_store_provenance_edge_count: 0,
            memory_temporal_graph_shadow_store_validity_window_edge_count: 0,
            memory_temporal_graph_shadow_store_supersedes_edge_count: 0,
            memory_temporal_graph_shadow_store_invalidated_node_count: 0,
            memory_temporal_graph_shadow_store_stage_required_count: 0,
            memory_temporal_graph_shadow_store_stage_projected_count: 0,
            memory_temporal_graph_shadow_store_digest_count: 0,
            memory_temporal_graph_shadow_store_freshness_pass_count: 0,
            memory_temporal_graph_shadow_store_replay_guard_pass_count: 0,
            memory_temporal_graph_shadow_store_stale_replay_rejected_count: 0,
            memory_temporal_graph_shadow_store_operator_approval_required_count: 0,
            memory_temporal_graph_shadow_store_operator_approval_recorded_count: 0,
            memory_temporal_graph_shadow_store_recorded_receipt_count: 0,
            memory_temporal_graph_shadow_store_persisted_receipt_count: 0,
            memory_temporal_graph_shadow_store_production_write_count: 0,
            memory_temporal_graph_shadow_store_graph_write_count: 0,
            ranked_recall_hybrid_signal_required_count: 0,
            ranked_recall_hybrid_signal_pass_count: 0,
            ranked_recall_lexical_bm25_check_pass: false,
            ranked_recall_recency_check_pass: false,
            ranked_recall_source_authority_check_pass: false,
            ranked_recall_temporal_validity_check_pass: false,
            ranked_recall_feedback_check_pass: false,
            ranked_recall_positive_hybrid_signal_required_count: 0,
            ranked_recall_positive_hybrid_signal_pass_count: 0,
            ranked_recall_hybrid_regression_blocked_count: 0,
            ranked_recall_hybrid_signal_min_basis_points: 0,
            ranked_recall_min_positive_hybrid_score_basis_points: 0,
            ranked_recall_routing_diff_fixture_count: 0,
            ranked_recall_routing_diff_shadow_only_count: 0,
            ranked_recall_routing_diff_win_count: 0,
            ranked_recall_routing_diff_loss_count: 0,
            ranked_recall_routing_diff_regression_blocked_count: 0,
            ranked_recall_routing_diff_delta_min_basis_points: 0,
            ranked_recall_min_positive_routing_diff_delta_basis_points: 0,
            ranked_recall_routing_diff_latency_delta_max_ms: 0,
            ranked_recall_max_positive_routing_diff_latency_delta_ms: 0,
            ranked_recall_routing_diff_token_tradeoff_min_basis_points: 0,
            ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: 0,
            ranked_recall_real_workload_trace_fixture_count: 0,
            ranked_recall_real_workload_trace_shadow_only_count: 0,
            ranked_recall_real_workload_trace_slo_pass_count: 0,
            ranked_recall_real_workload_trace_win_count: 0,
            ranked_recall_real_workload_trace_loss_count: 0,
            ranked_recall_real_workload_trace_operator_review_required_count: 0,
            ranked_recall_real_workload_trace_total_leak_count: 0,
            ranked_recall_real_workload_trace_max_leak_rate_basis_points: 0,
            ranked_recall_min_positive_real_workload_trace_coverage_basis_points: 0,
            ranked_recall_min_positive_real_workload_trace_precision_basis_points: 0,
            ranked_recall_total_positive_real_workload_trace_token_saved: 0,
            ranked_recall_max_positive_real_workload_trace_latency_ms: 0,
            ranked_recall_real_workload_trace_regression_loss_count: 0,
            ranked_recall_canary_precondition_fixture_count: 0,
            ranked_recall_canary_precondition_shadow_only_count: 0,
            ranked_recall_canary_precondition_pass_count: 0,
            ranked_recall_canary_feature_flag_registered_count: 0,
            ranked_recall_canary_feature_flag_disabled_count: 0,
            ranked_recall_canary_kill_switch_registered_count: 0,
            ranked_recall_canary_kill_switch_enabled_count: 0,
            ranked_recall_canary_rollback_rehearsal_covered_count: 0,
            ranked_recall_canary_activation_denial_covered_count: 0,
            ranked_recall_canary_precondition_operator_review_required_count: 0,
            ranked_recall_canary_precondition_route_opened_count: 0,
            ranked_recall_canary_precondition_rollback_write_count: 0,
            required_approval_scopes: Vec::new(),
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            adaptive_allocator_runtime_activation: false,
            source_aware_runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextPlaneOperatorApprovalPacket {
    pub fn from_matrix(matrix: &ContextPlaneActivationBlockerMatrix) -> Self {
        let mut blocker_reason_counts = BTreeMap::new();
        let mut recall_quality_blocking_reason_counts = BTreeMap::new();
        for row in matrix
            .rows
            .iter()
            .filter(|row| row.blocker_reason.is_blocking())
        {
            *blocker_reason_counts.entry(row.blocker_reason).or_insert(0) += 1;
            if row.target == ContextPlaneActivationTarget::RecallQualityGate {
                for reason in &row.recall_quality_blocking_reasons {
                    *recall_quality_blocking_reason_counts
                        .entry(*reason)
                        .or_insert(0) += 1;
                }
            }
        }

        let mut blocker_reason_counts = blocker_reason_counts
            .into_iter()
            .map(|(reason, count)| ContextPlaneOperatorApprovalBlockerReasonCount { reason, count })
            .collect::<Vec<_>>();
        blocker_reason_counts.sort_by_key(|entry| activation_blocker_reason_order(entry.reason));

        let mut recall_quality_blocking_reason_counts =
            recall_quality_blocking_reason_counts
                .into_iter()
                .map(|(reason, count)| {
                    ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount { reason, count }
                })
                .collect::<Vec<_>>();
        recall_quality_blocking_reason_counts
            .sort_by_key(|entry| recall_quality_blocker_reason_order(entry.reason));
        let recall_quality_blocking_reason_count = recall_quality_blocking_reason_counts.len();
        let canary_promotion_row = matrix
            .row_for_target(ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness);
        let provider_v2_row =
            matrix.row_for_target(ContextPlaneActivationTarget::MemoryProviderV2Boundary);
        let namespace_policy_row =
            matrix.row_for_target(ContextPlaneActivationTarget::MemoryNamespacePolicy);
        let write_chain_row =
            matrix.row_for_target(ContextPlaneActivationTarget::MemoryWriteChainReadiness);
        let write_chain_receipt_row =
            matrix.row_for_target(ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness);
        let temporal_graph_shadow_store_row =
            matrix.row_for_target(ContextPlaneActivationTarget::MemoryTemporalGraphShadowStore);
        let ranked_recall_row =
            matrix.row_for_target(ContextPlaneActivationTarget::MemoryRankedRecallShadowEval);

        let required_approval_scopes = required_operator_approval_scopes();

        Self {
            dry_run_only: true,
            approval_required: true,
            activation_command_present: false,
            matrix_row_count: matrix.rows.len(),
            threshold_satisfied_count: matrix.satisfied_count(),
            blocker_count: matrix.blocker_count,
            threshold_snapshot: ContextPlaneOperatorApprovalThresholdSnapshot::from_matrix(matrix),
            blocker_reason_counts,
            recall_quality_blocking_reason_count,
            recall_quality_blocking_reason_counts,
            canary_promotion_required_stable_window_count: canary_promotion_row
                .map(|row| row.canary_promotion_required_stable_window_count)
                .unwrap_or_default(),
            canary_promotion_observed_stable_window_count: canary_promotion_row
                .map(|row| row.canary_promotion_observed_stable_window_count)
                .unwrap_or_default(),
            canary_promotion_required_pass_streak: canary_promotion_row
                .map(|row| row.canary_promotion_required_pass_streak)
                .unwrap_or_default(),
            canary_promotion_observed_pass_streak: canary_promotion_row
                .map(|row| row.canary_promotion_observed_pass_streak)
                .unwrap_or_default(),
            canary_promotion_blocker_count: canary_promotion_row
                .map(|row| row.canary_promotion_blocker_count)
                .unwrap_or_default(),
            canary_promotion_checklist_required_count: canary_promotion_row
                .map(|row| row.canary_promotion_checklist_required_count)
                .unwrap_or_default(),
            canary_promotion_checklist_pass_count: canary_promotion_row
                .map(|row| row.canary_promotion_checklist_pass_count)
                .unwrap_or_default(),
            canary_promotion_readiness_check_pass: canary_promotion_row
                .map(|row| row.canary_promotion_readiness_check_pass)
                .unwrap_or_default(),
            canary_promotion_negative_rehearsal_check_pass: canary_promotion_row
                .map(|row| row.canary_promotion_negative_rehearsal_check_pass)
                .unwrap_or_default(),
            canary_promotion_audit_digest_check_pass: canary_promotion_row
                .map(|row| row.canary_promotion_audit_digest_check_pass)
                .unwrap_or_default(),
            canary_promotion_audit_freshness_check_pass: canary_promotion_row
                .map(|row| row.canary_promotion_audit_freshness_check_pass)
                .unwrap_or_default(),
            canary_promotion_rollback_rehearsal_count: canary_promotion_row
                .map(|row| row.canary_promotion_rollback_rehearsal_count)
                .unwrap_or_default(),
            canary_promotion_rollback_rehearsal_pass_count: canary_promotion_row
                .map(|row| row.canary_promotion_rollback_rehearsal_pass_count)
                .unwrap_or_default(),
            canary_promotion_kill_switch_rehearsal_count: canary_promotion_row
                .map(|row| row.canary_promotion_kill_switch_rehearsal_count)
                .unwrap_or_default(),
            canary_promotion_kill_switch_rehearsal_pass_count: canary_promotion_row
                .map(|row| row.canary_promotion_kill_switch_rehearsal_pass_count)
                .unwrap_or_default(),
            canary_promotion_soak_readback_window_count: canary_promotion_row
                .map(|row| row.canary_promotion_soak_readback_window_count)
                .unwrap_or_default(),
            canary_promotion_soak_readback_pass_count: canary_promotion_row
                .map(|row| row.canary_promotion_soak_readback_pass_count)
                .unwrap_or_default(),
            memory_provider_v2_lifecycle_required_count: provider_v2_row
                .map(|row| row.memory_provider_v2_lifecycle_required_count)
                .unwrap_or_default(),
            memory_provider_v2_lifecycle_pass_count: provider_v2_row
                .map(|row| row.memory_provider_v2_lifecycle_pass_count)
                .unwrap_or_default(),
            memory_provider_v2_query_check_pass: provider_v2_row
                .map(|row| row.memory_provider_v2_query_check_pass)
                .unwrap_or_default(),
            memory_provider_v2_update_context_check_pass: provider_v2_row
                .map(|row| row.memory_provider_v2_update_context_check_pass)
                .unwrap_or_default(),
            memory_provider_v2_propose_write_check_pass: provider_v2_row
                .map(|row| row.memory_provider_v2_propose_write_check_pass)
                .unwrap_or_default(),
            memory_provider_v2_add_check_pass: provider_v2_row
                .map(|row| row.memory_provider_v2_add_check_pass)
                .unwrap_or_default(),
            memory_provider_v2_clear_check_pass: provider_v2_row
                .map(|row| row.memory_provider_v2_clear_check_pass)
                .unwrap_or_default(),
            memory_provider_v2_close_check_pass: provider_v2_row
                .map(|row| row.memory_provider_v2_close_check_pass)
                .unwrap_or_default(),
            memory_provider_v2_candidate_count: provider_v2_row
                .map(|row| row.memory_provider_v2_candidate_count)
                .unwrap_or_default(),
            memory_provider_v2_operator_review_required_count: provider_v2_row
                .map(|row| row.memory_provider_v2_operator_review_required_count)
                .unwrap_or_default(),
            memory_namespace_policy_namespace_count: namespace_policy_row
                .map(|row| row.memory_namespace_policy_namespace_count)
                .unwrap_or_default(),
            memory_namespace_policy_operator_approval_required_count: namespace_policy_row
                .map(|row| row.memory_namespace_policy_operator_approval_required_count)
                .unwrap_or_default(),
            memory_namespace_policy_shadow_wal_required_count: namespace_policy_row
                .map(|row| row.memory_namespace_policy_shadow_wal_required_count)
                .unwrap_or_default(),
            memory_namespace_policy_readback_required_count: namespace_policy_row
                .map(|row| row.memory_namespace_policy_readback_required_count)
                .unwrap_or_default(),
            memory_namespace_policy_canary_required_count: namespace_policy_row
                .map(|row| row.memory_namespace_policy_canary_required_count)
                .unwrap_or_default(),
            memory_namespace_policy_rollback_supported_count: namespace_policy_row
                .map(|row| row.memory_namespace_policy_rollback_supported_count)
                .unwrap_or_default(),
            memory_namespace_policy_production_write_count: namespace_policy_row
                .map(|row| row.memory_namespace_policy_production_write_count)
                .unwrap_or_default(),
            memory_namespace_policy_graph_write_count: namespace_policy_row
                .map(|row| row.memory_namespace_policy_graph_write_count)
                .unwrap_or_default(),
            memory_write_chain_namespace_count: write_chain_row
                .map(|row| row.memory_write_chain_namespace_count)
                .unwrap_or_default(),
            memory_write_chain_stage_required_count: write_chain_row
                .map(|row| row.memory_write_chain_stage_required_count)
                .unwrap_or_default(),
            memory_write_chain_stage_pass_count: write_chain_row
                .map(|row| row.memory_write_chain_stage_pass_count)
                .unwrap_or_default(),
            memory_write_chain_propose_write_ready_count: write_chain_row
                .map(|row| row.memory_write_chain_propose_write_ready_count)
                .unwrap_or_default(),
            memory_write_chain_policy_approval_ready_count: write_chain_row
                .map(|row| row.memory_write_chain_policy_approval_ready_count)
                .unwrap_or_default(),
            memory_write_chain_operator_approval_ready_count: write_chain_row
                .map(|row| row.memory_write_chain_operator_approval_ready_count)
                .unwrap_or_default(),
            memory_write_chain_shadow_wal_ready_count: write_chain_row
                .map(|row| row.memory_write_chain_shadow_wal_ready_count)
                .unwrap_or_default(),
            memory_write_chain_readback_ready_count: write_chain_row
                .map(|row| row.memory_write_chain_readback_ready_count)
                .unwrap_or_default(),
            memory_write_chain_canary_ready_count: write_chain_row
                .map(|row| row.memory_write_chain_canary_ready_count)
                .unwrap_or_default(),
            memory_write_chain_rollback_ready_count: write_chain_row
                .map(|row| row.memory_write_chain_rollback_ready_count)
                .unwrap_or_default(),
            memory_write_chain_production_write_count: write_chain_row
                .map(|row| row.memory_write_chain_production_write_count)
                .unwrap_or_default(),
            memory_write_chain_graph_write_count: write_chain_row
                .map(|row| row.memory_write_chain_graph_write_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_namespace_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_namespace_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_required_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_required_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_projected_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_projected_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_digest_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_digest_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_freshness_pass_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_freshness_pass_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_replay_guard_pass_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_replay_guard_pass_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_stale_replay_rejected_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_stale_replay_rejected_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_recorded_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_recorded_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_persisted_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_persisted_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_production_write_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_production_write_count)
                .unwrap_or_default(),
            memory_write_chain_receipt_graph_write_count: write_chain_receipt_row
                .map(|row| row.memory_write_chain_receipt_graph_write_count)
                .unwrap_or_default(),
            memory_temporal_graph_shadow_store_node_count: temporal_graph_shadow_store_row
                .map(|row| row.memory_temporal_graph_shadow_store_node_count)
                .unwrap_or_default(),
            memory_temporal_graph_shadow_store_edge_count: temporal_graph_shadow_store_row
                .map(|row| row.memory_temporal_graph_shadow_store_edge_count)
                .unwrap_or_default(),
            memory_temporal_graph_shadow_store_provenance_edge_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_provenance_edge_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_validity_window_edge_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_validity_window_edge_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_supersedes_edge_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_supersedes_edge_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_invalidated_node_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_invalidated_node_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_stage_required_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_stage_required_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_stage_projected_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_stage_projected_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_digest_count: temporal_graph_shadow_store_row
                .map(|row| row.memory_temporal_graph_shadow_store_digest_count)
                .unwrap_or_default(),
            memory_temporal_graph_shadow_store_freshness_pass_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_freshness_pass_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_replay_guard_pass_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_replay_guard_pass_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_stale_replay_rejected_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_stale_replay_rejected_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_operator_approval_required_count:
                temporal_graph_shadow_store_row
                    .map(|row| {
                        row.memory_temporal_graph_shadow_store_operator_approval_required_count
                    })
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_operator_approval_recorded_count:
                temporal_graph_shadow_store_row
                    .map(|row| {
                        row.memory_temporal_graph_shadow_store_operator_approval_recorded_count
                    })
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_recorded_receipt_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_recorded_receipt_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_persisted_receipt_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_persisted_receipt_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_production_write_count:
                temporal_graph_shadow_store_row
                    .map(|row| row.memory_temporal_graph_shadow_store_production_write_count)
                    .unwrap_or_default(),
            memory_temporal_graph_shadow_store_graph_write_count: temporal_graph_shadow_store_row
                .map(|row| row.memory_temporal_graph_shadow_store_graph_write_count)
                .unwrap_or_default(),
            ranked_recall_hybrid_signal_required_count: ranked_recall_row
                .map(|row| row.ranked_recall_hybrid_signal_required_count)
                .unwrap_or_default(),
            ranked_recall_hybrid_signal_pass_count: ranked_recall_row
                .map(|row| row.ranked_recall_hybrid_signal_pass_count)
                .unwrap_or_default(),
            ranked_recall_lexical_bm25_check_pass: ranked_recall_row
                .map(|row| row.ranked_recall_lexical_bm25_check_pass)
                .unwrap_or_default(),
            ranked_recall_recency_check_pass: ranked_recall_row
                .map(|row| row.ranked_recall_recency_check_pass)
                .unwrap_or_default(),
            ranked_recall_source_authority_check_pass: ranked_recall_row
                .map(|row| row.ranked_recall_source_authority_check_pass)
                .unwrap_or_default(),
            ranked_recall_temporal_validity_check_pass: ranked_recall_row
                .map(|row| row.ranked_recall_temporal_validity_check_pass)
                .unwrap_or_default(),
            ranked_recall_feedback_check_pass: ranked_recall_row
                .map(|row| row.ranked_recall_feedback_check_pass)
                .unwrap_or_default(),
            ranked_recall_positive_hybrid_signal_required_count: ranked_recall_row
                .map(|row| row.ranked_recall_positive_hybrid_signal_required_count)
                .unwrap_or_default(),
            ranked_recall_positive_hybrid_signal_pass_count: ranked_recall_row
                .map(|row| row.ranked_recall_positive_hybrid_signal_pass_count)
                .unwrap_or_default(),
            ranked_recall_hybrid_regression_blocked_count: ranked_recall_row
                .map(|row| row.ranked_recall_hybrid_regression_blocked_count)
                .unwrap_or_default(),
            ranked_recall_hybrid_signal_min_basis_points: ranked_recall_row
                .map(|row| row.ranked_recall_hybrid_signal_min_basis_points)
                .unwrap_or_default(),
            ranked_recall_min_positive_hybrid_score_basis_points: ranked_recall_row
                .map(|row| row.ranked_recall_min_positive_hybrid_score_basis_points)
                .unwrap_or_default(),
            ranked_recall_routing_diff_fixture_count: ranked_recall_row
                .map(|row| row.ranked_recall_routing_diff_fixture_count)
                .unwrap_or_default(),
            ranked_recall_routing_diff_shadow_only_count: ranked_recall_row
                .map(|row| row.ranked_recall_routing_diff_shadow_only_count)
                .unwrap_or_default(),
            ranked_recall_routing_diff_win_count: ranked_recall_row
                .map(|row| row.ranked_recall_routing_diff_win_count)
                .unwrap_or_default(),
            ranked_recall_routing_diff_loss_count: ranked_recall_row
                .map(|row| row.ranked_recall_routing_diff_loss_count)
                .unwrap_or_default(),
            ranked_recall_routing_diff_regression_blocked_count: ranked_recall_row
                .map(|row| row.ranked_recall_routing_diff_regression_blocked_count)
                .unwrap_or_default(),
            ranked_recall_routing_diff_delta_min_basis_points: ranked_recall_row
                .map(|row| row.ranked_recall_routing_diff_delta_min_basis_points)
                .unwrap_or_default(),
            ranked_recall_min_positive_routing_diff_delta_basis_points: ranked_recall_row
                .map(|row| row.ranked_recall_min_positive_routing_diff_delta_basis_points)
                .unwrap_or_default(),
            ranked_recall_routing_diff_latency_delta_max_ms: ranked_recall_row
                .map(|row| row.ranked_recall_routing_diff_latency_delta_max_ms)
                .unwrap_or_default(),
            ranked_recall_max_positive_routing_diff_latency_delta_ms: ranked_recall_row
                .map(|row| row.ranked_recall_max_positive_routing_diff_latency_delta_ms)
                .unwrap_or_default(),
            ranked_recall_routing_diff_token_tradeoff_min_basis_points: ranked_recall_row
                .map(|row| row.ranked_recall_routing_diff_token_tradeoff_min_basis_points)
                .unwrap_or_default(),
            ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: ranked_recall_row
                .map(|row| row.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_fixture_count: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_fixture_count)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_shadow_only_count: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_shadow_only_count)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_slo_pass_count: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_slo_pass_count)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_win_count: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_win_count)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_loss_count: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_loss_count)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_operator_review_required_count: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_operator_review_required_count)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_total_leak_count: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_total_leak_count)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_max_leak_rate_basis_points: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_max_leak_rate_basis_points)
                .unwrap_or_default(),
            ranked_recall_min_positive_real_workload_trace_coverage_basis_points: ranked_recall_row
                .map(|row| row.ranked_recall_min_positive_real_workload_trace_coverage_basis_points)
                .unwrap_or_default(),
            ranked_recall_min_positive_real_workload_trace_precision_basis_points:
                ranked_recall_row
                    .map(|row| {
                        row.ranked_recall_min_positive_real_workload_trace_precision_basis_points
                    })
                    .unwrap_or_default(),
            ranked_recall_total_positive_real_workload_trace_token_saved: ranked_recall_row
                .map(|row| row.ranked_recall_total_positive_real_workload_trace_token_saved)
                .unwrap_or_default(),
            ranked_recall_max_positive_real_workload_trace_latency_ms: ranked_recall_row
                .map(|row| row.ranked_recall_max_positive_real_workload_trace_latency_ms)
                .unwrap_or_default(),
            ranked_recall_real_workload_trace_regression_loss_count: ranked_recall_row
                .map(|row| row.ranked_recall_real_workload_trace_regression_loss_count)
                .unwrap_or_default(),
            ranked_recall_canary_precondition_fixture_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_precondition_fixture_count)
                .unwrap_or_default(),
            ranked_recall_canary_precondition_shadow_only_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_precondition_shadow_only_count)
                .unwrap_or_default(),
            ranked_recall_canary_precondition_pass_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_precondition_pass_count)
                .unwrap_or_default(),
            ranked_recall_canary_feature_flag_registered_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_feature_flag_registered_count)
                .unwrap_or_default(),
            ranked_recall_canary_feature_flag_disabled_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_feature_flag_disabled_count)
                .unwrap_or_default(),
            ranked_recall_canary_kill_switch_registered_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_kill_switch_registered_count)
                .unwrap_or_default(),
            ranked_recall_canary_kill_switch_enabled_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_kill_switch_enabled_count)
                .unwrap_or_default(),
            ranked_recall_canary_rollback_rehearsal_covered_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_rollback_rehearsal_covered_count)
                .unwrap_or_default(),
            ranked_recall_canary_activation_denial_covered_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_activation_denial_covered_count)
                .unwrap_or_default(),
            ranked_recall_canary_precondition_operator_review_required_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_precondition_operator_review_required_count)
                .unwrap_or_default(),
            ranked_recall_canary_precondition_route_opened_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_precondition_route_opened_count)
                .unwrap_or_default(),
            ranked_recall_canary_precondition_rollback_write_count: ranked_recall_row
                .map(|row| row.ranked_recall_canary_precondition_rollback_write_count)
                .unwrap_or_default(),
            required_approval_scopes,
            production_write: matrix.production_write,
            graph_write: matrix.graph_write,
            runtime_activation: matrix.runtime_activation,
            adaptive_allocator_runtime_activation: matrix.adaptive_allocator_runtime_activation,
            source_aware_runtime_activation: matrix.source_aware_runtime_activation,
            prompt_assembly_change: matrix.prompt_assembly_change,
            operator_activation_allowed: matrix.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn has_packet_integrity(&self) -> bool {
        self.schema_version == CONTEXT_PLANE_OPERATOR_APPROVAL_PACKET_SCHEMA_VERSION
            && self.dry_run_only
            && self.approval_required
            && !self.activation_command_present
            && self.matrix_row_count == 22
            && self.threshold_satisfied_count + self.blocker_count == self.matrix_row_count
            && self.threshold_snapshot.has_snapshot_integrity()
            && self.threshold_snapshot.total_row_count == self.matrix_row_count
            && self.threshold_snapshot.threshold_satisfied_count == self.threshold_satisfied_count
            && self.threshold_snapshot.blocker_count == self.blocker_count
            && self.blocker_count == self.blocker_reason_count_total()
            && self
                .blocker_reason_counts
                .iter()
                .all(ContextPlaneOperatorApprovalBlockerReasonCount::has_count_integrity)
            && self.has_recall_quality_blocking_reason_count_integrity()
            && self.has_canary_promotion_checklist_integrity()
            && self.has_memory_namespace_policy_integrity()
            && self.has_memory_write_chain_readiness_integrity()
            && self.has_memory_write_chain_receipt_freshness_integrity()
            && self.has_memory_temporal_graph_shadow_store_integrity()
            && self.has_memory_provider_v2_lifecycle_integrity()
            && self.has_ranked_recall_hybrid_integrity()
            && self.has_required_approval_scopes()
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.adaptive_allocator_runtime_activation
            && !self.source_aware_runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    pub fn blocker_reason_count_total(&self) -> usize {
        self.blocker_reason_counts
            .iter()
            .map(|entry| entry.count)
            .sum()
    }

    pub fn blocker_reason_count(
        &self,
        reason: ContextPlaneActivationBlockerReason,
    ) -> Option<usize> {
        self.blocker_reason_counts
            .iter()
            .find(|entry| entry.reason == reason)
            .map(|entry| entry.count)
    }

    pub fn recall_quality_blocking_reason_count_total(&self) -> usize {
        self.recall_quality_blocking_reason_counts
            .iter()
            .map(|entry| entry.count)
            .sum()
    }

    pub fn recall_quality_blocking_reason_count_for(
        &self,
        reason: ContextMemoryRecallQualityGateBlockerReason,
    ) -> Option<usize> {
        self.recall_quality_blocking_reason_counts
            .iter()
            .find(|entry| entry.reason == reason)
            .map(|entry| entry.count)
    }

    pub fn required_scope_count(&self) -> usize {
        self.required_approval_scopes.len()
    }

    fn has_required_approval_scopes(&self) -> bool {
        let expected = required_operator_approval_scopes();
        self.required_approval_scopes == expected
            && self
                .required_approval_scopes
                .iter()
                .all(|scope| !scope.is_unknown())
    }

    fn has_recall_quality_blocking_reason_count_integrity(&self) -> bool {
        let reasons_are_unique = self
            .recall_quality_blocking_reason_counts
            .iter()
            .enumerate()
            .all(|(index, entry)| {
                !self.recall_quality_blocking_reason_counts[..index]
                    .iter()
                    .any(|prior| prior.reason == entry.reason)
            });

        self.recall_quality_blocking_reason_count
            == self.recall_quality_blocking_reason_counts.len()
            && reasons_are_unique
            && self.recall_quality_blocking_reason_counts.iter().all(
                ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount::has_count_integrity,
            )
    }

    fn has_canary_promotion_checklist_integrity(&self) -> bool {
        let checklist_pass_count = [
            self.canary_promotion_readiness_check_pass,
            self.canary_promotion_negative_rehearsal_check_pass,
            self.canary_promotion_audit_digest_check_pass,
            self.canary_promotion_audit_freshness_check_pass,
        ]
        .iter()
        .filter(|check| **check)
        .count();
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
            && no_promotion_blockers == checklist_complete
            && (!no_promotion_blockers
                || (stable_window_complete
                    && pass_streak_complete
                    && rollback_rehearsal_complete
                    && kill_switch_rehearsal_complete
                    && soak_readback_complete))
    }

    fn has_memory_provider_v2_lifecycle_integrity(&self) -> bool {
        let lifecycle_pass_count = [
            self.memory_provider_v2_query_check_pass,
            self.memory_provider_v2_update_context_check_pass,
            self.memory_provider_v2_propose_write_check_pass,
            self.memory_provider_v2_add_check_pass,
            self.memory_provider_v2_clear_check_pass,
            self.memory_provider_v2_close_check_pass,
        ]
        .iter()
        .filter(|check| **check)
        .count();

        self.memory_provider_v2_lifecycle_required_count
            == MEMORY_PROVIDER_V2_LIFECYCLE_REQUIRED_COUNT
            && self.memory_provider_v2_lifecycle_pass_count == lifecycle_pass_count
            && self.memory_provider_v2_lifecycle_pass_count
                <= self.memory_provider_v2_lifecycle_required_count
            && self.memory_provider_v2_operator_review_required_count
                <= self.memory_provider_v2_candidate_count
    }

    fn has_memory_namespace_policy_integrity(&self) -> bool {
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
    }

    fn has_memory_write_chain_readiness_integrity(&self) -> bool {
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
    }

    fn has_memory_write_chain_receipt_freshness_integrity(&self) -> bool {
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
    }

    fn has_memory_temporal_graph_shadow_store_integrity(&self) -> bool {
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
    }

    fn has_ranked_recall_hybrid_integrity(&self) -> bool {
        let hybrid_signal_pass_count = [
            self.ranked_recall_lexical_bm25_check_pass,
            self.ranked_recall_recency_check_pass,
            self.ranked_recall_source_authority_check_pass,
            self.ranked_recall_temporal_validity_check_pass,
            self.ranked_recall_feedback_check_pass,
        ]
        .iter()
        .filter(|check| **check)
        .count();

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
    }
}

pub(in crate::memory) fn required_operator_approval_scopes()
-> Vec<ContextPlaneOperatorApprovalScope> {
    vec![
        ContextPlaneOperatorApprovalScope::AdaptiveBudgetAllocationRuntime,
        ContextPlaneOperatorApprovalScope::SourceAwareRuntimeActivation,
        ContextPlaneOperatorApprovalScope::ProductionMemoryWrite,
        ContextPlaneOperatorApprovalScope::GraphWrite,
        ContextPlaneOperatorApprovalScope::PromptAssemblyChange,
        ContextPlaneOperatorApprovalScope::OperatorActivation,
    ]
}

fn recall_quality_blocker_reason_order(
    reason: ContextMemoryRecallQualityGateBlockerReason,
) -> usize {
    match reason {
        ContextMemoryRecallQualityGateBlockerReason::MissingCriticalFactRegression => 0,
        ContextMemoryRecallQualityGateBlockerReason::RecallCoverageRegression => 1,
        ContextMemoryRecallQualityGateBlockerReason::PrecisionRegression => 2,
        ContextMemoryRecallQualityGateBlockerReason::SafetyLeak => 3,
        ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression => 4,
        ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled => 5,
    }
}
