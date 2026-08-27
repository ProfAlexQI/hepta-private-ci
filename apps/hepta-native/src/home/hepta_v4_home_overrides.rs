//! Post-registration v4 overrides for the mobile HomeScreen stack.
//!
//! `home_screen` owns the upstream-compatible StackNavigation prototype. This
//! module loads immediately afterwards and upgrades that prototype to the v4
//! 56px header and 48px back-target contract without enabling any capability.

use makepad_widgets::*;

pub const HEPTA_V4_STACK_HEADER_HEIGHT: f64 = 56.0;
pub const HEPTA_V4_STACK_BACK_TARGET: f64 = 48.0;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.STACK_VIEW_HEADER_HEIGHT = (mod.widgets.HEPTA_V4_MOBILE_HEADER_HEIGHT)

    mod.widgets.HeptaV4RobrixStackNavigationView = mod.widgets.RobrixStackNavigationView {
        header +: {
            height: (mod.widgets.HEPTA_V4_MOBILE_HEADER_HEIGHT)
            content +: {
                height: (mod.widgets.HEPTA_V4_MOBILE_HEADER_HEIGHT)
                button_container +: {
                    left_button +: {
                        width: Fit{min: FitBound.Abs(48)}
                        height: Fit{min: FitBound.Abs(48)}
                        padding: 12
                        margin: Inset{left: 4, right: 0, top: 0, bottom: 0}
                        icon_walk: Walk{width: 20, height: 20}
                    }
                }
            }
        }
        body +: {
            margin: Inset{top: (mod.widgets.HEPTA_V4_MOBILE_HEADER_HEIGHT)}
        }
    }

    // Keep the compatibility name consumed by the existing stack templates.
    mod.widgets.RobrixStackNavigationView = mod.widgets.HeptaV4RobrixStackNavigationView {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_stack_metrics_meet_v4_contract() {
        assert!(HEPTA_V4_STACK_HEADER_HEIGHT >= 56.0);
        assert!(HEPTA_V4_STACK_BACK_TARGET >= 48.0);
    }
}
