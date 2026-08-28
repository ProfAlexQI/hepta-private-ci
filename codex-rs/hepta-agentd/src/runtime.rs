use std::sync::Arc;
use std::time::Duration;

#[cfg(test)]
use std::future::Future;

use codex_app_server_client::RemoteAppServerClient;
use codex_app_server_client::RemoteAppServerConnectArgs;
use codex_app_server_client::RemoteAppServerEndpoint;
use codex_arg0::Arg0DispatchPaths;
#[cfg(test)]
use codex_hepta_automation::AutomationError;
#[cfg(test)]
use codex_hepta_automation::AutomationStore;
#[cfg(test)]
use codex_hepta_memory::CognitiveRuntime;
#[cfg(test)]
use codex_hepta_memory::CognitiveStore;
#[cfg(test)]
use codex_hepta_memory::CognitiveStoreError;
use codex_utils_absolute_path::AbsolutePathBuf;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::AgentdConfig;
use crate::AgentdControlServer;
use crate::AgentdError;
use crate::AgentdIdentity;
use crate::AgentdState;
use crate::app_runtime::AgentAppServerService;
use crate::composition::AgentRuntimeComposition;
use crate::composition::AgentRuntimeParts;

#[cfg(test)]
const EVENT_CAPACITY: usize = 128;
const GENERATION_POLL_INTERVAL: Duration = Duration::from_millis(50);
const APP_SERVER_PROBE_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompletedRuntimeTask {
    Control,
    AppServer,
    Monitor,
    Automation,
}

pub async fn run(config: AgentdConfig, arg0_paths: Arg0DispatchPaths) -> Result<(), AgentdError> {
    let AgentRuntimeParts {
        identity,
        state,
        memory_service,
        automation_service,
        authority,
        product_graph: _product_graph,
        writer_lock: _writer_lock,
    } = AgentRuntimeComposition::open(config).await?.into_parts();

    let app_server = AgentAppServerService::new(
        identity.clone(),
        arg0_paths,
        memory_service.into_runtime(),
        authority,
        Arc::clone(&state),
    )?;
    let cancellation = CancellationToken::new();
    let control = AgentdControlServer::bind(
        identity.control_socket.clone(),
        Arc::clone(&state),
        cancellation.clone(),
    )
    .await?;
    let mut control_task = tokio::spawn(control.run());
    let mut app_server_task = tokio::spawn(app_server.run());
    let mut monitor_task = tokio::spawn(monitor_runtime(Arc::clone(&state)));
    let mut automation_task = tokio::spawn(automation_service.run(
        Arc::clone(&state),
        identity,
        cancellation.clone(),
    ));

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
        result = &mut automation_task => (
            joined("automation scheduler", result),
            Some(CompletedRuntimeTask::Automation),
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
        &mut automation_task,
    )
    .await;
    outcome
}

#[cfg(feature = "qualification-cognitive-write")]
#[cfg(test)]
fn require_cognitive_runtime_for_profile(
    runtime: CognitiveRuntime,
) -> Result<CognitiveRuntime, AgentdError> {
    if runtime.available_store().is_some() {
        Ok(runtime)
    } else {
        Err(AgentdError::QualificationCognitiveRuntimeUnavailable)
    }
}

#[cfg(not(feature = "qualification-cognitive-write"))]
#[cfg(test)]
fn require_cognitive_runtime_for_profile(
    runtime: CognitiveRuntime,
) -> Result<CognitiveRuntime, AgentdError> {
    Ok(runtime)
}

#[cfg(test)]
async fn open_automation_store_after_generation_fence<Open, OpenFuture>(
    state: &AgentdState,
    open: Open,
) -> Result<Option<AutomationStore>, AgentdError>
where
    Open: FnOnce() -> OpenFuture,
    OpenFuture: Future<Output = Result<AutomationStore, AutomationError>>,
{
    state.refresh_generation()?;
    let opened = open().await;
    state.refresh_generation()?;
    match opened {
        Ok(store) => Ok(Some(store)),
        Err(AutomationError::Unavailable | AutomationError::Corrupt) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

#[cfg(test)]
async fn open_cognitive_runtime_after_generation_fence<Open, OpenFuture>(
    state: &AgentdState,
    open: Open,
) -> Result<CognitiveRuntime, AgentdError>
where
    Open: FnOnce() -> OpenFuture,
    OpenFuture: Future<Output = Result<CognitiveStore, CognitiveStoreError>>,
{
    state.refresh_generation()?;
    let runtime = CognitiveRuntime::from_open_result(open().await);
    state.refresh_generation()?;
    Ok(runtime)
}

async fn monitor_runtime(state: Arc<AgentdState>) -> Result<(), AgentdError> {
    let mut app_server_ready = false;
    loop {
        if state.is_fenced()? {
            return Err(AgentdError::GenerationFenced(
                "agentd runtime was fenced by an owner or generation violation".to_string(),
            ));
        }
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

async fn cleanup_runtime_tasks<ControlOutput, AppServerOutput, MonitorOutput, AutomationOutput>(
    completed_task: Option<CompletedRuntimeTask>,
    control_task: &mut JoinHandle<ControlOutput>,
    app_server_task: &mut JoinHandle<AppServerOutput>,
    monitor_task: &mut JoinHandle<MonitorOutput>,
    automation_task: &mut JoinHandle<AutomationOutput>,
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
    if completed_task != Some(CompletedRuntimeTask::Automation) {
        abort_and_join(automation_task).await;
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
