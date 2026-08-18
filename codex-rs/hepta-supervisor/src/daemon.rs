use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::ReleaseId;
use codex_hepta_paths::HeptaFleetRoot;
use codex_uds::UnixListener;
use codex_uds::UnixStream;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::time::MissedTickBehavior;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::AgentRelease;
use crate::AgentSupervisorSnapshot;
use crate::Supervisor;
use crate::SupervisorConfig;
use crate::SupervisorError;
use crate::UnixProcessDriver;
use crate::daemon_protocol::MAX_SUPERVISORD_CONTROL_FRAME_BYTES;
use crate::daemon_protocol::MAX_SUPERVISORD_ROSTER;
use crate::daemon_protocol::SUPERVISORD_CONTROL_SCHEMA_VERSION;
use crate::daemon_protocol::SupervisordAgentStatus;
use crate::daemon_protocol::SupervisordHealth;
use crate::daemon_protocol::SupervisordMethod;
use crate::daemon_protocol::SupervisordPayload;
use crate::daemon_protocol::SupervisordRequest;
use crate::daemon_protocol::SupervisordResponse;

const CONNECTION_CAPACITY: usize = 64;
const IO_TIMEOUT: Duration = Duration::from_secs(2);
const TICK_INTERVAL: Duration = Duration::from_millis(25);

struct DaemonState {
    registry: FleetRegistry,
    supervisor: Mutex<Supervisor<UnixProcessDriver>>,
    observed_faults: AtomicU64,
}

/// Runs the one lifecycle-only supervisor daemon for a fleet.
///
/// This service owns no chat ingress, model provider, token stream, tool
/// dispatcher, or execution queue. Every mutation is a bounded per-Agent
/// lifecycle operation.
pub async fn run_supervisord(
    fleet_root: HeptaFleetRoot,
    cancellation: CancellationToken,
) -> Result<(), SupervisorError> {
    let registry = FleetRegistry::open_existing(fleet_root)?;
    let snapshot = registry.load()?;
    if snapshot.agents.len() > usize::from(MAX_SUPERVISORD_ROSTER) {
        return Err(SupervisorError::Invalid(format!(
            "fleet has {} agents but supervisord supports at most {MAX_SUPERVISORD_ROSTER}",
            snapshot.agents.len()
        )));
    }
    let layout = registry.layout().clone();
    let _instance = SingleInstanceLock::acquire(layout.supervisor_lock())?;
    let driver =
        UnixProcessDriver::new(256).map_err(|error| SupervisorError::Invalid(error.to_string()))?;
    let (supervisor, recovery) = Supervisor::recover(
        registry.clone(),
        driver,
        SupervisorConfig::local_default(),
        Instant::now(),
    )?;
    let state = Arc::new(DaemonState {
        registry,
        supervisor: Mutex::new(supervisor),
        observed_faults: AtomicU64::new(recovery.faults.len() as u64),
    });
    let server = SupervisordServer::bind(
        layout.supervisor_socket().to_path_buf(),
        Arc::clone(&state),
        cancellation.clone(),
    )
    .await?;
    let tick_state = Arc::clone(&state);
    let tick_cancellation = cancellation.clone();
    let ticker = tokio::spawn(async move {
        let mut interval = tokio::time::interval(TICK_INTERVAL);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            tokio::select! {
                _ = tick_cancellation.cancelled() => return,
                _ = interval.tick() => {
                    let faults = tick_state.supervisor.lock().await.tick(Instant::now()).faults;
                    tick_state.observed_faults.fetch_add(faults.len() as u64, Ordering::Relaxed);
                }
            }
        }
    });
    let result = server.run().await;
    cancellation.cancel();
    let _ = ticker.await;
    result
}

struct SupervisordServer {
    listener: UnixListener,
    socket_path: PathBuf,
    state: Arc<DaemonState>,
    cancellation: CancellationToken,
    connections: Arc<Semaphore>,
}

