//! The RoomsSideBar is the widget that contains the RoomsList and other items.
//!
//! It differs in what content it includes based on the adaptive view:
//! * On a narrow mobile view, it acts as the root_view of StackNavigation
//!   * It includes a title label, a search bar, and the RoomsList.
//! * On a wide desktop view, it acts as a permanent tab that is on the left side of the dock.
//!   * It includes the title, Telegram-style room filter, and RoomsList.

use makepad_widgets::*;

use crate::home::rooms_list::RoomsListWidgetExt;
use crate::settings::app_preferences::{AppPreferencesGlobal, AppPreferencesAction, ViewModeOverride};
use crate::shared::room_filter_input_bar::{MainFilterAction, RoomFilterInputBarWidgetExt};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.TelegramDialogFilterChip = Button {
        width: Fit,
        height: 25,
        padding: Inset{left: 10, right: 10, top: 5, bottom: 5}
        margin: 0
        spacing: 0
        align: Align{x: 0.5, y: 0.5}

        draw_bg +: {
            color: (COLOR_TELEGRAM_INPUT)
            color_hover: (COLOR_TELEGRAM_DIALOG_ACTIVE)
            color_down: (COLOR_TELEGRAM_BLUE)
            border_radius: 12.5
            border_size: 1.0
            border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
            border_color_hover: (COLOR_TELEGRAM_BLUE)
            border_color_down: (COLOR_TELEGRAM_BLUE)
        }

        draw_text +: {
            color: (COLOR_TELEGRAM_MUTED)
            color_hover: (COLOR_TELEGRAM_TEXT)
            color_down: (COLOR_TELEGRAM_TEXT)
            text_style: theme.font_regular { font_size: 9 },
        }
    }


    mod.widgets.RoomsSideBar = #(RoomsSideBar::register_widget(vm)) {
        Desktop := RoundedView {
            padding: Inset{top: 10, left: 10, right: 8}
            flow: Down, spacing: 4
            width: Fill, height: Fill

            show_bg: true
            draw_bg +: {
                color: (COLOR_TELEGRAM_PANEL)
                border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: 0.0
            }

            CachedWidget {
                rooms_list_header := RoomsListHeader {}
            }
            desktop_dialog_filter := View {
                width: Fill,
                height: 40,
                flow: Right
                padding: Inset{top: 4, bottom: 3, left: 0, right: 3}
                spacing: 5
                align: Align{y: 0.5}

                CachedWidget {
                    room_filter_input_bar := RoomFilterInputBar {
                        width: 150,
                    }
                }

                search_messages_button := SearchMessagesButton { }
            }
            desktop_dialog_filter_tabs := View {
                width: Fill,
                height: 31,
                flow: Right
                padding: Inset{top: 2, bottom: 4, left: 0, right: 3}
                spacing: 6

                all_filter_button := mod.widgets.TelegramDialogFilterChip { text: "All" }
                unread_filter_button := mod.widgets.TelegramDialogFilterChip { text: "Unread" }
                direct_filter_button := mod.widgets.TelegramDialogFilterChip { text: "Direct" }
                favorite_filter_button := mod.widgets.TelegramDialogFilterChip { text: "Fav" }
            }
            CachedWidget {
                rooms_list := RoomsList {}
            }
        },

        Mobile := View {
            width: Fill, height: Fill
            flow: Down,
            show_bg: true
            draw_bg.color: (COLOR_TELEGRAM_PANEL)

            RoundedShadowView {
                width: Fill, height: Fit
                padding: Inset{top: 12, left: 10, right: 10, bottom: 8}
                flow: Down,

                show_bg: true
                draw_bg +: {
                    color: (COLOR_TELEGRAM_PANEL)
                    border_radius: 0.0
                    border_size: 1.0
                    border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
                    shadow_color: (COLOR_TELEGRAM_GLASS_SHADOW)
                    shadow_radius: 12.0
                    shadow_offset: vec2(0.0, 0.0)

                    pixel: fn() {
                        let sdf = Sdf2d.viewport(self.pos * self.rect_size3)

                        let mut fill_color = self.color
                        if self.color_2.x > -0.5 {
                            let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                            let dir = if self.gradient_fill_horizontal > 0.5 self.pos.x else self.pos.y
                            fill_color = mix(self.color self.color_2 dir + dither)
                        }

                        let mut stroke_color = self.border_color
                        if self.border_color_2.x > -0.5 {
                            let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                            let dir = if self.gradient_border_horizontal > 0.5 self.pos.x else self.pos.y
                            stroke_color = mix(self.border_color self.border_color_2 dir + dither)
                        }

                        sdf.box(
                            self.sdf_rect_pos.x
                            self.sdf_rect_pos.y
                            self.sdf_rect_size.x
                            self.sdf_rect_size.y
                            max(1.0 self.border_radius)
                        )
                        if sdf.shape > -1.0 {
                            let m = self.shadow_radius
                            let o = self.shadow_offset + self.rect_shift
                            let v = GaussShadow.rounded_box_shadow(vec2(m) + o self.rect_size2+o self.pos * (self.rect_size3+vec2(m)) self.shadow_radius*0.5 self.border_radius*2.0)
                            // Only draw shadow on the bottom half of the view
                            let pixel_y = self.pos.y * self.rect_size3.y
                            let mid_y = self.sdf_rect_pos.y + self.sdf_rect_size.y * 0.5
                            let bottom_mask = smoothstep(mid_y - m * 0.3 mid_y + m * 0.3 pixel_y)
                            sdf.clear(self.shadow_color * v * bottom_mask)
                        }

                        sdf.fill_keep(fill_color)

                        if self.border_size > 0.0 {
                            sdf.stroke(stroke_color self.border_size)
                        }
                        return sdf.result
                    }
                }

                CachedWidget {
                    rooms_list_header := RoomsListHeader {}
                }

                View {
                    width: Fill,
                    height: 45,
                    flow: Right
                    padding: Inset{top: 5, bottom: 2}
                    spacing: 5
                    align: Align{y: 0.5}

                    CachedWidget {
                        room_filter_input_bar := RoomFilterInputBar {}
                    }

                    search_messages_button := SearchMessagesButton { }
                }

                mobile_dialog_filter_tabs := View {
                    width: Fill,
                    height: 31,
                    flow: Right
                    padding: Inset{top: 2, bottom: 4, left: 0, right: 3}
                    spacing: 6

                    all_filter_button := mod.widgets.TelegramDialogFilterChip { text: "All" }
                    unread_filter_button := mod.widgets.TelegramDialogFilterChip { text: "Unread" }
                    direct_filter_button := mod.widgets.TelegramDialogFilterChip { text: "Direct" }
                    favorite_filter_button := mod.widgets.TelegramDialogFilterChip { text: "Fav" }
                }
            }

            View {
                width: Fill, height: Fill
                padding: Inset{left: 0, right: 0}
                show_bg: true
                draw_bg.color: (COLOR_TELEGRAM_PANEL)

                CachedWidget {
                    rooms_list := RoomsList {}
                }
            }
        }
    }
}

