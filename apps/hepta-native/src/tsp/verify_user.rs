use makepad_widgets::*;
use matrix_sdk::ruma::OwnedUserId;

use crate::{
    shared::popup_list::{PopupKind, enqueue_popup_notification},
    tsp::{TspIdentityAction, TspRequest, submit_tsp_request, tsp_state_ref},
};

pub const TSP_VERIFY_LOCAL_COMPACT_LABEL: &str = "Local TSP boundary; cancel/remove is not wired.";
pub const TSP_ASSOCIATION_BLOCKED_METADATA_EVIDENCE: &str = "TspVerifyUser now shows local blocked metadata for TSP association cancel/remove surfaces from loaded target user id, entered DID/read-only DID availability, and current local association state. Initiator cancel and Remove TSP Association remain blocked/local-only and start no CancelAssociateDidRequest, VerificationCancel, TspRequest cancel, TSP state update, wallet database write, filesystem write, Matrix request, gateway/runtime/auth, or live mutation.";
pub const TSP_ASSOCIATION_CANCEL_REMOVE_PACKET_EVIDENCE: &str = "TspVerifyUser now shows a local association cancel/remove packet for initiator cancel and Remove TSP Association. The packet records missing backend request id, non-secret target/DID availability, disabled cancel/remove state, persistence scope, receive-loop scope, and stale-result policy; it starts no CancelAssociateDidRequest, VerificationCancel, TspRequest cancel, TSP state update, wallet database write, filesystem write, Matrix request, gateway/runtime/auth, or live mutation.";
pub const TSP_ASSOCIATION_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "TspVerifyUser association result taxonomy packet records local-only cancel not sent, remote cancel not sent, already answered local state, failed cancel not started, stale request blocked, remove not started, persistence result not started, receive-loop result not started, responder notification not sent, retry blocked until backend request id, and target/DID presence-only audit redaction.";

fn local_value_label(value: Option<&str>, fallback: &str) -> String {
    let value = value.unwrap_or("").trim();
    if value.is_empty() {
        fallback.to_string()
    } else {
        value.to_string()
    }
}

fn tsp_association_blocked_metadata_label(
    target_user_id: Option<&str>,
    did: Option<&str>,
    state: &str,
    action: &str,
) -> String {
    let target_user_id = local_value_label(target_user_id, "unknown target");
    let did_state = if did.unwrap_or("").trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let state = local_value_label(Some(state), "idle");
    let action = local_value_label(Some(action), "local blocked control");
    format!(
        "TSP association {action}: target {target_user_id}; DID {did_state}; state {state}. No CancelAssociateDidRequest, VerificationCancel, TspRequest cancel, TSP state update, wallet database write, filesystem write, Matrix request, gateway/runtime/auth, or live mutation starts."
    )
}

fn tsp_association_cancel_remove_packet_label(
    target_user_id: Option<&str>,
    did: Option<&str>,
    state: &str,
    action: &str,
) -> String {
    let target_state = if target_user_id.unwrap_or("").trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let did_state = if did.unwrap_or("").trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let action = local_value_label(Some(action), "local blocked control");
    let base = tsp_association_blocked_metadata_label(target_user_id, did, state, &action);
    format!(
        "TSP association {action} packet: request_id missing_backend_contract; local_association_key target_state:{target_state} did_state:{did_state}; cancel_state disabled_no_request; persistence_scope backend_required; receive_loop_scope backend_required; stale_result_policy backend_request_id_required; result_taxonomy local_only_cancel_not_sent|remote_cancel_not_sent|already_answered_local_state|failed_cancel_not_started|stale_request_blocked|remove_not_started; persistence_result_slot not_started; receive_loop_result_slot not_started; responder_notification_slot not_sent; retry_policy blocked_until_backend_request_id; audit_redaction target_did_presence_only. {base} {TSP_ASSOCIATION_CANCEL_REMOVE_PACKET_EVIDENCE} {TSP_ASSOCIATION_RESULT_TAXONOMY_PACKET_EVIDENCE}"
    )
}

