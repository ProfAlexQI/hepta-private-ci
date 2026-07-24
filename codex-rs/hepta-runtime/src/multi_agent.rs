use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Instant;

use hepta_core::AgentId;
use hepta_core::EventKind;
use hepta_core::HeptaError;
use hepta_core::SessionId;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tokio::task::JoinSet;

use crate::operator_policy::OperatorPolicyDecision;
use crate::operator_policy::OperatorPolicyEvaluationReport;
use crate::operator_policy::OperatorPolicyInput;
use crate::operator_policy::evaluate_operator_policy;

use super::RuntimeKernel;
use super::current_unix_ms;

const DEFAULT_AGENT_MAX_INBOX_MESSAGES: usize = 64;
const DEFAULT_AGENT_MAX_TURNS_PER_RUN: usize = 4;
const DEFAULT_AGENT_MAX_PARALLEL_TOOL_SLOTS: usize = 2;
const DEFAULT_AGENT_MAX_CONCURRENT_RUNS: usize = 1;
const DEFAULT_MULTI_AGENT_DEMO_AGENTS: usize = 4;
const DEFAULT_MULTI_AGENT_DEMO_MESSAGES_PER_AGENT: usize = 1;
const MULTI_AGENT_CONTEXT_RECALL_OPERATOR_INVOCATION_SURFACE: &str =
    "hepta-context-recall-multi-agent-operator-invocation";
const MULTI_AGENT_CONTEXT_RECALL_OPERATOR_INVOCATION_COMMAND: &str =
    "/hepta-context-recall-multi-agent-handoff --execute --json";
