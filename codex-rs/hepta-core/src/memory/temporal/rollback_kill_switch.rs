use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_SCHEMA_VERSION;
use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_SCHEMA_VERSION;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use super::canary_guard::ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport;

const TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_STAGE_COUNT: usize = 6;

/// Payload-light rollback and kill-switch evidence surface for temporal graph
/// shadow retrieval. This projects aggregate counters only; it never opens a
/// retrieval route, records rollback state, or writes graph state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport {
    pub schema_version: u32,
    pub source_retrieval_canary_guard_schema_version: u32,
    pub evidence_fixture_count: usize,
    pub evidence_stage_required_count: usize,
    pub evidence_stage_projected_count: usize,
    pub canary_guard_pass_count: usize,
    pub operator_approval_required_count: usize,
    pub operator_approval_recorded_count: usize,
    pub feature_flag_registered_count: usize,
    pub feature_flag_enabled_count: usize,
    pub kill_switch_registered_count: usize,
    pub kill_switch_readback_count: usize,
    pub kill_switch_pass_count: usize,
    pub rollback_rehearsal_required_count: usize,
    pub rollback_rehearsal_readback_count: usize,
    pub rollback_rehearsal_pass_count: usize,
    pub route_denial_count: usize,
    pub rollback_write_denial_count: usize,
    pub canary_route_opened_count: usize,
    pub aggregate_counters_only: bool,
    pub retrieval_rollback_kill_switch_digest: String,
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

