use serde::Deserialize;
use serde::Serialize;

use super::super::super::ContextMemoryRecallQualityGateBlockerReason;
use super::super::status::ContextPlaneStatusEntry;
use super::super::status::ContextPlaneStatusKind;
use super::super::status::context_plane_status_entry_has_side_effect_flag;
use super::target::ContextPlaneActivationBlockerReason;
use super::target::ContextPlaneActivationTarget;

const CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT: usize = 4;
const MEMORY_NAMESPACE_POLICY_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_NAMESPACE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_STAGE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_RECEIPT_NAMESPACE_REQUIRED_COUNT: usize = 6;
const MEMORY_WRITE_CHAIN_RECEIPT_REQUIRED_COUNT: usize = 18;
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

/// One activation-readiness threshold row.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextPlaneActivationBlockerRow {
    pub target: ContextPlaneActivationTarget,
    pub observed_status: ContextPlaneStatusKind,
    pub required_status: ContextPlaneStatusKind,
    pub threshold_satisfied: bool,
    pub blocker_reason: ContextPlaneActivationBlockerReason,
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

impl ContextPlaneActivationBlockerRow {
    fn satisfied(
        target: ContextPlaneActivationTarget,
        observed_status: ContextPlaneStatusKind,
        required_status: ContextPlaneStatusKind,
    ) -> Self {
        Self {
            target,
            observed_status,
            required_status,
            threshold_satisfied: true,
            blocker_reason: ContextPlaneActivationBlockerReason::None,
            ..Self::default()
        }
    }

    pub(super) fn blocked(
        target: ContextPlaneActivationTarget,
        observed_status: ContextPlaneStatusKind,
        required_status: ContextPlaneStatusKind,
        blocker_reason: ContextPlaneActivationBlockerReason,
    ) -> Self {
        Self {
            target,
            observed_status,
            required_status,
            threshold_satisfied: false,
            blocker_reason,
            ..Self::default()
        }
    }

    fn from_required_status(
        target: ContextPlaneActivationTarget,
        observed_status: ContextPlaneStatusKind,
        required_status: ContextPlaneStatusKind,
    ) -> Self {
        if observed_status == required_status {
            return Self::satisfied(target, observed_status, required_status);
        }

        let reason = match (target, observed_status) {
            (_, ContextPlaneStatusKind::Unknown) => {
                ContextPlaneActivationBlockerReason::StatusMissing
            }
            (_, ContextPlaneStatusKind::Blocked) => {
                ContextPlaneActivationBlockerReason::SectionBlocked
            }
            (
                ContextPlaneActivationTarget::AdaptiveBudgetAllocation,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryRankedRecallShadowEval,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryProviderBoundary,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryProviderV2Boundary,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryNamespacePolicy,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryWriteChainReadiness,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryWriteChainReceiptFreshnessShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryShadowCanaryReadiness,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness,
                ContextPlaneStatusKind::Shadow,
            ) => {
                ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly
            }
            (
                ContextPlaneActivationTarget::SourceAwareFrontDoor,
                ContextPlaneStatusKind::Disabled,
            ) => ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled,
            (_, ContextPlaneStatusKind::Shadow) => {
                ContextPlaneActivationBlockerReason::SectionShadowOnly
            }
            (_, ContextPlaneStatusKind::Disabled) => {
                ContextPlaneActivationBlockerReason::SectionDisabled
            }
            _ => ContextPlaneActivationBlockerReason::UnexpectedStatus,
        };

        Self::blocked(target, observed_status, required_status, reason)
    }

    pub(super) fn from_status_entry(
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
        required_status: ContextPlaneStatusKind,
        report_side_effect_flag_enabled: bool,
    ) -> Self {
        let observed_status = entry
            .map(|entry| entry.status)
            .unwrap_or(ContextPlaneStatusKind::Unknown);
        let entry_side_effect_flag_enabled = entry
            .map(context_plane_status_entry_has_side_effect_flag)
            .unwrap_or(false);
        if report_side_effect_flag_enabled || entry_side_effect_flag_enabled {
            return Self::blocked(
                target,
                observed_status,
                required_status,
                ContextPlaneActivationBlockerReason::SideEffectFlagEnabled,
            )
            .with_recall_quality_rollup(target, entry)
            .with_ranked_recall_rollup(target, entry)
            .with_canary_promotion_rollup(target, entry)
            .with_memory_namespace_policy_rollup(target, entry)
            .with_memory_write_chain_readiness_rollup(target, entry)
            .with_memory_write_chain_receipt_freshness_rollup(target, entry)
            .with_memory_provider_v2_rollup(target, entry);
        }

        Self::from_required_status(target, observed_status, required_status)
            .with_recall_quality_rollup(target, entry)
            .with_ranked_recall_rollup(target, entry)
            .with_canary_promotion_rollup(target, entry)
            .with_memory_namespace_policy_rollup(target, entry)
            .with_memory_write_chain_readiness_rollup(target, entry)
            .with_memory_write_chain_receipt_freshness_rollup(target, entry)
            .with_memory_provider_v2_rollup(target, entry)
    }

