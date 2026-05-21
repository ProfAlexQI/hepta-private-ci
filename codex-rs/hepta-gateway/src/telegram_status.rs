use serde::Serialize;
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::telegram_config::NativeTelegramConfigStatus;
use crate::telegram_cursor::{NativeTelegramCursorPlan, NativeTelegramCursorStatus};
use crate::telegram_delivery::NativeTelegramDeliveryLedgerStatus;
use crate::telegram_policy::{
    NativeTelegramExecutionPlan, NativeTelegramGatewayGateSummary, NativeTelegramIngressInspection,
    NativeTelegramModelExecutionReport, NativeTelegramModelInvocationRequestPlan,
    NativeTelegramModelTurnPlan, NativeTelegramSendExecutionReport, NativeTelegramSendRequestPlan,
    build_model_invocation_request_plan, plan_model_turn_for_updates,
    telegram_drain_execution_plan,
};
use crate::telegram_runtime::NativeTelegramSessionBridgePlan;
use crate::telegram_transport::{
    NativeTelegramSendPlan, NativeTelegramTransportPlan, telegram_get_updates_error_is_conflict,
    telegram_read_max_attempts_policy, telegram_read_retry_backoff_policy,
    telegram_redact_token_like_text, telegram_send_max_attempts_policy,
    telegram_send_min_interval_policy, telegram_send_retry_backoff_policy,
    telegram_typing_keepalive_interval_policy,
};
use hepta_runtime::{NativeTelegramModelRunnerPlan, native_telegram_model_timeout};

pub const DEFAULT_TELEGRAM_SOAK_MIN_POLLS: u64 = 3;
pub const MAX_TELEGRAM_SOAK_MIN_POLLS: u64 = 10_000;
pub const DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION: u64 = 0;
pub const MAX_TELEGRAM_SOAK_MAX_ATTENTION: u64 = 1_000;
pub const DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS: u64 = 120_000;
pub const MAX_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS: u64 = 3_600_000;
pub const MIN_TELEGRAM_POLL_LOOP_INTERVAL_MS: u64 = 500;
pub const MAX_TELEGRAM_POLL_LOOP_INTERVAL_MS: u64 = 60_000;

pub fn telegram_poll_loop_should_spawn(
    requested: bool,
    poll_loop_gate_enabled: bool,
    delivery_approval_gate_enabled: bool,
) -> bool {
    requested && poll_loop_gate_enabled && delivery_approval_gate_enabled
}

pub fn telegram_poll_loop_interval_ms_policy(value: u64) -> u64 {
    value.clamp(
        MIN_TELEGRAM_POLL_LOOP_INTERVAL_MS,
        MAX_TELEGRAM_POLL_LOOP_INTERVAL_MS,
    )
}

pub fn telegram_receive_limit_policy(value: usize) -> usize {
    value.clamp(1, 20)
}

pub fn telegram_soak_min_poll_iterations_policy(value: Option<u64>) -> u64 {
    value
        .map(|polls| polls.clamp(1, MAX_TELEGRAM_SOAK_MIN_POLLS))
        .unwrap_or(DEFAULT_TELEGRAM_SOAK_MIN_POLLS)
}

pub fn telegram_soak_max_attention_count_policy(value: Option<u64>) -> u64 {
    value
        .map(|count| count.min(MAX_TELEGRAM_SOAK_MAX_ATTENTION))
        .unwrap_or(DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION)
}

pub fn telegram_soak_max_observed_age_ms_policy(value: Option<u64>) -> u64 {
    value
        .map(|age_ms| age_ms.clamp(1_000, MAX_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS))
        .unwrap_or(DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS)
}

fn duration_millis_u64(duration: Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}

