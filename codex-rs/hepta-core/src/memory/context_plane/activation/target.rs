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
        ContextPlaneActivationTarget::MemoryTemporalFacts => 5,
        ContextPlaneActivationTarget::MemoryTemporalFactGraph => 6,
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval => 7,
        ContextPlaneActivationTarget::EvalHarnessSeed => 8,
        ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow => 9,
        ContextPlaneActivationTarget::RecallQualityGate => 10,
        ContextPlaneActivationTarget::MemoryRankedRecallShadowEval => 11,
        ContextPlaneActivationTarget::MemoryProviderBoundary => 12,
        ContextPlaneActivationTarget::MemoryProviderV2Boundary => 13,
        ContextPlaneActivationTarget::MemoryShadowCanaryReadiness => 14,
        ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness => 15,
        ContextPlaneActivationTarget::SourceAwareFrontDoor => 16,
        ContextPlaneActivationTarget::OperatorApproval => 17,
        ContextPlaneActivationTarget::Unknown => 18,
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
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly => 10,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly => 11,
        ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled => 12,
        ContextPlaneActivationBlockerReason::OperatorApprovalMissing => 13,
        ContextPlaneActivationBlockerReason::UnexpectedStatus => 14,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled => 15,
        ContextPlaneActivationBlockerReason::Unknown => 16,
    }
}
