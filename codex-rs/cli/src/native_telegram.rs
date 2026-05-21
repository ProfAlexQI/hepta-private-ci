use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, SystemTime};

use codex_arg0::Arg0DispatchPaths;
pub(crate) use hepta_gateway::{
    NativeTelegramConfigStatus, NativeTelegramConfigStatusInput, NativeTelegramCursorPlan,
    NativeTelegramCursorStatus, NativeTelegramDeliveryLedgerStatus,
    NativeTelegramDrainOnceApiResultInput, NativeTelegramDrainOncePreflightInput,
    NativeTelegramDrainOnceShellReadinessInput, NativeTelegramDrainOnceStatus,
    NativeTelegramDrainOnceStatusInput, NativeTelegramDrainPipelineInput,
    NativeTelegramGatewayGateSummary, NativeTelegramGatewayGateSummaryInput,
    NativeTelegramLiveSoakObservationReport, NativeTelegramLiveSoakObservationState,
    NativeTelegramLiveSoakStatus, NativeTelegramLiveSoakStatusInput,
    NativeTelegramModelBridgeStatus, NativeTelegramModelBridgeStatusInput,
    NativeTelegramModelRunnerPlan, NativeTelegramModelTurnPlanStatus,
    NativeTelegramModelTurnPlanStatusInput, NativeTelegramPluginStatus,
    NativeTelegramPluginStatusInput, NativeTelegramPollLoopStatus,
    NativeTelegramPollLoopStatusInput, NativeTelegramProductionGuardPolicyInput,
    NativeTelegramProductionGuardStatus, NativeTelegramProductionReadinessInput,
    NativeTelegramProductionReadinessStatus, NativeTelegramReceiveOnceApiResultInput,
    NativeTelegramReceiveOnceErrorInput, NativeTelegramReceiveOncePreflightInput,
    NativeTelegramReceiveOnceShellReadinessInput, NativeTelegramReceiveOnceStatus,
    NativeTelegramReplyTargetMaterial, NativeTelegramSendPlanStatus,
    NativeTelegramSendPlanStatusInput, NativeTelegramTokenObservationInput,
    TelegramTypingKeepalive,
};
use hepta_gateway::{
    TELEGRAM_ALLOWED_UPDATES, build_native_telegram_config_status,
    build_telegram_drain_once_status, build_telegram_gateway_gate_summary,
    build_telegram_live_soak_status, build_telegram_model_bridge_status,
    build_telegram_model_turn_plan_status, build_telegram_plugin_status,
    build_telegram_poll_loop_status, build_telegram_production_guard_status_from_policy,
    build_telegram_production_readiness_status, build_telegram_receive_once_error_status,
    build_telegram_receive_once_status_from_api_result, build_telegram_send_plan_status,
    execute_telegram_drain_pipeline_for_updates, extract_native_telegram_config_metadata,
    extract_native_telegram_exec_child_final_message,
    extract_native_telegram_openai_chat_completion_text, finalize_telegram_drain_pipeline_status,
    invoke_native_telegram_model_runner_with_plan, native_telegram_codex_core_prompt,
    native_telegram_exec_child_args, native_telegram_exec_child_status_error,
    native_telegram_mlx_chat_completion_body, native_telegram_model_failure_fallback_message,
    native_telegram_model_timeout, parse_telegram_env_truthy_value, parse_telegram_env_u64_value,
    plan_telegram_drain_once_api_result, plan_telegram_drain_once_preflight,
    plan_telegram_drain_once_shell_readiness, plan_telegram_receive_once_preflight_status,
    plan_telegram_receive_once_shell_readiness, resolve_native_telegram_token_observation,
    select_native_telegram_model_runner, telegram_bot_token_shape_ok as token_shape_ok,
    telegram_call_get_updates_once as gateway_telegram_call_get_updates_once,
    telegram_call_send_chat_action,
    telegram_call_send_message as gateway_telegram_call_send_message,
    telegram_cursor_status as gateway_telegram_cursor_status, telegram_cursor_status_from_path,
    telegram_get_updates_with_retry, telegram_poll_loop_interval_ms_policy,
    telegram_poll_loop_should_spawn, telegram_read_max_attempts_policy,
    telegram_read_retry_backoff_policy, telegram_receive_limit_policy,
    telegram_redact_token_like_text as redact_token_like_text, telegram_send_max_attempts_policy,
    telegram_send_min_interval_policy, telegram_send_retry_backoff_policy,
    telegram_soak_max_attention_count_policy, telegram_soak_max_observed_age_ms_policy,
    telegram_soak_min_poll_iterations_policy, telegram_start_typing_keepalive,
    telegram_system_time_unix_ms, telegram_transport_plan_for_config_status,
    telegram_typing_keepalive_interval_policy, telegram_wait_for_send_rate_limit,
    wait_for_native_telegram_model_child,
};
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use tokio::runtime::Handle;

