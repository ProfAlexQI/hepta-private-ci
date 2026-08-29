//! A v4 text input used to filter rooms and spaces.
//!
//! The filter is migrated as a bounded interaction unit: the outer stable
//! surface, the text input, and the clear affordance all meet the 48 logical
//! pixel target. Desktop and mobile callers place it in a 56px row.

use makepad_widgets::*;

pub const HEPTA_ROOM_FILTER_CONTROL_HEIGHT: f64 = 48.0;
pub const HEPTA_ROOM_FILTER_CLEAR_TARGET: f64 = 48.0;
pub const HEPTA_ROOM_FILTER_TEXT_SIZE: f64 = 15.0;
pub const HEPTA_ROOM_FILTER_ICON_SIZE: f64 = 18.0;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HEPTA_ROOM_FILTER_CONTROL_HEIGHT = 48
    mod.widgets.HEPTA_ROOM_FILTER_CLEAR_TARGET = 48
    mod.widgets.HEPTA_ROOM_FILTER_TEXT_SIZE = 15
    mod.widgets.HEPTA_ROOM_FILTER_ICON_SIZE = 18

    mod.widgets.RoomFilterInputBar = set_type_default() do #(RoomFilterInputBar::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill
        height: (mod.widgets.HEPTA_V4_FILTER_CONTROL_HEIGHT)
        flow: Right
        align: Align{x: 0.0, y: 0.5}
        padding: Inset{top: 0, bottom: 0, left: 10, right: 0}
        margin: 0
        spacing: 4

        show_bg: true
        draw_bg +: {
            color: (mod.widgets.COLOR_HEPTA_CONTENT)
            border_radius: (mod.widgets.HEPTA_RADIUS_CONTROL)
            border_color: (mod.widgets.COLOR_HEPTA_HAIRLINE)
            border_size: 1.0
        }

        Icon {
            draw_icon +: {
                svg: (mod.widgets.ICON_SEARCH)
                color: (mod.widgets.COLOR_HEPTA_MUTED)
            }
            icon_walk: Walk{
                width: (mod.widgets.HEPTA_ROOM_FILTER_ICON_SIZE)
                height: (mod.widgets.HEPTA_ROOM_FILTER_ICON_SIZE)
            }
        }

        input := mod.widgets.HeptaV4RobrixTextInput {
            width: Fill
            height: Fill
            flow: Right
            padding: Inset{top: 10, bottom: 10, left: 8, right: 8}
            empty_text: "Filter rooms & spaces..."
            autocapitalize: None

            // The enclosing filter surface owns the border and stable fill.
            // Keeping the input states transparent prevents nested material.
            draw_bg +: {
                color: (mod.widgets.COLOR_TRANSPARENT)
                color_hover: (mod.widgets.COLOR_TRANSPARENT)
                color_focus: (mod.widgets.COLOR_TRANSPARENT)
                color_down: (mod.widgets.COLOR_TRANSPARENT)
                color_empty: (mod.widgets.COLOR_TRANSPARENT)
                color_disabled: (mod.widgets.COLOR_HEPTA_DISABLED_SURFACE)
                border_size: 0.0
                border_color: (mod.widgets.COLOR_TRANSPARENT)
                border_color_hover: (mod.widgets.COLOR_TRANSPARENT)
                border_color_focus: (mod.widgets.COLOR_TRANSPARENT)
                border_color_down: (mod.widgets.COLOR_TRANSPARENT)
                border_color_empty: (mod.widgets.COLOR_TRANSPARENT)
                border_color_disabled: (mod.widgets.COLOR_TRANSPARENT)
            }
            draw_text +: {
                text_style: theme.font_regular {
                    font_size: (mod.widgets.HEPTA_ROOM_FILTER_TEXT_SIZE)
                }
            }
        }

        clear_button := RobrixNeutralIconButton {
            visible: false
            width: (mod.widgets.HEPTA_ROOM_FILTER_CLEAR_TARGET)
            height: (mod.widgets.HEPTA_ROOM_FILTER_CLEAR_TARGET)
            margin: 0
            padding: 15
            spacing: 0
            align: Align{x: 0.5, y: 0.5}
            text: ""
            draw_bg +: {
                color: (mod.widgets.COLOR_TRANSPARENT)
                color_hover: (mod.widgets.COLOR_HEPTA_FOCUS_SURFACE)
                color_down: (mod.widgets.COLOR_HEPTA_FOCUS_SURFACE_HOVER)
                border_radius: (mod.widgets.HEPTA_RADIUS_CONTROL)
                border_size: 0.0
            }
            draw_icon +: {
                svg: (mod.widgets.ICON_CLOSE)
                color: (mod.widgets.COLOR_HEPTA_MUTED)
                color_hover: (mod.widgets.COLOR_HEPTA_FOCUS)
                color_down: (mod.widgets.COLOR_HEPTA_FOCUS_HOVER)
            }
            icon_walk: Walk{
                width: (mod.widgets.HEPTA_ROOM_FILTER_ICON_SIZE)
                height: (mod.widgets.HEPTA_ROOM_FILTER_ICON_SIZE)
                margin: 0
            }
        }
    }
}

