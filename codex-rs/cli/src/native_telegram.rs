use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use codex_arg0::Arg0DispatchPaths;
use serde::Serialize;
use serde_json::Value;
use tokio::runtime::Handle;

const LEGACY_RUNTIME_SLUG: &str = "openclaw";
const LEGACY_CONFIG_FILE_NAME: &str = "openclaw.json";
const LOCAL_IMPORT_CONFIG_PATH: &str = ".hepta/local-import/private/config/openclaw.json";
const LOCAL_IMPORT_MANIFEST_PATH: &str = ".hepta/local-import/manifest.json";
const TELEGRAM_INGRESS_CURSOR_PATH: &str = ".hepta/telegram/ingress-drain-cursor.json";
const TELEGRAM_ALLOWED_UPDATES: &str =
    "[\"message\",\"edited_message\",\"callback_query\",\"message_reaction\"]";
pub(crate) const TELEGRAM_LIVE_READ_ENV: &str = "HEPTA_NATIVE_TELEGRAM_LIVE_READ";
pub(crate) const TELEGRAM_MODEL_TURN_GATE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_TURN";
pub(crate) const TELEGRAM_SEND_GATE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND";
pub(crate) const TELEGRAM_POLL_LOOP_ENV: &str = "HEPTA_NATIVE_TELEGRAM_POLL_LOOP";
pub(crate) const TELEGRAM_DELIVERY_APPROVED_ENV: &str = "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED";
pub(crate) const TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_IN_PROCESS_MODEL_RUNNER";
const TELEGRAM_MODEL_TIMEOUT_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_TIMEOUT_MS";
const TELEGRAM_MODEL_ENV: &str = "HEPTA_TELEGRAM_MODEL";
const HEPTA_DEFAULT_MODEL_ENV: &str = "HEPTA_DEFAULT_MODEL";
const TELEGRAM_MLX_BASE_URL_ENV: &str = "HEPTA_MLX_OPENAI_BASE_URL";
const DEFAULT_TELEGRAM_MLX_BASE_URL: &str = "http://127.0.0.1:11436/v1";
const TELEGRAM_MLX_MAX_TOKENS_ENV: &str = "HEPTA_MLX_TELEGRAM_MAX_TOKENS";
const DEFAULT_TELEGRAM_MLX_MAX_TOKENS: u64 = 512;
const MAX_TELEGRAM_MLX_MAX_TOKENS: u64 = 4096;
const TELEGRAM_TYPING_KEEPALIVE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_TYPING_KEEPALIVE";
const TELEGRAM_TYPING_KEEPALIVE_INTERVAL_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS";
const DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS: u64 = 4_000;
const MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS: u64 = 30_000;
const TELEGRAM_READ_MAX_ATTEMPTS_ENV: &str = "HEPTA_NATIVE_TELEGRAM_READ_MAX_ATTEMPTS";
const DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS: u64 = 1;
const MAX_TELEGRAM_READ_MAX_ATTEMPTS: u64 = 5;
const TELEGRAM_READ_RETRY_BACKOFF_ENV: &str = "HEPTA_NATIVE_TELEGRAM_READ_RETRY_BACKOFF_MS";
const DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS: u64 = 500;
const MAX_TELEGRAM_READ_RETRY_BACKOFF_MS: u64 = 30_000;
const TELEGRAM_SEND_MIN_INTERVAL_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND_MIN_INTERVAL_MS";
const MAX_TELEGRAM_SEND_MIN_INTERVAL_MS: u64 = 60_000;
const TELEGRAM_SEND_MAX_ATTEMPTS_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND_MAX_ATTEMPTS";
const DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS: u64 = 1;
const MAX_TELEGRAM_SEND_MAX_ATTEMPTS: u64 = 5;
const TELEGRAM_SEND_RETRY_BACKOFF_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND_RETRY_BACKOFF_MS";
const DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS: u64 = 700;
const MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS: u64 = 30_000;
const TELEGRAM_MODEL_FAILURE_FALLBACK_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK";
const DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS: u64 = 120_000;
const MAX_TELEGRAM_MODEL_TIMEOUT_MS: u64 = 600_000;
const TELEGRAM_SOAK_MIN_POLLS_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS";
const DEFAULT_TELEGRAM_SOAK_MIN_POLLS: u64 = 3;
const MAX_TELEGRAM_SOAK_MIN_POLLS: u64 = 10_000;
const TELEGRAM_SOAK_MAX_ATTENTION_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION";
const DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION: u64 = 0;
const MAX_TELEGRAM_SOAK_MAX_ATTENTION: u64 = 1_000;
const TELEGRAM_DRAIN_ONCE_STAGES: &[&str] = &[
    "receive_getUpdates",
    "duplicate_suppression",
    "model_turn",
    "sendMessage",
    "cursor_commit",
];
static TELEGRAM_SEND_RATE_LIMITS: OnceLock<Mutex<HashMap<i64, Instant>>> = OnceLock::new();
static TELEGRAM_LIVE_SOAK_OBSERVATION: OnceLock<Mutex<NativeTelegramLiveSoakObservationState>> =
    OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramPluginStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) in_process_supervisor_ready: bool,
    pub(crate) in_process_reply_loop_ready: bool,
    pub(crate) model_turn_bridge_ready: bool,
    pub(crate) bot_api_poll_ready: bool,
    pub(crate) bot_api_send_ready: bool,
    pub(crate) openclaw_gateway_runtime_dependency: bool,
    pub(crate) external_network_read: bool,
    pub(crate) external_send: bool,
    pub(crate) poll_ms: u64,
    pub(crate) allowed_updates: &'static str,
    pub(crate) config: NativeTelegramConfigStatus,
    pub(crate) transport_plan: NativeTelegramTransportPlan,
    pub(crate) ingress_parser: NativeTelegramIngressInspection,
    pub(crate) cursor_plan: NativeTelegramCursorPlan,
    pub(crate) model_turn_plan: NativeTelegramModelTurnPlan,
    pub(crate) migration_blocker: Option<&'static str>,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramConfigStatus {
    pub(crate) config_path: Option<String>,
    pub(crate) config_found: bool,
    pub(crate) enabled: bool,
    pub(crate) dm_policy: String,
    pub(crate) group_policy: String,
    pub(crate) allow_from_count: usize,
    pub(crate) group_count: usize,
    pub(crate) token_source: &'static str,
    pub(crate) token_secret_ref_present: bool,
    pub(crate) token_secret_provider: Option<String>,
    pub(crate) token_secret_id_present: bool,
    pub(crate) token_file_present: bool,
    pub(crate) token_file_mode_0600: bool,
    pub(crate) token_shape_ok: bool,
    pub(crate) raw_token_exposed: bool,
    pub(crate) binding_ready: bool,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramTransportPlan {
    pub(crate) bot_api_transport_plan_ready: bool,
    pub(crate) endpoint_template: &'static str,
    pub(crate) get_updates_method: &'static str,
    pub(crate) send_message_method: &'static str,
    pub(crate) send_chat_action_method: &'static str,
    pub(crate) allowed_updates: &'static str,
    pub(crate) offset_commit_strategy: &'static str,
    pub(crate) send_delivery_gate: &'static str,
    pub(crate) typing_keepalive_plan: &'static str,
    pub(crate) raw_token_exposed: bool,
    pub(crate) external_network_performed_by_status: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramIngressInspection {
    pub(crate) parser_ready: bool,
    pub(crate) update_count: usize,
    pub(crate) allowed_update_count: usize,
    pub(crate) latest_observed_update_id: Option<i64>,
    pub(crate) latest_allowed_update_id: Option<i64>,
    pub(crate) latest_allowed_next_update_offset: Option<i64>,
    pub(crate) latest_allowed_text_present: bool,
    pub(crate) message_count: usize,
    pub(crate) edited_message_count: usize,
    pub(crate) callback_query_count: usize,
    pub(crate) reaction_count: usize,
    pub(crate) raw_message_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_sender_id_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramCursorPlan {
    pub(crate) cursor_path: &'static str,
    pub(crate) duplicate_suppression_ready: bool,
    pub(crate) duplicate_suppression_rule_valid: bool,
    pub(crate) cursor_represents_next_update_offset: bool,
    pub(crate) commit_offset_after_delivery: bool,
    pub(crate) raw_update_payload_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramReceiveOnceStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) live_read_gate_env: &'static str,
    pub(crate) live_read_gate_enabled: bool,
    pub(crate) external_network_read: bool,
    pub(crate) external_send: bool,
    pub(crate) model_turn_started: bool,
    pub(crate) cursor_written: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_token_exposed: bool,
    pub(crate) limit: usize,
    pub(crate) get_updates_offset: Option<i64>,
    pub(crate) bot_api_ok: Option<bool>,
    pub(crate) local_next_update_offset: Option<i64>,
    pub(crate) config: NativeTelegramConfigStatus,
    pub(crate) transport_plan: NativeTelegramTransportPlan,
    pub(crate) cursor_plan: NativeTelegramCursorPlan,
    pub(crate) inspection: NativeTelegramIngressInspection,
    pub(crate) model_turn_plan: NativeTelegramModelTurnPlan,
    pub(crate) error: Option<String>,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramModelTurnPlanStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) model_turn_bridge_ready: bool,
    pub(crate) model_turn_started: bool,
    pub(crate) session_runner_invoked: bool,
    pub(crate) external_send: bool,
    pub(crate) cursor_written: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_sender_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
    pub(crate) config: NativeTelegramConfigStatus,
    pub(crate) cursor_plan: NativeTelegramCursorPlan,
    pub(crate) inspection: NativeTelegramIngressInspection,
    pub(crate) model_turn_plan: NativeTelegramModelTurnPlan,
    pub(crate) error: Option<String>,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramModelBridgeStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) model_turn_gate_env: &'static str,
    pub(crate) model_turn_gate_enabled: bool,
    pub(crate) send_gate_env: &'static str,
    pub(crate) model_turn_bridge_ready: bool,
    pub(crate) model_turn_started: bool,
    pub(crate) session_runner_invoked: bool,
    pub(crate) local_process_spawned: bool,
    pub(crate) external_network_read: bool,
    pub(crate) external_send: bool,
    pub(crate) cursor_written: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_sender_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
    pub(crate) config: NativeTelegramConfigStatus,
    pub(crate) cursor_plan: NativeTelegramCursorPlan,
    pub(crate) model_turn_plan: NativeTelegramModelTurnPlan,
    pub(crate) invocation_request: NativeTelegramModelInvocationRequestPlan,
    pub(crate) model_execution: NativeTelegramModelExecutionReport,
    pub(crate) bridge_plan: NativeTelegramSessionBridgePlan,
    pub(crate) error: Option<String>,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramSendPlanStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) send_gate_env: &'static str,
    pub(crate) send_gate_enabled: bool,
    pub(crate) bot_api_send_ready: bool,
    pub(crate) external_network_write: bool,
    pub(crate) external_send: bool,
    pub(crate) cursor_written: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
    pub(crate) raw_token_exposed: bool,
    pub(crate) config: NativeTelegramConfigStatus,
    pub(crate) transport_plan: NativeTelegramTransportPlan,
    pub(crate) send_plan: NativeTelegramSendPlan,
    pub(crate) send_request: NativeTelegramSendRequestPlan,
    pub(crate) error: Option<String>,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramDrainOnceStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) gates: NativeTelegramGatewayGateSummary,
    pub(crate) config: NativeTelegramConfigStatus,
    pub(crate) execution_plan: NativeTelegramExecutionPlan,
    pub(crate) cursor_plan: NativeTelegramCursorPlan,
    pub(crate) inspection: NativeTelegramIngressInspection,
    pub(crate) model_turn_plan: NativeTelegramModelTurnPlan,
    pub(crate) invocation_request: NativeTelegramModelInvocationRequestPlan,
    pub(crate) model_execution: NativeTelegramModelExecutionReport,
    pub(crate) send_plan: NativeTelegramSendPlan,
    pub(crate) send_request: NativeTelegramSendRequestPlan,
    pub(crate) send_execution: NativeTelegramSendExecutionReport,
    pub(crate) bot_api_ok: Option<bool>,
    pub(crate) local_next_update_offset: Option<i64>,
    pub(crate) get_updates_offset: Option<i64>,
    pub(crate) live_read_started: bool,
    pub(crate) model_turn_started: bool,
    pub(crate) send_started: bool,
    pub(crate) cursor_written: bool,
    pub(crate) external_network_read: bool,
    pub(crate) external_network_write: bool,
    pub(crate) external_send: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_token_exposed: bool,
    pub(crate) error: Option<String>,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramPollLoopStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) poll_loop_gate_env: &'static str,
    pub(crate) poll_loop_gate_enabled: bool,
    pub(crate) delivery_approval_gate_env: &'static str,
    pub(crate) delivery_approval_gate_enabled: bool,
    pub(crate) poll_ms: u64,
    pub(crate) drain_once_endpoint: &'static str,
    pub(crate) worker_spawned_by_status: bool,
    pub(crate) loop_invokes_drain_once: bool,
    pub(crate) requires_live_read_gate: &'static str,
    pub(crate) requires_model_turn_gate: &'static str,
    pub(crate) requires_send_gate: &'static str,
    pub(crate) requires_delivery_approval_gate: &'static str,
    pub(crate) external_network_read_by_status: bool,
    pub(crate) external_send_by_status: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_token_exposed: bool,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramLiveSoakStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) side_effect_free: bool,
    pub(crate) endpoint: &'static str,
    pub(crate) poll_loop_status: NativeTelegramPollLoopStatus,
    pub(crate) cursor_status: NativeTelegramCursorStatus,
    pub(crate) production_guards: NativeTelegramProductionGuardStatus,
    pub(crate) production_readiness: NativeTelegramProductionReadinessStatus,
    pub(crate) observation: NativeTelegramLiveSoakObservationReport,
    pub(crate) health_ready: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_token_exposed: bool,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramProductionGuardStatus {
    pub(crate) read_max_attempts_env: &'static str,
    pub(crate) read_max_attempts: u64,
    pub(crate) read_retry_backoff_env: &'static str,
    pub(crate) read_retry_backoff_ms: u64,
    pub(crate) retry_transient_read_errors: bool,
    pub(crate) typing_keepalive_env: &'static str,
    pub(crate) typing_keepalive_enabled: bool,
    pub(crate) typing_keepalive_interval_ms: u64,
    pub(crate) model_timeout_env: &'static str,
    pub(crate) model_timeout_ms: u64,
    pub(crate) model_failure_fallback_env: &'static str,
    pub(crate) model_failure_fallback_enabled: bool,
    pub(crate) send_min_interval_env: &'static str,
    pub(crate) send_min_interval_ms: u64,
    pub(crate) send_max_attempts_env: &'static str,
    pub(crate) send_max_attempts: u64,
    pub(crate) send_retry_backoff_env: &'static str,
    pub(crate) send_retry_backoff_ms: u64,
    pub(crate) retry_transient_send_errors: bool,
    pub(crate) rate_limit_scope: &'static str,
    pub(crate) raw_token_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramProductionReadinessStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) ready: bool,
    pub(crate) side_effect_free: bool,
    pub(crate) min_poll_iterations_env: &'static str,
    pub(crate) min_poll_iterations: u64,
    pub(crate) max_attention_count_env: &'static str,
    pub(crate) max_attention_count: u64,
    pub(crate) poll_loop_armed: bool,
    pub(crate) cursor_ready: bool,
    pub(crate) production_guards_ready: bool,
    pub(crate) observation_ready: bool,
    pub(crate) attention_budget_ok: bool,
    pub(crate) recent_bot_api_ok: bool,
    pub(crate) redaction_guards_ok: bool,
    pub(crate) readiness_blockers: Vec<&'static str>,
    pub(crate) readiness_warnings: Vec<&'static str>,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_token_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramLiveSoakObservationReport {
    pub(crate) poll_iterations: u64,
    pub(crate) drained_count: u64,
    pub(crate) busy_count: u64,
    pub(crate) attention_count: u64,
    pub(crate) empty_read_count: u64,
    pub(crate) model_turn_started_count: u64,
    pub(crate) send_started_count: u64,
    pub(crate) cursor_written_count: u64,
    pub(crate) external_send_count: u64,
    pub(crate) last_drained_at_unix_ms: Option<u64>,
    pub(crate) last_drained_next_update_offset: Option<i64>,
    pub(crate) last_observed_at_unix_ms: Option<u64>,
    pub(crate) last_status: Option<String>,
    pub(crate) last_error: Option<String>,
    pub(crate) last_bot_api_ok: Option<bool>,
    pub(crate) last_get_updates_offset: Option<i64>,
    pub(crate) last_local_next_update_offset: Option<i64>,
    pub(crate) last_update_count: usize,
    pub(crate) last_allowed_update_count: usize,
    pub(crate) last_model_turn_started: bool,
    pub(crate) last_send_started: bool,
    pub(crate) last_cursor_written: bool,
    pub(crate) last_external_send: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_token_exposed: bool,
}

