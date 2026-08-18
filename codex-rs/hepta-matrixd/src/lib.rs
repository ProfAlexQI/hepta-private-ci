//! Transport-neutral Matrix-to-App-Server bridge for one Hepta workspace Agent.
//!
//! This crate owns neither Matrix synchronization nor durable inbox/outbox
//! storage.  It binds one Matrix room to one App Server project and one durable
//! Codex thread, then submits a canonical Matrix event through Codex's existing
//! persistent thread queue.  The remote adapter connects only through the
//! exact-generation `agentd` session ingress and uses a bounded event stream.
//!
//! Matrix events never implement an approval or cancellation authority here.

#![forbid(unsafe_code)]
#![recursion_limit = "256"]

mod config;
mod runner;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;

use codex_app_server_client::AppServerEvent;
use codex_app_server_client::RemoteAppServerClient;
use codex_app_server_client::RemoteAppServerConnectArgs;
use codex_app_server_client::RemoteAppServerEndpoint;
use codex_app_server_client::RemoteAppServerRequestHandle;
use codex_app_server_client::TypedRequestError;
use codex_app_server_protocol::ClientRequest;
use codex_app_server_protocol::ProjectCreateParams;
use codex_app_server_protocol::ProjectCreateResponse;
use codex_app_server_protocol::ProjectRoot;
use codex_app_server_protocol::QueuedSubmission;
use codex_app_server_protocol::RequestId;
use codex_app_server_protocol::SortDirection;
use codex_app_server_protocol::ThreadHistoryMode;
use codex_app_server_protocol::ThreadItem;
use codex_app_server_protocol::ThreadListCwdFilter;
use codex_app_server_protocol::ThreadListParams;
use codex_app_server_protocol::ThreadListResponse;
use codex_app_server_protocol::ThreadQueueAddParams;
use codex_app_server_protocol::ThreadQueueAddResponse;
use codex_app_server_protocol::ThreadQueueListParams;
use codex_app_server_protocol::ThreadQueueListResponse;
use codex_app_server_protocol::ThreadSortKey;
use codex_app_server_protocol::ThreadSource;
use codex_app_server_protocol::ThreadStartParams;
use codex_app_server_protocol::ThreadStartResponse;
use codex_app_server_protocol::ThreadTurnsListParams;
use codex_app_server_protocol::ThreadTurnsListResponse;
use codex_app_server_protocol::TurnItemsView;
use codex_app_server_protocol::UserInput;
use codex_hepta_agentd::AgentdClient;
use codex_hepta_agentd::SessionTransport;
use codex_hepta_contracts::AgentId;
use codex_hepta_matrix_protocol::MatrixEventId;
use codex_hepta_matrix_protocol::MatrixRoomId;
use codex_hepta_matrix_protocol::client_user_message_id;
use codex_hepta_matrix_protocol::room_project_idempotency_key;
use codex_utils_absolute_path::AbsolutePathBuf;
use tokio::sync::Semaphore;

mod runtime;

pub use config::HEPTA_MATRIX_ALLOWED_ROOMS_ENV;
pub use config::HEPTA_MATRIX_ALLOWED_SENDERS_ENV;
pub use config::HEPTA_MATRIX_BINDING_REVISION_ENV;
pub use config::HEPTA_MATRIX_DEVICE_DISPLAY_NAME_ENV;
pub use config::HEPTA_MATRIX_DEVICE_ID_ENV;
pub use config::HEPTA_MATRIX_HOMESERVER_ENV;
pub use config::HEPTA_MATRIX_PASSWORD_ENV;
pub use config::HEPTA_MATRIX_REQUIRE_EXPLICIT_MENTION_ENV;
pub use config::HEPTA_MATRIX_STORE_PASSPHRASE_ENV;
pub use config::HEPTA_MATRIX_SYNC_TIMELINE_LIMIT_ENV;
pub use config::HEPTA_MATRIX_SYNC_TIMEOUT_MS_ENV;
pub use config::HEPTA_MATRIX_USER_ID_ENV;
pub use config::MatrixdConfig;
pub use config::MatrixdConfigError;
pub use config::MatrixdCredentials;
pub use runner::MatrixdRunError;
pub use runner::run;

pub use runtime::MatrixDispatchOutcome;
pub use runtime::MatrixEventProjection;
pub use runtime::MatrixRuntime;
pub use runtime::MatrixRuntimeBridge;
pub use runtime::MatrixRuntimeError;
pub use runtime::MatrixRuntimeFuture;
pub use runtime::MatrixRuntimeRecovery;

