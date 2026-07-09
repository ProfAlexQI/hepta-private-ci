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
        ContextPlaneActivationTarget::EvalHarnessSeed => 13,
        ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow => 14,
        ContextPlaneActivationTarget::RecallQualityGate => 15,
        ContextPlaneActivationTarget::MemoryRankedRecallShadowEval => 16,
        ContextPlaneActivationTarget::MemoryProviderBoundary => 17,
        ContextPlaneActivationTarget::MemoryProviderV2Boundary => 18,
        ContextPlaneActivationTarget::MemoryShadowCanaryReadiness => 19,
        ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness => 20,
        ContextPlaneActivationTarget::SourceAwareFrontDoor => 21,
        ContextPlaneActivationTarget::OperatorApproval => 22,
        ContextPlaneActivationTarget::Unknown => 23,
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
        ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly => 9,
        ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly => 10,
        ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly => 11,
        ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly => 12,
        ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly => 13,
        ContextPlaneActivationBlockerReason::MemoryWriteChainReceiptFreshnessShadowOnly => 14,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly => 15,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly => 16,
        ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled => 17,
        ContextPlaneActivationBlockerReason::OperatorApprovalMissing => 18,
        ContextPlaneActivationBlockerReason::UnexpectedStatus => 19,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled => 20,
        ContextPlaneActivationBlockerReason::Unknown => 21,
    }
}
