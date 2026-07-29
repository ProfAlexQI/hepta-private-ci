use super::turn_context::TurnEnvironment;
use super::*;
use crate::config::ConfigBuilder;
use crate::config::test_config;
use crate::context::ContextualUserFragment;
use crate::context::ExtensionPromptFragment;
use crate::context::ExtensionPromptSlot;
use crate::context::TurnAborted;
use crate::function_tool::FunctionCallError;
use crate::shell::default_user_shell;
use crate::skills::SkillRenderSideEffects;
use crate::skills::render::SkillMetadataBudget;
use crate::test_support::models_manager_with_provider;
use crate::tools::format_exec_output_str;
use codex_config::ConfigLayerStack;
use codex_config::ConfigLayerStackOrdering;
use codex_config::LoaderOverrides;
use codex_config::NetworkConstraints;
use codex_config::NetworkDomainPermissionToml;
use codex_config::NetworkDomainPermissionsToml;
use codex_config::RequirementSource;
use codex_config::Sourced;
use codex_config::loader::project_trust_key;
use codex_config::types::ToolSuggestDisabledTool;

use codex_features::Feature;
use codex_features::Features;
use codex_login::CodexAuth;
use codex_model_provider_info::ModelProviderInfo;
use codex_models_manager::bundled_models_response;
use codex_models_manager::model_info;
use codex_models_manager::test_support::construct_model_info_offline_for_tests;
use codex_models_manager::test_support::get_model_offline_for_tests;
use codex_protocol::AgentPath;
use codex_protocol::SessionId;
use codex_protocol::ThreadId;
use codex_protocol::account::PlanType as AccountPlanType;
use codex_protocol::config_types::Personality;
use codex_protocol::config_types::ServiceTier;
use codex_protocol::config_types::TrustLevel;
use codex_protocol::exec_output::ExecToolCallOutput;
use codex_protocol::models::ActivePermissionProfile;
use codex_protocol::models::BUILT_IN_PERMISSION_PROFILE_WORKSPACE;
use codex_protocol::models::FileSystemPermissions;
use codex_protocol::models::FunctionCallOutputBody;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::PermissionProfile;
use codex_protocol::models::SandboxEnforcement;
use codex_protocol::permissions::FileSystemAccessMode;
use codex_protocol::permissions::FileSystemPath;
use codex_protocol::permissions::FileSystemSandboxEntry;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_protocol::permissions::FileSystemSpecialPath;
use codex_protocol::protocol::NonSteerableTurnKind;
use codex_protocol::protocol::SandboxPolicy;
use codex_protocol::protocol::TurnEnvironmentSelection;
use codex_protocol::request_permissions::PermissionGrantScope;
use codex_protocol::request_permissions::RequestPermissionProfile;
use tracing::Span;

use crate::goals::ExternalGoalPreviousStatus;
use crate::goals::ExternalGoalSet;
use crate::goals::GoalRuntimeEvent;
use crate::goals::SetGoalRequest;
use crate::rollout::recorder::RolloutRecorder;
use crate::state::ActiveTurn;
use crate::state::TaskKind;
use crate::tasks::SessionTask;
use crate::tasks::SessionTaskContext;
use crate::tasks::UserShellCommandMode;
use crate::tasks::execute_user_shell_command;
use crate::tools::ToolRouter;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::handlers::CreateGoalHandler;
use crate::tools::handlers::ExecCommandHandler;
use crate::tools::handlers::ShellCommandHandler;
use crate::tools::handlers::UpdateGoalHandler;
use crate::tools::registry::ToolExecutor;
use crate::tools::router::ToolCallSource;
use crate::turn_diff_tracker::TurnDiffTracker;
use codex_app_server_protocol::AppInfo;
use codex_app_server_protocol::McpElicitationSchema;
use codex_config::config_toml::ConfigToml;
use codex_config::config_toml::ProjectConfig;
use codex_execpolicy::Decision;
use codex_execpolicy::NetworkRuleProtocol;
use codex_execpolicy::Policy;
use codex_network_proxy::NetworkProxyConfig;
use codex_otel::MetricsClient;
use codex_otel::MetricsConfig;
use codex_otel::THREAD_SKILLS_DESCRIPTION_TRUNCATED_CHARS_METRIC;
use codex_otel::THREAD_SKILLS_ENABLED_TOTAL_METRIC;
use codex_otel::THREAD_SKILLS_KEPT_TOTAL_METRIC;
use codex_otel::THREAD_SKILLS_TRUNCATED_METRIC;
use codex_otel::TelemetryAuthMode;
use codex_protocol::config_types::CollaborationMode;
use codex_protocol::config_types::ModeKind;
use codex_protocol::config_types::Settings;
use codex_protocol::models::BaseInstructions;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseInputItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::CompactedItem;
use codex_protocol::protocol::ConversationAudioParams;
use codex_protocol::protocol::CreditsSnapshot;
use codex_protocol::protocol::GranularApprovalConfig;
use codex_protocol::protocol::InitialHistory;
use codex_protocol::protocol::InterAgentCommunication;
use codex_protocol::protocol::NetworkApprovalProtocol;
use codex_protocol::protocol::RateLimitSnapshot;
use codex_protocol::protocol::RateLimitWindow;
use codex_protocol::protocol::RealtimeAudioFrame;
use codex_protocol::protocol::RealtimeConversationListVoicesResponseEvent;
use codex_protocol::protocol::RealtimeVoice;
use codex_protocol::protocol::RealtimeVoicesList;
use codex_protocol::protocol::ResumedHistory;
use codex_protocol::protocol::RolloutItem;
use codex_protocol::protocol::SkillScope;
use codex_protocol::protocol::Submission;
use codex_protocol::protocol::ThreadGoalStatus;
use codex_protocol::protocol::ThreadHistoryMode;
use codex_protocol::protocol::ThreadRolledBackEvent;
use codex_protocol::protocol::TokenCountEvent;
use codex_protocol::protocol::TokenUsage;
use codex_protocol::protocol::TokenUsageInfo;
use codex_protocol::protocol::TurnAbortedEvent;
use codex_protocol::protocol::TurnCompleteEvent;
use codex_protocol::protocol::TurnStartedEvent;
use codex_protocol::protocol::UserMessageEvent;
use codex_protocol::protocol::W3cTraceContext;
use codex_protocol::request_user_input::RequestUserInputAnswer;
use codex_protocol::request_user_input::RequestUserInputResponse;
use codex_rmcp_client::ElicitationAction;
use core_test_support::PathBufExt;
use core_test_support::PathExt;
use core_test_support::context_snapshot;
use core_test_support::context_snapshot::ContextSnapshotOptions;
use core_test_support::context_snapshot::ContextSnapshotRenderMode;
use core_test_support::responses::ev_assistant_message;
use core_test_support::responses::ev_completed;
use core_test_support::responses::ev_completed_with_tokens;
use core_test_support::responses::ev_function_call;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::mount_sse_once;
use core_test_support::responses::mount_sse_sequence;
use core_test_support::responses::sse;
use core_test_support::responses::start_mock_server;
use core_test_support::test_codex::test_codex;
use core_test_support::test_path_buf;
use core_test_support::tracing::install_test_tracing;
use core_test_support::wait_for_event;
use core_test_support::wait_for_event_match;
use opentelemetry::trace::TraceContextExt;
use opentelemetry::trace::TraceId;
use opentelemetry_sdk::metrics::InMemoryMetricExporter;
use opentelemetry_sdk::metrics::data::AggregatedMetrics;
use opentelemetry_sdk::metrics::data::Metric;
use opentelemetry_sdk::metrics::data::MetricData;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use std::path::Path;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;
use tokio::time::timeout;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use codex_protocol::mcp::CallToolResult as McpCallToolResult;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration as StdDuration;

struct InstructionsTestCase {
    slug: &'static str,
    expects_apply_patch_description: bool,
}

fn user_message(text: &str) -> ResponseItem {
    ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: text.to_string(),
        }],
        phase: None,
    }
}

fn assistant_message(text: &str) -> ResponseItem {
    ResponseItem::Message {
        id: None,
        role: "assistant".to_string(),
        content: vec![ContentItem::OutputText {
            text: text.to_string(),
        }],
        phase: None,
    }
}

fn test_session_telemetry_without_metadata() -> SessionTelemetry {
    let exporter = InMemoryMetricExporter::default();
    let metrics = MetricsClient::new(
        MetricsConfig::in_memory("test", "codex-core", env!("CARGO_PKG_VERSION"), exporter)
            .with_runtime_reader(),
    )
    .expect("in-memory metrics client");
    SessionTelemetry::new(
        ThreadId::new(),
        "gpt-5.4",
        "gpt-5.4",
        /*account_id*/ None,
        /*account_email*/ None,
        /*auth_mode*/ None,
        "test_originator".to_string(),
        /*log_user_prompts*/ false,
        "tty".to_string(),
        SessionSource::Cli,
    )
    .with_metrics_without_metadata_tags(metrics)
}

fn find_metric<'a>(resource_metrics: &'a ResourceMetrics, name: &str) -> &'a Metric {
    for scope_metrics in resource_metrics.scope_metrics() {
        for metric in scope_metrics.metrics() {
            if metric.name() == name {
                return metric;
            }
        }
    }
    panic!("metric {name} missing");
}

fn histogram_sum(resource_metrics: &ResourceMetrics, name: &str) -> u64 {
    let metric = find_metric(resource_metrics, name);
    match metric.data() {
        AggregatedMetrics::F64(data) => match data {
            MetricData::Histogram(histogram) => {
                let points: Vec<_> = histogram.data_points().collect();
                assert_eq!(points.len(), 1);
                points[0].sum().round() as u64
            }
            _ => panic!("unexpected histogram aggregation"),
        },
        _ => panic!("unexpected metric data type"),
    }
}

fn skill_message(text: &str) -> ResponseItem {
    ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: text.to_string(),
        }],
        phase: None,
    }
}

#[tokio::test]
async fn regular_turn_emits_turn_started_without_waiting_for_startup_prewarm() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let (_tx, startup_prewarm_rx) = tokio::sync::oneshot::channel::<()>();
    let handle = tokio::spawn(async move {
        let _ = startup_prewarm_rx.await;
        Ok(test_model_client_session())
    });

    sess.set_session_startup_prewarm(
        crate::session_startup_prewarm::SessionStartupPrewarmHandle::new(
            handle,
            std::time::Instant::now(),
            crate::client::WEBSOCKET_CONNECT_TIMEOUT,
        ),
    )
    .await;
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        crate::tasks::RegularTask::new(),
    )
    .await;

    let first = tokio::time::timeout(std::time::Duration::from_millis(200), rx.recv())
        .await
        .expect("expected turn started event without waiting for startup prewarm")
        .expect("channel open");
    assert!(matches!(
        first.msg,
        EventMsg::TurnStarted(TurnStartedEvent { turn_id, .. }) if turn_id == tc.sub_id
    ));

    sess.abort_all_tasks(TurnAbortReason::Interrupted).await;
}

#[tokio::test]
async fn request_mcp_server_elicitation_auto_accepts_when_auto_deny_is_enabled() {
    let (session, turn_context, rx) = make_session_and_context_with_rx().await;
    session
        .services
        .mcp_connection_manager
        .read()
        .await
        .set_elicitations_auto_deny(/*auto_deny*/ true);

    let requested_schema: McpElicitationSchema = serde_json::from_value(json!({
        "type": "object",
        "properties": {},
    }))
    .expect("schema should deserialize");
    let response = session
        .request_mcp_server_elicitation(
            turn_context.as_ref(),
            RequestId::String("request-1".into()),
            McpServerElicitationRequestParams {
                thread_id: session.conversation_id.to_string(),
                turn_id: Some(turn_context.sub_id.clone()),
                server_name: "codex_apps".to_string(),
                request: McpServerElicitationRequest::Form {
                    meta: None,
                    message: "Allow this request?".to_string(),
                    requested_schema,
                },
            },
        )
        .await;

    assert_eq!(
        response,
        Some(ElicitationResponse {
            action: ElicitationAction::Accept,
            content: Some(json!({})),
            meta: None,
        })
    );
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn interrupting_regular_turn_waiting_on_startup_prewarm_emits_turn_aborted() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let (_tx, startup_prewarm_rx) = tokio::sync::oneshot::channel::<()>();
    let handle = tokio::spawn(async move {
        let _ = startup_prewarm_rx.await;
        Ok(test_model_client_session())
    });

    sess.set_session_startup_prewarm(
        crate::session_startup_prewarm::SessionStartupPrewarmHandle::new(
            handle,
            std::time::Instant::now(),
            crate::client::WEBSOCKET_CONNECT_TIMEOUT,
        ),
    )
    .await;
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        crate::tasks::RegularTask::new(),
    )
    .await;

    let first = tokio::time::timeout(std::time::Duration::from_millis(200), rx.recv())
        .await
        .expect("expected turn started event without waiting for startup prewarm")
        .expect("channel open");
    assert!(matches!(
        first.msg,
        EventMsg::TurnStarted(TurnStartedEvent { turn_id, .. }) if turn_id == tc.sub_id
    ));

    sess.abort_all_tasks(TurnAbortReason::Interrupted).await;

    let second = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
        .await
        .expect("expected turn aborted event")
        .expect("channel open");
    let EventMsg::TurnAborted(TurnAbortedEvent {
        started_at,
        turn_id,
        reason,
        completed_at,
        duration_ms,
    }) = second.msg
    else {
        panic!("expected turn aborted event");
    };
    assert_eq!(turn_id, Some(tc.sub_id.clone()));
    assert_eq!(reason, TurnAbortReason::Interrupted);
    assert!(started_at.is_some());
    assert!(completed_at.is_some());
    assert!(duration_ms.is_some());
}

