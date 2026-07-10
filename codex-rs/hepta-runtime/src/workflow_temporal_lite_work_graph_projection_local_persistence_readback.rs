use serde::Serialize;

use crate::WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackEntry;
use crate::WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackReport;
use crate::hepta_workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_LOCAL_PERSISTENCE_READBACK_GATE: &str =
    "workflow_temporal_lite_work_graph_projection_local_persistence_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_work_graph_projection_local_persistence_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_adapter_gate: &'static str,
    pub source_adapter_ready: bool,
    pub source_adapter_entry_count: usize,
    pub source_append_only_event_store_interface_ready: bool,
    pub source_event_log_sqlite_adapter_derived_from_event_store_interface: bool,
    pub projection_scope: &'static str,
    pub sqlite_readback_scope: &'static str,
    pub work_graph_node_projection_count: usize,
    pub work_graph_event_edge_projection_count: usize,
    pub work_graph_state_edge_projection_count: usize,
    pub projection_key_count: usize,
    pub projection_checksum_count: usize,
    pub sqlite_readback_validated_count: usize,
    pub projection_persisted_count: usize,
    pub work_graph_store_write_count: usize,
    pub event_log_write_count: usize,
    pub sqlite_write_count: usize,
    pub projection_mismatch_count: usize,
    pub wal_mode_required: bool,
    pub local_tempdb_sqlite_read_covered_by_tests: bool,
    pub runtime_feature_gate_enabled: bool,
    pub projection_contract_readback_materialized: bool,
    pub work_graph_projection_derived_from_event_store_interface: bool,
    pub work_graph_projection_write_allowed: bool,
    pub work_graph_projection_persistence_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub work_graph_projection_local_persistence_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackEntry {
    pub event_contract_id: String,
    pub replay_order: usize,
    pub local_sequence: i64,
    pub source_event_id: String,
    pub event_log_record_key: String,
    pub sqlite_row_key: String,
    pub work_graph_node_key: String,
    pub work_graph_node_kind: &'static str,
    pub work_graph_event_edge_key: String,
    pub work_graph_state_edge_key: String,
    pub projection_key: String,
    pub projection_checksum: String,
    pub projection_state: &'static str,
    pub readback_state: &'static str,
    pub work_graph_node_projected: bool,
    pub work_graph_event_edge_projected: bool,
    pub work_graph_state_edge_projected: bool,
    pub projection_checksum_projected: bool,
    pub sqlite_readback_validated: bool,
    pub projection_mismatch_detected: bool,
    pub projection_persisted: bool,
    pub work_graph_store_written: bool,
    pub event_log_record_written: bool,
    pub sqlite_row_written: bool,
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
pub struct WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackSideEffects {
    pub report_written: bool,
    pub runtime_filesystem_written: bool,
    pub work_graph_projection_written: bool,
    pub work_graph_projection_persisted: bool,
    pub runtime_event_log_written: bool,
    pub runtime_sqlite_written: bool,
    pub runtime_store_persisted: bool,
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

pub fn hepta_workflow_temporal_lite_work_graph_projection_local_persistence_readback_report()
-> WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackReport {
    let source =
        hepta_workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_report();
    workflow_temporal_lite_work_graph_projection_local_persistence_readback_report_from_adapter(
        &source,
    )
}

pub fn workflow_temporal_lite_work_graph_projection_local_persistence_readback_report_from_adapter(
    source: &WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackReport,
) -> WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackReport {
    let entries =
        workflow_temporal_lite_work_graph_projection_local_persistence_readback_entries(source);
    let work_graph_node_projection_count = entries
        .iter()
        .filter(|entry| entry.work_graph_node_projected)
        .count();
    let work_graph_event_edge_projection_count = entries
        .iter()
        .filter(|entry| entry.work_graph_event_edge_projected)
        .count();
    let work_graph_state_edge_projection_count = entries
        .iter()
        .filter(|entry| entry.work_graph_state_edge_projected)
        .count();
    let projection_key_count = entries
        .iter()
        .filter(|entry| !entry.projection_key.is_empty())
        .count();
    let projection_checksum_count = entries
        .iter()
        .filter(|entry| entry.projection_checksum_projected)
        .count();
    let sqlite_readback_validated_count = entries
        .iter()
        .filter(|entry| entry.sqlite_readback_validated)
        .count();
    let projection_persisted_count = entries
        .iter()
        .filter(|entry| entry.projection_persisted)
        .count();
    let work_graph_store_write_count = entries
        .iter()
        .filter(|entry| entry.work_graph_store_written)
        .count();
    let event_log_write_count = entries
        .iter()
        .filter(|entry| entry.event_log_record_written)
        .count();
    let sqlite_write_count = entries
        .iter()
        .filter(|entry| entry.sqlite_row_written)
        .count();
    let projection_mismatch_count = entries
        .iter()
        .filter(|entry| entry.projection_mismatch_detected)
        .count();
    let work_graph_projection_derived_from_event_store_interface = source
        .source_append_only_event_store_interface_ready
        && source.event_log_sqlite_adapter_derived_from_event_store_interface;
    let ready = source.event_log_sqlite_adapter_local_persistence_readback_ready
        && work_graph_projection_derived_from_event_store_interface
        && source.source_anchor_pair_count == 9
        && source.event_log_adapter_readback_count == 9
        && source.sqlite_adapter_readback_count == 9
        && source.event_log_record_written_count == 0
        && source.sqlite_row_written_count == 0
        && source.adapter_persisted_count == 0
        && source.adapter_mismatch_count == 0
        && source.local_tempdb_sqlite_read_covered_by_tests
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.runtime_store_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.source_anchor_pair_count
        && work_graph_node_projection_count == entries.len()
        && work_graph_event_edge_projection_count == entries.len()
        && work_graph_state_edge_projection_count == entries.len()
        && projection_key_count == entries.len()
        && projection_checksum_count == entries.len()
        && sqlite_readback_validated_count == entries.len()
        && projection_persisted_count == 0
        && work_graph_store_write_count == 0
        && event_log_write_count == 0
        && sqlite_write_count == 0
        && projection_mismatch_count == 0
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

    WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_work_graph_projection_local_persistence_readback",
        status: if ready { "ready_blocked" } else { "blocked" },
        gate: WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_LOCAL_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION,
        source_adapter_gate: source.gate,
        source_adapter_ready: source.event_log_sqlite_adapter_local_persistence_readback_ready,
        source_adapter_entry_count: source.source_anchor_pair_count,
        source_append_only_event_store_interface_ready: source
            .source_append_only_event_store_interface_ready,
        source_event_log_sqlite_adapter_derived_from_event_store_interface: source
            .event_log_sqlite_adapter_derived_from_event_store_interface,
        projection_scope: "local_persistence_work_graph_projection_readback_no_persistence",
        sqlite_readback_scope: source.sqlite_readback_scope,
        work_graph_node_projection_count,
        work_graph_event_edge_projection_count,
        work_graph_state_edge_projection_count,
        projection_key_count,
        projection_checksum_count,
        sqlite_readback_validated_count,
        projection_persisted_count,
        work_graph_store_write_count,
        event_log_write_count,
        sqlite_write_count,
        projection_mismatch_count,
        wal_mode_required: true,
        local_tempdb_sqlite_read_covered_by_tests: true,
        runtime_feature_gate_enabled: false,
        projection_contract_readback_materialized: ready,
        work_graph_projection_derived_from_event_store_interface,
        work_graph_projection_write_allowed: false,
        work_graph_projection_persistence_allowed: false,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        runtime_store_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        work_graph_projection_local_persistence_readback_ready: ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "work_graph_projection_write_disabled",
            "work_graph_projection_persistence_disabled",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "runtime_store_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_work_graph_projection_local_persistence_readback_entries(
    source: &WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackReport,
) -> Vec<WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackEntry> {
    workflow_temporal_lite_work_graph_projection_local_persistence_readback_entries_from_adapter_entries(&source.entries)
}

pub fn workflow_temporal_lite_work_graph_projection_local_persistence_readback_entries_from_adapter_entries(
    adapter_entries: &[WorkflowTemporalLiteEventLogSqliteAdapterLocalPersistenceReadbackEntry],
) -> Vec<WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackEntry> {
    adapter_entries
        .iter()
        .map(|source_entry| {
            let work_graph_node_key = keyed_projection(
                "temporal-lite.local-work-graph.node-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let work_graph_event_edge_key = keyed_projection(
                "temporal-lite.local-work-graph.event-edge-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let work_graph_state_edge_key = keyed_projection(
                "temporal-lite.local-work-graph.state-edge-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let projection_key = keyed_projection(
                "temporal-lite.local-work-graph.projection-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let projection_checksum = format!(
                "temporal-lite.local-work-graph-projection-checksum.v1.{:03}.{}.{}.{}.{}",
                source_entry.replay_order,
                source_entry.event_contract_id,
                source_entry.event_log_record_key.len(),
                source_entry.sqlite_row_key.len(),
                source_entry.local_sequence
            );
            let projection_projected = source_entry.event_log_adapter_projected
                && source_entry.sqlite_adapter_projected
                && source_entry.serialization_contract_projected
                && source_entry.sqlite_readback_validated
                && !source_entry.adapter_mismatch_detected;

            WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackEntry {
                event_contract_id: source_entry.event_contract_id.clone(),
                replay_order: source_entry.replay_order,
                local_sequence: source_entry.local_sequence,
                source_event_id: source_entry.source_event_id.clone(),
                event_log_record_key: source_entry.event_log_record_key.clone(),
                sqlite_row_key: source_entry.sqlite_row_key.clone(),
                work_graph_node_key,
                work_graph_node_kind: work_graph_node_kind(&source_entry.event_contract_id),
                work_graph_event_edge_key,
                work_graph_state_edge_key,
                projection_key,
                projection_checksum,
                projection_state: "projected_from_local_persistence_not_persisted",
                readback_state:
                    "projected_from_sqlite_wal_local_persistence_readback_without_projection_writes",
                work_graph_node_projected: projection_projected,
                work_graph_event_edge_projected: projection_projected,
                work_graph_state_edge_projected: projection_projected,
                projection_checksum_projected: projection_projected,
                sqlite_readback_validated: source_entry.sqlite_readback_validated,
                projection_mismatch_detected: source_entry.adapter_mismatch_detected,
                projection_persisted: false,
                work_graph_store_written: false,
                event_log_record_written: false,
                sqlite_row_written: false,
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

fn keyed_projection(prefix: &str, replay_order: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{replay_order:03}.{event_contract_id}")
}

fn work_graph_node_kind(event_contract_id: &str) -> &'static str {
    if event_contract_id.contains("approval") {
        "approval_event"
    } else if event_contract_id.contains("task_result") {
        "task_result_event"
    } else if event_contract_id.contains("checkpoint") {
        "checkpoint_event"
    } else {
        "workflow_event"
    }
}

impl WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            runtime_filesystem_written: false,
            work_graph_projection_written: false,
            work_graph_projection_persisted: false,
            runtime_event_log_written: false,
            runtime_sqlite_written: false,
            runtime_store_persisted: false,
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
    use crate::workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_entries_from_lease_entries;
    use crate::workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_entries_from_anchor_entries;
    use crate::workflow_temporal_lite_minimal_local_events_from_test_implementation;

    #[test]
    fn local_work_graph_projection_projects_all_adapter_entries_without_writes() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_local_persistence_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_adapter_ready);
        assert_eq!(report.source_adapter_entry_count, 9);
        assert!(report.source_append_only_event_store_interface_ready);
        assert!(report.source_event_log_sqlite_adapter_derived_from_event_store_interface);
        assert_eq!(report.work_graph_node_projection_count, 9);
        assert_eq!(report.work_graph_event_edge_projection_count, 9);
        assert_eq!(report.work_graph_state_edge_projection_count, 9);
        assert_eq!(report.projection_key_count, 9);
        assert_eq!(report.projection_checksum_count, 9);
        assert_eq!(report.sqlite_readback_validated_count, 9);
        assert_eq!(report.projection_persisted_count, 0);
        assert_eq!(report.work_graph_store_write_count, 0);
        assert_eq!(report.event_log_write_count, 0);
        assert_eq!(report.sqlite_write_count, 0);
        assert_eq!(report.projection_mismatch_count, 0);
        assert!(report.projection_contract_readback_materialized);
        assert!(report.work_graph_projection_derived_from_event_store_interface);
        assert!(report.work_graph_projection_local_persistence_readback_ready);
    }

    #[tokio::test]
    async fn local_work_graph_projection_uses_reopened_sqlite_event_history() {
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
        let projection_entries =
            workflow_temporal_lite_work_graph_projection_local_persistence_readback_entries_from_adapter_entries(&adapter_entries);

        assert_eq!(stored_events.len(), 9);
        assert_eq!(adapter_entries.len(), 9);
        assert_eq!(projection_entries.len(), 9);
        assert!(projection_entries.iter().enumerate().all(|(index, entry)| {
            entry.replay_order == index + 1
                && entry.local_sequence >= 1
                && entry
                    .work_graph_node_key
                    .starts_with("temporal-lite.local-work-graph.node-readback.")
                && entry
                    .work_graph_event_edge_key
                    .starts_with("temporal-lite.local-work-graph.event-edge-readback.")
                && entry
                    .work_graph_state_edge_key
                    .starts_with("temporal-lite.local-work-graph.state-edge-readback.")
                && entry
                    .projection_key
                    .starts_with("temporal-lite.local-work-graph.projection-readback.")
                && entry
                    .projection_checksum
                    .starts_with("temporal-lite.local-work-graph-projection-checksum.v1.")
                && entry.projection_state == "projected_from_local_persistence_not_persisted"
                && entry.readback_state
                    == "projected_from_sqlite_wal_local_persistence_readback_without_projection_writes"
                && entry.work_graph_node_projected
                && entry.work_graph_event_edge_projected
                && entry.work_graph_state_edge_projected
                && entry.projection_checksum_projected
                && entry.sqlite_readback_validated
                && !entry.projection_mismatch_detected
                && !entry.projection_persisted
                && !entry.work_graph_store_written
                && !entry.event_log_record_written
                && !entry.sqlite_row_written
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
    fn local_work_graph_projection_side_effects_remain_closed() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_local_persistence_readback_report();

        assert!(report.wal_mode_required);
        assert!(report.local_tempdb_sqlite_read_covered_by_tests);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.work_graph_projection_write_allowed);
        assert!(!report.work_graph_projection_persistence_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.runtime_store_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.wal_mode_required
                && entry.feature_gate_required
                && entry.sqlite_readback_validated
                && !entry.projection_mismatch_detected
                && !entry.projection_persisted
                && !entry.work_graph_store_written
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.event_contract_id == "approval_event_intake"
                && entry.work_graph_node_kind == "approval_event"
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.event_contract_id == "task_result_event_intake"
                && entry.work_graph_node_kind == "task_result_event"
        }));
    }
}
