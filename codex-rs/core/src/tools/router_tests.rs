use std::sync::Arc;

use crate::config::Config;
use crate::function_tool::FunctionCallError;
use crate::session::tests::make_session_and_context;
use crate::tools::context::ToolPayload;
use crate::turn_diff_tracker::TurnDiffTracker;
use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistry;
use codex_extension_api::ExtensionRegistryBuilder;
use codex_extension_api::ResponsesApiTool;
use codex_extension_api::ToolCall as ExtensionToolCall;
use codex_extension_api::ToolExecutor;
use codex_protocol::dynamic_tools::DynamicToolSpec;
use codex_protocol::models::FunctionCallOutputBody;
use codex_protocol::models::ResponseInputItem;
use codex_protocol::models::ResponseItem;
use codex_tools::ResponsesApiNamespace;
use codex_tools::ResponsesApiNamespaceTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use codex_tools::default_namespace_description;
use pretty_assertions::assert_eq;
use serde_json::json;
use tokio_util::sync::CancellationToken;

use super::ToolCall;
use super::ToolCallSource;
use super::ToolRouter;
use super::ToolRouterParams;
use super::extension_tool_executors;

struct ExtensionEchoContributor;

impl codex_extension_api::ToolContributor for ExtensionEchoContributor {
    fn tools(
        &self,
        _session_store: &ExtensionData,
        _thread_store: &ExtensionData,
    ) -> Vec<Arc<dyn ToolExecutor<ExtensionToolCall>>> {
        vec![Arc::new(ExtensionEchoExecutor)]
    }
}

struct ExtensionEchoExecutor;

#[async_trait::async_trait]
impl ToolExecutor<ExtensionToolCall> for ExtensionEchoExecutor {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced("extension/", "echo")
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(ToolSpec::Namespace(ResponsesApiNamespace {
            name: "extension/".to_string(),
            description: default_namespace_description("extension/"),
            tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
                name: "echo".to_string(),
                description: "Echoes arguments through an extension tool.".to_string(),
                strict: true,
                parameters: codex_extension_api::parse_tool_input_schema(&json!({
                    "type": "object",
                    "properties": {
                        "message": { "type": "string" },
                    },
                    "required": ["message"],
                    "additionalProperties": false,
                }))
                .expect("extension schema should parse"),
                output_schema: None,
                defer_loading: None,
            })],
        }))
    }

    async fn handle(
        &self,
        call: ExtensionToolCall,
    ) -> Result<Box<dyn codex_tools::ToolOutput>, codex_tools::FunctionCallError> {
        let arguments: serde_json::Value =
            serde_json::from_str(call.function_arguments()?).expect("test arguments should parse");
        Ok(Box::new(codex_tools::JsonToolOutput::new(json!({
            "arguments": arguments,
            "callId": call.call_id,
            "ok": true,
        }))))
    }
}

fn extension_tool_test_registry() -> Arc<ExtensionRegistry<Config>> {
    let mut builder = ExtensionRegistryBuilder::new();
    builder.tool_contributor(Arc::new(ExtensionEchoContributor));
    Arc::new(builder.build())
}

#[tokio::test]
#[expect(
    clippy::await_holding_invalid_type,
    reason = "test builds a router from session-owned MCP manager state"
)]
async fn parallel_support_does_not_match_namespaced_local_tool_names() -> anyhow::Result<()> {
    let (session, turn) = make_session_and_context().await;
    let mcp_tools = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .list_all_tools()
        .await;
    let router = ToolRouter::from_config(
        &turn.tools_config,
        ToolRouterParams {
            deferred_mcp_tools: None,
            mcp_tools: Some(mcp_tools),
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: turn.dynamic_tools.as_slice(),
        },
    );

    let parallel_tool_name = ["exec_command", "shell_command"]
        .into_iter()
        .find(|name| {
            router.tool_supports_parallel(&ToolCall {
                tool_name: ToolName::plain(*name),
                call_id: "call-parallel-tool".to_string(),
                payload: ToolPayload::Function {
                    arguments: "{}".to_string(),
                },
            })
        })
        .expect("test session should expose a parallel shell-like tool");

    assert!(!router.tool_supports_parallel(&ToolCall {
        tool_name: ToolName::namespaced("mcp__server__", parallel_tool_name),
        call_id: "call-namespaced-tool".to_string(),
        payload: ToolPayload::Function {
            arguments: "{}".to_string(),
        },
    }));

    Ok(())
}

