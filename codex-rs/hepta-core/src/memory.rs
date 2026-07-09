//! Core contracts for session metadata, transcript recall, and retrievable memory.
//!
//! The goal here is to keep the boundary intentionally small so runtimes can
//! swap storage backends without pulling storage details into higher layers.

#[cfg(test)]
use crate::TranscriptSpanRef;
#[cfg(test)]
use crate::intelligence::TopicSession;
#[cfg(test)]
use crate::intelligence::TopicSessionStatus;
#[cfg(test)]
use crate::model::MessageRole;
use crate::runtime_types::AgentId;
use crate::runtime_types::SessionId;
use serde::Deserialize;
use serde::Serialize;

mod context_plane;
mod eval_harness;
mod formation;
mod provider_plane;
mod provider_plane_v2;
mod query;
mod recall;
mod recall_quality_gate;
mod restore;
mod snapshot;
mod snapshot_inspection;
mod taxonomy;
mod temporal;
mod transcript;
mod write_chain;
mod write_chain_receipt;
pub use context_plane::ContextPlaneActivationBlockerMatrix;
pub use context_plane::ContextPlaneActivationBlockerReason;
pub use context_plane::ContextPlaneActivationBlockerRow;
pub use context_plane::ContextPlaneActivationTarget;
pub use context_plane::ContextPlaneOperatorApprovalBlockerReasonCount;
pub use context_plane::ContextPlaneOperatorApprovalPacket;
pub use context_plane::ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount;
pub use context_plane::ContextPlaneOperatorApprovalScope;
pub use context_plane::ContextPlaneOperatorApprovalThresholdSnapshot;
pub use context_plane::ContextPlaneStatusEntry;
pub use context_plane::ContextPlaneStatusKind;
pub use context_plane::ContextPlaneStatusReport;
pub use context_plane::ContextPlaneStatusReportInput;
pub use context_plane::ContextPlaneStatusSection;
#[cfg(test)]
use context_plane::required_operator_approval_scopes;
pub use eval_harness::ContextMemoryAdaptiveAllocatorEvalArm;
pub use eval_harness::ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict;
pub use eval_harness::ContextMemoryAdaptiveAllocatorEvalShadowReport;
pub use eval_harness::ContextMemoryAdaptiveAllocatorEvalShadowResult;
pub use eval_harness::ContextMemoryAdaptiveAllocatorEvalShadowVerdict;
pub use eval_harness::ContextMemoryEvalFixtureKind;
pub use eval_harness::ContextMemoryEvalFixtureResult;
pub use eval_harness::ContextMemoryEvalHarnessReport;
pub use eval_harness::ContextMemoryEvalMetric;
pub use eval_harness::ContextMemoryRankedRecallShadowEvalFixtureKind;
pub use eval_harness::ContextMemoryRankedRecallShadowEvalFixtureResult;
pub use eval_harness::ContextMemoryRankedRecallShadowEvalMetric;
pub use eval_harness::ContextMemoryRankedRecallShadowEvalMode;
pub use eval_harness::ContextMemoryRankedRecallShadowEvalReport;
pub use eval_harness::ContextMemoryRankedRecallShadowHybridSignal;
pub use eval_harness::ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind;
pub use eval_harness::ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult;
pub use eval_harness::ContextMemorySelectedRecallSummaryCanaryEvalMetric;
pub use eval_harness::ContextMemorySelectedRecallSummaryCanaryEvalMode;
pub use eval_harness::ContextMemorySelectedRecallSummaryCanaryEvalReport;
pub use eval_harness::ContextMemoryShadowCanaryPromotionDecision;
pub use eval_harness::ContextMemoryShadowCanaryPromotionMode;
pub use eval_harness::ContextMemoryShadowCanaryPromotionReadinessReport;
pub use eval_harness::ContextMemoryShadowCanaryRehearsalVerdict;
pub use eval_harness::ContextMemoryShadowQualityOperatorSummary;
pub use eval_harness::ContextMemoryShadowQualitySummaryMode;
pub use eval_harness::ContextMemoryShadowQualitySummaryReport;
pub use eval_harness::ContextMemoryShadowQualityTrend;
pub use eval_harness::ContextMemoryShadowQualityTrendSnapshotMode;
pub use eval_harness::ContextMemoryShadowQualityTrendSnapshotReport;
pub use eval_harness::ContextMemoryShadowQualityTrendWindowVerdict;
pub use eval_harness::ContextMemoryShadowRegressionDashboardMode;
pub use eval_harness::ContextMemoryShadowRegressionDashboardReport;
pub use eval_harness::ContextMemoryTemporalGraphShadowEvalFixtureKind;
pub use eval_harness::ContextMemoryTemporalGraphShadowEvalFixtureResult;
pub use eval_harness::ContextMemoryTemporalGraphShadowEvalMetric;
pub use eval_harness::ContextMemoryTemporalGraphShadowEvalMode;
pub use eval_harness::ContextMemoryTemporalGraphShadowEvalReport;
pub use formation::ContextMemoryFormationCandidateType;
pub use formation::ContextMemoryFormationQueueItem;
pub use formation::ContextMemoryFormationQueueOperatorPolicy;
pub use formation::ContextMemoryFormationQueueReport;
pub use formation::ContextMemoryFormationReceipt;
pub use formation::ContextMemoryFormationReceiptReport;
pub use provider_plane::MemoryProvider;
pub use provider_plane::MemoryProviderCapability;
pub use provider_plane::MemoryProviderClearReport;
pub use provider_plane::MemoryProviderClearRequest;
pub use provider_plane::MemoryProviderClearScope;
pub use provider_plane::MemoryProviderContextUpdateEnvelope;
pub use provider_plane::MemoryProviderContextUpdateMode;
pub use provider_plane::MemoryProviderDescriptor;
pub use provider_plane::MemoryProviderKind;
pub use provider_plane::MemoryProviderPlaneReport;
pub use provider_plane::MemoryProviderReport;
pub use provider_plane::MemoryProviderStatus;
pub use provider_plane_v2::MEMORY_PROVIDER_V2_SHADOW_BOUNDARY_SCHEMA_VERSION;
pub use provider_plane_v2::MemoryProviderAddReport;
pub use provider_plane_v2::MemoryProviderAddRequest;
pub use provider_plane_v2::MemoryProviderCloseReport;
pub use provider_plane_v2::MemoryProviderV2;
pub use provider_plane_v2::MemoryProviderV2AuditReport;
pub use provider_plane_v2::MemoryProviderWriteMode;
pub use provider_plane_v2::MemoryProviderWriteProposalReport;
pub use query::MemoryQuery;
pub use query::MemoryQueryReport;
pub use query::QueryReportCoverage;
pub use query::QueryReportLimitPressure;
pub use recall::ContextBudget;
pub use recall::ContextRecallAvailability;
pub use recall::ContextRecallBundle;
pub use recall::ContextRecallCoverage;
pub use recall::ContextRecallCoverageCounts;
pub use recall::ContextRecallInspection;
pub use recall::ContextRecallItem;
pub use recall::ContextRecallLimitPressure;
pub use recall::ContextRecallOmissionCounts;
pub use recall::ContextRecallReport;
pub use recall::ContextRecallRequest;
pub use recall::ContextRecallScore;
pub use recall::ContextRecallSource;
pub use recall::ContextRecallSourceAvailability;
pub use recall::ContextRecallSourceCounts;
pub use recall::ContextRecallTranscriptProvenanceSummary;
pub use recall::IntelligenceTurnFrame;
use recall::basis_points;
use recall::privacy_class_is_payload_light;
use recall::stable_receipt_hash;
use recall::stable_receipt_hash_is_valid;
pub use recall_quality_gate::ContextMemoryRecallQualityFixtureGateReport;
pub use recall_quality_gate::ContextMemoryRecallQualityGateBlockerReason;
pub use recall_quality_gate::ContextMemoryRecallQualityGateReport;
pub use recall_quality_gate::ContextMemoryRecallQualityGateVerdict;
pub use restore::MemoryRestoreDelta;
pub use restore::RestoreDeltaCounts;
pub use restore::SessionRestoreDelta;
pub use restore::SnapshotRestoreDomain;
pub use restore::SnapshotRestoreDomainImpact;
pub use restore::SnapshotRestoreImpact;
pub use restore::SnapshotRestoreMutationProfile;
pub use restore::SnapshotRestorePreview;
pub use restore::SnapshotRestoreReadiness;
pub use restore::SnapshotRestoreSafety;
pub use restore::TranscriptRestoreDelta;
pub use snapshot::MemorySnapshotIntegrityReport;
pub use snapshot::MemorySnapshotManifest;
pub use snapshot::MemorySnapshotStats;
pub use snapshot::SessionAgentDescriptor;
pub use snapshot::SessionAgentInventory;
pub use snapshot::SnapshotMemoryDescriptor;
pub use snapshot::SnapshotSessionDescriptor;
pub use snapshot::SnapshotTranscriptDescriptor;
pub use snapshot::TranscriptSequenceCollision;
pub use snapshot::TranscriptSessionDescriptor;
pub use snapshot::TranscriptSessionInventory;
pub use snapshot::TranscriptSnapshotIntegrityReport;
pub use snapshot::TranscriptSnapshotManifest;
pub use snapshot::TranscriptSnapshotStats;
pub use snapshot_inspection::SnapshotAuditReport;
pub use snapshot_inspection::SnapshotInspectionBundle;
pub use snapshot_inspection::SnapshotInspectionDriftImpact;
pub use snapshot_inspection::SnapshotInspectionDriftReport;
pub use snapshot_inspection::SnapshotInspectionHealth;
pub use snapshot_inspection::SnapshotInspectionSection;
pub use snapshot_inspection::SnapshotIssueSummary;
pub use taxonomy::ContextMemoryNamespace;
pub use taxonomy::ContextMemoryNamespaceOwner;
pub use taxonomy::ContextMemoryNamespacePolicyBlock;
pub use taxonomy::ContextMemoryNamespacePolicyReport;
pub use taxonomy::ContextMemoryNamespacePrivacyTier;
pub use taxonomy::ContextMemoryNamespaceRedactionPolicy;
pub use taxonomy::ContextMemoryNamespaceTtlPolicy;
pub use taxonomy::ContextMemoryNamespaceWritePolicy;
pub use taxonomy::ContextMemoryTaxonomyBucket;
pub use taxonomy::ContextMemoryTaxonomyClass;
pub use taxonomy::ContextMemoryTaxonomyReport;
pub use temporal::ContextMemoryTemporalFact;
pub use temporal::ContextMemoryTemporalFactGraphEdge;
pub use temporal::ContextMemoryTemporalFactGraphEdgeKind;
pub use temporal::ContextMemoryTemporalFactGraphNode;
pub use temporal::ContextMemoryTemporalFactGraphReport;
pub use temporal::ContextMemoryTemporalFactReport;
pub use temporal::ContextMemoryTemporalFactType;
pub use temporal::ContextMemoryTemporalGraphShadowReplayReport;
pub use temporal::ContextMemoryTemporalGraphShadowStoreReport;
pub use temporal::ContextMemoryTemporalGraphShadowTraversalDiffReport;
pub use transcript::TranscriptEntry;
pub use transcript::TranscriptEntryKind;
pub use transcript::TranscriptQuery;
pub use transcript::TranscriptQueryReport;
pub use transcript::TranscriptRange;
pub use transcript::TranscriptSpan;
pub use write_chain::ContextMemoryWriteChainReadinessBlock;
pub use write_chain::ContextMemoryWriteChainReadinessReport;
pub use write_chain_receipt::ContextMemoryWriteChainReceiptFreshnessBlock;
pub use write_chain_receipt::ContextMemoryWriteChainReceiptFreshnessReport;

