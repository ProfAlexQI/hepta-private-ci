//! Isolated Native component probe for the Hepta UI v4 room filter.
//!
//! This binary has no Matrix startup, bridge activation, network request,
//! mutation, or production path. It renders the real shared
//! `RoomFilterInputBar`, measures the actual Makepad areas, optionally captures
//! a rendered frame, writes one bounded JSON receipt, and exits.

use std::{
    fs,
    path::{Path, PathBuf},
};

use hepta_native::makepad_widgets::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const RECEIPT_SCHEMA: &str = "hepta.ui.v4.native-component-metrics.v1";
const COMPONENT_ID: &str = "room-filter";
const MIN_TOUCH_TARGET: f64 = 48.0;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(FilterProbeApp::script_component(vm)) {
        ui: Root {
            probe_window := Window {
                show_caption_bar: false
                window.inner_size: vec2(390, 180)
                window.title: "Hepta UI v4 Filter Probe"
                pass.clear_color: (mod.widgets.COLOR_HEPTA_ENVIRONMENT)

                body +: {
                    flow: Down
                    align: Align{x: 0.5, y: 0.5}
                    padding: 16

                    probe_filter := mod.widgets.RoomFilterInputBar {
                        width: Fill
                    }
                }
            }
        }
    }
}

app_main!(FilterProbeApp);

#[derive(Script)]
struct FilterProbeApp {
    #[live]
    ui: WidgetRef,
    #[rust]
    capture_timer: Timer,
    #[rust]
    finish_timer: Timer,
    #[rust]
    pending_receipt: Option<Value>,
}

impl FilterProbeApp {
    fn prepare_component(&mut self, cx: &mut Cx) {
        let filter = self.ui.widget(cx, ids!(probe_filter));
        filter
            .text_input(cx, ids!(input))
            .set_text(cx, "Hepta diagnostics");
        filter
            .button(cx, ids!(clear_button))
            .set_visible(cx, true);
        self.ui.redraw(cx);
        self.capture_timer = cx.start_timeout(0.35);
    }

    fn capture_metrics(&mut self, cx: &mut Cx) {
        let filter = self.ui.widget(cx, ids!(probe_filter));
        let input = filter.text_input(cx, ids!(input));
        let clear = filter.button(cx, ids!(clear_button));
        let filter_rect = filter.area().rect(cx);
        let input_rect = input.area().rect(cx);
        let clear_rect = clear.area().rect(cx);
        let window = self.ui.window(cx, ids!(probe_window));
        let viewport = window.get_inner_size(cx);
        let dpi_factor = window
            .window_id()
            .filter(|window_id| cx.windows.is_valid(*window_id))
            .map(|window_id| cx.windows[window_id].effective_dpi_factor())
            .unwrap_or(1.0);

        let candidate_commit = std::env::var("HEPTA_CANDIDATE_COMMIT")
            .unwrap_or_else(|_| "UNBOUND".to_string());
        let candidate_tree = std::env::var("HEPTA_CANDIDATE_TREE")
            .unwrap_or_else(|_| "UNBOUND".to_string());
        let checks = json!({
            "candidateCommitBound": is_git_object_id(&candidate_commit),
            "candidateTreeBound": is_git_object_id(&candidate_tree),
            "filterHeightAtLeast48": filter_rect.size.y >= MIN_TOUCH_TARGET,
            "inputHeightAtLeast48": input_rect.size.y >= MIN_TOUCH_TARGET,
            "clearWidthAtLeast48": clear_rect.size.x >= MIN_TOUCH_TARGET,
            "clearHeightAtLeast48": clear_rect.size.y >= MIN_TOUCH_TARGET,
            "filterWithinViewport": rect_inside_viewport(filter_rect, viewport),
            "clearVisible": clear.visible(),
        });
        let metrics_pass = checks
            .as_object()
            .expect("checks object")
            .values()
            .all(|value| value == &Value::Bool(true));
        let capture_path = std::env::var_os("HEPTA_NATIVE_CAPTURE_FRAME_PATH")
            .map(PathBuf::from);

        self.pending_receipt = Some(json!({
            "schema": RECEIPT_SCHEMA,
            "status": if metrics_pass {
                "METRICS_PASS_SCREENSHOT_PENDING"
            } else {
                "FAIL_NATIVE_FILTER_COMPONENT_METRICS"
            },
            "candidate": {
                "commit": candidate_commit,
                "tree": candidate_tree,
            },
            "fixture": true,
            "component": COMPONENT_ID,
            "viewport": {
                "width": viewport.x,
                "height": viewport.y,
            },
            "dpiFactor": dpi_factor,
            "rects": {
                "filter": rect_json(filter_rect),
                "input": rect_json(input_rect),
                "clear": rect_json(clear_rect),
            },
            "checks": checks,
            "screenshot": {
                "requested": capture_path.is_some(),
                "present": false,
                "bytes": 0,
                "sha256": Value::Null,
            },
            "qualification": {
                "componentMetrics": metrics_pass,
                "componentScreenshot": false,
                "nativeProductRuntime": false,
                "systemMaterialBinding": false,
                "deviceMatrix": false,
            },
            "authority": {
                "network": false,
                "mutation": false,
                "effect": false,
                "liveAdapter": false,
                "production": false,
                "operatorAcceptance": false,
                "promotion": false,
                "release": false,
            }
        }));

        if let Some(path) = capture_path {
            ensure_parent(&path).expect("create screenshot parent");
            cx.capture_next_frame_to_file(path);
            self.ui.redraw(cx);
        }
        self.finish_timer = cx.start_timeout(1.0);
    }

