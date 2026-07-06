use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_report,
};
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
    hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report,
};
use crate::work_graph_scheduler_admission_dry_run_enforcement::{
    WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    hepta_work_graph_scheduler_admission_dry_run_enforcement_report,
};
use crate::work_graph_trace_guardrail_span_report_only::{
    WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
    hepta_work_graph_trace_guardrail_span_report_only_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_scheduler_gate: &'static str,
    pub source_scheduler_entrypoint_count: usize,
    pub source_scheduler_check_count: usize,
    pub source_trace_guardrail_gate: &'static str,
    pub source_trace_span_count: usize,
    pub source_blocking_guardrail_count: usize,
    pub source_entrypoint_emission_gate: &'static str,
    pub source_emission_count: usize,
    pub source_final_closeout_gate: &'static str,
    pub source_final_closeout_entry_count: usize,
    pub entrypoint_binding_count: usize,
    pub guardrail_check_count: usize,
    pub dry_run_decision_count: usize,
    pub required_prior_gate_count: usize,
    pub entrypoint_bindings: Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointBindingPreview>,
    pub guardrail_checks: Vec<WorkGraphSchedulerGuardrailBlockingDryRunCheckPreview>,
    pub dry_run_decisions: Vec<WorkGraphSchedulerGuardrailBlockingDryRunDecisionPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub scheduler_admission_dry_run_present: bool,
    pub blocking_guardrail_dry_run_attached: bool,
    pub pre_entrypoint_hook_contract_ready: bool,
    pub live_blocking_enforcement_enabled: bool,
    pub runtime_interception_enabled: bool,
    pub work_graph_event_persistence_enabled: bool,
    pub ready_for_work_graph_shadow_event_store_readback: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointBindingPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub hook_position: &'static str,
    pub dry_run_decision: &'static str,
    pub applied_check_ids: Vec<&'static str>,
    pub required_trace_fields: Vec<&'static str>,
    pub would_block_if_live: bool,
    pub dry_run_allows_current_runtime_to_continue: bool,
    pub live_blocking_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunCheckPreview {
    pub id: &'static str,
    pub source: &'static str,
    pub blocks_live_execution: bool,
    pub dry_run_explanation_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunDecisionPreview {
    pub entrypoint_id: &'static str,
    pub outcome: &'static str,
    pub reason: &'static str,
    pub trace_id: &'static str,
    pub allow_current_runtime_to_continue: bool,
    pub block_live_execution: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub live_blocking_hook_installed: bool,
    pub lease_acquired: bool,
    pub work_started: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub canary_traffic_routed: bool,
    pub operator_review_requested: bool,
    pub approval_recorded: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointReport {
    let scheduler = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
    let trace_guardrail = hepta_work_graph_trace_guardrail_span_report_only_report();
    let entrypoint_emission =
        hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report();
    let final_closeout =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_report();
    let entrypoint_bindings =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_bindings();
    let guardrail_checks =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_checks();
    let dry_run_decisions =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_decisions();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_required_prior_gates(
        );

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_SCHEMA_VERSION,
        preview_mode: "blocking_guardrail_dry_run_before_entrypoint_no_live_enforcement",
        source_scheduler_gate: scheduler.gate,
        source_scheduler_entrypoint_count: scheduler.entrypoint_count,
        source_scheduler_check_count: scheduler.check_count,
        source_trace_guardrail_gate: trace_guardrail.gate,
        source_trace_span_count: trace_guardrail.span_count,
        source_blocking_guardrail_count: trace_guardrail.blocking_guardrail_count,
        source_entrypoint_emission_gate: entrypoint_emission.gate,
        source_emission_count: entrypoint_emission.emission_count,
        source_final_closeout_gate: final_closeout.gate,
        source_final_closeout_entry_count: final_closeout.final_closeout_entry_count,
        entrypoint_binding_count: entrypoint_bindings.len(),
        guardrail_check_count: guardrail_checks.len(),
        dry_run_decision_count: dry_run_decisions.len(),
        required_prior_gate_count: required_prior_gates.len(),
        entrypoint_bindings,
        guardrail_checks,
        dry_run_decisions,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_RECOMMENDED_NEXT_GATE,
        scheduler_admission_dry_run_present: true,
        blocking_guardrail_dry_run_attached: true,
        pre_entrypoint_hook_contract_ready: true,
        live_blocking_enforcement_enabled: false,
        runtime_interception_enabled: false,
        work_graph_event_persistence_enabled: false,
        ready_for_work_graph_shadow_event_store_readback: true,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_bindings()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointBindingPreview> {
    let check_ids =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_check_ids();
    let trace_fields = vec![
        "traceId",
        "spanId",
        "parentSpanId",
        "guardrailId",
        "evidenceRef",
        "payloadHash",
    ];

    vec![
        entrypoint_binding(
            "spawn_agent_blocking_guardrail_dry_run",
            "multi_agent_v2_thread_spawn",
            "spawn_agent",
            "before agent_control.spawn_agent_with_metadata",
            check_ids.clone(),
            trace_fields.clone(),
        ),
        entrypoint_binding(
            "spawn_agents_on_csv_blocking_guardrail_dry_run",
            "agent_jobs_batch_workers",
            "spawn_agents_on_csv",
            "before CSV fanout creates or runs agent job items",
            check_ids.clone(),
            trace_fields.clone(),
        ),
        entrypoint_binding(
            "task_board_claim_blocking_guardrail_dry_run",
            "hepta_runtime_task_board",
            "task_board_claim",
            "before task board claim acquires or refreshes a lease",
            check_ids.clone(),
            trace_fields.clone(),
        ),
        entrypoint_binding(
            "worker_task_run_blocking_guardrail_dry_run",
            "hepta_runtime_worker_tasks",
            "worker_task_run",
            "before worker task starts command, tool, or agent work",
            check_ids,
            trace_fields,
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_checks()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunCheckPreview> {
    vec![
        check("dependencies_terminal_ready", "scheduler_admission"),
        check("lane_lease_available_and_owned", "scheduler_admission"),
        check(
            "approval_authority_present_when_required",
            "scheduler_admission",
        ),
        check("idempotency_replay_window_clear", "scheduler_admission"),
        check("budget_and_timeout_available", "scheduler_admission"),
        check(
            "task_result_contract_preview_present",
            "task_result_envelope",
        ),
        check("side_effect_boundary_locked", "scheduler_admission"),
        check("trace_guardrail_span_present", "trace_guardrail"),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_decisions()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunDecisionPreview> {
    vec![
        decision(
            "spawn_agent",
            "deny_live_allow_report_only",
            "spawn_agent would require blocking guardrail approval before live spawn",
            "trace-blocking-dry-run-spawn-agent-001",
        ),
        decision(
            "spawn_agents_on_csv",
            "deny_live_allow_report_only",
            "CSV fanout would require lease, budget, idempotency, and TaskResult evidence before live execution",
            "trace-blocking-dry-run-agent-jobs-001",
        ),
        decision(
            "task_board_claim",
            "deny_live_allow_report_only",
            "task board claim would require owned lease and dependency readback before live claim",
            "trace-blocking-dry-run-task-board-001",
        ),
        decision(
            "worker_task_run",
            "deny_live_allow_report_only",
            "worker task run would require side-effect boundary and guardrail span before live work",
            "trace-blocking-dry-run-worker-task-001",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
        WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            live_blocking_hook_installed: false,
            lease_acquired: false,
            work_started: false,
            config_written: false,
            feature_flag_mutated: false,
            canary_traffic_routed: false,
            operator_review_requested: false,
            approval_recorded: false,
            replay_executed: false,
            rollback_executed: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn entrypoint_binding(
    id: &'static str,
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
    hook_position: &'static str,
    applied_check_ids: Vec<&'static str>,
    required_trace_fields: Vec<&'static str>,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointBindingPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointBindingPreview {
        id,
        source_surface_id,
        entrypoint_id,
        hook_position,
        dry_run_decision: "deny_live_allow_report_only",
        applied_check_ids,
        required_trace_fields,
        would_block_if_live: true,
        dry_run_allows_current_runtime_to_continue: true,
        live_blocking_enabled: false,
    }
}

fn check(
    id: &'static str,
    source: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunCheckPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunCheckPreview {
        id,
        source,
        blocks_live_execution: true,
        dry_run_explanation_required: true,
    }
}

fn decision(
    entrypoint_id: &'static str,
    outcome: &'static str,
    reason: &'static str,
    trace_id: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunDecisionPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunDecisionPreview {
        entrypoint_id,
        outcome,
        reason,
        trace_id,
        allow_current_runtime_to_continue: true,
        block_live_execution: true,
    }
}

fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_check_ids()
-> Vec<&'static str> {
    work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_checks()
        .iter()
        .map(|check| check.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_guardrail_blocking_dry_run_derives_from_existing_priors() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report();

        assert_eq!(
            report.source_scheduler_gate,
            WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE
        );
        assert_eq!(report.source_scheduler_entrypoint_count, 4);
        assert_eq!(report.source_scheduler_check_count, 7);
        assert_eq!(
            report.source_trace_guardrail_gate,
            WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE
        );
        assert_eq!(report.source_trace_span_count, 9);
        assert_eq!(report.source_blocking_guardrail_count, 6);
        assert_eq!(
            report.source_entrypoint_emission_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE
        );
        assert_eq!(report.source_emission_count, 2);
        assert_eq!(
            report.source_final_closeout_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_GATE
        );
        assert_eq!(report.source_final_closeout_entry_count, 8);
    }

    #[test]
    fn scheduler_guardrail_blocking_dry_run_covers_requested_entrypoints() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report();
        let entrypoint_ids = report
            .entrypoint_bindings
            .iter()
            .map(|entrypoint| entrypoint.entrypoint_id)
            .collect::<Vec<_>>();

        assert_eq!(
            entrypoint_ids,
            [
                "spawn_agent",
                "spawn_agents_on_csv",
                "task_board_claim",
                "worker_task_run",
            ]
        );
        assert_eq!(report.entrypoint_binding_count, 4);
        assert_eq!(report.guardrail_check_count, 8);
        assert_eq!(report.dry_run_decision_count, 4);
        assert!(report.entrypoint_bindings.iter().all(|entrypoint| {
            entrypoint.would_block_if_live
                && entrypoint.dry_run_allows_current_runtime_to_continue
                && !entrypoint.live_blocking_enabled
                && entrypoint.applied_check_ids.len() == 8
                && entrypoint.required_trace_fields.len() == 6
        }));
    }

    #[test]
    fn scheduler_guardrail_blocking_dry_run_keeps_runtime_non_mutating() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report();

        assert_eq!(report.required_prior_gate_count, 4);
        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
                WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
            ]
        );
        assert!(report.scheduler_admission_dry_run_present);
        assert!(report.blocking_guardrail_dry_run_attached);
        assert!(report.pre_entrypoint_hook_contract_ready);
        assert!(!report.live_blocking_enforcement_enabled);
        assert!(!report.runtime_interception_enabled);
        assert!(!report.work_graph_event_persistence_enabled);
        assert!(report.ready_for_work_graph_shadow_event_store_readback);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn scheduler_guardrail_blocking_dry_run_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointSideEffects::none(
            )
        );
    }
}
