use serde::Deserialize;
use serde::Serialize;

use super::super::MemoryRecord;
use super::super::SessionRecord;
use super::super::TranscriptEntry;
use super::audit::SnapshotIssueSummary;
use super::drift::SnapshotInspectionDriftImpact;
use super::inspected::SnapshotInspectionBundle;

/// Compact snapshot-inspection readiness summary.
///
/// This combines the issue posture of an inspection bundle with its drift
/// impact relative to a payload-bearing snapshot so automation can answer
/// "is this inspection clean and aligned?" without carrying manifests,
/// integrity reports, or section-level mismatch names.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotInspectionHealth {
    pub issue_summary: SnapshotIssueSummary,
    pub drift_impact: SnapshotInspectionDriftImpact,
}

impl SnapshotInspectionHealth {
    pub fn from_bundle_and_records(
        inspection: &SnapshotInspectionBundle,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> Self {
        Self {
            issue_summary: inspection.issue_summary(),
            drift_impact: inspection
                .drift_report(sessions, memories, transcripts)
                .impact(),
        }
    }

    pub fn issue_count(&self) -> usize {
        self.issue_summary.total_issue_count
    }

    pub fn mismatch_count(&self) -> usize {
        self.drift_impact.mismatch_count
    }

    pub fn touches_memory(&self) -> bool {
        self.issue_summary.touches_memory() || self.drift_impact.touches_memory()
    }

    pub fn touches_transcripts(&self) -> bool {
        self.issue_summary.touches_transcripts() || self.drift_impact.touches_transcripts()
    }

    pub fn changed_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    pub fn has_issues(&self) -> bool {
        self.issue_summary.has_issues()
    }

    pub fn has_drift(&self) -> bool {
        !self.drift_impact.is_aligned()
    }

    pub fn inspection_aligned(&self) -> bool {
        self.drift_impact.is_aligned()
    }

    pub fn is_clean(&self) -> bool {
        self.issue_summary.is_clean()
    }

    pub fn is_ready(&self) -> bool {
        self.is_clean() && self.inspection_aligned()
    }
}