#[tokio::test]
async fn build_tool_call_uses_namespace_for_registry_name() -> anyhow::Result<()> {
    let tool_name = "create_event".to_string();

    let call = ToolRouter::build_tool_call(ResponseItem::FunctionCall {
        id: None,
        name: tool_name.clone(),
        namespace: Some("mcp__codex_apps__calendar".to_string()),
        arguments: "{}".to_string(),
        encrypted_function_args: None,
        call_id: "call-namespace".to_string(),
    })?
    .expect("function_call should produce a tool call");

    assert_eq!(
        call.tool_name,
        ToolName::namespaced("mcp__codex_apps__calendar", tool_name)
    );
    assert_eq!(call.call_id, "call-namespace");
    match call.payload {
        ToolPayload::Function { arguments } => {
            assert_eq!(arguments, "{}");
        }
        other => panic!("expected function payload, got {other:?}"),
    }

    Ok(())
}

#[test]
fn plaintext_collaboration_marker_is_narrow_and_fail_closed() {
    let plaintext = ResponseItem::FunctionCall {
        id: None,
        name: "send_message".to_string(),
        namespace: Some("collaboration".to_string()),
        arguments: r#"{"target":"/root/worker","message":"secret"}"#.to_string(),
        encrypted_function_args: Some(Vec::new()),
        call_id: "call-plaintext".to_string(),
    };
    assert_eq!(
        crate::tools::router::direct_source_for_response_item(&plaintext),
        ToolCallSource::DirectPlaintextMessage
    );

    let wrong_namespace = ResponseItem::FunctionCall {
        id: None,
        name: "send_message".to_string(),
        namespace: None,
        arguments: "{}".to_string(),
        encrypted_function_args: Some(Vec::new()),
        call_id: "call-wrong-namespace".to_string(),
    };
    assert_eq!(
        crate::tools::router::direct_source_for_response_item(&wrong_namespace),
        ToolCallSource::Direct
    );

    let wrong_tool = ResponseItem::FunctionCall {
        id: None,
        name: "close_agent".to_string(),
        namespace: Some("collaboration".to_string()),
        arguments: "{}".to_string(),
        encrypted_function_args: Some(Vec::new()),
        call_id: "call-wrong-tool".to_string(),
    };
    assert_eq!(
        crate::tools::router::direct_source_for_response_item(&wrong_tool),
        ToolCallSource::Direct
    );

    let encrypted = ResponseItem::FunctionCall {
        id: None,
        name: "send_message".to_string(),
        namespace: Some("collaboration".to_string()),
        arguments: "{}".to_string(),
        encrypted_function_args: Some(vec!["opaque".to_string()]),
        call_id: "call-encrypted".to_string(),
    };
    assert_eq!(
        crate::tools::router::direct_source_for_response_item(&encrypted),
        ToolCallSource::Direct
    );

    let payload = ToolPayload::Function {
        arguments: r#"{"message":"secret"}"#.to_string(),
    };
    assert_eq!(
        crate::tools::router::tool_log_payload(&payload, &ToolCallSource::DirectPlaintextMessage),
        "[plaintext arguments]"
    );
    assert_eq!(
        crate::tools::router::tool_log_payload(&payload, &ToolCallSource::Direct),
        r#"{"message":"secret"}"#
    );
}

