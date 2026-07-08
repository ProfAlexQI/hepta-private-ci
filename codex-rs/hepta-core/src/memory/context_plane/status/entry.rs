use serde::Deserialize;
use serde::Serialize;

use super::section::ContextPlaneStatusKind;
use super::section::ContextPlaneStatusSection;
use crate::memory::ContextMemoryRankedRecallShadowEvalReport;
use crate::memory::ContextMemoryRankedRecallShadowHybridSignal;
use crate::memory::ContextMemoryRecallQualityGateBlockerReason;
use crate::memory::ContextMemoryRecallQualityGateReport;
use crate::memory::ContextMemoryShadowCanaryPromotionReadinessReport;
use crate::memory::ContextMemoryShadowQualityTrendSnapshotReport;
use crate::memory::ContextMemoryTemporalGraphShadowEvalReport;
use crate::memory::MemoryProviderReport;
use crate::memory::MemoryProviderV2AuditReport;

const CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT: usize = 4;
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
