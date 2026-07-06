//! A modal dialog for creating a new TSP Decentralized Identity (DID).

use makepad_widgets::*;

use crate::tsp;

const TSP_DID_PENDING_CANCEL_COMPACT_LABEL: &str =
    "Local cancel before submit; pending cancel is not wired.";
pub const TSP_DID_PENDING_CANCEL_OPERATION_PACKET_EVIDENCE: &str = "CreateDidModal now shows a local pending-cancel operation packet while DID creation/publication is in flight. The packet records a non-secret local operation key, the missing backend operation id, disabled cancel state, stale-result policy, server fields, and alias availability; it starts no TspRequest cancel, DID rollback, wallet database write beyond the already-submitted create request, filesystem write, Matrix request, gateway/runtime/auth, or live mutation.";

fn tsp_did_pending_cancel_operation_packet_label(
    username: &str,
    alias: Option<&str>,
    server: &str,
    did_server: &str,
) -> String {
    let username = username.trim();
    let username_state = if username.is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let alias_state = alias
        .map(str::trim)
        .filter(|alias| !alias.is_empty())
        .map(|_| "loaded")
        .unwrap_or("missing");
    let server_state = if server.trim().is_empty() {
        "default"
    } else {
        "explicit"
    };
    let did_server_state = if did_server.trim().is_empty() {
        "default"
    } else {
        "explicit"
    };
    format!(
        "DID creation pending-cancel packet: operation_id missing_backend_contract; local_operation_key username_state:{username_state} username_chars:{} alias_state:{alias_state} server_state:{server_state} did_server_state:{did_server_state}; cancel_state disabled_no_request; stale_result_policy backend_operation_id_required; secret_redacted true. No TspRequest cancel, DID rollback, wallet database write, filesystem write, Matrix request, gateway/runtime/auth, or live mutation starts. {TSP_DID_PENDING_CANCEL_OPERATION_PACKET_EVIDENCE}",
        username.chars().count()
    )
}

