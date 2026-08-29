use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_infer_core::AgentId;
use codex_hepta_infer_core::AuthoritySnapshot;
use codex_hepta_infer_core::ClientMessage;
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
use codex_uds::UnixStream;
use tempfile::TempDir;
use tokio::fs;
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

fn request(tuple: Digest, request_id: &str, deadline_unix_ms: u64) -> InferenceRequest {
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
        deadline_unix_ms,
        model_tuple_digest: tuple,
        policy_digest: digest('b'),
        resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
        prompt_digest: digest('c'),
        prompt_byte_length: 12,
        output_token_limit: 32,
        authority: AuthoritySnapshot::qualification_only_closed(),
    }
}

fn config(temp: &TempDir) -> DaemonConfig {
    DaemonConfig::qualification_only(
        temp.path().join("socket").join("infer.sock"),
        temp.path().join("receipts"),
        digest('a'),
    )
}

struct RunningDaemon {
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<io::Result<()>>>,
}

impl RunningDaemon {
    async fn start(config: DaemonConfig) -> Self {
        let socket_path = config.socket_path.clone();
        let (shutdown, receiver) = oneshot::channel();
        let task = tokio::spawn(async move {
            serve_with_shutdown(config, async {
                let _ = receiver.await;
            })
            .await
        });
        wait_for_socket(&socket_path).await;
        Self {
            shutdown: Some(shutdown),
            task: Some(task),
        }
    }

    async fn stop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            let joined = must(timeout(Duration::from_secs(5), task).await);
            let daemon_result = must(joined);
            must(daemon_result);
        }
    }
}

impl Drop for RunningDaemon {
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
            sleep(Duration::from_millis(10)).await;
            return;
        }
        sleep(Duration::from_millis(10)).await;
    }
    panic!("socket did not become ready: {}", path.display());
}

async fn raw_exchange(path: &Path, request: ClientMessage) -> io::Result<ServerMessage> {
    let mut stream = UnixStream::connect(path).await?;
    stream.ensure_current_user_peer()?;
    let bytes = request.encode_canonical().map_err(infer_error_to_io)?;
    write_frame(&mut stream, &bytes, MAX_FRAME_BYTES).await?;
    let response = read_frame(&mut stream, MAX_FRAME_BYTES).await?;
    ServerMessage::decode_canonical(&response).map_err(infer_error_to_io)
}

fn assert_error(response: ServerMessage, expected: &str) {
    match response {
        ServerMessage::Error { code } => assert_eq!(code, expected),
        other => panic!("expected error {expected}, received {other:?}"),
    }
}

fn current_time_ms() -> u64 {
    let elapsed = must(SystemTime::now().duration_since(UNIX_EPOCH));
    must(u64::try_from(elapsed.as_millis()))
}

#[test]
fn daemon_configuration_rejects_invalid_resource_bounds() {
    let temp = must(TempDir::new());
    let mut invalid = config(&temp);
    invalid.max_connections = 0;
    assert!(invalid.validate().is_err());
    let mut invalid = config(&temp);
    invalid.max_receipt_files = 0;
    assert!(invalid.validate().is_err());
    let mut invalid = config(&temp);
    invalid.max_receipt_bytes = 1;
    assert!(invalid.validate().is_err());
    let mut invalid = config(&temp);
    invalid.frame_read_timeout = Duration::ZERO;
    assert!(invalid.validate().is_err());
}

