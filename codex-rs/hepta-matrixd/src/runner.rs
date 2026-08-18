use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_app_server_client::AppServerEvent;
use codex_hepta_agentd::AgentdClient;
use codex_hepta_matrix_sdk::MatrixIngress;
use codex_hepta_matrix_sdk::MatrixSdkClient;
use codex_hepta_matrix_sdk::MatrixSidecarConfig;
use codex_hepta_matrix_sdk::MatrixSyncExit;
use codex_hepta_matrix_sdk::OutboxDispatchConfig;
use codex_hepta_matrix_sdk::run_outbox_sender;
use codex_hepta_matrix_store::MatrixDurableConfig;
use codex_hepta_matrix_store::MatrixDurableStore;
use codex_hepta_matrix_store::RoomBindingDraft;
use codex_utils_absolute_path::AbsolutePathBuf;
use tokio::task::JoinSet;
use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;

use crate::MatrixAgentdConnectArgs;
use crate::MatrixAppServerBridge;
use crate::MatrixBridgeConfig;
use crate::MatrixRuntime;
use crate::MatrixdConfig;

const MATRIXD_CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INBOX_RECOVERY_LIMIT: usize = 1_024;
const INBOX_POLL: Duration = Duration::from_millis(100);
const AGENTD_HEALTH_POLL: Duration = Duration::from_secs(1);
const SHUTDOWN_GRACE: Duration = Duration::from_secs(5);
/// Stable generation of the per-Agent Matrix authority plane.
///
/// `agentd` spawn generations are replaceable execution leases. Matrix
/// cursors, inbox admissions, and stable transaction ids survive those
/// upgrades under the per-Agent process lock and exact binding revision.
const MATRIX_PLANE_GENERATION: u64 = 1;

fn matrix_plane_generation(_agentd_spawn_generation: u64) -> u64 {
    MATRIX_PLANE_GENERATION
}

/// Run one exact-generation Matrix sidecar for one workspace Agent.
///
/// This is deliberately process-local composition: Matrix sync, durable
/// ingress/egress, and the App Server event projector all terminate together.
/// The supervisor remains lifecycle-only and there is no fleet-wide execution
/// gateway or shared writable state.
pub async fn run(config: MatrixdConfig) -> Result<(), MatrixdRunError> {
    prepare_matrix_root(&config)?;
    let _process_lock = acquire_process_lock(&config)?;
    let store = MatrixDurableStore::open(&config.layout, MatrixDurableConfig::default()).await?;
    bind_rooms(&config, &store).await?;

    let workspace_root = AbsolutePathBuf::from_absolute_path(&config.workspace_root)
        .map_err(|error| MatrixdRunError::Invalid(error.to_string()))?;
    let connected = connect_via_agentd(&config).await?;
    let bridge = MatrixAppServerBridge::new(
        MatrixBridgeConfig::new(config.agent_id.clone(), workspace_root),
        connected.transport,
    )?;
    let runtime = Arc::new(MatrixRuntime::new(store.clone(), bridge));
    let sidecar_config = MatrixSidecarConfig {
        binding: config.binding.clone(),
        matrix_generation: matrix_plane_generation(config.spawn_generation),
        sync_timeline_limit: config.sync_timeline_limit,
        sync_timeout: config.sync_timeout,
    };
    sidecar_config.validate(&config.layout)?;
    let (sidecar, _session) = MatrixSdkClient::login_password(
        &config.layout,
        sidecar_config.clone(),
        config.credentials().password(),
        config.credentials().store_passphrase(),
        Some(&config.device_display_name),
    )
    .await?;
    let sidecar = Arc::new(sidecar);
    let ingress = MatrixIngress::new(sidecar_config, store.clone());

    // Reconcile durable work before accepting a new sync cycle.  Failure is
    // fatal: advancing the Matrix cursor while local admission is corrupt
    // would silently lose a user message.
    runtime
        .recover_pending(INBOX_RECOVERY_LIMIT, system_time_ms()?)
        .await?;

    let cancel = CancellationToken::new();
    let mut tasks = JoinSet::new();

    {
        let sidecar = Arc::clone(&sidecar);
        let store = store.clone();
        let ingress = ingress.clone();
        let cancel = cancel.clone();
        tasks.spawn(async move {
            match sidecar
                .sync_durable_until_cancelled(&store, &ingress, &cancel)
                .await?
            {
                MatrixSyncExit::Cancelled if cancel.is_cancelled() => Ok(()),
                MatrixSyncExit::Cancelled => Err(MatrixdRunError::TaskExited("matrix sync")),
                MatrixSyncExit::IngressFenced => Err(MatrixdRunError::IngressFenced),
            }
        });
    }
    {
        let runtime = Arc::clone(&runtime);
        let cancel = cancel.clone();
        tasks.spawn(async move { run_inbox_dispatcher(runtime, cancel).await });
    }
    {
        let runtime = Arc::clone(&runtime);
        let cancel = cancel.clone();
        tasks.spawn(async move { run_event_projector(runtime, connected.events, cancel).await });
    }
    {
        let sidecar = Arc::clone(&sidecar);
        let store = store.clone();
        let cancel = cancel.clone();
        tasks.spawn(async move {
            run_outbox_sender(
                &store,
                sidecar.as_ref(),
                &OutboxDispatchConfig::default(),
                &cancel,
            )
            .await?;
            if cancel.is_cancelled() {
                Ok(())
            } else {
                Err(MatrixdRunError::TaskExited("outbox sender"))
            }
        });
    }
    {
        let cancel = cancel.clone();
        let socket = config.layout.agentd_control_socket().to_path_buf();
        let agent_id = config.agent_id.clone();
        let spawn_generation = config.spawn_generation;
        tasks.spawn(async move {
            run_agentd_health_monitor(socket, agent_id, spawn_generation, cancel).await
        });
    }

    let first_result = tokio::select! {
        signal = shutdown_signal() => signal,
        joined = tasks.join_next() => match joined {
            Some(Ok(Ok(()))) => Err(MatrixdRunError::TaskExited("runtime task")),
            Some(Ok(Err(error))) => Err(error),
            Some(Err(error)) => Err(MatrixdRunError::TaskJoin(error.to_string())),
            None => Err(MatrixdRunError::TaskExited("runtime task set")),
        },
    };

    cancel.cancel();
    let _ = tokio::time::timeout(SHUTDOWN_GRACE, async {
        while tasks.join_next().await.is_some() {}
    })
    .await;
    tasks.abort_all();
    store.close().await;
    first_result
}

