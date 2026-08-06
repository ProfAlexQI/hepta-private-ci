use anyhow::Result;
use codex_core::config::Config;
use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistryBuilder;
use codex_extension_api::ModelProviderAttemptLease;
use codex_extension_api::ModelProviderInvocationInput;
use codex_extension_api::ModelProviderPolicyContributor;
use codex_extension_api::ModelProviderPolicyDecision;
use codex_extension_api::ModelProviderPolicyError;
use codex_extension_api::ModelProviderPolicyFuture;
use codex_extension_api::ModelProviderTerminal;
use codex_extension_api::PROMPT_ONLY_INPUT_PROPOSAL_SCHEMA_VERSION;
use codex_extension_api::PromptOnlyInputContext;
use codex_extension_api::PromptOnlyInputContributor;
use codex_extension_api::PromptOnlyInputProposal;
use codex_extension_api::PromptOnlyInputSource;
use codex_features::Feature;
use core_test_support::responses::ev_assistant_message;
use core_test_support::responses::ev_completed;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::mount_sse_once;
use core_test_support::responses::sse;
use core_test_support::responses::start_mock_server;
use core_test_support::responses::start_websocket_server;
use core_test_support::test_codex::test_codex;
use sha2::Digest as _;
use sha2::Sha256;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use wiremock::Mock;
use wiremock::ResponseTemplate;
use wiremock::matchers::method;
use wiremock::matchers::path;

#[derive(Clone, Copy)]
enum TestDecision {
    Allow,
    Block,
}

struct ProviderPolicyState {
    active: bool,
    decision: TestDecision,
    begin_count: AtomicUsize,
    terminal_count: AtomicUsize,
    completed_count: AtomicUsize,
    prompt_only_bindings: Mutex<Vec<(Option<String>, Option<String>)>>,
    terminal_entered: Notify,
    terminal_release: Semaphore,
}

impl ProviderPolicyState {
    fn new(active: bool, decision: TestDecision) -> Arc<Self> {
        Arc::new(Self {
            active,
            decision,
            begin_count: AtomicUsize::new(0),
            terminal_count: AtomicUsize::new(0),
            completed_count: AtomicUsize::new(0),
            prompt_only_bindings: Mutex::new(Vec::new()),
            terminal_entered: Notify::new(),
            terminal_release: Semaphore::new(0),
        })
    }

    async fn wait_for_terminal(&self) {
        while self.terminal_count.load(Ordering::SeqCst) == 0 {
            self.terminal_entered.notified().await;
        }
    }
}

struct TestProviderPolicy {
    state: Arc<ProviderPolicyState>,
}

impl ModelProviderPolicyContributor for TestProviderPolicy {
    fn is_active(&self, _thread_store: &ExtensionData) -> bool {
        self.state.active
    }

    fn begin<'a>(
        &'a self,
        input: ModelProviderInvocationInput<'a>,
    ) -> ModelProviderPolicyFuture<'a, ModelProviderPolicyDecision> {
        self.state
            .prompt_only_bindings
            .lock()
            .expect("prompt-only binding lock should not be poisoned")
            .push((
                input
                    .ephemeral_input_sha256
                    .map(|digest| digest.as_str().to_string()),
                input
                    .ephemeral_input_witness_sha256
                    .map(|digest| digest.as_str().to_string()),
            ));
        self.state.begin_count.fetch_add(1, Ordering::SeqCst);
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            match state.decision {
                TestDecision::Allow => Ok(ModelProviderPolicyDecision::Allow {
                    lease: Box::new(TestProviderLease { state }),
                }),
                TestDecision::Block => Ok(ModelProviderPolicyDecision::Block {
                    reason_code: "test_provider_block".to_string(),
                    message: "blocked by the test provider policy".to_string(),
                }),
            }
        })
    }
}

struct StaticPromptOnlyInput {
    content: String,
}

