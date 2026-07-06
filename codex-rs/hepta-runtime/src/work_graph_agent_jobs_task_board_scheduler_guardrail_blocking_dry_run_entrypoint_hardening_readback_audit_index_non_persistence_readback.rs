use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_GATE,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_terminal_no_enforcement_final_closeout_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackReport {
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
        WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackScopePreview,
    pub readback_entries:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackEntryPreview>,
    pub readback_blockers:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_accepted: bool,
    pub hardening_readback_visible: bool,
    pub hardening_readback_recorded: bool,
    pub hardening_readback_persisted: bool,
    pub hardening_readback_accepted: bool,
    pub audit_index_readback_recorded: bool,
    pub audit_index_readback_persisted: bool,
    pub audit_index_readback_accepted: bool,
    pub hardening_decision_recording_allowed: bool,
    pub hardening_decision_persistence_allowed: bool,
    pub live_blocking_enforcement_allowed: bool,
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
    pub ready_for_terminal_no_enforcement_final_closeout: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackScopePreview
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
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackEntryPreview
{
    pub id: &'static str,
    pub stable_readback_key: &'static str,
    pub observed_state: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub accepted: bool,
    pub authoritative: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackSideEffects
{
    pub filesystem_written: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_accepted: bool,
    pub audit_index_readback_recorded: bool,
    pub audit_index_readback_persisted: bool,
    pub audit_index_readback_accepted: bool,
    pub hardening_readback_recorded: bool,
    pub hardening_readback_persisted: bool,
    pub hardening_readback_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report();
    let readback_scope =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_scope();
    let readback_entries =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_entries();
    let readback_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_only",
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
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        hardening_readback_visible: true,
        hardening_readback_recorded: false,
        hardening_readback_persisted: false,
        hardening_readback_accepted: false,
        audit_index_readback_recorded: false,
        audit_index_readback_persisted: false,
        audit_index_readback_accepted: false,
        hardening_decision_recording_allowed: false,
        hardening_decision_persistence_allowed: false,
        live_blocking_enforcement_allowed: false,
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
        ready_for_terminal_no_enforcement_final_closeout: true,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_scope()
-> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackScopePreview{
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackScopePreview {
        id: "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index",
        readback_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_only",
        stable_readback_key: "work_graph.agent_jobs_task_board.scheduler_guardrail_blocking_dry_run.entrypoint_hardening.readback.audit_index.non_persistence_readback",
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

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_entries()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackEntryPreview>{
    vec![
        readback_entry(
            "hardening_audit_index_surface_non_persistence_readback",
            "entrypoint_hardening_readback_audit_index_visible_unrecorded",
            "audit_index_visible_without_record_persist_accept_or_authority",
        ),
        readback_entry(
            "hardening_audit_index_entry_inventory_non_persistence_readback",
            "entrypoint_hardening_readback_audit_index_entries_visible",
            "nine_audit_index_entries_visible_but_not_persisted",
        ),
        readback_entry(
            "hardening_audit_index_blocker_inventory_non_persistence_readback",
            "entrypoint_hardening_readback_audit_index_blockers_visible",
            "thirty_blockers_visible_and_still_blocking",
        ),
        readback_entry(
            "hardening_audit_index_prior_chain_non_persistence_readback",
            "entrypoint_hardening_readback_audit_index_priors_visible",
            "thirteen_required_prior_gates_visible_but_not_persisted",
        ),
        readback_entry(
            "hardening_audit_index_non_persistence_boundary_readback",
            "entrypoint_hardening_readback_audit_index_non_persistence_boundary",
            "audit_index_does_not_write_event_projection_scheduler_guardrail_or_runtime_state",
        ),
        readback_entry(
            "hardening_audit_index_no_live_authority_readback",
            "entrypoint_hardening_readback_audit_index_no_live_authority",
            "audit_index_does_not_authorize_enforcement_interception_work_start_agent_model_external_or_live_cutover",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_blockers()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackBlockerPreview>{
    vec![
        blocker(
            "audit_index_readback_record_blocked",
            "record_hardening_audit_index_non_persistence_readback",
        ),
        blocker(
            "audit_index_readback_persistence_blocked",
            "persist_hardening_audit_index_non_persistence_readback",
        ),
        blocker(
            "audit_index_readback_acceptance_blocked",
            "accept_hardening_audit_index_non_persistence_readback",
        ),
        blocker(
            "audit_index_record_blocked",
            "record_hardening_readback_audit_index",
        ),
        blocker(
            "audit_index_persistence_blocked",
            "persist_hardening_readback_audit_index",
        ),
        blocker(
            "audit_index_acceptance_blocked",
            "accept_hardening_readback_audit_index",
        ),
        blocker("readback_record_blocked", "record_hardening_readback"),
        blocker("readback_persistence_blocked", "persist_hardening_readback"),
        blocker("readback_acceptance_blocked", "accept_hardening_readback"),
        blocker(
            "hardening_decision_record_blocked",
            "record_hardening_decision",
        ),
        blocker(
            "hardening_decision_persistence_blocked",
            "persist_hardening_decision",
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
        blocker("agent_spawn_blocked", "spawn_agent"),
        blocker("model_invocation_blocked", "invoke_model"),
        blocker("external_send_blocked", "send_external_message"),
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

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_required_prior_gates()
-> Vec<&'static str> {
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            audit_index_recorded: false,
            audit_index_persisted: false,
            audit_index_accepted: false,
            audit_index_readback_recorded: false,
            audit_index_readback_persisted: false,
            audit_index_readback_accepted: false,
            hardening_readback_recorded: false,
            hardening_readback_persisted: false,
            hardening_readback_accepted: false,
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

fn readback_entry(
    id: &'static str,
    stable_readback_key: &'static str,
    observed_state: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackEntryPreview{
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackEntryPreview {
        id,
        stable_readback_key,
        observed_state,
        visible: true,
        recorded: false,
        persisted: false,
        accepted: false,
        authoritative: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackBlockerPreview{
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason:
            "scheduler/guardrail hardening readback audit index non-persistence readback cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_guardrail_audit_index_non_persistence_readback_derives_from_audit_index() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_report();

        assert_eq!(
            report.source_audit_index_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_GATE
        );
        assert_eq!(report.source_audit_index_entry_count, 9);
        assert_eq!(report.source_audit_index_blocker_count, 30);
        assert_eq!(report.source_required_prior_gate_count, 13);
        assert_eq!(report.readback_entry_count, 6);
        assert_eq!(report.readback_blocker_count, 33);
        assert_eq!(report.required_prior_gate_count, 14);
    }

    #[test]
    fn scheduler_guardrail_audit_index_non_persistence_readback_entries_are_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_report();

        assert_eq!(
            report.readback_scope.readback_mode,
            "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_only"
        );
        assert!(report.readback_scope.audit_index_visible);
        assert!(!report.readback_scope.audit_index_recorded);
        assert!(!report.readback_scope.audit_index_persisted);
        assert!(!report.readback_scope.audit_index_authoritative);
        assert!(!report.readback_scope.audit_index_accepted);
        assert!(!report.readback_scope.readback_recorded);
        assert!(!report.readback_scope.readback_persisted);
        assert!(!report.readback_scope.readback_accepted);
        assert!(report.readback_entries.iter().all(|entry| {
            entry.visible
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn scheduler_guardrail_audit_index_non_persistence_readback_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_report();

        assert!(report.audit_index_visible);
        assert!(report.hardening_readback_visible);
        assert!(report.ready_for_terminal_no_enforcement_final_closeout);
        assert!(
            report
                .readback_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(!report.audit_index_recorded);
        assert!(!report.audit_index_persisted);
        assert!(!report.audit_index_authoritative);
        assert!(!report.audit_index_accepted);
        assert!(!report.hardening_readback_recorded);
        assert!(!report.hardening_readback_persisted);
        assert!(!report.hardening_readback_accepted);
        assert!(!report.audit_index_readback_recorded);
        assert!(!report.audit_index_readback_persisted);
        assert!(!report.audit_index_readback_accepted);
        assert!(!report.hardening_decision_recording_allowed);
        assert!(!report.hardening_decision_persistence_allowed);
        assert!(!report.live_blocking_enforcement_allowed);
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
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn scheduler_guardrail_audit_index_non_persistence_readback_links_priors_and_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_GATE,
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
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexNonPersistenceReadbackSideEffects::none()
        );
    }
}