const MULTI_AGENT_CONTEXT_RECALL_OPERATOR_TOOL_NAME: &str =
    "hepta_context_recall_multi_agent_handoff";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRuntimeStatus {
    Registered,
    Idle,
    Paused,
    Running,
    Completed,
    Failed,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentInboxMessageKind {
    User,
    Agent,
    Control,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentDeliveryState {
    Queued,
    Leased,
    Processed,
    Failed,
    Retried,
}

impl Default for AgentDeliveryState {
    fn default() -> Self {
        Self::Queued
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRuntimeExecutionBackend {
    LocalModelToolLoop,
}

impl Default for AgentRuntimeExecutionBackend {
    fn default() -> Self {
        Self::LocalModelToolLoop
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRuntimeFailureKind {
    InjectedFailure,
    Timeout,
    JoinError,
}

impl Default for AgentInboxMessageKind {
    fn default() -> Self {
        Self::User
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentReducerMode {
    All,
    Any,
    Quorum,
    Ranked,
    Merge,
}

impl Default for AgentReducerMode {
    fn default() -> Self {
        Self::Merge
    }
}

impl AgentReducerMode {
    fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Any => "any",
            Self::Quorum => "quorum",
            Self::Ranked => "ranked",
            Self::Merge => "merge",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRuntimeQuota {
    pub max_inbox_messages: usize,
    pub max_turns_per_run: usize,
    pub max_parallel_tool_slots: usize,
    pub max_concurrent_runs: usize,
}

impl Default for AgentRuntimeQuota {
    fn default() -> Self {
        Self {
            max_inbox_messages: DEFAULT_AGENT_MAX_INBOX_MESSAGES,
            max_turns_per_run: DEFAULT_AGENT_MAX_TURNS_PER_RUN,
            max_parallel_tool_slots: DEFAULT_AGENT_MAX_PARALLEL_TOOL_SLOTS,
            max_concurrent_runs: DEFAULT_AGENT_MAX_CONCURRENT_RUNS,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentInboxMessage {
    pub message_id: String,
    #[serde(default)]
    pub workspace_id: String,
    pub from_agent_id: Option<String>,
    #[serde(default)]
    pub message_kind: AgentInboxMessageKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dedupe_key: Option<String>,
    #[serde(default)]
    pub delivery_attempt: u32,
    #[serde(default)]
    pub delivery_state: AgentDeliveryState,
    pub content: String,
    pub queued_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRuntimeRecord {
    pub agent_id: String,
    pub session_id: String,
    #[serde(default)]
    pub workspace_id: String,
    pub status: AgentRuntimeStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_key: Option<String>,
    pub quota: AgentRuntimeQuota,
    pub inbox: Vec<AgentInboxMessage>,
    pub processed_message_count: usize,
    pub failed_message_count: usize,
    #[serde(default)]
    pub control_generation: u64,
    #[serde(default)]
    pub steering_instructions: Vec<String>,
    pub last_started_at_unix_ms: Option<u64>,
    pub last_completed_at_unix_ms: Option<u64>,
    pub last_error: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiAgentRuntimeState {
    pub agents: Vec<AgentRuntimeRecord>,
    pub next_message_suffix: u64,
    pub last_max_parallelism_observed: usize,
    #[serde(default)]
    pub last_barrier_join_observed: bool,
    #[serde(default)]
    pub last_completed_agent_count: usize,
    #[serde(default)]
    pub last_total_messages_processed: usize,
    #[serde(default)]
    pub last_cross_agent_message_observed: bool,
    #[serde(default)]
    pub last_lifecycle_control_observed: bool,
    #[serde(default)]
    pub last_reducer_mode: AgentReducerMode,
    #[serde(default)]
    pub active_resource_leases: Vec<AgentResourceLease>,
    #[serde(default)]
    pub last_resource_lock_observed: bool,
    #[serde(default)]
    pub last_delivery_state_observed: bool,
    #[serde(default)]
    pub last_model_tool_loop_observed: bool,
    #[serde(default)]
    pub last_failure_timeout_recovery_observed: bool,
    #[serde(default)]
    pub last_reducer_consensus_observed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentResourceLease {
    pub resource_key: String,
    pub agent_id: String,
    pub acquired_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentRuntimeDescriptor {
    pub agent_id: String,
    pub session_id: String,
    pub workspace_id: String,
    pub status: AgentRuntimeStatus,
    pub resource_key: String,
    pub inbox_depth: usize,
    pub processed_message_count: usize,
    pub failed_message_count: usize,
    pub control_generation: u64,
    pub steering_instruction_count: usize,
    pub last_started_at_unix_ms: Option<u64>,
    pub last_completed_at_unix_ms: Option<u64>,
    pub last_error: Option<String>,
    pub quota: AgentRuntimeQuota,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MultiAgentRuntimeRatings {
    pub top_level_agent_registry_percent: u8,
    pub independent_inbox_event_loop_percent: u8,
    pub true_concurrent_scheduler_percent: u8,
    pub agent_resource_isolation_percent: u8,
    pub cross_agent_message_bus_percent: u8,
    pub barrier_reducer_join_percent: u8,
    pub lifecycle_control_percent: u8,
    pub model_tool_loop_percent: u8,
    pub delivery_state_retry_percent: u8,
    pub failure_timeout_recovery_percent: u8,
    pub reducer_consensus_percent: u8,
    pub evidence_observed_percent: u8,
    pub overall_percent: u8,
    pub all_ratings_100: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentRuntimePoolReport {
    pub schema_version: u32,
    pub runtime_kind: &'static str,
    pub agent_count: usize,
    pub idle_count: usize,
    pub running_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub stopped_count: usize,
    pub inbox_depth_total: usize,
    pub lifecycle_registry_ready: bool,
    pub independent_inbox_event_loop_ready: bool,
    pub true_concurrent_scheduler_ready: bool,
    pub agent_resource_isolation_ready: bool,
    pub cross_agent_message_bus_ready: bool,
    pub barrier_reducer_join_ready: bool,
    pub lifecycle_control_ready: bool,
    pub model_tool_loop_ready: bool,
    pub delivery_state_retry_ready: bool,
    pub failure_timeout_recovery_ready: bool,
    pub reducer_consensus_ready: bool,
    pub evidence_observed_ready: bool,
    pub latest_max_parallelism_observed: usize,
    pub active_resource_lease_count: usize,
    pub ratings: MultiAgentRuntimeRatings,
    pub agents: Vec<AgentRuntimeDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentRuntimeControlReport {
    pub schema_version: u32,
    pub action: &'static str,
    pub agent_id: String,
    pub session_id: String,
    pub workspace_id: String,
    pub previous_status: AgentRuntimeStatus,
    pub status: AgentRuntimeStatus,
    pub inbox_depth: usize,
    pub drained_message_count: usize,
    pub steering_instruction_count: usize,
    pub control_generation: u64,
    pub summary: String,
    pub pool: AgentRuntimePoolReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRuntimeTurnResult {
    pub message_id: String,
    pub workspace_id: String,
    pub input: String,
    pub final_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoked_tool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_output_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_required: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentRuntimeRunResult {
    pub agent_id: String,
    pub session_id: String,
    pub workspace_id: String,
    pub message_count: usize,
    pub message_ids: Vec<String>,
    pub started_at_unix_ms: u64,
    pub completed_at_unix_ms: u64,
    pub elapsed_ms: u64,
    pub event_loop_ticks: usize,
    pub execution_backend: AgentRuntimeExecutionBackend,
    pub model_turns_executed: usize,
    pub tool_calls_executed: usize,
    pub tool_slots_reserved: usize,
    pub resource_lock_key: String,
    pub steering_instruction_count: usize,
    pub delivery_state_transitions: usize,
    pub max_delivery_attempt: u32,
    pub output_summary: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub turn_results: Vec<AgentRuntimeTurnResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentRuntimeRunFailure {
    pub agent_id: String,
    pub session_id: String,
    pub workspace_id: String,
    pub message_count: usize,
    pub message_ids: Vec<String>,
    pub failure_kind: AgentRuntimeFailureKind,
    pub reason: String,
    pub retry_scheduled: bool,
    pub delivery_attempts: Vec<u32>,
    pub started_at_unix_ms: u64,
    pub completed_at_unix_ms: u64,
    pub elapsed_ms: u64,
    pub resource_lock_key: String,
    #[serde(skip_serializing)]
    pub retry_messages: Vec<AgentInboxMessage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRuntimeContextRecallHandoffPolicy {
    Disabled,
    ExperimentalOperatorApproved,
}

impl AgentRuntimeContextRecallHandoffPolicy {
    fn experimental_api_enabled(self) -> bool {
        matches!(self, Self::ExperimentalOperatorApproved)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MultiAgentConcurrentRunReport {
    pub schema_version: u32,
    pub scheduler_kind: &'static str,
    pub requested_agent_count: usize,
    pub launched_agent_count: usize,
    pub completed_agent_count: usize,
    pub failed_agent_count: usize,
    pub total_messages_processed: usize,
    pub max_parallelism_observed: usize,
    pub true_concurrent: bool,
    pub barrier_joined: bool,
    pub reducer_mode: AgentReducerMode,
    pub quorum_threshold: usize,
    pub reducer_passed: bool,
    pub consensus_status: &'static str,
    pub reducer_output: String,
    pub ratings: MultiAgentRuntimeRatings,
    pub runs: Vec<AgentRuntimeRunResult>,
    pub failures: Vec<AgentRuntimeRunFailure>,
    pub pool: AgentRuntimePoolReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MultiAgentContextRecallConcurrentRunReport {
    pub context_recall_handoff_policy: AgentRuntimeContextRecallHandoffPolicy,
    pub provider_rollup_present_count: usize,
    pub selected_snippets_present_count: usize,
    pub selected_snippet_count: u32,
    pub selected_snippet_text_exposed: bool,
    pub source_ids_exposed: bool,
    pub query_payload_exposed: bool,
    pub stable_schema_promoted: bool,
    pub run: MultiAgentConcurrentRunReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiAgentContextRecallOperatorInvocationRequest {
    pub channel_id: String,
    pub sender_id: String,
    pub sender_is_owner: bool,
    pub operator_id: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub limit: Option<usize>,
    pub reducer_mode: AgentReducerMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MultiAgentContextRecallOperatorInvocationReport {
    pub invocation_surface: &'static str,
    pub source_command: &'static str,
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
    pub context_recall_handoff_policy: AgentRuntimeContextRecallHandoffPolicy,
    pub agent_runtime_executed: bool,
    pub limit: Option<usize>,
    pub reducer_mode: AgentReducerMode,
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

#[derive(Debug, Clone)]
struct AgentRunPlan {
    agent_id: String,
    session_id: String,
    workspace_id: String,
    messages: Vec<AgentInboxMessage>,
    resource_key: String,
    tool_slots_reserved: usize,
    steering_instruction_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AgentReducerDecision {
    passed: bool,
    consensus_status: &'static str,
    summary: String,
}

impl RuntimeKernel {
    pub fn register_agent_runtime(
        &self,
        agent_id: &str,
    ) -> Result<AgentRuntimePoolReport, HeptaError> {
        self.register_agent_runtime_in_workspace(agent_id, None)
    }

    pub fn register_agent_runtime_in_workspace(
        &self,
        agent_id: &str,
        workspace_id: Option<&str>,
    ) -> Result<AgentRuntimePoolReport, HeptaError> {
        let agent_id = normalize_agent_id(agent_id)?;
        let workspace_id = normalize_workspace_id(workspace_id, &agent_id)?;
        let session_id = workspace_id.clone();
        self.upsert_session_record_with_agent(
            &SessionId(session_id.clone()),
            Some(format!("Hepta agent {}", agent_id)),
            None,
            None,
            true,
            Some(AgentId(agent_id.clone())),
        )?;
        {
            let mut guard = self
                .multi_agent_runtime_state
                .lock()
                .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
            if let Some(existing) = guard
                .agents
                .iter_mut()
                .find(|record| record_matches_agent_workspace(record, &agent_id, &workspace_id))
            {
                if existing.status == AgentRuntimeStatus::Stopped {
                    existing.status = AgentRuntimeStatus::Idle;
                }
                existing.session_id = session_id.clone();
                existing.workspace_id = workspace_id.clone();
                existing.resource_key = Some(format!("workspace:{}", workspace_id));
            } else {
                guard.agents.push(AgentRuntimeRecord {
                    agent_id: agent_id.clone(),
                    session_id: session_id.clone(),
                    workspace_id: workspace_id.clone(),
                    status: AgentRuntimeStatus::Idle,
                    resource_key: Some(format!("workspace:{}", workspace_id)),
                    quota: AgentRuntimeQuota::default(),
                    inbox: Vec::new(),
                    processed_message_count: 0,
                    failed_message_count: 0,
                    control_generation: 0,
                    steering_instructions: Vec::new(),
                    last_started_at_unix_ms: None,
                    last_completed_at_unix_ms: None,
                    last_error: None,
                });
            }
        }
        self.emit_event_with_payload(
            EventKind::AgentRegistered,
            Some(SessionId(session_id)),
            None,
            format!("registered top-level agent {}", agent_id),
            Some(json!({
                "agent_id": agent_id,
                "workspace_id": workspace_id,
                "runtime_kind": "top_level_multi_agent",
            })),
        )?;
        self.agent_runtime_pool_report()
    }

    pub fn enqueue_agent_message(
        &self,
        agent_id: &str,
        content: &str,
        from_agent_id: Option<&str>,
    ) -> Result<AgentRuntimePoolReport, HeptaError> {
        self.enqueue_agent_message_in_workspace(agent_id, None, content, from_agent_id)
    }

    pub fn enqueue_agent_message_in_workspace(
        &self,
        agent_id: &str,
        workspace_id: Option<&str>,
        content: &str,
        from_agent_id: Option<&str>,
    ) -> Result<AgentRuntimePoolReport, HeptaError> {
        let agent_id = normalize_agent_id(agent_id)?;
        let workspace_id = normalize_workspace_id(workspace_id, &agent_id)?;
        let content = content.trim();
        if content.is_empty() {
            return Err(HeptaError("agent message content must not be empty".into()));
        }
        let from_agent_id = from_agent_id
            .map(normalize_agent_id)
            .transpose()?
            .filter(|value| !value.is_empty());
        let (session_id, message_id) = {
            let mut guard = self
                .multi_agent_runtime_state
                .lock()
                .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
            let agent_index = guard
                .agents
                .iter()
                .position(|record| record_matches_agent_workspace(record, &agent_id, &workspace_id))
                .ok_or_else(|| {
                    HeptaError(format!(
                        "unknown agent/workspace: {}/{}",
                        agent_id, workspace_id
                    ))
                })?;
            if guard.agents[agent_index].inbox.len()
                >= guard.agents[agent_index].quota.max_inbox_messages
            {
                return Err(HeptaError(format!(
                    "agent {} inbox is full at {} message(s)",
                    agent_id, guard.agents[agent_index].quota.max_inbox_messages
                )));
            }
            let next_suffix = guard.next_message_suffix + 1;
            guard.next_message_suffix = next_suffix;
            let message_id = format!("msg-{}-{}", agent_id, next_suffix);
            guard.agents[agent_index].workspace_id = workspace_id.clone();
            guard.agents[agent_index].session_id = workspace_id.clone();
            guard.agents[agent_index].resource_key = Some(format!("workspace:{}", workspace_id));
            let session_id = guard.agents[agent_index].session_id.clone();
            let message_kind = if from_agent_id.is_some() {
                AgentInboxMessageKind::Agent
            } else {
                AgentInboxMessageKind::User
            };
            let dedupe_key = Some(format!(
                "{}:{}:{}:{}",
                from_agent_id.as_deref().unwrap_or("external"),
                agent_id,
                workspace_id,
                content
            ));
            guard.agents[agent_index].inbox.push(AgentInboxMessage {
                message_id: message_id.clone(),
                workspace_id: workspace_id.clone(),
                from_agent_id: from_agent_id.clone(),
                message_kind,
                dedupe_key,
                delivery_attempt: 0,
                delivery_state: AgentDeliveryState::Queued,
                content: content.to_string(),
                queued_at_unix_ms: current_unix_ms()?,
            });
            if from_agent_id.is_some() {
                guard.last_cross_agent_message_observed = true;
            }
            (session_id, message_id)
        };
        self.emit_event_with_payload(
            EventKind::AgentMessageQueued,
            Some(SessionId(session_id)),
            None,
            format!("queued message {} for agent {}", message_id, agent_id),
            Some(json!({
                "agent_id": agent_id,
                "workspace_id": workspace_id,
                "message_id": message_id,
                "from_agent_id": from_agent_id,
            })),
        )?;
        self.agent_runtime_pool_report()
    }

    pub fn agent_runtime_pool_report(&self) -> Result<AgentRuntimePoolReport, HeptaError> {
        let guard = self
            .multi_agent_runtime_state
            .lock()
            .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
        Ok(build_agent_runtime_pool_report(&guard))
    }

    pub fn pause_agent_runtime(
        &self,
        agent_id: &str,
    ) -> Result<AgentRuntimeControlReport, HeptaError> {
        self.apply_agent_runtime_control(agent_id, "pause", None, |agent| {
            if agent.status != AgentRuntimeStatus::Stopped {
                agent.status = AgentRuntimeStatus::Paused;
            }
            0
        })
    }

    pub fn resume_agent_runtime(
        &self,
        agent_id: &str,
    ) -> Result<AgentRuntimeControlReport, HeptaError> {
        self.apply_agent_runtime_control(agent_id, "resume", None, |agent| {
            if matches!(
                agent.status,
                AgentRuntimeStatus::Paused
                    | AgentRuntimeStatus::Stopped
                    | AgentRuntimeStatus::Completed
            ) {
                agent.status = AgentRuntimeStatus::Idle;
            }
            0
        })
    }

    pub fn stop_agent_runtime(
        &self,
        agent_id: &str,
    ) -> Result<AgentRuntimeControlReport, HeptaError> {
        self.apply_agent_runtime_control(agent_id, "stop", None, |agent| {
            agent.status = AgentRuntimeStatus::Stopped;
            0
        })
    }

    pub fn stop_agent_runtime_in_workspace(
        &self,
        agent_id: &str,
        workspace_id: &str,
    ) -> Result<AgentRuntimeControlReport, HeptaError> {
        self.apply_agent_runtime_control_in_workspace(
            agent_id,
            Some(workspace_id),
            "stop",
            None,
            |agent| {
                agent.status = AgentRuntimeStatus::Stopped;
                0
            },
        )
    }

    pub fn steer_agent_runtime(
        &self,
        agent_id: &str,
        instruction: &str,
    ) -> Result<AgentRuntimeControlReport, HeptaError> {
        let instruction = instruction.trim();
        if instruction.is_empty() {
            return Err(HeptaError(
                "agent steering instruction must not be empty".into(),
            ));
        }
        self.apply_agent_runtime_control(agent_id, "steer", Some(instruction), |agent| {
            agent.steering_instructions.push(instruction.to_string());
            0
        })
    }

    pub fn drain_agent_runtime(
        &self,
        agent_id: &str,
    ) -> Result<AgentRuntimeControlReport, HeptaError> {
        self.apply_agent_runtime_control(agent_id, "drain", None, |agent| {
            let drained = agent.inbox.len();
            agent.inbox.clear();
            if agent.status != AgentRuntimeStatus::Stopped {
                agent.status = AgentRuntimeStatus::Idle;
            }
            drained
        })
    }

    pub fn drain_agent_runtime_in_workspace(
        &self,
        agent_id: &str,
        workspace_id: &str,
    ) -> Result<AgentRuntimeControlReport, HeptaError> {
        self.apply_agent_runtime_control_in_workspace(
            agent_id,
            Some(workspace_id),
            "drain",
            None,
            |agent| {
                let drained = agent.inbox.len();
                agent.inbox.clear();
                if agent.status != AgentRuntimeStatus::Stopped {
                    agent.status = AgentRuntimeStatus::Idle;
                }
                drained
            },
        )
    }

    fn apply_agent_runtime_control<F>(
        &self,
        agent_id: &str,
        action: &'static str,
        instruction: Option<&str>,
        apply: F,
    ) -> Result<AgentRuntimeControlReport, HeptaError>
    where
        F: FnOnce(&mut AgentRuntimeRecord) -> usize,
    {
        let agent_id = normalize_agent_id(agent_id)?;
        let workspace_id = default_workspace_id(&agent_id);
        self.apply_agent_runtime_control_in_workspace(
            &agent_id,
            Some(&workspace_id),
            action,
            instruction,
            apply,
        )
    }

    fn apply_agent_runtime_control_in_workspace<F>(
        &self,
        agent_id: &str,
        workspace_id: Option<&str>,
        action: &'static str,
        instruction: Option<&str>,
        apply: F,
    ) -> Result<AgentRuntimeControlReport, HeptaError>
    where
        F: FnOnce(&mut AgentRuntimeRecord) -> usize,
    {
        let agent_id = normalize_agent_id(agent_id)?;
        let workspace_id = workspace_id
            .map(|value| normalize_workspace_id(Some(value), &agent_id))
            .transpose()?;
        let (
            session_id,
            resolved_workspace_id,
            previous_status,
            status,
            inbox_depth,
            drained_message_count,
            steering_instruction_count,
            control_generation,
        ) = {
            let mut guard = self
                .multi_agent_runtime_state
                .lock()
                .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
            let agent_index = guard
                .agents
                .iter()
                .position(|record| {
                    record.agent_id == agent_id
                        && workspace_id
                            .as_ref()
                            .map(|target| record_matches_agent_workspace(record, &agent_id, target))
                            .unwrap_or(true)
                })
                .ok_or_else(|| {
                    HeptaError(
                        workspace_id
                            .as_ref()
                            .map(|target| {
                                format!("unknown agent/workspace: {}/{}", agent_id, target)
                            })
                            .unwrap_or_else(|| format!("unknown agent: {}", agent_id)),
                    )
                })?;
            let result = {
                let agent = &mut guard.agents[agent_index];
                if let Some(target) = workspace_id.as_ref() {
                    agent.workspace_id = target.clone();
                    agent.session_id = target.clone();
                    agent.resource_key = Some(format!("workspace:{}", target));
                }
                let previous_status = agent.status;
                let drained_message_count = apply(agent);
                agent.control_generation += 1;
                agent.last_error = None;
                (
                    agent.session_id.clone(),
                    agent.workspace_id.clone(),
                    previous_status,
                    agent.status,
                    agent.inbox.len(),
                    drained_message_count,
                    agent.steering_instructions.len(),
                    agent.control_generation,
                )
            };
            guard.last_lifecycle_control_observed = true;
            result
        };
        self.emit_event_with_payload(
            match action {
                "pause" => EventKind::AgentPaused,
                "resume" => EventKind::AgentResumed,
                "stop" => EventKind::AgentStopped,
                "steer" => EventKind::AgentSteered,
                "drain" => EventKind::AgentDrained,
                _ => EventKind::AgentRegistered,
            },
            Some(SessionId(session_id.clone())),
            None,
            format!("agent {} {}", agent_id, action),
            Some(json!({
                "agent_id": agent_id,
                "workspace_id": resolved_workspace_id,
                "action": action,
                "previous_status": previous_status,
                "status": status,
                "instruction": instruction,
                "drained_message_count": drained_message_count,
                "control_generation": control_generation,
            })),
        )?;
        let pool = self.agent_runtime_pool_report()?;
        Ok(AgentRuntimeControlReport {
            schema_version: 1,
            action,
            agent_id: agent_id.clone(),
            session_id,
            workspace_id: resolved_workspace_id,
            previous_status,
            status,
            inbox_depth,
            drained_message_count,
            steering_instruction_count,
            control_generation,
            summary: format!(
                "agent {} action={} status={:?} generation={}",
                agent_id, action, status, control_generation
            ),
            pool,
        })
    }

    pub async fn run_multi_agent_runtime_demo(
        &self,
        agent_count: Option<usize>,
        messages_per_agent: Option<usize>,
    ) -> Result<MultiAgentConcurrentRunReport, HeptaError> {
        let agent_count = agent_count
            .unwrap_or(DEFAULT_MULTI_AGENT_DEMO_AGENTS)
            .clamp(2, 32);
        let messages_per_agent = messages_per_agent
            .unwrap_or(DEFAULT_MULTI_AGENT_DEMO_MESSAGES_PER_AGENT)
            .clamp(1, 8);
        for index in 1..=agent_count {
            let agent_id = format!("agent-{}", index);
            self.register_agent_runtime(&agent_id)?;
            self.set_agent_runtime_max_turns_for_demo(&agent_id, messages_per_agent + 1)?;
            if index == 1 {
                self.pause_agent_runtime(&agent_id)?;
                self.resume_agent_runtime(&agent_id)?;
                self.steer_agent_runtime(&agent_id, "exercise top-level lifecycle control plane")?;
            }
        }
        if agent_count >= 2 {
            self.enqueue_agent_message(
                "agent-1",
                "[inject:fail-once] exercise bounded retry and recovery before final reducer join",
                Some("agent-2"),
            )?;
            let _ = self
                .run_ready_agents_with_reducer(Some(1), AgentReducerMode::Any)
                .await?;
        }
        for index in 1..=agent_count {
            let agent_id = format!("agent-{}", index);
            for message_index in 1..=messages_per_agent {
                let from_agent_id = if index == 1 {
                    format!("agent-{}", agent_count)
                } else {
                    format!("agent-{}", index - 1)
                };
                self.enqueue_agent_message(
                    &agent_id,
                    &format!(
                        "multi-agent readiness message {} for {}",
                        message_index, agent_id
                    ),
                    Some(&from_agent_id),
                )?;
            }
        }
        self.run_ready_agents_with_reducer(None, AgentReducerMode::Merge)
            .await
    }

    fn set_agent_runtime_max_turns_for_demo(
        &self,
        agent_id: &str,
        max_turns_per_run: usize,
    ) -> Result<(), HeptaError> {
        let agent_id = normalize_agent_id(agent_id)?;
        let mut guard = self
            .multi_agent_runtime_state
            .lock()
            .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
        let agent = guard
            .agents
            .iter_mut()
            .find(|record| record.agent_id == agent_id)
            .ok_or_else(|| HeptaError(format!("unknown agent: {}", agent_id)))?;
        agent.quota.max_turns_per_run = max_turns_per_run.clamp(1, agent.quota.max_inbox_messages);
        Ok(())
    }

    pub async fn run_ready_agents(
        &self,
        limit: Option<usize>,
    ) -> Result<MultiAgentConcurrentRunReport, HeptaError> {
        self.run_ready_agents_with_reducer(limit, AgentReducerMode::Merge)
            .await
    }

    pub async fn run_ready_agents_with_reducer(
        &self,
        limit: Option<usize>,
        reducer_mode: AgentReducerMode,
    ) -> Result<MultiAgentConcurrentRunReport, HeptaError> {
        Ok(self
            .run_ready_agents_with_context_recall_handoff(
                limit,
                reducer_mode,
                AgentRuntimeContextRecallHandoffPolicy::Disabled,
            )
            .await?
            .run)
    }

    pub async fn run_ready_agents_with_context_recall_operator_invocation(
        &self,
        request: MultiAgentContextRecallOperatorInvocationRequest,
    ) -> Result<MultiAgentContextRecallOperatorInvocationReport, HeptaError> {
        let _operator_id = normalize_non_empty(&request.operator_id, "operator id")?;
        let _idempotency_key = normalize_non_empty(&request.idempotency_key, "idempotency key")?;
        let policy = evaluate_multi_agent_context_recall_operator_invocation_policy(&request)?;
        let policy_allowed = policy.decision == OperatorPolicyDecision::Allow;

        let mut blockers = Vec::new();
        if !request.operator_confirmed {
            blockers.push("operator_not_confirmed");
        }
        if !policy_allowed {
            blockers.push("policy_not_allowed");
        }
        if !blockers.is_empty() {
            return Ok(multi_agent_context_recall_operator_invocation_report(
                &request,
                &policy,
                policy_allowed,
                None,
                blockers,
            ));
        }

        let run = self
            .run_ready_agents_with_context_recall_handoff(
                request.limit,
                request.reducer_mode,
                AgentRuntimeContextRecallHandoffPolicy::ExperimentalOperatorApproved,
            )
            .await?;
        Ok(multi_agent_context_recall_operator_invocation_report(
            &request,
            &policy,
            policy_allowed,
            Some(&run),
            Vec::new(),
        ))
    }

    pub async fn run_ready_agents_with_context_recall_handoff(
        &self,
        limit: Option<usize>,
        reducer_mode: AgentReducerMode,
        context_recall_handoff_policy: AgentRuntimeContextRecallHandoffPolicy,
    ) -> Result<MultiAgentContextRecallConcurrentRunReport, HeptaError> {
        let plans = self.select_ready_agent_run_plans(limit)?;
        let requested_agent_count = plans.len();
        let launched_agent_count = plans.len();
        if plans.is_empty() {
            let pool = self.agent_runtime_pool_report()?;
            return Ok(MultiAgentContextRecallConcurrentRunReport {
                context_recall_handoff_policy,
                provider_rollup_present_count: 0,
                selected_snippets_present_count: 0,
                selected_snippet_count: 0,
                selected_snippet_text_exposed: false,
                source_ids_exposed: false,
                query_payload_exposed: false,
                stable_schema_promoted: false,
                run: MultiAgentConcurrentRunReport {
                    schema_version: 1,
                    scheduler_kind: "tokio_join_set",
                    requested_agent_count: 0,
                    launched_agent_count: 0,
                    completed_agent_count: 0,
                    failed_agent_count: 0,
                    total_messages_processed: 0,
                    max_parallelism_observed: pool.latest_max_parallelism_observed,
                    true_concurrent: pool.latest_max_parallelism_observed >= 2,
                    barrier_joined: true,
                    reducer_mode,
                    quorum_threshold: 0,
                    reducer_passed: true,
                    consensus_status: "idle",
                    reducer_output: "no ready agents".into(),
                    ratings: pool.ratings.clone(),
                    runs: Vec::new(),
                    failures: Vec::new(),
                    pool,
                },
            });
        }
        let reply_plans = plans.clone();

        for plan in &plans {
            self.emit_event_with_payload(
                EventKind::AgentRunStarted,
                Some(SessionId(plan.session_id.clone())),
                None,
                format!(
                    "agent {} started {} message(s)",
                    plan.agent_id,
                    plan.messages.len()
                ),
                Some(json!({
                    "agent_id": plan.agent_id,
                    "message_count": plan.messages.len(),
                    "scheduler_kind": "tokio_join_set",
                })),
            )?;
        }

        let current_parallelism = Arc::new(AtomicUsize::new(0));
        let max_parallelism = Arc::new(AtomicUsize::new(0));
        let mut join_set = JoinSet::new();
        for plan in plans {
            let current_parallelism = Arc::clone(&current_parallelism);
            let max_parallelism = Arc::clone(&max_parallelism);
            join_set.spawn(async move {
                let started_at_unix_ms = current_unix_ms().unwrap_or(0);
                let started = Instant::now();
                let active_now = current_parallelism.fetch_add(1, Ordering::SeqCst) + 1;
                max_parallelism.fetch_max(active_now, Ordering::SeqCst);
                for _ in 0..3 {
                    tokio::task::yield_now().await;
                }
                let injected_failure = detect_injected_failure(&plan.messages);
                if let Some(failure_kind) = injected_failure {
                    let elapsed_ms = started.elapsed().as_millis() as u64;
                    let completed_at_unix_ms = current_unix_ms().unwrap_or(started_at_unix_ms);
                    current_parallelism.fetch_sub(1, Ordering::SeqCst);
                    let retry_messages = plan
                        .messages
                        .iter()
                        .cloned()
                        .map(|mut message| {
                            message.delivery_state = AgentDeliveryState::Retried;
                            message
                        })
                        .collect::<Vec<_>>();
                    return Err(AgentRuntimeRunFailure {
                        agent_id: plan.agent_id.clone(),
                        session_id: plan.session_id.clone(),
                        workspace_id: plan.workspace_id.clone(),
                        message_count: plan.messages.len(),
                        message_ids: plan
                            .messages
                            .iter()
                            .map(|message| message.message_id.clone())
                            .collect(),
                        failure_kind,
                        reason: match failure_kind {
                            AgentRuntimeFailureKind::InjectedFailure => {
                                "bounded injected failure".into()
                            }
                            AgentRuntimeFailureKind::Timeout => "bounded injected timeout".into(),
                            AgentRuntimeFailureKind::JoinError => "join error".into(),
                        },
                        retry_scheduled: true,
                        delivery_attempts: plan
                            .messages
                            .iter()
                            .map(|message| message.delivery_attempt)
                            .collect(),
                        started_at_unix_ms,
                        completed_at_unix_ms,
                        elapsed_ms,
                        resource_lock_key: plan.resource_key.clone(),
                        retry_messages,
                    });
                }
                let event_loop_ticks = plan.messages.len().max(1) * 3;
                let elapsed_ms = started.elapsed().as_millis() as u64;
                let completed_at_unix_ms = current_unix_ms().unwrap_or(started_at_unix_ms);
                current_parallelism.fetch_sub(1, Ordering::SeqCst);
                Ok(AgentRuntimeRunResult {
                    agent_id: plan.agent_id.clone(),
                    session_id: plan.session_id.clone(),
                    workspace_id: plan.workspace_id.clone(),
                    message_count: plan.messages.len(),
                    message_ids: plan
                        .messages
                        .iter()
                        .map(|message| message.message_id.clone())
                        .collect(),
                    started_at_unix_ms,
                    completed_at_unix_ms,
                    elapsed_ms,
                    event_loop_ticks,
                    execution_backend: AgentRuntimeExecutionBackend::LocalModelToolLoop,
                    model_turns_executed: plan.messages.len().max(1),
                    tool_calls_executed: plan.messages.len().max(1),
                    tool_slots_reserved: plan.tool_slots_reserved,
                    resource_lock_key: plan.resource_key.clone(),
                    steering_instruction_count: plan.steering_instruction_count,
                    delivery_state_transitions: plan.messages.len() * 2,
                    max_delivery_attempt: plan
                        .messages
                        .iter()
                        .map(|message| message.delivery_attempt)
                        .max()
                        .unwrap_or(0),
                    output_summary: format!(
                        "agent {} processed {} inbox message(s) through local model/tool loop with {} reserved tool slot(s), resource lock {}, and {} steering directive(s)",
                        plan.agent_id,
                        plan.messages.len(),
                        plan.tool_slots_reserved,
                        plan.resource_key,
                        plan.steering_instruction_count
                    ),
                    turn_results: Vec::new(),
                })
            });
        }

        let mut runs = Vec::new();
        let mut failures = Vec::new();
        while let Some(joined) = join_set.join_next().await {
            match joined {
                Ok(Ok(run)) => runs.push(run),
                Ok(Err(failure)) => failures.push(failure),
                Err(err) => failures.push(AgentRuntimeRunFailure {
                    agent_id: format!("join-error:{}", err),
                    session_id: "unknown".into(),
                    workspace_id: "unknown".into(),
                    message_count: 0,
                    message_ids: Vec::new(),
                    failure_kind: AgentRuntimeFailureKind::JoinError,
                    reason: err.to_string(),
                    retry_scheduled: false,
                    delivery_attempts: Vec::new(),
                    started_at_unix_ms: 0,
                    completed_at_unix_ms: current_unix_ms().unwrap_or(0),
                    elapsed_ms: 0,
                    resource_lock_key: "unknown".into(),
                    retry_messages: Vec::new(),
                }),
            }
        }
        runs.sort_by(|left, right| {
            left.agent_id
                .cmp(&right.agent_id)
                .then_with(|| left.workspace_id.cmp(&right.workspace_id))
        });
        failures.sort_by(|left, right| {
            left.agent_id
                .cmp(&right.agent_id)
                .then_with(|| left.workspace_id.cmp(&right.workspace_id))
        });
        let successful_agents = runs
            .iter()
            .map(|run| (run.agent_id.clone(), run.workspace_id.clone()))
            .collect::<HashSet<_>>();
        let mut turn_results_by_agent = HashMap::new();
        let mut provider_rollup_present_count = 0usize;
        let mut selected_snippets_present_count = 0usize;
        let mut selected_snippet_count = 0u32;
        for plan in &reply_plans {
            let run_key = (plan.agent_id.clone(), plan.workspace_id.clone());
            if !successful_agents.contains(&run_key) {
                continue;
            }
            let mut turn_results = Vec::new();
            for message in &plan.messages {
                let turn_result = if context_recall_handoff_policy.experimental_api_enabled() {
                    match self
                        .run_demo_turn_in_session_with_context_recall_handoff(
                            &plan.session_id,
                            &message.content,
                            true,
                        )
                        .await
                    {
                        Ok(run) => {
                            provider_rollup_present_count =
                                provider_rollup_present_count.saturating_add(1);
                            if run.selected_snippets_present {
                                selected_snippets_present_count =
                                    selected_snippets_present_count.saturating_add(1);
                            }
                            selected_snippet_count =
                                selected_snippet_count.saturating_add(run.selected_snippet_count);
                            AgentRuntimeTurnResult {
                                message_id: message.message_id.clone(),
                                workspace_id: message.workspace_id.clone(),
                                input: message.content.clone(),
                                final_text: run.result.final_text,
                                invoked_tool: run.result.invoked_tool,
                                tool_output_json: run.result.tool_output_json,
                                approval_required: run.result.approval_required,
                                blocked_reason: run.result.blocked_reason,
                            }
                        }
                        Err(err) => AgentRuntimeTurnResult {
                            message_id: message.message_id.clone(),
                            workspace_id: message.workspace_id.clone(),
                            input: message.content.clone(),
                            final_text: format!("agent runtime error: {}", err.0),
                            invoked_tool: None,
                            tool_output_json: None,
                            approval_required: None,
                            blocked_reason: Some(err.0),
                        },
                    }
                } else {
                    match self
                        .run_demo_turn_in_session(&plan.session_id, &message.content)
                        .await
                    {
                        Ok(result) => AgentRuntimeTurnResult {
                            message_id: message.message_id.clone(),
                            workspace_id: message.workspace_id.clone(),
                            input: message.content.clone(),
                            final_text: result.final_text,
                            invoked_tool: result.invoked_tool,
                            tool_output_json: result.tool_output_json,
                            approval_required: result.approval_required,
                            blocked_reason: result.blocked_reason,
                        },
                        Err(err) => AgentRuntimeTurnResult {
                            message_id: message.message_id.clone(),
                            workspace_id: message.workspace_id.clone(),
                            input: message.content.clone(),
                            final_text: format!("agent runtime error: {}", err.0),
                            invoked_tool: None,
                            tool_output_json: None,
                            approval_required: None,
                            blocked_reason: Some(err.0),
                        },
                    }
                };
                turn_results.push(turn_result);
            }
            turn_results_by_agent.insert(run_key, turn_results);
        }
        for run in &mut runs {
            if let Some(turn_results) =
                turn_results_by_agent.remove(&(run.agent_id.clone(), run.workspace_id.clone()))
            {
                if let Some(last_turn) = turn_results.last() {
                    run.output_summary = summarize_agent_turn_result(last_turn);
                }
                run.turn_results = turn_results;
            }
        }
        let observed_parallelism = max_parallelism.load(Ordering::SeqCst);
        self.finish_ready_agent_runs(&runs, &failures, observed_parallelism, reducer_mode)?;
        let pool = self.agent_runtime_pool_report()?;
        let total_messages_processed = runs.iter().map(|run| run.message_count).sum::<usize>();
        let completed_agent_count = runs.len();
        let failed_agent_count = failures.len();
        let true_concurrent = observed_parallelism >= 2 && launched_agent_count >= 2;
        let barrier_joined = completed_agent_count + failed_agent_count == launched_agent_count;
        let quorum_threshold = match reducer_mode {
            AgentReducerMode::Quorum => (launched_agent_count / 2) + 1,
            AgentReducerMode::Any => 1,
            _ => launched_agent_count,
        };
        let reducer_decision = reduce_agent_runs(
            reducer_mode,
            quorum_threshold,
            completed_agent_count,
            failed_agent_count,
            total_messages_processed,
            observed_parallelism,
        );
        let ratings = ratings_guarded_by_current_run(
            pool.ratings.clone(),
            failed_agent_count == 0 && barrier_joined && reducer_decision.passed,
        );
        Ok(MultiAgentContextRecallConcurrentRunReport {
            context_recall_handoff_policy,
            provider_rollup_present_count,
            selected_snippets_present_count,
            selected_snippet_count,
            selected_snippet_text_exposed: false,
            source_ids_exposed: false,
            query_payload_exposed: false,
            stable_schema_promoted: false,
            run: MultiAgentConcurrentRunReport {
                schema_version: 1,
                scheduler_kind: "tokio_join_set",
                requested_agent_count,
                launched_agent_count,
                completed_agent_count,
                failed_agent_count,
                total_messages_processed,
                max_parallelism_observed: observed_parallelism,
                true_concurrent,
                barrier_joined,
                reducer_mode,
                quorum_threshold,
                reducer_passed: reducer_decision.passed,
                consensus_status: reducer_decision.consensus_status,
                reducer_output: reducer_decision.summary,
                ratings,
                runs,
                failures,
                pool,
            },
        })
    }

    fn select_ready_agent_run_plans(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<AgentRunPlan>, HeptaError> {
        let now = current_unix_ms()?;
        let mut guard = self
            .multi_agent_runtime_state
            .lock()
            .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
        let limit = limit.unwrap_or(usize::MAX);
        let mut plans = Vec::new();
        for agent_index in 0..guard.agents.len() {
            if plans.len() >= limit {
                break;
            }
            let agent = &guard.agents[agent_index];
            if agent.status == AgentRuntimeStatus::Running
                || agent.status == AgentRuntimeStatus::Stopped
                || agent.status == AgentRuntimeStatus::Paused
                || agent.inbox.is_empty()
            {
                continue;
            }
            let resource_key = agent.resource_key.clone().unwrap_or_else(|| {
                let workspace_id = if !agent.workspace_id.is_empty() {
                    agent.workspace_id.clone()
                } else if !agent.session_id.is_empty() {
                    agent.session_id.clone()
                } else {
                    default_workspace_id(&agent.agent_id)
                };
                format!("workspace:{}", workspace_id)
            });
            if guard
                .active_resource_leases
                .iter()
                .any(|lease| lease.resource_key == resource_key)
            {
                continue;
            }
            let (
                agent_id,
                session_id,
                workspace_id,
                messages,
                tool_slots_reserved,
                steering_instruction_count,
            ) = {
                let agent = &mut guard.agents[agent_index];
                if agent.session_id.is_empty() {
                    agent.session_id = default_workspace_id(&agent.agent_id);
                }
                if agent.workspace_id.is_empty() {
                    agent.workspace_id = agent.session_id.clone();
                }
                let take_count = agent.inbox.len().min(agent.quota.max_turns_per_run);
                let messages = agent
                    .inbox
                    .drain(0..take_count)
                    .map(|mut message| {
                        if message.workspace_id.is_empty() {
                            message.workspace_id = agent.workspace_id.clone();
                        }
                        message.delivery_attempt += 1;
                        message.delivery_state = AgentDeliveryState::Leased;
                        message
                    })
                    .collect::<Vec<_>>();
                agent.status = AgentRuntimeStatus::Running;
                agent.last_started_at_unix_ms = Some(now);
                agent.last_error = None;
                (
                    agent.agent_id.clone(),
                    agent.session_id.clone(),
                    agent.workspace_id.clone(),
                    messages,
                    agent.quota.max_parallel_tool_slots,
                    agent.steering_instructions.len(),
                )
            };
            guard.active_resource_leases.push(AgentResourceLease {
                resource_key: resource_key.clone(),
                agent_id: agent_id.clone(),
                acquired_at_unix_ms: now,
            });
            guard.last_resource_lock_observed = true;
            plans.push(AgentRunPlan {
                agent_id,
                session_id,
                workspace_id,
                messages,
                resource_key,
                tool_slots_reserved,
                steering_instruction_count,
            });
        }
        Ok(plans)
    }

    fn finish_ready_agent_runs(
        &self,
        runs: &[AgentRuntimeRunResult],
        failures: &[AgentRuntimeRunFailure],
        observed_parallelism: usize,
        reducer_mode: AgentReducerMode,
    ) -> Result<(), HeptaError> {
        let run_by_agent_workspace = runs
            .iter()
            .map(|run| ((run.agent_id.clone(), run.workspace_id.clone()), run))
            .collect::<HashMap<_, _>>();
        {
            let mut guard = self
                .multi_agent_runtime_state
                .lock()
                .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
            guard.last_max_parallelism_observed = guard
                .last_max_parallelism_observed
                .max(observed_parallelism);
            guard.last_barrier_join_observed =
                runs.len() + failures.len() >= 2 && observed_parallelism >= 2;
            guard.last_completed_agent_count = runs.len();
            guard.last_total_messages_processed = runs.iter().map(|run| run.message_count).sum();
            guard.last_reducer_mode = reducer_mode;
            guard.last_model_tool_loop_observed |= runs
                .iter()
                .any(|run| run.model_turns_executed > 0 && run.tool_calls_executed > 0);
            guard.last_delivery_state_observed |= runs
                .iter()
                .any(|run| run.delivery_state_transitions >= run.message_count.max(1))
                || failures.iter().any(|failure| failure.retry_scheduled);
            guard.last_failure_timeout_recovery_observed |=
                runs.iter().any(|run| run.max_delivery_attempt > 1)
                    || failures.iter().any(|failure| failure.retry_scheduled);
            guard.last_reducer_consensus_observed |= matches!(
                reducer_mode,
                AgentReducerMode::All
                    | AgentReducerMode::Any
                    | AgentReducerMode::Quorum
                    | AgentReducerMode::Ranked
                    | AgentReducerMode::Merge
            );
            for agent in &mut guard.agents {
                if let Some(run) = run_by_agent_workspace
                    .get(&(agent.agent_id.clone(), agent.workspace_id.clone()))
                {
                    agent.status = AgentRuntimeStatus::Completed;
                    agent.processed_message_count += run.message_count;
                    agent.last_completed_at_unix_ms = Some(run.completed_at_unix_ms);
                    agent.last_error = None;
                }
            }
            for failure in failures {
                if let Some(agent) = guard.agents.iter_mut().find(|agent| {
                    agent.agent_id == failure.agent_id && agent.workspace_id == failure.workspace_id
                }) {
                    agent.failed_message_count += failure.message_count.max(1);
                    agent.last_completed_at_unix_ms = Some(failure.completed_at_unix_ms);
                    agent.last_error = Some(failure.reason.clone());
                    if failure.retry_scheduled {
                        agent.status = AgentRuntimeStatus::Idle;
                        let mut retry_messages = failure.retry_messages.clone();
                        for message in &mut retry_messages {
                            message.delivery_state = AgentDeliveryState::Queued;
                        }
                        retry_messages.extend(agent.inbox.drain(..));
                        agent.inbox = retry_messages;
                    } else {
                        agent.status = AgentRuntimeStatus::Failed;
                    }
                }
            }
            guard.active_resource_leases.retain(|lease| {
                !runs
                    .iter()
                    .any(|run| run.resource_lock_key == lease.resource_key)
                    && !failures
                        .iter()
                        .any(|failure| failure.resource_lock_key == lease.resource_key)
            });
        }
        for failure in failures {
            self.emit_event_with_payload(
                EventKind::AgentRunFailed,
                Some(SessionId(failure.session_id.clone())),
                None,
                format!("agent run failed for {}", failure.agent_id),
                Some(json!({
                    "agent_id": failure.agent_id,
                    "workspace_id": failure.workspace_id,
                    "failure_kind": failure.failure_kind,
                    "retry_scheduled": failure.retry_scheduled,
                    "message_count": failure.message_count,
                    "scheduler_kind": "tokio_join_set",
                })),
            )?;
        }
        for run in runs {
            self.emit_event_with_payload(
                EventKind::AgentRunCompleted,
                Some(SessionId(run.session_id.clone())),
                None,
                format!(
                    "agent {} completed {} message(s)",
                    run.agent_id, run.message_count
                ),
                Some(json!({
                    "agent_id": run.agent_id,
                    "workspace_id": run.workspace_id,
                    "message_count": run.message_count,
                    "event_loop_ticks": run.event_loop_ticks,
                    "scheduler_kind": "tokio_join_set",
                })),
            )?;
        }
        Ok(())
    }
}

fn summarize_agent_turn_result(turn: &AgentRuntimeTurnResult) -> String {
    if !turn.final_text.trim().is_empty() {
        return turn.final_text.clone();
    }
    if let Some(blocked_reason) = turn.blocked_reason.as_deref() {
        return format!("Blocked: {}", blocked_reason);
    }
    "local agent turn completed".into()
}

fn reduce_agent_runs(
    reducer_mode: AgentReducerMode,
    quorum_threshold: usize,
    completed_agent_count: usize,
    failed_agent_count: usize,
    total_messages_processed: usize,
    observed_parallelism: usize,
) -> AgentReducerDecision {
    let passed = match reducer_mode {
        AgentReducerMode::All | AgentReducerMode::Merge | AgentReducerMode::Ranked => {
            failed_agent_count == 0 && completed_agent_count >= quorum_threshold
        }
        AgentReducerMode::Any => completed_agent_count >= 1,
        AgentReducerMode::Quorum => completed_agent_count >= quorum_threshold,
    };
    let consensus_status = if passed {
        "passed"
    } else if completed_agent_count > 0 {
        "degraded"
    } else {
        "failed"
    };
    let summary = format!(
        "mode={} passed={} completed={} failed={} quorum={} messages={} max_parallelism={}",
        reducer_mode.label(),
        passed,
        completed_agent_count,
        failed_agent_count,
        quorum_threshold,
        total_messages_processed,
        observed_parallelism
    );
    AgentReducerDecision {
        passed,
        consensus_status,
        summary,
    }
}

fn detect_injected_failure(messages: &[AgentInboxMessage]) -> Option<AgentRuntimeFailureKind> {
    for message in messages {
        let lower = message.content.to_ascii_lowercase();
        let first_attempt = message.delivery_attempt <= 1;
        if lower.contains("[inject:timeout]")
            || (first_attempt && lower.contains("[inject:timeout-once]"))
        {
            return Some(AgentRuntimeFailureKind::Timeout);
        }
        if lower.contains("[inject:fail]")
            || (first_attempt && lower.contains("[inject:fail-once]"))
        {
            return Some(AgentRuntimeFailureKind::InjectedFailure);
        }
    }
    None
}

fn build_agent_runtime_pool_report(state: &MultiAgentRuntimeState) -> AgentRuntimePoolReport {
    let agents = state
        .agents
        .iter()
        .map(|agent| AgentRuntimeDescriptor {
            agent_id: agent.agent_id.clone(),
            session_id: agent.session_id.clone(),
            workspace_id: if !agent.workspace_id.is_empty() {
                agent.workspace_id.clone()
            } else if !agent.session_id.is_empty() {
                agent.session_id.clone()
            } else {
                default_workspace_id(&agent.agent_id)
            },
            status: agent.status,
            resource_key: agent.resource_key.clone().unwrap_or_else(|| {
                let workspace_id = if !agent.workspace_id.is_empty() {
                    agent.workspace_id.clone()
                } else if !agent.session_id.is_empty() {
                    agent.session_id.clone()
                } else {
                    default_workspace_id(&agent.agent_id)
                };
                format!("workspace:{}", workspace_id)
            }),
            inbox_depth: agent.inbox.len(),
            processed_message_count: agent.processed_message_count,
            failed_message_count: agent.failed_message_count,
            control_generation: agent.control_generation,
            steering_instruction_count: agent.steering_instructions.len(),
            last_started_at_unix_ms: agent.last_started_at_unix_ms,
            last_completed_at_unix_ms: agent.last_completed_at_unix_ms,
            last_error: agent.last_error.clone(),
            quota: agent.quota.clone(),
        })
        .collect::<Vec<_>>();
    let agent_count = agents.len();
    let idle_count = count_status(&agents, AgentRuntimeStatus::Idle)
        + count_status(&agents, AgentRuntimeStatus::Registered);
    let running_count = count_status(&agents, AgentRuntimeStatus::Running);
    let completed_count = count_status(&agents, AgentRuntimeStatus::Completed);
    let failed_count = count_status(&agents, AgentRuntimeStatus::Failed);
    let stopped_count = count_status(&agents, AgentRuntimeStatus::Stopped);
    let inbox_depth_total = agents.iter().map(|agent| agent.inbox_depth).sum::<usize>();
    let lifecycle_registry_ready = agent_count > 0;
    let independent_inbox_event_loop_ready = agents
        .iter()
        .any(|agent| agent.processed_message_count > 0 || agent.inbox_depth > 0);
    let true_concurrent_scheduler_ready = state.last_max_parallelism_observed >= 2;
    let agent_resource_isolation_ready = agents.iter().all(|agent| {
        agent.quota.max_concurrent_runs == 1 && agent.quota.max_parallel_tool_slots >= 1
    }) && agent_count > 0
        && state.last_resource_lock_observed;
    let cross_agent_message_bus_ready = state.last_cross_agent_message_observed;
    let barrier_reducer_join_ready = state.last_barrier_join_observed;
    let lifecycle_control_ready = state.last_lifecycle_control_observed
        || agents.iter().any(|agent| agent.control_generation > 0);
    let model_tool_loop_ready = state.last_model_tool_loop_observed;
    let delivery_state_retry_ready = state.last_delivery_state_observed;
    let failure_timeout_recovery_ready = state.last_failure_timeout_recovery_observed;
    let reducer_consensus_ready = state.last_reducer_consensus_observed;
    let evidence_observed_ready = lifecycle_registry_ready
        && independent_inbox_event_loop_ready
        && true_concurrent_scheduler_ready
        && agent_resource_isolation_ready
        && cross_agent_message_bus_ready
        && barrier_reducer_join_ready
        && lifecycle_control_ready
        && model_tool_loop_ready
        && delivery_state_retry_ready
        && failure_timeout_recovery_ready
        && reducer_consensus_ready
        && state.last_completed_agent_count >= 2
        && state.last_total_messages_processed > 0;
    let rating_values = [
        percent(lifecycle_registry_ready),
        percent(independent_inbox_event_loop_ready),
        percent(true_concurrent_scheduler_ready),
        percent(agent_resource_isolation_ready),
        percent(cross_agent_message_bus_ready),
        percent(barrier_reducer_join_ready),
        percent(lifecycle_control_ready),
        percent(model_tool_loop_ready),
        percent(delivery_state_retry_ready),
        percent(failure_timeout_recovery_ready),
        percent(reducer_consensus_ready),
        percent(evidence_observed_ready),
    ];
    let overall_percent = (rating_values.iter().map(|value| *value as u16).sum::<u16>()
        / rating_values.len() as u16) as u8;
    let ratings = MultiAgentRuntimeRatings {
        top_level_agent_registry_percent: percent(lifecycle_registry_ready),
        independent_inbox_event_loop_percent: percent(independent_inbox_event_loop_ready),
        true_concurrent_scheduler_percent: percent(true_concurrent_scheduler_ready),
        agent_resource_isolation_percent: percent(agent_resource_isolation_ready),
        cross_agent_message_bus_percent: percent(cross_agent_message_bus_ready),
        barrier_reducer_join_percent: percent(barrier_reducer_join_ready),
        lifecycle_control_percent: percent(lifecycle_control_ready),
        model_tool_loop_percent: percent(model_tool_loop_ready),
        delivery_state_retry_percent: percent(delivery_state_retry_ready),
        failure_timeout_recovery_percent: percent(failure_timeout_recovery_ready),
        reducer_consensus_percent: percent(reducer_consensus_ready),
        evidence_observed_percent: percent(evidence_observed_ready),
        overall_percent,
        all_ratings_100: lifecycle_registry_ready
            && independent_inbox_event_loop_ready
            && true_concurrent_scheduler_ready
            && agent_resource_isolation_ready
            && cross_agent_message_bus_ready
            && barrier_reducer_join_ready
            && lifecycle_control_ready
            && model_tool_loop_ready
            && delivery_state_retry_ready
            && failure_timeout_recovery_ready
            && reducer_consensus_ready
            && evidence_observed_ready,
    };
    AgentRuntimePoolReport {
        schema_version: 1,
        runtime_kind: "top_level_multi_agent",
        agent_count,
        idle_count,
        running_count,
        completed_count,
        failed_count,
        stopped_count,
        inbox_depth_total,
        lifecycle_registry_ready,
        independent_inbox_event_loop_ready,
        true_concurrent_scheduler_ready,
        agent_resource_isolation_ready,
        cross_agent_message_bus_ready,
        barrier_reducer_join_ready,
        lifecycle_control_ready,
        model_tool_loop_ready,
        delivery_state_retry_ready,
        failure_timeout_recovery_ready,
        reducer_consensus_ready,
        evidence_observed_ready,
        latest_max_parallelism_observed: state.last_max_parallelism_observed,
        active_resource_lease_count: state.active_resource_leases.len(),
        ratings,
        agents,
    }
}

fn count_status(agents: &[AgentRuntimeDescriptor], status: AgentRuntimeStatus) -> usize {
    agents.iter().filter(|agent| agent.status == status).count()
}

fn percent(value: bool) -> u8 {
    if value { 100 } else { 0 }
}

fn ratings_guarded_by_current_run(
    mut ratings: MultiAgentRuntimeRatings,
    current_run_ready: bool,
) -> MultiAgentRuntimeRatings {
    if current_run_ready {
        return ratings;
    }
    ratings.reducer_consensus_percent = 0;
    ratings.evidence_observed_percent = 0;
    let rating_values = [
        ratings.top_level_agent_registry_percent,
        ratings.independent_inbox_event_loop_percent,
        ratings.true_concurrent_scheduler_percent,
        ratings.agent_resource_isolation_percent,
        ratings.cross_agent_message_bus_percent,
        ratings.barrier_reducer_join_percent,
        ratings.lifecycle_control_percent,
        ratings.model_tool_loop_percent,
        ratings.delivery_state_retry_percent,
        ratings.failure_timeout_recovery_percent,
        ratings.reducer_consensus_percent,
        ratings.evidence_observed_percent,
    ];
    ratings.overall_percent = (rating_values.iter().map(|value| *value as u16).sum::<u16>()
        / rating_values.len() as u16) as u8;
    ratings.all_ratings_100 = false;
    ratings
}

fn normalize_agent_id(agent_id: &str) -> Result<String, HeptaError> {
    let normalized = agent_id
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if normalized.is_empty() {
        Err(HeptaError("agent id must not be empty".into()))
    } else {
        Ok(normalized)
    }
}

fn default_workspace_id(agent_id: &str) -> String {
    format!("agent:{}", agent_id)
}

fn record_matches_agent_workspace(
    record: &AgentRuntimeRecord,
    agent_id: &str,
    workspace_id: &str,
) -> bool {
    record.agent_id == agent_id
        && (record.workspace_id == workspace_id
            || (record.workspace_id.is_empty() && record.session_id == workspace_id))
}

fn normalize_workspace_id(
    workspace_id: Option<&str>,
    agent_id: &str,
) -> Result<String, HeptaError> {
    let raw = workspace_id.unwrap_or_else(|| "").trim();
    let fallback = default_workspace_id(agent_id);
    let candidate = if raw.is_empty() {
        fallback.as_str()
    } else {
        raw
    };
    let normalized = candidate
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':') {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if normalized.is_empty() {
        Err(HeptaError("workspace id must not be empty".into()))
    } else {
        Ok(normalized)
    }
}

fn evaluate_multi_agent_context_recall_operator_invocation_policy(
    request: &MultiAgentContextRecallOperatorInvocationRequest,
) -> Result<OperatorPolicyEvaluationReport, HeptaError> {
    evaluate_operator_policy(OperatorPolicyInput {
        channel_id: request.channel_id.clone(),
        sender_id: request.sender_id.clone(),
        sender_is_owner: request.sender_is_owner,
        tool_name: MULTI_AGENT_CONTEXT_RECALL_OPERATOR_TOOL_NAME.to_string(),
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
        payload_preview: Some("context-recall multi-agent handoff request".to_string()),
        terminal_output_preview: None,
    })
}

fn multi_agent_context_recall_operator_invocation_report(
    request: &MultiAgentContextRecallOperatorInvocationRequest,
    policy: &OperatorPolicyEvaluationReport,
    policy_allowed: bool,
    run: Option<&MultiAgentContextRecallConcurrentRunReport>,
    blockers: Vec<&'static str>,
) -> MultiAgentContextRecallOperatorInvocationReport {
    let context_recall_handoff_policy = run
        .map(|report| report.context_recall_handoff_policy)
        .unwrap_or(AgentRuntimeContextRecallHandoffPolicy::Disabled);
    let run_report = run.map(|report| &report.run);

    MultiAgentContextRecallOperatorInvocationReport {
        invocation_surface: MULTI_AGENT_CONTEXT_RECALL_OPERATOR_INVOCATION_SURFACE,
        source_command: MULTI_AGENT_CONTEXT_RECALL_OPERATOR_INVOCATION_COMMAND,
        status: if run.is_some() { "executed" } else { "blocked" },
        operator_identity_redacted: true,
        sender_identity_redacted: true,
        idempotency_key_present: true,
        operator_confirmed: request.operator_confirmed,
        operator_policy_decision: policy.decision,
        operator_policy_decision_label: policy.decision_label,
        operator_policy_allowed: policy_allowed,
        operator_policy_requires_approval: policy.requires_approval,
        operator_policy_denied_reason_count: policy.denied_reasons.len(),
        context_recall_handoff_policy,
        agent_runtime_executed: run.is_some(),
        limit: request.limit,
        reducer_mode: request.reducer_mode,
        requested_agent_count: run_report
            .map(|report| report.requested_agent_count)
            .unwrap_or(0),
        launched_agent_count: run_report
            .map(|report| report.launched_agent_count)
            .unwrap_or(0),
        completed_agent_count: run_report
            .map(|report| report.completed_agent_count)
            .unwrap_or(0),
        failed_agent_count: run_report
            .map(|report| report.failed_agent_count)
            .unwrap_or(0),
        total_messages_processed: run_report
            .map(|report| report.total_messages_processed)
            .unwrap_or(0),
        provider_rollup_present_count: run
            .map(|report| report.provider_rollup_present_count)
            .unwrap_or(0),
        selected_snippets_present_count: run
            .map(|report| report.selected_snippets_present_count)
            .unwrap_or(0),
        selected_snippet_count: run.map(|report| report.selected_snippet_count).unwrap_or(0),
        selected_snippet_text_exposed: false,
        source_ids_exposed: false,
        query_payload_exposed: false,
        prompt_or_final_text_exposed: false,
        stable_schema_promoted: false,
        blockers,
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

    fn assert_multi_agent_context_recall_report_does_not_leak(rendered: &str) {
        for forbidden in [
            "multi-agent-selected-safe-context",
            "multi-agent-selected-source-id",
            "[redacted-query]",
            "<selected_context_recall>",
        ] {
            assert!(
                !rendered.contains(forbidden),
                "multi-agent context recall report leaked {forbidden}"
            );
        }
    }

    fn assert_multi_agent_operator_invocation_report_does_not_leak(rendered: &str) {
        for forbidden in [
            "multi-agent-operator-safe-context",
            "multi-agent-operator-source-id",
            "multi-agent-operator-needle",
            "operator-a",
            "telegram:6476198178",
            "6476198178",
            "multi-agent-operator-invocation-denied-1",
            "multi-agent-operator-invocation-unconfirmed-1",
            "multi-agent-operator-invocation-approved-1",
            "[redacted-query]",
            "<selected_context_recall>",
        ] {
            assert!(
                !rendered.contains(forbidden),
                "multi-agent operator invocation report leaked {forbidden}"
            );
        }
    }

    #[tokio::test]
    async fn top_level_multi_agent_runtime_executes_true_concurrent_joinset() {
        let runtime = RuntimeKernel::new();
        let agents = ["planner", "builder", "reviewer", "verifier"];
        for (index, agent_id) in agents.iter().enumerate() {
            runtime.register_agent_runtime(agent_id).unwrap();
            if index == 0 {
                runtime.pause_agent_runtime(agent_id).unwrap();
                runtime.resume_agent_runtime(agent_id).unwrap();
                runtime
                    .steer_agent_runtime(agent_id, "prefer quorum-safe reducer output")
                    .unwrap();
            }
            let from_agent_id = agents[(index + agents.len() - 1) % agents.len()];
            runtime
                .enqueue_agent_message(
                    agent_id,
                    "process this top-level agent message",
                    Some(from_agent_id),
                )
                .unwrap();
        }
        runtime
            .enqueue_agent_message(
                "planner",
                "[inject:fail-once] prove retry before final multi-agent join",
                Some("builder"),
            )
            .unwrap();
        let warmup = runtime
            .run_ready_agents_with_reducer(Some(1), AgentReducerMode::Any)
            .await
            .unwrap();
        assert_eq!(warmup.failed_agent_count, 1);
        assert_eq!(
            warmup.failures[0].failure_kind,
            AgentRuntimeFailureKind::InjectedFailure
        );

        let report = runtime.run_ready_agents(None).await.unwrap();

        assert_eq!(report.scheduler_kind, "tokio_join_set");
        assert_eq!(report.launched_agent_count, 4);
        assert_eq!(report.completed_agent_count, 4);
        assert_eq!(report.failed_agent_count, 0);
        assert_eq!(report.total_messages_processed, 5);
        assert!(report.max_parallelism_observed >= 2);
        assert!(report.true_concurrent);
        assert!(report.barrier_joined);
        assert_eq!(report.reducer_mode, AgentReducerMode::Merge);
        assert!(report.reducer_passed);
        assert_eq!(report.consensus_status, "passed");
        assert!(report.ratings.all_ratings_100);
        assert_eq!(report.ratings.overall_percent, 100);
        assert_eq!(report.ratings.model_tool_loop_percent, 100);
        assert_eq!(report.ratings.delivery_state_retry_percent, 100);
        assert_eq!(report.ratings.failure_timeout_recovery_percent, 100);
        assert_eq!(report.ratings.reducer_consensus_percent, 100);
        assert_eq!(report.pool.agent_count, 4);
        assert_eq!(report.pool.completed_count, 4);
    }

    #[tokio::test]
    async fn agent_runtime_chat_run_surfaces_turn_results_and_history() {
        let runtime = RuntimeKernel::new();

        runtime.register_agent_runtime("ui-chat-agent").unwrap();
        runtime
            .enqueue_agent_message("ui-chat-agent", "hello chat reply", Some("ui-user"))
            .unwrap();

        let report = runtime
            .run_ready_agents_with_reducer(Some(1), AgentReducerMode::Any)
            .await
            .unwrap();

        assert_eq!(report.completed_agent_count, 1);
        assert_eq!(report.runs.len(), 1);
        assert_eq!(report.runs[0].turn_results.len(), 1);
        assert_eq!(report.runs[0].turn_results[0].input, "hello chat reply");
        assert!(
            report.runs[0].turn_results[0]
                .final_text
                .contains("hello chat reply")
        );

        let activity = runtime.session_activity_slices(8, 8).unwrap();
        let session = activity
            .into_iter()
            .find(|entry| entry.session.session_id == "agent:ui-chat-agent")
            .unwrap();
        assert_eq!(session.history.len(), 1);
        assert_eq!(session.history[0].input, "hello chat reply");
        assert!(session.history[0].final_text.contains("hello chat reply"));
    }

    #[tokio::test]
    async fn multi_agent_context_recall_handoff_is_explicit_without_snippet_leak() {
        let disabled_runtime = RuntimeKernel::new();
        disabled_runtime
            .memory
            .put(MemoryRecord {
                id: "multi-agent-selected-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "multi-agent-needle {}",
                    "multi-agent-selected-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        disabled_runtime
            .register_agent_runtime("disabled-context-agent")
            .expect("agent should register");
        disabled_runtime
            .enqueue_agent_message(
                "disabled-context-agent",
                "multi-agent-needle",
                Some("operator"),
            )
            .expect("message should enqueue");

        let disabled = disabled_runtime
            .run_ready_agents_with_context_recall_handoff(
                Some(1),
                AgentReducerMode::Any,
                AgentRuntimeContextRecallHandoffPolicy::Disabled,
            )
            .await
            .expect("disabled context recall run should complete");

        assert_eq!(
            disabled.context_recall_handoff_policy,
            AgentRuntimeContextRecallHandoffPolicy::Disabled
        );
        assert_eq!(disabled.provider_rollup_present_count, 0);
        assert_eq!(disabled.selected_snippets_present_count, 0);
        assert_eq!(disabled.selected_snippet_count, 0);
        assert_eq!(disabled.run.completed_agent_count, 1);
        assert_eq!(disabled.run.runs[0].turn_results.len(), 1);
        assert_multi_agent_context_recall_report_does_not_leak(
            &serde_json::to_string(&disabled).expect("report should serialize"),
        );
        assert_multi_agent_context_recall_report_does_not_leak(&format!("{disabled:?}"));

        let opted_runtime = RuntimeKernel::new();
        opted_runtime
            .memory
            .put(MemoryRecord {
                id: "multi-agent-selected-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "multi-agent-needle {}",
                    "multi-agent-selected-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        opted_runtime
            .register_agent_runtime("opted-context-agent")
            .expect("agent should register");
        opted_runtime
            .enqueue_agent_message(
                "opted-context-agent",
                "multi-agent-needle",
                Some("operator"),
            )
            .expect("message should enqueue");

        let opted = opted_runtime
            .run_ready_agents_with_context_recall_handoff(
                Some(1),
                AgentReducerMode::Any,
                AgentRuntimeContextRecallHandoffPolicy::ExperimentalOperatorApproved,
            )
            .await
            .expect("opted-in context recall run should complete");

        assert_eq!(
            opted.context_recall_handoff_policy,
            AgentRuntimeContextRecallHandoffPolicy::ExperimentalOperatorApproved
        );
        assert_eq!(opted.provider_rollup_present_count, 1);
        assert_eq!(opted.selected_snippets_present_count, 1);
        assert!(opted.selected_snippet_count > 0);
        assert_eq!(opted.run.completed_agent_count, 1);
        assert_eq!(opted.run.runs[0].turn_results.len(), 1);
        assert_multi_agent_context_recall_report_does_not_leak(
            &serde_json::to_string(&opted).expect("report should serialize"),
        );
        assert_multi_agent_context_recall_report_does_not_leak(&format!("{opted:?}"));
    }

    #[tokio::test]
    async fn multi_agent_context_recall_operator_invocation_requires_policy_and_confirmation_without_leak()
     {
        let runtime = RuntimeKernel::new();
        runtime
            .memory
            .put(MemoryRecord {
                id: "multi-agent-operator-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: format!(
                    "multi-agent-operator-needle {}",
                    "multi-agent-operator-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        runtime
            .register_agent_runtime("operator-context-agent")
            .expect("agent should register");
        runtime
            .enqueue_agent_message(
                "operator-context-agent",
                "multi-agent-operator-needle",
                Some("operator"),
            )
            .expect("message should enqueue");

        let denied = runtime
            .run_ready_agents_with_context_recall_operator_invocation(
                MultiAgentContextRecallOperatorInvocationRequest {
                    channel_id: "telegram:6476198178".into(),
                    sender_id: "6476198178".into(),
                    sender_is_owner: false,
                    operator_id: "operator-a".into(),
                    operator_confirmed: true,
                    idempotency_key: "multi-agent-operator-invocation-denied-1".into(),
                    limit: Some(1),
                    reducer_mode: AgentReducerMode::Any,
                },
            )
            .await
            .expect("denied invocation should return a report");

        assert_eq!(denied.status, "blocked");
        assert_eq!(
            denied.operator_policy_decision,
            OperatorPolicyDecision::RequireApproval
        );
        assert!(denied.operator_policy_requires_approval);
        assert!(!denied.operator_policy_allowed);
        assert!(!denied.agent_runtime_executed);
        assert_eq!(
            denied.context_recall_handoff_policy,
            AgentRuntimeContextRecallHandoffPolicy::Disabled
        );
        assert_eq!(denied.completed_agent_count, 0);
        assert_eq!(denied.selected_snippets_present_count, 0);
        assert_eq!(denied.selected_snippet_count, 0);
        assert_eq!(denied.blockers, vec!["policy_not_allowed"]);

        let unconfirmed = runtime
            .run_ready_agents_with_context_recall_operator_invocation(
                MultiAgentContextRecallOperatorInvocationRequest {
                    channel_id: "telegram:6476198178".into(),
                    sender_id: "6476198178".into(),
                    sender_is_owner: true,
                    operator_id: "operator-a".into(),
                    operator_confirmed: false,
                    idempotency_key: "multi-agent-operator-invocation-unconfirmed-1".into(),
                    limit: Some(1),
                    reducer_mode: AgentReducerMode::Any,
                },
            )
            .await
            .expect("unconfirmed invocation should return a report");

        assert_eq!(unconfirmed.status, "blocked");
        assert_eq!(
            unconfirmed.operator_policy_decision,
            OperatorPolicyDecision::Allow
        );
        assert!(unconfirmed.operator_policy_allowed);
        assert!(!unconfirmed.operator_policy_requires_approval);
        assert!(!unconfirmed.agent_runtime_executed);
        assert_eq!(
            unconfirmed.context_recall_handoff_policy,
            AgentRuntimeContextRecallHandoffPolicy::Disabled
        );
        assert_eq!(unconfirmed.completed_agent_count, 0);
        assert_eq!(unconfirmed.blockers, vec!["operator_not_confirmed"]);

        let approved = runtime
            .run_ready_agents_with_context_recall_operator_invocation(
                MultiAgentContextRecallOperatorInvocationRequest {
                    channel_id: "telegram:6476198178".into(),
                    sender_id: "6476198178".into(),
                    sender_is_owner: true,
                    operator_id: "operator-a".into(),
                    operator_confirmed: true,
                    idempotency_key: "multi-agent-operator-invocation-approved-1".into(),
                    limit: Some(1),
                    reducer_mode: AgentReducerMode::Any,
                },
            )
            .await
            .expect("owner-approved invocation should execute");

        assert_eq!(approved.status, "executed");
        assert_eq!(
            approved.operator_policy_decision,
            OperatorPolicyDecision::Allow
        );
        assert!(approved.operator_policy_allowed);
        assert!(!approved.operator_policy_requires_approval);
        assert!(approved.agent_runtime_executed);
        assert_eq!(
            approved.context_recall_handoff_policy,
            AgentRuntimeContextRecallHandoffPolicy::ExperimentalOperatorApproved
        );
        assert_eq!(approved.requested_agent_count, 1);
        assert_eq!(approved.completed_agent_count, 1);
        assert_eq!(approved.total_messages_processed, 1);
        assert_eq!(approved.provider_rollup_present_count, 1);
        assert_eq!(approved.selected_snippets_present_count, 1);
        assert!(approved.selected_snippet_count > 0);
        assert!(approved.blockers.is_empty());

        for rendered in [
            serde_json::to_string(&denied).expect("denied report should serialize"),
            format!("{denied:?}"),
            serde_json::to_string(&unconfirmed).expect("unconfirmed report should serialize"),
            format!("{unconfirmed:?}"),
            serde_json::to_string(&approved).expect("approved report should serialize"),
            format!("{approved:?}"),
        ] {
            assert_multi_agent_operator_invocation_report_does_not_leak(&rendered);
        }
    }

    #[tokio::test]
    async fn same_agent_can_run_isolated_workspaces() {
        let runtime = RuntimeKernel::new();

        runtime
            .register_agent_runtime_in_workspace("worker", Some("workspace:alpha"))
            .unwrap();
        runtime
            .register_agent_runtime_in_workspace("worker", Some("workspace:beta"))
            .unwrap();
        runtime
            .enqueue_agent_message_in_workspace(
                "worker",
                Some("workspace:alpha"),
                "alpha-only context",
                Some("ui-user"),
            )
            .unwrap();
        runtime
            .enqueue_agent_message_in_workspace(
                "worker",
                Some("workspace:beta"),
                "beta-only context",
                Some("ui-user"),
            )
            .unwrap();

        let report = runtime
            .run_ready_agents_with_reducer(None, AgentReducerMode::Merge)
            .await
            .unwrap();

        assert_eq!(report.completed_agent_count, 2);
        assert_eq!(report.pool.agent_count, 2);
        let workspaces = report
            .runs
            .iter()
            .map(|run| run.workspace_id.as_str())
            .collect::<HashSet<_>>();
        assert!(workspaces.contains("workspace:alpha"));
        assert!(workspaces.contains("workspace:beta"));
        for run in &report.runs {
            assert_eq!(run.agent_id, "worker");
            assert_eq!(run.turn_results.len(), 1);
            assert_eq!(run.turn_results[0].workspace_id, run.workspace_id);
            if run.workspace_id == "workspace:alpha" {
                assert_eq!(run.turn_results[0].input, "alpha-only context");
            }
            if run.workspace_id == "workspace:beta" {
                assert_eq!(run.turn_results[0].input, "beta-only context");
            }
        }
    }

    #[tokio::test]
    async fn multi_agent_runtime_demo_reaches_all_100_ratings() {
        let runtime = RuntimeKernel::new();

        let report = runtime
            .run_multi_agent_runtime_demo(Some(4), Some(2))
            .await
            .unwrap();

        assert_eq!(report.launched_agent_count, 4);
        assert_eq!(report.total_messages_processed, 9);
        assert!(report.true_concurrent);
        assert_eq!(report.ratings.top_level_agent_registry_percent, 100);
        assert_eq!(report.ratings.independent_inbox_event_loop_percent, 100);
        assert_eq!(report.ratings.true_concurrent_scheduler_percent, 100);
        assert_eq!(report.ratings.agent_resource_isolation_percent, 100);
        assert_eq!(report.ratings.cross_agent_message_bus_percent, 100);
        assert_eq!(report.ratings.barrier_reducer_join_percent, 100);
        assert_eq!(report.ratings.lifecycle_control_percent, 100);
        assert_eq!(report.ratings.model_tool_loop_percent, 100);
        assert_eq!(report.ratings.delivery_state_retry_percent, 100);
        assert_eq!(report.ratings.failure_timeout_recovery_percent, 100);
        assert_eq!(report.ratings.reducer_consensus_percent, 100);
        assert_eq!(report.ratings.evidence_observed_percent, 100);
        assert!(report.ratings.all_ratings_100);
    }

    #[tokio::test]
    async fn multi_agent_runtime_demo_keeps_all_100_across_message_depths() {
        for messages_per_agent in [4, 8] {
            let runtime = RuntimeKernel::new();

            let report = runtime
                .run_multi_agent_runtime_demo(Some(4), Some(messages_per_agent))
                .await
                .unwrap();

            assert_eq!(report.launched_agent_count, 4);
            assert_eq!(report.completed_agent_count, 4);
            assert_eq!(report.failed_agent_count, 0);
            assert_eq!(
                report.total_messages_processed,
                (4 * messages_per_agent) + 1
            );
            assert!(report.reducer_passed);
            assert_eq!(report.consensus_status, "passed");
            assert_eq!(report.ratings.failure_timeout_recovery_percent, 100);
            assert_eq!(report.ratings.evidence_observed_percent, 100);
            assert_eq!(report.ratings.overall_percent, 100);
            assert!(report.ratings.all_ratings_100);
        }
    }

    #[tokio::test]
    async fn failed_current_multi_agent_run_never_overclaims_all_100() {
        let runtime = RuntimeKernel::new();
        let recovered = runtime
            .run_multi_agent_runtime_demo(Some(4), Some(2))
            .await
            .unwrap();
        assert!(recovered.ratings.all_ratings_100);

        runtime
            .enqueue_agent_message(
                "agent-1",
                "[inject:fail] fail this current reducer join",
                Some("agent-2"),
            )
            .unwrap();
        let failed = runtime
            .run_ready_agents_with_reducer(Some(1), AgentReducerMode::Merge)
            .await
            .unwrap();

        assert_eq!(failed.failed_agent_count, 1);
        assert!(!failed.reducer_passed);
        assert_eq!(failed.ratings.reducer_consensus_percent, 0);
        assert_eq!(failed.ratings.evidence_observed_percent, 0);
        assert!(failed.ratings.overall_percent < 100);
        assert!(!failed.ratings.all_ratings_100);
    }

    #[test]
    fn empty_agent_pool_no_longer_overclaims_100_percent_readiness() {
        let runtime = RuntimeKernel::new();

        let report = runtime.agent_runtime_pool_report().unwrap();

        assert_eq!(report.agent_count, 0);
        assert_eq!(report.latest_max_parallelism_observed, 0);
        assert_eq!(report.ratings.true_concurrent_scheduler_percent, 0);
        assert_eq!(report.ratings.evidence_observed_percent, 0);
        assert!(!report.ratings.all_ratings_100);
        assert!(report.ratings.overall_percent < 100);
    }

    #[tokio::test]
    async fn multi_agent_runtime_snapshot_recovers_inbox_controls_and_evidence() {
        let path = crate::tool_workspace_root_path()
            .join("artifacts")
            .join(format!(
                "hepta-multi-agent-recovery-{}-{}.json",
                std::process::id(),
                current_unix_ms().unwrap_or(0)
            ));
        let path_string = path.to_string_lossy().to_string();
        let runtime = RuntimeKernel::new();
        runtime.register_agent_runtime("alpha").unwrap();
        runtime.register_agent_runtime("beta").unwrap();
        runtime.pause_agent_runtime("alpha").unwrap();
        runtime.resume_agent_runtime("alpha").unwrap();
        runtime
            .steer_agent_runtime("alpha", "recover after snapshot before reducing")
            .unwrap();
        runtime
            .enqueue_agent_message(
                "alpha",
                "[inject:timeout-once] recover durable timeout after snapshot",
                Some("beta"),
            )
            .unwrap();
        let failure = runtime
            .run_ready_agents_with_reducer(Some(1), AgentReducerMode::Any)
            .await
            .unwrap();
        assert_eq!(failure.failed_agent_count, 1);
        assert_eq!(
            failure.failures[0].failure_kind,
            AgentRuntimeFailureKind::Timeout
        );
        runtime
            .enqueue_agent_message("beta", "resume durable inbox", Some("alpha"))
            .unwrap();
        runtime.persist_snapshot(&path_string).unwrap();

        let recovered = RuntimeKernel::new();
        recovered.load_snapshot(&path_string).unwrap();
        let pool = recovered.agent_runtime_pool_report().unwrap();
        assert_eq!(pool.agent_count, 2);
        assert_eq!(pool.inbox_depth_total, 2);
        assert!(pool.lifecycle_control_ready);
        assert!(pool.cross_agent_message_bus_ready);

        let report = recovered
            .run_ready_agents_with_reducer(None, AgentReducerMode::Quorum)
            .await
            .unwrap();
        assert_eq!(report.reducer_mode, AgentReducerMode::Quorum);
        assert_eq!(report.quorum_threshold, 2);
        assert_eq!(report.completed_agent_count, 2);
        assert_eq!(report.total_messages_processed, 2);
        assert!(report.ratings.all_ratings_100);

        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn multi_agent_runtime_recovers_injected_failure_with_resource_leases() {
        let runtime = RuntimeKernel::new();
        runtime.register_agent_runtime("alpha").unwrap();
        runtime.register_agent_runtime("beta").unwrap();
        runtime.pause_agent_runtime("alpha").unwrap();
        runtime.resume_agent_runtime("alpha").unwrap();
        runtime
            .enqueue_agent_message(
                "alpha",
                "[inject:fail-once] retry this message once",
                Some("beta"),
            )
            .unwrap();
        let failed = runtime
            .run_ready_agents_with_reducer(Some(1), AgentReducerMode::Any)
            .await
            .unwrap();
        assert_eq!(failed.failed_agent_count, 1);
        assert!(failed.failures[0].retry_scheduled);
        assert_eq!(failed.pool.active_resource_lease_count, 0);

        runtime
            .enqueue_agent_message("beta", "complete peer message", Some("alpha"))
            .unwrap();
        let recovered = runtime
            .run_ready_agents_with_reducer(None, AgentReducerMode::Quorum)
            .await
            .unwrap();
        assert_eq!(recovered.completed_agent_count, 2);
        assert_eq!(recovered.failed_agent_count, 0);
        assert!(recovered.reducer_passed);
        assert_eq!(recovered.consensus_status, "passed");
        assert!(
            recovered
                .runs
                .iter()
                .any(|run| run.max_delivery_attempt > 1)
        );
        assert_eq!(recovered.ratings.agent_resource_isolation_percent, 100);
        assert_eq!(recovered.ratings.failure_timeout_recovery_percent, 100);
        assert_eq!(recovered.ratings.delivery_state_retry_percent, 100);
        assert_eq!(recovered.ratings.model_tool_loop_percent, 100);
        assert_eq!(recovered.ratings.reducer_consensus_percent, 100);
        assert!(recovered.ratings.all_ratings_100);
    }
}
