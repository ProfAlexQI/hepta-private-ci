use std::path::PathBuf;

use codex_hepta_contracts::AgentId;

use crate::AgentCommand;
use crate::ProcessDriverError;
use crate::ProcessExit;
use crate::ProcessIdentity;
use crate::ProcessLog;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpawnSpec {
    pub agent_id: AgentId,
    pub generation: u64,
    pub fleet_root: PathBuf,
    pub workspace: PathBuf,
    pub home_root: PathBuf,
    pub run_root: PathBuf,
    pub control_socket: PathBuf,
    pub logs_root: PathBuf,
    pub command: AgentCommand,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdoptSpec {
    pub agent_id: AgentId,
    pub registry_generation: u64,
    pub spawn_generation: u64,
    pub workspace: PathBuf,
    pub home_root: PathBuf,
    pub run_root: PathBuf,
    pub control_socket: PathBuf,
    pub identity: ProcessIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Running { healthy: bool, drained: bool },
    Exited(ProcessExit),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessObservation {
    pub state: ProcessState,
    pub logs: Vec<ProcessLog>,
}

/// One independently controlled agent child.
///
/// Every method must return promptly and must never wait for process exit or health.
pub trait ManagedProcess: Send {
    fn poll(&mut self, max_logs: usize) -> Result<ProcessObservation, ProcessDriverError>;
    fn request_drain(&mut self) -> Result<(), ProcessDriverError>;
    fn request_stop(&mut self) -> Result<(), ProcessDriverError>;
    fn kill(&mut self) -> Result<(), ProcessDriverError>;
}

pub struct SpawnedProcess<P> {
    pub identity: ProcessIdentity,
    pub process: P,
}

pub enum Adoption<P> {
    Adopted(P),
    Missing,
    Rejected,
}

/// Creates or verifies per-agent child processes without owning a shared execution queue.
///
/// Spawn and adoption must be bounded operations. Runtime polling and control live on each
/// [`ManagedProcess`] handle so one child has no shared wait path with another child.
pub trait ProcessDriver {
    type Process: ManagedProcess;

    fn spawn(
        &mut self,
        spec: &SpawnSpec,
    ) -> Result<SpawnedProcess<Self::Process>, ProcessDriverError>;

    fn adopt(&mut self, spec: &AdoptSpec) -> Result<Adoption<Self::Process>, ProcessDriverError>;
}
