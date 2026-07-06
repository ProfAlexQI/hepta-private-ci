use serde::Serialize;

use crate::work_graph_scheduler_admission_controller::work_graph_scheduler_admission_checks;
use crate::work_graph_task_result_envelope_report_only_validator::WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE;

pub const WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE: &str =
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_SCHEMA_VERSION: &str =
    "work_graph_scheduler_admission_dry_run_enforcement_v1";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_event_store_shadow_path_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionDryRunEnforcementReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub entrypoint_count: usize,
    pub check_count: usize,
    pub decision_count: usize,
    pub explanation_count: usize,
    pub entrypoints: Vec<WorkGraphSchedulerAdmissionEntrypointPreview>,
    pub checks: Vec<WorkGraphSchedulerAdmissionDryRunCheckPreview>,
    pub decisions: Vec<WorkGraphSchedulerAdmissionDryRunDecisionPreview>,
    pub explanations: Vec<WorkGraphSchedulerAdmissionDryRunExplanationPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub dry_run_enforcement_enabled: bool,
    pub live_blocking_enforcement_enabled: bool,
    pub ready_for_append_only_event_store_shadow_path: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionEntrypointPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub entrypoint_kind: &'static str,
    pub admission_position: &'static str,
    pub required_input_fields: Vec<&'static str>,
    pub applied_check_ids: Vec<&'static str>,
    pub explanation_output_fields: Vec<&'static str>,
    pub dry_run_enforcement_enabled: bool,
    pub live_blocking_enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionDryRunCheckPreview {
    pub id: &'static str,
    pub blocks_live_execution: bool,
    pub explanation_required: bool,
    pub required_evidence_fields: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionDryRunDecisionPreview {
    pub id: &'static str,
    pub outcome: &'static str,
    pub allow_entrypoint_to_continue: bool,
    pub live_execution_blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionDryRunExplanationPreview {
    pub entrypoint_id: &'static str,
    pub decision_id: &'static str,
    pub allow: bool,
    pub explanation: &'static str,
    pub trace_id: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub live_admission_enforcement_enabled: bool,
    pub lease_acquired: bool,
    pub work_started: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_scheduler_admission_dry_run_enforcement_report()
-> WorkGraphSchedulerAdmissionDryRunEnforcementReport {
    let entrypoints = work_graph_scheduler_admission_dry_run_entrypoints();
    let checks = work_graph_scheduler_admission_dry_run_checks();
    let decisions = work_graph_scheduler_admission_dry_run_decisions();
    let explanations = work_graph_scheduler_admission_dry_run_explanations();

    WorkGraphSchedulerAdmissionDryRunEnforcementReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
        schema_version: WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_SCHEMA_VERSION,
        preview_mode: "dry_run_allow_deny_explanation_before_entrypoint_no_live_blocking",
        entrypoint_count: entrypoints.len(),
        check_count: checks.len(),
        decision_count: decisions.len(),
        explanation_count: explanations.len(),
        entrypoints,
        checks,
        decisions,
        explanations,
        required_prior_gates: vec![WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE],
        recommended_next_gate:
            WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_RECOMMENDED_NEXT_GATE,
        dry_run_enforcement_enabled: true,
        live_blocking_enforcement_enabled: false,
        ready_for_append_only_event_store_shadow_path: true,
        ready_for_live_execution: false,
        side_effects: WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects::none(),
    }
}

pub fn work_graph_scheduler_admission_dry_run_entrypoints()
-> Vec<WorkGraphSchedulerAdmissionEntrypointPreview> {
    let check_ids = work_graph_scheduler_admission_dry_run_check_ids();
    let explanation_fields = vec![
        "decision",
        "reason",
        "failedChecks",
        "requiredEvidence",
        "taskResultPreview",
        "traceId",
    ];

    vec![
        entrypoint(
            "spawn_agent",
            "multi_agent_v2_thread_spawn",
            "tool",
            "before spawn_agent calls agent_control.spawn_agent_with_metadata",
            vec![
                "task_name",
                "agent_type",
                "model",
                "service_tier",
                "trace_id",
            ],
            check_ids.clone(),
            explanation_fields.clone(),
        ),
        entrypoint(
            "spawn_agents_on_csv",
            "agent_jobs_batch_workers",
            "tool",
            "before CSV fanout creates/runs agent job items",
            vec![
                "job_id",
                "csv_path",
                "max_concurrency",
                "max_runtime_seconds",
                "trace_id",
            ],
            check_ids.clone(),
            explanation_fields.clone(),
        ),
        entrypoint(
            "task_board_claim",
            "hepta_runtime_task_board",
            "runtime",
            "before task board claim acquires or refreshes a lease",
            vec![
                "task_id",
                "depends_on",
                "claim_token",
                "lease_expires_at",
                "trace_id",
            ],
            check_ids.clone(),
            explanation_fields.clone(),
        ),
        entrypoint(
            "worker_task_run",
            "hepta_runtime_worker_tasks",
            "runtime",
            "before worker task run starts command, tool, or agent work",
            vec![
                "task_id",
                "attempt_count",
                "timeout_budget_ms",
                "side_effect_class",
                "trace_id",
            ],
            check_ids,
            explanation_fields,
        ),
    ]
}

pub fn work_graph_scheduler_admission_dry_run_checks()
-> Vec<WorkGraphSchedulerAdmissionDryRunCheckPreview> {
    work_graph_scheduler_admission_checks()
        .into_iter()
        .map(|check| WorkGraphSchedulerAdmissionDryRunCheckPreview {
            id: check.id,
            blocks_live_execution: true,
            explanation_required: true,
            required_evidence_fields: check.required_evidence_fields,
        })
        .collect()
}

pub fn work_graph_scheduler_admission_dry_run_decisions()
-> Vec<WorkGraphSchedulerAdmissionDryRunDecisionPreview> {
    vec![
        decision(
            "allow_dry_run",
            "allow",
            true,
            false,
            "all checks are satisfied for dry-run continuation; no live authority is granted",
        ),
        decision(
            "deny_dependencies_not_ready",
            "deny",
            false,
            true,
            "one or more blocking dependencies are missing or not terminal-ready",
        ),
        decision(
            "deny_lease_unavailable",
            "deny",
            false,
            true,
            "lane lease is missing, expired, or owned by another worker",
        ),
        decision(
            "deny_approval_missing",
            "deny",
            false,
            true,
            "approval is required for this risk class and no valid approval is attached",
        ),
        decision(
            "deny_idempotency_conflict",
            "deny",
            false,
            true,
            "idempotency readback indicates a duplicate or replay conflict",
        ),
        decision(
            "deny_budget_exhausted",
            "deny",
            false,
            true,
            "attempt, token, command, wall-clock, or concurrency budget is exhausted",
        ),
        decision(
            "deny_task_result_preview_missing",
            "deny",
            false,
            true,
            "entrypoint cannot run without a TaskResultEnvelope preview path",
        ),
        decision(
            "deny_side_effect_boundary_open",
            "deny",
            false,
            true,
            "requested side-effect class is not covered by an allowed boundary",
        ),
    ]
}

pub fn work_graph_scheduler_admission_dry_run_explanations()
-> Vec<WorkGraphSchedulerAdmissionDryRunExplanationPreview> {
    vec![
        explanation(
            "spawn_agent",
            "allow_dry_run",
            true,
            "spawn_agent may continue in dry-run-admitted mode with trace-bound TaskResult preview",
            "trace-admission-spawn-agent-preview-001",
        ),
        explanation(
            "spawn_agents_on_csv",
            "allow_dry_run",
            true,
            "spawn_agents_on_csv may fan out only after budget, lease, idempotency, and result envelope checks pass",
            "trace-admission-agent-jobs-preview-001",
        ),
        explanation(
            "task_board_claim",
            "deny_lease_unavailable",
            false,
            "task_board claim would be denied when the lane lease is absent or stale",
            "trace-admission-task-board-preview-001",
        ),
        explanation(
            "worker_task_run",
            "deny_side_effect_boundary_open",
            false,
            "worker task run would be denied when the side-effect boundary is not locked",
            "trace-admission-worker-task-preview-001",
        ),
    ]
}

impl WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            live_admission_enforcement_enabled: false,
            lease_acquired: false,
            work_started: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_scheduler_admission_dry_run_check_ids() -> Vec<&'static str> {
    work_graph_scheduler_admission_dry_run_checks()
        .iter()
        .map(|check| check.id)
        .collect()
}

fn entrypoint(
    id: &'static str,
    source_surface_id: &'static str,
    entrypoint_kind: &'static str,
    admission_position: &'static str,
    required_input_fields: Vec<&'static str>,
    applied_check_ids: Vec<&'static str>,
    explanation_output_fields: Vec<&'static str>,
) -> WorkGraphSchedulerAdmissionEntrypointPreview {
    WorkGraphSchedulerAdmissionEntrypointPreview {
        id,
        source_surface_id,
        entrypoint_kind,
        admission_position,
        required_input_fields,
        applied_check_ids,
        explanation_output_fields,
        dry_run_enforcement_enabled: true,
        live_blocking_enforcement_enabled: false,
    }
}

fn decision(
    id: &'static str,
    outcome: &'static str,
    allow_entrypoint_to_continue: bool,
    live_execution_blocked: bool,
    reason: &'static str,
) -> WorkGraphSchedulerAdmissionDryRunDecisionPreview {
    WorkGraphSchedulerAdmissionDryRunDecisionPreview {
        id,
        outcome,
        allow_entrypoint_to_continue,
        live_execution_blocked,
        reason,
    }
}

fn explanation(
    entrypoint_id: &'static str,
    decision_id: &'static str,
    allow: bool,
    explanation: &'static str,
    trace_id: &'static str,
) -> WorkGraphSchedulerAdmissionDryRunExplanationPreview {
    WorkGraphSchedulerAdmissionDryRunExplanationPreview {
        entrypoint_id,
        decision_id,
        allow,
        explanation,
        trace_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_admission_dry_run_covers_requested_entrypoints() {
        let report = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
        let entrypoint_ids = report
            .entrypoints
            .iter()
            .map(|entrypoint| entrypoint.id)
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
        assert_eq!(report.entrypoint_count, 4);
        assert!(
            report
                .entrypoints
                .iter()
                .all(|entrypoint| entrypoint.dry_run_enforcement_enabled
                    && !entrypoint.live_blocking_enforcement_enabled)
        );
    }

    #[test]
    fn scheduler_admission_dry_run_reuses_controller_checks() {
        let report = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
        let check_ids = report
            .checks
            .iter()
            .map(|check| check.id)
            .collect::<Vec<_>>();

        assert_eq!(
            check_ids,
            [
                "dependencies_terminal_ready",
                "lane_lease_available_and_owned",
                "approval_authority_present_when_required",
                "idempotency_replay_window_clear",
                "budget_and_timeout_available",
                "task_result_contract_preview_present",
                "side_effect_boundary_locked",
            ]
        );
        assert_eq!(report.check_count, 7);
        assert!(
            report
                .entrypoints
                .iter()
                .all(|entrypoint| entrypoint.applied_check_ids == check_ids)
        );
    }

    #[test]
    fn scheduler_admission_dry_run_emits_allow_and_deny_explanations() {
        let report = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
        let explanation_ids = report
            .explanations
            .iter()
            .map(|explanation| (explanation.entrypoint_id, explanation.decision_id))
            .collect::<Vec<_>>();

        assert_eq!(
            explanation_ids,
            [
                ("spawn_agent", "allow_dry_run"),
                ("spawn_agents_on_csv", "allow_dry_run"),
                ("task_board_claim", "deny_lease_unavailable"),
                ("worker_task_run", "deny_side_effect_boundary_open"),
            ]
        );
        assert_eq!(report.explanation_count, 4);
        assert!(
            report
                .explanations
                .iter()
                .all(|explanation| !explanation.trace_id.is_empty())
        );
    }

    #[test]
    fn scheduler_admission_dry_run_keeps_live_execution_disabled() {
        let report = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();

        assert_eq!(
            report.side_effects,
            WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects::none()
        );
        assert!(report.dry_run_enforcement_enabled);
        assert!(!report.live_blocking_enforcement_enabled);
        assert!(report.ready_for_append_only_event_store_shadow_path);
        assert!(!report.ready_for_live_execution);
    }
}