fn test_model_client_session() -> crate::client::ModelClientSession {
    let thread_id = ThreadId::try_from("00000000-0000-4000-8000-000000000001")
        .expect("test thread id should be valid");
    crate::client::ModelClient::new(
        /*auth_manager*/ None,
        thread_id.into(),
        thread_id,
        /*installation_id*/ "11111111-1111-4111-8111-111111111111".to_string(),
        ModelProviderInfo::create_openai_provider(/* base_url */ /*base_url*/ None),
        codex_protocol::protocol::SessionSource::Exec,
        /*model_verbosity*/ None,
        /*enable_request_compression*/ false,
        /*include_timing_metrics*/ false,
        /*beta_features_header*/ None,
        /*attestation_provider*/ None,
    )
    .new_session()
}

fn developer_input_texts(items: &[ResponseItem]) -> Vec<&str> {
    items
        .iter()
        .filter_map(|item| match item {
            ResponseItem::Message { role, content, .. } if role == "developer" => {
                Some(content.as_slice())
            }
            _ => None,
        })
        .flat_map(|content| content.iter())
        .filter_map(|item| match item {
            ContentItem::InputText { text } => Some(text.as_str()),
            _ => None,
        })
        .collect()
}

fn developer_message_texts(items: &[ResponseItem]) -> Vec<Vec<&str>> {
    items
        .iter()
        .filter_map(|item| match item {
            ResponseItem::Message { role, content, .. } if role == "developer" => {
                Some(content.as_slice())
            }
            _ => None,
        })
        .map(|content| {
            content
                .iter()
                .filter_map(|item| match item {
                    ContentItem::InputText { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn user_input_texts(items: &[ResponseItem]) -> Vec<&str> {
    items
        .iter()
        .filter_map(|item| match item {
            ResponseItem::Message { role, content, .. } if role == "user" => {
                Some(content.as_slice())
            }
            _ => None,
        })
        .flat_map(|content| content.iter())
        .filter_map(|item| match item {
            ContentItem::InputText { text } => Some(text.as_str()),
            _ => None,
        })
        .collect()
}

fn write_project_hooks(dot_codex: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dot_codex)?;
    std::fs::write(
        dot_codex.join("hooks.json"),
        r#"{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "echo hello from hook"
          }
        ]
      }
    ]
  }
}"#,
    )
}

async fn write_project_trust_config(
    codex_home: &Path,
    trusted_projects: &[(&Path, TrustLevel)],
) -> std::io::Result<()> {
    tokio::fs::write(
        codex_home.join(codex_config::CONFIG_TOML_FILE),
        toml::to_string(&ConfigToml {
            projects: Some(
                trusted_projects
                    .iter()
                    .map(|(project, trust_level)| {
                        (
                            project_trust_key(project),
                            ProjectConfig {
                                trust_level: Some(*trust_level),
                            },
                        )
                    })
                    .collect::<std::collections::HashMap<_, _>>(),
            ),
            ..Default::default()
        })
        .expect("serialize config"),
    )
    .await
}

async fn preview_session_start_hooks(
    config: &crate::config::Config,
) -> std::io::Result<Vec<codex_protocol::protocol::HookRunSummary>> {
    let hooks = Hooks::new(HooksConfig {
        feature_enabled: true,
        config_layer_stack: Some(config.config_layer_stack.clone()),
        ..HooksConfig::default()
    });

    Ok(
        hooks.preview_session_start(&codex_hooks::SessionStartRequest {
            session_id: ThreadId::new(),
            cwd: config.cwd.clone(),
            transcript_path: None,
            model: "gpt-5.2".to_string(),
            permission_mode: "default".to_string(),
            source: codex_hooks::SessionStartSource::Startup,
        }),
    )
}

fn test_tool_runtime(session: Arc<Session>, turn_context: Arc<TurnContext>) -> ToolCallRuntime {
    let router = Arc::new(ToolRouter::from_config(
        &turn_context.tools_config,
        crate::tools::router::ToolRouterParams {
            mcp_tools: None,
            deferred_mcp_tools: None,
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: turn_context.dynamic_tools.as_slice(),
        },
    ));
    let tracker = Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new()));
    ToolCallRuntime::new(router, session, turn_context, tracker)
}

fn make_connector(id: &str, name: &str) -> AppInfo {
    AppInfo {
        id: id.to_string(),
        name: name.to_string(),
        description: None,
        logo_url: None,
        logo_url_dark: None,
        distribution_channel: None,
        branding: None,
        app_metadata: None,
        labels: None,
        install_url: None,
        is_accessible: true,
        is_enabled: true,
        plugin_display_names: Vec::new(),
    }
}

#[test]
fn assistant_message_stream_parsers_can_be_seeded_from_output_item_added_text() {
    let mut parsers = AssistantMessageStreamParsers::new(/*plan_mode*/ false);
    let item_id = "msg-1";

    let seeded = parsers.seed_item_text(item_id, "hello <oai-mem-citation>doc");
    let parsed = parsers.parse_delta(item_id, "1</oai-mem-citation> world");
    let tail = parsers.finish_item(item_id);

    assert_eq!(seeded.visible_text, "hello ");
    assert_eq!(seeded.citations, Vec::<String>::new());
    assert_eq!(parsed.visible_text, " world");
    assert_eq!(parsed.citations, vec!["doc1".to_string()]);
    assert_eq!(tail.visible_text, "");
    assert_eq!(tail.citations, Vec::<String>::new());
}

#[test]
fn assistant_message_stream_parsers_seed_buffered_prefix_stays_out_of_finish_tail() {
    let mut parsers = AssistantMessageStreamParsers::new(/*plan_mode*/ false);
    let item_id = "msg-1";

    let seeded = parsers.seed_item_text(item_id, "hello <oai-mem-");
    let parsed = parsers.parse_delta(item_id, "citation>doc</oai-mem-citation> world");
    let tail = parsers.finish_item(item_id);

    assert_eq!(seeded.visible_text, "hello ");
    assert_eq!(seeded.citations, Vec::<String>::new());
    assert_eq!(parsed.visible_text, " world");
    assert_eq!(parsed.citations, vec!["doc".to_string()]);
    assert_eq!(tail.visible_text, "");
    assert_eq!(tail.citations, Vec::<String>::new());
}

#[test]
fn assistant_message_stream_parsers_seed_plan_parser_across_added_and_delta_boundaries() {
    let mut parsers = AssistantMessageStreamParsers::new(/*plan_mode*/ true);
    let item_id = "msg-1";

    let seeded = parsers.seed_item_text(item_id, "Intro\n<proposed");
    let parsed = parsers.parse_delta(item_id, "_plan>\n- step\n</proposed_plan>\nOutro");
    let tail = parsers.finish_item(item_id);

    assert_eq!(seeded.visible_text, "Intro\n");
    assert_eq!(
        seeded.plan_segments,
        vec![ProposedPlanSegment::Normal("Intro\n".to_string())]
    );
    assert_eq!(parsed.visible_text, "Outro");
    assert_eq!(
        parsed.plan_segments,
        vec![
            ProposedPlanSegment::ProposedPlanStart,
            ProposedPlanSegment::ProposedPlanDelta("- step\n".to_string()),
            ProposedPlanSegment::ProposedPlanEnd,
            ProposedPlanSegment::Normal("Outro".to_string()),
        ]
    );
    assert_eq!(tail.visible_text, "");
    assert!(tail.plan_segments.is_empty());
}

#[test]
fn validated_network_policy_amendment_host_allows_normalized_match() {
    let amendment = NetworkPolicyAmendment {
        host: "ExAmPlE.Com.:443".to_string(),
        action: NetworkPolicyRuleAction::Allow,
    };
    let context = NetworkApprovalContext {
        host: "example.com".to_string(),
        protocol: NetworkApprovalProtocol::Https,
    };

    let host = Session::validated_network_policy_amendment_host(&amendment, &context)
        .expect("normalized hosts should match");

    assert_eq!(host, "example.com");
}

#[test]
fn validated_network_policy_amendment_host_rejects_mismatch() {
    let amendment = NetworkPolicyAmendment {
        host: "evil.example.com".to_string(),
        action: NetworkPolicyRuleAction::Deny,
    };
    let context = NetworkApprovalContext {
        host: "api.example.com".to_string(),
        protocol: NetworkApprovalProtocol::Https,
    };

    let err = Session::validated_network_policy_amendment_host(&amendment, &context)
        .expect_err("mismatched hosts should be rejected");

    let message = err.to_string();
    assert!(message.contains("does not match approved host"));
}

#[tokio::test]
async fn start_managed_network_proxy_applies_execpolicy_network_rules() -> anyhow::Result<()> {
    let permission_profile = PermissionProfile::workspace_write();
    let spec = crate::config::NetworkProxySpec::from_config_and_constraints(
        NetworkProxyConfig::default(),
        /*requirements*/ None,
        &permission_profile,
    )?;
    let mut exec_policy = Policy::empty();
    exec_policy.add_network_rule(
        "example.com",
        NetworkRuleProtocol::Https,
        Decision::Allow,
        /*justification*/ None,
    )?;

    let (started_proxy, _) = Session::start_managed_network_proxy(
        &spec,
        &exec_policy,
        &permission_profile,
        /*network_policy_decider*/ None,
        /*blocked_request_observer*/ None,
        /*managed_network_requirements_enabled*/ false,
        crate::config::NetworkProxyAuditMetadata::default(),
    )
    .await?;

    let current_cfg = started_proxy.proxy().current_cfg().await?;
    assert_eq!(
        current_cfg.network.allowed_domains(),
        Some(vec!["example.com".to_string()])
    );
    Ok(())
}

#[tokio::test]
async fn start_managed_network_proxy_ignores_invalid_execpolicy_network_rules() -> anyhow::Result<()>
{
    let permission_profile = PermissionProfile::workspace_write();
    let spec = crate::config::NetworkProxySpec::from_config_and_constraints(
        NetworkProxyConfig::default(),
        Some(NetworkConstraints {
            domains: Some(NetworkDomainPermissionsToml {
                entries: std::collections::BTreeMap::from([(
                    "managed.example.com".to_string(),
                    NetworkDomainPermissionToml::Allow,
                )]),
            }),
            managed_allowed_domains_only: Some(true),
            ..Default::default()
        }),
        &permission_profile,
    )?;
    let mut exec_policy = Policy::empty();
    exec_policy.add_network_rule(
        "example.com",
        NetworkRuleProtocol::Https,
        Decision::Allow,
        /*justification*/ None,
    )?;

    let (started_proxy, _) = Session::start_managed_network_proxy(
        &spec,
        &exec_policy,
        &permission_profile,
        /*network_policy_decider*/ None,
        /*blocked_request_observer*/ None,
        /*managed_network_requirements_enabled*/ false,
        crate::config::NetworkProxyAuditMetadata::default(),
    )
    .await?;

    let current_cfg = started_proxy.proxy().current_cfg().await?;
    assert_eq!(
        current_cfg.network.allowed_domains(),
        Some(vec!["managed.example.com".to_string()])
    );
    Ok(())
}

#[tokio::test]
async fn managed_network_proxy_decider_survives_full_access_start() -> anyhow::Result<()> {
    let full_access_permission_profile = PermissionProfile::Disabled;
    let spec = crate::config::NetworkProxySpec::from_config_and_constraints(
        NetworkProxyConfig::default(),
        Some(NetworkConstraints {
            enabled: Some(true),
            ..Default::default()
        }),
        &full_access_permission_profile,
    )?;
    let exec_policy = Policy::empty();
    let decider_calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let network_policy_decider: Arc<dyn codex_network_proxy::NetworkPolicyDecider> = Arc::new({
        let decider_calls = Arc::clone(&decider_calls);
        move |_request| {
            decider_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async { codex_network_proxy::NetworkDecision::ask("not_allowed") }
        }
    });

    let (started_proxy, _) = Session::start_managed_network_proxy(
        &spec,
        &exec_policy,
        &full_access_permission_profile,
        Some(network_policy_decider),
        /*blocked_request_observer*/ None,
        /*managed_network_requirements_enabled*/ true,
        crate::config::NetworkProxyAuditMetadata::default(),
    )
    .await?;

    let spec = spec.recompute_for_permission_profile(&PermissionProfile::workspace_write())?;
    spec.apply_to_started_proxy(&started_proxy).await?;
    let current_cfg = started_proxy.proxy().current_cfg().await?;
    assert_eq!(current_cfg.network.allowed_domains(), None);

    use tokio::io::AsyncReadExt as _;
    use tokio::io::AsyncWriteExt as _;

    let mut stream = tokio::net::TcpStream::connect(started_proxy.proxy().http_addr()).await?;
    stream
        .write_all(
            b"GET http://example.com/ HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n",
        )
        .await?;
    let mut buffer = [0_u8; 4096];
    let bytes_read = tokio::time::timeout(StdDuration::from_secs(2), stream.read(&mut buffer))
        .await
        .expect("timed out waiting for proxy response")?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    assert!(
        response.starts_with("HTTP/1.1 403 Forbidden"),
        "unexpected proxy response: {response}"
    );
    assert!(
        response.contains("x-proxy-error: blocked-by-allowlist"),
        "unexpected proxy response: {response}"
    );
    assert_eq!(
        decider_calls.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "unexpected proxy response: {response}"
    );
    Ok(())
}

