use std::io;
use std::path::Path;
use std::time::Duration;

use codex_hepta_infer_core::AgentId;
use codex_hepta_infer_core::AuthoritySnapshot;
use codex_hepta_infer_core::ClientMessage;
use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::InferenceRequest;
use codex_hepta_infer_core::RequestId;
use codex_hepta_infer_core::RequestIdentity;
use codex_hepta_infer_core::ResourceBudgetId;
use codex_hepta_infer_core::ServerMessage;
use codex_hepta_infer_core::TaskId;
use codex_hepta_infer_core::TenantId;
use codex_hepta_infer_core::WorkspaceId;
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

fn request(tuple: Digest) -> InferenceRequest {
    InferenceRequest {
        identity: RequestIdentity {
            tenant_id: must(TenantId::parse("tenant-a")),
            workspace_id: must(WorkspaceId::parse("workspace-a")),
            agent_id: must(AgentId::parse("agent-a")),
            task_id: must(TaskId::parse("task-a")),
            request_id: must(RequestId::parse("request-a")),
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
        let socket_dir = temp.path().join("socket");
        let socket_path = socket_dir.join("infer.sock");
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

async fn exchange(path: &Path, message: ClientMessage) -> ServerMessage {
    let mut stream = must(UnixStream::connect(path).await);
    let payload = must(message.encode_canonical());
    let length = must(u32::try_from(payload.len()));
    must(stream.write_all(&length.to_be_bytes()).await);
    must(stream.write_all(&payload).await);
    must(stream.flush().await);
    let mut length_bytes = [0u8; 4];
    must(stream.read_exact(&mut length_bytes).await);
    let response_length = must(usize::try_from(u32::from_be_bytes(length_bytes)));
    let mut response = vec![0u8; response_length];
    must(stream.read_exact(&mut response).await);
    must(ServerMessage::decode_canonical(&response))
}

#[tokio::test]
async fn same_user_uds_round_trip_and_terminal_receipt_are_bounded() {
    let mut harness = Harness::start().await;
    assert_eq!(
        exchange(
            &harness.config.socket_path,
            ClientMessage::Ping { nonce: 7 }
        )
        .await,
        ServerMessage::Pong { nonce: 7 }
    );

    let accepted = exchange(
        &harness.config.socket_path,
        ClientMessage::Admit(request(digest('a'))),
    )
    .await;
    let backend_generation = match accepted {
        ServerMessage::Accepted(event) => event.backend_generation,
        other => panic!("unexpected admission response: {other:?}"),
    };
    let request_id = must(RequestId::parse("request-a"));
    let cancelled = exchange(
        &harness.config.socket_path,
        ClientMessage::Cancel {
            request_id: request_id.clone(),
            request_generation: 1,
            cancel_generation: 1,
            backend_generation,
        },
    )
    .await;
    let cancelled_receipt = match cancelled {
        ServerMessage::Receipt(receipt) => {
            assert_eq!(receipt.cancel_generation, 1);
            assert!(receipt.result_digest.is_none());
            assert_eq!(
                receipt.authority,
                AuthoritySnapshot::qualification_only_closed()
            );
            receipt
        }
        other => panic!("unexpected cancellation response: {other:?}"),
    };

    let queried = exchange(
        &harness.config.socket_path,
        ClientMessage::GetReceipt {
            request_id: request_id.clone(),
            request_generation: 1,
            backend_generation,
            minimum_sequence: cancelled_receipt.last_sequence,
        },
    )
    .await;
    assert_eq!(queried, ServerMessage::Receipt(cancelled_receipt.clone()));
    assert_eq!(
        exchange(
            &harness.config.socket_path,
            ClientMessage::GetReceipt {
                request_id,
                request_generation: 1,
                backend_generation,
                minimum_sequence: cancelled_receipt.last_sequence + 1,
            },
        )
        .await,
        ServerMessage::Error {
            code: "INF_RECEIPT_SEQUENCE_NOT_REACHED".to_owned(),
        }
    );

    let mut entries = must(tokio::fs::read_dir(&harness.config.receipt_dir).await);
    let mut receipt_files = Vec::new();
    while let Some(entry) = must(entries.next_entry().await) {
        receipt_files.push(entry.path());
    }
    assert_eq!(
        receipt_files.len(),
        1,
        "read-only query must not persist twice"
    );
    let bytes = must(tokio::fs::read(&receipt_files[0]).await);
    assert!(bytes.len() <= codex_hepta_infer_core::MAX_FRAME_BYTES);
    assert!(!String::from_utf8_lossy(&bytes).contains("prompt"));
    harness.stop().await;
}

#[tokio::test]
async fn second_controller_instance_fails_closed() {
    let mut harness = Harness::start().await;
    let (_shutdown, receiver) = oneshot::channel::<()>();
    let second = timeout(
        Duration::from_secs(5),
        serve_with_shutdown(harness.config.clone(), async {
            let _ = receiver.await;
        }),
    )
    .await;
    let error = match must(second) {
        Ok(()) => panic!("a second controller instance unexpectedly started"),
        Err(error) => error,
    };
    assert_eq!(error.kind(), io::ErrorKind::AddrInUse);
    harness.stop().await;
}

#[tokio::test]
async fn controlled_restart_increments_backend_generation() {
    let mut harness = Harness::start().await;
    let first = exchange(&harness.config.socket_path, ClientMessage::Snapshot).await;
    let first_generation = match first {
        ServerMessage::Snapshot(snapshot) => snapshot.backend_generation,
        other => panic!("unexpected first snapshot: {other:?}"),
    };
    harness.stop().await;

    let (shutdown, receiver) = oneshot::channel();
    let config = harness.config.clone();
    let socket = config.socket_path.clone();
    let task = tokio::spawn(async move {
        serve_with_shutdown(config, async {
            let _ = receiver.await;
        })
        .await
    });
    wait_for_socket(&socket).await;
    let second = exchange(&socket, ClientMessage::Snapshot).await;
    let second_generation = match second {
        ServerMessage::Snapshot(snapshot) => snapshot.backend_generation,
        other => panic!("unexpected second snapshot: {other:?}"),
    };
    assert_eq!(second_generation, first_generation + 1);
    let _ = shutdown.send(());
    match must(timeout(Duration::from_secs(5), task).await) {
        Ok(Ok(())) => {}
        Ok(Err(error)) => panic!("restarted daemon failed: {error}"),
        Err(error) => panic!("restarted daemon task failed: {error}"),
    }
}

#[tokio::test]
async fn truncated_connection_is_connection_local() {
    let mut harness = Harness::start().await;
    let stream = must(UnixStream::connect(&harness.config.socket_path).await);
    drop(stream);
    sleep(Duration::from_millis(25)).await;
    assert_eq!(
        exchange(
            &harness.config.socket_path,
            ClientMessage::Ping { nonce: 19 }
        )
        .await,
        ServerMessage::Pong { nonce: 19 }
    );
    harness.stop().await;
}
