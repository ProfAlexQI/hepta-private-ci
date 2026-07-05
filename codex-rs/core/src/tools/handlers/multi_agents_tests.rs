use super::*;
use crate::ThreadManager;
use crate::config::AgentCardManifestConfig;
use crate::config::AgentRoleConfig;
use crate::config::DEFAULT_AGENT_MAX_DEPTH;
use crate::function_tool::FunctionCallError;
use crate::init_state_db;
use crate::session::tests::make_session_and_context;
use crate::session_prefix::format_subagent_notification_message;
use crate::thread_manager::thread_store_from_config;
use crate::tools::context::ToolOutput;
use crate::tools::handlers::multi_agents_v2::CloseAgentHandler as CloseAgentHandlerV2;
use crate::tools::handlers::multi_agents_v2::FollowupTaskHandler as FollowupTaskHandlerV2;
use crate::tools::handlers::multi_agents_v2::ListAgentsHandler as ListAgentsHandlerV2;
use crate::tools::handlers::multi_agents_v2::SendMessageHandler as SendMessageHandlerV2;
use crate::tools::handlers::multi_agents_v2::SpawnAgentHandler as SpawnAgentHandlerV2;
use crate::tools::handlers::multi_agents_v2::WaitAgentHandler as WaitAgentHandlerV2;
use crate::turn_diff_tracker::TurnDiffTracker;
use codex_extension_api::empty_extension_registry;
use codex_features::Feature;
use codex_login::AuthManager;
use codex_login::CodexAuth;
use codex_model_provider::create_model_provider;
use codex_model_provider_info::built_in_model_providers;
use codex_protocol::AgentPath;
use codex_protocol::ThreadId;
use codex_protocol::config_types::ServiceTier;
use codex_protocol::config_types::ShellEnvironmentPolicy;
use codex_protocol::models::BaseInstructions;
use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputBody;
use codex_protocol::models::PermissionProfile;
use codex_protocol::models::ResponseInputItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::models::SandboxEnforcement;
use codex_protocol::openai_models::ReasoningEffort;
use codex_protocol::protocol::AgentStatus;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::FileSystemAccessMode;
use codex_protocol::protocol::FileSystemPath;
use codex_protocol::protocol::FileSystemSandboxEntry;
use codex_protocol::protocol::FileSystemSandboxPolicy;
use codex_protocol::protocol::InitialHistory;
use codex_protocol::protocol::InterAgentCommunication;
use codex_protocol::protocol::NetworkSandboxPolicy;
use codex_protocol::protocol::Op;
use codex_protocol::protocol::RolloutItem;
use codex_protocol::protocol::SandboxPolicy;
use codex_protocol::protocol::SessionSource;
use codex_protocol::protocol::SubAgentSource;
use codex_protocol::protocol::TurnAbortReason;
use codex_protocol::protocol::TurnAbortedEvent;
use codex_protocol::protocol::TurnCompleteEvent;
use codex_protocol::user_input::UserInput;
use core_test_support::TempDirExt;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

fn run_large_stack_async_test<F>(name: &'static str, future: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    const TEST_STACK_SIZE_BYTES: usize = 8 * 1024 * 1024;

    let handle = std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(TEST_STACK_SIZE_BYTES)
        .spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_stack_size(TEST_STACK_SIZE_BYTES)
                .enable_all()
                .build()
                .expect("large-stack runtime should build");
            runtime.block_on(future);
        })
        .expect("large-stack test thread should spawn");

    handle.join().expect("large-stack test thread panicked");
}

fn invocation(
    session: Arc<crate::session::session::Session>,
    turn: Arc<TurnContext>,
    tool_name: &str,
    payload: ToolPayload,
) -> ToolInvocation {
    ToolInvocation {
        session,
        turn,
        cancellation_token: CancellationToken::new(),
        tracker: Arc::new(Mutex::new(TurnDiffTracker::default())),
        call_id: "call-1".to_string(),
        tool_name: codex_tools::ToolName::plain(tool_name),
        source: crate::tools::context::ToolCallSource::Direct,
        payload,
    }
}

fn function_payload(args: serde_json::Value) -> ToolPayload {
    ToolPayload::Function {
        arguments: args.to_string(),
    }
}

fn parse_agent_id(id: &str) -> ThreadId {
    ThreadId::from_string(id).expect("agent id should be valid")
}

fn thread_manager() -> ThreadManager {
    ThreadManager::with_models_provider_for_tests(
        CodexAuth::from_api_key("dummy"),
        built_in_model_providers(/* openai_base_url */ /*openai_base_url*/ None)["openai"].clone(),
    )
}

async fn install_role_with_model_override(turn: &mut TurnContext) -> String {
    let role_name = "fork-context-role".to_string();
    tokio::fs::create_dir_all(&turn.config.codex_home)
        .await
        .expect("codex home should be created");
    let role_config_path = turn
        .config
        .codex_home
        .as_path()
        .join("fork-context-role.toml");
    tokio::fs::write(
        &role_config_path,
        r#"model = "gpt-5-role-override"
model_provider = "ollama"
model_reasoning_effort = "minimal"
"#,
    )
    .await
    .expect("role config should be written");

    let mut config = (*turn.config).clone();
    config.agent_roles.insert(
        role_name.clone(),
        AgentRoleConfig {
            description: Some("Role with model overrides".to_string()),
            config_file: Some(role_config_path),
            agent_card_manifest_source: None,
            agent_card_manifest_version: None,
            agent_card_manifest: None,
            nickname_candidates: None,
        },
    );
    turn.config = Arc::new(config);

    role_name
}

fn expect_text_output<T>(output: T) -> (String, Option<bool>)
where
    T: ToolOutput,
{
    let response = output.to_response_item(
        "call-1",
        &ToolPayload::Function {
            arguments: "{}".to_string(),
        },
    );
    match response {
        ResponseInputItem::FunctionCallOutput { output, .. }
        | ResponseInputItem::CustomToolCallOutput { output, .. } => {
            let content = match output.body {
                FunctionCallOutputBody::Text(text) => text,
                FunctionCallOutputBody::ContentItems(items) => {
                    codex_protocol::models::function_call_output_content_items_to_text(&items)
                        .unwrap_or_default()
                }
            };
            (content, output.success)
        }
        other => panic!("expected function output, got {other:?}"),
    }
}

#[derive(Debug, Deserialize)]
struct ListAgentsResult {
    agents: Vec<ListedAgentResult>,
}

#[derive(Debug, Deserialize)]
struct ListedAgentResult {
    agent_name: String,
    agent_status: serde_json::Value,
    last_task_message: Option<String>,
}

#[tokio::test]
async fn handler_rejects_non_function_payloads() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        ToolPayload::Custom {
            input: "hello".to_string(),
        },
    );
    let Err(err) = SpawnAgentHandler::default().handle(invocation).await else {
        panic!("payload should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "collab handler received unsupported payload".to_string()
        )
    );
}

#[tokio::test]
async fn spawn_agent_rejects_empty_message() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({"message": "   "})),
    );
    let Err(err) = SpawnAgentHandler::default().handle(invocation).await else {
        panic!("empty message should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("Empty message can't be sent to an agent".to_string())
    );
}

#[tokio::test]
async fn spawn_agent_rejects_when_message_and_items_are_both_set() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({
            "message": "hello",
            "items": [{"type": "mention", "name": "drive", "path": "app://drive"}]
        })),
    );
    let Err(err) = SpawnAgentHandler::default().handle(invocation).await else {
        panic!("message+items should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "Provide either message or items, but not both".to_string()
        )
    );
}

#[tokio::test]
async fn spawn_agent_uses_explorer_role_and_preserves_approval_policy() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        agent_id: String,
        nickname: Option<String>,
    }

    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let mut config = (*turn.config).clone();
    let provider_info =
        built_in_model_providers(/* openai_base_url */ /*openai_base_url*/ None)["ollama"].clone();
    config.model_provider_id = "ollama".to_string();
    config.model_provider = provider_info.clone();
    config
        .permissions
        .approval_policy
        .set(AskForApproval::OnRequest)
        .expect("approval policy should be set");
    turn.approval_policy
        .set(AskForApproval::OnRequest)
        .expect("approval policy should be set");
    turn.provider = create_model_provider(provider_info, turn.auth_manager.clone());
    turn.config = Arc::new(config);

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({
            "message": "inspect this repo",
            "agent_type": "explorer"
        })),
    );
    let output = SpawnAgentHandler::default()
        .handle(invocation)
        .await
        .expect("spawn_agent should succeed");
    let (content, _) = expect_text_output(output);
    let result: SpawnAgentResult =
        serde_json::from_str(&content).expect("spawn_agent result should be json");
    let agent_id = parse_agent_id(&result.agent_id);
    assert!(
        result
            .nickname
            .as_deref()
            .is_some_and(|nickname| !nickname.is_empty())
    );
    let snapshot = manager
        .get_thread(agent_id)
        .await
        .expect("spawned agent thread should exist")
        .config_snapshot()
        .await;
    assert_eq!(snapshot.approval_policy, AskForApproval::OnRequest);
    assert_eq!(snapshot.model_provider_id, "ollama");
}

#[tokio::test]
async fn spawn_agent_fork_context_rejects_agent_type_override() {
    let (mut session, mut turn) = make_session_and_context().await;
    let role_name = install_role_with_model_override(&mut turn).await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let err = SpawnAgentHandler::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "agent_type": role_name,
                "fork_context": true
            })),
        ))
        .await
        .err()
        .expect("fork_context should reject agent_type overrides");

    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "Full-history forked agents inherit the parent agent type, model, and reasoning effort; omit agent_type, model, and reasoning_effort, or spawn without a full-history fork.".to_string(),
        )
    );
}

#[tokio::test]
async fn spawn_agent_fork_context_rejects_child_model_overrides() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;

    let err = SpawnAgentHandler::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "model": "gpt-5-child-override",
                "reasoning_effort": "low",
                "fork_context": true
            })),
        ))
        .await
        .err()
        .expect("forked spawn should reject child model overrides");

    assert_eq!(
        err,
            FunctionCallError::RespondToModel(
            "Full-history forked agents inherit the parent agent type, model, and reasoning effort; omit agent_type, model, and reasoning_effort, or spawn without a full-history fork.".to_string(),
        )
    );
}

#[test]
fn multi_agent_v2_spawn_fork_turns_all_rejects_agent_type_override() {
    run_large_stack_async_test(
        "multi_agent_v2_spawn_fork_turns_all_rejects_agent_type_override",
        multi_agent_v2_spawn_fork_turns_all_rejects_agent_type_override_impl(),
    );
}

async fn multi_agent_v2_spawn_fork_turns_all_rejects_agent_type_override_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let role_name = install_role_with_model_override(&mut turn).await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    let turn = TurnContext {
        config: Arc::new(config),
        ..turn
    };

    let err = SpawnAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "fork_context_v2",
                "agent_type": role_name,
                "fork_turns": "all"
            })),
        ))
        .await
        .err()
        .expect("fork_turns=all should reject agent_type overrides");

    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "Full-history forked agents inherit the parent agent type, model, and reasoning effort; omit agent_type, model, and reasoning_effort, or spawn without a full-history fork.".to_string(),
        )
    );
}

#[test]
fn multi_agent_v2_spawn_defaults_to_full_fork_and_rejects_child_model_overrides() {
    run_large_stack_async_test(
        "multi_agent_v2_spawn_defaults_to_full_fork_and_rejects_child_model_overrides",
        multi_agent_v2_spawn_defaults_to_full_fork_and_rejects_child_model_overrides_impl(),
    );
}

async fn multi_agent_v2_spawn_defaults_to_full_fork_and_rejects_child_model_overrides_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let err = SpawnAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "fork_context_v2",
                "model": "gpt-5-child-override",
                "reasoning_effort": "low"
            })),
        ))
        .await
        .err()
        .expect("default full fork should reject child model overrides");

    assert_eq!(
        err,
            FunctionCallError::RespondToModel(
            "Full-history forked agents inherit the parent agent type, model, and reasoning effort; omit agent_type, model, and reasoning_effort, or spawn without a full-history fork.".to_string(),
        )
    );
}

#[test]
fn spawn_agent_service_tier_override_validates_the_effective_child_model() {
    run_large_stack_async_test(
        "spawn_agent_service_tier_override_validates_the_effective_child_model",
        spawn_agent_service_tier_override_validates_the_effective_child_model_impl(),
    );
}

async fn spawn_agent_service_tier_override_validates_the_effective_child_model_impl() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        agent_id: String,
    }

    {
        let (mut session, turn) = make_session_and_context().await;
        let manager = thread_manager();
        let root = manager
            .start_thread((*turn.config).clone())
            .await
            .expect("root thread should start");
        session.services.agent_control = manager.agent_control();
        session.conversation_id = root.thread_id;

        let output = SpawnAgentHandler::default()
            .handle(invocation(
                Arc::new(session),
                Arc::new(turn),
                "spawn_agent",
                function_payload(json!({
                    "message": "inspect this repo",
                    "model": "gpt-5.4",
                    "service_tier": ServiceTier::Fast.request_value()
                })),
            ))
            .await
            .expect("spawn_agent should accept a supported explicit service tier");
        let (content, _) = expect_text_output(output);
        let result: SpawnAgentResult =
            serde_json::from_str(&content).expect("spawn_agent result should be json");
        let snapshot = manager
            .get_thread(parse_agent_id(&result.agent_id))
            .await
            .expect("spawned agent thread should exist")
            .config_snapshot()
            .await;

        assert_eq!(
            snapshot.service_tier,
            Some(ServiceTier::Fast.request_value().to_string())
        );
    }

    {
        let (session, turn) = make_session_and_context().await;
        let err = SpawnAgentHandler::default()
            .handle(invocation(
                Arc::new(session),
                Arc::new(turn),
                "spawn_agent",
                function_payload(json!({
                    "message": "inspect this repo",
                    "model": "gpt-5.4",
                    "service_tier": "turbo"
                })),
            ))
            .await
            .err()
            .expect("unknown service tier should be rejected");

        assert_eq!(
            err,
            FunctionCallError::RespondToModel(
                "Service tier `turbo` is not supported for model `gpt-5.4`. Supported service tiers: priority"
                    .to_string()
            )
        );
    }

    {
        let (session, turn) = make_session_and_context().await;
        let err = SpawnAgentHandler::default()
            .handle(invocation(
                Arc::new(session),
                Arc::new(turn),
                "spawn_agent",
                function_payload(json!({
                    "message": "inspect this repo",
                    "model": "gpt-5.3-codex",
                    "service_tier": ServiceTier::Fast.request_value()
                })),
            ))
            .await
            .err()
            .expect("tier unsupported by the final child model should be rejected");

        assert_eq!(
            err,
            FunctionCallError::RespondToModel(
                "Service tier `priority` is not supported for model `gpt-5.3-codex`. Supported service tiers: none"
                    .to_string()
            )
        );
    }
}

#[test]
fn spawn_agent_service_tier_inheritance_preserves_supported_or_configured_tiers() {
    run_large_stack_async_test(
        "spawn_agent_service_tier_inheritance_preserves_supported_or_configured_tiers",
        spawn_agent_service_tier_inheritance_preserves_supported_or_configured_tiers_impl(),
    );
}

