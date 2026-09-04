//! Durable operation, outbox and reconciliation semantics for Hepta.
//!
//! Queue acknowledgement is deliberately separate from terminal effect
//! observation. Once dispatch may have crossed an external boundary, the
//! operation cannot be blindly retried; it remains indeterminate until a
//! current-fence observer reconciles it.

#![forbid(unsafe_code)]

mod error;
mod ledger;
mod model;
mod outbox;

pub use error::OperationError;
pub use ledger::OperationLedger;
pub use model::AuthorityWitness;
pub use model::OperationKey;
pub use model::OperationRecord;
pub use model::OperationState;
pub use model::ReconciliationOutcome;
pub use outbox::Outbox;
pub use outbox::OutboxIntent;
pub use outbox::OutboxState;
