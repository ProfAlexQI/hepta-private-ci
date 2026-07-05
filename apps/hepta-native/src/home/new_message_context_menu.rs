//! A context menu that appears when the user right-clicks
//! or long-presses on a message/event in a room timeline.

use std::cell::RefCell;

use bitflags::bitflags;
use makepad_widgets::*;
use matrix_sdk::{ruma::OwnedEventId, send_queue::SendHandle};
use matrix_sdk_ui::timeline::{EventTimelineItem, MsgLikeContent, TimelineEventItemId};

use crate::{
    app::{ConfirmDeleteAction, PositiveConfirmationModalAction},
    shared::{
        confirmation_modal::ConfirmationModalContent,
        popup_list::{PopupKind, enqueue_popup_notification},
    },
    sliding_sync::UserPowerLevels,
};

use super::room_screen::MessageAction;

const BUTTON_HEIGHT: f64 = 35.0; // KEEP IN SYNC WITH BUTTON_HEIGHT BELOW
const MENU_WIDTH: f64 = 215.0; // KEEP IN SYNC WITH MENU_WIDTH BELOW
pub const MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_EVIDENCE: &str = "NewMessageContextMenu keeps Report opening and Cancel as local-only moderation preview evidence while Spam, Abuse, and Custom reason require confirmation before MatrixRequest::ReportContent. Custom reason trims local input and empty custom reason stays unsent with a local warning. The confirmed report path sends only Room::report_content for the selected event and reason; it sends no moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_LABEL: &str =
    "Report preview opens locally; Spam/Abuse/Custom confirm before Matrix report_content.";
pub const MESSAGE_REPORT_LOADED_TARGET_METADATA_EVIDENCE: &str = "NewMessageContextMenu report preview shows loaded target metadata derived only from the selected loaded timeline row: row index, loaded event-id availability, loaded body preview, character count, byte count, related-event availability, thread-root availability, local echo send-handle availability, and highlight state. Opening the preview, updating custom reason, canceling, and viewing metadata send no Matrix report_content, moderation policy lookup, relations fetch, event-context fetch, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_REPORT_LOADED_TARGET_METADATA_LABEL: &str =
    "Loaded report target metadata only; no moderation lookup or report before confirmation.";
pub const MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_EVIDENCE: &str = "NewMessageContextMenu report preview shows custom reason draft metadata derived only from the local text input and the selected loaded target row: raw character count, raw byte count, whitespace-compacted character count, whitespace-compacted byte count, 240-character cap state, empty-versus-ready state, target row index, and target event-id availability. Updating the draft, pressing empty Send Custom, canceling, and viewing this draft metadata send no Matrix report_content before confirmation, moderation policy lookup, relations fetch, event-context fetch, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_LABEL: &str =
    "Custom reason draft metadata only; no ReportContent before confirmation.";
pub const MESSAGE_REPORT_CANCEL_LOCAL_EVIDENCE: &str = "NewMessageContextMenu Report Cancel and Escape only hide the local report preview, restore the Report button, reset focus/menu state, and show local popup evidence. Cancel does not submit MatrixRequest::ReportContent, does not reuse a draft reason, does not retry or cancel a moderation queue, does not fetch moderation policy, relations, or event context, and emits no redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_REPORT_CANCEL_LOCAL_LABEL: &str = "Report Cancel is local preview cleanup only; no ReportContent, retry, queue cancel, or moderation lookup.";
pub const MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE: &str = "NewMessageContextMenu report preview exposes moderation workflow boundary metadata from only the loaded target row and local custom-reason readiness. Moderation queue controls, server policy lookup, redact/delete, ban, kick, ignore/block, evidence queue, reviewer assignment, appeal flow, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth, and live mutation remain local blocked controls; the only real send path remains confirmed MatrixRequest::ReportContent for the selected event and reason.";
pub const MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_LABEL: &str = "Moderation workflow controls stay local blocked evidence; only confirmed ReportContent is wired.";
pub const MESSAGE_REPORT_SEND_COMPACT_LABEL: &str =
    "Report preview opened; no Matrix report before confirmation.";
pub const MESSAGE_REPORT_STAGING_COMPACT_LABEL: &str =
    "Report sends only after confirmation; empty custom reason stays local.";
pub const MESSAGE_LOCAL_SEND_CANCEL_COMPACT_LABEL: &str =
    "Cancel uses this local echo SendHandle only.";
const MESSAGE_CONTEXT_CONFIRMATION_COMPACT_LABEL: &str =
    "Confirmation required before the Matrix message action.";
