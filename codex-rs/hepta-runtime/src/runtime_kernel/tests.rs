use super::ApprovalRequirement;
use super::DoctorStatus;
use super::MergeOptions;
use super::ProviderTransportKind;
use super::RollbackGroupAttempt;
use super::RollbackGroupAttemptStatus;
use super::RuntimeKernel;
use super::ToolRegistry;
use super::WriteGroupLock;
use super::WriteTargetLock;
use super::WriteTransactionEntry;
use super::WriteTransactionGroup;
use super::current_unix_ms;
use super::extract_explicit_exec_tool_call;
use super::extract_explicit_process_tool_call;
use super::looks_like_assistant_identity_intent;
use super::looks_like_model_identity_intent;
use super::merge_runtime_config_value;
use super::native_pre_model_tool_call;
use super::preview_backup_path_from_ts;
use super::preview_transaction_checkpoint_path;
use super::render_native_tool_result_reply;
use super::should_offer_model_tools_for_turn;
use hepta_core::CorrelationId;
use hepta_core::EventKind;
use hepta_core::ExecutionProfile;
use hepta_core::FilesystemScope;
use hepta_core::IntuitionFeedbackOutcome;
use hepta_core::MemoryRecord;
use hepta_core::MemoryScope;
use hepta_core::MemoryStore;
use hepta_core::MessageRole;
use hepta_core::ModelMessage;
use hepta_core::ModelRef;
use hepta_core::ModelRequest;
use hepta_core::ModelToolSpec;
use hepta_core::SessionId;
use hepta_core::ThinkingLevel;
use hepta_core::ToolCallRequest;
use hepta_core::ToolContext;
use hepta_core::WritePathScope;
use hepta_intelligence::TopicAwareModelFeedbackOutcome;
use serde_json::Value;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

fn extract_json_string_field(json_text: &str, field: &str) -> Option<String> {
    serde_json::from_str::<Value>(json_text)
        .ok()?
        .get(field)?
        .as_str()
        .map(|value| value.to_string())
}

fn architecture_foundation_read_intent() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/decisions/ADR-0001-architecture-foundation.md")
        .canonicalize()
        .expect("architecture foundation ADR should resolve inside the workspace");
    format!("read:{}", path.display())
}

fn test_artifact_path(file_name: impl AsRef<std::path::Path>) -> PathBuf {
    crate::tool_workspace_root_path()
        .join("artifacts")
        .join(file_name)
}

fn provider_test_context(session: &str, correlation: &str) -> ToolContext {
    let attempt_id = uuid::Uuid::new_v4().to_string();
    ToolContext {
        session_id: Some(SessionId(session.into())),
        correlation_id: Some(CorrelationId(correlation.into())),
        idempotency_key: Some(format!(
            "hepta-execution:{attempt_id}:sha256:{}",
            "a".repeat(64)
        )),
        execution_attempt_id: Some(attempt_id),
    }
}

fn selected_context_recall_block(rendered: &str) -> Option<&str> {
    let (_, rest) = rendered.split_once("<selected_context_recall>")?;
    let (block, _) = rest.split_once("</selected_context_recall>")?;
    Some(block)
}

fn write_fake_workspace_backup(logical_path: &str, ts: u64, content: &str) -> PathBuf {
    let backup_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/backups/write_file/workspace")
        .join(format!("{}.hepta-bak-{}", logical_path, ts));
    fs::create_dir_all(backup_path.parent().expect("backup parent should exist"))
        .expect("backup parent should be creatable");
    fs::write(&backup_path, content).expect("backup file should be writable");
    backup_path
}

#[test]
fn runtime_config_merge_preserves_hepta_and_adds_source_runtime_increment() {
    let mut hepta_runtime = json!({
        "models": {
            "providers": {
                "mlx-local": {
                    "baseUrl": "http://hepta-runtime.local/v1",
                    "models": ["Gemma-A"]
                }
            }
        },
        "tools": {
            "allow": ["read"]
        }
    });
    let source_runtime_import = json!({
        "models": {
            "providers": {
                "mlx-local": {
                    "baseUrl": "http://source-runtime-import.local/v1",
                    "apiKey": "redacted-secret",
                    "models": ["Gemma-A", "Gemma-B"]
                },
                "ollama": {
                    "baseUrl": "http://localhost:11434/v1",
                    "models": ["llama"]
                }
            }
        },
        "tools": {
            "allow": ["read", "web_search"]
        }
    });

    merge_runtime_config_value(&mut hepta_runtime, source_runtime_import);

    assert_eq!(
        hepta_runtime["models"]["providers"]["mlx-local"]["baseUrl"],
        json!("http://hepta-runtime.local/v1")
    );
    assert_eq!(
        hepta_runtime["models"]["providers"]["mlx-local"]["apiKey"],
        json!("redacted-secret")
    );
    assert_eq!(
        hepta_runtime["models"]["providers"]["mlx-local"]["models"],
        json!(["Gemma-A", "Gemma-B"])
    );
    assert_eq!(
        hepta_runtime["models"]["providers"]["ollama"]["baseUrl"],
        json!("http://localhost:11434/v1")
    );
    assert_eq!(
        hepta_runtime["tools"]["allow"],
        json!(["read", "web_search"])
    );
}

#[test]
fn path_overlap_matches_same_ancestor_and_descendant_paths() {
    let base = PathBuf::from("/tmp/hepta-lock-root");
    let child = base.join("nested/file.txt");
    assert!(super::paths_overlap(&base, &base));
    assert!(super::paths_overlap(&base, &child));
    assert!(super::paths_overlap(&child, &base));
    assert!(!super::paths_overlap(
        &PathBuf::from("/tmp/hepta-a"),
        &PathBuf::from("/tmp/hepta-b")
    ));
}

#[tokio::test]
async fn switches_provider_and_routes_run_through_new_provider() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_model("mock-ollama/local-precise")
        .expect("switch should succeed");

    let result = runtime
        .run_demo_turn("tool:provider route")
        .await
        .expect("demo turn should succeed");

    assert_eq!(result.active_model.provider, "mock-ollama");
    assert_eq!(result.active_model.model, "local-precise");
    assert!(result.final_text.contains("[ollama-precise]"));
    assert!(result.final_text.contains("结构化结果已保留在本地"));
    assert!(!result.final_text.contains("structured="));
}

#[tokio::test]
async fn native_turn_messages_with_context_recall_handoff_consumes_opted_in_runtime_handoff_without_leak()
 {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .memory
        .put(MemoryRecord {
            id: "native-handoff-source-id".into(),
            scope: MemoryScope::LongTerm,
            content: format!("needle {}", "safe-context ".repeat(80)),
        })
        .await
        .expect("memory should store");
    runtime
        .memory
        .put(MemoryRecord {
            id: "native-handoff-control".into(),
            scope: MemoryScope::LongTerm,
            content: "[hepta-memory:tombstone] needle retired memory".into(),
        })
        .await
        .expect("control memory should store");

    let handoff = runtime
        .native_turn_messages_with_context_recall_handoff(
            "alpha", "needle", /*experimental_api_enabled*/ true,
        )
        .await
        .expect("native turn handoff should build");
    let rendered = handoff
        .messages
        .iter()
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let debug = format!("{handoff:?}");

    assert!(
        handoff
            .provider_rollup
            .recall_selection
            .has_count_integrity()
    );
    assert!(handoff.selected_snippets_present);
    assert!(handoff.selected_snippet_count > 0);
    assert_eq!(rendered.matches("<selected_context_recall>").count(), 1);
    let selected_block =
        selected_context_recall_block(&rendered).expect("selected context block should exist");
    assert!(selected_block.contains("[redacted-query]"));
    assert!(!selected_block.contains("needle"));
    assert!(!selected_block.contains("native-handoff-source-id"));
    assert!(!selected_block.contains("native-handoff-control"));
    assert!(!selected_block.contains("[hepta-memory:"));
    assert!(!selected_block.contains("source_id"));
    assert!(!selected_block.contains("source_memory_ids"));
    assert!(!selected_block.contains("query_payload"));
    assert!(!selected_block.contains("summary"));
    assert!(!selected_block.contains("reason"));
    assert!(!debug.contains("[redacted-query]"));
    assert!(!debug.contains("safe-context"));
    assert!(!debug.contains("native-handoff-source-id"));
}

#[tokio::test]
async fn native_turn_messages_with_context_recall_handoff_requires_opt_in_and_prompt_safe_text() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .memory
        .put(MemoryRecord {
            id: "native-handoff-unsafe".into(),
            scope: MemoryScope::LongTerm,
            content: "needle source_id: unsafe metadata should be dropped".into(),
        })
        .await
        .expect("memory should store");

    let unsafe_handoff = runtime
        .native_turn_messages_with_context_recall_handoff(
            "alpha", "needle", /*experimental_api_enabled*/ true,
        )
        .await
        .expect("unsafe native turn handoff should build");
    let unsafe_rendered = unsafe_handoff
        .messages
        .iter()
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        unsafe_handoff
            .provider_rollup
            .recall_selection
            .has_count_integrity()
    );
    assert!(!unsafe_handoff.selected_snippets_present);
    assert_eq!(unsafe_handoff.selected_snippet_count, 0);
    assert!(selected_context_recall_block(&unsafe_rendered).is_none());

    let no_opt_in_handoff = runtime
        .native_turn_messages_with_context_recall_handoff(
            "alpha", "needle", /*experimental_api_enabled*/ false,
        )
        .await
        .expect("no-opt-in native turn handoff should build");
    let no_opt_in_rendered = no_opt_in_handoff
        .messages
        .iter()
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        no_opt_in_handoff
            .provider_rollup
            .recall_selection
            .has_count_integrity()
    );
    assert!(!no_opt_in_handoff.selected_snippets_present);
    assert_eq!(no_opt_in_handoff.selected_snippet_count, 0);
    assert!(selected_context_recall_block(&no_opt_in_rendered).is_none());
}

#[tokio::test]
async fn run_demo_turn_in_session_with_context_recall_handoff_executes_opted_in_run_without_leak() {
    let runtime = RuntimeKernel::new();
    runtime
        .memory
        .put(MemoryRecord {
            id: "native-run-source-id".into(),
            scope: MemoryScope::LongTerm,
            content: format!("needle {}", "safe-run-context ".repeat(80)),
        })
        .await
        .expect("memory should store");

    let run = runtime
        .run_demo_turn_in_session_with_context_recall_handoff(
            "alpha", "needle", /*experimental_api_enabled*/ true,
        )
        .await
        .expect("native run handoff should execute");
    let debug = format!("{run:?}");
    let history = runtime
        .history_state
        .lock()
        .expect("history state should lock")
        .clone();

    assert!(run.provider_rollup.recall_selection.has_count_integrity());
    assert!(run.selected_snippets_present);
    assert!(run.selected_snippet_count > 0);
    assert_eq!(run.result.session_id, "alpha");
    assert_eq!(run.result.invoked_tool, None);
    assert!(run.result.final_text.contains("[chat] model reply: needle"));
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].input, "needle");
    assert!(!run.result.final_text.contains("safe-run-context"));
    assert!(!run.result.final_text.contains("native-run-source-id"));
    assert!(!debug.contains("safe-run-context"));
    assert!(!debug.contains("native-run-source-id"));
    assert!(!debug.contains("[redacted-query]"));

    let no_opt_in_runtime = RuntimeKernel::new();
    no_opt_in_runtime
        .memory
        .put(MemoryRecord {
            id: "native-run-no-opt-source-id".into(),
            scope: MemoryScope::LongTerm,
            content: format!("needle {}", "no-opt-context ".repeat(80)),
        })
        .await
        .expect("memory should store");
    let no_opt_in_run = no_opt_in_runtime
        .run_demo_turn_in_session_with_context_recall_handoff(
            "alpha", "needle", /*experimental_api_enabled*/ false,
        )
        .await
        .expect("no-opt-in native run handoff should execute");

    assert!(
        no_opt_in_run
            .provider_rollup
            .recall_selection
            .has_count_integrity()
    );
    assert!(!no_opt_in_run.selected_snippets_present);
    assert_eq!(no_opt_in_run.selected_snippet_count, 0);
}

#[test]
fn exposes_provider_catalog_separately_from_model_selection_state() {
    let runtime = RuntimeKernel::new();
    let catalog = runtime.provider_catalog();
    assert_eq!(catalog.providers.len(), 2);
    assert!(catalog.providers.iter().any(|provider| {
        provider.id == "demo"
            && provider.display_name == "Demo Provider"
            && provider.transport_kind == ProviderTransportKind::InProcess
            && provider.default_model.model == "demo-chat"
            && provider
                .available_models
                .iter()
                .any(|model| model.model == "demo-creative")
    }));
    assert!(catalog.providers.iter().any(|provider| {
        provider.id == "mock-ollama"
            && provider.transport_kind == ProviderTransportKind::OpenAiCompatibleHttp
            && provider
                .available_models
                .iter()
                .any(|model| model.model == "local-precise")
    }));

    let selection = runtime.model_selection().expect("selection should load");
    assert_eq!(selection.available.len(), 5);
    assert!(
        selection
            .available
            .iter()
            .any(|model| model.provider == "demo")
    );
    assert!(
        selection
            .available
            .iter()
            .any(|model| model.provider == "mock-ollama")
    );
}

#[tokio::test]
async fn medium_risk_tool_requires_approval_until_granted() {
    let runtime = RuntimeKernel::new();
    let read_path = architecture_foundation_read_intent();

    let blocked = runtime
        .run_demo_turn(&read_path)
        .await
        .expect("first run should return approval requirement");
    assert_eq!(blocked.approval_required.as_deref(), Some("read_file"));
    assert!(
        blocked
            .blocked_reason
            .as_deref()
            .unwrap_or_default()
            .contains("requires explicit approval")
    );

    let snapshot = runtime
        .approval_snapshot()
        .expect("snapshot should succeed");
    assert_eq!(snapshot.pending.len(), 1);
    assert_eq!(snapshot.pending[0].tool_name, "read_file");

    runtime
        .approve_tool("read_file")
        .expect("approval should succeed");

    let allowed = runtime
        .run_demo_turn(&read_path)
        .await
        .expect("second run should succeed after approval");
    assert_eq!(allowed.invoked_tool.as_deref(), Some("read_file"));
    assert!(allowed.approval_required.is_none());
    assert!(allowed.final_text.contains("read_file:"));

    let events = runtime.events(usize::MAX).expect("events should load");
    let kinds = events
        .into_iter()
        .map(|item| item.event.kind)
        .collect::<Vec<_>>();
    assert!(kinds.contains(&EventKind::ApprovalRequested));
    assert!(kinds.contains(&EventKind::ApprovalGranted));
    assert!(kinds.contains(&EventKind::ToolInvoked));
    assert!(kinds.contains(&EventKind::MemoryWritten));
}

#[tokio::test]
async fn custom_policy_rule_can_deny_low_risk_tool() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            Some("session-main"),
            Some("demo"),
            Some("echo"),
            None,
            ApprovalRequirement::Deny,
            Some("echo is blocked for session-main on demo"),
        )
        .expect("policy rule should be added");

    let blocked = runtime
        .run_demo_turn("tool:blocked echo")
        .await
        .expect("run should succeed with denial result");

    assert_eq!(blocked.invoked_tool, None);
    assert_eq!(blocked.approval_required, None);
    assert_eq!(blocked.final_text, "policy denied tool echo");
    assert_eq!(
        blocked.blocked_reason.as_deref(),
        Some("echo is blocked for session-main on demo")
    );
}

#[test]
fn openai_tool_schema_and_tool_call_parser_roundtrip() {
    let tools = vec![hepta_core::ModelToolSpec {
        name: "echo".into(),
        description: "Echo text".into(),
        input_schema_json: json!({
            "type": "object",
            "required": ["text"],
            "properties": {"text": {"type": "string"}}
        })
        .to_string(),
    }];
    let payloads = super::openai_tool_payloads(&tools);
    assert_eq!(payloads.len(), 1);
    assert_eq!(
        payloads[0].pointer("/type").and_then(Value::as_str),
        Some("function")
    );
    assert_eq!(
        payloads[0]
            .pointer("/function/name")
            .and_then(Value::as_str),
        Some("echo")
    );
    assert_eq!(
        payloads[0]
            .pointer("/function/parameters/required/0")
            .and_then(Value::as_str),
        Some("text")
    );

    let parsed = super::openai_tool_calls_from_message(&json!({
        "tool_calls": [{
            "type": "function",
            "function": {
                "name": "echo",
                "arguments": "{\"text\":\"hello\"}"
            }
        }]
    }));
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].name, "echo");
    assert_eq!(parsed[0].arguments_json, "{\"text\":\"hello\"}");

    let textual = super::textual_tool_calls_from_message_content(
        "<|tool_call>call:echo{text: \"ping\"}<tool_call|>",
        &tools,
    );
    assert_eq!(textual.len(), 1);
    assert_eq!(textual[0].name, "echo");
    assert_eq!(textual[0].arguments_json, "{\"text\":\"ping\"}");

    let json_textual = super::textual_tool_calls_from_message_content(
        r#"<tool_call>{"name":"echo","arguments":{"text":"pong"}}</tool_call>"#,
        &tools,
    );
    assert_eq!(json_textual.len(), 1);
    assert_eq!(json_textual[0].name, "echo");
    assert_eq!(json_textual[0].arguments_json, "{\"text\":\"pong\"}");
}

#[test]
fn qwen_chat_template_thinking_is_disabled_for_live_agent_requests() {
    let mut payload = json!({
        "model": "Qwen/Qwen3-8B",
        "chat_template_kwargs": {"preserve_other": true}
    });
    let request = ModelRequest {
        model: ModelRef {
            provider: "mlx-local".into(),
            model: "Qwen/Qwen3-8B".into(),
        },
        messages: vec![ModelMessage {
            role: MessageRole::User,
            content: "What's the temperature?".into(),
        }],
        thinking: ThinkingLevel::High,
        tools: vec![ModelToolSpec {
            name: "get_current_temperature".into(),
            description: "Get current temperature".into(),
            input_schema_json: json!({
                "type": "object",
                "properties": {"location": {"type": "string"}},
                "required": ["location"]
            })
            .to_string(),
        }],
        timeout_ms: None,
    };

    assert!(super::apply_qwen_openai_compatible_thinking_params(
        &mut payload,
        Some(super::QwenThinkingFormat::ChatTemplate),
        &request,
    ));
    assert_eq!(
        payload.pointer("/chat_template_kwargs/enable_thinking"),
        Some(&json!(false))
    );
    assert_eq!(
        payload.pointer("/chat_template_kwargs/preserve_other"),
        Some(&json!(true))
    );

    let mut top_level_payload = json!({"model": "qwen3"});
    let no_tool_request = ModelRequest {
        tools: vec![],
        ..request
    };
    assert!(super::apply_qwen_openai_compatible_thinking_params(
        &mut top_level_payload,
        Some(super::QwenThinkingFormat::TopLevel),
        &no_tool_request,
    ));
    assert_eq!(
        top_level_payload.get("enable_thinking"),
        Some(&json!(false))
    );
}

#[test]
fn openai_codex_jwt_account_id_decodes_without_secret_logging() {
    let token = "hdr.eyJodHRwczovL2FwaS5vcGVuYWkuY29tL2F1dGgiOnsiY2hhdGdwdF9hY2NvdW50X2lkIjoiYWNjdF90ZXN0XzEyMyJ9fQ.sig";

    assert_eq!(
        super::extract_chatgpt_account_id_from_jwt(token).as_deref(),
        Some("acct_test_123")
    );
}

