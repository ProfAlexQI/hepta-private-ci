use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_SCHEMA_VERSION;
use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_SCHEMA_VERSION;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use super::store::ContextMemoryTemporalGraphShadowStoreReport;

const TEMPORAL_GRAPH_SHADOW_REPLAY_STAGE_COUNT: usize = 6;

/// Payload-light replay evidence for the temporal graph shadow store WAL.
/// This projects receipts only; it never persists graph facts or receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowReplayReport {
    pub schema_version: u32,
    pub source_store_schema_version: u32,
    pub node_count: usize,
    pub edge_count: usize,
    pub provenance_replay_count: usize,
    pub bitemporal_validity_replay_count: usize,
    pub fact_invalidation_replay_count: usize,
    pub supersede_tombstone_replay_count: usize,
    pub wal_receipt_replay_projected: bool,
    pub provenance_replay_projected: bool,
    pub bitemporal_validity_replay_projected: bool,
    pub fact_invalidation_replay_projected: bool,
    pub supersede_tombstone_replay_projected: bool,
    pub digest_freshness_replay_projected: bool,
    pub wal_replay_digest: String,
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

impl Default for ContextMemoryTemporalGraphShadowReplayReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_SCHEMA_VERSION,
            source_store_schema_version: CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_SCHEMA_VERSION,
            node_count: 0,
            edge_count: 0,
            provenance_replay_count: 0,
            bitemporal_validity_replay_count: 0,
            fact_invalidation_replay_count: 0,
            supersede_tombstone_replay_count: 0,
            wal_receipt_replay_projected: false,
            provenance_replay_projected: false,
            bitemporal_validity_replay_projected: false,
            fact_invalidation_replay_projected: false,
            supersede_tombstone_replay_projected: false,
            digest_freshness_replay_projected: false,
            wal_replay_digest: String::new(),
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

impl ContextMemoryTemporalGraphShadowReplayReport {
    pub fn from_shadow_store(store: &ContextMemoryTemporalGraphShadowStoreReport) -> Self {
        let store_integrity = store.has_shadow_store_integrity();
        let supersede_tombstone_replay_count =
            store.supersedes_edge_count + store.invalidated_node_count;
        let wal_replay_digest = if store_integrity {
            stable_receipt_hash(&[
                "memory_temporal_graph_shadow_replay",
                &store.node_count.to_string(),
                &store.edge_count.to_string(),
                &store.provenance_edge_count.to_string(),
                &store.validity_window_edge_count.to_string(),
                &store.invalidated_node_count.to_string(),
                &supersede_tombstone_replay_count.to_string(),
                &store.store_digest,
                "shadow_replay_v1",
            ])
        } else {
            String::new()
        };

        Self {
            source_store_schema_version: store.schema_version,
            node_count: store.node_count,
            edge_count: store.edge_count,
            provenance_replay_count: store.provenance_edge_count,
            bitemporal_validity_replay_count: store.validity_window_edge_count,
            fact_invalidation_replay_count: store.invalidated_node_count,
            supersede_tombstone_replay_count,
            wal_receipt_replay_projected: store_integrity,
            provenance_replay_projected: store_integrity,
            bitemporal_validity_replay_projected: store_integrity,
            fact_invalidation_replay_projected: store_integrity,
            supersede_tombstone_replay_projected: store_integrity,
            digest_freshness_replay_projected: store_integrity,
            wal_replay_digest,
            freshness_check_pass: store_integrity,
            replay_guard_pass: store_integrity,
            stale_replay_rejected: store_integrity,
            production_route: store.production_route,
            production_write: store.production_write,
            graph_write: store.graph_write,
            hot_path_write: store.hot_path_write,
            prompt_assembly_change: store.prompt_assembly_change,
            runtime_activation: store.runtime_activation,
            operator_activation_allowed: store.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn replay_stage_required_count(&self) -> usize {
        TEMPORAL_GRAPH_SHADOW_REPLAY_STAGE_COUNT
    }

    pub fn replay_stage_projected_count(&self) -> usize {
        [
            self.wal_receipt_replay_projected,
            self.provenance_replay_projected,
            self.bitemporal_validity_replay_projected,
            self.fact_invalidation_replay_projected,
            self.supersede_tombstone_replay_projected,
            self.digest_freshness_replay_projected,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count()
    }

    pub fn replay_digest_count(&self) -> usize {
        if self.wal_replay_digest.is_empty() {
            0
        } else {
            self.replay_stage_projected_count()
        }
    }

    pub fn freshness_pass_count(&self) -> usize {
        if self.freshness_check_pass {
            self.replay_stage_projected_count()
        } else {
            0
        }
    }

    pub fn replay_guard_pass_count(&self) -> usize {
        if self.replay_guard_pass {
            self.replay_stage_projected_count()
        } else {
            0
        }
    }

    pub fn stale_replay_rejected_count(&self) -> usize {
        if self.stale_replay_rejected {
            self.replay_stage_projected_count()
        } else {
            0
        }
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

    pub fn has_shadow_replay_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_SCHEMA_VERSION
            && self.source_store_schema_version
                == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_SCHEMA_VERSION
            && self.node_count > 0
            && self.edge_count >= self.node_count
            && self.provenance_replay_count == self.node_count
            && self.bitemporal_validity_replay_count == self.node_count
            && self.fact_invalidation_replay_count <= self.node_count
            && self.supersede_tombstone_replay_count <= self.edge_count + self.node_count
            && self.replay_stage_projected_count() == self.replay_stage_required_count()
            && stable_receipt_hash_is_valid(&self.wal_replay_digest)
            && self.replay_digest_count() == self.replay_stage_required_count()
            && self.freshness_pass_count() == self.replay_stage_required_count()
            && self.replay_guard_pass_count() == self.replay_stage_required_count()
            && self.stale_replay_rejected_count() == self.replay_stage_required_count()
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
