//! Review-only admission controller for a future Windows product-material host.
//!
//! This module consumes already-qualified evidence as typed facts and decides
//! whether an implementation review may begin. It deliberately cannot bind a
//! system material, mutate the product lifecycle, or grant effect/production
//! authority. Product integration remains a separate, operator-governed tranche.

pub const HEPTA_WINDOWS_REVIEW_MAKEPAD_REVISION: &str = "c4335cee10b22aca768510c9d072b0ca1bba15c8";
pub const HEPTA_WINDOWS_REVIEW_SOURCE_IMPLEMENTED: bool = true;
pub const HEPTA_WINDOWS_REVIEW_RUNTIME_PROVENANCE_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_REVIEW_IMPLEMENTATION_APPROVED: bool = false;
pub const HEPTA_WINDOWS_REVIEW_PRODUCT_HOST_MAY_BIND: bool = false;
pub const HEPTA_WINDOWS_REVIEW_PRODUCT_BOUND: bool = false;
pub const HEPTA_WINDOWS_REVIEW_TRANSIENT_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_REVIEW_COMPLETE_PROFILE_BOUND: bool = false;
pub const HEPTA_WINDOWS_REVIEW_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_REVIEW_NATIVE_PRODUCT_RUNTIME: bool = false;
pub const HEPTA_WINDOWS_REVIEW_DEVICE_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_REVIEW_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_REVIEW_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_REVIEW_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_REVIEW_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_REVIEW_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_REVIEW_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsProvenanceLevel {
    Failed,
    PreRuntime,
    WindowsRuntime,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HeptaWindowsReviewAuthorityBoundary {
    pub network: bool,
    pub mutation: bool,
    pub effect: bool,
    pub live_adapter: bool,
    pub production: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsReviewAuthorityBoundary {
    pub const fn grants_none(self) -> bool {
        !self.network
            && !self.mutation
            && !self.effect
            && !self.live_adapter
            && !self.production
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HeptaWindowsReviewBindingBoundary {
    pub implementation_approved: bool,
    pub product_host_may_bind: bool,
    pub product_bound: bool,
    pub transient_system_material_bound: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub native_product_runtime: bool,
    pub device_validated: bool,
}

impl HeptaWindowsReviewBindingBoundary {
    pub const fn remains_unbound(self) -> bool {
        !self.implementation_approved
            && !self.product_host_may_bind
            && !self.product_bound
            && !self.transient_system_material_bound
            && !self.complete_profile_bound
            && !self.system_material_bound
            && !self.native_product_runtime
            && !self.device_validated
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostReviewEvidence {
    pub candidate_commit: String,
    pub candidate_tree: String,
    pub provenance_candidate_commit: String,
    pub provenance_candidate_tree: String,
    pub qualification_candidate_commit: String,
    pub qualification_candidate_tree: String,
    pub makepad_revision: String,
    pub provenance_level: HeptaWindowsProvenanceLevel,
    pub qualification_level: HeptaWindowsProvenanceLevel,
    pub root_window_index: u64,
    pub root_window_generation: u64,
    pub root_hwnd: u64,
    pub transient_window_index: u64,
    pub transient_window_generation: u64,
    pub transient_hwnd: u64,
    pub root_sequence: u64,
    pub acrylic_sequence: u64,
    pub rollback_sequence: u64,
    pub root_mica_exact: bool,
    pub transient_acrylic_exact: bool,
    pub solid_rollback_exact: bool,
    pub destroyed_acknowledged: bool,
    pub authority: HeptaWindowsReviewAuthorityBoundary,
    pub existing_binding: HeptaWindowsReviewBindingBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsProductHostReviewStatus {
    EligibleForImplementationReview,
    RejectedRuntimeProvenance,
    RejectedQualificationIndex,
    RejectedCandidateIdentity,
    RejectedMakepadRevision,
    RejectedWindowIdentity,
    RejectedRuntimeSequence,
    RejectedRuntimeChain,
    RejectedBindingEscape,
    RejectedAuthorityEscape,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostIntegrationPlan {
    pub feature_flag_default_enabled: bool,
    pub automatic_binding_allowed: bool,
    pub operator_acceptance_required: bool,
    pub device_validation_required: bool,
    pub rollback_to_solid_required: bool,
    pub high_contrast_recheck_required: bool,
    pub transparency_preference_recheck_required: bool,
    pub suspend_shutdown_unbind_required: bool,
}

impl Default for HeptaWindowsProductHostIntegrationPlan {
    fn default() -> Self {
        Self {
            feature_flag_default_enabled: false,
            automatic_binding_allowed: false,
            operator_acceptance_required: true,
            device_validation_required: true,
            rollback_to_solid_required: true,
            high_contrast_recheck_required: true,
            transparency_preference_recheck_required: true,
            suspend_shutdown_unbind_required: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostReviewReceipt {
    pub status: HeptaWindowsProductHostReviewStatus,
    pub accepted: bool,
    pub eligible_for_implementation_review: bool,
    pub plan: HeptaWindowsProductHostIntegrationPlan,
    pub implementation_approved: bool,
    pub product_host_may_bind: bool,
    pub product_bound: bool,
    pub transient_system_material_bound: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub native_product_runtime: bool,
    pub device_validated: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsProductHostReviewReceipt {
    pub const fn remains_review_only(self) -> bool {
        !self.implementation_approved
            && !self.product_host_may_bind
            && !self.product_bound
            && !self.transient_system_material_bound
            && !self.complete_profile_bound
            && !self.system_material_bound
            && !self.native_product_runtime
            && !self.device_validated
            && !self.plan.feature_flag_default_enabled
            && !self.plan.automatic_binding_allowed
    }

    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }
}

pub fn evaluate_windows_product_host_integration_review(
    evidence: &HeptaWindowsProductHostReviewEvidence,
) -> HeptaWindowsProductHostReviewReceipt {
    let status = validate(evidence);
    let accepted = status == HeptaWindowsProductHostReviewStatus::EligibleForImplementationReview;
    HeptaWindowsProductHostReviewReceipt {
        status,
        accepted,
        eligible_for_implementation_review: accepted,
        plan: HeptaWindowsProductHostIntegrationPlan::default(),
        implementation_approved: false,
        product_host_may_bind: false,
        product_bound: false,
        transient_system_material_bound: false,
        complete_profile_bound: false,
        system_material_bound: false,
        native_product_runtime: false,
        device_validated: false,
        production_authority: false,
        effect_authority: false,
        live_adapter_authority: false,
        operator_acceptance: false,
        promotion: false,
        release: false,
    }
}

fn validate(
    evidence: &HeptaWindowsProductHostReviewEvidence,
) -> HeptaWindowsProductHostReviewStatus {
    if evidence.provenance_level != HeptaWindowsProvenanceLevel::WindowsRuntime {
        return HeptaWindowsProductHostReviewStatus::RejectedRuntimeProvenance;
    }
    if evidence.qualification_level != HeptaWindowsProvenanceLevel::WindowsRuntime {
        return HeptaWindowsProductHostReviewStatus::RejectedQualificationIndex;
    }
    if !git_object_id(&evidence.candidate_commit)
        || !git_object_id(&evidence.candidate_tree)
        || evidence.candidate_commit != evidence.provenance_candidate_commit
        || evidence.candidate_tree != evidence.provenance_candidate_tree
        || evidence.candidate_commit != evidence.qualification_candidate_commit
        || evidence.candidate_tree != evidence.qualification_candidate_tree
    {
        return HeptaWindowsProductHostReviewStatus::RejectedCandidateIdentity;
    }
    if evidence.makepad_revision != HEPTA_WINDOWS_REVIEW_MAKEPAD_REVISION {
        return HeptaWindowsProductHostReviewStatus::RejectedMakepadRevision;
    }
    let same_window_id = evidence.root_window_index == evidence.transient_window_index
        && evidence.root_window_generation == evidence.transient_window_generation;
    if evidence.root_hwnd == 0
        || evidence.transient_hwnd == 0
        || evidence.root_hwnd == evidence.transient_hwnd
        || same_window_id
    {
        return HeptaWindowsProductHostReviewStatus::RejectedWindowIdentity;
    }
    if evidence.root_sequence != 1
        || evidence.acrylic_sequence != 2
        || evidence.rollback_sequence != 3
    {
        return HeptaWindowsProductHostReviewStatus::RejectedRuntimeSequence;
    }
    if !evidence.root_mica_exact
        || !evidence.transient_acrylic_exact
        || !evidence.solid_rollback_exact
        || !evidence.destroyed_acknowledged
    {
        return HeptaWindowsProductHostReviewStatus::RejectedRuntimeChain;
    }
    if !evidence.existing_binding.remains_unbound() {
        return HeptaWindowsProductHostReviewStatus::RejectedBindingEscape;
    }
    if !evidence.authority.grants_none() {
        return HeptaWindowsProductHostReviewStatus::RejectedAuthorityEscape;
    }
    HeptaWindowsProductHostReviewStatus::EligibleForImplementationReview
}

fn git_object_id(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence() -> HeptaWindowsProductHostReviewEvidence {
        HeptaWindowsProductHostReviewEvidence {
            candidate_commit: "a".repeat(40),
            candidate_tree: "b".repeat(40),
            provenance_candidate_commit: "a".repeat(40),
            provenance_candidate_tree: "b".repeat(40),
            qualification_candidate_commit: "a".repeat(40),
            qualification_candidate_tree: "b".repeat(40),
            makepad_revision: HEPTA_WINDOWS_REVIEW_MAKEPAD_REVISION.to_string(),
            provenance_level: HeptaWindowsProvenanceLevel::WindowsRuntime,
            qualification_level: HeptaWindowsProvenanceLevel::WindowsRuntime,
            root_window_index: 1,
            root_window_generation: 7,
            root_hwnd: 11,
            transient_window_index: 2,
            transient_window_generation: 9,
            transient_hwnd: 12,
            root_sequence: 1,
            acrylic_sequence: 2,
            rollback_sequence: 3,
            root_mica_exact: true,
            transient_acrylic_exact: true,
            solid_rollback_exact: true,
            destroyed_acknowledged: true,
            authority: HeptaWindowsReviewAuthorityBoundary::default(),
            existing_binding: HeptaWindowsReviewBindingBoundary::default(),
        }
    }

    #[test]
    fn exact_runtime_evidence_is_review_eligible_but_never_executable() {
        let receipt = evaluate_windows_product_host_integration_review(&evidence());
        assert_eq!(
            receipt.status,
            HeptaWindowsProductHostReviewStatus::EligibleForImplementationReview
        );
        assert!(receipt.accepted);
        assert!(receipt.eligible_for_implementation_review);
        assert!(receipt.remains_review_only());
        assert!(receipt.grants_no_authority());
        assert!(receipt.plan.operator_acceptance_required);
        assert!(receipt.plan.device_validation_required);
        assert!(receipt.plan.rollback_to_solid_required);
    }

    #[test]
    fn pre_runtime_or_missing_qualification_is_rejected() {
        let mut value = evidence();
        value.provenance_level = HeptaWindowsProvenanceLevel::PreRuntime;
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedRuntimeProvenance
        );
        let mut value = evidence();
        value.qualification_level = HeptaWindowsProvenanceLevel::PreRuntime;
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedQualificationIndex
        );
    }

    #[test]
    fn candidate_or_makepad_drift_is_rejected() {
        let mut value = evidence();
        value.provenance_candidate_commit = "c".repeat(40);
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedCandidateIdentity
        );
        let mut value = evidence();
        value.makepad_revision = "d".repeat(40);
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedMakepadRevision
        );
    }

    #[test]
    fn reused_window_or_sequence_drift_is_rejected() {
        let mut value = evidence();
        value.transient_hwnd = value.root_hwnd;
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedWindowIdentity
        );
        let mut value = evidence();
        value.rollback_sequence = 4;
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedRuntimeSequence
        );
    }

    #[test]
    fn incomplete_runtime_chain_is_rejected() {
        let mut value = evidence();
        value.destroyed_acknowledged = false;
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedRuntimeChain
        );
    }

    #[test]
    fn binding_or_authority_escape_is_rejected() {
        let mut value = evidence();
        value.existing_binding.product_bound = true;
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedBindingEscape
        );
        let mut value = evidence();
        value.authority.production = true;
        assert_eq!(
            evaluate_windows_product_host_integration_review(&value).status,
            HeptaWindowsProductHostReviewStatus::RejectedAuthorityEscape
        );
    }
}