#[test]
fn openai_codex_profile_selection_prefers_freshest_unexpired_profile() {
    let stale_first = super::OpenAiCodexAuthProfile {
        path: PathBuf::from("hepta/auth-profiles.json"),
        profile_id: "openai-codex:stale".into(),
        access: "stale-access".into(),
        refresh: Some("stale-refresh".into()),
        expires: Some(1_000),
        account_id: "acct_stale".into(),
    };
    let fresh_default = super::OpenAiCodexAuthProfile {
        path: PathBuf::from("main/auth-profiles.json"),
        profile_id: "openai-codex:default".into(),
        access: "fresh-access".into(),
        refresh: Some("fresh-refresh".into()),
        expires: Some(500_000),
        account_id: "acct_fresh".into(),
    };
    let freshest = super::OpenAiCodexAuthProfile {
        path: PathBuf::from("main/auth-profiles.json"),
        profile_id: "openai-codex:newest".into(),
        access: "freshest-access".into(),
        refresh: Some("freshest-refresh".into()),
        expires: Some(900_000),
        account_id: "acct_freshest".into(),
    };

    let selected = super::select_openai_codex_auth_profile(
        vec![stale_first, fresh_default, freshest],
        100_000,
    )
    .expect("a fresh profile should be selected");

    assert_eq!(selected.profile_id, "openai-codex:newest");
    assert_eq!(selected.account_id, "acct_freshest");
}

#[test]
fn openai_codex_profile_override_normalizes_email_or_full_profile_id() {
    assert_eq!(
        super::normalize_openai_codex_profile_id_override(" qiqianpkugsm@gmail.com ").as_deref(),
        Some("openai-codex:qiqianpkugsm@gmail.com")
    );
    assert_eq!(
        super::normalize_openai_codex_profile_id_override("openai-codex:qiqianpkugsm@gmail.com",)
            .as_deref(),
        Some("openai-codex:qiqianpkugsm@gmail.com")
    );
    assert_eq!(
        super::normalize_openai_codex_profile_id_override("  "),
        None
    );
}

#[test]
fn openai_codex_tool_schema_sanitizer_adds_missing_array_items() {
    let schema = json!({
        "type": "object",
        "properties": {
            "edits": {"type": "array"},
            "nested": {
                "type": "object",
                "properties": {
                    "labels": {"type": ["array", "null"]}
                }
            }
        }
    });

    let sanitized = super::sanitize_openai_codex_tool_schema(schema);

    assert_eq!(
        sanitized.pointer("/properties/edits/items"),
        Some(&json!({}))
    );
    assert_eq!(
        sanitized.pointer("/properties/nested/properties/labels/items"),
        Some(&json!({}))
    );
}

#[test]
fn openai_codex_request_body_matches_responses_shape() {
    let request = hepta_core::ModelRequest {
        model: ModelRef {
            provider: "openai-codex".into(),
            model: "gpt-5.5".into(),
        },
        messages: vec![
            hepta_core::ModelMessage {
                role: hepta_core::MessageRole::System,
                content: "Be concise".into(),
            },
            hepta_core::ModelMessage {
                role: hepta_core::MessageRole::User,
                content: "ping".into(),
            },
        ],
        thinking: hepta_core::ThinkingLevel::XHigh,
        tools: vec![hepta_core::ModelToolSpec {
            name: "echo".into(),
            description: "Echo text".into(),
            input_schema_json: json!({
                "type": "object",
                "properties": {"text": {"type": "string"}}
            })
            .to_string(),
        }],
        timeout_ms: None,
    };

    let body = super::openai_codex_responses_request_body(&request, Some("session-1"));

    assert_eq!(body.get("model").and_then(Value::as_str), Some("gpt-5.5"));
    assert_eq!(body.get("store").and_then(Value::as_bool), Some(false));
    assert_eq!(body.get("stream").and_then(Value::as_bool), Some(true));
    assert_eq!(
        body.get("instructions").and_then(Value::as_str),
        Some("Be concise")
    );
    assert_eq!(
        body.pointer("/input/0/content/0/type")
            .and_then(Value::as_str),
        Some("input_text")
    );
    assert_eq!(
        body.pointer("/text/verbosity").and_then(Value::as_str),
        Some("low")
    );
    assert_eq!(
        body.pointer("/reasoning/effort").and_then(Value::as_str),
        Some("xhigh")
    );
    assert_eq!(
        body.pointer("/tools/0/name").and_then(Value::as_str),
        Some("echo")
    );
    assert!(body.get("max_tokens").is_none());
}

#[test]
fn openai_codex_sse_text_and_usage_parse() {
    let sse = concat!(
        "data: {\"type\":\"response.created\",\"response\":{\"id\":\"resp_1\"}}\n\n",
        "data: {\"type\":\"response.output_text.delta\",\"delta\":\"你\"}\n\n",
        "data: {\"type\":\"response.output_text.delta\",\"delta\":\"好\"}\n\n",
        "data: {\"type\":\"response.completed\",\"response\":{\"status\":\"completed\",\"usage\":{\"input_tokens\":10,\"input_tokens_details\":{\"cached_tokens\":3},\"output_tokens\":2,\"total_tokens\":12}}}\n\n"
    );

    let response = super::parse_openai_codex_sse_response(sse).expect("SSE should parse");

    assert_eq!(response.finish_reason, hepta_core::FinishReason::Stop);
    assert_eq!(response.message.expect("message").content, "你好");
    assert_eq!(response.usage.input_tokens, 7);
    assert_eq!(response.usage.output_tokens, 2);
}

#[test]
fn openai_codex_sse_tool_call_parse() {
    let sse = concat!(
        "data: {\"type\":\"response.output_item.added\",\"item\":{\"type\":\"function_call\",\"name\":\"read_file\"}}\n\n",
        "data: {\"type\":\"response.function_call_arguments.delta\",\"delta\":\"{\\\"path\\\":\"}\n\n",
        "data: {\"type\":\"response.function_call_arguments.delta\",\"delta\":\"\\\"README.md\\\"}\"}\n\n",
        "data: {\"type\":\"response.output_item.done\",\"item\":{\"type\":\"function_call\",\"name\":\"read_file\"}}\n\n",
        "data: {\"type\":\"response.completed\",\"response\":{\"status\":\"completed\"}}\n\n"
    );

    let response = super::parse_openai_codex_sse_response(sse).expect("SSE should parse");

    assert_eq!(response.finish_reason, hepta_core::FinishReason::ToolCall);
    assert!(response.message.is_none());
    assert_eq!(response.tool_calls.len(), 1);
    assert_eq!(response.tool_calls[0].name, "read_file");
    assert_eq!(
        response.tool_calls[0].arguments_json,
        "{\"path\":\"README.md\"}"
    );
}

#[tokio::test]
async fn two_turn_memory_context_is_injected_into_model_prompt() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn("请记住暗号是蓝莓")
        .await
        .expect("first turn should succeed");
    let recalled = runtime
        .run_demo_turn("暗号是什么")
        .await
        .expect("second turn should succeed");

    assert!(recalled.final_text.contains("蓝莓"));
    assert!(recalled.recalled_memories >= 1);
}

#[tokio::test]
async fn generic_read_only_tool_call_runs_through_tool_loop() {
    let runtime = RuntimeKernel::new();
    let result = runtime
        .run_demo_turn("tool:generic read only")
        .await
        .expect("echo tool should run");

    assert_eq!(result.invoked_tool.as_deref(), Some("echo"));
    assert!(result.final_text.contains("结构化结果已保留在本地"));
    assert!(!result.final_text.contains("structured="));
}

#[tokio::test]
async fn write_tool_still_requires_approval_before_mutation() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::Ask,
            Some("test write approval gate"),
        )
        .expect("policy rule should be added");
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-approval-gate-test.txt");
    let _ = fs::remove_file(&path);

    let result = runtime
        .run_demo_turn("write:artifacts/hepta-approval-gate-test.txt => blocked")
        .await
        .expect("write request should return approval gate");

    assert_eq!(result.invoked_tool, None);
    assert_eq!(result.approval_required.as_deref(), Some("write_file"));
    assert!(result.final_text.contains("approval required"));
    assert!(!path.exists(), "write_file must not mutate before approval");
}

#[tokio::test]
async fn disk_junk_audit_is_read_only_and_does_not_delete() {
    let runtime = RuntimeKernel::new();
    let result = runtime
        .run_demo_turn("你扫一眼全盘，看看有什么垃圾可以清理")
        .await
        .expect("disk junk audit should run");

    assert_eq!(result.invoked_tool.as_deref(), Some("disk_junk_audit"));
    let output_json = result
        .tool_output_json
        .expect("audit output should be structured");
    let value: Value = serde_json::from_str(&output_json).expect("audit JSON should parse");
    assert_eq!(value.get("read_only").and_then(Value::as_bool), Some(true));
    assert_eq!(
        value.get("status").and_then(Value::as_str),
        Some("completed")
    );
    assert!(result.final_text.contains("没删任何文件"));
}

#[test]
fn explicit_exec_intent_is_quarantined_before_model_routing() {
    let input =
        "请必须调用 exec 工具后台运行：sleep 1 && echo hepta-ok；然后再调用 process log 查看结果。";
    let parsed = extract_explicit_exec_tool_call(input)
        .expect("test-only parser keeps the quarantined implementation covered");

    assert_eq!(parsed.name, "exec");
    let args: Value = serde_json::from_str(&parsed.arguments_json).expect("valid JSON args");
    assert_eq!(
        args.get("command").and_then(Value::as_str),
        Some("sleep 1 && echo hepta-ok")
    );
    assert_eq!(args.get("background").and_then(Value::as_bool), Some(true));
    assert!(native_pre_model_tool_call(input).is_none());
    assert!(!should_offer_model_tools_for_turn(input));
}

#[test]
fn explicit_echo_intent_extracts_required_text_before_model() {
    let call = native_pre_model_tool_call("请用 echo 工具返回 ping，不要只用文字回答。")
        .expect("explicit echo intent should be routed before the model");

    assert_eq!(call.name, "echo");
    let args: Value = serde_json::from_str(&call.arguments_json).expect("valid JSON args");
    assert_eq!(args.get("text").and_then(Value::as_str), Some("ping"));

    let json_call = native_pre_model_tool_call(
        "Use the echo tool with arguments exactly {\"text\":\"pong\"}. Do not answer directly.",
    )
    .expect("explicit JSON echo intent should be routed before the model");
    let json_args: Value =
        serde_json::from_str(&json_call.arguments_json).expect("valid JSON args");
    assert_eq!(json_args.get("text").and_then(Value::as_str), Some("pong"));
}

#[tokio::test]
async fn qwen_style_natural_echo_request_runs_without_missing_text() {
    let runtime = RuntimeKernel::new();
    let result = runtime
        .run_demo_turn("请用 echo 工具返回 ping，不要只用文字回答。")
        .await
        .expect("echo route should run");

    assert_eq!(result.invoked_tool.as_deref(), Some("echo"));
    assert!(result.blocked_reason.is_none());
    assert!(result.final_text.contains("结构化结果已保留在本地"));
}

#[test]
fn explicit_process_intent_is_quarantined_before_model_routing() {
    let input = "请调用 process 工具 log hepta-proc-1778630000-12345 查看输出";
    let parsed = extract_explicit_process_tool_call(input)
        .expect("test-only parser keeps the quarantined implementation covered");

    assert_eq!(parsed.name, "process");
    let args: Value = serde_json::from_str(&parsed.arguments_json).expect("valid JSON args");
    assert_eq!(args.get("action").and_then(Value::as_str), Some("log"));
    assert_eq!(
        args.get("sessionId").and_then(Value::as_str),
        Some("hepta-proc-1778630000-12345")
    );
    assert!(native_pre_model_tool_call(input).is_none());
    assert!(!should_offer_model_tools_for_turn(input));
}

#[test]
fn model_identity_question_is_not_native_process_intent() {
    assert!(native_pre_model_tool_call("你是什么模型").is_none());
    assert!(looks_like_model_identity_intent(
        "BodyForHeptaAgent:\n你用的是哪个模型"
    ));
    assert!(!looks_like_model_identity_intent("请列出可用模型列表"));
}

#[test]
fn assistant_identity_question_is_not_native_process_intent() {
    assert!(native_pre_model_tool_call("你是谁").is_none());
    assert!(looks_like_assistant_identity_intent(
        "BodyForHeptaAgent:\n你是谁"
    ));
    assert!(looks_like_assistant_identity_intent("who are you"));
    assert!(!looks_like_assistant_identity_intent(
        "请调用 process 工具 list"
    ));
}

#[test]
fn model_tools_are_only_offered_for_explicit_tool_turns() {
    assert!(!should_offer_model_tools_for_turn("你好，随便聊两句"));
    assert!(!should_offer_model_tools_for_turn(
        "BodyForHeptaAgent:\n你是谁"
    ));
    assert!(!should_offer_model_tools_for_turn(
        "BodyForHeptaAgent:\n你是什么模型"
    ));
    assert!(!should_offer_model_tools_for_turn("你有哪些工具可以用？"));

    assert!(should_offer_model_tools_for_turn(
        "请用 echo 工具返回 ping，不要只用文字回答。"
    ));
    assert!(!should_offer_model_tools_for_turn(
        "请调用 process 工具 list"
    ));
    assert!(!should_offer_model_tools_for_turn(
        "Use the exec tool to run: printf unsafe"
    ));
    assert!(!should_offer_model_tools_for_turn("exec: printf unsafe"));
    assert!(!should_offer_model_tools_for_turn("tool: exec"));
    assert!(!should_offer_model_tools_for_turn("tool:process"));
    assert!(!should_offer_model_tools_for_turn("tool : process"));
    assert!(should_offer_model_tools_for_turn(
        "Use the write_file tool with arguments exactly {\"path\":\"artifacts/a.txt\",\"content\":\"x\",\"mode\":\"create\"}."
    ));
    assert!(should_offer_model_tools_for_turn("read:README.md"));
}

#[tokio::test]
async fn model_identity_question_answers_without_tool_call() {
    let runtime = RuntimeKernel::new();
    let result = runtime
        .run_demo_turn_in_session("agent:main:telegram:direct:test", "你是什么模型")
        .await
        .expect("model identity question should run");
    let active_model_label = format!(
        "{}/{}",
        result.active_model.provider, result.active_model.model
    );

    assert_eq!(result.invoked_tool, None);
    assert!(result.approval_required.is_none());
    assert!(result.blocked_reason.is_none());
    assert!(result.final_text.contains(&active_model_label));
    assert!(!result.final_text.contains("native process"));
    assert!(!result.final_text.contains("后台进程记录"));
}

#[tokio::test]
async fn ordinary_chat_answers_without_tool_surface() {
    let runtime = RuntimeKernel::new();
    let result = runtime
        .run_demo_turn_in_session("agent:main:telegram:direct:test", "你好，随便聊两句")
        .await
        .expect("ordinary chat should run");

    assert_eq!(result.invoked_tool, None);
    assert!(result.approval_required.is_none());
    assert!(result.blocked_reason.is_none());
    assert!(!result.final_text.contains("native process"));
    assert!(!result.final_text.contains("后台进程记录"));
}

#[tokio::test]
async fn assistant_identity_question_answers_without_tool_call() {
    let runtime = RuntimeKernel::new();
    let result = runtime
        .run_demo_turn_in_session("agent:main:telegram:direct:test", "你是谁")
        .await
        .expect("assistant identity question should run");

    assert_eq!(result.invoked_tool, None);
    assert!(result.approval_required.is_none());
    assert!(result.blocked_reason.is_none());
    assert!(result.final_text.contains("发发_1"));
    assert!(result.final_text.contains("Hepta"));
    assert!(result.final_text.contains("没有调用工具"));
    assert!(!result.final_text.contains("native process"));
    assert!(!result.final_text.contains("后台进程记录"));
}

#[test]
fn explicit_write_file_intent_extracts_json_before_model() {
    let call = native_pre_model_tool_call(
                r#"Use the write_file tool with arguments exactly {"path":"artifacts/hepta-live-agent-e2e-approval.txt","content":"blocked-before-approval","mode":"create"}. Do not answer directly."#,
            )
            .expect("explicit write_file intent should be routed before the model");

    assert_eq!(call.name, "write_file");
    let args: Value = serde_json::from_str(&call.arguments_json).expect("valid JSON args");
    assert_eq!(
        args.get("path").and_then(Value::as_str),
        Some("artifacts/hepta-live-agent-e2e-approval.txt")
    );
    assert_eq!(
        args.get("content").and_then(Value::as_str),
        Some("blocked-before-approval")
    );
}

#[tokio::test]
async fn custom_policy_rule_can_allow_medium_risk_tool_for_provider() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_model("mock-ollama/local-chat")
        .expect("model switch should succeed");
    runtime
        .add_policy_rule(
            None,
            Some("mock-ollama"),
            Some("read_file"),
            None,
            ApprovalRequirement::None,
            Some("mock ollama can read files without approval"),
        )
        .expect("policy rule should be added");

    let read_path = architecture_foundation_read_intent();
    let allowed = runtime
        .run_demo_turn(&read_path)
        .await
        .expect("run should succeed without approval");

    assert_eq!(allowed.invoked_tool.as_deref(), Some("read_file"));
    assert!(allowed.approval_required.is_none());

    let report = runtime
        .policy_report()
        .await
        .expect("policy report should load");
    assert_eq!(report.custom_rules.len(), 1);
    assert!(report.effective_tool_decisions.iter().any(|item| {
        item.tool_name == "read_file"
            && item.requirement == ApprovalRequirement::None
            && item
                .matched_rule_id
                .as_deref()
                .unwrap_or_default()
                .starts_with("policy-")
    }));
}

#[tokio::test]
async fn quarantined_exec_intent_never_enters_the_production_tool_loop() {
    let runtime = RuntimeKernel::new();
    let started = std::time::Instant::now();
    let result = runtime
        .run_demo_turn_in_session(
            "timeout-session",
            "请调用 exec 工具执行：sleep 5；timeoutMs=100",
        )
        .await
        .expect("quarantined exec request should remain a normal model turn");

    assert!(started.elapsed() < std::time::Duration::from_secs(3));
    assert_eq!(result.invoked_tool, None);
    assert!(result.tool_output_json.is_none());
    assert!(result.blocked_reason.is_none());
}

#[tokio::test]
async fn no_tools_execution_profile_blocks_even_low_risk_tools() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_execution_profile(ExecutionProfile::NoTools)
        .expect("profile switch should succeed");

    let blocked = runtime
        .run_demo_turn("tool:hello profile")
        .await
        .expect("run should return blocked result");

    assert_eq!(blocked.invoked_tool, None);
    assert_eq!(blocked.final_text, "execution profile blocked tool echo");
    assert!(
        blocked
            .blocked_reason
            .as_deref()
            .unwrap_or_default()
            .contains("execution profile no_tools blocks tool echo")
    );
}

#[test]
fn session_export_roundtrip_preserves_execution_profile() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_execution_profile(ExecutionProfile::NoTools)
        .expect("profile switch should succeed");
    let export = runtime
        .session_export("session-main")
        .expect("session export should succeed");
    assert_eq!(export.execution_profile, ExecutionProfile::NoTools);

    runtime
        .switch_execution_profile(ExecutionProfile::FullAccess)
        .expect("profile reset should succeed");
    runtime
        .apply_session_export(export)
        .expect("session import should succeed");

    assert_eq!(
        runtime
            .execution_profile_for_session("session-main")
            .expect("profile should load"),
        ExecutionProfile::NoTools
    );
}

