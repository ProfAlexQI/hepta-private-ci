//! Append-only causal learning facts with independent outcomes and revocation
//! lineage.
//!
//! The crate owns no model, tool, network, filesystem, selection, promotion or
//! release authority. Callers persist immutable snapshots through separately
//! authorized storage boundaries.

#![forbid(unsafe_code)]

mod error;
mod ledger;
mod model;

pub use error::LedgerError;
pub use ledger::LearningLedger;
pub use model::AppendDisposition;
pub use model::AppendReceipt;
pub use model::CandidateSetCompleteness;
pub use model::CreditAssignment;
pub use model::EpisodeDecision;
pub use model::LedgerEvent;
pub use model::LedgerRecord;
pub use model::LedgerSnapshot;
pub use model::OutcomeFinality;
pub use model::OutcomeObservation;
pub use model::Revocation;
