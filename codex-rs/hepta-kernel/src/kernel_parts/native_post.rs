pub const HEPTA_KERNEL_CONTRACT: &str = "hepta-kernel-v1";
pub const HEPTA_KERNEL_OWNER: &str = "hepta-kernel";
pub const CODEX_ENGINE_ID: &str = "codex-engine";
pub const CODEX_TOOL_MENTION_SIGIL: char = '$';
pub const CODEX_PLUGIN_MENTION_SIGIL: char = '@';
pub const CODEX_AGENTS_MD_FILENAME: &str = "AGENTS.md";
pub const HEPTA_KERNEL_TELEGRAM_RUNNER_KIND: &str = "hepta_kernel_session_runner";
pub const HEPTA_KERNEL_TELEGRAM_RUNNER_STRATEGY: &str =
    "gated in-process Hepta kernel turn runner with Codex as an internal execution engine";
pub const HEPTA_KERNEL_TELEGRAM_DRAIN_ONCE_STAGES: &[&str] = &[
    "receive_getUpdates",
    "duplicate_suppression",
    "model_turn",
    "sendMessage",
    "cursor_commit",
];
pub const HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES: &str =
    "[\"message\",\"edited_message\",\"callback_query\",\"message_reaction\"]";
pub const DEFAULT_TELEGRAM_MLX_BASE_URL: &str = "http://127.0.0.1:11436/v1";
pub const DEFAULT_TELEGRAM_MLX_MAX_TOKENS: u64 = 512;
pub const MAX_TELEGRAM_MLX_MAX_TOKENS: u64 = 4096;
pub const DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS: u64 = 120_000;
pub const MAX_TELEGRAM_MODEL_TIMEOUT_MS: u64 = 600_000;
pub const MIN_TELEGRAM_MODEL_TIMEOUT_MS: u64 = 1_000;
pub const MLX_LOCAL_CHAT_COMPLETIONS_RUNNER_KIND: &str = "mlx_local_chat_completions";
pub const HEPTA_IN_PROCESS_EXEC_RUNNER_KIND: &str = "hepta_in_process_exec_runner";
pub const HEPTA_EXEC_CHILD_RUNNER_KIND: &str = "hepta_exec_child_runner";
pub const HEPTA_KERNEL_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE: &str =
    "本地模型这次响应超时或失败了。我已先收下这条消息，避免反复重试；请稍后再发一条继续。";
pub const HEPTA_KERNEL_TELEGRAM_DELIVERY_STORE_IDENTIFIER: &str = "/store/delivery";
pub const HEPTA_KERNEL_TELEGRAM_DELIVERY_MAX_RETRIES: u32 = 5;
pub const HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH: &str =
    ".hepta/telegram/ingress-drain-cursor.json";
pub const HEPTA_KERNEL_TELEGRAM_CURSOR_SCHEMA: &str = "hepta.telegram.cursor.v1";
pub const HEPTA_KERNEL_TELEGRAM_RECEIVE_ONCE_NEXT_MIGRATION_SLICE: &str = "manual receive is a diagnostic read path; use drain-once or the armed poll loop for model, send, and cursor side effects";
pub const DEFAULT_TELEGRAM_SOAK_MIN_POLLS: u64 = 3;
pub const MAX_TELEGRAM_SOAK_MIN_POLLS: u64 = 10_000;
pub const DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION: u64 = 8;
pub const MAX_TELEGRAM_SOAK_MAX_ATTENTION: u64 = 1_000;
pub const DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS: u64 = 120_000;
pub const MAX_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS: u64 = 3_600_000;
pub const MIN_TELEGRAM_POLL_LOOP_INTERVAL_MS: u64 = 500;
pub const MAX_TELEGRAM_POLL_LOOP_INTERVAL_MS: u64 = 60_000;
pub const DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS: u64 = 4_000;
pub const MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS: u64 = 30_000;
pub const DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS: u64 = 1;
pub const MAX_TELEGRAM_READ_MAX_ATTEMPTS: u64 = 5;
pub const DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS: u64 = 500;
pub const MAX_TELEGRAM_READ_RETRY_BACKOFF_MS: u64 = 30_000;
pub const MAX_TELEGRAM_SEND_MIN_INTERVAL_MS: u64 = 60_000;
pub const DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS: u64 = 1;
pub const MAX_TELEGRAM_SEND_MAX_ATTEMPTS: u64 = 5;
pub const DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS: u64 = 700;
pub const MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS: u64 = 30_000;
pub const HEPTA_KERNEL_NATIVE_POST_MAX_BODY_BYTES: usize = 64 * 1024;
pub const HEPTA_KERNEL_NATIVE_POST_REAL_HANDLERS_ENV: &str = "HEPTA_NATIVE_POST_REAL_HANDLERS";
pub const HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_APPROVAL_ENV: &str =
    "HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED";
pub const HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV: &str =
    "HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE";
pub const HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_PLAN_KINDS: &[&str] = &[];
pub const HEPTA_KERNEL_NATIVE_POST_COMPATIBILITY_HARNESS_PLAN_KINDS: &[&str] =
    &["approval_apply", "task_publish", "chat_send"];
pub const HEPTA_KERNEL_NATIVE_POST_EXECUTION_READINESS_ENDPOINT: &str =
    "/api/native-post-execution-readiness";
pub const HEPTA_KERNEL_NATIVE_POST_ACTIVATION_PLAN_ENDPOINT: &str =
    "/api/native-post-activation-plan";
pub const HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORES_ENDPOINT: &str =
    "/api/native-post-execution-stores";
pub const HEPTA_KERNEL_NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT: &str =
    "/api/native-post-rollout-evidence";
pub const HEPTA_KERNEL_NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT: &str =
    "/api/native-post-gray-release-evidence";
pub const HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORE_DIR_ENV: &str =
    "HEPTA_NATIVE_POST_EXECUTION_STORE_DIR";
pub const DEFAULT_HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORE_DIR: &str =
    ".hepta/native-post-execution";
pub const HEPTA_KERNEL_NATIVE_POST_STORE_MAX_BYTES_ENV: &str = "HEPTA_NATIVE_POST_STORE_MAX_BYTES";
pub const HEPTA_KERNEL_NATIVE_POST_STORE_MAX_LINES_ENV: &str = "HEPTA_NATIVE_POST_STORE_MAX_LINES";
pub const HEPTA_KERNEL_NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV: &str =
    "HEPTA_NATIVE_POST_RATE_LIMIT_WINDOW_MS";
