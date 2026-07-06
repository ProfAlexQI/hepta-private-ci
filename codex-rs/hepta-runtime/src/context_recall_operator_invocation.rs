use hepta_core::HeptaError;
use serde::Serialize;

use crate::AgentReducerMode;
use crate::AgentRuntimeContextRecallHandoffPolicy;
use crate::MultiAgentContextRecallOperatorInvocationReport;
use crate::MultiAgentContextRecallOperatorInvocationRequest;
use crate::OperatorPolicyDecision;
use crate::RuntimeKernel;
use crate::WorkerTaskContextRecallHandoffPolicy;
use crate::WorkerTaskContextRecallOperatorInvocationReport;
use crate::WorkerTaskContextRecallOperatorInvocationRequest;
use crate::WorkerTaskContextRecallOperatorSchedulerKind;

const CONTEXT_RECALL_OPERATOR_INVOCATION_SURFACE: &str =
    "hepta-context-recall-runtime-operator-invocation";
const CONTEXT_RECALL_OPERATOR_INVOCATION_COMMAND: &str =
    "/hepta-context-recall-handoff --execute --json";
const CONTEXT_RECALL_OPERATOR_COMMAND_SURFACE: &str =
    "hepta-context-recall-runtime-operator-command";
const CONTEXT_RECALL_OPERATOR_COMMAND_SOURCE: &str = "/hepta-context-recall-handoff --execute --json --target <worker-ready|worker-due|multi-agent-ready>";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextRecallOperatorInvocationTarget {
    WorkerReady,
    WorkerDue,
    MultiAgentReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextRecallOperatorInvocationHandoffPolicy {
    Disabled,
    ExperimentalOperatorApproved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextRecallOperatorInvocationRequest {
    pub target: ContextRecallOperatorInvocationTarget,
    pub channel_id: String,
    pub sender_id: String,
    pub sender_is_owner: bool,
    pub operator_id: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub ready_limit: Option<usize>,
    pub now_unix_ms: Option<u64>,
    pub reducer_mode: AgentReducerMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextRecallOperatorInvocationCommandRequest {
    pub target: String,
    pub channel_id: String,
    pub sender_id: String,
    pub sender_is_owner: bool,
    pub operator_id: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub ready_limit: Option<usize>,
    pub now_unix_ms: Option<u64>,
    pub reducer_mode: AgentReducerMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContextRecallOperatorInvocationReport {
    pub invocation_surface: &'static str,
    pub source_command: &'static str,
    pub target: ContextRecallOperatorInvocationTarget,
    pub status: &'static str,
    pub operator_identity_redacted: bool,
    pub sender_identity_redacted: bool,
    pub idempotency_key_present: bool,
    pub operator_confirmed: bool,
    pub operator_policy_decision: OperatorPolicyDecision,
    pub operator_policy_decision_label: &'static str,
    pub operator_policy_allowed: bool,
    pub operator_policy_requires_approval: bool,
    pub operator_policy_denied_reason_count: usize,
    pub context_recall_handoff_policy: ContextRecallOperatorInvocationHandoffPolicy,
    pub target_executed: bool,
    pub ready_limit: Option<usize>,
    pub now_unix_ms_present: bool,
    pub reducer_mode: AgentReducerMode,
    pub candidate_count: usize,
    pub ready_count: usize,
    pub due_count: usize,
    pub ran_count: usize,
    pub skipped_count: usize,
    pub blocked_count: usize,
    pub requested_agent_count: usize,
    pub launched_agent_count: usize,
    pub completed_agent_count: usize,
    pub failed_agent_count: usize,
    pub total_messages_processed: usize,
    pub provider_rollup_present_count: usize,
    pub selected_snippets_present_count: usize,
    pub selected_snippet_count: u32,
    pub selected_snippet_text_exposed: bool,
    pub source_ids_exposed: bool,
    pub query_payload_exposed: bool,
    pub prompt_or_final_text_exposed: bool,
    pub stable_schema_promoted: bool,
    pub blockers: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContextRecallOperatorInvocationCommandReport {
    pub command_surface: &'static str,
    pub source_command: &'static str,
    pub status: &'static str,
    pub target_supported: bool,
    pub target: Option<ContextRecallOperatorInvocationTarget>,
    pub dispatcher_ran: bool,
    pub operator_identity_redacted: bool,
    pub sender_identity_redacted: bool,
    pub idempotency_key_present: bool,
    pub operator_confirmed: bool,
    pub operator_policy_decision: Option<OperatorPolicyDecision>,
    pub operator_policy_decision_label: Option<&'static str>,
    pub operator_policy_allowed: bool,
    pub operator_policy_requires_approval: bool,
    pub operator_policy_denied_reason_count: usize,
    pub context_recall_handoff_policy: ContextRecallOperatorInvocationHandoffPolicy,
    pub target_executed: bool,
    pub ready_limit: Option<usize>,
    pub now_unix_ms_present: bool,
    pub reducer_mode: AgentReducerMode,
    pub candidate_count: usize,
    pub ready_count: usize,
    pub due_count: usize,
    pub ran_count: usize,
    pub skipped_count: usize,
    pub blocked_count: usize,
    pub requested_agent_count: usize,
    pub launched_agent_count: usize,
    pub completed_agent_count: usize,
    pub failed_agent_count: usize,
    pub total_messages_processed: usize,
    pub provider_rollup_present_count: usize,
    pub selected_snippets_present_count: usize,
    pub selected_snippet_count: u32,
    pub selected_snippet_text_exposed: bool,
    pub source_ids_exposed: bool,
    pub query_payload_exposed: bool,
    pub prompt_or_final_text_exposed: bool,
    pub stable_schema_promoted: bool,
    pub blockers: Vec<&'static str>,
}

impl RuntimeKernel {
    pub async fn run_context_recall_operator_invocation_command(
        &self,
        request: ContextRecallOperatorInvocationCommandRequest,
    ) -> Result<ContextRecallOperatorInvocationCommandReport, HeptaError> {
        let target = parse_context_recall_operator_invocation_target(&request.target);
        let mut blockers = Vec::new();
        if target.is_none() {
            blockers.push("unsupported_target");
        }
        if request.operator_id.trim().is_empty() {
            blockers.push("missing_operator_id");
        }
        if request.idempotency_key.trim().is_empty() {
            blockers.push("missing_idempotency_key");
        }
        if !blockers.is_empty() {
            return Ok(blocked_command_report(&request, target, blockers));
        }

        let target = target.expect("unsupported target handled above");
        let report = self
            .run_context_recall_operator_invocation(ContextRecallOperatorInvocationRequest {
                target,
                channel_id: request.channel_id.clone(),
                sender_id: request.sender_id.clone(),
                sender_is_owner: request.sender_is_owner,
                operator_id: request.operator_id.clone(),
                operator_confirmed: request.operator_confirmed,
                idempotency_key: request.idempotency_key.clone(),
                ready_limit: request.ready_limit,
                now_unix_ms: request.now_unix_ms,
                reducer_mode: request.reducer_mode,
            })
            .await?;

        Ok(command_report_from_dispatch(&request, report))
    }

    pub async fn run_context_recall_operator_invocation(
        &self,
        request: ContextRecallOperatorInvocationRequest,
    ) -> Result<ContextRecallOperatorInvocationReport, HeptaError> {
        match request.target {
            ContextRecallOperatorInvocationTarget::WorkerReady
            | ContextRecallOperatorInvocationTarget::WorkerDue => {
                let worker = self
                    .run_worker_scheduler_with_context_recall_operator_invocation(
                        WorkerTaskContextRecallOperatorInvocationRequest {
                            scheduler: match request.target {
                                ContextRecallOperatorInvocationTarget::WorkerReady => {
                                    WorkerTaskContextRecallOperatorSchedulerKind::Ready
                                }
                                ContextRecallOperatorInvocationTarget::WorkerDue => {
                                    WorkerTaskContextRecallOperatorSchedulerKind::Due
                                }
                                ContextRecallOperatorInvocationTarget::MultiAgentReady => {
                                    unreachable!("multi-agent target handled by outer match")
                                }
                            },
                            channel_id: request.channel_id.clone(),
                            sender_id: request.sender_id.clone(),
                            sender_is_owner: request.sender_is_owner,
                            operator_id: request.operator_id.clone(),
                            operator_confirmed: request.operator_confirmed,
                            idempotency_key: request.idempotency_key.clone(),
                            ready_limit: request.ready_limit,
                            now_unix_ms: request.now_unix_ms,
                        },
                    )
                    .await?;
                Ok(report_from_worker(&request, worker))
            }
            ContextRecallOperatorInvocationTarget::MultiAgentReady => {
                let multi_agent = self
                    .run_ready_agents_with_context_recall_operator_invocation(
                        MultiAgentContextRecallOperatorInvocationRequest {
                            channel_id: request.channel_id.clone(),
                            sender_id: request.sender_id.clone(),
                            sender_is_owner: request.sender_is_owner,
                            operator_id: request.operator_id.clone(),
                            operator_confirmed: request.operator_confirmed,
                            idempotency_key: request.idempotency_key.clone(),
                            limit: request.ready_limit,
                            reducer_mode: request.reducer_mode,
                        },
                    )
                    .await?;
                Ok(report_from_multi_agent(&request, multi_agent))
            }
        }
    }
}

fn parse_context_recall_operator_invocation_target(
    target: &str,
) -> Option<ContextRecallOperatorInvocationTarget> {
    match target
        .trim()
        .to_ascii_lowercase()
        .replace('_', "-")
        .as_str()
    {
        "worker-ready" | "worker-ready-scheduler" => {
            Some(ContextRecallOperatorInvocationTarget::WorkerReady)
        }
        "worker-due" | "worker-due-scheduler" => {
            Some(ContextRecallOperatorInvocationTarget::WorkerDue)
        }
        "multi-agent-ready" | "multiagent-ready" => {
            Some(ContextRecallOperatorInvocationTarget::MultiAgentReady)
        }
        _ => None,
    }
}

fn blocked_command_report(
    request: &ContextRecallOperatorInvocationCommandRequest,
    target: Option<ContextRecallOperatorInvocationTarget>,
    blockers: Vec<&'static str>,
) -> ContextRecallOperatorInvocationCommandReport {
    ContextRecallOperatorInvocationCommandReport {
        command_surface: CONTEXT_RECALL_OPERATOR_COMMAND_SURFACE,
        source_command: CONTEXT_RECALL_OPERATOR_COMMAND_SOURCE,
        status: "blocked",
        target_supported: target.is_some(),
        target,
        dispatcher_ran: false,
        operator_identity_redacted: true,
        sender_identity_redacted: true,
        idempotency_key_present: !request.idempotency_key.trim().is_empty(),
        operator_confirmed: request.operator_confirmed,
        operator_policy_decision: None,
        operator_policy_decision_label: None,
        operator_policy_allowed: false,
        operator_policy_requires_approval: false,
        operator_policy_denied_reason_count: 0,
        context_recall_handoff_policy: ContextRecallOperatorInvocationHandoffPolicy::Disabled,
        target_executed: false,
        ready_limit: request.ready_limit,
        now_unix_ms_present: request.now_unix_ms.is_some(),
        reducer_mode: request.reducer_mode,
        candidate_count: 0,
        ready_count: 0,
        due_count: 0,
        ran_count: 0,
        skipped_count: 0,
        blocked_count: 0,
        requested_agent_count: 0,
        launched_agent_count: 0,
        completed_agent_count: 0,
        failed_agent_count: 0,
        total_messages_processed: 0,
        provider_rollup_present_count: 0,
        selected_snippets_present_count: 0,
        selected_snippet_count: 0,
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        prompt_or_final_text_exposed: false,
        stable_schema_promoted: false,
        blockers,
    }
}

fn command_report_from_dispatch(
    request: &ContextRecallOperatorInvocationCommandRequest,
    report: ContextRecallOperatorInvocationReport,
) -> ContextRecallOperatorInvocationCommandReport {
    ContextRecallOperatorInvocationCommandReport {
        command_surface: CONTEXT_RECALL_OPERATOR_COMMAND_SURFACE,
        source_command: CONTEXT_RECALL_OPERATOR_COMMAND_SOURCE,
        status: report.status,
        target_supported: true,
        target: Some(report.target),
        dispatcher_ran: true,
        operator_identity_redacted: true,
        sender_identity_redacted: true,
        idempotency_key_present: report.idempotency_key_present,
        operator_confirmed: report.operator_confirmed,
        operator_policy_decision: Some(report.operator_policy_decision),
        operator_policy_decision_label: Some(report.operator_policy_decision_label),
        operator_policy_allowed: report.operator_policy_allowed,
        operator_policy_requires_approval: report.operator_policy_requires_approval,
        operator_policy_denied_reason_count: report.operator_policy_denied_reason_count,
        context_recall_handoff_policy: report.context_recall_handoff_policy,
        target_executed: report.target_executed,
        ready_limit: report.ready_limit,
        now_unix_ms_present: request.now_unix_ms.is_some(),
        reducer_mode: report.reducer_mode,
        candidate_count: report.candidate_count,
        ready_count: report.ready_count,
        due_count: report.due_count,
        ran_count: report.ran_count,
        skipped_count: report.skipped_count,
        blocked_count: report.blocked_count,
        requested_agent_count: report.requested_agent_count,
        launched_agent_count: report.launched_agent_count,
        completed_agent_count: report.completed_agent_count,
        failed_agent_count: report.failed_agent_count,
        total_messages_processed: report.total_messages_processed,
        provider_rollup_present_count: report.provider_rollup_present_count,
        selected_snippets_present_count: report.selected_snippets_present_count,
        selected_snippet_count: report.selected_snippet_count,
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        prompt_or_final_text_exposed: false,
        stable_schema_promoted: false,
        blockers: report.blockers,
    }
}

fn report_from_worker(
    request: &ContextRecallOperatorInvocationRequest,
    report: WorkerTaskContextRecallOperatorInvocationReport,
) -> ContextRecallOperatorInvocationReport {
    ContextRecallOperatorInvocationReport {
        invocation_surface: CONTEXT_RECALL_OPERATOR_INVOCATION_SURFACE,
        source_command: CONTEXT_RECALL_OPERATOR_INVOCATION_COMMAND,
        target: request.target,
        status: report.scheduler_report.status,
        operator_identity_redacted: true,
        sender_identity_redacted: true,
        idempotency_key_present: report.idempotency_key_present,
        operator_confirmed: report.operator_confirmed,
        operator_policy_decision: report.operator_policy_decision,
        operator_policy_decision_label: report.operator_policy_decision_label,
        operator_policy_allowed: report.operator_policy_allowed,
        operator_policy_requires_approval: report.operator_policy_requires_approval,
        operator_policy_denied_reason_count: report.operator_policy_denied_reason_count,
        context_recall_handoff_policy: worker_policy(
            report.scheduler_report.context_recall_handoff_policy,
        ),
        target_executed: report.scheduler_report.scheduler_executed,
        ready_limit: report.scheduler_report.ready_limit,
        now_unix_ms_present: request.now_unix_ms.is_some(),
        reducer_mode: request.reducer_mode,
        candidate_count: report.scheduler_report.candidate_count,
        ready_count: report.scheduler_report.ready_count,
        due_count: report.scheduler_report.due_count,
        ran_count: report.scheduler_report.ran_count,
        skipped_count: report.scheduler_report.skipped_count,
        blocked_count: report.scheduler_report.blocked_count,
        requested_agent_count: 0,
        launched_agent_count: 0,
        completed_agent_count: 0,
        failed_agent_count: 0,
        total_messages_processed: 0,
        provider_rollup_present_count: report.scheduler_report.provider_rollup_present_count,
        selected_snippets_present_count: report.scheduler_report.selected_snippets_present_count,
        selected_snippet_count: report.scheduler_report.selected_snippet_count,
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        prompt_or_final_text_exposed: false,
        stable_schema_promoted: false,
        blockers: report.scheduler_report.blockers,
    }
}

fn report_from_multi_agent(
    request: &ContextRecallOperatorInvocationRequest,
    report: MultiAgentContextRecallOperatorInvocationReport,
) -> ContextRecallOperatorInvocationReport {
    ContextRecallOperatorInvocationReport {
        invocation_surface: CONTEXT_RECALL_OPERATOR_INVOCATION_SURFACE,
        source_command: CONTEXT_RECALL_OPERATOR_INVOCATION_COMMAND,
        target: request.target,
        status: report.status,
        operator_identity_redacted: true,
        sender_identity_redacted: true,
        idempotency_key_present: report.idempotency_key_present,
        operator_confirmed: report.operator_confirmed,
        operator_policy_decision: report.operator_policy_decision,
        operator_policy_decision_label: report.operator_policy_decision_label,
        operator_policy_allowed: report.operator_policy_allowed,
        operator_policy_requires_approval: report.operator_policy_requires_approval,
        operator_policy_denied_reason_count: report.operator_policy_denied_reason_count,
        context_recall_handoff_policy: multi_agent_policy(report.context_recall_handoff_policy),
        target_executed: report.agent_runtime_executed,
        ready_limit: report.limit,
        now_unix_ms_present: request.now_unix_ms.is_some(),
        reducer_mode: report.reducer_mode,
        candidate_count: 0,
        ready_count: 0,
        due_count: 0,
        ran_count: 0,
        skipped_count: 0,
        blocked_count: 0,
        requested_agent_count: report.requested_agent_count,
        launched_agent_count: report.launched_agent_count,
        completed_agent_count: report.completed_agent_count,
        failed_agent_count: report.failed_agent_count,
        total_messages_processed: report.total_messages_processed,
        provider_rollup_present_count: report.provider_rollup_present_count,
        selected_snippets_present_count: report.selected_snippets_present_count,
        selected_snippet_count: report.selected_snippet_count,
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        prompt_or_final_text_exposed: false,
        stable_schema_promoted: false,
        blockers: report.blockers,
    }
}

fn worker_policy(
    policy: WorkerTaskContextRecallHandoffPolicy,
) -> ContextRecallOperatorInvocationHandoffPolicy {
    match policy {
        WorkerTaskContextRecallHandoffPolicy::Disabled => {
            ContextRecallOperatorInvocationHandoffPolicy::Disabled
        }
        WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved => {
            ContextRecallOperatorInvocationHandoffPolicy::ExperimentalOperatorApproved
        }
    }
}

fn multi_agent_policy(
    policy: AgentRuntimeContextRecallHandoffPolicy,
) -> ContextRecallOperatorInvocationHandoffPolicy {
    match policy {
        AgentRuntimeContextRecallHandoffPolicy::Disabled => {
            ContextRecallOperatorInvocationHandoffPolicy::Disabled
        }
        AgentRuntimeContextRecallHandoffPolicy::ExperimentalOperatorApproved => {
            ContextRecallOperatorInvocationHandoffPolicy::ExperimentalOperatorApproved
        }
    }
}

#[cfg(test)]
mod tests {
    use hepta_core::MemoryRecord;
    use hepta_core::MemoryScope;
    use hepta_core::MemoryStore;

    use super::*;

    fn assert_runtime_operator_invocation_report_does_not_leak(rendered: &str) {
        for forbidden in [
            "runtime-operator-worker-safe-context",
            "runtime-operator-worker-source-id",
            "runtime-operator-worker-needle",
            "runtime-operator-multi-agent-safe-context",
            "runtime-operator-multi-agent-source-id",
            "runtime-operator-multi-agent-needle",
            "runtime-command-worker-safe-context",
            "runtime-command-worker-source-id",
            "runtime-command-worker-needle",
            "runtime-command-multi-agent-safe-context",
            "runtime-command-multi-agent-source-id",
            "runtime-command-multi-agent-needle",
            "runtime-command-unsupported-target",
            "operator-a",
            "command-operator-a",
            "telegram:6476198178",
            "6476198178",
            "runtime-operator-worker-denied-1",
            "runtime-operator-worker-approved-1",
            "runtime-operator-multi-agent-approved-1",
            "runtime-command-worker-approved-1",
            "runtime-command-multi-agent-approved-1",
            "[redacted-query]",
            "<selected_context_recall>",
        ] {
            assert!(
                !rendered.contains(forbidden),
                "runtime operator invocation report leaked {forbidden}"
            );
        }
    }

    #[tokio::test]
    async fn context_recall_operator_invocation_dispatches_worker_and_multi_agent_without_leak() {
        let worker_runtime = RuntimeKernel::new();
        worker_runtime
            .memory
            .put(MemoryRecord {
                id: "runtime-operator-worker-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "runtime-operator-worker-needle {}",
                    "runtime-operator-worker-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        let task = worker_runtime
            .spawn_worker_task("runtime worker", "runtime-operator-worker-needle", None)
            .expect("worker task should spawn");

        let denied = worker_runtime
            .run_context_recall_operator_invocation(ContextRecallOperatorInvocationRequest {
                target: ContextRecallOperatorInvocationTarget::WorkerReady,
                channel_id: "telegram:6476198178".into(),
                sender_id: "6476198178".into(),
                sender_is_owner: false,
                operator_id: "operator-a".into(),
                operator_confirmed: true,
                idempotency_key: "runtime-operator-worker-denied-1".into(),
                ready_limit: Some(10),
                now_unix_ms: None,
                reducer_mode: AgentReducerMode::Any,
            })
            .await
            .expect("denied worker route should return a report");

        assert_eq!(
            denied.target,
            ContextRecallOperatorInvocationTarget::WorkerReady
        );
        assert_eq!(denied.status, "blocked");
        assert!(!denied.target_executed);
        assert_eq!(
            denied.operator_policy_decision,
            OperatorPolicyDecision::RequireApproval
        );
        assert_eq!(
            denied.context_recall_handoff_policy,
            ContextRecallOperatorInvocationHandoffPolicy::Disabled
        );
        assert_eq!(denied.blockers, vec!["policy_not_allowed"]);
        assert_eq!(
            worker_runtime
                .worker_task_status(&task.task.task_id)
                .expect("task should still exist")
                .task
                .status,
            crate::WorkerTaskStatus::Queued
        );

        let approved_worker = worker_runtime
            .run_context_recall_operator_invocation(ContextRecallOperatorInvocationRequest {
                target: ContextRecallOperatorInvocationTarget::WorkerReady,
                channel_id: "telegram:6476198178".into(),
                sender_id: "6476198178".into(),
                sender_is_owner: true,
                operator_id: "operator-a".into(),
                operator_confirmed: true,
                idempotency_key: "runtime-operator-worker-approved-1".into(),
                ready_limit: Some(10),
                now_unix_ms: None,
                reducer_mode: AgentReducerMode::Any,
            })
            .await
            .expect("approved worker route should execute");

        assert_eq!(approved_worker.status, "executed");
        assert!(approved_worker.target_executed);
        assert_eq!(
            approved_worker.context_recall_handoff_policy,
            ContextRecallOperatorInvocationHandoffPolicy::ExperimentalOperatorApproved
        );
        assert_eq!(approved_worker.ran_count, 1);
        assert_eq!(approved_worker.provider_rollup_present_count, 1);
        assert_eq!(approved_worker.selected_snippets_present_count, 1);
        assert!(approved_worker.selected_snippet_count > 0);

        let multi_runtime = RuntimeKernel::new();
        multi_runtime
            .memory
            .put(MemoryRecord {
                id: "runtime-operator-multi-agent-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "runtime-operator-multi-agent-needle {}",
                    "runtime-operator-multi-agent-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        multi_runtime
            .register_agent_runtime("runtime-operator-agent")
            .expect("agent should register");
        multi_runtime
            .enqueue_agent_message(
                "runtime-operator-agent",
                "runtime-operator-multi-agent-needle",
                Some("operator"),
            )
            .expect("message should enqueue");

        let approved_multi = multi_runtime
            .run_context_recall_operator_invocation(ContextRecallOperatorInvocationRequest {
                target: ContextRecallOperatorInvocationTarget::MultiAgentReady,
                channel_id: "telegram:6476198178".into(),
                sender_id: "6476198178".into(),
                sender_is_owner: true,
                operator_id: "operator-a".into(),
                operator_confirmed: true,
                idempotency_key: "runtime-operator-multi-agent-approved-1".into(),
                ready_limit: Some(1),
                now_unix_ms: None,
                reducer_mode: AgentReducerMode::Any,
            })
            .await
            .expect("approved multi-agent route should execute");

        assert_eq!(
            approved_multi.target,
            ContextRecallOperatorInvocationTarget::MultiAgentReady
        );
        assert_eq!(approved_multi.status, "executed");
        assert!(approved_multi.target_executed);
        assert_eq!(
            approved_multi.context_recall_handoff_policy,
            ContextRecallOperatorInvocationHandoffPolicy::ExperimentalOperatorApproved
        );
        assert_eq!(approved_multi.completed_agent_count, 1);
        assert_eq!(approved_multi.total_messages_processed, 1);
        assert_eq!(approved_multi.provider_rollup_present_count, 1);
        assert_eq!(approved_multi.selected_snippets_present_count, 1);
        assert!(approved_multi.selected_snippet_count > 0);

        for rendered in [
            serde_json::to_string(&denied).expect("denied report should serialize"),
            format!("{denied:?}"),
            serde_json::to_string(&approved_worker).expect("worker report should serialize"),
            format!("{approved_worker:?}"),
            serde_json::to_string(&approved_multi).expect("multi-agent report should serialize"),
            format!("{approved_multi:?}"),
        ] {
            assert_runtime_operator_invocation_report_does_not_leak(&rendered);
        }
    }

    #[tokio::test]
    async fn context_recall_operator_invocation_command_blocks_bad_envelope_and_dispatches_without_leak()
     {
        let worker_runtime = RuntimeKernel::new();
        worker_runtime
            .memory
            .put(MemoryRecord {
                id: "runtime-command-worker-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "runtime-command-worker-needle {}",
                    "runtime-command-worker-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        let task = worker_runtime
            .spawn_worker_task(
                "runtime command worker",
                "runtime-command-worker-needle",
                None,
            )
            .expect("worker task should spawn");

        let blocked = worker_runtime
            .run_context_recall_operator_invocation_command(
                ContextRecallOperatorInvocationCommandRequest {
                    target: "runtime-command-unsupported-target".into(),
                    channel_id: "telegram:6476198178".into(),
                    sender_id: "6476198178".into(),
                    sender_is_owner: true,
                    operator_id: "".into(),
                    operator_confirmed: true,
                    idempotency_key: "".into(),
                    ready_limit: Some(10),
                    now_unix_ms: None,
                    reducer_mode: AgentReducerMode::Any,
                },
            )
            .await
            .expect("bad command envelope should return blocked report");

        assert_eq!(blocked.status, "blocked");
        assert!(!blocked.target_supported);
        assert!(!blocked.dispatcher_ran);
        assert!(!blocked.target_executed);
        assert_eq!(
            blocked.context_recall_handoff_policy,
            ContextRecallOperatorInvocationHandoffPolicy::Disabled
        );
        assert_eq!(
            blocked.blockers,
            vec![
                "unsupported_target",
                "missing_operator_id",
                "missing_idempotency_key"
            ]
        );
        assert_eq!(
            worker_runtime
                .worker_task_status(&task.task.task_id)
                .expect("task should still exist")
                .task
                .status,
            crate::WorkerTaskStatus::Queued
        );

        let approved_worker = worker_runtime
            .run_context_recall_operator_invocation_command(
                ContextRecallOperatorInvocationCommandRequest {
                    target: "worker-ready".into(),
                    channel_id: "telegram:6476198178".into(),
                    sender_id: "6476198178".into(),
                    sender_is_owner: true,
                    operator_id: "command-operator-a".into(),
                    operator_confirmed: true,
                    idempotency_key: "runtime-command-worker-approved-1".into(),
                    ready_limit: Some(10),
                    now_unix_ms: None,
                    reducer_mode: AgentReducerMode::Any,
                },
            )
            .await
            .expect("approved worker command should dispatch");

        assert_eq!(approved_worker.status, "executed");
        assert!(approved_worker.target_supported);
        assert!(approved_worker.dispatcher_ran);
        assert!(approved_worker.target_executed);
        assert_eq!(
            approved_worker.target,
            Some(ContextRecallOperatorInvocationTarget::WorkerReady)
        );
        assert_eq!(approved_worker.ran_count, 1);
        assert_eq!(approved_worker.provider_rollup_present_count, 1);
        assert_eq!(approved_worker.selected_snippets_present_count, 1);
        assert!(approved_worker.selected_snippet_count > 0);

        let multi_runtime = RuntimeKernel::new();
        multi_runtime
            .memory
            .put(MemoryRecord {
                id: "runtime-command-multi-agent-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "runtime-command-multi-agent-needle {}",
                    "runtime-command-multi-agent-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        multi_runtime
            .register_agent_runtime("runtime-command-agent")
            .expect("agent should register");
        multi_runtime
            .enqueue_agent_message(
                "runtime-command-agent",
                "runtime-command-multi-agent-needle",
                Some("operator"),
            )
            .expect("message should enqueue");

        let approved_multi = multi_runtime
            .run_context_recall_operator_invocation_command(
                ContextRecallOperatorInvocationCommandRequest {
                    target: "multi-agent-ready".into(),
                    channel_id: "telegram:6476198178".into(),
                    sender_id: "6476198178".into(),
                    sender_is_owner: true,
                    operator_id: "command-operator-a".into(),
                    operator_confirmed: true,
                    idempotency_key: "runtime-command-multi-agent-approved-1".into(),
                    ready_limit: Some(1),
                    now_unix_ms: None,
                    reducer_mode: AgentReducerMode::Any,
                },
            )
            .await
            .expect("approved multi-agent command should dispatch");

        assert_eq!(approved_multi.status, "executed");
        assert!(approved_multi.target_supported);
        assert!(approved_multi.dispatcher_ran);
        assert!(approved_multi.target_executed);
        assert_eq!(
            approved_multi.target,
            Some(ContextRecallOperatorInvocationTarget::MultiAgentReady)
        );
        assert_eq!(approved_multi.completed_agent_count, 1);
        assert_eq!(approved_multi.total_messages_processed, 1);
        assert_eq!(approved_multi.provider_rollup_present_count, 1);
        assert_eq!(approved_multi.selected_snippets_present_count, 1);
        assert!(approved_multi.selected_snippet_count > 0);

        for rendered in [
            serde_json::to_string(&blocked).expect("blocked command report should serialize"),
            format!("{blocked:?}"),
            serde_json::to_string(&approved_worker)
                .expect("worker command report should serialize"),
            format!("{approved_worker:?}"),
            serde_json::to_string(&approved_multi)
                .expect("multi-agent command report should serialize"),
            format!("{approved_multi:?}"),
        ] {
            assert_runtime_operator_invocation_report_does_not_leak(&rendered);
        }
    }
}
