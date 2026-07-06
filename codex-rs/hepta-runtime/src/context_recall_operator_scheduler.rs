use hepta_core::HeptaError;
use serde::Serialize;

use crate::OperatorPolicyDecision;
use crate::OperatorPolicyEvaluationReport;
use crate::OperatorPolicyInput;
use crate::RuntimeKernel;
use crate::evaluate_operator_policy;
use crate::worker_tasks::WorkerTaskContextRecallDueRunReport;
use crate::worker_tasks::WorkerTaskContextRecallHandoffPolicy;
use crate::worker_tasks::WorkerTaskContextRecallReadyRunReport;

const CONTEXT_RECALL_OPERATOR_INVOCATION_SURFACE: &str =
    "hepta-context-recall-worker-scheduler-operator-invocation";
const CONTEXT_RECALL_OPERATOR_INVOCATION_COMMAND: &str =
    "/hepta-context-recall-worker-scheduler-handoff --execute --json";
const CONTEXT_RECALL_OPERATOR_TOOL_NAME: &str = "hepta_context_recall_worker_scheduler_handoff";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskContextRecallOperatorSchedulerKind {
    Ready,
    Due,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerTaskContextRecallOperatorSchedulerRequest {
    pub scheduler: WorkerTaskContextRecallOperatorSchedulerKind,
    pub operator_id: String,
    pub operator_confirmed: bool,
    pub policy_allowed: bool,
    pub idempotency_key: String,
    pub ready_limit: Option<usize>,
    pub now_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskContextRecallOperatorSchedulerReport {
    pub scheduler: WorkerTaskContextRecallOperatorSchedulerKind,
    pub status: &'static str,
    pub operator_confirmed: bool,
    pub policy_allowed: bool,
    pub operator_identity_redacted: bool,
    pub idempotency_key_present: bool,
    pub context_recall_handoff_policy: WorkerTaskContextRecallHandoffPolicy,
    pub scheduler_executed: bool,
    pub ready_scheduler_ran: bool,
    pub due_scheduler_ran: bool,
    pub ready_limit: Option<usize>,
    pub candidate_count: usize,
    pub ready_count: usize,
    pub due_count: usize,
    pub ran_count: usize,
    pub skipped_count: usize,
    pub blocked_count: usize,
    pub provider_rollup_present_count: usize,
    pub selected_snippets_present_count: usize,
    pub selected_snippet_count: u32,
    pub selected_snippet_text_exposed: bool,
    pub source_ids_exposed: bool,
    pub query_payload_exposed: bool,
    pub stable_schema_promoted: bool,
    pub blockers: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerTaskContextRecallOperatorInvocationRequest {
    pub scheduler: WorkerTaskContextRecallOperatorSchedulerKind,
    pub channel_id: String,
    pub sender_id: String,
    pub sender_is_owner: bool,
    pub operator_id: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub ready_limit: Option<usize>,
    pub now_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskContextRecallOperatorInvocationReport {
    pub invocation_surface: &'static str,
    pub source_command: &'static str,
    pub scheduler: WorkerTaskContextRecallOperatorSchedulerKind,
    pub operator_identity_redacted: bool,
    pub sender_identity_redacted: bool,
    pub idempotency_key_present: bool,
    pub operator_confirmed: bool,
    pub operator_policy_decision: OperatorPolicyDecision,
    pub operator_policy_decision_label: &'static str,
    pub operator_policy_allowed: bool,
    pub operator_policy_requires_approval: bool,
    pub operator_policy_denied_reason_count: usize,
    pub scheduler_report: WorkerTaskContextRecallOperatorSchedulerReport,
    pub selected_snippet_text_exposed: bool,
    pub source_ids_exposed: bool,
    pub query_payload_exposed: bool,
    pub stable_schema_promoted: bool,
}

impl RuntimeKernel {
    pub async fn run_worker_scheduler_with_context_recall_operator_invocation(
        &self,
        request: WorkerTaskContextRecallOperatorInvocationRequest,
    ) -> Result<WorkerTaskContextRecallOperatorInvocationReport, HeptaError> {
        let operator_id = normalize_non_empty(&request.operator_id, "operator id")?;
        let idempotency_key = normalize_non_empty(&request.idempotency_key, "idempotency key")?;
        let policy = evaluate_context_recall_operator_invocation_policy(&request)?;
        let policy_allowed = policy.decision == OperatorPolicyDecision::Allow;

        let scheduler_report = self
            .run_worker_scheduler_with_context_recall_operator_handoff(
                WorkerTaskContextRecallOperatorSchedulerRequest {
                    scheduler: request.scheduler,
                    operator_id,
                    operator_confirmed: request.operator_confirmed,
                    policy_allowed,
                    idempotency_key,
                    ready_limit: request.ready_limit,
                    now_unix_ms: request.now_unix_ms,
                },
            )
            .await?;

        Ok(WorkerTaskContextRecallOperatorInvocationReport {
            invocation_surface: CONTEXT_RECALL_OPERATOR_INVOCATION_SURFACE,
            source_command: CONTEXT_RECALL_OPERATOR_INVOCATION_COMMAND,
            scheduler: request.scheduler,
            operator_identity_redacted: true,
            sender_identity_redacted: true,
            idempotency_key_present: true,
            operator_confirmed: request.operator_confirmed,
            operator_policy_decision: policy.decision,
            operator_policy_decision_label: policy.decision_label,
            operator_policy_allowed: policy_allowed,
            operator_policy_requires_approval: policy.requires_approval,
            operator_policy_denied_reason_count: policy.denied_reasons.len(),
            scheduler_report,
            selected_snippet_text_exposed: false,
            source_ids_exposed: false,
            query_payload_exposed: false,
            stable_schema_promoted: false,
        })
    }

    pub async fn run_worker_scheduler_with_context_recall_operator_handoff(
        &self,
        request: WorkerTaskContextRecallOperatorSchedulerRequest,
    ) -> Result<WorkerTaskContextRecallOperatorSchedulerReport, HeptaError> {
        let _operator_id = normalize_non_empty(&request.operator_id, "operator id")?;
        let _idempotency_key = normalize_non_empty(&request.idempotency_key, "idempotency key")?;

        let mut blockers = Vec::new();
        if !request.operator_confirmed {
            blockers.push("operator_not_confirmed");
        }
        if !request.policy_allowed {
            blockers.push("policy_not_allowed");
        }
        if !blockers.is_empty() {
            return Ok(blocked_report(request, blockers));
        }

        match request.scheduler {
            WorkerTaskContextRecallOperatorSchedulerKind::Ready => {
                let report = self
                    .run_ready_worker_tasks_with_context_recall_handoff(
                        request.ready_limit,
                        request.now_unix_ms,
                        WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
                    )
                    .await?;
                Ok(ready_report(request, report))
            }
            WorkerTaskContextRecallOperatorSchedulerKind::Due => {
                let report = self
                    .run_due_worker_tasks_with_context_recall_handoff(
                        request.now_unix_ms,
                        WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
                    )
                    .await?;
                Ok(due_report(request, report))
            }
        }
    }
}

fn evaluate_context_recall_operator_invocation_policy(
    request: &WorkerTaskContextRecallOperatorInvocationRequest,
) -> Result<OperatorPolicyEvaluationReport, HeptaError> {
    evaluate_operator_policy(OperatorPolicyInput {
        channel_id: request.channel_id.clone(),
        sender_id: request.sender_id.clone(),
        sender_is_owner: request.sender_is_owner,
        tool_name: CONTEXT_RECALL_OPERATOR_TOOL_NAME.to_string(),
        tool_action: "run".to_string(),
        current_session_id: None,
        target_session_id: None,
        message_cross_context_allowed: false,
        message_action_allowed: false,
        provider_auth_ref: None,
        pairing_request_kind: None,
        pairing_provenance_verified: false,
        target_path: None,
        sandbox_mode: None,
        workspace_mount_path: None,
        payload_preview: Some("context-recall worker scheduler handoff request".to_string()),
        terminal_output_preview: None,
    })
}

fn blocked_report(
    request: WorkerTaskContextRecallOperatorSchedulerRequest,
    blockers: Vec<&'static str>,
) -> WorkerTaskContextRecallOperatorSchedulerReport {
    WorkerTaskContextRecallOperatorSchedulerReport {
        scheduler: request.scheduler,
        status: "blocked",
        operator_confirmed: request.operator_confirmed,
        policy_allowed: request.policy_allowed,
        operator_identity_redacted: true,
        idempotency_key_present: true,
        context_recall_handoff_policy: WorkerTaskContextRecallHandoffPolicy::Disabled,
        scheduler_executed: false,
        ready_scheduler_ran: false,
        due_scheduler_ran: false,
        ready_limit: request.ready_limit,
        candidate_count: 0,
        ready_count: 0,
        due_count: 0,
        ran_count: 0,
        skipped_count: 0,
        blocked_count: 0,
        provider_rollup_present_count: 0,
        selected_snippets_present_count: 0,
        selected_snippet_count: 0,
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        stable_schema_promoted: false,
        blockers,
    }
}

fn ready_report(
    request: WorkerTaskContextRecallOperatorSchedulerRequest,
    report: WorkerTaskContextRecallReadyRunReport,
) -> WorkerTaskContextRecallOperatorSchedulerReport {
    WorkerTaskContextRecallOperatorSchedulerReport {
        scheduler: request.scheduler,
        status: "executed",
        operator_confirmed: request.operator_confirmed,
        policy_allowed: request.policy_allowed,
        operator_identity_redacted: true,
        idempotency_key_present: true,
        context_recall_handoff_policy:
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
        scheduler_executed: true,
        ready_scheduler_ran: true,
        due_scheduler_ran: false,
        ready_limit: report.limit,
        candidate_count: report.candidate_count,
        ready_count: report.ready_count,
        due_count: 0,
        ran_count: report.ran_count,
        skipped_count: 0,
        blocked_count: report.blocked_count,
        provider_rollup_present_count: report
            .runs
            .iter()
            .filter(|run| run.provider_rollup.is_some())
            .count(),
        selected_snippets_present_count: report.selected_snippets_present_count,
        selected_snippet_count: report.selected_snippet_count,
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        stable_schema_promoted: false,
        blockers: Vec::new(),
    }
}

fn due_report(
    request: WorkerTaskContextRecallOperatorSchedulerRequest,
    report: WorkerTaskContextRecallDueRunReport,
) -> WorkerTaskContextRecallOperatorSchedulerReport {
    WorkerTaskContextRecallOperatorSchedulerReport {
        scheduler: request.scheduler,
        status: "executed",
        operator_confirmed: request.operator_confirmed,
        policy_allowed: request.policy_allowed,
        operator_identity_redacted: true,
        idempotency_key_present: true,
        context_recall_handoff_policy:
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
        scheduler_executed: true,
        ready_scheduler_ran: false,
        due_scheduler_ran: true,
        ready_limit: request.ready_limit,
        candidate_count: 0,
        ready_count: 0,
        due_count: report.due_count,
        ran_count: report.ran_count,
        skipped_count: report.skipped_count,
        blocked_count: 0,
        provider_rollup_present_count: report
            .runs
            .iter()
            .filter(|run| run.provider_rollup.is_some())
            .count(),
        selected_snippets_present_count: report.selected_snippets_present_count,
        selected_snippet_count: report.selected_snippet_count,
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        stable_schema_promoted: false,
        blockers: Vec::new(),
    }
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!("{label} is required")));
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use hepta_core::MemoryRecord;
    use hepta_core::MemoryScope;
    use hepta_core::MemoryStore;

    use super::*;

    fn assert_operator_report_does_not_leak(rendered: &str) {
        for forbidden in [
            "operator-scheduler-ready-safe-context",
            "operator-scheduler-due-safe-context",
            "operator-scheduler-ready-source-id",
            "operator-scheduler-due-source-id",
            "operator-a",
            "context-recall-scheduler",
            "telegram:6476198178",
            "6476198178",
            "[redacted-query]",
            "<selected_context_recall>",
        ] {
            assert!(
                !rendered.contains(forbidden),
                "operator scheduler report leaked {forbidden}"
            );
        }
    }

    #[tokio::test]
    async fn worker_task_context_recall_operator_invocation_requires_owner_policy_without_scheduler_execution()
     {
        let runtime = RuntimeKernel::new();
        runtime
            .memory
            .put(MemoryRecord {
                id: "operator-scheduler-ready-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "operator-ready-needle {}",
                    "operator-scheduler-ready-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        let task = runtime
            .spawn_worker_task("ready", "operator-ready-needle", None)
            .expect("ready task should spawn");

        let blocked = runtime
            .run_worker_scheduler_with_context_recall_operator_invocation(
                WorkerTaskContextRecallOperatorInvocationRequest {
                    scheduler: WorkerTaskContextRecallOperatorSchedulerKind::Ready,
                    channel_id: "telegram:6476198178".into(),
                    sender_id: "6476198178".into(),
                    sender_is_owner: false,
                    operator_id: "operator-a".into(),
                    operator_confirmed: true,
                    idempotency_key: "context-recall-scheduler-invocation-denied-1".into(),
                    ready_limit: Some(10),
                    now_unix_ms: None,
                },
            )
            .await
            .expect("blocked invocation should return a report");

        assert_eq!(
            blocked.operator_policy_decision,
            OperatorPolicyDecision::RequireApproval
        );
        assert!(blocked.operator_policy_requires_approval);
        assert!(!blocked.operator_policy_allowed);
        assert_eq!(blocked.scheduler_report.status, "blocked");
        assert!(!blocked.scheduler_report.scheduler_executed);
        assert_eq!(
            blocked.scheduler_report.context_recall_handoff_policy,
            WorkerTaskContextRecallHandoffPolicy::Disabled
        );
        assert_eq!(
            blocked.scheduler_report.blockers,
            vec!["policy_not_allowed"]
        );
        assert_eq!(
            runtime
                .worker_task_status(&task.task.task_id)
                .expect("task should still exist")
                .task
                .status,
            crate::WorkerTaskStatus::Queued
        );

        let encoded = serde_json::to_string(&blocked).expect("report should serialize");
        assert_operator_report_does_not_leak(&encoded);
        assert_operator_report_does_not_leak(&format!("{blocked:?}"));
    }

    #[tokio::test]
    async fn worker_task_context_recall_operator_invocation_executes_owner_approved_without_leak() {
        let runtime = RuntimeKernel::new();
        runtime
            .memory
            .put(MemoryRecord {
                id: "operator-scheduler-ready-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "operator-ready-needle {}",
                    "operator-scheduler-ready-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        let task = runtime
            .spawn_worker_task("ready", "operator-ready-needle", None)
            .expect("ready task should spawn");

        let report = runtime
            .run_worker_scheduler_with_context_recall_operator_invocation(
                WorkerTaskContextRecallOperatorInvocationRequest {
                    scheduler: WorkerTaskContextRecallOperatorSchedulerKind::Ready,
                    channel_id: "telegram:6476198178".into(),
                    sender_id: "6476198178".into(),
                    sender_is_owner: true,
                    operator_id: "operator-a".into(),
                    operator_confirmed: true,
                    idempotency_key: "context-recall-scheduler-invocation-ready-1".into(),
                    ready_limit: Some(10),
                    now_unix_ms: None,
                },
            )
            .await
            .expect("owner-approved invocation should execute");

        assert_eq!(
            report.operator_policy_decision,
            OperatorPolicyDecision::Allow
        );
        assert!(report.operator_policy_allowed);
        assert!(!report.operator_policy_requires_approval);
        assert_eq!(report.scheduler_report.status, "executed");
        assert!(report.scheduler_report.scheduler_executed);
        assert!(report.scheduler_report.ready_scheduler_ran);
        assert_eq!(
            report.scheduler_report.context_recall_handoff_policy,
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved
        );
        assert_eq!(report.scheduler_report.ran_count, 1);
        assert_eq!(report.scheduler_report.provider_rollup_present_count, 1);
        assert_eq!(report.scheduler_report.selected_snippets_present_count, 1);
        assert!(report.scheduler_report.selected_snippet_count > 0);
        assert_eq!(
            runtime
                .worker_task_status(&task.task.task_id)
                .expect("task should still exist")
                .task
                .status,
            crate::WorkerTaskStatus::Completed
        );

        let encoded = serde_json::to_string(&report).expect("report should serialize");
        assert_operator_report_does_not_leak(&encoded);
        assert_operator_report_does_not_leak(&format!("{report:?}"));
    }

    #[tokio::test]
    async fn worker_task_context_recall_operator_scheduler_handoff_blocks_without_approval() {
        let runtime = RuntimeKernel::new();
        runtime
            .memory
            .put(MemoryRecord {
                id: "operator-scheduler-ready-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "operator-ready-needle {}",
                    "operator-scheduler-ready-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        let task = runtime
            .spawn_worker_task("ready", "operator-ready-needle", None)
            .expect("ready task should spawn");

        let blocked = runtime
            .run_worker_scheduler_with_context_recall_operator_handoff(
                WorkerTaskContextRecallOperatorSchedulerRequest {
                    scheduler: WorkerTaskContextRecallOperatorSchedulerKind::Ready,
                    operator_id: "operator-a".into(),
                    operator_confirmed: false,
                    policy_allowed: true,
                    idempotency_key: "context-recall-scheduler-1".into(),
                    ready_limit: Some(10),
                    now_unix_ms: None,
                },
            )
            .await
            .expect("blocked report should be returned");

        assert_eq!(blocked.status, "blocked");
        assert!(!blocked.scheduler_executed);
        assert_eq!(
            blocked.context_recall_handoff_policy,
            WorkerTaskContextRecallHandoffPolicy::Disabled
        );
        assert_eq!(blocked.selected_snippets_present_count, 0);
        assert_eq!(blocked.selected_snippet_count, 0);
        assert_eq!(blocked.blockers, vec!["operator_not_confirmed"]);
        assert_eq!(
            runtime
                .worker_task_status(&task.task.task_id)
                .expect("task should still exist")
                .task
                .status,
            crate::WorkerTaskStatus::Queued
        );

        let encoded = serde_json::to_string(&blocked).expect("report should serialize");
        assert_operator_report_does_not_leak(&encoded);
        assert_operator_report_does_not_leak(&format!("{blocked:?}"));
    }

    #[tokio::test]
    async fn worker_task_context_recall_operator_scheduler_handoff_executes_ready_and_due_without_leak()
     {
        let ready_runtime = RuntimeKernel::new();
        ready_runtime
            .memory
            .put(MemoryRecord {
                id: "operator-scheduler-ready-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "operator-ready-needle {}",
                    "operator-scheduler-ready-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        let ready_task = ready_runtime
            .spawn_worker_task("ready", "operator-ready-needle", None)
            .expect("ready task should spawn");

        let ready = ready_runtime
            .run_worker_scheduler_with_context_recall_operator_handoff(
                WorkerTaskContextRecallOperatorSchedulerRequest {
                    scheduler: WorkerTaskContextRecallOperatorSchedulerKind::Ready,
                    operator_id: "operator-a".into(),
                    operator_confirmed: true,
                    policy_allowed: true,
                    idempotency_key: "context-recall-scheduler-ready-1".into(),
                    ready_limit: Some(10),
                    now_unix_ms: None,
                },
            )
            .await
            .expect("ready scheduler should execute");

        assert_eq!(ready.status, "executed");
        assert!(ready.scheduler_executed);
        assert!(ready.ready_scheduler_ran);
        assert!(!ready.due_scheduler_ran);
        assert_eq!(
            ready.context_recall_handoff_policy,
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved
        );
        assert_eq!(ready.ran_count, 1);
        assert_eq!(ready.provider_rollup_present_count, 1);
        assert_eq!(ready.selected_snippets_present_count, 1);
        assert!(ready.selected_snippet_count > 0);
        assert_eq!(
            ready_runtime
                .worker_task_status(&ready_task.task.task_id)
                .expect("task should still exist")
                .task
                .status,
            crate::WorkerTaskStatus::Completed
        );

        let due_runtime = RuntimeKernel::new();
        due_runtime
            .memory
            .put(MemoryRecord {
                id: "operator-scheduler-due-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "operator-due-needle {}",
                    "operator-scheduler-due-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        let due_task = due_runtime
            .spawn_worker_task("scheduler", "operator-due-needle", Some("delay:10ms"))
            .expect("scheduled task should spawn");
        let due_at = due_task
            .task
            .next_run_unix_ms
            .expect("scheduled task should have next run");

        let due = due_runtime
            .run_worker_scheduler_with_context_recall_operator_handoff(
                WorkerTaskContextRecallOperatorSchedulerRequest {
                    scheduler: WorkerTaskContextRecallOperatorSchedulerKind::Due,
                    operator_id: "operator-a".into(),
                    operator_confirmed: true,
                    policy_allowed: true,
                    idempotency_key: "context-recall-scheduler-due-1".into(),
                    ready_limit: None,
                    now_unix_ms: Some(due_at),
                },
            )
            .await
            .expect("due scheduler should execute");

        assert_eq!(due.status, "executed");
        assert!(due.scheduler_executed);
        assert!(!due.ready_scheduler_ran);
        assert!(due.due_scheduler_ran);
        assert_eq!(due.due_count, 1);
        assert_eq!(due.ran_count, 1);
        assert_eq!(due.provider_rollup_present_count, 1);
        assert_eq!(due.selected_snippets_present_count, 1);
        assert!(due.selected_snippet_count > 0);

        for rendered in [
            serde_json::to_string(&ready).expect("ready report should serialize"),
            format!("{ready:?}"),
            serde_json::to_string(&due).expect("due report should serialize"),
            format!("{due:?}"),
        ] {
            assert_operator_report_does_not_leak(&rendered);
        }
    }
}