#[tokio::test]
async fn workspace_only_filesystem_scope_blocks_reads_outside_workspace() {
    let runtime = RuntimeKernel::new();
    runtime
        .approve_tool("read_file")
        .expect("approval should succeed");

    let blocked = runtime
        .run_demo_turn("read:/etc/hosts")
        .await
        .expect("run should return blocked result");

    assert_eq!(blocked.invoked_tool, None);
    assert_eq!(
        blocked.final_text,
        "filesystem scope blocked tool read_file"
    );
    assert!(
        blocked
            .blocked_reason
            .as_deref()
            .unwrap_or_default()
            .contains(
                "filesystem scope workspace_only blocks read_file path /etc/hosts outside workspace"
            )
    );
}

#[test]
fn session_export_roundtrip_preserves_filesystem_scope() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_filesystem_scope(FilesystemScope::AnyPath)
        .expect("scope switch should succeed");
    let export = runtime
        .session_export("session-main")
        .expect("session export should succeed");
    assert_eq!(export.filesystem_scope, FilesystemScope::AnyPath);

    runtime
        .switch_filesystem_scope(FilesystemScope::WorkspaceOnly)
        .expect("scope reset should succeed");
    runtime
        .apply_session_export(export)
        .expect("session import should succeed");

    assert_eq!(
        runtime
            .filesystem_scope_for_session("session-main")
            .expect("scope should load"),
        FilesystemScope::AnyPath
    );
}

#[tokio::test]
async fn path_capability_gate_can_override_workspace_only_for_read_file() {
    let runtime = RuntimeKernel::new();
    runtime
        .approve_tool("read_file")
        .expect("approval should succeed");
    runtime
        .set_path_capability_gate("read_file", "path", FilesystemScope::AnyPath)
        .expect("capability gate should be set");
    let hosts = fs::canonicalize("/etc/hosts").expect("canonical hosts path");

    let result = runtime
        .run_demo_turn(&format!("read:{}", hosts.display()))
        .await
        .expect("run should succeed");

    assert_eq!(result.invoked_tool.as_deref(), Some("read_file"));
    assert!(result.final_text.contains("read_file:"));
}

#[tokio::test]
async fn path_capability_gate_can_tighten_any_path_back_to_workspace_only() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_filesystem_scope(FilesystemScope::AnyPath)
        .expect("scope switch should succeed");
    runtime
        .approve_tool("read_file")
        .expect("approval should succeed");
    runtime
        .set_path_capability_gate("read_file", "path", FilesystemScope::WorkspaceOnly)
        .expect("capability gate should be set");

    let blocked = runtime
        .run_demo_turn("read:/etc/hosts")
        .await
        .expect("run should return blocked result");

    assert_eq!(blocked.invoked_tool, None);
    assert_eq!(
        blocked.final_text,
        "filesystem scope blocked tool read_file"
    );
    assert!(
        blocked
            .blocked_reason
            .as_deref()
            .unwrap_or_default()
            .contains(
                "filesystem scope workspace_only blocks read_file path /etc/hosts outside workspace"
            )
    );
}

#[test]
fn session_export_roundtrip_preserves_path_capability_gates() {
    let runtime = RuntimeKernel::new();
    let gate = runtime
        .set_path_capability_gate("read_file", "path", FilesystemScope::AnyPath)
        .expect("capability gate should be set");
    let export = runtime
        .session_export("session-main")
        .expect("session export should succeed");
    assert_eq!(export.path_capability_gates, vec![gate.clone()]);

    runtime
        .remove_path_capability_gate(&gate.id)
        .expect("capability gate remove should succeed");
    runtime
        .apply_session_export(export)
        .expect("session import should succeed");

    assert_eq!(
        runtime
            .path_capability_gates_for_session("session-main")
            .expect("gates should load"),
        vec![gate]
    );
}

#[tokio::test]
async fn artifacts_only_write_scope_allows_writes_under_artifacts() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let result = runtime
        .run_demo_turn("write:artifacts/hepta-write-scope-test.txt => hello artifacts")
        .await
        .expect("write should succeed");

    assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
    let written = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-write-scope-test.txt");
    let content = fs::read_to_string(&written).expect("artifact file should exist");
    assert_eq!(content, "hello artifacts");
    let _ = fs::remove_file(&written);
}

#[tokio::test]
async fn artifacts_only_write_scope_blocks_workspace_source_paths() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let blocked = runtime
        .run_demo_turn("write:docs/hepta-write-scope-test.txt => blocked")
        .await
        .expect("run should return blocked result");

    assert_eq!(blocked.invoked_tool, None);
    assert_eq!(
        blocked.final_text,
        "write path scope blocked tool write_file"
    );
    assert!(blocked
                .blocked_reason
                .as_deref()
                .unwrap_or_default()
                .contains("write path scope artifacts_only blocks write_file path docs/hepta-write-scope-test.txt outside artifacts root"));
}

#[tokio::test]
async fn workspace_write_scope_allows_writes_outside_artifacts_but_inside_workspace() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");
    runtime
        .switch_write_path_scope(WritePathScope::WorkspaceOnly)
        .expect("write scope switch should succeed");

    let result = runtime
        .run_demo_turn("write:.hepta/runtime-write-scope-test.txt => hello workspace")
        .await
        .expect("write should succeed");

    assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
    let written = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".hepta/runtime-write-scope-test.txt");
    let content = fs::read_to_string(&written).expect("workspace file should exist");
    assert_eq!(content, "hello workspace");
    let _ = fs::remove_file(&written);
}

#[tokio::test]
async fn create_mode_refuses_silent_overwrite_for_existing_file() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-overwrite-guard-test.txt");
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "original").expect("seed file should be writable");

    let blocked = runtime
        .run_demo_turn("write:artifacts/hepta-overwrite-guard-test.txt => replacement")
        .await
        .expect("run should return blocked result");

    assert_eq!(blocked.invoked_tool, None);
    assert_eq!(
        blocked.final_text,
        "write semantics blocked tool write_file"
    );
    assert!(blocked
                .blocked_reason
                .as_deref()
                .unwrap_or_default()
                .contains("write_file refuses to overwrite existing path artifacts/hepta-overwrite-guard-test.txt"));
    assert_eq!(
        fs::read_to_string(&path).expect("seed file should still exist"),
        "original"
    );
    let _ = fs::remove_file(&path);
}

#[tokio::test]
async fn overwrite_mode_replaces_existing_file_when_explicitly_confirmed() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-explicit-overwrite-test.txt");
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "before").expect("seed file should be writable");

    let result = runtime
        .run_demo_turn("overwrite:artifacts/hepta-explicit-overwrite-test.txt => after")
        .await
        .expect("explicit overwrite should succeed");

    assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
    assert_eq!(
        fs::read_to_string(&path).expect("target file should exist"),
        "after"
    );
    let output_json = result.tool_output_json.expect("structured output expected");
    assert!(output_json.contains("\"mode_requested\":\"overwrite\""));
    assert!(output_json.contains("\"mode_applied\":\"overwrite\""));
    assert!(output_json.contains("\"existed_before\":true"));
    assert!(output_json.contains("\"backup_created\":true"));
    let backup_path = extract_json_string_field(&output_json, "backup_path")
        .expect("backup path should be present");
    assert_eq!(
        fs::read_to_string(&backup_path).expect("backup file should exist"),
        "before"
    );
    let _ = fs::remove_file(&backup_path);
    let _ = fs::remove_file(&path);
}

#[tokio::test]
async fn append_mode_appends_instead_of_replacing_existing_file() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-append-mode-test.txt");
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "before").expect("seed file should be writable");

    let result = runtime
        .run_demo_turn("append:artifacts/hepta-append-mode-test.txt => +after")
        .await
        .expect("append should succeed");

    assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
    assert_eq!(
        fs::read_to_string(&path).expect("target file should exist"),
        "before+after"
    );
    let output_json = result.tool_output_json.expect("structured output expected");
    assert!(output_json.contains("\"mode_requested\":\"append\""));
    assert!(output_json.contains("\"mode_applied\":\"append\""));
    if let Some(checkpoint_path) =
        extract_json_string_field(&output_json, "rollback_checkpoint_path")
    {
        let _ = fs::remove_file(checkpoint_path);
    }
    let _ = fs::remove_file(&path);
}

#[tokio::test]
async fn preview_write_reports_diff_and_backup_plan_without_mutating_file() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-preview-write-test.txt");
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "before").expect("seed file should be writable");

    let result = runtime
        .run_demo_turn("preview-write:artifacts/hepta-preview-write-test.txt => after")
        .await
        .expect("preview should succeed");

    assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
    assert_eq!(
        fs::read_to_string(&path).expect("target file should still exist"),
        "before"
    );
    let output_json = result.tool_output_json.expect("structured output expected");
    assert!(output_json.contains("\"preview_only\":true"));
    assert!(output_json.contains("\"backup_planned\":true"));
    assert!(output_json.contains("overwrite existing file"));
    let backup_path = extract_json_string_field(&output_json, "backup_path")
        .expect("preview backup path should be present");
    assert!(
        !PathBuf::from(&backup_path).exists(),
        "preview must not create the backup file"
    );
    let _ = fs::remove_file(&path);
}

#[tokio::test]
async fn backup_index_lists_generated_overwrite_backups() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-backup-index-test-{}.txt", unique);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path);
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "before").expect("seed file should be writable");

    runtime
        .run_demo_turn(&format!("overwrite:{} => after", logical_path))
        .await
        .expect("overwrite should succeed");

    let report = runtime
        .backup_index(Some(&logical_path))
        .expect("backup index should succeed");
    assert_eq!(report.backups.len(), 1);
    assert!(report.backups[0].target_path.ends_with(&logical_path));

    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(&report.backups[0].backup_path);
}

#[tokio::test]
async fn restore_backup_restores_target_and_backs_up_current_contents() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-restore-backup-test-{}.txt", unique);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path);
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "before").expect("seed file should be writable");

    runtime
        .run_demo_turn(&format!("overwrite:{} => after", logical_path))
        .await
        .expect("overwrite should succeed");

    let index = runtime
        .backup_index(Some(&logical_path))
        .expect("backup index should succeed");
    let backup = index.backups.first().expect("backup should exist").clone();

    let report = runtime
        .restore_backup(&backup.id)
        .expect("restore backup should succeed");

    assert_eq!(
        fs::read_to_string(&path).expect("restored target should exist"),
        "before"
    );
    assert!(report.transaction_id.starts_with("txn-"));
    let safety_backup = report
        .previous_target_backup_path
        .clone()
        .expect("restore should preserve replaced contents");
    assert_eq!(
        fs::read_to_string(&safety_backup).expect("safety backup should exist"),
        "after"
    );

    let events = runtime.events(20).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::BackupRestored)
    );

    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(&backup.backup_path);
    let _ = fs::remove_file(&safety_backup);
}

#[test]
fn preview_backup_path_avoids_timestamp_collision() {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let backup_root = workspace_root.join("artifacts/backups/write_file");
    let logical_path = PathBuf::from("artifacts/hepta-preview-backup-collision.txt");
    let relative = PathBuf::from("workspace").join(&logical_path);
    let file_name = relative
        .file_name()
        .and_then(|name| name.to_str())
        .expect("file name should exist");
    let backup_dir = backup_root.join(relative.parent().expect("relative parent should exist"));
    fs::create_dir_all(&backup_dir).expect("backup dir should be creatable");

    let start_ts = 424242u64;
    let existing = backup_dir.join(format!("{}.hepta-bak-{}", file_name, start_ts));
    fs::write(&existing, b"before").expect("existing collision file should be writable");

    let candidate = preview_backup_path_from_ts(&backup_root, &relative, file_name, start_ts)
        .expect("backup path should be planned");

    assert_ne!(candidate, existing);
    assert_eq!(
        candidate,
        backup_dir.join(format!("{}.hepta-bak-{}", file_name, start_ts + 1))
    );

    let _ = fs::remove_file(existing);
    let _ = fs::remove_file(candidate);
}

#[tokio::test]
async fn write_transaction_rollback_restores_previous_append_contents() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-write-rollback-test-{}.txt", unique);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path);
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "before").expect("seed file should be writable");

    let result = runtime
        .run_demo_turn(&format!("append:{} => +after", logical_path))
        .await
        .expect("append should succeed");

    let output_json = result.tool_output_json.expect("structured output expected");
    let transaction_id = extract_json_string_field(&output_json, "transaction_id")
        .expect("transaction id should exist");
    let rollback_checkpoint_path =
        extract_json_string_field(&output_json, "rollback_checkpoint_path")
            .expect("rollback checkpoint path should exist");
    assert_eq!(
        fs::read_to_string(&path).expect("target should exist"),
        "before+after"
    );
    assert_eq!(
        fs::read_to_string(&rollback_checkpoint_path).expect("checkpoint should exist"),
        "before"
    );

    let transaction_report = runtime
        .write_transactions(Some(&logical_path))
        .expect("transactions should load");
    assert_eq!(transaction_report.transactions.len(), 1);
    assert_eq!(
        transaction_report.transactions[0].transaction_id,
        transaction_id
    );

    let rollback = runtime
        .rollback_write_transaction(&transaction_id)
        .expect("rollback should succeed");
    assert_eq!(
        fs::read_to_string(&path).expect("target should exist after rollback"),
        "before"
    );
    assert_eq!(rollback.rollback_strategy, "restore_checkpoint");
    assert!(rollback.previous_target_backup_path.is_some());

    let events = runtime.events(40).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteTransactionRecorded)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteRolledBack)
    );

    let safety_backup = rollback
        .previous_target_backup_path
        .expect("rollback should create safety backup");
    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(&rollback_checkpoint_path);
    let _ = fs::remove_file(&safety_backup);
}

#[tokio::test]
async fn write_transaction_group_plan_tracks_reverse_multi_file_rollback_order() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path_a = format!("artifacts/hepta-write-group-a-{}.txt", unique);
    let logical_path_b = format!("artifacts/hepta-write-group-b-{}.txt", unique);
    let path_a = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path_a);
    let path_b = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path_b);
    fs::create_dir_all(path_a.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path_a, "before-a").expect("seed file a should be writable");
    fs::write(&path_b, "before-b").expect("seed file b should be writable");

    let group = runtime
        .begin_write_transaction_group(Some("grp-test"))
        .expect("group should open");
    runtime
        .run_demo_turn(&format!("append:{} => +after-a", logical_path_a))
        .await
        .expect("append a should succeed");
    runtime
        .run_demo_turn(&format!("append:{} => +after-b", logical_path_b))
        .await
        .expect("append b should succeed");
    runtime
        .end_write_transaction_group()
        .expect("group should close");

    let groups = runtime
        .write_transaction_groups()
        .expect("groups should load");
    assert_eq!(groups.groups.len(), 1);
    assert_eq!(groups.groups[0].group_id, group.group_id);
    assert_eq!(groups.groups[0].transaction_ids.len(), 2);

    let plan = runtime
        .rollback_write_plan(&group.group_id)
        .expect("rollback plan should load");
    assert!(plan.closed);
    assert!(plan.executable);
    assert_eq!(plan.steps.len(), 2);
    assert!(plan.steps[0].target_path.ends_with(&logical_path_b));
    assert!(plan.steps[1].target_path.ends_with(&logical_path_a));

    for entry in runtime
        .write_transactions(None)
        .expect("transactions should load")
        .transactions
    {
        if entry.target_path.ends_with(&logical_path_a)
            || entry.target_path.ends_with(&logical_path_b)
        {
            if let Some(checkpoint) = entry.rollback_checkpoint_path {
                let _ = fs::remove_file(checkpoint);
            }
        }
    }
    for logical_path in [&logical_path_a, &logical_path_b] {
        let backups = runtime
            .backup_index(Some(logical_path))
            .expect("backup index should load");
        for backup in backups.backups {
            let _ = fs::remove_file(backup.backup_path);
        }
    }
    let _ = fs::remove_file(&path_a);
    let _ = fs::remove_file(&path_b);
}

#[tokio::test]
async fn rollback_group_restores_multiple_files() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path_a = format!("artifacts/hepta-rollback-group-a-{}.txt", unique);
    let logical_path_b = format!("artifacts/hepta-rollback-group-b-{}.txt", unique);
    let path_a = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path_a);
    let path_b = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path_b);
    fs::create_dir_all(path_a.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path_a, "before-a").expect("seed file a should be writable");
    fs::write(&path_b, "before-b").expect("seed file b should be writable");

    let group = runtime
        .begin_write_transaction_group(None)
        .expect("group should open");
    runtime
        .run_demo_turn(&format!("append:{} => +after-a", logical_path_a))
        .await
        .expect("append a should succeed");
    runtime
        .run_demo_turn(&format!("append:{} => +after-b", logical_path_b))
        .await
        .expect("append b should succeed");
    runtime
        .end_write_transaction_group()
        .expect("group should close");

    let report = runtime
        .rollback_write_group(&group.group_id)
        .expect("rollback group should succeed");
    assert_eq!(report.executed_transaction_ids.len(), 2);
    assert_eq!(
        fs::read_to_string(&path_a).expect("path a should exist"),
        "before-a"
    );
    assert_eq!(
        fs::read_to_string(&path_b).expect("path b should exist"),
        "before-b"
    );

    let events = runtime.events(50).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteTransactionGroupOpened)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteTransactionGroupClosed)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteGroupRolledBack)
    );

    for entry in runtime
        .write_transactions(None)
        .expect("transactions should load")
        .transactions
    {
        if entry.target_path.ends_with(&logical_path_a)
            || entry.target_path.ends_with(&logical_path_b)
        {
            if let Some(checkpoint) = entry.rollback_checkpoint_path {
                let _ = fs::remove_file(checkpoint);
            }
        }
    }
    for logical_path in [&logical_path_a, &logical_path_b] {
        let backups = runtime
            .backup_index(Some(logical_path))
            .expect("backup index should load");
        for backup in backups.backups {
            let _ = fs::remove_file(backup.backup_path);
        }
    }
    let _ = fs::remove_file(&path_a);
    let _ = fs::remove_file(&path_b);
}

