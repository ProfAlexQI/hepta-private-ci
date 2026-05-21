use std::path::Path;
use std::process::{Child, ExitStatus};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;

pub use hepta_kernel::{
    CODEX_ENGINE_ID, DEFAULT_TELEGRAM_MLX_BASE_URL, DEFAULT_TELEGRAM_MLX_MAX_TOKENS,
    DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS, HEPTA_KERNEL_CONTRACT, HEPTA_KERNEL_OWNER,
    HEPTA_KERNEL_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE, HEPTA_KERNEL_TELEGRAM_RUNNER_KIND,
    HEPTA_KERNEL_TELEGRAM_RUNNER_STRATEGY, HeptaKernelEngine,
    HeptaKernelTelegramDrainFinalStatusPlan, HeptaKernelTelegramRunnerInvocationOutcome,
    HeptaKernelTelegramRunnerPlan, HeptaKernelTelegramSessionBridgePlan, HeptaKernelTurnChannel,
    HeptaKernelTurnInput, HeptaKernelTurnPlan, HeptaKernelTurnStagePlan,
    MAX_TELEGRAM_MLX_MAX_TOKENS, MAX_TELEGRAM_MODEL_TIMEOUT_MS, MIN_TELEGRAM_MODEL_TIMEOUT_MS,
    classify_hepta_kernel_telegram_runner_error, extract_hepta_kernel_exec_child_final_message,
    extract_hepta_kernel_openai_chat_completion_text, hepta_kernel_exec_child_args,
    hepta_kernel_exec_child_status_error, hepta_kernel_mlx_chat_completion_body,
    hepta_kernel_telegram_drain_final_status, hepta_kernel_telegram_model_failure_fallback_allowed,
    hepta_kernel_telegram_model_timeout, hepta_kernel_telegram_prompt,
    invoke_hepta_kernel_telegram_runner_with_plan, parse_hepta_kernel_mlx_model_ref,
    plan_hepta_kernel_telegram_session_bridge, plan_hepta_kernel_turn,
    redact_hepta_kernel_telegram_runner_error, select_hepta_kernel_telegram_runner,
};

pub type NativeTelegramModelRunnerPlan = HeptaKernelTelegramRunnerPlan;
pub type NativeTelegramModelRunnerInvocationOutcome = HeptaKernelTelegramRunnerInvocationOutcome;
pub type NativeTelegramSessionBridgePlan = HeptaKernelTelegramSessionBridgePlan;
pub type NativeTelegramDrainFinalStatusPlan = HeptaKernelTelegramDrainFinalStatusPlan;

pub fn invoke_native_telegram_model_runner_with_plan<M, I, C>(
    plan: &NativeTelegramModelRunnerPlan,
    prompt: &str,
    run_mlx_local: M,
    run_in_process: I,
    run_child_process: C,
) -> NativeTelegramModelRunnerInvocationOutcome
where
    M: FnOnce(&str, &NativeTelegramModelRunnerPlan) -> Result<String, String>,
    I: FnOnce(&str) -> Result<String, String>,
    C: FnOnce(&str) -> Result<String, String>,
{
    invoke_hepta_kernel_telegram_runner_with_plan(
        plan,
        prompt,
        run_mlx_local,
        run_in_process,
        run_child_process,
    )
}

pub fn classify_native_telegram_model_runner_error(error: &str) -> &'static str {
    classify_hepta_kernel_telegram_runner_error(error)
}

pub fn redact_native_telegram_model_runner_error(error: &str) -> String {
    redact_hepta_kernel_telegram_runner_error(error)
}

pub fn native_telegram_model_failure_fallback_message() -> &'static str {
    HEPTA_KERNEL_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE
}

pub fn native_telegram_model_failure_fallback_allowed(
    enabled: bool,
    session_runner_invoked: bool,
    status: &str,
    reply_target_present: bool,
    candidate_next_update_offset_present: bool,
) -> bool {
    hepta_kernel_telegram_model_failure_fallback_allowed(
        enabled,
        session_runner_invoked,
        status,
        reply_target_present,
        candidate_next_update_offset_present,
    )
}

pub fn native_telegram_drain_final_status(
    model_session_runner_invoked: bool,
    model_runner_process_spawned_by_status: bool,
    send_status: &str,
    send_error: Option<&str>,
    model_status: &str,
    model_error: Option<&str>,
    previous_status: &'static str,
    previous_error: Option<&str>,
) -> NativeTelegramDrainFinalStatusPlan {
    hepta_kernel_telegram_drain_final_status(
        model_session_runner_invoked,
        model_runner_process_spawned_by_status,
        send_status,
        send_error,
        model_status,
        model_error,
        previous_status,
        previous_error,
    )
}

