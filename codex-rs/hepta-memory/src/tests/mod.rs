use super::*;
use crate::recall_helpers::MEMORY_RECALL_CONFLICT_MARKER;
use crate::recall_helpers::MEMORY_RECALL_TOMBSTONE_MARKER;
use hepta_core::AgentId;
use hepta_core::ContextMemoryAdaptiveAllocatorEvalArm;
use hepta_core::ContextMemoryAdaptiveAllocatorEvalShadowVerdict;
use hepta_core::ContextMemoryEvalFixtureKind;
use hepta_core::ContextMemoryEvalMetric;
use hepta_core::ContextMemoryFormationCandidateType;
use hepta_core::ContextMemoryRankedRecallShadowEvalFixtureKind;
use hepta_core::ContextMemoryRankedRecallShadowEvalMetric;
use hepta_core::ContextMemoryRankedRecallShadowEvalMode;
use hepta_core::ContextMemoryRankedRecallShadowHybridSignal;
use hepta_core::ContextMemoryRecallQualityGateVerdict;
use hepta_core::ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind;
use hepta_core::ContextMemorySelectedRecallSummaryCanaryEvalMetric;
use hepta_core::ContextMemorySelectedRecallSummaryCanaryEvalMode;
use hepta_core::ContextMemoryShadowRegressionDashboardMode;
use hepta_core::ContextMemoryTaxonomyBucket;
use hepta_core::ContextMemoryTaxonomyClass;
use hepta_core::ContextMemoryTaxonomyReport;
use hepta_core::ContextMemoryTemporalFactType;
use hepta_core::ContextMemoryTemporalGraphShadowEvalFixtureKind;
use hepta_core::ContextMemoryTemporalGraphShadowEvalMetric;
use hepta_core::ContextMemoryTemporalGraphShadowEvalMode;
use hepta_core::ContextPlaneActivationBlockerReason;
use hepta_core::ContextPlaneActivationTarget;
use hepta_core::ContextPlaneOperatorApprovalPacket;
use hepta_core::ContextPlaneStatusKind;
use hepta_core::ContextPlaneStatusSection;
use hepta_core::ContextRecallAvailability;
use hepta_core::ContextRecallCoverage;
use hepta_core::ContextRecallCoverageCounts;
use hepta_core::ContextRecallLimitPressure;
use hepta_core::ContextRecallOmissionCounts;
use hepta_core::ContextRecallRequest;
use hepta_core::ContextRecallSource;
use hepta_core::ContextRecallSourceAvailability;
use hepta_core::ContextRecallSourceCounts;
use hepta_core::ContextRecallTranscriptProvenanceSummary;
use hepta_core::MemoryProviderClearRequest;
use hepta_core::MemoryProviderClearScope;
use hepta_core::MemoryProviderContextUpdateMode;
use hepta_core::MemoryScope;
use hepta_core::MemorySnapshotIntegrityReport;
use hepta_core::MemorySnapshotManifest;
use hepta_core::MessageRole;
use hepta_core::QueryReportCoverage;
use hepta_core::QueryReportLimitPressure;
use hepta_core::RestoreDeltaCounts;
use hepta_core::SnapshotAuditReport;
use hepta_core::SnapshotInspectionDriftImpact;
use hepta_core::SnapshotInspectionHealth;
use hepta_core::SnapshotInspectionSection;
use hepta_core::SnapshotIssueSummary;
use hepta_core::SnapshotRestoreDomain;
use hepta_core::SnapshotRestoreDomainImpact;
use hepta_core::SnapshotRestoreMutationProfile;
use hepta_core::SnapshotRestorePreview;
use hepta_core::SnapshotRestoreReadiness;
use hepta_core::SnapshotRestoreSafety;
use hepta_core::TranscriptEntryKind;
use hepta_core::TranscriptSnapshotIntegrityReport;
use hepta_core::TranscriptSnapshotManifest;
use hepta_core::TranscriptSpan;

fn assert_memory_report_store<T: MemoryReportStore>() {}

fn session_record(session_id: &str, title: &str, last_intent: Option<&str>) -> SessionRecord {
    SessionRecord {
        session_id: SessionId(session_id.into()),
        agent_id: AgentId("builder".into()),
        title: title.into(),
        created_at_unix_ms: 10,
        last_active_unix_ms: 20,
        last_user_intent_summary: last_intent.map(str::to_string),
        archived_at_unix_ms: None,
    }
}

fn memory_record(id: &str, scope: MemoryScope, content: &str) -> MemoryRecord {
    MemoryRecord {
        id: id.into(),
        scope,
        content: content.into(),
    }
}

fn transcript_entry(
    session_id: &str,
    sequence: u64,
    kind: TranscriptEntryKind,
    content: &str,
) -> TranscriptEntry {
    TranscriptEntry {
        entry_id: format!("{}-{}", session_id, sequence),
        session_id: SessionId(session_id.into()),
        sequence,
        kind,
        role: Some(MessageRole::Assistant),
        content: content.into(),
        created_at_unix_ms: 100 + sequence,
        tool_name: None,
        correlation_id: None,
        summary_of_range: None,
    }
}

mod context_memory;
mod context_plane;
mod recall_context_core;
mod recall_context_helpers;
mod recall_context_quality;
mod recall_memory;
mod restore_preview;
mod search;
mod snapshot_core;
mod snapshot_inspection;
mod snapshot_integrity;
mod snapshot_inventory;
mod snapshot_restore;
mod store;