#[derive(Debug, Clone, Default)]
struct NativeTelegramLiveSoakObservationState {
    poll_iterations: u64,
    drained_count: u64,
    busy_count: u64,
    attention_count: u64,
    empty_read_count: u64,
    model_turn_started_count: u64,
    send_started_count: u64,
    cursor_written_count: u64,
    external_send_count: u64,
    last_drained_at_unix_ms: Option<u64>,
    last_drained_next_update_offset: Option<i64>,
    last_observed_at_unix_ms: Option<u64>,
    last_status: Option<String>,
    last_error: Option<String>,
    last_bot_api_ok: Option<bool>,
    last_get_updates_offset: Option<i64>,
    last_local_next_update_offset: Option<i64>,
    last_update_count: usize,
    last_allowed_update_count: usize,
    last_model_turn_started: bool,
    last_send_started: bool,
    last_cursor_written: bool,
    last_external_send: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramCursorStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) cursor_path: &'static str,
    pub(crate) cursor_file_present: bool,
    pub(crate) cursor_parse_ok: bool,
    pub(crate) next_update_offset: Option<i64>,
    pub(crate) cursor_represents_next_update_offset: bool,
    pub(crate) duplicate_suppression_rule_valid: bool,
    pub(crate) cursor_write_policy: &'static str,
    pub(crate) cursor_written: bool,
    pub(crate) raw_update_payload_persisted: bool,
    pub(crate) error: Option<String>,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramGatewayGateSummary {
    pub(crate) delivery_approval_gate_env: &'static str,
    pub(crate) delivery_approval_gate_enabled: bool,
    pub(crate) live_read_gate_env: &'static str,
    pub(crate) live_read_gate_enabled: bool,
    pub(crate) model_turn_gate_env: &'static str,
    pub(crate) model_turn_gate_enabled: bool,
    pub(crate) send_gate_env: &'static str,
    pub(crate) send_gate_enabled: bool,
    pub(crate) readiness_summary_performs_live_read: bool,
    pub(crate) readiness_summary_invokes_model: bool,
    pub(crate) readiness_summary_sends_message: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramExecutionPlan {
    pub(crate) execution_plan_ready: bool,
    pub(crate) stages: &'static [&'static str],
    pub(crate) all_required_gates_enabled: bool,
    pub(crate) first_missing_gate: Option<&'static str>,
    pub(crate) receive_before_model: bool,
    pub(crate) send_after_model_success: bool,
    pub(crate) cursor_commit_after_delivery: bool,
    pub(crate) status_probe_executes_pipeline: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramDuplicateDecision {
    pub(crate) decision: &'static str,
    pub(crate) update_id: i64,
    pub(crate) current_next_update_offset: Option<i64>,
    pub(crate) candidate_next_update_offset: Option<i64>,
    pub(crate) already_drained: bool,
    pub(crate) should_invoke_model: bool,
    pub(crate) should_record_duplicate: bool,
    pub(crate) cursor_write_allowed_after_delivery: bool,
    pub(crate) raw_update_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramModelInvocationRequestPlan {
    pub(crate) request_builder_ready: bool,
    pub(crate) candidate_present: bool,
    pub(crate) candidate_kind: Option<String>,
    pub(crate) duplicate_decision: &'static str,
    pub(crate) prompt_material_in_memory: bool,
    pub(crate) prompt_material_serialized: bool,
    pub(crate) reply_target_available: bool,
    pub(crate) stable_session_key_ready: bool,
    pub(crate) should_invoke_model: bool,
    pub(crate) should_record_duplicate: bool,
    pub(crate) candidate_next_update_offset: Option<i64>,
    pub(crate) model_turn_gate_env: &'static str,
    pub(crate) model_turn_gate_enabled: bool,
    pub(crate) runner_invocation_allowed: bool,
    pub(crate) session_runner_invoked: bool,
    pub(crate) local_process_spawned: bool,
    pub(crate) external_send: bool,
    pub(crate) cursor_written: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_sender_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramModelExecutionReport {
    pub(crate) status: &'static str,
    pub(crate) execution_ready: bool,
    pub(crate) model_turn_gate_env: &'static str,
    pub(crate) model_turn_gate_enabled: bool,
    pub(crate) candidate_present: bool,
    pub(crate) prompt_material_present: bool,
    pub(crate) reply_target_available: bool,
    pub(crate) stable_session_key_ready: bool,
    pub(crate) candidate_next_update_offset: Option<i64>,
    pub(crate) runner_invocation_allowed: bool,
    pub(crate) session_runner_invoked: bool,
    pub(crate) local_process_spawned: bool,
    pub(crate) model_output_present: bool,
    pub(crate) external_send: bool,
    pub(crate) cursor_written: bool,
    pub(crate) raw_update_payload_exposed: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_sender_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramModelTurnPlan {
    pub(crate) planner_ready: bool,
    pub(crate) candidate_count: usize,
    pub(crate) text_candidate_count: usize,
    pub(crate) callback_candidate_count: usize,
    pub(crate) reaction_candidate_count: usize,
    pub(crate) reply_target_count: usize,
    pub(crate) candidate_kinds: Vec<String>,
    pub(crate) prompt_material_policy: &'static str,
    pub(crate) session_key_strategy: &'static str,
    pub(crate) reply_target_strategy: &'static str,
    pub(crate) model_turn_invocation_gate: &'static str,
    pub(crate) send_delivery_gate: &'static str,
    pub(crate) raw_message_text_exposed: bool,
    pub(crate) raw_callback_data_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_sender_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramSessionBridgePlan {
    pub(crate) bridge_plan_ready: bool,
    pub(crate) runner_kind: &'static str,
    pub(crate) runner_invocation_strategy: &'static str,
    pub(crate) prompt_material_policy: &'static str,
    pub(crate) session_key_strategy: &'static str,
    pub(crate) duplicate_policy: &'static str,
    pub(crate) cursor_commit_policy: &'static str,
    pub(crate) response_delivery_policy: &'static str,
    pub(crate) approval_policy: &'static str,
    pub(crate) failure_policy: &'static str,
    pub(crate) process_spawned_by_status: bool,
    pub(crate) raw_prompt_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_sender_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramSendPlan {
    pub(crate) send_plan_ready: bool,
    pub(crate) method: &'static str,
    pub(crate) request_builder_strategy: &'static str,
    pub(crate) response_source_policy: &'static str,
    pub(crate) reply_target_policy: &'static str,
    pub(crate) parse_mode_policy: &'static str,
    pub(crate) typing_keepalive_policy: &'static str,
    pub(crate) rate_limit_policy: &'static str,
    pub(crate) retry_policy: &'static str,
    pub(crate) cursor_commit_policy: &'static str,
    pub(crate) failure_policy: &'static str,
    pub(crate) request_body_materialized_by_status: bool,
    pub(crate) delivery_performed_by_status: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
    pub(crate) raw_token_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramSendRequestPlan {
    pub(crate) request_builder_ready: bool,
    pub(crate) model_output_present: bool,
    pub(crate) reply_target_available: bool,
    pub(crate) candidate_next_update_offset: Option<i64>,
    pub(crate) send_gate_env: &'static str,
    pub(crate) send_gate_enabled: bool,
    pub(crate) send_allowed: bool,
    pub(crate) request_body_materialized_by_status: bool,
    pub(crate) delivery_performed_by_status: bool,
    pub(crate) cursor_commit_allowed_after_delivery: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
    pub(crate) raw_token_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramSendExecutionReport {
    pub(crate) status: &'static str,
    pub(crate) execution_ready: bool,
    pub(crate) send_gate_env: &'static str,
    pub(crate) send_gate_enabled: bool,
    pub(crate) model_output_present: bool,
    pub(crate) reply_target_available: bool,
    pub(crate) candidate_next_update_offset: Option<i64>,
    pub(crate) send_allowed: bool,
    pub(crate) send_attempted: bool,
    pub(crate) bot_api_ack: Option<bool>,
    pub(crate) cursor_commit_attempted: bool,
    pub(crate) cursor_written: bool,
    pub(crate) request_body_materialized_by_execution: bool,
    pub(crate) external_network_write: bool,
    pub(crate) external_send: bool,
    pub(crate) raw_response_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_message_id_exposed: bool,
    pub(crate) raw_token_exposed: bool,
    pub(crate) error: Option<String>,
}

pub(crate) fn telegram_plugin_status(requested: bool, poll_ms: u64) -> NativeTelegramPluginStatus {
    if !requested {
        return NativeTelegramPluginStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested,
            status: "disabled",
            in_process_supervisor_ready: false,
            in_process_reply_loop_ready: false,
            model_turn_bridge_ready: false,
            bot_api_poll_ready: false,
            bot_api_send_ready: false,
            openclaw_gateway_runtime_dependency: false,
            external_network_read: false,
            external_send: false,
            poll_ms,
            allowed_updates: TELEGRAM_ALLOWED_UPDATES,
            config: NativeTelegramConfigStatus::disabled(),
            transport_plan: NativeTelegramTransportPlan::disabled(),
            ingress_parser: inspect_telegram_updates(&[]),
            cursor_plan: NativeTelegramCursorPlan::disabled(),
            model_turn_plan: NativeTelegramModelTurnPlan::disabled(),
            migration_blocker: None,
            next_migration_slice: "enable --with-telegram-plugin, then wire Bot API polling and model-turn delivery",
        };
    }

    let config = load_telegram_config_status();
    let supervisor_ready = config.error.is_none();
    let config_ready = config.enabled && config.token_shape_ok && config.binding_ready;
    let gate_summary = telegram_gateway_gate_summary();
    let bot_api_poll_ready = config_ready && gate_summary.live_read_gate_enabled;
    let model_turn_bridge_ready = config_ready && gate_summary.model_turn_gate_enabled;
    let bot_api_send_ready = config_ready && gate_summary.send_gate_enabled;
    let in_process_reply_loop_ready = bot_api_poll_ready
        && model_turn_bridge_ready
        && bot_api_send_ready
        && gate_summary.delivery_approval_gate_enabled
        && env_truthy(TELEGRAM_POLL_LOOP_ENV);
    let migration_blocker = if in_process_reply_loop_ready {
        None
    } else {
        Some(
            "enable live read, model, send, poll loop, and delivery approval gates before active reply-loop delivery",
        )
    };
    let next_migration_slice = if in_process_reply_loop_ready {
        "keep active Telegram live soak green and inspect /api/telegram-live-soak-status for cumulative delivery evidence"
    } else {
        "wire native Bot API getUpdates/sendMessage loop behind explicit delivery gates"
    };

    let status = if supervisor_ready && config_ready {
        "native_supervisor_ready"
    } else {
        "attention"
    };

    NativeTelegramPluginStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        in_process_supervisor_ready: supervisor_ready,
        in_process_reply_loop_ready,
        model_turn_bridge_ready,
        bot_api_poll_ready,
        bot_api_send_ready,
        openclaw_gateway_runtime_dependency: false,
        external_network_read: false,
        external_send: false,
        poll_ms,
        allowed_updates: TELEGRAM_ALLOWED_UPDATES,
        transport_plan: NativeTelegramTransportPlan::for_config(&config),
        config,
        ingress_parser: inspect_telegram_updates(&[]),
        cursor_plan: NativeTelegramCursorPlan::ready(),
        model_turn_plan: plan_model_turn_for_updates(&[]),
        migration_blocker,
        next_migration_slice,
    }
}

pub(crate) fn telegram_receive_once_status(
    requested: bool,
    limit: usize,
) -> NativeTelegramReceiveOnceStatus {
    telegram_receive_once_status_with_gate(requested, limit, env_truthy(TELEGRAM_LIVE_READ_ENV))
}

pub(crate) fn telegram_gateway_gate_summary() -> NativeTelegramGatewayGateSummary {
    NativeTelegramGatewayGateSummary {
        delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
        delivery_approval_gate_enabled: env_truthy(TELEGRAM_DELIVERY_APPROVED_ENV),
        live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
        live_read_gate_enabled: env_truthy(TELEGRAM_LIVE_READ_ENV),
        model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
        model_turn_gate_enabled: env_truthy(TELEGRAM_MODEL_TURN_GATE_ENV),
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        send_gate_enabled: env_truthy(TELEGRAM_SEND_GATE_ENV),
        readiness_summary_performs_live_read: false,
        readiness_summary_invokes_model: false,
        readiness_summary_sends_message: false,
    }
}

pub(crate) fn telegram_model_turn_plan_status(
    requested: bool,
) -> NativeTelegramModelTurnPlanStatus {
    let config = if requested {
        load_telegram_config_status()
    } else {
        NativeTelegramConfigStatus::disabled()
    };
    let cursor_plan = if requested {
        NativeTelegramCursorPlan::ready()
    } else {
        NativeTelegramCursorPlan::disabled()
    };
    let inspection = inspect_telegram_updates(&[]);
    let model_turn_plan = if requested {
        plan_model_turn_for_updates(&[])
    } else {
        NativeTelegramModelTurnPlan::disabled()
    };
    let config_ready = requested && config.enabled && config.token_shape_ok && config.binding_ready;
    let status = if !requested {
        "disabled"
    } else if config_ready {
        "planned"
    } else {
        "attention"
    };
    let error = if requested && !config_ready {
        Some("Telegram config, token shape, or binding is not ready".to_string())
    } else {
        None
    };

    NativeTelegramModelTurnPlanStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        model_turn_bridge_ready: false,
        model_turn_started: false,
        session_runner_invoked: false,
        external_send: false,
        cursor_written: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_chat_id_exposed: false,
        raw_sender_id_exposed: false,
        raw_message_id_exposed: false,
        config,
        cursor_plan,
        inspection,
        model_turn_plan,
        error,
        next_migration_slice: "wire the planned redacted candidates into a bounded Codex session runner",
    }
}

pub(crate) fn telegram_model_bridge_status(requested: bool) -> NativeTelegramModelBridgeStatus {
    telegram_model_bridge_status_with_gate(requested, env_truthy(TELEGRAM_MODEL_TURN_GATE_ENV))
}

fn telegram_model_bridge_status_with_gate(
    requested: bool,
    model_turn_gate_enabled: bool,
) -> NativeTelegramModelBridgeStatus {
    let config = if requested {
        load_telegram_config_status()
    } else {
        NativeTelegramConfigStatus::disabled()
    };
    let cursor_plan = if requested {
        NativeTelegramCursorPlan::ready()
    } else {
        NativeTelegramCursorPlan::disabled()
    };
    let model_turn_plan = if requested {
        plan_model_turn_for_updates(&[])
    } else {
        NativeTelegramModelTurnPlan::disabled()
    };
    let invocation_request = if requested {
        build_model_invocation_request_plan(&[], None, model_turn_gate_enabled)
    } else {
        NativeTelegramModelInvocationRequestPlan::disabled(model_turn_gate_enabled)
    };
    let model_execution = if requested {
        NativeTelegramModelExecutionReport::from_invocation_request(&invocation_request)
    } else {
        NativeTelegramModelExecutionReport::disabled(model_turn_gate_enabled)
    };
    let bridge_plan = if requested {
        NativeTelegramSessionBridgePlan::ready(telegram_in_process_model_runner_enabled())
    } else {
        NativeTelegramSessionBridgePlan::disabled()
    };
    let config_ready = requested && config.enabled && config.token_shape_ok && config.binding_ready;
    let status = if !requested {
        "disabled"
    } else if !model_turn_gate_enabled {
        "gated"
    } else if config_ready {
        "planned"
    } else {
        "attention"
    };
    let error = if requested && !model_turn_gate_enabled {
        Some(format!(
            "Telegram model-turn bridge is gated; set {TELEGRAM_MODEL_TURN_GATE_ENV}=1 only after runner invocation wiring is ready"
        ))
    } else if requested && !config_ready {
        Some("Telegram config, token shape, or binding is not ready".to_string())
    } else {
        None
    };

    NativeTelegramModelBridgeStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
        model_turn_gate_enabled,
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        model_turn_bridge_ready: requested && model_turn_gate_enabled && config_ready,
        model_turn_started: false,
        session_runner_invoked: false,
        local_process_spawned: false,
        external_network_read: false,
        external_send: false,
        cursor_written: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_chat_id_exposed: false,
        raw_sender_id_exposed: false,
        raw_message_id_exposed: false,
        config,
        cursor_plan,
        model_turn_plan,
        invocation_request,
        model_execution,
        bridge_plan,
        error,
        next_migration_slice: "implement the gated session-runner invocation and keep Telegram send behind HEPTA_NATIVE_TELEGRAM_SEND",
    }
}

pub(crate) fn telegram_send_plan_status(requested: bool) -> NativeTelegramSendPlanStatus {
    telegram_send_plan_status_with_gate(requested, env_truthy(TELEGRAM_SEND_GATE_ENV))
}

pub(crate) fn telegram_drain_once_status(requested: bool) -> NativeTelegramDrainOnceStatus {
    telegram_drain_once_status_with_gates(requested, telegram_gateway_gate_summary())
}

pub(crate) fn telegram_poll_loop_status(
    requested: bool,
    poll_ms: u64,
) -> NativeTelegramPollLoopStatus {
    let poll_loop_gate_enabled = env_truthy(TELEGRAM_POLL_LOOP_ENV);
    let delivery_approval_gate_enabled = env_truthy(TELEGRAM_DELIVERY_APPROVED_ENV);
    let status = if !requested {
        "disabled"
    } else if poll_loop_gate_enabled && delivery_approval_gate_enabled {
        "armed"
    } else if poll_loop_gate_enabled {
        "approval_required"
    } else {
        "gated"
    };
    NativeTelegramPollLoopStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        poll_loop_gate_env: TELEGRAM_POLL_LOOP_ENV,
        poll_loop_gate_enabled,
        delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
        delivery_approval_gate_enabled,
        poll_ms,
        drain_once_endpoint: "/api/telegram-drain-once",
        worker_spawned_by_status: false,
        loop_invokes_drain_once: requested
            && poll_loop_gate_enabled
            && delivery_approval_gate_enabled,
        requires_live_read_gate: TELEGRAM_LIVE_READ_ENV,
        requires_model_turn_gate: TELEGRAM_MODEL_TURN_GATE_ENV,
        requires_send_gate: TELEGRAM_SEND_GATE_ENV,
        requires_delivery_approval_gate: TELEGRAM_DELIVERY_APPROVED_ENV,
        external_network_read_by_status: false,
        external_send_by_status: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
        next_migration_slice: "continue live soak and inspect /api/telegram-live-soak for production guard health",
    }
}

pub(crate) fn telegram_live_soak_status(
    requested: bool,
    poll_ms: u64,
) -> NativeTelegramLiveSoakStatus {
    let poll_loop_status = telegram_poll_loop_status(requested, poll_ms);
    let cursor_status = telegram_cursor_status(requested);
    let observation = telegram_live_soak_observation_report();
    let production_guards = telegram_production_guard_status();
    let production_readiness = telegram_production_readiness_status_from_parts(
        requested,
        &poll_loop_status,
        &cursor_status,
        &production_guards,
        &observation,
    );
    let last_status = observation.last_status.as_deref();
    let status = if !requested {
        "disabled"
    } else if !poll_loop_status.loop_invokes_drain_once {
        "gated"
    } else if cursor_status.status == "attention"
        || last_status == Some("attention")
        || !production_readiness.attention_budget_ok
    {
        "attention"
    } else if observation.poll_iterations == 0 {
        "warming"
    } else if !production_readiness.production_guards_ready {
        "attention"
    } else {
        "soaking"
    };
    let health_ready = production_readiness.ready;

    NativeTelegramLiveSoakStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        side_effect_free: true,
        endpoint: "/api/telegram-live-soak",
        poll_loop_status,
        cursor_status,
        production_guards,
        production_readiness,
        observation,
        health_ready,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
        next_migration_slice: "keep the active gateway soaking; use this endpoint plus logs before broadening traffic or reducing guards",
    }
}

pub(crate) fn telegram_production_readiness_status(
    requested: bool,
    poll_ms: u64,
) -> NativeTelegramProductionReadinessStatus {
    let poll_loop_status = telegram_poll_loop_status(requested, poll_ms);
    let cursor_status = telegram_cursor_status(requested);
    let production_guards = telegram_production_guard_status();
    let observation = telegram_live_soak_observation_report();
    telegram_production_readiness_status_from_parts(
        requested,
        &poll_loop_status,
        &cursor_status,
        &production_guards,
        &observation,
    )
}

fn telegram_production_readiness_status_from_parts(
    requested: bool,
    poll_loop_status: &NativeTelegramPollLoopStatus,
    cursor_status: &NativeTelegramCursorStatus,
    production_guards: &NativeTelegramProductionGuardStatus,
    observation: &NativeTelegramLiveSoakObservationReport,
) -> NativeTelegramProductionReadinessStatus {
    let min_poll_iterations = telegram_soak_min_poll_iterations();
    let max_attention_count = telegram_soak_max_attention_count();
    let poll_loop_armed =
        requested && poll_loop_status.status == "armed" && poll_loop_status.loop_invokes_drain_once;
    let cursor_ready = cursor_status.status == "ready"
        && cursor_status.cursor_parse_ok
        && cursor_status.duplicate_suppression_rule_valid;
    let production_guards_ready = production_guards.typing_keepalive_enabled
        && production_guards.model_failure_fallback_enabled
        && production_guards.model_timeout_ms >= 1_000
        && production_guards.read_max_attempts >= 1
        && production_guards.send_max_attempts >= 1
        && production_guards.send_min_interval_ms > 0
        && production_guards.retry_transient_read_errors
        && production_guards.retry_transient_send_errors
        && !production_guards.raw_token_exposed;
    let observation_ready = observation.poll_iterations >= min_poll_iterations
        && observation.last_observed_at_unix_ms.is_some();
    let attention_budget_ok = observation.attention_count <= max_attention_count
        && observation.last_status.as_deref() != Some("attention");
    let recent_bot_api_ok = observation.last_bot_api_ok != Some(false);
    let redaction_guards_ok = !observation.raw_update_payload_exposed
        && !observation.raw_prompt_text_exposed
        && !observation.raw_response_text_exposed
        && !observation.raw_token_exposed
        && !poll_loop_status.raw_update_payload_exposed
        && !poll_loop_status.raw_prompt_text_exposed
        && !poll_loop_status.raw_response_text_exposed
        && !poll_loop_status.raw_token_exposed;

    let mut readiness_blockers = Vec::new();
    if !requested {
        readiness_blockers.push("telegram_plugin_not_requested");
    }
    if !poll_loop_armed {
        readiness_blockers.push("poll_loop_not_armed");
    }
    if !cursor_ready {
        readiness_blockers.push("cursor_not_ready");
    }
    if !production_guards_ready {
        readiness_blockers.push("production_guards_not_ready");
    }
    if !observation_ready {
        readiness_blockers.push("observation_min_poll_iterations");
    }
    if !attention_budget_ok {
        readiness_blockers.push("attention_budget_exceeded");
    }
    if !recent_bot_api_ok {
        readiness_blockers.push("bot_api_recent_failure");
    }
    if !redaction_guards_ok {
        readiness_blockers.push("redaction_guard_failed");
    }

    let mut readiness_warnings = Vec::new();
    if observation.busy_count > 0 {
        readiness_warnings.push("getupdates_busy_conflicts_observed");
    }
    if observation.drained_count == 0 {
        readiness_warnings.push("no_messages_drained_since_gateway_start");
    }
    if observation.external_send_count > observation.cursor_written_count {
        readiness_warnings.push("send_count_exceeds_cursor_write_count");
    }

    let ready = readiness_blockers.is_empty();
    let status = if !requested {
        "disabled"
    } else if !poll_loop_armed || !cursor_ready {
        "gated"
    } else if !attention_budget_ok || !recent_bot_api_ok || !redaction_guards_ok {
        "attention"
    } else if !observation_ready {
        "warming"
    } else if ready {
        "ready"
    } else {
        "attention"
    };

    NativeTelegramProductionReadinessStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        ready,
        side_effect_free: true,
        min_poll_iterations_env: TELEGRAM_SOAK_MIN_POLLS_ENV,
        min_poll_iterations,
        max_attention_count_env: TELEGRAM_SOAK_MAX_ATTENTION_ENV,
        max_attention_count,
        poll_loop_armed,
        cursor_ready,
        production_guards_ready,
        observation_ready,
        attention_budget_ok,
        recent_bot_api_ok,
        redaction_guards_ok,
        readiness_blockers,
        readiness_warnings,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
    }
}

pub(crate) fn spawn_telegram_poll_loop_if_enabled(
    requested: bool,
    poll_ms: u64,
) -> Option<thread::JoinHandle<()>> {
    if !(requested
        && env_truthy(TELEGRAM_POLL_LOOP_ENV)
        && env_truthy(TELEGRAM_DELIVERY_APPROVED_ENV))
    {
        return None;
    }

    Some(thread::spawn(move || {
        run_telegram_poll_loop(requested, poll_ms)
    }))
}

fn run_telegram_poll_loop(requested: bool, poll_ms: u64) {
    let poll_ms = poll_ms.clamp(500, 60_000);
    loop {
        let status = telegram_drain_once_status(requested);
        observe_telegram_live_soak(&status);
        if matches!(status.status, "attention") {
            eprintln!(
                "hepta-codex Telegram poll loop attention: {}",
                status
                    .error
                    .as_deref()
                    .map(redact_token_like_text)
                    .unwrap_or_else(|| "unknown redacted error".to_string())
            );
        }
        thread::sleep(Duration::from_millis(poll_ms));
    }
}

pub(crate) fn telegram_cursor_status(requested: bool) -> NativeTelegramCursorStatus {
    if !requested {
        return NativeTelegramCursorStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested,
            status: "disabled",
            cursor_path: TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present: false,
            cursor_parse_ok: false,
            next_update_offset: None,
            cursor_represents_next_update_offset: true,
            duplicate_suppression_rule_valid: true,
            cursor_write_policy: "disabled",
            cursor_written: false,
            raw_update_payload_persisted: false,
            error: None,
            next_migration_slice: "enable Telegram plugin before reading cursor state",
        };
    }

    telegram_cursor_status_from_path(Path::new(TELEGRAM_INGRESS_CURSOR_PATH))
}

fn telegram_cursor_status_from_path(path: &Path) -> NativeTelegramCursorStatus {
    let cursor_file_present = path.is_file();
    let mut status = NativeTelegramCursorStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: true,
        status: "missing",
        cursor_path: TELEGRAM_INGRESS_CURSOR_PATH,
        cursor_file_present,
        cursor_parse_ok: false,
        next_update_offset: None,
        cursor_represents_next_update_offset: true,
        duplicate_suppression_rule_valid: telegram_update_already_drained(41, Some(42))
            && !telegram_update_already_drained(42, Some(42)),
        cursor_write_policy: "write only after model output is delivered or duplicate suppression is recorded",
        cursor_written: false,
        raw_update_payload_persisted: false,
        error: None,
        next_migration_slice: "wire cursor write after gated send delivery success",
    };

    if !cursor_file_present {
        return status;
    }

    match fs::read_to_string(path)
        .map_err(|error| format!("failed to read Telegram cursor file: {error}"))
        .and_then(|raw| parse_telegram_cursor_next_update_offset(&raw))
    {
        Ok(next_update_offset) => {
            status.status = "ready";
            status.cursor_parse_ok = true;
            status.next_update_offset = Some(next_update_offset);
            status.next_migration_slice = "cursor is ready; continue active soak and expect writes only after delivery or duplicate suppression";
        }
        Err(error) => {
            status.status = "attention";
            status.error = Some(redact_token_like_text(&error));
        }
    }

    status
}

fn parse_telegram_cursor_next_update_offset(raw: &str) -> Result<i64, String> {
    let value: Value = serde_json::from_str(raw)
        .map_err(|error| format!("failed to parse Telegram cursor JSON: {error}"))?;
    let explicit_next_update_offset = value
        .get("next_update_offset")
        .or_else(|| value.get("nextUpdateOffset"))
        .and_then(Value::as_i64)
        .or_else(|| {
            value
                .get("next_server_offset")
                .or_else(|| value.get("nextServerOffset"))
                .and_then(Value::as_i64)
        });
    let legacy_last_drained_next_offset = value
        .get("last_drained_update_id")
        .or_else(|| value.get("lastDrainedUpdateId"))
        .and_then(Value::as_i64)
        .filter(|offset| *offset >= 0)
        .and_then(|offset| offset.checked_add(1));
    let offset = explicit_next_update_offset
        .or(legacy_last_drained_next_offset)
        .ok_or_else(|| {
            "Telegram cursor missing next_update_offset or legacy next_server_offset".to_string()
        })?;
    if offset < 0 {
        Err("Telegram cursor next_update_offset must be non-negative".to_string())
    } else {
        Ok(offset)
    }
}

