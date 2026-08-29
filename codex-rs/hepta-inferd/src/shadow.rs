//! Qualification-only INF-2A adapter admission shadow daemon.
//!
//! This target composes the exact-tuple adapter capability registry with the
//! generation-fenced INF-1 controller. It exposes only an owner-local UDS,
//! receives digest-only requests, performs no implicit model installation or
//! fallback, and rejects unsupported capabilities before any backend dispatch.

use std::collections::HashSet;
use std::env;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_infer_core::AdapterId;
use codex_hepta_infer_core::AdapterRegistry;
use codex_hepta_infer_core::AuthoritySnapshot;
use codex_hepta_infer_core::ClientMessage;
use codex_hepta_infer_core::ControllerConfig;
use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::DispatchRequirements;
use codex_hepta_infer_core::EventFence;
use codex_hepta_infer_core::ExactAdapterTuple;
use codex_hepta_infer_core::FallbackPolicy;
use codex_hepta_infer_core::InferError;
use codex_hepta_infer_core::MAX_FRAME_BYTES;
use codex_hepta_infer_core::PolicyProfile;
use codex_hepta_infer_core::QualifiedController;
use codex_hepta_infer_core::ServerMessage;
use codex_hepta_infer_core::TerminalReceipt;
use codex_uds::UnixListener;
use codex_uds::UnixStream;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

#[derive(Debug)]
enum ConnectionTaskError {
    Peer,
    Infrastructure(io::Error),
}

#[derive(Clone, Debug)]
pub struct ShadowDaemonConfig {
    pub socket_path: PathBuf,
    pub receipt_dir: PathBuf,
    pub controller: ControllerConfig,
    pub registry: AdapterRegistry,
    pub backend_generation: u64,
    pub max_frame_bytes: usize,
}

impl ShadowDaemonConfig {
    pub fn fixed_tuple(
        socket_path: PathBuf,
        receipt_dir: PathBuf,
        adapter: AdapterId,
        tuple_digest: Digest,
        policy: PolicyProfile,
        backend_generation: u64,
    ) -> io::Result<Self> {
        let tuple = match adapter {
            AdapterId::Ollama => ExactAdapterTuple::fixed_ollama_granite4_1b(tuple_digest.clone()),
            AdapterId::LmStudio => {
                ExactAdapterTuple::fixed_lmstudio_granite4_micro(tuple_digest.clone())
            }
        }
        .map_err(infer_error_to_io)?;
        let registry = AdapterRegistry::new([tuple], [policy], FallbackPolicy::closed())
            .map_err(infer_error_to_io)?;
        let mut registered_tuples = HashSet::new();
        registered_tuples.insert(tuple_digest);
        Ok(Self {
            socket_path,
            receipt_dir,
            controller: ControllerConfig {
                max_queue: 64,
                max_per_tenant: 8,
                registered_tuples,
                authority: AuthoritySnapshot::qualification_only_closed(),
            },
            registry,
            backend_generation,
            max_frame_bytes: MAX_FRAME_BYTES,
        })
    }

    pub fn validate(&self) -> io::Result<()> {
        self.controller.validate().map_err(infer_error_to_io)?;
        self.registry
            .validate_for_controller(&self.controller)
            .map_err(infer_error_to_io)?;
        if self.backend_generation == 0 {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "INF_SHADOW_BACKEND_GENERATION_INVALID",
            ));
        }
        if self.max_frame_bytes == 0 || self.max_frame_bytes > MAX_FRAME_BYTES {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "INF_SHADOW_FRAME_BOUND_INVALID",
            ));
        }
        let socket_parent = required_parent(&self.socket_path, "INF_SHADOW_SOCKET_PARENT_MISSING")?;
        let receipt_parent =
            required_parent(&self.receipt_dir, "INF_SHADOW_RECEIPT_PARENT_MISSING")?;
        if !self.socket_path.is_absolute()
            || !self.receipt_dir.is_absolute()
            || self.socket_path == self.receipt_dir
            || socket_parent == self.receipt_dir
            || receipt_parent == self.socket_path
        {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "INF_SHADOW_PATH_INVALID",
            ));
        }
        Ok(())
    }
}

