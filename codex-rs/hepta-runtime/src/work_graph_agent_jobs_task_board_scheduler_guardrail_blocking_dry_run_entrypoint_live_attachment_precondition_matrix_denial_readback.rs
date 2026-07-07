use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_GATE,
    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixSideEffects,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_matrix_gate: &'static str,
    pub source_entrypoint_count: usize,
    pub source_precondition_check_count: usize,
    pub source_blocking_precondition_count: usize,
    pub source_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub source_matrix_ready: bool,
    pub source_matrix_no_persistence_confirmed: bool,
    pub source_matrix_no_live_confirmed: bool,
    pub source_matrix_ready_for_denial_readback: bool,
    pub denial_readback_entry_count: usize,
    pub entrypoint_denial_readback_count: usize,
    pub denial_readback_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub denial_readback_scope:
        WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackScopePreview,
    pub denial_readback_entries:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackEntryPreview>,
    pub entrypoint_denial_readbacks:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixEntrypointDenialReadbackPreview>,
    pub denial_readback_blockers:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub denial_readback_visible: bool,
    pub denial_readback_recorded: bool,
    pub denial_readback_persisted: bool,
    pub denial_readback_authoritative: bool,
    pub denial_readback_accepted: bool,
    pub denial_readback_scope_visible_only_complete: bool,
    pub denial_readback_entries_complete: bool,
    pub entrypoint_denial_readbacks_complete: bool,
    pub denial_readback_blockers_complete: bool,
    pub denial_readback_preconditions_complete: bool,
    pub denial_readback_authorizes_live_attachment: bool,
    pub denial_readback_authorizes_live_blocking_hook: bool,
    pub denial_readback_authorizes_runtime_interception: bool,
    pub denial_readback_authorizes_scheduler_admission_enforcement: bool,
    pub denial_readback_authorizes_guardrail_enforcement: bool,
    pub denial_readback_authorizes_work_graph_persistence: bool,
    pub denial_readback_authorizes_lease_or_work_start: bool,
    pub denial_readback_authorizes_agent_model_or_external_send: bool,
    pub denial_readback_authorizes_live_task_result: bool,
    pub denial_readback_authorizes_replay_or_rollback: bool,
    pub denial_readback_authorizes_config_flag_or_traffic: bool,
    pub denial_readback_authorizes_operator_approval_or_live_cutover: bool,
    pub ready_for_denial_readback_audit_index: bool,
    pub ready_for_live_attachment: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub readback_mode: &'static str,
    pub stable_readback_key: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub accepted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackEntryPreview {
    pub id: &'static str,
    pub stable_readback_key: &'static str,
    pub observed_state: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub accepted: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixEntrypointDenialReadbackPreview
{
    pub entrypoint_id: &'static str,
    pub source_surface: &'static str,
    pub denial_readback_key: &'static str,
    pub live_attachment_allowed: bool,
    pub runtime_interception_allowed: bool,
    pub report_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackSideEffects
{
    pub filesystem_written: bool,
    pub denial_readback_recorded: bool,
    pub denial_readback_persisted: bool,
    pub denial_readback_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_report();
    let denial_readback_scope =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_scope();
    let denial_readback_entries =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_entries();
    let entrypoint_denial_readbacks =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_entrypoint_denial_readbacks();
    let denial_readback_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    let source_matrix_no_persistence_confirmed =
        source.source_final_closeout_no_persistence_confirmed
            && source.live_attachment_precondition_matrix_preconditions_complete
            && !source.matrix_recorded
            && !source.matrix_persisted
            && !source.matrix_authoritative
            && !source.matrix_accepted
            && !source.hardening_decision_recording_allowed
            && !source.hardening_decision_persistence_allowed
            && !source.work_graph_event_persistence_allowed
            && !source.projection_persistence_allowed
            && source.side_effects
                == WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixSideEffects::none();
    let source_matrix_no_live_confirmed = source.ready_for_denial_readback
        && !source.live_attachment_allowed
        && !source.live_blocking_hook_install_allowed
        && !source.runtime_interception_allowed
        && !source.scheduler_admission_enforcement_allowed
        && !source.guardrail_enforcement_allowed
        && !source.lease_acquisition_allowed
        && !source.work_start_allowed
        && !source.agent_spawn_allowed
        && !source.model_invocation_allowed
        && !source.external_send_allowed
        && !source.live_task_result_emission_allowed
        && !source.readback_execution_allowed
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
        && source_matrix_no_persistence_confirmed;
    let source_matrix_ready =
        source.gate
            == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_GATE
            && source.source_final_closeout_ready
            && source.source_final_closeout_no_persistence_confirmed
            && source.source_final_closeout_no_live_confirmed
            && source.source_final_closeout_ready_for_live_attachment_matrix
            && source.entrypoints_complete
            && source.precondition_matrix_complete
            && source.blocking_preconditions_complete
            && source.blockers_complete
            && source.live_attachment_precondition_matrix_preconditions_complete
            && source.entrypoint_count == 4
            && source.precondition_check_count == 14
            && source.blocking_precondition_count == 10
            && source.blocker_count == 33
            && source.required_prior_gate_count == 16
            && source_matrix_no_live_confirmed;
    let source_matrix_ready_for_denial_readback =
        source_matrix_ready && source.ready_for_denial_readback;
    let denial_readback_scope_visible_only_complete = denial_readback_scope.visible
        && !denial_readback_scope.recorded
        && !denial_readback_scope.persisted
        && !denial_readback_scope.authoritative
        && !denial_readback_scope.accepted;
    let denial_readback_entries_complete = denial_readback_entries.len() == 7
        && denial_readback_entries.iter().all(|entry| {
            entry.visible
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.accepted
                && !entry.mutation_allowed
        });
    let entrypoint_denial_readbacks_complete = entrypoint_denial_readbacks.len() == 4
        && entrypoint_denial_readbacks.iter().all(|entry| {
            entry.report_only
                && !entry.live_attachment_allowed
                && !entry.runtime_interception_allowed
        });
    let denial_readback_blockers_complete = denial_readback_blockers.len() == 36
        && denial_readback_blockers
            .iter()
            .all(|blocker| blocker.blocked);
    let denial_readback_preconditions_complete = source_matrix_ready_for_denial_readback
        && denial_readback_scope_visible_only_complete
        && denial_readback_entries_complete
        && entrypoint_denial_readbacks_complete
        && denial_readback_blockers_complete;

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_only",
        source_matrix_gate: source.gate,
        source_entrypoint_count: source.entrypoint_count,
        source_precondition_check_count: source.precondition_check_count,
        source_blocking_precondition_count: source.blocking_precondition_count,
        source_blocker_count: source.blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        source_matrix_ready,
        source_matrix_no_persistence_confirmed,
        source_matrix_no_live_confirmed,
        source_matrix_ready_for_denial_readback,
        denial_readback_entry_count: denial_readback_entries.len(),
        entrypoint_denial_readback_count: entrypoint_denial_readbacks.len(),
        denial_readback_blocker_count: denial_readback_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        denial_readback_scope,
        denial_readback_entries,
        entrypoint_denial_readbacks,
        denial_readback_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_RECOMMENDED_NEXT_GATE,
        denial_readback_visible: true,
        denial_readback_recorded: false,
        denial_readback_persisted: false,
        denial_readback_authoritative: false,
        denial_readback_accepted: false,
        denial_readback_scope_visible_only_complete,
        denial_readback_entries_complete,
        entrypoint_denial_readbacks_complete,
        denial_readback_blockers_complete,
        denial_readback_preconditions_complete,
        denial_readback_authorizes_live_attachment: false,
        denial_readback_authorizes_live_blocking_hook: false,
        denial_readback_authorizes_runtime_interception: false,
        denial_readback_authorizes_scheduler_admission_enforcement: false,
        denial_readback_authorizes_guardrail_enforcement: false,
        denial_readback_authorizes_work_graph_persistence: false,
        denial_readback_authorizes_lease_or_work_start: false,
        denial_readback_authorizes_agent_model_or_external_send: false,
        denial_readback_authorizes_live_task_result: false,
        denial_readback_authorizes_replay_or_rollback: false,
        denial_readback_authorizes_config_flag_or_traffic: false,
        denial_readback_authorizes_operator_approval_or_live_cutover: false,
        ready_for_denial_readback_audit_index: denial_readback_preconditions_complete,
        ready_for_live_attachment: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_scope()
-> WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackScopePreview {
    WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackScopePreview {
        id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_precondition_matrix_denial_readback_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix",
        readback_mode: "live_attachment_precondition_matrix_denial_readback_only",
        stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix.denial_readback",
        visible: true,
        recorded: false,
        persisted: false,
        authoritative: false,
        accepted: false,
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_entries()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackEntryPreview> {
    vec![
        readback_entry(
            "live_attachment_matrix_denial_state_readback",
            "live_attachment_matrix_denies_live_attachment",
            "deny_live_attachment_visible_without_record_accept_or_persistence",
        ),
        readback_entry(
            "live_attachment_entrypoint_inventory_readback",
            "live_attachment_matrix_entrypoints_visible",
            "four_entrypoints_visible_as_report_only_non_intercepting_surfaces",
        ),
        readback_entry(
            "live_attachment_precondition_check_catalog_readback",
            "live_attachment_matrix_precondition_checks_visible",
            "fourteen_checks_visible_with_ten_blocking_unsatisfied_preconditions",
        ),
        readback_entry(
            "live_attachment_blocker_catalog_readback",
            "live_attachment_matrix_blockers_visible",
            "thirty_three_blocked_actions_visible_without_authority_to_mutate",
        ),
        readback_entry(
            "live_attachment_prior_chain_readback",
            "live_attachment_matrix_prior_chain_visible",
            "sixteen_required_priors_visible_before_denial_readback",
        ),
        readback_entry(
            "live_attachment_non_attachment_boundary_readback",
            "live_attachment_denial_does_not_attach_runtime_hooks",
            "denial_readback_cannot_install_hooks_intercept_runtime_or_enforce_scheduler_guardrails",
        ),
        readback_entry(
            "live_attachment_no_live_authority_readback",
            "live_attachment_denial_does_not_unlock_live_paths",
            "denial_readback_cannot_authorize_work_start_agent_model_external_persistence_replay_config_traffic_operator_approval_or_cutover",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_entrypoint_denial_readbacks()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixEntrypointDenialReadbackPreview> {
    vec![
        entrypoint_readback("spawn_agent", "multi_agents_v2.spawn_agent"),
        entrypoint_readback("spawn_agents_on_csv", "agent_jobs.spawn_agents_on_csv"),
        entrypoint_readback("task_board_claim", "task_board.claim"),
        entrypoint_readback("worker_task_run", "worker_tasks.run_worker_task"),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_blockers()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackBlockerPreview> {
    vec![
        blocker(
            "denial_readback_record_blocked",
            "record_live_attachment_denial_readback",
        ),
        blocker(
            "denial_readback_persistence_blocked",
            "persist_live_attachment_denial_readback",
        ),
        blocker(
            "denial_readback_acceptance_blocked",
            "accept_live_attachment_denial_readback",
        ),
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

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_GATE,
    ];
    required_prior_gates.extend(crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_required_prior_gates());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            denial_readback_recorded: false,
            denial_readback_persisted: false,
            denial_readback_accepted: false,
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

fn readback_entry(
    id: &'static str,
    stable_readback_key: &'static str,
    observed_state: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackEntryPreview {
    WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackEntryPreview {
        id,
        stable_readback_key,
        observed_state,
        visible: true,
        recorded: false,
        persisted: false,
        authoritative: false,
        accepted: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn entrypoint_readback(
    entrypoint_id: &'static str,
    source_surface: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixEntrypointDenialReadbackPreview {
    WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixEntrypointDenialReadbackPreview {
        entrypoint_id,
        source_surface,
        denial_readback_key: "live_attachment_precondition_matrix_entrypoint_deny_live_allow_report_only",
        live_attachment_allowed: false,
        runtime_interception_allowed: false,
        report_only: true,
    }
}

fn blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackBlockerPreview {
    WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "scheduler/guardrail live attachment denial readback cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_attachment_denial_readback_derives_from_matrix() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_report();

        assert_eq!(
            report.source_matrix_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_GATE
        );
        assert_eq!(report.source_entrypoint_count, 4);
        assert_eq!(report.source_precondition_check_count, 14);
        assert_eq!(report.source_blocking_precondition_count, 10);
        assert_eq!(report.source_blocker_count, 33);
        assert_eq!(report.source_required_prior_gate_count, 16);
        assert!(report.source_matrix_ready);
        assert!(report.source_matrix_no_persistence_confirmed);
        assert!(report.source_matrix_no_live_confirmed);
        assert!(report.source_matrix_ready_for_denial_readback);
        assert_eq!(report.denial_readback_entry_count, 7);
        assert_eq!(report.entrypoint_denial_readback_count, 4);
        assert_eq!(report.denial_readback_blocker_count, 36);
        assert_eq!(report.required_prior_gate_count, 17);
    }

    #[test]
    fn live_attachment_denial_readback_keeps_entrypoints_report_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_report();

        assert_eq!(
            report
                .entrypoint_denial_readbacks
                .iter()
                .map(|entry| entry.entrypoint_id)
                .collect::<Vec<_>>(),
            vec![
                "spawn_agent",
                "spawn_agents_on_csv",
                "task_board_claim",
                "worker_task_run",
            ]
        );
        assert!(report.denial_readback_visible);
        assert!(!report.denial_readback_recorded);
        assert!(!report.denial_readback_persisted);
        assert!(!report.denial_readback_authoritative);
        assert!(!report.denial_readback_accepted);
        assert!(report.denial_readback_scope_visible_only_complete);
        assert!(report.denial_readback_entries_complete);
        assert!(report.entrypoint_denial_readbacks_complete);
        assert!(report.entrypoint_denial_readbacks.iter().all(|entry| {
            entry.report_only
                && !entry.live_attachment_allowed
                && !entry.runtime_interception_allowed
        }));
    }

    #[test]
    fn live_attachment_denial_readback_blocks_authority() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_report();

        assert!(
            report
                .denial_readback_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.denial_readback_blockers_complete);
        assert!(report.denial_readback_preconditions_complete);
        assert!(!report.denial_readback_authorizes_live_attachment);
        assert!(!report.denial_readback_authorizes_live_blocking_hook);
        assert!(!report.denial_readback_authorizes_runtime_interception);
        assert!(!report.denial_readback_authorizes_scheduler_admission_enforcement);
        assert!(!report.denial_readback_authorizes_guardrail_enforcement);
        assert!(!report.denial_readback_authorizes_work_graph_persistence);
        assert!(!report.denial_readback_authorizes_lease_or_work_start);
        assert!(!report.denial_readback_authorizes_agent_model_or_external_send);
        assert!(!report.denial_readback_authorizes_live_task_result);
        assert!(!report.denial_readback_authorizes_replay_or_rollback);
        assert!(!report.denial_readback_authorizes_config_flag_or_traffic);
        assert!(!report.denial_readback_authorizes_operator_approval_or_live_cutover);
        assert!(report.ready_for_denial_readback_audit_index);
        assert!(!report.ready_for_live_attachment);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn live_attachment_denial_readback_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_report();
        let side_effects = report.side_effects;

        assert!(
            !side_effects.filesystem_written
                && !side_effects.denial_readback_recorded
                && !side_effects.denial_readback_persisted
                && !side_effects.denial_readback_accepted
                && !side_effects.live_attachment_enabled
                && !side_effects.live_blocking_hook_installed
                && !side_effects.runtime_interception_enabled
                && !side_effects.scheduler_admission_enforced
                && !side_effects.guardrail_enforcement_enabled
                && !side_effects.work_graph_event_persisted
                && !side_effects.projection_index_persisted
                && !side_effects.lease_acquired
                && !side_effects.work_started
                && !side_effects.hardening_decision_recorded
                && !side_effects.hardening_decision_persisted
                && !side_effects.live_task_result_emitted
                && !side_effects.readback_executed
                && !side_effects.replay_executed
                && !side_effects.replay_diff_recorded
                && !side_effects.replay_diff_persisted
                && !side_effects.rollback_executed
                && !side_effects.idempotency_index_mutated
                && !side_effects.config_written
                && !side_effects.feature_flag_mutated
                && !side_effects.canary_traffic_routed
                && !side_effects.operator_review_requested
                && !side_effects.approval_recorded
                && !side_effects.live_cutover_performed
                && !side_effects.runtime_mutation_performed
                && !side_effects.agent_spawn_performed
                && !side_effects.external_send_performed
                && !side_effects.model_invoked
        );
    }
}