fn write_telegram_cursor_next_update_offset(path: &Path, offset: i64) -> Result<(), String> {
    if offset < 0 {
        return Err("Telegram cursor next_update_offset must be non-negative".to_string());
    }
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create Telegram cursor directory: {error}"))?;
    }
    let body = serde_json::json!({
        "schema": "hepta.telegram.cursor.v1",
        "next_update_offset": offset,
        "raw_update_payload_persisted": false,
    });
    let raw = serde_json::to_string_pretty(&body)
        .map_err(|error| format!("failed to encode Telegram cursor JSON: {error}"))?;
    fs::write(path, format!("{raw}\n"))
        .map_err(|error| format!("failed to write Telegram cursor file: {error}"))
}

fn telegram_drain_once_status_with_gates(
    requested: bool,
    gates: NativeTelegramGatewayGateSummary,
) -> NativeTelegramDrainOnceStatus {
    let config = if requested {
        load_telegram_config_status()
    } else {
        NativeTelegramConfigStatus::disabled()
    };
    let cursor_plan = if requested {
        NativeTelegramCursorPlan::ready()
    } else {
        NativeTelegramCursorPlan::disabled()
    };
    let mut inspection = inspect_telegram_updates(&[]);
    let mut model_turn_plan = if requested {
        plan_model_turn_for_updates(&[])
    } else {
        NativeTelegramModelTurnPlan::disabled()
    };
    let mut invocation_request = if requested {
        build_model_invocation_request_plan(&[], None, gates.model_turn_gate_enabled)
    } else {
        NativeTelegramModelInvocationRequestPlan::disabled(gates.model_turn_gate_enabled)
    };
    let send_plan = if requested {
        NativeTelegramSendPlan::ready()
    } else {
        NativeTelegramSendPlan::disabled()
    };
    let mut send_request = if requested {
        build_telegram_send_request_plan(None, false, None, gates.send_gate_enabled)
    } else {
        NativeTelegramSendRequestPlan::disabled(gates.send_gate_enabled)
    };
    let mut send_execution = if requested {
        NativeTelegramSendExecutionReport::from_send_request(&send_request)
    } else {
        NativeTelegramSendExecutionReport::disabled(gates.send_gate_enabled)
    };
    let mut model_execution = if requested {
        NativeTelegramModelExecutionReport::from_invocation_request(&invocation_request)
    } else {
        NativeTelegramModelExecutionReport::disabled(gates.model_turn_gate_enabled)
    };
    let first_missing_gate = first_missing_drain_once_gate(&gates);
    let all_required_gates_enabled = requested && first_missing_gate.is_none();
    let status_probe_executes_pipeline =
        requested && gates.delivery_approval_gate_enabled && gates.live_read_gate_enabled;
    let mut status = if !requested {
        "disabled"
    } else if all_required_gates_enabled {
        "planned"
    } else {
        "gated"
    };
    let mut error = if requested {
        first_missing_gate.map(|gate| {
            format!(
                "Telegram drain-once pipeline is gated before side effects; first missing gate: {gate}"
            )
        })
    } else {
        None
    };
    let mut bot_api_ok = None;
    let mut local_next_update_offset = None;
    let mut get_updates_offset = None;
    let mut live_read_started = false;
    let mut external_network_read = false;

    if status_probe_executes_pipeline {
        let cursor_status =
            telegram_cursor_status_from_path(Path::new(TELEGRAM_INGRESS_CURSOR_PATH));
        get_updates_offset = cursor_status.next_update_offset;
        if cursor_status.cursor_file_present && !cursor_status.cursor_parse_ok {
            status = "attention";
            error = Some(
                cursor_status
                    .error
                    .unwrap_or_else(|| "Telegram cursor state is not readable".to_string()),
            );
        } else if !(config.enabled && config.token_shape_ok && config.binding_ready) {
            status = "attention";
            error = Some("Telegram config, token shape, or binding is not ready".to_string());
        } else {
            match load_effective_telegram_token() {
                Ok(token) => {
                    live_read_started = true;
                    external_network_read = true;
                    match call_telegram_get_updates(&token, 20, get_updates_offset) {
                        Ok(api) => {
                            bot_api_ok = api.get("ok").and_then(Value::as_bool);
                            let updates = api
                                .get("result")
                                .and_then(Value::as_array)
                                .cloned()
                                .unwrap_or_default();
                            inspection = inspect_telegram_updates(&updates);
                            local_next_update_offset = inspection.latest_allowed_next_update_offset;
                            model_turn_plan = plan_model_turn_for_updates(&updates);
                            invocation_request = build_model_invocation_request_plan(
                                &updates,
                                cursor_status.next_update_offset,
                                gates.model_turn_gate_enabled,
                            );
                            if bot_api_ok != Some(false) {
                                let mut pipeline = execute_telegram_drain_pipeline_for_updates(
                                    &updates,
                                    cursor_status.next_update_offset,
                                    Some(token.as_str()),
                                    &gates,
                                    Path::new(TELEGRAM_INGRESS_CURSOR_PATH),
                                    run_hepta_model_turn,
                                    call_telegram_send_message,
                                );
                                if pipeline.model_execution.session_runner_invoked
                                    && !telegram_in_process_model_runner_enabled()
                                {
                                    pipeline.model_execution.local_process_spawned = true;
                                }
                                invocation_request = pipeline.invocation_request;
                                model_execution = pipeline.model_execution;
                                send_request = pipeline.send_request;
                                send_execution = pipeline.send_execution;
                                if send_execution.status == "delivered" {
                                    status = "drained";
                                    error = None;
                                } else if send_execution.status == "attention" {
                                    status = "attention";
                                    error = send_execution.error.clone();
                                } else if model_execution.status == "attention" {
                                    status = "attention";
                                    error = model_execution.error.clone();
                                }
                            }
                            if bot_api_ok == Some(false) {
                                status = "attention";
                                error = api
                                    .get("description")
                                    .and_then(Value::as_str)
                                    .map(redact_token_like_text)
                                    .or_else(|| {
                                        Some(
                                            "Telegram Bot API getUpdates returned ok=false"
                                                .to_string(),
                                        )
                                    });
                            }
                        }
                        Err(fetch_error) => {
                            let redacted_error = redact_token_like_text(&fetch_error);
                            status = if is_telegram_get_updates_conflict_error(&redacted_error) {
                                "busy"
                            } else {
                                "attention"
                            };
                            error = Some(redacted_error);
                        }
                    }
                }
                Err(token_error) => {
                    status = "attention";
                    error = Some(redact_token_like_text(&token_error));
                }
            }
        }
    }

    let model_turn_started = model_execution.session_runner_invoked;
    let send_started = send_execution.send_attempted;
    let cursor_written = send_execution.cursor_written;
    let external_network_write = send_execution.external_network_write;
    let external_send = send_execution.external_send;

    NativeTelegramDrainOnceStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        gates,
        config,
        execution_plan: NativeTelegramExecutionPlan {
            execution_plan_ready: requested,
            stages: TELEGRAM_DRAIN_ONCE_STAGES,
            all_required_gates_enabled,
            first_missing_gate,
            receive_before_model: true,
            send_after_model_success: true,
            cursor_commit_after_delivery: true,
            status_probe_executes_pipeline,
        },
        cursor_plan,
        inspection,
        model_turn_plan,
        invocation_request,
        model_execution,
        send_plan,
        send_request,
        send_execution,
        bot_api_ok,
        local_next_update_offset,
        get_updates_offset,
        live_read_started,
        model_turn_started,
        send_started,
        cursor_written,
        external_network_read,
        external_network_write,
        external_send,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
        error,
        next_migration_slice: "continue live production soak with bounded retries, typing keepalive, fallback, and send throttling",
    }
}

fn observe_telegram_live_soak(status: &NativeTelegramDrainOnceStatus) {
    let map = TELEGRAM_LIVE_SOAK_OBSERVATION
        .get_or_init(|| Mutex::new(NativeTelegramLiveSoakObservationState::default()));
    let mut guard = match map.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.observe(status);
}

fn telegram_live_soak_observation_report() -> NativeTelegramLiveSoakObservationReport {
    let map = TELEGRAM_LIVE_SOAK_OBSERVATION
        .get_or_init(|| Mutex::new(NativeTelegramLiveSoakObservationState::default()));
    match map.lock() {
        Ok(guard) => guard.report(),
        Err(poisoned) => poisoned.into_inner().report(),
    }
}

impl NativeTelegramLiveSoakObservationState {
    fn observe(&mut self, status: &NativeTelegramDrainOnceStatus) {
        self.poll_iterations = self.poll_iterations.saturating_add(1);
        let now = now_unix_ms();
        match status.status {
            "drained" => {
                self.drained_count = self.drained_count.saturating_add(1);
                self.last_drained_at_unix_ms = Some(now);
                self.last_drained_next_update_offset = status.local_next_update_offset;
            }
            "busy" => self.busy_count = self.busy_count.saturating_add(1),
            "attention" => self.attention_count = self.attention_count.saturating_add(1),
            _ if status.external_network_read && status.inspection.update_count == 0 => {
                self.empty_read_count = self.empty_read_count.saturating_add(1)
            }
            _ => {}
        }
        if status.model_turn_started {
            self.model_turn_started_count = self.model_turn_started_count.saturating_add(1);
        }
        if status.send_started {
            self.send_started_count = self.send_started_count.saturating_add(1);
        }
        if status.cursor_written {
            self.cursor_written_count = self.cursor_written_count.saturating_add(1);
        }
        if status.external_send {
            self.external_send_count = self.external_send_count.saturating_add(1);
        }
        self.last_observed_at_unix_ms = Some(now);
        self.last_status = Some(status.status.to_string());
        self.last_error = status
            .error
            .clone()
            .map(|error| redact_token_like_text(&error));
        self.last_bot_api_ok = status.bot_api_ok;
        self.last_get_updates_offset = status.get_updates_offset;
        self.last_local_next_update_offset = status.local_next_update_offset;
        self.last_update_count = status.inspection.update_count;
        self.last_allowed_update_count = status.inspection.allowed_update_count;
        self.last_model_turn_started = status.model_turn_started;
        self.last_send_started = status.send_started;
        self.last_cursor_written = status.cursor_written;
        self.last_external_send = status.external_send;
    }

    fn report(&self) -> NativeTelegramLiveSoakObservationReport {
        NativeTelegramLiveSoakObservationReport {
            poll_iterations: self.poll_iterations,
            drained_count: self.drained_count,
            busy_count: self.busy_count,
            attention_count: self.attention_count,
            empty_read_count: self.empty_read_count,
            model_turn_started_count: self.model_turn_started_count,
            send_started_count: self.send_started_count,
            cursor_written_count: self.cursor_written_count,
            external_send_count: self.external_send_count,
            last_drained_at_unix_ms: self.last_drained_at_unix_ms,
            last_drained_next_update_offset: self.last_drained_next_update_offset,
            last_observed_at_unix_ms: self.last_observed_at_unix_ms,
            last_status: self.last_status.clone(),
            last_error: self.last_error.clone(),
            last_bot_api_ok: self.last_bot_api_ok,
            last_get_updates_offset: self.last_get_updates_offset,
            last_local_next_update_offset: self.last_local_next_update_offset,
            last_update_count: self.last_update_count,
            last_allowed_update_count: self.last_allowed_update_count,
            last_model_turn_started: self.last_model_turn_started,
            last_send_started: self.last_send_started,
            last_cursor_written: self.last_cursor_written,
            last_external_send: self.last_external_send,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
            raw_token_exposed: false,
        }
    }
}

fn telegram_production_guard_status() -> NativeTelegramProductionGuardStatus {
    NativeTelegramProductionGuardStatus {
        read_max_attempts_env: TELEGRAM_READ_MAX_ATTEMPTS_ENV,
        read_max_attempts: telegram_read_max_attempts(),
        read_retry_backoff_env: TELEGRAM_READ_RETRY_BACKOFF_ENV,
        read_retry_backoff_ms: duration_millis_u64(telegram_read_retry_backoff()),
        retry_transient_read_errors: true,
        typing_keepalive_env: TELEGRAM_TYPING_KEEPALIVE_ENV,
        typing_keepalive_enabled: telegram_typing_keepalive_enabled(),
        typing_keepalive_interval_ms: duration_millis_u64(telegram_typing_keepalive_interval()),
        model_timeout_env: TELEGRAM_MODEL_TIMEOUT_ENV,
        model_timeout_ms: duration_millis_u64(telegram_model_timeout()),
        model_failure_fallback_env: TELEGRAM_MODEL_FAILURE_FALLBACK_ENV,
        model_failure_fallback_enabled: telegram_model_failure_fallback_enabled(),
        send_min_interval_env: TELEGRAM_SEND_MIN_INTERVAL_ENV,
        send_min_interval_ms: duration_millis_u64(telegram_send_min_interval()),
        send_max_attempts_env: TELEGRAM_SEND_MAX_ATTEMPTS_ENV,
        send_max_attempts: telegram_send_max_attempts(),
        send_retry_backoff_env: TELEGRAM_SEND_RETRY_BACKOFF_ENV,
        send_retry_backoff_ms: duration_millis_u64(telegram_send_retry_backoff()),
        retry_transient_send_errors: true,
        rate_limit_scope: "in-process per chat id; reset on gateway restart",
        raw_token_exposed: false,
    }
}

fn telegram_soak_min_poll_iterations() -> u64 {
    env::var(TELEGRAM_SOAK_MIN_POLLS_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|polls| polls.clamp(1, MAX_TELEGRAM_SOAK_MIN_POLLS))
        .unwrap_or(DEFAULT_TELEGRAM_SOAK_MIN_POLLS)
}

fn telegram_soak_max_attention_count() -> u64 {
    env::var(TELEGRAM_SOAK_MAX_ATTENTION_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|count| count.min(MAX_TELEGRAM_SOAK_MAX_ATTENTION))
        .unwrap_or(DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION)
}

fn duration_millis_u64(duration: Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration_millis_u64(duration))
        .unwrap_or(0)
}

fn first_missing_drain_once_gate(gates: &NativeTelegramGatewayGateSummary) -> Option<&'static str> {
    if !gates.delivery_approval_gate_enabled {
        Some(TELEGRAM_DELIVERY_APPROVED_ENV)
    } else if !gates.live_read_gate_enabled {
        Some(TELEGRAM_LIVE_READ_ENV)
    } else if !gates.model_turn_gate_enabled {
        Some(TELEGRAM_MODEL_TURN_GATE_ENV)
    } else if !gates.send_gate_enabled {
        Some(TELEGRAM_SEND_GATE_ENV)
    } else {
        None
    }
}

fn telegram_send_plan_status_with_gate(
    requested: bool,
    send_gate_enabled: bool,
) -> NativeTelegramSendPlanStatus {
    let config = if requested {
        load_telegram_config_status()
    } else {
        NativeTelegramConfigStatus::disabled()
    };
    let transport_plan = NativeTelegramTransportPlan::for_config(&config);
    let send_plan = if requested {
        NativeTelegramSendPlan::ready()
    } else {
        NativeTelegramSendPlan::disabled()
    };
    let send_request = if requested {
        build_telegram_send_request_plan(None, false, None, send_gate_enabled)
    } else {
        NativeTelegramSendRequestPlan::disabled(send_gate_enabled)
    };
    let config_ready = requested && config.enabled && config.token_shape_ok && config.binding_ready;
    let status = if !requested {
        "disabled"
    } else if !send_gate_enabled {
        "gated"
    } else if config_ready {
        "planned"
    } else {
        "attention"
    };
    let error = if requested && !send_gate_enabled {
        Some(format!(
            "Telegram send is gated; set {TELEGRAM_SEND_GATE_ENV}=1 only after model-turn delivery wiring is ready"
        ))
    } else if requested && !config_ready {
        Some("Telegram config, token shape, or binding is not ready".to_string())
    } else {
        None
    };

    NativeTelegramSendPlanStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        send_gate_enabled,
        bot_api_send_ready: requested && send_gate_enabled && config_ready,
        external_network_write: false,
        external_send: false,
        cursor_written: false,
        raw_response_text_exposed: false,
        raw_chat_id_exposed: false,
        raw_message_id_exposed: false,
        raw_token_exposed: false,
        config,
        transport_plan,
        send_plan,
        send_request,
        error,
        next_migration_slice: "wire sendMessage execution after model output, then commit cursor only after delivery success",
    }
}

fn telegram_receive_once_status_with_gate(
    requested: bool,
    limit: usize,
    live_read_gate_enabled: bool,
) -> NativeTelegramReceiveOnceStatus {
    let limit = limit.clamp(1, 20);
    let config = load_telegram_config_status();
    let transport_plan = NativeTelegramTransportPlan::for_config(&config);
    let cursor_plan = NativeTelegramCursorPlan::ready();

    if !requested {
        return NativeTelegramReceiveOnceStatus::base(
            requested,
            "disabled",
            live_read_gate_enabled,
            false,
            limit,
            config,
            transport_plan,
            cursor_plan,
            inspect_telegram_updates(&[]),
            None,
        );
    }

    if !live_read_gate_enabled {
        return NativeTelegramReceiveOnceStatus::base(
            requested,
            "gated",
            false,
            false,
            limit,
            config,
            transport_plan,
            cursor_plan,
            inspect_telegram_updates(&[]),
            Some(format!(
                "live Telegram receive is gated; set {TELEGRAM_LIVE_READ_ENV}=1 to run one redacted getUpdates read"
            )),
        );
    }

    if !(config.enabled && config.token_shape_ok && config.binding_ready) {
        return NativeTelegramReceiveOnceStatus::base(
            requested,
            "attention",
            true,
            false,
            limit,
            config,
            transport_plan,
            cursor_plan,
            inspect_telegram_updates(&[]),
            Some("Telegram config, token shape, or binding is not ready".to_string()),
        );
    }

    let token = match load_effective_telegram_token() {
        Ok(token) => token,
        Err(error) => {
            return NativeTelegramReceiveOnceStatus::base(
                requested,
                "attention",
                true,
                false,
                limit,
                config,
                transport_plan,
                cursor_plan,
                inspect_telegram_updates(&[]),
                Some(redact_token_like_text(&error)),
            );
        }
    };

    let cursor_status = telegram_cursor_status_from_path(Path::new(TELEGRAM_INGRESS_CURSOR_PATH));
    let get_updates_offset = cursor_status.next_update_offset;
    if cursor_status.cursor_file_present && !cursor_status.cursor_parse_ok {
        let mut report = NativeTelegramReceiveOnceStatus::base(
            requested,
            "attention",
            true,
            false,
            limit,
            config,
            transport_plan,
            cursor_plan,
            inspect_telegram_updates(&[]),
            cursor_status.error,
        );
        report.get_updates_offset = get_updates_offset;
        return report;
    }

    match call_telegram_get_updates(&token, limit, get_updates_offset) {
        Ok(api) => {
            let bot_api_ok = api.get("ok").and_then(Value::as_bool);
            let updates = api
                .get("result")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            let inspection = inspect_telegram_updates(&updates);
            let local_next_update_offset = inspection.latest_allowed_next_update_offset;
            let model_turn_plan = plan_model_turn_for_updates(&updates);
            let status = if bot_api_ok.unwrap_or(false) {
                "ready"
            } else {
                "attention"
            };
            let mut report = NativeTelegramReceiveOnceStatus::base(
                requested,
                status,
                true,
                true,
                limit,
                config,
                transport_plan,
                cursor_plan,
                inspection,
                None,
            );
            report.bot_api_ok = bot_api_ok;
            report.local_next_update_offset = local_next_update_offset;
            report.get_updates_offset = get_updates_offset;
            if bot_api_ok == Some(false) {
                report.error = api
                    .get("description")
                    .and_then(Value::as_str)
                    .map(redact_token_like_text)
                    .or_else(|| Some("Telegram Bot API getUpdates returned ok=false".to_string()));
            }
            report.model_turn_plan = model_turn_plan;
            report
        }
        Err(error) => {
            let redacted_error = redact_token_like_text(&error);
            let status = if is_telegram_get_updates_conflict_error(&redacted_error) {
                "busy"
            } else {
                "attention"
            };
            let mut report = NativeTelegramReceiveOnceStatus::base(
                requested,
                status,
                true,
                true,
                limit,
                config,
                transport_plan,
                cursor_plan,
                inspect_telegram_updates(&[]),
                Some(redacted_error),
            );
            report.get_updates_offset = get_updates_offset;
            report
        }
    }
}

fn load_telegram_config_status() -> NativeTelegramConfigStatus {
    let Some(config_path) = resolve_private_hepta_runtime_config_path() else {
        return NativeTelegramConfigStatus {
            config_path: None,
            config_found: false,
            enabled: false,
            dm_policy: String::new(),
            group_policy: String::new(),
            allow_from_count: 0,
            group_count: 0,
            token_source: "missing",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_shape_ok: false,
            raw_token_exposed: false,
            binding_ready: false,
            error: Some("Hepta private Telegram config not found".to_string()),
        };
    };

    match load_telegram_config_status_from_path(&config_path) {
        Ok(status) => status,
        Err(error) => NativeTelegramConfigStatus {
            config_path: Some(config_path.display().to_string()),
            config_found: config_path.is_file(),
            enabled: false,
            dm_policy: String::new(),
            group_policy: String::new(),
            allow_from_count: 0,
            group_count: 0,
            token_source: "error",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_shape_ok: false,
            raw_token_exposed: false,
            binding_ready: false,
            error: Some(redact_token_like_text(&error)),
        },
    }
}

