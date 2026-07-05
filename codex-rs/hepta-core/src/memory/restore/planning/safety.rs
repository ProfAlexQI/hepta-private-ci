use serde::Deserialize;
use serde::Serialize;

use super::super::delta::RestoreDeltaCounts;
use super::super::preview::SnapshotRestorePreview;
use super::impact::SnapshotRestoreImpact;
use super::readiness::SnapshotRestoreReadiness;

/// Compact restore-planning safety summary.
///
/// Unlike [`SnapshotRestoreReadiness`], this persists the derived gating
/// booleans that low-blast-radius automation often wants to inspect directly,
/// including whether the preview changes anything at all, whether it is purely
/// additive, whether it would touch existing records, and whether integrity
/// issues keep the plan from being ready.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestoreSafety {
    pub change_totals: RestoreDeltaCounts,
    pub changed_domain_count: usize,
    pub current_issue_count: usize,
    pub incoming_issue_count: usize,
    pub has_changes: bool,
    pub touches_existing_records: bool,
    pub additive_only: bool,
    pub has_integrity_issues: bool,
}

impl SnapshotRestoreSafety {
    pub fn from_preview(preview: &SnapshotRestorePreview) -> Self {
        Self::from_readiness(&preview.readiness())
    }

    pub fn from_impact(impact: &SnapshotRestoreImpact) -> Self {
        Self::from_readiness(&impact.readiness())
    }

    pub fn from_readiness(readiness: &SnapshotRestoreReadiness) -> Self {
        Self {
            change_totals: readiness.change_totals.clone(),
            changed_domain_count: readiness.changed_domain_count,
            current_issue_count: readiness.current_issue_count,
            incoming_issue_count: readiness.incoming_issue_count,
            has_changes: readiness.has_changes(),
            touches_existing_records: readiness.touches_existing_records(),
            additive_only: readiness.is_additive_only(),
            has_integrity_issues: readiness.has_integrity_issues(),
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

    pub fn total_issue_count(&self) -> usize {
        self.current_issue_count + self.incoming_issue_count
    }

    pub fn is_noop(&self) -> bool {
        !self.has_changes
    }

    pub fn is_ready(&self) -> bool {
        !self.has_integrity_issues
    }
}
