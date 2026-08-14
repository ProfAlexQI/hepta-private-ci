//! Contract for an explicitly scoped Mac, Nix, and Linux qualification.
//!
//! This module does not verify receipt contents, publish an aggregate, create a
//! signing challenge, or grant transition authority. It fixes the scope that a
//! later verifier and ceremony must bind without mutating the frozen V3 family.

mod evidence;
mod model;
mod profiles;

pub use evidence::assess;
pub use model::CandidateBindingV1;
pub use model::ClosedAuthorityV1;
pub use model::DeferredGateMilestoneV1;
pub use model::DeferredOwnerV1;
pub use model::DeferredReasonV1;
pub use model::DeferredResumeConditionV1;
pub use model::DevelopmentDeferralV1;
pub use model::DevelopmentMilestoneV1;
pub use model::DevelopmentWorkItemV1;
pub use model::FullMatrixVerdictV1;
pub use model::GateContractV1;
pub use model::GateEvidenceV1;
pub use model::GateIdV1;
pub use model::PlatformProfileV1;
pub use model::ReceiptManifestPinV1;
pub use model::RepositoryBindingV1;
pub use model::RequiredGateObservationV1;
pub use model::ScopeContractV1;
pub use model::ScopeVerdictV1;
pub use model::ScopedQualificationAssessmentV1;
pub use model::ScopedQualificationInputV1;
pub use model::UiCandidateBindingV1;
pub use model::UiRouteStrategyV1;
pub use profiles::ACCEPTANCE_ARTIFACT_PREFIX;
pub use profiles::AGGREGATE_ARTIFACT_PREFIX;
pub use profiles::ASSESSMENT_SCHEMA;
pub use profiles::BACKEND_CANDIDATE_HEAD;
pub use profiles::BACKEND_CANDIDATE_TREE;
pub use profiles::PROFILE_SET;
pub use profiles::QUALIFICATION_NAMESPACE;
pub use profiles::SCHEMA;
pub use profiles::SSHSIG_NAMESPACE;
pub use profiles::UI_CANDIDATE_HEAD;
pub use profiles::UI_CANDIDATE_TREE;
pub use profiles::exact_contract;
pub use profiles::validate_contract;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
