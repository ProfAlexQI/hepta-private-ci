//! Native adaptive-layout primitives for the v4 UI lane.
//!
//! The constants and widgets establish the mobile header, back-target, filter
//! row, safe inset, and desktop rail floors used during component migrations.

use makepad_widgets::*;

pub const HEPTA_V4_MOBILE_HEADER_HEIGHT: f64 = 56.0;
pub const HEPTA_V4_MOBILE_BACK_TARGET: f64 = 48.0;
pub const HEPTA_V4_FILTER_CONTROL_HEIGHT: f64 = 48.0;
pub const HEPTA_V4_FILTER_ROW_HEIGHT: f64 = 56.0;
pub const HEPTA_V4_DESKTOP_RAIL_MIN_WIDTH: f64 = 280.0;
pub const HEPTA_V4_COMPACT_DESKTOP_BREAKPOINT: f64 = 980.0;
pub const HEPTA_V4_MOBILE_BREAKPOINT: f64 = 700.0;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HEPTA_V4_MOBILE_HEADER_HEIGHT = 56
    mod.widgets.HEPTA_V4_MOBILE_BACK_TARGET = 48
    mod.widgets.HEPTA_V4_FILTER_CONTROL_HEIGHT = 48
    mod.widgets.HEPTA_V4_FILTER_ROW_HEIGHT = 56
    mod.widgets.HEPTA_V4_DESKTOP_RAIL_MIN_WIDTH = 280
    mod.widgets.HEPTA_V4_COMPACT_DESKTOP_BREAKPOINT = 980
    mod.widgets.HEPTA_V4_MOBILE_BREAKPOINT = 700

    mod.widgets.HeptaV4MobileTopBar = View {
        width: Fill
        height: (mod.widgets.HEPTA_V4_MOBILE_HEADER_HEIGHT)
        flow: Right
        align: Align{y: 0.5}
        spacing: 8
        padding: Inset{
            left: (mod.widgets.SAFE_INSET_PAD_LEFT)
            right: (mod.widgets.SAFE_INSET_PAD_RIGHT)
            top: 4
            bottom: 4
        }
        show_bg: true
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS_STRONG)
            border_size: 0.0
            border_radius: 0.0
        }
    }

    mod.widgets.HeptaV4BackButton = Button {
        width: Fit{min: FitBound.Abs(48)}
        height: Fit{min: FitBound.Abs(48)}
        padding: 12
        text: ""
        draw_bg +: {
            color: (mod.widgets.COLOR_TRANSPARENT)
            color_hover: (mod.widgets.COLOR_HEPTA_FOCUS_SURFACE)
            color_down: (mod.widgets.COLOR_HEPTA_FOCUS_SURFACE_HOVER)
            border_radius: (mod.widgets.HEPTA_RADIUS_CONTROL)
            border_size: 0.0
        }
        draw_icon +: {
            color: (mod.widgets.COLOR_HEPTA_TEXT)
            color_hover: (mod.widgets.COLOR_HEPTA_FOCUS)
            color_down: (mod.widgets.COLOR_HEPTA_FOCUS_HOVER)
        }
        icon_walk: Walk{width: 20, height: 20}
    }

    // A bounded row around a 48px control. The 4px vertical insets preserve
    // breathing room without shrinking the target or relying on overflow.
    mod.widgets.HeptaV4ControlRow = View {
        width: Fill
        height: (mod.widgets.HEPTA_V4_FILTER_ROW_HEIGHT)
        flow: Right
        align: Align{y: 0.5}
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}
        spacing: 0
    }

    mod.widgets.HeptaV4DesktopRail = View {
        width: (mod.widgets.HEPTA_V4_DESKTOP_RAIL_MIN_WIDTH)
        height: Fill
        flow: Down
        show_bg: true
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_GLASS)
            border_size: 0.0
            border_radius: 0.0
        }
    }

    mod.widgets.HeptaV4StableContent = View {
        width: Fill
        height: Fill
        show_bg: true
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_CONTENT)
            border_size: 1.0
            border_color: (mod.widgets.COLOR_HEPTA_HAIRLINE)
            border_radius: (mod.widgets.HEPTA_RADIUS_PANEL)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adaptive_layout_metrics_meet_v4_contract() {
        assert!(HEPTA_V4_MOBILE_HEADER_HEIGHT >= 56.0);
        assert!(HEPTA_V4_MOBILE_BACK_TARGET >= 48.0);
        assert!(HEPTA_V4_FILTER_CONTROL_HEIGHT >= 48.0);
        assert!(HEPTA_V4_FILTER_ROW_HEIGHT >= HEPTA_V4_FILTER_CONTROL_HEIGHT + 8.0);
        assert!(HEPTA_V4_DESKTOP_RAIL_MIN_WIDTH >= 280.0);
        assert_eq!(HEPTA_V4_COMPACT_DESKTOP_BREAKPOINT, 980.0);
        assert_eq!(HEPTA_V4_MOBILE_BREAKPOINT, 700.0);
    }
}
