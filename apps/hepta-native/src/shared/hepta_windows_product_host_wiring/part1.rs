use super::hepta_windows_material_adapter::{
    HeptaWindowsBackdropApi, HeptaWindowsBackdropKind, HeptaWindowsBackdropReadbackApi,
    HeptaWindowsDwmBackdropValue,
};
use super::hepta_windows_product_host_implementation::{
    HeptaWindowsProductHostActivationRequest, HeptaWindowsProductHostError,
    HeptaWindowsProductHostImplementationApproval, HeptaWindowsProductHostPhase,
    HeptaWindowsProductHostRuntimePreferences, HeptaWindowsProductHostRuntimeReceipt,
    HeptaWindowsProductHostWindowIdentity, HeptaWindowsProductMaterialBackend,
    HeptaWindowsProductMaterialBackendError, HeptaWindowsProductMaterialHost,
    HeptaWindowsVerifiedMaterialBinding, HeptaWindowsVerifiedMaterialRollback,
};
use super::hepta_windows_product_host_integration_review::{
    HEPTA_WINDOWS_REVIEW_MAKEPAD_REVISION, HeptaWindowsProductHostReviewReceipt,
    HeptaWindowsProductHostReviewStatus, HeptaWindowsReviewAuthorityBoundary,
};

#[cfg(target_os = "windows")]
use super::hepta_windows_material_adapter::HeptaWindowsDwmBackdropApi;

pub const HEPTA_WINDOWS_PRODUCT_WIRING_SOURCE_IMPLEMENTED: bool = true;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_FEATURE_DEFAULT_ENABLED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_AUTOMATIC_ACTIVATION: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_PRODUCT_CALLER_REGISTERED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_DEVICE_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_PRODUCT_BOUND: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_PRODUCT_WIRING_RELEASE: bool = false;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostBuildIdentity {
    pub commit: String,
    pub tree: String,
}