const MESSAGE_REPORT_CONFIRMATION_COMPACT_LABEL: &str =
    "Confirmation required before Matrix report_content.";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.NEW_MESSAGE_CONTEXT_MENU_BUTTON_HEIGHT = 35  // KEEP IN SYNC WITH BUTTON_HEIGHT ABOVE
    mod.widgets.NEW_MESSAGE_CONTEXT_MENU_WIDTH = 215    // KEEP IN SYNC WITH MENU_WIDTH ABOVE

    mod.widgets.NewMessageContextMenuButton = RobrixIconButton {
        height: (mod.widgets.NEW_MESSAGE_CONTEXT_MENU_BUTTON_HEIGHT)
        width: Fill,
        margin: 0,
        icon_walk: Walk{width: 16, height: 16, margin: Inset{right: 3}}
        draw_bg +: {
            color: (COLOR_TELEGRAM_PANEL)
            color_hover: (COLOR_TELEGRAM_INPUT)
            color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
        }
        draw_icon.color: (COLOR_TELEGRAM_MUTED)
        draw_text +: {
            color: (COLOR_TELEGRAM_TEXT)
            color_hover: (COLOR_TELEGRAM_TEXT)
            color_down: (COLOR_TELEGRAM_TEXT)
        }
    }

    mod.widgets.NewMessageContextMenu = set_type_default() do #(NewMessageContextMenu::register_widget(vm)) {
        ..mod.widgets.SolidView

        visible: false,
        flow: Overlay,
        width: Fill,
        height: Fill,
        cursor: MouseCursor.Default,
        // Align to top-left such that our coordinate adjustment
        // when showing this menu pane will work correctly.
        align: Align{x: 0, y: 0}

        // Show a slightly darkened translucent background to make the menu stand out.
        show_bg: true
        draw_bg +: {
            color: #0000004D
        }

        main_content := RoundedView {
            flow: Down
            width: (mod.widgets.NEW_MESSAGE_CONTEXT_MENU_WIDTH),
            height: Fit,
            padding: 10
            spacing: 0,
            align: Align{x: 0, y: 0}

            show_bg: true
            draw_bg +: {
                color: (COLOR_TELEGRAM_PANEL)
                border_radius: 7.0
                border_size: 0.5
                border_color: (COLOR_TELEGRAM_BORDER)
            }

            // Shows either the "Add Reaction" button or a reaction text input.
            react_view := View {
                flow: Overlay
                height: (mod.widgets.NEW_MESSAGE_CONTEXT_MENU_BUTTON_HEIGHT)
                align: Align{y: 0.5}

                react_button := mod.widgets.NewMessageContextMenuButton {
                    draw_icon +: { svg: (ICON_ADD_REACTION) }
                    text: "Add Reaction"
                }

                reaction_input_view := View {
                    width: Fill,
                    height: (mod.widgets.NEW_MESSAGE_CONTEXT_MENU_BUTTON_HEIGHT)
                    align: Align{y: 0.5}
                    flow: Right,
                    visible: false, // will be shown once the react_button is clicked

                    reaction_text_input := RobrixTextInput {
                        width: Fill,
                        height: Fit,
                        align: Align{x: 0, y: 0.5}
                        padding: 7
                        // TODO: we want the TextInput flow to show all text
                        // within the single-line box by scrolling horizontally
                        // when the text is too long, upon a user typing/pasting
                        // or navigating with the mouse or arrow keys.
                        // However, makepad doesn't yet support this feature,
                        // so we just make the TextInput non-wrap.
                        flow: Flow.Right{wrap: false}, // do not wrap
                        draw_bg.border_size: 0.0
                        draw_bg.color: (COLOR_TELEGRAM_INPUT)
                        draw_bg.color_empty: (COLOR_TELEGRAM_INPUT)
                        draw_bg.color_focus: (COLOR_TELEGRAM_INPUT)
                        draw_text.color: (COLOR_TELEGRAM_TEXT)
                        draw_text.color_empty: (COLOR_TELEGRAM_DIM)
                        empty_text: "Enter reaction..."
                    }
                    reaction_send_button := RobrixPositiveIconButton {
                        height: (mod.widgets.NEW_MESSAGE_CONTEXT_MENU_BUTTON_HEIGHT)
                        align: Align{x: 0.5, y: 0.5}
                        padding: Inset{left: 10, right: 10, top: 8, bottom: 8}
                        spacing: 0,
                        draw_icon.svg: (ICON_SEND)
                        icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                    }
                }
            }

            reply_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: { svg: (ICON_REPLY) }
                icon_walk +: { margin: Inset{top: 1, right: 3}}
                text: "Reply"
            }

            divider_after_react_reply := LineH {
                margin: Inset{top: 3, bottom: 3}
                width: Fill,
            }

            edit_message_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: { svg: (ICON_EDIT) }
                icon_walk +: { margin: Inset{top: -3, right: 3} }
                text: "Edit Message"
            }

            // TODO: check if the current user is allowed to pin/unpin messages:
            //       <https://matrix-org.github.io/matrix-rust-sdk/matrix_sdk_base/struct.RoomMember.html#method.can_pin_or_unpin_event>
            pin_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: { svg: (ICON_PIN) }
                text: "" // set dynamically to "Pin Message" or "Unpin Message"
            }

            copy_text_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: { svg: (ICON_COPY) }
                text: "Copy Text"
            }

            copy_html_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: { svg: (ICON_HTML_FILE) }
                icon_walk +: { margin: Inset{left: 1.5, right: 1.5} }
                text: "Copy Text as HTML"
            }

            copy_link_to_message_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: { svg: (ICON_LINK) }
                text: "Copy Link to Message"
            }

            view_source_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: { svg: (ICON_VIEW_SOURCE) }
                text: "View Source"
            }

            jump_to_related_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: { svg: (ICON_JUMP) }
                text: "Jump to Related Event"
            }

            divider_before_report_delete := LineH {
                margin: Inset{top: 3, bottom: 3}
                width: Fill,
            }

            report_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: {
                    svg: (ICON_TRASH)
                    color: (COLOR_FG_DANGER_RED),
                }
                icon_walk +: { margin: Inset{left: -2, right: 3} }
                draw_bg +: {
                    border_color: (COLOR_FG_DANGER_RED),
                    color: (COLOR_TELEGRAM_PANEL)
                    color_hover: #3A1F27
                    color_down: #4A2630
                }
                text: "Report"
                draw_text +: {
                    color: (COLOR_FG_DANGER_RED),
                }
            }

            cancel_local_send_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: {
                    svg: (ICON_CLOSE)
                    color: (COLOR_FG_DANGER_RED),
                }
                draw_bg +: {
                    border_color: (COLOR_FG_DANGER_RED),
                    color: (COLOR_TELEGRAM_PANEL)
                    color_hover: #3A1F27
                    color_down: #4A2630
                }
                draw_text.color: (COLOR_FG_DANGER_RED),
                text: "Cancel Send"
            }

            report_preview_view := RoundedView {
                visible: false
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5.0,
                margin: Inset{top: 3.0, bottom: 3.0}
                padding: Inset{top: 7.0, bottom: 7.0, left: 9.0, right: 9.0}
                show_bg: true
                draw_bg +: {
                    color: #2A1720
                    border_color: #FF5C7A66
                    border_size: 1.0
                    border_radius: 7.0
                }

                report_preview_title := Label {
                    width: Fill,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_FG_DANGER_RED)
                    }
                    text: "Local report preview"
                }

                report_preview_summary := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true}
                    draw_text +: {
                        color: (COLOR_TELEGRAM_TEXT)
                    }
                    text: "No Matrix report request was sent."
                }

                report_option_evidence := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true}
                    draw_text +: {
                        color: (COLOR_TELEGRAM_DIM)
                        text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 }
                    }
                    text: "Report staging stays local; no moderation request."
                }

                report_moderation_boundary := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true}
                    draw_text +: {
                        color: (COLOR_TELEGRAM_DIM)
                        text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 }
                    }
                    text: "Moderation workflow controls stay local blocked evidence."
                }

                report_spam_button := mod.widgets.NewMessageContextMenuButton {
                    text: "Spam"
                }
                report_abuse_button := mod.widgets.NewMessageContextMenuButton {
                    text: "Abuse"
                }
                report_reason_text_input := RobrixTextInput {
                    width: Fill,
                    height: Fit,
                    align: Align{x: 0, y: 0.5}
                    padding: 7
                    flow: Flow.Right{wrap: false}
                    draw_bg.border_size: 0.0
                    draw_bg.color: (COLOR_TELEGRAM_INPUT)
                    draw_bg.color_empty: (COLOR_TELEGRAM_INPUT)
                    draw_bg.color_focus: (COLOR_TELEGRAM_INPUT)
                    draw_text.color: (COLOR_TELEGRAM_TEXT)
                    draw_text.color_empty: (COLOR_TELEGRAM_DIM)
                    empty_text: "Custom reason..."
                }
                report_reason_draft_metadata := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true}
                    draw_text +: {
                        color: (COLOR_TELEGRAM_DIM)
                        text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 }
                    }
                    text: "Custom reason draft is empty; no Matrix report request was sent."
                }
                report_custom_button := mod.widgets.NewMessageContextMenuButton {
                    text: "Send Custom"
                }
                report_cancel_button := mod.widgets.NewMessageContextMenuButton {
                    text: "Cancel"
                }
            }

            // Note: we don't yet support deleting others' messages via admin/moderator power levels.
            //       For now we only consider whether its the user's own message.
            //       The caller needs to use `can_redact_own()` or `can_redact_other()`:
            //       https://matrix-org.github.io/matrix-rust-sdk/matrix_sdk_base/struct.RoomMember.html#method.can_redact_own

            delete_button := mod.widgets.NewMessageContextMenuButton {
                draw_icon +: {
                    svg: (ICON_TRASH)
                    color: (COLOR_FG_DANGER_RED),
                }
                draw_bg +: {
                    border_color: (COLOR_FG_DANGER_RED),
                    color: (COLOR_TELEGRAM_PANEL)
                    color_hover: #3A1F27
                    color_down: #4A2630
                }
                draw_text.color: (COLOR_FG_DANGER_RED),
                text: "Delete"
            }
        }
    }
}

