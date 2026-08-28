//! Post-registration v4 overrides for the Native HomeScreen.
//!
//! This module upgrades the mobile stack and migrates the room filter as one
//! bounded layout unit. It replaces only named prototypes/pages after their
//! canonical modules load; the full HomeScreen and RoomsSideBar Rust widgets
//! remain authoritative.

use makepad_widgets::*;

pub const HEPTA_V4_STACK_HEADER_HEIGHT: f64 = 56.0;
pub const HEPTA_V4_STACK_BACK_TARGET: f64 = 48.0;
pub const HEPTA_V4_FILTER_CONTROL_HEIGHT: f64 = 48.0;
pub const HEPTA_V4_FILTER_ROW_HEIGHT: f64 = 56.0;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // One reusable row owns the vertical contract for desktop and mobile.
    mod.widgets.HeptaV4RoomFilterRow = mod.widgets.HeptaV4ControlRow {
        CachedWidget {
            room_filter_input_bar := mod.widgets.RoomFilterInputBar {}
        }
    }

    // Replace the mobile RoomsSideBar variant as a bounded unit. The old custom
    // shadow shader and 45px row are not carried forward.
    mod.widgets.HeptaV4RoomsSideBar = mod.widgets.RoomsSideBar {
        Mobile := View {
            width: Fill
            height: Fill
            flow: Down

            filter_panel := RoundedShadowView {
                width: Fill
                height: Fit
                margin: Inset{top: 8, left: 8, right: 8, bottom: 4}
                padding: Inset{top: 12, left: 10, right: 10, bottom: 8}
                flow: Down
                spacing: 4
                show_bg: true
                draw_bg +: {
                    color: (mod.widgets.COLOR_HEPTA_GLASS_STRONG)
                    border_radius: (mod.widgets.HEPTA_RADIUS_PANEL)
                    border_size: 1.0
                    border_color: (mod.widgets.COLOR_HEPTA_HAIRLINE)
                    shadow_color: (mod.widgets.COLOR_HEPTA_SHADOW)
                    shadow_radius: 8.0
                    shadow_offset: vec2(0.0, 2.0)
                }

                CachedWidget {
                    rooms_list_header := mod.widgets.RoomsListHeader {}
                }

                mod.widgets.HeptaV4RoomFilterRow {
                    padding: Inset{left: 0, right: 0, top: 4, bottom: 4}
                }
            }

            rooms_list_container := View {
                width: Fill
                height: Fill
                padding: Inset{left: 10, right: 10}

                CachedWidget {
                    rooms_list := mod.widgets.RoomsList {}
                }
            }
        }
    }
    mod.widgets.RoomsSideBar = mod.widgets.HeptaV4RoomsSideBar {}

    // Replace only the desktop home page. The PageFlip, settings/add-room pages,
    // navigation state, and MainDesktopUI implementation remain unchanged.
    mod.widgets.HeptaV4HomeScreen = mod.widgets.HomeScreen {
        main_adaptive_view +: {
            Desktop +: {
                home_screen_page_flip +: {
                    home_page := View {
                        width: Fill
                        height: Fill
                        flow: Down

                        mod.widgets.HeptaV4RoomFilterRow {
                            margin: 0
                        }

                        mod.widgets.MainDesktopUI {}
                    }
                }
            }
        }
    }
    mod.widgets.HomeScreen = mod.widgets.HeptaV4HomeScreen {}

    // Upgrade the mobile stack prototype after HomeScreen registration.
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

    mod.widgets.RobrixStackNavigationView = mod.widgets.HeptaV4RobrixStackNavigationView {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_stack_and_filter_rows_meet_v4_contract() {
        assert!(HEPTA_V4_STACK_HEADER_HEIGHT >= 56.0);
        assert!(HEPTA_V4_STACK_BACK_TARGET >= 48.0);
        assert!(HEPTA_V4_FILTER_CONTROL_HEIGHT >= 48.0);
        assert!(HEPTA_V4_FILTER_ROW_HEIGHT >= HEPTA_V4_FILTER_CONTROL_HEIGHT + 8.0);
    }
}