async fn spawn_agent_service_tier_inheritance_preserves_supported_or_configured_tiers_impl() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        agent_id: String,
    }

    {
        let (mut session, turn) = make_session_and_context().await;
        let mut turn = turn
            .with_model("gpt-5.4".to_string(), &session.services.models_manager)
            .await;
        let mut config = (*turn.config).clone();
        config.service_tier = Some(ServiceTier::Fast.request_value().to_string());
        turn.config = Arc::new(config);
        let manager = thread_manager();
        let root = manager
            .start_thread((*turn.config).clone())
            .await
            .expect("root thread should start");
        session.services.agent_control = manager.agent_control();
        session.conversation_id = root.thread_id;

        let output = SpawnAgentHandler::default()
            .handle(invocation(
                Arc::new(session),
                Arc::new(turn),
                "spawn_agent",
                function_payload(json!({"message": "inspect this repo"})),
            ))
            .await
            .expect("spawn_agent should inherit a supported parent service tier");
        let (content, _) = expect_text_output(output);
        let result: SpawnAgentResult =
            serde_json::from_str(&content).expect("spawn_agent result should be json");
        let snapshot = manager
            .get_thread(parse_agent_id(&result.agent_id))
            .await
            .expect("spawned agent thread should exist")
            .config_snapshot()
            .await;

        assert_eq!(
            snapshot.service_tier,
            Some(ServiceTier::Fast.request_value().to_string())
        );
    }

    {
        let (mut session, turn) = make_session_and_context().await;
        let mut turn = turn
            .with_model("gpt-5.4".to_string(), &session.services.models_manager)
            .await;
        let mut config = (*turn.config).clone();
        config.service_tier = Some(ServiceTier::Fast.request_value().to_string());
        turn.config = Arc::new(config);
        let manager = thread_manager();
        let root = manager
            .start_thread((*turn.config).clone())
            .await
            .expect("root thread should start");
        session.services.agent_control = manager.agent_control();
        session.conversation_id = root.thread_id;

        let output = SpawnAgentHandler::default()
            .handle(invocation(
                Arc::new(session),
                Arc::new(turn),
                "spawn_agent",
                function_payload(json!({
                    "message": "inspect this repo",
                    "model": "gpt-5.3-codex"
                })),
            ))
            .await
            .expect("spawn_agent should clear unsupported inherited service tier");
        let (content, _) = expect_text_output(output);
        let result: SpawnAgentResult =
            serde_json::from_str(&content).expect("spawn_agent result should be json");
        let snapshot = manager
            .get_thread(parse_agent_id(&result.agent_id))
            .await
            .expect("spawned agent thread should exist")
            .config_snapshot()
            .await;

        assert_eq!(snapshot.service_tier, None);
    }

    {
        let (mut session, mut turn) = make_session_and_context().await;
        tokio::fs::create_dir_all(&turn.config.codex_home)
            .await
            .expect("codex home should be created");
        let role_config_path = turn
            .config
            .codex_home
            .as_path()
            .join("service-tier-role.toml");
        tokio::fs::write(
            &role_config_path,
            r#"model = "gpt-5.4"
service_tier = "priority"
"#,
        )
        .await
        .expect("role config should be written");

        let role_name = "service-tier-role".to_string();
        let mut config = (*turn.config).clone();
        config.agent_roles.insert(
            role_name.clone(),
            AgentRoleConfig {
                description: Some("Role with a child service tier".to_string()),
                config_file: Some(role_config_path),
                agent_card_manifest_source: None,
                agent_card_manifest_version: None,
                agent_card_manifest: None,
                nickname_candidates: None,
            },
        );
        turn.config = Arc::new(config);
        let manager = thread_manager();
        let root = manager
            .start_thread((*turn.config).clone())
            .await
            .expect("root thread should start");
        session.services.agent_control = manager.agent_control();
        session.conversation_id = root.thread_id;

        let output = SpawnAgentHandler::default()
            .handle(invocation(
                Arc::new(session),
                Arc::new(turn),
                "spawn_agent",
                function_payload(json!({
                    "message": "inspect this repo",
                    "agent_type": role_name
                })),
            ))
            .await
            .expect("spawn_agent should preserve the child role service tier");
        let (content, _) = expect_text_output(output);
        let result: SpawnAgentResult =
            serde_json::from_str(&content).expect("spawn_agent result should be json");
        let snapshot = manager
            .get_thread(parse_agent_id(&result.agent_id))
            .await
            .expect("spawned agent thread should exist")
            .config_snapshot()
            .await;

        assert_eq!(
            snapshot.service_tier,
            Some(ServiceTier::Fast.request_value().to_string())
        );
    }
}

#[test]
fn spawn_agent_full_history_fork_accepts_explicit_service_tier() {
    run_large_stack_async_test(
        "spawn_agent_full_history_fork_accepts_explicit_service_tier",
        spawn_agent_full_history_fork_accepts_explicit_service_tier_impl(),
    );
}

async fn spawn_agent_full_history_fork_accepts_explicit_service_tier_impl() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        agent_id: String,
    }

    let (mut session, turn) = make_session_and_context().await;
    let turn = turn
        .with_model("gpt-5.4".to_string(), &session.services.models_manager)
        .await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;

    let output = SpawnAgentHandler::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "fork_context": true,
                "service_tier": ServiceTier::Fast.request_value()
            })),
        ))
        .await
        .expect("full-history fork should accept explicit service tier");
    let (content, _) = expect_text_output(output);
    let result: SpawnAgentResult =
        serde_json::from_str(&content).expect("spawn_agent result should be json");
    let snapshot = manager
        .get_thread(parse_agent_id(&result.agent_id))
        .await
        .expect("spawned agent thread should exist")
        .config_snapshot()
        .await;

    assert_eq!(
        snapshot.service_tier,
        Some(ServiceTier::Fast.request_value().to_string())
    );
}

#[test]
fn multi_agent_v2_full_history_fork_accepts_explicit_service_tier() {
    run_large_stack_async_test(
        "multi_agent_v2_full_history_fork_accepts_explicit_service_tier",
        multi_agent_v2_full_history_fork_accepts_explicit_service_tier_impl(),
    );
}

async fn multi_agent_v2_full_history_fork_accepts_explicit_service_tier_impl() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        task_name: String,
    }

    let (mut session, turn) = make_session_and_context().await;
    let mut turn = turn
        .with_model("gpt-5.4".to_string(), &session.services.models_manager)
        .await;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    let output = SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "fork_with_tier",
                "service_tier": ServiceTier::Fast.request_value()
            })),
        ))
        .await
        .expect("multi-agent v2 full-history fork should accept explicit service tier");
    let (content, _) = expect_text_output(output);
    let result: SpawnAgentResult =
        serde_json::from_str(&content).expect("spawn_agent result should be json");
    let child_thread_id = session
        .services
        .agent_control
        .resolve_agent_reference(
            session.conversation_id,
            &turn.session_source,
            result.task_name.as_str(),
        )
        .await
        .expect("spawned task name should resolve");
    let snapshot = manager
        .get_thread(child_thread_id)
        .await
        .expect("spawned agent thread should exist")
        .config_snapshot()
        .await;

    assert_eq!(
        snapshot.service_tier,
        Some(ServiceTier::Fast.request_value().to_string())
    );
}

#[test]
fn multi_agent_v2_spawn_partial_fork_turns_allows_agent_type_override() {
    run_large_stack_async_test(
        "multi_agent_v2_spawn_partial_fork_turns_allows_agent_type_override",
        multi_agent_v2_spawn_partial_fork_turns_allows_agent_type_override_impl(),
    );
}

async fn multi_agent_v2_spawn_partial_fork_turns_allows_agent_type_override_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let role_name = install_role_with_model_override(&mut turn).await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    let turn = TurnContext {
        config: Arc::new(config),
        ..turn
    };

    let output = SpawnAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "partial_fork",
                "agent_type": role_name,
                "fork_turns": "1"
            })),
        ))
        .await
        .expect("partial fork should allow agent_type overrides");
    let (content, _) = expect_text_output(output);
    let result: serde_json::Value =
        serde_json::from_str(&content).expect("spawn_agent result should be json");
    assert_eq!(result["task_name"], "/root/partial_fork");
    let agent_id = manager
        .captured_ops()
        .into_iter()
        .map(|(thread_id, _)| thread_id)
        .find(|thread_id| *thread_id != root.thread_id)
        .expect("spawned agent should receive an op");
    let snapshot = manager
        .get_thread(agent_id)
        .await
        .expect("spawned agent thread should exist")
        .config_snapshot()
        .await;

    assert_eq!(snapshot.model, "gpt-5-role-override");
    assert_eq!(snapshot.model_provider_id, "ollama");
    assert_eq!(snapshot.reasoning_effort, Some(ReasoningEffort::Minimal));
}

#[tokio::test]
async fn spawn_agent_returns_agent_id_without_task_name() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();

    let output = SpawnAgentHandler::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo"
            })),
        ))
        .await
        .expect("spawn_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: serde_json::Value =
        serde_json::from_str(&content).expect("spawn_agent result should be json");

    assert!(result["agent_id"].is_string());
    assert!(result.get("task_name").is_none());
    assert!(result.get("nickname").is_some());
    assert_eq!(
        result["admission_shadow_decision"]["decision"],
        "deny_shadow_no_live_blocking"
    );
    assert_eq!(
        result["admission_shadow_decision"]["liveBlockingEnabled"],
        false
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["manifestId"],
        "agent-card:spawn_agent:default"
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["decision"],
        "deny_shadow_manifest_no_live_blocking"
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["lane"],
        "subagent"
    );
    assert!(
        result["admission_shadow_decision"]["denialReasons"]
            .as_array()
            .expect("denial reasons should be an array")
            .iter()
            .any(|reason| reason
                .as_str()
                .is_some_and(|reason| reason.contains("result_contract")))
    );
    assert_eq!(success, Some(true));
}

#[tokio::test]
async fn multi_agent_v2_spawn_requires_task_name() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({
            "message": "inspect this repo"
        })),
    );
    let Err(err) = SpawnAgentHandlerV2::default().handle(invocation).await else {
        panic!("missing task_name should be rejected");
    };
    let FunctionCallError::RespondToModel(message) = err else {
        panic!("missing task_name should surface as a model-facing error");
    };
    assert!(message.contains("missing field `task_name`"));
}

#[tokio::test]
async fn multi_agent_v2_spawn_rejects_legacy_items_field() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({
            "message": "inspect this repo",
            "items": [{"type": "text", "text": "inspect this repo"}],
            "task_name": "worker"
        })),
    );
    let Err(err) = SpawnAgentHandlerV2::default().handle(invocation).await else {
        panic!("legacy items field should be rejected");
    };
    let FunctionCallError::RespondToModel(message) = err else {
        panic!("legacy items field should surface as a model-facing error");
    };
    assert!(message.contains("unknown field `items`"));
}

#[tokio::test]
async fn spawn_agent_errors_when_manager_dropped() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({"message": "hello"})),
    );
    let Err(err) = SpawnAgentHandler::default().handle(invocation).await else {
        panic!("spawn should fail without a manager");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("collab manager unavailable".to_string())
    );
}

#[test]
fn multi_agent_v2_spawn_returns_path_and_send_message_accepts_relative_path() {
    run_large_stack_async_test(
        "multi_agent_v2_spawn_returns_path_and_send_message_accepts_relative_path",
        multi_agent_v2_spawn_returns_path_and_send_message_accepts_relative_path_impl(),
    );
}

async fn multi_agent_v2_spawn_returns_path_and_send_message_accepts_relative_path_impl() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        task_name: String,
        nickname: Option<String>,
    }

    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let session = Arc::new(session);
    let turn = Arc::new(turn);
    let spawn_output = SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "test_process"
            })),
        ))
        .await
        .expect("spawn_agent should succeed");
    let (content, _) = expect_text_output(spawn_output);
    let spawn_result: SpawnAgentResult =
        serde_json::from_str(&content).expect("spawn result should parse");
    assert_eq!(spawn_result.task_name, "/root/test_process");
    assert!(spawn_result.nickname.is_some());

    let child_thread_id = session
        .services
        .agent_control
        .resolve_agent_reference(
            session.conversation_id,
            &turn.session_source,
            "test_process",
        )
        .await
        .expect("relative path should resolve");
    let child_snapshot = manager
        .get_thread(child_thread_id)
        .await
        .expect("child thread should exist")
        .config_snapshot()
        .await;
    assert_eq!(
        child_snapshot.session_source.get_agent_path().as_deref(),
        Some("/root/test_process")
    );
    assert!(manager.captured_ops().iter().any(|(id, op)| {
        *id == child_thread_id
            && matches!(
                op,
                Op::InterAgentCommunication { communication }
                    if communication.author == AgentPath::root()
                        && communication.recipient.as_str() == "/root/test_process"
                        && communication.other_recipients.is_empty()
                        && communication.content == "inspect this repo"
                        && communication.trigger_turn
            )
    }));

    let send_output = SendMessageHandlerV2
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "send_message",
            function_payload(json!({
                "target": "test_process",
                "message": "continue"
            })),
        ))
        .await
        .expect("send_message should accept v2 path");
    let (send_content, send_success) = expect_text_output(send_output);
    let send_result: serde_json::Value =
        serde_json::from_str(&send_content).expect("send_message result should be json");
    assert_eq!(
        send_result["workGraphHandoffShadowDecision"]["definitionSource"],
        json!("explicit_agent_card_manifest")
    );
    assert_eq!(
        send_result["workGraphHandoffShadowDecision"]["attemptedTool"],
        json!("send_message")
    );
    assert_eq!(
        send_result["workGraphHandoffShadowDecision"]["toolAllowed"],
        json!(true)
    );
    assert_eq!(
        send_result["workGraphHandoffShadowDecision"]["observedLane"],
        json!("subagent_handoff")
    );
    assert_eq!(
        send_result["workGraphHandoffShadowDecision"]["laneAllowed"],
        json!(true)
    );
    assert_eq!(
        send_result["workGraphHandoffShadowDecision"]["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(send_success, Some(true));

    assert!(manager.captured_ops().iter().any(|(id, op)| {
        *id == child_thread_id
            && matches!(
                op,
                Op::InterAgentCommunication { communication }
                    if communication.author == AgentPath::root()
                        && communication.recipient.as_str() == "/root/test_process"
                        && communication.other_recipients.is_empty()
                        && communication.content == "continue"
                        && !communication.trigger_turn
            )
    }));
}

#[tokio::test]
async fn multi_agent_v2_spawn_rejects_legacy_fork_context() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let err = SpawnAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "worker",
                "fork_context": true
            })),
        ))
        .await
        .err()
        .expect("legacy fork_context should be rejected");

    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "fork_context is not supported in MultiAgentV2; use fork_turns instead".to_string()
        )
    );
}

#[tokio::test]
async fn multi_agent_v2_spawn_rejects_invalid_fork_turns_string() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let err = SpawnAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "worker",
                "fork_turns": "banana"
            })),
        ))
        .await
        .err()
        .expect("invalid fork_turns should be rejected");

    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "fork_turns must be `none`, `all`, or a positive integer string".to_string()
        )
    );
}

#[tokio::test]
async fn multi_agent_v2_spawn_rejects_zero_fork_turns() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let err = SpawnAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "worker",
                "fork_turns": "0"
            })),
        ))
        .await
        .err()
        .expect("zero turn count should be rejected");

    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "fork_turns must be `none`, `all`, or a positive integer string".to_string()
        )
    );
}

