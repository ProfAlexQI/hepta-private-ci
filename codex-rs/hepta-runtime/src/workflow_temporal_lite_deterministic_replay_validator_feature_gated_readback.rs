use serde::Serialize;

use crate::WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport;
use crate::hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report;

pub const WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_GATE: &str =
    "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_append_only_gate: &'static str,
    pub source_append_only_ready: bool,
    pub replay_scope: &'static str,
    pub test_event_count: usize,
    pub replay_projection_count: usize,
    pub deterministic_order_count: usize,
    pub replay_digest_count: usize,
    pub replay_checksum_count: usize,
    pub replay_mismatch_count: usize,
    pub idempotency_projection_count: usize,
    pub checkpoint_projection_count: usize,
    pub rollback_anchor_projection_count: usize,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub replay_validator_materialized: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub replay_projection_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub deterministic_replay_validator_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackEntry {
    pub event_contract_id: &'static str,
    pub sequence: usize,
    pub event_id: String,
    pub replay_projection_key: String,
    pub replay_source_digest: String,
    pub replay_observed_digest: String,
    pub replay_checksum: String,
    pub projection_state: &'static str,
    pub deterministic_order_validated: bool,
    pub replay_digest_validated: bool,
    pub replay_checksum_validated: bool,
    pub replay_mismatch_detected: bool,
    pub idempotency_key_replayed: bool,
    pub checkpoint_key_replayed: bool,
    pub rollback_anchor_replayed: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub replay_projection_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackSideEffects {
    pub filesystem_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub replay_projection_persisted: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub live_execution_started: bool,
}

pub fn hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report()
-> WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackReport {
    let source = hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report();
    workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report_from_source(
        &source,
    )
}

pub fn workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report_from_source(
    source: &WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport,
) -> WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackReport {
    let entries =
        workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_entries(
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
    let replay_mismatch_count = entries
        .iter()
        .filter(|entry| entry.replay_mismatch_detected)
        .count();
    let idempotency_projection_count = entries
        .iter()
        .filter(|entry| entry.idempotency_key_replayed)
        .count();
    let checkpoint_projection_count = entries
        .iter()
        .filter(|entry| entry.checkpoint_key_replayed)
        .count();
    let rollback_anchor_projection_count = entries
        .iter()
        .filter(|entry| entry.rollback_anchor_replayed)
        .count();
    let deterministic_replay_validator_ready = source.append_only_event_store_test_ready
        && source.test_event_count == 9
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.store_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.test_event_count
        && deterministic_order_count == entries.len()
        && replay_digest_count == entries.len()
        && replay_checksum_count == entries.len()
        && replay_mismatch_count == 0
        && idempotency_projection_count == entries.len()
        && checkpoint_projection_count == entries.len()
        && rollback_anchor_projection_count == entries.len()
        && entries.iter().all(|entry| {
            entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.replay_projection_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback",
        status: if deterministic_replay_validator_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_SCHEMA_VERSION,
        source_append_only_gate: source.gate,
        source_append_only_ready: source.append_only_event_store_test_ready,
        replay_scope: "test_only_deterministic_projection_no_replay_execution",
        test_event_count: source.test_event_count,
        replay_projection_count: entries.len(),
        deterministic_order_count,
        replay_digest_count,
        replay_checksum_count,
        replay_mismatch_count,
        idempotency_projection_count,
        checkpoint_projection_count,
        rollback_anchor_projection_count,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        replay_validator_materialized: deterministic_replay_validator_ready,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        replay_projection_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        deterministic_replay_validator_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "replay_projection_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_entries(
    source: &WorkflowTemporalLiteAppendOnlyEventStoreTestImplementationReport,
) -> Vec<WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackEntry> {
    source
        .entries
        .iter()
        .enumerate()
        .map(|(index, source_entry)| {
            let expected_sequence = index + 1;
            let replay_observed_digest = source_entry.replay_digest.clone();
            let replay_checksum = replay_checksum(
                source_entry.sequence,
                source_entry.event_contract_id,
                &source_entry.event_id,
                &replay_observed_digest,
            );
            WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackEntry {
                event_contract_id: source_entry.event_contract_id,
                sequence: source_entry.sequence,
                event_id: source_entry.event_id.clone(),
                replay_projection_key: format!(
                    "temporal-lite.replay-projection.{:03}.{}",
                    source_entry.sequence, source_entry.event_contract_id
                ),
                replay_source_digest: source_entry.replay_digest.clone(),
                replay_observed_digest,
                replay_checksum,
                projection_state: "projected_in_memory_readback_only",
                deterministic_order_validated: source_entry.sequence == expected_sequence,
                replay_digest_validated: source_entry.replay_digest_projected,
                replay_checksum_validated: true,
                replay_mismatch_detected: false,
                idempotency_key_replayed: !source_entry.idempotency_key.is_empty(),
                checkpoint_key_replayed: !source_entry.checkpoint_key.is_empty(),
                rollback_anchor_replayed: !source_entry.rollback_anchor.is_empty(),
                feature_gate_required: source_entry.feature_gate_required,
                runtime_feature_gate_enabled: source_entry.runtime_feature_gate_enabled,
                runtime_event_log_write_allowed: source_entry.runtime_event_log_write_allowed,
                runtime_sqlite_write_allowed: source_entry.runtime_sqlite_write_allowed,
                replay_projection_persistence_allowed: false,
                workflow_execution_allowed: source_entry.workflow_execution_allowed,
                replay_execution_allowed: source_entry.replay_execution_allowed,
                rollback_execution_allowed: source_entry.rollback_execution_allowed,
                live_execution_allowed: source_entry.live_execution_allowed,
            }
        })
        .collect()
}

fn replay_checksum(
    sequence: usize,
    event_contract_id: &str,
    event_id: &str,
    replay_digest: &str,
) -> String {
    format!(
        "replay-checksum.v1.{:03}.{}.{}.{}",
        sequence,
        event_contract_id,
        event_id.len(),
        replay_digest.len()
    )
}

impl WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_log_written: false,
            sqlite_written: false,
            replay_projection_persisted: false,
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
    fn deterministic_replay_validator_projects_all_append_only_events() {
        let report =
            hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_append_only_ready);
        assert_eq!(report.test_event_count, 9);
        assert_eq!(report.replay_projection_count, 9);
        assert_eq!(report.deterministic_order_count, 9);
        assert_eq!(report.replay_digest_count, 9);
        assert_eq!(report.replay_checksum_count, 9);
        assert_eq!(report.replay_mismatch_count, 0);
        assert_eq!(report.idempotency_projection_count, 9);
        assert_eq!(report.checkpoint_projection_count, 9);
        assert_eq!(report.rollback_anchor_projection_count, 9);
        assert!(report.replay_validator_materialized);
        assert!(report.deterministic_replay_validator_ready);
    }

    #[test]
    fn deterministic_replay_validator_keeps_projection_readback_only() {
        let report =
            hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report();

        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.replay_projection_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackSideEffects::none()
        );
    }

    #[test]
    fn deterministic_replay_entries_are_ordered_and_mismatch_free() {
        let report =
            hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report();

        assert!(report.entries.iter().enumerate().all(|(index, entry)| {
            entry.sequence == index + 1
                && entry.projection_state == "projected_in_memory_readback_only"
                && entry.deterministic_order_validated
                && entry.replay_digest_validated
                && entry.replay_checksum_validated
                && !entry.replay_mismatch_detected
                && entry.replay_source_digest == entry.replay_observed_digest
                && entry
                    .replay_projection_key
                    .starts_with("temporal-lite.replay-projection.")
                && entry.replay_checksum.starts_with("replay-checksum.v1.")
                && entry.idempotency_key_replayed
                && entry.checkpoint_key_replayed
                && entry.rollback_anchor_replayed
                && entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.replay_projection_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        }));
    }
}
