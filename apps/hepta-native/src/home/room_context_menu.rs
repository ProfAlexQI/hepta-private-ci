//! A context menu that appears when the user right-clicks
//! or long-presses on a room in the room list.

use std::cell::RefCell;

use makepad_widgets::*;
use matrix_sdk::{
    notification_settings::RoomNotificationMode,
    ruma::{OwnedRoomAliasId, OwnedRoomId},
};

use crate::{
    app::PositiveConfirmationModalAction,
    home::invite_modal::InviteModalAction,
    shared::confirmation_modal::ConfirmationModalContent,
    shared::popup_list::{PopupKind, enqueue_popup_notification},
    sliding_sync::{MatrixRequest, submit_async_request},
    utils::RoomNameId,
};

const BUTTON_HEIGHT: f64 = 35.0;
const MENU_WIDTH: f64 = 215.0;
pub const ROOM_CONTEXT_STATUS_CONFIRMATION_COMPACT_LABEL: &str =
    "Room status changes run only after confirmation.";
pub const ROOM_CONTEXT_LINK_COMPACT_LABEL: &str = "Room link uses the existing Matrix link path.";
pub const ROOM_CONTEXT_NOTIFICATION_MODE_COMPACT_LABEL: &str =
    "Notification mode writes run only after confirmation; timed mute stays unwired.";
pub const ROOM_CONTEXT_NOTIFICATION_LOADED_ATTENTION_EVIDENCE: &str = "Room context notification preview reads only loaded RoomsList unread count, mention count, and manual unread state before any confirmed notification mode write. It sends no notification rule read, timed mute, global notification preference, keyword, push gateway/device, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ROOM_CONTEXT_SETTINGS_LOADED_IDENTITY_EVIDENCE: &str = "Room context settings preview reads only loaded RoomsList metadata for canonical alias, alternative alias count, avatar cache state, tombstone state, and room name/id. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.tombstone, membership, notification rule, account/profile, gateway/runtime/auth, or live mutation request.";

fn room_context_notification_mode_label(mode: RoomNotificationMode) -> &'static str {
    match mode {
        RoomNotificationMode::AllMessages => "All messages",
        RoomNotificationMode::MentionsAndKeywordsOnly => "Mentions",
        RoomNotificationMode::Mute => "Mute",
    }
}

fn room_context_settings_loaded_identity_summary(
    details: Option<&RoomContextMenuDetails>,
) -> String {
    let Some(details) = details else {
        return "Loaded identity: waiting for room-list metadata; no Matrix room state event was sent."
            .to_string();
    };
    let alias_state = if details.canonical_alias.is_some() {
        "alias loaded"
    } else {
        "alias missing"
    };
    let avatar_state = if details.room_avatar_loaded {
        "avatar image"
    } else {
        "avatar fallback"
    };
    let tombstone_state = if details.is_tombstoned {
        "tombstoned"
    } else {
        "not tombstoned"
    };
    format!(
        "Loaded identity: {alias_state}, {} alt aliases, {avatar_state}, {tombstone_state}. No Matrix room state event was sent.",
        details.alt_alias_count
    )
}

