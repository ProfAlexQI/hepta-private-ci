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
//! let this crate filter memory hits differently. Explicit Hepta recall control
//! records are treated as metadata, not recall payloads.
//! The offline context-memory eval harness seed and adaptive-allocator eval
//! shadow feed a recall-quality gate verdict as payload-light reports with
//! fixed synthetic/redacted fixture metrics and no production memory, graph, or
//! runtime activation side effects. A unified context-plane status report
//! stitches those read-only memory/eval surfaces into one operator-facing
//! readiness summary; its activation-blocker matrix and operator approval
//! packet explain why promotion remains blocked without enabling runtime
//! behavior. The ranked-recall shadow eval is likewise exposed as a fixed,
//! payload-light helper so readiness gates can validate recall, precision,
//! token-saved, latency, regret, and disabled runtime activation. The
//! memory shadow regression dashboard then aggregates ranked recall, temporal
//! graph, recall-quality, and provider-boundary shadow reports as a
//! payload-light quality loop without activating a production route.
//! shadow quality summary: operator-readable but controlled, exposing only
//! trend enums, aggregate signal counts, threshold observations, and
//! side-effect booleans. The shadow quality trend snapshot rolls that summary
//! into a controlled, payload-light regression window without persisting
//! history or enabling production routes. The
//! selected-recall summary canary eval replay is likewise exposed as a fixed,
//! payload-light helper so readiness gates can validate replay counts,
//! rollback-readback coverage, proof coverage, thresholds, and disabled runtime
//! activation without hard-coding a live production route.
//! The reference store also implements the Hepta-native `MemoryProvider`
//! boundary as a shadow-only provider: query returns the existing recall bundle,
//! update/report return guarded payload-light envelopes, and clear attempts are
//! dry-run or blocked reports without mutating the store.

use std::sync::Arc;
use std::sync::Mutex;

use hepta_core::ContextRecallBundle;
use hepta_core::ContextRecallRequest;
use hepta_core::MemoryProviderClearReport;
use hepta_core::MemoryProviderClearRequest;
use hepta_core::MemoryProviderContextUpdateEnvelope;
use hepta_core::MemoryProviderDescriptor;
use hepta_core::MemoryProviderReport;
use hepta_core::MemoryQuery;
use hepta_core::MemoryQueryReport;
use hepta_core::MemoryRecord;
use hepta_core::MemoryReportStore;
use hepta_core::MemoryStore;
use hepta_core::SessionId;
use hepta_core::SessionRecord;
use hepta_core::SessionStore;
use hepta_core::SnapshotInspectionBundle;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptQuery;
use hepta_core::TranscriptQueryReport;
use hepta_core::TranscriptStore;
use serde::Deserialize;
use serde::Serialize;

mod context_plane_helpers;
mod recall_helpers;
mod snapshot_helpers;

/// Small non-durable store for local development, tests, and snapshot-backed
/// runtime state.
#[derive(Default, Clone)]
pub struct InMemoryStore {
    pub(crate) state: Arc<Mutex<StoreState>>,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct StoreState {
    pub(crate) sessions: Vec<SessionRecord>,
    pub(crate) memories: Vec<MemoryRecord>,
    pub(crate) transcripts: Vec<TranscriptEntry>,
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

    pub fn list_transcript_entries(&self) -> Result<Vec<TranscriptEntry>, hepta_core::MemoryError> {
        let guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("transcript store mutex poisoned".into()))?;
        Ok(guard.transcripts.clone())
    }

    pub fn put_memory_sync(&self, record: MemoryRecord) -> Result<(), hepta_core::MemoryError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| hepta_core::MemoryError("memory store mutex poisoned".into()))?;
        guard.memories.push(record);
        Ok(())
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
        let (mut hits, _) =
            recall_helpers::memory_records_matching_recall_query(&guard.memories, &query.text);
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

impl hepta_core::MemoryProvider for InMemoryStore {
    async fn query(
        &self,
        request: ContextRecallRequest,
    ) -> Result<ContextRecallBundle, hepta_core::MemoryError> {
        self.recall_context(request)
    }

    async fn update_context(
        &self,
        request: ContextRecallRequest,
    ) -> Result<MemoryProviderContextUpdateEnvelope, hepta_core::MemoryError> {
        let bundle = self.recall_context(request.clone())?;
        let limit_pressure = self.recall_context_limit_pressure(request)?;
        Ok(MemoryProviderContextUpdateEnvelope::from_bundle(
            "builtin",
            &bundle,
            limit_pressure,
        ))
    }

    async fn report(
        &self,
        request: ContextRecallRequest,
    ) -> Result<MemoryProviderReport, hepta_core::MemoryError> {
        Ok(MemoryProviderReport::from_update(
            MemoryProviderDescriptor::builtin(),
            self.update_context(request).await?,
        ))
    }

    async fn clear(
        &self,
        request: MemoryProviderClearRequest,
    ) -> Result<MemoryProviderClearReport, hepta_core::MemoryError> {
        if request.dry_run {
            Ok(MemoryProviderClearReport::dry_run("builtin", request.scope))
        } else {
            Ok(MemoryProviderClearReport::blocked("builtin", request.scope))
        }
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
mod tests;
