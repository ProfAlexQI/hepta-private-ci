mod generated_control_ui_assets {
    include!(concat!(env!("OUT_DIR"), "/control_ui_bundle_metadata.rs"));
}

pub const CONTROL_UI_HEPTA_AGENT_LOGO_PNG_SHA256: &str =
    "eeaa030d321f566157055ae07d5f540bf06baeaaa6a2e6209227eb1bf6d3f41c";
pub const CONTROL_UI_HEPTA_AGENT_LOGO_PNG: &[u8] =
    include_bytes!("../../../apps/hepta-control-ui/assets/hepta-agent-logo.png");

/// Individually embedded sources remain visible for provenance and tests, but
/// only `CONTROL_UI_JS` is served by the gateway.
pub const CONTROL_UI_BASE_JS: &[u8] =
    include_bytes!("../../../apps/hepta-control-ui/control-ui.js");
pub const CONTROL_UI_V4_RUNTIME_JS: &[u8] =
    include_bytes!("../../../apps/hepta-control-ui/control-ui-v4-runtime.js");
pub const CONTROL_UI_BASE_JS_SHA256: &str =
    generated_control_ui_assets::CONTROL_UI_BASE_JS_SHA256;
pub const CONTROL_UI_V4_RUNTIME_JS_SHA256: &str =
    generated_control_ui_assets::CONTROL_UI_V4_RUNTIME_JS_SHA256;
pub const CONTROL_UI_JS_SHA256: &str = generated_control_ui_assets::CONTROL_UI_JS_SHA256;
pub const CONTROL_UI_JS_ETAG: &str = generated_control_ui_assets::CONTROL_UI_JS_ETAG;
pub const CONTROL_UI_V4_RUNTIME_BOUND: bool =
    generated_control_ui_assets::CONTROL_UI_V4_RUNTIME_BOUND;
pub const CONTROL_UI_JS: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/control-ui.bundle.js"));
