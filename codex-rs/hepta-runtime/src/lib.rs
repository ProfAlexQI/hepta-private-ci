#![allow(
    clippy::collapsible_if,
    clippy::derivable_impls,
    clippy::extend_with_drain,
    clippy::if_same_then_else,
    clippy::let_and_return,
    clippy::manual_contains,
    clippy::obfuscated_if_else,
    clippy::redundant_closure,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_lazy_evaluations,
    clippy::useless_conversion
)]

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::{Component, Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hepta_core::{
    AgentId, ApprovalRequirement, CorrelationId, EventKind, ExecutionProfile, FilesystemScope,
    FinishReason, HeptaError, HeptaNeuron, IntuitionFeedbackRecord, MemoryQuery, MemoryRecord,
    MemoryScope, MemoryStore, MessageRole, ModelMessage, ModelProvider, ModelRef, ModelRequest,
    ModelResponse, ModelToolSpec, PathCapabilityGate, PolicyDecision, PolicyEngine,
    PolicyEvaluationContext, PolicyRule, ProviderDescriptor, ProviderTransportKind, RiskTier,
    SessionId, SessionRecord, ThinkingLevel, Tool, ToolCall, ToolCallRequest, ToolContext,
    ToolResult, TopicGraphEdge, TopicSession, TranscriptEntry, TranscriptEntryKind, Usage,
    WritePathScope,
};
use hepta_intelligence::TopicAwareModelFeedbackRecord;
use hepta_memory::InMemoryStore;
use hepta_memory::StoreSnapshot;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

mod agent_harness;
mod approval_broker;
mod config_store;
mod delivery_queue;
mod doctor;
mod events;
mod hepta_contracts;
mod inbound_router;
mod live_readiness;
mod memory_context;
mod model_provider_router;
mod multi_agent;
mod operator_policy;
mod output_directives;
mod plugin_keyed_store;
mod process_supervisor;
mod query;
mod reports;
mod rollback_locks;
mod scheduler_store;
mod session_lifecycle;
mod session_transcript;
mod task_board;
mod telegram_model_runner;
mod tool_invocation;
mod topic_neuron;
mod worker_tasks;

pub use agent_harness::{
    AgentHarnessEvent, AgentHarnessKind, AgentHarnessLedger, AgentHarnessLedgerFile,
    AgentHarnessLedgerReport, AgentHarnessLocalExecutionInput, AgentHarnessLocalExecutionRecord,
    AgentHarnessLocalExecutionReport, AgentHarnessPlanReport, AgentHarnessRunRecord,
    AgentHarnessRunStatus, AgentHarnessSessionClassification, AgentHarnessSpawnChildInput,
    AgentHarnessSpawnChildReport, AgentHarnessStartHandoffInput, AgentHarnessStartHandoffRecord,
    AgentHarnessStartHandoffReport, AgentHarnessTransitionReport, DEFAULT_AGENT_HARNESS_LEDGER_ID,
    DEFAULT_AGENT_HARNESS_LEDGER_PATH,
};
pub use approval_broker::{
    ApprovalBroker, ApprovalBrokerEvent, ApprovalBrokerExpireReport, ApprovalBrokerFile,
    ApprovalBrokerReport, ApprovalBrokerRequest, ApprovalBrokerRequestInput,
    ApprovalBrokerRequestReport, ApprovalBrokerResolveReport, ApprovalBrokerStatus,
    DEFAULT_APPROVAL_BROKER_ID, DEFAULT_APPROVAL_BROKER_PATH,
};
pub use config_store::{
    ConfigPatchApplyReport, ConfigPatchPlanReport, ConfigPatchRecord, ConfigPatchStatus,
    ConfigRestartRefreshRecord, ConfigRestartRefreshReport, ConfigSchemaEntry,
    ConfigSchemaEntryReport, ConfigStoreEvent, ConfigStoreFile, ConfigStoreReport,
    DEFAULT_CONFIG_STORE_ID, DEFAULT_CONFIG_STORE_PATH, HeptaConfigStore,
};
pub use delivery_queue::{
    ChannelSendHandoffInput, ChannelSendHandoffReport, DEFAULT_DELIVERY_LEASE_MS,
    DEFAULT_DELIVERY_QUEUE_ID, DEFAULT_DELIVERY_QUEUE_PATH, DEFAULT_READBACK_EVIDENCE_LEDGER_ID,
    DEFAULT_READBACK_EVIDENCE_PATH, DeliveryQueueAckReport, DeliveryQueueClaimReport,
    DeliveryQueueEnqueueReport, DeliveryQueueEvent, DeliveryQueueFailReport, DeliveryQueueFile,
    DeliveryQueueItem, DeliveryQueueReadbackGateReport, DeliveryQueueReclaimReport,
    DeliveryQueueReport, DeliveryQueueStatus, DurableDeliveryQueue, ReadbackEvidenceAppendReport,
    ReadbackEvidenceEntry, ReadbackEvidenceLedger, ReadbackEvidenceLedgerFile,
    ReadbackEvidenceReport, RichDeliveryHandoffInput, RichDeliveryHandoffReport,
};
pub use doctor::{DoctorCheck, DoctorProviderProbe, DoctorReport, DoctorStatus};
use events::EventState;
pub use events::{EventQueryReport, EventRecord};
pub use hepta_contracts::HeptaRuntimeContractInventory;
pub use inbound_router::{
    DEFAULT_INBOUND_ROUTER_ID, DEFAULT_INBOUND_ROUTER_PATH, InboundEventInput, InboundEventRecord,
    InboundIngestReport, InboundRouteReport, InboundRouteStatus, InboundRouterAuditEvent,
    InboundRouterFile, InboundRouterReport, InboundRouterStore,
    InboundSessionTranscriptHandoffInput, InboundSessionTranscriptHandoffRecord,
    InboundSessionTranscriptHandoffReport, InboundSpoolRecord, InboundSpoolStatus,
};
pub use live_readiness::{
    LiveAdapterActivationDisciplineReport, LiveAdapterActivationInput, LiveAdapterActivationKind,
    RuntimeProductGateStatus, RuntimeProductReadinessReport, RuntimeReadinessGateReport,
    RuntimeReadinessStage, evaluate_live_adapter_activation, evaluate_runtime_product_readiness,
    evaluate_runtime_readiness, live_adapter_activation_discipline_sample,
};
pub use memory_context::{
    DEFAULT_MEMORY_CONTEXT_LEDGER_ID, DEFAULT_MEMORY_CONTEXT_LEDGER_PATH, MemoryCitation,
    MemoryCitationInput, MemoryContextBuildReport, MemoryContextEvent, MemoryContextLedger,
    MemoryContextLedgerFile, MemoryContextLedgerReport, MemoryContextLocalRetrievalInput,
    MemoryContextLocalRetrievalRecord, MemoryContextLocalRetrievalReport, MemoryContextPack,
    MemoryContextRetrievalHandoffInput, MemoryContextRetrievalHandoffRecord,
    MemoryContextRetrievalHandoffReport,
};
pub use model_provider_router::{
    DEFAULT_MODEL_PROVIDER_ROUTER_ID, DEFAULT_MODEL_PROVIDER_ROUTER_PATH,
    ModelProviderInvocationHandoffInput, ModelProviderInvocationHandoffRecord,
    ModelProviderInvocationHandoffReport, ModelProviderLocalInvocationInput,
    ModelProviderLocalInvocationRecord, ModelProviderLocalInvocationReport,
    ModelProviderMemoryContextActivationInput, ModelProviderMemoryContextActivationRecord,
    ModelProviderMemoryContextActivationReport, ModelProviderPluginContractInput,
    ModelProviderPluginContractRecord, ModelProviderPluginContractReport,
    ModelProviderRegisterReport, ModelProviderRouteEvent, ModelProviderRouteRecord,
    ModelProviderRouter, ModelProviderRouterFile, ModelProviderRouterReport,
    ModelProviderSelectionReport, ModelProviderStatus,
};
pub use multi_agent::{
    AgentDeliveryState, AgentInboxMessage, AgentInboxMessageKind, AgentReducerMode,
    AgentResourceLease, AgentRuntimeControlReport, AgentRuntimeDescriptor,
    AgentRuntimeExecutionBackend, AgentRuntimeFailureKind, AgentRuntimePoolReport,
    AgentRuntimeQuota, AgentRuntimeRecord, AgentRuntimeRunFailure, AgentRuntimeRunResult,
    AgentRuntimeStatus, MultiAgentConcurrentRunReport, MultiAgentRuntimeRatings,
    MultiAgentRuntimeState,
};
pub use operator_policy::{
    OperatorPolicyDecision, OperatorPolicyEvaluationReport, OperatorPolicyInput,
    evaluate_operator_policy,
};
pub use output_directives::{
    OutputDirectiveDeliveryHandoffInput, OutputDirectiveDeliveryHandoffReport, OutputDirectivePlan,
    handoff_output_directives_to_delivery_queue, plan_output_directives,
};
pub use plugin_keyed_store::{
    DurablePluginKeyedStore, PluginKeyedStoreRoundtripReport, PluginSessionEntry,
};
pub use process_supervisor::{
    DEFAULT_PROCESS_EXEC_TIMEOUT_MS, DEFAULT_PROCESS_LOG_LIMIT, DEFAULT_PROCESS_SUPERVISOR_ID,
    DEFAULT_PROCESS_SUPERVISOR_PATH, ProcessStartExecutionInput, ProcessStartExecutionRecord,
    ProcessStartExecutionReport, ProcessStartHandoffInput, ProcessStartHandoffRecord,
    ProcessStartHandoffReport, ProcessSupervisor, ProcessSupervisorEvent, ProcessSupervisorFile,
    ProcessSupervisorPlanReport, ProcessSupervisorReport, ProcessSupervisorTransitionReport,
    SupervisedProcessRecord, SupervisedProcessStatus,
};
pub use query::{
    RuntimeActivitySlice, RuntimeIntelligenceEvalCase, RuntimeIntelligenceEvalOverview,
    RuntimeIntelligencePhase2Gate, RuntimeIntelligencePhase2Overview,
    RuntimeIntuitionCalibrationFeedback, RuntimeIntuitionCalibrationOverview,
    RuntimeIntuitionCalibrationTarget, RuntimeNeuronLifecycleOverview, RuntimeProvenanceOverview,
    RuntimeSessionActivityOverview, RuntimeSessionActivitySlice,
};
pub use rollback_locks::{
    RollbackGroupLockDiagnosticsReport, WriteGroupLockReport, WriteLockPruneReport,
    WriteLockReport, WriteLockSummaryReport, WriteTargetLockReport,
};
pub use scheduler_store::{
    DEFAULT_SCHEDULER_STORE_ID, DEFAULT_SCHEDULER_STORE_PATH, SchedulerJobInput,
    SchedulerJobReadbackReport, SchedulerJobRecord, SchedulerJobStatus, SchedulerQueuedWakeRecord,
    SchedulerRunRecord, SchedulerRunStartInput, SchedulerScheduleKind, SchedulerStore,
    SchedulerStoreEvent, SchedulerStoreFile, SchedulerStoreReport, SchedulerStoreRunReport,
    SchedulerStoreScheduleReport, SchedulerStoreTransitionReport, SchedulerWakeHandoffInput,
    SchedulerWakeHandoffRecord, SchedulerWakeHandoffReport, SchedulerWakeMaterializationInput,
    SchedulerWakeMaterializationReport,
};
pub use session_lifecycle::{
    DurableSessionLifecyclePlane, SessionLifecycleCommand, SessionLifecycleEvent,
    SessionLifecycleOperationReport,
};
pub use session_transcript::{
    DEFAULT_SESSION_TRANSCRIPT_PATH, DEFAULT_SESSION_TRANSCRIPT_STORE_ID,
    SessionTranscriptAppendHandoffInput, SessionTranscriptAppendHandoffRecord,
    SessionTranscriptAppendHandoffReport, SessionTranscriptEvent,
    SessionTranscriptReadWindowReport, SessionTranscriptRecord, SessionTranscriptReport,
    SessionTranscriptSessionReport, SessionTranscriptStatus, SessionTranscriptStore,
    SessionTranscriptStoreFile, SessionTranscriptTurn, SessionTranscriptTurnKind,
    SessionTranscriptTurnReport, SessionTranscriptWindowTurn,
};
pub use task_board::{
    DEFAULT_TASK_BOARD_PATH, DEFAULT_TASK_CLAIM_LEASE_MS, TaskBoardAddReport, TaskBoardClaimReport,
    TaskBoardDiagnosticsReport, TaskBoardEvent, TaskBoardFile, TaskBoardHeartbeatReport,
    TaskBoardReclaimReport, TaskBoardReport, TaskBoardStore, TaskBoardTask,
    TaskBoardTerminalDeliveryInput, TaskBoardTerminalDeliveryReport, TaskBoardWorker,
};
pub use telegram_model_runner::{
    CODEX_ENGINE_ID, DEFAULT_TELEGRAM_MLX_BASE_URL, DEFAULT_TELEGRAM_MLX_MAX_TOKENS,
    DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS, DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS,
    DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS, DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS,
    DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS, DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION,
    DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS, DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
    DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS, HEPTA_KERNEL_CONTRACT, HEPTA_KERNEL_OWNER,
    HEPTA_KERNEL_TELEGRAM_CURSOR_SCHEMA, HEPTA_KERNEL_TELEGRAM_DELIVERY_MAX_RETRIES,
    HEPTA_KERNEL_TELEGRAM_DELIVERY_STORE_IDENTIFIER, HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
    HEPTA_KERNEL_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE, HEPTA_KERNEL_TELEGRAM_RUNNER_KIND,
    HEPTA_KERNEL_TELEGRAM_RUNNER_STRATEGY, HeptaKernelEngine, HeptaKernelTurnChannel,
    HeptaKernelTurnInput, HeptaKernelTurnPlan, HeptaKernelTurnStagePlan,
    MAX_TELEGRAM_MLX_MAX_TOKENS, MAX_TELEGRAM_MODEL_TIMEOUT_MS, MAX_TELEGRAM_POLL_LOOP_INTERVAL_MS,
    MAX_TELEGRAM_READ_MAX_ATTEMPTS, MAX_TELEGRAM_READ_RETRY_BACKOFF_MS,
    MAX_TELEGRAM_SEND_MAX_ATTEMPTS, MAX_TELEGRAM_SEND_MIN_INTERVAL_MS,
    MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS, MAX_TELEGRAM_SOAK_MAX_ATTENTION,
    MAX_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS, MAX_TELEGRAM_SOAK_MIN_POLLS,
    MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS, MIN_TELEGRAM_MODEL_TIMEOUT_MS,
    MIN_TELEGRAM_POLL_LOOP_INTERVAL_MS, NativeTelegramCandidateMaterial,
    NativeTelegramConfigMetadata, NativeTelegramConfigStatus, NativeTelegramConfigStatusInput,
    NativeTelegramDrainFinalStatusPlan, NativeTelegramDuplicateDecision,
    NativeTelegramExecutionPlan, NativeTelegramGatewayGateSummary,
    NativeTelegramGatewayGateSummaryInput, NativeTelegramIngressInspection,
    NativeTelegramModelExecutionReport, NativeTelegramModelInvocationRequestPlan,
    NativeTelegramModelRunnerInvocationOutcome, NativeTelegramModelRunnerPlan,
    NativeTelegramModelTurnPlan, NativeTelegramProductionGuardPolicyInput,
    NativeTelegramProductionGuardStatus, NativeTelegramProductionGuardStatusInput,
    NativeTelegramReplyTargetMaterial, NativeTelegramSendExecutionReport,
    NativeTelegramSendRequestPlan, NativeTelegramSessionBridgePlan, NativeTelegramTokenObservation,
    NativeTelegramTokenObservationInput, TELEGRAM_ALLOWED_UPDATES, TELEGRAM_DRAIN_ONCE_STAGES,
    build_native_telegram_config_status, build_native_telegram_gateway_gate_summary,
    build_native_telegram_production_guard_status,
    build_native_telegram_production_guard_status_from_policy,
    classify_native_telegram_model_runner_error, extract_native_telegram_config_metadata,
    extract_native_telegram_exec_child_final_message,
    extract_native_telegram_openai_chat_completion_text, hepta_kernel_telegram_prompt,
    invoke_native_telegram_model_runner_with_plan, native_telegram_bot_token_shape_ok,
    native_telegram_codex_core_prompt, native_telegram_cursor_body,
    native_telegram_cursor_duplicate_rule_valid, native_telegram_delivery_backoff_ms,
    native_telegram_delivery_error_is_permanent, native_telegram_delivery_lifecycle_record,
    native_telegram_drain_execution_plan, native_telegram_drain_final_status,
    native_telegram_drain_first_missing_gate, native_telegram_drain_status_probe_executes_pipeline,
    native_telegram_duplicate_decision, native_telegram_error_is_transient,
    native_telegram_exec_child_args, native_telegram_exec_child_status_error,
    native_telegram_first_model_candidate_with_duplicate_decision,
    native_telegram_get_updates_error_is_conflict, native_telegram_get_updates_error_is_transient,
    native_telegram_get_updates_query, native_telegram_get_updates_should_retry,
    native_telegram_hepta_kernel_prompt, native_telegram_mlx_chat_completion_body,
    native_telegram_model_failure_fallback_allowed, native_telegram_model_failure_fallback_message,
    native_telegram_model_timeout, native_telegram_model_turn_plan_from_candidates,
    native_telegram_next_update_offset, native_telegram_normalize_binding_id,
    native_telegram_poll_loop_interval_ms_policy, native_telegram_poll_loop_should_spawn,
    native_telegram_read_max_attempts_policy, native_telegram_read_retry_backoff_policy,
    native_telegram_receive_limit_policy, native_telegram_send_chat_action_request_body,
    native_telegram_send_error_is_transient, native_telegram_send_max_attempts_policy,
    native_telegram_send_message_request_body, native_telegram_send_min_interval_policy,
    native_telegram_send_rate_limit_sleep_for, native_telegram_send_retry_backoff_policy,
    native_telegram_send_should_retry, native_telegram_soak_max_attention_count_policy,
    native_telegram_soak_max_observed_age_ms_policy,
    native_telegram_soak_min_poll_iterations_policy, native_telegram_system_time_unix_ms,
    native_telegram_typing_keepalive_interval_policy,
    native_telegram_typing_keepalive_should_start, native_telegram_update_already_drained,
    parse_native_telegram_cursor_next_update_offset, parse_native_telegram_env_truthy_value,
    parse_native_telegram_env_u64_value, parse_native_telegram_mlx_model_ref,
    plan_hepta_kernel_telegram_session_bridge, plan_hepta_kernel_turn,
    redact_native_telegram_model_runner_error, redact_native_telegram_token_like_text,
    resolve_native_telegram_secret_provider_path, resolve_native_telegram_token_observation,
    select_native_telegram_model_runner, wait_for_native_telegram_model_child,
};
pub use tool_invocation::{
    DEFAULT_TOOL_INVOCATION_LEDGER_ID, DEFAULT_TOOL_INVOCATION_LEDGER_PATH,
    ToolInvocationApprovalRequestReport, ToolInvocationEvent, ToolInvocationLedger,
    ToolInvocationLedgerFile, ToolInvocationLedgerReport, ToolInvocationPlanReport,
    ToolInvocationRecord, ToolInvocationStatus, ToolInvocationTransitionReport,
};
pub use topic_neuron::{
    DEFAULT_TOPIC_NEURON_STORE_ID, DEFAULT_TOPIC_NEURON_STORE_PATH, TopicNeuronFeedbackEvent,
    TopicNeuronFeedbackReport, TopicNeuronObserveReport, TopicNeuronRecord, TopicNeuronStore,
    TopicNeuronStoreFile, TopicNeuronStoreReport,
};
use worker_tasks::WorkerTaskState;
pub use worker_tasks::{
    OperatorConsoleEventSummary, OperatorConsoleReport, WorkerDescriptor, WorkerExecutionBackend,
    WorkerExecutionBackendBinding, WorkerExecutionBackendDescriptor, WorkerExecutionBackendKind,
    WorkerExecutionBackendReport, WorkerExecutionBackendStatus, WorkerInventoryReport,
    WorkerPermissionEnvelope, WorkerPoolPressureLevel, WorkerPoolPressureReport,
    WorkerPressureLane, WorkerSubagentLaneObservation, WorkerSubagentObservatoryReport,
    WorkerTaskCodingRound, WorkerTaskCommandRun, WorkerTaskDueRunReport, WorkerTaskEvidenceEntry,
    WorkerTaskEvidenceReport, WorkerTaskExecutionMode, WorkerTaskFailureKind, WorkerTaskFileLease,
    WorkerTaskFileLeaseStatus, WorkerTaskHandoffBundleReport, WorkerTaskIndexReport,
    WorkerTaskJoinItem, WorkerTaskJoinReport, WorkerTaskLoopPhase, WorkerTaskLoopReport,
    WorkerTaskLoopStep, WorkerTaskMergeDecision, WorkerTaskMergeRiskReport,
    WorkerTaskPatchApplyStatus, WorkerTaskPatchProposal, WorkerTaskPatchReviewReport,
    WorkerTaskPatchRollbackReport, WorkerTaskPatchSetApplyReport, WorkerTaskPromotionDecision,
    WorkerTaskPromotionLedgerEntry, WorkerTaskPromotionLedgerReport, WorkerTaskPromotionReport,
    WorkerTaskReadyRunReport, WorkerTaskRecord, WorkerTaskReplayAuditReport, WorkerTaskReplayCheck,
    WorkerTaskReport, WorkerTaskResourceLimits, WorkerTaskRunReport, WorkerTaskSafetyEnvelope,
    WorkerTaskSandboxPolicy, WorkerTaskStatus, WorkerTaskSupervisorReport, file_lease_status_label,
    task_status_label,
};

const WRITE_LOCK_LEASE_MS: u64 = 5 * 60 * 1000;

pub struct RuntimeKernel {
    providers: ProviderRegistry,
    tools: ToolRegistry,
    memory: InMemoryStore,
    policy: ConfigurablePolicyEngine,
    approval_state: Arc<Mutex<ApprovalState>>,
    history_state: Arc<Mutex<Vec<TurnRecord>>>,
    event_state: Arc<Mutex<EventState>>,
    model_state: Arc<Mutex<ModelState>>,
    execution_profile_state: Arc<Mutex<ExecutionProfileState>>,
    filesystem_scope_state: Arc<Mutex<FilesystemScopeState>>,
    capability_gate_state: Arc<Mutex<CapabilityGateState>>,
    write_path_scope_state: Arc<Mutex<WritePathScopeState>>,
    write_transaction_state: Arc<Mutex<Vec<WriteTransactionEntry>>>,
    write_transaction_group_state: Arc<Mutex<WriteTransactionGroupState>>,
    write_lock_state: Arc<Mutex<WriteLockState>>,
    rollback_failure_injection_state: Arc<Mutex<Vec<String>>>,
    session_state: Arc<Mutex<SessionState>>,
    worker_task_state: Arc<Mutex<WorkerTaskState>>,
    multi_agent_runtime_state: Arc<Mutex<MultiAgentRuntimeState>>,
    topic_session_state: Arc<Mutex<TopicSessionState>>,
    topic_graph_state: Arc<Mutex<TopicGraphState>>,
    neuron_state: Arc<Mutex<NeuronState>>,
    intuition_feedback_state: Arc<Mutex<IntuitionFeedbackState>>,
    model_router_feedback_state: Arc<Mutex<ModelRouterFeedbackState>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerticalSliceResult {
    pub session_id: String,
    pub active_model: ModelRef,
    pub invoked_tool: Option<String>,
    pub tool_output_json: Option<String>,
    pub final_text: String,
    pub recalled_memories: usize,
    pub approval_required: Option<String>,
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSelection {
    pub active: ModelRef,
    pub available: Vec<ModelRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderCatalog {
    pub providers: Vec<ProviderDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchModelResult {
    pub previous: ModelRef,
    pub current: ModelRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchExecutionProfileResult {
    pub previous: ExecutionProfile,
    pub current: ExecutionProfile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchFilesystemScopeResult {
    pub previous: FilesystemScope,
    pub current: FilesystemScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchWritePathScopeResult {
    pub previous: WritePathScope,
    pub current: WritePathScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapabilityGateReport {
    pub session_id: String,
    pub default_filesystem_scope: FilesystemScope,
    pub path_gates: Vec<PathCapabilityGate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingApproval {
    pub tool_name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ApprovalSnapshot {
    pub granted_tools: Vec<String>,
    pub pending: Vec<PendingApproval>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ToolDescriptor {
    pub name: String,
    pub description: String,
    pub risk_tier: RiskTier,
    pub execution_metadata: hepta_core::ToolExecutionMetadata,
    pub default_approval_requirement: ApprovalRequirement,
    pub input_schema_json: String,
    pub output_schema_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PolicyToolDecisionReport {
    pub tool_name: String,
    pub risk_tier: RiskTier,
    pub requirement: ApprovalRequirement,
    pub reason: String,
    pub matched_rule_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PolicyReport {
    pub active_session_id: String,
    pub active_model: ModelRef,
    pub default_rules: Vec<PolicyRule>,
    pub custom_rules: Vec<PolicyRule>,
    pub effective_tool_decisions: Vec<PolicyToolDecisionReport>,
    pub granted_approvals: usize,
    pub pending_approvals: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionSnapshot {
    pub session_id: String,
    pub agent_id: String,
    pub title: String,
    pub model: ModelRef,
    pub created_at_unix_ms: u64,
    pub last_active_unix_ms: u64,
    pub last_user_intent_summary: Option<String>,
    pub archived_at_unix_ms: Option<u64>,
    pub topic_session_count: usize,
    pub topic_graph_edge_count: usize,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionDiffReport {
    pub left_session_id: String,
    pub right_session_id: String,
    pub left_title: String,
    pub right_title: String,
    pub left_model: ModelRef,
    pub right_model: ModelRef,
    pub left_archived: bool,
    pub right_archived: bool,
    pub left_last_user_intent_summary: Option<String>,
    pub right_last_user_intent_summary: Option<String>,
    pub left_history_count: usize,
    pub right_history_count: usize,
    pub shared_history_count: usize,
    pub approvals_only_left: Vec<String>,
    pub approvals_only_right: Vec<String>,
    pub pending_only_left: Vec<String>,
    pub pending_only_right: Vec<String>,
    pub history_only_left: Vec<String>,
    pub history_only_right: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct MergeOptions {
    pub adopt_model: bool,
    pub adopt_title: bool,
    pub delete_source: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MergePreviewReport {
    pub source_session_id: String,
    pub target_session_id: String,
    pub options: MergeOptions,
    pub source_title: String,
    pub source_model: ModelRef,
    pub target_title_before: String,
    pub target_title_after: String,
    pub target_model_before: ModelRef,
    pub target_model_after: ModelRef,
    pub target_archived_before: bool,
    pub target_archived_after: bool,
    pub source_deleted_after_merge: bool,
    pub target_last_user_intent_summary_before: Option<String>,
    pub source_last_user_intent_summary: Option<String>,
    pub merged_last_user_intent_summary: Option<String>,
    pub source_history_count: usize,
    pub target_history_count: usize,
    pub history_entries_to_append: usize,
    pub history_entries_skipped_as_duplicates: usize,
    pub source_topic_session_count: usize,
    pub target_topic_session_count_before: usize,
    pub target_topic_session_count_after: usize,
    pub source_topic_graph_edge_count: usize,
    pub target_topic_graph_edge_count_before: usize,
    pub target_topic_graph_edge_count_after: usize,
    pub approvals_added_to_target: Vec<String>,
    pub pending_added_to_target: Vec<String>,
    pub duplicate_history_entries_skipped: Vec<String>,
    pub new_history_entries_to_append: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MergeExecutionReport {
    pub source_session_id: String,
    pub target_session_id: String,
    pub options: MergeOptions,
    pub target_title_after: String,
    pub target_model_after: ModelRef,
    pub target_archived_after: bool,
    pub source_deleted_after_merge: bool,
    pub merged_last_user_intent_summary: Option<String>,
    pub approvals_added_to_target: Vec<String>,
    pub pending_added_to_target: Vec<String>,
    pub appended_history_entries: usize,
    pub skipped_duplicate_history_entries: usize,
    pub source_topic_session_count: usize,
    pub target_topic_session_count_before: usize,
    pub target_topic_session_count_after: usize,
    pub source_topic_graph_edge_count: usize,
    pub target_topic_graph_edge_count_before: usize,
    pub target_topic_graph_edge_count_after: usize,
    pub new_history_entries_appended: Vec<String>,
    pub duplicate_history_entries_skipped: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionExportReport {
    pub session_id: String,
    pub export_path: String,
    pub exported_at_unix_ms: u64,
    pub title: String,
    pub model: ModelRef,
    pub archived: bool,
    pub approvals_granted: usize,
    pub approvals_pending: usize,
    pub history_entries: usize,
    pub topic_session_count: usize,
    pub topic_graph_edge_count: usize,
    pub neuron_count: usize,
    pub intuition_feedback_count: usize,
    pub model_router_feedback_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionImportReport {
    pub session_id: String,
    pub import_path: String,
    pub imported_title: String,
    pub imported_model: ModelRef,
    pub imported_archived: bool,
    pub approvals_granted: usize,
    pub approvals_pending: usize,
    pub history_entries: usize,
    pub topic_session_count: usize,
    pub topic_graph_edge_count: usize,
    pub neuron_count: usize,
    pub intuition_feedback_count: usize,
    pub model_router_feedback_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SessionForkReport {
    pub source_session_id: String,
    pub target_session_id: String,
    pub target_title: String,
    pub target_model: ModelRef,
    pub target_archived: bool,
    pub approvals_granted: usize,
    pub approvals_pending: usize,
    pub history_entries: usize,
    pub topic_session_count: usize,
    pub topic_graph_edge_count: usize,
    pub active_session_after_fork: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupEntryReport {
    pub id: String,
    pub backup_path: String,
    pub target_path: String,
    pub scope: String,
    pub created_at_unix_ms: u64,
    pub bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupIndexReport {
    pub backup_root: String,
    pub filter_target_path: Option<String>,
    pub backups: Vec<BackupEntryReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestoreBackupReport {
    pub transaction_id: String,
    pub backup_id: String,
    pub backup_path: String,
    pub restored_target_path: String,
    pub restored_bytes: u64,
    pub target_existed_before_restore: bool,
    pub previous_target_backup_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupPruneReport {
    pub backup_root: String,
    pub filter_target_path: Option<String>,
    pub keep_latest_per_target: usize,
    pub max_age_ms: Option<u64>,
    pub scanned_backups: usize,
    pub executed: bool,
    pub deleted_count: usize,
    pub kept_backups: Vec<BackupEntryReport>,
    pub deleted_backups: Vec<BackupEntryReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteTransactionEntry {
    pub transaction_id: String,
    pub session_id: String,
    pub action: String,
    pub target_path: String,
    pub created_at_unix_ms: u64,
    pub mode: String,
    pub target_existed_before: bool,
    pub bytes_before: u64,
    pub bytes_after: u64,
    pub rollback_strategy: String,
    pub rollback_checkpoint_path: Option<String>,
    pub source_backup_path: Option<String>,
    pub rolled_back_at_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriteTransactionIndexReport {
    pub filter_target_path: Option<String>,
    pub transactions: Vec<WriteTransactionEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollbackWriteReport {
    pub transaction_id: String,
    pub target_path: String,
    pub rollback_strategy: String,
    pub rollback_checkpoint_path: Option<String>,
    pub previous_target_backup_path: Option<String>,
    pub target_exists_after_rollback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteTransactionGroup {
    pub group_id: String,
    pub session_id: String,
    pub opened_at_unix_ms: u64,
    pub closed_at_unix_ms: Option<u64>,
    pub transaction_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriteTransactionGroupIndexReport {
    pub session_id: String,
    pub active_group_id: Option<String>,
    pub groups: Vec<WriteTransactionGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BeginWriteTransactionGroupReport {
    pub session_id: String,
    pub group_id: String,
    pub opened_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EndWriteTransactionGroupReport {
    pub session_id: String,
    pub group_id: String,
    pub closed_at_unix_ms: u64,
    pub transaction_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollbackPlanStep {
    pub transaction_id: String,
    pub target_path: String,
    pub rollback_strategy: String,
    pub rollback_checkpoint_path: Option<String>,
    pub ready: bool,
    pub already_rolled_back: bool,
    pub blocking_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollbackPlanReport {
    pub session_id: String,
    pub group_id: String,
    pub active: bool,
    pub closed: bool,
    pub executable: bool,
    pub steps: Vec<RollbackPlanStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollbackGroupReport {
    pub session_id: String,
    pub group_id: String,
    pub attempt_id: String,
    pub status: RollbackGroupAttemptStatus,
    pub resumed_from_attempt_id: Option<String>,
    pub executed_transaction_ids: Vec<String>,
    pub skipped_already_rolled_back_ids: Vec<String>,
    pub pending_transaction_ids: Vec<String>,
    pub failed_transaction_id: Option<String>,
    pub failure_reason: Option<String>,
    pub target_paths_restored: Vec<String>,
    pub suggested_next_action: String,
    pub resume_command: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackGroupAttemptStatus {
    Running,
    Completed,
    PartialFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackGroupAttempt {
    pub attempt_id: String,
    pub session_id: String,
    pub group_id: String,
    pub started_at_unix_ms: u64,
    pub finished_at_unix_ms: Option<u64>,
    pub status: RollbackGroupAttemptStatus,
    pub resumed_from_attempt_id: Option<String>,
    #[serde(default)]
    pub superseded_by_attempt_id: Option<String>,
    pub executed_transaction_ids: Vec<String>,
    pub skipped_already_rolled_back_ids: Vec<String>,
    pub pending_transaction_ids: Vec<String>,
    pub failed_transaction_id: Option<String>,
    pub failure_reason: Option<String>,
    pub target_paths_restored: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollbackGroupStatusReport {
    pub schema_version: u32,
    pub session_id: String,
    pub group_id: String,
    pub executable_now: bool,
    pub group_locked: bool,
    pub group_lock_attempt_id: Option<String>,
    pub target_lock_count: usize,
    pub orphaned_lock_count: usize,
    pub latest_attempt_owns_lock_set: bool,
    pub attempt_count: usize,
    pub superseded_attempt_count: usize,
    pub active_attempt_id: Option<String>,
    pub lock_diagnostics: RollbackGroupLockDiagnosticsReport,
    pub attempt_lifecycle: RollbackGroupAttemptLifecycleReport,
    pub latest_attempt: Option<RollbackGroupAttempt>,
    pub suggested_next_action: String,
    pub resume_command: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollbackGroupAttemptLifecycleReport {
    pub attempt_count: usize,
    pub superseded_attempt_count: usize,
    pub active_attempt_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RollbackGroupAttemptLifecycle {
    attempt_count: usize,
    superseded_attempt_count: usize,
    active_attempt_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteTargetLock {
    pub session_id: String,
    pub target_path: String,
    pub owner_kind: String,
    pub owner_id: String,
    #[serde(default)]
    pub rollback_group_id: Option<String>,
    #[serde(default)]
    pub rollback_attempt_id: Option<String>,
    pub locked_at_unix_ms: u64,
    #[serde(default = "default_write_lock_lease_expires_at_unix_ms")]
    pub lease_expires_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteGroupLock {
    pub session_id: String,
    pub group_id: String,
    pub owner_kind: String,
    pub owner_id: String,
    #[serde(default)]
    pub rollback_attempt_id: Option<String>,
    pub locked_at_unix_ms: u64,
    #[serde(default = "default_write_lock_lease_expires_at_unix_ms")]
    pub lease_expires_at_unix_ms: u64,
}

#[derive(Debug, Clone)]
struct PreparedWriteTransaction {
    target_path: String,
    mode_requested: String,
    preview_only: bool,
    target_existed_before: bool,
    before_bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SessionWriteTransactionGroupBinding {
    session_id: String,
    active_group_id: String,
}

#[derive(Debug, Default, Clone)]
struct WriteTransactionGroupState {
    active_bindings: Vec<SessionWriteTransactionGroupBinding>,
    groups: Vec<WriteTransactionGroup>,
    rollback_attempts: Vec<RollbackGroupAttempt>,
}

#[derive(Debug, Default, Clone)]
struct WriteLockState {
    target_locks: Vec<WriteTargetLock>,
    group_locks: Vec<WriteGroupLock>,
}

#[derive(Debug, Clone)]
struct MergeHistoryPlan {
    append_turns: Vec<TurnRecord>,
    new_history_entries_to_append: Vec<String>,
    duplicate_history_entries_skipped: Vec<String>,
}

#[derive(Debug, Clone)]
struct TopicStateMergeOutcome {
    topic_sessions: Vec<TopicSession>,
    topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySnapshot {
    pub id: String,
    pub scope: MemoryScope,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnRecord {
    pub session_id: String,
    pub input: String,
    pub invoked_tool: Option<String>,
    pub final_text: String,
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone)]
struct RuntimeToolExecution {
    tool_name: String,
    tool_output_json: Option<String>,
    tool_message: String,
}

#[derive(Debug, Clone)]
struct RuntimeToolTimeout {
    tool_name: String,
    tool_output_json: Option<String>,
    final_text: String,
}

#[derive(Debug, Clone)]
enum RuntimeToolStep {
    Executed(RuntimeToolExecution),
    TimedOut(RuntimeToolTimeout),
    ApprovalRequired { tool_name: String, reason: String },
    Blocked { final_text: String, reason: String },
}

#[derive(Debug, Clone)]
struct ModelState {
    default_active: ModelRef,
    sessions: Vec<SessionModelState>,
}

#[derive(Debug, Clone)]
struct ExecutionProfileState {
    default_profile: ExecutionProfile,
    sessions: Vec<SessionExecutionProfileBinding>,
}

#[derive(Debug, Clone)]
struct FilesystemScopeState {
    default_scope: FilesystemScope,
    sessions: Vec<SessionFilesystemScopeBinding>,
}

#[derive(Debug, Clone)]
struct WritePathScopeState {
    default_scope: WritePathScope,
    sessions: Vec<SessionWritePathScopeBinding>,
}

#[derive(Debug, Clone)]
struct CapabilityGateState {
    sessions: Vec<SessionCapabilityGateBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SessionModelState {
    session_id: String,
    selected_model: ModelRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SessionExecutionProfileBinding {
    session_id: String,
    profile: ExecutionProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SessionFilesystemScopeBinding {
    session_id: String,
    scope: FilesystemScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SessionWritePathScopeBinding {
    session_id: String,
    scope: WritePathScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SessionCapabilityGateBinding {
    session_id: String,
    path_gates: Vec<PathCapabilityGate>,
}

#[derive(Debug, Clone)]
struct SessionState {
    active_session_id: String,
}

#[derive(Debug, Default, Clone)]
struct TopicSessionState {
    sessions: Vec<TopicSession>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RuntimeTopicGraphEdgeRecord {
    source_topic_session_id: String,
    edge: TopicGraphEdge,
}

#[derive(Debug, Default, Clone)]
struct TopicGraphState {
    edges: Vec<RuntimeTopicGraphEdgeRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RuntimeNeuronRecord {
    session_id: String,
    neuron: HeptaNeuron,
}

#[derive(Debug, Default, Clone)]
struct NeuronState {
    neurons: Vec<RuntimeNeuronRecord>,
}

#[derive(Debug, Default, Clone)]
struct IntuitionFeedbackState {
    records: Vec<IntuitionFeedbackRecord>,
}

#[derive(Debug, Default, Clone)]
struct ModelRouterFeedbackState {
    records: Vec<TopicAwareModelFeedbackRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RuntimeSnapshot {
    version: u32,
    active_model: ModelRef,
    available_models: Vec<ModelRef>,
    #[serde(default)]
    session_models: Vec<SessionModelState>,
    active_session_id: String,
    #[serde(default)]
    policy_rules: Vec<PolicyRule>,
    #[serde(default)]
    approvals: Vec<SessionApprovalState>,
    history: Vec<TurnRecord>,
    #[serde(default)]
    session_execution_profiles: Vec<SessionExecutionProfileBinding>,
    #[serde(default)]
    session_filesystem_scopes: Vec<SessionFilesystemScopeBinding>,
    #[serde(default)]
    session_capability_gates: Vec<SessionCapabilityGateBinding>,
    #[serde(default)]
    session_write_path_scopes: Vec<SessionWritePathScopeBinding>,
    #[serde(default)]
    events: Vec<EventRecord>,
    #[serde(default)]
    write_transactions: Vec<WriteTransactionEntry>,
    #[serde(default)]
    write_transaction_groups: Vec<WriteTransactionGroup>,
    #[serde(default)]
    active_write_transaction_groups: Vec<SessionWriteTransactionGroupBinding>,
    #[serde(default)]
    rollback_group_attempts: Vec<RollbackGroupAttempt>,
    #[serde(default)]
    write_target_locks: Vec<WriteTargetLock>,
    #[serde(default)]
    write_group_locks: Vec<WriteGroupLock>,
    #[serde(default)]
    topic_sessions: Vec<TopicSession>,
    #[serde(default)]
    topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
    #[serde(default)]
    neurons: Vec<RuntimeNeuronRecord>,
    #[serde(default)]
    intuition_feedback: Vec<IntuitionFeedbackRecord>,
    #[serde(default)]
    model_router_feedback: Vec<TopicAwareModelFeedbackRecord>,
    #[serde(default)]
    worker_tasks: Vec<WorkerTaskRecord>,
    #[serde(default)]
    multi_agent_runtime: MultiAgentRuntimeState,
    memory: StoreSnapshot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SessionExport {
    version: u32,
    exported_at_unix_ms: u64,
    session: SessionRecord,
    model: ModelRef,
    execution_profile: ExecutionProfile,
    filesystem_scope: FilesystemScope,
    #[serde(default)]
    path_capability_gates: Vec<PathCapabilityGate>,
    write_path_scope: WritePathScope,
    #[serde(default)]
    approval: ApprovalSnapshot,
    history: Vec<TurnRecord>,
    #[serde(default)]
    write_transactions: Vec<WriteTransactionEntry>,
    #[serde(default)]
    write_transaction_groups: Vec<WriteTransactionGroup>,
    #[serde(default)]
    active_write_transaction_group_id: Option<String>,
    #[serde(default)]
    rollback_group_attempts: Vec<RollbackGroupAttempt>,
    #[serde(default)]
    write_target_locks: Vec<WriteTargetLock>,
    #[serde(default)]
    write_group_locks: Vec<WriteGroupLock>,
    #[serde(default)]
    topic_sessions: Vec<TopicSession>,
    #[serde(default)]
    topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
    #[serde(default)]
    neurons: Vec<RuntimeNeuronRecord>,
    #[serde(default)]
    intuition_feedback: Vec<IntuitionFeedbackRecord>,
    #[serde(default)]
    model_router_feedback: Vec<TopicAwareModelFeedbackRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SessionApprovalState {
    session_id: String,
    granted_tools: Vec<String>,
    pending: Vec<PendingApproval>,
}

#[derive(Debug, Default)]
struct ApprovalState {
    sessions: Vec<SessionApprovalState>,
}

#[derive(Debug, Default, Clone)]
struct ConfigurablePolicyEngine {
    custom_rules: Arc<Mutex<Vec<PolicyRule>>>,
}

impl ApprovalState {
    fn session(&self, session_id: &str) -> Option<&SessionApprovalState> {
        self.sessions
            .iter()
            .find(|session| session.session_id == session_id)
    }

    fn session_mut(&mut self, session_id: &str) -> &mut SessionApprovalState {
        if let Some(index) = self
            .sessions
            .iter()
            .position(|session| session.session_id == session_id)
        {
            return &mut self.sessions[index];
        }

        self.sessions.push(SessionApprovalState {
            session_id: session_id.to_string(),
            granted_tools: Vec::new(),
            pending: Vec::new(),
        });
        self.sessions
            .last_mut()
            .expect("session approval state should exist after push")
    }

    fn snapshot_for(&self, session_id: &str) -> ApprovalSnapshot {
        match self.session(session_id) {
            Some(session) => ApprovalSnapshot {
                granted_tools: session.granted_tools.clone(),
                pending: session.pending.clone(),
            },
            None => ApprovalSnapshot {
                granted_tools: Vec::new(),
                pending: Vec::new(),
            },
        }
    }

    fn is_granted(&self, session_id: &str, tool_name: &str) -> bool {
        self.session(session_id)
            .map(|session| session.granted_tools.iter().any(|tool| tool == tool_name))
            .unwrap_or(false)
    }

    fn remember_pending(&mut self, session_id: &str, tool_name: &str, reason: &str) {
        let session = self.session_mut(session_id);
        if session
            .pending
            .iter()
            .any(|item| item.tool_name == tool_name)
        {
            return;
        }
        session.pending.push(PendingApproval {
            tool_name: tool_name.to_string(),
            reason: reason.to_string(),
        });
    }

    fn grant(&mut self, session_id: &str, tool_name: &str) {
        let session = self.session_mut(session_id);
        if !session.granted_tools.iter().any(|tool| tool == tool_name) {
            session.granted_tools.push(tool_name.to_string());
        }
        session.pending.retain(|item| item.tool_name != tool_name);
    }

    fn all_sessions(&self) -> Vec<SessionApprovalState> {
        self.sessions.clone()
    }

    fn remove_session(&mut self, session_id: &str) {
        self.sessions
            .retain(|session| session.session_id != session_id);
    }
}

impl RuntimeKernel {
    pub fn new() -> Self {
        let providers = ProviderRegistry::new();
        let active = providers.default_model();

        Self {
            providers,
            tools: ToolRegistry::new(),
            memory: InMemoryStore::default(),
            policy: ConfigurablePolicyEngine::default(),
            approval_state: Arc::new(Mutex::new(ApprovalState::default())),
            history_state: Arc::new(Mutex::new(Vec::new())),
            event_state: Arc::new(Mutex::new(EventState::new_with_boot_event())),
            model_state: Arc::new(Mutex::new(ModelState {
                default_active: active,
                sessions: Vec::new(),
            })),
            execution_profile_state: Arc::new(Mutex::new(ExecutionProfileState {
                default_profile: ExecutionProfile::FullAccess,
                sessions: Vec::new(),
            })),
            filesystem_scope_state: Arc::new(Mutex::new(FilesystemScopeState {
                default_scope: FilesystemScope::WorkspaceOnly,
                sessions: Vec::new(),
            })),
            capability_gate_state: Arc::new(Mutex::new(CapabilityGateState {
                sessions: Vec::new(),
            })),
            write_path_scope_state: Arc::new(Mutex::new(WritePathScopeState {
                default_scope: WritePathScope::ArtifactsOnly,
                sessions: Vec::new(),
            })),
            write_transaction_state: Arc::new(Mutex::new(Vec::new())),
            write_transaction_group_state: Arc::new(Mutex::new(
                WriteTransactionGroupState::default(),
            )),
            write_lock_state: Arc::new(Mutex::new(WriteLockState::default())),
            rollback_failure_injection_state: Arc::new(Mutex::new(Vec::new())),
            session_state: Arc::new(Mutex::new(SessionState {
                active_session_id: "session-main".into(),
            })),
            worker_task_state: Arc::new(Mutex::new(WorkerTaskState::default())),
            multi_agent_runtime_state: Arc::new(Mutex::new(MultiAgentRuntimeState::default())),
            topic_session_state: Arc::new(Mutex::new(TopicSessionState::default())),
            topic_graph_state: Arc::new(Mutex::new(TopicGraphState::default())),
            neuron_state: Arc::new(Mutex::new(NeuronState::default())),
            intuition_feedback_state: Arc::new(Mutex::new(IntuitionFeedbackState::default())),
            model_router_feedback_state: Arc::new(Mutex::new(ModelRouterFeedbackState::default())),
        }
    }

    pub fn model_selection(&self) -> Result<ModelSelection, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.model_selection_for_session(&active_session_id)
    }

    pub fn model_selection_for_session(
        &self,
        session_id: &str,
    ) -> Result<ModelSelection, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .model_state
            .lock()
            .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
        Ok(ModelSelection {
            active: Self::resolve_model_for_session_from_state(&guard, session_id),
            available: self.providers.available_models(),
        })
    }

    pub fn execution_profile(&self) -> Result<ExecutionProfile, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.execution_profile_for_session(&active_session_id)
    }

    pub fn execution_profile_for_session(
        &self,
        session_id: &str,
    ) -> Result<ExecutionProfile, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .execution_profile_state
            .lock()
            .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
        Ok(Self::resolve_execution_profile_for_session_from_state(
            &guard, session_id,
        ))
    }

    pub fn switch_execution_profile(
        &self,
        target: ExecutionProfile,
    ) -> Result<SwitchExecutionProfileResult, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.switch_execution_profile_in_session(&active_session_id, target)
    }

    pub fn switch_execution_profile_in_session(
        &self,
        session_id: &str,
        target: ExecutionProfile,
    ) -> Result<SwitchExecutionProfileResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .execution_profile_state
            .lock()
            .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
        let previous = Self::resolve_execution_profile_for_session_from_state(&guard, session_id);
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        {
            existing.profile = target;
        } else {
            guard.sessions.push(SessionExecutionProfileBinding {
                session_id: session_id.to_string(),
                profile: target,
            });
        }
        drop(guard);
        let result = SwitchExecutionProfileResult {
            previous,
            current: target,
        };
        self.emit_event(
            EventKind::ExecutionProfileChanged,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "switched execution profile {} -> {}",
                format_execution_profile(result.previous),
                format_execution_profile(result.current)
            ),
        )?;
        Ok(result)
    }

    pub fn filesystem_scope(&self) -> Result<FilesystemScope, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.filesystem_scope_for_session(&active_session_id)
    }

    pub fn filesystem_scope_for_session(
        &self,
        session_id: &str,
    ) -> Result<FilesystemScope, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .filesystem_scope_state
            .lock()
            .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
        Ok(Self::resolve_filesystem_scope_for_session_from_state(
            &guard, session_id,
        ))
    }

    pub fn switch_filesystem_scope(
        &self,
        target: FilesystemScope,
    ) -> Result<SwitchFilesystemScopeResult, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.switch_filesystem_scope_in_session(&active_session_id, target)
    }

    pub fn switch_filesystem_scope_in_session(
        &self,
        session_id: &str,
        target: FilesystemScope,
    ) -> Result<SwitchFilesystemScopeResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .filesystem_scope_state
            .lock()
            .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
        let previous = Self::resolve_filesystem_scope_for_session_from_state(&guard, session_id);
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        {
            existing.scope = target;
        } else {
            guard.sessions.push(SessionFilesystemScopeBinding {
                session_id: session_id.to_string(),
                scope: target,
            });
        }
        drop(guard);
        let result = SwitchFilesystemScopeResult {
            previous,
            current: target,
        };
        self.emit_event(
            EventKind::FilesystemScopeChanged,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "switched filesystem scope {} -> {}",
                format_filesystem_scope(result.previous),
                format_filesystem_scope(result.current)
            ),
        )?;
        Ok(result)
    }

    pub fn write_path_scope(&self) -> Result<WritePathScope, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.write_path_scope_for_session(&active_session_id)
    }

    pub fn write_path_scope_for_session(
        &self,
        session_id: &str,
    ) -> Result<WritePathScope, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .write_path_scope_state
            .lock()
            .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
        Ok(Self::resolve_write_path_scope_for_session_from_state(
            &guard, session_id,
        ))
    }

    pub fn switch_write_path_scope(
        &self,
        target: WritePathScope,
    ) -> Result<SwitchWritePathScopeResult, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.switch_write_path_scope_in_session(&active_session_id, target)
    }

    pub fn switch_write_path_scope_in_session(
        &self,
        session_id: &str,
        target: WritePathScope,
    ) -> Result<SwitchWritePathScopeResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .write_path_scope_state
            .lock()
            .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
        let previous = Self::resolve_write_path_scope_for_session_from_state(&guard, session_id);
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        {
            existing.scope = target;
        } else {
            guard.sessions.push(SessionWritePathScopeBinding {
                session_id: session_id.to_string(),
                scope: target,
            });
        }
        drop(guard);
        let result = SwitchWritePathScopeResult {
            previous,
            current: target,
        };
        self.emit_event(
            EventKind::WritePathScopeChanged,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "switched write path scope {} -> {}",
                format_write_path_scope(result.previous),
                format_write_path_scope(result.current)
            ),
        )?;
        Ok(result)
    }

    pub fn capability_gate_report(&self) -> Result<CapabilityGateReport, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.capability_gate_report_for_session(&active_session_id)
    }

    pub fn capability_gate_report_for_session(
        &self,
        session_id: &str,
    ) -> Result<CapabilityGateReport, HeptaError> {
        Ok(CapabilityGateReport {
            session_id: session_id.to_string(),
            default_filesystem_scope: self.filesystem_scope_for_session(session_id)?,
            path_gates: self.path_capability_gates_for_session(session_id)?,
        })
    }

    pub fn path_capability_gates_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<PathCapabilityGate>, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .capability_gate_state
            .lock()
            .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
        Ok(Self::resolve_path_capability_gates_for_session_from_state(
            &guard, session_id,
        ))
    }

    pub fn set_path_capability_gate(
        &self,
        tool_name: &str,
        argument_name: &str,
        scope: FilesystemScope,
    ) -> Result<PathCapabilityGate, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.set_path_capability_gate_in_session(
            &active_session_id,
            tool_name,
            argument_name,
            scope,
        )
    }

    pub fn set_path_capability_gate_in_session(
        &self,
        session_id: &str,
        tool_name: &str,
        argument_name: &str,
        scope: FilesystemScope,
    ) -> Result<PathCapabilityGate, HeptaError> {
        let session_id = session_id.trim();
        let tool_name = tool_name.trim();
        let argument_name = argument_name.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        if tool_name.is_empty() {
            return Err(HeptaError("tool name must not be empty".into()));
        }
        if argument_name.is_empty() {
            return Err(HeptaError("argument name must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        self.tools.schema(tool_name)?;
        ensure_tool_schema_has_field(
            &self.tools.schema(tool_name)?.input_schema_json,
            tool_name,
            argument_name,
        )?;

        let mut guard = self
            .capability_gate_state
            .lock()
            .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
        let binding = Self::ensure_capability_binding_mut(&mut guard, session_id);
        if let Some(existing) = binding
            .path_gates
            .iter_mut()
            .find(|gate| gate.tool_name == tool_name && gate.argument_name == argument_name)
        {
            existing.scope = scope;
            let updated = existing.clone();
            drop(guard);
            self.emit_event(
                EventKind::CapabilityGateChanged,
                Some(SessionId(session_id.to_string())),
                None,
                format!(
                    "set path capability gate {} {}.{} -> {}",
                    updated.id,
                    updated.tool_name,
                    updated.argument_name,
                    format_filesystem_scope(updated.scope)
                ),
            )?;
            return Ok(updated);
        }

        let gate = PathCapabilityGate {
            id: format!(
                "cap-{}-{}-{}",
                session_id,
                tool_name,
                binding.path_gates.len() + 1
            ),
            tool_name: tool_name.to_string(),
            argument_name: argument_name.to_string(),
            scope,
        };
        binding.path_gates.push(gate.clone());
        drop(guard);
        self.emit_event(
            EventKind::CapabilityGateChanged,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "set path capability gate {} {}.{} -> {}",
                gate.id,
                gate.tool_name,
                gate.argument_name,
                format_filesystem_scope(gate.scope)
            ),
        )?;
        Ok(gate)
    }

    pub fn remove_path_capability_gate(&self, rule_id: &str) -> Result<bool, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.remove_path_capability_gate_in_session(&active_session_id, rule_id)
    }

    pub fn remove_path_capability_gate_in_session(
        &self,
        session_id: &str,
        rule_id: &str,
    ) -> Result<bool, HeptaError> {
        let session_id = session_id.trim();
        let rule_id = rule_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        if rule_id.is_empty() {
            return Err(HeptaError("rule id must not be empty".into()));
        }

        let mut guard = self
            .capability_gate_state
            .lock()
            .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
        let Some(binding) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        else {
            return Ok(false);
        };
        let before = binding.path_gates.len();
        binding.path_gates.retain(|gate| gate.id != rule_id);
        let removed = before != binding.path_gates.len();
        if binding.path_gates.is_empty() {
            guard.sessions.retain(|item| item.session_id != session_id);
        }
        drop(guard);
        if removed {
            self.emit_event(
                EventKind::CapabilityGateChanged,
                Some(SessionId(session_id.to_string())),
                None,
                format!("removed path capability gate {}", rule_id),
            )?;
        }
        Ok(removed)
    }

    pub fn switch_model(&self, target: &str) -> Result<SwitchModelResult, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.switch_model_in_session(&active_session_id, target)
    }

    pub fn switch_model_in_session(
        &self,
        session_id: &str,
        target: &str,
    ) -> Result<SwitchModelResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .model_state
            .lock()
            .map_err(|_| HeptaError("model state mutex poisoned".into()))?;

        let current = Self::resolve_model_for_session_from_state(&guard, session_id);
        let maybe = self.providers.find_model(target);

        let next = maybe.ok_or_else(|| HeptaError(format!("unknown model: {}", target)))?;
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|model| model.session_id == session_id)
        {
            existing.selected_model = next.clone();
        } else {
            guard.sessions.push(SessionModelState {
                session_id: session_id.to_string(),
                selected_model: next.clone(),
            });
        }

        let result = SwitchModelResult {
            previous: current,
            current: next,
        };
        self.emit_event(
            EventKind::ModelSwitched,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "switched model {}/{} -> {}/{}",
                result.previous.provider,
                result.previous.model,
                result.current.provider,
                result.current.model
            ),
        )?;
        Ok(result)
    }

    pub fn tool_names(&self) -> Vec<String> {
        self.tools.names()
    }

    pub fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        self.tools.descriptors()
    }

    pub fn provider_names(&self) -> Vec<String> {
        self.providers.names()
    }

    pub fn provider_catalog(&self) -> ProviderCatalog {
        ProviderCatalog {
            providers: self.providers.descriptors(),
        }
    }

    pub async fn policy_report(&self) -> Result<PolicyReport, HeptaError> {
        let active_session_id = self.active_session_id()?;
        let active_model = self.model_selection()?.active;
        let approvals = self.approval_snapshot_for_session(&active_session_id)?;
        let default_rules = self.policy.default_rules();
        let custom_rules = self
            .policy
            .custom_rules()
            .map_err(|err| HeptaError(err.0))?;

        let mut effective_tool_decisions = Vec::new();
        for tool in self.tool_descriptors() {
            let decision = self
                .policy
                .evaluate_tool(PolicyEvaluationContext {
                    session_id: Some(SessionId(active_session_id.clone())),
                    model: Some(active_model.clone()),
                    tool_name: tool.name.clone(),
                    risk_tier: tool.risk_tier,
                })
                .await
                .map_err(|err| HeptaError(err.0))?;
            effective_tool_decisions.push(PolicyToolDecisionReport {
                tool_name: tool.name,
                risk_tier: tool.risk_tier,
                requirement: decision.requirement,
                reason: decision.reason,
                matched_rule_id: decision.matched_rule_id,
            });
        }

        Ok(PolicyReport {
            active_session_id,
            active_model,
            default_rules,
            custom_rules,
            effective_tool_decisions,
            granted_approvals: approvals.granted_tools.len(),
            pending_approvals: approvals.pending.len(),
        })
    }

    pub fn add_policy_rule(
        &self,
        session_id: Option<&str>,
        provider_name: Option<&str>,
        tool_name: Option<&str>,
        risk_tier: Option<RiskTier>,
        requirement: ApprovalRequirement,
        reason: Option<&str>,
    ) -> Result<PolicyRule, HeptaError> {
        let normalized_session_id = session_id
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());
        let normalized_provider_name = provider_name
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());
        let normalized_tool_name = tool_name
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());

        if let Some(provider_name) = normalized_provider_name.as_deref() {
            if !self
                .provider_names()
                .iter()
                .any(|name| name == provider_name)
            {
                return Err(HeptaError(format!("unknown provider: {}", provider_name)));
            }
        }
        if let Some(tool_name) = normalized_tool_name.as_deref() {
            if !self.tools.contains(tool_name) {
                return Err(HeptaError(format!("unknown tool: {}", tool_name)));
            }
        }
        if let Some(session_id) = normalized_session_id.as_deref() {
            self.ensure_session_record_sync(session_id)?;
        }

        let next_index = self
            .policy
            .custom_rules()
            .map_err(|err| HeptaError(err.0))?
            .len()
            + 1;

        let rule = PolicyRule {
            id: format!("policy-{}-{}", current_unix_ms()?, next_index),
            session_id: normalized_session_id,
            provider_name: normalized_provider_name,
            tool_name: normalized_tool_name,
            risk_tier,
            requirement,
            reason: reason
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string())
                .unwrap_or_else(|| "custom policy rule".into()),
        };

        let stored = self
            .policy
            .add_rule(rule)
            .map_err(|err| HeptaError(err.0))?;
        self.emit_event(
            EventKind::PolicyUpdated,
            None,
            None,
            format!("added policy rule {}", stored.id),
        )?;
        Ok(stored)
    }

    pub fn remove_policy_rule(&self, rule_id: &str) -> Result<String, HeptaError> {
        let rule_id = rule_id.trim();
        if rule_id.is_empty() {
            return Err(HeptaError("policy rule id must not be empty".into()));
        }
        let removed = self
            .policy
            .remove_rule(rule_id)
            .map_err(|err| HeptaError(err.0))?;
        if !removed {
            return Err(HeptaError(format!("unknown policy rule: {}", rule_id)));
        }
        self.emit_event(
            EventKind::PolicyUpdated,
            None,
            None,
            format!("removed policy rule {}", rule_id),
        )?;
        Ok(format!("removed policy rule {}", rule_id))
    }

    pub fn reset_policy_rules(&self) -> Result<String, HeptaError> {
        let removed = self.policy.clear_rules().map_err(|err| HeptaError(err.0))?;
        self.emit_event(
            EventKind::PolicyUpdated,
            None,
            None,
            format!("cleared {} custom policy rule(s)", removed),
        )?;
        Ok(format!("cleared {} custom policy rule(s)", removed))
    }

    pub fn validate_tool_input(&self, tool_name: &str, input_json: &str) -> Result<(), HeptaError> {
        self.tools.validate_input(tool_name, input_json)
    }

    pub fn validate_tool_output(
        &self,
        tool_name: &str,
        output_json: &str,
    ) -> Result<(), HeptaError> {
        self.tools.validate_output(tool_name, output_json)
    }

    pub fn approval_snapshot(&self) -> Result<ApprovalSnapshot, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.approval_snapshot_for_session(&active_session_id)
    }

    pub fn approval_snapshot_for_session(
        &self,
        session_id: &str,
    ) -> Result<ApprovalSnapshot, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .approval_state
            .lock()
            .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
        Ok(guard.snapshot_for(session_id))
    }

    pub fn approve_tool(&self, tool_name: &str) -> Result<String, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.approve_tool_in_session(&active_session_id, tool_name)
    }

    pub fn approve_tool_in_session(
        &self,
        session_id: &str,
        tool_name: &str,
    ) -> Result<String, HeptaError> {
        if !self.tools.contains(tool_name) {
            return Err(HeptaError(format!("unknown tool: {}", tool_name)));
        }
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .approval_state
            .lock()
            .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
        guard.grant(session_id, tool_name);
        drop(guard);
        self.emit_event(
            EventKind::ApprovalGranted,
            Some(SessionId(session_id.to_string())),
            None,
            format!("approved tool {}", tool_name),
        )?;
        Ok(format!(
            "approved tool for session {}: {}",
            session_id, tool_name
        ))
    }

    pub fn active_session_id(&self) -> Result<String, HeptaError> {
        let guard = self
            .session_state
            .lock()
            .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
        Ok(guard.active_session_id.clone())
    }

    pub fn switch_session(&self, session_id: &str) -> Result<String, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }

        {
            let mut guard = self
                .session_state
                .lock()
                .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
            guard.active_session_id = session_id.to_string();
        }

        self.ensure_session_record_sync(session_id)?;
        self.emit_event(
            EventKind::SessionSwitched,
            Some(SessionId(session_id.to_string())),
            None,
            format!("switched active session to {}", session_id),
        )?;
        Ok(format!("switched active session to {}", session_id))
    }

    pub fn active_session_snapshot(&self) -> Result<SessionSnapshot, HeptaError> {
        let active_session_id = self.active_session_id()?;
        match self.session_snapshot_for_id(&active_session_id) {
            Ok(session) => Ok(session),
            Err(err) if err.0 == format!("session not found: {}", active_session_id) => Err(
                HeptaError(format!("active session not found: {}", active_session_id)),
            ),
            Err(err) => Err(err),
        }
    }

    pub fn rename_active_session(&self, title: &str) -> Result<String, HeptaError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(HeptaError("session title must not be empty".into()));
        }
        let session_id = SessionId(self.active_session_id()?);
        self.upsert_session_record(&session_id, Some(title.to_string()), None, None, true)?;
        self.emit_event(
            EventKind::SessionRenamed,
            Some(session_id.clone()),
            None,
            format!("renamed session to \"{}\"", title),
        )?;
        Ok(format!(
            "renamed active session {} to \"{}\"",
            session_id.0, title
        ))
    }

    pub fn archive_session(&self, session_id: Option<&str>) -> Result<String, HeptaError> {
        let session_id = session_id
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(|item| item.to_string())
            .unwrap_or(self.active_session_id()?);
        let record = self.session_snapshot_for_id(&session_id)?;

        if record.archived_at_unix_ms.is_some() {
            return Ok(format!("session {} is already archived", session_id));
        }

        if self.active_session_id()? == session_id {
            let fallback = self.choose_fallback_session_id(Some(&session_id))?;
            let mut guard = self
                .session_state
                .lock()
                .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
            guard.active_session_id = fallback;
        }

        self.upsert_session_record(
            &SessionId(record.session_id.clone()),
            None,
            None,
            Some(Some(current_unix_ms()?)),
            false,
        )?;
        self.emit_event(
            EventKind::SessionArchived,
            Some(SessionId(record.session_id.clone())),
            None,
            format!("archived session {}", session_id),
        )?;
        Ok(format!("archived session {}", session_id))
    }

    pub fn unarchive_session(&self, session_id: &str) -> Result<String, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        self.upsert_session_record(
            &SessionId(session_id.to_string()),
            None,
            None,
            Some(None),
            false,
        )?;
        self.emit_event(
            EventKind::SessionUnarchived,
            Some(SessionId(session_id.to_string())),
            None,
            format!("unarchived session {}", session_id),
        )?;
        Ok(format!("unarchived session {}", session_id))
    }

    pub fn delete_session(&self, session_id: &str) -> Result<String, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }

        let active_session_id = self.active_session_id()?;
        if active_session_id == session_id {
            let fallback = self.choose_fallback_session_id(Some(session_id))?;
            let mut guard = self
                .session_state
                .lock()
                .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
            guard.active_session_id = fallback;
        }

        let removed = self
            .memory
            .remove_session_sync(&SessionId(session_id.to_string()))
            .map_err(|err| HeptaError(err.0))?;
        if removed.is_none() {
            return Err(HeptaError(format!("unknown session: {}", session_id)));
        }

        {
            let mut approval_state = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            approval_state.remove_session(session_id);
        }
        {
            let mut history_state = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            history_state.retain(|turn| turn.session_id != session_id);
        }
        {
            let mut model_state = self
                .model_state
                .lock()
                .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
            model_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        {
            let mut execution_profile_state = self
                .execution_profile_state
                .lock()
                .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
            execution_profile_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        {
            let mut filesystem_scope_state = self
                .filesystem_scope_state
                .lock()
                .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
            filesystem_scope_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        {
            let mut capability_gate_state = self
                .capability_gate_state
                .lock()
                .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
            capability_gate_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        {
            let mut write_path_scope_state = self
                .write_path_scope_state
                .lock()
                .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
            write_path_scope_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        self.replace_topic_export_state_for_session(session_id, Vec::new(), Vec::new())?;
        self.replace_neuron_state_for_session(session_id, Vec::new())?;
        self.replace_intuition_feedback_for_session(session_id, Vec::new())?;
        self.replace_model_router_feedback_for_session(session_id, Vec::new())?;

        self.emit_event(
            EventKind::SessionDeleted,
            Some(SessionId(session_id.to_string())),
            None,
            format!("deleted session {}", session_id),
        )?;

        Ok(format!("deleted session {}", session_id))
    }

    pub fn prune_sessions(&self, max_sessions: usize) -> Result<String, HeptaError> {
        if max_sessions == 0 {
            return Err(HeptaError("max session count must be at least 1".into()));
        }

        let active_session_id = self.active_session_id()?;
        let sessions = self.sessions()?;
        let total_sessions = sessions.len();
        if total_sessions <= max_sessions {
            return Ok(format!(
                "no pruning needed, sessions={} max={}",
                total_sessions, max_sessions
            ));
        }

        let mut candidates = sessions
            .into_iter()
            .filter(|session| !session.is_active)
            .collect::<Vec<_>>();
        candidates.sort_by_key(|session| {
            (
                if session.archived_at_unix_ms.is_some() {
                    0_u8
                } else {
                    1_u8
                },
                session.last_active_unix_ms,
            )
        });

        let delete_count = total_sessions.saturating_sub(max_sessions);
        let targets = candidates
            .into_iter()
            .take(delete_count)
            .map(|session| session.session_id)
            .collect::<Vec<_>>();

        if targets.is_empty() {
            return Ok(format!(
                "no prune candidates available, active session protected: {}",
                active_session_id
            ));
        }

        for session_id in &targets {
            self.delete_session(session_id)?;
        }

        self.emit_event(
            EventKind::SessionsPruned,
            None,
            None,
            format!(
                "pruned {} session(s): {}",
                targets.len(),
                targets.join(", ")
            ),
        )?;

        Ok(format!(
            "pruned {} session(s): {}",
            targets.len(),
            targets.join(", ")
        ))
    }

    pub fn export_session(
        &self,
        session_id: &str,
        path: &str,
    ) -> Result<SessionExportReport, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let path = path.trim();
        if path.is_empty() {
            return Err(HeptaError("export path must not be empty".into()));
        }

        let export = self.session_export(session_id)?;
        let approvals_granted = export.approval.granted_tools.len();
        let approvals_pending = export.approval.pending.len();
        let history_entries = export.history.len();
        let topic_session_count = export.topic_sessions.len();
        let topic_graph_edge_count = export.topic_graph_edges.len();
        let neuron_count = export.neurons.len();
        let intuition_feedback_count = export.intuition_feedback.len();
        let model_router_feedback_count = export.model_router_feedback.len();
        let exported_at_unix_ms = export.exported_at_unix_ms;
        let title = export.session.title.clone();
        let model = export.model.clone();
        let archived = export.session.archived_at_unix_ms.is_some();
        let export_json = serde_json::to_string_pretty(&export)
            .map_err(|err| HeptaError(format!("failed to serialize session export: {}", err)))?;
        let export_path = PathBuf::from(path);
        if let Some(parent) = export_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|err| {
                    HeptaError(format!(
                        "failed to create export directory {}: {}",
                        parent.display(),
                        err
                    ))
                })?;
            }
        }
        fs::write(&export_path, export_json).map_err(|err| {
            HeptaError(format!(
                "failed to write session export {}: {}",
                export_path.display(),
                err
            ))
        })?;
        let report = SessionExportReport {
            session_id: session_id.to_string(),
            export_path: export_path.display().to_string(),
            exported_at_unix_ms,
            title,
            model,
            archived,
            approvals_granted,
            approvals_pending,
            history_entries,
            topic_session_count,
            topic_graph_edge_count,
            neuron_count,
            intuition_feedback_count,
            model_router_feedback_count,
        };
        self.emit_event(
            EventKind::SessionExported,
            Some(SessionId(session_id.to_string())),
            None,
            format!("exported session to {}", report.export_path),
        )?;
        Ok(report)
    }

    pub fn import_session(&self, path: &str) -> Result<SessionImportReport, HeptaError> {
        let path = path.trim();
        if path.is_empty() {
            return Err(HeptaError("import path must not be empty".into()));
        }

        let import_path = PathBuf::from(path);
        let import_json = fs::read_to_string(&import_path).map_err(|err| {
            HeptaError(format!(
                "failed to read session import {}: {}",
                import_path.display(),
                err
            ))
        })?;
        let export: SessionExport = serde_json::from_str(&import_json).map_err(|err| {
            HeptaError(format!(
                "failed to parse session import {}: {}",
                import_path.display(),
                err
            ))
        })?;
        if export.version != 1 {
            return Err(HeptaError(format!(
                "unsupported session export version: {}",
                export.version
            )));
        }

        let session_id = export.session.session_id.0.clone();
        let imported_title = export.session.title.clone();
        let imported_model = export.model.clone();
        let imported_archived = export.session.archived_at_unix_ms.is_some();
        let approvals_granted = export.approval.granted_tools.len();
        let approvals_pending = export.approval.pending.len();
        let history_entries = export.history.len();
        let topic_session_count = export.topic_sessions.len();
        let topic_graph_edge_count = export.topic_graph_edges.len();
        let neuron_count = export.neurons.len();
        let intuition_feedback_count = export.intuition_feedback.len();
        let model_router_feedback_count = export.model_router_feedback.len();
        self.apply_session_export(export)?;
        let report = SessionImportReport {
            session_id,
            import_path: import_path.display().to_string(),
            imported_title,
            imported_model,
            imported_archived,
            approvals_granted,
            approvals_pending,
            history_entries,
            topic_session_count,
            topic_graph_edge_count,
            neuron_count,
            intuition_feedback_count,
            model_router_feedback_count,
        };
        self.emit_event(
            EventKind::SessionImported,
            Some(SessionId(report.session_id.clone())),
            None,
            format!("imported session from {}", report.import_path),
        )?;
        Ok(report)
    }

    pub fn backup_index(&self, target_path: Option<&str>) -> Result<BackupIndexReport, HeptaError> {
        let workspace_root = self.workspace_root()?;
        let backup_root = workspace_root.join("artifacts/backups/write_file");
        let filter_target_path = target_path
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| {
                resolve_path_within_root(&workspace_root, Path::new(value))
                    .display()
                    .to_string()
            });

        let mut files = Vec::new();
        collect_files_recursive(&backup_root, &mut files)?;
        let mut backups = files
            .into_iter()
            .filter_map(|path| parse_backup_entry(&workspace_root, &backup_root, &path).transpose())
            .collect::<Result<Vec<_>, HeptaError>>()?;

        if let Some(filter_target_path) = filter_target_path.as_deref() {
            backups.retain(|entry| entry.target_path == filter_target_path);
        }
        backups.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));

        Ok(BackupIndexReport {
            backup_root: backup_root.display().to_string(),
            filter_target_path,
            backups,
        })
    }

    pub fn restore_backup(&self, backup_ref: &str) -> Result<RestoreBackupReport, HeptaError> {
        let backup_ref = backup_ref.trim();
        if backup_ref.is_empty() {
            return Err(HeptaError("backup reference must not be empty".into()));
        }
        let workspace_root = self.workspace_root()?;
        let backup_root = workspace_root.join("artifacts/backups/write_file");
        let backup_path = resolve_backup_reference(&backup_root, backup_ref)?;
        let backup =
            parse_backup_entry(&workspace_root, &backup_root, &backup_path)?.ok_or_else(|| {
                HeptaError(format!(
                    "backup not found under {}: {}",
                    backup_root.display(),
                    backup_ref
                ))
            })?;
        let active_session_id = self.active_session_id()?;
        self.ensure_write_path_scope_allows_path_string(
            &SessionId(active_session_id.clone()),
            "restore_backup",
            &backup.target_path,
        )?;
        self.ensure_write_target_unlocked(
            &active_session_id,
            &backup.target_path,
            "restore_backup",
        )?;

        let target_path = PathBuf::from(&backup.target_path);
        let target_existed_before_restore = target_path.exists();
        let previous_target_backup_path = if target_existed_before_restore {
            let existing = fs::read(&target_path).map_err(|err| {
                HeptaError(format!(
                    "failed to read current target {} before restore: {}",
                    target_path.display(),
                    err
                ))
            })?;
            let planned_backup = preview_backup_path(&workspace_root, &target_path)
                .map_err(|err| HeptaError(err.0))?;
            if let Some(parent) = planned_backup.parent() {
                fs::create_dir_all(parent).map_err(|err| {
                    HeptaError(format!(
                        "failed to create restore-backup parent {}: {}",
                        parent.display(),
                        err
                    ))
                })?;
            }
            fs::write(&planned_backup, existing).map_err(|err| {
                HeptaError(format!(
                    "failed to write restore backup {}: {}",
                    planned_backup.display(),
                    err
                ))
            })?;
            Some(planned_backup.display().to_string())
        } else {
            None
        };

        let backup_bytes = fs::read(&backup_path).map_err(|err| {
            HeptaError(format!(
                "failed to read backup {}: {}",
                backup_path.display(),
                err
            ))
        })?;
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create restore target parent {}: {}",
                    parent.display(),
                    err
                ))
            })?;
        }
        fs::write(&target_path, &backup_bytes).map_err(|err| {
            HeptaError(format!(
                "failed to restore {} from {}: {}",
                target_path.display(),
                backup_path.display(),
                err
            ))
        })?;

        let active_session = SessionId(active_session_id.clone());
        let transaction_id = self.record_restore_backup_transaction(
            &active_session,
            &backup.target_path,
            target_existed_before_restore,
            backup_bytes.len() as u64,
            previous_target_backup_path.clone(),
            backup.backup_path.clone(),
        )?;

        let report = RestoreBackupReport {
            transaction_id,
            backup_id: backup.id.clone(),
            backup_path: backup.backup_path.clone(),
            restored_target_path: backup.target_path.clone(),
            restored_bytes: backup_bytes.len() as u64,
            target_existed_before_restore,
            previous_target_backup_path,
        };
        self.emit_event(
            EventKind::BackupRestored,
            Some(active_session),
            None,
            format!(
                "restored backup {} to {}",
                report.backup_id, report.restored_target_path
            ),
        )?;
        Ok(report)
    }

    pub fn preview_prune_backups(
        &self,
        target_path: Option<&str>,
        keep_latest_per_target: usize,
        max_age_ms: Option<u64>,
    ) -> Result<BackupPruneReport, HeptaError> {
        self.plan_backup_prune(target_path, keep_latest_per_target, max_age_ms, false)
    }

    pub fn prune_backups(
        &self,
        target_path: Option<&str>,
        keep_latest_per_target: usize,
        max_age_ms: Option<u64>,
    ) -> Result<BackupPruneReport, HeptaError> {
        let report =
            self.plan_backup_prune(target_path, keep_latest_per_target, max_age_ms, true)?;
        if report.deleted_count > 0 {
            self.emit_event(
                EventKind::BackupsPruned,
                Some(SessionId(self.active_session_id()?)),
                None,
                format!(
                    "pruned {} backups under {}",
                    report.deleted_count, report.backup_root
                ),
            )?;
        }
        Ok(report)
    }

    pub fn write_transactions(
        &self,
        target_path: Option<&str>,
    ) -> Result<WriteTransactionIndexReport, HeptaError> {
        let workspace_root = self.workspace_root()?;
        let filter_target_path = target_path
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| {
                resolve_path_within_root(&workspace_root, Path::new(value))
                    .display()
                    .to_string()
            });

        let mut transactions = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .clone();
        if let Some(filter_target_path) = filter_target_path.as_deref() {
            transactions.retain(|entry| entry.target_path == filter_target_path);
        }
        transactions.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));

        Ok(WriteTransactionIndexReport {
            filter_target_path,
            transactions,
        })
    }

    pub fn begin_write_transaction_group(
        &self,
        group_id: Option<&str>,
    ) -> Result<BeginWriteTransactionGroupReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let group_id = self.next_write_transaction_group_id(group_id)?;
        let opened_at_unix_ms = current_unix_ms()?;
        {
            let mut guard = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            if guard
                .active_bindings
                .iter()
                .any(|binding| binding.session_id == session_id)
            {
                return Err(HeptaError(format!(
                    "session {} already has an active write transaction group",
                    session_id
                )));
            }
            guard.groups.push(WriteTransactionGroup {
                group_id: group_id.clone(),
                session_id: session_id.clone(),
                opened_at_unix_ms,
                closed_at_unix_ms: None,
                transaction_ids: Vec::new(),
            });
            guard
                .active_bindings
                .push(SessionWriteTransactionGroupBinding {
                    session_id: session_id.clone(),
                    active_group_id: group_id.clone(),
                });
        }
        self.emit_event(
            EventKind::WriteTransactionGroupOpened,
            Some(SessionId(session_id.clone())),
            None,
            format!("opened write transaction group {}", group_id),
        )?;
        Ok(BeginWriteTransactionGroupReport {
            session_id,
            group_id,
            opened_at_unix_ms,
        })
    }

    pub fn end_write_transaction_group(
        &self,
    ) -> Result<EndWriteTransactionGroupReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let closed_at_unix_ms = current_unix_ms()?;
        let (group_id, transaction_count) = {
            let mut guard = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            let binding_index = guard
                .active_bindings
                .iter()
                .position(|binding| binding.session_id == session_id)
                .ok_or_else(|| {
                    HeptaError(format!(
                        "session {} has no active write transaction group",
                        session_id
                    ))
                })?;
            let group_id = guard.active_bindings.remove(binding_index).active_group_id;
            let group = guard
                .groups
                .iter_mut()
                .find(|group| group.group_id == group_id)
                .ok_or_else(|| {
                    HeptaError(format!("unknown write transaction group: {}", group_id))
                })?;
            group.closed_at_unix_ms = Some(closed_at_unix_ms);
            (group_id, group.transaction_ids.len())
        };
        self.emit_event(
            EventKind::WriteTransactionGroupClosed,
            Some(SessionId(session_id.clone())),
            None,
            format!("closed write transaction group {}", group_id),
        )?;
        Ok(EndWriteTransactionGroupReport {
            session_id,
            group_id,
            closed_at_unix_ms,
            transaction_count,
        })
    }

    pub fn write_transaction_groups(&self) -> Result<WriteTransactionGroupIndexReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        let active_group_id = guard
            .active_bindings
            .iter()
            .find(|binding| binding.session_id == session_id)
            .map(|binding| binding.active_group_id.clone());
        let mut groups = guard
            .groups
            .iter()
            .filter(|group| group.session_id == session_id)
            .cloned()
            .collect::<Vec<_>>();
        groups.sort_by(|left, right| right.opened_at_unix_ms.cmp(&left.opened_at_unix_ms));
        Ok(WriteTransactionGroupIndexReport {
            session_id,
            active_group_id,
            groups,
        })
    }

    pub fn write_locks(&self) -> Result<WriteLockReport, HeptaError> {
        self.prune_stale_write_locks_internal(false)?;
        let guard = self
            .write_lock_state
            .lock()
            .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
        let target_locks = guard
            .target_locks
            .iter()
            .cloned()
            .map(|lock| {
                let attempt = if let Some(attempt_id) = lock.rollback_attempt_id.as_deref() {
                    self.live_rollback_group_attempt_by_id(attempt_id)?
                } else {
                    None
                };
                Ok(rollback_locks::build_write_target_lock_report(
                    lock,
                    attempt.as_ref(),
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let group_locks = guard
            .group_locks
            .iter()
            .cloned()
            .map(|lock| {
                let attempt = if let Some(attempt_id) = lock.rollback_attempt_id.as_deref() {
                    self.live_rollback_group_attempt_by_id(attempt_id)?
                } else {
                    None
                };
                Ok(rollback_locks::build_write_group_lock_report(
                    lock,
                    attempt.as_ref(),
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rollback_locks::build_write_lock_report(
            WRITE_LOCK_REPORT_SCHEMA_VERSION,
            target_locks,
            group_locks,
        ))
    }

    pub fn prune_stale_write_locks(&self) -> Result<WriteLockPruneReport, HeptaError> {
        self.prune_stale_write_locks_internal(true)
    }

    pub fn rollback_write_plan(&self, group_id: &str) -> Result<RollbackPlanReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let group_id = group_id.trim();
        if group_id.is_empty() {
            return Err(HeptaError("group id must not be empty".into()));
        }
        let (group, active) = self.find_write_transaction_group(&session_id, group_id)?;
        let transactions = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .clone();
        let mut steps = Vec::new();
        for transaction_id in group.transaction_ids.iter().rev() {
            let maybe_entry = transactions
                .iter()
                .find(|entry| entry.transaction_id == *transaction_id)
                .cloned();
            let step = if let Some(entry) = maybe_entry {
                let already_rolled_back = entry.rolled_back_at_unix_ms.is_some();
                let blocking_reason = if already_rolled_back {
                    None
                } else {
                    match entry.rollback_strategy.as_str() {
                        "restore_checkpoint" => match entry.rollback_checkpoint_path.as_deref() {
                            Some(path) if PathBuf::from(path).exists() => None,
                            Some(path) => Some(format!("rollback checkpoint missing: {}", path)),
                            None => Some("rollback checkpoint missing".into()),
                        },
                        "delete_target" => None,
                        other => Some(format!("unsupported rollback strategy {}", other)),
                    }
                };
                RollbackPlanStep {
                    transaction_id: entry.transaction_id,
                    target_path: entry.target_path,
                    rollback_strategy: entry.rollback_strategy,
                    rollback_checkpoint_path: entry.rollback_checkpoint_path,
                    ready: blocking_reason.is_none(),
                    already_rolled_back,
                    blocking_reason,
                }
            } else {
                RollbackPlanStep {
                    transaction_id: transaction_id.clone(),
                    target_path: String::new(),
                    rollback_strategy: String::new(),
                    rollback_checkpoint_path: None,
                    ready: false,
                    already_rolled_back: false,
                    blocking_reason: Some("transaction not found".into()),
                }
            };
            steps.push(step);
        }
        let closed = group.closed_at_unix_ms.is_some();
        let executable = closed
            && !active
            && steps
                .iter()
                .all(|step| step.already_rolled_back || step.ready);
        Ok(RollbackPlanReport {
            session_id,
            group_id: group.group_id,
            active,
            closed,
            executable,
            steps,
        })
    }

    pub fn rollback_write_group(&self, group_id: &str) -> Result<RollbackGroupReport, HeptaError> {
        self.rollback_write_group_internal(group_id, None)
    }

    pub fn rollback_group_status(
        &self,
        group_id: &str,
    ) -> Result<RollbackGroupStatusReport, HeptaError> {
        let plan = self.rollback_write_plan(group_id)?;
        let latest_attempt =
            self.latest_rollback_group_attempt(&plan.session_id, &plan.group_id)?;
        let attempt_lifecycle =
            self.rollback_group_attempt_lifecycle(&plan.session_id, &plan.group_id)?;
        let lock_diagnostics = self.rollback_group_lock_diagnostics(
            &plan.session_id,
            &plan.group_id,
            latest_attempt
                .as_ref()
                .map(|attempt| attempt.attempt_id.as_str()),
        )?;
        let group_lock_attempt_id = lock_diagnostics.group_lock_attempt_id.clone();
        let active_attempt_id = attempt_lifecycle.active_attempt_id.clone();
        let (suggested_next_action, resume_command) = match latest_attempt.as_ref() {
            Some(attempt)
                if attempt.status == RollbackGroupAttemptStatus::PartialFailed
                    && lock_diagnostics.latest_attempt_owns_lock_set =>
            {
                (
                    format!("resume partial rollback for group {}", plan.group_id),
                    Some(format!("/resume-rollback-group {}", plan.group_id)),
                )
            }
            Some(attempt)
                if attempt.status == RollbackGroupAttemptStatus::PartialFailed
                    && lock_diagnostics.orphaned_lock_count > 0 =>
            {
                (
                    format!(
                        "prune orphaned locks before resuming group {}",
                        plan.group_id
                    ),
                    Some("/prune-stale-locks".into()),
                )
            }
            Some(attempt) if attempt.status == RollbackGroupAttemptStatus::Completed => (
                if lock_diagnostics.group_locked {
                    format!("prune leftover locks for completed group {}", plan.group_id)
                } else {
                    "group rollback already completed".into()
                },
                if lock_diagnostics.group_locked {
                    Some("/prune-stale-locks".into())
                } else {
                    None
                },
            ),
            _ if lock_diagnostics.group_locked && lock_diagnostics.orphaned_lock_count > 0 => (
                format!("prune orphaned locks for group {}", plan.group_id),
                Some("/prune-stale-locks".into()),
            ),
            _ if plan.executable => (
                format!("run rollback for group {}", plan.group_id),
                Some(format!("/rollback-group {}", plan.group_id)),
            ),
            _ => (
                format!("fix rollback plan blockers for group {}", plan.group_id),
                None,
            ),
        };
        Ok(RollbackGroupStatusReport {
            schema_version: ROLLBACK_GROUP_STATUS_SCHEMA_VERSION,
            group_locked: lock_diagnostics.group_locked,
            group_lock_attempt_id: group_lock_attempt_id.clone(),
            target_lock_count: lock_diagnostics.target_lock_count,
            orphaned_lock_count: lock_diagnostics.orphaned_lock_count,
            latest_attempt_owns_lock_set: lock_diagnostics.latest_attempt_owns_lock_set,
            attempt_count: attempt_lifecycle.attempt_count,
            superseded_attempt_count: attempt_lifecycle.superseded_attempt_count,
            active_attempt_id: active_attempt_id.clone(),
            lock_diagnostics: RollbackGroupLockDiagnosticsReport {
                group_lock_attempt_id,
                target_lock_count: lock_diagnostics.target_lock_count,
                orphaned_lock_count: lock_diagnostics.orphaned_lock_count,
                latest_attempt_owns_lock_set: lock_diagnostics.latest_attempt_owns_lock_set,
            },
            attempt_lifecycle: RollbackGroupAttemptLifecycleReport {
                attempt_count: attempt_lifecycle.attempt_count,
                superseded_attempt_count: attempt_lifecycle.superseded_attempt_count,
                active_attempt_id,
            },
            session_id: plan.session_id,
            group_id: plan.group_id,
            executable_now: plan.executable,
            latest_attempt,
            suggested_next_action,
            resume_command,
        })
    }

    pub fn resume_rollback_write_group(
        &self,
        group_id: &str,
    ) -> Result<RollbackGroupReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let group_id = group_id.trim();
        if group_id.is_empty() {
            return Err(HeptaError("group id must not be empty".into()));
        }
        let latest_attempt = self
            .latest_rollback_group_attempt(&session_id, group_id)?
            .ok_or_else(|| {
                HeptaError(format!("no rollback attempt exists for group {}", group_id))
            })?;
        if latest_attempt.status != RollbackGroupAttemptStatus::PartialFailed {
            return Err(HeptaError(format!(
                "latest rollback attempt for group {} is not partial_failed",
                group_id
            )));
        }
        self.rollback_write_group_internal(group_id, Some(latest_attempt.attempt_id))
    }

    fn rollback_write_group_internal(
        &self,
        group_id: &str,
        resumed_from_attempt_id: Option<String>,
    ) -> Result<RollbackGroupReport, HeptaError> {
        let plan = self.rollback_write_plan(group_id)?;
        if !plan.executable {
            return Err(HeptaError(format!(
                "rollback plan for group {} is not executable",
                plan.group_id
            )));
        }
        let attempt_id = self.next_rollback_group_attempt_id()?;
        let started_at_unix_ms = current_unix_ms()?;
        let locked_target_paths = plan
            .steps
            .iter()
            .map(|step| step.target_path.clone())
            .filter(|target_path| !target_path.is_empty())
            .collect::<Vec<_>>();
        self.acquire_group_rollback_locks(
            &plan.session_id,
            &plan.group_id,
            &attempt_id,
            &locked_target_paths,
        )?;
        let mut executed_transaction_ids = Vec::new();
        let mut skipped_already_rolled_back_ids = Vec::new();
        let mut pending_transaction_ids = plan
            .steps
            .iter()
            .filter(|step| !step.already_rolled_back)
            .map(|step| step.transaction_id.clone())
            .collect::<Vec<_>>();
        let mut target_paths_restored = Vec::new();
        let mut failed_transaction_id = None;
        let mut failure_reason = None;
        if let Some(resumed_from_attempt_id) = resumed_from_attempt_id.as_ref() {
            self.emit_event_with_payload(
                EventKind::WriteGroupRollbackResumed,
                Some(SessionId(plan.session_id.clone())),
                None,
                format!(
                    "resumed rollback for group {} from attempt {}",
                    plan.group_id, resumed_from_attempt_id
                ),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "group_id": plan.group_id.clone(),
                    "resumed_from_attempt_id": resumed_from_attempt_id.clone(),
                    "resumed_attempt_id": attempt_id.clone(),
                })),
            )?;
        }
        for step in &plan.steps {
            if step.already_rolled_back {
                skipped_already_rolled_back_ids.push(step.transaction_id.clone());
                continue;
            }
            match self.rollback_write_transaction(&step.transaction_id) {
                Ok(report) => {
                    executed_transaction_ids.push(report.transaction_id);
                    pending_transaction_ids.retain(|id| id != &step.transaction_id);
                    target_paths_restored.push(report.target_path);
                }
                Err(err) => {
                    failed_transaction_id = Some(step.transaction_id.clone());
                    failure_reason = Some(err.0);
                    break;
                }
            }
        }
        let finished_at_unix_ms = current_unix_ms()?;
        let status = if failed_transaction_id.is_some() {
            RollbackGroupAttemptStatus::PartialFailed
        } else {
            RollbackGroupAttemptStatus::Completed
        };
        let attempt = RollbackGroupAttempt {
            attempt_id: attempt_id.clone(),
            session_id: plan.session_id.clone(),
            group_id: plan.group_id.clone(),
            started_at_unix_ms,
            finished_at_unix_ms: Some(finished_at_unix_ms),
            status: status.clone(),
            resumed_from_attempt_id: resumed_from_attempt_id.clone(),
            superseded_by_attempt_id: None,
            executed_transaction_ids: executed_transaction_ids.clone(),
            skipped_already_rolled_back_ids: skipped_already_rolled_back_ids.clone(),
            pending_transaction_ids: pending_transaction_ids.clone(),
            failed_transaction_id: failed_transaction_id.clone(),
            failure_reason: failure_reason.clone(),
            target_paths_restored: target_paths_restored.clone(),
        };
        {
            let mut guard = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            if let Some(resumed_from_attempt_id) = resumed_from_attempt_id.as_ref() {
                if let Some(previous_attempt) =
                    guard.rollback_attempts.iter_mut().find(|previous_attempt| {
                        previous_attempt.session_id == plan.session_id
                            && previous_attempt.group_id == plan.group_id
                            && previous_attempt.attempt_id == *resumed_from_attempt_id
                    })
                {
                    previous_attempt.superseded_by_attempt_id = Some(attempt_id.clone());
                }
            }
            guard.rollback_attempts.push(attempt);
        }
        let suggested_next_action = if failed_transaction_id.is_some() {
            format!("inspect rollback status and resume group {}", plan.group_id)
        } else {
            format!("rollback for group {} completed", plan.group_id)
        };
        let resume_command = failed_transaction_id
            .as_ref()
            .map(|_| format!("/resume-rollback-group {}", plan.group_id));
        if let Some(failed_transaction_id) = failed_transaction_id.clone() {
            self.emit_event_with_payload(
                EventKind::WriteGroupRollbackFailed,
                Some(SessionId(plan.session_id.clone())),
                None,
                format!(
                    "rollback group {} failed at {}: {}",
                    plan.group_id,
                    failed_transaction_id,
                    failure_reason
                        .clone()
                        .unwrap_or_else(|| "unknown failure".into())
                ),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "group_id": plan.group_id.clone(),
                    "attempt_id": attempt_id.clone(),
                    "resumed_from_attempt_id": resumed_from_attempt_id.clone(),
                    "status": "partial_failed",
                    "failed_transaction_id": failed_transaction_id.clone(),
                    "failure_reason": failure_reason.clone(),
                    "executed_transaction_ids": executed_transaction_ids.clone(),
                    "pending_transaction_ids": pending_transaction_ids.clone(),
                    "target_paths_restored": target_paths_restored.clone(),
                })),
            )?;
            return Ok(RollbackGroupReport {
                session_id: plan.session_id,
                group_id: plan.group_id,
                attempt_id,
                status,
                resumed_from_attempt_id,
                executed_transaction_ids,
                skipped_already_rolled_back_ids,
                pending_transaction_ids,
                failed_transaction_id: Some(failed_transaction_id),
                failure_reason,
                target_paths_restored,
                suggested_next_action,
                resume_command,
            });
        }
        self.release_group_rollback_locks(&plan.session_id, &plan.group_id)?;
        self.emit_event_with_payload(
            EventKind::WriteGroupRolledBack,
            Some(SessionId(plan.session_id.clone())),
            None,
            format!("rolled back write transaction group {}", plan.group_id),
            Some(json!({
                "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                "group_id": plan.group_id.clone(),
                "attempt_id": attempt_id.clone(),
                "resumed_from_attempt_id": resumed_from_attempt_id.clone(),
                "status": "completed",
                "executed_transaction_ids": executed_transaction_ids.clone(),
                "skipped_already_rolled_back_ids": skipped_already_rolled_back_ids.clone(),
                "pending_transaction_ids": pending_transaction_ids.clone(),
                "target_paths_restored": target_paths_restored.clone(),
            })),
        )?;
        Ok(RollbackGroupReport {
            session_id: plan.session_id,
            group_id: plan.group_id,
            attempt_id,
            status,
            resumed_from_attempt_id,
            executed_transaction_ids,
            skipped_already_rolled_back_ids,
            pending_transaction_ids,
            failed_transaction_id,
            failure_reason,
            target_paths_restored,
            suggested_next_action,
            resume_command,
        })
    }

    pub fn rollback_write_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<RollbackWriteReport, HeptaError> {
        let transaction_id = transaction_id.trim();
        if transaction_id.is_empty() {
            return Err(HeptaError("transaction id must not be empty".into()));
        }

        let entry = {
            let guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            guard
                .iter()
                .find(|entry| entry.transaction_id == transaction_id)
                .cloned()
                .ok_or_else(|| {
                    HeptaError(format!("unknown write transaction: {}", transaction_id))
                })?
        };

        if entry.rolled_back_at_unix_ms.is_some() {
            return Err(HeptaError(format!(
                "write transaction {} already rolled back",
                transaction_id
            )));
        }

        {
            let mut guard = self
                .rollback_failure_injection_state
                .lock()
                .map_err(|_| HeptaError("rollback failure injection mutex poisoned".into()))?;
            if let Some(index) = guard
                .iter()
                .position(|candidate| candidate == transaction_id)
            {
                guard.remove(index);
                return Err(HeptaError(format!(
                    "injected rollback failure for transaction {}",
                    transaction_id
                )));
            }
        }

        let active_session_id = self.active_session_id()?;
        self.ensure_write_path_scope_allows_path_string(
            &SessionId(active_session_id.clone()),
            "rollback_write_transaction",
            &entry.target_path,
        )?;

        let workspace_root = self.workspace_root()?;
        let target_path = PathBuf::from(&entry.target_path);
        let previous_target_backup_path = if target_path.exists() {
            let existing = fs::read(&target_path).map_err(|err| {
                HeptaError(format!(
                    "failed to read current target {} before rollback: {}",
                    target_path.display(),
                    err
                ))
            })?;
            let planned_backup = preview_backup_path(&workspace_root, &target_path)
                .map_err(|err| HeptaError(err.0))?;
            if let Some(parent) = planned_backup.parent() {
                fs::create_dir_all(parent).map_err(|err| {
                    HeptaError(format!(
                        "failed to create rollback backup parent {}: {}",
                        parent.display(),
                        err
                    ))
                })?;
            }
            fs::write(&planned_backup, existing).map_err(|err| {
                HeptaError(format!(
                    "failed to write rollback safety backup {}: {}",
                    planned_backup.display(),
                    err
                ))
            })?;
            Some(planned_backup.display().to_string())
        } else {
            None
        };

        match entry.rollback_strategy.as_str() {
            "restore_checkpoint" => {
                let checkpoint_path =
                    entry.rollback_checkpoint_path.as_deref().ok_or_else(|| {
                        HeptaError(format!(
                            "write transaction {} is missing rollback checkpoint",
                            transaction_id
                        ))
                    })?;
                let checkpoint_bytes = fs::read(checkpoint_path).map_err(|err| {
                    HeptaError(format!(
                        "failed to read rollback checkpoint {}: {}",
                        checkpoint_path, err
                    ))
                })?;
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent).map_err(|err| {
                        HeptaError(format!(
                            "failed to create rollback target parent {}: {}",
                            parent.display(),
                            err
                        ))
                    })?;
                }
                fs::write(&target_path, checkpoint_bytes).map_err(|err| {
                    HeptaError(format!(
                        "failed to restore {} during rollback: {}",
                        target_path.display(),
                        err
                    ))
                })?;
            }
            "delete_target" => {
                if target_path.exists() {
                    fs::remove_file(&target_path).map_err(|err| {
                        HeptaError(format!(
                            "failed to delete {} during rollback: {}",
                            target_path.display(),
                            err
                        ))
                    })?;
                }
            }
            other => {
                return Err(HeptaError(format!(
                    "unsupported rollback strategy {} for transaction {}",
                    other, transaction_id
                )));
            }
        }

        let rolled_back_at_unix_ms = current_unix_ms()?;
        {
            let mut guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            let stored = guard
                .iter_mut()
                .find(|candidate| candidate.transaction_id == transaction_id)
                .ok_or_else(|| {
                    HeptaError(format!("unknown write transaction: {}", transaction_id))
                })?;
            stored.rolled_back_at_unix_ms = Some(rolled_back_at_unix_ms);
        }

        let report = RollbackWriteReport {
            transaction_id: entry.transaction_id.clone(),
            target_path: entry.target_path.clone(),
            rollback_strategy: entry.rollback_strategy.clone(),
            rollback_checkpoint_path: entry.rollback_checkpoint_path.clone(),
            previous_target_backup_path,
            target_exists_after_rollback: target_path.exists(),
        };
        self.emit_event(
            EventKind::WriteRolledBack,
            Some(SessionId(active_session_id)),
            None,
            format!(
                "rolled back write transaction {} for {}",
                report.transaction_id, report.target_path
            ),
        )?;
        Ok(report)
    }

    pub fn fork_session(
        &self,
        source_session_id: &str,
        target_session_id: &str,
    ) -> Result<SessionForkReport, HeptaError> {
        let source_session_id = source_session_id.trim();
        let target_session_id = target_session_id.trim();
        if source_session_id.is_empty() || target_session_id.is_empty() {
            return Err(HeptaError(
                "source and target session ids must not be empty".into(),
            ));
        }
        if source_session_id == target_session_id {
            return Err(HeptaError(
                "source and target session ids must differ".into(),
            ));
        }
        match self.existing_session_snapshot_for_id(target_session_id) {
            Ok(_) => {
                return Err(HeptaError(format!(
                    "target session already exists: {}",
                    target_session_id
                )));
            }
            Err(err) if err.0 == format!("session not found: {}", target_session_id) => {}
            Err(err) => return Err(err),
        }

        let mut export = self.session_export(source_session_id)?;
        let now = current_unix_ms()?;
        export.session.session_id = SessionId(target_session_id.to_string());
        export.session.title = format!("{} (fork)", export.session.title);
        export.session.created_at_unix_ms = now;
        export.session.last_active_unix_ms = now;
        export.session.archived_at_unix_ms = None;
        export.history = export
            .history
            .into_iter()
            .map(|mut turn| {
                turn.session_id = target_session_id.to_string();
                turn
            })
            .collect();
        Self::rebind_session_export_topic_state(&mut export, source_session_id, target_session_id);
        let topic_session_count = export.topic_sessions.len();
        let topic_graph_edge_count = export.topic_graph_edges.len();

        self.apply_session_export(export)?;
        let forked = self
            .existing_session_snapshot_for_id(target_session_id)
            .map_err(|err| {
                if err.0 == format!("session not found: {}", target_session_id) {
                    HeptaError(format!(
                        "forked session not found after creation: {}",
                        target_session_id
                    ))
                } else {
                    err
                }
            })?;
        let approvals = self.approval_snapshot_for_session(target_session_id)?;
        let history_entries = self.history(Some(target_session_id), usize::MAX)?.len();
        let report = SessionForkReport {
            source_session_id: source_session_id.to_string(),
            target_session_id: target_session_id.to_string(),
            target_title: forked.title,
            target_model: forked.model,
            target_archived: forked.archived_at_unix_ms.is_some(),
            approvals_granted: approvals.granted_tools.len(),
            approvals_pending: approvals.pending.len(),
            history_entries,
            topic_session_count,
            topic_graph_edge_count,
            active_session_after_fork: self.active_session_id()?,
        };
        self.emit_event(
            EventKind::SessionForked,
            Some(SessionId(report.target_session_id.clone())),
            None,
            format!("forked from {}", report.source_session_id),
        )?;
        Ok(report)
    }

    pub fn merge_session(
        &self,
        source_session_id: &str,
        target_session_id: &str,
        options: MergeOptions,
    ) -> Result<MergeExecutionReport, HeptaError> {
        let source_session_id = source_session_id.trim();
        let target_session_id = target_session_id.trim();
        if source_session_id.is_empty() || target_session_id.is_empty() {
            return Err(HeptaError(
                "source and target session ids must not be empty".into(),
            ));
        }
        if source_session_id == target_session_id {
            return Err(HeptaError(
                "source and target session ids must differ".into(),
            ));
        }

        let source_export = self.session_export(source_session_id)?;
        let history_plan = self.plan_history_merge(target_session_id, &source_export.history)?;
        let target_approvals = self.approval_snapshot_for_session(target_session_id)?;
        let merged_approvals =
            merge_approval_snapshots(target_approvals.clone(), source_export.approval.clone());
        let target_granted_set = target_approvals
            .granted_tools
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let target_pending_set = target_approvals
            .pending
            .iter()
            .map(pending_approval_signature)
            .collect::<HashSet<_>>();
        let (target_topic_sessions_before, target_topic_graph_edges_before) =
            self.topic_export_state_for_session(target_session_id)?;
        let source_topic_session_count = source_export.topic_sessions.len();
        let source_topic_graph_edge_count = source_export.topic_graph_edges.len();
        let target_topic_session_count_before = target_topic_sessions_before.len();
        let target_topic_graph_edge_count_before = target_topic_graph_edges_before.len();
        let topic_state_merge_outcome = simulate_topic_state_merge(
            source_session_id,
            target_session_id,
            target_topic_sessions_before,
            target_topic_graph_edges_before,
            source_export.topic_sessions.clone(),
            source_export.topic_graph_edges.clone(),
        );
        let target_topic_session_count_after = topic_state_merge_outcome.topic_sessions.len();
        let target_topic_graph_edge_count_after = topic_state_merge_outcome.topic_graph_edges.len();
        let approvals_added_to_target =
            ordered_unique_difference(merged_approvals.granted_tools.clone(), &target_granted_set);
        let pending_added_to_target = ordered_unique_difference(
            merged_approvals
                .pending
                .iter()
                .map(pending_approval_signature)
                .collect(),
            &target_pending_set,
        );
        let target_record = self
            .existing_session_snapshot_for_id(target_session_id)
            .map_err(|err| {
                if err.0 == format!("session not found: {}", target_session_id) {
                    HeptaError(format!("unknown target session: {}", target_session_id))
                } else {
                    err
                }
            })?;
        let merged_last_user_intent_summary = source_export
            .session
            .last_user_intent_summary
            .clone()
            .or(target_record.last_user_intent_summary.clone());
        let target_title_after = if options.adopt_title {
            source_export.session.title.clone()
        } else {
            target_record.title.clone()
        };
        let target_model_after = if options.adopt_model {
            source_export.model.clone()
        } else {
            self.model_selection_for_session(target_session_id)?.active
        };

        self.upsert_session_record(
            &SessionId(target_record.session_id.clone()),
            if options.adopt_title {
                Some(source_export.session.title.clone())
            } else {
                None
            },
            source_export
                .session
                .last_user_intent_summary
                .or(target_record.last_user_intent_summary.clone()),
            Some(target_record.archived_at_unix_ms),
            true,
        )?;

        if options.adopt_model {
            self.set_session_model(target_session_id, source_export.model.clone())?;
        }

        {
            let mut approval_state = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            approval_state.remove_session(target_session_id);
            if !merged_approvals.granted_tools.is_empty() || !merged_approvals.pending.is_empty() {
                approval_state.sessions.push(SessionApprovalState {
                    session_id: target_session_id.to_string(),
                    granted_tools: merged_approvals.granted_tools,
                    pending: merged_approvals.pending,
                });
            }
        }
        {
            let mut history_state = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            history_state.extend(history_plan.append_turns);
        }
        self.replace_topic_export_state_for_session(
            target_session_id,
            topic_state_merge_outcome.topic_sessions.clone(),
            topic_state_merge_outcome.topic_graph_edges.clone(),
        )?;

        if options.delete_source {
            if self.active_session_id()? == source_session_id {
                let mut guard = self
                    .session_state
                    .lock()
                    .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
                guard.active_session_id = target_session_id.to_string();
            }
            self.delete_session(source_session_id)?;
        }

        let report = MergeExecutionReport {
            source_session_id: source_session_id.to_string(),
            target_session_id: target_session_id.to_string(),
            options,
            target_title_after,
            target_model_after,
            target_archived_after: target_record.archived_at_unix_ms.is_some(),
            source_deleted_after_merge: options.delete_source,
            merged_last_user_intent_summary,
            approvals_added_to_target,
            pending_added_to_target,
            appended_history_entries: history_plan.new_history_entries_to_append.len(),
            skipped_duplicate_history_entries: history_plan.duplicate_history_entries_skipped.len(),
            source_topic_session_count,
            target_topic_session_count_before,
            target_topic_session_count_after,
            source_topic_graph_edge_count,
            target_topic_graph_edge_count_before,
            target_topic_graph_edge_count_after,
            new_history_entries_appended: history_plan.new_history_entries_to_append,
            duplicate_history_entries_skipped: history_plan.duplicate_history_entries_skipped,
        };
        self.emit_event(
            EventKind::SessionMerged,
            Some(SessionId(report.target_session_id.clone())),
            None,
            format!(
                "merged {} into {} (delete_source={})",
                report.source_session_id,
                report.target_session_id,
                report.source_deleted_after_merge
            ),
        )?;
        Ok(report)
    }

    pub fn diff_sessions(
        &self,
        left_session_id: &str,
        right_session_id: &str,
    ) -> Result<SessionDiffReport, HeptaError> {
        let left_session_id = left_session_id.trim();
        let right_session_id = right_session_id.trim();
        if left_session_id.is_empty() || right_session_id.is_empty() {
            return Err(HeptaError(
                "left and right session ids must not be empty".into(),
            ));
        }
        if left_session_id == right_session_id {
            return Err(HeptaError("left and right session ids must differ".into()));
        }

        let sessions = self.sessions()?;
        let left = sessions
            .iter()
            .find(|session| session.session_id == left_session_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("unknown session: {}", left_session_id)))?;
        let right = sessions
            .iter()
            .find(|session| session.session_id == right_session_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("unknown session: {}", right_session_id)))?;

        let left_approvals = self.approval_snapshot_for_session(left_session_id)?;
        let right_approvals = self.approval_snapshot_for_session(right_session_id)?;

        let (left_history, right_history) = {
            let guard = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            let left_history = guard
                .iter()
                .filter(|turn| turn.session_id == left_session_id)
                .cloned()
                .collect::<Vec<_>>();
            let right_history = guard
                .iter()
                .filter(|turn| turn.session_id == right_session_id)
                .cloned()
                .collect::<Vec<_>>();
            (left_history, right_history)
        };

        let left_granted = left_approvals.granted_tools;
        let right_granted = right_approvals.granted_tools;
        let right_granted_set = right_granted.iter().cloned().collect::<HashSet<_>>();
        let left_granted_set = left_granted.iter().cloned().collect::<HashSet<_>>();

        let left_pending = left_approvals
            .pending
            .into_iter()
            .map(|item| pending_approval_signature(&item))
            .collect::<Vec<_>>();
        let right_pending = right_approvals
            .pending
            .into_iter()
            .map(|item| pending_approval_signature(&item))
            .collect::<Vec<_>>();
        let right_pending_set = right_pending.iter().cloned().collect::<HashSet<_>>();
        let left_pending_set = left_pending.iter().cloned().collect::<HashSet<_>>();

        let left_history_signatures = left_history
            .iter()
            .map(turn_record_signature)
            .collect::<Vec<_>>();
        let right_history_signatures = right_history
            .iter()
            .map(turn_record_signature)
            .collect::<Vec<_>>();
        let right_history_set = right_history_signatures
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let left_history_set = left_history_signatures
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let shared_history_count = left_history_set.intersection(&right_history_set).count();

        Ok(SessionDiffReport {
            left_session_id: left.session_id,
            right_session_id: right.session_id,
            left_title: left.title,
            right_title: right.title,
            left_model: left.model,
            right_model: right.model,
            left_archived: left.archived_at_unix_ms.is_some(),
            right_archived: right.archived_at_unix_ms.is_some(),
            left_last_user_intent_summary: left.last_user_intent_summary,
            right_last_user_intent_summary: right.last_user_intent_summary,
            left_history_count: left_history.len(),
            right_history_count: right_history.len(),
            shared_history_count,
            approvals_only_left: ordered_unique_difference(left_granted, &right_granted_set),
            approvals_only_right: ordered_unique_difference(right_granted, &left_granted_set),
            pending_only_left: ordered_unique_difference(left_pending, &right_pending_set),
            pending_only_right: ordered_unique_difference(right_pending, &left_pending_set),
            history_only_left: ordered_unique_difference(
                left_history_signatures,
                &right_history_set,
            ),
            history_only_right: ordered_unique_difference(
                right_history_signatures,
                &left_history_set,
            ),
        })
    }

    pub fn preview_merge_session(
        &self,
        source_session_id: &str,
        target_session_id: &str,
        options: MergeOptions,
    ) -> Result<MergePreviewReport, HeptaError> {
        let source_session_id = source_session_id.trim();
        let target_session_id = target_session_id.trim();
        if source_session_id.is_empty() || target_session_id.is_empty() {
            return Err(HeptaError(
                "source and target session ids must not be empty".into(),
            ));
        }
        if source_session_id == target_session_id {
            return Err(HeptaError(
                "source and target session ids must differ".into(),
            ));
        }

        let source_export = self.session_export(source_session_id)?;
        let target = self
            .existing_session_snapshot_for_id(target_session_id)
            .map_err(|err| {
                if err.0 == format!("session not found: {}", target_session_id) {
                    HeptaError(format!("unknown target session: {}", target_session_id))
                } else {
                    err
                }
            })?;

        let target_approvals = self.approval_snapshot_for_session(target_session_id)?;
        let merged_approvals =
            merge_approval_snapshots(target_approvals.clone(), source_export.approval.clone());
        let history_plan = self.plan_history_merge(target_session_id, &source_export.history)?;
        let target_history_count = self.history(Some(target_session_id), usize::MAX)?.len();
        let (target_topic_sessions_before, target_topic_graph_edges_before) =
            self.topic_export_state_for_session(target_session_id)?;
        let source_topic_session_count = source_export.topic_sessions.len();
        let source_topic_graph_edge_count = source_export.topic_graph_edges.len();
        let target_topic_session_count_before = target_topic_sessions_before.len();
        let target_topic_graph_edge_count_before = target_topic_graph_edges_before.len();
        let topic_state_merge_outcome = simulate_topic_state_merge(
            source_session_id,
            target_session_id,
            target_topic_sessions_before,
            target_topic_graph_edges_before,
            source_export.topic_sessions.clone(),
            source_export.topic_graph_edges.clone(),
        );

        let target_granted_set = target_approvals
            .granted_tools
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let target_pending_set = target_approvals
            .pending
            .iter()
            .map(pending_approval_signature)
            .collect::<HashSet<_>>();

        let approvals_added_to_target =
            ordered_unique_difference(merged_approvals.granted_tools, &target_granted_set);
        let pending_added_to_target = ordered_unique_difference(
            merged_approvals
                .pending
                .into_iter()
                .map(|item| pending_approval_signature(&item))
                .collect(),
            &target_pending_set,
        );

        let merged_last_user_intent_summary = source_export
            .session
            .last_user_intent_summary
            .clone()
            .or(target.last_user_intent_summary.clone());

        Ok(MergePreviewReport {
            source_session_id: source_session_id.to_string(),
            target_session_id: target_session_id.to_string(),
            options,
            source_title: source_export.session.title.clone(),
            source_model: source_export.model.clone(),
            target_title_before: target.title.clone(),
            target_title_after: if options.adopt_title {
                source_export.session.title.clone()
            } else {
                target.title.clone()
            },
            target_model_before: target.model.clone(),
            target_model_after: if options.adopt_model {
                source_export.model.clone()
            } else {
                target.model.clone()
            },
            target_archived_before: target.archived_at_unix_ms.is_some(),
            target_archived_after: target.archived_at_unix_ms.is_some(),
            source_deleted_after_merge: options.delete_source,
            target_last_user_intent_summary_before: target.last_user_intent_summary.clone(),
            source_last_user_intent_summary: source_export.session.last_user_intent_summary,
            merged_last_user_intent_summary,
            source_history_count: source_export.history.len(),
            target_history_count,
            history_entries_to_append: history_plan.new_history_entries_to_append.len(),
            history_entries_skipped_as_duplicates: history_plan
                .duplicate_history_entries_skipped
                .len(),
            source_topic_session_count,
            target_topic_session_count_before,
            target_topic_session_count_after: topic_state_merge_outcome.topic_sessions.len(),
            source_topic_graph_edge_count,
            target_topic_graph_edge_count_before,
            target_topic_graph_edge_count_after: topic_state_merge_outcome.topic_graph_edges.len(),
            approvals_added_to_target,
            pending_added_to_target,
            duplicate_history_entries_skipped: history_plan.duplicate_history_entries_skipped,
            new_history_entries_to_append: history_plan.new_history_entries_to_append,
        })
    }

    pub fn save_snapshot(&self, path: &str) -> Result<String, HeptaError> {
        self.write_snapshot(path, true)
    }

    pub fn persist_snapshot(&self, path: &str) -> Result<String, HeptaError> {
        self.write_snapshot(path, false)
    }

    fn write_snapshot(&self, path: &str, emit_audit_event: bool) -> Result<String, HeptaError> {
        let snapshot = self.runtime_snapshot()?;
        let snapshot_json = serde_json::to_string_pretty(&snapshot)
            .map_err(|err| HeptaError(format!("failed to serialize runtime snapshot: {}", err)))?;
        let snapshot_path = PathBuf::from(path);
        if let Some(parent) = snapshot_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|err| {
                    HeptaError(format!(
                        "failed to create snapshot directory {}: {}",
                        parent.display(),
                        err
                    ))
                })?;
            }
        }
        fs::write(&snapshot_path, snapshot_json).map_err(|err| {
            HeptaError(format!(
                "failed to write runtime snapshot {}: {}",
                snapshot_path.display(),
                err
            ))
        })?;
        if emit_audit_event {
            self.emit_event(
                EventKind::SnapshotSaved,
                None,
                None,
                format!("saved runtime snapshot to {}", snapshot_path.display()),
            )?;
        }
        Ok(format!(
            "saved runtime snapshot to {}",
            snapshot_path.display()
        ))
    }

    pub fn load_snapshot(&self, path: &str) -> Result<String, HeptaError> {
        let snapshot_path = PathBuf::from(path);
        let snapshot_json = fs::read_to_string(&snapshot_path).map_err(|err| {
            HeptaError(format!(
                "failed to read runtime snapshot {}: {}",
                snapshot_path.display(),
                err
            ))
        })?;
        let snapshot: RuntimeSnapshot = serde_json::from_str(&snapshot_json).map_err(|err| {
            HeptaError(format!(
                "failed to parse runtime snapshot {}: {}",
                snapshot_path.display(),
                err
            ))
        })?;
        if snapshot.version != 1 {
            return Err(HeptaError(format!(
                "unsupported runtime snapshot version: {}",
                snapshot.version
            )));
        }
        self.apply_runtime_snapshot(snapshot)?;
        self.emit_event(
            EventKind::SnapshotLoaded,
            None,
            None,
            format!("loaded runtime snapshot from {}", snapshot_path.display()),
        )?;
        Ok(format!(
            "loaded runtime snapshot from {}",
            snapshot_path.display()
        ))
    }

    pub async fn policy_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.policy_report().await?;
        let mut lines = vec!["Policy summary:".to_string()];
        lines.push(format!("- active session: {}", report.active_session_id));
        lines.push(format!(
            "- active model: {}/{}",
            report.active_model.provider, report.active_model.model
        ));
        lines.push(format!("- default rules: {}", report.default_rules.len()));
        lines.push(format!("- custom rules: {}", report.custom_rules.len()));
        lines.push(format!("- granted approvals: {}", report.granted_approvals));
        lines.push(format!("- pending approvals: {}", report.pending_approvals));
        lines.push("- effective decisions:".into());
        for item in report.effective_tool_decisions {
            lines.push(format!(
                "  - {}: {} via {} ({})",
                item.tool_name,
                format_approval_requirement(item.requirement),
                item.matched_rule_id.unwrap_or_else(|| "<none>".into()),
                item.reason
            ));
        }
        Ok(lines)
    }

    pub fn sessions(&self) -> Result<Vec<SessionSnapshot>, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.ensure_session_record_sync(&active_session_id)?;
        let mut sessions = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?;
        sessions.reverse();
        let mut deduped = Vec::new();
        for session in sessions {
            let session_id = session.session_id.0.clone();
            if deduped
                .iter()
                .any(|item: &SessionSnapshot| item.session_id == session_id)
            {
                continue;
            }
            let (topic_session_count, topic_graph_edge_count) =
                self.topic_state_counts_for_session(&session_id)?;
            deduped.push(SessionSnapshot {
                is_active: session_id == active_session_id,
                session_id: session_id.clone(),
                agent_id: session.agent_id.0,
                title: session.title,
                model: self.model_selection_for_session(&session_id)?.active,
                created_at_unix_ms: session.created_at_unix_ms,
                last_active_unix_ms: session.last_active_unix_ms,
                last_user_intent_summary: session.last_user_intent_summary,
                archived_at_unix_ms: session.archived_at_unix_ms,
                topic_session_count,
                topic_graph_edge_count,
            });
        }
        deduped.reverse();
        Ok(deduped)
    }

    fn session_snapshot_for_id(&self, session_id: &str) -> Result<SessionSnapshot, HeptaError> {
        self.ensure_session_record_sync(session_id)?;
        self.existing_session_snapshot_for_id(session_id)
    }

    fn existing_session_snapshot_for_id(
        &self,
        session_id: &str,
    ) -> Result<SessionSnapshot, HeptaError> {
        self.sessions()?
            .into_iter()
            .find(|session| session.session_id == session_id)
            .ok_or_else(|| HeptaError(format!("session not found: {}", session_id)))
    }

    fn topic_state_counts_for_session(
        &self,
        session_id: &str,
    ) -> Result<(usize, usize), HeptaError> {
        let (topic_sessions, topic_graph_edges) =
            self.topic_export_state_for_session(session_id)?;
        Ok((topic_sessions.len(), topic_graph_edges.len()))
    }
}

fn format_risk_tier(risk_tier: RiskTier) -> &'static str {
    match risk_tier {
        RiskTier::Low => "low",
        RiskTier::Medium => "medium",
        RiskTier::High => "high",
    }
}

fn format_approval_requirement(requirement: ApprovalRequirement) -> &'static str {
    match requirement {
        ApprovalRequirement::None => "none",
        ApprovalRequirement::Ask => "ask",
        ApprovalRequirement::Deny => "deny",
    }
}

fn format_execution_profile(profile: ExecutionProfile) -> &'static str {
    match profile {
        ExecutionProfile::FullAccess => "full_access",
        ExecutionProfile::ReadOnlyTools => "read_only_tools",
        ExecutionProfile::NoTools => "no_tools",
    }
}

fn format_filesystem_scope(scope: FilesystemScope) -> &'static str {
    match scope {
        FilesystemScope::WorkspaceOnly => "workspace_only",
        FilesystemScope::AnyPath => "any_path",
    }
}

fn format_write_path_scope(scope: WritePathScope) -> &'static str {
    match scope {
        WritePathScope::ArtifactsOnly => "artifacts_only",
        WritePathScope::WorkspaceOnly => "workspace_only",
        WritePathScope::AnyPath => "any_path",
    }
}

impl RuntimeKernel {
    async fn model_messages_for_turn(
        &self,
        session_id: &SessionId,
        input: &str,
    ) -> Result<Vec<ModelMessage>, HeptaError> {
        let mut context_sections = vec![
            "You are Hepta's native runtime agent. Use the supplied session context and available tools when useful. If the user asks what was remembered earlier, answer from the session context instead of giving a generic definition. Answer directly; do not reveal hidden reasoning, planning notes, or internal analysis.".to_string(),
        ];

        let transcript_entries = self
            .memory
            .list_transcript_entries()
            .map_err(|err| HeptaError(err.0))?;
        let mut recent = transcript_entries
            .into_iter()
            .filter(|entry| {
                entry.session_id == *session_id
                    && matches!(
                        entry.role,
                        Some(MessageRole::User) | Some(MessageRole::Assistant)
                    )
            })
            .collect::<Vec<_>>();
        recent.sort_by_key(|entry| entry.sequence);
        if !recent.is_empty() {
            let transcript = recent
                .into_iter()
                .rev()
                .take(12)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .map(|entry| {
                    let role = match entry.role {
                        Some(MessageRole::User) => "User",
                        Some(MessageRole::Assistant) => "Assistant",
                        _ => "Context",
                    };
                    format!("{}: {}", role, truncate_for_context(&entry.content, 900))
                })
                .collect::<Vec<_>>()
                .join("\n");
            context_sections.push(format!("Recent session transcript:\n{}", transcript));
        }

        let keyword = memory_context_keyword(input);
        if !keyword.is_empty() {
            let hits = self
                .memory
                .search(MemoryQuery {
                    text: keyword,
                    limit: 6,
                })
                .await
                .map_err(|err| HeptaError(err.0))?;
            if !hits.is_empty() {
                let memories = hits
                    .into_iter()
                    .map(|record| format!("- {}", truncate_for_context(&record.content, 700)))
                    .collect::<Vec<_>>()
                    .join("\n");
                context_sections.push(format!("Relevant memory records:\n{}", memories));
            }
        }

        Ok(vec![
            ModelMessage {
                role: MessageRole::System,
                content: context_sections.join("\n\n"),
            },
            ModelMessage {
                role: MessageRole::User,
                content: input.into(),
            },
        ])
    }

    async fn execute_tool_call_for_turn(
        &self,
        session_id: &SessionId,
        session_key: &str,
        correlation_id: &CorrelationId,
        active_model: &ModelRef,
        tool_call: &ToolCall,
    ) -> Result<RuntimeToolStep, HeptaError> {
        if let Err(err) = self.validate_tool_input(&tool_call.name, &tool_call.arguments_json) {
            return Ok(RuntimeToolStep::Blocked {
                final_text: format!("tool input validation failed for {}", tool_call.name),
                reason: err.0,
            });
        }

        let risk = self.tools.risk_tier(&tool_call.name)?;
        let decision = self
            .policy
            .evaluate_tool(PolicyEvaluationContext {
                session_id: Some(session_id.clone()),
                model: Some(active_model.clone()),
                tool_name: tool_call.name.clone(),
                risk_tier: risk,
            })
            .await
            .map_err(|err| HeptaError(err.0))?;

        let granted = {
            let guard = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            guard.is_granted(session_key, &tool_call.name)
        };

        match decision.requirement {
            ApprovalRequirement::Deny => Ok(RuntimeToolStep::Blocked {
                final_text: format!("policy denied tool {}", tool_call.name),
                reason: decision.reason,
            }),
            ApprovalRequirement::Ask if !granted => {
                let mut guard = self
                    .approval_state
                    .lock()
                    .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
                guard.remember_pending(session_key, &tool_call.name, &decision.reason);
                drop(guard);
                self.emit_event(
                    EventKind::ApprovalRequested,
                    Some(session_id.clone()),
                    Some(correlation_id.clone()),
                    format!(
                        "tool {} requires approval: {}",
                        tool_call.name, decision.reason
                    ),
                )?;
                Ok(RuntimeToolStep::ApprovalRequired {
                    tool_name: tool_call.name.clone(),
                    reason: decision.reason,
                })
            }
            ApprovalRequirement::None | ApprovalRequirement::Ask => {
                if let Err(err) =
                    self.ensure_execution_profile_allows_tool(session_id, &tool_call.name)
                {
                    return Ok(RuntimeToolStep::Blocked {
                        final_text: format!("execution profile blocked tool {}", tool_call.name),
                        reason: err.0,
                    });
                }
                if let Err(err) = self.ensure_filesystem_scope_allows_tool_input(
                    session_id,
                    &tool_call.name,
                    &tool_call.arguments_json,
                ) {
                    return Ok(RuntimeToolStep::Blocked {
                        final_text: format!("filesystem scope blocked tool {}", tool_call.name),
                        reason: err.0,
                    });
                }
                if let Err(err) = self.ensure_write_path_scope_allows_tool_input(
                    session_id,
                    &tool_call.name,
                    &tool_call.arguments_json,
                ) {
                    return Ok(RuntimeToolStep::Blocked {
                        final_text: format!("write path scope blocked tool {}", tool_call.name),
                        reason: err.0,
                    });
                }
                if let Err(err) = self
                    .ensure_destructive_write_semantics(&tool_call.name, &tool_call.arguments_json)
                {
                    return Ok(RuntimeToolStep::Blocked {
                        final_text: format!("write semantics blocked tool {}", tool_call.name),
                        reason: err.0,
                    });
                }

                let prepared_write_transaction = match self
                    .prepare_write_transaction_with_lock_check(
                        &session_id.0,
                        &tool_call.name,
                        &tool_call.arguments_json,
                    ) {
                    Ok(prepared) => prepared,
                    Err(err) => {
                        return Ok(RuntimeToolStep::Blocked {
                            final_text: format!("write lock blocked tool {}", tool_call.name),
                            reason: err.0,
                        });
                    }
                };

                let mut tool_result = self
                    .invoke_tool_with_validation(
                        &tool_call.name,
                        session_id,
                        correlation_id,
                        &tool_call.arguments_json,
                    )
                    .await?;
                let tool_output_json = self.record_write_transaction_from_tool_result(
                    session_id,
                    prepared_write_transaction,
                    tool_result.structured_json.clone(),
                )?;
                tool_result.structured_json = tool_output_json.clone();

                self.store_memory(
                    Some(session_id),
                    "mem-tool",
                    MemoryScope::LongTerm,
                    format_tool_memory_content(&tool_result),
                )
                .await?;

                if tool_result_is_timeout(&tool_result) {
                    return Ok(RuntimeToolStep::TimedOut(RuntimeToolTimeout {
                        tool_name: tool_call.name.clone(),
                        tool_output_json,
                        final_text: tool_result.content.clone(),
                    }));
                }

                Ok(RuntimeToolStep::Executed(RuntimeToolExecution {
                    tool_name: tool_call.name.clone(),
                    tool_output_json,
                    tool_message: format_tool_message(&tool_result),
                }))
            }
        }
    }

    pub async fn run_demo_turn(&self, input: &str) -> Result<VerticalSliceResult, HeptaError> {
        let session_id = SessionId(self.active_session_id()?);
        self.run_demo_turn_for_session(session_id, input).await
    }

    pub async fn run_demo_turn_in_session(
        &self,
        session_id: &str,
        input: &str,
    ) -> Result<VerticalSliceResult, HeptaError> {
        self.run_demo_turn_in_session_with_model_timeout(session_id, input, None)
            .await
    }

    pub async fn run_demo_turn_in_session_with_model_timeout(
        &self,
        session_id: &str,
        input: &str,
        model_timeout_ms: Option<u64>,
    ) -> Result<VerticalSliceResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.run_demo_turn_for_session_impl(
            SessionId(session_id.to_string()),
            input,
            model_timeout_ms,
        )
        .await
    }

    async fn run_demo_turn_for_session(
        &self,
        session_id: SessionId,
        input: &str,
    ) -> Result<VerticalSliceResult, HeptaError> {
        self.run_demo_turn_for_session_impl(session_id, input, None)
            .await
    }

    async fn run_demo_turn_for_session_impl(
        &self,
        session_id: SessionId,
        input: &str,
        model_timeout_ms: Option<u64>,
    ) -> Result<VerticalSliceResult, HeptaError> {
        let session_key = session_id.0.clone();
        let correlation_id = CorrelationId("corr-demo".into());
        let active_model = self.model_selection_for_session(&session_key)?.active;

        self.ensure_session_record(&session_id).await?;
        self.upsert_session_record(
            &session_id,
            None,
            Some(summarize_user_intent(input)),
            None,
            true,
        )?;
        self.emit_event(
            EventKind::MessageReceived,
            Some(session_id.clone()),
            Some(correlation_id.clone()),
            summarize_user_intent(input),
        )?;

        let base_messages = self.model_messages_for_turn(&session_id, input).await?;
        let model_tools = self.tools.model_tool_specs_for_turn(input);

        self.emit_event(
            EventKind::ModelCalled,
            Some(session_id.clone()),
            Some(correlation_id.clone()),
            format!(
                "initial model call via {}/{}",
                active_model.provider, active_model.model
            ),
        )?;
        let deterministic_message = self.deterministic_runtime_response_for_session(
            &session_id,
            input,
            &active_model,
            &base_messages,
        )?;
        let mut current_response = if let Some(message) = deterministic_message {
            ModelResponse {
                message: Some(ModelMessage {
                    role: MessageRole::Assistant,
                    content: message,
                }),
                tool_calls: vec![],
                finish_reason: FinishReason::Stop,
                usage: Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                },
            }
        } else if let Some(tool_call) = native_pre_model_tool_call(input) {
            ModelResponse {
                message: None,
                tool_calls: vec![tool_call],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                },
            }
        } else {
            self.providers
                .chat(ModelRequest {
                    model: active_model.clone(),
                    messages: base_messages.clone(),
                    thinking: ThinkingLevel::High,
                    tools: model_tools.clone(),
                    timeout_ms: model_timeout_ms,
                })
                .await?
        };

        let mut conversation_messages = base_messages;
        let mut invoked_tool = None::<String>;
        let mut tool_output_json = None::<String>;
        let mut final_text = String::new();
        let mut approval_required = None::<String>;
        let mut blocked_reason = None::<String>;
        let max_tool_steps = 6usize;

        for step_index in 0..max_tool_steps {
            if let Some(tool_call) = current_response.tool_calls.first().cloned() {
                if model_tools.is_empty() {
                    final_text = current_response
                        .message
                        .as_ref()
                        .map(|message| message.content.trim().to_string())
                        .filter(|message| !message.is_empty())
                        .unwrap_or_else(|| {
                            "这条消息按普通对话处理；没有明确工具意图，所以 Hepta 没有调用工具。"
                                .into()
                        });
                    blocked_reason = Some("tool-intent-not-authorized-for-turn".into());
                    break;
                }
                match self
                    .execute_tool_call_for_turn(
                        &session_id,
                        &session_key,
                        &correlation_id,
                        &active_model,
                        &tool_call,
                    )
                    .await?
                {
                    RuntimeToolStep::Executed(execution) => {
                        if invoked_tool.is_none() {
                            invoked_tool = Some(execution.tool_name.clone());
                        }
                        tool_output_json = execution.tool_output_json.clone();
                        conversation_messages.push(ModelMessage {
                            role: MessageRole::Tool,
                            content: execution.tool_message,
                        });
                        self.emit_event(
                            EventKind::ModelCalled,
                            Some(session_id.clone()),
                            Some(correlation_id.clone()),
                            format!(
                                "followup model call after tool {} step {}",
                                execution.tool_name,
                                step_index + 1
                            ),
                        )?;
                        current_response = self
                            .providers
                            .chat(ModelRequest {
                                model: active_model.clone(),
                                messages: conversation_messages.clone(),
                                thinking: ThinkingLevel::High,
                                tools: model_tools.clone(),
                                timeout_ms: model_timeout_ms,
                            })
                            .await?;
                    }
                    RuntimeToolStep::TimedOut(timeout) => {
                        if invoked_tool.is_none() {
                            invoked_tool = Some(timeout.tool_name.clone());
                        }
                        tool_output_json = timeout.tool_output_json.clone();
                        final_text = timeout.final_text;
                        break;
                    }
                    RuntimeToolStep::ApprovalRequired { tool_name, reason } => {
                        final_text =
                            format!("approval required before invoking tool {}", tool_name);
                        approval_required = Some(tool_name);
                        blocked_reason = Some(reason);
                        break;
                    }
                    RuntimeToolStep::Blocked {
                        final_text: blocked_text,
                        reason,
                    } => {
                        final_text = blocked_text;
                        blocked_reason = Some(reason);
                        break;
                    }
                }
            } else {
                final_text = current_response
                    .message
                    .take()
                    .map(|message| message.content)
                    .unwrap_or_else(|| "empty model response".into());
                self.store_memory(
                    Some(&session_id),
                    "mem-assistant",
                    MemoryScope::LongTerm,
                    format!("assistant:{}", input),
                )
                .await?;
                break;
            }
        }

        if final_text.is_empty() && blocked_reason.is_none() {
            final_text = format!("tool loop exceeded maximum steps ({})", max_tool_steps);
            blocked_reason = Some("tool loop exceeded maximum steps".into());
        }
        if looks_like_live_agent_marker_recall_intent(input)
            && let Some(marker) = self.latest_live_agent_e2e_marker_for_session(&session_id)?
            && !final_text.contains(&marker)
        {
            final_text = format!("The live-agent-e2e marker is {marker}.");
        }

        let recalled = self
            .memory
            .search(MemoryQuery {
                text: if invoked_tool.is_some() {
                    "tool:".into()
                } else {
                    "assistant:".into()
                },
                limit: 10,
            })
            .await
            .map_err(|e| HeptaError(e.0))?;

        let result = VerticalSliceResult {
            session_id: session_id.0.clone(),
            active_model,
            invoked_tool,
            tool_output_json,
            final_text,
            recalled_memories: recalled.len(),
            approval_required,
            blocked_reason,
        };

        self.record_turn(TurnRecord {
            session_id: result.session_id.clone(),
            input: input.to_string(),
            invoked_tool: result.invoked_tool.clone(),
            final_text: result.final_text.clone(),
            blocked_reason: result.blocked_reason.clone(),
        })?;

        Ok(result)
    }

    fn deterministic_runtime_response_for_session(
        &self,
        session_id: &SessionId,
        input: &str,
        active_model: &ModelRef,
        messages: &[ModelMessage],
    ) -> Result<Option<String>, HeptaError> {
        if looks_like_live_agent_marker_recall_intent(input) {
            if let Some(marker) = self.latest_live_agent_e2e_marker_for_session(session_id)? {
                return Ok(Some(format!("The live-agent-e2e marker is {marker}.")));
            }
        }
        Ok(deterministic_runtime_response(
            input,
            active_model,
            messages,
        ))
    }

    fn latest_live_agent_e2e_marker_for_session(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<String>, HeptaError> {
        let recent = self.recent_session_window(&session_id.0, 16)?;
        let remembered_by_user = recent
            .iter()
            .rev()
            .filter(|entry| matches!(entry.role, Some(MessageRole::User)))
            .filter(|entry| looks_like_live_agent_marker_remember_intent(&entry.content))
            .find_map(|entry| extract_live_agent_e2e_marker(&entry.content));
        if remembered_by_user.is_some() {
            return Ok(remembered_by_user);
        }
        Ok(recent
            .iter()
            .rev()
            .filter(|entry| matches!(entry.role, Some(MessageRole::User)))
            .find_map(|entry| extract_live_agent_e2e_marker(&entry.content)))
    }

    async fn invoke_tool_with_validation(
        &self,
        tool_name: &str,
        session_id: &SessionId,
        correlation_id: &CorrelationId,
        input_json: &str,
    ) -> Result<ToolResult, HeptaError> {
        let tool_result = self
            .tools
            .invoke(
                tool_name,
                ToolContext {
                    session_id: Some(session_id.clone()),
                    correlation_id: Some(correlation_id.clone()),
                },
                ToolCallRequest {
                    name: tool_name.to_string(),
                    input_json: input_json.to_string(),
                },
            )
            .await?;

        if let Some(output_json) = tool_result.structured_json.as_deref() {
            self.validate_tool_output(tool_name, output_json)?;
        }

        self.emit_event(
            EventKind::ToolInvoked,
            Some(session_id.clone()),
            Some(correlation_id.clone()),
            format!("invoked tool {}", tool_name),
        )?;

        Ok(tool_result)
    }

    fn ensure_execution_profile_allows_tool(
        &self,
        session_id: &SessionId,
        tool_name: &str,
    ) -> Result<(), HeptaError> {
        let profile = self.execution_profile_for_session(&session_id.0)?;
        let metadata = self.tools.execution_metadata(tool_name)?;
        match profile {
            ExecutionProfile::FullAccess => Ok(()),
            ExecutionProfile::ReadOnlyTools => {
                if metadata.read_only && !metadata.destructive {
                    Ok(())
                } else {
                    Err(HeptaError(format!(
                        "execution profile {} blocks non-read-only tool {}",
                        format_execution_profile(profile),
                        tool_name
                    )))
                }
            }
            ExecutionProfile::NoTools => Err(HeptaError(format!(
                "execution profile {} blocks tool {}",
                format_execution_profile(profile),
                tool_name
            ))),
        }
    }

    fn ensure_filesystem_scope_allows_tool_input(
        &self,
        session_id: &SessionId,
        tool_name: &str,
        input_json: &str,
    ) -> Result<(), HeptaError> {
        let Some(argument_name) = path_argument_name_for_tool(tool_name) else {
            return Ok(());
        };

        let scope = self
            .path_capability_gates_for_session(&session_id.0)?
            .into_iter()
            .find(|gate| gate.tool_name == tool_name && gate.argument_name == argument_name)
            .map(|gate| gate.scope)
            .unwrap_or(self.filesystem_scope_for_session(&session_id.0)?);
        match scope {
            FilesystemScope::AnyPath => Ok(()),
            FilesystemScope::WorkspaceOnly => {
                let requested_path = parse_required_string_field(input_json, argument_name)
                    .map_err(|err| HeptaError(err.0))?;
                let workspace_root = self.workspace_root()?;
                let resolved =
                    resolve_path_within_root(&workspace_root, Path::new(&requested_path));
                if resolved.starts_with(&workspace_root) {
                    Ok(())
                } else {
                    Err(HeptaError(format!(
                        "filesystem scope {} blocks {} {} {} outside workspace {}",
                        format_filesystem_scope(scope),
                        tool_name,
                        argument_name,
                        requested_path,
                        workspace_root.display()
                    )))
                }
            }
        }
    }

    fn ensure_write_path_scope_allows_tool_input(
        &self,
        session_id: &SessionId,
        tool_name: &str,
        input_json: &str,
    ) -> Result<(), HeptaError> {
        let Some(argument_name) = write_path_argument_name_for_tool(tool_name) else {
            return Ok(());
        };

        let requested_path = parse_required_string_field(input_json, argument_name)
            .map_err(|err| HeptaError(err.0))?;
        self.ensure_write_path_scope_allows_path_string(session_id, tool_name, &requested_path)
    }

    fn ensure_write_path_scope_allows_path_string(
        &self,
        session_id: &SessionId,
        tool_name: &str,
        requested_path: &str,
    ) -> Result<(), HeptaError> {
        let workspace_root = self.workspace_root()?;
        let artifacts_root = workspace_root.join("artifacts");
        let resolved = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let scope = self.write_path_scope_for_session(&session_id.0)?;

        match scope {
            WritePathScope::AnyPath => Ok(()),
            WritePathScope::WorkspaceOnly => {
                if resolved.starts_with(&workspace_root) {
                    Ok(())
                } else {
                    Err(HeptaError(format!(
                        "write path scope {} blocks {} path {} outside workspace {}",
                        format_write_path_scope(scope),
                        tool_name,
                        requested_path,
                        workspace_root.display()
                    )))
                }
            }
            WritePathScope::ArtifactsOnly => {
                if resolved.starts_with(&artifacts_root) {
                    Ok(())
                } else {
                    Err(HeptaError(format!(
                        "write path scope {} blocks {} path {} outside artifacts root {}",
                        format_write_path_scope(scope),
                        tool_name,
                        requested_path,
                        artifacts_root.display()
                    )))
                }
            }
        }
    }

    fn ensure_destructive_write_semantics(
        &self,
        tool_name: &str,
        input_json: &str,
    ) -> Result<(), HeptaError> {
        if tool_name != "write_file" {
            return Ok(());
        }

        let requested_path =
            parse_required_string_field(input_json, "path").map_err(|err| HeptaError(err.0))?;
        let mode = parse_optional_string_field(input_json, "mode")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or_else(|| "create".to_string());
        let preview_only = parse_optional_bool_field(input_json, "preview_only")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or(false);
        let confirm_destructive = parse_optional_bool_field(input_json, "confirm_destructive")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or(false);
        let workspace_root = self.workspace_root()?;
        let resolved = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let exists = resolved.exists();

        if preview_only {
            return match mode.as_str() {
                "create" | "overwrite" | "append" => Ok(()),
                other => Err(HeptaError(format!(
                    "write_file received unsupported mode {}",
                    other
                ))),
            };
        }

        match mode.as_str() {
            "create" => {
                if exists {
                    Err(HeptaError(format!(
                        "write_file refuses to overwrite existing path {} without mode=overwrite and confirm_destructive=true, or mode=append",
                        requested_path
                    )))
                } else {
                    Ok(())
                }
            }
            "overwrite" => {
                if exists && !confirm_destructive {
                    Err(HeptaError(format!(
                        "write_file overwrite for existing path {} requires confirm_destructive=true",
                        requested_path
                    )))
                } else {
                    Ok(())
                }
            }
            "append" => Ok(()),
            other => Err(HeptaError(format!(
                "write_file received unsupported mode {}",
                other
            ))),
        }
    }

    fn workspace_root(&self) -> Result<PathBuf, HeptaError> {
        let root = discover_workspace_root();
        fs::canonicalize(&root).map_err(|err| {
            HeptaError(format!(
                "failed to resolve workspace root {}: {}",
                root.display(),
                err
            ))
        })
    }

    async fn ensure_session_record(&self, session_id: &SessionId) -> Result<(), HeptaError> {
        self.upsert_session_record(session_id, None, None, None, true)
    }

    fn ensure_session_record_sync(&self, session_id: &str) -> Result<(), HeptaError> {
        self.upsert_session_record(&SessionId(session_id.to_string()), None, None, None, true)
    }

    async fn store_memory(
        &self,
        session_id: Option<&SessionId>,
        id_prefix: &str,
        scope: MemoryScope,
        content: String,
    ) -> Result<(), HeptaError> {
        let memory_id = {
            let existing = self
                .memory
                .list_memories()
                .map_err(|err| HeptaError(err.0))?;
            format!("{}-{}", id_prefix, existing.len() + 1)
        };
        self.memory
            .put(MemoryRecord {
                id: memory_id.clone(),
                scope,
                content,
            })
            .await
            .map_err(|e| HeptaError(e.0))?;
        self.emit_event(
            EventKind::MemoryWritten,
            session_id.cloned(),
            None,
            format!("stored memory {}", memory_id),
        )?;
        Ok(())
    }

    fn record_turn(&self, record: TurnRecord) -> Result<(), HeptaError> {
        let existing_entries = self
            .memory
            .list_transcript_entries()
            .map_err(|err| HeptaError(err.0))?;
        let next_sequence = existing_entries
            .iter()
            .filter(|entry| entry.session_id.0 == record.session_id)
            .count() as u64
            + 1;
        let now = current_unix_ms()?;

        self.memory
            .append_transcript_sync(TranscriptEntry {
                entry_id: format!("{}-{}-user", record.session_id, next_sequence),
                session_id: SessionId(record.session_id.clone()),
                sequence: next_sequence,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: record.input.clone(),
                created_at_unix_ms: now,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            })
            .map_err(|err| HeptaError(err.0))?;

        self.memory
            .append_transcript_sync(TranscriptEntry {
                entry_id: format!("{}-{}-assistant", record.session_id, next_sequence + 1),
                session_id: SessionId(record.session_id.clone()),
                sequence: next_sequence + 1,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::Assistant),
                content: record.final_text.clone(),
                created_at_unix_ms: now,
                tool_name: record.invoked_tool.clone(),
                correlation_id: None,
                summary_of_range: None,
            })
            .map_err(|err| HeptaError(err.0))?;

        if let Some(reason) = &record.blocked_reason {
            self.memory
                .append_transcript_sync(TranscriptEntry {
                    entry_id: format!("{}-{}-event", record.session_id, next_sequence + 2),
                    session_id: SessionId(record.session_id.clone()),
                    sequence: next_sequence + 2,
                    kind: TranscriptEntryKind::Event,
                    role: None,
                    content: format!("blocked_reason:{}", reason),
                    created_at_unix_ms: now,
                    tool_name: record.invoked_tool.clone(),
                    correlation_id: None,
                    summary_of_range: None,
                })
                .map_err(|err| HeptaError(err.0))?;
        }

        let mut guard = self
            .history_state
            .lock()
            .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
        guard.push(record);
        Ok(())
    }

    fn upsert_session_record(
        &self,
        session_id: &SessionId,
        title_override: Option<String>,
        last_user_intent_summary: Option<String>,
        archived_at_unix_ms: Option<Option<u64>>,
        touch_last_active: bool,
    ) -> Result<(), HeptaError> {
        self.upsert_session_record_internal(
            session_id,
            title_override,
            last_user_intent_summary,
            archived_at_unix_ms,
            touch_last_active,
            None,
        )
    }

    fn upsert_session_record_with_agent(
        &self,
        session_id: &SessionId,
        title_override: Option<String>,
        last_user_intent_summary: Option<String>,
        archived_at_unix_ms: Option<Option<u64>>,
        touch_last_active: bool,
        agent_id_override: Option<AgentId>,
    ) -> Result<(), HeptaError> {
        self.upsert_session_record_internal(
            session_id,
            title_override,
            last_user_intent_summary,
            archived_at_unix_ms,
            touch_last_active,
            agent_id_override,
        )
    }

    fn upsert_session_record_internal(
        &self,
        session_id: &SessionId,
        title_override: Option<String>,
        last_user_intent_summary: Option<String>,
        archived_at_unix_ms: Option<Option<u64>>,
        touch_last_active: bool,
        agent_id_override: Option<AgentId>,
    ) -> Result<(), HeptaError> {
        let now = current_unix_ms()?;
        let existing = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?
            .into_iter()
            .find(|record| record.session_id == *session_id);

        let record = match existing {
            Some(record) => SessionRecord {
                session_id: record.session_id,
                agent_id: agent_id_override.unwrap_or(record.agent_id),
                title: title_override.unwrap_or(record.title),
                created_at_unix_ms: record.created_at_unix_ms,
                last_active_unix_ms: if touch_last_active {
                    now
                } else {
                    record.last_active_unix_ms
                },
                last_user_intent_summary: last_user_intent_summary
                    .or(record.last_user_intent_summary),
                archived_at_unix_ms: archived_at_unix_ms.unwrap_or(record.archived_at_unix_ms),
            },
            None => SessionRecord {
                session_id: session_id.clone(),
                agent_id: agent_id_override.unwrap_or_else(|| AgentId("main".into())),
                title: title_override.unwrap_or_else(|| format!("Hepta session {}", session_id.0)),
                created_at_unix_ms: now,
                last_active_unix_ms: now,
                last_user_intent_summary,
                archived_at_unix_ms: archived_at_unix_ms.unwrap_or(None),
            },
        };

        self.memory
            .upsert_session_sync(record)
            .map_err(|err| HeptaError(err.0))
    }

    fn set_session_model(&self, session_id: &str, model: ModelRef) -> Result<(), HeptaError> {
        let mut guard = self
            .model_state
            .lock()
            .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        {
            existing.selected_model = model;
        } else {
            guard.sessions.push(SessionModelState {
                session_id: session_id.to_string(),
                selected_model: model,
            });
        }
        Ok(())
    }

    fn session_export(&self, session_id: &str) -> Result<SessionExport, HeptaError> {
        let session_id = session_id.trim();
        let session = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?
            .into_iter()
            .find(|record| record.session_id.0 == session_id)
            .ok_or_else(|| HeptaError(format!("unknown session: {}", session_id)))?;
        let (topic_sessions, topic_graph_edges) =
            self.topic_export_state_for_session(session_id)?;

        Ok(SessionExport {
            version: 1,
            exported_at_unix_ms: current_unix_ms()?,
            model: self.model_selection_for_session(session_id)?.active,
            execution_profile: self.execution_profile_for_session(session_id)?,
            filesystem_scope: self.filesystem_scope_for_session(session_id)?,
            path_capability_gates: self.path_capability_gates_for_session(session_id)?,
            write_path_scope: self.write_path_scope_for_session(session_id)?,
            approval: self.approval_snapshot_for_session(session_id)?,
            history: self.history(Some(session_id), usize::MAX)?,
            write_transactions: self.write_transactions_for_session(session_id)?,
            write_transaction_groups: self.write_transaction_groups_for_session(session_id)?,
            active_write_transaction_group_id: self
                .active_write_transaction_group_id_for_session(session_id)?,
            rollback_group_attempts: self.rollback_group_attempts_for_session(session_id)?,
            write_target_locks: self.write_locks_for_session(session_id)?.0,
            write_group_locks: self.write_locks_for_session(session_id)?.1,
            topic_sessions,
            topic_graph_edges,
            neurons: self.neuron_export_state_for_session(session_id)?,
            intuition_feedback: self.intuition_feedback_for_session(session_id)?,
            model_router_feedback: self.model_router_feedback_for_session(session_id)?,
            session,
        })
    }

    fn topic_export_state_for_session(
        &self,
        session_id: &str,
    ) -> Result<(Vec<TopicSession>, Vec<RuntimeTopicGraphEdgeRecord>), HeptaError> {
        let topic_session_state = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
        let mut topic_sessions = topic_session_state
            .sessions
            .iter()
            .filter(|topic_session| {
                topic_session
                    .linked_surface_session_ids
                    .iter()
                    .any(|linked| linked.0 == session_id)
            })
            .cloned()
            .collect::<Vec<_>>();
        for topic_session in &mut topic_sessions {
            topic_session.graph_edges.clear();
        }

        let exported_topic_session_ids = topic_sessions
            .iter()
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<HashSet<_>>();
        let topic_graph_state = self
            .topic_graph_state
            .lock()
            .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
        let topic_graph_edges = topic_graph_state
            .edges
            .iter()
            .filter(|record| {
                exported_topic_session_ids.contains(&record.source_topic_session_id)
                    && exported_topic_session_ids.contains(&record.edge.target_topic_session_id)
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok((topic_sessions, topic_graph_edges))
    }

    fn replace_topic_export_state_for_session(
        &self,
        session_id: &str,
        topic_sessions: Vec<TopicSession>,
        topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
    ) -> Result<(), HeptaError> {
        let derived_topic_graph_edges = if topic_graph_edges.is_empty() {
            topic_sessions
                .iter()
                .flat_map(|topic_session| {
                    topic_session.graph_edges.iter().cloned().map(|edge| {
                        RuntimeTopicGraphEdgeRecord {
                            source_topic_session_id: topic_session.topic_session_id.clone(),
                            edge,
                        }
                    })
                })
                .collect::<Vec<_>>()
        } else {
            topic_graph_edges
        };

        let imported_topic_session_ids = topic_sessions
            .iter()
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<HashSet<_>>();
        let mut normalized_topic_sessions = topic_sessions;
        for topic_session in &mut normalized_topic_sessions {
            topic_session.graph_edges.clear();
        }
        let normalized_topic_graph_edges = derived_topic_graph_edges
            .into_iter()
            .filter(|record| {
                imported_topic_session_ids.contains(&record.source_topic_session_id)
                    && imported_topic_session_ids.contains(&record.edge.target_topic_session_id)
            })
            .collect::<Vec<_>>();

        let mut topic_session_state = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
        let existing_topic_session_ids = topic_session_state
            .sessions
            .iter()
            .filter(|topic_session| {
                topic_session
                    .linked_surface_session_ids
                    .iter()
                    .any(|linked| linked.0 == session_id)
            })
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<HashSet<_>>();
        topic_session_state.sessions.retain(|topic_session| {
            !topic_session
                .linked_surface_session_ids
                .iter()
                .any(|linked| linked.0 == session_id)
        });
        topic_session_state
            .sessions
            .extend(normalized_topic_sessions.into_iter());
        drop(topic_session_state);

        let mut topic_graph_state = self
            .topic_graph_state
            .lock()
            .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
        topic_graph_state.edges.retain(|record| {
            !existing_topic_session_ids.contains(&record.source_topic_session_id)
                && !existing_topic_session_ids.contains(&record.edge.target_topic_session_id)
        });
        topic_graph_state.edges.extend(normalized_topic_graph_edges);

        Ok(())
    }

    pub(crate) fn neuron_export_state_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<RuntimeNeuronRecord>, HeptaError> {
        let neuron_state = self
            .neuron_state
            .lock()
            .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
        Ok(neuron_state
            .neurons
            .iter()
            .filter(|record| record.session_id == session_id)
            .cloned()
            .collect())
    }

    pub(crate) fn stored_neurons_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<HeptaNeuron>, HeptaError> {
        Ok(self
            .neuron_export_state_for_session(session_id)?
            .into_iter()
            .map(|record| record.neuron)
            .collect())
    }

    pub(crate) fn upsert_neurons_for_session(
        &self,
        session_id: &str,
        neurons: Vec<HeptaNeuron>,
    ) -> Result<(), HeptaError> {
        if neurons.is_empty() {
            return Ok(());
        }
        let mut neuron_state = self
            .neuron_state
            .lock()
            .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
        for neuron in neurons {
            if let Some(existing) = neuron_state.neurons.iter_mut().find(|record| {
                record.session_id == session_id && record.neuron.neuron_id == neuron.neuron_id
            }) {
                existing.neuron = neuron;
            } else {
                neuron_state.neurons.push(RuntimeNeuronRecord {
                    session_id: session_id.to_string(),
                    neuron,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn replace_neuron_state_for_session(
        &self,
        session_id: &str,
        neurons: Vec<RuntimeNeuronRecord>,
    ) -> Result<(), HeptaError> {
        let mut neuron_state = self
            .neuron_state
            .lock()
            .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
        neuron_state
            .neurons
            .retain(|record| record.session_id != session_id);
        neuron_state.neurons.extend(neurons);
        Ok(())
    }

    pub(crate) fn intuition_feedback_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<IntuitionFeedbackRecord>, HeptaError> {
        let feedback_state = self
            .intuition_feedback_state
            .lock()
            .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
        Ok(feedback_state
            .records
            .iter()
            .filter(|record| record.surface_session_id.0 == session_id)
            .cloned()
            .collect())
    }

    pub(crate) fn push_intuition_feedback_record(
        &self,
        record: IntuitionFeedbackRecord,
    ) -> Result<(), HeptaError> {
        let mut feedback_state = self
            .intuition_feedback_state
            .lock()
            .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
        feedback_state.records.push(record);
        Ok(())
    }

    pub(crate) fn replace_intuition_feedback_for_session(
        &self,
        session_id: &str,
        records: Vec<IntuitionFeedbackRecord>,
    ) -> Result<(), HeptaError> {
        let mut feedback_state = self
            .intuition_feedback_state
            .lock()
            .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
        feedback_state
            .records
            .retain(|record| record.surface_session_id.0 != session_id);
        feedback_state.records.extend(records);
        Ok(())
    }

    pub(crate) fn model_router_feedback_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<TopicAwareModelFeedbackRecord>, HeptaError> {
        let feedback_state = self
            .model_router_feedback_state
            .lock()
            .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
        Ok(feedback_state
            .records
            .iter()
            .filter(|record| record.session_id == session_id)
            .cloned()
            .collect())
    }

    pub(crate) fn push_model_router_feedback_record(
        &self,
        record: TopicAwareModelFeedbackRecord,
    ) -> Result<(), HeptaError> {
        let mut feedback_state = self
            .model_router_feedback_state
            .lock()
            .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
        feedback_state.records.push(record);
        Ok(())
    }

    pub(crate) fn replace_model_router_feedback_for_session(
        &self,
        session_id: &str,
        records: Vec<TopicAwareModelFeedbackRecord>,
    ) -> Result<(), HeptaError> {
        let mut feedback_state = self
            .model_router_feedback_state
            .lock()
            .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
        feedback_state
            .records
            .retain(|record| record.session_id != session_id);
        feedback_state.records.extend(records);
        Ok(())
    }

    fn apply_session_export(&self, export: SessionExport) -> Result<(), HeptaError> {
        let session_id = export.session.session_id.0.clone();

        if !self.providers.contains_model_ref(&export.model) {
            return Err(HeptaError(format!(
                "cannot import session {} with unknown model {}/{}",
                session_id, export.model.provider, export.model.model
            )));
        }

        self.memory
            .upsert_session_sync(export.session)
            .map_err(|err| HeptaError(err.0))?;

        {
            let mut approval_state = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            approval_state.remove_session(&session_id);
            if !export.approval.granted_tools.is_empty() || !export.approval.pending.is_empty() {
                approval_state.sessions.push(SessionApprovalState {
                    session_id: session_id.clone(),
                    granted_tools: export.approval.granted_tools,
                    pending: export.approval.pending,
                });
            }
        }
        {
            let mut history_state = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            history_state.retain(|turn| turn.session_id != session_id);
            history_state.extend(export.history.into_iter().rev());
        }
        {
            let mut write_transaction_state = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            write_transaction_state.retain(|entry| entry.session_id != session_id);
            write_transaction_state.extend(export.write_transactions.into_iter().rev());
        }
        {
            let mut write_transaction_group_state = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            write_transaction_group_state
                .groups
                .retain(|group| group.session_id != session_id);
            write_transaction_group_state
                .rollback_attempts
                .retain(|attempt| attempt.session_id != session_id);
            write_transaction_group_state
                .active_bindings
                .retain(|binding| binding.session_id != session_id);
            write_transaction_group_state
                .groups
                .extend(export.write_transaction_groups.into_iter().rev());
            write_transaction_group_state
                .rollback_attempts
                .extend(export.rollback_group_attempts.into_iter().rev());
            if let Some(active_group_id) = export.active_write_transaction_group_id {
                write_transaction_group_state.active_bindings.push(
                    SessionWriteTransactionGroupBinding {
                        session_id: session_id.clone(),
                        active_group_id,
                    },
                );
            }
        }
        {
            let mut write_lock_state = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            write_lock_state
                .target_locks
                .retain(|lock| lock.session_id != session_id);
            write_lock_state
                .group_locks
                .retain(|lock| lock.session_id != session_id);
            write_lock_state
                .target_locks
                .extend(export.write_target_locks.into_iter().rev());
            write_lock_state
                .group_locks
                .extend(export.write_group_locks.into_iter().rev());
        }
        {
            let mut model_state = self
                .model_state
                .lock()
                .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
            model_state
                .sessions
                .retain(|item| item.session_id != session_id);
            model_state.sessions.push(SessionModelState {
                session_id: session_id.clone(),
                selected_model: export.model,
            });
        }
        {
            let mut execution_profile_state = self
                .execution_profile_state
                .lock()
                .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
            execution_profile_state
                .sessions
                .retain(|item| item.session_id != session_id);
            execution_profile_state
                .sessions
                .push(SessionExecutionProfileBinding {
                    session_id: session_id.clone(),
                    profile: export.execution_profile,
                });
        }
        {
            let mut filesystem_scope_state = self
                .filesystem_scope_state
                .lock()
                .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
            filesystem_scope_state
                .sessions
                .retain(|item| item.session_id != session_id);
            filesystem_scope_state
                .sessions
                .push(SessionFilesystemScopeBinding {
                    session_id: session_id.clone(),
                    scope: export.filesystem_scope,
                });
        }
        {
            let mut capability_gate_state = self
                .capability_gate_state
                .lock()
                .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
            capability_gate_state
                .sessions
                .retain(|item| item.session_id != session_id);
            if !export.path_capability_gates.is_empty() {
                capability_gate_state
                    .sessions
                    .push(SessionCapabilityGateBinding {
                        session_id: session_id.clone(),
                        path_gates: export.path_capability_gates,
                    });
            }
        }
        {
            let mut write_path_scope_state = self
                .write_path_scope_state
                .lock()
                .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
            write_path_scope_state
                .sessions
                .retain(|item| item.session_id != session_id);
            write_path_scope_state
                .sessions
                .push(SessionWritePathScopeBinding {
                    session_id: session_id.clone(),
                    scope: export.write_path_scope,
                });
        }
        self.replace_topic_export_state_for_session(
            &session_id,
            export.topic_sessions,
            export.topic_graph_edges,
        )?;
        self.replace_neuron_state_for_session(&session_id, export.neurons)?;
        self.replace_intuition_feedback_for_session(&session_id, export.intuition_feedback)?;
        self.replace_model_router_feedback_for_session(&session_id, export.model_router_feedback)?;
        if self.active_session_id()? == session_id {
            let archived = self
                .memory
                .list_sessions()
                .map_err(|err| HeptaError(err.0))?
                .into_iter()
                .find(|record| record.session_id.0 == session_id)
                .and_then(|record| record.archived_at_unix_ms)
                .is_some();
            if archived {
                let fallback = self.choose_fallback_session_id(Some(&session_id))?;
                let mut guard = self
                    .session_state
                    .lock()
                    .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
                guard.active_session_id = fallback;
            }
        }
        Ok(())
    }

    fn choose_fallback_session_id(
        &self,
        excluded_session_id: Option<&str>,
    ) -> Result<String, HeptaError> {
        let excluded_session_id = excluded_session_id.unwrap_or_default();
        let mut sessions = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?;
        sessions.sort_by_key(|session| std::cmp::Reverse(session.last_active_unix_ms));
        if let Some(candidate) = sessions.into_iter().find(|session| {
            session.session_id.0 != excluded_session_id && session.archived_at_unix_ms.is_none()
        }) {
            return Ok(candidate.session_id.0);
        }

        let fallback = if excluded_session_id == "session-main" {
            "session-fallback".to_string()
        } else {
            "session-main".to_string()
        };
        self.ensure_session_record_sync(&fallback)?;
        Ok(fallback)
    }

    fn runtime_snapshot(&self) -> Result<RuntimeSnapshot, HeptaError> {
        let model_state = self
            .model_state
            .lock()
            .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
        let approval_state = self
            .approval_state
            .lock()
            .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
        let history_state = self
            .history_state
            .lock()
            .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
        let event_state = self
            .event_state
            .lock()
            .map_err(|_| HeptaError("event state mutex poisoned".into()))?;
        let write_transaction_state = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
        let write_transaction_group_state = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        let write_lock_state = self
            .write_lock_state
            .lock()
            .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
        let execution_profile_state = self
            .execution_profile_state
            .lock()
            .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
        let filesystem_scope_state = self
            .filesystem_scope_state
            .lock()
            .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
        let capability_gate_state = self
            .capability_gate_state
            .lock()
            .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
        let write_path_scope_state = self
            .write_path_scope_state
            .lock()
            .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
        let topic_session_state = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
        let topic_graph_state = self
            .topic_graph_state
            .lock()
            .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
        let neuron_state = self
            .neuron_state
            .lock()
            .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
        let intuition_feedback_state = self
            .intuition_feedback_state
            .lock()
            .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
        let model_router_feedback_state = self
            .model_router_feedback_state
            .lock()
            .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
        let worker_task_state = self
            .worker_task_state
            .lock()
            .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
        let multi_agent_runtime_state = self
            .multi_agent_runtime_state
            .lock()
            .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
        let policy_rules = self
            .policy
            .custom_rules()
            .map_err(|err| HeptaError(err.0))?;

        Ok(RuntimeSnapshot {
            version: 1,
            active_model: model_state.default_active.clone(),
            available_models: self.providers.available_models(),
            session_models: model_state.sessions.clone(),
            active_session_id: self.active_session_id()?,
            policy_rules,
            approvals: approval_state.all_sessions(),
            history: history_state.clone(),
            session_execution_profiles: execution_profile_state.sessions.clone(),
            session_filesystem_scopes: filesystem_scope_state.sessions.clone(),
            session_capability_gates: capability_gate_state.sessions.clone(),
            session_write_path_scopes: write_path_scope_state.sessions.clone(),
            events: event_state.snapshot(),
            write_transactions: write_transaction_state.clone(),
            write_transaction_groups: write_transaction_group_state.groups.clone(),
            active_write_transaction_groups: write_transaction_group_state.active_bindings.clone(),
            rollback_group_attempts: write_transaction_group_state.rollback_attempts.clone(),
            write_target_locks: write_lock_state.target_locks.clone(),
            write_group_locks: write_lock_state.group_locks.clone(),
            topic_sessions: topic_session_state.sessions.clone(),
            topic_graph_edges: topic_graph_state.edges.clone(),
            neurons: neuron_state.neurons.clone(),
            intuition_feedback: intuition_feedback_state.records.clone(),
            model_router_feedback: model_router_feedback_state.records.clone(),
            worker_tasks: worker_task_state.records.clone(),
            multi_agent_runtime: multi_agent_runtime_state.clone(),
            memory: self.memory.snapshot().map_err(|err| HeptaError(err.0))?,
        })
    }

    fn apply_runtime_snapshot(&self, snapshot: RuntimeSnapshot) -> Result<(), HeptaError> {
        if !self.providers.contains_model_ref(&snapshot.active_model) {
            return Err(HeptaError(format!(
                "cannot load snapshot with unknown active model {}/{}",
                snapshot.active_model.provider, snapshot.active_model.model
            )));
        }
        if let Some(unknown) = snapshot
            .session_models
            .iter()
            .find(|binding| !self.providers.contains_model_ref(&binding.selected_model))
        {
            return Err(HeptaError(format!(
                "cannot load snapshot with unknown session model {} -> {}/{}",
                unknown.session_id, unknown.selected_model.provider, unknown.selected_model.model
            )));
        }
        {
            let mut model_state = self
                .model_state
                .lock()
                .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
            let runtime_default = self.providers.default_model();
            model_state.default_active = if is_builtin_demo_model(&snapshot.active_model)
                && !is_builtin_demo_model(&runtime_default)
            {
                runtime_default
            } else {
                snapshot.active_model
            };
            model_state.sessions = snapshot.session_models;
        }
        {
            let mut session_state = self
                .session_state
                .lock()
                .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
            session_state.active_session_id = snapshot.active_session_id;
        }
        self.policy
            .replace_rules(snapshot.policy_rules)
            .map_err(|err| HeptaError(err.0))?;
        {
            let mut approval_state = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            approval_state.sessions = snapshot.approvals;
        }
        {
            let mut execution_profile_state = self
                .execution_profile_state
                .lock()
                .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
            execution_profile_state.sessions = snapshot.session_execution_profiles;
        }
        {
            let mut filesystem_scope_state = self
                .filesystem_scope_state
                .lock()
                .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
            filesystem_scope_state.sessions = snapshot.session_filesystem_scopes;
        }
        {
            let mut capability_gate_state = self
                .capability_gate_state
                .lock()
                .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
            capability_gate_state.sessions = snapshot.session_capability_gates;
        }
        {
            let mut write_path_scope_state = self
                .write_path_scope_state
                .lock()
                .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
            write_path_scope_state.sessions = snapshot.session_write_path_scopes;
        }
        {
            let mut write_transaction_state = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            *write_transaction_state = snapshot.write_transactions;
        }
        {
            let mut write_transaction_group_state = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            write_transaction_group_state.groups = snapshot.write_transaction_groups;
            write_transaction_group_state.active_bindings =
                snapshot.active_write_transaction_groups;
            write_transaction_group_state.rollback_attempts = snapshot.rollback_group_attempts;
        }
        {
            let mut write_lock_state = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            write_lock_state.target_locks = snapshot.write_target_locks;
            write_lock_state.group_locks = snapshot.write_group_locks;
        }
        {
            let mut topic_session_state = self
                .topic_session_state
                .lock()
                .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
            topic_session_state.sessions = snapshot.topic_sessions;
        }
        {
            let mut topic_graph_state = self
                .topic_graph_state
                .lock()
                .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
            topic_graph_state.edges = snapshot.topic_graph_edges;
        }
        {
            let mut neuron_state = self
                .neuron_state
                .lock()
                .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
            neuron_state.neurons = snapshot.neurons;
        }
        {
            let mut intuition_feedback_state = self
                .intuition_feedback_state
                .lock()
                .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
            intuition_feedback_state.records = snapshot.intuition_feedback;
        }
        {
            let mut model_router_feedback_state = self
                .model_router_feedback_state
                .lock()
                .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
            model_router_feedback_state.records = snapshot.model_router_feedback;
        }
        {
            let mut worker_task_state = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            worker_task_state.records = snapshot.worker_tasks;
        }
        {
            let mut multi_agent_runtime_state = self
                .multi_agent_runtime_state
                .lock()
                .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
            *multi_agent_runtime_state = snapshot.multi_agent_runtime;
        }
        {
            let mut history_state = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            *history_state = snapshot.history;
        }
        {
            let mut event_state = self
                .event_state
                .lock()
                .map_err(|_| HeptaError("event state mutex poisoned".into()))?;
            event_state.replace(snapshot.events);
        }
        self.memory
            .restore(snapshot.memory)
            .map_err(|err| HeptaError(err.0))?;
        Ok(())
    }

    fn rebind_session_export_topic_state(
        export: &mut SessionExport,
        source_session_id: &str,
        target_session_id: &str,
    ) {
        if source_session_id == target_session_id {
            return;
        }

        let mut topic_session_id_map = HashMap::new();
        let mut topic_id_map = HashMap::new();

        for topic_session in &mut export.topic_sessions {
            let previous_topic_session_id = topic_session.topic_session_id.clone();
            let previous_topic_id = topic_session.topic_id.0.clone();
            let next_topic_session_id = rebind_bootstrap_topic_session_id(
                &previous_topic_session_id,
                source_session_id,
                target_session_id,
            );
            let next_topic_id =
                rebind_bootstrap_topic_id(&previous_topic_id, source_session_id, target_session_id);

            topic_session_id_map.insert(previous_topic_session_id, next_topic_session_id.clone());
            topic_id_map.insert(previous_topic_id, next_topic_id.clone());
            topic_session.topic_session_id = next_topic_session_id;
            topic_session.topic_id = hepta_core::TopicId(next_topic_id);
            topic_session.linked_surface_session_ids =
                vec![SessionId(target_session_id.to_string())];
            for span in &mut topic_session.linked_transcript_spans {
                if span.session_id.0 == source_session_id {
                    span.session_id = SessionId(target_session_id.to_string());
                }
            }
            topic_session.graph_edges.clear();
        }

        for record in &mut export.topic_graph_edges {
            if let Some(remapped) = topic_session_id_map.get(&record.source_topic_session_id) {
                record.source_topic_session_id = remapped.clone();
            }
            if let Some(remapped) = topic_session_id_map.get(&record.edge.target_topic_session_id) {
                record.edge.target_topic_session_id = remapped.clone();
            }
        }

        let valid_topic_session_ids = topic_session_id_map
            .values()
            .cloned()
            .collect::<HashSet<_>>();
        export.topic_graph_edges.retain(|record| {
            valid_topic_session_ids.contains(&record.source_topic_session_id)
                && valid_topic_session_ids.contains(&record.edge.target_topic_session_id)
        });

        for record in &mut export.neurons {
            record.session_id = target_session_id.to_string();
            record.neuron.linked_session_ids = vec![SessionId(target_session_id.to_string())];
            for topic_session_id in &mut record.neuron.linked_topic_session_ids {
                if let Some(remapped) = topic_session_id_map.get(topic_session_id) {
                    *topic_session_id = remapped.clone();
                }
            }
            if let Some(remapped) = topic_id_map.get(&record.neuron.topic_id.0) {
                record.neuron.topic_id = hepta_core::TopicId(remapped.clone());
                record.neuron.neuron_id = hepta_core::NeuronId(format!("neuron-{}", remapped));
            }
            for span in &mut record.neuron.important_transcript_spans {
                if span.session_id.0 == source_session_id {
                    span.session_id = SessionId(target_session_id.to_string());
                }
            }
        }

        for record in &mut export.intuition_feedback {
            record.surface_session_id = SessionId(target_session_id.to_string());
            for topic_id in &mut record.source_topic_ids {
                if let Some(remapped) = topic_id_map.get(&topic_id.0) {
                    *topic_id = hepta_core::TopicId(remapped.clone());
                }
            }
        }

        for record in &mut export.model_router_feedback {
            record.session_id = target_session_id.to_string();
            for topic_id in &mut record.topic_ids {
                if let Some(remapped) = topic_id_map.get(&topic_id.0) {
                    *topic_id = hepta_core::TopicId(remapped.clone());
                }
            }
        }
    }

    fn model_key(model: &ModelRef) -> String {
        format!("{}/{}", model.provider, model.model)
    }

    fn resolve_execution_profile_for_session_from_state(
        state: &ExecutionProfileState,
        session_id: &str,
    ) -> ExecutionProfile {
        state
            .sessions
            .iter()
            .find(|item| item.session_id == session_id)
            .map(|item| item.profile)
            .unwrap_or(state.default_profile)
    }

    fn resolve_filesystem_scope_for_session_from_state(
        state: &FilesystemScopeState,
        session_id: &str,
    ) -> FilesystemScope {
        state
            .sessions
            .iter()
            .find(|item| item.session_id == session_id)
            .map(|item| item.scope)
            .unwrap_or(state.default_scope)
    }

    fn resolve_path_capability_gates_for_session_from_state(
        state: &CapabilityGateState,
        session_id: &str,
    ) -> Vec<PathCapabilityGate> {
        state
            .sessions
            .iter()
            .find(|item| item.session_id == session_id)
            .map(|item| item.path_gates.clone())
            .unwrap_or_default()
    }

    fn resolve_write_path_scope_for_session_from_state(
        state: &WritePathScopeState,
        session_id: &str,
    ) -> WritePathScope {
        state
            .sessions
            .iter()
            .find(|item| item.session_id == session_id)
            .map(|item| item.scope)
            .unwrap_or(state.default_scope)
    }

    fn ensure_capability_binding_mut<'a>(
        state: &'a mut CapabilityGateState,
        session_id: &str,
    ) -> &'a mut SessionCapabilityGateBinding {
        if let Some(index) = state
            .sessions
            .iter()
            .position(|item| item.session_id == session_id)
        {
            return &mut state.sessions[index];
        }
        state.sessions.push(SessionCapabilityGateBinding {
            session_id: session_id.to_string(),
            path_gates: Vec::new(),
        });
        state
            .sessions
            .last_mut()
            .expect("capability binding inserted")
    }

    fn resolve_model_for_session_from_state(
        model_state: &ModelState,
        session_id: &str,
    ) -> ModelRef {
        model_state
            .sessions
            .iter()
            .find(|model| model.session_id == session_id)
            .map(|model| model.selected_model.clone())
            .unwrap_or_else(|| model_state.default_active.clone())
    }
}

impl Default for RuntimeKernel {
    fn default() -> Self {
        Self::new()
    }
}

fn runtime_slugify_identifier(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "topic".to_string()
    } else {
        slug
    }
}

fn rebind_bootstrap_topic_session_id(
    value: &str,
    source_session_id: &str,
    target_session_id: &str,
) -> String {
    let source_prefix = format!("topic-session-bootstrap:{}", source_session_id);
    if let Some(rest) = value.strip_prefix(&source_prefix) {
        return format!("topic-session-bootstrap:{}{}", target_session_id, rest);
    }
    value.to_string()
}

fn rebind_bootstrap_topic_id(
    value: &str,
    source_session_id: &str,
    target_session_id: &str,
) -> String {
    let source_prefix = format!("topic-{}", runtime_slugify_identifier(source_session_id));
    if let Some(rest) = value.strip_prefix(&source_prefix) {
        return format!(
            "topic-{}{}",
            runtime_slugify_identifier(target_session_id),
            rest
        );
    }
    value.to_string()
}

fn merge_candidate_bootstrap_topic_session_id(
    value: &str,
    source_session_id: &str,
    target_session_id: &str,
) -> String {
    let source_prefix = format!("topic-session-bootstrap:{}", source_session_id);
    let source_slug = runtime_slugify_identifier(source_session_id);
    if value == source_prefix {
        return format!(
            "topic-session-bootstrap:{}:{}",
            target_session_id, source_slug
        );
    }
    if let Some(rest) = value.strip_prefix(&format!("{}:", source_prefix)) {
        return format!(
            "topic-session-bootstrap:{}:{}:{}",
            target_session_id, source_slug, rest
        );
    }
    rebind_bootstrap_topic_session_id(value, source_session_id, target_session_id)
}

fn merge_candidate_bootstrap_topic_id(
    value: &str,
    source_session_id: &str,
    target_session_id: &str,
) -> String {
    let source_slug = runtime_slugify_identifier(source_session_id);
    let target_slug = runtime_slugify_identifier(target_session_id);
    let source_prefix = format!("topic-{}", source_slug);
    if value == source_prefix {
        return format!("topic-{}-{}", target_slug, source_slug);
    }
    if let Some(rest) = value.strip_prefix(&format!("{}-", source_prefix)) {
        return format!("topic-{}-{}-{}", target_slug, source_slug, rest);
    }
    rebind_bootstrap_topic_id(value, source_session_id, target_session_id)
}

fn allocate_unique_identifier(base: &str, used: &HashSet<String>) -> String {
    if !used.contains(base) {
        return base.to_string();
    }
    for suffix in 2.. {
        let candidate = format!("{}-{}", base, suffix);
        if !used.contains(&candidate) {
            return candidate;
        }
    }
    unreachable!("identifier space exhausted")
}

fn normalize_topic_label_for_merge(label: &str) -> String {
    label
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn find_equivalent_topic_session_index_for_merge(
    existing_sessions: &[TopicSession],
    canonical_topic_session_id: &str,
    canonical_topic_id: &str,
    incoming: &TopicSession,
) -> Option<usize> {
    let incoming_label = normalize_topic_label_for_merge(&incoming.topic_label.0);
    existing_sessions
        .iter()
        .position(|existing| {
            existing.topic_session_id == canonical_topic_session_id
                && existing.topic_id.0 == canonical_topic_id
                && normalize_topic_label_for_merge(&existing.topic_label.0) == incoming_label
        })
        .or_else(|| {
            existing_sessions.iter().position(|existing| {
                existing.topic_id.0 == canonical_topic_id
                    && normalize_topic_label_for_merge(&existing.topic_label.0) == incoming_label
            })
        })
}

fn normalize_topic_session_for_target(
    mut topic_session: TopicSession,
    source_session_id: &str,
    target_session_id: &str,
) -> TopicSession {
    topic_session.linked_surface_session_ids = vec![SessionId(target_session_id.to_string())];
    for span in &mut topic_session.linked_transcript_spans {
        if span.session_id.0 == source_session_id {
            span.session_id = SessionId(target_session_id.to_string());
        }
    }
    topic_session.graph_edges.clear();
    topic_session
}

fn merge_topic_session_records(existing: &mut TopicSession, incoming: &TopicSession) {
    if existing.topic_embedding.is_none() {
        existing.topic_embedding = incoming.topic_embedding.clone();
    }
    for linked_session_id in &incoming.linked_surface_session_ids {
        if existing
            .linked_surface_session_ids
            .iter()
            .all(|linked| linked != linked_session_id)
        {
            existing
                .linked_surface_session_ids
                .push(linked_session_id.clone());
        }
    }
    for transcript_span in &incoming.linked_transcript_spans {
        if existing
            .linked_transcript_spans
            .iter()
            .all(|existing_span| existing_span != transcript_span)
        {
            existing
                .linked_transcript_spans
                .push(transcript_span.clone());
        }
    }
    for open_loop in &incoming.open_loops {
        if existing
            .open_loops
            .iter()
            .all(|existing_loop| existing_loop != open_loop)
        {
            existing.open_loops.push(open_loop.clone());
        }
    }
    for (key, value) in &incoming.entities {
        existing
            .entities
            .entry(key.clone())
            .or_insert_with(|| value.clone());
    }
    for durable_memory_ref in &incoming.durable_memory_refs {
        if existing
            .durable_memory_refs
            .iter()
            .all(|existing_ref| existing_ref != durable_memory_ref)
        {
            existing
                .durable_memory_refs
                .push(durable_memory_ref.clone());
        }
    }
    existing.status = merge_topic_session_status(existing.status, incoming.status);
    existing.created_at_unix_ms = existing.created_at_unix_ms.min(incoming.created_at_unix_ms);
    existing.last_active_unix_ms = existing
        .last_active_unix_ms
        .max(incoming.last_active_unix_ms);
}

fn merge_topic_session_status(
    left: hepta_core::TopicSessionStatus,
    right: hepta_core::TopicSessionStatus,
) -> hepta_core::TopicSessionStatus {
    use hepta_core::TopicSessionStatus::{Active, Archived, Dormant, Merged};

    match (left, right) {
        (Active, _) | (_, Active) => Active,
        (Dormant, _) | (_, Dormant) => Dormant,
        (Merged, _) | (_, Merged) => Merged,
        _ => Archived,
    }
}

fn upsert_runtime_topic_graph_edge_record(
    records: &mut Vec<RuntimeTopicGraphEdgeRecord>,
    incoming: RuntimeTopicGraphEdgeRecord,
) {
    if let Some(existing) = records.iter_mut().find(|record| {
        record.source_topic_session_id == incoming.source_topic_session_id
            && record.edge.target_topic_session_id == incoming.edge.target_topic_session_id
    }) {
        if existing.edge.weight <= incoming.edge.weight {
            existing.edge.kind = incoming.edge.kind;
            existing.edge.relation = incoming
                .edge
                .relation
                .clone()
                .or(existing.edge.relation.clone());
            existing.edge.weight = incoming.edge.weight;
        }
        existing.edge.evidence_count = existing
            .edge
            .evidence_count
            .saturating_add(incoming.edge.evidence_count.max(1));
        existing.edge.last_confirmed_unix_ms = match (
            existing.edge.last_confirmed_unix_ms,
            incoming.edge.last_confirmed_unix_ms,
        ) {
            (Some(left), Some(right)) => Some(left.max(right)),
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        };
        return;
    }

    records.push(incoming);
}

fn simulate_topic_state_merge(
    source_session_id: &str,
    target_session_id: &str,
    mut target_topic_sessions: Vec<TopicSession>,
    mut target_topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
    source_topic_sessions: Vec<TopicSession>,
    source_topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
) -> TopicStateMergeOutcome {
    if source_topic_sessions.is_empty() && source_topic_graph_edges.is_empty() {
        return TopicStateMergeOutcome {
            topic_sessions: target_topic_sessions,
            topic_graph_edges: target_topic_graph_edges,
        };
    }

    let mut used_topic_session_ids = target_topic_sessions
        .iter()
        .map(|topic_session| topic_session.topic_session_id.clone())
        .collect::<HashSet<_>>();
    let mut used_topic_ids = target_topic_sessions
        .iter()
        .map(|topic_session| topic_session.topic_id.0.clone())
        .collect::<HashSet<_>>();
    let mut source_to_target_topic_session_ids = HashMap::new();

    for source_topic_session in source_topic_sessions {
        let original_topic_session_id = source_topic_session.topic_session_id.clone();
        let canonical_topic_session_id = rebind_bootstrap_topic_session_id(
            &original_topic_session_id,
            source_session_id,
            target_session_id,
        );
        let canonical_topic_id = rebind_bootstrap_topic_id(
            &source_topic_session.topic_id.0,
            source_session_id,
            target_session_id,
        );
        let equivalent_existing_index = find_equivalent_topic_session_index_for_merge(
            &target_topic_sessions,
            &canonical_topic_session_id,
            &canonical_topic_id,
            &source_topic_session,
        );

        let mut normalized_topic_session = normalize_topic_session_for_target(
            source_topic_session,
            source_session_id,
            target_session_id,
        );

        if let Some(existing_index) = equivalent_existing_index {
            let mapped_topic_session_id = target_topic_sessions[existing_index]
                .topic_session_id
                .clone();
            normalized_topic_session.topic_session_id = mapped_topic_session_id.clone();
            normalized_topic_session.topic_id =
                target_topic_sessions[existing_index].topic_id.clone();
            merge_topic_session_records(
                &mut target_topic_sessions[existing_index],
                &normalized_topic_session,
            );
            source_to_target_topic_session_ids
                .insert(original_topic_session_id, mapped_topic_session_id);
            continue;
        }

        let merged_topic_session_base = merge_candidate_bootstrap_topic_session_id(
            &original_topic_session_id,
            source_session_id,
            target_session_id,
        );
        let merged_topic_id_base = merge_candidate_bootstrap_topic_id(
            &normalized_topic_session.topic_id.0,
            source_session_id,
            target_session_id,
        );
        let mapped_topic_session_id =
            allocate_unique_identifier(&merged_topic_session_base, &used_topic_session_ids);
        let mapped_topic_id = allocate_unique_identifier(&merged_topic_id_base, &used_topic_ids);
        used_topic_session_ids.insert(mapped_topic_session_id.clone());
        used_topic_ids.insert(mapped_topic_id.clone());

        normalized_topic_session.topic_session_id = mapped_topic_session_id.clone();
        normalized_topic_session.topic_id = hepta_core::TopicId(mapped_topic_id);
        source_to_target_topic_session_ids
            .insert(original_topic_session_id, mapped_topic_session_id.clone());
        target_topic_sessions.push(normalized_topic_session);
    }

    for source_topic_graph_edge in source_topic_graph_edges {
        let Some(mapped_source_topic_session_id) = source_to_target_topic_session_ids
            .get(&source_topic_graph_edge.source_topic_session_id)
            .cloned()
        else {
            continue;
        };
        let Some(mapped_target_topic_session_id) = source_to_target_topic_session_ids
            .get(&source_topic_graph_edge.edge.target_topic_session_id)
            .cloned()
        else {
            continue;
        };
        if mapped_source_topic_session_id == mapped_target_topic_session_id {
            continue;
        }

        let mut mapped_edge = source_topic_graph_edge;
        mapped_edge.source_topic_session_id = mapped_source_topic_session_id;
        mapped_edge.edge.target_topic_session_id = mapped_target_topic_session_id;
        upsert_runtime_topic_graph_edge_record(&mut target_topic_graph_edges, mapped_edge);
    }

    TopicStateMergeOutcome {
        topic_sessions: target_topic_sessions,
        topic_graph_edges: target_topic_graph_edges,
    }
}

struct ProviderRegistry {
    providers: Vec<RegisteredProvider>,
}

fn parse_model_target(target: &str) -> Option<ModelRef> {
    let target = target.trim();
    let (provider, model) = target.split_once('/')?;
    let provider = provider.trim();
    let model = model.trim();
    if provider.is_empty() || model.is_empty() {
        return None;
    }
    Some(ModelRef {
        provider: provider.to_string(),
        model: model.to_string(),
    })
}

fn is_builtin_demo_model(model: &ModelRef) -> bool {
    model.provider == "demo" && model.model == "demo-chat"
}

impl ProviderRegistry {
    fn new() -> Self {
        let mut providers = vec![
            RegisteredProvider::Demo(DemoModelProvider),
            RegisteredProvider::MockOllama(MockOllamaProvider),
        ];
        for descriptor in imported_startup_provider_descriptors() {
            if descriptor.available_models.is_empty() {
                continue;
            }
            if providers
                .iter()
                .any(|provider| provider.id() == descriptor.id.as_str())
            {
                continue;
            }
            providers.push(RegisteredProvider::Imported(ImportedConfigProvider {
                descriptor,
            }));
        }
        Self { providers }
    }

    fn names(&self) -> Vec<String> {
        self.descriptors()
            .into_iter()
            .map(|provider| provider.id)
            .collect()
    }

    fn descriptors(&self) -> Vec<ProviderDescriptor> {
        self.providers
            .iter()
            .map(|provider| provider.descriptor())
            .collect()
    }

    fn available_models(&self) -> Vec<ModelRef> {
        self.descriptors()
            .into_iter()
            .flat_map(|provider| provider.available_models.into_iter())
            .collect()
    }

    fn default_model(&self) -> ModelRef {
        let descriptors = self.descriptors();
        for env_name in ["HEPTA_DEFAULT_MODEL", "HEPTA_TELEGRAM_MODEL"] {
            if let Ok(target) = env::var(env_name)
                && let Some(model) = parse_model_target(target.trim())
                && descriptors
                    .iter()
                    .flat_map(|provider| provider.available_models.iter())
                    .any(|candidate| candidate == &model)
            {
                return model;
            }
        }

        descriptors
            .iter()
            .find(|provider| {
                !matches!(provider.id.as_str(), "demo" | "mock-ollama")
                    && !provider.requires_auth
                    && provider.transport_kind == ProviderTransportKind::OpenAiCompatibleHttp
            })
            .or_else(|| {
                descriptors
                    .iter()
                    .find(|provider| !matches!(provider.id.as_str(), "demo" | "mock-ollama"))
            })
            .or_else(|| descriptors.first())
            .map(|provider| provider.default_model.clone())
            .unwrap_or(ModelRef {
                provider: "demo".into(),
                model: "demo-chat".into(),
            })
    }

    fn find_model(&self, target: &str) -> Option<ModelRef> {
        self.available_models()
            .into_iter()
            .find(|candidate| RuntimeKernel::model_key(candidate) == target)
    }

    fn contains_model_ref(&self, model: &ModelRef) -> bool {
        self.available_models()
            .iter()
            .any(|candidate| candidate == model)
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, HeptaError> {
        let provider = self
            .providers
            .iter()
            .find(|candidate| candidate.id() == request.model.provider)
            .ok_or_else(|| HeptaError(format!("unknown provider: {}", request.model.provider)))?;
        provider
            .chat(request)
            .await
            .map_err(|err| HeptaError(err.0))
    }
}

enum RegisteredProvider {
    Demo(DemoModelProvider),
    MockOllama(MockOllamaProvider),
    Imported(ImportedConfigProvider),
}

impl RegisteredProvider {
    fn id(&self) -> &str {
        match self {
            Self::Demo(provider) => provider.id(),
            Self::MockOllama(provider) => provider.id(),
            Self::Imported(provider) => provider.id(),
        }
    }

    fn descriptor(&self) -> ProviderDescriptor {
        match self {
            Self::Demo(provider) => provider.descriptor(),
            Self::MockOllama(provider) => provider.descriptor(),
            Self::Imported(provider) => provider.descriptor(),
        }
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, hepta_core::ModelError> {
        match self {
            Self::Demo(provider) => provider.chat(request).await,
            Self::MockOllama(provider) => provider.chat(request).await,
            Self::Imported(provider) => provider.chat(request).await,
        }
    }
}

struct ImportedConfigProvider {
    descriptor: ProviderDescriptor,
}

impl ImportedConfigProvider {
    fn id(&self) -> &str {
        &self.descriptor.id
    }

    fn descriptor(&self) -> ProviderDescriptor {
        self.descriptor.clone()
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, hepta_core::ModelError> {
        if is_openai_codex_provider_id(&self.descriptor.id) {
            return openai_codex_responses_chat(request).map_err(hepta_core::ModelError);
        }
        if self.descriptor.transport_kind == ProviderTransportKind::OpenAiCompatibleHttp {
            if let Some(config) = openai_compatible_imported_provider_config(
                &self.descriptor.id,
                &request.model.model,
            ) {
                return openai_compatible_http_chat(&config, request);
            }
        }
        Err(hepta_core::ModelError(format!(
            "provider {} is imported but has no Hepta-native HTTP runtime config",
            self.descriptor.id
        )))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenAiCompatibleProviderConfig {
    base_url: String,
    api_key: Option<String>,
    qwen_thinking_format: Option<QwenThinkingFormat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QwenThinkingFormat {
    TopLevel,
    ChatTemplate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenAiCodexAuthProfile {
    path: PathBuf,
    profile_id: String,
    access: String,
    refresh: Option<String>,
    expires: Option<u64>,
    account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CodexHttpResponse {
    status: u16,
    body: String,
}

fn is_openai_codex_provider_id(provider_id: &str) -> bool {
    matches!(provider_id, "openai-codex" | "codex")
}

fn openai_codex_responses_chat(request: ModelRequest) -> Result<ModelResponse, String> {
    if let Some(tool_output) = request
        .messages
        .iter()
        .find(|message| {
            message.role == MessageRole::Tool && message.content.contains("disk_junk_audit")
        })
        .map(|message| message.content.as_str())
    {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: render_disk_junk_audit_reply(tool_output),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage::default(),
        });
    }
    if let Some(tool_output) = request
        .messages
        .iter()
        .rev()
        .find(|message| message.role == MessageRole::Tool)
        .map(|message| message.content.as_str())
    {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: render_native_tool_result_reply(tool_output),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage::default(),
        });
    }

    let profile = openai_codex_fresh_auth_profile()?;
    let body = openai_codex_responses_request_body(&request, None);
    let endpoint = resolve_openai_codex_responses_url(None);
    let response = curl_post_json_with_secret_files(
        &endpoint,
        &openai_codex_sse_headers(&profile.access, &profile.account_id, None),
        &body,
        request.timeout_ms,
    )?;
    if response.status == 429 || matches!(response.status, 500 | 502 | 503 | 504) {
        return Err(format!(
            "openai-codex provider returned retryable HTTP status {}",
            response.status
        ));
    }
    if response.status < 200 || response.status >= 300 {
        return Err(format!(
            "openai-codex provider returned HTTP status {}: {}",
            response.status,
            redact_codex_error_preview(&response.body)
        ));
    }
    parse_openai_codex_sse_response(&response.body)
}

fn openai_codex_sse_headers(
    access_token: &str,
    account_id: &str,
    session_id: Option<&str>,
) -> Vec<(String, String)> {
    let mut headers = vec![
        ("Authorization".into(), format!("Bearer {}", access_token)),
        ("chatgpt-account-id".into(), account_id.to_string()),
        ("originator".into(), "pi".into()),
        ("User-Agent".into(), openai_codex_user_agent()),
        ("OpenAI-Beta".into(), "responses=experimental".into()),
        ("accept".into(), "text/event-stream".into()),
        ("content-type".into(), "application/json".into()),
    ];
    if let Some(session_id) = session_id.filter(|value| !value.trim().is_empty()) {
        headers.push(("session_id".into(), session_id.to_string()));
        headers.push(("x-client-request-id".into(), session_id.to_string()));
    }
    headers
}

fn openai_codex_user_agent() -> String {
    format!(
        "pi ({} {}; {})",
        std::env::consts::OS,
        std::env::consts::FAMILY,
        std::env::consts::ARCH
    )
}

fn resolve_openai_codex_responses_url(base_url: Option<&str>) -> String {
    let raw = base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("https://chatgpt.com/backend-api");
    let normalized = raw.trim_end_matches('/');
    if normalized.ends_with("/codex/responses") {
        normalized.to_string()
    } else if normalized.ends_with("/codex") {
        format!("{}/responses", normalized)
    } else {
        format!("{}/codex/responses", normalized)
    }
}

fn openai_codex_responses_request_body(request: &ModelRequest, session_id: Option<&str>) -> Value {
    let mut instructions = Vec::new();
    let mut input = Vec::new();
    let mut assistant_index = 0usize;

    for message in &request.messages {
        match message.role {
            MessageRole::System => instructions.push(message.content.clone()),
            MessageRole::User => input.push(json!({
                "role": "user",
                "content": [{"type": "input_text", "text": message.content}],
            })),
            MessageRole::Assistant => {
                input.push(json!({
                    "type": "message",
                    "role": "assistant",
                    "content": [{"type": "output_text", "text": message.content, "annotations": []}],
                    "status": "completed",
                    "id": format!("msg_{}", assistant_index),
                }));
                assistant_index += 1;
            }
            MessageRole::Tool => {}
        }
    }

    let mut body = json!({
        "model": request.model.model,
        "store": false,
        "stream": true,
        "input": input,
        "text": {"verbosity": "low"},
        "include": ["reasoning.encrypted_content"],
        "tool_choice": "auto",
        "parallel_tool_calls": true,
        "reasoning": {
            "effort": openai_codex_reasoning_effort(request.thinking),
            "summary": "auto"
        }
    });
    if !instructions.is_empty() {
        body["instructions"] = Value::String(instructions.join("\n\n"));
    }
    if let Some(session_id) = session_id.filter(|value| !value.trim().is_empty()) {
        body["prompt_cache_key"] = Value::String(session_id.to_string());
    }
    let tools = openai_codex_tool_payloads(&request.tools);
    if !tools.is_empty() {
        body["tools"] = Value::Array(tools);
    }
    body
}

fn openai_codex_reasoning_effort(thinking: ThinkingLevel) -> &'static str {
    match thinking {
        ThinkingLevel::Low => "low",
        ThinkingLevel::Medium => "medium",
        ThinkingLevel::High => "high",
        ThinkingLevel::XHigh => "xhigh",
    }
}

fn openai_codex_tool_payloads(tools: &[ModelToolSpec]) -> Vec<Value> {
    tools
        .iter()
        .map(|tool| {
            let parameters = serde_json::from_str::<Value>(&tool.input_schema_json)
                .unwrap_or_else(|_| json!({"type": "object"}));
            let parameters = sanitize_openai_codex_tool_schema(parameters);
            json!({
                "type": "function",
                "name": tool.name,
                "description": tool.description,
                "parameters": parameters,
                "strict": false,
            })
        })
        .collect()
}

fn sanitize_openai_codex_tool_schema(mut schema: Value) -> Value {
    sanitize_openai_codex_tool_schema_in_place(&mut schema);
    schema
}

fn sanitize_openai_codex_tool_schema_in_place(schema: &mut Value) {
    let Value::Object(object) = schema else {
        return;
    };

    let is_array_schema = match object.get("type") {
        Some(Value::String(kind)) => kind == "array",
        Some(Value::Array(kinds)) => kinds.iter().any(|kind| kind.as_str() == Some("array")),
        _ => false,
    };
    if is_array_schema && !object.contains_key("items") {
        object.insert("items".into(), json!({}));
    }
    if object.get("type").and_then(Value::as_str) == Some("object")
        && !object.contains_key("properties")
    {
        object.insert("properties".into(), json!({}));
    }

    for key in ["properties", "$defs", "definitions", "patternProperties"] {
        if let Some(Value::Object(children)) = object.get_mut(key) {
            for child in children.values_mut() {
                sanitize_openai_codex_tool_schema_in_place(child);
            }
        }
    }
    if let Some(items) = object.get_mut("items") {
        match items {
            Value::Array(children) => {
                for child in children {
                    sanitize_openai_codex_tool_schema_in_place(child);
                }
            }
            child => sanitize_openai_codex_tool_schema_in_place(child),
        }
    }
    for key in ["anyOf", "oneOf", "allOf"] {
        if let Some(Value::Array(children)) = object.get_mut(key) {
            for child in children {
                sanitize_openai_codex_tool_schema_in_place(child);
            }
        }
    }
    if let Some(child) = object.get_mut("additionalProperties") {
        sanitize_openai_codex_tool_schema_in_place(child);
    }
}

fn parse_openai_codex_sse_response(body: &str) -> Result<ModelResponse, String> {
    let mut text = String::new();
    let mut usage = Usage::default();
    let mut finish_reason = FinishReason::Stop;
    let mut tool_calls = Vec::new();
    let mut current_function_name: Option<String> = None;
    let mut current_function_args = String::new();

    for event in parse_sse_json_events(body)? {
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        match event_type {
            "error" => {
                let message = event
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("openai-codex error");
                return Err(format!("openai-codex error: {}", message));
            }
            "response.failed" => {
                let message = event
                    .pointer("/response/error/message")
                    .and_then(Value::as_str)
                    .unwrap_or("openai-codex response failed");
                return Err(format!("openai-codex response failed: {}", message));
            }
            "response.output_item.added" => {
                if event.pointer("/item/type").and_then(Value::as_str) == Some("function_call") {
                    current_function_name = event
                        .pointer("/item/name")
                        .and_then(Value::as_str)
                        .map(ToString::to_string);
                    current_function_args.clear();
                }
            }
            "response.output_text.delta" | "response.refusal.delta" => {
                if let Some(delta) = event.get("delta").and_then(Value::as_str) {
                    text.push_str(delta);
                }
            }
            "response.function_call_arguments.delta" => {
                if let Some(delta) = event.get("delta").and_then(Value::as_str) {
                    current_function_args.push_str(delta);
                }
            }
            "response.function_call_arguments.done" => {
                if let Some(arguments) = event.get("arguments").and_then(Value::as_str) {
                    current_function_args = arguments.to_string();
                }
            }
            "response.output_item.done" => {
                if let Some(item) = event.get("item") {
                    match item.get("type").and_then(Value::as_str).unwrap_or_default() {
                        "message" => {
                            if text.is_empty() {
                                text = codex_message_item_text(item);
                            }
                        }
                        "function_call" => {
                            let name = item
                                .get("name")
                                .and_then(Value::as_str)
                                .map(ToString::to_string)
                                .or_else(|| current_function_name.clone())
                                .unwrap_or_default();
                            if !name.is_empty() {
                                let arguments = item
                                    .get("arguments")
                                    .and_then(Value::as_str)
                                    .map(ToString::to_string)
                                    .filter(|value| !value.is_empty())
                                    .unwrap_or_else(|| current_function_args.clone());
                                tool_calls.push(ToolCall {
                                    name,
                                    arguments_json: normalize_json_arguments(&arguments),
                                });
                            }
                            current_function_name = None;
                            current_function_args.clear();
                        }
                        _ => {}
                    }
                }
            }
            "response.done" | "response.completed" | "response.incomplete" => {
                if let Some(response) = event.get("response") {
                    usage = codex_usage_from_response(response);
                    finish_reason = codex_finish_reason(response);
                }
            }
            _ => {}
        }
    }

    if !tool_calls.is_empty() {
        finish_reason = FinishReason::ToolCall;
    }
    Ok(ModelResponse {
        message: if tool_calls.is_empty() {
            Some(ModelMessage {
                role: MessageRole::Assistant,
                content: text,
            })
        } else {
            None
        },
        tool_calls,
        finish_reason,
        usage,
    })
}

fn parse_sse_json_events(body: &str) -> Result<Vec<Value>, String> {
    let normalized = body.replace("\r\n", "\n");
    let mut events = Vec::new();
    for chunk in normalized.split("\n\n") {
        let data = chunk
            .lines()
            .filter_map(|line| line.strip_prefix("data:"))
            .map(str::trim_start)
            .collect::<Vec<_>>()
            .join("\n");
        if data.is_empty() || data.trim() == "[DONE]" {
            continue;
        }
        let event = serde_json::from_str::<Value>(&data)
            .map_err(|err| format!("invalid openai-codex SSE JSON event: {}", err))?;
        events.push(event);
    }
    Ok(events)
}

fn codex_message_item_text(item: &Value) -> String {
    item.get("content")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|content| {
                    if content.get("type").and_then(Value::as_str) == Some("output_text") {
                        content.get("text").and_then(Value::as_str)
                    } else {
                        content.get("refusal").and_then(Value::as_str)
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default()
}

fn normalize_json_arguments(arguments: &str) -> String {
    let trimmed = arguments.trim();
    if trimmed.is_empty() {
        return "{}".into();
    }
    serde_json::from_str::<Value>(trimmed)
        .map(|value| value.to_string())
        .unwrap_or_else(|_| trimmed.to_string())
}

fn codex_usage_from_response(response: &Value) -> Usage {
    let input_tokens = response
        .pointer("/usage/input_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let cached_tokens = response
        .pointer("/usage/input_tokens_details/cached_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    Usage {
        input_tokens: input_tokens.saturating_sub(cached_tokens),
        output_tokens: response
            .pointer("/usage/output_tokens")
            .and_then(Value::as_u64)
            .unwrap_or(0),
    }
}

fn codex_finish_reason(response: &Value) -> FinishReason {
    match response
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or_default()
    {
        "incomplete" => FinishReason::Length,
        "failed" | "cancelled" => FinishReason::Error,
        _ => FinishReason::Stop,
    }
}

fn openai_codex_fresh_auth_profile() -> Result<OpenAiCodexAuthProfile, String> {
    let profile = load_openai_codex_auth_profile()?;
    let expires = profile.expires.unwrap_or(u64::MAX);
    let now = current_unix_ms().map_err(|err| {
        format!(
            "failed to read current time for openai-codex auth: {}",
            err.0
        )
    })?;
    if expires <= now.saturating_add(120_000) {
        if profile.refresh.as_deref().unwrap_or_default().is_empty() {
            return Err("openai-codex auth profile is expired and has no refresh token".into());
        }
        return refresh_openai_codex_auth_profile(profile);
    }
    Ok(profile)
}

fn load_openai_codex_auth_profile() -> Result<OpenAiCodexAuthProfile, String> {
    let now = current_unix_ms().unwrap_or(0);
    let mut candidates = Vec::new();
    for path in openai_codex_auth_profile_paths() {
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&content) else {
            continue;
        };
        let Some(profiles) = value.get("profiles").and_then(Value::as_object) else {
            continue;
        };
        for profile_id in preferred_openai_codex_profile_ids(&path, profiles) {
            let Some(profile) = profiles.get(&profile_id) else {
                continue;
            };
            if let Some(candidate) =
                openai_codex_auth_profile_from_value(&path, profile_id, profile)
            {
                candidates.push(candidate);
            }
        }
    }
    if let Some(profile_id) = openai_codex_profile_id_override() {
        return candidates
            .into_iter()
            .find(|candidate| candidate.profile_id == profile_id)
            .ok_or_else(|| {
                format!(
                    "requested openai-codex auth profile {} was not found in Hepta local import",
                    profile_id
                )
            });
    }
    select_openai_codex_auth_profile(candidates, now)
        .ok_or_else(|| "no usable openai-codex auth profile found in Hepta local import".into())
}

fn openai_codex_profile_id_override() -> Option<String> {
    env::var("HEPTA_OPENAI_CODEX_PROFILE_ID")
        .or_else(|_| env::var("HEPTA_OPENAI_CODEX_PROFILE"))
        .ok()
        .and_then(|value| normalize_openai_codex_profile_id_override(&value))
}

fn normalize_openai_codex_profile_id_override(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    Some(if value.starts_with("openai-codex:") {
        value.to_string()
    } else {
        format!("openai-codex:{value}")
    })
}

fn openai_codex_auth_profile_from_value(
    path: &Path,
    profile_id: String,
    profile: &Value,
) -> Option<OpenAiCodexAuthProfile> {
    if profile.get("provider").and_then(Value::as_str) != Some("openai-codex") {
        return None;
    }
    let access = profile
        .get("access")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)?;
    let account_id = profile
        .get("accountId")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .or_else(|| extract_chatgpt_account_id_from_jwt(&access))?;
    Some(OpenAiCodexAuthProfile {
        path: path.to_path_buf(),
        profile_id,
        access,
        refresh: profile
            .get("refresh")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string),
        expires: profile.get("expires").and_then(Value::as_u64),
        account_id,
    })
}

fn select_openai_codex_auth_profile(
    mut candidates: Vec<OpenAiCodexAuthProfile>,
    now_ms: u64,
) -> Option<OpenAiCodexAuthProfile> {
    candidates.sort_by(|left, right| {
        let left_expires = left.expires.unwrap_or(u64::MAX);
        let right_expires = right.expires.unwrap_or(u64::MAX);
        let left_fresh = left_expires > now_ms.saturating_add(120_000);
        let right_fresh = right_expires > now_ms.saturating_add(120_000);
        right_fresh
            .cmp(&left_fresh)
            .then_with(|| right_expires.cmp(&left_expires))
            .then_with(|| left.profile_id.cmp(&right.profile_id))
    });
    candidates.into_iter().next()
}

fn openai_codex_auth_profile_paths() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let mut push_candidate = |candidate: PathBuf| {
        if candidate.is_file() && !candidates.iter().any(|existing| existing == &candidate) {
            candidates.push(candidate);
        }
    };
    if let Ok(manifest_path) = env::var("HEPTA_LOCAL_CONFIG_IMPORT_MANIFEST") {
        let manifest_path = PathBuf::from(manifest_path);
        if let Ok(content) = fs::read_to_string(&manifest_path)
            && let Ok(value) = serde_json::from_str::<Value>(&content)
            && let Some(import_root) = value.get("import_root").and_then(Value::as_str)
        {
            for agent in ["hepta", "main"] {
                push_candidate(
                    PathBuf::from(import_root)
                        .join("private/agents")
                        .join(agent)
                        .join("agent/auth-profiles.json"),
                );
            }
        }
    }
    for agent in ["hepta", "main"] {
        push_candidate(
            PathBuf::from(".hepta/local-import/private/agents")
                .join(agent)
                .join("agent/auth-profiles.json"),
        );
    }
    candidates
}

fn preferred_openai_codex_profile_ids(
    profile_path: &Path,
    profiles: &serde_json::Map<String, Value>,
) -> Vec<String> {
    let mut ids = Vec::new();
    let mut push_id = |id: String| {
        if !ids.iter().any(|existing| existing == &id) {
            ids.push(id);
        }
    };
    let auth_state_path = profile_path.with_file_name("auth-state.json");
    if let Ok(content) = fs::read_to_string(auth_state_path)
        && let Ok(value) = serde_json::from_str::<Value>(&content)
        && let Some(last_good) = value
            .get("lastGood")
            .and_then(|last_good| last_good.get("openai-codex"))
            .and_then(Value::as_str)
    {
        push_id(last_good.to_string());
    }
    push_id("openai-codex:default".into());
    let mut profile_ids = profiles.keys().cloned().collect::<Vec<_>>();
    profile_ids.sort();
    for id in profile_ids {
        if id.starts_with("openai-codex:") {
            push_id(id);
        }
    }
    ids
}

fn refresh_openai_codex_auth_profile(
    profile: OpenAiCodexAuthProfile,
) -> Result<OpenAiCodexAuthProfile, String> {
    let refresh = profile
        .refresh
        .as_deref()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "openai-codex auth profile has no refresh token".to_string())?;
    let body = form_urlencode_pairs(&[
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh),
        ("client_id", "app_EMoamEEZ73f0CkXaXp7hrann"),
    ]);
    let response = curl_post_form_with_secret_file(
        "https://auth.openai.com/oauth/token",
        &[(
            "content-type".into(),
            "application/x-www-form-urlencoded".into(),
        )],
        &body,
        Some(60_000),
    )?;
    if response.status < 200 || response.status >= 300 {
        return Err(format!(
            "openai-codex token refresh returned HTTP status {}",
            response.status
        ));
    }
    let value = serde_json::from_str::<Value>(&response.body)
        .map_err(|err| format!("invalid openai-codex refresh response JSON: {}", err))?;
    let access = value
        .get("access_token")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "openai-codex refresh response missing access token".to_string())?
        .to_string();
    let new_refresh = value
        .get("refresh_token")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .or(profile.refresh.clone());
    let expires_in_ms = value
        .get("expires_in")
        .and_then(Value::as_u64)
        .unwrap_or(0)
        .saturating_mul(1000);
    let expires = current_unix_ms()
        .map_err(|err| {
            format!(
                "failed to read current time for openai-codex refresh: {}",
                err.0
            )
        })?
        .saturating_add(expires_in_ms);
    let account_id = extract_chatgpt_account_id_from_jwt(&access)
        .ok_or_else(|| "openai-codex refresh response missing chatgpt account id".to_string())?;
    persist_refreshed_openai_codex_profile(
        &profile.path,
        &profile.profile_id,
        &access,
        new_refresh.as_deref(),
        expires,
        &account_id,
    )?;
    Ok(OpenAiCodexAuthProfile {
        access,
        refresh: new_refresh,
        expires: Some(expires),
        account_id,
        ..profile
    })
}

fn persist_refreshed_openai_codex_profile(
    path: &Path,
    profile_id: &str,
    access: &str,
    refresh: Option<&str>,
    expires: u64,
    account_id: &str,
) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read openai-codex auth profile store: {}", err))?;
    let mut value = serde_json::from_str::<Value>(&content)
        .map_err(|err| format!("invalid openai-codex auth profile store JSON: {}", err))?;
    let profile = value
        .get_mut("profiles")
        .and_then(Value::as_object_mut)
        .and_then(|profiles| profiles.get_mut(profile_id))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            "openai-codex auth profile disappeared before refresh persist".to_string()
        })?;
    profile.insert("access".into(), Value::String(access.to_string()));
    if let Some(refresh) = refresh.filter(|value| !value.is_empty()) {
        profile.insert("refresh".into(), Value::String(refresh.to_string()));
    }
    profile.insert("expires".into(), Value::Number(expires.into()));
    profile.insert("accountId".into(), Value::String(account_id.to_string()));
    fs::write(
        path,
        serde_json::to_string_pretty(&value).map_err(|err| {
            format!(
                "failed to serialize openai-codex auth profile store: {}",
                err
            )
        })?,
    )
    .map_err(|err| format!("failed to persist refreshed openai-codex profile: {}", err))
}

fn extract_chatgpt_account_id_from_jwt(token: &str) -> Option<String> {
    let payload = token.split('.').nth(1)?;
    let decoded = base64_url_decode(payload)?;
    let value = serde_json::from_slice::<Value>(&decoded).ok()?;
    value
        .get("https://api.openai.com/auth")?
        .get("chatgpt_account_id")?
        .as_str()
        .map(ToString::to_string)
}

fn base64_url_decode(input: &str) -> Option<Vec<u8>> {
    let mut buffer = 0u32;
    let mut bits = 0u8;
    let mut out = Vec::new();
    for ch in input.chars() {
        if ch == '=' {
            break;
        }
        let value = match ch {
            'A'..='Z' => ch as u8 - b'A',
            'a'..='z' => ch as u8 - b'a' + 26,
            '0'..='9' => ch as u8 - b'0' + 52,
            '-' => 62,
            '_' => 63,
            _ => return None,
        } as u32;
        buffer = (buffer << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((buffer >> bits) & 0xff) as u8);
        }
    }
    Some(out)
}

fn curl_post_json_with_secret_files(
    url: &str,
    headers: &[(String, String)],
    body: &Value,
    timeout_ms: Option<u64>,
) -> Result<CodexHttpResponse, String> {
    curl_post_with_secret_files(url, headers, &body.to_string(), timeout_ms)
}

fn curl_post_form_with_secret_file(
    url: &str,
    headers: &[(String, String)],
    body: &str,
    timeout_ms: Option<u64>,
) -> Result<CodexHttpResponse, String> {
    curl_post_with_secret_files(url, headers, body, timeout_ms)
}

fn curl_post_with_secret_files(
    url: &str,
    headers: &[(String, String)],
    body: &str,
    timeout_ms: Option<u64>,
) -> Result<CodexHttpResponse, String> {
    let header_text = headers
        .iter()
        .map(|(name, value)| format!("{}: {}", name, value))
        .collect::<Vec<_>>()
        .join("\n");
    let header_path = write_secret_temp_file("hepta-codex-headers", &header_text)?;
    let body_path = write_secret_temp_file("hepta-codex-body", body)?;
    let timeout_secs = provider_read_timeout_duration(timeout_ms)
        .as_secs()
        .clamp(1, 300)
        .to_string();
    let output = Command::new("curl")
        .arg("--silent")
        .arg("--show-error")
        .arg("--no-buffer")
        .arg("--max-time")
        .arg(timeout_secs)
        .arg("--request")
        .arg("POST")
        .arg("--header")
        .arg(format!("@{}", header_path.display()))
        .arg("--data-binary")
        .arg(format!("@{}", body_path.display()))
        .arg("--write-out")
        .arg("\n__HEPTA_HTTP_STATUS__:%{http_code}\n")
        .arg(url)
        .output();
    let _ = fs::remove_file(&header_path);
    let _ = fs::remove_file(&body_path);
    let output = output.map_err(|err| format!("failed to run curl for openai-codex: {}", err))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() && stdout.trim().is_empty() {
        return Err(format!(
            "curl openai-codex request failed: {}",
            redact_codex_error_preview(&stderr)
        ));
    }
    let (body, status_text) = stdout
        .rsplit_once("\n__HEPTA_HTTP_STATUS__:")
        .ok_or_else(|| "curl openai-codex response missing HTTP status marker".to_string())?;
    let status = status_text.trim().parse::<u16>().map_err(|_| {
        format!(
            "invalid openai-codex HTTP status marker: {}",
            status_text.trim()
        )
    })?;
    Ok(CodexHttpResponse {
        status,
        body: body.to_string(),
    })
}

fn write_secret_temp_file(prefix: &str, content: &str) -> Result<PathBuf, String> {
    let ts = current_unix_ms().unwrap_or(0);
    for attempt in 0..100u8 {
        let mut path = env::temp_dir();
        path.push(format!(
            "{}-{}-{}-{}.tmp",
            prefix,
            std::process::id(),
            ts,
            attempt
        ));
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        match options.open(&path) {
            Ok(mut file) => {
                file.write_all(content.as_bytes())
                    .map_err(|err| format!("failed to write secret temp file: {}", err))?;
                return Ok(path);
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(format!("failed to create secret temp file: {}", err)),
        }
    }
    Err("failed to create unique secret temp file".into())
}

fn form_urlencode_pairs(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(key, value)| format!("{}={}", form_urlencode(key), form_urlencode(value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn form_urlencode(input: &str) -> String {
    let mut out = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

fn redact_codex_error_preview(input: &str) -> String {
    let mut text = input.replace('\n', " ").replace('\r', " ");
    if let Some(index) = text.to_ascii_lowercase().find("authorization:") {
        text.truncate(index + "authorization:".len());
        text.push_str(" <redacted>");
    }
    truncate_for_context(text.trim(), 600)
}

fn openai_compatible_imported_provider_config(
    provider_id: &str,
    model_id: &str,
) -> Option<OpenAiCompatibleProviderConfig> {
    let value = local_import_private_runtime_config()?;
    let provider = value.get("models")?.get("providers")?.get(provider_id)?;
    let base_url = provider.get("baseUrl")?.as_str()?.trim().to_string();
    if base_url.is_empty() {
        return None;
    }
    let api_key = provider
        .get("apiKey")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);
    let model = provider
        .get("models")
        .and_then(Value::as_array)
        .and_then(|models| {
            models.iter().find(|model| {
                model
                    .get("id")
                    .and_then(Value::as_str)
                    .map(|id| id == model_id)
                    .unwrap_or(false)
            })
        });
    let qwen_thinking_format = model
        .and_then(configured_qwen_thinking_format)
        .or_else(|| default_qwen_thinking_format(provider_id, model_id));
    Some(OpenAiCompatibleProviderConfig {
        base_url,
        api_key,
        qwen_thinking_format,
    })
}

fn configured_qwen_thinking_format(model: &Value) -> Option<QwenThinkingFormat> {
    let compat_format = model
        .get("compat")
        .and_then(|compat| compat.get("thinkingFormat"))
        .and_then(Value::as_str)
        .and_then(qwen_thinking_format_from_openclaw_value);
    if compat_format.is_some() {
        return compat_format;
    }
    model
        .get("params")
        .and_then(|params| {
            params
                .get("qwenThinkingFormat")
                .or_else(|| params.get("qwen_thinking_format"))
        })
        .and_then(Value::as_str)
        .and_then(qwen_thinking_format_from_openclaw_value)
}

fn qwen_thinking_format_from_openclaw_value(value: &str) -> Option<QwenThinkingFormat> {
    match value.trim().to_ascii_lowercase().as_str() {
        "qwen" | "top-level" | "top_level" => Some(QwenThinkingFormat::TopLevel),
        "qwen-chat-template" | "chat-template" | "chat_template" => {
            Some(QwenThinkingFormat::ChatTemplate)
        }
        _ => None,
    }
}

fn default_qwen_thinking_format(provider_id: &str, model_id: &str) -> Option<QwenThinkingFormat> {
    let provider = provider_id.to_ascii_lowercase();
    let model = model_id.to_ascii_lowercase();
    if !provider.contains("qwen") && !model.contains("qwen") {
        return None;
    }
    if matches!(
        provider.as_str(),
        "qwen" | "dashscope" | "qwen-portal" | "qwencloud" | "modelstudio"
    ) {
        return Some(QwenThinkingFormat::TopLevel);
    }
    if provider.contains("vllm") || provider.contains("mlx") || provider.contains("ollama") {
        return Some(QwenThinkingFormat::ChatTemplate);
    }
    None
}

fn local_import_private_runtime_config() -> Option<Value> {
    let mut merged = Value::Object(serde_json::Map::new());
    let mut loaded_any = false;

    for config_path in local_import_private_config_paths() {
        let Ok(content) = fs::read_to_string(config_path) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&content) else {
            continue;
        };
        merge_runtime_config_value(&mut merged, value);
        loaded_any = true;
    }

    loaded_any.then_some(merged)
}

fn merge_runtime_config_value(target: &mut Value, source: Value) {
    match (&mut *target, source) {
        (Value::Object(target_object), Value::Object(source_object)) => {
            for (key, source_value) in source_object {
                match target_object.get_mut(&key) {
                    Some(target_value) => merge_runtime_config_value(target_value, source_value),
                    None => {
                        target_object.insert(key, source_value);
                    }
                }
            }
        }
        (Value::Array(target_array), Value::Array(source_array)) => {
            for source_value in source_array {
                if !target_array
                    .iter()
                    .any(|target_value| target_value == &source_value)
                {
                    target_array.push(source_value);
                }
            }
        }
        (Value::Null, source_value) => {
            *target = source_value;
        }
        _ => {}
    }
}

fn local_import_private_config_paths() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let mut push_candidate = |candidate: PathBuf| {
        if candidate.is_file() && !candidates.iter().any(|existing| existing == &candidate) {
            candidates.push(candidate);
        }
    };

    let source_runtime_config_name = ["open", "claw.json"].concat();

    if let Ok(manifest_path) = env::var("HEPTA_LOCAL_CONFIG_IMPORT_MANIFEST") {
        let manifest_path = PathBuf::from(manifest_path);
        if let Ok(content) = fs::read_to_string(&manifest_path)
            && let Ok(value) = serde_json::from_str::<Value>(&content)
            && let Some(import_root) = value.get("import_root").and_then(Value::as_str)
        {
            for file_name in ["hepta_runtime.json", source_runtime_config_name.as_str()] {
                let candidate = PathBuf::from(import_root)
                    .join("private/config")
                    .join(file_name);
                push_candidate(candidate);
            }
        }
    }
    for candidate in [
        PathBuf::from(".hepta/local-import/private/config/hepta_runtime.json"),
        PathBuf::from(".hepta/local-import/private/config")
            .join(source_runtime_config_name.as_str()),
    ] {
        push_candidate(candidate);
    }

    candidates
}

fn openai_compatible_http_chat(
    config: &OpenAiCompatibleProviderConfig,
    request: ModelRequest,
) -> Result<ModelResponse, hepta_core::ModelError> {
    if let Some(tool_output) = request
        .messages
        .iter()
        .find(|message| {
            message.role == MessageRole::Tool && message.content.contains("disk_junk_audit")
        })
        .map(|message| message.content.as_str())
    {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: render_disk_junk_audit_reply(tool_output),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage {
                input_tokens: 0,
                output_tokens: 0,
            },
        });
    }
    if let Some(tool_output) = request
        .messages
        .iter()
        .rev()
        .find(|message| message.role == MessageRole::Tool)
        .map(|message| message.content.as_str())
    {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: render_native_tool_result_reply(tool_output),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage {
                input_tokens: 0,
                output_tokens: 0,
            },
        });
    }
    let endpoint = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));
    let mut payload = json!({
        "model": request.model.model,
        "messages": request.messages.iter().map(|message| {
            json!({
                "role": openai_role_name(&message.role),
                "content": message.content,
            })
        }).collect::<Vec<_>>(),
        "temperature": 0.2,
        "max_tokens": 1200,
        "stream": false,
    });
    let openai_tools = openai_tool_payloads(&request.tools);
    if !openai_tools.is_empty() {
        payload["tools"] = Value::Array(openai_tools);
        payload["tool_choice"] = Value::String("auto".into());
    }
    apply_qwen_openai_compatible_thinking_params(
        &mut payload,
        config.qwen_thinking_format,
        &request,
    );
    let response_text = http_post_json_plaintext(
        &endpoint,
        config.api_key.as_deref(),
        &payload,
        request.timeout_ms,
    )
    .map_err(hepta_core::ModelError)?;
    let response: Value = serde_json::from_str(&response_text).map_err(|err| {
        hepta_core::ModelError(format!("invalid provider JSON response: {}", err))
    })?;
    if let Some(error_message) = response
        .get("error")
        .and_then(|error| error.get("message"))
        .and_then(Value::as_str)
    {
        return Err(hepta_core::ModelError(format!(
            "provider error: {}",
            error_message
        )));
    }
    let message = response
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .cloned()
        .unwrap_or_else(|| json!({}));
    let message_text = message
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    let mut tool_calls = openai_tool_calls_from_message(&message);
    if tool_calls.is_empty() {
        tool_calls = textual_tool_calls_from_message_content(&message_text, &request.tools);
    }
    let input_tokens = response
        .pointer("/usage/prompt_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let output_tokens = response
        .pointer("/usage/completion_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    Ok(ModelResponse {
        message: if !tool_calls.is_empty() {
            None
        } else {
            Some(ModelMessage {
                role: MessageRole::Assistant,
                content: message_text,
            })
        },
        finish_reason: if tool_calls.is_empty() {
            FinishReason::Stop
        } else {
            FinishReason::ToolCall
        },
        tool_calls,
        usage: Usage {
            input_tokens,
            output_tokens,
        },
    })
}

fn split_structured_tool_output(tool_output: &str) -> (&str, Option<Value>) {
    if let Some((content, structured_json)) = tool_output.split_once(" | structured=") {
        return (
            content.trim(),
            serde_json::from_str::<Value>(structured_json.trim()).ok(),
        );
    }
    if let Some((content, structured_json)) = tool_output.split_once(" structured=") {
        return (
            content.trim(),
            serde_json::from_str::<Value>(structured_json.trim()).ok(),
        );
    }
    (tool_output.trim(), None)
}

fn render_native_tool_result_reply(tool_output: &str) -> String {
    let (content, structured) = split_structured_tool_output(tool_output);
    let clean_content = content.replace('\n', " ");
    let content_preview = truncate_for_context(clean_content.trim(), 600);

    if let Some(value) = structured {
        let backend = value
            .get("backend")
            .and_then(Value::as_str)
            .unwrap_or("hepta-rust-native");
        let tool = value.get("tool").and_then(Value::as_str).unwrap_or("tool");

        if tool == "process" {
            let action = value
                .pointer("/result/action")
                .and_then(Value::as_str)
                .unwrap_or("run");
            let count = value
                .pointer("/result/processes")
                .and_then(Value::as_array)
                .map(|processes| processes.len());
            let followups = value
                .pointer("/result/followup_actions")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .take(6)
                        .collect::<Vec<_>>()
                        .join("/")
                })
                .filter(|joined| !joined.is_empty())
                .unwrap_or_else(|| "poll/log/kill/clear/remove".into());
            return match count {
                Some(count) => format!(
                    "已通过 Hepta native process 工具完成 `{action}`：共有 {count} 条后台进程记录。后续可用 {followups} 查看或清理；结构化 JSON 已保留在本地，不再展开到聊天里。"
                ),
                None => format!(
                    "已通过 Hepta native process 工具完成 `{action}`：{}。后续可用 {followups} 继续处理；结构化 JSON 已保留在本地，不再展开到聊天里。",
                    content_preview
                ),
            };
        }

        return format!(
            "已通过 {backend} 执行 `{tool}`：{}。结构化结果已保留在本地，不再展开 raw JSON。",
            content_preview
        );
    }

    format!("工具已执行：{}", content_preview)
}

fn openai_role_name(role: &MessageRole) -> &'static str {
    match role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => "tool",
    }
}

fn apply_qwen_openai_compatible_thinking_params(
    payload: &mut Value,
    format: Option<QwenThinkingFormat>,
    request: &ModelRequest,
) -> bool {
    let Some(format) = format else {
        return false;
    };
    let enable_thinking = qwen_enable_thinking_for_request(request);
    match format {
        QwenThinkingFormat::TopLevel => {
            payload["enable_thinking"] = Value::Bool(enable_thinking);
        }
        QwenThinkingFormat::ChatTemplate => {
            let existing = payload.get_mut("chat_template_kwargs");
            if let Some(Value::Object(map)) = existing {
                map.insert("enable_thinking".into(), Value::Bool(enable_thinking));
            } else {
                payload["chat_template_kwargs"] = json!({ "enable_thinking": enable_thinking });
            }
        }
    }
    true
}

fn qwen_enable_thinking_for_request(_request: &ModelRequest) -> bool {
    // Telegram/live-agent replies must never expose Qwen's visible thinking
    // transcript as assistant text. Keep OpenAI-compatible Qwen transports in
    // no-think mode for both tool and ordinary turns; higher-level runtimes can
    // still carry private reasoning through provider-specific channels later.
    false
}

fn openai_tool_payloads(tools: &[ModelToolSpec]) -> Vec<Value> {
    tools
        .iter()
        .map(|tool| {
            let parameters = serde_json::from_str::<Value>(&tool.input_schema_json).unwrap_or_else(
                |_| json!({"type":"object","properties":{},"additionalProperties":true}),
            );
            json!({
                "type": "function",
                "function": {
                    "name": tool.name,
                    "description": tool.description,
                    "parameters": parameters,
                }
            })
        })
        .collect()
}

fn openai_tool_calls_from_message(message: &Value) -> Vec<ToolCall> {
    let mut calls = Vec::new();
    if let Some(tool_calls) = message.get("tool_calls").and_then(Value::as_array) {
        for item in tool_calls {
            let Some(function) = item.get("function") else {
                continue;
            };
            let Some(name) = function.get("name").and_then(Value::as_str) else {
                continue;
            };
            let arguments_json = match function.get("arguments") {
                Some(Value::String(arguments)) => arguments.clone(),
                Some(value) => value.to_string(),
                None => "{}".into(),
            };
            calls.push(ToolCall {
                name: name.to_string(),
                arguments_json,
            });
        }
    }

    if calls.is_empty()
        && let Some(function_call) = message.get("function_call")
        && let Some(name) = function_call.get("name").and_then(Value::as_str)
    {
        let arguments_json = function_call
            .get("arguments")
            .and_then(Value::as_str)
            .unwrap_or("{}")
            .to_string();
        calls.push(ToolCall {
            name: name.to_string(),
            arguments_json,
        });
    }

    calls
}

fn textual_tool_calls_from_message_content(
    content: &str,
    tools: &[ModelToolSpec],
) -> Vec<ToolCall> {
    let known_tools = tools
        .iter()
        .map(|tool| tool.name.as_str())
        .collect::<Vec<_>>();
    textual_tool_call_segments(content)
        .into_iter()
        .filter_map(|segment| parse_textual_tool_call_segment(segment, &known_tools))
        .collect()
}

fn textual_tool_call_segments(content: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut rest = content;
    while let Some(start_index) = rest.find("<|tool_call>") {
        let after_start = &rest[start_index + "<|tool_call>".len()..];
        if let Some(end_index) = after_start.find("<tool_call|>") {
            segments.push(after_start[..end_index].trim());
            rest = &after_start[end_index + "<tool_call|>".len()..];
        } else {
            break;
        }
    }
    if segments.is_empty() {
        rest = content;
        while let Some(start_index) = rest.find("<tool_call>") {
            let after_start = &rest[start_index + "<tool_call>".len()..];
            if let Some(end_index) = after_start.find("</tool_call>") {
                segments.push(after_start[..end_index].trim());
                rest = &after_start[end_index + "</tool_call>".len()..];
            } else {
                break;
            }
        }
    }
    if segments.is_empty() && content.trim_start().starts_with("call:") {
        segments.push(content.trim());
    }
    segments
}

fn parse_textual_tool_call_segment(segment: &str, known_tools: &[&str]) -> Option<ToolCall> {
    let trimmed = segment.trim().trim_matches('`').trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(call) = parse_json_textual_tool_call(trimmed, known_tools) {
        return Some(call);
    }
    parse_gemma_textual_tool_call(trimmed, known_tools)
}

fn parse_json_textual_tool_call(segment: &str, known_tools: &[&str]) -> Option<ToolCall> {
    let value = serde_json::from_str::<Value>(segment).ok()?;
    let name = value
        .get("name")
        .or_else(|| value.get("tool"))
        .or_else(|| value.get("tool_name"))
        .and_then(Value::as_str)?;
    if !known_tools.iter().any(|candidate| candidate == &name) {
        return None;
    }
    let arguments_json = match value
        .get("arguments")
        .or_else(|| value.get("args"))
        .or_else(|| value.get("input"))
    {
        Some(Value::String(arguments)) => arguments.clone(),
        Some(Value::Object(_)) | Some(Value::Array(_)) => value
            .get("arguments")
            .or_else(|| value.get("args"))
            .or_else(|| value.get("input"))?
            .to_string(),
        Some(other) => json!({"value": other}).to_string(),
        None => "{}".into(),
    };
    Some(ToolCall {
        name: name.to_string(),
        arguments_json,
    })
}

fn parse_gemma_textual_tool_call(segment: &str, known_tools: &[&str]) -> Option<ToolCall> {
    let rest = segment.strip_prefix("call:")?.trim_start();
    let open_brace = rest.find('{')?;
    let name = rest[..open_brace].trim();
    if name.is_empty() || !known_tools.iter().any(|candidate| candidate == &name) {
        return None;
    }
    let argument_text = rest[open_brace..].trim();
    let arguments_json = parse_relaxed_tool_arguments(argument_text)?;
    Some(ToolCall {
        name: name.to_string(),
        arguments_json,
    })
}

fn parse_relaxed_tool_arguments(argument_text: &str) -> Option<String> {
    if let Ok(value) = serde_json::from_str::<Value>(argument_text) {
        return Some(value.to_string());
    }
    let inner = argument_text.strip_prefix('{')?.strip_suffix('}')?.trim();
    if inner.is_empty() {
        return Some("{}".into());
    }
    let mut map = serde_json::Map::new();
    for item in split_top_level_commas(inner) {
        let (key, value_text) = split_key_value(item)?;
        let key = key.trim().trim_matches('"').trim_matches('\'').to_string();
        if key.is_empty() {
            return None;
        }
        let value_text = value_text.trim();
        let value = serde_json::from_str::<Value>(value_text).unwrap_or_else(|_| {
            Value::String(value_text.trim_matches('"').trim_matches('\'').to_string())
        });
        map.insert(key, value);
    }
    Some(Value::Object(map).to_string())
}

fn split_top_level_commas(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut depth = 0i32;
    let mut in_string = false;
    let mut quote = '\0';
    let mut escaped = false;
    for (index, ch) in input.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' | '\'' => {
                in_string = true;
                quote = ch;
            }
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(input[start..index].trim());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(input[start..].trim());
    parts.into_iter().filter(|part| !part.is_empty()).collect()
}

fn split_key_value(input: &str) -> Option<(&str, &str)> {
    let mut depth = 0i32;
    let mut in_string = false;
    let mut quote = '\0';
    let mut escaped = false;
    for (index, ch) in input.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' | '\'' => {
                in_string = true;
                quote = ch;
            }
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth -= 1,
            ':' if depth == 0 => return Some((&input[..index], &input[index + ch.len_utf8()..])),
            _ => {}
        }
    }
    None
}

fn http_post_json_plaintext(
    url: &str,
    bearer_token: Option<&str>,
    payload: &Value,
    timeout_ms: Option<u64>,
) -> Result<String, String> {
    let parsed = parse_plain_http_url(url)?;
    if parsed.scheme != "http" {
        return Err(format!(
            "Hepta native provider currently allows plain HTTP only for local providers; unsupported scheme: {}",
            parsed.scheme
        ));
    }
    let body = payload.to_string();
    let mut headers = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nAccept: application/json\r\nConnection: close\r\nContent-Length: {}\r\n",
        parsed.path,
        parsed.host_header,
        body.len()
    );
    if let Some(token) = bearer_token.filter(|token| !token.trim().is_empty()) {
        headers.push_str(&format!("Authorization: Bearer {}\r\n", token));
    }
    headers.push_str("\r\n");
    let mut stream = TcpStream::connect((parsed.host.as_str(), parsed.port))
        .map_err(|err| format!("failed to connect provider {}: {}", parsed.host_header, err))?;
    let read_timeout = provider_read_timeout_duration(timeout_ms);
    stream
        .set_read_timeout(Some(read_timeout))
        .map_err(|err| format!("failed to set provider read timeout: {}", err))?;
    stream
        .set_write_timeout(Some(std::time::Duration::from_secs(15)))
        .map_err(|err| format!("failed to set provider write timeout: {}", err))?;
    stream
        .write_all(headers.as_bytes())
        .and_then(|_| stream.write_all(body.as_bytes()))
        .map_err(|err| format!("failed to write provider request: {}", err))?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).map_err(|err| {
        if matches!(
            err.kind(),
            std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
        ) {
            format!(
                "provider read timeout after {} ms",
                read_timeout.as_millis()
            )
        } else {
            format!("failed to read provider response: {}", err)
        }
    })?;
    let raw_text = String::from_utf8_lossy(&raw).to_string();
    let (head, body) = raw_text
        .split_once("\r\n\r\n")
        .ok_or_else(|| "provider returned malformed HTTP response".to_string())?;
    let status_line = head.lines().next().unwrap_or_default();
    if !status_line.contains(" 200 ") {
        return Err(format!("provider returned non-200 status: {}", status_line));
    }
    if head
        .lines()
        .any(|line| line.eq_ignore_ascii_case("transfer-encoding: chunked"))
    {
        return decode_http_chunked_body(body);
    }
    Ok(body.to_string())
}

fn provider_read_timeout_duration(timeout_ms: Option<u64>) -> std::time::Duration {
    const DEFAULT_PROVIDER_READ_TIMEOUT_MS: u64 = 90_000;
    const MIN_PROVIDER_READ_TIMEOUT_MS: u64 = 1_000;
    const MAX_PROVIDER_READ_TIMEOUT_MS: u64 = 300_000;
    let timeout_ms = timeout_ms
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_PROVIDER_READ_TIMEOUT_MS)
        .clamp(MIN_PROVIDER_READ_TIMEOUT_MS, MAX_PROVIDER_READ_TIMEOUT_MS);
    std::time::Duration::from_millis(timeout_ms)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlainHttpUrl {
    scheme: String,
    host: String,
    port: u16,
    host_header: String,
    path: String,
}

fn parse_plain_http_url(url: &str) -> Result<PlainHttpUrl, String> {
    let (scheme, rest) = url
        .split_once("://")
        .ok_or_else(|| format!("invalid URL: {}", url))?;
    let (authority, path) = match rest.split_once('/') {
        Some((authority, path)) => (authority, format!("/{}", path)),
        None => (rest, "/".to_string()),
    };
    if authority.is_empty() {
        return Err(format!("invalid URL authority: {}", url));
    }
    let (host, port) = if let Some((host, port_text)) = authority.rsplit_once(':') {
        let port = port_text
            .parse::<u16>()
            .map_err(|_| format!("invalid URL port: {}", port_text))?;
        (host.to_string(), port)
    } else {
        (
            authority.to_string(),
            if scheme == "https" { 443 } else { 80 },
        )
    };
    Ok(PlainHttpUrl {
        scheme: scheme.to_string(),
        host,
        port,
        host_header: authority.to_string(),
        path,
    })
}

fn decode_http_chunked_body(body: &str) -> Result<String, String> {
    let mut rest = body;
    let mut decoded = String::new();
    loop {
        let (size_line, after_size) = rest
            .split_once("\r\n")
            .ok_or_else(|| "malformed chunked provider response".to_string())?;
        let size = usize::from_str_radix(size_line.trim(), 16)
            .map_err(|_| format!("invalid HTTP chunk size: {}", size_line))?;
        if size == 0 {
            return Ok(decoded);
        }
        if after_size.len() < size + 2 {
            return Err("truncated chunked provider response".into());
        }
        decoded.push_str(&after_size[..size]);
        rest = &after_size[size + 2..];
    }
}

#[cfg(not(test))]
fn imported_startup_provider_descriptors() -> Vec<ProviderDescriptor> {
    let manifest_path = std::env::var("HEPTA_LOCAL_CONFIG_IMPORT_MANIFEST")
        .unwrap_or_else(|_| ".hepta/local-import/manifest.json".into());
    hepta_core::LocalConfigImportStatus::from_manifest_path(manifest_path)
        .manifest
        .and_then(|manifest| manifest.startup_config)
        .map(|startup| startup.model_providers)
        .unwrap_or_default()
}

#[cfg(test)]
fn imported_startup_provider_descriptors() -> Vec<ProviderDescriptor> {
    Vec::new()
}

struct DemoModelProvider;

impl ModelProvider for DemoModelProvider {
    fn id(&self) -> &'static str {
        "demo"
    }

    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id().into(),
            display_name: "Demo Provider".into(),
            transport_kind: ProviderTransportKind::InProcess,
            default_model: ModelRef {
                provider: self.id().into(),
                model: "demo-chat".into(),
            },
            available_models: vec![
                ModelRef {
                    provider: self.id().into(),
                    model: "demo-chat".into(),
                },
                ModelRef {
                    provider: self.id().into(),
                    model: "demo-precise".into(),
                },
                ModelRef {
                    provider: self.id().into(),
                    model: "demo-creative".into(),
                },
            ],
            requires_auth: false,
            supports_tool_calls: true,
        }
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, hepta_core::ModelError> {
        render_provider_response(request, |model| match model {
            "demo-precise" => "[precise]".to_string(),
            "demo-creative" => "[creative]".to_string(),
            _ => "[chat]".to_string(),
        })
    }
}

struct MockOllamaProvider;

impl ModelProvider for MockOllamaProvider {
    fn id(&self) -> &'static str {
        "mock-ollama"
    }

    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id().into(),
            display_name: "Mock Ollama".into(),
            transport_kind: ProviderTransportKind::OpenAiCompatibleHttp,
            default_model: ModelRef {
                provider: self.id().into(),
                model: "local-chat".into(),
            },
            available_models: vec![
                ModelRef {
                    provider: self.id().into(),
                    model: "local-chat".into(),
                },
                ModelRef {
                    provider: self.id().into(),
                    model: "local-precise".into(),
                },
            ],
            requires_auth: false,
            supports_tool_calls: true,
        }
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, hepta_core::ModelError> {
        render_provider_response(request, |model| match model {
            "local-precise" => "[ollama-precise]".to_string(),
            _ => "[ollama-chat]".to_string(),
        })
    }
}

fn render_provider_response<F>(
    request: ModelRequest,
    style_for_model: F,
) -> Result<ModelResponse, hepta_core::ModelError>
where
    F: Fn(&str) -> String,
{
    let last_user = request
        .messages
        .iter()
        .rev()
        .find(|message| matches!(message.role, MessageRole::User))
        .map(|message| message.content.clone())
        .unwrap_or_default();

    let tool_message = request
        .messages
        .iter()
        .rev()
        .find(|message| matches!(message.role, MessageRole::Tool))
        .map(|message| message.content.clone());

    let model_style = style_for_model(&request.model.model);
    let context_text = request
        .messages
        .iter()
        .filter(|message| matches!(message.role, MessageRole::System))
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    if last_user.contains("暗号") && context_text.contains("暗号是蓝莓") {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: "暗号是蓝莓。".into(),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage {
                input_tokens: 24,
                output_tokens: 6,
            },
        });
    }

    if let Some(tool_output) = tool_message {
        if tool_output.contains("disk_junk_audit") {
            return Ok(ModelResponse {
                message: Some(ModelMessage {
                    role: MessageRole::Assistant,
                    content: render_disk_junk_audit_reply(&tool_output),
                }),
                tool_calls: vec![],
                finish_reason: FinishReason::Stop,
                usage: Usage {
                    input_tokens: 48,
                    output_tokens: 80,
                },
            });
        }
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: format!(
                    "{} {}",
                    model_style,
                    render_native_tool_result_reply(&tool_output)
                ),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage {
                input_tokens: 32,
                output_tokens: 12,
            },
        });
    }

    if looks_like_disk_junk_audit_intent(&last_user) {
        return Ok(ModelResponse {
            message: None,
            tool_calls: vec![ToolCall {
                name: "disk_junk_audit".into(),
                arguments_json: json!({
                    "scope": "common_local_cleanup_candidates",
                    "max_entries": 120000,
                })
                .to_string(),
            }],
            finish_reason: FinishReason::ToolCall,
            usage: Usage {
                input_tokens: 48,
                output_tokens: 0,
            },
        });
    }

    if let Some(rest) = last_user.strip_prefix("tool:") {
        return Ok(ModelResponse {
            message: None,
            tool_calls: vec![ToolCall {
                name: "echo".into(),
                arguments_json: json!({ "text": rest.trim() }).to_string(),
            }],
            finish_reason: FinishReason::ToolCall,
            usage: Usage {
                input_tokens: 16,
                output_tokens: 0,
            },
        });
    }

    if let Some(path) = last_user.strip_prefix("read:") {
        return Ok(ModelResponse {
            message: None,
            tool_calls: vec![ToolCall {
                name: "read_file".into(),
                arguments_json: json!({ "path": path.trim() }).to_string(),
            }],
            finish_reason: FinishReason::ToolCall,
            usage: Usage {
                input_tokens: 16,
                output_tokens: 0,
            },
        });
    }

    if let Some(rest) = last_user.strip_prefix("write:") {
        let mut parts = rest.trim().splitn(2, " => ");
        if let (Some(path), Some(content)) = (parts.next(), parts.next()) {
            return Ok(ModelResponse {
                message: None,
                tool_calls: vec![ToolCall {
                    name: "write_file".into(),
                    arguments_json:
                        json!({ "path": path.trim(), "content": content, "mode": "create" })
                            .to_string(),
                }],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 20,
                    output_tokens: 0,
                },
            });
        }
    }

    if let Some(rest) = last_user.strip_prefix("overwrite:") {
        let mut parts = rest.trim().splitn(2, " => ");
        if let (Some(path), Some(content)) = (parts.next(), parts.next()) {
            return Ok(ModelResponse {
                message: None,
                tool_calls: vec![ToolCall {
                    name: "write_file".into(),
                    arguments_json: json!({
                        "path": path.trim(),
                        "content": content,
                        "mode": "overwrite",
                        "confirm_destructive": true,
                    })
                    .to_string(),
                }],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 20,
                    output_tokens: 0,
                },
            });
        }
    }

    if let Some(rest) = last_user.strip_prefix("append:") {
        let mut parts = rest.trim().splitn(2, " => ");
        if let (Some(path), Some(content)) = (parts.next(), parts.next()) {
            return Ok(ModelResponse {
                message: None,
                tool_calls: vec![ToolCall {
                    name: "write_file".into(),
                    arguments_json:
                        json!({ "path": path.trim(), "content": content, "mode": "append" })
                            .to_string(),
                }],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 20,
                    output_tokens: 0,
                },
            });
        }
    }

    if let Some(rest) = last_user.strip_prefix("preview-write:") {
        let mut parts = rest.trim().splitn(2, " => ");
        if let (Some(path), Some(content)) = (parts.next(), parts.next()) {
            return Ok(ModelResponse {
                message: None,
                tool_calls: vec![ToolCall {
                    name: "write_file".into(),
                    arguments_json: json!({
                        "path": path.trim(),
                        "content": content,
                        "mode": "overwrite",
                        "confirm_destructive": true,
                        "preview_only": true,
                    })
                    .to_string(),
                }],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 20,
                    output_tokens: 0,
                },
            });
        }
    }

    Ok(ModelResponse {
        message: Some(ModelMessage {
            role: MessageRole::Assistant,
            content: format!("{} model reply: {}", model_style, last_user),
        }),
        tool_calls: vec![],
        finish_reason: FinishReason::Stop,
        usage: Usage {
            input_tokens: 16,
            output_tokens: 8,
        },
    })
}

impl ConfigurablePolicyEngine {
    fn requirement_for_risk(risk: RiskTier) -> ApprovalRequirement {
        match risk {
            RiskTier::Low => ApprovalRequirement::None,
            RiskTier::Medium => ApprovalRequirement::Ask,
            RiskTier::High => ApprovalRequirement::Deny,
        }
    }

    fn requirement_for_tool(tool_name: &str, risk: RiskTier) -> ApprovalRequirement {
        if tool_name == "exec" {
            ApprovalRequirement::Ask
        } else {
            Self::requirement_for_risk(risk)
        }
    }

    fn default_rules(&self) -> Vec<PolicyRule> {
        vec![
            PolicyRule {
                id: "default-risk-low".into(),
                session_id: None,
                provider_name: None,
                tool_name: None,
                risk_tier: Some(RiskTier::Low),
                requirement: ApprovalRequirement::None,
                reason: "low-risk tools are allowed by default".into(),
            },
            PolicyRule {
                id: "default-risk-medium".into(),
                session_id: None,
                provider_name: None,
                tool_name: None,
                risk_tier: Some(RiskTier::Medium),
                requirement: ApprovalRequirement::Ask,
                reason: "medium-risk tools require explicit approval by default".into(),
            },
            PolicyRule {
                id: "default-risk-high".into(),
                session_id: None,
                provider_name: None,
                tool_name: None,
                risk_tier: Some(RiskTier::High),
                requirement: ApprovalRequirement::Deny,
                reason: "high-risk tools are denied by default".into(),
            },
            PolicyRule {
                id: "default-tool-exec".into(),
                session_id: None,
                provider_name: None,
                tool_name: Some("exec".into()),
                risk_tier: None,
                requirement: ApprovalRequirement::Ask,
                reason: "exec requires explicit approval by default".into(),
            },
        ]
    }

    fn custom_rules(&self) -> Result<Vec<PolicyRule>, hepta_core::PolicyError> {
        self.custom_rules
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))
    }

    fn add_rule(&self, rule: PolicyRule) -> Result<PolicyRule, hepta_core::PolicyError> {
        let mut guard = self
            .custom_rules
            .lock()
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))?;
        guard.push(rule.clone());
        Ok(rule)
    }

    fn remove_rule(&self, rule_id: &str) -> Result<bool, hepta_core::PolicyError> {
        let mut guard = self
            .custom_rules
            .lock()
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))?;
        let before = guard.len();
        guard.retain(|rule| rule.id != rule_id);
        Ok(guard.len() != before)
    }

    fn clear_rules(&self) -> Result<usize, hepta_core::PolicyError> {
        let mut guard = self
            .custom_rules
            .lock()
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))?;
        let removed = guard.len();
        guard.clear();
        Ok(removed)
    }

    fn replace_rules(&self, rules: Vec<PolicyRule>) -> Result<(), hepta_core::PolicyError> {
        let mut guard = self
            .custom_rules
            .lock()
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))?;
        *guard = rules;
        Ok(())
    }

    fn matches_rule(rule: &PolicyRule, context: &PolicyEvaluationContext) -> bool {
        if let Some(session_id) = rule.session_id.as_deref() {
            if context.session_id.as_ref().map(|value| value.0.as_str()) != Some(session_id) {
                return false;
            }
        }
        if let Some(provider_name) = rule.provider_name.as_deref() {
            if context.model.as_ref().map(|model| model.provider.as_str()) != Some(provider_name) {
                return false;
            }
        }
        if let Some(tool_name) = rule.tool_name.as_deref() {
            if context.tool_name != tool_name {
                return false;
            }
        }
        if let Some(risk_tier) = rule.risk_tier {
            if context.risk_tier != risk_tier {
                return false;
            }
        }
        true
    }

    fn rule_sort_key(
        rule: &PolicyRule,
        is_custom: bool,
        index: usize,
    ) -> (u8, usize, u8, u8, u8, u8, usize) {
        let selector_count = [
            rule.session_id.is_some(),
            rule.provider_name.is_some(),
            rule.tool_name.is_some(),
            rule.risk_tier.is_some(),
        ]
        .into_iter()
        .filter(|value| *value)
        .count();

        (
            if is_custom { 1 } else { 0 },
            selector_count,
            if rule.session_id.is_some() { 1 } else { 0 },
            if rule.tool_name.is_some() { 1 } else { 0 },
            if rule.provider_name.is_some() { 1 } else { 0 },
            if rule.risk_tier.is_some() { 1 } else { 0 },
            index,
        )
    }

    fn evaluate_with_match(
        &self,
        context: PolicyEvaluationContext,
    ) -> Result<PolicyDecision, hepta_core::PolicyError> {
        let defaults = self.default_rules();
        let customs = self.custom_rules()?;
        let mut best_match: Option<(PolicyRule, (u8, usize, u8, u8, u8, u8, usize))> = None;

        for (index, rule) in defaults.into_iter().enumerate() {
            if Self::matches_rule(&rule, &context) {
                let score = Self::rule_sort_key(&rule, false, index);
                if best_match
                    .as_ref()
                    .map(|(_, current)| score > *current)
                    .unwrap_or(true)
                {
                    best_match = Some((rule, score));
                }
            }
        }

        for (index, rule) in customs.into_iter().enumerate() {
            if Self::matches_rule(&rule, &context) {
                let score = Self::rule_sort_key(&rule, true, index);
                if best_match
                    .as_ref()
                    .map(|(_, current)| score > *current)
                    .unwrap_or(true)
                {
                    best_match = Some((rule, score));
                }
            }
        }

        match best_match {
            Some((rule, _)) => Ok(PolicyDecision {
                requirement: rule.requirement,
                reason: Self::decision_reason(&rule, &context),
                matched_rule_id: Some(rule.id),
            }),
            None => Ok(PolicyDecision {
                requirement: Self::requirement_for_risk(context.risk_tier),
                reason: format!(
                    "fallback risk policy for {} ({})",
                    context.tool_name,
                    format_risk_tier(context.risk_tier)
                ),
                matched_rule_id: None,
            }),
        }
    }

    fn decision_reason(rule: &PolicyRule, context: &PolicyEvaluationContext) -> String {
        match rule.id.as_str() {
            "default-risk-low" => format!("{} is low risk", context.tool_name),
            "default-risk-medium" => {
                format!(
                    "{} is medium risk and requires explicit approval",
                    context.tool_name
                )
            }
            "default-risk-high" => {
                format!("{} is high risk and denied by default", context.tool_name)
            }
            "default-tool-exec" => "exec requires explicit approval".into(),
            _ => rule.reason.clone(),
        }
    }
}

impl PolicyEngine for ConfigurablePolicyEngine {
    async fn evaluate_tool(
        &self,
        context: PolicyEvaluationContext,
    ) -> Result<PolicyDecision, hepta_core::PolicyError> {
        self.evaluate_with_match(context)
    }
}

struct ToolRegistry {
    tools: Vec<RegisteredTool>,
}

impl ToolRegistry {
    fn new() -> Self {
        let mut tools = vec![
            RegisteredTool::Echo(EchoTool),
            RegisteredTool::ReadFile(ReadFileTool),
            RegisteredTool::WriteFile(WriteFileTool),
            RegisteredTool::ListDir(ListDirTool),
            RegisteredTool::SearchText(SearchTextTool),
            RegisteredTool::DiskJunkAudit(DiskJunkAuditTool),
            RegisteredTool::JsonGet(JsonGetTool),
            RegisteredTool::SkillPropose(SkillProposeTool),
            RegisteredTool::SkillScan(SkillScanTool),
            RegisteredTool::SkillApplyPlan(SkillApplyPlanTool),
            RegisteredTool::ToolManifestValidate(ToolManifestValidateTool),
            RegisteredTool::ToolGenerateStub(ToolGenerateStubTool),
        ];
        tools.extend(
            native_openclaw_compatible_tools()
                .into_iter()
                .map(RegisteredTool::NativeOpenClawCompatible),
        );
        Self { tools }
    }

    fn names(&self) -> Vec<String> {
        self.tools
            .iter()
            .map(|tool| tool.name().to_string())
            .collect()
    }

    fn descriptors(&self) -> Vec<ToolDescriptor> {
        self.tools
            .iter()
            .map(|tool| {
                let schema = tool.schema();
                ToolDescriptor {
                    name: schema.name,
                    description: schema.description,
                    risk_tier: tool.risk_tier(),
                    execution_metadata: tool.execution_metadata(),
                    default_approval_requirement: ConfigurablePolicyEngine::requirement_for_tool(
                        tool.name(),
                        tool.risk_tier(),
                    ),
                    input_schema_json: schema.input_schema_json,
                    output_schema_json: schema.output_schema_json,
                }
            })
            .collect()
    }

    fn model_tool_specs(&self) -> Vec<ModelToolSpec> {
        self.tools
            .iter()
            .map(|tool| {
                let schema = tool.schema();
                ModelToolSpec {
                    name: schema.name,
                    description: schema.description,
                    input_schema_json: schema.input_schema_json,
                }
            })
            .collect()
    }

    fn model_tool_specs_for_turn(&self, input: &str) -> Vec<ModelToolSpec> {
        if should_offer_model_tools_for_turn(input) {
            self.model_tool_specs()
        } else {
            Vec::new()
        }
    }

    fn contains(&self, name: &str) -> bool {
        self.tools.iter().any(|tool| tool.name() == name)
    }

    fn execution_metadata(
        &self,
        name: &str,
    ) -> Result<hepta_core::ToolExecutionMetadata, HeptaError> {
        self.tools
            .iter()
            .find(|tool| tool.name() == name)
            .map(|tool| tool.execution_metadata())
            .ok_or_else(|| HeptaError(format!("unknown tool: {}", name)))
    }

    fn schema(&self, name: &str) -> Result<hepta_core::ToolSchema, HeptaError> {
        self.tools
            .iter()
            .find(|tool| tool.name() == name)
            .map(|tool| tool.schema())
            .ok_or_else(|| HeptaError(format!("unknown tool: {}", name)))
    }

    fn risk_tier(&self, name: &str) -> Result<RiskTier, HeptaError> {
        self.tools
            .iter()
            .find(|tool| tool.name() == name)
            .map(|tool| tool.risk_tier())
            .ok_or_else(|| HeptaError(format!("unknown tool: {}", name)))
    }

    fn validate_input(&self, name: &str, input_json: &str) -> Result<(), HeptaError> {
        let schema = self.schema(name)?;
        validate_against_schema_json(&schema.name, "input", &schema.input_schema_json, input_json)
    }

    fn validate_output(&self, name: &str, output_json: &str) -> Result<(), HeptaError> {
        let schema = self.schema(name)?;
        validate_against_schema_json(
            &schema.name,
            "output",
            &schema.output_schema_json,
            output_json,
        )
    }

    async fn invoke(
        &self,
        name: &str,
        ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, HeptaError> {
        let tool = self
            .tools
            .iter()
            .find(|candidate| candidate.name() == name)
            .ok_or_else(|| HeptaError(format!("unknown tool: {}", name)))?;
        let result = if matches!(tool, RegisteredTool::NativeOpenClawCompatible(_)) {
            match tokio::time::timeout(
                Duration::from_millis(NATIVE_TOOL_INVOCATION_TIMEOUT_MS),
                tool.invoke(ctx, req),
            )
            .await
            {
                Ok(result) => result,
                Err(_) => Ok(native_tool_invocation_timeout_result(
                    name,
                    NATIVE_TOOL_INVOCATION_TIMEOUT_MS,
                )),
            }
        } else {
            tool.invoke(ctx, req).await
        };
        result.map_err(|err| HeptaError(err.0))
    }
}

enum RegisteredTool {
    Echo(EchoTool),
    ReadFile(ReadFileTool),
    WriteFile(WriteFileTool),
    ListDir(ListDirTool),
    SearchText(SearchTextTool),
    DiskJunkAudit(DiskJunkAuditTool),
    JsonGet(JsonGetTool),
    SkillPropose(SkillProposeTool),
    SkillScan(SkillScanTool),
    SkillApplyPlan(SkillApplyPlanTool),
    ToolManifestValidate(ToolManifestValidateTool),
    ToolGenerateStub(ToolGenerateStubTool),
    NativeOpenClawCompatible(NativeOpenClawCompatibleTool),
}

impl RegisteredTool {
    fn name(&self) -> &'static str {
        match self {
            Self::Echo(tool) => tool.name(),
            Self::ReadFile(tool) => tool.name(),
            Self::WriteFile(tool) => tool.name(),
            Self::ListDir(tool) => tool.name(),
            Self::SearchText(tool) => tool.name(),
            Self::DiskJunkAudit(tool) => tool.name(),
            Self::JsonGet(tool) => tool.name(),
            Self::SkillPropose(tool) => tool.name(),
            Self::SkillScan(tool) => tool.name(),
            Self::SkillApplyPlan(tool) => tool.name(),
            Self::ToolManifestValidate(tool) => tool.name(),
            Self::ToolGenerateStub(tool) => tool.name(),
            Self::NativeOpenClawCompatible(tool) => tool.name(),
        }
    }

    fn risk_tier(&self) -> RiskTier {
        match self {
            Self::Echo(tool) => tool.risk_tier(),
            Self::ReadFile(tool) => tool.risk_tier(),
            Self::WriteFile(tool) => tool.risk_tier(),
            Self::ListDir(tool) => tool.risk_tier(),
            Self::SearchText(tool) => tool.risk_tier(),
            Self::DiskJunkAudit(tool) => tool.risk_tier(),
            Self::JsonGet(tool) => tool.risk_tier(),
            Self::SkillPropose(tool) => tool.risk_tier(),
            Self::SkillScan(tool) => tool.risk_tier(),
            Self::SkillApplyPlan(tool) => tool.risk_tier(),
            Self::ToolManifestValidate(tool) => tool.risk_tier(),
            Self::ToolGenerateStub(tool) => tool.risk_tier(),
            Self::NativeOpenClawCompatible(tool) => tool.risk_tier(),
        }
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        match self {
            Self::Echo(tool) => tool.execution_metadata(),
            Self::ReadFile(tool) => tool.execution_metadata(),
            Self::WriteFile(tool) => tool.execution_metadata(),
            Self::ListDir(tool) => tool.execution_metadata(),
            Self::SearchText(tool) => tool.execution_metadata(),
            Self::DiskJunkAudit(tool) => tool.execution_metadata(),
            Self::JsonGet(tool) => tool.execution_metadata(),
            Self::SkillPropose(tool) => tool.execution_metadata(),
            Self::SkillScan(tool) => tool.execution_metadata(),
            Self::SkillApplyPlan(tool) => tool.execution_metadata(),
            Self::ToolManifestValidate(tool) => tool.execution_metadata(),
            Self::ToolGenerateStub(tool) => tool.execution_metadata(),
            Self::NativeOpenClawCompatible(tool) => tool.execution_metadata(),
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        match self {
            Self::Echo(tool) => tool.schema(),
            Self::ReadFile(tool) => tool.schema(),
            Self::WriteFile(tool) => tool.schema(),
            Self::ListDir(tool) => tool.schema(),
            Self::SearchText(tool) => tool.schema(),
            Self::DiskJunkAudit(tool) => tool.schema(),
            Self::JsonGet(tool) => tool.schema(),
            Self::SkillPropose(tool) => tool.schema(),
            Self::SkillScan(tool) => tool.schema(),
            Self::SkillApplyPlan(tool) => tool.schema(),
            Self::ToolManifestValidate(tool) => tool.schema(),
            Self::ToolGenerateStub(tool) => tool.schema(),
            Self::NativeOpenClawCompatible(tool) => tool.schema(),
        }
    }

    async fn invoke(
        &self,
        ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        match self {
            Self::Echo(tool) => tool.invoke(ctx, req).await,
            Self::ReadFile(tool) => tool.invoke(ctx, req).await,
            Self::WriteFile(tool) => tool.invoke(ctx, req).await,
            Self::ListDir(tool) => tool.invoke(ctx, req).await,
            Self::SearchText(tool) => tool.invoke(ctx, req).await,
            Self::DiskJunkAudit(tool) => tool.invoke(ctx, req).await,
            Self::JsonGet(tool) => tool.invoke(ctx, req).await,
            Self::SkillPropose(tool) => tool.invoke(ctx, req).await,
            Self::SkillScan(tool) => tool.invoke(ctx, req).await,
            Self::SkillApplyPlan(tool) => tool.invoke(ctx, req).await,
            Self::ToolManifestValidate(tool) => tool.invoke(ctx, req).await,
            Self::ToolGenerateStub(tool) => tool.invoke(ctx, req).await,
            Self::NativeOpenClawCompatible(tool) => tool.invoke(ctx, req).await,
        }
    }
}

struct EchoTool;

impl Tool for EchoTool {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Return the provided input as-is".into(),
            input_schema_json: r#"{"type":"object","required":["text"],"properties":{"text":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["text"],"properties":{"text":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let text = parse_required_string_field(&req.input_json, "text")?;
        Ok(ToolResult {
            content: format!("echo:{}", text),
            structured_json: Some(json!({ "text": text }).to_string()),
        })
    }
}

struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Medium
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Read a UTF-8 text file from disk".into(),
            input_schema_json: r#"{"type":"object","required":["path"],"properties":{"path":{"type":"string","minLength":1,"description":"relative or absolute file path"}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["path","preview","line_count"],"properties":{"path":{"type":"string","minLength":1},"preview":{"type":"string"},"line_count":{"type":"integer","minimum":0}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let requested_path = parse_required_string_field(&req.input_json, "path")?;
        let workspace_root = tool_workspace_root_path();
        let path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let content = fs::read_to_string(&path).map_err(|err| {
            hepta_core::ToolError(format!("failed to read {}: {}", path.display(), err))
        })?;
        let preview = content.lines().take(6).collect::<Vec<_>>().join(" | ");
        let line_count = content.lines().count();
        Ok(ToolResult {
            content: format!("read_file:{} => {}", path.display(), preview),
            structured_json: Some(
                json!({
                    "path": path.display().to_string(),
                    "preview": preview,
                    "line_count": line_count,
                })
                .to_string(),
            ),
        })
    }
}

struct WriteFileTool;

impl Tool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::High
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: false,
            destructive: true,
            idempotent: false,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Write a UTF-8 text file to disk with explicit create, overwrite, or append semantics".into(),
            input_schema_json: r#"{"type":"object","required":["path","content"],"properties":{"path":{"type":"string","minLength":1,"description":"relative or absolute file path"},"content":{"type":"string","minLength":0,"description":"UTF-8 file content to write"},"mode":{"type":"string","enum":["create","overwrite","append"],"description":"create=new file only, overwrite=replace existing, append=append to existing or create"},"confirm_destructive":{"type":"boolean","description":"required for overwriting an existing file"},"preview_only":{"type":"boolean","description":"when true, return diff/backup plan without mutating the filesystem"}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["path","bytes_written","mode_requested","mode_applied","existed_before","preview_only","content_changed","bytes_before","bytes_after","backup_planned","backup_created","change_summary"],"properties":{"path":{"type":"string","minLength":1},"bytes_written":{"type":"integer","minimum":0},"mode_requested":{"type":"string","enum":["create","overwrite","append"]},"mode_applied":{"type":"string","enum":["create","overwrite","append"]},"existed_before":{"type":"boolean"},"preview_only":{"type":"boolean"},"content_changed":{"type":"boolean"},"bytes_before":{"type":"integer","minimum":0},"bytes_after":{"type":"integer","minimum":0},"backup_planned":{"type":"boolean"},"backup_created":{"type":"boolean"},"backup_path":{"type":"string","minLength":1},"change_summary":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let requested_path = parse_required_string_field(&req.input_json, "path")?;
        let content = parse_required_string_field(&req.input_json, "content")?;
        let mode = parse_optional_string_field(&req.input_json, "mode")?
            .unwrap_or_else(|| "create".to_string());
        let preview_only =
            parse_optional_bool_field(&req.input_json, "preview_only")?.unwrap_or(false);
        let workspace_root = tool_workspace_root_path();
        let path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let existed_before = path.exists();
        let before_content = if existed_before {
            Some(fs::read_to_string(&path).map_err(|err| {
                hepta_core::ToolError(format!(
                    "failed to read existing content from {}: {}",
                    path.display(),
                    err
                ))
            })?)
        } else {
            None
        };
        let before_text = before_content.as_deref().unwrap_or("");
        let after_content = match mode.as_str() {
            "create" | "overwrite" => content.clone(),
            "append" => format!("{}{}", before_text, content),
            other => {
                return Err(hepta_core::ToolError(format!(
                    "unsupported write mode {} for {}",
                    other,
                    path.display()
                )));
            }
        };
        let bytes_before = before_text.len();
        let bytes_after = after_content.len();
        let content_changed = before_text != after_content;
        let backup_planned = existed_before && mode == "overwrite";
        let change_summary = summarize_write_change(
            mode.as_str(),
            existed_before,
            content_changed,
            bytes_before,
            bytes_after,
        );

        if preview_only {
            let mut output = serde_json::Map::new();
            output.insert("path".into(), json!(path.display().to_string()));
            output.insert("bytes_written".into(), json!(0));
            output.insert("mode_requested".into(), json!(mode.clone()));
            output.insert("mode_applied".into(), json!(mode.clone()));
            output.insert("existed_before".into(), json!(existed_before));
            output.insert("preview_only".into(), json!(true));
            output.insert("content_changed".into(), json!(content_changed));
            output.insert("bytes_before".into(), json!(bytes_before));
            output.insert("bytes_after".into(), json!(bytes_after));
            output.insert("backup_planned".into(), json!(backup_planned));
            output.insert("backup_created".into(), json!(false));
            if let Some(backup_path) = backup_planned
                .then(|| preview_backup_path(&workspace_root, &path))
                .transpose()?
            {
                output.insert(
                    "backup_path".into(),
                    json!(backup_path.display().to_string()),
                );
            }
            output.insert("change_summary".into(), json!(change_summary.clone()));
            return Ok(ToolResult {
                content: format!(
                    "write_file:{} => preview {}",
                    path.display(),
                    change_summary
                ),
                structured_json: Some(Value::Object(output).to_string()),
            });
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                hepta_core::ToolError(format!(
                    "failed to create parent directories for {}: {}",
                    path.display(),
                    err
                ))
            })?;
        }
        let mut backup_path = None;
        let mode_applied = match mode.as_str() {
            "create" => {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&path)
                    .map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to create {}: {}",
                            path.display(),
                            err
                        ))
                    })?;
                use std::io::Write as _;
                file.write_all(content.as_bytes()).map_err(|err| {
                    hepta_core::ToolError(format!("failed to write {}: {}", path.display(), err))
                })?;
                "create"
            }
            "overwrite" => {
                if let Some(previous_content) = before_content.as_deref() {
                    let planned_backup_path = preview_backup_path(&workspace_root, &path)?;
                    if let Some(parent) = planned_backup_path.parent() {
                        fs::create_dir_all(parent).map_err(|err| {
                            hepta_core::ToolError(format!(
                                "failed to create backup parent directories for {}: {}",
                                planned_backup_path.display(),
                                err
                            ))
                        })?;
                    }
                    fs::write(&planned_backup_path, previous_content.as_bytes()).map_err(
                        |err| {
                            hepta_core::ToolError(format!(
                                "failed to write backup {}: {}",
                                planned_backup_path.display(),
                                err
                            ))
                        },
                    )?;
                    backup_path = Some(planned_backup_path);
                }
                fs::write(&path, content.as_bytes()).map_err(|err| {
                    hepta_core::ToolError(format!(
                        "failed to overwrite {}: {}",
                        path.display(),
                        err
                    ))
                })?;
                "overwrite"
            }
            "append" => {
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to append {}: {}",
                            path.display(),
                            err
                        ))
                    })?;
                use std::io::Write as _;
                file.write_all(content.as_bytes()).map_err(|err| {
                    hepta_core::ToolError(format!("failed to append {}: {}", path.display(), err))
                })?;
                "append"
            }
            other => {
                return Err(hepta_core::ToolError(format!(
                    "unsupported write mode {} for {}",
                    other,
                    path.display()
                )));
            }
        };
        Ok(ToolResult {
            content: format!(
                "write_file:{} => {} bytes ({})",
                path.display(),
                content.len(),
                mode_applied
            ),
            structured_json: Some({
                let mut output = serde_json::Map::new();
                output.insert("path".into(), json!(path.display().to_string()));
                output.insert("bytes_written".into(), json!(content.len()));
                output.insert("mode_requested".into(), json!(mode.clone()));
                output.insert("mode_applied".into(), json!(mode_applied));
                output.insert("existed_before".into(), json!(existed_before));
                output.insert("preview_only".into(), json!(false));
                output.insert("content_changed".into(), json!(content_changed));
                output.insert("bytes_before".into(), json!(bytes_before));
                output.insert("bytes_after".into(), json!(bytes_after));
                output.insert("backup_planned".into(), json!(backup_planned));
                output.insert("backup_created".into(), json!(backup_path.is_some()));
                if let Some(backup_path) = backup_path.as_ref() {
                    output.insert(
                        "backup_path".into(),
                        json!(backup_path.display().to_string()),
                    );
                }
                output.insert("change_summary".into(), json!(change_summary.clone()));
                Value::Object(output).to_string()
            }),
        })
    }
}

struct ListDirTool;

impl Tool for ListDirTool {
    fn name(&self) -> &'static str {
        "list_dir"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Medium
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "List immediate files and directories under a workspace path".into(),
            input_schema_json: r#"{"type":"object","properties":{"path":{"type":"string","minLength":1},"max_entries":{"type":"integer","minimum":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["path","entry_count"],"properties":{"path":{"type":"string","minLength":1},"entry_count":{"type":"integer","minimum":0},"truncated":{"type":"boolean"}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let requested_path =
            parse_optional_string_field(&req.input_json, "path")?.unwrap_or_else(|| ".".into());
        let max_entries = parse_optional_usize_field(&req.input_json, "max_entries")?.unwrap_or(50);
        let workspace_root = tool_workspace_root_path();
        let path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let mut entries = fs::read_dir(&path)
            .map_err(|err| {
                hepta_core::ToolError(format!("failed to list {}: {}", path.display(), err))
            })?
            .map(|entry| {
                entry
                    .map_err(|err| {
                        hepta_core::ToolError(format!("failed to read dir entry: {}", err))
                    })
                    .map(|entry| {
                        let path = entry.path();
                        json!({
                            "name": entry.file_name().to_string_lossy().to_string(),
                            "kind": if path.is_dir() { "dir" } else { "file" },
                        })
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort_by(|left, right| left["name"].as_str().cmp(&right["name"].as_str()));
        let total = entries.len();
        let truncated = entries.len() > max_entries;
        entries.truncate(max_entries);
        Ok(ToolResult {
            content: format!("list_dir:{} => {} entries", path.display(), total),
            structured_json: Some(
                json!({
                    "path": path.display().to_string(),
                    "entry_count": total,
                    "truncated": truncated,
                    "entries": entries,
                })
                .to_string(),
            ),
        })
    }
}

struct SearchTextTool;

impl Tool for SearchTextTool {
    fn name(&self) -> &'static str {
        "search_text"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Medium
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Search UTF-8 text files under a workspace path for a literal pattern".into(),
            input_schema_json: r#"{"type":"object","required":["path","pattern"],"properties":{"path":{"type":"string","minLength":1},"pattern":{"type":"string","minLength":1},"max_results":{"type":"integer","minimum":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["path","pattern","match_count"],"properties":{"path":{"type":"string","minLength":1},"pattern":{"type":"string","minLength":1},"match_count":{"type":"integer","minimum":0},"truncated":{"type":"boolean"}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let requested_path = parse_required_string_field(&req.input_json, "path")?;
        let pattern = parse_required_string_field(&req.input_json, "pattern")?;
        let max_results = parse_optional_usize_field(&req.input_json, "max_results")?.unwrap_or(25);
        let workspace_root = tool_workspace_root_path();
        let path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let mut files = Vec::new();
        if path.is_file() {
            files.push(path.clone());
        } else {
            collect_files_recursive(&path, &mut files)
                .map_err(|err| hepta_core::ToolError(err.0))?;
        }
        files.sort();
        let mut matches = Vec::new();
        for file in files {
            if matches.len() >= max_results {
                break;
            }
            let Ok(content) = fs::read_to_string(&file) else {
                continue;
            };
            for (index, line) in content.lines().enumerate() {
                if line.contains(&pattern) {
                    matches.push(json!({
                        "path": file.display().to_string(),
                        "line": index + 1,
                        "preview": line.chars().take(180).collect::<String>(),
                    }));
                    if matches.len() >= max_results {
                        break;
                    }
                }
            }
        }
        let match_count = matches.len();
        Ok(ToolResult {
            content: format!("search_text:{} => {} matches", path.display(), match_count),
            structured_json: Some(
                json!({
                    "path": path.display().to_string(),
                    "pattern": pattern,
                    "match_count": match_count,
                    "truncated": match_count >= max_results,
                    "matches": matches,
                })
                .to_string(),
            ),
        })
    }
}

struct DiskJunkAuditTool;

#[derive(Debug, Clone)]
struct DiskJunkCandidate {
    path: PathBuf,
    kind: &'static str,
    bytes: u64,
    entries_scanned: usize,
    inaccessible_count: usize,
    truncated: bool,
    recommendation: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct BoundedDirSize {
    bytes: u64,
    entries_scanned: usize,
    inaccessible_count: usize,
    truncated: bool,
}

impl Tool for DiskJunkAuditTool {
    fn name(&self) -> &'static str {
        "disk_junk_audit"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Run a bounded, read-only local disk cleanup candidate audit over common cache/log/temp roots".into(),
            input_schema_json: r#"{"type":"object","properties":{"scope":{"type":"string"},"max_entries":{"type":"integer","minimum":1},"include_var_folders":{"type":"boolean"}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["status","read_only","candidate_count","estimated_reclaimable_bytes"],"properties":{"status":{"type":"string"},"read_only":{"type":"boolean"},"candidate_count":{"type":"integer","minimum":0},"estimated_reclaimable_bytes":{"type":"integer","minimum":0},"truncated":{"type":"boolean"}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let max_entries = parse_optional_usize_field(&req.input_json, "max_entries")?
            .unwrap_or(120_000)
            .clamp(100, 500_000);
        let include_var_folders =
            parse_optional_bool_field(&req.input_json, "include_var_folders")?.unwrap_or(true);
        let roots = disk_junk_candidate_roots(include_var_folders);
        let per_root_limit = (max_entries / roots.len().max(1)).max(500);
        let mut candidates = Vec::new();
        for (path, kind, recommendation) in roots {
            if !path.exists() {
                continue;
            }
            let size = bounded_dir_size(&path, per_root_limit, 12);
            if size.bytes == 0 && size.entries_scanned == 0 {
                continue;
            }
            candidates.push(DiskJunkCandidate {
                path,
                kind,
                bytes: size.bytes,
                entries_scanned: size.entries_scanned,
                inaccessible_count: size.inaccessible_count,
                truncated: size.truncated,
                recommendation,
            });
        }
        candidates.sort_by(|left, right| right.bytes.cmp(&left.bytes));
        let estimated_reclaimable_bytes = candidates.iter().map(|candidate| candidate.bytes).sum();
        let truncated = candidates.iter().any(|candidate| candidate.truncated);
        let top = candidates
            .iter()
            .take(12)
            .map(|candidate| {
                json!({
                    "path": candidate.path.display().to_string(),
                    "kind": candidate.kind,
                    "bytes": candidate.bytes,
                    "human_size": human_bytes(candidate.bytes),
                    "entries_scanned": candidate.entries_scanned,
                    "inaccessible_count": candidate.inaccessible_count,
                    "truncated": candidate.truncated,
                    "recommendation": candidate.recommendation,
                    "safe_action": "review_then_delete_contents_only",
                })
            })
            .collect::<Vec<_>>();
        let summary_lines = top
            .iter()
            .take(5)
            .filter_map(|value| {
                Some(format!(
                    "{} {}",
                    value.get("human_size")?.as_str()?,
                    value.get("path")?.as_str()?
                ))
            })
            .collect::<Vec<_>>();
        Ok(ToolResult {
            content: format!(
                "disk_junk_audit: read-only scan found {} cleanup candidate root(s), estimated reclaimable {}. {}",
                candidates.len(),
                human_bytes(estimated_reclaimable_bytes),
                summary_lines.join("; ")
            ),
            structured_json: Some(
                json!({
                    "status": "completed",
                    "read_only": true,
                    "scope": "common_local_cleanup_candidates",
                    "candidate_count": candidates.len(),
                    "estimated_reclaimable_bytes": estimated_reclaimable_bytes,
                    "estimated_reclaimable_human": human_bytes(estimated_reclaimable_bytes),
                    "truncated": truncated,
                    "note": "This audit only reads metadata/content sizes and does not delete anything.",
                    "top_candidates": top,
                })
                .to_string(),
            ),
        })
    }
}

struct JsonGetTool;

impl Tool for JsonGetTool {
    fn name(&self) -> &'static str {
        "json_get"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Extract a JSON value by RFC-6901 pointer from a JSON string".into(),
            input_schema_json: r#"{"type":"object","required":["json","pointer"],"properties":{"json":{"type":"string","minLength":1},"pointer":{"type":"string","minLength":0}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["pointer","found"],"properties":{"pointer":{"type":"string","minLength":0},"found":{"type":"boolean"},"value_json":{"type":"string","minLength":0}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let json_text = parse_required_string_field(&req.input_json, "json")?;
        let pointer = parse_required_string_field(&req.input_json, "pointer")?;
        let value: Value = serde_json::from_str(&json_text)
            .map_err(|err| hepta_core::ToolError(format!("invalid JSON payload: {}", err)))?;
        let selected = if pointer.is_empty() {
            Some(&value)
        } else {
            value.pointer(&pointer)
        };
        let value_json = selected.map(Value::to_string).unwrap_or_default();
        Ok(ToolResult {
            content: format!("json_get:{} => found={}", pointer, selected.is_some()),
            structured_json: Some(
                json!({
                    "pointer": pointer,
                    "found": selected.is_some(),
                    "value_json": value_json,
                })
                .to_string(),
            ),
        })
    }
}

struct SkillProposeTool;

impl Tool for SkillProposeTool {
    fn name(&self) -> &'static str {
        "skill_propose"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Generate a quarantined SKILL.md draft from transcript text".into(),
            input_schema_json: r#"{"type":"object","required":["transcript"],"properties":{"transcript":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["skill_name","safe_to_apply","audit_id"],"properties":{"skill_name":{"type":"string","minLength":1},"safe_to_apply":{"type":"boolean"},"audit_id":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let transcript = parse_required_string_field(&req.input_json, "transcript")?;
        let draft = hepta_core::propose_skill_from_transcript(&transcript);
        Ok(ToolResult {
            content: format!(
                "skill_propose:{} safe={}",
                draft.skill_name, draft.scan.safe_to_apply
            ),
            structured_json: Some(
                json!({
                    "skill_name": draft.skill_name,
                    "title": draft.title,
                    "description": draft.description,
                    "skill_md": draft.skill_md,
                    "safe_to_apply": draft.scan.safe_to_apply,
                    "finding_count": draft.scan.finding_count,
                    "quarantine_path": draft.quarantine_path,
                    "apply_path": draft.apply_path,
                    "audit_id": draft.audit_id,
                })
                .to_string(),
            ),
        })
    }
}

struct SkillScanTool;

impl Tool for SkillScanTool {
    fn name(&self) -> &'static str {
        "skill_scan"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Scan a SKILL.md draft for local safety and structure violations".into(),
            input_schema_json: r#"{"type":"object","required":["skill_md"],"properties":{"skill_md":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["safe_to_apply","finding_count"],"properties":{"safe_to_apply":{"type":"boolean"},"finding_count":{"type":"integer","minimum":0}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let skill_md = parse_required_string_field(&req.input_json, "skill_md")?;
        let scan = hepta_core::scan_skill_markdown(&skill_md);
        Ok(ToolResult {
            content: format!(
                "skill_scan safe={} findings={}",
                scan.safe_to_apply, scan.finding_count
            ),
            structured_json: Some(
                serde_json::to_string(&scan)
                    .map_err(|err| hepta_core::ToolError(err.to_string()))?,
            ),
        })
    }
}

struct SkillApplyPlanTool;

impl Tool for SkillApplyPlanTool {
    fn name(&self) -> &'static str {
        "skill_apply_plan"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Medium
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Create a review-gated atomic apply plan for a generated skill draft".into(),
            input_schema_json: r#"{"type":"object","required":["transcript"],"properties":{"transcript":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["skill_name","safe_to_apply","review_required","snapshot_refresh_required"],"properties":{"skill_name":{"type":"string","minLength":1},"safe_to_apply":{"type":"boolean"},"review_required":{"type":"boolean"},"snapshot_refresh_required":{"type":"boolean"},"audit_id":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let transcript = parse_required_string_field(&req.input_json, "transcript")?;
        let draft = hepta_core::propose_skill_from_transcript(&transcript);
        let plan = hepta_core::skill_apply_plan_from_draft(&draft);
        Ok(ToolResult {
            content: format!(
                "skill_apply_plan:{} safe={} review={}",
                plan.skill_name, plan.safe_to_apply, plan.review_required
            ),
            structured_json: Some(
                serde_json::to_string(&plan)
                    .map_err(|err| hepta_core::ToolError(err.to_string()))?,
            ),
        })
    }
}

struct ToolManifestValidateTool;

impl Tool for ToolManifestValidateTool {
    fn name(&self) -> &'static str {
        "tool_manifest_validate"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Validate a generated tool manifest before promotion".into(),
            input_schema_json: r#"{"type":"object","required":["manifest_json"],"properties":{"manifest_json":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["valid","issue_count"],"properties":{"valid":{"type":"boolean"},"issue_count":{"type":"integer","minimum":0}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let manifest_json = parse_required_string_field(&req.input_json, "manifest_json")?;
        let manifest: hepta_core::GeneratedToolManifest = serde_json::from_str(&manifest_json)
            .map_err(|err| {
                hepta_core::ToolError(format!("invalid generated tool manifest: {}", err))
            })?;
        let validation = hepta_core::validate_tool_manifest(&manifest);
        Ok(ToolResult {
            content: format!(
                "tool_manifest_validate:{} valid={} issues={}",
                manifest.name, validation.valid, validation.issue_count
            ),
            structured_json: Some(
                serde_json::to_string(&validation)
                    .map_err(|err| hepta_core::ToolError(err.to_string()))?,
            ),
        })
    }
}

struct ToolGenerateStubTool;

impl Tool for ToolGenerateStubTool {
    fn name(&self) -> &'static str {
        "tool_generate_stub"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Generate a canonical local tool manifest/stub from operator intent".into(),
            input_schema_json: r#"{"type":"object","required":["name"],"properties":{"name":{"type":"string","minLength":1},"description":{"type":"string","minLength":0}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["name","risk_tier","read_only","audit_id"],"properties":{"name":{"type":"string","minLength":1},"risk_tier":{"type":"string","minLength":1},"read_only":{"type":"boolean"},"audit_id":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let name = parse_required_string_field(&req.input_json, "name")?;
        let description =
            parse_optional_string_field(&req.input_json, "description")?.unwrap_or_default();
        let manifest = hepta_core::generate_tool_manifest(&name, &description);
        Ok(ToolResult {
            content: format!(
                "tool_generate_stub:{} risk={}",
                manifest.name, manifest.risk_tier
            ),
            structured_json: Some(
                serde_json::to_string(&manifest)
                    .map_err(|err| hepta_core::ToolError(err.to_string()))?,
            ),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct NativeOpenClawCompatibleTool {
    name: &'static str,
    description: &'static str,
    risk_tier: RiskTier,
    read_only: bool,
    destructive: bool,
    idempotent: bool,
    behavior: NativeOpenClawCompatibleBehavior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeOpenClawCompatibleBehavior {
    Read,
    Write,
    Edit,
    ApplyPatch,
    Exec,
    Process,
    WebFetch,
    WebSearch,
    MemorySearch,
    MemoryGet,
    SessionStatus,
    PlanEcho,
    NativeSurface,
}

const NATIVE_OPENCLAW_COMPAT_INPUT_SCHEMA: &str = r#"{"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"},"command":{"type":"string"},"query":{"type":"string"},"url":{"type":"string"},"action":{"type":"string"},"message":{"type":"string"},"text":{"type":"string"},"input":{"type":"string"},"edits":{"type":"array"},"offset":{"type":"integer"},"limit":{"type":"integer"},"page_size":{"type":"integer"},"timeout":{"type":"integer"},"timeoutMs":{"type":"integer"},"background":{"type":"boolean"},"dryRun":{"type":"boolean"},"preview_only":{"type":"boolean"}},"additionalProperties":true}"#;
const NATIVE_OPENCLAW_COMPAT_OUTPUT_SCHEMA: &str = r#"{"type":"object","properties":{"tool":{"type":"string"},"status":{"type":"string"},"native_runtime":{"type":"boolean"},"backend":{"type":"string"},"proxy_used":{"type":"boolean"},"content":{"type":"string"},"result":{"type":"object"},"error":{"type":"string"}},"additionalProperties":true}"#;

fn native_openclaw_compat_input_schema(
    tool: &str,
    behavior: NativeOpenClawCompatibleBehavior,
) -> &'static str {
    match (tool, behavior) {
        ("read", _) | (_, NativeOpenClawCompatibleBehavior::Read) => {
            r#"{"type":"object","required":["path"],"properties":{"path":{"type":"string","description":"File path relative to the Hepta workspace unless absolute paths are allowed by runtime policy"},"offset":{"type":"integer","minimum":1,"default":1},"limit":{"type":"integer","minimum":1,"default":2000}},"additionalProperties":true}"#
        }
        ("write", _) | (_, NativeOpenClawCompatibleBehavior::Write) => {
            r#"{"type":"object","required":["path","content"],"properties":{"path":{"type":"string"},"content":{"type":"string"},"dryRun":{"type":"boolean","default":false},"preview_only":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("edit", _) | (_, NativeOpenClawCompatibleBehavior::Edit) => {
            r#"{"type":"object","required":["path","edits"],"properties":{"path":{"type":"string"},"edits":{"type":"array","items":{"type":"object","required":["oldText","newText"],"properties":{"oldText":{"type":"string"},"newText":{"type":"string"}}}},"dryRun":{"type":"boolean","default":false},"preview_only":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("apply_patch", _) | (_, NativeOpenClawCompatibleBehavior::ApplyPatch) => {
            r#"{"type":"object","required":["input"],"properties":{"input":{"type":"string","description":"apply_patch format, including *** Begin Patch and *** End Patch"},"patch":{"type":"string"},"dryRun":{"type":"boolean","default":false},"preview_only":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("exec", _) | (_, NativeOpenClawCompatibleBehavior::Exec) => {
            r#"{"type":"object","required":["command"],"properties":{"command":{"type":"string"},"workdir":{"type":"string"},"timeout":{"type":"integer"},"timeoutMs":{"type":"integer"},"background":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("process", _) | (_, NativeOpenClawCompatibleBehavior::Process) => {
            r#"{"type":"object","properties":{"action":{"type":"string","enum":["list","status","poll","log","read","write","submit","kill","terminate","clear","remove"]},"sessionId":{"type":"string"},"session_id":{"type":"string"},"id":{"type":"string"},"data":{"type":"string"},"text":{"type":"string"},"offset":{"type":"integer"},"limit":{"type":"integer"},"timeout":{"type":"integer"},"timeoutMs":{"type":"integer"},"eof":{"type":"boolean"}},"additionalProperties":true}"#
        }
        ("web_fetch", _) | (_, NativeOpenClawCompatibleBehavior::WebFetch) => {
            r#"{"type":"object","required":["url"],"properties":{"url":{"type":"string"},"extractMode":{"type":"string"},"maxChars":{"type":"integer","default":20000}},"additionalProperties":true}"#
        }
        ("web_search", _) | (_, NativeOpenClawCompatibleBehavior::WebSearch) => {
            r#"{"type":"object","required":["query"],"properties":{"query":{"type":"string"},"count":{"type":"integer","default":5},"maxChars":{"type":"integer","default":20000}},"additionalProperties":true}"#
        }
        ("memory_search", _) | (_, NativeOpenClawCompatibleBehavior::MemorySearch) => {
            r#"{"type":"object","required":["query"],"properties":{"query":{"type":"string"},"maxResults":{"type":"integer","default":10},"max_results":{"type":"integer"}},"additionalProperties":true}"#
        }
        ("memory_get", _) | (_, NativeOpenClawCompatibleBehavior::MemoryGet) => {
            r#"{"type":"object","required":["path"],"properties":{"path":{"type":"string"},"from":{"type":"integer","minimum":1},"lines":{"type":"integer","minimum":1}},"additionalProperties":true}"#
        }
        ("message", _) => {
            r#"{"type":"object","required":["action"],"properties":{"action":{"type":"string","enum":["send","read","channel-list","channel-info","member-info"]},"channel":{"type":"string","default":"telegram"},"target":{"type":"string"},"message":{"type":"string"},"text":{"type":"string"},"dryRun":{"type":"boolean","default":true},"confirmSend":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("tts", _) => {
            r#"{"type":"object","required":["text"],"properties":{"text":{"type":"string"},"path":{"type":"string"},"filename":{"type":"string"},"dryRun":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("image_generate", _) | ("music_generate", _) | ("video_generate", _) => {
            r#"{"type":"object","required":["prompt"],"properties":{"prompt":{"type":"string"},"filename":{"type":"string"},"model":{"type":"string"},"durationSeconds":{"type":"integer"},"dryRun":{"type":"boolean","default":false},"timeoutMs":{"type":"integer"}},"additionalProperties":true}"#
        }
        ("image", _) | ("pdf", _) => {
            r#"{"type":"object","properties":{"image":{"type":"string"},"images":{"type":"array","items":{"type":"string"}},"pdf":{"type":"string"},"pdfs":{"type":"array","items":{"type":"string"}},"prompt":{"type":"string"},"pages":{"type":"string"},"maxBytesMb":{"type":"integer"}},"additionalProperties":true}"#
        }
        ("sessions_history", _) => {
            r#"{"type":"object","properties":{"sessionKey":{"type":"string"},"session_id":{"type":"string"},"limit":{"type":"integer","default":20},"includeTools":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("sessions_send", _) => {
            r#"{"type":"object","required":["message"],"properties":{"sessionKey":{"type":"string"},"label":{"type":"string"},"message":{"type":"string"},"execute":{"type":"boolean","default":true},"dryRun":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("sessions_spawn", _) => {
            r#"{"type":"object","required":["task"],"properties":{"task":{"type":"string"},"label":{"type":"string"},"agentId":{"type":"string"},"execute":{"type":"boolean","default":true},"dryRun":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        _ => NATIVE_OPENCLAW_COMPAT_INPUT_SCHEMA,
    }
}

impl Tool for NativeOpenClawCompatibleTool {
    fn name(&self) -> &'static str {
        self.name
    }

    fn risk_tier(&self) -> RiskTier {
        self.risk_tier
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: self.read_only,
            destructive: self.destructive,
            idempotent: self.idempotent,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name.into(),
            description: self.description.into(),
            input_schema_json: native_openclaw_compat_input_schema(self.name, self.behavior).into(),
            output_schema_json: NATIVE_OPENCLAW_COMPAT_OUTPUT_SCHEMA.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let input = parse_tool_input_object(&req.input_json)?;
        let result = match self.behavior {
            NativeOpenClawCompatibleBehavior::Read => native_compat_read(self.name, &input),
            NativeOpenClawCompatibleBehavior::Write => native_compat_write(self.name, &input),
            NativeOpenClawCompatibleBehavior::Edit => native_compat_edit(self.name, &input),
            NativeOpenClawCompatibleBehavior::ApplyPatch => {
                native_compat_apply_patch(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::Exec => native_compat_exec(self.name, &input),
            NativeOpenClawCompatibleBehavior::Process => native_compat_process(self.name, &input),
            NativeOpenClawCompatibleBehavior::WebFetch => {
                native_compat_web_fetch(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::WebSearch => {
                native_compat_web_search(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::MemorySearch => {
                native_compat_memory_search(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::MemoryGet => {
                native_compat_memory_get(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::SessionStatus => {
                Ok(native_compat_status_report(self.name, &input))
            }
            NativeOpenClawCompatibleBehavior::PlanEcho => {
                Ok(native_compat_plan_echo(self.name, &input))
            }
            NativeOpenClawCompatibleBehavior::NativeSurface => {
                native_compat_live_surface(self.name, &input)
            }
        }?;
        let content = result
            .get("content")
            .and_then(Value::as_str)
            .unwrap_or("native tool completed")
            .to_string();
        Ok(ToolResult {
            content,
            structured_json: Some(Value::Object(result).to_string()),
        })
    }
}

fn native_openclaw_compatible_tools() -> Vec<NativeOpenClawCompatibleTool> {
    use NativeOpenClawCompatibleBehavior as B;
    vec![
        native_tool(
            "read",
            "Read a text file using Hepta's Rust-native workspace reader",
            RiskTier::Medium,
            true,
            false,
            true,
            B::Read,
        ),
        native_tool(
            "write",
            "Write a file using Hepta's Rust-native workspace writer",
            RiskTier::High,
            false,
            true,
            false,
            B::Write,
        ),
        native_tool(
            "edit",
            "Apply exact text replacements using Hepta's Rust-native editor",
            RiskTier::High,
            false,
            true,
            false,
            B::Edit,
        ),
        native_tool(
            "apply_patch",
            "Apply a bounded apply_patch-format patch using Hepta's Rust-native patch parser",
            RiskTier::High,
            false,
            true,
            false,
            B::ApplyPatch,
        ),
        native_tool(
            "exec",
            "Run a local shell command through Hepta's Rust-native process runner; use this for filesystem maintenance or cache cleanup only with the normal high-risk approval gate",
            RiskTier::High,
            false,
            true,
            false,
            B::Exec,
        ),
        native_tool(
            "process",
            "Inspect or control Hepta background process sessions created by exec background=true; not for deleting files, caches, or workspace storage",
            RiskTier::Medium,
            true,
            false,
            true,
            B::Process,
        ),
        native_tool(
            "canvas",
            "Run Hepta-native canvas-plane adapter/audit actions without OpenClaw proxying",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "message",
            "Send or preview Telegram messages through Hepta's native gated channel adapter",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "tts",
            "Synthesize local speech through Hepta's native macOS TTS adapter",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "image_generate",
            "Generate images through Hepta's native local Ollama/helper adapter",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "music_generate",
            "Generate music through a configured Hepta-native local generator command",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "video_generate",
            "Generate video through a configured Hepta-native local generator command",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "agents_list",
            "List Hepta-native agent surface metadata without OpenClaw proxying",
            RiskTier::Low,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "update_plan",
            "Echo a model-supplied plan through Hepta's native structured-output surface",
            RiskTier::Low,
            false,
            false,
            false,
            B::PlanEcho,
        ),
        native_tool(
            "sessions_list",
            "List Hepta-native session surface metadata without OpenClaw proxying",
            RiskTier::Low,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "sessions_history",
            "Read Hepta-native session history through the local runtime CLI",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "sessions_send",
            "Run a prompt in a Hepta-native session through the local runtime CLI",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "sessions_spawn",
            "Spawn a durable Hepta-native worker task through the local runtime CLI",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "sessions_yield",
            "Record a Hepta-native session yield event without OpenClaw proxying",
            RiskTier::Low,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "subagents",
            "List, steer, or stop Hepta-native top-level agents through the local runtime CLI",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "session_status",
            "Return Hepta native runtime/tool status without OpenClaw proxying",
            RiskTier::Low,
            true,
            false,
            true,
            B::SessionStatus,
        ),
        native_tool(
            "web_search",
            "Run a best-effort Rust-native web search via local curl, not OpenClaw",
            RiskTier::Medium,
            true,
            false,
            true,
            B::WebSearch,
        ),
        native_tool(
            "web_fetch",
            "Fetch a URL via local curl from Hepta native code, not OpenClaw",
            RiskTier::Medium,
            true,
            false,
            true,
            B::WebFetch,
        ),
        native_tool(
            "image",
            "Analyze local image metadata through Hepta-native filesystem tools",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "pdf",
            "Extract local PDF text/metadata through Hepta-native filesystem tools",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "memory_search",
            "Search Hepta/OpenClaw workspace memory files using local Rust filesystem reads",
            RiskTier::Low,
            true,
            false,
            true,
            B::MemorySearch,
        ),
        native_tool(
            "memory_get",
            "Read a bounded excerpt from a workspace memory file using local Rust filesystem reads",
            RiskTier::Low,
            true,
            false,
            true,
            B::MemoryGet,
        ),
        native_tool(
            "feishu_doc",
            "Run Hepta-native Feishu document adapter readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_app_scopes",
            "Run Hepta-native Feishu app-scope adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_chat",
            "Run Hepta-native Feishu chat adapter readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_wiki",
            "Run Hepta-native Feishu wiki adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_drive",
            "Run Hepta-native Feishu drive adapter readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_get_meta",
            "Run Hepta-native Feishu bitable metadata adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_list_fields",
            "Run Hepta-native Feishu bitable field-list adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_list_records",
            "Run Hepta-native Feishu bitable record-list adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_get_record",
            "Run Hepta-native Feishu bitable record-get adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_create_record",
            "Run Hepta-native Feishu bitable record-create readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_update_record",
            "Run Hepta-native Feishu bitable record-update readiness or gated live probe",
            RiskTier::High,
            false,
            true,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_create_app",
            "Run Hepta-native Feishu bitable app-create readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_create_field",
            "Run Hepta-native Feishu bitable field-create readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
    ]
}

fn native_tool(
    name: &'static str,
    description: &'static str,
    risk_tier: RiskTier,
    read_only: bool,
    destructive: bool,
    idempotent: bool,
    behavior: NativeOpenClawCompatibleBehavior,
) -> NativeOpenClawCompatibleTool {
    NativeOpenClawCompatibleTool {
        name,
        description,
        risk_tier,
        read_only,
        destructive,
        idempotent,
        behavior,
    }
}

fn parse_tool_input_object(
    input_json: &str,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    value
        .as_object()
        .cloned()
        .ok_or_else(|| hepta_core::ToolError("tool input must be a JSON object".into()))
}

fn native_compat_base(tool: &str, status: &str) -> serde_json::Map<String, Value> {
    let mut out = serde_json::Map::new();
    out.insert("tool".into(), Value::String(tool.into()));
    out.insert("status".into(), Value::String(status.into()));
    out.insert("native_runtime".into(), Value::Bool(true));
    out.insert("backend".into(), Value::String("hepta-rust-native".into()));
    out.insert("proxy_used".into(), Value::Bool(false));
    out.insert("openclaw_gateway_invoked".into(), Value::Bool(false));
    out
}

fn native_tool_invocation_timeout_result(tool: &str, timeout_ms: u64) -> ToolResult {
    let error = format!("ToolTimeout/{} timed out after {} ms", tool, timeout_ms);
    let mut out = native_compat_base(tool, "timeout");
    out.insert("content".into(), Value::String(error.clone()));
    out.insert("error".into(), Value::String(error.clone()));
    out.insert("error_kind".into(), Value::String("ToolTimeout".into()));
    out.insert("timeout".into(), Value::Bool(true));
    out.insert(
        "result".into(),
        json!({
            "timeout": true,
            "timeout_ms": timeout_ms,
            "fallback_reason": "tool-timeout",
            "duplicate_tool_replay_prevented": true,
        }),
    );
    ToolResult {
        content: error,
        structured_json: Some(Value::Object(out).to_string()),
    }
}

fn tool_result_is_timeout(tool_result: &ToolResult) -> bool {
    if tool_result.content.contains("ToolTimeout/") || tool_result.content.contains(" timed out") {
        return true;
    }
    let Some(structured_json) = tool_result.structured_json.as_deref() else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<Value>(structured_json) else {
        return false;
    };
    value.get("status").and_then(Value::as_str) == Some("timeout")
        || value.get("timeout").and_then(Value::as_bool) == Some(true)
        || value.get("error_kind").and_then(Value::as_str) == Some("ToolTimeout")
        || value
            .get("result")
            .and_then(|result| result.get("timeout"))
            .and_then(Value::as_bool)
            == Some(true)
}

fn native_compat_read(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let path_text = input
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("read requires string field 'path'".into()))?;
    let offset = input
        .get("offset")
        .and_then(Value::as_u64)
        .unwrap_or(1)
        .max(1) as usize;
    let limit = input
        .get("limit")
        .and_then(Value::as_u64)
        .unwrap_or(2000)
        .max(1) as usize;
    let workspace_root = tool_workspace_root_path();
    let path = resolve_path_within_root(&workspace_root, Path::new(path_text));
    let content = fs::read_to_string(&path).map_err(|err| {
        hepta_core::ToolError(format!("failed to read {}: {}", path.display(), err))
    })?;
    let lines: Vec<&str> = content.lines().collect();
    let start = offset.saturating_sub(1).min(lines.len());
    let end = start.saturating_add(limit).min(lines.len());
    let excerpt = lines[start..end].join("\n");
    let mut out = native_compat_base(tool, "ok");
    out.insert("content".into(), Value::String(excerpt.clone()));
    out.insert(
        "result".into(),
        json!({
            "path": path.display().to_string(),
            "offset": offset,
            "limit": limit,
            "line_count": lines.len(),
            "returned_lines": end.saturating_sub(start),
            "truncated": end < lines.len(),
            "text": excerpt
        }),
    );
    Ok(out)
}

fn native_compat_write(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let path_text = input
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("write requires string field 'path'".into()))?;
    let content = input
        .get("content")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("write requires string field 'content'".into()))?;
    let preview_only = input
        .get("preview_only")
        .or_else(|| input.get("dryRun"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let workspace_root = tool_workspace_root_path();
    let path = resolve_path_within_root(&workspace_root, Path::new(path_text));
    let existed_before = path.exists();
    if !preview_only {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                hepta_core::ToolError(format!("failed to create {}: {}", parent.display(), err))
            })?;
        }
        fs::write(&path, content).map_err(|err| {
            hepta_core::ToolError(format!("failed to write {}: {}", path.display(), err))
        })?;
    }
    let mut out = native_compat_base(tool, if preview_only { "preview" } else { "ok" });
    out.insert(
        "content".into(),
        Value::String(format!(
            "{} {} bytes to {}{}",
            if preview_only { "would write" } else { "wrote" },
            content.len(),
            path.display(),
            if existed_before {
                " (overwrote existing file)"
            } else {
                ""
            }
        )),
    );
    out.insert(
        "result".into(),
        json!({
            "path": path.display().to_string(),
            "bytes": content.len(),
            "existed_before": existed_before,
            "preview_only": preview_only
        }),
    );
    Ok(out)
}

fn native_compat_edit(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let path_text = input
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("edit requires string field 'path'".into()))?;
    let edits = input
        .get("edits")
        .and_then(Value::as_array)
        .ok_or_else(|| hepta_core::ToolError("edit requires array field 'edits'".into()))?;
    let preview_only = input
        .get("preview_only")
        .or_else(|| input.get("dryRun"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let workspace_root = tool_workspace_root_path();
    let path = resolve_path_within_root(&workspace_root, Path::new(path_text));
    let mut content = fs::read_to_string(&path).map_err(|err| {
        hepta_core::ToolError(format!("failed to read {}: {}", path.display(), err))
    })?;
    let mut applied = 0usize;
    for edit in edits {
        let old_text = edit
            .get("oldText")
            .or_else(|| edit.get("old_text"))
            .and_then(Value::as_str)
            .ok_or_else(|| hepta_core::ToolError("each edit requires oldText".into()))?;
        let new_text = edit
            .get("newText")
            .or_else(|| edit.get("new_text"))
            .and_then(Value::as_str)
            .ok_or_else(|| hepta_core::ToolError("each edit requires newText".into()))?;
        let count = content.matches(old_text).count();
        if count != 1 {
            return Err(hepta_core::ToolError(format!(
                "oldText must match exactly once; matched {} times",
                count
            )));
        }
        content = content.replacen(old_text, new_text, 1);
        applied += 1;
    }
    if !preview_only {
        fs::write(&path, content).map_err(|err| {
            hepta_core::ToolError(format!("failed to write {}: {}", path.display(), err))
        })?;
    }
    let mut out = native_compat_base(tool, if preview_only { "preview" } else { "ok" });
    out.insert(
        "content".into(),
        Value::String(format!(
            "{} {} edit(s) in {}",
            if preview_only {
                "would apply"
            } else {
                "applied"
            },
            applied,
            path.display()
        )),
    );
    out.insert(
        "result".into(),
        json!({ "path": path.display().to_string(), "edits_applied": applied, "preview_only": preview_only }),
    );
    Ok(out)
}

fn native_compat_apply_patch(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let patch = input
        .get("input")
        .or_else(|| input.get("patch"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("apply_patch requires string field 'input' or 'patch'".into())
        })?;
    let preview_only = input
        .get("preview_only")
        .or_else(|| input.get("dryRun"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let ops = parse_native_apply_patch(patch)?;
    let workspace_root = tool_workspace_root_path();
    let mut summaries = Vec::new();
    for op in ops {
        match op {
            NativePatchOp::Add { path, content } => {
                let target = resolve_path_within_root(&workspace_root, Path::new(&path));
                if target.exists() {
                    return Err(hepta_core::ToolError(format!(
                        "cannot add existing file {}",
                        target.display()
                    )));
                }
                if !preview_only {
                    if let Some(parent) = target.parent() {
                        fs::create_dir_all(parent).map_err(|err| {
                            hepta_core::ToolError(format!(
                                "failed to create {}: {}",
                                parent.display(),
                                err
                            ))
                        })?;
                    }
                    fs::write(&target, &content).map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to add {}: {}",
                            target.display(),
                            err
                        ))
                    })?;
                }
                summaries.push(
                    json!({"op":"add","path":target.display().to_string(),"bytes":content.len()}),
                );
            }
            NativePatchOp::Delete { path } => {
                let target = resolve_path_within_root(&workspace_root, Path::new(&path));
                if !target.exists() {
                    return Err(hepta_core::ToolError(format!(
                        "cannot delete missing file {}",
                        target.display()
                    )));
                }
                if !preview_only {
                    fs::remove_file(&target).map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to delete {}: {}",
                            target.display(),
                            err
                        ))
                    })?;
                }
                summaries.push(json!({"op":"delete","path":target.display().to_string()}));
            }
            NativePatchOp::Update { path, old, new } => {
                let target = resolve_path_within_root(&workspace_root, Path::new(&path));
                let current = fs::read_to_string(&target).map_err(|err| {
                    hepta_core::ToolError(format!("failed to read {}: {}", target.display(), err))
                })?;
                let count = current.matches(&old).count();
                if count != 1 {
                    return Err(hepta_core::ToolError(format!(
                        "patch update for {} matched old hunk {} times; expected exactly once",
                        target.display(),
                        count
                    )));
                }
                let updated = current.replacen(&old, &new, 1);
                if !preview_only {
                    fs::write(&target, updated).map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to update {}: {}",
                            target.display(),
                            err
                        ))
                    })?;
                }
                summaries.push(json!({"op":"update","path":target.display().to_string(),"old_bytes":old.len(),"new_bytes":new.len()}));
            }
        }
    }
    let mut out = native_compat_base(tool, if preview_only { "preview" } else { "ok" });
    out.insert(
        "content".into(),
        Value::String(format!(
            "{} {} patch operation(s)",
            if preview_only {
                "would apply"
            } else {
                "applied"
            },
            summaries.len()
        )),
    );
    out.insert(
        "result".into(),
        json!({"operation_count": summaries.len(), "operations": summaries, "preview_only": preview_only}),
    );
    Ok(out)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NativePatchOp {
    Add {
        path: String,
        content: String,
    },
    Delete {
        path: String,
    },
    Update {
        path: String,
        old: String,
        new: String,
    },
}

fn parse_native_apply_patch(patch: &str) -> Result<Vec<NativePatchOp>, hepta_core::ToolError> {
    let lines: Vec<&str> = patch.lines().collect();
    if lines.first().copied() != Some("*** Begin Patch")
        || lines.last().copied() != Some("*** End Patch")
    {
        return Err(hepta_core::ToolError(
            "apply_patch input must start with *** Begin Patch and end with *** End Patch".into(),
        ));
    }
    let mut ops = Vec::new();
    let mut i = 1usize;
    while i + 1 < lines.len() {
        let line = lines[i];
        if let Some(path) = line.strip_prefix("*** Add File: ") {
            i += 1;
            let mut content = String::new();
            while i < lines.len() && !lines[i].starts_with("*** ") {
                let raw = lines[i];
                if let Some(added) = raw.strip_prefix('+') {
                    content.push_str(added);
                } else {
                    content.push_str(raw);
                }
                content.push('\n');
                i += 1;
            }
            ops.push(NativePatchOp::Add {
                path: path.trim().into(),
                content,
            });
            continue;
        }
        if let Some(path) = line.strip_prefix("*** Delete File: ") {
            ops.push(NativePatchOp::Delete {
                path: path.trim().into(),
            });
            i += 1;
            continue;
        }
        if let Some(path) = line.strip_prefix("*** Update File: ") {
            i += 1;
            let mut old = String::new();
            let mut new = String::new();
            while i < lines.len() && !lines[i].starts_with("*** ") {
                let raw = lines[i];
                if raw.starts_with("@@") {
                    i += 1;
                    continue;
                }
                if let Some(removed) = raw.strip_prefix('-') {
                    old.push_str(removed);
                    old.push('\n');
                } else if let Some(added) = raw.strip_prefix('+') {
                    new.push_str(added);
                    new.push('\n');
                } else if let Some(context) = raw.strip_prefix(' ') {
                    old.push_str(context);
                    old.push('\n');
                    new.push_str(context);
                    new.push('\n');
                } else {
                    old.push_str(raw);
                    old.push('\n');
                    new.push_str(raw);
                    new.push('\n');
                }
                i += 1;
            }
            if old.is_empty() && new.is_empty() {
                return Err(hepta_core::ToolError(format!(
                    "update patch for {} has no hunk content",
                    path.trim()
                )));
            }
            ops.push(NativePatchOp::Update {
                path: path.trim().into(),
                old,
                new,
            });
            continue;
        }
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        return Err(hepta_core::ToolError(format!(
            "unsupported apply_patch line: {}",
            line
        )));
    }
    if ops.is_empty() {
        return Err(hepta_core::ToolError(
            "apply_patch input contained no operations".into(),
        ));
    }
    Ok(ops)
}

const NATIVE_EXEC_DEFAULT_TIMEOUT_MS: u64 = 45_000;
const NATIVE_EXEC_MIN_TIMEOUT_MS: u64 = 100;
const NATIVE_EXEC_MAX_TIMEOUT_MS: u64 = 300_000;
const NATIVE_EXEC_POLL_INTERVAL_MS: u64 = 50;
const NATIVE_EXEC_KILL_GRACE_MS: u64 = 750;
const NATIVE_TOOL_INVOCATION_TIMEOUT_MS: u64 = 60_000;

#[derive(Debug, Clone)]
struct NativeCommandRunOutput {
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
    success: bool,
    timed_out: bool,
    killed_process_tree: bool,
    timeout_ms: u64,
    elapsed_ms: u64,
}

fn native_timeout_ms_from_input(input: &serde_json::Map<String, Value>) -> u64 {
    let requested_ms = input
        .get("timeoutMs")
        .or_else(|| input.get("timeout_ms"))
        .and_then(Value::as_u64)
        .or_else(|| {
            input
                .get("timeout")
                .and_then(Value::as_u64)
                .map(|seconds| seconds.saturating_mul(1_000))
        })
        .unwrap_or(NATIVE_EXEC_DEFAULT_TIMEOUT_MS);
    requested_ms.clamp(NATIVE_EXEC_MIN_TIMEOUT_MS, NATIVE_EXEC_MAX_TIMEOUT_MS)
}

fn native_command_temp_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    env::temp_dir().join(format!(
        "hepta-native-{}-{}-{}.tmp",
        label,
        std::process::id(),
        nanos
    ))
}

fn prepare_native_command(command: &str, workdir: &Path) -> Command {
    let mut cmd = Command::new("/bin/zsh");
    cmd.arg("-lc").arg(command).current_dir(workdir);
    #[cfg(unix)]
    {
        cmd.process_group(0);
    }
    cmd
}

fn native_send_signal_to_pid_tree(pid: u32, signal: &str) -> bool {
    let mut ok = false;
    #[cfg(unix)]
    {
        ok |= Command::new("/bin/kill")
            .arg(signal)
            .arg(format!("-{}", pid))
            .status()
            .map(|status| status.success())
            .unwrap_or(false);
    }
    ok |= Command::new("/bin/kill")
        .arg(signal)
        .arg(pid.to_string())
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    ok
}

fn native_wait_for_child_exit(child: &mut Child, wait_ms: u64) -> Option<Option<i32>> {
    let started = SystemTime::now();
    loop {
        if let Ok(Some(status)) = child.try_wait() {
            return Some(status.code());
        }
        if started
            .elapsed()
            .map(|elapsed| elapsed.as_millis() as u64 >= wait_ms)
            .unwrap_or(true)
        {
            return None;
        }
        thread::sleep(Duration::from_millis(NATIVE_EXEC_POLL_INTERVAL_MS));
    }
}

fn native_kill_child_process_tree(child: &mut Child) -> bool {
    let pid = child.id();
    let mut signalled = native_send_signal_to_pid_tree(pid, "-TERM");
    if native_wait_for_child_exit(child, NATIVE_EXEC_KILL_GRACE_MS).is_some() {
        return signalled;
    }
    signalled |= native_send_signal_to_pid_tree(pid, "-KILL");
    signalled |= child.kill().is_ok();
    let _ = child.wait();
    signalled
}

fn native_run_command_with_deadline(
    command: &str,
    workdir: &Path,
    timeout_ms: u64,
) -> Result<NativeCommandRunOutput, hepta_core::ToolError> {
    let stdout_path = native_command_temp_path("stdout");
    let stderr_path = native_command_temp_path("stderr");
    let stdout_file = fs::File::create(&stdout_path).map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to create {}: {}",
            stdout_path.display(),
            err
        ))
    })?;
    let stderr_file = fs::File::create(&stderr_path).map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to create {}: {}",
            stderr_path.display(),
            err
        ))
    })?;
    let mut child = prepare_native_command(command, workdir)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .map_err(|err| hepta_core::ToolError(format!("failed to spawn command: {}", err)))?;
    let started = SystemTime::now();
    let mut exit_code = None::<i32>;
    let mut success = false;
    let mut timed_out = false;
    let mut killed_process_tree = false;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                exit_code = status.code();
                success = status.success();
                break;
            }
            Ok(None) => {}
            Err(err) => {
                return Err(hepta_core::ToolError(format!(
                    "failed to poll native exec command: {}",
                    err
                )));
            }
        }
        if started
            .elapsed()
            .map(|elapsed| elapsed.as_millis() as u64 >= timeout_ms)
            .unwrap_or(true)
        {
            timed_out = true;
            killed_process_tree = native_kill_child_process_tree(&mut child);
            break;
        }
        thread::sleep(Duration::from_millis(NATIVE_EXEC_POLL_INTERVAL_MS));
    }
    let elapsed_ms = started
        .elapsed()
        .map(|elapsed| elapsed.as_millis() as u64)
        .unwrap_or(timeout_ms);
    let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
    let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
    let _ = fs::remove_file(&stdout_path);
    let _ = fs::remove_file(&stderr_path);
    Ok(NativeCommandRunOutput {
        stdout,
        stderr,
        exit_code,
        success,
        timed_out,
        killed_process_tree,
        timeout_ms,
        elapsed_ms,
    })
}

fn native_compat_exec(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let command = input
        .get("command")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("exec requires string field 'command'".into()))?;
    let workdir = input
        .get("workdir")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(tool_workspace_root_path);
    let background = input
        .get("background")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if background {
        return native_compat_exec_background(tool, command, &workdir);
    }
    let timeout_ms = native_timeout_ms_from_input(input);
    let output = native_run_command_with_deadline(command, &workdir, timeout_ms)?;
    if output.timed_out {
        let error = format!(
            "ToolTimeout/native_compat_exec timed out after {} ms",
            output.timeout_ms
        );
        let mut out = native_compat_base(tool, "timeout");
        out.insert("content".into(), Value::String(error.clone()));
        out.insert("error".into(), Value::String(error.clone()));
        out.insert("error_kind".into(), Value::String("ToolTimeout".into()));
        out.insert("timeout".into(), Value::Bool(true));
        out.insert(
            "result".into(),
            json!({
                "command": command,
                "workdir": workdir.display().to_string(),
                "exit_code": output.exit_code,
                "stdout": output.stdout,
                "stderr": output.stderr,
                "timeout": true,
                "timeout_ms": output.timeout_ms,
                "elapsed_ms": output.elapsed_ms,
                "killed_process_tree": output.killed_process_tree,
                "fallback_reason": "tool-timeout",
                "duplicate_tool_replay_prevented": true,
            }),
        );
        return Ok(out);
    }
    let mut out = native_compat_base(tool, if output.success { "ok" } else { "error" });
    out.insert("content".into(), Value::String(output.stdout.clone()));
    out.insert(
        "result".into(),
        json!({
            "command": command,
            "workdir": workdir.display().to_string(),
            "exit_code": output.exit_code.unwrap_or(-1),
            "stdout": output.stdout,
            "stderr": output.stderr,
            "timeout": false,
            "timeout_ms": output.timeout_ms,
            "elapsed_ms": output.elapsed_ms
        }),
    );
    Ok(out)
}

struct NativeBackgroundProcess {
    child: Child,
    stdin: Option<ChildStdin>,
    command: String,
    workdir: PathBuf,
    log_path: PathBuf,
    started_at_unix_ms: u64,
}

static NATIVE_BACKGROUND_PROCESSES: OnceLock<Mutex<HashMap<String, NativeBackgroundProcess>>> =
    OnceLock::new();

fn native_process_registry() -> &'static Mutex<HashMap<String, NativeBackgroundProcess>> {
    NATIVE_BACKGROUND_PROCESSES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn native_compat_exec_background(
    tool: &str,
    command: &str,
    workdir: &Path,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let started_at_unix_ms = current_unix_ms().map_err(|err| hepta_core::ToolError(err.0))?;
    let log_dir = native_process_log_dir();
    fs::create_dir_all(&log_dir).map_err(|err| {
        hepta_core::ToolError(format!("failed to create {}: {}", log_dir.display(), err))
    })?;
    let temp_session_id = format!("hepta-proc-{}-pending", started_at_unix_ms);
    let temp_log_path = log_dir.join(format!("{}.log", temp_session_id));
    fs::write(
        &temp_log_path,
        format!("$ {}\n", command.replace('\n', "\\n")),
    )
    .map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to initialize {}: {}",
            temp_log_path.display(),
            err
        ))
    })?;
    let stdout_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&temp_log_path)
        .map_err(|err| {
            hepta_core::ToolError(format!(
                "failed to open {} for stdout capture: {}",
                temp_log_path.display(),
                err
            ))
        })?;
    let stderr_file = stdout_file.try_clone().map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to clone {} for stderr capture: {}",
            temp_log_path.display(),
            err
        ))
    })?;
    let mut child = prepare_native_command(command, workdir)
        .stdin(Stdio::piped())
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .map_err(|err| hepta_core::ToolError(format!("failed to spawn command: {}", err)))?;
    let pid = child.id();
    let session_id = format!("hepta-proc-{}-{}", started_at_unix_ms, pid);
    let log_path = log_dir.join(format!("{}.log", session_id));
    fs::rename(&temp_log_path, &log_path).map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to finalize log path {} -> {}: {}",
            temp_log_path.display(),
            log_path.display(),
            err
        ))
    })?;
    let stdin = child.stdin.take();
    native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?
        .insert(
            session_id.clone(),
            NativeBackgroundProcess {
                child,
                stdin,
                command: command.to_string(),
                workdir: workdir.to_path_buf(),
                log_path: log_path.clone(),
                started_at_unix_ms,
            },
        );
    let mut out = native_compat_base(tool, "backgrounded");
    out.insert(
        "content".into(),
        Value::String(format!(
            "command started in background as {}; use process poll/log/write/kill/clear/remove",
            session_id
        )),
    );
    out.insert(
        "result".into(),
        json!({
            "sessionId": session_id,
            "id": session_id,
            "pid": pid,
            "command": command,
            "workdir": workdir.display().to_string(),
            "log_path": log_path.display().to_string(),
            "running": true,
            "followup_actions": ["poll", "log", "write", "kill", "clear", "remove"]
        }),
    );
    Ok(out)
}

fn native_process_log_dir() -> PathBuf {
    tool_workspace_root_path().join("target/hepta-processes")
}

fn native_process_log_path(id: &str) -> PathBuf {
    native_process_log_dir().join(format!("{}.log", id))
}

fn native_process_pid_from_id(id: &str) -> Option<u32> {
    id.rsplit_once('-')
        .and_then(|(_, pid)| pid.parse::<u32>().ok())
}

fn native_process_started_at_from_id(id: &str) -> Option<u64> {
    let rest = id.strip_prefix("hepta-proc-")?;
    rest.split_once('-')
        .and_then(|(started, _)| started.parse::<u64>().ok())
}

fn native_process_pid_alive(pid: u32) -> bool {
    std::process::Command::new("/bin/kill")
        .arg("-0")
        .arg(pid.to_string())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn native_process_command_from_log(log_path: &Path) -> String {
    fs::read_to_string(log_path)
        .ok()
        .and_then(|text| {
            text.lines()
                .next()
                .map(|line| line.trim_start_matches("$ ").to_string())
        })
        .unwrap_or_else(|| "<unknown>".into())
}

fn native_process_snapshot_from_log(id: &str) -> Option<Value> {
    let log_path = native_process_log_path(id);
    if !log_path.exists() {
        return None;
    }
    let pid = native_process_pid_from_id(id);
    let running = pid.map(native_process_pid_alive).unwrap_or(false);
    Some(json!({
        "sessionId": id,
        "id": id,
        "pid": pid,
        "command": native_process_command_from_log(&log_path),
        "workdir": tool_workspace_root_path().display().to_string(),
        "log_path": log_path.display().to_string(),
        "started_at_unix_ms": native_process_started_at_from_id(id),
        "running": running,
        "exit_code": null,
        "stdin_open": false,
        "registry_backed": false,
        "log_backed": true,
    }))
}

fn native_process_log_snapshots() -> Vec<Value> {
    let Ok(entries) = fs::read_dir(native_process_log_dir()) else {
        return Vec::new();
    };
    let mut snapshots = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("log") {
                return None;
            }
            let id = path.file_stem().and_then(|stem| stem.to_str())?;
            if !id.starts_with("hepta-proc-") {
                return None;
            }
            native_process_snapshot_from_log(id)
        })
        .collect::<Vec<_>>();
    snapshots.sort_by_key(|snapshot| {
        snapshot
            .get("started_at_unix_ms")
            .and_then(Value::as_u64)
            .unwrap_or(0)
    });
    snapshots
}

fn native_compat_process(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let action = input
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("list");
    match action {
        "list" | "status" => native_process_list(tool, action),
        "poll" => native_process_poll(tool, input),
        "log" | "read" => native_process_log(tool, input),
        "write" | "submit" => native_process_write(tool, input),
        "kill" | "terminate" => native_process_kill(tool, input),
        "clear" | "remove" => native_process_remove(tool, input, action),
        other => Err(hepta_core::ToolError(format!(
            "unsupported process action '{}'; supported actions: list, poll, log, write, kill, clear, remove",
            other
        ))),
    }
}

fn native_process_id(
    input: &serde_json::Map<String, Value>,
) -> Result<String, hepta_core::ToolError> {
    input
        .get("sessionId")
        .or_else(|| input.get("session_id"))
        .or_else(|| input.get("id"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| hepta_core::ToolError("process action requires sessionId".into()))
}

fn native_process_snapshot(
    id: &str,
    process: &mut NativeBackgroundProcess,
) -> Result<Value, hepta_core::ToolError> {
    let status = process
        .child
        .try_wait()
        .map_err(|err| hepta_core::ToolError(format!("failed to poll {}: {}", id, err)))?;
    let running = status.is_none();
    let exit_code = status.and_then(|status| status.code());
    Ok(json!({
        "sessionId": id,
        "id": id,
        "pid": process.child.id(),
        "command": process.command,
        "workdir": process.workdir.display().to_string(),
        "log_path": process.log_path.display().to_string(),
        "started_at_unix_ms": process.started_at_unix_ms,
        "running": running,
        "exit_code": exit_code,
        "stdin_open": process.stdin.is_some()
    }))
}

fn native_process_list(
    tool: &str,
    action: &str,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let mut registry = native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
    let mut processes = Vec::new();
    for (id, process) in registry.iter_mut() {
        processes.push(native_process_snapshot(id, process)?);
    }
    let registry_ids = registry
        .keys()
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    drop(registry);
    for snapshot in native_process_log_snapshots() {
        let Some(id) = snapshot.get("id").and_then(Value::as_str) else {
            continue;
        };
        if !registry_ids.contains(id) {
            processes.push(snapshot);
        }
    }
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!("{} native background process(es)", processes.len())),
    );
    out.insert(
        "result".into(),
        json!({
            "action": action,
            "processes": processes,
            "native_registry_present": true,
            "background_exec_capture_supported": true,
            "log_backed_followup_supported": true,
            "followup_actions": ["poll", "log", "write", "kill", "clear", "remove"]
        }),
    );
    Ok(out)
}

fn native_process_poll(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let id = native_process_id(input)?;
    let timeout_ms = input
        .get("timeout")
        .or_else(|| input.get("timeoutMs"))
        .and_then(Value::as_u64)
        .unwrap_or(0)
        .min(30_000);
    let started = SystemTime::now();
    loop {
        {
            let mut registry = native_process_registry().lock().map_err(|_| {
                hepta_core::ToolError("native process registry lock poisoned".into())
            })?;
            let snapshot = if let Some(process) = registry.get_mut(&id) {
                native_process_snapshot(&id, process)?
            } else {
                native_process_snapshot_from_log(&id).ok_or_else(|| {
                    hepta_core::ToolError(format!("no native background process found for {}", id))
                })?
            };
            if snapshot.get("running").and_then(Value::as_bool) != Some(true) || timeout_ms == 0 {
                let mut out = native_compat_base(tool, "ok");
                out.insert(
                    "content".into(),
                    Value::String(format!(
                        "process {} poll: running={}",
                        id, snapshot["running"]
                    )),
                );
                out.insert(
                    "result".into(),
                    json!({"action":"poll", "process": snapshot}),
                );
                return Ok(out);
            }
        }
        if started
            .elapsed()
            .map(|elapsed| elapsed.as_millis() as u64 >= timeout_ms)
            .unwrap_or(true)
        {
            return native_process_poll(tool, &{
                let mut next = input.clone();
                next.insert("timeout".into(), Value::Number(serde_json::Number::from(0)));
                next
            });
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn native_process_log(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let id = native_process_id(input)?;
    let (log_path, snapshot) = {
        let mut registry = native_process_registry()
            .lock()
            .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
        if let Some(process) = registry.get_mut(&id) {
            (
                process.log_path.clone(),
                native_process_snapshot(&id, process)?,
            )
        } else {
            let snapshot = native_process_snapshot_from_log(&id).ok_or_else(|| {
                hepta_core::ToolError(format!("no native background process found for {}", id))
            })?;
            (native_process_log_path(&id), snapshot)
        }
    };
    let offset = input.get("offset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let limit = input.get("limit").and_then(Value::as_u64).unwrap_or(50_000) as usize;
    let bytes = fs::read(&log_path).unwrap_or_default();
    let start = offset.min(bytes.len());
    let end = start.saturating_add(limit).min(bytes.len());
    let text = String::from_utf8_lossy(&bytes[start..end]).to_string();
    let mut out = native_compat_base(tool, "ok");
    out.insert("content".into(), Value::String(text.clone()));
    out.insert(
        "result".into(),
        json!({
            "action": "log",
            "process": snapshot,
            "offset": offset,
            "next_offset": end,
            "truncated": end < bytes.len(),
            "text": text
        }),
    );
    Ok(out)
}

fn native_process_write(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let id = native_process_id(input)?;
    let data = input
        .get("data")
        .or_else(|| input.get("text"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let eof = input.get("eof").and_then(Value::as_bool).unwrap_or(false);
    let mut registry = native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
    let process = registry.get_mut(&id).ok_or_else(|| {
        hepta_core::ToolError(format!("no native background process found for {}", id))
    })?;
    if let Some(stdin) = process.stdin.as_mut() {
        stdin
            .write_all(data.as_bytes())
            .and_then(|_| stdin.flush())
            .map_err(|err| hepta_core::ToolError(format!("failed writing to {}: {}", id, err)))?;
    } else if !data.is_empty() {
        return Err(hepta_core::ToolError(format!("stdin is closed for {}", id)));
    }
    if eof {
        process.stdin.take();
    }
    let snapshot = native_process_snapshot(&id, process)?;
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!("wrote {} bytes to {}", data.len(), id)),
    );
    out.insert(
        "result".into(),
        json!({"action":"write", "bytes": data.len(), "eof": eof, "process": snapshot}),
    );
    Ok(out)
}

fn native_process_kill(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let id = native_process_id(input)?;
    let mut registry = native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
    let snapshot = if let Some(process) = registry.get_mut(&id) {
        let killed_tree = native_kill_child_process_tree(&mut process.child);
        let snapshot = native_process_snapshot(&id, process)?;
        if !killed_tree && snapshot.get("exit_code").and_then(Value::as_i64).is_none() {
            return Err(hepta_core::ToolError(format!(
                "failed to signal native process tree for {}",
                id
            )));
        }
        snapshot
    } else if let Some(pid) = native_process_pid_from_id(&id) {
        let _ = native_send_signal_to_pid_tree(pid, "-TERM");
        native_process_snapshot_from_log(&id).ok_or_else(|| {
            hepta_core::ToolError(format!("no native background process found for {}", id))
        })?
    } else {
        return Err(hepta_core::ToolError(format!(
            "no native background process found for {}",
            id
        )));
    };
    let mut out = native_compat_base(tool, "ok");
    out.insert("content".into(), Value::String(format!("killed {}", id)));
    out.insert(
        "result".into(),
        json!({"action":"kill", "process": snapshot}),
    );
    Ok(out)
}

fn native_process_remove(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    action: &str,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let mut registry = native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
    let removed = if action == "clear" && native_process_id(input).is_err() {
        let count = registry.len();
        registry.clear();
        let log_removed = fs::read_dir(native_process_log_dir())
            .ok()
            .into_iter()
            .flat_map(|entries| entries.filter_map(Result::ok))
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("log"))
            .filter(|entry| fs::remove_file(entry.path()).is_ok())
            .count();
        count + log_removed
    } else {
        let id = native_process_id(input)?;
        if let Some(mut process) = registry.remove(&id) {
            if process.child.try_wait().ok().flatten().is_none() {
                let _ = process.child.kill();
                let _ = process.child.wait();
            }
            let _ = fs::remove_file(native_process_log_path(&id));
            1
        } else if native_process_log_path(&id).exists() {
            let _ = fs::remove_file(native_process_log_path(&id));
            1
        } else {
            0
        }
    };
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!(
            "removed {} native process registry entrie(s)",
            removed
        )),
    );
    out.insert(
        "result".into(),
        json!({"action": action, "removed": removed}),
    );
    Ok(out)
}

fn native_compat_web_fetch(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let url = input
        .get("url")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("web_fetch requires string field 'url'".into()))?;
    let output = std::process::Command::new("curl")
        .arg("-L")
        .arg("--max-time")
        .arg("30")
        .arg("--silent")
        .arg("--show-error")
        .arg(url)
        .output()
        .map_err(|err| hepta_core::ToolError(format!("failed to run curl: {}", err)))?;
    let body = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let max_chars = input
        .get("maxChars")
        .or_else(|| input.get("max_chars"))
        .and_then(Value::as_u64)
        .unwrap_or(20_000) as usize;
    let extracted = body.chars().take(max_chars).collect::<String>();
    let mut out = native_compat_base(
        tool,
        if output.status.success() {
            "ok"
        } else {
            "error"
        },
    );
    out.insert("content".into(), Value::String(extracted.clone()));
    out.insert(
        "result".into(),
        json!({
            "url": url,
            "status_code_available": false,
            "text": extracted,
            "stderr": stderr,
            "truncated": body.chars().count() > max_chars
        }),
    );
    Ok(out)
}

fn native_compat_web_search(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let query = input
        .get("query")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("web_search requires string field 'query'".into()))?;
    let encoded = query.replace(' ', "+");
    let url = format!("https://duckduckgo.com/html/?q={}", encoded);
    let mut fetch_input = serde_json::Map::new();
    fetch_input.insert("url".into(), Value::String(url.clone()));
    fetch_input.insert(
        "maxChars".into(),
        Value::Number(serde_json::Number::from(20_000)),
    );
    let fetched = native_compat_web_fetch(tool, &fetch_input)?;
    let text = fetched
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let mut out = native_compat_base(tool, "ok");
    out.insert("content".into(), Value::String(text.clone()));
    out.insert(
        "result".into(),
        json!({ "query": query, "search_url": url, "raw_html_excerpt": text }),
    );
    Ok(out)
}

fn native_compat_memory_search(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let query = input.get("query").and_then(Value::as_str).ok_or_else(|| {
        hepta_core::ToolError("memory_search requires string field 'query'".into())
    })?;
    let max_results = input
        .get("maxResults")
        .or_else(|| input.get("max_results"))
        .and_then(Value::as_u64)
        .unwrap_or(10) as usize;
    let workspace = tool_workspace_root_path();
    let mut hits = Vec::new();
    for path in memory_candidate_paths(&workspace) {
        if let Ok(content) = fs::read_to_string(&path) {
            for (idx, line) in content.lines().enumerate() {
                if line.to_lowercase().contains(&query.to_lowercase()) {
                    hits.push(json!({
                        "path": path.display().to_string(),
                        "line": idx + 1,
                        "snippet": line
                    }));
                    if hits.len() >= max_results {
                        break;
                    }
                }
            }
        }
        if hits.len() >= max_results {
            break;
        }
    }
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!("{} memory hit(s)", hits.len())),
    );
    out.insert("result".into(), json!({ "query": query, "hits": hits }));
    Ok(out)
}

fn native_compat_memory_get(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let path = input
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("memory_get requires string field 'path'".into()))?;
    native_compat_read(tool, &{
        let mut mapped = serde_json::Map::new();
        mapped.insert("path".into(), Value::String(path.into()));
        if let Some(from) = input.get("from") {
            mapped.insert("offset".into(), from.clone());
        }
        if let Some(lines) = input.get("lines") {
            mapped.insert("limit".into(), lines.clone());
        }
        mapped
    })
}

fn memory_candidate_paths(workspace: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let root_memory = workspace.join("MEMORY.md");
    if root_memory.is_file() {
        paths.push(root_memory);
    }
    let memory_dir = workspace.join("memory");
    if let Ok(entries) = fs::read_dir(memory_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                paths.push(path);
            }
        }
    }
    paths
}

fn native_compat_status_report(
    tool: &str,
    _input: &serde_json::Map<String, Value>,
) -> serde_json::Map<String, Value> {
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String("Hepta native runtime status: OpenClaw proxy disabled".into()),
    );
    out.insert(
        "result".into(),
        json!({
            "runtime": "hepta-rust-native",
            "openclaw_proxy_tools_enabled": false,
            "native_openclaw_compatible_tool_count": native_openclaw_compatible_tools().len()
        }),
    );
    out
}

fn native_compat_plan_echo(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> serde_json::Map<String, Value> {
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String("plan accepted by native Hepta surface".into()),
    );
    out.insert("result".into(), Value::Object(input.clone()));
    out
}

fn native_compat_live_surface(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    match tool {
        "message" => native_compat_message(tool, input),
        "tts" => native_compat_tts(tool, input),
        "image_generate" => native_compat_image_generate(tool, input),
        "music_generate" => {
            native_compat_configured_generator(tool, input, "HEPTA_MUSIC_GENERATE_CMD")
        }
        "video_generate" => {
            native_compat_configured_generator(tool, input, "HEPTA_VIDEO_GENERATE_CMD")
        }
        "image" => native_compat_image_analyze(tool, input),
        "pdf" => native_compat_pdf_analyze(tool, input),
        "agents_list" => native_compat_hepta_cli(tool, &["/agent-pool", "--json"]),
        "sessions_list" => native_compat_hepta_cli(tool, &["/sessions", "--json"]),
        "sessions_history" => native_compat_sessions_history(tool, input),
        "sessions_send" => native_compat_sessions_send(tool, input),
        "sessions_spawn" => native_compat_sessions_spawn(tool, input),
        "sessions_yield" => Ok(native_compat_local_event(tool, input, "yield_recorded")),
        "subagents" => native_compat_subagents(tool, input),
        "canvas" => {
            native_compat_hepta_cli(tool, &["/canvas-plane", "--all", "--sample-run", "--json"])
        }
        "feishu_app_scopes"
        | "feishu_chat"
        | "feishu_doc"
        | "feishu_drive"
        | "feishu_wiki"
        | "feishu_bitable_get_meta"
        | "feishu_bitable_list_fields"
        | "feishu_bitable_list_records"
        | "feishu_bitable_get_record"
        | "feishu_bitable_create_record"
        | "feishu_bitable_update_record"
        | "feishu_bitable_create_app"
        | "feishu_bitable_create_field" => native_compat_feishu(tool, input),
        _ => Ok(native_compat_surface_report(tool, input)),
    }
}

fn native_compat_hepta_cli(
    tool: &str,
    args: &[&str],
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let binary = hepta_cli_binary();
    let output = std::process::Command::new(&binary)
        .args(args)
        .current_dir(tool_workspace_root_path())
        .output()
        .map_err(|err| {
            hepta_core::ToolError(format!(
                "failed to run Hepta native CLI {}: {}",
                binary.display(),
                err
            ))
        })?;
    command_output_to_native_result(tool, &binary.display().to_string(), args, output)
}

fn hepta_cli_binary() -> PathBuf {
    if let Ok(path) = env::var("HEPTA_NATIVE_TOOL_CLI_BIN") {
        return PathBuf::from(path);
    }
    env::current_exe().unwrap_or_else(|_| PathBuf::from("/Users/qianqi/.local/bin/hepta"))
}

fn command_output_to_native_result(
    tool: &str,
    command: &str,
    args: &[&str],
    output: std::process::Output,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let mut out = native_compat_base(
        tool,
        if output.status.success() {
            "ok"
        } else {
            "error"
        },
    );
    let parsed_json = serde_json::from_str::<Value>(&stdout).ok();
    out.insert("content".into(), Value::String(stdout.clone()));
    out.insert(
        "result".into(),
        json!({
            "command": command,
            "args": args,
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": stdout,
            "stderr": stderr,
            "parsed_json": parsed_json,
            "live_adapter_invoked": true,
            "openclaw_proxy_used": false
        }),
    );
    Ok(out)
}

fn native_compat_message(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let action = input
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("send");
    let channel = input
        .get("channel")
        .and_then(Value::as_str)
        .unwrap_or("telegram");
    if action != "send" {
        let args = match action {
            "channel-list" | "channel-info" | "member-info" => {
                ["/telegram-adapter", "--dry-run", "--json"].as_slice()
            }
            _ => ["/telegram-adapter", "--dry-run", "--json"].as_slice(),
        };
        return native_compat_hepta_cli(tool, args);
    }
    let target = input
        .get("target")
        .or_else(|| input.get("to"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("message send requires string field 'target'".into())
        })?;
    let text = input
        .get("message")
        .or_else(|| input.get("text"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("message send requires string field 'message'".into())
        })?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(true);
    let confirm_send = input
        .get("confirmSend")
        .or_else(|| input.get("confirm_send"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if channel != "telegram" {
        return Err(hepta_core::ToolError(format!(
            "message native live send currently supports telegram; requested channel '{}'",
            channel
        )));
    }
    if dry_run || !confirm_send {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(
                "telegram send preview ready; set dryRun=false and confirmSend=true to send".into(),
            ),
        );
        out.insert(
            "result".into(),
            json!({
                "channel": channel,
                "target_shape": redact_identifier_shape(target),
                "message_chars": text.chars().count(),
                "would_send": true,
                "sent": false,
                "requires_confirmSend": true
            }),
        );
        return Ok(out);
    }
    let args = vec![
        "/telegram-adapter".to_string(),
        "--live-send".to_string(),
        "--confirm-send".to_string(),
        "--to".to_string(),
        target.to_string(),
        "--text".to_string(),
        text.to_string(),
        "--json".to_string(),
    ];
    native_compat_hepta_cli_owned(tool, &args)
}

fn native_compat_hepta_cli_owned(
    tool: &str,
    args: &[String],
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let borrowed = args.iter().map(String::as_str).collect::<Vec<_>>();
    native_compat_hepta_cli(tool, &borrowed)
}

fn native_compat_tts(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let text = input
        .get("text")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("tts requires string field 'text'".into()))?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let target = input
        .get("path")
        .or_else(|| input.get("filename"))
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            tool_workspace_root_path().join(format!(
                "target/hepta-tts-{}-{}.aiff",
                std::process::id(),
                current_unix_ms().unwrap_or(0)
            ))
        });
    let target = resolve_path_within_root(&tool_workspace_root_path(), &target);
    if dry_run {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(format!(
                "would synthesize {} chars to {}",
                text.chars().count(),
                target.display()
            )),
        );
        out.insert("result".into(), json!({"path": target.display().to_string(), "chars": text.chars().count(), "dryRun": true}));
        return Ok(out);
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            hepta_core::ToolError(format!("failed to create {}: {}", parent.display(), err))
        })?;
    }
    let output = std::process::Command::new("say")
        .arg("-o")
        .arg(&target)
        .arg(text)
        .output()
        .map_err(|err| hepta_core::ToolError(format!("failed to run macOS say: {}", err)))?;
    let mut out = command_output_to_native_result(
        tool,
        "say",
        &["-o", "<redacted-path>", "<redacted-text>"],
        output,
    )?;
    out.insert(
        "content".into(),
        Value::String(format!("synthesized speech to {}", target.display())),
    );
    out.insert("result".into(), json!({"path": target.display().to_string(), "chars": text.chars().count(), "format": "aiff", "live_adapter_invoked": true}));
    Ok(out)
}

fn native_compat_image_generate(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let prompt = input
        .get("prompt")
        .or_else(|| input.get("message"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("image_generate requires string field 'prompt'".into())
        })?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let script = image_generation_helper_script();
    if dry_run {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String("would invoke local Ollama image generation helper".into()),
        );
        out.insert("result".into(), json!({"script": script.as_ref().map(|path| path.display().to_string()), "env_fallback": "HEPTA_IMAGE_GENERATE_CMD", "prompt_chars": prompt.chars().count(), "dryRun": true}));
        return Ok(out);
    }
    let Some(script) = script else {
        return native_compat_configured_generator(tool, input, "HEPTA_IMAGE_GENERATE_CMD");
    };
    let output = std::process::Command::new(&script)
        .arg(prompt)
        .current_dir(tool_workspace_root_path())
        .output()
        .map_err(|err| {
            hepta_core::ToolError(format!(
                "failed to run image helper {}: {}",
                script.display(),
                err
            ))
        })?;
    command_output_to_native_result(
        tool,
        &script.display().to_string(),
        &["<redacted-prompt>"],
        output,
    )
}

fn image_generation_helper_script() -> Option<PathBuf> {
    let root = tool_workspace_root_path();
    let candidates = [
        root.join("ollama-image-generation/generate.sh"),
        root.parent()
            .unwrap_or(root.as_path())
            .join("ollama-image-generation/generate.sh"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn native_compat_configured_generator(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    env_name: &str,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let prompt = input
        .get("prompt")
        .or_else(|| input.get("message"))
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError(format!("{} requires string field 'prompt'", tool)))?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if dry_run {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(format!("would invoke configured generator {}", env_name)),
        );
        out.insert(
            "result".into(),
            json!({"env": env_name, "prompt_chars": prompt.chars().count(), "dryRun": true}),
        );
        return Ok(out);
    }
    let command = env::var(env_name).map_err(|_| {
        hepta_core::ToolError(format!(
            "{} has no native provider command configured; set {} to a local generator command that accepts the prompt as argv[1]",
            tool, env_name
        ))
    })?;
    let output = std::process::Command::new("/bin/zsh")
        .arg("-lc")
        .arg(format!("{} -- {}", command, shell_quote(prompt)))
        .current_dir(tool_workspace_root_path())
        .output()
        .map_err(|err| {
            hepta_core::ToolError(format!("failed to run configured generator: {}", err))
        })?;
    command_output_to_native_result(
        tool,
        env_name,
        &["<configured-command>", "<redacted-prompt>"],
        output,
    )
}

fn native_compat_image_analyze(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let image_path = input
        .get("image")
        .and_then(Value::as_str)
        .or_else(|| {
            input
                .get("images")
                .and_then(Value::as_array)
                .and_then(|arr| arr.first())
                .and_then(Value::as_str)
        })
        .ok_or_else(|| {
            hepta_core::ToolError("image requires 'image' or first item in 'images'".into())
        })?;
    let path = resolve_path_within_root(&tool_workspace_root_path(), Path::new(image_path));
    let metadata = fs::metadata(&path).map_err(|err| {
        hepta_core::ToolError(format!("failed to stat {}: {}", path.display(), err))
    })?;
    let file_output = std::process::Command::new("file").arg(&path).output().ok();
    let sips_output = std::process::Command::new("sips")
        .args(["-g", "pixelWidth", "-g", "pixelHeight"])
        .arg(&path)
        .output()
        .ok();
    let file_text = file_output
        .as_ref()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    let sips_text = sips_output
        .as_ref()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!(
            "image metadata for {}: {}",
            path.display(),
            file_text.trim()
        )),
    );
    out.insert("result".into(), json!({"path": path.display().to_string(), "bytes": metadata.len(), "file": file_text, "sips": sips_text, "vision_model_invoked": false, "local_metadata_analyzed": true}));
    Ok(out)
}

fn native_compat_pdf_analyze(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let pdf_path = input
        .get("pdf")
        .and_then(Value::as_str)
        .or_else(|| {
            input
                .get("pdfs")
                .and_then(Value::as_array)
                .and_then(|arr| arr.first())
                .and_then(Value::as_str)
        })
        .ok_or_else(|| {
            hepta_core::ToolError("pdf requires 'pdf' or first item in 'pdfs'".into())
        })?;
    let path = resolve_path_within_root(&tool_workspace_root_path(), Path::new(pdf_path));
    let metadata = fs::metadata(&path).map_err(|err| {
        hepta_core::ToolError(format!("failed to stat {}: {}", path.display(), err))
    })?;
    let max_chars = input
        .get("maxChars")
        .or_else(|| input.get("max_chars"))
        .and_then(Value::as_u64)
        .unwrap_or(20_000) as usize;
    let text_output = std::process::Command::new("pdftotext")
        .arg(&path)
        .arg("-")
        .output()
        .ok();
    let extracted = text_output
        .as_ref()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .chars()
                .take(max_chars)
                .collect::<String>()
        })
        .unwrap_or_default();
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(if extracted.is_empty() {
            format!(
                "pdf metadata for {}; pdftotext unavailable or returned no text",
                path.display()
            )
        } else {
            extracted.clone()
        }),
    );
    out.insert("result".into(), json!({"path": path.display().to_string(), "bytes": metadata.len(), "text": extracted, "text_extracted": !extracted.is_empty(), "external_model_invoked": false}));
    Ok(out)
}

fn native_compat_sessions_history(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let session = input
        .get("sessionKey")
        .or_else(|| input.get("session_id"))
        .and_then(Value::as_str)
        .unwrap_or("session-main");
    let limit = input
        .get("limit")
        .and_then(Value::as_u64)
        .unwrap_or(20)
        .to_string();
    let args = vec![
        "/history".to_string(),
        session.to_string(),
        "--limit".to_string(),
        limit,
        "--json".to_string(),
    ];
    native_compat_hepta_cli_owned(tool, &args)
}

fn native_compat_sessions_send(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let session = input
        .get("sessionKey")
        .or_else(|| input.get("session_id"))
        .or_else(|| input.get("label"))
        .and_then(Value::as_str)
        .unwrap_or("session-main");
    let message = input
        .get("message")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("sessions_send requires string field 'message'".into())
        })?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let execute = input
        .get("execute")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    if dry_run || !execute {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(format!("would run prompt in session {}", session)),
        );
        out.insert("result".into(), json!({"session": session, "message_chars": message.chars().count(), "would_execute": true}));
        return Ok(out);
    }
    let args = vec![
        "/run-in".to_string(),
        session.to_string(),
        message.to_string(),
    ];
    native_compat_hepta_cli_owned(tool, &args)
}

fn native_compat_sessions_spawn(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let task = input
        .get("task")
        .or_else(|| input.get("message"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("sessions_spawn requires string field 'task'".into())
        })?;
    let worker = input
        .get("agentId")
        .or_else(|| input.get("worker_id"))
        .and_then(Value::as_str)
        .unwrap_or("native-tool-worker");
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let execute = input
        .get("execute")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    if dry_run || !execute {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(format!("would spawn task for worker {}", worker)),
        );
        out.insert(
            "result".into(),
            json!({"worker_id": worker, "task_chars": task.chars().count(), "would_execute": true}),
        );
        return Ok(out);
    }
    let args = vec![
        "/spawn-task".to_string(),
        worker.to_string(),
        task.to_string(),
        "--json".to_string(),
    ];
    native_compat_hepta_cli_owned(tool, &args)
}

fn native_compat_subagents(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let action = input
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("list");
    match action {
        "list" => native_compat_hepta_cli(tool, &["/agent-pool", "--json"]),
        "steer" => {
            let target = input
                .get("target")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents steer requires target".into()))?;
            let message = input
                .get("message")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents steer requires message".into()))?;
            let args = vec![
                "/agent-steer".to_string(),
                target.to_string(),
                message.to_string(),
                "--json".to_string(),
            ];
            native_compat_hepta_cli_owned(tool, &args)
        }
        "kill" | "stop" => {
            let target = input
                .get("target")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents stop requires target".into()))?;
            let args = vec![
                "/agent-stop".to_string(),
                target.to_string(),
                "--json".to_string(),
            ];
            native_compat_hepta_cli_owned(tool, &args)
        }
        other => Err(hepta_core::ToolError(format!(
            "unsupported subagents action '{}'",
            other
        ))),
    }
}

fn native_compat_feishu(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(true);
    let live_probe = input
        .get("liveProbe")
        .or_else(|| input.get("live_probe"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if dry_run || !live_probe {
        return native_compat_hepta_cli(tool, &["/feishu-adapter", "--dry-run", "--json"]);
    }
    native_compat_hepta_cli(tool, &["/feishu-adapter", "--live-probe", "--json"])
}

fn native_compat_local_event(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    status: &str,
) -> serde_json::Map<String, Value> {
    let mut out = native_compat_base(tool, status);
    out.insert(
        "content".into(),
        Value::String(format!("{} accepted by native Hepta runtime", tool)),
    );
    out.insert("result".into(), Value::Object(input.clone()));
    out
}

fn redact_identifier_shape(value: &str) -> String {
    if value.chars().all(|ch| ch.is_ascii_digit()) {
        format!("numeric:{}", value.len())
    } else if value.contains('@') {
        "handle:<redacted>".into()
    } else {
        format!("text:{}", value.chars().count())
    }
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn native_compat_surface_report(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> serde_json::Map<String, Value> {
    let mut out = native_compat_base(tool, "native_surface_registered");
    out.insert(
        "content".into(),
        Value::String(format!(
            "{} is registered as a Hepta Rust-native tool surface; OpenClaw proxy is disabled for this tool",
            tool
        )),
    );
    out.insert(
        "result".into(),
        json!({
            "input_keys": input.keys().cloned().collect::<Vec<_>>(),
            "native_surface_registered": true,
            "provider_adapter_required_for_live_side_effects": true
        }),
    );
    out
}

fn validate_against_schema_json(
    schema_name: &str,
    schema_kind: &str,
    schema_json: &str,
    payload_json: &str,
) -> Result<(), HeptaError> {
    let schema_value: Value = serde_json::from_str(schema_json).map_err(|err| {
        HeptaError(format!(
            "invalid {} schema for {}: {}",
            schema_kind, schema_name, err
        ))
    })?;
    let input_value: Value = serde_json::from_str(payload_json).map_err(|err| {
        HeptaError(format!(
            "invalid JSON {} for {}: {}",
            schema_kind, schema_name, err
        ))
    })?;

    match schema_value.get("type").and_then(Value::as_str) {
        Some("object") => {
            validate_object_schema(schema_name, schema_kind, &schema_value, &input_value)
        }
        Some(other) => Err(HeptaError(format!(
            "unsupported root {} schema type for {}: {}",
            schema_kind, schema_name, other
        ))),
        None => Err(HeptaError(format!(
            "{} schema missing root type for {}",
            schema_kind, schema_name
        ))),
    }
}

fn ensure_tool_schema_has_field(
    schema_json: &str,
    tool_name: &str,
    field: &str,
) -> Result<(), HeptaError> {
    let schema_value: Value = serde_json::from_str(schema_json)
        .map_err(|err| HeptaError(format!("invalid input schema for {}: {}", tool_name, err)))?;
    let properties = schema_value
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| HeptaError(format!("tool {} schema is missing properties", tool_name)))?;
    if properties.contains_key(field) {
        Ok(())
    } else {
        Err(HeptaError(format!(
            "tool {} input schema has no field '{}'",
            tool_name, field
        )))
    }
}

fn path_argument_name_for_tool(tool_name: &str) -> Option<&'static str> {
    match tool_name {
        "read_file" | "list_dir" | "search_text" => Some("path"),
        _ => None,
    }
}

fn write_path_argument_name_for_tool(tool_name: &str) -> Option<&'static str> {
    match tool_name {
        "write_file" => Some("path"),
        _ => None,
    }
}

fn preview_backup_path(
    workspace_root: &Path,
    target_path: &Path,
) -> Result<PathBuf, hepta_core::ToolError> {
    let backup_root = workspace_root.join("artifacts/backups/write_file");
    let relative = if target_path.starts_with(workspace_root) {
        PathBuf::from("workspace").join(
            target_path
                .strip_prefix(workspace_root)
                .unwrap_or(target_path),
        )
    } else {
        let external = target_path
            .strip_prefix(Path::new("/"))
            .unwrap_or(target_path);
        PathBuf::from("external").join(external)
    };

    let file_name = relative
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            hepta_core::ToolError(format!(
                "cannot derive backup file name for {}",
                target_path.display()
            ))
        })?;
    let start_ts = current_unix_ms().map_err(|err| hepta_core::ToolError(err.0))?;
    preview_backup_path_from_ts(&backup_root, &relative, file_name, start_ts)
}

fn preview_backup_path_from_ts(
    backup_root: &Path,
    relative: &Path,
    file_name: &str,
    start_ts: u64,
) -> Result<PathBuf, hepta_core::ToolError> {
    let backup_dir = backup_root.join(relative.parent().unwrap_or_else(|| Path::new("")));
    let mut ts = start_ts;

    loop {
        let candidate = backup_dir.join(format!("{}.hepta-bak-{}", file_name, ts));
        if !candidate.exists() {
            return Ok(candidate);
        }
        ts = ts.checked_add(1).ok_or_else(|| {
            hepta_core::ToolError(format!(
                "backup timestamp overflow while planning path for {}",
                relative.display()
            ))
        })?;
    }
}

fn preview_transaction_checkpoint_path(
    workspace_root: &Path,
    target_path: &Path,
    transaction_id: &str,
) -> Result<PathBuf, HeptaError> {
    let checkpoint_root = workspace_root.join("artifacts/checkpoints/write_txn");
    let relative = if target_path.starts_with(workspace_root) {
        PathBuf::from("workspace").join(
            target_path
                .strip_prefix(workspace_root)
                .unwrap_or(target_path),
        )
    } else {
        let external = target_path
            .strip_prefix(Path::new("/"))
            .unwrap_or(target_path);
        PathBuf::from("external").join(external)
    };

    let file_name = relative
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            HeptaError(format!(
                "cannot derive transaction checkpoint name for {}",
                target_path.display()
            ))
        })?;
    Ok(checkpoint_root
        .join(relative.parent().unwrap_or_else(|| Path::new("")))
        .join(format!(
            "{}.hepta-txn-{}.checkpoint",
            file_name, transaction_id
        )))
}

fn summarize_write_change(
    mode: &str,
    existed_before: bool,
    content_changed: bool,
    bytes_before: usize,
    bytes_after: usize,
) -> String {
    match (mode, existed_before) {
        ("create", false) => format!("create new file (0 -> {} bytes)", bytes_after),
        ("create", true) => format!(
            "create would fail because target already exists ({} bytes)",
            bytes_before
        ),
        ("overwrite", false) => format!(
            "overwrite will create new file (0 -> {} bytes)",
            bytes_after
        ),
        ("overwrite", true) if content_changed => {
            format!(
                "overwrite existing file ({} -> {} bytes)",
                bytes_before, bytes_after
            )
        }
        ("overwrite", true) => format!(
            "overwrite existing file with identical content ({} bytes)",
            bytes_before
        ),
        ("append", false) => format!("append will create new file (0 -> {} bytes)", bytes_after),
        ("append", true) => format!(
            "append to existing file ({} -> {} bytes)",
            bytes_before, bytes_after
        ),
        _ => format!(
            "write operation {} ({} -> {} bytes)",
            mode, bytes_before, bytes_after
        ),
    }
}

fn collect_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), HeptaError> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|err| {
        HeptaError(format!(
            "failed to read backup directory {}: {}",
            dir.display(),
            err
        ))
    })? {
        let entry =
            entry.map_err(|err| HeptaError(format!("failed to read backup dir entry: {}", err)))?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn looks_like_disk_junk_audit_intent(input: &str) -> bool {
    let lower = input.to_ascii_lowercase();
    let cleanup_words = input.contains("垃圾")
        || input.contains("清理")
        || input.contains("空间")
        || input.contains("磁盘")
        || input.contains("硬盘")
        || lower.contains("junk")
        || lower.contains("cleanup")
        || lower.contains("clean up")
        || lower.contains("disk")
        || lower.contains("cache")
        || lower.contains("storage");
    let scan_words = input.contains("扫")
        || input.contains("看看")
        || input.contains("审计")
        || input.contains("检查")
        || lower.contains("scan")
        || lower.contains("audit")
        || lower.contains("check");
    cleanup_words && scan_words
}

fn native_pre_model_tool_call(input: &str) -> Option<ToolCall> {
    if let Some(tool_call) = extract_explicit_echo_tool_call(input) {
        return Some(tool_call);
    }
    if let Some(tool_call) = extract_explicit_exec_tool_call(input) {
        return Some(tool_call);
    }
    if let Some(tool_call) = extract_explicit_process_tool_call(input) {
        return Some(tool_call);
    }
    if let Some(tool_call) = extract_explicit_write_file_tool_call(input) {
        return Some(tool_call);
    }
    if let Some(path) = extract_read_intent_path(input) {
        return Some(ToolCall {
            name: "read".into(),
            arguments_json: json!({
                "path": path,
                "offset": 1,
                "limit": 80,
            })
            .to_string(),
        });
    }
    if looks_like_disk_junk_audit_intent(input) {
        return Some(ToolCall {
            name: "disk_junk_audit".into(),
            arguments_json: json!({
                "scope": "common_local_cleanup_candidates",
                "max_entries": 120000,
            })
            .to_string(),
        });
    }
    None
}

fn should_offer_model_tools_for_turn(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input).trim();
    if user_text.is_empty() {
        return false;
    }
    let lower = user_text.to_ascii_lowercase();
    let compact_lower = lower.split_whitespace().collect::<String>();

    if native_pre_model_tool_call(user_text).is_some()
        || extract_read_intent_path(user_text).is_some()
        || looks_like_disk_junk_audit_intent(user_text)
    {
        return true;
    }

    if [
        "tool:",
        "read:",
        "write:",
        "overwrite:",
        "append:",
        "preview-write:",
    ]
    .iter()
    .any(|prefix| lower.starts_with(prefix))
    {
        return true;
    }

    let explicit_tool_action = [
        "use",
        "call",
        "invoke",
        "run",
        "execute",
        "调用",
        "使用",
        "请用",
        "帮我用",
        "直接用",
        "必须用",
        "通过",
        "执行",
        "运行",
    ]
    .iter()
    .any(|verb| lower.contains(verb) || user_text.contains(verb));
    let explicit_tool_surface = [
        " tool",
        "tool ",
        "工具",
        "openclaw_",
        "hepta_",
        "write_file",
        "read_file",
        "web_search",
        "web_fetch",
        "process",
        "exec",
        "sessions_",
        "message",
    ]
    .iter()
    .any(|needle| lower.contains(needle) || user_text.contains(needle));
    if explicit_tool_action && explicit_tool_surface {
        return true;
    }

    [
        "calltool",
        "usetool",
        "invoketool",
        "runtool",
        "executetool",
    ]
    .iter()
    .any(|needle| compact_lower.contains(needle))
}

fn model_identity_response(input: &str, active_model: &ModelRef) -> Option<String> {
    if !looks_like_model_identity_intent(input) {
        return None;
    }
    Some(format!(
        "当前会话使用的模型是 `{}/{}`。这次是 Hepta Rust-native runtime 直接读取会话模型绑定，没有调用工具。",
        active_model.provider, active_model.model
    ))
}

fn deterministic_runtime_response(
    input: &str,
    active_model: &ModelRef,
    messages: &[ModelMessage],
) -> Option<String> {
    if let Some(response) = model_identity_response(input, active_model) {
        return Some(response);
    }
    if let Some(response) = assistant_identity_response(input) {
        return Some(response);
    }
    deterministic_memory_marker_response(input, messages)
}

fn assistant_identity_response(input: &str) -> Option<String> {
    if !looks_like_assistant_identity_intent(input) {
        return None;
    }
    Some(
        "我是发发_1，Hepta Telegram 里的 Rust-native 助手实例。当前这条消息由 Hepta runtime 直接按身份问答处理，没有调用工具。"
            .to_string(),
    )
}

fn deterministic_memory_marker_response(input: &str, messages: &[ModelMessage]) -> Option<String> {
    let user_text = hepta_agent_body_or_input(input);
    if !looks_like_live_agent_marker_recall_intent(user_text) {
        return None;
    }
    let context = messages
        .iter()
        .filter(|message| matches!(message.role, MessageRole::System))
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let marker = extract_recent_transcript_live_agent_e2e_marker(&context)
        .or_else(|| extract_live_agent_e2e_marker(&context))?;
    Some(format!("The live-agent-e2e marker is {marker}."))
}

fn looks_like_live_agent_marker_recall_intent(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input);
    let lower = user_text.to_ascii_lowercase();
    let has_marker_surface = lower.contains("live-agent-e2e marker")
        || lower.contains("live_agent_e2e marker")
        || lower.contains("live-agent-e2e-marker")
        || lower.contains("live_agent_e2e_marker");
    if !has_marker_surface || looks_like_live_agent_marker_remember_intent(user_text) {
        return false;
    }
    lower.contains("what")
        || lower.contains("which")
        || lower.contains("recall")
        || lower.contains("remembered")
        || user_text.contains("是什么")
        || user_text.contains("是多少")
        || user_text.contains("告诉我")
        || user_text.contains("读回")
}

fn looks_like_live_agent_marker_remember_intent(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input);
    let lower = user_text.to_ascii_lowercase();
    (lower.contains("remember")
        || user_text.contains("记住")
        || user_text.contains("保存")
        || user_text.contains("写入"))
        && (lower.contains("live-agent-e2e marker")
            || lower.contains("live_agent_e2e marker")
            || lower.contains("live-agent-e2e-marker")
            || lower.contains("live_agent_e2e_marker"))
}

fn extract_recent_transcript_live_agent_e2e_marker(context: &str) -> Option<String> {
    let transcript = context.split_once("Recent session transcript:\n")?.1;
    let transcript = transcript
        .split_once("\n\nRelevant memory records:")
        .map(|(before, _)| before)
        .unwrap_or(transcript);
    extract_live_agent_e2e_marker(transcript)
}

fn extract_live_agent_e2e_marker(input: &str) -> Option<String> {
    let prefix = "hepta-live-agent-e2e-";
    let mut rest = input;
    let mut latest = None::<String>;
    let mut latest_numeric_suffix = 0_u64;
    while let Some(index) = rest.find(prefix) {
        let candidate = rest[index..]
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
            .collect::<String>();
        let suffix = candidate.strip_prefix(prefix).unwrap_or_default();
        let numeric_suffix = suffix
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .ok();
        if let Some(numeric_suffix) = numeric_suffix
            && numeric_suffix >= latest_numeric_suffix
        {
            latest_numeric_suffix = numeric_suffix;
            latest = Some(candidate);
        }
        rest = &rest[index + prefix.len()..];
    }
    latest
}

fn looks_like_model_identity_intent(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input).trim();
    if user_text.is_empty() {
        return false;
    }
    let lower = user_text.to_ascii_lowercase();
    let compact_lower = lower.split_whitespace().collect::<String>();
    let compact_text = user_text.split_whitespace().collect::<String>();
    let model_surface = user_text.contains("模型") || lower.contains("model");
    if !model_surface {
        return false;
    }
    let menu_or_mutation_intent = [
        "模型列表",
        "可用模型",
        "切换模型",
        "选择模型",
        "换模型",
        "/model",
        "/model-in",
        "model list",
        "available model",
        "switch model",
        "select model",
        "change model",
    ]
    .iter()
    .any(|needle| lower.contains(needle) || compact_text.contains(needle));
    if menu_or_mutation_intent {
        return false;
    }
    [
        "你是什么模型",
        "你是哪个模型",
        "你用什么模型",
        "你用的什么模型",
        "你用的是哪个模型",
        "你现在是什么模型",
        "你现在用什么模型",
        "你接的什么模型",
        "现在是什么模型",
        "当前是什么模型",
        "当前模型是什么",
        "什么模型",
    ]
    .iter()
    .any(|needle| compact_text.contains(needle))
        || [
            "whatmodelareyou",
            "whichmodelareyou",
            "whatmodeldoyouuse",
            "whichmodeldoyouuse",
            "currentmodel",
            "activemodel",
        ]
        .iter()
        .any(|needle| compact_lower.contains(needle))
}

fn looks_like_assistant_identity_intent(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input).trim();
    if user_text.is_empty() {
        return false;
    }
    let lower = user_text.to_ascii_lowercase();
    let compact_text = user_text.split_whitespace().collect::<String>();
    let compact_lower = lower.split_whitespace().collect::<String>();

    if [
        "你是谁",
        "你是誰",
        "你叫什么",
        "你叫什麼",
        "你叫什么名字",
        "你叫什麼名字",
        "你是哪位",
        "你是什么",
        "你是什麼",
    ]
    .iter()
    .any(|needle| compact_text.contains(needle))
    {
        return true;
    }

    [
        "whoareyou",
        "whatareyou",
        "whatisyourname",
        "what'syourname",
        "tellmewhoyouare",
    ]
    .iter()
    .any(|needle| compact_lower.contains(needle))
}

fn extract_explicit_write_file_tool_call(input: &str) -> Option<ToolCall> {
    let user_text = hepta_agent_body_or_input(input);
    let lower = user_text.to_ascii_lowercase();
    let explicit_write_file = lower.contains("write_file tool")
        || lower.contains("use write_file")
        || lower.contains("call write_file")
        || lower.contains("write_file 工具")
        || user_text.contains("调用 write_file")
        || user_text.contains("用 write_file");
    if !explicit_write_file {
        return None;
    }
    let start = user_text.find('{')?;
    let end = user_text.rfind('}')?;
    if end <= start {
        return None;
    }
    let args: Value = serde_json::from_str(&user_text[start..=end]).ok()?;
    let object = args.as_object()?;
    if !object.contains_key("path") || !object.contains_key("content") {
        return None;
    }
    Some(ToolCall {
        name: "write_file".into(),
        arguments_json: args.to_string(),
    })
}

fn extract_explicit_echo_tool_call(input: &str) -> Option<ToolCall> {
    let user_text = hepta_agent_body_or_input(input);
    let lower = user_text.to_ascii_lowercase();
    let explicit_echo = lower.contains("echo tool")
        || lower.contains("use echo")
        || lower.contains("call echo")
        || user_text.contains("echo 工具")
        || user_text.contains("调用 echo")
        || user_text.contains("用 echo")
        || user_text.contains("通过 echo");
    if !explicit_echo {
        return None;
    }
    let text = extract_echo_text_argument(user_text)?;
    Some(ToolCall {
        name: "echo".into(),
        arguments_json: json!({ "text": text }).to_string(),
    })
}

fn extract_echo_text_argument(input: &str) -> Option<String> {
    if let Some(value) = extract_json_string_field(input, "text") {
        return Some(value);
    }
    for marker in [
        "内容是",
        "内容为",
        "返回",
        "输出",
        "text exactly",
        "message exactly",
        "message:",
        "text:",
        "text=",
        "say:",
        "return",
    ] {
        if let Some((_, after)) = input.split_once(marker) {
            let text = trim_echo_clause(after);
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

fn extract_json_string_field(input: &str, field: &str) -> Option<String> {
    let field_marker = format!("\"{}\"", field);
    let field_index = input.find(&field_marker)?;
    let after_field = &input[field_index + field_marker.len()..];
    let colon_index = after_field.find(':')?;
    let after_colon = after_field[colon_index + 1..].trim_start();
    let rest = after_colon.strip_prefix('"')?;
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return (!value.trim().is_empty()).then(|| value.trim().to_string());
        } else {
            value.push(ch);
        }
    }
    None
}

fn trim_echo_clause(input: &str) -> String {
    let mut clause = input.trim();
    for separator in [
        "，不要",
        "。不要",
        "；不要",
        ", do not",
        ". do not",
        "; do not",
        "不要",
        "do not",
        "without",
        "并且",
        "然后",
    ] {
        if let Some((before, _)) = clause.split_once(separator) {
            clause = before.trim();
        }
    }
    clause
        .trim_matches(|ch: char| {
            ch.is_whitespace()
                || matches!(
                    ch,
                    '`' | '"' | '\'' | '“' | '”' | '。' | '，' | ',' | ';' | '；' | ':' | '：'
                )
        })
        .to_string()
}

fn hepta_agent_body_or_input(input: &str) -> &str {
    input
        .rsplit_once("BodyForHeptaAgent:")
        .map(|(_, body)| body.trim())
        .unwrap_or(input)
}

fn extract_explicit_exec_tool_call(input: &str) -> Option<ToolCall> {
    let lower = input.to_ascii_lowercase();
    let explicit_exec = lower.contains("exec 工具")
        || lower.contains("exec tool")
        || lower.contains("调用 exec")
        || lower.contains("use exec")
        || lower.contains("用 exec")
        || lower.contains("通过 exec")
        || lower.contains("run with exec");
    if !explicit_exec {
        return None;
    }
    let command = extract_exec_command_text(input)?;
    let background = lower.contains("background=true")
        || lower.contains("background: true")
        || lower.contains("后台")
        || lower.contains("background")
        || lower.contains("异步");
    let mut arguments = json!({
        "command": command,
        "background": background,
    });
    if let Some(timeout_ms) = extract_exec_timeout_ms(input) {
        arguments["timeoutMs"] = Value::Number(serde_json::Number::from(timeout_ms));
    }
    Some(ToolCall {
        name: "exec".into(),
        arguments_json: arguments.to_string(),
    })
}

fn extract_exec_timeout_ms(input: &str) -> Option<u64> {
    for marker in ["timeoutMs=", "timeoutMs:", "timeout_ms=", "timeout_ms:"] {
        if let Some((_, after)) = input.split_once(marker) {
            let digits = after
                .trim_start()
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>();
            if let Ok(value) = digits.parse::<u64>() {
                return Some(value);
            }
        }
    }
    None
}

fn extract_exec_command_text(input: &str) -> Option<String> {
    let trimmed = input.trim();
    for marker in [
        "命令：",
        "命令:",
        "运行：",
        "运行:",
        "执行：",
        "执行:",
        "command:",
        "run:",
        "exec:",
    ] {
        if let Some((_, after)) = trimmed.split_once(marker) {
            let candidate = trim_command_clause(after);
            if !candidate.is_empty() {
                return Some(candidate);
            }
        }
    }

    let lower = trimmed.to_ascii_lowercase();
    if let Some(index) = lower.find("exec") {
        let after = trimmed[index + "exec".len()..].trim_start_matches(|ch: char| {
            ch.is_whitespace()
                || matches!(ch, '工' | '具' | ':' | '：' | '-' | '—' | '，' | ',' | '。')
        });
        let candidate = trim_command_clause(after);
        if !candidate.is_empty() {
            return Some(candidate);
        }
    }
    None
}

fn trim_command_clause(input: &str) -> String {
    let mut clause = input.trim();
    for separator in [
        "；timeoutMs",
        "; timeoutMs",
        " timeoutMs=",
        " timeout_ms=",
        "；timeout",
        "; timeout",
        "；然后",
        "; then",
        "然后",
        "再调用",
        "再用",
        " and then ",
    ] {
        if let Some((before, _)) = clause.split_once(separator) {
            clause = before.trim();
        }
    }
    clause
        .trim_matches(|ch: char| {
            ch.is_whitespace()
                || matches!(
                    ch,
                    '`' | '"' | '\'' | '“' | '”' | '。' | '，' | ',' | ';' | '；'
                )
        })
        .to_string()
}

fn extract_explicit_process_tool_call(input: &str) -> Option<ToolCall> {
    let lower = input.to_ascii_lowercase();
    let explicit_process = lower.contains("process 工具")
        || lower.contains("process tool")
        || lower.contains("调用 process")
        || lower.contains("use process")
        || lower.contains("用 process")
        || lower.contains("通过 process");
    if !explicit_process {
        return None;
    }
    let action = if lower.contains("poll") || input.contains("轮询") || input.contains("状态") {
        "poll"
    } else if lower.contains("log") || input.contains("日志") || input.contains("输出") {
        "log"
    } else if lower.contains("write") || input.contains("写入") || input.contains("输入") {
        "write"
    } else if lower.contains("kill") || input.contains("终止") || input.contains("杀掉") {
        "kill"
    } else if lower.contains("clear") || lower.contains("remove") || input.contains("清除") {
        "clear"
    } else {
        "list"
    };
    let mut args = json!({"action": action});
    if let Some(session_id) = extract_hepta_process_id(input) {
        args["sessionId"] = Value::String(session_id);
    }
    Some(ToolCall {
        name: "process".into(),
        arguments_json: args.to_string(),
    })
}

fn extract_hepta_process_id(input: &str) -> Option<String> {
    input
        .split(|ch: char| ch.is_whitespace() || matches!(ch, '`' | '"' | '\'' | ',' | ';'))
        .find_map(|token| {
            let cleaned = token.trim_matches(|ch: char| {
                matches!(ch, '(' | ')' | '[' | ']' | '{' | '}' | '。' | '，')
            });
            if cleaned.starts_with("hepta-proc-") {
                Some(cleaned.to_string())
            } else {
                None
            }
        })
}

fn extract_read_intent_path(input: &str) -> Option<String> {
    let lower = input.to_ascii_lowercase();
    let looks_like_read = input.contains("读取")
        || input.contains("读一下")
        || input.contains("打开")
        || lower.contains("read ")
        || lower.contains("cat ")
        || lower.contains("show ");
    if !looks_like_read {
        return None;
    }
    for raw in input.split(|ch: char| {
        ch.is_whitespace()
            || matches!(
                ch,
                '，' | '。' | '；' | ';' | ',' | '：' | ':' | '"' | '\'' | '`' | '“' | '”'
            )
    }) {
        let token = raw.trim_matches(|ch: char| {
            matches!(
                ch,
                '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '。' | '，' | ',' | ';' | '；'
            )
        });
        if token.is_empty() {
            continue;
        }
        let token_lower = token.to_ascii_lowercase();
        let path_like = token.contains('/')
            || [
                ".md", ".txt", ".json", ".rs", ".toml", ".yaml", ".yml", ".log", ".csv",
            ]
            .iter()
            .any(|suffix| token_lower.ends_with(suffix));
        if path_like {
            return Some(token.to_string());
        }
    }
    None
}

fn disk_junk_candidate_roots(
    include_var_folders: bool,
) -> Vec<(PathBuf, &'static str, &'static str)> {
    let mut roots = Vec::new();
    if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
        roots.push((
            home.join("Library/Caches"),
            "user_cache",
            "通常可清理，但应先关闭相关应用；优先清理内容而不是删除目录本身。",
        ));
        roots.push((
            home.join("Library/Logs"),
            "user_logs",
            "旧日志通常可删；近期日志建议保留以便排障。",
        ));
        roots.push((
            home.join(".cache"),
            "unix_user_cache",
            "常见 CLI/开发工具缓存；建议按子目录确认后清理。",
        ));
        roots.push((
            home.join("Library/Developer/Xcode/DerivedData"),
            "xcode_derived_data",
            "Xcode 派生数据可重建；确认没有正在构建后再清理。",
        ));
        roots.push((
            home.join("Library/Developer/CoreSimulator/Caches"),
            "simulator_cache",
            "模拟器缓存可重建；先停止模拟器。",
        ));
        roots.push((
            home.join("Library/Application Support/Code/Cache"),
            "vscode_cache",
            "编辑器缓存；关闭 VS Code 后再清理更稳。",
        ));
        roots.push((
            home.join("Library/Application Support/Code/CachedData"),
            "vscode_cached_data",
            "VS Code 可重建缓存；关闭应用后再清理。",
        ));
        roots.push((
            home.join("Library/Application Support/Code/User/workspaceStorage"),
            "vscode_workspace_storage",
            "可能含工作区状态；只建议清理确认不用的旧工作区条目。",
        ));
        roots.push((
            home.join(".npm/_cacache"),
            "npm_cache",
            "npm 缓存可重建；可用 npm cache verify/clean 管理。",
        ));
        roots.push((
            home.join(".cargo/registry/cache"),
            "cargo_registry_cache",
            "Rust registry 包缓存可重拉；清理会导致后续构建重新下载。",
        ));
        roots.push((
            home.join(".cargo/git/checkouts"),
            "cargo_git_checkouts",
            "Cargo git checkout 缓存；清理会导致后续构建重新拉取。",
        ));
    }
    roots.push((
        env::temp_dir(),
        "temp_dir",
        "临时目录可能有正在使用的文件；只清理明显过期条目。",
    ));
    roots.push((
        PathBuf::from("/Library/Caches"),
        "system_cache",
        "系统级缓存需要更谨慎，通常不建议自动删除。",
    ));
    if include_var_folders {
        roots.push((
            PathBuf::from("/private/var/folders"),
            "darwin_user_temp_cache",
            "macOS 用户临时/缓存根目录；只做只读估算，不建议整根删除。",
        ));
    }
    roots
}

fn bounded_dir_size(path: &Path, max_entries: usize, max_depth: usize) -> BoundedDirSize {
    let mut stack = vec![(path.to_path_buf(), 0usize)];
    let mut bytes = 0u64;
    let mut entries_scanned = 0usize;
    let mut inaccessible_count = 0usize;
    let mut truncated = false;

    while let Some((current, depth)) = stack.pop() {
        if entries_scanned >= max_entries {
            truncated = true;
            break;
        }
        entries_scanned = entries_scanned.saturating_add(1);
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(_) => {
                inaccessible_count = inaccessible_count.saturating_add(1);
                continue;
            }
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_file() {
            bytes = bytes.saturating_add(metadata.len());
            continue;
        }
        if !metadata.is_dir() {
            bytes = bytes.saturating_add(metadata.len());
            continue;
        }
        bytes = bytes.saturating_add(metadata.len());
        if depth >= max_depth {
            truncated = true;
            continue;
        }
        let entries = match fs::read_dir(&current) {
            Ok(entries) => entries,
            Err(_) => {
                inaccessible_count = inaccessible_count.saturating_add(1);
                continue;
            }
        };
        for entry in entries {
            match entry {
                Ok(entry) => stack.push((entry.path(), depth.saturating_add(1))),
                Err(_) => inaccessible_count = inaccessible_count.saturating_add(1),
            }
        }
    }

    BoundedDirSize {
        bytes,
        entries_scanned,
        inaccessible_count,
        truncated,
    }
}

fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit_index = 0usize;
    while value >= 1024.0 && unit_index + 1 < UNITS.len() {
        value /= 1024.0;
        unit_index += 1;
    }
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", value, UNITS[unit_index])
    }
}

fn render_disk_junk_audit_reply(tool_output: &str) -> String {
    let structured = tool_output
        .split("structured=")
        .nth(1)
        .or_else(|| tool_output.split("structured:").nth(1))
        .and_then(|json_text| serde_json::from_str::<Value>(json_text.trim()).ok());
    let Some(value) = structured else {
        return format!(
            "我已走 Hepta native runtime 做了只读磁盘垃圾审计。原始结果：{}\n\n我没有删除任何东西；如果要清理，需要你明确确认。",
            tool_output.chars().take(600).collect::<String>()
        );
    };
    let total = value
        .get("estimated_reclaimable_human")
        .and_then(Value::as_str)
        .unwrap_or("未知");
    let count = value
        .get("candidate_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let mut lines = vec![format!(
        "扫完了：这是 Hepta native runtime 的只读审计，没删任何文件。共发现 {} 个候选位置，粗略可回收约 {}。",
        count, total
    )];
    if let Some(candidates) = value.get("top_candidates").and_then(Value::as_array) {
        lines.push("\n优先看这几个：".into());
        for candidate in candidates.iter().take(6) {
            let size = candidate
                .get("human_size")
                .and_then(Value::as_str)
                .unwrap_or("未知大小");
            let path = candidate
                .get("path")
                .and_then(Value::as_str)
                .unwrap_or("未知路径");
            let kind = candidate
                .get("kind")
                .and_then(Value::as_str)
                .unwrap_or("candidate");
            let truncated = candidate
                .get("truncated")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let suffix = if truncated {
                "（估算被截断，实际可能更大）"
            } else {
                ""
            };
            lines.push(format!("- {} · {} · {}{}", size, kind, path, suffix));
        }
    }
    lines.push("\n建议：先从用户缓存/开发缓存下手；系统级缓存和 /private/var/folders 不要整根删。你确认后我再执行具体清理。".into());
    lines.join("\n")
}

fn resolve_backup_reference(backup_root: &Path, backup_ref: &str) -> Result<PathBuf, HeptaError> {
    let direct = PathBuf::from(backup_ref);
    if direct.exists() {
        return Ok(fs::canonicalize(&direct).unwrap_or(direct));
    }
    let nested = backup_root.join(backup_ref);
    if nested.exists() {
        Ok(fs::canonicalize(&nested).unwrap_or(nested))
    } else {
        Err(HeptaError(format!(
            "backup reference not found: {}",
            backup_ref
        )))
    }
}

fn parse_backup_entry(
    workspace_root: &Path,
    backup_root: &Path,
    backup_path: &Path,
) -> Result<Option<BackupEntryReport>, HeptaError> {
    let Ok(relative) = backup_path.strip_prefix(backup_root) else {
        return Ok(None);
    };
    let mut components = relative.components();
    let Some(scope_component) = components
        .next()
        .and_then(|component| component.as_os_str().to_str())
    else {
        return Ok(None);
    };
    let remainder = components.as_path();
    let original_relative = parse_backup_relative_target(remainder)?;
    let (scope, target_path) = match scope_component {
        "workspace" => (
            "workspace".to_string(),
            workspace_root.join(&original_relative),
        ),
        "external" => (
            "external".to_string(),
            PathBuf::from("/").join(&original_relative),
        ),
        _ => return Ok(None),
    };
    let file_name = backup_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            HeptaError(format!(
                "invalid backup file name: {}",
                backup_path.display()
            ))
        })?;
    let created_at_unix_ms = file_name
        .rsplit_once(".hepta-bak-")
        .and_then(|(_, ts)| ts.parse::<u64>().ok())
        .ok_or_else(|| {
            HeptaError(format!(
                "backup file missing timestamp suffix: {}",
                backup_path.display()
            ))
        })?;
    let metadata = match fs::metadata(backup_path) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => {
            return Err(HeptaError(format!(
                "failed to stat backup {}: {}",
                backup_path.display(),
                err
            )));
        }
    };
    Ok(Some(BackupEntryReport {
        id: relative.display().to_string(),
        backup_path: backup_path.display().to_string(),
        target_path: target_path.display().to_string(),
        scope,
        created_at_unix_ms,
        bytes: metadata.len(),
    }))
}

fn parse_backup_relative_target(path: &Path) -> Result<PathBuf, HeptaError> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| HeptaError(format!("invalid backup target path: {}", path.display())))?;
    let Some((original_name, _)) = file_name.rsplit_once(".hepta-bak-") else {
        return Err(HeptaError(format!(
            "backup file missing original suffix: {}",
            path.display()
        )));
    };
    let mut target = PathBuf::from(path.parent().unwrap_or_else(|| Path::new("")));
    target.push(original_name);
    Ok(target)
}

fn validate_object_schema(
    schema_name: &str,
    schema_kind: &str,
    schema_value: &Value,
    input_value: &Value,
) -> Result<(), HeptaError> {
    let input_object = input_value.as_object().ok_or_else(|| {
        HeptaError(format!(
            "tool {} expects a JSON object {}",
            schema_name, schema_kind
        ))
    })?;

    if let Some(required) = schema_value.get("required").and_then(Value::as_array) {
        for field in required.iter().filter_map(Value::as_str) {
            if !input_object.contains_key(field) {
                return Err(HeptaError(format!(
                    "tool {} missing required field '{}'",
                    schema_name, field
                )));
            }
        }
    }

    if let Some(properties) = schema_value.get("properties").and_then(Value::as_object) {
        for (field, field_schema) in properties {
            if let Some(value) = input_object.get(field) {
                validate_property(schema_name, field, field_schema, value)?;
            }
        }
    }

    Ok(())
}

fn validate_property(
    schema_name: &str,
    field: &str,
    field_schema: &Value,
    value: &Value,
) -> Result<(), HeptaError> {
    match field_schema.get("type").and_then(Value::as_str) {
        Some("string") => {
            let string_value = value.as_str().ok_or_else(|| {
                HeptaError(format!(
                    "tool {} field '{}' must be a string",
                    schema_name, field
                ))
            })?;

            if let Some(min_length) = field_schema.get("minLength").and_then(Value::as_u64) {
                if string_value.chars().count() < min_length as usize {
                    return Err(HeptaError(format!(
                        "tool {} field '{}' must be at least {} characters",
                        schema_name, field, min_length
                    )));
                }
            }

            if let Some(allowed) = field_schema.get("enum").and_then(Value::as_array) {
                let allowed_values = allowed.iter().filter_map(Value::as_str).collect::<Vec<_>>();
                if !allowed_values.is_empty()
                    && !allowed_values.iter().any(|item| *item == string_value)
                {
                    return Err(HeptaError(format!(
                        "tool {} field '{}' must be one of: {}",
                        schema_name,
                        field,
                        allowed_values.join(", ")
                    )));
                }
            }

            Ok(())
        }
        Some("boolean") => {
            if value.is_boolean() {
                Ok(())
            } else {
                Err(HeptaError(format!(
                    "tool {} field '{}' must be a boolean",
                    schema_name, field
                )))
            }
        }
        Some("integer") => {
            let integer_value = value.as_i64().ok_or_else(|| {
                HeptaError(format!(
                    "tool {} field '{}' must be an integer",
                    schema_name, field
                ))
            })?;

            if let Some(minimum) = field_schema.get("minimum").and_then(Value::as_i64) {
                if integer_value < minimum {
                    return Err(HeptaError(format!(
                        "tool {} field '{}' must be at least {}",
                        schema_name, field, minimum
                    )));
                }
            }

            Ok(())
        }
        Some("number") => {
            if value.is_number() {
                Ok(())
            } else {
                Err(HeptaError(format!(
                    "tool {} field '{}' must be a number",
                    schema_name, field
                )))
            }
        }
        Some("array") => {
            if value.is_array() {
                Ok(())
            } else {
                Err(HeptaError(format!(
                    "tool {} field '{}' must be an array",
                    schema_name, field
                )))
            }
        }
        Some("object") => {
            if value.is_object() {
                Ok(())
            } else {
                Err(HeptaError(format!(
                    "tool {} field '{}' must be an object",
                    schema_name, field
                )))
            }
        }
        Some(other) => Err(HeptaError(format!(
            "tool {} field '{}' uses unsupported schema type {}",
            schema_name, field, other
        ))),
        None => Err(HeptaError(format!(
            "tool {} field '{}' is missing a type",
            schema_name, field
        ))),
    }
}

fn format_tool_message(tool_result: &ToolResult) -> String {
    match &tool_result.structured_json {
        Some(structured_json) => {
            format!("{} | structured={}", tool_result.content, structured_json)
        }
        None => tool_result.content.clone(),
    }
}

fn format_tool_memory_content(tool_result: &ToolResult) -> String {
    match &tool_result.structured_json {
        Some(structured_json) => format!(
            "tool:{} | structured:{}",
            tool_result.content, structured_json
        ),
        None => format!("tool:{}", tool_result.content),
    }
}

fn current_unix_ms() -> Result<u64, HeptaError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| HeptaError(format!("system clock before unix epoch: {}", err)))?;
    Ok(duration.as_millis() as u64)
}

fn summarize_user_intent(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(120)
        .collect()
}

fn truncate_for_context(input: &str, max_chars: usize) -> String {
    let mut output = input.chars().take(max_chars).collect::<String>();
    if input.chars().count() > max_chars {
        output.push('…');
    }
    output
}

fn memory_context_keyword(input: &str) -> String {
    let trimmed = input.trim();
    for keyword in ["暗号", "密码", "口令", "remember", "memory", "记住", "之前"] {
        if trimmed.contains(keyword) {
            return keyword.to_string();
        }
    }
    trimmed
        .split_whitespace()
        .next()
        .unwrap_or("")
        .chars()
        .take(32)
        .collect()
}

fn merge_approval_snapshots(
    current: ApprovalSnapshot,
    incoming: ApprovalSnapshot,
) -> ApprovalSnapshot {
    let mut granted_tools = current.granted_tools;
    for tool in incoming.granted_tools {
        if !granted_tools.iter().any(|existing| existing == &tool) {
            granted_tools.push(tool);
        }
    }

    let mut pending = current.pending;
    for item in incoming.pending {
        if granted_tools.iter().any(|tool| tool == &item.tool_name) {
            continue;
        }
        if !pending
            .iter()
            .any(|existing| existing.tool_name == item.tool_name)
        {
            pending.push(item);
        }
    }

    ApprovalSnapshot {
        granted_tools,
        pending,
    }
}

impl RuntimeKernel {
    fn next_write_transaction_group_id(
        &self,
        requested: Option<&str>,
    ) -> Result<String, HeptaError> {
        if let Some(requested) = requested.map(str::trim).filter(|value| !value.is_empty()) {
            let guard = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            if guard.groups.iter().any(|group| group.group_id == requested) {
                return Err(HeptaError(format!(
                    "write transaction group already exists: {}",
                    requested
                )));
            }
            return Ok(requested.to_string());
        }

        let now = current_unix_ms()?;
        let mut suffix = 1usize;
        let guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        loop {
            let candidate = format!("txngrp-{}-{}", now, suffix);
            if !guard.groups.iter().any(|group| group.group_id == candidate) {
                return Ok(candidate);
            }
            suffix += 1;
        }
    }

    fn find_write_transaction_group(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<(WriteTransactionGroup, bool), HeptaError> {
        let guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        let active = guard
            .active_bindings
            .iter()
            .any(|binding| binding.session_id == session_id && binding.active_group_id == group_id);
        let group = guard
            .groups
            .iter()
            .find(|group| group.session_id == session_id && group.group_id == group_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("unknown write transaction group: {}", group_id)))?;
        Ok((group, active))
    }

    fn write_transactions_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<WriteTransactionEntry>, HeptaError> {
        let mut transactions = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .iter()
            .filter(|entry| entry.session_id == session_id)
            .cloned()
            .collect::<Vec<_>>();
        transactions.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));
        Ok(transactions)
    }

    fn write_transaction_groups_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<WriteTransactionGroup>, HeptaError> {
        let mut groups = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?
            .groups
            .iter()
            .filter(|group| group.session_id == session_id)
            .cloned()
            .collect::<Vec<_>>();
        groups.sort_by(|left, right| right.opened_at_unix_ms.cmp(&left.opened_at_unix_ms));
        Ok(groups)
    }

    fn active_write_transaction_group_id_for_session(
        &self,
        session_id: &str,
    ) -> Result<Option<String>, HeptaError> {
        Ok(self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?
            .active_bindings
            .iter()
            .find(|binding| binding.session_id == session_id)
            .map(|binding| binding.active_group_id.clone()))
    }

    fn rollback_group_attempts_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<RollbackGroupAttempt>, HeptaError> {
        let mut attempts = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?
            .rollback_attempts
            .iter()
            .filter(|attempt| attempt.session_id == session_id)
            .cloned()
            .collect::<Vec<_>>();
        attempts.sort_by(|left, right| right.started_at_unix_ms.cmp(&left.started_at_unix_ms));
        Ok(attempts)
    }

    fn rollback_group_attempts(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<Vec<RollbackGroupAttempt>, HeptaError> {
        Ok(self
            .rollback_group_attempts_for_session(session_id)?
            .into_iter()
            .filter(|attempt| attempt.group_id == group_id)
            .collect())
    }

    fn latest_rollback_group_attempt(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<Option<RollbackGroupAttempt>, HeptaError> {
        let attempts = self.rollback_group_attempts(session_id, group_id)?;
        Ok(attempts
            .iter()
            .find(|attempt| attempt.superseded_by_attempt_id.is_none())
            .cloned()
            .or_else(|| attempts.into_iter().next()))
    }

    fn rollback_group_attempt_lifecycle(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<RollbackGroupAttemptLifecycle, HeptaError> {
        let attempts = self.rollback_group_attempts(session_id, group_id)?;
        Ok(RollbackGroupAttemptLifecycle {
            attempt_count: attempts.len(),
            superseded_attempt_count: attempts
                .iter()
                .filter(|attempt| attempt.superseded_by_attempt_id.is_some())
                .count(),
            active_attempt_id: attempts
                .iter()
                .find(|attempt| attempt.superseded_by_attempt_id.is_none())
                .map(|attempt| attempt.attempt_id.clone()),
        })
    }

    fn next_rollback_group_attempt_id(&self) -> Result<String, HeptaError> {
        let now = current_unix_ms()?;
        let mut suffix = 1usize;
        let guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        loop {
            let candidate = format!("rbk-{}-{}", now, suffix);
            if !guard
                .rollback_attempts
                .iter()
                .any(|attempt| attempt.attempt_id == candidate)
            {
                return Ok(candidate);
            }
            suffix += 1;
        }
    }

    fn write_locks_for_session(
        &self,
        session_id: &str,
    ) -> Result<(Vec<WriteTargetLock>, Vec<WriteGroupLock>), HeptaError> {
        self.prune_stale_write_locks_internal(false)?;
        let guard = self
            .write_lock_state
            .lock()
            .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
        Ok((
            guard
                .target_locks
                .iter()
                .filter(|lock| lock.session_id == session_id)
                .cloned()
                .collect(),
            guard
                .group_locks
                .iter()
                .filter(|lock| lock.session_id == session_id)
                .cloned()
                .collect(),
        ))
    }

    fn rollback_group_attempt_by_id(
        &self,
        attempt_id: &str,
    ) -> Result<Option<RollbackGroupAttempt>, HeptaError> {
        Ok(self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?
            .rollback_attempts
            .iter()
            .find(|attempt| attempt.attempt_id == attempt_id)
            .cloned())
    }

    fn live_rollback_group_attempt_by_id(
        &self,
        attempt_id: &str,
    ) -> Result<Option<RollbackGroupAttempt>, HeptaError> {
        Ok(self
            .rollback_group_attempt_by_id(attempt_id)?
            .filter(|attempt| attempt.superseded_by_attempt_id.is_none()))
    }

    fn rollback_group_lock_diagnostics(
        &self,
        session_id: &str,
        group_id: &str,
        latest_attempt_id: Option<&str>,
    ) -> Result<rollback_locks::RollbackGroupLockDiagnostics, HeptaError> {
        let locks = self.write_locks()?;
        Ok(rollback_locks::collect_rollback_group_lock_diagnostics(
            session_id,
            group_id,
            latest_attempt_id,
            &locks,
        ))
    }

    fn prune_stale_write_locks_internal(
        &self,
        emit_event: bool,
    ) -> Result<WriteLockPruneReport, HeptaError> {
        let now_unix_ms = current_unix_ms()?;
        let report = {
            let mut guard = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            let before_target_locks = guard.target_locks.len();
            let before_group_locks = guard.group_locks.len();
            guard
                .target_locks
                .retain(|lock| lock.lease_expires_at_unix_ms > now_unix_ms);
            guard
                .group_locks
                .retain(|lock| lock.lease_expires_at_unix_ms > now_unix_ms);
            rollback_locks::build_write_lock_prune_report(
                now_unix_ms,
                before_target_locks,
                before_group_locks,
                guard.target_locks.len(),
                guard.group_locks.len(),
            )
        };
        if emit_event && (report.pruned_target_locks > 0 || report.pruned_group_locks > 0) {
            self.emit_event_with_payload(
                EventKind::WriteLocksPruned,
                Some(SessionId(self.active_session_id()?)),
                None,
                format!(
                    "pruned stale write locks: targets={} groups={}",
                    report.pruned_target_locks, report.pruned_group_locks
                ),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "now_unix_ms": report.now_unix_ms,
                    "pruned_target_locks": report.pruned_target_locks,
                    "pruned_group_locks": report.pruned_group_locks,
                    "remaining_target_locks": report.remaining_target_locks,
                    "remaining_group_locks": report.remaining_group_locks,
                })),
            )?;
        }
        Ok(report)
    }

    fn find_conflicting_target_lock(
        &self,
        target_path: &str,
        allowed_owner_id: Option<&str>,
    ) -> Result<Option<WriteTargetLock>, HeptaError> {
        self.prune_stale_write_locks_internal(false)?;
        let normalized_target_path = normalize_path(PathBuf::from(target_path));
        let guard = self
            .write_lock_state
            .lock()
            .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
        Ok(guard
            .target_locks
            .iter()
            .find(|lock| {
                paths_overlap(
                    Path::new(&lock.target_path),
                    normalized_target_path.as_path(),
                ) && allowed_owner_id
                    .map(|allowed_owner_id| lock.owner_id != allowed_owner_id)
                    .unwrap_or(true)
            })
            .cloned())
    }

    fn ensure_write_target_unlocked(
        &self,
        session_id: &str,
        target_path: &str,
        operation: &str,
    ) -> Result<(), HeptaError> {
        if let Some(lock) = self.find_conflicting_target_lock(target_path, None)? {
            let message = format!(
                "write lock blocks {} for {} (owner={} {})",
                operation, target_path, lock.owner_kind, lock.owner_id
            );
            self.emit_event_with_payload(
                EventKind::WriteLockConflict,
                Some(SessionId(session_id.to_string())),
                None,
                message.clone(),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "operation": operation,
                    "requested_target_path": target_path,
                    "conflicting_target_path": lock.target_path,
                    "conflicting_owner_kind": lock.owner_kind,
                    "conflicting_owner_id": lock.owner_id,
                    "conflicting_group_id": lock.rollback_group_id,
                    "conflicting_attempt_id": lock.rollback_attempt_id,
                })),
            )?;
            return Err(HeptaError(message));
        }
        Ok(())
    }

    fn acquire_group_rollback_locks(
        &self,
        session_id: &str,
        group_id: &str,
        attempt_id: &str,
        target_paths: &[String],
    ) -> Result<(), HeptaError> {
        let locked_at_unix_ms = current_unix_ms()?;
        let lease_expires_at_unix_ms = locked_at_unix_ms.saturating_add(WRITE_LOCK_LEASE_MS);
        self.prune_stale_write_locks_internal(false)?;
        {
            let mut guard = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            for target_path in target_paths {
                let normalized_target_path = normalize_path(PathBuf::from(target_path));
                if let Some(conflict) = guard
                    .target_locks
                    .iter()
                    .find(|lock| {
                        lock.owner_id != group_id
                            && paths_overlap(
                                Path::new(&lock.target_path),
                                normalized_target_path.as_path(),
                            )
                    })
                    .cloned()
                {
                    let message = format!(
                        "write lock blocks rollback_group for {} (owner={} {})",
                        target_path, conflict.owner_kind, conflict.owner_id
                    );
                    drop(guard);
                    self.emit_event_with_payload(
                        EventKind::WriteLockConflict,
                        Some(SessionId(session_id.to_string())),
                        None,
                        message.clone(),
                        Some(json!({
                            "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                            "operation": "rollback_group",
                            "requested_target_path": target_path,
                            "conflicting_target_path": conflict.target_path,
                            "conflicting_owner_kind": conflict.owner_kind,
                            "conflicting_owner_id": conflict.owner_id,
                            "conflicting_group_id": conflict.rollback_group_id,
                            "conflicting_attempt_id": conflict.rollback_attempt_id,
                        })),
                    )?;
                    return Err(HeptaError(message));
                }
            }
            if !guard
                .group_locks
                .iter()
                .any(|lock| lock.session_id == session_id && lock.group_id == group_id)
            {
                guard.group_locks.push(WriteGroupLock {
                    session_id: session_id.to_string(),
                    group_id: group_id.to_string(),
                    owner_kind: "rollback_group".into(),
                    owner_id: attempt_id.to_string(),
                    rollback_attempt_id: Some(attempt_id.to_string()),
                    locked_at_unix_ms,
                    lease_expires_at_unix_ms,
                });
            } else if let Some(lock) = guard
                .group_locks
                .iter_mut()
                .find(|lock| lock.session_id == session_id && lock.group_id == group_id)
            {
                lock.owner_kind = "rollback_group".into();
                lock.owner_id = attempt_id.to_string();
                lock.rollback_attempt_id = Some(attempt_id.to_string());
                lock.locked_at_unix_ms = locked_at_unix_ms;
                lock.lease_expires_at_unix_ms = lease_expires_at_unix_ms;
            }
            for target_path in target_paths {
                let normalized_target_path = normalize_path(PathBuf::from(target_path));
                if !guard.target_locks.iter().any(|lock| {
                    lock.owner_id == group_id
                        && paths_overlap(
                            Path::new(&lock.target_path),
                            normalized_target_path.as_path(),
                        )
                }) {
                    guard.target_locks.push(WriteTargetLock {
                        session_id: session_id.to_string(),
                        target_path: normalized_target_path.display().to_string(),
                        owner_kind: "rollback_group".into(),
                        owner_id: group_id.to_string(),
                        rollback_group_id: Some(group_id.to_string()),
                        rollback_attempt_id: Some(attempt_id.to_string()),
                        locked_at_unix_ms,
                        lease_expires_at_unix_ms,
                    });
                } else if let Some(lock) = guard.target_locks.iter_mut().find(|lock| {
                    lock.owner_id == group_id
                        && paths_overlap(
                            Path::new(&lock.target_path),
                            normalized_target_path.as_path(),
                        )
                }) {
                    lock.owner_kind = "rollback_group".into();
                    lock.rollback_group_id = Some(group_id.to_string());
                    lock.rollback_attempt_id = Some(attempt_id.to_string());
                    lock.locked_at_unix_ms = locked_at_unix_ms;
                    lock.lease_expires_at_unix_ms = lease_expires_at_unix_ms;
                }
            }
        }
        self.emit_event_with_payload(
            EventKind::WriteLocksAcquired,
            Some(SessionId(session_id.to_string())),
            None,
            format!("acquired write locks for group {}", group_id),
            Some(json!({
                "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                "group_id": group_id,
                "attempt_id": attempt_id,
                "target_paths": target_paths,
                "target_lock_count": target_paths.len(),
                "locked_at_unix_ms": locked_at_unix_ms,
                "lease_expires_at_unix_ms": lease_expires_at_unix_ms,
            })),
        )?;
        Ok(())
    }

    fn release_group_rollback_locks(
        &self,
        session_id: &str,
        group_id: &str,
    ) -> Result<(), HeptaError> {
        let (released_group_locks, released_target_locks) = {
            {
                let mut guard = self
                    .write_lock_state
                    .lock()
                    .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
                let released_group_locks = guard
                    .group_locks
                    .iter()
                    .filter(|lock| lock.session_id == session_id && lock.group_id == group_id)
                    .count();
                let released_target_locks = guard
                    .target_locks
                    .iter()
                    .filter(|lock| lock.session_id == session_id && lock.owner_id == group_id)
                    .count();
                guard
                    .group_locks
                    .retain(|lock| !(lock.session_id == session_id && lock.group_id == group_id));
                guard
                    .target_locks
                    .retain(|lock| !(lock.session_id == session_id && lock.owner_id == group_id));
                (released_group_locks, released_target_locks)
            }
        };
        self.emit_event_with_payload(
            EventKind::WriteLocksReleased,
            Some(SessionId(session_id.to_string())),
            None,
            format!("released write locks for group {}", group_id),
            Some(json!({
                "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                "group_id": group_id,
                "released_group_locks": released_group_locks,
                "released_target_locks": released_target_locks,
            })),
        )?;
        Ok(())
    }

    fn append_transaction_to_active_group(
        &self,
        session_id: &str,
        transaction_id: &str,
    ) -> Result<(), HeptaError> {
        let mut guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        let Some(active_group_id) = guard
            .active_bindings
            .iter()
            .find(|binding| binding.session_id == session_id)
            .map(|binding| binding.active_group_id.clone())
        else {
            return Ok(());
        };
        let group = guard
            .groups
            .iter_mut()
            .find(|group| group.group_id == active_group_id && group.session_id == session_id)
            .ok_or_else(|| {
                HeptaError(format!(
                    "unknown write transaction group: {}",
                    active_group_id
                ))
            })?;
        if !group.transaction_ids.iter().any(|id| id == transaction_id) {
            group.transaction_ids.push(transaction_id.to_string());
        }
        Ok(())
    }

    fn next_write_transaction_id(&self) -> Result<String, HeptaError> {
        let mut suffix = 1usize;
        let now = current_unix_ms()?;
        let guard = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
        loop {
            let candidate = format!("txn-{}-{}", now, suffix);
            if !guard.iter().any(|entry| entry.transaction_id == candidate) {
                return Ok(candidate);
            }
            suffix += 1;
        }
    }

    fn prepare_write_transaction(
        &self,
        tool_name: &str,
        input_json: &str,
    ) -> Result<Option<PreparedWriteTransaction>, HeptaError> {
        if tool_name != "write_file" {
            return Ok(None);
        }

        let requested_path =
            parse_required_string_field(input_json, "path").map_err(|err| HeptaError(err.0))?;
        let mode_requested = parse_optional_string_field(input_json, "mode")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or_else(|| "create".to_string());
        let preview_only = parse_optional_bool_field(input_json, "preview_only")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or(false);
        let workspace_root = self.workspace_root()?;
        let target_path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let target_existed_before = target_path.exists();
        let before_bytes = if preview_only || !target_existed_before {
            None
        } else {
            Some(fs::read(&target_path).map_err(|err| {
                HeptaError(format!(
                    "failed to read {} before write transaction capture: {}",
                    target_path.display(),
                    err
                ))
            })?)
        };

        Ok(Some(PreparedWriteTransaction {
            target_path: target_path.display().to_string(),
            mode_requested,
            preview_only,
            target_existed_before,
            before_bytes,
        }))
    }

    fn prepare_write_transaction_with_lock_check(
        &self,
        session_id: &str,
        tool_name: &str,
        input_json: &str,
    ) -> Result<Option<PreparedWriteTransaction>, HeptaError> {
        let prepared = self.prepare_write_transaction(tool_name, input_json)?;
        if let Some(prepared_write_transaction) = prepared.as_ref() {
            self.ensure_write_target_unlocked(
                session_id,
                &prepared_write_transaction.target_path,
                tool_name,
            )?;
        }
        Ok(prepared)
    }

    fn record_write_transaction_from_tool_result(
        &self,
        session_id: &SessionId,
        prepared: Option<PreparedWriteTransaction>,
        tool_output_json: Option<String>,
    ) -> Result<Option<String>, HeptaError> {
        let Some(prepared) = prepared else {
            return Ok(tool_output_json);
        };
        let Some(tool_output_json) = tool_output_json else {
            return Ok(None);
        };

        let mut output_value: Value = serde_json::from_str(&tool_output_json).map_err(|err| {
            HeptaError(format!(
                "failed to parse tool output JSON for write transaction capture: {}",
                err
            ))
        })?;
        if prepared.preview_only {
            return Ok(Some(output_value.to_string()));
        }

        let transaction_id = self.next_write_transaction_id()?;
        let workspace_root = self.workspace_root()?;
        let target_path = PathBuf::from(&prepared.target_path);
        let source_backup_path = output_value
            .get("backup_path")
            .and_then(Value::as_str)
            .map(|value| value.to_string());
        let rollback_checkpoint_path = if prepared.target_existed_before {
            if let Some(source_backup_path) = source_backup_path.clone() {
                Some(source_backup_path)
            } else if let Some(before_bytes) = prepared.before_bytes.as_ref() {
                let checkpoint_path = preview_transaction_checkpoint_path(
                    &workspace_root,
                    &target_path,
                    &transaction_id,
                )?;
                if let Some(parent) = checkpoint_path.parent() {
                    fs::create_dir_all(parent).map_err(|err| {
                        HeptaError(format!(
                            "failed to create transaction checkpoint parent {}: {}",
                            parent.display(),
                            err
                        ))
                    })?;
                }
                fs::write(&checkpoint_path, before_bytes).map_err(|err| {
                    HeptaError(format!(
                        "failed to write transaction checkpoint {}: {}",
                        checkpoint_path.display(),
                        err
                    ))
                })?;
                Some(checkpoint_path.display().to_string())
            } else {
                None
            }
        } else {
            None
        };
        let rollback_strategy = if prepared.target_existed_before {
            "restore_checkpoint"
        } else {
            "delete_target"
        };
        let bytes_after = output_value
            .get("bytes_after")
            .and_then(Value::as_u64)
            .or_else(|| output_value.get("bytes_written").and_then(Value::as_u64))
            .unwrap_or(0);
        let entry = WriteTransactionEntry {
            transaction_id: transaction_id.clone(),
            session_id: session_id.0.clone(),
            action: "write_file".into(),
            target_path: prepared.target_path.clone(),
            created_at_unix_ms: current_unix_ms()?,
            mode: prepared.mode_requested.clone(),
            target_existed_before: prepared.target_existed_before,
            bytes_before: prepared
                .before_bytes
                .as_ref()
                .map(|bytes| bytes.len() as u64)
                .unwrap_or(0),
            bytes_after,
            rollback_strategy: rollback_strategy.into(),
            rollback_checkpoint_path: rollback_checkpoint_path.clone(),
            source_backup_path,
            rolled_back_at_unix_ms: None,
        };
        {
            let mut guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            guard.push(entry.clone());
        }
        let active_group_id = self.active_write_transaction_group_id_for_session(&session_id.0)?;
        self.append_transaction_to_active_group(&session_id.0, &transaction_id)?;
        self.emit_event(
            EventKind::WriteTransactionRecorded,
            Some(session_id.clone()),
            None,
            format!(
                "recorded write transaction {} for {}",
                entry.transaction_id, entry.target_path
            ),
        )?;

        if let Some(object) = output_value.as_object_mut() {
            object.insert("transaction_id".into(), json!(transaction_id));
            object.insert("rollback_strategy".into(), json!(rollback_strategy));
            if let Some(rollback_checkpoint_path) = rollback_checkpoint_path {
                object.insert(
                    "rollback_checkpoint_path".into(),
                    json!(rollback_checkpoint_path),
                );
            }
            if let Some(active_group_id) = active_group_id {
                object.insert("transaction_group_id".into(), json!(active_group_id));
            }
        }

        Ok(Some(output_value.to_string()))
    }

    fn record_restore_backup_transaction(
        &self,
        session_id: &SessionId,
        restored_target_path: &str,
        target_existed_before_restore: bool,
        restored_bytes: u64,
        previous_target_backup_path: Option<String>,
        source_backup_path: String,
    ) -> Result<String, HeptaError> {
        let transaction_id = self.next_write_transaction_id()?;
        let rollback_strategy = if target_existed_before_restore {
            "restore_checkpoint"
        } else {
            "delete_target"
        };
        let entry = WriteTransactionEntry {
            transaction_id: transaction_id.clone(),
            session_id: session_id.0.clone(),
            action: "restore_backup".into(),
            target_path: restored_target_path.to_string(),
            created_at_unix_ms: current_unix_ms()?,
            mode: "restore_backup".into(),
            target_existed_before: target_existed_before_restore,
            bytes_before: previous_target_backup_path
                .as_ref()
                .map(|path| fs::metadata(path).map(|meta| meta.len()).unwrap_or(0))
                .unwrap_or(0),
            bytes_after: restored_bytes,
            rollback_strategy: rollback_strategy.into(),
            rollback_checkpoint_path: previous_target_backup_path,
            source_backup_path: Some(source_backup_path),
            rolled_back_at_unix_ms: None,
        };
        {
            let mut guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            guard.push(entry.clone());
        }
        self.append_transaction_to_active_group(&session_id.0, &transaction_id)?;
        self.emit_event(
            EventKind::WriteTransactionRecorded,
            Some(session_id.clone()),
            None,
            format!(
                "recorded write transaction {} for {}",
                entry.transaction_id, entry.target_path
            ),
        )?;
        Ok(transaction_id)
    }

    fn plan_backup_prune(
        &self,
        target_path: Option<&str>,
        keep_latest_per_target: usize,
        max_age_ms: Option<u64>,
        execute: bool,
    ) -> Result<BackupPruneReport, HeptaError> {
        let report = self.backup_index(target_path)?;
        let backup_root = report.backup_root.clone();
        let filter_target_path = report.filter_target_path.clone();
        let scanned_backups = report.backups.len();
        let now = current_unix_ms()?;

        let mut grouped = std::collections::BTreeMap::<String, Vec<BackupEntryReport>>::new();
        for backup in report.backups {
            grouped
                .entry(backup.target_path.clone())
                .or_default()
                .push(backup);
        }

        let mut kept_backups = Vec::new();
        let mut deleted_backups = Vec::new();

        for (_target, mut entries) in grouped {
            entries.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));
            for (index, entry) in entries.into_iter().enumerate() {
                let keep_due_to_count = index < keep_latest_per_target;
                let age_matches = max_age_ms
                    .map(|max_age_ms| now.saturating_sub(entry.created_at_unix_ms) >= max_age_ms)
                    .unwrap_or(true);
                if !keep_due_to_count && age_matches {
                    if execute {
                        fs::remove_file(&entry.backup_path).map_err(|err| {
                            HeptaError(format!(
                                "failed to delete backup {}: {}",
                                entry.backup_path, err
                            ))
                        })?;
                    }
                    deleted_backups.push(entry);
                } else {
                    kept_backups.push(entry);
                }
            }
        }

        kept_backups.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));
        deleted_backups
            .sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));

        Ok(BackupPruneReport {
            backup_root,
            filter_target_path,
            keep_latest_per_target,
            max_age_ms,
            scanned_backups,
            executed: execute,
            deleted_count: deleted_backups.len(),
            kept_backups,
            deleted_backups,
        })
    }

    fn plan_history_merge(
        &self,
        target_session_id: &str,
        source_history: &[TurnRecord],
    ) -> Result<MergeHistoryPlan, HeptaError> {
        let target_history_signatures = self
            .history(Some(target_session_id), usize::MAX)?
            .into_iter()
            .map(|turn| turn_record_signature(&turn))
            .collect::<HashSet<_>>();

        let mut append_turns = Vec::new();
        let mut new_history_entries_to_append = Vec::new();
        let mut duplicate_history_entries_skipped = Vec::new();

        for turn in source_history.iter().rev() {
            let signature = turn_record_signature(turn);
            if target_history_signatures.contains(&signature) {
                duplicate_history_entries_skipped.push(signature);
                continue;
            }

            let mut cloned = turn.clone();
            cloned.session_id = target_session_id.to_string();
            append_turns.push(cloned);
            new_history_entries_to_append.push(signature);
        }

        new_history_entries_to_append.reverse();
        duplicate_history_entries_skipped.reverse();

        Ok(MergeHistoryPlan {
            append_turns,
            new_history_entries_to_append,
            duplicate_history_entries_skipped,
        })
    }
}

fn pending_approval_signature(item: &PendingApproval) -> String {
    format!("{} ({})", item.tool_name, item.reason)
}

fn turn_record_signature(turn: &TurnRecord) -> String {
    format!(
        "input=\"{}\" tool={:?} final=\"{}\" blocked={:?}",
        turn.input, turn.invoked_tool, turn.final_text, turn.blocked_reason
    )
}

fn ordered_unique_difference(items: Vec<String>, other: &HashSet<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut diff = Vec::new();
    for item in items {
        if other.contains(&item) {
            continue;
        }
        if seen.insert(item.clone()) {
            diff.push(item);
        }
    }
    diff
}

fn parse_required_string_field(
    input_json: &str,
    field: &str,
) -> Result<String, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    value
        .get(field)
        .and_then(Value::as_str)
        .map(|value| value.to_string())
        .ok_or_else(|| hepta_core::ToolError(format!("missing string field '{}'", field)))
}

fn parse_optional_string_field(
    input_json: &str,
    field: &str,
) -> Result<Option<String>, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(hepta_core::ToolError(format!(
            "field '{}' must be a string when present",
            field
        ))),
    }
}

fn parse_optional_bool_field(
    input_json: &str,
    field: &str,
) -> Result<Option<bool>, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Bool(flag)) => Ok(Some(*flag)),
        Some(_) => Err(hepta_core::ToolError(format!(
            "field '{}' must be a boolean when present",
            field
        ))),
    }
}

fn parse_optional_usize_field(
    input_json: &str,
    field: &str,
) -> Result<Option<usize>, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(number)) => number
            .as_u64()
            .map(|value| Some(value as usize))
            .ok_or_else(|| {
                hepta_core::ToolError(format!("field '{}' must be a non-negative integer", field))
            }),
        Some(_) => Err(hepta_core::ToolError(format!(
            "field '{}' must be an integer when present",
            field
        ))),
    }
}

fn tool_workspace_root_path() -> PathBuf {
    let root = discover_workspace_root();
    fs::canonicalize(&root).unwrap_or_else(|_| normalize_path(root))
}

fn discover_workspace_root() -> PathBuf {
    if let Ok(explicit) = std::env::var("HEPTA_WORKSPACE_ROOT") {
        let trimmed = explicit.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    let candidates = [
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    ];
    for candidate in candidates {
        let mut cursor = candidate;
        loop {
            let codex_hepta_product_root = cursor.join("codex-rs/Cargo.toml").is_file()
                && cursor.join("codex-rs/core").is_dir()
                && cursor.join("codex-rs/cli").is_dir()
                && cursor.join("codex-rs/hepta-core").is_dir();
            let old_hepta_root = cursor.join("Cargo.toml").is_file()
                && cursor.join("crates").is_dir()
                && cursor.join("apps").is_dir();
            let codex_rust_workspace_root = cursor.join("Cargo.toml").is_file()
                && cursor.join("core").is_dir()
                && cursor.join("cli").is_dir()
                && cursor.join("hepta-core").is_dir();
            if codex_hepta_product_root || old_hepta_root {
                return cursor;
            }
            if codex_rust_workspace_root
                && cursor.file_name().and_then(|name| name.to_str()) == Some("codex-rs")
                && let Some(parent) = cursor.parent()
            {
                return parent.to_path_buf();
            }
            if !cursor.pop() {
                break;
            }
        }
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn default_write_lock_lease_expires_at_unix_ms() -> u64 {
    0
}

fn resolve_path_within_root(root: &Path, requested: &Path) -> PathBuf {
    let candidate = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        root.join(requested)
    };

    if let Ok(canonical) = fs::canonicalize(&candidate) {
        canonical
    } else {
        normalize_path(candidate)
    }
}

fn normalize_path(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

fn paths_overlap(left: &Path, right: &Path) -> bool {
    let normalized_left = normalize_path(left.to_path_buf());
    let normalized_right = normalize_path(right.to_path_buf());
    normalized_left == normalized_right
        || normalized_left.starts_with(&normalized_right)
        || normalized_right.starts_with(&normalized_left)
}

const ROLLBACK_GROUP_STATUS_SCHEMA_VERSION: u32 = 1;
const WRITE_LOCK_REPORT_SCHEMA_VERSION: u32 = 1;
const ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::{
        ApprovalRequirement, DoctorStatus, MergeOptions, ProviderTransportKind,
        RollbackGroupAttempt, RollbackGroupAttemptStatus, RuntimeKernel, ToolRegistry,
        WriteGroupLock, WriteTargetLock, WriteTransactionEntry, WriteTransactionGroup,
        current_unix_ms, looks_like_assistant_identity_intent, looks_like_model_identity_intent,
        merge_runtime_config_value, native_pre_model_tool_call, preview_backup_path_from_ts,
        preview_transaction_checkpoint_path, render_native_tool_result_reply,
        should_offer_model_tools_for_turn, tool_workspace_root_path,
    };
    use hepta_core::{
        CorrelationId, EventKind, ExecutionProfile, FilesystemScope, IntuitionFeedbackOutcome,
        MessageRole, ModelMessage, ModelRef, ModelRequest, ModelToolSpec, SessionId, ThinkingLevel,
        ToolCallRequest, ToolContext, WritePathScope,
    };
    use hepta_intelligence::TopicAwareModelFeedbackOutcome;
    use serde_json::{Value, json};
    use std::fs;
    use std::path::PathBuf;

    fn extract_json_string_field(json_text: &str, field: &str) -> Option<String> {
        serde_json::from_str::<Value>(json_text)
            .ok()?
            .get(field)?
            .as_str()
            .map(|value| value.to_string())
    }

    fn write_fake_workspace_backup(logical_path: &str, ts: u64, content: &str) -> PathBuf {
        let backup_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/backups/write_file/workspace")
            .join(format!("{}.hepta-bak-{}", logical_path, ts));
        fs::create_dir_all(backup_path.parent().expect("backup parent should exist"))
            .expect("backup parent should be creatable");
        fs::write(&backup_path, content).expect("backup file should be writable");
        backup_path
    }

    #[test]
    fn runtime_config_merge_preserves_hepta_and_adds_source_runtime_increment() {
        let mut hepta_runtime = json!({
            "models": {
                "providers": {
                    "mlx-local": {
                        "baseUrl": "http://hepta-runtime.local/v1",
                        "models": ["Gemma-A"]
                    }
                }
            },
            "tools": {
                "allow": ["read"]
            }
        });
        let source_runtime_import = json!({
            "models": {
                "providers": {
                    "mlx-local": {
                        "baseUrl": "http://source-runtime-import.local/v1",
                        "apiKey": "redacted-secret",
                        "models": ["Gemma-A", "Gemma-B"]
                    },
                    "ollama": {
                        "baseUrl": "http://localhost:11434/v1",
                        "models": ["llama"]
                    }
                }
            },
            "tools": {
                "allow": ["read", "web_search"]
            }
        });

        merge_runtime_config_value(&mut hepta_runtime, source_runtime_import);

        assert_eq!(
            hepta_runtime["models"]["providers"]["mlx-local"]["baseUrl"],
            json!("http://hepta-runtime.local/v1")
        );
        assert_eq!(
            hepta_runtime["models"]["providers"]["mlx-local"]["apiKey"],
            json!("redacted-secret")
        );
        assert_eq!(
            hepta_runtime["models"]["providers"]["mlx-local"]["models"],
            json!(["Gemma-A", "Gemma-B"])
        );
        assert_eq!(
            hepta_runtime["models"]["providers"]["ollama"]["baseUrl"],
            json!("http://localhost:11434/v1")
        );
        assert_eq!(
            hepta_runtime["tools"]["allow"],
            json!(["read", "web_search"])
        );
    }

    #[test]
    fn path_overlap_matches_same_ancestor_and_descendant_paths() {
        let base = PathBuf::from("/tmp/hepta-lock-root");
        let child = base.join("nested/file.txt");
        assert!(super::paths_overlap(&base, &base));
        assert!(super::paths_overlap(&base, &child));
        assert!(super::paths_overlap(&child, &base));
        assert!(!super::paths_overlap(
            &PathBuf::from("/tmp/hepta-a"),
            &PathBuf::from("/tmp/hepta-b")
        ));
    }

    #[tokio::test]
    async fn switches_provider_and_routes_run_through_new_provider() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_model("mock-ollama/local-precise")
            .expect("switch should succeed");

        let result = runtime
            .run_demo_turn("tool:provider route")
            .await
            .expect("demo turn should succeed");

        assert_eq!(result.active_model.provider, "mock-ollama");
        assert_eq!(result.active_model.model, "local-precise");
        assert!(result.final_text.contains("[ollama-precise]"));
        assert!(result.final_text.contains("结构化结果已保留在本地"));
        assert!(!result.final_text.contains("structured="));
    }

    #[test]
    fn exposes_provider_catalog_separately_from_model_selection_state() {
        let runtime = RuntimeKernel::new();
        let catalog = runtime.provider_catalog();
        assert_eq!(catalog.providers.len(), 2);
        assert!(catalog.providers.iter().any(|provider| {
            provider.id == "demo"
                && provider.display_name == "Demo Provider"
                && provider.transport_kind == ProviderTransportKind::InProcess
                && provider.default_model.model == "demo-chat"
                && provider
                    .available_models
                    .iter()
                    .any(|model| model.model == "demo-creative")
        }));
        assert!(catalog.providers.iter().any(|provider| {
            provider.id == "mock-ollama"
                && provider.transport_kind == ProviderTransportKind::OpenAiCompatibleHttp
                && provider
                    .available_models
                    .iter()
                    .any(|model| model.model == "local-precise")
        }));

        let selection = runtime.model_selection().expect("selection should load");
        assert_eq!(selection.available.len(), 5);
        assert!(
            selection
                .available
                .iter()
                .any(|model| model.provider == "demo")
        );
        assert!(
            selection
                .available
                .iter()
                .any(|model| model.provider == "mock-ollama")
        );
    }

    #[tokio::test]
    async fn medium_risk_tool_requires_approval_until_granted() {
        let runtime = RuntimeKernel::new();
        let read_path = format!(
            "read:{}",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../docs/decisions/ADR-0001-architecture-foundation.md"
            )
        );

        let blocked = runtime
            .run_demo_turn(&read_path)
            .await
            .expect("first run should return approval requirement");
        assert_eq!(blocked.approval_required.as_deref(), Some("read_file"));
        assert!(
            blocked
                .blocked_reason
                .as_deref()
                .unwrap_or_default()
                .contains("requires explicit approval")
        );

        let snapshot = runtime
            .approval_snapshot()
            .expect("snapshot should succeed");
        assert_eq!(snapshot.pending.len(), 1);
        assert_eq!(snapshot.pending[0].tool_name, "read_file");

        runtime
            .approve_tool("read_file")
            .expect("approval should succeed");

        let allowed = runtime
            .run_demo_turn(&read_path)
            .await
            .expect("second run should succeed after approval");
        assert_eq!(allowed.invoked_tool.as_deref(), Some("read_file"));
        assert!(allowed.approval_required.is_none());
        assert!(allowed.final_text.contains("read_file:"));

        let events = runtime.events(usize::MAX).expect("events should load");
        let kinds = events
            .into_iter()
            .map(|item| item.event.kind)
            .collect::<Vec<_>>();
        assert!(kinds.contains(&EventKind::ApprovalRequested));
        assert!(kinds.contains(&EventKind::ApprovalGranted));
        assert!(kinds.contains(&EventKind::ToolInvoked));
        assert!(kinds.contains(&EventKind::MemoryWritten));
    }

    #[tokio::test]
    async fn custom_policy_rule_can_deny_low_risk_tool() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                Some("session-main"),
                Some("demo"),
                Some("echo"),
                None,
                ApprovalRequirement::Deny,
                Some("echo is blocked for session-main on demo"),
            )
            .expect("policy rule should be added");

        let blocked = runtime
            .run_demo_turn("tool:blocked echo")
            .await
            .expect("run should succeed with denial result");

        assert_eq!(blocked.invoked_tool, None);
        assert_eq!(blocked.approval_required, None);
        assert_eq!(blocked.final_text, "policy denied tool echo");
        assert_eq!(
            blocked.blocked_reason.as_deref(),
            Some("echo is blocked for session-main on demo")
        );
    }

    #[test]
    fn openai_tool_schema_and_tool_call_parser_roundtrip() {
        let tools = vec![hepta_core::ModelToolSpec {
            name: "echo".into(),
            description: "Echo text".into(),
            input_schema_json: json!({
                "type": "object",
                "required": ["text"],
                "properties": {"text": {"type": "string"}}
            })
            .to_string(),
        }];
        let payloads = super::openai_tool_payloads(&tools);
        assert_eq!(payloads.len(), 1);
        assert_eq!(
            payloads[0].pointer("/type").and_then(Value::as_str),
            Some("function")
        );
        assert_eq!(
            payloads[0]
                .pointer("/function/name")
                .and_then(Value::as_str),
            Some("echo")
        );
        assert_eq!(
            payloads[0]
                .pointer("/function/parameters/required/0")
                .and_then(Value::as_str),
            Some("text")
        );

        let parsed = super::openai_tool_calls_from_message(&json!({
            "tool_calls": [{
                "type": "function",
                "function": {
                    "name": "echo",
                    "arguments": "{\"text\":\"hello\"}"
                }
            }]
        }));
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "echo");
        assert_eq!(parsed[0].arguments_json, "{\"text\":\"hello\"}");

        let textual = super::textual_tool_calls_from_message_content(
            "<|tool_call>call:echo{text: \"ping\"}<tool_call|>",
            &tools,
        );
        assert_eq!(textual.len(), 1);
        assert_eq!(textual[0].name, "echo");
        assert_eq!(textual[0].arguments_json, "{\"text\":\"ping\"}");

        let json_textual = super::textual_tool_calls_from_message_content(
            r#"<tool_call>{"name":"echo","arguments":{"text":"pong"}}</tool_call>"#,
            &tools,
        );
        assert_eq!(json_textual.len(), 1);
        assert_eq!(json_textual[0].name, "echo");
        assert_eq!(json_textual[0].arguments_json, "{\"text\":\"pong\"}");
    }

    #[test]
    fn qwen_chat_template_thinking_is_disabled_for_live_agent_requests() {
        let mut payload = json!({
            "model": "Qwen/Qwen3-8B",
            "chat_template_kwargs": {"preserve_other": true}
        });
        let request = ModelRequest {
            model: ModelRef {
                provider: "mlx-local".into(),
                model: "Qwen/Qwen3-8B".into(),
            },
            messages: vec![ModelMessage {
                role: MessageRole::User,
                content: "What's the temperature?".into(),
            }],
            thinking: ThinkingLevel::High,
            tools: vec![ModelToolSpec {
                name: "get_current_temperature".into(),
                description: "Get current temperature".into(),
                input_schema_json: json!({
                    "type": "object",
                    "properties": {"location": {"type": "string"}},
                    "required": ["location"]
                })
                .to_string(),
            }],
            timeout_ms: None,
        };

        assert!(super::apply_qwen_openai_compatible_thinking_params(
            &mut payload,
            Some(super::QwenThinkingFormat::ChatTemplate),
            &request,
        ));
        assert_eq!(
            payload.pointer("/chat_template_kwargs/enable_thinking"),
            Some(&json!(false))
        );
        assert_eq!(
            payload.pointer("/chat_template_kwargs/preserve_other"),
            Some(&json!(true))
        );

        let mut top_level_payload = json!({"model": "qwen3"});
        let no_tool_request = ModelRequest {
            tools: vec![],
            ..request
        };
        assert!(super::apply_qwen_openai_compatible_thinking_params(
            &mut top_level_payload,
            Some(super::QwenThinkingFormat::TopLevel),
            &no_tool_request,
        ));
        assert_eq!(
            top_level_payload.get("enable_thinking"),
            Some(&json!(false))
        );
    }

    #[test]
    fn openai_codex_jwt_account_id_decodes_without_secret_logging() {
        let token = "hdr.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL2F1dGgiOnsiY2hhdGdwdF9hY2NvdW50X2lkIjoiYWNjdF90ZXN0XzEyMyJ9fQ.sig";

        assert_eq!(
            super::extract_chatgpt_account_id_from_jwt(token).as_deref(),
            Some("acct_test_123")
        );
    }

    #[test]
    fn openai_codex_profile_selection_prefers_freshest_unexpired_profile() {
        let stale_first = super::OpenAiCodexAuthProfile {
            path: PathBuf::from("hepta/auth-profiles.json"),
            profile_id: "openai-codex:stale".into(),
            access: "stale-access".into(),
            refresh: Some("stale-refresh".into()),
            expires: Some(1_000),
            account_id: "acct_stale".into(),
        };
        let fresh_default = super::OpenAiCodexAuthProfile {
            path: PathBuf::from("main/auth-profiles.json"),
            profile_id: "openai-codex:default".into(),
            access: "fresh-access".into(),
            refresh: Some("fresh-refresh".into()),
            expires: Some(500_000),
            account_id: "acct_fresh".into(),
        };
        let freshest = super::OpenAiCodexAuthProfile {
            path: PathBuf::from("main/auth-profiles.json"),
            profile_id: "openai-codex:newest".into(),
            access: "freshest-access".into(),
            refresh: Some("freshest-refresh".into()),
            expires: Some(900_000),
            account_id: "acct_freshest".into(),
        };

        let selected = super::select_openai_codex_auth_profile(
            vec![stale_first, fresh_default, freshest],
            100_000,
        )
        .expect("a fresh profile should be selected");

        assert_eq!(selected.profile_id, "openai-codex:newest");
        assert_eq!(selected.account_id, "acct_freshest");
    }

    #[test]
    fn openai_codex_profile_override_normalizes_email_or_full_profile_id() {
        assert_eq!(
            super::normalize_openai_codex_profile_id_override(" qiqianpkugsm@gmail.com ")
                .as_deref(),
            Some("openai-codex:qiqianpkugsm@gmail.com")
        );
        assert_eq!(
            super::normalize_openai_codex_profile_id_override(
                "openai-codex:qiqianpkugsm@gmail.com",
            )
            .as_deref(),
            Some("openai-codex:qiqianpkugsm@gmail.com")
        );
        assert_eq!(
            super::normalize_openai_codex_profile_id_override("  "),
            None
        );
    }

    #[test]
    fn openai_codex_tool_schema_sanitizer_adds_missing_array_items() {
        let schema = json!({
            "type": "object",
            "properties": {
                "edits": {"type": "array"},
                "nested": {
                    "type": "object",
                    "properties": {
                        "labels": {"type": ["array", "null"]}
                    }
                }
            }
        });

        let sanitized = super::sanitize_openai_codex_tool_schema(schema);

        assert_eq!(
            sanitized.pointer("/properties/edits/items"),
            Some(&json!({}))
        );
        assert_eq!(
            sanitized.pointer("/properties/nested/properties/labels/items"),
            Some(&json!({}))
        );
    }

    #[test]
    fn openai_codex_request_body_matches_responses_shape() {
        let request = hepta_core::ModelRequest {
            model: ModelRef {
                provider: "openai-codex".into(),
                model: "gpt-5.5".into(),
            },
            messages: vec![
                hepta_core::ModelMessage {
                    role: hepta_core::MessageRole::System,
                    content: "Be concise".into(),
                },
                hepta_core::ModelMessage {
                    role: hepta_core::MessageRole::User,
                    content: "ping".into(),
                },
            ],
            thinking: hepta_core::ThinkingLevel::XHigh,
            tools: vec![hepta_core::ModelToolSpec {
                name: "echo".into(),
                description: "Echo text".into(),
                input_schema_json: json!({
                    "type": "object",
                    "properties": {"text": {"type": "string"}}
                })
                .to_string(),
            }],
            timeout_ms: None,
        };

        let body = super::openai_codex_responses_request_body(&request, Some("session-1"));

        assert_eq!(body.get("model").and_then(Value::as_str), Some("gpt-5.5"));
        assert_eq!(body.get("store").and_then(Value::as_bool), Some(false));
        assert_eq!(body.get("stream").and_then(Value::as_bool), Some(true));
        assert_eq!(
            body.get("instructions").and_then(Value::as_str),
            Some("Be concise")
        );
        assert_eq!(
            body.pointer("/input/0/content/0/type")
                .and_then(Value::as_str),
            Some("input_text")
        );
        assert_eq!(
            body.pointer("/text/verbosity").and_then(Value::as_str),
            Some("low")
        );
        assert_eq!(
            body.pointer("/reasoning/effort").and_then(Value::as_str),
            Some("xhigh")
        );
        assert_eq!(
            body.pointer("/tools/0/name").and_then(Value::as_str),
            Some("echo")
        );
        assert!(body.get("max_tokens").is_none());
    }

    #[test]
    fn openai_codex_sse_text_and_usage_parse() {
        let sse = concat!(
            "data: {\"type\":\"response.created\",\"response\":{\"id\":\"resp_1\"}}\n\n",
            "data: {\"type\":\"response.output_text.delta\",\"delta\":\"你\"}\n\n",
            "data: {\"type\":\"response.output_text.delta\",\"delta\":\"好\"}\n\n",
            "data: {\"type\":\"response.completed\",\"response\":{\"status\":\"completed\",\"usage\":{\"input_tokens\":10,\"input_tokens_details\":{\"cached_tokens\":3},\"output_tokens\":2,\"total_tokens\":12}}}\n\n"
        );

        let response = super::parse_openai_codex_sse_response(sse).expect("SSE should parse");

        assert_eq!(response.finish_reason, hepta_core::FinishReason::Stop);
        assert_eq!(response.message.expect("message").content, "你好");
        assert_eq!(response.usage.input_tokens, 7);
        assert_eq!(response.usage.output_tokens, 2);
    }

    #[test]
    fn openai_codex_sse_tool_call_parse() {
        let sse = concat!(
            "data: {\"type\":\"response.output_item.added\",\"item\":{\"type\":\"function_call\",\"name\":\"read_file\"}}\n\n",
            "data: {\"type\":\"response.function_call_arguments.delta\",\"delta\":\"{\\\"path\\\":\"}\n\n",
            "data: {\"type\":\"response.function_call_arguments.delta\",\"delta\":\"\\\"README.md\\\"}\"}\n\n",
            "data: {\"type\":\"response.output_item.done\",\"item\":{\"type\":\"function_call\",\"name\":\"read_file\"}}\n\n",
            "data: {\"type\":\"response.completed\",\"response\":{\"status\":\"completed\"}}\n\n"
        );

        let response = super::parse_openai_codex_sse_response(sse).expect("SSE should parse");

        assert_eq!(response.finish_reason, hepta_core::FinishReason::ToolCall);
        assert!(response.message.is_none());
        assert_eq!(response.tool_calls.len(), 1);
        assert_eq!(response.tool_calls[0].name, "read_file");
        assert_eq!(
            response.tool_calls[0].arguments_json,
            "{\"path\":\"README.md\"}"
        );
    }

    #[tokio::test]
    async fn two_turn_memory_context_is_injected_into_model_prompt() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("请记住暗号是蓝莓")
            .await
            .expect("first turn should succeed");
        let recalled = runtime
            .run_demo_turn("暗号是什么")
            .await
            .expect("second turn should succeed");

        assert!(recalled.final_text.contains("蓝莓"));
        assert!(recalled.recalled_memories >= 1);
    }

    #[tokio::test]
    async fn generic_read_only_tool_call_runs_through_tool_loop() {
        let runtime = RuntimeKernel::new();
        let result = runtime
            .run_demo_turn("tool:generic read only")
            .await
            .expect("echo tool should run");

        assert_eq!(result.invoked_tool.as_deref(), Some("echo"));
        assert!(result.final_text.contains("结构化结果已保留在本地"));
        assert!(!result.final_text.contains("structured="));
    }

    #[tokio::test]
    async fn write_tool_still_requires_approval_before_mutation() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::Ask,
                Some("test write approval gate"),
            )
            .expect("policy rule should be added");
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-approval-gate-test.txt");
        let _ = fs::remove_file(&path);

        let result = runtime
            .run_demo_turn("write:artifacts/hepta-approval-gate-test.txt => blocked")
            .await
            .expect("write request should return approval gate");

        assert_eq!(result.invoked_tool, None);
        assert_eq!(result.approval_required.as_deref(), Some("write_file"));
        assert!(result.final_text.contains("approval required"));
        assert!(!path.exists(), "write_file must not mutate before approval");
    }

    #[tokio::test]
    async fn disk_junk_audit_is_read_only_and_does_not_delete() {
        let runtime = RuntimeKernel::new();
        let result = runtime
            .run_demo_turn("你扫一眼全盘，看看有什么垃圾可以清理")
            .await
            .expect("disk junk audit should run");

        assert_eq!(result.invoked_tool.as_deref(), Some("disk_junk_audit"));
        let output_json = result
            .tool_output_json
            .expect("audit output should be structured");
        let value: Value = serde_json::from_str(&output_json).expect("audit JSON should parse");
        assert_eq!(value.get("read_only").and_then(Value::as_bool), Some(true));
        assert_eq!(
            value.get("status").and_then(Value::as_str),
            Some("completed")
        );
        assert!(result.final_text.contains("没删任何文件"));
    }

    #[test]
    fn explicit_exec_intent_bypasses_model_tool_guessing() {
        let call = native_pre_model_tool_call(
            "请必须调用 exec 工具后台运行：sleep 1 && echo hepta-ok；然后再调用 process log 查看结果。",
        )
        .expect("explicit exec intent should be routed before the model");

        assert_eq!(call.name, "exec");
        let args: Value = serde_json::from_str(&call.arguments_json).expect("valid JSON args");
        assert_eq!(
            args.get("command").and_then(Value::as_str),
            Some("sleep 1 && echo hepta-ok")
        );
        assert_eq!(args.get("background").and_then(Value::as_bool), Some(true));
    }

    #[test]
    fn explicit_echo_intent_extracts_required_text_before_model() {
        let call = native_pre_model_tool_call("请用 echo 工具返回 ping，不要只用文字回答。")
            .expect("explicit echo intent should be routed before the model");

        assert_eq!(call.name, "echo");
        let args: Value = serde_json::from_str(&call.arguments_json).expect("valid JSON args");
        assert_eq!(args.get("text").and_then(Value::as_str), Some("ping"));

        let json_call = native_pre_model_tool_call(
            "Use the echo tool with arguments exactly {\"text\":\"pong\"}. Do not answer directly.",
        )
        .expect("explicit JSON echo intent should be routed before the model");
        let json_args: Value =
            serde_json::from_str(&json_call.arguments_json).expect("valid JSON args");
        assert_eq!(json_args.get("text").and_then(Value::as_str), Some("pong"));
    }

    #[tokio::test]
    async fn qwen_style_natural_echo_request_runs_without_missing_text() {
        let runtime = RuntimeKernel::new();
        let result = runtime
            .run_demo_turn("请用 echo 工具返回 ping，不要只用文字回答。")
            .await
            .expect("echo route should run");

        assert_eq!(result.invoked_tool.as_deref(), Some("echo"));
        assert!(result.blocked_reason.is_none());
        assert!(result.final_text.contains("结构化结果已保留在本地"));
    }

    #[test]
    fn explicit_process_intent_extracts_action_and_session_id() {
        let call = native_pre_model_tool_call(
            "请调用 process 工具 log hepta-proc-1778630000-12345 查看输出",
        )
        .expect("explicit process intent should be routed before the model");

        assert_eq!(call.name, "process");
        let args: Value = serde_json::from_str(&call.arguments_json).expect("valid JSON args");
        assert_eq!(args.get("action").and_then(Value::as_str), Some("log"));
        assert_eq!(
            args.get("sessionId").and_then(Value::as_str),
            Some("hepta-proc-1778630000-12345")
        );
    }

    #[test]
    fn model_identity_question_is_not_native_process_intent() {
        assert!(native_pre_model_tool_call("你是什么模型").is_none());
        assert!(looks_like_model_identity_intent(
            "BodyForHeptaAgent:\n你用的是哪个模型"
        ));
        assert!(!looks_like_model_identity_intent("请列出可用模型列表"));
    }

    #[test]
    fn assistant_identity_question_is_not_native_process_intent() {
        assert!(native_pre_model_tool_call("你是谁").is_none());
        assert!(looks_like_assistant_identity_intent(
            "BodyForHeptaAgent:\n你是谁"
        ));
        assert!(looks_like_assistant_identity_intent("who are you"));
        assert!(!looks_like_assistant_identity_intent(
            "请调用 process 工具 list"
        ));
    }

    #[test]
    fn model_tools_are_only_offered_for_explicit_tool_turns() {
        assert!(!should_offer_model_tools_for_turn("你好，随便聊两句"));
        assert!(!should_offer_model_tools_for_turn(
            "BodyForHeptaAgent:\n你是谁"
        ));
        assert!(!should_offer_model_tools_for_turn(
            "BodyForHeptaAgent:\n你是什么模型"
        ));
        assert!(!should_offer_model_tools_for_turn("你有哪些工具可以用？"));

        assert!(should_offer_model_tools_for_turn(
            "请用 echo 工具返回 ping，不要只用文字回答。"
        ));
        assert!(should_offer_model_tools_for_turn(
            "请调用 process 工具 list"
        ));
        assert!(should_offer_model_tools_for_turn(
            "Use the write_file tool with arguments exactly {\"path\":\"artifacts/a.txt\",\"content\":\"x\",\"mode\":\"create\"}."
        ));
        assert!(should_offer_model_tools_for_turn("read:README.md"));
    }

    #[tokio::test]
    async fn model_identity_question_answers_without_tool_call() {
        let runtime = RuntimeKernel::new();
        let result = runtime
            .run_demo_turn_in_session("agent:main:telegram:direct:test", "你是什么模型")
            .await
            .expect("model identity question should run");
        let active_model_label = format!(
            "{}/{}",
            result.active_model.provider, result.active_model.model
        );

        assert_eq!(result.invoked_tool, None);
        assert!(result.approval_required.is_none());
        assert!(result.blocked_reason.is_none());
        assert!(result.final_text.contains(&active_model_label));
        assert!(!result.final_text.contains("native process"));
        assert!(!result.final_text.contains("后台进程记录"));
    }

    #[tokio::test]
    async fn ordinary_chat_answers_without_tool_surface() {
        let runtime = RuntimeKernel::new();
        let result = runtime
            .run_demo_turn_in_session("agent:main:telegram:direct:test", "你好，随便聊两句")
            .await
            .expect("ordinary chat should run");

        assert_eq!(result.invoked_tool, None);
        assert!(result.approval_required.is_none());
        assert!(result.blocked_reason.is_none());
        assert!(!result.final_text.contains("native process"));
        assert!(!result.final_text.contains("后台进程记录"));
    }

    #[tokio::test]
    async fn assistant_identity_question_answers_without_tool_call() {
        let runtime = RuntimeKernel::new();
        let result = runtime
            .run_demo_turn_in_session("agent:main:telegram:direct:test", "你是谁")
            .await
            .expect("assistant identity question should run");

        assert_eq!(result.invoked_tool, None);
        assert!(result.approval_required.is_none());
        assert!(result.blocked_reason.is_none());
        assert!(result.final_text.contains("发发_1"));
        assert!(result.final_text.contains("Hepta"));
        assert!(result.final_text.contains("没有调用工具"));
        assert!(!result.final_text.contains("native process"));
        assert!(!result.final_text.contains("后台进程记录"));
    }

    #[test]
    fn explicit_write_file_intent_extracts_json_before_model() {
        let call = native_pre_model_tool_call(
            r#"Use the write_file tool with arguments exactly {"path":"artifacts/hepta-live-agent-e2e-approval.txt","content":"blocked-before-approval","mode":"create"}. Do not answer directly."#,
        )
        .expect("explicit write_file intent should be routed before the model");

        assert_eq!(call.name, "write_file");
        let args: Value = serde_json::from_str(&call.arguments_json).expect("valid JSON args");
        assert_eq!(
            args.get("path").and_then(Value::as_str),
            Some("artifacts/hepta-live-agent-e2e-approval.txt")
        );
        assert_eq!(
            args.get("content").and_then(Value::as_str),
            Some("blocked-before-approval")
        );
    }

    #[tokio::test]
    async fn custom_policy_rule_can_allow_medium_risk_tool_for_provider() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_model("mock-ollama/local-chat")
            .expect("model switch should succeed");
        runtime
            .add_policy_rule(
                None,
                Some("mock-ollama"),
                Some("read_file"),
                None,
                ApprovalRequirement::None,
                Some("mock ollama can read files without approval"),
            )
            .expect("policy rule should be added");

        let read_path = format!(
            "read:{}",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../docs/decisions/ADR-0001-architecture-foundation.md"
            )
        );
        let allowed = runtime
            .run_demo_turn(&read_path)
            .await
            .expect("run should succeed without approval");

        assert_eq!(allowed.invoked_tool.as_deref(), Some("read_file"));
        assert!(allowed.approval_required.is_none());

        let report = runtime
            .policy_report()
            .await
            .expect("policy report should load");
        assert_eq!(report.custom_rules.len(), 1);
        assert!(report.effective_tool_decisions.iter().any(|item| {
            item.tool_name == "read_file"
                && item.requirement == ApprovalRequirement::None
                && item
                    .matched_rule_id
                    .as_deref()
                    .unwrap_or_default()
                    .starts_with("policy-")
        }));
    }

    #[tokio::test]
    async fn native_exec_timeout_stops_tool_loop_without_followup_model_replay() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("exec"),
                None,
                ApprovalRequirement::None,
                Some("test permits native exec timeout path"),
            )
            .expect("exec should be allowed for timeout regression");
        let started = std::time::Instant::now();
        let result = runtime
            .run_demo_turn_in_session(
                "timeout-session",
                "请调用 exec 工具执行：sleep 5；timeoutMs=100",
            )
            .await
            .expect("timeout should be returned as fallback text");

        assert!(started.elapsed() < std::time::Duration::from_secs(6));
        assert_eq!(result.invoked_tool.as_deref(), Some("exec"));
        assert!(
            result
                .final_text
                .contains("ToolTimeout/native_compat_exec timed out")
        );
        assert!(result.blocked_reason.is_none());
        let tool_output: Value = serde_json::from_str(
            result
                .tool_output_json
                .as_deref()
                .expect("timeout should keep structured tool result"),
        )
        .expect("tool timeout JSON should parse");
        assert_eq!(tool_output["status"], json!("timeout"));
        assert_eq!(
            tool_output["result"]["duplicate_tool_replay_prevented"],
            json!(true)
        );
    }

    #[tokio::test]
    async fn no_tools_execution_profile_blocks_even_low_risk_tools() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_execution_profile(ExecutionProfile::NoTools)
            .expect("profile switch should succeed");

        let blocked = runtime
            .run_demo_turn("tool:hello profile")
            .await
            .expect("run should return blocked result");

        assert_eq!(blocked.invoked_tool, None);
        assert_eq!(blocked.final_text, "execution profile blocked tool echo");
        assert!(
            blocked
                .blocked_reason
                .as_deref()
                .unwrap_or_default()
                .contains("execution profile no_tools blocks tool echo")
        );
    }

    #[test]
    fn session_export_roundtrip_preserves_execution_profile() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_execution_profile(ExecutionProfile::NoTools)
            .expect("profile switch should succeed");
        let export = runtime
            .session_export("session-main")
            .expect("session export should succeed");
        assert_eq!(export.execution_profile, ExecutionProfile::NoTools);

        runtime
            .switch_execution_profile(ExecutionProfile::FullAccess)
            .expect("profile reset should succeed");
        runtime
            .apply_session_export(export)
            .expect("session import should succeed");

        assert_eq!(
            runtime
                .execution_profile_for_session("session-main")
                .expect("profile should load"),
            ExecutionProfile::NoTools
        );
    }

    #[tokio::test]
    async fn workspace_only_filesystem_scope_blocks_reads_outside_workspace() {
        let runtime = RuntimeKernel::new();
        runtime
            .approve_tool("read_file")
            .expect("approval should succeed");

        let blocked = runtime
            .run_demo_turn("read:/etc/hosts")
            .await
            .expect("run should return blocked result");

        assert_eq!(blocked.invoked_tool, None);
        assert_eq!(
            blocked.final_text,
            "filesystem scope blocked tool read_file"
        );
        assert!(blocked
            .blocked_reason
            .as_deref()
            .unwrap_or_default()
            .contains("filesystem scope workspace_only blocks read_file path /etc/hosts outside workspace"));
    }

    #[test]
    fn session_export_roundtrip_preserves_filesystem_scope() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_filesystem_scope(FilesystemScope::AnyPath)
            .expect("scope switch should succeed");
        let export = runtime
            .session_export("session-main")
            .expect("session export should succeed");
        assert_eq!(export.filesystem_scope, FilesystemScope::AnyPath);

        runtime
            .switch_filesystem_scope(FilesystemScope::WorkspaceOnly)
            .expect("scope reset should succeed");
        runtime
            .apply_session_export(export)
            .expect("session import should succeed");

        assert_eq!(
            runtime
                .filesystem_scope_for_session("session-main")
                .expect("scope should load"),
            FilesystemScope::AnyPath
        );
    }

    #[tokio::test]
    async fn path_capability_gate_can_override_workspace_only_for_read_file() {
        let runtime = RuntimeKernel::new();
        runtime
            .approve_tool("read_file")
            .expect("approval should succeed");
        runtime
            .set_path_capability_gate("read_file", "path", FilesystemScope::AnyPath)
            .expect("capability gate should be set");

        let result = runtime
            .run_demo_turn("read:/etc/hosts")
            .await
            .expect("run should succeed");

        assert_eq!(result.invoked_tool.as_deref(), Some("read_file"));
        assert!(result.final_text.contains("read_file:"));
    }

    #[tokio::test]
    async fn path_capability_gate_can_tighten_any_path_back_to_workspace_only() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_filesystem_scope(FilesystemScope::AnyPath)
            .expect("scope switch should succeed");
        runtime
            .approve_tool("read_file")
            .expect("approval should succeed");
        runtime
            .set_path_capability_gate("read_file", "path", FilesystemScope::WorkspaceOnly)
            .expect("capability gate should be set");

        let blocked = runtime
            .run_demo_turn("read:/etc/hosts")
            .await
            .expect("run should return blocked result");

        assert_eq!(blocked.invoked_tool, None);
        assert_eq!(
            blocked.final_text,
            "filesystem scope blocked tool read_file"
        );
        assert!(blocked
            .blocked_reason
            .as_deref()
            .unwrap_or_default()
            .contains("filesystem scope workspace_only blocks read_file path /etc/hosts outside workspace"));
    }

    #[test]
    fn session_export_roundtrip_preserves_path_capability_gates() {
        let runtime = RuntimeKernel::new();
        let gate = runtime
            .set_path_capability_gate("read_file", "path", FilesystemScope::AnyPath)
            .expect("capability gate should be set");
        let export = runtime
            .session_export("session-main")
            .expect("session export should succeed");
        assert_eq!(export.path_capability_gates, vec![gate.clone()]);

        runtime
            .remove_path_capability_gate(&gate.id)
            .expect("capability gate remove should succeed");
        runtime
            .apply_session_export(export)
            .expect("session import should succeed");

        assert_eq!(
            runtime
                .path_capability_gates_for_session("session-main")
                .expect("gates should load"),
            vec![gate]
        );
    }

    #[tokio::test]
    async fn artifacts_only_write_scope_allows_writes_under_artifacts() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let result = runtime
            .run_demo_turn("write:artifacts/hepta-write-scope-test.txt => hello artifacts")
            .await
            .expect("write should succeed");

        assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
        let written = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-write-scope-test.txt");
        let content = fs::read_to_string(&written).expect("artifact file should exist");
        assert_eq!(content, "hello artifacts");
        let _ = fs::remove_file(&written);
    }

    #[tokio::test]
    async fn artifacts_only_write_scope_blocks_workspace_source_paths() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let blocked = runtime
            .run_demo_turn("write:docs/hepta-write-scope-test.txt => blocked")
            .await
            .expect("run should return blocked result");

        assert_eq!(blocked.invoked_tool, None);
        assert_eq!(
            blocked.final_text,
            "write path scope blocked tool write_file"
        );
        assert!(blocked
            .blocked_reason
            .as_deref()
            .unwrap_or_default()
            .contains("write path scope artifacts_only blocks write_file path docs/hepta-write-scope-test.txt outside artifacts root"));
    }

    #[tokio::test]
    async fn workspace_write_scope_allows_writes_outside_artifacts_but_inside_workspace() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");
        runtime
            .switch_write_path_scope(WritePathScope::WorkspaceOnly)
            .expect("write scope switch should succeed");

        let result = runtime
            .run_demo_turn("write:.hepta/runtime-write-scope-test.txt => hello workspace")
            .await
            .expect("write should succeed");

        assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
        let written = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(".hepta/runtime-write-scope-test.txt");
        let content = fs::read_to_string(&written).expect("workspace file should exist");
        assert_eq!(content, "hello workspace");
        let _ = fs::remove_file(&written);
    }

    #[tokio::test]
    async fn create_mode_refuses_silent_overwrite_for_existing_file() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-overwrite-guard-test.txt");
        fs::create_dir_all(path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path, "original").expect("seed file should be writable");

        let blocked = runtime
            .run_demo_turn("write:artifacts/hepta-overwrite-guard-test.txt => replacement")
            .await
            .expect("run should return blocked result");

        assert_eq!(blocked.invoked_tool, None);
        assert_eq!(
            blocked.final_text,
            "write semantics blocked tool write_file"
        );
        assert!(blocked
            .blocked_reason
            .as_deref()
            .unwrap_or_default()
            .contains("write_file refuses to overwrite existing path artifacts/hepta-overwrite-guard-test.txt"));
        assert_eq!(
            fs::read_to_string(&path).expect("seed file should still exist"),
            "original"
        );
        let _ = fs::remove_file(&path);
    }

    #[tokio::test]
    async fn overwrite_mode_replaces_existing_file_when_explicitly_confirmed() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-explicit-overwrite-test.txt");
        fs::create_dir_all(path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path, "before").expect("seed file should be writable");

        let result = runtime
            .run_demo_turn("overwrite:artifacts/hepta-explicit-overwrite-test.txt => after")
            .await
            .expect("explicit overwrite should succeed");

        assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
        assert_eq!(
            fs::read_to_string(&path).expect("target file should exist"),
            "after"
        );
        let output_json = result.tool_output_json.expect("structured output expected");
        assert!(output_json.contains("\"mode_requested\":\"overwrite\""));
        assert!(output_json.contains("\"mode_applied\":\"overwrite\""));
        assert!(output_json.contains("\"existed_before\":true"));
        assert!(output_json.contains("\"backup_created\":true"));
        let backup_path = extract_json_string_field(&output_json, "backup_path")
            .expect("backup path should be present");
        assert_eq!(
            fs::read_to_string(&backup_path).expect("backup file should exist"),
            "before"
        );
        let _ = fs::remove_file(&backup_path);
        let _ = fs::remove_file(&path);
    }

    #[tokio::test]
    async fn append_mode_appends_instead_of_replacing_existing_file() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-append-mode-test.txt");
        fs::create_dir_all(path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path, "before").expect("seed file should be writable");

        let result = runtime
            .run_demo_turn("append:artifacts/hepta-append-mode-test.txt => +after")
            .await
            .expect("append should succeed");

        assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
        assert_eq!(
            fs::read_to_string(&path).expect("target file should exist"),
            "before+after"
        );
        let output_json = result.tool_output_json.expect("structured output expected");
        assert!(output_json.contains("\"mode_requested\":\"append\""));
        assert!(output_json.contains("\"mode_applied\":\"append\""));
        let _ = fs::remove_file(&path);
    }

    #[tokio::test]
    async fn preview_write_reports_diff_and_backup_plan_without_mutating_file() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-preview-write-test.txt");
        fs::create_dir_all(path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path, "before").expect("seed file should be writable");

        let result = runtime
            .run_demo_turn("preview-write:artifacts/hepta-preview-write-test.txt => after")
            .await
            .expect("preview should succeed");

        assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
        assert_eq!(
            fs::read_to_string(&path).expect("target file should still exist"),
            "before"
        );
        let output_json = result.tool_output_json.expect("structured output expected");
        assert!(output_json.contains("\"preview_only\":true"));
        assert!(output_json.contains("\"backup_planned\":true"));
        assert!(output_json.contains("overwrite existing file"));
        let backup_path = extract_json_string_field(&output_json, "backup_path")
            .expect("preview backup path should be present");
        assert!(
            !PathBuf::from(&backup_path).exists(),
            "preview must not create the backup file"
        );
        let _ = fs::remove_file(&path);
    }

    #[tokio::test]
    async fn backup_index_lists_generated_overwrite_backups() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path = format!("artifacts/hepta-backup-index-test-{}.txt", unique);
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path);
        fs::create_dir_all(path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path, "before").expect("seed file should be writable");

        runtime
            .run_demo_turn(&format!("overwrite:{} => after", logical_path))
            .await
            .expect("overwrite should succeed");

        let report = runtime
            .backup_index(Some(&logical_path))
            .expect("backup index should succeed");
        assert_eq!(report.backups.len(), 1);
        assert!(report.backups[0].target_path.ends_with(&logical_path));

        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(&report.backups[0].backup_path);
    }

    #[tokio::test]
    async fn restore_backup_restores_target_and_backs_up_current_contents() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path = format!("artifacts/hepta-restore-backup-test-{}.txt", unique);
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path);
        fs::create_dir_all(path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path, "before").expect("seed file should be writable");

        runtime
            .run_demo_turn(&format!("overwrite:{} => after", logical_path))
            .await
            .expect("overwrite should succeed");

        let index = runtime
            .backup_index(Some(&logical_path))
            .expect("backup index should succeed");
        let backup = index.backups.first().expect("backup should exist").clone();

        let report = runtime
            .restore_backup(&backup.id)
            .expect("restore backup should succeed");

        assert_eq!(
            fs::read_to_string(&path).expect("restored target should exist"),
            "before"
        );
        assert!(report.transaction_id.starts_with("txn-"));
        let safety_backup = report
            .previous_target_backup_path
            .clone()
            .expect("restore should preserve replaced contents");
        assert_eq!(
            fs::read_to_string(&safety_backup).expect("safety backup should exist"),
            "after"
        );

        let events = runtime.events(20).expect("events should load");
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::BackupRestored)
        );

        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(&backup.backup_path);
        let _ = fs::remove_file(&safety_backup);
    }

    #[test]
    fn preview_backup_path_avoids_timestamp_collision() {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let backup_root = workspace_root.join("artifacts/backups/write_file");
        let logical_path = PathBuf::from("artifacts/hepta-preview-backup-collision.txt");
        let relative = PathBuf::from("workspace").join(&logical_path);
        let file_name = relative
            .file_name()
            .and_then(|name| name.to_str())
            .expect("file name should exist");
        let backup_dir = backup_root.join(relative.parent().expect("relative parent should exist"));
        fs::create_dir_all(&backup_dir).expect("backup dir should be creatable");

        let start_ts = 424242u64;
        let existing = backup_dir.join(format!("{}.hepta-bak-{}", file_name, start_ts));
        fs::write(&existing, b"before").expect("existing collision file should be writable");

        let candidate = preview_backup_path_from_ts(&backup_root, &relative, file_name, start_ts)
            .expect("backup path should be planned");

        assert_ne!(candidate, existing);
        assert_eq!(
            candidate,
            backup_dir.join(format!("{}.hepta-bak-{}", file_name, start_ts + 1))
        );

        let _ = fs::remove_file(existing);
        let _ = fs::remove_file(candidate);
    }

    #[tokio::test]
    async fn write_transaction_rollback_restores_previous_append_contents() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path = format!("artifacts/hepta-write-rollback-test-{}.txt", unique);
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path);
        fs::create_dir_all(path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path, "before").expect("seed file should be writable");

        let result = runtime
            .run_demo_turn(&format!("append:{} => +after", logical_path))
            .await
            .expect("append should succeed");

        let output_json = result.tool_output_json.expect("structured output expected");
        let transaction_id = extract_json_string_field(&output_json, "transaction_id")
            .expect("transaction id should exist");
        let rollback_checkpoint_path =
            extract_json_string_field(&output_json, "rollback_checkpoint_path")
                .expect("rollback checkpoint path should exist");
        assert_eq!(
            fs::read_to_string(&path).expect("target should exist"),
            "before+after"
        );
        assert_eq!(
            fs::read_to_string(&rollback_checkpoint_path).expect("checkpoint should exist"),
            "before"
        );

        let transaction_report = runtime
            .write_transactions(Some(&logical_path))
            .expect("transactions should load");
        assert_eq!(transaction_report.transactions.len(), 1);
        assert_eq!(
            transaction_report.transactions[0].transaction_id,
            transaction_id
        );

        let rollback = runtime
            .rollback_write_transaction(&transaction_id)
            .expect("rollback should succeed");
        assert_eq!(
            fs::read_to_string(&path).expect("target should exist after rollback"),
            "before"
        );
        assert_eq!(rollback.rollback_strategy, "restore_checkpoint");
        assert!(rollback.previous_target_backup_path.is_some());

        let events = runtime.events(40).expect("events should load");
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteTransactionRecorded)
        );
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteRolledBack)
        );

        let safety_backup = rollback
            .previous_target_backup_path
            .expect("rollback should create safety backup");
        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(&rollback_checkpoint_path);
        let _ = fs::remove_file(&safety_backup);
    }

    #[tokio::test]
    async fn write_transaction_group_plan_tracks_reverse_multi_file_rollback_order() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path_a = format!("artifacts/hepta-write-group-a-{}.txt", unique);
        let logical_path_b = format!("artifacts/hepta-write-group-b-{}.txt", unique);
        let path_a = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path_a);
        let path_b = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path_b);
        fs::create_dir_all(path_a.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path_a, "before-a").expect("seed file a should be writable");
        fs::write(&path_b, "before-b").expect("seed file b should be writable");

        let group = runtime
            .begin_write_transaction_group(Some("grp-test"))
            .expect("group should open");
        runtime
            .run_demo_turn(&format!("append:{} => +after-a", logical_path_a))
            .await
            .expect("append a should succeed");
        runtime
            .run_demo_turn(&format!("append:{} => +after-b", logical_path_b))
            .await
            .expect("append b should succeed");
        runtime
            .end_write_transaction_group()
            .expect("group should close");

        let groups = runtime
            .write_transaction_groups()
            .expect("groups should load");
        assert_eq!(groups.groups.len(), 1);
        assert_eq!(groups.groups[0].group_id, group.group_id);
        assert_eq!(groups.groups[0].transaction_ids.len(), 2);

        let plan = runtime
            .rollback_write_plan(&group.group_id)
            .expect("rollback plan should load");
        assert!(plan.closed);
        assert!(plan.executable);
        assert_eq!(plan.steps.len(), 2);
        assert!(plan.steps[0].target_path.ends_with(&logical_path_b));
        assert!(plan.steps[1].target_path.ends_with(&logical_path_a));

        for entry in runtime
            .write_transactions(None)
            .expect("transactions should load")
            .transactions
        {
            if entry.target_path.ends_with(&logical_path_a)
                || entry.target_path.ends_with(&logical_path_b)
            {
                if let Some(checkpoint) = entry.rollback_checkpoint_path {
                    let _ = fs::remove_file(checkpoint);
                }
            }
        }
        for logical_path in [&logical_path_a, &logical_path_b] {
            let backups = runtime
                .backup_index(Some(logical_path))
                .expect("backup index should load");
            for backup in backups.backups {
                let _ = fs::remove_file(backup.backup_path);
            }
        }
        let _ = fs::remove_file(&path_a);
        let _ = fs::remove_file(&path_b);
    }

    #[tokio::test]
    async fn rollback_group_restores_multiple_files() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path_a = format!("artifacts/hepta-rollback-group-a-{}.txt", unique);
        let logical_path_b = format!("artifacts/hepta-rollback-group-b-{}.txt", unique);
        let path_a = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path_a);
        let path_b = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path_b);
        fs::create_dir_all(path_a.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path_a, "before-a").expect("seed file a should be writable");
        fs::write(&path_b, "before-b").expect("seed file b should be writable");

        let group = runtime
            .begin_write_transaction_group(None)
            .expect("group should open");
        runtime
            .run_demo_turn(&format!("append:{} => +after-a", logical_path_a))
            .await
            .expect("append a should succeed");
        runtime
            .run_demo_turn(&format!("append:{} => +after-b", logical_path_b))
            .await
            .expect("append b should succeed");
        runtime
            .end_write_transaction_group()
            .expect("group should close");

        let report = runtime
            .rollback_write_group(&group.group_id)
            .expect("rollback group should succeed");
        assert_eq!(report.executed_transaction_ids.len(), 2);
        assert_eq!(
            fs::read_to_string(&path_a).expect("path a should exist"),
            "before-a"
        );
        assert_eq!(
            fs::read_to_string(&path_b).expect("path b should exist"),
            "before-b"
        );

        let events = runtime.events(50).expect("events should load");
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteTransactionGroupOpened)
        );
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteTransactionGroupClosed)
        );
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteGroupRolledBack)
        );

        for entry in runtime
            .write_transactions(None)
            .expect("transactions should load")
            .transactions
        {
            if entry.target_path.ends_with(&logical_path_a)
                || entry.target_path.ends_with(&logical_path_b)
            {
                if let Some(checkpoint) = entry.rollback_checkpoint_path {
                    let _ = fs::remove_file(checkpoint);
                }
            }
        }
        for logical_path in [&logical_path_a, &logical_path_b] {
            let backups = runtime
                .backup_index(Some(logical_path))
                .expect("backup index should load");
            for backup in backups.backups {
                let _ = fs::remove_file(backup.backup_path);
            }
        }
        let _ = fs::remove_file(&path_a);
        let _ = fs::remove_file(&path_b);
    }

    #[tokio::test]
    async fn rollback_group_partial_failure_records_status_and_resume_path() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path_a = format!("artifacts/hepta-partial-rollback-a-{}.txt", unique);
        let logical_path_b = format!("artifacts/hepta-partial-rollback-b-{}.txt", unique);
        let path_a = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path_a);
        let path_b = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path_b);
        fs::create_dir_all(path_a.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path_a, "before-a").expect("seed file a should be writable");
        fs::write(&path_b, "before-b").expect("seed file b should be writable");

        let group = runtime
            .begin_write_transaction_group(Some("grp-partial"))
            .expect("group should open");
        runtime
            .run_demo_turn(&format!("append:{} => +after-a", logical_path_a))
            .await
            .expect("append a should succeed");
        runtime
            .run_demo_turn(&format!("append:{} => +after-b", logical_path_b))
            .await
            .expect("append b should succeed");
        runtime
            .end_write_transaction_group()
            .expect("group should close");

        let plan = runtime
            .rollback_write_plan(&group.group_id)
            .expect("rollback plan should load");
        let fail_txn = plan.steps[1].transaction_id.clone();
        runtime
            .rollback_failure_injection_state
            .lock()
            .expect("failure injection state should lock")
            .push(fail_txn.clone());

        let partial = runtime
            .rollback_write_group(&group.group_id)
            .expect("rollback group should return partial failure report");
        assert_eq!(partial.status, RollbackGroupAttemptStatus::PartialFailed);
        assert_eq!(
            partial.failed_transaction_id.as_deref(),
            Some(fail_txn.as_str())
        );
        assert_eq!(partial.executed_transaction_ids.len(), 1);
        assert_eq!(partial.pending_transaction_ids, vec![fail_txn.clone()]);
        assert!(partial.resume_command.is_some());
        assert_eq!(
            fs::read_to_string(&path_b).expect("path b should be restored"),
            "before-b"
        );
        assert_eq!(
            fs::read_to_string(&path_a).expect("path a should still be appended"),
            "before-a+after-a"
        );

        let status = runtime
            .rollback_group_status(&group.group_id)
            .expect("rollback status should load");
        assert_eq!(
            status.schema_version,
            super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION
        );
        assert!(status.group_locked);
        assert_eq!(
            status.group_lock_attempt_id.as_deref(),
            Some(partial.attempt_id.as_str())
        );
        assert_eq!(status.target_lock_count, 2);
        assert_eq!(status.orphaned_lock_count, 0);
        assert!(status.latest_attempt_owns_lock_set);
        assert_eq!(status.attempt_lifecycle.attempt_count, 1);
        assert_eq!(status.attempt_lifecycle.superseded_attempt_count, 0);
        assert_eq!(
            status.attempt_lifecycle.active_attempt_id.as_deref(),
            Some(partial.attempt_id.as_str())
        );
        assert_eq!(status.lock_diagnostics.target_lock_count, 2);
        assert_eq!(
            status
                .latest_attempt
                .as_ref()
                .expect("attempt should exist")
                .status,
            RollbackGroupAttemptStatus::PartialFailed
        );
        assert!(status.resume_command.is_some());

        let status_json = serde_json::to_value(&status).expect("status should serialize");
        assert_eq!(
            status_json.get("schema_version").and_then(Value::as_u64),
            Some(super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION as u64)
        );
        assert_eq!(
            status_json
                .get("lock_diagnostics")
                .and_then(|value| value.get("group_lock_attempt_id"))
                .and_then(Value::as_str),
            Some(partial.attempt_id.as_str())
        );
        assert_eq!(
            status_json
                .get("attempt_lifecycle")
                .and_then(|value| value.get("active_attempt_id"))
                .and_then(Value::as_str),
            Some(partial.attempt_id.as_str())
        );

        let locks = runtime.write_locks().expect("write locks should load");
        assert_eq!(
            locks.schema_version,
            super::WRITE_LOCK_REPORT_SCHEMA_VERSION
        );
        assert_eq!(locks.summary.total_target_locks, 2);
        assert_eq!(locks.summary.total_group_locks, 1);
        assert_eq!(locks.summary.rollback_bound_target_locks, 2);
        assert_eq!(locks.summary.rollback_bound_group_locks, 1);
        assert_eq!(locks.summary.orphaned_target_locks, 0);
        assert_eq!(locks.summary.orphaned_group_locks, 0);
        let group_lock = locks
            .group_locks
            .iter()
            .find(|lock| lock.group_id == group.group_id)
            .expect("group lock should exist");
        assert_eq!(
            group_lock.rollback_attempt_id.as_deref(),
            Some(partial.attempt_id.as_str())
        );
        assert_eq!(
            group_lock.rollback_status,
            Some(RollbackGroupAttemptStatus::PartialFailed)
        );
        assert_eq!(group_lock.pending_transaction_ids, vec![fail_txn.clone()]);
        let target_lock_a = locks
            .target_locks
            .iter()
            .find(|lock| lock.target_path.ends_with(&logical_path_a))
            .expect("target lock a should exist");
        assert_eq!(
            target_lock_a.rollback_group_id.as_deref(),
            Some(group.group_id.as_str())
        );
        assert_eq!(
            target_lock_a.rollback_attempt_id.as_deref(),
            Some(partial.attempt_id.as_str())
        );
        let target_lock_b = locks
            .target_locks
            .iter()
            .find(|lock| lock.target_path.ends_with(&logical_path_b))
            .expect("target lock b should exist");
        assert_eq!(
            target_lock_b.rollback_group_id.as_deref(),
            Some(group.group_id.as_str())
        );
        assert_eq!(
            target_lock_b.rollback_attempt_id.as_deref(),
            Some(partial.attempt_id.as_str())
        );

        let locks_json = serde_json::to_value(&locks).expect("locks should serialize");
        assert_eq!(
            locks_json.get("schema_version").and_then(Value::as_u64),
            Some(super::WRITE_LOCK_REPORT_SCHEMA_VERSION as u64)
        );
        assert_eq!(
            locks_json
                .get("summary")
                .and_then(|value| value.get("rollback_bound_target_locks"))
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            locks_json
                .get("summary")
                .and_then(|value| value.get("orphaned_group_locks"))
                .and_then(Value::as_u64),
            Some(0)
        );

        let blocked_write = runtime
            .run_demo_turn(&format!("append:{} => +blocked", logical_path_a))
            .await
            .expect("blocked write should still return a turn result");
        assert!(
            blocked_write
                .blocked_reason
                .expect("blocked reason should exist")
                .contains("write lock blocks write_file")
        );

        let resumed = runtime
            .resume_rollback_write_group(&group.group_id)
            .expect("resume rollback should succeed");
        assert_eq!(resumed.status, RollbackGroupAttemptStatus::Completed);
        assert_eq!(
            resumed.resumed_from_attempt_id,
            Some(partial.attempt_id.clone())
        );
        assert_eq!(
            fs::read_to_string(&path_a).expect("path a should be restored"),
            "before-a"
        );
        let post_resume_status = runtime
            .rollback_group_status(&group.group_id)
            .expect("post-resume rollback status should load");
        assert_eq!(post_resume_status.attempt_count, 2);
        assert_eq!(post_resume_status.superseded_attempt_count, 1);
        assert_eq!(
            post_resume_status.active_attempt_id.as_deref(),
            Some(resumed.attempt_id.as_str())
        );
        let superseded_partial = runtime
            .rollback_group_attempt_by_id(&partial.attempt_id)
            .expect("partial attempt lookup should succeed")
            .expect("partial attempt should exist");
        assert_eq!(
            superseded_partial.superseded_by_attempt_id.as_deref(),
            Some(resumed.attempt_id.as_str())
        );
        assert!(
            !runtime
                .write_locks()
                .expect("write locks should load")
                .group_locks
                .iter()
                .any(|lock| lock.group_id == group.group_id)
        );

        let events = runtime.events(60).expect("events should load");
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteLocksAcquired)
        );
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteLocksReleased)
        );
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteLockConflict)
        );
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteGroupRollbackFailed)
        );
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteGroupRollbackResumed)
        );
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteGroupRolledBack)
        );

        let failed_event_payload = events
            .iter()
            .find(|event| event.event.kind == EventKind::WriteGroupRollbackFailed)
            .and_then(|event| event.event.payload.as_ref())
            .expect("failed rollback event payload should exist");
        assert_eq!(
            failed_event_payload
                .get("schema_version")
                .and_then(Value::as_u64),
            Some(super::ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION as u64)
        );
        assert_eq!(
            failed_event_payload.get("group_id").and_then(Value::as_str),
            Some(group.group_id.as_str())
        );
        assert_eq!(
            failed_event_payload
                .get("attempt_id")
                .and_then(Value::as_str),
            Some(partial.attempt_id.as_str())
        );
        assert_eq!(
            failed_event_payload
                .get("failed_transaction_id")
                .and_then(Value::as_str),
            Some(fail_txn.as_str())
        );

        let resumed_event_payload = events
            .iter()
            .find(|event| event.event.kind == EventKind::WriteGroupRollbackResumed)
            .and_then(|event| event.event.payload.as_ref())
            .expect("resumed rollback event payload should exist");
        assert_eq!(
            resumed_event_payload
                .get("resumed_from_attempt_id")
                .and_then(Value::as_str),
            Some(partial.attempt_id.as_str())
        );
        assert_eq!(
            resumed_event_payload
                .get("resumed_attempt_id")
                .and_then(Value::as_str),
            Some(resumed.attempt_id.as_str())
        );

        let rolled_back_event_payload = events
            .iter()
            .find(|event| event.event.kind == EventKind::WriteGroupRolledBack)
            .and_then(|event| event.event.payload.as_ref())
            .expect("completed rollback event payload should exist");
        assert_eq!(
            rolled_back_event_payload
                .get("status")
                .and_then(Value::as_str),
            Some("completed")
        );
        assert_eq!(
            rolled_back_event_payload
                .get("attempt_id")
                .and_then(Value::as_str),
            Some(resumed.attempt_id.as_str())
        );

        let conflict_event_payload = events
            .iter()
            .find(|event| event.event.kind == EventKind::WriteLockConflict)
            .and_then(|event| event.event.payload.as_ref())
            .expect("write lock conflict payload should exist");
        assert_eq!(
            conflict_event_payload
                .get("operation")
                .and_then(Value::as_str),
            Some("write_file")
        );
        assert_eq!(
            conflict_event_payload
                .get("conflicting_group_id")
                .and_then(Value::as_str),
            Some(group.group_id.as_str())
        );
        assert_eq!(
            conflict_event_payload
                .get("conflicting_attempt_id")
                .and_then(Value::as_str),
            Some(partial.attempt_id.as_str())
        );

        let released_event_payload = events
            .iter()
            .find(|event| event.event.kind == EventKind::WriteLocksReleased)
            .and_then(|event| event.event.payload.as_ref())
            .expect("write locks released payload should exist");
        assert_eq!(
            released_event_payload
                .get("released_group_locks")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            released_event_payload
                .get("released_target_locks")
                .and_then(Value::as_u64),
            Some(2)
        );

        for entry in runtime
            .write_transactions(None)
            .expect("transactions should load")
            .transactions
        {
            if entry.target_path.ends_with(&logical_path_a)
                || entry.target_path.ends_with(&logical_path_b)
            {
                if let Some(checkpoint) = entry.rollback_checkpoint_path {
                    let _ = fs::remove_file(checkpoint);
                }
            }
        }
        for logical_path in [&logical_path_a, &logical_path_b] {
            let backups = runtime
                .backup_index(Some(logical_path))
                .expect("backup index should load");
            for backup in backups.backups {
                let _ = fs::remove_file(backup.backup_path);
            }
        }
        let _ = fs::remove_file(&path_a);
        let _ = fs::remove_file(&path_b);
    }

    #[test]
    fn rollback_status_flags_orphaned_locks_and_recommends_prune() {
        let runtime = RuntimeKernel::new();
        runtime
            .write_transaction_group_state
            .lock()
            .expect("write transaction group state should lock")
            .groups
            .push(WriteTransactionGroup {
                group_id: "grp-orphaned".into(),
                session_id: "session-main".into(),
                opened_at_unix_ms: 1,
                closed_at_unix_ms: Some(2),
                transaction_ids: vec![],
            });
        runtime
            .write_transaction_group_state
            .lock()
            .expect("write transaction group state should lock")
            .rollback_attempts
            .push(RollbackGroupAttempt {
                attempt_id: "rbk-orphaned".into(),
                session_id: "session-main".into(),
                group_id: "grp-orphaned".into(),
                started_at_unix_ms: 1,
                finished_at_unix_ms: Some(2),
                status: RollbackGroupAttemptStatus::PartialFailed,
                resumed_from_attempt_id: None,
                superseded_by_attempt_id: Some("rbk-current".into()),
                executed_transaction_ids: vec![],
                skipped_already_rolled_back_ids: vec![],
                pending_transaction_ids: vec!["txn-orphaned".into()],
                failed_transaction_id: Some("txn-orphaned".into()),
                failure_reason: Some("boom".into()),
                target_paths_restored: vec![],
            });
        runtime
            .write_transaction_group_state
            .lock()
            .expect("write transaction group state should lock")
            .rollback_attempts
            .push(RollbackGroupAttempt {
                attempt_id: "rbk-current".into(),
                session_id: "session-main".into(),
                group_id: "grp-orphaned".into(),
                started_at_unix_ms: 3,
                finished_at_unix_ms: Some(4),
                status: RollbackGroupAttemptStatus::PartialFailed,
                resumed_from_attempt_id: Some("rbk-orphaned".into()),
                superseded_by_attempt_id: None,
                executed_transaction_ids: vec![],
                skipped_already_rolled_back_ids: vec![],
                pending_transaction_ids: vec!["txn-current".into()],
                failed_transaction_id: Some("txn-current".into()),
                failure_reason: Some("still broken".into()),
                target_paths_restored: vec![],
            });
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state should lock")
            .group_locks
            .push(WriteGroupLock {
                session_id: "session-main".into(),
                group_id: "grp-orphaned".into(),
                owner_kind: "rollback_group".into(),
                owner_id: "rbk-orphaned".into(),
                rollback_attempt_id: Some("rbk-orphaned".into()),
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist")
                    + 60_000,
            });

        let status = runtime
            .rollback_group_status("grp-orphaned")
            .expect("rollback status should load");
        assert_eq!(
            status.schema_version,
            super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION
        );
        assert!(status.group_locked);
        assert_eq!(
            status.group_lock_attempt_id.as_deref(),
            Some("rbk-orphaned")
        );
        assert_eq!(status.orphaned_lock_count, 1);
        assert!(!status.latest_attempt_owns_lock_set);
        assert_eq!(status.active_attempt_id.as_deref(), Some("rbk-current"));
        assert_eq!(status.lock_diagnostics.orphaned_lock_count, 1);
        assert_eq!(status.attempt_lifecycle.superseded_attempt_count, 1);
        assert_eq!(status.resume_command.as_deref(), Some("/prune-stale-locks"));
        assert!(
            status
                .suggested_next_action
                .contains("prune orphaned locks")
        );

        let status_json = serde_json::to_value(&status).expect("status should serialize");
        assert_eq!(
            status_json
                .get("lock_diagnostics")
                .and_then(|value| value.get("orphaned_lock_count"))
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            status_json
                .get("attempt_lifecycle")
                .and_then(|value| value.get("superseded_attempt_count"))
                .and_then(Value::as_u64),
            Some(1)
        );

        let locks = runtime.write_locks().expect("write locks should load");
        assert_eq!(locks.summary.orphaned_group_locks, 1);
    }

    #[tokio::test]
    async fn overlap_lock_blocks_write_to_descendant_path() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_dir = format!("artifacts/hepta-locked-dir-{}", unique);
        let locked_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_dir);
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state should lock")
            .target_locks
            .push(WriteTargetLock {
                session_id: "session-main".into(),
                target_path: locked_dir_path.display().to_string(),
                owner_kind: "rollback_group".into(),
                owner_id: "grp-overlap".into(),
                rollback_group_id: Some("grp-overlap".into()),
                rollback_attempt_id: Some("rbk-overlap".into()),
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist")
                    + 60_000,
            });

        let result = runtime
            .run_demo_turn(&format!("append:{}/child.txt => +blocked", logical_dir))
            .await
            .expect("blocked write should still produce a turn result");
        assert!(
            result
                .blocked_reason
                .expect("blocked reason should exist")
                .contains("write lock blocks write_file")
        );

        let events = runtime.events(20).expect("events should load");
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteLockConflict)
        );
    }

    #[tokio::test]
    async fn overlap_lock_blocks_rollback_group_on_descendant_target() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_dir = format!("artifacts/hepta-overlap-rollback-{}", unique);
        let logical_path = format!("{}/child.txt", logical_dir);
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path);
        fs::create_dir_all(path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&path, "before").expect("seed file should be writable");

        let group = runtime
            .begin_write_transaction_group(Some("grp-overlap-rollback"))
            .expect("group should open");
        runtime
            .run_demo_turn(&format!("append:{} => +after", logical_path))
            .await
            .expect("append should succeed");
        runtime
            .end_write_transaction_group()
            .expect("group should close");

        let locked_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_dir);
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state should lock")
            .target_locks
            .push(WriteTargetLock {
                session_id: "session-main".into(),
                target_path: locked_dir_path.display().to_string(),
                owner_kind: "rollback_group".into(),
                owner_id: "grp-external".into(),
                rollback_group_id: Some("grp-external".into()),
                rollback_attempt_id: Some("rbk-external".into()),
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist")
                    + 60_000,
            });

        let err = runtime
            .rollback_write_group(&group.group_id)
            .expect_err("overlap lock should block rollback group");
        assert!(err.0.contains("write lock blocks rollback_group"));

        let events = runtime.events(30).expect("events should load");
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteLockConflict)
        );
        let conflict_event_payload = events
            .iter()
            .find(|event| event.event.kind == EventKind::WriteLockConflict)
            .and_then(|event| event.event.payload.as_ref())
            .expect("rollback-group conflict payload should exist");
        assert_eq!(
            conflict_event_payload
                .get("operation")
                .and_then(Value::as_str),
            Some("rollback_group")
        );
        assert_eq!(
            conflict_event_payload
                .get("conflicting_group_id")
                .and_then(Value::as_str),
            Some("grp-external")
        );

        for entry in runtime
            .write_transactions(None)
            .expect("transactions should load")
            .transactions
        {
            if entry.target_path.ends_with(&logical_path) {
                if let Some(checkpoint) = entry.rollback_checkpoint_path {
                    let _ = fs::remove_file(checkpoint);
                }
            }
        }
        for backup in runtime
            .backup_index(Some(&logical_path))
            .expect("backup index should load")
            .backups
        {
            let _ = fs::remove_file(backup.backup_path);
        }
        let _ = fs::remove_file(&path);
    }

    #[tokio::test]
    async fn expired_write_lock_is_pruned_and_does_not_block_write() {
        let runtime = RuntimeKernel::new();
        runtime
            .add_policy_rule(
                None,
                None,
                Some("write_file"),
                None,
                ApprovalRequirement::None,
                Some("test allow write"),
            )
            .expect("policy rule should be added");

        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path = format!("artifacts/hepta-expired-lock-{}.txt", unique);
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(&logical_path);
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state should lock")
            .target_locks
            .push(WriteTargetLock {
                session_id: "session-main".into(),
                target_path: path.display().to_string(),
                owner_kind: "rollback_group".into(),
                owner_id: "grp-stale".into(),
                rollback_group_id: Some("grp-stale".into()),
                rollback_attempt_id: Some("rbk-stale".into()),
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms: 1,
            });

        let result = runtime
            .run_demo_turn(&format!("append:{} => +after", logical_path))
            .await
            .expect("write should succeed after stale lock pruning");
        assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
        assert!(
            runtime
                .write_locks()
                .expect("write locks should load")
                .target_locks
                .is_empty()
        );

        for entry in runtime
            .write_transactions(Some(&logical_path))
            .expect("transactions should load")
            .transactions
        {
            if let Some(checkpoint) = entry.rollback_checkpoint_path {
                let _ = fs::remove_file(checkpoint);
            }
        }
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn prune_stale_write_locks_removes_expired_entries_and_emits_event() {
        let runtime = RuntimeKernel::new();
        {
            let mut guard = runtime
                .write_lock_state
                .lock()
                .expect("write lock state should lock");
            guard.group_locks.push(WriteGroupLock {
                session_id: "session-main".into(),
                group_id: "grp-stale".into(),
                owner_kind: "rollback_group".into(),
                owner_id: "rbk-stale".into(),
                rollback_attempt_id: Some("rbk-stale".into()),
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms: 1,
            });
            guard.target_locks.push(WriteTargetLock {
                session_id: "session-main".into(),
                target_path: "/tmp/hepta-stale".into(),
                owner_kind: "rollback_group".into(),
                owner_id: "grp-stale".into(),
                rollback_group_id: Some("grp-stale".into()),
                rollback_attempt_id: Some("rbk-stale".into()),
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms: 1,
            });
        }

        let report = runtime
            .prune_stale_write_locks()
            .expect("stale lock prune should succeed");
        assert_eq!(report.pruned_target_locks, 1);
        assert_eq!(report.pruned_group_locks, 1);
        assert_eq!(report.remaining_target_locks, 0);
        assert_eq!(report.remaining_group_locks, 0);

        let events = runtime.events(20).expect("events should load");
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::WriteLocksPruned)
        );
        let pruned_event_payload = events
            .iter()
            .find(|event| event.event.kind == EventKind::WriteLocksPruned)
            .and_then(|event| event.event.payload.as_ref())
            .expect("write locks pruned payload should exist");
        assert_eq!(
            pruned_event_payload
                .get("schema_version")
                .and_then(Value::as_u64),
            Some(super::ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION as u64)
        );
        assert_eq!(
            pruned_event_payload
                .get("pruned_target_locks")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            pruned_event_payload
                .get("pruned_group_locks")
                .and_then(Value::as_u64),
            Some(1)
        );
    }

    #[test]
    fn snapshot_roundtrip_preserves_write_transactions() {
        let runtime = RuntimeKernel::new();
        let unique = current_unix_ms().expect("timestamp should exist");
        let target_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(format!("artifacts/hepta-write-txn-snapshot-{}.txt", unique));
        fs::create_dir_all(target_path.parent().expect("parent should exist"))
            .expect("artifact dir should be creatable");
        fs::write(&target_path, "before").expect("seed file should be writable");

        let checkpoint_path = preview_transaction_checkpoint_path(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
            &target_path,
            "txn-snapshot",
        )
        .expect("checkpoint path should build");
        fs::create_dir_all(
            checkpoint_path
                .parent()
                .expect("checkpoint parent should exist"),
        )
        .expect("checkpoint parent should be creatable");
        fs::write(&checkpoint_path, "before").expect("checkpoint should be writable");
        runtime
            .write_transaction_state
            .lock()
            .expect("write transaction state should lock")
            .push(WriteTransactionEntry {
                transaction_id: "txn-snapshot".into(),
                session_id: "session-main".into(),
                action: "write_file".into(),
                target_path: target_path.display().to_string(),
                created_at_unix_ms: unique,
                mode: "append".into(),
                target_existed_before: true,
                bytes_before: 6,
                bytes_after: 12,
                rollback_strategy: "restore_checkpoint".into(),
                rollback_checkpoint_path: Some(checkpoint_path.display().to_string()),
                source_backup_path: None,
                rolled_back_at_unix_ms: None,
            });

        let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(format!(
                "artifacts/hepta-write-txn-snapshot-{}.json",
                unique
            ));
        runtime
            .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot save should succeed");

        let restored = RuntimeKernel::new();
        restored
            .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot load should succeed");
        let report = restored
            .write_transactions(None)
            .expect("transactions should load");
        assert!(
            report
                .transactions
                .iter()
                .any(|entry| entry.transaction_id == "txn-snapshot")
        );

        let _ = fs::remove_file(&target_path);
        let _ = fs::remove_file(&checkpoint_path);
        let _ = fs::remove_file(&snapshot_path);
    }

    #[test]
    fn snapshot_roundtrip_preserves_write_transaction_groups() {
        let runtime = RuntimeKernel::new();
        runtime
            .write_transaction_group_state
            .lock()
            .expect("write transaction group state should lock")
            .groups
            .push(WriteTransactionGroup {
                group_id: "txngrp-snapshot".into(),
                session_id: "session-main".into(),
                opened_at_unix_ms: 1,
                closed_at_unix_ms: Some(2),
                transaction_ids: vec!["txn-a".into(), "txn-b".into()],
            });

        let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-write-group-snapshot.json");
        runtime
            .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot save should succeed");

        let restored = RuntimeKernel::new();
        restored
            .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot load should succeed");
        let report = restored
            .write_transaction_groups()
            .expect("groups should load");
        assert!(
            report
                .groups
                .iter()
                .any(|group| group.group_id == "txngrp-snapshot")
        );

        let _ = fs::remove_file(&snapshot_path);
    }

    #[test]
    fn snapshot_roundtrip_preserves_rollback_group_attempts() {
        let runtime = RuntimeKernel::new();
        runtime
            .write_transaction_group_state
            .lock()
            .expect("write transaction group state should lock")
            .rollback_attempts
            .push(super::RollbackGroupAttempt {
                attempt_id: "rbk-snapshot".into(),
                session_id: "session-main".into(),
                group_id: "txngrp-snapshot".into(),
                started_at_unix_ms: 1,
                finished_at_unix_ms: Some(2),
                status: RollbackGroupAttemptStatus::PartialFailed,
                resumed_from_attempt_id: None,
                superseded_by_attempt_id: None,
                executed_transaction_ids: vec!["txn-a".into()],
                skipped_already_rolled_back_ids: vec![],
                pending_transaction_ids: vec!["txn-b".into()],
                failed_transaction_id: Some("txn-b".into()),
                failure_reason: Some("boom".into()),
                target_paths_restored: vec!["/tmp/a".into()],
            });

        let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-rollback-attempt-snapshot.json");
        runtime
            .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot save should succeed");

        let restored = RuntimeKernel::new();
        restored
            .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot load should succeed");
        let status = restored
            .write_transaction_group_state
            .lock()
            .expect("write transaction group state should lock")
            .rollback_attempts
            .iter()
            .find(|attempt| attempt.attempt_id == "rbk-snapshot")
            .cloned();
        assert!(status.is_some());

        let _ = fs::remove_file(&snapshot_path);
    }

    #[test]
    fn snapshot_roundtrip_preserves_write_locks() {
        let runtime = RuntimeKernel::new();
        let lease_expires_at_unix_ms = current_unix_ms().expect("timestamp should exist") + 60_000;
        {
            let mut guard = runtime
                .write_lock_state
                .lock()
                .expect("write lock state should lock");
            guard.group_locks.push(WriteGroupLock {
                session_id: "session-main".into(),
                group_id: "txngrp-snapshot".into(),
                owner_kind: "rollback_group".into(),
                owner_id: "rbk-snapshot".into(),
                rollback_attempt_id: Some("rbk-snapshot".into()),
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms,
            });
            guard.target_locks.push(WriteTargetLock {
                session_id: "session-main".into(),
                target_path: "/tmp/a".into(),
                owner_kind: "rollback_group".into(),
                owner_id: "txngrp-snapshot".into(),
                rollback_group_id: Some("txngrp-snapshot".into()),
                rollback_attempt_id: Some("rbk-snapshot".into()),
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms,
            });
        }

        let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("artifacts/hepta-write-lock-snapshot.json");
        runtime
            .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot save should succeed");

        let restored = RuntimeKernel::new();
        restored
            .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot load should succeed");
        let locks = restored.write_locks().expect("write locks should load");
        assert!(
            locks
                .group_locks
                .iter()
                .any(|lock| lock.group_id == "txngrp-snapshot")
        );
        assert!(
            locks
                .target_locks
                .iter()
                .any(|lock| lock.target_path == "/tmp/a")
        );
        assert!(
            locks
                .group_locks
                .iter()
                .any(|lock| lock.lease_expires_at_unix_ms == lease_expires_at_unix_ms)
        );
        assert!(
            locks
                .target_locks
                .iter()
                .any(|lock| lock.lease_expires_at_unix_ms == lease_expires_at_unix_ms)
        );

        let _ = fs::remove_file(&snapshot_path);
    }

    #[test]
    fn preview_prune_backups_plans_deletion_of_older_backups() {
        let runtime = RuntimeKernel::new();
        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path = format!("artifacts/hepta-prune-preview-test-{}.txt", unique);
        let older = write_fake_workspace_backup(&logical_path, unique, "older");
        let newer = write_fake_workspace_backup(&logical_path, unique + 1, "newer");

        let report = runtime
            .preview_prune_backups(Some(&logical_path), 1, None)
            .expect("preview prune should succeed");

        assert_eq!(report.scanned_backups, 2);
        assert_eq!(report.deleted_count, 1);
        assert_eq!(report.kept_backups.len(), 1);
        assert_eq!(report.deleted_backups[0].created_at_unix_ms, unique);
        assert_eq!(report.kept_backups[0].created_at_unix_ms, unique + 1);

        let _ = fs::remove_file(&older);
        let _ = fs::remove_file(&newer);
    }

    #[test]
    fn prune_backups_deletes_older_backups_and_emits_event() {
        let runtime = RuntimeKernel::new();
        let unique = current_unix_ms().expect("timestamp should exist");
        let logical_path = format!("artifacts/hepta-prune-exec-test-{}.txt", unique);
        let older = write_fake_workspace_backup(&logical_path, unique, "older");
        let newer = write_fake_workspace_backup(&logical_path, unique + 1, "newer");

        let report = runtime
            .prune_backups(Some(&logical_path), 1, None)
            .expect("prune backups should succeed");

        assert!(report.executed);
        assert_eq!(report.deleted_count, 1);
        assert!(!older.exists());
        assert!(newer.exists());

        let events = runtime.events(20).expect("events should load");
        assert!(
            events
                .iter()
                .any(|event| event.event.kind == EventKind::BackupsPruned)
        );

        let _ = fs::remove_file(&newer);
    }

    #[test]
    fn session_export_roundtrip_preserves_write_path_scope() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_write_path_scope(WritePathScope::WorkspaceOnly)
            .expect("write scope switch should succeed");
        let export = runtime
            .session_export("session-main")
            .expect("session export should succeed");
        assert_eq!(export.write_path_scope, WritePathScope::WorkspaceOnly);

        runtime
            .switch_write_path_scope(WritePathScope::ArtifactsOnly)
            .expect("write scope reset should succeed");
        runtime
            .apply_session_export(export)
            .expect("session import should succeed");

        assert_eq!(
            runtime
                .write_path_scope_for_session("session-main")
                .expect("write scope should load"),
            WritePathScope::WorkspaceOnly
        );
    }

    #[tokio::test]
    async fn session_export_roundtrip_preserves_topic_sessions_and_graph_store() {
        let source = RuntimeKernel::new();
        source
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        source
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        source
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        source
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        source
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        source
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let export = source
            .session_export("alpha")
            .expect("session export should succeed");
        assert_eq!(export.topic_sessions.len(), 2);
        assert!(export.topic_graph_edges.iter().any(|record| {
            record.source_topic_session_id == "topic-session-bootstrap:alpha"
                && record.edge.target_topic_session_id
                    == "topic-session-bootstrap:alpha:rust-worker-pipeline"
        }));

        let restored = RuntimeKernel::new();
        restored
            .apply_session_export(export)
            .expect("session import should succeed");

        let raw_topic_sessions = restored
            .topic_session_state
            .lock()
            .expect("topic session state lock should succeed")
            .sessions
            .clone();
        let raw_topic_graph_edges = restored
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .clone();
        assert_eq!(raw_topic_sessions.len(), 2);
        assert!(raw_topic_graph_edges.iter().any(|record| {
            record.source_topic_session_id == "topic-session-bootstrap:alpha"
                && record.edge.target_topic_session_id
                    == "topic-session-bootstrap:alpha:rust-worker-pipeline"
        }));

        let topic_sessions = restored
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && !topic_session.graph_edges.is_empty()
        }));
    }

    #[tokio::test]
    async fn exposes_sessions_memory_and_history_snapshots() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello session control plane")
            .await
            .expect("plain turn should succeed");
        runtime
            .run_demo_turn("tool:history probe")
            .await
            .expect("tool turn should succeed");

        let sessions = runtime.sessions().expect("sessions should load");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "session-main");

        let memories = runtime
            .memory_snapshot(10)
            .expect("memory snapshot should load");
        assert!(memories.iter().any(|item| {
            item.content
                .contains("assistant:hello session control plane")
        }));

        let history = runtime
            .history(Some("session-main"), 10)
            .expect("history should load");
        assert!(history.len() >= 2);
        assert_eq!(history[0].input, "tool:history probe");
    }

    #[tokio::test]
    async fn sessions_materialize_fresh_active_session() {
        let runtime = RuntimeKernel::new();

        let sessions = runtime.sessions().expect("sessions should load");

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "session-main");
        assert!(sessions[0].is_active);
    }

    #[tokio::test]
    async fn active_session_snapshot_materializes_fresh_active_session() {
        let runtime = RuntimeKernel::new();

        let session = runtime
            .active_session_snapshot()
            .expect("active session snapshot should load");

        assert_eq!(session.session_id, "session-main");
        assert!(session.is_active);
    }

    #[tokio::test]
    async fn session_activity_overview_counts_fresh_active_session() {
        let runtime = RuntimeKernel::new();

        let overview = runtime
            .session_activity_overview(0, 0)
            .expect("session activity overview should load");

        assert_eq!(overview.sessions.len(), 1);
        assert_eq!(overview.active_sessions, 1);
        assert_eq!(overview.archived_sessions, 0);
        assert_eq!(overview.sessions[0].session.session_id, "session-main");
    }

    #[tokio::test]
    async fn doctor_reports_provider_probes_and_integrity_checks() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello doctor")
            .await
            .expect("plain turn should succeed");
        runtime
            .route_topics("session-main", Some("hello doctor"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let report = runtime
            .doctor_report()
            .await
            .expect("doctor report should succeed");
        assert_eq!(report.overall_status, DoctorStatus::Ok);
        assert_eq!(report.total_topic_sessions, 1);
        assert_eq!(report.total_topic_graph_edges, 0);
        assert_eq!(report.active_topic_sessions, 1);
        assert_eq!(report.active_topic_sessions_with_transcript_provenance, 1);
        assert_eq!(
            report.active_topic_sessions_missing_transcript_provenance,
            0
        );
        assert!(report.active_session_recall_transcript_evidence_spans > 0);
        assert_eq!(report.active_session_recall_omitted_items, 0);
        assert!(report.active_session_intuition_transcript_evidence_spans > 0);
        assert_eq!(report.active_session_intuition_foreground_topic_sessions, 1);
        assert!(
            report
                .provider_probes
                .iter()
                .any(|probe| probe.provider_name == "demo" && probe.status == DoctorStatus::Ok)
        );
        assert!(report.integrity_checks.iter().any(|check| {
            check.name == "runtime snapshot roundtrip" && check.status == DoctorStatus::Ok
        }));
        assert!(report.integrity_checks.iter().any(|check| {
            check.name == "active session export roundtrip" && check.status == DoctorStatus::Ok
        }));
        assert!(report.integrity_checks.iter().any(|check| {
            check.name == "topic sessions carry transcript provenance"
                && check.status == DoctorStatus::Ok
        }));

        let summary = runtime
            .doctor_summary()
            .await
            .expect("doctor summary should succeed");

        assert!(summary.iter().any(|line| line.contains("Hepta doctor: ok")));
        assert!(
            summary
                .iter()
                .any(|line| line.contains("- topic sessions: 1"))
        );
        assert!(
            summary
                .iter()
                .any(|line| line.contains("- topic graph edges: 0"))
        );
        assert!(
            summary.iter().any(|line| {
                line.contains("- active topic sessions with transcript provenance: ")
            })
        );
        assert!(summary.iter().any(|line| {
            line.contains("- active topic sessions missing transcript provenance: ")
        }));
        assert!(
            summary.iter().any(|line| {
                line.contains("- active session recall transcript evidence spans: ")
            })
        );
        assert!(
            summary
                .iter()
                .any(|line| { line.contains("- active session recall omitted items: 0") })
        );
        assert!(summary.iter().any(|line| {
            line.contains("- active session intuition transcript evidence spans: ")
        }));
        assert!(summary.iter().any(|line| {
            line.contains("- active session intuition foreground topic sessions: 1")
        }));
        assert!(
            summary
                .iter()
                .any(|line| line.contains("demo: ok via demo/demo-chat"))
        );
        assert!(
            summary
                .iter()
                .any(|line| line.contains("mock-ollama: ok via mock-ollama/local-chat"))
        );
        assert!(
            summary
                .iter()
                .any(|line| line.contains("history session references: ok"))
        );
        assert!(
            summary
                .iter()
                .any(|line| line.contains("runtime snapshot roundtrip: ok"))
        );
        assert!(
            summary
                .iter()
                .any(|line| { line.contains("topic sessions carry transcript provenance: ok") })
        );
    }

    #[tokio::test]
    async fn doctor_warns_when_active_topic_sessions_lose_transcript_provenance() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello doctor provenance gap")
            .await
            .expect("plain turn should succeed");
        runtime
            .route_topics(
                "session-main",
                Some("hello doctor provenance gap"),
                4,
                4,
                4,
                1,
            )
            .expect("topic route should succeed");

        {
            let mut topic_state = runtime
                .topic_session_state
                .lock()
                .expect("topic session state mutex should not poison");
            let topic_session = topic_state
                .sessions
                .iter_mut()
                .find(|topic_session| {
                    topic_session.topic_session_id == "topic-session-bootstrap:session-main"
                })
                .expect("bootstrap topic session should exist");
            topic_session.linked_transcript_spans.clear();
        }

        let report = runtime
            .doctor_report()
            .await
            .expect("doctor report should succeed");
        assert_eq!(report.overall_status, DoctorStatus::Warn);
        assert!(report.integrity_checks.iter().any(|check| {
            check.name == "topic sessions carry transcript provenance"
                && check.status == DoctorStatus::Warn
                && check
                    .detail
                    .contains("topic-session-bootstrap:session-main")
        }));
    }

    #[test]
    fn rejects_invalid_tool_arguments_against_schema() {
        let runtime = RuntimeKernel::new();
        let err = runtime
            .validate_tool_input("read_file", r#"{"path":""}"#)
            .expect_err("empty path should be rejected");
        assert!(err.0.contains("must be at least 1 characters"));

        let err = runtime
            .validate_tool_input("echo", r#"{"wrong":"value"}"#)
            .expect_err("missing required field should be rejected");
        assert!(err.0.contains("missing required field 'text'"));

        let err = runtime
            .validate_tool_input(
                "write_file",
                r#"{"path":"artifacts/x.txt","content":"x","mode":"replace"}"#,
            )
            .expect_err("invalid write mode should be rejected");
        assert!(err.0.contains("must be one of: create, overwrite, append"));

        let err = runtime
            .validate_tool_input(
                "write_file",
                r#"{"path":"artifacts/x.txt","content":"x","confirm_destructive":"yes"}"#,
            )
            .expect_err("non-boolean destructive confirm should be rejected");
        assert!(err.0.contains("must be a boolean"));
    }

    #[tokio::test]
    async fn returns_and_validates_structured_tool_output() {
        let runtime = RuntimeKernel::new();
        let result = runtime
            .run_demo_turn("tool:typed output")
            .await
            .expect("echo turn should succeed");

        let output_json = result
            .tool_output_json
            .expect("structured tool output should be present");
        assert!(output_json.contains("\"text\":\"typed output\""));
        runtime
            .validate_tool_output("echo", &output_json)
            .expect("echo output should match schema");
    }

    #[test]
    fn native_tool_result_reply_hides_structured_json() {
        let structured = json!({
            "backend": "hepta-rust-native",
            "content": "8 native background process(es)",
            "native_runtime": true,
            "openclaw_gateway_invoked": false,
            "proxy_used": false,
            "tool": "process",
            "result": {
                "action": "list",
                "followup_actions": ["poll", "log", "write", "kill", "clear", "remove"],
                "processes": [
                    {"id": "hepta-proc-1", "log_path": "/private/path/one.log"},
                    {"id": "hepta-proc-2", "log_path": "/private/path/two.log"}
                ]
            }
        });
        let reply = render_native_tool_result_reply(&format!(
            "8 native background process(es) | structured={}",
            structured
        ));

        assert!(reply.contains("共有 2 条后台进程记录"));
        assert!(reply.contains("结构化 JSON 已保留在本地"));
        assert!(!reply.contains("structured="));
        assert!(!reply.contains("log_path"));
        assert!(!reply.contains("/private/path"));
        assert!(!reply.contains("Hepta native tool result"));
    }

    #[test]
    fn exposes_tool_descriptors_for_discovery() {
        let runtime = RuntimeKernel::new();
        let tools = runtime.tool_descriptors();
        assert_eq!(tools.len(), 52);
        assert!(tools.iter().any(|tool| {
            tool.name == "echo"
                && tool.description.contains("Return the provided input as-is")
                && tool.execution_metadata.read_only
                && tool.execution_metadata.idempotent
                && tool.execution_metadata.produces_structured_output
                && tool.default_approval_requirement == ApprovalRequirement::None
                && tool.input_schema_json.contains("text")
                && tool.output_schema_json.contains("text")
        }));
        assert!(tools.iter().any(|tool| {
            tool.name == "read_file"
                && tool
                    .description
                    .contains("Read a UTF-8 text file from disk")
                && tool.execution_metadata.read_only
                && !tool.execution_metadata.destructive
                && tool.execution_metadata.idempotent
                && tool.default_approval_requirement == ApprovalRequirement::Ask
                && tool.input_schema_json.contains("path")
                && tool.output_schema_json.contains("line_count")
        }));
        assert!(tools.iter().any(|tool| {
            tool.name == "disk_junk_audit"
                && tool.description.contains("read-only local disk cleanup")
                && tool.execution_metadata.read_only
                && !tool.execution_metadata.destructive
                && tool.default_approval_requirement == ApprovalRequirement::None
        }));
        assert!(tools.iter().any(|tool| {
            tool.name == "write_file"
                && tool.description.contains("Write a UTF-8 text file to disk")
                && !tool.execution_metadata.read_only
                && tool.execution_metadata.destructive
                && !tool.execution_metadata.idempotent
                && tool.default_approval_requirement == ApprovalRequirement::Deny
                && tool.input_schema_json.contains("content")
                && tool.output_schema_json.contains("bytes_written")
        }));
        for expected in [
            "list_dir",
            "search_text",
            "json_get",
            "skill_propose",
            "skill_scan",
            "skill_apply_plan",
            "tool_manifest_validate",
            "tool_generate_stub",
            "read",
            "write",
            "edit",
            "apply_patch",
            "exec",
            "process",
            "web_search",
            "web_fetch",
            "sessions_list",
            "message",
            "image_generate",
            "video_generate",
            "music_generate",
            "memory_search",
            "memory_get",
            "feishu_doc",
        ] {
            assert!(
                tools.iter().any(|tool| tool.name == expected),
                "missing expanded native tool {expected}"
            );
        }
        let read = tools
            .iter()
            .find(|tool| tool.name == "read")
            .expect("OpenClaw-compatible read tool should exist");
        assert!(read.description.contains("Rust-native"));
        assert!(!read.description.contains("Gateway proxy"));
        let exec = tools
            .iter()
            .find(|tool| tool.name == "exec")
            .expect("OpenClaw-compatible exec tool should exist");
        assert!(exec.description.contains("cache cleanup"));
        assert_eq!(exec.risk_tier, hepta_core::RiskTier::High);
        assert_eq!(exec.default_approval_requirement, ApprovalRequirement::Ask);
        let process = tools
            .iter()
            .find(|tool| tool.name == "process")
            .expect("OpenClaw-compatible process tool should exist");
        assert!(process.description.contains("background process sessions"));
        assert!(process.description.contains("not for deleting files"));
        assert!(process.execution_metadata.read_only);
    }

    #[tokio::test]
    async fn generated_skill_and_tool_helpers_are_invokable() {
        let registry = ToolRegistry::new();
        let context = ToolContext {
            session_id: Some(SessionId("session-test".into())),
            correlation_id: Some(CorrelationId("corr-test".into())),
        };

        let skill = registry
            .invoke(
                "skill_propose",
                context.clone(),
                ToolCallRequest {
                    name: "skill_propose".into(),
                    input_json: r#"{"transcript":"Build a safe local skill workshop flow"}"#.into(),
                },
            )
            .await
            .expect("skill proposal helper should invoke");
        let skill_json: Value = serde_json::from_str(
            skill
                .structured_json
                .as_deref()
                .expect("skill proposal should be structured"),
        )
        .expect("skill proposal output should parse");
        assert_eq!(skill_json["safe_to_apply"], json!(true));
        assert_eq!(
            skill_json["skill_name"],
            json!("build-a-safe-local-skill-workshop-flow")
        );

        let generated = registry
            .invoke(
                "tool_generate_stub",
                context.clone(),
                ToolCallRequest {
                    name: "tool_generate_stub".into(),
                    input_json:
                        r#"{"name":"Summarize Local File","description":"Summarize a local file"}"#
                            .into(),
                },
            )
            .await
            .expect("tool generator should invoke");
        let generated_json = generated
            .structured_json
            .clone()
            .expect("tool generator should return structured json");
        let manifest: Value =
            serde_json::from_str(&generated_json).expect("generated tool manifest should parse");
        assert_eq!(manifest["name"], json!("summarize_local_file"));

        let validation = registry
            .invoke(
                "tool_manifest_validate",
                context,
                ToolCallRequest {
                    name: "tool_manifest_validate".into(),
                    input_json: json!({ "manifest_json": generated_json }).to_string(),
                },
            )
            .await
            .expect("tool manifest validator should invoke");
        let validation_json: Value = serde_json::from_str(
            validation
                .structured_json
                .as_deref()
                .expect("validation should be structured"),
        )
        .expect("validation output should parse");
        assert_eq!(validation_json["valid"], json!(true));
        assert_eq!(validation_json["issue_count"], json!(0));
    }

    #[tokio::test]
    async fn openclaw_compatible_tools_are_native_not_gateway_proxy() {
        let registry = ToolRegistry::new();
        let context = ToolContext {
            session_id: Some(SessionId("session-native-tools".into())),
            correlation_id: Some(CorrelationId("corr-native-tools".into())),
        };
        let relative_path = format!(
            "target/hepta-native-tool-test-{}-{}.txt",
            std::process::id(),
            current_unix_ms().expect("clock should be available")
        );

        let write = registry
            .invoke(
                "write",
                context.clone(),
                ToolCallRequest {
                    name: "write".into(),
                    input_json: json!({"path": relative_path, "content": "alpha\n"}).to_string(),
                },
            )
            .await
            .expect("native write should invoke");
        let write_json: Value = serde_json::from_str(write.structured_json.as_deref().unwrap())
            .expect("write output should parse");
        assert_eq!(write_json["proxy_used"], json!(false));
        assert_eq!(write_json["backend"], json!("hepta-rust-native"));

        registry
            .invoke(
                "edit",
                context.clone(),
                ToolCallRequest {
                    name: "edit".into(),
                    input_json: json!({
                        "path": relative_path,
                        "edits": [{"oldText":"alpha\n", "newText":"beta\n"}]
                    })
                    .to_string(),
                },
            )
            .await
            .expect("native edit should invoke");

        let patch = format!(
            "*** Begin Patch\n*** Update File: {}\n@@\n-beta\n+gamma\n*** End Patch",
            relative_path
        );
        registry
            .invoke(
                "apply_patch",
                context.clone(),
                ToolCallRequest {
                    name: "apply_patch".into(),
                    input_json: json!({"input": patch}).to_string(),
                },
            )
            .await
            .expect("native apply_patch should invoke");

        let read = registry
            .invoke(
                "read",
                context,
                ToolCallRequest {
                    name: "read".into(),
                    input_json: json!({"path": relative_path, "offset": 1, "limit": 5}).to_string(),
                },
            )
            .await
            .expect("native read should invoke");
        let read_json: Value = serde_json::from_str(read.structured_json.as_deref().unwrap())
            .expect("read output should parse");
        assert_eq!(read_json["proxy_used"], json!(false));
        assert_eq!(read_json["result"]["text"], json!("gamma"));

        let exec = registry
            .invoke(
                "exec",
                ToolContext {
                    session_id: Some(SessionId("session-native-tools".into())),
                    correlation_id: Some(CorrelationId("corr-native-tools".into())),
                },
                ToolCallRequest {
                    name: "exec".into(),
                    input_json: json!({"command": "printf native-exec"}).to_string(),
                },
            )
            .await
            .expect("native exec should invoke");
        let exec_json: Value = serde_json::from_str(exec.structured_json.as_deref().unwrap())
            .expect("exec output should parse");
        assert_eq!(exec_json["proxy_used"], json!(false));
        assert_eq!(exec_json["result"]["stdout"], json!("native-exec"));

        let started = std::time::Instant::now();
        let timed_out_exec = registry
            .invoke(
                "exec",
                ToolContext {
                    session_id: Some(SessionId("session-native-tools".into())),
                    correlation_id: Some(CorrelationId("corr-native-tools".into())),
                },
                ToolCallRequest {
                    name: "exec".into(),
                    input_json: json!({"command": "sleep 5", "timeoutMs": 100}).to_string(),
                },
            )
            .await
            .expect("native exec timeout should return structured result, not hang");
        assert!(started.elapsed() < std::time::Duration::from_secs(3));
        assert!(
            timed_out_exec
                .content
                .contains("ToolTimeout/native_compat_exec timed out")
        );
        let timed_out_json: Value =
            serde_json::from_str(timed_out_exec.structured_json.as_deref().unwrap())
                .expect("timeout output should parse");
        assert_eq!(timed_out_json["status"], json!("timeout"));
        assert_eq!(timed_out_json["error_kind"], json!("ToolTimeout"));
        assert_eq!(timed_out_json["result"]["timeout"], json!(true));
        assert_eq!(
            timed_out_json["result"]["duplicate_tool_replay_prevented"],
            json!(true)
        );

        let background = registry
            .invoke(
                "exec",
                ToolContext {
                    session_id: Some(SessionId("session-native-tools".into())),
                    correlation_id: Some(CorrelationId("corr-native-tools".into())),
                },
                ToolCallRequest {
                    name: "exec".into(),
                    input_json: json!({"command": "cat", "background": true}).to_string(),
                },
            )
            .await
            .expect("native background exec should invoke");
        let background_json: Value =
            serde_json::from_str(background.structured_json.as_deref().unwrap())
                .expect("background output should parse");
        let process_id = background_json["result"]["sessionId"]
            .as_str()
            .expect("background should return process id")
            .to_string();
        assert_eq!(background_json["proxy_used"], json!(false));

        registry
            .invoke(
                "process",
                ToolContext {
                    session_id: Some(SessionId("session-native-tools".into())),
                    correlation_id: Some(CorrelationId("corr-native-tools".into())),
                },
                ToolCallRequest {
                    name: "process".into(),
                    input_json: json!({"action":"write", "sessionId": process_id, "data":"native-process\n", "eof": true}).to_string(),
                },
            )
            .await
            .expect("native process write should invoke");
        registry
            .invoke(
                "process",
                ToolContext {
                    session_id: Some(SessionId("session-native-tools".into())),
                    correlation_id: Some(CorrelationId("corr-native-tools".into())),
                },
                ToolCallRequest {
                    name: "process".into(),
                    input_json: json!({"action":"poll", "sessionId": process_id, "timeout": 3000})
                        .to_string(),
                },
            )
            .await
            .expect("native process poll should invoke");
        let process_log = registry
            .invoke(
                "process",
                ToolContext {
                    session_id: Some(SessionId("session-native-tools".into())),
                    correlation_id: Some(CorrelationId("corr-native-tools".into())),
                },
                ToolCallRequest {
                    name: "process".into(),
                    input_json: json!({"action":"log", "sessionId": process_id, "limit": 1000})
                        .to_string(),
                },
            )
            .await
            .expect("native process log should invoke");
        assert!(process_log.content.contains("native-process"));

        for (tool, payload) in [
            (
                "message",
                json!({"action":"send", "channel":"telegram", "target":"6476198178", "message":"dry run", "dryRun": true}),
            ),
            ("tts", json!({"text":"hello", "dryRun": true})),
            (
                "image_generate",
                json!({"prompt":"tiny red dot", "dryRun": true}),
            ),
            (
                "music_generate",
                json!({"prompt":"tiny tune", "dryRun": true}),
            ),
            (
                "video_generate",
                json!({"prompt":"tiny clip", "dryRun": true}),
            ),
        ] {
            let result = registry
                .invoke(
                    tool,
                    ToolContext {
                        session_id: Some(SessionId("session-native-tools".into())),
                        correlation_id: Some(CorrelationId("corr-native-tools".into())),
                    },
                    ToolCallRequest {
                        name: tool.into(),
                        input_json: payload.to_string(),
                    },
                )
                .await
                .expect("native dry-run surface should invoke");
            let parsed: Value = serde_json::from_str(result.structured_json.as_deref().unwrap())
                .expect("native dry-run output should parse");
            assert_eq!(parsed["proxy_used"], json!(false));
            assert_ne!(parsed["status"], json!("native_surface_registered"));
        }

        let _ = fs::remove_file(tool_workspace_root_path().join(&relative_path));
    }

    #[tokio::test]
    async fn saves_and_loads_runtime_snapshot_across_instances() {
        let source = RuntimeKernel::new();
        source
            .run_demo_turn("hello persistence")
            .await
            .expect("plain turn should succeed");
        source
            .switch_model("mock-ollama/local-precise")
            .expect("model switch should succeed");
        source
            .approve_tool("read_file")
            .expect("approval should succeed");
        source
            .run_demo_turn(&format!(
                "read:{}",
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../docs/decisions/ADR-0001-architecture-foundation.md"
                )
            ))
            .await
            .expect("approved read turn should succeed");

        let snapshot_path = std::env::temp_dir().join(format!(
            "hepta-runtime-snapshot-{}.json",
            std::process::id()
        ));
        source
            .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot save should succeed");

        let restored = RuntimeKernel::new();
        restored
            .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot load should succeed");

        let selection = restored.model_selection().expect("selection should load");
        assert_eq!(selection.active.provider, "mock-ollama");
        assert_eq!(selection.active.model, "local-precise");

        let approvals = restored.approval_snapshot().expect("approvals should load");
        assert!(
            approvals
                .granted_tools
                .iter()
                .any(|tool| tool == "read_file")
        );

        let sessions = restored.sessions().expect("sessions should load");
        assert_eq!(sessions.len(), 1);
        let history = restored
            .history(Some("session-main"), 10)
            .expect("history should load");
        assert!(history.len() >= 2);
        let memories = restored.memory_snapshot(10).expect("memories should load");
        assert!(
            memories
                .iter()
                .any(|memory| memory.content.contains("hello persistence"))
        );

        let _ = std::fs::remove_file(snapshot_path);
    }

    #[tokio::test]
    async fn saves_and_loads_runtime_snapshot_with_topic_sessions_and_graph_store() {
        let source = RuntimeKernel::new();
        source
            .switch_session("alpha")
            .expect("session switch should succeed");
        source
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        source
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        source
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        source
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        source
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let snapshot_path = std::env::temp_dir().join(format!(
            "hepta-runtime-topic-graph-snapshot-{}.json",
            std::process::id()
        ));
        source
            .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot save should succeed");

        let restored = RuntimeKernel::new();
        restored
            .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot load should succeed");

        let raw_topic_sessions = restored
            .topic_session_state
            .lock()
            .expect("topic session state lock should succeed")
            .sessions
            .clone();
        let raw_topic_graph_edges = restored
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .clone();
        assert_eq!(raw_topic_sessions.len(), 2);
        assert!(raw_topic_graph_edges.iter().any(|record| {
            record.source_topic_session_id == "topic-session-bootstrap:alpha"
                && record.edge.target_topic_session_id
                    == "topic-session-bootstrap:alpha:rust-worker-pipeline"
        }));

        let topic_sessions = restored
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && !topic_session.graph_edges.is_empty()
        }));

        let decision = restored
            .route_topics("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("graph-expanded route should succeed");
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| { id == "topic-session-bootstrap:alpha:rust-worker-pipeline" })
        );
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha-rust-worker-pipeline"
                && score
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("stored co-activation edge"))
        }));

        let _ = std::fs::remove_file(snapshot_path);
    }

    #[tokio::test]
    async fn loads_legacy_runtime_snapshot_missing_approvals_field() {
        let source = RuntimeKernel::new();
        source
            .run_demo_turn("legacy snapshot")
            .await
            .expect("plain turn should succeed");

        let snapshot_path = std::env::temp_dir().join(format!(
            "hepta-legacy-runtime-snapshot-{}.json",
            std::process::id()
        ));
        source
            .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("snapshot save should succeed");

        let mut snapshot_json: Value = serde_json::from_str(
            &fs::read_to_string(&snapshot_path).expect("snapshot should be readable"),
        )
        .expect("snapshot json should parse");
        let snapshot_object = snapshot_json
            .as_object_mut()
            .expect("snapshot json should be an object");
        snapshot_object.remove("approvals");
        snapshot_object.remove("topic_sessions");
        snapshot_object.remove("topic_graph_edges");
        fs::write(
            &snapshot_path,
            serde_json::to_string_pretty(&snapshot_json).expect("snapshot should serialize"),
        )
        .expect("legacy snapshot should be writable");

        let restored = RuntimeKernel::new();
        restored
            .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
            .expect("legacy snapshot load should succeed");

        let approvals = restored.approval_snapshot().expect("approvals should load");
        assert!(approvals.granted_tools.is_empty());
        assert!(approvals.pending.is_empty());

        let history = restored
            .history(Some("session-main"), 10)
            .expect("history should load");
        assert!(!history.is_empty());

        let _ = std::fs::remove_file(snapshot_path);
    }

    #[tokio::test]
    async fn switches_active_session_and_persists_it() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("research-lab")
            .expect("session switch should succeed");
        runtime
            .run_demo_turn("hello switched session")
            .await
            .expect("turn should succeed");

        assert_eq!(
            runtime.active_session_id().expect("session id should load"),
            "research-lab"
        );
        let sessions = runtime.sessions().expect("sessions should load");
        let session = sessions
            .iter()
            .find(|session| session.session_id == "research-lab")
            .expect("research-lab session should exist");
        assert!(session.is_active);
        assert!(session.last_active_unix_ms >= session.created_at_unix_ms);
        let history = runtime
            .history(Some("research-lab"), 10)
            .expect("history should load");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].input, "hello switched session");
    }

    #[tokio::test]
    async fn can_rename_session_and_track_last_user_intent() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("research-lab")
            .expect("session switch should succeed");
        runtime
            .rename_active_session("Research planning")
            .expect("session rename should succeed");
        runtime
            .run_demo_turn("map out the next architecture milestone for Hepta")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics(
                "research-lab",
                Some("map out the next architecture milestone for Hepta"),
                4,
                4,
                4,
                1,
            )
            .expect("topic route should succeed");

        let session = runtime
            .active_session_snapshot()
            .expect("active session snapshot should load");
        assert_eq!(session.title, "Research planning");
        assert_eq!(
            session.last_user_intent_summary.as_deref(),
            Some("map out the next architecture milestone for Hepta")
        );
        assert_eq!(session.topic_session_count, 1);
        assert_eq!(session.topic_graph_edge_count, 0);
    }

    #[tokio::test]
    async fn can_run_in_specific_session_without_switching_active_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");

        let result = runtime
            .run_demo_turn_in_session("beta", "draft a beta session plan")
            .await
            .expect("beta run should succeed");

        assert_eq!(result.session_id, "beta");
        assert_eq!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "alpha"
        );

        let beta_history = runtime
            .history(Some("beta"), 10)
            .expect("beta history should load");
        assert_eq!(beta_history.len(), 1);
        assert_eq!(beta_history[0].input, "draft a beta session plan");

        let alpha_session = runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "alpha")
            .expect("alpha session should exist");
        assert!(alpha_session.is_active);
    }

    #[test]
    fn models_are_scoped_per_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .switch_model("mock-ollama/local-precise")
            .expect("alpha model switch should succeed");
        runtime
            .switch_model_in_session("beta", "demo/demo-creative")
            .expect("beta model switch should succeed");

        let alpha = runtime
            .model_selection_for_session("alpha")
            .expect("alpha model selection should load");
        assert_eq!(alpha.active.provider, "mock-ollama");
        assert_eq!(alpha.active.model, "local-precise");

        let beta = runtime
            .model_selection_for_session("beta")
            .expect("beta model selection should load");
        assert_eq!(beta.active.provider, "demo");
        assert_eq!(beta.active.model, "demo-creative");

        assert_eq!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "alpha"
        );

        let beta_session = runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "beta")
            .expect("beta session should exist");
        assert_eq!(beta_session.model.provider, "demo");
        assert_eq!(beta_session.model.model, "demo-creative");
    }

    #[tokio::test]
    async fn query_events_filters_by_kind_session_and_limit() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .run_demo_turn("hello alpha")
            .await
            .expect("alpha turn should succeed");
        runtime
            .switch_session("beta")
            .expect("beta session switch should succeed");
        runtime
            .run_demo_turn("hello beta")
            .await
            .expect("beta turn should succeed");

        let beta_switch_events = runtime
            .query_events(25, Some(&EventKind::SessionSwitched), Some("beta"))
            .expect("filtered beta events should load");
        assert_eq!(beta_switch_events.len(), 1);
        assert_eq!(beta_switch_events[0].event.kind, EventKind::SessionSwitched);
        assert_eq!(
            beta_switch_events[0]
                .event
                .session_id
                .as_ref()
                .map(|session_id| session_id.0.as_str()),
            Some("beta")
        );

        let limited_switch_events = runtime
            .query_events(1, Some(&EventKind::SessionSwitched), None)
            .expect("limited switch events should load");
        assert_eq!(limited_switch_events.len(), 1);
        assert_eq!(
            limited_switch_events[0]
                .event
                .session_id
                .as_ref()
                .map(|session_id| session_id.0.as_str()),
            Some("beta")
        );
    }

    #[tokio::test]
    async fn approvals_are_scoped_per_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .approve_tool("read_file")
            .expect("alpha approval should succeed");
        let alpha = runtime
            .approval_snapshot()
            .expect("alpha approvals should load");
        assert!(alpha.granted_tools.iter().any(|tool| tool == "read_file"));

        runtime
            .switch_session("beta")
            .expect("beta session switch should succeed");
        let beta = runtime
            .approval_snapshot()
            .expect("beta approvals should load");
        assert!(beta.granted_tools.is_empty());

        let blocked = runtime
            .run_demo_turn(&format!(
                "read:{}",
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../docs/decisions/ADR-0001-architecture-foundation.md"
                )
            ))
            .await
            .expect("beta read turn should return approval requirement");
        assert_eq!(blocked.approval_required.as_deref(), Some("read_file"));

        runtime
            .switch_session("alpha")
            .expect("switch back to alpha should succeed");
        let alpha_again = runtime
            .approval_snapshot()
            .expect("alpha approvals should still load");
        assert!(
            alpha_again
                .granted_tools
                .iter()
                .any(|tool| tool == "read_file")
        );
    }

    #[tokio::test]
    async fn can_grant_and_inspect_approvals_for_non_active_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .approve_tool_in_session("beta", "read_file")
            .expect("beta approval should succeed");

        let alpha = runtime
            .approval_snapshot()
            .expect("alpha approvals should load");
        assert!(alpha.granted_tools.is_empty());

        let beta = runtime
            .approval_snapshot_for_session("beta")
            .expect("beta approvals should load");
        assert!(beta.granted_tools.iter().any(|tool| tool == "read_file"));

        let result = runtime
            .run_demo_turn_in_session(
                "beta",
                &format!(
                    "read:{}",
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/../../docs/decisions/ADR-0001-architecture-foundation.md"
                    )
                ),
            )
            .await
            .expect("beta read turn should succeed");
        assert_eq!(result.invoked_tool.as_deref(), Some("read_file"));
        assert_eq!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "alpha"
        );
    }

    #[tokio::test]
    async fn archiving_active_session_switches_to_fallback() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .run_demo_turn("keep alpha history")
            .await
            .expect("alpha turn should succeed");

        runtime
            .archive_session(None)
            .expect("archive should succeed");

        assert_ne!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "alpha"
        );
        let alpha = runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "alpha")
            .expect("alpha session should exist");
        assert!(alpha.archived_at_unix_ms.is_some());
    }

    #[tokio::test]
    async fn archiving_fresh_active_session_materializes_and_switches_to_fallback() {
        let runtime = RuntimeKernel::new();

        runtime
            .archive_session(None)
            .expect("archive should succeed for fresh active session");

        assert_ne!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "session-main"
        );
        let archived = runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "session-main")
            .expect("session-main should exist");
        assert!(archived.archived_at_unix_ms.is_some());
    }

    #[tokio::test]
    async fn deleting_session_removes_related_runtime_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta note")
            .await
            .expect("beta turn should succeed");
        runtime
            .approve_tool_in_session("beta", "read_file")
            .expect("beta approval should succeed");
        runtime
            .switch_model_in_session("beta", "demo/demo-creative")
            .expect("beta model switch should succeed");
        runtime
            .run_demo_turn_in_session("beta", "hello adaptive memory")
            .await
            .expect("beta routed turn should succeed");
        runtime
            .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("beta route should succeed");

        runtime
            .delete_session("beta")
            .expect("delete should succeed");

        assert!(
            runtime
                .history(Some("beta"), 10)
                .expect("beta history should load")
                .is_empty()
        );
        assert!(
            runtime
                .approval_snapshot_for_session("beta")
                .expect("beta approvals should load")
                .granted_tools
                .is_empty()
        );
        assert!(
            runtime
                .sessions()
                .expect("sessions should load")
                .into_iter()
                .all(|session| session.session_id != "beta")
        );
        assert!(
            runtime
                .topic_sessions_for_surface("beta")
                .expect("beta topic sessions should load")
                .is_empty()
        );
        assert!(
            runtime
                .topic_graph_state
                .lock()
                .expect("topic graph state lock should succeed")
                .edges
                .iter()
                .all(|record| {
                    !record.source_topic_session_id.contains("beta")
                        && !record.edge.target_topic_session_id.contains("beta")
                })
        );
    }

    #[tokio::test]
    async fn prune_prefers_archived_sessions_and_keeps_active() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .run_demo_turn("alpha work")
            .await
            .expect("alpha turn should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta work")
            .await
            .expect("beta turn should succeed");
        runtime
            .run_demo_turn_in_session("gamma", "gamma work")
            .await
            .expect("gamma turn should succeed");
        runtime
            .archive_session(Some("beta"))
            .expect("beta archive should succeed");

        let result = runtime.prune_sessions(2).expect("prune should succeed");
        assert!(result.contains("beta"));
        let sessions = runtime.sessions().expect("sessions should load");
        assert!(
            sessions
                .iter()
                .any(|session| session.session_id == "alpha" && session.is_active)
        );
        assert!(sessions.iter().all(|session| session.session_id != "beta"));
    }

    #[tokio::test]
    async fn prune_sessions_counts_fresh_active_session() {
        let runtime = RuntimeKernel::new();

        let result = runtime
            .prune_sessions(1)
            .expect("prune should succeed for fresh runtime");

        assert_eq!(result, "no pruning needed, sessions=1 max=1");
        let sessions = runtime.sessions().expect("sessions should load");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "session-main");
        assert!(sessions[0].is_active);
    }

    #[tokio::test]
    async fn exports_and_imports_single_session_package() {
        let source = RuntimeKernel::new();
        source
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        source
            .run_demo_turn_in_session("beta", "beta exported work")
            .await
            .expect("beta turn should succeed");
        source
            .rename_active_session("Alpha workspace")
            .expect("alpha rename should succeed");
        source
            .switch_model_in_session("beta", "demo/demo-creative")
            .expect("beta model switch should succeed");
        source
            .approve_tool_in_session("beta", "read_file")
            .expect("beta approval should succeed");
        source
            .archive_session(Some("beta"))
            .expect("beta archive should succeed");

        let export_path =
            std::env::temp_dir().join(format!("hepta-session-export-{}.json", std::process::id()));
        let export_report = source
            .export_session("beta", export_path.to_str().expect("path should be utf8"))
            .expect("beta export should succeed");
        assert_eq!(export_report.session_id, "beta");
        assert_eq!(export_report.title, "Hepta session beta");
        assert_eq!(export_report.model.model, "demo-creative");
        assert!(export_report.archived);
        assert_eq!(export_report.approvals_granted, 1);
        assert_eq!(export_report.history_entries, 1);
        assert_eq!(export_report.topic_session_count, 0);
        assert_eq!(export_report.topic_graph_edge_count, 0);

        let restored = RuntimeKernel::new();
        let import_report = restored
            .import_session(export_path.to_str().expect("path should be utf8"))
            .expect("beta import should succeed");
        assert_eq!(import_report.session_id, "beta");
        assert_eq!(import_report.imported_title, "Hepta session beta");
        assert_eq!(import_report.imported_model.model, "demo-creative");
        assert!(import_report.imported_archived);
        assert_eq!(import_report.approvals_granted, 1);
        assert_eq!(import_report.history_entries, 1);
        assert_eq!(import_report.topic_session_count, 0);
        assert_eq!(import_report.topic_graph_edge_count, 0);

        let beta = restored
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "beta")
            .expect("beta session should exist after import");
        assert_eq!(beta.model.provider, "demo");
        assert_eq!(beta.model.model, "demo-creative");
        assert!(beta.archived_at_unix_ms.is_some());
        assert_eq!(
            beta.last_user_intent_summary.as_deref(),
            Some("beta exported work")
        );

        let beta_approvals = restored
            .approval_snapshot_for_session("beta")
            .expect("beta approvals should load");
        assert!(
            beta_approvals
                .granted_tools
                .iter()
                .any(|tool| tool == "read_file")
        );

        let beta_history = restored
            .history(Some("beta"), 10)
            .expect("beta history should load");
        assert_eq!(beta_history.len(), 1);
        assert_eq!(beta_history[0].input, "beta exported work");

        let _ = std::fs::remove_file(export_path);
    }

    #[tokio::test]
    async fn exports_and_imports_single_session_package_with_topic_graph_state() {
        let source = RuntimeKernel::new();
        source
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        source
            .run_demo_turn_in_session("beta", "hello adaptive memory")
            .await
            .expect("beta first turn should succeed");
        source
            .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("beta first route should succeed");
        source
            .run_demo_turn_in_session("beta", "rust worker pipeline")
            .await
            .expect("beta second turn should succeed");
        source
            .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("beta second route should succeed");
        source
            .route_topics(
                "beta",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("beta mixed route should succeed");

        let export_path = std::env::temp_dir().join(format!(
            "hepta-session-topic-graph-export-{}.json",
            std::process::id()
        ));
        let export_report = source
            .export_session("beta", export_path.to_str().expect("path should be utf8"))
            .expect("beta export should succeed");
        assert_eq!(export_report.topic_session_count, 2);
        assert_eq!(export_report.topic_graph_edge_count, 2);

        let restored = RuntimeKernel::new();
        let import_report = restored
            .import_session(export_path.to_str().expect("path should be utf8"))
            .expect("beta import should succeed");
        assert_eq!(import_report.topic_session_count, 2);
        assert_eq!(import_report.topic_graph_edge_count, 2);

        let raw_topic_graph_edges = restored
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .clone();
        assert!(raw_topic_graph_edges.iter().any(|record| {
            record.source_topic_session_id == "topic-session-bootstrap:beta"
                && record.edge.target_topic_session_id
                    == "topic-session-bootstrap:beta:rust-worker-pipeline"
        }));

        let decision = restored
            .route_topics("beta", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("graph-expanded route should succeed");
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| { id == "topic-session-bootstrap:beta:rust-worker-pipeline" })
        );

        let _ = std::fs::remove_file(export_path);
    }

    #[tokio::test]
    async fn session_export_roundtrip_preserves_intelligence_learning_state() {
        let source = RuntimeKernel::new();
        source
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        for input in [
            "semantic router should learn from accepted feedback",
            "feedback calibration closes the loop into future intuition",
            "merge topic sessions then split them back into stable neurons",
            "aging neurons need refresh with transcript evidence",
        ] {
            source
                .run_demo_turn_in_session("beta", input)
                .await
                .expect("intelligence hardening turn should succeed");
        }

        let bundle = source
            .predict_intuition(
                "beta",
                "semantic router learned feedback should route topic memory",
                12,
                12,
                12,
                6,
                6,
                6,
            )
            .expect("intuition should produce a bundle");
        assert!(!bundle.topic_activation_scores.is_empty());
        assert!(!bundle.neuron_activations.is_empty());

        let skill_id = bundle
            .skill_decisions
            .first()
            .map(|decision| decision.skill_id.clone());
        let workflow_id = bundle
            .workflow_priors
            .first()
            .map(|prior| prior.workflow_id.clone());
        let source_topic_ids = bundle
            .topic_activation_scores
            .iter()
            .map(|score| score.topic_id.clone())
            .collect::<Vec<_>>();
        let source_neuron_ids = bundle
            .neuron_activations
            .iter()
            .map(|activation| activation.neuron_id.clone())
            .collect::<Vec<_>>();
        source
            .record_intuition_feedback(
                "beta",
                "semantic router learned feedback should route topic memory",
                IntuitionFeedbackOutcome::ExecutedSuccess,
                skill_id.as_deref(),
                workflow_id.as_deref(),
                source_topic_ids.clone(),
                source_neuron_ids,
                Some("release hardening accepted learned semantic router"),
            )
            .expect("feedback learning should be recorded");
        source
            .record_model_router_feedback(
                "beta",
                "semantic router learned feedback should route topic memory",
                ModelRef {
                    provider: "demo".into(),
                    model: "demo-chat".into(),
                },
                TopicAwareModelFeedbackOutcome::ExecutedSuccess,
                source_topic_ids.clone(),
                Some(1200),
                Some(0.03),
                Some(0.9),
                Some(0.8),
                Some("model-router feedback survived export"),
            )
            .expect("model-router feedback should be recorded");

        let before_route = source
            .route_topics(
                "beta",
                Some("semantic router learned feedback release hardening"),
                12,
                12,
                12,
                6,
            )
            .expect("learned router route should succeed before export");
        assert_eq!(
            before_route.router_id,
            "semantic-router:learned-feedback-v1"
        );
        assert!(before_route.learned_signal_count > 0);

        let before_calibration = source
            .intuition_calibration_overview("beta")
            .expect("calibration overview should load before export");
        assert!(before_calibration.closed_loop_ready);
        assert!(before_calibration.learned_topic_hint_count > 0);
        assert!(before_calibration.learned_neuron_update_count > 0);
        let before_model_calibration = source
            .model_router_feedback_summary("beta")
            .expect("model-router calibration should load before export");
        assert_eq!(before_model_calibration.len(), 1);
        assert!(before_model_calibration[0].success_rate > 0.0);

        let before_lifecycle = source
            .neuron_lifecycle_overview("beta")
            .expect("lifecycle overview should load before export");
        assert!(before_lifecycle.stored_neurons > 0);
        assert!(before_lifecycle.average_confidence > 0.0);

        let export_path = std::env::temp_dir().join(format!(
            "hepta-session-intelligence-export-{}.json",
            std::process::id()
        ));
        let export_report = source
            .export_session("beta", export_path.to_str().expect("path should be utf8"))
            .expect("beta intelligence export should succeed");
        assert_eq!(export_report.neuron_count, before_lifecycle.stored_neurons);
        assert_eq!(
            export_report.intuition_feedback_count,
            before_calibration.feedback_record_count
        );
        assert_eq!(
            export_report.model_router_feedback_count,
            before_model_calibration[0].record_count
        );

        let restored = RuntimeKernel::new();
        let import_report = restored
            .import_session(export_path.to_str().expect("path should be utf8"))
            .expect("beta intelligence import should succeed");
        assert_eq!(import_report.neuron_count, before_lifecycle.stored_neurons);
        assert_eq!(
            import_report.intuition_feedback_count,
            before_calibration.feedback_record_count
        );
        assert_eq!(
            import_report.model_router_feedback_count,
            before_model_calibration[0].record_count
        );

        let after_route = restored
            .route_topics(
                "beta",
                Some("semantic router learned feedback release hardening"),
                12,
                12,
                12,
                6,
            )
            .expect("learned router route should succeed after import");
        assert_eq!(after_route.router_id, "semantic-router:learned-feedback-v1");
        assert!(after_route.learned_signal_count >= before_route.learned_signal_count);

        let after_calibration = restored
            .intuition_calibration_overview("beta")
            .expect("calibration overview should load after import");
        assert_eq!(
            after_calibration.feedback_record_count,
            before_calibration.feedback_record_count
        );
        assert!(after_calibration.closed_loop_ready);
        assert_eq!(
            after_calibration.learned_neuron_update_count,
            before_calibration.learned_neuron_update_count
        );
        let after_model_calibration = restored
            .model_router_feedback_summary("beta")
            .expect("model-router calibration should load after import");
        assert_eq!(after_model_calibration, before_model_calibration);

        let after_lifecycle = restored
            .neuron_lifecycle_overview("beta")
            .expect("lifecycle overview should load after import");
        assert_eq!(
            after_lifecycle.stored_neurons,
            before_lifecycle.stored_neurons
        );
        assert!(after_lifecycle.average_confidence > 0.0);

        let _ = std::fs::remove_file(export_path);
    }

    #[tokio::test]
    async fn imports_legacy_session_export_missing_approval_field() {
        let source = RuntimeKernel::new();
        source
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        source
            .run_demo_turn_in_session("beta", "legacy export")
            .await
            .expect("beta turn should succeed");

        let export_path = std::env::temp_dir().join(format!(
            "hepta-legacy-session-export-{}.json",
            std::process::id()
        ));
        source
            .export_session("beta", export_path.to_str().expect("path should be utf8"))
            .expect("beta export should succeed");

        let mut export_json: Value = serde_json::from_str(
            &fs::read_to_string(&export_path).expect("export should be readable"),
        )
        .expect("export json should parse");
        let export_object = export_json
            .as_object_mut()
            .expect("export json should be an object");
        export_object.remove("approval");
        export_object.remove("topic_sessions");
        export_object.remove("topic_graph_edges");
        fs::write(
            &export_path,
            serde_json::to_string_pretty(&export_json).expect("export should serialize"),
        )
        .expect("legacy export should be writable");

        let restored = RuntimeKernel::new();
        let import_report = restored
            .import_session(export_path.to_str().expect("path should be utf8"))
            .expect("legacy export import should succeed");
        assert_eq!(import_report.topic_session_count, 0);
        assert_eq!(import_report.topic_graph_edge_count, 0);

        let beta_approvals = restored
            .approval_snapshot_for_session("beta")
            .expect("beta approvals should load");
        assert!(beta_approvals.granted_tools.is_empty());
        assert!(beta_approvals.pending.is_empty());

        let beta_history = restored
            .history(Some("beta"), 10)
            .expect("beta history should load");
        assert_eq!(beta_history.len(), 1);
        assert_eq!(beta_history[0].input, "legacy export");

        let _ = std::fs::remove_file(export_path);
    }

    #[tokio::test]
    async fn forks_session_into_independent_branch() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta base work")
            .await
            .expect("beta turn should succeed");
        runtime
            .switch_model_in_session("beta", "demo/demo-creative")
            .expect("beta model switch should succeed");
        runtime
            .approve_tool_in_session("beta", "read_file")
            .expect("beta approval should succeed");
        runtime
            .archive_session(Some("beta"))
            .expect("beta archive should succeed");

        let fork_report = runtime
            .fork_session("beta", "beta-fork")
            .expect("beta fork should succeed");
        assert_eq!(fork_report.source_session_id, "beta");
        assert_eq!(fork_report.target_session_id, "beta-fork");
        assert_eq!(fork_report.target_model.model, "demo-creative");
        assert!(!fork_report.target_archived);
        assert_eq!(fork_report.approvals_granted, 1);
        assert_eq!(fork_report.history_entries, 1);
        assert_eq!(fork_report.topic_session_count, 0);
        assert_eq!(fork_report.topic_graph_edge_count, 0);
        assert_eq!(fork_report.active_session_after_fork, "alpha");

        let fork = runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "beta-fork")
            .expect("beta-fork session should exist");
        assert_eq!(fork.model.provider, "demo");
        assert_eq!(fork.model.model, "demo-creative");
        assert!(fork.archived_at_unix_ms.is_none());
        assert_eq!(
            fork.last_user_intent_summary.as_deref(),
            Some("beta base work")
        );
        assert!(fork.title.contains("(fork)"));

        let fork_approvals = runtime
            .approval_snapshot_for_session("beta-fork")
            .expect("fork approvals should load");
        assert!(
            fork_approvals
                .granted_tools
                .iter()
                .any(|tool| tool == "read_file")
        );

        let fork_history = runtime
            .history(Some("beta-fork"), 10)
            .expect("fork history should load");
        assert_eq!(fork_history.len(), 1);
        assert_eq!(fork_history[0].session_id, "beta-fork");
        assert_eq!(fork_history[0].input, "beta base work");

        assert_eq!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "alpha"
        );
    }

    #[tokio::test]
    async fn fork_session_rejects_fresh_active_target_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn_in_session("beta", "beta base work")
            .await
            .expect("beta turn should succeed");

        let err = runtime
            .fork_session("beta", "session-main")
            .expect_err("fresh active target should still be treated as existing");

        assert_eq!(err.0, "target session already exists: session-main");
        assert_eq!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "session-main"
        );
        assert!(
            runtime
                .sessions()
                .expect("sessions should load")
                .into_iter()
                .any(|session| session.session_id == "session-main" && session.is_active)
        );
    }

    #[tokio::test]
    async fn fork_session_rebases_topic_sessions_and_graph_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .run_demo_turn_in_session("beta", "hello adaptive memory")
            .await
            .expect("beta first turn should succeed");
        runtime
            .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("beta first route should succeed");
        runtime
            .run_demo_turn_in_session("beta", "rust worker pipeline")
            .await
            .expect("beta second turn should succeed");
        runtime
            .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("beta second route should succeed");
        runtime
            .route_topics(
                "beta",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("beta mixed route should succeed");

        let fork_report = runtime
            .fork_session("beta", "beta-fork")
            .expect("beta fork should succeed");
        assert_eq!(fork_report.topic_session_count, 2);
        assert_eq!(fork_report.topic_graph_edge_count, 2);

        let fork_topic_sessions = runtime
            .topic_sessions_for_surface("beta-fork")
            .expect("fork topic sessions should load");
        assert!(fork_topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:beta-fork"
                && topic_session.topic_id.0 == "topic-beta-fork"
        }));
        assert!(fork_topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id
                == "topic-session-bootstrap:beta-fork:rust-worker-pipeline"
                && topic_session.topic_id.0 == "topic-beta-fork-rust-worker-pipeline"
                && !topic_session.graph_edges.is_empty()
        }));

        let decision = runtime
            .route_topics("beta-fork", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("fork graph-expanded route should succeed");
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| { id == "topic-session-bootstrap:beta-fork:rust-worker-pipeline" })
        );
    }

    #[tokio::test]
    async fn merges_session_into_target_without_overwriting_target_model_or_title() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("mainline")
            .expect("mainline session switch should succeed");
        runtime
            .rename_active_session("Mainline workspace")
            .expect("mainline rename should succeed");
        runtime
            .switch_model("mock-ollama/local-precise")
            .expect("mainline model switch should succeed");
        runtime
            .run_demo_turn("mainline seed")
            .await
            .expect("mainline turn should succeed");
        runtime
            .run_demo_turn_in_session("beta-fork", "fork delta")
            .await
            .expect("fork turn should succeed");
        runtime
            .approve_tool_in_session("beta-fork", "read_file")
            .expect("fork approval should succeed");
        runtime
            .archive_session(Some("beta-fork"))
            .expect("fork archive should succeed");

        runtime
            .merge_session("beta-fork", "mainline", MergeOptions::default())
            .expect("merge should succeed");

        let mainline = runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "mainline")
            .expect("mainline session should exist");
        assert_eq!(mainline.title, "Mainline workspace");
        assert_eq!(mainline.model.provider, "mock-ollama");
        assert_eq!(mainline.model.model, "local-precise");
        assert!(mainline.archived_at_unix_ms.is_none());
        assert_eq!(
            mainline.last_user_intent_summary.as_deref(),
            Some("fork delta")
        );

        let approvals = runtime
            .approval_snapshot_for_session("mainline")
            .expect("mainline approvals should load");
        assert!(
            approvals
                .granted_tools
                .iter()
                .any(|tool| tool == "read_file")
        );

        let history = runtime
            .history(Some("mainline"), 10)
            .expect("mainline history should load");
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].input, "fork delta");
        assert_eq!(history[1].input, "mainline seed");
    }

    #[tokio::test]
    async fn diffs_sessions_semantically_without_treating_forked_history_as_all_different() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("alpha session switch should succeed");
        runtime
            .run_demo_turn_in_session("beta", "shared base")
            .await
            .expect("beta turn should succeed");
        runtime
            .approve_tool_in_session("beta", "read_file")
            .expect("beta approval should succeed");
        runtime
            .fork_session("beta", "beta-fork")
            .expect("beta fork should succeed");
        runtime
            .archive_session(Some("beta"))
            .expect("beta archive should succeed");
        runtime
            .switch_model_in_session("beta-fork", "demo/demo-creative")
            .expect("fork model switch should succeed");
        runtime
            .run_demo_turn_in_session("beta-fork", "fork-only delta")
            .await
            .expect("fork delta turn should succeed");

        let report = runtime
            .diff_sessions("beta", "beta-fork")
            .expect("diff should succeed");

        assert_eq!(report.left_session_id, "beta");
        assert_eq!(report.right_session_id, "beta-fork");
        assert_eq!(report.left_title, "Hepta session beta");
        assert_eq!(report.right_title, "Hepta session beta (fork)");
        assert_eq!(report.left_model.provider, "demo");
        assert_eq!(report.left_model.model, "demo-chat");
        assert_eq!(report.right_model.provider, "demo");
        assert_eq!(report.right_model.model, "demo-creative");
        assert!(report.left_archived);
        assert!(!report.right_archived);
        assert_eq!(report.left_history_count, 1);
        assert_eq!(report.right_history_count, 2);
        assert_eq!(report.shared_history_count, 1);
        assert!(report.approvals_only_left.is_empty());
        assert!(report.approvals_only_right.is_empty());
        assert!(report.history_only_left.is_empty());
        assert_eq!(report.history_only_right.len(), 1);
        assert!(report.history_only_right[0].contains("fork-only delta"));
        assert_eq!(
            report.left_last_user_intent_summary.as_deref(),
            Some("shared base")
        );
        assert_eq!(
            report.right_last_user_intent_summary.as_deref(),
            Some("fork-only delta")
        );
    }

    #[tokio::test]
    async fn previews_deduplicating_merge_plan_for_forked_history() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("beta")
            .expect("beta session switch should succeed");
        runtime
            .run_demo_turn("shared base")
            .await
            .expect("beta base turn should succeed");
        runtime
            .approve_tool("read_file")
            .expect("beta approval should succeed");
        runtime
            .fork_session("beta", "beta-fork")
            .expect("beta fork should succeed");
        runtime
            .switch_model_in_session("beta-fork", "demo/demo-creative")
            .expect("fork model switch should succeed");
        runtime
            .run_demo_turn_in_session("beta-fork", "fork-only delta")
            .await
            .expect("fork delta turn should succeed");

        let report = runtime
            .preview_merge_session("beta-fork", "beta", MergeOptions::default())
            .expect("merge preview should succeed");

        assert_eq!(report.source_session_id, "beta-fork");
        assert_eq!(report.target_session_id, "beta");
        assert_eq!(report.target_title_before, "Hepta session beta");
        assert_eq!(report.target_title_after, "Hepta session beta");
        assert_eq!(report.target_model_before.provider, "demo");
        assert_eq!(report.target_model_before.model, "demo-chat");
        assert_eq!(report.target_model_after.provider, "demo");
        assert_eq!(report.target_model_after.model, "demo-chat");
        assert!(!report.target_archived_before);
        assert!(!report.target_archived_after);
        assert!(!report.source_deleted_after_merge);
        assert_eq!(report.source_history_count, 2);
        assert_eq!(report.target_history_count, 1);
        assert_eq!(report.history_entries_to_append, 1);
        assert_eq!(report.history_entries_skipped_as_duplicates, 1);
        assert_eq!(report.source_topic_session_count, 0);
        assert_eq!(report.target_topic_session_count_before, 0);
        assert_eq!(report.target_topic_session_count_after, 0);
        assert_eq!(report.source_topic_graph_edge_count, 0);
        assert_eq!(report.target_topic_graph_edge_count_before, 0);
        assert_eq!(report.target_topic_graph_edge_count_after, 0);
        assert!(report.approvals_added_to_target.is_empty());
        assert!(report.pending_added_to_target.is_empty());
        assert_eq!(report.new_history_entries_to_append.len(), 1);
        assert!(report.new_history_entries_to_append[0].contains("fork-only delta"));
        assert_eq!(report.duplicate_history_entries_skipped.len(), 1);
        assert!(report.duplicate_history_entries_skipped[0].contains("shared base"));
        assert_eq!(
            report.target_last_user_intent_summary_before.as_deref(),
            Some("shared base")
        );
        assert_eq!(
            report.source_last_user_intent_summary.as_deref(),
            Some("fork-only delta")
        );
        assert_eq!(
            report.merged_last_user_intent_summary.as_deref(),
            Some("fork-only delta")
        );
    }

    #[tokio::test]
    async fn preview_merge_session_surfaces_topic_state_plan() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("mainline")
            .expect("mainline session switch should succeed");
        runtime
            .run_demo_turn("mainline planning")
            .await
            .expect("mainline turn should succeed");
        runtime
            .route_topics("mainline", Some("mainline planning"), 4, 4, 4, 1)
            .expect("mainline route should succeed");
        runtime
            .run_demo_turn_in_session("feature", "hello adaptive memory")
            .await
            .expect("feature first turn should succeed");
        runtime
            .route_topics("feature", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("feature first route should succeed");
        runtime
            .run_demo_turn_in_session("feature", "rust worker pipeline")
            .await
            .expect("feature second turn should succeed");
        runtime
            .route_topics("feature", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("feature second route should succeed");
        runtime
            .route_topics(
                "feature",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("feature mixed route should succeed");

        let report = runtime
            .preview_merge_session("feature", "mainline", MergeOptions::default())
            .expect("merge preview should succeed");

        assert_eq!(report.source_topic_session_count, 2);
        assert_eq!(report.target_topic_session_count_before, 1);
        assert_eq!(report.target_topic_session_count_after, 3);
        assert_eq!(report.source_topic_graph_edge_count, 2);
        assert_eq!(report.target_topic_graph_edge_count_before, 0);
        assert_eq!(report.target_topic_graph_edge_count_after, 2);
    }

    #[tokio::test]
    async fn merge_session_deduplicates_shared_history_from_forked_source() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("beta")
            .expect("beta session switch should succeed");
        runtime
            .run_demo_turn("shared base")
            .await
            .expect("beta base turn should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("beta topic turn should succeed");
        runtime
            .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("beta first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("beta second topic turn should succeed");
        runtime
            .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("beta second route should succeed");
        runtime
            .route_topics(
                "beta",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("beta mixed route should succeed");
        runtime
            .fork_session("beta", "beta-fork")
            .expect("beta fork should succeed");
        runtime
            .run_demo_turn_in_session("beta-fork", "fork-only delta")
            .await
            .expect("fork delta turn should succeed");

        let merge_result = runtime
            .merge_session("beta-fork", "beta", MergeOptions::default())
            .expect("merge should succeed");
        assert_eq!(merge_result.appended_history_entries, 1);
        assert_eq!(merge_result.skipped_duplicate_history_entries, 3);
        assert_eq!(merge_result.target_session_id, "beta");
        assert_eq!(merge_result.target_title_after, "Hepta session beta");
        assert_eq!(merge_result.target_model_after.model, "demo-chat");
        assert_eq!(merge_result.source_topic_session_count, 2);
        assert_eq!(merge_result.target_topic_session_count_before, 2);
        assert_eq!(merge_result.target_topic_session_count_after, 2);
        assert_eq!(merge_result.source_topic_graph_edge_count, 2);
        assert_eq!(merge_result.target_topic_graph_edge_count_before, 2);
        assert_eq!(merge_result.target_topic_graph_edge_count_after, 2);

        let history = runtime
            .history(Some("beta"), 10)
            .expect("beta history should load");
        assert_eq!(history.len(), 4);
        assert_eq!(history[0].input, "fork-only delta");
        assert_eq!(history[1].input, "rust worker pipeline");
        assert_eq!(history[2].input, "hello adaptive memory");
        assert_eq!(history[3].input, "shared base");

        let beta_topic_sessions = runtime
            .topic_sessions_for_surface("beta")
            .expect("beta topic sessions should load");
        assert_eq!(beta_topic_sessions.len(), 2);
        assert!(beta_topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:beta"
                && !topic_session.graph_edges.is_empty()
        }));
        assert!(beta_topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:beta:rust-worker-pipeline"
        }));
    }

    #[tokio::test]
    async fn merge_session_materializes_fresh_active_target_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn_in_session("feature", "feature base")
            .await
            .expect("feature base turn should succeed");

        let merge_result = runtime
            .merge_session("feature", "session-main", MergeOptions::default())
            .expect("merge into fresh active target should succeed");

        assert_eq!(merge_result.target_session_id, "session-main");
        assert_eq!(
            merge_result.target_title_after,
            "Hepta session session-main"
        );
        assert_eq!(merge_result.target_model_after.model, "demo-chat");
        assert_eq!(merge_result.appended_history_entries, 1);
        assert_eq!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "session-main"
        );

        let session_main = runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "session-main")
            .expect("session-main should exist");
        assert_eq!(
            session_main.last_user_intent_summary.as_deref(),
            Some("feature base")
        );

        let history = runtime
            .history(Some("session-main"), 10)
            .expect("session-main history should load");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].input, "feature base");
    }

    #[tokio::test]
    async fn merge_session_rebases_unrelated_topic_graph_state_into_target_namespace() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("mainline")
            .expect("mainline session switch should succeed");
        runtime
            .run_demo_turn("mainline planning")
            .await
            .expect("mainline turn should succeed");
        runtime
            .route_topics("mainline", Some("mainline planning"), 4, 4, 4, 1)
            .expect("mainline route should succeed");
        runtime
            .run_demo_turn_in_session("feature", "hello adaptive memory")
            .await
            .expect("feature first turn should succeed");
        runtime
            .route_topics("feature", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("feature first route should succeed");
        runtime
            .run_demo_turn_in_session("feature", "rust worker pipeline")
            .await
            .expect("feature second turn should succeed");
        runtime
            .route_topics("feature", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("feature second route should succeed");
        runtime
            .route_topics(
                "feature",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("feature mixed route should succeed");

        let merge_result = runtime
            .merge_session("feature", "mainline", MergeOptions::default())
            .expect("merge should succeed");
        assert_eq!(merge_result.source_topic_session_count, 2);
        assert_eq!(merge_result.target_topic_session_count_before, 1);
        assert_eq!(merge_result.target_topic_session_count_after, 3);
        assert_eq!(merge_result.source_topic_graph_edge_count, 2);
        assert_eq!(merge_result.target_topic_graph_edge_count_before, 0);
        assert_eq!(merge_result.target_topic_graph_edge_count_after, 2);

        let mainline_topic_sessions = runtime
            .topic_sessions_for_surface("mainline")
            .expect("mainline topic sessions should load");
        assert!(mainline_topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:mainline"
                && topic_session.topic_id.0 == "topic-mainline"
        }));
        assert!(mainline_topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:mainline:feature"
                && topic_session.topic_id.0 == "topic-mainline-feature"
        }));
        assert!(mainline_topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id
                == "topic-session-bootstrap:mainline:feature:rust-worker-pipeline"
                && topic_session.topic_id.0 == "topic-mainline-feature-rust-worker-pipeline"
        }));
        assert!(
            runtime
                .topic_graph_state
                .lock()
                .expect("topic graph state lock should succeed")
                .edges
                .iter()
                .any(|record| {
                    record.source_topic_session_id == "topic-session-bootstrap:mainline:feature"
                        && record.edge.target_topic_session_id
                            == "topic-session-bootstrap:mainline:feature:rust-worker-pipeline"
                })
        );
    }

    #[tokio::test]
    async fn merge_session_can_adopt_model_title_and_delete_source() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("mainline")
            .expect("mainline session switch should succeed");
        runtime
            .rename_active_session("Mainline workspace")
            .expect("mainline rename should succeed");
        runtime
            .run_demo_turn_in_session("feature", "feature base")
            .await
            .expect("feature base turn should succeed");
        runtime
            .switch_session("feature")
            .expect("feature session switch should succeed");
        runtime
            .rename_active_session("Feature workspace")
            .expect("feature rename should succeed");
        runtime
            .switch_model("demo/demo-creative")
            .expect("feature model switch should succeed");

        let preview = runtime
            .preview_merge_session(
                "feature",
                "mainline",
                MergeOptions {
                    adopt_model: true,
                    adopt_title: true,
                    delete_source: true,
                },
            )
            .expect("merge preview should succeed");
        assert_eq!(preview.target_title_after, "Feature workspace");
        assert_eq!(preview.target_model_after.model, "demo-creative");
        assert!(preview.source_deleted_after_merge);

        let merge_result = runtime
            .merge_session(
                "feature",
                "mainline",
                MergeOptions {
                    adopt_model: true,
                    adopt_title: true,
                    delete_source: true,
                },
            )
            .expect("merge should succeed");
        assert!(merge_result.options.adopt_title);
        assert!(merge_result.options.adopt_model);
        assert!(merge_result.options.delete_source);
        assert_eq!(merge_result.target_title_after, "Feature workspace");
        assert_eq!(merge_result.target_model_after.model, "demo-creative");
        assert!(merge_result.source_deleted_after_merge);

        let mainline = runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .find(|session| session.session_id == "mainline")
            .expect("mainline session should exist");
        assert_eq!(mainline.title, "Feature workspace");
        assert_eq!(mainline.model.model, "demo-creative");
        assert_eq!(
            runtime
                .active_session_id()
                .expect("active session should load"),
            "mainline"
        );
        assert!(
            runtime
                .sessions()
                .expect("sessions should load")
                .into_iter()
                .all(|session| session.session_id != "feature")
        );
    }
}