#[tokio::test]
async fn new_turn_refreshes_managed_network_proxy_for_sandbox_change() -> anyhow::Result<()> {
    let (mut session, _turn_context) = make_session_and_context().await;
    let initial_permission_profile = PermissionProfile::workspace_write();
    let initial_policy = SandboxPolicy::new_workspace_write_policy();

    let mut network_config = NetworkProxyConfig::default();
    network_config
        .network
        .set_allowed_domains(vec!["evil.com".to_string()]);
    let requirements = NetworkConstraints {
        domains: Some(NetworkDomainPermissionsToml {
            entries: std::collections::BTreeMap::from([(
                "*.example.com".to_string(),
                NetworkDomainPermissionToml::Allow,
            )]),
        }),
        ..Default::default()
    };
    let spec = crate::config::NetworkProxySpec::from_config_and_constraints(
        network_config,
        Some(requirements),
        &initial_permission_profile,
    )?;
    let (started_proxy, _) = Session::start_managed_network_proxy(
        &spec,
        &Policy::empty(),
        &initial_permission_profile,
        /*network_policy_decider*/ None,
        /*blocked_request_observer*/ None,
        /*managed_network_requirements_enabled*/ false,
        crate::config::NetworkProxyAuditMetadata::default(),
    )
    .await?;
    assert_eq!(
        started_proxy
            .proxy()
            .current_cfg()
            .await?
            .network
            .allowed_domains(),
        Some(vec!["*.example.com".to_string(), "evil.com".to_string()])
    );

    {
        let mut state = session.state.lock().await;
        let mut config = (*state.session_configuration.original_config_do_not_use).clone();
        config.permissions.network = Some(spec);
        let cwd = config.cwd.clone();
        config
            .permissions
            .set_legacy_sandbox_policy(initial_policy.clone(), cwd.as_path())
            .expect("test setup should allow sandbox policy");
        state.session_configuration.original_config_do_not_use = Arc::new(config);
        state
            .session_configuration
            .set_permission_profile_for_tests(initial_permission_profile)
            .expect("test setup should allow permission profile");
    }
    session.services.network_proxy = Some(started_proxy);

    session
        .new_turn_with_sub_id(
            "sandbox-policy-change".to_string(),
            SessionSettingsUpdate {
                sandbox_policy: Some(SandboxPolicy::DangerFullAccess),
                ..Default::default()
            },
        )
        .await?;

    let started_proxy = session
        .services
        .network_proxy
        .as_ref()
        .expect("managed network proxy should be present");
    assert_eq!(
        started_proxy
            .proxy()
            .current_cfg()
            .await?
            .network
            .allowed_domains(),
        Some(vec!["*.example.com".to_string()])
    );

    Ok(())
}

#[tokio::test]
async fn danger_full_access_turns_do_not_expose_managed_network_proxy() -> anyhow::Result<()> {
    let network_spec = crate::config::NetworkProxySpec::from_config_and_constraints(
        NetworkProxyConfig::default(),
        Some(NetworkConstraints {
            enabled: Some(true),
            ..Default::default()
        }),
        &PermissionProfile::Disabled,
    )?;

    let session = make_session_with_config(move |config| {
        let cwd = config.cwd.clone();
        config
            .permissions
            .set_legacy_sandbox_policy(SandboxPolicy::DangerFullAccess, cwd.as_path())
            .expect("test setup should allow sandbox policy");
        config.permissions.network = Some(network_spec);
    })
    .await?;

    let turn_context = session.new_default_turn().await;
    assert!(turn_context.network.is_none());
    Ok(())
}

#[tokio::test]
async fn danger_full_access_tool_attempts_do_not_enforce_managed_network() -> anyhow::Result<()> {
    #[derive(Default)]
    struct ProbeToolRuntime {
        enforce_managed_network: Vec<bool>,
    }

    impl crate::tools::sandboxing::Approvable<()> for ProbeToolRuntime {
        type ApprovalKey = String;

        fn approval_keys(&self, _req: &()) -> Vec<Self::ApprovalKey> {
            vec!["probe".to_string()]
        }

        fn start_approval_async<'a>(
            &'a mut self,
            _req: &'a (),
            _ctx: crate::tools::sandboxing::ApprovalCtx<'a>,
        ) -> futures::future::BoxFuture<'a, ReviewDecision> {
            Box::pin(async { ReviewDecision::Approved })
        }
    }

    impl crate::tools::sandboxing::Sandboxable for ProbeToolRuntime {
        fn sandbox_preference(&self) -> codex_sandboxing::SandboxablePreference {
            codex_sandboxing::SandboxablePreference::Auto
        }
    }

    impl crate::tools::sandboxing::ToolRuntime<(), ()> for ProbeToolRuntime {
        async fn run(
            &mut self,
            _req: &(),
            attempt: &crate::tools::sandboxing::SandboxAttempt<'_>,
            _ctx: &crate::tools::sandboxing::ToolCtx,
        ) -> Result<(), crate::tools::sandboxing::ToolError> {
            self.enforce_managed_network
                .push(attempt.enforce_managed_network);
            Ok(())
        }
    }

    let network_spec = crate::config::NetworkProxySpec::from_config_and_constraints(
        NetworkProxyConfig::default(),
        Some(NetworkConstraints {
            enabled: Some(true),
            ..Default::default()
        }),
        &PermissionProfile::Disabled,
    )?;

    let session = make_session_with_config(move |config| {
        let cwd = config.cwd.clone();
        config
            .permissions
            .set_legacy_sandbox_policy(SandboxPolicy::DangerFullAccess, cwd.as_path())
            .expect("test setup should allow sandbox policy");
        config.permissions.network = Some(network_spec);

        let layers = config
            .config_layer_stack
            .get_layers(
                ConfigLayerStackOrdering::LowestPrecedenceFirst,
                /*include_disabled*/ true,
            )
            .into_iter()
            .cloned()
            .collect();
        let mut requirements = config.config_layer_stack.requirements().clone();
        requirements.network = Some(Sourced::new(
            NetworkConstraints {
                enabled: Some(true),
                ..Default::default()
            },
            RequirementSource::CloudRequirements,
        ));
        let mut requirements_toml = config.config_layer_stack.requirements_toml().clone();
        requirements_toml.network = Some(codex_config::NetworkRequirementsToml {
            enabled: Some(true),
            ..Default::default()
        });
        config.config_layer_stack = ConfigLayerStack::new(layers, requirements, requirements_toml)
            .expect("rebuild config layer stack with network requirements");
    })
    .await?;

    let turn = session.new_default_turn().await;
    assert!(turn.network.is_none());

    let mut orchestrator = crate::tools::orchestrator::ToolOrchestrator::new();
    let mut tool = ProbeToolRuntime::default();
    let tool_ctx = crate::tools::sandboxing::ToolCtx {
        session: Arc::clone(&session),
        turn: Arc::clone(&turn),
        call_id: "probe-call".to_string(),
        tool_name: codex_tools::ToolName::plain("probe"),
    };

    orchestrator
        .run(
            &mut tool,
            &(),
            &tool_ctx,
            turn.as_ref(),
            AskForApproval::Never,
        )
        .await
        .expect("probe runtime should succeed");

    assert_eq!(tool.enforce_managed_network, vec![false]);

    Ok(())
}

#[tokio::test]
async fn workspace_write_turns_continue_to_expose_managed_network_proxy() -> anyhow::Result<()> {
    let permission_profile = PermissionProfile::workspace_write();
    let sandbox_policy = SandboxPolicy::new_workspace_write_policy();
    let network_spec = crate::config::NetworkProxySpec::from_config_and_constraints(
        NetworkProxyConfig::default(),
        Some(NetworkConstraints {
            enabled: Some(true),
            ..Default::default()
        }),
        &permission_profile,
    )?;

    let session = make_session_with_config(move |config| {
        let cwd = config.cwd.clone();
        config
            .permissions
            .set_legacy_sandbox_policy(sandbox_policy, cwd.as_path())
            .expect("test setup should allow sandbox policy");
        config.permissions.network = Some(network_spec);
    })
    .await?;

    let turn_context = session.new_default_turn().await;
    assert!(turn_context.network.is_some());
    Ok(())
}

#[tokio::test]
async fn user_shell_commands_do_not_inherit_managed_network_proxy() -> anyhow::Result<()> {
    let permission_profile = PermissionProfile::workspace_write();
    let sandbox_policy = SandboxPolicy::new_workspace_write_policy();
    let network_spec = crate::config::NetworkProxySpec::from_config_and_constraints(
        NetworkProxyConfig::default(),
        Some(NetworkConstraints {
            enabled: Some(true),
            ..Default::default()
        }),
        &permission_profile,
    )?;

    let (session, rx) = make_session_with_config_and_rx(move |config| {
        let cwd = config.cwd.clone();
        config
            .permissions
            .set_legacy_sandbox_policy(sandbox_policy, cwd.as_path())
            .expect("test setup should allow sandbox policy");
        config.permissions.network = Some(network_spec);
    })
    .await?;

    let turn_context = session.new_default_turn().await;
    assert!(turn_context.network.is_some());

    #[cfg(windows)]
    let command = r#"$val = $env:HTTP_PROXY; if ([string]::IsNullOrEmpty($val)) { $val = 'not-set' } ; [System.Console]::Write($val)"#.to_string();
    #[cfg(not(windows))]
    let command = r#"sh -c "printf '%s' \"${HTTP_PROXY:-not-set}\"""#.to_string();

    execute_user_shell_command(
        Arc::clone(&session),
        turn_context,
        command,
        CancellationToken::new(),
        UserShellCommandMode::StandaloneTurn,
    )
    .await;

    loop {
        let event = rx.recv().await.expect("channel open");
        if let EventMsg::ExecCommandEnd(event) = event.msg {
            assert_eq!(event.exit_code, 0);
            assert_eq!(event.stdout.trim(), "not-set");
            break;
        }
    }

    Ok(())
}

#[tokio::test]
async fn get_base_instructions_no_user_content() {
    let prompt_with_apply_patch_instructions =
        include_str!("../../../prompt_with_apply_patch_instructions.md");
    let models_response = bundled_models_response()
        .unwrap_or_else(|err| panic!("bundled models.json should parse: {err}"));
    let model_info_for_slug = |slug: &str, config: &Config| {
        let model = models_response
            .models
            .iter()
            .find(|candidate| candidate.slug == slug)
            .cloned()
            .unwrap_or_else(|| panic!("model slug {slug} is missing from models.json"));
        model_info::with_config_overrides(model, &config.to_models_manager_config())
    };
    let test_cases = vec![
        InstructionsTestCase {
            slug: "gpt-5.4",
            expects_apply_patch_description: false,
        },
        InstructionsTestCase {
            slug: "gpt-5.4-mini",
            expects_apply_patch_description: false,
        },
        InstructionsTestCase {
            slug: "gpt-5.5",
            expects_apply_patch_description: false,
        },
        InstructionsTestCase {
            slug: "gpt-5.2",
            expects_apply_patch_description: false,
        },
    ];

    let (session, _turn_context) = make_session_and_context().await;
    let config = test_config().await;

    for test_case in test_cases {
        let model_info = model_info_for_slug(test_case.slug, &config);
        if test_case.expects_apply_patch_description {
            assert_eq!(
                model_info.base_instructions.as_str(),
                prompt_with_apply_patch_instructions
            );
        }

        {
            let mut state = session.state.lock().await;
            state.session_configuration.base_instructions = model_info.base_instructions.clone();
        }

        let base_instructions = session.get_base_instructions().await;
        assert_eq!(base_instructions.text, model_info.base_instructions);
    }
}

#[tokio::test]
async fn reload_user_config_layer_updates_effective_apps_config() {
    let (session, _turn_context) = make_session_and_context().await;
    let codex_home = session.codex_home().await;
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    let config_toml_path = codex_home.join(CONFIG_TOML_FILE);
    std::fs::write(
        &config_toml_path,
        "[apps.calendar]\nenabled = false\ndestructive_enabled = false\n",
    )
    .expect("write user config");

    session.reload_user_config_layer().await;

    let config = session.get_config().await;
    let apps_toml = config
        .config_layer_stack
        .effective_config()
        .as_table()
        .and_then(|table| table.get("apps"))
        .cloned()
        .expect("apps table");
    let apps = codex_config::types::AppsConfigToml::deserialize(apps_toml)
        .expect("deserialize apps config");
    let app = apps
        .apps
        .get("calendar")
        .expect("calendar app config exists");

    assert!(!app.enabled);
    assert_eq!(app.destructive_enabled, Some(false));
}

#[tokio::test]
async fn reload_user_config_layer_updates_base_and_selected_profile_layers() {
    let (session, _turn_context) = make_session_and_context().await;
    let codex_home = session.codex_home().await;
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    let base_config_path = codex_home.join(CONFIG_TOML_FILE);
    let profile_config_path = codex_home.join("work.config.toml");
    std::fs::write(
        &base_config_path,
        "model = \"base\"\napproval_policy = \"on-failure\"\n",
    )
    .expect("write base user config");
    std::fs::write(&profile_config_path, "model = \"profile-old\"\n")
        .expect("write profile user config");
    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.to_path_buf())
        .loader_overrides(LoaderOverrides {
            user_config_path: Some(profile_config_path.abs()),
            user_config_profile: Some("work".parse().expect("profile-v2 name")),
            ..LoaderOverrides::without_managed_config_for_tests()
        })
        .build()
        .await
        .expect("load profile config");
    {
        let mut state = session.state.lock().await;
        state.session_configuration.original_config_do_not_use = Arc::new(config);
    }
    std::fs::write(
        &base_config_path,
        "model = \"base\"\napproval_policy = \"never\"\n",
    )
    .expect("update base user config");
    std::fs::write(&profile_config_path, "model = \"profile-new\"\n")
        .expect("update profile user config");

    session.reload_user_config_layer().await;

    let config = session.get_config().await;
    assert_eq!(
        config
            .config_layer_stack
            .get_user_config_file()
            .map(codex_utils_absolute_path::AbsolutePathBuf::as_path),
        Some(profile_config_path.as_path())
    );
    let effective_user_config = config
        .config_layer_stack
        .effective_user_config()
        .expect("merged user config");
    assert_eq!(
        effective_user_config
            .get("model")
            .and_then(toml::Value::as_str),
        Some("profile-new")
    );
    assert_eq!(
        effective_user_config
            .get("approval_policy")
            .and_then(toml::Value::as_str),
        Some("never")
    );
}

