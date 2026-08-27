use hepta_core::control_ui::CONTROL_UI_BASE_JS_SHA256;
use hepta_core::control_ui::CONTROL_UI_JS;
use hepta_core::control_ui::CONTROL_UI_JS_ETAG;
use hepta_core::control_ui::CONTROL_UI_JS_SHA256;
use hepta_core::control_ui::CONTROL_UI_V4_RUNTIME_BOUND;
use hepta_core::control_ui::CONTROL_UI_V4_RUNTIME_JS_SHA256;

#[test]
fn control_ui_v4_runtime_is_bound_into_the_single_served_bundle() {
    assert!(CONTROL_UI_V4_RUNTIME_BOUND);
    assert_eq!(CONTROL_UI_BASE_JS_SHA256.len(), 64);
    assert_eq!(CONTROL_UI_V4_RUNTIME_JS_SHA256.len(), 64);
    assert_eq!(CONTROL_UI_JS_SHA256.len(), 64);
    assert_ne!(CONTROL_UI_BASE_JS_SHA256, CONTROL_UI_JS_SHA256);
    assert_ne!(CONTROL_UI_V4_RUNTIME_JS_SHA256, CONTROL_UI_JS_SHA256);
    assert_eq!(
        CONTROL_UI_JS_ETAG,
        format!("\"sha256-{CONTROL_UI_JS_SHA256}\"")
    );

    let Ok(bundle) = std::str::from_utf8(CONTROL_UI_JS) else {
        panic!("Control UI JavaScript bundle must remain UTF-8");
    };
    let Some(base_position) = bundle.find("const COMMAND_CATALOG") else {
        panic!("base Control UI controller is missing from the bundle");
    };
    let Some(boundary_position) = bundle.find("hepta-ui-v4-runtime-bundle-boundary") else {
        panic!("v4 runtime bundle boundary is missing");
    };
    let Some(runtime_position) = bundle.find("HeptaUiV4ReadState") else {
        panic!("v4 runtime controller is missing from the bundle");
    };

    assert!(base_position < boundary_position);
    assert!(boundary_position < runtime_position);
    assert!(bundle.contains("controlUiV4Runtime = \"ready\""));
    assert!(bundle.contains("controlUiV4RuntimeAuthority = \"local-ui-only\""));
}