#[tokio::test]
async fn mcp_parallel_support_uses_handler_data() -> anyhow::Result<()> {
    let (_, turn) = make_session_and_context().await;
    let router = ToolRouter::from_config(
        &turn.tools_config,
        ToolRouterParams {
            deferred_mcp_tools: None,
            mcp_tools: Some(vec![
                mcp_tool_info(
                    "echo",
                    /*supports_parallel_tool_calls*/ true,
                    "mcp__echo__",
                    "query_with_delay",
                ),
                mcp_tool_info(
                    "hello_echo",
                    /*supports_parallel_tool_calls*/ false,
                    "mcp__hello_echo__",
                    "query_with_delay",
                ),
            ]),
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: turn.dynamic_tools.as_slice(),
        },
    );

    let call = ToolCall {
        tool_name: ToolName::namespaced("mcp__echo__", "query_with_delay"),
        call_id: "call-handler".to_string(),
        payload: ToolPayload::Function {
            arguments: "{}".to_string(),
        },
    };
    assert!(router.tool_supports_parallel(&call));

    let different_server_call = ToolCall {
        tool_name: ToolName::namespaced("mcp__hello_echo__", "query_with_delay"),
        call_id: "call-other-server".to_string(),
        payload: ToolPayload::Function {
            arguments: "{}".to_string(),
        },
    };
    assert!(!router.tool_supports_parallel(&different_server_call));

    Ok(())
}

#[tokio::test]
async fn tools_without_handlers_do_not_support_parallel() -> anyhow::Result<()> {
    let (_, turn) = make_session_and_context().await;
    let router = ToolRouter::from_config(
        &turn.tools_config,
        ToolRouterParams {
            deferred_mcp_tools: None,
            mcp_tools: None,
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: turn.dynamic_tools.as_slice(),
        },
    );

    assert!(!router.tool_supports_parallel(&ToolCall {
        tool_name: ToolName::plain("web_search"),
        call_id: "call-web-search".to_string(),
        payload: ToolPayload::Function {
            arguments: "{}".to_string(),
        },
    }));

    Ok(())
}

#[tokio::test]
async fn specs_filter_deferred_dynamic_tools() -> anyhow::Result<()> {
    let (_, turn) = make_session_and_context().await;
    let hidden_tool = "hidden_dynamic_tool";
    let visible_tool = "visible_dynamic_tool";
    let dynamic_tools = vec![
        DynamicToolSpec {
            namespace: Some("codex_app".to_string()),
            name: hidden_tool.to_string(),
            description: "Hidden until discovered.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false,
            }),
            defer_loading: true,
        },
        DynamicToolSpec {
            namespace: Some("codex_app".to_string()),
            name: visible_tool.to_string(),
            description: "Visible immediately.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false,
            }),
            defer_loading: false,
        },
    ];

    let router = ToolRouter::from_config(
        &turn.tools_config,
        ToolRouterParams {
            deferred_mcp_tools: None,
            mcp_tools: None,
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: &dynamic_tools,
        },
    );

    assert_eq!(
        namespace_function_names(&router.model_visible_specs(), "codex_app"),
        vec![visible_tool.to_string()]
    );

    Ok(())
}

fn mcp_tool_info(
    server_name: &str,
    supports_parallel_tool_calls: bool,
    callable_namespace: &str,
    tool_name: &str,
) -> codex_mcp::ToolInfo {
    codex_mcp::ToolInfo {
        server_name: server_name.to_string(),
        supports_parallel_tool_calls,
        server_origin: None,
        callable_name: tool_name.to_string(),
        callable_namespace: callable_namespace.to_string(),
        namespace_description: None,
        tool: rmcp::model::Tool::new(
            tool_name.to_string(),
            "Test MCP tool",
            Arc::new(rmcp::model::object(json!({
                "type": "object",
            }))),
        ),
        connector_id: None,
        connector_name: None,
        plugin_display_names: Vec::new(),
    }
}

