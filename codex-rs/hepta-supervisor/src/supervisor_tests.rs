use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::AgentManifest;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::ResourceBudget;
use codex_hepta_fleet::WorkspaceBinding;
use codex_hepta_paths::HeptaFleetRoot;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::Supervisor;
use crate::AdoptSpec;
use crate::Adoption;
use crate::AgentCommand;
use crate::AgentRelease;
use crate::ManagedProcess;
use crate::ProcessDriver;
use crate::ProcessDriverError;
use crate::ProcessExit;
use crate::ProcessIdentity;
use crate::ProcessLog;
use crate::ProcessObservation;
use crate::ProcessState;
use crate::ProcessStream;
use crate::SpawnSpec;
use crate::SupervisorConfig;
use crate::SupervisorError;
use crate::SupervisorEventKind;
use crate::TickReport;
use crate::driver::SpawnedProcess;

const FIRST_AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const SECOND_AGENT_ID: &str = "019153a4-3088-7e03-a56a-9b1964f75dd3";

struct TestFleet {
    _temp: TempDir,
    registry: FleetRegistry,
    first: AgentId,
    second: AgentId,
}

impl TestFleet {
    fn new() -> Result<Self, SupervisorError> {
        let temp = tempfile::tempdir()?;
        let root = HeptaFleetRoot::parse(temp.path().join("fleet"))
            .map_err(|error| SupervisorError::Invalid(error.to_string()))?;
        let registry = FleetRegistry::initialize(root.clone())?;
        let first = register_agent(&registry, &root, temp.path(), FIRST_AGENT_ID, "workspace-a")?;
        let second = register_agent(
            &registry,
            &root,
            temp.path(),
            SECOND_AGENT_ID,
            "workspace-b",
        )?;
        Ok(Self {
            _temp: temp,
            registry,
            first,
            second,
        })
    }
}

fn register_agent(
    registry: &FleetRegistry,
    root: &HeptaFleetRoot,
    parent: &Path,
    id: &str,
    workspace_name: &str,
) -> Result<AgentId, SupervisorError> {
    let workspace = parent.join(workspace_name);
    std::fs::create_dir(&workspace)?;
    let agent_id =
        AgentId::parse(id).map_err(|error| SupervisorError::Invalid(error.to_string()))?;
    registry.register(AgentManifest::new(
        agent_id.clone(),
        WorkspaceBinding::new(workspace.canonicalize()?, root)?,
        ResourceBudget::local_default(),
    )?)?;
    Ok(agent_id)
}

fn command() -> Result<AgentCommand, SupervisorError> {
    AgentCommand::new("/fake/hepta-agentd", Vec::new())
}

fn release(identity: &str, program: &str) -> Result<AgentRelease, SupervisorError> {
    AgentRelease::new(identity, AgentCommand::new(program, Vec::new())?)
}

fn config() -> SupervisorConfig {
    SupervisorConfig {
        health_timeout: Duration::from_millis(10),
        drain_timeout: Duration::from_millis(10),
        stop_grace: Duration::from_millis(10),
        event_capacity: 8,
        log_capacity: 3,
        max_log_bytes: 8,
        driver_poll_batch: 16,
    }
}

#[derive(Clone, Default)]
struct FakeControl {
    world: Arc<Mutex<FakeWorld>>,
}

struct FakeDriver {
    world: Arc<Mutex<FakeWorld>>,
}

#[derive(Default)]
struct FakeWorld {
    next_id: u64,
    processes: BTreeMap<u64, FakeState>,
    reject_adoption: BTreeSet<AgentId>,
    reject_spawn_programs: BTreeSet<PathBuf>,
}

struct FakeState {
    agent_id: AgentId,
    identity: ProcessIdentity,
    healthy: bool,
    drained: bool,
    exit: Option<ProcessExit>,
    logs: VecDeque<ProcessLog>,
    drain_requests: usize,
    stop_requests: usize,
    kill_requests: usize,
}