#[tokio::test]
async fn reload_user_config_layer_refreshes_hooks() -> anyhow::Result<()> {
    let session = make_session_with_config(|config| {
        config
            .features
            .enable(Feature::CodexHooks)
            .expect("enable Codex hooks");
    })
    .await?;
    let codex_home = session.codex_home().await;
    std::fs::create_dir_all(&codex_home)?;
    let config_toml_path = codex_home.join(CONFIG_TOML_FILE);
    let user_config: codex_config::TomlValue = serde_json::from_value(serde_json::json!({
        "hooks": {
            "SessionStart": [{
                "hooks": [{
                    "type": "command",
                    "command": "python3 /tmp/user.py",
                }],
            }],
        },
    }))?;

    let request = codex_hooks::SessionStartRequest {
        session_id: session.conversation_id,
        cwd: session.get_config().await.cwd.clone(),
        transcript_path: None,
        model: "gpt-5.2".to_string(),
        permission_mode: "default".to_string(),
        source: codex_hooks::SessionStartSource::Startup,
    };
    assert!(session.hooks().preview_session_start(&request).is_empty());

    let config = session.get_config().await;
    let hook_list = codex_hooks::list_hooks(codex_hooks::HooksConfig {
        feature_enabled: true,
        config_layer_stack: Some(
            config
                .config_layer_stack
                .with_user_config(&config_toml_path, user_config.clone()),
        ),
        ..codex_hooks::HooksConfig::default()
    });
    assert_eq!(hook_list.hooks.len(), 1);
    assert_eq!(
        hook_list.hooks[0].trust_status,
        codex_protocol::protocol::HookTrustStatus::Untrusted
    );

    let trusted_user_config: codex_config::TomlValue = serde_json::from_value(serde_json::json!({
        "hooks": {
            "SessionStart": [{
                "hooks": [{
                    "type": "command",
                    "command": "python3 /tmp/user.py",
                }],
            }],
            "state": {
                hook_list.hooks[0].key.clone(): {
                    "trusted_hash": hook_list.hooks[0].current_hash.clone(),
                },
            },
        },
    }))?;
    std::fs::write(&config_toml_path, toml::to_string(&trusted_user_config)?)?;

    session.reload_user_config_layer().await;

    assert_eq!(session.hooks().preview_session_start(&request).len(), 1);
    Ok(())
}

#[tokio::test]
async fn refresh_runtime_config_refreshes_hooks() -> anyhow::Result<()> {
    let (session, _turn_context) = make_session_and_context().await;
    {
        let mut state = session.state.lock().await;
        let mut config = (*state.session_configuration.original_config_do_not_use).clone();
        config
            .features
            .enable(Feature::CodexHooks)
            .expect("enable Codex hooks");
        state.session_configuration.original_config_do_not_use = Arc::new(config);
    }
    let codex_home = session.codex_home().await;
    std::fs::create_dir_all(&codex_home)?;
    let config_toml_path = codex_home.join(CONFIG_TOML_FILE);
    #[derive(serde::Serialize)]
    struct NormalizedHookIdentity {
        event_name: &'static str,
        #[serde(flatten)]
        group: codex_config::MatcherGroup,
    }
    let trusted_hash = {
        let identity = NormalizedHookIdentity {
            event_name: "session_start",
            group: codex_config::MatcherGroup {
                matcher: None,
                hooks: vec![codex_config::HookHandlerConfig::Command {
                    command: "python3 /tmp/user.py".to_string(),
                    command_windows: None,
                    timeout_sec: Some(600),
                    r#async: false,
                    status_message: None,
                }],
            },
        };
        let identity = codex_config::TomlValue::try_from(identity)?;
        codex_config::version_for_toml(&identity)
    };
    let hook_key = format!("{}:session_start:0:0", config_toml_path.display());
    let trusted_user_config: codex_config::TomlValue = serde_json::from_value(serde_json::json!({
        "hooks": {
            "SessionStart": [{
                "hooks": [{
                    "type": "command",
                    "command": "python3 /tmp/user.py",
                }],
            }],
            "state": {
                hook_key: {
                    "trusted_hash": trusted_hash,
                },
            },
        },
    }))?;
    std::fs::write(&config_toml_path, toml::to_string(&trusted_user_config)?)?;

    let request = codex_hooks::SessionStartRequest {
        session_id: session.conversation_id,
        cwd: session.get_config().await.cwd.clone(),
        transcript_path: None,
        model: "gpt-5.2".to_string(),
        permission_mode: "default".to_string(),
        source: codex_hooks::SessionStartSource::Startup,
    };
    assert!(session.hooks().preview_session_start(&request).is_empty());

    let next_config = load_latest_config_for_session(&session).await;
    session.refresh_runtime_config(next_config).await;

    assert_eq!(session.hooks().preview_session_start(&request).len(), 1);
    Ok(())
}

#[tokio::test]
async fn reload_user_config_layer_updates_effective_tool_suggest_config() {
    let (session, _turn_context) = make_session_and_context().await;
    let codex_home = session.codex_home().await;
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    let config_toml_path = codex_home.join(CONFIG_TOML_FILE);
    std::fs::write(
        &config_toml_path,
        r#"[tool_suggest]
disabled_tools = [
  { type = "connector", id = " calendar " },
  { type = "plugin", id = "slack@openai-curated" },
]
"#,
    )
    .expect("write user config");

    session.reload_user_config_layer().await;

    let config = session.get_config().await;
    assert_eq!(
        config.tool_suggest.disabled_tools,
        vec![
            ToolSuggestDisabledTool::connector("calendar"),
            ToolSuggestDisabledTool::plugin("slack@openai-curated"),
        ]
    );
}

#[tokio::test]
async fn refresh_runtime_config_updates_runtime_refreshable_fields_and_keeps_session_static_settings()
 {
    let (session, _turn_context) = make_session_and_context().await;
    let codex_home = session.codex_home().await;
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    std::fs::write(
        codex_home.join(CONFIG_TOML_FILE),
        r#"[apps.calendar]
enabled = false
destructive_enabled = false

[tool_suggest]
disabled_tools = [
  { type = "connector", id = " calendar " },
  { type = "plugin", id = "slack@openai-curated" },
]
"#,
    )
    .expect("write user config");

    let original = session.get_config().await;
    let mut next_config = load_latest_config_for_session(&session).await;
    next_config.model = Some("gpt-5.4".to_string());
    next_config.notify = Some(vec!["echo".to_string()]);

    session.refresh_runtime_config(next_config).await;

    let config = session.get_config().await;
    let apps_toml = config
        .config_layer_stack
        .effective_config()
        .as_table()
        .and_then(|table| table.get("apps"))
        .cloned()
        .expect("apps table");
    let apps = codex_config::types::AppsConfigToml::deserialize(apps_toml)
        .expect("deserialize apps config");
    let app = apps
        .apps
        .get("calendar")
        .expect("calendar app config exists");

    assert!(!app.enabled);
    assert_eq!(app.destructive_enabled, Some(false));
    assert_eq!(config.model, original.model);
    assert_eq!(config.notify, original.notify);
    assert_eq!(
        config.tool_suggest.disabled_tools,
        vec![
            ToolSuggestDisabledTool::connector("calendar"),
            ToolSuggestDisabledTool::plugin("slack@openai-curated"),
        ]
    );
}

#[test]
fn filter_connectors_for_input_skips_duplicate_slug_mentions() {
    let connectors = vec![
        make_connector("one", "Foo Bar"),
        make_connector("two", "Foo-Bar"),
    ];
    let input = vec![user_message("use $foo-bar")];
    let explicitly_enabled_connectors = HashSet::new();
    let skill_name_counts_lower = HashMap::new();

    let selected = filter_connectors_for_input(
        &connectors,
        &input,
        &explicitly_enabled_connectors,
        &skill_name_counts_lower,
    );

    assert_eq!(selected, Vec::new());
}

#[test]
fn filter_connectors_for_input_skips_when_skill_name_conflicts() {
    let connectors = vec![make_connector("one", "Todoist")];
    let input = vec![user_message("use $todoist")];
    let explicitly_enabled_connectors = HashSet::new();
    let skill_name_counts_lower = HashMap::from([("todoist".to_string(), 1)]);

    let selected = filter_connectors_for_input(
        &connectors,
        &input,
        &explicitly_enabled_connectors,
        &skill_name_counts_lower,
    );

    assert_eq!(selected, Vec::new());
}

#[test]
fn filter_connectors_for_input_skips_disabled_connectors() {
    let mut connector = make_connector("calendar", "Calendar");
    connector.is_enabled = false;
    let input = vec![user_message("use $calendar")];
    let explicitly_enabled_connectors = HashSet::new();
    let selected = filter_connectors_for_input(
        &[connector],
        &input,
        &explicitly_enabled_connectors,
        &HashMap::new(),
    );

    assert_eq!(selected, Vec::new());
}

#[test]
fn filter_connectors_for_input_skips_plugin_mentions() {
    let connectors = vec![make_connector("figma", "Figma")];
    let input = vec![user_message("use [@figma](plugin://figma@openai-curated)")];
    let explicitly_enabled_connectors = HashSet::new();
    let selected = filter_connectors_for_input(
        &connectors,
        &input,
        &explicitly_enabled_connectors,
        &HashMap::new(),
    );

    assert_eq!(selected, Vec::new());
}

#[test]
fn collect_explicit_app_ids_from_skill_items_includes_linked_mentions() {
    let connectors = vec![make_connector("calendar", "Calendar")];
    let skill_items = vec![skill_message(
        "<skill>\n<name>demo</name>\n<path>/tmp/skills/demo/SKILL.md</path>\nuse [$calendar](app://calendar)\n</skill>",
    )];

    let connector_ids =
        collect_explicit_app_ids_from_skill_items(&skill_items, &connectors, &HashMap::new());

    assert_eq!(connector_ids, HashSet::from(["calendar".to_string()]));
}

#[test]
fn collect_explicit_app_ids_from_skill_items_resolves_unambiguous_plain_mentions() {
    let connectors = vec![make_connector("calendar", "Calendar")];
    let skill_items = vec![skill_message(
        "<skill>\n<name>demo</name>\n<path>/tmp/skills/demo/SKILL.md</path>\nuse $calendar\n</skill>",
    )];

    let connector_ids =
        collect_explicit_app_ids_from_skill_items(&skill_items, &connectors, &HashMap::new());

    assert_eq!(connector_ids, HashSet::from(["calendar".to_string()]));
}

#[test]
fn collect_explicit_app_ids_from_skill_items_skips_plain_mentions_with_skill_conflicts() {
    let connectors = vec![make_connector("calendar", "Calendar")];
    let skill_items = vec![skill_message(
        "<skill>\n<name>demo</name>\n<path>/tmp/skills/demo/SKILL.md</path>\nuse $calendar\n</skill>",
    )];
    let skill_name_counts_lower = HashMap::from([("calendar".to_string(), 1)]);

    let connector_ids = collect_explicit_app_ids_from_skill_items(
        &skill_items,
        &connectors,
        &skill_name_counts_lower,
    );

    assert_eq!(connector_ids, HashSet::<String>::new());
}

#[tokio::test]
async fn reconstruct_history_matches_live_compactions() {
    let (session, turn_context) = make_session_and_context().await;
    let (rollout_items, expected) = sample_rollout(&session, &turn_context).await;

    let reconstruction_turn = session.new_default_turn().await;
    let reconstructed = session
        .reconstruct_history_from_rollout(reconstruction_turn.as_ref(), &rollout_items)
        .await;

    assert_eq!(expected, reconstructed.history);
}

#[tokio::test]
async fn reconstruct_history_uses_replacement_history_verbatim() {
    let (session, turn_context) = make_session_and_context().await;
    let summary_item = ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "summary".to_string(),
        }],
        phase: None,
    };
    let replacement_history = vec![
        summary_item.clone(),
        ResponseItem::Message {
            id: None,
            role: "developer".to_string(),
            content: vec![ContentItem::InputText {
                text: "stale developer instructions".to_string(),
            }],
            phase: None,
        },
    ];
    let rollout_items = vec![RolloutItem::Compacted(CompactedItem {
        message: String::new(),
        replacement_history: Some(replacement_history.clone()),
    })];

    let reconstructed = session
        .reconstruct_history_from_rollout(&turn_context, &rollout_items)
        .await;

    assert_eq!(reconstructed.history, replacement_history);
}

#[tokio::test]
async fn record_initial_history_reconstructs_resumed_transcript() {
    let (session, turn_context) = make_session_and_context().await;
    let (rollout_items, expected) = sample_rollout(&session, &turn_context).await;

    session
        .record_initial_history(InitialHistory::Resumed(ResumedHistory {
            conversation_id: ThreadId::default(),
            history: rollout_items,
            rollout_path: Some(PathBuf::from("/tmp/resume.jsonl")),
        }))
        .await;

    let history = session.state.lock().await.clone_history();
    assert_eq!(expected, history.raw_items());
}

