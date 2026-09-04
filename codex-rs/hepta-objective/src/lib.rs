//! Deterministic, authority-free objective compilation.
//!
//! This crate accepts only typed structured evidence. It never interprets free
//! text as authority, relaxes hard constraints, selects an action, or executes
//! an external effect.

#![forbid(unsafe_code)]

mod compiler;
mod error;
mod model;

pub use compiler::compile;
pub use error::ObjectiveError;
pub use model::ActionClass;
pub use model::CompileDisposition;
pub use model::ConfirmationPolicy;
pub use model::Constraint;
pub use model::ConstraintClass;
pub use model::ConstraintRelation;
pub use model::ObjectiveCompileReceipt;
pub use model::ObjectiveConflictReceipt;
pub use model::ObjectiveFunction;
pub use model::ObjectiveSourceEnvelope;
pub use model::PredicateTerminality;
pub use model::SoftDirection;
pub use model::SoftPreference;
pub use model::SourceTrust;
pub use model::SuccessPredicate;
