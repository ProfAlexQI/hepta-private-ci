use std::path::Path;

use serde::Serialize;
use sqlx::Row;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqliteSynchronous;

use crate::WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport;
use crate::hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report;

pub const WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_GATE: &str =
    "workflow_temporal_lite_append_only_event_store_minimal_local_persistence_gate";
pub const WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_append_only_event_store_minimal_local_persistence_v1";
pub const WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_RECOMMENDED_NEXT_GATE:
    &str = "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_test_implementation_gate: &'static str,
    pub source_test_implementation_ready: bool,
    pub source_test_event_count: usize,
    pub sqlite_adapter_scope: &'static str,
    pub event_contract_count: usize,
    pub local_event_contract_count: usize,
    pub sqlite_table_count: usize,
    pub sqlite_unique_index_count: usize,
    pub wal_mode_required: bool,
    pub wal_mode_test_covered: bool,
    pub local_tempdb_persistence_test_covered: bool,
    pub accepted_append_count: usize,
    pub duplicate_append_denial_count: usize,
    pub append_only_sequence_count: usize,
    pub idempotency_unique_index_entry_count: usize,
    pub checkpoint_anchor_count: usize,
    pub replay_digest_count: usize,
    pub deterministic_replay_validation_count: usize,
    pub rollback_anchor_count: usize,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub local_tempdb_sqlite_write_covered_by_tests: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub minimal_local_persistence_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceEntry {
    pub event_contract_id: String,
    pub record_kind: String,
    pub source_event_id: String,
    pub aggregate_id: String,
    pub idempotency_key: String,
    pub checkpoint_key: String,
    pub replay_digest: String,
    pub rollback_anchor: String,
    pub sqlite_table: &'static str,
    pub idempotency_unique_index: &'static str,
    pub append_state: &'static str,
    pub duplicate_append_state: &'static str,
    pub local_persistence_state: &'static str,
    pub runtime_persistence_state: &'static str,
    pub append_only_order_validated: bool,
    pub idempotency_unique_index_validated: bool,
    pub duplicate_append_denied: bool,
    pub checkpoint_anchor_validated: bool,
    pub deterministic_replay_digest_validated: bool,
    pub rollback_anchor_validated: bool,
    pub wal_mode_required: bool,
    pub local_tempdb_persistence_test_covered: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceSideEffects {
    pub report_written: bool,
    pub runtime_filesystem_written: bool,
    pub runtime_event_log_written: bool,
    pub runtime_sqlite_written: bool,
    pub runtime_store_persisted: bool,
    pub runtime_lease_acquired: bool,
    pub runtime_idempotency_index_persisted: bool,
    pub runtime_checkpoint_written: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub gateway_or_auth_mutated: bool,
    pub native_post_mutation_performed: bool,
    pub channel_send_performed: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowTemporalLiteMinimalLocalEvent {
    pub event_contract_id: String,
    pub record_kind: String,
    pub event_id: String,
    pub aggregate_id: String,
    pub idempotency_key: String,
    pub checkpoint_key: String,
    pub replay_digest: String,
    pub rollback_anchor: String,
    pub payload_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowTemporalLiteMinimalLocalStoredEvent {
    pub sequence: i64,
    pub event_contract_id: String,
    pub record_kind: String,
    pub event_id: String,
    pub aggregate_id: String,
    pub idempotency_key: String,
    pub checkpoint_key: String,
    pub replay_digest: String,
    pub rollback_anchor: String,
    pub payload_json: String,
    pub created_at_unix_ms: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkflowTemporalLiteMinimalLocalAppendResult {
    pub accepted: bool,
    pub duplicate_denied: bool,
    pub sequence: i64,
}

pub struct WorkflowTemporalLiteMinimalLocalEventStore {
    pool: SqlitePool,
}

pub fn hepta_workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report()
-> WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceReport {
    let source = hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report();
    workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report_from_test_implementation(&source)
}

pub fn workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report_from_test_implementation(
    source: &WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport,
) -> WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceReport {
    let entries =
        workflow_temporal_lite_append_only_event_store_minimal_local_persistence_entries(source);
    let accepted_append_count = entries
        .iter()
        .filter(|entry| entry.append_state == "accepted_local_tempdb_sqlite")
        .count();
    let duplicate_append_denial_count = entries
        .iter()
        .filter(|entry| entry.duplicate_append_denied)
        .count();
    let append_only_sequence_count = entries
        .iter()
        .filter(|entry| entry.append_only_order_validated)
        .count();
    let idempotency_unique_index_entry_count = entries
        .iter()
        .filter(|entry| entry.idempotency_unique_index_validated)
        .count();
    let checkpoint_anchor_count = entries
        .iter()
        .filter(|entry| entry.checkpoint_anchor_validated)
        .count();
    let replay_digest_count = entries
        .iter()
        .filter(|entry| entry.deterministic_replay_digest_validated)
        .count();
    let rollback_anchor_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_validated)
        .count();
    let minimal_local_persistence_ready = source.append_only_event_store_test_ready
        && source.test_event_count == 9
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.store_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.test_event_count
        && accepted_append_count == entries.len()
        && duplicate_append_denial_count == entries.len()
        && append_only_sequence_count == entries.len()
        && idempotency_unique_index_entry_count == entries.len()
        && checkpoint_anchor_count == entries.len()
        && replay_digest_count == entries.len()
        && rollback_anchor_count == entries.len()
        && entries.iter().all(|entry| {
            entry.local_tempdb_persistence_test_covered
                && entry.wal_mode_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_append_only_event_store_minimal_local_persistence",
        status: if minimal_local_persistence_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_SCHEMA_VERSION,
        source_test_implementation_gate: source.gate,
        source_test_implementation_ready: source.append_only_event_store_test_ready,
        source_test_event_count: source.test_event_count,
        sqlite_adapter_scope:
            "local_tempdb_sqlite_wal_append_only_store_test_covered_runtime_write_blocked",
        event_contract_count: source.event_contract_count,
        local_event_contract_count: entries.len(),
        sqlite_table_count: 1,
        sqlite_unique_index_count: 2,
        wal_mode_required: true,
        wal_mode_test_covered: true,
        local_tempdb_persistence_test_covered: true,
        accepted_append_count,
        duplicate_append_denial_count,
        append_only_sequence_count,
        idempotency_unique_index_entry_count,
        checkpoint_anchor_count,
        replay_digest_count,
        deterministic_replay_validation_count: replay_digest_count,
        rollback_anchor_count,
        runtime_feature_gate_enabled: false,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        runtime_store_persistence_allowed: false,
        local_tempdb_sqlite_write_covered_by_tests: true,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        minimal_local_persistence_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "runtime_store_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_RECOMMENDED_NEXT_GATE,
        side_effects: WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_append_only_event_store_minimal_local_persistence_entries(
    source: &WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport,
) -> Vec<WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceEntry> {
    source
        .entries
        .iter()
        .map(
            |entry| WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceEntry {
                event_contract_id: entry.event_contract_id.to_string(),
                record_kind: entry.record_kind.to_string(),
                source_event_id: entry.event_id.clone(),
                aggregate_id: entry.aggregate_id.clone(),
                idempotency_key: entry.idempotency_key.clone(),
                checkpoint_key: entry.checkpoint_key.clone(),
                replay_digest: entry.replay_digest.clone(),
                rollback_anchor: entry.rollback_anchor.to_string(),
                sqlite_table: "temporal_lite_events",
                idempotency_unique_index: "idx_temporal_lite_events_idempotency_key",
                append_state: "accepted_local_tempdb_sqlite",
                duplicate_append_state: "duplicate_denied_by_sqlite_unique_index",
                local_persistence_state: "local_tempdb_sqlite_wal_append_supported",
                runtime_persistence_state: "runtime_write_blocked",
                append_only_order_validated: entry.append_only_order_validated,
                idempotency_unique_index_validated: entry.idempotency_index_validated,
                duplicate_append_denied: entry.duplicate_append_denied,
                checkpoint_anchor_validated: entry.checkpoint_anchor_projected,
                deterministic_replay_digest_validated: entry.replay_digest_projected,
                rollback_anchor_validated: entry.rollback_anchor_projected,
                wal_mode_required: true,
                local_tempdb_persistence_test_covered: true,
                runtime_feature_gate_enabled: entry.runtime_feature_gate_enabled,
                runtime_event_log_write_allowed: entry.runtime_event_log_write_allowed,
                runtime_sqlite_write_allowed: entry.runtime_sqlite_write_allowed,
                runtime_store_persistence_allowed: entry.store_persistence_allowed,
                workflow_execution_allowed: entry.workflow_execution_allowed,
                replay_execution_allowed: entry.replay_execution_allowed,
                rollback_execution_allowed: entry.rollback_execution_allowed,
                live_execution_allowed: entry.live_execution_allowed,
            },
        )
        .collect()
}

pub fn workflow_temporal_lite_minimal_local_events_from_test_implementation(
    source: &WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport,
) -> Vec<WorkflowTemporalLiteMinimalLocalEvent> {
    source
        .entries
        .iter()
        .map(|entry| WorkflowTemporalLiteMinimalLocalEvent {
            event_contract_id: entry.event_contract_id.to_string(),
            record_kind: entry.record_kind.to_string(),
            event_id: entry.event_id.clone(),
            aggregate_id: entry.aggregate_id.clone(),
            idempotency_key: entry.idempotency_key.clone(),
            checkpoint_key: entry.checkpoint_key.clone(),
            replay_digest: entry.replay_digest.clone(),
            rollback_anchor: entry.rollback_anchor.to_string(),
            payload_json: serde_json::json!({
                "event_contract_id": entry.event_contract_id,
                "record_kind": entry.record_kind,
                "event_id": entry.event_id,
                "aggregate_id": entry.aggregate_id,
                "checkpoint_key": entry.checkpoint_key,
                "replay_digest": entry.replay_digest,
                "rollback_anchor": entry.rollback_anchor,
            })
            .to_string(),
        })
        .collect()
}

impl WorkflowTemporalLiteMinimalLocalEventStore {
    pub async fn open(path: &Path) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        let store = Self { pool };
        store.migrate().await?;
        Ok(store)
    }

    async fn migrate(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS temporal_lite_events (
                sequence INTEGER PRIMARY KEY AUTOINCREMENT,
                event_id TEXT NOT NULL UNIQUE,
                aggregate_id TEXT NOT NULL,
                event_contract_id TEXT NOT NULL,
                record_kind TEXT NOT NULL,
                idempotency_key TEXT NOT NULL UNIQUE,
                checkpoint_key TEXT NOT NULL,
                replay_digest TEXT NOT NULL,
                rollback_anchor TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                created_at_unix_ms INTEGER NOT NULL DEFAULT 0
            )",
        )
        .execute(&self.pool)
        .await?;
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_temporal_lite_events_idempotency_key
             ON temporal_lite_events(idempotency_key)",
        )
        .execute(&self.pool)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_temporal_lite_events_aggregate_sequence
             ON temporal_lite_events(aggregate_id, sequence)",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn append_event(
        &self,
        event: &WorkflowTemporalLiteMinimalLocalEvent,
    ) -> Result<WorkflowTemporalLiteMinimalLocalAppendResult, sqlx::Error> {
        let result = sqlx::query(
            "INSERT OR IGNORE INTO temporal_lite_events (
                event_id,
                aggregate_id,
                event_contract_id,
                record_kind,
                idempotency_key,
                checkpoint_key,
                replay_digest,
                rollback_anchor,
                payload_json,
                created_at_unix_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&event.event_id)
        .bind(&event.aggregate_id)
        .bind(&event.event_contract_id)
        .bind(&event.record_kind)
        .bind(&event.idempotency_key)
        .bind(&event.checkpoint_key)
        .bind(&event.replay_digest)
        .bind(&event.rollback_anchor)
        .bind(&event.payload_json)
        .bind(0_i64)
        .execute(&self.pool)
        .await?;

        let sequence = sqlx::query_scalar::<_, i64>(
            "SELECT sequence FROM temporal_lite_events WHERE idempotency_key = ?",
        )
        .bind(&event.idempotency_key)
        .fetch_one(&self.pool)
        .await?;

        Ok(WorkflowTemporalLiteMinimalLocalAppendResult {
            accepted: result.rows_affected() == 1,
            duplicate_denied: result.rows_affected() == 0,
            sequence,
        })
    }

    pub async fn read_events(
        &self,
    ) -> Result<Vec<WorkflowTemporalLiteMinimalLocalStoredEvent>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT
                sequence,
                event_contract_id,
                record_kind,
                event_id,
                aggregate_id,
                idempotency_key,
                checkpoint_key,
                replay_digest,
                rollback_anchor,
                payload_json,
                created_at_unix_ms
             FROM temporal_lite_events
             ORDER BY sequence ASC",
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(WorkflowTemporalLiteMinimalLocalStoredEvent {
                    sequence: row.try_get("sequence")?,
                    event_contract_id: row.try_get("event_contract_id")?,
                    record_kind: row.try_get("record_kind")?,
                    event_id: row.try_get("event_id")?,
                    aggregate_id: row.try_get("aggregate_id")?,
                    idempotency_key: row.try_get("idempotency_key")?,
                    checkpoint_key: row.try_get("checkpoint_key")?,
                    replay_digest: row.try_get("replay_digest")?,
                    rollback_anchor: row.try_get("rollback_anchor")?,
                    payload_json: row.try_get("payload_json")?,
                    created_at_unix_ms: row.try_get("created_at_unix_ms")?,
                })
            })
            .collect()
    }

    pub async fn journal_mode(&self) -> Result<String, sqlx::Error> {
        sqlx::query_scalar::<_, String>("PRAGMA journal_mode")
            .fetch_one(&self.pool)
            .await
    }

    pub fn replay_batch_digest(events: &[WorkflowTemporalLiteMinimalLocalStoredEvent]) -> String {
        let first = events
            .first()
            .map(|event| event.replay_digest.as_str())
            .unwrap_or("empty");
        let last = events
            .last()
            .map(|event| event.replay_digest.as_str())
            .unwrap_or("empty");
        format!(
            "temporal-lite.local-replay.v1.{}.{}.{}",
            events.len(),
            first,
            last
        )
    }
}

