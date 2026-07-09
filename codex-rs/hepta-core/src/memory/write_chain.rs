use serde::Deserialize;
use serde::Serialize;

use super::CONTEXT_MEMORY_WRITE_CHAIN_READINESS_SCHEMA_VERSION;
use super::ContextMemoryNamespace;
use super::ContextMemoryNamespacePolicyBlock;
use super::ContextMemoryNamespacePolicyReport;

/// One namespace-level shadow readiness block for the future memory write chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryWriteChainReadinessBlock {
    pub namespace: ContextMemoryNamespace,
    pub propose_write_ready: bool,
    pub policy_approval_ready: bool,
    pub operator_approval_ready: bool,
    pub shadow_wal_ready: bool,
    pub readback_ready: bool,
    pub canary_ready: bool,
    pub rollback_ready: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
}

impl Default for ContextMemoryWriteChainReadinessBlock {
    fn default() -> Self {
        Self {
            namespace: ContextMemoryNamespace::Unknown,
            propose_write_ready: false,
            policy_approval_ready: false,
            operator_approval_ready: false,
            shadow_wal_ready: false,
            readback_ready: false,
            canary_ready: false,
            rollback_ready: false,
            production_write: false,
            graph_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
        }
    }
}

impl ContextMemoryWriteChainReadinessBlock {
    fn from_namespace_policy_block(block: &ContextMemoryNamespacePolicyBlock) -> Self {
        Self {
            namespace: block.namespace,
            propose_write_ready: block.propose_write_required,
            policy_approval_ready: block.policy_approval_required,
            operator_approval_ready: block.operator_approval_required,
            shadow_wal_ready: block.shadow_wal_required,
            readback_ready: block.readback_required,
            canary_ready: block.canary_required,
            rollback_ready: block.rollback_supported,
            production_write: block.production_write,
            graph_write: block.graph_write,
            hot_path_write: block.hot_path_write,
            prompt_assembly_change: block.prompt_assembly_change,
            runtime_activation: block.runtime_activation,
        }
    }

    pub fn has_readiness_integrity(&self) -> bool {
        !self.namespace.is_unknown()
            && self.propose_write_ready
            && self.policy_approval_ready
            && self.operator_approval_ready
            && self.shadow_wal_ready
            && self.readback_ready
            && self.canary_ready
            && self.rollback_ready
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
    }
}

/// Payload-light readiness/readback report for the future memory write chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryWriteChainReadinessReport {
    pub schema_version: u32,
    pub blocks: Vec<ContextMemoryWriteChainReadinessBlock>,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
}

impl Default for ContextMemoryWriteChainReadinessReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_WRITE_CHAIN_READINESS_SCHEMA_VERSION,
            blocks: Vec::new(),
            production_write: false,
            graph_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
        }
    }
}

impl ContextMemoryWriteChainReadinessReport {
    pub const REQUIRED_STAGE_COUNT: usize = 6;

    pub fn from_namespace_policy(namespace_policy: &ContextMemoryNamespacePolicyReport) -> Self {
        Self {
            blocks: namespace_policy
                .blocks
                .iter()
                .map(ContextMemoryWriteChainReadinessBlock::from_namespace_policy_block)
                .collect(),
            production_write: namespace_policy.production_write,
            graph_write: namespace_policy.graph_write,
            hot_path_write: namespace_policy.hot_path_write,
            prompt_assembly_change: namespace_policy.prompt_assembly_change,
            runtime_activation: namespace_policy.runtime_activation,
            ..Self::default()
        }
    }

    pub fn seeded() -> Self {
        Self::from_namespace_policy(&ContextMemoryNamespacePolicyReport::seeded())
    }

    pub fn namespace_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn stage_required_count(&self) -> usize {
        Self::REQUIRED_STAGE_COUNT
    }

    pub fn stage_pass_count(&self) -> usize {
        [
            self.propose_write_ready_count(),
            self.policy_approval_ready_count(),
            self.operator_approval_ready_count(),
            self.shadow_wal_ready_count(),
            self.readback_ready_count(),
            self.canary_ready_count(),
        ]
        .into_iter()
        .filter(|count| *count == self.namespace_count())
        .count()
    }

    pub fn propose_write_ready_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.propose_write_ready)
            .count()
    }

    pub fn policy_approval_ready_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.policy_approval_ready)
            .count()
    }

    pub fn operator_approval_ready_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.operator_approval_ready)
            .count()
    }

    pub fn shadow_wal_ready_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.shadow_wal_ready)
            .count()
    }

    pub fn readback_ready_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.readback_ready)
            .count()
    }

    pub fn canary_ready_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.canary_ready)
            .count()
    }

    pub fn rollback_ready_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.rollback_ready)
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

    pub fn has_readiness_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_WRITE_CHAIN_READINESS_SCHEMA_VERSION
            && self.required_namespaces_present_once()
            && self.stage_pass_count() == self.stage_required_count()
            && self
                .blocks
                .iter()
                .all(ContextMemoryWriteChainReadinessBlock::has_readiness_integrity)
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
