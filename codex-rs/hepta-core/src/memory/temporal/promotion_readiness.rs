use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_PROMOTION_READINESS_SCHEMA_VERSION;
use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_SCHEMA_VERSION;
use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use super::quality::ContextMemoryTemporalGraphShadowTraversalQualityReport;
use super::rollback_kill_switch::ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport;

const TEMPORAL_GRAPH_SHADOW_RETRIEVAL_PROMOTION_READINESS_STAGE_COUNT: usize = 7;

/// Payload-light canary promotion readiness surface for temporal graph shadow
/// retrieval. This projects aggregate counters only; it never opens a route,
/// persists graph facts, or writes canary/rollback state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowRetrievalPromotionReadinessReport {
    pub schema_version: u32,
    pub source_retrieval_rollback_kill_switch_schema_version: u32,
    pub source_traversal_quality_schema_version: u32,
    pub promotion_fixture_count: usize,
    pub promotion_stage_required_count: usize,
    pub promotion_stage_projected_count: usize,
    pub rollback_kill_switch_pass_count: usize,
    pub real_workload_trace_required_count: usize,
    pub real_workload_trace_shadow_only_count: usize,
    pub real_workload_trace_slo_pass_count: usize,
    pub real_workload_trace_win_count: usize,
    pub real_workload_trace_loss_count: usize,
    pub real_workload_trace_operator_review_required_count: usize,
    pub real_workload_trace_leak_rate_basis_points: u32,
    pub real_workload_trace_coverage_basis_points: u32,
    pub real_workload_trace_precision_basis_points: u32,
    pub real_workload_trace_token_saved_estimate: usize,
    pub real_workload_trace_latency_ms: u32,
    pub operator_approval_required_count: usize,
    pub operator_approval_recorded_count: usize,
    pub feature_flag_registered_count: usize,
    pub feature_flag_enabled_count: usize,
    pub kill_switch_pass_count: usize,
    pub rollback_rehearsal_pass_count: usize,
    pub route_denial_count: usize,
    pub rollback_write_denial_count: usize,
    pub canary_route_opened_count: usize,
    pub promotion_ready_shadow_only_count: usize,
    pub aggregate_counters_only: bool,
    pub retrieval_promotion_readiness_digest: String,
    pub freshness_check_pass: bool,
    pub replay_guard_pass: bool,
    pub stale_replay_rejected: bool,
    pub llm_rerank: bool,
    pub graph_persistence: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub rollback_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryTemporalGraphShadowRetrievalPromotionReadinessReport {
    fn default() -> Self {
        Self {
            schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_PROMOTION_READINESS_SCHEMA_VERSION,
            source_retrieval_rollback_kill_switch_schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_SCHEMA_VERSION,
            source_traversal_quality_schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION,
            promotion_fixture_count: 0,
            promotion_stage_required_count:
                TEMPORAL_GRAPH_SHADOW_RETRIEVAL_PROMOTION_READINESS_STAGE_COUNT,
            promotion_stage_projected_count: 0,
            rollback_kill_switch_pass_count: 0,
            real_workload_trace_required_count: 0,
            real_workload_trace_shadow_only_count: 0,
            real_workload_trace_slo_pass_count: 0,
            real_workload_trace_win_count: 0,
            real_workload_trace_loss_count: 0,
            real_workload_trace_operator_review_required_count: 0,
            real_workload_trace_leak_rate_basis_points: 0,
            real_workload_trace_coverage_basis_points: 0,
            real_workload_trace_precision_basis_points: 0,
            real_workload_trace_token_saved_estimate: 0,
            real_workload_trace_latency_ms: 0,
            operator_approval_required_count: 0,
            operator_approval_recorded_count: 0,
            feature_flag_registered_count: 0,
            feature_flag_enabled_count: 0,
            kill_switch_pass_count: 0,
            rollback_rehearsal_pass_count: 0,
            route_denial_count: 0,
            rollback_write_denial_count: 0,
            canary_route_opened_count: 0,
            promotion_ready_shadow_only_count: 0,
            aggregate_counters_only: false,
            retrieval_promotion_readiness_digest: String::new(),
            freshness_check_pass: false,
            replay_guard_pass: false,
            stale_replay_rejected: false,
            llm_rerank: false,
            graph_persistence: false,
            production_route: false,
            production_write: false,
            graph_write: false,
            rollback_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextMemoryTemporalGraphShadowRetrievalPromotionReadinessReport {
    pub fn from_rollback_kill_switch_and_traversal_quality(
        rollback_kill_switch: &ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport,
        traversal_quality: &ContextMemoryTemporalGraphShadowTraversalQualityReport,
    ) -> Self {
        let rollback_kill_switch_integrity =
            rollback_kill_switch.has_retrieval_rollback_kill_switch_integrity();
        let traversal_quality_integrity = traversal_quality.has_traversal_quality_integrity();
        let promotion_fixture_count = rollback_kill_switch.evidence_fixture_count;
        let real_workload_trace_slo = traversal_quality_integrity
            && traversal_quality.quality_slo_pass_count
                == traversal_quality.quality_slo_required_count
            && traversal_quality.leak_rate_basis_points == 0
            && traversal_quality.projected_latency_ms <= traversal_quality.latency_budget_ms
            && traversal_quality.operator_review_required_count == promotion_fixture_count;
        let feature_flag_guarded = rollback_kill_switch_integrity
            && rollback_kill_switch.feature_flag_registered_count == promotion_fixture_count
            && rollback_kill_switch.feature_flag_enabled_count == 0;
        let kill_switch_guarded = rollback_kill_switch_integrity
            && rollback_kill_switch.kill_switch_pass_count == promotion_fixture_count;
        let rollback_rehearsal_guarded = rollback_kill_switch_integrity
            && rollback_kill_switch.rollback_rehearsal_pass_count == promotion_fixture_count;
        let route_denied = rollback_kill_switch_integrity
            && rollback_kill_switch.route_denial_count == promotion_fixture_count
            && rollback_kill_switch.canary_route_opened_count == 0
            && !rollback_kill_switch.production_route;
        let replay_freshness_evidence = rollback_kill_switch_integrity
            && rollback_kill_switch.freshness_pass_count()
                == rollback_kill_switch.evidence_stage_required_count
            && rollback_kill_switch.replay_guard_pass_count()
                == rollback_kill_switch.evidence_stage_required_count
            && rollback_kill_switch.stale_replay_rejected_count()
                == rollback_kill_switch.evidence_stage_required_count;
        let promotion_stage_projected_count = [
            rollback_kill_switch_integrity,
            real_workload_trace_slo,
            feature_flag_guarded,
            kill_switch_guarded,
            rollback_rehearsal_guarded,
            route_denied,
            replay_freshness_evidence,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count();
        let aggregate_counters_only = promotion_stage_projected_count
            == TEMPORAL_GRAPH_SHADOW_RETRIEVAL_PROMOTION_READINESS_STAGE_COUNT;
        let retrieval_promotion_readiness_digest = if aggregate_counters_only {
            stable_receipt_hash(&[
                "memory_temporal_graph_shadow_retrieval_promotion_readiness",
                &promotion_fixture_count.to_string(),
                &promotion_stage_projected_count.to_string(),
                &traversal_quality.coverage_basis_points.to_string(),
                &traversal_quality.precision_basis_points.to_string(),
                &traversal_quality.leak_rate_basis_points.to_string(),
                &traversal_quality.token_saved_estimate.to_string(),
                &rollback_kill_switch.kill_switch_pass_count.to_string(),
                &rollback_kill_switch
                    .rollback_rehearsal_pass_count
                    .to_string(),
                &rollback_kill_switch.route_denial_count.to_string(),
                &rollback_kill_switch.retrieval_rollback_kill_switch_digest,
                &traversal_quality.traversal_quality_digest,
                "retrieval_promotion_readiness_v1",
            ])
        } else {
            String::new()
        };

        Self {
            source_retrieval_rollback_kill_switch_schema_version: rollback_kill_switch
                .schema_version,
            source_traversal_quality_schema_version: traversal_quality.schema_version,
            promotion_fixture_count,
            promotion_stage_projected_count,
            rollback_kill_switch_pass_count: if rollback_kill_switch_integrity {
                promotion_fixture_count
            } else {
                0
            },
            real_workload_trace_required_count: promotion_fixture_count,
            real_workload_trace_shadow_only_count: if real_workload_trace_slo {
                promotion_fixture_count
            } else {
                0
            },
            real_workload_trace_slo_pass_count: traversal_quality.quality_slo_pass_count,
            real_workload_trace_win_count: traversal_quality.traversal_win_count,
            real_workload_trace_loss_count: traversal_quality.traversal_loss_count,
            real_workload_trace_operator_review_required_count: traversal_quality
                .operator_review_required_count,
            real_workload_trace_leak_rate_basis_points: traversal_quality.leak_rate_basis_points,
            real_workload_trace_coverage_basis_points: traversal_quality.coverage_basis_points,
            real_workload_trace_precision_basis_points: traversal_quality.precision_basis_points,
            real_workload_trace_token_saved_estimate: traversal_quality.token_saved_estimate,
            real_workload_trace_latency_ms: traversal_quality.projected_latency_ms,
            operator_approval_required_count: rollback_kill_switch.operator_approval_required_count,
            operator_approval_recorded_count: rollback_kill_switch.operator_approval_recorded_count,
            feature_flag_registered_count: rollback_kill_switch.feature_flag_registered_count,
            feature_flag_enabled_count: rollback_kill_switch.feature_flag_enabled_count,
            kill_switch_pass_count: rollback_kill_switch.kill_switch_pass_count,
            rollback_rehearsal_pass_count: rollback_kill_switch.rollback_rehearsal_pass_count,
            route_denial_count: rollback_kill_switch.route_denial_count,
            rollback_write_denial_count: rollback_kill_switch.rollback_write_denial_count,
            canary_route_opened_count: rollback_kill_switch.canary_route_opened_count,
            promotion_ready_shadow_only_count: if aggregate_counters_only {
                promotion_fixture_count
            } else {
                0
            },
            aggregate_counters_only,
            retrieval_promotion_readiness_digest,
            freshness_check_pass: aggregate_counters_only,
            replay_guard_pass: aggregate_counters_only,
            stale_replay_rejected: aggregate_counters_only,
            llm_rerank: rollback_kill_switch.llm_rerank,
            graph_persistence: rollback_kill_switch.graph_persistence,
            production_route: rollback_kill_switch.production_route,
            production_write: rollback_kill_switch.production_write,
            graph_write: rollback_kill_switch.graph_write,
            rollback_write: rollback_kill_switch.rollback_write,
            hot_path_write: rollback_kill_switch.hot_path_write,
            prompt_assembly_change: rollback_kill_switch.prompt_assembly_change,
            runtime_activation: rollback_kill_switch.runtime_activation,
            operator_activation_allowed: rollback_kill_switch.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn retrieval_promotion_readiness_stage_required_count(&self) -> usize {
        TEMPORAL_GRAPH_SHADOW_RETRIEVAL_PROMOTION_READINESS_STAGE_COUNT
    }

    pub fn retrieval_promotion_readiness_digest_count(&self) -> usize {
        if self.retrieval_promotion_readiness_digest.is_empty() {
            0
        } else {
            self.promotion_stage_projected_count
        }
    }

    pub fn freshness_pass_count(&self) -> usize {
        if self.freshness_check_pass {
            self.promotion_stage_projected_count
        } else {
            0
        }
    }

    pub fn replay_guard_pass_count(&self) -> usize {
        if self.replay_guard_pass {
            self.promotion_stage_projected_count
        } else {
            0
        }
    }

    pub fn stale_replay_rejected_count(&self) -> usize {
        if self.stale_replay_rejected {
            self.promotion_stage_projected_count
        } else {
            0
        }
    }

    pub fn llm_rerank_count(&self) -> usize {
        usize::from(self.llm_rerank)
    }

    pub fn graph_persistence_count(&self) -> usize {
        usize::from(self.graph_persistence)
    }

    pub fn production_route_count(&self) -> usize {
        usize::from(self.production_route)
    }

    pub fn production_write_count(&self) -> usize {
        usize::from(self.production_write)
    }

    pub fn graph_write_count(&self) -> usize {
        usize::from(self.graph_write)
    }

    pub fn rollback_write_count(&self) -> usize {
        usize::from(self.rollback_write)
    }

    pub fn has_retrieval_promotion_readiness_integrity(&self) -> bool {
        self.schema_version
            == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_PROMOTION_READINESS_SCHEMA_VERSION
            && self.source_retrieval_rollback_kill_switch_schema_version
                == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_SCHEMA_VERSION
            && self.source_traversal_quality_schema_version
                == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION
            && self.promotion_fixture_count == 5
            && self.promotion_stage_required_count
                == self.retrieval_promotion_readiness_stage_required_count()
            && self.promotion_stage_projected_count == self.promotion_stage_required_count
            && self.rollback_kill_switch_pass_count == self.promotion_fixture_count
            && self.real_workload_trace_required_count == self.promotion_fixture_count
            && self.real_workload_trace_shadow_only_count == self.promotion_fixture_count
            && self.real_workload_trace_slo_pass_count == self.promotion_fixture_count
            && self.real_workload_trace_win_count == 1
            && self.real_workload_trace_loss_count == 0
            && self.real_workload_trace_operator_review_required_count
                == self.promotion_fixture_count
            && self.real_workload_trace_leak_rate_basis_points == 0
            && self.real_workload_trace_coverage_basis_points >= 8_000
            && self.real_workload_trace_precision_basis_points >= 8_000
            && self.real_workload_trace_token_saved_estimate > 0
            && self.real_workload_trace_latency_ms <= 20
            && self.operator_approval_required_count == self.promotion_fixture_count
            && self.operator_approval_recorded_count == 0
            && self.feature_flag_registered_count == self.promotion_fixture_count
            && self.feature_flag_enabled_count == 0
            && self.kill_switch_pass_count == self.promotion_fixture_count
            && self.rollback_rehearsal_pass_count == self.promotion_fixture_count
            && self.route_denial_count == self.promotion_fixture_count
            && self.rollback_write_denial_count == self.promotion_fixture_count
            && self.canary_route_opened_count == 0
            && self.promotion_ready_shadow_only_count == self.promotion_fixture_count
            && self.aggregate_counters_only
            && self.retrieval_promotion_readiness_digest_count()
                == self.promotion_stage_required_count
            && stable_receipt_hash_is_valid(&self.retrieval_promotion_readiness_digest)
            && self.freshness_pass_count() == self.promotion_stage_required_count
            && self.replay_guard_pass_count() == self.promotion_stage_required_count
            && self.stale_replay_rejected_count() == self.promotion_stage_required_count
            && !self.llm_rerank
            && !self.graph_persistence
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.rollback_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
            && !self.operator_activation_allowed
    }
}