/// A simple wrapper around `AdaptiveView` that contains several global singleton widgets.
///
/// * In the mobile view, it serves as the root view of the StackNavigation,
///   showing the title label, the search bar, and the RoomsList.
/// * In the desktop view, it is a permanent tab in the dock,
///   showing the title label, the room filter, and the RoomsList.
#[derive(Script, Widget)]
pub struct RoomsSideBar {
    #[deref]
    view: AdaptiveView,

    /// The most recently applied view-mode override.
    #[rust]
    applied_view_mode: ViewModeOverride,
}

impl ScriptHook for RoomsSideBar {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            // Here we set the global singleton for the RoomsList widget,
            // which is used to access the list of rooms from anywhere in the app.
            cx.set_global(self.view.rooms_list(cx, ids!(rooms_list)));

            // The RoomsSideBar is re-instantiated every time the HomeScreen's
            // AdaptiveView switches between Desktop and Mobile view modes
            // (cuz it's not wrapped in a CachedWidget).
            // Thus we just re-read the current value here and apply it.
            let mode = cx.global::<AppPreferencesGlobal>().0.view_mode;
            self.apply_view_mode(mode);
        });
    }
}

impl RoomsSideBar {
    fn apply_view_mode(&mut self, mode: ViewModeOverride) {
        self.view.set_variant_selector(mode.variant_selector());
        self.applied_view_mode = mode;
    }

    fn apply_dialog_filter(&mut self, cx: &mut Cx, keywords: &str) {
        let input = self.view.text_input(cx, ids!(room_filter_input_bar.input));
        let clear_button = self
            .view
            .button(cx, ids!(room_filter_input_bar.clear_button));
        input.set_text(cx, keywords);
        clear_button.set_visible(cx, !keywords.is_empty());
        cx.action(MainFilterAction::Changed(keywords.to_string()));
        self.view.redraw(cx);
    }
}

impl Widget for RoomsSideBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // If the main room filter input bar changed keywords, re-emit that action
        // as a MainFilterAction so that other widgets can handle it.
        if let Event::Actions(actions) = event {
            if let Some(keywords) = self
                .view
                .room_filter_input_bar(cx, ids!(room_filter_input_bar))
                .changed(actions)
            {
                cx.action(MainFilterAction::Changed(keywords));
            }

            if self
                .view
                .button(cx, ids!(all_filter_button))
                .clicked(actions)
            {
                self.apply_dialog_filter(cx, "");
            } else if self
                .view
                .button(cx, ids!(unread_filter_button))
                .clicked(actions)
            {
                self.apply_dialog_filter(cx, "is:unread");
            } else if self
                .view
                .button(cx, ids!(direct_filter_button))
                .clicked(actions)
            {
                self.apply_dialog_filter(cx, "is:direct");
            } else if self
                .view
                .button(cx, ids!(favorite_filter_button))
                .clicked(actions)
            {
                self.apply_dialog_filter(cx, "is:favorite");
            }

            for action in actions {
                if let Some(AppPreferencesAction::ViewModeChanged(new_mode)) = action.downcast_ref()
                {
                    if *new_mode != self.applied_view_mode {
                        self.apply_view_mode(*new_mode);
                        self.view.redraw(cx);
                    }
                }
            }
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
