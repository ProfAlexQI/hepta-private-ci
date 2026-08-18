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
use codex_hepta_fleet::FleetRegistryError;
use codex_hepta_fleet::ReleaseId;
use codex_hepta_paths::HeptaFleetRoot;
use codex_uds::UnixListener;
use codex_uds::UnixStream;
use constant_time_eq::constant_time_eq_32;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
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
use crate::ProcessDriver;
use crate::Supervisor;
use crate::SupervisorConfig;
use crate::SupervisorError;
use crate::UnixProcessDriver;
use crate::daemon_protocol::ControlStateDigest;
use crate::daemon_protocol::MAX_SUPERVISORD_CONTROL_FRAME_BYTES;
use crate::daemon_protocol::MAX_SUPERVISORD_ROSTER;
use crate::daemon_protocol::SUPERVISORD_CONTROL_SCHEMA_VERSION;
use crate::daemon_protocol::SupervisorEpoch;
use crate::daemon_protocol::SupervisordAgentStatus;
use crate::daemon_protocol::SupervisordControlFence;
use crate::daemon_protocol::SupervisordHealth;
use crate::daemon_protocol::SupervisordMethod;
use crate::daemon_protocol::SupervisordMutation;
use crate::daemon_protocol::SupervisordPayload;
use crate::daemon_protocol::SupervisordRequest;
use crate::daemon_protocol::SupervisordRequestValidationError;
use crate::daemon_protocol::SupervisordResponse;

const CONNECTION_CAPACITY: usize = 64;
const IO_TIMEOUT: Duration = Duration::from_secs(2);
const TICK_INTERVAL: Duration = Duration::from_millis(25);
const CONTROL_STATE_DIGEST_DOMAIN: &[u8] = b"hepta.supervisord.control-state.v2\0";
const CONTROL_FENCE_IDENTITY_DOMAIN: &[u8] = b"hepta.supervisord.control-fence-identity.v2\0";

