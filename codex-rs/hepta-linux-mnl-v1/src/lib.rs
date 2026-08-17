//! First safe, model-only slice for `LinuxExactMnlV1`.
//!
//! This crate fixes the composite product/UI/qualification identity and the
//! publication boundary for a future Linux target qualification. It performs
//! no host discovery, file I/O, command execution, installation, workload
//! signalling, receipt signing, or transition. Production plan construction
//! fails closed while any independently published profile, final-tooling, or
//! collector pin is absent, and until a fresh internal challenge generator is
//! implemented.

mod canonical;
mod identity;
mod phase1;
mod profiles;

pub use canonical::canonical_json;
pub use canonical::canonical_sha256;
pub use canonical::decode_canonical_json;
pub use identity::BACKEND_CANDIDATE_HEAD;
pub use identity::BACKEND_CANDIDATE_TREE;
pub use identity::COMPILED_SUCCESSOR_TOOLING_PINS_V1;
pub use identity::CompiledSuccessorToolingPinsV1;
pub use identity::CompositeIdentityV1;
pub use identity::RepositoryIdentityV1;
pub use identity::TOOLING_BASELINE_HEAD;
pub use identity::TOOLING_BASELINE_TREE;
pub use identity::UI_CANDIDATE_HEAD;
pub use identity::UI_CANDIDATE_TREE;
pub use identity::exact_composite_identity;
pub use identity::validate_composite_identity;
pub use phase1::AuthorityBoundaryV1;
pub use phase1::AuthorityLayerV1;
pub use phase1::CollectorEnvironmentV1;
pub use phase1::CompiledPhase1StatusV1;
pub use phase1::ObservationKindV1;
pub use phase1::Phase1CollectorPlanV1;
pub use phase1::ReceiptKindV1;
pub use phase1::ReceiptTopologyNodeV1;
pub use phase1::compiled_phase1_status;
pub use phase1::exact_receipt_topology;
pub use phase1::production_phase1_plan;
pub use profiles::COMPILED_PUBLISHED_PROFILE_PINS_V1;
pub use profiles::CompiledProfilePinStatusV1;
pub use profiles::ExternalWatermarkCurrentTipV1;
pub use profiles::ExternalWatermarkProviderProfileV1;
pub use profiles::InstallEpochCompletionV1;
pub use profiles::PRODUCTION_TARGET_ALIAS_V1;
pub use profiles::PublishedProfileDocumentsV1;
pub use profiles::StateRootProfileV1;
pub use profiles::TargetProfileV1;
pub use profiles::TargetRoleV1;
pub use profiles::TrustProfileV1;
pub use profiles::TrustPurposeV1;
pub use profiles::compiled_profile_status;
pub use profiles::validate_published_profiles;

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum LinuxMnlError {
    #[error("LinuxExactMnlV1 BLOCKED: {0}")]
    Blocked(String),
    #[error("invalid LinuxExactMnlV1 model: {0}")]
    Invalid(String),
}

pub(crate) fn blocked(message: impl Into<String>) -> LinuxMnlError {
    LinuxMnlError::Blocked(message.into())
}

pub(crate) fn invalid(message: impl Into<String>) -> LinuxMnlError {
    LinuxMnlError::Invalid(message.into())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
