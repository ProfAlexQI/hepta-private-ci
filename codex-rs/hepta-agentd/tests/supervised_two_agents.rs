#![cfg(unix)]

use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use codex_app_server_client::RemoteAppServerClient;
use codex_app_server_client::RemoteAppServerConnectArgs;
use codex_app_server_client::RemoteAppServerEndpoint;
use codex_hepta_agentd::AgentdClient;
use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::AgentManifest;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::ResourceBudget;
use codex_hepta_fleet::WorkspaceBinding;
use codex_hepta_paths::HeptaFleetRoot;
use codex_hepta_supervisor::AgentCommand;
use codex_hepta_supervisor::Supervisor;
use codex_hepta_supervisor::SupervisorConfig;
use codex_hepta_supervisor::UnixProcessDriver;
use codex_utils_absolute_path::AbsolutePathBuf;

const AGENT_A: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const AGENT_B: &str = "019153a4-3088-7e03-a56a-9b1964f75dd3";

struct FleetHarness {
    supervisor: Supervisor<UnixProcessDriver>,
    registry: FleetRegistry,
    agent_ids: Vec<AgentId>,
}

impl Drop for FleetHarness {
    fn drop(&mut self) {
        for agent_id in &self.agent_ids {
            let _ = self.supervisor.kill(agent_id);
        }
        let deadline = Instant::now() + Duration::from_secs(3);
        while Instant::now() < deadline {
            self.supervisor.tick(Instant::now());
            if self.agent_ids.iter().all(|agent_id| {
                self.supervisor
                    .snapshot(agent_id)
                    .is_none_or(|snapshot| !snapshot.active)
            }) {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_supervised_real_agentd_processes_are_fault_isolated() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().canonicalize()?;
    let fleet_path = root.join("fleet");
    let fleet_root = HeptaFleetRoot::parse(fleet_path)?;
    let registry = FleetRegistry::initialize(fleet_root.clone())?;
    let workspace_a = create_workspace(&root, "workspace-a")?;
    let workspace_b = create_workspace(&root, "workspace-b")?;
    let agent_a = AgentId::parse(AGENT_A).map_err(anyhow::Error::msg)?;
    let agent_b = AgentId::parse(AGENT_B).map_err(anyhow::Error::msg)?;
    register(&registry, &fleet_root, agent_a.clone(), &workspace_a)?;
    register(&registry, &fleet_root, agent_b.clone(), &workspace_b)?;

    let mut config = SupervisorConfig::local_default();
    config.health_timeout = Duration::from_secs(20);
    config.drain_timeout = Duration::from_secs(2);
    config.stop_grace = Duration::from_secs(1);
    let driver = UnixProcessDriver::new(128)?;
    let (supervisor, recovery) =
        Supervisor::recover(registry.clone(), driver, config, Instant::now())?;
    if !recovery.faults.is_empty() {
        bail!("unexpected recovery faults: {:?}", recovery.faults);
    }
    let mut harness = FleetHarness {
        supervisor,
        registry,
        agent_ids: vec![agent_a.clone(), agent_b.clone()],
    };
    let binary = PathBuf::from(env!("CARGO_BIN_EXE_codex-hepta-agentd"));
    let command = AgentCommand::new(binary, Vec::new())?;
    harness
        .supervisor
        .start(&agent_a, command.clone(), Instant::now())?;
    harness
        .supervisor
        .start(&agent_b, command, Instant::now())?;

    let layout = fleet_root.layout();
    let layout_a = layout.agent(&agent_a);
    let layout_b = layout.agent(&agent_b);
    let client_a = AgentdClient::new(
        layout_a.agentd_control_socket().to_path_buf(),
        agent_a.clone(),
        1,
    )?;
    let client_b = AgentdClient::new(
        layout_b.agentd_control_socket().to_path_buf(),
        agent_b.clone(),
        1,
    )?;
    let (health_a, health_b) =
        wait_for_both_ready(&mut harness, (&agent_a, &client_a), (&agent_b, &client_b)).await?;
    assert_ne!(health_a.process_id, health_b.process_id);
    assert_eq!(health_a.workspace, workspace_a);
    assert_eq!(health_b.workspace, workspace_b);
    assert_eq!(health_a.home_root, layout_a.home_root());
    assert_eq!(health_b.home_root, layout_b.home_root());
    assert_ne!(health_a.run_root, health_b.run_root);

    let ingress_a = client_a.session_ingress().await?;
    let ingress_b = client_b.session_ingress().await?;
    assert_eq!(ingress_a.socket_path, layout_a.app_server_socket());
    assert_eq!(ingress_b.socket_path, layout_b.app_server_socket());
    assert_ne!(ingress_a.socket_path, ingress_b.socket_path);
    assert_eq!(
        initialized_codex_home(&ingress_a.socket_path).await?,
        layout_a.home_root().to_string_lossy()
    );
    assert_eq!(
        initialized_codex_home(&ingress_b.socket_path).await?,
        layout_b.home_root().to_string_lossy()
    );

    let b_process_before = health_b.process_id;
    harness.supervisor.kill(&agent_a)?;
    wait_for_stopped(&mut harness, &agent_a).await?;
    let b_during_a_failure = client_b.health().await?;
    assert!(b_during_a_failure.ready);
    assert_eq!(b_during_a_failure.process_id, b_process_before);
    assert_eq!(
        initialized_codex_home(&ingress_b.socket_path).await?,
        layout_b.home_root().to_string_lossy()
    );

    harness.supervisor.restart(&agent_a, Instant::now())?;
    let restarted_generation = harness
        .registry
        .load()?
        .agent(&agent_a)
        .context("agent A missing after restart")?
        .lifecycle
        .generation;
    assert_eq!(restarted_generation, 5);
    let restarted_a = AgentdClient::new(
        layout_a.agentd_control_socket().to_path_buf(),
        agent_a.clone(),
        restarted_generation,
    )?;
    let (restarted_health_a, still_healthy_b) = wait_for_both_ready(
        &mut harness,
        (&agent_a, &restarted_a),
        (&agent_b, &client_b),
    )
    .await?;
    assert_ne!(restarted_health_a.process_id, health_a.process_id);
    assert_eq!(still_healthy_b.process_id, b_process_before);
    assert_eq!(
        initialized_codex_home(layout_a.app_server_socket()).await?,
        layout_a.home_root().to_string_lossy()
    );

    let events = restarted_a.events(0, 256).await?;
    assert!(!events.events.is_empty());
    let resumed = restarted_a.events(events.next_cursor, 256).await?;
    assert!(!resumed.gap);
    assert!(resumed.events.is_empty());

    assert_eq!(
        harness
            .registry
            .load()?
            .agent(&agent_b)
            .context("agent B missing")?
            .lifecycle
            .lifecycle,
        AgentLifecycle::Running
    );
    Ok(())
}

fn create_workspace(root: &Path, name: &str) -> Result<PathBuf> {
    let workspace = root.join(name);
    std::fs::create_dir(&workspace)?;
    Ok(workspace.canonicalize()?)
}

fn register(
    registry: &FleetRegistry,
    fleet_root: &HeptaFleetRoot,
    agent_id: AgentId,
    workspace: &Path,
) -> Result<()> {
    let binding = WorkspaceBinding::new(workspace, fleet_root)?;
    let manifest = AgentManifest::new(agent_id, binding, ResourceBudget::local_default())?;
    registry.register(manifest)?;
    Ok(())
}

async fn wait_for_both_ready(
    harness: &mut FleetHarness,
    first: (&AgentId, &AgentdClient),
    second: (&AgentId, &AgentdClient),
) -> Result<(
    codex_hepta_agentd::HealthSnapshot,
    codex_hepta_agentd::HealthSnapshot,
)> {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let report = harness.supervisor.tick(Instant::now());
        if !report.faults.is_empty() {
            bail!(
                "supervisor faults while waiting for readiness: {:?}",
                report.faults
            );
        }
        if let (Ok(first_health), Ok(second_health)) =
            (first.1.health().await, second.1.health().await)
            && first_health.ready
            && second_health.ready
        {
            return Ok((first_health, second_health));
        }
        if Instant::now() >= deadline {
            let first_snapshot = harness
                .supervisor
                .snapshot(first.0)
                .map(render_supervisor_snapshot);
            let second_snapshot = harness
                .supervisor
                .snapshot(second.0)
                .map(render_supervisor_snapshot);
            bail!(
                "timed out waiting for two independent agentd processes; first={first_snapshot:?}; second={second_snapshot:?}; registry={:?}",
                harness.registry.load()?
            );
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

fn render_supervisor_snapshot(snapshot: codex_hepta_supervisor::AgentSupervisorSnapshot) -> String {
    let logs = snapshot
        .logs
        .iter()
        .map(|log| {
            format!(
                "{:?}:{}",
                log.stream,
                String::from_utf8_lossy(&log.bytes).trim_end()
            )
        })
        .collect::<Vec<_>>()
        .join(" | ");
    format!(
        "active={} generation={:?} events={:?} logs={logs}",
        snapshot.active, snapshot.runtime_generation, snapshot.events
    )
}

async fn wait_for_stopped(harness: &mut FleetHarness, agent_id: &AgentId) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let report = harness.supervisor.tick(Instant::now());
        if !report.faults.is_empty() {
            bail!(
                "supervisor faults while stopping agent: {:?}",
                report.faults
            );
        }
        let inactive = harness
            .supervisor
            .snapshot(agent_id)
            .is_some_and(|snapshot| !snapshot.active);
        let stopped = harness
            .registry
            .load()?
            .agent(agent_id)
            .is_some_and(|record| record.lifecycle.lifecycle == AgentLifecycle::Stopped);
        if inactive && stopped {
            return Ok(());
        }
        if Instant::now() >= deadline {
            bail!("timed out waiting for agent {agent_id} to stop");
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

async fn initialized_codex_home(socket_path: &Path) -> Result<String> {
    let socket_path = AbsolutePathBuf::from_absolute_path(socket_path)?;
    let client = RemoteAppServerClient::connect(RemoteAppServerConnectArgs {
        endpoint: RemoteAppServerEndpoint::UnixSocket { socket_path },
        client_name: "hepta-agentd-e2e".to_string(),
        client_version: env!("CARGO_PKG_VERSION").to_string(),
        experimental_api: false,
        mcp_server_openai_form_elicitation: false,
        opt_out_notification_methods: Vec::new(),
        channel_capacity: 8,
    })
    .await?;
    let home = client
        .codex_home()
        .context("initialize response omitted Codex home")?
        .to_string();
    client.shutdown().await?;
    Ok(home)
}