struct DaemonState<D: ProcessDriver = UnixProcessDriver> {
    registry: FleetRegistry,
    supervisor: Mutex<Supervisor<D>>,
    supervisor_epoch: SupervisorEpoch,
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
        supervisor_epoch: SupervisorEpoch::new(),
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
    codex_uds::ensure_current_user_peer(&stream)?;
    let (reader, mut writer) = tokio::io::split(stream);
    let mut reader = BufReader::new(reader).take(MAX_SUPERVISORD_CONTROL_FRAME_BYTES + 1);
    let mut frame = Vec::new();
    let count = reader.read_until(b'\n', &mut frame).await?;
    if count == 0 || count as u64 > MAX_SUPERVISORD_CONTROL_FRAME_BYTES || !frame.ends_with(b"\n") {
        return Ok(());
    }
    let request: SupervisordRequest = match serde_json::from_slice(&frame) {
        Ok(request) => request,
        Err(_) => {
            write_response(
                &mut writer,
                error_response(
                    0,
                    "invalid_frame",
                    "request is not valid supervisord control JSON",
                    None,
                ),
            )
            .await?;
            return Ok(());
        }
    };
    let response = match request.validate() {
        Err(SupervisordRequestValidationError::UnsupportedSchema) => error_response(
            request.request_id,
            "unsupported_schema",
            "unsupported supervisord control schema",
            None,
        ),
        Err(SupervisordRequestValidationError::InvalidRequest) => error_response(
            request.request_id,
            "invalid_frame",
            "request is not valid supervisord control JSON",
            None,
        ),
        Ok(()) => SupervisordResponse {
            schema_version: SUPERVISORD_CONTROL_SCHEMA_VERSION,
            request_id: request.request_id,
            payload: handle_request(Arc::clone(&state), request.method).await,
        },
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

async fn handle_request<D: ProcessDriver>(
    state: Arc<DaemonState<D>>,
    method: SupervisordMethod,
) -> SupervisordPayload {
    match method {
        SupervisordMethod::Health => {
            let registered_agents = match state.registry.load() {
                Ok(snapshot) => snapshot.agents.len(),
                Err(error) => return safe_rejection(error.into(), None, false),
            };
            let registered_agents = match u16::try_from(registered_agents) {
                Ok(count) => count,
                Err(_) => {
                    return safe_rejection(
                        SupervisorError::Invalid("registered agent count exceeds u16".to_string()),
                        None,
                        false,
                    );
                }
            };
            SupervisordPayload::Health(SupervisordHealth {
                ready: true,
                supervisor_epoch: state.supervisor_epoch.clone(),
                process_id: std::process::id(),
                registered_agents,
                observed_faults: state.observed_faults.load(Ordering::Relaxed),
            })
        }
        SupervisordMethod::Roster { limit } => {
            if !(1..=MAX_SUPERVISORD_ROSTER).contains(&limit) {
                return error_payload(
                    "invalid_frame",
                    "request is not valid supervisord control JSON",
                    None,
                );
            }
            let supervisor = state.supervisor.lock().await;
            let records = match state.registry.load() {
                Ok(snapshot) => snapshot.agents,
                Err(error) => return safe_rejection(error.into(), None, false),
            };
            let agents = match records
                .into_iter()
                .take(usize::from(limit))
                .map(|(agent_id, record)| {
                    status_from(
                        &state.supervisor_epoch,
                        &record,
                        supervisor.snapshot(&agent_id),
                    )
                })
                .collect::<Result<Vec<_>, _>>()
            {
                Ok(agents) => agents,
                Err(error) => return safe_rejection(error, None, false),
            };
            SupervisordPayload::Roster { agents }
        }
        SupervisordMethod::Snapshot { agent_id } => match agent_status(&state, &agent_id).await {
            Ok(status) => SupervisordPayload::Agent(status),
            Err(error) => safe_rejection(error, None, false),
        },
        SupervisordMethod::Start { fence, release_id } => {
            let target = match resolve_release_outside_lock(
                Arc::clone(&state),
                fence.agent_id.clone(),
                release_id,
            )
            .await
            {
                Ok(target) => target,
                Err(error) => {
                    let actual = agent_status(&state, &fence.agent_id).await.ok();
                    return safe_rejection(error, actual, false);
                }
            };
            handle_mutation(state, SupervisordMutation::Start, fence, Some(target)).await
        }
        SupervisordMethod::Drain { fence } => {
            handle_mutation(state, SupervisordMutation::Drain, fence, None).await
        }
        SupervisordMethod::Stop { fence } => {
            handle_mutation(state, SupervisordMutation::Stop, fence, None).await
        }
        SupervisordMethod::Kill { fence } => {
            handle_mutation(state, SupervisordMutation::Kill, fence, None).await
        }
        SupervisordMethod::Restart { fence } => {
            handle_mutation(state, SupervisordMutation::Restart, fence, None).await
        }
        SupervisordMethod::Upgrade { fence, release_id } => {
            let target = match resolve_release_outside_lock(
                Arc::clone(&state),
                fence.agent_id.clone(),
                release_id,
            )
            .await
            {
                Ok(target) => target,
                Err(error) => {
                    let actual = agent_status(&state, &fence.agent_id).await.ok();
                    return safe_rejection(error, actual, false);
                }
            };
            handle_mutation(state, SupervisordMutation::Upgrade, fence, Some(target)).await
        }
        SupervisordMethod::Rollback { fence } => {
            handle_mutation(state, SupervisordMutation::Rollback, fence, None).await
        }
    }
}

async fn handle_mutation<D: ProcessDriver>(
    state: Arc<DaemonState<D>>,
    operation: SupervisordMutation,
    fence: SupervisordControlFence,
    target: Option<AgentRelease>,
) -> SupervisordPayload {
    let agent_id = fence.agent_id.clone();
    let accepted_state_digest = fence.state_digest.clone();
    let mut supervisor = state.supervisor.lock().await;
    let actual = match agent_status_locked(&state, &supervisor, &agent_id) {
        Ok(actual) => actual,
        Err(error) => return safe_rejection(error, None, false),
    };
    if !control_fence_matches(&fence, &actual.control_fence) {
        return error_payload(
            "stale_control_fence",
            "selected Agent changed; refresh before retry",
            Some(actual),
        );
    }

    let prepared = match (operation, target) {
        (SupervisordMutation::Start, Some(target)) => PreparedMutation::Start(target),
        (SupervisordMutation::Drain, None) => PreparedMutation::Drain,
        (SupervisordMutation::Stop, None) => PreparedMutation::Stop,
        (SupervisordMutation::Kill, None) => PreparedMutation::Kill,
        (SupervisordMutation::Restart, None) => PreparedMutation::Restart,
        (SupervisordMutation::Upgrade, Some(target)) => PreparedMutation::Upgrade(target),
        (SupervisordMutation::Rollback, None) => PreparedMutation::Rollback,
        (SupervisordMutation::Start | SupervisordMutation::Upgrade, None) => {
            return error_payload(
                "invalid_frame",
                "request is not valid supervisord control JSON",
                Some(actual),
            );
        }
        (
            SupervisordMutation::Drain
            | SupervisordMutation::Stop
            | SupervisordMutation::Kill
            | SupervisordMutation::Restart
            | SupervisordMutation::Rollback,
            Some(_),
        ) => {
            return error_payload(
                "invalid_frame",
                "request is not valid supervisord control JSON",
                Some(actual),
            );
        }
    };

    let preflight = match &prepared {
        PreparedMutation::Start(_) => supervisor.preflight_start(&agent_id),
        PreparedMutation::Drain => supervisor.preflight_drain(&agent_id),
        PreparedMutation::Stop | PreparedMutation::Kill => {
            supervisor.preflight_stop_or_kill(&agent_id)
        }
        PreparedMutation::Restart => supervisor.preflight_restart(&agent_id),
        PreparedMutation::Upgrade(target) => supervisor.preflight_upgrade(&agent_id, target),
        PreparedMutation::Rollback => supervisor.preflight_rollback(&agent_id),
    };
    if let Err(error) = preflight {
        let refreshed = agent_status_locked(&state, &supervisor, &agent_id).ok();
        return safe_rejection(error, refreshed.or(Some(actual)), false);
    }

    let next_revision = match supervisor.next_control_revision(&agent_id) {
        Ok(revision) => revision,
        Err(error) => return safe_rejection(error, Some(actual), false),
    };
    if let Err(error) = supervisor.set_control_revision(&agent_id, next_revision) {
        return safe_rejection(error, Some(actual), false);
    }

    let mutation = match prepared {
        PreparedMutation::Start(target) => {
            supervisor.start_release(&agent_id, target, Instant::now())
        }
        PreparedMutation::Drain => supervisor.drain(&agent_id, Instant::now()),
        PreparedMutation::Stop => supervisor.stop(&agent_id, Instant::now()),
        PreparedMutation::Kill => supervisor.kill(&agent_id),
        PreparedMutation::Restart => supervisor.restart(&agent_id, Instant::now()),
        PreparedMutation::Upgrade(target) => supervisor.upgrade(&agent_id, target, Instant::now()),
        PreparedMutation::Rollback => supervisor.rollback(&agent_id, Instant::now()),
    };
    let post = agent_status_locked(&state, &supervisor, &agent_id).ok();
    if let Err(_error) = mutation {
        return error_payload(
            "operation_indeterminate",
            "operation outcome is indeterminate; refresh before retry",
            post,
        );
    }
    let Some(agent) = post else {
        return error_payload(
            "operation_indeterminate",
            "operation outcome is indeterminate; refresh before retry",
            None,
        );
    };
    SupervisordPayload::MutationAccepted {
        operation,
        accepted_state_digest,
        agent,
    }
}

enum PreparedMutation {
    Start(AgentRelease),
    Drain,
    Stop,
    Kill,
    Restart,
    Upgrade(AgentRelease),
    Rollback,
}

async fn resolve_release_outside_lock<D: ProcessDriver>(
    state: Arc<DaemonState<D>>,
    agent_id: AgentId,
    release_id: ReleaseId,
) -> Result<AgentRelease, SupervisorError> {
    let registry = state.registry.clone();
    let release =
        tokio::task::spawn_blocking(move || registry.resolve_release(&agent_id, &release_id))
            .await
            .map_err(|_| SupervisorError::Invalid("release resolver task failed".to_string()))??;
    AgentRelease::try_from(release)
}

async fn agent_status<D: ProcessDriver>(
    state: &DaemonState<D>,
    agent_id: &AgentId,
) -> Result<SupervisordAgentStatus, SupervisorError> {
    let supervisor = state.supervisor.lock().await;
    agent_status_locked(state, &supervisor, agent_id)
}

fn agent_status_locked<D: ProcessDriver>(
    state: &DaemonState<D>,
    supervisor: &Supervisor<D>,
    agent_id: &AgentId,
) -> Result<SupervisordAgentStatus, SupervisorError> {
    let record = state
        .registry
        .load()?
        .agent(agent_id)
        .cloned()
        .ok_or_else(|| SupervisorError::UnknownAgent(agent_id.clone()))?;
    status_from(
        &state.supervisor_epoch,
        &record,
        supervisor.snapshot(agent_id),
    )
}

fn status_from(
    supervisor_epoch: &SupervisorEpoch,
    record: &codex_hepta_fleet::AgentRecord,
    snapshot: Option<AgentSupervisorSnapshot>,
) -> Result<SupervisordAgentStatus, SupervisorError> {
    let agent_id = record.manifest.agent_id.clone();
    let snapshot = snapshot.ok_or_else(|| SupervisorError::UnknownAgent(agent_id.clone()))?;
    let current_release = snapshot
        .active_release
        .clone()
        .map(ReleaseId::parse)
        .transpose()?;
    let previous_release = snapshot
        .previous_release
        .clone()
        .map(ReleaseId::parse)
        .transpose()?;
    let mut status = SupervisordAgentStatus {
        agent_id: agent_id.clone(),
        lifecycle: record.lifecycle.lifecycle,
        lifecycle_generation: record.lifecycle.generation,
        active: snapshot.active,
        healthy: snapshot.healthy && record.lifecycle.lifecycle == AgentLifecycle::Running,
        process_id: snapshot.process_system_id,
        spawn_generation: snapshot.spawn_generation,
        runtime_generation: snapshot.runtime_generation,
        current_release: current_release.clone(),
        previous_release: previous_release.clone(),
        release_change_pending: snapshot.release_change_pending,
        control_fence: SupervisordControlFence {
            agent_id,
            supervisor_epoch: supervisor_epoch.clone(),
            lifecycle: record.lifecycle.lifecycle,
            lifecycle_generation: record.lifecycle.generation,
            spawn_generation: snapshot.spawn_generation,
            runtime_generation: snapshot.runtime_generation,
            current_release,
            previous_release,
            release_change_pending: snapshot.release_change_pending,
            state_digest: ControlStateDigest::from_bytes([0_u8; 32]),
        },
    };
    status.control_fence.state_digest =
        control_state_digest(&status.control_fence, &status, record, &snapshot)?;
    Ok(status)
}

fn control_fence_matches(
    provided: &SupervisordControlFence,
    actual: &SupervisordControlFence,
) -> bool {
    let (Ok(provided_identity), Ok(actual_identity)) = (
        fence_identity_digest(provided),
        fence_identity_digest(actual),
    ) else {
        return false;
    };
    let identity_matches = constant_time_eq_32(&provided_identity, &actual_identity);
    let state_matches = constant_time_eq_32(
        &provided.state_digest.decode(),
        &actual.state_digest.decode(),
    );
    identity_matches & state_matches
}

#[derive(Serialize)]
struct FenceIdentity<'a> {
    agent_id: &'a AgentId,
    supervisor_epoch: &'a SupervisorEpoch,
    lifecycle: AgentLifecycle,
    lifecycle_generation: u64,
    spawn_generation: Option<u64>,
    runtime_generation: Option<u64>,
    current_release: &'a Option<ReleaseId>,
    previous_release: &'a Option<ReleaseId>,
    release_change_pending: bool,
}

impl<'a> From<&'a SupervisordControlFence> for FenceIdentity<'a> {
    fn from(fence: &'a SupervisordControlFence) -> Self {
        Self {
            agent_id: &fence.agent_id,
            supervisor_epoch: &fence.supervisor_epoch,
            lifecycle: fence.lifecycle,
            lifecycle_generation: fence.lifecycle_generation,
            spawn_generation: fence.spawn_generation,
            runtime_generation: fence.runtime_generation,
            current_release: &fence.current_release,
            previous_release: &fence.previous_release,
            release_change_pending: fence.release_change_pending,
        }
    }
}