#[tokio::test]
async fn multi_agent_v2_send_message_accepts_root_target_from_child() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let child_path = AgentPath::try_from("/root/worker").expect("agent path");
    let child_thread_id = session
        .services
        .agent_control
        .spawn_agent_with_metadata(
            (*turn.config).clone(),
            vec![UserInput::Text {
                text: "inspect this repo".to_string(),
                text_elements: Vec::new(),
            }]
            .into(),
            Some(SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
                parent_thread_id: root.thread_id,
                depth: 1,
                agent_path: Some(child_path.clone()),
                agent_nickname: None,
                agent_role: None,
            })),
            crate::agent::control::SpawnAgentOptions::default(),
        )
        .await
        .expect("worker spawn should succeed")
        .thread_id;
    session.conversation_id = child_thread_id;
    turn.session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: root.thread_id,
        depth: 1,
        agent_path: Some(child_path.clone()),
        agent_nickname: None,
        agent_role: None,
    });

    SendMessageHandlerV2
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "send_message",
            function_payload(json!({
                "target": "/root",
                "message": "done"
            })),
        ))
        .await
        .expect("send_message should accept the root agent path");

    assert!(manager.captured_ops().iter().any(|(id, op)| {
        *id == root.thread_id
            && matches!(
                op,
                Op::InterAgentCommunication { communication }
                    if communication.author == child_path
                        && communication.recipient == AgentPath::root()
                        && communication.other_recipients.is_empty()
                        && communication.content == "done"
                        && !communication.trigger_turn
            )
    }));
}

#[tokio::test]
async fn multi_agent_v2_followup_task_rejects_root_target_from_child() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let child_path = AgentPath::try_from("/root/worker").expect("agent path");
    let child_thread_id = session
        .services
        .agent_control
        .spawn_agent_with_metadata(
            (*turn.config).clone(),
            vec![UserInput::Text {
                text: "inspect this repo".to_string(),
                text_elements: Vec::new(),
            }]
            .into(),
            Some(SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
                parent_thread_id: root.thread_id,
                depth: 1,
                agent_path: Some(child_path.clone()),
                agent_nickname: None,
                agent_role: None,
            })),
            crate::agent::control::SpawnAgentOptions::default(),
        )
        .await
        .expect("worker spawn should succeed")
        .thread_id;
    session.conversation_id = child_thread_id;
    turn.session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: root.thread_id,
        depth: 1,
        agent_path: Some(child_path),
        agent_nickname: None,
        agent_role: None,
    });

    let Err(err) = FollowupTaskHandlerV2
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "followup_task",
            function_payload(json!({
                "target": "/root",
                "message": "run this",
            })),
        ))
        .await
    else {
        panic!("followup_task should reject the root target");
    };

    assert_eq!(
        err,
        FunctionCallError::RespondToModel("Tasks can't be assigned to the root agent".to_string())
    );
    let root_ops = manager
        .captured_ops()
        .into_iter()
        .filter_map(|(id, op)| (id == root.thread_id).then_some(op))
        .collect::<Vec<_>>();
    assert!(!root_ops.iter().any(|op| matches!(op, Op::Interrupt)));
    assert!(
        !root_ops
            .iter()
            .any(|op| matches!(op, Op::InterAgentCommunication { .. }))
    );
}

#[test]
fn multi_agent_v2_list_agents_returns_completed_status_and_last_task_message() {
    run_large_stack_async_test(
        "multi_agent_v2_list_agents_returns_completed_status_and_last_task_message",
        multi_agent_v2_list_agents_returns_completed_status_and_last_task_message_impl(),
    );
}

async fn multi_agent_v2_list_agents_returns_completed_status_and_last_task_message_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    let _ = config.features.enable(Feature::MultiAgentV2);
    turn.config = Arc::new(config);

    let session = Arc::new(session);
    let turn = Arc::new(turn);
    let spawn_output = SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn_agent should succeed");
    let _ = expect_text_output(spawn_output);

    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker path should resolve");
    let child_thread = manager
        .get_thread(agent_id)
        .await
        .expect("child thread should exist");
    let child_turn = child_thread.codex.session.new_default_turn().await;
    child_thread
        .codex
        .session
        .send_event(
            child_turn.as_ref(),
            EventMsg::TurnComplete(TurnCompleteEvent {
                turn_id: child_turn.sub_id.clone(),
                last_agent_message: Some("done".to_string()),
                completed_at: None,
                duration_ms: None,
                time_to_first_token_ms: None,
            }),
        )
        .await;

    let output = ListAgentsHandlerV2
        .handle(invocation(
            session,
            turn,
            "list_agents",
            function_payload(json!({})),
        ))
        .await
        .expect("list_agents should succeed");
    let (content, success) = expect_text_output(output);
    let result: ListAgentsResult =
        serde_json::from_str(&content).expect("list_agents result should be json");

    let agent_names = result
        .agents
        .iter()
        .map(|agent| agent.agent_name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(agent_names, vec!["/root", "/root/worker"]);
    let root_agent = result
        .agents
        .iter()
        .find(|agent| agent.agent_name == "/root")
        .expect("root agent should be listed");
    assert_eq!(root_agent.last_task_message.as_deref(), Some("Main thread"));
    let worker = result
        .agents
        .iter()
        .find(|agent| agent.agent_name == "/root/worker")
        .expect("worker agent should be listed");
    assert_eq!(worker.agent_status, json!({"completed": "done"}));
    assert_eq!(
        worker.last_task_message.as_deref(),
        Some("inspect this repo")
    );
    assert_eq!(success, Some(true));
}

#[test]
fn multi_agent_v2_list_agents_filters_by_relative_path_prefix() {
    run_large_stack_async_test(
        "multi_agent_v2_list_agents_filters_by_relative_path_prefix",
        multi_agent_v2_list_agents_filters_by_relative_path_prefix_impl(),
    );
}

async fn multi_agent_v2_list_agents_filters_by_relative_path_prefix_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    let _ = config.features.enable(Feature::MultiAgentV2);
    turn.config = Arc::new(config.clone());

    let researcher_path = AgentPath::from_string("/root/researcher".to_string()).expect("path");
    let worker_path = AgentPath::from_string("/root/researcher/worker".to_string()).expect("path");
    session
        .services
        .agent_control
        .spawn_agent_with_metadata(
            config.clone(),
            vec![UserInput::Text {
                text: "research".to_string(),
                text_elements: Vec::new(),
            }]
            .into(),
            Some(SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
                parent_thread_id: root.thread_id,
                depth: 1,
                agent_path: Some(researcher_path.clone()),
                agent_nickname: None,
                agent_role: None,
            })),
            crate::agent::control::SpawnAgentOptions::default(),
        )
        .await
        .expect("researcher agent should spawn");
    session
        .services
        .agent_control
        .spawn_agent_with_metadata(
            config,
            vec![UserInput::Text {
                text: "build".to_string(),
                text_elements: Vec::new(),
            }]
            .into(),
            Some(SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
                parent_thread_id: root.thread_id,
                depth: 2,
                agent_path: Some(worker_path.clone()),
                agent_nickname: None,
                agent_role: None,
            })),
            crate::agent::control::SpawnAgentOptions::default(),
        )
        .await
        .expect("worker agent should spawn");

    turn.session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: root.thread_id,
        depth: 1,
        agent_path: Some(researcher_path),
        agent_nickname: None,
        agent_role: None,
    });

    let output = ListAgentsHandlerV2
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "list_agents",
            function_payload(json!({
                "path_prefix": "worker"
            })),
        ))
        .await
        .expect("list_agents should succeed");
    let (content, _) = expect_text_output(output);
    let result: ListAgentsResult =
        serde_json::from_str(&content).expect("list_agents result should be json");

    assert_eq!(result.agents.len(), 1);
    assert_eq!(result.agents[0].agent_name, worker_path.as_str());
    assert_eq!(result.agents[0].last_task_message.as_deref(), Some("build"));
}

#[test]
fn multi_agent_v2_list_agents_omits_closed_agents() {
    run_large_stack_async_test(
        "multi_agent_v2_list_agents_omits_closed_agents",
        multi_agent_v2_list_agents_omits_closed_agents_impl(),
    );
}

async fn multi_agent_v2_list_agents_omits_closed_agents_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    let _ = config.features.enable(Feature::MultiAgentV2);
    turn.config = Arc::new(config);

    let session = Arc::new(session);
    let turn = Arc::new(turn);
    let spawn_output = SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn_agent should succeed");
    let _ = expect_text_output(spawn_output);

    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker path should resolve");
    session
        .services
        .agent_control
        .close_agent(agent_id)
        .await
        .expect("close_agent should succeed");

    let output = ListAgentsHandlerV2
        .handle(invocation(
            session,
            turn,
            "list_agents",
            function_payload(json!({})),
        ))
        .await
        .expect("list_agents should succeed");
    let (content, _) = expect_text_output(output);
    let result: ListAgentsResult =
        serde_json::from_str(&content).expect("list_agents result should be json");

    assert_eq!(result.agents.len(), 1);
    assert_eq!(result.agents[0].agent_name, "/root");
    assert_eq!(
        result.agents[0].last_task_message.as_deref(),
        Some("Main thread")
    );
}

#[tokio::test]
async fn multi_agent_v2_send_message_rejects_legacy_items_field() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = turn.config.as_ref().clone();
    let _ = config.features.enable(Feature::MultiAgentV2);
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");
    let invocation = invocation(
        session,
        turn,
        "send_message",
        function_payload(json!({
            "target": agent_id.to_string(),
            "items": [
                {"type": "mention", "name": "drive", "path": "app://google_drive"},
                {"type": "text", "text": "read the folder"}
            ]
        })),
    );

    let Err(err) = SendMessageHandlerV2.handle(invocation).await else {
        panic!("legacy items field should be rejected in v2");
    };
    let FunctionCallError::RespondToModel(message) = err else {
        panic!("legacy items field should surface as a model-facing error");
    };
    assert!(message.contains("unknown field `items`"));
}

#[tokio::test]
async fn multi_agent_v2_send_message_rejects_interrupt_parameter() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = turn.config.as_ref().clone();
    let _ = config.features.enable(Feature::MultiAgentV2);
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");

    let invocation = invocation(
        session,
        turn,
        "send_message",
        function_payload(json!({
            "target": agent_id.to_string(),
            "message": "continue",
            "interrupt": true
        })),
    );

    let Err(err) = SendMessageHandlerV2.handle(invocation).await else {
        panic!("send_message interrupt parameter should be rejected");
    };
    let FunctionCallError::RespondToModel(message) = err else {
        panic!("expected model-facing parse error");
    };
    assert!(message.starts_with(
        "failed to parse function arguments: unknown field `interrupt`, expected `target` or `message`"
    ));

    let ops = manager.captured_ops();
    let ops_for_agent: Vec<&Op> = ops
        .iter()
        .filter_map(|(id, op)| (*id == agent_id).then_some(op))
        .collect();
    assert!(!ops_for_agent.iter().any(|op| matches!(op, Op::Interrupt)));
    assert!(!ops_for_agent.iter().any(|op| matches!(
        op,
        Op::InterAgentCommunication { communication }
            if communication.author == AgentPath::root()
                && communication.recipient.as_str() == "/root/worker"
                && communication.other_recipients.is_empty()
                && communication.content == "continue"
                && !communication.trigger_turn
    )));
}

#[test]
fn multi_agent_v2_followup_task_completion_notifies_parent_on_every_turn() {
    run_large_stack_async_test(
        "multi_agent_v2_followup_task_completion_notifies_parent_on_every_turn",
        multi_agent_v2_followup_task_completion_notifies_parent_on_every_turn_impl(),
    );
}

