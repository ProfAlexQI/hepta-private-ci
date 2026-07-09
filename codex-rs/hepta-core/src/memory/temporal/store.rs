use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION;
use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_SCHEMA_VERSION;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use super::graph::ContextMemoryTemporalFactGraphReport;

const TEMPORAL_GRAPH_SHADOW_STORE_STAGE_COUNT: usize = 6;

/// Payload-light, approval-gated shadow store skeleton for temporal graph
/// promotion. This projects readiness only; it never persists graph facts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowStoreReport {
    pub schema_version: u32,
    pub source_graph_schema_version: u32,
    pub node_count: usize,
    pub edge_count: usize,
    pub provenance_edge_count: usize,
    pub validity_window_edge_count: usize,
    pub supersedes_edge_count: usize,
    pub open_node_count: usize,
    pub invalidated_node_count: usize,
    pub shadow_wal_projected: bool,
    pub provenance_projected: bool,
    pub bitemporal_validity_projected: bool,
    pub fact_invalidation_projected: bool,
    pub supersede_tombstone_projected: bool,
    pub digest_freshness_projected: bool,
    pub store_digest: String,
    pub freshness_check_pass: bool,
    pub replay_guard_pass: bool,
    pub stale_replay_rejected: bool,
    pub operator_approval_required: bool,
    pub operator_approval_recorded: bool,
    pub recorded_receipt: bool,
    pub persisted_receipt: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryTemporalGraphShadowStoreReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_SCHEMA_VERSION,
            source_graph_schema_version: CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION,
            node_count: 0,
            edge_count: 0,
            provenance_edge_count: 0,
            validity_window_edge_count: 0,
            supersedes_edge_count: 0,
            open_node_count: 0,
            invalidated_node_count: 0,
            shadow_wal_projected: false,
            provenance_projected: false,
            bitemporal_validity_projected: false,
            fact_invalidation_projected: false,
            supersede_tombstone_projected: false,
            digest_freshness_projected: false,
            store_digest: String::new(),
            freshness_check_pass: false,
            replay_guard_pass: false,
            stale_replay_rejected: false,
            operator_approval_required: true,
            operator_approval_recorded: false,
            recorded_receipt: false,
            persisted_receipt: false,
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

impl ContextMemoryTemporalGraphShadowStoreReport {
    pub fn from_fact_graph(graph: &ContextMemoryTemporalFactGraphReport) -> Self {
        let graph_integrity = graph.has_graph_integrity() && !graph.is_empty();
        let store_digest = if graph_integrity {
            stable_receipt_hash(&[
                "memory_temporal_graph_shadow_store",
                &graph.nodes.len().to_string(),
                &graph.edges.len().to_string(),
                &graph.provenance_edge_count().to_string(),
                &graph.validity_window_edge_count().to_string(),
                &graph.supersedes_edge_count().to_string(),
                "shadow_only_v1",
            ])
        } else {
            String::new()
        };

        Self {
            source_graph_schema_version: graph.schema_version,
            node_count: graph.nodes.len(),
            edge_count: graph.edges.len(),
            provenance_edge_count: graph.provenance_edge_count(),
            validity_window_edge_count: graph.validity_window_edge_count(),
            supersedes_edge_count: graph.supersedes_edge_count(),
            open_node_count: graph.open_node_count(),
            invalidated_node_count: graph.invalidated_node_count(),
            shadow_wal_projected: graph_integrity,
            provenance_projected: graph_integrity,
            bitemporal_validity_projected: graph_integrity,
            fact_invalidation_projected: graph_integrity,
            supersede_tombstone_projected: graph_integrity,
            digest_freshness_projected: graph_integrity,
            store_digest,
            freshness_check_pass: graph_integrity,
            replay_guard_pass: graph_integrity,
            stale_replay_rejected: graph_integrity,
            production_write: graph.production_write,
            graph_write: graph.graph_write,
            prompt_assembly_change: graph.prompt_assembly_change,
            runtime_activation: graph.runtime_activation,
            ..Self::default()
        }
    }

    pub fn readiness_stage_required_count(&self) -> usize {
        TEMPORAL_GRAPH_SHADOW_STORE_STAGE_COUNT
    }

    pub fn readiness_stage_projected_count(&self) -> usize {
        [
            self.shadow_wal_projected,
            self.provenance_projected,
            self.bitemporal_validity_projected,
            self.fact_invalidation_projected,
            self.supersede_tombstone_projected,
            self.digest_freshness_projected,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count()
    }

    pub fn receipt_recorded_count(&self) -> usize {
        usize::from(self.recorded_receipt)
    }

    pub fn receipt_persisted_count(&self) -> usize {
        usize::from(self.persisted_receipt)
    }

    pub fn production_write_count(&self) -> usize {
        usize::from(self.production_write || self.production_route)
    }

    pub fn graph_write_count(&self) -> usize {
        usize::from(self.graph_write)
    }

    pub fn has_shadow_store_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_SCHEMA_VERSION
            && self.source_graph_schema_version == CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION
            && self.node_count > 0
            && self.edge_count >= self.node_count
            && self.provenance_edge_count == self.node_count
            && self.validity_window_edge_count == self.node_count
            && self.open_node_count + self.invalidated_node_count == self.node_count
            && self.supersedes_edge_count <= self.edge_count
            && self.readiness_stage_projected_count() == self.readiness_stage_required_count()
            && stable_receipt_hash_is_valid(&self.store_digest)
            && self.freshness_check_pass
            && self.replay_guard_pass
            && self.stale_replay_rejected
            && self.operator_approval_required
            && !self.operator_approval_recorded
            && !self.recorded_receipt
            && !self.persisted_receipt
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
            && !self.operator_activation_allowed
    }
}