pub async fn serve_forever(config: ShadowDaemonConfig) -> io::Result<()> {
    serve_with_shutdown(config, std::future::pending::<()>()).await
}

pub async fn serve_with_shutdown<F>(config: ShadowDaemonConfig, shutdown: F) -> io::Result<()>
where
    F: Future<Output = ()>,
{
    config.validate()?;
    let socket_parent = required_parent(&config.socket_path, "INF_SHADOW_SOCKET_PARENT_MISSING")?;
    codex_uds::prepare_private_socket_directory(socket_parent).await?;
    codex_uds::prepare_private_socket_directory(&config.receipt_dir).await?;
    let mut listener = bind_single_instance(&config.socket_path).await?;
    let socket_guard = SocketGuard::new(config.socket_path.clone());
    let controller = QualifiedController::new(
        config.controller.clone(),
        config.backend_generation,
        config.registry.clone(),
    )
    .map_err(infer_error_to_io)?;
    let shared = Arc::new(Mutex::new(controller));
    let receipt_store = Arc::new(ReceiptStore::new(config.receipt_dir.clone()));
    let mut tasks = JoinSet::new();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            () = &mut shutdown => break,
            accepted = listener.accept() => {
                let stream = accepted?;
                if stream.ensure_current_user_peer().is_err() {
                    continue;
                }
                let controller = Arc::clone(&shared);
                let receipt_store = Arc::clone(&receipt_store);
                let maximum = config.max_frame_bytes;
                tasks.spawn(async move {
                    handle_connection(stream, controller, receipt_store, maximum).await
                });
            }
            joined = tasks.join_next(), if !tasks.is_empty() => {
                match joined {
                    Some(Ok(Ok(()))) => {}
                    Some(Ok(Err(ConnectionTaskError::Peer))) => {}
                    Some(Ok(Err(ConnectionTaskError::Infrastructure(error)))) => return Err(error),
                    Some(Err(error)) => {
                        return Err(io::Error::other(format!(
                            "INF_SHADOW_CONNECTION_TASK_FAILED: {error}"
                        )));
                    }
                    None => {}
                }
            }
        }
    }

    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
    drop(socket_guard);
    Ok(())
}

async fn handle_connection(
    mut stream: UnixStream,
    controller: Arc<Mutex<QualifiedController>>,
    receipt_store: Arc<ReceiptStore>,
    max_frame_bytes: usize,
) -> Result<(), ConnectionTaskError> {
    let request = read_message(&mut stream, max_frame_bytes)
        .await
        .map_err(|_| ConnectionTaskError::Peer)?;
    let response = {
        let now_unix_ms = unix_time_ms().map_err(ConnectionTaskError::Infrastructure)?;
        let mut controller = controller.lock().await;
        dispatch(&mut controller, request, now_unix_ms)
    };
    persist_terminal_responses(&receipt_store, &response)
        .await
        .map_err(ConnectionTaskError::Infrastructure)?;
    write_message(&mut stream, &response, max_frame_bytes)
        .await
        .map_err(|_| ConnectionTaskError::Peer)?;
    stream
        .shutdown()
        .await
        .map_err(|_| ConnectionTaskError::Peer)
}

