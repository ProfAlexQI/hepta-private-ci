//! In-memory reference store for Hepta session, memory, and transcript
//! contracts.
//!
//! The crate intentionally stays lightweight and storage-agnostic. Beyond the
//! async store traits from `hepta-core`, it exposes snapshot helpers that make
//! contract testing, doctor-style inspection, and restore preview automation
//! easy to exercise without a durable backend. Recall callers can also derive
//! compact per-source bundle counts through `hepta_core::ContextRecallSourceCounts`,
//! the pre-limit `hepta_core::ContextRecallAvailability` summary when they only
//! need source availability counts, a payload-light
//! `hepta_core::ContextRecallReport`, or the availability-aware
//! `hepta_core::ContextRecallInspection` when they need to detect clipped
//! recent-window or query results and quantify omitted items without diffing
//! counts by hand, including the compact
//! `hepta_core::ContextRecallOmissionCounts` summary when automation only
//! needs omission totals by source, plus `hepta_core::ContextRecallLimitPressure`
//! when they only need source-level truncation flags and omitted-count totals.
//! When callers need those pre-limit recall counts split the same way returned
//! recall payloads are split, they can use
//! `hepta_core::ContextRecallSourceAvailability` to keep durable-memory and
//! session-summary match counts separate.
//! When callers only need the shape of transcript evidence behind a recall
//! result, they can use the compact
//! `hepta_core::ContextRecallTranscriptProvenanceSummary` instead of carrying
//! the individual transcript span refs from a full inspection payload.
//! Snapshot-inspection callers can likewise
//! collapse section-level drift into `hepta_core::SnapshotInspectionDriftImpact`
//! when they only need memory-vs-transcript drift counts, and both
//! `hepta_core::SnapshotAuditReport` plus `hepta_core::SnapshotInspectionBundle`
//! now expose domain-level issue counters for payload-light health summaries,
//! including the compact `hepta_core::SnapshotIssueSummary` when callers only
//! need per-domain and total issue counts without the full audit payload.
//! When automation also needs to know whether a persisted inspection bundle is
//! still aligned with the full payload-bearing snapshot, it can use the compact
//! `hepta_core::SnapshotInspectionHealth` summary instead of carrying separate
//! issue and drift payloads.
//! Restore-preview callers can likewise collapse replace-style planning into
//! `hepta_core::SnapshotRestoreReadiness` when they only need aggregate change
//! counts, changed-domain totals, and integrity posture without carrying
//! per-domain vectors or changed identifier lists.
//! When automation also wants those restore gates materialized as booleans in a
//! portable payload, it can use `hepta_core::SnapshotRestoreSafety` instead of
//! recomputing readiness, additive-only, and existing-record touch checks from
//! the restore counts on its own.
//! When callers also want a compact count of how many restore domains are
//! additive-only versus rewriting existing records, they can use
//! `hepta_core::SnapshotRestoreMutationProfile`.
//! Low-blast-radius automation can further use the additive-only helper layer
//! on restore impact and readiness payloads, including
//! `RestoreDeltaCounts::is_additive_only()` and
//! `RestoreDeltaCounts::touches_existing_records()`, plus the matching
//! snapshot and store helpers in this crate when callers want to quickly gate
//! whether a preview only adds new records or would rewrite existing ones.
//! Plain memory and transcript query callers can likewise collapse their
//! report envelopes into `hepta_core::QueryReportCoverage` or
//! `hepta_core::QueryReportLimitPressure` when they only need returned-vs-
//! matched counts or omission pressure without the full hit payload.
//! The reference store also treats `ContextRecallRequest::allow_cross_session`
//! as an advisory flag for memory
//! scope only: transcript hits stay session-scoped, and the portable
//! `MemoryRecord` contract does not carry session ownership metadata that would
//! let this crate filter memory hits differently.

use std::sync::{Arc, Mutex};

use hepta_core::{
    ContextBudget, ContextRecallAvailability, ContextRecallBundle, ContextRecallCoverage,
    ContextRecallInspection, ContextRecallLimitPressure, ContextRecallOmissionCounts,
    ContextRecallReport, ContextRecallRequest, ContextRecallSourceAvailability,
    ContextRecallTranscriptProvenanceSummary, MemoryQuery, MemoryQueryReport, MemoryRecord,
    MemoryReportStore, MemoryScope, MemorySnapshotIntegrityReport, MemorySnapshotManifest,
    MemorySnapshotStats, MemoryStore, QueryReportCoverage, QueryReportLimitPressure,
    SessionAgentInventory, SessionId, SessionRecord, SessionStore, SnapshotAuditReport,
    SnapshotInspectionBundle, SnapshotInspectionDriftImpact, SnapshotInspectionDriftReport,
    SnapshotInspectionHealth, SnapshotIssueSummary, SnapshotRestoreDomain,
    SnapshotRestoreDomainImpact, SnapshotRestoreImpact, SnapshotRestoreMutationProfile,
    SnapshotRestorePreview, SnapshotRestoreReadiness, SnapshotRestoreSafety, TranscriptEntry,
    TranscriptQuery, TranscriptQueryReport, TranscriptSessionInventory,
    TranscriptSnapshotIntegrityReport, TranscriptSnapshotManifest, TranscriptSnapshotStats,
    TranscriptSpan, TranscriptStore,
};
use serde::{Deserialize, Serialize};

/// Small non-durable store for local development, tests, and snapshot-backed
/// runtime state.
#[derive(Default, Clone)]
pub struct InMemoryStore {
    state: Arc<Mutex<StoreState>>,
}

#[derive(Default, Serialize, Deserialize)]
struct StoreState {
    sessions: Vec<SessionRecord>,
    memories: Vec<MemoryRecord>,
    transcripts: Vec<TranscriptEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreSnapshot {
    pub sessions: Vec<SessionRecord>,
    pub memories: Vec<MemoryRecord>,
    #[serde(default)]
    pub transcripts: Vec<TranscriptEntry>,
}

/// Portable export-ready snapshot envelope that pairs full payloads with the
/// lighter-weight inspection bundle derived from the same data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InspectedStoreSnapshot {
    pub snapshot: StoreSnapshot,
    pub inspection: SnapshotInspectionBundle,
}

#[derive(Debug, Deserialize)]
struct InspectedStoreSnapshotSerde {
    snapshot: StoreSnapshot,
    #[serde(default)]
    inspection: Option<SnapshotInspectionBundle>,
}

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

    /// Builds the portable memory query report directly from the snapshot.
    pub fn search_report(&self, query: &MemoryQuery) -> MemoryQueryReport {
        let matched = self
            .memories
            .iter()
            .filter(|record| record.content.contains(&query.text))
            .cloned()
            .collect::<Vec<_>>();
        let matched_count = matched.len();
        let mut hits = matched;
        hits.truncate(query.limit);

        MemoryQueryReport::from_hits(query.clone(), matched_count, hits)
    }

    /// Builds the compact returned-vs-matched memory query coverage summary.
    pub fn search_coverage(&self, query: &MemoryQuery) -> QueryReportCoverage {
        self.search_report(query).coverage()
    }

    /// Builds the compact omission-focused memory query pressure summary.
    pub fn search_limit_pressure(&self, query: &MemoryQuery) -> QueryReportLimitPressure {
        self.search_report(query).limit_pressure()
    }

    /// Builds the portable transcript query report directly from the snapshot.
    pub fn transcript_search_report(&self, query: &TranscriptQuery) -> TranscriptQueryReport {
        let matched = self
            .transcripts
            .iter()
            .filter(|entry| entry.matches_query(query))
            .cloned()
            .map(TranscriptSpan::from_entry)
            .collect::<Vec<_>>();
        let matched_count = matched.len();
        let mut hits = matched;
        hits.truncate(query.limit);

        TranscriptQueryReport::from_hits(query.clone(), matched_count, hits)
    }

    /// Builds the compact returned-vs-matched transcript query coverage
    /// summary.
    pub fn transcript_search_coverage(&self, query: &TranscriptQuery) -> QueryReportCoverage {
        self.transcript_search_report(query).coverage()
    }

    /// Builds the compact omission-focused transcript query pressure summary.
    pub fn transcript_search_limit_pressure(
        &self,
        query: &TranscriptQuery,
    ) -> QueryReportLimitPressure {
        self.transcript_search_report(query).limit_pressure()
    }

    fn recall_context_parts(
        &self,
        request: &ContextRecallRequest,
    ) -> (
        ContextRecallBundle,
        ContextRecallAvailability,
        ContextRecallSourceAvailability,
    ) {
        let mut recent_entries = self
            .transcripts
            .iter()
            .filter(|entry| entry.session_id == request.session_id)
            .cloned()
            .collect::<Vec<_>>();
        recent_entries.sort_by_key(|entry| entry.sequence);
        let total_recent_entry_count = recent_entries.len();
        if recent_entries.len() > request.recent_window_limit {
            recent_entries =
                recent_entries.split_off(recent_entries.len() - request.recent_window_limit);
        }

        let transcript_query = request.transcript_query();
        let (total_transcript_match_count, transcript_hits) = if request.has_query_text() {
            let mut hits = self
                .transcripts
                .iter()
                .filter(|entry| entry.matches_query(&transcript_query))
                .cloned()
                .map(TranscriptSpan::from_entry)
                .collect::<Vec<_>>();
            let matched_count = hits.len();
            hits.truncate(transcript_query.limit);
            (matched_count, hits)
        } else {
            (0, Vec::new())
        };

        let memory_query = request.memory_query();
        let memory_hits = self
            .memories
            .iter()
            .filter(|record| record.content.contains(&memory_query.text))
            .cloned()
            .collect::<Vec<_>>();
        let total_memory_match_count = memory_hits.len();
        let total_durable_memory_match_count = memory_hits
            .iter()
            .filter(|record| record.scope == MemoryScope::LongTerm)
            .count();
        let total_summary_memory_match_count = memory_hits
            .iter()
            .filter(|record| record.scope == MemoryScope::Session)
            .count();
        let mut limited_memory_hits = memory_hits;
        limited_memory_hits.truncate(memory_query.limit);

        let durable_memory_hits = limited_memory_hits
            .iter()
            .filter(|record| record.scope == MemoryScope::LongTerm)
            .cloned()
            .collect();
        let summary_hits = limited_memory_hits
            .into_iter()
            .filter(|record| record.scope == MemoryScope::Session)
            .collect();

        (
            ContextRecallBundle {
                request: request.clone(),
                recent_entries,
                transcript_hits,
                durable_memory_hits,
                summary_hits,
                active_topic_sessions: vec![],
                active_neurons: Vec::new(),
                budget: ContextBudget::from_request(request),
                ranked_items: Vec::new(),
                omitted_by_budget: 0,
                truncated: total_transcript_match_count > transcript_query.limit
                    || total_memory_match_count > memory_query.limit,
            },
            ContextRecallAvailability {
                total_recent_entry_count,
                total_transcript_match_count,
                total_memory_match_count,
            },
            ContextRecallSourceAvailability {
                recent_entry_count: total_recent_entry_count,
                transcript_match_count: total_transcript_match_count,
                durable_memory_match_count: total_durable_memory_match_count,
                summary_memory_match_count: total_summary_memory_match_count,
            },
        )
    }

    /// Builds a portable reference recall bundle directly from the snapshot.
    ///
    /// This mirrors the lightweight in-memory search semantics used by the
    /// store: transcript hits are session-scoped and query-driven, while memory
    /// hits use simple substring matching across the snapshot's memory records.
    pub fn recall_context(&self, request: &ContextRecallRequest) -> ContextRecallBundle {
        self.recall_context_parts(request).0
    }

    /// Builds the compact pre-limit recall availability summary for
    /// `request` without carrying returned item payloads.
    pub fn recall_context_availability(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallAvailability {
        self.recall_context_parts(request).1
    }

    /// Builds the compact pre-limit recall availability summary with
    /// durable-memory and session-summary matches split into separate counts.
    pub fn recall_context_source_availability(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallSourceAvailability {
        self.recall_context_parts(request).2
    }

    /// Builds the payload-light recall report for `request` without embedding
    /// the full transcript and memory payloads.
    pub fn recall_context_report(&self, request: &ContextRecallRequest) -> ContextRecallReport {
        self.recall_context(request).report()
    }

    /// Builds a payload-light recall inspection view that includes pre-limit
    /// availability counts for recent entries, transcript matches, and memory
    /// matches.
    pub fn recall_context_inspection(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallInspection {
        let (bundle, availability, _) = self.recall_context_parts(request);

        bundle.inspection(availability)
    }

    /// Builds the compact transcript-provenance summary for `request`
    /// without carrying the individual transcript span refs.
    pub fn recall_context_transcript_provenance_summary(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallTranscriptProvenanceSummary {
        self.recall_context_inspection(request)
            .transcript_provenance_summary()
    }

    /// Builds a payload-light recall coverage summary for `request`.
    pub fn recall_context_coverage(&self, request: &ContextRecallRequest) -> ContextRecallCoverage {
        self.recall_context_inspection(request).coverage()
    }

    /// Builds a compact omission summary for recall sources and totals.
    pub fn recall_context_omission_counts(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallOmissionCounts {
        self.recall_context_coverage(request).omission_counts()
    }

    /// Builds a compact limit-pressure summary for recall sources and totals.
    pub fn recall_context_limit_pressure(
        &self,
        request: &ContextRecallRequest,
    ) -> ContextRecallLimitPressure {
        self.recall_context_coverage(request).limit_pressure()
    }
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

impl InMemoryStore {
    pub fn list_sessions(&self) -> Result<Vec<SessionRecord>, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("session store mutex poisoned".into()))?;
        Ok(guard.sessions.clone())
    }

    pub fn list_memories(&self) -> Result<Vec<MemoryRecord>, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("memory store mutex poisoned".into()))?;
        Ok(guard.memories.clone())
    }

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

    pub fn recall_context(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallBundle, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context(&request))
    }

    pub fn recall_context_availability(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallAvailability, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_availability(&request))
    }

    pub fn recall_context_report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallReport, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_report(&request))
    }

    pub fn recall_context_source_availability(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallSourceAvailability, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_source_availability(&request))
    }

    pub fn recall_context_inspection(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallInspection, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_inspection(&request))
    }

    pub fn recall_context_transcript_provenance_summary(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallTranscriptProvenanceSummary, hepta_core::MemoryError> {
        Ok(self
            .snapshot()?
            .recall_context_transcript_provenance_summary(&request))
    }

    pub fn recall_context_coverage(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallCoverage, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_coverage(&request))
    }

    pub fn recall_context_omission_counts(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallOmissionCounts, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_omission_counts(&request))
    }

    pub fn recall_context_limit_pressure(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallLimitPressure, hepta_core::MemoryError> {
        Ok(self.snapshot()?.recall_context_limit_pressure(&request))
    }

    pub fn search_report(
        &self,
        query: MemoryQuery,
    ) -> Result<MemoryQueryReport, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("memory store mutex poisoned".into()))?;
        let matched = guard
            .memories
            .iter()
            .filter(|record| record.content.contains(&query.text))
            .cloned()
            .collect::<Vec<_>>();
        let matched_count = matched.len();
        let mut hits = matched;
        hits.truncate(query.limit);

        Ok(MemoryQueryReport::from_hits(query, matched_count, hits))
    }

    pub fn search_coverage(
        &self,
        query: MemoryQuery,
    ) -> Result<QueryReportCoverage, hepta_core::MemoryError> {
        Ok(self.search_report(query)?.coverage())
    }

    pub fn search_limit_pressure(
        &self,
        query: MemoryQuery,
    ) -> Result<QueryReportLimitPressure, hepta_core::MemoryError> {
        Ok(self.search_report(query)?.limit_pressure())
    }

    pub fn list_transcript_entries(&self) -> Result<Vec<TranscriptEntry>, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("transcript store mutex poisoned".into()))?;
        Ok(guard.transcripts.clone())
    }

    pub fn append_transcript_sync(
        &self,
        entry: TranscriptEntry,
    ) -> Result<(), hepta_core::MemoryError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("transcript store mutex poisoned".into()))?;
        guard.transcripts.push(entry);
        Ok(())
    }

    pub fn transcript_search_report(
        &self,
        query: TranscriptQuery,
    ) -> Result<TranscriptQueryReport, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("transcript store mutex poisoned".into()))?;
        let matched = guard
            .transcripts
            .iter()
            .filter(|entry| entry.matches_query(&query))
            .cloned()
            .map(TranscriptSpan::from_entry)
            .collect::<Vec<_>>();
        let matched_count = matched.len();
        let mut hits = matched;
        hits.truncate(query.limit);

        Ok(TranscriptQueryReport::from_hits(query, matched_count, hits))
    }

    pub fn transcript_search_coverage(
        &self,
        query: TranscriptQuery,
    ) -> Result<QueryReportCoverage, hepta_core::MemoryError> {
        Ok(self.transcript_search_report(query)?.coverage())
    }

    pub fn transcript_search_limit_pressure(
        &self,
        query: TranscriptQuery,
    ) -> Result<QueryReportLimitPressure, hepta_core::MemoryError> {
        Ok(self.transcript_search_report(query)?.limit_pressure())
    }

    pub fn restore(&self, snapshot: StoreSnapshot) -> Result<(), hepta_core::MemoryError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("memory store mutex poisoned".into()))?;
        guard.sessions = snapshot.sessions;
        guard.memories = snapshot.memories;
        guard.transcripts = snapshot.transcripts;
        Ok(())
    }

    pub fn upsert_session_sync(
        &self,
        record: SessionRecord,
    ) -> Result<(), hepta_core::MemoryError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("session store mutex poisoned".into()))?;
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|existing| existing.session_id == record.session_id)
        {
            *existing = record;
        } else {
            guard.sessions.push(record);
        }
        Ok(())
    }

    pub fn remove_session_sync(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<SessionRecord>, hepta_core::MemoryError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("session store mutex poisoned".into()))?;
        if let Some(index) = guard
            .sessions
            .iter()
            .position(|record| &record.session_id == session_id)
        {
            Ok(Some(guard.sessions.remove(index)))
        } else {
            Ok(None)
        }
    }
}