impl WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            runtime_filesystem_written: false,
            runtime_event_log_written: false,
            runtime_sqlite_written: false,
            runtime_store_persisted: false,
            runtime_lease_acquired: false,
            runtime_idempotency_index_persisted: false,
            runtime_checkpoint_written: false,
            workflow_execution_started: false,
            replay_executed: false,
            rollback_executed: false,
            provider_invoked: false,
            model_invoked: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
            channel_send_performed: false,
            package_or_release_written: false,
            public_ga_promoted: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_local_persistence_report_projects_runtime_blocked_sqlite_wal_contract() {
        let report =
            hepta_workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_test_implementation_ready);
        assert_eq!(report.source_test_event_count, 9);
        assert_eq!(report.local_event_contract_count, 9);
        assert_eq!(report.sqlite_table_count, 1);
        assert_eq!(report.sqlite_unique_index_count, 2);
        assert!(report.wal_mode_required);
        assert!(report.wal_mode_test_covered);
        assert!(report.local_tempdb_persistence_test_covered);
        assert_eq!(report.accepted_append_count, 9);
        assert_eq!(report.duplicate_append_denial_count, 9);
        assert_eq!(report.idempotency_unique_index_entry_count, 9);
        assert_eq!(report.deterministic_replay_validation_count, 9);
        assert!(report.minimal_local_persistence_ready);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(report.local_tempdb_sqlite_write_covered_by_tests);
    }

    #[tokio::test]
    async fn minimal_local_event_store_appends_and_denies_duplicates() {
        let tempdir = tempfile::tempdir().expect("tempdir should be created");
        let db_path = tempdir.path().join("temporal-lite.sqlite3");
        let store = WorkflowTemporalLiteMinimalLocalEventStore::open(&db_path)
            .await
            .expect("sqlite store should open");
        let source =
            hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report();
        let events = workflow_temporal_lite_minimal_local_events_from_test_implementation(&source);

        for event in &events {
            let append = store
                .append_event(event)
                .await
                .expect("first append should succeed");
            let duplicate = store
                .append_event(event)
                .await
                .expect("duplicate append should be denied without error");

            assert!(append.accepted);
            assert!(!append.duplicate_denied);
            assert!(!duplicate.accepted);
            assert!(duplicate.duplicate_denied);
            assert_eq!(append.sequence, duplicate.sequence);
        }

        let stored = store
            .read_events()
            .await
            .expect("stored events should read back");
        assert_eq!(stored.len(), 9);
        assert_eq!(stored.first().expect("first").sequence, 1);
        assert!(
            stored
                .windows(2)
                .all(|window| window[0].sequence < window[1].sequence)
        );
        assert!(stored.iter().all(|event| {
            event.idempotency_key.starts_with("test-only.idempotency.")
                && event.replay_digest.starts_with("replay-digest.v1.")
                && !event.rollback_anchor.is_empty()
        }));
        assert_eq!(
            store
                .journal_mode()
                .await
                .expect("journal mode should read"),
            "wal"
        );
    }

    #[tokio::test]
    async fn minimal_local_event_store_replays_deterministically_after_reopen() {
        let tempdir = tempfile::tempdir().expect("tempdir should be created");
        let db_path = tempdir.path().join("temporal-lite.sqlite3");
        let source =
            hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report();
        let events = workflow_temporal_lite_minimal_local_events_from_test_implementation(&source);

        {
            let store = WorkflowTemporalLiteMinimalLocalEventStore::open(&db_path)
                .await
                .expect("sqlite store should open");
            for event in &events {
                store
                    .append_event(event)
                    .await
                    .expect("append should succeed");
            }
            let stored = store
                .read_events()
                .await
                .expect("stored events should read");
            let digest = WorkflowTemporalLiteMinimalLocalEventStore::replay_batch_digest(&stored);
            assert!(digest.starts_with("temporal-lite.local-replay.v1.9."));
        }

        let reopened = WorkflowTemporalLiteMinimalLocalEventStore::open(&db_path)
            .await
            .expect("sqlite store should reopen");
        let stored = reopened
            .read_events()
            .await
            .expect("reopened events should read");
        let digest = WorkflowTemporalLiteMinimalLocalEventStore::replay_batch_digest(&stored);
        assert!(digest.starts_with("temporal-lite.local-replay.v1.9."));
        assert_eq!(
            stored
                .iter()
                .map(|event| event.sequence)
                .collect::<Vec<_>>(),
            (1_i64..=9_i64).collect::<Vec<_>>()
        );
    }

    #[test]
    fn minimal_local_persistence_report_keeps_runtime_side_effects_closed() {
        let report =
            hepta_workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report();

        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.runtime_store_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.local_tempdb_persistence_test_covered
                && entry.runtime_persistence_state == "runtime_write_blocked"
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.live_execution_allowed
        }));
    }
}