script_mod! {
    link tsp_enabled

    use mod.prelude.widgets.*
    use mod.widgets.*


    // A view that allows the user to verify a new DID and associate it
    // with a particular Matrix User ID.
    // This is currently shown as part of the UserProfileSlidingPane.
    mod.widgets.TspVerifyUser = #(TspVerifyUser::register_widget(vm)) {
        width: Fill, height: Fit
        flow: Down
        spacing: 20,

        LineH { padding: 15 }

        View {
            width: Fill, height: Fit
            flow: Down
            spacing: 10
            padding: Inset{ left: 10, right: 10, bottom: 10}

            Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    text_style: USERNAME_TEXT_STYLE { font_size: 11.5 },
                    color: #000
                }
                text: "TSP User Verification"
            }

            // Content shown when this user has been verified via TSP.
            verified_tsp := View {
                visible: false,
                width: Fill, height: Fit
                flow: Down,
                spacing: 10,
                // margin: Inset{ left: 7 }

                Label {
                    width: Fill, height: Fit
                    flow: Flow.Right{wrap: true}
                    draw_text +: {
                        color: (COLOR_FG_ACCEPT_GREEN),
                        text_style: MESSAGE_TEXT_STYLE { font_size: 11 },
                    }
                    text: "✅ Verified via TSP"
                }

                tsp_did_read_only_input := RobrixTextInput {
                    is_read_only: true
                }

                remove_tsp_association_button := RobrixNegativeIconButton {
                    padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                    draw_icon.svg: (ICON_CLOSE)
                    icon_walk: Walk{width: 22, height: 16, margin: Inset{left: -5, right: -3, top: 1, bottom: -1} }
                    text: "Remove TSP Association"
                }

                remove_association_evidence := Label {
                    width: Fill, height: Fit
                    flow: Flow.Right{wrap: true}
                    draw_text +: {
                        color: (MESSAGE_TEXT_COLOR),
                        text_style: MESSAGE_TEXT_STYLE { font_size: 10 },
                    }
                    text: "Local TSP boundary; cancel/remove is not wired."
                }
            }


            // Content shown when this user has NOT been verified via TSP.
            unverified_tsp := View {
                visible: true,
                width: Fill, height: Fit
                flow: Down,
                spacing: 10,
                // margin: Inset{ left: 7 }

                Label {
                    width: Fill, height: Fit
                    flow: Flow.Right{wrap: true},
                    draw_text +: {
                        color: (MESSAGE_TEXT_COLOR),
                        text_style: MESSAGE_TEXT_STYLE { font_size: 11 },
                    }
                    text: "Interactively verify this user by associating their TSP identity (DID) with their Matrix User ID:"
                }

                tsp_did_input := RobrixTextInput {
                    empty_text: "Enter their TSP DID..."
                }

                verify_user_button := RobrixPositiveIconButton {
                    padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 22, height: 16, margin: Inset{left: -5, right: -3, top: 1, bottom: -1} }
                    text: "Verify this user via TSP"
                }

                association_cancel_evidence := Label {
                    width: Fill, height: Fit
                    flow: Flow.Right{wrap: true}
                    draw_text +: {
                        color: (MESSAGE_TEXT_COLOR),
                        text_style: MESSAGE_TEXT_STYLE { font_size: 10 },
                    }
                    text: "Verify submits association request; initiator cancel is not wired."
                }
            }
        }
    }
}

