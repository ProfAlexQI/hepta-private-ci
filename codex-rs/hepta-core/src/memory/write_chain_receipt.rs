use serde::Deserialize;
use serde::Serialize;

use super::CONTEXT_MEMORY_WRITE_CHAIN_READINESS_SCHEMA_VERSION;
use super::CONTEXT_MEMORY_WRITE_CHAIN_RECEIPT_FRESHNESS_SCHEMA_VERSION;
use super::ContextMemoryNamespace;
use super::ContextMemoryWriteChainReadinessBlock;
use super::ContextMemoryWriteChainReadinessReport;
use super::stable_receipt_hash;
use super::stable_receipt_hash_is_valid;

const RECEIPT_STAGES_PER_NAMESPACE: usize = 3;

/// One namespace-level projected receipt freshness/digest block for the future
/// memory write chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryWriteChainReceiptFreshnessBlock {
    pub namespace: ContextMemoryNamespace,
    pub sequence: u32,
    pub expires_after_sequence: u32,
    pub shadow_wal_receipt_projected: bool,
    pub readback_receipt_projected: bool,
    pub canary_receipt_projected: bool,
    pub receipt_digest: String,
    pub freshness_check_pass: bool,
    pub replay_guard_pass: bool,
    pub stale_replay_rejected: bool,
    pub recorded_receipt: bool,
    pub persisted_receipt: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
}

impl Default for ContextMemoryWriteChainReceiptFreshnessBlock {
    fn default() -> Self {
        Self {
            namespace: ContextMemoryNamespace::Unknown,
            sequence: 0,
            expires_after_sequence: 0,
            shadow_wal_receipt_projected: false,
            readback_receipt_projected: false,
            canary_receipt_projected: false,
            receipt_digest: String::new(),
            freshness_check_pass: false,
            replay_guard_pass: false,
            stale_replay_rejected: false,
            recorded_receipt: false,
            persisted_receipt: false,
            production_write: false,
            graph_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
        }
    }
}

impl ContextMemoryWriteChainReceiptFreshnessBlock {
    fn from_readiness_block(block: &ContextMemoryWriteChainReadinessBlock, index: usize) -> Self {
        let sequence = u32::try_from(index + 1).unwrap_or(u32::MAX);
        let readiness_integrity = block.has_readiness_integrity();
        let receipt_digest = if readiness_integrity {
            stable_receipt_hash(&[
                "memory_write_chain_receipt_freshness",
                block.namespace.as_str(),
                "shadow_wal",
                "readback",
                "canary",
                "shadow_only_v1",
            ])
        } else {
            String::new()
        };

        Self {
            namespace: block.namespace,
            sequence,
            expires_after_sequence: sequence.saturating_add(1),
            shadow_wal_receipt_projected: readiness_integrity && block.shadow_wal_ready,
            readback_receipt_projected: readiness_integrity && block.readback_ready,
            canary_receipt_projected: readiness_integrity && block.canary_ready,
            receipt_digest,
            freshness_check_pass: readiness_integrity,
            replay_guard_pass: readiness_integrity,
            stale_replay_rejected: readiness_integrity,
            recorded_receipt: false,
            persisted_receipt: false,
            production_write: block.production_write,
            graph_write: block.graph_write,
            hot_path_write: block.hot_path_write,
            prompt_assembly_change: block.prompt_assembly_change,
            runtime_activation: block.runtime_activation,
        }
    }

    pub fn projected_receipt_count(&self) -> usize {
        [
            self.shadow_wal_receipt_projected,
            self.readback_receipt_projected,
            self.canary_receipt_projected,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count()
    }

    pub fn has_receipt_integrity(&self) -> bool {
        !self.namespace.is_unknown()
            && self.sequence > 0
            && self.expires_after_sequence == self.sequence.saturating_add(1)
            && self.projected_receipt_count() == RECEIPT_STAGES_PER_NAMESPACE
            && stable_receipt_hash_is_valid(&self.receipt_digest)
            && self.freshness_check_pass
            && self.replay_guard_pass
            && self.stale_replay_rejected
            && !self.recorded_receipt
            && !self.persisted_receipt
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
    }
}

/// Payload-light projected receipt freshness/digest report for the future
/// memory write chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryWriteChainReceiptFreshnessReport {
    pub schema_version: u32,
    pub source_readiness_schema_version: u32,
    pub blocks: Vec<ContextMemoryWriteChainReceiptFreshnessBlock>,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
}