#[tokio::test]
async fn rollback_group_partial_failure_records_status_and_resume_path() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path_a = format!("artifacts/hepta-partial-rollback-a-{}.txt", unique);
    let logical_path_b = format!("artifacts/hepta-partial-rollback-b-{}.txt", unique);
    let path_a = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path_a);
    let path_b = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path_b);
    fs::create_dir_all(path_a.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path_a, "before-a").expect("seed file a should be writable");
    fs::write(&path_b, "before-b").expect("seed file b should be writable");

    let group = runtime
        .begin_write_transaction_group(Some("grp-partial"))
        .expect("group should open");
    runtime
        .run_demo_turn(&format!("append:{} => +after-a", logical_path_a))
        .await
        .expect("append a should succeed");
    runtime
        .run_demo_turn(&format!("append:{} => +after-b", logical_path_b))
        .await
        .expect("append b should succeed");
    runtime
        .end_write_transaction_group()
        .expect("group should close");

    let plan = runtime
        .rollback_write_plan(&group.group_id)
        .expect("rollback plan should load");
    let fail_txn = plan.steps[1].transaction_id.clone();
    runtime
        .rollback_failure_injection_state
        .lock()
        .expect("failure injection state should lock")
        .push(fail_txn.clone());

    let partial = runtime
        .rollback_write_group(&group.group_id)
        .expect("rollback group should return partial failure report");
    assert_eq!(partial.status, RollbackGroupAttemptStatus::PartialFailed);
    assert_eq!(
        partial.failed_transaction_id.as_deref(),
        Some(fail_txn.as_str())
    );
    assert_eq!(partial.executed_transaction_ids.len(), 1);
    assert_eq!(partial.pending_transaction_ids, vec![fail_txn.clone()]);
    assert!(partial.resume_command.is_some());
    assert_eq!(
        fs::read_to_string(&path_b).expect("path b should be restored"),
        "before-b"
    );
    assert_eq!(
        fs::read_to_string(&path_a).expect("path a should still be appended"),
        "before-a+after-a"
    );

    let status = runtime
        .rollback_group_status(&group.group_id)
        .expect("rollback status should load");
    assert_eq!(
        status.schema_version,
        super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION
    );
    assert!(status.group_locked);
    assert_eq!(
        status.group_lock_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(status.target_lock_count, 2);
    assert_eq!(status.orphaned_lock_count, 0);
    assert!(status.latest_attempt_owns_lock_set);
    assert_eq!(status.attempt_lifecycle.attempt_count, 1);
    assert_eq!(status.attempt_lifecycle.superseded_attempt_count, 0);
    assert_eq!(
        status.attempt_lifecycle.active_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(status.lock_diagnostics.target_lock_count, 2);
    assert_eq!(
        status
            .latest_attempt
            .as_ref()
            .expect("attempt should exist")
            .status,
        RollbackGroupAttemptStatus::PartialFailed
    );
    assert!(status.resume_command.is_some());

    let status_json = serde_json::to_value(&status).expect("status should serialize");
    assert_eq!(
        status_json.get("schema_version").and_then(Value::as_u64),
        Some(super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION as u64)
    );
    assert_eq!(
        status_json
            .get("lock_diagnostics")
            .and_then(|value| value.get("group_lock_attempt_id"))
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(
        status_json
            .get("attempt_lifecycle")
            .and_then(|value| value.get("active_attempt_id"))
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );

    let locks = runtime.write_locks().expect("write locks should load");
    assert_eq!(
        locks.schema_version,
        super::WRITE_LOCK_REPORT_SCHEMA_VERSION
    );
    assert_eq!(locks.summary.total_target_locks, 2);
    assert_eq!(locks.summary.total_group_locks, 1);
    assert_eq!(locks.summary.rollback_bound_target_locks, 2);
    assert_eq!(locks.summary.rollback_bound_group_locks, 1);
    assert_eq!(locks.summary.orphaned_target_locks, 0);
    assert_eq!(locks.summary.orphaned_group_locks, 0);
    let group_lock = locks
        .group_locks
        .iter()
        .find(|lock| lock.group_id == group.group_id)
        .expect("group lock should exist");
    assert_eq!(
        group_lock.rollback_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(
        group_lock.rollback_status,
        Some(RollbackGroupAttemptStatus::PartialFailed)
    );
    assert_eq!(group_lock.pending_transaction_ids, vec![fail_txn.clone()]);
    let target_lock_a = locks
        .target_locks
        .iter()
        .find(|lock| lock.target_path.ends_with(&logical_path_a))
        .expect("target lock a should exist");
    assert_eq!(
        target_lock_a.rollback_group_id.as_deref(),
        Some(group.group_id.as_str())
    );
    assert_eq!(
        target_lock_a.rollback_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );
    let target_lock_b = locks
        .target_locks
        .iter()
        .find(|lock| lock.target_path.ends_with(&logical_path_b))
        .expect("target lock b should exist");
    assert_eq!(
        target_lock_b.rollback_group_id.as_deref(),
        Some(group.group_id.as_str())
    );
    assert_eq!(
        target_lock_b.rollback_attempt_id.as_deref(),
        Some(partial.attempt_id.as_str())
    );

    let locks_json = serde_json::to_value(&locks).expect("locks should serialize");
    assert_eq!(
        locks_json.get("schema_version").and_then(Value::as_u64),
        Some(super::WRITE_LOCK_REPORT_SCHEMA_VERSION as u64)
    );
    assert_eq!(
        locks_json
            .get("summary")
            .and_then(|value| value.get("rollback_bound_target_locks"))
            .and_then(Value::as_u64),
        Some(2)
    );
    assert_eq!(
        locks_json
            .get("summary")
            .and_then(|value| value.get("orphaned_group_locks"))
            .and_then(Value::as_u64),
        Some(0)
    );

    let blocked_write = runtime
        .run_demo_turn(&format!("append:{} => +blocked", logical_path_a))
        .await
        .expect("blocked write should still return a turn result");
    assert!(
        blocked_write
            .blocked_reason
            .expect("blocked reason should exist")
            .contains("write lock blocks write_file")
    );

    let resumed = runtime
        .resume_rollback_write_group(&group.group_id)
        .expect("resume rollback should succeed");
    assert_eq!(resumed.status, RollbackGroupAttemptStatus::Completed);
    assert_eq!(
        resumed.resumed_from_attempt_id,
        Some(partial.attempt_id.clone())
    );
    assert_eq!(
        fs::read_to_string(&path_a).expect("path a should be restored"),
        "before-a"
    );
    let post_resume_status = runtime
        .rollback_group_status(&group.group_id)
        .expect("post-resume rollback status should load");
    assert_eq!(post_resume_status.attempt_count, 2);
    assert_eq!(post_resume_status.superseded_attempt_count, 1);
    assert_eq!(
        post_resume_status.active_attempt_id.as_deref(),
        Some(resumed.attempt_id.as_str())
    );
    let superseded_partial = runtime
        .rollback_group_attempt_by_id(&partial.attempt_id)
        .expect("partial attempt lookup should succeed")
        .expect("partial attempt should exist");
    assert_eq!(
        superseded_partial.superseded_by_attempt_id.as_deref(),
        Some(resumed.attempt_id.as_str())
    );
    assert!(
        !runtime
            .write_locks()
            .expect("write locks should load")
            .group_locks
            .iter()
            .any(|lock| lock.group_id == group.group_id)
    );

    let events = runtime.events(60).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLocksAcquired)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLocksReleased)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLockConflict)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteGroupRollbackFailed)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteGroupRollbackResumed)
    );
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteGroupRolledBack)
    );

    let failed_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteGroupRollbackFailed)
        .and_then(|event| event.event.payload.as_ref())
        .expect("failed rollback event payload should exist");
    assert_eq!(
        failed_event_payload
            .get("schema_version")
            .and_then(Value::as_u64),
        Some(super::ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION as u64)
    );
    assert_eq!(
        failed_event_payload.get("group_id").and_then(Value::as_str),
        Some(group.group_id.as_str())
    );
    assert_eq!(
        failed_event_payload
            .get("attempt_id")
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(
        failed_event_payload
            .get("failed_transaction_id")
            .and_then(Value::as_str),
        Some(fail_txn.as_str())
    );

    let resumed_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteGroupRollbackResumed)
        .and_then(|event| event.event.payload.as_ref())
        .expect("resumed rollback event payload should exist");
    assert_eq!(
        resumed_event_payload
            .get("resumed_from_attempt_id")
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );
    assert_eq!(
        resumed_event_payload
            .get("resumed_attempt_id")
            .and_then(Value::as_str),
        Some(resumed.attempt_id.as_str())
    );

    let rolled_back_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteGroupRolledBack)
        .and_then(|event| event.event.payload.as_ref())
        .expect("completed rollback event payload should exist");
    assert_eq!(
        rolled_back_event_payload
            .get("status")
            .and_then(Value::as_str),
        Some("completed")
    );
    assert_eq!(
        rolled_back_event_payload
            .get("attempt_id")
            .and_then(Value::as_str),
        Some(resumed.attempt_id.as_str())
    );

    let conflict_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteLockConflict)
        .and_then(|event| event.event.payload.as_ref())
        .expect("write lock conflict payload should exist");
    assert_eq!(
        conflict_event_payload
            .get("operation")
            .and_then(Value::as_str),
        Some("write_file")
    );
    assert_eq!(
        conflict_event_payload
            .get("conflicting_group_id")
            .and_then(Value::as_str),
        Some(group.group_id.as_str())
    );
    assert_eq!(
        conflict_event_payload
            .get("conflicting_attempt_id")
            .and_then(Value::as_str),
        Some(partial.attempt_id.as_str())
    );

    let released_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteLocksReleased)
        .and_then(|event| event.event.payload.as_ref())
        .expect("write locks released payload should exist");
    assert_eq!(
        released_event_payload
            .get("released_group_locks")
            .and_then(Value::as_u64),
        Some(1)
    );
    assert_eq!(
        released_event_payload
            .get("released_target_locks")
            .and_then(Value::as_u64),
        Some(2)
    );

    for entry in runtime
        .write_transactions(None)
        .expect("transactions should load")
        .transactions
    {
        if entry.target_path.ends_with(&logical_path_a)
            || entry.target_path.ends_with(&logical_path_b)
        {
            if let Some(checkpoint) = entry.rollback_checkpoint_path {
                let _ = fs::remove_file(checkpoint);
            }
        }
    }
    for logical_path in [&logical_path_a, &logical_path_b] {
        let backups = runtime
            .backup_index(Some(logical_path))
            .expect("backup index should load");
        for backup in backups.backups {
            let _ = fs::remove_file(backup.backup_path);
        }
    }
    let _ = fs::remove_file(&path_a);
    let _ = fs::remove_file(&path_b);
}

#[test]
fn rollback_status_flags_orphaned_locks_and_recommends_prune() {
    let runtime = RuntimeKernel::new();
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .groups
        .push(WriteTransactionGroup {
            group_id: "grp-orphaned".into(),
            session_id: "session-main".into(),
            opened_at_unix_ms: 1,
            closed_at_unix_ms: Some(2),
            transaction_ids: vec![],
        });
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .rollback_attempts
        .push(RollbackGroupAttempt {
            attempt_id: "rbk-orphaned".into(),
            session_id: "session-main".into(),
            group_id: "grp-orphaned".into(),
            started_at_unix_ms: 1,
            finished_at_unix_ms: Some(2),
            status: RollbackGroupAttemptStatus::PartialFailed,
            resumed_from_attempt_id: None,
            superseded_by_attempt_id: Some("rbk-current".into()),
            executed_transaction_ids: vec![],
            skipped_already_rolled_back_ids: vec![],
            pending_transaction_ids: vec!["txn-orphaned".into()],
            failed_transaction_id: Some("txn-orphaned".into()),
            failure_reason: Some("boom".into()),
            target_paths_restored: vec![],
        });
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .rollback_attempts
        .push(RollbackGroupAttempt {
            attempt_id: "rbk-current".into(),
            session_id: "session-main".into(),
            group_id: "grp-orphaned".into(),
            started_at_unix_ms: 3,
            finished_at_unix_ms: Some(4),
            status: RollbackGroupAttemptStatus::PartialFailed,
            resumed_from_attempt_id: Some("rbk-orphaned".into()),
            superseded_by_attempt_id: None,
            executed_transaction_ids: vec![],
            skipped_already_rolled_back_ids: vec![],
            pending_transaction_ids: vec!["txn-current".into()],
            failed_transaction_id: Some("txn-current".into()),
            failure_reason: Some("still broken".into()),
            target_paths_restored: vec![],
        });
    runtime
        .write_lock_state
        .lock()
        .expect("write lock state should lock")
        .group_locks
        .push(WriteGroupLock {
            session_id: "session-main".into(),
            group_id: "grp-orphaned".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "rbk-orphaned".into(),
            rollback_attempt_id: Some("rbk-orphaned".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist") + 60_000,
        });

    let status = runtime
        .rollback_group_status("grp-orphaned")
        .expect("rollback status should load");
    assert_eq!(
        status.schema_version,
        super::ROLLBACK_GROUP_STATUS_SCHEMA_VERSION
    );
    assert!(status.group_locked);
    assert_eq!(
        status.group_lock_attempt_id.as_deref(),
        Some("rbk-orphaned")
    );
    assert_eq!(status.orphaned_lock_count, 1);
    assert!(!status.latest_attempt_owns_lock_set);
    assert_eq!(status.active_attempt_id.as_deref(), Some("rbk-current"));
    assert_eq!(status.lock_diagnostics.orphaned_lock_count, 1);
    assert_eq!(status.attempt_lifecycle.superseded_attempt_count, 1);
    assert_eq!(status.resume_command.as_deref(), Some("/prune-stale-locks"));
    assert!(
        status
            .suggested_next_action
            .contains("prune orphaned locks")
    );

    let status_json = serde_json::to_value(&status).expect("status should serialize");
    assert_eq!(
        status_json
            .get("lock_diagnostics")
            .and_then(|value| value.get("orphaned_lock_count"))
            .and_then(Value::as_u64),
        Some(1)
    );
    assert_eq!(
        status_json
            .get("attempt_lifecycle")
            .and_then(|value| value.get("superseded_attempt_count"))
            .and_then(Value::as_u64),
        Some(1)
    );

    let locks = runtime.write_locks().expect("write locks should load");
    assert_eq!(locks.summary.orphaned_group_locks, 1);
}

#[tokio::test]
async fn overlap_lock_blocks_write_to_descendant_path() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_dir = format!("artifacts/hepta-locked-dir-{}", unique);
    let locked_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_dir);
    runtime
        .write_lock_state
        .lock()
        .expect("write lock state should lock")
        .target_locks
        .push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: locked_dir_path.display().to_string(),
            owner_kind: "rollback_group".into(),
            owner_id: "grp-overlap".into(),
            rollback_group_id: Some("grp-overlap".into()),
            rollback_attempt_id: Some("rbk-overlap".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist") + 60_000,
        });

    let result = runtime
        .run_demo_turn(&format!("append:{}/child.txt => +blocked", logical_dir))
        .await
        .expect("blocked write should still produce a turn result");
    assert!(
        result
            .blocked_reason
            .expect("blocked reason should exist")
            .contains("write lock blocks write_file")
    );

    let events = runtime.events(20).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLockConflict)
    );
}

#[tokio::test]
async fn active_write_reservation_blocks_parallel_write_and_rollback() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");
    let relative_path = format!(
        "artifacts/hepta-write-reservation-{}.txt",
        current_unix_ms().expect("timestamp should exist")
    );
    let target_path = crate::tool_workspace_root_path().join(&relative_path);
    fs::create_dir_all(target_path.parent().expect("target parent"))
        .expect("target parent should be created");
    fs::write(&target_path, "before").expect("target should be seeded");
    let write = runtime
        .run_demo_turn(&format!("overwrite:{} => after", relative_path))
        .await
        .expect("seed write should succeed");
    let transaction_id = extract_json_string_field(
        write.tool_output_json.as_deref().expect("write output"),
        "transaction_id",
    )
    .expect("transaction id");
    let backup = runtime
        .backup_index(Some(&relative_path))
        .expect("backup index")
        .backups[0]
        .clone();
    let arguments = json!({
        "path": &relative_path,
        "content": "reserved",
        "mode": "overwrite"
    })
    .to_string();
    let (reserved_tx, reserved_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();

    std::thread::scope(|scope| {
        let runtime_ref = &runtime;
        let arguments_ref = &arguments;
        let holder = scope.spawn(move || {
            let prepared = runtime_ref
                .prepare_write_transaction_with_lock_check(
                    "session-main",
                    "write_file",
                    arguments_ref,
                )
                .expect("first writer should reserve target")
                .expect("write preparation should exist");
            reserved_tx.send(()).expect("reservation signal");
            release_rx.recv().expect("release signal");
            drop(prepared);
        });

        reserved_rx.recv().expect("reservation should be active");
        let write_error = runtime
            .prepare_write_transaction_with_lock_check("session-other", "write_file", &arguments)
            .expect_err("parallel writer must be blocked");
        assert!(write_error.0.contains("tool_execution_reservation"));

        let target_path = crate::tool_workspace_root_path().join(&relative_path);
        let rollback_error = runtime
            .acquire_group_rollback_locks(
                "session-main",
                "group-concurrent",
                "attempt-concurrent",
                &[target_path.display().to_string()],
            )
            .expect_err("rollback must not cross an active write");
        assert!(
            rollback_error
                .0
                .contains("write lock blocks rollback_group")
        );
        let restore_error = runtime
            .restore_backup(&backup.id)
            .expect_err("active write must block restore");
        assert!(restore_error.0.contains("tool_execution_reservation"));
        let transaction_error = runtime
            .rollback_write_transaction(&transaction_id)
            .expect_err("active write must block public rollback");
        assert!(transaction_error.0.contains("tool_execution_reservation"));

        release_tx.send(()).expect("release holder");
        holder.join().expect("reservation holder should finish");
    });

    let prepared = runtime
        .prepare_write_transaction_with_lock_check("session-main", "write_file", &arguments)
        .expect("released target should be reservable");
    drop(prepared);
    assert!(
        runtime
            .write_lock_state
            .lock()
            .expect("write lock state")
            .active_target_reservations
            .is_empty()
    );
    runtime
        .acquire_group_rollback_locks(
            "session-main",
            "group-after-release",
            "attempt-after-release",
            &[target_path.display().to_string()],
        )
        .expect("released target should permit rollback lock");
    runtime
        .release_group_rollback_locks("session-main", "group-after-release")
        .expect("rollback lock should release");
    let _ = fs::remove_file(target_path);
    let _ = fs::remove_file(backup.backup_path);
}

#[tokio::test]
async fn overlap_lock_blocks_rollback_group_on_descendant_target() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_dir = format!("artifacts/hepta-overlap-rollback-{}", unique);
    let logical_path = format!("{}/child.txt", logical_dir);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path);
    fs::create_dir_all(path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&path, "before").expect("seed file should be writable");

    let group = runtime
        .begin_write_transaction_group(Some("grp-overlap-rollback"))
        .expect("group should open");
    runtime
        .run_demo_turn(&format!("append:{} => +after", logical_path))
        .await
        .expect("append should succeed");
    runtime
        .end_write_transaction_group()
        .expect("group should close");

    let locked_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_dir);
    runtime
        .write_lock_state
        .lock()
        .expect("write lock state should lock")
        .target_locks
        .push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: locked_dir_path.display().to_string(),
            owner_kind: "rollback_group".into(),
            owner_id: "grp-external".into(),
            rollback_group_id: Some("grp-external".into()),
            rollback_attempt_id: Some("rbk-external".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: current_unix_ms().expect("timestamp should exist") + 60_000,
        });

    let err = runtime
        .rollback_write_group(&group.group_id)
        .expect_err("overlap lock should block rollback group");
    assert!(err.0.contains("write lock blocks rollback_group"));

    let events = runtime.events(30).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLockConflict)
    );
    let conflict_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteLockConflict)
        .and_then(|event| event.event.payload.as_ref())
        .expect("rollback-group conflict payload should exist");
    assert_eq!(
        conflict_event_payload
            .get("operation")
            .and_then(Value::as_str),
        Some("rollback_group")
    );
    assert_eq!(
        conflict_event_payload
            .get("conflicting_group_id")
            .and_then(Value::as_str),
        Some("grp-external")
    );

    for entry in runtime
        .write_transactions(None)
        .expect("transactions should load")
        .transactions
    {
        if entry.target_path.ends_with(&logical_path) {
            if let Some(checkpoint) = entry.rollback_checkpoint_path {
                let _ = fs::remove_file(checkpoint);
            }
        }
    }
    for backup in runtime
        .backup_index(Some(&logical_path))
        .expect("backup index should load")
        .backups
    {
        let _ = fs::remove_file(backup.backup_path);
    }
    let _ = fs::remove_file(&path);
}