const LEGACY_RUNTIME_SLUG: &str = "openclaw";
const LEGACY_CONFIG_FILE_NAME: &str = "openclaw.json";
const LOCAL_IMPORT_CONFIG_PATH: &str = ".hepta/local-import/private/config/openclaw.json";
const LOCAL_IMPORT_MANIFEST_PATH: &str = ".hepta/local-import/manifest.json";
const TELEGRAM_INGRESS_CURSOR_PATH: &str = ".hepta/telegram/ingress-drain-cursor.json";
const TELEGRAM_DELIVERY_LEDGER_PATH: &str = ".hepta/telegram/delivery-ledger.jsonl";
pub(crate) const TELEGRAM_LIVE_READ_ENV: &str = "HEPTA_NATIVE_TELEGRAM_LIVE_READ";
pub(crate) const TELEGRAM_MODEL_TURN_GATE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_TURN";
pub(crate) const TELEGRAM_SEND_GATE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND";
pub(crate) const TELEGRAM_POLL_LOOP_ENV: &str = "HEPTA_NATIVE_TELEGRAM_POLL_LOOP";
pub(crate) const TELEGRAM_DELIVERY_APPROVED_ENV: &str = "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED";
pub(crate) const TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_IN_PROCESS_MODEL_RUNNER";
pub(crate) const TELEGRAM_CODEX_CORE_RUNNER_ENV: &str = "HEPTA_NATIVE_TELEGRAM_CODEX_CORE_RUNNER";
const TELEGRAM_HEPTA_INTELLIGENCE_CONTEXT_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_HEPTA_INTELLIGENCE_CONTEXT";
const TELEGRAM_PLUGIN_CAPABILITY_CONTEXT_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_PLUGIN_CAPABILITY_CONTEXT";
const TELEGRAM_MODEL_TIMEOUT_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_TIMEOUT_MS";
const TELEGRAM_MODEL_ENV: &str = "HEPTA_TELEGRAM_MODEL";
const HEPTA_DEFAULT_MODEL_ENV: &str = "HEPTA_DEFAULT_MODEL";
const TELEGRAM_MLX_BASE_URL_ENV: &str = "HEPTA_MLX_OPENAI_BASE_URL";
const TELEGRAM_MLX_MAX_TOKENS_ENV: &str = "HEPTA_MLX_TELEGRAM_MAX_TOKENS";
const TELEGRAM_TYPING_KEEPALIVE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_TYPING_KEEPALIVE";
const TELEGRAM_TYPING_KEEPALIVE_INTERVAL_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS";
const TELEGRAM_READ_MAX_ATTEMPTS_ENV: &str = "HEPTA_NATIVE_TELEGRAM_READ_MAX_ATTEMPTS";
const TELEGRAM_READ_RETRY_BACKOFF_ENV: &str = "HEPTA_NATIVE_TELEGRAM_READ_RETRY_BACKOFF_MS";
const TELEGRAM_SEND_MIN_INTERVAL_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND_MIN_INTERVAL_MS";
const TELEGRAM_SEND_MAX_ATTEMPTS_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND_MAX_ATTEMPTS";
const TELEGRAM_SEND_RETRY_BACKOFF_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND_RETRY_BACKOFF_MS";
const TELEGRAM_MODEL_FAILURE_FALLBACK_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK";
const TELEGRAM_SOAK_MIN_POLLS_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS";
const TELEGRAM_SOAK_MAX_ATTENTION_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION";
const TELEGRAM_SOAK_MAX_OBSERVED_AGE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS";
static TELEGRAM_LIVE_SOAK_OBSERVATION: OnceLock<Mutex<NativeTelegramLiveSoakObservationState>> =
    OnceLock::new();

pub(crate) fn telegram_plugin_status(requested: bool, poll_ms: u64) -> NativeTelegramPluginStatus {
    let config = if requested {
        load_telegram_config_status()
    } else {
        NativeTelegramConfigStatus::disabled()
    };
    build_telegram_plugin_status(NativeTelegramPluginStatusInput {
        requested,
        poll_ms,
        allowed_updates: TELEGRAM_ALLOWED_UPDATES,
        config,
        gates: telegram_gateway_gate_summary(),
        poll_loop_gate_enabled: env_truthy(TELEGRAM_POLL_LOOP_ENV),
    })
}

pub(crate) fn telegram_receive_once_status(
    requested: bool,
    limit: usize,
) -> NativeTelegramReceiveOnceStatus {
    telegram_receive_once_status_with_gate(requested, limit, env_truthy(TELEGRAM_LIVE_READ_ENV))
}

pub(crate) fn telegram_gateway_gate_summary() -> NativeTelegramGatewayGateSummary {
    build_telegram_gateway_gate_summary(NativeTelegramGatewayGateSummaryInput {
        delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
        delivery_approval_gate_enabled: env_truthy(TELEGRAM_DELIVERY_APPROVED_ENV),
        live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
        live_read_gate_enabled: env_truthy(TELEGRAM_LIVE_READ_ENV),
        model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
        model_turn_gate_enabled: env_truthy(TELEGRAM_MODEL_TURN_GATE_ENV),
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        send_gate_enabled: env_truthy(TELEGRAM_SEND_GATE_ENV),
    })
}