impl PromptOnlyInputContributor for StaticPromptOnlyInput {
    fn contribute<'a>(
        &'a self,
        input: PromptOnlyInputContext,
        _session_store: &'a ExtensionData,
        _thread_store: &'a ExtensionData,
        _turn_store: &'a ExtensionData,
    ) -> codex_extension_api::ExtensionFuture<
        'a,
        Result<Option<PromptOnlyInputProposal>, ModelProviderPolicyError>,
    > {
        Box::pin(async move {
            if !input.host_authority_enabled {
                return Ok(None);
            }
            Ok(Some(PromptOnlyInputProposal {
                schema_version: PROMPT_ONLY_INPUT_PROPOSAL_SCHEMA_VERSION,
                source: PromptOnlyInputSource::parse("hepta_memory_same_thread_v1")?,
                thread_id: input.thread_id,
                turn_id: input.turn_id,
                source_binding_sha256: api_digest(b"same-thread-source-binding"),
                content_sha256: api_digest(self.content.as_bytes()),
                content: self.content.clone(),
                claimed_token_count: u32::try_from(self.content.len()).map_err(|error| {
                    ModelProviderPolicyError::new(
                        "test_prompt_only_content_unbounded",
                        error.to_string(),
                    )
                })?,
            }))
        })
    }
}

struct TestProviderLease {
    state: Arc<ProviderPolicyState>,
}

impl ModelProviderAttemptLease for TestProviderLease {
    fn finish(
        self: Box<Self>,
        terminal: ModelProviderTerminal,
    ) -> ModelProviderPolicyFuture<'static, ()> {
        Box::pin(async move {
            self.state.terminal_count.fetch_add(1, Ordering::SeqCst);
            if matches!(terminal, ModelProviderTerminal::Completed { .. }) {
                self.state.completed_count.fetch_add(1, Ordering::SeqCst);
            }
            self.state.terminal_entered.notify_waiters();
            let permit = self
                .state
                .terminal_release
                .acquire()
                .await
                .map_err(|error| {
                    ModelProviderPolicyError::new("test_terminal_gate_closed", error.to_string())
                })?;
            permit.forget();
            Ok(())
        })
    }
}

fn extensions_with_policy(
    state: Arc<ProviderPolicyState>,
) -> Arc<codex_extension_api::ExtensionRegistry<Config>> {
    let mut extensions = ExtensionRegistryBuilder::<Config>::new();
    extensions.model_provider_policy_contributor(Arc::new(TestProviderPolicy { state }));
    Arc::new(extensions.build())
}

fn extensions_with_prompt_only_input(
    state: Arc<ProviderPolicyState>,
    content: &str,
) -> Arc<codex_extension_api::ExtensionRegistry<Config>> {
    let mut extensions = ExtensionRegistryBuilder::<Config>::new();
    extensions.model_provider_policy_contributor(Arc::new(TestProviderPolicy { state }));
    extensions.prompt_only_input_contributor(Arc::new(StaticPromptOnlyInput {
        content: content.to_string(),
    }));
    Arc::new(extensions.build())
}