#[test]
fn mcp_binding_derives_tool_names_and_resource_helpers_from_one_catalog() {
    let direct = mcp_tool_info("echo", false, "mcp__echo__", "query");
    let deferred = mcp_tool_info("search", true, "mcp__search__", "lookup");
    let direct_name = direct.canonical_tool_name();
    let deferred_name = deferred.canonical_tool_name();
    let binding = codex_mcp::McpBinding::from_tools(
        7,
        std::slice::from_ref(&direct),
        std::slice::from_ref(&deferred),
        /*has_servers*/ true,
    );

    assert_eq!(binding.generation(), 7);
    assert!(binding.has_servers());
    assert_eq!(
        binding
            .tools()
            .iter()
            .map(codex_mcp::ToolInfo::canonical_tool_name)
            .collect::<Vec<_>>(),
        vec![direct_name.clone(), deferred_name.clone()]
    );
    assert!(binding.contains(&direct_name));
    assert!(binding.contains(&deferred_name));
    assert!(binding.contains(&ToolName::plain("list_mcp_resources")));
    assert!(binding.contains(&ToolName::plain("list_mcp_resource_templates")));
    assert!(binding.contains(&ToolName::plain("read_mcp_resource")));
}

#[tokio::test]
async fn advertised_mcp_tool_fails_closed_after_manager_generation_changes() {
    let (session, turn) = make_session_and_context().await;
    let tool = mcp_tool_info(
        "echo",
        /*supports_parallel_tool_calls*/ false,
        "mcp__echo__",
        "query",
    );
    let tool_name = tool.canonical_tool_name();
    let advertised_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();
    let binding = Arc::new(codex_mcp::McpBinding::from_tools(
        advertised_generation,
        std::slice::from_ref(&tool),
        &[],
        /*has_servers*/ true,
    ));
    let router = ToolRouter::from_config(
        &turn.tools_config,
        ToolRouterParams {
            deferred_mcp_tools: None,
            mcp_tools: Some(vec![tool]),
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: turn.dynamic_tools.as_slice(),
        },
    )
    .bind_mcp(binding);

    let replacement = codex_mcp::McpConnectionManager::new_uninitialized_with_permission_profile(
        &turn.approval_policy,
        &turn.permission_profile(),
    );
    let auth_binding =
        crate::state::FrozenMcpAuthSnapshot::capture(session.services.auth_manager.as_ref())
            .await
            .expect("capture MCP auth snapshot")
            .binding();
    let mut old_manager = session
        .services
        .mcp_connection_manager
        .write()
        .await
        .publish(
            replacement,
            auth_binding,
            codex_protocol::protocol::McpElicitationAuthority {
                approval_policy: turn.approval_policy.value(),
                permission_profile: turn.permission_profile(),
                approvals_reviewer: turn.config.approvals_reviewer,
                apps_approvals_reviewers: Default::default(),
            },
        );
    old_manager.shutdown().await;

    let result = router
        .dispatch_tool_call_with_code_mode_result(
            Arc::new(session),
            Arc::new(turn),
            CancellationToken::new(),
            Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new())),
            ToolCall {
                tool_name,
                call_id: "call-stale-generation".to_string(),
                payload: ToolPayload::Function {
                    arguments: "{}".to_string(),
                },
            },
            ToolCallSource::Direct,
        )
        .await;
    assert!(matches!(
        result,
        Err(FunctionCallError::RespondToModel(message))
            if message.contains("catalog generation is stale")
    ));
}