impl SupervisordServer {
    async fn bind(
        socket_path: PathBuf,
        state: Arc<DaemonState>,
        cancellation: CancellationToken,
    ) -> Result<Self, SupervisorError> {
        prepare_socket(&socket_path).await?;
        let listener = UnixListener::bind(&socket_path).await?;
        set_owner_only(&socket_path).await?;
        Ok(Self {
            listener,
            socket_path,
            state,
            cancellation,
            connections: Arc::new(Semaphore::new(CONNECTION_CAPACITY)),
        })
    }

    async fn run(mut self) -> Result<(), SupervisorError> {
        loop {
            let stream = tokio::select! {
                _ = self.cancellation.cancelled() => return Ok(()),
                accepted = self.listener.accept() => accepted?,
            };
            let Ok(permit) = Arc::clone(&self.connections).try_acquire_owned() else {
                drop(stream);
                continue;
            };
            let state = Arc::clone(&self.state);
            tokio::spawn(async move {
                let _permit = permit;
                let _ = timeout(IO_TIMEOUT, serve_connection(stream, state)).await;
            });
        }
    }
}

impl Drop for SupervisordServer {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_file(&self.socket_path)
            && error.kind() != ErrorKind::NotFound
        {
            eprintln!(
                "failed to remove supervisord control socket {}: {error}",
                self.socket_path.display()
            );
        }
    }
}

async fn serve_connection(
    stream: UnixStream,
    state: Arc<DaemonState>,
) -> Result<(), SupervisorError> {
    let (reader, mut writer) = tokio::io::split(stream);
    let mut reader = BufReader::new(reader).take(MAX_SUPERVISORD_CONTROL_FRAME_BYTES + 1);
    let mut frame = Vec::new();
    let count = reader.read_until(b'\n', &mut frame).await?;
    if count == 0 || count as u64 > MAX_SUPERVISORD_CONTROL_FRAME_BYTES || !frame.ends_with(b"\n") {
        return Ok(());
    }
    let request: SupervisordRequest = match serde_json::from_slice(&frame) {
        Ok(request) => request,
        Err(error) => {
            write_response(
                &mut writer,
                error_response(0, "invalid_frame", &error.to_string()),
            )
            .await?;
            return Ok(());
        }
    };
    let response = if request.schema_version != SUPERVISORD_CONTROL_SCHEMA_VERSION
        || request.request_id == 0
    {
        error_response(
            request.request_id,
            "unsupported_schema",
            "unsupported supervisord control schema or request identity",
        )
    } else {
        match handle_request(Arc::clone(&state), request.method).await {
            Ok(payload) => SupervisordResponse {
                schema_version: SUPERVISORD_CONTROL_SCHEMA_VERSION,
                request_id: request.request_id,
                payload,
            },
            Err(error) => {
                error_response(request.request_id, "request_rejected", &error.to_string())
            }
        }
    };
    write_response(&mut writer, response).await
}

async fn write_response(
    writer: &mut tokio::io::WriteHalf<UnixStream>,
    response: SupervisordResponse,
) -> Result<(), SupervisorError> {
    let mut bytes = serde_json::to_vec(&response)
        .map_err(|error| SupervisorError::Invalid(format!("encode control response: {error}")))?;
    bytes.push(b'\n');
    if bytes.len() as u64 > MAX_SUPERVISORD_CONTROL_FRAME_BYTES {
        return Err(SupervisorError::Invalid(
            "supervisord response exceeded frame bound".to_string(),
        ));
    }
    writer.write_all(&bytes).await?;
    writer.shutdown().await?;
    Ok(())
}