fn prepare_matrix_root(config: &MatrixdConfig) -> Result<(), MatrixdRunError> {
    let matrix_root = config.layout.matrix_root();
    fs::create_dir_all(matrix_root)?;
    if matrix_root.canonicalize()? != matrix_root {
        return Err(MatrixdRunError::Invalid(
            "Matrix root must remain the canonical per-Agent path".to_string(),
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(matrix_root, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

async fn connect_via_agentd(
    config: &MatrixdConfig,
) -> Result<crate::ConnectedMatrixAppServer, MatrixdRunError> {
    Ok(crate::connect_via_agentd(MatrixAgentdConnectArgs::new(
        config.layout.agentd_control_socket().to_path_buf(),
        config.agent_id.clone(),
        config.spawn_generation,
        MATRIXD_CLIENT_VERSION,
    ))
    .await?)
}

fn acquire_process_lock(config: &MatrixdConfig) -> Result<File, MatrixdRunError> {
    let lock_path = config.layout.matrix_root().join("matrixd.lock");
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(lock_path)?;
    lock.try_lock()
        .map_err(|_| MatrixdRunError::AlreadyRunning)?;
    Ok(lock)
}

async fn bind_rooms(
    config: &MatrixdConfig,
    store: &MatrixDurableStore,
) -> Result<(), MatrixdRunError> {
    let matrix_generation = matrix_plane_generation(config.spawn_generation);
    for room_id in &config.binding.allowed_rooms {
        let existing = store.room_binding(room_id).await?;
        let expected_revision = match existing.as_ref() {
            None => {
                if config.binding.revision != 1 {
                    return Err(MatrixdRunError::Invalid(
                        "a new Matrix room binding must begin at revision 1".to_string(),
                    ));
                }
                None
            }
            Some(binding)
                if binding.agent_user_id == config.binding.expected_mxid
                    && binding.generation == matrix_generation
                    && binding.revision == config.binding.revision =>
            {
                Some(binding.revision)
            }
            Some(_) => {
                return Err(MatrixdRunError::Invalid(
                    "Matrix room binding disagrees with the stable Matrix-plane generation or revision"
                        .to_string(),
                ));
            }
        };
        let bound = store
            .bind_room(&RoomBindingDraft {
                room_id: room_id.clone(),
                agent_user_id: config.binding.expected_mxid.clone(),
                expected_revision,
                generation: matrix_generation,
                changed_at_ms: system_time_ms()?,
            })
            .await?;
        if bound.revision != config.binding.revision
            || bound.generation != matrix_generation
            || bound.agent_user_id != config.binding.expected_mxid
        {
            return Err(MatrixdRunError::Invalid(
                "Matrix durable room binding did not converge to the configured identity"
                    .to_string(),
            ));
        }
    }
    Ok(())
}

async fn run_inbox_dispatcher<B>(
    runtime: Arc<MatrixRuntime<B>>,
    cancel: CancellationToken,
) -> Result<(), MatrixdRunError>
where
    B: crate::MatrixRuntimeBridge + 'static,
{
    let mut interval = tokio::time::interval(INBOX_POLL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            biased;
            _ = cancel.cancelled() => return Ok(()),
            _ = interval.tick() => {
                runtime
                    .recover_pending(INBOX_RECOVERY_LIMIT, system_time_ms()?)
                    .await?;
            }
        }
    }
}

async fn run_event_projector<B>(
    runtime: Arc<MatrixRuntime<B>>,
    mut events: crate::RemoteMatrixAppServerEvents,
    cancel: CancellationToken,
) -> Result<(), MatrixdRunError>
where
    B: crate::MatrixRuntimeBridge + 'static,
{
    loop {
        let event = tokio::select! {
            biased;
            _ = cancel.cancelled() => return Ok(()),
            event = events.next_event() => event.ok_or(MatrixdRunError::AppServerDisconnected)?,
        };
        // ServerRequest (including approvals) is intentionally ignored by the
        // projector.  Only the owner-local control plane may resolve it.
        if matches!(event, AppServerEvent::ServerRequest(_)) {
            continue;
        }
        runtime
            .project_app_server_event(&event, system_time_ms()?)
            .await?;
    }
}

async fn run_agentd_health_monitor(
    socket: std::path::PathBuf,
    agent_id: codex_hepta_contracts::AgentId,
    spawn_generation: u64,
    cancel: CancellationToken,
) -> Result<(), MatrixdRunError> {
    let client = AgentdClient::new(socket, agent_id, spawn_generation)?;
    let mut interval = tokio::time::interval(AGENTD_HEALTH_POLL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            biased;
            _ = cancel.cancelled() => return Ok(()),
            _ = interval.tick() => {
                let health = client.health().await?;
                if !health.ready || health.fenced {
                    return Err(MatrixdRunError::AgentdFenced);
                }
            }
        }
    }
}

async fn shutdown_signal() -> Result<(), MatrixdRunError> {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => result?,
            _ = terminate.recv() => {},
        }
        Ok(())
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await?;
        Ok(())
    }
}

fn system_time_ms() -> Result<u64, MatrixdRunError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| MatrixdRunError::Clock)?
        .as_millis();
    u64::try_from(millis).map_err(|_| MatrixdRunError::Clock)
}