bitflags! {
    /// Possible actions that the user can perform on a message.
    ///
    /// This is used to determine which buttons to show in the message context menu.
    #[derive(Copy, Clone, Debug)]
    pub struct MessageAbilities: u8 {
        /// Whether the user can react to this message.
        const CanReact = 1 << 0;
        /// Whether the user can reply to this message.
        const CanReplyTo = 1 << 1;
        /// Whether the user can edit this message.
        const CanEdit = 1 << 2;
        /// Whether the user can pin this message.
        /// This should only be set for non-pinned messages.
        const CanPin = 1 << 3;
        /// Whether the user can unpin this message.
        /// This should only be set for currently-pinned messages.
        const CanUnpin = 1 << 4;
        /// Whether the user can delete/redact this message.
        const CanDelete = 1 << 5;
        /// Whether this message contains HTML content that the user can copy.
        const HasHtml = 1 << 6;
        /// Whether this local echo still exposes an SDK send queue handle.
        const CanCancelLocalSend = 1 << 7;
    }
}
impl MessageAbilities {
    pub fn from_user_power_and_event(
        user_power_levels: &UserPowerLevels,
        event_tl_item: &EventTimelineItem,
        _message: &MsgLikeContent,
        pinned_events: &[OwnedEventId],
        has_html: bool,
    ) -> Self {
        let mut abilities = Self::empty();
        abilities.set(Self::CanEdit, event_tl_item.is_editable());
        // Currently we only support deleting one's own messages.
        if event_tl_item.is_own() {
            abilities.set(Self::CanDelete, user_power_levels.can_redact_own());
        }
        abilities.set(Self::CanReplyTo, event_tl_item.can_be_replied_to());
        if let Some(event_id) = event_tl_item.event_id()
            && user_power_levels.can_pin()
        {
            if pinned_events.iter().any(|ev| ev == event_id) {
                abilities.set(Self::CanUnpin, true);
            } else {
                abilities.set(Self::CanPin, true);
            }
        }
        abilities.set(Self::CanReact, user_power_levels.can_send_reaction());
        abilities.set(Self::HasHtml, has_html);
        abilities.set(
            Self::CanCancelLocalSend,
            event_tl_item.local_echo_send_handle().is_some(),
        );
        abilities
    }
}

#[derive(Clone, Debug)]
pub struct MessageReportTargetMetadata {
    pub item_id: usize,
    pub body_preview: String,
    pub body_char_count: usize,
    pub body_byte_count: usize,
    pub event_id_loaded: bool,
    pub related_event_loaded: bool,
    pub thread_root_loaded: bool,
    pub local_echo_send_handle_loaded: bool,
    pub highlighted: bool,
}

impl MessageReportTargetMetadata {
    pub fn from_loaded_body(
        item_id: usize,
        body: &str,
        event_id_loaded: bool,
        related_event_loaded: bool,
        thread_root_loaded: bool,
        local_echo_send_handle_loaded: bool,
        highlighted: bool,
    ) -> Self {
        Self {
            item_id,
            body_preview: compact_report_target_preview(body),
            body_char_count: body.chars().count(),
            body_byte_count: body.len(),
            event_id_loaded,
            related_event_loaded,
            thread_root_loaded,
            local_echo_send_handle_loaded,
            highlighted,
        }
    }
}

/// Details about the message that define its context menu content.
#[derive(Clone, Debug)]
pub struct MessageDetails {
    /// The index of this message in its room's timeline.
    pub item_id: usize,
    /// The stable identifier of this event timeline item.
    pub timeline_event_id: TimelineEventItemId,
    /// The event ID of the message that this message is related to, if any,
    /// such as the replied-to message.
    pub related_event_id: Option<OwnedEventId>,
    /// The event ID of the thread root if this message is part of a thread
    /// (or if this message is itself the thread root).
    pub thread_root_event_id: Option<OwnedEventId>,
    /// The widget ID of the RoomScreen that contains this message.
    pub room_screen_widget_uid: WidgetUid,
    /// Whether this message should be highlighted, i.e.,
    /// if it mentions the room/current user or is a reply to the current user.
    pub should_be_highlighted: bool,
    /// The abilities that the user has on this message.
    pub abilities: MessageAbilities,
    /// SDK send-queue handle for a pending local echo, if this row can still be cancelled.
    pub local_send_handle: Option<SendHandle>,
    /// Loaded target metadata shown in the local Report preview before any report is sent.
    pub loaded_report_target_metadata: MessageReportTargetMetadata,
}

