use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_TERMINAL_NO_ENFORCEMENT_FINAL_CLOSEOUT_GATE,
    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningTerminalNoEnforcementFinalCloseoutSideEffects,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_final_closeout_gate: &'static str,
    pub source_final_closeout_entry_count: usize,
    pub source_final_closeout_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub source_final_closeout_ready: bool,
    pub source_final_closeout_no_persistence_confirmed: bool,
    pub source_final_closeout_no_live_confirmed: bool,
    pub source_final_closeout_ready_for_live_attachment_matrix: bool,
    pub entrypoint_count: usize,
    pub precondition_check_count: usize,
    pub precondition_satisfied_count: usize,
    pub precondition_unsatisfied_count: usize,
    pub blocking_precondition_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub entrypoints:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionEntrypointPreview>,
    pub precondition_checks:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionCheckPreview>,
    pub blockers:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub matrix_mode: &'static str,
    pub matrix_visible: bool,
    pub matrix_recorded: bool,
    pub matrix_persisted: bool,
    pub matrix_authoritative: bool,
    pub matrix_accepted: bool,
    pub entrypoints_complete: bool,
    pub precondition_matrix_complete: bool,
    pub blocking_preconditions_complete: bool,
    pub blockers_complete: bool,
    pub live_attachment_precondition_matrix_preconditions_complete: bool,
    pub live_attachment_allowed: bool,
    pub live_blocking_hook_install_allowed: bool,
    pub runtime_interception_allowed: bool,
    pub scheduler_admission_enforcement_allowed: bool,
    pub guardrail_enforcement_allowed: bool,
    pub work_graph_event_persistence_allowed: bool,
    pub projection_persistence_allowed: bool,
    pub lease_acquisition_allowed: bool,
    pub work_start_allowed: bool,
    pub agent_spawn_allowed: bool,
    pub model_invocation_allowed: bool,
    pub external_send_allowed: bool,
    pub live_task_result_emission_allowed: bool,
    pub hardening_decision_recording_allowed: bool,
    pub hardening_decision_persistence_allowed: bool,
    pub readback_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub replay_diff_recording_allowed: bool,
    pub replay_diff_persistence_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub idempotency_mutation_allowed: bool,
    pub config_write_allowed: bool,
    pub feature_flag_mutation_allowed: bool,
    pub canary_traffic_allowed: bool,
    pub operator_review_request_allowed: bool,
    pub approval_recording_allowed: bool,
    pub live_cutover_allowed: bool,
    pub ready_for_denial_readback: bool,
    pub ready_for_live_attachment: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionEntrypointPreview
{
    pub id: &'static str,
    pub surface: &'static str,
    pub live_attachment_candidate: bool,
    pub live_attachment_allowed: bool,
    pub report_only: bool,
    pub runtime_interception_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionCheckPreview
{
    pub id: &'static str,
    pub category: &'static str,
    pub required: bool,
    pub satisfied: bool,
    pub blocking: bool,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixSideEffects
{
    pub filesystem_written: bool,
    pub matrix_recorded: bool,
    pub matrix_persisted: bool,
    pub matrix_accepted: bool,
    pub live_attachment_enabled: bool,
    pub live_blocking_hook_installed: bool,
    pub runtime_interception_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub work_graph_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub lease_acquired: bool,
    pub work_started: bool,
    pub hardening_decision_recorded: bool,
    pub hardening_decision_persisted: bool,
    pub live_task_result_emitted: bool,
    pub readback_executed: bool,
    pub replay_executed: bool,
    pub replay_diff_recorded: bool,
    pub replay_diff_persisted: bool,
    pub rollback_executed: bool,
    pub idempotency_index_mutated: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub canary_traffic_routed: bool,
    pub operator_review_requested: bool,
    pub approval_recorded: bool,
    pub live_cutover_performed: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_report();
    let entrypoints =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_entrypoints();
    let precondition_checks =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_checks();
    let blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_TERMINAL_NO_ENFORCEMENT_FINAL_CLOSEOUT_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    let precondition_satisfied_count = precondition_checks
        .iter()
        .filter(|check| check.satisfied)
        .count();
    let precondition_unsatisfied_count = precondition_checks.len() - precondition_satisfied_count;
    let blocking_precondition_count = precondition_checks
        .iter()
        .filter(|check| check.blocking)
        .count();
    let source_final_closeout_no_persistence_confirmed =
        source.source_non_persistence_readback_no_persistence_confirmed
            && source.terminal_no_enforcement_final_closeout_preconditions_complete
            && !source.final_closeout_recorded
            && !source.final_closeout_persisted
            && !source.final_closeout_authoritative
            && !source.final_closeout_accepted
            && !source.source_audit_index_persisted
            && !source.source_readback_persisted
            && !source.hardening_decision_recording_allowed
            && !source.hardening_decision_persistence_allowed
            && !source.work_graph_event_persistence_allowed
            && !source.projection_persistence_allowed
            && source.side_effects
                == WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningTerminalNoEnforcementFinalCloseoutSideEffects::none();
    let source_final_closeout_no_live_confirmed = source
        .ready_for_live_attachment_precondition_matrix
        && !source.live_blocking_enforcement_allowed
        && !source.runtime_interception_allowed
        && !source.scheduler_admission_enforcement_allowed
        && !source.guardrail_enforcement_allowed
        && !source.lease_acquisition_allowed
        && !source.work_start_allowed
        && !source.agent_spawn_allowed
        && !source.model_invocation_allowed
        && !source.external_send_allowed
        && !source.replay_execution_allowed
        && !source.replay_diff_recording_allowed
        && !source.replay_diff_persistence_allowed
        && !source.rollback_execution_allowed
        && !source.idempotency_mutation_allowed
        && !source.config_write_allowed
        && !source.feature_flag_mutation_allowed
        && !source.canary_traffic_allowed
        && !source.operator_review_request_allowed
        && !source.approval_recording_allowed
        && !source.live_cutover_allowed
        && !source.ready_for_live_execution
        && source_final_closeout_no_persistence_confirmed;
    let source_final_closeout_ready = source.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_TERMINAL_NO_ENFORCEMENT_FINAL_CLOSEOUT_GATE
        && source.source_non_persistence_readback_ready
        && source.source_non_persistence_readback_no_persistence_confirmed
        && source.source_non_persistence_readback_no_live_confirmed
        && source.source_non_persistence_readback_ready_for_terminal_closeout
        && source.terminal_no_enforcement_branch_closed
        && source.final_closeout_scope_visible_only_complete
        && source.final_closeout_entries_complete
        && source.final_closeout_blockers_complete
        && source.terminal_no_enforcement_final_closeout_preconditions_complete
        && source.final_closeout_entry_count == 9
        && source.final_closeout_blocker_count == 36
        && source.required_prior_gate_count == 15
        && source_final_closeout_no_live_confirmed;
    let source_final_closeout_ready_for_live_attachment_matrix =
        source_final_closeout_ready && source.ready_for_live_attachment_precondition_matrix;
    let entrypoints_complete = entrypoints.len() == 4
        && entrypoints.iter().all(|entry| {
            entry.live_attachment_candidate
                && entry.report_only
                && !entry.live_attachment_allowed
                && !entry.runtime_interception_allowed
        });
    let precondition_matrix_complete = precondition_checks.len() == 14
        && precondition_satisfied_count == 4
        && precondition_unsatisfied_count == 10
        && precondition_checks.iter().all(|check| check.required);
    let blocking_preconditions_complete = blocking_precondition_count == 10
        && precondition_checks
            .iter()
            .filter(|check| check.blocking)
            .all(|check| !check.satisfied);
    let blockers_complete = blockers.len() == 33 && blockers.iter().all(|blocker| blocker.blocked);
    let live_attachment_precondition_matrix_preconditions_complete =
        source_final_closeout_ready_for_live_attachment_matrix
            && entrypoints_complete
            && precondition_matrix_complete
            && blocking_preconditions_complete
            && blockers_complete;

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_deny_only",
        source_final_closeout_gate: source.gate,
        source_final_closeout_entry_count: source.final_closeout_entry_count,
        source_final_closeout_blocker_count: source.final_closeout_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        source_final_closeout_ready,
        source_final_closeout_no_persistence_confirmed,
        source_final_closeout_no_live_confirmed,
        source_final_closeout_ready_for_live_attachment_matrix,
        entrypoint_count: entrypoints.len(),
        precondition_check_count: precondition_checks.len(),
        precondition_satisfied_count,
        precondition_unsatisfied_count,
        blocking_precondition_count,
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        entrypoints,
        precondition_checks,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_RECOMMENDED_NEXT_GATE,
        matrix_mode:
            "deny_live_attachment_until_explicit_scheduler_guardrail_enforcement_authorization",
        matrix_visible: true,
        matrix_recorded: false,
        matrix_persisted: false,
        matrix_authoritative: false,
        matrix_accepted: false,
        entrypoints_complete,
        precondition_matrix_complete,
        blocking_preconditions_complete,
        blockers_complete,
        live_attachment_precondition_matrix_preconditions_complete,
        live_attachment_allowed: false,
        live_blocking_hook_install_allowed: false,
        runtime_interception_allowed: false,
        scheduler_admission_enforcement_allowed: false,
        guardrail_enforcement_allowed: false,
        work_graph_event_persistence_allowed: false,
        projection_persistence_allowed: false,
        lease_acquisition_allowed: false,
        work_start_allowed: false,
        agent_spawn_allowed: false,
        model_invocation_allowed: false,
        external_send_allowed: false,
        live_task_result_emission_allowed: false,
        hardening_decision_recording_allowed: false,
        hardening_decision_persistence_allowed: false,
        readback_execution_allowed: false,
        replay_execution_allowed: false,
        replay_diff_recording_allowed: false,
        replay_diff_persistence_allowed: false,
        rollback_execution_allowed: false,
        idempotency_mutation_allowed: false,
        config_write_allowed: false,
        feature_flag_mutation_allowed: false,
        canary_traffic_allowed: false,
        operator_review_request_allowed: false,
        approval_recording_allowed: false,
        live_cutover_allowed: false,
        ready_for_denial_readback: live_attachment_precondition_matrix_preconditions_complete,
        ready_for_live_attachment: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_entrypoints()
-> Vec<
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionEntrypointPreview,
> {
    vec![
        entrypoint("spawn_agent", "multi_agents_v2.spawn_agent"),
        entrypoint("spawn_agents_on_csv", "agent_jobs.spawn_agents_on_csv"),
        entrypoint("task_board_claim", "task_board.claim"),
        entrypoint("worker_task_run", "worker_tasks.run_worker_task"),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_checks()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionCheckPreview> {
    vec![
        check(
            "terminal_no_enforcement_final_closeout_ready",
            "source_evidence",
            true,
            false,
            "terminal no-enforcement closeout is visible as report-only evidence",
        ),
        check(
            "entrypoint_scope_inventory_visible",
            "entrypoint_scope",
            true,
            false,
            "four entrypoint surfaces are covered by the dry-run hardening chain",
        ),
        check(
            "deterministic_decision_keys_visible",
            "decision_key",
            true,
            false,
            "hardened dry-run decisions have deterministic keys for later attachment",
        ),
        check(
            "trace_evidence_join_visible",
            "trace_evidence",
            true,
            false,
            "trace and evidence references are visible without being persisted",
        ),
        check(
            "live_blocking_hook_authorization_missing",
            "live_hook_boundary",
            false,
            true,
            "no authorization exists to install live blocking hooks",
        ),
        check(
            "runtime_interception_authorization_missing",
            "runtime_boundary",
            false,
            true,
            "runtime interception remains explicitly disallowed",
        ),
        check(
            "scheduler_admission_enforcement_authorization_missing",
            "scheduler_boundary",
            false,
            true,
            "scheduler admission remains dry-run only",
        ),
        check(
            "guardrail_enforcement_authorization_missing",
            "guardrail_boundary",
            false,
            true,
            "guardrail spans remain report-only and cannot block live traffic",
        ),
        check(
            "work_graph_persistence_authorization_missing",
            "work_graph_persistence_boundary",
            false,
            true,
            "WorkGraph event and projection persistence remain disabled",
        ),
        check(
            "lease_and_work_start_authorization_missing",
            "lease_work_start_boundary",
            false,
            true,
            "lane leases and entrypoint work starts cannot be acquired from this matrix",
        ),
        check(
            "live_task_result_acceptance_missing",
            "task_result_boundary",
            false,
            true,
            "TaskResult emission remains report-only and not live-accepted",
        ),
        check(
            "replay_rollback_execution_authorization_missing",
            "replay_rollback_boundary",
            false,
            true,
            "replay, replay diff recording, rollback, and idempotency mutation remain disabled",
        ),
        check(
            "config_flag_traffic_authorization_missing",
            "config_flag_traffic_boundary",
            false,
            true,
            "config writes, feature flag mutation, and canary traffic remain disallowed",
        ),
        check(
            "operator_approval_live_cutover_authorization_missing",
            "operator_live_boundary",
            false,
            true,
            "operator review, approval recording, and live cutover remain absent",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_blockers()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionBlockerPreview>
{
    vec![
        blocker(
            "matrix_record_blocked",
            "record_live_attachment_precondition_matrix",
        ),
        blocker(
            "matrix_persistence_blocked",
            "persist_live_attachment_precondition_matrix",
        ),
        blocker(
            "matrix_acceptance_blocked",
            "accept_live_attachment_precondition_matrix",
        ),
        blocker(
            "live_attachment_enablement_blocked",
            "enable_live_attachment",
        ),
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
            "work_graph_event_persistence_blocked",
            "persist_work_graph_event",
        ),
        blocker(
            "projection_index_persistence_blocked",
            "persist_projection_index",
        ),
        blocker("lease_acquisition_blocked", "acquire_lane_lease"),
        blocker("work_start_blocked", "start_entrypoint_work"),
        blocker("spawn_agent_blocked", "spawn_agent"),
        blocker("spawn_agents_on_csv_blocked", "spawn_agents_on_csv"),
        blocker("task_board_claim_blocked", "claim_task_board_work"),
        blocker("worker_task_run_blocked", "run_worker_task"),
        blocker("model_invocation_blocked", "invoke_model"),
        blocker("external_send_blocked", "send_external_message"),
        blocker("live_task_result_emit_blocked", "emit_live_task_result"),
        blocker(
            "hardening_decision_record_blocked",
            "record_hardening_decision",
        ),
        blocker(
            "hardening_decision_persistence_blocked",
            "persist_hardening_decision",
        ),
        blocker("readback_execution_blocked", "execute_readback"),
        blocker("replay_execution_blocked", "execute_replay"),
        blocker("replay_diff_recording_blocked", "record_replay_diff"),
        blocker("replay_diff_persistence_blocked", "persist_replay_diff"),
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

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_required_prior_gates()
-> Vec<&'static str> {
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_TERMINAL_NO_ENFORCEMENT_FINAL_CLOSEOUT_GATE,
    ];
    required_prior_gates.extend(crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_required_prior_gates());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            matrix_recorded: false,
            matrix_persisted: false,
            matrix_accepted: false,
            live_attachment_enabled: false,
            live_blocking_hook_installed: false,
            runtime_interception_enabled: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            work_graph_event_persisted: false,
            projection_index_persisted: false,
            lease_acquired: false,
            work_started: false,
            hardening_decision_recorded: false,
            hardening_decision_persisted: false,
            live_task_result_emitted: false,
            readback_executed: false,
            replay_executed: false,
            replay_diff_recorded: false,
            replay_diff_persisted: false,
            rollback_executed: false,
            idempotency_index_mutated: false,
            config_written: false,
            feature_flag_mutated: false,
            canary_traffic_routed: false,
            operator_review_requested: false,
            approval_recorded: false,
            live_cutover_performed: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn entrypoint(
    id: &'static str,
    surface: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionEntrypointPreview
{
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionEntrypointPreview {
        id,
        surface,
        live_attachment_candidate: true,
        live_attachment_allowed: false,
        report_only: true,
        runtime_interception_allowed: false,
    }
}

fn check(
    id: &'static str,
    category: &'static str,
    satisfied: bool,
    blocking: bool,
    explanation: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionCheckPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionCheckPreview {
        id,
        category,
        required: true,
        satisfied,
        blocking,
        explanation,
    }
}

fn blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionBlockerPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "scheduler/guardrail live attachment precondition matrix cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_attachment_precondition_matrix_derives_from_terminal_closeout() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_report();

        assert_eq!(
            report.source_final_closeout_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_TERMINAL_NO_ENFORCEMENT_FINAL_CLOSEOUT_GATE
        );
        assert_eq!(report.source_final_closeout_entry_count, 9);
        assert_eq!(report.source_final_closeout_blocker_count, 36);
        assert_eq!(report.source_required_prior_gate_count, 15);
        assert!(report.source_final_closeout_ready);
        assert!(report.source_final_closeout_no_persistence_confirmed);
        assert!(report.source_final_closeout_no_live_confirmed);
        assert!(report.source_final_closeout_ready_for_live_attachment_matrix);
        assert_eq!(report.entrypoint_count, 4);
        assert_eq!(report.precondition_check_count, 14);
        assert_eq!(report.precondition_satisfied_count, 4);
        assert_eq!(report.precondition_unsatisfied_count, 10);
        assert_eq!(report.blocking_precondition_count, 10);
        assert_eq!(report.blocker_count, 33);
        assert_eq!(report.required_prior_gate_count, 16);
    }

    #[test]
    fn live_attachment_precondition_matrix_keeps_entrypoints_report_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_report();

        assert_eq!(
            report
                .entrypoints
                .iter()
                .map(|entry| entry.id)
                .collect::<Vec<_>>(),
            vec![
                "spawn_agent",
                "spawn_agents_on_csv",
                "task_board_claim",
                "worker_task_run",
            ]
        );
        assert!(report.matrix_visible);
        assert!(!report.matrix_recorded);
        assert!(!report.matrix_persisted);
        assert!(!report.matrix_authoritative);
        assert!(!report.matrix_accepted);
        assert!(report.entrypoints_complete);
        assert!(report.entrypoints.iter().all(|entry| {
            entry.live_attachment_candidate
                && entry.report_only
                && !entry.live_attachment_allowed
                && !entry.runtime_interception_allowed
        }));
    }

    #[test]
    fn live_attachment_precondition_matrix_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_report();

        assert!(
            report
                .precondition_checks
                .iter()
                .filter(|check| check.blocking)
                .all(|check| !check.satisfied)
        );
        assert!(report.precondition_matrix_complete);
        assert!(report.blocking_preconditions_complete);
        assert!(report.blockers_complete);
        assert!(report.live_attachment_precondition_matrix_preconditions_complete);
        assert!(report.blockers.iter().all(|blocker| blocker.blocked));
        assert!(!report.live_attachment_allowed);
        assert!(!report.live_blocking_hook_install_allowed);
        assert!(!report.runtime_interception_allowed);
        assert!(!report.scheduler_admission_enforcement_allowed);
        assert!(!report.guardrail_enforcement_allowed);
        assert!(!report.work_graph_event_persistence_allowed);
        assert!(!report.projection_persistence_allowed);
        assert!(!report.lease_acquisition_allowed);
        assert!(!report.work_start_allowed);
        assert!(!report.agent_spawn_allowed);
        assert!(!report.model_invocation_allowed);
        assert!(!report.external_send_allowed);
        assert!(!report.live_task_result_emission_allowed);
        assert!(!report.hardening_decision_recording_allowed);
        assert!(!report.hardening_decision_persistence_allowed);
        assert!(!report.readback_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.replay_diff_recording_allowed);
        assert!(!report.replay_diff_persistence_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.idempotency_mutation_allowed);
        assert!(!report.config_write_allowed);
        assert!(!report.feature_flag_mutation_allowed);
        assert!(!report.canary_traffic_allowed);
        assert!(!report.operator_review_request_allowed);
        assert!(!report.approval_recording_allowed);
        assert!(!report.live_cutover_allowed);
        assert!(report.ready_for_denial_readback);
        assert!(!report.ready_for_live_attachment);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn live_attachment_precondition_matrix_links_priors_and_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_TERMINAL_NO_ENFORCEMENT_FINAL_CLOSEOUT_GATE,
                "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate",
                "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate",
                "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate",
                "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate",
                "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate",
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
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixSideEffects::none()
        );
    }
}
