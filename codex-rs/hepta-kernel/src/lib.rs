//! Hepta kernel boundary.
//!
//! This crate owns the fused turn-level contract for Hepta. Codex remains a
//! powerful internal execution engine, but the product kernel owns turn
//! planning, memory/intelligence context, plugin capability posture, and
//! post-turn persistence boundaries.

use hepta_contracts::LEGACY_CONTROL_UI_MUTATION_NEXT_ACTION;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use sha2::Digest;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

mod safety_gate;
mod telegram_config;
mod telegram_cursor;
mod telegram_production_guard;
mod telegram_transport;

pub use safety_gate::HeptaKernelAdmissionEvidence;
pub use safety_gate::HeptaKernelExactCandidateBinding;
pub use safety_gate::HeptaKernelPolicyEvidence;
pub use safety_gate::HeptaKernelSafetyAdmission;
pub use safety_gate::HeptaKernelSafetyAuthorization;
pub use safety_gate::HeptaKernelSafetyGate;
pub use safety_gate::HeptaKernelSafetyGateError;
pub use safety_gate::admission_reason;
pub use telegram_config::HeptaKernelTelegramConfigMetadata;
pub use telegram_config::HeptaKernelTelegramConfigStatus;
pub use telegram_config::HeptaKernelTelegramConfigStatusInput;
pub use telegram_config::HeptaKernelTelegramTokenObservation;
pub use telegram_config::HeptaKernelTelegramTokenObservationInput;
pub use telegram_config::build_hepta_kernel_telegram_config_status;
pub use telegram_config::extract_hepta_kernel_telegram_config_metadata;
pub use telegram_config::hepta_kernel_telegram_env_truthy_value;
pub use telegram_config::hepta_kernel_telegram_env_u64_value;
pub use telegram_config::hepta_kernel_telegram_normalize_binding_id;
pub use telegram_config::hepta_kernel_telegram_token_observation;
pub use telegram_config::resolve_hepta_kernel_telegram_secret_provider_path;
pub use telegram_cursor::HeptaKernelTelegramCursorPlan;
pub use telegram_cursor::HeptaKernelTelegramCursorStatus;
pub use telegram_cursor::HeptaKernelTelegramCursorStatusInput;
pub use telegram_cursor::build_hepta_kernel_telegram_cursor_status;
pub use telegram_cursor::hepta_kernel_telegram_cursor_body;
pub use telegram_cursor::hepta_kernel_telegram_cursor_duplicate_rule_valid;
pub use telegram_cursor::hepta_kernel_telegram_update_already_drained;
pub use telegram_cursor::parse_hepta_kernel_telegram_cursor_next_update_offset;
pub use telegram_production_guard::HeptaKernelTelegramProductionGuardPolicyInput;
pub use telegram_production_guard::HeptaKernelTelegramProductionGuardStatus;
pub use telegram_production_guard::HeptaKernelTelegramProductionGuardStatusInput;
pub use telegram_production_guard::build_hepta_kernel_telegram_production_guard_status;
pub use telegram_production_guard::build_hepta_kernel_telegram_production_guard_status_from_policy;
pub use telegram_production_guard::hepta_kernel_telegram_model_timeout;
pub use telegram_production_guard::hepta_kernel_telegram_model_timeout_ms;
pub use telegram_production_guard::hepta_kernel_telegram_read_max_attempts_policy;
pub use telegram_production_guard::hepta_kernel_telegram_read_retry_backoff_policy;
pub use telegram_production_guard::hepta_kernel_telegram_send_max_attempts_policy;
pub use telegram_production_guard::hepta_kernel_telegram_send_min_interval_policy;
pub use telegram_production_guard::hepta_kernel_telegram_send_retry_backoff_policy;
pub use telegram_production_guard::hepta_kernel_telegram_typing_keepalive_interval_policy;
pub use telegram_transport::HeptaKernelTelegramGetUpdatesProviderResultInput;
pub use telegram_transport::HeptaKernelTelegramGetUpdatesProviderResultPlan;
pub use telegram_transport::HeptaKernelTelegramSendProviderResultInput;
pub use telegram_transport::HeptaKernelTelegramSendProviderResultPlan;
pub use telegram_transport::HeptaKernelTelegramTransportPlan;
pub use telegram_transport::hepta_kernel_telegram_bot_api_client_build_error;
pub use telegram_transport::hepta_kernel_telegram_bot_api_http_status_error;
pub use telegram_transport::hepta_kernel_telegram_bot_api_json_parse_error;
pub use telegram_transport::hepta_kernel_telegram_bot_api_request_failed_error;
pub use telegram_transport::hepta_kernel_telegram_bot_token_shape_ok;
pub use telegram_transport::hepta_kernel_telegram_error_is_transient;
pub use telegram_transport::hepta_kernel_telegram_get_updates_error_is_conflict;
pub use telegram_transport::hepta_kernel_telegram_get_updates_error_is_transient;
pub use telegram_transport::hepta_kernel_telegram_get_updates_query;
pub use telegram_transport::hepta_kernel_telegram_get_updates_should_retry;
pub use telegram_transport::hepta_kernel_telegram_send_chat_action_request_body;
pub use telegram_transport::hepta_kernel_telegram_send_error_is_transient;
pub use telegram_transport::hepta_kernel_telegram_send_message_request_body;
pub use telegram_transport::hepta_kernel_telegram_send_rate_limit_sleep_for;
pub use telegram_transport::hepta_kernel_telegram_send_should_retry;
pub use telegram_transport::hepta_kernel_telegram_transport_plan_for_config_status;
pub use telegram_transport::hepta_kernel_telegram_typing_keepalive_should_start;
pub use telegram_transport::plan_hepta_kernel_telegram_get_updates_provider_result;
pub use telegram_transport::plan_hepta_kernel_telegram_send_provider_result;
pub use telegram_transport::redact_hepta_kernel_telegram_token_like_text;

include!("kernel_parts/native_post.rs");
include!("kernel_parts/telegram_runtime.rs");

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