pub(crate) fn telegram_model_turn_plan_status(
    requested: bool,
) -> NativeTelegramModelTurnPlanStatus {
    let config = if requested {
        load_telegram_config_status()
    } else {
        NativeTelegramConfigStatus::disabled()
    };
    build_telegram_model_turn_plan_status(NativeTelegramModelTurnPlanStatusInput {
        requested,
        config,
    })
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
    let model_runner_plan = telegram_model_runner_plan();
    build_telegram_model_bridge_status(NativeTelegramModelBridgeStatusInput {
        requested,
        config,
        model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
        model_turn_gate_enabled,
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        model_runner_plan: &model_runner_plan,
    })
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
    build_telegram_poll_loop_status(NativeTelegramPollLoopStatusInput {
        requested,
        poll_ms,
        poll_loop_gate_env: TELEGRAM_POLL_LOOP_ENV,
        poll_loop_gate_enabled: env_truthy(TELEGRAM_POLL_LOOP_ENV),
        delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
        delivery_approval_gate_enabled: env_truthy(TELEGRAM_DELIVERY_APPROVED_ENV),
        live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
        model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
    })
}

pub(crate) fn telegram_live_soak_status(
    requested: bool,
    poll_ms: u64,
) -> NativeTelegramLiveSoakStatus {
    let poll_loop_status = telegram_poll_loop_status(requested, poll_ms);
    let cursor_status = telegram_cursor_status(requested);
    let delivery_ledger_status = telegram_delivery_ledger_status(requested);
    let observation = telegram_live_soak_observation_report();
    let production_guards = telegram_production_guard_status();
    let production_readiness = telegram_production_readiness_status_from_parts(
        requested,
        &poll_loop_status,
        &cursor_status,
        &delivery_ledger_status,
        &production_guards,
        &observation,
    );
    build_telegram_live_soak_status(NativeTelegramLiveSoakStatusInput {
        requested,
        poll_loop_status,
        cursor_status,
        delivery_ledger_status,
        production_guards,
        production_readiness,
        observation,
    })
}

pub(crate) fn telegram_production_readiness_status(
    requested: bool,
    poll_ms: u64,
) -> NativeTelegramProductionReadinessStatus {
    let poll_loop_status = telegram_poll_loop_status(requested, poll_ms);
    let cursor_status = telegram_cursor_status(requested);
    let delivery_ledger_status = telegram_delivery_ledger_status(requested);
    let production_guards = telegram_production_guard_status();
    let observation = telegram_live_soak_observation_report();
    telegram_production_readiness_status_from_parts(
        requested,
        &poll_loop_status,
        &cursor_status,
        &delivery_ledger_status,
        &production_guards,
        &observation,
    )
}

fn telegram_production_readiness_status_from_parts(
    requested: bool,
    poll_loop_status: &NativeTelegramPollLoopStatus,
    cursor_status: &NativeTelegramCursorStatus,
    delivery_ledger_status: &NativeTelegramDeliveryLedgerStatus,
    production_guards: &NativeTelegramProductionGuardStatus,
    observation: &NativeTelegramLiveSoakObservationReport,
) -> NativeTelegramProductionReadinessStatus {
    build_telegram_production_readiness_status(NativeTelegramProductionReadinessInput {
        requested,
        min_poll_iterations_env: TELEGRAM_SOAK_MIN_POLLS_ENV,
        min_poll_iterations: telegram_soak_min_poll_iterations(),
        max_attention_count_env: TELEGRAM_SOAK_MAX_ATTENTION_ENV,
        max_attention_count: telegram_soak_max_attention_count(),
        max_observed_age_env: TELEGRAM_SOAK_MAX_OBSERVED_AGE_ENV,
        max_observed_age_ms: telegram_soak_max_observed_age_ms(),
        poll_loop_status,
        cursor_status,
        delivery_ledger_status,
        production_guards,
        observation,
        now_unix_ms: now_unix_ms(),
    })
}

pub(crate) fn spawn_telegram_poll_loop_if_enabled(
    requested: bool,
    poll_ms: u64,
) -> Option<thread::JoinHandle<()>> {
    if !telegram_poll_loop_should_spawn(
        requested,
        env_truthy(TELEGRAM_POLL_LOOP_ENV),
        env_truthy(TELEGRAM_DELIVERY_APPROVED_ENV),
    ) {
        return None;
    }

    Some(thread::spawn(move || {
        run_telegram_poll_loop(requested, poll_ms)
    }))
}

fn run_telegram_poll_loop(requested: bool, poll_ms: u64) {
    let poll_ms = telegram_poll_loop_interval_ms_policy(poll_ms);
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
    gateway_telegram_cursor_status(requested, Path::new(TELEGRAM_INGRESS_CURSOR_PATH))
}

