//! Pure Phase-A model for the Nix member of the MNL successor.
//!
//! This crate deliberately contains no Docker, Nix, filesystem, process,
//! network, host-discovery, receipt-writing, or live authority implementation.
//! It can describe and shape-check evidence, but the production planner and
//! verifier remain compile-time blocked until separately frozen inputs exist.

mod final_join;
mod model;
mod run_plan;
mod sandbox_feasibility;
mod verify;

pub use final_join::*;
pub use model::*;
pub use run_plan::*;
pub use sandbox_feasibility::*;
pub use verify::*;

use thiserror::Error;

pub const MAX_CANONICAL_BYTES: usize = 1024 * 1024;

#[derive(Debug, Error)]
pub enum NixMnlError {
    #[error("BLOCKED: {0}")]
    Blocked(String),
    #[error("invalid Nix MNL successor v1 model: {0}")]
    Invalid(String),
    #[error("Nix MNL successor v1 serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub(crate) fn blocked(message: impl Into<String>) -> NixMnlError {
    NixMnlError::Blocked(message.into())
}

pub(crate) fn invalid(message: impl Into<String>) -> NixMnlError {
    NixMnlError::Invalid(message.into())
}

#[cfg(test)]
mod final_join_tests;

#[cfg(test)]
mod run_plan_tests;

#[cfg(test)]
mod tests;
