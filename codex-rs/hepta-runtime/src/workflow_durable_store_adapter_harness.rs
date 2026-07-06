use serde::Serialize;

use crate::WorkflowDurableStoreAppendPlanReport;
use crate::hepta_workflow_durable_store_append_plan_report;

pub const WORKFLOW_DURABLE_STORE_ADAPTER_HARNESS_GATE: &str =
    "hepta_workflow_durable_store_adapter_harness_gate";
pub const WORKFLOW_DURABLE_STORE_ADAPTER_HARNESS_SCHEMA_VERSION: &str =
    "workflow_durable_store_adapter_harness_v1";
pub const WORKFLOW_DURABLE_STORE_ADAPTER_HARNESS_RECOMMENDED_NEXT_GATE: &str =
    "hepta_workflow_durable_store_adapter_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAdapterHarnessReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_append_plan_gate: &'static str,
    pub source_append_plan_ready: bool,
    pub noop_harness_binding_present: bool,
    pub append_plan_count: usize,
    pub noop_receipt_count: usize,
    pub append_attempt_count: usize,
    pub event_log_write_attempt_count: usize,
    pub sqlite_write_attempt_count: usize,
    pub checkpoint_write_attempt_count: usize,
    pub readback_execution_count: usize,
    pub replay_execution_count: usize,
    pub rollback_execution_count: usize,
    pub adapter_harness_ready: bool,
    pub ready_for_adapter_contract: bool,
    pub ready_for_event_log_write: bool,
    pub ready_for_live_execution: bool,
    pub receipts: Vec<WorkflowDurableStoreAdapterNoopReceipt>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowDurableStoreAdapterHarnessSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAdapterNoopReceipt {
    pub event_contract_id: &'static str,
    pub route: WorkflowDurableStoreAdapterHarnessRoute,
    pub append_suppressed_by_feature_gate: bool,
    pub noop_receipt_projected: bool,
    pub event_log_write_attempted: bool,
    pub sqlite_write_attempted: bool,
    pub checkpoint_write_attempted: bool,
    pub replay_execution_attempted: bool,
    pub rollback_execution_attempted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowDurableStoreAdapterHarnessRoute {
    NoopReceiptProjected,
    BlockedByMissingAppendPlan,
    BlockedByEnabledWriteAttempt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAdapterHarnessSideEffects {
    pub filesystem_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub checkpoint_written: bool,
    pub readback_performed: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub live_execution_started: bool,
}

pub fn hepta_workflow_durable_store_adapter_harness_report()
-> WorkflowDurableStoreAdapterHarnessReport {
    let append_plan = hepta_workflow_durable_store_append_plan_report();
    workflow_durable_store_adapter_harness_report(&append_plan)
}

pub fn workflow_durable_store_adapter_harness_report(
    append_plan: &WorkflowDurableStoreAppendPlanReport,
) -> WorkflowDurableStoreAdapterHarnessReport {
    let receipts = append_plan
        .append_plans
        .iter()
        .map(|plan| WorkflowDurableStoreAdapterNoopReceipt {
            event_contract_id: plan.event_contract_id,
            route: if plan.event_log_write_enabled
                || plan.sqlite_write_enabled
                || plan.checkpoint_write_enabled
                || plan.replay_execution_enabled
                || plan.rollback_execution_enabled
            {
                WorkflowDurableStoreAdapterHarnessRoute::BlockedByEnabledWriteAttempt
            } else if append_plan.status != "ready" {
                WorkflowDurableStoreAdapterHarnessRoute::BlockedByMissingAppendPlan
            } else {
                WorkflowDurableStoreAdapterHarnessRoute::NoopReceiptProjected
            },
            append_suppressed_by_feature_gate: !append_plan.feature_gate_enabled,
            noop_receipt_projected: append_plan.status == "ready"
                && !append_plan.feature_gate_enabled,
            event_log_write_attempted: false,
            sqlite_write_attempted: false,
            checkpoint_write_attempted: false,
            replay_execution_attempted: false,
            rollback_execution_attempted: false,
        })
        .collect::<Vec<_>>();
    let noop_receipt_count = receipts
        .iter()
        .filter(|receipt| receipt.noop_receipt_projected)
        .count();
    let adapter_harness_ready = append_plan.ready_for_adapter_harness
        && noop_receipt_count == append_plan.append_plan_count
        && receipts.iter().all(|receipt| {
            receipt.route == WorkflowDurableStoreAdapterHarnessRoute::NoopReceiptProjected
                && receipt.append_suppressed_by_feature_gate
                && !receipt.event_log_write_attempted
                && !receipt.sqlite_write_attempted
                && !receipt.checkpoint_write_attempted
                && !receipt.replay_execution_attempted
                && !receipt.rollback_execution_attempted
        });

    WorkflowDurableStoreAdapterHarnessReport {
        runtime: "hepta",
        surface: "workflow_durable_store_adapter_harness",
        status: if adapter_harness_ready {
            "ready"
        } else {
            "blocked"
        },
        gate: WORKFLOW_DURABLE_STORE_ADAPTER_HARNESS_GATE,
        schema_version: WORKFLOW_DURABLE_STORE_ADAPTER_HARNESS_SCHEMA_VERSION,
        source_append_plan_gate: append_plan.gate,
        source_append_plan_ready: append_plan.ready_for_adapter_harness,
        noop_harness_binding_present: true,
        append_plan_count: append_plan.append_plan_count,
        noop_receipt_count,
        append_attempt_count: 0,
        event_log_write_attempt_count: 0,
        sqlite_write_attempt_count: 0,
        checkpoint_write_attempt_count: 0,
        readback_execution_count: 0,
        replay_execution_count: 0,
        rollback_execution_count: 0,
        adapter_harness_ready,
        ready_for_adapter_contract: adapter_harness_ready,
        ready_for_event_log_write: false,
        ready_for_live_execution: false,
        receipts,
        recommended_next_gate: WORKFLOW_DURABLE_STORE_ADAPTER_HARNESS_RECOMMENDED_NEXT_GATE,
        side_effects: WorkflowDurableStoreAdapterHarnessSideEffects::none(),
    }
}

impl WorkflowDurableStoreAdapterHarnessSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_log_written: false,
            sqlite_written: false,
            checkpoint_written: false,
            readback_performed: false,
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
    fn durable_store_adapter_harness_projects_noop_receipts_for_all_append_plans() {
        let report = hepta_workflow_durable_store_adapter_harness_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.append_plan_count, 9);
        assert_eq!(report.noop_receipt_count, 9);
        assert!(report.adapter_harness_ready);
        assert!(report.ready_for_adapter_contract);
        assert!(report.noop_harness_binding_present);
        assert!(report.receipts.iter().all(|receipt| {
            receipt.route == WorkflowDurableStoreAdapterHarnessRoute::NoopReceiptProjected
                && receipt.append_suppressed_by_feature_gate
                && receipt.noop_receipt_projected
        }));
    }

    #[test]
    fn durable_store_adapter_harness_attempts_no_writes_or_execution() {
        let report = hepta_workflow_durable_store_adapter_harness_report();

        assert_eq!(report.append_attempt_count, 0);
        assert_eq!(report.event_log_write_attempt_count, 0);
        assert_eq!(report.sqlite_write_attempt_count, 0);
        assert_eq!(report.checkpoint_write_attempt_count, 0);
        assert_eq!(report.readback_execution_count, 0);
        assert_eq!(report.replay_execution_count, 0);
        assert_eq!(report.rollback_execution_count, 0);
        assert!(!report.ready_for_event_log_write);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkflowDurableStoreAdapterHarnessSideEffects::none()
        );
    }
}
