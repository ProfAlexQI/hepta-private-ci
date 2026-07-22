use crate::HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES;
use crate::HeptaKernelTelegramConfigStatus;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaKernelTelegramTransportPlan {
    pub bot_api_transport_plan_ready: bool,
    pub endpoint_template: &'static str,
    pub get_updates_method: &'static str,
    pub send_message_method: &'static str,
    pub send_chat_action_method: &'static str,
    pub allowed_updates: &'static str,
    pub offset_commit_strategy: &'static str,
    pub send_delivery_gate: &'static str,
    pub typing_keepalive_plan: &'static str,
    pub raw_token_exposed: bool,
    pub external_network_performed_by_status: bool,
}

impl HeptaKernelTelegramTransportPlan {
    pub fn disabled() -> Self {
        Self {
            bot_api_transport_plan_ready: false,
            endpoint_template: "https://api.telegram.org/bot<redacted-token>/{method}",
            get_updates_method: "getUpdates",
            send_message_method: "sendMessage",
            send_chat_action_method: "sendChatAction",
            allowed_updates: HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES,
            offset_commit_strategy: "disabled",
            send_delivery_gate: "disabled",
            typing_keepalive_plan: "disabled",
            raw_token_exposed: false,
            external_network_performed_by_status: false,
        }
    }

    pub fn for_config_state(enabled: bool, token_shape_ok: bool, binding_ready: bool) -> Self {
        let ready = enabled && token_shape_ok && binding_ready;
        Self {
            bot_api_transport_plan_ready: ready,
            endpoint_template: "https://api.telegram.org/bot<redacted-token>/{method}",
            get_updates_method: "getUpdates",
            send_message_method: "sendMessage",
            send_chat_action_method: "sendChatAction",
            allowed_updates: HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES,
            offset_commit_strategy: "commit getUpdates offset only after delivery succeeds or duplicate suppression is recorded",
            send_delivery_gate: "sendMessage requires a successful model-turn or command dispatch plus explicit confirm-send runtime gate",
            typing_keepalive_plan: "sendChatAction typing keepalive is planned while the model turn is running, with bounded TTL",
            raw_token_exposed: false,
            external_network_performed_by_status: false,
        }
    }
}

pub fn hepta_kernel_telegram_transport_plan_for_config_status(
    config: &HeptaKernelTelegramConfigStatus,
) -> HeptaKernelTelegramTransportPlan {
    let config_ready = config.config_ready();
    HeptaKernelTelegramTransportPlan::for_config_state(config_ready, config_ready, config_ready)
}

pub fn hepta_kernel_telegram_get_updates_query(
    limit: usize,
    offset: Option<i64>,
) -> Vec<(&'static str, String)> {
    let mut query = vec![
        ("timeout", "0".to_string()),
        ("limit", limit.clamp(1, 20).to_string()),
        (
            "allowed_updates",
            HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES.to_string(),
        ),
    ];
    if let Some(offset) = offset.filter(|offset| *offset >= 0) {
        query.push(("offset", offset.to_string()));
    }
    query
}

pub fn hepta_kernel_telegram_send_chat_action_request_body(chat_id: i64) -> Result<Value, String> {
    if chat_id == 0 {
        return Err("Telegram sendChatAction chat id must be non-zero".to_string());
    }
    Ok(json!({
        "chat_id": chat_id,
        "action": "typing",
    }))
}

pub fn hepta_kernel_telegram_send_message_request_body(
    message_text: &str,
    chat_id: i64,
    reply_to_message_id: Option<i64>,
) -> Result<Value, String> {
    let text = message_text.trim();
    if text.is_empty() {
        return Err("Telegram sendMessage text must be non-empty".to_string());
    }
    let mut body = json!({
        "chat_id": chat_id,
        "text": text,
        "disable_web_page_preview": true,
    });
    if let Some(message_id) = reply_to_message_id {
        if message_id <= 0 {
            return Err("Telegram reply message id must be positive".to_string());
        }
        body["reply_parameters"] = json!({
            "message_id": message_id,
            "allow_sending_without_reply": true,
        });
    }
    Ok(body)
}

pub fn hepta_kernel_telegram_bot_api_http_status_error(
    method: &str,
    status_code: u16,
    description: Option<&str>,
) -> String {
    let description = description
        .map(redact_hepta_kernel_telegram_token_like_text)
        .unwrap_or_else(|| "missing".to_string());
    format!("Telegram Bot API {method} HTTP status {status_code}; description={description}")
}

