use serde::Deserialize;
use serde::Serialize;

use super::super::preview::SnapshotRestorePreview;
use super::impact::SnapshotRestoreImpact;

const SNAPSHOT_RESTORE_DOMAIN_COUNT: usize = 3;

/// Compact restore-planning domain summary.
///
/// Unlike [`SnapshotRestoreImpact`], this keeps only the count of restore
/// domains that changed, how many of those domains are additive-only versus
/// touching existing records, and whether removals or integrity issues are
/// present. This is intended for low-blast-radius automation that needs to
/// quickly answer "how much rewrite pressure does this restore create?"
/// without carrying per-domain vectors or changed identifier lists.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestoreMutationProfile {
    pub changed_domain_count: usize,
    pub unchanged_domain_count: usize,
    pub addition_domain_count: usize,
    pub additive_only_domain_count: usize,
    pub existing_record_domain_count: usize,
    pub removal_domain_count: usize,
    pub current_issue_count: usize,
    pub incoming_issue_count: usize,
}

impl SnapshotRestoreMutationProfile {
    pub fn from_preview(preview: &SnapshotRestorePreview) -> Self {
        Self::from_impact(&preview.impact())
    }

    pub fn from_impact(impact: &SnapshotRestoreImpact) -> Self {
        let domain_impacts = &impact.domain_impacts;
        let changed_domain_count = impact.changed_domain_count();

        let domain_count = domain_impacts
            .len()
            .max(changed_domain_count)
            .max(SNAPSHOT_RESTORE_DOMAIN_COUNT);

        let count_or_fallback = |count: usize, fallback: bool| {
            if domain_impacts.is_empty() {
                usize::from(fallback)
            } else {
                count
            }
        };

        let addition_domain_count = count_or_fallback(
            domain_impacts
                .iter()
                .filter(|impact| impact.has_additions())
                .count(),
            impact.has_additions(),
        );
        let additive_only_domain_count = count_or_fallback(
            domain_impacts
                .iter()
                .filter(|impact| impact.has_changes() && impact.is_additive_only())
                .count(),
            impact.is_additive_only(),
        );
        let existing_record_domain_count = count_or_fallback(
            domain_impacts
                .iter()
                .filter(|impact| impact.touches_existing_records())
                .count(),
            impact.touches_existing_records(),
        );
        let removal_domain_count = count_or_fallback(
            domain_impacts
                .iter()
                .filter(|impact| impact.has_removals())
                .count(),
            impact.has_removals(),
        );

        Self {
            changed_domain_count,
            unchanged_domain_count: domain_count.saturating_sub(changed_domain_count),
            addition_domain_count,
            additive_only_domain_count,
            existing_record_domain_count,
            removal_domain_count,
            current_issue_count: impact.current_issue_count,
            incoming_issue_count: impact.incoming_issue_count,
        }
    }

    pub fn total_issue_count(&self) -> usize {
        self.current_issue_count + self.incoming_issue_count
    }

    pub fn has_changes(&self) -> bool {
        self.changed_domain_count > 0
    }

    pub fn has_additive_domains(&self) -> bool {
        self.additive_only_domain_count > 0
    }

    pub fn touches_existing_records(&self) -> bool {
        self.existing_record_domain_count > 0
    }

    pub fn has_removals(&self) -> bool {
        self.removal_domain_count > 0
    }

    pub fn is_additive_only(&self) -> bool {
        self.has_changes() && !self.touches_existing_records()
    }

    pub fn has_integrity_issues(&self) -> bool {
        self.total_issue_count() > 0
    }

    pub fn is_noop(&self) -> bool {
        !self.has_changes()
    }

    pub fn is_ready(&self) -> bool {
        !self.has_integrity_issues()
    }
}
