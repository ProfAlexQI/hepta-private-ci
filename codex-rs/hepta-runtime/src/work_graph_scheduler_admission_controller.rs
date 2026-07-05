use serde::Serialize;

pub const WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_PREVIEW_GATE: &str =
    "hepta_work_graph_scheduler_admission_controller_preview_gate";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_SCHEMA_VERSION: &str =
    "work_graph_scheduler_admission_controller_preview_v1";
pub const WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_observability_timeline_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionControllerPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub check_count: usize,
    pub decision_count: usize,
    pub adapter_preview_count: usize,
    pub checks: Vec<WorkGraphSchedulerAdmissionCheckPreview>,
    pub decisions: Vec<WorkGraphSchedulerAdmissionDecisionPreview>,
    pub adapter_previews: Vec<WorkGraphSchedulerAdmissionAdapterPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_observability_timeline_preview: bool,
    pub ready_for_scheduler_cutover: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphSchedulerAdmissionControllerPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionCheckPreview {
    pub id: &'static str,
    pub required: bool,
    pub blocks_execution: bool,
    pub required_evidence_fields: Vec<&'static str>,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionDecisionPreview {
    pub id: &'static str,
    pub runnable_in_preview: bool,
    pub terminal_denial: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionAdapterPreview {
    pub source_surface_id: &'static str,
    pub target_node_kind: &'static str,
    pub source_fields: Vec<&'static str>,
    pub applied_check_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub blocker_ids: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphSchedulerAdmissionControllerPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub admission_enforcement_enabled: bool,
    pub lease_acquired: bool,
    pub work_started: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_scheduler_admission_controller_preview_report()
-> WorkGraphSchedulerAdmissionControllerPreviewReport {
    let checks = work_graph_scheduler_admission_checks();
    let decisions = work_graph_scheduler_admission_decisions();
    let adapter_previews = work_graph_scheduler_admission_adapter_previews();

    WorkGraphSchedulerAdmissionControllerPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_SCHEMA_VERSION,
        preview_mode: "dry_run_admission_explain_only_no_execution",
        check_count: checks.len(),
        decision_count: decisions.len(),
        adapter_preview_count: adapter_previews.len(),
        checks,
        decisions,
        adapter_previews,
        recommended_next_gate: WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_RECOMMENDED_NEXT_GATE,
        ready_for_observability_timeline_preview: true,
        ready_for_scheduler_cutover: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphSchedulerAdmissionControllerPreviewSideEffects::none(),
    }
}

pub fn work_graph_scheduler_admission_checks() -> Vec<WorkGraphSchedulerAdmissionCheckPreview> {
    vec![
        check(
            "dependencies_terminal_ready",
            vec!["depends_on", "dependency_statuses", "trace_id"],
            "work cannot become runnable until all blocking dependencies are terminal-ready",
        ),
        check(
            "lane_lease_available_and_owned",
            vec!["lane_id", "lease_state", "owner_agent_id"],
            "scheduler handoff must not run without a lane-owned lease boundary",
        ),
        check(
            "approval_authority_present_when_required",
            vec!["approval_id", "authority_state", "expiry_state"],
            "high-risk or external handoff work needs explicit non-expired approval evidence",
        ),
        check(
            "idempotency_replay_window_clear",
            vec!["idempotency_key_hash", "readback_evidence_id"],
            "retries must not duplicate already observed work or delivery effects",
        ),
        check(
            "budget_and_timeout_available",
            vec!["budget_state", "timeout_budget_ms", "attempt_count"],
            "work must have remaining attempt, wall-clock, and resource budget",
        ),
        check(
            "task_result_contract_preview_present",
            vec!["schema_version", "validator_ids"],
            "terminal promotion requires the TaskResult contract preview to be available first",
        ),
        check(
            "side_effect_boundary_locked",
            vec!["preview_mode", "side_effects"],
            "this gate explains allow or deny decisions without acquiring leases or starting work",
        ),
    ]
}

pub fn work_graph_scheduler_admission_decisions() -> Vec<WorkGraphSchedulerAdmissionDecisionPreview>
{
    vec![
        decision(
            "allow_preview_only",
            true,
            false,
            "all preconditions are satisfied for a dry-run explanation, not live execution",
        ),
        decision(
            "deny_blocked_dependency",
            false,
            true,
            "one or more dependencies are missing, blocked, failed, or not terminal-ready",
        ),
        decision(
            "deny_missing_lane_lease",
            false,
            true,
            "no lane-owned lease can be proven for the target work item",
        ),
        decision(
            "deny_missing_required_approval",
            false,
            true,
            "required operator approval is missing, expired, superseded, or out of scope",
        ),
        decision(
            "deny_idempotency_conflict",
            false,
            true,
            "readback evidence or idempotency key indicates a duplicate effect risk",
        ),
        decision(
            "deny_budget_or_timeout_exhausted",
            false,
            true,
            "attempt, resource, token, or wall-clock budget is exhausted",
        ),
        decision(
            "deny_task_result_contract_missing",
            false,
            true,
            "terminal work cannot be admitted if the TaskResult preview contract is absent",
        ),
    ]
}

pub fn work_graph_scheduler_admission_adapter_previews()
-> Vec<WorkGraphSchedulerAdmissionAdapterPreview> {
    let all_checks = work_graph_scheduler_admission_check_ids();

    vec![
        adapter(
            "hepta_runtime_scheduler_store",
            "scheduler_run",
            vec![
                "job_id",
                "run_id",
                "status",
                "idempotency_key",
                "readback_evidence_id",
            ],
            all_checks.clone(),
            vec!["scheduler_run_admission_not_enforced"],
        ),
        adapter(
            "hepta_runtime_task_board",
            "worker_task",
            vec![
                "task_id",
                "status",
                "depends_on",
                "claim_token",
                "lease_expires_at",
            ],
            all_checks.clone(),
            vec!["task_board_admission_not_enforced"],
        ),
        adapter(
            "hepta_runtime_worker_tasks",
            "worker_task",
            vec![
                "task_id",
                "status",
                "depends_on",
                "attempt_count",
                "timeout_budget_ms",
            ],
            all_checks.clone(),
            vec!["worker_task_admission_not_enforced"],
        ),
        adapter(
            "multi_agent_v2_thread_spawn",
            "agent_task",
            vec!["agent_path", "thread_id", "parent_thread_id", "role_id"],
            all_checks.clone(),
            vec!["agent_task_admission_not_enforced"],
        ),
        adapter(
            "agent_jobs_batch_workers",
            "worker_task",
            vec![
                "job_id",
                "item_id",
                "assigned_thread_id",
                "attempt_count",
                "max_runtime_seconds",
            ],
            all_checks,
            vec!["agent_job_item_admission_not_enforced"],
        ),
    ]
}

impl WorkGraphSchedulerAdmissionControllerPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            admission_enforcement_enabled: false,
            lease_acquired: false,
            work_started: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_scheduler_admission_check_ids() -> Vec<&'static str> {
    work_graph_scheduler_admission_checks()
        .iter()
        .map(|check| check.id)
        .collect()
}

