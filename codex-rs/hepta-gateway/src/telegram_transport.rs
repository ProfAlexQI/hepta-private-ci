use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use serde_json::Value;

use crate::telegram_config::NativeTelegramConfigStatus;
use crate::telegram_cursor::write_telegram_cursor_next_update_offset;
use crate::telegram_delivery::append_telegram_delivery_lifecycle_record;
use crate::telegram_delivery::telegram_delivery_lifecycle_record;
use crate::telegram_policy::NativeTelegramReplyTargetMaterial;
use crate::telegram_policy::NativeTelegramSendExecutionReport;
use crate::telegram_policy::NativeTelegramSendRequestPlan;
use hepta_runtime::native_telegram_bot_api_client_build_error;
use hepta_runtime::native_telegram_bot_api_http_status_error;
use hepta_runtime::native_telegram_bot_api_json_parse_error;
use hepta_runtime::native_telegram_bot_api_request_failed_error;
use hepta_runtime::native_telegram_bot_token_shape_ok;
use hepta_runtime::native_telegram_get_updates_error_is_conflict;
use hepta_runtime::native_telegram_get_updates_error_is_transient;
use hepta_runtime::native_telegram_get_updates_query;
use hepta_runtime::native_telegram_get_updates_should_retry;
use hepta_runtime::native_telegram_read_max_attempts_policy;
use hepta_runtime::native_telegram_read_retry_backoff_policy;
use hepta_runtime::native_telegram_send_chat_action_request_body;
use hepta_runtime::native_telegram_send_error_is_transient;
use hepta_runtime::native_telegram_send_max_attempts_policy;
use hepta_runtime::native_telegram_send_message_request_body;
use hepta_runtime::native_telegram_send_min_interval_policy;
use hepta_runtime::native_telegram_send_rate_limit_sleep_for;
use hepta_runtime::native_telegram_send_retry_backoff_policy;
use hepta_runtime::native_telegram_send_should_retry;
use hepta_runtime::native_telegram_transport_plan_for_config_status;
use hepta_runtime::native_telegram_typing_keepalive_interval_policy;
use hepta_runtime::native_telegram_typing_keepalive_should_start;
use hepta_runtime::plan_native_telegram_get_updates_provider_result;
use hepta_runtime::plan_native_telegram_send_execution_preflight;
use hepta_runtime::plan_native_telegram_send_provider_result;
use hepta_runtime::redact_native_telegram_token_like_text;

pub use hepta_runtime::DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS;
pub use hepta_runtime::DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS;
pub use hepta_runtime::DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS;
pub use hepta_runtime::DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS;
pub use hepta_runtime::DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS;
pub use hepta_runtime::MAX_TELEGRAM_READ_MAX_ATTEMPTS;
pub use hepta_runtime::MAX_TELEGRAM_READ_RETRY_BACKOFF_MS;
pub use hepta_runtime::MAX_TELEGRAM_SEND_MAX_ATTEMPTS;
pub use hepta_runtime::MAX_TELEGRAM_SEND_MIN_INTERVAL_MS;
pub use hepta_runtime::MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS;
pub use hepta_runtime::MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS;
pub use hepta_runtime::NativeTelegramGetUpdatesProviderResultInput;
pub use hepta_runtime::NativeTelegramSendExecutionPreflightInput;
pub use hepta_runtime::NativeTelegramSendPlan;
pub use hepta_runtime::NativeTelegramSendProviderResultInput;
pub use hepta_runtime::NativeTelegramTransportPlan;
pub use hepta_runtime::TELEGRAM_ALLOWED_UPDATES;
const TELEGRAM_BOT_API_BASE_URL: &str = "https://api.telegram.org";
static TELEGRAM_SEND_RATE_LIMITS: OnceLock<Mutex<HashMap<i64, Instant>>> = OnceLock::new();