pub fn select_native_telegram_model_runner(
    model_ref: Option<&str>,
    mlx_base_url: Option<&str>,
    mlx_max_tokens: Option<u64>,
    in_process_runner_enabled: bool,
    codex_core_runner_enabled: bool,
) -> NativeTelegramModelRunnerPlan {
    select_hepta_kernel_telegram_runner(
        model_ref,
        mlx_base_url,
        mlx_max_tokens,
        in_process_runner_enabled,
        codex_core_runner_enabled,
    )
}

pub fn native_telegram_codex_core_prompt(
    prompt: &str,
    hepta_intelligence_context: bool,
    plugin_capability_context: bool,
) -> Result<String, String> {
    native_telegram_hepta_kernel_prompt(
        prompt,
        hepta_intelligence_context,
        plugin_capability_context,
    )
}

pub fn native_telegram_hepta_kernel_prompt(
    prompt: &str,
    hepta_intelligence_context: bool,
    plugin_capability_context: bool,
) -> Result<String, String> {
    hepta_kernel_telegram_prompt(
        prompt,
        hepta_intelligence_context,
        plugin_capability_context,
    )
}

pub fn parse_native_telegram_mlx_model_ref(model_ref: &str) -> Option<String> {
    parse_hepta_kernel_mlx_model_ref(model_ref)
}

pub fn native_telegram_mlx_chat_completion_body(
    model: &str,
    prompt: &str,
    max_tokens: u64,
) -> Result<Value, String> {
    hepta_kernel_mlx_chat_completion_body(model, prompt, max_tokens)
}

pub fn extract_native_telegram_openai_chat_completion_text(body: &Value) -> Result<String, String> {
    extract_hepta_kernel_openai_chat_completion_text(body)
}

pub fn native_telegram_exec_child_args(last_message_path: &Path, prompt: &str) -> Vec<String> {
    hepta_kernel_exec_child_args(last_message_path.to_string_lossy().as_ref(), prompt)
}

pub fn native_telegram_model_timeout(value_ms: Option<u64>) -> Duration {
    hepta_kernel_telegram_model_timeout(value_ms)
}

pub fn extract_native_telegram_exec_child_final_message(output: &str) -> Result<String, String> {
    extract_hepta_kernel_exec_child_final_message(output)
}

