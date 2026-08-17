//! Pure trust model for the scoped MNL successor.
//!
//! This crate has no filesystem, process, network, clock, replay-store, or
//! signing implementation. It records the exact completion blockers and can
//! structurally inspect an out-of-tree Git ancestry path or prepare an exact
//! replay slot/full-binding record from already inspected signature roles.
//! These structural observations are not trust decisions and never authorize
//! a live action.

#![forbid(unsafe_code)]

mod ancestry;
mod capability;
mod model;
mod replay;
mod signature;

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
pub use model::DetachedSignatureManifestV1;
pub use model::DetachedSignatureRoleV1;
pub use model::EvidenceOriginV1;
pub use model::GitAncestryPathManifestV1;
pub use model::GitAncestryPathPolicyV1;
pub use model::GitCommitManifestEntryV1;
pub use model::RawDetachedEd25519SignatureV1;
pub use model::RawGitCommitSidecarV1;
pub use model::RepositoryIdentityV1;
pub use model::StructuralAncestryInspectionV1;
pub use model::VerifiedDetachedSignatureInspectionV1;
pub use replay::COPY_ACK_REPLAY_CLAIM_SCHEMA;
pub use replay::CopyAckReplayClaimWireV1;
pub use replay::ExpectedPreparedPreRunReplayClaimLineageV1;
pub use replay::MAX_REPLAY_CLAIM_BYTES;
pub use replay::MAX_SIGNED_FRESHNESS_LIFETIME_SECONDS;
pub use replay::MatchedPreparedPreRunReplayClaimInspectionV1;
pub use replay::PRE_RUN_REPLAY_CLAIM_SCHEMA;
pub use replay::PreRunReplayClaimWireV1;
pub use replay::PreparedCopyAckReplayClaimV1;
pub use replay::PreparedPreRunReplayClaimV1;
pub use replay::ReplayClaimNamespaceV1;
pub use replay::ReplayPlatformScopeV1;
pub use replay::SIGNED_PRE_RUN_REPLAY_PROFILE_SCHEMA;
pub use replay::SignedPreRunReplayProfileV1;
pub use replay::derive_copy_ack_replay_slot_sha256;
pub use replay::derive_pre_run_replay_slot_sha256;
pub use replay::derive_run_identity_sha256;
pub use replay::inspect_canonical_copy_ack_replay_claim;
pub use replay::inspect_canonical_pre_run_replay_claim;
pub use replay::inspect_prepared_pre_run_replay_claim_lineage;
pub use signature::ALL_DETACHED_SIGNATURE_ROLES;
pub use signature::DETACHED_SIGNATURE_ALGORITHM;
pub use signature::DETACHED_SIGNATURE_MANIFEST_SCHEMA;
pub use signature::MAX_DETACHED_SIGNATURE_MANIFEST_BYTES;
pub use signature::MAX_DETACHED_SIGNATURE_PAYLOAD_BYTES;
pub use signature::PRODUCTION_SIGNATURE_POLICY_AVAILABLE;
pub use signature::inspect_final_artifact_freeze_signature;
pub use signature::inspect_freeze_manifest_signature;
pub use signature::inspect_independent_copy_ack_signature;
pub use signature::inspect_post_run_result_envelope_signature;
pub use signature::inspect_pre_run_profile_signature;
pub use signature::inspect_supervisor_seal_signature;
pub use signature::inspect_terminal_manifest_signature;

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
mod replay_tests;
#[cfg(test)]
mod signature_tests;
#[cfg(test)]
mod tests;
