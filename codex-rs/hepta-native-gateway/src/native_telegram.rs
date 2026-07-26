use std::cell::RefCell;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use anyhow::Context;
#[cfg(feature = "codex-in-process-runner")]
use codex_arg0::Arg0DispatchPaths;
pub(crate) use hepta_gateway::NativeTelegramConfigStatus;
pub(crate) use hepta_gateway::NativeTelegramConfigStatusInput;
pub(crate) use hepta_gateway::NativeTelegramCursorPlan;
pub(crate) use hepta_gateway::NativeTelegramCursorStatus;
pub(crate) use hepta_gateway::NativeTelegramDeliveryLedgerStatus;
pub(crate) use hepta_gateway::NativeTelegramDrainOnceApiResultInput;
pub(crate) use hepta_gateway::NativeTelegramDrainOncePreflightInput;
pub(crate) use hepta_gateway::NativeTelegramDrainOnceShellReadinessInput;
pub(crate) use hepta_gateway::NativeTelegramDrainOnceStatus;
pub(crate) use hepta_gateway::NativeTelegramDrainOnceStatusInput;
pub(crate) use hepta_gateway::NativeTelegramDrainPipelineInput;
pub(crate) use hepta_gateway::NativeTelegramGatewayGateSummary;
pub(crate) use hepta_gateway::NativeTelegramGatewayGateSummaryInput;
pub(crate) use hepta_gateway::NativeTelegramLiveSoakObservationReport;
pub(crate) use hepta_gateway::NativeTelegramLiveSoakObservationState;
pub(crate) use hepta_gateway::NativeTelegramLiveSoakStatus;
pub(crate) use hepta_gateway::NativeTelegramLiveSoakStatusInput;
pub(crate) use hepta_gateway::NativeTelegramModelBridgeStatus;
pub(crate) use hepta_gateway::NativeTelegramModelBridgeStatusInput;
pub(crate) use hepta_gateway::NativeTelegramModelRunnerPlan;
pub(crate) use hepta_gateway::NativeTelegramModelTurnPlanStatus;
pub(crate) use hepta_gateway::NativeTelegramModelTurnPlanStatusInput;
pub(crate) use hepta_gateway::NativeTelegramPluginStatus;
pub(crate) use hepta_gateway::NativeTelegramPluginStatusInput;
pub(crate) use hepta_gateway::NativeTelegramPollLoopStatus;
pub(crate) use hepta_gateway::NativeTelegramPollLoopStatusInput;
pub(crate) use hepta_gateway::NativeTelegramProductionGuardPolicyInput;
pub(crate) use hepta_gateway::NativeTelegramProductionGuardStatus;
pub(crate) use hepta_gateway::NativeTelegramProductionReadinessInput;
pub(crate) use hepta_gateway::NativeTelegramProductionReadinessStatus;
pub(crate) use hepta_gateway::NativeTelegramReceiveOnceApiResultInput;
pub(crate) use hepta_gateway::NativeTelegramReceiveOnceErrorInput;
pub(crate) use hepta_gateway::NativeTelegramReceiveOncePreflightInput;
pub(crate) use hepta_gateway::NativeTelegramReceiveOnceShellReadinessInput;
pub(crate) use hepta_gateway::NativeTelegramReceiveOnceStatus;
pub(crate) use hepta_gateway::NativeTelegramReplyTargetMaterial;
pub(crate) use hepta_gateway::NativeTelegramSendPlanStatus;
pub(crate) use hepta_gateway::NativeTelegramSendPlanStatusInput;
pub(crate) use hepta_gateway::NativeTelegramTokenObservationInput;
use hepta_gateway::TELEGRAM_ALLOWED_UPDATES;
pub(crate) use hepta_gateway::TelegramTypingKeepalive;
use hepta_gateway::build_native_telegram_config_status;
use hepta_gateway::build_telegram_drain_once_status;
use hepta_gateway::build_telegram_gateway_gate_summary;
use hepta_gateway::build_telegram_live_soak_status;
use hepta_gateway::build_telegram_model_bridge_status;
use hepta_gateway::build_telegram_model_turn_plan_status;
use hepta_gateway::build_telegram_plugin_status;
use hepta_gateway::build_telegram_poll_loop_status;
use hepta_gateway::build_telegram_production_guard_status_from_policy;
use hepta_gateway::build_telegram_production_readiness_status;
use hepta_gateway::build_telegram_receive_once_error_status;
use hepta_gateway::build_telegram_receive_once_status_from_api_result;
use hepta_gateway::build_telegram_send_plan_status;
use hepta_gateway::execute_telegram_drain_pipeline_for_updates;
use hepta_gateway::extract_native_telegram_config_metadata;
use hepta_gateway::extract_native_telegram_exec_child_final_message;
use hepta_gateway::extract_native_telegram_openai_chat_completion_text;
use hepta_gateway::extract_telegram_candidate_material;
use hepta_gateway::finalize_telegram_drain_pipeline_status;
use hepta_gateway::invoke_native_telegram_model_runner_with_plan;
use hepta_gateway::native_telegram_exec_child_args;
use hepta_gateway::native_telegram_exec_child_status_error;
use hepta_gateway::native_telegram_hepta_kernel_prompt;
use hepta_gateway::native_telegram_mlx_chat_completion_body;
use hepta_gateway::native_telegram_model_failure_fallback_message;
use hepta_gateway::native_telegram_model_timeout;
use hepta_gateway::parse_telegram_env_truthy_value;
use hepta_gateway::parse_telegram_env_u64_value;
use hepta_gateway::plan_telegram_drain_once_api_result;
use hepta_gateway::plan_telegram_drain_once_preflight;
use hepta_gateway::plan_telegram_drain_once_shell_readiness;
use hepta_gateway::plan_telegram_receive_once_preflight_status;
use hepta_gateway::plan_telegram_receive_once_shell_readiness;
use hepta_gateway::resolve_native_telegram_token_observation;
use hepta_gateway::select_native_telegram_model_runner;
use hepta_gateway::telegram_bot_token_shape_ok as token_shape_ok;
use hepta_gateway::telegram_call_get_updates_once as gateway_telegram_call_get_updates_once;
use hepta_gateway::telegram_call_send_chat_action;
use hepta_gateway::telegram_call_send_message as gateway_telegram_call_send_message;
use hepta_gateway::telegram_cursor_status_from_path;
use hepta_gateway::telegram_get_updates_with_retry;
use hepta_gateway::telegram_poll_loop_interval_ms_policy;
use hepta_gateway::telegram_poll_loop_should_spawn;
use hepta_gateway::telegram_read_max_attempts_policy;
use hepta_gateway::telegram_read_retry_backoff_policy;
use hepta_gateway::telegram_receive_limit_policy;
use hepta_gateway::telegram_redact_token_like_text as redact_token_like_text;
use hepta_gateway::telegram_send_max_attempts_policy;
use hepta_gateway::telegram_send_min_interval_policy;
use hepta_gateway::telegram_send_retry_backoff_policy;
use hepta_gateway::telegram_soak_max_attention_count_policy;
use hepta_gateway::telegram_soak_max_observed_age_ms_policy;
use hepta_gateway::telegram_soak_min_poll_iterations_policy;
use hepta_gateway::telegram_start_typing_keepalive;
use hepta_gateway::telegram_system_time_unix_ms;
use hepta_gateway::telegram_transport_plan_for_config_status;
use hepta_gateway::telegram_typing_keepalive_interval_policy;
use hepta_gateway::telegram_wait_for_send_rate_limit;
use hepta_gateway::wait_for_native_telegram_model_child;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
#[cfg(feature = "codex-in-process-runner")]
use tokio::runtime::Handle;
use zeroize::Zeroizing;

