//! Hepta UI v4 semantic overrides.
//!
//! This module is intentionally loaded after the legacy shared style graph. It
//! raises the product typography floor without rewriting the upstream Robrix
//! widget tree and publishes the material/interaction contract needed by new
//! components. It does not grant a live adapter, effect, or production authority.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // Source-bound implementation markers.
    mod.widgets.HEPTA_UI_V4_ENABLED = true
    mod.widgets.HEPTA_UI_V4_PRODUCTION_AUTHORITY = false
    mod.widgets.HEPTA_UI_V4_EFFECT_AUTHORITY = false
    mod.widgets.HEPTA_UI_V4_LIVE_ADAPTER_AUTHORITY = false
    mod.widgets.HEPTA_UI_V4_OPERATOR_ACCEPTANCE = false
    mod.widgets.HEPTA_UI_V4_PROMOTION = false

    // Material roles. Stable content never uses a backdrop blur.
    mod.widgets.HEPTA_V4_MATERIAL_ENVIRONMENT = 0
    mod.widgets.HEPTA_V4_MATERIAL_CONTENT = 1
    mod.widgets.HEPTA_V4_MATERIAL_CHROME = 2
    mod.widgets.HEPTA_V4_MATERIAL_TRANSIENT = 3
    mod.widgets.HEPTA_V4_MATERIAL_FALLBACK = 4

    mod.widgets.HEPTA_V4_CONTENT_ALPHA = 0.98
    mod.widgets.HEPTA_V4_CONTENT_BLUR = 0.0
    mod.widgets.HEPTA_V4_CHROME_ALPHA = 0.90
    mod.widgets.HEPTA_V4_CHROME_BLUR = 12.0
    mod.widgets.HEPTA_V4_TRANSIENT_ALPHA = 0.96
    mod.widgets.HEPTA_V4_TRANSIENT_BLUR = 18.0
    mod.widgets.HEPTA_V4_FALLBACK_ALPHA = 1.0
    mod.widgets.HEPTA_V4_FALLBACK_BLUR = 0.0
    mod.widgets.HEPTA_V4_MAX_VISIBLE_BACKDROP_LAYERS = 2
    mod.widgets.HEPTA_V4_MAX_STABLE_CONTENT_BACKDROP_LAYERS = 0

    // Typography floor. These compatibility names are consumed throughout the
    // existing Makepad tree; loading this module after `styles` upgrades them in
    // one bounded place.
    mod.widgets.HEPTA_V4_FONT_DISPLAY = 24
    mod.widgets.HEPTA_V4_FONT_TITLE = 18
    mod.widgets.HEPTA_V4_FONT_BODY = 15
    mod.widgets.HEPTA_V4_FONT_MESSAGE = 15
    mod.widgets.HEPTA_V4_FONT_LABEL = 13
    mod.widgets.HEPTA_V4_FONT_METADATA = 12
    mod.widgets.HEPTA_V4_FONT_MICRO = 11

    mod.widgets.TITLE_TEXT = theme.font_regular {
        font_size: (mod.widgets.HEPTA_V4_FONT_TITLE),
    }

    mod.widgets.REGULAR_TEXT = theme.font_regular {
        font_size: (mod.widgets.HEPTA_V4_FONT_BODY),
    }

    mod.widgets.TEXT_SUB = theme.font_regular {
        font_size: (mod.widgets.HEPTA_V4_FONT_METADATA),
    }

    mod.widgets.USERNAME_FONT_SIZE = (mod.widgets.HEPTA_V4_FONT_LABEL)
    mod.widgets.USERNAME_TEXT_STYLE = theme.font_bold {
        font_size: (mod.widgets.USERNAME_FONT_SIZE),
    }

    mod.widgets.MESSAGE_FONT_SIZE = (mod.widgets.HEPTA_V4_FONT_MESSAGE)
    mod.widgets.REDACTED_MESSAGE_FONT_SIZE = 14
    mod.widgets.MESSAGE_TEXT_LINE_SPACING = 1.45
    mod.widgets.MESSAGE_TEXT_STYLE = theme.font_regular {
        font_size: (mod.widgets.MESSAGE_FONT_SIZE),
        line_spacing: (mod.widgets.MESSAGE_TEXT_LINE_SPACING),
    }

    mod.widgets.MESSAGE_REPLY_PREVIEW_FONT_SIZE = (mod.widgets.HEPTA_V4_FONT_LABEL)
    mod.widgets.SMALL_STATE_FONT_SIZE = (mod.widgets.HEPTA_V4_FONT_METADATA)
    mod.widgets.SMALL_STATE_TEXT_STYLE = theme.font_regular {
        font_size: (mod.widgets.SMALL_STATE_FONT_SIZE),
    }

    mod.widgets.TIMESTAMP_FONT_SIZE = (mod.widgets.HEPTA_V4_FONT_METADATA)
    mod.widgets.TIMESTAMP_TEXT_STYLE = theme.font_regular {
        font_size: (mod.widgets.TIMESTAMP_FONT_SIZE),
    }

    mod.widgets.SETTINGS_REGULAR_FONT_SIZE = (mod.widgets.HEPTA_V4_FONT_BODY)
    mod.widgets.SETTINGS_REGULAR_TEXT_STYLE = theme.font_regular {
        font_size: (mod.widgets.SETTINGS_REGULAR_FONT_SIZE),
    }

    // Interaction floor. New controls consume the v4 name; compatibility
    // dimensions used by settings/navigation are also raised here.
    mod.widgets.HEPTA_V4_MIN_POINTER_TARGET = 44.0
    mod.widgets.HEPTA_V4_MIN_TOUCH_TARGET = 48.0
    mod.widgets.HEPTA_V4_MIN_CONTROL_HEIGHT = 48.0
    mod.widgets.SETTINGS_BUTTON_HEIGHT = (mod.widgets.HEPTA_V4_MIN_CONTROL_HEIGHT)
    mod.widgets.NAVIGATION_TAB_BAR_SIZE = 56

    // Motion contract. Platform accessibility preferences may cut these to zero.
    mod.widgets.HEPTA_V4_MOTION_FAST = 0.10
    mod.widgets.HEPTA_V4_MOTION_NORMAL = 0.18
    mod.widgets.HEPTA_V4_MOTION_LAYOUT = 0.22

    // New v4 controls should inherit these templates rather than introducing
    // independent hard-coded typography and sizing.
    mod.widgets.HeptaV4BodyLabel = Label {
        draw_text +: {
            color: (mod.widgets.COLOR_HEPTA_TEXT)
            text_style: theme.font_regular {
                font_size: (mod.widgets.HEPTA_V4_FONT_BODY)
                line_spacing: 1.5
            }
        }
    }

    mod.widgets.HeptaV4MetadataLabel = Label {
        draw_text +: {
            color: (mod.widgets.COLOR_HEPTA_MUTED)
            text_style: theme.font_regular {
                font_size: (mod.widgets.HEPTA_V4_FONT_METADATA)
                line_spacing: 1.35
            }
        }
    }

    mod.widgets.HeptaV4TouchButton = Button {
        width: Fit
        height: (mod.widgets.HEPTA_V4_MIN_TOUCH_TARGET)
        padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_STRONG)
            color_hover: (mod.widgets.COLOR_HEPTA_FOCUS_SURFACE)
            color_down: (mod.widgets.COLOR_HEPTA_FOCUS_SURFACE_HOVER)
            border_color: (mod.widgets.COLOR_HEPTA_HAIRLINE)
            border_color_hover: (mod.widgets.COLOR_HEPTA_FOCUS)
            border_color_down: (mod.widgets.COLOR_HEPTA_FOCUS_HOVER)
            border_size: 1.0
            border_radius: (mod.widgets.HEPTA_RADIUS_CONTROL)
        }
        draw_text +: {
            color: (mod.widgets.COLOR_HEPTA_TEXT)
            color_hover: (mod.widgets.COLOR_HEPTA_TEXT)
            color_down: (mod.widgets.COLOR_HEPTA_TEXT)
            text_style: theme.font_regular {
                font_size: (mod.widgets.HEPTA_V4_FONT_LABEL)
            }
        }
    }
}

