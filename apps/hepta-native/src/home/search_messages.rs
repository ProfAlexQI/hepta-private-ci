//! UI widgets for searching messages in one or more rooms.

use makepad_widgets::*;

use crate::shared::popup_list::{PopupKind, enqueue_popup_notification};

pub const SIDEBAR_MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_EVIDENCE: &str = "SearchMessagesButton only opens the current room's local loaded-timeline search preview. It does not submit a Matrix-backed message search query, server-side history request, event context fetch, pagination request, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const SIDEBAR_MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_LABEL: &str =
    "Messages search opens local loaded-timeline search only; no Matrix-backed history query.";
pub const SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_EVIDENCE: &str = "SearchMessagesButton emits SearchMessagesAction::LocalPreviewOpened, and the active RoomScreen handles that action by opening the existing telegram_message_search_strip for the currently selected room. This handoff only reveals the loaded-timeline search UI and sends no Matrix-backed search, server-side history query, event context fetch, timeline pagination/reload, room preview fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_LABEL: &str =
    "Messages opens this chat's loaded-timeline search strip.";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.SearchMessagesButton = set_type_default() do #(SearchMessagesButton::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fit,
        height: 34,
        margin: 0

        draw_bg +: {
            color: (COLOR_TELEGRAM_INPUT)
            color_hover: #243447
            color_down: #2B5278
            border_radius: 17.0
            border_color: (COLOR_TELEGRAM_BORDER)
            border_size: 1.0
        }
        draw_icon +: {
            svg: (ICON_SEARCH)
            color: (COLOR_TELEGRAM_BLUE)
        }
        icon_walk: Walk{width: 15, height: 15, margin: Inset{left: -1, right: -2} }

        text: "Messages"
        draw_text +: {
            color: (COLOR_TELEGRAM_TEXT)
            text_style: theme.font_bold { font_size: 10.5 },
        }
    }


}

#[derive(Script, ScriptHook, Widget)]
pub struct SearchMessagesButton {
    #[deref]
    button: Button,
}

impl Widget for SearchMessagesButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.button.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self.button.clicked(actions) {
                enqueue_popup_notification(
                    SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_LABEL,
                    PopupKind::Info,
                    Some(4.0),
                );
                cx.action(SearchMessagesAction::LocalPreviewOpened);
            }
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.button.draw_walk(cx, scope, walk)
    }
}

#[derive(Debug)]
pub enum SearchMessagesAction {
    LocalPreviewOpened,
}