struct FakeProcess {
    id: u64,
    world: Arc<Mutex<FakeWorld>>,
}

impl FakeControl {
    fn driver(&self) -> FakeDriver {
        FakeDriver {
            world: self.world.clone(),
        }
    }

    fn update(&self, agent_id: &AgentId, update: impl FnOnce(&mut FakeState)) {
        let mut world = self.world.lock().expect("fake world lock");
        let state = world
            .processes
            .values_mut()
            .rev()
            .find(|state| &state.agent_id == agent_id)
            .expect("fake process");
        update(state);
    }

    fn set_healthy(&self, agent_id: &AgentId) {
        self.update(agent_id, |state| state.healthy = true);
    }

    fn set_drained(&self, agent_id: &AgentId) {
        self.update(agent_id, |state| state.drained = true);
    }

    fn set_exit(&self, agent_id: &AgentId) {
        self.update(agent_id, |state| {
            state.exit = Some(ProcessExit {
                success: true,
                code: Some(0),
            });
        });
    }

    fn push_logs(&self, agent_id: &AgentId, count: usize) {
        self.update(agent_id, |state| {
            for index in 0..count {
                state.logs.push_back(ProcessLog {
                    stream: ProcessStream::Stdout,
                    bytes: format!("log-{index}-oversized").into_bytes(),
                });
            }
        });
    }

    fn reject_adoption(&self, agent_id: AgentId) {
        self.world
            .lock()
            .expect("fake world lock")
            .reject_adoption
            .insert(agent_id);
    }

    fn reject_spawn_program(&self, program: impl Into<PathBuf>) {
        self.world
            .lock()
            .expect("fake world lock")
            .reject_spawn_programs
            .insert(program.into());
    }

    fn counts(&self, agent_id: &AgentId) -> (usize, usize, usize) {
        let world = self.world.lock().expect("fake world lock");
        let state = world
            .processes
            .values()
            .rev()
            .find(|state| &state.agent_id == agent_id)
            .expect("fake process");
        (
            state.drain_requests,
            state.stop_requests,
            state.kill_requests,
        )
    }

    fn spawn_count(&self, agent_id: &AgentId) -> usize {
        self.world
            .lock()
            .expect("fake world lock")
            .processes
            .values()
            .filter(|state| &state.agent_id == agent_id)
            .count()
    }
}

impl ProcessDriver for FakeDriver {
    type Process = FakeProcess;

    fn spawn(
        &mut self,
        spec: &SpawnSpec,
    ) -> Result<SpawnedProcess<Self::Process>, ProcessDriverError> {
        let mut world = self.world.lock().expect("fake world lock");
        if world.reject_spawn_programs.contains(&spec.command.program) {
            return Err(ProcessDriverError::new("injected spawn failure"));
        }
        world.next_id += 1;
        let id = world.next_id;
        let identity = ProcessIdentity::new(id, format!("fake-{id}-{}", spec.generation))
            .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        world.processes.insert(
            id,
            FakeState {
                agent_id: spec.agent_id.clone(),
                identity: identity.clone(),
                healthy: false,
                drained: false,
                exit: None,
                logs: VecDeque::new(),
                drain_requests: 0,
                stop_requests: 0,
                kill_requests: 0,
            },
        );
        Ok(SpawnedProcess {
            identity,
            process: FakeProcess {
                id,
                world: self.world.clone(),
            },
        })
    }

    fn adopt(&mut self, spec: &AdoptSpec) -> Result<Adoption<Self::Process>, ProcessDriverError> {
        let world = self.world.lock().expect("fake world lock");
        if world.reject_adoption.contains(&spec.agent_id) {
            return Ok(Adoption::Rejected);
        }
        let Some((&id, _)) = world.processes.iter().find(|(_, state)| {
            state.agent_id == spec.agent_id
                && state.identity == spec.identity
                && state.exit.is_none()
        }) else {
            return Ok(Adoption::Missing);
        };
        Ok(Adoption::Adopted(FakeProcess {
            id,
            world: self.world.clone(),
        }))
    }
}

