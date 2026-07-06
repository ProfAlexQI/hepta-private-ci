use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_non_persistence_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_hardening_readback_gate: &'static str,
    pub source_readback_entry_count: usize,
    pub source_entrypoint_readback_count: usize,
    pub source_readback_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub audit_index_entry_count: usize,
    pub audit_index_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub audit_index_scope:
        WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexScopePreview,
    pub audit_index_entries:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexEntryPreview>,
    pub audit_index_blockers:
        Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexBlockerPreview>,
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
    pub audit_index_authorizes_hardening_readback_recording: bool,
    pub audit_index_authorizes_hardening_readback_persistence: bool,
    pub audit_index_authorizes_hardening_decision_recording: bool,
    pub audit_index_authorizes_hardening_decision_persistence: bool,
    pub audit_index_authorizes_live_blocking_enforcement: bool,
    pub audit_index_authorizes_runtime_interception: bool,
    pub audit_index_authorizes_scheduler_admission_enforcement: bool,
    pub audit_index_authorizes_guardrail_enforcement: bool,
    pub audit_index_authorizes_work_graph_event_persistence: bool,
    pub audit_index_authorizes_projection_persistence: bool,
    pub audit_index_authorizes_lease_acquisition: bool,
    pub audit_index_authorizes_work_start: bool,
    pub audit_index_authorizes_agent_spawn: bool,
    pub audit_index_authorizes_model_invocation: bool,
    pub audit_index_authorizes_external_send: bool,
    pub audit_index_authorizes_replay_execution: bool,
    pub audit_index_authorizes_replay_diff_recording: bool,
    pub audit_index_authorizes_replay_diff_persistence: bool,
    pub audit_index_authorizes_rollback_execution: bool,
    pub audit_index_authorizes_idempotency_mutation: bool,
    pub audit_index_authorizes_config_write: bool,
    pub audit_index_authorizes_feature_flag_mutation: bool,
    pub audit_index_authorizes_canary_traffic: bool,
    pub audit_index_authorizes_operator_review_request: bool,
    pub audit_index_authorizes_approval_recording: bool,
    pub audit_index_authorizes_live_cutover: bool,
    pub ready_for_non_persistence_readback: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexScopePreview
{
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub index_mode: &'static str,
    pub stable_index_key: &'static str,
    pub index_visible: bool,
    pub index_recorded: bool,
    pub index_persisted: bool,
    pub index_authoritative: bool,
    pub index_accepted: bool,
    pub live_acceptance_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexEntryPreview
{
    pub id: &'static str,
    pub stable_index_key: &'static str,
    pub source_readback_id: &'static str,
    pub audit_category: &'static str,
    pub indexed: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub accepted: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
    pub required_before_acceptance: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexSideEffects
{
    pub filesystem_written: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_report();
    let audit_index_scope =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_scope();
    let audit_index_entries =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_entries();
    let audit_index_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report_only",
        source_hardening_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_entrypoint_readback_count: source.entrypoint_readback_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        audit_index_entry_count: audit_index_entries.len(),
        audit_index_blocker_count: audit_index_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        audit_index_scope,
        audit_index_entries,
        audit_index_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE,
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        hardening_readback_visible: true,
        hardening_readback_recorded: false,
        hardening_readback_persisted: false,
        hardening_readback_accepted: false,
        audit_index_authorizes_hardening_readback_recording: false,
        audit_index_authorizes_hardening_readback_persistence: false,
        audit_index_authorizes_hardening_decision_recording: false,
        audit_index_authorizes_hardening_decision_persistence: false,
        audit_index_authorizes_live_blocking_enforcement: false,
        audit_index_authorizes_runtime_interception: false,
        audit_index_authorizes_scheduler_admission_enforcement: false,
        audit_index_authorizes_guardrail_enforcement: false,
        audit_index_authorizes_work_graph_event_persistence: false,
        audit_index_authorizes_projection_persistence: false,
        audit_index_authorizes_lease_acquisition: false,
        audit_index_authorizes_work_start: false,
        audit_index_authorizes_agent_spawn: false,
        audit_index_authorizes_model_invocation: false,
        audit_index_authorizes_external_send: false,
        audit_index_authorizes_replay_execution: false,
        audit_index_authorizes_replay_diff_recording: false,
        audit_index_authorizes_replay_diff_persistence: false,
        audit_index_authorizes_rollback_execution: false,
        audit_index_authorizes_idempotency_mutation: false,
        audit_index_authorizes_config_write: false,
        audit_index_authorizes_feature_flag_mutation: false,
        audit_index_authorizes_canary_traffic: false,
        audit_index_authorizes_operator_review_request: false,
        audit_index_authorizes_approval_recording: false,
        audit_index_authorizes_live_cutover: false,
        ready_for_non_persistence_readback: true,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_scope()
-> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexScopePreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexScopePreview {
        id: "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback",
        index_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report_only",
        stable_index_key: "work_graph.agent_jobs_task_board.scheduler_guardrail_blocking_dry_run.entrypoint_hardening.readback.audit_index",
        index_visible: true,
        index_recorded: false,
        index_persisted: false,
        index_authoritative: false,
        index_accepted: false,
        live_acceptance_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_entries()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexEntryPreview> {
    vec![
        audit_index_entry(
            "hardening_readback_scope_audit_index",
            "entrypoint_hardening_readback.audit_index.scope",
            "agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_scope",
            "hardening_readback_scope",
        ),
        audit_index_entry(
            "hardening_readback_entry_inventory_audit_index",
            "entrypoint_hardening_readback.audit_index.readback_entry_inventory",
            "hardening_contract_inventory_readback",
            "hardening_readback_entries",
        ),
        audit_index_entry(
            "hardening_entrypoint_readback_inventory_audit_index",
            "entrypoint_hardening_readback.audit_index.entrypoint_readback_inventory",
            "spawn_agent_hardening_readback",
            "entrypoint_readbacks",
        ),
        audit_index_entry(
            "hardening_readback_blocker_inventory_audit_index",
            "entrypoint_hardening_readback.audit_index.blocker_inventory",
            "hardening_blocker_inventory_readback",
            "readback_blockers",
        ),
        audit_index_entry(
            "hardening_readback_prior_chain_audit_index",
            "entrypoint_hardening_readback.audit_index.prior_chain",
            "hardening_prior_chain_readback",
            "required_prior_chain",
        ),
        audit_index_entry(
            "hardening_readback_non_live_guard_audit_index",
            "entrypoint_hardening_readback.audit_index.non_live_guard",
            "hardening_non_live_guard_readback",
            "non_live_guard_contract",
        ),
        audit_index_entry(
            "hardening_readback_no_live_authority_audit_index",
            "entrypoint_hardening_readback.audit_index.no_live_authority",
            "hardening_no_live_authority_readback",
            "no_live_authority",
        ),
        audit_index_entry(
            "hardening_source_decision_trace_audit_index",
            "entrypoint_hardening_readback.audit_index.source_decision_trace",
            "hardening_decision_inventory_readback",
            "source_decision_trace",
        ),
        audit_index_entry(
            "hardening_live_boundary_audit_index",
            "entrypoint_hardening_readback.audit_index.live_boundary",
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_GATE,
            "live_cutover_boundary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_blockers()
-> Vec<WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexBlockerPreview>
{
    vec![
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

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_required_prior_gates()
-> Vec<&'static str> {
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_report();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            audit_index_recorded: false,
            audit_index_persisted: false,
            audit_index_accepted: false,
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

fn audit_index_entry(
    id: &'static str,
    stable_index_key: &'static str,
    source_readback_id: &'static str,
    audit_category: &'static str,
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexEntryPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexEntryPreview {
        id,
        stable_index_key,
        source_readback_id,
        audit_category,
        indexed: true,
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
) -> WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexBlockerPreview {
    WorkGraphSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "required before scheduler/guardrail hardening readback audit index can be recorded, accepted, enforced, or cut live",
        required_before_acceptance: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_guardrail_entrypoint_hardening_readback_audit_index_derives_from_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report();

        assert_eq!(
            report.source_hardening_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 7);
        assert_eq!(report.source_entrypoint_readback_count, 4);
        assert_eq!(report.source_readback_blocker_count, 27);
        assert_eq!(report.source_required_prior_gate_count, 12);
        assert_eq!(report.audit_index_entry_count, 9);
        assert_eq!(report.audit_index_blocker_count, 30);
        assert_eq!(report.required_prior_gate_count, 13);
    }

    #[test]
    fn scheduler_guardrail_entrypoint_hardening_readback_audit_index_is_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report();

        assert!(report.audit_index_scope.index_visible);
        assert!(!report.audit_index_scope.index_recorded);
        assert!(!report.audit_index_scope.index_persisted);
        assert!(!report.audit_index_scope.index_authoritative);
        assert!(!report.audit_index_scope.index_accepted);
        assert!(!report.audit_index_scope.live_acceptance_allowed);
        assert!(report.audit_index_entries.iter().all(|entry| {
            entry.indexed
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.accepted
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn scheduler_guardrail_entrypoint_hardening_readback_audit_index_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report();

        assert!(report.audit_index_visible);
        assert!(!report.audit_index_recorded);
        assert!(!report.audit_index_persisted);
        assert!(!report.audit_index_authoritative);
        assert!(!report.audit_index_accepted);
        assert!(report.hardening_readback_visible);
        assert!(!report.hardening_readback_recorded);
        assert!(!report.hardening_readback_persisted);
        assert!(!report.hardening_readback_accepted);
        assert!(
            report
                .audit_index_blockers
                .iter()
                .all(|blocker| blocker.blocked && blocker.required_before_acceptance)
        );
        assert!(!report.audit_index_authorizes_hardening_readback_recording);
        assert!(!report.audit_index_authorizes_hardening_readback_persistence);
        assert!(!report.audit_index_authorizes_hardening_decision_recording);
        assert!(!report.audit_index_authorizes_hardening_decision_persistence);
        assert!(!report.audit_index_authorizes_live_blocking_enforcement);
        assert!(!report.audit_index_authorizes_runtime_interception);
        assert!(!report.audit_index_authorizes_scheduler_admission_enforcement);
        assert!(!report.audit_index_authorizes_guardrail_enforcement);
        assert!(!report.audit_index_authorizes_work_graph_event_persistence);
        assert!(!report.audit_index_authorizes_projection_persistence);
        assert!(!report.audit_index_authorizes_lease_acquisition);
        assert!(!report.audit_index_authorizes_work_start);
        assert!(!report.audit_index_authorizes_agent_spawn);
        assert!(!report.audit_index_authorizes_model_invocation);
        assert!(!report.audit_index_authorizes_external_send);
        assert!(!report.audit_index_authorizes_replay_execution);
        assert!(!report.audit_index_authorizes_replay_diff_recording);
        assert!(!report.audit_index_authorizes_replay_diff_persistence);
        assert!(!report.audit_index_authorizes_rollback_execution);
        assert!(!report.audit_index_authorizes_idempotency_mutation);
        assert!(!report.audit_index_authorizes_config_write);
        assert!(!report.audit_index_authorizes_feature_flag_mutation);
        assert!(!report.audit_index_authorizes_canary_traffic);
        assert!(!report.audit_index_authorizes_operator_review_request);
        assert!(!report.audit_index_authorizes_approval_recording);
        assert!(!report.audit_index_authorizes_live_cutover);
        assert!(report.ready_for_non_persistence_readback);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn scheduler_guardrail_entrypoint_hardening_readback_audit_index_links_priors_and_side_effects()
    {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_audit_index_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_GATE,
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
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_HARDENING_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointHardeningReadbackAuditIndexSideEffects::none()
        );
    }
}