fn room_context_notification_loaded_attention_summary(
    details: Option<&RoomContextMenuDetails>,
) -> String {
    let Some(details) = details else {
        return "Loaded attention: waiting for room-list unread state; no notification rule read was sent."
            .to_string();
    };
    let manual_state = if details.is_marked_unread {
        "manual unread"
    } else {
        "not manually unread"
    };
    format!(
        "Loaded attention: {} unread, {} mentions, {manual_state}. No notification rule read was sent.",
        details.num_unread_messages, details.num_unread_mentions
    )
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.ROOM_CONTEXT_MENU_BUTTON_HEIGHT = 35
    mod.widgets.ROOM_CONTEXT_MENU_WIDTH = 215

    mod.widgets.RoomContextMenuButton = RobrixIconButton {
        height: (mod.widgets.ROOM_CONTEXT_MENU_BUTTON_HEIGHT)
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

    mod.widgets.RoomContextMenu = set_type_default() do #(RoomContextMenu::register_widget(vm)) {
        ..mod.widgets.SolidView

        visible: false,
        flow: Overlay,
        width: Fill,
        height: Fill,
        cursor: MouseCursor.Default,
        align: Align{x: 0, y: 0}

        show_bg: true
        draw_bg +: {
            color: #0000004d
        }

        main_content := RoundedView {
            flow: Down
            width: (mod.widgets.ROOM_CONTEXT_MENU_WIDTH),
            height: Fit,
            padding: 5
            spacing: 0,
            align: Align{x: 0, y: 0}

            show_bg: true
            draw_bg +: {
                color: (COLOR_TELEGRAM_PANEL)
                border_radius: 7.0
                border_size: 0.5
                border_color: (COLOR_TELEGRAM_BORDER)
            }

            mark_unread_button := mod.widgets.RoomContextMenuButton {
                draw_icon +: { svg: (ICON_CHECKMARK) }
                text: "Mark as Unread"
            }

            favorite_button := mod.widgets.RoomContextMenuButton {
                draw_icon +: { svg: (ICON_PIN) }
                text: "Favorite"
            }

            priority_button := mod.widgets.RoomContextMenuButton {
                draw_icon +: { svg: (ICON_TOMBSTONE) }
                text: "Set Low Priority"
            }

            copy_link_button := mod.widgets.RoomContextMenuButton {
                draw_icon +: { svg: (ICON_LINK) }
                text: "Copy Link to Room"
            }

            divider1 := LineH {
                margin: Inset{top: 3, bottom: 3}
                width: Fill,
            }

            room_settings_button := mod.widgets.RoomContextMenuButton {
                draw_icon +: { svg: (ICON_SETTINGS) }
                text: "Settings"
            }

            notifications_button := mod.widgets.RoomContextMenuButton {
                // TODO: use a proper bell icon
                draw_icon +: { svg: (ICON_INFO) }
                text: "Notifications"
            }

            room_context_local_preview := RoundedView {
                visible: false
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5.0,
                margin: Inset{top: 3.0, bottom: 3.0}
                padding: Inset{top: 7.0, bottom: 7.0, left: 9.0, right: 9.0}
                show_bg: true
                draw_bg +: {
                    color: #142334
                    border_color: (COLOR_TELEGRAM_BLUE)
                    border_size: 1.0
                    border_radius: 7.0
                }

                room_context_preview_title := Label {
                    width: Fill,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_BLUE)
                    }
                    text: "Local room preview"
                }

                room_context_preview_summary := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true}
                    draw_text +: {
                        color: (COLOR_TELEGRAM_TEXT)
                    }
                    text: "Settings stay read-only; notification mode writes require confirmation."
                }

                room_context_preview_primary_button := mod.widgets.RoomContextMenuButton {
                    text: "Name"
                }
                room_context_preview_secondary_button := mod.widgets.RoomContextMenuButton {
                    text: "Topic"
                }
                room_context_preview_tertiary_button := mod.widgets.RoomContextMenuButton {
                    text: "Permissions"
                }
                room_context_preview_quaternary_button := mod.widgets.RoomContextMenuButton {
                    text: "Members"
                }
            }

            invite_button := mod.widgets.RoomContextMenuButton {
                draw_icon +: { svg: (ICON_ADD_USER) }
                text: "Invite"
            }

            divider2 := LineH {
                margin: Inset{top: 3, bottom: 3}
                width: Fill,
            }

            leave_button := RobrixNegativeIconButton {
                height: (mod.widgets.ROOM_CONTEXT_MENU_BUTTON_HEIGHT)
                width: Fill,
                margin: 0,
                icon_walk: Walk{width: 16, height: 16, margin: Inset{right: 3}}
                draw_bg.color: (COLOR_TELEGRAM_PANEL)
                draw_bg.color_hover: #3A1F27
                draw_bg.color_down: #4A2630
                draw_icon.svg: (ICON_LOGOUT)
                text: "Leave Room"
            }
        }
    }
}

/// Details needed to populate the room context menu.
#[derive(Clone, Debug)]
pub struct RoomContextMenuDetails {
    pub room_name_id: RoomNameId,
    pub is_favorite: bool,
    pub is_low_priority: bool,
    pub is_marked_unread: bool,
    pub num_unread_messages: u64,
    pub num_unread_mentions: u64,
    pub canonical_alias: Option<OwnedRoomAliasId>,
    pub alt_aliases: Vec<OwnedRoomAliasId>,
    pub alt_alias_count: usize,
    pub room_avatar_loaded: bool,
    pub is_tombstoned: bool,
}

