use std::path::Path;
use std::time::Duration;

use serde_json::Value;

use crate::telegram_policy::{
    NativeTelegramCandidateMaterial, NativeTelegramDuplicateDecision,
    NativeTelegramGatewayGateSummary, NativeTelegramModelExecutionReport,
    NativeTelegramModelInvocationRequestPlan, NativeTelegramReplyTargetMaterial,
    NativeTelegramSendExecutionReport, NativeTelegramSendRequestPlan,
    first_model_candidate_with_duplicate_decision,
};
use crate::telegram_transport::{
    NativeTelegramSendExecutionInput, execute_telegram_send_after_model_output,
};
use hepta_runtime::redact_native_telegram_model_runner_error;

pub use hepta_runtime::{
    HEPTA_KERNEL_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE as NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE,
    NativeTelegramSessionBridgePlan, native_telegram_drain_final_status,
    native_telegram_model_failure_fallback_allowed, native_telegram_model_failure_fallback_message,
};

#[derive(Debug, Clone)]
pub struct NativeTelegramModelExecutionInput {
    pub candidate: Option<NativeTelegramCandidateMaterial>,
    pub duplicate_decision: Option<NativeTelegramDuplicateDecision>,
    pub model_turn_gate_env: &'static str,
    pub model_turn_gate_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramModelExecutionOutcome {
    pub report: NativeTelegramModelExecutionReport,
    pub model_output: Option<String>,
    pub reply_target: Option<NativeTelegramReplyTargetMaterial>,
    pub candidate_next_update_offset: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramDrainPipelineOutcome {
    pub invocation_request: NativeTelegramModelInvocationRequestPlan,
    pub model_execution: NativeTelegramModelExecutionReport,
    pub send_request: NativeTelegramSendRequestPlan,
    pub send_execution: NativeTelegramSendExecutionReport,
}

#[derive(Debug, Clone)]
pub struct NativeTelegramDrainPipelineFinalStatus {
    pub status: &'static str,
    pub error: Option<String>,
    pub outcome: NativeTelegramDrainPipelineOutcome,
}

#[derive(Debug, Clone, Copy)]
pub struct NativeTelegramDrainPipelineInput<'a> {
    pub updates: &'a [Value],
    pub next_update_offset: Option<i64>,
    pub token: Option<&'a str>,
    pub gates: &'a NativeTelegramGatewayGateSummary,
    pub cursor_path: &'a Path,
    pub delivery_ledger_path: &'a Path,
    pub model_failure_fallback_enabled: bool,
    pub model_failure_fallback_message: &'a str,
    pub send_max_attempts: u64,
    pub send_retry_backoff: Duration,
}

pub fn finalize_telegram_drain_pipeline_status(
    mut outcome: NativeTelegramDrainPipelineOutcome,
    model_runner_process_spawned_by_status: bool,
    previous_status: &'static str,
    previous_error: Option<String>,
) -> NativeTelegramDrainPipelineFinalStatus {
    let final_status = native_telegram_drain_final_status(
        outcome.model_execution.session_runner_invoked,
        model_runner_process_spawned_by_status,
        outcome.send_execution.status,
        outcome.send_execution.error.as_deref(),
        outcome.model_execution.status,
        outcome.model_execution.error.as_deref(),
        previous_status,
        previous_error.as_deref(),
    );
    if final_status.local_process_spawned {
        outcome.model_execution.local_process_spawned = true;
    }

    NativeTelegramDrainPipelineFinalStatus {
        status: final_status.status,
        error: final_status.error,
        outcome,
    }
}

pub fn execute_telegram_model_turn_after_candidate<F>(
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
                input.model_turn_gate_env,
                input.model_turn_gate_enabled,
            )
        }
        (Some(candidate), _) if !candidate.requires_model => {
            NativeTelegramModelInvocationRequestPlan::attention(
                candidate,
                "not_model_candidate",
                None,
                input.model_turn_gate_env,
                input.model_turn_gate_enabled,
            )
        }
        (Some(candidate), None) if candidate.requires_model => {
            NativeTelegramModelInvocationRequestPlan::attention(
                candidate,
                "missing_update_id",
                None,
                input.model_turn_gate_env,
                input.model_turn_gate_enabled,
            )
        }
        _ => NativeTelegramModelInvocationRequestPlan::empty(
            input.model_turn_gate_env,
            input.model_turn_gate_enabled,
        ),
    };
    let mut report =
        NativeTelegramModelExecutionReport::from_invocation_request(&invocation_request);

    if !input.model_turn_gate_enabled {
        report.error = Some(format!(
            "Telegram model execution is gated by {}",
            input.model_turn_gate_env
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
            report.error = Some(redact_native_telegram_model_runner_error(&error));
            NativeTelegramModelExecutionOutcome {
                report,
                model_output: None,
                reply_target: candidate.reply_target,
                candidate_next_update_offset: invocation_request.candidate_next_update_offset,
            }
        }
    }
}