fn load_telegram_config_status_from_path(
    path: &Path,
) -> Result<NativeTelegramConfigStatus, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("failed to read Hepta private Telegram config: {error}"))?;
    let config: Value = serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse Hepta private Telegram config: {error}"))?;
    let telegram = config
        .pointer("/channels/telegram")
        .ok_or_else(|| "channels.telegram config is missing".to_string())?;

    let enabled = telegram
        .get("enabled")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let dm_policy = telegram
        .get("dmPolicy")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let group_policy = telegram
        .get("groupPolicy")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let allow_from = telegram
        .get("allowFrom")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(normalize_telegram_id)
                .filter(|item| !item.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let group_count = telegram
        .get("groups")
        .and_then(Value::as_array)
        .map(Vec::len)
        .or_else(|| {
            telegram
                .get("groups")
                .and_then(Value::as_object)
                .map(|groups| groups.len())
        })
        .unwrap_or(0);

    let bot_token_ref = telegram.get("botToken");
    let token_secret_ref_present = bot_token_ref
        .and_then(|value| value.get("source"))
        .and_then(Value::as_str)
        == Some("file");
    let token_secret_provider = bot_token_ref
        .and_then(|value| value.get("provider"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let token_secret_id_present = bot_token_ref
        .and_then(|value| value.get("id"))
        .and_then(Value::as_str)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    let token_path = token_secret_provider
        .as_deref()
        .and_then(|provider| secret_provider_path(path, &config, provider));
    let token_file_present = token_path
        .as_ref()
        .map(|path| path.is_file())
        .unwrap_or(false);
    let token_file_mode_0600 = token_path.as_ref().map(file_mode_is_0600).unwrap_or(false);
    let inline_token = bot_token_ref
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let env_token = env::var("HEPTA_TELEGRAM_BOT_TOKEN")
        .ok()
        .or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let file_token = token_path
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let (token_source, token) = if let Some(token) = env_token {
        ("env", Some(token))
    } else if let Some(token) = file_token {
        ("secret_file", Some(token))
    } else if let Some(token) = inline_token {
        ("inline_config", Some(token))
    } else if token_secret_ref_present {
        ("secret_file_missing", None)
    } else {
        ("missing", None)
    };
    let token_shape_ok = token.as_deref().map(token_shape_ok).unwrap_or(false);
    let binding_ready = enabled
        && token_shape_ok
        && (!allow_from.is_empty()
            || group_count > 0
            || matches!(dm_policy.as_str(), "allow" | "trusted" | "all"));

    Ok(NativeTelegramConfigStatus {
        config_path: Some(path.display().to_string()),
        config_found: true,
        enabled,
        dm_policy,
        group_policy,
        allow_from_count: allow_from.len(),
        group_count,
        token_source,
        token_secret_ref_present,
        token_secret_provider,
        token_secret_id_present,
        token_file_present,
        token_file_mode_0600,
        token_shape_ok,
        raw_token_exposed: false,
        binding_ready,
        error: None,
    })
}

fn load_effective_telegram_token() -> Result<String, String> {
    let config_path = resolve_private_hepta_runtime_config_path()
        .ok_or_else(|| "Hepta private Telegram config not found".to_string())?;
    let raw = fs::read_to_string(&config_path)
        .map_err(|error| format!("failed to read Hepta private Telegram config: {error}"))?;
    let config: Value = serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse Hepta private Telegram config: {error}"))?;
    let telegram = config
        .pointer("/channels/telegram")
        .ok_or_else(|| "channels.telegram config is missing".to_string())?;
    let bot_token_ref = telegram.get("botToken");
    let token_secret_provider = bot_token_ref
        .and_then(|value| value.get("provider"))
        .and_then(Value::as_str);
    let token_path = token_secret_provider
        .and_then(|provider| secret_provider_path(&config_path, &config, provider));
    let inline_token = bot_token_ref
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let env_token = env::var("HEPTA_TELEGRAM_BOT_TOKEN")
        .ok()
        .or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let file_token = token_path
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let token = env_token
        .or(file_token)
        .or(inline_token)
        .ok_or_else(|| "Telegram bot token is not configured".to_string())?;
    if token_shape_ok(&token) {
        Ok(token)
    } else {
        Err("Telegram bot token shape is invalid".to_string())
    }
}

fn resolve_private_hepta_runtime_config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("HEPTA_CONFIG_PATH") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }

    let relative = PathBuf::from(LOCAL_IMPORT_CONFIG_PATH);
    if relative.is_file() {
        return Some(relative);
    }

    let manifest = PathBuf::from(LOCAL_IMPORT_MANIFEST_PATH);
    if let Ok(raw) = fs::read_to_string(&manifest) {
        if let Ok(value) = serde_json::from_str::<Value>(&raw) {
            if let Some(import_root) = value.get("import_root").and_then(Value::as_str) {
                let candidate = PathBuf::from(import_root)
                    .join("private/config")
                    .join(LEGACY_CONFIG_FILE_NAME);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
    }

    let home_config = env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join(format!(".{LEGACY_RUNTIME_SLUG}"))
            .join(LEGACY_CONFIG_FILE_NAME)
    });
    home_config.filter(|path| path.is_file())
}

fn call_telegram_get_updates(
    token: &str,
    limit: usize,
    offset: Option<i64>,
) -> Result<Value, String> {
    let max_attempts = telegram_read_max_attempts();
    let retry_backoff = telegram_read_retry_backoff();
    for attempt in 1..=max_attempts {
        match call_telegram_get_updates_once(token, limit, offset) {
            Ok(api) => return Ok(api),
            Err(error) => {
                let error = redact_token_like_text(&error);
                if attempt < max_attempts
                    && is_telegram_get_updates_transient_error(&error)
                    && !is_telegram_get_updates_conflict_error(&error)
                {
                    thread::sleep(retry_backoff);
                    continue;
                }
                return Err(error);
            }
        }
    }
    Err("Telegram Bot API getUpdates retry loop exited unexpectedly".to_string())
}

fn call_telegram_get_updates_once(
    token: &str,
    limit: usize,
    offset: Option<i64>,
) -> Result<Value, String> {
    let endpoint = format!("https://api.telegram.org/bot{token}/getUpdates");
    let query = telegram_get_updates_query(limit, offset);
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|error| format!("failed to build Telegram Bot API client: {error}"))?;
    let response = client.get(endpoint).query(&query).send().map_err(|error| {
        format!(
            "Telegram Bot API getUpdates request failed: {}",
            error.without_url()
        )
    })?;
    let status = response.status();
    let body = response
        .json::<Value>()
        .map_err(|error| format!("failed to parse Telegram Bot API response JSON: {error}"))?;
    if status.is_success() {
        Ok(body)
    } else {
        Err(format!(
            "Telegram Bot API getUpdates HTTP status {}; description={}",
            status.as_u16(),
            body.get("description")
                .and_then(Value::as_str)
                .map(redact_token_like_text)
                .unwrap_or_else(|| "missing".to_string())
        ))
    }
}

fn is_telegram_get_updates_conflict_error(error: &str) -> bool {
    error.contains("Telegram Bot API getUpdates HTTP status 409")
        && error.contains("terminated by other getUpdates request")
}

fn telegram_get_updates_query(limit: usize, offset: Option<i64>) -> Vec<(&'static str, String)> {
    let mut query = vec![
        ("timeout", "0".to_string()),
        ("limit", limit.clamp(1, 20).to_string()),
        ("allowed_updates", TELEGRAM_ALLOWED_UPDATES.to_string()),
    ];
    if let Some(offset) = offset.filter(|offset| *offset >= 0) {
        query.push(("offset", offset.to_string()));
    }
    query
}

#[allow(dead_code)]
fn call_telegram_send_message(
    token: &str,
    chat_id: i64,
    message_text: &str,
    reply_to_message_id: Option<i64>,
) -> Result<Value, String> {
    let endpoint = format!("https://api.telegram.org/bot{token}/sendMessage");
    let body = telegram_send_message_request_body(message_text, chat_id, reply_to_message_id)?;
    wait_for_telegram_send_rate_limit(chat_id);
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|error| format!("failed to build Telegram Bot API client: {error}"))?;
    let response = client.post(endpoint).json(&body).send().map_err(|error| {
        format!(
            "Telegram Bot API sendMessage request failed: {}",
            error.without_url()
        )
    })?;
    let status = response.status();
    let body = response
        .json::<Value>()
        .map_err(|error| format!("failed to parse Telegram Bot API send response JSON: {error}"))?;
    if status.is_success() {
        Ok(body)
    } else {
        Err(format!(
            "Telegram Bot API sendMessage HTTP status {}; description={}",
            status.as_u16(),
            body.get("description")
                .and_then(Value::as_str)
                .map(redact_token_like_text)
                .unwrap_or_else(|| "missing".to_string())
        ))
    }
}

fn call_telegram_send_chat_action(token: &str, chat_id: i64) -> Result<Value, String> {
    let endpoint = format!("https://api.telegram.org/bot{token}/sendChatAction");
    let body = telegram_send_chat_action_request_body(chat_id)?;
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|error| format!("failed to build Telegram Bot API client: {error}"))?;
    let response = client.post(endpoint).json(&body).send().map_err(|error| {
        format!(
            "Telegram Bot API sendChatAction request failed: {}",
            error.without_url()
        )
    })?;
    let status = response.status();
    let body = response.json::<Value>().map_err(|error| {
        format!("failed to parse Telegram Bot API sendChatAction response JSON: {error}")
    })?;
    if status.is_success() {
        Ok(body)
    } else {
        Err(format!(
            "Telegram Bot API sendChatAction HTTP status {}; description={}",
            status.as_u16(),
            body.get("description")
                .and_then(Value::as_str)
                .map(redact_token_like_text)
                .unwrap_or_else(|| "missing".to_string())
        ))
    }
}

fn telegram_send_chat_action_request_body(chat_id: i64) -> Result<Value, String> {
    if chat_id == 0 {
        return Err("Telegram sendChatAction chat id must be non-zero".to_string());
    }
    Ok(serde_json::json!({
        "chat_id": chat_id,
        "action": "typing",
    }))
}

fn telegram_send_message_request_body(
    message_text: &str,
    chat_id: i64,
    reply_to_message_id: Option<i64>,
) -> Result<Value, String> {
    let text = message_text.trim();
    if text.is_empty() {
        return Err("Telegram sendMessage text must be non-empty".to_string());
    }
    let mut body = serde_json::json!({
        "chat_id": chat_id,
        "text": text,
        "disable_web_page_preview": true,
    });
    if let Some(message_id) = reply_to_message_id {
        if message_id <= 0 {
            return Err("Telegram reply message id must be positive".to_string());
        }
        body["reply_parameters"] = serde_json::json!({
            "message_id": message_id,
            "allow_sending_without_reply": true,
        });
    }
    Ok(body)
}

fn env_truthy(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn secret_provider_path(config_path: &Path, config: &Value, provider: &str) -> Option<PathBuf> {
    let raw = config
        .get("secrets")?
        .get("providers")?
        .get(provider)?
        .get("path")?
        .as_str()?;
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        Some(path)
    } else {
        config_path.parent().map(|parent| parent.join(path))
    }
}

#[cfg(unix)]
fn file_mode_is_0600(path: &PathBuf) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o777 == 0o600)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn file_mode_is_0600(path: &PathBuf) -> bool {
    path.is_file()
}

