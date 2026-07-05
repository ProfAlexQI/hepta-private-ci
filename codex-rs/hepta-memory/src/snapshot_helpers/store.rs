use crate::InMemoryStore;
use crate::InspectedStoreSnapshot;
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

impl InMemoryStore {
    pub fn snapshot(&self) -> Result<StoreSnapshot, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("memory store mutex poisoned".into()))?;
        Ok(StoreSnapshot {
            sessions: guard.sessions.clone(),
            memories: guard.memories.clone(),
            transcripts: guard.transcripts.clone(),
        })
    }

    pub fn inspected_snapshot(&self) -> Result<InspectedStoreSnapshot, hepta_core::MemoryError> {
        Ok(InspectedStoreSnapshot::from_snapshot(self.snapshot()?))
    }

    pub fn snapshot_stats(&self) -> Result<MemorySnapshotStats, hepta_core::MemoryError> {
        Ok(self.snapshot()?.stats())
    }

    pub fn snapshot_manifest(&self) -> Result<MemorySnapshotManifest, hepta_core::MemoryError> {
        Ok(self.snapshot()?.manifest())
    }

    pub fn session_agent_inventory(
        &self,
    ) -> Result<SessionAgentInventory, hepta_core::MemoryError> {
        Ok(self.snapshot()?.session_agent_inventory())
    }

    pub fn transcript_snapshot_stats(
        &self,
    ) -> Result<TranscriptSnapshotStats, hepta_core::MemoryError> {
        Ok(self.snapshot()?.transcript_stats())
    }

    pub fn transcript_snapshot_manifest(
        &self,
    ) -> Result<TranscriptSnapshotManifest, hepta_core::MemoryError> {
        Ok(self.snapshot()?.transcript_manifest())
    }

    pub fn transcript_session_inventory(
        &self,
    ) -> Result<TranscriptSessionInventory, hepta_core::MemoryError> {
        Ok(self.snapshot()?.transcript_session_inventory())
    }

    pub fn snapshot_integrity_report(
        &self,
    ) -> Result<MemorySnapshotIntegrityReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.integrity_report())
    }

    pub fn transcript_snapshot_integrity_report(
        &self,
    ) -> Result<TranscriptSnapshotIntegrityReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.transcript_integrity_report())
    }

    pub fn snapshot_audit_report(&self) -> Result<SnapshotAuditReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.audit_report())
    }

    pub fn snapshot_issue_summary(&self) -> Result<SnapshotIssueSummary, hepta_core::MemoryError> {
        Ok(self.snapshot()?.issue_summary())
    }

    pub fn snapshot_inspection_bundle(
        &self,
    ) -> Result<SnapshotInspectionBundle, hepta_core::MemoryError> {
        Ok(self.snapshot()?.inspection_bundle())
    }

    pub fn snapshot_inspection_matches(
        &self,
        inspection: &SnapshotInspectionBundle,
    ) -> Result<bool, hepta_core::MemoryError> {
        Ok(self.snapshot()?.inspection_matches(inspection))
    }

    pub fn snapshot_inspection_drift_report(
        &self,
        inspection: &SnapshotInspectionBundle,
    ) -> Result<SnapshotInspectionDriftReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.inspection_drift_report(inspection))
    }

    pub fn snapshot_inspection_drift_impact(
        &self,
        inspection: &SnapshotInspectionBundle,
    ) -> Result<SnapshotInspectionDriftImpact, hepta_core::MemoryError> {
        Ok(self.snapshot()?.inspection_drift_impact(inspection))
    }

    pub fn snapshot_inspection_health(
        &self,
        inspection: &SnapshotInspectionBundle,
    ) -> Result<SnapshotInspectionHealth, hepta_core::MemoryError> {
        Ok(self.snapshot()?.inspection_health(inspection))
    }

    pub fn preview_restore(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<SnapshotRestorePreview, hepta_core::MemoryError> {
        Ok(snapshot.restore_preview_against(&self.snapshot()?))
    }

    pub fn preview_restore_impact(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<SnapshotRestoreImpact, hepta_core::MemoryError> {
        Ok(self.preview_restore(snapshot)?.impact())
    }

    pub fn preview_restore_readiness(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<SnapshotRestoreReadiness, hepta_core::MemoryError> {
        Ok(self.preview_restore(snapshot)?.readiness())
    }

    pub fn preview_restore_safety(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<SnapshotRestoreSafety, hepta_core::MemoryError> {
        Ok(self.preview_restore(snapshot)?.safety())
    }

    pub fn preview_restore_mutation_profile(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<SnapshotRestoreMutationProfile, hepta_core::MemoryError> {
        Ok(self.preview_restore(snapshot)?.mutation_profile())
    }

    pub fn preview_restore_is_additive_only(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<bool, hepta_core::MemoryError> {
        Ok(self.preview_restore_readiness(snapshot)?.is_additive_only())
    }

    pub fn preview_restore_touches_existing_records(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<bool, hepta_core::MemoryError> {
        Ok(self
            .preview_restore_readiness(snapshot)?
            .touches_existing_records())
    }

    pub fn preview_restore_domain_impacts(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<Vec<SnapshotRestoreDomainImpact>, hepta_core::MemoryError> {
        Ok(self.preview_restore(snapshot)?.domain_impacts())
    }

    pub fn preview_restore_changed_domains(
        &self,
        snapshot: &StoreSnapshot,
    ) -> Result<Vec<SnapshotRestoreDomain>, hepta_core::MemoryError> {
        Ok(self.preview_restore(snapshot)?.changed_domains())
    }
}