impl ManagedProcess for FakeProcess {
    fn poll(&mut self, max_logs: usize) -> Result<ProcessObservation, ProcessDriverError> {
        let mut world = self.world.lock().expect("fake world lock");
        let state = world.processes.get_mut(&self.id).expect("fake process");
        let logs = (0..max_logs)
            .filter_map(|_| state.logs.pop_front())
            .collect();
        let process_state = state.exit.map_or(
            ProcessState::Running {
                healthy: state.healthy,
                drained: state.drained,
            },
            ProcessState::Exited,
        );
        Ok(ProcessObservation {
            state: process_state,
            logs,
        })
    }

    fn request_drain(&mut self) -> Result<(), ProcessDriverError> {
        self.world
            .lock()
            .expect("fake world lock")
            .processes
            .get_mut(&self.id)
            .expect("fake process")
            .drain_requests += 1;
        Ok(())
    }

    fn request_stop(&mut self) -> Result<(), ProcessDriverError> {
        self.world
            .lock()
            .expect("fake world lock")
            .processes
            .get_mut(&self.id)
            .expect("fake process")
            .stop_requests += 1;
        Ok(())
    }

    fn kill(&mut self) -> Result<(), ProcessDriverError> {
        self.world
            .lock()
            .expect("fake world lock")
            .processes
            .get_mut(&self.id)
            .expect("fake process")
            .kill_requests += 1;
        Ok(())
    }
}

#[test]
fn hung_agent_is_stopped_and_killed_without_blocking_peer() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, recovered) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    assert_eq!(recovered, TickReport::default());
    supervisor.start(&fleet.first, command()?, now)?;
    supervisor.start(&fleet.second, command()?, now)?;
    control.set_healthy(&fleet.second);
    control.push_logs(&fleet.first, 10);
    control.push_logs(&fleet.second, 10);

    assert_eq!(supervisor.tick(now), TickReport::default());
    assert_eq!(
        fleet
            .registry
            .load()?
            .agent(&fleet.second)
            .unwrap()
            .lifecycle
            .lifecycle,
        AgentLifecycle::Running
    );
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(11)),
        TickReport::default()
    );
    assert_eq!(control.counts(&fleet.first), (0, 1, 0));
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(22)),
        TickReport::default()
    );
    assert_eq!(control.counts(&fleet.first), (0, 1, 1));
    assert_eq!(
        fleet
            .registry
            .load()?
            .agent(&fleet.second)
            .unwrap()
            .lifecycle
            .lifecycle,
        AgentLifecycle::Running
    );
    control.set_exit(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(23)),
        TickReport::default()
    );

    let first = supervisor.snapshot(&fleet.first).expect("first slot");
    let second = supervisor.snapshot(&fleet.second).expect("second slot");
    assert!(!first.active);
    assert!(second.active);
    assert_eq!((first.logs.len(), second.logs.len()), (3, 3));
    assert!(first.events.len() <= 8);
    assert!(second.events.len() <= 8);
    assert!(first.logs.iter().all(|log| log.bytes.len() <= 8));
    assert!(second.logs.iter().all(|log| log.bytes.len() <= 8));
    Ok(())
}

#[test]
fn restart_drains_one_agent_and_spawns_a_new_generation() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    supervisor.start(&fleet.first, command()?, now)?;
    control.set_healthy(&fleet.first);
    supervisor.tick(now);
    supervisor.restart(&fleet.first, now)?;
    control.set_drained(&fleet.first);
    supervisor.tick(now);
    assert_eq!(control.counts(&fleet.first), (1, 1, 0));
    control.set_exit(&fleet.first);
    supervisor.tick(now);

    assert_eq!(control.spawn_count(&fleet.first), 2);
    assert_eq!(
        fleet
            .registry
            .load()?
            .agent(&fleet.first)
            .unwrap()
            .lifecycle
            .lifecycle,
        AgentLifecycle::Starting
    );
    Ok(())
}

