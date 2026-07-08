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
        ContextPlaneActivationTarget::MemoryProviderBoundary => 11,
        ContextPlaneActivationTarget::MemoryProviderV2Boundary => 12,
        ContextPlaneActivationTarget::MemoryShadowCanaryReadiness => 13,
        ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness => 14,
        ContextPlaneActivationTarget::SourceAwareFrontDoor => 15,
        ContextPlaneActivationTarget::OperatorApproval => 16,
        ContextPlaneActivationTarget::Unknown => 17,
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
        ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly => 7,
        ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly => 8,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly => 9,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly => 10,
        ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled => 11,
        ContextPlaneActivationBlockerReason::OperatorApprovalMissing => 12,
        ContextPlaneActivationBlockerReason::UnexpectedStatus => 13,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled => 14,
        ContextPlaneActivationBlockerReason::Unknown => 15,
    }
}