    fn finish(&mut self, cx: &mut Cx) {
        let mut receipt = self
            .pending_receipt
            .take()
            .unwrap_or_else(|| json!({
                "schema": RECEIPT_SCHEMA,
                "status": "FAIL_METRICS_NOT_CAPTURED"
            }));

        let screenshot = std::env::var_os("HEPTA_NATIVE_CAPTURE_FRAME_PATH")
            .map(PathBuf::from)
            .and_then(|path| screenshot_evidence(&path).ok());
        if let Some(screenshot) = screenshot {
            let present = screenshot["present"] == Value::Bool(true);
            receipt["screenshot"] = screenshot;
            receipt["qualification"]["componentScreenshot"] = Value::Bool(present);
        }

        let metrics_pass = receipt["qualification"]["componentMetrics"] == Value::Bool(true);
        let screenshot_requested = receipt["screenshot"]["requested"] == Value::Bool(true);
        let screenshot_pass = receipt["qualification"]["componentScreenshot"] == Value::Bool(true);
        receipt["status"] = Value::String(
            if metrics_pass && (!screenshot_requested || screenshot_pass) {
                "PASS_NATIVE_FILTER_COMPONENT_METRICS"
            } else {
                "FAIL_NATIVE_FILTER_COMPONENT_METRICS"
            }
            .to_string(),
        );

        let receipt_path = std::env::var_os("HEPTA_NATIVE_UI_METRICS_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("hepta-ui-v4-filter-metrics.json"));
        write_json_atomic(&receipt_path, &receipt).expect("write metrics receipt");
        cx.quit();
    }
}

impl MatchEvent for FilterProbeApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.prepare_component(cx);
    }
}

impl AppMain for FilterProbeApp {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::theme_mod(vm);
        script_eval!(vm, {
            mod.theme = mod.themes.light
        });
        makepad_widgets::widgets_mod(vm);
        hepta_native::shared::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if self.capture_timer.is_event(event).is_some() {
            self.capture_timer = Timer::empty();
            self.capture_metrics(cx);
        }
        if self.finish_timer.is_event(event).is_some() {
            self.finish_timer = Timer::empty();
            self.finish(cx);
            return;
        }

        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn is_git_object_id(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn rect_inside_viewport(rect: Rect, viewport: Vec2d) -> bool {
    rect.pos.x >= 0.0
        && rect.pos.y >= 0.0
        && rect.pos.x + rect.size.x <= viewport.x + 0.5
        && rect.pos.y + rect.size.y <= viewport.y + 0.5
}

fn rect_json(rect: Rect) -> Value {
    json!({
        "x": rect.pos.x,
        "y": rect.pos.y,
        "width": rect.size.x,
        "height": rect.size.y,
    })
}

fn screenshot_evidence(path: &Path) -> std::io::Result<Value> {
    let bytes = fs::read(path)?;
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    Ok(json!({
        "requested": true,
        "present": !bytes.is_empty(),
        "bytes": bytes.len(),
        "sha256": sha256,
    }))
}

fn ensure_parent(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn write_json_atomic(path: &Path, value: &Value) -> std::io::Result<()> {
    ensure_parent(path)?;
    let temp_path = path.with_extension("tmp");
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    fs::write(&temp_path, bytes)?;
    fs::rename(temp_path, path)
}
