use serde::Deserialize;
use serde::Serialize;

/// Top-level context-plane readiness section.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextPlaneStatusSection {
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
    EvalHarnessSeed,
    AdaptiveAllocatorEvalShadow,
    RecallQualityGate,
    MemoryRankedRecallShadowEval,
    MemoryProviderBoundary,
    MemoryProviderV2Boundary,
    MemoryShadowCanaryReadiness,
    MemoryShadowCanaryPromotionReadiness,
    SourceAwareFrontDoor,
    #[default]
    Unknown,
}

impl ContextPlaneStatusSection {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light status for one context-plane section.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextPlaneStatusKind {
    Ready,
    Shadow,
    Disabled,
    Blocked,
    #[default]
    Unknown,
}

impl ContextPlaneStatusKind {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
