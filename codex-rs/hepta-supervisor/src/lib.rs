//! Per-agent process lifecycle control for the Hepta workspace fleet.
//!
//! The supervisor does not execute turns or forward messages, models, or tokens.

mod control;
mod daemon;
mod daemon_client;
mod daemon_protocol;
mod driver;
mod error;
mod lease;
mod model;
mod recovery;
mod release;
mod runtime;
mod supervisor;
mod tick;

#[cfg(unix)]
mod unix;

pub use daemon::run_supervisord;
pub use daemon_client::SupervisordClient;
pub use daemon_protocol::ControlStateDigest;
pub use daemon_protocol::SUPERVISORD_CONTROL_SCHEMA_VERSION;
pub use daemon_protocol::SupervisorEpoch;
pub use daemon_protocol::SupervisordAgentStatus;
pub use daemon_protocol::SupervisordControlFence;
pub use daemon_protocol::SupervisordHealth;
pub use daemon_protocol::SupervisordMethod;
pub use daemon_protocol::SupervisordMutation;
pub use daemon_protocol::SupervisordMutationAccepted;
pub use daemon_protocol::SupervisordPayload;
pub use daemon_protocol::SupervisordRequest;
pub use daemon_protocol::SupervisordRequestValidationError;
pub use daemon_protocol::SupervisordResponse;
pub use driver::AdoptSpec;
pub use driver::Adoption;
pub use driver::ManagedProcess;
pub use driver::ProcessDriver;
pub use driver::ProcessObservation;
pub use driver::ProcessState;
pub use driver::SpawnSpec;
pub use driver::SpawnedProcess;
pub use error::ProcessDriverError;
pub use error::SupervisorError;
pub use model::AgentCommand;
pub use model::AgentFault;
pub use model::AgentRelease;
pub use model::AgentSupervisorSnapshot;
pub(crate) use model::ControlReleaseChange;
pub(crate) use model::ControlReleaseChangePhase;
pub(crate) use model::ControlRuntimePhase;
pub use model::ProcessExit;
pub use model::ProcessIdentity;
pub use model::ProcessLog;
pub use model::ProcessStream;
pub use model::SupervisorConfig;
pub use model::SupervisorEvent;
pub use model::SupervisorEventKind;
pub use model::TickReport;
pub use supervisor::Supervisor;

#[cfg(unix)]
pub use unix::UnixManagedProcess;
#[cfg(unix)]
pub use unix::UnixProcessDriver;