#[test]
fn recovery_adopts_one_orphan_and_rejects_another() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut first_supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    first_supervisor.start(&fleet.first, command()?, now)?;
    first_supervisor.start(&fleet.second, command()?, now)?;
    control.set_healthy(&fleet.first);
    control.set_healthy(&fleet.second);
    first_supervisor.tick(now);
    drop(first_supervisor);
    control.reject_adoption(fleet.second.clone());

    let (supervisor, report) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    assert_eq!(report, TickReport::default());
    assert!(supervisor.snapshot(&fleet.first).unwrap().active);
    assert!(!supervisor.snapshot(&fleet.second).unwrap().active);
    assert!(
        supervisor
            .snapshot(&fleet.first)
            .unwrap()
            .events
            .iter()
            .any(|event| event.kind == SupervisorEventKind::OrphanAdopted)
    );
    assert!(
        supervisor
            .snapshot(&fleet.second)
            .unwrap()
            .events
            .iter()
            .any(|event| event.kind == SupervisorEventKind::OrphanRejected)
    );
    assert_eq!(
        fleet
            .registry
            .load()?
            .agent(&fleet.second)
            .unwrap()
            .lifecycle
            .lifecycle,
        AgentLifecycle::Failed
    );
    Ok(())
}

#[test]
fn stale_runtime_is_fenced_without_touching_peer() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    supervisor.start(&fleet.first, command()?, now)?;
    supervisor.start(&fleet.second, command()?, now)?;
    control.set_healthy(&fleet.first);
    control.set_healthy(&fleet.second);
    supervisor.tick(now);
    let first = fleet
        .registry
        .load()?
        .agent(&fleet.first)
        .unwrap()
        .lifecycle
        .clone();
    fleet.registry.compare_and_transition(
        &fleet.first,
        first.generation,
        AgentLifecycle::Draining,
    )?;

    supervisor.tick(now);
    assert_eq!(control.counts(&fleet.first).2, 1);
    assert_eq!(control.counts(&fleet.second).2, 0);
    assert!(
        supervisor
            .snapshot(&fleet.first)
            .unwrap()
            .events
            .iter()
            .any(|event| matches!(event.kind, SupervisorEventKind::GenerationFenced { .. }))
    );
    assert_eq!(
        fleet
            .registry
            .load()?
            .agent(&fleet.second)
            .unwrap()
            .lifecycle
            .lifecycle,
        AgentLifecycle::Running
    );
    Ok(())
}

#[test]
fn successful_upgrade_and_explicit_rollback_change_only_target_agent() -> Result<(), SupervisorError>
{
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    supervisor.start_release(
        &fleet.first,
        release("release-v1", "/fake/release-v1/hepta-agentd")?,
        now,
    )?;
    supervisor.start_release(
        &fleet.second,
        release("peer-release", "/fake/peer/hepta-agentd")?,
        now,
    )?;
    control.set_healthy(&fleet.first);
    control.set_healthy(&fleet.second);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let peer_before = supervisor.snapshot(&fleet.second).expect("peer snapshot");

    supervisor.upgrade(
        &fleet.first,
        release("release-v2", "/fake/release-v2/hepta-agentd")?,
        now,
    )?;
    assert!(matches!(
        supervisor.restart(&fleet.first, now),
        Err(SupervisorError::ReleaseChangePending(agent_id)) if agent_id == fleet.first
    ));
    finish_release_drain(&mut supervisor, &control, &fleet.first, now);
    control.set_healthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let upgraded = supervisor
        .snapshot(&fleet.first)
        .expect("upgraded snapshot");
    assert_eq!(upgraded.active_release.as_deref(), Some("release-v2"));
    assert_eq!(upgraded.previous_release.as_deref(), Some("release-v1"));
    assert!(!upgraded.release_change_pending);
    assert!(upgraded.events.iter().any(|event| {
        matches!(
            &event.kind,
            SupervisorEventKind::UpgradeCommitted { previous, target }
                if previous == "release-v1" && target == "release-v2"
        )
    }));

    supervisor.rollback(&fleet.first, now)?;
    finish_release_drain(&mut supervisor, &control, &fleet.first, now);
    control.set_healthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let rolled_back = supervisor
        .snapshot(&fleet.first)
        .expect("rollback snapshot");
    assert_eq!(rolled_back.active_release.as_deref(), Some("release-v1"));
    assert_eq!(rolled_back.previous_release.as_deref(), Some("release-v2"));
    assert!(!rolled_back.release_change_pending);
    assert!(rolled_back.events.iter().any(|event| {
        matches!(
            &event.kind,
            SupervisorEventKind::ExplicitRollbackCommitted { previous, target }
                if previous == "release-v2" && target == "release-v1"
        )
    }));

    let peer_after = supervisor.snapshot(&fleet.second).expect("peer snapshot");
    assert_eq!(peer_after.process_system_id, peer_before.process_system_id);
    assert_eq!(peer_after.spawn_generation, peer_before.spawn_generation);
    assert_eq!(control.counts(&fleet.second), (0, 0, 0));
    Ok(())
}

