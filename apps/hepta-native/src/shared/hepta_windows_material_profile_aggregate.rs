//! Fail-closed aggregation of Windows persistent-root and transient-window evidence.
//!
//! A successful aggregate means only that the evidence set is internally
//! consistent and ready for a separately governed product-integration review.
//! It never binds the product host and grants no system-material, effect, or
//! production authority.

use makepad_widgets::WindowBackdrop;

use super::hepta_platform_material::HeptaPlatform;
use super::hepta_window_visual_ack::{
    HeptaWindowVisualAckReceipt, HeptaWindowVisualAckStatus, HeptaWindowVisualBackend,
    HeptaWindowVisualReadbackScope,
};
use super::hepta_windows_backend_ack_bridge::HeptaWindowsBackendWindowIdentity;

pub const HEPTA_WINDOWS_PROFILE_AGGREGATE_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_PROFILE_AGGREGATE_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_PROFILE_PRODUCT_INTEGRATION_ELIGIBLE: bool = false;
pub const HEPTA_WINDOWS_PROFILE_PRODUCT_BOUND: bool = false;
pub const HEPTA_WINDOWS_PROFILE_TRANSIENT_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_PROFILE_COMPLETE_PROFILE_BOUND: bool = false;
pub const HEPTA_WINDOWS_PROFILE_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_PROFILE_NATIVE_PRODUCT_RUNTIME: bool = false;
pub const HEPTA_WINDOWS_PROFILE_DEVICE_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_PROFILE_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PROFILE_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PROFILE_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_PROFILE_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_PROFILE_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_PROFILE_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientEvidenceStatus {
    VerifiedAcrylicWithBackdropReadback,
    VerifiedSolidRollbackWithBackdropReadback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsTransientWindowIdentity {
    pub hwnd: isize,
    pub window_index: usize,
    pub window_generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsTransientReceipt {
    pub status: HeptaWindowsTransientEvidenceStatus,
    pub accepted: bool,
    pub request_sequence: u64,
    pub parent_window_index: usize,
    pub parent_window_generation: u64,
    pub transient: HeptaWindowsTransientWindowIdentity,
    pub requested_backdrop: WindowBackdrop,
    pub observed_backdrop: WindowBackdrop,
    pub backdrop_exact: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsTransientReceipt {
    pub const fn remains_partial(self) -> bool {
        !self.complete_profile_bound && !self.system_material_bound
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsTransientLifecycleEvidence {
    pub parent_hwnd: isize,
    pub parent_window_index: usize,
    pub parent_window_generation: u64,
    pub transient: HeptaWindowsTransientWindowIdentity,
    pub acrylic: HeptaWindowsTransientReceipt,
    pub solid_rollback: HeptaWindowsTransientReceipt,
    pub destroyed: HeptaWindowsTransientWindowIdentity,
    pub destroyed_acknowledged: bool,
    pub product_bound: bool,
    pub runtime_validated: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsTransientLifecycleEvidence {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsMaterialProfileAggregateStatus {
    ReadyForProductIntegrationReview,
    RejectedRootReceipt,
    RejectedRootIdentity,
    RejectedTransientParent,
    RejectedTransientIdentity,
    RejectedAcrylicReceipt,
    RejectedSolidRollbackReceipt,
    RejectedSequenceOrder,
    RejectedDestroyedEvidence,
    RejectedAuthorityEscape,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsMaterialProfileAggregateReceipt {
    pub status: HeptaWindowsMaterialProfileAggregateStatus,
    pub accepted: bool,
    pub root_window_index: usize,
    pub root_window_generation: u64,
    pub root_hwnd: isize,
    pub transient_window_index: usize,
    pub transient_window_generation: u64,
    pub transient_hwnd: isize,
    pub root_mica_acknowledged: bool,
    pub transient_acrylic_acknowledged: bool,
    pub transient_solid_rollback_acknowledged: bool,
    pub transient_destroyed_acknowledged: bool,
    pub eligible_for_product_integration_review: bool,
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

impl HeptaWindowsMaterialProfileAggregateReceipt {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }

    pub const fn remains_unbound(self) -> bool {
        !self.product_bound
            && !self.transient_system_material_bound
            && !self.complete_profile_bound
            && !self.system_material_bound
            && !self.native_product_runtime
            && !self.device_validated
    }
}

pub fn aggregate_windows_material_profile(
    root_identity: HeptaWindowsBackendWindowIdentity,
    root: HeptaWindowVisualAckReceipt,
    transient: HeptaWindowsTransientLifecycleEvidence,
) -> HeptaWindowsMaterialProfileAggregateReceipt {
    let status = validate(root_identity, root, transient);
    let accepted =
        status == HeptaWindowsMaterialProfileAggregateStatus::ReadyForProductIntegrationReview;
    HeptaWindowsMaterialProfileAggregateReceipt {
        status,
        accepted,
        root_window_index: root_identity.window_index,
        root_window_generation: root_identity.window_generation,
        root_hwnd: root_identity.hwnd,
        transient_window_index: transient.transient.window_index,
        transient_window_generation: transient.transient.window_generation,
        transient_hwnd: transient.transient.hwnd,
        root_mica_acknowledged: accepted,
        transient_acrylic_acknowledged: accepted,
        transient_solid_rollback_acknowledged: accepted,
        transient_destroyed_acknowledged: accepted,
        eligible_for_product_integration_review: accepted,
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
    root_identity: HeptaWindowsBackendWindowIdentity,
    root: HeptaWindowVisualAckReceipt,
    transient: HeptaWindowsTransientLifecycleEvidence,
) -> HeptaWindowsMaterialProfileAggregateStatus {
    if root_identity.hwnd == 0
        || !root.accepted
        || root.platform != HeptaPlatform::Windows
        || root.backend != HeptaWindowVisualBackend::WindowsDwm
        || root.status != HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback
        || root.readback_scope != HeptaWindowVisualReadbackScope::BackdropOnly
        || root.requested_visuals.backdrop != WindowBackdrop::Mica
        || root.observed_backdrop != Some(WindowBackdrop::Mica)
        || !root.backdrop_exact
        || !root.persistent_chrome_acknowledged
        || !root.remains_partial()
    {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedRootReceipt;
    }
    if root.window_index != root_identity.window_index
        || root.window_generation != root_identity.window_generation
    {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedRootIdentity;
    }
    if transient.parent_hwnd != root_identity.hwnd
        || transient.parent_window_index != root_identity.window_index
        || transient.parent_window_generation != root_identity.window_generation
    {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedTransientParent;
    }
    if transient.transient.hwnd == 0
        || transient.transient.hwnd == root_identity.hwnd
        || transient.destroyed != transient.transient
    {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedTransientIdentity;
    }
    if !valid_transient_receipt(
        transient.acrylic,
        transient,
        HeptaWindowsTransientEvidenceStatus::VerifiedAcrylicWithBackdropReadback,
        WindowBackdrop::Acrylic,
    ) {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedAcrylicReceipt;
    }
    if !valid_transient_receipt(
        transient.solid_rollback,
        transient,
        HeptaWindowsTransientEvidenceStatus::VerifiedSolidRollbackWithBackdropReadback,
        WindowBackdrop::None,
    ) {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedSolidRollbackReceipt;
    }
    if transient.acrylic.request_sequence >= transient.solid_rollback.request_sequence {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedSequenceOrder;
    }
    if !transient.destroyed_acknowledged {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedDestroyedEvidence;
    }
    if !root.grants_no_authority()
        || !transient.acrylic.grants_no_authority()
        || !transient.solid_rollback.grants_no_authority()
        || !transient.grants_no_authority()
        || transient.product_bound
        || transient.runtime_validated
        || transient.complete_profile_bound
        || transient.system_material_bound
    {
        return HeptaWindowsMaterialProfileAggregateStatus::RejectedAuthorityEscape;
    }
    HeptaWindowsMaterialProfileAggregateStatus::ReadyForProductIntegrationReview
}

fn valid_transient_receipt(
    receipt: HeptaWindowsTransientReceipt,
    evidence: HeptaWindowsTransientLifecycleEvidence,
    status: HeptaWindowsTransientEvidenceStatus,
    backdrop: WindowBackdrop,
) -> bool {
    receipt.accepted
        && receipt.status == status
        && receipt.parent_window_index == evidence.parent_window_index
        && receipt.parent_window_generation == evidence.parent_window_generation
        && receipt.transient == evidence.transient
        && receipt.requested_backdrop == backdrop
        && receipt.observed_backdrop == backdrop
        && receipt.backdrop_exact
        && receipt.remains_partial()
}

#[cfg(test)]
mod tests {
    use makepad_widgets::{WindowBackdrop, WindowVisuals};

    use super::*;

    fn root_identity() -> HeptaWindowsBackendWindowIdentity {
        HeptaWindowsBackendWindowIdentity {
            hwnd: 11,
            window_index: 1,
            window_generation: 7,
        }
    }

    fn root_receipt() -> HeptaWindowVisualAckReceipt {
        HeptaWindowVisualAckReceipt {
            status: HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback,
            accepted: true,
            request_sequence: 1,
            platform: HeptaPlatform::Windows,
            window_index: 1,
            window_generation: 7,
            backend: HeptaWindowVisualBackend::WindowsDwm,
            requested_visuals: WindowVisuals {
                transparent: true,
                backdrop: WindowBackdrop::Mica,
                backdrop_intensity: 0.9,
            },
            readback_scope: HeptaWindowVisualReadbackScope::BackdropOnly,
            observed_backdrop: Some(WindowBackdrop::Mica),
            observed_visuals: None,
            backdrop_exact: true,
            full_visuals_exact: false,
            persistent_chrome_acknowledged: true,
            solid_fallback_acknowledged: false,
            transient_system_material_bound: false,
            complete_profile_bound: false,
            system_material_bound: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    fn transient_receipt(
        sequence: u64,
        status: HeptaWindowsTransientEvidenceStatus,
        backdrop: WindowBackdrop,
    ) -> HeptaWindowsTransientReceipt {
        HeptaWindowsTransientReceipt {
            status,
            accepted: true,
            request_sequence: sequence,
            parent_window_index: 1,
            parent_window_generation: 7,
            transient: HeptaWindowsTransientWindowIdentity {
                hwnd: 12,
                window_index: 2,
                window_generation: 9,
            },
            requested_backdrop: backdrop,
            observed_backdrop: backdrop,
            backdrop_exact: true,
            complete_profile_bound: false,
            system_material_bound: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    fn evidence() -> HeptaWindowsTransientLifecycleEvidence {
        let transient = HeptaWindowsTransientWindowIdentity {
            hwnd: 12,
            window_index: 2,
            window_generation: 9,
        };
        HeptaWindowsTransientLifecycleEvidence {
            parent_hwnd: 11,
            parent_window_index: 1,
            parent_window_generation: 7,
            transient,
            acrylic: transient_receipt(
                2,
                HeptaWindowsTransientEvidenceStatus::VerifiedAcrylicWithBackdropReadback,
                WindowBackdrop::Acrylic,
            ),
            solid_rollback: transient_receipt(
                3,
                HeptaWindowsTransientEvidenceStatus::VerifiedSolidRollbackWithBackdropReadback,
                WindowBackdrop::None,
            ),
            destroyed: transient,
            destroyed_acknowledged: true,
            product_bound: false,
            runtime_validated: false,
            complete_profile_bound: false,
            system_material_bound: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    #[test]
    fn exact_dual_receipt_set_is_review_eligible_but_unbound() {
        let receipt =
            aggregate_windows_material_profile(root_identity(), root_receipt(), evidence());
        assert_eq!(
            receipt.status,
            HeptaWindowsMaterialProfileAggregateStatus::ReadyForProductIntegrationReview
        );
        assert!(receipt.accepted);
        assert!(receipt.eligible_for_product_integration_review);
        assert!(receipt.grants_no_authority());
        assert!(receipt.remains_unbound());
    }

    #[test]
    fn reused_root_hwnd_or_missing_destroy_is_rejected() {
        let mut value = evidence();
        value.transient.hwnd = value.parent_hwnd;
        assert_eq!(
            aggregate_windows_material_profile(root_identity(), root_receipt(), value).status,
            HeptaWindowsMaterialProfileAggregateStatus::RejectedTransientIdentity
        );
        let mut value = evidence();
        value.destroyed_acknowledged = false;
        assert_eq!(
            aggregate_windows_material_profile(root_identity(), root_receipt(), value).status,
            HeptaWindowsMaterialProfileAggregateStatus::RejectedDestroyedEvidence
        );
    }

    #[test]
    fn sequence_and_authority_escape_are_rejected() {
        let mut value = evidence();
        value.solid_rollback.request_sequence = value.acrylic.request_sequence;
        assert_eq!(
            aggregate_windows_material_profile(root_identity(), root_receipt(), value).status,
            HeptaWindowsMaterialProfileAggregateStatus::RejectedSequenceOrder
        );
        let mut value = evidence();
        value.production_authority = true;
        assert_eq!(
            aggregate_windows_material_profile(root_identity(), root_receipt(), value).status,
            HeptaWindowsMaterialProfileAggregateStatus::RejectedAuthorityEscape
        );
    }

    #[test]
    fn source_authority_constants_remain_false() {
        assert!(HEPTA_WINDOWS_PROFILE_AGGREGATE_SOURCE_WIRED);
        assert!(!HEPTA_WINDOWS_PROFILE_AGGREGATE_RUNTIME_VALIDATED);
        assert!(!HEPTA_WINDOWS_PROFILE_PRODUCT_INTEGRATION_ELIGIBLE);
        assert!(!HEPTA_WINDOWS_PROFILE_PRODUCT_BOUND);
        assert!(!HEPTA_WINDOWS_PROFILE_TRANSIENT_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_PROFILE_COMPLETE_PROFILE_BOUND);
        assert!(!HEPTA_WINDOWS_PROFILE_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_PROFILE_NATIVE_PRODUCT_RUNTIME);
        assert!(!HEPTA_WINDOWS_PROFILE_DEVICE_VALIDATED);
        assert!(!HEPTA_WINDOWS_PROFILE_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PROFILE_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PROFILE_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PROFILE_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_WINDOWS_PROFILE_PROMOTION);
        assert!(!HEPTA_WINDOWS_PROFILE_RELEASE);
    }
}
