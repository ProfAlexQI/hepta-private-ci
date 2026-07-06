use serde::Serialize;

use crate::WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackReport;
use crate::hepta_workflow_temporal_lite_work_graph_projection_feature_gated_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_FEATURE_GATED_READBACK_GATE:
    &str = "workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_FEATURE_GATED_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_projection_gate: &'static str,
    pub source_projection_ready: bool,
    pub source_projection_entry_count: usize,
    pub alignment_scope: &'static str,
    pub replay_alignment_projection_count: usize,
    pub projection_replay_key_count: usize,
    pub replay_alignment_checksum_count: usize,
    pub deterministic_alignment_count: usize,
    pub replay_executed_count: usize,
    pub projection_alignment_persisted_count: usize,
    pub work_graph_store_write_count: usize,
    pub event_log_write_count: usize,
    pub sqlite_write_count: usize,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub replay_alignment_contract_readback_materialized: bool,
    pub replay_execution_allowed: bool,
    pub projection_alignment_persistence_allowed: bool,
    pub work_graph_projection_write_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub replay_alignment_readback_ready: bool,
    pub entries:
        Vec<WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects:
        WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackEntry {
    pub event_contract_id: &'static str,
    pub sequence: usize,
    pub event_id: String,
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
    pub replay_executed: bool,
    pub projection_alignment_persisted: bool,
    pub work_graph_store_written: bool,
    pub event_log_record_written: bool,
    pub sqlite_row_written: bool,
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
pub struct WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackSideEffects {
    pub filesystem_written: bool,
    pub replay_executed: bool,
    pub projection_alignment_written: bool,
    pub projection_alignment_persisted: bool,
    pub work_graph_projection_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
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

pub fn hepta_workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_report()
-> WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackReport {
    let source = hepta_workflow_temporal_lite_work_graph_projection_feature_gated_readback_report();
    workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_report_from_source(&source)
}

pub fn workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_report_from_source(
    source: &WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackReport,
) -> WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackReport {
    let entries =
        workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_entries(source);
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
    let replay_alignment_readback_ready = source.work_graph_projection_readback_ready
        && source.source_adapter_entry_count == 9
        && source.work_graph_node_projection_count == 9
        && source.work_graph_event_edge_projection_count == 9
        && source.work_graph_state_edge_projection_count == 9
        && source.projection_key_count == 9
        && source.projection_checksum_count == 9
        && source.projection_persisted_count == 0
        && source.work_graph_store_write_count == 0
        && source.event_log_write_count == 0
        && source.sqlite_write_count == 0
        && !source.runtime_feature_gate_enabled
        && !source.work_graph_projection_write_allowed
        && !source.work_graph_projection_persistence_allowed
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.source_adapter_entry_count
        && replay_alignment_projection_count == entries.len()
        && projection_replay_key_count == entries.len()
        && replay_alignment_checksum_count == entries.len()
        && deterministic_alignment_count == entries.len()
        && replay_executed_count == 0
        && projection_alignment_persisted_count == 0
        && work_graph_store_write_count == 0
        && event_log_write_count == 0
        && sqlite_write_count == 0
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

    WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback",
        status: if replay_alignment_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_FEATURE_GATED_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_FEATURE_GATED_READBACK_SCHEMA_VERSION,
        source_projection_gate: source.gate,
        source_projection_ready: source.work_graph_projection_readback_ready,
        source_projection_entry_count: source.source_adapter_entry_count,
        alignment_scope: "test_only_work_graph_projection_replay_alignment_readback_no_execution",
        replay_alignment_projection_count,
        projection_replay_key_count,
        replay_alignment_checksum_count,
        deterministic_alignment_count,
        replay_executed_count,
        projection_alignment_persisted_count,
        work_graph_store_write_count,
        event_log_write_count,
        sqlite_write_count,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        replay_alignment_contract_readback_materialized: replay_alignment_readback_ready,
        replay_execution_allowed: false,
        projection_alignment_persistence_allowed: false,
        work_graph_projection_write_allowed: false,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        workflow_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        replay_alignment_readback_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "replay_execution_disabled",
            "projection_alignment_persistence_disabled",
            "work_graph_projection_write_disabled",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "workflow_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_REPLAY_ALIGNMENT_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_entries(
    source: &WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackReport,
) -> Vec<WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackEntry> {
    source
        .entries
        .iter()
        .map(|source_entry| {
            let replay_alignment_key = keyed_alignment(
                "temporal-lite.work-graph.replay-alignment.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let projection_replay_key = keyed_alignment(
                "temporal-lite.work-graph.projection-replay.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let replay_alignment_checksum = format!(
                "work-graph-replay-alignment-checksum.v1.{:03}.{}.{}.{}",
                source_entry.sequence,
                source_entry.event_contract_id,
                source_entry.projection_key.len(),
                source_entry.projection_checksum.len()
            );
            let work_graph_projection_projected = source_entry.work_graph_node_projected
                && source_entry.work_graph_event_edge_projected
                && source_entry.work_graph_state_edge_projected
                && source_entry.projection_checksum_projected;
            let deterministic_alignment_projected =
                work_graph_projection_projected && !source_entry.projection_key.is_empty();

            WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackEntry {
                event_contract_id: source_entry.event_contract_id,
                sequence: source_entry.sequence,
                event_id: source_entry.event_id.clone(),
                work_graph_node_key: source_entry.work_graph_node_key.clone(),
                work_graph_event_edge_key: source_entry.work_graph_event_edge_key.clone(),
                work_graph_state_edge_key: source_entry.work_graph_state_edge_key.clone(),
                projection_key: source_entry.projection_key.clone(),
                projection_checksum: source_entry.projection_checksum.clone(),
                replay_alignment_key,
                projection_replay_key,
                replay_alignment_checksum,
                expected_replay_projection_key: source_entry.projection_key.clone(),
                alignment_state: "aligned_not_replayed",
                readback_state: "work_graph_projection_replay_alignment_contract_projected_in_memory_only",
                work_graph_projection_projected,
                replay_alignment_projected: work_graph_projection_projected,
                projection_replay_key_projected: !source_entry.projection_key.is_empty(),
                replay_alignment_checksum_projected: !source_entry.projection_checksum.is_empty(),
                deterministic_alignment_projected,
                replay_executed: false,
                projection_alignment_persisted: false,
                work_graph_store_written: false,
                event_log_record_written: false,
                sqlite_row_written: false,
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

fn keyed_alignment(prefix: &str, sequence: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{sequence:03}.{event_contract_id}")
}

impl WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            replay_executed: false,
            projection_alignment_written: false,
            projection_alignment_persisted: false,
            work_graph_projection_written: false,
            event_log_written: false,
            sqlite_written: false,
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

    #[test]
    fn replay_alignment_projects_all_work_graph_entries_without_execution() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_projection_ready);
        assert_eq!(report.source_projection_entry_count, 9);
        assert_eq!(report.replay_alignment_projection_count, 9);
        assert_eq!(report.projection_replay_key_count, 9);
        assert_eq!(report.replay_alignment_checksum_count, 9);
        assert_eq!(report.deterministic_alignment_count, 9);
        assert_eq!(report.replay_executed_count, 0);
        assert_eq!(report.projection_alignment_persisted_count, 0);
        assert_eq!(report.work_graph_store_write_count, 0);
        assert_eq!(report.event_log_write_count, 0);
        assert_eq!(report.sqlite_write_count, 0);
        assert!(report.replay_alignment_contract_readback_materialized);
        assert!(report.replay_alignment_readback_ready);
    }

    #[test]
    fn replay_alignment_entries_are_contract_only() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry
                .replay_alignment_key
                .starts_with("temporal-lite.work-graph.replay-alignment.readback.")
                && entry
                    .projection_replay_key
                    .starts_with("temporal-lite.work-graph.projection-replay.readback.")
                && entry
                    .replay_alignment_checksum
                    .starts_with("work-graph-replay-alignment-checksum.v1.")
                && entry.expected_replay_projection_key == entry.projection_key
                && entry.alignment_state == "aligned_not_replayed"
                && entry.readback_state
                    == "work_graph_projection_replay_alignment_contract_projected_in_memory_only"
                && entry.work_graph_projection_projected
                && entry.replay_alignment_projected
                && entry.projection_replay_key_projected
                && entry.replay_alignment_checksum_projected
                && entry.deterministic_alignment_projected
                && !entry.replay_executed
                && !entry.projection_alignment_persisted
                && !entry.work_graph_store_written
                && !entry.event_log_record_written
                && !entry.sqlite_row_written
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
    fn replay_alignment_side_effects_remain_closed() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_report();

        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.replay_execution_allowed);
        assert!(!report.projection_alignment_persistence_allowed);
        assert!(!report.work_graph_projection_write_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteWorkGraphProjectionReplayAlignmentFeatureGatedReadbackSideEffects::none()
        );
    }
}