/// Actions emitted by the [`RoomFilterInputBar`].
#[derive(Clone, Debug, Default)]
pub enum FilterAction {
    Changed(String),
    #[default]
    None,
}

impl ActionDefaultRef for FilterAction {
    fn default_ref() -> &'static Self {
        static DEFAULT: FilterAction = FilterAction::None;
        &DEFAULT
    }
}

/// The main home-filter action consumed by RoomsList and SpacesBar.
#[derive(Debug)]
pub enum MainFilterAction {
    Changed(String),
}

#[derive(Script, Widget)]
pub struct RoomFilterInputBar {
    #[deref]
    view: View,
}

impl ScriptHook for RoomFilterInputBar {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        // Visibility is runtime state and must survive a script reapply.
        if !apply.is_script_reapply() {
            return;
        }
        let cx = vm.cx_mut();
        let has_text = !self.text_input(cx, ids!(input)).text().is_empty();
        self.button(cx, ids!(clear_button))
            .set_visible(cx, has_text);
    }
}

impl Widget for RoomFilterInputBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl RoomFilterInputBar {
    pub fn changed(&self, actions: &Actions) -> Option<String> {
        let item = actions.find_widget_action(self.widget_uid())?;
        let FilterAction::Changed(keywords) = item.cast() else {
            return None;
        };
        Some(keywords)
    }
}

impl RoomFilterInputBarRef {
    pub fn changed(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }
}

impl WidgetMatchEvent for RoomFilterInputBar {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let input = self.text_input(cx, ids!(input));
        let clear_button = self.button(cx, ids!(clear_button));

        if let Some(keywords) = input.changed(actions) {
            let trimmed = keywords.trim();
            let keywords = if trimmed.len() < keywords.len() {
                trimmed.to_string()
            } else {
                keywords
            };
            clear_button.set_visible(cx, !keywords.is_empty());
            clear_button.reset_hover(cx);
            cx.widget_action(self.widget_uid(), FilterAction::Changed(keywords));
        }

        if clear_button.clicked(actions) {
            input.set_text(cx, "");
            clear_button.set_visible(cx, false);
            input.set_key_focus(cx);
            cx.widget_action(self.widget_uid(), FilterAction::Changed(String::new()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_filter_meets_v4_interaction_and_type_floors() {
        assert!(HEPTA_ROOM_FILTER_CONTROL_HEIGHT >= 48.0);
        assert!(HEPTA_ROOM_FILTER_CLEAR_TARGET >= 48.0);
        assert!(HEPTA_ROOM_FILTER_TEXT_SIZE >= 15.0);
        assert!((18.0..=24.0).contains(&HEPTA_ROOM_FILTER_ICON_SIZE));
    }
}