fn api_digest(bytes: &[u8]) -> codex_extension_api::ModelProviderSha256Digest {
    codex_extension_api::ModelProviderSha256Digest::parse(format!("{:x}", Sha256::digest(bytes)))
        .expect("test digest")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn provider_policy_block_prevents_http_send() -> Result<()> {
    let server = start_mock_server().await;
    let response_mock = mount_sse_once(
        &server,
        sse(vec![ev_response_created("resp-1"), ev_completed("resp-1")]),
    )
    .await;
    let state = ProviderPolicyState::new(true, TestDecision::Block);
    let test = test_codex()
        .with_extensions(extensions_with_policy(Arc::clone(&state)))
        .build(&server)
        .await?;

    test.submit_turn("this request must be blocked before transport")
        .await?;

    assert_eq!(state.begin_count.load(Ordering::SeqCst), 1);
    assert!(response_mock.requests().is_empty());
    assert_eq!(state.terminal_count.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn prompt_only_input_is_witnessed_per_send_and_never_enters_later_history() -> Result<()> {
    let server = start_mock_server().await;
    let first = mount_sse_once(
        &server,
        sse(vec![
            ev_response_created("resp-1"),
            ev_assistant_message("msg-1", "first done"),
            ev_completed("resp-1"),
        ]),
    )
    .await;
    let second = mount_sse_once(
        &server,
        sse(vec![
            ev_response_created("resp-2"),
            ev_assistant_message("msg-2", "second done"),
            ev_completed("resp-2"),
        ]),
    )
    .await;
    let state = ProviderPolicyState::new(true, TestDecision::Allow);
    state.terminal_release.add_permits(2);
    let test = test_codex()
        .with_extensions(extensions_with_prompt_only_input(
            Arc::clone(&state),
            "reviewed same-thread memory",
        ))
        .with_config(|config| {
            for feature in [
                Feature::HeptaGovernance,
                Feature::HeptaMemory,
                Feature::HeptaMemoryReadOnly,
            ] {
                config
                    .features
                    .enable(feature)
                    .expect("test feature should enable");
            }
        })
        .build(&server)
        .await?;

    test.submit_turn("first user request").await?;
    test.submit_turn("second user request").await?;

    let first_request = first.single_request();
    let second_request = second.single_request();
    for request in [&first_request, &second_request] {
        let references = request
            .message_input_texts("user")
            .into_iter()
            .filter(|text| text.contains("<hepta_memory_reference"))
            .collect::<Vec<_>>();
        assert_eq!(references.len(), 1);
        assert!(references[0].contains("reviewed same-thread memory"));
    }
    let witnesses = {
        let bindings = state
            .prompt_only_bindings
            .lock()
            .expect("prompt-only binding lock should not be poisoned");
        assert_eq!(bindings.len(), 2);
        assert!(bindings.iter().all(|(input, witness)| {
            input.as_ref().is_some_and(|digest| digest.len() == 64)
                && witness.as_ref().is_some_and(|digest| digest.len() == 64)
        }));
        (bindings[0].1.clone(), bindings[1].1.clone())
    };
    assert_ne!(witnesses.0, witnesses.1);
    let rollout = tokio::fs::read_to_string(
        test.codex
            .rollout_path()
            .expect("prompt-only test rollout path"),
    )
    .await?;
    assert!(!rollout.contains("<hepta_memory_reference"));
    assert!(!rollout.contains("reviewed same-thread memory"));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn prompt_only_input_crosses_websocket_with_one_witnessed_physical_send() -> Result<()> {
    let server = start_websocket_server(vec![vec![
        vec![ev_response_created("warm-1"), ev_completed("warm-1")],
        vec![
            ev_response_created("resp-1"),
            ev_assistant_message("msg-1", "done"),
            ev_completed("resp-1"),
        ],
    ]])
    .await;
    let state = ProviderPolicyState::new(true, TestDecision::Allow);
    state.terminal_release.add_permits(2);
    let mut builder = test_codex()
        .with_extensions(extensions_with_prompt_only_input(
            Arc::clone(&state),
            "websocket same-thread memory",
        ))
        .with_config(|config| {
            for feature in [
                Feature::HeptaGovernance,
                Feature::HeptaMemory,
                Feature::HeptaMemoryReadOnly,
            ] {
                config
                    .features
                    .enable(feature)
                    .expect("test feature should enable");
            }
        });
    let test = builder.build_with_websocket_server(&server).await?;

    test.submit_turn("websocket user request").await?;

    let connection = server.single_connection();
    assert_eq!(connection.len(), 2);
    let warmup = connection[0].body_json().to_string();
    let turn = connection[1].body_json().to_string();
    assert!(!warmup.contains("<hepta_memory_reference"));
    assert!(turn.contains("<hepta_memory_reference"));
    assert!(turn.contains("websocket same-thread memory"));
    {
        let bindings = state
            .prompt_only_bindings
            .lock()
            .expect("prompt-only binding lock should not be poisoned");
        assert_eq!(bindings.len(), 2);
        assert_eq!(
            bindings.iter().filter(|(input, _)| input.is_some()).count(),
            1
        );
        assert_eq!(
            bindings
                .iter()
                .filter(|(_, witness)| witness.is_some())
                .count(),
            1
        );
    }
    let rollout = tokio::fs::read_to_string(
        test.codex
            .rollout_path()
            .expect("websocket prompt-only rollout path"),
    )
    .await?;
    assert!(!rollout.contains("<hepta_memory_reference"));
    assert!(!rollout.contains("websocket same-thread memory"));
    server.shutdown().await;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn provider_policy_completed_terminal_precedes_turn_completion() -> Result<()> {
    let server = start_mock_server().await;
    let response_mock = mount_sse_once(
        &server,
        sse(vec![
            ev_response_created("resp-1"),
            ev_assistant_message("msg-1", "done"),
            ev_completed("resp-1"),
        ]),
    )
    .await;
    let state = ProviderPolicyState::new(true, TestDecision::Allow);
    let test = test_codex()
        .with_extensions(extensions_with_policy(Arc::clone(&state)))
        .build(&server)
        .await?;

    let submit = test.submit_turn("wait for the durable provider terminal");
    tokio::pin!(submit);
    tokio::select! {
        result = &mut submit => panic!("turn completed before provider terminal entered: {result:?}"),
        () = state.wait_for_terminal() => {}
    }

    assert_eq!(state.begin_count.load(Ordering::SeqCst), 1);
    assert_eq!(response_mock.requests().len(), 1);
    assert_eq!(state.terminal_count.load(Ordering::SeqCst), 1);
    assert_eq!(state.completed_count.load(Ordering::SeqCst), 1);
    assert!(
        timeout(Duration::from_millis(50), &mut submit)
            .await
            .is_err(),
        "turn must not complete while provider terminal persistence is blocked"
    );

    state.terminal_release.add_permits(1);
    timeout(Duration::from_secs(5), &mut submit).await??;
    assert_eq!(state.terminal_count.load(Ordering::SeqCst), 1);
    assert_eq!(state.completed_count.load(Ordering::SeqCst), 1);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inactive_provider_policy_preserves_http_request_behavior() -> Result<()> {
    let server = start_mock_server().await;
    let response_mock = mount_sse_once(
        &server,
        sse(vec![
            ev_response_created("resp-1"),
            ev_assistant_message("msg-1", "unchanged"),
            ev_completed("resp-1"),
        ]),
    )
    .await;
    let state = ProviderPolicyState::new(false, TestDecision::Block);
    let test = test_codex()
        .with_extensions(extensions_with_policy(Arc::clone(&state)))
        .build(&server)
        .await?;

    test.submit_turn("inactive policy must not affect transport")
        .await?;

    assert_eq!(state.begin_count.load(Ordering::SeqCst), 0);
    assert_eq!(response_mock.requests().len(), 1);
    assert_eq!(state.terminal_count.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn active_provider_policy_claims_every_http_transport_invocation() -> Result<()> {
    let server = start_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/v1/responses"))
        .respond_with(
            ResponseTemplate::new(500)
                .insert_header("content-type", "application/json")
                .set_body_string(r#"{"error":{"message":"retryable test failure"}}"#),
        )
        .mount(&server)
        .await;

    let state = ProviderPolicyState::new(true, TestDecision::Allow);
    // A failed physical send must be finalized before a later host retry may
    // claim another attempt. Keep enough permits available for that outer
    // retry loop without allowing the API client's hidden retry loop to share
    // one lease.
    state.terminal_release.add_permits(16);
    let test = test_codex()
        .with_config(|config| {
            config.model_provider.stream_max_retries = Some(3);
        })
        .with_extensions(extensions_with_policy(Arc::clone(&state)))
        .build(&server)
        .await?;

    let _ = timeout(
        Duration::from_secs(5),
        test.submit_turn("each transport invocation needs its own durable claim"),
    )
    .await?;

    let request_count = server
        .received_requests()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|request| request.url.path() == "/v1/responses")
        .count();
    let claim_count = state.begin_count.load(Ordering::SeqCst);
    assert!(
        request_count > 0,
        "the test must reach the provider transport"
    );
    assert_eq!(
        request_count, claim_count,
        "one durable policy claim must never cover multiple transport sends"
    );
    assert_eq!(
        state.terminal_count.load(Ordering::SeqCst),
        claim_count,
        "every claimed failed send must reach one terminal"
    );
    Ok(())
}