    fn with_ranked_recall_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::MemoryRankedRecallShadowEval
            && let Some(entry) = entry
        {
            self.ranked_recall_hybrid_signal_required_count =
                entry.ranked_recall_hybrid_signal_required_count;
            self.ranked_recall_hybrid_signal_pass_count =
                entry.ranked_recall_hybrid_signal_pass_count;
            self.ranked_recall_lexical_bm25_check_pass =
                entry.ranked_recall_lexical_bm25_check_pass;
            self.ranked_recall_recency_check_pass = entry.ranked_recall_recency_check_pass;
            self.ranked_recall_source_authority_check_pass =
                entry.ranked_recall_source_authority_check_pass;
            self.ranked_recall_temporal_validity_check_pass =
                entry.ranked_recall_temporal_validity_check_pass;
            self.ranked_recall_feedback_check_pass = entry.ranked_recall_feedback_check_pass;
            self.ranked_recall_positive_hybrid_signal_required_count =
                entry.ranked_recall_positive_hybrid_signal_required_count;
            self.ranked_recall_positive_hybrid_signal_pass_count =
                entry.ranked_recall_positive_hybrid_signal_pass_count;
            self.ranked_recall_hybrid_regression_blocked_count =
                entry.ranked_recall_hybrid_regression_blocked_count;
            self.ranked_recall_hybrid_signal_min_basis_points =
                entry.ranked_recall_hybrid_signal_min_basis_points;
            self.ranked_recall_min_positive_hybrid_score_basis_points =
                entry.ranked_recall_min_positive_hybrid_score_basis_points;
            self.ranked_recall_routing_diff_fixture_count =
                entry.ranked_recall_routing_diff_fixture_count;
            self.ranked_recall_routing_diff_shadow_only_count =
                entry.ranked_recall_routing_diff_shadow_only_count;
            self.ranked_recall_routing_diff_win_count = entry.ranked_recall_routing_diff_win_count;
            self.ranked_recall_routing_diff_loss_count =
                entry.ranked_recall_routing_diff_loss_count;
            self.ranked_recall_routing_diff_regression_blocked_count =
                entry.ranked_recall_routing_diff_regression_blocked_count;
            self.ranked_recall_routing_diff_delta_min_basis_points =
                entry.ranked_recall_routing_diff_delta_min_basis_points;
            self.ranked_recall_min_positive_routing_diff_delta_basis_points =
                entry.ranked_recall_min_positive_routing_diff_delta_basis_points;
            self.ranked_recall_routing_diff_latency_delta_max_ms =
                entry.ranked_recall_routing_diff_latency_delta_max_ms;
            self.ranked_recall_max_positive_routing_diff_latency_delta_ms =
                entry.ranked_recall_max_positive_routing_diff_latency_delta_ms;
            self.ranked_recall_routing_diff_token_tradeoff_min_basis_points =
                entry.ranked_recall_routing_diff_token_tradeoff_min_basis_points;
            self.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points =
                entry.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points;
            self.ranked_recall_real_workload_trace_fixture_count =
                entry.ranked_recall_real_workload_trace_fixture_count;
            self.ranked_recall_real_workload_trace_shadow_only_count =
                entry.ranked_recall_real_workload_trace_shadow_only_count;
            self.ranked_recall_real_workload_trace_slo_pass_count =
                entry.ranked_recall_real_workload_trace_slo_pass_count;
            self.ranked_recall_real_workload_trace_win_count =
                entry.ranked_recall_real_workload_trace_win_count;
            self.ranked_recall_real_workload_trace_loss_count =
                entry.ranked_recall_real_workload_trace_loss_count;
            self.ranked_recall_real_workload_trace_operator_review_required_count =
                entry.ranked_recall_real_workload_trace_operator_review_required_count;
            self.ranked_recall_real_workload_trace_total_leak_count =
                entry.ranked_recall_real_workload_trace_total_leak_count;
            self.ranked_recall_real_workload_trace_max_leak_rate_basis_points =
                entry.ranked_recall_real_workload_trace_max_leak_rate_basis_points;
            self.ranked_recall_min_positive_real_workload_trace_coverage_basis_points =
                entry.ranked_recall_min_positive_real_workload_trace_coverage_basis_points;
            self.ranked_recall_min_positive_real_workload_trace_precision_basis_points =
                entry.ranked_recall_min_positive_real_workload_trace_precision_basis_points;
            self.ranked_recall_total_positive_real_workload_trace_token_saved =
                entry.ranked_recall_total_positive_real_workload_trace_token_saved;
            self.ranked_recall_max_positive_real_workload_trace_latency_ms =
                entry.ranked_recall_max_positive_real_workload_trace_latency_ms;
            self.ranked_recall_real_workload_trace_regression_loss_count =
                entry.ranked_recall_real_workload_trace_regression_loss_count;
            self.ranked_recall_canary_precondition_fixture_count =
                entry.ranked_recall_canary_precondition_fixture_count;
            self.ranked_recall_canary_precondition_shadow_only_count =
                entry.ranked_recall_canary_precondition_shadow_only_count;
            self.ranked_recall_canary_precondition_pass_count =
                entry.ranked_recall_canary_precondition_pass_count;
            self.ranked_recall_canary_feature_flag_registered_count =
                entry.ranked_recall_canary_feature_flag_registered_count;
            self.ranked_recall_canary_feature_flag_disabled_count =
                entry.ranked_recall_canary_feature_flag_disabled_count;
            self.ranked_recall_canary_kill_switch_registered_count =
                entry.ranked_recall_canary_kill_switch_registered_count;
            self.ranked_recall_canary_kill_switch_enabled_count =
                entry.ranked_recall_canary_kill_switch_enabled_count;
            self.ranked_recall_canary_rollback_rehearsal_covered_count =
                entry.ranked_recall_canary_rollback_rehearsal_covered_count;
            self.ranked_recall_canary_activation_denial_covered_count =
                entry.ranked_recall_canary_activation_denial_covered_count;
            self.ranked_recall_canary_precondition_operator_review_required_count =
                entry.ranked_recall_canary_precondition_operator_review_required_count;
            self.ranked_recall_canary_precondition_route_opened_count =
                entry.ranked_recall_canary_precondition_route_opened_count;
            self.ranked_recall_canary_precondition_rollback_write_count =
                entry.ranked_recall_canary_precondition_rollback_write_count;
        }
        self
    }

    fn with_recall_quality_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::RecallQualityGate
            && let Some(entry) = entry
        {
            self.recall_quality_blocking_reason_count = entry.recall_quality_blocking_reason_count;
            self.recall_quality_blocking_reasons = entry.recall_quality_blocking_reasons.clone();
        }
        self
    }

    fn with_canary_promotion_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness
            && let Some(entry) = entry
        {
            self.canary_promotion_required_stable_window_count =
                entry.canary_promotion_required_stable_window_count;
            self.canary_promotion_observed_stable_window_count =
                entry.canary_promotion_observed_stable_window_count;
            self.canary_promotion_required_pass_streak =
                entry.canary_promotion_required_pass_streak;
            self.canary_promotion_observed_pass_streak =
                entry.canary_promotion_observed_pass_streak;
            self.canary_promotion_blocker_count = entry.canary_promotion_blocker_count;
            self.canary_promotion_checklist_required_count =
                entry.canary_promotion_checklist_required_count;
            self.canary_promotion_checklist_pass_count =
                entry.canary_promotion_checklist_pass_count;
            self.canary_promotion_readiness_check_pass =
                entry.canary_promotion_readiness_check_pass;
            self.canary_promotion_negative_rehearsal_check_pass =
                entry.canary_promotion_negative_rehearsal_check_pass;
            self.canary_promotion_audit_digest_check_pass =
                entry.canary_promotion_audit_digest_check_pass;
            self.canary_promotion_audit_freshness_check_pass =
                entry.canary_promotion_audit_freshness_check_pass;
            self.canary_promotion_rollback_rehearsal_count =
                entry.canary_promotion_rollback_rehearsal_count;
            self.canary_promotion_rollback_rehearsal_pass_count =
                entry.canary_promotion_rollback_rehearsal_pass_count;
            self.canary_promotion_kill_switch_rehearsal_count =
                entry.canary_promotion_kill_switch_rehearsal_count;
            self.canary_promotion_kill_switch_rehearsal_pass_count =
                entry.canary_promotion_kill_switch_rehearsal_pass_count;
            self.canary_promotion_soak_readback_window_count =
                entry.canary_promotion_soak_readback_window_count;
            self.canary_promotion_soak_readback_pass_count =
                entry.canary_promotion_soak_readback_pass_count;
        }
        self
    }

    fn with_memory_provider_v2_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::MemoryProviderV2Boundary
            && let Some(entry) = entry
        {
            self.memory_provider_v2_lifecycle_required_count =
                entry.memory_provider_v2_lifecycle_required_count;
            self.memory_provider_v2_lifecycle_pass_count =
                entry.memory_provider_v2_lifecycle_pass_count;
            self.memory_provider_v2_query_check_pass = entry.memory_provider_v2_query_check_pass;
            self.memory_provider_v2_update_context_check_pass =
                entry.memory_provider_v2_update_context_check_pass;
            self.memory_provider_v2_propose_write_check_pass =
                entry.memory_provider_v2_propose_write_check_pass;
            self.memory_provider_v2_add_check_pass = entry.memory_provider_v2_add_check_pass;
            self.memory_provider_v2_clear_check_pass = entry.memory_provider_v2_clear_check_pass;
            self.memory_provider_v2_close_check_pass = entry.memory_provider_v2_close_check_pass;
            self.memory_provider_v2_candidate_count = entry.memory_provider_v2_candidate_count;
            self.memory_provider_v2_operator_review_required_count =
                entry.memory_provider_v2_operator_review_required_count;
        }
        self
    }

    fn with_memory_namespace_policy_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::MemoryNamespacePolicy
            && let Some(entry) = entry
        {
            self.memory_namespace_policy_namespace_count =
                entry.memory_namespace_policy_namespace_count;
            self.memory_namespace_policy_operator_approval_required_count =
                entry.memory_namespace_policy_operator_approval_required_count;
            self.memory_namespace_policy_shadow_wal_required_count =
                entry.memory_namespace_policy_shadow_wal_required_count;
            self.memory_namespace_policy_readback_required_count =
                entry.memory_namespace_policy_readback_required_count;
            self.memory_namespace_policy_canary_required_count =
                entry.memory_namespace_policy_canary_required_count;
            self.memory_namespace_policy_rollback_supported_count =
                entry.memory_namespace_policy_rollback_supported_count;
            self.memory_namespace_policy_production_write_count =
                entry.memory_namespace_policy_production_write_count;
            self.memory_namespace_policy_graph_write_count =
                entry.memory_namespace_policy_graph_write_count;
        }
        self
    }

    fn with_memory_write_chain_readiness_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::MemoryWriteChainReadiness
            && let Some(entry) = entry
        {
            self.memory_write_chain_namespace_count = entry.memory_write_chain_namespace_count;
            self.memory_write_chain_stage_required_count =
                entry.memory_write_chain_stage_required_count;
            self.memory_write_chain_stage_pass_count = entry.memory_write_chain_stage_pass_count;
            self.memory_write_chain_propose_write_ready_count =
                entry.memory_write_chain_propose_write_ready_count;
            self.memory_write_chain_policy_approval_ready_count =
                entry.memory_write_chain_policy_approval_ready_count;
            self.memory_write_chain_operator_approval_ready_count =
                entry.memory_write_chain_operator_approval_ready_count;
            self.memory_write_chain_shadow_wal_ready_count =
                entry.memory_write_chain_shadow_wal_ready_count;
            self.memory_write_chain_readback_ready_count =
                entry.memory_write_chain_readback_ready_count;
            self.memory_write_chain_canary_ready_count =
                entry.memory_write_chain_canary_ready_count;
            self.memory_write_chain_rollback_ready_count =
                entry.memory_write_chain_rollback_ready_count;
            self.memory_write_chain_production_write_count =
                entry.memory_write_chain_production_write_count;
            self.memory_write_chain_graph_write_count = entry.memory_write_chain_graph_write_count;
        }
        self
    }

    fn with_memory_write_chain_receipt_freshness_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness
            && let Some(entry) = entry
        {
            self.memory_write_chain_receipt_namespace_count =
                entry.memory_write_chain_receipt_namespace_count;
            self.memory_write_chain_receipt_required_count =
                entry.memory_write_chain_receipt_required_count;
            self.memory_write_chain_receipt_projected_count =
                entry.memory_write_chain_receipt_projected_count;
            self.memory_write_chain_receipt_digest_count =
                entry.memory_write_chain_receipt_digest_count;
            self.memory_write_chain_receipt_freshness_pass_count =
                entry.memory_write_chain_receipt_freshness_pass_count;
            self.memory_write_chain_receipt_replay_guard_pass_count =
                entry.memory_write_chain_receipt_replay_guard_pass_count;
            self.memory_write_chain_receipt_stale_replay_rejected_count =
                entry.memory_write_chain_receipt_stale_replay_rejected_count;
            self.memory_write_chain_receipt_recorded_count =
                entry.memory_write_chain_receipt_recorded_count;
            self.memory_write_chain_receipt_persisted_count =
                entry.memory_write_chain_receipt_persisted_count;
            self.memory_write_chain_receipt_production_write_count =
                entry.memory_write_chain_receipt_production_write_count;
            self.memory_write_chain_receipt_graph_write_count =
                entry.memory_write_chain_receipt_graph_write_count;
        }
        self
    }

    pub fn has_row_integrity(&self) -> bool {
        !self.target.is_unknown()
            && !self.observed_status.is_unknown()
            && !self.required_status.is_unknown()
            && !self.blocker_reason.is_unknown()
            && self.threshold_satisfied != self.blocker_reason.is_blocking()
            && self.has_recall_quality_rollup_integrity()
            && self.has_ranked_recall_rollup_integrity()
            && self.has_canary_promotion_rollup_integrity()
            && self.has_memory_namespace_policy_rollup_integrity()
            && self.has_memory_write_chain_readiness_rollup_integrity()
            && self.has_memory_write_chain_receipt_freshness_rollup_integrity()
            && self.has_memory_provider_v2_rollup_integrity()
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    fn has_recall_quality_rollup_integrity(&self) -> bool {
        if self.target != ContextPlaneActivationTarget::RecallQualityGate {
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
            && (!self.threshold_satisfied || self.recall_quality_blocking_reasons.is_empty())
            && (self.observed_status != ContextPlaneStatusKind::Blocked
                || !self.recall_quality_blocking_reasons.is_empty())
    }

    fn has_canary_promotion_rollup_integrity(&self) -> bool {
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

        if self.target != ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness {
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
            && no_promotion_blockers == checklist_complete
            && (!no_promotion_blockers
                || (stable_window_complete
                    && pass_streak_complete
                    && rollback_rehearsal_complete
                    && kill_switch_rehearsal_complete
                    && soak_readback_complete))
            && (self.canary_promotion_blocker_count == 0 || self.blocker_reason.is_blocking())
    }

    fn has_ranked_recall_rollup_integrity(&self) -> bool {
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

        if self.target != ContextPlaneActivationTarget::MemoryRankedRecallShadowEval {
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
            && (self.ranked_recall_positive_hybrid_signal_pass_count
                == self.ranked_recall_positive_hybrid_signal_required_count
                || self.blocker_reason.is_blocking())
    }

    fn has_memory_provider_v2_rollup_integrity(&self) -> bool {
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

        if self.target != ContextPlaneActivationTarget::MemoryProviderV2Boundary {
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
            && (self.memory_provider_v2_lifecycle_pass_count
                == self.memory_provider_v2_lifecycle_required_count
                || self.blocker_reason.is_blocking())
    }

    fn has_memory_namespace_policy_rollup_integrity(&self) -> bool {
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

        if self.target != ContextPlaneActivationTarget::MemoryNamespacePolicy {
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
            && (self.threshold_satisfied || self.blocker_reason.is_blocking())
    }

    fn has_memory_write_chain_readiness_rollup_integrity(&self) -> bool {
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

        if self.target != ContextPlaneActivationTarget::MemoryWriteChainReadiness {
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
            && (self.threshold_satisfied || self.blocker_reason.is_blocking())
    }

    fn has_memory_write_chain_receipt_freshness_rollup_integrity(&self) -> bool {
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

        if self.target != ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness {
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
            && (self.threshold_satisfied || self.blocker_reason.is_blocking())
    }
}