#[tokio::test]
async fn record_initial_history_new_defers_initial_context_until_first_turn() {
    let (session, _turn_context) = make_session_and_context().await;

    session.record_initial_history(InitialHistory::New).await;

    let history = session.clone_history().await;
    assert_eq!(history.raw_items().to_vec(), Vec::<ResponseItem>::new());
    assert!(session.reference_context_item().await.is_none());
    assert_eq!(session.previous_turn_settings().await, None);
}

#[tokio::test]
async fn refresh_mcp_config_replaces_managed_requirements_without_changing_model() {
    let (session, _turn_context) = make_session_and_context().await;
    let server = serde_json::from_value::<McpServerConfig>(json!({
        "url": "https://example.com/mcp",
        "enabled": true
    }))
    .expect("valid test MCP server");
    let requirement = serde_json::from_value::<codex_config::McpServerRequirement>(json!({
        "identity": { "url": "https://example.com/mcp" }
    }))
    .expect("valid managed MCP requirement");
    let plugin_requirements = std::collections::BTreeMap::from([(
        "example-plugin".to_string(),
        codex_config::PluginRequirementsToml {
            mcp_servers: Some(std::collections::BTreeMap::from([(
                "beta".to_string(),
                requirement,
            )])),
        },
    )]);

    let original_model = session.get_config().await.model.clone();
    let mut next_config = session.get_config().await.as_ref().clone();
    next_config.model = Some("must-not-replace-active-model".to_string());
    next_config.mcp_servers = codex_config::Constrained::normalized(
        HashMap::from([("beta".to_string(), server.clone())]),
        |mut servers: HashMap<String, McpServerConfig>| {
            servers.retain(|name, _| name == "beta");
            servers
        },
    )
    .expect("valid refreshed MCP constraints");
    let mut requirements = next_config.config_layer_stack.requirements().clone();
    requirements.plugins = Some(Sourced::new(
        plugin_requirements.clone(),
        RequirementSource::LegacyManagedConfigTomlFromMdm,
    ));
    let mut requirements_toml = next_config.config_layer_stack.requirements_toml().clone();
    requirements_toml.plugins = Some(plugin_requirements.clone());
    let layers = next_config
        .config_layer_stack
        .get_layers(
            ConfigLayerStackOrdering::LowestPrecedenceFirst,
            /*include_disabled*/ true,
        )
        .into_iter()
        .cloned()
        .collect();
    next_config.config_layer_stack = ConfigLayerStack::new(layers, requirements, requirements_toml)
        .expect("managed MCP and plugin requirements");
    let refresh_config = McpServerRefreshConfig {
        mcp_servers: json!({}),
        mcp_oauth_credentials_store_mode: serde_json::to_value(
            next_config.mcp_oauth_credentials_store_mode,
        )
        .expect("serialize store mode"),
        elicitation_authority: None,
    };

    session
        .refresh_mcp_config(next_config, refresh_config)
        .await;

    let config = session.get_config().await;
    assert_eq!(config.model, original_model);
    let mut managed_servers = config.mcp_servers.clone();
    managed_servers
        .set(HashMap::from([
            ("alpha".to_string(), server.clone()),
            ("beta".to_string(), server.clone()),
        ]))
        .expect("apply refreshed managed MCP constraints");
    assert_eq!(
        managed_servers.get(),
        &HashMap::from([("beta".to_string(), server.clone())])
    );
    assert_eq!(
        config
            .config_layer_stack
            .requirements()
            .plugins
            .as_ref()
            .map(|requirements| &requirements.value),
        Some(&plugin_requirements)
    );
}

#[tokio::test]
async fn resumed_history_injects_initial_context_on_first_context_update_only() {
    let (session, turn_context) = make_session_and_context().await;
    let (rollout_items, mut expected) = sample_rollout(&session, &turn_context).await;

    session
        .record_initial_history(InitialHistory::Resumed(ResumedHistory {
            conversation_id: ThreadId::default(),
            history: rollout_items,
            rollout_path: Some(PathBuf::from("/tmp/resume.jsonl")),
        }))
        .await;

    let history_before_seed = session.state.lock().await.clone_history();
    assert_eq!(expected, history_before_seed.raw_items());

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    expected.extend(session.build_initial_context(&turn_context).await);
    let history_after_seed = session.clone_history().await;
    assert_eq!(expected, history_after_seed.raw_items());

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let history_after_second_seed = session.clone_history().await;
    assert_eq!(
        history_after_seed.raw_items(),
        history_after_second_seed.raw_items()
    );
}

#[tokio::test]
async fn record_initial_history_seeds_token_info_from_rollout() {
    let (session, turn_context) = make_session_and_context().await;
    let (mut rollout_items, _expected) = sample_rollout(&session, &turn_context).await;

    let info1 = TokenUsageInfo {
        total_token_usage: TokenUsage {
            input_tokens: 10,
            cached_input_tokens: 0,
            output_tokens: 20,
            reasoning_output_tokens: 0,
            total_tokens: 30,
        },
        last_token_usage: TokenUsage {
            input_tokens: 3,
            cached_input_tokens: 0,
            output_tokens: 4,
            reasoning_output_tokens: 0,
            total_tokens: 7,
        },
        model_context_window: Some(1_000),
    };
    let info2 = TokenUsageInfo {
        total_token_usage: TokenUsage {
            input_tokens: 100,
            cached_input_tokens: 50,
            output_tokens: 200,
            reasoning_output_tokens: 25,
            total_tokens: 375,
        },
        last_token_usage: TokenUsage {
            input_tokens: 10,
            cached_input_tokens: 0,
            output_tokens: 20,
            reasoning_output_tokens: 5,
            total_tokens: 35,
        },
        model_context_window: Some(2_000),
    };

    rollout_items.push(RolloutItem::EventMsg(EventMsg::TokenCount(
        TokenCountEvent {
            info: Some(info1),
            rate_limits: None,
        },
    )));
    rollout_items.push(RolloutItem::EventMsg(EventMsg::TokenCount(
        TokenCountEvent {
            info: None,
            rate_limits: None,
        },
    )));
    rollout_items.push(RolloutItem::EventMsg(EventMsg::TokenCount(
        TokenCountEvent {
            info: Some(info2.clone()),
            rate_limits: None,
        },
    )));
    rollout_items.push(RolloutItem::EventMsg(EventMsg::TokenCount(
        TokenCountEvent {
            info: None,
            rate_limits: None,
        },
    )));

    session
        .record_initial_history(InitialHistory::Resumed(ResumedHistory {
            conversation_id: ThreadId::default(),
            history: rollout_items,
            rollout_path: Some(PathBuf::from("/tmp/resume.jsonl")),
        }))
        .await;

    let actual = session.state.lock().await.token_info();
    assert_eq!(actual, Some(info2));
}

#[tokio::test]
async fn recompute_token_usage_uses_session_base_instructions() {
    let (session, turn_context) = make_session_and_context().await;

    let override_instructions = "SESSION_OVERRIDE_INSTRUCTIONS_ONLY".repeat(120);
    {
        let mut state = session.state.lock().await;
        state.session_configuration.base_instructions = override_instructions.clone();
    }

    let item = user_message("hello");
    session
        .record_into_history(std::slice::from_ref(&item), &turn_context)
        .await;

    let history = session.clone_history().await;
    let session_base_instructions = BaseInstructions {
        text: override_instructions,
    };
    let expected_tokens = history
        .estimate_token_count_with_base_instructions(&session_base_instructions)
        .expect("estimate with session base instructions");
    let model_estimated_tokens = history
        .estimate_token_count(&turn_context)
        .expect("estimate with model instructions");
    assert_ne!(expected_tokens, model_estimated_tokens);

    session.recompute_token_usage(&turn_context).await;

    let actual_tokens = session
        .state
        .lock()
        .await
        .token_info()
        .expect("token info")
        .last_token_usage
        .total_tokens;
    assert_eq!(actual_tokens, expected_tokens.max(0));
}

#[tokio::test]
async fn recompute_token_usage_updates_model_context_window() {
    let (session, mut turn_context) = make_session_and_context().await;

    {
        let mut state = session.state.lock().await;
        state.set_token_info(Some(TokenUsageInfo {
            total_token_usage: TokenUsage::default(),
            last_token_usage: TokenUsage::default(),
            model_context_window: Some(258_400),
        }));
    }

    turn_context.model_info.context_window = Some(128_000);
    turn_context.model_info.effective_context_window_percent = 100;

    session.recompute_token_usage(&turn_context).await;

    let actual = session.state.lock().await.token_info().expect("token info");
    assert_eq!(actual.model_context_window, Some(128_000));
}

#[tokio::test]
async fn record_token_usage_info_notifies_extension_contributors() {
    struct SessionTokenUsageMarker;
    struct ThreadTokenUsageMarker;

    #[derive(Debug, PartialEq, Eq)]
    struct RecordedTokenUsage {
        session_level_id: String,
        thread_level_id: String,
        turn_level_id: String,
        token_usage: TokenUsageInfo,
        saw_session_store: bool,
        saw_thread_store: bool,
    }

    struct TokenUsageRecorder {
        records: Arc<std::sync::Mutex<Vec<RecordedTokenUsage>>>,
    }

    impl codex_extension_api::TokenUsageContributor for TokenUsageRecorder {
        fn on_token_usage(
            &self,
            session_store: &codex_extension_api::ExtensionData,
            thread_store: &codex_extension_api::ExtensionData,
            turn_store: &codex_extension_api::ExtensionData,
            token_usage: &TokenUsageInfo,
        ) {
            self.records
                .lock()
                .expect("token usage records lock")
                .push(RecordedTokenUsage {
                    session_level_id: session_store.level_id().to_string(),
                    thread_level_id: thread_store.level_id().to_string(),
                    turn_level_id: turn_store.level_id().to_string(),
                    token_usage: token_usage.clone(),
                    saw_session_store: session_store.get::<SessionTokenUsageMarker>().is_some(),
                    saw_thread_store: thread_store.get::<ThreadTokenUsageMarker>().is_some(),
                });
        }
    }

    let (mut session, turn_context) = make_session_and_context().await;
    let records = Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut builder = codex_extension_api::ExtensionRegistryBuilder::<crate::config::Config>::new();
    builder.token_usage_contributor(Arc::new(TokenUsageRecorder {
        records: Arc::clone(&records),
    }));
    session.services.extensions = Arc::new(builder.build());
    session
        .services
        .session_extension_data
        .insert(SessionTokenUsageMarker);
    session
        .services
        .thread_extension_data
        .insert(ThreadTokenUsageMarker);

    let first_usage = TokenUsage {
        input_tokens: 10,
        cached_input_tokens: 2,
        output_tokens: 20,
        reasoning_output_tokens: 3,
        total_tokens: 33,
    };
    let second_usage = TokenUsage {
        input_tokens: 7,
        cached_input_tokens: 1,
        output_tokens: 8,
        reasoning_output_tokens: 5,
        total_tokens: 20,
    };

    session
        .record_token_usage_info(&turn_context, Some(&first_usage))
        .await;
    session
        .record_token_usage_info(&turn_context, Some(&second_usage))
        .await;

    let mut expected_total_usage = first_usage.clone();
    expected_total_usage.add_assign(&second_usage);
    let expected = vec![
        RecordedTokenUsage {
            session_level_id: session.session_id().to_string(),
            thread_level_id: session.conversation_id.to_string(),
            turn_level_id: turn_context.sub_id.clone(),
            token_usage: TokenUsageInfo {
                total_token_usage: first_usage.clone(),
                last_token_usage: first_usage,
                model_context_window: turn_context.model_context_window(),
            },
            saw_session_store: true,
            saw_thread_store: true,
        },
        RecordedTokenUsage {
            session_level_id: session.session_id().to_string(),
            thread_level_id: session.conversation_id.to_string(),
            turn_level_id: turn_context.sub_id.clone(),
            token_usage: TokenUsageInfo {
                total_token_usage: expected_total_usage,
                last_token_usage: second_usage,
                model_context_window: turn_context.model_context_window(),
            },
            saw_session_store: true,
            saw_thread_store: true,
        },
    ];
    let actual = records
        .lock()
        .expect("token usage records lock")
        .drain(..)
        .collect::<Vec<_>>();
    assert_eq!(expected, actual);
}

