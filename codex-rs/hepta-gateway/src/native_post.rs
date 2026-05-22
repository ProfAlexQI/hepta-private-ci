use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub use hepta_runtime::{
    NATIVE_POST_REAL_HANDLER_PLAN_KINDS, NativePostBodySchema, NativePostPlanRouteSpec,
};

pub const NATIVE_POST_MAX_BODY_BYTES: usize = 64 * 1024;
pub const NATIVE_POST_REAL_HANDLERS_ENV: &str = "HEPTA_NATIVE_POST_REAL_HANDLERS";
pub const NATIVE_POST_REAL_HANDLER_APPROVAL_ENV: &str = "HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED";
pub const NATIVE_POST_REAL_HANDLER_SCOPE_ENV: &str = "HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE";
pub const NATIVE_POST_EXECUTION_STORE_DIR_ENV: &str = "HEPTA_NATIVE_POST_EXECUTION_STORE_DIR";
pub const NATIVE_POST_STORE_MAX_BYTES_ENV: &str = "HEPTA_NATIVE_POST_STORE_MAX_BYTES";
pub const NATIVE_POST_STORE_MAX_LINES_ENV: &str = "HEPTA_NATIVE_POST_STORE_MAX_LINES";
pub const NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV: &str = "HEPTA_NATIVE_POST_RATE_LIMIT_WINDOW_MS";
pub const NATIVE_POST_EXECUTION_READINESS_ENDPOINT: &str = "/api/native-post-execution-readiness";
pub const NATIVE_POST_EXECUTION_STORES_ENDPOINT: &str = "/api/native-post-execution-stores";
pub const NATIVE_POST_ACTIVATION_PLAN_ENDPOINT: &str = "/api/native-post-activation-plan";
pub const NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT: &str = "/api/native-post-rollout-evidence";
pub const NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT: &str =
    "/api/native-post-gray-release-evidence";
pub const DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS: u64 = 1_000;
pub const DEFAULT_NATIVE_POST_STORE_MAX_BYTES: u64 = 10 * 1024 * 1024;
pub const DEFAULT_NATIVE_POST_STORE_MAX_LINES: u64 = 100_000;
pub const DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR: &str = ".hepta/native-post-execution";

pub fn native_post_plan_route_specs() -> &'static [NativePostPlanRouteSpec] {
    hepta_runtime::native_post_plan_route_specs()
}

pub fn native_post_plan_parameter<'a>(
    spec: &NativePostPlanRouteSpec,
    path: &'a str,
) -> Option<Option<&'a str>> {
    hepta_runtime::native_post_plan_parameter(spec, path)
}

pub fn native_post_plan_kind_has_real_handler(plan_kind: &str) -> bool {
    hepta_runtime::native_post_plan_kind_has_real_handler(plan_kind)
}