async fn multi_agent_v2_followup_task_completion_notifies_parent_on_every_turn_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = turn.config.as_ref().clone();
    let _ = config.features.enable(Feature::MultiAgentV2);
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");
    let thread = manager
        .get_thread(agent_id)
        .await
        .expect("worker thread should exist");
    let worker_path = AgentPath::try_from("/root/worker").expect("worker path");

    let first_turn = thread.codex.session.new_default_turn().await;
    thread
        .codex
        .session
        .send_event(
            first_turn.as_ref(),
            EventMsg::TurnComplete(TurnCompleteEvent {
                turn_id: first_turn.sub_id.clone(),
                last_agent_message: Some("first done".to_string()),
                completed_at: None,
                duration_ms: None,
                time_to_first_token_ms: None,
            }),
        )
        .await;

    FollowupTaskHandlerV2
        .handle(invocation(
            session,
            turn,
            "followup_task",
            function_payload(json!({
                "target": agent_id.to_string(),
                "message": "continue",
            })),
        ))
        .await
        .expect("followup_task should succeed");

    let second_turn = thread.codex.session.new_default_turn().await;
    thread
        .codex
        .session
        .send_event(
            second_turn.as_ref(),
            EventMsg::TurnComplete(TurnCompleteEvent {
                turn_id: second_turn.sub_id.clone(),
                last_agent_message: Some("second done".to_string()),
                completed_at: None,
                duration_ms: None,
                time_to_first_token_ms: None,
            }),
        )
        .await;

    let first_notification = format_subagent_notification_message(
        worker_path.as_str(),
        &AgentStatus::Completed(Some("first done".to_string())),
    );
    let second_notification = format_subagent_notification_message(
        worker_path.as_str(),
        &AgentStatus::Completed(Some("second done".to_string())),
    );

    let notifications = timeout(Duration::from_secs(5), async {
        loop {
            let notifications = manager
                .captured_ops()
                .into_iter()
                .filter_map(|(id, op)| {
                    (id == root.thread_id)
                        .then_some(op)
                        .and_then(|op| match op {
                            Op::InterAgentCommunication { communication }
                                if communication.author == worker_path
                                    && communication.recipient == AgentPath::root()
                                    && communication.other_recipients.is_empty()
                                    && !communication.trigger_turn =>
                            {
                                Some(communication.content)
                            }
                            _ => None,
                        })
                })
                .collect::<Vec<_>>();
            let first_count = notifications
                .iter()
                .filter(|message| **message == first_notification)
                .count();
            let second_count = notifications
                .iter()
                .filter(|message| **message == second_notification)
                .count();
            if first_count == 1 && second_count == 1 {
                break notifications;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("parent should receive one completion notification per child turn");

    assert_eq!(notifications.len(), 2);
}

#[test]
fn multi_agent_v2_followup_task_rejects_legacy_items_field() {
    run_large_stack_async_test(
        "multi_agent_v2_followup_task_rejects_legacy_items_field",
        multi_agent_v2_followup_task_rejects_legacy_items_field_impl(),
    );
}

async fn multi_agent_v2_followup_task_rejects_legacy_items_field_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = turn.config.as_ref().clone();
    let _ = config.features.enable(Feature::MultiAgentV2);
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");
    let invocation = invocation(
        session,
        turn,
        "followup_task",
        function_payload(json!({
            "target": agent_id.to_string(),
            "items": [{"type": "text", "text": "continue"}],
        })),
    );

    let Err(err) = FollowupTaskHandlerV2.handle(invocation).await else {
        panic!("legacy items field should be rejected in v2");
    };
    let FunctionCallError::RespondToModel(message) = err else {
        panic!("legacy items field should surface as a model-facing error");
    };
    assert!(message.contains("unknown field `items`"));
}

#[test]
fn multi_agent_v2_interrupted_turn_does_not_notify_parent() {
    run_large_stack_async_test(
        "multi_agent_v2_interrupted_turn_does_not_notify_parent",
        multi_agent_v2_interrupted_turn_does_not_notify_parent_impl(),
    );
}

async fn multi_agent_v2_interrupted_turn_does_not_notify_parent_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = turn.config.as_ref().clone();
    let _ = config.features.enable(Feature::MultiAgentV2);
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");
    let thread = manager
        .get_thread(agent_id)
        .await
        .expect("worker thread should exist");

    let aborted_turn = thread.codex.session.new_default_turn().await;
    thread
        .codex
        .session
        .send_event(
            aborted_turn.as_ref(),
            EventMsg::TurnAborted(TurnAbortedEvent {
                turn_id: Some(aborted_turn.sub_id.clone()),
                reason: TurnAbortReason::Interrupted,
                completed_at: None,
                duration_ms: None,
            }),
        )
        .await;

    let notifications = manager
        .captured_ops()
        .into_iter()
        .filter_map(|(id, op)| {
            (id == root.thread_id)
                .then_some(op)
                .and_then(|op| match op {
                    Op::InterAgentCommunication { communication }
                        if communication.author.as_str() == "/root/worker"
                            && communication.recipient == AgentPath::root()
                            && communication.other_recipients.is_empty()
                            && !communication.trigger_turn =>
                    {
                        Some(communication.content)
                    }
                    _ => None,
                })
        })
        .collect::<Vec<_>>();

    assert_eq!(notifications, Vec::<String>::new());
}

#[test]
fn multi_agent_v2_spawn_omits_agent_id_when_named() {
    run_large_stack_async_test(
        "multi_agent_v2_spawn_omits_agent_id_when_named",
        multi_agent_v2_spawn_omits_agent_id_when_named_impl(),
    );
}

async fn multi_agent_v2_spawn_omits_agent_id_when_named_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let output = SpawnAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "test_process"
            })),
        ))
        .await
        .expect("spawn_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: serde_json::Value =
        serde_json::from_str(&content).expect("spawn_agent result should be json");

    assert!(result.get("agent_id").is_none());
    assert_eq!(result["task_name"], "/root/test_process");
    assert!(result.get("nickname").is_some());
    assert_eq!(
        result["admission_shadow_decision"]["decision"],
        "deny_shadow_no_live_blocking"
    );
    assert_eq!(
        result["admission_shadow_decision"]["sourceSurfaceId"],
        "spawn_agent_v2"
    );
    assert_eq!(
        result["admission_shadow_decision"]["liveCutoverEnabled"],
        false
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["manifestId"],
        "agent-card:spawn_agent_v2:default"
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["decision"],
        "deny_shadow_manifest_no_live_blocking"
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["configuredManifestShadowDecision"]
            ["decision"],
        "configured_manifest_missing_shadow_no_live_blocking"
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["configuredManifestShadowDecision"]
            ["registrySource"],
        "default_agent_card_manifest_registry"
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["lane"],
        "subagent"
    );
    let task_result_plan = &result["admission_shadow_decision"]["roleManifestShadowDecision"]["taskResultContractShadowPlan"];
    assert_eq!(
        task_result_plan["decision"],
        "task_result_contract_plan_blocked_shadow_no_live_cutover"
    );
    assert_eq!(
        task_result_plan["taskResultContractId"],
        "subagent_task_result_contract_v1"
    );
    assert_eq!(
        task_result_plan["terminalDeliverySurface"],
        "wait_agent(result_required=true)"
    );
    assert_eq!(
        task_result_plan["missingContractParts"],
        json!(["task_result_contract", "verifier", "reducer"])
    );
    assert_eq!(task_result_plan["contractPlanReady"], false);
    assert_eq!(task_result_plan["liveCutoverEnabled"], false);
    assert_eq!(success, Some(true));
}

#[test]
fn multi_agent_v2_spawn_records_configured_role_manifest_shadow_source() {
    run_large_stack_async_test(
        "multi_agent_v2_spawn_records_configured_role_manifest_shadow_source",
        multi_agent_v2_spawn_records_configured_role_manifest_shadow_source_impl(),
    );
}

async fn multi_agent_v2_spawn_records_configured_role_manifest_shadow_source_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    config.agent_roles.insert(
        "reviewer".to_string(),
        AgentRoleConfig {
            description: Some("Reviews configured manifest shadow output".to_string()),
            config_file: None,
            agent_card_manifest_source: Some("agent-card://reviewer".to_string()),
            agent_card_manifest_version: Some("hepta.agent_card_manifest.v1".to_string()),
            agent_card_manifest: Some(AgentCardManifestConfig {
                schema_version: Some("hepta.agent_card_manifest.v1".to_string()),
                source_surface_id: Some("spawn_agent_v2".to_string()),
                capabilities: vec![
                    "local_subagent_spawn".to_string(),
                    "inter_agent_mailbox".to_string(),
                    "named_task_path".to_string(),
                ],
                allowed_tools: vec![
                    "send_message".to_string(),
                    "followup_task".to_string(),
                    "wait_agent".to_string(),
                    "close_agent".to_string(),
                ],
                lane: Some("subagent".to_string()),
                max_threads: Some(2),
                max_depth: None,
            }),
            nickname_candidates: None,
        },
    );
    turn.config = Arc::new(config);

    let output = SpawnAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "review_task",
                "agent_type": "reviewer",
                "fork_turns": "none"
            })),
        ))
        .await
        .expect("spawn_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: serde_json::Value =
        serde_json::from_str(&content).expect("spawn_agent result should be json");

    assert_eq!(success, Some(true));
    let configured_decision = &result["admission_shadow_decision"]["roleManifestShadowDecision"]["configuredManifestShadowDecision"];
    assert_eq!(
        configured_decision["decision"],
        "configured_manifest_present_shadow_no_live_blocking"
    );
    assert_eq!(configured_decision["configuredManifestPresent"], true);
    assert_eq!(
        configured_decision["configuredManifestSource"],
        "agent-card://reviewer"
    );
    assert_eq!(
        configured_decision["expectedManifestVersion"],
        "hepta.agent_card_manifest.v1"
    );
    assert_eq!(
        configured_decision["configuredManifestVersion"],
        "hepta.agent_card_manifest.v1"
    );
    assert_eq!(configured_decision["stale"], false);
    assert_eq!(configured_decision["versionMatches"], true);
    assert_eq!(
        configured_decision["configuredManifestOverlayShadowDecision"]["decision"],
        "configured_manifest_overlay_compatible_shadow_no_live_blocking"
    );
    assert_eq!(
        configured_decision["configuredManifestOverlayShadowDecision"]["laneMatches"],
        true
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["promotionReadinessShadowDecision"]
            ["decision"],
        "promotion_not_ready_shadow_no_live_cutover"
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["promotionReadinessShadowDecision"]
            ["configuredManifestReady"],
        true
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["promotionReadinessShadowDecision"]
            ["configuredOverlayReady"],
        true
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["promotionReadinessShadowDecision"]
            ["liveBlockingEnabled"],
        false
    );
    assert_eq!(
        result["admission_shadow_decision"]["roleManifestShadowDecision"]["roleDeclared"],
        true
    );
    assert_eq!(
        result["admission_shadow_decision"]["liveCutoverEnabled"],
        false
    );
}

#[tokio::test]
async fn multi_agent_v2_spawn_surfaces_task_name_validation_errors() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({
            "message": "inspect this repo",
            "task_name": "BadName"
        })),
    );
    let Err(err) = SpawnAgentHandlerV2::default().handle(invocation).await else {
        panic!("invalid agent name should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "agent_name must use only lowercase letters, digits, and underscores".to_string()
        )
    );
}

#[tokio::test]
async fn spawn_agent_reapplies_runtime_sandbox_after_role_config() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        agent_id: String,
        nickname: Option<String>,
    }

    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let expected_sandbox = turn.config.legacy_sandbox_policy();
    #[allow(deprecated)]
    let mut expected_file_system_sandbox_policy =
        FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(&expected_sandbox, &turn.cwd);
    expected_file_system_sandbox_policy
        .entries
        .push(FileSystemSandboxEntry {
            path: FileSystemPath::GlobPattern {
                pattern: "**/.env".to_string(),
            },
            access: FileSystemAccessMode::None,
        });
    let expected_network_sandbox_policy = NetworkSandboxPolicy::from(&expected_sandbox);
    let expected_permission_profile = PermissionProfile::from_runtime_permissions_with_enforcement(
        SandboxEnforcement::from_legacy_sandbox_policy(&expected_sandbox),
        &expected_file_system_sandbox_policy,
        expected_network_sandbox_policy,
    );
    turn.approval_policy
        .set(AskForApproval::OnRequest)
        .expect("approval policy should be set");
    turn.permission_profile = expected_permission_profile.clone();
    assert_ne!(
        expected_permission_profile,
        turn.config.permissions.effective_permission_profile(),
        "test requires a runtime profile override that differs from base config"
    );

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({
            "message": "await this command",
            "agent_type": "explorer"
        })),
    );
    let output = SpawnAgentHandler::default()
        .handle(invocation)
        .await
        .expect("spawn_agent should succeed");
    let (content, _) = expect_text_output(output);
    let result: SpawnAgentResult =
        serde_json::from_str(&content).expect("spawn_agent result should be json");
    let agent_id = parse_agent_id(&result.agent_id);
    assert!(
        result
            .nickname
            .as_deref()
            .is_some_and(|nickname| !nickname.is_empty())
    );

    let snapshot = manager
        .get_thread(agent_id)
        .await
        .expect("spawned agent thread should exist")
        .config_snapshot()
        .await;
    assert_eq!(snapshot.sandbox_policy(), expected_sandbox);
    assert_eq!(snapshot.approval_policy, AskForApproval::OnRequest);
    assert_eq!(snapshot.permission_profile, expected_permission_profile);
    let child_thread = manager
        .get_thread(agent_id)
        .await
        .expect("spawned agent thread should exist");
    let child_turn = child_thread.codex.session.new_default_turn().await;
    assert_eq!(
        child_turn.file_system_sandbox_policy(),
        expected_file_system_sandbox_policy
    );
    assert_eq!(
        child_turn.network_sandbox_policy(),
        expected_network_sandbox_policy
    );
    assert_eq!(child_turn.permission_profile(), expected_permission_profile);
}

#[tokio::test]
async fn spawn_agent_rejects_when_depth_limit_exceeded() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();

    let max_depth = turn.config.agent_max_depth;
    turn.session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: session.conversation_id,
        depth: max_depth,
        agent_path: None,
        agent_nickname: None,
        agent_role: None,
    });

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({"message": "hello"})),
    );
    let Err(err) = SpawnAgentHandler::default().handle(invocation).await else {
        panic!("spawn should fail when depth limit exceeded");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "Agent depth limit reached. Solve the task yourself.".to_string()
        )
    );
}

#[tokio::test]
async fn spawn_agent_allows_depth_up_to_configured_max_depth() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        agent_id: String,
        nickname: Option<String>,
    }

    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();

    let mut config = (*turn.config).clone();
    config.agent_max_depth = DEFAULT_AGENT_MAX_DEPTH + 1;
    turn.config = Arc::new(config);
    turn.session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: session.conversation_id,
        depth: DEFAULT_AGENT_MAX_DEPTH,
        agent_path: None,
        agent_nickname: None,
        agent_role: None,
    });

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({"message": "hello"})),
    );
    let output = SpawnAgentHandler::default()
        .handle(invocation)
        .await
        .expect("spawn should succeed within configured depth");
    let (content, success) = expect_text_output(output);
    let result: SpawnAgentResult =
        serde_json::from_str(&content).expect("spawn_agent result should be json");
    assert!(!result.agent_id.is_empty());
    assert!(
        result
            .nickname
            .as_deref()
            .is_some_and(|nickname| !nickname.is_empty())
    );
    assert_eq!(success, Some(true));
}

#[tokio::test]
async fn multi_agent_v2_spawn_agent_ignores_configured_max_depth() {
    #[derive(Debug, Deserialize)]
    struct SpawnAgentResult {
        task_name: String,
        nickname: Option<String>,
    }

    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let mut config = (*turn.config).clone();
    config.agent_max_depth = 1;
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    let root = manager
        .start_thread(config.clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    turn.config = Arc::new(config);
    let parent_path = AgentPath::try_from("/root/parent").expect("agent path");
    turn.session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: root.thread_id,
        depth: 1,
        agent_path: Some(parent_path),
        agent_nickname: None,
        agent_role: None,
    });

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "spawn_agent",
        function_payload(json!({
            "message": "hello",
            "task_name": "child",
            "fork_turns": "none"
        })),
    );
    let output = SpawnAgentHandlerV2::default()
        .handle(invocation)
        .await
        .expect("multi-agent v2 spawn should ignore max depth");
    let (content, success) = expect_text_output(output);
    let result: SpawnAgentResult =
        serde_json::from_str(&content).expect("spawn_agent result should be json");
    assert_eq!(result.task_name, "/root/parent/child");
    assert!(result.nickname.is_some());
    assert_eq!(success, Some(true));
}

#[tokio::test]
async fn send_input_rejects_empty_message() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "send_input",
        function_payload(json!({"target": ThreadId::new().to_string(), "message": ""})),
    );
    let Err(err) = SendInputHandler.handle(invocation).await else {
        panic!("empty message should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("Empty message can't be sent to an agent".to_string())
    );
}

#[tokio::test]
async fn send_input_rejects_when_message_and_items_are_both_set() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "send_input",
        function_payload(json!({
            "target": ThreadId::new().to_string(),
            "message": "hello",
            "items": [{"type": "mention", "name": "drive", "path": "app://drive"}]
        })),
    );
    let Err(err) = SendInputHandler.handle(invocation).await else {
        panic!("message+items should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "Provide either message or items, but not both".to_string()
        )
    );
}

#[tokio::test]
async fn send_input_rejects_invalid_id() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "send_input",
        function_payload(json!({"target": "not-a-uuid", "message": "hi"})),
    );
    let Err(err) = SendInputHandler.handle(invocation).await else {
        panic!("invalid id should be rejected");
    };
    let FunctionCallError::RespondToModel(msg) = err else {
        panic!("expected respond-to-model error");
    };
    assert!(msg.starts_with("invalid agent id not-a-uuid:"));
}

#[tokio::test]
async fn send_input_reports_missing_agent() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let agent_id = ThreadId::new();
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "send_input",
        function_payload(json!({"target": agent_id.to_string(), "message": "hi"})),
    );
    let Err(err) = SendInputHandler.handle(invocation).await else {
        panic!("missing agent should be reported");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(format!("agent with id {agent_id} not found"))
    );
}

#[tokio::test]
async fn send_input_interrupts_before_prompt() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let config = turn.config.as_ref().clone();
    let thread = manager
        .start_thread(config.clone())
        .await
        .expect("start thread");
    let agent_id = thread.thread_id;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "send_input",
        function_payload(json!({
            "target": agent_id.to_string(),
            "message": "hi",
            "interrupt": true
        })),
    );
    SendInputHandler
        .handle(invocation)
        .await
        .expect("send_input should succeed");

    let ops = manager.captured_ops();
    let ops_for_agent: Vec<&Op> = ops
        .iter()
        .filter_map(|(id, op)| (*id == agent_id).then_some(op))
        .collect();
    assert_eq!(ops_for_agent.len(), 2);
    assert!(matches!(ops_for_agent[0], Op::Interrupt));
    assert!(matches!(ops_for_agent[1], Op::UserInput { .. }));

    let _ = thread
        .thread
        .submit(Op::Shutdown {})
        .await
        .expect("shutdown should submit");
}

