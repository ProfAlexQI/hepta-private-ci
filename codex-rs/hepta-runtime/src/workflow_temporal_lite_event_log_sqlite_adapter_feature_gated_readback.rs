use serde::Serialize;

use crate::WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackReport;
use crate::hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report;

pub const WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_GATE: &str =
    "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_gate";
pub const WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_SCHEMA_VERSION:
    &str = "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_v1";
pub const WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "temporal_lite_work_graph_projection_feature_gated_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_lease_idempotency_gate: &'static str,
    pub source_lease_idempotency_ready: bool,
    pub source_lease_idempotency_entry_count: usize,
    pub adapter_scope: &'static str,
    pub event_log_adapter_readback_count: usize,
    pub sqlite_adapter_readback_count: usize,
    pub event_log_record_key_count: usize,
    pub sqlite_row_key_count: usize,
    pub serialization_contract_count: usize,
    pub transaction_boundary_count: usize,
    pub event_log_record_written_count: usize,
    pub sqlite_row_written_count: usize,
    pub adapter_persisted_count: usize,
    pub feature_gate_required: bool,
    pub runtime_feature_gate_enabled: bool,
    pub adapter_contract_readback_materialized: bool,
    pub runtime_event_log_write_allowed: bool,
    pub runtime_sqlite_write_allowed: bool,
    pub event_log_adapter_write_allowed: bool,
    pub sqlite_adapter_write_allowed: bool,
    pub adapter_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_execution_allowed: bool,
    pub event_log_sqlite_adapter_readback_ready: bool,
    pub entries: Vec<WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackEntry {
    pub event_contract_id: &'static str,
    pub sequence: usize,
    pub event_id: String,
    pub lease_key: String,
    pub idempotency_index_key: String,
    pub idempotency_key: String,
    pub event_log_adapter_key: String,
    pub event_log_stream: &'static str,
    pub event_log_record_key: String,
    pub event_log_record_schema: &'static str,
    pub sqlite_adapter_key: String,
    pub sqlite_table: &'static str,
    pub sqlite_row_key: String,
    pub sqlite_schema_version: &'static str,
    pub serialization_contract_key: String,
    pub transaction_boundary_key: String,
    pub adapter_state: &'static str,
    pub readback_state: &'static str,
    pub event_log_adapter_projected: bool,
    pub sqlite_adapter_projected: bool,
    pub serialization_contract_projected: bool,
    pub transaction_boundary_projected: bool,
    pub event_log_record_written: bool,
    pub sqlite_row_written: bool,
    pub adapter_persisted: bool,
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
pub struct WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackSideEffects {
    pub filesystem_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub adapter_persisted: bool,
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

pub fn hepta_workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report()
-> WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackReport {
    let source =
        hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report();
    workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report_from_source(
        &source,
    )
}

pub fn workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report_from_source(
    source: &WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackReport,
) -> WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackReport {
    let entries =
        workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_entries(source);
    let event_log_adapter_readback_count = entries
        .iter()
        .filter(|entry| entry.event_log_adapter_projected)
        .count();
    let sqlite_adapter_readback_count = entries
        .iter()
        .filter(|entry| entry.sqlite_adapter_projected)
        .count();
    let event_log_record_key_count = entries
        .iter()
        .filter(|entry| !entry.event_log_record_key.is_empty())
        .count();
    let sqlite_row_key_count = entries
        .iter()
        .filter(|entry| !entry.sqlite_row_key.is_empty())
        .count();
    let serialization_contract_count = entries
        .iter()
        .filter(|entry| entry.serialization_contract_projected)
        .count();
    let transaction_boundary_count = entries
        .iter()
        .filter(|entry| entry.transaction_boundary_projected)
        .count();
    let event_log_record_written_count = entries
        .iter()
        .filter(|entry| entry.event_log_record_written)
        .count();
    let sqlite_row_written_count = entries
        .iter()
        .filter(|entry| entry.sqlite_row_written)
        .count();
    let adapter_persisted_count = entries
        .iter()
        .filter(|entry| entry.adapter_persisted)
        .count();
    let event_log_sqlite_adapter_readback_ready = source.lease_idempotency_index_readback_ready
        && source.source_anchor_pair_count == 9
        && source.lease_readback_count == 9
        && source.idempotency_index_readback_count == 9
        && source.lease_acquired_count == 0
        && source.lease_persisted_count == 0
        && source.idempotency_index_written_count == 0
        && source.idempotency_index_persisted_count == 0
        && !source.runtime_feature_gate_enabled
        && !source.runtime_event_log_write_allowed
        && !source.runtime_sqlite_write_allowed
        && !source.workflow_execution_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_execution_allowed
        && entries.len() == source.source_anchor_pair_count
        && event_log_adapter_readback_count == entries.len()
        && sqlite_adapter_readback_count == entries.len()
        && event_log_record_key_count == entries.len()
        && sqlite_row_key_count == entries.len()
        && serialization_contract_count == entries.len()
        && transaction_boundary_count == entries.len()
        && event_log_record_written_count == 0
        && sqlite_row_written_count == 0
        && adapter_persisted_count == 0
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

    WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackReport {
        runtime: "hepta",
        surface: "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback",
        status: if event_log_sqlite_adapter_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_GATE,
        schema_version:
            WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_SCHEMA_VERSION,
        source_lease_idempotency_gate: source.gate,
        source_lease_idempotency_ready: source.lease_idempotency_index_readback_ready,
        source_lease_idempotency_entry_count: source.source_anchor_pair_count,
        adapter_scope: "test_only_event_log_sqlite_adapter_readback_no_writes",
        event_log_adapter_readback_count,
        sqlite_adapter_readback_count,
        event_log_record_key_count,
        sqlite_row_key_count,
        serialization_contract_count,
        transaction_boundary_count,
        event_log_record_written_count,
        sqlite_row_written_count,
        adapter_persisted_count,
        feature_gate_required: true,
        runtime_feature_gate_enabled: false,
        adapter_contract_readback_materialized: event_log_sqlite_adapter_readback_ready,
        runtime_event_log_write_allowed: false,
        runtime_sqlite_write_allowed: false,
        event_log_adapter_write_allowed: false,
        sqlite_adapter_write_allowed: false,
        adapter_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_execution_allowed: false,
        event_log_sqlite_adapter_readback_ready,
        entries,
        blockers: vec![
            "runtime_feature_gate_closed",
            "runtime_event_log_write_disabled",
            "runtime_sqlite_write_disabled",
            "event_log_adapter_write_disabled",
            "sqlite_adapter_write_disabled",
            "adapter_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects:
            WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackSideEffects::none(),
    }
}

pub fn workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_entries(
    source: &WorkflowTemporalLiteLeaseIdempotencyIndexFeatureGatedReadbackReport,
) -> Vec<WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackEntry> {
    source
        .entries
        .iter()
        .map(|source_entry| {
            let event_log_adapter_key = keyed_adapter(
                "temporal-lite.event-log.adapter.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let event_log_record_key = format!(
                "event-log-record.v1.{:03}.{}.{}",
                source_entry.sequence,
                source_entry.event_contract_id,
                source_entry.idempotency_key.len()
            );
            let sqlite_adapter_key = keyed_adapter(
                "temporal-lite.sqlite.adapter.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let sqlite_row_key = format!(
                "sqlite-row.v1.{:03}.{}.{}",
                source_entry.sequence,
                source_entry.event_contract_id,
                source_entry.lease_key.len()
            );
            let serialization_contract_key = keyed_adapter(
                "temporal-lite.serialization-contract.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );
            let transaction_boundary_key = keyed_adapter(
                "temporal-lite.transaction-boundary.readback",
                source_entry.sequence,
                source_entry.event_contract_id,
            );

            WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackEntry {
                event_contract_id: source_entry.event_contract_id,
                sequence: source_entry.sequence,
                event_id: source_entry.event_id.clone(),
                lease_key: source_entry.lease_key.clone(),
                idempotency_index_key: source_entry.idempotency_index_key.clone(),
                idempotency_key: source_entry.idempotency_key.clone(),
                event_log_adapter_key,
                event_log_stream: "temporal_lite_test_only_event_log_stream",
                event_log_record_key,
                event_log_record_schema: "temporal_lite_event_log_record_v1",
                sqlite_adapter_key,
                sqlite_table: "temporal_lite_test_only_events",
                sqlite_row_key,
                sqlite_schema_version: "temporal_lite_sqlite_adapter_v1",
                serialization_contract_key,
                transaction_boundary_key,
                adapter_state: "projected_not_persisted",
                readback_state: "adapter_contract_projected_in_memory_only",
                event_log_adapter_projected: source_entry.lease_readback_projected,
                sqlite_adapter_projected: source_entry.idempotency_index_projected,
                serialization_contract_projected: source_entry.duplicate_guard_projected,
                transaction_boundary_projected: source_entry.duplicate_guard_projected,
                event_log_record_written: false,
                sqlite_row_written: false,
                adapter_persisted: false,
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

fn keyed_adapter(prefix: &str, sequence: usize, event_contract_id: &str) -> String {
    format!("{prefix}.{sequence:03}.{event_contract_id}")
}

impl WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_log_written: false,
            sqlite_written: false,
            adapter_persisted: false,
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
    fn event_log_sqlite_adapter_projects_all_entries_without_writes() {
        let report =
            hepta_workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_lease_idempotency_ready);
        assert_eq!(report.source_lease_idempotency_entry_count, 9);
        assert_eq!(report.event_log_adapter_readback_count, 9);
        assert_eq!(report.sqlite_adapter_readback_count, 9);
        assert_eq!(report.event_log_record_key_count, 9);
        assert_eq!(report.sqlite_row_key_count, 9);
        assert_eq!(report.serialization_contract_count, 9);
        assert_eq!(report.transaction_boundary_count, 9);
        assert_eq!(report.event_log_record_written_count, 0);
        assert_eq!(report.sqlite_row_written_count, 0);
        assert_eq!(report.adapter_persisted_count, 0);
        assert!(report.adapter_contract_readback_materialized);
        assert!(report.event_log_sqlite_adapter_readback_ready);
    }

    #[test]
    fn event_log_sqlite_adapter_entries_are_contract_only() {
        let report =
            hepta_workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry
                .event_log_adapter_key
                .starts_with("temporal-lite.event-log.adapter.readback.")
                && entry.event_log_stream == "temporal_lite_test_only_event_log_stream"
                && entry
                    .event_log_record_key
                    .starts_with("event-log-record.v1.")
                && entry.event_log_record_schema == "temporal_lite_event_log_record_v1"
                && entry
                    .sqlite_adapter_key
                    .starts_with("temporal-lite.sqlite.adapter.readback.")
                && entry.sqlite_table == "temporal_lite_test_only_events"
                && entry.sqlite_row_key.starts_with("sqlite-row.v1.")
                && entry.sqlite_schema_version == "temporal_lite_sqlite_adapter_v1"
                && entry
                    .serialization_contract_key
                    .starts_with("temporal-lite.serialization-contract.readback.")
                && entry
                    .transaction_boundary_key
                    .starts_with("temporal-lite.transaction-boundary.readback.")
                && entry.adapter_state == "projected_not_persisted"
                && entry.readback_state == "adapter_contract_projected_in_memory_only"
                && entry.event_log_adapter_projected
                && entry.sqlite_adapter_projected
                && entry.serialization_contract_projected
                && entry.transaction_boundary_projected
                && !entry.event_log_record_written
                && !entry.sqlite_row_written
                && !entry.adapter_persisted
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
    fn event_log_sqlite_adapter_side_effects_remain_closed() {
        let report =
            hepta_workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report();

        assert!(report.feature_gate_required);
        assert!(!report.runtime_feature_gate_enabled);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.runtime_sqlite_write_allowed);
        assert!(!report.event_log_adapter_write_allowed);
        assert!(!report.sqlite_adapter_write_allowed);
        assert!(!report.adapter_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkflowTemporalLiteEventLogSqliteAdapterFeatureGatedReadbackSideEffects::none()
        );
    }
}
