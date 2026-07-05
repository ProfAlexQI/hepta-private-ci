use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReport {
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
    pub final_closeout_entry_count: usize,
    pub final_closeout_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub final_closeout_scope:
        WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutScopePreview,
    pub final_closeout_entries:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutEntryPreview>,
    pub final_closeout_blockers:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub terminal_no_attachment_branch_closed: bool,
    pub final_closeout_visible: bool,
    pub final_closeout_recorded: bool,
    pub final_closeout_persisted: bool,
    pub final_closeout_authoritative: bool,
    pub final_closeout_accepted: bool,
    pub source_audit_index_visible: bool,
    pub source_audit_index_persisted: bool,
    pub source_readback_persisted: bool,
    pub attachability_readback_persisted: bool,
    pub attachability_readiness_recording_allowed: bool,
    pub attachability_readiness_persistence_allowed: bool,
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
    pub ready_for_terminal_no_attachment_final_closeout_readback: bool,
    pub ready_for_live_attachment: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutScopePreview
{
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub closeout_mode: &'static str,
    pub stable_closeout_key: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub accepted: bool,
    pub terminal: bool,
    pub mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutEntryPreview
{
    pub id: &'static str,
    pub stable_closeout_key: &'static str,
    pub source_readback_id: &'static str,
    pub closeout_category: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub accepted: bool,
    pub authoritative: bool,
    pub mutation_allowed: bool,
    pub closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutSideEffects
{
    pub filesystem_written: bool,
    pub final_closeout_recorded: bool,
    pub final_closeout_persisted: bool,
    pub final_closeout_accepted: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_accepted: bool,
    pub audit_index_readback_recorded: bool,
    pub audit_index_readback_persisted: bool,
    pub audit_index_readback_accepted: bool,
    pub attachability_readback_recorded: bool,
    pub attachability_readback_persisted: bool,
    pub attachability_readback_accepted: bool,
    pub attachability_readiness_recorded: bool,
    pub attachability_readiness_persisted: bool,
    pub attachability_readiness_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report();
    let final_closeout_scope =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_scope();
    let final_closeout_entries =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_entries();
    let final_closeout_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_report_only",
        source_non_persistence_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        final_closeout_entry_count: final_closeout_entries.len(),
        final_closeout_blocker_count: final_closeout_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        final_closeout_scope,
        final_closeout_entries,
        final_closeout_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE,
        terminal_no_attachment_branch_closed: true,
        final_closeout_visible: true,
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_authoritative: false,
        final_closeout_accepted: false,
        source_audit_index_visible: source.audit_index_visible,
        source_audit_index_persisted: false,
        source_readback_persisted: false,
        attachability_readback_persisted: false,
        attachability_readiness_recording_allowed: false,
        attachability_readiness_persistence_allowed: false,
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
        ready_for_terminal_no_attachment_final_closeout_readback: true,
        ready_for_live_attachment: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_scope()
-> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutScopePreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutScopePreview {
        id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_readback_terminal_no_attachment_final_closeout_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback",
        closeout_mode:
            "live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_report_only",
        stable_closeout_key:
            "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback.terminal_no_attachment.final_closeout",
        visible: true,
        recorded: false,
        persisted: false,
        authoritative: false,
        accepted: false,
        terminal: true,
        mutation_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_entries()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutEntryPreview>{
    vec![
        final_closeout_entry(
            "live_attachment_attachability_no_attachment_branch_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.branch_closed",
            "attachability_audit_index_no_live_authority_readback",
            "terminal_no_attachment_branch",
        ),
        final_closeout_entry(
            "live_attachment_attachability_audit_index_surface_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.audit_index_surface",
            "attachability_audit_index_surface_non_persistence_readback",
            "audit_index_surface",
        ),
        final_closeout_entry(
            "live_attachment_attachability_audit_index_entry_inventory_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.audit_index_entries",
            "attachability_audit_index_entry_inventory_non_persistence_readback",
            "audit_index_entry_inventory",
        ),
        final_closeout_entry(
            "live_attachment_attachability_audit_index_blocker_inventory_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.audit_index_blockers",
            "attachability_audit_index_blocker_inventory_non_persistence_readback",
            "audit_index_blocker_inventory",
        ),
        final_closeout_entry(
            "live_attachment_attachability_audit_index_prior_chain_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.required_priors",
            "attachability_audit_index_prior_chain_non_persistence_readback",
            "required_prior_chain",
        ),
        final_closeout_entry(
            "live_attachment_attachability_non_persistence_boundary_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.non_persistence_boundary",
            "attachability_audit_index_non_persistence_boundary_readback",
            "non_persistence_boundary",
        ),
        final_closeout_entry(
            "live_attachment_attachability_no_live_authority_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.no_live_authority",
            "attachability_audit_index_no_live_authority_readback",
            "no_live_authority",
        ),
        final_closeout_entry(
            "live_attachment_attachability_entrypoint_scope_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.entrypoint_scope",
            "attachability_audit_index_entry_inventory_non_persistence_readback",
            "entrypoint_scope",
        ),
        final_closeout_entry(
            "live_attachment_attachability_scheduler_guardrail_boundary_final_closeout",
            "live_attachment_attachability_readback.terminal_no_attachment.final_closeout.scheduler_guardrail_boundary",
            "attachability_audit_index_no_live_authority_readback",
            "scheduler_guardrail_attachment_boundary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_blockers()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutBlockerPreview>{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report();
    let mut blockers = vec![
        blocker(
            "final_closeout_record_blocked",
            "record_live_attachment_attachability_terminal_no_attachment_final_closeout",
        ),
        blocker(
            "final_closeout_persistence_blocked",
            "persist_live_attachment_attachability_terminal_no_attachment_final_closeout",
        ),
        blocker(
            "final_closeout_acceptance_blocked",
            "accept_live_attachment_attachability_terminal_no_attachment_final_closeout",
        ),
    ];
    blockers.extend(
        source
            .readback_blockers
            .iter()
            .map(|source_blocker| blocker(source_blocker.id, source_blocker.blocked_action)),
    );
    blockers
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_required_prior_gates()
-> Vec<&'static str> {
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            final_closeout_recorded: false,
            final_closeout_persisted: false,
            final_closeout_accepted: false,
            audit_index_recorded: false,
            audit_index_persisted: false,
            audit_index_accepted: false,
            audit_index_readback_recorded: false,
            audit_index_readback_persisted: false,
            audit_index_readback_accepted: false,
            attachability_readback_recorded: false,
            attachability_readback_persisted: false,
            attachability_readback_accepted: false,
            attachability_readiness_recorded: false,
            attachability_readiness_persisted: false,
            attachability_readiness_accepted: false,
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

fn final_closeout_entry(
    id: &'static str,
    stable_closeout_key: &'static str,
    source_readback_id: &'static str,
    closeout_category: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutEntryPreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutEntryPreview {
        id,
        stable_closeout_key,
        source_readback_id,
        closeout_category,
        visible: true,
        recorded: false,
        persisted: false,
        accepted: false,
        authoritative: false,
        mutation_allowed: false,
        closed: true,
    }
}

fn blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutBlockerPreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason:
            "scheduler/guardrail live attachment attachability terminal no-attachment final closeout cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_attachment_attachability_terminal_no_attachment_final_closeout_derives_from_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_report();

        assert_eq!(
            report.source_non_persistence_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 6);
        assert_eq!(report.source_readback_blocker_count, 59);
        assert_eq!(report.source_required_prior_gate_count, 24);
        assert_eq!(report.final_closeout_entry_count, 9);
        assert_eq!(report.final_closeout_blocker_count, 62);
        assert_eq!(report.required_prior_gate_count, 25);
    }

    #[test]
    fn live_attachment_attachability_terminal_no_attachment_final_closeout_is_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_report();

        assert!(report.final_closeout_scope.visible);
        assert!(!report.final_closeout_scope.recorded);
        assert!(!report.final_closeout_scope.persisted);
        assert!(!report.final_closeout_scope.authoritative);
        assert!(!report.final_closeout_scope.accepted);
        assert!(report.final_closeout_scope.terminal);
        assert!(!report.final_closeout_scope.mutation_allowed);
        assert!(report.final_closeout_entries.iter().all(|entry| {
            entry.visible
                && entry.closed
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn live_attachment_attachability_terminal_no_attachment_final_closeout_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_report();

        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert!(
            report
                .final_closeout_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.terminal_no_attachment_branch_closed);
        assert!(report.final_closeout_visible);
        assert!(!report.final_closeout_recorded);
        assert!(!report.final_closeout_persisted);
        assert!(!report.final_closeout_authoritative);
        assert!(!report.final_closeout_accepted);
        assert!(report.source_audit_index_visible);
        assert!(!report.source_audit_index_persisted);
        assert!(!report.source_readback_persisted);
        assert!(!report.attachability_readback_persisted);
        assert!(!report.attachability_readiness_recording_allowed);
        assert!(!report.attachability_readiness_persistence_allowed);
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
        assert!(report.ready_for_terminal_no_attachment_final_closeout_readback);
        assert!(!report.ready_for_live_attachment);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn live_attachment_attachability_terminal_no_attachment_final_closeout_links_priors_and_side_effects()
     {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_report();

        assert_eq!(
            report.required_prior_gates,
            work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_required_prior_gates()
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackTerminalNoAttachmentFinalCloseoutSideEffects::none()
        );
    }
}
