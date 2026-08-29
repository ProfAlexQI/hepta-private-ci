use std::io;
use std::path::Path;
use std::time::Duration;

use codex_hepta_infer_core::AgentId;
use codex_hepta_infer_core::AuthoritySnapshot;
use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::InferenceRequest;
use codex_hepta_infer_core::LifecycleState;
use codex_hepta_infer_core::RequestId;
use codex_hepta_infer_core::RequestIdentity;
use codex_hepta_infer_core::ResourceBudgetId;
use codex_hepta_infer_core::ServerMessage;
use codex_hepta_infer_core::TaskId;
use codex_hepta_infer_core::TenantId;
use codex_hepta_infer_core::WorkspaceId;
use codex_hepta_inferd::DaemonConfig;
use codex_hepta_inferd::serve_with_shutdown;
use codex_uds::UnixListener;
use codex_uds::UnixStream;
use tempfile::TempDir;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::time::timeout;

use super::*;

fn must<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected error: {error}"),
    }
}

fn digest(fill: char) -> Digest {
    must(Digest::parse(&format!(
        "sha256:{}",
        fill.to_string().repeat(64)
    )))
}

fn request(tuple: Digest, request_id: &str) -> InferenceRequest {
    InferenceRequest {
        identity: RequestIdentity {
            tenant_id: must(TenantId::parse("tenant-a")),
            workspace_id: must(WorkspaceId::parse("workspace-a")),
            agent_id: must(AgentId::parse("agent-a")),
            task_id: must(TaskId::parse("task-a")),
            request_id: must(RequestId::parse(request_id)),
        },
        agent_generation: 1,
        request_generation: 1,
        cancel_generation: 0,
        deadline_unix_ms: u64::MAX,
        model_tuple_digest: tuple,
        policy_digest: digest('b'),
        resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
        prompt_digest: digest('c'),
        prompt_byte_length: 12,
        output_token_limit: 32,
        authority: AuthoritySnapshot::qualification_only_closed(),
    }
}

struct Harness {
    _temp: TempDir,
    config: DaemonConfig,
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<io::Result<()>>>,
}

impl Harness {
    async fn start() -> Self {
        let temp = must(TempDir::new());
        let socket_path = temp.path().join("socket").join("infer.sock");
        let receipt_dir = temp.path().join("receipts");
        let config = DaemonConfig::qualification_only(socket_path, receipt_dir, digest('a'));
        let (shutdown, receiver) = oneshot::channel();
        let task_config = config.clone();
        let task = tokio::spawn(async move {
            serve_with_shutdown(task_config, async {
                let _ = receiver.await;
            })
            .await
        });
        wait_for_socket(&config.socket_path).await;
        Self {
            _temp: temp,
            config,
            shutdown: Some(shutdown),
            task: Some(task),
        }
    }

    fn client(&self) -> InferdClient {
        must(InferdClient::new(ClientConfig::qualification_only(
            self.config.socket_path.clone(),
        )))
    }

    async fn stop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            match must(timeout(Duration::from_secs(5), task).await) {
                Ok(Ok(())) => {}
                Ok(Err(error)) => panic!("daemon failed: {error}"),
                Err(error) => panic!("daemon task failed: {error}"),
            }
        }
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

async fn wait_for_socket(path: &Path) {
    for _ in 0..200 {
        if UnixStream::connect(path).await.is_ok() {
            return;
        }
        sleep(Duration::from_millis(10)).await;
    }
    panic!("socket did not become ready: {}", path.display());
}