pub const CONTEXT_MEMORY_EVAL_HARNESS_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_ADAPTIVE_ALLOCATOR_EVAL_SHADOW_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_RANKED_RECALL_SHADOW_EVAL_SCHEMA_VERSION: u32 = 6;
pub const CONTEXT_MEMORY_SELECTED_RECALL_SUMMARY_CANARY_EVAL_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_SHADOW_CANARY_PROMOTION_READINESS_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_SHADOW_REGRESSION_DASHBOARD_SCHEMA_VERSION: u32 = 5;
pub const CONTEXT_MEMORY_SHADOW_QUALITY_SUMMARY_SCHEMA_VERSION: u32 = 5;
pub const CONTEXT_MEMORY_SHADOW_QUALITY_TREND_SNAPSHOT_SCHEMA_VERSION: u32 = 5;
pub const CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_EVAL_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_RECALL_QUALITY_GATE_SCHEMA_VERSION: u32 = 2;
pub const CONTEXT_MEMORY_FORMATION_QUEUE_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_NAMESPACE_POLICY_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_WRITE_CHAIN_READINESS_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_WRITE_CHAIN_RECEIPT_FRESHNESS_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_STORE_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_SCHEMA_VERSION: u32 = 1;
pub const CONTEXT_PLANE_STATUS_SCHEMA_VERSION: u32 = 17;
pub const CONTEXT_PLANE_ACTIVATION_BLOCKER_SCHEMA_VERSION: u32 = 17;
pub const CONTEXT_PLANE_OPERATOR_APPROVAL_PACKET_SCHEMA_VERSION: u32 = 16;