#[tokio::test]
async fn expired_write_lock_is_pruned_and_does_not_block_write() {
    let runtime = RuntimeKernel::new();
    runtime
        .add_policy_rule(
            None,
            None,
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("test allow write"),
        )
        .expect("policy rule should be added");

    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-expired-lock-{}.txt", unique);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(&logical_path);
    runtime
        .write_lock_state
        .lock()
        .expect("write lock state should lock")
        .target_locks
        .push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: path.display().to_string(),
            owner_kind: "rollback_group".into(),
            owner_id: "grp-stale".into(),
            rollback_group_id: Some("grp-stale".into()),
            rollback_attempt_id: Some("rbk-stale".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: 1,
        });

    let result = runtime
        .run_demo_turn(&format!("append:{} => +after", logical_path))
        .await
        .expect("write should succeed after stale lock pruning");
    assert_eq!(result.invoked_tool.as_deref(), Some("write_file"));
    assert!(
        runtime
            .write_locks()
            .expect("write locks should load")
            .target_locks
            .is_empty()
    );

    for entry in runtime
        .write_transactions(Some(&logical_path))
        .expect("transactions should load")
        .transactions
    {
        if let Some(checkpoint) = entry.rollback_checkpoint_path {
            let _ = fs::remove_file(checkpoint);
        }
    }
    let _ = fs::remove_file(&path);
}

#[test]
fn prune_stale_write_locks_removes_expired_entries_and_emits_event() {
    let runtime = RuntimeKernel::new();
    {
        let mut guard = runtime
            .write_lock_state
            .lock()
            .expect("write lock state should lock");
        guard.group_locks.push(WriteGroupLock {
            session_id: "session-main".into(),
            group_id: "grp-stale".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "rbk-stale".into(),
            rollback_attempt_id: Some("rbk-stale".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: 1,
        });
        guard.target_locks.push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: "/tmp/hepta-stale".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "grp-stale".into(),
            rollback_group_id: Some("grp-stale".into()),
            rollback_attempt_id: Some("rbk-stale".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms: 1,
        });
    }

    let report = runtime
        .prune_stale_write_locks()
        .expect("stale lock prune should succeed");
    assert_eq!(report.pruned_target_locks, 1);
    assert_eq!(report.pruned_group_locks, 1);
    assert_eq!(report.remaining_target_locks, 0);
    assert_eq!(report.remaining_group_locks, 0);

    let events = runtime.events(20).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::WriteLocksPruned)
    );
    let pruned_event_payload = events
        .iter()
        .find(|event| event.event.kind == EventKind::WriteLocksPruned)
        .and_then(|event| event.event.payload.as_ref())
        .expect("write locks pruned payload should exist");
    assert_eq!(
        pruned_event_payload
            .get("schema_version")
            .and_then(Value::as_u64),
        Some(super::ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION as u64)
    );
    assert_eq!(
        pruned_event_payload
            .get("pruned_target_locks")
            .and_then(Value::as_u64),
        Some(1)
    );
    assert_eq!(
        pruned_event_payload
            .get("pruned_group_locks")
            .and_then(Value::as_u64),
        Some(1)
    );
}

#[test]
fn snapshot_roundtrip_preserves_write_transactions() {
    let runtime = RuntimeKernel::new();
    let unique = current_unix_ms().expect("timestamp should exist");
    let target_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(format!("artifacts/hepta-write-txn-snapshot-{}.txt", unique));
    fs::create_dir_all(target_path.parent().expect("parent should exist"))
        .expect("artifact dir should be creatable");
    fs::write(&target_path, "before").expect("seed file should be writable");

    let checkpoint_path = preview_transaction_checkpoint_path(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        &target_path,
        "txn-snapshot",
    )
    .expect("checkpoint path should build");
    fs::create_dir_all(
        checkpoint_path
            .parent()
            .expect("checkpoint parent should exist"),
    )
    .expect("checkpoint parent should be creatable");
    fs::write(&checkpoint_path, "before").expect("checkpoint should be writable");
    runtime
        .write_transaction_state
        .lock()
        .expect("write transaction state should lock")
        .push(WriteTransactionEntry {
            transaction_id: "txn-snapshot".into(),
            session_id: "session-main".into(),
            action: "write_file".into(),
            target_path: target_path.display().to_string(),
            created_at_unix_ms: unique,
            mode: "append".into(),
            target_existed_before: true,
            bytes_before: 6,
            bytes_after: 12,
            before_content_hash: None,
            after_content_hash: None,
            effect_plan_hash: None,
            effect_ack_hash: None,
            before_file_identity: None,
            after_file_identity: None,
            rollback_strategy: "restore_checkpoint".into(),
            rollback_checkpoint_path: Some(checkpoint_path.display().to_string()),
            source_backup_path: None,
            rolled_back_at_unix_ms: None,
        });

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(format!(
            "artifacts/hepta-write-txn-snapshot-{}.json",
            unique
        ));
    runtime
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");
    let report = restored
        .write_transactions(None)
        .expect("transactions should load");
    assert!(
        report
            .transactions
            .iter()
            .any(|entry| entry.transaction_id == "txn-snapshot")
    );

    let _ = fs::remove_file(&target_path);
    let _ = fs::remove_file(&checkpoint_path);
    let _ = fs::remove_file(&snapshot_path);
}

#[test]
fn snapshot_roundtrip_preserves_write_transaction_groups() {
    let runtime = RuntimeKernel::new();
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .groups
        .push(WriteTransactionGroup {
            group_id: "txngrp-snapshot".into(),
            session_id: "session-main".into(),
            opened_at_unix_ms: 1,
            closed_at_unix_ms: Some(2),
            transaction_ids: vec!["txn-a".into(), "txn-b".into()],
        });

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-write-group-snapshot.json");
    runtime
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");
    let report = restored
        .write_transaction_groups()
        .expect("groups should load");
    assert!(
        report
            .groups
            .iter()
            .any(|group| group.group_id == "txngrp-snapshot")
    );

    let _ = fs::remove_file(&snapshot_path);
}

#[test]
fn snapshot_roundtrip_preserves_rollback_group_attempts() {
    let runtime = RuntimeKernel::new();
    runtime
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .rollback_attempts
        .push(super::RollbackGroupAttempt {
            attempt_id: "rbk-snapshot".into(),
            session_id: "session-main".into(),
            group_id: "txngrp-snapshot".into(),
            started_at_unix_ms: 1,
            finished_at_unix_ms: Some(2),
            status: RollbackGroupAttemptStatus::PartialFailed,
            resumed_from_attempt_id: None,
            superseded_by_attempt_id: None,
            executed_transaction_ids: vec!["txn-a".into()],
            skipped_already_rolled_back_ids: vec![],
            pending_transaction_ids: vec!["txn-b".into()],
            failed_transaction_id: Some("txn-b".into()),
            failure_reason: Some("boom".into()),
            target_paths_restored: vec!["/tmp/a".into()],
        });

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-rollback-attempt-snapshot.json");
    runtime
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");
    let status = restored
        .write_transaction_group_state
        .lock()
        .expect("write transaction group state should lock")
        .rollback_attempts
        .iter()
        .find(|attempt| attempt.attempt_id == "rbk-snapshot")
        .cloned();
    assert!(status.is_some());

    let _ = fs::remove_file(&snapshot_path);
}

#[test]
fn snapshot_roundtrip_preserves_write_locks() {
    let runtime = RuntimeKernel::new();
    let lease_expires_at_unix_ms = current_unix_ms().expect("timestamp should exist") + 60_000;
    {
        let mut guard = runtime
            .write_lock_state
            .lock()
            .expect("write lock state should lock");
        guard.group_locks.push(WriteGroupLock {
            session_id: "session-main".into(),
            group_id: "txngrp-snapshot".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "rbk-snapshot".into(),
            rollback_attempt_id: Some("rbk-snapshot".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms,
        });
        guard.target_locks.push(WriteTargetLock {
            session_id: "session-main".into(),
            target_path: "/tmp/a".into(),
            owner_kind: "rollback_group".into(),
            owner_id: "txngrp-snapshot".into(),
            rollback_group_id: Some("txngrp-snapshot".into()),
            rollback_attempt_id: Some("rbk-snapshot".into()),
            locked_at_unix_ms: 1,
            lease_expires_at_unix_ms,
        });
    }

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/hepta-write-lock-snapshot.json");
    runtime
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");
    let locks = restored.write_locks().expect("write locks should load");
    assert!(
        locks
            .group_locks
            .iter()
            .any(|lock| lock.group_id == "txngrp-snapshot")
    );
    assert!(
        locks
            .target_locks
            .iter()
            .any(|lock| lock.target_path == "/tmp/a")
    );
    assert!(
        locks
            .group_locks
            .iter()
            .any(|lock| lock.lease_expires_at_unix_ms == lease_expires_at_unix_ms)
    );
    assert!(
        locks
            .target_locks
            .iter()
            .any(|lock| lock.lease_expires_at_unix_ms == lease_expires_at_unix_ms)
    );

    let _ = fs::remove_file(&snapshot_path);
}

#[test]
fn preview_prune_backups_plans_deletion_of_older_backups() {
    let runtime = RuntimeKernel::new();
    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-prune-preview-test-{}.txt", unique);
    let older = write_fake_workspace_backup(&logical_path, unique, "older");
    let newer = write_fake_workspace_backup(&logical_path, unique + 1, "newer");

    let report = runtime
        .preview_prune_backups(Some(&logical_path), 1, None)
        .expect("preview prune should succeed");

    assert_eq!(report.scanned_backups, 2);
    assert_eq!(report.deleted_count, 1);
    assert_eq!(report.kept_backups.len(), 1);
    assert_eq!(report.deleted_backups[0].created_at_unix_ms, unique);
    assert_eq!(report.kept_backups[0].created_at_unix_ms, unique + 1);

    let _ = fs::remove_file(&older);
    let _ = fs::remove_file(&newer);
}

#[test]
fn prune_backups_deletes_older_backups_and_emits_event() {
    let runtime = RuntimeKernel::new();
    let unique = current_unix_ms().expect("timestamp should exist");
    let logical_path = format!("artifacts/hepta-prune-exec-test-{}.txt", unique);
    let older = write_fake_workspace_backup(&logical_path, unique, "older");
    let newer = write_fake_workspace_backup(&logical_path, unique + 1, "newer");

    let report = runtime
        .prune_backups(Some(&logical_path), 1, None)
        .expect("prune backups should succeed");

    assert!(report.executed);
    assert_eq!(report.deleted_count, 1);
    assert!(!older.exists());
    assert!(newer.exists());

    let events = runtime.events(20).expect("events should load");
    assert!(
        events
            .iter()
            .any(|event| event.event.kind == EventKind::BackupsPruned)
    );

    let _ = fs::remove_file(&newer);
}

#[test]
fn session_export_roundtrip_preserves_write_path_scope() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_write_path_scope(WritePathScope::WorkspaceOnly)
        .expect("write scope switch should succeed");
    let export = runtime
        .session_export("session-main")
        .expect("session export should succeed");
    assert_eq!(export.write_path_scope, WritePathScope::WorkspaceOnly);

    runtime
        .switch_write_path_scope(WritePathScope::ArtifactsOnly)
        .expect("write scope reset should succeed");
    runtime
        .apply_session_export(export)
        .expect("session import should succeed");

    assert_eq!(
        runtime
            .write_path_scope_for_session("session-main")
            .expect("write scope should load"),
        WritePathScope::WorkspaceOnly
    );
}

#[tokio::test]
async fn session_export_roundtrip_preserves_topic_sessions_and_graph_store() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    source
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    source
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    source
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    source
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");
    source
        .route_topics(
            "alpha",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("mixed route should succeed");

    let export = source
        .session_export("alpha")
        .expect("session export should succeed");
    assert_eq!(export.topic_sessions.len(), 2);
    assert!(export.topic_graph_edges.iter().any(|record| {
        record.source_topic_session_id == "topic-session-bootstrap:alpha"
            && record.edge.target_topic_session_id
                == "topic-session-bootstrap:alpha:rust-worker-pipeline"
    }));

    let restored = RuntimeKernel::new();
    restored
        .apply_session_export(export)
        .expect("session import should succeed");

    let raw_topic_sessions = restored
        .topic_session_state
        .lock()
        .expect("topic session state lock should succeed")
        .sessions
        .clone();
    let raw_topic_graph_edges = restored
        .topic_graph_state
        .lock()
        .expect("topic graph state lock should succeed")
        .edges
        .clone();
    assert_eq!(raw_topic_sessions.len(), 2);
    assert!(raw_topic_graph_edges.iter().any(|record| {
        record.source_topic_session_id == "topic-session-bootstrap:alpha"
            && record.edge.target_topic_session_id
                == "topic-session-bootstrap:alpha:rust-worker-pipeline"
    }));

    let topic_sessions = restored
        .topic_sessions_for_surface("alpha")
        .expect("topic sessions should load");
    assert!(topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:alpha"
            && !topic_session.graph_edges.is_empty()
    }));
}

#[tokio::test]
async fn exposes_sessions_memory_and_history_snapshots() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn("hello session control plane")
        .await
        .expect("plain turn should succeed");
    runtime
        .run_demo_turn("tool:history probe")
        .await
        .expect("tool turn should succeed");

    let sessions = runtime.sessions().expect("sessions should load");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].session_id, "session-main");

    let memories = runtime
        .memory_snapshot(10)
        .expect("memory snapshot should load");
    assert!(memories.iter().any(|item| {
        item.content
            .contains("assistant:hello session control plane")
    }));

    let history = runtime
        .history(Some("session-main"), 10)
        .expect("history should load");
    assert!(history.len() >= 2);
    assert_eq!(history[0].input, "tool:history probe");
}

#[test]
fn fresh_active_session_is_consistent_across_control_plane_views() {
    let runtime = RuntimeKernel::new();
    let sessions = runtime.sessions().expect("sessions should load");
    let session = runtime
        .active_session_snapshot()
        .expect("active session snapshot should load");
    let overview = runtime
        .session_activity_overview(0, 0)
        .expect("session activity overview should load");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].session_id, "session-main");
    assert!(sessions[0].is_active);
    assert_eq!(session.session_id, "session-main");
    assert!(session.is_active);
    assert_eq!(overview.sessions.len(), 1);
    assert_eq!(overview.active_sessions, 1);
    assert_eq!(overview.archived_sessions, 0);
    assert_eq!(overview.sessions[0].session.session_id, "session-main");
}

#[tokio::test]
async fn doctor_reports_provider_probes_and_integrity_checks() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn("hello doctor")
        .await
        .expect("plain turn should succeed");
    runtime
        .route_topics("session-main", Some("hello doctor"), 4, 4, 4, 1)
        .expect("topic route should succeed");

    let report = runtime
        .doctor_report()
        .await
        .expect("doctor report should succeed");
    assert_eq!(
        report.overall_status,
        DoctorStatus::Warn,
        "unexpected doctor report: {report:#?}"
    );
    assert_eq!(report.total_topic_sessions, 1);
    assert_eq!(report.total_topic_graph_edges, 0);
    assert_eq!(report.active_topic_sessions, 1);
    assert_eq!(report.active_topic_sessions_with_transcript_provenance, 1);
    assert_eq!(
        report.active_topic_sessions_missing_transcript_provenance,
        0
    );
    assert!(report.active_session_recall_transcript_evidence_spans > 0);
    assert_eq!(report.active_session_recall_omitted_items, 0);
    assert!(report.active_session_intuition_transcript_evidence_spans > 0);
    assert_eq!(report.active_session_intuition_foreground_topic_sessions, 1);
    assert!(
        report
            .provider_probes
            .iter()
            .any(|probe| probe.provider_name == "demo" && probe.status == DoctorStatus::Ok)
    );
    assert!(report.integrity_checks.iter().any(|check| {
        check.name == "runtime snapshot roundtrip" && check.status == DoctorStatus::Ok
    }));
    assert!(report.integrity_checks.iter().any(|check| {
        check.name == "active session export roundtrip" && check.status == DoctorStatus::Ok
    }));
    assert!(report.integrity_checks.iter().any(|check| {
        check.name == "topic sessions carry transcript provenance"
            && check.status == DoctorStatus::Ok
    }));

    let summary = runtime
        .doctor_summary()
        .await
        .expect("doctor summary should succeed");

    for needle in [
        "Hepta doctor: warn",
        "- topic sessions: 1",
        "- topic graph edges: 0",
        "- active topic sessions with transcript provenance: ",
        "- active topic sessions missing transcript provenance: ",
        "- active session recall transcript evidence spans: ",
        "- active session recall omitted items: 0",
        "- active session intuition transcript evidence spans: ",
        "- active session intuition foreground topic sessions: 1",
        "demo: ok via demo/demo-chat",
        "mock-ollama: ok via mock-ollama/local-chat",
        "history session references: ok",
        "runtime snapshot roundtrip: ok",
        "topic sessions carry transcript provenance: ok",
    ] {
        assert!(summary.iter().any(|line| line.contains(needle)), "{needle}");
    }
}

#[tokio::test]
async fn doctor_warns_when_active_topic_sessions_lose_transcript_provenance() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn("hello doctor provenance gap")
        .await
        .expect("plain turn should succeed");
    runtime
        .route_topics(
            "session-main",
            Some("hello doctor provenance gap"),
            4,
            4,
            4,
            1,
        )
        .expect("topic route should succeed");

    {
        let mut topic_state = runtime
            .topic_session_state
            .lock()
            .expect("topic session state mutex should not poison");
        let topic_session = topic_state
            .sessions
            .iter_mut()
            .find(|topic_session| {
                topic_session.topic_session_id == "topic-session-bootstrap:session-main"
            })
            .expect("bootstrap topic session should exist");
        topic_session.linked_transcript_spans.clear();
    }

    let report = runtime
        .doctor_report()
        .await
        .expect("doctor report should succeed");
    assert_eq!(report.overall_status, DoctorStatus::Warn);
    assert!(report.integrity_checks.iter().any(|check| {
        check.name == "topic sessions carry transcript provenance"
            && check.status == DoctorStatus::Warn
            && check
                .detail
                .contains("topic-session-bootstrap:session-main")
    }));
}