fn dispatch(
    controller: &mut QualifiedController,
    request: ClientMessage,
    now_unix_ms: u64,
) -> ServerMessage {
    let result = match request {
        ClientMessage::Ping { nonce } => return ServerMessage::Pong { nonce },
        ClientMessage::Admit(request) => controller
            .admit(request, now_unix_ms)
            .map(ServerMessage::Accepted),
        ClientMessage::Start {
            request_id,
            request_generation,
            backend_generation,
        } => controller
            .start(&request_id, request_generation, backend_generation)
            .map(ServerMessage::State),
        ClientMessage::Token {
            request_id,
            request_generation,
            backend_generation,
            sequence,
            token_digest,
            token_byte_length,
        } => controller
            .publish_token(
                EventFence {
                    request_id: &request_id,
                    request_generation,
                    backend_generation,
                    sequence,
                },
                &token_digest,
                token_byte_length,
            )
            .map(ServerMessage::State),
        ClientMessage::Complete {
            request_id,
            request_generation,
            backend_generation,
            sequence,
            result_digest,
            output_tokens,
        } => controller
            .complete(
                EventFence {
                    request_id: &request_id,
                    request_generation,
                    backend_generation,
                    sequence,
                },
                result_digest,
                output_tokens,
            )
            .map(ServerMessage::Receipt),
        ClientMessage::Cancel {
            request_id,
            request_generation,
            cancel_generation,
            backend_generation,
        } => controller
            .cancel(
                &request_id,
                request_generation,
                cancel_generation,
                backend_generation,
            )
            .map(ServerMessage::Receipt),
        ClientMessage::RestartBackend {
            expected_generation,
        } => controller
            .restart_backend(expected_generation)
            .map(|receipts| ServerMessage::Restarted {
                backend_generation: controller.backend_generation(),
                receipts,
            }),
        ClientMessage::Snapshot => return ServerMessage::Snapshot(controller.snapshot()),
    };
    result.unwrap_or_else(|error| ServerMessage::Error {
        code: error.code().to_owned(),
    })
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct ShadowClient {
    socket_path: PathBuf,
    max_frame_bytes: usize,
}

#[cfg(test)]
impl ShadowClient {
    pub fn new(socket_path: PathBuf, max_frame_bytes: usize) -> io::Result<Self> {
        if !socket_path.is_absolute() {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "INF_SHADOW_CLIENT_SOCKET_MUST_BE_ABSOLUTE",
            ));
        }
        if max_frame_bytes == 0 || max_frame_bytes > MAX_FRAME_BYTES {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "INF_SHADOW_CLIENT_FRAME_BOUND_INVALID",
            ));
        }
        Ok(Self {
            socket_path,
            max_frame_bytes,
        })
    }

    pub async fn exchange(&self, message: ClientMessage) -> io::Result<ServerMessage> {
        let mut stream = UnixStream::connect(&self.socket_path).await?;
        stream.ensure_current_user_peer()?;
        write_client_message(&mut stream, &message, self.max_frame_bytes).await?;
        read_server_message(&mut stream, self.max_frame_bytes).await
    }
}

async fn read_message(
    stream: &mut UnixStream,
    max_frame_bytes: usize,
) -> io::Result<ClientMessage> {
    let bytes = read_frame(stream, max_frame_bytes).await?;
    ClientMessage::decode_canonical(&bytes).map_err(infer_error_to_io)
}

#[cfg(test)]
async fn read_server_message(
    stream: &mut UnixStream,
    max_frame_bytes: usize,
) -> io::Result<ServerMessage> {
    let bytes = read_frame(stream, max_frame_bytes).await?;
    ServerMessage::decode_canonical(&bytes).map_err(infer_error_to_io)
}

async fn read_frame(stream: &mut UnixStream, max_frame_bytes: usize) -> io::Result<Vec<u8>> {
    let mut length_bytes = [0u8; 4];
    stream.read_exact(&mut length_bytes).await?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes))
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "INF_SHADOW_FRAME_LENGTH_INVALID"))?;
    if length == 0 || length > max_frame_bytes {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "INF_SHADOW_FRAME_LENGTH_OUT_OF_BOUNDS",
        ));
    }
    let mut bytes = vec![0u8; length];
    stream.read_exact(&mut bytes).await?;
    Ok(bytes)
}

#[cfg(test)]
async fn write_client_message(
    stream: &mut UnixStream,
    message: &ClientMessage,
    max_frame_bytes: usize,
) -> io::Result<()> {
    let bytes = message.encode_canonical().map_err(infer_error_to_io)?;
    write_frame(stream, &bytes, max_frame_bytes).await
}

async fn write_message(
    stream: &mut UnixStream,
    message: &ServerMessage,
    max_frame_bytes: usize,
) -> io::Result<()> {
    let bytes = message.encode_canonical().map_err(infer_error_to_io)?;
    write_frame(stream, &bytes, max_frame_bytes).await
}

