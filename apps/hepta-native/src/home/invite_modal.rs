//! A modal dialog for inviting a user to a room.

use std::cell::RefCell;

use makepad_widgets::*;
use ruma::OwnedUserId;

use crate::{
    home::room_screen::{InviteAction, InviteResultAction},
    shared::{
        confirmation_modal::ConfirmationModalContent,
        popup_list::{PopupKind, enqueue_popup_notification},
    },
    sliding_sync::{MatrixRequest, submit_async_request},
    utils::RoomNameId,
};

pub const INVITE_MODAL_CONFIRMATION_COMPACT_LABEL: &str = "Invite is sent only after confirmation.";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.InviteModal = #(InviteModal::register_widget(vm)) {
        width: Fit
        height: Fit

        RoundedView {
            width: 400
            height: Fit
            align: Align{x: 0.5}
            flow: Down
            padding: Inset{top: 30, right: 25, bottom: 20, left: 25}

            show_bg: true
            draw_bg +: {
                color: (COLOR_PRIMARY)
                border_radius: 4.0
            }

            title_view := View {
                width: Fill,
                height: Fit,
                padding: Inset{top: 0, bottom: 25}
                align: Align{x: 0.5, y: 0.0}

                title := Label {
                    width: Fill
                    height: Fit
                    align: Align{x: 0.5}
                    flow: Flow.Right{wrap: true},
                    draw_text +: {
                        text_style: TITLE_TEXT {font_size: 13},
                        color: #000
                    }
                    text: "Invite to Room"
                }
            }

            user_id_input := RobrixTextInput {
                draw_text +: {
                    text_style: REGULAR_TEXT {font_size: 11},
                    color: #000
                }
                empty_text: "@user:example.org",
            }

            View {
                width: Fill, height: Fit
                flow: Right,
                padding: Inset{top: 20, bottom: 10}
                align: Align{x: 1.0, y: 0.5}
                spacing: 20

                cancel_button := RobrixNeutralIconButton {
                    width: 120,
                    align: Align{x: 0.5, y: 0.5}
                    padding: 12,
                    draw_icon.svg: (ICON_FORBIDDEN)
                    icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                    text: "Cancel"
                }

                confirm_button := RobrixPositiveIconButton {
                    width: 120
                    align: Align{x: 0.5, y: 0.5}
                    padding: 12,
                    draw_icon.svg: (ICON_ADD_USER)
                    icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                    text: "Invite"
                }

                okay_button := RobrixIconButton {
                    visible: false
                    width: 120
                    align: Align{x: 0.5, y: 0.5}
                    padding: 12,
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                    text: "Okay"
                }
            }

            status_label_view := View {
                visible: false
                width: Fill,
                height: Fit,
                align: Align{x: 0.5, y: 0.0}

                status_label := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true},
                    align: Align{x: 0.5, y: 0.0}
                    margin: Inset{top: 10}
                    draw_text +: {
                        text_style: REGULAR_TEXT {font_size: 11},
                        color: #000
                    }
                    text: ""
                }
            }
        }
    }
}

