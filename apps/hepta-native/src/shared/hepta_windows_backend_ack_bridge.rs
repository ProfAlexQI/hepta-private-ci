//! Fail-closed bridge between a future correlated Makepad Windows backend hook
//! and the Hepta UI v4 DWM acknowledgement producer.
//!
//! The pinned Makepad revision currently exposes `SetWindowVisuals(WindowId,
//! WindowVisuals)` and applies it inside the Windows backend, but it does not
//! expose a correlated post-apply callback carrying the exact request sequence
//! and HWND. This module therefore implements the complete Hepta-side state
//! machine while keeping the actual framework hook explicitly unbound.
//!
//! A future backend hook must deliver exactly one processed event for the
//! registered request, including the request sequence, full `WindowId`
//! index/generation, explicit HWND, normalized visuals, popup flag, and the DWM
//! set result. The bridge never searches for HWNDs and never treats backdrop
//! readback as full `WindowVisuals` readback or complete platform binding.

use makepad_widgets::WindowVisuals;

use super::hepta_platform_material::HeptaPlatform;
use super::hepta_window_visual_ack::{
    verify_window_visual_acknowledgement, HeptaWindowVisualAckReceipt,
    HeptaWindowVisualBackend, HeptaWindowVisualBackendObservation,
    HeptaWindowVisualReadback, HeptaWindowVisualRequestIdentity,
};
use super::hepta_windows_material_adapter::HeptaWindowsBackdropReadbackApi;
use super::hepta_windows_window_ack_producer::{
    HeptaWindowsDwmAckProducerError, HeptaWindowsDwmWindowAckProducer,
    HeptaWindowsDwmWindowBinding,
};