async fn write_frame(
    stream: &mut UnixStream,
    bytes: &[u8],
    max_frame_bytes: usize,
) -> io::Result<()> {
    if bytes.is_empty() || bytes.len() > max_frame_bytes {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "INF_SHADOW_FRAME_OUT_OF_BOUNDS",
        ));
    }
    let length = u32::try_from(bytes.len())
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "INF_SHADOW_FRAME_TOO_LARGE"))?;
    stream.write_all(&length.to_be_bytes()).await?;
    stream.write_all(bytes).await?;
    stream.flush().await
}

async fn persist_terminal_responses(
    store: &ReceiptStore,
    response: &ServerMessage,
) -> io::Result<()> {
    match response {
        ServerMessage::Receipt(receipt) => store.persist(receipt).await,
        ServerMessage::Restarted { receipts, .. } => {
            for receipt in receipts {
                store.persist(receipt).await?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

struct ReceiptStore {
    root: PathBuf,
}

impl ReceiptStore {
    fn new(root: PathBuf) -> Self {
        Self { root }
    }

    async fn persist(&self, receipt: &TerminalReceipt) -> io::Result<()> {
        let path = self.root.join(format!(
            "receipt-{}-{}-{}.cbor",
            receipt.request_id, receipt.request_generation, receipt.last_sequence
        ));
        let bytes = ServerMessage::Receipt(receipt.clone())
            .encode_canonical()
            .map_err(infer_error_to_io)?;
        let mut options = fs::OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(unix)]
        {
            options.mode(0o600);
        }
        let mut file = options.open(path).await?;
        file.write_all(&bytes).await?;
        file.sync_all().await
    }
}

async fn bind_single_instance(socket_path: &Path) -> io::Result<UnixListener> {
    match fs::symlink_metadata(socket_path).await {
        Ok(_) => match UnixStream::connect(socket_path).await {
            Ok(_) => {
                return Err(io::Error::new(
                    ErrorKind::AddrInUse,
                    "INF_SHADOW_DAEMON_ALREADY_RUNNING",
                ));
            }
            Err(_) => {
                if codex_uds::is_stale_socket_path(socket_path).await? {
                    fs::remove_file(socket_path).await?;
                } else {
                    return Err(io::Error::new(
                        ErrorKind::AlreadyExists,
                        "INF_SHADOW_SOCKET_PATH_NOT_STALE_SOCKET",
                    ));
                }
            }
        },
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    let listener = UnixListener::bind(socket_path).await?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o600)).await?;
    }
    Ok(listener)
}

pub async fn run_from_env() -> io::Result<()> {
    let socket_path = required_path("HEPTA_INFER_SOCKET")?;
    let receipt_dir = required_path("HEPTA_INFER_RECEIPT_DIR")?;
    let tuple_digest = required_digest("HEPTA_INFER_MODEL_TUPLE_DIGEST")?;
    let policy_digest = required_digest("HEPTA_INFER_POLICY_DIGEST")?;
    let adapter = match required_string("HEPTA_INFER_SHADOW_ADAPTER")?.as_str() {
        "ollama" => AdapterId::Ollama,
        "lmstudio" => AdapterId::LmStudio,
        _ => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "HEPTA_INFER_SHADOW_ADAPTER_INVALID",
            ));
        }
    };
    let requirements = match required_string("HEPTA_INFER_POLICY_PROFILE")?.as_str() {
        "semantic_text" => DispatchRequirements::semantic_text(),
        "native_tool_call" => DispatchRequirements::native_tool_call(),
        "strict_sse" => DispatchRequirements::strict_sse(),
        "cancel_required" => DispatchRequirements::cancel_required(),
        _ => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "HEPTA_INFER_POLICY_PROFILE_INVALID",
            ));
        }
    };
    let generation = required_string("HEPTA_INFER_BACKEND_GENERATION")?
        .parse::<u64>()
        .map_err(|_| {
            io::Error::new(
                ErrorKind::InvalidInput,
                "HEPTA_INFER_BACKEND_GENERATION_INVALID",
            )
        })?;
    let config = ShadowDaemonConfig::fixed_tuple(
        socket_path,
        receipt_dir,
        adapter,
        tuple_digest,
        PolicyProfile::new(policy_digest, requirements),
        generation,
    )?;
    serve_forever(config).await
}

