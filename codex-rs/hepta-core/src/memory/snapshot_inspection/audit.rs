use serde::Deserialize;
use serde::Serialize;

use super::super::MemoryRecord;
use super::super::MemorySnapshotIntegrityReport;
use super::super::MemorySnapshotStats;
use super::super::SessionRecord;
use super::super::TranscriptEntry;
use super::super::TranscriptSnapshotIntegrityReport;
use super::super::TranscriptSnapshotStats;
use super::inspected::SnapshotInspectionBundle;

/// Combined audit view over portable session, memory, and transcript state.
///
/// This gives callers a single machine-readable contract for storage preflight,
/// export/import validation, and lightweight doctor checks without forcing them
/// to manually stitch together stats and integrity reports across layers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotAuditReport {
    pub memory_stats: MemorySnapshotStats,
    pub memory_integrity: MemorySnapshotIntegrityReport,
    pub transcript_stats: TranscriptSnapshotStats,
    pub transcript_integrity: TranscriptSnapshotIntegrityReport,
}

impl SnapshotAuditReport {
    pub fn from_records_and_entries(
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> Self {
        Self {
            memory_stats: MemorySnapshotStats::from_records(sessions, memories),
            memory_integrity: MemorySnapshotIntegrityReport::from_records(sessions, memories),
            transcript_stats: TranscriptSnapshotStats::from_entries(transcripts),
            transcript_integrity: TranscriptSnapshotIntegrityReport::from_entries(transcripts),
        }
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
    /// that does not need the full stats and integrity payloads.
    pub fn issue_summary(&self) -> SnapshotIssueSummary {
        SnapshotIssueSummary::from_audit_report(self)
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
        self.memory_stats.is_empty() && self.transcript_stats.is_empty()
    }
}

/// Compact issue-count summary derived from snapshot audit or inspection
/// payloads.
///
/// This lets automation carry the cross-domain integrity posture of a
/// snapshot without embedding the full stats, manifests, or integrity
/// reports.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotIssueSummary {
    pub memory_issue_count: usize,
    pub transcript_issue_count: usize,
    pub total_issue_count: usize,
    pub issue_domain_count: usize,
}

impl SnapshotIssueSummary {
    pub fn from_audit_report(report: &SnapshotAuditReport) -> Self {
        Self {
            memory_issue_count: report.memory_issue_count(),
            transcript_issue_count: report.transcript_issue_count(),
            total_issue_count: report.issue_count(),
            issue_domain_count: report.issue_domain_count(),
        }
    }

    pub fn from_inspection(inspection: &SnapshotInspectionBundle) -> Self {
        Self {
            memory_issue_count: inspection.memory_issue_count(),
            transcript_issue_count: inspection.transcript_issue_count(),
            total_issue_count: inspection.issue_count(),
            issue_domain_count: inspection.issue_domain_count(),
        }
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_issue_count > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_issue_count > 0
    }

    pub fn has_issues(&self) -> bool {
        self.total_issue_count > 0
    }

    pub fn is_clean(&self) -> bool {
        !self.has_issues()
    }
}
