//! Isolated Windows transient Acrylic lifecycle probe.
//!
//! The enabled build requires Windows plus the explicit vendored Makepad hook
//! cfg. It first proves the persistent root Mica identity, creates a separate
//! Makepad popup window, then validates Acrylic, synthetic focus-loss cleanup,
//! a second Acrylic cycle, explicit close cleanup, and exact Destroyed evidence.
//!
//! This is a fixture-only producer. It starts no Matrix runtime, provider,
//! network request, mutation, product material host, or production path.

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
use hepta_native::shared::hepta_windows_transient_material_host::{
    HeptaWindowsTransientAckReceipt, HeptaWindowsTransientAckStatus,
    HeptaWindowsTransientCleanupReason, HeptaWindowsTransientDestroyReceipt,
    HeptaWindowsTransientFrameworkWindowIdentity, HeptaWindowsTransientMaterialHost,
    HeptaWindowsTransientRequest, HeptaWindowsTransientVisualsProcessed,
};
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use serde_json::{json, Value};

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const RECEIPT_SCHEMA: &str = "hepta.ui.v4.windows-transient-material-probe.v1";
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const MAKEPAD_REVISION: &str = "c4335cee10b22aca768510c9d072b0ca1bba15c8";
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const ROOT_MICA_SEQUENCE: u64 = 1;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const TRANSIENT_ACRYLIC_SEQUENCE: u64 = 101;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const FOCUS_CLEANUP_SEQUENCE: u64 = 102;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const TRANSIENT_REACRYLIC_SEQUENCE: u64 = 103;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const CLOSE_CLEANUP_SEQUENCE: u64 = 104;

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(WindowsTransientProbeApp::script_component(vm)) {
        ui: Root {
            root_window := Window {
                show_caption_bar: true
                window.inner_size: vec2(760, 460)
                window.title: "Hepta UI v4 Windows transient material probe"
                pass.clear_color: #f4f7fb
                body +: {
                    flow: Down
                    align: Align{x: 0.5, y: 0.5}
                    spacing: 12
                    Label {
                        draw_text.text_style.font_size: 18
                        text: "Persistent Mica + separate transient Acrylic"
                    }
                    Label {
                        draw_text.text_style.font_size: 14
                        text: "Acrylic → focus-loss solid cleanup → Acrylic → close cleanup → Destroyed"
                    }
                }
            }
        }
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
app_main!(WindowsTransientProbeApp);

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProbePhase {
    WaitingToQueueRoot,
    WaitingForRootMica,
    WaitingForPopupCreation,
    WaitingForInitialAcrylic,
    WaitingForFocusCleanup,
    WaitingForSecondAcrylic,
    WaitingForCloseCleanup,
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
struct WindowsTransientProbeApp {
    #[live]
    ui: WidgetRef,
    #[rust]
    start_timer: Timer,
    #[rust]
    popup_poll_timer: Timer,
    #[rust]
    watchdog_timer: Timer,
    #[rust]
    phase: ProbePhase,
    #[rust]
    root_window_id: Option<WindowId>,
    #[rust]
    transient_window_id: Option<WindowId>,
    #[rust]
    root_pending: Option<HeptaWindowVisualRequestIdentity>,
    #[rust]
    root_bridge: HeptaWindowsBackendAckBridge,
    #[rust]
    transient_host: HeptaWindowsTransientMaterialHost,
    #[rust]
    root_receipt: Option<HeptaWindowVisualAckReceipt>,
    #[rust]
    initial_acrylic: Option<HeptaWindowsTransientAckReceipt>,
    #[rust]
    focus_cleanup: Option<HeptaWindowsTransientAckReceipt>,
    #[rust]
    second_acrylic: Option<HeptaWindowsTransientAckReceipt>,
    #[rust]
    close_cleanup: Option<HeptaWindowsTransientAckReceipt>,
    #[rust]
    destroy_receipt: Option<HeptaWindowsTransientDestroyReceipt>,
    #[rust]
    root_identity: Option<HeptaWindowsBackendWindowIdentity>,
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl WindowsTransientProbeApp {
    fn receipt_path() -> PathBuf {
        std::env::var_os("HEPTA_WINDOWS_TRANSIENT_RECEIPT_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from("hepta-ui-v4-windows-transient-material-receipt.json")
            })
    }

    fn root_window(&self, cx: &mut Cx) -> Result<WindowId, String> {
        self.ui
            .window(cx, ids!(root_window))
            .window_id()
            .filter(|window_id| cx.windows.is_valid(*window_id))
            .ok_or_else(|| "root WindowId is unavailable".to_string())
    }

    fn queue_root_mica(&mut self, cx: &mut Cx) -> Result<(), String> {
        if self.root_pending.is_some() {
            return Err("root request is already pending".to_string());
        }
        let window_id = self.root_window(cx)?;
        if !cx.windows[window_id].is_created || cx.windows[window_id].is_popup {
            return Err("root window is not a created ordinary window".to_string());
        }
        let visuals = WindowVisuals {
            transparent: true,
            backdrop: WindowBackdrop::Mica,
            backdrop_intensity: 0.90,
        }
        .normalized();
        queue_correlated(cx, window_id, ROOT_MICA_SEQUENCE, visuals)?;
        let request = root_request(window_id, ROOT_MICA_SEQUENCE, visuals)?;
        self.root_window_id = Some(window_id);
        self.root_pending = Some(request);
        self.phase = ProbePhase::WaitingForRootMica;
        self.ui.redraw(cx);
        Ok(())
    }

    fn create_transient_popup(&mut self, cx: &mut Cx) -> Result<(), String> {
        let root = self
            .root_window_id
            .ok_or_else(|| "root WindowId was not recorded".to_string())?;
        let popup = WindowHandle::new_popup(
            cx,
            root,
            dvec2(150.0, 120.0),
            dvec2(380.0, 240.0),
        );
        let popup_id = popup.window_id();
        if popup_id == root {
            return Err("transient popup reused the root WindowId".to_string());
        }
        self.transient_window_id = Some(popup_id);
        self.phase = ProbePhase::WaitingForPopupCreation;
        self.popup_poll_timer = cx.start_timeout(0.10);
        Ok(())
    }

    fn poll_popup_creation(&mut self, cx: &mut Cx) -> Result<(), String> {
        let popup = self
            .transient_window_id
            .ok_or_else(|| "transient WindowId is missing".to_string())?;
        if !cx.windows.is_valid(popup) || !cx.windows[popup].is_created {
            self.popup_poll_timer = cx.start_timeout(0.10);
            return Ok(());
        }
        if !cx.windows[popup].is_popup {
            return Err("transient window is not a Makepad popup".to_string());
        }
        self.transient_host
            .observe_transient_window(
                HeptaWindowsTransientFrameworkWindowIdentity::new(
                    popup.0,
                    popup.1,
                    true,
                )
                .map_err(|error| format!("transient identity rejected: {error:?}"))?,
            )
            .map_err(|error| format!("transient window observation rejected: {error:?}"))?;
        let request = self
            .transient_host
            .begin_acrylic(TRANSIENT_ACRYLIC_SEQUENCE)
            .map_err(|error| format!("initial Acrylic request rejected: {error:?}"))?;
        self.queue_transient_request(cx, request)?;
        self.phase = ProbePhase::WaitingForInitialAcrylic;
        Ok(())
    }

    fn queue_transient_request(
        &mut self,
        cx: &mut Cx,
        request: HeptaWindowsTransientRequest,
    ) -> Result<(), String> {
        let window_id = WindowId(
            request.window.window_index,
            request.window.window_generation,
        );
        if !cx.windows.is_valid(window_id)
            || !cx.windows[window_id].is_created
            || !cx.windows[window_id].is_popup
        {
            return Err("transient request target is not a created popup".to_string());
        }
        queue_correlated(cx, window_id, request.sequence, request.visuals)?;
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
                            request_sequence,
                            window_id,
                            native_window_handle,
                            visuals,
                            backend_apply_succeeded,
                            is_popup,
                        )
                        .map(RootOrTransientReceipt::Root)
                    } else if self.transient_window_id == Some(window_id) {
                        self.process_transient_event(
                            request_sequence,
                            window_id,
                            native_window_handle,
                            visuals,
                            backend_apply_succeeded,
                            is_popup,
                        )
                        .map(RootOrTransientReceipt::Transient)
                    } else {
                        Ok(RootOrTransientReceipt::Ignored)
                    }
                }
                WindowsWindowVisualsHookEvent::Destroyed {
                    window_id,
                    native_window_handle,
                } => {
                    if self.transient_window_id == Some(window_id) {
                        self.process_transient_destroyed(
                            window_id,
                            native_window_handle,
                        )
                        .map(RootOrTransientReceipt::Destroyed)
                    } else if self.root_window_id == Some(window_id)
                        && !matches!(self.phase, ProbePhase::Complete | ProbePhase::Failed)
                    {
                        Err("root window was destroyed before probe completion".to_string())
                    } else {
                        Ok(RootOrTransientReceipt::Ignored)
                    }
                }
            };

            match result {
                Ok(RootOrTransientReceipt::Root(receipt)) => {
                    if let Err(error) = self.accept_root_receipt(cx, receipt) {
                        self.fail(cx, error);
                        return;
                    }
                }
                Ok(RootOrTransientReceipt::Transient(receipt)) => {
                    if let Err(error) = self.accept_transient_receipt(cx, receipt) {
                        self.fail(cx, error);
                        return;
                    }
                }
                Ok(RootOrTransientReceipt::Destroyed(receipt)) => {
                    if let Err(error) = self.accept_destroyed(cx, receipt) {
                        self.fail(cx, error);
                        return;
                    }
                }
                Ok(RootOrTransientReceipt::Ignored) => {}
                Err(error) => {
                    self.fail(cx, error);
                    return;
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn process_root_event(
        &mut self,
        request_sequence: u64,
        window_id: WindowId,
        native_window_handle: isize,
        visuals: WindowVisuals,
        backend_apply_succeeded: bool,
        is_popup: bool,
    ) -> Result<HeptaWindowVisualAckReceipt, String> {
        let request = self
            .root_pending
            .ok_or_else(|| "root backend event arrived without a pending request".to_string())?;
        if is_popup {
            return Err("root backend event was marked as popup".to_string());
        }
        if request_sequence != request.request_sequence()
            || window_id.0 != request.window_index()
            || window_id.1 != request.window_generation()
        {
            return Err("root backend request identity drifted".to_string());
        }
        let processed = HeptaWindowsBackendVisualsProcessed::new(
            request_sequence,
            window_id.0,
            window_id.1,
            native_window_handle,
            visuals,
            backend_apply_succeeded,
            false,
        )
        .map_err(|error| format!("root processed event rejected: {error:?}"))?;
        self.root_bridge
            .bind_window(processed.identity())
            .map_err(|error| format!("root window binding rejected: {error:?}"))?;
        self.root_bridge
            .register_request(request)
            .map_err(|error| format!("root request registration rejected: {error:?}"))?;
        self.root_pending = None;
        let mut api = HeptaWindowsDwmBackdropApi;
        let receipt = self
            .root_bridge
            .process_backend_event(processed, &mut api)
            .map_err(|error| format!("root DWM observation rejected: {error:?}"))?;
        self.root_identity = Some(processed.identity());
        Ok(receipt)
    }

    #[allow(clippy::too_many_arguments)]
    fn process_transient_event(
        &mut self,
        request_sequence: u64,
        window_id: WindowId,
        native_window_handle: isize,
        visuals: WindowVisuals,
        backend_apply_succeeded: bool,
        is_popup: bool,
    ) -> Result<HeptaWindowsTransientAckReceipt, String> {
        let processed = HeptaWindowsTransientVisualsProcessed::new(
            request_sequence,
            window_id.0,
            window_id.1,
            native_window_handle,
            visuals,
            backend_apply_succeeded,
            is_popup,
        )
        .map_err(|error| format!("transient processed event rejected: {error:?}"))?;
        let mut api = HeptaWindowsDwmBackdropApi;
        self.transient_host
            .process_backend_event(processed, &mut api)
            .map_err(|error| format!("transient DWM observation rejected: {error:?}"))
    }

    fn process_transient_destroyed(
        &mut self,
        window_id: WindowId,
        native_window_handle: isize,
    ) -> Result<HeptaWindowsTransientDestroyReceipt, String> {
        let identity =
            hepta_native::shared::hepta_windows_transient_material_host::
                HeptaWindowsTransientNativeWindowIdentity::new(
                    native_window_handle,
                    window_id.0,
                    window_id.1,
                )
                .map_err(|error| format!("destroyed identity rejected: {error:?}"))?;
        Ok(self.transient_host.process_destroyed(identity))
    }

    fn accept_root_receipt(
        &mut self,
        cx: &mut Cx,
        receipt: HeptaWindowVisualAckReceipt,
    ) -> Result<(), String> {
        if self.phase != ProbePhase::WaitingForRootMica
            || receipt.status
                != HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback
            || !receipt.accepted
            || !receipt.backdrop_exact
            || receipt.full_visuals_exact
            || receipt.observed_backdrop != Some(WindowBackdrop::Mica)
            || !receipt.grants_no_authority()
            || !receipt.remains_partial()
        {
            return Err(format!("unexpected root Mica receipt: {:?}", receipt.status));
        }
        let root = self
            .root_identity
            .ok_or_else(|| "root native identity is missing".to_string())?;
        self.transient_host
            .bind_root(root)
            .map_err(|error| format!("transient host root binding rejected: {error:?}"))?;
        self.root_receipt = Some(receipt);
        self.create_transient_popup(cx)
    }

    fn accept_transient_receipt(
        &mut self,
        cx: &mut Cx,
        receipt: HeptaWindowsTransientAckReceipt,
    ) -> Result<(), String> {
        if !receipt.grants_no_authority() || !receipt.remains_partial() {
            return Err("transient receipt escaped the partial authority boundary".to_string());
        }
        match self.phase {
            ProbePhase::WaitingForInitialAcrylic => {
                require_acrylic(&receipt)?;
                self.initial_acrylic = Some(receipt);
                let cleanup = self
                    .transient_host
                    .begin_focus_loss_cleanup(FOCUS_CLEANUP_SEQUENCE)
                    .map_err(|error| format!("focus cleanup request rejected: {error:?}"))?;
                self.queue_transient_request(cx, cleanup)?;
                self.phase = ProbePhase::WaitingForFocusCleanup;
                Ok(())
            }
            ProbePhase::WaitingForFocusCleanup => {
                require_cleanup(
                    &receipt,
                    HeptaWindowsTransientCleanupReason::FocusLost,
                )?;
                self.focus_cleanup = Some(receipt);
                let acrylic = self
                    .transient_host
                    .begin_acrylic(TRANSIENT_REACRYLIC_SEQUENCE)
                    .map_err(|error| format!("second Acrylic request rejected: {error:?}"))?;
                self.queue_transient_request(cx, acrylic)?;
                self.phase = ProbePhase::WaitingForSecondAcrylic;
                Ok(())
            }
            ProbePhase::WaitingForSecondAcrylic => {
                require_acrylic(&receipt)?;
                self.second_acrylic = Some(receipt);
                let cleanup = self
                    .transient_host
                    .begin_close_cleanup(CLOSE_CLEANUP_SEQUENCE)
                    .map_err(|error| format!("close cleanup request rejected: {error:?}"))?;
                self.queue_transient_request(cx, cleanup)?;
                self.phase = ProbePhase::WaitingForCloseCleanup;
                Ok(())
            }
            ProbePhase::WaitingForCloseCleanup => {
                require_cleanup(&receipt, HeptaWindowsTransientCleanupReason::Close)?;
                self.close_cleanup = Some(receipt);
                self.transient_host
                    .mark_close_requested()
                    .map_err(|error| format!("close request rejected: {error:?}"))?;
                let transient = self
                    .transient_window_id
                    .ok_or_else(|| "transient WindowId is missing".to_string())?;
                cx.push_unique_platform_op(CxOsOp::CloseWindow(transient));
                self.phase = ProbePhase::WaitingForDestroyed;
                Ok(())
            }
            phase => Err(format!(
                "transient receipt arrived in invalid phase: {phase:?}"
            )),
        }
    }

    fn accept_destroyed(
        &mut self,
        cx: &mut Cx,
        receipt: HeptaWindowsTransientDestroyReceipt,
    ) -> Result<(), String> {
        if self.phase != ProbePhase::WaitingForDestroyed
            || !receipt.accepted
            || !receipt.exact_window_identity
            || !receipt.cleanup_confirmed_before_close
            || !receipt.closed
            || !receipt.grants_no_authority()
        {
            return Err("transient Destroyed receipt was not exact and clean".to_string());
        }
        self.destroy_receipt = Some(receipt);
        self.complete(cx)
    }

    fn complete(&mut self, cx: &mut Cx) -> Result<(), String> {
        let root = self.root_receipt.ok_or_else(|| "root receipt missing".to_string())?;
        let initial = self
            .initial_acrylic
            .ok_or_else(|| "initial Acrylic receipt missing".to_string())?;
        let focus = self
            .focus_cleanup
            .ok_or_else(|| "focus cleanup receipt missing".to_string())?;
        let second = self
            .second_acrylic
            .ok_or_else(|| "second Acrylic receipt missing".to_string())?;
        let close = self
            .close_cleanup
            .ok_or_else(|| "close cleanup receipt missing".to_string())?;
        let destroyed = self
            .destroy_receipt
            .ok_or_else(|| "Destroyed receipt missing".to_string())?;
        let root_identity = self
            .root_identity
            .ok_or_else(|| "root identity missing".to_string())?;
        let transient_identity = self
            .transient_host
            .snapshot()
            .transient_native
            .unwrap_or(
                hepta_native::shared::hepta_windows_transient_material_host::
                    HeptaWindowsTransientNativeWindowIdentity {
                        hwnd: destroyed.hwnd,
                        window_index: destroyed.window_index,
                        window_generation: destroyed.window_generation,
                    },
            );
        let candidate_commit =
            std::env::var("HEPTA_CANDIDATE_COMMIT").unwrap_or_else(|_| "UNBOUND".into());
        let candidate_tree =
            std::env::var("HEPTA_CANDIDATE_TREE").unwrap_or_else(|_| "UNBOUND".into());
        if !is_git_object_id(&candidate_commit) || !is_git_object_id(&candidate_tree) {
            return Err("candidate commit/tree are not exact Git object IDs".to_string());
        }
        let distinct = root_identity.hwnd != transient_identity.hwnd
            && (root_identity.window_index != transient_identity.window_index
                || root_identity.window_generation != transient_identity.window_generation);
        if !distinct {
            return Err("root and transient identities are not distinct".to_string());
        }

        let payload = json!({
            "schema": RECEIPT_SCHEMA,
            "status": "PASS_WINDOWS_TRANSIENT_MATERIAL_PROBE",
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
            "environment": {
                "focusLossSynthetic": true,
                "physicalDevice": false,
            },
            "rootWindow": {
                "index": root_identity.window_index,
                "generation": root_identity.window_generation,
                "nativeHandle": root_identity.hwnd.to_string(),
                "popup": false,
            },
            "transientWindow": {
                "index": destroyed.window_index,
                "generation": destroyed.window_generation,
                "nativeHandle": destroyed.hwnd.to_string(),
                "popup": true,
                "distinctFromRoot": distinct,
            },
            "requests": {
                "rootMica": root_ack_json(root),
                "initialAcrylic": transient_ack_json(initial),
                "focusLossCleanup": transient_ack_json(focus),
                "secondAcrylic": transient_ack_json(second),
                "closeCleanup": transient_ack_json(close),
                "destroyed": destroy_json(destroyed),
            },
            "qualification": {
                "patchedMakepadRuntime": true,
                "correlatedHookRuntime": true,
                "exactRootHwndFromFramework": true,
                "exactTransientHwndFromFramework": true,
                "rootTransientDistinct": true,
                "persistentChromeBackdropAcknowledged": true,
                "transientAcrylicBackdropAcknowledged": true,
                "focusLossCleanupAcknowledged": true,
                "focusLossSynthetic": true,
                "closeCleanupAcknowledged": true,
                "destroyedAfterCleanup": true,
                "backdropReadback": true,
                "rollbackRuntime": false,
                "fullVisualReadback": false,
                "transientSystemMaterial": false,
                "completeProfile": false,
                "systemMaterialBinding": false,
                "nativeProductRuntime": false,
                "deviceValidation": false,
            },
            "authority": authority_json(),
        });
        write_json_atomic(&Self::receipt_path(), &payload)
            .map_err(|error| format!("write receipt: {error}"))?;
        self.phase = ProbePhase::Complete;
        cx.quit();
        Ok(())
    }

    fn fail(&mut self, cx: &mut Cx, error: String) {
        if matches!(self.phase, ProbePhase::Complete | ProbePhase::Failed) {
            return;
        }
        let payload = json!({
            "schema": RECEIPT_SCHEMA,
            "status": "FAIL_WINDOWS_TRANSIENT_MATERIAL_PROBE",
            "error": error,
            "candidate": {
                "commit": std::env::var("HEPTA_CANDIDATE_COMMIT").unwrap_or_else(|_| "UNBOUND".into()),
                "tree": std::env::var("HEPTA_CANDIDATE_TREE").unwrap_or_else(|_| "UNBOUND".into()),
            },
            "makepad": {
                "revision": MAKEPAD_REVISION,
                "vendoredPatchBuild": true,
                "defaultDependencySwitched": false,
            },
            "fixture": true,
            "environment": {
                "focusLossSynthetic": true,
                "physicalDevice": false,
            },
            "qualification": {
                "patchedMakepadRuntime": false,
                "correlatedHookRuntime": false,
                "exactRootHwndFromFramework": false,
                "exactTransientHwndFromFramework": false,
                "rootTransientDistinct": false,
                "persistentChromeBackdropAcknowledged": false,
                "transientAcrylicBackdropAcknowledged": false,
                "focusLossCleanupAcknowledged": false,
                "focusLossSynthetic": true,
                "closeCleanupAcknowledged": false,
                "destroyedAfterCleanup": false,
                "backdropReadback": false,
                "rollbackRuntime": false,
                "fullVisualReadback": false,
                "transientSystemMaterial": false,
                "completeProfile": false,
                "systemMaterialBinding": false,
                "nativeProductRuntime": false,
                "deviceValidation": false,
            },
            "authority": authority_json(),
        });
        let _ = write_json_atomic(&Self::receipt_path(), &payload);
        self.phase = ProbePhase::Failed;
        cx.quit();
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
enum RootOrTransientReceipt {
    Root(HeptaWindowVisualAckReceipt),
    Transient(HeptaWindowsTransientAckReceipt),
    Destroyed(HeptaWindowsTransientDestroyReceipt),
    Ignored,
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl MatchEvent for WindowsTransientProbeApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.start_timer = cx.start_timeout(0.75);
        self.watchdog_timer = cx.start_timeout(45.0);
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl AppMain for WindowsTransientProbeApp {
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
        if self.start_timer.is_event(event).is_some() {
            self.start_timer = Timer::empty();
            if let Err(error) = self.queue_root_mica(cx) {
                self.fail(cx, error);
                return;
            }
        }
        if self.popup_poll_timer.is_event(event).is_some() {
            self.popup_poll_timer = Timer::empty();
            if let Err(error) = self.poll_popup_creation(cx) {
                self.fail(cx, error);
                return;
            }
        }
        if self.watchdog_timer.is_event(event).is_some() {
            self.watchdog_timer = Timer::empty();
            self.fail(cx, "transient material probe timed out".to_string());
            return;
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn queue_correlated(
    cx: &mut Cx,
    window_id: WindowId,
    sequence: u64,
    visuals: WindowVisuals,
) -> Result<(), String> {
    let visuals = visuals.normalized();
    if !cx.windows.is_valid(window_id) || !cx.windows[window_id].is_created {
        return Err("correlated request target is not created".to_string());
    }
    if cx.windows[window_id].window_visuals() == visuals {
        return Err("correlated request would be a deduplicated no-op".to_string());
    }
    cx.windows[window_id].transparent = visuals.transparent;
    cx.windows[window_id].backdrop = visuals.backdrop;
    cx.windows[window_id].backdrop_intensity = visuals.backdrop_intensity;
    cx.push_unique_platform_op(CxOsOp::SetWindowVisualsCorrelated {
        window_id,
        visuals,
        request_sequence: sequence,
    });
    Ok(())
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn root_request(
    window_id: WindowId,
    sequence: u64,
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
fn require_acrylic(receipt: &HeptaWindowsTransientAckReceipt) -> Result<(), String> {
    if receipt.status != HeptaWindowsTransientAckStatus::VerifiedAcrylicWithBackdropReadback
        || !receipt.accepted
        || !receipt.acrylic_backdrop_acknowledged
        || !receipt.backdrop_exact
        || receipt.full_visuals_exact
        || receipt.observed_backdrop
            != Some(
                hepta_native::shared::hepta_windows_material_adapter::
                    HeptaWindowsDwmBackdropValue::Acrylic,
            )
    {
        return Err(format!(
            "unexpected transient Acrylic receipt: {:?}",
            receipt.status
        ));
    }
    Ok(())
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn require_cleanup(
    receipt: &HeptaWindowsTransientAckReceipt,
    reason: HeptaWindowsTransientCleanupReason,
) -> Result<(), String> {
    if receipt.status
        != HeptaWindowsTransientAckStatus::VerifiedSolidCleanupWithBackdropReadback
        || !receipt.accepted
        || !receipt.solid_cleanup_acknowledged
        || !receipt.backdrop_exact
        || receipt.full_visuals_exact
        || receipt.cleanup_reason != Some(reason)
        || receipt.observed_backdrop
            != Some(
                hepta_native::shared::hepta_windows_material_adapter::
                    HeptaWindowsDwmBackdropValue::None,
            )
    {
        return Err(format!(
            "unexpected transient cleanup receipt: {:?}",
            receipt.status
        ));
    }
    Ok(())
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn root_ack_json(receipt: HeptaWindowVisualAckReceipt) -> Value {
    json!({
        "status": format!("{:?}", receipt.status),
        "accepted": receipt.accepted,
        "requestSequence": receipt.request_sequence,
        "windowIndex": receipt.window_index,
        "windowGeneration": receipt.window_generation,
        "readbackScope": format!("{:?}", receipt.readback_scope),
        "observedBackdrop": receipt.observed_backdrop.map(|value| format!("{value:?}")),
        "backdropExact": receipt.backdrop_exact,
        "fullVisualsExact": receipt.full_visuals_exact,
    })
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn transient_ack_json(receipt: HeptaWindowsTransientAckReceipt) -> Value {
    json!({
        "status": format!("{:?}", receipt.status),
        "accepted": receipt.accepted,
        "sequence": receipt.sequence,
        "windowIndex": receipt.window_index,
        "windowGeneration": receipt.window_generation,
        "nativeHandle": receipt.hwnd.to_string(),
        "requestKind": format!("{:?}", receipt.request_kind),
        "requestedBackdrop": format!("{:?}", receipt.requested_visuals.backdrop),
        "observedBackdrop": receipt.observed_backdrop.map(|value| format!("{value:?}")),
        "backdropExact": receipt.backdrop_exact,
        "acrylicBackdropAcknowledged": receipt.acrylic_backdrop_acknowledged,
        "solidCleanupAcknowledged": receipt.solid_cleanup_acknowledged,
        "cleanupReason": receipt.cleanup_reason.map(|value| format!("{value:?}")),
        "fullVisualsExact": false,
        "transientSystemMaterialBound": false,
        "completeProfileBound": false,
        "systemMaterialBound": false,
    })
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn destroy_json(receipt: HeptaWindowsTransientDestroyReceipt) -> Value {
    json!({
        "accepted": receipt.accepted,
        "exactWindowIdentity": receipt.exact_window_identity,
        "cleanupConfirmedBeforeClose": receipt.cleanup_confirmed_before_close,
        "closed": receipt.closed,
        "windowIndex": receipt.window_index,
        "windowGeneration": receipt.window_generation,
        "nativeHandle": receipt.hwnd.to_string(),
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
fn is_git_object_id(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
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
        "hepta-ui-v4-windows-transient-material-probe requires Windows plus the vendored Makepad hook cfg"
    );
}