pub const HEPTA_WINDOWS_BACKEND_ACK_BRIDGE_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_BACKEND_ACK_CORRELATED_HOOK_CONTRACT_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_BACKEND_ACK_MAKEPAD_PATCH_APPLIED: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_HOOK_BOUND: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_EXPLICIT_HWND_FROM_FRAMEWORK: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_BACKDROP_READBACK_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_FULL_VISUAL_READBACK_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_TRANSIENT_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_COMPLETE_PROFILE_BOUND: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_NATIVE_PRODUCT_RUNTIME: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_DEVICE_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_BACKEND_ACK_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsBackendAckBridgePhase {
    Unbound,
    WindowBound,
    RequestPending,
    Acknowledged,
    Rejected,
    Invalidated,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsBackendWindowIdentity {
    pub hwnd: isize,
    pub window_index: usize,
    pub window_generation: u64,
}

impl HeptaWindowsBackendWindowIdentity {
    pub const fn new(
        hwnd: isize,
        window_index: usize,
        window_generation: u64,
    ) -> Result<Self, HeptaWindowsBackendAckBridgeError> {
        if hwnd == 0 {
            return Err(HeptaWindowsBackendAckBridgeError::InvalidHostHandle);
        }
        Ok(Self {
            hwnd,
            window_index,
            window_generation,
        })
    }

    pub const fn matches_request(self, request: HeptaWindowVisualRequestIdentity) -> bool {
        self.window_index == request.window_index()
            && self.window_generation == request.window_generation()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsBackendVisualsProcessed {
    pub request_sequence: u64,
    pub window_index: usize,
    pub window_generation: u64,
    pub hwnd: isize,
    pub visuals: WindowVisuals,
    pub backend_apply_succeeded: bool,
    pub is_popup: bool,
}

impl HeptaWindowsBackendVisualsProcessed {
    pub fn new(
        request_sequence: u64,
        window_index: usize,
        window_generation: u64,
        hwnd: isize,
        visuals: WindowVisuals,
        backend_apply_succeeded: bool,
        is_popup: bool,
    ) -> Result<Self, HeptaWindowsBackendAckBridgeError> {
        if hwnd == 0 {
            return Err(HeptaWindowsBackendAckBridgeError::InvalidHostHandle);
        }
        if is_popup {
            return Err(HeptaWindowsBackendAckBridgeError::PopupWindowRejected);
        }
        Ok(Self {
            request_sequence,
            window_index,
            window_generation,
            hwnd,
            visuals: visuals.normalized(),
            backend_apply_succeeded,
            is_popup,
        })
    }

    pub const fn identity(self) -> HeptaWindowsBackendWindowIdentity {
        HeptaWindowsBackendWindowIdentity {
            hwnd: self.hwnd,
            window_index: self.window_index,
            window_generation: self.window_generation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsBackendAckBridgeError {
    InvalidHostHandle,
    PopupWindowRejected,
    BridgeShutdown,
    UnsupportedPlatform,
    NoBoundWindow,
    WindowIdentityMismatch,
    StaleWindowGeneration,
    PendingRequestExists,
    NoPendingRequest,
    StaleRequestSequence,
    DispatchSequenceMismatch,
    DispatchVisualsMismatch,
    BackendObservation(HeptaWindowsDwmAckProducerError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsBackendAckBridgeSnapshot {
    pub phase: HeptaWindowsBackendAckBridgePhase,
    pub bound_window: Option<HeptaWindowsBackendWindowIdentity>,
    pub pending_request_sequence: Option<u64>,
    pub last_registered_sequence: u64,
    pub last_ack_accepted: Option<bool>,
    pub makepad_patch_applied: bool,
    pub hook_bound: bool,
    pub runtime_validated: bool,
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

pub struct HeptaWindowsBackendAckBridge {
    phase: HeptaWindowsBackendAckBridgePhase,
    binding: Option<HeptaWindowsBackendWindowIdentity>,
    pending: Option<HeptaWindowVisualRequestIdentity>,
    last_registered_sequence: u64,
    last_receipt: Option<HeptaWindowVisualAckReceipt>,
}

impl Default for HeptaWindowsBackendAckBridge {
    fn default() -> Self {
        Self {
            phase: HeptaWindowsBackendAckBridgePhase::Unbound,
            binding: None,
            pending: None,
            last_registered_sequence: 0,
            last_receipt: None,
        }
    }
}

impl HeptaWindowsBackendAckBridge {
    pub const fn phase(&self) -> HeptaWindowsBackendAckBridgePhase {
        self.phase
    }

    pub const fn bound_window(&self) -> Option<HeptaWindowsBackendWindowIdentity> {
        self.binding
    }

    pub const fn pending_request(&self) -> Option<HeptaWindowVisualRequestIdentity> {
        self.pending
    }

    pub const fn last_receipt(&self) -> Option<HeptaWindowVisualAckReceipt> {
        self.last_receipt
    }

    pub const fn snapshot(&self) -> HeptaWindowsBackendAckBridgeSnapshot {
        HeptaWindowsBackendAckBridgeSnapshot {
            phase: self.phase,
            bound_window: self.binding,
            pending_request_sequence: match self.pending {
                Some(request) => Some(request.request_sequence()),
                None => None,
            },
            last_registered_sequence: self.last_registered_sequence,
            last_ack_accepted: match self.last_receipt {
                Some(receipt) => Some(receipt.accepted),
                None => None,
            },
            makepad_patch_applied: false,
            hook_bound: false,
            runtime_validated: false,
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

    pub fn bind_window(
        &mut self,
        binding: HeptaWindowsBackendWindowIdentity,
    ) -> Result<(), HeptaWindowsBackendAckBridgeError> {
        self.ensure_active()?;
        if let Some(current) = self.binding {
            if current == binding {
                return Ok(());
            }
            if current.window_index == binding.window_index
                && binding.window_generation <= current.window_generation
            {
                return Err(HeptaWindowsBackendAckBridgeError::StaleWindowGeneration);
            }
            self.pending = None;
            self.last_receipt = None;
        }
        self.binding = Some(binding);
        self.phase = HeptaWindowsBackendAckBridgePhase::WindowBound;
        Ok(())
    }

    pub fn register_request(
        &mut self,
        request: HeptaWindowVisualRequestIdentity,
    ) -> Result<(), HeptaWindowsBackendAckBridgeError> {
        self.ensure_active()?;
        if request.platform() != HeptaPlatform::Windows {
            return Err(HeptaWindowsBackendAckBridgeError::UnsupportedPlatform);
        }
        let binding = self
            .binding
            .ok_or(HeptaWindowsBackendAckBridgeError::NoBoundWindow)?;
        if !binding.matches_request(request) {
            return Err(HeptaWindowsBackendAckBridgeError::WindowIdentityMismatch);
        }
        if self.pending.is_some() {
            return Err(HeptaWindowsBackendAckBridgeError::PendingRequestExists);
        }
        if request.request_sequence() <= self.last_registered_sequence {
            return Err(HeptaWindowsBackendAckBridgeError::StaleRequestSequence);
        }
        self.last_registered_sequence = request.request_sequence();
        self.pending = Some(request);
        self.last_receipt = None;
        self.phase = HeptaWindowsBackendAckBridgePhase::RequestPending;
        Ok(())
    }

    pub fn process_backend_event<A: HeptaWindowsBackdropReadbackApi>(
        &mut self,
        event: HeptaWindowsBackendVisualsProcessed,
        api: &mut A,
    ) -> Result<HeptaWindowVisualAckReceipt, HeptaWindowsBackendAckBridgeError> {
        self.ensure_active()?;
        let request = self
            .pending
            .ok_or(HeptaWindowsBackendAckBridgeError::NoPendingRequest)?;
        let binding = self
            .binding
            .ok_or(HeptaWindowsBackendAckBridgeError::NoBoundWindow)?;

        if event.request_sequence != request.request_sequence() {
            return Err(HeptaWindowsBackendAckBridgeError::DispatchSequenceMismatch);
        }
        if event.identity() != binding || !binding.matches_request(request) {
            return Err(HeptaWindowsBackendAckBridgeError::WindowIdentityMismatch);
        }
        if event.visuals.normalized() != request.requested_visuals().normalized() {
            return Err(HeptaWindowsBackendAckBridgeError::DispatchVisualsMismatch);
        }

        let receipt = if event.backend_apply_succeeded {
            let producer_binding = HeptaWindowsDwmWindowBinding::new(
                binding.hwnd,
                binding.window_index,
                binding.window_generation,
            )
            .map_err(HeptaWindowsBackendAckBridgeError::BackendObservation)?;
            let mut producer = HeptaWindowsDwmWindowAckProducer::new(
                producer_binding,
                BorrowedReadbackApi(api),
            );
            let observation = match producer.observe(request) {
                Ok(observation) => observation,
                Err(error) => {
                    self.pending = None;
                    self.last_receipt = None;
                    self.phase = HeptaWindowsBackendAckBridgePhase::Rejected;
                    return Err(HeptaWindowsBackendAckBridgeError::BackendObservation(error));
                }
            };
            verify_window_visual_acknowledgement(request, observation)
        } else {
            verify_window_visual_acknowledgement(
                request,
                HeptaWindowVisualBackendObservation {
                    request_sequence: request.request_sequence(),
                    platform: request.platform(),
                    window_index: request.window_index(),
                    window_generation: request.window_generation(),
                    backend: HeptaWindowVisualBackend::WindowsDwm,
                    attempted: true,
                    applied: false,
                    readback: HeptaWindowVisualReadback::None,
                },
            )
        };

        self.pending = None;
        self.phase = if receipt.accepted {
            HeptaWindowsBackendAckBridgePhase::Acknowledged
        } else {
            HeptaWindowsBackendAckBridgePhase::Rejected
        };
        self.last_receipt = Some(receipt);
        Ok(receipt)
    }

    /// Invalidates state only when the destruction event identifies the exact
    /// currently-bound window. A stale destroy event for an older generation
    /// must never detach a newer replacement window.
    pub fn invalidate_destroyed_window(
        &mut self,
        destroyed: HeptaWindowsBackendWindowIdentity,
    ) -> bool {
        if self.phase == HeptaWindowsBackendAckBridgePhase::Shutdown {
            return false;
        }
        if self.binding != Some(destroyed) {
            return false;
        }
        self.binding = None;
        self.pending = None;
        self.last_receipt = None;
        self.phase = HeptaWindowsBackendAckBridgePhase::Invalidated;
        true
    }

    pub fn shutdown(&mut self) {
        self.binding = None;
        self.pending = None;
        self.last_receipt = None;
        self.phase = HeptaWindowsBackendAckBridgePhase::Shutdown;
    }

    fn ensure_active(&self) -> Result<(), HeptaWindowsBackendAckBridgeError> {
        if self.phase == HeptaWindowsBackendAckBridgePhase::Shutdown {
            Err(HeptaWindowsBackendAckBridgeError::BridgeShutdown)
        } else {
            Ok(())
        }
    }
}

struct BorrowedReadbackApi<'a, A>(&'a mut A);

impl<A: HeptaWindowsBackdropReadbackApi> HeptaWindowsBackdropReadbackApi
    for BorrowedReadbackApi<'_, A>
{
    fn read_backdrop(
        &mut self,
        window: isize,
    ) -> Result<
        super::hepta_windows_material_adapter::HeptaWindowsDwmBackdropValue,
        super::hepta_windows_material_adapter::HeptaWindowsBackdropReadbackError,
    > {
        self.0.read_backdrop(window)
    }
}

#[cfg(test)]
mod tests {
    use makepad_widgets::{WindowBackdrop, WindowVisuals};

    use super::*;
    use crate::shared::hepta_makepad_window_material::{
        HeptaMakepadWindowMaterialPhase, HeptaMakepadWindowMaterialReceipt,
    };
    use crate::shared::hepta_window_visual_ack::HeptaWindowVisualAckStatus;
    use crate::shared::hepta_windows_material_adapter::{
        HeptaWindowsBackdropReadbackError, HeptaWindowsDwmBackdropValue,
    };

    struct RecordingApi {
        calls: Vec<isize>,
        result: Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError>,
    }

    impl HeptaWindowsBackdropReadbackApi for RecordingApi {
        fn read_backdrop(
            &mut self,
            window: isize,
        ) -> Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError> {
            self.calls.push(window);
            self.result
        }
    }

    fn request(sequence: u64, generation: u64) -> HeptaWindowVisualRequestIdentity {
        HeptaWindowVisualRequestIdentity::from_makepad_receipt(
            HeptaMakepadWindowMaterialReceipt {
                generation: sequence,
                platform: HeptaPlatform::Windows,
                window_index: Some(4),
                window_generation: Some(generation),
                phase: HeptaMakepadWindowMaterialPhase::PersistentChromeRequested,
                requested_visuals: WindowVisuals {
                    transparent: true,
                    backdrop: WindowBackdrop::Mica,
                    backdrop_intensity: 0.9,
                },
                framework_state_updated: true,
                framework_request_queued: true,
                persistent_chrome_requested: true,
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
            },
        )
        .unwrap()
    }

    fn binding(generation: u64) -> HeptaWindowsBackendWindowIdentity {
        HeptaWindowsBackendWindowIdentity::new(404, 4, generation).unwrap()
    }

    fn event(
        request: HeptaWindowVisualRequestIdentity,
        backend_apply_succeeded: bool,
    ) -> HeptaWindowsBackendVisualsProcessed {
        HeptaWindowsBackendVisualsProcessed::new(
            request.request_sequence(),
            request.window_index(),
            request.window_generation(),
            404,
            request.requested_visuals(),
            backend_apply_succeeded,
            false,
        )
        .unwrap()
    }

    fn api(result: HeptaWindowsDwmBackdropValue) -> RecordingApi {
        RecordingApi {
            calls: Vec::new(),
            result: Ok(result),
        }
    }

    #[test]
    fn exact_processed_event_produces_backdrop_only_acknowledgement() {
        let request = request(1, 8);
        let mut bridge = HeptaWindowsBackendAckBridge::default();
        bridge.bind_window(binding(8)).unwrap();
        bridge.register_request(request).unwrap();
        let mut api = api(HeptaWindowsDwmBackdropValue::Mica);

        let receipt = bridge
            .process_backend_event(event(request, true), &mut api)
            .unwrap();

        assert_eq!(
            receipt.status,
            HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback
        );
        assert!(receipt.accepted);
        assert!(receipt.backdrop_exact);
        assert!(!receipt.full_visuals_exact);
        assert!(receipt.remains_partial());
        assert!(receipt.grants_no_authority());
        assert_eq!(api.calls, vec![404]);
        assert_eq!(bridge.phase(), HeptaWindowsBackendAckBridgePhase::Acknowledged);
    }

    #[test]
    fn stale_sequence_and_visuals_are_rejected_before_dwm_readback() {
        let request = request(3, 8);
        let mut bridge = HeptaWindowsBackendAckBridge::default();
        bridge.bind_window(binding(8)).unwrap();
        bridge.register_request(request).unwrap();
        let mut api = api(HeptaWindowsDwmBackdropValue::Mica);
        let mut stale = event(request, true);
        stale.request_sequence += 1;
        assert_eq!(
            bridge.process_backend_event(stale, &mut api),
            Err(HeptaWindowsBackendAckBridgeError::DispatchSequenceMismatch)
        );
        assert!(api.calls.is_empty());

        let mut wrong_visuals = event(request, true);
        wrong_visuals.visuals.backdrop = WindowBackdrop::None;
        assert_eq!(
            bridge.process_backend_event(wrong_visuals, &mut api),
            Err(HeptaWindowsBackendAckBridgeError::DispatchVisualsMismatch)
        );
        assert!(api.calls.is_empty());
    }

    #[test]
    fn backend_apply_failure_is_a_rejected_ack_without_readback() {
        let request = request(5, 8);
        let mut bridge = HeptaWindowsBackendAckBridge::default();
        bridge.bind_window(binding(8)).unwrap();
        bridge.register_request(request).unwrap();
        let mut api = api(HeptaWindowsDwmBackdropValue::Mica);

        let receipt = bridge
            .process_backend_event(event(request, false), &mut api)
            .unwrap();

        assert_eq!(receipt.status, HeptaWindowVisualAckStatus::RejectedBackendFailure);
        assert!(!receipt.accepted);
        assert!(api.calls.is_empty());
        assert_eq!(bridge.phase(), HeptaWindowsBackendAckBridgePhase::Rejected);
    }

    #[test]
    fn readback_error_consumes_the_processed_request_and_stays_fail_closed() {
        let request = request(6, 8);
        let mut bridge = HeptaWindowsBackendAckBridge::default();
        bridge.bind_window(binding(8)).unwrap();
        bridge.register_request(request).unwrap();
        let mut api = RecordingApi {
            calls: Vec::new(),
            result: Err(HeptaWindowsBackdropReadbackError::SystemCallFailed(-5)),
        };

        let result = bridge.process_backend_event(event(request, true), &mut api);

        assert!(matches!(
            result,
            Err(HeptaWindowsBackendAckBridgeError::BackendObservation(
                HeptaWindowsDwmAckProducerError::Readback(
                    HeptaWindowsBackdropReadbackError::SystemCallFailed(-5)
                )
            ))
        ));
        assert_eq!(api.calls, vec![404]);
        assert!(bridge.pending_request().is_none());
        assert_eq!(bridge.phase(), HeptaWindowsBackendAckBridgePhase::Rejected);
        assert!(bridge.last_receipt().is_none());
    }

    #[test]
    fn generation_rebind_invalidates_pending_and_stale_destroy_cannot_detach_new_window() {
        let old_request = request(7, 8);
        let mut bridge = HeptaWindowsBackendAckBridge::default();
        bridge.bind_window(binding(8)).unwrap();
        bridge.register_request(old_request).unwrap();

        bridge.bind_window(binding(9)).unwrap();
        assert!(bridge.pending_request().is_none());
        assert_eq!(bridge.bound_window(), Some(binding(9)));
        assert!(!bridge.invalidate_destroyed_window(binding(8)));
        assert_eq!(bridge.bound_window(), Some(binding(9)));
        assert!(bridge.invalidate_destroyed_window(binding(9)));
        assert_eq!(bridge.phase(), HeptaWindowsBackendAckBridgePhase::Invalidated);
    }

    #[test]
    fn popup_zero_handle_and_nonmonotonic_registration_fail_closed() {
        assert_eq!(
            HeptaWindowsBackendWindowIdentity::new(0, 4, 8),
            Err(HeptaWindowsBackendAckBridgeError::InvalidHostHandle)
        );
        assert_eq!(
            HeptaWindowsBackendVisualsProcessed::new(
                1,
                4,
                8,
                404,
                WindowVisuals::default(),
                true,
                true,
            ),
            Err(HeptaWindowsBackendAckBridgeError::PopupWindowRejected)
        );

        let mut bridge = HeptaWindowsBackendAckBridge::default();
        bridge.bind_window(binding(8)).unwrap();
        let first = request(9, 8);
        bridge.register_request(first).unwrap();
        let mut api = api(HeptaWindowsDwmBackdropValue::Mica);
        bridge
            .process_backend_event(event(first, false), &mut api)
            .unwrap();
        assert_eq!(
            bridge.register_request(first),
            Err(HeptaWindowsBackendAckBridgeError::StaleRequestSequence)
        );
    }

    #[test]
    fn shutdown_is_terminal_and_snapshot_grants_no_authority() {
        let mut bridge = HeptaWindowsBackendAckBridge::default();
        bridge.bind_window(binding(8)).unwrap();
        bridge.shutdown();
        assert_eq!(
            bridge.bind_window(binding(9)),
            Err(HeptaWindowsBackendAckBridgeError::BridgeShutdown)
        );
        let snapshot = bridge.snapshot();
        assert_eq!(snapshot.phase, HeptaWindowsBackendAckBridgePhase::Shutdown);
        assert!(!snapshot.makepad_patch_applied);
        assert!(!snapshot.hook_bound);
        assert!(!snapshot.runtime_validated);
        assert!(!snapshot.transient_system_material_bound);
        assert!(!snapshot.complete_profile_bound);
        assert!(!snapshot.system_material_bound);
        assert!(!snapshot.production_authority);
        assert!(!snapshot.effect_authority);
        assert!(!snapshot.live_adapter_authority);
        assert!(!snapshot.operator_acceptance);
        assert!(!snapshot.promotion);
        assert!(!snapshot.release);
    }

    #[test]
    fn source_authority_constants_remain_false() {
        assert!(HEPTA_WINDOWS_BACKEND_ACK_BRIDGE_SOURCE_WIRED);
        assert!(HEPTA_WINDOWS_BACKEND_ACK_CORRELATED_HOOK_CONTRACT_SOURCE_WIRED);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_MAKEPAD_PATCH_APPLIED);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_HOOK_BOUND);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_EXPLICIT_HWND_FROM_FRAMEWORK);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_RUNTIME_VALIDATED);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_BACKDROP_READBACK_VALIDATED);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_FULL_VISUAL_READBACK_VALIDATED);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_TRANSIENT_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_COMPLETE_PROFILE_BOUND);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_NATIVE_PRODUCT_RUNTIME);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_DEVICE_VALIDATED);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_PROMOTION);
        assert!(!HEPTA_WINDOWS_BACKEND_ACK_RELEASE);
    }
}