#[tokio::test]
async fn send_input_accepts_structured_items() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let config = turn.config.as_ref().clone();
    let thread = manager
        .start_thread(config.clone())
        .await
        .expect("start thread");
    let agent_id = thread.thread_id;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "send_input",
        function_payload(json!({
            "target": agent_id.to_string(),
            "items": [
                {"type": "mention", "name": "drive", "path": "app://google_drive"},
                {"type": "text", "text": "read the folder"}
            ]
        })),
    );
    SendInputHandler
        .handle(invocation)
        .await
        .expect("send_input should succeed");

    let expected = Op::UserInput {
        environments: None,
        items: vec![
            UserInput::Mention {
                name: "drive".to_string(),
                path: "app://google_drive".to_string(),
            },
            UserInput::Text {
                text: "read the folder".to_string(),
                text_elements: Vec::new(),
            },
        ],
        final_output_json_schema: None,
        responsesapi_client_metadata: None,
    };
    let captured = manager
        .captured_ops()
        .into_iter()
        .find(|(id, op)| *id == agent_id && *op == expected);
    assert_eq!(captured, Some((agent_id, expected)));

    let _ = thread
        .thread
        .submit(Op::Shutdown {})
        .await
        .expect("shutdown should submit");
}

#[tokio::test]
async fn resume_agent_rejects_invalid_id() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "resume_agent",
        function_payload(json!({"id": "not-a-uuid"})),
    );
    let Err(err) = ResumeAgentHandler.handle(invocation).await else {
        panic!("invalid id should be rejected");
    };
    let FunctionCallError::RespondToModel(msg) = err else {
        panic!("expected respond-to-model error");
    };
    assert!(msg.starts_with("invalid agent id not-a-uuid:"));
}

#[tokio::test]
async fn resume_agent_reports_missing_agent() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let agent_id = ThreadId::new();
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "resume_agent",
        function_payload(json!({"id": agent_id.to_string()})),
    );
    let Err(err) = ResumeAgentHandler.handle(invocation).await else {
        panic!("missing agent should be reported");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(format!("agent with id {agent_id} not found"))
    );
}

#[tokio::test]
async fn resume_agent_noops_for_active_agent() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let config = turn.config.as_ref().clone();
    let thread = manager
        .start_thread(config.clone())
        .await
        .expect("start thread");
    let agent_id = thread.thread_id;
    let status_before = manager.agent_control().get_status(agent_id).await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "resume_agent",
        function_payload(json!({"id": agent_id.to_string()})),
    );

    let output = ResumeAgentHandler
        .handle(invocation)
        .await
        .expect("resume_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: resume_agent::ResumeAgentResult =
        serde_json::from_str(&content).expect("resume_agent result should be json");
    assert_eq!(result.status, status_before);
    assert_eq!(success, Some(true));

    let thread_ids = manager.list_thread_ids().await;
    assert_eq!(thread_ids, vec![agent_id]);

    let _ = thread
        .thread
        .submit(Op::Shutdown {})
        .await
        .expect("shutdown should submit");
}

#[tokio::test]
async fn resume_agent_restores_closed_agent_and_accepts_send_input() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let config = turn.config.as_ref().clone();
    let thread = manager
        .resume_thread_with_history(
            config.clone(),
            InitialHistory::Forked(vec![RolloutItem::ResponseItem(ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputText {
                    text: "materialized".to_string(),
                }],
                phase: None,
            })]),
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("dummy")),
            /*persist_extended_history*/ false,
            /*parent_trace*/ None,
        )
        .await
        .expect("start thread");
    let agent_id = thread.thread_id;
    let _ = manager
        .agent_control()
        .shutdown_live_agent(agent_id)
        .await
        .expect("shutdown agent");
    assert_eq!(
        manager.agent_control().get_status(agent_id).await,
        AgentStatus::NotFound
    );
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    let resume_invocation = invocation(
        session.clone(),
        turn.clone(),
        "resume_agent",
        function_payload(json!({"id": agent_id.to_string()})),
    );
    let output = ResumeAgentHandler
        .handle(resume_invocation)
        .await
        .expect("resume_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: resume_agent::ResumeAgentResult =
        serde_json::from_str(&content).expect("resume_agent result should be json");
    assert_ne!(result.status, AgentStatus::NotFound);
    assert_eq!(success, Some(true));

    let send_invocation = invocation(
        session,
        turn,
        "send_input",
        function_payload(json!({"target": agent_id.to_string(), "message": "hello"})),
    );
    let output = SendInputHandler
        .handle(send_invocation)
        .await
        .expect("send_input should succeed after resume");
    let (content, success) = expect_text_output(output);
    let result: serde_json::Value =
        serde_json::from_str(&content).expect("send_input result should be json");
    let submission_id = result
        .get("submission_id")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    assert!(!submission_id.is_empty());
    assert_eq!(success, Some(true));

    let _ = manager
        .agent_control()
        .shutdown_live_agent(agent_id)
        .await
        .expect("shutdown resumed agent");
}

#[tokio::test]
async fn resume_agent_rejects_when_depth_limit_exceeded() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();

    let max_depth = turn.config.agent_max_depth;
    turn.session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: session.conversation_id,
        depth: max_depth,
        agent_path: None,
        agent_nickname: None,
        agent_role: None,
    });

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "resume_agent",
        function_payload(json!({"id": ThreadId::new().to_string()})),
    );
    let Err(err) = ResumeAgentHandler.handle(invocation).await else {
        panic!("resume should fail when depth limit exceeded");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "Agent depth limit reached. Solve the task yourself.".to_string()
        )
    );
}

#[tokio::test]
async fn wait_agent_rejects_non_positive_timeout() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "wait_agent",
        function_payload(json!({
            "targets": [ThreadId::new().to_string()],
            "timeout_ms": 0
        })),
    );
    let Err(err) = WaitAgentHandler::default().handle(invocation).await else {
        panic!("non-positive timeout should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("timeout_ms must be greater than zero".to_string())
    );
}

#[tokio::test]
async fn wait_agent_rejects_invalid_target() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "wait_agent",
        function_payload(json!({"targets": ["invalid"]})),
    );
    let Err(err) = WaitAgentHandler::default().handle(invocation).await else {
        panic!("invalid id should be rejected");
    };
    let FunctionCallError::RespondToModel(msg) = err else {
        panic!("expected respond-to-model error");
    };
    assert!(msg.starts_with("invalid agent id invalid:"));
}

#[tokio::test]
async fn wait_agent_rejects_empty_targets() {
    let (session, turn) = make_session_and_context().await;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "wait_agent",
        function_payload(json!({"targets": []})),
    );
    let Err(err) = WaitAgentHandler::default().handle(invocation).await else {
        panic!("empty ids should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("agent ids must be non-empty".to_string())
    );
}

#[test]
fn multi_agent_v2_wait_agent_accepts_timeout_only_argument() {
    run_large_stack_async_test(
        "multi_agent_v2_wait_agent_accepts_timeout_only_argument",
        multi_agent_v2_wait_agent_accepts_timeout_only_argument_impl(),
    );
}

async fn multi_agent_v2_wait_agent_accepts_timeout_only_argument_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");
    let worker_path = session
        .services
        .agent_control
        .get_agent_metadata(agent_id)
        .expect("worker metadata")
        .agent_path
        .expect("worker path");

    let wait_task = tokio::spawn({
        let session = session.clone();
        let turn = turn.clone();
        async move {
            WaitAgentHandlerV2::default()
                .handle(invocation(
                    session,
                    turn,
                    "wait_agent",
                    function_payload(json!({"timeout_ms": 10_000})),
                ))
                .await
        }
    });
    tokio::task::yield_now().await;

    session.enqueue_mailbox_communication(InterAgentCommunication::new(
        worker_path,
        AgentPath::root(),
        Vec::new(),
        "hello from worker".to_string(),
        /*trigger_turn*/ false,
    ));

    let output = wait_task
        .await
        .expect("wait task should join")
        .expect("timeout-only args should be accepted in v2 mode");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait completed.".to_string(),
            timed_out: false,
        }
    );
    assert_eq!(success, None);
}

#[tokio::test]
async fn multi_agent_v2_wait_agent_rejects_timeout_below_configured_min() {
    let (session, mut turn) = make_session_and_context().await;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    config.multi_agent_v2.min_wait_timeout_ms = 50;
    config.multi_agent_v2.max_wait_timeout_ms = 1_000;
    config.multi_agent_v2.default_wait_timeout_ms = 50;
    turn.config = Arc::new(config);

    let Err(err) = WaitAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "wait_agent",
            function_payload(json!({"timeout_ms": 1})),
        ))
        .await
    else {
        panic!("timeout below configured minimum should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("timeout_ms must be at least 50".to_string())
    );
}

#[tokio::test]
async fn multi_agent_v2_wait_agent_accepts_explicit_timeout_at_configured_min() {
    let (session, mut turn) = make_session_and_context().await;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    config.multi_agent_v2.min_wait_timeout_ms = 1;
    config.multi_agent_v2.max_wait_timeout_ms = 1_000;
    config.multi_agent_v2.default_wait_timeout_ms = 50;
    turn.config = Arc::new(config);

    let output = WaitAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "wait_agent",
            function_payload(json!({"timeout_ms": 1})),
        ))
        .await
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait timed out.".to_string(),
            timed_out: true,
        }
    );
    assert_eq!(success, None);
}

#[tokio::test]
async fn multi_agent_v2_wait_agent_uses_configured_default_timeout() {
    let (session, mut turn) = make_session_and_context().await;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    config.multi_agent_v2.min_wait_timeout_ms = 1;
    config.multi_agent_v2.max_wait_timeout_ms = 1_000;
    config.multi_agent_v2.default_wait_timeout_ms = 50;
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    let early = timeout(
        Duration::from_millis(/*millis*/ 20),
        WaitAgentHandlerV2::default().handle(invocation(
            session.clone(),
            turn.clone(),
            "wait_agent",
            function_payload(json!({})),
        )),
    )
    .await;
    assert!(
        early.is_err(),
        "wait_agent should not return before the configured default timeout"
    );

    let output = timeout(
        Duration::from_secs(/*secs*/ 1),
        WaitAgentHandlerV2::default().handle(invocation(
            session,
            turn,
            "wait_agent",
            function_payload(json!({})),
        )),
    )
    .await
    .expect("configured default should be shorter than the test timeout")
    .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait timed out.".to_string(),
            timed_out: true,
        }
    );
    assert_eq!(success, None);
}

#[tokio::test]
async fn multi_agent_v2_wait_agent_allows_zero_configured_timeout() {
    let (session, mut turn) = make_session_and_context().await;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    config.multi_agent_v2.min_wait_timeout_ms = 0;
    config.multi_agent_v2.max_wait_timeout_ms = 0;
    config.multi_agent_v2.default_wait_timeout_ms = 0;
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    let output = timeout(
        Duration::from_secs(/*secs*/ 1),
        WaitAgentHandlerV2::default().handle(invocation(
            session,
            turn,
            "wait_agent",
            function_payload(json!({})),
        )),
    )
    .await
    .expect("zero timeout should complete immediately")
    .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait timed out.".to_string(),
            timed_out: true,
        }
    );
    assert_eq!(success, None);
}

#[tokio::test]
async fn multi_agent_v2_wait_agent_rejects_timeout_above_configured_max() {
    let (session, mut turn) = make_session_and_context().await;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    config.multi_agent_v2.min_wait_timeout_ms = 1;
    config.multi_agent_v2.max_wait_timeout_ms = 50;
    config.multi_agent_v2.default_wait_timeout_ms = 1;
    turn.config = Arc::new(config);

    let Err(err) = WaitAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "wait_agent",
            function_payload(json!({"timeout_ms": 500})),
        ))
        .await
    else {
        panic!("timeout above configured maximum should be rejected");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("timeout_ms must be at most 50".to_string())
    );
}

#[tokio::test]
async fn multi_agent_v2_wait_agent_accepts_explicit_timeout_at_configured_max() {
    let (session, mut turn) = make_session_and_context().await;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    config.multi_agent_v2.min_wait_timeout_ms = 1;
    config.multi_agent_v2.max_wait_timeout_ms = 1;
    config.multi_agent_v2.default_wait_timeout_ms = 1;
    turn.config = Arc::new(config);

    let output = WaitAgentHandlerV2::default()
        .handle(invocation(
            Arc::new(session),
            Arc::new(turn),
            "wait_agent",
            function_payload(json!({"timeout_ms": 1})),
        ))
        .await
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait timed out.".to_string(),
            timed_out: true,
        }
    );
    assert_eq!(success, None);
}

#[tokio::test]
async fn wait_agent_returns_not_found_for_missing_agents() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let id_a = ThreadId::new();
    let id_b = ThreadId::new();
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "wait_agent",
        function_payload(json!({
            "targets": [id_a.to_string(), id_b.to_string()],
            "timeout_ms": 10_000
        })),
    );
    let output = WaitAgentHandler::default()
        .handle(invocation)
        .await
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        wait::WaitAgentResult {
            status: HashMap::from([
                (id_a.to_string(), AgentStatus::NotFound),
                (id_b.to_string(), AgentStatus::NotFound),
            ]),
            timed_out: false
        }
    );
    assert_eq!(success, None);
}

#[tokio::test]
async fn wait_agent_times_out_when_status_is_not_final() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let config = turn.config.as_ref().clone();
    let thread = manager
        .start_thread(config.clone())
        .await
        .expect("start thread");
    let agent_id = thread.thread_id;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "wait_agent",
        function_payload(json!({
            "targets": [agent_id.to_string()],
            "timeout_ms": MIN_WAIT_TIMEOUT_MS
        })),
    );
    let output = WaitAgentHandler::default()
        .handle(invocation)
        .await
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        wait::WaitAgentResult {
            status: HashMap::new(),
            timed_out: true
        }
    );
    assert_eq!(success, None);

    let _ = thread
        .thread
        .submit(Op::Shutdown {})
        .await
        .expect("shutdown should submit");
}

#[tokio::test]
async fn wait_agent_clamps_short_timeouts_to_minimum() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let config = turn.config.as_ref().clone();
    let thread = manager
        .start_thread(config.clone())
        .await
        .expect("start thread");
    let agent_id = thread.thread_id;
    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "wait_agent",
        function_payload(json!({
            "targets": [agent_id.to_string()],
            "timeout_ms": 10
        })),
    );

    let early = timeout(
        Duration::from_millis(50),
        WaitAgentHandler::default().handle(invocation),
    )
    .await;
    assert!(
        early.is_err(),
        "wait_agent should not return before the minimum timeout clamp"
    );

    let _ = thread
        .thread
        .submit(Op::Shutdown {})
        .await
        .expect("shutdown should submit");
}