pub struct TelegramTypingKeepalive {
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

pub fn telegram_transport_plan_for_config_status(
    config: &NativeTelegramConfigStatus,
) -> NativeTelegramTransportPlan {
    native_telegram_transport_plan_for_config_status(config)
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramSendExecutionInput<'a> {
    pub token: Option<&'a str>,
    pub model_output: Option<&'a str>,
    pub reply_target: Option<&'a NativeTelegramReplyTargetMaterial>,
    pub candidate_next_update_offset: Option<i64>,
    pub send_gate_env: &'static str,
    pub send_gate_enabled: bool,
    pub cursor_path: &'a Path,
    pub delivery_ledger_path: &'a Path,
    pub send_max_attempts: u64,
    pub send_retry_backoff: Duration,
}

pub fn telegram_get_updates_query(
    limit: usize,
    offset: Option<i64>,
) -> Vec<(&'static str, String)> {
    native_telegram_get_updates_query(limit, offset)
}

pub fn telegram_typing_keepalive_interval_policy(value_ms: Option<u64>) -> Duration {
    native_telegram_typing_keepalive_interval_policy(value_ms)
}

pub fn telegram_read_max_attempts_policy(value: Option<u64>) -> u64 {
    native_telegram_read_max_attempts_policy(value)
}

pub fn telegram_read_retry_backoff_policy(value_ms: Option<u64>) -> Duration {
    native_telegram_read_retry_backoff_policy(value_ms)
}

pub fn telegram_send_min_interval_policy(value_ms: Option<u64>) -> Duration {
    native_telegram_send_min_interval_policy(value_ms)
}

pub fn telegram_send_max_attempts_policy(value: Option<u64>) -> u64 {
    native_telegram_send_max_attempts_policy(value)
}

pub fn telegram_send_retry_backoff_policy(value_ms: Option<u64>) -> Duration {
    native_telegram_send_retry_backoff_policy(value_ms)
}

pub fn telegram_send_chat_action_request_body(chat_id: i64) -> Result<Value, String> {
    native_telegram_send_chat_action_request_body(chat_id)
}

pub fn telegram_send_message_request_body(
    message_text: &str,
    chat_id: i64,
    reply_to_message_id: Option<i64>,
) -> Result<Value, String> {
    native_telegram_send_message_request_body(message_text, chat_id, reply_to_message_id)
}

pub fn telegram_call_get_updates_once(
    token: &str,
    limit: usize,
    offset: Option<i64>,
    timeout: Duration,
) -> Result<Value, String> {
    let endpoint = telegram_bot_api_endpoint(token, "getUpdates")?;
    let query = telegram_get_updates_query(limit, offset);
    let client = telegram_blocking_client(timeout, "getUpdates")?;
    let response = client.get(endpoint).query(&query).send().map_err(|error| {
        native_telegram_bot_api_request_failed_error("getUpdates", &error.without_url().to_string())
    })?;
    telegram_bot_api_json_response(response, "getUpdates")
}

pub fn telegram_call_send_message(
    token: &str,
    chat_id: i64,
    message_text: &str,
    reply_to_message_id: Option<i64>,
    timeout: Duration,
) -> Result<Value, String> {
    let endpoint = telegram_bot_api_endpoint(token, "sendMessage")?;
    let body = telegram_send_message_request_body(message_text, chat_id, reply_to_message_id)?;
    let client = telegram_blocking_client(timeout, "sendMessage")?;
    let response = client.post(endpoint).json(&body).send().map_err(|error| {
        native_telegram_bot_api_request_failed_error(
            "sendMessage",
            &error.without_url().to_string(),
        )
    })?;
    telegram_bot_api_json_response(response, "sendMessage")
}

pub fn telegram_call_send_chat_action(
    token: &str,
    chat_id: i64,
    timeout: Duration,
) -> Result<Value, String> {
    let endpoint = telegram_bot_api_endpoint(token, "sendChatAction")?;
    let body = telegram_send_chat_action_request_body(chat_id)?;
    let client = telegram_blocking_client(timeout, "sendChatAction")?;
    let response = client.post(endpoint).json(&body).send().map_err(|error| {
        native_telegram_bot_api_request_failed_error(
            "sendChatAction",
            &error.without_url().to_string(),
        )
    })?;
    telegram_bot_api_json_response(response, "sendChatAction")
}

pub fn telegram_typing_keepalive_should_start(enabled: bool, token: &str, chat_id: i64) -> bool {
    native_telegram_typing_keepalive_should_start(enabled, token, chat_id)
}

pub fn telegram_start_typing_keepalive<F>(
    enabled: bool,
    token: &str,
    chat_id: i64,
    interval: Duration,
    send_chat_action: F,
) -> Option<TelegramTypingKeepalive>
where
    F: Fn(&str, i64) -> Result<Value, String> + Send + Sync + 'static,
{
    if !telegram_typing_keepalive_should_start(enabled, token, chat_id) {
        return None;
    }

    let token = token.to_string();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let interval = if interval.is_zero() {
        Duration::from_millis(1_000)
    } else {
        interval
    };
    let send_chat_action = Arc::new(send_chat_action);
    let handle = thread::spawn(move || {
        while !thread_stop.load(Ordering::Relaxed) {
            let _ = send_chat_action(&token, chat_id);
            let started = std::time::Instant::now();
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

pub fn telegram_send_rate_limit_sleep_for(
    last_elapsed: Option<Duration>,
    min_interval: Duration,
) -> Duration {
    native_telegram_send_rate_limit_sleep_for(last_elapsed, min_interval)
}

pub fn telegram_wait_for_send_rate_limit(chat_id: i64, min_interval: Duration) {
    let sleep_for = telegram_note_send_rate_limit(chat_id, min_interval);
    if !sleep_for.is_zero() {
        thread::sleep(sleep_for);
    }
}

pub fn telegram_bot_token_shape_ok(token: &str) -> bool {
    native_telegram_bot_token_shape_ok(token)
}

pub fn telegram_redact_token_like_text(text: &str) -> String {
    redact_native_telegram_token_like_text(text)
}

pub fn telegram_get_updates_error_is_conflict(error: &str) -> bool {
    native_telegram_get_updates_error_is_conflict(error)
}

pub fn telegram_send_error_is_transient(error: &str) -> bool {
    native_telegram_send_error_is_transient(error)
}

pub fn telegram_get_updates_error_is_transient(error: &str) -> bool {
    native_telegram_get_updates_error_is_transient(error)
}

pub fn telegram_get_updates_should_retry(attempt: u64, max_attempts: u64, error: &str) -> bool {
    native_telegram_get_updates_should_retry(attempt, max_attempts, error)
}

pub fn telegram_send_should_retry(attempt: u64, max_attempts: u64, error: &str) -> bool {
    native_telegram_send_should_retry(attempt, max_attempts, error)
}

pub fn execute_telegram_send_after_model_output<F>(
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
    let request = NativeTelegramSendRequestPlan::from_model_output(
        model_output,
        input.reply_target.is_some(),
        input.candidate_next_update_offset,
        input.send_gate_env,
        input.send_gate_enabled,
    );
    let token = input
        .token
        .map(str::trim)
        .filter(|token| telegram_bot_token_shape_ok(token));
    let preflight =
        plan_native_telegram_send_execution_preflight(NativeTelegramSendExecutionPreflightInput {
            model_output_present: request.model_output_present,
            reply_target_available: request.reply_target_available,
            candidate_next_update_offset: request.candidate_next_update_offset,
            token_shape_ok: token.is_some(),
            send_gate_env: input.send_gate_env,
            send_gate_enabled: input.send_gate_enabled,
        });
    debug_assert_eq!(preflight.request, request);
    let mut report = preflight.report;
    if !preflight.execution_can_attempt_send {
        return report;
    };
    let model_output = model_output.expect("send preflight requires non-empty model output");
    let reply_target = input
        .reply_target
        .expect("send preflight requires reply target");
    let candidate_next_update_offset = input
        .candidate_next_update_offset
        .expect("send preflight requires next-update offset");
    let token = token.expect("send preflight requires valid Bot API token");

    report = report.with_delivery_ledger_write_attempted();
    let enqueued_record = telegram_delivery_lifecycle_record(
        "enqueued",
        input.candidate_next_update_offset,
        report.model_output_present,
        false,
        None,
        false,
        None,
    );
    match append_telegram_delivery_lifecycle_record(input.delivery_ledger_path, &enqueued_record) {
        Ok(()) => {
            report = report.with_delivery_ledger_written("enqueued");
        }
        Err(error) => {
            return report.with_redacted_attention_error(&error);
        }
    }

    report = report.with_sending_attempt_started();

    let max_attempts = input.send_max_attempts.max(1);
    for attempt in 1..=max_attempts {
        match send_message(
            token,
            reply_target.chat_id,
            model_output,
            reply_target.reply_to_message_id,
        ) {
            Ok(api) => {
                let provider_result = plan_native_telegram_send_provider_result(
                    NativeTelegramSendProviderResultInput {
                        attempt,
                        max_attempts,
                        api_result: Ok(&api),
                    },
                );
                report = report.with_bot_api_ack(provider_result.bot_api_ack);
                if provider_result.should_retry {
                    thread::sleep(input.send_retry_backoff);
                    continue;
                }
                if !provider_result.external_send {
                    let error = provider_result.error.unwrap_or_else(|| {
                        "Telegram Bot API sendMessage returned ok=false".to_string()
                    });
                    if let Err(ledger_error) = append_telegram_delivery_lifecycle_record(
                        input.delivery_ledger_path,
                        &telegram_delivery_lifecycle_record(
                            provider_result.delivery_ledger_stage.unwrap_or("failed"),
                            input.candidate_next_update_offset,
                            report.model_output_present,
                            true,
                            Some(false),
                            provider_result.provider_message_id_present,
                            Some(&error),
                        ),
                    ) {
                        return report.with_redacted_attention_error(&ledger_error);
                    }
                    return report
                        .with_delivery_ledger_written("failed")
                        .with_attention_error(error);
                }

                report = report.with_external_send(provider_result.external_send);
                match append_telegram_delivery_lifecycle_record(
                    input.delivery_ledger_path,
                    &telegram_delivery_lifecycle_record(
                        provider_result.delivery_ledger_stage.unwrap_or("acked"),
                        input.candidate_next_update_offset,
                        report.model_output_present,
                        true,
                        Some(true),
                        provider_result.provider_message_id_present,
                        None,
                    ),
                ) {
                    Ok(()) => {
                        report = report.with_delivery_ledger_written("acked");
                    }
                    Err(error) => {
                        return report.with_redacted_attention_error(&error);
                    }
                }
                report = report.with_cursor_commit_attempted();
                match write_telegram_cursor_next_update_offset(
                    input.cursor_path,
                    candidate_next_update_offset,
                ) {
                    Ok(()) => {
                        report = report.with_cursor_written();
                    }
                    Err(error) => {
                        report = report.with_redacted_attention_error(&error);
                    }
                }
                return report;
            }
            Err(error) => {
                let provider_result = plan_native_telegram_send_provider_result(
                    NativeTelegramSendProviderResultInput {
                        attempt,
                        max_attempts,
                        api_result: Err(&error),
                    },
                );
                if provider_result.should_retry {
                    thread::sleep(input.send_retry_backoff);
                    continue;
                }
                let error = provider_result.error.unwrap_or(error);
                if let Err(ledger_error) = append_telegram_delivery_lifecycle_record(
                    input.delivery_ledger_path,
                    &telegram_delivery_lifecycle_record(
                        provider_result.delivery_ledger_stage.unwrap_or("failed"),
                        input.candidate_next_update_offset,
                        report.model_output_present,
                        true,
                        None,
                        provider_result.provider_message_id_present,
                        Some(&error),
                    ),
                ) {
                    return report.with_redacted_attention_error(&ledger_error);
                }
                return report
                    .with_delivery_ledger_written("failed")
                    .with_attention_error(error);
            }
        }
    }

    report
}

pub fn telegram_get_updates_with_retry<F>(
    max_attempts: u64,
    retry_backoff: Duration,
    mut call_once: F,
) -> Result<Value, String>
where
    F: FnMut() -> Result<Value, String>,
{
    for attempt in 1..=max_attempts {
        match call_once() {
            Ok(api) => {
                let provider_result = plan_native_telegram_get_updates_provider_result(
                    NativeTelegramGetUpdatesProviderResultInput {
                        attempt,
                        max_attempts,
                        api_result: Ok(&api),
                    },
                );
                if provider_result.should_retry {
                    thread::sleep(retry_backoff);
                    continue;
                }
                return Ok(api);
            }
            Err(error) => {
                let provider_result = plan_native_telegram_get_updates_provider_result(
                    NativeTelegramGetUpdatesProviderResultInput {
                        attempt,
                        max_attempts,
                        api_result: Err(&error),
                    },
                );
                if provider_result.should_retry {
                    thread::sleep(retry_backoff);
                    continue;
                }
                return Err(provider_result.error.unwrap_or(error));
            }
        }
    }
    Err("Telegram Bot API getUpdates retry loop exited unexpectedly".to_string())
}

fn telegram_note_send_rate_limit(chat_id: i64, min_interval: Duration) -> Duration {
    if min_interval.is_zero() {
        return Duration::default();
    }
    let map = TELEGRAM_SEND_RATE_LIMITS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = match map.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let now = Instant::now();
    let sleep_for =
        telegram_send_rate_limit_sleep_for(guard.get(&chat_id).map(Instant::elapsed), min_interval);
    guard.insert(chat_id, now + sleep_for);
    sleep_for
}

fn telegram_bot_api_endpoint(token: &str, method: &str) -> Result<String, String> {
    let token = token.trim();
    if !telegram_bot_token_shape_ok(token) {
        return Err(format!(
            "Telegram Bot API {method} requires a valid Bot API token"
        ));
    }
    Ok(format!("{TELEGRAM_BOT_API_BASE_URL}/bot{token}/{method}"))
}

fn telegram_blocking_client(
    timeout: Duration,
    method: &str,
) -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|error| native_telegram_bot_api_client_build_error(method, &error.to_string()))
}

fn telegram_bot_api_json_response(
    response: reqwest::blocking::Response,
    method: &str,
) -> Result<Value, String> {
    let status = response.status();
    let body = response
        .json::<Value>()
        .map_err(|error| native_telegram_bot_api_json_parse_error(method, &error.to_string()))?;
    if status.is_success() {
        Ok(body)
    } else {
        Err(native_telegram_bot_api_http_status_error(
            method,
            status.as_u16(),
            body.get("description").and_then(Value::as_str),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS;
    use super::DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS;
    use super::DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS;
    use super::DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS;
    use super::DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS;
    use super::MAX_TELEGRAM_READ_MAX_ATTEMPTS;
    use super::MAX_TELEGRAM_READ_RETRY_BACKOFF_MS;
    use super::MAX_TELEGRAM_SEND_MAX_ATTEMPTS;
    use super::MAX_TELEGRAM_SEND_MIN_INTERVAL_MS;
    use super::MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS;
    use super::MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS;
    use super::NativeTelegramSendExecutionInput;
    use super::TELEGRAM_ALLOWED_UPDATES;
    use super::execute_telegram_send_after_model_output;
    use super::telegram_bot_token_shape_ok;
    use super::telegram_call_get_updates_once;
    use super::telegram_call_send_chat_action;
    use super::telegram_call_send_message;
    use super::telegram_get_updates_error_is_conflict;
    use super::telegram_get_updates_error_is_transient;
    use super::telegram_get_updates_query;
    use super::telegram_get_updates_should_retry;
    use super::telegram_get_updates_with_retry;
    use super::telegram_read_max_attempts_policy;
    use super::telegram_read_retry_backoff_policy;
    use super::telegram_redact_token_like_text;
    use super::telegram_send_chat_action_request_body;
    use super::telegram_send_error_is_transient;
    use super::telegram_send_max_attempts_policy;
    use super::telegram_send_message_request_body;
    use super::telegram_send_min_interval_policy;
    use super::telegram_send_rate_limit_sleep_for;
    use super::telegram_send_retry_backoff_policy;
    use super::telegram_send_should_retry;
    use super::telegram_start_typing_keepalive;
    use super::telegram_transport_plan_for_config_status;
    use super::telegram_typing_keepalive_interval_policy;
    use super::telegram_typing_keepalive_should_start;
    use crate::telegram_config::NativeTelegramConfigStatus;
    use crate::telegram_cursor::telegram_cursor_status_from_path;
    use crate::telegram_delivery::telegram_delivery_ledger_status_from_path;
    use crate::telegram_policy::NativeTelegramReplyTargetMaterial;
    use std::time::Duration;

    use serde_json::Value;

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
    fn env_policy_helpers_clamp_retry_typing_and_send_defaults() {
        assert_eq!(
            telegram_typing_keepalive_interval_policy(None),
            Duration::from_millis(DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS)
        );
        assert_eq!(
            telegram_typing_keepalive_interval_policy(Some(1)),
            Duration::from_millis(1_000)
        );
        assert_eq!(
            telegram_typing_keepalive_interval_policy(Some(999_999)),
            Duration::from_millis(MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS)
        );
        assert_eq!(
            telegram_read_max_attempts_policy(None),
            DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS
        );
        assert_eq!(telegram_read_max_attempts_policy(Some(0)), 1);
        assert_eq!(
            telegram_read_max_attempts_policy(Some(999)),
            MAX_TELEGRAM_READ_MAX_ATTEMPTS
        );
        assert_eq!(
            telegram_read_retry_backoff_policy(None),
            Duration::from_millis(DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS)
        );
        assert_eq!(
            telegram_read_retry_backoff_policy(Some(999_999)),
            Duration::from_millis(MAX_TELEGRAM_READ_RETRY_BACKOFF_MS)
        );
        assert_eq!(telegram_send_min_interval_policy(None), Duration::ZERO);
        assert_eq!(
            telegram_send_min_interval_policy(Some(999_999)),
            Duration::from_millis(MAX_TELEGRAM_SEND_MIN_INTERVAL_MS)
        );
        assert_eq!(
            telegram_send_max_attempts_policy(None),
            DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS
        );
        assert_eq!(telegram_send_max_attempts_policy(Some(0)), 1);
        assert_eq!(
            telegram_send_max_attempts_policy(Some(999)),
            MAX_TELEGRAM_SEND_MAX_ATTEMPTS
        );
        assert_eq!(
            telegram_send_retry_backoff_policy(None),
            Duration::from_millis(DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS)
        );
        assert_eq!(
            telegram_send_retry_backoff_policy(Some(999_999)),
            Duration::from_millis(MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS)
        );
    }

    #[test]
    fn transport_plan_for_config_status_requires_enabled_token_and_binding() {
        let mut config = NativeTelegramConfigStatus::disabled();
        config.enabled = true;
        config.token_source = "env";
        config.token_shape_ok = true;
        config.binding_ready = true;

        let ready = telegram_transport_plan_for_config_status(&config);
        assert!(ready.bot_api_transport_plan_ready);
        assert!(!ready.external_network_performed_by_status);
        assert!(!ready.raw_token_exposed);

        config.binding_ready = false;
        assert!(!telegram_transport_plan_for_config_status(&config).bot_api_transport_plan_ready);
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
    fn bot_api_call_wrappers_reject_bad_tokens_before_network() {
        let timeout = Duration::from_millis(1);

        let get_updates = telegram_call_get_updates_once("not-a-token", 20, None, timeout)
            .expect_err("bad getUpdates token rejected");
        assert!(get_updates.contains("getUpdates requires a valid Bot API token"));

        let send_message =
            telegram_call_send_message("not-a-token", 6476198178, "hello", Some(11), timeout)
                .expect_err("bad sendMessage token rejected");
        assert!(send_message.contains("sendMessage requires a valid Bot API token"));

        let chat_action = telegram_call_send_chat_action("not-a-token", 6476198178, timeout)
            .expect_err("bad sendChatAction token rejected");
        assert!(chat_action.contains("sendChatAction requires a valid Bot API token"));
    }

    #[test]
    fn typing_keepalive_policy_stays_gated_and_token_checked() {
        let token = "123456789:abcdefghijklmnopqrstuvwxyz";

        assert!(telegram_typing_keepalive_should_start(
            true, token, 6476198178
        ));
        assert!(!telegram_typing_keepalive_should_start(
            false, token, 6476198178
        ));
        assert!(!telegram_typing_keepalive_should_start(
            true,
            "not-a-token",
            6476198178
        ));
        assert!(!telegram_typing_keepalive_should_start(true, token, 0));

        let keepalive = telegram_start_typing_keepalive(
            false,
            token,
            6476198178,
            Duration::from_millis(1),
            |_token, _chat_id| panic!("disabled keepalive must not call Bot API"),
        );
        assert!(keepalive.is_none());
    }

    #[test]
    fn send_rate_limit_sleep_policy_is_bounded_by_min_interval() {
        assert_eq!(
            telegram_send_rate_limit_sleep_for(None, Duration::from_millis(750)),
            Duration::default()
        );
        assert_eq!(
            telegram_send_rate_limit_sleep_for(
                Some(Duration::from_millis(250)),
                Duration::from_millis(750)
            ),
            Duration::from_millis(500)
        );
        assert_eq!(
            telegram_send_rate_limit_sleep_for(
                Some(Duration::from_millis(900)),
                Duration::from_millis(750)
            ),
            Duration::default()
        );
        assert_eq!(
            telegram_send_rate_limit_sleep_for(Some(Duration::ZERO), Duration::ZERO),
            Duration::default()
        );
    }

    #[test]
    fn send_execution_commits_cursor_only_after_ack() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let delivery_ledger_path = temp.path().join("delivery-ledger.jsonl");
        let reply_target = NativeTelegramReplyTargetMaterial {
            chat_id: 6476198178,
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
                send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
                send_gate_enabled: true,
                cursor_path: &cursor_path,
                delivery_ledger_path: &delivery_ledger_path,
                send_max_attempts: 1,
                send_retry_backoff: Duration::ZERO,
            },
            |observed_token, chat_id, text, reply_to_message_id| {
                assert_eq!(observed_token, token);
                assert_eq!(chat_id, 6476198178);
                assert_eq!(text, "private model response text");
                assert_eq!(reply_to_message_id, Some(11));
                Ok(serde_json::json!({
                    "ok": true,
                    "result": { "message_id": 99 }
                }))
            },
        );

        assert_eq!(report.status, "delivered");
        assert_eq!(report.delivery_ledger_written_count, 2);
        assert!(report.cursor_written);
        assert_eq!(
            telegram_cursor_status_from_path(&cursor_path).next_update_offset,
            Some(50)
        );
        let delivery =
            telegram_delivery_ledger_status_from_path(&delivery_ledger_path, "/store/delivery");
        assert_eq!(delivery.acked_count, 1);
        assert!(delivery.durable_delivery_evidence_present);
        let serialized = serde_json::to_string(&report).expect("serialize");
        assert!(!serialized.contains("private model response text"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains(token));
    }

    #[test]
    fn send_execution_requires_gate_before_network_write() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let delivery_ledger_path = temp.path().join("delivery-ledger.jsonl");
        let reply_target = NativeTelegramReplyTargetMaterial {
            chat_id: 6476198178,
            reply_to_message_id: Some(11),
            raw_identifiers_exposed: false,
        };

        let report = execute_telegram_send_after_model_output(
            NativeTelegramSendExecutionInput {
                token: Some("123456:ABCDEFGHIJKLMNOPQRSTUVWX"),
                model_output: Some("private model response text"),
                reply_target: Some(&reply_target),
                candidate_next_update_offset: Some(50),
                send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
                send_gate_enabled: false,
                cursor_path: &cursor_path,
                delivery_ledger_path: &delivery_ledger_path,
                send_max_attempts: 1,
                send_retry_backoff: Duration::ZERO,
            },
            |_, _, _, _| panic!("sendMessage must not run while gated"),
        );

        assert_eq!(report.status, "gated");
        assert!(!report.send_attempted);
        assert!(!report.delivery_ledger_write_attempted);
        assert!(!report.cursor_written);
        assert!(!cursor_path.exists());
        assert!(!delivery_ledger_path.exists());
    }

    #[test]
    fn send_execution_keeps_cursor_uncommitted_on_send_failure() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let delivery_ledger_path = temp.path().join("delivery-ledger.jsonl");
        let reply_target = NativeTelegramReplyTargetMaterial {
            chat_id: 6476198178,
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
                    send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
                    send_gate_enabled: true,
                    cursor_path: &cursor_path,
                    delivery_ledger_path: &delivery_ledger_path,
                    send_max_attempts: 1,
                    send_retry_backoff: Duration::ZERO,
                },
                |_, _, _, _| {
                    Err("Telegram Bot API sendMessage HTTP status 500; description=temporary outage"
                    .to_string())
                },
            );

        assert_eq!(report.status, "attention");
        assert!(report.send_attempted);
        assert_eq!(report.bot_api_ack, None);
        assert_eq!(report.delivery_ledger_written_count, 2);
        assert_eq!(
            report.latest_delivery_ledger_stage.as_deref(),
            Some("failed")
        );
        assert!(!report.cursor_commit_attempted);
        assert!(!report.cursor_written);
        assert!(report.external_network_write);
        assert!(!report.external_send);
        assert!(report.error.unwrap().contains("temporary outage"));
        assert!(!cursor_path.exists());
        let delivery =
            telegram_delivery_ledger_status_from_path(&delivery_ledger_path, "/store/delivery");
        assert_eq!(delivery.status, "empty");
        assert_eq!(delivery.failed_count, 1);
        assert!(!delivery.durable_delivery_evidence_present);
    }

    #[test]
    fn token_shape_and_redaction_stay_local_and_bounded() {
        assert!(telegram_bot_token_shape_ok(
            "123456789:abcdefghijklmnopqrstuvwxyz"
        ));
        assert!(!telegram_bot_token_shape_ok("not-a-token"));
        assert_eq!(
            telegram_redact_token_like_text(
                "failed 123456789:abcdefghijklmnopqrstuvwxyz while sending"
            ),
            "failed [redacted-telegram-token] while sending"
        );
    }

    #[test]
    fn get_updates_conflict_error_is_busy_not_attention() {
        let conflict = "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request; make sure that only one bot instance is running";
        assert!(telegram_get_updates_error_is_conflict(conflict));

        let auth_error = "Telegram Bot API getUpdates HTTP status 401; description=Unauthorized";
        assert!(!telegram_get_updates_error_is_conflict(auth_error));
    }

    #[test]
    fn send_transient_error_classifier_keeps_auth_failures_terminal() {
        assert!(telegram_send_error_is_transient(
            "Telegram Bot API sendMessage HTTP status 429; description=Too Many Requests"
        ));
        assert!(telegram_send_error_is_transient(
            "Telegram Bot API sendMessage HTTP status 503; description=temporary outage"
        ));
        assert!(!telegram_send_error_is_transient(
            "Telegram Bot API sendMessage HTTP status 401; description=Unauthorized"
        ));
    }

    #[test]
    fn get_updates_transient_error_classifier_keeps_conflicts_busy() {
        assert!(telegram_get_updates_error_is_transient(
            "Telegram Bot API getUpdates request failed: error sending request"
        ));
        assert!(telegram_get_updates_error_is_transient(
            "Telegram Bot API getUpdates HTTP status 503; description=temporary outage"
        ));
        assert!(!telegram_get_updates_error_is_transient(
            "Telegram Bot API getUpdates HTTP status 401; description=Unauthorized"
        ));
        assert!(telegram_get_updates_error_is_conflict(
            "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request"
        ));
    }

    #[test]
    fn get_updates_retry_decision_keeps_conflicts_terminal() {
        let transient = "Telegram Bot API getUpdates request failed: connection reset";
        let conflict = "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request";

        assert!(telegram_get_updates_should_retry(1, 2, transient));
        assert!(!telegram_get_updates_should_retry(2, 2, transient));
        assert!(!telegram_get_updates_should_retry(1, 2, conflict));
        assert!(!telegram_get_updates_should_retry(
            1,
            2,
            "Telegram Bot API getUpdates HTTP status 401; description=Unauthorized"
        ));
    }

    #[test]
    fn send_retry_decision_only_retries_transient_errors_before_last_attempt() {
        assert!(telegram_send_should_retry(
            1,
            2,
            "Telegram Bot API sendMessage HTTP status 429; description=Too Many Requests"
        ));
        assert!(!telegram_send_should_retry(
            2,
            2,
            "Telegram Bot API sendMessage HTTP status 429; description=Too Many Requests"
        ));
        assert!(!telegram_send_should_retry(
            1,
            2,
            "Telegram Bot API sendMessage HTTP status 401; description=Unauthorized"
        ));
    }

    #[test]
    fn get_updates_retry_loop_retries_transient_and_redacts_terminal_error() {
        let mut attempts = 0_u64;
        let api = telegram_get_updates_with_retry(2, Duration::from_millis(0), || {
            attempts = attempts.saturating_add(1);
            if attempts == 1 {
                Err("Telegram Bot API getUpdates request failed 123456789:abcdefghijklmnopqrstuvwxyz"
                    .to_string())
            } else {
                Ok(serde_json::json!({"ok": true, "result": []}))
            }
        })
        .expect("transient retry succeeds");
        assert_eq!(attempts, 2);
        assert_eq!(api.get("ok").and_then(Value::as_bool), Some(true));

        let terminal = telegram_get_updates_with_retry(1, Duration::from_millis(0), || {
            Err(
                "Telegram Bot API getUpdates request failed 123456789:abcdefghijklmnopqrstuvwxyz"
                    .to_string(),
            )
        })
        .expect_err("terminal error redacted");
        assert!(terminal.contains("[redacted-telegram-token]"));
        assert!(!terminal.contains("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn get_updates_retry_loop_does_not_retry_conflict_errors() {
        let mut attempts = 0_u64;
        let conflict = telegram_get_updates_with_retry(3, Duration::from_millis(0), || {
            attempts = attempts.saturating_add(1);
            Err("Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request".to_string())
        })
        .expect_err("conflict is terminal busy signal");

        assert_eq!(attempts, 1);
        assert!(conflict.contains("HTTP status 409"));
    }
}
