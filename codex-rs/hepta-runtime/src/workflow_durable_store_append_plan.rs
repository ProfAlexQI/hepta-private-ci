use serde::Serialize;

use crate::WorkGraphAppendOnlyEventContractPreview;
use crate::hepta_work_graph_append_only_event_intake_preview_report;

pub const WORKFLOW_DURABLE_STORE_APPEND_PLAN_GATE: &str =
    "hepta_workflow_durable_store_append_plan_gate";
pub const WORKFLOW_DURABLE_STORE_APPEND_PLAN_SCHEMA_VERSION: &str =
    "workflow_durable_store_append_plan_v1";
pub const WORKFLOW_DURABLE_STORE_APPEND_PLAN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_workflow_durable_store_adapter_harness_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAppendPlanReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_gate: &'static str,
    pub source_append_only_event_intake_ready: bool,
    pub event_contract_count: usize,
    pub append_plan_count: usize,
    pub lease_metadata_count: usize,
    pub idempotency_metadata_count: usize,
    pub checkpoint_metadata_count: usize,
    pub replay_validation_count: usize,
    pub rollback_metadata_count: usize,
    pub feature_gate_required: bool,
    pub feature_gate_enabled: bool,
    pub ready_for_adapter_harness: bool,
    pub ready_for_event_log_write: bool,
    pub ready_for_sqlite_write: bool,
    pub ready_for_live_execution: bool,
    pub append_plans: Vec<WorkflowDurableStoreAppendPlanEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: WorkflowDurableStoreAppendPlanSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAppendPlanEntry {
    pub event_contract_id: &'static str,
    pub record_kind: &'static str,
    pub target_collection_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub idempotency_key_fields: Vec<&'static str>,
    pub lease_scope: &'static str,
    pub checkpoint_policy: &'static str,
    pub replay_validation_policy: &'static str,
    pub rollback_anchor: &'static str,
    pub append_policy: &'static str,
    pub feature_gate_required: bool,
    pub event_log_write_enabled: bool,
    pub sqlite_write_enabled: bool,
    pub checkpoint_write_enabled: bool,
    pub replay_execution_enabled: bool,
    pub rollback_execution_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkflowDurableStoreAppendPlanSideEffects {
    pub filesystem_written: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub lease_acquired: bool,
    pub idempotency_index_mutated: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub workflow_execution_started: bool,
    pub live_execution_started: bool,
}

pub fn hepta_workflow_durable_store_append_plan_report() -> WorkflowDurableStoreAppendPlanReport {
    let source = hepta_work_graph_append_only_event_intake_preview_report();
    let append_plans = workflow_durable_store_append_plans_from_contracts(&source.event_contracts);
    let ready_for_adapter_harness = source.status == "ready"
        && source.ready_for_replay_readback_preview
        && !source.ready_for_append_only_store_enablement
        && !source.ready_for_store_persistence
        && !source.ready_for_live_execution
        && append_plans.len() == source.event_contract_count
        && append_plans.iter().all(|plan| {
            plan.feature_gate_required
                && !plan.event_log_write_enabled
                && !plan.sqlite_write_enabled
                && !plan.checkpoint_write_enabled
                && !plan.replay_execution_enabled
                && !plan.rollback_execution_enabled
        });

    WorkflowDurableStoreAppendPlanReport {
        runtime: "hepta",
        surface: "workflow_durable_store_append_plan",
        status: if ready_for_adapter_harness {
            "ready"
        } else {
            "blocked"
        },
        gate: WORKFLOW_DURABLE_STORE_APPEND_PLAN_GATE,
        schema_version: WORKFLOW_DURABLE_STORE_APPEND_PLAN_SCHEMA_VERSION,
        source_gate: source.gate,
        source_append_only_event_intake_ready: source.status == "ready",
        event_contract_count: source.event_contract_count,
        append_plan_count: append_plans.len(),
        lease_metadata_count: append_plans.len(),
        idempotency_metadata_count: append_plans.len(),
        checkpoint_metadata_count: append_plans.len(),
        replay_validation_count: append_plans.len(),
        rollback_metadata_count: append_plans.len(),
        feature_gate_required: true,
        feature_gate_enabled: false,
        ready_for_adapter_harness,
        ready_for_event_log_write: false,
        ready_for_sqlite_write: false,
        ready_for_live_execution: false,
        append_plans,
        recommended_next_gate: WORKFLOW_DURABLE_STORE_APPEND_PLAN_RECOMMENDED_NEXT_GATE,
        side_effects: WorkflowDurableStoreAppendPlanSideEffects::none(),
    }
}

pub fn workflow_durable_store_append_plans_from_contracts(
    contracts: &[WorkGraphAppendOnlyEventContractPreview],
) -> Vec<WorkflowDurableStoreAppendPlanEntry> {
    contracts.iter().map(append_plan_entry).collect()
}

fn append_plan_entry(
    contract: &WorkGraphAppendOnlyEventContractPreview,
) -> WorkflowDurableStoreAppendPlanEntry {
    WorkflowDurableStoreAppendPlanEntry {
        event_contract_id: contract.id,
        record_kind: contract.record_kind,
        target_collection_ids: contract.target_collection_ids.clone(),
        required_fields: contract.required_fields.clone(),
        idempotency_key_fields: contract.idempotency_key_fields.clone(),
        lease_scope: lease_scope_for_record_kind(contract.record_kind),
        checkpoint_policy: "checkpoint_metadata_only_no_checkpoint_write",
        replay_validation_policy: "deterministic_replay_validation_metadata_only",
        rollback_anchor: rollback_anchor_for_record_kind(contract.record_kind),
        append_policy: "append_plan_only_feature_gate_required",
        feature_gate_required: true,
        event_log_write_enabled: false,
        sqlite_write_enabled: false,
        checkpoint_write_enabled: false,
        replay_execution_enabled: false,
        rollback_execution_enabled: false,
    }
}

fn lease_scope_for_record_kind(record_kind: &str) -> &'static str {
    match record_kind {
        "plan_step_event" => "workflow_run_plan_projection_lease",
        "agent_spawn_event" => "workflow_run_agent_spawn_lease",
        "mailbox_delivery_event" => "workflow_run_mailbox_delivery_lease",
        "agent_job_item_event" => "workflow_run_agent_job_item_lease",
        "worker_task_event" => "workflow_run_worker_task_lease",
        "scheduler_run_event" => "workflow_run_scheduler_lease",
        "artifact_event" => "workflow_run_artifact_lease",
        "approval_event" => "workflow_run_approval_projection_lease",
        "task_result_event" => "workflow_run_task_result_lease",
        _ => "workflow_run_unknown_projection_lease",
    }
}