pub const HEPTA_UI_V4_ENABLED: bool = true;
pub const HEPTA_UI_V4_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_UI_V4_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_UI_V4_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_UI_V4_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_UI_V4_PROMOTION: bool = false;

pub const HEPTA_V4_FONT_BODY: f64 = 15.0;
pub const HEPTA_V4_FONT_MESSAGE: f64 = 15.0;
pub const HEPTA_V4_FONT_METADATA: f64 = 12.0;
pub const HEPTA_V4_MIN_TOUCH_TARGET: f64 = 48.0;
pub const HEPTA_V4_MIN_CONTROL_HEIGHT: f64 = 48.0;
pub const HEPTA_V4_CONTENT_BLUR: f64 = 0.0;
pub const HEPTA_V4_MAX_VISIBLE_BACKDROP_LAYERS: usize = 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v4_contract_is_fail_closed_and_readable() {
        assert!(HEPTA_UI_V4_ENABLED);
        assert!(!HEPTA_UI_V4_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_UI_V4_EFFECT_AUTHORITY);
        assert!(!HEPTA_UI_V4_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_UI_V4_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_UI_V4_PROMOTION);
        assert!(HEPTA_V4_FONT_BODY >= 15.0);
        assert!(HEPTA_V4_FONT_MESSAGE >= 15.0);
        assert!(HEPTA_V4_FONT_METADATA >= 12.0);
        assert!(HEPTA_V4_MIN_TOUCH_TARGET >= 48.0);
        assert!(HEPTA_V4_MIN_CONTROL_HEIGHT >= 48.0);
        assert_eq!(HEPTA_V4_CONTENT_BLUR, 0.0);
        assert!(HEPTA_V4_MAX_VISIBLE_BACKDROP_LAYERS <= 2);
    }
}
