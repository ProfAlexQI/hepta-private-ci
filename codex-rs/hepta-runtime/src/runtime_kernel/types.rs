pub struct RuntimeKernel {
    providers: ProviderRegistry,
    tools: ToolRegistry,
    memory: InMemoryStore,
    policy: ConfigurablePolicyEngine,
    approval_state: Arc<Mutex<ApprovalState>>,
    context_revision_state: Arc<Mutex<ContextRevisionState>>,
    execution_lease_registry: Arc<Mutex<ExecutionLeaseRegistry>>,
    execution_outcome_state: Arc<Mutex<ExecutionOutcomeState>>,
    outcome_sink: runtime_kernel::outcome_sink::SharedOutcomeReceiptSink,
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
    pub execution_receipt: Option<RuntimeExecutionReceipt>,
    pub final_text: String,
    pub recalled_memories: usize,
    pub approval_required: Option<String>,
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeExecutionReceipt {
    pub attempt_id: String,
    pub durable_intent_recorded: bool,
    pub effect_plan_recorded: bool,
    pub provider_effect_ack_hash: Option<String>,
    pub terminal_receipt_id: String,
    pub terminal_receipt_hash: String,
    pub terminal_outcome_hash: String,
    pub terminal_evidence_hash: String,
    pub terminal_status: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct RuntimeNativeTurnContextRecallHandoff {
    pub provider_rollup: RuntimeContextRecallProviderRollup,
    pub selected_snippets_present: bool,
    pub selected_snippet_count: u32,
    pub messages: Vec<ModelMessage>,
}

impl std::fmt::Debug for RuntimeNativeTurnContextRecallHandoff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeNativeTurnContextRecallHandoff")
            .field("provider_rollup", &self.provider_rollup)
            .field("selected_snippets_present", &self.selected_snippets_present)
            .field("selected_snippet_count", &self.selected_snippet_count)
            .field("message_count", &self.messages.len())
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct RuntimeNativeTurnContextRecallRun {
    pub provider_rollup: RuntimeContextRecallProviderRollup,
    pub selected_snippets_present: bool,
    pub selected_snippet_count: u32,
    pub result: VerticalSliceResult,
}

impl std::fmt::Debug for RuntimeNativeTurnContextRecallRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeNativeTurnContextRecallRun")
            .field("provider_rollup", &self.provider_rollup)
            .field("selected_snippets_present", &self.selected_snippets_present)
            .field("selected_snippet_count", &self.selected_snippet_count)
            .field("session_id", &self.result.session_id)
            .field("active_model", &self.result.active_model)
            .field("invoked_tool", &self.result.invoked_tool)
            .field("approval_required", &self.result.approval_required)
            .field("blocked_reason", &self.result.blocked_reason)
            .finish()
    }
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
    /// Compatibility display name; never sufficient to authorize execution.
    pub tool_name: String,
    /// Human-readable policy reason.
    pub reason: String,
    /// Exact candidate binding for operator selection, if created by V2 safety.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidate_binding_hash: Option<String>,
    /// Exact canonical tool-name-and-arguments digest, without raw arguments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ApprovalSnapshot {
    /// Display-only compatibility grants; snapshots never restore authority.
    pub granted_tools: Vec<String>,
    /// Display projection of pending approvals; imported entries are non-authoritative.
    pub pending: Vec<PendingApproval>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ToolDescriptor {
    pub name: String,
    pub executor_provider: String,
    pub operation: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_plan_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_ack_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_file_identity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_file_identity: Option<String>,
    pub rollback_strategy: String,
    pub rollback_checkpoint_path: Option<String>,
    pub source_backup_path: Option<String>,
    pub rolled_back_at_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionEffectInspectionState {
    Unplanned,
    NotApplied,
    AppliedAcknowledged,
    AppliedUnacknowledged,
    InDoubt,
    Drifted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PendingExecutionEffectInspection {
    pub attempt_id: String,
    pub tool_name: String,
    pub state: ExecutionEffectInspectionState,
    pub target_path: Option<String>,
    pub expected_before_content_hash: Option<String>,
    pub expected_after_content_hash: Option<String>,
    pub observed_content_hash: Option<String>,
    pub effect_plan_hash: Option<String>,
    pub effect_ack_hash: Option<String>,
    pub detail: String,
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

#[derive(Debug)]
struct PreparedWriteTransaction {
    operation: String,
    requested_path: String,
    target_path: String,
    mode_requested: String,
    preview_only: bool,
    target_existed_before: bool,
    before_bytes: Option<Vec<u8>>,
    staged_after_bytes: Option<Vec<u8>>,
    sealed_target: SealedWriteTarget,
    _reservation: WriteTargetReservation,
}

#[derive(Debug)]
struct PreparedWriteReservationSet {
    transactions: Vec<PreparedWriteTransaction>,
}

impl PreparedWriteReservationSet {
    fn empty() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }
}

/// Non-cloneable, read-only proof of the exact bytes and filesystem identity
/// captured before kernel authorization.
///
/// This capability is intentionally independent from write reservations and
/// mutation receipts. The retained descriptors are used only to revalidate
/// namespace identity; providers consume `bytes` and never reopen `requested_path`.
#[derive(Debug)]
struct PreparedReadCapability {
    tool_name: String,
    argument_name: String,
    requested_path: String,
    resolved_path: PathBuf,
    anchor_path: PathBuf,
    relative_components: Vec<std::ffi::OsString>,
    anchor_identity: FileIdentity,
    parent_identity: FileIdentity,
    file_identity: FileIdentity,
    content_hash: String,
    bytes: Vec<u8>,
    anchor_directory: fs::File,
    retained_file: fs::File,
}

#[derive(Debug)]
struct SealedWriteCandidate {
    operation: String,
    requested_path: String,
    target_path: String,
    mode_requested: String,
    preview_only: bool,
    target_existed_before: bool,
    before_bytes: Option<Vec<u8>>,
    sealed_target: SealedWriteTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileIdentity {
    device: u64,
    inode: u64,
}

/// Filesystem identity used by the in-process registry and OS advisory-lock
/// key derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SealedWriteIdentity {
    canonical_namespace: PathBuf,
    existing_target: Option<FileIdentity>,
    anchor: FileIdentity,
    anchor_suffix: Vec<std::ffi::OsString>,
}

#[derive(Debug, Clone)]
struct ProcessWriteReservationEntry {
    lock: WriteTargetLock,
    identity: SealedWriteIdentity,
}

#[derive(Debug, Default)]
struct ProcessWriteReservationRegistry {
    active: Vec<ProcessWriteReservationEntry>,
}

/// Non-cloneable capability proving ownership of one process-global rollback
/// group reservation. Callers cannot reconstruct this witness from public
/// session, group, attempt, or lock-report strings.
#[derive(Debug)]
struct GroupRollbackReservation {
    token: String,
    session_id: String,
    group_id: String,
    attempt_id: String,
}

#[derive(Debug, Clone)]
struct ActiveGroupRollbackReservation {
    token: String,
    session_id: String,
    group_id: String,
    attempt_id: String,
    lease_expires_at_unix_ms: u64,
    cross_process_lease:
        Arc<runtime_kernel::cross_process_write_lock::CrossProcessWriteLease>,
}

/// Filesystem capability captured before authorization.
///
/// The open anchor directory is deliberately retained through dispatch. The
/// provider can therefore create/open the leaf relative to the authorized
/// directory identity instead of resolving an attacker-controlled path again.
#[derive(Debug)]
struct SealedWriteTarget {
    workspace_root: PathBuf,
    canonical_path: PathBuf,
    canonical_anchor: PathBuf,
    missing_parent_components: Vec<std::ffi::OsString>,
    leaf_name: std::ffi::OsString,
    anchor_identity: FileIdentity,
    target_identity: Option<FileIdentity>,
    namespace_case_insensitive: bool,
    #[cfg(unix)]
    anchor_directory: fs::File,
}

#[derive(Debug)]
struct WriteTargetReservation {
    reservation_id: String,
    write_lock_state: Arc<Mutex<WriteLockState>>,
    process_reservation_id: Option<String>,
    _cross_process_lease:
        Option<runtime_kernel::cross_process_write_lock::CrossProcessWriteLease>,
}

impl Drop for WriteTargetReservation {
    fn drop(&mut self) {
        if let Some(reservation_id) = self.process_reservation_id.as_deref() {
            let mut registry = match process_write_reservation_registry().lock() {
                Ok(registry) => registry,
                Err(poisoned) => poisoned.into_inner(),
            };
            registry
                .active
                .retain(|entry| entry.lock.owner_id != reservation_id);
        }
        let mut state = match self.write_lock_state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        state
            .active_target_reservations
            .retain(|lock| lock.owner_id != self.reservation_id);
    }
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
    active_target_reservations: Vec<WriteTargetLock>,
    active_group_rollback_reservations: Vec<ActiveGroupRollbackReservation>,
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
    execution_receipt: RuntimeExecutionReceipt,
    tool_message: String,
}

#[derive(Debug, Clone)]
struct RuntimeToolTimeout {
    tool_name: String,
    tool_output_json: Option<String>,
    execution_receipt: RuntimeExecutionReceipt,
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

#[derive(Debug, Default, Clone)]
struct ConfigurablePolicyEngine {
    custom_rules: Arc<Mutex<Vec<PolicyRule>>>,
}
