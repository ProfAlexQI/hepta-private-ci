#![cfg(unix)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;
use app_test_support::MockResponsesConfig;
use codex_hepta_agentd::AgentdClient;
use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::AgentManifest;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::ReleaseId;
use codex_hepta_fleet::ResourceBudget;
use codex_hepta_fleet::WorkspaceBinding;
use codex_hepta_paths::HeptaAgentLayout;
use codex_hepta_paths::HeptaFleetRoot;
use codex_hepta_supervisor::SupervisordAgentStatus;
use codex_hepta_supervisor::SupervisordClient;
use core_test_support::responses;

const CHILD_MODE_ENV: &str = "HEPTA_TEST_SUPERVISORD_CHILD";
const CHILD_FLEET_ROOT_ENV: &str = "HEPTA_TEST_SUPERVISORD_FLEET_ROOT";
const READY_TIMEOUT: Duration = Duration::from_secs(120);
const DAEMON_TIMEOUT: Duration = Duration::from_secs(15);
const AGENT_IDS: [&str; 5] = [
    "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12",
    "019153a4-3088-7e03-a56a-9b1964f75dd3",
    "019153a4-3088-7e03-a56a-9b1964f75dd4",
    "019153a4-3088-7e03-a56a-9b1964f75dd5",
    "019153a4-3088-7e03-a56a-9b1964f75dd6",
];

