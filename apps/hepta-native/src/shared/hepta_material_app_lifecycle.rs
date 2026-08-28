//! Root-window lifecycle bridge for Hepta UI v4 platform materials.
//!
//! The bridge receives application lifecycle events through the canonical
//! Makepad Window event tree and drives the consolidated material host. This
//! tranche deliberately uses the unbound adapter with zero host capabilities:
//! it can publish only `SemanticIntentOnly`, `SolidFallback`, `Suspended`, or
//! `Shutdown`. Real system material requires a later explicit host-object
//! adapter and runtime receipt.

use makepad_widgets::*;

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
pub const HEPTA_MATERIAL_APP_SYSTEM_ADAPTER_AVAILABLE: bool = false;
pub const HEPTA_MATERIAL_APP_WINDOW_HANDLE_BOUND: bool = false;
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
    // prototype here injects the lifecycle node without forking app.rs. No
    // system material is applied because the node owns only the unbound adapter.
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
}

impl HeptaV4MaterialLifecycleNode {
    fn log_result(
        &self,
        event: HeptaMaterialAppLifecycleEvent,
        result: Result<HeptaMaterialHostSnapshot, HeptaMaterialHostError>,
    ) {
        match result {
            Ok(snapshot) => log!(
                "Hepta material lifecycle {event:?}: phase={:?}, bound={}, generation={}, authority=false",
                snapshot.phase,
                snapshot.system_material_bound,
                snapshot.generation,
            ),
            Err(error) => error!(
                "Hepta material lifecycle {event:?} remained fail-closed: {error:?}"
            ),
        }
    }
}

impl Widget for HeptaV4MaterialLifecycleNode {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event {
            Event::Startup => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Startup;
                let result = self
                    .lifecycle
                    .activate(lifecycle_event, current_system_preferences());
                self.log_result(lifecycle_event, result);
            }
            Event::Resume => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Resume;
                let result = self
                    .lifecycle
                    .activate(lifecycle_event, current_system_preferences());
                self.log_result(lifecycle_event, result);
            }
            Event::Foreground => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Foreground;
                let result = self
                    .lifecycle
                    .activate(lifecycle_event, current_system_preferences());
                self.log_result(lifecycle_event, result);
            }
            Event::Pause => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Pause;
                let snapshot = self.lifecycle.suspend(lifecycle_event);
                self.log_result(lifecycle_event, Ok(snapshot));
            }
            Event::Background => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Background;
                let snapshot = self.lifecycle.suspend(lifecycle_event);
                self.log_result(lifecycle_event, Ok(snapshot));
            }
            Event::Shutdown => {
                let lifecycle_event = HeptaMaterialAppLifecycleEvent::Shutdown;
                let snapshot = self.lifecycle.shutdown();
                self.log_result(lifecycle_event, Ok(snapshot));
            }
            _ => {}
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
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
    fn unbound_app_lifecycle_never_claims_system_material_or_authority() {
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
    fn app_lifecycle_authority_constants_remain_false() {
        assert!(HEPTA_MATERIAL_APP_LIFECYCLE_SOURCE_WIRED);
        assert!(!HEPTA_MATERIAL_APP_SYSTEM_ADAPTER_AVAILABLE);
        assert!(!HEPTA_MATERIAL_APP_WINDOW_HANDLE_BOUND);
        assert!(!HEPTA_MATERIAL_APP_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_MATERIAL_APP_EFFECT_AUTHORITY);
        assert!(!HEPTA_MATERIAL_APP_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_MATERIAL_APP_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_MATERIAL_APP_PROMOTION);
        assert!(!HEPTA_MATERIAL_APP_RELEASE);
    }
}