impl Default for ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport {
    fn default() -> Self {
        Self {
            schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_SCHEMA_VERSION,
            source_retrieval_canary_guard_schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_SCHEMA_VERSION,
            evidence_fixture_count: 0,
            evidence_stage_required_count:
                TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_STAGE_COUNT,
            evidence_stage_projected_count: 0,
            canary_guard_pass_count: 0,
            operator_approval_required_count: 0,
            operator_approval_recorded_count: 0,
            feature_flag_registered_count: 0,
            feature_flag_enabled_count: 0,
            kill_switch_registered_count: 0,
            kill_switch_readback_count: 0,
            kill_switch_pass_count: 0,
            rollback_rehearsal_required_count: 0,
            rollback_rehearsal_readback_count: 0,
            rollback_rehearsal_pass_count: 0,
            route_denial_count: 0,
            rollback_write_denial_count: 0,
            canary_route_opened_count: 0,
            aggregate_counters_only: false,
            retrieval_rollback_kill_switch_digest: String::new(),
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

impl ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport {
    pub fn from_retrieval_canary_guard(
        retrieval_canary_guard: &ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport,
    ) -> Self {
        let canary_guard_integrity = retrieval_canary_guard.has_retrieval_canary_guard_integrity();
        let evidence_fixture_count = retrieval_canary_guard.guard_fixture_count;
        let kill_switch_readback = canary_guard_integrity
            && retrieval_canary_guard.kill_switch_registered_count == evidence_fixture_count
            && retrieval_canary_guard.kill_switch_ready_count == evidence_fixture_count;
        let rollback_rehearsal_readback = canary_guard_integrity
            && retrieval_canary_guard.rollback_rehearsal_required_count == evidence_fixture_count
            && retrieval_canary_guard.rollback_rehearsal_pass_count == evidence_fixture_count;
        let route_denial = canary_guard_integrity
            && retrieval_canary_guard.activation_denial_count == evidence_fixture_count
            && retrieval_canary_guard.canary_route_opened_count == 0
            && !retrieval_canary_guard.production_route;
        let rollback_write_denial = canary_guard_integrity
            && retrieval_canary_guard.rollback_write_count() == 0
            && !retrieval_canary_guard.production_write
            && !retrieval_canary_guard.graph_write;
        let replay_freshness_evidence = canary_guard_integrity
            && retrieval_canary_guard.freshness_pass_count()
                == retrieval_canary_guard.guard_stage_required_count
            && retrieval_canary_guard.replay_guard_pass_count()
                == retrieval_canary_guard.guard_stage_required_count
            && retrieval_canary_guard.stale_replay_rejected_count()
                == retrieval_canary_guard.guard_stage_required_count;
        let evidence_stage_projected_count = [
            canary_guard_integrity,
            kill_switch_readback,
            rollback_rehearsal_readback,
            route_denial,
            rollback_write_denial,
            replay_freshness_evidence,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count();
        let aggregate_counters_only = evidence_stage_projected_count
            == TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_STAGE_COUNT;
        let retrieval_rollback_kill_switch_digest = if aggregate_counters_only {
            stable_receipt_hash(&[
                "memory_temporal_graph_shadow_retrieval_rollback_kill_switch",
                &evidence_fixture_count.to_string(),
                &evidence_stage_projected_count.to_string(),
                &retrieval_canary_guard.kill_switch_ready_count.to_string(),
                &retrieval_canary_guard
                    .rollback_rehearsal_pass_count
                    .to_string(),
                &retrieval_canary_guard.activation_denial_count.to_string(),
                &retrieval_canary_guard.retrieval_canary_guard_digest,
                "retrieval_rollback_kill_switch_v1",
            ])
        } else {
            String::new()
        };

        Self {
            source_retrieval_canary_guard_schema_version: retrieval_canary_guard.schema_version,
            evidence_fixture_count,
            evidence_stage_projected_count,
            canary_guard_pass_count: if canary_guard_integrity {
                evidence_fixture_count
            } else {
                0
            },
            operator_approval_required_count: retrieval_canary_guard
                .operator_approval_required_count,
            feature_flag_registered_count: retrieval_canary_guard.feature_flag_registered_count,
            kill_switch_registered_count: retrieval_canary_guard.kill_switch_registered_count,
            kill_switch_readback_count: if kill_switch_readback {
                evidence_fixture_count
            } else {
                0
            },
            kill_switch_pass_count: if kill_switch_readback {
                evidence_fixture_count
            } else {
                0
            },
            rollback_rehearsal_required_count: retrieval_canary_guard
                .rollback_rehearsal_required_count,
            rollback_rehearsal_readback_count: if rollback_rehearsal_readback {
                evidence_fixture_count
            } else {
                0
            },
            rollback_rehearsal_pass_count: if rollback_rehearsal_readback {
                evidence_fixture_count
            } else {
                0
            },
            route_denial_count: if route_denial {
                evidence_fixture_count
            } else {
                0
            },
            rollback_write_denial_count: if rollback_write_denial {
                evidence_fixture_count
            } else {
                0
            },
            aggregate_counters_only,
            retrieval_rollback_kill_switch_digest,
            freshness_check_pass: aggregate_counters_only,
            replay_guard_pass: aggregate_counters_only,
            stale_replay_rejected: aggregate_counters_only,
            llm_rerank: retrieval_canary_guard.llm_rerank,
            graph_persistence: retrieval_canary_guard.graph_persistence,
            production_route: retrieval_canary_guard.production_route,
            production_write: retrieval_canary_guard.production_write,
            graph_write: retrieval_canary_guard.graph_write,
            rollback_write: retrieval_canary_guard.rollback_write,
            hot_path_write: retrieval_canary_guard.hot_path_write,
            prompt_assembly_change: retrieval_canary_guard.prompt_assembly_change,
            runtime_activation: retrieval_canary_guard.runtime_activation,
            operator_activation_allowed: retrieval_canary_guard.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn retrieval_rollback_kill_switch_stage_required_count(&self) -> usize {
        TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_STAGE_COUNT
    }

    pub fn retrieval_rollback_kill_switch_digest_count(&self) -> usize {
        if self.retrieval_rollback_kill_switch_digest.is_empty() {
            0
        } else {
            self.evidence_stage_projected_count
        }
    }

    pub fn freshness_pass_count(&self) -> usize {
        if self.freshness_check_pass {
            self.evidence_stage_projected_count
        } else {
            0
        }
    }

    pub fn replay_guard_pass_count(&self) -> usize {
        if self.replay_guard_pass {
            self.evidence_stage_projected_count
        } else {
            0
        }
    }

    pub fn stale_replay_rejected_count(&self) -> usize {
        if self.stale_replay_rejected {
            self.evidence_stage_projected_count
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

    pub fn has_retrieval_rollback_kill_switch_integrity(&self) -> bool {
        self.schema_version
            == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_SCHEMA_VERSION
            && self.source_retrieval_canary_guard_schema_version
                == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_CANARY_GUARD_SCHEMA_VERSION
            && self.evidence_fixture_count == 5
            && self.evidence_stage_required_count
                == self.retrieval_rollback_kill_switch_stage_required_count()
            && self.evidence_stage_projected_count == self.evidence_stage_required_count
            && self.canary_guard_pass_count == self.evidence_fixture_count
            && self.operator_approval_required_count == self.evidence_fixture_count
            && self.operator_approval_recorded_count == 0
            && self.feature_flag_registered_count == self.evidence_fixture_count
            && self.feature_flag_enabled_count == 0
            && self.kill_switch_registered_count == self.evidence_fixture_count
            && self.kill_switch_readback_count == self.evidence_fixture_count
            && self.kill_switch_pass_count == self.evidence_fixture_count
            && self.rollback_rehearsal_required_count == self.evidence_fixture_count
            && self.rollback_rehearsal_readback_count == self.evidence_fixture_count
            && self.rollback_rehearsal_pass_count == self.evidence_fixture_count
            && self.route_denial_count == self.evidence_fixture_count
            && self.rollback_write_denial_count == self.evidence_fixture_count
            && self.canary_route_opened_count == 0
            && self.aggregate_counters_only
            && self.retrieval_rollback_kill_switch_digest_count()
                == self.evidence_stage_required_count
            && stable_receipt_hash_is_valid(&self.retrieval_rollback_kill_switch_digest)
            && self.freshness_pass_count() == self.evidence_stage_required_count
            && self.replay_guard_pass_count() == self.evidence_stage_required_count
            && self.stale_replay_rejected_count() == self.evidence_stage_required_count
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