pub fn execute_telegram_drain_pipeline_for_updates<F, S>(
    input: NativeTelegramDrainPipelineInput<'_>,
    run_model: F,
    send_message: S,
) -> NativeTelegramDrainPipelineOutcome
where
    F: FnOnce(Option<&NativeTelegramReplyTargetMaterial>, &str) -> Result<String, String>,
    S: FnMut(&str, i64, &str, Option<i64>) -> Result<Value, String>,
{
    let (candidate, duplicate_decision, invocation_request) =
        first_model_candidate_with_duplicate_decision(
            input.updates,
            input.next_update_offset,
            input.gates.model_turn_gate_env,
            input.gates.model_turn_gate_enabled,
        );

    let typing_reply_target = candidate
        .as_ref()
        .and_then(|candidate| candidate.reply_target.clone());
    let model_outcome = match (candidate.clone(), duplicate_decision.clone()) {
        (Some(candidate), Some(decision)) => execute_telegram_model_turn_after_candidate(
            NativeTelegramModelExecutionInput {
                candidate: Some(candidate),
                duplicate_decision: Some(decision),
                model_turn_gate_env: input.gates.model_turn_gate_env,
                model_turn_gate_enabled: input.gates.model_turn_gate_enabled,
            },
            |prompt| run_model(typing_reply_target.as_ref(), prompt),
        ),
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

    let fallback_output = telegram_model_failure_fallback_output(
        &model_outcome,
        input.model_failure_fallback_enabled,
    )
    .map(|_| input.model_failure_fallback_message.to_string());
    let delivery_output = model_outcome
        .model_output
        .as_deref()
        .or(fallback_output.as_deref());

    let send_request = NativeTelegramSendRequestPlan::from_model_output(
        delivery_output,
        model_outcome.reply_target.is_some(),
        model_outcome.candidate_next_update_offset,
        input.gates.send_gate_env,
        input.gates.send_gate_enabled,
    );
    let send_execution = execute_telegram_send_after_model_output(
        NativeTelegramSendExecutionInput {
            token: input.token,
            model_output: delivery_output,
            reply_target: model_outcome.reply_target.as_ref(),
            candidate_next_update_offset: model_outcome.candidate_next_update_offset,
            send_gate_env: input.gates.send_gate_env,
            send_gate_enabled: input.gates.send_gate_enabled,
            cursor_path: input.cursor_path,
            delivery_ledger_path: input.delivery_ledger_path,
            send_max_attempts: input.send_max_attempts,
            send_retry_backoff: input.send_retry_backoff,
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

fn telegram_model_failure_fallback_output(
    outcome: &NativeTelegramModelExecutionOutcome,
    enabled: bool,
) -> Option<()> {
    let report = &outcome.report;
    if native_telegram_model_failure_fallback_allowed(
        enabled,
        report.session_runner_invoked,
        report.status,
        outcome.reply_target.is_some(),
        outcome.candidate_next_update_offset.is_some(),
    ) {
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telegram_cursor::telegram_cursor_status_from_path;
    use crate::telegram_delivery::telegram_delivery_ledger_status_from_path;
    use crate::telegram_policy::{
        extract_telegram_candidate_material, telegram_duplicate_decision,
    };

    const MODEL_GATE: &str = "HEPTA_NATIVE_TELEGRAM_MODEL_TURN";
    const SEND_GATE: &str = "HEPTA_NATIVE_TELEGRAM_SEND";

    fn gates(
        model_turn_gate_enabled: bool,
        send_gate_enabled: bool,
    ) -> NativeTelegramGatewayGateSummary {
        NativeTelegramGatewayGateSummary {
            delivery_approval_gate_env: "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED",
            delivery_approval_gate_enabled: true,
            live_read_gate_env: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
            live_read_gate_enabled: true,
            model_turn_gate_env: MODEL_GATE,
            model_turn_gate_enabled,
            send_gate_env: SEND_GATE,
            send_gate_enabled,
            readiness_summary_performs_live_read: false,
            readiness_summary_invokes_model: false,
            readiness_summary_sends_message: false,
        }
    }

    fn baseline_pipeline_outcome() -> NativeTelegramDrainPipelineOutcome {
        let invocation_request =
            NativeTelegramModelInvocationRequestPlan::disabled(MODEL_GATE, true);
        let model_execution =
            NativeTelegramModelExecutionReport::from_invocation_request(&invocation_request);
        let send_request = NativeTelegramSendRequestPlan::disabled(SEND_GATE, true);
        let send_execution = NativeTelegramSendExecutionReport::from_send_request(&send_request);
        NativeTelegramDrainPipelineOutcome {
            invocation_request,
            model_execution,
            send_request,
            send_execution,
        }
    }

    #[test]
    fn drain_pipeline_finalizer_marks_delivered_and_process_spawn() {
        let mut outcome = baseline_pipeline_outcome();
        outcome.model_execution.session_runner_invoked = true;
        outcome.send_execution.status = "delivered";
        outcome.send_execution.error = Some("stale error".to_string());

        let final_status =
            finalize_telegram_drain_pipeline_status(outcome, true, "ready", Some("old".into()));

        assert_eq!(final_status.status, "drained");
        assert_eq!(final_status.error, None);
        assert!(final_status.outcome.model_execution.local_process_spawned);
    }

    #[test]
    fn drain_pipeline_finalizer_prefers_send_attention_over_model_attention() {
        let mut outcome = baseline_pipeline_outcome();
        outcome.model_execution.status = "attention";
        outcome.model_execution.error = Some("model failed".to_string());
        outcome.send_execution.status = "attention";
        outcome.send_execution.error = Some("send failed".to_string());

        let final_status = finalize_telegram_drain_pipeline_status(outcome, false, "ready", None);

        assert_eq!(final_status.status, "attention");
        assert_eq!(final_status.error.as_deref(), Some("send failed"));
        assert!(!final_status.outcome.model_execution.local_process_spawned);
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
                model_turn_gate_env: MODEL_GATE,
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
        assert!(outcome.report.model_output_present);
        assert_eq!(
            outcome.model_output.as_deref(),
            Some("private model response text")
        );
        assert!(outcome.reply_target.is_some());

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
                model_turn_gate_env: MODEL_GATE,
                model_turn_gate_enabled: false,
            },
            |_| panic!("model runner must not run while gated"),
        );

        assert_eq!(outcome.report.status, "gated");
        assert!(!outcome.report.runner_invocation_allowed);
        assert!(!outcome.report.session_runner_invoked);
        assert!(!outcome.report.model_output_present);
        assert!(outcome.report.error.unwrap().contains(MODEL_GATE));
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
                model_turn_gate_env: MODEL_GATE,
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
    fn drain_pipeline_delivers_without_serializing_private_material() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        let delivery_ledger_path = temp.path().join("delivery-ledger.jsonl");
        let token = "123456:ABCDEFGHIJKLMNOPQRSTUVWX";
        let gates = gates(true, true);
        let update = serde_json::json!({
            "update_id": 49,
            "message": {
                "message_id": 15,
                "text": "private pipeline prompt",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        });

        let outcome = execute_telegram_drain_pipeline_for_updates(
            NativeTelegramDrainPipelineInput {
                updates: &[update],
                next_update_offset: Some(49),
                token: Some(token),
                gates: &gates,
                cursor_path: &cursor_path,
                delivery_ledger_path: &delivery_ledger_path,
                model_failure_fallback_enabled: false,
                model_failure_fallback_message: "",
                send_max_attempts: 1,
                send_retry_backoff: Duration::ZERO,
            },
            |reply_target, prompt| {
                assert!(reply_target.is_some());
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

        assert_eq!(outcome.model_execution.status, "completed");
        assert_eq!(outcome.send_execution.status, "delivered");
        assert!(outcome.send_execution.cursor_written);
        assert_eq!(
            telegram_cursor_status_from_path(&cursor_path).next_update_offset,
            Some(50)
        );
        assert_eq!(
            telegram_delivery_ledger_status_from_path(&delivery_ledger_path, "/store/delivery")
                .latest_stage
                .as_deref(),
            Some("acked")
        );

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
        let delivery_ledger_path = temp.path().join("delivery-ledger.jsonl");
        let gates = gates(false, true);
        let update = serde_json::json!({
            "update_id": 49,
            "message": {
                "message_id": 15,
                "text": "private pipeline prompt",
                "chat": { "id": 6476198178_i64, "type": "private" }
            }
        });

        let outcome = execute_telegram_drain_pipeline_for_updates(
            NativeTelegramDrainPipelineInput {
                updates: &[update],
                next_update_offset: Some(49),
                token: Some("123456:ABCDEFGHIJKLMNOPQRSTUVWX"),
                gates: &gates,
                cursor_path: &cursor_path,
                delivery_ledger_path: &delivery_ledger_path,
                model_failure_fallback_enabled: false,
                model_failure_fallback_message: "",
                send_max_attempts: 1,
                send_retry_backoff: Duration::ZERO,
            },
            |_, _| panic!("model runner must not run while model gate is closed"),
            |_, _, _, _| panic!("sendMessage must not run without model output"),
        );

        assert_eq!(outcome.model_execution.status, "gated");
        assert!(!outcome.model_execution.session_runner_invoked);
        assert!(!outcome.model_execution.model_output_present);
        assert_eq!(outcome.send_execution.status, "waiting_model_output");
        assert!(!outcome.send_execution.send_attempted);
        assert!(!outcome.send_execution.cursor_written);
        assert!(!outcome.send_execution.external_send);
        assert!(outcome.model_execution.error.unwrap().contains(MODEL_GATE));
        assert!(!cursor_path.exists());
        assert!(!delivery_ledger_path.exists());
    }

    #[test]
    fn model_failure_fallback_message_is_bounded_and_static() {
        let message = native_telegram_model_failure_fallback_message();
        assert!(!message.trim().is_empty());
        assert!(message.len() < 512);
        assert_eq!(message, NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE);
    }
}
