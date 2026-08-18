use std::ffi::OsString;
use std::path::Component;
use std::path::PathBuf;
use std::time::Duration;

use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use serde::Deserialize;
use serde::Serialize;

use crate::SupervisorError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentCommand {
    pub program: PathBuf,
    pub args: Vec<OsString>,
}

impl AgentCommand {
    pub fn new(program: impl Into<PathBuf>, args: Vec<OsString>) -> Result<Self, SupervisorError> {
        let program = program.into();
        let argument_bytes = args.iter().try_fold(0_usize, |total, argument| {
            total.checked_add(argument.to_string_lossy().len())
        });
        if !program.is_absolute()
            || !program
                .components()
                .any(|component| matches!(component, Component::Normal(_)))
            || program
                .components()
                .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
            || args.len() > 128
            || program.to_string_lossy().len() > 4_096
            || argument_bytes.is_none_or(|bytes| bytes > 65_536)
        {
            return Err(SupervisorError::Invalid(
                "agent command path and arguments exceed bounded local-process limits".to_string(),
            ));
        }
        Ok(Self { program, args })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisorConfig {
    pub health_timeout: Duration,
    pub drain_timeout: Duration,
    pub stop_grace: Duration,
    pub event_capacity: usize,
    pub log_capacity: usize,
    pub max_log_bytes: usize,
    pub driver_poll_batch: usize,
}

impl SupervisorConfig {
    pub fn local_default() -> Self {
        Self {
            // A cold App Server start may need to hydrate its local runtime and
            // scan agent-owned configuration before the UDS health endpoint is
            // available. Keep this well above the observed debug/cold-start
            // time so the lifecycle controller does not kill a healthy agent.
            health_timeout: Duration::from_secs(60),
            drain_timeout: Duration::from_secs(30),
            stop_grace: Duration::from_secs(5),
            event_capacity: 128,
            log_capacity: 256,
            max_log_bytes: 4_096,
            driver_poll_batch: 64,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), SupervisorError> {
        if self.health_timeout.is_zero()
            || self.drain_timeout.is_zero()
            || self.stop_grace.is_zero()
            || !(1..=4_096).contains(&self.event_capacity)
            || !(1..=16_384).contains(&self.log_capacity)
            || !(1..=65_536).contains(&self.max_log_bytes)
            || !(1..=1_024).contains(&self.driver_poll_batch)
        {
            return Err(SupervisorError::Invalid(
                "supervisor deadlines and buffer bounds must be finite and non-zero".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessIdentity {
    system_id: u64,
    incarnation: String,
}

impl ProcessIdentity {
    pub fn new(system_id: u64, incarnation: impl Into<String>) -> Result<Self, SupervisorError> {
        let incarnation = incarnation.into();
        if system_id == 0
            || incarnation.is_empty()
            || incarnation.len() > 128
            || !incarnation.is_ascii()
        {
            return Err(SupervisorError::Invalid(
                "process identity must have a non-zero system id and bounded ASCII incarnation"
                    .to_string(),
            ));
        }
        Ok(Self {
            system_id,
            incarnation,
        })
    }

    pub fn system_id(&self) -> u64 {
        self.system_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessStream {
    Stdout,
    Stderr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessLog {
    pub stream: ProcessStream,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessExit {
    pub success: bool,
    pub code: Option<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SupervisorEventKind {
    Spawned,
    Healthy,
    Lifecycle(AgentLifecycle),
    DrainRequested,
    StopRequested,
    KillRequested,
    RestartQueued,
    Exited(ProcessExit),
    OrphanAdopted,
    OrphanMissing,
    OrphanRejected,
    GenerationFenced { runtime: u64, registry: u64 },
    DriverFault(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisorEvent {
    pub generation: u64,
    pub kind: SupervisorEventKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentFault {
    pub agent_id: AgentId,
    pub message: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TickReport {
    pub faults: Vec<AgentFault>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentSupervisorSnapshot {
    pub active: bool,
    pub runtime_generation: Option<u64>,
    pub events: Vec<SupervisorEvent>,
    pub logs: Vec<ProcessLog>,
}
