use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_GATE,
    hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_terminal_no_attachment_final_closeout_gate: &'static str,
    pub source_final_closeout_entry_count: usize,
    pub source_final_closeout_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub attachability_entrypoint_count: usize,
    pub attachability_precondition_check_count: usize,
    pub attachability_precondition_satisfied_count: usize,
    pub attachability_precondition_unsatisfied_count: usize,
    pub blocking_precondition_count: usize,
    pub attachability_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub attachability_entrypoints:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityEntrypointReadinessPreview>,
    pub attachability_precondition_checks:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionCheckPreview>,
    pub attachability_blockers:
        Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub readiness_mode: &'static str,
    pub readiness_visible: bool,
    pub readiness_recorded: bool,
    pub readiness_persisted: bool,
    pub readiness_authoritative: bool,
    pub readiness_accepted: bool,
    pub terminal_no_attachment_branch_closed: bool,
    pub attachability_candidates_identified: bool,
    pub attachability_preconditions_satisfied: bool,
    pub source_final_closeout_visible: bool,
    pub source_final_closeout_persisted: bool,
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
    pub ready_for_attachability_precondition_readiness_readback: bool,
    pub ready_for_live_attachment: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityEntrypointReadinessPreview {
    pub id: &'static str,
    pub surface: &'static str,
    pub attachability_status: &'static str,
    pub required_contracts: Vec<&'static str>,
    pub live_attachment_candidate: bool,
    pub live_attachment_allowed: bool,
    pub report_only: bool,
    pub runtime_interception_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionCheckPreview {
    pub id: &'static str,
    pub category: &'static str,
    pub required: bool,
    pub satisfied: bool,
    pub blocking: bool,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessSideEffects
{
    pub filesystem_written: bool,
    pub readiness_recorded: bool,
    pub readiness_persisted: bool,
    pub readiness_accepted: bool,
    pub final_closeout_recorded: bool,
    pub final_closeout_persisted: bool,
    pub final_closeout_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_report()
-> WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_report();
    let attachability_entrypoints =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_entrypoints();
    let attachability_precondition_checks =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_checks();
    let attachability_blockers =
        work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    let attachability_precondition_satisfied_count = attachability_precondition_checks
        .iter()
        .filter(|check| check.satisfied)
        .count();
    let blocking_precondition_count = attachability_precondition_checks
        .iter()
        .filter(|check| check.blocking)
        .count();

    WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_SCHEMA_VERSION,
        preview_mode:
            "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_report_only",
        source_terminal_no_attachment_final_closeout_gate: source.gate,
        source_final_closeout_entry_count: source.final_closeout_entry_count,
        source_final_closeout_blocker_count: source.final_closeout_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        attachability_entrypoint_count: attachability_entrypoints.len(),
        attachability_precondition_check_count: attachability_precondition_checks.len(),
        attachability_precondition_satisfied_count,
        attachability_precondition_unsatisfied_count: attachability_precondition_checks.len()
            - attachability_precondition_satisfied_count,
        blocking_precondition_count,
        attachability_blocker_count: attachability_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        attachability_entrypoints,
        attachability_precondition_checks,
        attachability_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_RECOMMENDED_NEXT_GATE,
        readiness_mode:
            "identify_live_attachment_candidates_but_keep_attachment_blocked_until_all_priors_authorized",
        readiness_visible: true,
        readiness_recorded: false,
        readiness_persisted: false,
        readiness_authoritative: false,
        readiness_accepted: false,
        terminal_no_attachment_branch_closed: source.terminal_no_attachment_branch_closed,
        attachability_candidates_identified: true,
        attachability_preconditions_satisfied: false,
        source_final_closeout_visible: source.final_closeout_visible,
        source_final_closeout_persisted: false,
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
        ready_for_attachability_precondition_readiness_readback: true,
        ready_for_live_attachment: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_entrypoints()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityEntrypointReadinessPreview> {
    vec![
        entrypoint("spawn_agent", "multi_agents_v2.spawn_agent"),
        entrypoint("spawn_agents_on_csv", "agent_jobs.spawn_agents_on_csv"),
        entrypoint("task_board_claim", "task_board.claim"),
        entrypoint("worker_task_run", "worker_tasks.run_worker_task"),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_checks()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionCheckPreview> {
    vec![
        check(
            "terminal_no_attachment_final_closeout_ready",
            "source_evidence",
            true,
            false,
            "terminal no-attachment branch is closed as report-only evidence",
        ),
        check(
            "entrypoint_inventory_stable",
            "entrypoint_scope",
            true,
            false,
            "four entrypoint surfaces are stable enough to evaluate for attachment",
        ),
        check(
            "hardening_contracts_visible",
            "hardening_contract",
            true,
            false,
            "hardened dry-run contracts are visible for all attachment candidates",
        ),
        check(
            "denial_readback_chain_visible",
            "readback_chain",
            true,
            false,
            "denial readback, audit index, and non-persistence proof are present",
        ),
        check(
            "deterministic_decision_keys_visible",
            "decision_key",
            true,
            false,
            "candidate decisions can be keyed deterministically before any live hook exists",
        ),
        check(
            "trace_evidence_fields_visible",
            "trace_evidence",
            true,
            false,
            "trace and evidence references are present but not persisted",
        ),
        check(
            "shadow_replay_no_execution_closeout_visible",
            "replay_diff_boundary",
            true,
            false,
            "shadow replay/diff no-execution closeout is already in the prior chain",
        ),
        check(
            "live_hook_authorization_missing",
            "live_hook_boundary",
            false,
            true,
            "live blocking hooks cannot be installed without explicit authorization",
        ),
        check(
            "runtime_interception_authorization_missing",
            "runtime_boundary",
            false,
            true,
            "runtime interception remains disabled for every candidate entrypoint",
        ),
        check(
            "scheduler_guardrail_enforcement_authorization_missing",
            "scheduler_guardrail_boundary",
            false,
            true,
            "scheduler admission and guardrail blocking remain dry-run only",
        ),
        check(
            "work_graph_persistence_authorization_missing",
            "work_graph_persistence_boundary",
            false,
            true,
            "WorkGraph event store and projection persistence remain disabled",
        ),
        check(
            "task_result_live_acceptance_missing",
            "task_result_boundary",
            false,
            true,
            "TaskResultEnvelope emission remains report-only and cannot become live output",
        ),
        check(
            "lease_work_start_authorization_missing",
            "lease_work_start_boundary",
            false,
            true,
            "candidate attachment cannot acquire leases or start entrypoint work",
        ),
        check(
            "config_flag_traffic_authorization_missing",
            "config_flag_traffic_boundary",
            false,
            true,
            "config writes, feature flag mutation, and canary traffic remain disallowed",
        ),
        check(
            "operator_approval_cutover_authorization_missing",
            "operator_live_boundary",
            false,
            true,
            "operator review, approval recording, and live cutover remain absent",
        ),
        check(
            "replay_rollback_rehearsal_execution_missing",
            "replay_rollback_boundary",
            false,
            true,
            "replay, replay diff recording, rollback, and idempotency mutation remain disabled",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_blockers()
-> Vec<WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionBlockerPreview> {
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_report();
    let mut blockers = vec![
        blocker(
            "attachability_readiness_record_blocked",
            "record_live_attachment_attachability_precondition_readiness",
        ),
        blocker(
            "attachability_readiness_persistence_blocked",
            "persist_live_attachment_attachability_precondition_readiness",
        ),
        blocker(
            "attachability_readiness_acceptance_blocked",
            "accept_live_attachment_attachability_precondition_readiness",
        ),
        blocker(
            "attach_live_blocking_hook_to_entrypoints_blocked",
            "attach_live_blocking_hook_to_entrypoints",
        ),
        blocker(
            "promote_attachability_readiness_to_live_blocked",
            "promote_attachability_readiness_to_live",
        ),
    ];
    blockers.extend(
        source
            .final_closeout_blockers
            .iter()
            .map(|source_blocker| blocker(source_blocker.id, source_blocker.blocked_action)),
    );
    blockers
}

pub fn work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_required_prior_gates()
-> Vec<&'static str> {
    let source =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_report();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    required_prior_gates
}

impl WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            readiness_recorded: false,
            readiness_persisted: false,
            readiness_accepted: false,
            final_closeout_recorded: false,
            final_closeout_persisted: false,
            final_closeout_accepted: false,
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
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityEntrypointReadinessPreview {
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityEntrypointReadinessPreview {
        id,
        surface,
        attachability_status: "candidate_but_blocked",
        required_contracts: vec![
            "deterministic_decision_key",
            "trace_evidence_ref",
            "report_only_task_result_preview",
            "shadow_event_join_preview",
            "non_persistence_readback",
        ],
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
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionCheckPreview {
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionCheckPreview {
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
) -> WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionBlockerPreview {
    WorkGraphSchedulerGuardrailLiveAttachmentAttachabilityPreconditionBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "scheduler/guardrail live attachment attachability readiness cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_attachment_attachability_readiness_derives_from_terminal_closeout() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_report();

        assert_eq!(
            report.source_terminal_no_attachment_final_closeout_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_GATE
        );
        assert_eq!(report.source_final_closeout_entry_count, 9);
        assert_eq!(report.source_final_closeout_blocker_count, 45);
        assert_eq!(report.source_required_prior_gate_count, 20);
        assert_eq!(report.attachability_entrypoint_count, 4);
        assert_eq!(report.attachability_precondition_check_count, 16);
        assert_eq!(report.attachability_precondition_satisfied_count, 7);
        assert_eq!(report.attachability_precondition_unsatisfied_count, 9);
        assert_eq!(report.blocking_precondition_count, 9);
        assert_eq!(report.attachability_blocker_count, 50);
        assert_eq!(report.required_prior_gate_count, 21);
    }

    #[test]
    fn live_attachment_attachability_readiness_keeps_candidates_report_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_report();

        assert_eq!(
            report
                .attachability_entrypoints
                .iter()
                .map(|entrypoint| entrypoint.id)
                .collect::<Vec<_>>(),
            vec![
                "spawn_agent",
                "spawn_agents_on_csv",
                "task_board_claim",
                "worker_task_run",
            ]
        );
        assert!(report.attachability_entrypoints.iter().all(|entrypoint| {
            entrypoint.live_attachment_candidate
                && entrypoint.report_only
                && entrypoint.attachability_status == "candidate_but_blocked"
                && !entrypoint.live_attachment_allowed
                && !entrypoint.runtime_interception_allowed
                && entrypoint.required_contracts.len() == 5
        }));
    }

    #[test]
    fn live_attachment_attachability_readiness_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_report();

        assert!(report.readiness_visible);
        assert!(!report.readiness_recorded);
        assert!(!report.readiness_persisted);
        assert!(!report.readiness_authoritative);
        assert!(!report.readiness_accepted);
        assert!(report.terminal_no_attachment_branch_closed);
        assert!(report.attachability_candidates_identified);
        assert!(!report.attachability_preconditions_satisfied);
        assert!(report.source_final_closeout_visible);
        assert!(!report.source_final_closeout_persisted);
        assert!(
            report
                .attachability_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(
            report
                .attachability_precondition_checks
                .iter()
                .all(|check| { check.required && (check.satisfied || check.blocking) })
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
        assert!(report.ready_for_attachability_precondition_readiness_readback);
        assert!(!report.ready_for_live_attachment);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn live_attachment_attachability_readiness_links_priors_and_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_report();

        assert_eq!(
            report.required_prior_gates,
            work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_required_prior_gates()
        );
        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_PRECONDITION_MATRIX_DENIAL_READBACK_TERMINAL_NO_ATTACHMENT_FINAL_CLOSEOUT_GATE
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_LIVE_ATTACHMENT_ATTACHABILITY_PRECONDITION_READINESS_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointLiveAttachmentAttachabilityPreconditionReadinessSideEffects::none()
        );
    }
}