/// Whether another user has been verified using TSP.
#[derive(Default)]
pub enum TspVerifiedInfo {
    #[default]
    Unverified,
    Verified {
        did: String,
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct TspVerifyUser {
    #[deref]
    view: View,
    /// The Matrix User ID of the other user that we want to verify.
    #[rust]
    user_id: Option<OwnedUserId>,
    /// Info about whether the other user has or has not been verified via TSP.
    #[rust]
    verified_info: TspVerifiedInfo,
}

impl Widget for TspVerifyUser {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl MatchEvent for TspVerifyUser {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .view
            .button(cx, ids!(remove_tsp_association_button))
            .clicked(actions)
        {
            let did = match &self.verified_info {
                TspVerifiedInfo::Verified { did } => Some(did.as_str()),
                TspVerifiedInfo::Unverified => None,
            };
            let label = tsp_association_cancel_remove_packet_label(
                self.user_id.as_ref().map(|user_id| user_id.as_str()),
                did,
                "remove blocked",
                "remove",
            );
            self.view
                .label(cx, ids!(remove_association_evidence))
                .set_text(cx, &label);
            enqueue_popup_notification(
                "Remove TSP Association is not implemented. No TSP mutation started.",
                PopupKind::Warning,
                Some(5.0),
            );
        }

        let verify_user_button = self.view.button(cx, ids!(verify_user_button));
        if verify_user_button.clicked(actions) {
            let did_input = self.view.view(cx, ids!(tsp_did_input));
            let did = did_input.text().trim().to_string();
            log!("verify_user_button was clicked. DID: {}", did);
            if did.is_empty() {
                enqueue_popup_notification(
                    "Please enter a valid TSP DID to verify this user.",
                    PopupKind::Error,
                    Some(5.0),
                );
            } else if let Some(user_id) = self.user_id.clone() {
                let label = tsp_association_cancel_remove_packet_label(
                    Some(user_id.as_str()),
                    Some(did.as_str()),
                    "pending",
                    "initiator cancel",
                );
                submit_tsp_request(TspRequest::AssociateDidWithUserId { did, user_id });
                verify_user_button.set_enabled(cx, false);
                verify_user_button.set_text(cx, "Sending request...");
                self.view
                    .label(cx, ids!(association_cancel_evidence))
                    .set_text(cx, &label);
            }
        }

        for action in actions {
            match action.downcast_ref() {
                Some(TspIdentityAction::SentDidAssociationRequest { user_id, .. })
                    if Some(user_id) == self.user_id.as_ref() =>
                {
                    verify_user_button.set_text(cx, "Sent request!");
                    enqueue_popup_notification(
                        format!(
                            "Sent TSP verification request.\n\nWaiting for \"{user_id}\" to respond..."
                        ),
                        PopupKind::Info,
                        Some(5.0),
                    );
                }
                Some(TspIdentityAction::ErrorSendingDidAssociationRequest {
                    user_id,
                    error,
                    ..
                }) if Some(user_id) == self.user_id.as_ref() => {
                    verify_user_button.set_enabled(cx, true);
                    verify_user_button.set_text(cx, "Verify this user via TSP");
                    let label = tsp_association_cancel_remove_packet_label(
                        Some(user_id.as_str()),
                        None,
                        "failed",
                        "initiator cancel",
                    );
                    self.view
                        .label(cx, ids!(association_cancel_evidence))
                        .set_text(cx, &label);
                    enqueue_popup_notification(
                        format!("Error sending TSP verification request to \"{user_id}\": {error}"),
                        PopupKind::Error,
                        None,
                    );
                }
                Some(TspIdentityAction::ReceivedDidAssociationResponse {
                    did,
                    user_id,
                    accepted,
                }) if Some(user_id) == self.user_id.as_ref() => {
                    if *accepted {
                        enqueue_popup_notification(
                            format!("User \"{user_id}\" accepted your TSP verification request."),
                            PopupKind::Success,
                            None,
                        );
                        self.verified_info = TspVerifiedInfo::Verified { did: did.clone() };
                    } else {
                        enqueue_popup_notification(
                            format!("User \"{user_id}\" rejected your TSP verification request."),
                            PopupKind::Warning,
                            None,
                        );
                    }
                    // Repopulate the content of this widget.
                    self.refresh_from_verified_info(cx);
                    self.redraw(cx);
                }
                _ => {}
            }
        }
    }
}

impl TspVerifyUser {
    /// Repopulates this widget's UI content from its inner verified info.
    fn refresh_from_verified_info(&mut self, cx: &mut Cx) {
        let verified_tsp_view = self.view.view(cx, ids!(verified_tsp));
        let unverified_tsp_view = self.view.view(cx, ids!(unverified_tsp));
        match &self.verified_info {
            TspVerifiedInfo::Verified { did } => {
                verified_tsp_view.set_visible(cx, true);
                unverified_tsp_view.set_visible(cx, false);
                verified_tsp_view
                    .text_input(cx, ids!(tsp_did_read_only_input))
                    .set_text(cx, did);
                let label = tsp_association_cancel_remove_packet_label(
                    self.user_id.as_ref().map(|user_id| user_id.as_str()),
                    Some(did.as_str()),
                    "verified",
                    "remove",
                );
                self.view
                    .label(cx, ids!(remove_association_evidence))
                    .set_text(cx, &label);
            }
            TspVerifiedInfo::Unverified => {
                verified_tsp_view.set_visible(cx, false);
                unverified_tsp_view.set_visible(cx, true);
                unverified_tsp_view
                    .text_input(cx, ids!(tsp_did_input))
                    .set_text(cx, "");
                let verify_user_button = unverified_tsp_view.button(cx, ids!(verify_user_button));
                verify_user_button.set_enabled(cx, true);
                verify_user_button.set_text(cx, "Verify this user via TSP");
                let label = tsp_association_cancel_remove_packet_label(
                    self.user_id.as_ref().map(|user_id| user_id.as_str()),
                    None,
                    "idle",
                    "initiator cancel",
                );
                self.view
                    .label(cx, ids!(association_cancel_evidence))
                    .set_text(cx, &label);
            }
        }
    }

    fn show(&mut self, cx: &mut Cx, user_id: OwnedUserId) {
        let verified_info = tsp_state_ref()
            .lock()
            .unwrap()
            .get_associated_did(&user_id)
            .map_or(TspVerifiedInfo::Unverified, |did| {
                TspVerifiedInfo::Verified {
                    did: did.to_string(),
                }
            });

        self.verified_info = verified_info;
        self.user_id = Some(user_id);
        self.refresh_from_verified_info(cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tsp_association_blocked_metadata_summarizes_verified_remove() {
        let label = tsp_association_blocked_metadata_label(
            Some("@alice:example.org"),
            Some("did:tsp:alice"),
            "verified",
            "remove",
        );

        assert!(label.contains("target @alice:example.org"));
        assert!(label.contains("DID loaded"));
        assert!(label.contains("state verified"));
        assert!(label.contains("CancelAssociateDidRequest"));
        assert!(label.contains("wallet database write"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_association_blocked_metadata_uses_safe_fallbacks() {
        let label = tsp_association_blocked_metadata_label(None, None, " ", " ");

        assert!(label.contains("target unknown target"));
        assert!(label.contains("DID missing"));
        assert!(label.contains("state idle"));
        assert!(label.contains("local blocked control"));
        assert!(label.contains("TspRequest cancel"));
    }

    #[test]
    fn tsp_association_cancel_remove_packet_records_backend_gaps() {
        let label = tsp_association_cancel_remove_packet_label(
            Some("@alice:example.org"),
            Some("did:tsp:alice"),
            "verified",
            "remove",
        );

        assert!(label.contains("request_id missing_backend_contract"));
        assert!(label.contains("target_state:loaded"));
        assert!(label.contains("did_state:loaded"));
        assert!(label.contains("cancel_state disabled_no_request"));
        assert!(label.contains("persistence_scope backend_required"));
        assert!(label.contains("receive_loop_scope backend_required"));
        assert!(label.contains("stale_result_policy backend_request_id_required"));
        assert!(label.contains("result_taxonomy local_only_cancel_not_sent"));
        assert!(label.contains("remote_cancel_not_sent"));
        assert!(label.contains("already_answered_local_state"));
        assert!(label.contains("failed_cancel_not_started"));
        assert!(label.contains("stale_request_blocked"));
        assert!(label.contains("remove_not_started"));
        assert!(label.contains("persistence_result_slot not_started"));
        assert!(label.contains("receive_loop_result_slot not_started"));
        assert!(label.contains("responder_notification_slot not_sent"));
        assert!(label.contains("retry_policy blocked_until_backend_request_id"));
        assert!(label.contains("audit_redaction target_did_presence_only"));
        assert!(label.contains("No CancelAssociateDidRequest"));
        assert!(label.contains("TspRequest cancel"));
        assert!(label.contains("live mutation"));
        assert!(
            TSP_ASSOCIATION_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("responder notification not sent")
        );
    }
}

impl TspVerifyUserRef {
    pub fn show(&self, cx: &mut Cx, user_id: OwnedUserId) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.show(cx, user_id);
    }
}
