use serde::Serialize;
use serde_json::Value;

pub const TELEGRAM_DRAIN_ONCE_STAGES: &[&str] = &[
    "receive_getUpdates",
    "duplicate_suppression",
    "model_turn",
    "sendMessage",
    "cursor_commit",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramGatewayGateSummary {
    pub delivery_approval_gate_env: &'static str,
    pub delivery_approval_gate_enabled: bool,
    pub live_read_gate_env: &'static str,
    pub live_read_gate_enabled: bool,
    pub model_turn_gate_env: &'static str,
    pub model_turn_gate_enabled: bool,
    pub send_gate_env: &'static str,
    pub send_gate_enabled: bool,
    pub readiness_summary_performs_live_read: bool,
    pub readiness_summary_invokes_model: bool,
    pub readiness_summary_sends_message: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeTelegramGatewayGateSummaryInput {
    pub delivery_approval_gate_env: &'static str,
    pub delivery_approval_gate_enabled: bool,
    pub live_read_gate_env: &'static str,
    pub live_read_gate_enabled: bool,
    pub model_turn_gate_env: &'static str,
    pub model_turn_gate_enabled: bool,
    pub send_gate_env: &'static str,
    pub send_gate_enabled: bool,
}

pub fn build_telegram_gateway_gate_summary(
    input: NativeTelegramGatewayGateSummaryInput,
) -> NativeTelegramGatewayGateSummary {
    NativeTelegramGatewayGateSummary {
        delivery_approval_gate_env: input.delivery_approval_gate_env,
        delivery_approval_gate_enabled: input.delivery_approval_gate_enabled,
        live_read_gate_env: input.live_read_gate_env,
        live_read_gate_enabled: input.live_read_gate_enabled,
        model_turn_gate_env: input.model_turn_gate_env,
        model_turn_gate_enabled: input.model_turn_gate_enabled,
        send_gate_env: input.send_gate_env,
        send_gate_enabled: input.send_gate_enabled,
        readiness_summary_performs_live_read: false,
        readiness_summary_invokes_model: false,
        readiness_summary_sends_message: false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramExecutionPlan {
    pub execution_plan_ready: bool,
    pub stages: &'static [&'static str],
    pub all_required_gates_enabled: bool,
    pub first_missing_gate: Option<&'static str>,
    pub receive_before_model: bool,
    pub send_after_model_success: bool,
    pub cursor_commit_after_delivery: bool,
    pub status_probe_executes_pipeline: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramDuplicateDecision {
    pub decision: &'static str,
    pub update_id: i64,
    pub current_next_update_offset: Option<i64>,
    pub candidate_next_update_offset: Option<i64>,
    pub already_drained: bool,
    pub should_invoke_model: bool,
    pub should_record_duplicate: bool,
    pub cursor_write_allowed_after_delivery: bool,
    pub raw_update_payload_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramIngressInspection {
    pub parser_ready: bool,
    pub update_count: usize,
    pub allowed_update_count: usize,
    pub latest_observed_update_id: Option<i64>,
    pub latest_allowed_update_id: Option<i64>,
    pub latest_allowed_next_update_offset: Option<i64>,
    pub latest_allowed_text_present: bool,
    pub message_count: usize,
    pub edited_message_count: usize,
    pub callback_query_count: usize,
    pub reaction_count: usize,
    pub raw_message_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_sender_id_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeTelegramCandidateMaterial {
    pub update_id: Option<i64>,
    pub kind: String,
    pub prompt_text: Option<String>,
    pub has_reply_target: bool,
    pub reply_target: Option<NativeTelegramReplyTargetMaterial>,
    pub requires_model: bool,
    pub raw_identifiers_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeTelegramReplyTargetMaterial {
    pub chat_id: i64,
    pub reply_to_message_id: Option<i64>,
    pub raw_identifiers_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramModelInvocationRequestPlan {
    pub request_builder_ready: bool,
    pub candidate_present: bool,
    pub candidate_kind: Option<String>,
    pub duplicate_decision: &'static str,
    pub prompt_material_in_memory: bool,
    pub prompt_material_serialized: bool,
    pub reply_target_available: bool,
    pub stable_session_key_ready: bool,
    pub should_invoke_model: bool,
    pub should_record_duplicate: bool,
    pub candidate_next_update_offset: Option<i64>,
    pub model_turn_gate_env: &'static str,
    pub model_turn_gate_enabled: bool,
    pub runner_invocation_allowed: bool,
    pub session_runner_invoked: bool,
    pub local_process_spawned: bool,
    pub external_send: bool,
    pub cursor_written: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_sender_id_exposed: bool,
    pub raw_message_id_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramModelExecutionReport {
    pub status: &'static str,
    pub execution_ready: bool,
    pub model_turn_gate_env: &'static str,
    pub model_turn_gate_enabled: bool,
    pub candidate_present: bool,
    pub prompt_material_present: bool,
    pub reply_target_available: bool,
    pub stable_session_key_ready: bool,
    pub candidate_next_update_offset: Option<i64>,
    pub runner_invocation_allowed: bool,
    pub session_runner_invoked: bool,
    pub local_process_spawned: bool,
    pub model_output_present: bool,
    pub external_send: bool,
    pub cursor_written: bool,
    pub raw_update_payload_exposed: bool,
    pub raw_prompt_text_exposed: bool,
    pub raw_response_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_sender_id_exposed: bool,
    pub raw_message_id_exposed: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramModelTurnPlan {
    pub planner_ready: bool,
    pub candidate_count: usize,
    pub text_candidate_count: usize,
    pub callback_candidate_count: usize,
    pub reaction_candidate_count: usize,
    pub reply_target_count: usize,
    pub candidate_kinds: Vec<String>,
    pub prompt_material_policy: &'static str,
    pub session_key_strategy: &'static str,
    pub reply_target_strategy: &'static str,
    pub model_turn_invocation_gate: &'static str,
    pub send_delivery_gate: &'static str,
    pub raw_message_text_exposed: bool,
    pub raw_callback_data_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_sender_id_exposed: bool,
    pub raw_message_id_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramSendRequestPlan {
    pub request_builder_ready: bool,
    pub model_output_present: bool,
    pub reply_target_available: bool,
    pub candidate_next_update_offset: Option<i64>,
    pub send_gate_env: &'static str,
    pub send_gate_enabled: bool,
    pub send_allowed: bool,
    pub request_body_materialized_by_status: bool,
    pub delivery_performed_by_status: bool,
    pub cursor_commit_allowed_after_delivery: bool,
    pub raw_response_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_message_id_exposed: bool,
    pub raw_token_exposed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramSendExecutionReport {
    pub status: &'static str,
    pub execution_ready: bool,
    pub send_gate_env: &'static str,
    pub send_gate_enabled: bool,
    pub model_output_present: bool,
    pub reply_target_available: bool,
    pub candidate_next_update_offset: Option<i64>,
    pub send_allowed: bool,
    pub send_attempted: bool,
    pub bot_api_ack: Option<bool>,
    pub delivery_ledger_write_attempted: bool,
    pub delivery_ledger_written_count: usize,
    pub latest_delivery_ledger_stage: Option<String>,
    pub cursor_commit_attempted: bool,
    pub cursor_written: bool,
    pub request_body_materialized_by_execution: bool,
    pub external_network_write: bool,
    pub external_send: bool,
    pub raw_response_text_exposed: bool,
    pub raw_chat_id_exposed: bool,
    pub raw_message_id_exposed: bool,
    pub raw_token_exposed: bool,
    pub error: Option<String>,
}

impl NativeTelegramModelTurnPlan {
    pub fn disabled() -> Self {
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

    pub fn ready() -> Self {
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
    pub fn disabled(model_turn_gate_env: &'static str, model_turn_gate_enabled: bool) -> Self {
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
            model_turn_gate_env,
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

    pub fn empty(model_turn_gate_env: &'static str, model_turn_gate_enabled: bool) -> Self {
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
            model_turn_gate_env,
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

    pub fn attention(
        candidate: NativeTelegramCandidateMaterial,
        duplicate_decision: &'static str,
        candidate_next_update_offset: Option<i64>,
        model_turn_gate_env: &'static str,
        model_turn_gate_enabled: bool,
    ) -> Self {
        Self::from_parts(
            candidate,
            duplicate_decision,
            false,
            false,
            candidate_next_update_offset,
            model_turn_gate_env,
            model_turn_gate_enabled,
        )
    }

    pub fn from_candidate(
        candidate: NativeTelegramCandidateMaterial,
        decision: NativeTelegramDuplicateDecision,
        model_turn_gate_env: &'static str,
        model_turn_gate_enabled: bool,
    ) -> Self {
        Self::from_parts(
            candidate,
            decision.decision,
            decision.should_invoke_model,
            decision.should_record_duplicate,
            decision.candidate_next_update_offset,
            model_turn_gate_env,
            model_turn_gate_enabled,
        )
    }

    fn from_parts(
        candidate: NativeTelegramCandidateMaterial,
        duplicate_decision: &'static str,
        should_invoke_model: bool,
        should_record_duplicate: bool,
        candidate_next_update_offset: Option<i64>,
        model_turn_gate_env: &'static str,
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
            model_turn_gate_env,
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
    pub fn disabled(model_turn_gate_env: &'static str, model_turn_gate_enabled: bool) -> Self {
        Self {
            status: "disabled",
            execution_ready: false,
            model_turn_gate_env,
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

    pub fn from_invocation_request(request: &NativeTelegramModelInvocationRequestPlan) -> Self {
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

impl NativeTelegramSendRequestPlan {
    pub fn disabled(send_gate_env: &'static str, send_gate_enabled: bool) -> Self {
        Self {
            request_builder_ready: false,
            model_output_present: false,
            reply_target_available: false,
            candidate_next_update_offset: None,
            send_gate_env,
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

    pub fn from_model_output(
        model_output: Option<&str>,
        reply_target_available: bool,
        candidate_next_update_offset: Option<i64>,
        send_gate_env: &'static str,
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
            send_gate_env,
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
    pub fn disabled(send_gate_env: &'static str, send_gate_enabled: bool) -> Self {
        Self {
            status: "disabled",
            execution_ready: false,
            send_gate_env,
            send_gate_enabled,
            model_output_present: false,
            reply_target_available: false,
            candidate_next_update_offset: None,
            send_allowed: false,
            send_attempted: false,
            bot_api_ack: None,
            delivery_ledger_write_attempted: false,
            delivery_ledger_written_count: 0,
            latest_delivery_ledger_stage: None,
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

    pub fn from_send_request(request: &NativeTelegramSendRequestPlan) -> Self {
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
            delivery_ledger_write_attempted: false,
            delivery_ledger_written_count: 0,
            latest_delivery_ledger_stage: None,
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

pub fn telegram_drain_first_missing_gate(
    gates: &NativeTelegramGatewayGateSummary,
) -> Option<&'static str> {
    if !gates.delivery_approval_gate_enabled {
        Some(gates.delivery_approval_gate_env)
    } else if !gates.live_read_gate_enabled {
        Some(gates.live_read_gate_env)
    } else if !gates.model_turn_gate_enabled {
        Some(gates.model_turn_gate_env)
    } else if !gates.send_gate_enabled {
        Some(gates.send_gate_env)
    } else {
        None
    }
}

pub fn telegram_drain_status_probe_executes_pipeline(
    requested: bool,
    gates: &NativeTelegramGatewayGateSummary,
) -> bool {
    requested && gates.delivery_approval_gate_enabled && gates.live_read_gate_enabled
}

pub fn telegram_drain_execution_plan(
    requested: bool,
    gates: &NativeTelegramGatewayGateSummary,
) -> NativeTelegramExecutionPlan {
    let first_missing_gate = telegram_drain_first_missing_gate(gates);
    NativeTelegramExecutionPlan {
        execution_plan_ready: requested,
        stages: TELEGRAM_DRAIN_ONCE_STAGES,
        all_required_gates_enabled: requested && first_missing_gate.is_none(),
        first_missing_gate,
        receive_before_model: true,
        send_after_model_success: true,
        cursor_commit_after_delivery: true,
        status_probe_executes_pipeline: telegram_drain_status_probe_executes_pipeline(
            requested, gates,
        ),
    }
}

pub fn telegram_update_already_drained(update_id: i64, next_update_offset: Option<i64>) -> bool {
    next_update_offset
        .map(|cursor| update_id < cursor)
        .unwrap_or(false)
}

pub fn telegram_duplicate_decision(
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

pub fn telegram_next_update_offset(update_id: i64) -> Option<i64> {
    update_id.checked_add(1)
}

pub fn telegram_message_is_reply_candidate(message: &Value) -> bool {
    telegram_message_has_reply_target(message) && telegram_message_text_present(message)
}

pub fn telegram_message_text_present(message: &Value) -> bool {
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

pub fn telegram_message_has_reply_target(message: &Value) -> bool {
    telegram_message_reply_target_material(message).is_some()
}

pub fn extract_telegram_candidate_material(
    update: &Value,
) -> Option<NativeTelegramCandidateMaterial> {
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

pub fn inspect_telegram_updates(updates: &[Value]) -> NativeTelegramIngressInspection {
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

pub fn plan_model_turn_for_updates(updates: &[Value]) -> NativeTelegramModelTurnPlan {
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

pub fn build_model_invocation_request_plan(
    updates: &[Value],
    next_update_offset: Option<i64>,
    model_turn_gate_env: &'static str,
    model_turn_gate_enabled: bool,
) -> NativeTelegramModelInvocationRequestPlan {
    let (_, _, request) = first_model_candidate_with_duplicate_decision(
        updates,
        next_update_offset,
        model_turn_gate_env,
        model_turn_gate_enabled,
    );
    request
}

pub fn first_model_candidate_with_duplicate_decision(
    updates: &[Value],
    next_update_offset: Option<i64>,
    model_turn_gate_env: &'static str,
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
                model_turn_gate_env,
                model_turn_gate_enabled,
            );
            return (Some(candidate), None, request);
        };
        let decision = telegram_duplicate_decision(update_id, next_update_offset);
        let request = NativeTelegramModelInvocationRequestPlan::from_candidate(
            candidate.clone(),
            decision.clone(),
            model_turn_gate_env,
            model_turn_gate_enabled,
        );
        return (Some(candidate), Some(decision), request);
    }

    (
        None,
        None,
        NativeTelegramModelInvocationRequestPlan::empty(
            model_turn_gate_env,
            model_turn_gate_enabled,
        ),
    )
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

#[cfg(test)]
mod tests {
    use super::{
        NativeTelegramGatewayGateSummary, NativeTelegramGatewayGateSummaryInput,
        NativeTelegramModelExecutionReport, NativeTelegramSendExecutionReport,
        NativeTelegramSendRequestPlan, TELEGRAM_DRAIN_ONCE_STAGES,
        build_model_invocation_request_plan, build_telegram_gateway_gate_summary,
        extract_telegram_candidate_material, inspect_telegram_updates, plan_model_turn_for_updates,
        telegram_drain_execution_plan, telegram_drain_first_missing_gate,
        telegram_drain_status_probe_executes_pipeline, telegram_duplicate_decision,
        telegram_next_update_offset, telegram_update_already_drained,
    };

    fn gates(
        delivery: bool,
        live_read: bool,
        model_turn: bool,
        send: bool,
    ) -> NativeTelegramGatewayGateSummary {
        build_telegram_gateway_gate_summary(NativeTelegramGatewayGateSummaryInput {
            delivery_approval_gate_env: "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED",
            delivery_approval_gate_enabled: delivery,
            live_read_gate_env: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
            live_read_gate_enabled: live_read,
            model_turn_gate_env: "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
            model_turn_gate_enabled: model_turn,
            send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
            send_gate_enabled: send,
        })
    }

    #[test]
    fn gate_summary_builder_is_side_effect_free() {
        let summary = gates(true, false, true, false);
        assert!(summary.delivery_approval_gate_enabled);
        assert!(!summary.live_read_gate_enabled);
        assert!(summary.model_turn_gate_enabled);
        assert!(!summary.send_gate_enabled);
        assert!(!summary.readiness_summary_performs_live_read);
        assert!(!summary.readiness_summary_invokes_model);
        assert!(!summary.readiness_summary_sends_message);
    }

    #[test]
    fn drain_gate_order_is_delivery_then_read_then_model_then_send() {
        assert_eq!(
            telegram_drain_first_missing_gate(&gates(false, false, false, false)),
            Some("HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED")
        );
        assert_eq!(
            telegram_drain_first_missing_gate(&gates(true, false, false, false)),
            Some("HEPTA_NATIVE_TELEGRAM_LIVE_READ")
        );
        assert_eq!(
            telegram_drain_first_missing_gate(&gates(true, true, false, false)),
            Some("HEPTA_NATIVE_TELEGRAM_MODEL_TURN")
        );
        assert_eq!(
            telegram_drain_first_missing_gate(&gates(true, true, true, false)),
            Some("HEPTA_NATIVE_TELEGRAM_SEND")
        );
        assert_eq!(
            telegram_drain_first_missing_gate(&gates(true, true, true, true)),
            None
        );
    }

    #[test]
    fn drain_status_probe_only_needs_delivery_and_live_read_gates() {
        assert!(!telegram_drain_status_probe_executes_pipeline(
            false,
            &gates(true, true, false, false)
        ));
        assert!(!telegram_drain_status_probe_executes_pipeline(
            true,
            &gates(true, false, true, true)
        ));
        assert!(telegram_drain_status_probe_executes_pipeline(
            true,
            &gates(true, true, false, false)
        ));
    }

    #[test]
    fn drain_execution_plan_preserves_cursor_after_delivery_policy() {
        let plan = telegram_drain_execution_plan(true, &gates(true, true, true, true));

        assert!(plan.execution_plan_ready);
        assert_eq!(plan.stages, TELEGRAM_DRAIN_ONCE_STAGES);
        assert!(plan.all_required_gates_enabled);
        assert_eq!(plan.first_missing_gate, None);
        assert!(plan.receive_before_model);
        assert!(plan.send_after_model_success);
        assert!(plan.cursor_commit_after_delivery);
        assert!(plan.status_probe_executes_pipeline);
    }

    #[test]
    fn duplicate_policy_treats_cursor_as_next_update_offset() {
        assert!(telegram_update_already_drained(41, Some(42)));
        assert!(!telegram_update_already_drained(42, Some(42)));
        assert_eq!(telegram_next_update_offset(42), Some(43));
        assert_eq!(telegram_next_update_offset(i64::MAX), None);

        let duplicate = telegram_duplicate_decision(41, Some(42));
        assert_eq!(duplicate.decision, "skip_already_drained");
        assert!(duplicate.should_record_duplicate);
        assert!(!duplicate.should_invoke_model);
        assert!(!duplicate.cursor_write_allowed_after_delivery);

        let candidate = telegram_duplicate_decision(42, Some(42));
        assert_eq!(candidate.decision, "model_candidate");
        assert!(candidate.should_invoke_model);
        assert!(!candidate.should_record_duplicate);
        assert!(candidate.cursor_write_allowed_after_delivery);
        assert_eq!(candidate.candidate_next_update_offset, Some(43));
    }

    #[test]
    fn send_request_and_execution_report_preserve_gates() {
        let disabled = NativeTelegramSendRequestPlan::disabled("HEPTA_NATIVE_TELEGRAM_SEND", false);
        assert!(!disabled.request_builder_ready);
        assert!(!disabled.send_allowed);
        assert_eq!(
            NativeTelegramSendExecutionReport::from_send_request(&disabled).status,
            "disabled"
        );

        let gated = NativeTelegramSendRequestPlan::from_model_output(
            Some("private model response text"),
            true,
            Some(43),
            "HEPTA_NATIVE_TELEGRAM_SEND",
            false,
        );
        assert!(gated.request_builder_ready);
        assert!(gated.model_output_present);
        assert!(gated.reply_target_available);
        assert_eq!(gated.candidate_next_update_offset, Some(43));
        assert!(!gated.request_body_materialized_by_status);
        assert!(!gated.delivery_performed_by_status);
        assert!(!gated.cursor_commit_allowed_after_delivery);
        assert!(!gated.raw_response_text_exposed);
        assert!(!gated.raw_chat_id_exposed);
        assert!(!gated.raw_message_id_exposed);
        assert!(!gated.raw_token_exposed);
        assert!(!gated.send_allowed);
        assert!(
            !serde_json::to_string(&gated)
                .expect("serialize")
                .contains("private model response text")
        );
        assert_eq!(
            NativeTelegramSendExecutionReport::from_send_request(&gated).status,
            "gated"
        );

        let ready = NativeTelegramSendRequestPlan::from_model_output(
            Some(" hello "),
            true,
            Some(43),
            "HEPTA_NATIVE_TELEGRAM_SEND",
            true,
        );
        assert!(ready.send_allowed);
        assert!(ready.cursor_commit_allowed_after_delivery);
        let report = NativeTelegramSendExecutionReport::from_send_request(&ready);
        assert_eq!(report.status, "ready");
        assert!(report.execution_ready);
        assert!(!report.external_send);
        assert!(!report.cursor_written);

        let without_reply_target = NativeTelegramSendRequestPlan::from_model_output(
            Some("private model response text"),
            false,
            Some(43),
            "HEPTA_NATIVE_TELEGRAM_SEND",
            true,
        );
        assert!(without_reply_target.model_output_present);
        assert!(without_reply_target.send_gate_enabled);
        assert!(!without_reply_target.reply_target_available);
        assert!(!without_reply_target.send_allowed);
        assert!(!without_reply_target.cursor_commit_allowed_after_delivery);

        let without_offset = NativeTelegramSendRequestPlan::from_model_output(
            Some("private model response text"),
            true,
            None,
            "HEPTA_NATIVE_TELEGRAM_SEND",
            true,
        );
        assert!(without_offset.model_output_present);
        assert!(without_offset.reply_target_available);
        assert!(!without_offset.send_allowed);
        assert!(!without_offset.cursor_commit_allowed_after_delivery);
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

        let request = build_model_invocation_request_plan(
            &[update],
            Some(47),
            "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
            false,
        );
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
        assert_eq!(
            request.model_turn_gate_env,
            "HEPTA_NATIVE_TELEGRAM_MODEL_TURN"
        );
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

        let request = build_model_invocation_request_plan(
            &[update],
            Some(48),
            "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
            true,
        );
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
        assert_eq!(
            NativeTelegramModelExecutionReport::from_invocation_request(&request).status,
            "duplicate_suppressed"
        );

        let serialized = serde_json::to_string(&request).expect("serialize");
        assert!(!serialized.contains("private duplicate prompt"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains("private_user"));
    }

    #[test]
    fn ingress_inspection_counts_allowed_updates_without_serializing_private_material() {
        let updates = vec![
            serde_json::json!({
                "update_id": 50,
                "message": {
                    "message_id": 12,
                    "text": "private message prompt",
                    "chat": { "id": 6476198178_i64, "type": "private" },
                    "from": { "id": 6476198178_i64, "username": "private_user" }
                }
            }),
            serde_json::json!({
                "update_id": 51,
                "callback_query": {
                    "id": "opaque-callback-id",
                    "data": "button_secret_payload",
                    "message": {
                        "message_id": 13,
                        "chat": { "id": 6476198178_i64, "type": "private" }
                    }
                }
            }),
            serde_json::json!({
                "update_id": 52,
                "message_reaction": {
                    "chat": { "id": 6476198178_i64, "type": "private" }
                }
            }),
        ];

        let inspection = inspect_telegram_updates(&updates);

        assert!(inspection.parser_ready);
        assert_eq!(inspection.update_count, 3);
        assert_eq!(inspection.allowed_update_count, 3);
        assert_eq!(inspection.latest_observed_update_id, Some(52));
        assert_eq!(inspection.latest_allowed_update_id, Some(52));
        assert_eq!(inspection.latest_allowed_next_update_offset, Some(53));
        assert!(inspection.latest_allowed_text_present);
        assert_eq!(inspection.message_count, 1);
        assert_eq!(inspection.callback_query_count, 1);
        assert_eq!(inspection.reaction_count, 1);
        assert!(!inspection.raw_message_text_exposed);
        assert!(!inspection.raw_chat_id_exposed);
        assert!(!inspection.raw_sender_id_exposed);

        let serialized = serde_json::to_string(&inspection).expect("serialize");
        assert!(!serialized.contains("private message prompt"));
        assert!(!serialized.contains("button_secret_payload"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains("private_user"));
        assert!(!serialized.contains("opaque-callback-id"));
    }
}
