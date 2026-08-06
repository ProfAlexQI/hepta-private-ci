use anyhow::Result;
use codex_core::MemoryModelProviderPolicyHandle;
use codex_core::ModelClient;
use codex_core::Prompt;
use codex_core::ResponseEvent;
use codex_core::UserMessageAdmission;
use codex_core::detached_memory_responses_metadata;
use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistryBuilder;
use codex_extension_api::ModelProviderAttemptLease;
use codex_extension_api::ModelProviderInvocationInput;
use codex_extension_api::ModelProviderPolicyContributor;
use codex_extension_api::ModelProviderPolicyDecision;
use codex_extension_api::ModelProviderPolicyFuture;
use codex_extension_api::ModelProviderRequestKind;
use codex_extension_api::ModelProviderTerminal;
use codex_features::Feature;
use codex_login::auth::AgentIdentityAuthPolicy;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::Op;
use codex_protocol::protocol::ThreadHistoryMode;
use codex_protocol::user_input::UserInput;
use core_test_support::responses;
use core_test_support::responses::ev_completed;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::mount_sse_once_match;
use core_test_support::responses::sse;
use core_test_support::responses::start_mock_server;
use core_test_support::streaming_sse::StreamingSseChunk;
use core_test_support::streaming_sse::start_streaming_sse_server;
use core_test_support::test_codex::TestCodex;
use core_test_support::test_codex::test_codex;
use core_test_support::wait_for_event;
use futures::StreamExt;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::sync::Barrier;
use tokio::sync::oneshot;
use tokio::time::timeout;

#[derive(Clone, Debug, Eq, PartialEq)]
struct InvocationRecord {
    request_kind: ModelProviderRequestKind,
    thread_id: String,
    turn_id: String,
}

struct MemoryPolicyState {
    active: bool,
    begin_count: AtomicUsize,
    terminal_count: AtomicUsize,
    invocations: Mutex<Vec<InvocationRecord>>,
}

impl MemoryPolicyState {
    fn new(active: bool) -> Arc<Self> {
        Arc::new(Self {
            active,
            begin_count: AtomicUsize::new(0),
            terminal_count: AtomicUsize::new(0),
            invocations: Mutex::new(Vec::new()),
        })
    }

    fn invocations(&self) -> Vec<InvocationRecord> {
        self.invocations
            .lock()
            .expect("memory policy invocation lock should not be poisoned")
            .clone()
    }
}

struct MemoryPolicy {
    state: Arc<MemoryPolicyState>,
}

impl ModelProviderPolicyContributor for MemoryPolicy {
    fn is_active(&self, _thread_store: &ExtensionData) -> bool {
        self.state.active
    }

    fn begin<'a>(
        &'a self,
        input: ModelProviderInvocationInput<'a>,
    ) -> ModelProviderPolicyFuture<'a, ModelProviderPolicyDecision> {
        self.state.begin_count.fetch_add(1, Ordering::SeqCst);
        self.state
            .invocations
            .lock()
            .expect("memory policy invocation lock should not be poisoned")
            .push(InvocationRecord {
                request_kind: input.request_kind,
                thread_id: input.thread_id.to_string(),
                turn_id: input.turn_id.to_string(),
            });
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            if input.request_kind == ModelProviderRequestKind::Memory {
                Ok(ModelProviderPolicyDecision::Block {
                    reason_code: "test_memory_provider_block".to_string(),
                    message: "blocked by the memory provider policy test".to_string(),
                })
            } else {
                Ok(ModelProviderPolicyDecision::Allow {
                    lease: Box::new(MemoryPolicyLease { state }),
                })
            }
        })
    }
}

struct MemoryPolicyLease {
    state: Arc<MemoryPolicyState>,
}