fn check(
    id: &'static str,
    required_evidence_fields: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphSchedulerAdmissionCheckPreview {
    WorkGraphSchedulerAdmissionCheckPreview {
        id,
        required: true,
        blocks_execution: true,
        required_evidence_fields,
        reason,
    }
}

fn decision(
    id: &'static str,
    runnable_in_preview: bool,
    terminal_denial: bool,
    reason: &'static str,
) -> WorkGraphSchedulerAdmissionDecisionPreview {
    WorkGraphSchedulerAdmissionDecisionPreview {
        id,
        runnable_in_preview,
        terminal_denial,
        reason,
    }
}

fn adapter(
    source_surface_id: &'static str,
    target_node_kind: &'static str,
    source_fields: Vec<&'static str>,
    applied_check_ids: Vec<&'static str>,
    blocker_ids: Vec<&'static str>,
) -> WorkGraphSchedulerAdmissionAdapterPreview {
    WorkGraphSchedulerAdmissionAdapterPreview {
        source_surface_id,
        target_node_kind,
        source_fields,
        applied_check_ids,
        enforcement_enabled: false,
        blocker_ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_admission_preview_declares_required_checks() {
        let report = hepta_work_graph_scheduler_admission_controller_preview_report();
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
        assert!(report.checks.iter().all(|check| check.required));
        assert!(report.checks.iter().all(|check| check.blocks_execution));
    }

    #[test]
    fn scheduler_admission_preview_keeps_execution_disabled() {
        let report = hepta_work_graph_scheduler_admission_controller_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphSchedulerAdmissionControllerPreviewSideEffects::none()
        );
        assert!(report.ready_for_observability_timeline_preview);
        assert!(!report.ready_for_scheduler_cutover);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .adapter_previews
                .iter()
                .all(|adapter| !adapter.enforcement_enabled)
        );
    }

    #[test]
    fn scheduler_admission_preview_names_allow_and_denial_decisions() {
        let report = hepta_work_graph_scheduler_admission_controller_preview_report();
        let decision_ids = report
            .decisions
            .iter()
            .map(|decision| decision.id)
            .collect::<Vec<_>>();

        assert_eq!(
            decision_ids,
            [
                "allow_preview_only",
                "deny_blocked_dependency",
                "deny_missing_lane_lease",
                "deny_missing_required_approval",
                "deny_idempotency_conflict",
                "deny_budget_or_timeout_exhausted",
                "deny_task_result_contract_missing",
            ]
        );
        assert_eq!(report.decision_count, 7);
        assert!(
            report
                .decisions
                .iter()
                .filter(|decision| decision.id.starts_with("deny_"))
                .all(|decision| decision.terminal_denial && !decision.runnable_in_preview)
        );
    }

    #[test]
    fn scheduler_admission_preview_projects_existing_scheduling_surfaces() {
        let report = hepta_work_graph_scheduler_admission_controller_preview_report();
        let adapter_ids = report
            .adapter_previews
            .iter()
            .map(|adapter| adapter.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            adapter_ids,
            [
                "hepta_runtime_scheduler_store",
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
            ]
        );
        assert_eq!(report.adapter_preview_count, 5);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_RECOMMENDED_NEXT_GATE
        );
    }
}