script_mod! {
    link tsp_enabled

    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.CreateDidModal = #(CreateDidModal::register_widget(vm)) {
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
                    flow: Flow.Right{wrap: true},
                    draw_text +: {
                        text_style: TITLE_TEXT {font_size: 13},
                        color: #000
                    }
                    text: "Create New Identity (DID)"
                }
            }

            RoundedView {
                width: 350,
                height: Fit,
                spacing: 15,
                padding: 15,
                align: Align{x: 0.5}
                flow: Down,

                show_bg: true
                draw_bg +: {
                    color: (COLOR_SECONDARY)
                    border_radius: 4.0
                }

                username_input := RobrixTextInput {
                    width: Fill,
                    height: Fit,
                    padding: 10,
                    draw_text +: {
                        text_style: REGULAR_TEXT {font_size: 12},
                        color: #000
                    }
                    empty_text: "Identity Username",
                }

                alias_input := RobrixTextInput {
                    width: Fill,
                    height: Fit,
                    padding: 10,
                    draw_text +: {
                        text_style: REGULAR_TEXT {font_size: 12},
                        color: #000
                    }
                    empty_text: "Enter an alias (optional)",
                }

                did_type_radio_buttons := View {
                    spacing: 20,
                    width: Fit, height: Fit,
                    did_web := RadioButtonFlat {
                        text: "Web"
                        draw_text +: { color: (COLOR_TEXT) }
                        animator: { active: { default: on } }
                    }
                    did_webvh := RadioButtonFlat {
                        text: "WebVH"
                        draw_text +: { color: (COLOR_TEXT) }
                        animator: { disabled: { default: on } }
                    }
                    did_peer := RadioButtonFlat {
                        text: "Peer",
                        draw_text +: { color: (COLOR_TEXT) }
                        animator: { disabled: { default: on } }
                    }
                }

                View {
                    width: Fill, height: Fit
                    flow: Down

                    server_input := RobrixTextInput {
                        width: Fill, height: Fit,
                        flow: Right, // do not wrap
                        padding: Inset { left: 10, right: 10, top: 5, bottom: 5 }
                        empty_text: "p.teaspoon.world",
                        draw_text +: {
                            text_style: REGULAR_TEXT {font_size: 10.0}
                        }
                    }

                    View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        padding: Inset{top: 5, left: 2, right: 2, bottom: 2}
                        spacing: 0.0,
                        align: Align{x: 0.5, y: 0.5} // center horizontally and vertically

                        left_line := LineH {
                            draw_bg.color: #C8C8C8
                        }

                        Label {
                            width: Fit, height: Fit
                            padding:  0
                            draw_text +: {
                                color: #777777
                                text_style: REGULAR_TEXT {font_size: 9}
                            }
                            text: "Intermediary server domain"
                        }

                        right_line := LineH {
                            draw_bg.color: #C8C8C8
                        }
                    }
                }

                View {
                    width: Fill, height: Fit
                    flow: Down

                    did_server_input := RobrixTextInput {
                        width: Fill, height: Fit,
                        flow: Right, // do not wrap
                        padding: Inset { left: 10, right: 10, top: 5, bottom: 5 }
                        empty_text: "did.teaspoon.world",
                        draw_text +: {
                            text_style: REGULAR_TEXT {font_size: 10.0}
                        }
                    }

                    View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        padding: Inset{top: 5, left: 2, right: 2, bottom: 2}
                        spacing: 0.0,
                        align: Align{x: 0.5, y: 0.5} // center horizontally and vertically

                        left_line := LineH {
                            draw_bg.color: #C8C8C8
                        }

                        Label {
                            width: Fit, height: Fit
                            padding: 0
                            draw_text +: {
                                color: #777777
                                text_style: REGULAR_TEXT {font_size: 9}
                            }
                            text: "DID server domain"
                        }

                        right_line := LineH {
                            draw_bg.color: #C8C8C8
                        }
                    }
                }
            }

            View {
                width: Fill, height: Fit
                flow: Right,
                padding: Inset{top: 20, bottom: 20}
                align: Align{x: 1.0, y: 0.5}
                spacing: 20

                cancel_button := RobrixNegativeIconButton {
                    width: 100,
                    align: Align{x: 0.5, y: 0.5}
                    padding: 15,
                    draw_icon.svg: (ICON_FORBIDDEN)
                    icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                    text: "Cancel"
                }

                accept_button := RobrixPositiveIconButton {
                    width: 140
                    align: Align{x: 0.5, y: 0.5}
                    padding: 15,
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 16, height: 16, margin: Inset{left: -2, right: -1} }
                    text: "Create DID"
                }
            }

            status_label := Label {
                width: Fill,
                height: Fit,
                padding: 0,
                margin: 0,
                flow: Flow.Right{wrap: true},
                align: Align{x: 0.5, y: 0.0}
                draw_text +: {
                    text_style: REGULAR_TEXT {font_size: 11},
                    color: #000
                }
                text: "status label"
            }

            pending_cancel_evidence := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                margin: Inset{top: 8}
                draw_text +: {
                    text_style: REGULAR_TEXT {font_size: 10},
                    color: #555
                }
                text: "Local cancel before submit; pending cancel is not wired."
            }
        }
    }
}

/// Actions emitted by other widgets to instruct the main settings screen
/// to open or close the `CreateDidModal`.
#[derive(Clone, Copy, Debug)]
pub enum CreateDidModalAction {
    /// The settings screen should open the modal.
    Open,
    /// The settings screen should close the modal.
    Close,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum CreateDidModalState {
    /// Waiting for the user to enter identity details.
    #[default]
    WaitingForUserInput,
    /// Waiting for the identity to be created.
    WaitingForIdentityCreation,
    /// The identity was created successfully.
    IdentityCreated,
    /// An error occurred while creating the identity.
    IdentityCreationError,
}

#[derive(Script, ScriptHook, Widget)]
pub struct CreateDidModal {
    #[deref]
    view: View,
    #[rust]
    state: CreateDidModalState,
    #[rust]
    is_showing_error: bool,
}

impl Widget for CreateDidModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for CreateDidModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let mut accept_button = self.view.button(cx, ids!(accept_button));
        let cancel_button = self.view.button(cx, ids!(cancel_button));

