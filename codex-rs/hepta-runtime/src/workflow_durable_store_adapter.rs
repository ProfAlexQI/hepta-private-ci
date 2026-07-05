use serde::Serialize;

use crate::WorkflowDurableStoreAdapterHarnessReport;
use crate::WorkflowDurableStoreAppendPlanReport;
use crate::hepta_workflow_durable_store_adapter_harness_report;
use crate::hepta_workflow_durable_store_append_plan_report;

pub const WORKFLOW_DURABLE_STORE_ADAPTER_GATE: &str = "hepta_workflow_durable_store_adapter_gate";
pub const WORKFLOW_DURABLE_STORE_ADAPTER_SCHEMA_VERSION: &str = "workflow_durable_store_adapter_v1";
pub const WORKFLOW_DURABLE_STORE_ADAPTER_RECOMMENDED_NEXT_GATE: &str =
    "phase4_thread_thin_hepta_system_status_e2e_read_only_chain";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAdapterReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_append_plan_surface: &'static str,
    pub source_append_plan_ready: bool,
    pub source_harness_surface: &'static str,
    pub source_harness_ready: bool,
    pub event_contract_count: usize,
    pub append_plan_count: usize,
    pub lease_metadata_count: usize,
    pub idempotency_metadata_count: usize,
    pub checkpoint_metadata_count: usize,
    pub replay_validation_count: usize,
    pub rollback_metadata_count: usize,
    pub noop_receipt_count: usize,
    pub adapter_entry_count: usize,
    pub feature_gate_required: bool,
    pub feature_gate_enabled: bool,
    pub adapter_contract_ready: bool,
    pub temporal_lite_adapter_ready: bool,
    pub ready_for_event_log_write: bool,
    pub ready_for_sqlite_write: bool,
    pub ready_for_workflow_execution: bool,
    pub ready_for_replay_execution: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_live_execution: bool,
    pub entries: Vec<WorkflowDurableStoreAdapterEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowDurableStoreAdapterSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAdapterEntry {
    pub event_contract_id: &'static str,
    pub record_kind: &'static str,
    pub adapter_route: WorkflowDurableStoreAdapterRoute,
    pub lease_scope: &'static str,
    pub checkpoint_policy: &'static str,
    pub replay_validation_policy: &'static str,
    pub rollback_anchor: &'static str,
    pub feature_gate_required: bool,
    pub feature_gate_enabled: bool,
    pub event_log_write_enabled: bool,
    pub sqlite_write_enabled: bool,
    pub workflow_execution_enabled: bool,
    pub replay_execution_enabled: bool,
    pub rollback_execution_enabled: bool,
    pub live_execution_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowDurableStoreAdapterRoute {
    TemporalLitePlanReadyBehindFeatureGate,
    BlockedByAppendPlan,
    BlockedByHarness,
    BlockedByEnabledWriteOrExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAdapterSideEffects {
    pub filesystem_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub lease_acquired: bool,
    pub idempotency_index_mutated: bool,
    pub checkpoint_written: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub readback_performed: bool,
    pub live_execution_started: bool,
}

pub fn hepta_workflow_durable_store_adapter_report() -> WorkflowDurableStoreAdapterReport {
    let append_plan = hepta_workflow_durable_store_append_plan_report();
    let harness = hepta_workflow_durable_store_adapter_harness_report();
    workflow_durable_store_adapter_report(&append_plan, &harness)
}

pub fn workflow_durable_store_adapter_report(
    append_plan: &WorkflowDurableStoreAppendPlanReport,
    harness: &WorkflowDurableStoreAdapterHarnessReport,
) -> WorkflowDurableStoreAdapterReport {
    let entries = append_plan
        .append_plans
        .iter()
        .map(|plan| {
            let write_or_execution_enabled = plan.event_log_write_enabled
                || plan.sqlite_write_enabled
                || plan.checkpoint_write_enabled
                || plan.replay_execution_enabled
                || plan.rollback_execution_enabled
                || append_plan.feature_gate_enabled;
            let adapter_route = if write_or_execution_enabled {
                WorkflowDurableStoreAdapterRoute::BlockedByEnabledWriteOrExecution
            } else if !append_plan.ready_for_adapter_harness {
                WorkflowDurableStoreAdapterRoute::BlockedByAppendPlan
            } else if !harness.adapter_harness_ready {
                WorkflowDurableStoreAdapterRoute::BlockedByHarness
            } else {
                WorkflowDurableStoreAdapterRoute::TemporalLitePlanReadyBehindFeatureGate
            };

            WorkflowDurableStoreAdapterEntry {
                event_contract_id: plan.event_contract_id,
                record_kind: plan.record_kind,
                adapter_route,
                lease_scope: plan.lease_scope,
                checkpoint_policy: plan.checkpoint_policy,
                replay_validation_policy: plan.replay_validation_policy,
                rollback_anchor: plan.rollback_anchor,
                feature_gate_required: plan.feature_gate_required,
                feature_gate_enabled: append_plan.feature_gate_enabled,
                event_log_write_enabled: false,
                sqlite_write_enabled: false,
                workflow_execution_enabled: false,
                replay_execution_enabled: false,
                rollback_execution_enabled: false,
                live_execution_enabled: false,
            }
        })
        .collect::<Vec<_>>();
    let adapter_contract_ready = append_plan.ready_for_adapter_harness
        && harness.adapter_harness_ready
        && entries.len() == append_plan.append_plan_count
        && entries.iter().all(|entry| {
            entry.adapter_route
                == WorkflowDurableStoreAdapterRoute::TemporalLitePlanReadyBehindFeatureGate
                && entry.feature_gate_required
                && !entry.feature_gate_enabled
                && !entry.event_log_write_enabled
                && !entry.sqlite_write_enabled
                && !entry.workflow_execution_enabled
                && !entry.replay_execution_enabled
                && !entry.rollback_execution_enabled
                && !entry.live_execution_enabled
        });

    WorkflowDurableStoreAdapterReport {
        runtime: "hepta",
        surface: "workflow_durable_store_adapter",
        status: if adapter_contract_ready {
            "ready"
        } else {
            "blocked"
        },
        gate: WORKFLOW_DURABLE_STORE_ADAPTER_GATE,
        schema_version: WORKFLOW_DURABLE_STORE_ADAPTER_SCHEMA_VERSION,
        source_append_plan_surface: append_plan.surface,
        source_append_plan_ready: append_plan.ready_for_adapter_harness,
        source_harness_surface: harness.surface,
        source_harness_ready: harness.adapter_harness_ready,
        event_contract_count: append_plan.event_contract_count,
        append_plan_count: append_plan.append_plan_count,
        lease_metadata_count: append_plan.lease_metadata_count,
        idempotency_metadata_count: append_plan.idempotency_metadata_count,
        checkpoint_metadata_count: append_plan.checkpoint_metadata_count,
        replay_validation_count: append_plan.replay_validation_count,
        rollback_metadata_count: append_plan.rollback_metadata_count,
        noop_receipt_count: harness.noop_receipt_count,
        adapter_entry_count: entries.len(),
        feature_gate_required: true,
        feature_gate_enabled: false,
        adapter_contract_ready,
        temporal_lite_adapter_ready: adapter_contract_ready,
        ready_for_event_log_write: false,
        ready_for_sqlite_write: false,
        ready_for_workflow_execution: false,
        ready_for_replay_execution: false,
        ready_for_rollback_execution: false,
        ready_for_live_execution: false,
        entries,
        blockers: vec![
            "workflow_durable_store_feature_gate_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "rollback_execution_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate: WORKFLOW_DURABLE_STORE_ADAPTER_RECOMMENDED_NEXT_GATE,
        side_effects: WorkflowDurableStoreAdapterSideEffects::none(),
    }
}

impl WorkflowDurableStoreAdapterSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_log_written: false,
            sqlite_written: false,
            lease_acquired: false,
            idempotency_index_mutated: false,
            checkpoint_written: false,
            workflow_execution_started: false,
            replay_executed: false,
            rollback_executed: false,
            readback_performed: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durable_store_adapter_binds_append_plan_and_noop_harness() {
        let report = hepta_workflow_durable_store_adapter_report();

        assert_eq!(report.status, "ready");
        assert_eq!(
            report.source_append_plan_surface,
            "workflow_durable_store_append_plan"
        );
        assert!(report.source_append_plan_ready);
        assert_eq!(
            report.source_harness_surface,
            "workflow_durable_store_adapter_harness"
        );
        assert!(report.source_harness_ready);
        assert_eq!(report.event_contract_count, 9);
        assert_eq!(report.append_plan_count, 9);
        assert_eq!(report.adapter_entry_count, 9);
        assert_eq!(report.noop_receipt_count, 9);
        assert_eq!(report.lease_metadata_count, 9);
        assert_eq!(report.idempotency_metadata_count, 9);
        assert_eq!(report.checkpoint_metadata_count, 9);
        assert_eq!(report.replay_validation_count, 9);
        assert_eq!(report.rollback_metadata_count, 9);
        assert!(report.adapter_contract_ready);
        assert!(report.temporal_lite_adapter_ready);
    }

    #[test]
    fn durable_store_adapter_is_feature_gated_and_non_executing() {
        let report = hepta_workflow_durable_store_adapter_report();

        assert!(report.feature_gate_required);
        assert!(!report.feature_gate_enabled);
        assert!(!report.ready_for_event_log_write);
        assert!(!report.ready_for_sqlite_write);
        assert!(!report.ready_for_workflow_execution);
        assert!(!report.ready_for_replay_execution);
        assert!(!report.ready_for_rollback_execution);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkflowDurableStoreAdapterSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            entry.adapter_route
                == WorkflowDurableStoreAdapterRoute::TemporalLitePlanReadyBehindFeatureGate
                && entry.feature_gate_required
                && !entry.feature_gate_enabled
                && !entry.event_log_write_enabled
                && !entry.sqlite_write_enabled
                && !entry.workflow_execution_enabled
                && !entry.replay_execution_enabled
                && !entry.rollback_execution_enabled
                && !entry.live_execution_enabled
        }));
    }
}
