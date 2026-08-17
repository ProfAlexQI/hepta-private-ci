//! Read-only Phase A verifier for the MNL successor family.
//!
//! This sibling binds the exact backend product, UI product, qualification
//! tooling integration base, development freezes, gate dispositions, and the
//! currently present frozen receipt identities. It cannot produce PASS,
//! aggregate, ceremony, authority, ref changes, or transition actions.

mod model;
mod profiles;
mod verifier;

pub use model::ClosedAuthorityV1;
pub use model::DecisionReceiptBindingV1;
pub use model::DeferredGateMilestoneV1;
pub use model::DeferredOwnerV1;
pub use model::DeferredReasonV1;
pub use model::DeferredResumeConditionV1;
pub use model::DevelopmentDeferralV1;
pub use model::DevelopmentMilestoneV1;
pub use model::DevelopmentWorkItemV1;
pub use model::FullMatrixVerdictV1;
pub use model::GateContractV1;
pub use model::GateIdV1;
pub use model::GateVerificationStateV1;
pub use model::GateVerificationV1;
pub use model::PhaseAVerdictV1;
pub use model::PlatformProfileV1;
pub use model::PlatformReceiptBindingV1;
pub use model::ProductCandidateBindingV1;
pub use model::ReceiptArtifactPinV1;
pub use model::ReceiptLayerBindingV1;
pub use model::ReceiptLayerIdV1;
pub use model::RepositoryBindingV1;
pub use model::SuccessorContractV1;
pub use model::SuccessorVerificationV1;
pub use model::ToolingIntegrationBindingV1;
pub use model::UiCandidateBindingV1;
pub use model::UiRouteStrategyV1;
pub use model::VerifiedDecisionReceiptV1;
pub use model::VerifiedReceiptIdentityV1;
pub use profiles::BACKEND_CANDIDATE_HEAD;
pub use profiles::BACKEND_CANDIDATE_TREE;
pub use profiles::DEVELOPMENT_FREEZE_DECISION_SHA256;
pub use profiles::DEVELOPMENT_FREEZE_MANIFEST_SHA256;
pub use profiles::DEVELOPMENT_FREEZE_RECEIPT_ROOT_NAME;
pub use profiles::MAC_OUTER_MANIFEST_SHA256;
pub use profiles::MAC_RECEIPT_ROOT_NAME;
pub use profiles::NIX_INNER_MANIFEST_SHA256;
pub use profiles::NIX_OUTER_MANIFEST_SHA256;
pub use profiles::NIX_RECEIPT_ROOT_NAME;
pub use profiles::PROFILE_SET;
pub use profiles::RECEIPTS_PARENT;
pub use profiles::SCHEMA;
pub use profiles::STRATEGY_DECISION_SHA256;
pub use profiles::STRATEGY_RECEIPT_MANIFEST_SHA256;
pub use profiles::STRATEGY_RECEIPT_ROOT_NAME;
pub use profiles::TOOLING_INTEGRATION_BASE;
pub use profiles::TOOLING_INTEGRATION_TREE;
pub use profiles::UI_CANDIDATE_HEAD;
pub use profiles::UI_CANDIDATE_TREE;
pub use profiles::VERIFICATION_SCHEMA;
pub use profiles::exact_contract;
pub use profiles::validate_contract;
pub use verifier::verify_current_receipts;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
