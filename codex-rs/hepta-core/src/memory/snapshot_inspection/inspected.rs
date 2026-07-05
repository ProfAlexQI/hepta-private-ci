use serde::Deserialize;
use serde::Serialize;

use super::super::MemoryRecord;
use super::super::MemorySnapshotIntegrityReport;
use super::super::MemorySnapshotManifest;
use super::super::SessionRecord;
use super::super::TranscriptEntry;
use super::super::TranscriptSnapshotIntegrityReport;
use super::super::TranscriptSnapshotManifest;
use super::audit::SnapshotAuditReport;
use super::audit::SnapshotIssueSummary;
use super::drift::SnapshotInspectionDriftImpact;
use super::drift::SnapshotInspectionDriftReport;
use super::health::SnapshotInspectionHealth;

/// Compact inspection bundle for portable session, memory, and transcript
/// snapshots.
///
/// Unlike [`SnapshotAuditReport`], this envelope keeps the manifest views for
/// both storage domains alongside their integrity reports so tooling can show a
/// stable inventory without loading the full payload-bearing snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotInspectionBundle {
    pub memory_manifest: MemorySnapshotManifest,
    pub memory_integrity: MemorySnapshotIntegrityReport,
    pub transcript_manifest: TranscriptSnapshotManifest,
    pub transcript_integrity: TranscriptSnapshotIntegrityReport,
}

impl SnapshotInspectionBundle {
    pub fn from_records_and_entries(
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> Self {
        Self {
            memory_manifest: MemorySnapshotManifest::from_records(sessions, memories),
            memory_integrity: MemorySnapshotIntegrityReport::from_records(sessions, memories),
            transcript_manifest: TranscriptSnapshotManifest::from_entries(transcripts),
            transcript_integrity: TranscriptSnapshotIntegrityReport::from_entries(transcripts),
        }
    }

    /// Reconstructs the lighter-weight audit view from an inspection bundle.
    ///
    /// This lets export/import tooling keep a single manifest-oriented payload
    /// on disk while still deriving the aggregate health summary used by doctor
    /// checks and automation.
    pub fn audit_report(&self) -> SnapshotAuditReport {
        SnapshotAuditReport {
            memory_stats: self.memory_manifest.stats.clone(),
            memory_integrity: self.memory_integrity.clone(),
            transcript_stats: self.transcript_manifest.stats.clone(),
            transcript_integrity: self.transcript_integrity.clone(),
        }
    }

    /// Returns `true` when this inspection bundle still reflects the supplied
    /// portable snapshot payloads.
    pub fn matches_records_and_entries(
        &self,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> bool {
        self == &Self::from_records_and_entries(sessions, memories, transcripts)
    }

    /// Returns a section-level drift report describing which inspection views,
    /// if any, no longer match the supplied portable snapshot payloads.
    pub fn drift_report(
        &self,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> SnapshotInspectionDriftReport {
        SnapshotInspectionDriftReport::from_bundle_and_records(
            self,
            sessions,
            memories,
            transcripts,
        )
    }

    /// Returns a compact domain-level drift summary for this inspection
    /// bundle relative to the supplied portable snapshot payloads.
    pub fn drift_impact_against_records(
        &self,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> SnapshotInspectionDriftImpact {
        self.drift_report(sessions, memories, transcripts).impact()
    }

    pub fn memory_issue_count(&self) -> usize {
        self.memory_integrity.issue_count()
    }

    pub fn transcript_issue_count(&self) -> usize {
        self.transcript_integrity.issue_count()
    }

    pub fn issue_count(&self) -> usize {
        self.memory_issue_count() + self.transcript_issue_count()
    }

    pub fn issue_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    /// Returns a compact machine-readable issue-count summary for automation
    /// that does not need the full manifest and integrity payloads.
    pub fn issue_summary(&self) -> SnapshotIssueSummary {
        SnapshotIssueSummary::from_inspection(self)
    }

    /// Returns a compact readiness summary that combines issue counts with
    /// inspection-drift impact relative to the supplied snapshot payloads.
    pub fn health_against_records(
        &self,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> SnapshotInspectionHealth {
        SnapshotInspectionHealth::from_bundle_and_records(self, sessions, memories, transcripts)
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_issue_count() > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_issue_count() > 0
    }

    pub fn is_clean(&self) -> bool {
        self.issue_count() == 0
    }

    pub fn is_empty(&self) -> bool {
        self.memory_manifest.is_empty() && self.transcript_manifest.is_empty()
    }
}
