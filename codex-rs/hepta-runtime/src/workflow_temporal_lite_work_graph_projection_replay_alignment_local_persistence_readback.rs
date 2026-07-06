use serde::Serialize;

use crate::WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackEntry;
use crate::WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackReport;
use crate::hepta_workflow_temporal_lite_work_graph_projection_local_persistence_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_LOCAL_PERSISTENCE_READBACK_GATE: &str =
    "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_projection_gate: &'static str,
    pub source_projection_ready: bool,
    pub source_projection_entry_count: usize,
    pub alignment_scope: &'static str,
    pub sqlite_readback_scope: &'static str,
    pub replay_alignment_projection_count: usize,
    pub projection_replay_key_count: usize,
    pub replay_alignment_checksum_count: usize,
    pub deterministic_alignment_count: usize,
    pub sqlite_readback_validated_count: usize,
    pub replay_alignment_mismatch_count: usize,
    pub replay_executed_count: usize,
    pub projection_alignment_persisted_count: usize,
    pub work_graph_store_write_count: usize,
    pub event_log_write_count: usize,
    pub sqlite_write_count: usize,
    pub wal_mode_required: bool,
    pub local_tempdb_sqlite_read_covered_by_tests: bool,
    pub runtime_feature_gate_enabled: bool,
    pub replay_alignment_contract_readback_materialized: bool,
    pub replay_execution_allowed: bool,
    pub projection_alignment_persistence_allowed: bool,
    pub work_graph_projection_write_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub replay_alignment_local_persistence_readback_ready: bool,
    pub entries:
        Vec<WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackEntry {
    pub event_contract_id: String,
    pub replay_order: usize,
    pub local_sequence: i64,
    pub source_event_id: String,
    pub work_graph_node_key: String,
    pub work_graph_event_edge_key: String,
    pub work_graph_state_edge_key: String,
    pub projection_key: String,
    pub projection_checksum: String,
    pub replay_alignment_key: String,
    pub projection_replay_key: String,
    pub replay_alignment_checksum: String,
    pub expected_replay_projection_key: String,
    pub alignment_state: &'static str,
    pub readback_state: &'static str,
    pub work_graph_projection_projected: bool,
    pub replay_alignment_projected: bool,
    pub projection_replay_key_projected: bool,
    pub replay_alignment_checksum_projected: bool,
    pub deterministic_alignment_projected: bool,
    pub sqlite_readback_validated: bool,
    pub replay_alignment_mismatch_detected: bool,
    pub replay_executed: bool,
    pub projection_alignment_persisted: bool,
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
pub struct WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackSideEffects
{
    pub report_written: bool,
    pub runtime_filesystem_written: bool,
    pub replay_executed: bool,
    pub projection_alignment_written: bool,
    pub projection_alignment_persisted: bool,
    pub work_graph_projection_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub runtime_store_persisted: bool,
    pub workflow_execution_started: bool,
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

pub fn hepta_workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_report()
-> WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackReport {
    let source =
        hepta_workflow_temporal_lite_work_graph_projection_local_persistence_readback_report();
    workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_report_from_projection(&source)
}

pub fn workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_report_from_projection(
    source: &WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackReport,
) -> WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackReport {
    let entries =
        workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_entries(source);
    let replay_alignment_projection_count = entries
        .iter()
        .filter(|entry| entry.replay_alignment_projected)
        .count();
    let projection_replay_key_count = entries
        .iter()
        .filter(|entry| entry.projection_replay_key_projected)
        .count();
    let replay_alignment_checksum_count = entries
        .iter()
        .filter(|entry| entry.replay_alignment_checksum_projected)
        .count();
    let deterministic_alignment_count = entries
        .iter()
        .filter(|entry| entry.deterministic_alignment_projected)
        .count();
    let sqlite_readback_validated_count = entries
        .iter()
        .filter(|entry| entry.sqlite_readback_validated)
        .count();
    let replay_alignment_mismatch_count = entries
        .iter()
        .filter(|entry| entry.replay_alignment_mismatch_detected)
        .count();
    let replay_executed_count = entries.iter().filter(|entry| entry.replay_executed).count();
    let projection_alignment_persisted_count = entries
        .iter()
        .filter(|entry| entry.projection_alignment_persisted)
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
    let ready = source.work_graph_projection_local_persistence_readback_ready
        && source.source_adapter_entry_count == 9
        && source.work_graph_node_projection_count == 9
        && source.work_graph_event_edge_projection_count == 9
        && source.work_graph_state_edge_projection_count == 9
        && source.projection_key_count == 9
        && source.projection_checksum_count == 9
        && source.sqlite_readback_validated_count == 9
        && source.projection_persisted_count == 0
        && source.work_graph_store_write_count == 0
        && source.event_log_write_count == 0
        && source.sqlite_write_count == 0
        && source.projection_mismatch_count == 0
        && source.local_tempdb_sqlite_read_covered_by_tests
        && !source.runtime_feature_gate_enabled
        && !source.work_graph_projection_write_allowed
        && !source.work_graph_projection_persistence_allowed
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.runtime_store_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.source_adapter_entry_count
        && replay_alignment_projection_count == entries.len()
        && projection_replay_key_count == entries.len()
        && replay_alignment_checksum_count == entries.len()
        && deterministic_alignment_count == entries.len()
        && sqlite_readback_validated_count == entries.len()
        && replay_alignment_mismatch_count == 0
        && replay_executed_count == 0
        && projection_alignment_persisted_count == 0
        && work_graph_store_write_count == 0
        && event_log_write_count == 0
        && sqlite_write_count == 0
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

    WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackReport {
        runtime: "hepta",
        surface:
            "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback",
        status: if ready { "ready_blocked" } else { "blocked" },
        gate:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_LOCAL_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION,
        source_projection_gate: source.gate,
        source_projection_ready: source.work_graph_projection_local_persistence_readback_ready,
        source_projection_entry_count: source.source_adapter_entry_count,
        alignment_scope:
            "local_persistence_work_graph_projection_replay_alignment_readback_no_execution",
        sqlite_readback_scope: source.sqlite_readback_scope,
        replay_alignment_projection_count,
        projection_replay_key_count,
        replay_alignment_checksum_count,
        deterministic_alignment_count,
        sqlite_readback_validated_count,
        replay_alignment_mismatch_count,
        replay_executed_count,
        projection_alignment_persisted_count,
        work_graph_store_write_count,
        event_log_write_count,
        sqlite_write_count,
        wal_mode_required: true,
        local_tempdb_sqlite_read_covered_by_tests: true,
        runtime_feature_gate_enabled: false,
        replay_alignment_contract_readback_materialized: ready,
        replay_execution_allowed: false,
        projection_alignment_persistence_allowed: false,
        work_graph_projection_write_allowed: false,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        runtime_store_persistence_allowed: false,
        workflow_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        replay_alignment_local_persistence_readback_ready: ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "replay_execution_disabled",
            "projection_alignment_persistence_disabled",
            "work_graph_projection_write_disabled",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "runtime_store_persistence_disabled",
            "workflow_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_entries(
    source: &WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackReport,
) -> Vec<WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackEntry> {
    workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_entries_from_projection_entries(&source.entries)
}

pub fn workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_entries_from_projection_entries(
    projection_entries: &[WorkflowTemporalLiteWorkGraphProjectionLocalPersistenceReadbackEntry],
) -> Vec<WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackEntry> {
    projection_entries
        .iter()
        .map(|source_entry| {
            let replay_alignment_key = keyed_alignment(
                "temporal-lite.local-work-graph.replay-alignment-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let projection_replay_key = keyed_alignment(
                "temporal-lite.local-work-graph.projection-replay-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let replay_alignment_checksum = format!(
                "temporal-lite.local-work-graph-replay-alignment-checksum.v1.{:03}.{}.{}.{}.{}",
                source_entry.replay_order,
                source_entry.event_contract_id,
                source_entry.projection_key.len(),
                source_entry.projection_checksum.len(),
                source_entry.local_sequence
            );
            let work_graph_projection_projected = source_entry.work_graph_node_projected
                && source_entry.work_graph_event_edge_projected
                && source_entry.work_graph_state_edge_projected
                && source_entry.projection_checksum_projected
                && source_entry.sqlite_readback_validated
                && !source_entry.projection_mismatch_detected;
            let deterministic_alignment_projected =
                work_graph_projection_projected && !source_entry.projection_key.is_empty();

            WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackEntry {
                event_contract_id: source_entry.event_contract_id.clone(),
                replay_order: source_entry.replay_order,
                local_sequence: source_entry.local_sequence,
                source_event_id: source_entry.source_event_id.clone(),
                work_graph_node_key: source_entry.work_graph_node_key.clone(),
                work_graph_event_edge_key: source_entry.work_graph_event_edge_key.clone(),
                work_graph_state_edge_key: source_entry.work_graph_state_edge_key.clone(),
                projection_key: source_entry.projection_key.clone(),
                projection_checksum: source_entry.projection_checksum.clone(),
                replay_alignment_key,
                projection_replay_key,
                replay_alignment_checksum,
                expected_replay_projection_key: source_entry.projection_key.clone(),
                alignment_state: "aligned_from_local_persistence_not_replayed",
                readback_state:
                    "projected_from_sqlite_wal_local_persistence_readback_without_replay_execution",
                work_graph_projection_projected,
                replay_alignment_projected: work_graph_projection_projected,
                projection_replay_key_projected: !source_entry.projection_key.is_empty(),
                replay_alignment_checksum_projected: !source_entry.projection_checksum.is_empty(),
                deterministic_alignment_projected,
                sqlite_readback_validated: source_entry.sqlite_readback_validated,
                replay_alignment_mismatch_detected: !deterministic_alignment_projected,
                replay_executed: false,
                projection_alignment_persisted: false,
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

fn keyed_alignment(prefix: &str, replay_order: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{replay_order:03}.{event_contract_id}")
}

impl WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            runtime_filesystem_written: false,
            replay_executed: false,
            projection_alignment_written: false,
            projection_alignment_persisted: false,
            work_graph_projection_written: false,
            event_log_written: false,
            sqlite_written: false,
            runtime_store_persisted: false,
            workflow_execution_started: false,
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
    use crate::workflow_temporal_lite_work_graph_projection_local_persistence_readback_entries_from_adapter_entries;

    #[test]
    fn local_replay_alignment_projects_all_work_graph_entries_without_execution() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_projection_ready);
        assert_eq!(report.source_projection_entry_count, 9);
        assert_eq!(report.replay_alignment_projection_count, 9);
        assert_eq!(report.projection_replay_key_count, 9);
        assert_eq!(report.replay_alignment_checksum_count, 9);
        assert_eq!(report.deterministic_alignment_count, 9);
        assert_eq!(report.sqlite_readback_validated_count, 9);
        assert_eq!(report.replay_alignment_mismatch_count, 0);
        assert_eq!(report.replay_executed_count, 0);
        assert_eq!(report.projection_alignment_persisted_count, 0);
        assert_eq!(report.work_graph_store_write_count, 0);
        assert_eq!(report.event_log_write_count, 0);
        assert_eq!(report.sqlite_write_count, 0);
        assert!(report.replay_alignment_contract_readback_materialized);
        assert!(report.replay_alignment_local_persistence_readback_ready);
    }

    #[tokio::test]
    async fn local_replay_alignment_uses_reopened_sqlite_event_history() {
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
        let alignment_entries =
            workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_entries_from_projection_entries(&projection_entries);

        assert_eq!(stored_events.len(), 9);
        assert_eq!(projection_entries.len(), 9);
        assert_eq!(alignment_entries.len(), 9);
        assert!(alignment_entries.iter().enumerate().all(|(index, entry)| {
            entry.replay_order == index + 1
                && entry.local_sequence >= 1
                && entry
                    .replay_alignment_key
                    .starts_with("temporal-lite.local-work-graph.replay-alignment-readback.")
                && entry
                    .projection_replay_key
                    .starts_with("temporal-lite.local-work-graph.projection-replay-readback.")
                && entry
                    .replay_alignment_checksum
                    .starts_with("temporal-lite.local-work-graph-replay-alignment-checksum.v1.")
                && entry.expected_replay_projection_key == entry.projection_key
                && entry.alignment_state == "aligned_from_local_persistence_not_replayed"
                && entry.readback_state
                    == "projected_from_sqlite_wal_local_persistence_readback_without_replay_execution"
                && entry.work_graph_projection_projected
                && entry.replay_alignment_projected
                && entry.projection_replay_key_projected
                && entry.replay_alignment_checksum_projected
                && entry.deterministic_alignment_projected
                && entry.sqlite_readback_validated
                && !entry.replay_alignment_mismatch_detected
                && !entry.replay_executed
                && !entry.projection_alignment_persisted
                && !entry.work_graph_store_written
                && !entry.event_log_record_written
                && !entry.sqlite_row_written
                && entry.wal_mode_required
                && entry.feature_gate_required
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn local_replay_alignment_side_effects_remain_closed() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_report();

        assert!(report.wal_mode_required);
        assert!(report.local_tempdb_sqlite_read_covered_by_tests);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.replay_execution_allowed);
        assert!(!report.projection_alignment_persistence_allowed);
        assert!(!report.work_graph_projection_write_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.runtime_store_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentLocalPersistenceReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.wal_mode_required
                && entry.feature_gate_required
                && entry.sqlite_readback_validated
                && !entry.replay_alignment_mismatch_detected
                && !entry.replay_executed
                && !entry.projection_alignment_persisted
                && !entry.work_graph_store_written
                && !entry.event_log_record_written
                && !entry.sqlite_row_written
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
