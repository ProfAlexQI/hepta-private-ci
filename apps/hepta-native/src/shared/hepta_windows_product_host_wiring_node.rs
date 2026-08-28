//! Product-lifecycle wiring node for the default-off Windows material host.
//!
//! The node is registered only when the product wiring patch enables the
//! `hepta_ui_windows_system_material_v4` Cargo feature. It never activates on
//! startup, resume, draw, focus, or any other implicit event. An explicit caller
//! must provide a sealed evidence bundle and exact window identities.

use makepad_widgets::ScriptVm;

#[cfg(target_os = "windows")]
mod windows {
    use makepad_widgets::*;

    use super::super::hepta_system_preferences::current_system_preferences;
    use super::super::hepta_windows_product_host_implementation::{
        HeptaWindowsProductHostRuntimePreferences, HeptaWindowsProductHostRuntimeReceipt,
    };
    use super::super::hepta_windows_product_host_wiring::{
        HeptaWindowsDefaultProductHostCoordinator, HeptaWindowsProductHostEvidenceSeal,
        HeptaWindowsProductHostWiringError,
    };

    script_mod! {
        use mod.prelude.widgets.*
        use mod.widgets.*

        mod.widgets.HeptaV4WindowsProductHostWiringNode = set_type_default() do #(
            HeptaV4WindowsProductHostWiringNode::register_widget(vm)
        ) {
            ..mod.widgets.View
            width: 0
            height: 0
            visible: true
        }

        mod.widgets.HeptaV4WindowsProductHostWindow = mod.widgets.Window {
            body +: {
                hepta_v4_windows_product_host_wiring :=
                    mod.widgets.HeptaV4WindowsProductHostWiringNode {}
            }
        }

        mod.widgets.Window = mod.widgets.HeptaV4WindowsProductHostWindow {}
    }

    #[derive(Script, Widget)]
    pub struct HeptaV4WindowsProductHostWiringNode {
        #[deref]
        view: View,
        #[rust]
        coordinator: HeptaWindowsDefaultProductHostCoordinator,
        #[rust]
        last_receipt: Option<HeptaWindowsProductHostRuntimeReceipt>,
    }

    impl HeptaV4WindowsProductHostWiringNode {
        pub fn activate_explicit(
            &mut self,
            seal: &HeptaWindowsProductHostEvidenceSeal,
        ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostWiringError>
        {
            let preferences = current_system_preferences().preferences;
            let receipt = self.coordinator.activate_explicit(
                seal,
                HeptaWindowsProductHostRuntimePreferences {
                    transparency_allowed: preferences.transparency_allowed,
                    high_contrast: preferences.high_contrast,
                },
            )?;
            self.last_receipt = Some(receipt);
            Ok(receipt)
        }

        pub fn last_receipt(&self) -> Option<HeptaWindowsProductHostRuntimeReceipt> {
            self.last_receipt
        }

        fn reconcile_preferences(&mut self) {
            let preferences = current_system_preferences().preferences;
            match self.coordinator.reconcile_preferences(
                HeptaWindowsProductHostRuntimePreferences {
                    transparency_allowed: preferences.transparency_allowed,
                    high_contrast: preferences.high_contrast,
                },
            ) {
                Ok(Some(receipt)) => {
                    self.last_receipt = Some(receipt);
                    log!(
                        "Hepta Windows product material reconciled to {:?}; authority=false",
                        receipt.phase
                    );
                }
                Ok(None) => {}
                Err(error) => {
                    error!(
                        "Hepta Windows product material preference reconciliation failed closed: {:?}",
                        error
                    );
                }
            }
        }

        fn suspend(&mut self) {
            match self.coordinator.suspend() {
                Ok(receipt) => {
                    self.last_receipt = Some(receipt);
                    log!(
                        "Hepta Windows product material suspended at {:?}; authority=false",
                        receipt.phase
                    );
                }
                Err(error) => {
                    error!(
                        "Hepta Windows product material suspend failed closed: {:?}",
                        error
                    );
                }
            }
        }

        fn shutdown(&mut self) {
            match self.coordinator.shutdown() {
                Ok(receipt) => {
                    self.last_receipt = Some(receipt);
                    log!(
                        "Hepta Windows product material shutdown at {:?}; authority=false",
                        receipt.phase
                    );
                }
                Err(error) => {
                    error!(
                        "Hepta Windows product material shutdown failed closed: {:?}",
                        error
                    );
                }
            }
        }
    }

    impl Widget for HeptaV4WindowsProductHostWiringNode {
        fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
            match event {
                Event::Resume | Event::Foreground => self.reconcile_preferences(),
                Event::Pause | Event::Background => self.suspend(),
                Event::Shutdown => self.shutdown(),
                _ => {}
            }
            self.view.handle_event(cx, event, scope);
        }

        fn draw_walk(
            &mut self,
            cx: &mut Cx2d,
            scope: &mut Scope,
            walk: Walk,
        ) -> DrawStep {
            self.view.draw_walk(cx, scope, walk)
        }

        fn is_interactive(&self) -> bool {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn script_mod(vm: &mut ScriptVm) {
    windows::script_mod(vm);
}

#[cfg(not(target_os = "windows"))]
pub fn script_mod(_vm: &mut ScriptVm) {}
