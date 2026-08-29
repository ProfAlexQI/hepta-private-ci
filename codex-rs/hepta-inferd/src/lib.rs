//! Qualification-only UDS control plane for the minimal Hepta inference controller.
//!
//! The daemon exposes no TCP listener, model downloader, remote endpoint, effect
//! executor, Memory/KG writer, or production authority. It accepts only same-user
//! Unix-domain-socket peers and canonical bounded CBOR frames.

use std::collections::HashSet;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_infer_core::AuthoritySnapshot;
use codex_hepta_infer_core::ClientMessage;
use codex_hepta_infer_core::Controller;
use codex_hepta_infer_core::ControllerConfig;
use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::EventFence;
use codex_hepta_infer_core::InferError;
use codex_hepta_infer_core::MAX_FRAME_BYTES;
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
pub struct DaemonConfig {
    pub socket_path: PathBuf,
    pub receipt_dir: PathBuf,
    pub generation_file: PathBuf,
    pub controller: ControllerConfig,
    pub max_frame_bytes: usize,
}

impl DaemonConfig {
    pub fn qualification_only(
        socket_path: PathBuf,
        receipt_dir: PathBuf,
        model_tuple_digest: Digest,
    ) -> Self {
        let mut tuples = HashSet::new();
        tuples.insert(model_tuple_digest);
        let socket_dir = socket_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        Self {
            socket_path,
            receipt_dir,
            generation_file: socket_dir.join("backend.generation"),
            controller: ControllerConfig {
                max_queue: 64,
                max_per_tenant: 8,
                registered_tuples: tuples,
                authority: AuthoritySnapshot::qualification_only_closed(),
            },
            max_frame_bytes: MAX_FRAME_BYTES,
        }
    }

    pub fn validate(&self) -> io::Result<()> {
        self.controller.validate().map_err(infer_error_to_io)?;
        if self.max_frame_bytes == 0 || self.max_frame_bytes > MAX_FRAME_BYTES {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "INF_DAEMON_FRAME_BOUND_INVALID",
            ));
        }
        let socket_dir = required_parent(&self.socket_path, "INF_SOCKET_PARENT_MISSING")?;
        let receipt_parent = required_parent(&self.receipt_dir, "INF_RECEIPT_PARENT_MISSING")?;
        if !self.socket_path.is_absolute()
            || !self.receipt_dir.is_absolute()
            || !self.generation_file.is_absolute()
            || socket_dir == receipt_parent && self.socket_path == self.receipt_dir
        {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "INF_DAEMON_PATH_INVALID",
            ));
        }
        Ok(())
    }
}

pub async fn serve_forever(config: DaemonConfig) -> io::Result<()> {
    serve_with_shutdown(config, std::future::pending::<()>()).await
}

pub async fn serve_with_shutdown<F>(config: DaemonConfig, shutdown: F) -> io::Result<()>
where
    F: Future<Output = ()>,
{
    config.validate()?;
    let socket_dir = required_parent(&config.socket_path, "INF_SOCKET_PARENT_MISSING")?;
    codex_uds::prepare_private_socket_directory(socket_dir).await?;
    codex_uds::prepare_private_socket_directory(&config.receipt_dir).await?;

    let mut listener = bind_single_instance(&config.socket_path).await?;
    let socket_guard = SocketGuard::new(config.socket_path.clone());
    let backend_generation = next_backend_generation(&config.generation_file).await?;
    let controller = Controller::new(config.controller.clone(), backend_generation)
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
                        return Err(io::Error::other(format!("INF_CONNECTION_TASK_FAILED: {error}")));
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
    controller: Arc<Mutex<Controller>>,
    receipt_store: Arc<ReceiptStore>,
    max_frame_bytes: usize,
) -> Result<(), ConnectionTaskError> {
    let request = read_message(&mut stream, max_frame_bytes)
        .await
        .map_err(|_| ConnectionTaskError::Peer)?;
    let creates_terminal_receipt = request.creates_terminal_receipt();
    let response = {
        let now_unix_ms = unix_time_ms().map_err(ConnectionTaskError::Infrastructure)?;
        let mut controller = controller.lock().await;
        dispatch(&mut controller, request, now_unix_ms)
    };
    if creates_terminal_receipt {
        persist_terminal_responses(&receipt_store, &response)
            .await
            .map_err(ConnectionTaskError::Infrastructure)?;
    }
    write_message(&mut stream, &response, max_frame_bytes)
        .await
        .map_err(|_| ConnectionTaskError::Peer)?;
    stream
        .shutdown()
        .await
        .map_err(|_| ConnectionTaskError::Peer)
}

