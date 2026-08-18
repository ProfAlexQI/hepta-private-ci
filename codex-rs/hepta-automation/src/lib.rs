//! Per-Agent durable automation queue.
//!
//! This crate stores schedules and leases, then emits a typed request to the
//! owning Agent's normal App Server thread queue. It has no model, tool, or
//! fleet-wide execution authority.

#![forbid(unsafe_code)]

mod model;
mod scheduler;
mod store;

pub use model::AutomationAdmission;
pub use model::AutomationError;
pub use model::AutomationLease;
pub use model::AutomationQueueReceipt;
pub use model::AutomationSchedule;
pub use model::AutomationTask;
pub use model::AutomationTaskDraft;
pub use model::AutomationTaskId;
pub use model::AutomationTaskState;
pub use model::AutomationTick;
pub use scheduler::AutomationFuture;
pub use scheduler::AutomationScheduler;
pub use scheduler::AutomationTurnQueue;
pub use store::AutomationStore;

pub const AUTOMATION_SCHEMA_VERSION: u32 = 1;
