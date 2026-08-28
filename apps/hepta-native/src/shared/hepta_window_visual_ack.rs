//! Exact backend acknowledgement contract for Hepta UI v4 root-window visuals.
//!
//! Makepad's public `SetWindowVisuals` request proves only that framework state
//! changed and that a platform operation was queued. This module defines the
//! next evidence boundary: a backend observation must bind the exact request
//! sequence, platform, `WindowId` index and generation, and normalized visuals.
//! Even a verified persistent-chrome acknowledgement remains partial: transient
//! material, complete platform profile, product runtime, effect and production
//! authority stay false.

use makepad_widgets::WindowVisuals;

use super::hepta_makepad_window_material::HeptaMakepadWindowMaterialReceipt;
use super::hepta_platform_material::HeptaPlatform;

pub const HEPTA_WINDOW_VISUAL_ACK_CONTRACT_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOW_VISUAL_ACK_PRODUCER_BOUND: bool = false;
pub const HEPTA_WINDOW_VISUAL_ACK_RUNTIME_VALIDATED: bool = false;
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
pub enum HeptaWindowVisualAckStatus {
    VerifiedPersistentChromeWithReadback,
    VerifiedPersistentChromeWithoutReadback,
    VerifiedSolidFallbackWithReadback,
    VerifiedSolidFallbackWithoutReadback,
    RejectedNoQueuedRequest,
    RejectedRequestSequence,
    RejectedPlatform,
    RejectedWindowIdentity,
    RejectedBackend,
    RejectedBackendFailure,
    RejectedReadbackMissing,
    RejectedUnexpectedReadback,
    RejectedReadbackMismatch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowVisualRequestIdentity {
    pub request_sequence: u64,
    pub platform: HeptaPlatform,
    pub window_index: usize,
    pub window_generation: u64,
    pub requested_visuals: WindowVisuals,
    pub persistent_chrome_requested: bool,
}

impl HeptaWindowVisualRequestIdentity {
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
    pub readback_supported: bool,
    pub observed_visuals: Option<WindowVisuals>,
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
    pub observed_visuals: Option<WindowVisuals>,
    pub exact_readback: bool,
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
    } else if observation.readback_supported && observation.observed_visuals.is_none() {
        Some(HeptaWindowVisualAckStatus::RejectedReadbackMissing)
    } else if !observation.readback_supported && observation.observed_visuals.is_some() {
        Some(HeptaWindowVisualAckStatus::RejectedUnexpectedReadback)
    } else if observation
        .observed_visuals
        .map(WindowVisuals::normalized)
        .is_some_and(|visuals| visuals != request.requested_visuals.normalized())
    {
        Some(HeptaWindowVisualAckStatus::RejectedReadbackMismatch)
    } else {
        None
    };

    if let Some(status) = rejection {
        return receipt(request, observation, status, false, false);
    }

    let exact_readback = observation.readback_supported
        && observation
            .observed_visuals
            .map(WindowVisuals::normalized)
            == Some(request.requested_visuals.normalized());
    let status = match (request.persistent_chrome_requested, exact_readback) {
        (true, true) => HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithReadback,
        (true, false) => HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithoutReadback,
        (false, true) => HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithReadback,
        (false, false) => HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithoutReadback,
    };
    receipt(request, observation, status, true, exact_readback)
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
    exact_readback: bool,
) -> HeptaWindowVisualAckReceipt {
    HeptaWindowVisualAckReceipt {
        status,
        accepted,
        request_sequence: request.request_sequence,
        platform: request.platform,
        window_index: request.window_index,
        window_generation: request.window_generation,
        backend: observation.backend,
        requested_visuals: request.requested_visuals.normalized(),
        observed_visuals: observation.observed_visuals.map(WindowVisuals::normalized),
        exact_readback,
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
        readback_supported: bool,
    ) -> HeptaWindowVisualBackendObservation {
        HeptaWindowVisualBackendObservation {
            request_sequence: request.request_sequence,
            platform: request.platform,
            window_index: request.window_index,
            window_generation: request.window_generation,
            backend: HeptaWindowVisualBackend::WindowsDwm,
            attempted: true,
            applied: true,
            readback_supported,
            observed_visuals: readback_supported.then_some(request.requested_visuals),
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
        assert_eq!(request.request_sequence, 7);
        assert_eq!(request.window_index, 2);
        assert_eq!(request.window_generation, 11);
    }

    #[test]
    fn stale_sequence_and_window_generation_are_rejected() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, true))
                .unwrap();
        let mut stale = observation(request, true);
        stale.request_sequence += 1;
        assert_eq!(
            verify_window_visual_acknowledgement(request, stale).status,
            HeptaWindowVisualAckStatus::RejectedRequestSequence
        );
        let mut stale_window = observation(request, true);
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
        let mut wrong_backend = observation(request, true);
        wrong_backend.backend = HeptaWindowVisualBackend::MacosAppKit;
        assert_eq!(
            verify_window_visual_acknowledgement(request, wrong_backend).status,
            HeptaWindowVisualAckStatus::RejectedBackend
        );

        let mut missing = observation(request, true);
        missing.observed_visuals = None;
        assert_eq!(
            verify_window_visual_acknowledgement(request, missing).status,
            HeptaWindowVisualAckStatus::RejectedReadbackMissing
        );

        let mut mismatch = observation(request, true);
        mismatch.observed_visuals = Some(WindowVisuals::default());
        assert_eq!(
            verify_window_visual_acknowledgement(request, mismatch).status,
            HeptaWindowVisualAckStatus::RejectedReadbackMismatch
        );
    }

    #[test]
    fn verified_persistent_chrome_remains_partial_and_authority_free() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, true))
                .unwrap();
        let with_readback = verify_window_visual_acknowledgement(request, observation(request, true));
        assert_eq!(
            with_readback.status,
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithReadback
        );
        assert!(with_readback.accepted);
        assert!(with_readback.exact_readback);
        assert!(with_readback.persistent_chrome_acknowledged);
        assert!(with_readback.remains_partial());
        assert!(with_readback.grants_no_authority());

        let without_readback =
            verify_window_visual_acknowledgement(request, observation(request, false));
        assert_eq!(
            without_readback.status,
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithoutReadback
        );
        assert!(without_readback.accepted);
        assert!(!without_readback.exact_readback);
        assert!(without_readback.remains_partial());
    }

    #[test]
    fn solid_fallback_acknowledgement_is_not_a_material_binding_claim() {
        let request =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt(true, false))
                .unwrap();
        let receipt = verify_window_visual_acknowledgement(request, observation(request, true));
        assert_eq!(
            receipt.status,
            HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithReadback
        );
        assert!(receipt.solid_fallback_acknowledged);
        assert!(receipt.remains_partial());
        assert!(receipt.grants_no_authority());
    }

    #[test]
    fn acknowledgement_authority_constants_remain_false() {
        assert!(HEPTA_WINDOW_VISUAL_ACK_CONTRACT_SOURCE_WIRED);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_PRODUCER_BOUND);
        assert!(!HEPTA_WINDOW_VISUAL_ACK_RUNTIME_VALIDATED);
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
