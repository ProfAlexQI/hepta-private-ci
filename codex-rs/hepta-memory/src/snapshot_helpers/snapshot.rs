use crate::StoreSnapshot;
use hepta_core::MemorySnapshotIntegrityReport;
use hepta_core::MemorySnapshotManifest;
use hepta_core::MemorySnapshotStats;
use hepta_core::SessionAgentInventory;
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
use hepta_core::TranscriptSessionInventory;
use hepta_core::TranscriptSnapshotIntegrityReport;
use hepta_core::TranscriptSnapshotManifest;
use hepta_core::TranscriptSnapshotStats;

impl StoreSnapshot {
    pub fn stats(&self) -> MemorySnapshotStats {
        MemorySnapshotStats::from_records(&self.sessions, &self.memories)
    }

    pub fn transcript_stats(&self) -> TranscriptSnapshotStats {
        TranscriptSnapshotStats::from_entries(&self.transcripts)
    }

    pub fn manifest(&self) -> MemorySnapshotManifest {
        MemorySnapshotManifest::from_records(&self.sessions, &self.memories)
    }

    pub fn session_agent_inventory(&self) -> SessionAgentInventory {
        SessionAgentInventory::from_records(&self.sessions)
    }

    pub fn transcript_manifest(&self) -> TranscriptSnapshotManifest {
        TranscriptSnapshotManifest::from_entries(&self.transcripts)
    }

    pub fn transcript_session_inventory(&self) -> TranscriptSessionInventory {
        TranscriptSessionInventory::from_entries(&self.transcripts)
    }

    pub fn integrity_report(&self) -> MemorySnapshotIntegrityReport {
        MemorySnapshotIntegrityReport::from_records(&self.sessions, &self.memories)
    }

    pub fn transcript_integrity_report(&self) -> TranscriptSnapshotIntegrityReport {
        TranscriptSnapshotIntegrityReport::from_entries(&self.transcripts)
    }

    pub fn audit_report(&self) -> SnapshotAuditReport {
        SnapshotAuditReport::from_records_and_entries(
            &self.sessions,
            &self.memories,
            &self.transcripts,
        )
    }

    /// Builds the compact snapshot issue summary without carrying the full
    /// audit payload.
    pub fn issue_summary(&self) -> SnapshotIssueSummary {
        self.audit_report().issue_summary()
    }

    pub fn inspection_bundle(&self) -> SnapshotInspectionBundle {
        SnapshotInspectionBundle::from_records_and_entries(
            &self.sessions,
            &self.memories,
            &self.transcripts,
        )
    }

    /// Returns `true` when `inspection` still reflects this snapshot's
    /// portable session, memory, and transcript payloads.
    pub fn inspection_matches(&self, inspection: &SnapshotInspectionBundle) -> bool {
        inspection.matches_records_and_entries(&self.sessions, &self.memories, &self.transcripts)
    }

    /// Computes section-level drift for `inspection` relative to this snapshot.
    pub fn inspection_drift_report(
        &self,
        inspection: &SnapshotInspectionBundle,
    ) -> SnapshotInspectionDriftReport {
        inspection.drift_report(&self.sessions, &self.memories, &self.transcripts)
    }

    /// Computes the compact domain-level drift impact for `inspection`
    /// relative to this snapshot.
    pub fn inspection_drift_impact(
        &self,
        inspection: &SnapshotInspectionBundle,
    ) -> SnapshotInspectionDriftImpact {
        self.inspection_drift_report(inspection).impact()
    }

    /// Computes the compact issue-plus-drift readiness summary for
    /// `inspection` relative to this snapshot.
    pub fn inspection_health(
        &self,
        inspection: &SnapshotInspectionBundle,
    ) -> SnapshotInspectionHealth {
        inspection.health_against_records(&self.sessions, &self.memories, &self.transcripts)
    }

    /// Computes the replace-style restore preview that would result from
    /// applying this snapshot over `current`.
    pub fn restore_preview_against(&self, current: &StoreSnapshot) -> SnapshotRestorePreview {
        SnapshotRestorePreview::from_records_and_entries(
            &current.sessions,
            &current.memories,
            &current.transcripts,
            &self.sessions,
            &self.memories,
            &self.transcripts,
        )
    }

    /// Returns the compact impact summary derived from the replace-style
    /// restore preview against `current`.
    pub fn restore_impact_against(&self, current: &StoreSnapshot) -> SnapshotRestoreImpact {
        self.restore_preview_against(current).impact()
    }

    /// Returns the payload-light readiness summary derived from the replace-
    /// style restore preview against `current`.
    pub fn restore_readiness_against(&self, current: &StoreSnapshot) -> SnapshotRestoreReadiness {
        self.restore_preview_against(current).readiness()
    }

    /// Returns the compact safety summary derived from the replace-style
    /// restore preview against `current`.
    pub fn restore_safety_against(&self, current: &StoreSnapshot) -> SnapshotRestoreSafety {
        self.restore_preview_against(current).safety()
    }

    /// Returns the compact domain-shape summary derived from the replace-style
    /// restore preview against `current`.
    pub fn restore_mutation_profile_against(
        &self,
        current: &StoreSnapshot,
    ) -> SnapshotRestoreMutationProfile {
        self.restore_preview_against(current).mutation_profile()
    }

    /// Returns `true` when applying this snapshot over `current` would only
    /// add new records without updating or removing existing ones.
    pub fn restore_is_additive_only_against(&self, current: &StoreSnapshot) -> bool {
        self.restore_readiness_against(current).is_additive_only()
    }

    /// Returns `true` when applying this snapshot over `current` would update
    /// or remove existing records.
    pub fn restore_touches_existing_records_against(&self, current: &StoreSnapshot) -> bool {
        self.restore_readiness_against(current)
            .touches_existing_records()
    }

    /// Returns per-domain restore counts in stable domain order for automation
    /// that needs a payload lighter than the full preview.
    pub fn restore_domain_impacts_against(
        &self,
        current: &StoreSnapshot,
    ) -> Vec<SnapshotRestoreDomainImpact> {
        self.restore_preview_against(current).domain_impacts()
    }

    /// Returns the restore domains that would change if this snapshot were
    /// applied over `current`.
    pub fn restore_changed_domains_against(
        &self,
        current: &StoreSnapshot,
    ) -> Vec<SnapshotRestoreDomain> {
        self.restore_preview_against(current).changed_domains()
    }
}