#[tokio::test]
async fn wait_agent_returns_final_status_without_timeout() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let config = turn.config.as_ref().clone();
    let thread = manager
        .start_thread(config.clone())
        .await
        .expect("start thread");
    let agent_id = thread.thread_id;
    let mut status_rx = manager
        .agent_control()
        .subscribe_status(agent_id)
        .await
        .expect("subscribe should succeed");

    let _ = thread
        .thread
        .submit(Op::Shutdown {})
        .await
        .expect("shutdown should submit");
    let _ = timeout(Duration::from_secs(1), status_rx.changed())
        .await
        .expect("shutdown status should arrive");

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "wait_agent",
        function_payload(json!({
            "targets": [agent_id.to_string()],
            "timeout_ms": 10_000
        })),
    );
    let output = WaitAgentHandler::default()
        .handle(invocation)
        .await
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        wait::WaitAgentResult {
            status: HashMap::from([(agent_id.to_string(), AgentStatus::Shutdown)]),
            timed_out: false
        }
    );
    assert_eq!(success, None);
}

#[test]
fn multi_agent_v2_wait_agent_returns_summary_for_mailbox_activity() {
    run_large_stack_async_test(
        "multi_agent_v2_wait_agent_returns_summary_for_mailbox_activity",
        multi_agent_v2_wait_agent_returns_summary_for_mailbox_activity_impl(),
    );
}

async fn multi_agent_v2_wait_agent_returns_summary_for_mailbox_activity_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let session = Arc::new(session);
    let turn = Arc::new(turn);
    let spawn_output = SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "test_process"
            })),
        ))
        .await
        .expect("spawn_agent should succeed");
    let _ = expect_text_output(spawn_output);

    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(
            session.conversation_id,
            &turn.session_source,
            "test_process",
        )
        .await
        .expect("relative path should resolve");
    let worker_path = session
        .services
        .agent_control
        .get_agent_metadata(agent_id)
        .expect("worker metadata")
        .agent_path
        .expect("worker path");
    let wait_task = tokio::spawn({
        let session = session.clone();
        let turn = turn.clone();
        async move {
            WaitAgentHandlerV2::default()
                .handle(invocation(
                    session,
                    turn,
                    "wait_agent",
                    function_payload(json!({"timeout_ms": 10_000})),
                ))
                .await
        }
    });
    tokio::task::yield_now().await;

    session.enqueue_mailbox_communication(InterAgentCommunication::new(
        worker_path,
        AgentPath::root(),
        Vec::new(),
        "completed".to_string(),
        /*trigger_turn*/ false,
    ));

    let wait_output = wait_task
        .await
        .expect("wait task should join")
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(wait_output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait completed.".to_string(),
            timed_out: false,
        }
    );
    assert_eq!(success, None);
}

#[test]
fn multi_agent_v2_wait_agent_returns_for_already_queued_mail() {
    run_large_stack_async_test(
        "multi_agent_v2_wait_agent_returns_for_already_queued_mail",
        multi_agent_v2_wait_agent_returns_for_already_queued_mail_impl(),
    );
}

