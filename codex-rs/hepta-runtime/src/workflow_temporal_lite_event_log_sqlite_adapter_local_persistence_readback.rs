use serde::Serialize;

use crate::WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackEntry;
use crate::WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackReport;
use crate::hepta_workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_GATE: &str =
    "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "workflow_temporal_lite_work_graph_projection_local_persistence_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_lease_idempotency_gate: &'static str,
    pub source_lease_idempotency_ready: bool,
    pub source_anchor_pair_count: usize,
    pub adapter_scope: &'static str,
    pub sqlite_readback_scope: &'static str,
    pub event_log_adapter_readback_count: usize,
    pub sqlite_adapter_readback_count: usize,
    pub event_log_record_key_count: usize,
    pub sqlite_row_key_count: usize,
    pub serialization_contract_count: usize,
    pub transaction_boundary_count: usize,
    pub sqlite_readback_validated_count: usize,
    pub event_log_record_written_count: usize,
    pub sqlite_row_written_count: usize,
    pub adapter_persisted_count: usize,
    pub adapter_mismatch_count: usize,
    pub wal_mode_required: bool,
    pub local_tempdb_sqlite_read_covered_by_tests: bool,
    pub runtime_feature_gate_enabled: bool,
    pub adapter_contract_readback_materialized: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub event_log_adapter_write_allowed: bool,
    pub sqlite_adapter_write_allowed: bool,
    pub adapter_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub event_log_sqlite_adapter_local_persistence_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackEntry {
    pub event_contract_id: String,
    pub replay_order: usize,
    pub local_sequence: i64,
    pub source_event_id: String,
    pub replay_projection_key: String,
    pub lease_readback_key: String,
    pub idempotency_index_readback_key: String,
    pub idempotency_key: String,
    pub event_log_adapter_key: String,
    pub event_log_stream: &'static str,
    pub event_log_record_key: String,
    pub event_log_record_schema: &'static str,
    pub sqlite_adapter_key: String,
    pub sqlite_table: &'static str,
    pub sqlite_row_key: String,
    pub sqlite_schema_version: &'static str,
    pub serialization_contract_key: String,
    pub transaction_boundary_key: String,
    pub adapter_state: &'static str,
    pub readback_state: &'static str,
    pub event_log_adapter_projected: bool,
    pub sqlite_adapter_projected: bool,
    pub serialization_contract_projected: bool,
    pub transaction_boundary_projected: bool,
    pub sqlite_readback_validated: bool,
    pub adapter_mismatch_detected: bool,
    pub event_log_record_written: bool,
    pub sqlite_row_written: bool,
    pub adapter_persisted: bool,
    pub wal_mode_required: bool,
    pub feature_gate_required: bool,
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
pub struct WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackSideEffects {
    pub report_written: bool,
    pub runtime_filesystem_written: bool,
    pub runtime_event_log_written: bool,
    pub runtime_sqlite_written: bool,
    pub runtime_store_persisted: bool,
    pub event_log_adapter_written: bool,
    pub sqlite_adapter_written: bool,
    pub adapter_persisted: bool,
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

pub fn hepta_workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_report()
-> WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackReport {
    let source =
        hepta_workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report();
    workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_report_from_lease_idempotency(&source)
}

pub fn workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_report_from_lease_idempotency(
    source: &WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackReport,
) -> WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackReport {
    let entries =
        workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_entries(source);
    let event_log_adapter_readback_count = entries
        .iter()
        .filter(|entry| entry.event_log_adapter_projected)
        .count();
    let sqlite_adapter_readback_count = entries
        .iter()
        .filter(|entry| entry.sqlite_adapter_projected)
        .count();
    let event_log_record_key_count = entries
        .iter()
        .filter(|entry| !entry.event_log_record_key.is_empty())
        .count();
    let sqlite_row_key_count = entries
        .iter()
        .filter(|entry| !entry.sqlite_row_key.is_empty())
        .count();
    let serialization_contract_count = entries
        .iter()
        .filter(|entry| entry.serialization_contract_projected)
        .count();
    let transaction_boundary_count = entries
        .iter()
        .filter(|entry| entry.transaction_boundary_projected)
        .count();
    let sqlite_readback_validated_count = entries
        .iter()
        .filter(|entry| entry.sqlite_readback_validated)
        .count();
    let event_log_record_written_count = entries
        .iter()
        .filter(|entry| entry.event_log_record_written)
        .count();
    let sqlite_row_written_count = entries
        .iter()
        .filter(|entry| entry.sqlite_row_written)
        .count();
    let adapter_persisted_count = entries
        .iter()
        .filter(|entry| entry.adapter_persisted)
        .count();
    let adapter_mismatch_count = entries
        .iter()
        .filter(|entry| entry.adapter_mismatch_detected)
        .count();
    let ready = source.lease_idempotency_index_local_persistence_readback_ready
        && source.source_anchor_pair_count == 9
        && source.lease_readback_count == 9
        && source.idempotency_index_readback_count == 9
        && source.duplicate_guard_readback_count == 9
        && source.lease_acquired_count == 0
        && source.lease_persisted_count == 0
        && source.idempotency_index_written_count == 0
        && source.idempotency_index_persisted_count == 0
        && source.lease_idempotency_mismatch_count == 0
        && source.local_tempdb_sqlite_read_covered_by_tests
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.runtime_store_persistence_allowed
        && !source.lease_acquire_allowed
        && !source.idempotency_index_write_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.source_anchor_pair_count
        && event_log_adapter_readback_count == entries.len()
        && sqlite_adapter_readback_count == entries.len()
        && event_log_record_key_count == entries.len()
        && sqlite_row_key_count == entries.len()
        && serialization_contract_count == entries.len()
        && transaction_boundary_count == entries.len()
        && sqlite_readback_validated_count == entries.len()
        && event_log_record_written_count == 0
        && sqlite_row_written_count == 0
        && adapter_persisted_count == 0
        && adapter_mismatch_count == 0
        && entries.iter().all(|entry| {
            entry.wal_mode_required
                && entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback",
        status: if ready { "ready_blocked" } else { "blocked" },
        gate: WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION,
        source_lease_idempotency_gate: source.gate,
        source_lease_idempotency_ready: source
            .lease_idempotency_index_local_persistence_readback_ready,
        source_anchor_pair_count: source.source_anchor_pair_count,
        adapter_scope: "local_persistence_event_log_sqlite_adapter_readback_no_runtime_writes",
        sqlite_readback_scope: source.sqlite_readback_scope,
        event_log_adapter_readback_count,
        sqlite_adapter_readback_count,
        event_log_record_key_count,
        sqlite_row_key_count,
        serialization_contract_count,
        transaction_boundary_count,
        sqlite_readback_validated_count,
        event_log_record_written_count,
        sqlite_row_written_count,
        adapter_persisted_count,
        adapter_mismatch_count,
        wal_mode_required: true,
        local_tempdb_sqlite_read_covered_by_tests: true,
        runtime_feature_gate_enabled: false,
        adapter_contract_readback_materialized: ready,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        runtime_store_persistence_allowed: false,
        event_log_adapter_write_allowed: false,
        sqlite_adapter_write_allowed: false,
        adapter_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        event_log_sqlite_adapter_local_persistence_readback_ready: ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "runtime_store_persistence_disabled",
            "event_log_adapter_write_disabled",
            "sqlite_adapter_write_disabled",
            "adapter_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_entries(
    source: &WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackReport,
) -> Vec<WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackEntry> {
    workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_entries_from_lease_entries(&source.entries)
}

pub fn workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_entries_from_lease_entries(
    lease_entries: &[WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackEntry],
) -> Vec<WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackEntry> {
    lease_entries
        .iter()
        .map(|source_entry| {
            let event_log_adapter_key = keyed_adapter(
                "temporal-lite.local-event-log-adapter-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let event_log_record_key = format!(
                "temporal-lite.local-event-log-record.v1.{:03}.{}.{}.{}",
                source_entry.replay_order,
                source_entry.event_contract_id,
                source_entry.idempotency_key.len(),
                source_entry.local_sequence
            );
            let sqlite_adapter_key = keyed_adapter(
                "temporal-lite.local-sqlite-adapter-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let sqlite_row_key = format!(
                "temporal-lite.local-sqlite-row.v1.{:03}.{}.{}.{}",
                source_entry.replay_order,
                source_entry.event_contract_id,
                source_entry.source_event_id.len(),
                source_entry.local_sequence
            );
            let serialization_contract_key = keyed_adapter(
                "temporal-lite.local-serialization-contract-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let transaction_boundary_key = keyed_adapter(
                "temporal-lite.local-transaction-boundary-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let adapter_projected = source_entry.lease_readback_projected
                && source_entry.idempotency_index_projected
                && source_entry.duplicate_guard_projected
                && source_entry.sqlite_readback_validated
                && !source_entry.lease_idempotency_mismatch_detected;

            WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackEntry {
                event_contract_id: source_entry.event_contract_id.clone(),
                replay_order: source_entry.replay_order,
                local_sequence: source_entry.local_sequence,
                source_event_id: source_entry.source_event_id.clone(),
                replay_projection_key: source_entry.replay_projection_key.clone(),
                lease_readback_key: source_entry.lease_readback_key.clone(),
                idempotency_index_readback_key: source_entry
                    .idempotency_index_readback_key
                    .clone(),
                idempotency_key: source_entry.idempotency_key.clone(),
                event_log_adapter_key,
                event_log_stream: "temporal_lite_local_persistence_event_log_stream",
                event_log_record_key,
                event_log_record_schema: "temporal_lite_local_event_log_record_v1",
                sqlite_adapter_key,
                sqlite_table: "temporal_lite_events",
                sqlite_row_key,
                sqlite_schema_version: "temporal_lite_local_sqlite_adapter_v1",
                serialization_contract_key,
                transaction_boundary_key,
                adapter_state: "projected_from_local_persistence_not_persisted",
                readback_state:
                    "projected_from_sqlite_wal_local_persistence_readback_without_runtime_writes",
                event_log_adapter_projected: adapter_projected,
                sqlite_adapter_projected: adapter_projected,
                serialization_contract_projected: adapter_projected,
                transaction_boundary_projected: adapter_projected,
                sqlite_readback_validated: source_entry.sqlite_readback_validated,
                adapter_mismatch_detected: source_entry.lease_idempotency_mismatch_detected,
                event_log_record_written: false,
                sqlite_row_written: false,
                adapter_persisted: false,
                wal_mode_required: source_entry.wal_mode_required,
                feature_gate_required: source_entry.feature_gate_required,
                runtime_feature_gate_enabled: source_entry.runtime_feature_gate_enabled,
                runtime_event_log_write_allowed: source_entry.runtime_event_log_write_allowed,
                runtime_sqlite_write_allowed: source_entry.runtime_sqlite_write_allowed,
                runtime_store_persistence_allowed: source_entry.runtime_store_persistence_allowed,
                workflow_execution_allowed: source_entry.workflow_execution_allowed,
                replay_execution_allowed: source_entry.replay_execution_allowed,
                rollback_execution_allowed: source_entry.rollback_execution_allowed,
                live_execution_allowed: source_entry.live_execution_allowed,
            }
        })
        .collect()
}

fn keyed_adapter(prefix: &str, replay_order: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{replay_order:03}.{event_contract_id}")
}

impl WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            runtime_filesystem_written: false,
            runtime_event_log_written: false,
            runtime_sqlite_written: false,
            runtime_store_persisted: false,
            event_log_adapter_written: false,
            sqlite_adapter_written: false,
            adapter_persisted: false,
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
    use crate::WorkflowTemporalLiteMinimalLocalEventStore;
    use crate::hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report;
    use crate::workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_entries_from_replay_entries;
    use crate::workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_entries_from_stored_events;
    use crate::workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_entries_from_anchor_entries;
    use crate::workflow_temporal_lite_minimal_local_events_from_test_implementation;

    #[test]
    fn local_event_log_sqlite_adapter_projects_all_entries_without_writes() {
        let report =
            hepta_workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_report(
            );

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_lease_idempotency_ready);
        assert_eq!(report.source_anchor_pair_count, 9);
        assert_eq!(report.event_log_adapter_readback_count, 9);
        assert_eq!(report.sqlite_adapter_readback_count, 9);
        assert_eq!(report.event_log_record_key_count, 9);
        assert_eq!(report.sqlite_row_key_count, 9);
        assert_eq!(report.serialization_contract_count, 9);
        assert_eq!(report.transaction_boundary_count, 9);
        assert_eq!(report.sqlite_readback_validated_count, 9);
        assert_eq!(report.event_log_record_written_count, 0);
        assert_eq!(report.sqlite_row_written_count, 0);
        assert_eq!(report.adapter_persisted_count, 0);
        assert_eq!(report.adapter_mismatch_count, 0);
        assert!(report.adapter_contract_readback_materialized);
        assert!(report.event_log_sqlite_adapter_local_persistence_readback_ready);
    }

    #[tokio::test]
    async fn local_event_log_sqlite_adapter_uses_reopened_sqlite_event_history() {
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
        }

        let reopened = WorkflowTemporalLiteMinimalLocalEventStore::open(&db_path)
            .await
            .expect("sqlite store should reopen");
        let stored_events = reopened
            .read_events()
            .await
            .expect("stored events should read back");
        let replay_entries =
            workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_entries_from_stored_events(&stored_events);
        let anchor_entries =
            workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_entries_from_replay_entries(&replay_entries);
        let lease_entries =
            workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_entries_from_anchor_entries(&anchor_entries);
        let adapter_entries =
            workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_entries_from_lease_entries(&lease_entries);

        assert_eq!(stored_events.len(), 9);
        assert_eq!(lease_entries.len(), 9);
        assert_eq!(adapter_entries.len(), 9);
        assert!(adapter_entries.iter().enumerate().all(|(index, entry)| {
            entry.replay_order == index + 1
                && entry.local_sequence >= 1
                && entry
                    .event_log_adapter_key
                    .starts_with("temporal-lite.local-event-log-adapter-readback.")
                && entry.event_log_stream == "temporal_lite_local_persistence_event_log_stream"
                && entry
                    .event_log_record_key
                    .starts_with("temporal-lite.local-event-log-record.v1.")
                && entry.event_log_record_schema == "temporal_lite_local_event_log_record_v1"
                && entry
                    .sqlite_adapter_key
                    .starts_with("temporal-lite.local-sqlite-adapter-readback.")
                && entry.sqlite_table == "temporal_lite_events"
                && entry
                    .sqlite_row_key
                    .starts_with("temporal-lite.local-sqlite-row.v1.")
                && entry.sqlite_schema_version == "temporal_lite_local_sqlite_adapter_v1"
                && entry
                    .serialization_contract_key
                    .starts_with("temporal-lite.local-serialization-contract-readback.")
                && entry
                    .transaction_boundary_key
                    .starts_with("temporal-lite.local-transaction-boundary-readback.")
                && entry.adapter_state == "projected_from_local_persistence_not_persisted"
                && entry.readback_state
                    == "projected_from_sqlite_wal_local_persistence_readback_without_runtime_writes"
                && entry.event_log_adapter_projected
                && entry.sqlite_adapter_projected
                && entry.serialization_contract_projected
                && entry.transaction_boundary_projected
                && entry.sqlite_readback_validated
                && !entry.adapter_mismatch_detected
                && !entry.event_log_record_written
                && !entry.sqlite_row_written
                && !entry.adapter_persisted
                && entry.wal_mode_required
                && entry.feature_gate_required
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn local_event_log_sqlite_adapter_side_effects_remain_closed() {
        let report =
            hepta_workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_report(
            );

        assert!(report.wal_mode_required);
        assert!(report.local_tempdb_sqlite_read_covered_by_tests);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.runtime_store_persistence_allowed);
        assert!(!report.event_log_adapter_write_allowed);
        assert!(!report.sqlite_adapter_write_allowed);
        assert!(!report.adapter_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.wal_mode_required
                && entry.feature_gate_required
                && entry.sqlite_readback_validated
                && !entry.adapter_mismatch_detected
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }
}
