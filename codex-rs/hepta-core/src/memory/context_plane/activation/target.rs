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
    MemoryTemporalGraphShadowTraversalQuality,
    MemoryTemporalGraphShadowRetrievalCanaryGuard,
    MemoryTemporalGraphShadowRetrievalRollbackKillSwitch,
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
    TemporalGraphShadowTraversalQualityShadowOnly,
    TemporalGraphShadowRetrievalCanaryGuardShadowOnly,
    TemporalGraphShadowRetrievalRollbackKillSwitchShadowOnly,
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
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalQuality => 14,
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard => 15,
        ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch => 16,
        ContextPlaneActivationTarget::EvalHarnessSeed => 17,
        ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow => 18,
        ContextPlaneActivationTarget::RecallQualityGate => 19,
        ContextPlaneActivationTarget::MemoryRankedRecallShadowEval => 20,
        ContextPlaneActivationTarget::MemoryProviderBoundary => 21,
        ContextPlaneActivationTarget::MemoryProviderV2Boundary => 22,
        ContextPlaneActivationTarget::MemoryShadowCanaryReadiness => 23,
        ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness => 24,
        ContextPlaneActivationTarget::SourceAwareFrontDoor => 25,
        ContextPlaneActivationTarget::OperatorApproval => 26,
        ContextPlaneActivationTarget::Unknown => 27,
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
        ContextPlaneActivationBlockerReason::TemporalGraphShadowTraversalQualityShadowOnly => 10,
        ContextPlaneActivationBlockerReason::TemporalGraphShadowRetrievalCanaryGuardShadowOnly => {
            11
        }
        ContextPlaneActivationBlockerReason::TemporalGraphShadowRetrievalRollbackKillSwitchShadowOnly => {
            12
        }
        ContextPlaneActivationBlockerReason::MemoryRankedRecallShadowEvalShadowOnly => 13,
        ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly => 14,
        ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly => 15,
        ContextPlaneActivationBlockerReason::MemoryNamespacePolicyShadowOnly => 16,
        ContextPlaneActivationBlockerReason::MemoryWriteChainReadinessShadowOnly => 17,
        ContextPlaneActivationBlockerReason::MemoryWriteChainReceiptFreshnessShadowOnly => 18,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly => 19,
        ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly => 20,
        ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled => 21,
        ContextPlaneActivationBlockerReason::OperatorApprovalMissing => 22,
        ContextPlaneActivationBlockerReason::UnexpectedStatus => 23,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled => 24,
        ContextPlaneActivationBlockerReason::Unknown => 25,
    }
}
