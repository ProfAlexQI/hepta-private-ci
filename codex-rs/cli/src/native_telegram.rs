use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Serialize;
use serde_json::Value;

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
const TELEGRAM_DRAIN_ONCE_STAGES: &[&str] = &[
    "receive_getUpdates",
    "duplicate_suppression",
    "model_turn",
    "sendMessage",
    "cursor_commit",
];

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
        in_process_reply_loop_ready: false,
        model_turn_bridge_ready: false,
        bot_api_poll_ready: false,
        bot_api_send_ready: false,
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
        migration_blocker: Some(
            "Bot API polling/send and Codex model-turn bridge are not enabled in hepta-codex yet",
        ),
        next_migration_slice: "wire native Bot API getUpdates/sendMessage loop behind explicit delivery gates",
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
    let bridge_plan = if requested {
        NativeTelegramSessionBridgePlan::ready()
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
    let offset = value
        .get("next_update_offset")
        .or_else(|| value.get("nextUpdateOffset"))
        .and_then(Value::as_i64)
        .ok_or_else(|| "Telegram cursor missing next_update_offset".to_string())?;
    if offset < 0 {
        Err("Telegram cursor next_update_offset must be non-negative".to_string())
    } else {
        Ok(offset)
    }
}

#[cfg(test)]
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
    let first_missing_gate = first_missing_drain_once_gate(&gates);
    let all_required_gates_enabled = requested && first_missing_gate.is_none();
    let status = if !requested {
        "disabled"
    } else if all_required_gates_enabled {
        "planned"
    } else {
        "gated"
    };
    let error = if requested {
        first_missing_gate.map(|gate| {
            format!(
                "Telegram drain-once pipeline is gated before side effects; first missing gate: {gate}"
            )
        })
    } else {
        None
    };

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
            status_probe_executes_pipeline: false,
        },
        live_read_started: false,
        model_turn_started: false,
        send_started: false,
        cursor_written: false,
        external_network_read: false,
        external_network_write: false,
        external_send: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
        error,
        next_migration_slice: "replace this side-effect-free drain plan with gated live read, model turn, send, and cursor commit execution",
    }
}

fn first_missing_drain_once_gate(gates: &NativeTelegramGatewayGateSummary) -> Option<&'static str> {
    if !gates.live_read_gate_enabled {
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

    match call_telegram_get_updates(&token, limit) {
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
        Err(error) => NativeTelegramReceiveOnceStatus::base(
            requested,
            "attention",
            true,
            true,
            limit,
            config,
            transport_plan,
            cursor_plan,
            inspect_telegram_updates(&[]),
            Some(redact_token_like_text(&error)),
        ),
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

fn call_telegram_get_updates(token: &str, limit: usize) -> Result<Value, String> {
    let endpoint = format!("https://api.telegram.org/bot{token}/getUpdates");
    let limit = limit.clamp(1, 20).to_string();
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|error| format!("failed to build Telegram Bot API client: {error}"))?;
    let response = client
        .get(endpoint)
        .query(&[
            ("timeout", "0"),
            ("limit", limit.as_str()),
            ("allowed_updates", TELEGRAM_ALLOWED_UPDATES),
        ])
        .send()
        .map_err(|error| {
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
    message
        .get("chat")
        .and_then(|chat| chat.get("id"))
        .is_some()
        && message.get("message_id").is_some()
        && telegram_message_text_present(message)
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
    requires_model: bool,
    raw_identifiers_exposed: bool,
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
    for update in updates.iter().take(20) {
        let Some(candidate) = extract_telegram_candidate_material(update) else {
            continue;
        };
        if !candidate.requires_model {
            continue;
        }

        let Some(update_id) = candidate.update_id else {
            return NativeTelegramModelInvocationRequestPlan::attention(
                candidate,
                "missing_update_id",
                None,
                model_turn_gate_enabled,
            );
        };
        let decision = telegram_duplicate_decision(update_id, next_update_offset);
        return NativeTelegramModelInvocationRequestPlan::from_candidate(
            candidate,
            decision,
            model_turn_gate_enabled,
        );
    }

    NativeTelegramModelInvocationRequestPlan::empty(model_turn_gate_enabled)
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
        return Some(NativeTelegramCandidateMaterial {
            update_id,
            kind: "callback_query:redacted".to_string(),
            prompt_text: callback
                .get("data")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned),
            has_reply_target: callback
                .get("message")
                .map(telegram_message_has_reply_target)
                .unwrap_or(false),
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
    Some(NativeTelegramCandidateMaterial {
        update_id,
        kind: format!("{prefix}:{kind}"),
        prompt_text: Some(prompt_text),
        has_reply_target: telegram_message_has_reply_target(message),
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
    message
        .get("chat")
        .and_then(|chat| chat.get("id"))
        .is_some()
        && message.get("message_id").is_some()
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

    fn ready() -> Self {
        Self {
            bridge_plan_ready: true,
            runner_kind: "codex_session_runner",
            runner_invocation_strategy: "in-process session runner preferred; local process spawn remains disabled for status probes",
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
            next_migration_slice: "connect redacted receive candidates to Codex model-turn bridge, then enable gated send",
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
        assert!(status.error.unwrap().contains(TELEGRAM_SEND_GATE_ENV));
    }

    #[test]
    fn drain_once_without_gates_stops_before_side_effects() {
        let gates = NativeTelegramGatewayGateSummary {
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
            Some(TELEGRAM_LIVE_READ_ENV)
        );
        assert!(!status.execution_plan.all_required_gates_enabled);
        assert!(status.execution_plan.receive_before_model);
        assert!(status.execution_plan.send_after_model_success);
        assert!(status.execution_plan.cursor_commit_after_delivery);
        assert!(!status.execution_plan.status_probe_executes_pipeline);
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
}
