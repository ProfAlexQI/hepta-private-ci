use serde::Deserialize;
use serde::Serialize;

/// Activation-readiness target represented in the context-plane blocker matrix.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextPlaneActivationTarget {
    SourceRegistry,
    AdaptiveBudgetAllocation,
    MemoryTaxonomy,
    MemoryFormationReceipts,
    MemoryFormationQueue,
    MemoryNamespacePolicy,
    MemoryWriteChainReadiness,
    MemoryWriteChainReceiptFreshness,
    MemoryTemporalFacts,
    MemoryTemporalFactGraph,
    MemoryTemporalGraphShadowEval,
    MemoryTemporalGraphShadowStore,
    MemoryTemporalGraphShadowReplay,
    MemoryTemporalGraphShadowTraversalDiff,
    EvalHarnessSeed,
    AdaptiveAllocatorEvalShadow,
    RecallQualityGate,
    MemoryRankedRecallShadowEval,
    MemoryProviderBoundary,
    MemoryProviderV2Boundary,
    MemoryShadowCanaryReadiness,
    MemoryShadowCanaryPromotionReadiness,
    SourceAwareFrontDoor,
    OperatorApproval,
    #[default]
    Unknown,
}

impl ContextPlaneActivationTarget {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Controlled activation-blocker reason for a context-plane matrix row.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextPlaneActivationBlockerReason {
    None,
    StatusMissing,
    SectionBlocked,
    SectionShadowOnly,
    SectionDisabled,
    AdaptiveBudgetAllocationShadowOnly,
    TemporalGraphShadowEvalShadowOnly,
    TemporalGraphShadowStoreShadowOnly,
    TemporalGraphShadowReplayShadowOnly,
    TemporalGraphShadowTraversalDiffShadowOnly,
    MemoryProviderBoundaryShadowOnly,
    MemoryRankedRecallShadowEvalShadowOnly,
    MemoryProviderV2BoundaryShadowOnly,
    MemoryNamespacePolicyShadowOnly,
    MemoryWriteChainReadinessShadowOnly,
    MemoryWriteChainReceiptFreshnessShadowOnly,
    MemoryShadowCanaryReadinessShadowOnly,
    MemoryShadowCanaryPromotionReadinessShadowOnly,
    SourceAwareFrontDoorDisabled,
    OperatorApprovalMissing,
    UnexpectedStatus,
    SideEffectFlagEnabled,
    #[default]
    Unknown,
}

impl ContextPlaneActivationBlockerReason {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub fn is_blocking(&self) -> bool {
        !matches!(self, Self::None)
    }
}

pub(super) fn activation_target_order(target: ContextPlaneActivationTarget) -> usize {
    match target {
        ContextPlaneActivationTarget::SourceRegistry => 0,
        ContextPlaneActivationTarget::AdaptiveBudgetAllocation => 1,
        ContextPlaneActivationTarget::MemoryTaxonomy => 2,
        ContextPlaneActivationTarget::MemoryFormationReceipts => 3,
        ContextPlaneActivationTarget::MemoryFormationQueue => 4,
        ContextPlaneActivationTarget::MemoryNamespacePolicy => 5,
        ContextPlaneActivationTarget::MemoryWriteChainReadiness => 6,
        ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness => 7,
        ContextPlaneActivationTarget::MemoryTemporalFacts => 8,
        ContextPlaneActivationTarget::MemoryTemporalFactGraph => 9,
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval => 10,
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowStore => 11,
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowReplay => 12,
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalDiff => 13,
        ContextPlaneActivationTarget::EvalHarnessSeed => 14,
        ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow => 15,
        ContextPlaneActivationTarget::RecallQualityGate => 16,
        ContextPlaneActivationTarget::MemoryRankedRecallShadowEval => 17,
        ContextPlaneActivationTarget::MemoryProviderBoundary => 18,
        ContextPlaneActivationTarget::MemoryProviderV2Boundary => 19,
        ContextPlaneActivationTarget::MemoryShadowCanaryReadiness => 20,
        ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness => 21,
        ContextPlaneActivationTarget::SourceAwareFrontDoor => 22,
        ContextPlaneActivationTarget::OperatorApproval => 23,
        ContextPlaneActivationTarget::Unknown => 24,
    }
}

pub(in crate::memory::context_plane) fn activation_blocker_reason_order(
    reason: ContextPlaneActivationBlockerReason,
) -> usize {
    match reason {
        ContextPlaneActivationBlockerReason::None => 0,
        ContextPlaneActivationBlockerReason::StatusMissing => 1,
        ContextPlaneActivationBlockerReason::SectionBlocked => 2,
        ContextPlaneActivationBlockerReason::SectionShadowOnly => 3,
        ContextPlaneActivationBlockerReason::SectionDisabled => 4,
        ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly => 5,
        ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly => 6,
        ContextPlaneActivationBlockerReason::TemporalGraphShadowStoreShadowOnly => 7,
        ContextPlaneActivationBlockerReason::TemporalGraphShadowReplayShadowOnly => 8,
        ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalDiffShadowOnly => 9,
        ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly => 10,
        ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly => 11,
        ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly => 12,
        ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly => 13,
        ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly => 14,
        ContextPlaneActivationBlockerReason::MemoryWriteChainReceiptFreshnessShadowOnly => 15,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly => 16,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly => 17,
        ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled => 18,
        ContextPlaneActivationBlockerReason::OperatorApprovalMissing => 19,
        ContextPlaneActivationBlockerReason::UnexpectedStatus => 20,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled => 21,
        ContextPlaneActivationBlockerReason::Unknown => 22,
    }
}