impl SessionStore for InMemoryStore {
    async fn create(&self, record: SessionRecord) -> Result<(), hepta_core::MemoryError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("session store mutex poisoned".into()))?;
        guard.sessions.push(record);
        Ok(())
    }

    async fn get(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<SessionRecord>, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("session store mutex poisoned".into()))?;
        Ok(guard
            .sessions
            .iter()
            .find(|record| &record.session_id == session_id)
            .cloned())
    }
}

impl MemoryStore for InMemoryStore {
    async fn put(&self, record: MemoryRecord) -> Result<(), hepta_core::MemoryError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("memory store mutex poisoned".into()))?;
        guard.memories.push(record);
        Ok(())
    }

    async fn search(
        &self,
        query: MemoryQuery,
    ) -> Result<Vec<MemoryRecord>, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("memory store mutex poisoned".into()))?;
        let mut hits = guard
            .memories
            .iter()
            .filter(|record| record.content.contains(&query.text))
            .cloned()
            .collect::<Vec<_>>();
        hits.truncate(query.limit);
        Ok(hits)
    }
}

impl MemoryReportStore for InMemoryStore {
    async fn search_report(
        &self,
        query: MemoryQuery,
    ) -> Result<MemoryQueryReport, hepta_core::MemoryError> {
        InMemoryStore::search_report(self, query)
    }
}

impl TranscriptStore for InMemoryStore {
    async fn append(&self, entry: TranscriptEntry) -> Result<(), hepta_core::MemoryError> {
        self.append_transcript_sync(entry)
    }

    async fn query(
        &self,
        query: TranscriptQuery,
    ) -> Result<TranscriptQueryReport, hepta_core::MemoryError> {
        self.transcript_search_report(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hepta_core::{
        AgentId, ContextRecallAvailability, ContextRecallCoverage, ContextRecallCoverageCounts,
        ContextRecallLimitPressure, ContextRecallOmissionCounts, ContextRecallSourceAvailability,
        ContextRecallSourceCounts, ContextRecallTranscriptProvenanceSummary, MemoryScope,
        MessageRole, QueryReportCoverage, QueryReportLimitPressure, RestoreDeltaCounts,
        SnapshotInspectionDriftImpact, SnapshotInspectionHealth, SnapshotInspectionSection,
        SnapshotIssueSummary, SnapshotRestoreDomain, SnapshotRestoreDomainImpact,
        SnapshotRestoreMutationProfile, SnapshotRestoreReadiness, SnapshotRestoreSafety,
        TranscriptEntryKind,
    };

    fn assert_memory_report_store<T: MemoryReportStore>() {}

    fn session_record(session_id: &str, title: &str, last_intent: Option<&str>) -> SessionRecord {
        SessionRecord {
            session_id: SessionId(session_id.into()),
            agent_id: AgentId("builder".into()),
            title: title.into(),
            created_at_unix_ms: 10,
            last_active_unix_ms: 20,
            last_user_intent_summary: last_intent.map(str::to_string),
            archived_at_unix_ms: None,
        }
    }

    fn memory_record(id: &str, scope: MemoryScope, content: &str) -> MemoryRecord {
        MemoryRecord {
            id: id.into(),
            scope,
            content: content.into(),
        }
    }

    fn transcript_entry(
        session_id: &str,
        sequence: u64,
        kind: TranscriptEntryKind,
        content: &str,
    ) -> TranscriptEntry {
        TranscriptEntry {
            entry_id: format!("{}-{}", session_id, sequence),
            session_id: SessionId(session_id.into()),
            sequence,
            kind,
            role: Some(MessageRole::Assistant),
            content: content.into(),
            created_at_unix_ms: 100 + sequence,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        }
    }

    #[test]
    fn upsert_session_sync_replaces_existing_record_without_duplication() {
        let store = InMemoryStore::default();

        store
            .upsert_session_sync(session_record("session-1", "Initial", Some("draft")))
            .expect("first upsert should succeed");
        store
            .upsert_session_sync(session_record("session-1", "Renamed", Some("finalize")))
            .expect("second upsert should succeed");

        let sessions = store.list_sessions().expect("sessions should load");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].title, "Renamed");
        assert_eq!(
            sessions[0].last_user_intent_summary.as_deref(),
            Some("finalize")
        );
    }

    #[test]
    fn remove_session_sync_returns_removed_record_and_updates_store() {
        let store = InMemoryStore::default();
        let record = session_record("session-1", "Foundation", None);
        let session_id = record.session_id.clone();

        store
            .upsert_session_sync(record.clone())
            .expect("upsert should succeed");

        let removed = store
            .remove_session_sync(&session_id)
            .expect("remove should succeed");

        assert_eq!(removed, Some(record));
        assert!(
            store
                .list_sessions()
                .expect("sessions should load")
                .is_empty()
        );
    }

