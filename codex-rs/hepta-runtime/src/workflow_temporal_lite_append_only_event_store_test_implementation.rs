use std::collections::BTreeSet;

use serde::Serialize;

use crate::WorkflowDurableStoreTestOnlyAppendFixtureReport;
use crate::hepta_workflow_durable_store_test_only_append_fixture_report;

pub const WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_GATE: &str =
    "workflow_temporal_lite_append_only_event_store_test_implementation_gate";
pub const WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_SCHEMA_VERSION: &str =
    "workflow_temporal_lite_append_only_event_store_test_implementation_v1";
pub const WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_RECOMMENDED_NEXT_GATE:
    &str = "temporal_lite_deterministic_replay_validator_feature_gated_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_fixture_gate: &'static str,
    pub source_fixture_ready: bool,
    pub test_store_scope: &'static str,
    pub event_contract_count: usize,
    pub test_event_count: usize,
    pub append_attempt_count: usize,
    pub accepted_append_count: usize,
    pub duplicate_append_denial_count: usize,
    pub append_only_sequence_count: usize,
    pub idempotency_index_entry_count: usize,
    pub checkpoint_anchor_count: usize,
    pub replay_digest_count: usize,
    pub rollback_anchor_count: usize,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub in_memory_store_materialized: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub store_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub append_only_event_store_test_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationEntry {
    pub event_contract_id: &'static str,
    pub record_kind: &'static str,
    pub sequence: usize,
    pub event_id: String,
    pub aggregate_id: String,
    pub idempotency_key: String,
    pub checkpoint_key: String,
    pub replay_digest: String,
    pub rollback_anchor: &'static str,
    pub append_state: &'static str,
    pub duplicate_append_state: &'static str,
    pub appended_to_in_memory_store: bool,
    pub append_only_order_validated: bool,
    pub idempotency_index_validated: bool,
    pub duplicate_append_denied: bool,
    pub checkpoint_anchor_projected: bool,
    pub replay_digest_projected: bool,
    pub rollback_anchor_projected: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub store_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationSideEffects {
    pub filesystem_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub store_persisted: bool,
    pub lease_acquired: bool,
    pub idempotency_index_persisted: bool,
    pub checkpoint_written: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowTemporalLiteAppendOnlyTestEvent {
    pub event_contract_id: &'static str,
    pub record_kind: &'static str,
    pub sequence: usize,
    pub event_id: String,
    pub aggregate_id: String,
    pub idempotency_key: String,
    pub checkpoint_key: String,
    pub replay_digest: String,
    pub rollback_anchor: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowTemporalLiteAppendAttempt {
    pub accepted: bool,
    pub duplicate_denied: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkflowTemporalLiteAppendOnlyTestStore {
    events: Vec<WorkflowTemporalLiteAppendOnlyTestEvent>,
    idempotency_keys: BTreeSet<String>,
}

impl WorkflowTemporalLiteAppendOnlyTestStore {
    pub fn append(
        &mut self,
        event: WorkflowTemporalLiteAppendOnlyTestEvent,
    ) -> WorkflowTemporalLiteAppendAttempt {
        if !self.idempotency_keys.insert(event.idempotency_key.clone()) {
            return WorkflowTemporalLiteAppendAttempt {
                accepted: false,
                duplicate_denied: true,
            };
        }

        self.events.push(event);
        WorkflowTemporalLiteAppendAttempt {
            accepted: true,
            duplicate_denied: false,
        }
    }

    pub fn events(&self) -> &[WorkflowTemporalLiteAppendOnlyTestEvent] {
        &self.events
    }

    pub fn idempotency_key_count(&self) -> usize {
        self.idempotency_keys.len()
    }
}

pub fn hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report()
-> WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport {
    let fixture = hepta_workflow_durable_store_test_only_append_fixture_report();
    workflow_temporal_lite_append_only_event_store_test_implementation_report_from_fixture(&fixture)
}

pub fn workflow_temporal_lite_append_only_event_store_test_implementation_report_from_fixture(
    fixture: &WorkflowDurableStoreTestOnlyAppendFixtureReport,
) -> WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport {
    let (entries, store, duplicate_append_denial_count) =
        workflow_temporal_lite_append_only_event_store_test_implementation_entries(fixture);
    let append_only_sequence_count = entries
        .iter()
        .filter(|entry| entry.append_only_order_validated)
        .count();
    let checkpoint_anchor_count = entries
        .iter()
        .filter(|entry| entry.checkpoint_anchor_projected)
        .count();
    let replay_digest_count = entries
        .iter()
        .filter(|entry| entry.replay_digest_projected)
        .count();
    let rollback_anchor_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_projected)
        .count();
    let accepted_append_count = entries
        .iter()
        .filter(|entry| entry.append_state == "accepted_in_memory")
        .count();
    let append_only_event_store_test_ready = fixture.test_only_append_fixture_ready
        && fixture.fixture_entry_count == 9
        && !fixture.runtime_feature_gate_enabled
        && !fixture.runtime_event_log_write_allowed
        && !fixture.runtime_sqlite_write_allowed
        && !fixture.fixture_persistence_allowed
        && !fixture.workflow_execution_allowed
        && !fixture.replay_execution_allowed
        && !fixture.rollback_execution_allowed
        && !fixture.live_execution_allowed
        && entries.len() == fixture.fixture_entry_count
        && store.events().len() == entries.len()
        && store.idempotency_key_count() == entries.len()
        && accepted_append_count == entries.len()
        && duplicate_append_denial_count == entries.len()
        && append_only_sequence_count == entries.len()
        && checkpoint_anchor_count == entries.len()
        && replay_digest_count == entries.len()
        && rollback_anchor_count == entries.len()
        && entries.iter().all(|entry| {
            entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_append_only_event_store_test_implementation",
        status: if append_only_event_store_test_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_SCHEMA_VERSION,
        source_fixture_gate: fixture.gate,
        source_fixture_ready: fixture.test_only_append_fixture_ready,
        test_store_scope: "test_only_in_memory_append_only_store_no_runtime_persistence",
        event_contract_count: fixture.event_contract_count,
        test_event_count: entries.len(),
        append_attempt_count: entries.len() * 2,
        accepted_append_count,
        duplicate_append_denial_count,
        append_only_sequence_count,
        idempotency_index_entry_count: store.idempotency_key_count(),
        checkpoint_anchor_count,
        replay_digest_count,
        rollback_anchor_count,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        in_memory_store_materialized: append_only_event_store_test_ready,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        store_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        append_only_event_store_test_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "store_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_RECOMMENDED_NEXT_GATE,
        side_effects: WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_append_only_event_store_test_implementation_entries(
    fixture: &WorkflowDurableStoreTestOnlyAppendFixtureReport,
) -> (
    Vec<WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationEntry>,
    WorkflowTemporalLiteAppendOnlyTestStore,
    usize,
) {
    let mut store = WorkflowTemporalLiteAppendOnlyTestStore::default();
    let mut duplicate_append_denial_count = 0;
    let mut entries = Vec::with_capacity(fixture.entries.len());

    for fixture_entry in &fixture.entries {
        let event = WorkflowTemporalLiteAppendOnlyTestEvent {
            event_contract_id: fixture_entry.event_contract_id,
            record_kind: fixture_entry.record_kind,
            sequence: fixture_entry.fixture_sequence,
            event_id: format!(
                "temporal-lite.test-event.{:03}.{}",
                fixture_entry.fixture_sequence, fixture_entry.event_contract_id
            ),
            aggregate_id: format!(
                "workflow://hepta/test-only/{}",
                fixture_entry.event_contract_id
            ),
            idempotency_key: fixture_entry.fixture_idempotency_key.clone(),
            checkpoint_key: fixture_entry.fixture_checkpoint_key.clone(),
            replay_digest: replay_digest(
                fixture_entry.fixture_sequence,
                fixture_entry.event_contract_id,
                &fixture_entry.fixture_replay_validation_key,
            ),
            rollback_anchor: fixture_entry.fixture_rollback_anchor,
        };
        let accepted = store.append(event.clone());
        let duplicate = store.append(event.clone());
        if duplicate.duplicate_denied {
            duplicate_append_denial_count += 1;
        }

        entries.push(
            WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationEntry {
                event_contract_id: event.event_contract_id,
                record_kind: event.record_kind,
                sequence: event.sequence,
                event_id: event.event_id,
                aggregate_id: event.aggregate_id,
                idempotency_key: event.idempotency_key,
                checkpoint_key: event.checkpoint_key,
                replay_digest: event.replay_digest,
                rollback_anchor: event.rollback_anchor,
                append_state: if accepted.accepted {
                    "accepted_in_memory"
                } else {
                    "blocked"
                },
                duplicate_append_state: if duplicate.duplicate_denied {
                    "duplicate_denied"
                } else {
                    "not_denied"
                },
                appended_to_in_memory_store: accepted.accepted,
                append_only_order_validated: accepted.accepted
                    && store
                        .events()
                        .last()
                        .is_some_and(|stored| stored.sequence == fixture_entry.fixture_sequence),
                idempotency_index_validated: store
                    .idempotency_keys
                    .contains(&fixture_entry.fixture_idempotency_key),
                duplicate_append_denied: duplicate.duplicate_denied,
                checkpoint_anchor_projected: !fixture_entry.fixture_checkpoint_key.is_empty(),
                replay_digest_projected: true,
                rollback_anchor_projected: !fixture_entry.fixture_rollback_anchor.is_empty(),
                feature_gate_required: fixture_entry.feature_gate_required,
                runtime_feature_gate_enabled: fixture_entry.runtime_feature_gate_enabled,
                runtime_event_log_write_allowed: fixture_entry.runtime_event_log_write_allowed,
                runtime_sqlite_write_allowed: fixture_entry.runtime_sqlite_write_allowed,
                store_persistence_allowed: false,
                workflow_execution_allowed: fixture_entry.workflow_execution_allowed,
                replay_execution_allowed: fixture_entry.replay_execution_allowed,
                rollback_execution_allowed: fixture_entry.rollback_execution_allowed,
                live_execution_allowed: fixture_entry.live_execution_allowed,
            },
        );
    }

    (entries, store, duplicate_append_denial_count)
}

fn replay_digest(sequence: usize, event_contract_id: &str, replay_key: &str) -> String {
    format!(
        "replay-digest.v1.{:03}.{}.{}",
        sequence,
        event_contract_id,
        replay_key.len()
    )
}

impl WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_log_written: false,
            sqlite_written: false,
            store_persisted: false,
            lease_acquired: false,
            idempotency_index_persisted: false,
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
    fn append_only_event_store_accepts_fixture_events_in_memory() {
        let report =
            hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_fixture_ready);
        assert_eq!(report.event_contract_count, 9);
        assert_eq!(report.test_event_count, 9);
        assert_eq!(report.append_attempt_count, 18);
        assert_eq!(report.accepted_append_count, 9);
        assert_eq!(report.duplicate_append_denial_count, 9);
        assert_eq!(report.append_only_sequence_count, 9);
        assert_eq!(report.idempotency_index_entry_count, 9);
        assert_eq!(report.checkpoint_anchor_count, 9);
        assert_eq!(report.replay_digest_count, 9);
        assert_eq!(report.rollback_anchor_count, 9);
        assert!(report.in_memory_store_materialized);
        assert!(report.append_only_event_store_test_ready);
    }

    #[test]
    fn append_only_event_store_denies_duplicate_idempotency_keys() {
        let fixture = hepta_workflow_durable_store_test_only_append_fixture_report();
        let (entries, store, duplicate_append_denial_count) =
            workflow_temporal_lite_append_only_event_store_test_implementation_entries(&fixture);

        assert_eq!(entries.len(), 9);
        assert_eq!(store.events().len(), 9);
        assert_eq!(store.idempotency_key_count(), 9);
        assert_eq!(duplicate_append_denial_count, 9);
        assert!(entries.iter().all(|entry| {
            entry.append_state == "accepted_in_memory"
                && entry.duplicate_append_state == "duplicate_denied"
                && entry.appended_to_in_memory_store
                && entry.append_only_order_validated
                && entry.idempotency_index_validated
                && entry.duplicate_append_denied
                && entry.checkpoint_anchor_projected
                && entry.replay_digest_projected
                && entry.rollback_anchor_projected
                && entry.replay_digest.starts_with("replay-digest.v1.")
        }));
    }

    #[test]
    fn append_only_event_store_keeps_runtime_writes_closed() {
        let report =
            hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report();

        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.store_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.store_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }
}
