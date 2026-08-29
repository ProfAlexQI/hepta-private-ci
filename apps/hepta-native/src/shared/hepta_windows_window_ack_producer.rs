//! Windows DWM backdrop-only acknowledgement producer for Hepta UI v4.
//!
//! The producer requires an explicit HWND plus the exact Makepad `WindowId`
//! index/generation that the host has independently bound. It performs no HWND
//! discovery and does not call `DwmSetWindowAttribute`; it reads the backdrop
//! after the exact queued Makepad request has been processed.
//!
//! `DWMWA_SYSTEMBACKDROP_TYPE` can prove only the system backdrop enum. It does
//! not prove Makepad transparency or intensity, transient Acrylic, a complete
//! platform profile, product runtime, or any effect/production authority.

use makepad_widgets::WindowBackdrop;

use super::hepta_platform_material::HeptaPlatform;
use super::hepta_window_visual_ack::{
    HeptaWindowVisualBackend, HeptaWindowVisualBackendObservation, HeptaWindowVisualReadback,
    HeptaWindowVisualRequestIdentity,
};
use super::hepta_windows_material_adapter::{
    HeptaWindowsBackdropReadbackApi, HeptaWindowsBackdropReadbackError,
    HeptaWindowsDwmBackdropValue,
};

pub const HEPTA_WINDOWS_DWM_ACK_PRODUCER_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_DWM_ACK_EXPLICIT_HWND_REQUIRED: bool = true;
pub const HEPTA_WINDOWS_DWM_ACK_HOST_BINDING_AVAILABLE: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_BACKDROP_READBACK_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_FULL_VISUAL_READBACK_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_TRANSIENT_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_COMPLETE_PROFILE_BOUND: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_DWM_ACK_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsDwmWindowBinding {
    pub hwnd: isize,
    pub window_index: usize,
    pub window_generation: u64,
}