pub(crate) fn telegram_delivery_ledger_status(
    requested: bool,
) -> NativeTelegramDeliveryLedgerStatus {
    hepta_gateway::telegram_delivery_ledger_status(
        requested,
        Path::new(TELEGRAM_DELIVERY_LEDGER_PATH),
        TELEGRAM_DELIVERY_LEDGER_PATH,
    )
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
    let preflight = plan_telegram_drain_once_preflight(NativeTelegramDrainOncePreflightInput {
        requested,
        gates: &gates,
    });
    let cursor_plan = preflight.cursor_plan;
    let mut inspection = preflight.inspection;
    let mut model_turn_plan = preflight.model_turn_plan;
    let mut invocation_request = preflight.invocation_request;
    let send_plan = preflight.send_plan;
    let mut send_request = preflight.send_request;
    let mut send_execution = preflight.send_execution;
    let mut model_execution = preflight.model_execution;
    let execution_plan = preflight.execution_plan;
    let status_probe_executes_pipeline = preflight.status_probe_executes_pipeline;
    let mut status = preflight.status;
    let mut error = preflight.error;
    let mut bot_api_ok = None;
    let mut local_next_update_offset = None;
    let mut get_updates_offset = None;
    let mut live_read_started = false;
    let mut external_network_read = false;

    if status_probe_executes_pipeline {
        let cursor_status =
            telegram_cursor_status_from_path(Path::new(TELEGRAM_INGRESS_CURSOR_PATH));
        get_updates_offset = cursor_status.next_update_offset;
        let shell_readiness =
            plan_telegram_drain_once_shell_readiness(NativeTelegramDrainOnceShellReadinessInput {
                cursor_file_present: cursor_status.cursor_file_present,
                cursor_parse_ok: cursor_status.cursor_parse_ok,
                cursor_error: cursor_status.error.as_deref(),
                config_ready: config.config_ready(),
                token_error: None,
            });
        if !shell_readiness.may_call_bot_api {
            status = shell_readiness.status;
            error = shell_readiness.error;
        } else {
            match load_effective_telegram_token() {
                Ok(token) => {
                    live_read_started = true;
                    external_network_read = true;
                    match call_telegram_get_updates(&token, 20, get_updates_offset) {
                        Ok(api) => {
                            let updates = api
                                .get("result")
                                .and_then(Value::as_array)
                                .cloned()
                                .unwrap_or_default();
                            let fetch_plan = plan_telegram_drain_once_api_result(
                                NativeTelegramDrainOnceApiResultInput {
                                    requested,
                                    gates: &gates,
                                    next_update_offset: cursor_status.next_update_offset,
                                    api_result: Ok(&api),
                                },
                            );
                            bot_api_ok = fetch_plan.bot_api_ok;
                            inspection = fetch_plan.inspection;
                            local_next_update_offset = fetch_plan.local_next_update_offset;
                            model_turn_plan = fetch_plan.model_turn_plan;
                            invocation_request = fetch_plan.invocation_request;
                            status = fetch_plan.status;
                            error = fetch_plan.error;
                            if fetch_plan.should_execute_pipeline {
                                let model_runner_plan = telegram_model_runner_plan();
                                let pipeline = execute_telegram_drain_pipeline_for_updates(
                                    NativeTelegramDrainPipelineInput {
                                        updates: &updates,
                                        next_update_offset: cursor_status.next_update_offset,
                                        token: Some(token.as_str()),
                                        gates: &gates,
                                        cursor_path: Path::new(TELEGRAM_INGRESS_CURSOR_PATH),
                                        delivery_ledger_path: Path::new(
                                            TELEGRAM_DELIVERY_LEDGER_PATH,
                                        ),
                                        model_failure_fallback_enabled:
                                            telegram_model_failure_fallback_enabled(),
                                        model_failure_fallback_message:
                                            native_telegram_model_failure_fallback_message(),
                                        send_max_attempts: telegram_send_max_attempts(),
                                        send_retry_backoff: telegram_send_retry_backoff(),
                                    },
                                    |reply_target, prompt| {
                                        run_model_with_optional_typing_keepalive(
                                            Some(token.as_str()),
                                            reply_target,
                                            prompt,
                                            |prompt| {
                                                run_hepta_model_turn_with_plan(
                                                    prompt,
                                                    &model_runner_plan,
                                                )
                                            },
                                        )
                                    },
                                    call_telegram_send_message,
                                );
                                let final_status = finalize_telegram_drain_pipeline_status(
                                    pipeline,
                                    model_runner_plan.process_spawned_by_status,
                                    status,
                                    error,
                                );
                                status = final_status.status;
                                error = final_status.error;
                                invocation_request = final_status.outcome.invocation_request;
                                model_execution = final_status.outcome.model_execution;
                                send_request = final_status.outcome.send_request;
                                send_execution = final_status.outcome.send_execution;
                            }
                        }
                        Err(fetch_error) => {
                            let fetch_plan = plan_telegram_drain_once_api_result(
                                NativeTelegramDrainOnceApiResultInput {
                                    requested,
                                    gates: &gates,
                                    next_update_offset: cursor_status.next_update_offset,
                                    api_result: Err(&fetch_error),
                                },
                            );
                            bot_api_ok = fetch_plan.bot_api_ok;
                            inspection = fetch_plan.inspection;
                            local_next_update_offset = fetch_plan.local_next_update_offset;
                            model_turn_plan = fetch_plan.model_turn_plan;
                            invocation_request = fetch_plan.invocation_request;
                            status = fetch_plan.status;
                            error = fetch_plan.error;
                        }
                    }
                }
                Err(token_error) => {
                    let token_readiness = plan_telegram_drain_once_shell_readiness(
                        NativeTelegramDrainOnceShellReadinessInput {
                            cursor_file_present: cursor_status.cursor_file_present,
                            cursor_parse_ok: cursor_status.cursor_parse_ok,
                            cursor_error: cursor_status.error.as_deref(),
                            config_ready: config.config_ready(),
                            token_error: Some(&token_error),
                        },
                    );
                    status = token_readiness.status;
                    error = token_readiness.error;
                }
            }
        }
    }

    build_telegram_drain_once_status(NativeTelegramDrainOnceStatusInput {
        requested,
        status,
        gates,
        config,
        execution_plan,
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
        external_network_read,
        error,
    })
}

