use super::hepta_windows_product_host_integration_review::{
    HeptaWindowsProductHostReviewReceipt, HeptaWindowsProductHostReviewStatus,
    HeptaWindowsReviewAuthorityBoundary,
};

pub const HEPTA_WINDOWS_PRODUCT_HOST_FEATURE_NAME: &str =
    "hepta_ui_windows_system_material_v4";
pub const HEPTA_WINDOWS_PRODUCT_HOST_FEATURE_DEFAULT_ENABLED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_AUTOMATIC_BINDING_ALLOWED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_SOURCE_IMPLEMENTED: bool = true;
pub const HEPTA_WINDOWS_PRODUCT_HOST_PRODUCT_WIRED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_LIFECYCLE_WIRED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_PRODUCT_BOUND: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_DEVICE_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_HOST_RELEASE: bool = false;

pub const fn hepta_windows_product_host_feature_compiled() -> bool {
    cfg!(feature = "hepta_ui_windows_system_material_v4")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsProductHostPhase {
    Dormant,
    Bound,
    SolidFallback,
    Suspended,
    Shutdown,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsProductHostError {
    FeatureNotCompiled,
    FeatureDisabled,
    ReviewEnvelopeRejected,
    ImplementationApprovalMissing,
    OperatorAcceptanceMissing,
    DeviceValidationMissing,
    RollbackDrillMissing,
    CandidateIdentityInvalid,
    ReviewDigestInvalid,
    AuthorityEscape,
    WindowIdentityInvalid,
    TransparencyDisabled,
    HighContrast,
    AlreadyBound,
    HostShutdown,
    BackendBindFailed,
    BackendBindingReceiptRejected,
    RollbackFailed,
    RollbackReceiptRejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostWindowIdentity {
    pub root_window_index: u64,
    pub root_window_generation: u64,
    pub root_hwnd: u64,
    pub transient_window_index: u64,
    pub transient_window_generation: u64,
    pub transient_hwnd: u64,
}

impl HeptaWindowsProductHostWindowIdentity {
    pub const fn is_valid(self) -> bool {
        let same_window_id = self.root_window_index == self.transient_window_index
            && self.root_window_generation == self.transient_window_generation;
        self.root_hwnd != 0
            && self.transient_hwnd != 0
            && self.root_hwnd != self.transient_hwnd
            && !same_window_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostImplementationApproval {
    pub candidate_commit: String,
    pub candidate_tree: String,
    pub review_binding_digest: String,
    pub implementation_approved: bool,
    pub operator_accepted: bool,
    pub physical_device_validated: bool,
    pub rollback_drill_validated: bool,
    pub feature_flag_enabled: bool,
    pub authority: HeptaWindowsReviewAuthorityBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostRuntimePreferences {
    pub transparency_allowed: bool,
    pub high_contrast: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsVerifiedMaterialBinding {
    pub identity: HeptaWindowsProductHostWindowIdentity,
    pub root_mica_exact: bool,
    pub transient_acrylic_exact: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub authority: HeptaWindowsReviewAuthorityBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsVerifiedMaterialRollback {
    pub identity: HeptaWindowsProductHostWindowIdentity,
    pub root_none_exact: bool,
    pub transient_none_exact: bool,
    pub rollback_verified: bool,
    pub authority: HeptaWindowsReviewAuthorityBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsProductMaterialBackendError {
    BindFailed,
    RollbackFailed,
}

pub trait HeptaWindowsProductMaterialBackend {
    fn bind_verified(
        &mut self,
        identity: HeptaWindowsProductHostWindowIdentity,
    ) -> Result<HeptaWindowsVerifiedMaterialBinding, HeptaWindowsProductMaterialBackendError>;

    fn rollback_to_solid_verified(
        &mut self,
        identity: HeptaWindowsProductHostWindowIdentity,
    ) -> Result<HeptaWindowsVerifiedMaterialRollback, HeptaWindowsProductMaterialBackendError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostActivationRequest {
    pub review: HeptaWindowsProductHostReviewReceipt,
    pub approval: HeptaWindowsProductHostImplementationApproval,
    pub identity: HeptaWindowsProductHostWindowIdentity,
    pub preferences: HeptaWindowsProductHostRuntimePreferences,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostRuntimeReceipt {
    pub phase: HeptaWindowsProductHostPhase,
    pub accepted: bool,
    pub generation: u64,
    pub implementation_approved: bool,
    pub operator_acceptance_verified: bool,
    pub device_validation_verified: bool,
    pub rollback_drill_verified: bool,
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

impl HeptaWindowsProductHostRuntimeReceipt {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.promotion
            && !self.release
    }
}