#[test]
fn configuration_is_absolute_bounded_and_nonzero() {
    let relative = InferdClient::new(ClientConfig::qualification_only("infer.sock"));
    assert!(matches!(relative, Err(ClientError::Config(_))));

    let temp = must(TempDir::new());
    let mut config = ClientConfig::qualification_only(temp.path().join("infer.sock"));
    config.connect_timeout = Duration::ZERO;
    assert!(matches!(
        InferdClient::new(config),
        Err(ClientError::Config("INF_CLIENT_TIMEOUT_ZERO"))
    ));

    let mut config = ClientConfig::qualification_only(temp.path().join("infer.sock"));
    config.max_frame_bytes = MAX_FRAME_BYTES + 1;
    assert!(matches!(
        InferdClient::new(config),
        Err(ClientError::Config("INF_CLIENT_FRAME_BOUND_INVALID"))
    ));
}

#[test]
fn exact_capability_profile_rejects_unsupported_and_known_gap_routes() {
    let tuple = digest('a');
    let profile = ExactCapabilityProfile::new(
        tuple.clone(),
        CapabilityDisposition::Qualified,
        CapabilityDisposition::UnsupportedFailClosed,
        CapabilityDisposition::KnownGapNotRouted,
        CapabilityDisposition::UnsupportedFailClosed,
    );
    assert_eq!(profile.route_mode(), RouteMode::ShadowCompareOnly);
    assert!(
        profile
            .require(&tuple, InferenceCapability::SemanticText)
            .is_ok()
    );
    assert!(matches!(
        profile.require(&tuple, InferenceCapability::NativeToolCall),
        Err(ClientError::CapabilityUnsupported(
            InferenceCapability::NativeToolCall
        ))
    ));
    assert!(matches!(
        profile.require(&tuple, InferenceCapability::StrictSse),
        Err(ClientError::CapabilityKnownGap(
            InferenceCapability::StrictSse
        ))
    ));
    assert!(matches!(
        profile.require(&digest('f'), InferenceCapability::SemanticText),
        Err(ClientError::ModelTupleNotRouted)
    ));
}

#[tokio::test]
async fn client_round_trip_admit_cancel_and_restart_are_bounded() {
    let mut harness = Harness::start().await;
    let client = harness.client();
    must(client.ping(17).await);
    let snapshot = must(client.snapshot().await);
    assert_eq!(
        snapshot.authority,
        AuthoritySnapshot::qualification_only_closed()
    );

    let admitted = must(client.admit(request(digest('a'), "request-client-a")).await);
    let backend_generation = match admitted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    let cancelled = must(
        client
            .cancel(
                must(RequestId::parse("request-client-a")),
                1,
                1,
                backend_generation,
            )
            .await,
    );
    match cancelled {
        ServerMessage::Receipt(receipt) => {
            assert_eq!(receipt.terminal_state, LifecycleState::Cancelled);
            assert_eq!(receipt.cancel_generation, 1);
            assert!(!receipt.forced_worker_termination);
            assert_eq!(
                receipt.authority,
                AuthoritySnapshot::qualification_only_closed()
            );
        }
        other => panic!("unexpected cancellation response: {other:?}"),
    }

    let restarted = must(client.restart_backend(backend_generation).await);
    match restarted {
        ServerMessage::Restarted {
            backend_generation: next,
            receipts,
        } => {
            assert_eq!(next, backend_generation + 1);
            assert!(receipts.is_empty());
        }
        other => panic!("unexpected restart response: {other:?}"),
    }
    harness.stop().await;
}

#[tokio::test]
async fn shadow_route_checks_capability_before_dispatch() {
    let mut harness = Harness::start().await;
    let tuple = digest('a');
    let shadow = ShadowInferdClient::new(
        harness.client(),
        ExactCapabilityProfile::new(
            tuple.clone(),
            CapabilityDisposition::Qualified,
            CapabilityDisposition::UnsupportedFailClosed,
            CapabilityDisposition::UnsupportedFailClosed,
            CapabilityDisposition::UnsupportedFailClosed,
        ),
    );
    assert_eq!(shadow.route_mode(), RouteMode::ShadowCompareOnly);
    let error = shadow
        .admit(
            InferenceCapability::NativeToolCall,
            request(tuple.clone(), "request-tool-denied"),
        )
        .await
        .expect_err("unsupported capability must fail before dispatch");
    assert!(matches!(
        error,
        ClientError::CapabilityUnsupported(InferenceCapability::NativeToolCall)
    ));
    let snapshot = must(shadow.snapshot().await);
    assert_eq!(snapshot.queued_requests, 0);

    let response = must(
        shadow
            .admit(
                InferenceCapability::SemanticText,
                request(tuple, "request-shadow-a"),
            )
            .await,
    );
    assert!(matches!(response, ServerMessage::Accepted(_)));
    harness.stop().await;
}

