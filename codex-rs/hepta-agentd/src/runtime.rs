use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use codex_app_server_client::RemoteAppServerClient;
use codex_app_server_client::RemoteAppServerConnectArgs;
use codex_app_server_client::RemoteAppServerEndpoint;
use codex_arg0::Arg0DispatchPaths;
use codex_hepta_memory::CognitiveRuntime;
use codex_hepta_memory::CognitiveStore;
use codex_utils_absolute_path::AbsolutePathBuf;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::AgentdConfig;
use crate::AgentdControlServer;
use crate::AgentdError;
use crate::AgentdIdentity;
use crate::AgentdState;
use crate::app_runtime::run_app_server;

const EVENT_CAPACITY: usize = 128;
const GENERATION_POLL_INTERVAL: Duration = Duration::from_millis(50);
const APP_SERVER_PROBE_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompletedRuntimeTask {
    Control,
    AppServer,
    Monitor,
}

pub async fn run(config: AgentdConfig, arg0_paths: Arg0DispatchPaths) -> Result<(), AgentdError> {
    let (identity, registry, writer_lock) = config.into_parts();
    let _writer_lock = writer_lock;
    let state = Arc::new(AgentdState::new(
        identity.clone(),
        registry,
        EVENT_CAPACITY,
    )?);
    let cognitive_layout = identity.layout.clone();
    let cognitive_runtime = open_cognitive_runtime_after_generation_fence(&state, || async move {
        CognitiveStore::open(&cognitive_layout).await
    })
    .await?;
    let cancellation = CancellationToken::new();
    let control = AgentdControlServer::bind(
        identity.control_socket.clone(),
        Arc::clone(&state),
        cancellation.clone(),
    )
    .await?;
    let mut control_task = tokio::spawn(control.run());
    let mut app_server_task = tokio::spawn(run_app_server(
        identity.clone(),
        arg0_paths,
        cognitive_runtime,
    ));
    let mut monitor_task = tokio::spawn(monitor_runtime(Arc::clone(&state)));

    let (outcome, completed_task) = tokio::select! {
        result = &mut control_task => (
            joined("control server", result),
            Some(CompletedRuntimeTask::Control),
        ),
        result = &mut app_server_task => (
            joined_io("Codex App Server", result),
            Some(CompletedRuntimeTask::AppServer),
        ),
        result = &mut monitor_task => (
            joined("generation monitor", result),
            Some(CompletedRuntimeTask::Monitor),
        ),
        signal = shutdown_signal() => {
            signal?;
            state.mark_draining()?;
            (Ok(()), None)
        }
    };
    cancellation.cancel();
    cleanup_runtime_tasks(
        completed_task,
        &mut control_task,
        &mut app_server_task,
        &mut monitor_task,
    )
    .await;
    outcome
}

async fn open_cognitive_runtime_after_generation_fence<Open, OpenFuture>(
    state: &AgentdState,
    open: Open,
) -> Result<CognitiveRuntime, AgentdError>
where
    Open: FnOnce() -> OpenFuture,
    OpenFuture: Future<Output = Result<CognitiveStore, codex_hepta_memory::CognitiveStoreError>>,
{
    state.refresh_generation()?;
    let cognitive_runtime = CognitiveRuntime::from_open_result(open().await);
    // Opening and migrating the store is bounded durable work. Fence again
    // before binding control or starting App Server so a generation change
    // concurrent with that work cannot reach a serving runtime.
    state.refresh_generation()?;
    Ok(cognitive_runtime)
}

async fn monitor_runtime(state: Arc<AgentdState>) -> Result<(), AgentdError> {
    let mut app_server_ready = false;
    loop {
        if let Err(error) = state.refresh_generation() {
            state.mark_fenced();
            return Err(error);
        }
        if !app_server_ready {
            match probe_app_server(state.identity()).await {
                Ok(()) => {
                    state.mark_app_server_ready()?;
                    app_server_ready = true;
                }
                Err(error @ AgentdError::GenerationFenced(_)) => {
                    state.mark_fenced();
                    return Err(error);
                }
                Err(_not_ready) => {}
            }
        }
        tokio::time::sleep(GENERATION_POLL_INTERVAL).await;
    }
}

async fn probe_app_server(identity: &AgentdIdentity) -> Result<(), AgentdError> {
    let socket_path = AbsolutePathBuf::from_absolute_path(&identity.app_server_socket)?;
    let client = timeout(
        APP_SERVER_PROBE_TIMEOUT,
        RemoteAppServerClient::connect(RemoteAppServerConnectArgs {
            endpoint: RemoteAppServerEndpoint::UnixSocket { socket_path },
            client_name: "hepta-agentd-readiness".to_string(),
            client_version: env!("CARGO_PKG_VERSION").to_string(),
            experimental_api: false,
            mcp_server_openai_form_elicitation: false,
            opt_out_notification_methods: Vec::new(),
            channel_capacity: 8,
        }),
    )
    .await
    .map_err(|_| AgentdError::Protocol("App Server readiness probe timed out".to_string()))??;
    let expected_home = identity.home_root.to_string_lossy();
    if client.codex_home() != Some(expected_home.as_ref()) {
        let actual = client.codex_home().unwrap_or("<missing>").to_string();
        let _ = client.shutdown().await;
        return Err(AgentdError::GenerationFenced(format!(
            "App Server home {actual} does not match agent home {expected_home}"
        )));
    }
    client.shutdown().await?;
    Ok(())
}

fn joined(
    label: &str,
    result: Result<Result<(), AgentdError>, tokio::task::JoinError>,
) -> Result<(), AgentdError> {
    match result {
        Ok(Ok(())) => Err(AgentdError::Protocol(format!(
            "{label} exited before agentd shutdown"
        ))),
        Ok(Err(error)) => Err(error),
        Err(error) => Err(AgentdError::Protocol(format!(
            "{label} task failed: {error}"
        ))),
    }
}

fn joined_io(
    label: &str,
    result: Result<std::io::Result<()>, tokio::task::JoinError>,
) -> Result<(), AgentdError> {
    match result {
        Ok(Ok(())) => Err(AgentdError::Protocol(format!(
            "{label} exited before agentd shutdown"
        ))),
        Ok(Err(error)) => Err(error.into()),
        Err(error) => Err(AgentdError::Protocol(format!(
            "{label} task failed: {error}"
        ))),
    }
}

async fn abort_and_join<T>(task: &mut JoinHandle<T>) {
    if !task.is_finished() {
        task.abort();
    }
    let _ = task.await;
}

async fn cleanup_runtime_tasks<ControlOutput, AppServerOutput, MonitorOutput>(
    completed_task: Option<CompletedRuntimeTask>,
    control_task: &mut JoinHandle<ControlOutput>,
    app_server_task: &mut JoinHandle<AppServerOutput>,
    monitor_task: &mut JoinHandle<MonitorOutput>,
) {
    if completed_task != Some(CompletedRuntimeTask::Control) {
        abort_and_join(control_task).await;
    }
    if completed_task != Some(CompletedRuntimeTask::AppServer) {
        abort_and_join(app_server_task).await;
    }
    if completed_task != Some(CompletedRuntimeTask::Monitor) {
        abort_and_join(monitor_task).await;
    }
}

#[cfg(unix)]
async fn shutdown_signal() -> Result<(), AgentdError> {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    terminate.recv().await.ok_or_else(|| {
        AgentdError::Protocol("SIGTERM listener closed before receiving a signal".to_string())
    })
}

#[cfg(not(unix))]
async fn shutdown_signal() -> Result<(), AgentdError> {
    tokio::signal::ctrl_c().await.map_err(Into::into)
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