#[derive(Debug, Serialize)]
pub struct NativePostPlanResponse {
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
    pub body_schema: NativePostBodySchema,
    pub body_admission: NativePostBodyAdmission,
    pub confirmation_contract: NativePostConfirmationContract,
    pub rollback_contract: NativePostRollbackContract,
    pub idempotency_evidence: NativePostIdempotencyEvidence,
    pub audit_event_contract: NativePostAuditEventContract,
    pub execution_admission: NativePostExecutionAdmission,
    pub real_handler_harness_ready: bool,
    pub real_handler_harness: NativePostRealHandlerHarness,
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

#[derive(Debug, Serialize)]
pub struct NativePostBodyAdmission {
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

#[derive(Debug, Serialize)]
pub struct NativePostConfirmationContract {
    pub current_plan_requires_confirmation: bool,
    pub real_mutation_requires_confirmation: bool,
    pub accepted_confirmation_field: Option<&'static str>,
    pub operator_approval_required: bool,
    pub confirmation_mechanism: &'static str,
    pub raw_confirmation_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
pub struct NativePostRollbackContract {
    pub current_plan_noop: bool,
    pub state_written_by_plan: bool,
    pub current_plan_rollback_strategy: &'static str,
    pub real_handler_requires_rollback_contract: bool,
    pub destructive_without_rollback: bool,
    pub rollback_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
pub struct NativePostIdempotencyEvidence {
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

#[derive(Debug, Serialize)]
pub struct NativePostAuditEventContract {
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

#[derive(Debug, Serialize)]
pub struct NativePostExecutionAdmission {
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

#[derive(Debug, Serialize)]
pub struct NativePostRealHandlerHarness {
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
    pub store_write_report: Option<NativePostExecutionStoreWriteReport>,
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

#[derive(Debug, Serialize)]
pub struct NativePostExecutionStoreRecord {
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

#[derive(Debug, Serialize)]
pub struct NativePostExecutionStoreWriteReport {
    pub status: &'static str,
    pub root: String,
    pub written_file_count: usize,
    pub written_files: Vec<String>,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct NativePostExecutionStoreLimits {
    pub max_store_bytes: u64,
    pub max_store_lines: u64,
    pub rate_limit_window_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct NativePostExecutionReadinessResponse {
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
    pub routes: Vec<NativePostExecutionReadinessRoute>,
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

#[derive(Debug, Serialize)]
pub struct NativePostExecutionReadinessRoute {
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

#[derive(Debug, Serialize)]
pub struct NativePostExecutionStoresResponse {
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
    pub stores: Vec<NativePostExecutionStoreFileStatus>,
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

#[derive(Debug, Serialize)]
pub struct NativePostExecutionStoreFileStatus {
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

#[derive(Debug, Serialize)]
pub struct NativePostActivationPlanResponse {
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
    pub required_gates: Vec<NativePostActivationGate>,
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

#[derive(Debug, Serialize)]
pub struct NativePostActivationGate {
    pub env: &'static str,
    pub enabled: bool,
    pub required_for_activation: bool,
    pub purpose: &'static str,
}

#[derive(Debug, Serialize)]
pub struct NativePostRolloutEvidenceResponse {
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
    pub plan_kind_counts: Vec<NativePostRolloutEvidencePlanKindCount>,
    pub latest_record: Option<NativePostRolloutEvidenceRecordSummary>,
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

#[derive(Debug, Serialize)]
pub struct NativePostRolloutEvidencePlanKindCount {
    pub plan_kind: String,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct NativePostRolloutEvidenceRecordSummary {
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

#[derive(Debug, Serialize)]
pub struct NativePostSelectedHandlerRolloutEvidence {
    pub selected_handler_kind: Option<String>,
    pub record_count: u64,
    pub dry_run_record_count: u64,
    pub rollback_anchor_count: u64,
    pub dry_run_record_present: bool,
    pub rollback_anchor_present: bool,
    pub latest_record: Option<NativePostRolloutEvidenceRecordSummary>,
    pub raw_request_body_exposed: bool,
    pub raw_field_values_exposed: bool,
    pub raw_idempotency_key_exposed: bool,
    pub raw_audit_payload_exposed: bool,
}

#[derive(Debug, Serialize)]
pub struct NativePostGrayReleaseEvidenceResponse {
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
    pub selected_handler_evidence: NativePostSelectedHandlerRolloutEvidence,
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

pub fn native_post_body_schema(
    plan_kind: &str,
    body_read_during_plan: bool,
) -> NativePostBodySchema {
    hepta_runtime::native_post_body_schema(plan_kind, body_read_during_plan)
}

pub fn native_post_body_admission(
    spec: &NativePostPlanRouteSpec,
    schema: &NativePostBodySchema,
    request_body: Option<&str>,
) -> NativePostBodyAdmission {
    let body_received = request_body
        .map(str::trim)
        .map(|body| !body.is_empty())
        .unwrap_or(false);
    let request_body_read = request_body.is_some();
    let body_size_bytes = request_body.map(str::len).unwrap_or(0);
    let body_size_within_limit = body_size_bytes <= NATIVE_POST_MAX_BODY_BYTES;
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
    let confirm_field_truthy = json_field_truthy(confirm_field);
    let dry_run_field = object.and_then(|object| object.get("dry_run"));
    let dry_run_field_present = dry_run_field.is_some();
    let dry_run_first_satisfied =
        !spec.confirmation_required_for_real_mutation || json_field_truthy(dry_run_field);
    let idempotency_key_required = spec.confirmation_required_for_real_mutation;
    let idempotency_key_value = object
        .and_then(|object| object.get("idempotency_key"))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let idempotency_key_present = idempotency_key_value.is_some();
    let idempotency_key_fingerprint = idempotency_key_value.map(native_post_redacted_fingerprint);

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

    NativePostBodyAdmission {
        admission_status,
        body_received,
        request_body_read,
        request_body_redacted: true,
        body_size_bytes,
        max_body_bytes: NATIVE_POST_MAX_BODY_BYTES,
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

pub fn native_post_redacted_fingerprint(value: &str) -> String {
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

fn json_field_truthy(value: Option<&serde_json::Value>) -> bool {
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

pub fn native_post_confirmation_contract(
    spec: &NativePostPlanRouteSpec,
) -> NativePostConfirmationContract {
    NativePostConfirmationContract {
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

pub fn native_post_rollback_contract() -> NativePostRollbackContract {
    NativePostRollbackContract {
        current_plan_noop: true,
        state_written_by_plan: false,
        current_plan_rollback_strategy: "noop_no_state_written",
        real_handler_requires_rollback_contract: true,
        destructive_without_rollback: false,
        rollback_payload_exposed: false,
    }
}

pub fn native_post_idempotency_evidence(
    spec: &NativePostPlanRouteSpec,
    body_admission: &NativePostBodyAdmission,
) -> NativePostIdempotencyEvidence {
    let required = spec.confirmation_required_for_real_mutation;
    NativePostIdempotencyEvidence {
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

pub fn native_post_audit_event_contract(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
) -> NativePostAuditEventContract {
    let required = spec.confirmation_required_for_real_mutation;
    NativePostAuditEventContract {
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

pub fn native_post_execution_admission_with_scope(
    spec: &NativePostPlanRouteSpec,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    enablement_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
) -> NativePostExecutionAdmission {
    let allowlisted_for_real_handler = spec.confirmation_required_for_real_mutation;
    let real_handler_implemented = native_post_plan_kind_has_real_handler(spec.plan_kind);
    let handler_scope_configured = handler_scope
        .map(str::trim)
        .map(|scope| !scope.is_empty())
        .unwrap_or(false);
    let handler_scope_matches = !allowlisted_for_real_handler
        || native_post_real_handler_scope_matches(spec.plan_kind, handler_scope);
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
    NativePostExecutionAdmission {
        admission_status: if current_plan_executes_real_handler {
            "harness_ready"
        } else {
            "blocked"
        },
        current_plan_executes_real_handler,
        real_handler_currently_enabled: enablement_gate_enabled,
        real_handler_implemented,
        allowlisted_for_real_handler,
        enablement_gate_env: NATIVE_POST_REAL_HANDLERS_ENV,
        enablement_gate_enabled,
        operator_approval_env: NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
        operator_approval_enabled,
        handler_scope_env: NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
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

pub fn native_post_real_handler_scope_matches(
    plan_kind: &str,
    handler_scope: Option<&str>,
) -> bool {
    handler_scope
        .map(native_post_real_handler_scope_tokens)
        .unwrap_or_default()
        .iter()
        .any(|token| *token == plan_kind)
}

pub fn native_post_real_handler_scope_selected_kinds(
    handler_scope: Option<&str>,
) -> Vec<&'static str> {
    NATIVE_POST_REAL_HANDLER_PLAN_KINDS
        .iter()
        .copied()
        .filter(|plan_kind| native_post_real_handler_scope_matches(plan_kind, handler_scope))
        .collect()
}

fn native_post_real_handler_scope_tokens(handler_scope: &str) -> Vec<&str> {
    handler_scope
        .split(|ch: char| matches!(ch, ',' | ';' | ' ' | '\t' | '\n' | '\r'))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .collect()
}

pub fn native_post_execution_readiness_report(
    real_handler_gate_enabled: bool,
    handler_scope: Option<&str>,
) -> NativePostExecutionReadinessResponse {
    let handler_scope = handler_scope
        .map(str::trim)
        .filter(|scope| !scope.is_empty())
        .map(str::to_string);
    let selected_handler_kinds =
        native_post_real_handler_scope_selected_kinds(handler_scope.as_deref());
    let selected_handler_count = selected_handler_kinds.len();
    let handler_scope_configured = handler_scope.is_some();
    let single_handler_scope_ready = selected_handler_count == 1;
    let routes = native_post_plan_route_specs()
        .iter()
        .map(native_post_execution_readiness_route)
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

    NativePostExecutionReadinessResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if all_evidence_contracts_ready {
            "ready"
        } else {
            "attention"
        },
        endpoint: NATIVE_POST_EXECUTION_READINESS_ENDPOINT,
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
        real_handler_gate_env: NATIVE_POST_REAL_HANDLERS_ENV,
        real_handler_gate_enabled,
        real_handler_scope_env: NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
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
        next_migration_slice: "activate the task-publish real-handler harness only under dual gate plus operator approval, then keep expanding one handler at a time",
    }
}

fn native_post_execution_readiness_route(
    spec: &NativePostPlanRouteSpec,
) -> NativePostExecutionReadinessRoute {
    let body_schema = native_post_body_schema(spec.plan_kind, false);
    let allowlisted_for_real_handler = spec.confirmation_required_for_real_mutation;
    let execution_evidence_contract_ready = true;
    let real_handler_implemented = native_post_plan_kind_has_real_handler(spec.plan_kind);
    NativePostExecutionReadinessRoute {
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

pub fn native_post_plan_report(
    spec: &NativePostPlanRouteSpec,
    parameter: Option<&str>,
    request_body: Option<&str>,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
    store_root: &Path,
    store_limits: NativePostExecutionStoreLimits,
) -> NativePostPlanResponse {
    let body_schema = native_post_body_schema(spec.plan_kind, request_body.is_some());
    let body_admission = native_post_body_admission(spec, &body_schema, request_body);
    let confirmation_contract = native_post_confirmation_contract(spec);
    let rollback_contract = native_post_rollback_contract();
    let mut idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let mut audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let execution_admission = native_post_execution_admission_with_scope(
        spec,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        real_handler_gate_enabled,
        operator_approval_enabled,
        handler_scope,
    );
    let real_handler_harness = native_post_real_handler_harness(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        &execution_admission,
        store_root,
        store_limits,
    );
    if real_handler_harness.duplicate_check_performed {
        idempotency_evidence.current_plan_lookup_performed = true;
    }
    if real_handler_harness.store_write_succeeded {
        idempotency_evidence.current_plan_store_written = true;
        audit_event_contract.current_plan_emits_audit_event = true;
        audit_event_contract.current_plan_persists_audit_event = true;
    }
    NativePostPlanResponse {
        product: "Hepta",
        runtime: "hepta-codex",
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
        parameter_present: parameter.is_some(),
        parameter_redacted: parameter.is_some(),
        parameter_length: parameter.map(str::len),
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
        next_migration_slice: "replace dry-run response with first real handler only after idempotency/audit/rollback stores are active",
    }
}

pub fn native_post_dispatch_plan_report(
    method: &str,
    path: &str,
    request_body: Option<&str>,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
    store_root: &Path,
    store_limits: NativePostExecutionStoreLimits,
) -> Option<NativePostPlanResponse> {
    if method != "POST" {
        return None;
    }

    native_post_plan_route_specs().iter().find_map(|spec| {
        native_post_plan_parameter(spec, path).map(|parameter| {
            native_post_plan_report(
                spec,
                parameter,
                request_body,
                real_handler_gate_enabled,
                operator_approval_enabled,
                handler_scope,
                store_root,
                store_limits,
            )
        })
    })
}

pub fn native_post_execution_stores_report(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> NativePostExecutionStoresResponse {
    let store_files =
        native_post_execution_store_file_statuses(root, max_store_bytes, max_store_lines);
    let root_exists = root.exists();
    let root_is_dir = root.is_dir();
    let existing_file_count = store_files.iter().filter(|file| file.exists).count();
    let total_bytes = store_files.iter().map(|file| file.bytes).sum::<u64>();
    let total_line_count = store_files.iter().map(|file| file.line_count).sum::<u64>();
    let valid_json_line_count = store_files
        .iter()
        .map(|file| file.valid_json_line_count)
        .sum::<u64>();
    let invalid_json_line_count = store_files
        .iter()
        .map(|file| file.invalid_json_line_count)
        .sum::<u64>();
    let store_jsonl_valid = store_files
        .iter()
        .all(|file| file.jsonl_readable && file.invalid_json_line_count == 0);
    let store_capacity_ok = store_files
        .iter()
        .all(|file| file.bytes_within_limit && file.line_count_within_limit);
    NativePostExecutionStoresResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if store_jsonl_valid && store_capacity_ok {
            "ready"
        } else {
            "attention"
        },
        endpoint: NATIVE_POST_EXECUTION_STORES_ENDPOINT,
        source_command: "/native-post-execution-stores --json",
        native_route: true,
        compatibility_mode: "native_post_execution_stores",
        side_effect_free: true,
        store_root_env: NATIVE_POST_EXECUTION_STORE_DIR_ENV,
        store_root: root.display().to_string(),
        root_exists,
        root_is_dir,
        store_file_count: store_files.len(),
        existing_file_count,
        max_store_bytes_env: NATIVE_POST_STORE_MAX_BYTES_ENV,
        max_store_bytes,
        max_store_lines_env: NATIVE_POST_STORE_MAX_LINES_ENV,
        max_store_lines,
        total_bytes,
        store_jsonl_valid,
        store_capacity_ok,
        total_line_count,
        valid_json_line_count,
        invalid_json_line_count,
        stores: store_files,
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

pub fn native_post_activation_plan_report(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
) -> NativePostActivationPlanResponse {
    let readiness =
        native_post_execution_readiness_report(real_handler_gate_enabled, handler_scope);
    let stores = native_post_execution_stores_report(root, max_store_bytes, max_store_lines);
    let handler_scope = handler_scope
        .map(str::trim)
        .filter(|scope| !scope.is_empty())
        .map(str::to_string);
    let selected_handler_kinds =
        native_post_real_handler_scope_selected_kinds(handler_scope.as_deref());
    let selected_handler_count = selected_handler_kinds.len();
    let handler_scope_configured = handler_scope.is_some();
    let single_handler_scope_ready = selected_handler_count == 1;
    let all_handlers_implemented =
        readiness.real_handler_implemented_count == readiness.real_handler_candidate_count;
    let store_contracts_ready = stores.persistence_implementation_ready
        && stores.idempotency_store_ready
        && stores.audit_store_ready
        && stores.rollback_store_ready
        && stores.rate_limit_store_ready
        && stores.store_jsonl_valid
        && stores.store_capacity_ok;
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
    let rollback_ready = activation_preflight_ready && stores.rollback_store_ready;

    NativePostActivationPlanResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if activation_preflight_ready {
            "ready"
        } else {
            "attention"
        },
        endpoint: NATIVE_POST_ACTIVATION_PLAN_ENDPOINT,
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
        handler_scope_env: NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
        handler_scope,
        handler_scope_configured,
        single_handler_scope_ready,
        selected_handler_count,
        selected_handler_kinds,
        execution_evidence_ready: readiness.all_evidence_contracts_ready,
        store_contracts_ready,
        store_jsonl_valid: stores.store_jsonl_valid,
        store_capacity_ok: stores.store_capacity_ok,
        required_gates: vec![
            NativePostActivationGate {
                env: NATIVE_POST_REAL_HANDLERS_ENV,
                enabled: real_handler_gate_enabled,
                required_for_activation: true,
                purpose: "allow native POST real-handler harness execution",
            },
            NativePostActivationGate {
                env: NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
                enabled: operator_approval_enabled,
                required_for_activation: true,
                purpose: "operator approval for confirm-required native POST mutations",
            },
            NativePostActivationGate {
                env: NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
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
        next_migration_slice: "activate one handler only under dual gate after this plan remains ready and rollback anchors are observed",
    }
}

pub fn native_post_rollout_evidence_report(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
    handler_scope: Option<&str>,
) -> NativePostRolloutEvidenceResponse {
    let store_files =
        native_post_execution_store_file_statuses(root, max_store_bytes, max_store_lines);
    let store_jsonl_valid = store_files
        .iter()
        .all(|file| file.jsonl_readable && file.invalid_json_line_count == 0);
    let store_capacity_ok = store_files
        .iter()
        .all(|file| file.bytes_within_limit && file.line_count_within_limit);
    let rollback_path = root.join("rollback.jsonl");
    let scan = native_post_rollout_evidence_scan(&rollback_path);
    let rollout_evidence_ready = store_jsonl_valid
        && store_capacity_ok
        && scan.jsonl_readable
        && scan.invalid_json_line_count == 0;
    let activation_scope = handler_scope
        .map(str::trim)
        .filter(|scope| !scope.is_empty())
        .map(str::to_string);
    let selected_handler_kinds =
        native_post_real_handler_scope_selected_kinds(activation_scope.as_deref());
    let selected_handler_count = selected_handler_kinds.len();

    NativePostRolloutEvidenceResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if rollout_evidence_ready {
            "ready"
        } else {
            "attention"
        },
        endpoint: NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT,
        source_command: "/native-post-rollout-evidence --json",
        native_route: true,
        compatibility_mode: "native_post_rollout_evidence",
        side_effect_free: true,
        store_root_env: NATIVE_POST_EXECUTION_STORE_DIR_ENV,
        store_root: root.display().to_string(),
        rollback_store_file: "rollback.jsonl",
        store_jsonl_valid,
        store_capacity_ok,
        rollout_evidence_ready,
        activation_scope_env: NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
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

pub fn native_post_gray_release_evidence_report(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
    handler_scope: Option<&str>,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
) -> NativePostGrayReleaseEvidenceResponse {
    let readiness =
        native_post_execution_readiness_report(real_handler_gate_enabled, handler_scope);
    let store_files =
        native_post_execution_store_file_statuses(root, max_store_bytes, max_store_lines);
    let store_jsonl_valid = store_files
        .iter()
        .all(|file| file.jsonl_readable && file.invalid_json_line_count == 0);
    let store_capacity_ok = store_files
        .iter()
        .all(|file| file.bytes_within_limit && file.line_count_within_limit);
    let store_contracts_ready = store_jsonl_valid && store_capacity_ok;
    let all_handlers_implemented =
        readiness.real_handler_implemented_count == readiness.real_handler_candidate_count;
    let activation_preflight_ready =
        readiness.all_evidence_contracts_ready && all_handlers_implemented && store_contracts_ready;
    let selected_handler_kinds = native_post_real_handler_scope_selected_kinds(handler_scope);
    let selected_handler_count = selected_handler_kinds.len();
    let single_handler_scope_ready = selected_handler_count == 1;
    let selected_handler_kind = single_handler_scope_ready.then(|| selected_handler_kinds[0]);
    let activation_currently_enabled = activation_preflight_ready
        && real_handler_gate_enabled
        && operator_approval_enabled
        && single_handler_scope_ready;
    let rollout_evidence =
        native_post_rollout_evidence_report(root, max_store_bytes, max_store_lines, handler_scope);
    let selected_handler_evidence = native_post_selected_handler_rollout_evidence(
        &root.join("rollback.jsonl"),
        selected_handler_kind,
    );
    let selected_handler_evidence_ready = selected_handler_evidence.dry_run_record_present
        && selected_handler_evidence.rollback_anchor_present
        && !selected_handler_evidence.raw_request_body_exposed
        && !selected_handler_evidence.raw_field_values_exposed
        && !selected_handler_evidence.raw_idempotency_key_exposed
        && !selected_handler_evidence.raw_audit_payload_exposed;
    let gray_release_evidence_ready = activation_preflight_ready
        && single_handler_scope_ready
        && rollout_evidence.rollout_evidence_ready
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

    NativePostGrayReleaseEvidenceResponse {
        product: "Hepta",
        runtime: "hepta-codex",
        status: if gray_release_ready {
            "ready"
        } else if activation_preflight_ready {
            "staged"
        } else {
            "attention"
        },
        endpoint: NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT,
        source_command: "/native-post-gray-release-evidence --json",
        native_route: true,
        compatibility_mode: "native_post_gray_release_evidence",
        side_effect_free: true,
        activation_plan_endpoint: NATIVE_POST_ACTIVATION_PLAN_ENDPOINT,
        rollout_evidence_endpoint: NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT,
        store_root_env: NATIVE_POST_EXECUTION_STORE_DIR_ENV,
        store_root: root.display().to_string(),
        handler_scope_env: NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
        handler_scope: handler_scope
            .map(str::trim)
            .filter(|scope| !scope.is_empty())
            .map(str::to_string),
        selected_handler_count,
        selected_handler_kinds,
        selected_handler_kind: selected_handler_kind.map(str::to_string),
        single_handler_scope_ready,
        real_handler_gate_env: NATIVE_POST_REAL_HANDLERS_ENV,
        real_handler_gate_enabled,
        operator_approval_env: NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
        operator_approval_enabled,
        activation_preflight_ready,
        activation_currently_enabled,
        store_jsonl_valid,
        store_capacity_ok,
        rollout_evidence_ready: rollout_evidence.rollout_evidence_ready,
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
        raw_request_body_exposed: rollout_evidence.raw_request_body_exposed,
        raw_field_values_exposed: rollout_evidence.raw_field_values_exposed,
        raw_idempotency_key_exposed: rollout_evidence.raw_idempotency_key_exposed,
        raw_audit_payload_exposed: rollout_evidence.raw_audit_payload_exposed,
        next_migration_slice: "run exactly one scoped POST dry-run canary and require rollback evidence before any real mutation wiring",
    }
}

pub fn native_post_real_handler_harness(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    execution_admission: &NativePostExecutionAdmission,
    store_root: &Path,
    store_limits: NativePostExecutionStoreLimits,
) -> NativePostRealHandlerHarness {
    let dual_gate_satisfied = execution_admission.enablement_gate_enabled
        && execution_admission.operator_approval_enabled;
    let duplicate_check_performed = execution_admission.current_plan_executes_real_handler
        && idempotency_evidence.key_fingerprint.is_some();
    let (duplicate_found, duplicate_check_error) = if duplicate_check_performed {
        match native_post_idempotency_duplicate_present(
            store_root,
            idempotency_evidence.key_fingerprint.as_deref(),
        ) {
            Ok(found) => (found, None),
            Err(_error) => (false, Some("native_post_idempotency_check_failed")),
        }
    } else {
        (false, None)
    };
    let duplicate_suppressed = duplicate_check_performed && duplicate_found;
    let rate_limit_check_performed = execution_admission.current_plan_executes_real_handler
        && !duplicate_suppressed
        && duplicate_check_error.is_none();
    let (rate_limited, rate_limit_check_error) = if rate_limit_check_performed {
        match native_post_rate_limit_recent_present(
            store_root,
            spec.plan_kind,
            store_limits.rate_limit_window_ms,
        ) {
            Ok(limited) => (limited, None),
            Err(_error) => (false, Some("native_post_rate_limit_check_failed")),
        }
    } else {
        (false, None)
    };
    let capacity_check_performed = execution_admission.current_plan_executes_real_handler
        && !duplicate_suppressed
        && duplicate_check_error.is_none()
        && !rate_limited
        && rate_limit_check_error.is_none();
    let pending_record = if capacity_check_performed {
        Some(native_post_execution_store_record(
            spec,
            body_schema,
            body_admission,
            idempotency_evidence,
            audit_event_contract,
            true,
        ))
    } else {
        None
    };
    let (store_capacity_ok, store_capacity_check_error) = if let Some(record) = &pending_record {
        match native_post_execution_store_capacity_allows_append_with_limits(
            store_root,
            record,
            store_limits.max_store_bytes,
            store_limits.max_store_lines,
        ) {
            Ok(ok) => (ok, None),
            Err(_error) => (false, Some("native_post_store_capacity_check_failed")),
        }
    } else {
        (true, None)
    };
    let store_write_attempted =
        capacity_check_performed && store_capacity_ok && store_capacity_check_error.is_none();
    let (store_write_succeeded, store_write_report, store_write_error) = if store_write_attempted {
        match persist_native_post_execution_store_record(
            store_root,
            pending_record
                .as_ref()
                .expect("pending record exists before store write"),
        ) {
            Ok(report) => (true, Some(report), None),
            Err(_error) => (
                false,
                None,
                Some("native_post_execution_store_write_failed"),
            ),
        }
    } else {
        (false, None, None)
    };
    NativePostRealHandlerHarness {
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
        enablement_gate_env: NATIVE_POST_REAL_HANDLERS_ENV,
        enablement_gate_enabled: execution_admission.enablement_gate_enabled,
        operator_approval_env: NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
        operator_approval_enabled: execution_admission.operator_approval_enabled,
        handler_scope_env: NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
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
        rate_limit_window_ms: store_limits.rate_limit_window_ms,
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

pub fn native_post_execution_store_record(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    current_plan_executes_real_handler: bool,
) -> NativePostExecutionStoreRecord {
    NativePostExecutionStoreRecord {
        schema_id: "hepta.post.execution_store_record.v1",
        recorded_at_unix_ms: native_post_now_unix_ms(),
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

pub fn native_post_execution_store_capacity_allows_append_with_limits(
    root: &Path,
    record: &NativePostExecutionStoreRecord,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> Result<bool, String> {
    let line = serde_json::to_string(record)
        .map_err(|error| format!("failed to serialize native POST execution record: {error}"))?;
    let projected_line_bytes = line.len() as u64 + 1;
    for spec in native_post_execution_store_specs() {
        let status =
            native_post_execution_store_file_status(root, spec, max_store_bytes, max_store_lines);
        if !status.jsonl_readable || !status.jsonl_valid {
            return Ok(false);
        }
        if status.bytes.saturating_add(projected_line_bytes) > max_store_bytes {
            return Ok(false);
        }
        if status.line_count.saturating_add(1) > max_store_lines {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn persist_native_post_execution_store_record(
    root: &Path,
    record: &NativePostExecutionStoreRecord,
) -> Result<NativePostExecutionStoreWriteReport, String> {
    fs::create_dir_all(root).map_err(|error| {
        format!(
            "failed to create native POST execution store root {}: {error}",
            root.display()
        )
    })?;
    let line = serde_json::to_string(record)
        .map_err(|error| format!("failed to serialize native POST execution record: {error}"))?;
    let mut written_files = Vec::new();
    for spec in native_post_execution_store_specs() {
        let path = root.join(spec.filename);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|error| {
                format!(
                    "failed to open native POST execution store {}: {error}",
                    path.display()
                )
            })?;
        writeln!(file, "{line}").map_err(|error| {
            format!(
                "failed to append native POST execution store {}: {error}",
                path.display()
            )
        })?;
        written_files.push(path.display().to_string());
    }
    Ok(NativePostExecutionStoreWriteReport {
        status: "written",
        root: root.display().to_string(),
        written_file_count: written_files.len(),
        written_files,
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
    })
}

fn native_post_idempotency_duplicate_present(
    root: &Path,
    key_fingerprint: Option<&str>,
) -> Result<bool, String> {
    let Some(key_fingerprint) = key_fingerprint else {
        return Ok(false);
    };
    let path = root.join("idempotency.jsonl");
    match fs::read_to_string(&path) {
        Ok(content) => Ok(content.lines().any(|line| line.contains(key_fingerprint))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(format!(
            "failed to read native POST idempotency store {}: {error}",
            path.display()
        )),
    }
}

fn native_post_rate_limit_recent_present(
    root: &Path,
    bucket: &str,
    window_ms: u64,
) -> Result<bool, String> {
    let path = root.join("rate-limit.jsonl");
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(format!(
                "failed to read native POST rate-limit store {}: {error}",
                path.display()
            ));
        }
    };
    let now_ms = native_post_now_unix_ms();
    for line in content.lines() {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        let Some(record_bucket) = value
            .get("rate_limit_bucket")
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if record_bucket != bucket {
            continue;
        }
        let Some(recorded_at_ms) = value
            .get("recorded_at_unix_ms")
            .and_then(serde_json::Value::as_u64)
        else {
            continue;
        };
        if now_ms.saturating_sub(recorded_at_ms) <= window_ms {
            return Ok(true);
        }
    }
    Ok(false)
}

struct NativePostExecutionStoreFileSpec {
    store_kind: &'static str,
    schema_id: &'static str,
    filename: &'static str,
}

fn native_post_execution_store_specs() -> &'static [NativePostExecutionStoreFileSpec] {
    &[
        NativePostExecutionStoreFileSpec {
            store_kind: "idempotency",
            schema_id: "hepta.post.idempotency_entry.v1",
            filename: "idempotency.jsonl",
        },
        NativePostExecutionStoreFileSpec {
            store_kind: "audit",
            schema_id: "hepta.post.execution_audit.v1",
            filename: "audit.jsonl",
        },
        NativePostExecutionStoreFileSpec {
            store_kind: "rollback",
            schema_id: "hepta.post.rollback_anchor.v1",
            filename: "rollback.jsonl",
        },
        NativePostExecutionStoreFileSpec {
            store_kind: "rate_limit",
            schema_id: "hepta.post.rate_limit_entry.v1",
            filename: "rate-limit.jsonl",
        },
    ]
}

fn native_post_execution_store_file_statuses(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> Vec<NativePostExecutionStoreFileStatus> {
    native_post_execution_store_specs()
        .iter()
        .map(|spec| {
            native_post_execution_store_file_status(root, spec, max_store_bytes, max_store_lines)
        })
        .collect()
}

fn native_post_execution_store_file_status(
    root: &Path,
    spec: &NativePostExecutionStoreFileSpec,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> NativePostExecutionStoreFileStatus {
    let path = root.join(spec.filename);
    let metadata = path.metadata().ok();
    let exists = metadata.as_ref().is_some_and(std::fs::Metadata::is_file);
    let (jsonl_readable, line_count, valid_json_line_count, invalid_json_line_count) =
        native_post_execution_store_jsonl_health(&path, exists);
    let bytes = metadata.as_ref().map(std::fs::Metadata::len).unwrap_or(0);
    NativePostExecutionStoreFileStatus {
        store_kind: spec.store_kind,
        schema_id: spec.schema_id,
        filename: spec.filename,
        path: path.display().to_string(),
        exists,
        bytes,
        max_bytes: max_store_bytes,
        bytes_within_limit: bytes <= max_store_bytes,
        append_only: true,
        jsonl: true,
        jsonl_readable,
        jsonl_valid: jsonl_readable && invalid_json_line_count == 0,
        line_count,
        max_lines: max_store_lines,
        line_count_within_limit: line_count <= max_store_lines,
        valid_json_line_count,
        invalid_json_line_count,
        raw_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
    }
}

fn native_post_execution_store_jsonl_health(path: &Path, exists: bool) -> (bool, u64, u64, u64) {
    if !exists {
        return (true, 0, 0, 0);
    }
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return (false, 0, 0, 0),
    };
    let mut line_count = 0_u64;
    let mut valid_json_line_count = 0_u64;
    let mut invalid_json_line_count = 0_u64;
    for line in content.lines() {
        line_count = line_count.saturating_add(1);
        if serde_json::from_str::<serde_json::Value>(line).is_ok() {
            valid_json_line_count = valid_json_line_count.saturating_add(1);
        } else {
            invalid_json_line_count = invalid_json_line_count.saturating_add(1);
        }
    }
    (
        true,
        line_count,
        valid_json_line_count,
        invalid_json_line_count,
    )
}

struct NativePostRolloutEvidenceScan {
    jsonl_readable: bool,
    read_error: Option<&'static str>,
    line_count: u64,
    valid_json_line_count: u64,
    invalid_json_line_count: u64,
    record_count: u64,
    dry_run_record_count: u64,
    rollback_anchor_count: u64,
    plan_kind_counts: Vec<NativePostRolloutEvidencePlanKindCount>,
    latest_record: Option<NativePostRolloutEvidenceRecordSummary>,
    raw_request_body_exposed: bool,
    raw_field_values_exposed: bool,
    raw_idempotency_key_exposed: bool,
    raw_audit_payload_exposed: bool,
}

fn native_post_rollout_evidence_scan(path: &Path) -> NativePostRolloutEvidenceScan {
    let mut line_count = 0_u64;
    let mut valid_json_line_count = 0_u64;
    let mut invalid_json_line_count = 0_u64;
    let mut record_count = 0_u64;
    let mut dry_run_record_count = 0_u64;
    let mut rollback_anchor_count = 0_u64;
    let mut plan_kind_counts = BTreeMap::<String, u64>::new();
    let mut latest_record: Option<NativePostRolloutEvidenceRecordSummary> = None;
    let mut latest_recorded_at = 0_u64;
    let mut raw_request_body_exposed = false;
    let mut raw_field_values_exposed = false;
    let mut raw_idempotency_key_exposed = false;
    let mut raw_audit_payload_exposed = false;

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return NativePostRolloutEvidenceScan {
                jsonl_readable: true,
                read_error: None,
                line_count,
                valid_json_line_count,
                invalid_json_line_count,
                record_count,
                dry_run_record_count,
                rollback_anchor_count,
                plan_kind_counts: Vec::new(),
                latest_record,
                raw_request_body_exposed,
                raw_field_values_exposed,
                raw_idempotency_key_exposed,
                raw_audit_payload_exposed,
            };
        }
        Err(_) => {
            return NativePostRolloutEvidenceScan {
                jsonl_readable: false,
                read_error: Some("rollback_store_read_failed"),
                line_count,
                valid_json_line_count,
                invalid_json_line_count,
                record_count,
                dry_run_record_count,
                rollback_anchor_count,
                plan_kind_counts: Vec::new(),
                latest_record,
                raw_request_body_exposed,
                raw_field_values_exposed,
                raw_idempotency_key_exposed,
                raw_audit_payload_exposed,
            };
        }
    };

    for line in content.lines() {
        line_count = line_count.saturating_add(1);
        let value = match serde_json::from_str::<serde_json::Value>(line) {
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
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        *plan_kind_counts.entry(plan_kind).or_insert(0) += 1;
        let current_plan_executes_real_handler =
            json_bool_field(&value, "current_plan_executes_real_handler");
        if current_plan_executes_real_handler {
            dry_run_record_count = dry_run_record_count.saturating_add(1);
        }
        if value
            .get("rollback_strategy")
            .and_then(serde_json::Value::as_str)
            == Some("pending_real_handler_rollback_anchor")
        {
            rollback_anchor_count = rollback_anchor_count.saturating_add(1);
        }
        raw_request_body_exposed |= json_bool_field(&value, "raw_request_body_exposed");
        raw_field_values_exposed |= json_bool_field(&value, "raw_field_values_exposed");
        raw_idempotency_key_exposed |= json_bool_field(&value, "raw_idempotency_key_exposed");
        raw_audit_payload_exposed |= json_bool_field(&value, "raw_audit_payload_exposed");

        let recorded_at = value
            .get("recorded_at_unix_ms")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);
        if latest_record.is_none() || recorded_at >= latest_recorded_at {
            latest_recorded_at = recorded_at;
            latest_record = Some(native_post_rollout_evidence_record_summary(&value));
        }
    }

    NativePostRolloutEvidenceScan {
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
            .map(|(plan_kind, count)| NativePostRolloutEvidencePlanKindCount { plan_kind, count })
            .collect(),
        latest_record,
        raw_request_body_exposed,
        raw_field_values_exposed,
        raw_idempotency_key_exposed,
        raw_audit_payload_exposed,
    }
}

fn native_post_rollout_evidence_record_summary(
    value: &serde_json::Value,
) -> NativePostRolloutEvidenceRecordSummary {
    NativePostRolloutEvidenceRecordSummary {
        recorded_at_unix_ms: value
            .get("recorded_at_unix_ms")
            .and_then(serde_json::Value::as_u64),
        route_pattern: json_string_field(value, "route_pattern"),
        capability: json_string_field(value, "capability"),
        plan_kind: json_string_field(value, "plan_kind"),
        body_schema_id: json_string_field(value, "body_schema_id"),
        body_admission_status: json_string_field(value, "body_admission_status"),
        rollback_strategy: json_string_field(value, "rollback_strategy"),
        rate_limit_bucket: json_string_field(value, "rate_limit_bucket"),
        current_plan_executes_real_handler: json_bool_field(
            value,
            "current_plan_executes_real_handler",
        ),
        idempotency_key_redacted: json_bool_field(value, "idempotency_key_redacted"),
        idempotency_key_fingerprint_present: value
            .get("idempotency_key_fingerprint")
            .and_then(serde_json::Value::as_str)
            .map(|fingerprint| !fingerprint.trim().is_empty())
            .unwrap_or(false),
        raw_request_body_exposed: json_bool_field(value, "raw_request_body_exposed"),
        raw_field_values_exposed: json_bool_field(value, "raw_field_values_exposed"),
        raw_idempotency_key_exposed: json_bool_field(value, "raw_idempotency_key_exposed"),
        raw_audit_payload_exposed: json_bool_field(value, "raw_audit_payload_exposed"),
    }
}

fn native_post_selected_handler_rollout_evidence(
    path: &Path,
    selected_handler_kind: Option<&str>,
) -> NativePostSelectedHandlerRolloutEvidence {
    let selected_handler_kind_string = selected_handler_kind.map(str::to_string);
    let mut record_count = 0_u64;
    let mut dry_run_record_count = 0_u64;
    let mut rollback_anchor_count = 0_u64;
    let mut latest_record: Option<NativePostRolloutEvidenceRecordSummary> = None;
    let mut latest_recorded_at = 0_u64;
    let mut raw_request_body_exposed = false;
    let mut raw_field_values_exposed = false;
    let mut raw_idempotency_key_exposed = false;
    let mut raw_audit_payload_exposed = false;

    let Some(selected_handler_kind) = selected_handler_kind else {
        return NativePostSelectedHandlerRolloutEvidence {
            selected_handler_kind: selected_handler_kind_string,
            record_count,
            dry_run_record_count,
            rollback_anchor_count,
            dry_run_record_present: false,
            rollback_anchor_present: false,
            latest_record,
            raw_request_body_exposed,
            raw_field_values_exposed,
            raw_idempotency_key_exposed,
            raw_audit_payload_exposed,
        };
    };

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            return NativePostSelectedHandlerRolloutEvidence {
                selected_handler_kind: selected_handler_kind_string,
                record_count,
                dry_run_record_count,
                rollback_anchor_count,
                dry_run_record_present: false,
                rollback_anchor_present: false,
                latest_record,
                raw_request_body_exposed,
                raw_field_values_exposed,
                raw_idempotency_key_exposed,
                raw_audit_payload_exposed,
            };
        }
    };

    for line in content.lines() {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if value.get("plan_kind").and_then(serde_json::Value::as_str) != Some(selected_handler_kind)
        {
            continue;
        }
        record_count = record_count.saturating_add(1);
        let current_plan_executes_real_handler =
            json_bool_field(&value, "current_plan_executes_real_handler");
        if current_plan_executes_real_handler {
            dry_run_record_count = dry_run_record_count.saturating_add(1);
        }
        if value
            .get("rollback_strategy")
            .and_then(serde_json::Value::as_str)
            == Some("pending_real_handler_rollback_anchor")
        {
            rollback_anchor_count = rollback_anchor_count.saturating_add(1);
        }
        raw_request_body_exposed |= json_bool_field(&value, "raw_request_body_exposed");
        raw_field_values_exposed |= json_bool_field(&value, "raw_field_values_exposed");
        raw_idempotency_key_exposed |= json_bool_field(&value, "raw_idempotency_key_exposed");
        raw_audit_payload_exposed |= json_bool_field(&value, "raw_audit_payload_exposed");

        let recorded_at = value
            .get("recorded_at_unix_ms")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);
        if latest_record.is_none() || recorded_at >= latest_recorded_at {
            latest_recorded_at = recorded_at;
            latest_record = Some(native_post_rollout_evidence_record_summary(&value));
        }
    }

    NativePostSelectedHandlerRolloutEvidence {
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

fn json_string_field(value: &serde_json::Value, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
}

fn json_bool_field(value: &serde_json::Value, field: &str) -> bool {
    value
        .get(field)
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

fn native_post_now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{
        native_post_plan_kind_has_real_handler, native_post_plan_parameter,
        native_post_plan_route_specs,
    };

    #[test]
    fn native_post_route_contracts_cover_real_handler_candidates() {
        let specs = native_post_plan_route_specs();

        for plan_kind in ["approval_apply", "task_publish", "chat_send"] {
            assert!(specs.iter().any(|spec| spec.plan_kind == plan_kind));
            assert!(native_post_plan_kind_has_real_handler(plan_kind));
        }
    }

    #[test]
    fn native_post_route_parameter_matches_exact_and_prefix_routes() {
        let specs = native_post_plan_route_specs();
        let action = specs
            .iter()
            .find(|spec| spec.pattern == "/api/actions/<action>")
            .expect("action spec should exist");
        assert_eq!(
            native_post_plan_parameter(action, "/api/actions/reload"),
            Some(Some("reload"))
        );

        let task_publish = specs
            .iter()
            .find(|spec| spec.pattern == "/api/tasks/publish")
            .expect("task publish spec should exist");
        assert_eq!(
            native_post_plan_parameter(task_publish, "/api/tasks/publish"),
            Some(None)
        );
    }

    #[test]
    fn native_post_body_admission_validates_redacted_real_handler_input() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission = super::native_post_body_admission(
            spec,
            &schema,
            Some(
                r#"{"task":"secret task","confirm":true,"dry_run":true,"idempotency_key":"secret-key"}"#,
            ),
        );

        assert_eq!(schema.schema_id, "hepta.post.task_publish.v1");
        assert_eq!(admission.admission_status, "ready_for_real_handler");
        assert_eq!(admission.ready_for_real_handler_input, true);
        assert_eq!(admission.idempotency_key_present, true);
        assert_eq!(admission.raw_body_exposed, false);
        assert_eq!(admission.raw_field_values_exposed, false);
        let fingerprint = admission
            .idempotency_key_fingerprint
            .as_deref()
            .expect("fingerprint");
        assert!(fingerprint.starts_with("sha256:"));
        assert!(!fingerprint.contains("secret-key"));
    }

    #[test]
    fn native_post_audit_contract_waits_for_valid_real_handler_input() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "chat_send")
            .expect("chat send spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission =
            super::native_post_body_admission(spec, &schema, Some(r#"{"chat_id":"c1"}"#));
        let idempotency = super::native_post_idempotency_evidence(spec, &admission);
        let audit =
            super::native_post_audit_event_contract(spec, &schema, &admission, &idempotency);

        assert_eq!(admission.admission_status, "missing_required_fields");
        assert_eq!(idempotency.key_shape_valid, false);
        assert_eq!(audit.ready_for_real_handler, false);
        assert_eq!(audit.raw_body_exposed, false);
        assert_eq!(audit.raw_idempotency_key_exposed, false);
    }

    #[test]
    fn native_post_execution_admission_requires_matching_scope() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "chat_send")
            .expect("chat send spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission = super::native_post_body_admission(
            spec,
            &schema,
            Some(
                r#"{"chat_id":"c1","message":"secret","confirm":true,"dry_run":true,"idempotency_key":"key"}"#,
            ),
        );
        let idempotency = super::native_post_idempotency_evidence(spec, &admission);
        let audit =
            super::native_post_audit_event_contract(spec, &schema, &admission, &idempotency);

        let mismatched = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish"),
        );
        assert_eq!(mismatched.admission_status, "blocked");
        assert_eq!(mismatched.blocked_reason, "handler_scope_not_selected");
        assert_eq!(mismatched.current_plan_executes_real_handler, false);

        let matched = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish, chat_send"),
        );
        assert_eq!(matched.admission_status, "harness_ready");
        assert_eq!(matched.current_plan_executes_real_handler, true);
        assert_eq!(matched.blocked_reason, "real_handler_harness_dry_run_only");
    }

    #[test]
    fn native_post_real_handler_scope_selection_uses_gateway_registry() {
        let selected =
            super::native_post_real_handler_scope_selected_kinds(Some("approval_apply chat_send"));

        assert_eq!(selected, vec!["approval_apply", "chat_send"]);
        assert!(super::native_post_real_handler_scope_matches(
            "chat_send",
            Some("task_publish,chat_send")
        ));
        assert!(!super::native_post_real_handler_scope_matches(
            "approval_apply",
            Some("task_publish,chat_send")
        ));
    }

    #[test]
    fn native_post_execution_readiness_report_is_gateway_owned() {
        let report =
            super::native_post_execution_readiness_report(false, Some("task_publish chat_send"));

        assert_eq!(report.status, "ready");
        assert_eq!(report.post_route_count, 12);
        assert_eq!(report.real_handler_candidate_count, 3);
        assert_eq!(report.real_handler_implemented_count, 3);
        assert_eq!(report.selected_handler_count, 2);
        assert_eq!(report.all_real_handlers_blocked, true);
        assert_eq!(report.real_handler_gate_enabled, false);
        assert!(report.routes.iter().any(|route| {
            route.plan_kind == "task_publish"
                && route.ready_for_real_handler_wiring
                && route.blocked_reason == "real_handler_gate_disabled"
        }));
    }

    #[test]
    fn native_post_real_handler_harness_persists_redacted_evidence_in_gateway() {
        let temp = tempfile::tempdir().expect("tempdir");
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission = super::native_post_body_admission(
            spec,
            &schema,
            Some(
                r#"{"task":"secret gateway task","confirm":true,"dry_run":true,"idempotency_key":"secret-gateway-idem"}"#,
            ),
        );
        let idempotency = super::native_post_idempotency_evidence(spec, &admission);
        let audit =
            super::native_post_audit_event_contract(spec, &schema, &admission, &idempotency);
        let execution = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish"),
        );

        let harness = super::native_post_real_handler_harness(
            spec,
            &schema,
            &admission,
            &idempotency,
            &audit,
            &execution,
            temp.path(),
            super::NativePostExecutionStoreLimits {
                max_store_bytes: super::DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
                max_store_lines: super::DEFAULT_NATIVE_POST_STORE_MAX_LINES,
                rate_limit_window_ms: super::DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS,
            },
        );

        assert_eq!(harness.status, "dry_run_recorded");
        assert_eq!(harness.store_write_attempted, true);
        assert_eq!(harness.store_write_succeeded, true);
        assert_eq!(harness.task_published, false);
        assert_eq!(harness.external_side_effects, false);
        for filename in [
            "idempotency.jsonl",
            "audit.jsonl",
            "rollback.jsonl",
            "rate-limit.jsonl",
        ] {
            let content =
                std::fs::read_to_string(temp.path().join(filename)).expect("store content");
            assert!(content.contains("hepta.post.execution_store_record.v1"));
            assert!(content.contains("\"current_plan_executes_real_handler\":true"));
            assert!(!content.contains("secret gateway task"));
            assert!(!content.contains("secret-gateway-idem"));
        }
    }

    #[test]
    fn native_post_real_handler_harness_suppresses_duplicate_in_gateway() {
        let temp = tempfile::tempdir().expect("tempdir");
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission = super::native_post_body_admission(
            spec,
            &schema,
            Some(
                r#"{"task":"secret duplicate gateway task","confirm":true,"dry_run":true,"idempotency_key":"secret-gateway-duplicate"}"#,
            ),
        );
        let idempotency = super::native_post_idempotency_evidence(spec, &admission);
        let audit =
            super::native_post_audit_event_contract(spec, &schema, &admission, &idempotency);
        let execution = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish"),
        );
        let limits = super::NativePostExecutionStoreLimits {
            max_store_bytes: super::DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
            max_store_lines: super::DEFAULT_NATIVE_POST_STORE_MAX_LINES,
            rate_limit_window_ms: super::DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS,
        };

        let first = super::native_post_real_handler_harness(
            spec,
            &schema,
            &admission,
            &idempotency,
            &audit,
            &execution,
            temp.path(),
            limits,
        );
        let second = super::native_post_real_handler_harness(
            spec,
            &schema,
            &admission,
            &idempotency,
            &audit,
            &execution,
            temp.path(),
            limits,
        );

        assert_eq!(first.status, "dry_run_recorded");
        assert_eq!(second.status, "duplicate_suppressed");
        assert_eq!(second.store_write_attempted, false);
        let idempotency_content =
            std::fs::read_to_string(temp.path().join("idempotency.jsonl")).expect("store");
        assert_eq!(idempotency_content.lines().count(), 1);
        assert!(!idempotency_content.contains("secret duplicate gateway task"));
        assert!(!idempotency_content.contains("secret-gateway-duplicate"));
    }
}