#[test]
fn rejects_invalid_tool_arguments_against_schema() {
    let runtime = RuntimeKernel::new();
    let err = runtime
        .validate_tool_input("read_file", r#"{"path":""}"#)
        .expect_err("empty path should be rejected");
    assert!(err.0.contains("must be at least 1 characters"));

    let err = runtime
        .validate_tool_input("echo", r#"{"wrong":"value"}"#)
        .expect_err("missing required field should be rejected");
    assert!(err.0.contains("missing required field 'text'"));

    let err = runtime
        .validate_tool_input(
            "write_file",
            r#"{"path":"artifacts/x.txt","content":"x","mode":"replace"}"#,
        )
        .expect_err("invalid write mode should be rejected");
    assert!(err.0.contains("must be one of: create, overwrite, append"));

    let err = runtime
        .validate_tool_input(
            "write_file",
            r#"{"path":"artifacts/x.txt","content":"x","confirm_destructive":"yes"}"#,
        )
        .expect_err("non-boolean destructive confirm should be rejected");
    assert!(err.0.contains("must be a boolean"));
}

#[tokio::test]
async fn returns_and_validates_structured_tool_output() {
    let runtime = RuntimeKernel::new();
    let result = runtime
        .run_demo_turn("tool:typed output")
        .await
        .expect("echo turn should succeed");

    let output_json = result
        .tool_output_json
        .expect("structured tool output should be present");
    assert!(output_json.contains("\"text\":\"typed output\""));
    runtime
        .validate_tool_output("echo", &output_json)
        .expect("echo output should match schema");
}

#[test]
fn native_tool_result_reply_hides_structured_json() {
    let structured = json!({
        "backend": "hepta-rust-native",
        "content": "8 native background process(es)",
        "native_runtime": true,
        "openclaw_gateway_invoked": false,
        "proxy_used": false,
        "tool": "process",
        "result": {
            "action": "list",
            "followup_actions": ["poll", "log", "write", "kill", "clear", "remove"],
            "processes": [
                {"id": "hepta-proc-1", "log_path": "/private/path/one.log"},
                {"id": "hepta-proc-2", "log_path": "/private/path/two.log"}
            ]
        }
    });
    let reply = render_native_tool_result_reply(&format!(
        "8 native background process(es) | structured={}",
        structured
    ));

    assert!(reply.contains("共有 2 条后台进程记录"));
    assert!(reply.contains("结构化 JSON 已保留在本地"));
    assert!(!reply.contains("structured="));
    assert!(!reply.contains("log_path"));
    assert!(!reply.contains("/private/path"));
    assert!(!reply.contains("Hepta native tool result"));
}

#[test]
fn exposes_tool_descriptors_for_discovery() {
    let runtime = RuntimeKernel::new();
    let tools = runtime.tool_descriptors();
    assert_eq!(tools.len(), 42);
    assert!(tools.iter().any(|tool| {
        tool.name == "echo"
            && tool.description.contains("Return the provided input as-is")
            && tool.execution_metadata.read_only
            && tool.execution_metadata.idempotent
            && tool.execution_metadata.produces_structured_output
            && tool.default_approval_requirement == ApprovalRequirement::None
            && tool.input_schema_json.contains("text")
            && tool.output_schema_json.contains("text")
    }));
    assert!(tools.iter().any(|tool| {
        tool.name == "read_file"
            && tool
                .description
                .contains("Read a UTF-8 text file from disk")
            && tool.execution_metadata.read_only
            && !tool.execution_metadata.destructive
            && tool.execution_metadata.idempotent
            && tool.default_approval_requirement == ApprovalRequirement::Ask
            && tool.input_schema_json.contains("path")
            && tool.output_schema_json.contains("line_count")
    }));
    assert!(tools.iter().any(|tool| {
        tool.name == "disk_junk_audit"
            && tool.description.contains("read-only local disk cleanup")
            && tool.execution_metadata.read_only
            && !tool.execution_metadata.destructive
            && tool.default_approval_requirement == ApprovalRequirement::None
    }));
    assert!(tools.iter().any(|tool| {
        tool.name == "write_file"
            && tool.description.contains("Write a UTF-8 text file to disk")
            && !tool.execution_metadata.read_only
            && tool.execution_metadata.destructive
            && !tool.execution_metadata.idempotent
            && tool.default_approval_requirement == ApprovalRequirement::Deny
            && tool.input_schema_json.contains("content")
            && tool.output_schema_json.contains("bytes_written")
    }));
    for expected in [
        "json_get",
        "skill_propose",
        "skill_scan",
        "skill_apply_plan",
        "tool_manifest_validate",
        "tool_generate_stub",
        "read",
        "write",
        "edit",
        "apply_patch",
        "web_search",
        "web_fetch",
        "sessions_list",
        "message",
        "memory_get",
        "feishu_doc",
    ] {
        assert!(
            tools.iter().any(|tool| tool.name == expected),
            "missing expanded native tool {expected}"
        );
    }
    let read = tools
        .iter()
        .find(|tool| tool.name == "read")
        .expect("OpenClaw-compatible read tool should exist");
    assert!(read.description.contains("Rust-native"));
    assert!(!read.description.contains("Gateway proxy"));
    for quarantined in [
        "exec",
        "process",
        "list_dir",
        "search_text",
        "memory_search",
        "image",
        "pdf",
        "image_generate",
        "music_generate",
        "video_generate",
    ] {
        assert!(!tools.iter().any(|tool| tool.name == quarantined));
    }
    let model_tools = runtime.tools.model_tool_specs();
    assert_eq!(model_tools.len(), 42);
    for quarantined in [
        "exec",
        "process",
        "list_dir",
        "search_text",
        "memory_search",
        "image",
        "pdf",
        "image_generate",
        "music_generate",
        "video_generate",
    ] {
        assert!(!model_tools.iter().any(|tool| tool.name == quarantined));
    }
}

#[tokio::test]
async fn generated_skill_and_tool_helpers_are_invokable() {
    let registry = ToolRegistry::new();
    let context = ToolContext {
        session_id: Some(SessionId("session-test".into())),
        correlation_id: Some(CorrelationId("corr-test".into())),
        execution_attempt_id: None,
        idempotency_key: None,
    };

    let skill = registry
        .invoke(
            "skill_propose",
            context.clone(),
            ToolCallRequest {
                name: "skill_propose".into(),
                input_json: r#"{"transcript":"Build a safe local skill workshop flow"}"#.into(),
            },
        )
        .await
        .expect("skill proposal helper should invoke");
    let skill_json: Value = serde_json::from_str(
        skill
            .structured_json
            .as_deref()
            .expect("skill proposal should be structured"),
    )
    .expect("skill proposal output should parse");
    assert_eq!(skill_json["safe_to_apply"], json!(true));
    assert_eq!(
        skill_json["skill_name"],
        json!("build-a-safe-local-skill-workshop-flow")
    );

    let generated = registry
        .invoke(
            "tool_generate_stub",
            context.clone(),
            ToolCallRequest {
                name: "tool_generate_stub".into(),
                input_json:
                    r#"{"name":"Summarize Local File","description":"Summarize a local file"}"#
                        .into(),
            },
        )
        .await
        .expect("tool generator should invoke");
    let generated_json = generated
        .structured_json
        .clone()
        .expect("tool generator should return structured json");
    let manifest: Value =
        serde_json::from_str(&generated_json).expect("generated tool manifest should parse");
    assert_eq!(manifest["name"], json!("summarize_local_file"));

    let validation = registry
        .invoke(
            "tool_manifest_validate",
            context,
            ToolCallRequest {
                name: "tool_manifest_validate".into(),
                input_json: json!({ "manifest_json": generated_json }).to_string(),
            },
        )
        .await
        .expect("tool manifest validator should invoke");
    let validation_json: Value = serde_json::from_str(
        validation
            .structured_json
            .as_deref()
            .expect("validation should be structured"),
    )
    .expect("validation output should parse");
    assert_eq!(validation_json["valid"], json!(true));
    assert_eq!(validation_json["issue_count"], json!(0));
}

#[tokio::test]
async fn openclaw_compatible_tools_are_native_not_gateway_proxy() {
    let registry = ToolRegistry::new_with_all_quarantined_tools_for_test();
    let context = ToolContext {
        session_id: Some(SessionId("session-native-tools".into())),
        correlation_id: Some(CorrelationId("corr-native-tools".into())),
        execution_attempt_id: None,
        idempotency_key: None,
    };
    for (tool, input_json) in [
            (
                "write",
                json!({"path":"artifacts/direct-write.txt","content":"blocked"}).to_string(),
            ),
            (
                "edit",
                json!({"path":"artifacts/direct-edit.txt","edits":[]}).to_string(),
            ),
            (
                "apply_patch",
                json!({"input":"*** Begin Patch\n*** Add File: artifacts/direct.txt\n+x\n*** End Patch"}).to_string(),
            ),
        ] {
            let error = registry
                .invoke(
                    tool,
                    context.clone(),
                    ToolCallRequest {
                        name: tool.into(),
                        input_json,
                    },
                )
                .await
                .expect_err("direct native mutation must fail closed");
            assert!(
                error.0.contains("identity-bound"),
                "{tool} failed with unexpected error: {}",
                error.0
            );
        }

    let exec = registry
        .invoke(
            "exec",
            provider_test_context("session-native-tools", "corr-native-exec"),
            ToolCallRequest {
                name: "exec".into(),
                input_json: json!({"command": "printf native-exec"}).to_string(),
            },
        )
        .await
        .expect("native exec should invoke");
    let exec_json: Value = serde_json::from_str(exec.structured_json.as_deref().unwrap())
        .expect("exec output should parse");
    assert_eq!(exec_json["proxy_used"], json!(false));
    assert_eq!(exec_json["result"]["stdout"], json!("native-exec"));

    let started = std::time::Instant::now();
    let timed_out_exec = registry
        .invoke(
            "exec",
            provider_test_context("session-native-tools", "corr-native-timeout"),
            ToolCallRequest {
                name: "exec".into(),
                input_json: json!({"command": "sleep 5", "timeoutMs": 100}).to_string(),
            },
        )
        .await
        .expect("native exec timeout should return structured result, not hang");
    assert!(started.elapsed() < std::time::Duration::from_secs(3));
    assert!(
        timed_out_exec
            .content
            .contains("ToolTimeout/exec timed out")
    );
    let timed_out_json: Value =
        serde_json::from_str(timed_out_exec.structured_json.as_deref().unwrap())
            .expect("timeout output should parse");
    assert_eq!(timed_out_json["status"], json!("timeout"));
    assert_eq!(timed_out_json["error_kind"], json!("ToolTimeout"));
    assert_eq!(timed_out_json["result"]["timeout"], json!(true));
    assert_eq!(
        timed_out_json["result"]["duplicate_tool_replay_prevented"],
        json!(true)
    );

    let background = registry
        .invoke(
            "exec",
            provider_test_context("session-native-tools", "corr-native-background"),
            ToolCallRequest {
                name: "exec".into(),
                input_json: json!({"command": "cat", "background": true}).to_string(),
            },
        )
        .await
        .expect("native background exec should invoke");
    let background_json: Value =
        serde_json::from_str(background.structured_json.as_deref().unwrap())
            .expect("background output should parse");
    let process_id = background_json["result"]["sessionId"]
        .as_str()
        .expect("background should return process id")
        .to_string();
    assert_eq!(background_json["proxy_used"], json!(false));

    registry
                .invoke(
                    "process",
                    provider_test_context("session-native-tools", "corr-native-process-write"),
                    ToolCallRequest {
                        name: "process".into(),
                        input_json: json!({"action":"write", "sessionId": process_id, "data":"native-process\n", "eof": true}).to_string(),
                    },
                )
                .await
                .expect("native process write should invoke");
    let process_poll = registry
        .invoke(
            "process",
            ToolContext {
                session_id: Some(SessionId("session-native-tools".into())),
                correlation_id: Some(CorrelationId("corr-native-tools".into())),
                execution_attempt_id: None,
                idempotency_key: None,
            },
            ToolCallRequest {
                name: "process".into(),
                input_json: json!({"action":"poll", "sessionId": process_id, "timeout": 3000})
                    .to_string(),
            },
        )
        .await
        .expect("native process poll should invoke");
    let process_poll_json: Value =
        serde_json::from_str(process_poll.structured_json.as_deref().unwrap())
            .expect("process poll output should parse");
    assert_eq!(process_poll_json["tool"], json!("process"));
    assert_eq!(process_poll_json["proxy_used"], json!(false));

    let tts_error = registry
        .invoke(
            "tts",
            context.clone(),
            ToolCallRequest {
                name: "tts".into(),
                input_json: json!({
                    "text":"hello",
                    "path":"artifacts/direct-tts.aiff",
                    "dryRun":true
                })
                .to_string(),
            },
        )
        .await
        .expect_err("direct TTS output must fail closed");
    assert!(tts_error.0.contains("identity-bound"));

    for (tool, payload) in [
        (
            "message",
            json!({"action":"send", "channel":"telegram", "target":"6476198178", "message":"dry run", "dryRun": true}),
        ),
        (
            "image_generate",
            json!({"prompt":"tiny red dot", "dryRun": true}),
        ),
        (
            "music_generate",
            json!({"prompt":"tiny tune", "dryRun": true}),
        ),
        (
            "video_generate",
            json!({"prompt":"tiny clip", "dryRun": true}),
        ),
    ] {
        let result = registry
            .invoke(
                tool,
                ToolContext {
                    session_id: Some(SessionId("session-native-tools".into())),
                    correlation_id: Some(CorrelationId("corr-native-tools".into())),
                    execution_attempt_id: None,
                    idempotency_key: None,
                },
                ToolCallRequest {
                    name: tool.into(),
                    input_json: payload.to_string(),
                },
            )
            .await
            .expect("native dry-run surface should invoke");
        let parsed: Value = serde_json::from_str(result.structured_json.as_deref().unwrap())
            .expect("native dry-run output should parse");
        assert_eq!(parsed["proxy_used"], json!(false));
        assert_ne!(parsed["status"], json!("native_surface_registered"));
    }
}

#[tokio::test]
async fn saves_and_loads_runtime_snapshot_across_instances() {
    let source = RuntimeKernel::new();
    source
        .run_demo_turn("hello persistence")
        .await
        .expect("plain turn should succeed");
    source
        .switch_model("mock-ollama/local-precise")
        .expect("model switch should succeed");
    source
        .approve_tool("read_file")
        .expect("approval should succeed");
    source
        .run_demo_turn(&architecture_foundation_read_intent())
        .await
        .expect("approved read turn should succeed");

    let snapshot_path = test_artifact_path(format!(
        "hepta-runtime-snapshot-{}.json",
        std::process::id()
    ));
    source
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");

    let selection = restored.model_selection().expect("selection should load");
    assert_eq!(selection.active.provider, "mock-ollama");
    assert_eq!(selection.active.model, "local-precise");

    let approvals = restored.approval_snapshot().expect("approvals should load");
    assert!(
        approvals
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let sessions = restored.sessions().expect("sessions should load");
    assert_eq!(sessions.len(), 1);
    let history = restored
        .history(Some("session-main"), 10)
        .expect("history should load");
    assert!(history.len() >= 2);
    let memories = restored.memory_snapshot(10).expect("memories should load");
    assert!(
        memories
            .iter()
            .any(|memory| memory.content.contains("hello persistence"))
    );

    let _ = std::fs::remove_file(snapshot_path);
}

#[tokio::test]
async fn saves_and_loads_runtime_snapshot_with_topic_sessions_and_graph_store() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("session switch should succeed");
    source
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    source
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    source
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    source
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");
    source
        .route_topics(
            "alpha",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("mixed route should succeed");

    let snapshot_path = test_artifact_path(format!(
        "hepta-runtime-topic-graph-snapshot-{}.json",
        std::process::id()
    ));
    source
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");

    let raw_topic_sessions = restored
        .topic_session_state
        .lock()
        .expect("topic session state lock should succeed")
        .sessions
        .clone();
    let raw_topic_graph_edges = restored
        .topic_graph_state
        .lock()
        .expect("topic graph state lock should succeed")
        .edges
        .clone();
    assert_eq!(raw_topic_sessions.len(), 2);
    assert!(raw_topic_graph_edges.iter().any(|record| {
        record.source_topic_session_id == "topic-session-bootstrap:alpha"
            && record.edge.target_topic_session_id
                == "topic-session-bootstrap:alpha:rust-worker-pipeline"
    }));

    let topic_sessions = restored
        .topic_sessions_for_surface("alpha")
        .expect("topic sessions should load");
    assert!(topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:alpha"
            && !topic_session.graph_edges.is_empty()
    }));

    let decision = restored
        .route_topics("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
        .expect("graph-expanded route should succeed");
    assert!(
        decision
            .active_topic_session_ids
            .iter()
            .any(|id| { id == "topic-session-bootstrap:alpha:rust-worker-pipeline" })
    );
    assert!(decision.activation_scores.iter().any(|score| {
        score.topic_id.0 == "topic-alpha-rust-worker-pipeline"
            && score
                .reason
                .as_deref()
                .is_some_and(|reason| reason.contains("stored co-activation edge"))
    }));

    let _ = std::fs::remove_file(snapshot_path);
}

#[tokio::test]
async fn loads_legacy_runtime_snapshot_missing_approvals_field() {
    let source = RuntimeKernel::new();
    source
        .run_demo_turn("legacy snapshot")
        .await
        .expect("plain turn should succeed");

    let snapshot_path = test_artifact_path(format!(
        "hepta-legacy-runtime-snapshot-{}.json",
        std::process::id()
    ));
    source
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let mut snapshot_json: Value = serde_json::from_str(
        &fs::read_to_string(&snapshot_path).expect("snapshot should be readable"),
    )
    .expect("snapshot json should parse");
    let snapshot_object = snapshot_json
        .as_object_mut()
        .expect("snapshot json should be an object");
    snapshot_object.remove("approvals");
    snapshot_object.remove("topic_sessions");
    snapshot_object.remove("topic_graph_edges");
    fs::write(
        &snapshot_path,
        serde_json::to_string_pretty(&snapshot_json).expect("snapshot should serialize"),
    )
    .expect("legacy snapshot should be writable");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("legacy snapshot load should succeed");

    let approvals = restored.approval_snapshot().expect("approvals should load");
    assert!(approvals.granted_tools.is_empty());
    assert!(approvals.pending.is_empty());

    let history = restored
        .history(Some("session-main"), 10)
        .expect("history should load");
    assert!(!history.is_empty());

    let _ = std::fs::remove_file(snapshot_path);
}

#[tokio::test]
async fn switches_active_session_and_persists_it() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("research-lab")
        .expect("session switch should succeed");
    runtime
        .run_demo_turn("hello switched session")
        .await
        .expect("turn should succeed");

    assert_eq!(
        runtime.active_session_id().expect("session id should load"),
        "research-lab"
    );
    let sessions = runtime.sessions().expect("sessions should load");
    let session = sessions
        .iter()
        .find(|session| session.session_id == "research-lab")
        .expect("research-lab session should exist");
    assert!(session.is_active);
    assert!(session.last_active_unix_ms >= session.created_at_unix_ms);
    let history = runtime
        .history(Some("research-lab"), 10)
        .expect("history should load");
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].input, "hello switched session");
}

#[tokio::test]
async fn can_rename_session_and_track_last_user_intent() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("research-lab")
        .expect("session switch should succeed");
    runtime
        .rename_active_session("Research planning")
        .expect("session rename should succeed");
    runtime
        .run_demo_turn("map out the next architecture milestone for Hepta")
        .await
        .expect("turn should succeed");
    runtime
        .route_topics(
            "research-lab",
            Some("map out the next architecture milestone for Hepta"),
            4,
            4,
            4,
            1,
        )
        .expect("topic route should succeed");

    let session = runtime
        .active_session_snapshot()
        .expect("active session snapshot should load");
    assert_eq!(session.title, "Research planning");
    assert_eq!(
        session.last_user_intent_summary.as_deref(),
        Some("map out the next architecture milestone for Hepta")
    );
    assert_eq!(session.topic_session_count, 1);
    assert_eq!(session.topic_graph_edge_count, 0);
}

#[tokio::test]
async fn can_run_in_specific_session_without_switching_active_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");

    let result = runtime
        .run_demo_turn_in_session("beta", "draft a beta session plan")
        .await
        .expect("beta run should succeed");

    assert_eq!(result.session_id, "beta");
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );

    let beta_history = runtime
        .history(Some("beta"), 10)
        .expect("beta history should load");
    assert_eq!(beta_history.len(), 1);
    assert_eq!(beta_history[0].input, "draft a beta session plan");

    let alpha_session = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "alpha")
        .expect("alpha session should exist");
    assert!(alpha_session.is_active);
}

#[test]
fn models_are_scoped_per_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .switch_model("mock-ollama/local-precise")
        .expect("alpha model switch should succeed");
    runtime
        .switch_model_in_session("beta", "demo/demo-creative")
        .expect("beta model switch should succeed");

    let alpha = runtime
        .model_selection_for_session("alpha")
        .expect("alpha model selection should load");
    assert_eq!(alpha.active.provider, "mock-ollama");
    assert_eq!(alpha.active.model, "local-precise");

    let beta = runtime
        .model_selection_for_session("beta")
        .expect("beta model selection should load");
    assert_eq!(beta.active.provider, "demo");
    assert_eq!(beta.active.model, "demo-creative");

    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );

    let beta_session = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "beta")
        .expect("beta session should exist");
    assert_eq!(beta_session.model.provider, "demo");
    assert_eq!(beta_session.model.model, "demo-creative");
}

