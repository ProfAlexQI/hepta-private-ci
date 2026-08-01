use super::*;

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
    assert!(super::super::paths_overlap(&base, &base));
    assert!(super::super::paths_overlap(&base, &child));
    assert!(super::super::paths_overlap(&child, &base));
    assert!(!super::super::paths_overlap(
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
    let payloads = super::super::openai_tool_payloads(&tools);
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

    let parsed = super::super::openai_tool_calls_from_message(&json!({
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

    let textual = super::super::textual_tool_calls_from_message_content(
        "<|tool_call>call:echo{text: \"ping\"}<tool_call|>",
        &tools,
    );
    assert_eq!(textual.len(), 1);
    assert_eq!(textual[0].name, "echo");
    assert_eq!(textual[0].arguments_json, "{\"text\":\"ping\"}");

    let json_textual = super::super::textual_tool_calls_from_message_content(
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

    assert!(super::super::apply_qwen_openai_compatible_thinking_params(
        &mut payload,
        Some(super::super::QwenThinkingFormat::ChatTemplate),
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
    assert!(super::super::apply_qwen_openai_compatible_thinking_params(
        &mut top_level_payload,
        Some(super::super::QwenThinkingFormat::TopLevel),
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
        super::super::extract_chatgpt_account_id_from_jwt(token).as_deref(),
        Some("acct_test_123")
    );
}

#[test]
fn openai_codex_profile_selection_prefers_freshest_unexpired_profile() {
    let stale_first = super::super::OpenAiCodexAuthProfile {
        path: PathBuf::from("hepta/auth-profiles.json"),
        profile_id: "openai-codex:stale".into(),
        access: "stale-access".into(),
        refresh: Some("stale-refresh".into()),
        expires: Some(1_000),
        account_id: "acct_stale".into(),
    };
    let fresh_default = super::super::OpenAiCodexAuthProfile {
        path: PathBuf::from("main/auth-profiles.json"),
        profile_id: "openai-codex:default".into(),
        access: "fresh-access".into(),
        refresh: Some("fresh-refresh".into()),
        expires: Some(500_000),
        account_id: "acct_fresh".into(),
    };
    let freshest = super::super::OpenAiCodexAuthProfile {
        path: PathBuf::from("main/auth-profiles.json"),
        profile_id: "openai-codex:newest".into(),
        access: "freshest-access".into(),
        refresh: Some("freshest-refresh".into()),
        expires: Some(900_000),
        account_id: "acct_freshest".into(),
    };

    let selected = super::super::select_openai_codex_auth_profile(
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
        super::super::normalize_openai_codex_profile_id_override(" qiqianpkugsm@gmail.com ")
            .as_deref(),
        Some("openai-codex:qiqianpkugsm@gmail.com")
    );
    assert_eq!(
        super::super::normalize_openai_codex_profile_id_override(
            "openai-codex:qiqianpkugsm@gmail.com",
        )
        .as_deref(),
        Some("openai-codex:qiqianpkugsm@gmail.com")
    );
    assert_eq!(
        super::super::normalize_openai_codex_profile_id_override("  "),
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

    let sanitized = super::super::sanitize_openai_codex_tool_schema(schema);

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

    let body = super::super::openai_codex_responses_request_body(&request, Some("session-1"));

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

    let response = super::super::parse_openai_codex_sse_response(sse).expect("SSE should parse");

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

    let response = super::super::parse_openai_codex_sse_response(sse).expect("SSE should parse");

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
