use serde::Deserialize;
use serde::Serialize;

use super::super::delta::RestoreDeltaCounts;
use super::super::preview::SnapshotRestorePreview;
use super::impact::SnapshotRestoreImpact;
use super::safety::SnapshotRestoreSafety;

/// Compact restore-planning readiness summary.
///
/// Unlike [`SnapshotRestoreImpact`], this keeps only aggregate change counts,
/// the number of changed domains, and the integrity posture of the current and
/// incoming snapshots. It is intended for automation that needs to answer
/// "is this restore plan clean and how much does it change?" without carrying
/// per-domain vectors or record identifiers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestoreReadiness {
    pub change_totals: RestoreDeltaCounts,
    pub changed_domain_count: usize,
    pub current_issue_count: usize,
    pub incoming_issue_count: usize,
}

impl SnapshotRestoreReadiness {
    pub fn from_preview(preview: &SnapshotRestorePreview) -> Self {
        Self {
            change_totals: preview.change_totals(),
            changed_domain_count: preview.changed_domain_count(),
            current_issue_count: preview.current_audit.issue_count(),
            incoming_issue_count: preview.incoming_audit.issue_count(),
        }
    }

    pub fn from_impact(impact: &SnapshotRestoreImpact) -> Self {
        Self {
            change_totals: impact.change_totals.clone(),
            changed_domain_count: impact.changed_domain_count(),
            current_issue_count: impact.current_issue_count,
            incoming_issue_count: impact.incoming_issue_count,
        }
    }

    pub fn change_count(&self) -> usize {
        self.change_totals.change_count()
    }

    pub fn has_additions(&self) -> bool {
        self.change_totals.has_additions()
    }

    pub fn has_removals(&self) -> bool {
        self.change_totals.has_removals()
    }

    pub fn has_updates(&self) -> bool {
        self.change_totals.has_updates()
    }

    pub fn touches_existing_records(&self) -> bool {
        self.change_totals.touches_existing_records()
    }

    pub fn is_additive_only(&self) -> bool {
        self.change_totals.is_additive_only()
    }

    pub fn total_issue_count(&self) -> usize {
        self.current_issue_count + self.incoming_issue_count
    }

    pub fn has_changes(&self) -> bool {
        !self.is_noop()
    }

    pub fn has_integrity_issues(&self) -> bool {
        self.total_issue_count() > 0
    }

    pub fn is_noop(&self) -> bool {
        self.change_count() == 0
    }

    pub fn is_ready(&self) -> bool {
        !self.has_integrity_issues()
    }

    /// Returns a compact safety summary derived from this readiness payload.
    pub fn safety(&self) -> SnapshotRestoreSafety {
        SnapshotRestoreSafety::from_readiness(self)
    }
}
