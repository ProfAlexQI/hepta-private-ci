//! Per-agent process lifecycle control for the Hepta workspace fleet.
//!
//! The supervisor does not execute turns or forward messages, models, or tokens.

mod driver;
mod error;
mod lease;
mod model;
mod recovery;
mod runtime;
mod supervisor;

#[cfg(unix)]
mod unix;

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
pub use model::AgentSupervisorSnapshot;
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