pub fn telegram_system_time_unix_ms(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(duration_millis_u64)
        .unwrap_or(0)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramPluginStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub in_process_supervisor_ready: bool,
    pub in_process_reply_loop_ready: bool,
    pub model_turn_bridge_ready: bool,
    pub bot_api_poll_ready: bool,
    pub bot_api_send_ready: bool,
    pub openclaw_gateway_runtime_dependency: bool,
    pub external_network_read: bool,
    pub external_send: bool,
    pub poll_ms: u64,
    pub allowed_updates: &'static str,
    pub config: NativeTelegramConfigStatus,
    pub transport_plan: NativeTelegramTransportPlan,
    pub ingress_parser: NativeTelegramIngressInspection,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub model_turn_plan: NativeTelegramModelTurnPlan,
    pub migration_blocker: Option<&'static str>,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramReceiveOnceStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub live_read_gate_env: &'static str,
    pub live_read_gate_enabled: bool,
    pub external_network_read: bool,
    pub external_send: bool,
    pub model_turn_started: bool,
    pub cursor_written: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_token_exposed: bool,
    pub limit: usize,
    pub get_updates_offset: Option<i64>,
    pub bot_api_ok: Option<bool>,
    pub local_next_update_offset: Option<i64>,
    pub config: NativeTelegramConfigStatus,
    pub transport_plan: NativeTelegramTransportPlan,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub inspection: NativeTelegramIngressInspection,
    pub model_turn_plan: NativeTelegramModelTurnPlan,
    pub error: Option<String>,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramReceiveOnceStatusInput {
    pub requested: bool,
    pub status: &'static str,
    pub live_read_gate_env: &'static str,
    pub live_read_gate_enabled: bool,
    pub external_network_read: bool,
    pub limit: usize,
    pub config: NativeTelegramConfigStatus,
    pub transport_plan: NativeTelegramTransportPlan,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub inspection: NativeTelegramIngressInspection,
    pub model_turn_plan: Option<NativeTelegramModelTurnPlan>,
    pub get_updates_offset: Option<i64>,
    pub bot_api_ok: Option<bool>,
    pub local_next_update_offset: Option<i64>,
    pub error: Option<String>,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramReceiveOncePreflightInput<'a> {
    pub requested: bool,
    pub live_read_gate_env: &'static str,
    pub live_read_gate_enabled: bool,
    pub limit: usize,
    pub config: &'a NativeTelegramConfigStatus,
    pub transport_plan: &'a NativeTelegramTransportPlan,
    pub cursor_plan: &'a NativeTelegramCursorPlan,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramReceiveOnceApiResultInput<'a> {
    pub requested: bool,
    pub live_read_gate_env: &'static str,
    pub live_read_gate_enabled: bool,
    pub external_network_read: bool,
    pub limit: usize,
    pub config: NativeTelegramConfigStatus,
    pub transport_plan: NativeTelegramTransportPlan,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub get_updates_offset: Option<i64>,
    pub api_result: Result<&'a Value, &'a str>,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramReceiveOnceErrorInput {
    pub requested: bool,
    pub live_read_gate_env: &'static str,
    pub live_read_gate_enabled: bool,
    pub limit: usize,
    pub config: NativeTelegramConfigStatus,
    pub transport_plan: NativeTelegramTransportPlan,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub get_updates_offset: Option<i64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramReceiveOnceShellReadinessInput<'a> {
    pub token_error: Option<&'a str>,
    pub cursor_file_present: bool,
    pub cursor_parse_ok: bool,
    pub cursor_error: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeTelegramReceiveOnceShellReadinessPlan {
    pub status: &'static str,
    pub error: Option<String>,
    pub may_call_bot_api: bool,
}

impl NativeTelegramReceiveOnceStatus {
    #[allow(clippy::too_many_arguments)]
    pub fn base(
        requested: bool,
        status: &'static str,
        live_read_gate_env: &'static str,
        live_read_gate_enabled: bool,
        external_network_read: bool,
        limit: usize,
        config: NativeTelegramConfigStatus,
        transport_plan: NativeTelegramTransportPlan,
        cursor_plan: NativeTelegramCursorPlan,
        inspection: NativeTelegramIngressInspection,
        error: Option<String>,
        next_migration_slice: &'static str,
    ) -> Self {
        build_telegram_receive_once_status(NativeTelegramReceiveOnceStatusInput {
            requested,
            status,
            live_read_gate_env,
            live_read_gate_enabled,
            external_network_read,
            limit,
            config,
            transport_plan,
            cursor_plan,
            inspection,
            model_turn_plan: None,
            get_updates_offset: None,
            bot_api_ok: None,
            local_next_update_offset: None,
            error,
            next_migration_slice,
        })
    }
}

pub fn build_telegram_receive_once_error_status(
    input: NativeTelegramReceiveOnceErrorInput,
) -> NativeTelegramReceiveOnceStatus {
    build_telegram_receive_once_status(NativeTelegramReceiveOnceStatusInput {
        requested: input.requested,
        status: "attention",
        live_read_gate_env: input.live_read_gate_env,
        live_read_gate_enabled: input.live_read_gate_enabled,
        external_network_read: false,
        limit: input.limit,
        config: input.config,
        transport_plan: input.transport_plan,
        cursor_plan: input.cursor_plan,
        inspection: crate::telegram_policy::inspect_telegram_updates(&[]),
        model_turn_plan: None,
        get_updates_offset: input.get_updates_offset,
        bot_api_ok: None,
        local_next_update_offset: None,
        error: input
            .error
            .map(|error| telegram_redact_token_like_text(&error)),
        next_migration_slice: TELEGRAM_RECEIVE_ONCE_NEXT_MIGRATION_SLICE,
    })
}

pub fn build_telegram_receive_once_status(
    input: NativeTelegramReceiveOnceStatusInput,
) -> NativeTelegramReceiveOnceStatus {
    let local_next_update_offset = input
        .local_next_update_offset
        .or(input.inspection.latest_allowed_next_update_offset);
    let model_turn_plan = input.model_turn_plan.unwrap_or_else(|| {
        if input.requested {
            plan_model_turn_for_updates(&[])
        } else {
            NativeTelegramModelTurnPlan::disabled()
        }
    });

    NativeTelegramReceiveOnceStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: input.requested,
        status: input.status,
        live_read_gate_env: input.live_read_gate_env,
        live_read_gate_enabled: input.live_read_gate_enabled,
        external_network_read: input.external_network_read,
        external_send: false,
        model_turn_started: false,
        cursor_written: false,
        raw_update_payload_exposed: false,
        raw_token_exposed: false,
        limit: input.limit,
        get_updates_offset: input.get_updates_offset,
        bot_api_ok: input.bot_api_ok,
        local_next_update_offset,
        config: input.config,
        transport_plan: input.transport_plan,
        cursor_plan: input.cursor_plan,
        inspection: input.inspection,
        model_turn_plan,
        error: input.error,
        next_migration_slice: input.next_migration_slice,
    }
}

pub fn plan_telegram_receive_once_preflight_status(
    input: NativeTelegramReceiveOncePreflightInput<'_>,
) -> Option<NativeTelegramReceiveOnceStatus> {
    let inspection = crate::telegram_policy::inspect_telegram_updates(&[]);
    if !input.requested {
        return Some(build_telegram_receive_once_status(
            NativeTelegramReceiveOnceStatusInput {
                requested: false,
                status: "disabled",
                live_read_gate_env: input.live_read_gate_env,
                live_read_gate_enabled: input.live_read_gate_enabled,
                external_network_read: false,
                limit: input.limit,
                config: input.config.clone(),
                transport_plan: input.transport_plan.clone(),
                cursor_plan: input.cursor_plan.clone(),
                inspection,
                model_turn_plan: None,
                get_updates_offset: None,
                bot_api_ok: None,
                local_next_update_offset: None,
                error: None,
                next_migration_slice: TELEGRAM_RECEIVE_ONCE_NEXT_MIGRATION_SLICE,
            },
        ));
    }

    if !input.live_read_gate_enabled {
        return Some(build_telegram_receive_once_status(
            NativeTelegramReceiveOnceStatusInput {
                requested: true,
                status: "gated",
                live_read_gate_env: input.live_read_gate_env,
                live_read_gate_enabled: false,
                external_network_read: false,
                limit: input.limit,
                config: input.config.clone(),
                transport_plan: input.transport_plan.clone(),
                cursor_plan: input.cursor_plan.clone(),
                inspection,
                model_turn_plan: None,
                get_updates_offset: None,
                bot_api_ok: None,
                local_next_update_offset: None,
                error: Some(format!(
                    "live Telegram receive is gated; set {}=1 to run one redacted getUpdates read",
                    input.live_read_gate_env
                )),
                next_migration_slice: TELEGRAM_RECEIVE_ONCE_NEXT_MIGRATION_SLICE,
            },
        ));
    }

    if !input.config.config_ready() {
        return Some(build_telegram_receive_once_status(
            NativeTelegramReceiveOnceStatusInput {
                requested: true,
                status: "attention",
                live_read_gate_env: input.live_read_gate_env,
                live_read_gate_enabled: true,
                external_network_read: false,
                limit: input.limit,
                config: input.config.clone(),
                transport_plan: input.transport_plan.clone(),
                cursor_plan: input.cursor_plan.clone(),
                inspection,
                model_turn_plan: None,
                get_updates_offset: None,
                bot_api_ok: None,
                local_next_update_offset: None,
                error: Some("Telegram config, token shape, or binding is not ready".to_string()),
                next_migration_slice: TELEGRAM_RECEIVE_ONCE_NEXT_MIGRATION_SLICE,
            },
        ));
    }

    None
}

pub fn plan_telegram_receive_once_shell_readiness(
    input: NativeTelegramReceiveOnceShellReadinessInput<'_>,
) -> NativeTelegramReceiveOnceShellReadinessPlan {
    if let Some(token_error) = input.token_error {
        return NativeTelegramReceiveOnceShellReadinessPlan {
            status: "attention",
            error: Some(telegram_redact_token_like_text(token_error)),
            may_call_bot_api: false,
        };
    }

    if input.cursor_file_present && !input.cursor_parse_ok {
        return NativeTelegramReceiveOnceShellReadinessPlan {
            status: "attention",
            error: Some(
                input
                    .cursor_error
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| "Telegram cursor state is not readable".to_string()),
            ),
            may_call_bot_api: false,
        };
    }

    NativeTelegramReceiveOnceShellReadinessPlan {
        status: "planned",
        error: None,
        may_call_bot_api: true,
    }
}

pub fn build_telegram_receive_once_status_from_api_result(
    input: NativeTelegramReceiveOnceApiResultInput<'_>,
) -> NativeTelegramReceiveOnceStatus {
    match input.api_result {
        Ok(api) => {
            let bot_api_ok = api.get("ok").and_then(Value::as_bool);
            let updates = api
                .get("result")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            let inspection = crate::telegram_policy::inspect_telegram_updates(&updates);
            let local_next_update_offset = inspection.latest_allowed_next_update_offset;
            let model_turn_plan = plan_model_turn_for_updates(&updates);
            let status = if bot_api_ok.unwrap_or(false) {
                "ready"
            } else {
                "attention"
            };
            let error = if bot_api_ok == Some(false) {
                api.get("description")
                    .and_then(Value::as_str)
                    .map(telegram_redact_token_like_text)
                    .or_else(|| Some("Telegram Bot API getUpdates returned ok=false".to_string()))
            } else {
                None
            };

            build_telegram_receive_once_status(NativeTelegramReceiveOnceStatusInput {
                requested: input.requested,
                status,
                live_read_gate_env: input.live_read_gate_env,
                live_read_gate_enabled: input.live_read_gate_enabled,
                external_network_read: input.external_network_read,
                limit: input.limit,
                config: input.config,
                transport_plan: input.transport_plan,
                cursor_plan: input.cursor_plan,
                inspection,
                model_turn_plan: Some(model_turn_plan),
                get_updates_offset: input.get_updates_offset,
                bot_api_ok,
                local_next_update_offset,
                error,
                next_migration_slice: TELEGRAM_RECEIVE_ONCE_NEXT_MIGRATION_SLICE,
            })
        }
        Err(error) => {
            let redacted_error = telegram_redact_token_like_text(error);
            let status = if telegram_get_updates_error_is_conflict(&redacted_error) {
                "busy"
            } else {
                "attention"
            };

            build_telegram_receive_once_status(NativeTelegramReceiveOnceStatusInput {
                requested: input.requested,
                status,
                live_read_gate_env: input.live_read_gate_env,
                live_read_gate_enabled: input.live_read_gate_enabled,
                external_network_read: input.external_network_read,
                limit: input.limit,
                config: input.config,
                transport_plan: input.transport_plan,
                cursor_plan: input.cursor_plan,
                inspection: crate::telegram_policy::inspect_telegram_updates(&[]),
                model_turn_plan: None,
                get_updates_offset: input.get_updates_offset,
                bot_api_ok: None,
                local_next_update_offset: None,
                error: Some(redacted_error),
                next_migration_slice: TELEGRAM_RECEIVE_ONCE_NEXT_MIGRATION_SLICE,
            })
        }
    }
}

const TELEGRAM_RECEIVE_ONCE_NEXT_MIGRATION_SLICE: &str = "manual receive is a diagnostic read path; use drain-once or the armed poll loop for model, send, and cursor side effects";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramModelTurnPlanStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub model_turn_bridge_ready: bool,
    pub model_turn_started: bool,
    pub session_runner_invoked: bool,
    pub external_send: bool,
    pub cursor_written: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_sender_id_exposed: bool,
    pub raw_message_id_exposed: bool,
    pub config: NativeTelegramConfigStatus,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub inspection: NativeTelegramIngressInspection,
    pub model_turn_plan: NativeTelegramModelTurnPlan,
    pub error: Option<String>,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramModelBridgeStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub model_turn_gate_env: &'static str,
    pub model_turn_gate_enabled: bool,
    pub send_gate_env: &'static str,
    pub model_turn_bridge_ready: bool,
    pub model_turn_started: bool,
    pub session_runner_invoked: bool,
    pub local_process_spawned: bool,
    pub external_network_read: bool,
    pub external_send: bool,
    pub cursor_written: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_sender_id_exposed: bool,
    pub raw_message_id_exposed: bool,
    pub config: NativeTelegramConfigStatus,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub model_turn_plan: NativeTelegramModelTurnPlan,
    pub invocation_request: NativeTelegramModelInvocationRequestPlan,
    pub model_execution: NativeTelegramModelExecutionReport,
    pub bridge_plan: NativeTelegramSessionBridgePlan,
    pub error: Option<String>,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramSendPlanStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub send_gate_env: &'static str,
    pub send_gate_enabled: bool,
    pub bot_api_send_ready: bool,
    pub external_network_write: bool,
    pub external_send: bool,
    pub cursor_written: bool,
    pub raw_response_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_message_id_exposed: bool,
    pub raw_token_exposed: bool,
    pub config: NativeTelegramConfigStatus,
    pub transport_plan: NativeTelegramTransportPlan,
    pub send_plan: NativeTelegramSendPlan,
    pub send_request: NativeTelegramSendRequestPlan,
    pub error: Option<String>,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramDrainOnceStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub gates: NativeTelegramGatewayGateSummary,
    pub config: NativeTelegramConfigStatus,
    pub execution_plan: NativeTelegramExecutionPlan,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub inspection: NativeTelegramIngressInspection,
    pub model_turn_plan: NativeTelegramModelTurnPlan,
    pub invocation_request: NativeTelegramModelInvocationRequestPlan,
    pub model_execution: NativeTelegramModelExecutionReport,
    pub send_plan: NativeTelegramSendPlan,
    pub send_request: NativeTelegramSendRequestPlan,
    pub send_execution: NativeTelegramSendExecutionReport,
    pub bot_api_ok: Option<bool>,
    pub local_next_update_offset: Option<i64>,
    pub get_updates_offset: Option<i64>,
    pub live_read_started: bool,
    pub model_turn_started: bool,
    pub send_started: bool,
    pub cursor_written: bool,
    pub external_network_read: bool,
    pub external_network_write: bool,
    pub external_send: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_response_text_exposed: bool,
    pub raw_token_exposed: bool,
    pub error: Option<String>,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramPollLoopStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub poll_loop_gate_env: &'static str,
    pub poll_loop_gate_enabled: bool,
    pub delivery_approval_gate_env: &'static str,
    pub delivery_approval_gate_enabled: bool,
    pub poll_ms: u64,
    pub drain_once_endpoint: &'static str,
    pub worker_spawned_by_status: bool,
    pub loop_invokes_drain_once: bool,
    pub requires_live_read_gate: &'static str,
    pub requires_model_turn_gate: &'static str,
    pub requires_send_gate: &'static str,
    pub requires_delivery_approval_gate: &'static str,
    pub external_network_read_by_status: bool,
    pub external_send_by_status: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_response_text_exposed: bool,
    pub raw_token_exposed: bool,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramLiveSoakStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub side_effect_free: bool,
    pub endpoint: &'static str,
    pub poll_loop_status: NativeTelegramPollLoopStatus,
    pub cursor_status: NativeTelegramCursorStatus,
    pub delivery_ledger_status: NativeTelegramDeliveryLedgerStatus,
    pub production_guards: NativeTelegramProductionGuardStatus,
    pub production_readiness: NativeTelegramProductionReadinessStatus,
    pub observation: NativeTelegramLiveSoakObservationReport,
    pub health_ready: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_response_text_exposed: bool,
    pub raw_token_exposed: bool,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramProductionGuardStatus {
    pub read_max_attempts_env: &'static str,
    pub read_max_attempts: u64,
    pub read_retry_backoff_env: &'static str,
    pub read_retry_backoff_ms: u64,
    pub retry_transient_read_errors: bool,
    pub typing_keepalive_env: &'static str,
    pub typing_keepalive_enabled: bool,
    pub typing_keepalive_interval_ms: u64,
    pub model_timeout_env: &'static str,
    pub model_timeout_ms: u64,
    pub model_failure_fallback_env: &'static str,
    pub model_failure_fallback_enabled: bool,
    pub send_min_interval_env: &'static str,
    pub send_min_interval_ms: u64,
    pub send_max_attempts_env: &'static str,
    pub send_max_attempts: u64,
    pub send_retry_backoff_env: &'static str,
    pub send_retry_backoff_ms: u64,
    pub retry_transient_send_errors: bool,
    pub rate_limit_scope: &'static str,
    pub raw_token_exposed: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramProductionGuardStatusInput {
    pub read_max_attempts_env: &'static str,
    pub read_max_attempts: u64,
    pub read_retry_backoff_env: &'static str,
    pub read_retry_backoff_ms: u64,
    pub typing_keepalive_env: &'static str,
    pub typing_keepalive_enabled: bool,
    pub typing_keepalive_interval_ms: u64,
    pub model_timeout_env: &'static str,
    pub model_timeout_ms: u64,
    pub model_failure_fallback_env: &'static str,
    pub model_failure_fallback_enabled: bool,
    pub send_min_interval_env: &'static str,
    pub send_min_interval_ms: u64,
    pub send_max_attempts_env: &'static str,
    pub send_max_attempts: u64,
    pub send_retry_backoff_env: &'static str,
    pub send_retry_backoff_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramProductionGuardPolicyInput {
    pub read_max_attempts_env: &'static str,
    pub read_max_attempts: Option<u64>,
    pub read_retry_backoff_env: &'static str,
    pub read_retry_backoff_ms: Option<u64>,
    pub typing_keepalive_env: &'static str,
    pub typing_keepalive_enabled: bool,
    pub typing_keepalive_interval_ms: Option<u64>,
    pub model_timeout_env: &'static str,
    pub model_timeout_ms: Option<u64>,
    pub model_failure_fallback_env: &'static str,
    pub model_failure_fallback_enabled: bool,
    pub send_min_interval_env: &'static str,
    pub send_min_interval_ms: Option<u64>,
    pub send_max_attempts_env: &'static str,
    pub send_max_attempts: Option<u64>,
    pub send_retry_backoff_env: &'static str,
    pub send_retry_backoff_ms: Option<u64>,
}

pub fn build_telegram_production_guard_status(
    input: NativeTelegramProductionGuardStatusInput,
) -> NativeTelegramProductionGuardStatus {
    NativeTelegramProductionGuardStatus {
        read_max_attempts_env: input.read_max_attempts_env,
        read_max_attempts: input.read_max_attempts,
        read_retry_backoff_env: input.read_retry_backoff_env,
        read_retry_backoff_ms: input.read_retry_backoff_ms,
        retry_transient_read_errors: true,
        typing_keepalive_env: input.typing_keepalive_env,
        typing_keepalive_enabled: input.typing_keepalive_enabled,
        typing_keepalive_interval_ms: input.typing_keepalive_interval_ms,
        model_timeout_env: input.model_timeout_env,
        model_timeout_ms: input.model_timeout_ms,
        model_failure_fallback_env: input.model_failure_fallback_env,
        model_failure_fallback_enabled: input.model_failure_fallback_enabled,
        send_min_interval_env: input.send_min_interval_env,
        send_min_interval_ms: input.send_min_interval_ms,
        send_max_attempts_env: input.send_max_attempts_env,
        send_max_attempts: input.send_max_attempts,
        send_retry_backoff_env: input.send_retry_backoff_env,
        send_retry_backoff_ms: input.send_retry_backoff_ms,
        retry_transient_send_errors: true,
        rate_limit_scope: "in-process per chat id; reset on gateway restart",
        raw_token_exposed: false,
    }
}

pub fn build_telegram_production_guard_status_from_policy(
    input: NativeTelegramProductionGuardPolicyInput,
) -> NativeTelegramProductionGuardStatus {
    build_telegram_production_guard_status(NativeTelegramProductionGuardStatusInput {
        read_max_attempts_env: input.read_max_attempts_env,
        read_max_attempts: telegram_read_max_attempts_policy(input.read_max_attempts),
        read_retry_backoff_env: input.read_retry_backoff_env,
        read_retry_backoff_ms: duration_millis_u64(telegram_read_retry_backoff_policy(
            input.read_retry_backoff_ms,
        )),
        typing_keepalive_env: input.typing_keepalive_env,
        typing_keepalive_enabled: input.typing_keepalive_enabled,
        typing_keepalive_interval_ms: duration_millis_u64(
            telegram_typing_keepalive_interval_policy(input.typing_keepalive_interval_ms),
        ),
        model_timeout_env: input.model_timeout_env,
        model_timeout_ms: duration_millis_u64(native_telegram_model_timeout(
            input.model_timeout_ms,
        )),
        model_failure_fallback_env: input.model_failure_fallback_env,
        model_failure_fallback_enabled: input.model_failure_fallback_enabled,
        send_min_interval_env: input.send_min_interval_env,
        send_min_interval_ms: duration_millis_u64(telegram_send_min_interval_policy(
            input.send_min_interval_ms,
        )),
        send_max_attempts_env: input.send_max_attempts_env,
        send_max_attempts: telegram_send_max_attempts_policy(input.send_max_attempts),
        send_retry_backoff_env: input.send_retry_backoff_env,
        send_retry_backoff_ms: duration_millis_u64(telegram_send_retry_backoff_policy(
            input.send_retry_backoff_ms,
        )),
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramProductionReadinessStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub ready: bool,
    pub side_effect_free: bool,
    pub min_poll_iterations_env: &'static str,
    pub min_poll_iterations: u64,
    pub max_attention_count_env: &'static str,
    pub max_attention_count: u64,
    pub max_observed_age_env: &'static str,
    pub max_observed_age_ms: u64,
    pub poll_loop_armed: bool,
    pub cursor_ready: bool,
    pub production_guards_ready: bool,
    pub observation_ready: bool,
    pub observation_fresh: bool,
    pub durable_cursor_evidence_present: bool,
    pub durable_delivery_evidence_required: bool,
    pub durable_delivery_evidence_present: bool,
    pub durable_delivery_evidence_fresh: bool,
    pub delivery_ledger_ready: bool,
    pub attention_budget_ok: bool,
    pub recent_bot_api_ok: bool,
    pub redaction_guards_ok: bool,
    pub readiness_blockers: Vec<&'static str>,
    pub readiness_warnings: Vec<&'static str>,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_response_text_exposed: bool,
    pub raw_token_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramLiveSoakObservationReport {
    pub poll_iterations: u64,
    pub drained_count: u64,
    pub busy_count: u64,
    pub attention_count: u64,
    pub empty_read_count: u64,
    pub model_turn_started_count: u64,
    pub send_started_count: u64,
    pub cursor_written_count: u64,
    pub external_send_count: u64,
    pub last_drained_at_unix_ms: Option<u64>,
    pub last_drained_next_update_offset: Option<i64>,
    pub last_observed_at_unix_ms: Option<u64>,
    pub last_status: Option<String>,
    pub last_error: Option<String>,
    pub last_bot_api_ok: Option<bool>,
    pub last_get_updates_offset: Option<i64>,
    pub last_local_next_update_offset: Option<i64>,
    pub last_update_count: usize,
    pub last_allowed_update_count: usize,
    pub last_model_turn_started: bool,
    pub last_send_started: bool,
    pub last_cursor_written: bool,
    pub last_external_send: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_response_text_exposed: bool,
    pub raw_token_exposed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct NativeTelegramLiveSoakObservationState {
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

impl NativeTelegramLiveSoakObservationState {
    pub fn observe(&mut self, status: &NativeTelegramDrainOnceStatus, observed_at_unix_ms: u64) {
        self.poll_iterations = self.poll_iterations.saturating_add(1);
        match status.status {
            "drained" => {
                self.drained_count = self.drained_count.saturating_add(1);
                self.last_drained_at_unix_ms = Some(observed_at_unix_ms);
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
        self.last_observed_at_unix_ms = Some(observed_at_unix_ms);
        self.last_status = Some(status.status.to_string());
        self.last_error = status
            .error
            .clone()
            .map(|error| telegram_redact_token_like_text(&error));
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

    pub fn report(&self) -> NativeTelegramLiveSoakObservationReport {
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

#[derive(Debug, Clone)]
pub struct NativeTelegramPluginStatusInput {
    pub requested: bool,
    pub poll_ms: u64,
    pub allowed_updates: &'static str,
    pub config: NativeTelegramConfigStatus,
    pub gates: NativeTelegramGatewayGateSummary,
    pub poll_loop_gate_enabled: bool,
}

pub fn build_telegram_plugin_status(
    input: NativeTelegramPluginStatusInput,
) -> NativeTelegramPluginStatus {
    if !input.requested {
        return NativeTelegramPluginStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested: false,
            status: "disabled",
            in_process_supervisor_ready: false,
            in_process_reply_loop_ready: false,
            model_turn_bridge_ready: false,
            bot_api_poll_ready: false,
            bot_api_send_ready: false,
            openclaw_gateway_runtime_dependency: false,
            external_network_read: false,
            external_send: false,
            poll_ms: input.poll_ms,
            allowed_updates: input.allowed_updates,
            config: NativeTelegramConfigStatus::disabled(),
            transport_plan: NativeTelegramTransportPlan::disabled(),
            ingress_parser: crate::telegram_policy::inspect_telegram_updates(&[]),
            cursor_plan: NativeTelegramCursorPlan::disabled(),
            model_turn_plan: NativeTelegramModelTurnPlan::disabled(),
            migration_blocker: None,
            next_migration_slice: "enable --with-telegram-plugin, then wire Bot API polling and model-turn delivery",
        };
    }

    let supervisor_ready = input.config.error.is_none();
    let config_ready = input.config.config_ready();
    let bot_api_poll_ready = config_ready && input.gates.live_read_gate_enabled;
    let model_turn_bridge_ready = config_ready && input.gates.model_turn_gate_enabled;
    let bot_api_send_ready = config_ready && input.gates.send_gate_enabled;
    let in_process_reply_loop_ready = bot_api_poll_ready
        && model_turn_bridge_ready
        && bot_api_send_ready
        && input.gates.delivery_approval_gate_enabled
        && input.poll_loop_gate_enabled;
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
        requested: true,
        status,
        in_process_supervisor_ready: supervisor_ready,
        in_process_reply_loop_ready,
        model_turn_bridge_ready,
        bot_api_poll_ready,
        bot_api_send_ready,
        openclaw_gateway_runtime_dependency: false,
        external_network_read: false,
        external_send: false,
        poll_ms: input.poll_ms,
        allowed_updates: input.allowed_updates,
        transport_plan: NativeTelegramTransportPlan::for_config_state(
            input.config.enabled,
            input.config.token_shape_ok,
            input.config.binding_ready,
        ),
        config: input.config,
        ingress_parser: crate::telegram_policy::inspect_telegram_updates(&[]),
        cursor_plan: NativeTelegramCursorPlan::ready(),
        model_turn_plan: plan_model_turn_for_updates(&[]),
        migration_blocker,
        next_migration_slice,
    }
}

#[derive(Debug, Clone)]
pub struct NativeTelegramModelTurnPlanStatusInput {
    pub requested: bool,
    pub config: NativeTelegramConfigStatus,
}

pub fn build_telegram_model_turn_plan_status(
    input: NativeTelegramModelTurnPlanStatusInput,
) -> NativeTelegramModelTurnPlanStatus {
    let cursor_plan = if input.requested {
        NativeTelegramCursorPlan::ready()
    } else {
        NativeTelegramCursorPlan::disabled()
    };
    let inspection = crate::telegram_policy::inspect_telegram_updates(&[]);
    let model_turn_plan = if input.requested {
        plan_model_turn_for_updates(&[])
    } else {
        NativeTelegramModelTurnPlan::disabled()
    };
    let config_ready = input.requested && input.config.config_ready();
    let status = if !input.requested {
        "disabled"
    } else if config_ready {
        "planned"
    } else {
        "attention"
    };
    let error = if input.requested && !config_ready {
        Some("Telegram config, token shape, or binding is not ready".to_string())
    } else {
        None
    };

    NativeTelegramModelTurnPlanStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: input.requested,
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
        config: input.config,
        cursor_plan,
        inspection,
        model_turn_plan,
        error,
        next_migration_slice: "wire the planned redacted candidates into a bounded Codex session runner",
    }
}

#[derive(Debug, Clone)]
pub struct NativeTelegramModelBridgeStatusInput<'a> {
    pub requested: bool,
    pub config: NativeTelegramConfigStatus,
    pub model_turn_gate_env: &'static str,
    pub model_turn_gate_enabled: bool,
    pub send_gate_env: &'static str,
    pub model_runner_plan: &'a NativeTelegramModelRunnerPlan,
}

pub fn build_telegram_model_bridge_status(
    input: NativeTelegramModelBridgeStatusInput<'_>,
) -> NativeTelegramModelBridgeStatus {
    let cursor_plan = if input.requested {
        NativeTelegramCursorPlan::ready()
    } else {
        NativeTelegramCursorPlan::disabled()
    };
    let model_turn_plan = if input.requested {
        plan_model_turn_for_updates(&[])
    } else {
        NativeTelegramModelTurnPlan::disabled()
    };
    let invocation_request = if input.requested {
        build_model_invocation_request_plan(
            &[],
            None,
            input.model_turn_gate_env,
            input.model_turn_gate_enabled,
        )
    } else {
        NativeTelegramModelInvocationRequestPlan::disabled(
            input.model_turn_gate_env,
            input.model_turn_gate_enabled,
        )
    };
    let model_execution = if input.requested {
        NativeTelegramModelExecutionReport::from_invocation_request(&invocation_request)
    } else {
        NativeTelegramModelExecutionReport::disabled(
            input.model_turn_gate_env,
            input.model_turn_gate_enabled,
        )
    };
    let bridge_plan = if input.requested {
        NativeTelegramSessionBridgePlan::ready(input.model_runner_plan)
    } else {
        NativeTelegramSessionBridgePlan::disabled()
    };
    let config_ready = input.requested && input.config.config_ready();
    let status = if !input.requested {
        "disabled"
    } else if !input.model_turn_gate_enabled {
        "gated"
    } else if config_ready {
        "planned"
    } else {
        "attention"
    };
    let error = if input.requested && !input.model_turn_gate_enabled {
        Some(format!(
            "Telegram model-turn bridge is gated; set {}=1 only after runner invocation wiring is ready",
            input.model_turn_gate_env
        ))
    } else if input.requested && !config_ready {
        Some("Telegram config, token shape, or binding is not ready".to_string())
    } else {
        None
    };

    NativeTelegramModelBridgeStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: input.requested,
        status,
        model_turn_gate_env: input.model_turn_gate_env,
        model_turn_gate_enabled: input.model_turn_gate_enabled,
        send_gate_env: input.send_gate_env,
        model_turn_bridge_ready: input.requested && input.model_turn_gate_enabled && config_ready,
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
        config: input.config,
        cursor_plan,
        model_turn_plan,
        invocation_request,
        model_execution,
        bridge_plan,
        error,
        next_migration_slice: "implement the gated session-runner invocation and keep Telegram send behind HEPTA_NATIVE_TELEGRAM_SEND",
    }
}

#[derive(Debug, Clone)]
pub struct NativeTelegramSendPlanStatusInput {
    pub requested: bool,
    pub config: NativeTelegramConfigStatus,
    pub send_gate_env: &'static str,
    pub send_gate_enabled: bool,
}

pub fn build_telegram_send_plan_status(
    input: NativeTelegramSendPlanStatusInput,
) -> NativeTelegramSendPlanStatus {
    let transport_plan = NativeTelegramTransportPlan::for_config_state(
        input.config.enabled,
        input.config.token_shape_ok,
        input.config.binding_ready,
    );
    let send_plan = if input.requested {
        NativeTelegramSendPlan::ready()
    } else {
        NativeTelegramSendPlan::disabled()
    };
    let send_request = if input.requested {
        NativeTelegramSendRequestPlan::from_model_output(
            None,
            false,
            None,
            input.send_gate_env,
            input.send_gate_enabled,
        )
    } else {
        NativeTelegramSendRequestPlan::disabled(input.send_gate_env, input.send_gate_enabled)
    };
    let config_ready = input.requested && input.config.config_ready();
    let status = if !input.requested {
        "disabled"
    } else if !input.send_gate_enabled {
        "gated"
    } else if config_ready {
        "planned"
    } else {
        "attention"
    };
    let error = if input.requested && !input.send_gate_enabled {
        Some(format!(
            "Telegram send is gated; set {}=1 only after model-turn delivery wiring is ready",
            input.send_gate_env
        ))
    } else if input.requested && !config_ready {
        Some("Telegram config, token shape, or binding is not ready".to_string())
    } else {
        None
    };

    NativeTelegramSendPlanStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: input.requested,
        status,
        send_gate_env: input.send_gate_env,
        send_gate_enabled: input.send_gate_enabled,
        bot_api_send_ready: input.requested && input.send_gate_enabled && config_ready,
        external_network_write: false,
        external_send: false,
        cursor_written: false,
        raw_response_text_exposed: false,
        raw_chat_id_exposed: false,
        raw_message_id_exposed: false,
        raw_token_exposed: false,
        config: input.config,
        transport_plan,
        send_plan,
        send_request,
        error,
        next_migration_slice: "wire sendMessage execution after model output, then commit cursor only after delivery success",
    }
}

#[derive(Debug, Clone)]
pub struct NativeTelegramDrainOnceStatusInput {
    pub requested: bool,
    pub status: &'static str,
    pub gates: NativeTelegramGatewayGateSummary,
    pub config: NativeTelegramConfigStatus,
    pub execution_plan: NativeTelegramExecutionPlan,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub inspection: NativeTelegramIngressInspection,
    pub model_turn_plan: NativeTelegramModelTurnPlan,
    pub invocation_request: NativeTelegramModelInvocationRequestPlan,
    pub model_execution: NativeTelegramModelExecutionReport,
    pub send_plan: NativeTelegramSendPlan,
    pub send_request: NativeTelegramSendRequestPlan,
    pub send_execution: NativeTelegramSendExecutionReport,
    pub bot_api_ok: Option<bool>,
    pub local_next_update_offset: Option<i64>,
    pub get_updates_offset: Option<i64>,
    pub live_read_started: bool,
    pub external_network_read: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramDrainOncePreflightInput<'a> {
    pub requested: bool,
    pub gates: &'a NativeTelegramGatewayGateSummary,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramDrainOncePreflightPlan {
    pub status: &'static str,
    pub error: Option<String>,
    pub execution_plan: NativeTelegramExecutionPlan,
    pub status_probe_executes_pipeline: bool,
    pub cursor_plan: NativeTelegramCursorPlan,
    pub inspection: NativeTelegramIngressInspection,
    pub model_turn_plan: NativeTelegramModelTurnPlan,
    pub invocation_request: NativeTelegramModelInvocationRequestPlan,
    pub model_execution: NativeTelegramModelExecutionReport,
    pub send_plan: NativeTelegramSendPlan,
    pub send_request: NativeTelegramSendRequestPlan,
    pub send_execution: NativeTelegramSendExecutionReport,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramDrainOnceApiResultInput<'a> {
    pub requested: bool,
    pub gates: &'a NativeTelegramGatewayGateSummary,
    pub next_update_offset: Option<i64>,
    pub api_result: Result<&'a Value, &'a str>,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramDrainOnceApiResultPlan {
    pub status: &'static str,
    pub error: Option<String>,
    pub should_execute_pipeline: bool,
    pub bot_api_ok: Option<bool>,
    pub local_next_update_offset: Option<i64>,
    pub inspection: NativeTelegramIngressInspection,
    pub model_turn_plan: NativeTelegramModelTurnPlan,
    pub invocation_request: NativeTelegramModelInvocationRequestPlan,
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramDrainOnceShellReadinessInput<'a> {
    pub cursor_file_present: bool,
    pub cursor_parse_ok: bool,
    pub cursor_error: Option<&'a str>,
    pub config_ready: bool,
    pub token_error: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeTelegramDrainOnceShellReadinessPlan {
    pub status: &'static str,
    pub error: Option<String>,
    pub may_call_bot_api: bool,
}

pub fn plan_telegram_drain_once_preflight(
    input: NativeTelegramDrainOncePreflightInput<'_>,
) -> NativeTelegramDrainOncePreflightPlan {
    let cursor_plan = if input.requested {
        NativeTelegramCursorPlan::ready()
    } else {
        NativeTelegramCursorPlan::disabled()
    };
    let inspection = crate::telegram_policy::inspect_telegram_updates(&[]);
    let model_turn_plan = if input.requested {
        plan_model_turn_for_updates(&[])
    } else {
        NativeTelegramModelTurnPlan::disabled()
    };
    let invocation_request = if input.requested {
        build_model_invocation_request_plan(
            &[],
            None,
            input.gates.model_turn_gate_env,
            input.gates.model_turn_gate_enabled,
        )
    } else {
        NativeTelegramModelInvocationRequestPlan::disabled(
            input.gates.model_turn_gate_env,
            input.gates.model_turn_gate_enabled,
        )
    };
    let send_plan = if input.requested {
        NativeTelegramSendPlan::ready()
    } else {
        NativeTelegramSendPlan::disabled()
    };
    let send_request = if input.requested {
        NativeTelegramSendRequestPlan::from_model_output(
            None,
            false,
            None,
            input.gates.send_gate_env,
            input.gates.send_gate_enabled,
        )
    } else {
        NativeTelegramSendRequestPlan::disabled(
            input.gates.send_gate_env,
            input.gates.send_gate_enabled,
        )
    };
    let send_execution = if input.requested {
        NativeTelegramSendExecutionReport::from_send_request(&send_request)
    } else {
        NativeTelegramSendExecutionReport::disabled(
            input.gates.send_gate_env,
            input.gates.send_gate_enabled,
        )
    };
    let model_execution = if input.requested {
        NativeTelegramModelExecutionReport::from_invocation_request(&invocation_request)
    } else {
        NativeTelegramModelExecutionReport::disabled(
            input.gates.model_turn_gate_env,
            input.gates.model_turn_gate_enabled,
        )
    };
    let execution_plan = telegram_drain_execution_plan(input.requested, input.gates);
    let first_missing_gate = execution_plan.first_missing_gate;
    let all_required_gates_enabled = execution_plan.all_required_gates_enabled;
    let status_probe_executes_pipeline = execution_plan.status_probe_executes_pipeline;
    let status = if !input.requested {
        "disabled"
    } else if all_required_gates_enabled {
        "planned"
    } else {
        "gated"
    };
    let error = if input.requested {
        first_missing_gate.map(|gate| {
            format!(
                "Telegram drain-once pipeline is gated before side effects; first missing gate: {gate}"
            )
        })
    } else {
        None
    };

    NativeTelegramDrainOncePreflightPlan {
        status,
        error,
        execution_plan,
        status_probe_executes_pipeline,
        cursor_plan,
        inspection,
        model_turn_plan,
        invocation_request,
        model_execution,
        send_plan,
        send_request,
        send_execution,
    }
}

pub fn plan_telegram_drain_once_shell_readiness(
    input: NativeTelegramDrainOnceShellReadinessInput<'_>,
) -> NativeTelegramDrainOnceShellReadinessPlan {
    if input.cursor_file_present && !input.cursor_parse_ok {
        return NativeTelegramDrainOnceShellReadinessPlan {
            status: "attention",
            error: Some(
                input
                    .cursor_error
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| "Telegram cursor state is not readable".to_string()),
            ),
            may_call_bot_api: false,
        };
    }

    if !input.config_ready {
        return NativeTelegramDrainOnceShellReadinessPlan {
            status: "attention",
            error: Some("Telegram config, token shape, or binding is not ready".to_string()),
            may_call_bot_api: false,
        };
    }

    if let Some(token_error) = input.token_error {
        return NativeTelegramDrainOnceShellReadinessPlan {
            status: "attention",
            error: Some(telegram_redact_token_like_text(token_error)),
            may_call_bot_api: false,
        };
    }

    NativeTelegramDrainOnceShellReadinessPlan {
        status: "planned",
        error: None,
        may_call_bot_api: true,
    }
}

pub fn plan_telegram_drain_once_api_result(
    input: NativeTelegramDrainOnceApiResultInput<'_>,
) -> NativeTelegramDrainOnceApiResultPlan {
    match input.api_result {
        Ok(api) => {
            let bot_api_ok = api.get("ok").and_then(Value::as_bool);
            let updates = api
                .get("result")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            let inspection = crate::telegram_policy::inspect_telegram_updates(&updates);
            let model_turn_plan = plan_model_turn_for_updates(&updates);
            let invocation_request = build_model_invocation_request_plan(
                &updates,
                input.next_update_offset,
                input.gates.model_turn_gate_env,
                input.gates.model_turn_gate_enabled,
            );
            if bot_api_ok == Some(false) {
                return NativeTelegramDrainOnceApiResultPlan {
                    status: "attention",
                    error: api
                        .get("description")
                        .and_then(Value::as_str)
                        .map(telegram_redact_token_like_text)
                        .or_else(|| {
                            Some("Telegram Bot API getUpdates returned ok=false".to_string())
                        }),
                    should_execute_pipeline: false,
                    bot_api_ok,
                    local_next_update_offset: inspection.latest_allowed_next_update_offset,
                    inspection,
                    model_turn_plan,
                    invocation_request,
                };
            }

            NativeTelegramDrainOnceApiResultPlan {
                status: "planned",
                error: None,
                should_execute_pipeline: true,
                bot_api_ok,
                local_next_update_offset: inspection.latest_allowed_next_update_offset,
                inspection,
                model_turn_plan,
                invocation_request,
            }
        }
        Err(error) => {
            let redacted_error = telegram_redact_token_like_text(error);
            let status = if telegram_get_updates_error_is_conflict(&redacted_error) {
                "busy"
            } else {
                "attention"
            };
            let updates = Vec::new();
            NativeTelegramDrainOnceApiResultPlan {
                status,
                error: Some(redacted_error),
                should_execute_pipeline: false,
                bot_api_ok: None,
                local_next_update_offset: None,
                inspection: crate::telegram_policy::inspect_telegram_updates(&updates),
                model_turn_plan: plan_model_turn_for_updates(&updates),
                invocation_request: build_model_invocation_request_plan(
                    &updates,
                    input.next_update_offset,
                    input.gates.model_turn_gate_env,
                    input.gates.model_turn_gate_enabled,
                ),
            }
        }
    }
}

pub fn build_telegram_drain_once_status(
    input: NativeTelegramDrainOnceStatusInput,
) -> NativeTelegramDrainOnceStatus {
    let model_turn_started = input.model_execution.session_runner_invoked;
    let send_started = input.send_execution.send_attempted;
    let cursor_written = input.send_execution.cursor_written;
    let external_network_write = input.send_execution.external_network_write;
    let external_send = input.send_execution.external_send;

    NativeTelegramDrainOnceStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: input.requested,
        status: input.status,
        gates: input.gates,
        config: input.config,
        execution_plan: input.execution_plan,
        cursor_plan: input.cursor_plan,
        inspection: input.inspection,
        model_turn_plan: input.model_turn_plan,
        invocation_request: input.invocation_request,
        model_execution: input.model_execution,
        send_plan: input.send_plan,
        send_request: input.send_request,
        send_execution: input.send_execution,
        bot_api_ok: input.bot_api_ok,
        local_next_update_offset: input.local_next_update_offset,
        get_updates_offset: input.get_updates_offset,
        live_read_started: input.live_read_started,
        model_turn_started,
        send_started,
        cursor_written,
        external_network_read: input.external_network_read,
        external_network_write,
        external_send,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
        error: input.error,
        next_migration_slice: "continue live production soak with bounded retries, typing keepalive, fallback, and send throttling",
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramPollLoopStatusInput {
    pub requested: bool,
    pub poll_ms: u64,
    pub poll_loop_gate_env: &'static str,
    pub poll_loop_gate_enabled: bool,
    pub delivery_approval_gate_env: &'static str,
    pub delivery_approval_gate_enabled: bool,
    pub live_read_gate_env: &'static str,
    pub model_turn_gate_env: &'static str,
    pub send_gate_env: &'static str,
}

pub fn build_telegram_poll_loop_status(
    input: NativeTelegramPollLoopStatusInput,
) -> NativeTelegramPollLoopStatus {
    let status = if !input.requested {
        "disabled"
    } else if input.poll_loop_gate_enabled && input.delivery_approval_gate_enabled {
        "armed"
    } else if input.poll_loop_gate_enabled {
        "approval_required"
    } else {
        "gated"
    };

    NativeTelegramPollLoopStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: input.requested,
        status,
        poll_loop_gate_env: input.poll_loop_gate_env,
        poll_loop_gate_enabled: input.poll_loop_gate_enabled,
        delivery_approval_gate_env: input.delivery_approval_gate_env,
        delivery_approval_gate_enabled: input.delivery_approval_gate_enabled,
        poll_ms: input.poll_ms,
        drain_once_endpoint: "/api/telegram-drain-once",
        worker_spawned_by_status: false,
        loop_invokes_drain_once: input.requested
            && input.poll_loop_gate_enabled
            && input.delivery_approval_gate_enabled,
        requires_live_read_gate: input.live_read_gate_env,
        requires_model_turn_gate: input.model_turn_gate_env,
        requires_send_gate: input.send_gate_env,
        requires_delivery_approval_gate: input.delivery_approval_gate_env,
        external_network_read_by_status: false,
        external_send_by_status: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
        next_migration_slice: "continue live soak and inspect /api/telegram-live-soak for production guard health",
    }
}

#[derive(Debug, Clone)]
pub struct NativeTelegramLiveSoakStatusInput {
    pub requested: bool,
    pub poll_loop_status: NativeTelegramPollLoopStatus,
    pub cursor_status: NativeTelegramCursorStatus,
    pub delivery_ledger_status: NativeTelegramDeliveryLedgerStatus,
    pub production_guards: NativeTelegramProductionGuardStatus,
    pub production_readiness: NativeTelegramProductionReadinessStatus,
    pub observation: NativeTelegramLiveSoakObservationReport,
}

pub fn build_telegram_live_soak_status(
    input: NativeTelegramLiveSoakStatusInput,
) -> NativeTelegramLiveSoakStatus {
    let last_status = input.observation.last_status.as_deref();
    let status = if !input.requested {
        "disabled"
    } else if !input.poll_loop_status.loop_invokes_drain_once {
        "gated"
    } else if input.cursor_status.status == "attention"
        || last_status == Some("attention")
        || !input.production_readiness.attention_budget_ok
    {
        "attention"
    } else if input.observation.poll_iterations == 0 {
        "warming"
    } else if !input.production_readiness.production_guards_ready {
        "attention"
    } else {
        "soaking"
    };

    NativeTelegramLiveSoakStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: input.requested,
        status,
        side_effect_free: true,
        endpoint: "/api/telegram-live-soak",
        poll_loop_status: input.poll_loop_status,
        cursor_status: input.cursor_status,
        delivery_ledger_status: input.delivery_ledger_status,
        production_guards: input.production_guards,
        health_ready: input.production_readiness.ready,
        production_readiness: input.production_readiness,
        observation: input.observation,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
        next_migration_slice: "keep the active gateway soaking; use this endpoint plus logs before broadening traffic or reducing guards",
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramProductionReadinessInput<'a> {
    pub requested: bool,
    pub poll_loop_status: &'a NativeTelegramPollLoopStatus,
    pub cursor_status: &'a NativeTelegramCursorStatus,
    pub delivery_ledger_status: &'a NativeTelegramDeliveryLedgerStatus,
    pub production_guards: &'a NativeTelegramProductionGuardStatus,
    pub observation: &'a NativeTelegramLiveSoakObservationReport,
    pub min_poll_iterations_env: &'static str,
    pub min_poll_iterations: u64,
    pub max_attention_count_env: &'static str,
    pub max_attention_count: u64,
    pub max_observed_age_env: &'static str,
    pub max_observed_age_ms: u64,
    pub now_unix_ms: u64,
}

pub fn build_telegram_production_readiness_status(
    input: NativeTelegramProductionReadinessInput<'_>,
) -> NativeTelegramProductionReadinessStatus {
    let poll_loop_armed = input.requested
        && input.poll_loop_status.status == "armed"
        && input.poll_loop_status.loop_invokes_drain_once;
    let cursor_ready = input.cursor_status.status == "ready"
        && input.cursor_status.cursor_parse_ok
        && input.cursor_status.duplicate_suppression_rule_valid;
    let production_guards_ready = input.production_guards.typing_keepalive_enabled
        && input.production_guards.model_failure_fallback_enabled
        && input.production_guards.model_timeout_ms >= 1_000
        && input.production_guards.read_max_attempts >= 1
        && input.production_guards.send_max_attempts >= 1
        && input.production_guards.send_min_interval_ms > 0
        && input.production_guards.retry_transient_read_errors
        && input.production_guards.retry_transient_send_errors
        && !input.production_guards.raw_token_exposed;
    let observation_ready = input.observation.poll_iterations >= input.min_poll_iterations
        && input.observation.last_observed_at_unix_ms.is_some();
    let observation_fresh = input
        .observation
        .last_observed_at_unix_ms
        .map(|last_observed| {
            input.now_unix_ms.saturating_sub(last_observed) <= input.max_observed_age_ms
        })
        .unwrap_or(false);
    let durable_cursor_evidence_present = input.cursor_status.durable_cursor_evidence_present;
    let durable_delivery_evidence_required = input.observation.drained_count > 0
        || input.observation.send_started_count > 0
        || input.observation.cursor_written_count > 0
        || input.observation.external_send_count > 0;
    let durable_delivery_evidence_present = input
        .delivery_ledger_status
        .durable_delivery_evidence_present;
    let delivery_evidence_reference_ms = input
        .observation
        .last_drained_at_unix_ms
        .or(input.observation.last_observed_at_unix_ms);
    let durable_delivery_evidence_fresh = if durable_delivery_evidence_required {
        input
            .delivery_ledger_status
            .latest_acked_created_unix_seconds
            .map(|created| created.saturating_mul(1_000))
            .zip(delivery_evidence_reference_ms)
            .map(|(acked_ms, reference_ms)| {
                acked_ms.saturating_add(input.max_observed_age_ms) >= reference_ms
            })
            .unwrap_or(false)
    } else {
        true
    };
    let delivery_ledger_ready = if durable_delivery_evidence_required {
        input.delivery_ledger_status.status == "ready"
            && input.delivery_ledger_status.jsonl_valid
            && durable_delivery_evidence_present
            && durable_delivery_evidence_fresh
    } else {
        !matches!(input.delivery_ledger_status.status, "attention")
    };
    let attention_budget_ok = input.observation.attention_count <= input.max_attention_count
        && input.observation.last_status.as_deref() != Some("attention");
    let recent_bot_api_ok = input.observation.last_bot_api_ok != Some(false);
    let redaction_guards_ok = !input.observation.raw_update_payload_exposed
        && !input.observation.raw_prompt_text_exposed
        && !input.observation.raw_response_text_exposed
        && !input.observation.raw_token_exposed
        && !input.poll_loop_status.raw_update_payload_exposed
        && !input.poll_loop_status.raw_prompt_text_exposed
        && !input.poll_loop_status.raw_response_text_exposed
        && !input.poll_loop_status.raw_token_exposed
        && !input.delivery_ledger_status.raw_response_text_logged
        && !input.delivery_ledger_status.raw_chat_id_logged
        && !input.delivery_ledger_status.raw_message_id_logged
        && !input.delivery_ledger_status.raw_token_logged;

    let mut readiness_blockers = Vec::new();
    if !input.requested {
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
    if !observation_fresh {
        readiness_blockers.push("observation_stale");
    }
    if !delivery_ledger_ready {
        readiness_blockers.push("delivery_ledger_not_ready");
    }
    if durable_delivery_evidence_required && !durable_delivery_evidence_present {
        readiness_blockers.push("durable_delivery_evidence_missing");
    }
    if durable_delivery_evidence_required && !durable_delivery_evidence_fresh {
        readiness_blockers.push("durable_delivery_evidence_stale");
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
    if input.observation.busy_count > 0 {
        readiness_warnings.push("getupdates_busy_conflicts_observed");
    }
    if input.observation.drained_count == 0
        && !durable_cursor_evidence_present
        && !durable_delivery_evidence_present
    {
        readiness_warnings.push("no_messages_drained_since_gateway_start");
    }
    if input.observation.external_send_count > input.observation.cursor_written_count {
        readiness_warnings.push("send_count_exceeds_cursor_write_count");
    }

    let ready = readiness_blockers.is_empty();
    let status = if !input.requested {
        "disabled"
    } else if !poll_loop_armed || !cursor_ready {
        "gated"
    } else if !observation_fresh
        || !attention_budget_ok
        || !recent_bot_api_ok
        || !redaction_guards_ok
    {
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
        requested: input.requested,
        status,
        ready,
        side_effect_free: true,
        min_poll_iterations_env: input.min_poll_iterations_env,
        min_poll_iterations: input.min_poll_iterations,
        max_attention_count_env: input.max_attention_count_env,
        max_attention_count: input.max_attention_count,
        max_observed_age_env: input.max_observed_age_env,
        max_observed_age_ms: input.max_observed_age_ms,
        poll_loop_armed,
        cursor_ready,
        production_guards_ready,
        observation_ready,
        observation_fresh,
        durable_cursor_evidence_present,
        durable_delivery_evidence_required,
        durable_delivery_evidence_present,
        durable_delivery_evidence_fresh,
        delivery_ledger_ready,
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

#[cfg(test)]
mod tests {
    use super::*;

    const LIVE_READ_ENV: &str = "HEPTA_NATIVE_TELEGRAM_LIVE_READ";
    const MODEL_TURN_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_TURN";
    const SEND_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND";
    const DELIVERY_APPROVED_ENV: &str = "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED";
    const POLL_LOOP_ENV: &str = "HEPTA_NATIVE_TELEGRAM_POLL_LOOP";
    const SOAK_MIN_POLLS_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS";
    const SOAK_MAX_ATTENTION_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION";
    const SOAK_MAX_OBSERVED_AGE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS";
    const TEST_CURSOR_PATH: &str = ".hepta/telegram/ingress-drain-cursor.json";
    const TEST_DELIVERY_LEDGER_PATH: &str = ".hepta/telegram/delivery-ledger.jsonl";
    const TEST_NOW_MS: u64 = 1_000_000;

    fn gateway_gates(all_enabled: bool) -> NativeTelegramGatewayGateSummary {
        NativeTelegramGatewayGateSummary {
            delivery_approval_gate_env: DELIVERY_APPROVED_ENV,
            delivery_approval_gate_enabled: all_enabled,
            live_read_gate_env: LIVE_READ_ENV,
            live_read_gate_enabled: all_enabled,
            model_turn_gate_env: MODEL_TURN_ENV,
            model_turn_gate_enabled: all_enabled,
            send_gate_env: SEND_ENV,
            send_gate_enabled: all_enabled,
            readiness_summary_performs_live_read: false,
            readiness_summary_invokes_model: false,
            readiness_summary_sends_message: false,
        }
    }

    fn ready_config() -> NativeTelegramConfigStatus {
        NativeTelegramConfigStatus {
            config_path: Some("private/config/openclaw.json".to_string()),
            config_found: true,
            enabled: true,
            dm_policy: "trusted".to_string(),
            group_policy: "deny".to_string(),
            allow_from_count: 1,
            group_count: 0,
            token_source: "secret_file",
            token_secret_ref_present: true,
            token_secret_provider: Some("telegram_bot".to_string()),
            token_secret_id_present: true,
            token_file_present: true,
            token_file_mode_0600: true,
            token_shape_ok: true,
            raw_token_exposed: false,
            binding_ready: true,
            error: None,
        }
    }

    fn production_readiness_status_from_test_parts(
        requested: bool,
        poll_loop_status: &NativeTelegramPollLoopStatus,
        cursor_status: &NativeTelegramCursorStatus,
        delivery_ledger_status: &NativeTelegramDeliveryLedgerStatus,
        production_guards: &NativeTelegramProductionGuardStatus,
        observation: &NativeTelegramLiveSoakObservationReport,
    ) -> NativeTelegramProductionReadinessStatus {
        build_telegram_production_readiness_status(NativeTelegramProductionReadinessInput {
            requested,
            poll_loop_status,
            cursor_status,
            delivery_ledger_status,
            production_guards,
            observation,
            min_poll_iterations_env: SOAK_MIN_POLLS_ENV,
            min_poll_iterations: DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
            max_attention_count_env: SOAK_MAX_ATTENTION_ENV,
            max_attention_count: DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION,
            max_observed_age_env: SOAK_MAX_OBSERVED_AGE_ENV,
            max_observed_age_ms: DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS,
            now_unix_ms: TEST_NOW_MS,
        })
    }

    fn ready_poll_loop_status() -> NativeTelegramPollLoopStatus {
        NativeTelegramPollLoopStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested: true,
            status: "armed",
            poll_loop_gate_env: POLL_LOOP_ENV,
            poll_loop_gate_enabled: true,
            delivery_approval_gate_env: DELIVERY_APPROVED_ENV,
            delivery_approval_gate_enabled: true,
            poll_ms: 1500,
            drain_once_endpoint: "/api/telegram-drain-once",
            worker_spawned_by_status: false,
            loop_invokes_drain_once: true,
            requires_live_read_gate: LIVE_READ_ENV,
            requires_model_turn_gate: MODEL_TURN_ENV,
            requires_send_gate: SEND_ENV,
            requires_delivery_approval_gate: DELIVERY_APPROVED_ENV,
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
            cursor_path: TEST_CURSOR_PATH,
            cursor_file_present: true,
            cursor_parse_ok: true,
            next_update_offset: Some(917025970),
            cursor_updated_at_unix_ms: Some(TEST_NOW_MS),
            last_delivered_next_update_offset: Some(917025970),
            durable_cursor_evidence_present: true,
            cursor_represents_next_update_offset: true,
            duplicate_suppression_rule_valid: true,
            cursor_write_policy: "write only after model output is delivered or duplicate suppression is recorded",
            cursor_written: false,
            raw_update_payload_persisted: false,
            error: None,
            next_migration_slice: "test",
        }
    }

    fn ready_delivery_ledger_status() -> NativeTelegramDeliveryLedgerStatus {
        NativeTelegramDeliveryLedgerStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested: true,
            status: "ready",
            ledger_path: TEST_DELIVERY_LEDGER_PATH,
            ledger_file_present: true,
            jsonl_readable: true,
            jsonl_valid: true,
            line_count: 2,
            valid_json_line_count: 2,
            invalid_json_line_count: 0,
            acked_count: 1,
            failed_count: 0,
            latest_stage: Some("acked".to_string()),
            latest_created_unix_seconds: Some(TEST_NOW_MS / 1_000),
            latest_acked_created_unix_seconds: Some(TEST_NOW_MS / 1_000),
            ledger_updated_at_unix_ms: Some(TEST_NOW_MS),
            provider_message_id_present: true,
            durable_delivery_evidence_present: true,
            raw_response_text_logged: false,
            raw_chat_id_logged: false,
            raw_message_id_logged: false,
            raw_token_logged: false,
            error: None,
            next_migration_slice: "test",
        }
    }

    fn ready_production_guards() -> NativeTelegramProductionGuardStatus {
        NativeTelegramProductionGuardStatus {
            read_max_attempts_env: "HEPTA_NATIVE_TELEGRAM_READ_MAX_ATTEMPTS",
            read_max_attempts: 3,
            read_retry_backoff_env: "HEPTA_NATIVE_TELEGRAM_READ_RETRY_BACKOFF_MS",
            read_retry_backoff_ms: 700,
            retry_transient_read_errors: true,
            typing_keepalive_env: "HEPTA_NATIVE_TELEGRAM_TYPING_KEEPALIVE",
            typing_keepalive_enabled: true,
            typing_keepalive_interval_ms: 4000,
            model_timeout_env: "HEPTA_NATIVE_TELEGRAM_MODEL_TIMEOUT_MS",
            model_timeout_ms: 120000,
            model_failure_fallback_env: "HEPTA_NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK",
            model_failure_fallback_enabled: true,
            send_min_interval_env: "HEPTA_NATIVE_TELEGRAM_SEND_MIN_INTERVAL_MS",
            send_min_interval_ms: 1200,
            send_max_attempts_env: "HEPTA_NATIVE_TELEGRAM_SEND_MAX_ATTEMPTS",
            send_max_attempts: 3,
            send_retry_backoff_env: "HEPTA_NATIVE_TELEGRAM_SEND_RETRY_BACKOFF_MS",
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
            last_observed_at_unix_ms: Some(TEST_NOW_MS),
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

    fn ready_model_runner_plan() -> NativeTelegramModelRunnerPlan {
        NativeTelegramModelRunnerPlan {
            runner_plan_ready: true,
            runner_kind: "hepta_in_process_exec_runner",
            runner_invocation_strategy: "in_process",
            codex_core_runner_enabled: false,
            in_process_runner_enabled: true,
            mlx_base_url: None,
            mlx_model: None,
            mlx_max_tokens: None,
            local_network_call: false,
            process_spawned_by_status: false,
            hepta_intelligence_context_injected: true,
            plugin_capability_context_injected: true,
            raw_prompt_text_exposed: false,
        }
    }

    #[test]
    fn soak_env_policy_helpers_clamp_and_default() {
        assert_eq!(
            telegram_soak_min_poll_iterations_policy(None),
            DEFAULT_TELEGRAM_SOAK_MIN_POLLS
        );
        assert_eq!(telegram_soak_min_poll_iterations_policy(Some(0)), 1);
        assert_eq!(
            telegram_soak_min_poll_iterations_policy(Some(999_999)),
            MAX_TELEGRAM_SOAK_MIN_POLLS
        );
        assert_eq!(
            telegram_soak_max_attention_count_policy(None),
            DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION
        );
        assert_eq!(
            telegram_soak_max_attention_count_policy(Some(999_999)),
            MAX_TELEGRAM_SOAK_MAX_ATTENTION
        );
        assert_eq!(
            telegram_soak_max_observed_age_ms_policy(None),
            DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS
        );
        assert_eq!(telegram_soak_max_observed_age_ms_policy(Some(1)), 1_000);
        assert_eq!(
            telegram_soak_max_observed_age_ms_policy(Some(999_999_999)),
            MAX_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS
        );
    }

    #[test]
    fn poll_loop_policies_are_side_effect_free() {
        assert!(telegram_poll_loop_should_spawn(true, true, true));
        assert!(!telegram_poll_loop_should_spawn(false, true, true));
        assert!(!telegram_poll_loop_should_spawn(true, false, true));
        assert!(!telegram_poll_loop_should_spawn(true, true, false));
        assert_eq!(
            telegram_poll_loop_interval_ms_policy(1),
            MIN_TELEGRAM_POLL_LOOP_INTERVAL_MS
        );
        assert_eq!(telegram_poll_loop_interval_ms_policy(1_500), 1_500);
        assert_eq!(
            telegram_poll_loop_interval_ms_policy(999_999),
            MAX_TELEGRAM_POLL_LOOP_INTERVAL_MS
        );
        assert_eq!(telegram_receive_limit_policy(0), 1);
        assert_eq!(telegram_receive_limit_policy(7), 7);
        assert_eq!(telegram_receive_limit_policy(999), 20);
    }

    #[test]
    fn plugin_status_reports_native_supervisor_without_reply_loop_claim() {
        let plugin = build_telegram_plugin_status(NativeTelegramPluginStatusInput {
            requested: true,
            poll_ms: 1_500,
            allowed_updates: "[\"message\",\"callback_query\"]",
            config: ready_config(),
            gates: NativeTelegramGatewayGateSummary {
                delivery_approval_gate_env: DELIVERY_APPROVED_ENV,
                delivery_approval_gate_enabled: false,
                live_read_gate_env: LIVE_READ_ENV,
                live_read_gate_enabled: false,
                model_turn_gate_env: MODEL_TURN_ENV,
                model_turn_gate_enabled: false,
                send_gate_env: SEND_ENV,
                send_gate_enabled: false,
                readiness_summary_performs_live_read: false,
                readiness_summary_invokes_model: false,
                readiness_summary_sends_message: false,
            },
            poll_loop_gate_enabled: false,
        });

        assert_eq!(plugin.status, "native_supervisor_ready");
        assert!(plugin.in_process_supervisor_ready);
        assert!(!plugin.in_process_reply_loop_ready);
        assert!(!plugin.model_turn_bridge_ready);
        assert!(!plugin.bot_api_poll_ready);
        assert!(!plugin.bot_api_send_ready);
        assert!(!plugin.openclaw_gateway_runtime_dependency);
        assert!(!plugin.external_network_read);
        assert!(!plugin.external_send);
        assert!(plugin.cursor_plan.duplicate_suppression_ready);
        assert!(plugin.cursor_plan.commit_offset_after_delivery);
        assert!(plugin.model_turn_plan.planner_ready);
    }

    #[test]
    fn model_bridge_without_gate_is_gated_and_side_effect_free() {
        let runner_plan = ready_model_runner_plan();
        let status = build_telegram_model_bridge_status(NativeTelegramModelBridgeStatusInput {
            requested: true,
            config: ready_config(),
            model_turn_gate_env: MODEL_TURN_ENV,
            model_turn_gate_enabled: false,
            send_gate_env: SEND_ENV,
            model_runner_plan: &runner_plan,
        });

        assert_eq!(status.status, "gated");
        assert_eq!(status.model_turn_gate_env, MODEL_TURN_ENV);
        assert_eq!(status.send_gate_env, SEND_ENV);
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
        assert!(status.error.unwrap().contains(MODEL_TURN_ENV));
    }

    #[test]
    fn send_plan_without_gate_is_gated_and_side_effect_free() {
        let status = build_telegram_send_plan_status(NativeTelegramSendPlanStatusInput {
            requested: true,
            config: ready_config(),
            send_gate_env: SEND_ENV,
            send_gate_enabled: false,
        });

        assert_eq!(status.status, "gated");
        assert_eq!(status.send_gate_env, SEND_ENV);
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
        assert!(status.error.unwrap().contains(SEND_ENV));
    }

    #[test]
    fn system_time_unix_ms_conversion_is_bounded() {
        assert_eq!(telegram_system_time_unix_ms(UNIX_EPOCH), 0);
        assert_eq!(
            telegram_system_time_unix_ms(UNIX_EPOCH + Duration::from_millis(42)),
            42
        );
        assert_eq!(
            telegram_system_time_unix_ms(UNIX_EPOCH - Duration::from_millis(1)),
            0
        );
    }

    #[test]
    fn receive_once_preflight_reports_gate_without_side_effects() {
        let config = ready_config();
        let transport_plan = NativeTelegramTransportPlan::for_config_state(true, true, true);
        let cursor_plan = NativeTelegramCursorPlan::ready();

        let report =
            plan_telegram_receive_once_preflight_status(NativeTelegramReceiveOncePreflightInput {
                requested: true,
                live_read_gate_env: LIVE_READ_ENV,
                live_read_gate_enabled: false,
                limit: 99,
                config: &config,
                transport_plan: &transport_plan,
                cursor_plan: &cursor_plan,
            })
            .expect("missing live-read gate should produce a status report");

        assert_eq!(report.status, "gated");
        assert_eq!(report.limit, 99);
        assert!(!report.external_network_read);
        assert!(!report.external_send);
        assert!(!report.cursor_written);
        assert!(!report.raw_token_exposed);
        assert_eq!(report.live_read_gate_env, LIVE_READ_ENV);
        assert_eq!(
            report.error.as_deref(),
            Some(
                "live Telegram receive is gated; set HEPTA_NATIVE_TELEGRAM_LIVE_READ=1 to run one redacted getUpdates read"
            )
        );
    }

    #[test]
    fn receive_once_preflight_allows_ready_live_read_to_cli_shell() {
        let config = ready_config();
        let transport_plan = NativeTelegramTransportPlan::for_config_state(true, true, true);
        let cursor_plan = NativeTelegramCursorPlan::ready();

        let report =
            plan_telegram_receive_once_preflight_status(NativeTelegramReceiveOncePreflightInput {
                requested: true,
                live_read_gate_env: LIVE_READ_ENV,
                live_read_gate_enabled: true,
                limit: 20,
                config: &config,
                transport_plan: &transport_plan,
                cursor_plan: &cursor_plan,
            });

        assert!(report.is_none());
    }

    #[test]
    fn receive_once_status_builder_preserves_observed_fields() {
        let config = ready_config();
        let transport_plan = NativeTelegramTransportPlan::for_config_state(true, true, true);
        let cursor_plan = NativeTelegramCursorPlan::ready();
        let inspection = crate::telegram_policy::inspect_telegram_updates(&[]);
        let model_turn_plan = NativeTelegramModelTurnPlan::disabled();

        let report = build_telegram_receive_once_status(NativeTelegramReceiveOnceStatusInput {
            requested: true,
            status: "ready",
            live_read_gate_env: LIVE_READ_ENV,
            live_read_gate_enabled: true,
            external_network_read: true,
            limit: 3,
            config,
            transport_plan,
            cursor_plan,
            inspection,
            model_turn_plan: Some(model_turn_plan),
            get_updates_offset: Some(41),
            bot_api_ok: Some(true),
            local_next_update_offset: Some(42),
            error: None,
            next_migration_slice: "test",
        });

        assert_eq!(report.status, "ready");
        assert_eq!(report.get_updates_offset, Some(41));
        assert_eq!(report.bot_api_ok, Some(true));
        assert_eq!(report.local_next_update_offset, Some(42));
        assert!(report.external_network_read);
        assert!(!report.raw_update_payload_exposed);
        assert!(!report.raw_token_exposed);
    }

    #[test]
    fn receive_once_error_status_builder_redacts_and_preserves_offset() {
        let report =
            build_telegram_receive_once_error_status(NativeTelegramReceiveOnceErrorInput {
                requested: true,
                live_read_gate_env: LIVE_READ_ENV,
                live_read_gate_enabled: true,
                limit: 20,
                config: ready_config(),
                transport_plan: NativeTelegramTransportPlan::for_config_state(true, true, true),
                cursor_plan: NativeTelegramCursorPlan::ready(),
                get_updates_offset: Some(41),
                error: Some("failed with 123456789:abcdefghijklmnopqrstuvwxyz".to_string()),
            });

        assert_eq!(report.status, "attention");
        assert_eq!(report.get_updates_offset, Some(41));
        assert!(!report.external_network_read);
        assert!(!report.external_send);
        assert!(!report.raw_token_exposed);
        let error = report.error.expect("error");
        assert!(error.contains("[redacted-telegram-token]"));
        assert!(!error.contains("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn receive_once_shell_readiness_redacts_token_error_before_bot_api() {
        let plan = plan_telegram_receive_once_shell_readiness(
            NativeTelegramReceiveOnceShellReadinessInput {
                token_error: Some("bad token 123456789:abcdefghijklmnopqrstuvwxyz"),
                cursor_file_present: false,
                cursor_parse_ok: true,
                cursor_error: None,
            },
        );

        assert_eq!(plan.status, "attention");
        assert!(!plan.may_call_bot_api);
        let error = plan.error.expect("error");
        assert!(error.contains("[redacted-telegram-token]"));
        assert!(!error.contains("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn receive_once_shell_readiness_blocks_cursor_parse_error() {
        let plan = plan_telegram_receive_once_shell_readiness(
            NativeTelegramReceiveOnceShellReadinessInput {
                token_error: None,
                cursor_file_present: true,
                cursor_parse_ok: false,
                cursor_error: Some("cursor JSON is malformed"),
            },
        );

        assert_eq!(plan.status, "attention");
        assert!(!plan.may_call_bot_api);
        assert_eq!(plan.error.as_deref(), Some("cursor JSON is malformed"));
    }

    #[test]
    fn receive_once_shell_readiness_allows_bot_api_after_ready_shell() {
        let plan = plan_telegram_receive_once_shell_readiness(
            NativeTelegramReceiveOnceShellReadinessInput {
                token_error: None,
                cursor_file_present: true,
                cursor_parse_ok: true,
                cursor_error: None,
            },
        );

        assert_eq!(plan.status, "planned");
        assert!(plan.error.is_none());
        assert!(plan.may_call_bot_api);
    }

    #[test]
    fn drain_once_preflight_reports_disabled_without_side_effects() {
        let gates = gateway_gates(false);
        let plan = plan_telegram_drain_once_preflight(NativeTelegramDrainOncePreflightInput {
            requested: false,
            gates: &gates,
        });

        assert_eq!(plan.status, "disabled");
        assert!(plan.error.is_none());
        assert!(!plan.status_probe_executes_pipeline);
        assert!(!plan.cursor_plan.duplicate_suppression_ready);
        assert_eq!(plan.model_turn_plan.candidate_count, 0);
        assert!(!plan.invocation_request.request_builder_ready);
        assert_eq!(plan.model_execution.status, "disabled");
        assert!(!plan.send_plan.send_plan_ready);
        assert!(!plan.send_request.request_builder_ready);
        assert_eq!(plan.send_execution.status, "disabled");
    }

    #[test]
    fn drain_once_preflight_stops_at_first_missing_gate() {
        let gates = gateway_gates(false);
        let plan = plan_telegram_drain_once_preflight(NativeTelegramDrainOncePreflightInput {
            requested: true,
            gates: &gates,
        });

        assert_eq!(plan.status, "gated");
        assert_eq!(
            plan.execution_plan.first_missing_gate,
            Some(DELIVERY_APPROVED_ENV)
        );
        assert!(!plan.execution_plan.all_required_gates_enabled);
        assert!(!plan.status_probe_executes_pipeline);
        assert!(plan.cursor_plan.duplicate_suppression_ready);
        assert!(plan.model_turn_plan.planner_ready);
        assert!(plan.invocation_request.request_builder_ready);
        assert!(!plan.invocation_request.model_turn_gate_enabled);
        assert_eq!(plan.model_execution.status, "gated");
        assert!(plan.send_plan.send_plan_ready);
        assert!(plan.send_request.request_builder_ready);
        assert!(!plan.send_request.send_gate_enabled);
        assert_eq!(plan.send_execution.status, "gated");
        assert!(
            plan.error
                .as_deref()
                .unwrap_or_default()
                .contains(DELIVERY_APPROVED_ENV)
        );
    }

    #[test]
    fn drain_once_preflight_allows_pipeline_only_when_all_gates_are_enabled() {
        let gates = gateway_gates(true);
        let plan = plan_telegram_drain_once_preflight(NativeTelegramDrainOncePreflightInput {
            requested: true,
            gates: &gates,
        });

        assert_eq!(plan.status, "planned");
        assert!(plan.error.is_none());
        assert!(plan.execution_plan.all_required_gates_enabled);
        assert!(plan.status_probe_executes_pipeline);
        assert!(plan.cursor_plan.commit_offset_after_delivery);
        assert!(plan.model_turn_plan.planner_ready);
        assert!(plan.invocation_request.request_builder_ready);
        assert!(!plan.invocation_request.candidate_present);
        assert_eq!(plan.model_execution.status, "waiting_candidate");
        assert!(plan.send_plan.send_plan_ready);
        assert!(plan.send_request.request_builder_ready);
        assert!(!plan.send_request.model_output_present);
        assert_eq!(plan.send_execution.status, "waiting_model_output");
        assert!(!plan.send_execution.external_send);
        assert!(!plan.send_execution.cursor_written);
    }

    #[test]
    fn drain_once_shell_readiness_blocks_cursor_parse_error() {
        let plan =
            plan_telegram_drain_once_shell_readiness(NativeTelegramDrainOnceShellReadinessInput {
                cursor_file_present: true,
                cursor_parse_ok: false,
                cursor_error: Some("cursor JSON is malformed"),
                config_ready: true,
                token_error: None,
            });

        assert_eq!(plan.status, "attention");
        assert!(!plan.may_call_bot_api);
        assert_eq!(plan.error.as_deref(), Some("cursor JSON is malformed"));
    }

    #[test]
    fn drain_once_shell_readiness_blocks_bad_config_before_token_read() {
        let plan =
            plan_telegram_drain_once_shell_readiness(NativeTelegramDrainOnceShellReadinessInput {
                cursor_file_present: false,
                cursor_parse_ok: true,
                cursor_error: None,
                config_ready: false,
                token_error: None,
            });

        assert_eq!(plan.status, "attention");
        assert!(!plan.may_call_bot_api);
        assert_eq!(
            plan.error.as_deref(),
            Some("Telegram config, token shape, or binding is not ready")
        );
    }

    #[test]
    fn drain_once_shell_readiness_redacts_token_error() {
        let plan =
            plan_telegram_drain_once_shell_readiness(NativeTelegramDrainOnceShellReadinessInput {
                cursor_file_present: false,
                cursor_parse_ok: true,
                cursor_error: None,
                config_ready: true,
                token_error: Some("bad token 123456789:abcdefghijklmnopqrstuvwxyz"),
            });

        assert_eq!(plan.status, "attention");
        assert!(!plan.may_call_bot_api);
        let error = plan.error.expect("error");
        assert!(error.contains("[redacted-telegram-token]"));
        assert!(!error.contains("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn drain_once_shell_readiness_allows_bot_api_after_ready_shell() {
        let plan =
            plan_telegram_drain_once_shell_readiness(NativeTelegramDrainOnceShellReadinessInput {
                cursor_file_present: true,
                cursor_parse_ok: true,
                cursor_error: None,
                config_ready: true,
                token_error: None,
            });

        assert_eq!(plan.status, "planned");
        assert!(plan.error.is_none());
        assert!(plan.may_call_bot_api);
    }

    #[test]
    fn drain_once_api_result_plan_allows_pipeline_on_ok_response() {
        let gates = gateway_gates(true);
        let api = serde_json::json!({
            "ok": true,
            "result": [{
                "update_id": 51,
                "message": {
                    "message_id": 11,
                    "text": "private drain prompt",
                    "chat": { "id": 6476198178_i64, "type": "private" },
                    "from": { "id": 6476198178_i64, "username": "private_user" }
                }
            }]
        });

        let plan = plan_telegram_drain_once_api_result(NativeTelegramDrainOnceApiResultInput {
            requested: true,
            gates: &gates,
            next_update_offset: Some(51),
            api_result: Ok(&api),
        });

        assert_eq!(plan.status, "planned");
        assert!(plan.error.is_none());
        assert!(plan.should_execute_pipeline);
        assert_eq!(plan.bot_api_ok, Some(true));
        assert_eq!(plan.local_next_update_offset, Some(52));
        assert_eq!(plan.inspection.allowed_update_count, 1);
        assert_eq!(plan.model_turn_plan.text_candidate_count, 1);
        assert!(plan.invocation_request.candidate_present);
        assert!(plan.invocation_request.runner_invocation_allowed);
    }

    #[test]
    fn drain_once_api_result_plan_redacts_ok_false_without_pipeline() {
        let gates = gateway_gates(true);
        let api = serde_json::json!({
            "ok": false,
            "description": "Unauthorized 123456789:abcdefghijklmnopqrstuvwxyz token rejected"
        });

        let plan = plan_telegram_drain_once_api_result(NativeTelegramDrainOnceApiResultInput {
            requested: true,
            gates: &gates,
            next_update_offset: Some(7),
            api_result: Ok(&api),
        });

        assert_eq!(plan.status, "attention");
        assert!(!plan.should_execute_pipeline);
        assert_eq!(plan.bot_api_ok, Some(false));
        assert_eq!(
            plan.error.as_deref(),
            Some("Unauthorized [redacted-telegram-token] token rejected")
        );
        assert_eq!(plan.inspection.update_count, 0);
        assert!(!plan.invocation_request.candidate_present);
    }

    #[test]
    fn drain_once_api_result_plan_classifies_conflict_as_busy() {
        let gates = gateway_gates(true);
        let plan = plan_telegram_drain_once_api_result(NativeTelegramDrainOnceApiResultInput {
            requested: true,
            gates: &gates,
            next_update_offset: Some(9),
            api_result: Err(
                "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request",
            ),
        });

        assert_eq!(plan.status, "busy");
        assert!(!plan.should_execute_pipeline);
        assert_eq!(plan.bot_api_ok, None);
        assert_eq!(plan.local_next_update_offset, None);
        assert_eq!(plan.inspection.update_count, 0);
        assert!(plan.error.as_deref().unwrap_or_default().contains("409"));
    }

    #[test]
    fn receive_once_api_result_builder_classifies_ok_response_without_raw_payloads() {
        let api = serde_json::json!({
            "ok": true,
            "result": [{
                "update_id": 41,
                "message": {
                    "message_id": 9,
                    "text": "private user prompt",
                    "chat": { "id": 6476198178_i64, "type": "private" },
                    "from": { "id": 6476198178_i64, "username": "private_user" }
                }
            }]
        });

        let report = build_telegram_receive_once_status_from_api_result(
            NativeTelegramReceiveOnceApiResultInput {
                requested: true,
                live_read_gate_env: LIVE_READ_ENV,
                live_read_gate_enabled: true,
                external_network_read: true,
                limit: 20,
                config: ready_config(),
                transport_plan: NativeTelegramTransportPlan::for_config_state(true, true, true),
                cursor_plan: NativeTelegramCursorPlan::ready(),
                get_updates_offset: Some(40),
                api_result: Ok(&api),
            },
        );

        assert_eq!(report.status, "ready");
        assert_eq!(report.bot_api_ok, Some(true));
        assert_eq!(report.get_updates_offset, Some(40));
        assert_eq!(report.local_next_update_offset, Some(42));
        assert_eq!(report.inspection.allowed_update_count, 1);
        assert_eq!(report.model_turn_plan.text_candidate_count, 1);
        assert!(!report.raw_update_payload_exposed);
        assert!(!report.raw_token_exposed);
        assert!(report.error.is_none());
    }

    #[test]
    fn receive_once_api_result_builder_redacts_ok_false_description() {
        let api = serde_json::json!({
            "ok": false,
            "description": "Unauthorized 123456789:abcdefghijklmnopqrstuvwxyz token rejected"
        });

        let report = build_telegram_receive_once_status_from_api_result(
            NativeTelegramReceiveOnceApiResultInput {
                requested: true,
                live_read_gate_env: LIVE_READ_ENV,
                live_read_gate_enabled: true,
                external_network_read: true,
                limit: 1,
                config: ready_config(),
                transport_plan: NativeTelegramTransportPlan::for_config_state(true, true, true),
                cursor_plan: NativeTelegramCursorPlan::ready(),
                get_updates_offset: Some(7),
                api_result: Ok(&api),
            },
        );

        assert_eq!(report.status, "attention");
        assert_eq!(report.bot_api_ok, Some(false));
        assert_eq!(report.get_updates_offset, Some(7));
        assert_eq!(
            report.error.as_deref(),
            Some("Unauthorized [redacted-telegram-token] token rejected")
        );
        assert!(!report.raw_token_exposed);
    }

    #[test]
    fn receive_once_api_result_builder_classifies_conflict_as_busy() {
        let report = build_telegram_receive_once_status_from_api_result(
            NativeTelegramReceiveOnceApiResultInput {
                requested: true,
                live_read_gate_env: LIVE_READ_ENV,
                live_read_gate_enabled: true,
                external_network_read: true,
                limit: 20,
                config: ready_config(),
                transport_plan: NativeTelegramTransportPlan::for_config_state(true, true, true),
                cursor_plan: NativeTelegramCursorPlan::ready(),
                get_updates_offset: Some(9),
                api_result: Err(
                    "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request",
                ),
            },
        );

        assert_eq!(report.status, "busy");
        assert_eq!(report.get_updates_offset, Some(9));
        assert_eq!(report.inspection.update_count, 0);
        assert!(report.error.as_deref().unwrap_or_default().contains("409"));
        assert!(!report.external_send);
        assert!(!report.cursor_written);
    }

    #[test]
    fn production_readiness_requires_minimum_soak_observations() {
        let poll_loop = ready_poll_loop_status();
        let cursor = ready_cursor_status();
        let delivery_ledger = ready_delivery_ledger_status();
        let guards = ready_production_guards();
        let observation = live_soak_observation(2, 0, Some("planned"), Some(true));

        let readiness = production_readiness_status_from_test_parts(
            true,
            &poll_loop,
            &cursor,
            &delivery_ledger,
            &guards,
            &observation,
        );

        assert!(!readiness.ready);
        assert_eq!(readiness.status, "warming");
        assert_eq!(
            readiness.min_poll_iterations,
            DEFAULT_TELEGRAM_SOAK_MIN_POLLS
        );
        assert!(!readiness.observation_ready);
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
        let delivery_ledger = ready_delivery_ledger_status();
        let guards = ready_production_guards();
        let observation = live_soak_observation(3, 0, Some("planned"), Some(true));

        let readiness = production_readiness_status_from_test_parts(
            true,
            &poll_loop,
            &cursor,
            &delivery_ledger,
            &guards,
            &observation,
        );

        assert!(readiness.ready);
        assert_eq!(readiness.status, "ready");
        assert!(readiness.poll_loop_armed);
        assert!(readiness.cursor_ready);
        assert!(readiness.production_guards_ready);
        assert!(readiness.observation_ready);
        assert!(readiness.observation_fresh);
        assert!(readiness.durable_cursor_evidence_present);
        assert!(!readiness.durable_delivery_evidence_required);
        assert!(readiness.durable_delivery_evidence_fresh);
        assert!(readiness.delivery_ledger_ready);
        assert!(readiness.attention_budget_ok);
        assert!(readiness.recent_bot_api_ok);
        assert!(readiness.redaction_guards_ok);
        assert!(readiness.readiness_blockers.is_empty());
        assert!(readiness.readiness_warnings.is_empty());
    }

    #[test]
    fn production_readiness_warns_when_no_durable_delivery_evidence_exists() {
        let poll_loop = ready_poll_loop_status();
        let mut cursor = ready_cursor_status();
        cursor.durable_cursor_evidence_present = false;
        cursor.cursor_updated_at_unix_ms = None;
        cursor.last_delivered_next_update_offset = None;
        let mut delivery_ledger = ready_delivery_ledger_status();
        delivery_ledger.durable_delivery_evidence_present = false;
        delivery_ledger.status = "empty";
        delivery_ledger.acked_count = 0;
        delivery_ledger.latest_acked_created_unix_seconds = None;
        let guards = ready_production_guards();
        let observation = live_soak_observation(3, 0, Some("planned"), Some(true));

        let readiness = production_readiness_status_from_test_parts(
            true,
            &poll_loop,
            &cursor,
            &delivery_ledger,
            &guards,
            &observation,
        );

        assert!(readiness.ready);
        assert!(!readiness.durable_cursor_evidence_present);
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
        let delivery_ledger = ready_delivery_ledger_status();
        let guards = ready_production_guards();
        let observation = live_soak_observation(3, 1, Some("attention"), Some(false));

        let readiness = production_readiness_status_from_test_parts(
            true,
            &poll_loop,
            &cursor,
            &delivery_ledger,
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

    #[test]
    fn production_readiness_flags_stale_soak_observations() {
        let poll_loop = ready_poll_loop_status();
        let cursor = ready_cursor_status();
        let delivery_ledger = ready_delivery_ledger_status();
        let guards = ready_production_guards();
        let mut observation = live_soak_observation(3, 0, Some("planned"), Some(true));
        observation.last_observed_at_unix_ms = Some(1);

        let readiness = production_readiness_status_from_test_parts(
            true,
            &poll_loop,
            &cursor,
            &delivery_ledger,
            &guards,
            &observation,
        );

        assert!(!readiness.ready);
        assert_eq!(readiness.status, "attention");
        assert_eq!(readiness.max_observed_age_env, SOAK_MAX_OBSERVED_AGE_ENV);
        assert_eq!(
            readiness.max_observed_age_ms,
            DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS
        );
        assert!(!readiness.observation_fresh);
        assert!(readiness.readiness_blockers.contains(&"observation_stale"));
    }

    #[test]
    fn production_readiness_blocks_when_send_was_observed_without_delivery_ledger() {
        let poll_loop = ready_poll_loop_status();
        let cursor = ready_cursor_status();
        let mut delivery_ledger = ready_delivery_ledger_status();
        delivery_ledger.status = "empty";
        delivery_ledger.acked_count = 0;
        delivery_ledger.provider_message_id_present = false;
        delivery_ledger.durable_delivery_evidence_present = false;
        delivery_ledger.latest_acked_created_unix_seconds = None;
        let guards = ready_production_guards();
        let mut observation = live_soak_observation(3, 0, Some("drained"), Some(true));
        observation.drained_count = 1;
        observation.send_started_count = 1;
        observation.cursor_written_count = 1;
        observation.external_send_count = 1;
        observation.last_send_started = true;
        observation.last_cursor_written = true;
        observation.last_external_send = true;

        let readiness = production_readiness_status_from_test_parts(
            true,
            &poll_loop,
            &cursor,
            &delivery_ledger,
            &guards,
            &observation,
        );

        assert!(!readiness.ready);
        assert!(readiness.durable_delivery_evidence_required);
        assert!(!readiness.durable_delivery_evidence_present);
        assert!(!readiness.delivery_ledger_ready);
        assert!(
            readiness
                .readiness_blockers
                .contains(&"durable_delivery_evidence_missing")
        );
    }

    #[test]
    fn production_readiness_blocks_stale_delivery_ledger_after_send() {
        let poll_loop = ready_poll_loop_status();
        let cursor = ready_cursor_status();
        let mut delivery_ledger = ready_delivery_ledger_status();
        delivery_ledger.latest_acked_created_unix_seconds = Some(1);
        let guards = ready_production_guards();
        let mut observation = live_soak_observation(3, 0, Some("drained"), Some(true));
        observation.drained_count = 1;
        observation.send_started_count = 1;
        observation.cursor_written_count = 1;
        observation.external_send_count = 1;
        observation.last_drained_at_unix_ms = Some(TEST_NOW_MS);
        observation.last_send_started = true;
        observation.last_cursor_written = true;
        observation.last_external_send = true;

        let readiness = production_readiness_status_from_test_parts(
            true,
            &poll_loop,
            &cursor,
            &delivery_ledger,
            &guards,
            &observation,
        );

        assert!(!readiness.ready);
        assert!(readiness.durable_delivery_evidence_required);
        assert!(readiness.durable_delivery_evidence_present);
        assert!(!readiness.durable_delivery_evidence_fresh);
        assert!(!readiness.delivery_ledger_ready);
        assert!(
            readiness
                .readiness_blockers
                .contains(&"durable_delivery_evidence_stale")
        );
    }

    #[test]
    fn production_guard_builder_preserves_redaction_and_retry_contract() {
        let report =
            build_telegram_production_guard_status(NativeTelegramProductionGuardStatusInput {
                read_max_attempts_env: "READ_ATTEMPTS",
                read_max_attempts: 3,
                read_retry_backoff_env: "READ_BACKOFF",
                read_retry_backoff_ms: 500,
                typing_keepalive_env: "TYPING",
                typing_keepalive_enabled: true,
                typing_keepalive_interval_ms: 4_000,
                model_timeout_env: "MODEL_TIMEOUT",
                model_timeout_ms: 120_000,
                model_failure_fallback_env: "MODEL_FALLBACK",
                model_failure_fallback_enabled: true,
                send_min_interval_env: "SEND_MIN",
                send_min_interval_ms: 1_500,
                send_max_attempts_env: "SEND_ATTEMPTS",
                send_max_attempts: 2,
                send_retry_backoff_env: "SEND_BACKOFF",
                send_retry_backoff_ms: 700,
            });

        assert_eq!(report.read_max_attempts, 3);
        assert_eq!(report.send_max_attempts, 2);
        assert!(report.retry_transient_read_errors);
        assert!(report.retry_transient_send_errors);
        assert_eq!(
            report.rate_limit_scope,
            "in-process per chat id; reset on gateway restart"
        );
        assert!(!report.raw_token_exposed);
    }

    #[test]
    fn production_guard_policy_builder_clamps_raw_env_values() {
        let report = build_telegram_production_guard_status_from_policy(
            NativeTelegramProductionGuardPolicyInput {
                read_max_attempts_env: "READ_ATTEMPTS",
                read_max_attempts: Some(0),
                read_retry_backoff_env: "READ_BACKOFF",
                read_retry_backoff_ms: Some(999_999),
                typing_keepalive_env: "TYPING",
                typing_keepalive_enabled: true,
                typing_keepalive_interval_ms: Some(1),
                model_timeout_env: "MODEL_TIMEOUT",
                model_timeout_ms: Some(1),
                model_failure_fallback_env: "MODEL_FALLBACK",
                model_failure_fallback_enabled: true,
                send_min_interval_env: "SEND_MIN",
                send_min_interval_ms: Some(999_999),
                send_max_attempts_env: "SEND_ATTEMPTS",
                send_max_attempts: Some(0),
                send_retry_backoff_env: "SEND_BACKOFF",
                send_retry_backoff_ms: Some(999_999),
            },
        );

        assert_eq!(report.read_max_attempts, 1);
        assert_eq!(report.read_retry_backoff_ms, 30_000);
        assert_eq!(report.typing_keepalive_interval_ms, 1_000);
        assert_eq!(report.model_timeout_ms, 1_000);
        assert_eq!(report.send_min_interval_ms, 60_000);
        assert_eq!(report.send_max_attempts, 1);
        assert_eq!(report.send_retry_backoff_ms, 30_000);
        assert!(report.typing_keepalive_enabled);
        assert!(report.model_failure_fallback_enabled);
        assert!(!report.raw_token_exposed);
    }

    #[test]
    fn live_soak_observation_state_accumulates_redacted_report() {
        let gates = NativeTelegramGatewayGateSummary {
            delivery_approval_gate_env: "DELIVERY",
            delivery_approval_gate_enabled: false,
            live_read_gate_env: "READ",
            live_read_gate_enabled: false,
            model_turn_gate_env: "MODEL",
            model_turn_gate_enabled: false,
            send_gate_env: "SEND",
            send_gate_enabled: false,
            readiness_summary_performs_live_read: false,
            readiness_summary_invokes_model: false,
            readiness_summary_sends_message: false,
        };
        let invocation_request = NativeTelegramModelInvocationRequestPlan::disabled("MODEL", false);
        let send_request = NativeTelegramSendRequestPlan::disabled("SEND", false);
        let status = build_telegram_drain_once_status(NativeTelegramDrainOnceStatusInput {
            requested: true,
            status: "drained",
            gates: gates.clone(),
            config: ready_config(),
            execution_plan: crate::telegram_policy::telegram_drain_execution_plan(true, &gates),
            cursor_plan: NativeTelegramCursorPlan::ready(),
            inspection: crate::telegram_policy::inspect_telegram_updates(&[]),
            model_turn_plan: NativeTelegramModelTurnPlan::disabled(),
            invocation_request: invocation_request.clone(),
            model_execution: NativeTelegramModelExecutionReport::disabled("MODEL", false),
            send_plan: NativeTelegramSendPlan::ready(),
            send_request,
            send_execution: NativeTelegramSendExecutionReport::disabled("SEND", false),
            bot_api_ok: Some(true),
            local_next_update_offset: Some(42),
            get_updates_offset: Some(41),
            live_read_started: true,
            external_network_read: true,
            error: None,
        });
        let mut state = NativeTelegramLiveSoakObservationState::default();

        state.observe(&status, 123_456);
        let report = state.report();

        assert_eq!(report.poll_iterations, 1);
        assert_eq!(report.drained_count, 1);
        assert_eq!(report.last_drained_at_unix_ms, Some(123_456));
        assert_eq!(report.last_drained_next_update_offset, Some(42));
        assert_eq!(report.last_bot_api_ok, Some(true));
        assert_eq!(report.last_get_updates_offset, Some(41));
        assert!(!report.raw_update_payload_exposed);
        assert!(!report.raw_prompt_text_exposed);
        assert!(!report.raw_response_text_exposed);
        assert!(!report.raw_token_exposed);
    }
}