use crate::runtime_composition::NativeGatewayRuntime;
use crate::runtime_composition::RuntimeTelegramReceiveAuthority;
use crate::telegram_authority::TelegramPipelinePermit;
use crate::telegram_authority::TelegramPipelineReceipt;
use crate::telegram_authority::TelegramProviderAck;
use crate::telegram_authority::TelegramReadResult;

const LEGACY_RUNTIME_SLUG: &str = "openclaw";
const LEGACY_CONFIG_FILE_NAME: &str = "openclaw.json";
const LOCAL_IMPORT_CONFIG_PATH: &str = ".hepta/local-import/private/config/openclaw.json";
const LOCAL_IMPORT_MANIFEST_PATH: &str = ".hepta/local-import/manifest.json";
pub(crate) const TELEGRAM_INGRESS_CURSOR_PATH: &str = ".hepta/telegram/ingress-drain-cursor.json";
pub(crate) const TELEGRAM_DELIVERY_LEDGER_PATH: &str = ".hepta/telegram/delivery-ledger.jsonl";
pub(crate) const TELEGRAM_LIVE_READ_ENV: &str = "HEPTA_NATIVE_TELEGRAM_LIVE_READ";
pub(crate) const TELEGRAM_MODEL_TURN_GATE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_TURN";
pub(crate) const TELEGRAM_SEND_GATE_ENV: &str = "HEPTA_NATIVE_TELEGRAM_SEND";
pub(crate) const TELEGRAM_POLL_LOOP_ENV: &str = "HEPTA_NATIVE_TELEGRAM_POLL_LOOP";
pub(crate) const TELEGRAM_DELIVERY_APPROVED_ENV: &str = "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED";
pub(crate) const TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_IN_PROCESS_MODEL_RUNNER";
pub(crate) const TELEGRAM_HEPTA_KERNEL_RUNNER_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_HEPTA_KERNEL_RUNNER";
// Legacy compatibility: old installs used this name before the Hepta kernel
// boundary made Codex an internal engine rather than the product runner.
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
const TELEGRAM_ALLOW_LEGACY_INLINE_TOKEN_ENV: &str =
    "HEPTA_NATIVE_TELEGRAM_ALLOW_LEGACY_INLINE_TOKEN";