#[tokio::test]
async fn config_change_contributor_observes_effective_config_changes() {
    struct SessionConfigMarker;
    struct ThreadConfigMarker;

    #[derive(Debug, PartialEq)]
    struct RecordedConfigChange {
        previous_model: Option<String>,
        new_model: Option<String>,
        previous_disabled_tools: Vec<ToolSuggestDisabledTool>,
        new_disabled_tools: Vec<ToolSuggestDisabledTool>,
        saw_session_store: bool,
        saw_thread_store: bool,
    }

    struct ConfigRecorder {
        records: Arc<std::sync::Mutex<Vec<RecordedConfigChange>>>,
    }

    impl codex_extension_api::ConfigContributor<crate::config::Config> for ConfigRecorder {
        fn on_config_changed(
            &self,
            session_store: &codex_extension_api::ExtensionData,
            thread_store: &codex_extension_api::ExtensionData,
            previous_config: &crate::config::Config,
            new_config: &crate::config::Config,
        ) {
            self.records
                .lock()
                .expect("config change records lock")
                .push(RecordedConfigChange {
                    previous_model: previous_config.model.clone(),
                    new_model: new_config.model.clone(),
                    previous_disabled_tools: previous_config.tool_suggest.disabled_tools.clone(),
                    new_disabled_tools: new_config.tool_suggest.disabled_tools.clone(),
                    saw_session_store: session_store.get::<SessionConfigMarker>().is_some(),
                    saw_thread_store: thread_store.get::<ThreadConfigMarker>().is_some(),
                });
        }
    }

    let (mut session, _turn_context) = make_session_and_context().await;
    let records = Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut builder = codex_extension_api::ExtensionRegistryBuilder::<crate::config::Config>::new();
    builder.config_contributor(Arc::new(ConfigRecorder {
        records: Arc::clone(&records),
    }));
    session.services.extensions = Arc::new(builder.build());
    session
        .services
        .session_extension_data
        .insert(SessionConfigMarker);
    session
        .services
        .thread_extension_data
        .insert(ThreadConfigMarker);

    let original_model = session.collaboration_mode().await.model().to_string();
    let original_disabled_tools = session
        .get_config()
        .await
        .tool_suggest
        .disabled_tools
        .clone();
    let next_model = if original_model == "gpt-5.4" {
        "gpt-5.2"
    } else {
        "gpt-5.4"
    };
    let collaboration_mode = session.collaboration_mode().await.with_updates(
        Some(next_model.to_string()),
        /*effort*/ None,
        /*developer_instructions*/ None,
    );
    session
        .update_settings(SessionSettingsUpdate {
            collaboration_mode: Some(collaboration_mode),
            ..Default::default()
        })
        .await
        .expect("update settings");

    let codex_home = session.codex_home().await;
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    std::fs::write(
        codex_home.join(CONFIG_TOML_FILE),
        r#"[tool_suggest]
disabled_tools = [
  { type = "connector", id = " calendar " },
  { type = "plugin", id = "slack@openai-curated" },
]
"#,
    )
    .expect("write user config");
    let next_config = load_latest_config_for_session(&session).await;
    session.refresh_runtime_config(next_config).await;

    let expected_disabled_tools = vec![
        ToolSuggestDisabledTool::connector("calendar"),
        ToolSuggestDisabledTool::plugin("slack@openai-curated"),
    ];
    let expected = vec![
        RecordedConfigChange {
            previous_model: Some(original_model),
            new_model: Some(next_model.to_string()),
            previous_disabled_tools: original_disabled_tools.clone(),
            new_disabled_tools: original_disabled_tools.clone(),
            saw_session_store: true,
            saw_thread_store: true,
        },
        RecordedConfigChange {
            previous_model: Some(next_model.to_string()),
            new_model: Some(next_model.to_string()),
            previous_disabled_tools: original_disabled_tools,
            new_disabled_tools: expected_disabled_tools,
            saw_session_store: true,
            saw_thread_store: true,
        },
    ];
    let actual = records
        .lock()
        .expect("config change records lock")
        .drain(..)
        .collect::<Vec<_>>();
    assert_eq!(expected, actual);
}

#[tokio::test]
async fn record_initial_history_reconstructs_forked_transcript() {
    let (session, turn_context) = make_session_and_context().await;
    let (rollout_items, expected) = sample_rollout(&session, &turn_context).await;

    session
        .record_initial_history(InitialHistory::Forked(rollout_items))
        .await;

    let history = session.state.lock().await.clone_history();
    assert_eq!(expected, history.raw_items());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn session_configured_reports_permission_profile_for_external_sandbox() -> anyhow::Result<()>
{
    let server = start_mock_server().await;
    let sandbox_policy = SandboxPolicy::ExternalSandbox {
        network_access: codex_protocol::protocol::NetworkAccess::Restricted,
    };
    let permission_profile = PermissionProfile::External {
        network: NetworkSandboxPolicy::Restricted,
    };
    let expected_permission_profile = permission_profile.clone();
    let mut builder = test_codex().with_config(move |config| {
        config
            .permissions
            .set_permission_profile(permission_profile.clone())
            .expect("set permission profile");
        config
            .set_legacy_sandbox_policy(sandbox_policy)
            .expect("set sandbox policy");
    });

    let test = builder.build(&server).await?;

    assert_eq!(
        test.session_configured.permission_profile, expected_permission_profile,
        "ExternalSandbox is represented explicitly instead of as a lossy root-write profile"
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn session_permission_profile_rebinds_runtime_workspace_roots() -> anyhow::Result<()> {
    let codex_home = tempfile::TempDir::new()?;
    let cwd = tempfile::TempDir::new()?;
    let old_root = test_path_buf("/workspace/old").abs();
    let new_root = test_path_buf("/workspace/new").abs();
    let config = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .harness_overrides(crate::config::ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            default_permissions: Some(BUILT_IN_PERMISSION_PROFILE_WORKSPACE.to_string()),
            additional_writable_roots: vec![old_root.to_path_buf()],
            ..Default::default()
        })
        .build()
        .await?;

    let session_permission_profile_state = session_permission_profile_state_from_config(&config)?;
    let stored_file_system_policy = session_permission_profile_state
        .permission_profile()
        .file_system_sandbox_policy();
    assert!(
        !stored_file_system_policy
            .can_write_path_with_cwd(old_root.as_path(), config.cwd.as_path()),
        "session permission profile state should keep runtime workspace roots symbolic"
    );

    let mut session_configuration = make_session_configuration_for_tests().await;
    session_configuration.cwd = config.cwd.clone();
    session_configuration.workspace_roots = config.workspace_roots.clone();
    session_configuration.permission_profile_state = session_permission_profile_state;

    let initial_policy = session_configuration.file_system_sandbox_policy();
    assert!(initial_policy.can_write_path_with_cwd(old_root.as_path(), config.cwd.as_path()));

    let updated = session_configuration.apply(&SessionSettingsUpdate {
        workspace_roots: Some(vec![new_root.clone()]),
        ..Default::default()
    })?;
    let updated_policy = updated.file_system_sandbox_policy();
    assert!(updated_policy.can_write_path_with_cwd(new_root.as_path(), updated.cwd.as_path()));
    assert!(!updated_policy.can_write_path_with_cwd(old_root.as_path(), updated.cwd.as_path()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn fork_startup_context_then_first_turn_diff_snapshot() -> anyhow::Result<()> {
    let server = start_mock_server().await;
    mount_sse_once(
        &server,
        sse(vec![ev_response_created("resp-1"), ev_completed("resp-1")]),
    )
    .await;
    let first_forked_request = mount_sse_once(
        &server,
        sse(vec![ev_response_created("resp-2"), ev_completed("resp-2")]),
    )
    .await;

    let mut builder = test_codex().with_config(|config| {
        config.permissions.approval_policy =
            codex_config::Constrained::allow_any(AskForApproval::OnRequest);
    });
    let initial = builder.build(&server).await?;
    let rollout_path = initial
        .session_configured
        .rollout_path
        .clone()
        .expect("rollout path");

    initial
        .codex
        .submit(Op::UserInput {
            environments: None,
            items: vec![UserInput::Text {
                text: "fork seed".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        })
        .await?;
    wait_for_event(&initial.codex, |ev| matches!(ev, EventMsg::TurnComplete(_))).await;
    // Forking reads the persisted rollout JSONL, so force the completed source turn to disk
    // before snapshotting from it.
    initial.codex.ensure_rollout_materialized().await;
    initial
        .codex
        .flush_rollout()
        .await
        .expect("source rollout should flush before fork");

    let mut fork_config = initial.config.clone();
    fork_config.permissions.approval_policy =
        codex_config::Constrained::allow_any(AskForApproval::UnlessTrusted);
    let forked = initial
        .thread_manager
        .fork_thread(
            usize::MAX,
            fork_config.clone(),
            rollout_path,
            /*thread_source*/ None,
            /*persist_extended_history*/ false,
            /*parent_trace*/ None,
        )
        .await?;

    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Plan,
        settings: Settings {
            model: forked.session_configured.model.clone(),
            reasoning_effort: None,
            developer_instructions: Some("Fork turn collaboration instructions.".to_string()),
        },
    };
    forked
        .thread
        .submit(Op::OverrideTurnContext {
            cwd: None,
            approval_policy: Some(AskForApproval::Never),
            approvals_reviewer: None,
            sandbox_policy: None,
            permission_profile: None,
            windows_sandbox_level: None,
            model: None,
            effort: None,
            summary: None,
            service_tier: None,
            collaboration_mode: Some(collaboration_mode),
            personality: None,
        })
        .await?;

    forked
        .thread
        .submit(Op::UserInput {
            environments: None,
            items: vec![UserInput::Text {
                text: "after fork".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        })
        .await?;
    wait_for_event(&forked.thread, |ev| matches!(ev, EventMsg::TurnComplete(_))).await;

    let request = first_forked_request.single_request();
    let snapshot = context_snapshot::format_labeled_requests_snapshot(
        "First request after fork when startup preserves the parent baseline, the fork changes approval policy, and the first forked turn enters plan mode.",
        &[("First Forked Turn Request", &request)],
        &ContextSnapshotOptions::default()
            .render_mode(ContextSnapshotRenderMode::KindWithTextPrefix { max_chars: 96 })
            .strip_capability_instructions()
            .strip_agents_md_user_context(),
    );

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("snapshots");
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        insta::assert_snapshot!(
            "codex_core__codex_tests__fork_startup_context_then_first_turn_diff",
            snapshot
        );
    });

    Ok(())
}

#[tokio::test]
async fn record_initial_history_forked_hydrates_previous_turn_settings() {
    let (session, turn_context) = make_session_and_context().await;
    let previous_model = "forked-rollout-model";
    let previous_context_item = TurnContextItem {
        turn_id: Some(turn_context.sub_id.clone()),
        trace_id: turn_context.trace_id.clone(),
        #[allow(deprecated)]
        cwd: turn_context.cwd.to_path_buf(),
        current_date: turn_context.current_date.clone(),
        timezone: turn_context.timezone.clone(),
        approval_policy: turn_context.approval_policy.value(),
        sandbox_policy: turn_context.sandbox_policy(),
        permission_profile: None,
        network: None,
        file_system_sandbox_policy: None,
        model: previous_model.to_string(),
        personality: turn_context.personality,
        collaboration_mode: Some(turn_context.collaboration_mode.clone()),
        realtime_active: Some(turn_context.realtime_active),
        effort: turn_context.reasoning_effort,
        summary: turn_context.reasoning_summary,
        user_instructions: None,
        developer_instructions: None,
        final_output_json_schema: None,
        truncation_policy: Some(turn_context.truncation_policy),
        context_manifest: None,
    };
    let turn_id = previous_context_item
        .turn_id
        .clone()
        .expect("turn context should have turn_id");
    let rollout_items = vec![
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: turn_id.clone(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::EventMsg(EventMsg::UserMessage(
            codex_protocol::protocol::UserMessageEvent {
                message: "forked seed".to_string(),
                images: None,
                local_images: Vec::new(),
                text_elements: Vec::new(),
                ..Default::default()
            },
        )),
        RolloutItem::TurnContext(previous_context_item.clone()),
        RolloutItem::EventMsg(EventMsg::TurnComplete(
            codex_protocol::protocol::TurnCompleteEvent {
                started_at: None,
                error: None,
                turn_id,
                last_agent_message: None,
                completed_at: None,
                duration_ms: None,
                time_to_first_token_ms: None,
            },
        )),
    ];

    session
        .record_initial_history(InitialHistory::Forked(rollout_items))
        .await;

    let history = session.clone_history().await;
    assert_eq!(
        session.previous_turn_settings().await,
        Some(PreviousTurnSettings {
            model: previous_model.to_string(),
            realtime_active: Some(turn_context.realtime_active),
        })
    );
    assert_eq!(history.raw_items(), &[]);
    assert_eq!(
        serde_json::to_value(session.reference_context_item().await)
            .expect("serialize fork reference context item"),
        serde_json::to_value(Some(previous_context_item))
            .expect("serialize expected reference context item")
    );
}

#[tokio::test]
async fn thread_rollback_drops_last_turn_from_history() {
    let (mut sess, tc, rx) = make_session_and_context_with_rx().await;
    let rollout_path = attach_thread_persistence(
        Arc::get_mut(&mut sess).expect("session should not have additional references"),
    )
    .await;

    let initial_context = sess.build_initial_context(tc.as_ref()).await;
    let turn_1 = vec![
        user_message("turn 1 user"),
        assistant_message("turn 1 assistant"),
    ];
    let turn_2 = vec![
        user_message("turn 2 user"),
        assistant_message("turn 2 assistant"),
    ];
    let mut full_history = Vec::new();
    full_history.extend(initial_context.clone());
    full_history.extend(turn_1.clone());
    full_history.extend(turn_2);
    sess.replace_history(full_history.clone(), Some(tc.to_turn_context_item()))
        .await;
    let rollout_items: Vec<RolloutItem> = full_history
        .into_iter()
        .map(RolloutItem::ResponseItem)
        .collect();
    sess.persist_rollout_items(&rollout_items).await;
    sess.set_previous_turn_settings(Some(PreviousTurnSettings {
        model: "stale-model".to_string(),
        realtime_active: Some(tc.realtime_active),
    }))
    .await;
    {
        let mut state = sess.state.lock().await;
        state.set_reference_context_item(Some(tc.to_turn_context_item()));
    }

    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 1).await;

    let rollback_event = wait_for_thread_rolled_back(&rx).await;
    assert_eq!(rollback_event.num_turns, 1);

    let mut expected = Vec::new();
    expected.extend(initial_context);
    expected.extend(turn_1);

    let history = sess.clone_history().await;
    assert_eq!(expected, history.raw_items());
    assert_eq!(sess.previous_turn_settings().await, None);
    assert!(sess.reference_context_item().await.is_none());

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    assert!(resumed.history.iter().any(|item| {
        matches!(
            item,
            RolloutItem::EventMsg(EventMsg::ThreadRolledBack(rollback))
            if rollback.num_turns == 1
        )
    }));
}

#[tokio::test]
async fn thread_rollback_clears_history_when_num_turns_exceeds_existing_turns() {
    let (mut sess, tc, rx) = make_session_and_context_with_rx().await;
    attach_thread_persistence(
        Arc::get_mut(&mut sess).expect("session should not have additional references"),
    )
    .await;

    let initial_context = sess.build_initial_context(tc.as_ref()).await;
    let turn_1 = vec![user_message("turn 1 user")];
    let mut full_history = Vec::new();
    full_history.extend(initial_context.clone());
    full_history.extend(turn_1);
    sess.replace_history(full_history.clone(), Some(tc.to_turn_context_item()))
        .await;
    let rollout_items: Vec<RolloutItem> = full_history
        .into_iter()
        .map(RolloutItem::ResponseItem)
        .collect();
    sess.persist_rollout_items(&rollout_items).await;

    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 99).await;

    let rollback_event = wait_for_thread_rolled_back(&rx).await;
    assert_eq!(rollback_event.num_turns, 99);

    let history = sess.clone_history().await;
    assert_eq!(initial_context, history.raw_items());
}

#[tokio::test]
async fn thread_rollback_fails_without_persisted_thread_history() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;

    let initial_context = sess.build_initial_context(tc.as_ref()).await;
    sess.record_into_history(&initial_context, tc.as_ref())
        .await;

    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 1).await;

    let error_event = wait_for_thread_rollback_failed(&rx).await;
    assert_eq!(
        error_event.message,
        "thread rollback requires persisted thread history"
    );
    assert_eq!(
        error_event.codex_error_info,
        Some(CodexErrorInfo::ThreadRollbackFailed)
    );
    assert_eq!(sess.clone_history().await.raw_items(), initial_context);
}

#[tokio::test]
async fn thread_rollback_recomputes_previous_turn_settings_and_reference_context_from_replay() {
    let (mut sess, tc, rx) = make_session_and_context_with_rx().await;
    attach_thread_persistence(
        Arc::get_mut(&mut sess).expect("session should not have additional references"),
    )
    .await;

    let first_context_item = tc.to_turn_context_item();
    let first_turn_id = first_context_item
        .turn_id
        .clone()
        .expect("turn context should have turn_id");
    let mut rolled_back_context_item = first_context_item.clone();
    rolled_back_context_item.turn_id = Some("rolled-back-turn".to_string());
    rolled_back_context_item.model = "rolled-back-model".to_string();
    let rolled_back_turn_id = rolled_back_context_item
        .turn_id
        .clone()
        .expect("turn context should have turn_id");
    let turn_one_user = user_message("turn 1 user");
    let turn_one_assistant = assistant_message("turn 1 assistant");
    let turn_two_user = user_message("turn 2 user");
    let turn_two_assistant = assistant_message("turn 2 assistant");

    sess.persist_rollout_items(&[
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: first_turn_id.clone(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::EventMsg(EventMsg::UserMessage(
            codex_protocol::protocol::UserMessageEvent {
                message: "turn 1 user".to_string(),
                images: None,
                local_images: Vec::new(),
                text_elements: Vec::new(),
                ..Default::default()
            },
        )),
        RolloutItem::TurnContext(first_context_item.clone()),
        RolloutItem::ResponseItem(turn_one_user.clone()),
        RolloutItem::ResponseItem(turn_one_assistant.clone()),
        RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
            started_at: None,
            error: None,
            turn_id: first_turn_id,
            last_agent_message: None,
            completed_at: None,
            duration_ms: None,
            time_to_first_token_ms: None,
        })),
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: rolled_back_turn_id.clone(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::EventMsg(EventMsg::UserMessage(
            codex_protocol::protocol::UserMessageEvent {
                message: "turn 2 user".to_string(),
                images: None,
                local_images: Vec::new(),
                text_elements: Vec::new(),
                ..Default::default()
            },
        )),
        RolloutItem::TurnContext(rolled_back_context_item),
        RolloutItem::ResponseItem(turn_two_user),
        RolloutItem::ResponseItem(turn_two_assistant),
        RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
            started_at: None,
            error: None,
            turn_id: rolled_back_turn_id,
            last_agent_message: None,
            completed_at: None,
            duration_ms: None,
            time_to_first_token_ms: None,
        })),
    ])
    .await;
    sess.replace_history(
        vec![assistant_message("stale history")],
        Some(first_context_item.clone()),
    )
    .await;
    sess.set_previous_turn_settings(Some(PreviousTurnSettings {
        model: "stale-model".to_string(),
        realtime_active: None,
    }))
    .await;

    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 1).await;
    let rollback_event = wait_for_thread_rolled_back(&rx).await;
    assert_eq!(rollback_event.num_turns, 1);

    assert_eq!(
        sess.clone_history().await.raw_items(),
        vec![turn_one_user, turn_one_assistant]
    );
    assert_eq!(
        sess.previous_turn_settings().await,
        Some(PreviousTurnSettings {
            model: tc.model_info.slug.clone(),
            realtime_active: Some(tc.realtime_active),
        })
    );
    assert_eq!(
        serde_json::to_value(sess.reference_context_item().await)
            .expect("serialize replay reference context item"),
        serde_json::to_value(Some(first_context_item))
            .expect("serialize expected reference context item")
    );
}