#[tokio::test]
async fn query_events_filters_by_kind_session_and_limit() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn("hello alpha")
        .await
        .expect("alpha turn should succeed");
    runtime
        .switch_session("beta")
        .expect("beta session switch should succeed");
    runtime
        .run_demo_turn("hello beta")
        .await
        .expect("beta turn should succeed");

    let beta_switch_events = runtime
        .query_events(25, Some(&EventKind::SessionSwitched), Some("beta"))
        .expect("filtered beta events should load");
    assert_eq!(beta_switch_events.len(), 1);
    assert_eq!(beta_switch_events[0].event.kind, EventKind::SessionSwitched);
    assert_eq!(
        beta_switch_events[0]
            .event
            .session_id
            .as_ref()
            .map(|session_id| session_id.0.as_str()),
        Some("beta")
    );

    let limited_switch_events = runtime
        .query_events(1, Some(&EventKind::SessionSwitched), None)
        .expect("limited switch events should load");
    assert_eq!(limited_switch_events.len(), 1);
    assert_eq!(
        limited_switch_events[0]
            .event
            .session_id
            .as_ref()
            .map(|session_id| session_id.0.as_str()),
        Some("beta")
    );
}

#[tokio::test]
async fn approvals_are_scoped_per_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .approve_tool("read_file")
        .expect("alpha approval should succeed");
    let alpha = runtime
        .approval_snapshot()
        .expect("alpha approvals should load");
    assert!(alpha.granted_tools.iter().any(|tool| tool == "read_file"));

    runtime
        .switch_session("beta")
        .expect("beta session switch should succeed");
    let beta = runtime
        .approval_snapshot()
        .expect("beta approvals should load");
    assert!(beta.granted_tools.is_empty());

    let blocked = runtime
        .run_demo_turn(&architecture_foundation_read_intent())
        .await
        .expect("beta read turn should return approval requirement");
    assert_eq!(blocked.approval_required.as_deref(), Some("read_file"));

    runtime
        .switch_session("alpha")
        .expect("switch back to alpha should succeed");
    let alpha_again = runtime
        .approval_snapshot()
        .expect("alpha approvals should still load");
    assert!(
        alpha_again
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );
}

#[tokio::test]
async fn can_grant_and_inspect_approvals_for_non_active_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");

    let alpha = runtime
        .approval_snapshot()
        .expect("alpha approvals should load");
    assert!(alpha.granted_tools.is_empty());

    let beta = runtime
        .approval_snapshot_for_session("beta")
        .expect("beta approvals should load");
    assert!(beta.granted_tools.iter().any(|tool| tool == "read_file"));

    let result = runtime
        .run_demo_turn_in_session("beta", &architecture_foundation_read_intent())
        .await
        .expect("beta read turn should succeed");
    assert_eq!(result.invoked_tool.as_deref(), Some("read_file"));
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );
}

#[tokio::test]
async fn archiving_active_session_switches_to_fallback() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn("keep alpha history")
        .await
        .expect("alpha turn should succeed");

    runtime
        .archive_session(None)
        .expect("archive should succeed");

    assert_ne!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );
    let alpha = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "alpha")
        .expect("alpha session should exist");
    assert!(alpha.archived_at_unix_ms.is_some());
}

#[tokio::test]
async fn archiving_fresh_active_session_materializes_and_switches_to_fallback() {
    let runtime = RuntimeKernel::new();

    runtime
        .archive_session(None)
        .expect("archive should succeed for fresh active session");

    assert_ne!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "session-main"
    );
    let archived = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "session-main")
        .expect("session-main should exist");
    assert!(archived.archived_at_unix_ms.is_some());
}

#[tokio::test]
async fn deleting_session_removes_related_runtime_state() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta note")
        .await
        .expect("beta turn should succeed");
    runtime
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");
    runtime
        .switch_model_in_session("beta", "demo/demo-creative")
        .expect("beta model switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "hello adaptive memory")
        .await
        .expect("beta routed turn should succeed");
    runtime
        .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("beta route should succeed");

    runtime
        .delete_session("beta")
        .expect("delete should succeed");

    assert!(
        runtime
            .history(Some("beta"), 10)
            .expect("beta history should load")
            .is_empty()
    );
    assert!(
        runtime
            .approval_snapshot_for_session("beta")
            .expect("beta approvals should load")
            .granted_tools
            .is_empty()
    );
    assert!(
        runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .all(|session| session.session_id != "beta")
    );
    assert!(
        runtime
            .topic_sessions_for_surface("beta")
            .expect("beta topic sessions should load")
            .is_empty()
    );
    assert!(
        runtime
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .iter()
            .all(|record| {
                !record.source_topic_session_id.contains("beta")
                    && !record.edge.target_topic_session_id.contains("beta")
            })
    );
}

#[tokio::test]
async fn prune_prefers_archived_sessions_and_keeps_active() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn("alpha work")
        .await
        .expect("alpha turn should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta work")
        .await
        .expect("beta turn should succeed");
    runtime
        .run_demo_turn_in_session("gamma", "gamma work")
        .await
        .expect("gamma turn should succeed");
    runtime
        .archive_session(Some("beta"))
        .expect("beta archive should succeed");

    let result = runtime.prune_sessions(2).expect("prune should succeed");
    assert!(result.contains("beta"));
    let sessions = runtime.sessions().expect("sessions should load");
    assert!(
        sessions
            .iter()
            .any(|session| session.session_id == "alpha" && session.is_active)
    );
    assert!(sessions.iter().all(|session| session.session_id != "beta"));
}

#[tokio::test]
async fn prune_sessions_counts_fresh_active_session() {
    let runtime = RuntimeKernel::new();

    let result = runtime
        .prune_sessions(1)
        .expect("prune should succeed for fresh runtime");

    assert_eq!(result, "no pruning needed, sessions=1 max=1");
    let sessions = runtime.sessions().expect("sessions should load");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].session_id, "session-main");
    assert!(sessions[0].is_active);
}

#[tokio::test]
async fn exports_and_imports_single_session_package() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    source
        .run_demo_turn_in_session("beta", "beta exported work")
        .await
        .expect("beta turn should succeed");
    source
        .rename_active_session("Alpha workspace")
        .expect("alpha rename should succeed");
    source
        .switch_model_in_session("beta", "demo/demo-creative")
        .expect("beta model switch should succeed");
    source
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");
    source
        .archive_session(Some("beta"))
        .expect("beta archive should succeed");

    let export_path =
        test_artifact_path(format!("hepta-session-export-{}.json", std::process::id()));
    let export_report = source
        .export_session("beta", export_path.to_str().expect("path should be utf8"))
        .expect("beta export should succeed");
    assert_eq!(export_report.session_id, "beta");
    assert_eq!(export_report.title, "Hepta session beta");
    assert_eq!(export_report.model.model, "demo-creative");
    assert!(export_report.archived);
    assert_eq!(export_report.approvals_granted, 1);
    assert_eq!(export_report.history_entries, 1);
    assert_eq!(export_report.topic_session_count, 0);
    assert_eq!(export_report.topic_graph_edge_count, 0);

    let restored = RuntimeKernel::new();
    let import_report = restored
        .import_session(export_path.to_str().expect("path should be utf8"))
        .expect("beta import should succeed");
    assert_eq!(import_report.session_id, "beta");
    assert_eq!(import_report.imported_title, "Hepta session beta");
    assert_eq!(import_report.imported_model.model, "demo-creative");
    assert!(import_report.imported_archived);
    assert_eq!(import_report.approvals_granted, 1);
    assert_eq!(import_report.history_entries, 1);
    assert_eq!(import_report.topic_session_count, 0);
    assert_eq!(import_report.topic_graph_edge_count, 0);

    let beta = restored
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "beta")
        .expect("beta session should exist after import");
    assert_eq!(beta.model.provider, "demo");
    assert_eq!(beta.model.model, "demo-creative");
    assert!(beta.archived_at_unix_ms.is_some());
    assert_eq!(
        beta.last_user_intent_summary.as_deref(),
        Some("beta exported work")
    );

    let beta_approvals = restored
        .approval_snapshot_for_session("beta")
        .expect("beta approvals should load");
    assert!(
        beta_approvals
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let beta_history = restored
        .history(Some("beta"), 10)
        .expect("beta history should load");
    assert_eq!(beta_history.len(), 1);
    assert_eq!(beta_history[0].input, "beta exported work");

    let _ = std::fs::remove_file(export_path);
}

#[tokio::test]
async fn exports_and_imports_single_session_package_with_topic_graph_state() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    source
        .run_demo_turn_in_session("beta", "hello adaptive memory")
        .await
        .expect("beta first turn should succeed");
    source
        .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("beta first route should succeed");
    source
        .run_demo_turn_in_session("beta", "rust worker pipeline")
        .await
        .expect("beta second turn should succeed");
    source
        .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("beta second route should succeed");
    source
        .route_topics(
            "beta",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("beta mixed route should succeed");

    let export_path = test_artifact_path(format!(
        "hepta-session-topic-graph-export-{}.json",
        std::process::id()
    ));
    let export_report = source
        .export_session("beta", export_path.to_str().expect("path should be utf8"))
        .expect("beta export should succeed");
    assert_eq!(export_report.topic_session_count, 2);
    assert_eq!(export_report.topic_graph_edge_count, 2);

    let restored = RuntimeKernel::new();
    let import_report = restored
        .import_session(export_path.to_str().expect("path should be utf8"))
        .expect("beta import should succeed");
    assert_eq!(import_report.topic_session_count, 2);
    assert_eq!(import_report.topic_graph_edge_count, 2);

    let raw_topic_graph_edges = restored
        .topic_graph_state
        .lock()
        .expect("topic graph state lock should succeed")
        .edges
        .clone();
    assert!(raw_topic_graph_edges.iter().any(|record| {
        record.source_topic_session_id == "topic-session-bootstrap:beta"
            && record.edge.target_topic_session_id
                == "topic-session-bootstrap:beta:rust-worker-pipeline"
    }));

    let decision = restored
        .route_topics("beta", Some("hello adaptive memory"), 8, 8, 8, 2)
        .expect("graph-expanded route should succeed");
    assert!(
        decision
            .active_topic_session_ids
            .iter()
            .any(|id| { id == "topic-session-bootstrap:beta:rust-worker-pipeline" })
    );

    let _ = std::fs::remove_file(export_path);
}

#[tokio::test]
async fn session_export_roundtrip_preserves_intelligence_learning_state() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    for input in [
        "semantic router should learn from accepted feedback",
        "feedback calibration closes the loop into future intuition",
        "merge topic sessions then split them back into stable neurons",
        "aging neurons need refresh with transcript evidence",
    ] {
        source
            .run_demo_turn_in_session("beta", input)
            .await
            .expect("intelligence hardening turn should succeed");
    }

    let bundle = source
        .predict_intuition(
            "beta",
            "semantic router learned feedback should route topic memory",
            12,
            12,
            12,
            6,
            6,
            6,
        )
        .expect("intuition should produce a bundle");
    assert!(!bundle.topic_activation_scores.is_empty());
    assert!(!bundle.neuron_activations.is_empty());

    let skill_id = bundle
        .skill_decisions
        .first()
        .map(|decision| decision.skill_id.clone());
    let workflow_id = bundle
        .workflow_priors
        .first()
        .map(|prior| prior.workflow_id.clone());
    let source_topic_ids = bundle
        .topic_activation_scores
        .iter()
        .map(|score| score.topic_id.clone())
        .collect::<Vec<_>>();
    let source_neuron_ids = bundle
        .neuron_activations
        .iter()
        .map(|activation| activation.neuron_id.clone())
        .collect::<Vec<_>>();
    source
        .record_intuition_feedback(
            "beta",
            "semantic router learned feedback should route topic memory",
            IntuitionFeedbackOutcome::ExecutedSuccess,
            skill_id.as_deref(),
            workflow_id.as_deref(),
            source_topic_ids.clone(),
            source_neuron_ids,
            Some("release hardening accepted learned semantic router"),
        )
        .expect("feedback learning should be recorded");
    source
        .record_model_router_feedback(
            "beta",
            "semantic router learned feedback should route topic memory",
            ModelRef {
                provider: "demo".into(),
                model: "demo-chat".into(),
            },
            TopicAwareModelFeedbackOutcome::ExecutedSuccess,
            source_topic_ids.clone(),
            Some(1200),
            Some(0.03),
            Some(0.9),
            Some(0.8),
            Some("model-router feedback survived export"),
        )
        .expect("model-router feedback should be recorded");

    let before_route = source
        .route_topics(
            "beta",
            Some("semantic router learned feedback release hardening"),
            12,
            12,
            12,
            6,
        )
        .expect("learned router route should succeed before export");
    assert_eq!(
        before_route.router_id,
        "semantic-router:learned-feedback-v1"
    );
    assert!(before_route.learned_signal_count > 0);

    let before_calibration = source
        .intuition_calibration_overview("beta")
        .expect("calibration overview should load before export");
    assert!(before_calibration.closed_loop_ready);
    assert!(before_calibration.learned_topic_hint_count > 0);
    assert!(before_calibration.learned_neuron_update_count > 0);
    let before_model_calibration = source
        .model_router_feedback_summary("beta")
        .expect("model-router calibration should load before export");
    assert_eq!(before_model_calibration.len(), 1);
    assert!(before_model_calibration[0].success_rate > 0.0);

    let before_lifecycle = source
        .neuron_lifecycle_overview("beta")
        .expect("lifecycle overview should load before export");
    assert!(before_lifecycle.stored_neurons > 0);
    assert!(before_lifecycle.average_confidence > 0.0);

    let export_path = test_artifact_path(format!(
        "hepta-session-intelligence-export-{}.json",
        std::process::id()
    ));
    let export_report = source
        .export_session("beta", export_path.to_str().expect("path should be utf8"))
        .expect("beta intelligence export should succeed");
    assert_eq!(export_report.neuron_count, before_lifecycle.stored_neurons);
    assert_eq!(
        export_report.intuition_feedback_count,
        before_calibration.feedback_record_count
    );
    assert_eq!(
        export_report.model_router_feedback_count,
        before_model_calibration[0].record_count
    );

    let restored = RuntimeKernel::new();
    let import_report = restored
        .import_session(export_path.to_str().expect("path should be utf8"))
        .expect("beta intelligence import should succeed");
    assert_eq!(import_report.neuron_count, before_lifecycle.stored_neurons);
    assert_eq!(
        import_report.intuition_feedback_count,
        before_calibration.feedback_record_count
    );
    assert_eq!(
        import_report.model_router_feedback_count,
        before_model_calibration[0].record_count
    );

    let after_route = restored
        .route_topics(
            "beta",
            Some("semantic router learned feedback release hardening"),
            12,
            12,
            12,
            6,
        )
        .expect("learned router route should succeed after import");
    assert_eq!(after_route.router_id, "semantic-router:learned-feedback-v1");
    assert!(after_route.learned_signal_count >= before_route.learned_signal_count);

    let after_calibration = restored
        .intuition_calibration_overview("beta")
        .expect("calibration overview should load after import");
    assert_eq!(
        after_calibration.feedback_record_count,
        before_calibration.feedback_record_count
    );
    assert!(after_calibration.closed_loop_ready);
    assert_eq!(
        after_calibration.learned_neuron_update_count,
        before_calibration.learned_neuron_update_count
    );
    let after_model_calibration = restored
        .model_router_feedback_summary("beta")
        .expect("model-router calibration should load after import");
    assert_eq!(after_model_calibration, before_model_calibration);

    let after_lifecycle = restored
        .neuron_lifecycle_overview("beta")
        .expect("lifecycle overview should load after import");
    assert_eq!(
        after_lifecycle.stored_neurons,
        before_lifecycle.stored_neurons
    );
    assert!(after_lifecycle.average_confidence > 0.0);

    let _ = std::fs::remove_file(export_path);
}

#[tokio::test]
async fn imports_legacy_session_export_missing_approval_field() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    source
        .run_demo_turn_in_session("beta", "legacy export")
        .await
        .expect("beta turn should succeed");

    let export_path = test_artifact_path(format!(
        "hepta-legacy-session-export-{}.json",
        std::process::id()
    ));
    source
        .export_session("beta", export_path.to_str().expect("path should be utf8"))
        .expect("beta export should succeed");

    let mut export_json: Value =
        serde_json::from_str(&fs::read_to_string(&export_path).expect("export should be readable"))
            .expect("export json should parse");
    let export_object = export_json
        .as_object_mut()
        .expect("export json should be an object");
    export_object.remove("approval");
    export_object.remove("topic_sessions");
    export_object.remove("topic_graph_edges");
    fs::write(
        &export_path,
        serde_json::to_string_pretty(&export_json).expect("export should serialize"),
    )
    .expect("legacy export should be writable");

    let restored = RuntimeKernel::new();
    let import_report = restored
        .import_session(export_path.to_str().expect("path should be utf8"))
        .expect("legacy export import should succeed");
    assert_eq!(import_report.topic_session_count, 0);
    assert_eq!(import_report.topic_graph_edge_count, 0);

    let beta_approvals = restored
        .approval_snapshot_for_session("beta")
        .expect("beta approvals should load");
    assert!(beta_approvals.granted_tools.is_empty());
    assert!(beta_approvals.pending.is_empty());

    let beta_history = restored
        .history(Some("beta"), 10)
        .expect("beta history should load");
    assert_eq!(beta_history.len(), 1);
    assert_eq!(beta_history[0].input, "legacy export");

    let _ = std::fs::remove_file(export_path);
}

#[tokio::test]
async fn forks_session_into_independent_branch() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta base work")
        .await
        .expect("beta turn should succeed");
    runtime
        .switch_model_in_session("beta", "demo/demo-creative")
        .expect("beta model switch should succeed");
    runtime
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");
    runtime
        .archive_session(Some("beta"))
        .expect("beta archive should succeed");

    let fork_report = runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    assert_eq!(fork_report.source_session_id, "beta");
    assert_eq!(fork_report.target_session_id, "beta-fork");
    assert_eq!(fork_report.target_model.model, "demo-creative");
    assert!(!fork_report.target_archived);
    assert_eq!(fork_report.approvals_granted, 1);
    assert_eq!(fork_report.history_entries, 1);
    assert_eq!(fork_report.topic_session_count, 0);
    assert_eq!(fork_report.topic_graph_edge_count, 0);
    assert_eq!(fork_report.active_session_after_fork, "alpha");

    let fork = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "beta-fork")
        .expect("beta-fork session should exist");
    assert_eq!(fork.model.provider, "demo");
    assert_eq!(fork.model.model, "demo-creative");
    assert!(fork.archived_at_unix_ms.is_none());
    assert_eq!(
        fork.last_user_intent_summary.as_deref(),
        Some("beta base work")
    );
    assert!(fork.title.contains("(fork)"));

    let fork_approvals = runtime
        .approval_snapshot_for_session("beta-fork")
        .expect("fork approvals should load");
    assert!(
        fork_approvals
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let fork_history = runtime
        .history(Some("beta-fork"), 10)
        .expect("fork history should load");
    assert_eq!(fork_history.len(), 1);
    assert_eq!(fork_history[0].session_id, "beta-fork");
    assert_eq!(fork_history[0].input, "beta base work");

    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );
}