/// The parent test spawns this integration-test executable as an isolated
/// process so the exact product daemon implementation can itself own and
/// survive independently from the test coordinator.
#[test]
fn supervisord_child_entry() -> Result<()> {
    if std::env::var_os(CHILD_MODE_ENV).is_none() {
        return Ok(());
    }
    let fleet_root = std::env::var_os(CHILD_FLEET_ROOT_ENV)
        .map(PathBuf::from)
        .context("child fleet root is missing")?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    runtime.block_on(codex_hepta_supervisor::run_supervisord(
        HeptaFleetRoot::parse(fleet_root)?,
        tokio_util::sync::CancellationToken::new(),
    ))?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn five_real_agents_survive_daemon_restart_and_isolate_one_agent_release_changes()
-> Result<()> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().canonicalize()?;
    let fleet_root = HeptaFleetRoot::parse(root.join("fleet"))?;
    let registry = FleetRegistry::initialize(fleet_root.clone())?;
    let agents = register_agents(&registry, &fleet_root, &root)?;
    assert_distinct_roots(&agents)?;

    let model = responses::start_mock_server().await;
    for agent in &agents {
        MockResponsesConfig::new(&model.uri()).write(agent.layout.home_root())?;
    }

    let sources = root.join("release-sources");
    std::fs::create_dir(&sources)?;
    let agentd = PathBuf::from(env!("CARGO_BIN_EXE_codex-hepta-agentd"));
    let source_v1 = agentd_wrapper(&sources, "agentd-v1", &agentd)?;
    let source_v2 = agentd_wrapper(&sources, "agentd-v2", &agentd)?;
    let release_v1 = ReleaseId::parse("agentd-v1")?;
    let release_v2 = ReleaseId::parse("agentd-v2")?;
    registry.install_release(release_v1.clone(), &source_v1, Vec::new())?;
    registry.install_release(release_v2.clone(), &source_v2, Vec::new())?;
    for agent in &agents {
        registry.allow_release(&agent.agent_id, &release_v1)?;
    }
    registry.allow_release(&agents[0].agent_id, &release_v2)?;

    let log_path = root.join("supervisord-child.log");
    let mut process_guard = AgentProcessGuard::default();
    let mut daemon = DaemonChild::spawn(&fleet_root, &log_path)?;
    let client = wait_for_daemon(&registry, &mut daemon, &log_path).await?;

    for agent in &agents[..2] {
        client
            .start(agent.agent_id.clone(), release_v1.clone())
            .await?;
        wait_for_release(&client, agent, &release_v1, None, &mut process_guard).await?;
    }
    let dual = client.roster(16).await?;
    ensure!(
        dual.iter().filter(|status| status.healthy).count() == 2,
        "the initial two independent agents did not become healthy: {dual:?}"
    );

    let peer_b_before_rejected_upgrade = client.snapshot(agents[1].agent_id.clone()).await?;
    ensure!(
        client
            .upgrade(agents[1].agent_id.clone(), release_v2.clone())
            .await
            .is_err(),
        "Agent B used a release allowed only for Agent A"
    );
    assert_exact_status(
        &peer_b_before_rejected_upgrade,
        &client.snapshot(agents[1].agent_id.clone()).await?,
        "Agent B changed after a rejected release",
    )?;

    for agent in &agents[2..] {
        client
            .start(agent.agent_id.clone(), release_v1.clone())
            .await?;
        wait_for_release(&client, agent, &release_v1, None, &mut process_guard).await?;
    }
    let five = client.roster(16).await?;
    ensure!(
        five.len() == 5 && five.iter().all(|status| status.healthy),
        "the five-Agent fleet did not become independently healthy: {five:?}"
    );

    let peer_baseline = peer_baseline(&client, &agents[1..]).await?;
    let initial_a = client.snapshot(agents[0].agent_id.clone()).await?;
    let initial_a_pid = require_pid(&initial_a)?;

    client.kill(agents[0].agent_id.clone()).await?;
    wait_for_stopped(&client, &agents[0], &mut process_guard).await?;
    assert_peers_unchanged(&client, &agents[1..], &peer_baseline).await?;

    client.restart(agents[0].agent_id.clone()).await?;
    let restarted_a =
        wait_for_release(&client, &agents[0], &release_v1, None, &mut process_guard).await?;
    ensure!(
        require_pid(&restarted_a)? != initial_a_pid,
        "Agent A restart reused its killed process"
    );
    assert_peers_unchanged(&client, &agents[1..], &peer_baseline).await?;

    client
        .upgrade(agents[0].agent_id.clone(), release_v2.clone())
        .await?;
    let upgraded_a = wait_for_release(
        &client,
        &agents[0],
        &release_v2,
        Some(&release_v1),
        &mut process_guard,
    )
    .await?;
    ensure!(
        require_pid(&upgraded_a)? != require_pid(&restarted_a)?,
        "Agent A upgrade did not replace its process"
    );
    assert_peers_unchanged(&client, &agents[1..], &peer_baseline).await?;

    let before_daemon_restart = snapshot_all(&client, &agents).await?;
    assert_agentd_endpoints(&agents, &before_daemon_restart).await?;
    daemon.kill()?;
    assert_agentd_endpoints(&agents, &before_daemon_restart).await?;

    let mut restarted_daemon = DaemonChild::spawn(&fleet_root, &log_path)?;
    let restarted_client = wait_for_daemon(&registry, &mut restarted_daemon, &log_path).await?;
    for (agent, before) in agents.iter().zip(&before_daemon_restart) {
        let expected_release = before
            .current_release
            .as_ref()
            .context("healthy agent has no current release")?;
        let adopted = wait_for_release(
            &restarted_client,
            agent,
            expected_release,
            before.previous_release.as_ref(),
            &mut process_guard,
        )
        .await?;
        assert_exact_status(before, &adopted, "daemon restart replaced an agent")?;
    }
    assert_agentd_endpoints(&agents, &before_daemon_restart).await?;

    restarted_client
        .rollback(agents[0].agent_id.clone())
        .await?;
    let rolled_back_a = wait_for_release(
        &restarted_client,
        &agents[0],
        &release_v1,
        Some(&release_v2),
        &mut process_guard,
    )
    .await?;
    ensure!(
        require_pid(&rolled_back_a)? != require_pid(&upgraded_a)?,
        "Agent A rollback did not replace its process"
    );
    assert_peers_unchanged(&restarted_client, &agents[1..], &peer_baseline).await?;

    for agent in &agents {
        restarted_client.kill(agent.agent_id.clone()).await?;
    }
    for agent in &agents {
        wait_for_stopped(&restarted_client, agent, &mut process_guard).await?;
    }
    restarted_daemon.kill()?;
    Ok(())
}

#[derive(Clone)]
struct AgentFixture {
    agent_id: AgentId,
    layout: HeptaAgentLayout,
}

fn register_agents(
    registry: &FleetRegistry,
    fleet_root: &HeptaFleetRoot,
    root: &Path,
) -> Result<Vec<AgentFixture>> {
    AGENT_IDS
        .iter()
        .enumerate()
        .map(|(index, raw_agent_id)| {
            let workspace = root.join(format!("workspace-{index}"));
            std::fs::create_dir(&workspace)?;
            let workspace = workspace.canonicalize()?;
            let agent_id = AgentId::parse(*raw_agent_id).map_err(anyhow::Error::msg)?;
            let record = registry.register(AgentManifest::new(
                agent_id.clone(),
                WorkspaceBinding::new(workspace, fleet_root)?,
                ResourceBudget::local_default(),
            )?)?;
            Ok(AgentFixture {
                agent_id,
                layout: record.layout,
            })
        })
        .collect()
}

