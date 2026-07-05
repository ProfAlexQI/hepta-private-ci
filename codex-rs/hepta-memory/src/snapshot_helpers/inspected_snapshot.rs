use crate::InspectedStoreSnapshot;
use crate::StoreSnapshot;
use hepta_core::SnapshotAuditReport;
use hepta_core::SnapshotInspectionBundle;
use hepta_core::SnapshotInspectionDriftImpact;
use hepta_core::SnapshotInspectionDriftReport;
use hepta_core::SnapshotInspectionHealth;
use hepta_core::SnapshotIssueSummary;
use hepta_core::SnapshotRestoreDomain;
use hepta_core::SnapshotRestoreDomainImpact;
use hepta_core::SnapshotRestoreImpact;
use hepta_core::SnapshotRestoreMutationProfile;
use hepta_core::SnapshotRestorePreview;
use hepta_core::SnapshotRestoreReadiness;
use hepta_core::SnapshotRestoreSafety;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InspectedStoreSnapshotSerde {
    snapshot: StoreSnapshot,
    #[serde(default)]
    inspection: Option<SnapshotInspectionBundle>,
}

impl InspectedStoreSnapshot {
    pub fn from_snapshot(snapshot: StoreSnapshot) -> Self {
        let inspection = snapshot.inspection_bundle();

        Self {
            snapshot,
            inspection,
        }
    }

    pub fn audit_report(&self) -> SnapshotAuditReport {
        self.inspection.audit_report()
    }

    /// Returns the compact issue-count summary derived from the embedded
    /// inspection bundle.
    pub fn issue_summary(&self) -> SnapshotIssueSummary {
        self.inspection.issue_summary()
    }

    /// Computes the replace-style restore preview that would result from
    /// applying this inspected snapshot over `current`.
    pub fn restore_preview_against(&self, current: &StoreSnapshot) -> SnapshotRestorePreview {
        self.snapshot.restore_preview_against(current)
    }

    /// Returns the compact impact summary derived from the replace-style
    /// restore preview against `current`.
    pub fn restore_impact_against(&self, current: &StoreSnapshot) -> SnapshotRestoreImpact {
        self.snapshot.restore_impact_against(current)
    }

    /// Returns the payload-light readiness summary derived from the replace-
    /// style restore preview against `current`.
    pub fn restore_readiness_against(&self, current: &StoreSnapshot) -> SnapshotRestoreReadiness {
        self.snapshot.restore_readiness_against(current)
    }

    /// Returns the compact safety summary derived from the replace-style
    /// restore preview against `current`.
    pub fn restore_safety_against(&self, current: &StoreSnapshot) -> SnapshotRestoreSafety {
        self.snapshot.restore_safety_against(current)
    }

    /// Returns the compact domain-shape summary derived from the replace-style
    /// restore preview against `current`.
    pub fn restore_mutation_profile_against(
        &self,
        current: &StoreSnapshot,
    ) -> SnapshotRestoreMutationProfile {
        self.snapshot.restore_mutation_profile_against(current)
    }

    /// Returns `true` when applying this inspected snapshot over `current`
    /// would only add new records without updating or removing existing ones.
    pub fn restore_is_additive_only_against(&self, current: &StoreSnapshot) -> bool {
        self.snapshot.restore_is_additive_only_against(current)
    }

    /// Returns `true` when applying this inspected snapshot over `current`
    /// would update or remove existing records.
    pub fn restore_touches_existing_records_against(&self, current: &StoreSnapshot) -> bool {
        self.snapshot
            .restore_touches_existing_records_against(current)
    }

    /// Returns per-domain restore counts in stable domain order for automation
    /// that only needs the compact restore summary.
    pub fn restore_domain_impacts_against(
        &self,
        current: &StoreSnapshot,
    ) -> Vec<SnapshotRestoreDomainImpact> {
        self.snapshot.restore_domain_impacts_against(current)
    }

    /// Returns the restore domains that would change if this inspected
    /// snapshot were applied over `current`.
    pub fn restore_changed_domains_against(
        &self,
        current: &StoreSnapshot,
    ) -> Vec<SnapshotRestoreDomain> {
        self.snapshot.restore_changed_domains_against(current)
    }

    /// Returns `true` when the persisted inspection view still matches the
    /// full payload-bearing snapshot.
    pub fn inspection_matches_snapshot(&self) -> bool {
        self.inspection.matches_records_and_entries(
            &self.snapshot.sessions,
            &self.snapshot.memories,
            &self.snapshot.transcripts,
        )
    }

    /// Rebuilds the inspection bundle from the snapshot payload so callers can
    /// normalize envelopes loaded from external storage.
    pub fn normalized(&self) -> Self {
        Self::from_snapshot(self.snapshot.clone())
    }

    pub fn issue_count(&self) -> usize {
        self.inspection.issue_count()
    }

    /// Returns a section-level drift report describing whether the embedded
    /// inspection bundle is still aligned with the embedded snapshot payload.
    pub fn inspection_drift_report(&self) -> SnapshotInspectionDriftReport {
        self.snapshot.inspection_drift_report(&self.inspection)
    }

    /// Returns the compact domain-level drift impact for the embedded
    /// inspection bundle relative to the embedded snapshot payload.
    pub fn inspection_drift_impact(&self) -> SnapshotInspectionDriftImpact {
        self.inspection_drift_report().impact()
    }

    /// Returns the compact issue-plus-drift readiness summary for the
    /// embedded inspection bundle relative to the embedded snapshot payload.
    pub fn inspection_health(&self) -> SnapshotInspectionHealth {
        self.snapshot.inspection_health(&self.inspection)
    }

    pub fn is_clean(&self) -> bool {
        self.inspection.is_clean()
    }
}

impl<'de> Deserialize<'de> for InspectedStoreSnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serde_value = InspectedStoreSnapshotSerde::deserialize(deserializer)?;
        let inspection = serde_value
            .inspection
            .unwrap_or_else(|| serde_value.snapshot.inspection_bundle());

        Ok(Self {
            snapshot: serde_value.snapshot,
            inspection,
        })
    }
}