#[tokio::test]
async fn same_uid_public_peer_cannot_publish_worker_or_operator_messages() {
    let temp = must(TempDir::new());
    let config = config(&temp);
    let mut daemon = RunningDaemon::start(config.clone()).await;
    let request_id = must(RequestId::parse("request-role-denial"));
    let privileged = [
        ClientMessage::Start {
            request_id: request_id.clone(),
            request_generation: 1,
            backend_generation: 1,
        },
        ClientMessage::Token {
            request_id: request_id.clone(),
            request_generation: 1,
            backend_generation: 1,
            sequence: 1,
            token_digest: digest('d'),
            token_byte_length: 1,
        },
        ClientMessage::Complete {
            request_id: request_id.clone(),
            request_generation: 1,
            backend_generation: 1,
            sequence: 1,
            result_digest: digest('e'),
            output_tokens: 1,
        },
        ClientMessage::RestartBackend {
            expected_generation: 1,
        },
    ];
    for message in privileged {
        assert_error(
            must(raw_exchange(&config.socket_path, message).await),
            "INF_ROLE_NOT_AUTHORIZED",
        );
    }
    assert!(matches!(
        must(
            raw_exchange(
                &config.socket_path,
                ClientMessage::Ping { nonce: 91 },
            )
            .await
        ),
        ServerMessage::Pong { nonce: 91 }
    ));
    daemon.stop().await;
}

#[tokio::test]
async fn receipt_is_memory_bounded_and_recovered_across_restart() {
    let temp = must(TempDir::new());
    let config = config(&temp);
    let mut first = RunningDaemon::start(config.clone()).await;
    let request_id = must(RequestId::parse("request-recovery"));
    let admitted = must(
        raw_exchange(
            &config.socket_path,
            ClientMessage::Admit(request(digest('a'), request_id.as_str(), u64::MAX)),
        )
        .await,
    );
    let generation = match admitted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    let receipt = match must(
        raw_exchange(
            &config.socket_path,
            ClientMessage::Cancel {
                request_id: request_id.clone(),
                request_generation: 1,
                cancel_generation: 1,
                backend_generation: generation,
            },
        )
        .await,
    ) {
        ServerMessage::Receipt(receipt) => receipt,
        other => panic!("unexpected cancel response: {other:?}"),
    };
    assert_eq!(receipt.terminal_state, LifecycleState::Cancelled);
    match must(raw_exchange(&config.socket_path, ClientMessage::Snapshot).await) {
        ServerMessage::Snapshot(snapshot) => assert_eq!(snapshot.terminal_receipts, 0),
        other => panic!("unexpected snapshot response: {other:?}"),
    }
    first.stop().await;

    let mut second = RunningDaemon::start(config.clone()).await;
    let recovered = must(
        raw_exchange(
            &config.socket_path,
            ClientMessage::GetReceipt {
                request_id: request_id.clone(),
                request_generation: 1,
                backend_generation: generation,
                minimum_sequence: receipt.last_sequence,
            },
        )
        .await,
    );
    assert_eq!(recovered, ServerMessage::Receipt(receipt));
    assert_error(
        must(
            raw_exchange(
                &config.socket_path,
                ClientMessage::Admit(request(digest('a'), request_id.as_str(), u64::MAX)),
            )
            .await
        ),
        "INF_DUPLICATE_REQUEST",
    );
    second.stop().await;

    let mut entries = must(fs::read_dir(&config.receipt_dir).await);
    let mut files = 0usize;
    while must(entries.next_entry().await).is_some() {
        files += 1;
    }
    assert_eq!(files, 1);
}

#[tokio::test]
async fn corrupt_receipt_store_fails_closed_on_startup() {
    let temp = must(TempDir::new());
    let config = config(&temp);
    must(codex_uds::prepare_private_socket_directory(&config.receipt_dir).await);
    must(fs::write(config.receipt_dir.join("receipt-corrupt.cbor"), b"not-cbor").await);
    let error = serve_with_shutdown(config, std::future::pending::<()>())
        .await
        .expect_err("corrupt receipt store must fail startup");
    assert_eq!(error.kind(), ErrorKind::InvalidData);
}

