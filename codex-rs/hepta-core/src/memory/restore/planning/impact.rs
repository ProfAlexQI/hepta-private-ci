use serde::Deserialize;
use serde::Serialize;

use super::super::delta::RestoreDeltaCounts;
use super::super::domain::SnapshotRestoreDomain;
use super::super::domain::SnapshotRestoreDomainImpact;
use super::super::preview::SnapshotRestorePreview;
use super::mutation::SnapshotRestoreMutationProfile;
use super::readiness::SnapshotRestoreReadiness;
use super::safety::SnapshotRestoreSafety;

/// Compact impact summary derived from a full restore preview.
///
/// This keeps the aggregate counts, touched domains, and integrity posture in
/// one portable payload for CLI summaries and automation that do not need the
/// per-record identifier lists carried by [`SnapshotRestorePreview`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestoreImpact {
    pub change_totals: RestoreDeltaCounts,
    #[serde(default)]
    pub changed_domains: Vec<SnapshotRestoreDomain>,
    #[serde(default)]
    pub domain_impacts: Vec<SnapshotRestoreDomainImpact>,
    pub current_issue_count: usize,
    pub incoming_issue_count: usize,
}

impl SnapshotRestoreImpact {
    pub fn from_preview(preview: &SnapshotRestorePreview) -> Self {
        let domain_impacts = preview.domain_impacts();
        let changed_domains = preview.changed_domains();

        Self {
            change_totals: preview.change_totals(),
            changed_domains,
            domain_impacts,
            current_issue_count: preview.current_audit.issue_count(),
            incoming_issue_count: preview.incoming_audit.issue_count(),
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

    pub fn changed_domain_count(&self) -> usize {
        self.changed_domains.len()
    }

    pub fn total_issue_count(&self) -> usize {
        self.current_issue_count + self.incoming_issue_count
    }

    pub fn touches(&self, domain: SnapshotRestoreDomain) -> bool {
        self.changed_domains.contains(&domain)
    }

    pub fn impact_for(
        &self,
        domain: SnapshotRestoreDomain,
    ) -> Option<&SnapshotRestoreDomainImpact> {
        self.domain_impacts
            .iter()
            .find(|impact| impact.domain == domain)
    }

    pub fn has_integrity_issues(&self) -> bool {
        self.total_issue_count() > 0
    }

    pub fn is_noop(&self) -> bool {
        self.change_count() == 0
    }

    /// Returns a payload-light readiness summary derived from this impact.
    pub fn readiness(&self) -> SnapshotRestoreReadiness {
        SnapshotRestoreReadiness::from_impact(self)
    }

    /// Returns a compact safety summary derived from this impact.
    pub fn safety(&self) -> SnapshotRestoreSafety {
        SnapshotRestoreSafety::from_impact(self)
    }

    /// Returns a compact domain-shape summary derived from this impact.
    pub fn mutation_profile(&self) -> SnapshotRestoreMutationProfile {
        SnapshotRestoreMutationProfile::from_impact(self)
    }
}