impl MessageDetails {
    pub fn event_id(&self) -> Option<&OwnedEventId> {
        match &self.timeline_event_id {
            TimelineEventItemId::EventId(id) => Some(id),
            TimelineEventItemId::TransactionId(_) => None,
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct NewMessageContextMenu {
    #[deref]
    view: View,
    #[source]
    source: ScriptObjectRef,
    #[rust]
    details: Option<MessageDetails>,
}

impl Widget for NewMessageContextMenu {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.details.is_none() {
            self.visible = false;
        };

        let step = self.view.draw_walk(cx, scope, walk);
        if self.visible {
            let main_content_area = self.view(cx, ids!(main_content)).area();
            cx.block_scrolling_except_within(main_content_area);
        }
        step
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }
        self.view.handle_event(cx, event, scope);

        let area = self.view.area();

        // Close the menu if:
        // 1. The back navigational gesture/action occurs (e.g., Back on Android),
        // 2. The escape key is pressed if this menu has key focus,
        // 3. The user clicks/touches outside the main_content view area.
        let close_menu = {
            event.back_pressed()
                || match event.hits_with_capture_overload(cx, area, true) {
                    Hit::KeyUp(key) => key.key_code == KeyCode::Escape,
                    Hit::FingerDown(fde) => {
                        let reaction_text_input = self
                            .view
                            .text_input(cx, ids!(reaction_input_view.reaction_text_input));
                        if reaction_text_input.area().rect(cx).contains(fde.abs) {
                            reaction_text_input.set_key_focus(cx);
                        } else {
                            let report_reason_text_input = self
                                .view
                                .text_input(cx, ids!(report_preview_view.report_reason_text_input));
                            if report_reason_text_input.area().rect(cx).contains(fde.abs) {
                                report_reason_text_input.set_key_focus(cx);
                            } else {
                                cx.set_key_focus(area);
                            }
                        }
                        false
                    }
                    Hit::FingerUp(fue) if fue.is_over => !self
                        .view(cx, ids!(main_content))
                        .area()
                        .rect(cx)
                        .contains(fue.abs),
                    _ => false,
                }
        };
        if close_menu {
            self.close(cx);
            return;
        }

        self.widget_match_event(cx, event, scope);
    }
}

impl WidgetMatchEvent for NewMessageContextMenu {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let Some(details) = self.details.as_ref() else {
            return;
        };
        let mut close_menu = false;

