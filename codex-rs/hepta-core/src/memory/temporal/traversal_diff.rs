use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_SCHEMA_VERSION;
use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_SCHEMA_VERSION;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use super::replay::ContextMemoryTemporalGraphShadowReplayReport;

const TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_STAGE_COUNT: usize = 5;

/// Payload-light shadow diff for temporal graph retrieval and traversal.
/// This projects aggregate counters only; it never exports candidates or paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowTraversalDiffReport {
    pub schema_version: u32,
    pub source_replay_schema_version: u32,
    pub production_selection_count: usize,
    pub lexical_bm25_candidate_count: usize,
    pub semantic_candidate_count: usize,
    pub graph_traversal_candidate_count: usize,
    pub hybrid_candidate_count: usize,
    pub overlap_candidate_count: usize,
    pub graph_expansion_candidate_count: usize,
    pub traversal_diff_win_count: usize,
    pub traversal_diff_loss_count: usize,
    pub traversal_diff_cost_count: usize,
    pub lexical_bm25_projected: bool,
    pub semantic_projected: bool,
    pub graph_traversal_projected: bool,
    pub traversal_diff_projected: bool,
    pub aggregate_counters_only: bool,
    pub traversal_diff_digest: String,
    pub freshness_check_pass: bool,
    pub replay_guard_pass: bool,
    pub stale_replay_rejected: bool,
    pub llm_rerank: bool,
    pub graph_persistence: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryTemporalGraphShadowTraversalDiffReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_SCHEMA_VERSION,
            source_replay_schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_SCHEMA_VERSION,
            production_selection_count: 0,
            lexical_bm25_candidate_count: 0,
            semantic_candidate_count: 0,
            graph_traversal_candidate_count: 0,
            hybrid_candidate_count: 0,
            overlap_candidate_count: 0,
            graph_expansion_candidate_count: 0,
            traversal_diff_win_count: 0,
            traversal_diff_loss_count: 0,
            traversal_diff_cost_count: 0,
            lexical_bm25_projected: false,
            semantic_projected: false,
            graph_traversal_projected: false,
            traversal_diff_projected: false,
            aggregate_counters_only: false,
            traversal_diff_digest: String::new(),
            freshness_check_pass: false,
            replay_guard_pass: false,
            stale_replay_rejected: false,
            llm_rerank: false,
            graph_persistence: false,
            production_route: false,
            production_write: false,
            graph_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextMemoryTemporalGraphShadowTraversalDiffReport {
    pub fn from_shadow_replay(replay: &ContextMemoryTemporalGraphShadowReplayReport) -> Self {
        let replay_integrity = replay.has_shadow_replay_integrity();
        let production_selection_count = replay.node_count;
        let lexical_bm25_candidate_count = replay.node_count;
        let semantic_candidate_count = replay.provenance_replay_count;
        let graph_traversal_candidate_count = replay.edge_count;
        let overlap_candidate_count = production_selection_count.min(semantic_candidate_count);
        let graph_expansion_candidate_count =
            graph_traversal_candidate_count.saturating_sub(overlap_candidate_count);
        let hybrid_candidate_count = lexical_bm25_candidate_count + graph_expansion_candidate_count;
        let traversal_diff_win_count = usize::from(graph_expansion_candidate_count > 0);
        let traversal_diff_loss_count = usize::from(replay.fact_invalidation_replay_count > 0);
        let traversal_diff_cost_count = graph_expansion_candidate_count + traversal_diff_loss_count;
        let traversal_diff_digest = if replay_integrity {
            stable_receipt_hash(&[
                "memory_temporal_graph_shadow_traversal_diff",
                &production_selection_count.to_string(),
                &lexical_bm25_candidate_count.to_string(),
                &semantic_candidate_count.to_string(),
                &graph_traversal_candidate_count.to_string(),
                &hybrid_candidate_count.to_string(),
                &graph_expansion_candidate_count.to_string(),
                &replay.wal_replay_digest,
                "aggregate_counters_only_v1",
            ])
        } else {
            String::new()
        };

        Self {
            source_replay_schema_version: replay.schema_version,
            production_selection_count,
            lexical_bm25_candidate_count,
            semantic_candidate_count,
            graph_traversal_candidate_count,
            hybrid_candidate_count,
            overlap_candidate_count,
            graph_expansion_candidate_count,
            traversal_diff_win_count,
            traversal_diff_loss_count,
            traversal_diff_cost_count,
            lexical_bm25_projected: replay_integrity,
            semantic_projected: replay_integrity,
            graph_traversal_projected: replay_integrity,
            traversal_diff_projected: replay_integrity,
            aggregate_counters_only: replay_integrity,
            traversal_diff_digest,
            freshness_check_pass: replay_integrity,
            replay_guard_pass: replay_integrity,
            stale_replay_rejected: replay_integrity,
            production_route: replay.production_route,
            production_write: replay.production_write,
            graph_write: replay.graph_write,
            hot_path_write: replay.hot_path_write,
            prompt_assembly_change: replay.prompt_assembly_change,
            runtime_activation: replay.runtime_activation,
            operator_activation_allowed: replay.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn traversal_stage_required_count(&self) -> usize {
        TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_STAGE_COUNT
    }

    pub fn traversal_stage_projected_count(&self) -> usize {
        [
            self.lexical_bm25_projected,
            self.semantic_projected,
            self.graph_traversal_projected,
            self.traversal_diff_projected,
            self.aggregate_counters_only,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count()
    }

    pub fn traversal_digest_count(&self) -> usize {
        if self.traversal_diff_digest.is_empty() {
            0
        } else {
            self.traversal_stage_projected_count()
        }
    }

    pub fn freshness_pass_count(&self) -> usize {
        if self.freshness_check_pass {
            self.traversal_stage_projected_count()
        } else {
            0
        }
    }

    pub fn replay_guard_pass_count(&self) -> usize {
        if self.replay_guard_pass {
            self.traversal_stage_projected_count()
        } else {
            0
        }
    }

    pub fn stale_replay_rejected_count(&self) -> usize {
        if self.stale_replay_rejected {
            self.traversal_stage_projected_count()
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

    pub fn has_traversal_diff_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_SCHEMA_VERSION
            && self.source_replay_schema_version
                == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_SCHEMA_VERSION
            && self.production_selection_count > 0
            && self.lexical_bm25_candidate_count == self.production_selection_count
            && self.semantic_candidate_count == self.production_selection_count
            && self.graph_traversal_candidate_count >= self.production_selection_count
            && self.hybrid_candidate_count >= self.production_selection_count
            && self.overlap_candidate_count <= self.production_selection_count
            && self.graph_expansion_candidate_count
                == self
                    .graph_traversal_candidate_count
                    .saturating_sub(self.overlap_candidate_count)
            && self.traversal_diff_win_count
                == usize::from(self.graph_expansion_candidate_count > 0)
            && self.traversal_diff_loss_count <= self.production_selection_count
            && self.traversal_diff_cost_count
                == self.graph_expansion_candidate_count + self.traversal_diff_loss_count
            && self.traversal_stage_projected_count() == self.traversal_stage_required_count()
            && self.traversal_digest_count() == self.traversal_stage_required_count()
            && stable_receipt_hash_is_valid(&self.traversal_diff_digest)
            && self.freshness_pass_count() == self.traversal_stage_required_count()
            && self.replay_guard_pass_count() == self.traversal_stage_required_count()
            && self.stale_replay_rejected_count() == self.traversal_stage_required_count()
            && !self.llm_rerank
            && !self.graph_persistence
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
            && !self.operator_activation_allowed
    }
}
