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
        ContextPlaneActivationTarget::EvalHarnessSeed => 11,
        ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow => 12,
        ContextPlaneActivationTarget::RecallQualityGate => 13,
        ContextPlaneActivationTarget::MemoryRankedRecallShadowEval => 14,
        ContextPlaneActivationTarget::MemoryProviderBoundary => 15,
        ContextPlaneActivationTarget::MemoryProviderV2Boundary => 16,
        ContextPlaneActivationTarget::MemoryShadowCanaryReadiness => 17,
        ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness => 18,
        ContextPlaneActivationTarget::SourceAwareFrontDoor => 19,
        ContextPlaneActivationTarget::OperatorApproval => 20,
        ContextPlaneActivationTarget::Unknown => 21,
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
        ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly => 7,
        ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly => 8,
        ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly => 9,
        ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly => 10,
        ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly => 11,
        ContextPlaneActivationBlockerReason::MemoryWriteChainReceiptFreshnessShadowOnly => 12,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly => 13,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly => 14,
        ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled => 15,
        ContextPlaneActivationBlockerReason::OperatorApprovalMissing => 16,
        ContextPlaneActivationBlockerReason::UnexpectedStatus => 17,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled => 18,
        ContextPlaneActivationBlockerReason::Unknown => 19,
    }
}