fn token_shape_ok(token: &str) -> bool {
    let Some((bot_id, secret)) = token.split_once(':') else {
        return false;
    };
    !bot_id.is_empty()
        && bot_id.chars().all(|ch| ch.is_ascii_digit())
        && secret.len() >= 20
        && secret
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

fn normalize_telegram_id(raw: &str) -> String {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("telegram:") {
        return trimmed["telegram:".len()..].trim().to_string();
    }
    if lower.starts_with("tg:") {
        return trimmed["tg:".len()..].trim().to_string();
    }
    trimmed.to_string()
}

fn redact_token_like_text(text: &str) -> String {
    text.split_whitespace()
        .map(|part| {
            if token_shape_ok(part.trim_matches(|ch: char| {
                !ch.is_ascii_alphanumeric() && ch != ':' && ch != '_' && ch != '-'
            })) {
                "[redacted-telegram-token]".to_string()
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn inspect_telegram_updates(updates: &[Value]) -> NativeTelegramIngressInspection {
    let mut inspection = NativeTelegramIngressInspection {
        parser_ready: true,
        update_count: updates.len(),
        allowed_update_count: 0,
        latest_observed_update_id: None,
        latest_allowed_update_id: None,
        latest_allowed_next_update_offset: None,
        latest_allowed_text_present: false,
        message_count: 0,
        edited_message_count: 0,
        callback_query_count: 0,
        reaction_count: 0,
        raw_message_text_exposed: false,
        raw_chat_id_exposed: false,
        raw_sender_id_exposed: false,
    };

    for update in updates {
        let update_id = update.get("update_id").and_then(Value::as_i64);
        if let Some(update_id) = update_id {
            inspection.latest_observed_update_id = Some(
                inspection
                    .latest_observed_update_id
                    .map(|current| current.max(update_id))
                    .unwrap_or(update_id),
            );
        }

        let (allowed, text_present) = if let Some(message) = update.get("message") {
            inspection.message_count = inspection.message_count.saturating_add(1);
            (
                telegram_message_is_reply_candidate(message),
                telegram_message_text_present(message),
            )
        } else if let Some(message) = update.get("edited_message") {
            inspection.edited_message_count = inspection.edited_message_count.saturating_add(1);
            (
                telegram_message_is_reply_candidate(message),
                telegram_message_text_present(message),
            )
        } else if update.get("callback_query").is_some() {
            inspection.callback_query_count = inspection.callback_query_count.saturating_add(1);
            (true, false)
        } else if update.get("message_reaction").is_some() {
            inspection.reaction_count = inspection.reaction_count.saturating_add(1);
            (true, false)
        } else {
            (false, false)
        };

        if allowed {
            inspection.allowed_update_count = inspection.allowed_update_count.saturating_add(1);
            if let Some(update_id) = update_id {
                inspection.latest_allowed_update_id = Some(
                    inspection
                        .latest_allowed_update_id
                        .map(|current| current.max(update_id))
                        .unwrap_or(update_id),
                );
                inspection.latest_allowed_next_update_offset =
                    telegram_next_update_offset(update_id);
            }
            inspection.latest_allowed_text_present |= text_present;
        }
    }

    inspection
}

fn telegram_message_is_reply_candidate(message: &Value) -> bool {
    telegram_message_has_reply_target(message) && telegram_message_text_present(message)
}

fn telegram_message_text_present(message: &Value) -> bool {
    message
        .get("text")
        .and_then(Value::as_str)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
        || message
            .get("caption")
            .and_then(Value::as_str)
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
}

fn telegram_update_already_drained(update_id: i64, next_update_offset: Option<i64>) -> bool {
    next_update_offset
        .map(|cursor| update_id < cursor)
        .unwrap_or(false)
}

fn telegram_duplicate_decision(
    update_id: i64,
    next_update_offset: Option<i64>,
) -> NativeTelegramDuplicateDecision {
    let already_drained = telegram_update_already_drained(update_id, next_update_offset);
    let candidate_next_update_offset = telegram_next_update_offset(update_id);
    if already_drained {
        NativeTelegramDuplicateDecision {
            decision: "skip_already_drained",
            update_id,
            current_next_update_offset: next_update_offset,
            candidate_next_update_offset,
            already_drained: true,
            should_invoke_model: false,
            should_record_duplicate: true,
            cursor_write_allowed_after_delivery: false,
            raw_update_payload_exposed: false,
        }
    } else {
        NativeTelegramDuplicateDecision {
            decision: "model_candidate",
            update_id,
            current_next_update_offset: next_update_offset,
            candidate_next_update_offset,
            already_drained: false,
            should_invoke_model: true,
            should_record_duplicate: false,
            cursor_write_allowed_after_delivery: candidate_next_update_offset.is_some(),
            raw_update_payload_exposed: false,
        }
    }
}

fn telegram_next_update_offset(update_id: i64) -> Option<i64> {
    update_id.checked_add(1)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NativeTelegramCandidateMaterial {
    update_id: Option<i64>,
    kind: String,
    prompt_text: Option<String>,
    has_reply_target: bool,
    reply_target: Option<NativeTelegramReplyTargetMaterial>,
    requires_model: bool,
    raw_identifiers_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NativeTelegramReplyTargetMaterial {
    chat_id: i64,
    reply_to_message_id: Option<i64>,
    raw_identifiers_exposed: bool,
}

#[derive(Debug, Clone)]
struct NativeTelegramModelExecutionInput {
    candidate: Option<NativeTelegramCandidateMaterial>,
    duplicate_decision: Option<NativeTelegramDuplicateDecision>,
    model_turn_gate_enabled: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct NativeTelegramModelExecutionOutcome {
    report: NativeTelegramModelExecutionReport,
    model_output: Option<String>,
    reply_target: Option<NativeTelegramReplyTargetMaterial>,
    candidate_next_update_offset: Option<i64>,
}

#[derive(Debug, Clone)]
struct NativeTelegramDrainPipelineOutcome {
    invocation_request: NativeTelegramModelInvocationRequestPlan,
    model_execution: NativeTelegramModelExecutionReport,
    send_request: NativeTelegramSendRequestPlan,
    send_execution: NativeTelegramSendExecutionReport,
}

fn plan_model_turn_for_updates(updates: &[Value]) -> NativeTelegramModelTurnPlan {
    let mut plan = NativeTelegramModelTurnPlan::ready();

    for update in updates.iter().take(20) {
        if let Some(candidate) = extract_telegram_candidate_material(update) {
            let _prompt_material_is_held_in_memory = candidate.prompt_text.is_some();
            plan.candidate_count = plan.candidate_count.saturating_add(1);
            if candidate.requires_model
                && (candidate.kind.starts_with("message:")
                    || candidate.kind.starts_with("edited_message:"))
            {
                plan.text_candidate_count = plan.text_candidate_count.saturating_add(1);
            } else if candidate.requires_model && candidate.kind == "callback_query:redacted" {
                plan.callback_candidate_count = plan.callback_candidate_count.saturating_add(1);
            } else if candidate.kind == "message_reaction:redacted" {
                plan.reaction_candidate_count = plan.reaction_candidate_count.saturating_add(1);
            }
            if candidate.has_reply_target {
                plan.reply_target_count = plan.reply_target_count.saturating_add(1);
            }
            if candidate.raw_identifiers_exposed {
                plan.raw_chat_id_exposed = true;
                plan.raw_sender_id_exposed = true;
                plan.raw_message_id_exposed = true;
            }
            plan.candidate_kinds.push(candidate.kind);
        }
    }

    plan
}

fn build_model_invocation_request_plan(
    updates: &[Value],
    next_update_offset: Option<i64>,
    model_turn_gate_enabled: bool,
) -> NativeTelegramModelInvocationRequestPlan {
    let (_, _, request) = first_model_candidate_with_duplicate_decision(
        updates,
        next_update_offset,
        model_turn_gate_enabled,
    );
    request
}

fn first_model_candidate_with_duplicate_decision(
    updates: &[Value],
    next_update_offset: Option<i64>,
    model_turn_gate_enabled: bool,
) -> (
    Option<NativeTelegramCandidateMaterial>,
    Option<NativeTelegramDuplicateDecision>,
    NativeTelegramModelInvocationRequestPlan,
) {
    for update in updates.iter().take(20) {
        let Some(candidate) = extract_telegram_candidate_material(update) else {
            continue;
        };
        if !candidate.requires_model {
            continue;
        }

        let Some(update_id) = candidate.update_id else {
            let request = NativeTelegramModelInvocationRequestPlan::attention(
                candidate.clone(),
                "missing_update_id",
                None,
                model_turn_gate_enabled,
            );
            return (Some(candidate), None, request);
        };
        let decision = telegram_duplicate_decision(update_id, next_update_offset);
        let request = NativeTelegramModelInvocationRequestPlan::from_candidate(
            candidate.clone(),
            decision.clone(),
            model_turn_gate_enabled,
        );
        return (Some(candidate), Some(decision), request);
    }

    (
        None,
        None,
        NativeTelegramModelInvocationRequestPlan::empty(model_turn_gate_enabled),
    )
}

fn execute_telegram_model_turn_after_candidate<F>(
    input: NativeTelegramModelExecutionInput,
    run_model: F,
) -> NativeTelegramModelExecutionOutcome
where
    F: FnOnce(&str) -> Result<String, String>,
{
    let invocation_request = match (input.candidate.clone(), input.duplicate_decision.clone()) {
        (Some(candidate), Some(decision)) if candidate.requires_model => {
            NativeTelegramModelInvocationRequestPlan::from_candidate(
                candidate,
                decision,
                input.model_turn_gate_enabled,
            )
        }
        (Some(candidate), _) if !candidate.requires_model => {
            NativeTelegramModelInvocationRequestPlan::attention(
                candidate,
                "not_model_candidate",
                None,
                input.model_turn_gate_enabled,
            )
        }
        (Some(candidate), None) if candidate.requires_model => {
            NativeTelegramModelInvocationRequestPlan::attention(
                candidate,
                "missing_update_id",
                None,
                input.model_turn_gate_enabled,
            )
        }
        _ => NativeTelegramModelInvocationRequestPlan::empty(input.model_turn_gate_enabled),
    };
    let mut report =
        NativeTelegramModelExecutionReport::from_invocation_request(&invocation_request);

    if !input.model_turn_gate_enabled {
        report.error = Some(format!(
            "Telegram model execution is gated by {}",
            TELEGRAM_MODEL_TURN_GATE_ENV
        ));
        return NativeTelegramModelExecutionOutcome {
            report,
            model_output: None,
            reply_target: None,
            candidate_next_update_offset: invocation_request.candidate_next_update_offset,
        };
    }

    let Some(candidate) = input.candidate else {
        report.error = Some("Telegram model execution requires a candidate".to_string());
        return NativeTelegramModelExecutionOutcome {
            report,
            model_output: None,
            reply_target: None,
            candidate_next_update_offset: invocation_request.candidate_next_update_offset,
        };
    };
    if invocation_request.should_record_duplicate {
        return NativeTelegramModelExecutionOutcome {
            report,
            model_output: None,
            reply_target: candidate.reply_target,
            candidate_next_update_offset: invocation_request.candidate_next_update_offset,
        };
    }
    let Some(prompt_text) = candidate
        .prompt_text
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        report.status = "attention";
        report.error =
            Some("Telegram model execution requires non-empty prompt material".to_string());
        return NativeTelegramModelExecutionOutcome {
            report,
            model_output: None,
            reply_target: candidate.reply_target,
            candidate_next_update_offset: invocation_request.candidate_next_update_offset,
        };
    };
    if !invocation_request.runner_invocation_allowed {
        report.status = "attention";
        report.error = Some("Telegram model execution request is not runner-eligible".to_string());
        return NativeTelegramModelExecutionOutcome {
            report,
            model_output: None,
            reply_target: candidate.reply_target,
            candidate_next_update_offset: invocation_request.candidate_next_update_offset,
        };
    }

    report.status = "running";
    report.session_runner_invoked = true;
    match run_model(prompt_text) {
        Ok(output) => {
            let output = output.trim().to_string();
            if output.is_empty() {
                report.status = "attention";
                report.error = Some("Telegram model execution returned empty output".to_string());
                NativeTelegramModelExecutionOutcome {
                    report,
                    model_output: None,
                    reply_target: candidate.reply_target,
                    candidate_next_update_offset: invocation_request.candidate_next_update_offset,
                }
            } else {
                report.status = "completed";
                report.model_output_present = true;
                NativeTelegramModelExecutionOutcome {
                    report,
                    model_output: Some(output),
                    reply_target: candidate.reply_target,
                    candidate_next_update_offset: invocation_request.candidate_next_update_offset,
                }
            }
        }
        Err(error) => {
            report.status = "attention";
            report.error = Some(redact_token_like_text(&error));
            NativeTelegramModelExecutionOutcome {
                report,
                model_output: None,
                reply_target: candidate.reply_target,
                candidate_next_update_offset: invocation_request.candidate_next_update_offset,
            }
        }
    }
}

fn build_telegram_send_request_plan(
    model_output: Option<&str>,
    reply_target_available: bool,
    candidate_next_update_offset: Option<i64>,
    send_gate_enabled: bool,
) -> NativeTelegramSendRequestPlan {
    NativeTelegramSendRequestPlan::from_model_output(
        model_output,
        reply_target_available,
        candidate_next_update_offset,
        send_gate_enabled,
    )
}

#[derive(Debug, Clone, Copy)]
struct NativeTelegramSendExecutionInput<'a> {
    token: Option<&'a str>,
    model_output: Option<&'a str>,
    reply_target: Option<&'a NativeTelegramReplyTargetMaterial>,
    candidate_next_update_offset: Option<i64>,
    send_gate_enabled: bool,
    cursor_path: &'a Path,
}

#[allow(dead_code)]
fn execute_telegram_send_after_model_output<F>(
    input: NativeTelegramSendExecutionInput<'_>,
    mut send_message: F,
) -> NativeTelegramSendExecutionReport
where
    F: FnMut(&str, i64, &str, Option<i64>) -> Result<Value, String>,
{
    let model_output = input
        .model_output
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let request = build_telegram_send_request_plan(
        model_output,
        input.reply_target.is_some(),
        input.candidate_next_update_offset,
        input.send_gate_enabled,
    );
    let mut report = NativeTelegramSendExecutionReport::from_send_request(&request);

    if !input.send_gate_enabled {
        report.error = Some(format!(
            "Telegram send execution is gated by {}",
            TELEGRAM_SEND_GATE_ENV
        ));
        return report;
    }
    let Some(model_output) = model_output else {
        report.error = Some("Telegram send execution requires non-empty model output".to_string());
        return report;
    };
    let Some(reply_target) = input.reply_target else {
        report.error = Some("Telegram send execution requires an opaque reply target".to_string());
        return report;
    };
    let Some(candidate_next_update_offset) = input.candidate_next_update_offset else {
        report.error =
            Some("Telegram send execution requires a candidate next-update offset".to_string());
        return report;
    };
    let Some(token) = input
        .token
        .map(str::trim)
        .filter(|token| token_shape_ok(token))
    else {
        report.status = "attention";
        report.error = Some("Telegram send execution requires a valid Bot API token".to_string());
        return report;
    };

    report.status = "sending";
    report.request_body_materialized_by_execution = true;
    report.send_attempted = true;
    report.external_network_write = true;

    let max_attempts = telegram_send_max_attempts();
    let retry_backoff = telegram_send_retry_backoff();
    for attempt in 1..=max_attempts {
        match send_message(
            token,
            reply_target.chat_id,
            model_output,
            reply_target.reply_to_message_id,
        ) {
            Ok(api) => {
                let ok = api.get("ok").and_then(Value::as_bool).unwrap_or(false);
                report.bot_api_ack = Some(ok);
                if !ok {
                    let error = api
                        .get("description")
                        .and_then(Value::as_str)
                        .map(redact_token_like_text)
                        .unwrap_or_else(|| {
                            "Telegram Bot API sendMessage returned ok=false".to_string()
                        });
                    if attempt < max_attempts && is_telegram_send_transient_error(&error) {
                        thread::sleep(retry_backoff);
                        continue;
                    }
                    report.status = "attention";
                    report.error = Some(error);
                    return report;
                }

                report.external_send = true;
                report.cursor_commit_attempted = true;
                match write_telegram_cursor_next_update_offset(
                    input.cursor_path,
                    candidate_next_update_offset,
                ) {
                    Ok(()) => {
                        report.status = "delivered";
                        report.cursor_written = true;
                    }
                    Err(error) => {
                        report.status = "attention";
                        report.error = Some(redact_token_like_text(&error));
                    }
                }
                return report;
            }
            Err(error) => {
                let error = redact_token_like_text(&error);
                if attempt < max_attempts && is_telegram_send_transient_error(&error) {
                    thread::sleep(retry_backoff);
                    continue;
                }
                report.status = "attention";
                report.error = Some(error);
                return report;
            }
        }
    }

    report
}

fn execute_telegram_drain_pipeline_for_updates<F, S>(
    updates: &[Value],
    next_update_offset: Option<i64>,
    token: Option<&str>,
    gates: &NativeTelegramGatewayGateSummary,
    cursor_path: &Path,
    run_model: F,
    send_message: S,
) -> NativeTelegramDrainPipelineOutcome
where
    F: FnOnce(&str) -> Result<String, String>,
    S: FnMut(&str, i64, &str, Option<i64>) -> Result<Value, String>,
{
    let (candidate, duplicate_decision, invocation_request) =
        first_model_candidate_with_duplicate_decision(
            updates,
            next_update_offset,
            gates.model_turn_gate_enabled,
        );

    let typing_reply_target = candidate
        .as_ref()
        .and_then(|candidate| candidate.reply_target.clone());
    let model_outcome = match (candidate.clone(), duplicate_decision.clone()) {
        (Some(candidate), Some(decision)) => {
            let run_model_with_typing = |prompt: &str| {
                run_model_with_optional_typing_keepalive(
                    token,
                    typing_reply_target.as_ref(),
                    prompt,
                    run_model,
                )
            };
            execute_telegram_model_turn_after_candidate(
                NativeTelegramModelExecutionInput {
                    candidate: Some(candidate),
                    duplicate_decision: Some(decision),
                    model_turn_gate_enabled: gates.model_turn_gate_enabled,
                },
                run_model_with_typing,
            )
        }
        _ => {
            let mut report =
                NativeTelegramModelExecutionReport::from_invocation_request(&invocation_request);
            if invocation_request.duplicate_decision == "missing_update_id" {
                report.status = "attention";
                report.error =
                    Some("Telegram model execution requires an update id for cursor safety".into());
            }
            NativeTelegramModelExecutionOutcome {
                report,
                model_output: None,
                reply_target: candidate.and_then(|candidate| candidate.reply_target),
                candidate_next_update_offset: invocation_request.candidate_next_update_offset,
            }
        }
    };

    let fallback_output = telegram_model_failure_fallback_output(&model_outcome);
    let delivery_output = model_outcome
        .model_output
        .as_deref()
        .or(fallback_output.as_deref());

    let send_request = build_telegram_send_request_plan(
        delivery_output,
        model_outcome.reply_target.is_some(),
        model_outcome.candidate_next_update_offset,
        gates.send_gate_enabled,
    );
    let send_execution = execute_telegram_send_after_model_output(
        NativeTelegramSendExecutionInput {
            token,
            model_output: delivery_output,
            reply_target: model_outcome.reply_target.as_ref(),
            candidate_next_update_offset: model_outcome.candidate_next_update_offset,
            send_gate_enabled: gates.send_gate_enabled,
            cursor_path,
        },
        send_message,
    );

    NativeTelegramDrainPipelineOutcome {
        invocation_request,
        model_execution: model_outcome.report,
        send_request,
        send_execution,
    }
}

fn run_hepta_model_turn(prompt: &str) -> Result<String, String> {
    if let Some(config) = telegram_mlx_local_chat_config() {
        return run_mlx_local_chat_completion(prompt, &config);
    }
    if telegram_in_process_model_runner_enabled() {
        run_hepta_in_process_model_turn(prompt)
    } else {
        run_hepta_exec_child_model_turn(prompt)
    }
}

fn run_model_with_optional_typing_keepalive<F>(
    token: Option<&str>,
    reply_target: Option<&NativeTelegramReplyTargetMaterial>,
    prompt: &str,
    run_model: F,
) -> Result<String, String>
where
    F: FnOnce(&str) -> Result<String, String>,
{
    let _typing_keepalive = token
        .zip(reply_target)
        .and_then(|(token, target)| start_telegram_typing_keepalive(token, target.chat_id));
    run_model(prompt)
}

fn telegram_model_failure_fallback_output(
    outcome: &NativeTelegramModelExecutionOutcome,
) -> Option<String> {
    if !telegram_model_failure_fallback_enabled() {
        return None;
    }
    let report = &outcome.report;
    if !(report.session_runner_invoked && report.status == "attention") {
        return None;
    }
    if outcome.reply_target.is_none() || outcome.candidate_next_update_offset.is_none() {
        return None;
    }
    Some(telegram_model_failure_fallback_message())
}

fn telegram_model_failure_fallback_enabled() -> bool {
    env_truthy(TELEGRAM_MODEL_FAILURE_FALLBACK_ENV)
}

fn telegram_model_failure_fallback_message() -> String {
    "本地模型这次响应超时或失败了。我已先收下这条消息，避免反复重试；请稍后再发一条继续。"
        .to_string()
}

struct TelegramTypingKeepalive {
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Drop for TelegramTypingKeepalive {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn start_telegram_typing_keepalive(token: &str, chat_id: i64) -> Option<TelegramTypingKeepalive> {
    if !telegram_typing_keepalive_enabled() || !token_shape_ok(token) || chat_id == 0 {
        return None;
    }
    let token = token.to_string();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let interval = telegram_typing_keepalive_interval();
    let handle = thread::spawn(move || {
        while !thread_stop.load(Ordering::Relaxed) {
            let _ = call_telegram_send_chat_action(&token, chat_id);
            let started = Instant::now();
            while !thread_stop.load(Ordering::Relaxed) && started.elapsed() < interval {
                thread::sleep(Duration::from_millis(100));
            }
        }
    });
    Some(TelegramTypingKeepalive {
        stop,
        handle: Some(handle),
    })
}

fn telegram_typing_keepalive_enabled() -> bool {
    env_truthy(TELEGRAM_TYPING_KEEPALIVE_ENV)
}

fn telegram_typing_keepalive_interval() -> Duration {
    let millis = env::var(TELEGRAM_TYPING_KEEPALIVE_INTERVAL_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|ms| ms.clamp(1_000, MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS))
        .unwrap_or(DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS);
    Duration::from_millis(millis)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TelegramMlxLocalChatConfig {
    base_url: String,
    model: String,
    max_tokens: u64,
}

fn telegram_mlx_local_chat_config() -> Option<TelegramMlxLocalChatConfig> {
    let model_ref = env::var(TELEGRAM_MODEL_ENV)
        .ok()
        .or_else(|| env::var(HEPTA_DEFAULT_MODEL_ENV).ok())?;
    let model = parse_mlx_local_model_ref(&model_ref)?;
    let base_url = env::var(TELEGRAM_MLX_BASE_URL_ENV)
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_TELEGRAM_MLX_BASE_URL.to_string());
    let max_tokens = env::var(TELEGRAM_MLX_MAX_TOKENS_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|value| value.clamp(1, MAX_TELEGRAM_MLX_MAX_TOKENS))
        .unwrap_or(DEFAULT_TELEGRAM_MLX_MAX_TOKENS);
    Some(TelegramMlxLocalChatConfig {
        base_url,
        model,
        max_tokens,
    })
}

fn parse_mlx_local_model_ref(model_ref: &str) -> Option<String> {
    model_ref
        .trim()
        .strip_prefix("mlx-local/")
        .map(str::trim)
        .filter(|model| !model.is_empty())
        .map(ToOwned::to_owned)
}

fn run_mlx_local_chat_completion(
    prompt: &str,
    config: &TelegramMlxLocalChatConfig,
) -> Result<String, String> {
    let prompt = prompt.trim();
    if prompt.is_empty() {
        return Err("Telegram MLX runner requires non-empty prompt material".to_string());
    }
    let endpoint = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));
    let client = reqwest::blocking::Client::builder()
        .timeout(telegram_model_timeout())
        .build()
        .map_err(|error| format!("failed to build local MLX model client: {error}"))?;
    let response = client
        .post(endpoint)
        .json(&serde_json::json!({
            "model": config.model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are Hepta replying in Telegram. Answer naturally, concisely, and in the user's language."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": config.max_tokens,
            "max_kv_size": 4096,
            "temperature": 0.2,
            "stream": false,
            "strip_thinking": true
        }))
        .send()
        .map_err(|error| {
            format!(
                "local MLX chat-completions request failed: {}",
                error.without_url()
            )
        })?;
    let status = response.status();
    let body = response
        .json::<Value>()
        .map_err(|error| format!("failed to parse local MLX response JSON: {error}"))?;
    if !status.is_success() {
        return Err(format!(
            "local MLX chat-completions HTTP status {}; description={}",
            status.as_u16(),
            body.pointer("/error/message")
                .and_then(Value::as_str)
                .map(redact_token_like_text)
                .unwrap_or_else(|| "missing".to_string())
        ));
    }
    extract_openai_chat_completion_text(&body)
}

fn extract_openai_chat_completion_text(body: &Value) -> Result<String, String> {
    body.pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .or_else(|| body.pointer("/choices/0/text").and_then(Value::as_str))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| "local MLX chat-completions response did not include text".to_string())
}

fn run_hepta_in_process_model_turn(prompt: &str) -> Result<String, String> {
    let prompt = prompt.trim();
    if prompt.is_empty() {
        return Err("Telegram model runner requires non-empty prompt material".to_string());
    }

    let timeout = telegram_model_timeout();
    let prompt = prompt.to_string();
    let arg0_paths = Arg0DispatchPaths {
        codex_self_exe: env::current_exe().ok(),
        codex_linux_sandbox_exe: None,
        main_execve_wrapper_exe: None,
    };
    let run = async move {
        tokio::time::timeout(
            timeout,
            codex_exec::run_prompt_to_last_message(prompt, arg0_paths),
        )
        .await
        .map_err(|_| {
            format!(
                "gated in-process Hepta exec runner timed out after {} ms",
                timeout.as_millis()
            )
        })?
        .map_err(|error| format!("gated in-process Hepta exec runner failed: {error}"))
    };

    match Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(run)),
        Err(_) => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| {
                format!("failed to build runtime for in-process Hepta exec runner: {error}")
            })?
            .block_on(run),
    }
}

fn telegram_in_process_model_runner_enabled() -> bool {
    env_truthy(TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV)
}

fn run_hepta_exec_child_model_turn(prompt: &str) -> Result<String, String> {
    let prompt = prompt.trim();
    if prompt.is_empty() {
        return Err("Telegram model runner requires non-empty prompt material".to_string());
    }

    let exe = env::current_exe()
        .map_err(|error| format!("failed to resolve current Hepta executable: {error}"))?;
    let tempdir = tempfile::Builder::new()
        .prefix("hepta-telegram-model-")
        .tempdir()
        .map_err(|error| format!("failed to create Telegram model tempdir: {error}"))?;
    let last_message_path = tempdir.path().join("last-message.txt");
    let args = hepta_exec_child_args(&last_message_path, prompt);
    let timeout = telegram_model_timeout();
    let mut child = Command::new(&exe)
        .args(args)
        .env("HEPTA_NATIVE_TELEGRAM_EXEC_CHILD", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| {
            format!(
                "failed to spawn gated Hepta exec runner {}: {error}",
                exe.display()
            )
        })?;

    let status = wait_for_telegram_model_child(&mut child, timeout)?;
    if !status.success() {
        return Err(format!(
            "gated Hepta exec runner exited with status {}",
            status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "signal".to_string())
        ));
    }

    let output = fs::read_to_string(&last_message_path)
        .map_err(|error| format!("failed to read gated Hepta exec last message: {error}"))?;
    let output = output.trim();
    if output.is_empty() {
        return Err("gated Hepta exec runner produced an empty final message".to_string());
    }
    Ok(output.to_string())
}

fn hepta_exec_child_args(last_message_path: &Path, prompt: &str) -> Vec<String> {
    vec![
        "-c".to_string(),
        "approval_policy=\"never\"".to_string(),
        "exec".to_string(),
        "--skip-git-repo-check".to_string(),
        "--ephemeral".to_string(),
        "--ignore-rules".to_string(),
        "--sandbox".to_string(),
        "read-only".to_string(),
        "--output-last-message".to_string(),
        last_message_path.to_string_lossy().to_string(),
        prompt.to_string(),
    ]
}

fn telegram_model_timeout() -> Duration {
    let millis = env::var(TELEGRAM_MODEL_TIMEOUT_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|ms| ms.clamp(1_000, MAX_TELEGRAM_MODEL_TIMEOUT_MS))
        .unwrap_or(DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS);
    Duration::from_millis(millis)
}

fn wait_for_telegram_send_rate_limit(chat_id: i64) {
    let min_interval = telegram_send_min_interval();
    if min_interval.is_zero() {
        return;
    }
    let map = TELEGRAM_SEND_RATE_LIMITS.get_or_init(|| Mutex::new(HashMap::new()));
    let sleep_for = {
        let mut guard = match map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let now = Instant::now();
        let sleep_for = guard
            .get(&chat_id)
            .and_then(|last| min_interval.checked_sub(last.elapsed()))
            .unwrap_or_default();
        guard.insert(chat_id, now + sleep_for);
        sleep_for
    };
    if !sleep_for.is_zero() {
        thread::sleep(sleep_for);
    }
}

fn telegram_send_min_interval() -> Duration {
    let millis = env::var(TELEGRAM_SEND_MIN_INTERVAL_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|ms| ms.min(MAX_TELEGRAM_SEND_MIN_INTERVAL_MS))
        .unwrap_or(0);
    Duration::from_millis(millis)
}

fn telegram_read_max_attempts() -> u64 {
    env::var(TELEGRAM_READ_MAX_ATTEMPTS_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|attempts| attempts.clamp(1, MAX_TELEGRAM_READ_MAX_ATTEMPTS))
        .unwrap_or(DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS)
}

fn telegram_read_retry_backoff() -> Duration {
    let millis = env::var(TELEGRAM_READ_RETRY_BACKOFF_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|ms| ms.min(MAX_TELEGRAM_READ_RETRY_BACKOFF_MS))
        .unwrap_or(DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS);
    Duration::from_millis(millis)
}

fn telegram_send_max_attempts() -> u64 {
    env::var(TELEGRAM_SEND_MAX_ATTEMPTS_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|attempts| attempts.clamp(1, MAX_TELEGRAM_SEND_MAX_ATTEMPTS))
        .unwrap_or(DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS)
}

fn telegram_send_retry_backoff() -> Duration {
    let millis = env::var(TELEGRAM_SEND_RETRY_BACKOFF_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(|ms| ms.min(MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS))
        .unwrap_or(DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS);
    Duration::from_millis(millis)
}

fn is_telegram_send_transient_error(error: &str) -> bool {
    error.contains("request failed")
        || error.contains("HTTP status 429")
        || error.contains("HTTP status 500")
        || error.contains("HTTP status 502")
        || error.contains("HTTP status 503")
        || error.contains("HTTP status 504")
        || error.contains("Too Many Requests")
}

fn is_telegram_get_updates_transient_error(error: &str) -> bool {
    error.contains("request failed")
        || error.contains("HTTP status 429")
        || error.contains("HTTP status 500")
        || error.contains("HTTP status 502")
        || error.contains("HTTP status 503")
        || error.contains("HTTP status 504")
        || error.contains("Too Many Requests")
}

fn wait_for_telegram_model_child(
    child: &mut std::process::Child,
    timeout: Duration,
) -> Result<std::process::ExitStatus, String> {
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if started.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!(
                    "gated Hepta exec runner timed out after {} ms",
                    timeout.as_millis()
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(50)),
            Err(error) => {
                return Err(format!(
                    "failed while waiting for gated Hepta exec runner: {error}"
                ));
            }
        }
    }
}

fn extract_telegram_candidate_material(update: &Value) -> Option<NativeTelegramCandidateMaterial> {
    let update_id = update.get("update_id").and_then(Value::as_i64);
    if let Some(message) = update.get("message") {
        return telegram_message_prompt_material(update_id, "message", message);
    }
    if let Some(message) = update.get("edited_message") {
        return telegram_message_prompt_material(update_id, "edited_message", message);
    }
    if let Some(callback) = update.get("callback_query") {
        let reply_target = callback
            .get("message")
            .and_then(telegram_message_reply_target_material);
        return Some(NativeTelegramCandidateMaterial {
            update_id,
            kind: "callback_query:redacted".to_string(),
            prompt_text: callback
                .get("data")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned),
            has_reply_target: reply_target.is_some(),
            reply_target,
            requires_model: true,
            raw_identifiers_exposed: false,
        });
    }
    if update.get("message_reaction").is_some() {
        return Some(NativeTelegramCandidateMaterial {
            update_id,
            kind: "message_reaction:redacted".to_string(),
            prompt_text: None,
            has_reply_target: false,
            reply_target: None,
            requires_model: false,
            raw_identifiers_exposed: false,
        });
    }
    None
}

fn telegram_message_prompt_material(
    update_id: Option<i64>,
    prefix: &str,
    message: &Value,
) -> Option<NativeTelegramCandidateMaterial> {
    let (kind, prompt_text) = telegram_message_prompt_kind_and_text(message)?;
    let reply_target = telegram_message_reply_target_material(message);
    Some(NativeTelegramCandidateMaterial {
        update_id,
        kind: format!("{prefix}:{kind}"),
        prompt_text: Some(prompt_text),
        has_reply_target: reply_target.is_some(),
        reply_target,
        requires_model: true,
        raw_identifiers_exposed: false,
    })
}

fn telegram_message_prompt_kind_and_text(message: &Value) -> Option<(&'static str, String)> {
    if let Some(text) = message
        .get("text")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(("text", text.to_string()))
    } else {
        message
            .get("caption")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|caption| ("caption", caption.to_string()))
    }
}

fn telegram_message_has_reply_target(message: &Value) -> bool {
    telegram_message_reply_target_material(message).is_some()
}

fn telegram_message_reply_target_material(
    message: &Value,
) -> Option<NativeTelegramReplyTargetMaterial> {
    let chat_id = message.get("chat")?.get("id")?.as_i64()?;
    let reply_to_message_id = message
        .get("message_id")
        .and_then(Value::as_i64)
        .filter(|message_id| *message_id > 0)?;
    Some(NativeTelegramReplyTargetMaterial {
        chat_id,
        reply_to_message_id: Some(reply_to_message_id),
        raw_identifiers_exposed: false,
    })
}

impl NativeTelegramConfigStatus {
    fn disabled() -> Self {
        Self {
            config_path: None,
            config_found: false,
            enabled: false,
            dm_policy: String::new(),
            group_policy: String::new(),
            allow_from_count: 0,
            group_count: 0,
            token_source: "disabled",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_shape_ok: false,
            raw_token_exposed: false,
            binding_ready: false,
            error: None,
        }
    }
}

impl NativeTelegramCursorPlan {
    fn disabled() -> Self {
        Self {
            cursor_path: TELEGRAM_INGRESS_CURSOR_PATH,
            duplicate_suppression_ready: false,
            duplicate_suppression_rule_valid: true,
            cursor_represents_next_update_offset: true,
            commit_offset_after_delivery: false,
            raw_update_payload_persisted: false,
        }
    }

    fn ready() -> Self {
        Self {
            cursor_path: TELEGRAM_INGRESS_CURSOR_PATH,
            duplicate_suppression_ready: true,
            duplicate_suppression_rule_valid: telegram_update_already_drained(41, Some(42))
                && !telegram_update_already_drained(42, Some(42)),
            cursor_represents_next_update_offset: true,
            commit_offset_after_delivery: true,
            raw_update_payload_persisted: false,
        }
    }
}

impl NativeTelegramModelTurnPlan {
    fn disabled() -> Self {
        Self {
            planner_ready: false,
            candidate_count: 0,
            text_candidate_count: 0,
            callback_candidate_count: 0,
            reaction_candidate_count: 0,
            reply_target_count: 0,
            candidate_kinds: Vec::new(),
            prompt_material_policy: "disabled",
            session_key_strategy: "disabled",
            reply_target_strategy: "disabled",
            model_turn_invocation_gate: "disabled",
            send_delivery_gate: "disabled",
            raw_message_text_exposed: false,
            raw_callback_data_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
        }
    }