#[tokio::test]
async fn product_shadow_lifecycle_queries_one_fenced_terminal_receipt() {
    let mut harness = Harness::start().await;
    let tuple = digest('a');
    let shadow = ShadowInferdClient::new(
        harness.client(),
        ExactCapabilityProfile::new(
            tuple.clone(),
            CapabilityDisposition::Qualified,
            CapabilityDisposition::UnsupportedFailClosed,
            CapabilityDisposition::UnsupportedFailClosed,
            CapabilityDisposition::UnsupportedFailClosed,
        ),
    );
    let request_id = must(RequestId::parse("request-shadow-complete"));
    let admitted = must(
        shadow
            .admit(
                InferenceCapability::SemanticText,
                request(tuple, request_id.as_str()),
            )
            .await,
    );
    let backend_generation = match admitted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    let started = must(
        shadow
            .start(request_id.clone(), 1, backend_generation)
            .await,
    );
    assert_eq!(started.sequence, 2);
    let token = must(
        shadow
            .token(request_id.clone(), 1, backend_generation, 3, digest('d'), 2)
            .await,
    );
    assert_eq!(token.sequence, 3);
    let expected_result = digest('e');
    let completed = must(
        shadow
            .complete(
                request_id.clone(),
                1,
                backend_generation,
                4,
                expected_result.clone(),
                4,
            )
            .await,
    );
    assert_eq!(completed.result_digest, Some(expected_result));
    let queried = must(
        shadow
            .receipt(request_id.clone(), 1, backend_generation, 4)
            .await,
    );
    assert_eq!(queried, completed);
    let repeated = must(shadow.receipt(request_id, 1, backend_generation, 4).await);
    assert_eq!(repeated, completed);

    let mut entries = must(tokio::fs::read_dir(&harness.config.receipt_dir).await);
    let mut count = 0usize;
    while must(entries.next_entry().await).is_some() {
        count += 1;
    }
    assert_eq!(count, 1, "receipt queries must remain read-only");
    harness.stop().await;
}

#[tokio::test]
async fn terminal_receipt_poll_uses_one_total_deadline() {
    let mut harness = Harness::start().await;
    let client = harness.client();
    let request_id = must(RequestId::parse("request-poll"));
    let admitted = must(
        client
            .admit(request(digest('a'), request_id.as_str()))
            .await,
    );
    let backend_generation = match admitted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    let producer = client.clone();
    let producer_request_id = request_id.clone();
    let task = tokio::spawn(async move {
        sleep(Duration::from_millis(30)).await;
        must(
            producer
                .start(producer_request_id.clone(), 1, backend_generation)
                .await,
        );
        must(
            producer
                .complete(
                    producer_request_id,
                    1,
                    backend_generation,
                    3,
                    digest('d'),
                    2,
                )
                .await,
        )
    });
    let receipt = must(
        client
            .await_terminal_receipt(
                request_id,
                1,
                backend_generation,
                3,
                Duration::from_millis(5),
            )
            .await,
    );
    assert_eq!(receipt.last_sequence, 3);
    must(task.await);
    harness.stop().await;
}