        // Handle canceling/closing the modal.
        let cancel_clicked = cancel_button.clicked(actions);
        if cancel_clicked
            || actions
                .iter()
                .any(|a| matches!(a.downcast_ref(), Some(ModalAction::Dismissed)))
        {
            // If the modal was dismissed by clicking outside of it, we MUST NOT emit
            // a `CreateDidModalAction::Close` action, as that would cause
            // an infinite action feedback loop.
            if cancel_clicked {
                cx.action(CreateDidModalAction::Close);
            }

            // TODO: if possible, cancel the wallet creation request if it's still pending.

            return;
        }

        let username_input = self.view.text_input(cx, ids!(username_input));
        let alias_input = self.view.text_input(cx, ids!(alias_input));
        let server_input = self.view.text_input(cx, ids!(server_input));
        let did_server_input = self.view.text_input(cx, ids!(did_server_input));
        let mut status_label = self.view.label(cx, ids!(status_label));

        // Handle clicking the accept button.
        let mut needs_redraw = false;
        if accept_button.clicked(actions) {
            match self.state {
                // If the modal is in the "final" state, just close the modal.
                CreateDidModalState::IdentityCreated => {
                    self.state = CreateDidModalState::WaitingForUserInput;
                    cx.action(CreateDidModalAction::Close);
                }

                CreateDidModalState::WaitingForUserInput => {
                    let username_full = username_input.text();
                    let username = username_full.trim();

                    // Check to ensure that the user has entered all required fields.
                    if username.is_empty() {
                        self.is_showing_error = true;
                        script_apply_eval!(cx, status_label, {
                            text: "Please enter a DID username.",
                            draw_text +: {
                                color: mod.widgets.COLOR_FG_DANGER_RED,
                            },
                        });
                    } else {
                        let alias = match alias_input.text().trim() {
                            "" => None,
                            non_empty => Some(non_empty.to_string()),
                        };
                        let server = match server_input.text().trim() {
                            "" => server_input.empty_text(),
                            non_empty => non_empty.to_string(),
                        };
                        let did_server = match did_server_input.text().trim() {
                            "" => did_server_input.empty_text(),
                            non_empty => non_empty.to_string(),
                        };
                        let pending_cancel_packet = tsp_did_pending_cancel_operation_packet_label(
                            username,
                            alias.as_deref(),
                            &server,
                            &did_server,
                        );

                        // Submit the identity creation request to the TSP async worker thread.
                        tsp::submit_tsp_request(tsp::TspRequest::CreateDid {
                            username: username.to_string(),
                            alias,
                            server,
                            did_server,
                        });

                        self.state = CreateDidModalState::WaitingForIdentityCreation;
                        self.is_showing_error = false;
                        script_apply_eval!(cx, status_label, {
                            text: "Waiting for identity to be created and published...",
                            draw_text +: {
                                color: mod.widgets.COLOR_ACTIVE_PRIMARY_DARKER,
                            },
                        });
                        self.view
                            .label(cx, ids!(pending_cancel_evidence))
                            .set_text(cx, &pending_cancel_packet);
                        accept_button.set_enabled(cx, false);
                        cancel_button.set_enabled(cx, false); // TODO: support canceling the identity creation request?
                        username_input.set_is_read_only(cx, true);
                        alias_input.set_is_read_only(cx, true);
                        server_input.set_is_read_only(cx, true);
                        did_server_input.set_is_read_only(cx, true);
                    }

                    needs_redraw = true;
                }

                _ => {}
            }
        }

        // If the user changes any of the input fields, clear the error message
        // and reset the accept button to its default state.
        if self.is_showing_error {
            if username_input.changed(actions).is_some()
                || alias_input.changed(actions).is_some()
                || server_input.changed(actions).is_some()
                || did_server_input.changed(actions).is_some()
            {
                self.is_showing_error = false;
                self.view.label(cx, ids!(status_label)).set_text(cx, "");
                self.state = CreateDidModalState::WaitingForUserInput;
                script_apply_eval!(cx, accept_button, {
                    text: "Create DID",
                    enabled: true,
                    draw_text +: {
                        color: mod.widgets.COLOR_FG_ACCEPT_GREEN,
                    },
                });
                needs_redraw = true;
            }
        }