async fn handle_request(
    state: Arc<DaemonState>,
    method: SupervisordMethod,
) -> Result<SupervisordPayload, SupervisorError> {
    match method {
        SupervisordMethod::Health => {
            let registered_agents = state.registry.load()?.agents.len();
            Ok(SupervisordPayload::Health(SupervisordHealth {
                ready: true,
                process_id: std::process::id(),
                registered_agents: u16::try_from(registered_agents).map_err(|_| {
                    SupervisorError::Invalid("registered agent count exceeds u16".to_string())
                })?,
                observed_faults: state.observed_faults.load(Ordering::Relaxed),
            }))
        }
        SupervisordMethod::Roster { limit } => {
            if !(1..=MAX_SUPERVISORD_ROSTER).contains(&limit) {
                return Err(SupervisorError::Invalid(format!(
                    "roster limit must be 1..={MAX_SUPERVISORD_ROSTER}"
                )));
            }
            let records = state.registry.load()?.agents;
            let supervisor = state.supervisor.lock().await;
            let agents = records
                .into_iter()
                .take(usize::from(limit))
                .map(|(agent_id, record)| {
                    status_from(
                        agent_id.clone(),
                        record.lifecycle.lifecycle,
                        record.lifecycle.generation,
                        supervisor.snapshot(&agent_id),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(SupervisordPayload::Roster { agents })
        }
        SupervisordMethod::Snapshot { agent_id } => Ok(SupervisordPayload::Agent(
            agent_status(&state, &agent_id).await?,
        )),
        SupervisordMethod::Start {
            agent_id,
            release_id,
        } => {
            let release = resolve_release(Arc::clone(&state), agent_id.clone(), release_id).await?;
            state
                .supervisor
                .lock()
                .await
                .start_release(&agent_id, release, Instant::now())?;
            Ok(SupervisordPayload::Agent(
                agent_status(&state, &agent_id).await?,
            ))
        }
        SupervisordMethod::Drain { agent_id } => {
            state
                .supervisor
                .lock()
                .await
                .drain(&agent_id, Instant::now())?;
            Ok(SupervisordPayload::Agent(
                agent_status(&state, &agent_id).await?,
            ))
        }
        SupervisordMethod::Stop { agent_id } => {
            state
                .supervisor
                .lock()
                .await
                .stop(&agent_id, Instant::now())?;
            Ok(SupervisordPayload::Agent(
                agent_status(&state, &agent_id).await?,
            ))
        }
        SupervisordMethod::Kill { agent_id } => {
            state.supervisor.lock().await.kill(&agent_id)?;
            Ok(SupervisordPayload::Agent(
                agent_status(&state, &agent_id).await?,
            ))
        }
        SupervisordMethod::Restart { agent_id } => {
            state
                .supervisor
                .lock()
                .await
                .restart(&agent_id, Instant::now())?;
            Ok(SupervisordPayload::Agent(
                agent_status(&state, &agent_id).await?,
            ))
        }
        SupervisordMethod::Upgrade {
            agent_id,
            release_id,
        } => {
            let release = resolve_release(Arc::clone(&state), agent_id.clone(), release_id).await?;
            state
                .supervisor
                .lock()
                .await
                .upgrade(&agent_id, release, Instant::now())?;
            Ok(SupervisordPayload::Agent(
                agent_status(&state, &agent_id).await?,
            ))
        }
        SupervisordMethod::Rollback { agent_id } => {
            state
                .supervisor
                .lock()
                .await
                .rollback(&agent_id, Instant::now())?;
            Ok(SupervisordPayload::Agent(
                agent_status(&state, &agent_id).await?,
            ))
        }
    }
}

async fn resolve_release(
    state: Arc<DaemonState>,
    agent_id: AgentId,
    release_id: ReleaseId,
) -> Result<AgentRelease, SupervisorError> {
    let registry = state.registry.clone();
    let release =
        tokio::task::spawn_blocking(move || registry.resolve_release(&agent_id, &release_id))
            .await
            .map_err(|error| {
                SupervisorError::Invalid(format!("release resolver task failed: {error}"))
            })??;
    AgentRelease::try_from(release)
}

async fn agent_status(
    state: &DaemonState,
    agent_id: &AgentId,
) -> Result<SupervisordAgentStatus, SupervisorError> {
    let record = state
        .registry
        .load()?
        .agent(agent_id)
        .cloned()
        .ok_or_else(|| SupervisorError::UnknownAgent(agent_id.clone()))?;
    let snapshot = state.supervisor.lock().await.snapshot(agent_id);
    status_from(
        agent_id.clone(),
        record.lifecycle.lifecycle,
        record.lifecycle.generation,
        snapshot,
    )
}

fn status_from(
    agent_id: AgentId,
    lifecycle: AgentLifecycle,
    lifecycle_generation: u64,
    snapshot: Option<AgentSupervisorSnapshot>,
) -> Result<SupervisordAgentStatus, SupervisorError> {
    let snapshot = snapshot.ok_or_else(|| SupervisorError::UnknownAgent(agent_id.clone()))?;
    let current_release = snapshot.active_release.map(ReleaseId::parse).transpose()?;
    let previous_release = snapshot
        .previous_release
        .map(ReleaseId::parse)
        .transpose()?;
    Ok(SupervisordAgentStatus {
        agent_id,
        lifecycle,
        lifecycle_generation,
        active: snapshot.active,
        healthy: snapshot.healthy && lifecycle == AgentLifecycle::Running,
        process_id: snapshot.process_system_id,
        spawn_generation: snapshot.spawn_generation,
        runtime_generation: snapshot.runtime_generation,
        current_release,
        previous_release,
        release_change_pending: snapshot.release_change_pending,
    })
}

fn error_response(request_id: u64, code: &str, message: &str) -> SupervisordResponse {
    SupervisordResponse {
        schema_version: SUPERVISORD_CONTROL_SCHEMA_VERSION,
        request_id,
        payload: SupervisordPayload::Error {
            code: code.to_string(),
            message: message.chars().take(512).collect(),
        },
    }
}

async fn prepare_socket(socket_path: &Path) -> Result<(), SupervisorError> {
    let parent = socket_path.parent().ok_or_else(|| {
        SupervisorError::Invalid("supervisord socket has no parent directory".to_string())
    })?;
    codex_uds::prepare_private_socket_directory(parent).await?;
    match UnixStream::connect(socket_path).await {
        Ok(_) => {
            return Err(SupervisorError::Io(std::io::Error::new(
                ErrorKind::AddrInUse,
                format!(
                    "supervisord socket is already live at {}",
                    socket_path.display()
                ),
            )));
        }
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) if error.kind() == ErrorKind::ConnectionRefused => {}
        Err(_error) if !socket_path.exists() => return Ok(()),
        Err(error) => return Err(error.into()),
    }
    if codex_uds::is_stale_socket_path(socket_path).await? {
        tokio::fs::remove_file(socket_path).await?;
        Ok(())
    } else {
        Err(SupervisorError::Io(std::io::Error::new(
            ErrorKind::AlreadyExists,
            format!(
                "supervisord socket path is not a stale socket: {}",
                socket_path.display()
            ),
        )))
    }
}

#[cfg(unix)]
async fn set_owner_only(path: &Path) -> Result<(), SupervisorError> {
    use std::os::unix::fs::PermissionsExt;
    tokio::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).await?;
    Ok(())
}

#[cfg(not(unix))]
async fn set_owner_only(_path: &Path) -> Result<(), SupervisorError> {
    Ok(())
}

struct SingleInstanceLock {
    file: File,
}

impl SingleInstanceLock {
    fn acquire(path: &Path) -> Result<Self, SupervisorError> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)?;
        set_lock_owner_only(path)?;
        // SAFETY: flock only operates on this live File descriptor with fixed flags.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == -1 {
            let error = std::io::Error::last_os_error();
            return Err(SupervisorError::Io(std::io::Error::new(
                ErrorKind::AddrInUse,
                format!("another supervisord owns {}: {error}", path.display()),
            )));
        }
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        writeln!(file, "{}", std::process::id())?;
        file.sync_all()?;
        Ok(Self { file })
    }
}

impl Drop for SingleInstanceLock {
    fn drop(&mut self) {
        // SAFETY: unlocks the same live File descriptor acquired above.
        let _ = unsafe { libc::flock(self.file.as_raw_fd(), libc::LOCK_UN) };
    }
}

#[cfg(unix)]
fn set_lock_owner_only(path: &Path) -> Result<(), SupervisorError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_lock_owner_only(path: &Path) -> Result<(), SupervisorError> {
    let mut permissions = std::fs::metadata(path)?.permissions();
    permissions.set_readonly(false);
    std::fs::set_permissions(path, permissions)?;
    Ok(())
}