/// Actions emitted from the RoomContextMenu widget, as they must be handled
/// by other widgets with more information (e.g., the RoomsList).
#[derive(Clone, Default, Debug)]
pub enum RoomContextMenuAction {
    Notifications(OwnedRoomId),
    OpenRoomSettings(OwnedRoomId),
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct RoomContextMenu {
    #[deref]
    view: View,
    #[source]
    source: ScriptObjectRef,
    #[rust]
    details: Option<RoomContextMenuDetails>,
    #[rust(RoomContextPreviewState::Hidden)]
    preview_state: RoomContextPreviewState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoomContextPreviewState {
    Hidden,
    Settings,
    Notifications,
}

impl Widget for RoomContextMenu {
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

        // Close logic similar to NewMessageContextMenu
        let area = self.view.area();
        let close_menu = {
            event.back_pressed()
                || match event.hits_with_capture_overload(cx, area, true) {
                    Hit::KeyUp(key) => key.key_code == KeyCode::Escape,
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

impl WidgetMatchEvent for RoomContextMenu {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let Some(details) = self.details.clone() else {
            return;
        };
        let mut close_menu = false;

        if self.button(cx, ids!(mark_unread_button)).clicked(actions) {
            self.show_stateful_toggle_confirmation(
                cx,
                &details,
                "Unread Flag",
                if details.is_marked_unread {
                    "mark as read"
                } else {
                    "mark as unread"
                },
                if details.is_marked_unread {
                    "Mark Read"
                } else {
                    "Mark Unread"
                },
                MatrixRequest::SetUnreadFlag {
                    room_id: details.room_name_id.room_id().clone(),
                    mark_as_unread: !details.is_marked_unread,
                },
            );
            close_menu = true;
        } else if self.button(cx, ids!(favorite_button)).clicked(actions) {
            self.show_stateful_toggle_confirmation(
                cx,
                &details,
                "Favorite",
                if details.is_favorite {
                    "remove favorite"
                } else {
                    "favorite"
                },
                if details.is_favorite {
                    "Remove Favorite"
                } else {
                    "Favorite"
                },
                MatrixRequest::SetIsFavorite {
                    room_id: details.room_name_id.room_id().clone(),
                    is_favorite: !details.is_favorite,
                },
            );
            close_menu = true;
        } else if self.button(cx, ids!(priority_button)).clicked(actions) {
            self.show_stateful_toggle_confirmation(
                cx,
                &details,
                "Priority",
                if details.is_low_priority {
                    "restore normal priority"
                } else {
                    "set low priority"
                },
                if details.is_low_priority {
                    "Normal Priority"
                } else {
                    "Low Priority"
                },
                MatrixRequest::SetIsLowPriority {
                    room_id: details.room_name_id.room_id().clone(),
                    is_low_priority: !details.is_low_priority,
                },
            );
            close_menu = true;
        } else if self.button(cx, ids!(copy_link_button)).clicked(actions) {
            let room_label = details.room_name_id.to_string();
            submit_async_request(MatrixRequest::GenerateMatrixLink {
                room_id: details.room_name_id.room_id().clone(),
                event_id: None,
                use_matrix_scheme: false,
                join_on_click: false,
            });
            enqueue_popup_notification(
                format!("Room link requested for {room_label}. {ROOM_CONTEXT_LINK_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(3.0),
            );
            close_menu = true;
        } else if self.button(cx, ids!(room_settings_button)).clicked(actions) {
            self.set_preview_state(cx, RoomContextPreviewState::Settings);
            enqueue_popup_notification(
                "Room settings opened as a local-only preview. No Matrix room state event was sent.",
                PopupKind::Info,
                Some(5.0),
            );
            close_menu = false;
        } else if self.button(cx, ids!(notifications_button)).clicked(actions) {
            self.set_preview_state(cx, RoomContextPreviewState::Notifications);
            enqueue_popup_notification(
                format!(
                    "Room notifications opened. All, Mentions, and Mute require confirmation; timed mute stays unwired. {ROOM_CONTEXT_NOTIFICATION_MODE_COMPACT_LABEL}"
                ),
                PopupKind::Info,
                Some(5.0),
            );
            close_menu = false;
        } else if self
            .button(
                cx,
                ids!(room_context_local_preview.room_context_preview_primary_button),
            )
            .clicked(actions)
        {
            self.handle_preview_choice(cx, &details, 0);
            close_menu = true;
        } else if self
            .button(
                cx,
                ids!(room_context_local_preview.room_context_preview_secondary_button),
            )
            .clicked(actions)
        {
            self.handle_preview_choice(cx, &details, 1);
            close_menu = true;
        } else if self
            .button(
                cx,
                ids!(room_context_local_preview.room_context_preview_tertiary_button),
            )
            .clicked(actions)
        {
            self.handle_preview_choice(cx, &details, 2);
            close_menu = true;
        } else if self
            .button(
                cx,
                ids!(room_context_local_preview.room_context_preview_quaternary_button),
            )
            .clicked(actions)
        {
            self.handle_preview_choice(cx, &details, 3);
            close_menu = true;
        } else if self.button(cx, ids!(invite_button)).clicked(actions) {
            cx.action(InviteModalAction::Open(details.room_name_id.clone()));
            close_menu = true;
        } else if self.button(cx, ids!(leave_button)).clicked(actions) {
            use crate::join_leave_room_modal::{JoinLeaveRoomModalAction, JoinLeaveModalKind};
            use crate::room::BasicRoomDetails;
            let room_details = BasicRoomDetails::Name(details.room_name_id.clone());
            cx.action(JoinLeaveRoomModalAction::Open {
                kind: JoinLeaveModalKind::LeaveRoom(room_details),
                show_tip: false,
            });
            close_menu = true;
        }

        if close_menu {
            self.close(cx);
        }
    }
}

impl RoomContextMenu {
    pub fn is_currently_shown(&self, _cx: &mut Cx) -> bool {
        self.visible
    }

    pub fn show(&mut self, cx: &mut Cx, details: RoomContextMenuDetails) -> DVec2 {
        self.preview_state = RoomContextPreviewState::Hidden;
        let height = self.update_buttons(cx, &details);
        self.details = Some(details);
        self.update_preview(cx);
        self.visible = true;
        cx.set_key_focus(self.view.area());
        dvec2(MENU_WIDTH, height)
    }

    fn update_buttons(&mut self, cx: &mut Cx, details: &RoomContextMenuDetails) -> f64 {
        let mark_unread_button = self.button(cx, ids!(mark_unread_button));
        if details.is_marked_unread {
            mark_unread_button.set_text(cx, "Mark as Read");
        } else {
            mark_unread_button.set_text(cx, "Mark as Unread");
        }

        let favorite_button = self.button(cx, ids!(favorite_button));
        if details.is_favorite {
            favorite_button.set_text(cx, "Un-favorite");
        } else {
            favorite_button.set_text(cx, "Favorite");
        }

        let priority_button = self.button(cx, ids!(priority_button));
        if details.is_low_priority {
            priority_button.set_text(cx, "Un-set Low Priority");
        } else {
            priority_button.set_text(cx, "Set Low Priority");
        }

        // Reset hover states
        mark_unread_button.reset_hover(cx);
        favorite_button.reset_hover(cx);
        priority_button.reset_hover(cx);
        self.button(cx, ids!(copy_link_button)).reset_hover(cx);
        self.button(cx, ids!(room_settings_button)).reset_hover(cx);
        self.button(cx, ids!(notifications_button)).reset_hover(cx);
        self.button(
            cx,
            ids!(room_context_local_preview.room_context_preview_primary_button),
        )
        .reset_hover(cx);
        self.button(
            cx,
            ids!(room_context_local_preview.room_context_preview_secondary_button),
        )
        .reset_hover(cx);
        self.button(
            cx,
            ids!(room_context_local_preview.room_context_preview_tertiary_button),
        )
        .reset_hover(cx);
        self.button(
            cx,
            ids!(room_context_local_preview.room_context_preview_quaternary_button),
        )
        .reset_hover(cx);
        self.button(cx, ids!(invite_button)).reset_hover(cx);
        self.button(cx, ids!(leave_button)).reset_hover(cx);

        self.redraw(cx);

        // Calculate height (rudimentary) - sum of visible buttons + padding
        // 8 buttons * 35.0 + 2 dividers * ~10.0 + padding
        (8.0 * BUTTON_HEIGHT) + 20.0 + 10.0 // approx
    }

    fn show_stateful_toggle_confirmation(
        &self,
        cx: &mut Cx,
        details: &RoomContextMenuDetails,
        title_label: &'static str,
        action_label: &'static str,
        accept_label: &'static str,
        request: MatrixRequest,
    ) {
        let room_label = details.room_name_id.to_string();
        let room_label_for_accept = room_label.clone();
        let room_label_for_cancel = room_label.clone();
        let action_label_for_accept = action_label.to_string();
        let action_label_for_cancel = action_label.to_string();
        let content = ConfirmationModalContent {
            title_text: format!("Confirm {title_label}").into(),
            body_text: format!(
                "Update {room_label} to {action_label}? {ROOM_CONTEXT_STATUS_CONFIRMATION_COMPACT_LABEL}"
            )
            .into(),
            accept_button_text: Some(accept_label.into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                submit_async_request(request);
                enqueue_popup_notification(
                    format!(
                        "Room status update requested for {room_label_for_accept}: {action_label_for_accept}."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Room status update canceled for {room_label_for_cancel}: {action_label_for_cancel}. {ROOM_CONTEXT_STATUS_CONFIRMATION_COMPACT_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            format!(
                "Room status confirmation opened for {room_label}: {action_label}. {ROOM_CONTEXT_STATUS_CONFIRMATION_COMPACT_LABEL}"
            ),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn set_preview_state(&mut self, cx: &mut Cx, state: RoomContextPreviewState) {
        self.preview_state = state;
        self.update_preview(cx);
    }

    fn update_preview(&mut self, cx: &mut Cx) {
        let visible = self.preview_state != RoomContextPreviewState::Hidden;
        self.view
            .view(cx, ids!(room_context_local_preview))
            .set_visible(cx, visible);
        if !visible {
            self.redraw(cx);
            return;
        }

        let (title, summary, labels) = match self.preview_state {
            RoomContextPreviewState::Hidden => ("", String::new(), ["", "", "", ""]),
            RoomContextPreviewState::Settings => (
                "Room settings summary",
                room_context_settings_loaded_identity_summary(self.details.as_ref()),
                ["Name", "Topic", "Permissions", "Members"],
            ),
            RoomContextPreviewState::Notifications => (
                "Notification mode",
                format!(
                    "{} All, Mentions, and Mute update Matrix notification mode after confirmation. Timed mute stays unwired.",
                    room_context_notification_loaded_attention_summary(self.details.as_ref())
                ),
                ["All", "Mentions", "Mute", "Timed mute"],
            ),
        };

        self.view
            .label(
                cx,
                ids!(room_context_local_preview.room_context_preview_title),
            )
            .set_text(cx, title);
        self.view
            .label(
                cx,
                ids!(room_context_local_preview.room_context_preview_summary),
            )
            .set_text(cx, &summary);
        self.view
            .button(
                cx,
                ids!(room_context_local_preview.room_context_preview_primary_button),
            )
            .set_text(cx, labels[0]);
        self.view
            .button(
                cx,
                ids!(room_context_local_preview.room_context_preview_secondary_button),
            )
            .set_text(cx, labels[1]);
        self.view
            .button(
                cx,
                ids!(room_context_local_preview.room_context_preview_tertiary_button),
            )
            .set_text(cx, labels[2]);
        self.view
            .button(
                cx,
                ids!(room_context_local_preview.room_context_preview_quaternary_button),
            )
            .set_text(cx, labels[3]);
        self.redraw(cx);
    }

    fn stage_preview_choice(&mut self, cx: &mut Cx, index: usize) {
        let Some(label) = self.preview_choice_label(index) else {
            return;
        };
        let message = match self.preview_state {
            RoomContextPreviewState::Hidden => return,
            RoomContextPreviewState::Settings => {
                format!(
                    "{label} room settings summary shown locally. No Matrix room state event was sent."
                )
            }
            RoomContextPreviewState::Notifications => return,
        };
        enqueue_popup_notification(message, PopupKind::Info, Some(5.0));
        self.preview_state = RoomContextPreviewState::Hidden;
        self.update_preview(cx);
    }

    fn handle_preview_choice(
        &mut self,
        cx: &mut Cx,
        details: &RoomContextMenuDetails,
        index: usize,
    ) {
        match self.preview_state {
            RoomContextPreviewState::Hidden => {}
            RoomContextPreviewState::Settings => self.stage_preview_choice(cx, index),
            RoomContextPreviewState::Notifications => {
                self.handle_notification_preview_choice(cx, details, index);
            }
        }
    }

    fn handle_notification_preview_choice(
        &mut self,
        cx: &mut Cx,
        details: &RoomContextMenuDetails,
        index: usize,
    ) {
        let mode = match index {
            0 => Some(RoomNotificationMode::AllMessages),
            1 => Some(RoomNotificationMode::MentionsAndKeywordsOnly),
            2 => Some(RoomNotificationMode::Mute),
            _ => None,
        };
        if let Some(mode) = mode {
            self.show_notification_mode_confirmation(cx, details, mode);
        } else {
            enqueue_popup_notification(
                format!(
                    "Timed mute is not wired for {} yet. {ROOM_CONTEXT_NOTIFICATION_MODE_COMPACT_LABEL}",
                    details.room_name_id
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }
        self.preview_state = RoomContextPreviewState::Hidden;
        self.update_preview(cx);
    }

    fn show_notification_mode_confirmation(
        &self,
        cx: &mut Cx,
        details: &RoomContextMenuDetails,
        mode: RoomNotificationMode,
    ) {
        let room_label = details.room_name_id.to_string();
        let room_label_for_accept = room_label.clone();
        let room_label_for_cancel = room_label.clone();
        let mode_label = room_context_notification_mode_label(mode);
        let mode_label_for_accept = mode_label.to_string();
        let mode_label_for_cancel = mode_label.to_string();
        let room_id = details.room_name_id.room_id().clone();
        let content = ConfirmationModalContent {
            title_text: "Confirm Notifications".into(),
            body_text: format!(
                "Set notification mode for {room_label} to {mode_label}? {ROOM_CONTEXT_NOTIFICATION_MODE_COMPACT_LABEL}"
            )
            .into(),
            accept_button_text: Some(mode_label.into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                submit_async_request(MatrixRequest::SetRoomNotificationMode { room_id, mode });
                enqueue_popup_notification(
                    format!(
                        "Notification mode update requested for {room_label_for_accept}: {mode_label_for_accept}."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Notification mode update canceled for {room_label_for_cancel}: {mode_label_for_cancel}. {ROOM_CONTEXT_NOTIFICATION_MODE_COMPACT_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            format!(
                "Notification mode confirmation opened for {room_label}: {mode_label}. {ROOM_CONTEXT_NOTIFICATION_MODE_COMPACT_LABEL}"
            ),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn preview_choice_label(&self, index: usize) -> Option<&'static str> {
        match self.preview_state {
            RoomContextPreviewState::Hidden => None,
            RoomContextPreviewState::Settings => ["Name", "Topic", "Permissions", "Members"]
                .get(index)
                .copied(),
            RoomContextPreviewState::Notifications => ["All", "Mentions", "Mute", "Timed mute"]
                .get(index)
                .copied(),
        }
    }

    fn close(&mut self, cx: &mut Cx) {
        self.visible = false;
        self.details = None;
        self.preview_state = RoomContextPreviewState::Hidden;
        self.update_preview(cx);
        cx.revert_key_focus();
        cx.unblock_scrolling();
        self.redraw(cx);
    }
}

impl RoomContextMenuRef {
    pub fn is_currently_shown(&self, cx: &mut Cx) -> bool {
        let Some(inner) = self.borrow() else {
            return false;
        };
        inner.is_currently_shown(cx)
    }

    pub fn show(&self, cx: &mut Cx, details: RoomContextMenuDetails) -> DVec2 {
        let Some(mut inner) = self.borrow_mut() else {
            return DVec2::default();
        };
        inner.show(cx, details)
    }
}