fn observe_telegram_live_soak(status: &NativeTelegramDrainOnceStatus) {
    let map = TELEGRAM_LIVE_SOAK_OBSERVATION
        .get_or_init(|| Mutex::new(NativeTelegramLiveSoakObservationState::default()));
    let mut guard = match map.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.observe(status, now_unix_ms());
}

fn telegram_live_soak_observation_report() -> NativeTelegramLiveSoakObservationReport {
    let map = TELEGRAM_LIVE_SOAK_OBSERVATION
        .get_or_init(|| Mutex::new(NativeTelegramLiveSoakObservationState::default()));
    match map.lock() {
        Ok(guard) => guard.report(),
        Err(poisoned) => poisoned.into_inner().report(),
    }
}

fn telegram_production_guard_status() -> NativeTelegramProductionGuardStatus {
    build_telegram_production_guard_status_from_policy(NativeTelegramProductionGuardPolicyInput {
        read_max_attempts_env: TELEGRAM_READ_MAX_ATTEMPTS_ENV,
        read_max_attempts: env_u64(TELEGRAM_READ_MAX_ATTEMPTS_ENV),
        read_retry_backoff_env: TELEGRAM_READ_RETRY_BACKOFF_ENV,
        read_retry_backoff_ms: env_u64(TELEGRAM_READ_RETRY_BACKOFF_ENV),
        typing_keepalive_env: TELEGRAM_TYPING_KEEPALIVE_ENV,
        typing_keepalive_enabled: telegram_typing_keepalive_enabled(),
        typing_keepalive_interval_ms: env_u64(TELEGRAM_TYPING_KEEPALIVE_INTERVAL_ENV),
        model_timeout_env: TELEGRAM_MODEL_TIMEOUT_ENV,
        model_timeout_ms: env_u64(TELEGRAM_MODEL_TIMEOUT_ENV),
        model_failure_fallback_env: TELEGRAM_MODEL_FAILURE_FALLBACK_ENV,
        model_failure_fallback_enabled: telegram_model_failure_fallback_enabled(),
        send_min_interval_env: TELEGRAM_SEND_MIN_INTERVAL_ENV,
        send_min_interval_ms: env_u64(TELEGRAM_SEND_MIN_INTERVAL_ENV),
        send_max_attempts_env: TELEGRAM_SEND_MAX_ATTEMPTS_ENV,
        send_max_attempts: env_u64(TELEGRAM_SEND_MAX_ATTEMPTS_ENV),
        send_retry_backoff_env: TELEGRAM_SEND_RETRY_BACKOFF_ENV,
        send_retry_backoff_ms: env_u64(TELEGRAM_SEND_RETRY_BACKOFF_ENV),
    })
}

fn telegram_soak_min_poll_iterations() -> u64 {
    telegram_soak_min_poll_iterations_policy(env_u64(TELEGRAM_SOAK_MIN_POLLS_ENV))
}

fn telegram_soak_max_attention_count() -> u64 {
    telegram_soak_max_attention_count_policy(env_u64(TELEGRAM_SOAK_MAX_ATTENTION_ENV))
}

fn telegram_soak_max_observed_age_ms() -> u64 {
    telegram_soak_max_observed_age_ms_policy(env_u64(TELEGRAM_SOAK_MAX_OBSERVED_AGE_ENV))
}

fn now_unix_ms() -> u64 {
    telegram_system_time_unix_ms(SystemTime::now())
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
    build_telegram_send_plan_status(NativeTelegramSendPlanStatusInput {
        requested,
        config,
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        send_gate_enabled,
    })
}