impl Default for ContextMemoryWriteChainReceiptFreshnessReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_WRITE_CHAIN_RECEIPT_FRESHNESS_SCHEMA_VERSION,
            source_readiness_schema_version: CONTEXT_MEMORY_WRITE_CHAIN_READINESS_SCHEMA_VERSION,
            blocks: Vec::new(),
            production_write: false,
            graph_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
        }
    }
}

impl ContextMemoryWriteChainReceiptFreshnessReport {
    pub fn from_readiness(readiness: &ContextMemoryWriteChainReadinessReport) -> Self {
        Self {
            source_readiness_schema_version: readiness.schema_version,
            blocks: readiness
                .blocks
                .iter()
                .enumerate()
                .map(|(index, block)| {
                    ContextMemoryWriteChainReceiptFreshnessBlock::from_readiness_block(block, index)
                })
                .collect(),
            production_write: readiness.production_write,
            graph_write: readiness.graph_write,
            hot_path_write: readiness.hot_path_write,
            prompt_assembly_change: readiness.prompt_assembly_change,
            runtime_activation: readiness.runtime_activation,
            ..Self::default()
        }
    }

    pub fn seeded() -> Self {
        Self::from_readiness(&ContextMemoryWriteChainReadinessReport::seeded())
    }

    pub fn namespace_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn receipt_required_count(&self) -> usize {
        self.namespace_count() * RECEIPT_STAGES_PER_NAMESPACE
    }

    pub fn receipt_projected_count(&self) -> usize {
        self.blocks
            .iter()
            .map(ContextMemoryWriteChainReceiptFreshnessBlock::projected_receipt_count)
            .sum()
    }

    pub fn receipt_digest_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| stable_receipt_hash_is_valid(&block.receipt_digest))
            .count()
    }

    pub fn freshness_pass_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.freshness_check_pass)
            .count()
    }

    pub fn replay_guard_pass_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.replay_guard_pass)
            .count()
    }

    pub fn stale_replay_rejected_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.stale_replay_rejected)
            .count()
    }

    pub fn recorded_receipt_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.recorded_receipt)
            .count()
    }

    pub fn persisted_receipt_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.persisted_receipt)
            .count()
    }

    pub fn production_write_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.production_write)
            .count()
    }

    pub fn graph_write_count(&self) -> usize {
        self.blocks.iter().filter(|block| block.graph_write).count()
    }

    pub fn has_receipt_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_WRITE_CHAIN_RECEIPT_FRESHNESS_SCHEMA_VERSION
            && self.source_readiness_schema_version
                == CONTEXT_MEMORY_WRITE_CHAIN_READINESS_SCHEMA_VERSION
            && self.required_namespaces_present_once()
            && self.receipt_projected_count() == self.receipt_required_count()
            && self.receipt_digest_count() == self.namespace_count()
            && self.freshness_pass_count() == self.namespace_count()
            && self.replay_guard_pass_count() == self.namespace_count()
            && self.stale_replay_rejected_count() == self.namespace_count()
            && self.recorded_receipt_count() == 0
            && self.persisted_receipt_count() == 0
            && self.production_write_count() == 0
            && self.graph_write_count() == 0
            && self
                .blocks
                .iter()
                .all(ContextMemoryWriteChainReceiptFreshnessBlock::has_receipt_integrity)
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
    }

    fn required_namespaces_present_once(&self) -> bool {
        if self.blocks.len() != ContextMemoryNamespace::REQUIRED.len() {
            return false;
        }

        let mut actual = self
            .blocks
            .iter()
            .map(|block| block.namespace.as_str())
            .collect::<Vec<_>>();
        actual.sort_unstable();
        actual.dedup();

        let mut expected = ContextMemoryNamespace::REQUIRED
            .iter()
            .map(|namespace| namespace.as_str())
            .collect::<Vec<_>>();
        expected.sort_unstable();

        actual == expected
    }
}