        let reaction_text_input = self
            .view
            .text_input(cx, ids!(reaction_input_view.reaction_text_input));
        let reaction_send_button = self
            .view
            .button(cx, ids!(reaction_input_view.reaction_send_button));
        let report_reason_text_input = self
            .view
            .text_input(cx, ids!(report_preview_view.report_reason_text_input));
        if let Some(reason) = report_reason_text_input.changed(actions) {
            self.update_report_reason_draft_metadata_label(
                cx,
                &reason,
                &details.loaded_report_target_metadata,
            );
        }
        if reaction_send_button.clicked(actions) || reaction_text_input.returned(actions).is_some()
        {
            cx.widget_action(
                details.room_screen_widget_uid,
                MessageAction::React {
                    details: details.clone(),
                    reaction: reaction_text_input.text(),
                },
            );
            close_menu = true;
        } else if reaction_text_input.escaped(actions) {
            close_menu = true;
        } else if self.button(cx, ids!(react_button)).clicked(actions) {
            // Show a box to allow the user to input the reaction.
            // In the future, we'll show an emoji chooser.
            self.view
                .button(cx, ids!(react_button))
                .set_visible(cx, false);
            self.view
                .view(cx, ids!(reaction_input_view))
                .set_visible(cx, true);
            self.text_input(cx, ids!(reaction_input_view.reaction_text_input))
                .set_key_focus(cx);
            self.redraw(cx);
            close_menu = false;
        } else if self.button(cx, ids!(reply_button)).clicked(actions) {
            cx.widget_action(
                details.room_screen_widget_uid,
                MessageAction::Reply(details.clone()),
            );
            close_menu = true;
        } else if self.button(cx, ids!(edit_message_button)).clicked(actions) {
            cx.widget_action(
                details.room_screen_widget_uid,
                MessageAction::Edit(details.clone()),
            );
            close_menu = true;
        } else if self.button(cx, ids!(pin_button)).clicked(actions) {
            if details.abilities.contains(MessageAbilities::CanPin) {
                let pin_details = details.clone();
                let room_screen_widget_uid = details.room_screen_widget_uid;
                let event_label = details
                    .event_id()
                    .map(|event_id| event_id.as_str().to_string())
                    .unwrap_or_else(|| "this pending local event".to_string());
                let content = ConfirmationModalContent {
                    title_text: "Pin Message".into(),
                    body_text: format!(
                        "Pin {event_label}? {MESSAGE_CONTEXT_CONFIRMATION_COMPACT_LABEL}"
                    )
                    .into(),
                    accept_button_text: Some("Pin".into()),
                    cancel_button_text: Some("Cancel".into()),
                    on_accept_clicked: Some(Box::new(move |cx| {
                        cx.widget_action(room_screen_widget_uid, MessageAction::Pin(pin_details));
                        enqueue_popup_notification(
                            "Pin confirmed. Existing Matrix pin/unpin path was requested.",
                            PopupKind::Info,
                            Some(4.0),
                        );
                    })),
                    on_cancel_clicked: Some(Box::new(|_cx| {
                        enqueue_popup_notification(
                            "Pin canceled. Matrix pin/unpin request was not sent.",
                            PopupKind::Info,
                            Some(3.0),
                        );
                    })),
                };
                enqueue_popup_notification(
                    format!(
                        "Pin confirmation opened. {MESSAGE_CONTEXT_CONFIRMATION_COMPACT_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                    content,
                ))));
            } else if details.abilities.contains(MessageAbilities::CanUnpin) {
                let unpin_details = details.clone();
                let room_screen_widget_uid = details.room_screen_widget_uid;
                let event_label = details
                    .event_id()
                    .map(|event_id| event_id.as_str().to_string())
                    .unwrap_or_else(|| "this pending local event".to_string());
                let content = ConfirmationModalContent {
                    title_text: "Unpin Message".into(),
                    body_text: format!(
                        "Unpin {event_label}? {MESSAGE_CONTEXT_CONFIRMATION_COMPACT_LABEL}"
                    )
                    .into(),
                    accept_button_text: Some("Unpin".into()),
                    cancel_button_text: Some("Cancel".into()),
                    on_accept_clicked: Some(Box::new(move |cx| {
                        cx.widget_action(
                            room_screen_widget_uid,
                            MessageAction::Unpin(unpin_details),
                        );
                        enqueue_popup_notification(
                            "Unpin confirmed. Existing Matrix pin/unpin path was requested.",
                            PopupKind::Info,
                            Some(4.0),
                        );
                    })),
                    on_cancel_clicked: Some(Box::new(|_cx| {
                        enqueue_popup_notification(
                            "Unpin canceled. Matrix pin/unpin request was not sent.",
                            PopupKind::Info,
                            Some(3.0),
                        );
                    })),
                };
                enqueue_popup_notification(
                    format!(
                        "Unpin confirmation opened. {MESSAGE_CONTEXT_CONFIRMATION_COMPACT_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                cx.action(ConfirmDeleteAction::Show(RefCell::new(Some(content))));
            }
            close_menu = true;
        } else if self.button(cx, ids!(copy_text_button)).clicked(actions) {
            cx.widget_action(
                details.room_screen_widget_uid,
                MessageAction::CopyText(details.clone()),
            );
            close_menu = true;
        } else if self.button(cx, ids!(copy_html_button)).clicked(actions) {
            cx.widget_action(
                details.room_screen_widget_uid,
                MessageAction::CopyHtml(details.clone()),
            );
            close_menu = true;
        } else if self
            .button(cx, ids!(copy_link_to_message_button))
            .clicked(actions)
        {
            cx.widget_action(
                details.room_screen_widget_uid,
                MessageAction::CopyLink(details.clone()),
            );
            close_menu = true;
        } else if self.button(cx, ids!(view_source_button)).clicked(actions) {
            cx.widget_action(
                details.room_screen_widget_uid,
                MessageAction::ViewSource(details.clone()),
            );
            close_menu = true;
        } else if self
            .button(cx, ids!(jump_to_related_button))
            .clicked(actions)
        {
            cx.widget_action(
                details.room_screen_widget_uid,
                MessageAction::JumpToRelated(details.clone()),
            );
            close_menu = true;
        } else if self.button(cx, ids!(report_button)).clicked(actions) {
            self.view
                .view(cx, ids!(report_preview_view))
                .set_visible(cx, true);
            self.view
                .button(cx, ids!(report_button))
                .set_visible(cx, false);
            enqueue_popup_notification(
                MESSAGE_REPORT_SEND_COMPACT_LABEL,
                PopupKind::Info,
                Some(5.0),
            );
            self.update_report_reason_draft_metadata_label(
                cx,
                &report_reason_text_input.text(),
                &details.loaded_report_target_metadata,
            );
            self.text_input(cx, ids!(report_preview_view.report_reason_text_input))
                .set_key_focus(cx);
            self.redraw(cx);
            close_menu = false;
        } else if self
            .button(cx, ids!(cancel_local_send_button))
            .clicked(actions)
        {
            let cancel_details = details.clone();
            let room_screen_widget_uid = details.room_screen_widget_uid;
            let content = ConfirmationModalContent {
                title_text: "Cancel Send".into(),
                body_text: format!(
                    "Cancel this queued local send? {MESSAGE_LOCAL_SEND_CANCEL_COMPACT_LABEL}"
                )
                .into(),
                accept_button_text: Some("Cancel Send".into()),
                cancel_button_text: Some("Keep Sending".into()),
                on_accept_clicked: Some(Box::new(move |cx| {
                    cx.widget_action(
                        room_screen_widget_uid,
                        MessageAction::CancelLocalSend(cancel_details),
                    );
                    enqueue_popup_notification(
                        "Cancel queued send requested for this local echo.",
                        PopupKind::Info,
                        Some(4.0),
                    );
                })),
                on_cancel_clicked: Some(Box::new(|_cx| {
                    enqueue_popup_notification(
                        "Queued send kept. No SendHandle abort was requested.",
                        PopupKind::Info,
                        Some(3.0),
                    );
                })),
            };
            enqueue_popup_notification(
                format!(
                    "Cancel Send confirmation opened. {MESSAGE_LOCAL_SEND_CANCEL_COMPACT_LABEL}"
                ),
                PopupKind::Info,
                Some(4.0),
            );
            cx.action(ConfirmDeleteAction::Show(RefCell::new(Some(content))));
            close_menu = true;
        } else if self
            .button(cx, ids!(report_preview_view.report_spam_button))
            .clicked(actions)
        {
            self.show_report_confirmation(cx, details, "Spam", "spam".to_string());
            close_menu = true;
        } else if self
            .button(cx, ids!(report_preview_view.report_abuse_button))
            .clicked(actions)
        {
            self.show_report_confirmation(cx, details, "Abuse", "abuse".to_string());
            close_menu = true;
        } else if self
            .button(cx, ids!(report_preview_view.report_custom_button))
            .clicked(actions)
            || report_reason_text_input.returned(actions).is_some()
        {
            let reason = compact_custom_report_reason(&report_reason_text_input.text());
            if reason.is_empty() {
                enqueue_popup_notification(
                    "Custom report reason is empty. Matrix report request was not sent.",
                    PopupKind::Warning,
                    Some(4.0),
                );
                close_menu = false;
            } else {
                self.show_report_confirmation(cx, details, "Custom", reason);
                close_menu = true;
            }
        } else if report_reason_text_input.escaped(actions) {
            self.view
                .view(cx, ids!(report_preview_view))
                .set_visible(cx, false);
            self.view
                .button(cx, ids!(report_button))
                .set_visible(cx, true);
            enqueue_popup_notification(
                format!("Report preview canceled locally. {MESSAGE_REPORT_CANCEL_LOCAL_LABEL}"),
                PopupKind::Info,
                Some(3.0),
            );
            self.redraw(cx);
            close_menu = true;
        } else if self
            .button(cx, ids!(report_preview_view.report_cancel_button))
            .clicked(actions)
        {
            self.view
                .view(cx, ids!(report_preview_view))
                .set_visible(cx, false);
            self.view
                .button(cx, ids!(report_button))
                .set_visible(cx, true);
            enqueue_popup_notification(
                format!("Report preview canceled locally. {MESSAGE_REPORT_CANCEL_LOCAL_LABEL}"),
                PopupKind::Info,
                Some(3.0),
            );
            self.redraw(cx);
            close_menu = false;
        } else if self.button(cx, ids!(delete_button)).clicked(actions) {
            let delete_details = details.clone();
            let room_screen_widget_uid = details.room_screen_widget_uid;
            let event_label = details
                .event_id()
                .map(|event_id| event_id.as_str().to_string())
                .unwrap_or_else(|| "this pending local event".to_string());
            let content = ConfirmationModalContent {
                title_text: "Delete Message".into(),
                body_text: format!(
                    "Delete {event_label} for everyone? {MESSAGE_CONTEXT_CONFIRMATION_COMPACT_LABEL}"
                )
                .into(),
                accept_button_text: Some("Delete".into()),
                cancel_button_text: Some("Cancel".into()),
                on_accept_clicked: Some(Box::new(move |cx| {
                    cx.widget_action(
                        room_screen_widget_uid,
                        MessageAction::Redact {
                            details: delete_details,
                            reason: None,
                        },
                    );
                    enqueue_popup_notification(
                        "Delete confirmed. Existing Matrix redaction path was requested.",
                        PopupKind::Info,
                        Some(4.0),
                    );
                })),
                on_cancel_clicked: Some(Box::new(|_cx| {
                    enqueue_popup_notification(
                        "Delete canceled. Matrix redaction request was not sent.",
                        PopupKind::Info,
                        Some(3.0),
                    );
                })),
            };
            enqueue_popup_notification(
                format!("Delete confirmation opened. {MESSAGE_CONTEXT_CONFIRMATION_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(4.0),
            );
            cx.action(ConfirmDeleteAction::Show(RefCell::new(Some(content))));
            close_menu = true;
        }

        if close_menu {
            self.close(cx);
        }
    }
}

impl NewMessageContextMenu {
    /// Returns `true` if this menu is currently being shown.
    pub fn is_currently_shown(&self, _cx: &mut Cx) -> bool {
        self.visible
    }