const BRIDGE_SCHEMA: &str = "hepta.matrix.bridge.v1";
const BRIDGE_THREAD_SOURCE: &str = "hepta.matrix";
const DEFAULT_PAGE_SIZE: u32 = 100;
const DEFAULT_COMMAND_CAPACITY: usize = 64;
const DEFAULT_EVENT_CAPACITY: usize = 512;
const MAX_RECONCILIATION_PAGES: usize = 1_024;
const JSON_RPC_INVALID_REQUEST_CODE: i64 = -32_600;

/// The strongest admission guarantee this bridge and Core queue jointly make.
///
/// This covers model-turn admission for one stable Matrix client message id,
/// not Matrix delivery or arbitrary external tool effects. Queue dispatch
/// waits for rollout persistence before deleting a row and, after a crash,
/// removes any stale duplicate row by matching the durable client id.
pub const DELIVERY_GUARANTEE: DeliveryGuarantee =
    DeliveryGuarantee::ExactlyOncePersistedCoreAdmissionPerClientMessageId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryGuarantee {
    ExactlyOncePersistedCoreAdmissionPerClientMessageId,
}

pub type BridgeFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, MatrixBridgeError>> + Send + 'a>>;

/// Narrow App Server surface consumed by the deterministic bridge core.
///
/// A durable Matrix store can use [`MatrixAppServerBridge`] with this trait
/// without depending on the remote socket implementation or on Matrix SDK
/// types.  Implementations must preserve opaque pagination cursors exactly.
pub trait MatrixAppServerTransport: Send + Sync {
    fn create_project(&self, request: BridgeProjectCreate) -> BridgeFuture<'_, BridgeProject>;

    fn list_threads(&self, request: BridgeThreadList)
    -> BridgeFuture<'_, BridgePage<BridgeThread>>;

    fn start_thread(&self, request: BridgeThreadStart) -> BridgeFuture<'_, BridgeThread>;

    fn list_queue(
        &self,
        request: BridgeQueueList,
    ) -> BridgeFuture<'_, BridgePage<BridgeQueuedSubmission>>;

    fn list_turn_client_messages(
        &self,
        request: BridgeTurnList,
    ) -> BridgeFuture<'_, BridgePage<BridgeTurnClientMessage>>;

