use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_AUDIT_INDEX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_AUDIT_INDEX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_denial_readback_gate: &'static str,
    pub source_denial_readback_entry_count: usize,
    pub source_entrypoint_denial_readback_count: usize,
    pub source_denial_readback_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub audit_index_entry_count: usize,
    pub audit_index_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub audit_index_scope:
        WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexScopePreview,
    pub audit_index_entries:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexEntryPreview>,
    pub audit_index_blockers:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_accepted: bool,
    pub denial_readback_visible: bool,
    pub denial_readback_recorded: bool,
    pub denial_readback_persisted: bool,
    pub denial_readback_authoritative: bool,
    pub denial_readback_accepted: bool,
    pub audit_index_authorizes_denial_readback_recording: bool,
    pub audit_index_authorizes_denial_readback_persistence: bool,
    pub audit_index_authorizes_matrix_recording: bool,
    pub audit_index_authorizes_matrix_persistence: bool,
    pub audit_index_authorizes_live_attachment: bool,
    pub audit_index_authorizes_live_blocking_hook: bool,
    pub audit_index_authorizes_runtime_interception: bool,
    pub audit_index_authorizes_scheduler_admission_enforcement: bool,
    pub audit_index_authorizes_guardrail_enforcement: bool,
    pub audit_index_authorizes_work_graph_persistence: bool,
    pub audit_index_authorizes_projection_persistence: bool,
    pub audit_index_authorizes_lease_or_work_start: bool,
    pub audit_index_authorizes_agent_model_or_external_send: bool,
    pub audit_index_authorizes_live_task_result: bool,
    pub audit_index_authorizes_replay_or_rollback: bool,
    pub audit_index_authorizes_config_flag_or_traffic: bool,
    pub audit_index_authorizes_operator_approval_or_live_cutover: bool,
    pub ready_for_non_persistence_readback: bool,
    pub ready_for_live_attachment: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexScopePreview
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
pub struct WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexEntryPreview
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
pub struct WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
    pub required_before_acceptance: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexSideEffects
{
    pub filesystem_written: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_accepted: bool,
    pub denial_readback_recorded: bool,
    pub denial_readback_persisted: bool,
    pub denial_readback_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_report();
    let audit_index_scope =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_scope();
    let audit_index_entries =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_entries();
    let audit_index_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_AUDIT_INDEX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_AUDIT_INDEX_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_report_only",
        source_denial_readback_gate: source.gate,
        source_denial_readback_entry_count: source.denial_readback_entry_count,
        source_entrypoint_denial_readback_count: source.entrypoint_denial_readback_count,
        source_denial_readback_blocker_count: source.denial_readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        audit_index_entry_count: audit_index_entries.len(),
        audit_index_blocker_count: audit_index_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        audit_index_scope,
        audit_index_entries,
        audit_index_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE,
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        denial_readback_visible: true,
        denial_readback_recorded: false,
        denial_readback_persisted: false,
        denial_readback_authoritative: false,
        denial_readback_accepted: false,
        audit_index_authorizes_denial_readback_recording: false,
        audit_index_authorizes_denial_readback_persistence: false,
        audit_index_authorizes_matrix_recording: false,
        audit_index_authorizes_matrix_persistence: false,
        audit_index_authorizes_live_attachment: false,
        audit_index_authorizes_live_blocking_hook: false,
        audit_index_authorizes_runtime_interception: false,
        audit_index_authorizes_scheduler_admission_enforcement: false,
        audit_index_authorizes_guardrail_enforcement: false,
        audit_index_authorizes_work_graph_persistence: false,
        audit_index_authorizes_projection_persistence: false,
        audit_index_authorizes_lease_or_work_start: false,
        audit_index_authorizes_agent_model_or_external_send: false,
        audit_index_authorizes_live_task_result: false,
        audit_index_authorizes_replay_or_rollback: false,
        audit_index_authorizes_config_flag_or_traffic: false,
        audit_index_authorizes_operator_approval_or_live_cutover: false,
        ready_for_non_persistence_readback: true,
        ready_for_live_attachment: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_scope()
-> WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexScopePreview {
    WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexScopePreview {
        id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_denial_readback_audit_index_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix_denial_readback",
        index_mode: "live_attachment_precondition_matrix_denial_readback_audit_index_report_only",
        stable_index_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix.denial_readback.audit_index",
        index_visible: true,
        index_recorded: false,
        index_persisted: false,
        index_authoritative: false,
        index_accepted: false,
        live_acceptance_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_entries()
-> Vec<
    WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexEntryPreview,
> {
    vec![
        audit_index_entry(
            "live_attachment_denial_readback_scope_audit_index",
            "live_attachment_denial_readback.audit_index.scope",
            "agent_jobs_task_board_scheduler_guardrail_live_attachment_precondition_matrix_denial_readback_scope",
            "denial_readback_scope",
        ),
        audit_index_entry(
            "live_attachment_denial_state_audit_index",
            "live_attachment_denial_readback.audit_index.denial_state",
            "live_attachment_matrix_denial_state_readback",
            "denial_state",
        ),
        audit_index_entry(
            "live_attachment_entrypoint_inventory_audit_index",
            "live_attachment_denial_readback.audit_index.entrypoint_inventory",
            "live_attachment_entrypoint_inventory_readback",
            "entrypoint_inventory",
        ),
        audit_index_entry(
            "live_attachment_precondition_check_catalog_audit_index",
            "live_attachment_denial_readback.audit_index.precondition_checks",
            "live_attachment_precondition_check_catalog_readback",
            "precondition_check_catalog",
        ),
        audit_index_entry(
            "live_attachment_blocker_inventory_audit_index",
            "live_attachment_denial_readback.audit_index.blocker_inventory",
            "live_attachment_blocker_catalog_readback",
            "blocker_inventory",
        ),
        audit_index_entry(
            "live_attachment_prior_chain_audit_index",
            "live_attachment_denial_readback.audit_index.prior_chain",
            "live_attachment_prior_chain_readback",
            "prior_chain",
        ),
        audit_index_entry(
            "live_attachment_non_attachment_boundary_audit_index",
            "live_attachment_denial_readback.audit_index.non_attachment_boundary",
            "live_attachment_non_attachment_boundary_readback",
            "non_attachment_boundary",
        ),
        audit_index_entry(
            "live_attachment_no_live_authority_audit_index",
            "live_attachment_denial_readback.audit_index.no_live_authority",
            "live_attachment_no_live_authority_readback",
            "no_live_authority",
        ),
        audit_index_entry(
            "live_attachment_entrypoint_surface_audit_index",
            "live_attachment_denial_readback.audit_index.entrypoint_surface",
            "live_attachment_precondition_matrix_entrypoint_deny_live_allow_report_only",
            "entrypoint_surface_denial",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_blockers()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexBlockerPreview>{
    vec![
        blocker(
            "audit_index_record_blocked",
            "record_live_attachment_denial_readback_audit_index",
        ),
        blocker(
            "audit_index_persistence_blocked",
            "persist_live_attachment_denial_readback_audit_index",
        ),
        blocker(
            "audit_index_acceptance_blocked",
            "accept_live_attachment_denial_readback_audit_index",
        ),
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

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_required_prior_gates()
-> Vec<&'static str> {
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_report();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            audit_index_recorded: false,
            audit_index_persisted: false,
            audit_index_accepted: false,
            denial_readback_recorded: false,
            denial_readback_persisted: false,
            denial_readback_accepted: false,
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

fn audit_index_entry(
    id: &'static str,
    stable_index_key: &'static str,
    source_readback_id: &'static str,
    audit_category: &'static str,
) -> WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexEntryPreview
{
    WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexEntryPreview {
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
) -> WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexBlockerPreview
{
    WorkGraphSchedulerGuardrailLiveAttachmentPreconditionMatrixDenialReadbackAuditIndexBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "required before live attachment denial readback audit index can be recorded, accepted, enforced, or cut live",
        required_before_acceptance: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_attachment_denial_readback_audit_index_derives_from_denial_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_report();

        assert_eq!(
            report.source_denial_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_GATE
        );
        assert_eq!(report.source_denial_readback_entry_count, 7);
        assert_eq!(report.source_entrypoint_denial_readback_count, 4);
        assert_eq!(report.source_denial_readback_blocker_count, 36);
        assert_eq!(report.source_required_prior_gate_count, 17);
        assert_eq!(report.audit_index_entry_count, 9);
        assert_eq!(report.audit_index_blocker_count, 39);
        assert_eq!(report.required_prior_gate_count, 18);
    }

    #[test]
    fn live_attachment_denial_readback_audit_index_is_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_report();

        assert!(report.audit_index_visible);
        assert!(!report.audit_index_recorded);
        assert!(!report.audit_index_persisted);
        assert!(!report.audit_index_authoritative);
        assert!(!report.audit_index_accepted);
        assert!(report.denial_readback_visible);
        assert!(!report.denial_readback_recorded);
        assert!(!report.denial_readback_persisted);
        assert!(!report.denial_readback_authoritative);
        assert!(!report.denial_readback_accepted);
        assert!(report.audit_index_entries.iter().all(|entry| {
            entry.indexed
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.accepted
                && !entry.mutation_allowed
                && entry.ready
        }));
    }

    #[test]
    fn live_attachment_denial_readback_audit_index_blocks_live_authority() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_report();

        assert!(
            report
                .audit_index_blockers
                .iter()
                .all(|blocker| blocker.blocked && blocker.required_before_acceptance)
        );
        assert!(!report.audit_index_authorizes_denial_readback_recording);
        assert!(!report.audit_index_authorizes_denial_readback_persistence);
        assert!(!report.audit_index_authorizes_matrix_recording);
        assert!(!report.audit_index_authorizes_matrix_persistence);
        assert!(!report.audit_index_authorizes_live_attachment);
        assert!(!report.audit_index_authorizes_live_blocking_hook);
        assert!(!report.audit_index_authorizes_runtime_interception);
        assert!(!report.audit_index_authorizes_scheduler_admission_enforcement);
        assert!(!report.audit_index_authorizes_guardrail_enforcement);
        assert!(!report.audit_index_authorizes_work_graph_persistence);
        assert!(!report.audit_index_authorizes_projection_persistence);
        assert!(!report.audit_index_authorizes_lease_or_work_start);
        assert!(!report.audit_index_authorizes_agent_model_or_external_send);
        assert!(!report.audit_index_authorizes_live_task_result);
        assert!(!report.audit_index_authorizes_replay_or_rollback);
        assert!(!report.audit_index_authorizes_config_flag_or_traffic);
        assert!(!report.audit_index_authorizes_operator_approval_or_live_cutover);
        assert!(report.ready_for_non_persistence_readback);
        assert!(!report.ready_for_live_attachment);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn live_attachment_denial_readback_audit_index_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_report();
        let side_effects = report.side_effects;

        assert!(
            !side_effects.filesystem_written
                && !side_effects.audit_index_recorded
                && !side_effects.audit_index_persisted
                && !side_effects.audit_index_accepted
                && !side_effects.denial_readback_recorded
                && !side_effects.denial_readback_persisted
                && !side_effects.denial_readback_accepted
                && !side_effects.matrix_recorded
                && !side_effects.matrix_persisted
                && !side_effects.matrix_accepted
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