impl HeptaWindowsProductHostBuildIdentity {
    pub fn is_valid(&self) -> bool {
        git_object_id(&self.commit) && git_object_id(&self.tree)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostReviewedRuntimeIdentity {
    pub window: HeptaWindowsProductHostWindowIdentity,
    pub root_sequence: u64,
    pub acrylic_sequence: u64,
    pub rollback_sequence: u64,
    pub destroyed_acknowledged: bool,
}

impl HeptaWindowsProductHostReviewedRuntimeIdentity {
    pub const fn is_valid(self) -> bool {
        self.window.is_valid()
            && self.root_sequence == 1
            && self.acrylic_sequence == 2
            && self.rollback_sequence == 3
            && self.destroyed_acknowledged
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostReviewEnvelopeSeal {
    pub candidate_commit: String,
    pub candidate_tree: String,
    pub makepad_revision: String,
    pub binding_digest: String,
    pub source_run_id: u64,
    pub runtime_identity: HeptaWindowsProductHostReviewedRuntimeIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostDeviceQualificationSeal {
    pub implementation_commit: String,
    pub implementation_tree: String,
    pub review_binding_digest: String,
    pub operator_acceptance_digest: String,
    pub device_qualification_digest: String,
    pub feature_enabled: bool,
    pub physical_device_validated: bool,
    pub rollback_drill_validated: bool,
    pub high_contrast_fallback_validated: bool,
    pub transparency_disabled_fallback_validated: bool,
    pub suspend_rollback_validated: bool,
    pub shutdown_rollback_validated: bool,
    pub final_state_unbound: bool,
    pub runtime_identity: HeptaWindowsProductHostReviewedRuntimeIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeptaWindowsProductHostEvidenceSeal {
    build: HeptaWindowsProductHostBuildIdentity,
    review: HeptaWindowsProductHostReviewReceipt,
    approval: HeptaWindowsProductHostImplementationApproval,
    review_envelope: HeptaWindowsProductHostReviewEnvelopeSeal,
    device: HeptaWindowsProductHostDeviceQualificationSeal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsProductHostWiringError {
    BuildIdentityInvalid,
    ApprovalCandidateDrift,
    ReviewEnvelopeRejected,
    ReviewCandidateInvalid,
    ReviewMakepadRevisionDrift,
    ReviewDigestDrift,
    SourceRunInvalid,
    OperatorAcceptanceDigestInvalid,
    DeviceQualificationDigestInvalid,
    DeviceCandidateDrift,
    DeviceReviewDigestDrift,
    DeviceQualificationRejected,
    RuntimeIdentityDrift,
    WindowIdentityInvalid,
    AuthorityEscape,
    Host(HeptaWindowsProductHostError),
}

impl From<HeptaWindowsProductHostError> for HeptaWindowsProductHostWiringError {
    fn from(value: HeptaWindowsProductHostError) -> Self {
        Self::Host(value)
    }
}

impl HeptaWindowsProductHostEvidenceSeal {
    pub fn try_new(
        build: HeptaWindowsProductHostBuildIdentity,
        review: HeptaWindowsProductHostReviewReceipt,
        approval: HeptaWindowsProductHostImplementationApproval,
        review_envelope: HeptaWindowsProductHostReviewEnvelopeSeal,
        device: HeptaWindowsProductHostDeviceQualificationSeal,
    ) -> Result<Self, HeptaWindowsProductHostWiringError> {
        if !build.is_valid() {
            return Err(HeptaWindowsProductHostWiringError::BuildIdentityInvalid);
        }
        if approval.candidate_commit != build.commit || approval.candidate_tree != build.tree {
            return Err(HeptaWindowsProductHostWiringError::ApprovalCandidateDrift);
        }
        if !review.accepted
            || review.status != HeptaWindowsProductHostReviewStatus::EligibleForImplementationReview
            || !review.remains_review_only()
            || !review.grants_no_authority()
        {
            return Err(HeptaWindowsProductHostWiringError::ReviewEnvelopeRejected);
        }
        if !git_object_id(&review_envelope.candidate_commit)
            || !git_object_id(&review_envelope.candidate_tree)
        {
            return Err(HeptaWindowsProductHostWiringError::ReviewCandidateInvalid);
        }
        if review_envelope.makepad_revision != HEPTA_WINDOWS_REVIEW_MAKEPAD_REVISION {
            return Err(HeptaWindowsProductHostWiringError::ReviewMakepadRevisionDrift);
        }
        if !sha256_hex(&review_envelope.binding_digest)
            || approval.review_binding_digest != review_envelope.binding_digest
        {
            return Err(HeptaWindowsProductHostWiringError::ReviewDigestDrift);
        }
        if review_envelope.source_run_id == 0 {
            return Err(HeptaWindowsProductHostWiringError::SourceRunInvalid);
        }
        if !sha256_hex(&device.operator_acceptance_digest) {
            return Err(HeptaWindowsProductHostWiringError::OperatorAcceptanceDigestInvalid);
        }
        if !sha256_hex(&device.device_qualification_digest) {
            return Err(HeptaWindowsProductHostWiringError::DeviceQualificationDigestInvalid);
        }
        if device.implementation_commit != build.commit || device.implementation_tree != build.tree {
            return Err(HeptaWindowsProductHostWiringError::DeviceCandidateDrift);
        }
        if device.review_binding_digest != review_envelope.binding_digest {
            return Err(HeptaWindowsProductHostWiringError::DeviceReviewDigestDrift);
        }
        if !device.feature_enabled
            || !device.physical_device_validated
            || !device.rollback_drill_validated
            || !device.high_contrast_fallback_validated
            || !device.transparency_disabled_fallback_validated
            || !device.suspend_rollback_validated
            || !device.shutdown_rollback_validated
            || !device.final_state_unbound
        {
            return Err(HeptaWindowsProductHostWiringError::DeviceQualificationRejected);
        }
        if !review_envelope.runtime_identity.is_valid()
            || !device.runtime_identity.is_valid()
            || review_envelope.runtime_identity != device.runtime_identity
        {
            return Err(HeptaWindowsProductHostWiringError::RuntimeIdentityDrift);
        }
        if !review_envelope.runtime_identity.window.is_valid() {
            return Err(HeptaWindowsProductHostWiringError::WindowIdentityInvalid);
        }
        if !approval.authority.grants_none() {
            return Err(HeptaWindowsProductHostWiringError::AuthorityEscape);
        }

        Ok(Self {
            build,
            review,
            approval,
            review_envelope,
            device,
        })
    }

    pub fn build(&self) -> &HeptaWindowsProductHostBuildIdentity {
        &self.build
    }

    pub fn review_binding_digest(&self) -> &str {
        &self.review_envelope.binding_digest
    }

    pub fn source_run_id(&self) -> u64 {
        self.review_envelope.source_run_id
    }

    pub const fn runtime_identity(&self) -> HeptaWindowsProductHostReviewedRuntimeIdentity {
        self.review_envelope.runtime_identity
    }

    pub fn activation_request(
        &self,
        preferences: HeptaWindowsProductHostRuntimePreferences,
    ) -> HeptaWindowsProductHostActivationRequest {
        HeptaWindowsProductHostActivationRequest {
            review: self.review,
            approval: self.approval.clone(),
            identity: self.review_envelope.runtime_identity.window,
            preferences,
        }
    }
}
