//! Exact backend acknowledgement contract for Hepta UI v4 root-window visuals.
//!
//! Makepad's public `SetWindowVisuals` request proves only that framework state
//! changed and that a platform operation was queued. A backend observation must
//! bind the exact request sequence, platform, `WindowId` index and generation.
//!
//! Readback is explicitly scoped. Windows DWM can read back the system backdrop
//! type, but that does not prove Makepad transparency or intensity state. Even a
//! verified full-visual acknowledgement remains partial: transient material,
//! complete platform profile, product runtime, effect and production authority
//! stay false.

use makepad_widgets::{WindowBackdrop, WindowVisuals};

use super::hepta_makepad_window_material::HeptaMakepadWindowMaterialReceipt;
use super::hepta_platform_material::HeptaPlatform;

pub const HEPTA_WINDOW_VISUAL_ACK_CONTRACT_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOW_VISUAL_ACK_SCOPED_READBACK_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOW_VISUAL_ACK_PRODUCER_BOUND: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_FULL_VISUAL_READBACK_VALIDATED: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_TRANSIENT_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_COMPLETE_PROFILE_BOUND: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_PROMOTION: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowVisualBackend {
    WindowsDwm,
    MacosAppKit,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowVisualReadbackScope {
    None,
    BackdropOnly,
    FullVisuals,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeptaWindowVisualReadback {
    None,
    Backdrop(WindowBackdrop),
    Full(WindowVisuals),
}

impl HeptaWindowVisualReadback {
    pub const fn scope(self) -> HeptaWindowVisualReadbackScope {
        match self {
            Self::None => HeptaWindowVisualReadbackScope::None,
            Self::Backdrop(_) => HeptaWindowVisualReadbackScope::BackdropOnly,
            Self::Full(_) => HeptaWindowVisualReadbackScope::FullVisuals,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowVisualAckStatus {
    VerifiedPersistentChromeWithoutReadback,
    VerifiedPersistentChromeWithBackdropReadback,
    VerifiedPersistentChromeWithFullReadback,
    VerifiedSolidFallbackWithoutReadback,
    VerifiedSolidFallbackWithBackdropReadback,
    VerifiedSolidFallbackWithFullReadback,
    RejectedNoQueuedRequest,
    RejectedRequestSequence,
    RejectedPlatform,
    RejectedWindowIdentity,
    RejectedBackend,
    RejectedBackendFailure,
    RejectedReadbackMismatch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowVisualRequestIdentity {
    request_sequence: u64,
    platform: HeptaPlatform,
    window_index: usize,
    window_generation: u64,
    requested_visuals: WindowVisuals,
    persistent_chrome_requested: bool,
}

impl HeptaWindowVisualRequestIdentity {
    pub const fn request_sequence(self) -> u64 {
        self.request_sequence
    }

    pub const fn platform(self) -> HeptaPlatform {
        self.platform
    }

    pub const fn window_index(self) -> usize {
        self.window_index
    }

    pub const fn window_generation(self) -> u64 {
        self.window_generation
    }

    pub const fn requested_visuals(self) -> WindowVisuals {
        self.requested_visuals
    }

    pub const fn persistent_chrome_requested(self) -> bool {
        self.persistent_chrome_requested
    }

    /// Creates an acknowledgement identity only for an exact framework request
    /// that was actually queued. A deduplicated/no-op request cannot manufacture
    /// backend evidence.
    pub fn from_makepad_receipt(
        receipt: HeptaMakepadWindowMaterialReceipt,
    ) -> Result<Self, HeptaWindowVisualAckStatus> {
        if !receipt.framework_request_queued {
            return Err(HeptaWindowVisualAckStatus::RejectedNoQueuedRequest);
        }
        let (Some(window_index), Some(window_generation)) =
            (receipt.window_index, receipt.window_generation)
        else {
            return Err(HeptaWindowVisualAckStatus::RejectedWindowIdentity);
        };
        Ok(Self {
            request_sequence: receipt.generation,
            platform: receipt.platform,
            window_index,
            window_generation,
            requested_visuals: receipt.requested_visuals.normalized(),
            persistent_chrome_requested: receipt.persistent_chrome_requested,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowVisualBackendObservation {
    pub request_sequence: u64,
    pub platform: HeptaPlatform,
    pub window_index: usize,
    pub window_generation: u64,
    pub backend: HeptaWindowVisualBackend,
    pub attempted: bool,
    pub applied: bool,
    pub readback: HeptaWindowVisualReadback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowVisualAckReceipt {
    pub status: HeptaWindowVisualAckStatus,
    pub accepted: bool,
    pub request_sequence: u64,
    pub platform: HeptaPlatform,
    pub window_index: usize,
    pub window_generation: u64,
    pub backend: HeptaWindowVisualBackend,
    pub requested_visuals: WindowVisuals,
    pub readback_scope: HeptaWindowVisualReadbackScope,
    pub observed_backdrop: Option<WindowBackdrop>,
    pub observed_visuals: Option<WindowVisuals>,
    pub backdrop_exact: bool,
    pub full_visuals_exact: bool,
    pub persistent_chrome_acknowledged: bool,
    pub solid_fallback_acknowledged: bool,
    pub transient_system_material_bound: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowVisualAckReceipt {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }

    pub const fn remains_partial(self) -> bool {
        !self.transient_system_material_bound
            && !self.complete_profile_bound
            && !self.system_material_bound
    }
}

pub fn verify_window_visual_acknowledgement(
    request: HeptaWindowVisualRequestIdentity,
    observation: HeptaWindowVisualBackendObservation,
) -> HeptaWindowVisualAckReceipt {
    let rejection = if observation.request_sequence != request.request_sequence {
        Some(HeptaWindowVisualAckStatus::RejectedRequestSequence)
    } else if observation.platform != request.platform {
        Some(HeptaWindowVisualAckStatus::RejectedPlatform)
    } else if observation.window_index != request.window_index
        || observation.window_generation != request.window_generation
    {
        Some(HeptaWindowVisualAckStatus::RejectedWindowIdentity)
    } else if !backend_matches_platform(observation.backend, request.platform) {
        Some(HeptaWindowVisualAckStatus::RejectedBackend)
    } else if !observation.attempted || !observation.applied {
        Some(HeptaWindowVisualAckStatus::RejectedBackendFailure)
    } else if readback_mismatches(request.requested_visuals, observation.readback) {
        Some(HeptaWindowVisualAckStatus::RejectedReadbackMismatch)
    } else {
        None
    };

    if let Some(status) = rejection {
        return receipt(request, observation, status, false);
    }

    let status = match (
        request.persistent_chrome_requested,
        observation.readback.scope(),
    ) {
        (true, HeptaWindowVisualReadbackScope::None) => {
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithoutReadback
        }
        (true, HeptaWindowVisualReadbackScope::BackdropOnly) => {
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback
        }
        (true, HeptaWindowVisualReadbackScope::FullVisuals) => {
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithFullReadback
        }
        (false, HeptaWindowVisualReadbackScope::None) => {
            HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithoutReadback
        }
        (false, HeptaWindowVisualReadbackScope::BackdropOnly) => {
            HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithBackdropReadback
        }
        (false, HeptaWindowVisualReadbackScope::FullVisuals) => {
            HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithFullReadback
        }
    };
    receipt(request, observation, status, true)
}

fn readback_mismatches(
    requested_visuals: WindowVisuals,
    readback: HeptaWindowVisualReadback,
) -> bool {
    let requested_visuals = requested_visuals.normalized();
    match readback {
        HeptaWindowVisualReadback::None => false,
        HeptaWindowVisualReadback::Backdrop(backdrop) => {
            backdrop != requested_visuals.backdrop
        }
        HeptaWindowVisualReadback::Full(visuals) => {
            visuals.normalized() != requested_visuals
        }
    }
}

const fn backend_matches_platform(
    backend: HeptaWindowVisualBackend,
    platform: HeptaPlatform,
) -> bool {
    matches!(
        (backend, platform),
        (HeptaWindowVisualBackend::WindowsDwm, HeptaPlatform::Windows)
            | (HeptaWindowVisualBackend::MacosAppKit, HeptaPlatform::MacOs)
    )
}

fn receipt(
    request: HeptaWindowVisualRequestIdentity,
    observation: HeptaWindowVisualBackendObservation,
    status: HeptaWindowVisualAckStatus,
    accepted: bool,
) -> HeptaWindowVisualAckReceipt {
    let requested_visuals = request.requested_visuals.normalized();
    let (observed_backdrop, observed_visuals, backdrop_exact, full_visuals_exact) =
        match observation.readback {
            HeptaWindowVisualReadback::None => (None, None, false, false),
            HeptaWindowVisualReadback::Backdrop(backdrop) => (
                Some(backdrop),
                None,
                backdrop == requested_visuals.backdrop,
                false,
            ),
            HeptaWindowVisualReadback::Full(visuals) => {
                let visuals = visuals.normalized();
                (
                    Some(visuals.backdrop),
                    Some(visuals),
                    visuals.backdrop == requested_visuals.backdrop,
                    visuals == requested_visuals,
                )
            }
        };

    HeptaWindowVisualAckReceipt {
        status,
        accepted,
        request_sequence: request.request_sequence,
        platform: request.platform,
        window_index: request.window_index,
        window_generation: request.window_generation,
        backend: observation.backend,
        requested_visuals,
        readback_scope: observation.readback.scope(),
        observed_backdrop,
        observed_visuals,
        backdrop_exact,
        full_visuals_exact,
        persistent_chrome_acknowledged: accepted && request.persistent_chrome_requested,
        solid_fallback_acknowledged: accepted && !request.persistent_chrome_requested,
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

#[cfg(test)]
mod tests {
    use makepad_widgets::{WindowBackdrop, WindowVisuals};

    use super::*;
    use crate::shared::hepta_makepad_window_material::{
        HeptaMakepadWindowMaterialPhase, HeptaMakepadWindowMaterialReceipt,
    };

    fn makepad_receipt(queued: bool, persistent: bool) -> HeptaMakepadWindowMaterialReceipt {
        let visuals = if persistent {
            WindowVisuals {
                transparent: true,
                backdrop: WindowBackdrop::Mica,
                backdrop_intensity: 0.9,
            }
        } else {
            WindowVisuals::default()
        };
        HeptaMakepadWindowMaterialReceipt {
            generation: 7,
            platform: HeptaPlatform::Windows,
            window_index: Some(2),
            window_generation: Some(11),
            phase: if persistent {
                HeptaMakepadWindowMaterialPhase::PersistentChromeRequested
            } else {
                HeptaMakepadWindowMaterialPhase::SolidRequested
            },
            requested_visuals: visuals,
            framework_state_updated: queued,
            framework_request_queued: queued,
            persistent_chrome_requested: persistent,
            transient_system_material_bound: false,
            complete_profile_bound: false,
            system_material_bound: false,
            runtime_readback: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    fn observation(
        request: HeptaWindowVisualRequestIdentity,
        readback: HeptaWindowVisualReadback,
    ) -> HeptaWindowVisualBackendObservation {
        HeptaWindowVisualBackendObservation {
            request_sequence: request.request_sequence,
            platform: request.platform,
            window_index: request.window_index,
            window_generation: request.window_generation,
            backend: HeptaWindowVisualBackend::WindowsDwm,
            attempted: true,
            applied: true,
            readback,
        }
    }

    #[test]
    fn acknowledgement_identity_requires_a_queued_framework_request() {
        assert_eq!(
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(false, true)),
            Err(HeptaWindowVisualAckStatus::RejectedNoQueuedRequest)
        );
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, true))
                .unwrap();
        assert_eq!(request.request_sequence(), 7);
        assert_eq!(request.window_index(), 2);
        assert_eq!(request.window_generation(), 11);
    }

    #[test]
    fn stale_sequence_and_window_generation_are_rejected() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, true))
                .unwrap();
        let mut stale = observation(
            request,
            HeptaWindowVisualReadback::Backdrop(WindowBackdrop::Mica),
        );
        stale.request_sequence += 1;
        assert_eq!(
            verify_window_visual_acknowledgement(request, stale).status,
            HeptaWindowVisualAckStatus::RejectedRequestSequence
        );
        let mut stale_window = observation(
            request,
            HeptaWindowVisualReadback::Backdrop(WindowBackdrop::Mica),
        );
        stale_window.window_generation += 1;
        assert_eq!(
            verify_window_visual_acknowledgement(request, stale_window).status,
            HeptaWindowVisualAckStatus::RejectedWindowIdentity
        );
    }

    #[test]
    fn backend_and_readback_contracts_fail_closed() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, true))
                .unwrap();
        let mut wrong_backend = observation(
            request,
            HeptaWindowVisualReadback::Backdrop(WindowBackdrop::Mica),
        );
        wrong_backend.backend = HeptaWindowVisualBackend::MacosAppKit;
        assert_eq!(
            verify_window_visual_acknowledgement(request, wrong_backend).status,
            HeptaWindowVisualAckStatus::RejectedBackend
        );

        let mut failed = observation(request, HeptaWindowVisualReadback::None);
        failed.applied = false;
        assert_eq!(
            verify_window_visual_acknowledgement(request, failed).status,
            HeptaWindowVisualAckStatus::RejectedBackendFailure
        );

        let mismatch = observation(
            request,
            HeptaWindowVisualReadback::Backdrop(WindowBackdrop::None),
        );
        assert_eq!(
            verify_window_visual_acknowledgement(request, mismatch).status,
            HeptaWindowVisualAckStatus::RejectedReadbackMismatch
        );
    }

    #[test]
    fn backdrop_only_readback_is_partial_not_full_visual_readback() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, true))
                .unwrap();
        let receipt = verify_window_visual_acknowledgement(
            request,
            observation(
                request,
                HeptaWindowVisualReadback::Backdrop(WindowBackdrop::Mica),
            ),
        );
        assert_eq!(
            receipt.status,
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback
        );
        assert_eq!(
            receipt.readback_scope,
            HeptaWindowVisualReadbackScope::BackdropOnly
        );
        assert!(receipt.backdrop_exact);
        assert!(!receipt.full_visuals_exact);
        assert!(receipt.observed_visuals.is_none());
        assert!(receipt.remains_partial());
        assert!(receipt.grants_no_authority());
    }

    #[test]
    fn full_readback_requires_every_visual_field_to_match() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, true))
                .unwrap();
        let exact = verify_window_visual_acknowledgement(
            request,
            observation(request, HeptaWindowVisualReadback::Full(request.requested_visuals())),
        );
        assert_eq!(
            exact.status,
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithFullReadback
        );
        assert!(exact.full_visuals_exact);

        let mismatch = WindowVisuals {
            backdrop_intensity: 0.5,
            ..request.requested_visuals()
        };
        assert_eq!(
            verify_window_visual_acknowledgement(
                request,
                observation(request, HeptaWindowVisualReadback::Full(mismatch)),
            )
            .status,
            HeptaWindowVisualAckStatus::RejectedReadbackMismatch
        );
    }

    #[test]
    fn verified_persistent_chrome_without_readback_remains_partial() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, true))
                .unwrap();
        let receipt = verify_window_visual_acknowledgement(
            request,
            observation(request, HeptaWindowVisualReadback::None),
        );
        assert_eq!(
            receipt.status,
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithoutReadback
        );
        assert!(receipt.accepted);
        assert!(!receipt.backdrop_exact);
        assert!(!receipt.full_visuals_exact);
        assert!(receipt.persistent_chrome_acknowledged);
        assert!(receipt.remains_partial());
        assert!(receipt.grants_no_authority());
    }

    #[test]
    fn solid_fallback_backdrop_acknowledgement_is_not_a_material_binding_claim() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, false))
                .unwrap();
        let receipt = verify_window_visual_acknowledgement(
            request,
            observation(
                request,
                HeptaWindowVisualReadback::Backdrop(WindowBackdrop::None),
            ),
        );
        assert_eq!(
            receipt.status,
            HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithBackdropReadback
        );
        assert!(receipt.solid_fallback_acknowledged);
        assert!(receipt.remains_partial());
        assert!(receipt.grants_no_authority());
    }

    #[test]
    fn acknowledgement_authority_constants_remain_false() {
        assert!(HEPTA_WINDOW_VISUAL_ACK_CONTRACT_SOURCE_WIRED);
        assert!(HEPTA_WINDOW_VISUAL_ACK_SCOPED_READBACK_SOURCE_WIRED);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_PRODUCER_BOUND);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_RUNTIME_VALIDATED);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_FULL_VISUAL_READBACK_VALIDATED);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_TRANSIENT_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_COMPLETE_PROFILE_BOUND);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_PROMOTION);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_RELEASE);
    }
}