#[tokio::test]
async fn advertised_mcp_tool_fails_closed_after_config_source_generation_changes() {
    let (session, turn) = make_session_and_context().await;
    let tool = mcp_tool_info(
        "echo",
        /*supports_parallel_tool_calls*/ false,
        "mcp__echo__",
        "query",
    );
    let tool_name = tool.canonical_tool_name();
    let advertised_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();
    let binding = Arc::new(codex_mcp::McpBinding::from_tools(
        advertised_generation,
        std::slice::from_ref(&tool),
        &[],
        /*has_servers*/ true,
    ));
    let router = ToolRouter::from_config(
        &turn.tools_config,
        ToolRouterParams {
            deferred_mcp_tools: None,
            mcp_tools: Some(vec![tool]),
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: turn.dynamic_tools.as_slice(),
        },
    )
    .bind_mcp(binding);
    let source = turn.config.config_generation().source();
    source.publish(turn.config.config_generation().value().saturating_add(1));

    let result = router
        .dispatch_tool_call_with_code_mode_result(
            Arc::new(session),
            Arc::new(turn),
            CancellationToken::new(),
            Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new())),
            ToolCall {
                tool_name,
                call_id: "call-stale-source-generation".to_string(),
                payload: ToolPayload::Function {
                    arguments: "{}".to_string(),
                },
            },
            ToolCallSource::Direct,
        )
        .await;
    assert!(matches!(
        result,
        Err(FunctionCallError::RespondToModel(message))
            if message.contains("source generation is stale")
    ));
}

#[tokio::test]
async fn extension_tool_executors_are_model_visible_and_dispatchable() -> anyhow::Result<()> {
    let (mut session, turn) = make_session_and_context().await;
    session.services.extensions = extension_tool_test_registry();

    let router = ToolRouter::from_config(
        &turn.tools_config,
        ToolRouterParams {
            deferred_mcp_tools: None,
            mcp_tools: None,
            discoverable_tools: None,
            extension_tool_executors: extension_tool_executors(&session, &turn.extension_data),
            dynamic_tools: turn.dynamic_tools.as_slice(),
        },
    );

    assert!(
        router.model_visible_specs().iter().any(
            |spec| matches!(spec, ToolSpec::Namespace(namespace)
            if namespace.name == "extension/"
                && namespace.tools.iter().any(|tool| matches!(
                    tool,
                    ResponsesApiNamespaceTool::Function(tool) if tool.name == "echo"
                )))
        ),
        "expected extension-provided tool to be visible to the model"
    );

    let call = ToolRouter::build_tool_call(ResponseItem::FunctionCall {
        id: None,
        name: "echo".to_string(),
        namespace: Some("extension/".to_string()),
        arguments: json!({ "message": "hello" }).to_string(),
        encrypted_function_args: None,
        call_id: "call-extension".to_string(),
    })?
    .expect("function_call should produce a tool call");
    let result = router
        .dispatch_tool_call_with_code_mode_result(
            Arc::new(session),
            Arc::new(turn),
            CancellationToken::new(),
            Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new())),
            call,
            ToolCallSource::Direct,
        )
        .await?;

    let response = result.into_response();
    match response {
        ResponseInputItem::FunctionCallOutput { call_id, output } => {
            assert_eq!(call_id, "call-extension");
            let FunctionCallOutputBody::Text(text) = output.body else {
                panic!("expected text function call output")
            };
            let value: serde_json::Value =
                serde_json::from_str(&text).expect("extension tool output should be json");
            assert_eq!(
                value,
                json!({
                    "arguments": { "message": "hello" },
                    "callId": "call-extension",
                    "ok": true,
                })
            );
        }
        other => panic!("expected function call output, got {other:?}"),
    }

    Ok(())
}

fn namespace_function_names(specs: &[ToolSpec], namespace_name: &str) -> Vec<String> {
    specs
        .iter()
        .find_map(|spec| match spec {
            ToolSpec::Namespace(namespace) if namespace.name == namespace_name => Some(
                namespace
                    .tools
                    .iter()
                    .map(|tool| match tool {
                        ResponsesApiNamespaceTool::Function(tool) => tool.name.clone(),
                    })
                    .collect(),
            ),
            ToolSpec::Function(_)
            | ToolSpec::Freeform(_)
            | ToolSpec::ToolSearch { .. }
            | ToolSpec::ImageGeneration { .. }
            | ToolSpec::WebSearch { .. }
            | ToolSpec::Namespace(_) => None,
        })
        .unwrap_or_default()
}