    #[test]
    fn store_snapshot_roundtrips_through_json() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("audit memory"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "doctor snapshot integrity",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "snapshot captured",
            )],
        };

        let json = serde_json::to_string(&snapshot).expect("snapshot should serialize");
        let parsed: StoreSnapshot =
            serde_json::from_str(&json).expect("snapshot should deserialize");

        assert_eq!(parsed, snapshot);
    }

    #[test]
    fn store_snapshot_deserializes_without_transcripts_field() {
        let parsed: StoreSnapshot = serde_json::from_str(r#"{"sessions":[],"memories":[]}"#)
            .expect("legacy snapshot should deserialize");

        assert!(parsed.transcripts.is_empty());
    }

    #[test]
    fn inspected_store_snapshot_matches_snapshot_helpers() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("inspect snapshot"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot contract payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "inspection summary",
            )],
        };

        let inspected = InspectedStoreSnapshot::from_snapshot(snapshot.clone());

        assert_eq!(inspected.snapshot, snapshot);
        assert_eq!(inspected.inspection, snapshot.inspection_bundle());
        assert_eq!(inspected.audit_report(), snapshot.audit_report());
        assert!(snapshot.inspection_matches(&inspected.inspection));
        assert!(inspected.inspection_matches_snapshot());
        assert_eq!(inspected.issue_count(), 0);
        assert!(inspected.is_clean());
    }

    #[test]
    fn inspected_store_snapshot_restore_helpers_delegate_to_snapshot_payload() {
        let current = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Current foundation",
                Some("current"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "current payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "current summary",
            )],
        };
        let inspected = InspectedStoreSnapshot::from_snapshot(StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Updated foundation",
                Some("incoming"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "incoming payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "incoming summary",
            )],
        });

        assert_eq!(
            inspected.restore_preview_against(&current),
            inspected.snapshot.restore_preview_against(&current)
        );
        assert_eq!(
            inspected.restore_impact_against(&current),
            inspected.snapshot.restore_impact_against(&current)
        );
        assert_eq!(
            inspected.restore_readiness_against(&current),
            inspected.snapshot.restore_readiness_against(&current)
        );
        assert_eq!(
            inspected.restore_safety_against(&current),
            inspected.snapshot.restore_safety_against(&current)
        );
        assert_eq!(
            inspected.restore_mutation_profile_against(&current),
            inspected
                .snapshot
                .restore_mutation_profile_against(&current)
        );
        assert_eq!(
            inspected.restore_domain_impacts_against(&current),
            inspected.snapshot.restore_domain_impacts_against(&current)
        );
        assert_eq!(
            inspected.restore_changed_domains_against(&current),
            inspected.snapshot.restore_changed_domains_against(&current)
        );
    }

    #[test]
    fn inspected_store_snapshot_restore_helpers_ignore_drifted_inspection_state() {
        let current = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Current foundation",
                Some("current"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::Session,
                "current payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "current message",
            )],
        };
        let incoming = StoreSnapshot {
            sessions: vec![session_record(
                "session-2",
                "Added foundation",
                Some("incoming"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "updated payload",
            )],
            transcripts: vec![transcript_entry(
                "session-2",
                1,
                TranscriptEntryKind::Summary,
                "added summary",
            )],
        };
        let drifted = InspectedStoreSnapshot {
            snapshot: incoming.clone(),
            inspection: SnapshotInspectionBundle::default(),
        };

        assert!(!drifted.inspection_matches_snapshot());
        assert_eq!(
            drifted.restore_preview_against(&current),
            incoming.restore_preview_against(&current)
        );
        assert_eq!(
            drifted.restore_impact_against(&current),
            incoming.restore_impact_against(&current)
        );
        assert_eq!(
            drifted.restore_readiness_against(&current),
            incoming.restore_readiness_against(&current)
        );
        assert_eq!(
            drifted.restore_safety_against(&current),
            incoming.restore_safety_against(&current)
        );
        assert_eq!(
            drifted.restore_mutation_profile_against(&current),
            incoming.restore_mutation_profile_against(&current)
        );
        assert_eq!(
            drifted.restore_domain_impacts_against(&current),
            incoming.restore_domain_impacts_against(&current)
        );
        assert_eq!(
            drifted.restore_changed_domains_against(&current),
            incoming.restore_changed_domains_against(&current)
        );
    }

    #[test]
    fn inspected_store_snapshot_roundtrips_through_json() {
        let inspected = InspectedStoreSnapshot::from_snapshot(StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("roundtrip inspected snapshot"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::Session,
                "roundtrip contract payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "roundtrip transcript",
            )],
        });

        let json = serde_json::to_string(&inspected).expect("inspected snapshot should serialize");
        let parsed: InspectedStoreSnapshot =
            serde_json::from_str(&json).expect("inspected snapshot should deserialize");

        assert_eq!(parsed, inspected);
        assert_eq!(parsed.audit_report(), inspected.audit_report());
        assert!(parsed.inspection_matches_snapshot());
    }

    #[test]
    fn inspected_store_snapshot_deserializes_without_inspection_field() {
        let canonical = InspectedStoreSnapshot::from_snapshot(StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("backfill inspection"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "snapshot captured",
            )],
        });
        let mut json = serde_json::to_value(&canonical).expect("snapshot should serialize");
        json.as_object_mut()
            .expect("inspected snapshot should serialize as an object")
            .remove("inspection");

        let parsed: InspectedStoreSnapshot =
            serde_json::from_value(json).expect("legacy inspected snapshot should deserialize");

        assert_eq!(parsed, canonical);
        assert!(parsed.inspection_matches_snapshot());
        assert_eq!(parsed.audit_report(), parsed.snapshot.audit_report());
    }

    #[test]
    fn inspected_store_snapshot_normalized_rebuilds_drifted_inspection() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("normalize inspection bundle"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "summary payload",
            )],
        };
        let canonical = InspectedStoreSnapshot::from_snapshot(snapshot.clone());
        let drifted = InspectedStoreSnapshot {
            snapshot,
            inspection: SnapshotInspectionBundle::default(),
        };

        assert!(!drifted.inspection_matches_snapshot());
        assert_eq!(drifted.audit_report(), SnapshotAuditReport::default());

        let normalized = drifted.normalized();

        assert_eq!(normalized, canonical);
        assert!(normalized.inspection_matches_snapshot());
    }

    #[test]
    fn store_snapshot_inspection_drift_report_tracks_section_level_drift() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("track inspection drift"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "summary payload",
            )],
        };

        let drift = snapshot.inspection_drift_report(&SnapshotInspectionBundle::default());

        assert_eq!(
            drift.mismatched_sections,
            vec![
                SnapshotInspectionSection::MemoryManifest,
                SnapshotInspectionSection::TranscriptManifest,
            ]
        );
        assert_eq!(drift.mismatch_count(), 2);
        assert!(!drift.is_aligned());
    }

    #[test]
    fn inspected_store_snapshot_inspection_drift_report_matches_alignment_state() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("inspect drift state"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::Session,
                "snapshot payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "snapshot captured",
            )],
        };

        let aligned = InspectedStoreSnapshot::from_snapshot(snapshot.clone());
        let drifted = InspectedStoreSnapshot {
            snapshot,
            inspection: SnapshotInspectionBundle::default(),
        };

        assert!(aligned.inspection_drift_report().is_aligned());
        assert_eq!(aligned.inspection_drift_report().mismatch_count(), 0);

        let drift = drifted.inspection_drift_report();
        assert!(!drift.is_aligned());
        assert_eq!(drift.mismatch_count(), 2);
        assert!(drift.mismatches(SnapshotInspectionSection::MemoryManifest));
        assert!(drift.mismatches(SnapshotInspectionSection::TranscriptManifest));
        assert!(!drift.mismatches(SnapshotInspectionSection::MemoryIntegrity));
        assert!(!drift.mismatches(SnapshotInspectionSection::TranscriptIntegrity));
    }

    #[test]
    fn store_snapshot_inspection_drift_impact_collapses_sections_by_domain() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("collapse drift domains"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "summary payload",
            )],
        };

        let impact = snapshot.inspection_drift_impact(&SnapshotInspectionBundle::default());

        assert_eq!(
            impact,
            SnapshotInspectionDriftImpact {
                mismatch_count: 2,
                memory_mismatch_count: 1,
                transcript_mismatch_count: 1,
            }
        );
        assert_eq!(impact.changed_domain_count(), 2);
        assert!(impact.touches_memory());
        assert!(impact.touches_transcripts());
        assert!(!impact.is_aligned());
    }

    #[test]
    fn inspected_store_snapshot_inspection_drift_impact_matches_report_impact() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("impact alignment"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::Session,
                "snapshot payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "snapshot captured",
            )],
        };
        let aligned = InspectedStoreSnapshot::from_snapshot(snapshot.clone());
        let drifted = InspectedStoreSnapshot {
            snapshot,
            inspection: SnapshotInspectionBundle::default(),
        };

        assert_eq!(
            aligned.inspection_drift_impact(),
            aligned.inspection_drift_report().impact()
        );
        assert!(aligned.inspection_drift_impact().is_aligned());

        let impact = drifted.inspection_drift_impact();
        assert_eq!(impact, drifted.inspection_drift_report().impact());
        assert_eq!(impact.mismatch_count, 2);
        assert_eq!(impact.changed_domain_count(), 2);
        assert!(impact.touches_memory());
        assert!(impact.touches_transcripts());
    }

    #[test]
    fn store_snapshot_inspection_health_combines_issue_summary_and_drift_impact() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("health summary"),
            )],
            memories: vec![memory_record("memory-1", MemoryScope::LongTerm, " ")],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                " ",
            )],
        };

        let canonical = snapshot.inspection_bundle();
        let inspection = SnapshotInspectionBundle {
            memory_manifest: MemorySnapshotManifest::default(),
            memory_integrity: canonical.memory_integrity,
            transcript_manifest: TranscriptSnapshotManifest::default(),
            transcript_integrity: canonical.transcript_integrity,
        };

        let health = snapshot.inspection_health(&inspection);

        assert_eq!(
            health,
            SnapshotInspectionHealth {
                issue_summary: inspection.issue_summary(),
                drift_impact: snapshot.inspection_drift_impact(&inspection),
            }
        );
        assert_eq!(health.issue_count(), 2);
        assert_eq!(health.mismatch_count(), 2);
        assert_eq!(health.changed_domain_count(), 2);
        assert!(health.touches_memory());
        assert!(health.touches_transcripts());
        assert!(health.has_issues());
        assert!(health.has_drift());
        assert!(!health.inspection_aligned());
        assert!(!health.is_clean());
        assert!(!health.is_ready());
    }

    #[test]
    fn inspected_store_snapshot_inspection_health_uses_embedded_bundle() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("embedded inspection health"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::Session,
                "snapshot payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "snapshot captured",
            )],
        };
        let aligned = InspectedStoreSnapshot::from_snapshot(snapshot.clone());
        let drifted = InspectedStoreSnapshot {
            snapshot,
            inspection: SnapshotInspectionBundle::default(),
        };

        assert_eq!(
            aligned.inspection_health(),
            aligned.snapshot.inspection_health(&aligned.inspection)
        );
        assert!(aligned.inspection_health().is_ready());

        let drifted_health = drifted.inspection_health();
        assert_eq!(
            drifted_health,
            drifted.snapshot.inspection_health(&drifted.inspection)
        );
        assert_eq!(drifted_health.issue_count(), 0);
        assert_eq!(drifted_health.mismatch_count(), 2);
        assert!(drifted_health.has_drift());
        assert!(!drifted_health.is_ready());
    }

    #[test]
    fn store_snapshot_recall_context_uses_recent_window_query_hits_and_scope_split() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
                memory_record("memory-3", MemoryScope::LongTerm, "unrelated note"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "start diagnosis",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    3,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout in another session",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 2,
            transcript_limit: 1,
            memory_limit: 3,
            allow_cross_session: true,
        };

        let bundle = snapshot.recall_context(&request);

        assert_eq!(bundle.recent_entries.len(), 2);
        assert_eq!(bundle.recent_entries[0].sequence, 2);
        assert_eq!(bundle.recent_entries[1].sequence, 3);
        assert_eq!(bundle.transcript_hits.len(), 1);
        assert_eq!(bundle.transcript_hits[0].range.start_sequence, 2);
        assert_eq!(bundle.durable_memory_hits.len(), 1);
        assert_eq!(bundle.durable_memory_hits[0].id, "memory-1");
        assert_eq!(bundle.summary_hits.len(), 1);
        assert_eq!(bundle.summary_hits[0].id, "memory-2");
        assert_eq!(
            bundle.source_counts(),
            ContextRecallSourceCounts {
                recent_entry_count: 2,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 1,
            }
        );
        assert_eq!(bundle.query_hit_count(), 3);
        assert_eq!(bundle.total_item_count(), 5);
        assert!(bundle.truncated);
    }

    #[test]
    fn store_snapshot_recall_context_treats_blank_query_as_memory_only_default_search() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record(
                    "memory-1",
                    MemoryScope::LongTerm,
                    "always included for blank query",
                ),
                memory_record("memory-2", MemoryScope::Session, "session summary fallback"),
            ],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            )],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("   ".into()),
            recent_window_limit: 4,
            transcript_limit: 5,
            memory_limit: 1,
            allow_cross_session: true,
        };

        let bundle = snapshot.recall_context(&request);

        assert_eq!(bundle.recent_entries.len(), 1);
        assert!(bundle.transcript_hits.is_empty());
        assert_eq!(bundle.durable_memory_hits.len(), 1);
        assert_eq!(bundle.durable_memory_hits[0].id, "memory-1");
        assert!(bundle.summary_hits.is_empty());
        assert_eq!(
            bundle.source_counts(),
            ContextRecallSourceCounts {
                recent_entry_count: 1,
                transcript_hit_count: 0,
                durable_memory_hit_count: 1,
                summary_hit_count: 0,
            }
        );
        assert!(bundle.has_query_matches());
        assert!(bundle.truncated);
    }

    #[test]
    fn store_snapshot_recall_context_treats_cross_session_flag_as_advisory_only() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout in another session",
                ),
            ],
        };
        let session_scoped = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 4,
            transcript_limit: 4,
            memory_limit: 4,
            allow_cross_session: false,
        };
        let mut cross_session = session_scoped.clone();
        cross_session.allow_cross_session = true;

        let session_scoped_bundle = snapshot.recall_context(&session_scoped);
        let cross_session_bundle = snapshot.recall_context(&cross_session);

        assert_eq!(
            session_scoped_bundle.recent_entries,
            cross_session_bundle.recent_entries
        );
        assert_eq!(
            session_scoped_bundle.transcript_hits,
            cross_session_bundle.transcript_hits
        );
        assert_eq!(
            session_scoped_bundle.durable_memory_hits,
            cross_session_bundle.durable_memory_hits
        );
        assert_eq!(
            session_scoped_bundle.summary_hits,
            cross_session_bundle.summary_hits
        );
        assert_eq!(session_scoped_bundle.transcript_hits.len(), 1);
        assert_eq!(
            session_scoped_bundle.transcript_hits[0].session_id,
            SessionId("session-1".into())
        );
        assert_eq!(session_scoped_bundle.query_hit_count(), 3);
        assert_eq!(cross_session_bundle.query_hit_count(), 3);
        assert!(!session_scoped_bundle.truncated);
        assert!(!cross_session_bundle.truncated);
        assert!(!session_scoped_bundle.request.allow_cross_session);
        assert!(cross_session_bundle.request.allow_cross_session);
    }

    #[test]
    fn store_snapshot_recall_context_report_matches_bundle_report() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 2,
            transcript_limit: 1,
            memory_limit: 5,
            allow_cross_session: true,
        };

        let report = snapshot.recall_context_report(&request);
        let bundle = snapshot.recall_context(&request);

        assert_eq!(report, bundle.report());
        assert_eq!(report.request, request);
        assert_eq!(report.source_counts, bundle.source_counts());
        assert_eq!(report.query_hit_count(), 3);
        assert_eq!(report.total_item_count(), 5);
        assert!(report.has_query_matches());
        assert!(!report.is_empty());
        assert!(report.truncated);
    }

    #[test]
    fn store_snapshot_recall_context_inspection_tracks_availability_counts() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
                memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "start diagnosis",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    3,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout in another session",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 2,
            transcript_limit: 1,
            memory_limit: 2,
            allow_cross_session: true,
        };

        let inspection = snapshot.recall_context_inspection(&request);
        let bundle = snapshot.recall_context(&request);

        assert_eq!(inspection.report, bundle.report());
        assert_eq!(inspection.availability.total_recent_entry_count, 3);
        assert_eq!(inspection.availability.total_transcript_match_count, 2);
        assert_eq!(inspection.availability.total_memory_match_count, 3);
        assert_eq!(inspection.returned_query_hit_count(), 3);
        assert_eq!(inspection.omitted_recent_entry_count(), 1);
        assert_eq!(inspection.omitted_transcript_hit_count(), 1);
        assert_eq!(inspection.omitted_memory_hit_count(), 1);
        assert_eq!(inspection.omitted_query_hit_count(), 2);
        assert_eq!(inspection.matched_query_hit_count(), 5);
        assert_eq!(inspection.returned_total_item_count(), 5);
        assert_eq!(inspection.matched_total_item_count(), 8);
        assert_eq!(inspection.omitted_total_item_count(), 3);
        assert_eq!(
            inspection.omission_counts(),
            ContextRecallOmissionCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                memory_hit_count: 1,
                query_hit_count: 2,
                total_item_count: 3,
            }
        );
        assert!(inspection.has_omissions());
        assert!(inspection.recent_entries_truncated());
        assert!(inspection.transcript_hits_truncated());
        assert!(inspection.memory_hits_truncated());
        assert!(inspection.has_query_matches());
        assert!(!inspection.is_complete());
        assert!(!inspection.is_empty());
    }

    #[test]
    fn store_snapshot_recall_context_transcript_provenance_summary_matches_inspection_helper() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            )],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };

        let summary = snapshot.recall_context_transcript_provenance_summary(&request);

        assert_eq!(
            summary,
            snapshot
                .recall_context_inspection(&request)
                .transcript_provenance_summary()
        );
        assert_eq!(
            summary,
            ContextRecallTranscriptProvenanceSummary {
                span_count: 2,
                session_count: 1,
                spans_with_reason_count: 2,
                distinct_reason_count: 2,
            }
        );
    }

    #[test]
    fn store_snapshot_recall_context_coverage_matches_inspection_helper() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
                memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "start diagnosis",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    3,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout in another session",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 2,
            transcript_limit: 1,
            memory_limit: 2,
            allow_cross_session: true,
        };

        let coverage = snapshot.recall_context_coverage(&request);

        assert_eq!(
            coverage,
            snapshot.recall_context_inspection(&request).coverage()
        );
        assert_eq!(
            coverage,
            ContextRecallCoverage {
                recent_entries: ContextRecallCoverageCounts {
                    returned_count: 2,
                    available_count: 3,
                },
                transcript_hits: ContextRecallCoverageCounts {
                    returned_count: 1,
                    available_count: 2,
                },
                memory_hits: ContextRecallCoverageCounts {
                    returned_count: 2,
                    available_count: 3,
                },
                query_hits: ContextRecallCoverageCounts {
                    returned_count: 3,
                    available_count: 5,
                },
                total_items: ContextRecallCoverageCounts {
                    returned_count: 5,
                    available_count: 8,
                },
            }
        );
        assert_eq!(coverage.omitted_total_item_count(), 3);
        assert_eq!(
            coverage.omission_counts(),
            ContextRecallOmissionCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                memory_hit_count: 1,
                query_hit_count: 2,
                total_item_count: 3,
            }
        );
        assert!(coverage.has_omissions());
        assert!(!coverage.is_complete());
    }

    #[test]
    fn store_snapshot_recall_context_omission_counts_match_coverage_helper() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
                memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "start diagnosis",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    3,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout in another session",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 2,
            transcript_limit: 1,
            memory_limit: 2,
            allow_cross_session: true,
        };

        let omission_counts = snapshot.recall_context_omission_counts(&request);

        assert_eq!(
            omission_counts,
            snapshot.recall_context_coverage(&request).omission_counts()
        );
        assert_eq!(
            omission_counts,
            ContextRecallOmissionCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                memory_hit_count: 1,
                query_hit_count: 2,
                total_item_count: 3,
            }
        );
        assert!(omission_counts.has_omissions());
        assert!(!omission_counts.is_empty());
    }

    #[test]
    fn store_snapshot_recall_context_limit_pressure_matches_coverage_helper() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
                memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "start diagnosis",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    3,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout in another session",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 2,
            transcript_limit: 1,
            memory_limit: 2,
            allow_cross_session: true,
        };

        let pressure = snapshot.recall_context_limit_pressure(&request);

        assert_eq!(
            pressure,
            snapshot.recall_context_coverage(&request).limit_pressure()
        );
        assert_eq!(
            pressure,
            ContextRecallLimitPressure {
                recent_entries_truncated: true,
                transcript_hits_truncated: true,
                memory_hits_truncated: true,
                omission_counts: ContextRecallOmissionCounts {
                    recent_entry_count: 1,
                    transcript_hit_count: 1,
                    memory_hit_count: 1,
                    query_hit_count: 2,
                    total_item_count: 3,
                },
            }
        );
        assert!(pressure.query_hits_truncated());
        assert!(pressure.has_omissions());
        assert!(!pressure.is_complete());
    }

    #[tokio::test]
    async fn store_recall_context_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-2",
                MemoryScope::Session,
                "session timeout summary",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 4,
            transcript_limit: 2,
            memory_limit: 4,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context(request.clone())
            .expect("context recall should succeed");

        assert_eq!(from_store, snapshot.recall_context(&request));
        assert_eq!(
            from_store.source_counts(),
            ContextRecallSourceCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 1,
            }
        );
        assert_eq!(from_store.query_hit_count(), 3);
        assert_eq!(from_store.total_item_count(), 4);
        assert!(!from_store.truncated);
    }

    #[tokio::test]
    async fn store_recall_context_report_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 4,
            transcript_limit: 2,
            memory_limit: 4,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context_report(request.clone())
            .expect("context recall report should succeed");

        assert_eq!(from_store, snapshot.recall_context_report(&request));
        assert_eq!(from_store.request, request);
        assert_eq!(from_store.query_hit_count(), 2);
        assert_eq!(from_store.total_item_count(), 3);
        assert!(from_store.has_query_matches());
        assert!(!from_store.truncated);
    }

    #[test]
    fn store_snapshot_recall_context_availability_matches_inspection_helper() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
                memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "start diagnosis",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    3,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout in another session",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };

        let availability = snapshot.recall_context_availability(&request);

        assert_eq!(
            availability,
            ContextRecallAvailability {
                total_recent_entry_count: 3,
                total_transcript_match_count: 2,
                total_memory_match_count: 3,
            }
        );
        assert_eq!(
            availability,
            snapshot.recall_context_inspection(&request).availability
        );
        assert_eq!(availability.query_match_count(), 5);
        assert_eq!(availability.total_item_count(), 8);
        assert!(availability.has_query_matches());
        assert!(!availability.is_empty());
    }

    #[test]
    fn store_snapshot_recall_context_source_availability_preserves_memory_scope_split() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
                memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
                memory_record(
                    "memory-4",
                    MemoryScope::Session,
                    "timeout summary follow-up",
                ),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "start diagnosis",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout in another session",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };

        let availability = snapshot.recall_context_source_availability(&request);

        assert_eq!(
            availability,
            ContextRecallSourceAvailability {
                recent_entry_count: 2,
                transcript_match_count: 1,
                durable_memory_match_count: 2,
                summary_memory_match_count: 2,
            }
        );
        assert_eq!(availability.memory_match_count(), 4);
        assert_eq!(availability.query_match_count(), 5);
        assert_eq!(availability.total_item_count(), 7);
        assert!(availability.has_query_matches());
        assert!(!availability.is_empty());
    }

    #[tokio::test]
    async fn store_recall_context_inspection_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-2",
                MemoryScope::Session,
                "session timeout summary",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context_inspection(request.clone())
            .expect("context recall inspection should succeed");

        assert_eq!(from_store, snapshot.recall_context_inspection(&request));
        assert_eq!(
            from_store.report,
            snapshot.recall_context(&request).report()
        );
        assert_eq!(from_store.availability.total_recent_entry_count, 2);
        assert_eq!(from_store.availability.total_transcript_match_count, 2);
        assert_eq!(from_store.availability.total_memory_match_count, 2);
        assert_eq!(from_store.omitted_recent_entry_count(), 1);
        assert_eq!(from_store.omitted_transcript_hit_count(), 1);
        assert_eq!(from_store.omitted_memory_hit_count(), 1);
        assert_eq!(from_store.omitted_query_hit_count(), 2);
        assert_eq!(from_store.omitted_total_item_count(), 3);
        assert_eq!(
            from_store.omission_counts(),
            ContextRecallOmissionCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                memory_hit_count: 1,
                query_hit_count: 2,
                total_item_count: 3,
            }
        );
        assert!(from_store.has_omissions());
        assert!(from_store.recent_entries_truncated());
        assert!(from_store.transcript_hits_truncated());
        assert!(from_store.memory_hits_truncated());
        assert!(!from_store.is_complete());
    }

    #[tokio::test]
    async fn store_recall_context_availability_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-2",
                MemoryScope::Session,
                "session timeout summary",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context_availability(request.clone())
            .expect("context recall availability should succeed");

        assert_eq!(from_store, snapshot.recall_context_availability(&request));
        assert_eq!(
            from_store,
            ContextRecallAvailability {
                total_recent_entry_count: 2,
                total_transcript_match_count: 2,
                total_memory_match_count: 2,
            }
        );
        assert_eq!(
            from_store,
            store
                .recall_context_inspection(request)
                .expect("context recall inspection should succeed")
                .availability
        );
    }

    #[tokio::test]
    async fn store_recall_context_source_availability_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-2",
                MemoryScope::Session,
                "session timeout summary",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-3",
                MemoryScope::LongTerm,
                "timeout rollback note",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context_source_availability(request.clone())
            .expect("context recall source availability should succeed");

        assert_eq!(
            from_store,
            snapshot.recall_context_source_availability(&request)
        );
        assert_eq!(
            from_store,
            ContextRecallSourceAvailability {
                recent_entry_count: 2,
                transcript_match_count: 2,
                durable_memory_match_count: 2,
                summary_memory_match_count: 1,
            }
        );
        assert_eq!(from_store.memory_match_count(), 3);
        assert_eq!(from_store.query_match_count(), 5);
        assert_eq!(from_store.total_item_count(), 7);
    }

    #[tokio::test]
    async fn store_recall_context_transcript_provenance_summary_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context_transcript_provenance_summary(request.clone())
            .expect("context recall provenance summary should succeed");

        assert_eq!(
            from_store,
            snapshot.recall_context_transcript_provenance_summary(&request)
        );
        assert_eq!(
            from_store,
            ContextRecallTranscriptProvenanceSummary {
                span_count: 2,
                session_count: 1,
                spans_with_reason_count: 2,
                distinct_reason_count: 2,
            }
        );
    }

    #[tokio::test]
    async fn store_recall_context_coverage_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-2",
                MemoryScope::Session,
                "session timeout summary",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context_coverage(request.clone())
            .expect("context recall coverage should succeed");

        assert_eq!(from_store, snapshot.recall_context_coverage(&request));
        assert_eq!(
            from_store,
            ContextRecallCoverage {
                recent_entries: ContextRecallCoverageCounts {
                    returned_count: 1,
                    available_count: 2,
                },
                transcript_hits: ContextRecallCoverageCounts {
                    returned_count: 1,
                    available_count: 2,
                },
                memory_hits: ContextRecallCoverageCounts {
                    returned_count: 1,
                    available_count: 2,
                },
                query_hits: ContextRecallCoverageCounts {
                    returned_count: 2,
                    available_count: 4,
                },
                total_items: ContextRecallCoverageCounts {
                    returned_count: 3,
                    available_count: 6,
                },
            }
        );
        assert!(from_store.has_omissions());
        assert!(!from_store.is_complete());
    }

    #[tokio::test]
    async fn store_recall_context_omission_counts_match_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-2",
                MemoryScope::Session,
                "session timeout summary",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context_omission_counts(request.clone())
            .expect("context recall omission counts should succeed");

        assert_eq!(
            from_store,
            snapshot.recall_context_omission_counts(&request)
        );
        assert_eq!(
            from_store,
            ContextRecallOmissionCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                memory_hit_count: 1,
                query_hit_count: 2,
                total_item_count: 3,
            }
        );
        assert!(from_store.has_omissions());
        assert!(!from_store.is_empty());
    }

    #[tokio::test]
    async fn store_recall_context_limit_pressure_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "timeout retry guidance",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-2",
                MemoryScope::Session,
                "session timeout summary",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ))
            .await
            .expect("append should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        };
        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .recall_context_limit_pressure(request.clone())
            .expect("context recall limit pressure should succeed");

        assert_eq!(from_store, snapshot.recall_context_limit_pressure(&request));
        assert_eq!(
            from_store,
            ContextRecallLimitPressure {
                recent_entries_truncated: true,
                transcript_hits_truncated: true,
                memory_hits_truncated: true,
                omission_counts: ContextRecallOmissionCounts {
                    recent_entry_count: 1,
                    transcript_hit_count: 1,
                    memory_hit_count: 1,
                    query_hit_count: 2,
                    total_item_count: 3,
                },
            }
        );
        assert!(from_store.query_hits_truncated());
        assert!(from_store.has_omissions());
        assert!(!from_store.is_complete());
    }

    #[test]
    fn store_snapshot_recall_context_with_zero_limits_reports_full_omission_pressure() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
                memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "timeout surfaced during tool run",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Summary,
                    "timeout retried successfully",
                ),
            ],
        };
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 0,
            transcript_limit: 0,
            memory_limit: 0,
            allow_cross_session: true,
        };

        let bundle = snapshot.recall_context(&request);
        let inspection = snapshot.recall_context_inspection(&request);
        let coverage = snapshot.recall_context_coverage(&request);
        let pressure = snapshot.recall_context_limit_pressure(&request);

        assert!(bundle.recent_entries.is_empty());
        assert!(bundle.transcript_hits.is_empty());
        assert!(bundle.durable_memory_hits.is_empty());
        assert!(bundle.summary_hits.is_empty());
        assert!(bundle.truncated);
        assert_eq!(inspection.availability.total_recent_entry_count, 2);
        assert_eq!(inspection.availability.total_transcript_match_count, 2);
        assert_eq!(inspection.availability.total_memory_match_count, 2);
        assert_eq!(
            coverage,
            ContextRecallCoverage {
                recent_entries: ContextRecallCoverageCounts {
                    returned_count: 0,
                    available_count: 2,
                },
                transcript_hits: ContextRecallCoverageCounts {
                    returned_count: 0,
                    available_count: 2,
                },
                memory_hits: ContextRecallCoverageCounts {
                    returned_count: 0,
                    available_count: 2,
                },
                query_hits: ContextRecallCoverageCounts {
                    returned_count: 0,
                    available_count: 4,
                },
                total_items: ContextRecallCoverageCounts {
                    returned_count: 0,
                    available_count: 6,
                },
            }
        );
        assert_eq!(
            pressure,
            ContextRecallLimitPressure {
                recent_entries_truncated: true,
                transcript_hits_truncated: true,
                memory_hits_truncated: true,
                omission_counts: ContextRecallOmissionCounts {
                    recent_entry_count: 2,
                    transcript_hit_count: 2,
                    memory_hit_count: 2,
                    query_hit_count: 4,
                    total_item_count: 6,
                },
            }
        );
        assert!(pressure.query_hits_truncated());
        assert!(pressure.has_omissions());
        assert!(!pressure.is_complete());
    }

    #[test]
    fn transcript_snapshot_stats_summarize_kinds_and_sessions() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "user asks for a report",
                ),
                transcript_entry(
                    "session-1",
                    2,
                    TranscriptEntryKind::Summary,
                    "summary written",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Event,
                    "session archived",
                ),
            ],
        };

        let stats = snapshot.transcript_stats();

        assert_eq!(stats.total_entry_count, 3);
        assert_eq!(stats.session_count, 2);
        assert_eq!(stats.message_count, 1);
        assert_eq!(stats.summary_count, 1);
        assert_eq!(stats.event_count, 1);
        assert!(!stats.is_empty());
    }

    #[test]
    fn snapshot_stats_summarize_active_archived_and_memory_scope_counts() {
        let snapshot = StoreSnapshot {
            sessions: vec![
                session_record("session-1", "Foundation", Some("audit memory")),
                SessionRecord {
                    archived_at_unix_ms: Some(30),
                    ..session_record("session-2", "Archived foundation", None)
                },
            ],
            memories: vec![
                memory_record(
                    "memory-1",
                    MemoryScope::Session,
                    "doctor snapshot integrity",
                ),
                memory_record("memory-2", MemoryScope::LongTerm, "export manifest"),
            ],
            transcripts: vec![],
        };

        let stats = snapshot.stats();

        assert_eq!(stats.session_count, 2);
        assert_eq!(stats.active_session_count, 1);
        assert_eq!(stats.archived_session_count, 1);
        assert_eq!(stats.total_memory_count, 2);
        assert_eq!(stats.session_memory_count, 1);
        assert_eq!(stats.long_term_memory_count, 1);
    }

    #[test]
    fn session_agent_inventory_summarizes_sessions_by_agent() {
        let snapshot = StoreSnapshot {
            sessions: vec![
                SessionRecord {
                    session_id: SessionId("session-2".into()),
                    agent_id: AgentId("reviewer".into()),
                    title: "Reviewer lane".into(),
                    created_at_unix_ms: 9,
                    last_active_unix_ms: 20,
                    last_user_intent_summary: None,
                    archived_at_unix_ms: None,
                },
                SessionRecord {
                    archived_at_unix_ms: Some(30),
                    last_active_unix_ms: 25,
                    ..session_record("session-1", "Builder lane", Some("contracts"))
                },
                SessionRecord {
                    session_id: SessionId("session-3".into()),
                    agent_id: AgentId("builder".into()),
                    title: "Builder follow-up".into(),
                    created_at_unix_ms: 11,
                    last_active_unix_ms: 40,
                    last_user_intent_summary: None,
                    archived_at_unix_ms: None,
                },
                SessionRecord {
                    session_id: SessionId("session-4".into()),
                    agent_id: AgentId("   ".into()),
                    title: "Blank agent lane".into(),
                    created_at_unix_ms: 12,
                    last_active_unix_ms: 50,
                    last_user_intent_summary: None,
                    archived_at_unix_ms: None,
                },
            ],
            memories: vec![],
            transcripts: vec![],
        };

        let inventory = snapshot.session_agent_inventory();

        assert_eq!(inventory.total_session_count, 4);
        assert_eq!(inventory.blank_agent_id_session_count, 1);
        assert_eq!(inventory.agent_count(), 2);
        assert_eq!(inventory.inventoried_session_count(), 3);
        assert_eq!(inventory.agents[0].agent_id.0, "builder");
        assert_eq!(inventory.agents[0].session_count, 2);
        assert_eq!(inventory.agents[0].active_session_count, 1);
        assert_eq!(inventory.agents[0].archived_session_count, 1);
        assert_eq!(inventory.agents[0].latest_activity_unix_ms, 40);
        assert_eq!(inventory.agents[1].agent_id.0, "reviewer");
        assert_eq!(inventory.agents[1].session_count, 1);
    }

    #[tokio::test]
    async fn store_session_agent_inventory_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Builder lane",
                Some("capture agent inventory"),
            ))
            .expect("upsert should succeed");
        store
            .upsert_session_sync(SessionRecord {
                session_id: SessionId("session-2".into()),
                agent_id: AgentId("reviewer".into()),
                title: "Reviewer lane".into(),
                created_at_unix_ms: 11,
                last_active_unix_ms: 40,
                last_user_intent_summary: None,
                archived_at_unix_ms: Some(50),
            })
            .expect("upsert should succeed");

        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .session_agent_inventory()
            .expect("agent inventory should load");

        assert_eq!(from_store, snapshot.session_agent_inventory());
        assert_eq!(from_store.total_session_count, 2);
        assert_eq!(from_store.blank_agent_id_session_count, 0);
        assert_eq!(from_store.agent_count(), 2);
        assert_eq!(from_store.inventoried_session_count(), 2);
        assert_eq!(from_store.agents[0].agent_id.0, "builder");
        assert_eq!(from_store.agents[1].agent_id.0, "reviewer");
        assert_eq!(from_store.agents[1].archived_session_count, 1);
    }

    #[tokio::test]
    async fn snapshot_restore_roundtrip_recovers_sessions_and_memories() {
        let source = InMemoryStore::default();
        let session = session_record("session-1", "Foundation", Some("capture snapshot"));
        let memory = memory_record(
            "memory-1",
            MemoryScope::Session,
            "snapshot integrity contract",
        );
        let transcript = transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "snapshot integrity contract",
        );

        source
            .upsert_session_sync(session.clone())
            .expect("upsert should succeed");
        source
            .put(memory.clone())
            .await
            .expect("memory put should succeed");
        source
            .append(transcript.clone())
            .await
            .expect("transcript append should succeed");

        let snapshot = source.snapshot().expect("snapshot should succeed");
        let restored = InMemoryStore::default();
        restored
            .restore(snapshot.clone())
            .expect("restore should succeed");

        assert_eq!(snapshot.sessions, vec![session]);
        assert_eq!(snapshot.memories, vec![memory]);
        assert_eq!(snapshot.transcripts, vec![transcript]);
        assert_eq!(
            restored.snapshot().expect("snapshot should succeed"),
            snapshot
        );
    }

    #[tokio::test]
    async fn search_filters_results_and_honors_limit() {
        let store = InMemoryStore::default();
        let matching_a = memory_record("memory-1", MemoryScope::Session, "doctor snapshot ok");
        let non_matching = memory_record("memory-2", MemoryScope::LongTerm, "approval ledger");
        let matching_b = memory_record("memory-3", MemoryScope::LongTerm, "snapshot rollback");

        store
            .put(matching_a.clone())
            .await
            .expect("put should succeed");
        store.put(non_matching).await.expect("put should succeed");
        store
            .put(matching_b.clone())
            .await
            .expect("put should succeed");

        let hits = store
            .search(MemoryQuery {
                text: "snapshot".into(),
                limit: 1,
            })
            .await
            .expect("search should succeed");

        assert_eq!(hits, vec![matching_a]);
    }

    #[tokio::test]
    async fn search_report_tracks_total_matches_and_truncation() {
        let store = InMemoryStore::default();
        let matching_a = memory_record("memory-1", MemoryScope::Session, "doctor snapshot ok");
        let non_matching = memory_record("memory-2", MemoryScope::LongTerm, "approval ledger");
        let matching_b = memory_record("memory-3", MemoryScope::LongTerm, "snapshot rollback");

        store
            .put(matching_a.clone())
            .await
            .expect("put should succeed");
        store.put(non_matching).await.expect("put should succeed");
        store.put(matching_b).await.expect("put should succeed");

        let report = store
            .search_report(MemoryQuery {
                text: "snapshot".into(),
                limit: 1,
            })
            .expect("search report should succeed");

        assert_eq!(report.query.text, "snapshot");
        assert_eq!(report.query.limit, 1);
        assert_eq!(report.matched_count, 2);
        assert_eq!(report.returned_count, 1);
        assert!(report.truncated);
        assert_eq!(report.hits, vec![matching_a]);
        assert!(!report.is_empty());
    }

    #[tokio::test]
    async fn search_coverage_and_limit_pressure_match_report_helpers() {
        let store = InMemoryStore::default();
        let matching_a = memory_record("memory-1", MemoryScope::Session, "doctor snapshot ok");
        let matching_b = memory_record("memory-2", MemoryScope::LongTerm, "snapshot rollback");
        let query = MemoryQuery {
            text: "snapshot".into(),
            limit: 1,
        };

        store
            .put(matching_a.clone())
            .await
            .expect("put should succeed");
        store
            .put(matching_b.clone())
            .await
            .expect("put should succeed");

        let snapshot = store.snapshot().expect("snapshot should load");
        let report = store
            .search_report(query.clone())
            .expect("search report should succeed");

        assert_eq!(snapshot.search_report(&query), report);
        assert_eq!(
            snapshot.search_coverage(&query),
            QueryReportCoverage {
                returned_count: 1,
                matched_count: 2,
            }
        );
        assert_eq!(
            store
                .search_coverage(query.clone())
                .expect("search coverage should succeed"),
            report.coverage()
        );
        assert_eq!(
            snapshot.search_limit_pressure(&query),
            QueryReportLimitPressure {
                truncated: true,
                omitted_count: 1,
            }
        );
        assert_eq!(
            store
                .search_limit_pressure(query)
                .expect("search limit pressure should succeed"),
            report.limit_pressure()
        );
    }

    #[tokio::test]
    async fn search_report_with_zero_limit_preserves_match_counts_and_full_omission() {
        let store = InMemoryStore::default();
        let query = MemoryQuery {
            text: "snapshot".into(),
            limit: 0,
        };

        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "doctor snapshot ok",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-2",
                MemoryScope::LongTerm,
                "snapshot rollback",
            ))
            .await
            .expect("put should succeed");

        let report = store
            .search_report(query.clone())
            .expect("search report should succeed");

        assert_eq!(report.query, query);
        assert_eq!(report.matched_count, 2);
        assert_eq!(report.returned_count, 0);
        assert!(report.truncated);
        assert!(report.hits.is_empty());
        assert_eq!(report.omitted_count(), 2);
        assert!(!report.is_complete());
        assert_eq!(
            report.coverage(),
            QueryReportCoverage {
                returned_count: 0,
                matched_count: 2,
            }
        );
        assert_eq!(
            report.limit_pressure(),
            QueryReportLimitPressure {
                truncated: true,
                omitted_count: 2,
            }
        );
    }

    #[tokio::test]
    async fn search_report_hits_match_async_search_results() {
        let store = InMemoryStore::default();
        let matching = memory_record("memory-1", MemoryScope::LongTerm, "manifest payload");
        let query = MemoryQuery {
            text: "manifest".into(),
            limit: 5,
        };

        store
            .put(matching.clone())
            .await
            .expect("put should succeed");

        let report = store
            .search_report(query.clone())
            .expect("search report should succeed");
        let hits = store.search(query).await.expect("search should succeed");

        assert_eq!(report.matched_count, 1);
        assert_eq!(report.returned_count, 1);
        assert!(!report.truncated);
        assert_eq!(report.hits, hits);
        assert_eq!(report.hits, vec![matching]);
    }

    #[tokio::test]
    async fn memory_report_store_trait_matches_inherent_search_report() {
        assert_memory_report_store::<InMemoryStore>();

        let store = InMemoryStore::default();
        let record = memory_record("memory-1", MemoryScope::LongTerm, "manifest payload");
        let query = MemoryQuery {
            text: "manifest".into(),
            limit: 5,
        };

        store.put(record.clone()).await.expect("put should succeed");

        let inherent = store
            .search_report(query.clone())
            .expect("inherent search report should succeed");
        let via_trait = <InMemoryStore as MemoryReportStore>::search_report(&store, query)
            .await
            .expect("trait search report should succeed");

        assert_eq!(via_trait, inherent);
        assert_eq!(via_trait.hits, vec![record]);
        assert_eq!(via_trait.returned_count, 1);
    }

    #[tokio::test]
    async fn transcript_query_filters_by_session_tracks_counts_and_honors_limit() {
        let store = InMemoryStore::default();
        let matching_a = transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "approval requested",
        );
        let matching_b = transcript_entry(
            "session-1",
            2,
            TranscriptEntryKind::Approval,
            "approval granted",
        );
        let other_session = transcript_entry(
            "session-2",
            1,
            TranscriptEntryKind::Message,
            "approval elsewhere",
        );

        store
            .append(matching_a.clone())
            .await
            .expect("append should succeed");
        store
            .append(matching_b.clone())
            .await
            .expect("append should succeed");
        store
            .append(other_session)
            .await
            .expect("append should succeed");

        let report = store
            .query(TranscriptQuery {
                session_id: Some(SessionId("session-1".into())),
                text: "approval".into(),
                limit: 1,
            })
            .await
            .expect("query should succeed");

        assert_eq!(report.query.session_id, Some(SessionId("session-1".into())));
        assert_eq!(report.matched_count, 2);
        assert_eq!(report.returned_count, 1);
        assert!(report.truncated);
        assert_eq!(report.hits.len(), 1);
        assert_eq!(report.hits[0], TranscriptSpan::from_entry(matching_a));
    }

    #[tokio::test]
    async fn transcript_search_report_returns_single_entry_spans() {
        let store = InMemoryStore::default();
        let transcript = transcript_entry(
            "session-1",
            7,
            TranscriptEntryKind::ToolResult,
            "manifest export complete",
        );

        store
            .append(transcript.clone())
            .await
            .expect("append should succeed");

        let report = store
            .transcript_search_report(TranscriptQuery {
                session_id: None,
                text: "manifest".into(),
                limit: 5,
            })
            .expect("search report should succeed");

        assert_eq!(report.matched_count, 1);
        assert_eq!(report.returned_count, 1);
        assert!(!report.truncated);
        assert_eq!(report.hits, vec![TranscriptSpan::from_entry(transcript)]);
    }

    #[tokio::test]
    async fn transcript_search_coverage_and_limit_pressure_match_report_helpers() {
        let store = InMemoryStore::default();
        let matching_a = transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "approval requested",
        );
        let matching_b = transcript_entry(
            "session-1",
            2,
            TranscriptEntryKind::Approval,
            "approval granted",
        );
        let query = TranscriptQuery {
            session_id: Some(SessionId("session-1".into())),
            text: "approval".into(),
            limit: 1,
        };

        store
            .append(matching_a.clone())
            .await
            .expect("append should succeed");
        store
            .append(matching_b.clone())
            .await
            .expect("append should succeed");

        let snapshot = store.snapshot().expect("snapshot should load");
        let report = store
            .transcript_search_report(query.clone())
            .expect("transcript search report should succeed");

        assert_eq!(snapshot.transcript_search_report(&query), report);
        assert_eq!(
            snapshot.transcript_search_coverage(&query),
            QueryReportCoverage {
                returned_count: 1,
                matched_count: 2,
            }
        );
        assert_eq!(
            store
                .transcript_search_coverage(query.clone())
                .expect("transcript search coverage should succeed"),
            report.coverage()
        );
        assert_eq!(
            snapshot.transcript_search_limit_pressure(&query),
            QueryReportLimitPressure {
                truncated: true,
                omitted_count: 1,
            }
        );
        assert_eq!(
            store
                .transcript_search_limit_pressure(query)
                .expect("transcript search limit pressure should succeed"),
            report.limit_pressure()
        );
    }

    #[tokio::test]
    async fn transcript_search_report_with_zero_limit_preserves_match_counts_and_omissions() {
        let store = InMemoryStore::default();
        let query = TranscriptQuery {
            session_id: Some(SessionId("session-1".into())),
            text: "approval".into(),
            limit: 0,
        };

        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "approval requested",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Approval,
                "approval granted",
            ))
            .await
            .expect("append should succeed");

        let report = store
            .transcript_search_report(query.clone())
            .expect("transcript search report should succeed");

        assert_eq!(report.query, query);
        assert_eq!(report.matched_count, 2);
        assert_eq!(report.returned_count, 0);
        assert!(report.truncated);
        assert!(report.hits.is_empty());
        assert_eq!(report.omitted_count(), 2);
        assert!(!report.is_complete());
        assert_eq!(
            report.coverage(),
            QueryReportCoverage {
                returned_count: 0,
                matched_count: 2,
            }
        );
        assert_eq!(
            report.limit_pressure(),
            QueryReportLimitPressure {
                truncated: true,
                omitted_count: 2,
            }
        );
    }

    #[tokio::test]
    async fn inspected_snapshot_matches_store_audit_report() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Foundation",
                Some("inspect live store"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "inspection payload",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "inspection summary",
            ))
            .await
            .expect("append should succeed");

        let inspected = store
            .inspected_snapshot()
            .expect("inspected snapshot should load");

        assert_eq!(
            inspected.snapshot,
            store.snapshot().expect("snapshot should load")
        );
        assert_eq!(
            inspected.audit_report(),
            store
                .snapshot_audit_report()
                .expect("audit report should load")
        );
        assert_eq!(
            inspected.issue_summary(),
            store
                .snapshot_issue_summary()
                .expect("issue summary should load")
        );
        assert!(inspected.is_clean());
    }

    #[tokio::test]
    async fn snapshot_stats_follow_store_updates() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Foundation",
                Some("sync stats"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "contract scaffolding",
            ))
            .await
            .expect("put should succeed");

        let stats = store.snapshot_stats().expect("stats should load");

        assert_eq!(stats.session_count, 1);
        assert_eq!(stats.active_session_count, 1);
        assert_eq!(stats.archived_session_count, 0);
        assert_eq!(stats.total_memory_count, 1);
        assert_eq!(stats.session_memory_count, 0);
        assert_eq!(stats.long_term_memory_count, 1);
    }

    #[tokio::test]
    async fn snapshot_manifest_tracks_sorted_records_and_sizes() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(SessionRecord {
                archived_at_unix_ms: Some(30),
                ..session_record("session-b", "Archived foundation", None)
            })
            .expect("upsert should succeed");
        store
            .upsert_session_sync(session_record(
                "session-a",
                "Active foundation",
                Some("capture manifest"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-z",
                MemoryScope::LongTerm,
                "manifest export payload",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                "memory-a",
                MemoryScope::Session,
                "session payload",
            ))
            .await
            .expect("put should succeed");

        let manifest = store
            .snapshot_manifest()
            .expect("manifest should be available");

        assert_eq!(manifest.stats.session_count, 2);
        assert_eq!(manifest.stats.archived_session_count, 1);
        assert_eq!(manifest.sessions.len(), 2);
        assert_eq!(manifest.sessions[0].session_id.0, "session-a");
        assert_eq!(manifest.sessions[1].session_id.0, "session-b");
        assert_eq!(manifest.memories.len(), 2);
        assert_eq!(manifest.memories[0].id, "memory-a");
        assert_eq!(manifest.memories[0].content_bytes, "session payload".len());
        assert_eq!(manifest.memories[1].id, "memory-z");
        assert_eq!(
            manifest.memories[1].content_bytes,
            "manifest export payload".len()
        );
    }

    #[tokio::test]
    async fn transcript_snapshot_manifest_tracks_sorted_entries_and_sizes() {
        let store = InMemoryStore::default();
        store
            .append(transcript_entry(
                "session-z",
                3,
                TranscriptEntryKind::ToolResult,
                "tool result payload",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-a",
                2,
                TranscriptEntryKind::Summary,
                "summary payload",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-a",
                1,
                TranscriptEntryKind::Message,
                "message payload",
            ))
            .await
            .expect("append should succeed");

        let manifest = store
            .transcript_snapshot_manifest()
            .expect("transcript manifest should be available");

        assert_eq!(manifest.stats.total_entry_count, 3);
        assert_eq!(manifest.stats.session_count, 2);
        assert_eq!(manifest.entries.len(), 3);
        assert_eq!(manifest.entries[0].session_id.0, "session-a");
        assert_eq!(manifest.entries[0].sequence, 1);
        assert_eq!(manifest.entries[0].content_bytes, "message payload".len());
        assert_eq!(manifest.entries[1].sequence, 2);
        assert_eq!(manifest.entries[2].session_id.0, "session-z");
    }

    #[test]
    fn transcript_session_inventory_summarizes_sessions_ranges_and_blank_ids() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![],
            transcripts: vec![
                transcript_entry(
                    "session-z",
                    3,
                    TranscriptEntryKind::ToolResult,
                    "tool result payload",
                ),
                transcript_entry(
                    "session-a",
                    2,
                    TranscriptEntryKind::Summary,
                    "summary payload",
                ),
                transcript_entry(
                    "session-a",
                    1,
                    TranscriptEntryKind::Message,
                    "message payload",
                ),
                TranscriptEntry {
                    entry_id: "blank-session".into(),
                    session_id: SessionId("   ".into()),
                    sequence: 4,
                    kind: TranscriptEntryKind::Event,
                    role: None,
                    content: "missing session id".into(),
                    created_at_unix_ms: 104,
                    tool_name: None,
                    correlation_id: None,
                    summary_of_range: None,
                },
            ],
        };

        let inventory = snapshot.transcript_session_inventory();

        assert_eq!(inventory.total_entry_count, 4);
        assert_eq!(inventory.blank_session_id_entry_count, 1);
        assert_eq!(inventory.session_count(), 2);
        assert_eq!(inventory.inventoried_entry_count(), 3);
        assert_eq!(inventory.sessions[0].session_id.0, "session-a");
        assert_eq!(inventory.sessions[0].entry_count, 2);
        assert_eq!(inventory.sessions[0].first_sequence, 1);
        assert_eq!(inventory.sessions[0].last_sequence, 2);
        assert_eq!(inventory.sessions[0].message_count, 1);
        assert_eq!(inventory.sessions[0].summary_count, 1);
        assert_eq!(inventory.sessions[1].session_id.0, "session-z");
        assert_eq!(inventory.sessions[1].tool_result_count, 1);
    }

    #[tokio::test]
    async fn store_transcript_session_inventory_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .append(transcript_entry(
                "session-b",
                5,
                TranscriptEntryKind::ToolCall,
                "write call",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-b",
                6,
                TranscriptEntryKind::ToolResult,
                "write ok",
            ))
            .await
            .expect("append should succeed");
        store
            .append(transcript_entry(
                "session-a",
                1,
                TranscriptEntryKind::Message,
                "hello",
            ))
            .await
            .expect("append should succeed");

        let snapshot = store.snapshot().expect("snapshot should load");
        let from_store = store
            .transcript_session_inventory()
            .expect("inventory should load");

        assert_eq!(from_store, snapshot.transcript_session_inventory());
        assert_eq!(from_store.total_entry_count, 3);
        assert_eq!(from_store.session_count(), 2);
        assert_eq!(from_store.sessions[0].session_id.0, "session-a");
        assert_eq!(from_store.sessions[1].session_id.0, "session-b");
        assert_eq!(from_store.sessions[1].first_sequence, 5);
        assert_eq!(from_store.sessions[1].last_sequence, 6);
        assert_eq!(from_store.sessions[1].tool_call_count, 1);
        assert_eq!(from_store.sessions[1].tool_result_count, 1);
    }

    #[test]
    fn store_snapshot_manifest_matches_snapshot_stats() {
        let snapshot = StoreSnapshot {
            sessions: vec![SessionRecord {
                archived_at_unix_ms: Some(30),
                ..session_record("session-1", "Foundation", Some("manifest alignment"))
            }],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "manifest alignment payload",
            )],
            transcripts: vec![],
        };

        let manifest = snapshot.manifest();

        assert_eq!(manifest.stats, snapshot.stats());
        assert_eq!(manifest.sessions[0].title, "Foundation");
        assert_eq!(manifest.memories[0].id, "memory-1");
        assert_eq!(
            manifest.memories[0].content_bytes,
            "manifest alignment payload".len()
        );
    }

    #[tokio::test]
    async fn snapshot_integrity_report_flags_duplicate_and_blank_records() {
        let store = InMemoryStore::default();

        store
            .create(session_record(
                "session-1",
                "Foundation",
                Some("audit integrity"),
            ))
            .await
            .expect("create should succeed");
        store
            .create(session_record("session-1", "   ", None))
            .await
            .expect("create should succeed");
        store
            .create(session_record("   ", "Blank session id", None))
            .await
            .expect("create should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "snapshot payload",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record("memory-1", MemoryScope::LongTerm, "   "))
            .await
            .expect("put should succeed");
        store
            .put(memory_record(
                " ",
                MemoryScope::LongTerm,
                "manifest payload",
            ))
            .await
            .expect("put should succeed");

        let report = store
            .snapshot_integrity_report()
            .expect("integrity report should load");

        assert_eq!(
            report.duplicate_session_ids,
            vec![SessionId("session-1".into())]
        );
        assert_eq!(report.duplicate_memory_ids, vec!["memory-1".to_string()]);
        assert_eq!(report.blank_session_id_count, 1);
        assert_eq!(report.blank_memory_id_count, 1);
        assert_eq!(report.blank_session_title_count, 1);
        assert_eq!(report.blank_memory_content_count, 1);
        assert_eq!(report.issue_count(), 6);
        assert!(!report.is_clean());
    }

    #[tokio::test]
    async fn transcript_snapshot_integrity_report_flags_duplicate_and_blank_entries() {
        let store = InMemoryStore::default();

        store
            .append(TranscriptEntry {
                entry_id: "entry-1".into(),
                session_id: SessionId("session-1".into()),
                sequence: 1,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: "hello".into(),
                created_at_unix_ms: 1,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            })
            .await
            .expect("append should succeed");
        store
            .append(TranscriptEntry {
                entry_id: "entry-1".into(),
                session_id: SessionId("session-1".into()),
                sequence: 1,
                kind: TranscriptEntryKind::ToolResult,
                role: Some(MessageRole::Tool),
                content: "result".into(),
                created_at_unix_ms: 2,
                tool_name: Some("write".into()),
                correlation_id: None,
                summary_of_range: None,
            })
            .await
            .expect("append should succeed");
        store
            .append(TranscriptEntry {
                entry_id: "   ".into(),
                session_id: SessionId("   ".into()),
                sequence: 2,
                kind: TranscriptEntryKind::Event,
                role: None,
                content: "   ".into(),
                created_at_unix_ms: 3,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            })
            .await
            .expect("append should succeed");

        let report = store
            .transcript_snapshot_integrity_report()
            .expect("transcript integrity report should load");

        assert_eq!(report.duplicate_entry_ids, vec!["entry-1".to_string()]);
        assert_eq!(report.blank_entry_id_count, 1);
        assert_eq!(report.blank_session_id_count, 1);
        assert_eq!(report.blank_content_count, 1);
        assert_eq!(report.duplicate_sequence_collisions.len(), 1);
        assert_eq!(
            report.duplicate_sequence_collisions[0].session_id.0,
            "session-1"
        );
        assert_eq!(report.duplicate_sequence_collisions[0].sequence, 1);
        assert_eq!(report.issue_count(), 5);
        assert!(!report.is_clean());
    }

    #[test]
    fn store_snapshot_integrity_report_matches_clean_snapshot() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("manifest alignment"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "manifest alignment payload",
            )],
            transcripts: vec![],
        };

        let report = snapshot.integrity_report();

        assert_eq!(report, MemorySnapshotIntegrityReport::default());
        assert!(report.is_clean());
    }

    #[test]
    fn store_snapshot_transcript_helpers_match_clean_snapshot() {
        let snapshot = StoreSnapshot {
            sessions: vec![],
            memories: vec![],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "clean transcript summary",
            )],
        };

        let stats = snapshot.transcript_stats();
        let manifest = snapshot.transcript_manifest();
        let report = snapshot.transcript_integrity_report();

        assert_eq!(stats.total_entry_count, 1);
        assert_eq!(manifest.stats, stats);
        assert_eq!(manifest.entries.len(), 1);
        assert_eq!(manifest.entries[0].entry_id, "session-1-1");
        assert_eq!(report, TranscriptSnapshotIntegrityReport::default());
        assert!(report.is_clean());
    }

    #[test]
    fn store_snapshot_audit_report_matches_clean_snapshot() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("combined audit"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot audit payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "clean transcript summary",
            )],
        };

        let report = snapshot.audit_report();

        assert_eq!(report.memory_stats, snapshot.stats());
        assert_eq!(report.transcript_stats, snapshot.transcript_stats());
        assert_eq!(report.memory_integrity, snapshot.integrity_report());
        assert_eq!(
            report.transcript_integrity,
            snapshot.transcript_integrity_report()
        );
        assert_eq!(report.issue_summary(), snapshot.issue_summary());
        assert_eq!(report.issue_count(), 0);
        assert!(report.is_clean());
    }

    #[test]
    fn store_snapshot_issue_summary_matches_audit_and_inspection_helpers() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                " ",
                Some("issue summary alignment"),
            )],
            memories: vec![memory_record("memory-1", MemoryScope::LongTerm, "   ")],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "   ",
            )],
        };

        let summary = snapshot.issue_summary();

        assert_eq!(summary, snapshot.audit_report().issue_summary());
        assert_eq!(summary, snapshot.inspection_bundle().issue_summary());
        assert_eq!(
            summary,
            SnapshotIssueSummary {
                memory_issue_count: 2,
                transcript_issue_count: 1,
                total_issue_count: 3,
                issue_domain_count: 2,
            }
        );
        assert!(summary.touches_memory());
        assert!(summary.touches_transcripts());
        assert!(summary.has_issues());
        assert!(!summary.is_clean());
    }

    #[test]
    fn store_snapshot_inspection_bundle_matches_snapshot_helpers() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("inspection bundle"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot audit payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "clean transcript summary",
            )],
        };

        let bundle = snapshot.inspection_bundle();

        assert_eq!(bundle.memory_manifest, snapshot.manifest());
        assert_eq!(bundle.memory_integrity, snapshot.integrity_report());
        assert_eq!(bundle.transcript_manifest, snapshot.transcript_manifest());
        assert_eq!(
            bundle.transcript_integrity,
            snapshot.transcript_integrity_report()
        );
        assert_eq!(bundle.issue_count(), 0);
        assert!(bundle.is_clean());
    }

    #[test]
    fn store_snapshot_inspection_match_helper_tracks_alignment() {
        let snapshot = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("inspection alignment"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot audit payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "clean transcript summary",
            )],
        };

        let canonical = snapshot.inspection_bundle();
        let drifted = SnapshotInspectionBundle {
            memory_manifest: MemorySnapshotManifest::default(),
            ..canonical.clone()
        };

        assert!(snapshot.inspection_matches(&canonical));
        assert_eq!(
            snapshot.inspection_matches(&canonical),
            canonical.matches_records_and_entries(
                &snapshot.sessions,
                &snapshot.memories,
                &snapshot.transcripts,
            )
        );
        assert!(!snapshot.inspection_matches(&drifted));
    }

    #[test]
    fn store_snapshot_inspection_drift_helpers_match_snapshot_helpers() {
        let store = InMemoryStore::default();
        store
            .restore(StoreSnapshot {
                sessions: vec![session_record(
                    "session-1",
                    "Foundation",
                    Some("store drift helper alignment"),
                )],
                memories: vec![memory_record(
                    "memory-1",
                    MemoryScope::LongTerm,
                    "snapshot payload",
                )],
                transcripts: vec![transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Summary,
                    "clean transcript summary",
                )],
            })
            .expect("restore should succeed");

        let snapshot = store.snapshot().expect("snapshot should load");
        let canonical = snapshot.inspection_bundle();
        let drifted = SnapshotInspectionBundle {
            transcript_manifest: TranscriptSnapshotManifest::default(),
            ..canonical.clone()
        };

        assert!(
            store
                .snapshot_inspection_matches(&canonical)
                .expect("canonical inspection match should succeed")
        );
        assert!(snapshot.inspection_matches(&canonical));
        assert!(
            !store
                .snapshot_inspection_matches(&drifted)
                .expect("drifted inspection match should succeed")
        );
        assert_eq!(
            store
                .snapshot_inspection_drift_report(&drifted)
                .expect("drift report should succeed"),
            snapshot.inspection_drift_report(&drifted)
        );
        assert_eq!(
            store
                .snapshot_inspection_drift_impact(&drifted)
                .expect("drift impact should succeed"),
            snapshot.inspection_drift_impact(&drifted)
        );
        assert_eq!(
            store
                .snapshot_inspection_health(&drifted)
                .expect("inspection health should succeed"),
            snapshot.inspection_health(&drifted)
        );
        assert_eq!(
            store
                .snapshot_inspection_drift_impact(&canonical)
                .expect("canonical drift impact should succeed"),
            SnapshotInspectionDriftImpact::default()
        );
        assert!(
            store
                .snapshot_inspection_health(&canonical)
                .expect("canonical inspection health should succeed")
                .is_ready()
        );
    }

    #[test]
    fn store_snapshot_restore_preview_matches_core_report() {
        let current = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Current title",
                Some("current"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "current payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "current summary",
            )],
        };
        let incoming = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Updated title",
                Some("incoming"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "incoming payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "incoming summary",
            )],
        };

        let preview = incoming.restore_preview_against(&current);

        assert_eq!(
            preview,
            SnapshotRestorePreview::from_records_and_entries(
                &current.sessions,
                &current.memories,
                &current.transcripts,
                &incoming.sessions,
                &incoming.memories,
                &incoming.transcripts,
            )
        );
        assert_eq!(
            preview.session_delta.updated_session_ids,
            vec![SessionId("session-1".into())]
        );
        assert_eq!(
            preview.memory_delta.updated_memory_ids,
            vec!["memory-1".to_string()]
        );
        assert_eq!(
            preview.transcript_delta.updated_entry_ids,
            vec!["session-1-1".to_string()]
        );
        assert_eq!(
            incoming.restore_changed_domains_against(&current),
            vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
                SnapshotRestoreDomain::Transcripts,
            ]
        );
        assert_eq!(preview.changed_domain_count(), 3);
    }

    #[test]
    fn store_snapshot_restore_impact_matches_preview_impact() {
        let current = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Current title",
                Some("current"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "current payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "current summary",
            )],
        };
        let incoming = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Updated title",
                Some("incoming"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "incoming payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "incoming summary",
            )],
        };

        let impact = incoming.restore_impact_against(&current);
        let preview = incoming.restore_preview_against(&current);

        assert_eq!(impact, preview.impact());
        assert_eq!(
            impact.domain_impacts,
            vec![
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Sessions,
                    counts: RestoreDeltaCounts {
                        added_count: 0,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 0,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Memories,
                    counts: RestoreDeltaCounts {
                        added_count: 0,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 0,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Transcripts,
                    counts: RestoreDeltaCounts {
                        added_count: 0,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 0,
                    },
                },
            ]
        );
        assert_eq!(impact.changed_domain_count(), 3);
        assert!(impact.touches(SnapshotRestoreDomain::Sessions));
        assert!(impact.touches(SnapshotRestoreDomain::Memories));
        assert!(impact.touches(SnapshotRestoreDomain::Transcripts));
        assert_eq!(impact.change_count(), 3);
        assert!(!impact.is_noop());
    }

    #[test]
    fn store_snapshot_restore_mutation_profile_matches_preview_and_impact_helpers() {
        let current = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Current title",
                Some("current"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "current payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "current summary",
            )],
        };
        let incoming = StoreSnapshot {
            sessions: vec![
                session_record("session-1", "Current title", Some("current")),
                session_record("session-2", "Added title", Some("incoming")),
            ],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "incoming payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "current summary",
            )],
        };

        let profile = incoming.restore_mutation_profile_against(&current);
        let preview = incoming.restore_preview_against(&current);

        assert_eq!(profile, preview.mutation_profile());
        assert_eq!(
            profile,
            incoming.restore_impact_against(&current).mutation_profile()
        );
        assert_eq!(
            profile,
            SnapshotRestoreMutationProfile {
                changed_domain_count: 2,
                unchanged_domain_count: 1,
                addition_domain_count: 1,
                additive_only_domain_count: 1,
                existing_record_domain_count: 1,
                removal_domain_count: 0,
                current_issue_count: 0,
                incoming_issue_count: 0,
            }
        );
        assert!(profile.has_changes());
        assert!(profile.has_additive_domains());
        assert!(profile.touches_existing_records());
        assert!(!profile.has_removals());
        assert!(!profile.is_additive_only());
        assert!(profile.is_ready());
    }

    #[test]
    fn store_snapshot_restore_readiness_matches_preview_and_impact_helpers() {
        let current = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Current title",
                Some("current"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "current payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "current summary",
            )],
        };
        let incoming = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Updated title",
                Some("incoming"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "incoming payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "incoming summary",
            )],
        };

        let readiness = incoming.restore_readiness_against(&current);
        let preview = incoming.restore_preview_against(&current);

        assert_eq!(readiness, preview.readiness());
        assert_eq!(
            readiness,
            incoming.restore_impact_against(&current).readiness()
        );
        assert_eq!(
            readiness,
            SnapshotRestoreReadiness {
                change_totals: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 3,
                    unchanged_count: 0,
                },
                changed_domain_count: 3,
                current_issue_count: 0,
                incoming_issue_count: 0,
            }
        );
        assert_eq!(readiness.change_count(), 3);
        assert!(readiness.has_changes());
        assert!(!readiness.has_integrity_issues());
        assert!(!readiness.is_noop());
        assert!(readiness.is_ready());

        let safety = incoming.restore_safety_against(&current);

        assert_eq!(safety, preview.safety());
        assert_eq!(safety, incoming.restore_impact_against(&current).safety());
        assert_eq!(safety, readiness.safety());
        assert_eq!(
            safety,
            SnapshotRestoreSafety {
                change_totals: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 3,
                    unchanged_count: 0,
                },
                changed_domain_count: 3,
                current_issue_count: 0,
                incoming_issue_count: 0,
                has_changes: true,
                touches_existing_records: true,
                additive_only: false,
                has_integrity_issues: false,
            }
        );
        assert_eq!(safety.change_count(), 3);
        assert!(safety.touches_existing_records);
        assert!(!safety.additive_only);
        assert!(safety.is_ready());
    }

    #[tokio::test]
    async fn preview_restore_safety_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Current foundation",
                Some("preview restore safety"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "current payload",
            ))
            .await
            .expect("put should succeed");

        let incoming = StoreSnapshot {
            sessions: vec![
                session_record("session-1", "Updated foundation", Some("incoming")),
                session_record("session-2", "Added foundation", None),
            ],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "updated payload",
            )],
            transcripts: vec![],
        };

        let from_store = store
            .preview_restore_safety(&incoming)
            .expect("restore safety should succeed");
        let current = store.snapshot().expect("snapshot should load");

        assert_eq!(from_store, incoming.restore_safety_against(&current));
        assert_eq!(
            from_store,
            incoming.restore_preview_against(&current).safety()
        );
        assert_eq!(from_store.change_count(), 3);
        assert!(from_store.has_changes);
        assert!(from_store.touches_existing_records);
        assert!(!from_store.additive_only);
        assert!(!from_store.has_integrity_issues);
        assert!(from_store.is_ready());
    }

    #[tokio::test]
    async fn preview_restore_mutation_profile_matches_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Current foundation",
                Some("preview restore mutation profile"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "current payload",
            ))
            .await
            .expect("put should succeed");

        let incoming = StoreSnapshot {
            sessions: vec![
                session_record(
                    "session-1",
                    "Current foundation",
                    Some("preview restore mutation profile"),
                ),
                session_record("session-2", "Added foundation", Some("incoming")),
            ],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::Session,
                "updated payload",
            )],
            transcripts: vec![],
        };
        let current = store.snapshot().expect("snapshot should load");

        let from_store = store
            .preview_restore_mutation_profile(&incoming)
            .expect("restore mutation profile should succeed");

        assert_eq!(
            from_store,
            incoming.restore_mutation_profile_against(&current)
        );
        assert_eq!(
            from_store,
            SnapshotRestoreMutationProfile {
                changed_domain_count: 2,
                unchanged_domain_count: 1,
                addition_domain_count: 1,
                additive_only_domain_count: 1,
                existing_record_domain_count: 1,
                removal_domain_count: 0,
                current_issue_count: 0,
                incoming_issue_count: 0,
            }
        );
        assert!(from_store.has_changes());
        assert!(from_store.has_additive_domains());
        assert!(from_store.touches_existing_records());
        assert!(!from_store.has_removals());
        assert!(!from_store.is_additive_only());
    }

    #[tokio::test]
    async fn preview_restore_domain_impacts_match_snapshot_helper() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Current foundation",
                Some("preview restore domains"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "current payload",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "current message",
            ))
            .await
            .expect("append should succeed");

        let incoming = StoreSnapshot {
            sessions: vec![
                session_record("session-1", "Updated foundation", Some("incoming")),
                session_record("session-2", "Added foundation", None),
            ],
            memories: vec![
                memory_record("memory-1", MemoryScope::Session, "updated payload"),
                memory_record("memory-2", MemoryScope::LongTerm, "added payload"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "updated message",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Summary,
                    "added summary",
                ),
            ],
        };

        let from_store = store
            .preview_restore_domain_impacts(&incoming)
            .expect("restore domain impacts should succeed");

        assert_eq!(
            from_store,
            incoming
                .restore_domain_impacts_against(&store.snapshot().expect("snapshot should load"))
        );
        assert_eq!(
            from_store,
            vec![
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Sessions,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 0,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Memories,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 0,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Transcripts,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 0,
                    },
                },
            ]
        );

        let changed_domains = store
            .preview_restore_changed_domains(&incoming)
            .expect("restore changed domains should succeed");

        assert_eq!(
            changed_domains,
            incoming
                .restore_changed_domains_against(&store.snapshot().expect("snapshot should load"))
        );
        assert_eq!(
            changed_domains,
            vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
                SnapshotRestoreDomain::Transcripts,
            ]
        );
    }

    #[tokio::test]
    async fn snapshot_audit_report_tracks_memory_and_transcript_issues_together() {
        let store = InMemoryStore::default();

        store
            .create(session_record(
                "session-1",
                "Foundation",
                Some("audit combined snapshot"),
            ))
            .await
            .expect("create should succeed");
        store
            .create(session_record("session-1", "   ", None))
            .await
            .expect("create should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "manifest payload",
            ))
            .await
            .expect("put should succeed");
        store
            .put(memory_record("memory-1", MemoryScope::Session, "   "))
            .await
            .expect("put should succeed");
        store
            .append(TranscriptEntry {
                entry_id: "entry-1".into(),
                session_id: SessionId("session-1".into()),
                sequence: 1,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: "hello".into(),
                created_at_unix_ms: 1,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            })
            .await
            .expect("append should succeed");
        store
            .append(TranscriptEntry {
                entry_id: "entry-1".into(),
                session_id: SessionId("session-1".into()),
                sequence: 1,
                kind: TranscriptEntryKind::ToolResult,
                role: Some(MessageRole::Tool),
                content: "result".into(),
                created_at_unix_ms: 2,
                tool_name: Some("write".into()),
                correlation_id: None,
                summary_of_range: None,
            })
            .await
            .expect("append should succeed");

        let report = store
            .snapshot_audit_report()
            .expect("audit report should load");
        let summary = store
            .snapshot_issue_summary()
            .expect("issue summary should load");
        let inspection = store
            .snapshot_inspection_bundle()
            .expect("inspection bundle should load");

        assert_eq!(report.memory_stats.session_count, 2);
        assert_eq!(report.memory_stats.total_memory_count, 2);
        assert_eq!(report.transcript_stats.total_entry_count, 2);
        assert_eq!(report.memory_integrity.issue_count(), 4);
        assert_eq!(report.transcript_integrity.issue_count(), 2);
        assert_eq!(report.memory_issue_count(), 4);
        assert_eq!(report.transcript_issue_count(), 2);
        assert_eq!(report.issue_count(), 6);
        assert_eq!(report.issue_domain_count(), 2);
        assert!(report.touches_memory());
        assert!(report.touches_transcripts());
        assert!(!report.is_clean());

        assert_eq!(summary, report.issue_summary());
        assert_eq!(summary, inspection.issue_summary());
        assert_eq!(summary.memory_issue_count, 4);
        assert_eq!(summary.transcript_issue_count, 2);
        assert_eq!(summary.total_issue_count, 6);
        assert_eq!(summary.issue_domain_count, 2);
        assert!(summary.has_issues());
        assert!(!summary.is_clean());

        assert_eq!(inspection.memory_manifest.stats, report.memory_stats);
        assert_eq!(inspection.memory_integrity, report.memory_integrity);
        assert_eq!(
            inspection.transcript_manifest.stats,
            report.transcript_stats
        );
        assert_eq!(inspection.transcript_integrity, report.transcript_integrity);
        assert_eq!(inspection.memory_issue_count(), 4);
        assert_eq!(inspection.transcript_issue_count(), 2);
        assert_eq!(inspection.issue_count(), 6);
        assert_eq!(inspection.issue_domain_count(), 2);
        assert!(inspection.touches_memory());
        assert!(inspection.touches_transcripts());
        assert!(!inspection.is_clean());
    }

    #[tokio::test]
    async fn preview_restore_summarizes_replace_style_changes_before_restore() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Current foundation",
                Some("preview restore"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "current payload",
            ))
            .await
            .expect("put should succeed");
        store
            .append(transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "current message",
            ))
            .await
            .expect("append should succeed");

        let incoming = StoreSnapshot {
            sessions: vec![
                session_record("session-1", "Updated foundation", Some("incoming")),
                session_record("session-2", "Added foundation", None),
            ],
            memories: vec![
                memory_record("memory-1", MemoryScope::Session, "updated payload"),
                memory_record("memory-2", MemoryScope::LongTerm, "added payload"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-1",
                    1,
                    TranscriptEntryKind::Message,
                    "updated message",
                ),
                transcript_entry(
                    "session-2",
                    1,
                    TranscriptEntryKind::Summary,
                    "added summary",
                ),
            ],
        };

        let preview = store
            .preview_restore(&incoming)
            .expect("restore preview should succeed");

        assert_eq!(
            preview.session_delta.added_session_ids,
            vec![SessionId("session-2".into())]
        );
        assert_eq!(
            preview.session_delta.updated_session_ids,
            vec![SessionId("session-1".into())]
        );
        assert_eq!(
            preview.memory_delta.added_memory_ids,
            vec!["memory-2".to_string()]
        );
        assert_eq!(
            preview.memory_delta.updated_memory_ids,
            vec!["memory-1".to_string()]
        );
        assert_eq!(
            preview.transcript_delta.added_entry_ids,
            vec!["session-2-1".to_string()]
        );
        assert_eq!(
            preview.transcript_delta.updated_entry_ids,
            vec!["session-1-1".to_string()]
        );
        assert_eq!(
            preview.session_delta.counts(),
            RestoreDeltaCounts {
                added_count: 1,
                removed_count: 0,
                updated_count: 1,
                unchanged_count: 0,
            }
        );
        assert_eq!(
            preview.memory_delta.counts(),
            RestoreDeltaCounts {
                added_count: 1,
                removed_count: 0,
                updated_count: 1,
                unchanged_count: 0,
            }
        );
        assert_eq!(
            preview.transcript_delta.counts(),
            RestoreDeltaCounts {
                added_count: 1,
                removed_count: 0,
                updated_count: 1,
                unchanged_count: 0,
            }
        );
        assert_eq!(
            preview.change_totals(),
            RestoreDeltaCounts {
                added_count: 3,
                removed_count: 0,
                updated_count: 3,
                unchanged_count: 0,
            }
        );
        assert_eq!(preview.change_count(), 6);
        assert!(!preview.is_noop());
        assert!(!preview.has_integrity_issues());
    }

    #[tokio::test]
    async fn preview_restore_impact_compacts_store_restore_summary() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Current foundation",
                Some("preview restore impact"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "current payload",
            ))
            .await
            .expect("put should succeed");

        let incoming = StoreSnapshot {
            sessions: vec![session_record(
                "session-2",
                "Added foundation",
                Some("incoming"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "updated payload",
            )],
            transcripts: vec![transcript_entry(
                "session-2",
                1,
                TranscriptEntryKind::Summary,
                "added summary",
            )],
        };

        let impact = store
            .preview_restore_impact(&incoming)
            .expect("restore impact should succeed");
        let preview = store
            .preview_restore(&incoming)
            .expect("restore preview should succeed");

        assert_eq!(impact, preview.impact());
        assert_eq!(
            impact.changed_domains,
            vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
                SnapshotRestoreDomain::Transcripts,
            ]
        );
        assert_eq!(
            impact.change_totals,
            RestoreDeltaCounts {
                added_count: 2,
                removed_count: 1,
                updated_count: 1,
                unchanged_count: 0,
            }
        );
        assert_eq!(impact.change_count(), 4);
        assert_eq!(impact.total_issue_count(), 0);
        assert!(!impact.has_integrity_issues());
        assert!(!impact.is_noop());
    }

    #[tokio::test]
    async fn preview_restore_readiness_compacts_store_restore_summary() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Current foundation",
                Some("preview restore readiness"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "current payload",
            ))
            .await
            .expect("put should succeed");

        let incoming = StoreSnapshot {
            sessions: vec![session_record(
                "session-2",
                "Added foundation",
                Some("incoming"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "updated payload",
            )],
            transcripts: vec![transcript_entry(
                "session-2",
                1,
                TranscriptEntryKind::Summary,
                "added summary",
            )],
        };

        let readiness = store
            .preview_restore_readiness(&incoming)
            .expect("restore readiness should succeed");
        let preview = store
            .preview_restore(&incoming)
            .expect("restore preview should succeed");

        assert_eq!(readiness, preview.readiness());
        assert_eq!(
            readiness,
            incoming.restore_readiness_against(&store.snapshot().expect("snapshot should load"))
        );
        assert_eq!(
            readiness.change_totals,
            RestoreDeltaCounts {
                added_count: 2,
                removed_count: 1,
                updated_count: 1,
                unchanged_count: 0,
            }
        );
        assert_eq!(readiness.changed_domain_count, 3);
        assert_eq!(readiness.change_count(), 4);
        assert_eq!(readiness.total_issue_count(), 0);
        assert!(readiness.has_changes());
        assert!(!readiness.has_integrity_issues());
        assert!(!readiness.is_noop());
        assert!(readiness.is_ready());
    }

    #[test]
    fn store_snapshot_restore_helpers_flag_additive_only_previews() {
        let current = StoreSnapshot {
            sessions: vec![],
            memories: vec![],
            transcripts: vec![],
        };
        let incoming = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Added foundation",
                Some("additive restore"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "added payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "added summary",
            )],
        };
        let inspected = InspectedStoreSnapshot::from_snapshot(incoming.clone());

        assert!(incoming.restore_is_additive_only_against(&current));
        assert!(!incoming.restore_touches_existing_records_against(&current));
        assert!(inspected.restore_is_additive_only_against(&current));
        assert!(!inspected.restore_touches_existing_records_against(&current));
    }

    #[tokio::test]
    async fn store_preview_restore_helpers_detect_existing_record_changes() {
        let store = InMemoryStore::default();
        store
            .upsert_session_sync(session_record(
                "session-1",
                "Current foundation",
                Some("existing session"),
            ))
            .expect("upsert should succeed");
        store
            .put(memory_record(
                "memory-1",
                MemoryScope::Session,
                "current payload",
            ))
            .await
            .expect("put should succeed");

        let incoming = StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Updated foundation",
                Some("updated session"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "updated payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "added summary",
            )],
        };
        let current = store.snapshot().expect("snapshot should load");
        let inspected = InspectedStoreSnapshot::from_snapshot(incoming.clone());

        assert!(!incoming.restore_is_additive_only_against(&current));
        assert!(incoming.restore_touches_existing_records_against(&current));
        assert!(!inspected.restore_is_additive_only_against(&current));
        assert!(inspected.restore_touches_existing_records_against(&current));
        assert!(
            !store
                .preview_restore_is_additive_only(&incoming)
                .expect("restore additive-only helper should succeed")
        );
        assert!(
            store
                .preview_restore_touches_existing_records(&incoming)
                .expect("restore existing-record helper should succeed")
        );
    }

    #[test]
    fn sync_listing_helpers_surface_memories_and_transcripts() {
        let store = InMemoryStore::default();
        let memory = memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "listed through sync helper",
        );
        let transcript = transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "appended through sync helper",
        );

        store
            .restore(StoreSnapshot {
                sessions: vec![session_record(
                    "session-1",
                    "Foundation",
                    Some("sync listing helpers"),
                )],
                memories: vec![memory.clone()],
                transcripts: vec![],
            })
            .expect("restore should succeed");
        store
            .append_transcript_sync(transcript.clone())
            .expect("sync transcript append should succeed");

        assert_eq!(
            store.list_memories().expect("memory list should load"),
            vec![memory]
        );
        assert_eq!(
            store
                .list_transcript_entries()
                .expect("transcript list should load"),
            vec![transcript]
        );
        assert_eq!(
            store.snapshot().expect("snapshot should load").transcripts,
            store
                .list_transcript_entries()
                .expect("transcript list should load")
        );
    }

    #[test]
    fn memory_context_safety_keeps_transcript_recall_session_scoped_with_cross_session_memory() {
        let store = InMemoryStore::default();
        store
            .restore(StoreSnapshot {
                sessions: vec![
                    session_record("session-main", "Main", Some("needle main")),
                    session_record("session-other", "Other", Some("needle other")),
                ],
                memories: vec![
                    memory_record(
                        "memory-long-term",
                        MemoryScope::LongTerm,
                        "needle durable preference",
                    ),
                    memory_record(
                        "memory-summary",
                        MemoryScope::Session,
                        "needle session summary",
                    ),
                ],
                transcripts: vec![
                    transcript_entry(
                        "session-main",
                        1,
                        TranscriptEntryKind::Message,
                        "needle local transcript",
                    ),
                    transcript_entry(
                        "session-other",
                        1,
                        TranscriptEntryKind::Message,
                        "needle other transcript should stay out",
                    ),
                ],
            })
            .expect("restore should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-main".into()),
            query_text: Some("needle".into()),
            recent_window_limit: 5,
            transcript_limit: 5,
            memory_limit: 5,
            allow_cross_session: true,
        };
        let bundle = store
            .recall_context(request.clone())
            .expect("context recall should succeed");
        let report = store
            .recall_context_report(request)
            .expect("context recall report should succeed");

        assert_eq!(bundle.recent_entries.len(), 1);
        assert_eq!(bundle.transcript_hits.len(), 1);
        assert!(
            bundle
                .recent_entries
                .iter()
                .all(|entry| entry.session_id.0 == "session-main")
        );
        assert!(
            bundle
                .transcript_hits
                .iter()
                .all(|span| span.session_id.0 == "session-main")
        );
        assert_eq!(bundle.durable_memory_hits.len(), 1);
        assert_eq!(bundle.summary_hits.len(), 1);
        assert_eq!(report.source_counts.transcript_hit_count, 1);
        assert_eq!(report.source_counts.durable_memory_hit_count, 1);
        assert_eq!(report.source_counts.summary_hit_count, 1);
    }

    #[test]
    fn memory_context_safety_reports_limit_pressure_without_leaking_hidden_context() {
        let store = InMemoryStore::default();
        store
            .restore(StoreSnapshot {
                sessions: vec![session_record(
                    "session-main",
                    "Main",
                    Some("limit pressure"),
                )],
                memories: vec![
                    memory_record("memory-1", MemoryScope::LongTerm, "needle alpha"),
                    memory_record("memory-2", MemoryScope::LongTerm, "needle beta"),
                    memory_record("memory-3", MemoryScope::Session, "needle gamma"),
                ],
                transcripts: vec![
                    transcript_entry(
                        "session-main",
                        1,
                        TranscriptEntryKind::Message,
                        "needle transcript alpha",
                    ),
                    transcript_entry(
                        "session-main",
                        2,
                        TranscriptEntryKind::ToolResult,
                        "needle transcript beta",
                    ),
                ],
            })
            .expect("restore should succeed");

        let request = ContextRecallRequest {
            session_id: SessionId("session-main".into()),
            query_text: Some("needle".into()),
            recent_window_limit: 1,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: false,
        };
        let inspection = store
            .recall_context_inspection(request.clone())
            .expect("inspection should succeed");
        let pressure = store
            .recall_context_limit_pressure(request)
            .expect("pressure should succeed");

        assert!(inspection.report.truncated);
        assert!(pressure.transcript_hits_truncated);
        assert!(pressure.memory_hits_truncated);
        assert_eq!(inspection.availability.total_transcript_match_count, 2);
        assert_eq!(inspection.availability.total_memory_match_count, 3);
        assert_eq!(
            store
                .snapshot()
                .expect("snapshot should load")
                .memories
                .iter()
                .filter(|record| record.content.contains("hidden_runtime_context"))
                .count(),
            0
        );
    }
}
