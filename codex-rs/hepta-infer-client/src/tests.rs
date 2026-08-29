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
use codex_uds::UnixStream;
use tempfile::TempDir;
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
    assert!(matches!(
        InferdClient::new(ClientConfig::qualification_only("infer.sock")),
        Err(ClientError::Config(_))
    ));

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
fn exact_capability_profile_is_fail_closed() {
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
async fn public_client_rejects_worker_and_operator_messages_before_connect() {
    let temp = must(TempDir::new());
    let client = must(InferdClient::new(ClientConfig::qualification_only(
        temp.path().join("missing.sock"),
    )));
    let request_id = must(RequestId::parse("request-role"));
    assert!(matches!(
        client
            .exchange(ClientMessage::Start {
                request_id: request_id.clone(),
                request_generation: 1,
                backend_generation: 7,
            })
            .await,
        Err(ClientError::RoleNotAuthorized)
    ));
    assert!(matches!(
        client
            .exchange(ClientMessage::RestartBackend {
                expected_generation: 7,
            })
            .await,
        Err(ClientError::RoleNotAuthorized)
    ));
}

#[tokio::test]
async fn admit_cancel_and_durable_receipt_query_use_public_operations_only() {
    let mut harness = Harness::start().await;
    let client = harness.client();
    must(client.ping(17).await);
    let snapshot = must(client.snapshot().await);
    assert_eq!(
        snapshot.authority,
        AuthoritySnapshot::qualification_only_closed()
    );

    let request_id = must(RequestId::parse("request-client-a"));
    let admitted = must(
        client
            .admit(request(digest('a'), request_id.as_str()))
            .await,
    );
    let backend_generation = match admitted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    let cancelled = must(
        client
            .cancel(request_id.clone(), 1, 1, backend_generation)
            .await,
    );
    let receipt = match cancelled {
        ServerMessage::Receipt(receipt) => receipt,
        other => panic!("unexpected cancellation response: {other:?}"),
    };
    assert_eq!(receipt.terminal_state, LifecycleState::Cancelled);
    assert_eq!(receipt.cancel_generation, 1);
    assert!(!receipt.forced_worker_termination);

    let queried = must(client.receipt(request_id, 1, backend_generation, 2).await);
    assert_eq!(queried, receipt);
    let snapshot = must(client.snapshot().await);
    assert_eq!(snapshot.terminal_receipts, 0);
    harness.stop().await;
}

#[tokio::test]
async fn shadow_client_checks_capability_before_public_admission() {
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
    assert_eq!(must(shadow.snapshot().await).queued_requests, 0);

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
async fn unknown_tuple_and_stale_generation_errors_are_preserved() {
    let mut harness = Harness::start().await;
    let client = harness.client();
    assert!(matches!(
        client
            .admit(request(digest('f'), "request-unknown"))
            .await,
        Err(ClientError::Remote(ref code)) if code == "INF_UNKNOWN_MODEL_TUPLE"
    ));

    let request_id = must(RequestId::parse("request-stale"));
    let admitted = must(
        client
            .admit(request(digest('a'), request_id.as_str()))
            .await,
    );
    let generation = match admitted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    assert!(matches!(
        client.cancel(request_id, 1, 1, generation + 1).await,
        Err(ClientError::Remote(ref code)) if code == "INF_STALE_BACKEND_GENERATION"
    ));
    harness.stop().await;
}