impl HeptaWindowsDwmWindowBinding {
    pub const fn new(
        hwnd: isize,
        window_index: usize,
        window_generation: u64,
    ) -> Result<Self, HeptaWindowsDwmAckProducerError> {
        if hwnd == 0 {
            return Err(HeptaWindowsDwmAckProducerError::InvalidHostHandle);
        }
        Ok(Self {
            hwnd,
            window_index,
            window_generation,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsDwmAckProducerError {
    InvalidHostHandle,
    RequestPlatformMismatch,
    WindowIdentityMismatch,
    UnsupportedRequestedBackdrop(WindowBackdrop),
    UnsupportedObservedBackdrop(HeptaWindowsDwmBackdropValue),
    Readback(HeptaWindowsBackdropReadbackError),
}

pub struct HeptaWindowsDwmWindowAckProducer<A> {
    binding: HeptaWindowsDwmWindowBinding,
    api: A,
}

impl<A> HeptaWindowsDwmWindowAckProducer<A> {
    pub const fn new(binding: HeptaWindowsDwmWindowBinding, api: A) -> Self {
        Self { binding, api }
    }

    pub const fn binding(&self) -> HeptaWindowsDwmWindowBinding {
        self.binding
    }

    pub fn into_inner(self) -> A {
        self.api
    }
}

impl<A: HeptaWindowsBackdropReadbackApi> HeptaWindowsDwmWindowAckProducer<A> {
    pub fn observe(
        &mut self,
        request: HeptaWindowVisualRequestIdentity,
    ) -> Result<HeptaWindowVisualBackendObservation, HeptaWindowsDwmAckProducerError> {
        if request.platform() != HeptaPlatform::Windows {
            return Err(HeptaWindowsDwmAckProducerError::RequestPlatformMismatch);
        }
        if request.window_index() != self.binding.window_index
            || request.window_generation() != self.binding.window_generation
        {
            return Err(HeptaWindowsDwmAckProducerError::WindowIdentityMismatch);
        }

        validate_requested_root_backdrop(request)?;
        let observed_kind = self
            .api
            .read_backdrop(self.binding.hwnd)
            .map_err(HeptaWindowsDwmAckProducerError::Readback)?;
        let observed_backdrop = map_observed_backdrop(observed_kind)?;

        Ok(HeptaWindowVisualBackendObservation {
            request_sequence: request.request_sequence(),
            platform: request.platform(),
            window_index: request.window_index(),
            window_generation: request.window_generation(),
            backend: HeptaWindowVisualBackend::WindowsDwm,
            attempted: true,
            applied: true,
            readback: HeptaWindowVisualReadback::Backdrop(observed_backdrop),
        })
    }
}

fn validate_requested_root_backdrop(
    request: HeptaWindowVisualRequestIdentity,
) -> Result<(), HeptaWindowsDwmAckProducerError> {
    match (
        request.persistent_chrome_requested(),
        request.requested_visuals().backdrop,
    ) {
        (true, WindowBackdrop::Mica) | (false, WindowBackdrop::None) => Ok(()),
        (_, backdrop) => {
            Err(HeptaWindowsDwmAckProducerError::UnsupportedRequestedBackdrop(backdrop))
        }
    }
}

fn map_observed_backdrop(
    kind: HeptaWindowsDwmBackdropValue,
) -> Result<WindowBackdrop, HeptaWindowsDwmAckProducerError> {
    match kind {
        HeptaWindowsDwmBackdropValue::Auto => Ok(WindowBackdrop::Auto),
        HeptaWindowsDwmBackdropValue::None => Ok(WindowBackdrop::None),
        HeptaWindowsDwmBackdropValue::Mica => Ok(WindowBackdrop::Mica),
        HeptaWindowsDwmBackdropValue::Acrylic => Ok(WindowBackdrop::Acrylic),
        HeptaWindowsDwmBackdropValue::MicaAlt => {
            Err(HeptaWindowsDwmAckProducerError::UnsupportedObservedBackdrop(kind))
        }
    }
}

#[cfg(test)]
mod tests {
    use makepad_widgets::{WindowBackdrop, WindowVisuals};

    use super::*;
    use crate::shared::hepta_window_visual_ack::{
        HeptaWindowVisualAckStatus, HeptaWindowVisualReadbackScope,
        verify_window_visual_acknowledgement,
    };

    struct RecordingReadbackApi {
        calls: Vec<isize>,
        result: Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError>,
    }

    impl HeptaWindowsBackdropReadbackApi for RecordingReadbackApi {
        fn read_backdrop(
            &mut self,
            window: isize,
        ) -> Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError> {
            self.calls.push(window);
            self.result
        }
    }

    fn makepad_receipt_for_test(
        persistent: bool,
    ) -> crate::shared::hepta_makepad_window_material::HeptaMakepadWindowMaterialReceipt {
        use crate::shared::hepta_makepad_window_material::{
            HeptaMakepadWindowMaterialPhase, HeptaMakepadWindowMaterialReceipt,
        };

        let requested_visuals = if persistent {
            WindowVisuals {
                transparent: true,
                backdrop: WindowBackdrop::Mica,
                backdrop_intensity: 0.9,
            }
        } else {
            WindowVisuals::default()
        };
        HeptaMakepadWindowMaterialReceipt {
            generation: 17,
            platform: HeptaPlatform::Windows,
            window_index: Some(3),
            window_generation: Some(9),
            phase: if persistent {
                HeptaMakepadWindowMaterialPhase::PersistentChromeRequested
            } else {
                HeptaMakepadWindowMaterialPhase::SolidRequested
            },
            requested_visuals,
            framework_state_updated: true,
            framework_request_queued: true,
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

    fn request(persistent: bool) -> HeptaWindowVisualRequestIdentity {
        HeptaWindowVisualRequestIdentity::from_makepad_receipt(makepad_receipt_for_test(persistent))
            .unwrap()
    }

    fn producer(
        result: Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError>,
    ) -> HeptaWindowsDwmWindowAckProducer<RecordingReadbackApi> {
        HeptaWindowsDwmWindowAckProducer::new(
            HeptaWindowsDwmWindowBinding::new(101, 3, 9).unwrap(),
            RecordingReadbackApi {
                calls: Vec::new(),
                result,
            },
        )
    }

    #[test]
    fn producer_requires_explicit_nonzero_hwnd_and_exact_window_identity() {
        assert_eq!(
            HeptaWindowsDwmWindowBinding::new(0, 3, 9),
            Err(HeptaWindowsDwmAckProducerError::InvalidHostHandle)
        );
        let mut producer = producer(Ok(HeptaWindowsDwmBackdropValue::Mica));
        let stale_receipt =
            crate::shared::hepta_makepad_window_material::HeptaMakepadWindowMaterialReceipt {
                window_generation: Some(10),
                ..makepad_receipt_for_test(true)
            };
        let stale = HeptaWindowVisualRequestIdentity::from_makepad_receipt(stale_receipt).unwrap();
        assert_eq!(
            producer.observe(stale),
            Err(HeptaWindowsDwmAckProducerError::WindowIdentityMismatch)
        );
        assert!(producer.into_inner().calls.is_empty());
    }

    #[test]
    fn mica_readback_produces_backdrop_only_partial_acknowledgement() {
        let request = request(true);
        let mut producer = producer(Ok(HeptaWindowsDwmBackdropValue::Mica));
        let observation = producer.observe(request).unwrap();
        let receipt = verify_window_visual_acknowledgement(request, observation);
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
        assert!(receipt.remains_partial());
        assert!(receipt.grants_no_authority());
        assert_eq!(producer.into_inner().calls, vec![101]);
    }

    #[test]
    fn dwmsbt_none_proves_solid_fallback_not_auto() {
        let request = request(false);
        let mut producer = producer(Ok(HeptaWindowsDwmBackdropValue::None));
        let receipt =
            verify_window_visual_acknowledgement(request, producer.observe(request).unwrap());
        assert_eq!(
            receipt.status,
            HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithBackdropReadback
        );
        assert_eq!(receipt.observed_backdrop, Some(WindowBackdrop::None));
        assert!(receipt.solid_fallback_acknowledged);
        assert!(receipt.remains_partial());
    }

    #[test]
    fn auto_or_acrylic_readback_does_not_validate_a_mica_request() {
        for observed in [
            HeptaWindowsDwmBackdropValue::Auto,
            HeptaWindowsDwmBackdropValue::Acrylic,
        ] {
            let request = request(true);
            let mut producer = producer(Ok(observed));
            let receipt =
                verify_window_visual_acknowledgement(request, producer.observe(request).unwrap());
            assert_eq!(
                receipt.status,
                HeptaWindowVisualAckStatus::RejectedReadbackMismatch
            );
            assert!(!receipt.accepted);
        }
    }

    #[test]
    fn readback_errors_and_unsupported_values_fail_closed() {
        let mut failed = producer(Err(HeptaWindowsBackdropReadbackError::SystemCallFailed(-5)));
        assert_eq!(
            failed.observe(request(true)),
            Err(HeptaWindowsDwmAckProducerError::Readback(
                HeptaWindowsBackdropReadbackError::SystemCallFailed(-5)
            ))
        );

        let mut mica_alt = producer(Ok(HeptaWindowsDwmBackdropValue::MicaAlt));
        assert_eq!(
            mica_alt.observe(request(true)),
            Err(
                HeptaWindowsDwmAckProducerError::UnsupportedObservedBackdrop(
                    HeptaWindowsDwmBackdropValue::MicaAlt
                )
            )
        );
    }

    #[test]
    fn unsupported_request_profiles_are_rejected_before_readback() {
        let mut producer = producer(Ok(HeptaWindowsDwmBackdropValue::Mica));
        let unsupported_receipt =
            crate::shared::hepta_makepad_window_material::HeptaMakepadWindowMaterialReceipt {
                requested_visuals: WindowVisuals {
                    backdrop: WindowBackdrop::Acrylic,
                    ..makepad_receipt_for_test(true).requested_visuals
                },
                ..makepad_receipt_for_test(true)
            };
        let unsupported =
            HeptaWindowVisualRequestIdentity::from_makepad_receipt(unsupported_receipt).unwrap();
        assert_eq!(
            producer.observe(unsupported),
            Err(
                HeptaWindowsDwmAckProducerError::UnsupportedRequestedBackdrop(
                    WindowBackdrop::Acrylic
                )
            )
        );
        assert!(producer.into_inner().calls.is_empty());
    }

    #[test]
    fn producer_authority_constants_remain_false() {
        assert!(HEPTA_WINDOWS_DWM_ACK_PRODUCER_SOURCE_WIRED);
        assert!(HEPTA_WINDOWS_DWM_ACK_EXPLICIT_HWND_REQUIRED);
        assert!(!HEPTA_WINDOWS_DWM_ACK_HOST_BINDING_AVAILABLE);
        assert!(!HEPTA_WINDOWS_DWM_ACK_RUNTIME_VALIDATED);
        assert!(!HEPTA_WINDOWS_DWM_ACK_BACKDROP_READBACK_VALIDATED);
        assert!(!HEPTA_WINDOWS_DWM_ACK_FULL_VISUAL_READBACK_VALIDATED);
        assert!(!HEPTA_WINDOWS_DWM_ACK_TRANSIENT_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_DWM_ACK_COMPLETE_PROFILE_BOUND);
        assert!(!HEPTA_WINDOWS_DWM_ACK_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_DWM_ACK_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_DWM_ACK_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_DWM_ACK_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_WINDOWS_DWM_ACK_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_WINDOWS_DWM_ACK_PROMOTION);
        assert!(!HEPTA_WINDOWS_DWM_ACK_RELEASE);
    }
}
