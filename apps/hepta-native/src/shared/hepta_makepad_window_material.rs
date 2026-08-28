//! Makepad-owned root-window material request path for Hepta UI v4.
//!
//! This module intentionally uses Makepad's public `WindowId`, `WindowVisuals`,
//! `WindowBackdrop`, and `CxOsOp::SetWindowVisuals` APIs. It never reads or
//! guesses an HWND, NSWindow, UIView, Activity, or browser host object.
//!
//! A queued framework request is not an operating-system readback receipt. The
//! controller may request persistent root-window chrome, but it always keeps
//! transient-system-material, complete-profile, effect, and production claims
//! false until a later platform-specific verifier proves them.

use makepad_widgets::*;

use super::hepta_platform_material::HeptaPlatform;
use super::hepta_system_preferences::HeptaMaterialRuntimePreferences;

pub const HEPTA_MAKEPAD_WINDOW_VISUALS_SOURCE_WIRED: bool = true;
pub const HEPTA_MAKEPAD_WINDOW_ID_EVENT_BOUND_SOURCE: bool = true;
pub const HEPTA_MAKEPAD_PERSISTENT_CHROME_REQUEST_SOURCE: bool = true;
pub const HEPTA_MAKEPAD_TRANSIENT_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_MAKEPAD_COMPLETE_PROFILE_BOUND: bool = false;
pub const HEPTA_MAKEPAD_RUNTIME_READBACK: bool = false;
pub const HEPTA_MAKEPAD_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_MAKEPAD_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_MAKEPAD_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_MAKEPAD_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_MAKEPAD_PROMOTION: bool = false;
pub const HEPTA_MAKEPAD_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMakepadWindowMaterialPhase {
    WaitingForWindow,
    SolidRequested,
    PersistentChromeRequested,
    Unsupported,
    Suspended,
    Shutdown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaMakepadWindowMaterialReceipt {
    pub generation: u64,
    pub platform: HeptaPlatform,
    pub window_index: Option<usize>,
    pub window_generation: Option<u64>,
    pub phase: HeptaMakepadWindowMaterialPhase,
    pub requested_visuals: WindowVisuals,
    pub framework_state_updated: bool,
    pub framework_request_queued: bool,
    pub persistent_chrome_requested: bool,
    pub transient_system_material_bound: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub runtime_readback: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaMakepadWindowMaterialReceipt {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }

    pub const fn makes_no_complete_binding_claim(self) -> bool {
        !self.transient_system_material_bound
            && !self.complete_profile_bound
            && !self.system_material_bound
            && !self.runtime_readback
    }
}

#[derive(Debug)]
pub struct HeptaMakepadWindowMaterialController {
    generation: u64,
    window_id: Option<WindowId>,
    shutdown: bool,
    last_receipt: HeptaMakepadWindowMaterialReceipt,
}

impl Default for HeptaMakepadWindowMaterialController {
    fn default() -> Self {
        Self {
            generation: 0,
            window_id: None,
            shutdown: false,
            last_receipt: receipt(
                0,
                HeptaPlatform::Unknown,
                None,
                HeptaMakepadWindowMaterialPhase::WaitingForWindow,
                WindowVisuals::default(),
                false,
                false,
                false,
            ),
        }
    }
}

impl HeptaMakepadWindowMaterialController {
    pub const fn last_receipt(&self) -> HeptaMakepadWindowMaterialReceipt {
        self.last_receipt
    }

    pub const fn window_id(&self) -> Option<WindowId> {
        self.window_id
    }

    pub fn owns_window(&self, window_id: WindowId) -> bool {
        self.window_id == Some(window_id)
    }

    /// Binds the controller to the exact root `WindowId` delivered by Makepad.
    /// Popup windows are deliberately ignored because this tranche does not yet
    /// own a governed transient-system-material host.
    pub fn observe_window(&mut self, cx: &Cx, window_id: WindowId) -> bool {
        if self.shutdown || !cx.windows.is_valid(window_id) || cx.windows[window_id].is_popup {
            return false;
        }
        let changed = self.window_id != Some(window_id);
        self.window_id = Some(window_id);
        changed
    }

    pub fn forget_window(&mut self, window_id: WindowId) {
        if self.window_id == Some(window_id) {
            self.window_id = None;
        }
    }

    pub fn request_active(
        &mut self,
        cx: &mut Cx,
        platform: HeptaPlatform,
        preferences: HeptaMaterialRuntimePreferences,
        focused: bool,
    ) -> HeptaMakepadWindowMaterialReceipt {
        if self.shutdown {
            return self.install_receipt(
                platform,
                HeptaMakepadWindowMaterialPhase::Shutdown,
                WindowVisuals::default(),
                false,
                false,
                false,
            );
        }

        self.generation = self.generation.saturating_add(1);
        let Some(window_id) = self.valid_root_window(cx) else {
            self.window_id = None;
            return self.install_receipt(
                platform,
                HeptaMakepadWindowMaterialPhase::WaitingForWindow,
                WindowVisuals::default(),
                false,
                false,
                false,
            );
        };

        let (phase, visuals, persistent_chrome_requested) =
            desired_root_window_visuals(platform, preferences, focused);
        let (framework_state_updated, framework_request_queued) =
            queue_window_visuals(cx, window_id, visuals);

        self.install_receipt(
            platform,
            phase,
            visuals,
            framework_state_updated,
            framework_request_queued,
            persistent_chrome_requested,
        )
    }

    pub fn suspend(
        &mut self,
        cx: &mut Cx,
        platform: HeptaPlatform,
    ) -> HeptaMakepadWindowMaterialReceipt {
        self.generation = self.generation.saturating_add(1);
        let visuals = WindowVisuals::default();
        let (framework_state_updated, framework_request_queued) = self
            .valid_root_window(cx)
            .map(|window_id| queue_window_visuals(cx, window_id, visuals))
            .unwrap_or((false, false));
        self.install_receipt(
            platform,
            HeptaMakepadWindowMaterialPhase::Suspended,
            visuals,
            framework_state_updated,
            framework_request_queued,
            false,
        )
    }

    pub fn shutdown(
        &mut self,
        cx: &mut Cx,
        platform: HeptaPlatform,
    ) -> HeptaMakepadWindowMaterialReceipt {
        self.shutdown = true;
        self.generation = self.generation.saturating_add(1);
        let visuals = WindowVisuals::default();
        let (framework_state_updated, framework_request_queued) = self
            .valid_root_window(cx)
            .map(|window_id| queue_window_visuals(cx, window_id, visuals))
            .unwrap_or((false, false));
        self.install_receipt(
            platform,
            HeptaMakepadWindowMaterialPhase::Shutdown,
            visuals,
            framework_state_updated,
            framework_request_queued,
            false,
        )
    }

    fn valid_root_window(&self, cx: &Cx) -> Option<WindowId> {
        let window_id = self.window_id?;
        if cx.windows.is_valid(window_id) && !cx.windows[window_id].is_popup {
            Some(window_id)
        } else {
            None
        }
    }

    fn install_receipt(
        &mut self,
        platform: HeptaPlatform,
        phase: HeptaMakepadWindowMaterialPhase,
        requested_visuals: WindowVisuals,
        framework_state_updated: bool,
        framework_request_queued: bool,
        persistent_chrome_requested: bool,
    ) -> HeptaMakepadWindowMaterialReceipt {
        self.last_receipt = receipt(
            self.generation,
            platform,
            self.window_id,
            phase,
            requested_visuals,
            framework_state_updated,
            framework_request_queued,
            persistent_chrome_requested,
        );
        self.last_receipt
    }
}

pub fn desired_root_window_visuals(
    platform: HeptaPlatform,
    preferences: HeptaMaterialRuntimePreferences,
    focused: bool,
) -> (HeptaMakepadWindowMaterialPhase, WindowVisuals, bool) {
    if !preferences.transparency_allowed || preferences.high_contrast {
        return (
            HeptaMakepadWindowMaterialPhase::SolidRequested,
            WindowVisuals::default(),
            false,
        );
    }

    let (backdrop, active_intensity, inactive_intensity) = match platform {
        HeptaPlatform::Windows => (WindowBackdrop::Mica, 0.90, 0.82),
        HeptaPlatform::MacOs => (WindowBackdrop::Vibrancy, 0.88, 0.78),
        _ => {
            return (
                HeptaMakepadWindowMaterialPhase::Unsupported,
                WindowVisuals::default(),
                false,
            );
        }
    };

    (
        HeptaMakepadWindowMaterialPhase::PersistentChromeRequested,
        WindowVisuals {
            transparent: true,
            backdrop,
            backdrop_intensity: if focused {
                active_intensity
            } else {
                inactive_intensity
            },
        }
        .normalized(),
        true,
    )
}

fn queue_window_visuals(
    cx: &mut Cx,
    window_id: WindowId,
    visuals: WindowVisuals,
) -> (bool, bool) {
    let visuals = visuals.normalized();
    let (changed, created) = {
        let window = &mut cx.windows[window_id];
        if window.window_visuals() == visuals {
            (false, window.is_created)
        } else {
            window.transparent = visuals.transparent;
            window.backdrop = visuals.backdrop;
            window.backdrop_intensity = visuals.backdrop_intensity;
            (true, window.is_created)
        }
    };

    let queued = changed && created;
    if queued {
        cx.push_unique_platform_op(CxOsOp::SetWindowVisuals(window_id, visuals));
    }
    (changed, queued)
}

const fn receipt(
    generation: u64,
    platform: HeptaPlatform,
    window_id: Option<WindowId>,
    phase: HeptaMakepadWindowMaterialPhase,
    requested_visuals: WindowVisuals,
    framework_state_updated: bool,
    framework_request_queued: bool,
    persistent_chrome_requested: bool,
) -> HeptaMakepadWindowMaterialReceipt {
    HeptaMakepadWindowMaterialReceipt {
        generation,
        platform,
        window_index: match window_id {
            Some(window_id) => Some(window_id.0),
            None => None,
        },
        window_generation: match window_id {
            Some(window_id) => Some(window_id.1),
            None => None,
        },
        phase,
        requested_visuals,
        framework_state_updated,
        framework_request_queued,
        persistent_chrome_requested,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn transparent_preferences() -> HeptaMaterialRuntimePreferences {
        HeptaMaterialRuntimePreferences {
            transparency_allowed: true,
            high_contrast: false,
            reduced_motion: false,
            dynamic_color_available: false,
        }
    }

    fn test_window() -> (Cx, WindowId) {
        let mut cx = Cx::new(Box::new(|_, _| {}));
        let handle = WindowHandle::new(&mut cx);
        let window_id = handle.window_id();
        cx.windows[window_id].is_created = true;
        (cx, window_id)
    }

    #[test]
    fn windows_mica_request_is_event_bound_and_partial_only() {
        let (mut cx, window_id) = test_window();
        let mut controller = HeptaMakepadWindowMaterialController::default();
        assert!(controller.observe_window(&cx, window_id));

        let receipt = controller.request_active(
            &mut cx,
            HeptaPlatform::Windows,
            transparent_preferences(),
            true,
        );

        assert_eq!(
            receipt.phase,
            HeptaMakepadWindowMaterialPhase::PersistentChromeRequested
        );
        assert_eq!(receipt.requested_visuals.backdrop, WindowBackdrop::Mica);
        assert_eq!(cx.windows[window_id].backdrop, WindowBackdrop::Mica);
        assert!(receipt.framework_request_queued);
        assert!(receipt.persistent_chrome_requested);
        assert!(receipt.grants_no_authority());
        assert!(receipt.makes_no_complete_binding_claim());
    }

    #[test]
    fn high_contrast_and_suspend_restore_solid_visuals() {
        let (mut cx, window_id) = test_window();
        let mut controller = HeptaMakepadWindowMaterialController::default();
        assert!(controller.observe_window(&cx, window_id));
        controller.request_active(
            &mut cx,
            HeptaPlatform::Windows,
            transparent_preferences(),
            true,
        );

        let high_contrast = controller.request_active(
            &mut cx,
            HeptaPlatform::Windows,
            HeptaMaterialRuntimePreferences {
                high_contrast: true,
                ..transparent_preferences()
            },
            true,
        );
        assert_eq!(
            high_contrast.phase,
            HeptaMakepadWindowMaterialPhase::SolidRequested
        );
        assert_eq!(cx.windows[window_id].backdrop, WindowBackdrop::None);
        assert!(!cx.windows[window_id].transparent);

        let suspended = controller.suspend(&mut cx, HeptaPlatform::Windows);
        assert_eq!(suspended.phase, HeptaMakepadWindowMaterialPhase::Suspended);
        assert!(suspended.makes_no_complete_binding_claim());
    }

    #[test]
    fn unsupported_mobile_platforms_never_request_window_glass() {
        for platform in [HeptaPlatform::Ios, HeptaPlatform::Android] {
            let (phase, visuals, requested) =
                desired_root_window_visuals(platform, transparent_preferences(), true);
            assert_eq!(phase, HeptaMakepadWindowMaterialPhase::Unsupported);
            assert_eq!(visuals, WindowVisuals::default());
            assert!(!requested);
        }
    }

    #[test]
    fn macos_vibrancy_request_still_requires_runtime_readback() {
        let (phase, visuals, requested) = desired_root_window_visuals(
            HeptaPlatform::MacOs,
            transparent_preferences(),
            false,
        );
        assert_eq!(
            phase,
            HeptaMakepadWindowMaterialPhase::PersistentChromeRequested
        );
        assert_eq!(visuals.backdrop, WindowBackdrop::Vibrancy);
        assert!(requested);
        assert!(visuals.backdrop_intensity < 0.88);
    }

    #[test]
    fn source_authority_constants_remain_false() {
        assert!(HEPTA_MAKEPAD_WINDOW_VISUALS_SOURCE_WIRED);
        assert!(HEPTA_MAKEPAD_WINDOW_ID_EVENT_BOUND_SOURCE);
        assert!(HEPTA_MAKEPAD_PERSISTENT_CHROME_REQUEST_SOURCE);
        assert!(!HEPTA_MAKEPAD_TRANSIENT_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_MAKEPAD_COMPLETE_PROFILE_BOUND);
        assert!(!HEPTA_MAKEPAD_RUNTIME_READBACK);
        assert!(!HEPTA_MAKEPAD_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_MAKEPAD_EFFECT_AUTHORITY);
        assert!(!HEPTA_MAKEPAD_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_MAKEPAD_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_MAKEPAD_PROMOTION);
        assert!(!HEPTA_MAKEPAD_RELEASE);
    }
}