#[derive(Serialize)]
struct HiddenControlState<'a> {
    control_revision: u64,
    active: bool,
    healthy: bool,
    process_id: Option<u64>,
    release_state_generation: u64,
    registry_current_release: &'a Option<ReleaseId>,
    registry_previous_release: &'a Option<ReleaseId>,
    restart_pending: bool,
    runtime_phase: &'a Option<crate::ControlRuntimePhase>,
    runtime_release: &'a Option<String>,
    runtime_incarnation: &'a Option<String>,
    runtime_fenced: bool,
    release_change: &'a Option<crate::ControlReleaseChange>,
    has_last_command: bool,
}

#[derive(Serialize)]
struct ControlStateMaterial<'a> {
    fence: FenceIdentity<'a>,
    hidden: HiddenControlState<'a>,
}

fn control_state_digest(
    fence: &SupervisordControlFence,
    status: &SupervisordAgentStatus,
    record: &codex_hepta_fleet::AgentRecord,
    snapshot: &AgentSupervisorSnapshot,
) -> Result<ControlStateDigest, SupervisorError> {
    let material = ControlStateMaterial {
        fence: FenceIdentity::from(fence),
        hidden: HiddenControlState {
            control_revision: snapshot.control_revision,
            active: status.active,
            healthy: status.healthy,
            process_id: status.process_id,
            release_state_generation: snapshot.release_state_generation,
            registry_current_release: &record.release_state.current,
            registry_previous_release: &record.release_state.previous,
            restart_pending: snapshot.restart_pending,
            runtime_phase: &snapshot.runtime_phase,
            runtime_release: &snapshot.runtime_release,
            runtime_incarnation: &snapshot.runtime_incarnation,
            runtime_fenced: snapshot.runtime_fenced,
            release_change: &snapshot.release_change,
            has_last_command: snapshot.has_last_command,
        },
    };
    let encoded = serde_json::to_vec(&material)
        .map_err(|_| SupervisorError::Invalid("encode control state digest".to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(CONTROL_STATE_DIGEST_DOMAIN);
    hasher.update(encoded);
    Ok(ControlStateDigest::from_bytes(hasher.finalize().into()))
}

fn fence_identity_digest(fence: &SupervisordControlFence) -> Result<[u8; 32], serde_json::Error> {
    let encoded = serde_json::to_vec(&FenceIdentity::from(fence))?;
    let mut hasher = Sha256::new();
    hasher.update(CONTROL_FENCE_IDENTITY_DOMAIN);
    hasher.update(encoded);
    Ok(hasher.finalize().into())
}

fn safe_rejection(
    error: SupervisorError,
    actual: Option<SupervisordAgentStatus>,
    mutation_started: bool,
) -> SupervisordPayload {
    if mutation_started {
        return error_payload(
            "operation_indeterminate",
            "operation outcome is indeterminate; refresh before retry",
            actual,
        );
    }
    match error {
        SupervisorError::UnknownAgent(_) => {
            error_payload("unknown_agent", "selected Agent is not registered", None)
        }
        SupervisorError::Registry(FleetRegistryError::UnknownRelease(_)) => error_payload(
            "release_not_found",
            "selected release is not installed",
            actual,
        ),
        SupervisorError::Registry(FleetRegistryError::ReleaseNotAllowed { .. }) => error_payload(
            "release_not_allowed",
            "selected release is not allowed for this Agent",
            actual,
        ),
        SupervisorError::NoPreviousRelease(_) => error_payload(
            "no_previous_release",
            "selected Agent has no previous release",
            actual,
        ),
        SupervisorError::NoPreviousCommand(_) => error_payload(
            "no_previous_command",
            "selected Agent has no runnable release",
            actual,
        ),
        SupervisorError::ReleaseChangePending(_) => error_payload(
            "release_change_pending",
            "selected Agent already has a lifecycle change in progress",
            actual,
        ),
        SupervisorError::TargetReleaseUnchanged(_) => error_payload(
            "target_release_unchanged",
            "selected release is already active for this Agent",
            actual,
        ),
        SupervisorError::UnresolvedLease(_) => error_payload(
            "unresolved_lease",
            "selected Agent has an unresolved process lease",
            actual,
        ),
        SupervisorError::GenerationFence { .. }
        | SupervisorError::Registry(FleetRegistryError::StaleGeneration { .. })
        | SupervisorError::Registry(FleetRegistryError::StaleReleaseGeneration { .. }) => {
            error_payload(
                "generation_fenced",
                "selected Agent generation changed; refresh before retry",
                actual,
            )
        }
        SupervisorError::Registry(FleetRegistryError::InvalidTransition { .. })
        | SupervisorError::AlreadyActive(_)
        | SupervisorError::Invalid(_) => error_payload(
            "invalid_transition",
            "selected Agent cannot transition from its current state",
            actual,
        ),
        SupervisorError::Driver { .. } => error_payload(
            "control_state_unavailable",
            "Agent control state is unavailable; refresh before retry",
            actual,
        ),
        SupervisorError::CorruptLease(_)
        | SupervisorError::Registry(_)
        | SupervisorError::Io(_) => error_payload(
            "control_state_unavailable",
            "Agent control state is unavailable; refresh before retry",
            actual,
        ),
    }
}

fn error_payload(
    code: &str,
    message: &str,
    actual: Option<SupervisordAgentStatus>,
) -> SupervisordPayload {
    SupervisordPayload::Error {
        code: code.to_string(),
        message: message.to_string(),
        actual,
    }
}

fn error_response(
    request_id: u64,
    code: &str,
    message: &str,
    actual: Option<SupervisordAgentStatus>,
) -> SupervisordResponse {
    SupervisordResponse {
        schema_version: SUPERVISORD_CONTROL_SCHEMA_VERSION,
        request_id,
        payload: SupervisordPayload::Error {
            code: code.to_string(),
            message: message.to_string(),
            actual,
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

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_fleet::AgentLifecycle;
    use codex_hepta_fleet::ReleaseId;

    use super::*;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
    const PEER_AGENT_ID: &str = "019153a4-3088-7e03-a56a-9b1964f75dd3";
    const EPOCH: &str = "018f4f72-5f8f-4cc1-8f55-df9fb3aa2c12";
    const PEER_EPOCH: &str = "019153a4-3088-4e03-a56a-9b1964f75dd3";
    const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const PEER_DIGEST: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    fn fence() -> SupervisordControlFence {
        SupervisordControlFence {
            agent_id: AgentId::parse(AGENT_ID).expect("fixed AgentId"),
            supervisor_epoch: SupervisorEpoch::parse(EPOCH).expect("fixed epoch"),
            lifecycle: AgentLifecycle::Running,
            lifecycle_generation: 7,
            spawn_generation: Some(5),
            runtime_generation: Some(7),
            current_release: Some(ReleaseId::parse("agentd-v1").expect("fixed release")),
            previous_release: None,
            release_change_pending: false,
            state_digest: ControlStateDigest::parse(DIGEST).expect("fixed digest"),
        }
    }

    #[test]
    fn every_external_fence_field_participates_in_constant_time_cas_identity() {
        let actual = fence();
        assert!(control_fence_matches(&actual, &actual));

        let mut stale_fences = Vec::new();
        let mut stale = actual.clone();
        stale.agent_id = AgentId::parse(PEER_AGENT_ID).expect("fixed peer AgentId");
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.supervisor_epoch = SupervisorEpoch::parse(PEER_EPOCH).expect("fixed peer epoch");
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.lifecycle = AgentLifecycle::Draining;
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.lifecycle_generation += 1;
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.spawn_generation = Some(6);
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.runtime_generation = Some(8);
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.current_release = Some(ReleaseId::parse("agentd-v2").expect("fixed release"));
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.previous_release =
            Some(ReleaseId::parse("agentd-v0").expect("fixed previous release"));
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.release_change_pending = true;
        stale_fences.push(stale);
        let mut stale = actual.clone();
        stale.state_digest = ControlStateDigest::parse(PEER_DIGEST).expect("fixed peer digest");
        stale_fences.push(stale);

        for stale in stale_fences {
            assert!(
                !control_fence_matches(&stale, &actual),
                "accepted {stale:?}"
            );
        }
    }

    #[test]
    fn wire_errors_are_closed_and_never_expose_driver_or_filesystem_detail() {
        let agent_id = AgentId::parse(AGENT_ID).expect("fixed AgentId");
        let cases = [
            safe_rejection(SupervisorError::UnknownAgent(agent_id.clone()), None, false),
            safe_rejection(
                SupervisorError::NoPreviousRelease(agent_id.clone()),
                None,
                false,
            ),
            safe_rejection(
                SupervisorError::NoPreviousCommand(agent_id.clone()),
                None,
                false,
            ),
            safe_rejection(
                SupervisorError::ReleaseChangePending(agent_id.clone()),
                None,
                false,
            ),
            safe_rejection(
                SupervisorError::UnresolvedLease(agent_id.clone()),
                None,
                false,
            ),
            safe_rejection(
                SupervisorError::GenerationFence {
                    agent_id: agent_id.clone(),
                    runtime: 1,
                    registry: 2,
                },
                None,
                false,
            ),
            safe_rejection(
                SupervisorError::TargetReleaseUnchanged(agent_id.clone()),
                None,
                false,
            ),
            safe_rejection(
                SupervisorError::Driver {
                    agent_id,
                    message: "/secret/bin/agentd --token raw-driver-secret".to_string(),
                },
                None,
                false,
            ),
        ];
        let allowed = [
            "unknown_agent",
            "invalid_transition",
            "release_change_pending",
            "release_not_found",
            "release_not_allowed",
            "target_release_unchanged",
            "no_previous_release",
            "no_previous_command",
            "unresolved_lease",
            "generation_fenced",
            "control_state_unavailable",
            "operation_indeterminate",
            "invalid_frame",
            "unsupported_schema",
        ];
        for payload in cases {
            let SupervisordPayload::Error { code, .. } = &payload else {
                panic!("safe rejection returned a non-error payload");
            };
            assert!(
                allowed.contains(&code.as_str()),
                "unapproved wire code {code}"
            );
            let encoded = serde_json::to_string(&payload).expect("serialize safe error");
            assert!(!encoded.contains("/secret"));
            assert!(!encoded.contains("raw-driver-secret"));
            assert!(!encoded.contains("--token"));
        }
    }
}

#[cfg(not(unix))]
fn set_lock_owner_only(path: &Path) -> Result<(), SupervisorError> {
    let mut permissions = std::fs::metadata(path)?.permissions();
    permissions.set_readonly(false);
    std::fs::set_permissions(path, permissions)?;
    Ok(())
}