    fn ready() -> Self {
        Self {
            planner_ready: true,
            candidate_count: 0,
            text_candidate_count: 0,
            callback_candidate_count: 0,
            reaction_candidate_count: 0,
            reply_target_count: 0,
            candidate_kinds: Vec::new(),
            prompt_material_policy: "carry prompt text only inside the later model-turn call; never expose it in readiness JSON",
            session_key_strategy: "derive a stable internal session key from redacted Telegram binding metadata",
            reply_target_strategy: "retain only an opaque reply target handle for later sendMessage reply_parameters",
            model_turn_invocation_gate: "requires receive candidate, duplicate-suppression decision, and explicit model bridge enablement",
            send_delivery_gate: "requires successful model-turn output and explicit Telegram send gate",
            raw_message_text_exposed: false,
            raw_callback_data_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
        }
    }
}

impl NativeTelegramModelInvocationRequestPlan {
    fn disabled(model_turn_gate_enabled: bool) -> Self {
        Self {
            request_builder_ready: false,
            candidate_present: false,
            candidate_kind: None,
            duplicate_decision: "disabled",
            prompt_material_in_memory: false,
            prompt_material_serialized: false,
            reply_target_available: false,
            stable_session_key_ready: false,
            should_invoke_model: false,
            should_record_duplicate: false,
            candidate_next_update_offset: None,
            model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
            model_turn_gate_enabled,
            runner_invocation_allowed: false,
            session_runner_invoked: false,
            local_process_spawned: false,
            external_send: false,
            cursor_written: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
        }
    }

    fn empty(model_turn_gate_enabled: bool) -> Self {
        Self {
            request_builder_ready: true,
            candidate_present: false,
            candidate_kind: None,
            duplicate_decision: "no_model_candidate",
            prompt_material_in_memory: false,
            prompt_material_serialized: false,
            reply_target_available: false,
            stable_session_key_ready: false,
            should_invoke_model: false,
            should_record_duplicate: false,
            candidate_next_update_offset: None,
            model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
            model_turn_gate_enabled,
            runner_invocation_allowed: false,
            session_runner_invoked: false,
            local_process_spawned: false,
            external_send: false,
            cursor_written: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
        }
    }

    fn attention(
        candidate: NativeTelegramCandidateMaterial,
        duplicate_decision: &'static str,
        candidate_next_update_offset: Option<i64>,
        model_turn_gate_enabled: bool,
    ) -> Self {
        Self::from_parts(
            candidate,
            duplicate_decision,
            false,
            false,
            candidate_next_update_offset,
            model_turn_gate_enabled,
        )
    }

    fn from_candidate(
        candidate: NativeTelegramCandidateMaterial,
        decision: NativeTelegramDuplicateDecision,
        model_turn_gate_enabled: bool,
    ) -> Self {
        Self::from_parts(
            candidate,
            decision.decision,
            decision.should_invoke_model,
            decision.should_record_duplicate,
            decision.candidate_next_update_offset,
            model_turn_gate_enabled,
        )
    }

    fn from_parts(
        candidate: NativeTelegramCandidateMaterial,
        duplicate_decision: &'static str,
        should_invoke_model: bool,
        should_record_duplicate: bool,
        candidate_next_update_offset: Option<i64>,
        model_turn_gate_enabled: bool,
    ) -> Self {
        let prompt_material_in_memory = candidate.prompt_text.is_some();
        let stable_session_key_ready =
            candidate.has_reply_target && !candidate.raw_identifiers_exposed;
        Self {
            request_builder_ready: true,
            candidate_present: true,
            candidate_kind: Some(candidate.kind),
            duplicate_decision,
            prompt_material_in_memory,
            prompt_material_serialized: false,
            reply_target_available: candidate.has_reply_target,
            stable_session_key_ready,
            should_invoke_model,
            should_record_duplicate,
            candidate_next_update_offset,
            model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
            model_turn_gate_enabled,
            runner_invocation_allowed: model_turn_gate_enabled && should_invoke_model,
            session_runner_invoked: false,
            local_process_spawned: false,
            external_send: false,
            cursor_written: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
        }
    }
}

impl NativeTelegramModelExecutionReport {
    fn disabled(model_turn_gate_enabled: bool) -> Self {
        Self {
            status: "disabled",
            execution_ready: false,
            model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
            model_turn_gate_enabled,
            candidate_present: false,
            prompt_material_present: false,
            reply_target_available: false,
            stable_session_key_ready: false,
            candidate_next_update_offset: None,
            runner_invocation_allowed: false,
            session_runner_invoked: false,
            local_process_spawned: false,
            model_output_present: false,
            external_send: false,
            cursor_written: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
            error: None,
        }
    }

    fn from_invocation_request(request: &NativeTelegramModelInvocationRequestPlan) -> Self {
        let status = if !request.request_builder_ready {
            "disabled"
        } else if !request.model_turn_gate_enabled {
            "gated"
        } else if !request.candidate_present {
            "waiting_candidate"
        } else if request.should_record_duplicate {
            "duplicate_suppressed"
        } else if !request.prompt_material_in_memory {
            "waiting_prompt"
        } else if request.runner_invocation_allowed {
            "ready"
        } else {
            "attention"
        };

        Self {
            status,
            execution_ready: request.request_builder_ready,
            model_turn_gate_env: request.model_turn_gate_env,
            model_turn_gate_enabled: request.model_turn_gate_enabled,
            candidate_present: request.candidate_present,
            prompt_material_present: request.prompt_material_in_memory,
            reply_target_available: request.reply_target_available,
            stable_session_key_ready: request.stable_session_key_ready,
            candidate_next_update_offset: request.candidate_next_update_offset,
            runner_invocation_allowed: request.runner_invocation_allowed,
            session_runner_invoked: false,
            local_process_spawned: false,
            model_output_present: false,
            external_send: false,
            cursor_written: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
            error: None,
        }
    }
}

impl NativeTelegramSessionBridgePlan {
    fn disabled() -> Self {
        Self {
            bridge_plan_ready: false,
            runner_kind: "disabled",
            runner_invocation_strategy: "disabled",
            prompt_material_policy: "disabled",
            session_key_strategy: "disabled",
            duplicate_policy: "disabled",
            cursor_commit_policy: "disabled",
            response_delivery_policy: "disabled",
            approval_policy: "disabled",
            failure_policy: "disabled",
            process_spawned_by_status: false,
            raw_prompt_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
        }
    }

    fn ready(in_process_runner_enabled: bool) -> Self {
        let (runner_kind, runner_invocation_strategy) = if in_process_runner_enabled {
            (
                "hepta_in_process_exec_runner",
                "gated in-process Hepta exec runner with read-only sandbox and final-message capture",
            )
        } else {
            (
                "hepta_exec_child_runner",
                "gated hepta exec child runner with read-only sandbox and output-last-message capture; set HEPTA_NATIVE_TELEGRAM_IN_PROCESS_MODEL_RUNNER=1 to use the in-process runner",
            )
        };
        Self {
            bridge_plan_ready: true,
            runner_kind,
            runner_invocation_strategy,
            prompt_material_policy: "raw Telegram text is held only in the pending model-turn invocation and is never serialized into status JSON",
            session_key_strategy: "map each Telegram conversation to a stable internal Hepta session key without exposing raw chat ids",
            duplicate_policy: "suppress candidates whose update id is below the committed next-update cursor before any model turn",
            cursor_commit_policy: "write the next-update cursor only after model output is handled or duplicate suppression is recorded",
            response_delivery_policy: "convert model output to a Telegram send plan only after HEPTA_NATIVE_TELEGRAM_SEND is explicitly enabled",
            approval_policy: "reuse the Hepta session approval policy; do not auto-escalate shell/tool approvals from Telegram ingress",
            failure_policy: "on runner failure, keep cursor uncommitted and return a redacted diagnostic instead of sending partial output",
            process_spawned_by_status: false,
            raw_prompt_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_sender_id_exposed: false,
            raw_message_id_exposed: false,
        }
    }
}

impl NativeTelegramSendPlan {
    fn disabled() -> Self {
        Self {
            send_plan_ready: false,
            method: "disabled",
            request_builder_strategy: "disabled",
            response_source_policy: "disabled",
            reply_target_policy: "disabled",
            parse_mode_policy: "disabled",
            typing_keepalive_policy: "disabled",
            rate_limit_policy: "disabled",
            retry_policy: "disabled",
            cursor_commit_policy: "disabled",
            failure_policy: "disabled",
            request_body_materialized_by_status: false,
            delivery_performed_by_status: false,
            raw_response_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_message_id_exposed: false,
            raw_token_exposed: false,
        }
    }

    fn ready() -> Self {
        Self {
            send_plan_ready: true,
            method: "sendMessage",
            request_builder_strategy: "build a Telegram sendMessage request only from successful model output and an opaque reply target handle",
            response_source_policy: "model output stays in memory until the gated send execution path; status JSON exposes only policy metadata",
            reply_target_policy: "use reply_parameters when an opaque reply target is available, otherwise send to the resolved conversation handle",
            parse_mode_policy: "start with plain text; enable parse_mode only after escaping and formatting tests land",
            typing_keepalive_policy: "sendChatAction typing may run only while a gated model turn is active and must stop before final send",
            rate_limit_policy: "apply per-chat send throttling before Bot API delivery",
            retry_policy: "retry transient Bot API failures with bounded backoff; never duplicate sends after an acknowledged delivery",
            cursor_commit_policy: "commit next-update cursor only after sendMessage succeeds or duplicate suppression is recorded",
            failure_policy: "on send failure, keep cursor uncommitted and return redacted diagnostics without exposing model output",
            request_body_materialized_by_status: false,
            delivery_performed_by_status: false,
            raw_response_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_message_id_exposed: false,
            raw_token_exposed: false,
        }
    }
}

impl NativeTelegramSendRequestPlan {
    fn disabled(send_gate_enabled: bool) -> Self {
        Self {
            request_builder_ready: false,
            model_output_present: false,
            reply_target_available: false,
            candidate_next_update_offset: None,
            send_gate_env: TELEGRAM_SEND_GATE_ENV,
            send_gate_enabled,
            send_allowed: false,
            request_body_materialized_by_status: false,
            delivery_performed_by_status: false,
            cursor_commit_allowed_after_delivery: false,
            raw_response_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_message_id_exposed: false,
            raw_token_exposed: false,
        }
    }

    fn from_model_output(
        model_output: Option<&str>,
        reply_target_available: bool,
        candidate_next_update_offset: Option<i64>,
        send_gate_enabled: bool,
    ) -> Self {
        let model_output_present = model_output
            .map(str::trim)
            .map(|value| !value.is_empty())
            .unwrap_or(false);
        let send_allowed = send_gate_enabled
            && model_output_present
            && reply_target_available
            && candidate_next_update_offset.is_some();
        Self {
            request_builder_ready: true,
            model_output_present,
            reply_target_available,
            candidate_next_update_offset,
            send_gate_env: TELEGRAM_SEND_GATE_ENV,
            send_gate_enabled,
            send_allowed,
            request_body_materialized_by_status: false,
            delivery_performed_by_status: false,
            cursor_commit_allowed_after_delivery: send_allowed
                && candidate_next_update_offset.is_some(),
            raw_response_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_message_id_exposed: false,
            raw_token_exposed: false,
        }
    }
}

impl NativeTelegramSendExecutionReport {
    fn disabled(send_gate_enabled: bool) -> Self {
        Self {
            status: "disabled",
            execution_ready: false,
            send_gate_env: TELEGRAM_SEND_GATE_ENV,
            send_gate_enabled,
            model_output_present: false,
            reply_target_available: false,
            candidate_next_update_offset: None,
            send_allowed: false,
            send_attempted: false,
            bot_api_ack: None,
            cursor_commit_attempted: false,
            cursor_written: false,
            request_body_materialized_by_execution: false,
            external_network_write: false,
            external_send: false,
            raw_response_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_message_id_exposed: false,
            raw_token_exposed: false,
            error: None,
        }
    }

    fn from_send_request(request: &NativeTelegramSendRequestPlan) -> Self {
        let status = if !request.request_builder_ready {
            "disabled"
        } else if !request.send_gate_enabled {
            "gated"
        } else if !request.model_output_present {
            "waiting_model_output"
        } else if !request.reply_target_available {
            "waiting_reply_target"
        } else if request.candidate_next_update_offset.is_none() {
            "waiting_cursor_offset"
        } else if request.send_allowed {
            "ready"
        } else {
            "attention"
        };

        Self {
            status,
            execution_ready: request.request_builder_ready,
            send_gate_env: request.send_gate_env,
            send_gate_enabled: request.send_gate_enabled,
            model_output_present: request.model_output_present,
            reply_target_available: request.reply_target_available,
            candidate_next_update_offset: request.candidate_next_update_offset,
            send_allowed: request.send_allowed,
            send_attempted: false,
            bot_api_ack: None,
            cursor_commit_attempted: false,
            cursor_written: false,
            request_body_materialized_by_execution: false,
            external_network_write: false,
            external_send: false,
            raw_response_text_exposed: false,
            raw_chat_id_exposed: false,
            raw_message_id_exposed: false,
            raw_token_exposed: false,
            error: None,
        }
    }
}

impl NativeTelegramReceiveOnceStatus {
    #[allow(clippy::too_many_arguments)]
    fn base(
        requested: bool,
        status: &'static str,
        live_read_gate_enabled: bool,
        external_network_read: bool,
        limit: usize,
        config: NativeTelegramConfigStatus,
        transport_plan: NativeTelegramTransportPlan,
        cursor_plan: NativeTelegramCursorPlan,
        inspection: NativeTelegramIngressInspection,
        error: Option<String>,
    ) -> Self {
        Self {
            product: "Hepta",
            runtime: "hepta-codex",
            requested,
            status,
            live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
            live_read_gate_enabled,
            external_network_read,
            external_send: false,
            model_turn_started: false,
            cursor_written: false,
            raw_update_payload_exposed: false,
            raw_token_exposed: false,
            limit,
            get_updates_offset: None,
            bot_api_ok: None,
            local_next_update_offset: inspection.latest_allowed_next_update_offset,
            config,
            transport_plan,
            cursor_plan,
            inspection,
            model_turn_plan: if requested {
                plan_model_turn_for_updates(&[])
            } else {
                NativeTelegramModelTurnPlan::disabled()
            },
            error,
            next_migration_slice: "manual receive is a diagnostic read path; use drain-once or the armed poll loop for model, send, and cursor side effects",
        }
    }
}

impl NativeTelegramTransportPlan {
    fn disabled() -> Self {
        Self {
            bot_api_transport_plan_ready: false,
            endpoint_template: "https://api.telegram.org/bot<redacted-token>/{method}",
            get_updates_method: "getUpdates",
            send_message_method: "sendMessage",
            send_chat_action_method: "sendChatAction",
            allowed_updates: TELEGRAM_ALLOWED_UPDATES,
            offset_commit_strategy: "disabled",
            send_delivery_gate: "disabled",
            typing_keepalive_plan: "disabled",
            raw_token_exposed: false,
            external_network_performed_by_status: false,
        }
    }