    /// Shows this context menu with the given message details.
    ///
    /// Returns the expected (approximate) dimensions of the context menu,
    /// which can be used to proactively reposition it such that it fits on screen.
    pub fn show(&mut self, cx: &mut Cx, details: MessageDetails) -> DVec2 {
        self.details = Some(details);
        self.visible = true;
        cx.set_key_focus(self.view.area());

        // log!("Showing context menu for message: {:?}", self.details);
        let height = self.set_button_visibility(cx);

        dvec2(MENU_WIDTH, height)
    }

    /// Sets up all of the buttons based this context menu's inner details.
    ///
    /// Returns the total height of all visible items.
    fn set_button_visibility(&mut self, cx: &mut Cx) -> f64 {
        let Some(details) = self.details.as_ref() else {
            return 0.0;
        };

        let react_button = self.view.button(cx, ids!(react_button));
        let reply_button = self.view.button(cx, ids!(reply_button));
        let edit_button = self.view.button(cx, ids!(edit_message_button));
        let pin_button = self.view.button(cx, ids!(pin_button));
        let copy_text_button = self.view.button(cx, ids!(copy_text_button));
        let copy_html_button = self.view.button(cx, ids!(copy_html_button));
        let copy_link_button = self.view.button(cx, ids!(copy_link_to_message_button));
        let view_source_button = self.view.button(cx, ids!(view_source_button));
        let jump_to_related_button = self.view.button(cx, ids!(jump_to_related_button));
        let report_button = self.view.button(cx, ids!(report_button));
        let cancel_local_send_button = self.view.button(cx, ids!(cancel_local_send_button));
        let delete_button = self.view.button(cx, ids!(delete_button));

        // Determine which buttons should be shown.
        // Note that some buttons are always enabled:
        // `copy_text_button`, `copy_link_to_message_button`, and `view_source_button`
        let show_react = details.abilities.contains(MessageAbilities::CanReact);
        let show_reply_to = details.abilities.contains(MessageAbilities::CanReplyTo);
        let show_divider_after_react_reply = show_react || show_reply_to;
        let show_edit = details.abilities.contains(MessageAbilities::CanEdit);
        let show_pin: bool;
        let show_copy_text = true;
        let show_copy_html = details.abilities.contains(MessageAbilities::HasHtml);
        let show_copy_link = true;
        let show_view_source = true;
        let show_jump_to_related = details.related_event_id.is_some();
        let show_report = true;
        let show_cancel_local_send = details
            .abilities
            .contains(MessageAbilities::CanCancelLocalSend);
        let show_delete = details.abilities.contains(MessageAbilities::CanDelete);
        let show_divider_before_report_delete =
            show_delete || show_report || show_cancel_local_send;

        // Actually set the buttons' visibility.
        self.view
            .view(cx, ids!(react_view))
            .set_visible(cx, show_react);
        react_button.set_visible(cx, show_react);
        reply_button.set_visible(cx, show_reply_to);
        self.view
            .view(cx, ids!(divider_after_react_reply))
            .set_visible(cx, show_divider_after_react_reply);
        edit_button.set_visible(cx, show_edit);
        if details.abilities.contains(MessageAbilities::CanPin) {
            pin_button.set_text(cx, "Pin Message");
            show_pin = true;
        } else if details.abilities.contains(MessageAbilities::CanUnpin) {
            pin_button.set_text(cx, "Unpin Message");
            show_pin = true;
        } else {
            show_pin = false;
        }
        pin_button.set_visible(cx, show_pin);
        copy_html_button.set_visible(cx, show_copy_html);
        jump_to_related_button.set_visible(cx, show_jump_to_related);
        self.view
            .view(cx, ids!(divider_before_report_delete))
            .set_visible(cx, show_divider_before_report_delete);
        report_button.set_visible(cx, show_report);
        cancel_local_send_button.set_visible(cx, show_cancel_local_send);
        self.view
            .view(cx, ids!(report_preview_view))
            .set_visible(cx, false);
        self.view
            .label(cx, ids!(report_preview_view.report_preview_summary))
            .set_text(
                cx,
                &loaded_message_report_target_metadata_label(
                    &details.loaded_report_target_metadata,
                ),
            );
        self.view
            .label(cx, ids!(report_preview_view.report_option_evidence))
            .set_text(cx, MESSAGE_REPORT_LOADED_TARGET_METADATA_LABEL);
        self.update_report_reason_draft_metadata_label(
            cx,
            "",
            &details.loaded_report_target_metadata,
        );
        delete_button.set_visible(cx, show_delete);

        // Reset the hover state of each button.
        react_button.reset_hover(cx);
        reply_button.reset_hover(cx);
        edit_button.reset_hover(cx);
        pin_button.reset_hover(cx);
        copy_text_button.reset_hover(cx);
        copy_html_button.reset_hover(cx);
        copy_link_button.reset_hover(cx);
        view_source_button.reset_hover(cx);
        jump_to_related_button.reset_hover(cx);
        report_button.reset_hover(cx);
        cancel_local_send_button.reset_hover(cx);
        delete_button.reset_hover(cx);

        // Reset reaction input view stuff.
        self.view
            .view(cx, ids!(reaction_input_view))
            .set_visible(cx, false); // hide until the react_button is clicked
        self.text_input(cx, ids!(reaction_input_view.reaction_text_input))
            .set_text(cx, "");
        self.text_input(cx, ids!(report_preview_view.report_reason_text_input))
            .set_text(cx, "");
        self.update_report_reason_draft_metadata_label(
            cx,
            "",
            &details.loaded_report_target_metadata,
        );

        self.redraw(cx);

        let num_visible_buttons = show_react as u8
            + show_reply_to as u8
            + show_edit as u8
            + show_pin as u8
            + show_copy_text as u8
            + show_copy_html as u8
            + show_copy_link as u8
            + show_view_source as u8
            + show_jump_to_related as u8
            + show_report as u8
            + show_cancel_local_send as u8
            + show_delete as u8;

        // Calculate and return the total expected height:
        (num_visible_buttons as f64 * BUTTON_HEIGHT)
            + if show_divider_after_react_reply { 10.0 } else { 0.0 }
            + if show_divider_before_report_delete { 10.0 } else { 0.0 }
            + 20.0  // top and bottom padding
            + 1.0 // top and bottom border
    }