/// Declares whether a memory record belongs to a single session or to a wider
/// cross-session corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryScope {
    Session,
    LongTerm,
}

/// Minimal session projection that storage adapters must preserve across
/// snapshots, export/import, and lookup flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRecord {
    pub session_id: SessionId,
    pub agent_id: AgentId,
    pub title: String,
    pub created_at_unix_ms: u64,
    pub last_active_unix_ms: u64,
    pub last_user_intent_summary: Option<String>,
    pub archived_at_unix_ms: Option<u64>,
}

/// Portable memory payload used at the storage boundary.
///
/// Backends can maintain richer indexes internally, but the contract that moves
/// between crates remains this stable representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,
    pub scope: MemoryScope,
    pub content: String,
}

/// Semantic class for a promoted memory item.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotedMemoryKind {
    Preference,
    Task,
    Decision,
    Fact,
    Summary,
    #[default]
    Other,
}

/// Provenance attached to a promoted memory without forcing every lightweight
/// memory hit to carry full source payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PromotedMemoryProvenance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_session_id: Option<SessionId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_turn_range: Option<TranscriptRange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_entry_ids: Vec<String>,
}

/// Provenance-aware promoted memory envelope used by Hepta Intelligence.
///
/// `MemoryRecord` remains the portable storage/search payload; this wrapper is
/// the richer contract for durable promoted memories that must cite where they
/// came from and when their confidence was last checked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromotedMemoryRecord {
    pub record: MemoryRecord,
    #[serde(default)]
    pub memory_kind: PromotedMemoryKind,
    #[serde(default)]
    pub provenance: PromotedMemoryProvenance,
    pub confidence: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_revalidated_unix_ms: Option<u64>,
}