async fn multi_agent_v2_wait_agent_returns_for_already_queued_mail_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");
    let worker_path = session
        .services
        .agent_control
        .get_agent_metadata(agent_id)
        .expect("worker metadata")
        .agent_path
        .expect("worker path");

    session.enqueue_mailbox_communication(InterAgentCommunication::new(
        worker_path,
        AgentPath::root(),
        Vec::new(),
        "already queued".to_string(),
        /*trigger_turn*/ false,
    ));

    let output = timeout(
        Duration::from_millis(500),
        WaitAgentHandlerV2::default().handle(invocation(
            session,
            turn,
            "wait_agent",
            function_payload(json!({
                "timeout_ms": 10_000,
                "task_name": "worker",
                "task_id": "task-1",
                "barrier_id": "barrier-1",
                "result_required": false
            })),
        )),
    )
    .await
    .expect("already queued mail should complete wait_agent immediately")
    .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    let result_value: serde_json::Value =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait completed.".to_string(),
            timed_out: false,
        }
    );
    assert_eq!(result_value["barrier_id"], json!("barrier-1"));
    assert_eq!(result_value["task_name"], json!("worker"));
    assert_eq!(result_value["task_id"], json!("task-1"));
    assert_eq!(result_value["task_thread_id"], serde_json::Value::Null);
    assert_eq!(result_value["task_status"], serde_json::Value::Null);
    assert_eq!(result_value["task_result"], serde_json::Value::Null);
    assert_eq!(result_value["result_required"], json!(false));
    assert_eq!(result_value["wait_condition"], json!("mailbox_change"));
    assert_eq!(
        result_value["durable_mailbox"]["live_blocking_enabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["attemptedTool"],
        json!("wait_agent")
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["toolAllowed"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["observedLane"],
        json!("subagent_lifecycle")
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["laneAllowed"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(success, None);
}

#[test]
fn multi_agent_v2_wait_agent_waits_for_task_terminal_status() {
    run_large_stack_async_test(
        "multi_agent_v2_wait_agent_waits_for_task_terminal_status",
        multi_agent_v2_wait_agent_waits_for_task_terminal_status_impl(),
    );
}

async fn multi_agent_v2_wait_agent_waits_for_task_terminal_status_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");
    let child_thread = manager
        .get_thread(agent_id)
        .await
        .expect("worker thread should exist");

    let wait_task = tokio::spawn({
        let session = session.clone();
        let turn = turn.clone();
        async move {
            WaitAgentHandlerV2::default()
                .handle(invocation(
                    session,
                    turn,
                    "wait_agent",
                    function_payload(json!({
                        "timeout_ms": 10_000,
                        "task_name": "worker",
                        "task_id": "task-1",
                        "barrier_id": "barrier-task-1",
                        "result_required": true
                    })),
                ))
                .await
        }
    });
    tokio::task::yield_now().await;

    child_thread
        .submit(Op::Shutdown {})
        .await
        .expect("shutdown should submit");

    let output = wait_task
        .await
        .expect("wait task should join")
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    let result_value: serde_json::Value =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait completed.".to_string(),
            timed_out: false,
        }
    );
    assert_eq!(result_value["barrier_id"], json!("barrier-task-1"));
    assert_eq!(result_value["task_name"], json!("worker"));
    assert_eq!(result_value["task_id"], json!("task-1"));
    assert_eq!(result_value["task_thread_id"], json!(agent_id.to_string()));
    assert_eq!(result_value["task_status"], json!("shutdown"));
    assert_eq!(result_value["task_result"], serde_json::Value::Null);
    assert_eq!(result_value["result_required"], json!(true));
    assert_eq!(
        result_value["wait_condition"],
        json!("task_terminal_status")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["decision"],
        json!("task_result_delivery_blocked_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["taskResultEnvelopePresent"],
        json!(false)
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["taskResultContractId"],
        json!("subagent_task_result_contract_v1")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["terminalDeliverySurface"],
        json!("wait_agent(result_required=true)")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["decision"],
        json!("parent_reducer_shadow_receipt_blocked_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["reducerId"],
        json!("subagent_parent_reducer_v1")
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["parentReducerReceiptReady"],
        json!(false)
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["task_result_delivery_shadow_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["parent_reducer_shadow_receipt_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["task_result_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_surface_audit_packet_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_surface_audit_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_receipt_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_closeout_receipt_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_closeout_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_audit_chain_closeout_receipt_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_operator_review_packet_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_audit_chain_closeout_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_replay_consistency_decision"]["decision"],
        json!("wait_task_result_replay_mismatch_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_replay_consistency_decision"]["readbackReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_replay_consistency_decision"]["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_replay_consistency_decision"]["decision"],
        json!("wait_surface_audit_replay_mismatch_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_replay_consistency_decision"]["readbackReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_replay_consistency_decision"]["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"],
        serde_json::Value::Null
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["sourceSurfaceId"],
        json!("wait_agent")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["readinessStatus"],
        json!("blocked_wait_task_result_readback_not_ready")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["nextBlocker"],
        json!("wait_task_result_readback_not_ready")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["taskResultContractId"],
        json!("subagent_task_result_contract_v1")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["terminalDeliverySurface"],
        json!("wait_agent(result_required=true)")
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["decision"],
        json!("wait_task_result_surface_audit_blocked_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["auditChainSegmentCount"],
        json!(3)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["auditChainReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["auditChainMissingSegmentIds"],
        json!([
            "wait_task_result_delivery_shadow",
            "wait_parent_reducer_shadow_receipt",
            "wait_task_result_replay_consistency"
        ])
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["decision"],
        json!("work_graph_surface_audit_blocked_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["auditChainSegmentCount"],
        json!(5)
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["auditChainReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["auditChainMissingSegmentIds"],
        json!([
            "wait_task_result_delivery_shadow",
            "wait_parent_reducer_shadow_receipt",
            "wait_task_result_replay_consistency",
            "wait_surface_audit_packet",
            "wait_surface_audit_replay_consistency"
        ])
    );
    let global_wait_rows =
        result_value["work_graph_global_surface_audit_packet"]["operatorMatrixRows"]
            .as_array()
            .expect("global surface audit should expose operator rows");
    let global_wait_row = global_wait_rows
        .iter()
        .find(|row| row["sourceSurfaceId"] == json!("wait_agent"))
        .expect("global surface audit should include wait_agent row");
    assert_eq!(
        global_wait_row["readinessStatus"],
        json!("blocked_audit_chain_or_no_live_guardrail_not_ready")
    );
    assert_eq!(
        global_wait_row["taskResultContractId"],
        json!("subagent_task_result_contract_v1")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_receipt"]["decision"],
        json!("work_graph_canonical_projection_blocked_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_receipt"]["readProjectionReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_receipt"]["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_replay_consistency_decision"]["decision"],
        json!("work_graph_canonical_projection_replay_mismatch_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_receipt"]["decision"],
        json!("work_graph_canonical_projection_closeout_blocked_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_receipt"]["closeoutReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_receipt"]["noCutoverTerminalReceipt"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_replay_consistency_decision"]["decision"],
        json!("work_graph_canonical_projection_closeout_replay_mismatch_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_replay_consistency_decision"]["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_receipt"]["decision"],
        json!(
            "work_graph_canonical_projection_audit_chain_closeout_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_receipt"]["auditChainCloseoutReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_receipt"]["noCutoverTerminalReceipt"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["decision"],
        json!(
            "work_graph_canonical_projection_enablement_operator_review_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["enablementOperatorReviewReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_operator_review_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["noLiveEnablementRehearsalCloseoutReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision"]
            ["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["decision"],
        json!(
            "work_graph_canonical_projection_enablement_audit_chain_closeout_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["enablementAuditChainCloseoutReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision"]
            ["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_precondition_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["activationPreconditionReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["reviewedFlagEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_precondition_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["activationNoLiveCloseoutReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["activationAuditChainCloseoutReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["reviewedFlagEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["activationOperatorApprovalReadinessPreflightReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["operatorApprovalRequiredBeforeActivation"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["approvalRecordMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["reviewedFlagRequiredBeforeActivation"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["reviewedFlagMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["activationOperatorApprovalReadinessPreflightPacketMatchesReadback"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["approvalRecordMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["reviewedFlagMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_blocked_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["approvalReviewSideEffectLockCloseoutReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["activationOperatorApprovalReadinessPreflightReplayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["approvalRecordMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["reviewedFlagMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_mismatch_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["activationApprovalReviewSideEffectLockCloseoutPacketMatchesReadback"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["approvalReviewSideEffectLockCloseoutReady"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["approvalRecordMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["reviewedFlagMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(success, None);
}

#[test]
fn multi_agent_v2_wait_agent_satisfies_from_task_result_evidence() {
    run_large_stack_async_test(
        "multi_agent_v2_wait_agent_satisfies_from_task_result_evidence",
        multi_agent_v2_wait_agent_satisfies_from_task_result_evidence_impl(),
    );
}

async fn multi_agent_v2_wait_agent_satisfies_from_task_result_evidence_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let state_db = init_state_db(&turn.config)
        .await
        .expect("state db should initialize");
    session.services.state_db = Some(state_db.clone());
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let job_id = "job-task-result-evidence";
    let item_id = "row-1";
    let task_id = format!("agent-job:{job_id}:{item_id}");
    state_db
        .create_agent_job(
            &codex_state::AgentJobCreateParams {
                id: job_id.to_string(),
                name: "task result evidence".to_string(),
                instruction: "report a result".to_string(),
                auto_export: false,
                max_runtime_seconds: None,
                output_schema_json: None,
                input_headers: vec!["id".to_string()],
                input_csv_path: "input.csv".to_string(),
                output_csv_path: "output.csv".to_string(),
            },
            &[codex_state::AgentJobItemCreateParams {
                item_id: item_id.to_string(),
                row_index: 0,
                source_id: Some("source-1".to_string()),
                row_json: json!({"id": "source-1"}),
            }],
        )
        .await
        .expect("agent job should create");
    state_db
        .mark_agent_job_running(job_id)
        .await
        .expect("agent job should run");
    let reporting_thread_id = session.conversation_id.to_string();
    state_db
        .mark_agent_job_item_running_with_thread(job_id, item_id, reporting_thread_id.as_str())
        .await
        .expect("agent job item should run");
    let task_result_envelope = json!({
        "schemaVersion": "hepta.task_result.v1",
        "taskId": task_id.clone(),
        "status": "completed",
        "summary": "done",
        "liveBlockingEnabled": false,
        "liveCutoverEnabled": false,
    });
    state_db
        .report_agent_job_item_result(
            job_id,
            item_id,
            reporting_thread_id.as_str(),
            &json!({"ok": true}),
            Some(&task_result_envelope),
        )
        .await
        .expect("task result evidence should record");

    let session = Arc::new(session);
    let turn = Arc::new(turn);
    let output = WaitAgentHandlerV2::default()
        .handle(invocation(
            session,
            turn,
            "wait_agent",
            function_payload(json!({
                "timeout_ms": 10_000,
                "task_id": task_id.clone(),
                "barrier_id": "barrier-task-result-evidence",
                "result_required": true
            })),
        ))
        .await
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    let result_value: serde_json::Value =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait completed.".to_string(),
            timed_out: false,
        }
    );
    assert_eq!(
        result_value["barrier_id"],
        json!("barrier-task-result-evidence")
    );
    assert_eq!(result_value["task_id"], json!(task_id));
    assert_eq!(result_value["task_thread_id"], serde_json::Value::Null);
    assert_eq!(result_value["task_status"], serde_json::Value::Null);
    assert_eq!(
        result_value["task_result"]["schemaVersion"],
        json!("hepta.task_result.v1")
    );
    assert_eq!(result_value["task_result"]["status"], json!("completed"));
    assert_eq!(result_value["result_required"], json!(true));
    assert_eq!(
        result_value["wait_condition"],
        json!("task_result_evidence")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["decision"],
        json!("task_result_delivery_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["taskResultEnvelopePresent"],
        json!(true)
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["taskResultStatus"],
        json!("completed")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["taskResultContractId"],
        json!("subagent_task_result_contract_v1")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["terminalDeliverySurface"],
        json!("wait_agent(result_required=true)")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["verifierId"],
        json!("subagent_task_result_verifier_v1")
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["shadowDeliveryReady"],
        json!(true)
    );
    assert_eq!(
        result_value["task_result_delivery_shadow"]["liveBlockingEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["decision"],
        json!("parent_reducer_shadow_receipt_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["taskResultEnvelopeObserved"],
        json!(true)
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["parentReducerReceiptReady"],
        json!(true)
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["reducedIntoParentWorkGraph"],
        json!(false)
    );
    assert_eq!(
        result_value["parent_reducer_shadow_receipt"]["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["durable_mailbox"]["task_result_delivery_shadow_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["parent_reducer_shadow_receipt_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["task_result_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_surface_audit_packet_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_surface_audit_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_receipt_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_closeout_receipt_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_closeout_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_audit_chain_closeout_receipt_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_operator_review_packet_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_audit_chain_closeout_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["durable_mailbox"]["wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_replay_consistency_decision"]["decision"],
        json!("wait_task_result_replay_consistent_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_replay_consistency_decision"]["taskResultDeliveryMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_replay_consistency_decision"]["parentReducerReceiptMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_replay_consistency_decision"]["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_replay_consistency_decision"]["decision"],
        json!("wait_surface_audit_replay_consistent_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_replay_consistency_decision"]["waitSurfaceAuditPacketMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_replay_consistency_decision"]["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["taskResultDeliveryShadowEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["parentReducerShadowReceiptEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["taskResultReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitSurfaceAuditPacketEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitSurfaceAuditReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionReceiptEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionCloseoutReceiptEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionCloseoutReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionAuditChainCloseoutReceiptEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionAuditChainCloseoutReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementOperatorReviewPacketEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementOperatorReviewReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementNoLiveRehearsalCloseoutEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementAuditChainCloseoutEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationPreconditionOperatorPacketEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationPreconditionReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationNoLiveCloseoutEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationAuditChainCloseoutEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyEvents"],
        json!(1)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestTaskResultDeliveryDecision"],
        json!("task_result_delivery_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestParentReducerDecision"],
        json!("parent_reducer_shadow_receipt_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestTaskResultReplayConsistencyDecision"],
        json!("wait_task_result_replay_consistent_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitSurfaceAuditDecision"],
        json!("wait_task_result_surface_audit_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitSurfaceAuditReplayConsistencyDecision"],
        json!("wait_surface_audit_replay_consistent_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionDecision"],
        json!("work_graph_canonical_projection_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionReplayConsistencyDecision"],
        json!("work_graph_canonical_projection_replay_consistent_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionCloseoutDecision"],
        json!("work_graph_canonical_projection_closeout_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionCloseoutReplayConsistencyDecision"],
        json!("work_graph_canonical_projection_closeout_replay_consistent_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionAuditChainCloseoutDecision"],
        json!(
            "work_graph_canonical_projection_audit_chain_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision"],
        json!(
            "work_graph_canonical_projection_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementOperatorReviewDecision"],
        json!(
            "work_graph_canonical_projection_enablement_operator_review_ready_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision"],
        json!(
            "work_graph_canonical_projection_enablement_operator_review_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutDecision"],
        json!(
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision"],
        json!(
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementAuditChainCloseoutDecision"],
        json!(
            "work_graph_canonical_projection_enablement_audit_chain_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision"],
        json!(
            "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementActivationPreconditionOperatorDecision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_precondition_ready_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_precondition_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementActivationNoLiveCloseoutDecision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementActivationAuditChainCloseoutDecision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightDecision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["latestWaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["readbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["replayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitSurfaceAuditPacketReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitSurfaceAuditReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionReceiptReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionCloseoutReceiptReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionCloseoutReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionAuditChainCloseoutReceiptReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionAuditChainCloseoutReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementOperatorReviewPacketReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementOperatorReviewReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementAuditChainCloseoutReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationPreconditionOperatorPacketReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationPreconditionReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationNoLiveCloseoutReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationAuditChainCloseoutReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketReadbackReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitSurfaceAuditPacketReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitSurfaceAuditReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionReceiptReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionCloseoutReceiptReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionCloseoutReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionAuditChainCloseoutReceiptReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionAuditChainCloseoutReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementOperatorReviewReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementOperatorReviewReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementAuditChainCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementAuditChainCloseoutReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationPreconditionReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationPreconditionReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationNoLiveCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationAuditChainCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["waitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionAuditChainCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionAuditChainCloseoutReplayReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementOperatorReviewReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementAuditChainCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementAuditChainCloseoutReplayReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementActivationPreconditionReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementActivationPreconditionReplayReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementActivationNoLiveCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementActivationNoLiveCloseoutReplayReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementActivationAuditChainCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementActivationAuditChainCloseoutReplayReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitTaskResultReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["directWaitSurfaceAuditReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_task_result_readback"]["noLiveGuardrailsReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["sourceSurfaceId"],
        json!("wait_agent")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["readinessStatus"],
        json!("blocked_canonical_work_graph_write_disabled")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["nextBlocker"],
        json!("canonical_work_graph_write_disabled")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["durableFactSourcePresent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["resultContractReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["verifierReducerReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["replayConsistent"],
        serde_json::Value::Null
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["taskResultContractPlanDecision"],
        json!("task_result_delivery_readback_ready_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["taskResultContractId"],
        json!("subagent_task_result_contract_v1")
    );
    assert_eq!(
        result_value["work_graph_wait_operator_matrix_row"]["terminalDeliverySurface"],
        json!("wait_agent(result_required=true)")
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["decision"],
        json!("wait_task_result_surface_audit_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["auditChainSegmentCount"],
        json!(3)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["auditChainReadySegmentCount"],
        json!(3)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["auditChainReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["auditChainMissingSegmentIds"],
        json!([])
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["auditChainInconsistentSegmentIds"],
        json!([])
    );
    assert_eq!(
        result_value["work_graph_wait_surface_audit_packet"]["operatorMatrixRows"][0]["sourceSurfaceId"],
        json!("wait_agent")
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["decision"],
        json!("work_graph_surface_audit_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["auditChainSegmentCount"],
        json!(5)
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["auditChainReadySegmentCount"],
        json!(5)
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["auditChainReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["auditChainMissingSegmentIds"],
        json!([])
    );
    assert_eq!(
        result_value["work_graph_global_surface_audit_packet"]["auditChainInconsistentSegmentIds"],
        json!([])
    );
    let global_wait_rows =
        result_value["work_graph_global_surface_audit_packet"]["operatorMatrixRows"]
            .as_array()
            .expect("global surface audit should expose operator rows");
    let global_wait_row = global_wait_rows
        .iter()
        .find(|row| row["sourceSurfaceId"] == json!("wait_agent"))
        .expect("global surface audit should include wait_agent row");
    assert_eq!(
        global_wait_row["readinessStatus"],
        json!("blocked_canonical_work_graph_write_disabled")
    );
    assert_eq!(
        global_wait_row["nextBlocker"],
        json!("canonical_work_graph_write_disabled")
    );
    assert_eq!(
        global_wait_row["taskResultContractId"],
        json!("subagent_task_result_contract_v1")
    );
    assert_eq!(
        global_wait_row["terminalDeliverySurface"],
        json!("wait_agent(result_required=true)")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_receipt"]["decision"],
        json!("work_graph_canonical_projection_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_receipt"]["projectedWorkNodeCount"],
        json!(5)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_receipt"]["readProjectionReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_receipt"]["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_replay_consistency_decision"]["decision"],
        json!("work_graph_canonical_projection_replay_consistent_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_receipt"]["decision"],
        json!("work_graph_canonical_projection_closeout_recorded_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_receipt"]["closeoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_receipt"]["noCutoverTerminalReceipt"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_receipt"]["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_replay_consistency_decision"]["decision"],
        json!("work_graph_canonical_projection_closeout_replay_consistent_shadow_no_live_cutover")
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_replay_consistency_decision"]["closeoutReceiptMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_closeout_replay_consistency_decision"]["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_receipt"]["decision"],
        json!(
            "work_graph_canonical_projection_audit_chain_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_receipt"]["auditChainCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_receipt"]["noCutoverTerminalReceipt"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_receipt"]["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision"]
            ["auditChainCloseoutReceiptMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["decision"],
        json!(
            "work_graph_canonical_projection_enablement_operator_review_ready_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["enablementOperatorReviewReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["noLiveEnablementRehearsalReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_packet"]["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_operator_review_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["enablementOperatorReviewPacketMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["noLiveEnablementRehearsalCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision"]
            ["noLiveRehearsalCloseoutMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision"]
            ["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["decision"],
        json!(
            "work_graph_canonical_projection_enablement_audit_chain_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["enablementAuditChainCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"]["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision"]
            ["enablementAuditChainCloseoutMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision"]
            ["enablementAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_precondition_ready_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["activationPreconditionReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["reviewedFlagEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_precondition_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision"]
            ["activationPreconditionOperatorPacketMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["activationNoLiveCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["reviewedFlagEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision"]
            ["activationNoLiveCloseoutMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["activationAuditChainCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["reviewedFlagEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["activationOperatorApprovalReadinessPreflightReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["operatorApprovalRequiredBeforeActivation"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["approvalRecordMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["reviewedFlagRequiredBeforeActivation"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["reviewedFlagEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["reviewedFlagMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["activationOperatorApprovalReadinessPreflightPacketMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["activationOperatorApprovalReadinessPreflightReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["operatorApprovalRequiredBeforeActivation"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["approvalRecordMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["reviewedFlagRequiredBeforeActivation"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["reviewedFlagMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_recorded_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["approvalReviewSideEffectLockCloseoutReady"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["approvalReviewSideEffectsLocked"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["activationOperatorApprovalReadinessPreflightReplayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["approvalRecordMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["reviewedFlagMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["decision"],
        json!(
            "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent_shadow_no_live_cutover"
        )
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["replayConsistent"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["activationApprovalReviewSideEffectLockCloseoutPacketMatchesReadback"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["approvalReviewSideEffectsLocked"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["activationAllowed"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["operatorApprovalRecorded"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["approvalRecordMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["reviewedFlagMutationEnabled"],
        json!(false)
    );
    assert_eq!(
        result_value["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["canonicalWriteEnabled"],
        json!(false)
    );
    assert_eq!(success, None);
}

#[test]
fn multi_agent_v2_wait_agent_wakes_on_any_mailbox_notification() {
    run_large_stack_async_test(
        "multi_agent_v2_wait_agent_wakes_on_any_mailbox_notification",
        multi_agent_v2_wait_agent_wakes_on_any_mailbox_notification_impl(),
    );
}

async fn multi_agent_v2_wait_agent_wakes_on_any_mailbox_notification_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    for task_name in ["worker_a", "worker_b"] {
        SpawnAgentHandlerV2::default()
            .handle(invocation(
                session.clone(),
                turn.clone(),
                "spawn_agent",
                function_payload(json!({
                    "message": format!("boot {task_name}"),
                    "task_name": task_name
                })),
            ))
            .await
            .expect("spawn worker");
    }
    let worker_b_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker_b")
        .await
        .expect("worker_b should resolve");
    let worker_b_path = session
        .services
        .agent_control
        .get_agent_metadata(worker_b_id)
        .expect("worker_b metadata")
        .agent_path
        .expect("worker_b path");

    let wait_task = tokio::spawn({
        let session = session.clone();
        let turn = turn.clone();
        async move {
            WaitAgentHandlerV2::default()
                .handle(invocation(
                    session,
                    turn,
                    "wait_agent",
                    function_payload(json!({"timeout_ms": 10_000})),
                ))
                .await
        }
    });
    tokio::task::yield_now().await;

    session.enqueue_mailbox_communication(InterAgentCommunication::new(
        worker_b_path,
        AgentPath::root(),
        Vec::new(),
        "from worker b".to_string(),
        /*trigger_turn*/ false,
    ));

    let output = wait_task
        .await
        .expect("wait task should join")
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait completed.".to_string(),
            timed_out: false,
        }
    );
    assert_eq!(success, None);
}

#[test]
fn multi_agent_v2_wait_agent_does_not_return_completed_content() {
    run_large_stack_async_test(
        "multi_agent_v2_wait_agent_does_not_return_completed_content",
        multi_agent_v2_wait_agent_does_not_return_completed_content_impl(),
    );
}

async fn multi_agent_v2_wait_agent_does_not_return_completed_content_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);
    let session = Arc::new(session);
    let turn = Arc::new(turn);

    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "boot worker",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn worker");
    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker should resolve");
    let worker_path = session
        .services
        .agent_control
        .get_agent_metadata(agent_id)
        .expect("worker metadata")
        .agent_path
        .expect("worker path");
    let wait_task = tokio::spawn({
        let session = session.clone();
        let turn = turn.clone();
        async move {
            WaitAgentHandlerV2::default()
                .handle(invocation(
                    session,
                    turn,
                    "wait_agent",
                    function_payload(json!({"timeout_ms": 10_000})),
                ))
                .await
        }
    });
    tokio::task::yield_now().await;

    session.enqueue_mailbox_communication(InterAgentCommunication::new(
        worker_path,
        AgentPath::root(),
        Vec::new(),
        "sensitive child output".to_string(),
        /*trigger_turn*/ false,
    ));

    let output = wait_task
        .await
        .expect("wait task should join")
        .expect("wait_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult =
        serde_json::from_str(&content).expect("wait_agent result should be json");
    assert_eq!(
        result,
        crate::tools::handlers::multi_agents_v2::wait::WaitAgentResult {
            message: "Wait completed.".to_string(),
            timed_out: false,
        }
    );
    assert!(!content.contains("sensitive child output"));
    assert_eq!(success, None);
}

#[test]
fn multi_agent_v2_close_agent_accepts_task_name_target() {
    run_large_stack_async_test(
        "multi_agent_v2_close_agent_accepts_task_name_target",
        multi_agent_v2_close_agent_accepts_task_name_target_impl(),
    );
}

async fn multi_agent_v2_close_agent_accepts_task_name_target_impl() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let session = Arc::new(session);
    let turn = Arc::new(turn);
    SpawnAgentHandlerV2::default()
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "spawn_agent",
            function_payload(json!({
                "message": "inspect this repo",
                "task_name": "worker"
            })),
        ))
        .await
        .expect("spawn_agent should succeed");

    let agent_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, "worker")
        .await
        .expect("worker path should resolve");

    let output = CloseAgentHandlerV2
        .handle(invocation(
            session,
            turn,
            "close_agent",
            function_payload(json!({"target": "worker"})),
        ))
        .await
        .expect("close_agent should succeed for v2 task names");
    let (content, success) = expect_text_output(output);
    let result: close_agent::CloseAgentResult =
        serde_json::from_str(&content).expect("close_agent result should be json");
    let result_value: serde_json::Value =
        serde_json::from_str(&content).expect("close_agent result should be json");
    assert_ne!(result.previous_status, AgentStatus::NotFound);
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["attemptedTool"],
        json!("close_agent")
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["toolAllowed"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["observedLane"],
        json!("subagent_lifecycle")
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["laneAllowed"],
        json!(true)
    );
    assert_eq!(
        result_value["work_graph_lifecycle_shadow_decision"]["liveCutoverEnabled"],
        json!(false)
    );
    assert_eq!(success, Some(true));
    assert_eq!(
        manager.agent_control().get_status(agent_id).await,
        AgentStatus::NotFound
    );
}

#[tokio::test]
async fn multi_agent_v2_close_agent_rejects_root_target_and_id() {
    let (mut session, mut turn) = make_session_and_context().await;
    let manager = thread_manager();
    let root = manager
        .start_thread((*turn.config).clone())
        .await
        .expect("root thread should start");
    session.services.agent_control = manager.agent_control();
    session.conversation_id = root.thread_id;
    let mut config = (*turn.config).clone();
    config
        .features
        .enable(Feature::MultiAgentV2)
        .expect("test config should allow feature update");
    turn.config = Arc::new(config);

    let session = Arc::new(session);
    let turn = Arc::new(turn);
    let root_path_error = CloseAgentHandlerV2
        .handle(invocation(
            session.clone(),
            turn.clone(),
            "close_agent",
            function_payload(json!({"target": "/root"})),
        ))
        .await
        .err()
        .expect("close_agent should reject the root path");
    assert_eq!(
        root_path_error,
        FunctionCallError::RespondToModel("root is not a spawned agent".to_string())
    );

    let root_id_error = CloseAgentHandlerV2
        .handle(invocation(
            session,
            turn,
            "close_agent",
            function_payload(json!({"target": root.thread_id.to_string()})),
        ))
        .await
        .err()
        .expect("close_agent should reject the root thread id");
    assert_eq!(
        root_id_error,
        FunctionCallError::RespondToModel("root is not a spawned agent".to_string())
    );
}

#[tokio::test]
async fn close_agent_submits_shutdown_and_returns_previous_status() {
    let (mut session, turn) = make_session_and_context().await;
    let manager = thread_manager();
    session.services.agent_control = manager.agent_control();
    let config = turn.config.as_ref().clone();
    let thread = manager
        .start_thread(config.clone())
        .await
        .expect("start thread");
    let agent_id = thread.thread_id;
    let status_before = manager.agent_control().get_status(agent_id).await;

    let invocation = invocation(
        Arc::new(session),
        Arc::new(turn),
        "close_agent",
        function_payload(json!({"target": agent_id.to_string()})),
    );
    let output = CloseAgentHandler
        .handle(invocation)
        .await
        .expect("close_agent should succeed");
    let (content, success) = expect_text_output(output);
    let result: close_agent::CloseAgentResult =
        serde_json::from_str(&content).expect("close_agent result should be json");
    assert_eq!(result.previous_status, status_before);
    assert_eq!(success, Some(true));

    let ops = manager.captured_ops();
    let submitted_shutdown = ops
        .iter()
        .any(|(id, op)| *id == agent_id && matches!(op, Op::Shutdown));
    assert_eq!(submitted_shutdown, true);

    let status_after = manager.agent_control().get_status(agent_id).await;
    assert_eq!(status_after, AgentStatus::NotFound);
}

#[test]
fn tool_handlers_cascade_close_and_resume_and_keep_explicitly_closed_subtrees_closed() {
    run_large_stack_async_test(
        "tool_handlers_cascade_close_and_resume_and_keep_explicitly_closed_subtrees_closed",
        tool_handlers_cascade_close_and_resume_and_keep_explicitly_closed_subtrees_closed_impl(),
    );
}

async fn tool_handlers_cascade_close_and_resume_and_keep_explicitly_closed_subtrees_closed_impl() {
    let (_session, turn) = make_session_and_context().await;
    let mut config = turn.config.as_ref().clone();
    config.agent_max_depth = 3;
    config
        .features
        .enable(Feature::Sqlite)
        .expect("test config should allow sqlite");
    let state_db = init_state_db(&config).await;
    let manager = ThreadManager::new(
        &config,
        AuthManager::from_auth_for_testing(CodexAuth::from_api_key("dummy")),
        SessionSource::Exec,
        Arc::new(codex_exec_server::EnvironmentManager::default_for_tests()),
        empty_extension_registry(),
        /*analytics_events_client*/ None,
        thread_store_from_config(&config, state_db.clone()),
        state_db.clone(),
        "11111111-1111-4111-8111-111111111111".to_string(),
        /*attestation_provider*/ None,
    );

    let parent = manager
        .start_thread(config.clone())
        .await
        .expect("parent thread should start");
    let parent_thread_id = parent.thread_id;
    let parent_session = parent.thread.codex.session.clone();

    let child_turn = parent_session.new_default_turn().await;
    let child_spawn_output = SpawnAgentHandler::default()
        .handle(invocation(
            parent_session.clone(),
            child_turn,
            "spawn_agent",
            function_payload(json!({"message": "hello child"})),
        ))
        .await
        .expect("child spawn should succeed");
    let (child_content, child_success) = expect_text_output(child_spawn_output);
    let child_result: serde_json::Value =
        serde_json::from_str(&child_content).expect("child spawn result should be json");
    let child_thread_id = parse_agent_id(
        child_result
            .get("agent_id")
            .and_then(serde_json::Value::as_str)
            .expect("child spawn result should include agent_id"),
    );
    assert_eq!(child_success, Some(true));

    let child_thread = manager
        .get_thread(child_thread_id)
        .await
        .expect("child thread should exist");
    let child_session = child_thread.codex.session.clone();
    let grandchild_spawn_output = SpawnAgentHandler::default()
        .handle(invocation(
            child_session.clone(),
            child_session.new_default_turn().await,
            "spawn_agent",
            function_payload(json!({"message": "hello grandchild"})),
        ))
        .await
        .expect("grandchild spawn should succeed");
    let (grandchild_content, grandchild_success) = expect_text_output(grandchild_spawn_output);
    let grandchild_result: serde_json::Value =
        serde_json::from_str(&grandchild_content).expect("grandchild spawn result should be json");
    let grandchild_thread_id = parse_agent_id(
        grandchild_result
            .get("agent_id")
            .and_then(serde_json::Value::as_str)
            .expect("grandchild spawn result should include agent_id"),
    );
    assert_eq!(grandchild_success, Some(true));

    let close_output = CloseAgentHandler
        .handle(invocation(
            parent_session.clone(),
            parent_session.new_default_turn().await,
            "close_agent",
            function_payload(json!({"target": child_thread_id.to_string()})),
        ))
        .await
        .expect("close_agent should close the child subtree");
    let (close_content, close_success) = expect_text_output(close_output);
    let close_result: close_agent::CloseAgentResult =
        serde_json::from_str(&close_content).expect("close_agent result should be json");
    assert_ne!(close_result.previous_status, AgentStatus::NotFound);
    assert_eq!(close_success, Some(true));
    assert_eq!(
        manager.agent_control().get_status(child_thread_id).await,
        AgentStatus::NotFound
    );
    assert_eq!(
        manager
            .agent_control()
            .get_status(grandchild_thread_id)
            .await,
        AgentStatus::NotFound
    );

    let child_resume_output = ResumeAgentHandler
        .handle(invocation(
            parent_session.clone(),
            parent_session.new_default_turn().await,
            "resume_agent",
            function_payload(json!({"id": child_thread_id.to_string()})),
        ))
        .await
        .expect("resume_agent should reopen the child subtree");
    let (child_resume_content, child_resume_success) = expect_text_output(child_resume_output);
    let child_resume_result: resume_agent::ResumeAgentResult =
        serde_json::from_str(&child_resume_content).expect("resume result should be json");
    assert_ne!(child_resume_result.status, AgentStatus::NotFound);
    assert_eq!(child_resume_success, Some(true));
    assert_ne!(
        manager.agent_control().get_status(child_thread_id).await,
        AgentStatus::NotFound
    );
    assert_ne!(
        manager
            .agent_control()
            .get_status(grandchild_thread_id)
            .await,
        AgentStatus::NotFound
    );

    let close_again_output = CloseAgentHandler
        .handle(invocation(
            parent_session.clone(),
            parent_session.new_default_turn().await,
            "close_agent",
            function_payload(json!({"target": child_thread_id.to_string()})),
        ))
        .await
        .expect("close_agent should be repeatable for the child subtree");
    let (close_again_content, close_again_success) = expect_text_output(close_again_output);
    let close_again_result: close_agent::CloseAgentResult =
        serde_json::from_str(&close_again_content)
            .expect("second close_agent result should be json");
    assert_ne!(close_again_result.previous_status, AgentStatus::NotFound);
    assert_eq!(close_again_success, Some(true));
    assert_eq!(
        manager.agent_control().get_status(child_thread_id).await,
        AgentStatus::NotFound
    );
    assert_eq!(
        manager
            .agent_control()
            .get_status(grandchild_thread_id)
            .await,
        AgentStatus::NotFound
    );

    let operator = manager
        .start_thread(config.clone())
        .await
        .expect("operator thread should start");
    let operator_session = operator.thread.codex.session.clone();
    let _ = manager
        .agent_control()
        .shutdown_live_agent(parent_thread_id)
        .await
        .expect("parent shutdown should succeed");
    assert_eq!(
        manager.agent_control().get_status(parent_thread_id).await,
        AgentStatus::NotFound
    );

    let parent_resume_output = ResumeAgentHandler
        .handle(invocation(
            operator_session,
            operator.thread.codex.session.new_default_turn().await,
            "resume_agent",
            function_payload(json!({"id": parent_thread_id.to_string()})),
        ))
        .await
        .expect("resume_agent should reopen the parent thread");
    let (parent_resume_content, parent_resume_success) = expect_text_output(parent_resume_output);
    let parent_resume_result: resume_agent::ResumeAgentResult =
        serde_json::from_str(&parent_resume_content).expect("parent resume result should be json");
    assert_ne!(parent_resume_result.status, AgentStatus::NotFound);
    assert_eq!(parent_resume_success, Some(true));
    assert_ne!(
        manager.agent_control().get_status(parent_thread_id).await,
        AgentStatus::NotFound
    );
    assert_eq!(
        manager.agent_control().get_status(child_thread_id).await,
        AgentStatus::NotFound
    );
    assert_eq!(
        manager
            .agent_control()
            .get_status(grandchild_thread_id)
            .await,
        AgentStatus::NotFound
    );

    let shutdown_report = manager
        .shutdown_all_threads_bounded(Duration::from_secs(5))
        .await;
    assert_eq!(shutdown_report.submit_failed, Vec::<ThreadId>::new());
    assert_eq!(shutdown_report.timed_out, Vec::<ThreadId>::new());
}

#[tokio::test]
async fn build_agent_spawn_config_uses_turn_context_values() {
    fn pick_allowed_sandbox_policy(
        permissions: &crate::config::Permissions,
        base: SandboxPolicy,
        cwd: &std::path::Path,
    ) -> SandboxPolicy {
        let candidates = [
            SandboxPolicy::new_read_only_policy(),
            SandboxPolicy::new_workspace_write_policy(),
            SandboxPolicy::DangerFullAccess,
        ];
        candidates
            .into_iter()
            .find(|candidate| {
                if *candidate == base {
                    return false;
                }
                permissions
                    .can_set_legacy_sandbox_policy(candidate, cwd)
                    .is_ok()
            })
            .unwrap_or(base)
    }

    let (_session, mut turn) = make_session_and_context().await;
    let base_instructions = BaseInstructions {
        text: "base".to_string(),
    };
    turn.developer_instructions = Some("dev".to_string());
    turn.compact_prompt = Some("compact".to_string());
    turn.shell_environment_policy = ShellEnvironmentPolicy {
        use_profile: true,
        ..ShellEnvironmentPolicy::default()
    };
    let temp_dir = tempfile::tempdir().expect("temp dir");
    #[allow(deprecated)]
    {
        turn.cwd = temp_dir.abs();
    }
    turn.codex_linux_sandbox_exe = Some(PathBuf::from("/bin/echo"));
    #[allow(deprecated)]
    let turn_cwd = turn.cwd.clone();
    let sandbox_policy = pick_allowed_sandbox_policy(
        &turn.config.permissions,
        turn.config.legacy_sandbox_policy(),
        turn_cwd.as_path(),
    );
    let file_system_sandbox_policy =
        FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(&sandbox_policy, &turn_cwd);
    let network_sandbox_policy = NetworkSandboxPolicy::from(&sandbox_policy);
    let permission_profile = PermissionProfile::from_runtime_permissions_with_enforcement(
        SandboxEnforcement::from_legacy_sandbox_policy(&sandbox_policy),
        &file_system_sandbox_policy,
        network_sandbox_policy,
    );
    turn.permission_profile = permission_profile.clone();
    turn.approval_policy
        .set(AskForApproval::OnRequest)
        .expect("approval policy set");

    let config = build_agent_spawn_config(&base_instructions, &turn).expect("spawn config");
    let mut expected = (*turn.config).clone();
    expected.base_instructions = Some(base_instructions.text);
    expected.model = Some(turn.model_info.slug.clone());
    expected.model_provider = turn.provider.info().clone();
    expected.model_reasoning_effort = turn.reasoning_effort;
    expected.model_reasoning_summary = Some(turn.reasoning_summary);
    expected.developer_instructions = turn.developer_instructions.clone();
    expected.compact_prompt = turn.compact_prompt.clone();
    expected.permissions.shell_environment_policy = turn.shell_environment_policy.clone();
    expected.codex_linux_sandbox_exe = turn.codex_linux_sandbox_exe.clone();
    #[allow(deprecated)]
    {
        expected.cwd = turn.cwd.clone();
    }
    expected
        .permissions
        .approval_policy
        .set(AskForApproval::OnRequest)
        .expect("approval policy set");
    expected
        .permissions
        .set_permission_profile(permission_profile)
        .expect("permission profile set");
    assert_eq!(config, expected);
}

#[tokio::test]
async fn build_agent_spawn_config_preserves_base_user_instructions() {
    let (_session, mut turn) = make_session_and_context().await;
    let mut base_config = (*turn.config).clone();
    base_config.user_instructions = Some("base-user".to_string());
    turn.user_instructions = Some("resolved-user".to_string());
    turn.config = Arc::new(base_config.clone());
    let base_instructions = BaseInstructions {
        text: "base".to_string(),
    };

    let config = build_agent_spawn_config(&base_instructions, &turn).expect("spawn config");

    assert_eq!(config.user_instructions, base_config.user_instructions);
}

#[tokio::test]
async fn build_agent_resume_config_clears_base_instructions() {
    let (_session, mut turn) = make_session_and_context().await;
    let mut base_config = (*turn.config).clone();
    base_config.base_instructions = Some("caller-base".to_string());
    turn.config = Arc::new(base_config);
    turn.approval_policy
        .set(AskForApproval::OnRequest)
        .expect("approval policy set");

    let config = build_agent_resume_config(&turn, /*child_depth*/ 0).expect("resume config");

    let mut expected = (*turn.config).clone();
    expected.base_instructions = None;
    expected.model = Some(turn.model_info.slug.clone());
    expected.model_provider = turn.provider.info().clone();
    expected.model_reasoning_effort = turn.reasoning_effort;
    expected.model_reasoning_summary = Some(turn.reasoning_summary);
    expected.developer_instructions = turn.developer_instructions.clone();
    expected.compact_prompt = turn.compact_prompt.clone();
    expected.permissions.shell_environment_policy = turn.shell_environment_policy.clone();
    expected.codex_linux_sandbox_exe = turn.codex_linux_sandbox_exe.clone();
    #[allow(deprecated)]
    {
        expected.cwd = turn.cwd.clone();
    }
    expected
        .permissions
        .approval_policy
        .set(AskForApproval::OnRequest)
        .expect("approval policy set");
    expected
        .permissions
        .set_permission_profile(turn.permission_profile())
        .expect("permission profile set");
    assert_eq!(config, expected);
}