    fn add_queue(&self, request: BridgeQueueAdd) -> BridgeFuture<'_, BridgeQueuedSubmission>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeProjectCreate {
    pub name: String,
    pub roots: Vec<AbsolutePathBuf>,
    pub metadata: BTreeMap<String, String>,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeProject {
    pub id: String,
    pub roots: Vec<AbsolutePathBuf>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeThreadList {
    pub project_id: String,
    pub cwd: AbsolutePathBuf,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeThreadStart {
    pub project_id: String,
    pub cwd: AbsolutePathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeThread {
    pub id: String,
    pub project_id: Option<String>,
    pub cwd: AbsolutePathBuf,
    pub ephemeral: bool,
    pub thread_source: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeQueueList {
    pub thread_id: String,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeTurnList {
    pub thread_id: String,
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeQueueAdd {
    pub thread_id: String,
    pub input: Vec<UserInput>,
    pub client_user_message_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeQueuedSubmission {
    pub id: String,
    pub client_user_message_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeTurnClientMessage {
    pub turn_id: String,
    pub client_user_message_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgePage<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomThreadBinding {
    pub project_id: String,
    pub thread_id: String,
    /// True when `thread/list` recovered the thread rather than this call
    /// issuing `thread/start`.
    pub recovered: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatrixSubmission {
    pub binding: RoomThreadBinding,
    pub client_user_message_id: String,
    pub state: MatrixSubmissionState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatrixSubmissionState {
    Queued { queued_submission_id: String },
    ReconciledQueued { queued_submission_id: String },
    ReconciledTurn { turn_id: String },
}

/// Whether a bridge retry may create a new Core queue row when reconciliation
/// finds neither a queued submission nor a persisted turn.
///
/// A durable dispatch that already recorded `queued` or `admitted` must use
/// [`MatrixAdmissionMode::ReconcileOnly`].  Treating a missing Core record as
/// permission to submit again would weaken the exactly-once admission bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatrixAdmissionMode {
    AllowIfAbsent,
    ReconcileOnly,
}

#[derive(Clone, Debug)]
pub struct MatrixBridgeConfig {
    pub agent_id: AgentId,
    pub workspace_root: AbsolutePathBuf,
    pub page_size: u32,
}

impl MatrixBridgeConfig {
    pub fn new(agent_id: AgentId, workspace_root: AbsolutePathBuf) -> Self {
        Self {
            agent_id,
            workspace_root,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }

    pub fn validate(&self) -> Result<(), MatrixBridgeError> {
        if self.page_size == 0 {
            return Err(MatrixBridgeError::Invalid(
                "Matrix bridge page size must be non-zero".to_string(),
            ));
        }
        Ok(())
    }
}

/// Deterministic room/thread and message-submission core.
///
/// The single permit is intentionally process-local. Product wiring must still
/// run a single fenced `matrixd` for one Agent generation; it is not a
/// distributed lock and never creates a central gateway.
pub struct MatrixAppServerBridge<T> {
    config: MatrixBridgeConfig,
    transport: T,
    operation: Semaphore,
}

impl<T> MatrixAppServerBridge<T>
where
    T: MatrixAppServerTransport,
{
    pub fn new(config: MatrixBridgeConfig, transport: T) -> Result<Self, MatrixBridgeError> {
        config.validate()?;
        Ok(Self {
            config,
            transport,
            operation: Semaphore::new(1),
        })
    }

    pub fn config(&self) -> &MatrixBridgeConfig {
        &self.config
    }

    pub fn transport(&self) -> &T {
        &self.transport
    }

    pub async fn ensure_room_thread(
        &self,
        room_id: &MatrixRoomId,
    ) -> Result<RoomThreadBinding, MatrixBridgeError> {
        let _operation = self.operation.acquire().await.map_err(|_| {
            MatrixBridgeError::Protocol("Matrix bridge operation gate closed".to_string())
        })?;
        self.ensure_room_thread_locked(room_id, None).await
    }

    /// Recover an already-durable room/thread identity without creating a
    /// replacement when App Server has not materialized the first thread yet.
    ///
    /// `thread/list` intentionally omits a newly started thread until its
    /// first user message is materialized.  The Matrix durable store is the
    /// restart authority for the exact thread ID during that interval.  A
    /// caller supplying that ID must therefore reconcile it, never interpret
    /// an empty list as permission to start another thread.
    pub async fn reconcile_room_thread(
        &self,
        room_id: &MatrixRoomId,
        expected_thread_id: &str,
    ) -> Result<RoomThreadBinding, MatrixBridgeError> {
        if expected_thread_id.is_empty() {
            return Err(MatrixBridgeError::Invalid(
                "durable Matrix thread identity cannot be empty".to_string(),
            ));
        }
        let _operation = self.operation.acquire().await.map_err(|_| {
            MatrixBridgeError::Protocol("Matrix bridge operation gate closed".to_string())
        })?;
        self.ensure_room_thread_locked(room_id, Some(expected_thread_id))
            .await
    }

    pub async fn submit_matrix_event(
        &self,
        room_id: &MatrixRoomId,
        event_id: &MatrixEventId,
        input: Vec<UserInput>,
    ) -> Result<MatrixSubmission, MatrixBridgeError> {
        let _operation = self.operation.acquire().await.map_err(|_| {
            MatrixBridgeError::Protocol("Matrix bridge operation gate closed".to_string())
        })?;
        let binding = self.ensure_room_thread_locked(room_id, None).await?;
        self.submit_matrix_event_on_binding_locked(
            room_id,
            event_id,
            input,
            &binding,
            MatrixAdmissionMode::AllowIfAbsent,
        )
        .await
    }

    /// Submit or reconcile one event against an already persisted exact room
    /// binding.  Runtime recovery uses `ReconcileOnly` after it has durably
    /// observed a Core queue/turn identity.
    pub(crate) async fn submit_matrix_event_on_binding(
        &self,
        room_id: &MatrixRoomId,
        event_id: &MatrixEventId,
        input: Vec<UserInput>,
        binding: &RoomThreadBinding,
        admission_mode: MatrixAdmissionMode,
    ) -> Result<MatrixSubmission, MatrixBridgeError> {
        let _operation = self.operation.acquire().await.map_err(|_| {
            MatrixBridgeError::Protocol("Matrix bridge operation gate closed".to_string())
        })?;
        self.submit_matrix_event_on_binding_locked(
            room_id,
            event_id,
            input,
            binding,
            admission_mode,
        )
        .await
    }

    async fn submit_matrix_event_on_binding_locked(
        &self,
        room_id: &MatrixRoomId,
        event_id: &MatrixEventId,
        input: Vec<UserInput>,
        binding: &RoomThreadBinding,
        admission_mode: MatrixAdmissionMode,
    ) -> Result<MatrixSubmission, MatrixBridgeError> {
        if input.is_empty() {
            return Err(MatrixBridgeError::Invalid(
                "Matrix event cannot submit empty user input".to_string(),
            ));
        }
        if binding.project_id.is_empty() || binding.thread_id.is_empty() {
            return Err(MatrixBridgeError::Protocol(
                "Matrix submission binding has an empty project or thread identity".to_string(),
            ));
        }
        let client_id = client_user_message_id(&self.config.agent_id, room_id, event_id);
        let queue_match = self
            .find_queued_client_id(&binding.thread_id, &client_id)
            .await?;
        let turn_match = self
            .find_turn_client_id(&binding.thread_id, &client_id)
            .await?;
        let state = match (queue_match, turn_match) {
            (Some(queued), None) => MatrixSubmissionState::ReconciledQueued {
                queued_submission_id: queued.id,
            },
            (None, Some(turn)) => MatrixSubmissionState::ReconciledTurn {
                turn_id: turn.turn_id,
            },
            (Some(_), Some(_)) => {
                return Err(MatrixBridgeError::Protocol(format!(
                    "client message id {client_id} exists in both queue and turn history"
                )));
            }
            (None, None) if admission_mode == MatrixAdmissionMode::AllowIfAbsent => {
                let queued = self
                    .transport
                    .add_queue(BridgeQueueAdd {
                        thread_id: binding.thread_id.clone(),
                        input,
                        client_user_message_id: client_id.clone(),
                    })
                    .await?;
                if queued.client_user_message_id != client_id || queued.id.is_empty() {
                    return Err(MatrixBridgeError::Protocol(
                        "thread/queue/add returned a mismatched or empty submission identity"
                            .to_string(),
                    ));
                }
                MatrixSubmissionState::Queued {
                    queued_submission_id: queued.id,
                }
            }
            (None, None) => {
                return Err(MatrixBridgeError::Protocol(format!(
                    "durable client message id {client_id} is missing from both Core queue and turn history"
                )));
            }
        };
        Ok(MatrixSubmission {
            binding: binding.clone(),
            client_user_message_id: client_id,
            state,
        })
    }

    async fn ensure_room_thread_locked(
        &self,
        room_id: &MatrixRoomId,
        expected_thread_id: Option<&str>,
    ) -> Result<RoomThreadBinding, MatrixBridgeError> {
        let project_request = self.project_request(room_id);
        let project = self
            .transport
            .create_project(project_request.clone())
            .await?;
        validate_project(&project, &project_request)?;

        let mut cursor = None;
        let mut cursors = HashSet::new();
        let mut matches = Vec::new();
        let mut exhausted = false;
        for _ in 0..MAX_RECONCILIATION_PAGES {
            let page = self
                .transport
                .list_threads(BridgeThreadList {
                    project_id: project.id.clone(),
                    cwd: self.config.workspace_root.clone(),
                    cursor: cursor.clone(),
                    limit: self.config.page_size,
                })
                .await?;
            for thread in page.data {
                validate_thread(&thread, &project.id, &self.config.workspace_root)?;
                matches.push(thread);
            }
            let Some(next) = page.next_cursor else {
                exhausted = true;
                break;
            };
            if !cursors.insert(next.clone()) {
                return Err(MatrixBridgeError::Protocol(
                    "thread/list repeated a pagination cursor".to_string(),
                ));
            }
            cursor = Some(next);
        }
        if !exhausted {
            return Err(MatrixBridgeError::Protocol(
                "thread/list exceeded the reconciliation page bound".to_string(),
            ));
        }
        if let Some(expected_thread_id) = expected_thread_id {
            return match matches.as_slice() {
                [] => Ok(RoomThreadBinding {
                    project_id: project.id,
                    thread_id: expected_thread_id.to_string(),
                    recovered: true,
                }),
                [thread] if thread.id == expected_thread_id => Ok(RoomThreadBinding {
                    project_id: project.id,
                    thread_id: thread.id.clone(),
                    recovered: true,
                }),
                [thread] => Err(MatrixBridgeError::Protocol(format!(
                    "App Server Matrix room thread {} disagrees with durable thread {expected_thread_id}",
                    thread.id
                ))),
                _ => Err(MatrixBridgeError::Protocol(format!(
                    "Matrix room project {} has multiple active bridge threads",
                    project.id
                ))),
            };
        }

        match matches.as_slice() {
            [thread] => Ok(RoomThreadBinding {
                project_id: project.id,
                thread_id: thread.id.clone(),
                recovered: true,
            }),
            [] => {
                let thread = self
                    .transport
                    .start_thread(BridgeThreadStart {
                        project_id: project.id.clone(),
                        cwd: self.config.workspace_root.clone(),
                    })
                    .await?;
                validate_thread(&thread, &project.id, &self.config.workspace_root)?;
                Ok(RoomThreadBinding {
                    project_id: project.id,
                    thread_id: thread.id,
                    recovered: false,
                })
            }
            _ => Err(MatrixBridgeError::Protocol(format!(
                "Matrix room project {} has multiple active bridge threads",
                project.id
            ))),
        }
    }

    fn project_request(&self, room_id: &MatrixRoomId) -> BridgeProjectCreate {
        let key = room_project_idempotency_key(&self.config.agent_id, room_id);
        let suffix = key.rsplit('-').next().unwrap_or(key.as_str());
        let short_suffix = suffix.get(..16).unwrap_or(suffix);
        BridgeProjectCreate {
            name: format!("Hepta Matrix {short_suffix}"),
            roots: vec![self.config.workspace_root.clone()],
            metadata: BTreeMap::from([
                ("hepta.bridge".to_string(), BRIDGE_SCHEMA.to_string()),
                (
                    "hepta.agent_id".to_string(),
                    self.config.agent_id.as_str().to_string(),
                ),
                ("hepta.matrix.room_id".to_string(), room_id.to_string()),
            ]),
            idempotency_key: key,
        }
    }

    async fn find_queued_client_id(
        &self,
        thread_id: &str,
        client_id: &str,
    ) -> Result<Option<BridgeQueuedSubmission>, MatrixBridgeError> {
        let mut cursor = None;
        let mut cursors = HashSet::new();
        let mut found = None;
        for _ in 0..MAX_RECONCILIATION_PAGES {
            let page = self
                .transport
                .list_queue(BridgeQueueList {
                    thread_id: thread_id.to_string(),
                    cursor: cursor.clone(),
                    limit: self.config.page_size,
                })
                .await?;
            for queued in page.data {
                if queued.client_user_message_id == client_id && found.is_none() {
                    // Concurrent/retried queue/add calls can leave more than
                    // one row with the same client id. Core's durable dispatch
                    // join collapses them before a second turn can start.
                    found = Some(queued);
                }
            }
            let Some(next) = page.next_cursor else {
                return Ok(found);
            };
            if !cursors.insert(next.clone()) {
                return Err(MatrixBridgeError::Protocol(
                    "thread/queue/list repeated a pagination cursor".to_string(),
                ));
            }
            cursor = Some(next);
        }
        Err(MatrixBridgeError::Protocol(
            "thread/queue/list exceeded the reconciliation page bound".to_string(),
        ))
    }

    async fn find_turn_client_id(
        &self,
        thread_id: &str,
        client_id: &str,
    ) -> Result<Option<BridgeTurnClientMessage>, MatrixBridgeError> {
        let mut cursor = None;
        let mut cursors = HashSet::new();
        let mut found = None;
        for _ in 0..MAX_RECONCILIATION_PAGES {
            let page = self
                .transport
                .list_turn_client_messages(BridgeTurnList {
                    thread_id: thread_id.to_string(),
                    cursor: cursor.clone(),
                    limit: self.config.page_size,
                })
                .await?;
            for message in page.data {
                if message.client_user_message_id == client_id && found.replace(message).is_some() {
                    return Err(MatrixBridgeError::Protocol(format!(
                        "client message id {client_id} appears in multiple persisted turns"
                    )));
                }
            }
            let Some(next) = page.next_cursor else {
                return Ok(found);
            };
            if !cursors.insert(next.clone()) {
                return Err(MatrixBridgeError::Protocol(
                    "thread/turns/list repeated a pagination cursor".to_string(),
                ));
            }
            cursor = Some(next);
        }
        Err(MatrixBridgeError::Protocol(
            "thread/turns/list exceeded the reconciliation page bound".to_string(),
        ))
    }
}

fn validate_project(
    project: &BridgeProject,
    request: &BridgeProjectCreate,
) -> Result<(), MatrixBridgeError> {
    if project.id.is_empty()
        || project.roots != request.roots
        || project.metadata != request.metadata
    {
        return Err(MatrixBridgeError::Protocol(
            "project/create returned a project outside the exact Agent/room binding".to_string(),
        ));
    }
    Ok(())
}

fn validate_thread(
    thread: &BridgeThread,
    project_id: &str,
    workspace_root: &AbsolutePathBuf,
) -> Result<(), MatrixBridgeError> {
    if thread.id.is_empty()
        || thread.ephemeral
        || thread.project_id.as_deref() != Some(project_id)
        || &thread.cwd != workspace_root
        || thread.thread_source.as_deref() != Some(BRIDGE_THREAD_SOURCE)
    {
        return Err(MatrixBridgeError::Protocol(
            "App Server returned a thread outside the exact Matrix room binding".to_string(),
        ));
    }
    Ok(())
}

/// Exact-generation connection settings for one Agent's App Server ingress.
#[derive(Clone, Debug)]
pub struct MatrixAgentdConnectArgs {
    pub agentd_control_socket: PathBuf,
    pub agent_id: AgentId,
    pub spawn_generation: u64,
    pub client_version: String,
    pub command_channel_capacity: usize,
    pub event_channel_capacity: usize,
}

impl MatrixAgentdConnectArgs {
    pub fn new(
        agentd_control_socket: PathBuf,
        agent_id: AgentId,
        spawn_generation: u64,
        client_version: impl Into<String>,
    ) -> Self {
        Self {
            agentd_control_socket,
            agent_id,
            spawn_generation,
            client_version: client_version.into(),
            command_channel_capacity: DEFAULT_COMMAND_CAPACITY,
            event_channel_capacity: DEFAULT_EVENT_CAPACITY,
        }
    }
}

pub struct ConnectedMatrixAppServer {
    pub transport: RemoteMatrixAppServerTransport,
    pub events: RemoteMatrixAppServerEvents,
}

/// Connects through `AgentdClient::session_ingress`; direct fleet-wide routing
/// or a central execution gateway is intentionally absent.
pub async fn connect_via_agentd(
    args: MatrixAgentdConnectArgs,
) -> Result<ConnectedMatrixAppServer, MatrixBridgeError> {
    if args.client_version.is_empty()
        || args.command_channel_capacity == 0
        || args.event_channel_capacity == 0
    {
        return Err(MatrixBridgeError::Invalid(
            "matrixd App Server connection requires non-empty version and bounded capacities"
                .to_string(),
        ));
    }
    let agentd = AgentdClient::new(
        args.agentd_control_socket,
        args.agent_id.clone(),
        args.spawn_generation,
    )?;
    let health = agentd.health().await?;
    if !health.ready || health.fenced {
        return Err(MatrixBridgeError::Invalid(
            "matrixd cannot attach to an unready or fenced agentd generation".to_string(),
        ));
    }
    let ingress = agentd.session_ingress().await?;
    if ingress.transport != SessionTransport::CodexAppServerWebsocketOverUds {
        return Err(MatrixBridgeError::Protocol(
            "agentd returned an unsupported session ingress transport".to_string(),
        ));
    }
    let socket_path = AbsolutePathBuf::from_absolute_path(&ingress.socket_path)?;
    let client = RemoteAppServerClient::connect_with_bounded_events(
        RemoteAppServerConnectArgs {
            endpoint: RemoteAppServerEndpoint::UnixSocket { socket_path },
            client_name: "hepta-matrixd".to_string(),
            client_version: args.client_version,
            experimental_api: true,
            mcp_server_openai_form_elicitation: false,
            opt_out_notification_methods: Vec::new(),
            channel_capacity: args.command_channel_capacity,
        },
        args.event_channel_capacity,
    )
    .await?;
    let expected_home = health.home_root.to_string_lossy();
    if client.codex_home() != Some(expected_home.as_ref()) {
        let actual = client.codex_home().unwrap_or("<missing>").to_string();
        let _ = client.shutdown().await;
        return Err(MatrixBridgeError::Protocol(format!(
            "App Server home {actual} does not match Agent home {expected_home}"
        )));
    }
    let request_handle = client.request_handle();
    Ok(ConnectedMatrixAppServer {
        transport: RemoteMatrixAppServerTransport {
            request_handle,
            next_request_id: Arc::new(AtomicI64::new(1)),
        },
        events: RemoteMatrixAppServerEvents { client },
    })
}

#[derive(Clone)]
pub struct RemoteMatrixAppServerTransport {
    request_handle: RemoteAppServerRequestHandle,
    next_request_id: Arc<AtomicI64>,
}

impl RemoteMatrixAppServerTransport {
    fn request_id(&self) -> RequestId {
        RequestId::Integer(self.next_request_id.fetch_add(1, Ordering::Relaxed))
    }

    async fn request<T>(&self, request: ClientRequest) -> Result<T, MatrixBridgeError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.request_handle
            .request_typed(request)
            .await
            .map_err(|error| MatrixBridgeError::AppServer(error.to_string()))
    }
}

pub struct RemoteMatrixAppServerEvents {
    client: RemoteAppServerClient,
}

impl RemoteMatrixAppServerEvents {
    pub fn server_version(&self) -> Option<&str> {
        self.client.server_version()
    }

    pub fn codex_home(&self) -> Option<&str> {
        self.client.codex_home()
    }

    pub async fn next_event(&mut self) -> Option<AppServerEvent> {
        self.client.next_event().await
    }

    pub async fn shutdown(self) -> std::io::Result<()> {
        self.client.shutdown().await
    }
}

impl MatrixAppServerTransport for RemoteMatrixAppServerTransport {
    fn create_project(&self, request: BridgeProjectCreate) -> BridgeFuture<'_, BridgeProject> {
        Box::pin(async move {
            let response: ProjectCreateResponse = self
                .request(ClientRequest::ProjectCreate {
                    request_id: self.request_id(),
                    params: ProjectCreateParams {
                        name: request.name,
                        roots: request
                            .roots
                            .into_iter()
                            .map(|path| ProjectRoot { path })
                            .collect(),
                        metadata: Some(request.metadata),
                        idempotency_key: request.idempotency_key,
                    },
                })
                .await?;
            Ok(BridgeProject {
                id: response.project.id,
                roots: response
                    .project
                    .roots
                    .into_iter()
                    .map(|root| root.path)
                    .collect(),
                metadata: response.project.metadata,
            })
        })
    }

    fn list_threads(
        &self,
        request: BridgeThreadList,
    ) -> BridgeFuture<'_, BridgePage<BridgeThread>> {
        Box::pin(async move {
            let response: ThreadListResponse = self
                .request(ClientRequest::ThreadList {
                    request_id: self.request_id(),
                    params: ThreadListParams {
                        cursor: request.cursor,
                        limit: Some(request.limit),
                        sort_key: Some(ThreadSortKey::CreatedAt),
                        sort_direction: Some(SortDirection::Asc),
                        model_providers: None,
                        source_kinds: None,
                        archived: Some(false),
                        section_id: None,
                        project_id: Some(Some(request.project_id)),
                        cwd: Some(ThreadListCwdFilter::One(
                            request.cwd.as_path().to_string_lossy().into_owned(),
                        )),
                        use_state_db_only: false,
                        search_term: None,
                        parent_thread_id: None,
                        ancestor_thread_id: None,
                    },
                })
                .await?;
            Ok(BridgePage {
                data: response.data.into_iter().map(bridge_thread).collect(),
                next_cursor: response.next_cursor,
            })
        })
    }

    fn start_thread(&self, request: BridgeThreadStart) -> BridgeFuture<'_, BridgeThread> {
        Box::pin(async move {
            let cwd = request.cwd.as_path().to_string_lossy().into_owned();
            let response: ThreadStartResponse = self
                .request(ClientRequest::ThreadStart {
                    request_id: self.request_id(),
                    params: ThreadStartParams {
                        cwd: Some(cwd),
                        runtime_workspace_roots: Some(vec![request.cwd]),
                        ephemeral: Some(false),
                        history_mode: Some(ThreadHistoryMode::Paginated),
                        thread_source: Some(ThreadSource::Feature(
                            BRIDGE_THREAD_SOURCE.to_string(),
                        )),
                        project_id: Some(request.project_id),
                        ..Default::default()
                    },
                })
                .await?;
            Ok(bridge_thread(response.thread))
        })
    }

    fn list_queue(
        &self,
        request: BridgeQueueList,
    ) -> BridgeFuture<'_, BridgePage<BridgeQueuedSubmission>> {
        Box::pin(async move {
            let response: ThreadQueueListResponse = self
                .request(ClientRequest::ThreadQueueList {
                    request_id: self.request_id(),
                    params: ThreadQueueListParams {
                        thread_id: request.thread_id,
                        cursor: request.cursor,
                        limit: Some(request.limit),
                    },
                })
                .await?;
            Ok(BridgePage {
                data: response
                    .data
                    .into_iter()
                    .map(bridge_queued_submission)
                    .collect(),
                next_cursor: response.next_cursor,
            })
        })
    }

    fn list_turn_client_messages(
        &self,
        request: BridgeTurnList,
    ) -> BridgeFuture<'_, BridgePage<BridgeTurnClientMessage>> {
        Box::pin(async move {
            let thread_id = request.thread_id;
            let cursor = request.cursor;
            let response: ThreadTurnsListResponse = match self
                .request_handle
                .request_typed(ClientRequest::ThreadTurnsList {
                    request_id: self.request_id(),
                    params: ThreadTurnsListParams {
                        thread_id: thread_id.clone(),
                        cursor: cursor.clone(),
                        limit: Some(request.limit),
                        sort_direction: Some(SortDirection::Asc),
                        items_view: Some(TurnItemsView::Full),
                    },
                })
                .await
            {
                Ok(response) => response,
                Err(error)
                    if is_unmaterialized_thread_turns_list_error(
                        &error,
                        &thread_id,
                        cursor.as_deref(),
                    ) =>
                {
                    return Ok(BridgePage {
                        data: Vec::new(),
                        next_cursor: None,
                    });
                }
                Err(error) => return Err(MatrixBridgeError::AppServer(error.to_string())),
            };
            let mut data = Vec::new();
            for turn in response.data {
                if turn.items_view != TurnItemsView::Full {
                    return Err(MatrixBridgeError::Protocol(
                        "thread/turns/list did not return the requested full item view".to_string(),
                    ));
                }
                for item in turn.items {
                    if let ThreadItem::UserMessage {
                        client_id: Some(client_user_message_id),
                        ..
                    } = item
                    {
                        data.push(BridgeTurnClientMessage {
                            turn_id: turn.id.clone(),
                            client_user_message_id,
                        });
                    }
                }
            }
            Ok(BridgePage {
                data,
                next_cursor: response.next_cursor,
            })
        })
    }

    fn add_queue(&self, request: BridgeQueueAdd) -> BridgeFuture<'_, BridgeQueuedSubmission> {
        Box::pin(async move {
            let response: ThreadQueueAddResponse = self
                .request(ClientRequest::ThreadQueueAdd {
                    request_id: self.request_id(),
                    params: ThreadQueueAddParams {
                        thread_id: request.thread_id,
                        input: request.input,
                        client_user_message_id: request.client_user_message_id,
                    },
                })
                .await?;
            Ok(bridge_queued_submission(response.queued_submission))
        })
    }
}

fn is_unmaterialized_thread_turns_list_error(
    error: &TypedRequestError,
    thread_id: &str,
    cursor: Option<&str>,
) -> bool {
    if cursor.is_some() {
        return false;
    }
    let expected_message = format!(
        "thread {thread_id} is not materialized yet; thread/turns/list is unavailable before first user message"
    );
    matches!(
        error,
        TypedRequestError::Server { method, source }
            if method == "thread/turns/list"
                && source.code == JSON_RPC_INVALID_REQUEST_CODE
                && source.data.is_none()
                && source.message == expected_message
    )
}

fn bridge_thread(thread: codex_app_server_protocol::Thread) -> BridgeThread {
    BridgeThread {
        id: thread.id,
        project_id: thread.project_id,
        cwd: thread.cwd,
        ephemeral: thread.ephemeral,
        thread_source: match thread.thread_source {
            Some(ThreadSource::Feature(source)) => Some(source),
            _ => None,
        },
    }
}

fn bridge_queued_submission(queued: QueuedSubmission) -> BridgeQueuedSubmission {
    BridgeQueuedSubmission {
        id: queued.id,
        client_user_message_id: queued.client_user_message_id,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MatrixBridgeError {
    #[error("invalid Matrix App Server bridge configuration: {0}")]
    Invalid(String),
    #[error("Matrix App Server protocol violation: {0}")]
    Protocol(String),
    #[error("Matrix App Server request failed: {0}")]
    AppServer(String),
    #[error(transparent)]
    Agentd(#[from] codex_hepta_agentd::AgentdError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests;
