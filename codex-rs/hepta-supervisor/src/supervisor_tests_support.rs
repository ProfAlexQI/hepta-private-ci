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
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::AgentManifest;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::ReleaseId;
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
use crate::MatrixAdoptSpec;
use crate::MatrixSpawnSpec;
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

    fn program(&self, identity: &str) -> Result<PathBuf, SupervisorError> {
        let root = self._temp.path().join("programs");
        std::fs::create_dir_all(&root)?;
        let path = root.join(format!("{identity}.bin"));
        if !path.exists() {
            std::fs::write(&path, b"hepta supervisor test program\n")?;
        }
        Ok(path)
    }

    fn command(&self) -> Result<AgentCommand, SupervisorError> {
        AgentCommand::new(self.program("unversioned")?, Vec::new())
    }

    fn release(&self, identity: &str) -> Result<AgentRelease, SupervisorError> {
        AgentRelease::new(
            identity,
            AgentCommand::new(self.program(identity)?, Vec::new())?,
        )
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
    role: FakeRole,
    identity: ProcessIdentity,
    healthy: bool,
    drained: bool,
    exit: Option<ProcessExit>,
    logs: VecDeque<ProcessLog>,
    drain_requests: usize,
    stop_requests: usize,
    kill_requests: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FakeRole {
    Agentd,
    Matrixd,
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
        self.update_role(agent_id, FakeRole::Agentd, update);
    }

    fn update_role(&self, agent_id: &AgentId, role: FakeRole, update: impl FnOnce(&mut FakeState)) {
        let mut world = self.world.lock().expect("fake world lock");
        let state = world
            .processes
            .values_mut()
            .rev()
            .find(|state| &state.agent_id == agent_id && state.role == role)
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

    fn set_matrix_healthy(&self, agent_id: &AgentId) {
        self.update_role(agent_id, FakeRole::Matrixd, |state| state.healthy = true);
    }

    fn set_matrix_unhealthy(&self, agent_id: &AgentId) {
        self.update_role(agent_id, FakeRole::Matrixd, |state| state.healthy = false);
    }

    fn set_matrix_exit(&self, agent_id: &AgentId) {
        self.update_role(agent_id, FakeRole::Matrixd, |state| {
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
        self.counts_role(agent_id, FakeRole::Agentd)
    }

    fn matrix_counts(&self, agent_id: &AgentId) -> (usize, usize, usize) {
        self.counts_role(agent_id, FakeRole::Matrixd)
    }

    fn counts_role(&self, agent_id: &AgentId, role: FakeRole) -> (usize, usize, usize) {
        let world = self.world.lock().expect("fake world lock");
        let state = world
            .processes
            .values()
            .rev()
            .find(|state| &state.agent_id == agent_id && state.role == role)
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
            .filter(|state| &state.agent_id == agent_id && state.role == FakeRole::Agentd)
            .count()
    }

    fn matrix_spawn_count(&self, agent_id: &AgentId) -> usize {
        self.world
            .lock()
            .expect("fake world lock")
            .processes
            .values()
            .filter(|state| &state.agent_id == agent_id && state.role == FakeRole::Matrixd)
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
                role: FakeRole::Agentd,
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
                && state.role == FakeRole::Agentd
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

    fn spawn_matrixd(
        &mut self,
        spec: &MatrixSpawnSpec,
    ) -> Result<SpawnedProcess<Self::Process>, ProcessDriverError> {
        let mut world = self.world.lock().expect("fake world lock");
        if world.reject_spawn_programs.contains(&spec.command.program) {
            return Err(ProcessDriverError::new("injected matrix spawn failure"));
        }
        world.next_id += 1;
        let id = world.next_id;
        let identity =
            ProcessIdentity::new(id, format!("fake-matrix-{id}-{}", spec.agent_generation))
                .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        world.processes.insert(
            id,
            FakeState {
                agent_id: spec.agent_id.clone(),
                role: FakeRole::Matrixd,
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

    fn adopt_matrixd(
        &mut self,
        spec: &MatrixAdoptSpec,
    ) -> Result<Adoption<Self::Process>, ProcessDriverError> {
        let world = self.world.lock().expect("fake world lock");
        let Some((&id, _)) = world.processes.iter().find(|(_, state)| {
            state.agent_id == spec.agent_id
                && state.role == FakeRole::Matrixd
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