fn required_string(name: &str) -> io::Result<String> {
    env::var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, format!("{name}_REQUIRED")))
}

fn required_path(name: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(required_string(name)?);
    if path.is_absolute() {
        Ok(path)
    } else {
        Err(io::Error::new(
            ErrorKind::InvalidInput,
            format!("{name}_MUST_BE_ABSOLUTE"),
        ))
    }
}

fn required_digest(name: &str) -> io::Result<Digest> {
    Digest::parse(&required_string(name)?)
        .map_err(|error| io::Error::new(ErrorKind::InvalidInput, error))
}

fn required_parent<'a>(path: &'a Path, code: &'static str) -> io::Result<&'a Path> {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, code))
}

fn unix_time_ms() -> io::Result<u64> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::other("INF_SHADOW_SYSTEM_TIME_BEFORE_EPOCH"))?;
    u64::try_from(duration.as_millis())
        .map_err(|_| io::Error::other("INF_SHADOW_SYSTEM_TIME_OVERFLOW"))
}

fn infer_error_to_io(error: InferError) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, error)
}

struct SocketGuard {
    path: PathBuf,
}

impl SocketGuard {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for SocketGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use codex_hepta_infer_core::AgentId;
    use codex_hepta_infer_core::InferenceRequest;
    use codex_hepta_infer_core::RequestId;
    use codex_hepta_infer_core::RequestIdentity;
    use codex_hepta_infer_core::ResourceBudgetId;
    use codex_hepta_infer_core::TaskId;
    use codex_hepta_infer_core::TenantId;
    use codex_hepta_infer_core::WorkspaceId;
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

    fn request(tuple: Digest, policy: Digest, name: &str) -> InferenceRequest {
        InferenceRequest {
            identity: RequestIdentity {
                tenant_id: must(TenantId::parse("tenant-a")),
                workspace_id: must(WorkspaceId::parse("workspace-a")),
                agent_id: must(AgentId::parse("agent-a")),
                task_id: must(TaskId::parse("task-a")),
                request_id: must(RequestId::parse(name)),
            },
            agent_generation: 1,
            request_generation: 1,
            cancel_generation: 0,
            deadline_unix_ms: u64::MAX,
            model_tuple_digest: tuple,
            policy_digest: policy,
            resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
            prompt_digest: digest('c'),
            prompt_byte_length: 12,
            output_token_limit: 32,
            authority: AuthoritySnapshot::qualification_only_closed(),
        }
    }

    struct Harness {
        _temp: TempDir,
        config: ShadowDaemonConfig,
        shutdown: Option<oneshot::Sender<()>>,
        task: Option<JoinHandle<io::Result<()>>>,
    }