        for action in actions {
            match action.downcast_ref() {
                Some(tsp::TspIdentityAction::DidCreationResult(Ok(did))) => {
                    self.state = CreateDidModalState::IdentityCreated;
                    self.is_showing_error = false;
                    let message = format!("Successfully created and published DID: \"{}\"", did);
                    script_apply_eval!(cx, status_label, {
                        text: #(message),
                        draw_text +: {
                            color: mod.widgets.COLOR_FG_ACCEPT_GREEN,
                        },
                    });
                    script_apply_eval!(cx, accept_button, {
                        enabled: true,
                        text: "Okay",
                        draw_bg +: {
                            color: mod.widgets.COLOR_ACTIVE_PRIMARY,
                        },
                        draw_icon +: {
                            color: mod.widgets.COLOR_PRIMARY,
                        }
                        draw_text +: {
                            color: mod.widgets.COLOR_PRIMARY,
                        },
                    });
                    cancel_button.set_visible(cx, false);
                    needs_redraw = true;
                }

                // Upon an error, update the status label and disable the accept button.
                // Re-enable the input fields so the user can change the input values to try again.
                Some(tsp::TspIdentityAction::DidCreationResult(Err(e))) => {
                    self.state = CreateDidModalState::IdentityCreationError;
                    self.is_showing_error = true;
                    let message = format!("Failed to create DID: {e}");
                    script_apply_eval!(cx, status_label, {
                        text: #(message),
                        draw_text +: {
                            color: mod.widgets.COLOR_FG_DANGER_RED,
                        },
                    });
                    self.view
                        .label(cx, ids!(pending_cancel_evidence))
                        .set_text(cx, "DID creation failed; inputs are unlocked again.");
                    accept_button.set_enabled(cx, false);
                    cancel_button.set_enabled(cx, true);
                    username_input.set_is_read_only(cx, false);
                    alias_input.set_is_read_only(cx, false);
                    server_input.set_is_read_only(cx, false);
                    did_server_input.set_is_read_only(cx, false);
                    needs_redraw = true;
                }

                _ => {}
            }
        }

        if needs_redraw {
            self.view.redraw(cx);
        }
    }
}

impl CreateDidModal {
    pub fn show(&mut self, cx: &mut Cx) {
        self.state = CreateDidModalState::WaitingForUserInput;
        let accept_button = self.view.button(cx, ids!(accept_button));
        let cancel_button = self.view.button(cx, ids!(cancel_button));
        accept_button.set_text(cx, "Create DID");
        cancel_button.set_text(cx, "Cancel");
        accept_button.reset_hover(cx);
        cancel_button.reset_hover(cx);
        accept_button.set_enabled(cx, true);
        cancel_button.set_enabled(cx, true);
        accept_button.set_visible(cx, true);
        cancel_button.set_visible(cx, true);
        // TODO: return buttons to their default state/appearance
        self.view
            .text_input(cx, ids!(username_input))
            .set_is_read_only(cx, false);
        self.view
            .text_input(cx, ids!(alias_input))
            .set_is_read_only(cx, false);
        self.view
            .text_input(cx, ids!(server_input))
            .set_is_read_only(cx, false);
        self.view
            .text_input(cx, ids!(did_server_input))
            .set_is_read_only(cx, false);
        self.view.label(cx, ids!(status_label)).set_text(cx, "");
        self.view
            .label(cx, ids!(pending_cancel_evidence))
            .set_text(cx, TSP_DID_PENDING_CANCEL_COMPACT_LABEL);
        self.is_showing_error = false;
        self.view.redraw(cx);
    }
}

impl CreateDidModalRef {
    pub fn show(&self, cx: &mut Cx) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.show(cx);
    }
}
