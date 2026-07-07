use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackAuditIndexNonPersistenceReadbackSideEffects,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_TERMINAL_CLOSEOUT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_TERMINAL_CLOSEOUT_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_TERMINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_non_persistence_readback_gate: &'static str,
    pub source_readback_entry_count: usize,
    pub source_readback_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub source_non_persistence_readback_ready: bool,
    pub source_non_persistence_readback_no_persistence_confirmed: bool,
    pub source_non_persistence_readback_no_live_confirmed: bool,
    pub source_non_persistence_readback_ready_for_terminal_closeout: bool,
    pub terminal_closeout_entry_count: usize,
    pub terminal_closeout_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub terminal_closeout_scope:
        WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutScopePreview,
    pub terminal_closeout_entries:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutEntryPreview>,
    pub terminal_closeout_blockers:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub terminal_closeout_scope_complete: bool,
    pub terminal_closeout_entries_complete: bool,
    pub terminal_closeout_blockers_complete: bool,
    pub terminal_closeout_preconditions_complete: bool,
    pub terminal_closeout_visible: bool,
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_authoritative: bool,
    pub terminal_closeout_accepted: bool,
    pub source_audit_index_visible: bool,
    pub source_audit_index_recorded: bool,
    pub source_audit_index_persisted: bool,
    pub source_audit_index_authoritative: bool,
    pub source_audit_index_accepted: bool,
    pub source_readback_recorded: bool,
    pub source_readback_persisted: bool,
    pub source_readback_accepted: bool,
    pub terminal_no_attachment_branch_closed: bool,
    pub terminal_closeout_readback_recording_allowed: bool,
    pub terminal_closeout_readback_persistence_allowed: bool,
    pub terminal_closeout_recording_allowed: bool,
    pub terminal_closeout_persistence_allowed: bool,
    pub terminal_closeout_acceptance_allowed: bool,
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
    pub ready_for_terminal_closeout_readback: bool,
    pub ready_for_live_attachment: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutScopePreview
{
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub closeout_mode: &'static str,
    pub stable_closeout_key: &'static str,
    pub closeout_visible: bool,
    pub closeout_recorded: bool,
    pub closeout_persisted: bool,
    pub closeout_authoritative: bool,
    pub closeout_accepted: bool,
    pub live_acceptance_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutEntryPreview
{
    pub id: &'static str,
    pub stable_closeout_key: &'static str,
    pub source_readback_id: &'static str,
    pub closeout_category: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub accepted: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
    pub required_before_acceptance: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutSideEffects
{
    pub filesystem_written: bool,
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_accepted: bool,
    pub terminal_closeout_readback_recorded: bool,
    pub terminal_closeout_readback_persisted: bool,
    pub terminal_closeout_readback_accepted: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_accepted: bool,
    pub audit_index_readback_recorded: bool,
    pub audit_index_readback_persisted: bool,
    pub audit_index_readback_accepted: bool,
    pub live_attachment_enabled: bool,
    pub live_blocking_hook_installed: bool,
    pub runtime_interception_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub work_graph_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub lease_acquired: bool,
    pub work_started: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback_report();
    let terminal_closeout_scope =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_scope();
    let terminal_closeout_entries =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_entries();
    let terminal_closeout_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_required_prior_gates();
    let source_non_persistence_side_effects_all_false = source.side_effects
        == WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackAuditIndexNonPersistenceReadbackSideEffects::none();
    let source_non_persistence_readback_no_persistence_confirmed = source
        .non_persistence_readback_preconditions_complete
        && source.audit_index_visible
        && !source.audit_index_recorded
        && !source.audit_index_persisted
        && !source.audit_index_authoritative
        && !source.audit_index_accepted
        && !source.audit_index_readback_recorded
        && !source.audit_index_readback_persisted
        && !source.audit_index_readback_accepted
        && !source.terminal_closeout_recording_allowed
        && !source.terminal_closeout_persistence_allowed
        && !source.work_graph_event_persistence_allowed
        && !source.projection_persistence_allowed
        && source_non_persistence_side_effects_all_false;
    let source_non_persistence_readback_no_live_confirmed = source
        .ready_for_terminal_no_attachment_final_closeout_readback_terminal_closeout
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
        && !source.hardening_decision_recording_allowed
        && !source.hardening_decision_persistence_allowed
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
        && !source.ready_for_live_attachment
        && !source.ready_for_live_execution
        && source_non_persistence_readback_no_persistence_confirmed;
    let source_non_persistence_readback_ready = source.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        && source.source_audit_index_ready
        && source.source_audit_index_no_persistence_confirmed
        && source.source_audit_index_no_live_confirmed
        && source.source_audit_index_ready_for_non_persistence_readback
        && source.non_persistence_readback_preconditions_complete
        && source.readback_entry_count == 6
        && source.readback_blocker_count == 71
        && source.required_prior_gate_count == 28
        && source_non_persistence_readback_no_live_confirmed;
    let source_non_persistence_readback_ready_for_terminal_closeout =
        source_non_persistence_readback_ready
            && source.ready_for_terminal_no_attachment_final_closeout_readback_terminal_closeout;
    let terminal_closeout_scope_complete = terminal_closeout_scope.closeout_visible
        && !terminal_closeout_scope.closeout_recorded
        && !terminal_closeout_scope.closeout_persisted
        && !terminal_closeout_scope.closeout_authoritative
        && !terminal_closeout_scope.closeout_accepted
        && !terminal_closeout_scope.live_acceptance_allowed;
    let terminal_closeout_entries_complete = terminal_closeout_entries.len() == 8
        && terminal_closeout_entries.iter().all(|entry| {
            entry.visible
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.accepted
                && !entry.mutation_allowed
        });
    let terminal_closeout_blockers_complete = terminal_closeout_blockers.len() == 74
        && terminal_closeout_blockers
            .iter()
            .all(|blocker| blocker.blocked && blocker.required_before_acceptance);
    let terminal_closeout_preconditions_complete =
        source_non_persistence_readback_ready_for_terminal_closeout
            && terminal_closeout_scope_complete
            && terminal_closeout_entries_complete
            && terminal_closeout_blockers_complete;

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_TERMINAL_CLOSEOUT_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_TERMINAL_CLOSEOUT_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_report_only",
        source_non_persistence_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        source_non_persistence_readback_ready,
        source_non_persistence_readback_no_persistence_confirmed,
        source_non_persistence_readback_no_live_confirmed,
        source_non_persistence_readback_ready_for_terminal_closeout,
        terminal_closeout_entry_count: terminal_closeout_entries.len(),
        terminal_closeout_blocker_count: terminal_closeout_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        terminal_closeout_scope,
        terminal_closeout_entries,
        terminal_closeout_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_TERMINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE,
        terminal_closeout_scope_complete,
        terminal_closeout_entries_complete,
        terminal_closeout_blockers_complete,
        terminal_closeout_preconditions_complete,
        terminal_closeout_visible: true,
        terminal_closeout_recorded: false,
        terminal_closeout_persisted: false,
        terminal_closeout_authoritative: false,
        terminal_closeout_accepted: false,
        source_audit_index_visible: source.audit_index_visible,
        source_audit_index_recorded: source.audit_index_recorded,
        source_audit_index_persisted: source.audit_index_persisted,
        source_audit_index_authoritative: source.audit_index_authoritative,
        source_audit_index_accepted: source.audit_index_accepted,
        source_readback_recorded: source.audit_index_readback_recorded,
        source_readback_persisted: source.audit_index_readback_persisted,
        source_readback_accepted: source.audit_index_readback_accepted,
        terminal_no_attachment_branch_closed: source.terminal_no_attachment_branch_closed,
        terminal_closeout_readback_recording_allowed: false,
        terminal_closeout_readback_persistence_allowed: false,
        terminal_closeout_recording_allowed: false,
        terminal_closeout_persistence_allowed: false,
        terminal_closeout_acceptance_allowed: false,
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
        ready_for_terminal_closeout_readback: terminal_closeout_preconditions_complete,
        ready_for_live_attachment: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_scope()
-> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutScopePreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutScopePreview {
        id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_scope",
        source_surface_id:
            "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback",
        closeout_mode:
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_report_only",
        stable_closeout_key:
            "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback.terminal_no_attachment_final_closeout.readback.terminal_closeout",
        closeout_visible: true,
        closeout_recorded: false,
        closeout_persisted: false,
        closeout_authoritative: false,
        closeout_accepted: false,
        live_acceptance_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_entries()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutEntryPreview>{
    vec![
        terminal_closeout_entry(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_branch_closed",
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.branch_closed",
            "terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback",
            "branch_state",
        ),
        terminal_closeout_entry(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_source_readback",
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.source_non_persistence_readback",
            "terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback",
            "source_readback",
        ),
        terminal_closeout_entry(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_inventory",
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.inventory",
            "terminal_no_attachment_final_closeout_readback_audit_index_entry_inventory_non_persistence_readback",
            "inventory",
        ),
        terminal_closeout_entry(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_blockers",
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.blockers",
            "terminal_no_attachment_final_closeout_readback_audit_index_blocker_inventory_non_persistence_readback",
            "blockers",
        ),
        terminal_closeout_entry(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_priors",
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.priors",
            "terminal_no_attachment_final_closeout_readback_audit_index_prior_chain_non_persistence_readback",
            "prior_chain",
        ),
        terminal_closeout_entry(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_non_persistence_boundary",
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.non_persistence_boundary",
            "terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_boundary_readback",
            "non_persistence",
        ),
        terminal_closeout_entry(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_no_live_authority",
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.no_live_authority",
            "terminal_no_attachment_final_closeout_readback_audit_index_no_live_authority_readback",
            "no_live_authority",
        ),
        terminal_closeout_entry(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_entrypoint_surface",
            "live_attachment_attachability_terminal_no_attachment_final_closeout_readback.terminal_closeout.entrypoint_surface",
            "terminal_no_attachment_final_closeout_readback_audit_index_surface_non_persistence_readback",
            "entrypoint_surface",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_blockers()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutBlockerPreview>{
    let source_blockers =
        crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback_blockers();
    let mut blockers = vec![
        terminal_closeout_blocker(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_record_blocked",
            "record_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout",
        ),
        terminal_closeout_blocker(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_persistence_blocked",
            "persist_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout",
        ),
        terminal_closeout_blocker(
            "terminal_no_attachment_final_closeout_readback_terminal_closeout_acceptance_blocked",
            "accept_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout",
        ),
    ];
    blockers.extend(
        source_blockers
            .into_iter()
            .map(|blocker| terminal_closeout_blocker(blocker.id, blocker.blocked_action)),
    );
    blockers
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_required_prior_gates()
-> Vec<&'static str> {
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    ];
    required_prior_gates.extend(crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback_required_prior_gates());
    required_prior_gates
}

fn terminal_closeout_entry(
    id: &'static str,
    stable_closeout_key: &'static str,
    source_readback_id: &'static str,
    closeout_category: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutEntryPreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutEntryPreview {
        id,
        stable_closeout_key,
        source_readback_id,
        closeout_category,
        visible: true,
        recorded: false,
        persisted: false,
        authoritative: false,
        accepted: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn terminal_closeout_blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutBlockerPreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "terminal no-attachment final closeout readback terminal closeout cannot authorize this action",
        required_before_acceptance: true,
    }
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            terminal_closeout_recorded: false,
            terminal_closeout_persisted: false,
            terminal_closeout_accepted: false,
            terminal_closeout_readback_recorded: false,
            terminal_closeout_readback_persisted: false,
            terminal_closeout_readback_accepted: false,
            audit_index_recorded: false,
            audit_index_persisted: false,
            audit_index_accepted: false,
            audit_index_readback_recorded: false,
            audit_index_readback_persisted: false,
            audit_index_readback_accepted: false,
            live_attachment_enabled: false,
            live_blocking_hook_installed: false,
            runtime_interception_enabled: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            work_graph_event_persisted: false,
            projection_index_persisted: false,
            lease_acquired: false,
            work_started: false,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_no_attachment_final_closeout_readback_terminal_closeout_derives_from_non_persistence_readback()
     {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_report();

        assert_eq!(
            report.source_non_persistence_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 6);
        assert_eq!(report.source_readback_blocker_count, 71);
        assert_eq!(report.source_required_prior_gate_count, 28);
        assert!(report.source_non_persistence_readback_ready);
        assert!(report.source_non_persistence_readback_no_persistence_confirmed);
        assert!(report.source_non_persistence_readback_no_live_confirmed);
        assert!(report.source_non_persistence_readback_ready_for_terminal_closeout);
        assert_eq!(report.terminal_closeout_entry_count, 8);
        assert_eq!(
            report.terminal_closeout_blocker_count,
            report.source_readback_blocker_count + 3
        );
        assert_eq!(
            report.required_prior_gate_count,
            report.source_required_prior_gate_count + 1
        );
    }

    #[test]
    fn terminal_no_attachment_final_closeout_readback_terminal_closeout_is_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_report();

        assert!(report.terminal_closeout_visible);
        assert!(!report.terminal_closeout_recorded);
        assert!(!report.terminal_closeout_persisted);
        assert!(!report.terminal_closeout_authoritative);
        assert!(!report.terminal_closeout_accepted);
        assert!(report.source_audit_index_visible);
        assert!(!report.source_audit_index_recorded);
        assert!(!report.source_audit_index_persisted);
        assert!(!report.source_audit_index_authoritative);
        assert!(!report.source_audit_index_accepted);
        assert!(report.terminal_no_attachment_branch_closed);
        assert!(!report.source_readback_recorded);
        assert!(!report.source_readback_persisted);
        assert!(!report.source_readback_accepted);
        assert!(report.terminal_closeout_scope_complete);
        assert!(report.terminal_closeout_entries_complete);
        assert!(report.terminal_closeout_blockers_complete);
        assert!(report.terminal_closeout_preconditions_complete);
        assert!(report.ready_for_terminal_closeout_readback);
        assert!(!report.ready_for_live_attachment);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn terminal_no_attachment_final_closeout_readback_terminal_closeout_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_report();
        let blocked_actions: Vec<&str> = report
            .terminal_closeout_blockers
            .iter()
            .map(|blocker| blocker.blocked_action)
            .collect();

        for action in [
            "record_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout",
            "persist_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout",
            "accept_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout",
            "record_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback",
            "persist_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback",
            "accept_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_audit_index_non_persistence_readback",
            "enable_live_attachment",
            "install_live_blocking_hook",
            "enable_runtime_interception",
            "enforce_scheduler_admission",
            "enable_guardrail_enforcement",
            "persist_work_graph_event",
            "spawn_agent",
            "spawn_agents_on_csv",
            "claim_task_board_work",
            "run_worker_task",
            "emit_live_task_result",
            "execute_readback",
            "execute_replay",
            "execute_rollback",
            "write_config",
            "mutate_feature_flag",
            "route_canary_traffic",
            "request_operator_review",
            "record_operator_approval",
            "perform_live_cutover",
        ] {
            assert!(
                blocked_actions.contains(&action),
                "missing blocked action {action}"
            );
        }

        assert!(
            report
                .terminal_closeout_blockers
                .iter()
                .all(|blocker| blocker.blocked && blocker.required_before_acceptance)
        );
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
        assert!(!report.readback_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.config_write_allowed);
        assert!(!report.feature_flag_mutation_allowed);
        assert!(!report.canary_traffic_allowed);
        assert!(!report.operator_review_request_allowed);
        assert!(!report.approval_recording_allowed);
        assert!(!report.live_cutover_allowed);
    }

    #[test]
    fn terminal_no_attachment_final_closeout_readback_terminal_closeout_links_priors_and_side_effects()
     {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_report();

        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert_eq!(
            report.required_prior_gates,
            work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_required_prior_gates()
        );
        assert_eq!(
            report.terminal_closeout_entries,
            work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_entries()
        );
        assert!(
            report
                .terminal_closeout_entries
                .iter()
                .all(|entry| entry.visible
                    && entry.ready
                    && !entry.recorded
                    && !entry.persisted
                    && !entry.authoritative
                    && !entry.accepted
                    && !entry.mutation_allowed)
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReadbackTerminalCloseoutSideEffects::none()
        );
    }
}