fn dispatch(
    controller: &mut Controller,
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
        ClientMessage::GetReceipt {
            request_id,
            request_generation,
            backend_generation,
            minimum_sequence,
        } => controller
            .terminal_receipt_fenced(
                &request_id,
                request_generation,
                backend_generation,
                minimum_sequence,
            )
            .cloned()
            .map(ServerMessage::Receipt),
        ClientMessage::Snapshot => return ServerMessage::Snapshot(controller.snapshot()),
    };
    result.unwrap_or_else(|error| ServerMessage::Error {
        code: error.code().to_owned(),
    })
}

async fn read_message(
    stream: &mut UnixStream,
    max_frame_bytes: usize,
) -> io::Result<ClientMessage> {
    let mut length_bytes = [0u8; 4];
    stream.read_exact(&mut length_bytes).await?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes))
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "INF_FRAME_LENGTH_INVALID"))?;
    if length == 0 || length > max_frame_bytes {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "INF_FRAME_LENGTH_OUT_OF_BOUNDS",
        ));
    }
    let mut bytes = vec![0u8; length];
    stream.read_exact(&mut bytes).await?;
    ClientMessage::decode_canonical(&bytes).map_err(infer_error_to_io)
}

async fn write_message(
    stream: &mut UnixStream,
    message: &ServerMessage,
    max_frame_bytes: usize,
) -> io::Result<()> {
    let bytes = message.encode_canonical().map_err(infer_error_to_io)?;
    if bytes.is_empty() || bytes.len() > max_frame_bytes {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "INF_RESPONSE_FRAME_OUT_OF_BOUNDS",
        ));
    }
    let length = u32::try_from(bytes.len())
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "INF_RESPONSE_FRAME_TOO_LARGE"))?;
    stream.write_all(&length.to_be_bytes()).await?;
    stream.write_all(&bytes).await?;
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
        let file_name = format!(
            "receipt-{}-{}-{}.cbor",
            receipt.request_id, receipt.request_generation, receipt.last_sequence
        );
        let path = self.root.join(file_name);
        let bytes = ServerMessage::Receipt(receipt.clone())
            .encode_canonical()
            .map_err(infer_error_to_io)?;
        let mut options = fs::OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(unix)]
        {
            options.mode(0o600);
        }
        let mut file = options.open(&path).await?;
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
                    "INF_DAEMON_ALREADY_RUNNING",
                ));
            }
            Err(_) => {
                if codex_uds::is_stale_socket_path(socket_path).await? {
                    fs::remove_file(socket_path).await?;
                } else {
                    return Err(io::Error::new(
                        ErrorKind::AlreadyExists,
                        "INF_SOCKET_PATH_NOT_STALE_SOCKET",
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

async fn next_backend_generation(path: &Path) -> io::Result<u64> {
    let current = match fs::read_to_string(path).await {
        Ok(text) => text
            .trim()
            .parse::<u64>()
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "INF_GENERATION_FILE_INVALID"))?,
        Err(error) if error.kind() == ErrorKind::NotFound => 0,
        Err(error) => return Err(error),
    };
    let next = current
        .checked_add(1)
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "INF_GENERATION_OVERFLOW"))?;
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut options = fs::OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        options.mode(0o600);
    }
    let mut file = options.open(&temporary).await?;
    file.write_all(format!("{next}\n").as_bytes()).await?;
    file.sync_all().await?;
    drop(file);
    fs::rename(&temporary, path).await?;
    Ok(next)
}

fn required_parent<'a>(path: &'a Path, code: &'static str) -> io::Result<&'a Path> {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, code))
}

fn unix_time_ms() -> io::Result<u64> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::other("INF_SYSTEM_TIME_BEFORE_EPOCH"))?;
    u64::try_from(duration.as_millis()).map_err(|_| io::Error::other("INF_SYSTEM_TIME_OVERFLOW"))
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
mod tests;