/// Actions emitted by other widgets to show or hide the `InviteModal`.
#[derive(Clone, Debug)]
pub enum InviteModalAction {
    /// Open the modal to invite a user to the given room or space.
    Open(RoomNameId),
    /// Close the modal.
    Close,
    /// Start the existing invite request path after the user has confirmed.
    InviteConfirmed {
        room_name_id: RoomNameId,
        user_id: OwnedUserId,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum InviteModalState {
    /// Waiting for the user to enter a user ID.
    #[default]
    WaitingForUserInput,
    /// Waiting for the invite to be sent.
    WaitingForInvite(OwnedUserId),
    /// The invite was sent successfully.
    InviteSuccess,
    /// An error occurred while sending the invite.
    InviteError,
}

#[derive(Script, ScriptHook, Widget)]
pub struct InviteModal {
    #[deref]
    view: View,
    #[rust]
    state: InviteModalState,
    #[rust]
    room_name_id: Option<RoomNameId>,
}

impl Widget for InviteModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for InviteModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let cancel_button = self.view.button(cx, ids!(cancel_button));

        // Handle canceling/closing the modal.
        let cancel_clicked = cancel_button.clicked(actions);
        if cancel_clicked
            || actions
                .iter()
                .any(|a| matches!(a.downcast_ref(), Some(ModalAction::Dismissed)))
        {
            // If the modal was dismissed by clicking outside of it, we MUST NOT emit
            // a `InviteModalAction::Close` action, as that would cause
            // an infinite action feedback loop.
            if cancel_clicked {
                cx.action(InviteModalAction::Close);
            }
            return;
        }

        // Handle the okay button (shown after invite success).
        let okay_button = self.view.button(cx, ids!(okay_button));
        if okay_button.clicked(actions) {
            cx.action(InviteModalAction::Close);
            return;
        }

        let confirm_button = self.view.button(cx, ids!(confirm_button));
        let user_id_input = self.view.text_input(cx, ids!(user_id_input));
        let status_view = self.view.view(cx, ids!(status_label_view));
        let mut status_label = self.view.label(cx, ids!(status_label_view.status_label));

        for action in actions {
            if let Some(InviteModalAction::InviteConfirmed {
                room_name_id,
                user_id,
            }) = action.downcast_ref()
            {
                let room_matches = self
                    .room_name_id
                    .as_ref()
                    .is_some_and(|current| current.room_id() == room_name_id.room_id());
                if room_matches {
                    submit_async_request(MatrixRequest::InviteUser {
                        room_id: room_name_id.room_id().clone(),
                        user_id: user_id.clone(),
                    });
                    self.state = InviteModalState::WaitingForInvite(user_id.clone());
                    let status = format!("Sending invite to {user_id}...");
                    script_apply_eval!(cx, status_label, {
                        text: #(status),
                        draw_text +: {
                            color: mod.widgets.COLOR_ACTIVE_PRIMARY_DARKER,
                        },
                    });
                    status_view.set_visible(cx, true);
                    confirm_button.set_enabled(cx, false);
                    user_id_input.set_is_read_only(cx, true);
                    enqueue_popup_notification(
                        format!(
                            "Invite confirmed for {user_id}. Existing InviteUser path requested."
                        ),
                        PopupKind::Info,
                        Some(3.0),
                    );
                    self.view.redraw(cx);
                    break;
                }
            }
        }

        // Handle return key or invite button click.
        if let Some(user_id_str) = confirm_button
            .clicked(actions)
            .then(|| user_id_input.text())
            .or_else(|| user_id_input.returned(actions).map(|(t, _)| t))
        {
            // Validate the user ID
            if user_id_str.is_empty() {
                script_apply_eval!(cx, status_label, {
                    text: "Please enter a user ID.",
                    draw_text +: {
                        color: mod.widgets.COLOR_FG_DANGER_RED,
                    },
                });
                status_view.set_visible(cx, true);
                self.view.redraw(cx);
                return;
            }

            // Try to parse the user ID
            match ruma::UserId::parse(&user_id_str) {
                Ok(user_id) => {
                    if let Some(room_name_id) = &self.room_name_id {
                        let room_name_id_for_action = room_name_id.clone();
                        let room_label = room_name_id.to_string();
                        let user_id_for_action = user_id.to_owned();
                        let user_id_for_cancel = user_id.to_owned();
                        let user_id_for_status = user_id.to_owned();
                        let content = ConfirmationModalContent {
                            title_text: "Send Invitation".into(),
                            body_text: format!(
                                "Invite {user_id} to {room_label}? {INVITE_MODAL_CONFIRMATION_COMPACT_LABEL}"
                            )
                            .into(),
                            accept_button_text: Some("Invite".into()),
                            cancel_button_text: Some("Cancel".into()),
                            on_accept_clicked: Some(Box::new(move |cx| {
                                cx.action(InviteModalAction::InviteConfirmed {
                                    room_name_id: room_name_id_for_action.clone(),
                                    user_id: user_id_for_action.clone(),
                                });
                            })),
                            on_cancel_clicked: Some(Box::new(move |_cx| {
                                enqueue_popup_notification(
                                    format!(
                                        "Invite canceled for {user_id_for_cancel}. {INVITE_MODAL_CONFIRMATION_COMPACT_LABEL}"
                                    ),
                                    PopupKind::Info,
                                    Some(3.0),
                                );
                            })),
                        };
                        enqueue_popup_notification(
                            format!(
                                "Invite confirmation opened for {user_id_for_status}. {INVITE_MODAL_CONFIRMATION_COMPACT_LABEL}"
                            ),
                            PopupKind::Info,
                            Some(3.0),
                        );
                        cx.action(InviteAction::ShowInviteConfirmationModal(RefCell::new(
                            Some(content),
                        )));
                        script_apply_eval!(cx, status_label, {
                            text: "Invite confirmation opened; request is still pending confirmation.",
                            draw_text +: {
                                color: mod.widgets.COLOR_ACTIVE_PRIMARY_DARKER,
                            },
                        });
                        status_view.set_visible(cx, true);
                    }
                }
                Err(_) => {
                    script_apply_eval!(cx, status_label, {
                        text: "Invalid User ID. Expected format: @user:server.xyz",
                        draw_text +: {
                            color: mod.widgets.COLOR_FG_DANGER_RED,
                        },
                    });
                    status_view.set_visible(cx, true);
                    user_id_input.set_key_focus(cx);
                }
            }
            self.view.redraw(cx);
        }

        // Handle the result of a previously-sent invite.
        if let InviteModalState::WaitingForInvite(invited_user_id) = &self.state {
            for action in actions {
                let new_state = match action.downcast_ref() {
                    Some(InviteResultAction::Sent { room_id, user_id })
                        if self
                            .room_name_id
                            .as_ref()
                            .is_some_and(|rni| rni.room_id() == room_id)
                            && invited_user_id == user_id =>
                    {
                        let status = format!("Successfully invited {user_id}!");
                        script_apply_eval!(cx, status_label, {
                            text: #(status),
                            draw_text +: {
                                color: mod.widgets.COLOR_FG_ACCEPT_GREEN
                            }
                        });
                        status_view.set_visible(cx, true);
                        confirm_button.set_visible(cx, false);
                        cancel_button.set_visible(cx, false);
                        okay_button.set_visible(cx, true);
                        Some(InviteModalState::InviteSuccess)
                    }
                    Some(InviteResultAction::Failed {
                        room_id,
                        user_id,
                        error,
                    }) if self
                        .room_name_id
                        .as_ref()
                        .is_some_and(|rni| rni.room_id() == room_id)
                        && invited_user_id == user_id =>
                    {
                        let status = format!("Failed to send invite: {error}");
                        script_apply_eval!(cx, status_label, {
                            text: #(status),
                            draw_text +: {
                                color: mod.widgets.COLOR_FG_DANGER_RED,
                            }
                        });
                        status_view.set_visible(cx, true);
                        confirm_button.set_enabled(cx, true);
                        user_id_input.set_is_read_only(cx, false);
                        user_id_input.set_key_focus(cx);
                        Some(InviteModalState::InviteError)
                    }
                    _ => None,
                };
                if let Some(new_state) = new_state {
                    self.state = new_state;
                    self.view.redraw(cx);
                    break;
                }
            }
        }
    }
}

