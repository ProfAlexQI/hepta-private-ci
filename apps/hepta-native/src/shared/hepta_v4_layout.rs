//! Native adaptive-layout primitives for the v4 UI lane.
//!
//! The constants and widgets establish the mobile header, back-target, safe
//! inset, and desktop rail floors used during the next component migrations.

use makepad_widgets::*;

pub const HEPTA_V4_MOBILE_HEADER_HEIGHT: f64 = 56.0;
pub const HEPTA_V4_MOBILE_BACK_TARGET: f64 = 48.0;
pub const HEPTA_V4_DESKTOP_RAIL_MIN_WIDTH: f64 = 280.0;
pub const HEPTA_V4_COMPACT_DESKTOP_BREAKPOINT: f64 = 980.0;
pub const HEPTA_V4_MOBILE_BREAKPOINT: f64 = 700.0;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HeptaV4MobileTopBar = View {
        width: Fill
        height: 56
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

    mod.widgets.HeptaV4DesktopRail = View {
        width: 280
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
        assert!(HEPTA_V4_DESKTOP_RAIL_MIN_WIDTH >= 280.0);
        assert_eq!(HEPTA_V4_COMPACT_DESKTOP_BREAKPOINT, 980.0);
        assert_eq!(HEPTA_V4_MOBILE_BREAKPOINT, 700.0);
    }
}