#[test]
fn failed_spawn_and_failed_health_each_auto_rollback_once() -> Result<(), SupervisorError> {
    let fleet = TestFleet::new()?;
    let control = FakeControl::default();
    let now = Instant::now();
    let (mut supervisor, _) =
        Supervisor::recover(fleet.registry.clone(), control.driver(), config(), now)?;
    supervisor.start_release(
        &fleet.first,
        release("release-v1", "/fake/release-v1/hepta-agentd")?,
        now,
    )?;
    control.set_healthy(&fleet.first);
    supervisor.tick(now);

    control.reject_spawn_program("/fake/release-spawn-fails/hepta-agentd");
    supervisor.upgrade(
        &fleet.first,
        release(
            "release-spawn-fails",
            "/fake/release-spawn-fails/hepta-agentd",
        )?,
        now,
    )?;
    finish_release_drain(&mut supervisor, &control, &fleet.first, now);
    control.set_healthy(&fleet.first);
    assert_eq!(supervisor.tick(now), TickReport::default());
    let recovered = supervisor
        .snapshot(&fleet.first)
        .expect("recovered snapshot");
    assert_eq!(recovered.active_release.as_deref(), Some("release-v1"));
    assert!(!recovered.release_change_pending);

    supervisor.upgrade(
        &fleet.first,
        release(
            "release-health-fails",
            "/fake/release-health-fails/hepta-agentd",
        )?,
        now,
    )?;
    finish_release_drain(&mut supervisor, &control, &fleet.first, now);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(11)),
        TickReport::default()
    );
    control.set_exit(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(12)),
        TickReport::default()
    );
    control.set_healthy(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_millis(12)),
        TickReport::default()
    );
    let final_snapshot = supervisor.snapshot(&fleet.first).expect("final snapshot");
    assert_eq!(final_snapshot.active_release.as_deref(), Some("release-v1"));
    assert!(!final_snapshot.release_change_pending);
    assert!(final_snapshot.events.iter().any(|event| {
        matches!(
            &event.kind,
            SupervisorEventKind::AutomaticRollbackCommitted { failed, restored }
                if failed == "release-health-fails" && restored == "release-v1"
        )
    }));
    let spawn_count = control.spawn_count(&fleet.first);
    assert_eq!(
        supervisor.tick(now + Duration::from_secs(10)),
        TickReport::default()
    );
    assert_eq!(control.spawn_count(&fleet.first), spawn_count);
    Ok(())
}

fn finish_release_drain(
    supervisor: &mut Supervisor<FakeDriver>,
    control: &FakeControl,
    agent_id: &AgentId,
    now: Instant,
) {
    control.set_drained(agent_id);
    assert_eq!(supervisor.tick(now), TickReport::default());
    control.set_exit(agent_id);
    assert_eq!(supervisor.tick(now), TickReport::default());
}
