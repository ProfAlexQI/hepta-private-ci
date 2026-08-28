//! Isolated end-to-end Windows material-profile producer for Hepta UI v4.
//!
//! The enabled build requires Windows plus the explicit vendored Makepad hook
//! cfg. It proves, in one UI-thread-ordered process, a persistent root Mica
//! acknowledgement and a dedicated popup Acrylic -> explicit None rollback ->
//! exact Destroyed lifecycle, then runs the fail-closed aggregate verifier.
//!
//! A passing receipt is still fixture-only and only makes the evidence set
//! eligible for a later product-integration review. It does not bind the
//! product material host, complete the Windows profile, or grant authority.

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::makepad_widgets::*;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::shared::hepta_makepad_window_material::{
    HeptaMakepadWindowMaterialPhase, HeptaMakepadWindowMaterialReceipt,
};
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::shared::hepta_platform_material::HeptaPlatform;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::shared::hepta_window_visual_ack::{
    HeptaWindowVisualAckReceipt, HeptaWindowVisualAckStatus,
    HeptaWindowVisualRequestIdentity,
};
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::shared::hepta_windows_backend_ack_bridge::{
    HeptaWindowsBackendAckBridge, HeptaWindowsBackendVisualsProcessed,
    HeptaWindowsBackendWindowIdentity,
};
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::shared::hepta_windows_material_adapter::HeptaWindowsDwmBackdropApi;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::shared::hepta_windows_material_profile_aggregate::{
    aggregate_windows_material_profile, HeptaWindowsMaterialProfileAggregateStatus,
    HeptaWindowsTransientLifecycleEvidence, HeptaWindowsTransientReceipt,
};
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::shared::hepta_windows_transient_material_host::{
    HeptaWindowsTransientBackendProcessed, HeptaWindowsTransientMaterialHost,
    HeptaWindowsTransientMaterialKind, HeptaWindowsTransientReceiptStatus,
    HeptaWindowsTransientRequestIdentity, HeptaWindowsTransientWindowIdentity,
};
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use serde_json::{json, Value};

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const RECEIPT_SCHEMA: &str = "hepta.ui.v4.windows-material-profile-aggregate.v1";
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const MAKEPAD_REVISION: &str = "c4335cee10b22aca768510c9d072b0ca1bba15c8";
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const ROOT_MICA_SEQUENCE: u64 = 1;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const TRANSIENT_ACRYLIC_SEQUENCE: u64 = 2;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const TRANSIENT_SOLID_SEQUENCE: u64 = 3;

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(WindowsMaterialProfileProbeApp::script_component(vm)) {
        ui: Root {
            root_window := Window {
                show_caption_bar: true
                window.inner_size: vec2(760, 460)
                window.title: "Hepta UI v4 Windows Material Profile Probe"
                pass.clear_color: #f4f7fb
                body +: {
                    flow: Down
                    align: Align{x: 0.5, y: 0.5}
                    spacing: 12
                    Label {
                        draw_text.text_style.font_size: 18
                        text: "Hepta Windows material-profile evidence probe"
                    }
                    Label {
                        draw_text.text_style.font_size: 14
                        text: "Root Mica → dedicated popup Acrylic → explicit None rollback → Destroyed"
                    }
                }
            }
        }
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
app_main!(WindowsMaterialProfileProbeApp);

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProbePhase {
    WaitingToQueueRoot,
    WaitingForRootMica,
    WaitingForPopupCreation,
    WaitingForAcrylic,
    WaitingForSolidRollback,
    WaitingForDestroyed,
    Complete,
    Failed,
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl Default for ProbePhase {
    fn default() -> Self {
        Self::WaitingToQueueRoot
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
#[derive(Script)]
struct WindowsMaterialProfileProbeApp {
    #[live]
    ui: WidgetRef,
    #[rust]
    start_timer: Timer,
    #[rust]
    progress_timer: Timer,
    #[rust]
    watchdog_timer: Timer,
    #[rust]
    phase: ProbePhase,
    #[rust]
    root_window_id: Option<WindowId>,
    #[rust]
    popup_window_id: Option<WindowId>,
    #[rust]
    root_pending_request: Option<HeptaWindowVisualRequestIdentity>,
    #[rust]
    root_bridge: HeptaWindowsBackendAckBridge,
    #[rust]
    root_identity: Option<HeptaWindowsBackendWindowIdentity>,
    #[rust]
    root_receipt: Option<HeptaWindowVisualAckReceipt>,
    #[rust]
    transient_host: HeptaWindowsTransientMaterialHost,
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl WindowsMaterialProfileProbeApp {
    fn receipt_path() -> PathBuf {
        std::env::var_os("HEPTA_WINDOWS_PROFILE_RECEIPT_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from("hepta-ui-v4-windows-material-profile-receipt.json")
            })
    }

    fn queue_root_mica(&mut self, cx: &mut Cx) -> Result<(), String> {
        let window = self.ui.window(cx, ids!(root_window));
        let window_id = window
            .window_id()
            .filter(|window_id| cx.windows.is_valid(*window_id))
            .ok_or_else(|| "root WindowId is unavailable".to_string())?;
        if !cx.windows[window_id].is_created {
            return Err("root window has not been created".to_string());
        }
        if cx.windows[window_id].is_popup {
            return Err("root window unexpectedly became a popup".to_string());
        }

        let visuals = WindowVisuals {
            transparent: true,
            backdrop: WindowBackdrop::Mica,
            backdrop_intensity: 0.90,
        }
        .normalized();
        let request = root_request_identity(ROOT_MICA_SEQUENCE, window_id, visuals)?;
        queue_correlated_visuals(cx, window_id, visuals, ROOT_MICA_SEQUENCE)?;

        self.root_window_id = Some(window_id);
        self.root_pending_request = Some(request);
        self.phase = ProbePhase::WaitingForRootMica;
        self.ui.redraw(cx);
        Ok(())
    }

    fn create_transient_popup(&mut self, cx: &mut Cx) -> Result<(), String> {
        let root_window_id = self
            .root_window_id
            .ok_or_else(|| "root WindowId is missing".to_string())?;
        let root_identity = self
            .root_identity
            .ok_or_else(|| "root backend identity is missing".to_string())?;
        self.transient_host
            .bind_parent(
                HeptaWindowsTransientWindowIdentity::new(
                    root_identity.hwnd,
                    root_identity.window_index,
                    root_identity.window_generation,
                )
                .map_err(|error| format!("transient parent rejected: {error:?}"))?,
            )
            .map_err(|error| format!("transient parent binding failed: {error:?}"))?;

        let popup = WindowHandle::new_popup(
            cx,
            root_window_id,
            dvec2(104.0, 96.0),
            dvec2(380.0, 240.0),
        );
        let popup_window_id = popup.window_id();
        if popup_window_id == root_window_id {
            return Err("popup reused the root WindowId".to_string());
        }
        self.popup_window_id = Some(popup_window_id);
        self.phase = ProbePhase::WaitingForPopupCreation;
        self.progress_timer = cx.start_timeout(0.10);
        self.ui.redraw(cx);
        Ok(())
    }

    fn queue_popup_acrylic_if_ready(&mut self, cx: &mut Cx) -> Result<(), String> {
        let popup_window_id = self
            .popup_window_id
            .ok_or_else(|| "popup WindowId is missing".to_string())?;
        if !cx.windows.is_valid(popup_window_id)
            || !cx.windows[popup_window_id].is_created
        {
            self.progress_timer = cx.start_timeout(0.10);
            return Ok(());
        }
        if !cx.windows[popup_window_id].is_popup {
            return Err("transient window is not marked as popup".to_string());
        }
        let root_window_id = self
            .root_window_id
            .ok_or_else(|| "root WindowId is missing".to_string())?;
        let visuals = WindowVisuals {
            transparent: true,
            backdrop: WindowBackdrop::Acrylic,
            backdrop_intensity: 0.86,
        }
        .normalized();
        let request = HeptaWindowsTransientRequestIdentity::new(
            TRANSIENT_ACRYLIC_SEQUENCE,
            root_window_id,
            popup_window_id,
            visuals,
            HeptaWindowsTransientMaterialKind::Acrylic,
        )
        .map_err(|error| format!("Acrylic request rejected: {error:?}"))?;
        self.transient_host
            .register_request(request)
            .map_err(|error| format!("Acrylic request registration failed: {error:?}"))?;
        queue_correlated_visuals(
            cx,
            popup_window_id,
            visuals,
            TRANSIENT_ACRYLIC_SEQUENCE,
        )?;
        self.phase = ProbePhase::WaitingForAcrylic;
        self.ui.redraw(cx);
        Ok(())
    }

    fn queue_popup_solid_rollback(&mut self, cx: &mut Cx) -> Result<(), String> {
        let root_window_id = self
            .root_window_id
            .ok_or_else(|| "root WindowId is missing".to_string())?;
        let popup_window_id = self
            .popup_window_id
            .ok_or_else(|| "popup WindowId is missing".to_string())?;
        let visuals = WindowVisuals::default();
        let request = HeptaWindowsTransientRequestIdentity::new(
            TRANSIENT_SOLID_SEQUENCE,
            root_window_id,
            popup_window_id,
            visuals,
            HeptaWindowsTransientMaterialKind::SolidRollback,
        )
        .map_err(|error| format!("solid rollback request rejected: {error:?}"))?;
        self.transient_host
            .register_request(request)
            .map_err(|error| format!("solid rollback registration failed: {error:?}"))?;
        queue_correlated_visuals(
            cx,
            popup_window_id,
            visuals,
            TRANSIENT_SOLID_SEQUENCE,
        )?;
        self.phase = ProbePhase::WaitingForSolidRollback;
        self.ui.redraw(cx);
        Ok(())
    }

    fn begin_popup_close(&mut self, cx: &mut Cx) -> Result<(), String> {
        self.transient_host
            .begin_close()
            .map_err(|error| format!("transient close rejected: {error:?}"))?;
        let popup_window_id = self
            .popup_window_id
            .ok_or_else(|| "popup WindowId is missing".to_string())?;
        cx.push_unique_platform_op(CxOsOp::CloseWindow(popup_window_id));
        self.phase = ProbePhase::WaitingForDestroyed;
        self.progress_timer = cx.start_timeout(0.10);
        self.ui.redraw(cx);
        Ok(())
    }

    fn drain_backend_hook(&mut self, cx: &mut Cx) {
        for event in take_windows_window_visuals_hook_events() {
            let result = match event {
                WindowsWindowVisualsHookEvent::Processed {
                    request_sequence,
                    window_id,
                    native_window_handle,
                    visuals,
                    backend_apply_succeeded,
                    is_popup,
                } => {
                    if self.root_window_id == Some(window_id) {
                        self.process_root_event(
                            cx,
                            request_sequence,
                            window_id,
                            native_window_handle,
                            visuals,
                            backend_apply_succeeded,
                            is_popup,
                        )
                    } else if self.popup_window_id == Some(window_id) {
                        self.process_transient_event(
                            cx,
                            request_sequence,
                            window_id,
                            native_window_handle,
                            visuals,
                            backend_apply_succeeded,
                            is_popup,
                        )
                    } else {
                        Err(format!(
                            "unexpected correlated processed event for WindowId {}:{}",
                            window_id.0, window_id.1
                        ))
                    }
                }
                WindowsWindowVisualsHookEvent::Destroyed {
                    window_id,
                    native_window_handle,
                } => {
                    if self.popup_window_id == Some(window_id) {
                        self.process_transient_destroyed(cx, window_id, native_window_handle)
                    } else if self.root_window_id == Some(window_id) {
                        Err("root window was destroyed before profile completion".to_string())
                    } else {
                        Ok(())
                    }
                }
            };
            if let Err(error) = result {
                self.fail(cx, error);
                return;
            }
            if matches!(self.phase, ProbePhase::Complete | ProbePhase::Failed) {
                return;
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn process_root_event(
        &mut self,
        cx: &mut Cx,
        request_sequence: u64,
        window_id: WindowId,
        native_window_handle: isize,
        visuals: WindowVisuals,
        backend_apply_succeeded: bool,
        is_popup: bool,
    ) -> Result<(), String> {
        if self.phase != ProbePhase::WaitingForRootMica {
            return Err(format!(
                "root processed event arrived in invalid phase {:?}",
                self.phase
            ));
        }
        let request = self
            .root_pending_request
            .ok_or_else(|| "root event arrived without pending request".to_string())?;
        if request_sequence != request.request_sequence()
            || window_id.0 != request.window_index()
            || window_id.1 != request.window_generation()
        {
            return Err("root request identity drifted".to_string());
        }
        let processed = HeptaWindowsBackendVisualsProcessed::new(
            request_sequence,
            window_id.0,
            window_id.1,
            native_window_handle,
            visuals,
            backend_apply_succeeded,
            is_popup,
        )
        .map_err(|error| format!("root processed event rejected: {error:?}"))?;
        self.root_bridge
            .bind_window(processed.identity())
            .map_err(|error| format!("root window binding rejected: {error:?}"))?;
        self.root_bridge
            .register_request(request)
            .map_err(|error| format!("root request registration rejected: {error:?}"))?;
        self.root_pending_request = None;

        let mut api = HeptaWindowsDwmBackdropApi;
        let receipt = self
            .root_bridge
            .process_backend_event(processed, &mut api)
            .map_err(|error| format!("root DWM observation rejected: {error:?}"))?;
        if !receipt.accepted
            || receipt.status
                != HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback
            || receipt.observed_backdrop != Some(WindowBackdrop::Mica)
            || !receipt.backdrop_exact
            || receipt.full_visuals_exact
            || !receipt.grants_no_authority()
            || !receipt.remains_partial()
        {
            return Err(format!(
                "root Mica receipt escaped contract: {:?}",
                receipt.status
            ));
        }

        self.root_identity = Some(processed.identity());
        self.root_receipt = Some(receipt);
        self.create_transient_popup(cx)
    }

    #[allow(clippy::too_many_arguments)]
    fn process_transient_event(
        &mut self,
        cx: &mut Cx,
        request_sequence: u64,
        window_id: WindowId,
        native_window_handle: isize,
        visuals: WindowVisuals,
        backend_apply_succeeded: bool,
        is_popup: bool,
    ) -> Result<(), String> {
        let processed = HeptaWindowsTransientBackendProcessed::new(
            request_sequence,
            window_id,
            native_window_handle,
            visuals,
            backend_apply_succeeded,
            is_popup,
        )
        .map_err(|error| format!("transient processed event rejected: {error:?}"))?;
        let mut api = HeptaWindowsDwmBackdropApi;
        let receipt = self
            .transient_host
            .process_backend_event(processed, &mut api)
            .map_err(|error| format!("transient DWM observation rejected: {error:?}"))?;
        if !receipt.accepted
            || !receipt.grants_no_authority()
            || !receipt.remains_partial()
            || !receipt.backdrop_exact
        {
            return Err(format!(
                "transient receipt escaped contract: {:?}",
                receipt.status
            ));
        }

        match self.phase {
            ProbePhase::WaitingForAcrylic => {
                if receipt.status
                    != HeptaWindowsTransientReceiptStatus::VerifiedAcrylicWithBackdropReadback
                    || receipt.requested_backdrop != WindowBackdrop::Acrylic
                    || receipt.observed_backdrop != Some(WindowBackdrop::Acrylic)
                {
                    return Err(format!(
                        "unexpected Acrylic receipt: {:?}",
                        receipt.status
                    ));
                }
                self.queue_popup_solid_rollback(cx)
            }
            ProbePhase::WaitingForSolidRollback => {
                if receipt.status
                    != HeptaWindowsTransientReceiptStatus::VerifiedSolidRollbackWithBackdropReadback
                    || receipt.requested_backdrop != WindowBackdrop::None
                    || receipt.observed_backdrop != Some(WindowBackdrop::None)
                {
                    return Err(format!(
                        "unexpected solid rollback receipt: {:?}",
                        receipt.status
                    ));
                }
                self.begin_popup_close(cx)
            }
            phase => Err(format!(
                "transient receipt arrived in invalid phase {phase:?}"
            )),
        }
    }

    fn process_transient_destroyed(
        &mut self,
        cx: &mut Cx,
        window_id: WindowId,
        native_window_handle: isize,
    ) -> Result<(), String> {
        if self.phase != ProbePhase::WaitingForDestroyed {
            return Err(format!(
                "Destroyed event arrived in invalid phase {:?}",
                self.phase
            ));
        }
        let destroyed = HeptaWindowsTransientWindowIdentity::from_window_id(
            native_window_handle,
            window_id,
        )
        .map_err(|error| format!("Destroyed identity rejected: {error:?}"))?;
        if !self
            .transient_host
            .process_destroyed(destroyed)
            .map_err(|error| format!("Destroyed event rejected: {error:?}"))?
        {
            return Err("Destroyed identity did not match the transient host".to_string());
        }
        self.complete(cx)
    }

    fn complete(&mut self, cx: &mut Cx) -> Result<(), String> {
        let candidate_commit = exact_candidate("HEPTA_CANDIDATE_COMMIT")?;
        let candidate_tree = exact_candidate("HEPTA_CANDIDATE_TREE")?;
        let root_identity = self
            .root_identity
            .ok_or_else(|| "missing root identity".to_string())?;
        let root_receipt = self
            .root_receipt
            .ok_or_else(|| "missing root receipt".to_string())?;
        let transient = self
            .transient_host
            .profile_evidence()
            .map_err(|error| format!("transient evidence export failed: {error:?}"))?;
        let aggregate =
            aggregate_windows_material_profile(root_identity, root_receipt, transient);
        if !aggregate.accepted
            || aggregate.status
                != HeptaWindowsMaterialProfileAggregateStatus::ReadyForProductIntegrationReview
            || !aggregate.eligible_for_product_integration_review
            || !aggregate.remains_unbound()
            || !aggregate.grants_no_authority()
        {
            return Err(format!(
                "aggregate rejected runtime evidence: {:?}",
                aggregate.status
            ));
        }

        let receipt = success_receipt(
            candidate_commit,
            candidate_tree,
            root_identity,
            root_receipt,
            transient,
        );
        write_json_atomic(&Self::receipt_path(), &receipt)
            .map_err(|error| format!("write aggregate receipt: {error}"))?;
        self.phase = ProbePhase::Complete;
        cx.quit();
        Ok(())
    }

    fn fail(&mut self, cx: &mut Cx, error: String) {
        if matches!(self.phase, ProbePhase::Complete | ProbePhase::Failed) {
            return;
        }
        let receipt = failure_receipt(self.phase, error);
        let _ = write_json_atomic(&Self::receipt_path(), &receipt);
        self.phase = ProbePhase::Failed;
        cx.quit();
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl MatchEvent for WindowsMaterialProfileProbeApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.start_timer = cx.start_timeout(0.75);
        self.watchdog_timer = cx.start_timeout(45.0);
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl AppMain for WindowsMaterialProfileProbeApp {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::theme_mod(vm);
        script_eval!(vm, {
            mod.theme = mod.themes.light
        });
        makepad_widgets::widgets_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.drain_backend_hook(cx);
        if matches!(self.phase, ProbePhase::Complete | ProbePhase::Failed) {
            return;
        }

        if self.start_timer.is_event(event).is_some() {
            self.start_timer = Timer::empty();
            if let Err(error) = self.queue_root_mica(cx) {
                self.fail(cx, error);
                return;
            }
        }

        if self.progress_timer.is_event(event).is_some() {
            self.progress_timer = Timer::empty();
            let result = match self.phase {
                ProbePhase::WaitingForPopupCreation => {
                    self.queue_popup_acrylic_if_ready(cx)
                }
                ProbePhase::WaitingForDestroyed => {
                    self.progress_timer = cx.start_timeout(0.10);
                    self.ui.redraw(cx);
                    Ok(())
                }
                _ => Ok(()),
            };
            if let Err(error) = result {
                self.fail(cx, error);
                return;
            }
        }

        if self.watchdog_timer.is_event(event).is_some() {
            self.watchdog_timer = Timer::empty();
            self.fail(
                cx,
                format!("profile probe timed out in phase {:?}", self.phase),
            );
            return;
        }

        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn root_request_identity(
    sequence: u64,
    window_id: WindowId,
    visuals: WindowVisuals,
) -> Result<HeptaWindowVisualRequestIdentity, String> {
    HeptaWindowVisualRequestIdentity::from_makepad_receipt(
        HeptaMakepadWindowMaterialReceipt {
            generation: sequence,
            platform: HeptaPlatform::Windows,
            window_index: Some(window_id.0),
            window_generation: Some(window_id.1),
            phase: HeptaMakepadWindowMaterialPhase::PersistentChromeRequested,
            requested_visuals: visuals,
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
    .map_err(|error| format!("root request identity rejected: {error:?}"))
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn queue_correlated_visuals(
    cx: &mut Cx,
    window_id: WindowId,
    visuals: WindowVisuals,
    request_sequence: u64,
) -> Result<(), String> {
    if !cx.windows.is_valid(window_id) {
        return Err("target WindowId is invalid".to_string());
    }
    if !cx.windows[window_id].is_created {
        return Err("target window is not created".to_string());
    }
    let visuals = visuals.normalized();
    if cx.windows[window_id].window_visuals() == visuals {
        return Err("correlated request would be a deduplicated no-op".to_string());
    }
    cx.windows[window_id].transparent = visuals.transparent;
    cx.windows[window_id].backdrop = visuals.backdrop;
    cx.windows[window_id].backdrop_intensity = visuals.backdrop_intensity;
    cx.push_unique_platform_op(CxOsOp::SetWindowVisualsCorrelated {
        window_id,
        visuals,
        request_sequence,
    });
    Ok(())
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn success_receipt(
    candidate_commit: String,
    candidate_tree: String,
    root_identity: HeptaWindowsBackendWindowIdentity,
    root: HeptaWindowVisualAckReceipt,
    transient: HeptaWindowsTransientLifecycleEvidence,
) -> Value {
    json!({
        "schema": RECEIPT_SCHEMA,
        "status": "PASS_WINDOWS_MATERIAL_PROFILE_AGGREGATE",
        "candidate": {
            "commit": candidate_commit,
            "tree": candidate_tree,
        },
        "makepad": {
            "revision": MAKEPAD_REVISION,
            "vendoredPatchBuild": true,
            "defaultDependencySwitched": false,
        },
        "fixture": true,
        "root": {
            "identity": identity_json(
                root_identity.window_index,
                root_identity.window_generation,
                root_identity.hwnd,
            ),
            "requestSequence": root.request_sequence,
            "backend": format!("{:?}", root.backend),
            "status": format!("{:?}", root.status),
            "readbackScope": format!("{:?}", root.readback_scope),
            "requestedBackdrop": format!("{:?}", root.requested_visuals.backdrop),
            "observedBackdrop": root
                .observed_backdrop
                .map(|value| format!("{value:?}"))
                .unwrap_or_else(|| "MISSING".to_string()),
            "backdropExact": root.backdrop_exact,
        },
        "transient": {
            "parent": identity_json(
                transient.parent_window_index,
                transient.parent_window_generation,
                transient.parent_hwnd,
            ),
            "identity": identity_json(
                transient.transient.window_index,
                transient.transient.window_generation,
                transient.transient.hwnd,
            ),
            "separateFromRoot": transient.transient.hwnd != root_identity.hwnd,
            "acrylic": transient_receipt_json(transient.acrylic),
            "solidRollback": transient_receipt_json(transient.solid_rollback),
            "destroyedIdentity": identity_json(
                transient.destroyed.window_index,
                transient.destroyed.window_generation,
                transient.destroyed.hwnd,
            ),
            "destroyedAcknowledged": transient.destroyed_acknowledged,
        },
        "qualification": {
            "rootRuntimeReceipt": true,
            "transientRuntimeReceipt": true,
            "dualReceiptAggregateRuntime": true,
            "eligibleForProductIntegrationReview": true,
            "productBound": false,
            "transientSystemMaterialBound": false,
            "completeProfileBound": false,
            "systemMaterialBound": false,
            "nativeProductRuntime": false,
            "deviceValidation": false,
        },
        "authority": authority_json(),
    })
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn failure_receipt(phase: ProbePhase, error: String) -> Value {
    json!({
        "schema": RECEIPT_SCHEMA,
        "status": "FAIL_WINDOWS_MATERIAL_PROFILE_AGGREGATE",
        "candidate": {
            "commit": std::env::var("HEPTA_CANDIDATE_COMMIT")
                .unwrap_or_else(|_| "UNBOUND".to_string()),
            "tree": std::env::var("HEPTA_CANDIDATE_TREE")
                .unwrap_or_else(|_| "UNBOUND".to_string()),
        },
        "makepad": {
            "revision": MAKEPAD_REVISION,
            "vendoredPatchBuild": true,
            "defaultDependencySwitched": false,
        },
        "fixture": true,
        "phase": format!("{phase:?}"),
        "failures": [error],
        "qualification": {
            "rootRuntimeReceipt": false,
            "transientRuntimeReceipt": false,
            "dualReceiptAggregateRuntime": false,
            "eligibleForProductIntegrationReview": false,
            "productBound": false,
            "transientSystemMaterialBound": false,
            "completeProfileBound": false,
            "systemMaterialBound": false,
            "nativeProductRuntime": false,
            "deviceValidation": false,
        },
        "authority": authority_json(),
    })
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn transient_receipt_json(receipt: HeptaWindowsTransientReceipt) -> Value {
    json!({
        "status": format!("{:?}", receipt.status),
        "requestSequence": receipt.request_sequence,
        "requestedBackdrop": format!("{:?}", receipt.requested_backdrop),
        "observedBackdrop": format!("{:?}", receipt.observed_backdrop),
        "backdropExact": receipt.backdrop_exact,
    })
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn identity_json(index: usize, generation: u64, hwnd: isize) -> Value {
    json!({
        "index": index,
        "generation": generation,
        "nativeHandle": (hwnd as usize).to_string(),
    })
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn authority_json() -> Value {
    json!({
        "network": false,
        "mutation": false,
        "effect": false,
        "liveAdapter": false,
        "production": false,
        "operatorAcceptance": false,
        "promotion": false,
        "release": false,
    })
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn exact_candidate(name: &str) -> Result<String, String> {
    let value = std::env::var(name).map_err(|_| format!("{name} is missing"))?;
    if value.len() != 40 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{name} is not an exact Git object ID"));
    }
    Ok(value)
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn ensure_parent(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn write_json_atomic(path: &Path, value: &Value) -> std::io::Result<()> {
    ensure_parent(path)?;
    let temporary = path.with_extension("tmp");
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    fs::write(&temporary, bytes)?;
    fs::rename(temporary, path)
}

#[cfg(not(all(target_os = "windows", hepta_makepad_windows_ack_hook)))]
fn main() {
    eprintln!(
        "hepta-ui-v4-windows-material-profile-probe requires Windows plus the vendored Makepad hook cfg"
    );
}