fn assert_distinct_roots(agents: &[AgentFixture]) -> Result<()> {
    let homes = agents
        .iter()
        .map(|agent| agent.layout.home_root().to_path_buf())
        .collect::<BTreeSet<_>>();
    let controls = agents
        .iter()
        .map(|agent| agent.layout.agentd_control_socket().to_path_buf())
        .collect::<BTreeSet<_>>();
    let cognitive = agents
        .iter()
        .map(|agent| agent.layout.cognitive_root().to_path_buf())
        .collect::<BTreeSet<_>>();
    let automation = agents
        .iter()
        .map(|agent| agent.layout.automation_root().to_path_buf())
        .collect::<BTreeSet<_>>();
    ensure!(
        [
            homes.len(),
            controls.len(),
            cognitive.len(),
            automation.len()
        ] == [5; 4],
        "five Agent identities do not own independent path sets"
    );
    Ok(())
}

fn agentd_wrapper(root: &Path, name: &str, agentd: &Path) -> Result<PathBuf> {
    ensure!(
        !agentd.to_string_lossy().contains('\''),
        "fixture binary path cannot contain a shell quote"
    );
    let wrapper = root.join(name);
    std::fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\n# immutable test release: {name}\nexec '{}' \"$@\"\n",
            agentd.display()
        ),
    )?;
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o555))?;
    Ok(wrapper)
}