#[tokio::test]
async fn fork_session_rejects_fresh_active_target_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn_in_session("beta", "beta base work")
        .await
        .expect("beta turn should succeed");

    let err = runtime
        .fork_session("beta", "session-main")
        .expect_err("fresh active target should still be treated as existing");

    assert_eq!(err.0, "target session already exists: session-main");
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "session-main"
    );
    assert!(
        runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .any(|session| session.session_id == "session-main" && session.is_active)
    );
}

#[tokio::test]
async fn fork_session_rebases_topic_sessions_and_graph_state() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "hello adaptive memory")
        .await
        .expect("beta first turn should succeed");
    runtime
        .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("beta first route should succeed");
    runtime
        .run_demo_turn_in_session("beta", "rust worker pipeline")
        .await
        .expect("beta second turn should succeed");
    runtime
        .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("beta second route should succeed");
    runtime
        .route_topics(
            "beta",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("beta mixed route should succeed");

    let fork_report = runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    assert_eq!(fork_report.topic_session_count, 2);
    assert_eq!(fork_report.topic_graph_edge_count, 2);

    let fork_topic_sessions = runtime
        .topic_sessions_for_surface("beta-fork")
        .expect("fork topic sessions should load");
    assert!(fork_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:beta-fork"
            && topic_session.topic_id.0 == "topic-beta-fork"
    }));
    assert!(fork_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:beta-fork:rust-worker-pipeline"
            && topic_session.topic_id.0 == "topic-beta-fork-rust-worker-pipeline"
            && !topic_session.graph_edges.is_empty()
    }));

    let decision = runtime
        .route_topics("beta-fork", Some("hello adaptive memory"), 8, 8, 8, 2)
        .expect("fork graph-expanded route should succeed");
    assert!(
        decision
            .active_topic_session_ids
            .iter()
            .any(|id| { id == "topic-session-bootstrap:beta-fork:rust-worker-pipeline" })
    );
}

#[tokio::test]
async fn merges_session_into_target_without_overwriting_target_model_or_title() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("mainline")
        .expect("mainline session switch should succeed");
    runtime
        .rename_active_session("Mainline workspace")
        .expect("mainline rename should succeed");
    runtime
        .switch_model("mock-ollama/local-precise")
        .expect("mainline model switch should succeed");
    runtime
        .run_demo_turn("mainline seed")
        .await
        .expect("mainline turn should succeed");
    runtime
        .run_demo_turn_in_session("beta-fork", "fork delta")
        .await
        .expect("fork turn should succeed");
    runtime
        .approve_tool_in_session("beta-fork", "read_file")
        .expect("fork approval should succeed");
    runtime
        .archive_session(Some("beta-fork"))
        .expect("fork archive should succeed");

    runtime
        .merge_session("beta-fork", "mainline", MergeOptions::default())
        .expect("merge should succeed");

    let mainline = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "mainline")
        .expect("mainline session should exist");
    assert_eq!(mainline.title, "Mainline workspace");
    assert_eq!(mainline.model.provider, "mock-ollama");
    assert_eq!(mainline.model.model, "local-precise");
    assert!(mainline.archived_at_unix_ms.is_none());
    assert_eq!(
        mainline.last_user_intent_summary.as_deref(),
        Some("fork delta")
    );

    let approvals = runtime
        .approval_snapshot_for_session("mainline")
        .expect("mainline approvals should load");
    assert!(
        approvals
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let history = runtime
        .history(Some("mainline"), 10)
        .expect("mainline history should load");
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].input, "fork delta");
    assert_eq!(history[1].input, "mainline seed");
}

#[tokio::test]
async fn diffs_sessions_semantically_without_treating_forked_history_as_all_different() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "shared base")
        .await
        .expect("beta turn should succeed");
    runtime
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");
    runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    runtime
        .archive_session(Some("beta"))
        .expect("beta archive should succeed");
    runtime
        .switch_model_in_session("beta-fork", "demo/demo-creative")
        .expect("fork model switch should succeed");
    runtime
        .run_demo_turn_in_session("beta-fork", "fork-only delta")
        .await
        .expect("fork delta turn should succeed");

    let report = runtime
        .diff_sessions("beta", "beta-fork")
        .expect("diff should succeed");

    assert_eq!(report.left_session_id, "beta");
    assert_eq!(report.right_session_id, "beta-fork");
    assert_eq!(report.left_title, "Hepta session beta");
    assert_eq!(report.right_title, "Hepta session beta (fork)");
    assert_eq!(report.left_model.provider, "demo");
    assert_eq!(report.left_model.model, "demo-chat");
    assert_eq!(report.right_model.provider, "demo");
    assert_eq!(report.right_model.model, "demo-creative");
    assert!(report.left_archived);
    assert!(!report.right_archived);
    assert_eq!(report.left_history_count, 1);
    assert_eq!(report.right_history_count, 2);
    assert_eq!(report.shared_history_count, 1);
    assert!(report.approvals_only_left.is_empty());
    assert!(report.approvals_only_right.is_empty());
    assert!(report.history_only_left.is_empty());
    assert_eq!(report.history_only_right.len(), 1);
    assert!(report.history_only_right[0].contains("fork-only delta"));
    assert_eq!(
        report.left_last_user_intent_summary.as_deref(),
        Some("shared base")
    );
    assert_eq!(
        report.right_last_user_intent_summary.as_deref(),
        Some("fork-only delta")
    );
}

#[tokio::test]
async fn previews_deduplicating_merge_plan_for_forked_history() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("beta")
        .expect("beta session switch should succeed");
    runtime
        .run_demo_turn("shared base")
        .await
        .expect("beta base turn should succeed");
    runtime
        .approve_tool("read_file")
        .expect("beta approval should succeed");
    runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    runtime
        .switch_model_in_session("beta-fork", "demo/demo-creative")
        .expect("fork model switch should succeed");
    runtime
        .run_demo_turn_in_session("beta-fork", "fork-only delta")
        .await
        .expect("fork delta turn should succeed");

    let report = runtime
        .preview_merge_session("beta-fork", "beta", MergeOptions::default())
        .expect("merge preview should succeed");

    assert_eq!(report.source_session_id, "beta-fork");
    assert_eq!(report.target_session_id, "beta");
    assert_eq!(report.target_title_before, "Hepta session beta");
    assert_eq!(report.target_title_after, "Hepta session beta");
    assert_eq!(report.target_model_before.provider, "demo");
    assert_eq!(report.target_model_before.model, "demo-chat");
    assert_eq!(report.target_model_after.provider, "demo");
    assert_eq!(report.target_model_after.model, "demo-chat");
    assert!(!report.target_archived_before);
    assert!(!report.target_archived_after);
    assert!(!report.source_deleted_after_merge);
    assert_eq!(report.source_history_count, 2);
    assert_eq!(report.target_history_count, 1);
    assert_eq!(report.history_entries_to_append, 1);
    assert_eq!(report.history_entries_skipped_as_duplicates, 1);
    assert_eq!(report.source_topic_session_count, 0);
    assert_eq!(report.target_topic_session_count_before, 0);
    assert_eq!(report.target_topic_session_count_after, 0);
    assert_eq!(report.source_topic_graph_edge_count, 0);
    assert_eq!(report.target_topic_graph_edge_count_before, 0);
    assert_eq!(report.target_topic_graph_edge_count_after, 0);
    assert!(report.approvals_added_to_target.is_empty());
    assert!(report.pending_added_to_target.is_empty());
    assert_eq!(report.new_history_entries_to_append.len(), 1);
    assert!(report.new_history_entries_to_append[0].contains("fork-only delta"));
    assert_eq!(report.duplicate_history_entries_skipped.len(), 1);
    assert!(report.duplicate_history_entries_skipped[0].contains("shared base"));
    assert_eq!(
        report.target_last_user_intent_summary_before.as_deref(),
        Some("shared base")
    );
    assert_eq!(
        report.source_last_user_intent_summary.as_deref(),
        Some("fork-only delta")
    );
    assert_eq!(
        report.merged_last_user_intent_summary.as_deref(),
        Some("fork-only delta")
    );
}

#[tokio::test]
async fn preview_merge_session_surfaces_topic_state_plan() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("mainline")
        .expect("mainline session switch should succeed");
    runtime
        .run_demo_turn("mainline planning")
        .await
        .expect("mainline turn should succeed");
    runtime
        .route_topics("mainline", Some("mainline planning"), 4, 4, 4, 1)
        .expect("mainline route should succeed");
    runtime
        .run_demo_turn_in_session("feature", "hello adaptive memory")
        .await
        .expect("feature first turn should succeed");
    runtime
        .route_topics("feature", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("feature first route should succeed");
    runtime
        .run_demo_turn_in_session("feature", "rust worker pipeline")
        .await
        .expect("feature second turn should succeed");
    runtime
        .route_topics("feature", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("feature second route should succeed");
    runtime
        .route_topics(
            "feature",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("feature mixed route should succeed");

    let report = runtime
        .preview_merge_session("feature", "mainline", MergeOptions::default())
        .expect("merge preview should succeed");

    assert_eq!(report.source_topic_session_count, 2);
    assert_eq!(report.target_topic_session_count_before, 1);
    assert_eq!(report.target_topic_session_count_after, 3);
    assert_eq!(report.source_topic_graph_edge_count, 2);
    assert_eq!(report.target_topic_graph_edge_count_before, 0);
    assert_eq!(report.target_topic_graph_edge_count_after, 2);
}

#[tokio::test]
async fn merge_session_deduplicates_shared_history_from_forked_source() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("beta")
        .expect("beta session switch should succeed");
    runtime
        .run_demo_turn("shared base")
        .await
        .expect("beta base turn should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("beta topic turn should succeed");
    runtime
        .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("beta first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("beta second topic turn should succeed");
    runtime
        .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("beta second route should succeed");
    runtime
        .route_topics(
            "beta",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("beta mixed route should succeed");
    runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    runtime
        .run_demo_turn_in_session("beta-fork", "fork-only delta")
        .await
        .expect("fork delta turn should succeed");

    let merge_result = runtime
        .merge_session("beta-fork", "beta", MergeOptions::default())
        .expect("merge should succeed");
    assert_eq!(merge_result.appended_history_entries, 1);
    assert_eq!(merge_result.skipped_duplicate_history_entries, 3);
    assert_eq!(merge_result.target_session_id, "beta");
    assert_eq!(merge_result.target_title_after, "Hepta session beta");
    assert_eq!(merge_result.target_model_after.model, "demo-chat");
    assert_eq!(merge_result.source_topic_session_count, 2);
    assert_eq!(merge_result.target_topic_session_count_before, 2);
    assert_eq!(merge_result.target_topic_session_count_after, 2);
    assert_eq!(merge_result.source_topic_graph_edge_count, 2);
    assert_eq!(merge_result.target_topic_graph_edge_count_before, 2);
    assert_eq!(merge_result.target_topic_graph_edge_count_after, 2);

    let history = runtime
        .history(Some("beta"), 10)
        .expect("beta history should load");
    assert_eq!(history.len(), 4);
    assert_eq!(history[0].input, "fork-only delta");
    assert_eq!(history[1].input, "rust worker pipeline");
    assert_eq!(history[2].input, "hello adaptive memory");
    assert_eq!(history[3].input, "shared base");

    let beta_topic_sessions = runtime
        .topic_sessions_for_surface("beta")
        .expect("beta topic sessions should load");
    assert_eq!(beta_topic_sessions.len(), 2);
    assert!(beta_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:beta"
            && !topic_session.graph_edges.is_empty()
    }));
    assert!(beta_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:beta:rust-worker-pipeline"
    }));
}

#[tokio::test]
async fn merge_session_materializes_fresh_active_target_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn_in_session("feature", "feature base")
        .await
        .expect("feature base turn should succeed");

    let merge_result = runtime
        .merge_session("feature", "session-main", MergeOptions::default())
        .expect("merge into fresh active target should succeed");

    assert_eq!(merge_result.target_session_id, "session-main");
    assert_eq!(
        merge_result.target_title_after,
        "Hepta session session-main"
    );
    assert_eq!(merge_result.target_model_after.model, "demo-chat");
    assert_eq!(merge_result.appended_history_entries, 1);
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "session-main"
    );

    let session_main = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "session-main")
        .expect("session-main should exist");
    assert_eq!(
        session_main.last_user_intent_summary.as_deref(),
        Some("feature base")
    );

    let history = runtime
        .history(Some("session-main"), 10)
        .expect("session-main history should load");
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].input, "feature base");
}

#[tokio::test]
async fn merge_session_rebases_unrelated_topic_graph_state_into_target_namespace() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("mainline")
        .expect("mainline session switch should succeed");
    runtime
        .run_demo_turn("mainline planning")
        .await
        .expect("mainline turn should succeed");
    runtime
        .route_topics("mainline", Some("mainline planning"), 4, 4, 4, 1)
        .expect("mainline route should succeed");
    runtime
        .run_demo_turn_in_session("feature", "hello adaptive memory")
        .await
        .expect("feature first turn should succeed");
    runtime
        .route_topics("feature", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("feature first route should succeed");
    runtime
        .run_demo_turn_in_session("feature", "rust worker pipeline")
        .await
        .expect("feature second turn should succeed");
    runtime
        .route_topics("feature", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("feature second route should succeed");
    runtime
        .route_topics(
            "feature",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("feature mixed route should succeed");

    let merge_result = runtime
        .merge_session("feature", "mainline", MergeOptions::default())
        .expect("merge should succeed");
    assert_eq!(merge_result.source_topic_session_count, 2);
    assert_eq!(merge_result.target_topic_session_count_before, 1);
    assert_eq!(merge_result.target_topic_session_count_after, 3);
    assert_eq!(merge_result.source_topic_graph_edge_count, 2);
    assert_eq!(merge_result.target_topic_graph_edge_count_before, 0);
    assert_eq!(merge_result.target_topic_graph_edge_count_after, 2);

    let mainline_topic_sessions = runtime
        .topic_sessions_for_surface("mainline")
        .expect("mainline topic sessions should load");
    assert!(mainline_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:mainline"
            && topic_session.topic_id.0 == "topic-mainline"
    }));
    assert!(mainline_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:mainline:feature"
            && topic_session.topic_id.0 == "topic-mainline-feature"
    }));
    assert!(mainline_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id
            == "topic-session-bootstrap:mainline:feature:rust-worker-pipeline"
            && topic_session.topic_id.0 == "topic-mainline-feature-rust-worker-pipeline"
    }));
    assert!(
        runtime
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .iter()
            .any(|record| {
                record.source_topic_session_id == "topic-session-bootstrap:mainline:feature"
                    && record.edge.target_topic_session_id
                        == "topic-session-bootstrap:mainline:feature:rust-worker-pipeline"
            })
    );
}

#[tokio::test]
async fn merge_session_can_adopt_model_title_and_delete_source() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("mainline")
        .expect("mainline session switch should succeed");
    runtime
        .rename_active_session("Mainline workspace")
        .expect("mainline rename should succeed");
    runtime
        .run_demo_turn_in_session("feature", "feature base")
        .await
        .expect("feature base turn should succeed");
    runtime
        .switch_session("feature")
        .expect("feature session switch should succeed");
    runtime
        .rename_active_session("Feature workspace")
        .expect("feature rename should succeed");
    runtime
        .switch_model("demo/demo-creative")
        .expect("feature model switch should succeed");

    let preview = runtime
        .preview_merge_session(
            "feature",
            "mainline",
            MergeOptions {
                adopt_model: true,
                adopt_title: true,
                delete_source: true,
            },
        )
        .expect("merge preview should succeed");
    assert_eq!(preview.target_title_after, "Feature workspace");
    assert_eq!(preview.target_model_after.model, "demo-creative");
    assert!(preview.source_deleted_after_merge);

    let merge_result = runtime
        .merge_session(
            "feature",
            "mainline",
            MergeOptions {
                adopt_model: true,
                adopt_title: true,
                delete_source: true,
            },
        )
        .expect("merge should succeed");
    assert!(merge_result.options.adopt_title);
    assert!(merge_result.options.adopt_model);
    assert!(merge_result.options.delete_source);
    assert_eq!(merge_result.target_title_after, "Feature workspace");
    assert_eq!(merge_result.target_model_after.model, "demo-creative");
    assert!(merge_result.source_deleted_after_merge);

    let mainline = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "mainline")
        .expect("mainline session should exist");
    assert_eq!(mainline.title, "Feature workspace");
    assert_eq!(mainline.model.model, "demo-creative");
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "mainline"
    );
    assert!(
        runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .all(|session| session.session_id != "feature")
    );
}

mod architecture_v2_exact_safety_tests {
    use super::*;
    use crate::SafetyGateClient;

    include!("tests/architecture_v2_exact_safety_support.rs");
    include!("tests/architecture_v2_exact_safety.rs");
}

mod architecture_v2_execution_lease_tests {
    use super::*;
    use crate::ExecutionBus;
    use crate::SafetyGateClient;

    include!("tests/architecture_v2_execution_lease.rs");
}

mod architecture_v2_outcome_receipt_tests {
    include!("tests/architecture_v2_terminal_outcome_support.rs");
    include!("tests/architecture_v2_terminal_outcome.rs");
}

mod architecture_v2_outcome_flow_tests {
    include!("tests/architecture_v2_outcome_flow.rs");
}

mod architecture_v2_resource_reservation_tests {
    include!("tests/architecture_v2_resource_reservation.rs");
}

mod architecture_v2_capability_descriptor_tests {
    include!("tests/architecture_v2_capability_descriptor.rs");
}

mod architecture_v2_symlink_reservation_tests {
    include!("tests/architecture_v2_symlink_reservation.rs");
}

mod architecture_v2_dispatch_selector_tests {
    include!("tests/architecture_v2_dispatch_selector.rs");
}

mod architecture_v2_process_reservation_tests {
    include!("tests/architecture_v2_process_reservation.rs");
}

mod architecture_v2_native_mutation_tests {
    include!("tests/architecture_v2_native_mutation.rs");
}

mod architecture_v2_maintenance_mutation_tests {
    include!("tests/architecture_v2_maintenance_mutation.rs");
}

mod architecture_v2_process_control_tests {
    include!("tests/architecture_v2_process_control.rs");
}

mod architecture_v2_provider_idempotency_tests {
    include!("tests/architecture_v2_provider_idempotency.rs");
}

mod architecture_v2_provider_effect_tests {
    include!("tests/architecture_v2_provider_effect.rs");
}

mod architecture_v2_sealed_read_tests {
    include!("tests/architecture_v2_sealed_read.rs");
}

#[cfg(unix)]
#[test]
fn durable_runtime_hydrates_session_state_on_reopen() {
    let root = tempfile::tempdir().expect("tempdir");
    let outcome_path = root.path().join("outcomes.sqlite3");
    let state_path = root.path().join("runtime-state.json");
    let runtime = RuntimeKernel::bootstrap_with_durable_outcomes_and_state(
        &outcome_path,
        hepta_memory::DurableIntegrityKey::from_bytes([11; 32]),
        &state_path,
        hepta_memory::DurableIntegrityKey::from_bytes([12; 32]),
    )
    .expect("bootstrap durable runtime");
    runtime
        .switch_session("durable-session")
        .expect("create durable session");
    drop(runtime);

    let recovered = RuntimeKernel::open_with_durable_outcomes_and_state(
        &outcome_path,
        hepta_memory::DurableIntegrityKey::from_bytes([11; 32]),
        &state_path,
        hepta_memory::DurableIntegrityKey::from_bytes([12; 32]),
    )
    .expect("open durable runtime");
    assert!(
        recovered
            .sessions()
            .expect("list recovered sessions")
            .iter()
            .any(|session| session.session_id == "durable-session")
    );
}