impl InviteModal {
    pub fn show(&mut self, cx: &mut Cx, room_name_id: RoomNameId) {
        self.view
            .label(cx, ids!(title))
            .set_text(cx, &format!("Invite to {room_name_id}"));
        self.state = InviteModalState::WaitingForUserInput;
        self.room_name_id = Some(room_name_id);

        // Reset the UI state
        let confirm_button = self.view.button(cx, ids!(confirm_button));
        let cancel_button = self.view.button(cx, ids!(cancel_button));
        let okay_button = self.view.button(cx, ids!(okay_button));
        let user_id_input = self.view.text_input(cx, ids!(user_id_input));
        confirm_button.set_visible(cx, true);
        confirm_button.set_enabled(cx, true);
        confirm_button.reset_hover(cx);
        cancel_button.set_visible(cx, true);
        cancel_button.set_enabled(cx, true);
        cancel_button.reset_hover(cx);
        okay_button.set_visible(cx, false);
        okay_button.reset_hover(cx);
        user_id_input.set_is_read_only(cx, false);
        user_id_input.set_text(cx, "");
        self.view
            .view(cx, ids!(status_label_view))
            .set_visible(cx, false);
        self.view
            .label(cx, ids!(status_label_view.status_label))
            .set_text(cx, "");
        self.view.redraw(cx);
        user_id_input.set_key_focus(cx);
    }
}

impl InviteModalRef {
    pub fn show(&self, cx: &mut Cx, room_name_id: RoomNameId) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.show(cx, room_name_id);
    }
}