const TELEGRAM_SECRET_FILE_MAX_BYTES: u64 = 4096;
const OPERATOR_TELEGRAM_SELECTION_RULE: &str =
    "first_allowed_chat_update_at_or_after_cursor_with_prompt_reply_target_and_model_requirement";
const OPERATOR_TELEGRAM_PROMPT_SCOPE: &str =
    "trimmed_message_text_caption_or_callback_data_nonempty_and_bounded_by_authority";
static TELEGRAM_LIVE_SOAK_OBSERVATION: OnceLock<Mutex<NativeTelegramLiveSoakObservationState>> =
    OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct OperatorTelegramExecutionIdentity {
    schema: &'static str,
    session_model_provider: String,
    session_model: String,
    runner_plan: NativeTelegramModelRunnerPlan,
    config_generation: String,
    dm_policy: String,
    group_policy: String,
    allowed_chat_ids: Vec<i64>,
    selection_rule: &'static str,
    prompt_scope: &'static str,
    hepta_intelligence_context_enabled: bool,
    plugin_capability_context_enabled: bool,
}

impl OperatorTelegramExecutionIdentity {
    pub(crate) fn binding_hash(&self) -> anyhow::Result<String> {
        let bytes = serde_json::to_vec(self).context("encode Telegram execution identity")?;
        Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
    }

    fn allows_chat(&self, chat_id: i64) -> bool {
        self.allowed_chat_ids.binary_search(&chat_id).is_ok()
    }

    fn model_runner_plan(&self) -> &NativeTelegramModelRunnerPlan {
        &self.runner_plan
    }
}