pub const DEFAULT_HEPTA_KERNEL_NATIVE_POST_RATE_LIMIT_WINDOW_MS: u64 = 1_000;
pub const DEFAULT_HEPTA_KERNEL_NATIVE_POST_STORE_MAX_BYTES: u64 = 10 * 1024 * 1024;
pub const DEFAULT_HEPTA_KERNEL_NATIVE_POST_STORE_MAX_LINES: u64 = 100_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeptaKernelNativePostExecutionStoreLimits {
    pub max_store_bytes: u64,
    pub max_store_lines: u64,
    pub rate_limit_window_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeptaKernelNativePostPlanRouteSpec {
    pub pattern: &'static str,
    pub prefix: Option<&'static str>,
    pub exact_path: Option<&'static str>,
    pub source_command: &'static str,
    pub capability: &'static str,
    pub plan_kind: &'static str,
    pub compatibility_mode: &'static str,
    pub dry_run_only: bool,
    pub confirmation_required_for_real_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostBodySchema {
    pub schema_id: &'static str,
    pub content_type: &'static str,
    pub body_required_for_real_handler: bool,
    pub required_fields: Vec<&'static str>,
    pub optional_fields: Vec<&'static str>,
    pub body_read_during_plan: bool,
    pub raw_body_exposed: bool,
    pub raw_field_values_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostBodyAdmission {
    pub admission_status: &'static str,
    pub body_received: bool,
    pub request_body_read: bool,
    pub request_body_redacted: bool,
    pub body_size_bytes: usize,
    pub max_body_bytes: usize,
    pub body_size_within_limit: bool,
    pub json_parse_attempted: bool,
    pub json_parse_ok: Option<bool>,
    pub json_object_present: bool,
    pub required_fields_present: bool,
    pub missing_required_fields: Vec<&'static str>,
    pub optional_field_count_present: usize,
    pub confirm_field_present: bool,
    pub confirm_field_truthy: bool,
    pub dry_run_field_present: bool,
    pub dry_run_first_satisfied: bool,
    pub idempotency_key_required: bool,
    pub idempotency_key_present: bool,
    pub idempotency_key_fingerprint: Option<String>,
    pub ready_for_real_handler_input: bool,
    pub raw_body_exposed: bool,
    pub raw_field_values_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostConfirmationContract {
    pub current_plan_requires_confirmation: bool,
    pub real_mutation_requires_confirmation: bool,
    pub accepted_confirmation_field: Option<&'static str>,
    pub operator_approval_required: bool,
    pub confirmation_mechanism: &'static str,
    pub raw_confirmation_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostRollbackContract {
    pub current_plan_noop: bool,
    pub state_written_by_plan: bool,
    pub current_plan_rollback_strategy: &'static str,
    pub real_handler_requires_rollback_contract: bool,
    pub destructive_without_rollback: bool,
    pub rollback_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostIdempotencyEvidence {
    pub required: bool,
    pub key_present: bool,
    pub key_redacted: bool,
    pub key_fingerprint: Option<String>,
    pub key_shape_valid: bool,
    pub lookup_required_before_real_handler: bool,
    pub duplicate_suppression_required: bool,
    pub durable_store_required: bool,
    pub current_plan_lookup_performed: bool,
    pub current_plan_store_written: bool,
    pub raw_key_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostAuditEventContract {
    pub required: bool,
    pub schema_id: &'static str,
    pub event_kind: &'static str,
    pub body_schema_id: &'static str,
    pub route_pattern_recorded: bool,
    pub capability_recorded: bool,
    pub body_admission_status_recorded: bool,
    pub idempotency_evidence_recorded: bool,
    pub rollback_contract_recorded: bool,
    pub operator_approval_recorded: bool,
    pub ready_for_real_handler: bool,
    pub current_plan_emits_audit_event: bool,
    pub current_plan_persists_audit_event: bool,
    pub raw_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_parameter_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionAdmission {
    pub admission_status: &'static str,
    pub current_plan_executes_real_handler: bool,
    pub real_handler_currently_enabled: bool,
    pub real_handler_implemented: bool,
    pub allowlisted_for_real_handler: bool,
    pub enablement_gate_env: &'static str,
    pub enablement_gate_enabled: bool,
    pub operator_approval_env: &'static str,
    pub operator_approval_enabled: bool,
    pub handler_scope_env: &'static str,
    pub handler_scope: Option<String>,
    pub handler_scope_configured: bool,
    pub handler_scope_required: bool,
    pub handler_scope_matches: bool,
    pub request_body_admission_status: &'static str,
    pub request_body_ready_for_real_handler: bool,
    pub requires_body_schema: bool,
    pub requires_confirmation_contract: bool,
    pub requires_rollback_contract: bool,
    pub requires_idempotency_key: bool,
    pub idempotency_evidence_ready: bool,
    pub requires_audit_event: bool,
    pub audit_event_contract_ready: bool,
    pub requires_rate_limit: bool,
    pub requires_dry_run_first: bool,
    pub external_side_effects_possible: bool,
    pub blocked_reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionReadinessResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub endpoint: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub compatibility_mode: &'static str,
    pub side_effect_free: bool,
    pub post_route_count: usize,
    pub real_handler_candidate_count: usize,
    pub plan_only_route_count: usize,
    pub evidence_contract_route_count: usize,
    pub all_evidence_contracts_ready: bool,
    pub real_handler_implemented_count: usize,
    pub real_handler_ready_count: usize,
    pub real_handler_gate_env: &'static str,
    pub real_handler_gate_enabled: bool,
    pub real_handler_scope_env: &'static str,
    pub real_handler_scope: Option<String>,
    pub real_handler_scope_configured: bool,
    pub single_handler_scope_ready: bool,
    pub selected_handler_count: usize,
    pub selected_handler_kinds: Vec<&'static str>,
    pub all_real_handlers_blocked: bool,
    pub routes: Vec<HeptaKernelNativePostExecutionReadinessRoute>,
    pub action_dispatched: bool,
    pub command_executed: bool,
    pub approval_applied: bool,
    pub task_published: bool,
    pub chat_mutated: bool,
    pub raw_request_body_exposed: bool,
    pub raw_parameter_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
    pub external_side_effects: bool,
    pub gateway_mutation_performed: bool,
    pub telegram_read_performed: bool,
    pub model_invoked: bool,
    pub message_sent: bool,
    pub cursor_written: bool,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionReadinessRoute {
    pub pattern: &'static str,
    pub capability: &'static str,
    pub plan_kind: &'static str,
    pub compatibility_mode: &'static str,
    pub dry_run_only: bool,
    pub allowlisted_for_real_handler: bool,
    pub body_schema_id: &'static str,
    pub body_required_for_real_handler: bool,
    pub body_schema_ready: bool,
    pub confirmation_contract_ready: bool,
    pub rollback_contract_ready: bool,
    pub idempotency_evidence_contract_ready: bool,
    pub audit_event_contract_ready: bool,
    pub rate_limit_contract_ready: bool,
    pub execution_evidence_contract_ready: bool,
    pub ready_for_real_handler_wiring: bool,
    pub current_plan_executes_real_handler: bool,
    pub real_handler_implemented: bool,
    pub blocked_reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostActivationPlanResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub endpoint: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub compatibility_mode: &'static str,
    pub side_effect_free: bool,
    pub activation_preflight_ready: bool,
    pub activation_currently_enabled: bool,
    pub activation_blocked_reason: &'static str,
    pub handler_candidate_count: usize,
    pub handler_implemented_count: usize,
    pub all_handlers_implemented: bool,
    pub handler_scope_env: &'static str,
    pub handler_scope: Option<String>,
    pub handler_scope_configured: bool,
    pub single_handler_scope_ready: bool,
    pub selected_handler_count: usize,
    pub selected_handler_kinds: Vec<&'static str>,
    pub execution_evidence_ready: bool,
    pub store_contracts_ready: bool,
    pub store_jsonl_valid: bool,
    pub store_capacity_ok: bool,
    pub required_gates: Vec<HeptaKernelNativePostActivationGate>,
    pub rollback_ready: bool,
    pub rollback_anchor_required: bool,
    pub rollback_store_kind: &'static str,
    pub rollback_store_file: &'static str,
    pub rollback_schema_id: &'static str,
    pub rollback_actions: Vec<&'static str>,
    pub dry_run_only: bool,
    pub real_mutation_performed: bool,
    pub store_write_attempted: bool,
    pub approval_applied: bool,
    pub task_published: bool,
    pub chat_mutated: bool,
    pub external_side_effects: bool,
    pub gateway_mutation_performed: bool,
    pub telegram_read_performed: bool,
    pub model_invoked: bool,
    pub message_sent: bool,
    pub cursor_written: bool,
    pub raw_request_body_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostActivationGate {
    pub env: &'static str,
    pub enabled: bool,
    pub required_for_activation: bool,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionStoreRecord {
    pub schema_id: &'static str,
    pub recorded_at_unix_ms: u64,
    pub route_pattern: &'static str,
    pub capability: &'static str,
    pub plan_kind: &'static str,
    pub body_schema_id: &'static str,
    pub body_admission_status: &'static str,
    pub idempotency_key_required: bool,
    pub idempotency_key_present: bool,
    pub idempotency_key_redacted: bool,
    pub idempotency_key_fingerprint: Option<String>,
    pub duplicate_suppression_required: bool,
    pub audit_event_schema_id: &'static str,
    pub audit_event_ready_for_real_handler: bool,
    pub rollback_strategy: &'static str,
    pub rate_limit_bucket: &'static str,
    pub current_plan_executes_real_handler: bool,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionStoreWriteReport {
    pub status: &'static str,
    pub root: String,
    pub written_file_count: usize,
    pub written_files: Vec<String>,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
}

pub fn hepta_kernel_native_post_execution_store_write_report(
    root: String,
    written_files: Vec<String>,
) -> HeptaKernelNativePostExecutionStoreWriteReport {
    HeptaKernelNativePostExecutionStoreWriteReport {
        status: "written",
        root,
        written_file_count: written_files.len(),
        written_files,
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
    }
}

pub fn hepta_kernel_native_post_execution_store_record_json_line(
    record: &HeptaKernelNativePostExecutionStoreRecord,
) -> Result<String, String> {
    serde_json::to_string(record)
        .map_err(|error| format!("failed to serialize native POST execution record: {error}"))
}

pub fn hepta_kernel_native_post_execution_store_record_projected_append_bytes(
    record: &HeptaKernelNativePostExecutionStoreRecord,
) -> Result<u64, String> {
    hepta_kernel_native_post_execution_store_record_json_line(record)
        .map(|line| line.len() as u64 + 1)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostRealHandlerHarness {
    pub status: &'static str,
    pub handler_kind: &'static str,
    pub dry_run_only: bool,
    pub handler_implemented: bool,
    pub dual_gate_satisfied: bool,
    pub enablement_gate_env: &'static str,
    pub enablement_gate_enabled: bool,
    pub operator_approval_env: &'static str,
    pub operator_approval_enabled: bool,
    pub handler_scope_env: &'static str,
    pub handler_scope: Option<String>,
    pub handler_scope_configured: bool,
    pub handler_scope_required: bool,
    pub handler_scope_matches: bool,
    pub duplicate_check_performed: bool,
    pub duplicate_found: bool,
    pub duplicate_suppressed: bool,
    pub duplicate_check_error: Option<&'static str>,
    pub rate_limit_check_performed: bool,
    pub rate_limited: bool,
    pub rate_limit_suppressed: bool,
    pub rate_limit_window_ms: u64,
    pub rate_limit_check_error: Option<&'static str>,
    pub capacity_check_performed: bool,
    pub store_capacity_ok: bool,
    pub store_capacity_check_error: Option<&'static str>,
    pub store_write_attempted: bool,
    pub store_write_succeeded: bool,
    pub store_write_report: Option<HeptaKernelNativePostExecutionStoreWriteReport>,
    pub store_write_error: Option<&'static str>,
    pub task_published: bool,
    pub external_side_effects: bool,
    pub gateway_mutation_performed: bool,
    pub telegram_read_performed: bool,
    pub model_invoked: bool,
    pub message_sent: bool,
    pub cursor_written: bool,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostRealHandlerObservation {
    pub duplicate_check_performed: bool,
    pub duplicate_found: bool,
    pub duplicate_check_error: Option<&'static str>,
    pub rate_limit_check_performed: bool,
    pub rate_limited: bool,
    pub rate_limit_window_ms: u64,
    pub rate_limit_check_error: Option<&'static str>,
    pub capacity_check_performed: bool,
    pub store_capacity_ok: bool,
    pub store_capacity_check_error: Option<&'static str>,
    pub store_write_attempted: bool,
    pub store_write_succeeded: bool,
    pub store_write_report: Option<HeptaKernelNativePostExecutionStoreWriteReport>,
    pub store_write_error: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostStoreEffectProjection {
    pub idempotency_evidence: HeptaKernelNativePostIdempotencyEvidence,
    pub audit_event_contract: HeptaKernelNativePostAuditEventContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostPlanResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub method: &'static str,
    pub pattern: &'static str,
    pub source_command: &'static str,
    pub capability: &'static str,
    pub native_route: bool,
    pub compatibility_mode: &'static str,
    pub side_effect_free: bool,
    pub plan_kind: &'static str,
    pub dry_run_only: bool,
    pub confirmation_required_for_real_mutation: bool,
    pub parameter_present: bool,
    pub parameter_redacted: bool,
    pub parameter_length: Option<usize>,
    pub request_body_read: bool,
    pub request_body_redacted: bool,
    pub body_schema_ready: bool,
    pub body_admission_ready: bool,
    pub confirmation_contract_ready: bool,
    pub rollback_contract_ready: bool,
    pub idempotency_evidence_ready: bool,
    pub audit_event_contract_ready: bool,
    pub execution_admission_ready: bool,
    pub body_schema: HeptaKernelNativePostBodySchema,
    pub body_admission: HeptaKernelNativePostBodyAdmission,
    pub confirmation_contract: HeptaKernelNativePostConfirmationContract,
    pub rollback_contract: HeptaKernelNativePostRollbackContract,
    pub idempotency_evidence: HeptaKernelNativePostIdempotencyEvidence,
    pub audit_event_contract: HeptaKernelNativePostAuditEventContract,
    pub execution_admission: HeptaKernelNativePostExecutionAdmission,
    pub real_handler_harness_ready: bool,
    pub real_handler_harness: HeptaKernelNativePostRealHandlerHarness,
    pub action_dispatched: bool,
    pub command_executed: bool,
    pub approval_applied: bool,
    pub task_published: bool,
    pub chat_mutated: bool,
    pub raw_request_body_exposed: bool,
    pub raw_parameter_exposed: bool,
    pub raw_token_exposed: bool,
    pub raw_transcript_exposed: bool,
    pub model_invoked: bool,
    pub external_side_effects: bool,
    pub gateway_mutation_performed: bool,
    pub telegram_read_performed: bool,
    pub message_sent: bool,
    pub cursor_written: bool,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionStoreFileSpec {
    pub store_kind: &'static str,
    pub schema_id: &'static str,
    pub filename: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionStoreFileStatus {
    pub store_kind: &'static str,
    pub schema_id: &'static str,
    pub filename: &'static str,
    pub path: String,
    pub exists: bool,
    pub bytes: u64,
    pub max_bytes: u64,
    pub bytes_within_limit: bool,
    pub append_only: bool,
    pub jsonl: bool,
    pub jsonl_readable: bool,
    pub jsonl_valid: bool,
    pub line_count: u64,
    pub max_lines: u64,
    pub line_count_within_limit: bool,
    pub valid_json_line_count: u64,
    pub invalid_json_line_count: u64,
    pub raw_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionStoreJsonlHealth {
    pub jsonl_readable: bool,
    pub line_count: u64,
    pub valid_json_line_count: u64,
    pub invalid_json_line_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelNativePostExecutionStoreFileObservation {
    pub path: String,
    pub exists: bool,
    pub bytes: u64,
    pub max_bytes: u64,
    pub max_lines: u64,
    pub jsonl_observation: HeptaKernelNativePostStoreReadObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelNativePostStoreReadObservation {
    pub content: Option<String>,
    pub missing: bool,
    pub read_failed: bool,
}

pub fn hepta_kernel_native_post_execution_store_specs()
-> &'static [HeptaKernelNativePostExecutionStoreFileSpec] {
    &[
        HeptaKernelNativePostExecutionStoreFileSpec {
            store_kind: "idempotency",
            schema_id: "hepta.post.idempotency_entry.v1",
            filename: "idempotency.jsonl",
        },
        HeptaKernelNativePostExecutionStoreFileSpec {
            store_kind: "audit",
            schema_id: "hepta.post.execution_audit.v1",
            filename: "audit.jsonl",
        },
        HeptaKernelNativePostExecutionStoreFileSpec {
            store_kind: "rollback",
            schema_id: "hepta.post.rollback_anchor.v1",
            filename: "rollback.jsonl",
        },
        HeptaKernelNativePostExecutionStoreFileSpec {
            store_kind: "rate_limit",
            schema_id: "hepta.post.rate_limit_entry.v1",
            filename: "rate-limit.jsonl",
        },
    ]
}

pub fn hepta_kernel_native_post_execution_store_jsonl_health_missing()
-> HeptaKernelNativePostExecutionStoreJsonlHealth {
    HeptaKernelNativePostExecutionStoreJsonlHealth {
        jsonl_readable: true,
        line_count: 0,
        valid_json_line_count: 0,
        invalid_json_line_count: 0,
    }
}

pub fn hepta_kernel_native_post_execution_store_jsonl_health_read_failed()
-> HeptaKernelNativePostExecutionStoreJsonlHealth {
    HeptaKernelNativePostExecutionStoreJsonlHealth {
        jsonl_readable: false,
        line_count: 0,
        valid_json_line_count: 0,
        invalid_json_line_count: 0,
    }
}

pub fn hepta_kernel_native_post_execution_store_jsonl_health_from_content(
    content: &str,
) -> HeptaKernelNativePostExecutionStoreJsonlHealth {
    let mut line_count = 0_u64;
    let mut valid_json_line_count = 0_u64;
    let mut invalid_json_line_count = 0_u64;
    for line in content.lines() {
        line_count = line_count.saturating_add(1);
        if serde_json::from_str::<Value>(line).is_ok() {
            valid_json_line_count = valid_json_line_count.saturating_add(1);
        } else {
            invalid_json_line_count = invalid_json_line_count.saturating_add(1);
        }
    }
    HeptaKernelNativePostExecutionStoreJsonlHealth {
        jsonl_readable: true,
        line_count,
        valid_json_line_count,
        invalid_json_line_count,
    }
}

pub fn hepta_kernel_native_post_execution_store_jsonl_health_from_observation(
    observation: HeptaKernelNativePostStoreReadObservation,
) -> HeptaKernelNativePostExecutionStoreJsonlHealth {
    if let Some(content) = observation.content {
        return hepta_kernel_native_post_execution_store_jsonl_health_from_content(&content);
    }
    if observation.missing {
        return hepta_kernel_native_post_execution_store_jsonl_health_missing();
    }
    if observation.read_failed {
        return hepta_kernel_native_post_execution_store_jsonl_health_read_failed();
    }
    hepta_kernel_native_post_execution_store_jsonl_health_read_failed()
}

pub fn hepta_kernel_native_post_execution_store_file_status_report(
    spec: &HeptaKernelNativePostExecutionStoreFileSpec,
    path: String,
    exists: bool,
    bytes: u64,
    max_bytes: u64,
    max_lines: u64,
    jsonl_readable: bool,
    line_count: u64,
    valid_json_line_count: u64,
    invalid_json_line_count: u64,
) -> HeptaKernelNativePostExecutionStoreFileStatus {
    HeptaKernelNativePostExecutionStoreFileStatus {
        store_kind: spec.store_kind,
        schema_id: spec.schema_id,
        filename: spec.filename,
        path,
        exists,
        bytes,
        max_bytes,
        bytes_within_limit: bytes <= max_bytes,
        append_only: true,
        jsonl: true,
        jsonl_readable,
        jsonl_valid: jsonl_readable && invalid_json_line_count == 0,
        line_count,
        max_lines,
        line_count_within_limit: line_count <= max_lines,
        valid_json_line_count,
        invalid_json_line_count,
        raw_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
    }
}

pub fn hepta_kernel_native_post_execution_store_file_status_from_observation(
    spec: &HeptaKernelNativePostExecutionStoreFileSpec,
    observation: HeptaKernelNativePostExecutionStoreFileObservation,
) -> HeptaKernelNativePostExecutionStoreFileStatus {
    let jsonl_health = hepta_kernel_native_post_execution_store_jsonl_health_from_observation(
        observation.jsonl_observation,
    );
    hepta_kernel_native_post_execution_store_file_status_report(
        spec,
        observation.path,
        observation.exists,
        observation.bytes,
        observation.max_bytes,
        observation.max_lines,
        jsonl_health.jsonl_readable,
        jsonl_health.line_count,
        jsonl_health.valid_json_line_count,
        jsonl_health.invalid_json_line_count,
    )
}

pub fn hepta_kernel_native_post_execution_store_capacity_allows_append(
    stores: &[HeptaKernelNativePostExecutionStoreFileStatus],
    projected_line_bytes: u64,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> bool {
    stores.iter().all(|status| {
        status.jsonl_readable
            && status.jsonl_valid
            && status.bytes.saturating_add(projected_line_bytes) <= max_store_bytes
            && status.line_count.saturating_add(1) <= max_store_lines
    })
}

pub fn hepta_kernel_native_post_execution_store_jsonl_valid(
    stores: &[HeptaKernelNativePostExecutionStoreFileStatus],
) -> bool {
    stores
        .iter()
        .all(|file| file.jsonl_readable && file.invalid_json_line_count == 0)
}

pub fn hepta_kernel_native_post_execution_store_capacity_ok(
    stores: &[HeptaKernelNativePostExecutionStoreFileStatus],
) -> bool {
    stores
        .iter()
        .all(|file| file.bytes_within_limit && file.line_count_within_limit)
}

pub fn hepta_kernel_native_post_idempotency_duplicate_present_in_content(
    content: &str,
    key_fingerprint: Option<&str>,
) -> bool {
    let Some(key_fingerprint) = key_fingerprint else {
        return false;
    };
    content.lines().any(|line| line.contains(key_fingerprint))
}

pub fn hepta_kernel_native_post_idempotency_duplicate_present_from_observation(
    observation: HeptaKernelNativePostStoreReadObservation,
    key_fingerprint: Option<&str>,
) -> Result<bool, &'static str> {
    if key_fingerprint.is_none() {
        return Ok(false);
    }
    if let Some(content) = observation.content {
        return Ok(
            hepta_kernel_native_post_idempotency_duplicate_present_in_content(
                &content,
                key_fingerprint,
            ),
        );
    }
    if observation.missing {
        return Ok(false);
    }
    Err("native_post_idempotency_check_failed")
}

pub fn hepta_kernel_native_post_rate_limit_recent_present_in_content(
    content: &str,
    bucket: &str,
    window_ms: u64,
    now_ms: u64,
) -> bool {
    for line in content.lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        let Some(record_bucket) = value.get("rate_limit_bucket").and_then(Value::as_str) else {
            continue;
        };
        if record_bucket != bucket {
            continue;
        }
        let Some(recorded_at_ms) = value.get("recorded_at_unix_ms").and_then(Value::as_u64) else {
            continue;
        };
        if now_ms.saturating_sub(recorded_at_ms) <= window_ms {
            return true;
        }
    }
    false
}

pub fn hepta_kernel_native_post_rate_limit_recent_present_from_observation(
    observation: HeptaKernelNativePostStoreReadObservation,
    bucket: &str,
    window_ms: u64,
    now_ms: u64,
) -> Result<bool, &'static str> {
    if let Some(content) = observation.content {
        return Ok(
            hepta_kernel_native_post_rate_limit_recent_present_in_content(
                &content, bucket, window_ms, now_ms,
            ),
        );
    }
    if observation.missing {
        return Ok(false);
    }
    Err("native_post_rate_limit_check_failed")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostExecutionStoresResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub endpoint: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub compatibility_mode: &'static str,
    pub side_effect_free: bool,
    pub store_root_env: &'static str,
    pub store_root: String,
    pub root_exists: bool,
    pub root_is_dir: bool,
    pub store_file_count: usize,
    pub existing_file_count: usize,
    pub max_store_bytes_env: &'static str,
    pub max_store_bytes: u64,
    pub max_store_lines_env: &'static str,
    pub max_store_lines: u64,
    pub total_bytes: u64,
    pub store_jsonl_valid: bool,
    pub store_capacity_ok: bool,
    pub total_line_count: u64,
    pub valid_json_line_count: u64,
    pub invalid_json_line_count: u64,
    pub stores: Vec<HeptaKernelNativePostExecutionStoreFileStatus>,
    pub persistence_implementation_ready: bool,
    pub idempotency_store_ready: bool,
    pub audit_store_ready: bool,
    pub rollback_store_ready: bool,
    pub rate_limit_store_ready: bool,
    pub status_probe_creates_directory: bool,
    pub status_probe_writes_files: bool,
    pub current_plan_executes_real_handler: bool,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
    pub action_dispatched: bool,
    pub command_executed: bool,
    pub approval_applied: bool,
    pub task_published: bool,
    pub chat_mutated: bool,
    pub external_side_effects: bool,
    pub gateway_mutation_performed: bool,
    pub telegram_read_performed: bool,
    pub model_invoked: bool,
    pub message_sent: bool,
    pub cursor_written: bool,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostRolloutEvidencePlanKindCount {
    pub plan_kind: String,
    pub count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostRolloutEvidenceRecordSummary {
    pub recorded_at_unix_ms: Option<u64>,
    pub route_pattern: Option<String>,
    pub capability: Option<String>,
    pub plan_kind: Option<String>,
    pub body_schema_id: Option<String>,
    pub body_admission_status: Option<String>,
    pub rollback_strategy: Option<String>,
    pub rate_limit_bucket: Option<String>,
    pub current_plan_executes_real_handler: bool,
    pub idempotency_key_redacted: bool,
    pub idempotency_key_fingerprint_present: bool,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostRolloutEvidenceScan {
    pub jsonl_readable: bool,
    pub read_error: Option<&'static str>,
    pub line_count: u64,
    pub valid_json_line_count: u64,
    pub invalid_json_line_count: u64,
    pub record_count: u64,
    pub dry_run_record_count: u64,
    pub rollback_anchor_count: u64,
    pub plan_kind_counts: Vec<HeptaKernelNativePostRolloutEvidencePlanKindCount>,
    pub latest_record: Option<HeptaKernelNativePostRolloutEvidenceRecordSummary>,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelNativePostRolloutEvidenceFileObservation {
    pub content: Option<String>,
    pub missing: bool,
    pub read_failed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostRolloutEvidenceResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub endpoint: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub compatibility_mode: &'static str,
    pub side_effect_free: bool,
    pub store_root_env: &'static str,
    pub store_root: String,
    pub rollback_store_file: &'static str,
    pub store_jsonl_valid: bool,
    pub store_capacity_ok: bool,
    pub rollout_evidence_ready: bool,
    pub activation_scope_env: &'static str,
    pub activation_scope: Option<String>,
    pub single_handler_scope_ready: bool,
    pub selected_handler_count: usize,
    pub selected_handler_kinds: Vec<&'static str>,
    pub rollback_anchor_present: bool,
    pub dry_run_record_present: bool,
    pub record_count: u64,
    pub dry_run_record_count: u64,
    pub rollback_anchor_count: u64,
    pub line_count: u64,
    pub valid_json_line_count: u64,
    pub invalid_json_line_count: u64,
    pub jsonl_readable: bool,
    pub read_error: Option<&'static str>,
    pub plan_kind_counts: Vec<HeptaKernelNativePostRolloutEvidencePlanKindCount>,
    pub latest_record: Option<HeptaKernelNativePostRolloutEvidenceRecordSummary>,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
    pub real_mutation_performed: bool,
    pub approval_applied: bool,
    pub task_published: bool,
    pub chat_mutated: bool,
    pub external_side_effects: bool,
    pub gateway_mutation_performed: bool,
    pub telegram_read_performed: bool,
    pub model_invoked: bool,
    pub message_sent: bool,
    pub cursor_written: bool,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostSelectedHandlerRolloutEvidence {
    pub selected_handler_kind: Option<String>,
    pub record_count: u64,
    pub dry_run_record_count: u64,
    pub rollback_anchor_count: u64,
    pub dry_run_record_present: bool,
    pub rollback_anchor_present: bool,
    pub latest_record: Option<HeptaKernelNativePostRolloutEvidenceRecordSummary>,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelNativePostGrayReleaseEvidenceResponse {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub endpoint: &'static str,
    pub source_command: &'static str,
    pub native_route: bool,
    pub compatibility_mode: &'static str,
    pub side_effect_free: bool,
    pub activation_plan_endpoint: &'static str,
    pub rollout_evidence_endpoint: &'static str,
    pub store_root_env: &'static str,
    pub store_root: String,
    pub handler_scope_env: &'static str,
    pub handler_scope: Option<String>,
    pub selected_handler_count: usize,
    pub selected_handler_kinds: Vec<&'static str>,
    pub selected_handler_kind: Option<String>,
    pub single_handler_scope_ready: bool,
    pub real_handler_gate_env: &'static str,
    pub real_handler_gate_enabled: bool,
    pub operator_approval_env: &'static str,
    pub operator_approval_enabled: bool,
    pub activation_preflight_ready: bool,
    pub activation_currently_enabled: bool,
    pub store_jsonl_valid: bool,
    pub store_capacity_ok: bool,
    pub rollout_evidence_ready: bool,
    pub gray_release_evidence_ready: bool,
    pub selected_handler_evidence_ready: bool,
    pub gray_release_ready: bool,
    pub gray_release_phase: &'static str,
    pub selected_handler_evidence: HeptaKernelNativePostSelectedHandlerRolloutEvidence,
    pub rollback_actions: Vec<&'static str>,
    pub dry_run_only: bool,
    pub real_mutation_performed: bool,
    pub store_write_attempted: bool,
    pub approval_applied: bool,
    pub task_published: bool,
    pub chat_mutated: bool,
    pub external_side_effects: bool,
    pub gateway_mutation_performed: bool,
    pub telegram_read_performed: bool,
    pub model_invoked: bool,
    pub message_sent: bool,
    pub cursor_written: bool,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
    pub next_migration_slice: &'static str,
}

pub const HEPTA_KERNEL_NATIVE_POST_PLAN_ROUTE_SPECS: &[HeptaKernelNativePostPlanRouteSpec] = &[
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/actions/<action>",
        prefix: Some("/api/actions/"),
        exact_path: None,
        source_command: "/ui-action-plan <action> --dry-run --json",
        capability: "guarded-action-post",
        plan_kind: "ui_action",
        compatibility_mode: "native_action_post_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/commands/<id>",
        prefix: Some("/api/commands/"),
        exact_path: None,
        source_command: "/<allowlisted read-only command> --json",
        capability: "readonly-command-runner",
        plan_kind: "readonly_command",
        compatibility_mode: "native_readonly_command_plan",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/approvals/exec/apply",
        prefix: None,
        exact_path: Some("/api/approvals/exec/apply"),
        source_command: "/approvals exec apply --dry-run --json",
        capability: "exec-approvals-apply-bridge",
        plan_kind: "approval_apply",
        compatibility_mode: "native_approvals_exec_apply_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: true,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/tasks/plan",
        prefix: None,
        exact_path: Some("/api/tasks/plan"),
        source_command: "/tasks plan --dry-run --json",
        capability: "task-publisher-plan",
        plan_kind: "task_plan",
        compatibility_mode: "native_task_plan_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/tasks/publish",
        prefix: None,
        exact_path: Some("/api/tasks/publish"),
        source_command: "/tasks publish --confirm --json",
        capability: "task-publisher-publish",
        plan_kind: "task_publish",
        compatibility_mode: "native_task_publish_confirm_required",
        dry_run_only: false,
        confirmation_required_for_real_mutation: true,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/chat/register",
        prefix: None,
        exact_path: Some("/api/chat/register"),
        source_command: "/chat register --json",
        capability: "agent-chat-register",
        plan_kind: "chat_register",
        compatibility_mode: "native_chat_register_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/chat/archive",
        prefix: None,
        exact_path: Some("/api/chat/archive"),
        source_command: "/chat archive --json",
        capability: "agent-chat-archive",
        plan_kind: "chat_archive",
        compatibility_mode: "native_chat_archive_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/chat/unarchive",
        prefix: None,
        exact_path: Some("/api/chat/unarchive"),
        source_command: "/chat unarchive --json",
        capability: "agent-chat-unarchive",
        plan_kind: "chat_unarchive",
        compatibility_mode: "native_chat_unarchive_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/chat/delete",
        prefix: None,
        exact_path: Some("/api/chat/delete"),
        source_command: "/chat delete --json",
        capability: "agent-chat-delete",
        plan_kind: "chat_delete",
        compatibility_mode: "native_chat_delete_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/chat/plan",
        prefix: None,
        exact_path: Some("/api/chat/plan"),
        source_command: "/chat plan --json",
        capability: "agent-chat-plan",
        plan_kind: "chat_plan",
        compatibility_mode: "native_chat_plan_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/chat",
        prefix: None,
        exact_path: Some("/api/chat"),
        source_command: "/chat send --json",
        capability: "agent-chat-send",
        plan_kind: "chat_send",
        compatibility_mode: "native_chat_send_confirm_required",
        dry_run_only: false,
        confirmation_required_for_real_mutation: true,
    },
    HeptaKernelNativePostPlanRouteSpec {
        pattern: "/api/runtime/operator",
        prefix: None,
        exact_path: Some("/api/runtime/operator"),
        source_command: "/runtime/operator --dry-run --json",
        capability: "runtime-operator-plan",
        plan_kind: "runtime_operator",
        compatibility_mode: "native_runtime_operator_dry_run",
        dry_run_only: true,
        confirmation_required_for_real_mutation: false,
    },
];

pub fn hepta_kernel_native_post_plan_route_specs() -> &'static [HeptaKernelNativePostPlanRouteSpec]
{
    HEPTA_KERNEL_NATIVE_POST_PLAN_ROUTE_SPECS
}

pub fn hepta_kernel_native_post_plan_parameter<'a>(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    path: &'a str,
) -> Option<Option<&'a str>> {
    if let Some(prefix) = spec.prefix {
        return path
            .strip_prefix(prefix)
            .filter(|parameter| !parameter.is_empty())
            .map(Some);
    }
    spec.exact_path
        .filter(|exact_path| *exact_path == path)
        .map(|_| None)
}

pub fn hepta_kernel_native_post_plan_kind_has_real_handler(plan_kind: &str) -> bool {
    HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_PLAN_KINDS.contains(&plan_kind)
}

pub fn hepta_kernel_native_post_body_schema(
    plan_kind: &str,
    body_read_during_plan: bool,
) -> HeptaKernelNativePostBodySchema {
    let (schema_id, body_required_for_real_handler, required_fields, optional_fields) =
        match plan_kind {
            "ui_action" => (
                "hepta.post.ui_action.v1",
                false,
                vec![],
                vec!["action_payload", "dry_run", "confirm", "reason"],
            ),
            "readonly_command" => (
                "hepta.post.readonly_command.v1",
                false,
                vec![],
                vec!["command_args", "dry_run"],
            ),
            "approval_apply" => (
                "hepta.post.approval_apply.v1",
                true,
                vec!["approval_id", "confirm"],
                vec!["dry_run", "reason", "idempotency_key"],
            ),
            "task_plan" => (
                "hepta.post.task_plan.v1",
                false,
                vec![],
                vec!["task", "channel", "delivery", "dry_run"],
            ),
            "task_publish" => (
                "hepta.post.task_publish.v1",
                true,
                vec!["task", "confirm"],
                vec![
                    "delivery",
                    "timeout_seconds",
                    "rollback_hint",
                    "dry_run",
                    "idempotency_key",
                ],
            ),
            "chat_register" => (
                "hepta.post.chat_register.v1",
                true,
                vec!["chat_id"],
                vec!["label", "metadata"],
            ),
            "chat_archive" => (
                "hepta.post.chat_archive.v1",
                true,
                vec!["chat_id"],
                vec!["reason"],
            ),
            "chat_unarchive" => (
                "hepta.post.chat_unarchive.v1",
                true,
                vec!["chat_id"],
                vec!["reason"],
            ),
            "chat_delete" => (
                "hepta.post.chat_delete.v1",
                true,
                vec!["chat_id"],
                vec!["reason", "confirm"],
            ),
            "chat_plan" => (
                "hepta.post.chat_plan.v1",
                false,
                vec![],
                vec!["chat_id", "message", "dry_run"],
            ),
            "chat_send" => (
                "hepta.post.chat_send.v1",
                true,
                vec!["chat_id", "message", "confirm"],
                vec![
                    "thread_id",
                    "delivery",
                    "rollback_hint",
                    "dry_run",
                    "idempotency_key",
                ],
            ),
            _ => ("hepta.post.unknown.v1", false, vec![], vec!["dry_run"]),
        };

    HeptaKernelNativePostBodySchema {
        schema_id,
        content_type: "application/json",
        body_required_for_real_handler,
        required_fields,
        optional_fields,
        body_read_during_plan,
        raw_body_exposed: false,
        raw_field_values_exposed: false,
    }
}

pub fn hepta_kernel_native_post_body_admission(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    schema: &HeptaKernelNativePostBodySchema,
    request_body: Option<&str>,
) -> HeptaKernelNativePostBodyAdmission {
    let body_received = request_body
        .map(str::trim)
        .map(|body| !body.is_empty())
        .unwrap_or(false);
    let request_body_read = request_body.is_some();
    let body_size_bytes = request_body.map(str::len).unwrap_or(0);
    let body_size_within_limit = body_size_bytes <= HEPTA_KERNEL_NATIVE_POST_MAX_BODY_BYTES;
    let json_parse_attempted = body_received && body_size_within_limit;
    let parsed_body = if json_parse_attempted {
        request_body.and_then(|body| serde_json::from_str::<serde_json::Value>(body).ok())
    } else {
        None
    };
    let json_parse_ok = json_parse_attempted.then_some(parsed_body.is_some());
    let object = parsed_body.as_ref().and_then(serde_json::Value::as_object);
    let json_object_present = object.is_some();
    let missing_required_fields = if schema.body_required_for_real_handler || body_received {
        schema
            .required_fields
            .iter()
            .copied()
            .filter(|field| {
                object
                    .map(|object| !object.contains_key(*field))
                    .unwrap_or(true)
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let required_fields_present = missing_required_fields.is_empty();
    let optional_field_count_present = object
        .map(|object| {
            schema
                .optional_fields
                .iter()
                .filter(|field| object.contains_key(**field))
                .count()
        })
        .unwrap_or(0);
    let confirm_field = object.and_then(|object| object.get("confirm"));
    let confirm_field_present = confirm_field.is_some();
    let confirm_field_truthy = hepta_kernel_native_post_json_field_truthy(confirm_field);
    let dry_run_field = object.and_then(|object| object.get("dry_run"));
    let dry_run_field_present = dry_run_field.is_some();
    let dry_run_first_satisfied = !spec.confirmation_required_for_real_mutation
        || hepta_kernel_native_post_json_field_truthy(dry_run_field);
    let idempotency_key_required = spec.confirmation_required_for_real_mutation;
    let idempotency_key_value = object
        .and_then(|object| object.get("idempotency_key"))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let idempotency_key_present = idempotency_key_value.is_some();
    let idempotency_key_fingerprint =
        idempotency_key_value.map(hepta_kernel_native_post_redacted_fingerprint);

    let admission_status = if !body_received && schema.body_required_for_real_handler {
        "missing_body"
    } else if !body_size_within_limit {
        "body_too_large"
    } else if json_parse_attempted && json_parse_ok != Some(true) {
        "invalid_json"
    } else if body_received && !json_object_present {
        "body_not_json_object"
    } else if !required_fields_present {
        "missing_required_fields"
    } else if spec.confirmation_required_for_real_mutation && !confirm_field_truthy {
        "confirmation_missing"
    } else if idempotency_key_required && !idempotency_key_present {
        "idempotency_key_missing"
    } else if spec.confirmation_required_for_real_mutation && !dry_run_first_satisfied {
        "dry_run_first_required"
    } else if spec.confirmation_required_for_real_mutation {
        "ready_for_real_handler"
    } else if body_received {
        "validated_plan_input"
    } else {
        "not_required"
    };
    let ready_for_real_handler_input = admission_status == "ready_for_real_handler";

    HeptaKernelNativePostBodyAdmission {
        admission_status,
        body_received,
        request_body_read,
        request_body_redacted: true,
        body_size_bytes,
        max_body_bytes: HEPTA_KERNEL_NATIVE_POST_MAX_BODY_BYTES,
        body_size_within_limit,
        json_parse_attempted,
        json_parse_ok,
        json_object_present,
        required_fields_present,
        missing_required_fields,
        optional_field_count_present,
        confirm_field_present,
        confirm_field_truthy,
        dry_run_field_present,
        dry_run_first_satisfied,
        idempotency_key_required,
        idempotency_key_present,
        idempotency_key_fingerprint,
        ready_for_real_handler_input,
        raw_body_exposed: false,
        raw_field_values_exposed: false,
    }
}

pub fn hepta_kernel_native_post_redacted_fingerprint(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"hepta-native-post-idempotency-v1:");
    hasher.update(value.as_bytes());
    let digest = hasher.finalize();
    let mut encoded = String::with_capacity("sha256:".len() + digest.len() * 2);
    encoded.push_str("sha256:");
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut encoded, "{byte:02x}");
    }
    encoded
}

fn hepta_kernel_native_post_json_field_truthy(value: Option<&serde_json::Value>) -> bool {
    match value {
        Some(serde_json::Value::Bool(true)) => true,
        Some(serde_json::Value::String(value)) => {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        }
        Some(serde_json::Value::Number(value)) => value.as_i64() == Some(1),
        _ => false,
    }
}

pub fn hepta_kernel_native_post_confirmation_contract(
    spec: &HeptaKernelNativePostPlanRouteSpec,
) -> HeptaKernelNativePostConfirmationContract {
    HeptaKernelNativePostConfirmationContract {
        current_plan_requires_confirmation: false,
        real_mutation_requires_confirmation: spec.confirmation_required_for_real_mutation,
        accepted_confirmation_field: spec
            .confirmation_required_for_real_mutation
            .then_some("confirm"),
        operator_approval_required: spec.confirmation_required_for_real_mutation,
        confirmation_mechanism: if spec.confirmation_required_for_real_mutation {
            "explicit_confirm_field_plus_operator_approval"
        } else {
            "not_required_for_plan_only_route"
        },
        raw_confirmation_payload_exposed: false,
    }
}

pub fn hepta_kernel_native_post_rollback_contract() -> HeptaKernelNativePostRollbackContract {
    HeptaKernelNativePostRollbackContract {
        current_plan_noop: true,
        state_written_by_plan: false,
        current_plan_rollback_strategy: "noop_no_state_written",
        real_handler_requires_rollback_contract: true,
        destructive_without_rollback: false,
        rollback_payload_exposed: false,
    }
}

pub fn hepta_kernel_native_post_idempotency_evidence(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    body_admission: &HeptaKernelNativePostBodyAdmission,
) -> HeptaKernelNativePostIdempotencyEvidence {
    let required = spec.confirmation_required_for_real_mutation;
    HeptaKernelNativePostIdempotencyEvidence {
        required,
        key_present: body_admission.idempotency_key_present,
        key_redacted: body_admission.idempotency_key_present,
        key_fingerprint: body_admission.idempotency_key_fingerprint.clone(),
        key_shape_valid: !required || body_admission.idempotency_key_present,
        lookup_required_before_real_handler: required,
        duplicate_suppression_required: required,
        durable_store_required: required,
        current_plan_lookup_performed: false,
        current_plan_store_written: false,
        raw_key_exposed: false,
    }
}

pub fn hepta_kernel_native_post_audit_event_contract(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    body_schema: &HeptaKernelNativePostBodySchema,
    body_admission: &HeptaKernelNativePostBodyAdmission,
    idempotency_evidence: &HeptaKernelNativePostIdempotencyEvidence,
) -> HeptaKernelNativePostAuditEventContract {
    let required = spec.confirmation_required_for_real_mutation;
    HeptaKernelNativePostAuditEventContract {
        required,
        schema_id: "hepta.post.execution_audit.v1",
        event_kind: spec.plan_kind,
        body_schema_id: body_schema.schema_id,
        route_pattern_recorded: true,
        capability_recorded: true,
        body_admission_status_recorded: true,
        idempotency_evidence_recorded: required,
        rollback_contract_recorded: required,
        operator_approval_recorded: required,
        ready_for_real_handler: !required
            || (body_admission.ready_for_real_handler_input
                && idempotency_evidence.key_shape_valid),
        current_plan_emits_audit_event: false,
        current_plan_persists_audit_event: false,
        raw_body_exposed: false,
        raw_field_values_exposed: false,
        raw_parameter_exposed: false,
        raw_idempotency_key_exposed: false,
    }
}

pub fn hepta_kernel_native_post_execution_admission_with_scope(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    body_admission: &HeptaKernelNativePostBodyAdmission,
    idempotency_evidence: &HeptaKernelNativePostIdempotencyEvidence,
    audit_event_contract: &HeptaKernelNativePostAuditEventContract,
    enablement_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
) -> HeptaKernelNativePostExecutionAdmission {
    let allowlisted_for_real_handler = spec.confirmation_required_for_real_mutation;
    let real_handler_implemented =
        hepta_kernel_native_post_plan_kind_has_real_handler(spec.plan_kind);
    let handler_scope_configured = handler_scope
        .map(str::trim)
        .map(|scope| !scope.is_empty())
        .unwrap_or(false);
    let handler_scope_matches = !allowlisted_for_real_handler
        || hepta_kernel_native_post_real_handler_scope_matches(spec.plan_kind, handler_scope);
    let handler_scope_required = allowlisted_for_real_handler && real_handler_implemented;
    let request_body_ready_for_real_handler =
        !allowlisted_for_real_handler || body_admission.ready_for_real_handler_input;
    let execution_evidence_ready = !allowlisted_for_real_handler
        || (idempotency_evidence.key_shape_valid && audit_event_contract.ready_for_real_handler);
    let current_plan_executes_real_handler = allowlisted_for_real_handler
        && request_body_ready_for_real_handler
        && execution_evidence_ready
        && real_handler_implemented
        && enablement_gate_enabled
        && operator_approval_enabled
        && (!handler_scope_required || handler_scope_matches);

    HeptaKernelNativePostExecutionAdmission {
        admission_status: if current_plan_executes_real_handler {
            "harness_ready"
        } else {
            "blocked"
        },
        current_plan_executes_real_handler,
        real_handler_currently_enabled: enablement_gate_enabled,
        real_handler_implemented,
        allowlisted_for_real_handler,
        enablement_gate_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLERS_ENV,
        enablement_gate_enabled,
        operator_approval_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
        operator_approval_enabled,
        handler_scope_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
        handler_scope: handler_scope
            .map(str::trim)
            .filter(|scope| !scope.is_empty())
            .map(str::to_string),
        handler_scope_configured,
        handler_scope_required,
        handler_scope_matches,
        request_body_admission_status: body_admission.admission_status,
        request_body_ready_for_real_handler,
        requires_body_schema: allowlisted_for_real_handler,
        requires_confirmation_contract: allowlisted_for_real_handler,
        requires_rollback_contract: allowlisted_for_real_handler,
        requires_idempotency_key: allowlisted_for_real_handler,
        idempotency_evidence_ready: execution_evidence_ready,
        requires_audit_event: allowlisted_for_real_handler,
        audit_event_contract_ready: execution_evidence_ready,
        requires_rate_limit: allowlisted_for_real_handler,
        requires_dry_run_first: true,
        external_side_effects_possible: allowlisted_for_real_handler,
        blocked_reason: if allowlisted_for_real_handler && !request_body_ready_for_real_handler {
            "body_admission_not_ready"
        } else if allowlisted_for_real_handler && !execution_evidence_ready {
            "execution_evidence_not_ready"
        } else if allowlisted_for_real_handler && !real_handler_implemented {
            "real_handler_not_wired"
        } else if allowlisted_for_real_handler && !enablement_gate_enabled {
            "real_handler_gate_disabled"
        } else if allowlisted_for_real_handler && !operator_approval_enabled {
            "operator_approval_required"
        } else if allowlisted_for_real_handler && handler_scope_required && !handler_scope_matches {
            "handler_scope_not_selected"
        } else if allowlisted_for_real_handler {
            "real_handler_harness_dry_run_only"
        } else {
            "plan_only_route"
        },
    }
}

pub fn hepta_kernel_native_post_duplicate_check_required(
    execution_admission: &HeptaKernelNativePostExecutionAdmission,
    idempotency_evidence: &HeptaKernelNativePostIdempotencyEvidence,
) -> bool {
    execution_admission.current_plan_executes_real_handler
        && idempotency_evidence.key_fingerprint.is_some()
}

pub fn hepta_kernel_native_post_rate_limit_check_required(
    execution_admission: &HeptaKernelNativePostExecutionAdmission,
    duplicate_check_performed: bool,
    duplicate_found: bool,
    duplicate_check_error: Option<&'static str>,
) -> bool {
    execution_admission.current_plan_executes_real_handler
        && !(duplicate_check_performed && duplicate_found)
        && duplicate_check_error.is_none()
}

pub fn hepta_kernel_native_post_store_capacity_check_required(
    execution_admission: &HeptaKernelNativePostExecutionAdmission,
    duplicate_check_performed: bool,
    duplicate_found: bool,
    duplicate_check_error: Option<&'static str>,
    rate_limited: bool,
    rate_limit_check_error: Option<&'static str>,
) -> bool {
    hepta_kernel_native_post_rate_limit_check_required(
        execution_admission,
        duplicate_check_performed,
        duplicate_found,
        duplicate_check_error,
    ) && !rate_limited
        && rate_limit_check_error.is_none()
}

pub fn hepta_kernel_native_post_store_write_attempt_required(
    capacity_check_performed: bool,
    store_capacity_ok: bool,
    store_capacity_check_error: Option<&'static str>,
) -> bool {
    capacity_check_performed && store_capacity_ok && store_capacity_check_error.is_none()
}

pub fn hepta_kernel_native_post_real_handler_scope_matches(
    plan_kind: &str,
    handler_scope: Option<&str>,
) -> bool {
    handler_scope
        .map(hepta_kernel_native_post_real_handler_scope_tokens)
        .unwrap_or_default()
        .contains(&plan_kind)
}

pub fn hepta_kernel_native_post_real_handler_scope_selected_kinds(
    handler_scope: Option<&str>,
) -> Vec<&'static str> {
    HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_PLAN_KINDS
        .iter()
        .copied()
        .filter(|plan_kind| {
            hepta_kernel_native_post_real_handler_scope_matches(plan_kind, handler_scope)
        })
        .collect()
}

pub fn hepta_kernel_native_post_real_handler_scope_single_selected_kind(
    handler_scope: Option<&str>,
) -> Option<&'static str> {
    let selected_handler_kinds =
        hepta_kernel_native_post_real_handler_scope_selected_kinds(handler_scope);
    (selected_handler_kinds.len() == 1).then(|| selected_handler_kinds[0])
}

fn hepta_kernel_native_post_real_handler_scope_tokens(handler_scope: &str) -> Vec<&str> {
    handler_scope
        .split([',', ';', ' ', '\t', '\n', '\r'])
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .collect()
}

pub fn hepta_kernel_native_post_execution_readiness_report(
    real_handler_gate_enabled: bool,
    handler_scope: Option<&str>,
) -> HeptaKernelNativePostExecutionReadinessResponse {
    let handler_scope = handler_scope
        .map(str::trim)
        .filter(|scope| !scope.is_empty())
        .map(str::to_string);
    let selected_handler_kinds =
        hepta_kernel_native_post_real_handler_scope_selected_kinds(handler_scope.as_deref());
    let selected_handler_count = selected_handler_kinds.len();
    let handler_scope_configured = handler_scope.is_some();
    let single_handler_scope_ready = selected_handler_count == 1;
    let routes = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .map(hepta_kernel_native_post_execution_readiness_route)
        .collect::<Vec<_>>();
    let real_handler_candidate_count = routes
        .iter()
        .filter(|route| route.allowlisted_for_real_handler)
        .count();
    let plan_only_route_count = routes.len().saturating_sub(real_handler_candidate_count);
    let evidence_contract_route_count = routes
        .iter()
        .filter(|route| route.execution_evidence_contract_ready)
        .count();
    let real_handler_implemented_count = routes
        .iter()
        .filter(|route| route.real_handler_implemented)
        .count();
    let real_handler_ready_count = routes
        .iter()
        .filter(|route| route.ready_for_real_handler_wiring)
        .count();
    let all_evidence_contracts_ready = evidence_contract_route_count == routes.len();
    let all_real_handlers_blocked = routes
        .iter()
        .all(|route| !route.current_plan_executes_real_handler);

    HeptaKernelNativePostExecutionReadinessResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if all_evidence_contracts_ready {
            "ready"
        } else {
            "attention"
        },
        endpoint: HEPTA_KERNEL_NATIVE_POST_EXECUTION_READINESS_ENDPOINT,
        source_command: "/native-post-execution-readiness --json",
        native_route: true,
        compatibility_mode: "native_post_execution_readiness",
        side_effect_free: true,
        post_route_count: routes.len(),
        real_handler_candidate_count,
        plan_only_route_count,
        evidence_contract_route_count,
        all_evidence_contracts_ready,
        real_handler_implemented_count,
        real_handler_ready_count,
        real_handler_gate_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLERS_ENV,
        real_handler_gate_enabled,
        real_handler_scope_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
        real_handler_scope: handler_scope,
        real_handler_scope_configured: handler_scope_configured,
        single_handler_scope_ready,
        selected_handler_count,
        selected_handler_kinds,
        all_real_handlers_blocked,
        routes,
        action_dispatched: false,
        command_executed: false,
        approval_applied: false,
        task_published: false,
        chat_mutated: false,
        raw_request_body_exposed: false,
        raw_parameter_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: LEGACY_CONTROL_UI_MUTATION_NEXT_ACTION,
    }
}

fn hepta_kernel_native_post_execution_readiness_route(
    spec: &HeptaKernelNativePostPlanRouteSpec,
) -> HeptaKernelNativePostExecutionReadinessRoute {
    let body_schema = hepta_kernel_native_post_body_schema(spec.plan_kind, false);
    let allowlisted_for_real_handler = spec.confirmation_required_for_real_mutation;
    let execution_evidence_contract_ready = true;
    let real_handler_implemented =
        hepta_kernel_native_post_plan_kind_has_real_handler(spec.plan_kind);
    HeptaKernelNativePostExecutionReadinessRoute {
        pattern: spec.pattern,
        capability: spec.capability,
        plan_kind: spec.plan_kind,
        compatibility_mode: spec.compatibility_mode,
        dry_run_only: spec.dry_run_only,
        allowlisted_for_real_handler,
        body_schema_id: body_schema.schema_id,
        body_required_for_real_handler: body_schema.body_required_for_real_handler,
        body_schema_ready: true,
        confirmation_contract_ready: true,
        rollback_contract_ready: true,
        idempotency_evidence_contract_ready: true,
        audit_event_contract_ready: true,
        rate_limit_contract_ready: true,
        execution_evidence_contract_ready,
        ready_for_real_handler_wiring: allowlisted_for_real_handler
            && real_handler_implemented
            && execution_evidence_contract_ready,
        current_plan_executes_real_handler: false,
        real_handler_implemented,
        blocked_reason: if allowlisted_for_real_handler && real_handler_implemented {
            "real_handler_gate_disabled"
        } else if allowlisted_for_real_handler {
            "real_handler_not_wired"
        } else {
            "plan_only_route"
        },
    }
}

pub fn hepta_kernel_native_post_activation_plan_report(
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
    store_contracts_ready: bool,
    store_jsonl_valid: bool,
    store_capacity_ok: bool,
    rollback_store_ready: bool,
) -> HeptaKernelNativePostActivationPlanResponse {
    let readiness = hepta_kernel_native_post_execution_readiness_report(
        real_handler_gate_enabled,
        handler_scope,
    );
    let handler_scope = readiness.real_handler_scope.clone();
    let selected_handler_kinds = readiness.selected_handler_kinds.clone();
    let selected_handler_count = readiness.selected_handler_count;
    let handler_scope_configured = readiness.real_handler_scope_configured;
    let single_handler_scope_ready = readiness.single_handler_scope_ready;
    let all_handlers_implemented =
        readiness.real_handler_implemented_count == readiness.real_handler_candidate_count;
    let activation_preflight_ready =
        readiness.all_evidence_contracts_ready && all_handlers_implemented && store_contracts_ready;
    let activation_currently_enabled = activation_preflight_ready
        && real_handler_gate_enabled
        && operator_approval_enabled
        && single_handler_scope_ready;
    let activation_blocked_reason = if !readiness.all_evidence_contracts_ready {
        "execution_evidence_not_ready"
    } else if !all_handlers_implemented {
        "real_handler_not_implemented"
    } else if !store_contracts_ready {
        "store_contract_not_ready"
    } else if !real_handler_gate_enabled {
        "real_handler_gate_disabled"
    } else if !operator_approval_enabled {
        "operator_approval_required"
    } else if !handler_scope_configured {
        "handler_scope_not_selected"
    } else if !single_handler_scope_ready {
        "handler_scope_not_single"
    } else {
        "single_handler_scope_satisfied_dry_run_harness_only"
    };
    let rollback_ready = activation_preflight_ready && rollback_store_ready;

    HeptaKernelNativePostActivationPlanResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if activation_preflight_ready {
            "ready"
        } else {
            "attention"
        },
        endpoint: HEPTA_KERNEL_NATIVE_POST_ACTIVATION_PLAN_ENDPOINT,
        source_command: "/native-post-activation-plan --json",
        native_route: true,
        compatibility_mode: "native_post_activation_plan",
        side_effect_free: true,
        activation_preflight_ready,
        activation_currently_enabled,
        activation_blocked_reason,
        handler_candidate_count: readiness.real_handler_candidate_count,
        handler_implemented_count: readiness.real_handler_implemented_count,
        all_handlers_implemented,
        handler_scope_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
        handler_scope,
        handler_scope_configured,
        single_handler_scope_ready,
        selected_handler_count,
        selected_handler_kinds,
        execution_evidence_ready: readiness.all_evidence_contracts_ready,
        store_contracts_ready,
        store_jsonl_valid,
        store_capacity_ok,
        required_gates: vec![
            HeptaKernelNativePostActivationGate {
                env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLERS_ENV,
                enabled: real_handler_gate_enabled,
                required_for_activation: true,
                purpose: "allow native POST real-handler harness execution",
            },
            HeptaKernelNativePostActivationGate {
                env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
                enabled: operator_approval_enabled,
                required_for_activation: true,
                purpose: "operator approval for confirm-required native POST mutations",
            },
            HeptaKernelNativePostActivationGate {
                env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
                enabled: single_handler_scope_ready,
                required_for_activation: true,
                purpose: "select exactly one native POST handler for canary dry-run harness execution",
            },
        ],
        rollback_ready,
        rollback_anchor_required: true,
        rollback_store_kind: "rollback",
        rollback_store_file: "rollback.jsonl",
        rollback_schema_id: "hepta.post.rollback_anchor.v1",
        rollback_actions: vec![
            "unset HEPTA_NATIVE_POST_REAL_HANDLERS, HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED, and HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE",
            "restart ai.hepta.gateway through launchctl kickstart",
            "inspect /api/native-post-execution-stores for valid rollback anchors",
            "restore the latest hepta-codex binary/plist backup if gateway health regresses",
        ],
        dry_run_only: true,
        real_mutation_performed: false,
        store_write_attempted: false,
        approval_applied: false,
        task_published: false,
        chat_mutated: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        raw_request_body_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
        next_migration_slice: LEGACY_CONTROL_UI_MUTATION_NEXT_ACTION,
    }
}

pub fn hepta_kernel_native_post_execution_store_record(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    body_schema: &HeptaKernelNativePostBodySchema,
    body_admission: &HeptaKernelNativePostBodyAdmission,
    idempotency_evidence: &HeptaKernelNativePostIdempotencyEvidence,
    audit_event_contract: &HeptaKernelNativePostAuditEventContract,
    current_plan_executes_real_handler: bool,
    recorded_at_unix_ms: u64,
) -> HeptaKernelNativePostExecutionStoreRecord {
    HeptaKernelNativePostExecutionStoreRecord {
        schema_id: "hepta.post.execution_store_record.v1",
        recorded_at_unix_ms,
        route_pattern: spec.pattern,
        capability: spec.capability,
        plan_kind: spec.plan_kind,
        body_schema_id: body_schema.schema_id,
        body_admission_status: body_admission.admission_status,
        idempotency_key_required: body_admission.idempotency_key_required,
        idempotency_key_present: idempotency_evidence.key_present,
        idempotency_key_redacted: idempotency_evidence.key_redacted,
        idempotency_key_fingerprint: idempotency_evidence.key_fingerprint.clone(),
        duplicate_suppression_required: idempotency_evidence.duplicate_suppression_required,
        audit_event_schema_id: audit_event_contract.schema_id,
        audit_event_ready_for_real_handler: audit_event_contract.ready_for_real_handler,
        rollback_strategy: "pending_real_handler_rollback_anchor",
        rate_limit_bucket: spec.plan_kind,
        current_plan_executes_real_handler,
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn hepta_kernel_native_post_real_handler_harness(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    execution_admission: &HeptaKernelNativePostExecutionAdmission,
    duplicate_check_performed: bool,
    duplicate_found: bool,
    duplicate_check_error: Option<&'static str>,
    rate_limit_check_performed: bool,
    rate_limited: bool,
    rate_limit_window_ms: u64,
    rate_limit_check_error: Option<&'static str>,
    capacity_check_performed: bool,
    store_capacity_ok: bool,
    store_capacity_check_error: Option<&'static str>,
    store_write_attempted: bool,
    store_write_succeeded: bool,
    store_write_report: Option<HeptaKernelNativePostExecutionStoreWriteReport>,
    store_write_error: Option<&'static str>,
) -> HeptaKernelNativePostRealHandlerHarness {
    let observation_allowed = execution_admission.current_plan_executes_real_handler;
    let duplicate_check_performed = observation_allowed && duplicate_check_performed;
    let duplicate_found = duplicate_check_performed && duplicate_found;
    let duplicate_check_error = observation_allowed
        .then_some(duplicate_check_error)
        .flatten();
    let rate_limit_check_performed = observation_allowed && rate_limit_check_performed;
    let rate_limited = rate_limit_check_performed && rate_limited;
    let rate_limit_check_error = observation_allowed
        .then_some(rate_limit_check_error)
        .flatten();
    let capacity_check_performed = observation_allowed && capacity_check_performed;
    let store_capacity_ok = observation_allowed && store_capacity_ok;
    let store_capacity_check_error = observation_allowed
        .then_some(store_capacity_check_error)
        .flatten();
    let store_write_attempted = observation_allowed && store_write_attempted;
    let store_write_succeeded = store_write_attempted && store_write_succeeded;
    let store_write_report = store_write_succeeded
        .then_some(store_write_report)
        .flatten();
    let store_write_error = observation_allowed.then_some(store_write_error).flatten();
    let duplicate_suppressed = duplicate_check_performed && duplicate_found;
    let dual_gate_satisfied = execution_admission.enablement_gate_enabled
        && execution_admission.operator_approval_enabled;

    HeptaKernelNativePostRealHandlerHarness {
        status: if !execution_admission.allowlisted_for_real_handler {
            "plan_only_route"
        } else if !execution_admission.real_handler_implemented {
            "not_implemented"
        } else if !store_write_attempted {
            if duplicate_suppressed {
                "duplicate_suppressed"
            } else if duplicate_check_error.is_some() {
                "idempotency_check_failed"
            } else if rate_limited {
                "rate_limited"
            } else if rate_limit_check_error.is_some() {
                "rate_limit_check_failed"
            } else if !store_capacity_ok {
                "store_capacity_blocked"
            } else if store_capacity_check_error.is_some() {
                "store_capacity_check_failed"
            } else {
                "blocked"
            }
        } else if store_write_succeeded {
            "dry_run_recorded"
        } else {
            "store_write_failed"
        },
        handler_kind: spec.plan_kind,
        dry_run_only: true,
        handler_implemented: execution_admission.real_handler_implemented,
        dual_gate_satisfied,
        enablement_gate_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLERS_ENV,
        enablement_gate_enabled: execution_admission.enablement_gate_enabled,
        operator_approval_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
        operator_approval_enabled: execution_admission.operator_approval_enabled,
        handler_scope_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
        handler_scope: execution_admission.handler_scope.clone(),
        handler_scope_configured: execution_admission.handler_scope_configured,
        handler_scope_required: execution_admission.handler_scope_required,
        handler_scope_matches: execution_admission.handler_scope_matches,
        duplicate_check_performed,
        duplicate_found,
        duplicate_suppressed,
        duplicate_check_error,
        rate_limit_check_performed,
        rate_limited,
        rate_limit_suppressed: rate_limit_check_performed && rate_limited,
        rate_limit_window_ms,
        rate_limit_check_error,
        capacity_check_performed,
        store_capacity_ok,
        store_capacity_check_error,
        store_write_attempted,
        store_write_succeeded,
        store_write_report,
        store_write_error,
        task_published: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
    }
}

pub fn hepta_kernel_native_post_real_handler_harness_from_observation(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    execution_admission: &HeptaKernelNativePostExecutionAdmission,
    observation: HeptaKernelNativePostRealHandlerObservation,
) -> HeptaKernelNativePostRealHandlerHarness {
    hepta_kernel_native_post_real_handler_harness(
        spec,
        execution_admission,
        observation.duplicate_check_performed,
        observation.duplicate_found,
        observation.duplicate_check_error,
        observation.rate_limit_check_performed,
        observation.rate_limited,
        observation.rate_limit_window_ms,
        observation.rate_limit_check_error,
        observation.capacity_check_performed,
        observation.store_capacity_ok,
        observation.store_capacity_check_error,
        observation.store_write_attempted,
        observation.store_write_succeeded,
        observation.store_write_report,
        observation.store_write_error,
    )
}

pub fn hepta_kernel_native_post_store_effect_projection(
    mut idempotency_evidence: HeptaKernelNativePostIdempotencyEvidence,
    mut audit_event_contract: HeptaKernelNativePostAuditEventContract,
    real_handler_harness: &HeptaKernelNativePostRealHandlerHarness,
) -> HeptaKernelNativePostStoreEffectProjection {
    if real_handler_harness.duplicate_check_performed {
        idempotency_evidence.current_plan_lookup_performed = true;
    }
    if real_handler_harness.store_write_succeeded {
        idempotency_evidence.current_plan_store_written = true;
        audit_event_contract.current_plan_emits_audit_event = true;
        audit_event_contract.current_plan_persists_audit_event = true;
    }

    HeptaKernelNativePostStoreEffectProjection {
        idempotency_evidence,
        audit_event_contract,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn hepta_kernel_native_post_plan_response(
    spec: &HeptaKernelNativePostPlanRouteSpec,
    parameter_present: bool,
    parameter_length: Option<usize>,
    body_schema: HeptaKernelNativePostBodySchema,
    body_admission: HeptaKernelNativePostBodyAdmission,
    confirmation_contract: HeptaKernelNativePostConfirmationContract,
    rollback_contract: HeptaKernelNativePostRollbackContract,
    idempotency_evidence: HeptaKernelNativePostIdempotencyEvidence,
    audit_event_contract: HeptaKernelNativePostAuditEventContract,
    execution_admission: HeptaKernelNativePostExecutionAdmission,
    real_handler_harness: HeptaKernelNativePostRealHandlerHarness,
) -> HeptaKernelNativePostPlanResponse {
    HeptaKernelNativePostPlanResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if spec.confirmation_required_for_real_mutation {
            "confirm_required"
        } else {
            "dry_run_ready"
        },
        method: "POST",
        pattern: spec.pattern,
        source_command: spec.source_command,
        capability: spec.capability,
        native_route: true,
        compatibility_mode: spec.compatibility_mode,
        side_effect_free: !real_handler_harness.store_write_attempted,
        plan_kind: spec.plan_kind,
        dry_run_only: spec.dry_run_only,
        confirmation_required_for_real_mutation: spec.confirmation_required_for_real_mutation,
        parameter_present,
        parameter_redacted: parameter_present,
        parameter_length,
        request_body_read: body_admission.request_body_read,
        request_body_redacted: true,
        body_schema_ready: true,
        body_admission_ready: true,
        confirmation_contract_ready: true,
        rollback_contract_ready: true,
        idempotency_evidence_ready: true,
        audit_event_contract_ready: true,
        execution_admission_ready: true,
        body_schema,
        body_admission,
        confirmation_contract,
        rollback_contract,
        idempotency_evidence,
        audit_event_contract,
        execution_admission,
        real_handler_harness_ready: true,
        real_handler_harness,
        action_dispatched: false,
        command_executed: false,
        approval_applied: false,
        task_published: false,
        chat_mutated: false,
        raw_request_body_exposed: false,
        raw_parameter_exposed: false,
        raw_token_exposed: false,
        raw_transcript_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "legacy control-UI POST remains compatibility-plan-only; use the governed mutation registry for real effects",
    }
}

pub fn hepta_kernel_native_post_execution_stores_report(
    store_root: String,
    root_exists: bool,
    root_is_dir: bool,
    max_store_bytes: u64,
    max_store_lines: u64,
    stores: Vec<HeptaKernelNativePostExecutionStoreFileStatus>,
) -> HeptaKernelNativePostExecutionStoresResponse {
    let existing_file_count = stores.iter().filter(|file| file.exists).count();
    let total_bytes = stores.iter().map(|file| file.bytes).sum::<u64>();
    let total_line_count = stores.iter().map(|file| file.line_count).sum::<u64>();
    let valid_json_line_count = stores
        .iter()
        .map(|file| file.valid_json_line_count)
        .sum::<u64>();
    let invalid_json_line_count = stores
        .iter()
        .map(|file| file.invalid_json_line_count)
        .sum::<u64>();
    let store_jsonl_valid = hepta_kernel_native_post_execution_store_jsonl_valid(&stores);
    let store_capacity_ok = hepta_kernel_native_post_execution_store_capacity_ok(&stores);

    HeptaKernelNativePostExecutionStoresResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if store_jsonl_valid && store_capacity_ok {
            "ready"
        } else {
            "attention"
        },
        endpoint: HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORES_ENDPOINT,
        source_command: "/native-post-execution-stores --json",
        native_route: true,
        compatibility_mode: "native_post_execution_stores",
        side_effect_free: true,
        store_root_env: HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORE_DIR_ENV,
        store_root,
        root_exists,
        root_is_dir,
        store_file_count: stores.len(),
        existing_file_count,
        max_store_bytes_env: HEPTA_KERNEL_NATIVE_POST_STORE_MAX_BYTES_ENV,
        max_store_bytes,
        max_store_lines_env: HEPTA_KERNEL_NATIVE_POST_STORE_MAX_LINES_ENV,
        max_store_lines,
        total_bytes,
        store_jsonl_valid,
        store_capacity_ok,
        total_line_count,
        valid_json_line_count,
        invalid_json_line_count,
        stores,
        persistence_implementation_ready: true,
        idempotency_store_ready: true,
        audit_store_ready: true,
        rollback_store_ready: true,
        rate_limit_store_ready: true,
        status_probe_creates_directory: false,
        status_probe_writes_files: false,
        current_plan_executes_real_handler: false,
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
        action_dispatched: false,
        command_executed: false,
        approval_applied: false,
        task_published: false,
        chat_mutated: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "wire a first low-risk real handler only after these stores are called under HEPTA_NATIVE_POST_REAL_HANDLERS with operator approval",
    }
}

pub fn hepta_kernel_native_post_execution_store_contracts_ready(
    stores: &HeptaKernelNativePostExecutionStoresResponse,
) -> bool {
    stores.persistence_implementation_ready
        && stores.idempotency_store_ready
        && stores.audit_store_ready
        && stores.rollback_store_ready
        && stores.rate_limit_store_ready
        && stores.store_jsonl_valid
        && stores.store_capacity_ok
}

pub fn hepta_kernel_native_post_rollout_evidence_scan_missing()
-> HeptaKernelNativePostRolloutEvidenceScan {
    hepta_kernel_native_post_empty_rollout_evidence_scan(true, None)
}

pub fn hepta_kernel_native_post_rollout_evidence_scan_read_failed()
-> HeptaKernelNativePostRolloutEvidenceScan {
    hepta_kernel_native_post_empty_rollout_evidence_scan(false, Some("rollback_store_read_failed"))
}

pub fn hepta_kernel_native_post_rollout_evidence_scan_from_content(
    content: &str,
) -> HeptaKernelNativePostRolloutEvidenceScan {
    let mut line_count = 0_u64;
    let mut valid_json_line_count = 0_u64;
    let mut invalid_json_line_count = 0_u64;
    let mut record_count = 0_u64;
    let mut dry_run_record_count = 0_u64;
    let mut rollback_anchor_count = 0_u64;
    let mut plan_kind_counts = BTreeMap::<String, u64>::new();
    let mut latest_record: Option<HeptaKernelNativePostRolloutEvidenceRecordSummary> = None;
    let mut latest_recorded_at = 0_u64;
    let mut raw_request_body_exposed = false;
    let mut raw_field_values_exposed = false;
    let mut raw_idempotency_key_exposed = false;
    let mut raw_audit_payload_exposed = false;

    for line in content.lines() {
        line_count = line_count.saturating_add(1);
        let value = match serde_json::from_str::<Value>(line) {
            Ok(value) => value,
            Err(_) => {
                invalid_json_line_count = invalid_json_line_count.saturating_add(1);
                continue;
            }
        };
        valid_json_line_count = valid_json_line_count.saturating_add(1);
        record_count = record_count.saturating_add(1);
        let plan_kind = value
            .get("plan_kind")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        *plan_kind_counts.entry(plan_kind).or_insert(0) += 1;
        let current_plan_executes_real_handler =
            hepta_kernel_json_bool_field(&value, "current_plan_executes_real_handler");
        if current_plan_executes_real_handler {
            dry_run_record_count = dry_run_record_count.saturating_add(1);
        }
        if value.get("rollback_strategy").and_then(Value::as_str)
            == Some("pending_real_handler_rollback_anchor")
        {
            rollback_anchor_count = rollback_anchor_count.saturating_add(1);
        }
        raw_request_body_exposed |=
            hepta_kernel_json_bool_field(&value, "raw_request_body_exposed");
        raw_field_values_exposed |=
            hepta_kernel_json_bool_field(&value, "raw_field_values_exposed");
        raw_idempotency_key_exposed |=
            hepta_kernel_json_bool_field(&value, "raw_idempotency_key_exposed");
        raw_audit_payload_exposed |=
            hepta_kernel_json_bool_field(&value, "raw_audit_payload_exposed");

        let recorded_at = value
            .get("recorded_at_unix_ms")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        if latest_record.is_none() || recorded_at >= latest_recorded_at {
            latest_recorded_at = recorded_at;
            latest_record = Some(hepta_kernel_native_post_rollout_evidence_record_summary(
                &value,
            ));
        }
    }

    HeptaKernelNativePostRolloutEvidenceScan {
        jsonl_readable: true,
        read_error: None,
        line_count,
        valid_json_line_count,
        invalid_json_line_count,
        record_count,
        dry_run_record_count,
        rollback_anchor_count,
        plan_kind_counts: plan_kind_counts
            .into_iter()
            .map(
                |(plan_kind, count)| HeptaKernelNativePostRolloutEvidencePlanKindCount {
                    plan_kind,
                    count,
                },
            )
            .collect(),
        latest_record,
        raw_request_body_exposed,
        raw_field_values_exposed,
        raw_idempotency_key_exposed,
        raw_audit_payload_exposed,
    }
}

pub fn hepta_kernel_native_post_rollout_evidence_scan_from_observation(
    observation: HeptaKernelNativePostRolloutEvidenceFileObservation,
) -> HeptaKernelNativePostRolloutEvidenceScan {
    if let Some(content) = observation.content {
        return hepta_kernel_native_post_rollout_evidence_scan_from_content(&content);
    }
    if observation.missing {
        return hepta_kernel_native_post_rollout_evidence_scan_missing();
    }
    if observation.read_failed {
        return hepta_kernel_native_post_rollout_evidence_scan_read_failed();
    }
    hepta_kernel_native_post_rollout_evidence_scan_read_failed()
}

pub fn hepta_kernel_native_post_rollout_evidence_report(
    store_root: String,
    store_jsonl_valid: bool,
    store_capacity_ok: bool,
    handler_scope: Option<&str>,
    scan: HeptaKernelNativePostRolloutEvidenceScan,
) -> HeptaKernelNativePostRolloutEvidenceResponse {
    let rollout_evidence_ready = store_jsonl_valid
        && store_capacity_ok
        && scan.jsonl_readable
        && scan.invalid_json_line_count == 0;
    let activation_scope = handler_scope
        .map(str::trim)
        .filter(|scope| !scope.is_empty())
        .map(str::to_string);
    let selected_handler_kinds =
        hepta_kernel_native_post_real_handler_scope_selected_kinds(activation_scope.as_deref());
    let selected_handler_count = selected_handler_kinds.len();

    HeptaKernelNativePostRolloutEvidenceResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if rollout_evidence_ready {
            "ready"
        } else {
            "attention"
        },
        endpoint: HEPTA_KERNEL_NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT,
        source_command: "/native-post-rollout-evidence --json",
        native_route: true,
        compatibility_mode: "native_post_rollout_evidence",
        side_effect_free: true,
        store_root_env: HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORE_DIR_ENV,
        store_root,
        rollback_store_file: "rollback.jsonl",
        store_jsonl_valid,
        store_capacity_ok,
        rollout_evidence_ready,
        activation_scope_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
        activation_scope,
        single_handler_scope_ready: selected_handler_count == 1,
        selected_handler_count,
        selected_handler_kinds,
        rollback_anchor_present: scan.rollback_anchor_count > 0,
        dry_run_record_present: scan.dry_run_record_count > 0,
        record_count: scan.record_count,
        dry_run_record_count: scan.dry_run_record_count,
        rollback_anchor_count: scan.rollback_anchor_count,
        line_count: scan.line_count,
        valid_json_line_count: scan.valid_json_line_count,
        invalid_json_line_count: scan.invalid_json_line_count,
        jsonl_readable: scan.jsonl_readable,
        read_error: scan.read_error,
        plan_kind_counts: scan.plan_kind_counts,
        latest_record: scan.latest_record,
        raw_request_body_exposed: scan.raw_request_body_exposed,
        raw_field_values_exposed: scan.raw_field_values_exposed,
        raw_idempotency_key_exposed: scan.raw_idempotency_key_exposed,
        raw_audit_payload_exposed: scan.raw_audit_payload_exposed,
        real_mutation_performed: false,
        approval_applied: false,
        task_published: false,
        chat_mutated: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "run one scoped dry-run canary until rollback evidence is present, then decide whether to wire a real handler behind the same scope gate",
    }
}

pub fn hepta_kernel_native_post_selected_handler_rollout_evidence_missing(
    selected_handler_kind: Option<&str>,
) -> HeptaKernelNativePostSelectedHandlerRolloutEvidence {
    hepta_kernel_native_post_empty_selected_handler_rollout_evidence(selected_handler_kind)
}

pub fn hepta_kernel_native_post_selected_handler_rollout_evidence_from_content(
    selected_handler_kind: Option<&str>,
    content: &str,
) -> HeptaKernelNativePostSelectedHandlerRolloutEvidence {
    let selected_handler_kind_string = selected_handler_kind.map(str::to_string);
    let mut record_count = 0_u64;
    let mut dry_run_record_count = 0_u64;
    let mut rollback_anchor_count = 0_u64;
    let mut latest_record: Option<HeptaKernelNativePostRolloutEvidenceRecordSummary> = None;
    let mut latest_recorded_at = 0_u64;
    let mut raw_request_body_exposed = false;
    let mut raw_field_values_exposed = false;
    let mut raw_idempotency_key_exposed = false;
    let mut raw_audit_payload_exposed = false;

    let Some(selected_handler_kind) = selected_handler_kind else {
        return hepta_kernel_native_post_empty_selected_handler_rollout_evidence(None);
    };

    for line in content.lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if value.get("plan_kind").and_then(Value::as_str) != Some(selected_handler_kind) {
            continue;
        }
        record_count = record_count.saturating_add(1);
        let current_plan_executes_real_handler =
            hepta_kernel_json_bool_field(&value, "current_plan_executes_real_handler");
        if current_plan_executes_real_handler {
            dry_run_record_count = dry_run_record_count.saturating_add(1);
        }
        if value.get("rollback_strategy").and_then(Value::as_str)
            == Some("pending_real_handler_rollback_anchor")
        {
            rollback_anchor_count = rollback_anchor_count.saturating_add(1);
        }
        raw_request_body_exposed |=
            hepta_kernel_json_bool_field(&value, "raw_request_body_exposed");
        raw_field_values_exposed |=
            hepta_kernel_json_bool_field(&value, "raw_field_values_exposed");
        raw_idempotency_key_exposed |=
            hepta_kernel_json_bool_field(&value, "raw_idempotency_key_exposed");
        raw_audit_payload_exposed |=
            hepta_kernel_json_bool_field(&value, "raw_audit_payload_exposed");

        let recorded_at = value
            .get("recorded_at_unix_ms")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        if latest_record.is_none() || recorded_at >= latest_recorded_at {
            latest_recorded_at = recorded_at;
            latest_record = Some(hepta_kernel_native_post_rollout_evidence_record_summary(
                &value,
            ));
        }
    }

    HeptaKernelNativePostSelectedHandlerRolloutEvidence {
        selected_handler_kind: selected_handler_kind_string,
        record_count,
        dry_run_record_count,
        rollback_anchor_count,
        dry_run_record_present: dry_run_record_count > 0,
        rollback_anchor_present: rollback_anchor_count > 0,
        latest_record,
        raw_request_body_exposed,
        raw_field_values_exposed,
        raw_idempotency_key_exposed,
        raw_audit_payload_exposed,
    }
}

pub fn hepta_kernel_native_post_selected_handler_rollout_evidence_from_observation(
    selected_handler_kind: Option<&str>,
    observation: HeptaKernelNativePostRolloutEvidenceFileObservation,
) -> HeptaKernelNativePostSelectedHandlerRolloutEvidence {
    if let Some(content) = observation.content {
        return hepta_kernel_native_post_selected_handler_rollout_evidence_from_content(
            selected_handler_kind,
            &content,
        );
    }
    hepta_kernel_native_post_selected_handler_rollout_evidence_missing(selected_handler_kind)
}

pub fn hepta_kernel_native_post_gray_release_evidence_report(
    store_root: String,
    handler_scope: Option<&str>,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
    store_jsonl_valid: bool,
    store_capacity_ok: bool,
    rollout_evidence_ready: bool,
    rollout_raw_request_body_exposed: bool,
    rollout_raw_field_values_exposed: bool,
    rollout_raw_idempotency_key_exposed: bool,
    rollout_raw_audit_payload_exposed: bool,
    selected_handler_evidence: HeptaKernelNativePostSelectedHandlerRolloutEvidence,
) -> HeptaKernelNativePostGrayReleaseEvidenceResponse {
    let readiness = hepta_kernel_native_post_execution_readiness_report(
        real_handler_gate_enabled,
        handler_scope,
    );
    let store_contracts_ready = store_jsonl_valid && store_capacity_ok;
    let all_handlers_implemented =
        readiness.real_handler_implemented_count == readiness.real_handler_candidate_count;
    let activation_preflight_ready =
        readiness.all_evidence_contracts_ready && all_handlers_implemented && store_contracts_ready;
    let selected_handler_kinds =
        hepta_kernel_native_post_real_handler_scope_selected_kinds(handler_scope);
    let selected_handler_count = selected_handler_kinds.len();
    let single_handler_scope_ready = selected_handler_count == 1;
    let selected_handler_kind =
        hepta_kernel_native_post_real_handler_scope_single_selected_kind(handler_scope);
    let activation_currently_enabled = activation_preflight_ready
        && real_handler_gate_enabled
        && operator_approval_enabled
        && single_handler_scope_ready;
    let selected_handler_evidence_ready = selected_handler_evidence.dry_run_record_present
        && selected_handler_evidence.rollback_anchor_present
        && !selected_handler_evidence.raw_request_body_exposed
        && !selected_handler_evidence.raw_field_values_exposed
        && !selected_handler_evidence.raw_idempotency_key_exposed
        && !selected_handler_evidence.raw_audit_payload_exposed;
    let gray_release_evidence_ready = activation_preflight_ready
        && single_handler_scope_ready
        && rollout_evidence_ready
        && selected_handler_evidence_ready;
    let gray_release_ready = activation_currently_enabled && gray_release_evidence_ready;
    let gray_release_phase = if !activation_preflight_ready {
        "activation_preflight_not_ready"
    } else if !single_handler_scope_ready {
        "handler_scope_not_single"
    } else if !real_handler_gate_enabled {
        "real_handler_gate_disabled"
    } else if !operator_approval_enabled {
        "operator_approval_required"
    } else if !selected_handler_evidence.dry_run_record_present {
        "awaiting_scoped_dry_run_record"
    } else if !selected_handler_evidence.rollback_anchor_present {
        "rollback_anchor_missing"
    } else if !selected_handler_evidence_ready {
        "redaction_attention"
    } else {
        "gray_release_ready"
    };

    HeptaKernelNativePostGrayReleaseEvidenceResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if gray_release_ready {
            "ready"
        } else if activation_preflight_ready {
            "staged"
        } else {
            "attention"
        },
        endpoint: HEPTA_KERNEL_NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT,
        source_command: "/native-post-gray-release-evidence --json",
        native_route: true,
        compatibility_mode: "native_post_gray_release_evidence",
        side_effect_free: true,
        activation_plan_endpoint: HEPTA_KERNEL_NATIVE_POST_ACTIVATION_PLAN_ENDPOINT,
        rollout_evidence_endpoint: HEPTA_KERNEL_NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT,
        store_root_env: HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORE_DIR_ENV,
        store_root,
        handler_scope_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
        handler_scope: handler_scope
            .map(str::trim)
            .filter(|scope| !scope.is_empty())
            .map(str::to_string),
        selected_handler_count,
        selected_handler_kinds,
        selected_handler_kind: selected_handler_kind.map(str::to_string),
        single_handler_scope_ready,
        real_handler_gate_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLERS_ENV,
        real_handler_gate_enabled,
        operator_approval_env: HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
        operator_approval_enabled,
        activation_preflight_ready,
        activation_currently_enabled,
        store_jsonl_valid,
        store_capacity_ok,
        rollout_evidence_ready,
        gray_release_evidence_ready,
        selected_handler_evidence_ready,
        gray_release_ready,
        gray_release_phase,
        selected_handler_evidence,
        rollback_actions: vec![
            "unset HEPTA_NATIVE_POST_REAL_HANDLERS, HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED, and HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE",
            "restart ai.hepta.gateway after plist/env changes",
            "inspect /api/native-post-gray-release-evidence and /api/native-post-rollout-evidence before reattempting activation",
            "restore the latest hepta-codex binary/plist backup if gateway health regresses",
        ],
        dry_run_only: true,
        real_mutation_performed: false,
        store_write_attempted: false,
        approval_applied: false,
        task_published: false,
        chat_mutated: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        raw_request_body_exposed: rollout_raw_request_body_exposed,
        raw_field_values_exposed: rollout_raw_field_values_exposed,
        raw_idempotency_key_exposed: rollout_raw_idempotency_key_exposed,
        raw_audit_payload_exposed: rollout_raw_audit_payload_exposed,
        next_migration_slice: "run exactly one scoped POST dry-run canary and require rollback evidence before any real mutation wiring",
    }
}

fn hepta_kernel_native_post_empty_rollout_evidence_scan(
    jsonl_readable: bool,
    read_error: Option<&'static str>,
) -> HeptaKernelNativePostRolloutEvidenceScan {
    HeptaKernelNativePostRolloutEvidenceScan {
        jsonl_readable,
        read_error,
        line_count: 0,
        valid_json_line_count: 0,
        invalid_json_line_count: 0,
        record_count: 0,
        dry_run_record_count: 0,
        rollback_anchor_count: 0,
        plan_kind_counts: Vec::new(),
        latest_record: None,
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
    }
}

fn hepta_kernel_native_post_empty_selected_handler_rollout_evidence(
    selected_handler_kind: Option<&str>,
) -> HeptaKernelNativePostSelectedHandlerRolloutEvidence {
    HeptaKernelNativePostSelectedHandlerRolloutEvidence {
        selected_handler_kind: selected_handler_kind.map(str::to_string),
        record_count: 0,
        dry_run_record_count: 0,
        rollback_anchor_count: 0,
        dry_run_record_present: false,
        rollback_anchor_present: false,
        latest_record: None,
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
    }
}

fn hepta_kernel_native_post_rollout_evidence_record_summary(
    value: &Value,
) -> HeptaKernelNativePostRolloutEvidenceRecordSummary {
    HeptaKernelNativePostRolloutEvidenceRecordSummary {
        recorded_at_unix_ms: value.get("recorded_at_unix_ms").and_then(Value::as_u64),
        route_pattern: hepta_kernel_json_string_field(value, "route_pattern"),
        capability: hepta_kernel_json_string_field(value, "capability"),
        plan_kind: hepta_kernel_json_string_field(value, "plan_kind"),
        body_schema_id: hepta_kernel_json_string_field(value, "body_schema_id"),
        body_admission_status: hepta_kernel_json_string_field(value, "body_admission_status"),
        rollback_strategy: hepta_kernel_json_string_field(value, "rollback_strategy"),
        rate_limit_bucket: hepta_kernel_json_string_field(value, "rate_limit_bucket"),
        current_plan_executes_real_handler: hepta_kernel_json_bool_field(
            value,
            "current_plan_executes_real_handler",
        ),
        idempotency_key_redacted: hepta_kernel_json_bool_field(value, "idempotency_key_redacted"),
        idempotency_key_fingerprint_present: value
            .get("idempotency_key_fingerprint")
            .and_then(Value::as_str)
            .map(|fingerprint| !fingerprint.trim().is_empty())
            .unwrap_or(false),
        raw_request_body_exposed: hepta_kernel_json_bool_field(value, "raw_request_body_exposed"),
        raw_field_values_exposed: hepta_kernel_json_bool_field(value, "raw_field_values_exposed"),
        raw_idempotency_key_exposed: hepta_kernel_json_bool_field(
            value,
            "raw_idempotency_key_exposed",
        ),
        raw_audit_payload_exposed: hepta_kernel_json_bool_field(value, "raw_audit_payload_exposed"),
    }
}

fn hepta_kernel_json_string_field(value: &Value, field: &str) -> Option<String> {
    value.get(field).and_then(Value::as_str).map(str::to_string)
}

fn hepta_kernel_json_bool_field(value: &Value, field: &str) -> bool {
    value.get(field).and_then(Value::as_bool).unwrap_or(false)
}