    fn show_report_confirmation(
        &self,
        cx: &mut Cx,
        details: &MessageDetails,
        label: &'static str,
        reason: String,
    ) {
        let report_details = details.clone();
        let room_screen_widget_uid = details.room_screen_widget_uid;
        let event_label = details
            .event_id()
            .map(|event_id| event_id.as_str().to_string())
            .unwrap_or_else(|| "this pending local event".to_string());
        let target_summary =
            loaded_message_report_target_confirmation_label(&details.loaded_report_target_metadata);
        let body_text = if label == "Custom" {
            format!(
                "Report {event_label} with this custom reason: \"{reason}\"? Target: {target_summary}. {MESSAGE_REPORT_CONFIRMATION_COMPACT_LABEL}"
            )
        } else {
            format!(
                "Report {event_label} as {label}? Target: {target_summary}. {MESSAGE_REPORT_CONFIRMATION_COMPACT_LABEL}"
            )
        };
        let content = ConfirmationModalContent {
            title_text: "Report Message".into(),
            body_text: body_text.into(),
            accept_button_text: Some("Report".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                cx.widget_action(
                    room_screen_widget_uid,
                    MessageAction::Report {
                        details: report_details,
                        reason: reason.to_string(),
                    },
                );
                enqueue_popup_notification(
                    "Report confirmed. Matrix report_content was requested.",
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(|_cx| {
                enqueue_popup_notification(
                    "Report canceled. Matrix report request was not sent.",
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        enqueue_popup_notification(
            format!("Report confirmation opened. {MESSAGE_REPORT_CONFIRMATION_COMPACT_LABEL}"),
            PopupKind::Info,
            Some(4.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn update_report_reason_draft_metadata_label(
        &self,
        cx: &mut Cx,
        raw_reason: &str,
        target_metadata: &MessageReportTargetMetadata,
    ) {
        self.view
            .label(cx, ids!(report_preview_view.report_reason_draft_metadata))
            .set_text(
                cx,
                &message_report_custom_reason_draft_metadata_label(raw_reason, target_metadata),
            );
        self.view
            .label(cx, ids!(report_preview_view.report_moderation_boundary))
            .set_text(
                cx,
                &message_report_moderation_workflow_boundary_label(raw_reason, target_metadata),
            );
    }

    fn close(&mut self, cx: &mut Cx) {
        self.visible = false;
        self.details = None;
        cx.revert_key_focus();
        cx.unblock_scrolling();
        self.redraw(cx);
    }
}

fn compact_custom_report_reason(raw_reason: &str) -> String {
    raw_reason
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(240)
        .collect()
}

pub(crate) fn message_report_custom_reason_draft_metadata_label(
    raw_reason: &str,
    target_metadata: &MessageReportTargetMetadata,
) -> String {
    let compact_uncapped = raw_reason.split_whitespace().collect::<Vec<_>>().join(" ");
    let compact_reason = compact_custom_report_reason(raw_reason);
    let cap_state = if compact_uncapped.chars().count() > compact_reason.chars().count() {
        "240 char cap applied"
    } else {
        "within 240 char cap"
    };
    let readiness_state = if compact_reason.is_empty() {
        "empty draft; Send Custom stays local"
    } else {
        "ready for confirmation only"
    };
    let event_id_state = if target_metadata.event_id_loaded {
        "event id loaded"
    } else {
        "pending local event"
    };

    format!(
        "Custom reason draft: raw {} chars/{} bytes; trimmed {} chars/{} bytes; {}; {}; target row {}, {}. {}",
        raw_reason.chars().count(),
        raw_reason.len(),
        compact_reason.chars().count(),
        compact_reason.len(),
        cap_state,
        readiness_state,
        target_metadata.item_id,
        event_id_state,
        MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_LABEL
    )
}

pub(crate) fn message_report_moderation_workflow_boundary_label(
    raw_reason: &str,
    target_metadata: &MessageReportTargetMetadata,
) -> String {
    let reason_state = if compact_custom_report_reason(raw_reason).is_empty() {
        "custom reason empty"
    } else {
        "custom reason ready for confirmation"
    };
    let event_state = if target_metadata.event_id_loaded {
        "event id loaded"
    } else {
        "pending local event"
    };
    format!(
        "Moderation workflow boundary: target row {}, {}, {}; queue controls, policy lookup, redact/delete, ban, kick, ignore/block, reviewer assignment, appeal flow, room-state, membership, gateway/runtime/auth, and live mutation stay unwired. {}",
        target_metadata.item_id,
        event_state,
        reason_state,
        MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_LABEL
    )
}

fn compact_report_target_preview(body: &str) -> String {
    let compact = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.is_empty() {
        return "empty loaded body".to_string();
    }

    let mut preview = compact.chars().take(72).collect::<String>();
    if compact.chars().count() > 72 {
        preview.push_str("...");
    }
    preview
}

pub(crate) fn loaded_message_report_target_metadata_label(
    metadata: &MessageReportTargetMetadata,
) -> String {
    let event_id_state = if metadata.event_id_loaded {
        "event id loaded"
    } else {
        "event id missing"
    };
    let related_state = if metadata.related_event_loaded {
        "related event loaded"
    } else {
        "related event missing"
    };
    let thread_state = if metadata.thread_root_loaded {
        "thread root loaded"
    } else {
        "thread root missing"
    };
    let local_echo_state = if metadata.local_echo_send_handle_loaded {
        "local echo present"
    } else {
        "local echo absent"
    };
    let highlight_state = if metadata.highlighted {
        "highlighted"
    } else {
        "not highlighted"
    };

    format!(
        "Loaded report target: row {}, {}, {} chars, {} bytes, {}, {}, {}, {}. Preview: {}. {}",
        metadata.item_id,
        event_id_state,
        metadata.body_char_count,
        metadata.body_byte_count,
        related_state,
        thread_state,
        local_echo_state,
        highlight_state,
        metadata.body_preview,
        MESSAGE_REPORT_LOADED_TARGET_METADATA_LABEL
    )
}

fn loaded_message_report_target_confirmation_label(
    metadata: &MessageReportTargetMetadata,
) -> String {
    let event_id_state = if metadata.event_id_loaded {
        "event id loaded"
    } else {
        "pending local event"
    };
    format!(
        "row {}, {}, {} chars, {} bytes",
        metadata.item_id, event_id_state, metadata.body_char_count, metadata.body_byte_count
    )
}

#[cfg(test)]
mod message_report_target_metadata_tests {
    use super::*;

    #[test]
    fn loaded_message_report_target_metadata_label_summarizes_loaded_target() {
        let metadata = MessageReportTargetMetadata::from_loaded_body(
            12,
            "Hello report target",
            true,
            true,
            false,
            false,
            true,
        );

        let label = loaded_message_report_target_metadata_label(&metadata);

        assert!(label.contains("Loaded report target: row 12"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("19 chars"));
        assert!(label.contains("19 bytes"));
        assert!(label.contains("related event loaded"));
        assert!(label.contains("thread root missing"));
        assert!(label.contains("local echo absent"));
        assert!(label.contains("highlighted"));
        assert!(label.contains("Preview: Hello report target"));
        assert!(label.contains("no moderation lookup or report before confirmation"));
    }

    #[test]
    fn loaded_message_report_target_metadata_label_counts_bytes() {
        let metadata = MessageReportTargetMetadata::from_loaded_body(
            3, "hello!", false, false, true, true, false,
        );

        let label = loaded_message_report_target_metadata_label(&metadata);

        assert!(label.contains("6 chars"));
        assert!(label.contains("6 bytes"));
        assert!(label.contains("event id missing"));
        assert!(label.contains("thread root loaded"));
        assert!(label.contains("local echo present"));
        assert!(label.contains("not highlighted"));
    }

    #[test]
    fn message_report_custom_reason_draft_metadata_label_tracks_empty_local_draft() {
        let metadata = MessageReportTargetMetadata::from_loaded_body(
            7,
            "reported message",
            false,
            false,
            false,
            false,
            false,
        );

        let label = message_report_custom_reason_draft_metadata_label("   ", &metadata);

        assert!(label.contains("raw 3 chars/3 bytes"));
        assert!(label.contains("trimmed 0 chars/0 bytes"));
        assert!(label.contains("within 240 char cap"));
        assert!(label.contains("empty draft; Send Custom stays local"));
        assert!(label.contains("target row 7"));
        assert!(label.contains("pending local event"));
        assert!(label.contains("no ReportContent before confirmation"));
    }

    #[test]
    fn message_report_custom_reason_draft_metadata_label_marks_ready_capped_draft() {
        let metadata = MessageReportTargetMetadata::from_loaded_body(
            11,
            "reported message",
            true,
            false,
            false,
            false,
            false,
        );
        let long_reason = format!("  {}  ", "a".repeat(260));

        let label = message_report_custom_reason_draft_metadata_label(&long_reason, &metadata);

        assert!(label.contains("trimmed 240 chars/240 bytes"));
        assert!(label.contains("240 char cap applied"));
        assert!(label.contains("ready for confirmation only"));
        assert!(label.contains("target row 11"));
        assert!(label.contains("event id loaded"));
    }

    #[test]
    fn message_report_moderation_workflow_boundary_stays_local_only() {
        let metadata = MessageReportTargetMetadata::from_loaded_body(
            19,
            "reported message",
            true,
            false,
            false,
            false,
            false,
        );

        let label = message_report_moderation_workflow_boundary_label(" spam ", &metadata);

        assert!(label.contains("Moderation workflow boundary"));
        assert!(label.contains("target row 19"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("custom reason ready for confirmation"));
        assert!(label.contains("queue controls"));
        assert!(label.contains("policy lookup"));
        assert!(label.contains("redact/delete"));
        assert!(label.contains("ban"));
        assert!(label.contains("kick"));
        assert!(label.contains("ignore/block"));
        assert!(label.contains("reviewer assignment"));
        assert!(label.contains("appeal flow"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_LABEL));
        assert!(
            MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE.contains("local blocked controls")
        );
        assert!(
            MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE
                .contains("confirmed MatrixRequest::ReportContent")
        );
    }

    #[test]
    fn message_report_moderation_workflow_boundary_reports_empty_pending_state() {
        let metadata = MessageReportTargetMetadata::from_loaded_body(
            2,
            "pending local report",
            false,
            false,
            false,
            false,
            false,
        );

        let label = message_report_moderation_workflow_boundary_label("   ", &metadata);

        assert!(label.contains("target row 2"));
        assert!(label.contains("pending local event"));
        assert!(label.contains("custom reason empty"));
    }
}

impl NewMessageContextMenuRef {
    /// See [`NewMessageContextMenu::is_currently_shown()`].
    pub fn is_currently_shown(&self, cx: &mut Cx) -> bool {
        let Some(inner) = self.borrow() else {
            return false;
        };
        inner.is_currently_shown(cx)
    }

    /// See [`NewMessageContextMenu::show()`].
    pub fn show(&self, cx: &mut Cx, details: MessageDetails) -> DVec2 {
        let Some(mut inner) = self.borrow_mut() else {
            return DVec2::default();
        };
        inner.show(cx, details)
    }
}