#[tokio::test]
async fn thread_rollback_restores_cleared_reference_context_item_after_compaction() {
    let (mut sess, tc, rx) = make_session_and_context_with_rx().await;
    attach_thread_persistence(
        Arc::get_mut(&mut sess).expect("session should not have additional references"),
    )
    .await;

    let first_context_item = tc.to_turn_context_item();
    let first_turn_id = first_context_item
        .turn_id
        .clone()
        .expect("turn context should have turn_id");
    let compact_turn_id = "compact-turn".to_string();
    let rolled_back_turn_id = "rolled-back-turn".to_string();
    let compacted_history = vec![
        user_message("turn 1 user"),
        user_message("summary after compaction"),
    ];

    sess.persist_rollout_items(&[
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: first_turn_id.clone(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::EventMsg(EventMsg::UserMessage(UserMessageEvent {
            message: "turn 1 user".to_string(),
            images: None,
            local_images: Vec::new(),
            text_elements: Vec::new(),
            ..Default::default()
        })),
        RolloutItem::TurnContext(first_context_item.clone()),
        RolloutItem::ResponseItem(user_message("turn 1 user")),
        RolloutItem::ResponseItem(assistant_message("turn 1 assistant")),
        RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
            started_at: None,
            error: None,
            turn_id: first_turn_id,
            last_agent_message: None,
            completed_at: None,
            duration_ms: None,
            time_to_first_token_ms: None,
        })),
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: compact_turn_id.clone(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::Compacted(CompactedItem {
            message: "summary after compaction".to_string(),
            replacement_history: Some(compacted_history.clone()),
        }),
        RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
            started_at: None,
            error: None,
            turn_id: compact_turn_id,
            last_agent_message: None,
            completed_at: None,
            duration_ms: None,
            time_to_first_token_ms: None,
        })),
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: rolled_back_turn_id.clone(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::EventMsg(EventMsg::UserMessage(UserMessageEvent {
            message: "turn 2 user".to_string(),
            images: None,
            local_images: Vec::new(),
            text_elements: Vec::new(),
            ..Default::default()
        })),
        RolloutItem::TurnContext(TurnContextItem {
            turn_id: Some(rolled_back_turn_id.clone()),
            model: "rolled-back-model".to_string(),
            ..first_context_item.clone()
        }),
        RolloutItem::ResponseItem(user_message("turn 2 user")),
        RolloutItem::ResponseItem(assistant_message("turn 2 assistant")),
        RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
            started_at: None,
            error: None,
            turn_id: rolled_back_turn_id,
            last_agent_message: None,
            completed_at: None,
            duration_ms: None,
            time_to_first_token_ms: None,
        })),
    ])
    .await;
    sess.replace_history(
        vec![assistant_message("stale history")],
        Some(first_context_item),
    )
    .await;

    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 1).await;
    let rollback_event = wait_for_thread_rolled_back(&rx).await;
    assert_eq!(rollback_event.num_turns, 1);

    assert_eq!(sess.clone_history().await.raw_items(), compacted_history);
    assert!(sess.reference_context_item().await.is_none());
}

#[tokio::test]
async fn thread_rollback_persists_marker_and_replays_cumulatively() {
    let (mut sess, tc, rx) = make_session_and_context_with_rx().await;
    let rollout_path = attach_thread_persistence(
        Arc::get_mut(&mut sess).expect("session should not have additional references"),
    )
    .await;
    let turn_context_item = tc.to_turn_context_item();

    sess.persist_rollout_items(&[
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: "turn-1".to_string(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::EventMsg(EventMsg::UserMessage(UserMessageEvent {
            message: "turn 1 user".to_string(),
            images: None,
            local_images: Vec::new(),
            text_elements: Vec::new(),
            ..Default::default()
        })),
        RolloutItem::TurnContext(turn_context_item.clone()),
        RolloutItem::ResponseItem(user_message("turn 1 user")),
        RolloutItem::ResponseItem(assistant_message("turn 1 assistant")),
        RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
            started_at: None,
            error: None,
            turn_id: "turn-1".to_string(),
            last_agent_message: None,
            completed_at: None,
            duration_ms: None,
            time_to_first_token_ms: None,
        })),
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: "turn-2".to_string(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::EventMsg(EventMsg::UserMessage(UserMessageEvent {
            message: "turn 2 user".to_string(),
            images: None,
            local_images: Vec::new(),
            text_elements: Vec::new(),
            ..Default::default()
        })),
        RolloutItem::TurnContext(turn_context_item.clone()),
        RolloutItem::ResponseItem(user_message("turn 2 user")),
        RolloutItem::ResponseItem(assistant_message("turn 2 assistant")),
        RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
            started_at: None,
            error: None,
            turn_id: "turn-2".to_string(),
            last_agent_message: None,
            completed_at: None,
            duration_ms: None,
            time_to_first_token_ms: None,
        })),
        RolloutItem::EventMsg(EventMsg::TurnStarted(
            codex_protocol::protocol::TurnStartedEvent {
                turn_id: "turn-3".to_string(),
                started_at: None,
                model_context_window: Some(128_000),
                collaboration_mode_kind: ModeKind::Default,
            },
        )),
        RolloutItem::EventMsg(EventMsg::UserMessage(UserMessageEvent {
            message: "turn 3 user".to_string(),
            images: None,
            local_images: Vec::new(),
            text_elements: Vec::new(),
            ..Default::default()
        })),
        RolloutItem::TurnContext(turn_context_item),
        RolloutItem::ResponseItem(user_message("turn 3 user")),
        RolloutItem::ResponseItem(assistant_message("turn 3 assistant")),
        RolloutItem::EventMsg(EventMsg::TurnComplete(TurnCompleteEvent {
            started_at: None,
            error: None,
            turn_id: "turn-3".to_string(),
            last_agent_message: None,
            completed_at: None,
            duration_ms: None,
            time_to_first_token_ms: None,
        })),
    ])
    .await;

    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 1).await;
    let first_rollback = wait_for_thread_rolled_back(&rx).await;
    assert_eq!(first_rollback.num_turns, 1);
    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 1).await;
    let second_rollback = wait_for_thread_rolled_back(&rx).await;
    assert_eq!(second_rollback.num_turns, 1);

    assert_eq!(
        sess.clone_history().await.raw_items(),
        vec![
            user_message("turn 1 user"),
            assistant_message("turn 1 assistant")
        ]
    );

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    let rollback_markers = resumed
        .history
        .iter()
        .filter(|item| matches!(item, RolloutItem::EventMsg(EventMsg::ThreadRolledBack(_))))
        .count();
    assert_eq!(rollback_markers, 2);
}

#[tokio::test]
async fn thread_rollback_fails_when_turn_in_progress() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;

    let initial_context = sess.build_initial_context(tc.as_ref()).await;
    sess.record_into_history(&initial_context, tc.as_ref())
        .await;

    *sess.active_turn.lock().await = Some(crate::state::ActiveTurn::default());
    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 1).await;

    let error_event = wait_for_thread_rollback_failed(&rx).await;
    assert_eq!(
        error_event.codex_error_info,
        Some(CodexErrorInfo::ThreadRollbackFailed)
    );

    let history = sess.clone_history().await;
    assert_eq!(initial_context, history.raw_items());
}

#[tokio::test]
async fn thread_rollback_fails_when_num_turns_is_zero() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;

    let initial_context = sess.build_initial_context(tc.as_ref()).await;
    sess.record_into_history(&initial_context, tc.as_ref())
        .await;

    handlers::thread_rollback(&sess, "sub-1".to_string(), /*num_turns*/ 0).await;

    let error_event = wait_for_thread_rollback_failed(&rx).await;
    assert_eq!(error_event.message, "num_turns must be >= 1");
    assert_eq!(
        error_event.codex_error_info,
        Some(CodexErrorInfo::ThreadRollbackFailed)
    );

    let history = sess.clone_history().await;
    assert_eq!(initial_context, history.raw_items());
}

