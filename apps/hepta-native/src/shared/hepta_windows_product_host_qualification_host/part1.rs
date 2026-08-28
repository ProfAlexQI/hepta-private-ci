use super::hepta_windows_product_host_integration_review::{
    HeptaWindowsProductHostReviewReceipt, HeptaWindowsProductHostReviewStatus,
    HeptaWindowsReviewAuthorityBoundary,
};

pub const HEPTA_WINDOWS_QUALIFICATION_FEATURE_NAME: &str =
    "hepta_ui_windows_system_material_v4";
pub const HEPTA_WINDOWS_QUALIFICATION_FEATURE_DEFAULT_ENABLED: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_AUTOMATIC_BINDING_ALLOWED: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_PRODUCT_WIRED: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_LIFECYCLE_WIRED: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_PRODUCT_BOUND: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_QUALIFICATION_RELEASE: bool = false;

pub const fn hepta_windows_qualification_feature_compiled() -> bool {
    cfg!(feature = "hepta_ui_windows_system_material_v4")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsQualificationPhase {
    Dormant,
    QualificationBound,
    QualifiedUnbound,
    RejectedSafe,
    RejectedUnsafe,
    Suspended,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsQualificationError {
    FeatureNotCompiled,
    FeatureDisabled,
    ReviewEnvelopeRejected,
    ReviewSealInvalid,
    ImplementationCandidateInvalid,
    ImplementationCandidateMismatch,
    ReviewDigestMismatch,
    ImplementationApprovalMissing,
    OperatorAcceptanceMissing,
    AuthorityEscape,
    WindowIdentityInvalid,
    TransparencyDisabled,
    HighContrast,
    RollbackRequired,
    RollbackNotRequired,
    HostShutdown,
    BackendBindFailed,
    BackendBindingReceiptRejected,
    RollbackFailed,
    RollbackReceiptRejected,
    QualificationEvidenceUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostReviewSeal {
    pub evidence_candidate_commit: String,
    pub evidence_candidate_tree: String,
    pub binding_digest: String,
}

impl HeptaWindowsProductHostReviewSeal {
    pub fn is_valid(&self) -> bool {
        git_object_id(&self.evidence_candidate_commit)
            && git_object_id(&self.evidence_candidate_tree)
            && sha256_hex(&self.binding_digest)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsQualificationApproval {
    pub implementation_candidate_commit: String,
    pub implementation_candidate_tree: String,
    pub review_binding_digest: String,
    pub feature_flag_enabled: bool,
    pub implementation_approved: bool,
    pub operator_accepted: bool,
    pub authority: HeptaWindowsReviewAuthorityBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsQualificationPreferences {
    pub transparency_allowed: bool,
    pub high_contrast: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsQualificationWindowIdentity {
    pub root_window_index: u64,
    pub root_window_generation: u64,
    pub root_hwnd: u64,
    pub transient_window_index: u64,
    pub transient_window_generation: u64,
    pub transient_hwnd: u64,
}

impl HeptaWindowsQualificationWindowIdentity {
    pub const fn is_valid(self) -> bool {
        let same_window_id = self.root_window_index == self.transient_window_index
            && self.root_window_generation == self.transient_window_generation;
        self.root_hwnd != 0
            && self.transient_hwnd != 0
            && self.root_hwnd != self.transient_hwnd
            && !same_window_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsQualificationBindingReceipt {
    pub identity: HeptaWindowsQualificationWindowIdentity,
    pub root_mica_exact: bool,
    pub transient_acrylic_exact: bool,
    pub authority: HeptaWindowsReviewAuthorityBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsQualificationRollbackReceipt {
    pub identity: HeptaWindowsQualificationWindowIdentity,
    pub root_none_exact: bool,
    pub transient_none_exact: bool,
    pub rollback_verified: bool,
    pub authority: HeptaWindowsReviewAuthorityBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsQualificationBackendError {
    BindFailed,
    RollbackFailed,
}

pub trait HeptaWindowsQualificationBackend {
    fn bind_qualification_verified(
        &mut self,
        identity: HeptaWindowsQualificationWindowIdentity,
    ) -> Result<HeptaWindowsQualificationBindingReceipt, HeptaWindowsQualificationBackendError>;

    fn rollback_qualification_to_solid_verified(
        &mut self,
        identity: HeptaWindowsQualificationWindowIdentity,
    ) -> Result<HeptaWindowsQualificationRollbackReceipt, HeptaWindowsQualificationBackendError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsQualificationRequest {
    pub review: HeptaWindowsProductHostReviewReceipt,
    pub review_seal: HeptaWindowsProductHostReviewSeal,
    pub implementation_candidate_commit: String,
    pub implementation_candidate_tree: String,
    pub approval: HeptaWindowsQualificationApproval,
    pub identity: HeptaWindowsQualificationWindowIdentity,
    pub preferences: HeptaWindowsQualificationPreferences,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsQualificationRuntimeReceipt {
    pub phase: HeptaWindowsQualificationPhase,
    pub generation: u64,
    pub accepted: bool,
    pub qualification_material_active: bool,
    pub root_mica_exact: bool,
    pub transient_acrylic_exact: bool,
    pub root_none_exact: bool,
    pub transient_none_exact: bool,
    pub rollback_required: bool,
    pub qualified_unbound: bool,
    pub implementation_approved: bool,
    pub operator_acceptance_verified: bool,
    pub product_host_may_bind: bool,
    pub product_bound: bool,
    pub transient_system_material_bound: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub native_product_runtime: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsQualificationRuntimeReceipt {
    pub const fn remains_non_product(self) -> bool {
        !self.product_host_may_bind
            && !self.product_bound
            && !self.transient_system_material_bound
            && !self.complete_profile_bound
            && !self.system_material_bound
            && !self.native_product_runtime
    }

    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.promotion
            && !self.release
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsQualificationEvidence {
    pub implementation_candidate_commit: String,
    pub implementation_candidate_tree: String,
    pub review_evidence_candidate_commit: String,
    pub review_evidence_candidate_tree: String,
    pub review_binding_digest: String,
    pub identity: HeptaWindowsQualificationWindowIdentity,
    pub root_mica_exact: bool,
    pub transient_acrylic_exact: bool,
    pub root_none_exact: bool,
    pub transient_none_exact: bool,
    pub qualified_unbound: bool,
    pub product_bound: bool,
    pub system_material_bound: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsQualificationEvidence {
    pub const fn remains_non_product(&self) -> bool {
        !self.product_bound && !self.system_material_bound
    }

    pub const fn grants_no_authority(&self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.promotion
            && !self.release
    }
}
