//! Root-window lifecycle bridge for Hepta UI v4 platform materials.
//!
//! The bridge receives application and exact-window events through the
//! canonical Makepad Window event tree. It drives two deliberately separate
//! layers:
//!
//! 1. the consolidated semantic material host, which still owns only the
//!    unbound full-profile adapter; and
//! 2. a Makepad public-API controller that may queue a persistent root-window
//!    Mica/Vibrancy request for the exact `WindowId` delivered by the framework.
//!
//! A queued `WindowVisuals` request is not an OS readback receipt and cannot
//! claim transient material, a complete profile, effect, or production
//! authority.

use makepad_widgets::*;

use super::hepta_makepad_window_material::{
    HeptaMakepadWindowMaterialController, HeptaMakepadWindowMaterialReceipt,
};
use super::hepta_platform_material_host::{
    HeptaMaterialHostCapabilities, HeptaMaterialHostError, HeptaMaterialHostSnapshot,
    HeptaPlatformMaterialHost,
};
use super::hepta_platform_material_runtime::{
    HeptaUnboundSystemMaterialAdapter, current_platform,
};
use super::hepta_system_preferences::{
    HeptaSystemPreferenceSnapshot, current_system_preferences,
};

pub const HEPTA_MATERIAL_APP_LIFECYCLE_SOURCE_WIRED: bool = true;
pub const HEPTA_MATERIAL_APP_FRAMEWORK_WINDOW_VISUALS_SOURCE_WIRED: bool = true;
pub const HEPTA_MATERIAL_APP_EXACT_WINDOW_ID_EVENT_BOUND_SOURCE: bool = true;
pub const HEPTA_MATERIAL_APP_PERSISTENT_CHROME_REQUEST_SOURCE: bool = true;
pub const HEPTA_MATERIAL_APP_SYSTEM_ADAPTER_AVAILABLE: bool = false;
pub const HEPTA_MATERIAL_APP_NATIVE_WINDOW_HANDLE_BOUND: bool = false;
pub const HEPTA_MATERIAL_APP_TRANSIENT_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_MATERIAL_APP_COMPLETE_PROFILE_BOUND: bool = false;
pub const HEPTA_MATERIAL_APP_RUNTIME_READBACK: bool = false;
pub const HEPTA_MATERIAL_APP_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_MATERIAL_APP_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_MATERIAL_APP_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_MATERIAL_APP_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_MATERIAL_APP_PROMOTION: bool = false;
pub const HEPTA_MATERIAL_APP_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMaterialAppLifecycleEvent {
    Startup,
    Resume,
    Foreground,
    Pause,
    Background,
    Shutdown,
}

pub struct HeptaMaterialAppLifecycle {
    host: HeptaPlatformMaterialHost,
    adapter: HeptaUnboundSystemMaterialAdapter,
    last_event: Option<HeptaMaterialAppLifecycleEvent>,
}

impl Default for HeptaMaterialAppLifecycle {
    fn default() -> Self {
        Self {
            host: HeptaPlatformMaterialHost::new(current_platform()),
            adapter: HeptaUnboundSystemMaterialAdapter,
            last_event: None,
        }
    }
}

impl HeptaMaterialAppLifecycle {
    pub const fn snapshot(&self) -> HeptaMaterialHostSnapshot {
        self.host.snapshot()
    }

    pub const fn last_event(&self) -> Option<HeptaMaterialAppLifecycleEvent> {
        self.last_event
    }

    pub fn activate(
        &mut self,
        event: HeptaMaterialAppLifecycleEvent,
        preferences: HeptaSystemPreferenceSnapshot,
    ) -> Result<HeptaMaterialHostSnapshot, HeptaMaterialHostError> {
        debug_assert!(matches!(
            event,
            HeptaMaterialAppLifecycleEvent::Startup
                | HeptaMaterialAppLifecycleEvent::Resume
                | HeptaMaterialAppLifecycleEvent::Foreground
        ));
        self.last_event = Some(event);
        self.host.apply_snapshot(
            &mut self.adapter,
            preferences,
            HeptaMaterialHostCapabilities::default(),
        )
    }

    pub fn suspend(
        &mut self,
        event: HeptaMaterialAppLifecycleEvent,
    ) -> HeptaMaterialHostSnapshot {
        debug_assert!(matches!(
            event,
            HeptaMaterialAppLifecycleEvent::Pause | HeptaMaterialAppLifecycleEvent::Background
        ));
        self.last_event = Some(event);
        self.host.suspend(&mut self.adapter)
    }