    impl Harness {
        async fn start(adapter: AdapterId, requirements: DispatchRequirements) -> Self {
            let temp = must(TempDir::new());
            let socket_path = temp.path().join("socket").join("infer-shadow.sock");
            let receipt_dir = temp.path().join("receipts");
            let config = must(ShadowDaemonConfig::fixed_tuple(
                socket_path,
                receipt_dir,
                adapter,
                digest('a'),
                PolicyProfile::new(digest('b'), requirements),
                7,
            ));
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

        fn client(&self) -> ShadowClient {
            must(ShadowClient::new(
                self.config.socket_path.clone(),
                self.config.max_frame_bytes,
            ))
        }

        async fn stop(&mut self) {
            if let Some(shutdown) = self.shutdown.take() {
                let _ = shutdown.send(());
            }
            if let Some(task) = self.task.take() {
                match must(timeout(Duration::from_secs(5), task).await) {
                    Ok(Ok(())) => {}
                    Ok(Err(error)) => panic!("shadow daemon failed: {error}"),
                    Err(error) => panic!("shadow daemon task failed: {error}"),
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
        panic!("shadow socket did not become ready: {}", path.display());
    }

    #[tokio::test]
    async fn semantic_shadow_path_is_owner_local_and_persists_digest_only_receipt() {
        let mut harness =
            Harness::start(AdapterId::Ollama, DispatchRequirements::semantic_text()).await;
        let client = harness.client();
        assert_eq!(
            must(client.exchange(ClientMessage::Ping { nonce: 9 }).await),
            ServerMessage::Pong { nonce: 9 }
        );
        let accepted = must(
            client
                .exchange(ClientMessage::Admit(request(
                    digest('a'),
                    digest('b'),
                    "request-semantic",
                )))
                .await,
        );
        let backend_generation = match accepted {
            ServerMessage::Accepted(event) => event.backend_generation,
            other => panic!("unexpected admission response: {other:?}"),
        };
        let request_id = must(RequestId::parse("request-semantic"));
        let started = must(
            client
                .exchange(ClientMessage::Start {
                    request_id: request_id.clone(),
                    request_generation: 1,
                    backend_generation,
                })
                .await,
        );
        assert!(matches!(started, ServerMessage::State(_)));
        let receipt = must(
            client
                .exchange(ClientMessage::Complete {
                    request_id,
                    request_generation: 1,
                    backend_generation,
                    sequence: 3,
                    result_digest: digest('d'),
                    output_tokens: 4,
                })
                .await,
        );
        assert!(matches!(receipt, ServerMessage::Receipt(_)));
        let mut entries = must(fs::read_dir(&harness.config.receipt_dir).await);
        let entry = match must(entries.next_entry().await) {
            Some(entry) => entry,
            None => panic!("shadow terminal receipt was not persisted"),
        };
        let bytes = must(fs::read(entry.path()).await);
        assert!(bytes.len() <= MAX_FRAME_BYTES);
        assert!(!String::from_utf8_lossy(&bytes).contains("prompt"));
        harness.stop().await;
    }

    #[tokio::test]
    async fn provider_cancel_requirement_is_rejected_before_queueing() {
        let mut harness =
            Harness::start(AdapterId::Ollama, DispatchRequirements::cancel_required()).await;
        let client = harness.client();
        assert_eq!(
            must(
                client
                    .exchange(ClientMessage::Admit(request(
                        digest('a'),
                        digest('b'),
                        "request-cancel-required",
                    )))
                    .await,
            ),
            ServerMessage::Error {
                code: "INF_ADAPTER_PROVIDER_CANCEL_UNSUPPORTED".to_owned(),
            }
        );
        let snapshot = must(client.exchange(ClientMessage::Snapshot).await);
        match snapshot {
            ServerMessage::Snapshot(snapshot) => assert_eq!(snapshot.queued_requests, 0),
            other => panic!("unexpected snapshot response: {other:?}"),
        }
        harness.stop().await;
    }

    #[tokio::test]
    async fn lmstudio_tool_call_requirement_is_rejected_before_queueing() {
        let mut harness = Harness::start(
            AdapterId::LmStudio,
            DispatchRequirements::native_tool_call(),
        )
        .await;
        let client = harness.client();
        assert_eq!(
            must(
                client
                    .exchange(ClientMessage::Admit(request(
                        digest('a'),
                        digest('b'),
                        "request-tool-required",
                    )))
                    .await,
            ),
            ServerMessage::Error {
                code: "INF_ADAPTER_TOOL_CALL_UNSUPPORTED".to_owned(),
            }
        );
        let snapshot = must(client.exchange(ClientMessage::Snapshot).await);
        match snapshot {
            ServerMessage::Snapshot(snapshot) => assert_eq!(snapshot.queued_requests, 0),
            other => panic!("unexpected snapshot response: {other:?}"),
        }
        harness.stop().await;
    }

    #[test]
    fn client_rejects_relative_socket_and_unbounded_frames() {
        assert!(ShadowClient::new(PathBuf::from("relative.sock"), MAX_FRAME_BYTES).is_err());
        assert!(ShadowClient::new(PathBuf::from("/tmp/infer.sock"), 0).is_err());
        assert!(ShadowClient::new(PathBuf::from("/tmp/infer.sock"), MAX_FRAME_BYTES + 1).is_err());
    }
}