fn rollback_anchor_for_record_kind(record_kind: &str) -> &'static str {
    match record_kind {
        "plan_step_event" => "rollback_to_prior_plan_projection_checkpoint",
        "agent_spawn_event" => "rollback_to_parent_thread_spawn_anchor",
        "mailbox_delivery_event" => "rollback_to_prior_mailbox_sequence_anchor",
        "agent_job_item_event" => "rollback_to_prior_job_item_attempt_anchor",
        "worker_task_event" => "rollback_to_prior_worker_task_attempt_anchor",
        "scheduler_run_event" => "rollback_to_prior_scheduler_lease_anchor",
        "artifact_event" => "rollback_to_prior_artifact_hash_anchor",
        "approval_event" => "rollback_to_prior_approval_scope_anchor",
        "task_result_event" => "rollback_to_prior_task_result_anchor",
        _ => "rollback_to_prior_unknown_event_anchor",
    }
}

impl WorkflowDurableStoreAppendPlanSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_log_written: false,
            sqlite_written: false,
            lease_acquired: false,
            idempotency_index_mutated: false,
            checkpoint_written: false,
            replay_executed: false,
            rollback_executed: false,
            workflow_execution_started: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durable_store_append_plan_covers_all_append_only_event_contracts() {
        let report = hepta_workflow_durable_store_append_plan_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.event_contract_count, 9);
        assert_eq!(report.append_plan_count, 9);
        assert_eq!(report.lease_metadata_count, 9);
        assert_eq!(report.idempotency_metadata_count, 9);
        assert_eq!(report.checkpoint_metadata_count, 9);
        assert_eq!(report.replay_validation_count, 9);
        assert_eq!(report.rollback_metadata_count, 9);
        assert!(report.ready_for_adapter_harness);
    }

    #[test]
    fn durable_store_append_plan_keeps_writes_and_execution_disabled() {
        let report = hepta_workflow_durable_store_append_plan_report();

        assert!(report.feature_gate_required);
        assert!(!report.feature_gate_enabled);
        assert!(!report.ready_for_event_log_write);
        assert!(!report.ready_for_sqlite_write);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkflowDurableStoreAppendPlanSideEffects::none()
        );
        assert!(report.append_plans.iter().all(|plan| {
            plan.feature_gate_required
                && !plan.event_log_write_enabled
                && !plan.sqlite_write_enabled
                && !plan.checkpoint_write_enabled
                && !plan.replay_execution_enabled
                && !plan.rollback_execution_enabled
        }));
    }
}
