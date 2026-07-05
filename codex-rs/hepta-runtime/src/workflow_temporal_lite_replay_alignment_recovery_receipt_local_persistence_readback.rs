use serde::Serialize;

use crate::WorkflowTemporalLiteReplayAlignmentRecoveryWindowLocalPersistenceReadbackEntry;
use crate::WorkflowTemporalLiteReplayAlignmentRecoveryWindowLocalPersistenceReadbackReport;
use crate::hepta_workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_RECEIPT_LOCAL_PERSISTENCE_READBACK_GATE:
    &str = "workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_RECEIPT_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_RECEIPT_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_gate_recursion_cost_boundary_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_recovery_window_gate: &'static str,
    pub source_recovery_window_ready: bool,
    pub source_recovery_window_entry_count: usize,
    pub receipt_scope: &'static str,
    pub sqlite_readback_scope: &'static str,
    pub recovery_receipt_projection_count: usize,
    pub recovery_receipt_key_count: usize,
    pub recovery_receipt_digest_count: usize,
    pub recovery_receipt_ack_count: usize,
    pub replay_alignment_receipt_match_count: usize,
    pub sqlite_readback_validated_count: usize,
    pub recovery_receipt_mismatch_count: usize,
    pub replay_executed_count: usize,
    pub checkpoint_written_count: usize,
    pub rollback_anchor_written_count: usize,
    pub recovery_window_persisted_count: usize,
    pub recovery_receipt_persisted_count: usize,
    pub work_graph_store_write_count: usize,
    pub event_log_write_count: usize,
    pub sqlite_write_count: usize,
    pub wal_mode_required: bool,
    pub local_tempdb_sqlite_read_covered_by_tests: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub recovery_receipt_contract_readback_materialized: bool,
    pub replay_execution_allowed: bool,
    pub checkpoint_write_allowed: bool,
    pub rollback_anchor_write_allowed: bool,
    pub recovery_window_persistence_allowed: bool,
    pub recovery_receipt_persistence_allowed: bool,
    pub work_graph_projection_write_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub source_gate_recursion_bounded: bool,
    pub recovery_receipt_local_persistence_readback_ready: bool,
    pub entries:
        Vec<WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackEntry {
    pub event_contract_id: String,
    pub replay_order: usize,
    pub local_sequence: i64,
    pub source_event_id: String,
    pub replay_alignment_key: String,
    pub projection_replay_key: String,
    pub rollback_consistency_key: String,
    pub rollback_readback_key: String,
    pub recovery_window_key: String,
    pub recovery_window_start_key: String,
    pub recovery_window_end_key: String,
    pub recovery_window_digest: String,
    pub recovery_receipt_key: String,
    pub recovery_receipt_ack_key: String,
    pub recovery_receipt_digest: String,
    pub expected_recovery_receipt_key: String,
    pub receipt_state: &'static str,
    pub readback_state: &'static str,
    pub recovery_window_projected: bool,
    pub recovery_receipt_projected: bool,
    pub recovery_receipt_key_projected: bool,
    pub recovery_receipt_digest_projected: bool,
    pub recovery_receipt_ack_projected: bool,
    pub replay_alignment_receipt_matches: bool,
    pub sqlite_readback_validated: bool,
    pub recovery_receipt_mismatch_detected: bool,
    pub replay_executed: bool,
    pub checkpoint_written: bool,
    pub rollback_anchor_written: bool,
    pub recovery_window_persisted: bool,
    pub recovery_receipt_persisted: bool,
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
pub struct WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackSideEffects {
    pub report_written: bool,
    pub runtime_filesystem_written: bool,
    pub replay_executed: bool,
    pub checkpoint_written: bool,
    pub rollback_anchor_written: bool,
    pub recovery_window_written: bool,
    pub recovery_window_persisted: bool,
    pub recovery_receipt_written: bool,
    pub recovery_receipt_persisted: bool,
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

pub fn hepta_workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_report()
-> WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackReport {
    let source =
        hepta_workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback_report();
    workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_report_from_source(&source)
}

pub fn workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_report_from_source(
    source: &WorkflowTemporalLiteReplayAlignmentRecoveryWindowLocalPersistenceReadbackReport,
) -> WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackReport {
    let entries =
        workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_entries(
            source,
        );
    let recovery_receipt_projection_count = entries
        .iter()
        .filter(|entry| entry.recovery_receipt_projected)
        .count();
    let recovery_receipt_key_count = entries
        .iter()
        .filter(|entry| entry.recovery_receipt_key_projected)
        .count();
    let recovery_receipt_digest_count = entries
        .iter()
        .filter(|entry| entry.recovery_receipt_digest_projected)
        .count();
    let recovery_receipt_ack_count = entries
        .iter()
        .filter(|entry| entry.recovery_receipt_ack_projected)
        .count();
    let replay_alignment_receipt_match_count = entries
        .iter()
        .filter(|entry| entry.replay_alignment_receipt_matches)
        .count();
    let sqlite_readback_validated_count = entries
        .iter()
        .filter(|entry| entry.sqlite_readback_validated)
        .count();
    let recovery_receipt_mismatch_count = entries
        .iter()
        .filter(|entry| entry.recovery_receipt_mismatch_detected)
        .count();
    let replay_executed_count = entries.iter().filter(|entry| entry.replay_executed).count();
    let checkpoint_written_count = entries
        .iter()
        .filter(|entry| entry.checkpoint_written)
        .count();
    let rollback_anchor_written_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_written)
        .count();
    let recovery_window_persisted_count = entries
        .iter()
        .filter(|entry| entry.recovery_window_persisted)
        .count();
    let recovery_receipt_persisted_count = entries
        .iter()
        .filter(|entry| entry.recovery_receipt_persisted)
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
    let ready = source.recovery_window_local_persistence_readback_ready
        && source.source_rollback_consistency_entry_count == 9
        && source.recovery_window_projection_count == 9
        && source.recovery_window_key_count == 9
        && source.recovery_window_digest_count == 9
        && source.replay_alignment_recovery_match_count == 9
        && source.sqlite_readback_validated_count == 9
        && source.recovery_window_mismatch_count == 0
        && source.replay_executed_count == 0
        && source.checkpoint_written_count == 0
        && source.rollback_anchor_written_count == 0
        && source.recovery_window_persisted_count == 0
        && source.work_graph_store_write_count == 0
        && source.event_log_write_count == 0
        && source.sqlite_write_count == 0
        && source.local_tempdb_sqlite_read_covered_by_tests
        && !source.runtime_feature_gate_enabled
        && !source.replay_execution_allowed
        && !source.checkpoint_write_allowed
        && !source.rollback_anchor_write_allowed
        && !source.recovery_window_persistence_allowed
        && !source.work_graph_projection_write_allowed
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.runtime_store_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.entries.len()
        && recovery_receipt_projection_count == entries.len()
        && recovery_receipt_key_count == entries.len()
        && recovery_receipt_digest_count == entries.len()
        && recovery_receipt_ack_count == entries.len()
        && replay_alignment_receipt_match_count == entries.len()
        && sqlite_readback_validated_count == entries.len()
        && recovery_receipt_mismatch_count == 0
        && replay_executed_count == 0
        && checkpoint_written_count == 0
        && rollback_anchor_written_count == 0
        && recovery_window_persisted_count == 0
        && recovery_receipt_persisted_count == 0
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

    WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback",
        status: if ready { "ready_blocked" } else { "blocked" },
        gate:
            WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_RECEIPT_LOCAL_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_RECEIPT_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION,
        source_recovery_window_gate: source.gate,
        source_recovery_window_ready: source.recovery_window_local_persistence_readback_ready,
        source_recovery_window_entry_count: source.entries.len(),
        receipt_scope: "local_persistence_replay_alignment_recovery_receipt_readback_no_execution",
        sqlite_readback_scope: source.sqlite_readback_scope,
        recovery_receipt_projection_count,
        recovery_receipt_key_count,
        recovery_receipt_digest_count,
        recovery_receipt_ack_count,
        replay_alignment_receipt_match_count,
        sqlite_readback_validated_count,
        recovery_receipt_mismatch_count,
        replay_executed_count,
        checkpoint_written_count,
        rollback_anchor_written_count,
        recovery_window_persisted_count,
        recovery_receipt_persisted_count,
        work_graph_store_write_count,
        event_log_write_count,
        sqlite_write_count,
        wal_mode_required: true,
        local_tempdb_sqlite_read_covered_by_tests: true,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        recovery_receipt_contract_readback_materialized: ready,
        replay_execution_allowed: false,
        checkpoint_write_allowed: false,
        rollback_anchor_write_allowed: false,
        recovery_window_persistence_allowed: false,
        recovery_receipt_persistence_allowed: false,
        work_graph_projection_write_allowed: false,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        runtime_store_persistence_allowed: false,
        workflow_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        source_gate_recursion_bounded: true,
        recovery_receipt_local_persistence_readback_ready: ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "source_gate_recursion_bounded_to_report_invariants",
            "replay_execution_disabled",
            "checkpoint_write_disabled",
            "rollback_anchor_write_disabled",
            "recovery_window_persistence_disabled",
            "recovery_receipt_persistence_disabled",
            "work_graph_projection_write_disabled",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "runtime_store_persistence_disabled",
            "workflow_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_RECEIPT_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_entries(
    source: &WorkflowTemporalLiteReplayAlignmentRecoveryWindowLocalPersistenceReadbackReport,
) -> Vec<WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackEntry> {
    workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_entries_from_recovery_window_entries(&source.entries)
}

pub fn workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_entries_from_recovery_window_entries(
    recovery_window_entries: &[WorkflowTemporalLiteReplayAlignmentRecoveryWindowLocalPersistenceReadbackEntry],
) -> Vec<WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackEntry> {
    recovery_window_entries
        .iter()
        .map(|source_entry| {
            let recovery_receipt_key = keyed_recovery_receipt(
                "temporal-lite.local-replay-alignment.recovery-receipt-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let recovery_receipt_ack_key = keyed_recovery_receipt(
                "temporal-lite.local-recovery-receipt.ack-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let recovery_receipt_digest = format!(
                "temporal-lite.local-replay-alignment-recovery-receipt-digest.v1.{:03}.{}.{}.{}.{}",
                source_entry.replay_order,
                source_entry.event_contract_id,
                source_entry.recovery_window_digest.len(),
                source_entry.recovery_window_end_key.len(),
                source_entry.local_sequence
            );
            let replay_alignment_receipt_matches = source_entry.recovery_window_projected
                && source_entry.replay_alignment_recovery_matches
                && source_entry.sqlite_readback_validated
                && !source_entry.recovery_window_mismatch_detected;

            WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackEntry {
                event_contract_id: source_entry.event_contract_id.clone(),
                replay_order: source_entry.replay_order,
                local_sequence: source_entry.local_sequence,
                source_event_id: source_entry.source_event_id.clone(),
                replay_alignment_key: source_entry.replay_alignment_key.clone(),
                projection_replay_key: source_entry.projection_replay_key.clone(),
                rollback_consistency_key: source_entry.rollback_consistency_key.clone(),
                rollback_readback_key: source_entry.rollback_readback_key.clone(),
                recovery_window_key: source_entry.recovery_window_key.clone(),
                recovery_window_start_key: source_entry.recovery_window_start_key.clone(),
                recovery_window_end_key: source_entry.recovery_window_end_key.clone(),
                recovery_window_digest: source_entry.recovery_window_digest.clone(),
                recovery_receipt_key,
                recovery_receipt_ack_key: recovery_receipt_ack_key.clone(),
                recovery_receipt_digest,
                expected_recovery_receipt_key: recovery_receipt_ack_key,
                receipt_state: "recovery_receipt_projected_from_local_persistence_not_written",
                readback_state:
                    "projected_from_sqlite_wal_local_persistence_readback_without_recovery_receipt_writes",
                recovery_window_projected: source_entry.recovery_window_projected,
                recovery_receipt_projected: replay_alignment_receipt_matches,
                recovery_receipt_key_projected: true,
                recovery_receipt_digest_projected: !source_entry.recovery_window_digest.is_empty(),
                recovery_receipt_ack_projected: true,
                replay_alignment_receipt_matches,
                sqlite_readback_validated: source_entry.sqlite_readback_validated,
                recovery_receipt_mismatch_detected: !replay_alignment_receipt_matches,
                replay_executed: false,
                checkpoint_written: false,
                rollback_anchor_written: false,
                recovery_window_persisted: false,
                recovery_receipt_persisted: false,
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

fn keyed_recovery_receipt(prefix: &str, replay_order: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{replay_order:03}.{event_contract_id}")
}

impl WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            runtime_filesystem_written: false,
            replay_executed: false,
            checkpoint_written: false,
            rollback_anchor_written: false,
            recovery_window_written: false,
            recovery_window_persisted: false,
            recovery_receipt_written: false,
            recovery_receipt_persisted: false,
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
    use crate::workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback_entries_from_replay_alignment_entries;
    use crate::workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback_entries_from_rollback_consistency_entries;
    use crate::workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback_entries_from_checkpoint_consistency_entries;
    use crate::workflow_temporal_lite_work_graph_projection_local_persistence_readback_entries_from_adapter_entries;
    use crate::workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_entries_from_projection_entries;

    #[test]
    fn local_recovery_receipt_projects_all_recovery_window_entries_without_writes() {
        let report =
            hepta_workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_recovery_window_ready);
        assert_eq!(report.source_recovery_window_entry_count, 9);
        assert_eq!(report.recovery_receipt_projection_count, 9);
        assert_eq!(report.recovery_receipt_key_count, 9);
        assert_eq!(report.recovery_receipt_digest_count, 9);
        assert_eq!(report.recovery_receipt_ack_count, 9);
        assert_eq!(report.replay_alignment_receipt_match_count, 9);
        assert_eq!(report.sqlite_readback_validated_count, 9);
        assert_eq!(report.recovery_receipt_mismatch_count, 0);
        assert_eq!(report.replay_executed_count, 0);
        assert_eq!(report.checkpoint_written_count, 0);
        assert_eq!(report.rollback_anchor_written_count, 0);
        assert_eq!(report.recovery_window_persisted_count, 0);
        assert_eq!(report.recovery_receipt_persisted_count, 0);
        assert_eq!(report.work_graph_store_write_count, 0);
        assert_eq!(report.event_log_write_count, 0);
        assert_eq!(report.sqlite_write_count, 0);
        assert!(report.recovery_receipt_contract_readback_materialized);
        assert!(report.source_gate_recursion_bounded);
        assert!(report.recovery_receipt_local_persistence_readback_ready);
    }

    #[tokio::test]
    async fn local_recovery_receipt_uses_reopened_sqlite_event_history() {
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
        let checkpoint_entries =
            workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback_entries_from_replay_alignment_entries(&alignment_entries);
        let rollback_entries =
            workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback_entries_from_checkpoint_consistency_entries(&checkpoint_entries);
        let recovery_window_entries =
            workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback_entries_from_rollback_consistency_entries(&rollback_entries);
        let recovery_receipt_entries =
            workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_entries_from_recovery_window_entries(&recovery_window_entries);

        assert_eq!(stored_events.len(), 9);
        assert_eq!(recovery_window_entries.len(), 9);
        assert_eq!(recovery_receipt_entries.len(), 9);
        assert!(recovery_receipt_entries
            .iter()
            .enumerate()
            .all(|(index, entry)| {
                entry.replay_order == index + 1
                    && entry.local_sequence >= 1
                    && entry
                        .recovery_receipt_key
                        .starts_with("temporal-lite.local-replay-alignment.recovery-receipt-readback.")
                    && entry
                        .recovery_receipt_ack_key
                        .starts_with("temporal-lite.local-recovery-receipt.ack-readback.")
                    && entry
                        .recovery_receipt_digest
                        .starts_with("temporal-lite.local-replay-alignment-recovery-receipt-digest.v1.")
                    && entry.expected_recovery_receipt_key == entry.recovery_receipt_ack_key
                    && entry.receipt_state
                        == "recovery_receipt_projected_from_local_persistence_not_written"
                    && entry.readback_state
                        == "projected_from_sqlite_wal_local_persistence_readback_without_recovery_receipt_writes"
                    && entry.recovery_window_projected
                    && entry.recovery_receipt_projected
                    && entry.recovery_receipt_key_projected
                    && entry.recovery_receipt_digest_projected
                    && entry.recovery_receipt_ack_projected
                    && entry.replay_alignment_receipt_matches
                    && entry.sqlite_readback_validated
                    && !entry.recovery_receipt_mismatch_detected
                    && !entry.replay_executed
                    && !entry.checkpoint_written
                    && !entry.rollback_anchor_written
                    && !entry.recovery_window_persisted
                    && !entry.recovery_receipt_persisted
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
    fn local_recovery_receipt_side_effects_remain_closed() {
        let report =
            hepta_workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_report();

        assert!(report.wal_mode_required);
        assert!(report.local_tempdb_sqlite_read_covered_by_tests);
        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.replay_execution_allowed);
        assert!(!report.checkpoint_write_allowed);
        assert!(!report.rollback_anchor_write_allowed);
        assert!(!report.recovery_window_persistence_allowed);
        assert!(!report.recovery_receipt_persistence_allowed);
        assert!(!report.work_graph_projection_write_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.runtime_store_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteReplayAlignmentRecoveryReceiptLocalPersistenceReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.wal_mode_required
                && entry.feature_gate_required
                && entry.sqlite_readback_validated
                && !entry.recovery_receipt_mismatch_detected
                && !entry.replay_executed
                && !entry.checkpoint_written
                && !entry.rollback_anchor_written
                && !entry.recovery_window_persisted
                && !entry.recovery_receipt_persisted
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