#[tokio::test]
async fn unknown_tuple_and_stale_generation_errors_are_preserved() {
    let mut harness = Harness::start().await;
    let client = harness.client();
    let unknown = client
        .admit(request(digest('f'), "request-unknown"))
        .await
        .expect_err("unknown tuple must fail closed");
    assert!(matches!(
        unknown,
        ClientError::Remote(ref code) if code == "INF_UNKNOWN_MODEL_TUPLE"
    ));

    let admitted = must(client.admit(request(digest('a'), "request-stale")).await);
    let generation = match admitted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    let stale = client
        .cancel(
            must(RequestId::parse("request-stale")),
            1,
            1,
            generation + 1,
        )
        .await
        .expect_err("stale backend generation must be preserved");
    assert!(matches!(
        stale,
        ClientError::Remote(ref code) if code == "INF_STALE_BACKEND_GENERATION"
    ));
    harness.stop().await;
}

async fn start_scripted_server(
    socket_path: &Path,
    response: ScriptedResponse,
) -> JoinHandle<io::Result<()>> {
    let parent = socket_path.parent().expect("test socket parent");
    must(codex_uds::prepare_private_socket_directory(parent).await);
    let mut listener = must(UnixListener::bind(socket_path).await);
    tokio::spawn(async move {
        let mut stream = listener.accept().await?;
        let mut length_bytes = [0u8; 4];
        stream.read_exact(&mut length_bytes).await?;
        let request_length =
            usize::try_from(u32::from_be_bytes(length_bytes)).map_err(io::Error::other)?;
        let mut request = vec![0u8; request_length];
        stream.read_exact(&mut request).await?;
        match response {
            ScriptedResponse::Delay(duration) => sleep(duration).await,
            ScriptedResponse::Length(length) => {
                stream.write_all(&length.to_be_bytes()).await?;
                stream.flush().await?;
            }
            ScriptedResponse::Payload(payload) => {
                let length = u32::try_from(payload.len()).map_err(io::Error::other)?;
                stream.write_all(&length.to_be_bytes()).await?;
                stream.write_all(&payload).await?;
                stream.flush().await?;
            }
        }
        Ok(())
    })
}

enum ScriptedResponse {
    Delay(Duration),
    Length(u32),
    Payload(Vec<u8>),
}

#[tokio::test]
async fn exchange_timeout_is_fail_closed() {
    let temp = must(TempDir::new());
    let socket = temp.path().join("socket").join("infer.sock");
    let task =
        start_scripted_server(&socket, ScriptedResponse::Delay(Duration::from_secs(1))).await;
    let mut config = ClientConfig::qualification_only(socket);
    config.exchange_timeout = Duration::from_millis(50);
    let client = must(InferdClient::new(config));
    let error = client
        .ping(1)
        .await
        .expect_err("a stalled peer must reach the exchange deadline");
    assert!(matches!(error, ClientError::ExchangeTimeout));
    task.abort();
}

#[tokio::test]
async fn oversized_and_malformed_responses_fail_closed() {
    let temp = must(TempDir::new());
    let oversized_socket = temp.path().join("oversized").join("infer.sock");
    let oversized = must(u32::try_from(MAX_FRAME_BYTES + 1));
    let oversized_task =
        start_scripted_server(&oversized_socket, ScriptedResponse::Length(oversized)).await;
    let oversized_client = must(InferdClient::new(ClientConfig::qualification_only(
        oversized_socket,
    )));
    let error = oversized_client
        .ping(1)
        .await
        .expect_err("oversized response must fail closed");
    assert_eq!(error.code(), "INF_CLIENT_IO");
    must(must(oversized_task.await));

    let malformed_socket = temp.path().join("malformed").join("infer.sock");
    let malformed_task =
        start_scripted_server(&malformed_socket, ScriptedResponse::Payload(vec![0xff])).await;
    let malformed_client = must(InferdClient::new(ClientConfig::qualification_only(
        malformed_socket,
    )));
    let error = malformed_client
        .ping(2)
        .await
        .expect_err("non-canonical response must fail closed");
    assert!(matches!(error, ClientError::Protocol(_)));
    must(must(malformed_task.await));
}