#[derive(Debug, thiserror::Error)]
pub enum MatrixdRunError {
    #[error("invalid matrixd runtime configuration: {0}")]
    Invalid(String),
    #[error("another matrixd already owns this Agent root")]
    AlreadyRunning,
    #[error("the Matrix ingress durable write path was fenced")]
    IngressFenced,
    #[error("the exact-generation agentd is no longer ready")]
    AgentdFenced,
    #[error("the Agent App Server event stream disconnected")]
    AppServerDisconnected,
    #[error("matrixd runtime task exited unexpectedly: {0}")]
    TaskExited(&'static str),
    #[error("matrixd runtime task failed to join: {0}")]
    TaskJoin(String),
    #[error("system clock is outside the supported range")]
    Clock,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Agentd(#[from] codex_hepta_agentd::AgentdError),
    #[error(transparent)]
    Bridge(#[from] crate::MatrixBridgeError),
    #[error(transparent)]
    Runtime(#[from] crate::MatrixRuntimeError),
    #[error(transparent)]
    Store(#[from] codex_hepta_matrix_store::MatrixDurableError),
    #[error(transparent)]
    Sdk(#[from] codex_hepta_matrix_sdk::MatrixSdkError),
    #[error(transparent)]
    SdkConfig(#[from] codex_hepta_matrix_sdk::MatrixSidecarConfigError),
    #[error(transparent)]
    Outbox(#[from] codex_hepta_matrix_sdk::OutboxDispatchError),
}

#[cfg(test)]
mod tests {
    use super::MATRIX_PLANE_GENERATION;
    use super::matrix_plane_generation;

    #[test]
    fn product_runner_has_one_durable_sync_authority() {
        let source = include_str!("runner.rs");
        let handler_api = ["install_ingress", "_handler"].concat();
        let unsafe_sync_api = ["sync_until", "_cancelled"].concat();
        assert!(source.contains("sync_durable_until_cancelled"));
        assert!(!source.contains(&handler_api));
        assert!(!source.contains(&unsafe_sync_api));
        assert!(
            source.find("acquire_process_lock(&config)")
                < source.find("MatrixDurableStore::open(&config.layout"),
            "the per-Agent matrixd lock must precede SQLite open/migration",
        );
        assert_eq!(MATRIX_PLANE_GENERATION, 1);
        assert_eq!(matrix_plane_generation(1), matrix_plane_generation(2));
        assert_eq!(matrix_plane_generation(u64::MAX), MATRIX_PLANE_GENERATION);
    }
}