pub(crate) fn telegram_plugin_status(requested: bool, poll_ms: u64) -> NativeTelegramPluginStatus {
    let config = if requested {
        load_telegram_config_metadata_status()
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
    authority: &RuntimeTelegramReceiveAuthority,
) -> NativeTelegramReceiveOnceStatus {
    let _request_binding_hash = authority.request_binding_hash();
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
        load_telegram_config_metadata_status()
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
        load_telegram_config_metadata_status()
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
    telegram_drain_once_status_with_gates(requested, telegram_gateway_gate_summary(), None)
}

/// Executes one operator-authorized read → model → send pipeline.
///
/// Every secret/config read and external call is inside a phase closure, so
/// `TelegramPipelinePermit` durably records the corresponding intent before
/// this adapter can reach it. The legacy poll loop never receives such a
/// permit and therefore remains fail-closed.
pub(crate) fn execute_operator_authorized_telegram_drain(
    permit: TelegramPipelinePermit<'_>,
    execution_identity: OperatorTelegramExecutionIdentity,
) -> Result<TelegramPipelineReceipt, anyhow::Error> {
    let gates = telegram_gateway_gate_summary();
    if !gates.delivery_approval_gate_enabled
        || !gates.live_read_gate_enabled
        || !gates.model_turn_gate_enabled
        || !gates.send_gate_enabled
    {
        anyhow::bail!("operator-authorized Telegram pipeline requires all legacy live gates");
    }
    let token = RefCell::new(None::<Zeroizing<String>>);
    permit.execute_with(
        Path::new(TELEGRAM_DELIVERY_LEDGER_PATH),
        Path::new(TELEGRAM_INGRESS_CURSOR_PATH),
        |request| {
            let config = load_telegram_execution_config_status();
            if !config.config_ready() {
                anyhow::bail!("Telegram execution config is not ready");
            }
            let effective_token = load_effective_telegram_token().map_err(anyhow::Error::msg)?;
            let api = call_telegram_get_updates(&effective_token, 20, request.cursor)
                .map_err(anyhow::Error::msg)?;
            if api.get("ok").and_then(Value::as_bool) != Some(true) {
                anyhow::bail!("Telegram getUpdates did not return a provider success ACK");
            }
            let updates = api
                .get("result")
                .and_then(Value::as_array)
                .context("Telegram getUpdates result is not an array")?;
            let candidate = updates
                .iter()
                .filter_map(extract_telegram_candidate_material)
                .find(|candidate| {
                    candidate.update_id.is_some_and(|update_id| {
                        request.cursor.is_none_or(|cursor| update_id >= cursor)
                    }) && candidate.prompt_text.is_some()
                        && candidate.reply_target.is_some()
                        && candidate
                            .reply_target
                            .as_ref()
                            .is_some_and(|target| execution_identity.allows_chat(target.chat_id))
                        && candidate.requires_model
                })
                .context("Telegram getUpdates returned no allowed exact model/reply candidate")?;
            let update_id = candidate
                .update_id
                .context("Telegram candidate update id missing")?;
            let reply_target = candidate
                .reply_target
                .context("Telegram candidate reply target missing")?;
            let prompt = candidate
                .prompt_text
                .context("Telegram candidate prompt missing")?;
            token.replace(Some(Zeroizing::new(effective_token)));
            Ok(TelegramReadResult {
                update_id,
                chat_id: reply_target.chat_id,
                reply_to_message_id: reply_target.reply_to_message_id,
                prompt,
            })
        },
        |request| {
            run_hepta_model_turn_with_execution_identity(&request.prompt, &execution_identity)
                .map_err(anyhow::Error::msg)
        },
        |plan| {
            let token = token.borrow();
            let token = token
                .as_deref()
                .context("Telegram send lost its read-phase token")?;
            let provider_response = call_telegram_send_message(
                token,
                plan.chat_id,
                &plan.message_text,
                plan.reply_to_message_id,
            )
            .map_err(anyhow::Error::msg)?;
            if provider_response.get("ok").and_then(Value::as_bool) != Some(true) {
                anyhow::bail!("Telegram sendMessage did not return a provider success ACK");
            }
            let result = provider_response
                .get("result")
                .context("Telegram sendMessage result is missing")?;
            let provider_message_id = result
                .get("message_id")
                .and_then(Value::as_i64)
                .context("Telegram sendMessage ACK lacks message_id")?;
            let chat_id = result
                .get("chat")
                .and_then(|chat| chat.get("id"))
                .and_then(Value::as_i64)
                .context("Telegram sendMessage ACK lacks chat.id")?;
            let canonical_response =
                serde_json::to_vec(&provider_response).context("encode Telegram provider ACK")?;
            Ok(TelegramProviderAck {
                provider: "telegram-bot-api".into(),
                provider_message_id,
                chat_id,
                raw_response_hash: format!("{:x}", Sha256::digest(canonical_response)),
            })
        },
    )
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
    runtime: Arc<NativeGatewayRuntime>,
) -> Option<thread::JoinHandle<()>> {
    if !telegram_poll_loop_should_spawn(
        requested,
        env_truthy(TELEGRAM_POLL_LOOP_ENV),
        env_truthy(TELEGRAM_DELIVERY_APPROVED_ENV),
    ) {
        return None;
    }

    Some(thread::spawn(move || {
        run_telegram_poll_loop(requested, poll_ms, runtime)
    }))
}

fn run_telegram_poll_loop(requested: bool, poll_ms: u64, runtime: Arc<NativeGatewayRuntime>) {
    let poll_ms = telegram_poll_loop_interval_ms_policy(poll_ms);
    loop {
        let status = telegram_drain_once_status_with_gates(
            requested,
            telegram_gateway_gate_summary(),
            Some(runtime.as_ref()),
        );
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
    crate::telegram_durable_files::cursor_status(
        requested,
        Path::new(TELEGRAM_INGRESS_CURSOR_PATH),
        TELEGRAM_INGRESS_CURSOR_PATH,
    )
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
    runtime: Option<&NativeGatewayRuntime>,
) -> NativeTelegramDrainOnceStatus {
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
    let runtime_cursor_status = preflight
        .status_probe_executes_pipeline
        .then(|| telegram_cursor_status_from_path(Path::new(TELEGRAM_INGRESS_CURSOR_PATH)));
    let runtime_admission_error = if preflight.status_probe_executes_pipeline {
        match runtime {
            Some(runtime) => runtime
                .preflight_telegram_drain(
                    runtime_cursor_status
                        .as_ref()
                        .and_then(|status| status.next_update_offset),
                )
                .and_then(|receipt| {
                    receipt
                        .require_live_pipeline_authority()
                        .map_err(anyhow::Error::from)
                })
                .err()
                .map(|error| error.to_string()),
            None => Some("telegram_runtime_admission.runtime_unavailable".to_string()),
        }
    } else {
        None
    };
    let status_probe_executes_pipeline =
        preflight.status_probe_executes_pipeline && runtime_admission_error.is_none();
    let config = if !requested {
        NativeTelegramConfigStatus::disabled()
    } else if runtime_admission_error.is_some() {
        NativeTelegramConfigStatus::error(
            None,
            false,
            "runtime admission denied before Telegram config or token observation".to_string(),
        )
    } else {
        load_telegram_execution_config_status()
    };
    let mut status = preflight.status;
    let mut error = preflight.error;
    let mut bot_api_ok = None;
    let mut local_next_update_offset = None;
    let mut get_updates_offset = None;
    let mut live_read_started = false;
    let mut external_network_read = false;

    if let Some(runtime_admission_error) = runtime_admission_error {
        status = "attention";
        error = Some(runtime_admission_error);
    }

    if status_probe_executes_pipeline && let Some(cursor_status) = runtime_cursor_status {
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
        load_telegram_config_metadata_status()
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
    let config = load_telegram_execution_config_status();
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

fn load_telegram_config_metadata_status() -> NativeTelegramConfigStatus {
    let Some(config_path) = resolve_private_hepta_runtime_config_path() else {
        return NativeTelegramConfigStatus::missing(
            "Hepta private Telegram config not found".to_string(),
        );
    };

    match load_telegram_config_metadata_status_from_path(&config_path) {
        Ok(status) => status,
        Err(error) => NativeTelegramConfigStatus::error(
            Some(config_path.display().to_string()),
            config_path.is_file(),
            redact_token_like_text(&error),
        ),
    }
}

fn load_telegram_config_metadata_status_from_path(
    path: &Path,
) -> Result<NativeTelegramConfigStatus, String> {
    let file_metadata = fs::metadata(path)
        .map_err(|error| format!("failed to inspect Hepta private Telegram config: {error}"))?;
    if !file_metadata.is_file() {
        return Err("Hepta private Telegram config is not a regular file".to_string());
    }

    Ok(build_native_telegram_config_status(
        NativeTelegramConfigStatusInput {
            config_path: Some(path.display().to_string()),
            config_found: true,
            enabled: false,
            dm_policy: "unobserved".to_string(),
            group_policy: "unobserved".to_string(),
            allow_from_count: 0,
            group_count: 0,
            token_source: "config_content_unobserved",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_file_security_ready: false,
            token_shape_ok: false,
            error: None,
        },
    ))
}

#[derive(Serialize)]
struct OperatorTelegramConfigGeneration {
    schema: &'static str,
    enabled: bool,
    dm_policy: String,
    group_policy: String,
    allowed_chat_ids: Vec<i64>,
    token_reference_binding: String,
    selection_rule: &'static str,
    prompt_scope: &'static str,
}

pub(crate) fn operator_telegram_execution_identity(
    session_model_provider: &str,
    session_model: &str,
) -> anyhow::Result<OperatorTelegramExecutionIdentity> {
    let config_path = resolve_private_hepta_runtime_config_path()
        .context("Hepta private Telegram config not found")?;
    let runner_plan = telegram_model_runner_plan();
    operator_telegram_execution_identity_from_path(
        &config_path,
        session_model_provider,
        session_model,
        runner_plan,
        telegram_hepta_intelligence_context_enabled(),
        telegram_plugin_capability_context_enabled(),
    )
}

fn operator_telegram_execution_identity_from_path(
    config_path: &Path,
    session_model_provider: &str,
    session_model: &str,
    runner_plan: NativeTelegramModelRunnerPlan,
    hepta_intelligence_context_enabled: bool,
    plugin_capability_context_enabled: bool,
) -> anyhow::Result<OperatorTelegramExecutionIdentity> {
    if !runner_plan.runner_plan_ready {
        anyhow::bail!("operator Telegram model runner plan is not ready");
    }
    let raw = fs::read_to_string(config_path)
        .with_context(|| format!("read Telegram config {}", config_path.display()))?;
    let config: Value = serde_json::from_str(&raw).context("decode Telegram config")?;
    let metadata = extract_native_telegram_config_metadata(config_path, &config)
        .map_err(anyhow::Error::msg)?;
    if !metadata.enabled {
        anyhow::bail!("operator Telegram config is disabled");
    }
    let telegram = config
        .pointer("/channels/telegram")
        .context("channels.telegram config is missing")?;
    let allowed_chat_ids = operator_telegram_allowed_chat_ids(telegram);
    if allowed_chat_ids.is_empty() {
        anyhow::bail!("operator Telegram execution requires an explicit numeric chat allowlist");
    }
    let generation_material = OperatorTelegramConfigGeneration {
        schema: "hepta.native.telegram-config-generation.v1",
        enabled: metadata.enabled,
        dm_policy: metadata.dm_policy.clone(),
        group_policy: metadata.group_policy.clone(),
        allowed_chat_ids: allowed_chat_ids.clone(),
        token_reference_binding: operator_telegram_token_reference_binding(telegram),
        selection_rule: OPERATOR_TELEGRAM_SELECTION_RULE,
        prompt_scope: OPERATOR_TELEGRAM_PROMPT_SCOPE,
    };
    let generation_bytes =
        serde_json::to_vec(&generation_material).context("encode Telegram config generation")?;
    Ok(OperatorTelegramExecutionIdentity {
        schema: "hepta.native.operator-telegram-execution-identity.v1",
        session_model_provider: session_model_provider.to_owned(),
        session_model: session_model.to_owned(),
        runner_plan,
        config_generation: format!("sha256:{:x}", Sha256::digest(generation_bytes)),
        dm_policy: metadata.dm_policy,
        group_policy: metadata.group_policy,
        allowed_chat_ids,
        selection_rule: OPERATOR_TELEGRAM_SELECTION_RULE,
        prompt_scope: OPERATOR_TELEGRAM_PROMPT_SCOPE,
        hepta_intelligence_context_enabled,
        plugin_capability_context_enabled,
    })
}

fn operator_telegram_allowed_chat_ids(telegram: &Value) -> Vec<i64> {
    let mut allowed = BTreeSet::new();
    if let Some(values) = telegram.get("allowFrom").and_then(Value::as_array) {
        for value in values {
            if let Some(chat_id) = value.as_str().and_then(operator_telegram_parse_binding_id) {
                allowed.insert(chat_id);
            }
        }
    }
    if let Some(groups) = telegram.get("groups") {
        match groups {
            Value::Array(values) => {
                for value in values {
                    if let Some(chat_id) = operator_telegram_group_chat_id(value) {
                        allowed.insert(chat_id);
                    }
                }
            }
            Value::Object(values) => {
                for value in values.values() {
                    if let Some(chat_id) = operator_telegram_group_chat_id(value) {
                        allowed.insert(chat_id);
                    }
                }
            }
            _ => {}
        }
    }
    allowed.into_iter().collect()
}

fn operator_telegram_parse_binding_id(raw: &str) -> Option<i64> {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    let normalized = lower
        .strip_prefix("telegram:")
        .or_else(|| lower.strip_prefix("tg:"))
        .unwrap_or(&lower)
        .trim();
    normalized
        .parse::<i64>()
        .ok()
        .filter(|chat_id| *chat_id != 0)
}

fn operator_telegram_group_chat_id(value: &Value) -> Option<i64> {
    value
        .as_i64()
        .or_else(|| value.as_str().and_then(operator_telegram_parse_binding_id))
        .or_else(|| {
            value
                .get("id")
                .and_then(|id| id.as_i64().or_else(|| id.as_str()?.parse().ok()))
        })
        .filter(|chat_id| *chat_id != 0)
}

fn operator_telegram_token_reference_binding(telegram: &Value) -> String {
    let Some(token) = telegram.get("botToken") else {
        return "missing".to_string();
    };
    if token.as_str().is_some_and(|value| !value.trim().is_empty()) {
        return "legacy_inline_present".to_string();
    }
    let source = token
        .get("source")
        .and_then(Value::as_str)
        .unwrap_or("missing")
        .trim();
    let provider = token
        .get("provider")
        .and_then(Value::as_str)
        .unwrap_or("missing")
        .trim();
    let secret_id = token
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("missing")
        .trim();
    format!("source={source};provider={provider};id={secret_id}")
}

fn load_telegram_execution_config_status() -> NativeTelegramConfigStatus {
    let Some(config_path) = resolve_private_hepta_runtime_config_path() else {
        return NativeTelegramConfigStatus::missing(
            "Hepta private Telegram config not found".to_string(),
        );
    };

    match load_telegram_execution_config_status_from_path(&config_path) {
        Ok(status) => status,
        Err(error) => NativeTelegramConfigStatus::error(
            Some(config_path.display().to_string()),
            config_path.is_file(),
            redact_token_like_text(&error),
        ),
    }
}

fn load_telegram_execution_config_status_from_path(
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
    let token_file_security = metadata
        .token_secret_path
        .as_ref()
        .map(|path| inspect_telegram_secret_file(path))
        .unwrap_or_default();
    let env_token = env::var("HEPTA_TELEGRAM_BOT_TOKEN")
        .ok()
        .or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let file_token_result = if env_token.is_none() {
        metadata
            .token_secret_path
            .as_ref()
            .map(|path| read_secure_telegram_secret_file(path))
    } else {
        None
    };
    let file_token = file_token_result
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let inline_token = bot_token_ref
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let legacy_inline_allowed = env_truthy(TELEGRAM_ALLOW_LEGACY_INLINE_TOKEN_ENV);
    let inline_token_selected = env_token.is_none()
        && file_token.is_none()
        && inline_token.is_some()
        && legacy_inline_allowed;
    let inline_token_rejected = env_token.is_none()
        && file_token.is_none()
        && inline_token.is_some()
        && !legacy_inline_allowed;
    let token_observation =
        resolve_native_telegram_token_observation(NativeTelegramTokenObservationInput {
            env_token_present: env_token.is_some(),
            env_token_shape_ok: env_token.as_deref().map(token_shape_ok).unwrap_or(false),
            file_token_present: file_token.is_some(),
            file_token_shape_ok: file_token.as_deref().map(token_shape_ok).unwrap_or(false),
            inline_token_present: inline_token_selected,
            inline_token_shape_ok: inline_token.as_deref().map(token_shape_ok).unwrap_or(false),
            token_secret_ref_present: metadata.token_secret_ref_present,
        });
    let token_source = if inline_token_rejected {
        "inline_config_rejected"
    } else if token_observation.token_source == "inline_config" {
        "inline_config_legacy_override"
    } else {
        token_observation.token_source
    };
    let error = if inline_token_rejected {
        Some(format!(
            "inline Telegram bot token is rejected; migrate to an environment variable or a secure secret file (temporary compatibility requires {TELEGRAM_ALLOW_LEGACY_INLINE_TOKEN_ENV}=1)"
        ))
    } else if env_token.is_some() {
        None
    } else {
        file_token_result.and_then(Result::err)
    };
    Ok(build_native_telegram_config_status(
        NativeTelegramConfigStatusInput {
            config_path: Some(path.display().to_string()),
            config_found: true,
            enabled: metadata.enabled,
            dm_policy: metadata.dm_policy,
            group_policy: metadata.group_policy,
            allow_from_count: metadata.allow_from_count,
            group_count: metadata.group_count,
            token_source,
            token_secret_ref_present: metadata.token_secret_ref_present,
            token_secret_provider: metadata.token_secret_provider,
            token_secret_id_present: metadata.token_secret_id_present,
            token_file_present: token_file_security.present,
            token_file_mode_0600: token_file_security.mode_0600,
            token_file_security_ready: token_file_security.ready,
            token_shape_ok: token_observation.token_shape_ok && !inline_token_rejected,
            error,
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
    if let Some(token) = env_token {
        return validate_effective_telegram_token(token);
    }
    if let Some(path) = metadata.token_secret_path.as_ref() {
        return validate_effective_telegram_token(read_secure_telegram_secret_file(path)?);
    }
    if let Some(token) = inline_token {
        if env_truthy(TELEGRAM_ALLOW_LEGACY_INLINE_TOKEN_ENV) {
            return validate_effective_telegram_token(token);
        }
        return Err(format!(
            "inline Telegram bot token is rejected; migrate to an environment variable or a secure secret file (temporary compatibility requires {TELEGRAM_ALLOW_LEGACY_INLINE_TOKEN_ENV}=1)"
        ));
    }
    Err("Telegram bot token is not configured".to_string())
}

fn validate_effective_telegram_token(token: String) -> Result<String, String> {
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
    if let Ok(raw) = fs::read_to_string(&manifest)
        && let Ok(value) = serde_json::from_str::<Value>(&raw)
        && let Some(import_root) = value.get("import_root").and_then(Value::as_str)
    {
        let candidate = PathBuf::from(import_root)
            .join("private/config")
            .join(LEGACY_CONFIG_FILE_NAME);
        if candidate.is_file() {
            return Some(candidate);
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

#[derive(Debug, Default)]
struct TelegramSecretFileSecurity {
    present: bool,
    mode_0600: bool,
    ready: bool,
}

fn inspect_telegram_secret_file(path: &Path) -> TelegramSecretFileSecurity {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return TelegramSecretFileSecurity::default();
    };
    let present = true;

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        use std::os::unix::fs::PermissionsExt;

        let symlink_safe = !metadata.file_type().is_symlink();
        let regular_file = metadata.file_type().is_file();
        let mode_0600 = metadata.permissions().mode() & 0o7777 == 0o600;
        // SAFETY: geteuid has no preconditions and does not dereference pointers.
        let effective_uid = unsafe { libc::geteuid() };
        let owner_current_user = metadata.uid() == effective_uid;
        TelegramSecretFileSecurity {
            present,
            mode_0600,
            ready: symlink_safe && regular_file && mode_0600 && owner_current_user,
        }
    }

    #[cfg(not(unix))]
    {
        let _ = metadata;
        // Fail closed until an ACL-specific owner/private-access verifier is
        // available. Environment-backed tokens remain supported cross-platform.
        TelegramSecretFileSecurity {
            present,
            mode_0600: false,
            ready: false,
        }
    }
}

fn read_secure_telegram_secret_file(path: &Path) -> Result<String, String> {
    let mut file = open_telegram_secret_file_no_follow(path)?;
    let metadata = file
        .metadata()
        .map_err(|error| format!("failed to inspect Telegram token secret file: {error}"))?;
    validate_open_telegram_secret_file(&metadata)?;

    let mut bytes = Vec::new();
    file.by_ref()
        .take(TELEGRAM_SECRET_FILE_MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("failed to read Telegram token secret file: {error}"))?;
    if bytes.len() as u64 > TELEGRAM_SECRET_FILE_MAX_BYTES {
        return Err(format!(
            "Telegram token secret file exceeds {TELEGRAM_SECRET_FILE_MAX_BYTES} bytes"
        ));
    }
    String::from_utf8(bytes)
        .map(|value| value.trim().to_string())
        .map_err(|_| "Telegram token secret file is not valid UTF-8".to_string())
}

#[cfg(unix)]
fn open_telegram_secret_file_no_follow(path: &Path) -> Result<File, String> {
    use std::os::unix::fs::OpenOptionsExt;

    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .map_err(|error| {
            format!("failed to open Telegram token secret file without following symlinks: {error}")
        })
}

#[cfg(not(unix))]
fn open_telegram_secret_file_no_follow(path: &Path) -> Result<File, String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("failed to inspect Telegram token secret file: {error}"))?;
    if metadata.file_type().is_symlink() {
        return Err("Telegram token secret file must not be a symlink".to_string());
    }
    if !metadata.file_type().is_file() {
        return Err("Telegram token secret path must be a regular file".to_string());
    }
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|error| format!("failed to open Telegram token secret file: {error}"))
}

#[cfg(unix)]
fn validate_open_telegram_secret_file(metadata: &fs::Metadata) -> Result<(), String> {
    // SAFETY: geteuid has no preconditions and does not dereference pointers.
    let effective_uid = unsafe { libc::geteuid() };
    validate_unix_telegram_secret_file(metadata, effective_uid)
}

#[cfg(unix)]
fn validate_unix_telegram_secret_file(
    metadata: &fs::Metadata,
    expected_uid: libc::uid_t,
) -> Result<(), String> {
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::PermissionsExt;

    if !metadata.file_type().is_file() {
        return Err("Telegram token secret path must be a regular file".to_string());
    }
    if metadata.uid() != expected_uid {
        return Err("Telegram token secret file must be owned by the current user".to_string());
    }
    if metadata.permissions().mode() & 0o7777 != 0o600 {
        return Err("Telegram token secret file permissions must be 0600".to_string());
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_open_telegram_secret_file(metadata: &fs::Metadata) -> Result<(), String> {
    if !metadata.file_type().is_file() {
        return Err("Telegram token secret path must be a regular file".to_string());
    }
    Err(
        "Telegram token secret files require platform ACL owner/private-access verification; use an environment-backed token on this platform"
            .to_string(),
    )
}

fn run_hepta_model_turn_with_plan(
    prompt: &str,
    model_runner_plan: &NativeTelegramModelRunnerPlan,
) -> Result<String, String> {
    run_hepta_model_turn_with_frozen_context(
        prompt,
        model_runner_plan,
        telegram_hepta_intelligence_context_enabled(),
        telegram_plugin_capability_context_enabled(),
    )
}

fn run_hepta_model_turn_with_execution_identity(
    prompt: &str,
    execution_identity: &OperatorTelegramExecutionIdentity,
) -> Result<String, String> {
    run_hepta_model_turn_with_frozen_context(
        prompt,
        execution_identity.model_runner_plan(),
        execution_identity.hepta_intelligence_context_enabled,
        execution_identity.plugin_capability_context_enabled,
    )
}

fn run_hepta_model_turn_with_frozen_context(
    prompt: &str,
    model_runner_plan: &NativeTelegramModelRunnerPlan,
    hepta_intelligence_context_enabled: bool,
    plugin_capability_context_enabled: bool,
) -> Result<String, String> {
    invoke_native_telegram_model_runner_with_plan(
        model_runner_plan,
        prompt,
        run_mlx_local_chat_completion,
        |prompt| {
            run_hepta_in_process_model_turn_with_context(
                prompt,
                hepta_intelligence_context_enabled,
                plugin_capability_context_enabled,
            )
        },
        |prompt| {
            run_hepta_exec_child_model_turn_with_context(
                prompt,
                hepta_intelligence_context_enabled,
                plugin_capability_context_enabled,
            )
        },
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
        telegram_hepta_kernel_runner_enabled(),
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

fn run_hepta_in_process_model_turn_with_context(
    prompt: &str,
    hepta_intelligence_context_enabled: bool,
    plugin_capability_context_enabled: bool,
) -> Result<String, String> {
    #[cfg(feature = "codex-in-process-runner")]
    {
        run_hepta_in_process_model_turn_with_codex_exec(
            prompt,
            hepta_intelligence_context_enabled,
            plugin_capability_context_enabled,
        )
    }

    #[cfg(not(feature = "codex-in-process-runner"))]
    {
        let _ = (
            prompt,
            hepta_intelligence_context_enabled,
            plugin_capability_context_enabled,
        );
        Err("gated in-process Codex exec runner is not compiled into the active hepta-cli service binary".to_string())
    }
}

#[cfg(feature = "codex-in-process-runner")]
fn run_hepta_in_process_model_turn_with_codex_exec(
    prompt: &str,
    hepta_intelligence_context_enabled: bool,
    plugin_capability_context_enabled: bool,
) -> Result<String, String> {
    let prompt = prompt.trim();
    if prompt.is_empty() {
        return Err("Telegram model runner requires non-empty prompt material".to_string());
    }
    let prompt = native_telegram_hepta_kernel_prompt(
        prompt,
        hepta_intelligence_context_enabled,
        plugin_capability_context_enabled,
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

pub(crate) fn telegram_hepta_kernel_runner_enabled() -> bool {
    env_truthy(TELEGRAM_HEPTA_KERNEL_RUNNER_ENV) || env_truthy(TELEGRAM_CODEX_CORE_RUNNER_ENV)
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

fn run_hepta_exec_child_model_turn_with_context(
    prompt: &str,
    hepta_intelligence_context_enabled: bool,
    plugin_capability_context_enabled: bool,
) -> Result<String, String> {
    let prompt = prompt.trim();
    if prompt.is_empty() {
        return Err("Telegram model runner requires non-empty prompt material".to_string());
    }
    let prompt = native_telegram_hepta_kernel_prompt(
        prompt,
        hepta_intelligence_context_enabled,
        plugin_capability_context_enabled,
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
#[path = "../tests/unit/native_telegram.rs"]
mod tests;
