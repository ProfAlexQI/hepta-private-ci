//! Native UI v4 control primitives.
//!
//! These widgets provide a migration target for legacy controls that are below
//! the shared 48 logical-pixel interaction contract. They do not enable any
//! network, mutation, effect, or production capability.

use makepad_widgets::*;

pub const HEPTA_V4_CONTROL_MIN_HEIGHT: f64 = 48.0;
pub const HEPTA_V4_ICON_HIT_TARGET: f64 = 48.0;
pub const HEPTA_V4_CONTROL_RADIUS: f64 = 10.0;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaV4TextInput = TextInput {
        width: Fill
        height: Fit{min: FitBound.Abs(48)}
        flow: Flow.Right{wrap: true}
        align: Align{y: 0.5}
        padding: Inset{left: 12, right: 12, top: 10, bottom: 10}
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_hover: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_focus: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_down: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_empty: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_disabled: (mod.widgets.COLOR_HEPTA_DISABLED_SURFACE)
            border_radius: (mod.widgets.HEPTA_RADIUS_CONTROL)
            border_size: 1.0
            border_color: (mod.widgets.COLOR_HEPTA_HAIRLINE)
            border_color_hover: (mod.widgets.COLOR_HEPTA_FOCUS)
            border_color_focus: (mod.widgets.COLOR_HEPTA_FOCUS)
            border_color_down: (mod.widgets.COLOR_HEPTA_FOCUS_HOVER)
            border_color_empty: (mod.widgets.COLOR_HEPTA_HAIRLINE)
            border_color_disabled: (mod.widgets.COLOR_HEPTA_DISABLED)
        }
        draw_text +: {
            color: (mod.widgets.COLOR_HEPTA_TEXT)
            color_hover: (mod.widgets.COLOR_HEPTA_TEXT)
            color_focus: (mod.widgets.COLOR_HEPTA_TEXT)
            color_down: (mod.widgets.COLOR_HEPTA_TEXT)
            color_empty: (mod.widgets.COLOR_HEPTA_DIM)
            color_empty_hover: (mod.widgets.COLOR_HEPTA_DIM)
            color_empty_focus: (mod.widgets.COLOR_HEPTA_DIM)
            color_disabled: (mod.widgets.COLOR_HEPTA_DISABLED)
            text_style: theme.font_regular { font_size: 15 }
        }
        draw_cursor +: { color: (mod.widgets.COLOR_HEPTA_TEXT) }
        draw_selection +: { color: (mod.widgets.COLOR_HEPTA_SELECTION) }
    }

    mod.widgets.HeptaV4IconButton = Button {
        width: Fit{min: FitBound.Abs(48)}
        height: Fit{min: FitBound.Abs(48)}
        align: Align{x: 0.5, y: 0.5}
        padding: 12
        text: ""
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_STRONG)
            color_hover: (mod.widgets.COLOR_HEPTA_FOCUS_SURFACE)
            color_down: (mod.widgets.COLOR_HEPTA_FOCUS_SURFACE_HOVER)
            border_radius: (mod.widgets.HEPTA_RADIUS_CONTROL)
            border_size: 1.0
            border_color: (mod.widgets.COLOR_HEPTA_HAIRLINE)
            border_color_hover: (mod.widgets.COLOR_HEPTA_FOCUS)
            border_color_down: (mod.widgets.COLOR_HEPTA_FOCUS_HOVER)
        }
        draw_icon +: {
            color: (mod.widgets.COLOR_HEPTA_TEXT)
            color_hover: (mod.widgets.COLOR_HEPTA_FOCUS)
            color_down: (mod.widgets.COLOR_HEPTA_FOCUS_HOVER)
        }
        icon_walk: Walk{width: 22, height: 22}
    }

    mod.widgets.HeptaV4PrimaryButton = Button {
        height: Fit{min: FitBound.Abs(48)}
        padding: Inset{left: 16, right: 16, top: 11, bottom: 11}
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_FOCUS)
            color_hover: (mod.widgets.COLOR_HEPTA_FOCUS_HOVER)
            color_down: (mod.widgets.COLOR_HEPTA_FOCUS_HOVER)
            border_radius: (mod.widgets.HEPTA_RADIUS_CONTROL)
            border_size: 0.0
        }
        draw_text +: {
            color: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_hover: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_down: (mod.widgets.COLOR_HEPTA_CONTENT)
            text_style: theme.font_bold { font_size: 15 }
        }
        draw_icon +: {
            color: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_hover: (mod.widgets.COLOR_HEPTA_CONTENT)
            color_down: (mod.widgets.COLOR_HEPTA_CONTENT)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v4_controls_meet_the_shared_interaction_floor() {
        assert!(HEPTA_V4_CONTROL_MIN_HEIGHT >= 48.0);
        assert!(HEPTA_V4_ICON_HIT_TARGET >= 48.0);
        assert!(HEPTA_V4_CONTROL_RADIUS >= 10.0);
    }
}
