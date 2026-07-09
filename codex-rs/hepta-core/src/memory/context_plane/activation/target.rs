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
        ContextPlaneActivationTarget::MemoryTemporalFacts => 7,
        ContextPlaneActivationTarget::MemoryTemporalFactGraph => 8,
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval => 9,
        ContextPlaneActivationTarget::EvalHarnessSeed => 10,
        ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow => 11,
        ContextPlaneActivationTarget::RecallQualityGate => 12,
        ContextPlaneActivationTarget::MemoryRankedRecallShadowEval => 13,
        ContextPlaneActivationTarget::MemoryProviderBoundary => 14,
        ContextPlaneActivationTarget::MemoryProviderV2Boundary => 15,
        ContextPlaneActivationTarget::MemoryShadowCanaryReadiness => 16,
        ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness => 17,
        ContextPlaneActivationTarget::SourceAwareFrontDoor => 18,
        ContextPlaneActivationTarget::OperatorApproval => 19,
        ContextPlaneActivationTarget::Unknown => 20,
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
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly => 12,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly => 13,
        ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled => 14,
        ContextPlaneActivationBlockerReason::OperatorApprovalMissing => 15,
        ContextPlaneActivationBlockerReason::UnexpectedStatus => 16,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled => 17,
        ContextPlaneActivationBlockerReason::Unknown => 18,
    }
}