async fn wait_for_daemon(
    registry: &FleetRegistry,
    daemon: &mut DaemonChild,
    log_path: &Path,
) -> Result<SupervisordClient> {
    let deadline = Instant::now() + DAEMON_TIMEOUT;
    let client = SupervisordClient::new(registry.layout().supervisor_socket().to_path_buf())?;
    loop {
        if client.health().await.is_ok() {
            return Ok(client);
        }
        if let Some(status) = daemon.try_wait()? {
            let log = std::fs::read_to_string(log_path).unwrap_or_default();
            bail!("supervisord exited during startup with {status}; log={log}");
        }
        if Instant::now() >= deadline {
            let log = std::fs::read_to_string(log_path).unwrap_or_default();
            bail!("supervisord did not become ready; log={log}");
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

async fn wait_for_release(
    client: &SupervisordClient,
    agent: &AgentFixture,
    current: &ReleaseId,
    previous: Option<&ReleaseId>,
    process_guard: &mut AgentProcessGuard,
) -> Result<SupervisordAgentStatus> {
    let deadline = Instant::now() + READY_TIMEOUT;
    let mut last = None;
    loop {
        if let Ok(status) = client.snapshot(agent.agent_id.clone()).await {
            process_guard.observe(&status)?;
            if status.lifecycle == AgentLifecycle::Running
                && status.active
                && status.healthy
                && status.current_release.as_ref() == Some(current)
                && status.previous_release.as_ref() == previous
                && !status.release_change_pending
            {
                let agentd = AgentdClient::new(
                    agent.layout.agentd_control_socket().to_path_buf(),
                    agent.agent_id.clone(),
                    status
                        .spawn_generation
                        .context("healthy agent has no spawn generation")?,
                )?;
                let health = agentd.health().await?;
                ensure!(
                    health.ready && u64::from(health.process_id) == require_pid(&status)?,
                    "supervisord status did not match the real agentd endpoint"
                );
                return Ok(status);
            }
            last = Some(status);
        }
        if Instant::now() >= deadline {
            bail!(
                "timed out waiting for agent {} release {current}; last={last:?}",
                agent.agent_id
            );
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

async fn wait_for_stopped(
    client: &SupervisordClient,
    agent: &AgentFixture,
    process_guard: &mut AgentProcessGuard,
) -> Result<()> {
    let deadline = Instant::now() + READY_TIMEOUT;
    let mut last = None;
    loop {
        if let Ok(status) = client.snapshot(agent.agent_id.clone()).await {
            process_guard.observe(&status)?;
            if status.lifecycle == AgentLifecycle::Stopped
                && !status.active
                && status.process_id.is_none()
            {
                return Ok(());
            }
            last = Some(status);
        }
        if Instant::now() >= deadline {
            bail!(
                "timed out waiting for agent {} to stop; last={last:?}",
                agent.agent_id
            );
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

async fn peer_baseline(
    client: &SupervisordClient,
    peers: &[AgentFixture],
) -> Result<Vec<SupervisordAgentStatus>> {
    let mut baseline = Vec::new();
    for peer in peers {
        let status = client.snapshot(peer.agent_id.clone()).await?;
        ensure!(status.healthy, "peer {} is not healthy", peer.agent_id);
        baseline.push(status);
    }
    Ok(baseline)
}

async fn assert_peers_unchanged(
    client: &SupervisordClient,
    peers: &[AgentFixture],
    baseline: &[SupervisordAgentStatus],
) -> Result<()> {
    ensure!(
        peers.len() == baseline.len(),
        "peer baseline length drifted"
    );
    for (peer, expected) in peers.iter().zip(baseline) {
        let actual = client.snapshot(peer.agent_id.clone()).await?;
        assert_exact_status(expected, &actual, "an isolated peer changed")?;
        assert_agentd_endpoint(peer, expected).await?;
    }
    Ok(())
}

async fn snapshot_all(
    client: &SupervisordClient,
    agents: &[AgentFixture],
) -> Result<Vec<SupervisordAgentStatus>> {
    let mut statuses = Vec::new();
    for agent in agents {
        statuses.push(client.snapshot(agent.agent_id.clone()).await?);
    }
    Ok(statuses)
}

async fn assert_agentd_endpoints(
    agents: &[AgentFixture],
    statuses: &[SupervisordAgentStatus],
) -> Result<()> {
    ensure!(
        agents.len() == statuses.len(),
        "agent status length drifted"
    );
    for (agent, status) in agents.iter().zip(statuses) {
        assert_agentd_endpoint(agent, status).await?;
    }
    Ok(())
}

async fn assert_agentd_endpoint(
    agent: &AgentFixture,
    status: &SupervisordAgentStatus,
) -> Result<()> {
    let client = AgentdClient::new(
        agent.layout.agentd_control_socket().to_path_buf(),
        agent.agent_id.clone(),
        status
            .spawn_generation
            .context("running agent has no spawn generation")?,
    )?;
    let health = client.health().await?;
    ensure!(
        health.ready,
        "agent {} endpoint is not ready",
        agent.agent_id
    );
    ensure!(
        u64::from(health.process_id) == require_pid(status)?,
        "agent {} endpoint PID changed",
        agent.agent_id
    );
    Ok(())
}

fn assert_exact_status(
    expected: &SupervisordAgentStatus,
    actual: &SupervisordAgentStatus,
    message: &str,
) -> Result<()> {
    ensure!(
        actual == expected,
        "{message}: expected={expected:?} actual={actual:?}"
    );
    Ok(())
}

fn require_pid(status: &SupervisordAgentStatus) -> Result<u64> {
    status
        .process_id
        .context("active agent has no process identity")
}

struct DaemonChild(Child);

impl DaemonChild {
    fn spawn(fleet_root: &HeptaFleetRoot, log_path: &Path) -> Result<Self> {
        let log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;
        let child = Command::new(std::env::current_exe()?)
            .arg("--exact")
            .arg("supervisord_child_entry")
            .arg("--nocapture")
            .arg("--test-threads=1")
            .env(CHILD_MODE_ENV, "1")
            .env(CHILD_FLEET_ROOT_ENV, fleet_root.as_path())
            .stdin(Stdio::null())
            .stdout(Stdio::from(log.try_clone()?))
            .stderr(Stdio::from(log))
            .spawn()?;
        Ok(Self(child))
    }

    fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>> {
        Ok(self.0.try_wait()?)
    }

    fn kill(&mut self) -> Result<()> {
        if self.0.try_wait()?.is_none() {
            self.0.kill()?;
        }
        self.0.wait()?;
        Ok(())
    }
}

impl Drop for DaemonChild {
    fn drop(&mut self) {
        if self.0.try_wait().ok().flatten().is_none() {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

#[derive(Default)]
struct AgentProcessGuard {
    process_ids: BTreeMap<AgentId, u32>,
}

impl AgentProcessGuard {
    fn observe(&mut self, status: &SupervisordAgentStatus) -> Result<()> {
        if let Some(process_id) = status.process_id {
            self.process_ids.insert(
                status.agent_id.clone(),
                u32::try_from(process_id).context("agent PID does not fit pid_t")?,
            );
        } else {
            self.process_ids.remove(&status.agent_id);
        }
        Ok(())
    }
}

impl Drop for AgentProcessGuard {
    fn drop(&mut self) {
        for process_id in self.process_ids.values() {
            let _ = Command::new("/bin/kill")
                .arg("-KILL")
                .arg(process_id.to_string())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
}
