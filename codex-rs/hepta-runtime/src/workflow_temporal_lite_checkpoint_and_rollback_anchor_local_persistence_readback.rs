use serde::Serialize;

use crate::WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackEntry;
use crate::WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackReport;
use crate::hepta_workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_LOCAL_PERSISTENCE_READBACK_GATE:
    &str = "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_replay_validator_gate: &'static str,
    pub source_replay_validator_ready: bool,
    pub source_replay_projection_count: usize,
    pub source_append_only_event_store_interface_ready: bool,
    pub source_replay_validator_derived_from_event_store_interface: bool,
    pub anchor_scope: &'static str,
    pub sqlite_readback_scope: &'static str,
    pub replay_readback_projection_count: usize,
    pub checkpoint_anchor_readback_count: usize,
    pub rollback_anchor_readback_count: usize,
    pub durable_anchor_pair_count: usize,
    pub checkpoint_digest_count: usize,
    pub rollback_digest_count: usize,
    pub anchor_mismatch_count: usize,
    pub wal_mode_required: bool,
    pub local_tempdb_sqlite_read_covered_by_tests: bool,
    pub runtime_feature_gate_enabled: bool,
    pub anchor_readback_materialized: bool,
    pub checkpoint_anchors_derived_from_event_store_interface: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub checkpoint_write_allowed: bool,
    pub rollback_anchor_write_allowed: bool,
    pub anchor_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub checkpoint_and_rollback_anchor_local_persistence_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackEntry {
    pub event_contract_id: String,
    pub replay_order: usize,
    pub local_sequence: i64,
    pub source_event_id: String,
    pub replay_projection_key: String,
    pub checkpoint_anchor_key: String,
    pub rollback_anchor_key: String,
    pub checkpoint_source_key: String,
    pub rollback_source_anchor: String,
    pub replay_batch_digest: String,
    pub checkpoint_readback_digest: String,
    pub rollback_readback_digest: String,
    pub anchor_pair_state: &'static str,
    pub checkpoint_anchor_projected: bool,
    pub rollback_anchor_projected: bool,
    pub durable_anchor_pair_projected: bool,
    pub checkpoint_digest_validated: bool,
    pub rollback_digest_validated: bool,
    pub anchor_mismatch_detected: bool,
    pub sqlite_readback_validated: bool,
    pub wal_mode_required: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub checkpoint_write_allowed: bool,
    pub rollback_anchor_write_allowed: bool,
    pub anchor_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackSideEffects {
    pub report_written: bool,
    pub runtime_filesystem_written: bool,
    pub runtime_event_log_written: bool,
    pub runtime_sqlite_written: bool,
    pub runtime_store_persisted: bool,
    pub checkpoint_written: bool,
    pub rollback_anchor_written: bool,
    pub anchor_persisted: bool,
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

pub fn hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_report()
-> WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackReport {
    let source =
        hepta_workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report();
    workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_report_from_replay_validator(&source)
}

pub fn workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_report_from_replay_validator(
    source: &WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackReport,
) -> WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackReport {
    let entries =
        workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_entries(
            source,
        );
    let checkpoint_anchor_readback_count = entries
        .iter()
        .filter(|entry| entry.checkpoint_anchor_projected)
        .count();
    let rollback_anchor_readback_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_projected)
        .count();
    let durable_anchor_pair_count = entries
        .iter()
        .filter(|entry| entry.durable_anchor_pair_projected)
        .count();
    let checkpoint_digest_count = entries
        .iter()
        .filter(|entry| entry.checkpoint_digest_validated)
        .count();
    let rollback_digest_count = entries
        .iter()
        .filter(|entry| entry.rollback_digest_validated)
        .count();
    let anchor_mismatch_count = entries
        .iter()
        .filter(|entry| entry.anchor_mismatch_detected)
        .count();
    let checkpoint_anchors_derived_from_event_store_interface = source
        .source_append_only_event_store_interface_ready
        && source.replay_validator_derived_from_event_store_interface;
    let ready = source.deterministic_replay_validator_local_persistence_readback_ready
        && checkpoint_anchors_derived_from_event_store_interface
        && source.replay_readback_projection_count == 9
        && source.replay_mismatch_count == 0
        && source.local_tempdb_sqlite_read_covered_by_tests
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.runtime_store_persistence_allowed
        && !source.replay_projection_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.replay_readback_projection_count
        && checkpoint_anchor_readback_count == entries.len()
        && rollback_anchor_readback_count == entries.len()
        && durable_anchor_pair_count == entries.len()
        && checkpoint_digest_count == entries.len()
        && rollback_digest_count == entries.len()
        && anchor_mismatch_count == 0
        && entries.iter().all(|entry| {
            entry.sqlite_readback_validated
                && entry.wal_mode_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.checkpoint_write_allowed
                && !entry.rollback_anchor_write_allowed
                && !entry.anchor_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback",
        status: if ready { "ready_blocked" } else { "blocked" },
        gate: WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_LOCAL_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION,
        source_replay_validator_gate: source.gate,
        source_replay_validator_ready: source.deterministic_replay_validator_local_persistence_readback_ready,
        source_replay_projection_count: source.replay_readback_projection_count,
        source_append_only_event_store_interface_ready: source.source_append_only_event_store_interface_ready,
        source_replay_validator_derived_from_event_store_interface: source.replay_validator_derived_from_event_store_interface,
        anchor_scope: "local_persistence_checkpoint_and_rollback_anchor_readback_no_writes",
        sqlite_readback_scope: source.sqlite_readback_scope,
        replay_readback_projection_count: source.replay_readback_projection_count,
        checkpoint_anchor_readback_count,
        rollback_anchor_readback_count,
        durable_anchor_pair_count,
        checkpoint_digest_count,
        rollback_digest_count,
        anchor_mismatch_count,
        wal_mode_required: true,
        local_tempdb_sqlite_read_covered_by_tests: true,
        runtime_feature_gate_enabled: false,
        anchor_readback_materialized: ready,
        checkpoint_anchors_derived_from_event_store_interface,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        runtime_store_persistence_allowed: false,
        checkpoint_write_allowed: false,
        rollback_anchor_write_allowed: false,
        anchor_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        checkpoint_and_rollback_anchor_local_persistence_readback_ready: ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "runtime_store_persistence_disabled",
            "checkpoint_write_disabled",
            "rollback_anchor_write_disabled",
            "anchor_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_entries(
    source: &WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackReport,
) -> Vec<WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackEntry> {
    workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_entries_from_replay_entries(&source.entries)
}

pub fn workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_entries_from_replay_entries(
    replay_entries: &[WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackEntry],
) -> Vec<WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackEntry> {
    replay_entries
        .iter()
        .map(|source_entry| {
            let checkpoint_anchor_key = format!(
                "temporal-lite.local-checkpoint-anchor.{:03}.{}",
                source_entry.replay_order, source_entry.event_contract_id
            );
            let rollback_anchor_key = format!(
                "temporal-lite.local-rollback-anchor.{:03}.{}",
                source_entry.replay_order, source_entry.event_contract_id
            );
            let checkpoint_readback_digest = local_anchor_digest(
                "checkpoint",
                source_entry.replay_order,
                &source_entry.event_contract_id,
                &source_entry.replay_checksum,
                &source_entry.replay_batch_digest,
            );
            let rollback_readback_digest = local_anchor_digest(
                "rollback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
                &source_entry.replay_checksum,
                &source_entry.replay_batch_digest,
            );

            WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackEntry {
                event_contract_id: source_entry.event_contract_id.clone(),
                replay_order: source_entry.replay_order,
                local_sequence: source_entry.local_sequence,
                source_event_id: source_entry.source_event_id.clone(),
                replay_projection_key: source_entry.replay_projection_key.clone(),
                checkpoint_anchor_key,
                rollback_anchor_key,
                checkpoint_source_key: source_entry.checkpoint_key.clone(),
                rollback_source_anchor: source_entry.rollback_anchor.clone(),
                replay_batch_digest: source_entry.replay_batch_digest.clone(),
                checkpoint_readback_digest,
                rollback_readback_digest,
                anchor_pair_state:
                    "projected_from_local_persistence_readback_without_anchor_writes",
                checkpoint_anchor_projected: source_entry.checkpoint_key_replayed,
                rollback_anchor_projected: source_entry.rollback_anchor_replayed,
                durable_anchor_pair_projected: source_entry.checkpoint_key_replayed
                    && source_entry.rollback_anchor_replayed,
                checkpoint_digest_validated: source_entry.replay_checksum_validated
                    && source_entry.replay_batch_digest_validated,
                rollback_digest_validated: source_entry.replay_checksum_validated
                    && source_entry.replay_batch_digest_validated,
                anchor_mismatch_detected: source_entry.replay_mismatch_detected,
                sqlite_readback_validated: source_entry.sqlite_readback_validated,
                wal_mode_required: source_entry.wal_mode_required,
                feature_gate_required: source_entry.feature_gate_required,
                runtime_feature_gate_enabled: source_entry.runtime_feature_gate_enabled,
                runtime_event_log_write_allowed: source_entry.runtime_event_log_write_allowed,
                runtime_sqlite_write_allowed: source_entry.runtime_sqlite_write_allowed,
                runtime_store_persistence_allowed: source_entry.runtime_store_persistence_allowed,
                checkpoint_write_allowed: false,
                rollback_anchor_write_allowed: false,
                anchor_persistence_allowed: false,
                workflow_execution_allowed: source_entry.workflow_execution_allowed,
                replay_execution_allowed: source_entry.replay_execution_allowed,
                rollback_execution_allowed: source_entry.rollback_execution_allowed,
                live_execution_allowed: source_entry.live_execution_allowed,
            }
        })
        .collect()
}

fn local_anchor_digest(
    anchor_kind: &str,
    replay_order: usize,
    event_contract_id: &str,
    replay_checksum: &str,
    replay_batch_digest: &str,
) -> String {
    format!(
        "temporal-lite.local-{anchor_kind}-anchor-digest.v1.{replay_order:03}.{event_contract_id}.{}.{}",
        replay_checksum.len(),
        replay_batch_digest.len()
    )
}

impl WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            runtime_filesystem_written: false,
            runtime_event_log_written: false,
            runtime_sqlite_written: false,
            runtime_store_persisted: false,
            checkpoint_written: false,
            rollback_anchor_written: false,
            anchor_persisted: false,
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
    use crate::workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_entries_from_stored_events;
    use crate::workflow_temporal_lite_minimal_local_events_from_test_implementation;

    #[test]
    fn local_checkpoint_and_rollback_anchor_readback_projects_all_replay_entries() {
        let report =
            hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_replay_validator_ready);
        assert_eq!(report.source_replay_projection_count, 9);
        assert!(report.source_append_only_event_store_interface_ready);
        assert!(report.source_replay_validator_derived_from_event_store_interface);
        assert_eq!(report.checkpoint_anchor_readback_count, 9);
        assert_eq!(report.rollback_anchor_readback_count, 9);
        assert_eq!(report.durable_anchor_pair_count, 9);
        assert_eq!(report.checkpoint_digest_count, 9);
        assert_eq!(report.rollback_digest_count, 9);
        assert_eq!(report.anchor_mismatch_count, 0);
        assert!(report.local_tempdb_sqlite_read_covered_by_tests);
        assert!(report.anchor_readback_materialized);
        assert!(report.checkpoint_anchors_derived_from_event_store_interface);
        assert!(report.checkpoint_and_rollback_anchor_local_persistence_readback_ready);
    }

    #[tokio::test]
    async fn local_checkpoint_and_rollback_anchor_readback_uses_reopened_sqlite_history() {
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

        assert_eq!(stored_events.len(), 9);
        assert_eq!(replay_entries.len(), 9);
        assert_eq!(anchor_entries.len(), 9);
        assert!(anchor_entries.iter().enumerate().all(|(index, entry)| {
            entry.replay_order == index + 1
                && entry.local_sequence >= 1
                && entry
                    .checkpoint_anchor_key
                    .starts_with("temporal-lite.local-checkpoint-anchor.")
                && entry
                    .rollback_anchor_key
                    .starts_with("temporal-lite.local-rollback-anchor.")
                && entry
                    .checkpoint_readback_digest
                    .starts_with("temporal-lite.local-checkpoint-anchor-digest.v1.")
                && entry
                    .rollback_readback_digest
                    .starts_with("temporal-lite.local-rollback-anchor-digest.v1.")
                && entry.anchor_pair_state
                    == "projected_from_local_persistence_readback_without_anchor_writes"
                && entry.checkpoint_anchor_projected
                && entry.rollback_anchor_projected
                && entry.durable_anchor_pair_projected
                && entry.checkpoint_digest_validated
                && entry.rollback_digest_validated
                && !entry.anchor_mismatch_detected
                && entry.sqlite_readback_validated
                && !entry.checkpoint_write_allowed
                && !entry.rollback_anchor_write_allowed
                && !entry.anchor_persistence_allowed
                && !entry.replay_execution_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn local_checkpoint_and_rollback_anchor_readback_keeps_writes_closed() {
        let report =
            hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_report();

        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.runtime_store_persistence_allowed);
        assert!(!report.checkpoint_write_allowed);
        assert!(!report.rollback_anchor_write_allowed);
        assert!(!report.anchor_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.wal_mode_required
                && entry.feature_gate_required
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.checkpoint_write_allowed
                && !entry.rollback_anchor_write_allowed
                && !entry.anchor_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }
}
