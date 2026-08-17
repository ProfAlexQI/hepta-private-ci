//! Pure trust model for the scoped MNL successor.
//!
//! This crate has no filesystem, process, network, clock, replay-store, or
//! signing implementation. It records the exact completion blockers and can
//! structurally inspect an out-of-tree Git ancestry path. Structural ancestry
//! is not a trust decision and never authorizes a live action.

#![forbid(unsafe_code)]

mod ancestry;
mod capability;
mod model;

pub use ancestry::ANCESTRY_PATH_PROOF_SCHEMA;
pub use ancestry::MAX_ANCESTRY_COMMITS;
pub use ancestry::MAX_ANCESTRY_MANIFEST_BYTES;
pub use ancestry::MAX_COMMIT_PARENTS;
pub use ancestry::MAX_RAW_COMMIT_BYTES;
pub use ancestry::MAX_TOTAL_RAW_COMMIT_BYTES;
pub use ancestry::PHASE_A_ANCHOR_COMMIT_RAW_BYTES;
pub use ancestry::PHASE_A_ANCHOR_COMMIT_RAW_SHA256;
pub use ancestry::PHASE_A_ANCHOR_HEAD;
pub use ancestry::PHASE_A_ANCHOR_TREE;
pub use ancestry::exact_phase_a_anchor;
pub use ancestry::inspect_canonical_ancestry_path;
pub use capability::CAPABILITY_LEDGER_SCHEMA;
pub use capability::MAX_CAPABILITY_LEDGER_BYTES;
pub use capability::exact_phase_a_capability_ledger;
pub use capability::exact_phase_a_capability_ledger_bytes;
pub use capability::inspect_canonical_phase_a_capability_ledger;
pub use capability::validate_phase_a_capability_ledger;
pub use model::AbsentCapabilityLedgerInspectionV1;
pub use model::AuthorityDispositionV1;
pub use model::CapabilityEntryV1;
pub use model::CapabilityStateV1;
pub use model::CompletionCapabilityLedgerV1;
pub use model::CompletionCapabilityV1;
pub use model::CompletionDispositionV1;
pub use model::EvidenceOriginV1;
pub use model::GitAncestryPathManifestV1;
pub use model::GitAncestryPathPolicyV1;
pub use model::GitCommitManifestEntryV1;
pub use model::RawGitCommitSidecarV1;
pub use model::RepositoryIdentityV1;
pub use model::StructuralAncestryInspectionV1;

#[derive(Debug, thiserror::Error)]
pub enum MnlTrustError {
    #[error("invalid MNL trust material: {0}")]
    Invalid(String),
    #[error("MNL trust serialization failed: {0}")]
    Serialization(String),
}

pub(crate) fn invalid(message: impl Into<String>) -> MnlTrustError {
    MnlTrustError::Invalid(message.into())
}

#[cfg(test)]
mod tests;
