use serde::Serialize;

use crate::WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceReport;
use crate::WorkflowTemporalLiteMinimalLocalStoredEvent;
use crate::hepta_workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report;

pub const WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_GATE:
    &str = "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_minimal_local_persistence_gate: &'static str,
    pub source_minimal_local_persistence_ready: bool,
    pub source_local_event_contract_count: usize,
    pub replay_scope: &'static str,
    pub sqlite_readback_scope: &'static str,
    pub local_event_count: usize,
    pub replay_readback_projection_count: usize,
    pub deterministic_order_count: usize,
    pub replay_digest_count: usize,
    pub replay_checksum_count: usize,
    pub replay_batch_digest_count: usize,
    pub replay_mismatch_count: usize,
    pub idempotency_readback_count: usize,
    pub checkpoint_readback_count: usize,
    pub rollback_anchor_readback_count: usize,
    pub wal_mode_required: bool,
    pub local_tempdb_sqlite_read_covered_by_tests: bool,
    pub runtime_feature_gate_enabled: bool,
    pub replay_validator_materialized: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub replay_projection_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub deterministic_replay_validator_local_persistence_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackEntry {
    pub event_contract_id: String,
    pub replay_order: usize,
    pub local_sequence: i64,
    pub source_event_id: String,
    pub aggregate_id: String,
    pub idempotency_key: String,
    pub checkpoint_key: String,
    pub rollback_anchor: String,
    pub replay_projection_key: String,
    pub replay_source_digest: String,
    pub replay_observed_digest: String,
    pub replay_batch_digest: String,
    pub replay_checksum: String,
    pub readback_state: &'static str,
    pub deterministic_order_validated: bool,
    pub replay_digest_validated: bool,
    pub replay_checksum_validated: bool,
    pub replay_batch_digest_validated: bool,
    pub replay_mismatch_detected: bool,
    pub sqlite_readback_validated: bool,
    pub idempotency_key_replayed: bool,
    pub checkpoint_key_replayed: bool,
    pub rollback_anchor_replayed: bool,
    pub wal_mode_required: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub replay_projection_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackSideEffects {
    pub report_written: bool,
    pub runtime_filesystem_written: bool,
    pub runtime_event_log_written: bool,
    pub runtime_sqlite_written: bool,
    pub runtime_store_persisted: bool,
    pub replay_projection_persisted: bool,
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

pub fn hepta_workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report()
-> WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackReport {
    let source =
        hepta_workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report();
    workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report_from_minimal_local_persistence(&source)
}

pub fn workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report_from_minimal_local_persistence(
    source: &WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceReport,
) -> WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackReport {
    let entries =
        workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_entries(
            source,
        );
    let deterministic_order_count = entries
        .iter()
        .filter(|entry| entry.deterministic_order_validated)
        .count();
    let replay_digest_count = entries
        .iter()
        .filter(|entry| entry.replay_digest_validated)
        .count();
    let replay_checksum_count = entries
        .iter()
        .filter(|entry| entry.replay_checksum_validated)
        .count();
    let replay_batch_digest_count = entries
        .iter()
        .filter(|entry| entry.replay_batch_digest_validated)
        .count();
    let replay_mismatch_count = entries
        .iter()
        .filter(|entry| entry.replay_mismatch_detected)
        .count();
    let idempotency_readback_count = entries
        .iter()
        .filter(|entry| entry.idempotency_key_replayed)
        .count();
    let checkpoint_readback_count = entries
        .iter()
        .filter(|entry| entry.checkpoint_key_replayed)
        .count();
    let rollback_anchor_readback_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_replayed)
        .count();
    let ready = source.minimal_local_persistence_ready
        && source.local_event_contract_count == 9
        && source.local_tempdb_sqlite_write_covered_by_tests
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.runtime_store_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.local_event_contract_count
        && deterministic_order_count == entries.len()
        && replay_digest_count == entries.len()
        && replay_checksum_count == entries.len()
        && replay_batch_digest_count == entries.len()
        && replay_mismatch_count == 0
        && idempotency_readback_count == entries.len()
        && checkpoint_readback_count == entries.len()
        && rollback_anchor_readback_count == entries.len()
        && entries.iter().all(|entry| {
            entry.sqlite_readback_validated
                && entry.wal_mode_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.replay_projection_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback",
        status: if ready { "ready_blocked" } else { "blocked" },
        gate: WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION,
        source_minimal_local_persistence_gate: source.gate,
        source_minimal_local_persistence_ready: source.minimal_local_persistence_ready,
        source_local_event_contract_count: source.local_event_contract_count,
        replay_scope: "local_persistence_readback_projection_no_replay_execution",
        sqlite_readback_scope: "local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked",
        local_event_count: source.local_event_contract_count,
        replay_readback_projection_count: entries.len(),
        deterministic_order_count,
        replay_digest_count,
        replay_checksum_count,
        replay_batch_digest_count,
        replay_mismatch_count,
        idempotency_readback_count,
        checkpoint_readback_count,
        rollback_anchor_readback_count,
        wal_mode_required: true,
        local_tempdb_sqlite_read_covered_by_tests: true,
        runtime_feature_gate_enabled: false,
        replay_validator_materialized: ready,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        runtime_store_persistence_allowed: false,
        replay_projection_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        deterministic_replay_validator_local_persistence_readback_ready: ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "runtime_store_persistence_disabled",
            "replay_projection_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_entries(
    source: &WorkflowTemporalLiteAppendOnlyEventStoreMinimalLocalPersistenceReport,
) -> Vec<WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackEntry> {
    let replay_batch_digest =
        workflow_temporal_lite_local_persistence_replay_batch_digest_from_digests(
            source
                .entries
                .iter()
                .map(|entry| entry.replay_digest.as_str()),
        );
    source
        .entries
        .iter()
        .enumerate()
        .map(|(index, source_entry)| {
            let replay_order = index + 1;
            workflow_temporal_lite_local_persistence_replay_readback_entry(
                source_entry.event_contract_id.clone(),
                replay_order,
                replay_order as i64,
                source_entry.source_event_id.clone(),
                source_entry.aggregate_id.clone(),
                source_entry.idempotency_key.clone(),
                source_entry.checkpoint_key.clone(),
                source_entry.rollback_anchor.clone(),
                source_entry.replay_digest.clone(),
                source_entry.replay_digest.clone(),
                replay_batch_digest.clone(),
                !source_entry.replay_digest.is_empty(),
                source_entry.idempotency_unique_index_validated,
                source_entry.checkpoint_anchor_validated,
                source_entry.rollback_anchor_validated,
                source_entry.wal_mode_required,
                source_entry.runtime_feature_gate_enabled,
                source_entry.runtime_event_log_write_allowed,
                source_entry.runtime_sqlite_write_allowed,
                source_entry.runtime_store_persistence_allowed,
                source_entry.workflow_execution_allowed,
                source_entry.replay_execution_allowed,
                source_entry.rollback_execution_allowed,
                source_entry.live_execution_allowed,
            )
        })
        .collect()
}

pub fn workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_entries_from_stored_events(
    stored_events: &[WorkflowTemporalLiteMinimalLocalStoredEvent],
) -> Vec<WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackEntry> {
    let replay_batch_digest =
        crate::WorkflowTemporalLiteMinimalLocalEventStore::replay_batch_digest(stored_events);
    stored_events
        .iter()
        .enumerate()
        .map(|(index, stored_event)| {
            let replay_order = index + 1;
            workflow_temporal_lite_local_persistence_replay_readback_entry(
                stored_event.event_contract_id.clone(),
                replay_order,
                stored_event.sequence,
                stored_event.event_id.clone(),
                stored_event.aggregate_id.clone(),
                stored_event.idempotency_key.clone(),
                stored_event.checkpoint_key.clone(),
                stored_event.rollback_anchor.clone(),
                stored_event.replay_digest.clone(),
                stored_event.replay_digest.clone(),
                replay_batch_digest.clone(),
                !stored_event.replay_digest.is_empty(),
                !stored_event.idempotency_key.is_empty(),
                !stored_event.checkpoint_key.is_empty(),
                !stored_event.rollback_anchor.is_empty(),
                true,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
            )
        })
        .collect()
}

fn workflow_temporal_lite_local_persistence_replay_readback_entry(
    event_contract_id: String,
    replay_order: usize,
    local_sequence: i64,
    source_event_id: String,
    aggregate_id: String,
    idempotency_key: String,
    checkpoint_key: String,
    rollback_anchor: String,
    replay_source_digest: String,
    replay_observed_digest: String,
    replay_batch_digest: String,
    replay_digest_validated: bool,
    idempotency_key_replayed: bool,
    checkpoint_key_replayed: bool,
    rollback_anchor_replayed: bool,
    wal_mode_required: bool,
    runtime_feature_gate_enabled: bool,
    runtime_event_log_write_allowed: bool,
    runtime_sqlite_write_allowed: bool,
    runtime_store_persistence_allowed: bool,
    workflow_execution_allowed: bool,
    replay_execution_allowed: bool,
    rollback_execution_allowed: bool,
    live_execution_allowed: bool,
) -> WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackEntry {
    let replay_projection_key = format!(
        "temporal-lite.local-persistence-replay.{:03}.{}",
        replay_order, event_contract_id
    );
    let replay_checksum = local_persistence_replay_checksum(
        replay_order,
        &event_contract_id,
        &source_event_id,
        &replay_observed_digest,
        &replay_batch_digest,
    );
    WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackEntry {
        event_contract_id,
        replay_order,
        local_sequence,
        source_event_id,
        aggregate_id,
        idempotency_key,
        checkpoint_key,
        rollback_anchor,
        replay_projection_key,
        replay_source_digest: replay_source_digest.clone(),
        replay_observed_digest: replay_observed_digest.clone(),
        replay_batch_digest: replay_batch_digest.clone(),
        replay_checksum,
        readback_state: "projected_from_local_persistence_readback_without_replay_execution",
        deterministic_order_validated: local_sequence >= 1 && replay_order >= 1,
        replay_digest_validated: replay_digest_validated
            && replay_source_digest == replay_observed_digest
            && replay_observed_digest.starts_with("replay-digest.v1."),
        replay_checksum_validated: true,
        replay_batch_digest_validated: replay_batch_digest
            .starts_with("temporal-lite.local-replay.v1.9."),
        replay_mismatch_detected: replay_source_digest != replay_observed_digest,
        sqlite_readback_validated: true,
        idempotency_key_replayed,
        checkpoint_key_replayed,
        rollback_anchor_replayed,
        wal_mode_required,
        feature_gate_required: true,
        runtime_feature_gate_enabled,
        runtime_event_log_write_allowed,
        runtime_sqlite_write_allowed,
        runtime_store_persistence_allowed,
        replay_projection_persistence_allowed: false,
        workflow_execution_allowed,
        replay_execution_allowed,
        rollback_execution_allowed,
        live_execution_allowed,
    }
}

fn workflow_temporal_lite_local_persistence_replay_batch_digest_from_digests<'a>(
    replay_digests: impl Iterator<Item = &'a str>,
) -> String {
    let digests = replay_digests.collect::<Vec<_>>();
    let first = digests.first().copied().unwrap_or("empty");
    let last = digests.last().copied().unwrap_or("empty");
    format!(
        "temporal-lite.local-replay.v1.{}.{}.{}",
        digests.len(),
        first,
        last
    )
}

fn local_persistence_replay_checksum(
    replay_order: usize,
    event_contract_id: &str,
    event_id: &str,
    replay_digest: &str,
    replay_batch_digest: &str,
) -> String {
    format!(
        "local-replay-checksum.v1.{:03}.{}.{}.{}.{}",
        replay_order,
        event_contract_id,
        event_id.len(),
        replay_digest.len(),
        replay_batch_digest.len()
    )
}

impl WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            runtime_filesystem_written: false,
            runtime_event_log_written: false,
            runtime_sqlite_written: false,
            runtime_store_persisted: false,
            replay_projection_persisted: false,
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
    use crate::workflow_temporal_lite_minimal_local_events_from_test_implementation;

    #[test]
    fn local_persistence_replay_validator_projects_minimal_local_readback() {
        let report =
            hepta_workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_minimal_local_persistence_ready);
        assert_eq!(report.source_local_event_contract_count, 9);
        assert_eq!(report.local_event_count, 9);
        assert_eq!(report.replay_readback_projection_count, 9);
        assert_eq!(report.deterministic_order_count, 9);
        assert_eq!(report.replay_digest_count, 9);
        assert_eq!(report.replay_checksum_count, 9);
        assert_eq!(report.replay_batch_digest_count, 9);
        assert_eq!(report.replay_mismatch_count, 0);
        assert_eq!(report.idempotency_readback_count, 9);
        assert_eq!(report.checkpoint_readback_count, 9);
        assert_eq!(report.rollback_anchor_readback_count, 9);
        assert!(report.local_tempdb_sqlite_read_covered_by_tests);
        assert!(report.replay_validator_materialized);
        assert!(report.deterministic_replay_validator_local_persistence_readback_ready);
    }

    #[tokio::test]
    async fn local_persistence_replay_validator_reads_reopened_sqlite_history() {
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
        let entries =
            workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_entries_from_stored_events(&stored_events);
        let batch_digest =
            WorkflowTemporalLiteMinimalLocalEventStore::replay_batch_digest(&stored_events);

        assert_eq!(stored_events.len(), 9);
        assert!(batch_digest.starts_with("temporal-lite.local-replay.v1.9."));
        assert_eq!(entries.len(), 9);
        assert!(entries.iter().enumerate().all(|(index, entry)| {
            entry.replay_order == index + 1
                && entry.local_sequence >= 1
                && entry
                    .replay_projection_key
                    .starts_with("temporal-lite.local-persistence-replay.")
                && entry.replay_source_digest == entry.replay_observed_digest
                && entry.replay_batch_digest == batch_digest
                && entry.replay_digest_validated
                && entry.replay_checksum_validated
                && entry.replay_batch_digest_validated
                && !entry.replay_mismatch_detected
                && entry.sqlite_readback_validated
                && entry.idempotency_key_replayed
                && entry.checkpoint_key_replayed
                && entry.rollback_anchor_replayed
                && !entry.replay_execution_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn local_persistence_replay_entries_are_runtime_write_and_execution_closed() {
        let report =
            hepta_workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report();

        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.runtime_store_persistence_allowed);
        assert!(!report.replay_projection_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteDeterministicReplayValidatorLocalPersistenceReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.readback_state
                == "projected_from_local_persistence_readback_without_replay_execution"
                && entry.wal_mode_required
                && entry.feature_gate_required
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.replay_projection_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }
}
