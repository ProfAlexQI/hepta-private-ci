//! Fail-closed contracts and independent verification for Linux exact v8.
//!
//! This crate is a qualification sibling. It does not enable any historical
//! v1-v7 artifact, install privileged services, stop runners, or execute the
//! product candidate.

mod identity;

pub use identity::*;

pub const SCHEMA_FAMILY: &str = "hepta_linux_exact_v8";
pub const CANDIDATE_HEAD: &str = "52ec4b3868fc5272e19ed516d00e11e44c549ea4";
pub const CANDIDATE_TREE: &str = "247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d";

#[derive(Debug, thiserror::Error)]
pub enum QualificationError {
    #[error("invalid Linux v8 qualification evidence: {0}")]
    Invalid(String),
}

pub(crate) fn invalid(message: impl Into<String>) -> QualificationError {
    QualificationError::Invalid(message.into())
}
