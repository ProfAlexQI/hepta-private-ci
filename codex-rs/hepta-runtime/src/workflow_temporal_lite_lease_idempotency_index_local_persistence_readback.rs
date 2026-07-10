use serde::Serialize;

use crate::WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackEntry;
use crate::WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackReport;
use crate::hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_GATE: &str =
    "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_checkpoint_rollback_gate: &'static str,
    pub source_checkpoint_rollback_ready: bool,
    pub source_anchor_pair_count: usize,
    pub source_append_only_event_store_interface_ready: bool,
    pub source_checkpoint_anchors_derived_from_event_store_interface: bool,
    pub lease_scope: &'static str,
    pub sqlite_readback_scope: &'static str,
    pub lease_readback_count: usize,
    pub idempotency_index_readback_count: usize,
    pub duplicate_guard_readback_count: usize,
    pub lease_digest_count: usize,
    pub idempotency_digest_count: usize,
    pub lease_idempotency_pair_count: usize,
    pub lease_acquired_count: usize,
    pub lease_persisted_count: usize,
    pub idempotency_index_written_count: usize,
    pub idempotency_index_persisted_count: usize,
    pub lease_idempotency_mismatch_count: usize,
    pub wal_mode_required: bool,
    pub local_tempdb_sqlite_read_covered_by_tests: bool,
    pub runtime_feature_gate_enabled: bool,
    pub lease_idempotency_readback_materialized: bool,
    pub lease_idempotency_derived_from_event_store_interface: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub lease_acquire_allowed: bool,
    pub lease_persistence_allowed: bool,
    pub idempotency_index_write_allowed: bool,
    pub idempotency_index_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub lease_idempotency_index_local_persistence_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackEntry {
    pub event_contract_id: String,
    pub replay_order: usize,
    pub local_sequence: i64,
    pub source_event_id: String,
    pub replay_projection_key: String,
    pub checkpoint_anchor_key: String,
    pub rollback_anchor_key: String,
    pub lease_readback_key: String,
    pub lease_scope_key: String,
    pub lease_owner: &'static str,
    pub lease_ttl_ms: u64,
    pub lease_digest: String,
    pub lease_state: &'static str,
    pub idempotency_index_readback_key: String,
    pub idempotency_key: String,
    pub idempotency_digest: String,
    pub duplicate_guard_key: String,
    pub duplicate_guard_state: &'static str,
    pub readback_state: &'static str,
    pub lease_readback_projected: bool,
    pub lease_digest_validated: bool,
    pub idempotency_index_projected: bool,
    pub idempotency_digest_validated: bool,
    pub duplicate_guard_projected: bool,
    pub lease_idempotency_pair_projected: bool,
    pub lease_idempotency_mismatch_detected: bool,
    pub sqlite_readback_validated: bool,
    pub wal_mode_required: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub runtime_store_persistence_allowed: bool,
    pub lease_acquired: bool,
    pub lease_persisted: bool,
    pub idempotency_index_written: bool,
    pub idempotency_index_persisted: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackSideEffects {
    pub report_written: bool,
    pub runtime_filesystem_written: bool,
    pub runtime_event_log_written: bool,
    pub runtime_sqlite_written: bool,
    pub runtime_store_persisted: bool,
    pub lease_acquired: bool,
    pub lease_persisted: bool,
    pub idempotency_index_written: bool,
    pub idempotency_index_persisted: bool,
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

pub fn hepta_workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report()
-> WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackReport {
    let source =
        hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_report();
    workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report_from_checkpoint_rollback(&source)
}

pub fn workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report_from_checkpoint_rollback(
    source: &WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackReport,
) -> WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackReport {
    let entries =
        workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_entries(source);
    let lease_readback_count = entries
        .iter()
        .filter(|entry| entry.lease_readback_projected)
        .count();
    let idempotency_index_readback_count = entries
        .iter()
        .filter(|entry| entry.idempotency_index_projected)
        .count();
    let duplicate_guard_readback_count = entries
        .iter()
        .filter(|entry| entry.duplicate_guard_projected)
        .count();
    let lease_digest_count = entries
        .iter()
        .filter(|entry| entry.lease_digest_validated)
        .count();
    let idempotency_digest_count = entries
        .iter()
        .filter(|entry| entry.idempotency_digest_validated)
        .count();
    let lease_idempotency_pair_count = entries
        .iter()
        .filter(|entry| entry.lease_idempotency_pair_projected)
        .count();
    let lease_acquired_count = entries.iter().filter(|entry| entry.lease_acquired).count();
    let lease_persisted_count = entries.iter().filter(|entry| entry.lease_persisted).count();
    let idempotency_index_written_count = entries
        .iter()
        .filter(|entry| entry.idempotency_index_written)
        .count();
    let idempotency_index_persisted_count = entries
        .iter()
        .filter(|entry| entry.idempotency_index_persisted)
        .count();
    let lease_idempotency_mismatch_count = entries
        .iter()
        .filter(|entry| entry.lease_idempotency_mismatch_detected)
        .count();
    let lease_idempotency_derived_from_event_store_interface = source
        .source_append_only_event_store_interface_ready
        && source.checkpoint_anchors_derived_from_event_store_interface;
    let ready = source.checkpoint_and_rollback_anchor_local_persistence_readback_ready
        && lease_idempotency_derived_from_event_store_interface
        && source.durable_anchor_pair_count == 9
        && source.anchor_mismatch_count == 0
        && source.local_tempdb_sqlite_read_covered_by_tests
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.runtime_store_persistence_allowed
        && !source.checkpoint_write_allowed
        && !source.rollback_anchor_write_allowed
        && !source.anchor_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.durable_anchor_pair_count
        && lease_readback_count == entries.len()
        && idempotency_index_readback_count == entries.len()
        && duplicate_guard_readback_count == entries.len()
        && lease_digest_count == entries.len()
        && idempotency_digest_count == entries.len()
        && lease_idempotency_pair_count == entries.len()
        && lease_acquired_count == 0
        && lease_persisted_count == 0
        && idempotency_index_written_count == 0
        && idempotency_index_persisted_count == 0
        && lease_idempotency_mismatch_count == 0
        && entries.iter().all(|entry| {
            entry.sqlite_readback_validated
                && entry.wal_mode_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.lease_acquired
                && !entry.lease_persisted
                && !entry.idempotency_index_written
                && !entry.idempotency_index_persisted
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback",
        status: if ready { "ready_blocked" } else { "blocked" },
        gate: WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_SCHEMA_VERSION,
        source_checkpoint_rollback_gate: source.gate,
        source_checkpoint_rollback_ready: source
            .checkpoint_and_rollback_anchor_local_persistence_readback_ready,
        source_anchor_pair_count: source.durable_anchor_pair_count,
        source_append_only_event_store_interface_ready: source
            .source_append_only_event_store_interface_ready,
        source_checkpoint_anchors_derived_from_event_store_interface: source
            .checkpoint_anchors_derived_from_event_store_interface,
        lease_scope: "local_persistence_lease_idempotency_readback_no_acquire_no_persistence",
        sqlite_readback_scope: source.sqlite_readback_scope,
        lease_readback_count,
        idempotency_index_readback_count,
        duplicate_guard_readback_count,
        lease_digest_count,
        idempotency_digest_count,
        lease_idempotency_pair_count,
        lease_acquired_count,
        lease_persisted_count,
        idempotency_index_written_count,
        idempotency_index_persisted_count,
        lease_idempotency_mismatch_count,
        wal_mode_required: true,
        local_tempdb_sqlite_read_covered_by_tests: true,
        runtime_feature_gate_enabled: false,
        lease_idempotency_readback_materialized: ready,
        lease_idempotency_derived_from_event_store_interface,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        runtime_store_persistence_allowed: false,
        lease_acquire_allowed: false,
        lease_persistence_allowed: false,
        idempotency_index_write_allowed: false,
        idempotency_index_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        lease_idempotency_index_local_persistence_readback_ready: ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "runtime_store_persistence_disabled",
            "lease_acquire_disabled",
            "lease_persistence_disabled",
            "idempotency_index_write_disabled",
            "idempotency_index_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_entries(
    source: &WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackReport,
) -> Vec<WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackEntry> {
    workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_entries_from_anchor_entries(&source.entries)
}

pub fn workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_entries_from_anchor_entries(
    anchor_entries: &[WorkflowTemporalLiteCheckpointAndRollbackAnchorLocalPersistenceReadbackEntry],
) -> Vec<WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackEntry> {
    anchor_entries
        .iter()
        .map(|source_entry| {
            let lease_readback_key = keyed_readback(
                "temporal-lite.local-lease-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let lease_scope_key = keyed_readback(
                "temporal-lite.local-lease-scope",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let lease_digest = lease_idempotency_digest(
                "lease",
                source_entry.replay_order,
                &source_entry.event_contract_id,
                &source_entry.checkpoint_readback_digest,
                &source_entry.rollback_readback_digest,
            );
            let idempotency_index_readback_key = keyed_readback(
                "temporal-lite.local-idempotency-index-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let idempotency_key = format!(
                "idempotency-key.local.v1.{:03}.{}.{}.{}",
                source_entry.replay_order,
                source_entry.event_contract_id,
                source_entry.source_event_id.len(),
                source_entry.replay_projection_key.len()
            );
            let idempotency_digest = lease_idempotency_digest(
                "idempotency",
                source_entry.replay_order,
                &source_entry.event_contract_id,
                &idempotency_key,
                &source_entry.replay_batch_digest,
            );
            let duplicate_guard_key = keyed_readback(
                "temporal-lite.local-duplicate-guard-readback",
                source_entry.replay_order,
                &source_entry.event_contract_id,
            );
            let lease_readback_projected = source_entry.durable_anchor_pair_projected;
            let idempotency_index_projected = source_entry.durable_anchor_pair_projected;
            let lease_idempotency_pair_projected =
                lease_readback_projected && idempotency_index_projected;

            WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackEntry {
                event_contract_id: source_entry.event_contract_id.clone(),
                replay_order: source_entry.replay_order,
                local_sequence: source_entry.local_sequence,
                source_event_id: source_entry.source_event_id.clone(),
                replay_projection_key: source_entry.replay_projection_key.clone(),
                checkpoint_anchor_key: source_entry.checkpoint_anchor_key.clone(),
                rollback_anchor_key: source_entry.rollback_anchor_key.clone(),
                lease_readback_key,
                lease_scope_key,
                lease_owner: "hepta-temporal-lite-local-test-worker",
                lease_ttl_ms: 30_000,
                lease_digest,
                lease_state: "projected_from_local_persistence_not_acquired",
                idempotency_index_readback_key,
                idempotency_key,
                idempotency_digest,
                duplicate_guard_key,
                duplicate_guard_state: "projected_duplicate_denial_boundary",
                readback_state:
                    "projected_from_sqlite_wal_local_persistence_readback_without_runtime_writes",
                lease_readback_projected,
                lease_digest_validated: source_entry.checkpoint_digest_validated
                    && source_entry.rollback_digest_validated,
                idempotency_index_projected,
                idempotency_digest_validated: source_entry.checkpoint_digest_validated
                    && source_entry.rollback_digest_validated,
                duplicate_guard_projected: source_entry.durable_anchor_pair_projected,
                lease_idempotency_pair_projected,
                lease_idempotency_mismatch_detected: source_entry.anchor_mismatch_detected,
                sqlite_readback_validated: source_entry.sqlite_readback_validated,
                wal_mode_required: source_entry.wal_mode_required,
                feature_gate_required: source_entry.feature_gate_required,
                runtime_feature_gate_enabled: source_entry.runtime_feature_gate_enabled,
                runtime_event_log_write_allowed: source_entry.runtime_event_log_write_allowed,
                runtime_sqlite_write_allowed: source_entry.runtime_sqlite_write_allowed,
                runtime_store_persistence_allowed: source_entry.runtime_store_persistence_allowed,
                lease_acquired: false,
                lease_persisted: false,
                idempotency_index_written: false,
                idempotency_index_persisted: false,
                workflow_execution_allowed: source_entry.workflow_execution_allowed,
                replay_execution_allowed: source_entry.replay_execution_allowed,
                rollback_execution_allowed: source_entry.rollback_execution_allowed,
                live_execution_allowed: source_entry.live_execution_allowed,
            }
        })
        .collect()
}

fn keyed_readback(prefix: &str, replay_order: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{replay_order:03}.{event_contract_id}")
}

fn lease_idempotency_digest(
    digest_kind: &str,
    replay_order: usize,
    event_contract_id: &str,
    left: &str,
    right: &str,
) -> String {
    format!(
        "temporal-lite.local-{digest_kind}-digest.v1.{replay_order:03}.{event_contract_id}.{}.{}",
        left.len(),
        right.len()
    )
}

impl WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            runtime_filesystem_written: false,
            runtime_event_log_written: false,
            runtime_sqlite_written: false,
            runtime_store_persisted: false,
            lease_acquired: false,
            lease_persisted: false,
            idempotency_index_written: false,
            idempotency_index_persisted: false,
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
    use crate::workflow_temporal_lite_minimal_local_events_from_test_implementation;

    #[test]
    fn local_lease_idempotency_readback_projects_all_anchor_pairs_without_acquire() {
        let report =
            hepta_workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report(
            );

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_checkpoint_rollback_ready);
        assert_eq!(report.source_anchor_pair_count, 9);
        assert!(report.source_append_only_event_store_interface_ready);
        assert!(report.source_checkpoint_anchors_derived_from_event_store_interface);
        assert_eq!(report.lease_readback_count, 9);
        assert_eq!(report.idempotency_index_readback_count, 9);
        assert_eq!(report.duplicate_guard_readback_count, 9);
        assert_eq!(report.lease_digest_count, 9);
        assert_eq!(report.idempotency_digest_count, 9);
        assert_eq!(report.lease_idempotency_pair_count, 9);
        assert_eq!(report.lease_acquired_count, 0);
        assert_eq!(report.lease_persisted_count, 0);
        assert_eq!(report.idempotency_index_written_count, 0);
        assert_eq!(report.idempotency_index_persisted_count, 0);
        assert_eq!(report.lease_idempotency_mismatch_count, 0);
        assert!(report.lease_idempotency_readback_materialized);
        assert!(report.lease_idempotency_derived_from_event_store_interface);
        assert!(report.lease_idempotency_index_local_persistence_readback_ready);
    }

    #[tokio::test]
    async fn local_lease_idempotency_readback_uses_reopened_sqlite_anchor_history() {
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

        assert_eq!(stored_events.len(), 9);
        assert_eq!(replay_entries.len(), 9);
        assert_eq!(anchor_entries.len(), 9);
        assert_eq!(lease_entries.len(), 9);
        assert!(lease_entries.iter().enumerate().all(|(index, entry)| {
            entry.replay_order == index + 1
                && entry.local_sequence >= 1
                && entry
                    .lease_readback_key
                    .starts_with("temporal-lite.local-lease-readback.")
                && entry
                    .lease_scope_key
                    .starts_with("temporal-lite.local-lease-scope.")
                && entry.lease_owner == "hepta-temporal-lite-local-test-worker"
                && entry.lease_ttl_ms == 30_000
                && entry
                    .lease_digest
                    .starts_with("temporal-lite.local-lease-digest.v1.")
                && entry.lease_state == "projected_from_local_persistence_not_acquired"
                && entry
                    .idempotency_index_readback_key
                    .starts_with("temporal-lite.local-idempotency-index-readback.")
                && entry
                    .idempotency_key
                    .starts_with("idempotency-key.local.v1.")
                && entry
                    .idempotency_digest
                    .starts_with("temporal-lite.local-idempotency-digest.v1.")
                && entry
                    .duplicate_guard_key
                    .starts_with("temporal-lite.local-duplicate-guard-readback.")
                && entry.duplicate_guard_state == "projected_duplicate_denial_boundary"
                && entry.readback_state
                    == "projected_from_sqlite_wal_local_persistence_readback_without_runtime_writes"
                && entry.lease_readback_projected
                && entry.lease_digest_validated
                && entry.idempotency_index_projected
                && entry.idempotency_digest_validated
                && entry.duplicate_guard_projected
                && entry.lease_idempotency_pair_projected
                && !entry.lease_idempotency_mismatch_detected
                && entry.sqlite_readback_validated
                && entry.wal_mode_required
                && !entry.lease_acquired
                && !entry.lease_persisted
                && !entry.idempotency_index_written
                && !entry.idempotency_index_persisted
                && !entry.workflow_execution_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn local_lease_idempotency_readback_keeps_writes_and_live_closed() {
        let report =
            hepta_workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report(
            );

        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.runtime_store_persistence_allowed);
        assert!(!report.lease_acquire_allowed);
        assert!(!report.lease_persistence_allowed);
        assert!(!report.idempotency_index_write_allowed);
        assert!(!report.idempotency_index_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteLeaseIdempotencyIndexLocalPersistenceReadbackSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.feature_gate_required
                && entry.sqlite_readback_validated
                && entry.wal_mode_required
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.runtime_store_persistence_allowed
                && !entry.lease_acquired
                && !entry.lease_persisted
                && !entry.idempotency_index_written
                && !entry.idempotency_index_persisted
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }
}
