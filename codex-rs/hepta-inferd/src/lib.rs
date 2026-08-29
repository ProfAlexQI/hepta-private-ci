//! Qualification-only UDS control plane for the minimal Hepta inference controller.
//!
//! The daemon exposes no TCP listener, model downloader, remote endpoint, effect
//! executor, Memory/KG writer, or production authority. The public owner-local
//! socket accepts only unprivileged client operations. Worker events and operator
//! commands require future private capability channels and fail closed here.

use std::collections::HashMap;
use std::collections::HashSet;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_infer_core::AuthoritySnapshot;
use codex_hepta_infer_core::ClientMessage;
use codex_hepta_infer_core::Controller;
use codex_hepta_infer_core::ControllerConfig;
use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::InferError;
use codex_hepta_infer_core::MAX_FRAME_BYTES;
use codex_hepta_infer_core::MessageRole;
use codex_hepta_infer_core::RequestId;
use codex_hepta_infer_core::ServerMessage;
use codex_hepta_infer_core::TerminalReceipt;
use codex_uds::UnixListener;
use codex_uds::UnixStream;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::sync::OwnedSemaphorePermit;
use tokio::sync::RwLock;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time;
use tokio::time::MissedTickBehavior;

const DEFAULT_MAX_CONNECTIONS: usize = 64;
const DEFAULT_MAX_RECEIPT_FILES: usize = 4096;
const DEFAULT_MAX_RECEIPT_BYTES: u64 = 64 * 1024 * 1024;
const DEFAULT_FRAME_READ_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_RESPONSE_WRITE_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_DEADLINE_SWEEP_INTERVAL: Duration = Duration::from_millis(250);

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
    pub max_connections: usize,
    pub max_receipt_files: usize,
    pub max_receipt_bytes: u64,
    pub frame_read_timeout: Duration,
    pub response_write_timeout: Duration,
    pub deadline_sweep_interval: Duration,
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
            max_connections: DEFAULT_MAX_CONNECTIONS,
            max_receipt_files: DEFAULT_MAX_RECEIPT_FILES,
            max_receipt_bytes: DEFAULT_MAX_RECEIPT_BYTES,
            frame_read_timeout: DEFAULT_FRAME_READ_TIMEOUT,
            response_write_timeout: DEFAULT_RESPONSE_WRITE_TIMEOUT,
            deadline_sweep_interval: DEFAULT_DEADLINE_SWEEP_INTERVAL,
        }
    }

    pub fn validate(&self) -> io::Result<()> {
        self.controller.validate().map_err(infer_error_to_io)?;
        if self.max_frame_bytes == 0 || self.max_frame_bytes > MAX_FRAME_BYTES {
            return Err(invalid_input("INF_DAEMON_FRAME_BOUND_INVALID"));
        }
        let minimum_receipt_bytes = u64::try_from(self.max_frame_bytes)
            .map_err(|_| invalid_input("INF_DAEMON_FRAME_BOUND_INVALID"))?;
        if self.max_connections == 0
            || self.max_receipt_files == 0
            || self.max_receipt_bytes < minimum_receipt_bytes
            || self.frame_read_timeout.is_zero()
            || self.response_write_timeout.is_zero()
            || self.deadline_sweep_interval.is_zero()
        {
            return Err(invalid_input("INF_DAEMON_RESOURCE_BOUND_INVALID"));
        }
        let socket_dir = required_parent(&self.socket_path, "INF_SOCKET_PARENT_MISSING")?;
        let _receipt_parent = required_parent(&self.receipt_dir, "INF_RECEIPT_PARENT_MISSING")?;
        let generation_parent =
            required_parent(&self.generation_file, "INF_GENERATION_PARENT_MISSING")?;
        if !self.socket_path.is_absolute()
            || !self.receipt_dir.is_absolute()
            || !self.generation_file.is_absolute()
            || self.socket_path == self.receipt_dir
            || self.socket_path == self.generation_file
            || self.receipt_dir == self.generation_file
            || socket_dir != generation_parent
        {
            return Err(invalid_input("INF_DAEMON_PATH_INVALID"));
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
    let receipt_store = Arc::new(
        ReceiptStore::open(
            config.receipt_dir.clone(),
            config.max_receipt_files,
            config.max_receipt_bytes,
        )
        .await?,
    );
    let connections = Arc::new(Semaphore::new(config.max_connections));
    let mut tasks = JoinSet::new();
    let mut deadline_sweep = time::interval(config.deadline_sweep_interval);
    deadline_sweep.set_missed_tick_behavior(MissedTickBehavior::Delay);
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            () = &mut shutdown => break,
            _ = deadline_sweep.tick() => {
                run_deadline_sweep(&shared, &receipt_store).await?;
            }
            accepted = listener.accept() => {
                let stream = accepted?;
                if stream.ensure_current_user_peer().is_err() {
                    continue;
                }
                let permit = match Arc::clone(&connections).try_acquire_owned() {
                    Ok(permit) => permit,
                    Err(_) => continue,
                };
                let controller = Arc::clone(&shared);
                let receipt_store = Arc::clone(&receipt_store);
                let maximum = config.max_frame_bytes;
                let read_timeout = config.frame_read_timeout;
                let write_timeout = config.response_write_timeout;
                tasks.spawn(async move {
                    handle_connection(
                        stream,
                        controller,
                        receipt_store,
                        maximum,
                        read_timeout,
                        write_timeout,
                        permit,
                    )
                    .await
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
    frame_read_timeout: Duration,
    response_write_timeout: Duration,
    _permit: OwnedSemaphorePermit,
) -> Result<(), ConnectionTaskError> {
    let request = time::timeout(
        frame_read_timeout,
        read_message(&mut stream, max_frame_bytes),
    )
    .await
    .map_err(|_| ConnectionTaskError::Peer)?
    .map_err(|_| ConnectionTaskError::Peer)?;
    let creates_terminal_receipt = request.creates_terminal_receipt();
    let now_unix_ms = unix_time_ms().map_err(ConnectionTaskError::Infrastructure)?;
    let response = process_public_request(&controller, &receipt_store, request, now_unix_ms).await;
    if creates_terminal_receipt {
        persist_created_terminal_responses(&receipt_store, &controller, &response)
            .await
            .map_err(ConnectionTaskError::Infrastructure)?;
    }
    time::timeout(
        response_write_timeout,
        write_message(&mut stream, &response, max_frame_bytes),
    )
    .await
    .map_err(|_| ConnectionTaskError::Peer)?
    .map_err(|_| ConnectionTaskError::Peer)?;
    time::timeout(response_write_timeout, stream.shutdown())
        .await
        .map_err(|_| ConnectionTaskError::Peer)?
        .map_err(|_| ConnectionTaskError::Peer)
}

async fn process_public_request(
    controller: &Arc<Mutex<Controller>>,
    receipt_store: &ReceiptStore,
    request: ClientMessage,
    now_unix_ms: u64,
) -> ServerMessage {
    if request.required_role() != MessageRole::PublicClient {
        return infer_error_response(InferError::RoleNotAuthorized);
    }
    match request {
        ClientMessage::GetReceipt {
            request_id,
            request_generation,
            backend_generation,
            minimum_sequence,
        } => {
            let in_memory = {
                let controller = controller.lock().await;
                controller
                    .terminal_receipt_fenced(
                        &request_id,
                        request_generation,
                        backend_generation,
                        minimum_sequence,
                    )
                    .cloned()
            };
            match in_memory {
                Ok(receipt) => ServerMessage::Receipt(receipt),
                Err(InferError::UnknownRequest | InferError::RequestNotTerminal) => receipt_store
                    .lookup(
                        &request_id,
                        request_generation,
                        backend_generation,
                        minimum_sequence,
                    )
                    .await
                    .map(ServerMessage::Receipt)
                    .unwrap_or_else(infer_error_response),
                Err(error) => infer_error_response(error),
            }
        }
        ClientMessage::Admit(request) => {
            if receipt_store
                .contains_request_id(&request.identity.request_id)
                .await
            {
                return infer_error_response(InferError::DuplicateRequest);
            }
            let mut controller = controller.lock().await;
            dispatch_public(&mut controller, ClientMessage::Admit(request), now_unix_ms)
        }
        other => {
            let mut controller = controller.lock().await;
            dispatch_public(&mut controller, other, now_unix_ms)
        }
    }
}

fn dispatch_public(
    controller: &mut Controller,
    request: ClientMessage,
    now_unix_ms: u64,
) -> ServerMessage {
    let result = match request {
        ClientMessage::Ping { nonce } => return ServerMessage::Pong { nonce },
        ClientMessage::Admit(request) => controller
            .admit(request, now_unix_ms)
            .map(ServerMessage::Accepted),
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
        ClientMessage::Snapshot => return ServerMessage::Snapshot(controller.snapshot()),
        ClientMessage::GetReceipt { .. } => Err(InferError::InvalidTransition),
        ClientMessage::Start { .. }
        | ClientMessage::Token { .. }
        | ClientMessage::Complete { .. }
        | ClientMessage::RestartBackend { .. } => Err(InferError::RoleNotAuthorized),
    };
    result.unwrap_or_else(infer_error_response)
}

async fn run_deadline_sweep(
    controller: &Arc<Mutex<Controller>>,
    receipt_store: &ReceiptStore,
) -> io::Result<()> {
    let now_unix_ms = unix_time_ms()?;
    let receipts = {
        let mut controller = controller.lock().await;
        controller
            .expire_deadlines(now_unix_ms)
            .map_err(infer_error_to_io)?
    };
    persist_and_forget(receipt_store, controller, &receipts).await
}

async fn persist_created_terminal_responses(
    store: &ReceiptStore,
    controller: &Arc<Mutex<Controller>>,
    response: &ServerMessage,
) -> io::Result<()> {
    let receipts = match response {
        ServerMessage::Receipt(receipt) => vec![receipt.clone()],
        ServerMessage::Restarted { receipts, .. } => receipts.clone(),
        _ => Vec::new(),
    };
    persist_and_forget(store, controller, &receipts).await
}

async fn persist_and_forget(
    store: &ReceiptStore,
    controller: &Arc<Mutex<Controller>>,
    receipts: &[TerminalReceipt],
) -> io::Result<()> {
    for receipt in receipts {
        store.persist(receipt).await?;
        controller
            .lock()
            .await
            .forget_terminal(&receipt.request_id)
            .map_err(infer_error_to_io)?;
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ReceiptKey {
    request_id: RequestId,
    request_generation: u64,
    backend_generation: u64,
}

impl From<&TerminalReceipt> for ReceiptKey {
    fn from(receipt: &TerminalReceipt) -> Self {
        Self {
            request_id: receipt.request_id.clone(),
            request_generation: receipt.request_generation,
            backend_generation: receipt.backend_generation,
        }
    }
}

#[derive(Debug)]
struct ReceiptState {
    receipts: HashMap<ReceiptKey, TerminalReceipt>,
    files: usize,
    total_bytes: u64,
}

#[derive(Debug)]
struct ReceiptStore {
    root: PathBuf,
    max_files: usize,
    max_bytes: u64,
    nonce: AtomicU64,
    persist_lock: Mutex<()>,
    state: RwLock<ReceiptState>,
}

impl ReceiptStore {
    async fn open(root: PathBuf, max_files: usize, max_bytes: u64) -> io::Result<Self> {
        let mut state = ReceiptState {
            receipts: HashMap::new(),
            files: 0,
            total_bytes: 0,
        };
        let mut entries = fs::read_dir(&root).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file_type = entry.file_type().await?;
            if !file_type.is_file() {
                return Err(invalid_data("INF_RECEIPT_STORE_NON_FILE_ENTRY"));
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.contains(".tmp-") {
                fs::remove_file(entry.path()).await?;
                continue;
            }
            if !name.starts_with("receipt-") || !name.ends_with(".cbor") {
                return Err(invalid_data("INF_RECEIPT_STORE_UNKNOWN_FILE"));
            }
            let metadata = entry.metadata().await?;
            let length = metadata.len();
            let next_files = state
                .files
                .checked_add(1)
                .ok_or_else(|| invalid_data("INF_RECEIPT_FILE_COUNT_OVERFLOW"))?;
            let next_bytes = state
                .total_bytes
                .checked_add(length)
                .ok_or_else(|| invalid_data("INF_RECEIPT_BYTE_COUNT_OVERFLOW"))?;
            if next_files > max_files || next_bytes > max_bytes || length == 0 {
                return Err(invalid_data("INF_RECEIPT_STORE_BUDGET_EXCEEDED"));
            }
            let bytes = fs::read(entry.path()).await?;
            let receipt =
                match ServerMessage::decode_canonical(&bytes).map_err(infer_error_to_io)? {
                    ServerMessage::Receipt(receipt) => receipt,
                    _ => return Err(invalid_data("INF_RECEIPT_STORE_INVALID_MESSAGE")),
                };
            receipt
                .authority
                .validate_closed()
                .map_err(infer_error_to_io)?;
            if name != receipt_file_name(&receipt) {
                return Err(invalid_data("INF_RECEIPT_STORE_FILENAME_MISMATCH"));
            }
            let key = ReceiptKey::from(&receipt);
            if state.receipts.insert(key, receipt).is_some() {
                return Err(invalid_data("INF_RECEIPT_STORE_DUPLICATE"));
            }
            state.files = next_files;
            state.total_bytes = next_bytes;
        }
        Ok(Self {
            root,
            max_files,
            max_bytes,
            nonce: AtomicU64::new(1),
            persist_lock: Mutex::new(()),
            state: RwLock::new(state),
        })
    }

    async fn contains_request_id(&self, request_id: &RequestId) -> bool {
        self.state
            .read()
            .await
            .receipts
            .values()
            .any(|receipt| receipt.request_id == *request_id)
    }

    async fn lookup(
        &self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
        minimum_sequence: u64,
    ) -> codex_hepta_infer_core::Result<TerminalReceipt> {
        let state = self.state.read().await;
        let key = ReceiptKey {
            request_id: request_id.clone(),
            request_generation,
            backend_generation,
        };
        if let Some(receipt) = state.receipts.get(&key) {
            if receipt.last_sequence < minimum_sequence {
                return Err(InferError::ReceiptSequenceNotReached);
            }
            return Ok(receipt.clone());
        }
        if state.receipts.values().any(|receipt| {
            receipt.request_id == *request_id && receipt.request_generation != request_generation
        }) {
            return Err(InferError::StaleRequestGeneration);
        }
        if state.receipts.values().any(|receipt| {
            receipt.request_id == *request_id
                && receipt.request_generation == request_generation
                && receipt.backend_generation != backend_generation
        }) {
            return Err(InferError::StaleBackendGeneration);
        }
        Err(InferError::UnknownRequest)
    }

    async fn persist(&self, receipt: &TerminalReceipt) -> io::Result<()> {
        let _persist_guard = self.persist_lock.lock().await;
        receipt
            .authority
            .validate_closed()
            .map_err(infer_error_to_io)?;
        let message = ServerMessage::Receipt(receipt.clone());
        let bytes = message.encode_canonical().map_err(infer_error_to_io)?;
        let length =
            u64::try_from(bytes.len()).map_err(|_| invalid_data("INF_RECEIPT_LENGTH_OVERFLOW"))?;
        let key = ReceiptKey::from(receipt);
        {
            let state = self.state.read().await;
            if state.receipts.contains_key(&key) {
                return Err(io::Error::new(
                    ErrorKind::AlreadyExists,
                    "INF_RECEIPT_ALREADY_EXISTS",
                ));
            }
            let next_files = state
                .files
                .checked_add(1)
                .ok_or_else(|| invalid_data("INF_RECEIPT_FILE_COUNT_OVERFLOW"))?;
            let next_bytes = state
                .total_bytes
                .checked_add(length)
                .ok_or_else(|| invalid_data("INF_RECEIPT_BYTE_COUNT_OVERFLOW"))?;
            if next_files > self.max_files || next_bytes > self.max_bytes {
                return Err(io::Error::other("INF_RECEIPT_STORE_BUDGET_EXCEEDED"));
            }
        }

        let nonce = self.nonce.fetch_add(1, Ordering::Relaxed);
        let final_path = self.root.join(receipt_file_name(receipt));
        let temp_path = self.root.join(format!(
            ".receipt-{}-{}-{}-{}.tmp-{nonce}",
            receipt.request_id,
            receipt.request_generation,
            receipt.backend_generation,
            receipt.last_sequence
        ));
        if fs::symlink_metadata(&final_path).await.is_ok() {
            return Err(io::Error::new(
                ErrorKind::AlreadyExists,
                "INF_RECEIPT_ALREADY_EXISTS",
            ));
        }
        let mut options = fs::OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(unix)]
        {
            options.mode(0o600);
        }
        let mut file = options.open(&temp_path).await?;
        if let Err(error) = async {
            file.write_all(&bytes).await?;
            file.flush().await?;
            file.sync_all().await?;
            drop(file);
            fs::rename(&temp_path, &final_path).await?;
            sync_directory(self.root.clone()).await
        }
        .await
        {
            let _ = fs::remove_file(&temp_path).await;
            return Err(error);
        }

        let mut state = self.state.write().await;
        if state.receipts.contains_key(&key) {
            return Err(io::Error::other("INF_RECEIPT_INDEX_DUPLICATE"));
        }
        state.files = state
            .files
            .checked_add(1)
            .ok_or_else(|| invalid_data("INF_RECEIPT_FILE_COUNT_OVERFLOW"))?;
        state.total_bytes = state
            .total_bytes
            .checked_add(length)
            .ok_or_else(|| invalid_data("INF_RECEIPT_BYTE_COUNT_OVERFLOW"))?;
        state.receipts.insert(key, receipt.clone());
        Ok(())
    }
}

fn receipt_file_name(receipt: &TerminalReceipt) -> String {
    format!(
        "receipt-{}-{}-{}-{}.cbor",
        receipt.request_id,
        receipt.request_generation,
        receipt.backend_generation,
        receipt.last_sequence
    )
}

async fn read_message(
    stream: &mut UnixStream,
    max_frame_bytes: usize,
) -> io::Result<ClientMessage> {
    let bytes = read_frame(stream, max_frame_bytes).await?;
    ClientMessage::decode_canonical(&bytes).map_err(infer_error_to_io)
}

async fn read_frame(stream: &mut UnixStream, max_frame_bytes: usize) -> io::Result<Vec<u8>> {
    let mut length_bytes = [0u8; 4];
    stream.read_exact(&mut length_bytes).await?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes))
        .map_err(|_| invalid_data("INF_FRAME_LENGTH_INVALID"))?;
    if length == 0 || length > max_frame_bytes {
        return Err(invalid_data("INF_FRAME_LENGTH_OUT_OF_BOUNDS"));
    }
    let mut bytes = vec![0u8; length];
    stream.read_exact(&mut bytes).await?;
    Ok(bytes)
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
        return Err(invalid_data("INF_FRAME_OUT_OF_BOUNDS"));
    }
    let length = u32::try_from(bytes.len()).map_err(|_| invalid_data("INF_FRAME_TOO_LARGE"))?;
    stream.write_all(&length.to_be_bytes()).await?;
    stream.write_all(bytes).await?;
    stream.flush().await
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
                        ErrorKind::AddrInUse,
                        "INF_DAEMON_SOCKET_PATH_OCCUPIED",
                    ));
                }
            }
        },
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    UnixListener::bind(socket_path)
}

