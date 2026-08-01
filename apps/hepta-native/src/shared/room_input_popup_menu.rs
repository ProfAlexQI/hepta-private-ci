//! Room-input attachment menu selectively adapted from Robrix.
//!
//! The widget is registered and compile-ready, but intentionally not wired into Hepta's custom
//! `RoomInputBar`. That bar already owns confirmation-first attachment and location semantics.

use makepad_widgets::makepad_platform::event::finger::TouchState;
use makepad_widgets::*;

pub const ROBRIX_UPSTREAM_COMMIT: &str = "a5a664da569c577ab1a3e5a33f45dcc9364954a0";
pub const INTAKE_STATUS: &str = "registered_widget_not_wired_to_custom_composer";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.HEPTA_INTAKE_ICON_ADD_IMAGE = crate_resource("self://resources/icons/add_image.svg")
    mod.widgets.HEPTA_INTAKE_ICON_UPLOAD = crate_resource("self://resources/icons/upload.svg")
    mod.widgets.HEPTA_INTAKE_ICON_LOCATION = crate_resource("self://resources/icons/location-person.svg")

    mod.widgets.RoomInputPopupMenuButton = RobrixIconButton {
        height: 44
        width: Fill
        margin: 0
        padding: Inset{left: 12, right: 14, top: 11, bottom: 11}
        spacing: 10
        align: Align{x: 0, y: 0.5}

        draw_bg +: {
            color: (COLOR_HEPTA_GLASS_PANEL)
            color_hover: (COLOR_HEPTA_GLASS_ACTIVE_SURFACE)
            color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
            border_color: (COLOR_HEPTA_GLASS_HAIRLINE)
            border_color_hover: (COLOR_HEPTA_GLASS_FOCUS)
            border_color_down: (COLOR_HEPTA_GLASS_FOCUS)
            border_size: 1.0
            border_radius: (HEPTA_GLASS_CONTROL_RADIUS)
        }
        draw_text +: {
            color: (COLOR_HEPTA_GLASS_TEXT)
            color_hover: (COLOR_HEPTA_GLASS_TEXT)
            color_down: (COLOR_HEPTA_GLASS_TEXT)
            text_style: REGULAR_TEXT {font_size: 11}
        }
        draw_icon.color: (COLOR_HEPTA_GLASS_FOCUS)
        icon_walk: Walk{width: 18, height: 18}
    }

    mod.widgets.RoomInputPopupMenu = set_type_default() do #(RoomInputPopupMenu::register_widget(vm)) {
        ..mod.widgets.SolidView

        visible: false
        width: Fill
        height: Fill
        flow: Overlay
        align: Align{x: 0, y: 1}
        cursor: MouseCursor.Default
        show_bg: false
        draw_bg +: { color: #00000000 }

        main_content := RoundedShadowView {
            width: 264
            height: Fit
            flow: Down
            padding: 8
            spacing: 4
            align: Align{x: 0, y: 0}
            show_bg: true
            draw_bg +: {
                color: (COLOR_HEPTA_GLASS_PANEL)
                border_radius: (HEPTA_GLASS_FLOATING_RADIUS)
                border_size: 1.0
                border_color: (COLOR_HEPTA_GLASS_HAIRLINE)
                shadow_color: (COLOR_HEPTA_GLASS_SHADOW)
                shadow_radius: 12.0
                shadow_offset: vec2(0.0, 4.0)
            }

            upload_photo_video_button := mod.widgets.RoomInputPopupMenuButton {
                draw_icon.svg: (mod.widgets.HEPTA_INTAKE_ICON_ADD_IMAGE)
                text: "Upload photo or video"
            }
            upload_file_button := mod.widgets.RoomInputPopupMenuButton {
                draw_icon.svg: (mod.widgets.HEPTA_INTAKE_ICON_UPLOAD)
                text: "Upload file"
            }
            send_location_button := mod.widgets.RoomInputPopupMenuButton {
                draw_icon.svg: (mod.widgets.HEPTA_INTAKE_ICON_LOCATION)
                text: "Send current location"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RoomInputPopupMenuAction {
    Show {
        button_rect: Rect,
    },
    UploadPhotoOrVideo,
    UploadFile,
    SendCurrentLocation,
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct RoomInputPopupMenu {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for RoomInputPopupMenu {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        if matches!(
            event,
            Event::KeyUp(KeyEvent {
                key_code: KeyCode::Escape,
                ..
            })
        ) || event.back_pressed()
        {
            self.close(cx);
            return;
        }

        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for RoomInputPopupMenu {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let action = if self
            .button(cx, ids!(upload_photo_video_button))
            .clicked(actions)
        {
            RoomInputPopupMenuAction::UploadPhotoOrVideo
        } else if self.button(cx, ids!(upload_file_button)).clicked(actions) {
            RoomInputPopupMenuAction::UploadFile
        } else if self.button(cx, ids!(send_location_button)).clicked(actions) {
            RoomInputPopupMenuAction::SendCurrentLocation
        } else {
            RoomInputPopupMenuAction::None
        };

        if action != RoomInputPopupMenuAction::None {
            self.close(cx);
            cx.widget_action(self.widget_uid(), action);
        }
    }
}

impl RoomInputPopupMenu {
    pub fn is_open(&self) -> bool {
        self.visible
    }

    pub fn show(&mut self, cx: &mut Cx) {
        self.reset_button_hover(cx);
        self.visible = true;
        cx.set_key_focus(self.view.area());
        self.redraw(cx);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        self.visible = false;
        cx.revert_key_focus();
        self.redraw(cx);
    }

    fn reset_button_hover(&mut self, cx: &mut Cx) {
        self.button(cx, ids!(upload_photo_video_button))
            .reset_hover(cx);
        self.button(cx, ids!(upload_file_button)).reset_hover(cx);
        self.button(cx, ids!(send_location_button)).reset_hover(cx);
    }

    pub fn is_event_within_popup_menu(&self, cx: &mut Cx, event: &Event) -> bool {
        let main_rect = self.view(cx, ids!(main_content)).area().rect(cx);
        match event {
            Event::MouseDown(event) => main_rect.contains(event.abs),
            Event::MouseUp(event) => main_rect.contains(event.abs),
            Event::MouseMove(event) => main_rect.contains(event.abs),
            Event::Scroll(event) => main_rect.contains(event.abs),
            Event::LongPress(event) => main_rect.contains(event.abs),
            Event::TouchUpdate(event) => event
                .touches
                .iter()
                .any(|touch| main_rect.contains(touch.abs)),
            _ => false,
        }
    }

    pub fn should_dismiss_for_outside_event(&self, cx: &mut Cx, event: &Event) -> bool {
        let main_rect = self.view(cx, ids!(main_content)).area().rect(cx);
        match event {
            Event::MouseDown(event) => !main_rect.contains(event.abs),
            Event::LongPress(event) => !main_rect.contains(event.abs),
            Event::TouchUpdate(event) => event
                .touches
                .iter()
                .any(|touch| touch.state == TouchState::Start && !main_rect.contains(touch.abs)),
            _ => false,
        }
    }
}

impl RoomInputPopupMenuRef {
    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open())
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn show(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx);
        }
    }

    pub fn selected(&self, actions: &Actions) -> Option<RoomInputPopupMenuAction> {
        match actions.find_widget_action(self.widget_uid()).cast() {
            RoomInputPopupMenuAction::None => None,
            action => Some(action),
        }
    }
}
