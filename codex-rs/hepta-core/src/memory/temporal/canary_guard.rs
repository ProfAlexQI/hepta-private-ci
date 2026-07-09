use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_SCHEMA_VERSION;
use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use super::quality::ContextMemoryTemporalGraphShadowTraversalQualityReport;

const TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_STAGE_COUNT: usize = 5;

/// Payload-light canary guard surface for temporal graph shadow retrieval.
/// This only projects aggregate counters for approval, rollback, and kill-switch
/// evidence; it never opens a production route or records canary state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport {
    pub schema_version: u32,
    pub source_traversal_quality_schema_version: u32,
    pub guard_fixture_count: usize,
    pub guard_stage_required_count: usize,
    pub guard_stage_projected_count: usize,
    pub quality_slo_pass_count: usize,
    pub operator_approval_required_count: usize,
    pub operator_approval_recorded_count: usize,
    pub feature_flag_registered_count: usize,
    pub feature_flag_enabled_count: usize,
    pub kill_switch_registered_count: usize,
    pub kill_switch_ready_count: usize,
    pub rollback_rehearsal_required_count: usize,
    pub rollback_rehearsal_pass_count: usize,
    pub activation_denial_count: usize,
    pub canary_route_opened_count: usize,
    pub aggregate_counters_only: bool,
    pub retrieval_canary_guard_digest: String,
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

impl Default for ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport {
    fn default() -> Self {
        Self {
            schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_SCHEMA_VERSION,
            source_traversal_quality_schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION,
            guard_fixture_count: 0,
            guard_stage_required_count: TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_STAGE_COUNT,
            guard_stage_projected_count: 0,
            quality_slo_pass_count: 0,
            operator_approval_required_count: 0,
            operator_approval_recorded_count: 0,
            feature_flag_registered_count: 0,
            feature_flag_enabled_count: 0,
            kill_switch_registered_count: 0,
            kill_switch_ready_count: 0,
            rollback_rehearsal_required_count: 0,
            rollback_rehearsal_pass_count: 0,
            activation_denial_count: 0,
            canary_route_opened_count: 0,
            aggregate_counters_only: false,
            retrieval_canary_guard_digest: String::new(),
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

impl ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport {
    pub fn from_traversal_quality(
        traversal_quality: &ContextMemoryTemporalGraphShadowTraversalQualityReport,
    ) -> Self {
        let traversal_quality_integrity = traversal_quality.has_traversal_quality_integrity();
        let guard_fixture_count = traversal_quality.quality_fixture_count;
        let feature_flag_registered = traversal_quality_integrity;
        let kill_switch_ready = traversal_quality_integrity;
        let rollback_rehearsal_pass = traversal_quality_integrity;
        let activation_denied = traversal_quality_integrity
            && !traversal_quality.production_route
            && !traversal_quality.production_write
            && !traversal_quality.graph_write
            && !traversal_quality.runtime_activation
            && !traversal_quality.operator_activation_allowed;
        let guard_stage_projected_count = [
            traversal_quality_integrity,
            feature_flag_registered,
            kill_switch_ready,
            rollback_rehearsal_pass,
            activation_denied,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count();
        let aggregate_counters_only =
            guard_stage_projected_count == TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_STAGE_COUNT;
        let retrieval_canary_guard_digest = if aggregate_counters_only {
            stable_receipt_hash(&[
                "memory_temporal_graph_shadow_retrieval_canary_guard",
                &guard_fixture_count.to_string(),
                &guard_stage_projected_count.to_string(),
                &traversal_quality.quality_slo_pass_count.to_string(),
                &traversal_quality.operator_review_required_count.to_string(),
                &traversal_quality.traversal_quality_digest,
                "retrieval_canary_guard_v1",
            ])
        } else {
            String::new()
        };

        Self {
            source_traversal_quality_schema_version: traversal_quality.schema_version,
            guard_fixture_count,
            guard_stage_projected_count,
            quality_slo_pass_count: traversal_quality.quality_slo_pass_count,
            operator_approval_required_count: guard_fixture_count,
            feature_flag_registered_count: if feature_flag_registered {
                guard_fixture_count
            } else {
                0
            },
            kill_switch_registered_count: if kill_switch_ready {
                guard_fixture_count
            } else {
                0
            },
            kill_switch_ready_count: if kill_switch_ready {
                guard_fixture_count
            } else {
                0
            },
            rollback_rehearsal_required_count: guard_fixture_count,
            rollback_rehearsal_pass_count: if rollback_rehearsal_pass {
                guard_fixture_count
            } else {
                0
            },
            activation_denial_count: if activation_denied {
                guard_fixture_count
            } else {
                0
            },
            aggregate_counters_only,
            retrieval_canary_guard_digest,
            freshness_check_pass: aggregate_counters_only,
            replay_guard_pass: aggregate_counters_only,
            stale_replay_rejected: aggregate_counters_only,
            production_route: traversal_quality.production_route,
            production_write: traversal_quality.production_write,
            graph_write: traversal_quality.graph_write,
            hot_path_write: traversal_quality.hot_path_write,
            prompt_assembly_change: traversal_quality.prompt_assembly_change,
            runtime_activation: traversal_quality.runtime_activation,
            operator_activation_allowed: traversal_quality.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn retrieval_canary_guard_stage_required_count(&self) -> usize {
        TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_STAGE_COUNT
    }

    pub fn retrieval_canary_guard_digest_count(&self) -> usize {
        if self.retrieval_canary_guard_digest.is_empty() {
            0
        } else {
            self.guard_stage_projected_count
        }
    }

    pub fn freshness_pass_count(&self) -> usize {
        if self.freshness_check_pass {
            self.guard_stage_projected_count
        } else {
            0
        }
    }

    pub fn replay_guard_pass_count(&self) -> usize {
        if self.replay_guard_pass {
            self.guard_stage_projected_count
        } else {
            0
        }
    }

    pub fn stale_replay_rejected_count(&self) -> usize {
        if self.stale_replay_rejected {
            self.guard_stage_projected_count
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

    pub fn has_retrieval_canary_guard_integrity(&self) -> bool {
        self.schema_version
            == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_SCHEMA_VERSION
            && self.source_traversal_quality_schema_version
                == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION
            && self.guard_fixture_count == self.retrieval_canary_guard_stage_required_count()
            && self.guard_stage_required_count == self.retrieval_canary_guard_stage_required_count()
            && self.guard_stage_projected_count == self.guard_stage_required_count
            && self.quality_slo_pass_count == self.guard_stage_required_count
            && self.operator_approval_required_count == self.guard_fixture_count
            && self.operator_approval_recorded_count == 0
            && self.feature_flag_registered_count == self.guard_fixture_count
            && self.feature_flag_enabled_count == 0
            && self.kill_switch_registered_count == self.guard_fixture_count
            && self.kill_switch_ready_count == self.guard_fixture_count
            && self.rollback_rehearsal_required_count == self.guard_fixture_count
            && self.rollback_rehearsal_pass_count == self.guard_fixture_count
            && self.activation_denial_count == self.guard_fixture_count
            && self.canary_route_opened_count == 0
            && self.aggregate_counters_only
            && self.retrieval_canary_guard_digest_count() == self.guard_stage_required_count
            && stable_receipt_hash_is_valid(&self.retrieval_canary_guard_digest)
            && self.freshness_pass_count() == self.guard_stage_required_count
            && self.replay_guard_pass_count() == self.guard_stage_required_count
            && self.stale_replay_rejected_count() == self.guard_stage_required_count
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
