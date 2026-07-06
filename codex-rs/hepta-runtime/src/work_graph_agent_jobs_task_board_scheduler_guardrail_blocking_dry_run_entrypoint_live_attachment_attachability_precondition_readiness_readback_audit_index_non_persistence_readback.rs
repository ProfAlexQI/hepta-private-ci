use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_GATE,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_audit_index_gate: &'static str,
    pub source_audit_index_entry_count: usize,
    pub source_audit_index_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub readback_entry_count: usize,
    pub readback_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_scope:
        WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackScopePreview,
    pub readback_entries:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackEntryPreview>,
    pub readback_blockers:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_accepted: bool,
    pub attachability_readback_visible: bool,
    pub attachability_readback_recorded: bool,
    pub attachability_readback_persisted: bool,
    pub attachability_readback_authoritative: bool,
    pub attachability_readback_accepted: bool,
    pub audit_index_readback_recorded: bool,
    pub audit_index_readback_persisted: bool,
    pub audit_index_readback_accepted: bool,
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
    pub ready_for_terminal_no_attachment_final_closeout: bool,
    pub ready_for_live_attachment: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackScopePreview
{
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub readback_mode: &'static str,
    pub stable_readback_key: &'static str,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_accepted: bool,
    pub readback_recorded: bool,
    pub readback_persisted: bool,
    pub readback_accepted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackEntryPreview
{
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
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackSideEffects
{
    pub filesystem_written: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_report();
    let readback_scope =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_scope();
    let readback_entries =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_entries();
    let readback_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_only",
        source_audit_index_gate: source.gate,
        source_audit_index_entry_count: source.audit_index_entry_count,
        source_audit_index_blocker_count: source.audit_index_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        readback_entry_count: readback_entries.len(),
        readback_blocker_count: readback_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_scope,
        readback_entries,
        readback_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        attachability_readback_visible: true,
        attachability_readback_recorded: false,
        attachability_readback_persisted: false,
        attachability_readback_authoritative: false,
        attachability_readback_accepted: false,
        audit_index_readback_recorded: false,
        audit_index_readback_persisted: false,
        audit_index_readback_accepted: false,
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
        ready_for_terminal_no_attachment_final_closeout: true,
        ready_for_live_attachment: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_scope()
-> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackScopePreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackScopePreview {
        id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_readback_audit_index_non_persistence_readback_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_precondition_readiness_readback_audit_index",
        readback_mode: "live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_only",
        stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback.audit_index.non_persistence_readback",
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        readback_recorded: false,
        readback_persisted: false,
        readback_accepted: false,
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_entries()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackEntryPreview>{
    vec![
        readback_entry(
            "attachability_audit_index_surface_non_persistence_readback",
            "live_attachment_attachability_readback_audit_index_visible_unrecorded",
            "audit_index_visible_without_record_persist_accept_or_authority",
        ),
        readback_entry(
            "attachability_audit_index_entry_inventory_non_persistence_readback",
            "live_attachment_attachability_readback_audit_index_entries_visible",
            "nine_audit_index_entries_visible_but_not_persisted",
        ),
        readback_entry(
            "attachability_audit_index_blocker_inventory_non_persistence_readback",
            "live_attachment_attachability_readback_audit_index_blockers_visible",
            "fifty_six_blockers_visible_and_still_blocking",
        ),
        readback_entry(
            "attachability_audit_index_prior_chain_non_persistence_readback",
            "live_attachment_attachability_readback_audit_index_priors_visible",
            "twenty_three_required_prior_gates_visible_but_not_persisted",
        ),
        readback_entry(
            "attachability_audit_index_non_persistence_boundary_readback",
            "live_attachment_attachability_readback_audit_index_non_persistence_boundary",
            "audit_index_does_not_write_event_projection_scheduler_guardrail_or_runtime_state",
        ),
        readback_entry(
            "attachability_audit_index_no_live_authority_readback",
            "live_attachment_attachability_readback_audit_index_no_live_authority",
            "audit_index_does_not_authorize_attachment_enforcement_interception_work_start_agent_model_external_or_live_cutover",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_blockers()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackBlockerPreview>{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_report();
    let mut blockers = vec![
        blocker(
            "audit_index_readback_record_blocked",
            "record_live_attachment_attachability_audit_index_non_persistence_readback",
        ),
        blocker(
            "audit_index_readback_persistence_blocked",
            "persist_live_attachment_attachability_audit_index_non_persistence_readback",
        ),
        blocker(
            "audit_index_readback_acceptance_blocked",
            "accept_live_attachment_attachability_audit_index_non_persistence_readback",
        ),
    ];
    blockers.extend(
        source
            .audit_index_blockers
            .iter()
            .map(|source_blocker| blocker(source_blocker.id, source_blocker.blocked_action)),
    );
    blockers
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_required_prior_gates()
-> Vec<&'static str> {
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_report();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
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

fn readback_entry(
    id: &'static str,
    stable_readback_key: &'static str,
    observed_state: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackEntryPreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackEntryPreview {
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

fn blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackBlockerPreview{
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "live attachment attachability readback audit index non-persistence readback cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_attachment_attachability_audit_index_non_persistence_readback_derives_from_audit_index()
    {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report();

        assert_eq!(
            report.source_audit_index_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_GATE
        );
        assert_eq!(report.source_audit_index_entry_count, 9);
        assert_eq!(report.source_audit_index_blocker_count, 56);
        assert_eq!(report.source_required_prior_gate_count, 23);
        assert_eq!(report.readback_entry_count, 6);
        assert_eq!(report.readback_blocker_count, 59);
        assert_eq!(report.required_prior_gate_count, 24);
    }

    #[test]
    fn live_attachment_attachability_audit_index_non_persistence_readback_stays_unpersisted() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report();

        assert!(report.audit_index_visible);
        assert!(!report.audit_index_recorded);
        assert!(!report.audit_index_persisted);
        assert!(!report.audit_index_authoritative);
        assert!(!report.audit_index_accepted);
        assert!(report.attachability_readback_visible);
        assert!(!report.attachability_readback_recorded);
        assert!(!report.attachability_readback_persisted);
        assert!(!report.attachability_readback_authoritative);
        assert!(!report.attachability_readback_accepted);
        assert!(!report.audit_index_readback_recorded);
        assert!(!report.audit_index_readback_persisted);
        assert!(!report.audit_index_readback_accepted);
        assert!(report.readback_entries.iter().all(|entry| {
            entry.visible
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.accepted
                && !entry.mutation_allowed
                && entry.ready
        }));
    }

    #[test]
    fn live_attachment_attachability_audit_index_non_persistence_readback_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report();

        assert!(
            report
                .readback_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
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
        assert!(report.ready_for_terminal_no_attachment_final_closeout);
        assert!(!report.ready_for_live_attachment);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn live_attachment_attachability_audit_index_non_persistence_readback_links_priors_and_side_effects()
     {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_report();

        assert_eq!(
            report.required_prior_gates,
            work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_required_prior_gates()
        );
        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_GATE
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReadbackAuditIndexNonPersistenceReadbackSideEffects::none()
        );
    }
}