    pub fn shutdown(&mut self) -> HeptaMaterialHostSnapshot {
        self.last_event = Some(HeptaMaterialAppLifecycleEvent::Shutdown);
        self.host.shutdown(&mut self.adapter)
    }
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // Zero-size, non-interactive lifecycle node. It participates in the event
    // tree but contributes no layout, drawing, hit target, or accessibility node.
    mod.widgets.HeptaV4MaterialLifecycleNode = set_type_default() do #(HeptaV4MaterialLifecycleNode::register_widget(vm)) {
        ..mod.widgets.View
        width: 0
        height: 0
        visible: true
    }

    // The App creates its main Window after shared modules load. Rebinding the
    // prototype here injects the lifecycle node without forking app.rs.
    mod.widgets.HeptaV4LifecycleWindow = mod.widgets.Window {
        body +: {
            hepta_v4_material_lifecycle := mod.widgets.HeptaV4MaterialLifecycleNode {}
        }
    }
    mod.widgets.Window = mod.widgets.HeptaV4LifecycleWindow {}
}

#[derive(Script, Widget)]
pub struct HeptaV4MaterialLifecycleNode {
    #[deref]
    view: View,
    #[rust]
    lifecycle: HeptaMaterialAppLifecycle,
    #[rust]
    window_material: HeptaMakepadWindowMaterialController,
    #[rust]
    last_preferences: HeptaSystemPreferenceSnapshot,
    #[rust]
    window_focused: bool,
}

impl HeptaV4MaterialLifecycleNode {
    fn activate(
        &mut self,
        cx: &mut Cx,
        lifecycle_event: HeptaMaterialAppLifecycleEvent,
    ) {
        let preferences = current_system_preferences();
        self.last_preferences = preferences;
        let host_result = self.lifecycle.activate(lifecycle_event, preferences);
        self.log_host_result(lifecycle_event, host_result);

        let window_receipt = self.window_material.request_active(
            cx,
            current_platform(),
            preferences.preferences,
            self.window_focused,
        );
        self.log_window_receipt(lifecycle_event, window_receipt);
    }

    fn log_host_result(
        &self,
        event: HeptaMaterialAppLifecycleEvent,
        result: Result<HeptaMaterialHostSnapshot, HeptaMaterialHostError>,
    ) {
        match result {
            Ok(snapshot) => log!(
                "Hepta material host {event:?}: phase={:?}, complete_bound={}, generation={}, authority=false",
                snapshot.phase,
                snapshot.system_material_bound,
                snapshot.generation,
            ),
            Err(error) => error!(
                "Hepta material host {event:?} remained fail-closed: {error:?}"
            ),
        }
    }

    fn log_window_receipt(
        &self,
        event: HeptaMaterialAppLifecycleEvent,
        receipt: HeptaMakepadWindowMaterialReceipt,
    ) {
        log!(
            "Hepta framework window material {event:?}: phase={:?}, window={:?}:{:?}, request_queued={}, persistent_chrome_requested={}, transient_bound=false, complete_bound=false, readback=false, authority=false",
            receipt.phase,
            receipt.window_index,
            receipt.window_generation,
            receipt.framework_request_queued,
            receipt.persistent_chrome_requested,
        );
    }

    fn refresh_window_request(&mut self, cx: &mut Cx) {
        let receipt = self.window_material.request_active(
            cx,
            current_platform(),
            self.last_preferences.preferences,
            self.window_focused,
        );
        self.log_window_receipt(
            self.lifecycle
                .last_event()
                .unwrap_or(HeptaMaterialAppLifecycleEvent::Startup),
            receipt,
        );
    }
}

