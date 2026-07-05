use serde::Serialize;

use crate::WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackReport;
use crate::hepta_workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_GATE: &str =
    "workflow_temporal_lite_work_graph_projection_feature_gated_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_SCHEMA_VERSION: &str =
    "workflow_temporal_lite_work_graph_projection_feature_gated_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_adapter_gate: &'static str,
    pub source_adapter_ready: bool,
    pub source_adapter_entry_count: usize,
    pub projection_scope: &'static str,
    pub work_graph_node_projection_count: usize,
    pub work_graph_event_edge_projection_count: usize,
    pub work_graph_state_edge_projection_count: usize,
    pub projection_key_count: usize,
    pub projection_checksum_count: usize,
    pub projection_persisted_count: usize,
    pub work_graph_store_write_count: usize,
    pub event_log_write_count: usize,
    pub sqlite_write_count: usize,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub projection_contract_readback_materialized: bool,
    pub work_graph_projection_write_allowed: bool,
    pub work_graph_projection_persistence_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub work_graph_projection_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackEntry {
    pub event_contract_id: &'static str,
    pub sequence: usize,
    pub event_id: String,
    pub event_log_record_key: String,
    pub sqlite_row_key: String,
    pub work_graph_node_key: String,
    pub work_graph_node_kind: &'static str,
    pub work_graph_event_edge_key: String,
    pub work_graph_state_edge_key: String,
    pub projection_key: String,
    pub projection_checksum: String,
    pub projection_state: &'static str,
    pub readback_state: &'static str,
    pub work_graph_node_projected: bool,
    pub work_graph_event_edge_projected: bool,
    pub work_graph_state_edge_projected: bool,
    pub projection_checksum_projected: bool,
    pub projection_persisted: bool,
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
pub struct WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackSideEffects {
    pub filesystem_written: bool,
    pub work_graph_projection_written: bool,
    pub work_graph_projection_persisted: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
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

pub fn hepta_workflow_temporal_lite_work_graph_projection_feature_gated_readback_report()
-> WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackReport {
    let source =
        hepta_workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report();
    workflow_temporal_lite_work_graph_projection_feature_gated_readback_report_from_source(&source)
}

pub fn workflow_temporal_lite_work_graph_projection_feature_gated_readback_report_from_source(
    source: &WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackReport,
) -> WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackReport {
    let entries =
        workflow_temporal_lite_work_graph_projection_feature_gated_readback_entries(source);
    let work_graph_node_projection_count = entries
        .iter()
        .filter(|entry| entry.work_graph_node_projected)
        .count();
    let work_graph_event_edge_projection_count = entries
        .iter()
        .filter(|entry| entry.work_graph_event_edge_projected)
        .count();
    let work_graph_state_edge_projection_count = entries
        .iter()
        .filter(|entry| entry.work_graph_state_edge_projected)
        .count();
    let projection_key_count = entries
        .iter()
        .filter(|entry| !entry.projection_key.is_empty())
        .count();
    let projection_checksum_count = entries
        .iter()
        .filter(|entry| entry.projection_checksum_projected)
        .count();
    let projection_persisted_count = entries
        .iter()
        .filter(|entry| entry.projection_persisted)
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
    let work_graph_projection_readback_ready = source.event_log_sqlite_adapter_readback_ready
        && source.source_lease_idempotency_entry_count == 9
        && source.event_log_adapter_readback_count == 9
        && source.sqlite_adapter_readback_count == 9
        && source.event_log_record_written_count == 0
        && source.sqlite_row_written_count == 0
        && source.adapter_persisted_count == 0
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.source_lease_idempotency_entry_count
        && work_graph_node_projection_count == entries.len()
        && work_graph_event_edge_projection_count == entries.len()
        && work_graph_state_edge_projection_count == entries.len()
        && projection_key_count == entries.len()
        && projection_checksum_count == entries.len()
        && projection_persisted_count == 0
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

    WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_work_graph_projection_feature_gated_readback",
        status: if work_graph_projection_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_SCHEMA_VERSION,
        source_adapter_gate: source.gate,
        source_adapter_ready: source.event_log_sqlite_adapter_readback_ready,
        source_adapter_entry_count: source.source_lease_idempotency_entry_count,
        projection_scope: "test_only_work_graph_projection_readback_no_persistence",
        work_graph_node_projection_count,
        work_graph_event_edge_projection_count,
        work_graph_state_edge_projection_count,
        projection_key_count,
        projection_checksum_count,
        projection_persisted_count,
        work_graph_store_write_count,
        event_log_write_count,
        sqlite_write_count,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        projection_contract_readback_materialized: work_graph_projection_readback_ready,
        work_graph_projection_write_allowed: false,
        work_graph_projection_persistence_allowed: false,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        work_graph_projection_readback_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "work_graph_projection_write_disabled",
            "work_graph_projection_persistence_disabled",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_work_graph_projection_feature_gated_readback_entries(
    source: &WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackReport,
) -> Vec<WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackEntry> {
    source
        .entries
        .iter()
        .map(|source_entry| {
            let work_graph_node_key = keyed_projection(
                "temporal-lite.work-graph.node.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let work_graph_event_edge_key = keyed_projection(
                "temporal-lite.work-graph.event-edge.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let work_graph_state_edge_key = keyed_projection(
                "temporal-lite.work-graph.state-edge.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let projection_key = keyed_projection(
                "temporal-lite.work-graph.projection.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let projection_checksum = format!(
                "work-graph-projection-checksum.v1.{:03}.{}.{}.{}",
                source_entry.sequence,
                source_entry.event_contract_id,
                source_entry.event_log_record_key.len(),
                source_entry.sqlite_row_key.len()
            );

            WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackEntry {
                event_contract_id: source_entry.event_contract_id,
                sequence: source_entry.sequence,
                event_id: source_entry.event_id.clone(),
                event_log_record_key: source_entry.event_log_record_key.clone(),
                sqlite_row_key: source_entry.sqlite_row_key.clone(),
                work_graph_node_key,
                work_graph_node_kind: work_graph_node_kind(source_entry.event_contract_id),
                work_graph_event_edge_key,
                work_graph_state_edge_key,
                projection_key,
                projection_checksum,
                projection_state: "projected_not_persisted",
                readback_state: "work_graph_projection_contract_projected_in_memory_only",
                work_graph_node_projected: source_entry.event_log_adapter_projected,
                work_graph_event_edge_projected: source_entry.event_log_adapter_projected,
                work_graph_state_edge_projected: source_entry.sqlite_adapter_projected,
                projection_checksum_projected: source_entry.serialization_contract_projected,
                projection_persisted: false,
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

fn keyed_projection(prefix: &str, sequence: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{sequence:03}.{event_contract_id}")
}

fn work_graph_node_kind(event_contract_id: &str) -> &'static str {
    if event_contract_id.contains("approval") {
        "approval_event"
    } else if event_contract_id.contains("task_result") {
        "task_result_event"
    } else if event_contract_id.contains("checkpoint") {
        "checkpoint_event"
    } else {
        "workflow_event"
    }
}

impl WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            work_graph_projection_written: false,
            work_graph_projection_persisted: false,
            event_log_written: false,
            sqlite_written: false,
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

    #[test]
    fn work_graph_projection_projects_all_adapter_entries_without_writes() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_feature_gated_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_adapter_ready);
        assert_eq!(report.source_adapter_entry_count, 9);
        assert_eq!(report.work_graph_node_projection_count, 9);
        assert_eq!(report.work_graph_event_edge_projection_count, 9);
        assert_eq!(report.work_graph_state_edge_projection_count, 9);
        assert_eq!(report.projection_key_count, 9);
        assert_eq!(report.projection_checksum_count, 9);
        assert_eq!(report.projection_persisted_count, 0);
        assert_eq!(report.work_graph_store_write_count, 0);
        assert_eq!(report.event_log_write_count, 0);
        assert_eq!(report.sqlite_write_count, 0);
        assert!(report.projection_contract_readback_materialized);
        assert!(report.work_graph_projection_readback_ready);
    }

    #[test]
    fn work_graph_projection_entries_are_contract_only() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_feature_gated_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry
                .work_graph_node_key
                .starts_with("temporal-lite.work-graph.node.readback.")
                && entry
                    .work_graph_event_edge_key
                    .starts_with("temporal-lite.work-graph.event-edge.readback.")
                && entry
                    .work_graph_state_edge_key
                    .starts_with("temporal-lite.work-graph.state-edge.readback.")
                && entry
                    .projection_key
                    .starts_with("temporal-lite.work-graph.projection.readback.")
                && entry
                    .projection_checksum
                    .starts_with("work-graph-projection-checksum.v1.")
                && entry.projection_state == "projected_not_persisted"
                && entry.readback_state == "work_graph_projection_contract_projected_in_memory_only"
                && entry.work_graph_node_projected
                && entry.work_graph_event_edge_projected
                && entry.work_graph_state_edge_projected
                && entry.projection_checksum_projected
                && !entry.projection_persisted
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
        assert!(report.entries.iter().any(|entry| {
            entry.event_contract_id == "approval_event_intake"
                && entry.work_graph_node_kind == "approval_event"
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.event_contract_id == "task_result_event_intake"
                && entry.work_graph_node_kind == "task_result_event"
        }));
    }

    #[test]
    fn work_graph_projection_side_effects_remain_closed() {
        let report =
            hepta_workflow_temporal_lite_work_graph_projection_feature_gated_readback_report();

        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.work_graph_projection_write_allowed);
        assert!(!report.work_graph_projection_persistence_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteWorkGraphProjectionFeatureGatedReadbackSideEffects::none()
        );
    }
}
