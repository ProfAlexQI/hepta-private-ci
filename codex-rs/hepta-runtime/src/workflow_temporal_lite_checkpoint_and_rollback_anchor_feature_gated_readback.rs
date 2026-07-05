use serde::Serialize;

use crate::WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackReport;
use crate::hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_GATE: &str =
    "workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "current_reality_matrix_compact_cache_boundary_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_replay_validator_gate: &'static str,
    pub source_replay_validator_ready: bool,
    pub anchor_scope: &'static str,
    pub replay_projection_count: usize,
    pub checkpoint_anchor_readback_count: usize,
    pub rollback_anchor_readback_count: usize,
    pub durable_anchor_pair_count: usize,
    pub checkpoint_digest_count: usize,
    pub rollback_digest_count: usize,
    pub anchor_mismatch_count: usize,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub anchor_readback_materialized: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub checkpoint_write_allowed: bool,
    pub rollback_anchor_write_allowed: bool,
    pub anchor_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub checkpoint_and_rollback_anchor_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackEntry {
    pub event_contract_id: &'static str,
    pub sequence: usize,
    pub event_id: String,
    pub replay_projection_key: String,
    pub checkpoint_anchor_key: String,
    pub rollback_anchor_key: String,
    pub checkpoint_readback_digest: String,
    pub rollback_readback_digest: String,
    pub anchor_pair_state: &'static str,
    pub checkpoint_anchor_projected: bool,
    pub rollback_anchor_projected: bool,
    pub durable_anchor_pair_projected: bool,
    pub checkpoint_digest_validated: bool,
    pub rollback_digest_validated: bool,
    pub anchor_mismatch_detected: bool,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub checkpoint_write_allowed: bool,
    pub rollback_anchor_write_allowed: bool,
    pub anchor_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackSideEffects {
    pub filesystem_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub checkpoint_written: bool,
    pub rollback_anchor_written: bool,
    pub anchor_persisted: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub live_execution_started: bool,
}

pub fn hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report()
-> WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackReport {
    let source =
        hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report();
    workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report_from_source(
        &source,
    )
}

pub fn workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report_from_source(
    source: &WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackReport,
) -> WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackReport {
    let entries =
        workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_entries(
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
    let checkpoint_and_rollback_anchor_readback_ready = source.deterministic_replay_validator_ready
        && source.replay_projection_count == 9
        && source.replay_mismatch_count == 0
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.replay_projection_persistence_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.replay_projection_count
        && checkpoint_anchor_readback_count == entries.len()
        && rollback_anchor_readback_count == entries.len()
        && durable_anchor_pair_count == entries.len()
        && checkpoint_digest_count == entries.len()
        && rollback_digest_count == entries.len()
        && anchor_mismatch_count == 0
        && entries.iter().all(|entry| {
            entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
                && !entry.checkpoint_write_allowed
                && !entry.rollback_anchor_write_allowed
                && !entry.anchor_persistence_allowed
                && !entry.workflow_execution_allowed
                && !entry.replay_execution_allowed
                && !entry.rollback_execution_allowed
                && !entry.live_execution_allowed
        });

    WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback",
        status: if checkpoint_and_rollback_anchor_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_SCHEMA_VERSION,
        source_replay_validator_gate: source.gate,
        source_replay_validator_ready: source.deterministic_replay_validator_ready,
        anchor_scope: "test_only_checkpoint_and_rollback_anchor_readback_no_writes",
        replay_projection_count: source.replay_projection_count,
        checkpoint_anchor_readback_count,
        rollback_anchor_readback_count,
        durable_anchor_pair_count,
        checkpoint_digest_count,
        rollback_digest_count,
        anchor_mismatch_count,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        anchor_readback_materialized: checkpoint_and_rollback_anchor_readback_ready,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        checkpoint_write_allowed: false,
        rollback_anchor_write_allowed: false,
        anchor_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        checkpoint_and_rollback_anchor_readback_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "checkpoint_write_disabled",
            "rollback_anchor_write_disabled",
            "anchor_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_entries(
    source: &WorkflowTemporalLiteDeterministicReplayValidatorFeatureGatedReadbackReport,
) -> Vec<WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackEntry> {
    source
        .entries
        .iter()
        .map(|source_entry| {
            let checkpoint_anchor_key = format!(
                "temporal-lite.checkpoint-anchor.{:03}.{}",
                source_entry.sequence, source_entry.event_contract_id
            );
            let rollback_anchor_key = format!(
                "temporal-lite.rollback-anchor.{:03}.{}",
                source_entry.sequence, source_entry.event_contract_id
            );
            let checkpoint_readback_digest = anchor_digest(
                "checkpoint",
                source_entry.sequence,
                source_entry.event_contract_id,
                &source_entry.replay_checksum,
            );
            let rollback_readback_digest = anchor_digest(
                "rollback",
                source_entry.sequence,
                source_entry.event_contract_id,
                &source_entry.replay_checksum,
            );

            WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackEntry {
                event_contract_id: source_entry.event_contract_id,
                sequence: source_entry.sequence,
                event_id: source_entry.event_id.clone(),
                replay_projection_key: source_entry.replay_projection_key.clone(),
                checkpoint_anchor_key,
                rollback_anchor_key,
                checkpoint_readback_digest,
                rollback_readback_digest,
                anchor_pair_state: "projected_in_memory_readback_only",
                checkpoint_anchor_projected: source_entry.checkpoint_key_replayed,
                rollback_anchor_projected: source_entry.rollback_anchor_replayed,
                durable_anchor_pair_projected: source_entry.checkpoint_key_replayed
                    && source_entry.rollback_anchor_replayed,
                checkpoint_digest_validated: source_entry.replay_checksum_validated,
                rollback_digest_validated: source_entry.replay_checksum_validated,
                anchor_mismatch_detected: source_entry.replay_mismatch_detected,
                feature_gate_required: source_entry.feature_gate_required,
                runtime_feature_gate_enabled: source_entry.runtime_feature_gate_enabled,
                runtime_event_log_write_allowed: source_entry.runtime_event_log_write_allowed,
                runtime_sqlite_write_allowed: source_entry.runtime_sqlite_write_allowed,
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

fn anchor_digest(
    anchor_kind: &str,
    sequence: usize,
    event_contract_id: &str,
    replay_checksum: &str,
) -> String {
    format!(
        "temporal-lite.{anchor_kind}-anchor-digest.v1.{sequence:03}.{event_contract_id}.{}",
        replay_checksum.len()
    )
}

impl WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_log_written: false,
            sqlite_written: false,
            checkpoint_written: false,
            rollback_anchor_written: false,
            anchor_persisted: false,
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
    fn checkpoint_and_rollback_anchor_readback_projects_all_replay_entries() {
        let report =
            hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_replay_validator_ready);
        assert_eq!(report.replay_projection_count, 9);
        assert_eq!(report.checkpoint_anchor_readback_count, 9);
        assert_eq!(report.rollback_anchor_readback_count, 9);
        assert_eq!(report.durable_anchor_pair_count, 9);
        assert_eq!(report.checkpoint_digest_count, 9);
        assert_eq!(report.rollback_digest_count, 9);
        assert_eq!(report.anchor_mismatch_count, 0);
        assert!(report.anchor_readback_materialized);
        assert!(report.checkpoint_and_rollback_anchor_readback_ready);
    }

    #[test]
    fn checkpoint_and_rollback_anchor_readback_keeps_writes_closed() {
        let report =
            hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report();

        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.checkpoint_write_allowed);
        assert!(!report.rollback_anchor_write_allowed);
        assert!(!report.anchor_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteCheckpointAndRollbackAnchorFeatureGatedReadbackSideEffects::none()
        );
    }

    #[test]
    fn checkpoint_and_rollback_anchor_entries_are_paired_and_mismatch_free() {
        let report =
            hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry.anchor_pair_state == "projected_in_memory_readback_only"
                && entry
                    .checkpoint_anchor_key
                    .starts_with("temporal-lite.checkpoint-anchor.")
                && entry
                    .rollback_anchor_key
                    .starts_with("temporal-lite.rollback-anchor.")
                && entry
                    .checkpoint_readback_digest
                    .starts_with("temporal-lite.checkpoint-anchor-digest.v1.")
                && entry
                    .rollback_readback_digest
                    .starts_with("temporal-lite.rollback-anchor-digest.v1.")
                && entry.checkpoint_anchor_projected
                && entry.rollback_anchor_projected
                && entry.durable_anchor_pair_projected
                && entry.checkpoint_digest_validated
                && entry.rollback_digest_validated
                && !entry.anchor_mismatch_detected
                && entry.feature_gate_required
                && !entry.runtime_feature_gate_enabled
                && !entry.runtime_event_log_write_allowed
                && !entry.runtime_sqlite_write_allowed
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