impl Widget for HeptaV4MaterialLifecycleNode {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event {
            Event::Startup => {
                self.activate(cx, HeptaMaterialAppLifecycleEvent::Startup);
            }
            Event::Resume => {
                self.activate(cx, HeptaMaterialAppLifecycleEvent::Resume);
            }
            Event::Foreground => {
                self.activate(cx, HeptaMaterialAppLifecycleEvent::Foreground);
            }
            Event::WindowGotFocus(window_id) => {
                let _ = self.window_material.observe_window(cx, *window_id);
                if self.window_material.owns_window(*window_id) {
                    self.window_focused = true;
                    self.refresh_window_request(cx);
                }
            }
            Event::WindowLostFocus(window_id) if self.window_material.owns_window(*window_id) => {
                self.window_focused = false;
                self.refresh_window_request(cx);
            }
            Event::WindowClosed(event) => {
                self.window_material.forget_window(event.window_id);
            }
            Event::Pause => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Pause;
                let snapshot = self.lifecycle.suspend(lifecycle_event);
                self.log_host_result(lifecycle_event, Ok(snapshot));
                let receipt = self.window_material.suspend(cx, current_platform());
                self.log_window_receipt(lifecycle_event, receipt);
            }
            Event::Background => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Background;
                let snapshot = self.lifecycle.suspend(lifecycle_event);
                self.log_host_result(lifecycle_event, Ok(snapshot));
                let receipt = self.window_material.suspend(cx, current_platform());
                self.log_window_receipt(lifecycle_event, receipt);
            }
            Event::Shutdown => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Shutdown;
                let snapshot = self.lifecycle.shutdown();
                self.log_host_result(lifecycle_event, Ok(snapshot));
                let receipt = self.window_material.shutdown(cx, current_platform());
                self.log_window_receipt(lifecycle_event, receipt);
            }
            _ => {}
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let step = self.view.draw_walk(cx, scope, walk);
        if step.is_done() {
            let area = self.view.area();
            if let Some(window_id) = cx.get_window_id_of(&area)
                && self.window_material.observe_window(cx, window_id)
            {
                self.refresh_window_request(cx);
            }
        }
        step
    }

    fn is_interactive(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::hepta_platform_material_host::HeptaMaterialHostPhase;

    #[test]
    fn unbound_full_profile_host_never_claims_system_material_or_authority() {
        let mut lifecycle = HeptaMaterialAppLifecycle::default();
        let active = lifecycle
            .activate(
                HeptaMaterialAppLifecycleEvent::Startup,
                HeptaSystemPreferenceSnapshot::fail_closed(),
            )
            .unwrap();
        assert!(!active.system_material_bound);
        assert!(active.grants_no_authority());
        assert!(matches!(
            active.phase,
            HeptaMaterialHostPhase::SolidFallback
                | HeptaMaterialHostPhase::SemanticIntentOnly
        ));

        let suspended = lifecycle.suspend(HeptaMaterialAppLifecycleEvent::Pause);
        assert_eq!(suspended.phase, HeptaMaterialHostPhase::Suspended);
        assert!(!suspended.system_material_bound);
        assert!(suspended.grants_no_authority());

        let shutdown = lifecycle.shutdown();
        assert_eq!(shutdown.phase, HeptaMaterialHostPhase::Shutdown);
        assert!(shutdown.grants_no_authority());
        assert_eq!(
            lifecycle.activate(
                HeptaMaterialAppLifecycleEvent::Resume,
                HeptaSystemPreferenceSnapshot::fail_closed(),
            ),
            Err(HeptaMaterialHostError::HostShutdown),
        );
    }

    #[test]
    fn app_lifecycle_claim_boundary_is_partial_and_fail_closed() {
        assert!(HEPTA_MATERIAL_APP_LIFECYCLE_SOURCE_WIRED);
        assert!(HEPTA_MATERIAL_APP_FRAMEWORK_WINDOW_VISUALS_SOURCE_WIRED);
        assert!(HEPTA_MATERIAL_APP_EXACT_WINDOW_ID_EVENT_BOUND_SOURCE);
        assert!(HEPTA_MATERIAL_APP_PERSISTENT_CHROME_REQUEST_SOURCE);
        assert!(!HEPTA_MATERIAL_APP_SYSTEM_ADAPTER_AVAILABLE);
        assert!(!HEPTA_MATERIAL_APP_NATIVE_WINDOW_HANDLE_BOUND);
        assert!(!HEPTA_MATERIAL_APP_TRANSIENT_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_MATERIAL_APP_COMPLETE_PROFILE_BOUND);
        assert!(!HEPTA_MATERIAL_APP_RUNTIME_READBACK);
        assert!(!HEPTA_MATERIAL_APP_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_MATERIAL_APP_EFFECT_AUTHORITY);
        assert!(!HEPTA_MATERIAL_APP_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_MATERIAL_APP_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_MATERIAL_APP_PROMOTION);
        assert!(!HEPTA_MATERIAL_APP_RELEASE);
    }
}