pub fn hepta_kernel_telegram_bot_api_request_failed_error(method: &str, error: &str) -> String {
    let error = redact_hepta_kernel_telegram_token_like_text(error);
    format!("Telegram Bot API {method} request failed: {error}")
}

pub fn hepta_kernel_telegram_bot_api_client_build_error(method: &str, error: &str) -> String {
    let error = redact_hepta_kernel_telegram_token_like_text(error);
    format!("failed to build Telegram Bot API {method} client: {error}")
}

pub fn hepta_kernel_telegram_bot_api_json_parse_error(method: &str, error: &str) -> String {
    let error = redact_hepta_kernel_telegram_token_like_text(error);
    format!("failed to parse Telegram Bot API {method} response JSON: {error}")
}

#[derive(Debug, Clone, Copy)]
pub struct HeptaKernelTelegramSendProviderResultInput<'a> {
    pub attempt: u64,
    pub max_attempts: u64,
    pub api_result: Result<&'a Value, &'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelTelegramSendProviderResultPlan {
    pub bot_api_ack: Option<bool>,
    pub provider_message_id_present: bool,
    pub external_send: bool,
    pub should_retry: bool,
    pub delivery_ledger_stage: Option<&'static str>,
    pub report_status: &'static str,
    pub error: Option<String>,
    pub raw_response_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_message_id_exposed: bool,
    pub raw_token_exposed: bool,
}