#[tokio::test]
async fn set_rate_limits_retains_previous_credits() {
    let codex_home = tempfile::tempdir().expect("create temp dir");
    let config = build_test_config(codex_home.path()).await;
    let config = Arc::new(config);
    let model = get_model_offline_for_tests(config.model.as_deref());
    let model_info =
        construct_model_info_offline_for_tests(model.as_str(), &config.to_models_manager_config());
    let reasoning_effort = config.model_reasoning_effort;
    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model,
            reasoning_effort,
            developer_instructions: None,
        },
    };
    let session_configuration = SessionConfiguration {
        provider: config.model_provider.clone(),
        collaboration_mode,
        model_reasoning_summary: config.model_reasoning_summary,
        developer_instructions: config.developer_instructions.clone(),
        user_instructions: config.user_instructions.clone(),
        service_tier: None,
        personality: config.personality,
        history_mode: ThreadHistoryMode::Legacy,
        base_instructions: config
            .base_instructions
            .clone()
            .unwrap_or_else(|| model_info.get_model_instructions(config.personality)),
        compact_prompt: config.compact_prompt.clone(),
        approval_policy: config.permissions.approval_policy.clone(),
        approvals_reviewer: config.approvals_reviewer,
        permission_profile_state: config.permissions.permission_profile_state().clone(),
        windows_sandbox_level: WindowsSandboxLevel::from_config(&config),
        cwd: config.cwd.clone(),
        workspace_roots: config.workspace_roots.clone(),
        codex_home: config.codex_home.clone(),
        thread_name: None,
        environments: Vec::new(),
        original_config_do_not_use: Arc::clone(&config),
        metrics_service_name: None,
        app_server_client_name: None,
        app_server_client_version: None,
        session_source: SessionSource::Exec,
        thread_source: None,
        dynamic_tools: Vec::new(),
        persist_extended_history: false,
        inherited_shell_snapshot: None,
        user_shell_override: None,
    };

    let mut state = SessionState::new(session_configuration);
    let initial = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 10.0,
            window_minutes: Some(15),
            resets_at: Some(1_700),
        }),
        secondary: None,
        credits: Some(CreditsSnapshot {
            has_credits: true,
            unlimited: false,
            balance: Some("10.00".to_string()),
        }),
        plan_type: Some(codex_protocol::account::PlanType::Plus),
        rate_limit_reached_type: None,
    };
    state.set_rate_limits(initial.clone());

    let update = RateLimitSnapshot {
        limit_id: Some("codex_other".to_string()),
        limit_name: Some("codex_other".to_string()),
        primary: Some(RateLimitWindow {
            used_percent: 40.0,
            window_minutes: Some(30),
            resets_at: Some(1_800),
        }),
        secondary: Some(RateLimitWindow {
            used_percent: 5.0,
            window_minutes: Some(60),
            resets_at: Some(1_900),
        }),
        credits: None,
        plan_type: None,
        rate_limit_reached_type: None,
    };
    state.set_rate_limits(update.clone());

    assert_eq!(
        state.latest_rate_limits,
        Some(RateLimitSnapshot {
            limit_id: Some("codex_other".to_string()),
            limit_name: Some("codex_other".to_string()),
            primary: update.primary.clone(),
            secondary: update.secondary,
            credits: initial.credits,
            plan_type: initial.plan_type,
            rate_limit_reached_type: None,
        })
    );
}

#[tokio::test]
async fn set_rate_limits_updates_plan_type_when_present() {
    let codex_home = tempfile::tempdir().expect("create temp dir");
    let config = build_test_config(codex_home.path()).await;
    let config = Arc::new(config);
    let model = get_model_offline_for_tests(config.model.as_deref());
    let model_info =
        construct_model_info_offline_for_tests(model.as_str(), &config.to_models_manager_config());
    let reasoning_effort = config.model_reasoning_effort;
    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model,
            reasoning_effort,
            developer_instructions: None,
        },
    };
    let session_configuration = SessionConfiguration {
        provider: config.model_provider.clone(),
        collaboration_mode,
        model_reasoning_summary: config.model_reasoning_summary,
        developer_instructions: config.developer_instructions.clone(),
        user_instructions: config.user_instructions.clone(),
        service_tier: None,
        personality: config.personality,
        history_mode: ThreadHistoryMode::Legacy,
        base_instructions: config
            .base_instructions
            .clone()
            .unwrap_or_else(|| model_info.get_model_instructions(config.personality)),
        compact_prompt: config.compact_prompt.clone(),
        approval_policy: config.permissions.approval_policy.clone(),
        approvals_reviewer: config.approvals_reviewer,
        permission_profile_state: config.permissions.permission_profile_state().clone(),
        windows_sandbox_level: WindowsSandboxLevel::from_config(&config),
        cwd: config.cwd.clone(),
        workspace_roots: config.workspace_roots.clone(),
        codex_home: config.codex_home.clone(),
        thread_name: None,
        environments: Vec::new(),
        original_config_do_not_use: Arc::clone(&config),
        metrics_service_name: None,
        app_server_client_name: None,
        app_server_client_version: None,
        session_source: SessionSource::Exec,
        thread_source: None,
        dynamic_tools: Vec::new(),
        persist_extended_history: false,
        inherited_shell_snapshot: None,
        user_shell_override: None,
    };

    let mut state = SessionState::new(session_configuration);
    let initial = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 15.0,
            window_minutes: Some(20),
            resets_at: Some(1_600),
        }),
        secondary: Some(RateLimitWindow {
            used_percent: 5.0,
            window_minutes: Some(45),
            resets_at: Some(1_650),
        }),
        credits: Some(CreditsSnapshot {
            has_credits: true,
            unlimited: false,
            balance: Some("15.00".to_string()),
        }),
        plan_type: Some(codex_protocol::account::PlanType::Plus),
        rate_limit_reached_type: None,
    };
    state.set_rate_limits(initial.clone());

    let update = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 35.0,
            window_minutes: Some(25),
            resets_at: Some(1_700),
        }),
        secondary: None,
        credits: None,
        plan_type: Some(codex_protocol::account::PlanType::Pro),
        rate_limit_reached_type: None,
    };
    state.set_rate_limits(update.clone());

    assert_eq!(
        state.latest_rate_limits,
        Some(RateLimitSnapshot {
            limit_id: Some("codex".to_string()),
            limit_name: None,
            primary: update.primary,
            secondary: update.secondary,
            credits: initial.credits,
            plan_type: update.plan_type,
            rate_limit_reached_type: None,
        })
    );
}

#[test]
fn prefers_structured_content_when_present() {
    let ctr = McpCallToolResult {
        // Content present but should be ignored because structured_content is set.
        content: vec![text_block("ignored")],
        is_error: None,
        structured_content: Some(json!({
            "ok": true,
            "value": 42
        })),
        meta: None,
    };

    let got = ctr.into_function_call_output_payload();
    let expected = FunctionCallOutputPayload {
        body: FunctionCallOutputBody::Text(
            serde_json::to_string(&json!({
                "ok": true,
                "value": 42
            }))
            .unwrap(),
        ),
        success: Some(true),
    };

    assert_eq!(expected, got);
}

#[tokio::test]
async fn includes_timed_out_message() {
    let exec = ExecToolCallOutput {
        exit_code: 0,
        stdout: StreamOutput::new(String::new()),
        stderr: StreamOutput::new(String::new()),
        aggregated_output: StreamOutput::new("Command output".to_string()),
        duration: StdDuration::from_secs(1),
        timed_out: true,
    };
    let (_, turn_context) = make_session_and_context().await;

    let out = format_exec_output_str(&exec, turn_context.truncation_policy);

    assert_eq!(
        out,
        "command timed out after 1000 milliseconds\nCommand output"
    );
}

#[tokio::test]
async fn turn_context_with_model_updates_model_fields() {
    let (session, mut turn_context) = make_session_and_context().await;
    turn_context.reasoning_effort = Some(ReasoningEffortConfig::Minimal);
    let updated = turn_context
        .with_model("gpt-5.4".to_string(), &session.services.models_manager)
        .await;
    let expected_model_info = session
        .services
        .models_manager
        .get_model_info(
            "gpt-5.4",
            &updated.config.as_ref().to_models_manager_config(),
        )
        .await;

    assert_eq!(updated.config.model.as_deref(), Some("gpt-5.4"));
    assert_eq!(updated.collaboration_mode.model(), "gpt-5.4");
    assert_eq!(updated.model_info, expected_model_info);
    assert_eq!(
        updated.reasoning_effort,
        Some(ReasoningEffortConfig::Medium)
    );
    assert_eq!(
        updated.collaboration_mode.reasoning_effort(),
        Some(ReasoningEffortConfig::Medium)
    );
    assert_eq!(
        updated.config.model_reasoning_effort,
        Some(ReasoningEffortConfig::Medium)
    );
    assert_eq!(
        updated.truncation_policy,
        expected_model_info.truncation_policy.into()
    );
}

#[test]
fn falls_back_to_content_when_structured_is_null() {
    let ctr = McpCallToolResult {
        content: vec![text_block("hello"), text_block("world")],
        is_error: None,
        structured_content: Some(serde_json::Value::Null),
        meta: None,
    };

    let got = ctr.into_function_call_output_payload();
    let expected = FunctionCallOutputPayload {
        body: FunctionCallOutputBody::Text(
            serde_json::to_string(&vec![text_block("hello"), text_block("world")]).unwrap(),
        ),
        success: Some(true),
    };

    assert_eq!(expected, got);
}

#[test]
fn success_flag_reflects_is_error_true() {
    let ctr = McpCallToolResult {
        content: vec![text_block("unused")],
        is_error: Some(true),
        structured_content: Some(json!({ "message": "bad" })),
        meta: None,
    };

    let got = ctr.into_function_call_output_payload();
    let expected = FunctionCallOutputPayload {
        body: FunctionCallOutputBody::Text(
            serde_json::to_string(&json!({ "message": "bad" })).unwrap(),
        ),
        success: Some(false),
    };

    assert_eq!(expected, got);
}

#[test]
fn success_flag_true_with_no_error_and_content_used() {
    let ctr = McpCallToolResult {
        content: vec![text_block("alpha")],
        is_error: Some(false),
        structured_content: None,
        meta: None,
    };

    let got = ctr.into_function_call_output_payload();
    let expected = FunctionCallOutputPayload {
        body: FunctionCallOutputBody::Text(
            serde_json::to_string(&vec![text_block("alpha")]).unwrap(),
        ),
        success: Some(true),
    };

    assert_eq!(expected, got);
}

async fn wait_for_thread_rolled_back(rx: &async_channel::Receiver<Event>) -> ThreadRolledBackEvent {
    let deadline = StdDuration::from_secs(2);
    let start = std::time::Instant::now();
    loop {
        let remaining = deadline.saturating_sub(start.elapsed());
        let evt = tokio::time::timeout(remaining, rx.recv())
            .await
            .expect("timeout waiting for event")
            .expect("event");
        match evt.msg {
            EventMsg::ThreadRolledBack(payload) => return payload,
            _ => continue,
        }
    }
}

async fn wait_for_thread_rollback_failed(rx: &async_channel::Receiver<Event>) -> ErrorEvent {
    let deadline = StdDuration::from_secs(2);
    let start = std::time::Instant::now();
    loop {
        let remaining = deadline.saturating_sub(start.elapsed());
        let evt = tokio::time::timeout(remaining, rx.recv())
            .await
            .expect("timeout waiting for event")
            .expect("event");
        match evt.msg {
            EventMsg::Error(payload)
                if payload.codex_error_info == Some(CodexErrorInfo::ThreadRollbackFailed) =>
            {
                return payload;
            }
            _ => continue,
        }
    }
}

async fn attach_thread_persistence(session: &mut Session) -> PathBuf {
    let config = session.get_config().await;
    let live_thread = LiveThread::create(
        Arc::clone(&session.services.thread_store),
        CreateThreadParams {
            thread_id: session.conversation_id,
            forked_from_id: None,
            source: SessionSource::Exec,
            thread_source: None,
            base_instructions: BaseInstructions::default(),
            dynamic_tools: Vec::new(),
            history_mode: ThreadHistoryMode::Legacy,
            metadata: ThreadPersistenceMetadata {
                cwd: Some(config.cwd.to_path_buf()),
                model_provider: config.model_provider_id.clone(),
                memory_mode: if config.memories.generate_memories {
                    ThreadMemoryMode::Enabled
                } else {
                    ThreadMemoryMode::Disabled
                },
            },
            event_persistence_mode: ThreadEventPersistenceMode::Limited,
        },
    )
    .await
    .expect("create thread persistence");
    session.services.live_thread = Some(live_thread);
    session.ensure_rollout_materialized().await;
    session
        .flush_rollout()
        .await
        .expect("attached rollout should flush");
    session
        .current_rollout_path()
        .await
        .expect("load rollout path")
        .expect("thread should have rollout path")
}

fn text_block(s: &str) -> serde_json::Value {
    json!({
        "type": "text",
        "text": s,
    })
}

async fn build_test_config(codex_home: &Path) -> Config {
    ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.to_path_buf())
        .build()
        .await
        .expect("load default test config")
}

fn session_telemetry(
    conversation_id: ThreadId,
    config: &Config,
    model_info: &ModelInfo,
    session_source: SessionSource,
) -> SessionTelemetry {
    SessionTelemetry::new(
        conversation_id,
        get_model_offline_for_tests(config.model.as_deref()).as_str(),
        model_info.slug.as_str(),
        /*account_id*/ None,
        Some("test@test.com".to_string()),
        Some(TelemetryAuthMode::Chatgpt),
        "test_originator".to_string(),
        /*log_user_prompts*/ false,
        "test".to_string(),
        session_source,
    )
}