pub fn wait_for_native_telegram_model_child(
    child: &mut Child,
    timeout: Duration,
) -> Result<ExitStatus, String> {
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

pub fn native_telegram_exec_child_status_error(status: ExitStatus) -> Option<String> {
    hepta_kernel_exec_child_status_error(status.success(), status.code())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn mlx_runner_plan_requires_provider_prefix_and_does_not_spawn_process() {
        let plan = select_native_telegram_model_runner(
            Some(" mlx-local/froggeric/Qwen3.6-35B-A3B-Uncensored-Heretic-MLX-4bit "),
            Some(" http://127.0.0.1:11436/v1/ "),
            Some(8_000),
            true,
            false,
        );

        assert!(plan.runner_plan_ready);
        assert_eq!(plan.runner_kind, "mlx_local_chat_completions");
        assert!(!plan.codex_core_runner_enabled);
        assert_eq!(
            plan.mlx_model.as_deref(),
            Some("froggeric/Qwen3.6-35B-A3B-Uncensored-Heretic-MLX-4bit")
        );
        assert_eq!(
            plan.mlx_base_url.as_deref(),
            Some("http://127.0.0.1:11436/v1")
        );
        assert_eq!(plan.mlx_max_tokens, Some(MAX_TELEGRAM_MLX_MAX_TOKENS));
        assert!(plan.local_network_call);
        assert!(!plan.process_spawned_by_status);
        assert!(!plan.raw_prompt_text_exposed);
    }

    #[test]
    fn runner_plan_can_force_codex_core_session_over_mlx_model_ref() {
        let plan = select_native_telegram_model_runner(
            Some("mlx-local/local-model"),
            Some("http://127.0.0.1:11436/v1"),
            Some(128),
            false,
            true,
        );

        assert_eq!(plan.runner_kind, HEPTA_KERNEL_TELEGRAM_RUNNER_KIND);
        assert!(plan.codex_core_runner_enabled);
        assert!(plan.in_process_runner_enabled);
        assert!(!plan.local_network_call);
        assert!(!plan.process_spawned_by_status);
        assert!(plan.hepta_intelligence_context_injected);
        assert!(plan.plugin_capability_context_injected);
    }

    #[test]
    fn runner_plan_prefers_in_process_only_without_mlx_model_ref() {
        let plan =
            select_native_telegram_model_runner(Some("openai/gpt-5.5"), None, None, true, false);

        assert_eq!(plan.runner_kind, "hepta_in_process_exec_runner");
        assert!(plan.in_process_runner_enabled);
        assert!(plan.hepta_intelligence_context_injected);
        assert!(plan.plugin_capability_context_injected);
        assert!(!plan.local_network_call);
        assert!(!plan.process_spawned_by_status);
    }

    #[test]
    fn runner_plan_defaults_to_child_process_without_mlx_or_in_process() {
        let plan = select_native_telegram_model_runner(None, None, None, false, false);

        assert_eq!(plan.runner_kind, "hepta_exec_child_runner");
        assert!(!plan.in_process_runner_enabled);
        assert!(!plan.local_network_call);
        assert!(plan.process_spawned_by_status);
    }

    #[test]
    fn mlx_model_ref_parser_rejects_empty_or_other_providers() {
        assert_eq!(parse_native_telegram_mlx_model_ref("mlx-local/   "), None);
        assert_eq!(
            parse_native_telegram_mlx_model_ref("openai/gpt-5.5").as_deref(),
            None
        );
    }

    #[test]
    fn mlx_chat_completion_body_is_bounded_and_openai_compatible() {
        let body =
            native_telegram_mlx_chat_completion_body("local-model", " private prompt ", 999_999)
                .expect("request body");

        assert_eq!(body["model"], "local-model");
        assert_eq!(body["messages"][0]["role"], "system");
        assert_eq!(body["messages"][1]["role"], "user");
        assert_eq!(body["messages"][1]["content"], "private prompt");
        assert_eq!(body["max_tokens"], MAX_TELEGRAM_MLX_MAX_TOKENS);
        assert_eq!(body["stream"], false);
        assert_eq!(body["strip_thinking"], true);

        assert!(
            native_telegram_mlx_chat_completion_body("   ", "prompt", 12)
                .expect_err("empty model rejected")
                .contains("selected model")
        );
        assert!(
            native_telegram_mlx_chat_completion_body("model", "   ", 12)
                .expect_err("empty prompt rejected")
                .contains("non-empty prompt")
        );
    }

    #[test]
    fn openai_chat_completion_text_extractor_accepts_message_or_text() {
        let chat = json!({
            "choices": [{
                "message": { "role": "assistant", "content": "  local reply  " }
            }]
        });
        assert_eq!(
            extract_native_telegram_openai_chat_completion_text(&chat).expect("chat content"),
            "local reply"
        );

        let completion = json!({
            "choices": [{ "text": "  completion reply  " }]
        });
        assert_eq!(
            extract_native_telegram_openai_chat_completion_text(&completion)
                .expect("completion text"),
            "completion reply"
        );

        let missing = json!({ "choices": [{ "message": { "content": "   " }}]});
        assert!(
            extract_native_telegram_openai_chat_completion_text(&missing)
                .expect_err("empty text rejected")
                .contains("did not include text")
        );
    }

    #[test]
    fn hepta_kernel_prompt_wraps_telegram_text_with_intelligence_and_plugin_context() {
        let prompt =
            native_telegram_codex_core_prompt("  解释一下架构  ", true, true).expect("prompt");

        assert!(prompt.contains("Hepta kernel owns the turn loop"));
        assert!(prompt.contains("Codex is an internal execution engine"));
        assert!(prompt.contains("Hepta intelligence stage"));
        assert!(prompt.contains("Plugin capability stage"));
        assert!(prompt.contains("Inbound Telegram user message:\n解释一下架构"));
        assert!(
            native_telegram_codex_core_prompt("  ", true, true)
                .expect_err("empty prompt rejected")
                .contains("non-empty prompt")
        );
    }

    #[test]
    fn exec_child_args_are_ephemeral_read_only_and_capture_last_message() {
        let last_message_path = Path::new("/tmp/hepta-telegram-last-message.txt");
        let args = native_telegram_exec_child_args(last_message_path, "private prompt");

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
    fn model_timeout_policy_clamps_and_defaults() {
        assert_eq!(
            native_telegram_model_timeout(None),
            Duration::from_millis(DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS)
        );
        assert_eq!(
            native_telegram_model_timeout(Some(1)),
            Duration::from_millis(MIN_TELEGRAM_MODEL_TIMEOUT_MS)
        );
        assert_eq!(
            native_telegram_model_timeout(Some(999_999_999)),
            Duration::from_millis(MAX_TELEGRAM_MODEL_TIMEOUT_MS)
        );
    }

    #[test]
    fn exec_child_final_message_extractor_trims_and_rejects_empty() {
        assert_eq!(
            extract_native_telegram_exec_child_final_message("  final answer \n")
                .expect("final message"),
            "final answer"
        );
        assert!(
            extract_native_telegram_exec_child_final_message(" \n\t ")
                .expect_err("empty output rejected")
                .contains("empty final message")
        );
    }

    #[test]
    fn exec_child_wait_and_status_helpers_report_process_outcomes() {
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg("exit 0")
            .spawn()
            .expect("spawn child");
        let status = wait_for_native_telegram_model_child(&mut child, Duration::from_secs(5))
            .expect("wait child");
        assert!(status.success());
        assert_eq!(native_telegram_exec_child_status_error(status), None);

        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg("exit 7")
            .status()
            .expect("status child");
        assert!(
            native_telegram_exec_child_status_error(status)
                .expect("nonzero status")
                .contains("7")
        );
    }

    #[test]
    fn invocation_facade_selects_mlx_and_redacts_runner_errors() {
        let plan = select_native_telegram_model_runner(
            Some("mlx-local/local-model"),
            Some("http://127.0.0.1:11436/v1"),
            Some(128),
            false,
            false,
        );

        let outcome = invoke_native_telegram_model_runner_with_plan(
            &plan,
            " private prompt ",
            |prompt, selected_plan| {
                assert_eq!(prompt, "private prompt");
                assert_eq!(selected_plan.runner_kind, "mlx_local_chat_completions");
                Err(
                    "local MLX chat-completions HTTP status 500; token 123456:ABCDEFGHIJKLMNOPQRSTUVWX"
                        .to_string(),
                )
            },
            |_| panic!("in-process runner must not be selected for mlx-local"),
            |_| panic!("child runner must not be selected for mlx-local"),
        );

        assert_eq!(outcome.status, "attention");
        assert!(outcome.runner_invoked);
        assert!(outcome.local_network_call);
        assert!(!outcome.local_process_spawned);
        assert_eq!(outcome.error_kind, Some("local_mlx_http_status"));
        let error = outcome.error.expect("redacted error");
        assert!(error.contains("telegram_model_runner_error[local_mlx_http_status]"));
        assert!(error.contains("[redacted-telegram-token]"));
        assert!(!error.contains("ABCDEFGHIJKLMNOPQRSTUVWX"));
        assert!(!outcome.raw_prompt_text_exposed);
        assert!(!outcome.raw_response_text_exposed);
    }

    #[test]
    fn invocation_facade_reports_child_process_spawn_and_trims_output() {
        let plan = select_native_telegram_model_runner(None, None, None, false, false);

        let outcome = invoke_native_telegram_model_runner_with_plan(
            &plan,
            " private prompt ",
            |_, _| panic!("mlx runner must not be selected for default child plan"),
            |_| panic!("in-process runner must not be selected for default child plan"),
            |prompt| {
                assert_eq!(prompt, "private prompt");
                Ok(" child reply \n".to_string())
            },
        );

        assert_eq!(outcome.status, "completed");
        assert!(outcome.runner_invoked);
        assert!(!outcome.local_network_call);
        assert!(outcome.local_process_spawned);
        assert_eq!(outcome.model_output.as_deref(), Some("child reply"));
        assert_eq!(outcome.into_result().expect("model output"), "child reply");
    }

    #[test]
    fn invocation_facade_rejects_empty_prompt_before_runner() {
        let plan = select_native_telegram_model_runner(None, None, None, true, false);

        let outcome = invoke_native_telegram_model_runner_with_plan(
            &plan,
            " \n ",
            |_, _| panic!("mlx runner must not run for empty prompt"),
            |_| panic!("in-process runner must not run for empty prompt"),
            |_| panic!("child runner must not run for empty prompt"),
        );

        assert_eq!(outcome.status, "attention");
        assert!(!outcome.runner_invoked);
        assert_eq!(outcome.error_kind, Some("empty_prompt"));
    }

    #[test]
    fn model_runner_error_classifier_distinguishes_common_failures() {
        assert_eq!(
            classify_native_telegram_model_runner_error("gated runner timed out after 1000 ms"),
            "timeout"
        );
        assert_eq!(
            classify_native_telegram_model_runner_error("failed to spawn gated Hepta exec runner"),
            "child_spawn"
        );
        assert_eq!(
            classify_native_telegram_model_runner_error(
                "gated Hepta exec runner exited with status 7"
            ),
            "child_exit"
        );
        assert_eq!(
            classify_native_telegram_model_runner_error("failed to parse local MLX response JSON"),
            "local_mlx_parse"
        );
    }
}
