//! Hepta's semantic light-glass palette.
//!
//! Message and content surfaces stay nearly opaque. Translucent color,
//! hairlines, and soft highlights are reserved for navigation, the composer,
//! and floating controls so the interface still reads as a chat product.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.COLOR_HEPTA_ENVIRONMENT = #xEEF3F5FF
    mod.widgets.COLOR_HEPTA_CONTENT = #xFCFDFEFF
    mod.widgets.COLOR_HEPTA_SURFACE = #xF7FAFBFA
    mod.widgets.COLOR_HEPTA_GLASS = #xEAF1F3E8
    mod.widgets.COLOR_HEPTA_GLASS_STRONG = #xF5F8F9EE
    mod.widgets.COLOR_HEPTA_INPUT = #xF8FAFBF8

    mod.widgets.COLOR_HEPTA_TEXT = #x142A32FF
    mod.widgets.COLOR_HEPTA_MUTED = #x566A78FF
    // This is also used for placeholders and timestamps. Keep it at >= 4.5:1
    // against the lightest Hepta surfaces instead of treating it as decoration.
    mod.widgets.COLOR_HEPTA_DIM = #x5C6E79FF

    mod.widgets.COLOR_HEPTA_HAIRLINE = #x8FB4BF66
    mod.widgets.COLOR_HEPTA_HAIRLINE_STRONG = #x6F9EAA8F
    mod.widgets.COLOR_HEPTA_FOCUS = #x0F7290FF
    mod.widgets.COLOR_HEPTA_FOCUS_HOVER = #x0B5D78FF
    mod.widgets.COLOR_HEPTA_FOCUS_SURFACE = #xDCEFF4E8
    mod.widgets.COLOR_HEPTA_FOCUS_SURFACE_HOVER = #xD0E8EEF2
    mod.widgets.COLOR_HEPTA_SELECTION = #x0F729026

    mod.widgets.COLOR_HEPTA_SUCCESS = #x137A5AFF
    mod.widgets.COLOR_HEPTA_SUCCESS_SURFACE = #xE7F6EFF2
    mod.widgets.COLOR_HEPTA_DANGER = #xB63845FF
    mod.widgets.COLOR_HEPTA_DANGER_SURFACE = #xFCEDEFF2
    mod.widgets.COLOR_HEPTA_WARNING = #x9A6500FF
    mod.widgets.COLOR_HEPTA_DISABLED = #xB9C4C9FF
    mod.widgets.COLOR_HEPTA_DISABLED_SURFACE = #xE6ECEEFF
    mod.widgets.COLOR_HEPTA_SHADOW = #x17304714

    mod.widgets.HEPTA_RADIUS_CONTROL = 10.0
    mod.widgets.HEPTA_RADIUS_PANEL = 14.0
    mod.widgets.HEPTA_RADIUS_FLOATING = 18.0
    mod.widgets.HEPTA_LAYER_STABLE_ALPHA = 0.98
    mod.widgets.HEPTA_LAYER_STABLE_BLUR = 0.0
    mod.widgets.HEPTA_LAYER_CHROME_ALPHA = 0.88
    mod.widgets.HEPTA_LAYER_CHROME_BLUR = 14.0
    mod.widgets.HEPTA_LAYER_FLOATING_ALPHA = 0.94
    mod.widgets.HEPTA_LAYER_FLOATING_BLUR = 20.0
}

pub const COLOR_HEPTA_CONTENT: Vec4 = vec4(0.988, 0.992, 0.996, 1.0);
pub const COLOR_HEPTA_FOCUS: Vec4 = vec4(0.059, 0.447, 0.565, 1.0);
pub const COLOR_HEPTA_FOCUS_HOVER: Vec4 = vec4(0.043, 0.365, 0.471, 1.0);
pub const COLOR_HEPTA_SUCCESS: Vec4 = vec4(0.075, 0.478, 0.353, 1.0);