/// Session persistence boundary.
///
/// Implementations may expose additional helper methods, but these two calls
/// define the minimum runtime dependency on session storage.
pub trait SessionStore: Send + Sync {
    async fn create(&self, record: SessionRecord) -> Result<(), crate::MemoryError>;
    async fn get(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<SessionRecord>, crate::MemoryError>;
}

/// Transcript persistence boundary used by the runtime.
pub trait TranscriptStore: Send + Sync {
    async fn append(&self, entry: TranscriptEntry) -> Result<(), crate::MemoryError>;
    async fn query(
        &self,
        query: TranscriptQuery,
    ) -> Result<TranscriptQueryReport, crate::MemoryError>;
}

/// Memory persistence boundary used by the runtime.
pub trait MemoryStore: Send + Sync {
    async fn put(&self, record: MemoryRecord) -> Result<(), crate::MemoryError>;
    async fn search(&self, query: MemoryQuery) -> Result<Vec<MemoryRecord>, crate::MemoryError>;
}

/// Optional report-bearing extension for memory retrieval backends.
///
/// This keeps [`MemoryStore`] small for simple adapters while giving richer
/// backends and automation a stable contract for matched counts and truncation
/// metadata, mirroring the report envelope already used by [`TranscriptStore`].
pub trait MemoryReportStore: MemoryStore {
    async fn search_report(
        &self,
        query: MemoryQuery,
    ) -> Result<MemoryQueryReport, crate::MemoryError>;
}

#[cfg(test)]
mod tests;
