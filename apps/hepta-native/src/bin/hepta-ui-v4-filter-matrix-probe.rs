//! Isolated multi-scenario Native component probe for Hepta UI v4.
//!
//! The binary renders the canonical shared `RoomFilterInputBar`, applies one
//! explicitly disclosed component-layout scenario, measures actual Makepad
//! areas, optionally captures one rendered frame, writes a bounded JSON receipt,
//! and exits. It starts no Matrix runtime, Hepta bridge, network, mutation,
//! provider, effect, production, or release path.

use std::{
    fs,
    path::{Path, PathBuf},
};

use hepta_native::makepad_widgets::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const RECEIPT_SCHEMA: &str = "hepta.ui.v4.native-component-scenario.v1";
const COMPONENT_ID: &str = "room-filter";
const MIN_TOUCH_TARGET: f64 = 48.0;
const BASE_PADDING: f64 = 16.0;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(FilterMatrixProbeApp::script_component(vm)) {
        ui: Root {
            probe_window := Window {
                show_caption_bar: false
                window.inner_size: vec2(390, 844)
                window.title: "Hepta UI v4 Filter Matrix Probe"
                pass.clear_color: (mod.widgets.COLOR_HEPTA_ENVIRONMENT)

                body +: {
                    probe_shell := View {
                        width: Fill
                        height: Fill
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
}

app_main!(FilterMatrixProbeApp);

#[derive(Clone, Debug)]
struct ProbeConfig {
    scenario: String,
    width: f64,
    height: f64,
    safe_top: f64,
    safe_right: f64,
    safe_bottom: f64,
    safe_left: f64,
    keyboard_inset: f64,
    ui_scale: f64,
}

impl ProbeConfig {
    fn from_env() -> Result<Self, String> {
        Ok(Self {
            scenario: required_env("HEPTA_PROBE_SCENARIO")?,
            width: positive_env("HEPTA_PROBE_WIDTH")?,
            height: positive_env("HEPTA_PROBE_HEIGHT")?,
            safe_top: nonnegative_env("HEPTA_PROBE_SAFE_TOP")?,
            safe_right: nonnegative_env("HEPTA_PROBE_SAFE_RIGHT")?,
            safe_bottom: nonnegative_env("HEPTA_PROBE_SAFE_BOTTOM")?,
            safe_left: nonnegative_env("HEPTA_PROBE_SAFE_LEFT")?,
            keyboard_inset: nonnegative_env("HEPTA_PROBE_KEYBOARD_INSET")?,
            ui_scale: positive_env("HEPTA_PROBE_UI_SCALE")?,
        })
    }

    fn synthetic(&self) -> bool {
        self.safe_top > 0.0
            || self.safe_right > 0.0
            || self.safe_bottom > 0.0
            || self.safe_left > 0.0
            || self.keyboard_inset > 0.0
            || (self.ui_scale - 1.0).abs() > f64::EPSILON
    }

    fn safe_area(&self) -> SafeAreaInsets {
        SafeAreaInsets {
            top: self.safe_top,
            right: self.safe_right,
            bottom: self.safe_bottom,
            left: self.safe_left,
        }
    }
}

#[derive(Script)]
struct FilterMatrixProbeApp {
    #[live]
    ui: WidgetRef,
    #[rust]
    config: Option<ProbeConfig>,
    #[rust]
    capture_timer: Timer,
    #[rust]
    finish_timer: Timer,
    #[rust]
    pending_receipt: Option<Value>,
}

impl FilterMatrixProbeApp {
    fn prepare_component(&mut self, cx: &mut Cx) {
        let config = ProbeConfig::from_env().expect("valid bounded probe configuration");
        let window = self.ui.window(cx, ids!(probe_window));
        window.configure_window(
            cx,
            dvec2(config.width, config.height),
            dvec2(32.0, 32.0),
            false,
            format!("Hepta UI v4 — {}", config.scenario),
        );

        if let Some(window_id) = window
            .window_id()
            .filter(|window_id| cx.windows.is_valid(*window_id))
        {
            if (config.ui_scale - 1.0).abs() > f64::EPSILON {
                cx.set_window_dpi_override(window_id, Some(config.ui_scale));
            }
            cx.windows[window_id].window_geom.safe_area_insets = config.safe_area();
        }

        let shell = self.ui.view(cx, ids!(probe_shell));
        if let Some(mut shell) = shell.borrow_mut() {
            shell.layout.padding = Inset {
                top: BASE_PADDING + config.safe_top,
                right: BASE_PADDING + config.safe_right,
                bottom: BASE_PADDING + config.safe_bottom + config.keyboard_inset,
                left: BASE_PADDING + config.safe_left,
            };
        }

        let filter = self.ui.widget(cx, ids!(probe_filter));
        filter
            .text_input(cx, ids!(input))
            .set_text(cx, "Hepta diagnostics");
        filter.button(cx, ids!(clear_button)).set_visible(cx, true);
        self.config = Some(config);
        self.ui.redraw(cx);
        self.capture_timer = cx.start_timeout(0.6);
    }

    fn capture_metrics(&mut self, cx: &mut Cx) {
        let config = self.config.clone().expect("probe configuration");
        let filter = self.ui.widget(cx, ids!(probe_filter));
        let input = filter.text_input(cx, ids!(input));
        let clear = filter.button(cx, ids!(clear_button));
        let filter_rect = filter.area().rect(cx);
        let input_rect = input.area().rect(cx);
        let clear_rect = clear.area().rect(cx);
        let window = self.ui.window(cx, ids!(probe_window));
        let window_id = window
            .window_id()
            .filter(|window_id| cx.windows.is_valid(*window_id));
        let (viewport, dpi_factor, exact_window) = window_id
            .map(|window_id| {
                let window = &cx.windows[window_id];
                (
                    window.get_inner_size(),
                    window.effective_dpi_factor(),
                    json!({
                        "index": window_id.0,
                        "generation": window_id.1,
                    }),
                )
            })
            .unwrap_or((Vec2d::default(), 1.0, Value::Null));

        let candidate_commit =
            std::env::var("HEPTA_CANDIDATE_COMMIT").unwrap_or_else(|_| "UNBOUND".to_string());
        let candidate_tree =
            std::env::var("HEPTA_CANDIDATE_TREE").unwrap_or_else(|_| "UNBOUND".to_string());
        let available_bottom = viewport.y - config.safe_bottom - config.keyboard_inset + 0.5;
        let checks = json!({
            "scenarioBound": !config.scenario.trim().is_empty(),
            "candidateCommitBound": is_git_object_id(&candidate_commit),
            "candidateTreeBound": is_git_object_id(&candidate_tree),
            "exactWindowIdBound": exact_window != Value::Null,
            "viewportPositive": viewport.x > 0.0 && viewport.y > 0.0,
            "filterHeightAtLeast48": filter_rect.size.y >= MIN_TOUCH_TARGET,
            "inputHeightAtLeast48": input_rect.size.y >= MIN_TOUCH_TARGET,
            "clearWidthAtLeast48": clear_rect.size.x >= MIN_TOUCH_TARGET,
            "clearHeightAtLeast48": clear_rect.size.y >= MIN_TOUCH_TARGET,
            "filterWithinViewport": rect_inside_viewport(filter_rect, viewport),
            "safeAreaAndKeyboardRespected":
                filter_rect.pos.x + 0.5 >= config.safe_left
                && filter_rect.pos.y + 0.5 >= config.safe_top
                && filter_rect.pos.x + filter_rect.size.x <= viewport.x - config.safe_right + 0.5
                && filter_rect.pos.y + filter_rect.size.y <= available_bottom,
            "uiScaleApplied": if (config.ui_scale - 1.0).abs() <= f64::EPSILON {
                dpi_factor > 0.0
            } else {
                (dpi_factor - config.ui_scale).abs() <= 0.05
            },
            "clearVisible": clear.visible(),
        });
        let metrics_pass = checks
            .as_object()
            .expect("checks object")
            .values()
            .all(|value| value == &Value::Bool(true));
        let capture_path = std::env::var_os("HEPTA_NATIVE_CAPTURE_FRAME_PATH").map(PathBuf::from);

        self.pending_receipt = Some(json!({
            "schema": RECEIPT_SCHEMA,
            "status": if metrics_pass {
                "METRICS_PASS_SCREENSHOT_PENDING"
            } else {
                "FAIL_NATIVE_COMPONENT_SCENARIO"
            },
            "candidate": {
                "commit": candidate_commit,
                "tree": candidate_tree,
            },
            "fixture": true,
            "component": COMPONENT_ID,
            "scenario": config.scenario,
            "environment": {
                "synthetic": config.synthetic(),
                "requestedViewport": {
                    "width": config.width,
                    "height": config.height,
                },
                "safeArea": {
                    "top": config.safe_top,
                    "right": config.safe_right,
                    "bottom": config.safe_bottom,
                    "left": config.safe_left,
                },
                "keyboardInset": config.keyboard_inset,
                "uiScale": config.ui_scale,
            },
            "windowId": exact_window,
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
                "scenarioMetrics": metrics_pass,
                "scenarioScreenshot": false,
                "componentMatrix": false,
                "nativeProductRuntime": false,
                "systemMaterialBinding": false,
                "deviceMatrix": false,
            },
            "authority": false_authority(),
        }));

        if let Some(path) = capture_path {
            ensure_parent(&path).expect("create screenshot parent");
            cx.capture_next_frame_to_file(path);
            self.ui.redraw(cx);
        }
        self.finish_timer = cx.start_timeout(1.2);
    }

    fn finish(&mut self, cx: &mut Cx) {
        let mut receipt = self.pending_receipt.take().unwrap_or_else(|| {
            json!({
                "schema": RECEIPT_SCHEMA,
                "status": "FAIL_METRICS_NOT_CAPTURED",
                "authority": false_authority(),
            })
        });

        let screenshot = std::env::var_os("HEPTA_NATIVE_CAPTURE_FRAME_PATH")
            .map(PathBuf::from)
            .and_then(|path| screenshot_evidence(&path).ok());
        if let Some(screenshot) = screenshot {
            let present = screenshot["present"] == Value::Bool(true);
            receipt["screenshot"] = screenshot;
            receipt["qualification"]["scenarioScreenshot"] = Value::Bool(present);
        }

        let metrics_pass = receipt["qualification"]["scenarioMetrics"] == Value::Bool(true);
        let screenshot_requested = receipt["screenshot"]["requested"] == Value::Bool(true);
        let screenshot_pass = receipt["qualification"]["scenarioScreenshot"] == Value::Bool(true);
        receipt["status"] = Value::String(
            if metrics_pass && (!screenshot_requested || screenshot_pass) {
                "PASS_NATIVE_COMPONENT_SCENARIO"
            } else {
                "FAIL_NATIVE_COMPONENT_SCENARIO"
            }
            .to_string(),
        );

        let receipt_path = std::env::var_os("HEPTA_NATIVE_UI_SCENARIO_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("hepta-ui-v4-component-scenario.json"));
        write_json_atomic(&receipt_path, &receipt).expect("write scenario receipt");
        cx.quit();
    }
}

impl MatchEvent for FilterMatrixProbeApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.prepare_component(cx);
    }
}

impl AppMain for FilterMatrixProbeApp {
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

fn required_env(name: &str) -> Result<String, String> {
    let value = std::env::var(name).map_err(|_| format!("missing {name}"))?;
    if value.trim().is_empty() {
        Err(format!("empty {name}"))
    } else {
        Ok(value)
    }
}

fn positive_env(name: &str) -> Result<f64, String> {
    let value = required_env(name)?
        .parse::<f64>()
        .map_err(|error| format!("invalid {name}: {error}"))?;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!("{name} must be finite and positive"))
    }
}

fn nonnegative_env(name: &str) -> Result<f64, String> {
    let value = required_env(name)?
        .parse::<f64>()
        .map_err(|error| format!("invalid {name}: {error}"))?;
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(format!("{name} must be finite and nonnegative"))
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
    Ok(json!({
        "requested": true,
        "present": !bytes.is_empty(),
        "bytes": bytes.len(),
        "sha256": format!("{:x}", Sha256::digest(&bytes)),
    }))
}

fn false_authority() -> Value {
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