fn telegram_receive_once_status_with_gate(
    requested: bool,
    limit: usize,
    live_read_gate_enabled: bool,
) -> NativeTelegramReceiveOnceStatus {
    let limit = telegram_receive_limit_policy(limit);
    let config = load_telegram_config_status();
    let transport_plan = telegram_transport_plan_for_config_status(&config);
    let cursor_plan = NativeTelegramCursorPlan::ready();

    if let Some(report) =
        plan_telegram_receive_once_preflight_status(NativeTelegramReceiveOncePreflightInput {
            requested,
            live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
            live_read_gate_enabled,
            limit,
            config: &config,
            transport_plan: &transport_plan,
            cursor_plan: &cursor_plan,
        })
    {
        return report;
    }

    let token = match load_effective_telegram_token() {
        Ok(token) => token,
        Err(error) => {
            let shell_readiness = plan_telegram_receive_once_shell_readiness(
                NativeTelegramReceiveOnceShellReadinessInput {
                    token_error: Some(&error),
                    cursor_file_present: false,
                    cursor_parse_ok: true,
                    cursor_error: None,
                },
            );
            return build_telegram_receive_once_error_status(NativeTelegramReceiveOnceErrorInput {
                requested,
                live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
                live_read_gate_enabled: true,
                limit,
                config,
                transport_plan,
                cursor_plan,
                get_updates_offset: None,
                error: shell_readiness.error,
            });
        }
    };

    let cursor_status = telegram_cursor_status_from_path(Path::new(TELEGRAM_INGRESS_CURSOR_PATH));
    let get_updates_offset = cursor_status.next_update_offset;
    let shell_readiness =
        plan_telegram_receive_once_shell_readiness(NativeTelegramReceiveOnceShellReadinessInput {
            token_error: None,
            cursor_file_present: cursor_status.cursor_file_present,
            cursor_parse_ok: cursor_status.cursor_parse_ok,
            cursor_error: cursor_status.error.as_deref(),
        });
    if !shell_readiness.may_call_bot_api {
        return build_telegram_receive_once_error_status(NativeTelegramReceiveOnceErrorInput {
            requested,
            live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
            live_read_gate_enabled: true,
            limit,
            config,
            transport_plan,
            cursor_plan,
            get_updates_offset,
            error: shell_readiness.error,
        });
    }

    match call_telegram_get_updates(&token, limit, get_updates_offset) {
        Ok(api) => build_telegram_receive_once_status_from_api_result(
            NativeTelegramReceiveOnceApiResultInput {
                requested,
                live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
                live_read_gate_enabled: true,
                external_network_read: true,
                limit,
                config,
                transport_plan,
                cursor_plan,
                get_updates_offset,
                api_result: Ok(&api),
            },
        ),
        Err(error) => build_telegram_receive_once_status_from_api_result(
            NativeTelegramReceiveOnceApiResultInput {
                requested,
                live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
                live_read_gate_enabled: true,
                external_network_read: true,
                limit,
                config,
                transport_plan,
                cursor_plan,
                get_updates_offset,
                api_result: Err(&error),
            },
        ),
    }
}

fn load_telegram_config_status() -> NativeTelegramConfigStatus {
    let Some(config_path) = resolve_private_hepta_runtime_config_path() else {
        return NativeTelegramConfigStatus::missing(
            "Hepta private Telegram config not found".to_string(),
        );
    };

    match load_telegram_config_status_from_path(&config_path) {
        Ok(status) => status,
        Err(error) => NativeTelegramConfigStatus::error(
            Some(config_path.display().to_string()),
            config_path.is_file(),
            redact_token_like_text(&error),
        ),
    }
}

fn load_telegram_config_status_from_path(
    path: &Path,
) -> Result<NativeTelegramConfigStatus, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("failed to read Hepta private Telegram config: {error}"))?;
    let config: Value = serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse Hepta private Telegram config: {error}"))?;
    let metadata = extract_native_telegram_config_metadata(path, &config)?;
    let telegram = config
        .pointer("/channels/telegram")
        .ok_or_else(|| "channels.telegram config is missing".to_string())?;
    let bot_token_ref = telegram.get("botToken");
    let token_file_present = metadata
        .token_secret_path
        .as_ref()
        .map(|path| path.is_file())
        .unwrap_or(false);
    let token_file_mode_0600 = metadata
        .token_secret_path
        .as_ref()
        .map(file_mode_is_0600)
        .unwrap_or(false);
    let inline_token = bot_token_ref
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let env_token = env::var("HEPTA_TELEGRAM_BOT_TOKEN")
        .ok()
        .or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let file_token = metadata
        .token_secret_path
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let token_observation =
        resolve_native_telegram_token_observation(NativeTelegramTokenObservationInput {
            env_token_present: env_token.is_some(),
            env_token_shape_ok: env_token.as_deref().map(token_shape_ok).unwrap_or(false),
            file_token_present: file_token.is_some(),
            file_token_shape_ok: file_token.as_deref().map(token_shape_ok).unwrap_or(false),
            inline_token_present: metadata.inline_token_present,
            inline_token_shape_ok: inline_token.as_deref().map(token_shape_ok).unwrap_or(false),
            token_secret_ref_present: metadata.token_secret_ref_present,
        });
    Ok(build_native_telegram_config_status(
        NativeTelegramConfigStatusInput {
            config_path: Some(path.display().to_string()),
            config_found: true,
            enabled: metadata.enabled,
            dm_policy: metadata.dm_policy,
            group_policy: metadata.group_policy,
            allow_from_count: metadata.allow_from_count,
            group_count: metadata.group_count,
            token_source: token_observation.token_source,
            token_secret_ref_present: metadata.token_secret_ref_present,
            token_secret_provider: metadata.token_secret_provider,
            token_secret_id_present: metadata.token_secret_id_present,
            token_file_present,
            token_file_mode_0600,
            token_shape_ok: token_observation.token_shape_ok,
            error: None,
        },
    ))
}

