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
use codex_extension_api::ModelProviderRequestKind;
use codex_extension_api::ModelProviderTerminal;
use codex_extension_api::ModelProviderTransport;
use core_test_support::responses::ev_assistant_message;
use core_test_support::responses::ev_completed;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::mount_sse_once;
use core_test_support::responses::sse;
use core_test_support::responses::start_mock_server;
use core_test_support::responses::start_websocket_server;
use core_test_support::skip_if_no_network;
use core_test_support::test_codex::test_codex;
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

#[derive(Debug, Eq, PartialEq)]
struct ProviderAttemptObservation {
    request_kind: ModelProviderRequestKind,
    transport: ModelProviderTransport,
    has_previous_response: bool,
    generate: bool,
}

struct ProviderPolicyState {
    active: bool,
    decision: TestDecision,
    begin_count: AtomicUsize,
    terminal_count: AtomicUsize,
    completed_count: AtomicUsize,
    attempts: Mutex<Vec<ProviderAttemptObservation>>,
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
            attempts: Mutex::new(Vec::new()),
            terminal_entered: Notify::new(),
            terminal_release: Semaphore::new(0),
        })
    }

    async fn wait_for_terminal_count(&self, expected: usize) {
        while self.terminal_count.load(Ordering::SeqCst) < expected {
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
            .attempts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(ProviderAttemptObservation {
                request_kind: input.request_kind,
                transport: input.transport,
                has_previous_response: input.previous_response_id_sha256.is_some(),
                generate: input.generate,
            });
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
            self.state.terminal_entered.notify_one();
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
        () = state.wait_for_terminal_count(1) => {}
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
    state.terminal_release.add_permits(16);
    let test = test_codex()
        .with_config(|config| {
            config.model_provider.stream_max_retries = Some(3);
        })
        .with_extensions(extensions_with_policy(Arc::clone(&state)))
        .build(&server)
        .await?;

    let _ = timeout(
        Duration::from_secs(10),
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn active_provider_policy_governs_websocket_prewarm_and_turn() -> Result<()> {
    skip_if_no_network!(Ok(()));

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
    let mut builder = test_codex()
        .with_config(|config| {
            config.model_provider.stream_max_retries = Some(0);
        })
        .with_extensions(extensions_with_policy(Arc::clone(&state)));
    let test = builder.build_with_websocket_server(&server).await?;

    let submit = test.submit_turn("prewarm and turn need separate websocket claims");
    tokio::pin!(submit);
    tokio::select! {
        result = &mut submit => panic!("turn completed before prewarm terminal entered: {result:?}"),
        () = state.wait_for_terminal_count(1) => {}
    }
    assert_eq!(state.begin_count.load(Ordering::SeqCst), 1);
    assert_eq!(state.terminal_count.load(Ordering::SeqCst), 1);
    assert_eq!(state.completed_count.load(Ordering::SeqCst), 1);
    assert!(
        timeout(Duration::from_millis(50), &mut submit)
            .await
            .is_err(),
        "turn must wait for the prewarm terminal acknowledgement"
    );

    state.terminal_release.add_permits(1);
    timeout(Duration::from_secs(5), state.wait_for_terminal_count(2)).await?;
    assert!(
        timeout(Duration::from_millis(50), &mut submit)
            .await
            .is_err(),
        "turn must wait for its own provider terminal acknowledgement"
    );
    state.terminal_release.add_permits(1);
    timeout(Duration::from_secs(10), &mut submit).await??;

    assert_eq!(state.begin_count.load(Ordering::SeqCst), 2);
    assert_eq!(state.terminal_count.load(Ordering::SeqCst), 2);
    assert_eq!(state.completed_count.load(Ordering::SeqCst), 2);
    assert_eq!(
        *state
            .attempts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
        vec![
            ProviderAttemptObservation {
                request_kind: ModelProviderRequestKind::Prewarm,
                transport: ModelProviderTransport::WebSocket,
                has_previous_response: false,
                generate: false,
            },
            ProviderAttemptObservation {
                request_kind: ModelProviderRequestKind::Turn,
                transport: ModelProviderTransport::WebSocket,
                has_previous_response: false,
                generate: true,
            }
        ]
    );
    let connections = server.connections();
    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].len(), 2);

    server.shutdown().await;
    Ok(())
}