#[tokio::test]
async fn partial_frame_times_out_and_releases_connection_permit() {
    let temp = must(TempDir::new());
    let mut config = config(&temp);
    config.max_connections = 1;
    config.frame_read_timeout = Duration::from_millis(50);
    let mut daemon = RunningDaemon::start(config.clone()).await;
    let mut stalled = must(UnixStream::connect(&config.socket_path).await);
    must(stalled.write_all(&[0]).await);
    sleep(Duration::from_millis(150)).await;
    drop(stalled);
    assert!(matches!(
        must(
            raw_exchange(
                &config.socket_path,
                ClientMessage::Ping { nonce: 73 },
            )
            .await
        ),
        ServerMessage::Pong { nonce: 73 }
    ));
    daemon.stop().await;
}

#[tokio::test]
async fn deadline_sweep_persists_failure_and_releases_capacity() {
    let temp = must(TempDir::new());
    let mut config = config(&temp);
    config.controller.max_queue = 1;
    config.controller.max_per_tenant = 1;
    config.deadline_sweep_interval = Duration::from_millis(10);
    let mut daemon = RunningDaemon::start(config.clone()).await;
    let request_id = must(RequestId::parse("request-deadline"));
    let admitted = must(
        raw_exchange(
            &config.socket_path,
            ClientMessage::Admit(request(
                digest('a'),
                request_id.as_str(),
                current_time_ms() + 75,
            )),
        )
        .await,
    );
    let generation = match admitted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    let mut terminal = None;
    for _ in 0..50 {
        sleep(Duration::from_millis(10)).await;
        match must(
            raw_exchange(
                &config.socket_path,
                ClientMessage::GetReceipt {
                    request_id: request_id.clone(),
                    request_generation: 1,
                    backend_generation: generation,
                    minimum_sequence: 2,
                },
            )
            .await,
        ) {
            ServerMessage::Receipt(receipt) => {
                terminal = Some(receipt);
                break;
            }
            ServerMessage::Error { code }
                if code == "INF_REQUEST_NOT_TERMINAL" || code == "INF_UNKNOWN_REQUEST" => {}
            other => panic!("unexpected receipt polling response: {other:?}"),
        }
    }
    let receipt = terminal.expect("deadline sweep must create a terminal receipt");
    assert_eq!(receipt.terminal_state, LifecycleState::FailedClosed);
    assert!(!receipt.forced_worker_termination);
    assert!(matches!(
        must(
            raw_exchange(
                &config.socket_path,
                ClientMessage::Admit(request(
                    digest('a'),
                    "request-after-deadline",
                    u64::MAX,
                )),
            )
            .await
        ),
        ServerMessage::Accepted(_)
    ));
    daemon.stop().await;
}

#[tokio::test]
async fn receipt_store_budget_exhaustion_stops_daemon_fail_closed() {
    let temp = must(TempDir::new());
    let mut config = config(&temp);
    config.max_receipt_files = 1;
    let mut daemon = RunningDaemon::start(config.clone()).await;
    for index in 0..2 {
        let request_id = must(RequestId::parse(&format!("request-budget-{index}")));
        let admitted = must(
            raw_exchange(
                &config.socket_path,
                ClientMessage::Admit(request(digest('a'), request_id.as_str(), u64::MAX)),
            )
            .await,
        );
        let generation = match admitted {
            ServerMessage::Accepted(event) => event.backend_generation,
            other => panic!("unexpected admission response: {other:?}"),
        };
        let result = raw_exchange(
            &config.socket_path,
            ClientMessage::Cancel {
                request_id,
                request_generation: 1,
                cancel_generation: 1,
                backend_generation: generation,
            },
        )
        .await;
        if index == 0 {
            assert!(matches!(must(result), ServerMessage::Receipt(_)));
        } else {
            assert!(result.is_err());
        }
    }

    let task = match daemon.task.take() {
        Some(task) => task,
        None => panic!("daemon task must be present"),
    };
    let joined = must(timeout(Duration::from_secs(5), task).await);
    let daemon_result = must(joined);
    let error = match daemon_result {
        Ok(()) => panic!("receipt budget exhaustion must stop the daemon"),
        Err(error) => error,
    };
    assert_eq!(error.kind(), ErrorKind::Other);
    let _ = daemon.shutdown.take();
}