async fn next_backend_generation(path: &Path) -> io::Result<u64> {
    let current = match fs::read_to_string(path).await {
        Ok(value) => value
            .trim()
            .parse::<u64>()
            .map_err(|_| invalid_data("INF_GENERATION_FILE_INVALID"))?,
        Err(error) if error.kind() == ErrorKind::NotFound => 0,
        Err(error) => return Err(error),
    };
    let next = current
        .checked_add(1)
        .ok_or_else(|| invalid_data("INF_GENERATION_OVERFLOW"))?;
    let parent = required_parent(path, "INF_GENERATION_PARENT_MISSING")?;
    codex_uds::prepare_private_socket_directory(parent).await?;
    let nonce = unix_time_ms()?;
    let temp_path = parent.join(format!(
        ".backend.generation.{}.tmp-{nonce}",
        std::process::id()
    ));
    let mut options = fs::OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        options.mode(0o600);
    }
    let mut file = options.open(&temp_path).await?;
    if let Err(error) = async {
        file.write_all(format!("{next}\n").as_bytes()).await?;
        file.flush().await?;
        file.sync_all().await?;
        drop(file);
        fs::rename(&temp_path, path).await?;
        sync_directory(parent.to_path_buf()).await
    }
    .await
    {
        let _ = fs::remove_file(&temp_path).await;
        return Err(error);
    }
    Ok(next)
}

#[cfg(unix)]
async fn sync_directory(path: PathBuf) -> io::Result<()> {
    tokio::task::spawn_blocking(move || std::fs::File::open(path)?.sync_all())
        .await
        .map_err(|error| io::Error::other(format!("INF_DIRECTORY_SYNC_TASK_FAILED: {error}")))?
}

#[cfg(not(unix))]
async fn sync_directory(_path: PathBuf) -> io::Result<()> {
    Ok(())
}

fn required_parent<'a>(path: &'a Path, code: &'static str) -> io::Result<&'a Path> {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| invalid_input(code))
}

fn unix_time_ms() -> io::Result<u64> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::other("INF_SYSTEM_TIME_BEFORE_EPOCH"))?;
    u64::try_from(duration.as_millis()).map_err(|_| io::Error::other("INF_SYSTEM_TIME_OVERFLOW"))
}

fn infer_error_response(error: InferError) -> ServerMessage {
    ServerMessage::Error {
        code: error.code().to_owned(),
    }
}

fn infer_error_to_io(error: InferError) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, error)
}

fn invalid_input(code: &'static str) -> io::Error {
    io::Error::new(ErrorKind::InvalidInput, code)
}

fn invalid_data(code: &'static str) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, code)
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