fn load_effective_telegram_token() -> Result<String, String> {
    let config_path = resolve_private_hepta_runtime_config_path()
        .ok_or_else(|| "Hepta private Telegram config not found".to_string())?;
    let raw = fs::read_to_string(&config_path)
        .map_err(|error| format!("failed to read Hepta private Telegram config: {error}"))?;
    let config: Value = serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse Hepta private Telegram config: {error}"))?;
    let metadata = extract_native_telegram_config_metadata(&config_path, &config)?;
    let telegram = config
        .pointer("/channels/telegram")
        .ok_or_else(|| "channels.telegram config is missing".to_string())?;
    let bot_token_ref = telegram.get("botToken");
    let inline_token = bot_token_ref
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let env_token = env::var("HEPTA_TELEGRAM_BOT_TOKEN")
        .ok()
        .or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let file_token = metadata
        .token_secret_path
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

pub(crate) fn effective_telegram_token_fingerprint() -> Option<String> {
    load_effective_telegram_token()
        .ok()
        .and_then(|token| redacted_telegram_token_fingerprint(&token))
}

pub(crate) fn redacted_telegram_token_fingerprint(token: &str) -> Option<String> {
    let token = token.trim();
    if !token_shape_ok(token) {
        return None;
    }
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let digest = hasher.finalize();
    let mut encoded = String::with_capacity("sha256:".len() + 16);
    encoded.push_str("sha256:");
    for byte in digest.iter().take(8) {
        encoded.push_str(&format!("{byte:02x}"));
    }
    Some(encoded)
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
    telegram_get_updates_with_retry(
        telegram_read_max_attempts(),
        telegram_read_retry_backoff(),
        || call_telegram_get_updates_once(token, limit, offset),
    )
}

fn call_telegram_get_updates_once(
    token: &str,
    limit: usize,
    offset: Option<i64>,
) -> Result<Value, String> {
    gateway_telegram_call_get_updates_once(token, limit, offset, Duration::from_secs(15))
}

#[allow(dead_code)]
fn call_telegram_send_message(
    token: &str,
    chat_id: i64,
    message_text: &str,
    reply_to_message_id: Option<i64>,
) -> Result<Value, String> {
    wait_for_telegram_send_rate_limit(chat_id);
    gateway_telegram_call_send_message(
        token,
        chat_id,
        message_text,
        reply_to_message_id,
        Duration::from_secs(15),
    )
}

fn call_telegram_send_chat_action(token: &str, chat_id: i64) -> Result<Value, String> {
    telegram_call_send_chat_action(token, chat_id, Duration::from_secs(5))
}

fn env_truthy(name: &str) -> bool {
    env::var(name)
        .map(|value| parse_telegram_env_truthy_value(&value))
        .unwrap_or(false)
}

fn env_u64(name: &str) -> Option<u64> {
    env::var(name)
        .ok()
        .and_then(|value| parse_telegram_env_u64_value(&value))
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

fn run_hepta_model_turn_with_plan(
    prompt: &str,
    model_runner_plan: &NativeTelegramModelRunnerPlan,
) -> Result<String, String> {
    invoke_native_telegram_model_runner_with_plan(
        model_runner_plan,
        prompt,
        run_mlx_local_chat_completion,
        run_hepta_in_process_model_turn,
        run_hepta_exec_child_model_turn,
    )
    .into_result()
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

fn telegram_model_failure_fallback_enabled() -> bool {
    env_truthy(TELEGRAM_MODEL_FAILURE_FALLBACK_ENV)
}

fn start_telegram_typing_keepalive(token: &str, chat_id: i64) -> Option<TelegramTypingKeepalive> {
    telegram_start_typing_keepalive(
        telegram_typing_keepalive_enabled(),
        token,
        chat_id,
        telegram_typing_keepalive_interval(),
        call_telegram_send_chat_action,
    )
}

fn telegram_typing_keepalive_enabled() -> bool {
    env_truthy(TELEGRAM_TYPING_KEEPALIVE_ENV)
}

fn telegram_typing_keepalive_interval() -> Duration {
    telegram_typing_keepalive_interval_policy(env_u64(TELEGRAM_TYPING_KEEPALIVE_INTERVAL_ENV))
}

pub(crate) fn telegram_model_runner_plan() -> NativeTelegramModelRunnerPlan {
    let model_ref = env::var(TELEGRAM_MODEL_ENV)
        .ok()
        .or_else(|| env::var(HEPTA_DEFAULT_MODEL_ENV).ok());
    let mlx_base_url = env::var(TELEGRAM_MLX_BASE_URL_ENV).ok();
    let mlx_max_tokens = env_u64(TELEGRAM_MLX_MAX_TOKENS_ENV);
    select_native_telegram_model_runner(
        model_ref.as_deref(),
        mlx_base_url.as_deref(),
        mlx_max_tokens,
        telegram_in_process_model_runner_enabled(),
        telegram_codex_core_runner_enabled(),
    )
}

fn run_mlx_local_chat_completion(
    prompt: &str,
    model_runner_plan: &NativeTelegramModelRunnerPlan,
) -> Result<String, String> {
    let prompt = prompt.trim();
    if prompt.is_empty() {
        return Err("Telegram MLX runner requires non-empty prompt material".to_string());
    }
    let base_url = model_runner_plan
        .mlx_base_url
        .as_deref()
        .ok_or_else(|| "Telegram MLX runner requires a local base URL".to_string())?;
    let model = model_runner_plan
        .mlx_model
        .as_deref()
        .ok_or_else(|| "Telegram MLX runner requires a selected model".to_string())?;
    let max_tokens = model_runner_plan
        .mlx_max_tokens
        .ok_or_else(|| "Telegram MLX runner requires a max token limit".to_string())?;
    let endpoint = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let body = native_telegram_mlx_chat_completion_body(model, prompt, max_tokens)?;
    let client = reqwest::blocking::Client::builder()
        .timeout(telegram_model_timeout())
        .build()
        .map_err(|error| format!("failed to build local MLX model client: {error}"))?;
    let response = client.post(endpoint).json(&body).send().map_err(|error| {
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
    extract_native_telegram_openai_chat_completion_text(&body)
}

fn run_hepta_in_process_model_turn(prompt: &str) -> Result<String, String> {
    let prompt = prompt.trim();
    if prompt.is_empty() {
        return Err("Telegram model runner requires non-empty prompt material".to_string());
    }
    let prompt = native_telegram_codex_core_prompt(
        prompt,
        telegram_hepta_intelligence_context_enabled(),
        telegram_plugin_capability_context_enabled(),
    )?;

    let timeout = telegram_model_timeout();
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

fn telegram_codex_core_runner_enabled() -> bool {
    env_truthy(TELEGRAM_CODEX_CORE_RUNNER_ENV)
}

fn telegram_hepta_intelligence_context_enabled() -> bool {
    env::var(TELEGRAM_HEPTA_INTELLIGENCE_CONTEXT_ENV)
        .map(|value| parse_telegram_env_truthy_value(&value))
        .unwrap_or(true)
}

fn telegram_plugin_capability_context_enabled() -> bool {
    env::var(TELEGRAM_PLUGIN_CAPABILITY_CONTEXT_ENV)
        .map(|value| parse_telegram_env_truthy_value(&value))
        .unwrap_or(true)
}

fn run_hepta_exec_child_model_turn(prompt: &str) -> Result<String, String> {
    let prompt = prompt.trim();
    if prompt.is_empty() {
        return Err("Telegram model runner requires non-empty prompt material".to_string());
    }
    let prompt = native_telegram_codex_core_prompt(
        prompt,
        telegram_hepta_intelligence_context_enabled(),
        telegram_plugin_capability_context_enabled(),
    )?;

    let exe = env::current_exe()
        .map_err(|error| format!("failed to resolve current Hepta executable: {error}"))?;
    let tempdir = tempfile::Builder::new()
        .prefix("hepta-telegram-model-")
        .tempdir()
        .map_err(|error| format!("failed to create Telegram model tempdir: {error}"))?;
    let last_message_path = tempdir.path().join("last-message.txt");
    let args = native_telegram_exec_child_args(&last_message_path, &prompt);
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

    let status = wait_for_native_telegram_model_child(&mut child, timeout)?;
    if let Some(error) = native_telegram_exec_child_status_error(status) {
        return Err(error);
    }

    let output = fs::read_to_string(&last_message_path)
        .map_err(|error| format!("failed to read gated Hepta exec last message: {error}"))?;
    extract_native_telegram_exec_child_final_message(&output)
}

fn telegram_model_timeout() -> Duration {
    native_telegram_model_timeout(env_u64(TELEGRAM_MODEL_TIMEOUT_ENV))
}

fn wait_for_telegram_send_rate_limit(chat_id: i64) {
    telegram_wait_for_send_rate_limit(chat_id, telegram_send_min_interval());
}

fn telegram_send_min_interval() -> Duration {
    telegram_send_min_interval_policy(env_u64(TELEGRAM_SEND_MIN_INTERVAL_ENV))
}

fn telegram_read_max_attempts() -> u64 {
    telegram_read_max_attempts_policy(env_u64(TELEGRAM_READ_MAX_ATTEMPTS_ENV))
}

fn telegram_read_retry_backoff() -> Duration {
    telegram_read_retry_backoff_policy(env_u64(TELEGRAM_READ_RETRY_BACKOFF_ENV))
}

fn telegram_send_max_attempts() -> u64 {
    telegram_send_max_attempts_policy(env_u64(TELEGRAM_SEND_MAX_ATTEMPTS_ENV))
}

fn telegram_send_retry_backoff() -> Duration {
    telegram_send_retry_backoff_policy(env_u64(TELEGRAM_SEND_RETRY_BACKOFF_ENV))
}

#[allow(clippy::too_many_arguments)]
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
}