impl ModelProviderAttemptLease for MemoryPolicyLease {
    fn finish(
        self: Box<Self>,
        _terminal: ModelProviderTerminal,
    ) -> ModelProviderPolicyFuture<'static, ()> {
        Box::pin(async move {
            self.state.terminal_count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
    }
}

fn extensions_with_memory_policy(
    state: Arc<MemoryPolicyState>,
) -> Arc<codex_extension_api::ExtensionRegistry<codex_core::config::Config>> {
    let mut extensions = ExtensionRegistryBuilder::new();
    extensions.model_provider_policy_contributor(Arc::new(MemoryPolicy { state }));
    Arc::new(extensions.build())
}

fn user_input(text: &str) -> Op {
    Op::UserInput {
        items: vec![UserInput::Text {
            text: text.to_string(),
            text_elements: Vec::new(),
        }],
        final_output_json_schema: None,
        responsesapi_client_metadata: None,
        additional_context: Default::default(),
        thread_settings: Default::default(),
    }
}

fn request_body_contains(request: &wiremock::Request, needle: &str) -> bool {
    String::from_utf8_lossy(&request.body).contains(needle)
}

async fn submit_parent_and_capture_memory_policy(
    test: &TestCodex,
) -> Result<(UserMessageAdmission, MemoryModelProviderPolicyHandle)> {
    let captured = test
        .codex
        .submit_user_input_and_capture_memory_policy(
            user_input("parent turn for detached memory"),
            /*trace*/ None,
            Some("memory-parent-message".to_string()),
        )
        .await?;
    wait_for_event(&test.codex, |event| {
        matches!(event, EventMsg::TurnComplete(_))
    })
    .await;
    Ok(captured)
}

async fn stream_detached_memory(
    test: &TestCodex,
    provider_policy: &MemoryModelProviderPolicyHandle,
) -> Result<()> {
    let config = test.codex.config().await;
    let config_snapshot = test.codex.config_snapshot().await;
    let model = codex_core::test_support::get_model_offline(config.model.as_deref());
    let model_info = codex_core::test_support::construct_model_info_offline(&model, &config);
    let model_client = ModelClient::new(
        Some(test.thread_manager.auth_manager()),
        AgentIdentityAuthPolicy::JwtOnly,
        test.session_configured.thread_id,
        config.model_provider.clone(),
        config_snapshot.session_source.clone(),
        config_snapshot.originator,
        config.model_verbosity,
        config.features.enabled(Feature::EnableRequestCompression),
        config.features.enabled(Feature::RuntimeMetrics),
        /*beta_features_header*/ None,
        /*concurrent_reasoning_summaries_enabled*/ false,
        /*attestation_provider*/ None,
        config.http_client_factory(),
    );
    let thread_id = test.session_configured.thread_id.to_string();
    let responses_metadata = detached_memory_responses_metadata(
        "memory-policy-test-installation".to_string(),
        thread_id.clone(),
        thread_id.clone(),
        format!("{thread_id}:memory-policy-test"),
        &config_snapshot.session_source,
        &config.cwd,
        /*sandbox*/ None,
    )
    .await;
    let mut prompt = Prompt::default();
    prompt.input.push(ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "detached memory sample".to_string(),
        }],
        phase: None,
        internal_chat_message_metadata_passthrough: None,
    });
    let mut client_session = model_client.new_session();
    let mut stream = client_session
        .stream_memory_with_policy(
            &prompt,
            &model_info,
            &test.codex.session_telemetry(),
            config.model_reasoning_effort.clone(),
            config
                .model_reasoning_summary
                .unwrap_or(model_info.default_reasoning_summary),
            config_snapshot.service_tier,
            &responses_metadata,
            &codex_rollout_trace::InferenceTraceContext::disabled(),
            provider_policy,
        )
        .await?;

    while let Some(event) = stream.next().await.transpose()? {
        if matches!(event, ResponseEvent::Completed { .. }) {
            return Ok(());
        }
    }
    anyhow::bail!("detached memory response ended before completion")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_started_and_steered_memory_handles_share_the_exact_admitted_turn() -> Result<()>
{
    let (release_response, response_gate) = oneshot::channel();
    let (server, _completions) = start_streaming_sse_server(vec![
        vec![
            StreamingSseChunk {
                gate: None,
                body: responses::sse(vec![ev_response_created("resp-parent-1")]),
            },
            StreamingSseChunk {
                gate: Some(response_gate),
                body: responses::sse(vec![ev_completed("resp-parent-1")]),
            },
        ],
        vec![StreamingSseChunk {
            gate: None,
            body: responses::sse(vec![
                ev_response_created("resp-parent-2"),
                ev_completed("resp-parent-2"),
            ]),
        }],
    ])
    .await;
    let test = test_codex()
        .with_model("gpt-5.4")
        .with_history_mode(ThreadHistoryMode::Paginated)
        .build_with_streaming_server(&server)
        .await?;
    let codex = Arc::clone(&test.codex);
    let barrier = Arc::new(Barrier::new(3));

    let first_submission = tokio::spawn({
        let codex = Arc::clone(&codex);
        let barrier = Arc::clone(&barrier);
        async move {
            barrier.wait().await;
            codex
                .submit_user_input_and_capture_memory_policy(
                    user_input("first concurrent parent message"),
                    /*trace*/ None,
                    Some("memory-client-message-1".to_string()),
                )
                .await
        }
    });
    let second_submission = tokio::spawn({
        let codex = Arc::clone(&codex);
        let barrier = Arc::clone(&barrier);
        async move {
            barrier.wait().await;
            codex
                .submit_user_input_and_capture_memory_policy(
                    user_input("second concurrent parent message"),
                    /*trace*/ None,
                    Some("memory-client-message-2".to_string()),
                )
                .await
        }
    });
    barrier.wait().await;

    let (first, second) = timeout(Duration::from_secs(5), async {
        tokio::join!(first_submission, second_submission)
    })
    .await?;
    let (first_admission, first_policy) = first??;
    let (second_admission, second_policy) = second??;
    assert!(matches!(
        (&first_admission, &second_admission),
        (
            UserMessageAdmission::Started { .. },
            UserMessageAdmission::Steered { .. }
        ) | (
            UserMessageAdmission::Steered { .. },
            UserMessageAdmission::Started { .. }
        )
    ));
    assert_eq!(first_admission.turn_id(), second_admission.turn_id());
    assert_eq!(
        codex_core::test_support::memory_model_provider_policy_parent_turn_id(&first_policy),
        first_admission.turn_id()
    );
    assert_eq!(
        codex_core::test_support::memory_model_provider_policy_parent_turn_id(&second_policy),
        second_admission.turn_id()
    );
    assert!(
        codex_core::test_support::memory_model_provider_policy_handles_share_parent_turn(
            &first_policy,
            &second_policy
        )
    );

    release_response
        .send(())
        .expect("parent response gate should remain open");
    loop {
        let event = timeout(Duration::from_secs(10), codex.next_event()).await??;
        match event.msg {
            EventMsg::Error(error) => anyhow::bail!("admitted parent failed: {error:?}"),
            EventMsg::StreamError(error) => {
                anyhow::bail!("admitted parent stream failed: {error:?}")
            }
            EventMsg::TurnComplete(_) => break,
            _ => {}
        }
    }
    server.shutdown().await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn memory_policy_block_after_parent_completion_prevents_detached_http_send() -> Result<()> {
    let server = start_mock_server().await;
    let parent_response = mount_sse_once_match(
        &server,
        |request: &wiremock::Request| {
            request_body_contains(request, "parent turn for detached memory")
        },
        sse(vec![
            ev_response_created("resp-parent"),
            ev_completed("resp-parent"),
        ]),
    )
    .await;
    let memory_response = mount_sse_once_match(
        &server,
        |request: &wiremock::Request| request_body_contains(request, "detached memory sample"),
        sse(vec![
            ev_response_created("resp-memory"),
            ev_completed("resp-memory"),
        ]),
    )
    .await;
    let state = MemoryPolicyState::new(true);
    let test = test_codex()
        .with_extensions(extensions_with_memory_policy(Arc::clone(&state)))
        .build(&server)
        .await?;

    let (admission, provider_policy) = submit_parent_and_capture_memory_policy(&test).await?;
    assert_eq!(parent_response.requests().len(), 1);
    let error = stream_detached_memory(&test, &provider_policy)
        .await
        .expect_err("active memory policy should block before transport");
    assert!(
        error
            .to_string()
            .contains("blocked by the memory provider policy test")
    );
    assert!(memory_response.requests().is_empty());

    let invocations = state.invocations();
    assert_eq!(
        invocations
            .iter()
            .filter(|record| record.request_kind == ModelProviderRequestKind::Memory)
            .collect::<Vec<_>>(),
        vec![&InvocationRecord {
            request_kind: ModelProviderRequestKind::Memory,
            thread_id: test.session_configured.thread_id.to_string(),
            turn_id: admission.turn_id().to_string(),
        }]
    );
    assert_eq!(state.begin_count.load(Ordering::SeqCst), 2);
    assert_eq!(state.terminal_count.load(Ordering::SeqCst), 1);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inactive_policy_preserves_memory_send_after_zero_latency_parent_completion() -> Result<()>
{
    let server = start_mock_server().await;
    let parent_response = mount_sse_once_match(
        &server,
        |request: &wiremock::Request| {
            request_body_contains(request, "parent turn for detached memory")
        },
        sse(vec![
            ev_response_created("resp-parent"),
            ev_completed("resp-parent"),
        ]),
    )
    .await;
    let memory_response = mount_sse_once_match(
        &server,
        |request: &wiremock::Request| request_body_contains(request, "detached memory sample"),
        sse(vec![
            ev_response_created("resp-memory"),
            ev_completed("resp-memory"),
        ]),
    )
    .await;
    let state = MemoryPolicyState::new(false);
    let test = test_codex()
        .with_extensions(extensions_with_memory_policy(Arc::clone(&state)))
        .build(&server)
        .await?;

    let (admission, provider_policy) = submit_parent_and_capture_memory_policy(&test).await?;
    assert_eq!(
        codex_core::test_support::memory_model_provider_policy_parent_turn_id(&provider_policy),
        admission.turn_id()
    );
    stream_detached_memory(&test, &provider_policy).await?;

    assert_eq!(parent_response.requests().len(), 1);
    assert_eq!(memory_response.requests().len(), 1);
    assert_eq!(state.begin_count.load(Ordering::SeqCst), 0);
    assert_eq!(state.terminal_count.load(Ordering::SeqCst), 0);
    assert!(state.invocations().is_empty());
    Ok(())
}
