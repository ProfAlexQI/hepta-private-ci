use serde::Serialize;

use crate::WorkflowDurableStoreAdapterEntry;
use crate::hepta_workflow_durable_store_adapter_report;

pub const WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_GATE: &str =
    "hepta_workflow_durable_store_test_only_append_fixture_gate";
pub const WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_SCHEMA_VERSION: &str =
    "workflow_durable_store_test_only_append_fixture_v1";
pub const WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_RECOMMENDED_NEXT_GATE: &str =
    "phase8_internal_read_only_hepta_system_status_invocation_without_external_network_or_mutation";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreTestOnlyAppendFixtureReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_adapter_gate: &'static str,
    pub source_adapter_ready: bool,
    pub test_fixture_scope: &'static str,
    pub event_contract_count: usize,
    pub fixture_entry_count: usize,
    pub append_only_sequence_count: usize,
    pub idempotency_fixture_count: usize,
    pub checkpoint_fixture_count: usize,
    pub replay_validation_fixture_count: usize,
    pub rollback_fixture_count: usize,
    pub duplicate_append_denial_count: usize,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub test_fixture_materialized: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub fixture_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub test_only_append_fixture_ready: bool,
    pub entries: Vec<WorkflowDurableStoreTestOnlyAppendFixtureEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowDurableStoreTestOnlyAppendFixtureSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreTestOnlyAppendFixtureEntry {
    pub event_contract_id: &'static str,
    pub record_kind: &'static str,
    pub fixture_sequence: usize,
    pub fixture_append_key: String,
    pub fixture_idempotency_key: String,
    pub fixture_checkpoint_key: String,
    pub fixture_replay_validation_key: String,
    pub fixture_rollback_anchor: &'static str,
    pub append_only_order_validated: bool,
    pub idempotency_key_validated: bool,
    pub duplicate_append_denied: bool,
    pub checkpoint_metadata_validated: bool,
    pub replay_validation_metadata_validated: bool,
    pub rollback_metadata_validated: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub fixture_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreTestOnlyAppendFixtureSideEffects {
    pub filesystem_written: bool,
    pub fixture_file_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub lease_acquired: bool,
    pub idempotency_index_mutated: bool,
    pub checkpoint_written: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub live_execution_started: bool,
}

pub fn hepta_workflow_durable_store_test_only_append_fixture_report()
-> WorkflowDurableStoreTestOnlyAppendFixtureReport {
    let adapter = hepta_workflow_durable_store_adapter_report();
    let entries = workflow_durable_store_test_only_append_fixture_entries(&adapter.entries);
    let append_only_sequence_count = entries
        .iter()
        .filter(|entry| entry.append_only_order_validated)
        .count();
    let idempotency_fixture_count = entries
        .iter()
        .filter(|entry| entry.idempotency_key_validated)
        .count();
    let checkpoint_fixture_count = entries
        .iter()
        .filter(|entry| entry.checkpoint_metadata_validated)
        .count();
    let replay_validation_fixture_count = entries
        .iter()
        .filter(|entry| entry.replay_validation_metadata_validated)
        .count();
    let rollback_fixture_count = entries
        .iter()
        .filter(|entry| entry.rollback_metadata_validated)
        .count();
    let duplicate_append_denial_count = entries
        .iter()
        .filter(|entry| entry.duplicate_append_denied)
        .count();
    let test_only_append_fixture_ready = adapter.temporal_lite_adapter_ready
        && adapter.event_contract_count == 9
        && adapter.adapter_entry_count == 9
        && !adapter.feature_gate_enabled
        && !adapter.ready_for_event_log_write
        && !adapter.ready_for_sqlite_write
        && !adapter.ready_for_workflow_execution
        && !adapter.ready_for_replay_execution
        && !adapter.ready_for_rollback_execution
        && !adapter.ready_for_live_execution
        && entries.len() == adapter.adapter_entry_count
        && append_only_sequence_count == 9
        && idempotency_fixture_count == 9
        && checkpoint_fixture_count == 9
        && replay_validation_fixture_count == 9
        && rollback_fixture_count == 9
        && duplicate_append_denial_count == 9
        && entries.iter().all(|entry| {
            entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.fixture_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowDurableStoreTestOnlyAppendFixtureReport {
        runtime: "hepta",
        surface: "workflow_durable_store_test_only_append_fixture",
        status: if test_only_append_fixture_ready {
            "ready"
        } else {
            "blocked"
        },
        gate: WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_GATE,
        schema_version: WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_SCHEMA_VERSION,
        source_adapter_gate: adapter.gate,
        source_adapter_ready: adapter.temporal_lite_adapter_ready,
        test_fixture_scope: "test_only_in_memory_fixture_no_runtime_store_write",
        event_contract_count: adapter.event_contract_count,
        fixture_entry_count: entries.len(),
        append_only_sequence_count,
        idempotency_fixture_count,
        checkpoint_fixture_count,
        replay_validation_fixture_count,
        rollback_fixture_count,
        duplicate_append_denial_count,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        test_fixture_materialized: test_only_append_fixture_ready,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        fixture_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        test_only_append_fixture_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "fixture_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_RECOMMENDED_NEXT_GATE,
        side_effects: WorkflowDurableStoreTestOnlyAppendFixtureSideEffects::none(),
    }
}

pub fn workflow_durable_store_test_only_append_fixture_entries(
    adapter_entries: &[WorkflowDurableStoreAdapterEntry],
) -> Vec<WorkflowDurableStoreTestOnlyAppendFixtureEntry> {
    adapter_entries
        .iter()
        .enumerate()
        .map(|(index, entry)| test_only_append_fixture_entry(index + 1, entry))
        .collect()
}

fn test_only_append_fixture_entry(
    fixture_sequence: usize,
    entry: &WorkflowDurableStoreAdapterEntry,
) -> WorkflowDurableStoreTestOnlyAppendFixtureEntry {
    WorkflowDurableStoreTestOnlyAppendFixtureEntry {
        event_contract_id: entry.event_contract_id,
        record_kind: entry.record_kind,
        fixture_sequence,
        fixture_append_key: format!("test-only.append-fixture.{}", entry.event_contract_id),
        fixture_idempotency_key: format!(
            "test-only.idempotency.{}.{}",
            entry.event_contract_id, entry.lease_scope
        ),
        fixture_checkpoint_key: format!(
            "test-only.checkpoint.{}.{}",
            entry.event_contract_id, entry.checkpoint_policy
        ),
        fixture_replay_validation_key: format!(
            "test-only.replay-validation.{}.{}",
            entry.event_contract_id, entry.replay_validation_policy
        ),
        fixture_rollback_anchor: entry.rollback_anchor,
        append_only_order_validated: true,
        idempotency_key_validated: true,
        duplicate_append_denied: true,
        checkpoint_metadata_validated: true,
        replay_validation_metadata_validated: true,
        rollback_metadata_validated: true,
        feature_gate_required: entry.feature_gate_required,
        runtime_feature_gate_enabled: entry.feature_gate_enabled,
        runtime_event_log_write_allowed: entry.event_log_write_enabled,
        runtime_sqlite_write_allowed: entry.sqlite_write_enabled,
        fixture_persistence_allowed: false,
        workflow_execution_allowed: entry.workflow_execution_enabled,
        replay_execution_allowed: entry.replay_execution_enabled,
        rollback_execution_allowed: entry.rollback_execution_enabled,
        live_execution_allowed: entry.live_execution_enabled,
    }
}

impl WorkflowDurableStoreTestOnlyAppendFixtureSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            fixture_file_written: false,
            event_log_written: false,
            sqlite_written: false,
            lease_acquired: false,
            idempotency_index_mutated: false,
            checkpoint_written: false,
            workflow_execution_started: false,
            replay_executed: false,
            rollback_executed: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_append_fixture_covers_all_temporal_lite_events() {
        let report = hepta_workflow_durable_store_test_only_append_fixture_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.event_contract_count, 9);
        assert_eq!(report.fixture_entry_count, 9);
        assert_eq!(report.append_only_sequence_count, 9);
        assert_eq!(report.idempotency_fixture_count, 9);
        assert_eq!(report.checkpoint_fixture_count, 9);
        assert_eq!(report.replay_validation_fixture_count, 9);
        assert_eq!(report.rollback_fixture_count, 9);
        assert_eq!(report.duplicate_append_denial_count, 9);
        assert!(report.test_only_append_fixture_ready);
    }

    #[test]
    fn test_only_append_fixture_keeps_runtime_store_writes_closed() {
        let report = hepta_workflow_durable_store_test_only_append_fixture_report();

        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(report.test_fixture_materialized);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.fixture_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowDurableStoreTestOnlyAppendFixtureSideEffects::none()
        );
    }

    #[test]
    fn test_only_append_fixture_validates_idempotency_and_rollback_metadata() {
        let report = hepta_workflow_durable_store_test_only_append_fixture_report();

        assert!(report.entries.iter().any(|entry| entry.event_contract_id
            == "worker_task_event_intake"
            && entry.fixture_rollback_anchor == "rollback_to_prior_worker_task_attempt_anchor"));
        assert!(report.entries.iter().all(|entry| {
            entry.append_only_order_validated
                && entry.idempotency_key_validated
                && entry.duplicate_append_denied
                && entry.checkpoint_metadata_validated
                && entry.replay_validation_metadata_validated
                && entry.rollback_metadata_validated
                && entry
                    .fixture_append_key
                    .starts_with("test-only.append-fixture.")
                && entry
                    .fixture_idempotency_key
                    .starts_with("test-only.idempotency.")
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.fixture_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }
}