pub fn plan_hepta_kernel_telegram_send_provider_result(
    input: HeptaKernelTelegramSendProviderResultInput<'_>,
) -> HeptaKernelTelegramSendProviderResultPlan {
    match input.api_result {
        Ok(api) => {
            let ok = api.get("ok").and_then(Value::as_bool).unwrap_or(false);
            let provider_message_id_present = api
                .pointer("/result/message_id")
                .and_then(Value::as_i64)
                .is_some();
            if ok {
                return HeptaKernelTelegramSendProviderResultPlan {
                    bot_api_ack: Some(true),
                    provider_message_id_present,
                    external_send: true,
                    should_retry: false,
                    delivery_ledger_stage: Some("acked"),
                    report_status: "provider_acked",
                    error: None,
                    raw_response_text_exposed: false,
                    raw_chat_id_exposed: false,
                    raw_message_id_exposed: false,
                    raw_token_exposed: false,
                };
            }

            let error = api
                .get("description")
                .and_then(Value::as_str)
                .map(redact_hepta_kernel_telegram_token_like_text)
                .unwrap_or_else(|| "Telegram Bot API sendMessage returned ok=false".to_string());
            let should_retry =
                hepta_kernel_telegram_send_should_retry(input.attempt, input.max_attempts, &error);
            HeptaKernelTelegramSendProviderResultPlan {
                bot_api_ack: Some(false),
                provider_message_id_present,
                external_send: false,
                should_retry,
                delivery_ledger_stage: (!should_retry).then_some("failed"),
                report_status: if should_retry { "sending" } else { "attention" },
                error: Some(error),
                raw_response_text_exposed: false,
                raw_chat_id_exposed: false,
                raw_message_id_exposed: false,
                raw_token_exposed: false,
            }
        }
        Err(error) => {
            let error = redact_hepta_kernel_telegram_token_like_text(error);
            let should_retry =
                hepta_kernel_telegram_send_should_retry(input.attempt, input.max_attempts, &error);
            HeptaKernelTelegramSendProviderResultPlan {
                bot_api_ack: None,
                provider_message_id_present: false,
                external_send: false,
                should_retry,
                delivery_ledger_stage: (!should_retry).then_some("failed"),
                report_status: if should_retry { "sending" } else { "attention" },
                error: Some(error),
                raw_response_text_exposed: false,
                raw_chat_id_exposed: false,
                raw_message_id_exposed: false,
                raw_token_exposed: false,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HeptaKernelTelegramGetUpdatesProviderResultInput<'a> {
    pub attempt: u64,
    pub max_attempts: u64,
    pub api_result: Result<&'a Value, &'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelTelegramGetUpdatesProviderResultPlan {
    pub bot_api_ok: Option<bool>,
    pub external_read: bool,
    pub should_retry: bool,
    pub report_status: &'static str,
    pub error: Option<String>,
    pub raw_response_text_exposed: bool,
    pub raw_token_exposed: bool,
}

pub fn plan_hepta_kernel_telegram_get_updates_provider_result(
    input: HeptaKernelTelegramGetUpdatesProviderResultInput<'_>,
) -> HeptaKernelTelegramGetUpdatesProviderResultPlan {
    match input.api_result {
        Ok(api) => HeptaKernelTelegramGetUpdatesProviderResultPlan {
            bot_api_ok: api.get("ok").and_then(Value::as_bool),
            external_read: true,
            should_retry: false,
            report_status: "provider_returned",
            error: None,
            raw_response_text_exposed: false,
            raw_token_exposed: false,
        },
        Err(error) => {
            let error = redact_hepta_kernel_telegram_token_like_text(error);
            let should_retry = hepta_kernel_telegram_get_updates_should_retry(
                input.attempt,
                input.max_attempts,
                &error,
            );
            let report_status = if should_retry {
                "reading"
            } else if hepta_kernel_telegram_get_updates_error_is_conflict(&error) {
                "busy"
            } else {
                "attention"
            };
            HeptaKernelTelegramGetUpdatesProviderResultPlan {
                bot_api_ok: None,
                external_read: false,
                should_retry,
                report_status,
                error: Some(error),
                raw_response_text_exposed: false,
                raw_token_exposed: false,
            }
        }
    }
}

pub fn hepta_kernel_telegram_typing_keepalive_should_start(
    enabled: bool,
    token: &str,
    chat_id: i64,
) -> bool {
    enabled && hepta_kernel_telegram_bot_token_shape_ok(token) && chat_id != 0
}

pub fn hepta_kernel_telegram_send_rate_limit_sleep_for(
    last_elapsed: Option<Duration>,
    min_interval: Duration,
) -> Duration {
    if min_interval.is_zero() {
        return Duration::default();
    }
    last_elapsed
        .and_then(|elapsed| min_interval.checked_sub(elapsed))
        .unwrap_or_default()
}

pub fn hepta_kernel_telegram_bot_token_shape_ok(token: &str) -> bool {
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

pub fn redact_hepta_kernel_telegram_token_like_text(text: &str) -> String {
    text.split_whitespace()
        .map(|part| {
            let candidate = part.trim_matches(|ch: char| {
                !ch.is_ascii_alphanumeric() && ch != ':' && ch != '_' && ch != '-' && ch != '='
            });
            let token_like = hepta_kernel_telegram_bot_token_shape_ok(candidate)
                || candidate
                    .rsplit_once('=')
                    .is_some_and(|(_, value)| hepta_kernel_telegram_bot_token_shape_ok(value));
            if token_like {
                "[redacted-telegram-token]".to_string()
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn hepta_kernel_telegram_get_updates_error_is_conflict(error: &str) -> bool {
    error.contains("Telegram Bot API getUpdates HTTP status 409")
        && error.contains("terminated by other getUpdates request")
}

pub fn hepta_kernel_telegram_error_is_transient(error: &str) -> bool {
    error.contains("request failed")
        || error.contains("HTTP status 429")
        || error.contains("HTTP status 500")
        || error.contains("HTTP status 502")
        || error.contains("HTTP status 503")
        || error.contains("HTTP status 504")
        || error.contains("Too Many Requests")
}

pub fn hepta_kernel_telegram_send_error_is_transient(error: &str) -> bool {
    hepta_kernel_telegram_error_is_transient(error)
}

pub fn hepta_kernel_telegram_get_updates_error_is_transient(error: &str) -> bool {
    hepta_kernel_telegram_error_is_transient(error)
}

pub fn hepta_kernel_telegram_get_updates_should_retry(
    attempt: u64,
    max_attempts: u64,
    error: &str,
) -> bool {
    attempt < max_attempts
        && hepta_kernel_telegram_get_updates_error_is_transient(error)
        && !hepta_kernel_telegram_get_updates_error_is_conflict(error)
}

pub fn hepta_kernel_telegram_send_should_retry(
    attempt: u64,
    max_attempts: u64,
    error: &str,
) -> bool {
    attempt < max_attempts && hepta_kernel_telegram_send_error_is_transient(error)
}

#[cfg(test)]
#[path = "telegram_transport_tests.rs"]
mod tests;
