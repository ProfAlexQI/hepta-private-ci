use serde::Serialize;

use crate::WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackReport;
use crate::hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_GATE: &str =
    "workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "temporal_lite_event_log_sqlite_adapter_feature_gated_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_checkpoint_rollback_gate: &'static str,
    pub source_checkpoint_rollback_ready: bool,
    pub source_anchor_pair_count: usize,
    pub lease_scope: &'static str,
    pub lease_readback_count: usize,
    pub idempotency_index_readback_count: usize,
    pub lease_token_count: usize,
    pub idempotency_key_count: usize,
    pub duplicate_guard_count: usize,
    pub lease_acquired_count: usize,
    pub lease_persisted_count: usize,
    pub idempotency_index_written_count: usize,
    pub idempotency_index_persisted_count: usize,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub lease_idempotency_readback_materialized: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub lease_acquisition_allowed: bool,
    pub lease_persistence_allowed: bool,
    pub idempotency_index_write_allowed: bool,
    pub idempotency_index_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub lease_idempotency_index_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackEntry {
    pub event_contract_id: &'static str,
    pub sequence: usize,
    pub event_id: String,
    pub checkpoint_anchor_key: String,
    pub rollback_anchor_key: String,
    pub checkpoint_readback_digest: String,
    pub rollback_readback_digest: String,
    pub lease_key: String,
    pub lease_token: String,
    pub lease_owner: &'static str,
    pub lease_ttl_ms: u64,
    pub lease_state: &'static str,
    pub idempotency_index_key: String,
    pub idempotency_key: String,
    pub idempotency_index_state: &'static str,
    pub duplicate_guard_key: String,
    pub duplicate_guard_state: &'static str,
    pub readback_state: &'static str,
    pub lease_readback_projected: bool,
    pub lease_token_projected: bool,
    pub idempotency_index_projected: bool,
    pub duplicate_guard_projected: bool,
    pub lease_acquired: bool,
    pub lease_persisted: bool,
    pub idempotency_index_written: bool,
    pub idempotency_index_persisted: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackSideEffects {
    pub filesystem_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub lease_acquired: bool,
    pub lease_persisted: bool,
    pub idempotency_index_written: bool,
    pub idempotency_index_persisted: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub live_execution_started: bool,
}

pub fn hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report()
-> WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackReport {
    let source =
        hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report();
    workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report_from_source(
        &source,
    )
}

pub fn workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report_from_source(
    source: &WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackReport,
) -> WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackReport {
    let entries =
        workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_entries(source);
    let lease_readback_count = entries
        .iter()
        .filter(|entry| entry.lease_readback_projected)
        .count();
    let idempotency_index_readback_count = entries
        .iter()
        .filter(|entry| entry.idempotency_index_projected)
        .count();
    let lease_token_count = entries
        .iter()
        .filter(|entry| !entry.lease_token.is_empty())
        .count();
    let idempotency_key_count = entries
        .iter()
        .filter(|entry| !entry.idempotency_key.is_empty())
        .count();
    let duplicate_guard_count = entries
        .iter()
        .filter(|entry| entry.duplicate_guard_projected)
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
    let lease_idempotency_index_readback_ready = source
        .checkpoint_and_rollback_anchor_readback_ready
        && source.replay_projection_count == 9
        && source.durable_anchor_pair_count == 9
        && source.anchor_mismatch_count == 0
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.checkpoint_write_allowed
        && !source.rollback_anchor_write_allowed
        && !source.anchor_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.replay_projection_count
        && lease_readback_count == entries.len()
        && idempotency_index_readback_count == entries.len()
        && lease_token_count == entries.len()
        && idempotency_key_count == entries.len()
        && duplicate_guard_count == entries.len()
        && lease_acquired_count == 0
        && lease_persisted_count == 0
        && idempotency_index_written_count == 0
        && idempotency_index_persisted_count == 0
        && entries.iter().all(|entry| {
            entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_lease_idempotency_index_feature_gated_readback",
        status: if lease_idempotency_index_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_SCHEMA_VERSION,
        source_checkpoint_rollback_gate: source.gate,
        source_checkpoint_rollback_ready: source.checkpoint_and_rollback_anchor_readback_ready,
        source_anchor_pair_count: source.durable_anchor_pair_count,
        lease_scope: "test_only_lease_and_idempotency_readback_no_acquire_no_persistence",
        lease_readback_count,
        idempotency_index_readback_count,
        lease_token_count,
        idempotency_key_count,
        duplicate_guard_count,
        lease_acquired_count,
        lease_persisted_count,
        idempotency_index_written_count,
        idempotency_index_persisted_count,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        lease_idempotency_readback_materialized: lease_idempotency_index_readback_ready,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        lease_acquisition_allowed: false,
        lease_persistence_allowed: false,
        idempotency_index_write_allowed: false,
        idempotency_index_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        lease_idempotency_index_readback_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "lease_acquisition_disabled",
            "lease_persistence_disabled",
            "idempotency_index_write_disabled",
            "idempotency_index_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_entries(
    source: &WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackReport,
) -> Vec<WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackEntry> {
    source
        .entries
        .iter()
        .map(|source_entry| {
            let lease_key = keyed_readback(
                "temporal-lite.lease.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let lease_token = format!(
                "lease-token.v1.{:03}.{}.{}",
                source_entry.sequence,
                source_entry.event_contract_id,
                source_entry.checkpoint_anchor_key.len()
            );
            let idempotency_index_key = keyed_readback(
                "temporal-lite.idempotency-index.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let idempotency_key = format!(
                "idempotency-key.v1.{:03}.{}.{}",
                source_entry.sequence,
                source_entry.event_contract_id,
                source_entry.event_id.len()
            );
            let duplicate_guard_key = keyed_readback(
                "temporal-lite.duplicate-guard.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );

            WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackEntry {
                event_contract_id: source_entry.event_contract_id,
                sequence: source_entry.sequence,
                event_id: source_entry.event_id.clone(),
                checkpoint_anchor_key: source_entry.checkpoint_anchor_key.clone(),
                rollback_anchor_key: source_entry.rollback_anchor_key.clone(),
                checkpoint_readback_digest: source_entry.checkpoint_readback_digest.clone(),
                rollback_readback_digest: source_entry.rollback_readback_digest.clone(),
                lease_key,
                lease_token,
                lease_owner: "hepta-temporal-lite-test-worker",
                lease_ttl_ms: 30_000,
                lease_state: "projected_not_acquired",
                idempotency_index_key,
                idempotency_key,
                idempotency_index_state: "projected_not_persisted",
                duplicate_guard_key,
                duplicate_guard_state: "projected_duplicate_denial_boundary",
                readback_state: "projected_in_memory_readback_only",
                lease_readback_projected: source_entry.durable_anchor_pair_projected,
                lease_token_projected: true,
                idempotency_index_projected: source_entry.durable_anchor_pair_projected,
                duplicate_guard_projected: source_entry.durable_anchor_pair_projected,
                lease_acquired: false,
                lease_persisted: false,
                idempotency_index_written: false,
                idempotency_index_persisted: false,
                feature_gate_required: source_entry.feature_gate_required,
                runtime_feature_gate_enabled: source_entry.runtime_feature_gate_enabled,
                runtime_event_log_write_allowed: source_entry.runtime_event_log_write_allowed,
                runtime_sqlite_write_allowed: source_entry.runtime_sqlite_write_allowed,
                workflow_execution_allowed: source_entry.workflow_execution_allowed,
                replay_execution_allowed: source_entry.replay_execution_allowed,
                rollback_execution_allowed: source_entry.rollback_execution_allowed,
                live_execution_allowed: source_entry.live_execution_allowed,
            }
        })
        .collect()
}

fn keyed_readback(prefix: &str, sequence: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{sequence:03}.{event_contract_id}")
}

impl WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_log_written: false,
            sqlite_written: false,
            lease_acquired: false,
            lease_persisted: false,
            idempotency_index_written: false,
            idempotency_index_persisted: false,
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
    fn lease_idempotency_readback_projects_all_anchor_pairs_without_acquire() {
        let report =
            hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_checkpoint_rollback_ready);
        assert_eq!(report.source_anchor_pair_count, 9);
        assert_eq!(report.lease_readback_count, 9);
        assert_eq!(report.idempotency_index_readback_count, 9);
        assert_eq!(report.lease_token_count, 9);
        assert_eq!(report.idempotency_key_count, 9);
        assert_eq!(report.duplicate_guard_count, 9);
        assert_eq!(report.lease_acquired_count, 0);
        assert_eq!(report.lease_persisted_count, 0);
        assert_eq!(report.idempotency_index_written_count, 0);
        assert_eq!(report.idempotency_index_persisted_count, 0);
        assert!(report.lease_idempotency_readback_materialized);
        assert!(report.lease_idempotency_index_readback_ready);
    }

    #[test]
    fn lease_idempotency_entries_are_projected_and_write_blocked() {
        let report =
            hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry.lease_key.starts_with("temporal-lite.lease.readback.")
                && entry.lease_token.starts_with("lease-token.v1.")
                && entry.lease_owner == "hepta-temporal-lite-test-worker"
                && entry.lease_ttl_ms == 30_000
                && entry.lease_state == "projected_not_acquired"
                && entry
                    .idempotency_index_key
                    .starts_with("temporal-lite.idempotency-index.readback.")
                && entry.idempotency_key.starts_with("idempotency-key.v1.")
                && entry.idempotency_index_state == "projected_not_persisted"
                && entry
                    .duplicate_guard_key
                    .starts_with("temporal-lite.duplicate-guard.readback.")
                && entry.duplicate_guard_state == "projected_duplicate_denial_boundary"
                && entry.readback_state == "projected_in_memory_readback_only"
                && entry.lease_readback_projected
                && entry.lease_token_projected
                && entry.idempotency_index_projected
                && entry.duplicate_guard_projected
                && !entry.lease_acquired
                && !entry.lease_persisted
                && !entry.idempotency_index_written
                && !entry.idempotency_index_persisted
                && entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn lease_idempotency_side_effects_remain_closed() {
        let report =
            hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report();

        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.lease_acquisition_allowed);
        assert!(!report.lease_persistence_allowed);
        assert!(!report.idempotency_index_write_allowed);
        assert!(!report.idempotency_index_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackSideEffects::none()
        );
    }
}