    fn for_config(config: &NativeTelegramConfigStatus) -> Self {
        let ready = config.enabled && config.token_shape_ok && config.binding_ready;
        Self {
            bot_api_transport_plan_ready: ready,
            endpoint_template: "https://api.telegram.org/bot<redacted-token>/{method}",
            get_updates_method: "getUpdates",
            send_message_method: "sendMessage",
            send_chat_action_method: "sendChatAction",
            allowed_updates: TELEGRAM_ALLOWED_UPDATES,
            offset_commit_strategy: "commit getUpdates offset only after delivery succeeds or duplicate suppression is recorded",
            send_delivery_gate: "sendMessage requires a successful model-turn or command dispatch plus explicit confirm-send runtime gate",
            typing_keepalive_plan: "sendChatAction typing keepalive is planned while the model turn is running, with bounded TTL",
            raw_token_exposed: false,
            external_network_performed_by_status: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telegram_config_status_reads_secret_file_without_exposing_token() {
        let temp = tempfile::tempdir().expect("tempdir");
        let secret_path = temp.path().join("telegram-token.txt");
        fs::write(&secret_path, "123456789:abcdefghijklmnopqrstuvwxyz").expect("write token");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&secret_path, fs::Permissions::from_mode(0o600)).expect("set mode");
        }
        let config_path = temp.path().join("openclaw.json");
        fs::write(
            &config_path,
            format!(
                r#"{{
                    "secrets": {{ "providers": {{ "telegram_bot": {{ "path": "{}" }} }} }},
                    "channels": {{
                        "telegram": {{
                            "enabled": true,
                            "dmPolicy": "allow",
                            "groupPolicy": "mention",
                            "allowFrom": ["telegram:6476198178"],
                            "botToken": {{
                                "source": "file",
                                "provider": "telegram_bot",
                                "id": "bot-token"
                            }}
                        }}
                    }}
                }}"#,
                secret_path.display()
            ),
        )
        .expect("write config");

        let status = load_telegram_config_status_from_path(&config_path).expect("load config");
        assert!(status.enabled);
        assert_eq!(status.token_source, "secret_file");
        assert!(status.token_shape_ok);
        assert!(status.binding_ready);
        assert!(!status.raw_token_exposed);

        let serialized = serde_json::to_string(&status).expect("serialize");
        assert!(!serialized.contains("abcdefghijklmnopqrstuvwxyz"));
        assert!(serialized.contains("\"raw_token_exposed\":false"));
    }

    #[test]
    fn requested_plugin_reports_native_supervisor_without_reply_loop_claim() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join("openclaw.json");
        fs::write(
            &config_path,
            r#"{
                "channels": {
                    "telegram": {
                        "enabled": true,
                        "dmPolicy": "allow",
                        "allowFrom": ["6476198178"],
                        "botToken": "123456789:abcdefghijklmnopqrstuvwxyz"
                    }
                }
            }"#,
        )
        .expect("write config");

        let config = load_telegram_config_status_from_path(&config_path).expect("load config");
        let plugin = NativeTelegramPluginStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested: true,
            status: "native_supervisor_ready",
            in_process_supervisor_ready: true,
            in_process_reply_loop_ready: false,
            model_turn_bridge_ready: false,
            bot_api_poll_ready: false,
            bot_api_send_ready: false,
            openclaw_gateway_runtime_dependency: false,
            external_network_read: false,
            external_send: false,
            poll_ms: 1500,
            allowed_updates: TELEGRAM_ALLOWED_UPDATES,
            transport_plan: NativeTelegramTransportPlan::for_config(&config),
            config,
            ingress_parser: inspect_telegram_updates(&[]),
            cursor_plan: NativeTelegramCursorPlan::ready(),
            model_turn_plan: plan_model_turn_for_updates(&[]),
            migration_blocker: Some(
                "Bot API polling/send and Codex model-turn bridge are not enabled in hepta-codex yet",
            ),
            next_migration_slice: "wire native Bot API getUpdates/sendMessage loop behind explicit delivery gates",
        };

        assert_eq!(plugin.status, "native_supervisor_ready");
        assert!(plugin.in_process_supervisor_ready);
        assert!(!plugin.in_process_reply_loop_ready);
        assert!(!plugin.external_send);
        assert!(plugin.transport_plan.bot_api_transport_plan_ready);
        assert!(!plugin.transport_plan.external_network_performed_by_status);
        assert!(!plugin.transport_plan.raw_token_exposed);
        assert!(plugin.ingress_parser.parser_ready);
        assert!(!plugin.ingress_parser.raw_message_text_exposed);
        assert!(plugin.cursor_plan.duplicate_suppression_ready);
        assert!(plugin.cursor_plan.commit_offset_after_delivery);
        assert!(plugin.model_turn_plan.planner_ready);
        assert!(!plugin.model_turn_plan.raw_message_text_exposed);
    }

    #[test]
    fn ingress_parser_counts_allowed_updates_without_exposing_private_fields() {
        let update = serde_json::json!({
            "update_id": 42,
            "message": {
                "message_id": 7,
                "text": "private prompt text",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        });

        let inspection = inspect_telegram_updates(&[update]);
        assert!(inspection.parser_ready);
        assert_eq!(inspection.update_count, 1);
        assert_eq!(inspection.allowed_update_count, 1);
        assert_eq!(inspection.latest_observed_update_id, Some(42));
        assert_eq!(inspection.latest_allowed_update_id, Some(42));
        assert_eq!(inspection.latest_allowed_next_update_offset, Some(43));
        assert!(inspection.latest_allowed_text_present);

        let serialized = serde_json::to_string(&inspection).expect("serialize");
        assert!(!serialized.contains("private prompt text"));
        assert!(!serialized.contains("6476198178"));
        assert!(!inspection.raw_message_text_exposed);
        assert!(!inspection.raw_chat_id_exposed);
        assert!(!inspection.raw_sender_id_exposed);
    }

    #[test]
    fn cursor_helpers_treat_cursor_as_next_update_offset() {
        assert!(!telegram_update_already_drained(41, None));
        assert!(telegram_update_already_drained(41, Some(42)));
        assert!(!telegram_update_already_drained(42, Some(42)));
        assert_eq!(telegram_next_update_offset(42), Some(43));
        assert_eq!(telegram_next_update_offset(i64::MAX), None);
    }

    #[test]
    fn get_updates_query_uses_cursor_offset_when_available() {
        let without_offset = telegram_get_updates_query(999, None);
        assert_eq!(
            without_offset,
            vec![
                ("timeout", "0".to_string()),
                ("limit", "20".to_string()),
                ("allowed_updates", TELEGRAM_ALLOWED_UPDATES.to_string()),
            ]
        );

        let with_offset = telegram_get_updates_query(5, Some(43));
        assert_eq!(
            with_offset,
            vec![
                ("timeout", "0".to_string()),
                ("limit", "5".to_string()),
                ("allowed_updates", TELEGRAM_ALLOWED_UPDATES.to_string()),
                ("offset", "43".to_string()),
            ]
        );

        let negative_offset = telegram_get_updates_query(5, Some(-1));
        assert!(!negative_offset.iter().any(|(name, _)| *name == "offset"));
    }

    #[test]
    fn duplicate_decision_skips_already_drained_updates() {
        let decision = telegram_duplicate_decision(41, Some(42));
        assert_eq!(decision.decision, "skip_already_drained");
        assert!(decision.already_drained);
        assert!(!decision.should_invoke_model);
        assert!(decision.should_record_duplicate);
        assert!(!decision.cursor_write_allowed_after_delivery);
        assert!(!decision.raw_update_payload_exposed);
    }

    #[test]
    fn duplicate_decision_allows_new_model_candidate() {
        let decision = telegram_duplicate_decision(42, Some(42));
        assert_eq!(decision.decision, "model_candidate");
        assert!(!decision.already_drained);
        assert!(decision.should_invoke_model);
        assert!(!decision.should_record_duplicate);
        assert!(decision.cursor_write_allowed_after_delivery);
        assert_eq!(decision.candidate_next_update_offset, Some(43));
        assert!(!decision.raw_update_payload_exposed);
    }

    #[test]
    fn model_turn_plan_counts_candidates_without_exposing_private_fields() {
        let updates = vec![
            serde_json::json!({
                "update_id": 42,
                "message": {
                    "message_id": 7,
                    "text": "private prompt text",
                    "chat": { "id": 6476198178_i64, "type": "private" },
                    "from": { "id": 6476198178_i64, "username": "private_user" }
                }
            }),
            serde_json::json!({
                "update_id": 43,
                "callback_query": {
                    "id": "opaque-callback-id",
                    "data": "button_secret_payload",
                    "message": {
                        "message_id": 8,
                        "chat": { "id": 6476198178_i64, "type": "private" }
                    }
                }
            }),
            serde_json::json!({
                "update_id": 44,
                "message_reaction": {
                    "chat": { "id": 6476198178_i64 },
                    "user": { "id": 6476198178_i64 }
                }
            }),
        ];

        let plan = plan_model_turn_for_updates(&updates);
        assert!(plan.planner_ready);
        assert_eq!(plan.candidate_count, 3);
        assert_eq!(plan.text_candidate_count, 1);
        assert_eq!(plan.callback_candidate_count, 1);
        assert_eq!(plan.reaction_candidate_count, 1);
        assert_eq!(plan.reply_target_count, 2);
        assert_eq!(
            plan.candidate_kinds,
            vec![
                "message:text".to_string(),
                "callback_query:redacted".to_string(),
                "message_reaction:redacted".to_string(),
            ]
        );

        let serialized = serde_json::to_string(&plan).expect("serialize");
        assert!(!serialized.contains("private prompt text"));
        assert!(!serialized.contains("button_secret_payload"));
        assert!(!serialized.contains("opaque-callback-id"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains("private_user"));
        assert!(!plan.raw_message_text_exposed);
        assert!(!plan.raw_callback_data_exposed);
        assert!(!plan.raw_chat_id_exposed);
        assert!(!plan.raw_sender_id_exposed);
        assert!(!plan.raw_message_id_exposed);
    }

    #[test]
    fn candidate_material_holds_prompt_in_memory_without_public_plan_exposure() {
        let update = serde_json::json!({
            "update_id": 45,
            "message": {
                "message_id": 9,
                "text": "private prompt text",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        });

        let candidate = extract_telegram_candidate_material(&update).expect("candidate");
        assert_eq!(candidate.kind, "message:text");
        assert_eq!(
            candidate.prompt_text.as_deref(),
            Some("private prompt text")
        );
        assert!(candidate.has_reply_target);
        let reply_target = candidate.reply_target.as_ref().expect("reply target");
        assert_eq!(reply_target.chat_id, 6476198178);
        assert_eq!(reply_target.reply_to_message_id, Some(9));
        assert!(!reply_target.raw_identifiers_exposed);
        assert!(candidate.requires_model);
        assert!(!candidate.raw_identifiers_exposed);

        let plan = plan_model_turn_for_updates(&[update]);
        let serialized = serde_json::to_string(&plan).expect("serialize");
        assert!(!serialized.contains("private prompt text"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains("private_user"));
    }

    #[test]
    fn candidate_material_redacts_callback_kind_but_keeps_data_in_memory() {
        let update = serde_json::json!({
            "update_id": 46,
            "callback_query": {
                "id": "opaque-callback-id",
                "data": "button_secret_payload",
                "message": {
                    "message_id": 10,
                    "chat": { "id": 6476198178_i64, "type": "private" }
                }
            }
        });

        let candidate = extract_telegram_candidate_material(&update).expect("candidate");
        assert_eq!(candidate.kind, "callback_query:redacted");
        assert_eq!(
            candidate.prompt_text.as_deref(),
            Some("button_secret_payload")
        );
        assert!(candidate.has_reply_target);
        let reply_target = candidate.reply_target.as_ref().expect("reply target");
        assert_eq!(reply_target.chat_id, 6476198178);
        assert_eq!(reply_target.reply_to_message_id, Some(10));
        assert!(!reply_target.raw_identifiers_exposed);
        assert!(candidate.requires_model);
        assert!(!candidate.raw_identifiers_exposed);

        let plan = plan_model_turn_for_updates(&[update]);
        let serialized = serde_json::to_string(&plan).expect("serialize");
        assert!(serialized.contains("callback_query:redacted"));
        assert!(!serialized.contains("button_secret_payload"));
        assert!(!serialized.contains("opaque-callback-id"));
        assert!(!serialized.contains("6476198178"));
    }

    #[test]
    fn model_invocation_request_builder_uses_candidate_without_serializing_prompt() {
        let update = serde_json::json!({
            "update_id": 47,
            "message": {
                "message_id": 11,
                "text": "private model prompt",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        });

        let request = build_model_invocation_request_plan(&[update], Some(47), false);
        assert!(request.request_builder_ready);
        assert!(request.candidate_present);
        assert_eq!(request.candidate_kind.as_deref(), Some("message:text"));
        assert_eq!(request.duplicate_decision, "model_candidate");
        assert!(request.prompt_material_in_memory);
        assert!(!request.prompt_material_serialized);
        assert!(request.reply_target_available);
        assert!(request.stable_session_key_ready);
        assert!(request.should_invoke_model);
        assert!(!request.should_record_duplicate);
        assert_eq!(request.candidate_next_update_offset, Some(48));
        assert_eq!(request.model_turn_gate_env, TELEGRAM_MODEL_TURN_GATE_ENV);
        assert!(!request.model_turn_gate_enabled);
        assert!(!request.runner_invocation_allowed);
        assert!(!request.session_runner_invoked);
        assert!(!request.local_process_spawned);
        assert!(!request.external_send);
        assert!(!request.cursor_written);
        assert!(!request.raw_update_payload_exposed);
        assert!(!request.raw_prompt_text_exposed);
        assert!(!request.raw_chat_id_exposed);
        assert!(!request.raw_sender_id_exposed);
        assert!(!request.raw_message_id_exposed);

        let serialized = serde_json::to_string(&request).expect("serialize");
        assert!(!serialized.contains("private model prompt"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains("private_user"));
    }

    #[test]
    fn model_invocation_request_builder_suppresses_duplicate_before_runner() {
        let update = serde_json::json!({
            "update_id": 47,
            "message": {
                "message_id": 12,
                "text": "private duplicate prompt",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        });

        let request = build_model_invocation_request_plan(&[update], Some(48), true);
        assert!(request.request_builder_ready);
        assert!(request.candidate_present);
        assert_eq!(request.duplicate_decision, "skip_already_drained");
        assert!(request.prompt_material_in_memory);
        assert!(!request.should_invoke_model);
        assert!(request.should_record_duplicate);
        assert_eq!(request.candidate_next_update_offset, Some(48));
        assert!(request.model_turn_gate_enabled);
        assert!(!request.runner_invocation_allowed);
        assert!(!request.session_runner_invoked);
        assert!(!request.cursor_written);

        let serialized = serde_json::to_string(&request).expect("serialize");
        assert!(!serialized.contains("private duplicate prompt"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains("private_user"));
    }

    #[test]
    fn model_execution_runs_runner_without_serializing_prompt_or_output() {
        let update = serde_json::json!({
            "update_id": 48,
            "message": {
                "message_id": 13,
                "text": "private model prompt",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        });
        let candidate = extract_telegram_candidate_material(&update).expect("candidate");
        let decision = telegram_duplicate_decision(48, Some(48));

        let outcome = execute_telegram_model_turn_after_candidate(
            NativeTelegramModelExecutionInput {
                candidate: Some(candidate),
                duplicate_decision: Some(decision),
                model_turn_gate_enabled: true,
            },
            |prompt| {
                assert_eq!(prompt, "private model prompt");
                Ok(" private model response text ".to_string())
            },
        );

        assert_eq!(outcome.report.status, "completed");
        assert!(outcome.report.execution_ready);
        assert!(outcome.report.model_turn_gate_enabled);
        assert!(outcome.report.candidate_present);
        assert!(outcome.report.prompt_material_present);
        assert!(outcome.report.reply_target_available);
        assert!(outcome.report.stable_session_key_ready);
        assert_eq!(outcome.report.candidate_next_update_offset, Some(49));
        assert!(outcome.report.runner_invocation_allowed);
        assert!(outcome.report.session_runner_invoked);
        assert!(!outcome.report.local_process_spawned);
        assert!(outcome.report.model_output_present);
        assert!(!outcome.report.external_send);
        assert!(!outcome.report.cursor_written);
        assert!(!outcome.report.raw_update_payload_exposed);
        assert!(!outcome.report.raw_prompt_text_exposed);
        assert!(!outcome.report.raw_response_text_exposed);
        assert!(!outcome.report.raw_chat_id_exposed);
        assert!(!outcome.report.raw_sender_id_exposed);
        assert!(!outcome.report.raw_message_id_exposed);
        assert_eq!(
            outcome.model_output.as_deref(),
            Some("private model response text")
        );
        assert!(outcome.reply_target.is_some());
        assert_eq!(outcome.candidate_next_update_offset, Some(49));

        let serialized = serde_json::to_string(&outcome.report).expect("serialize");
        assert!(!serialized.contains("private model prompt"));
        assert!(!serialized.contains("private model response text"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains("private_user"));
    }

    #[test]
    fn model_execution_requires_gate_before_runner_invocation() {
        let update = serde_json::json!({
            "update_id": 48,
            "message": {
                "message_id": 13,
                "text": "private model prompt",
                "chat": { "id": 6476198178_i64, "type": "private" }
            }
        });
        let candidate = extract_telegram_candidate_material(&update).expect("candidate");
        let decision = telegram_duplicate_decision(48, Some(48));

        let outcome = execute_telegram_model_turn_after_candidate(
            NativeTelegramModelExecutionInput {
                candidate: Some(candidate),
                duplicate_decision: Some(decision),
                model_turn_gate_enabled: false,
            },
            |_| panic!("model runner must not run while gated"),
        );

        assert_eq!(outcome.report.status, "gated");
        assert!(!outcome.report.runner_invocation_allowed);
        assert!(!outcome.report.session_runner_invoked);
        assert!(!outcome.report.model_output_present);
        assert!(
            outcome
                .report
                .error
                .unwrap()
                .contains(TELEGRAM_MODEL_TURN_GATE_ENV)
        );
        assert_eq!(outcome.model_output, None);
    }

    #[test]
    fn model_execution_suppresses_duplicate_before_runner() {
        let update = serde_json::json!({
            "update_id": 48,
            "message": {
                "message_id": 13,
                "text": "private duplicate prompt",
                "chat": { "id": 6476198178_i64, "type": "private" }
            }
        });
        let candidate = extract_telegram_candidate_material(&update).expect("candidate");
        let decision = telegram_duplicate_decision(48, Some(49));

        let outcome = execute_telegram_model_turn_after_candidate(
            NativeTelegramModelExecutionInput {
                candidate: Some(candidate),
                duplicate_decision: Some(decision),
                model_turn_gate_enabled: true,
            },
            |_| panic!("duplicate candidate must not invoke model runner"),
        );

        assert_eq!(outcome.report.status, "duplicate_suppressed");
        assert!(!outcome.report.runner_invocation_allowed);
        assert!(!outcome.report.session_runner_invoked);
        assert!(!outcome.report.model_output_present);
        assert_eq!(outcome.model_output, None);
        assert_eq!(outcome.candidate_next_update_offset, Some(49));
    }

    #[test]
    fn model_bridge_without_gate_is_gated_and_side_effect_free() {
        let status = telegram_model_bridge_status_with_gate(true, false);
        assert_eq!(status.status, "gated");
        assert_eq!(status.model_turn_gate_env, TELEGRAM_MODEL_TURN_GATE_ENV);
        assert_eq!(status.send_gate_env, TELEGRAM_SEND_GATE_ENV);
        assert!(!status.model_turn_gate_enabled);
        assert!(!status.model_turn_bridge_ready);
        assert!(!status.model_turn_started);
        assert!(!status.session_runner_invoked);
        assert!(!status.local_process_spawned);
        assert!(!status.external_network_read);
        assert!(!status.external_send);
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_exposed);
        assert!(!status.raw_prompt_text_exposed);
        assert!(!status.raw_chat_id_exposed);
        assert!(!status.raw_sender_id_exposed);
        assert!(!status.raw_message_id_exposed);
        assert!(status.invocation_request.request_builder_ready);
        assert!(!status.invocation_request.candidate_present);
        assert!(!status.invocation_request.session_runner_invoked);
        assert!(!status.invocation_request.local_process_spawned);
        assert_eq!(status.model_execution.status, "gated");
        assert!(!status.model_execution.session_runner_invoked);
        assert!(!status.model_execution.local_process_spawned);
        assert!(!status.model_execution.model_output_present);
        assert!(status.bridge_plan.bridge_plan_ready);
        assert!(!status.bridge_plan.process_spawned_by_status);
        assert!(!status.bridge_plan.raw_prompt_text_exposed);
        assert!(status.error.unwrap().contains(TELEGRAM_MODEL_TURN_GATE_ENV));
    }

    #[test]
    fn send_plan_without_gate_is_gated_and_side_effect_free() {
        let status = telegram_send_plan_status_with_gate(true, false);
        assert_eq!(status.status, "gated");
        assert_eq!(status.send_gate_env, TELEGRAM_SEND_GATE_ENV);
        assert!(!status.send_gate_enabled);
        assert!(!status.bot_api_send_ready);
        assert!(!status.external_network_write);
        assert!(!status.external_send);
        assert!(!status.cursor_written);
        assert!(!status.raw_response_text_exposed);
        assert!(!status.raw_chat_id_exposed);
        assert!(!status.raw_message_id_exposed);
        assert!(!status.raw_token_exposed);
        assert!(status.send_plan.send_plan_ready);
        assert_eq!(status.send_plan.method, "sendMessage");
        assert!(!status.send_plan.request_body_materialized_by_status);
        assert!(!status.send_plan.delivery_performed_by_status);
        assert!(!status.send_plan.raw_response_text_exposed);
        assert!(!status.send_plan.raw_chat_id_exposed);
        assert!(!status.send_plan.raw_message_id_exposed);
        assert!(!status.send_plan.raw_token_exposed);
        assert!(status.send_request.request_builder_ready);
        assert!(!status.send_request.model_output_present);
        assert!(!status.send_request.send_allowed);
        assert!(!status.send_request.delivery_performed_by_status);
        assert!(status.error.unwrap().contains(TELEGRAM_SEND_GATE_ENV));
    }

    #[test]
    fn send_request_builder_consumes_model_output_without_serializing_response() {
        let request = build_telegram_send_request_plan(
            Some("private model response text"),
            true,
            Some(49),
            false,
        );
        assert!(request.request_builder_ready);
        assert!(request.model_output_present);
        assert!(request.reply_target_available);
        assert_eq!(request.candidate_next_update_offset, Some(49));
        assert_eq!(request.send_gate_env, TELEGRAM_SEND_GATE_ENV);
        assert!(!request.send_gate_enabled);
        assert!(!request.send_allowed);
        assert!(!request.request_body_materialized_by_status);
        assert!(!request.delivery_performed_by_status);
        assert!(!request.cursor_commit_allowed_after_delivery);
        assert!(!request.raw_response_text_exposed);
        assert!(!request.raw_chat_id_exposed);
        assert!(!request.raw_message_id_exposed);
        assert!(!request.raw_token_exposed);

        let serialized = serde_json::to_string(&request).expect("serialize");
        assert!(!serialized.contains("private model response text"));
    }

    #[test]
    fn send_request_builder_requires_gate_and_reply_target_before_delivery() {
        let without_reply_target = build_telegram_send_request_plan(
            Some("private model response text"),
            false,
            Some(49),
            true,
        );
        assert!(without_reply_target.model_output_present);
        assert!(without_reply_target.send_gate_enabled);
        assert!(!without_reply_target.reply_target_available);
        assert!(!without_reply_target.send_allowed);
        assert!(!without_reply_target.cursor_commit_allowed_after_delivery);

        let without_offset =
            build_telegram_send_request_plan(Some("private model response text"), true, None, true);
        assert!(without_offset.model_output_present);
        assert!(without_offset.reply_target_available);
        assert!(!without_offset.send_allowed);
        assert!(!without_offset.cursor_commit_allowed_after_delivery);

        let allowed = build_telegram_send_request_plan(
            Some("private model response text"),
            true,
            Some(49),
            true,
        );
        assert!(allowed.send_allowed);
        assert!(allowed.cursor_commit_allowed_after_delivery);
        assert!(!allowed.request_body_materialized_by_status);
        assert!(!allowed.delivery_performed_by_status);
        assert!(!allowed.raw_response_text_exposed);
    }

    #[test]
    fn send_message_request_body_shapes_plain_reply_without_parse_mode() {
        let body = telegram_send_message_request_body(
            "  private model response text  ",
            6476198178_i64,
            Some(11),
        )
        .expect("request body");
        assert_eq!(
            body.get("chat_id").and_then(Value::as_i64),
            Some(6476198178)
        );
        assert_eq!(
            body.get("text").and_then(Value::as_str),
            Some("private model response text")
        );
        assert_eq!(
            body.pointer("/reply_parameters/message_id")
                .and_then(Value::as_i64),
            Some(11)
        );
        assert_eq!(
            body.pointer("/reply_parameters/allow_sending_without_reply")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            body.get("disable_web_page_preview")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert!(body.get("parse_mode").is_none());
    }

    #[test]
    fn send_message_request_body_rejects_empty_text_and_bad_reply_id() {
        let empty = telegram_send_message_request_body("   ", 6476198178_i64, Some(11))
            .expect_err("empty text rejected");
        assert!(empty.contains("text must be non-empty"));

        let bad_reply = telegram_send_message_request_body(
            "private model response text",
            6476198178_i64,
            Some(0),
        )
        .expect_err("bad reply rejected");
        assert!(bad_reply.contains("reply message id must be positive"));
    }

    #[test]
    fn send_chat_action_request_body_shapes_typing_action() {
        let body =
            telegram_send_chat_action_request_body(6476198178_i64).expect("typing request body");
        assert_eq!(
            body.get("chat_id").and_then(Value::as_i64),
            Some(6476198178)
        );
        assert_eq!(body.get("action").and_then(Value::as_str), Some("typing"));
        let bad = telegram_send_chat_action_request_body(0).expect_err("bad chat id rejected");
        assert!(bad.contains("chat id must be non-zero"));
    }

    #[test]
    fn model_failure_fallback_message_is_bounded_and_non_empty() {
        let message = telegram_model_failure_fallback_message();
        assert!(!message.trim().is_empty());
        assert!(message.len() < 512);
    }

    #[test]
    fn send_execution_commits_cursor_only_after_bot_api_ack() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let reply_target = NativeTelegramReplyTargetMaterial {
            chat_id: 6476198178_i64,
            reply_to_message_id: Some(11),
            raw_identifiers_exposed: false,
        };
        let token = "123456:ABCDEFGHIJKLMNOPQRSTUVWX";

        let report = execute_telegram_send_after_model_output(
            NativeTelegramSendExecutionInput {
                token: Some(token),
                model_output: Some("  private model response text  "),
                reply_target: Some(&reply_target),
                candidate_next_update_offset: Some(50),
                send_gate_enabled: true,
                cursor_path: &cursor_path,
            },
            |observed_token, chat_id, text, reply_to_message_id| {
                assert_eq!(observed_token, token);
                assert_eq!(chat_id, 6476198178_i64);
                assert_eq!(text, "private model response text");
                assert_eq!(reply_to_message_id, Some(11));
                Ok(serde_json::json!({
                    "ok": true,
                    "result": { "message_id": 99 }
                }))
            },
        );

        assert_eq!(report.status, "delivered");
        assert!(report.execution_ready);
        assert!(report.send_gate_enabled);
        assert!(report.model_output_present);
        assert!(report.reply_target_available);
        assert_eq!(report.candidate_next_update_offset, Some(50));
        assert!(report.send_allowed);
        assert!(report.send_attempted);
        assert_eq!(report.bot_api_ack, Some(true));
        assert!(report.cursor_commit_attempted);
        assert!(report.cursor_written);
        assert!(report.request_body_materialized_by_execution);
        assert!(report.external_network_write);
        assert!(report.external_send);
        assert!(!report.raw_response_text_exposed);
        assert!(!report.raw_chat_id_exposed);
        assert!(!report.raw_message_id_exposed);
        assert!(!report.raw_token_exposed);
        assert_eq!(report.error, None);

        let cursor = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(cursor.next_update_offset, Some(50));
        let serialized = serde_json::to_string(&report).expect("serialize");
        assert!(!serialized.contains("private model response text"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains(token));
    }

    #[test]
    fn send_execution_keeps_cursor_uncommitted_on_send_failure() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let reply_target = NativeTelegramReplyTargetMaterial {
            chat_id: 6476198178_i64,
            reply_to_message_id: Some(11),
            raw_identifiers_exposed: false,
        };

        let report =
            execute_telegram_send_after_model_output(
                NativeTelegramSendExecutionInput {
                    token: Some("123456:ABCDEFGHIJKLMNOPQRSTUVWX"),
                    model_output: Some("private model response text"),
                    reply_target: Some(&reply_target),
                    candidate_next_update_offset: Some(50),
                    send_gate_enabled: true,
                    cursor_path: &cursor_path,
                },
                |_, _, _, _| {
                    Err("Telegram Bot API sendMessage HTTP status 500; description=temporary outage"
                    .to_string())
                },
            );

        assert_eq!(report.status, "attention");
        assert!(report.send_attempted);
        assert_eq!(report.bot_api_ack, None);
        assert!(!report.cursor_commit_attempted);
        assert!(!report.cursor_written);
        assert!(report.external_network_write);
        assert!(!report.external_send);
        assert!(report.error.unwrap().contains("temporary outage"));
        assert!(!cursor_path.exists());
    }

    #[test]
    fn send_execution_requires_gate_before_network_write() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let reply_target = NativeTelegramReplyTargetMaterial {
            chat_id: 6476198178_i64,
            reply_to_message_id: Some(11),
            raw_identifiers_exposed: false,
        };

        let report = execute_telegram_send_after_model_output(
            NativeTelegramSendExecutionInput {
                token: Some("123456:ABCDEFGHIJKLMNOPQRSTUVWX"),
                model_output: Some("private model response text"),
                reply_target: Some(&reply_target),
                candidate_next_update_offset: Some(50),
                send_gate_enabled: false,
                cursor_path: &cursor_path,
            },
            |_, _, _, _| panic!("sendMessage must not run while gated"),
        );

        assert_eq!(report.status, "gated");
        assert!(!report.send_allowed);
        assert!(!report.send_attempted);
        assert!(!report.cursor_commit_attempted);
        assert!(!report.cursor_written);
        assert!(!report.external_network_write);
        assert!(!report.external_send);
        assert!(report.error.unwrap().contains(TELEGRAM_SEND_GATE_ENV));
        assert!(!cursor_path.exists());
    }

    #[test]
    fn drain_pipeline_runs_model_then_send_and_commits_cursor_after_ack() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let token = "123456:ABCDEFGHIJKLMNOPQRSTUVWX";
        let update = serde_json::json!({
            "update_id": 49,
            "message": {
                "message_id": 15,
                "text": "private pipeline prompt",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        });
        let gates = NativeTelegramGatewayGateSummary {
            delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
            delivery_approval_gate_enabled: true,
            live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
            live_read_gate_enabled: true,
            model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
            model_turn_gate_enabled: true,
            send_gate_env: TELEGRAM_SEND_GATE_ENV,
            send_gate_enabled: true,
            readiness_summary_performs_live_read: false,
            readiness_summary_invokes_model: false,
            readiness_summary_sends_message: false,
        };

        let outcome = execute_telegram_drain_pipeline_for_updates(
            &[update],
            Some(49),
            Some(token),
            &gates,
            &cursor_path,
            |prompt| {
                assert_eq!(prompt, "private pipeline prompt");
                Ok("private pipeline response".to_string())
            },
            |observed_token, chat_id, text, reply_to_message_id| {
                assert_eq!(observed_token, token);
                assert_eq!(chat_id, 6476198178_i64);
                assert_eq!(text, "private pipeline response");
                assert_eq!(reply_to_message_id, Some(15));
                Ok(serde_json::json!({
                    "ok": true,
                    "result": { "message_id": 101 }
                }))
            },
        );

        assert!(outcome.invocation_request.request_builder_ready);
        assert_eq!(
            outcome.invocation_request.duplicate_decision,
            "model_candidate"
        );
        assert_eq!(
            outcome.invocation_request.candidate_next_update_offset,
            Some(50)
        );
        assert_eq!(outcome.model_execution.status, "completed");
        assert!(outcome.model_execution.session_runner_invoked);
        assert!(outcome.model_execution.model_output_present);
        assert!(outcome.send_request.send_allowed);
        assert_eq!(outcome.send_execution.status, "delivered");
        assert!(outcome.send_execution.send_attempted);
        assert_eq!(outcome.send_execution.bot_api_ack, Some(true));
        assert!(outcome.send_execution.cursor_written);
        assert!(outcome.send_execution.external_send);

        let cursor = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(cursor.next_update_offset, Some(50));
        let model_json = serde_json::to_string(&outcome.model_execution).expect("serialize");
        let send_json = serde_json::to_string(&outcome.send_execution).expect("serialize");
        assert!(!model_json.contains("private pipeline prompt"));
        assert!(!model_json.contains("private pipeline response"));
        assert!(!send_json.contains("private pipeline response"));
        assert!(!send_json.contains("6476198178"));
        assert!(!send_json.contains(token));
    }

    #[test]
    fn drain_pipeline_respects_model_gate_before_runner_and_send() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let update = serde_json::json!({
            "update_id": 49,
            "message": {
                "message_id": 15,
                "text": "private pipeline prompt",
                "chat": { "id": 6476198178_i64, "type": "private" }
            }
        });
        let gates = NativeTelegramGatewayGateSummary {
            delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
            delivery_approval_gate_enabled: true,
            live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
            live_read_gate_enabled: true,
            model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
            model_turn_gate_enabled: false,
            send_gate_env: TELEGRAM_SEND_GATE_ENV,
            send_gate_enabled: true,
            readiness_summary_performs_live_read: false,
            readiness_summary_invokes_model: false,
            readiness_summary_sends_message: false,
        };

        let outcome = execute_telegram_drain_pipeline_for_updates(
            &[update],
            Some(49),
            Some("123456:ABCDEFGHIJKLMNOPQRSTUVWX"),
            &gates,
            &cursor_path,
            |_| panic!("model runner must not run while model gate is closed"),
            |_, _, _, _| panic!("sendMessage must not run without model output"),
        );

        assert_eq!(outcome.model_execution.status, "gated");
        assert!(!outcome.model_execution.session_runner_invoked);
        assert!(!outcome.model_execution.model_output_present);
        assert_eq!(outcome.send_execution.status, "waiting_model_output");
        assert!(!outcome.send_execution.send_attempted);
        assert!(!outcome.send_execution.cursor_written);
        assert!(!outcome.send_execution.external_send);
        assert!(
            outcome
                .model_execution
                .error
                .unwrap()
                .contains(TELEGRAM_MODEL_TURN_GATE_ENV)
        );
        assert!(!cursor_path.exists());
    }

    #[test]
    fn mlx_local_model_ref_parser_requires_provider_prefix() {
        assert_eq!(
            parse_mlx_local_model_ref(
                " mlx-local/froggeric/Qwen3.6-35B-A3B-Uncensored-Heretic-MLX-4bit "
            )
            .as_deref(),
            Some("froggeric/Qwen3.6-35B-A3B-Uncensored-Heretic-MLX-4bit")
        );
        assert_eq!(parse_mlx_local_model_ref("mlx-local/   "), None);
        assert_eq!(parse_mlx_local_model_ref("openai/gpt-5.5").as_deref(), None);
    }

    #[test]
    fn openai_chat_completion_text_extractor_accepts_message_or_text() {
        let chat = serde_json::json!({
            "choices": [{
                "message": { "role": "assistant", "content": "  local reply  " }
            }]
        });
        assert_eq!(
            extract_openai_chat_completion_text(&chat).expect("chat content"),
            "local reply"
        );

        let completion = serde_json::json!({
            "choices": [{ "text": "  completion reply  " }]
        });
        assert_eq!(
            extract_openai_chat_completion_text(&completion).expect("completion text"),
            "completion reply"
        );

        let missing = serde_json::json!({ "choices": [{ "message": { "content": "   " }}]});
        assert!(
            extract_openai_chat_completion_text(&missing)
                .expect_err("empty text rejected")
                .contains("did not include text")
        );
    }

    #[test]
    fn hepta_exec_child_args_are_ephemeral_read_only_and_capture_last_message() {
        let last_message_path = Path::new("/tmp/hepta-telegram-last-message.txt");
        let args = hepta_exec_child_args(last_message_path, "private prompt");

        assert_eq!(args[0], "-c");
        assert_eq!(args[1], "approval_policy=\"never\"");
        assert_eq!(args[2], "exec");
        assert!(args.contains(&"--skip-git-repo-check".to_string()));
        assert!(args.contains(&"--ephemeral".to_string()));
        assert!(args.contains(&"--ignore-rules".to_string()));
        assert_eq!(
            args.windows(2)
                .find(|pair| pair[0] == "--sandbox")
                .map(|pair| pair[1].as_str()),
            Some("read-only")
        );
        assert_eq!(
            args.windows(2)
                .find(|pair| pair[0] == "--output-last-message")
                .map(|pair| pair[1].as_str()),
            Some("/tmp/hepta-telegram-last-message.txt")
        );
        assert_eq!(args.last().map(String::as_str), Some("private prompt"));
    }

    #[test]
    fn drain_once_without_gates_stops_before_side_effects() {
        let gates = NativeTelegramGatewayGateSummary {
            delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
            delivery_approval_gate_enabled: false,
            live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
            live_read_gate_enabled: false,
            model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
            model_turn_gate_enabled: false,
            send_gate_env: TELEGRAM_SEND_GATE_ENV,
            send_gate_enabled: false,
            readiness_summary_performs_live_read: false,
            readiness_summary_invokes_model: false,
            readiness_summary_sends_message: false,
        };
        let status = telegram_drain_once_status_with_gates(true, gates);
        assert_eq!(status.status, "gated");
        assert_eq!(
            status.execution_plan.first_missing_gate,
            Some(TELEGRAM_DELIVERY_APPROVED_ENV)
        );
        assert!(!status.execution_plan.all_required_gates_enabled);
        assert!(status.execution_plan.receive_before_model);
        assert!(status.execution_plan.send_after_model_success);
        assert!(status.execution_plan.cursor_commit_after_delivery);
        assert!(!status.execution_plan.status_probe_executes_pipeline);
        assert!(status.cursor_plan.duplicate_suppression_ready);
        assert!(status.inspection.parser_ready);
        assert_eq!(status.inspection.update_count, 0);
        assert!(status.model_turn_plan.planner_ready);
        assert!(status.invocation_request.request_builder_ready);
        assert!(!status.invocation_request.candidate_present);
        assert!(!status.invocation_request.runner_invocation_allowed);
        assert_eq!(status.model_execution.status, "gated");
        assert!(!status.model_execution.session_runner_invoked);
        assert!(status.send_plan.send_plan_ready);
        assert!(!status.send_plan.delivery_performed_by_status);
        assert!(status.send_request.request_builder_ready);
        assert!(!status.send_request.model_output_present);
        assert!(!status.send_request.send_allowed);
        assert_eq!(status.send_execution.status, "gated");
        assert!(!status.send_execution.send_attempted);
        assert!(!status.send_execution.cursor_written);
        assert!(!status.live_read_started);
        assert!(!status.model_turn_started);
        assert!(!status.send_started);
        assert!(!status.cursor_written);
        assert!(!status.external_network_read);
        assert!(!status.external_network_write);
        assert!(!status.external_send);
        assert!(!status.raw_update_payload_exposed);
        assert!(!status.raw_prompt_text_exposed);
        assert!(!status.raw_response_text_exposed);
        assert!(!status.raw_token_exposed);
        assert!(
            status
                .error
                .unwrap()
                .contains(TELEGRAM_DELIVERY_APPROVED_ENV)
        );
    }

    #[test]
    fn drain_once_with_model_and_send_gates_still_waits_for_live_read() {
        let gates = NativeTelegramGatewayGateSummary {
            delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
            delivery_approval_gate_enabled: true,
            live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
            live_read_gate_enabled: false,
            model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
            model_turn_gate_enabled: true,
            send_gate_env: TELEGRAM_SEND_GATE_ENV,
            send_gate_enabled: true,
            readiness_summary_performs_live_read: false,
            readiness_summary_invokes_model: false,
            readiness_summary_sends_message: false,
        };
        let status = telegram_drain_once_status_with_gates(true, gates);
        assert_eq!(status.status, "gated");
        assert!(!status.execution_plan.all_required_gates_enabled);
        assert_eq!(
            status.execution_plan.first_missing_gate,
            Some(TELEGRAM_LIVE_READ_ENV)
        );
        assert!(!status.execution_plan.status_probe_executes_pipeline);
        assert!(status.cursor_plan.duplicate_suppression_ready);
        assert!(status.model_turn_plan.planner_ready);
        assert!(status.invocation_request.request_builder_ready);
        assert!(!status.invocation_request.candidate_present);
        assert!(status.invocation_request.model_turn_gate_enabled);
        assert!(!status.invocation_request.runner_invocation_allowed);
        assert_eq!(status.model_execution.status, "waiting_candidate");
        assert!(!status.model_execution.session_runner_invoked);
        assert!(status.send_plan.send_plan_ready);
        assert_eq!(status.send_execution.status, "waiting_model_output");
        assert!(!status.send_execution.send_attempted);
        assert!(!status.live_read_started);
        assert!(!status.model_turn_started);
        assert!(!status.send_started);
        assert!(!status.cursor_written);
        assert!(!status.external_network_read);
        assert!(!status.external_network_write);
        assert!(!status.external_send);
        assert!(!status.raw_prompt_text_exposed);
        assert!(!status.raw_response_text_exposed);
        assert!(!status.raw_token_exposed);
        assert!(status.error.unwrap().contains(TELEGRAM_LIVE_READ_ENV));
    }

    #[test]
    fn cursor_status_reads_next_update_offset_without_writing() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        fs::write(
            &cursor_path,
            r#"{"next_update_offset": 43, "updated_at_unix_ms": 123}"#,
        )
        .expect("write cursor");

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "ready");
        assert!(status.cursor_file_present);
        assert!(status.cursor_parse_ok);
        assert_eq!(status.next_update_offset, Some(43));
        assert!(status.cursor_represents_next_update_offset);
        assert!(status.duplicate_suppression_rule_valid);
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_persisted);
    }

    #[test]
    fn cursor_status_reads_legacy_next_server_offset_without_writing() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        fs::write(
            &cursor_path,
            r#"{"next_server_offset": 917025960, "last_drained_update_id": 917025959}"#,
        )
        .expect("write cursor");

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "ready");
        assert!(status.cursor_file_present);
        assert!(status.cursor_parse_ok);
        assert_eq!(status.next_update_offset, Some(917025960));
        assert!(status.cursor_represents_next_update_offset);
        assert!(status.duplicate_suppression_rule_valid);
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_persisted);
    }

    #[test]
    fn cursor_status_derives_legacy_next_offset_from_last_drained_id() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        fs::write(&cursor_path, r#"{"last_drained_update_id": 917025959}"#).expect("write cursor");

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "ready");
        assert!(status.cursor_file_present);
        assert!(status.cursor_parse_ok);
        assert_eq!(status.next_update_offset, Some(917025960));
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_persisted);
    }

    #[test]
    fn cursor_status_rejects_negative_offsets() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        fs::write(&cursor_path, r#"{"next_update_offset": -1}"#).expect("write cursor");

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "attention");
        assert!(status.cursor_file_present);
        assert!(!status.cursor_parse_ok);
        assert_eq!(status.next_update_offset, None);
        assert!(
            status
                .error
                .unwrap()
                .contains("next_update_offset must be non-negative")
        );
    }

    #[test]
    fn cursor_write_helper_persists_next_offset_without_raw_payload() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("nested").join("cursor.json");

        write_telegram_cursor_next_update_offset(&cursor_path, 77).expect("write cursor");
        let raw = fs::read_to_string(&cursor_path).expect("read cursor");
        assert!(raw.contains("\"schema\": \"hepta.telegram.cursor.v1\""));
        assert!(raw.contains("\"next_update_offset\": 77"));
        assert!(raw.contains("\"raw_update_payload_persisted\": false"));

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "ready");
        assert_eq!(status.next_update_offset, Some(77));
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_persisted);
    }

    #[test]
    fn cursor_write_helper_rejects_negative_offsets() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");

        let err = write_telegram_cursor_next_update_offset(&cursor_path, -1)
            .expect_err("negative offset should be rejected");
        assert!(err.contains("next_update_offset must be non-negative"));
        assert!(!cursor_path.exists());
    }

    #[test]
    fn receive_once_without_live_gate_is_gated_and_side_effect_free() {
        let report = telegram_receive_once_status_with_gate(true, 999, false);
        assert_eq!(report.status, "gated");
        assert_eq!(report.limit, 20);
        assert!(!report.live_read_gate_enabled);
        assert!(!report.external_network_read);
        assert!(!report.external_send);
        assert!(!report.model_turn_started);
        assert!(!report.cursor_written);
        assert!(!report.raw_update_payload_exposed);
        assert!(!report.raw_token_exposed);
        assert!(report.error.unwrap().contains(TELEGRAM_LIVE_READ_ENV));
    }

    #[test]
    fn get_updates_conflict_error_is_busy_not_attention() {
        let conflict = "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request; make sure that only one bot instance is running";
        assert!(is_telegram_get_updates_conflict_error(conflict));

        let auth_error = "Telegram Bot API getUpdates HTTP status 401; description=Unauthorized";
        assert!(!is_telegram_get_updates_conflict_error(auth_error));
    }

    #[test]
    fn send_transient_error_classifier_keeps_auth_failures_terminal() {
        assert!(is_telegram_send_transient_error(
            "Telegram Bot API sendMessage HTTP status 429; description=Too Many Requests"
        ));
        assert!(is_telegram_send_transient_error(
            "Telegram Bot API sendMessage HTTP status 503; description=temporary outage"
        ));
        assert!(!is_telegram_send_transient_error(
            "Telegram Bot API sendMessage HTTP status 401; description=Unauthorized"
        ));
    }

    #[test]
    fn get_updates_transient_error_classifier_keeps_conflicts_busy() {
        assert!(is_telegram_get_updates_transient_error(
            "Telegram Bot API getUpdates request failed: error sending request"
        ));
        assert!(is_telegram_get_updates_transient_error(
            "Telegram Bot API getUpdates HTTP status 503; description=temporary outage"
        ));
        assert!(!is_telegram_get_updates_transient_error(
            "Telegram Bot API getUpdates HTTP status 401; description=Unauthorized"
        ));
        assert!(is_telegram_get_updates_conflict_error(
            "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request"
        ));
    }

    #[test]
    fn production_readiness_requires_minimum_soak_observations() {
        let poll_loop = ready_poll_loop_status();
        let cursor = ready_cursor_status();
        let guards = ready_production_guards();
        let observation = live_soak_observation(2, 0, Some("planned"), Some(true));

        let readiness = telegram_production_readiness_status_from_parts(
            true,
            &poll_loop,
            &cursor,
            &guards,
            &observation,
        );

        assert!(!readiness.ready);
        assert_eq!(readiness.status, "warming");
        assert_eq!(
            readiness.min_poll_iterations,
            DEFAULT_TELEGRAM_SOAK_MIN_POLLS
        );
        assert!(readiness.observation_ready == false);
        assert!(
            readiness
                .readiness_blockers
                .contains(&"observation_min_poll_iterations")
        );
        assert!(!readiness.raw_update_payload_exposed);
        assert!(!readiness.raw_prompt_text_exposed);
        assert!(!readiness.raw_response_text_exposed);
        assert!(!readiness.raw_token_exposed);
    }

    #[test]
    fn production_readiness_is_ready_after_clean_guarded_soak() {
        let poll_loop = ready_poll_loop_status();
        let cursor = ready_cursor_status();
        let guards = ready_production_guards();
        let observation = live_soak_observation(3, 0, Some("planned"), Some(true));

        let readiness = telegram_production_readiness_status_from_parts(
            true,
            &poll_loop,
            &cursor,
            &guards,
            &observation,
        );

        assert!(readiness.ready);
        assert_eq!(readiness.status, "ready");
        assert!(readiness.poll_loop_armed);
        assert!(readiness.cursor_ready);
        assert!(readiness.production_guards_ready);
        assert!(readiness.observation_ready);
        assert!(readiness.attention_budget_ok);
        assert!(readiness.recent_bot_api_ok);
        assert!(readiness.redaction_guards_ok);
        assert!(readiness.readiness_blockers.is_empty());
        assert!(
            readiness
                .readiness_warnings
                .contains(&"no_messages_drained_since_gateway_start")
        );
    }

    #[test]
    fn production_readiness_flags_attention_budget_failures() {
        let poll_loop = ready_poll_loop_status();
        let cursor = ready_cursor_status();
        let guards = ready_production_guards();
        let observation = live_soak_observation(3, 1, Some("attention"), Some(false));

        let readiness = telegram_production_readiness_status_from_parts(
            true,
            &poll_loop,
            &cursor,
            &guards,
            &observation,
        );

        assert!(!readiness.ready);
        assert_eq!(readiness.status, "attention");
        assert!(!readiness.attention_budget_ok);
        assert!(!readiness.recent_bot_api_ok);
        assert!(
            readiness
                .readiness_blockers
                .contains(&"attention_budget_exceeded")
        );
        assert!(
            readiness
                .readiness_blockers
                .contains(&"bot_api_recent_failure")
        );
    }

    fn ready_poll_loop_status() -> NativeTelegramPollLoopStatus {
        NativeTelegramPollLoopStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested: true,
            status: "armed",
            poll_loop_gate_env: TELEGRAM_POLL_LOOP_ENV,
            poll_loop_gate_enabled: true,
            delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
            delivery_approval_gate_enabled: true,
            poll_ms: 1500,
            drain_once_endpoint: "/api/telegram-drain-once",
            worker_spawned_by_status: false,
            loop_invokes_drain_once: true,
            requires_live_read_gate: TELEGRAM_LIVE_READ_ENV,
            requires_model_turn_gate: TELEGRAM_MODEL_TURN_GATE_ENV,
            requires_send_gate: TELEGRAM_SEND_GATE_ENV,
            requires_delivery_approval_gate: TELEGRAM_DELIVERY_APPROVED_ENV,
            external_network_read_by_status: false,
            external_send_by_status: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
            raw_token_exposed: false,
            next_migration_slice: "test",
        }
    }

    fn ready_cursor_status() -> NativeTelegramCursorStatus {
        NativeTelegramCursorStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested: true,
            status: "ready",
            cursor_path: TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present: true,
            cursor_parse_ok: true,
            next_update_offset: Some(917025970),
            cursor_represents_next_update_offset: true,
            duplicate_suppression_rule_valid: true,
            cursor_write_policy: "write only after model output is delivered or duplicate suppression is recorded",
            cursor_written: false,
            raw_update_payload_persisted: false,
            error: None,
            next_migration_slice: "test",
        }
    }

    fn ready_production_guards() -> NativeTelegramProductionGuardStatus {
        NativeTelegramProductionGuardStatus {
            read_max_attempts_env: TELEGRAM_READ_MAX_ATTEMPTS_ENV,
            read_max_attempts: 3,
            read_retry_backoff_env: TELEGRAM_READ_RETRY_BACKOFF_ENV,
            read_retry_backoff_ms: 700,
            retry_transient_read_errors: true,
            typing_keepalive_env: TELEGRAM_TYPING_KEEPALIVE_ENV,
            typing_keepalive_enabled: true,
            typing_keepalive_interval_ms: 4000,
            model_timeout_env: TELEGRAM_MODEL_TIMEOUT_ENV,
            model_timeout_ms: 120000,
            model_failure_fallback_env: TELEGRAM_MODEL_FAILURE_FALLBACK_ENV,
            model_failure_fallback_enabled: true,
            send_min_interval_env: TELEGRAM_SEND_MIN_INTERVAL_ENV,
            send_min_interval_ms: 1200,
            send_max_attempts_env: TELEGRAM_SEND_MAX_ATTEMPTS_ENV,
            send_max_attempts: 3,
            send_retry_backoff_env: TELEGRAM_SEND_RETRY_BACKOFF_ENV,
            send_retry_backoff_ms: 700,
            retry_transient_send_errors: true,
            rate_limit_scope: "in-process per chat id; reset on gateway restart",
            raw_token_exposed: false,
        }
    }

    fn live_soak_observation(
        poll_iterations: u64,
        attention_count: u64,
        last_status: Option<&str>,
        last_bot_api_ok: Option<bool>,
    ) -> NativeTelegramLiveSoakObservationReport {
        NativeTelegramLiveSoakObservationReport {
            poll_iterations,
            drained_count: 0,
            busy_count: 0,
            attention_count,
            empty_read_count: poll_iterations.saturating_sub(attention_count),
            model_turn_started_count: 0,
            send_started_count: 0,
            cursor_written_count: 0,
            external_send_count: 0,
            last_drained_at_unix_ms: None,
            last_drained_next_update_offset: None,
            last_observed_at_unix_ms: Some(1),
            last_status: last_status.map(str::to_string),
            last_error: None,
            last_bot_api_ok,
            last_get_updates_offset: Some(917025970),
            last_local_next_update_offset: None,
            last_update_count: 0,
            last_allowed_update_count: 0,
            last_model_turn_started: false,
            last_send_started: false,
            last_cursor_written: false,
            last_external_send: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
            raw_token_exposed: false,
        }
    }
}
