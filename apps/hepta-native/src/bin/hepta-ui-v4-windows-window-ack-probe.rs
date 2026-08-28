//! Isolated Windows DWM acknowledgement probe for the vendored Makepad hook.
//!
//! The enabled build requires both Windows and the explicit
//! `hepta_makepad_windows_ack_hook` cfg. It creates one ordinary Makepad window,
//! submits a correlated Mica request followed by a correlated solid-fallback
//! request, consumes the exact backend hook events, performs backdrop-only DWM
//! readback through the existing Hepta verifier, writes one bounded JSON
//! receipt, and exits. It starts no Matrix runtime, bridge, network request,
//! mutation, provider, or production path.

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use std::{fs, path::{Path, PathBuf}};

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
};
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use hepta_native::shared::hepta_windows_material_adapter::HeptaWindowsDwmBackdropApi;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
use serde_json::{json, Value};

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const RECEIPT_SCHEMA: &str = "hepta.ui.v4.windows-dwm-hook-probe.v1";
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const MAKEPAD_REVISION: &str = "c4335cee10b22aca768510c9d072b0ca1bba15c8";
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const MICA_SEQUENCE: u64 = 1;
#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
const SOLID_SEQUENCE: u64 = 2;

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(WindowsAckProbeApp::script_component(vm)) {
        ui: Root {
            probe_window := Window {
                show_caption_bar: true
                window.inner_size: vec2(720, 420)
                window.title: "Hepta UI v4 Windows DWM Hook Probe"
                pass.clear_color: #f4f7fb
                body +: {
                    flow: Down
                    align: Align{x: 0.5, y: 0.5}
                    spacing: 12
                    Label {
                        draw_text.text_style.font_size: 18
                        text: "Hepta Windows DWM acknowledgement probe"
                    }
                    Label {
                        draw_text.text_style.font_size: 14
                        text: "Mica acknowledgement → explicit solid fallback → bounded receipt"
                    }
                }
            }
        }
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
app_main!(WindowsAckProbeApp);

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProbePhase {
    WaitingToQueue,
    WaitingForMica,
    WaitingForSolid,
    Complete,
    Failed,
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl Default for ProbePhase {
    fn default() -> Self {
        Self::WaitingToQueue
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
#[derive(Script)]
struct WindowsAckProbeApp {
    #[live]
    ui: WidgetRef,
    #[rust]
    start_timer: Timer,
    #[rust]
    watchdog_timer: Timer,
    #[rust]
    phase: ProbePhase,
    #[rust]
    window_id: Option<WindowId>,
    #[rust]
    pending_request: Option<HeptaWindowVisualRequestIdentity>,
    #[rust]
    bridge: HeptaWindowsBackendAckBridge,
    #[rust]
    mica_receipt: Option<HeptaWindowVisualAckReceipt>,
    #[rust]
    solid_receipt: Option<HeptaWindowVisualAckReceipt>,
    #[rust]
    native_window_handle: Option<isize>,
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl WindowsAckProbeApp {
    fn receipt_path() -> PathBuf {
        std::env::var_os("HEPTA_WINDOWS_ACK_RECEIPT_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("hepta-ui-v4-windows-dwm-hook-receipt.json"))
    }

    fn queue_request(
        &mut self,
        cx: &mut Cx,
        sequence: u64,
        visuals: WindowVisuals,
        persistent: bool,
    ) -> Result<(), String> {
        if self.pending_request.is_some() {
            return Err("pending request already exists".to_string());
        }
        let window = self.ui.window(cx, ids!(probe_window));
        let window_id = window
            .window_id()
            .filter(|window_id| cx.windows.is_valid(*window_id))
            .ok_or_else(|| "probe WindowId is unavailable".to_string())?;
        if !cx.windows[window_id].is_created {
            return Err("probe window has not been created".to_string());
        }
        if cx.windows[window_id].is_popup {
            return Err("probe window unexpectedly became a popup".to_string());
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
            request_sequence: sequence,
        });

        let request = HeptaWindowVisualRequestIdentity::from_makepad_receipt(
            HeptaMakepadWindowMaterialReceipt {
                generation: sequence,
                platform: HeptaPlatform::Windows,
                window_index: Some(window_id.0),
                window_generation: Some(window_id.1),
                phase: if persistent {
                    HeptaMakepadWindowMaterialPhase::PersistentChromeRequested
                } else {
                    HeptaMakepadWindowMaterialPhase::SolidRequested
                },
                requested_visuals: visuals,
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
            },
        )
        .map_err(|error| format!("request identity rejected: {error:?}"))?;
        self.window_id = Some(window_id);
        self.pending_request = Some(request);
        self.ui.redraw(cx);
        Ok(())
    }

    fn queue_mica(&mut self, cx: &mut Cx) -> Result<(), String> {
        self.queue_request(
            cx,
            MICA_SEQUENCE,
            WindowVisuals {
                transparent: true,
                backdrop: WindowBackdrop::Mica,
                backdrop_intensity: 0.90,
            },
            true,
        )?;
        self.phase = ProbePhase::WaitingForMica;
        Ok(())
    }

    fn queue_solid(&mut self, cx: &mut Cx) -> Result<(), String> {
        self.queue_request(cx, SOLID_SEQUENCE, WindowVisuals::default(), false)?;
        self.phase = ProbePhase::WaitingForSolid;
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
                } => self.process_visuals_event(
                    request_sequence,
                    window_id,
                    native_window_handle,
                    visuals,
                    backend_apply_succeeded,
                    is_popup,
                ),
                WindowsWindowVisualsHookEvent::Destroyed {
                    window_id,
                    native_window_handle,
                } => {
                    if self.window_id == Some(window_id) {
                        Err(format!(
                            "probe window destroyed before completion: {native_window_handle}"
                        ))
                    } else {
                        Ok(None)
                    }
                }
            };

            match result {
                Ok(Some(receipt)) => {
                    if let Err(error) = self.accept_receipt(cx, receipt) {
                        self.fail(cx, error);
                        return;
                    }
                }
                Ok(None) => {}
                Err(error) => {
                    self.fail(cx, error);
                    return;
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn process_visuals_event(
        &mut self,
        request_sequence: u64,
        window_id: WindowId,
        native_window_handle: isize,
        visuals: WindowVisuals,
        backend_apply_succeeded: bool,
        is_popup: bool,
    ) -> Result<Option<HeptaWindowVisualAckReceipt>, String> {
        let request = self
            .pending_request
            .ok_or_else(|| "backend event arrived without a pending request".to_string())?;
        if request_sequence != request.request_sequence() {
            return Err("backend request sequence drifted".to_string());
        }
        if window_id.0 != request.window_index()
            || window_id.1 != request.window_generation()
        {
            return Err("backend WindowId identity drifted".to_string());
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
        .map_err(|error| format!("processed event rejected: {error:?}"))?;

        self.bridge
            .bind_window(processed.identity())
            .map_err(|error| format!("window binding rejected: {error:?}"))?;
        self.bridge
            .register_request(request)
            .map_err(|error| format!("request registration rejected: {error:?}"))?;
        self.pending_request = None;
        self.native_window_handle = Some(native_window_handle);

        let mut api = HeptaWindowsDwmBackdropApi;
        let receipt = self
            .bridge
            .process_backend_event(processed, &mut api)
            .map_err(|error| format!("DWM observation rejected: {error:?}"))?;
        Ok(Some(receipt))
    }

    fn accept_receipt(
        &mut self,
        cx: &mut Cx,
        receipt: HeptaWindowVisualAckReceipt,
    ) -> Result<(), String> {
        if !receipt.accepted || !receipt.grants_no_authority() || !receipt.remains_partial() {
            return Err(format!("receipt escaped partial boundary: {:?}", receipt.status));
        }
        match self.phase {
            ProbePhase::WaitingForMica => {
                if receipt.status
                    != HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback
                    || !receipt.backdrop_exact
                    || receipt.full_visuals_exact
                    || receipt.observed_backdrop != Some(WindowBackdrop::Mica)
                {
                    return Err(format!("unexpected Mica receipt: {:?}", receipt.status));
                }
                self.mica_receipt = Some(receipt);
                self.queue_solid(cx)
            }
            ProbePhase::WaitingForSolid => {
                if receipt.status
                    != HeptaWindowVisualAckStatus::VerifiedSolidFallbackWithBackdropReadback
                    || !receipt.backdrop_exact
                    || receipt.full_visuals_exact
                    || receipt.observed_backdrop != Some(WindowBackdrop::None)
                {
                    return Err(format!("unexpected solid receipt: {:?}", receipt.status));
                }
                self.solid_receipt = Some(receipt);
                self.complete(cx)
            }
            phase => Err(format!("receipt arrived in invalid phase: {phase:?}")),
        }
    }

    fn complete(&mut self, cx: &mut Cx) -> Result<(), String> {
        let mica = self.mica_receipt.ok_or_else(|| "missing Mica receipt".to_string())?;
        let solid = self.solid_receipt.ok_or_else(|| "missing solid receipt".to_string())?;
        let candidate_commit = std::env::var("HEPTA_CANDIDATE_COMMIT")
            .unwrap_or_else(|_| "UNBOUND".to_string());
        let candidate_tree = std::env::var("HEPTA_CANDIDATE_TREE")
            .unwrap_or_else(|_| "UNBOUND".to_string());
        if !is_git_object_id(&candidate_commit) || !is_git_object_id(&candidate_tree) {
            return Err("candidate commit/tree are not exact Git object IDs".to_string());
        }
        let window_id = self.window_id.ok_or_else(|| "missing WindowId".to_string())?;
        let native_window_handle = self
            .native_window_handle
            .filter(|handle| *handle != 0)
            .ok_or_else(|| "missing explicit HWND".to_string())?;
        let receipt = json!({
            "schema": RECEIPT_SCHEMA,
            "status": "PASS_WINDOWS_DWM_HOOK_PROBE",
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
            "window": {
                "index": window_id.0,
                "generation": window_id.1,
                "nativeHandle": native_window_handle.to_string(),
                "nativeHandleNonzero": true,
                "popup": false,
            },
            "requests": {
                "mica": ack_json(mica),
                "solidFallback": ack_json(solid),
            },
            "qualification": {
                "patchedMakepadRuntime": true,
                "correlatedHookRuntime": true,
                "exactHwndFromFramework": true,
                "persistentChromeBackdropAcknowledged": true,
                "solidFallbackAcknowledged": true,
                "backdropReadback": true,
                "fullVisualReadback": false,
                "transientSystemMaterial": false,
                "completeProfile": false,
                "systemMaterialBinding": false,
                "nativeProductRuntime": false,
                "deviceValidation": false,
            },
            "authority": authority_json(),
        });
        write_json_atomic(&Self::receipt_path(), &receipt)
            .map_err(|error| format!("write receipt: {error}"))?;
        self.phase = ProbePhase::Complete;
        cx.quit();
        Ok(())
    }

    fn fail(&mut self, cx: &mut Cx, error: String) {
        if matches!(self.phase, ProbePhase::Complete | ProbePhase::Failed) {
            return;
        }
        let receipt = json!({
            "schema": RECEIPT_SCHEMA,
            "status": "FAIL_WINDOWS_DWM_HOOK_PROBE",
            "error": error,
            "candidate": {
                "commit": std::env::var("HEPTA_CANDIDATE_COMMIT").unwrap_or_else(|_| "UNBOUND".to_string()),
                "tree": std::env::var("HEPTA_CANDIDATE_TREE").unwrap_or_else(|_| "UNBOUND".to_string()),
            },
            "makepad": {
                "revision": MAKEPAD_REVISION,
                "vendoredPatchBuild": true,
                "defaultDependencySwitched": false,
            },
            "fixture": true,
            "qualification": {
                "patchedMakepadRuntime": false,
                "correlatedHookRuntime": false,
                "exactHwndFromFramework": false,
                "persistentChromeBackdropAcknowledged": false,
                "solidFallbackAcknowledged": false,
                "backdropReadback": false,
                "fullVisualReadback": false,
                "transientSystemMaterial": false,
                "completeProfile": false,
                "systemMaterialBinding": false,
                "nativeProductRuntime": false,
                "deviceValidation": false,
            },
            "authority": authority_json(),
        });
        let _ = write_json_atomic(&Self::receipt_path(), &receipt);
        self.phase = ProbePhase::Failed;
        cx.quit();
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl MatchEvent for WindowsAckProbeApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.start_timer = cx.start_timeout(0.75);
        self.watchdog_timer = cx.start_timeout(30.0);
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
impl AppMain for WindowsAckProbeApp {
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
            if let Err(error) = self.queue_mica(cx) {
                self.fail(cx, error);
                return;
            }
        }
        if self.watchdog_timer.is_event(event).is_some() {
            self.watchdog_timer = Timer::empty();
            self.fail(cx, "probe timed out before both acknowledgements".to_string());
            return;
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

#[cfg(all(target_os = "windows", hepta_makepad_windows_ack_hook))]
fn ack_json(receipt: HeptaWindowVisualAckReceipt) -> Value {
    json!({
        "status": format!("{:?}", receipt.status),
        "accepted": receipt.accepted,
        "requestSequence": receipt.request_sequence,
        "windowIndex": receipt.window_index,
        "windowGeneration": receipt.window_generation,
        "backend": format!("{:?}", receipt.backend),
        "requestedBackdrop": format!("{:?}", receipt.requested_visuals.backdrop),
        "readbackScope": format!("{:?}", receipt.readback_scope),
        "observedBackdrop": receipt.observed_backdrop.map(|value| format!("{value:?}")),
        "backdropExact": receipt.backdrop_exact,
        "fullVisualsExact": receipt.full_visuals_exact,
        "persistentChromeAcknowledged": receipt.persistent_chrome_acknowledged,
        "solidFallbackAcknowledged": receipt.solid_fallback_acknowledged,
        "transientSystemMaterialBound": false,
        "completeProfileBound": false,
        "systemMaterialBound": false,
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
        "hepta-ui-v4-windows-window-ack-probe requires Windows plus the vendored Makepad hook cfg"
    );
}
