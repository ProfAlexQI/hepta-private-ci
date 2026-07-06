use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report,
};
use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_GATE,
    hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_entrypoint_gate: &'static str,
    pub source_entrypoint_binding_count: usize,
    pub source_guardrail_check_count: usize,
    pub source_dry_run_decision_count: usize,
    pub source_entrypoint_required_prior_gate_count: usize,
    pub source_terminal_no_execution_final_closeout_gate: &'static str,
    pub source_terminal_no_execution_final_closeout_entry_count: usize,
    pub source_terminal_no_execution_final_closeout_blocker_count: usize,
    pub source_terminal_no_execution_final_closeout_required_prior_gate_count: usize,
    pub hardened_entrypoint_count: usize,
    pub hardening_check_count: usize,
    pub hardening_decision_count: usize,
    pub hardening_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub hardened_entrypoints:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBindingPreview>,
    pub hardening_checks:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningCheckPreview>,
    pub hardening_decisions:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningDecisionPreview>,
    pub hardening_blockers:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub scheduler_guardrail_entrypoint_dry_run_present: bool,
    pub terminal_no_execution_final_closeout_present: bool,
    pub deny_live_allow_report_only_hardened: bool,
    pub pre_entrypoint_hook_contract_hardened: bool,
    pub deterministic_decision_key_ready: bool,
    pub trace_evidence_contract_ready: bool,
    pub shadow_event_join_ready: bool,
    pub live_blocking_enforcement_enabled: bool,
    pub runtime_interception_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub work_graph_event_persistence_enabled: bool,
    pub ready_for_hardening_readback: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBindingPreview {
    pub id: &'static str,
    pub source_entrypoint_binding_id: &'static str,
    pub entrypoint_id: &'static str,
    pub hardened_hook_position: &'static str,
    pub deterministic_decision_key: &'static str,
    pub dry_run_outcome: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub required_non_live_guards: Vec<&'static str>,
    pub would_block_if_live: bool,
    pub report_only_allows_current_runtime: bool,
    pub live_blocking_enabled: bool,
    pub runtime_interception_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningCheckPreview {
    pub id: &'static str,
    pub source: &'static str,
    pub live_blocking_condition: &'static str,
    pub hardening_requirement: &'static str,
    pub blocks_live_execution: bool,
    pub dry_run_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningDecisionPreview {
    pub entrypoint_id: &'static str,
    pub outcome: &'static str,
    pub explanation: &'static str,
    pub trace_id: &'static str,
    pub deterministic_decision_key: &'static str,
    pub allow_current_runtime_to_continue: bool,
    pub block_live_execution: bool,
    pub decision_recorded: bool,
    pub decision_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningSideEffects
{
    pub filesystem_written: bool,
    pub hardening_decision_recorded: bool,
    pub hardening_decision_persisted: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub live_blocking_hook_installed: bool,
    pub runtime_interception_enabled: bool,
    pub lease_acquired: bool,
    pub work_started: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub canary_traffic_routed: bool,
    pub operator_review_requested: bool,
    pub approval_recorded: bool,
    pub replay_executed: bool,
    pub replay_diff_recorded: bool,
    pub replay_diff_persisted: bool,
    pub rollback_executed: bool,
    pub idempotency_index_mutated: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReport {
    let entrypoint =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report();
    let terminal_closeout =
        hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report();
    let hardened_entrypoints =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardened_bindings();
    let hardening_checks =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_checks();
    let hardening_decisions =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_decisions();
    let hardening_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_required_prior_gates();

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_SCHEMA_VERSION,
        preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_report_only",
        source_entrypoint_gate: entrypoint.gate,
        source_entrypoint_binding_count: entrypoint.entrypoint_binding_count,
        source_guardrail_check_count: entrypoint.guardrail_check_count,
        source_dry_run_decision_count: entrypoint.dry_run_decision_count,
        source_entrypoint_required_prior_gate_count: entrypoint.required_prior_gate_count,
        source_terminal_no_execution_final_closeout_gate: terminal_closeout.gate,
        source_terminal_no_execution_final_closeout_entry_count:
            terminal_closeout.final_closeout_entry_count,
        source_terminal_no_execution_final_closeout_blocker_count:
            terminal_closeout.final_closeout_blocker_count,
        source_terminal_no_execution_final_closeout_required_prior_gate_count:
            terminal_closeout.required_prior_gate_count,
        hardened_entrypoint_count: hardened_entrypoints.len(),
        hardening_check_count: hardening_checks.len(),
        hardening_decision_count: hardening_decisions.len(),
        hardening_blocker_count: hardening_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        hardened_entrypoints,
        hardening_checks,
        hardening_decisions,
        hardening_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_RECOMMENDED_NEXT_GATE,
        scheduler_guardrail_entrypoint_dry_run_present: true,
        terminal_no_execution_final_closeout_present: true,
        deny_live_allow_report_only_hardened: true,
        pre_entrypoint_hook_contract_hardened: true,
        deterministic_decision_key_ready: true,
        trace_evidence_contract_ready: true,
        shadow_event_join_ready: true,
        live_blocking_enforcement_enabled: false,
        runtime_interception_enabled: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        work_graph_event_persistence_enabled: false,
        ready_for_hardening_readback: true,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardened_bindings()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBindingPreview> {
    vec![
        hardened_binding(
            "spawn_agent_hardened_dry_run_contract",
            "spawn_agent_blocking_guardrail_dry_run",
            "spawn_agent",
            "before agent_control.spawn_agent_with_metadata",
            "scheduler_guardrail.spawn_agent.v1",
        ),
        hardened_binding(
            "spawn_agents_on_csv_hardened_dry_run_contract",
            "spawn_agents_on_csv_blocking_guardrail_dry_run",
            "spawn_agents_on_csv",
            "before CSV fanout creates or runs agent job items",
            "scheduler_guardrail.spawn_agents_on_csv.v1",
        ),
        hardened_binding(
            "task_board_claim_hardened_dry_run_contract",
            "task_board_claim_blocking_guardrail_dry_run",
            "task_board_claim",
            "before task board claim acquires or refreshes a lease",
            "scheduler_guardrail.task_board_claim.v1",
        ),
        hardened_binding(
            "worker_task_run_hardened_dry_run_contract",
            "worker_task_run_blocking_guardrail_dry_run",
            "worker_task_run",
            "before worker task starts command, tool, or agent work",
            "scheduler_guardrail.worker_task_run.v1",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_checks()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningCheckPreview> {
    vec![
        hardening_check(
            "dependencies_readback_required",
            "scheduler_admission",
            "dependencies_not_terminal",
            "terminal dependency readback must be attached before live entrypoint execution",
        ),
        hardening_check(
            "lane_lease_snapshot_required",
            "scheduler_admission",
            "lane_lease_missing_or_not_owned",
            "lease ownership snapshot must be part of dry-run explanation",
        ),
        hardening_check(
            "approval_authority_snapshot_required",
            "scheduler_admission",
            "approval_authority_missing_when_required",
            "operator or policy approval authority must be named before live execution",
        ),
        hardening_check(
            "idempotency_decision_key_required",
            "scheduler_admission",
            "idempotency_key_missing_or_unstable",
            "deterministic decision key must suppress duplicate live work",
        ),
        hardening_check(
            "budget_timeout_snapshot_required",
            "scheduler_admission",
            "budget_or_timeout_unavailable",
            "budget and timeout snapshot must be present before live execution",
        ),
        hardening_check(
            "task_result_envelope_preview_required",
            "task_result_envelope",
            "task_result_contract_missing",
            "TaskResultEnvelope preview must exist before live entrypoint work",
        ),
        hardening_check(
            "side_effect_boundary_class_required",
            "scheduler_admission",
            "side_effect_boundary_unclassified",
            "side-effect class must remain locked and visible in dry-run evidence",
        ),
        hardening_check(
            "trace_guardrail_span_required",
            "trace_guardrail",
            "blocking_guardrail_span_missing",
            "traceId/spanId/guardrailId/evidenceRef/payloadHash must be linked",
        ),
        hardening_check(
            "shadow_event_join_preview_required",
            "work_graph_shadow_event_store",
            "shadow_event_join_missing",
            "entrypoint dry-run must join a redacted shadow event preview",
        ),
        hardening_check(
            "replay_diff_terminal_no_execution_closeout_required",
            "work_graph_shadow_event_store",
            "replay_diff_no_execution_branch_not_closed",
            "replay/diff no-execution final closeout must precede live hardening",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_decisions()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningDecisionPreview> {
    vec![
        hardening_decision(
            "spawn_agent",
            "spawn_agent remains deny-live until deterministic decision key, guardrail span, and shadow-event join are recordable",
            "trace-hardened-blocking-dry-run-spawn-agent-001",
            "scheduler_guardrail.spawn_agent.v1",
        ),
        hardening_decision(
            "spawn_agents_on_csv",
            "CSV fanout remains deny-live until lease, budget, idempotency, and TaskResult evidence are recordable",
            "trace-hardened-blocking-dry-run-agent-jobs-001",
            "scheduler_guardrail.spawn_agents_on_csv.v1",
        ),
        hardening_decision(
            "task_board_claim",
            "task board claim remains deny-live until lease and dependency readback evidence are recordable",
            "trace-hardened-blocking-dry-run-task-board-001",
            "scheduler_guardrail.task_board_claim.v1",
        ),
        hardening_decision(
            "worker_task_run",
            "worker task run remains deny-live until side-effect boundary and guardrail span evidence are recordable",
            "trace-hardened-blocking-dry-run-worker-task-001",
            "scheduler_guardrail.worker_task_run.v1",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_blockers()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBlockerPreview> {
    vec![
        blocker(
            "live_blocking_hook_install_blocked",
            "install_live_blocking_hook",
        ),
        blocker(
            "runtime_interception_blocked",
            "enable_runtime_interception",
        ),
        blocker(
            "scheduler_admission_enforcement_blocked",
            "enforce_scheduler_admission",
        ),
        blocker(
            "guardrail_enforcement_blocked",
            "enable_guardrail_enforcement",
        ),
        blocker(
            "hardening_decision_record_blocked",
            "record_hardening_decision",
        ),
        blocker(
            "hardening_decision_persistence_blocked",
            "persist_hardening_decision",
        ),
        blocker(
            "work_graph_event_persistence_blocked",
            "persist_work_graph_event",
        ),
        blocker(
            "projection_index_persistence_blocked",
            "persist_projection_index",
        ),
        blocker("lease_acquisition_blocked", "acquire_lane_lease"),
        blocker("work_start_blocked", "start_entrypoint_work"),
        blocker("agent_spawn_blocked", "spawn_agent"),
        blocker("model_invocation_blocked", "invoke_model"),
        blocker("external_send_blocked", "send_external_message"),
        blocker("replay_diff_recording_blocked", "record_replay_diff"),
        blocker("replay_execution_blocked", "execute_replay"),
        blocker("rollback_execution_blocked", "execute_rollback"),
        blocker("idempotency_mutation_blocked", "mutate_idempotency_index"),
        blocker("config_write_blocked", "write_config"),
        blocker("feature_flag_mutation_blocked", "mutate_feature_flag"),
        blocker("canary_traffic_blocked", "route_canary_traffic"),
        blocker("operator_review_request_blocked", "request_operator_review"),
        blocker("approval_recording_blocked", "record_operator_approval"),
        blocker("live_cutover_blocked", "perform_live_cutover"),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_required_prior_gates()
-> Vec<&'static str> {
    let entrypoint =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report();
    let terminal_closeout =
        hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_GATE,
    ];
    required_prior_gates.extend(entrypoint.required_prior_gates.iter().copied());
    required_prior_gates.extend(terminal_closeout.required_prior_gates.iter().copied());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            hardening_decision_recorded: false,
            hardening_decision_persisted: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            projection_index_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            live_blocking_hook_installed: false,
            runtime_interception_enabled: false,
            lease_acquired: false,
            work_started: false,
            config_written: false,
            feature_flag_mutated: false,
            canary_traffic_routed: false,
            operator_review_requested: false,
            approval_recorded: false,
            replay_executed: false,
            replay_diff_recorded: false,
            replay_diff_persisted: false,
            rollback_executed: false,
            idempotency_index_mutated: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn hardened_binding(
    id: &'static str,
    source_entrypoint_binding_id: &'static str,
    entrypoint_id: &'static str,
    hardened_hook_position: &'static str,
    deterministic_decision_key: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBindingPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBindingPreview {
        id,
        source_entrypoint_binding_id,
        entrypoint_id,
        hardened_hook_position,
        deterministic_decision_key,
        dry_run_outcome: "deny_live_allow_report_only_hardened",
        required_evidence_fields: hardened_evidence_fields(),
        required_non_live_guards: hardened_non_live_guards(),
        would_block_if_live: true,
        report_only_allows_current_runtime: true,
        live_blocking_enabled: false,
        runtime_interception_enabled: false,
    }
}

fn hardening_check(
    id: &'static str,
    source: &'static str,
    live_blocking_condition: &'static str,
    hardening_requirement: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningCheckPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningCheckPreview {
        id,
        source,
        live_blocking_condition,
        hardening_requirement,
        blocks_live_execution: true,
        dry_run_only: true,
    }
}

fn hardening_decision(
    entrypoint_id: &'static str,
    explanation: &'static str,
    trace_id: &'static str,
    deterministic_decision_key: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningDecisionPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningDecisionPreview {
        entrypoint_id,
        outcome: "deny_live_allow_report_only_hardened",
        explanation,
        trace_id,
        deterministic_decision_key,
        allow_current_runtime_to_continue: true,
        block_live_execution: true,
        decision_recorded: false,
        decision_persisted: false,
    }
}

fn blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBlockerPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "scheduler/guardrail blocking dry-run entrypoint hardening cannot authorize this action",
    }
}

fn hardened_evidence_fields() -> Vec<&'static str> {
    vec![
        "traceId",
        "spanId",
        "parentSpanId",
        "entrypointId",
        "decisionKey",
        "guardrailId",
        "evidenceRef",
        "payloadHash",
        "sideEffectClass",
    ]
}

fn hardened_non_live_guards() -> Vec<&'static str> {
    vec![
        "live_blocking_enabled=false",
        "runtime_interception_enabled=false",
        "scheduler_admission_enforced=false",
        "guardrail_enforcement_enabled=false",
        "work_graph_event_persisted=false",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_guardrail_entrypoint_hardening_derives_from_entrypoint_and_closeout_priors() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_report();

        assert_eq!(
            report.source_entrypoint_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE
        );
        assert_eq!(report.source_entrypoint_binding_count, 4);
        assert_eq!(report.source_guardrail_check_count, 8);
        assert_eq!(report.source_dry_run_decision_count, 4);
        assert_eq!(report.source_entrypoint_required_prior_gate_count, 4);
        assert_eq!(
            report.source_terminal_no_execution_final_closeout_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_GATE
        );
        assert_eq!(
            report.source_terminal_no_execution_final_closeout_entry_count,
            9
        );
        assert_eq!(
            report.source_terminal_no_execution_final_closeout_blocker_count,
            26
        );
        assert_eq!(
            report.source_terminal_no_execution_final_closeout_required_prior_gate_count,
            5
        );
        assert_eq!(report.hardened_entrypoint_count, 4);
        assert_eq!(report.hardening_check_count, 10);
        assert_eq!(report.hardening_decision_count, 4);
        assert_eq!(report.hardening_blocker_count, 23);
        assert_eq!(report.required_prior_gate_count, 11);
    }

    #[test]
    fn scheduler_guardrail_entrypoint_hardening_covers_four_entrypoints() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_report();
        let entrypoint_ids = report
            .hardened_entrypoints
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
        assert!(report.hardened_entrypoints.iter().all(|entrypoint| {
            entrypoint.dry_run_outcome == "deny_live_allow_report_only_hardened"
                && entrypoint.required_evidence_fields.len() == 9
                && entrypoint.required_non_live_guards.len() == 5
                && entrypoint.would_block_if_live
                && entrypoint.report_only_allows_current_runtime
                && !entrypoint.live_blocking_enabled
                && !entrypoint.runtime_interception_enabled
        }));
    }

    #[test]
    fn scheduler_guardrail_entrypoint_hardening_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_report();

        assert!(report.scheduler_guardrail_entrypoint_dry_run_present);
        assert!(report.terminal_no_execution_final_closeout_present);
        assert!(report.deny_live_allow_report_only_hardened);
        assert!(report.pre_entrypoint_hook_contract_hardened);
        assert!(report.deterministic_decision_key_ready);
        assert!(report.trace_evidence_contract_ready);
        assert!(report.shadow_event_join_ready);
        assert!(report.hardening_checks.iter().all(|check| {
            check.blocks_live_execution
                && check.dry_run_only
                && !check.hardening_requirement.is_empty()
        }));
        assert!(report.hardening_decisions.iter().all(|decision| {
            decision.outcome == "deny_live_allow_report_only_hardened"
                && decision.allow_current_runtime_to_continue
                && decision.block_live_execution
                && !decision.decision_recorded
                && !decision.decision_persisted
        }));
        assert!(
            report
                .hardening_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(!report.live_blocking_enforcement_enabled);
        assert!(!report.runtime_interception_enabled);
        assert!(!report.scheduler_admission_enforced);
        assert!(!report.guardrail_enforcement_enabled);
        assert!(!report.work_graph_event_persistence_enabled);
        assert!(report.ready_for_hardening_readback);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn scheduler_guardrail_entrypoint_hardening_links_priors_and_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_GATE,
                "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate",
                "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
                "hepta_work_graph_trace_guardrail_span_report_only_gate",
                "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningSideEffects::none()
        );
    }
}
