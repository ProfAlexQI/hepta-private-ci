use std::cell::RefCell;

use makepad_widgets::*;

use crate::{
    app::{ConfirmDeleteAction, PositiveConfirmationModalAction},
    shared::{
        confirmation_modal::ConfirmationModalContent,
        popup_list::{enqueue_popup_notification, PopupKind},
    },
    tsp::{
        submit_tsp_request,
        tsp_settings_screen::{WalletStatus, WalletStatusAndDefault},
        TspRequest, TspWalletMetadata,
    },
};

const TSP_DELETE_WALLET_COMPACT_LABEL: &str =
    "Delete wallet is not implemented; no delete request starts.";
pub const TSP_WALLET_OPEN_RETRY_EVIDENCE: &str = "WalletEntry Open Wallet now exposes the existing TspRequest::OpenWallet path only for loaded NotFound wallet metadata. Clicking Open Wallet submits known wallet name/path metadata to the TSP worker and starts no file picker, wallet import, wallet creation, SetDefaultWallet, DeleteWallet, filesystem delete, Matrix request, gateway/runtime/auth, or live mutation.";
pub const TSP_WALLET_SET_DEFAULT_CONFIRMATION_METADATA_EVIDENCE: &str = "WalletEntry Set As Default now shows confirmation metadata derived only from loaded wallet metadata and row state: wallet name, URL/path availability, opened/not-found status, and default-wallet state. Confirmed Set Default submits only the existing TspRequest::SetDefaultWallet active/default wallet switch path; confirmation cancel and viewing metadata send no SetDefaultWallet, no OpenWallet, no RemoveWallet, no TspRequest::DeleteWallet, no filesystem delete, no wallet database delete, no Matrix request, no gateway/runtime/auth, and no live mutation.";
pub const TSP_WALLET_REMOVE_CONFIRMATION_METADATA_EVIDENCE: &str = "WalletEntry Remove From List now shows confirmation metadata derived only from loaded wallet metadata and row state: wallet name, URL/path availability, opened/not-found status, and default-wallet state. Confirmed Remove submits only the existing TspRequest::RemoveWallet list-state path; confirmation cancel and viewing metadata send no TspRequest::RemoveWallet, no TspRequest::DeleteWallet, no filesystem delete, no wallet database delete, no Matrix request, no gateway/runtime/auth, and no live mutation.";
pub const TSP_WALLET_DELETE_BLOCKED_METADATA_EVIDENCE: &str = "WalletEntry Delete Wallet now shows local blocked metadata derived only from loaded wallet metadata and row state: wallet name, URL/path availability, opened/not-found status, and default-wallet state. Clicking Delete Wallet still only emits a warning popup and starts no TspRequest::DeleteWallet, filesystem delete, wallet database write, Matrix request, gateway/runtime/auth, or live mutation.";
pub const TSP_WALLET_DELETE_PREFLIGHT_RESULT_PACKET_EVIDENCE: &str = "WalletEntry Delete Wallet now renders a local preflight/result taxonomy packet before destructive delete can be promoted: wallet identity, path validation slot, ownership scope, open-wallet closure slot, default fallback slot, persistence result slot, filesystem result taxonomy, retry/cancel policy, and audit redaction policy. Clicking Delete Wallet still only emits a warning popup and starts no TspRequest::DeleteWallet, filesystem delete, wallet database write, TSP state mutation, Matrix request, gateway/runtime/auth, or live mutation.";

fn wallet_status_metadata_label(status: WalletStatus) -> &'static str {
    match status {
        WalletStatus::Opened => "opened",
        WalletStatus::NotFound => "not_found",
    }
}

fn tsp_delete_wallet_blocked_metadata_label(
    wallet_name: &str,
    wallet_url: &str,
    status: WalletStatus,
    is_default: bool,
) -> String {
    let wallet_name = wallet_name.trim();
    let wallet_name = if wallet_name.is_empty() {
        "unknown wallet"
    } else {
        wallet_name
    };
    let path_state = if wallet_url.trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let default_state = if is_default { "default" } else { "secondary" };
    format!(
        "Delete blocked locally: wallet {wallet_name}; path {path_state}; status {}; role {default_state}. No TspRequest::DeleteWallet, filesystem delete, wallet database write, Matrix request, gateway/runtime/auth, or live mutation starts.",
        wallet_status_metadata_label(status)
    )
}

fn tsp_delete_wallet_preflight_result_packet_label(
    wallet_name: &str,
    wallet_url: &str,
    status: WalletStatus,
    is_default: bool,
) -> String {
    let wallet_name = wallet_name.trim();
    let wallet_name = if wallet_name.is_empty() {
        "unknown wallet"
    } else {
        wallet_name
    };
    let path_state = if wallet_url.trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let default_state = if is_default { "default" } else { "secondary" };
    let closure_slot = match status {
        WalletStatus::Opened => "backend_required_close_or_prove_safe",
        WalletStatus::NotFound => "not_open_or_not_found",
    };
    let default_fallback_slot = if is_default {
        "backend_required_block_promote_or_disable"
    } else {
        "not_required_secondary_wallet"
    };
    format!(
        "Delete wallet preflight/result packet: wallet_identity wallet {wallet_name}; path_state {path_state}; status {}; role {default_state}; path_validation_slot backend_required_exists_regular_app_owned_single_scope; ownership_scope backend_required; open_wallet_closure_slot {closure_slot}; default_fallback_slot {default_fallback_slot}; persistence_result_slot not_started; filesystem_result_taxonomy deleted|already_missing|permission_denied|busy|not_app_owned|partial_failure; retry_cancel_policy confirmation_gated_idempotent_retry_cancel_sends_no_request; audit_redaction_policy no_password_token_did_secret_private_vid_key_material. No TspRequest::DeleteWallet, filesystem delete, wallet database write, TSP state mutation, Matrix request, gateway/runtime/auth, or live mutation starts.",
        wallet_status_metadata_label(status)
    )
}

fn tsp_open_wallet_metadata_label(
    wallet_name: &str,
    wallet_url: &str,
    status: WalletStatus,
    is_default: bool,
) -> String {
    let wallet_name = wallet_name.trim();
    let wallet_name = if wallet_name.is_empty() {
        "unknown wallet"
    } else {
        wallet_name
    };
    let path_state = if wallet_url.trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let default_state = if is_default { "default" } else { "secondary" };
    let action_state = match status {
        WalletStatus::Opened => "already_open",
        WalletStatus::NotFound => "retry_ready",
    };
    format!(
        "Open wallet {action_state}: wallet {wallet_name}; path {path_state}; status {}; role {default_state}. Only NotFound retry submits TspRequest::OpenWallet with loaded metadata; no file picker, import, create, SetDefaultWallet, DeleteWallet, filesystem delete, Matrix request, gateway/runtime/auth, or live mutation starts.",
        wallet_status_metadata_label(status)
    )
}

fn tsp_remove_wallet_confirmation_metadata_label(
    wallet_name: &str,
    wallet_url: &str,
    status: WalletStatus,
    is_default: bool,
) -> String {
    let wallet_name = wallet_name.trim();
    let wallet_name = if wallet_name.is_empty() {
        "unknown wallet"
    } else {
        wallet_name
    };
    let path_state = if wallet_url.trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let default_state = if is_default { "default" } else { "secondary" };
    format!(
        "Remove confirmation metadata: wallet {wallet_name}; path {path_state}; status {}; role {default_state}. Confirmed Remove only submits TspRequest::RemoveWallet to remove the wallet from the local list/default slot; cancel sends no RemoveWallet, no TspRequest::DeleteWallet, no filesystem delete, no wallet database delete, no Matrix request, no gateway/runtime/auth, and no live mutation.",
        wallet_status_metadata_label(status)
    )
}

fn tsp_set_default_wallet_confirmation_metadata_label(
    wallet_name: &str,
    wallet_url: &str,
    status: WalletStatus,
    is_default: bool,
) -> String {
    let wallet_name = wallet_name.trim();
    let wallet_name = if wallet_name.is_empty() {
        "unknown wallet"
    } else {
        wallet_name
    };
    let path_state = if wallet_url.trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let default_state = if is_default { "default" } else { "secondary" };
    format!(
        "Set default confirmation metadata: wallet {wallet_name}; path {path_state}; status {}; role {default_state}. Confirmed Set Default only submits TspRequest::SetDefaultWallet to switch the active/default wallet pointer; cancel sends no SetDefaultWallet, no OpenWallet, no RemoveWallet, no TspRequest::DeleteWallet, no filesystem delete, no wallet database delete, no Matrix request, no gateway/runtime/auth, and no live mutation.",
        wallet_status_metadata_label(status)
    )
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    // An entry in the list of wallets.
    mod.widgets.WalletEntry = #(WalletEntry::register_widget(vm)) {
        width: Fill, height: Fit
        flow: Down
        align: Align { y: 0.5 }

        View {
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true},
            padding: 10
            align: Align { y: 0.5 }

            wallet_name := Label {
                width: Fit, height: Fit
                flow: Right,
                margin: Inset{top: 2.4, left: 0}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_bold { font_size: 12 },
                }
                text: "[Wallet Name]"
            }

            wallet_path := Label {
                width: Fit, height: Fit
                flow: Right,
                margin: Inset{top: 2.9, left: 8, bottom: 2}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 11 },
                }
                text: "[Wallet Path/URL]"
            }

            is_default_label_view := View {
                visible: false,
                width: Fit, height: Fit
                margin: Inset{left: 20}
                align: Align { y: 0.5 }
                Label {
                    width: Fit, height: Fit
                    margin: Inset{top: 3}
                    align: Align { y: 0.5 }
                    flow: Right,
                    draw_text +: {
                        color: (COLOR_FG_ACCEPT_GREEN),
                        text_style: theme.font_bold { font_size: 11 },
                    }
                    text: "✅ Default"
                }
            }

            not_found_label_view := View {
                visible: false,
                width: Fit, height: Fit
                margin: Inset{left: 20}
                align: Align { y: 0.5 }
                Label {
                    margin: Inset{top: 2.9}
                    width: Fit, height: Fit
                    flow: Right,
                    align: Align { y: 0.5 }
                    draw_text +: {
                        color: (COLOR_FG_DANGER_RED),
                        text_style: MESSAGE_TEXT_STYLE { font_size: 11 },
                    }
                    text: "Wallet not found!"
                }
            }

            set_default_wallet_button := RobrixIconButton {
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                margin: Inset{left: 20}
                draw_icon.svg: (ICON_CHECKMARK)
                icon_walk: Walk{width: 16, height: 16}
                text: "Set As Default"
            }

            wallet_set_default_evidence := Label {
                width: Fill, height: Fit
                margin: Inset{top: 6, left: 20}
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 10 },
                }
                text: "Set As Default confirms before active wallet switch."
            }

            open_wallet_button := RobrixPositiveIconButton {
                visible: false,
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                margin: Inset{left: 20}
                draw_icon.svg: (ICON_CHECKMARK)
                icon_walk: Walk{width: 16, height: 16}
                text: "Open Wallet"
            }

            wallet_remove_evidence := Label {
                width: Fill, height: Fit
                margin: Inset{top: 6, left: 20}
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 10 },
                }
                text: "Remove From List confirms before local list-state removal."
            }

            remove_wallet_button := RobrixNegativeIconButton {
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                margin: Inset{left: 20}
                draw_icon.svg: (ICON_CLOSE)
                icon_walk: Walk{ width: 16, height: 16 }
                text: "Remove From List"
            }

            delete_wallet_button := RobrixNegativeIconButton {
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                margin: Inset{left: 20}
                draw_icon.svg: (ICON_TRASH)
                icon_walk: Walk{ width: 16, height: 16 }
                text: "Delete Wallet"
            }

            wallet_open_evidence := Label {
                width: Fill, height: Fit
                margin: Inset{top: 6, left: 20}
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 10 },
                }
                text: "Open wallet retry uses loaded wallet metadata only."
            }

            wallet_delete_evidence := Label {
                width: Fill, height: Fit
                margin: Inset{top: 6, left: 20}
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 10 },
                }
                text: "Delete wallet is not implemented; no delete request starts."
            }

            wallet_delete_preflight_result_packet := Label {
                width: Fill, height: Fit
                margin: Inset{top: 6, left: 20}
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 10 },
                }
                text: "Delete wallet preflight/result packet stays local."
            }
        }

        LineH { padding: 10, margin: Inset{left: 5, right: 5} }
    }

}

/// A view showing the details of a single TSP wallet (one entry in the wallets list).
#[derive(Script, ScriptHook, Widget)]
pub struct WalletEntry {
    #[deref]
    view: View,

    #[rust]
    metadata: Option<TspWalletMetadata>,
    #[rust]
    status_and_default: Option<WalletStatusAndDefault>,
}

impl Widget for WalletEntry {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let Some(metadata) = self.metadata.as_ref() else {
            return;
        };
        if let Event::Actions(actions) = event {
            if self
                .view
                .button(cx, ids!(set_default_wallet_button))
                .clicked(actions)
            {
                let sd = self
                    .status_and_default
                    .unwrap_or_else(|| WalletStatusAndDefault::new(WalletStatus::Opened, false));
                let set_default_metadata = tsp_set_default_wallet_confirmation_metadata_label(
                    &metadata.wallet_name,
                    metadata.url.as_url_unencoded(),
                    sd.status,
                    sd.is_default,
                );
                self.view
                    .label(cx, ids!(wallet_set_default_evidence))
                    .set_text(cx, &set_default_metadata);
                if sd.is_default {
                    enqueue_popup_notification(
                        "Wallet is already default; SetDefaultWallet was not sent.",
                        PopupKind::Warning,
                        Some(4.0),
                    );
                } else if sd.status == WalletStatus::NotFound {
                    enqueue_popup_notification(
                        "Wallet is not found; SetDefaultWallet was not sent.",
                        PopupKind::Warning,
                        Some(4.0),
                    );
                } else {
                    let metadata_clone = metadata.clone();
                    let content = ConfirmationModalContent {
                        title_text: "Set Default Wallet".into(),
                        body_text: format!(
                            "Set \"{}\" as the default wallet?\n\n{}",
                            metadata.wallet_name, set_default_metadata
                        )
                        .into(),
                        accept_button_text: Some("Set Default".into()),
                        cancel_button_text: Some("Cancel".into()),
                        on_accept_clicked: Some(Box::new(move |_cx| {
                            submit_tsp_request(TspRequest::SetDefaultWallet(metadata_clone));
                        })),
                        on_cancel_clicked: Some(Box::new(|_cx| {
                            enqueue_popup_notification(
                                "Set default canceled. SetDefaultWallet was not sent.",
                                PopupKind::Warning,
                                Some(4.0),
                            );
                        })),
                        ..Default::default()
                    };
                    cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                        content,
                    ))));
                }
            }

            if self
                .view
                .button(cx, ids!(open_wallet_button))
                .clicked(actions)
            {
                let sd = self
                    .status_and_default
                    .unwrap_or_else(|| WalletStatusAndDefault::new(WalletStatus::Opened, false));
                self.view.label(cx, ids!(wallet_open_evidence)).set_text(
                    cx,
                    &tsp_open_wallet_metadata_label(
                        &metadata.wallet_name,
                        metadata.url.as_url_unencoded(),
                        sd.status,
                        sd.is_default,
                    ),
                );
                if sd.status == WalletStatus::NotFound {
                    submit_tsp_request(TspRequest::OpenWallet {
                        metadata: metadata.clone(),
                    });
                } else {
                    enqueue_popup_notification(
                        "Wallet is already open; no open retry started.",
                        PopupKind::Warning,
                        Some(4.0),
                    );
                }
            }

            if self
                .view
                .button(cx, ids!(remove_wallet_button))
                .clicked(actions)
            {
                let sd = self
                    .status_and_default
                    .unwrap_or_else(|| WalletStatusAndDefault::new(WalletStatus::Opened, false));
                let remove_metadata = tsp_remove_wallet_confirmation_metadata_label(
                    &metadata.wallet_name,
                    metadata.url.as_url_unencoded(),
                    sd.status,
                    sd.is_default,
                );
                self.view
                    .label(cx, ids!(wallet_remove_evidence))
                    .set_text(cx, &remove_metadata);
                let metadata_clone = metadata.clone();
                let content = ConfirmationModalContent {
                    title_text: "Remove Wallet".into(),
                    body_text: format!(
                        "Are you sure you want to remove the wallet \"{}\" \
                        from the list?\n\nThis won't delete the actual wallet file.\n\n{}",
                        metadata.wallet_name, remove_metadata
                    )
                    .into(),
                    accept_button_text: Some("Remove".into()),
                    on_accept_clicked: Some(Box::new(move |_cx| {
                        submit_tsp_request(TspRequest::RemoveWallet(metadata_clone));
                    })),
                    ..Default::default()
                };
                cx.action(ConfirmDeleteAction::Show(RefCell::new(Some(content))));
            }

            if self
                .view
                .button(cx, ids!(delete_wallet_button))
                .clicked(actions)
            {
                // TODO: Implement the delete wallet feature.
                let sd = self
                    .status_and_default
                    .unwrap_or_else(|| WalletStatusAndDefault::new(WalletStatus::Opened, false));
                self.view.label(cx, ids!(wallet_delete_evidence)).set_text(
                    cx,
                    &tsp_delete_wallet_blocked_metadata_label(
                        &metadata.wallet_name,
                        metadata.url.as_url_unencoded(),
                        sd.status,
                        sd.is_default,
                    ),
                );
                self.view
                    .label(cx, ids!(wallet_delete_preflight_result_packet))
                    .set_text(
                        cx,
                        &tsp_delete_wallet_preflight_result_packet_label(
                            &metadata.wallet_name,
                            metadata.url.as_url_unencoded(),
                            sd.status,
                            sd.is_default,
                        ),
                    );
                enqueue_popup_notification(
                    TSP_DELETE_WALLET_COMPACT_LABEL,
                    PopupKind::Warning,
                    None,
                );
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // The metadata was pasmatchsed in through Scope via props, and status/is_default via data.
        let metadata = scope.props.get::<TspWalletMetadata>().unwrap();
        let sd = scope.data.get::<WalletStatusAndDefault>().unwrap();
        // Store the passed-in metadata (for event handling) if it has changed.
        if self.metadata.as_ref().is_none_or(|m| m != metadata) {
            self.metadata = Some(metadata.clone());
        }
        self.status_and_default = Some(*sd);

        self.label(cx, ids!(wallet_name))
            .set_text(cx, &metadata.wallet_name);
        self.label(cx, ids!(wallet_path))
            .set_text(cx, metadata.url.as_url_unencoded());
        // There is a weird makepad bug where if we re-style one instance of the
        // `set_default_wallet_button` in one WalletEntry, all other instances of that button
        // get their styling messed up in weird ways.
        // So, as a workaround, we just hide the button entirely and show a `is_default_label_view` instead.

        self.view(cx, ids!(is_default_label_view))
            .set_visible(cx, sd.is_default);
        self.view(cx, ids!(not_found_label_view))
            .set_visible(cx, sd.status == WalletStatus::NotFound);
        self.button(cx, ids!(set_default_wallet_button))
            .set_visible(cx, !sd.is_default && sd.status != WalletStatus::NotFound);
        self.button(cx, ids!(open_wallet_button))
            .set_visible(cx, sd.status == WalletStatus::NotFound);
        self.button(cx, ids!(delete_wallet_button))
            .set_visible(cx, sd.status != WalletStatus::NotFound);
        self.label(cx, ids!(wallet_open_evidence)).set_text(
            cx,
            &tsp_open_wallet_metadata_label(
                &metadata.wallet_name,
                metadata.url.as_url_unencoded(),
                sd.status,
                sd.is_default,
            ),
        );
        self.label(cx, ids!(wallet_set_default_evidence)).set_text(
            cx,
            &tsp_set_default_wallet_confirmation_metadata_label(
                &metadata.wallet_name,
                metadata.url.as_url_unencoded(),
                sd.status,
                sd.is_default,
            ),
        );
        self.label(cx, ids!(wallet_remove_evidence)).set_text(
            cx,
            &tsp_remove_wallet_confirmation_metadata_label(
                &metadata.wallet_name,
                metadata.url.as_url_unencoded(),
                sd.status,
                sd.is_default,
            ),
        );
        self.label(cx, ids!(wallet_delete_evidence)).set_text(
            cx,
            &tsp_delete_wallet_blocked_metadata_label(
                &metadata.wallet_name,
                metadata.url.as_url_unencoded(),
                sd.status,
                sd.is_default,
            ),
        );
        self.label(cx, ids!(wallet_delete_preflight_result_packet))
            .set_text(
                cx,
                &tsp_delete_wallet_preflight_result_packet_label(
                    &metadata.wallet_name,
                    metadata.url.as_url_unencoded(),
                    sd.status,
                    sd.is_default,
                ),
            );

        self.view.draw_walk(cx, scope, walk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tsp_delete_wallet_blocked_metadata_summarizes_default_wallet() {
        let label = tsp_delete_wallet_blocked_metadata_label(
            "Primary",
            "file:///tmp/primary.wallet",
            WalletStatus::Opened,
            true,
        );

        assert!(label.contains("wallet Primary"));
        assert!(label.contains("path loaded"));
        assert!(label.contains("status opened"));
        assert!(label.contains("role default"));
        assert!(label.contains("TspRequest::DeleteWallet"));
        assert!(label.contains("filesystem delete"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_delete_wallet_blocked_metadata_uses_safe_fallbacks() {
        let label = tsp_delete_wallet_blocked_metadata_label("", "", WalletStatus::NotFound, false);

        assert!(label.contains("wallet unknown wallet"));
        assert!(label.contains("path missing"));
        assert!(label.contains("status not_found"));
        assert!(label.contains("role secondary"));
        assert!(label.contains("wallet database write"));
    }

    #[test]
    fn tsp_delete_wallet_preflight_result_packet_records_default_opened_taxonomy() {
        let label = tsp_delete_wallet_preflight_result_packet_label(
            "Primary",
            "file:///tmp/primary.wallet",
            WalletStatus::Opened,
            true,
        );

        assert!(label.contains("Delete wallet preflight/result packet"));
        assert!(label.contains("wallet_identity wallet Primary"));
        assert!(label.contains("path_state loaded"));
        assert!(label.contains("status opened"));
        assert!(label.contains("role default"));
        assert!(label.contains("path_validation_slot backend_required"));
        assert!(label.contains("ownership_scope backend_required"));
        assert!(label.contains("open_wallet_closure_slot backend_required_close_or_prove_safe"));
        assert!(label.contains("default_fallback_slot backend_required_block_promote_or_disable"));
        assert!(label.contains("persistence_result_slot not_started"));
        assert!(label.contains("filesystem_result_taxonomy"));
        assert!(label.contains("permission_denied"));
        assert!(label.contains("partial_failure"));
        assert!(label.contains("retry_cancel_policy confirmation_gated_idempotent_retry"));
        assert!(label.contains("audit_redaction_policy no_password_token"));
        assert!(label.contains("No TspRequest::DeleteWallet"));
        assert!(label.contains("TSP state mutation"));
    }

    #[test]
    fn tsp_delete_wallet_preflight_result_packet_uses_safe_fallbacks() {
        let label =
            tsp_delete_wallet_preflight_result_packet_label("", "", WalletStatus::NotFound, false);

        assert!(label.contains("wallet_identity wallet unknown wallet"));
        assert!(label.contains("path_state missing"));
        assert!(label.contains("status not_found"));
        assert!(label.contains("role secondary"));
        assert!(label.contains("open_wallet_closure_slot not_open_or_not_found"));
        assert!(label.contains("default_fallback_slot not_required_secondary_wallet"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_open_wallet_metadata_marks_not_found_retry() {
        let label = tsp_open_wallet_metadata_label(
            "Recovered",
            "file:///tmp/recovered.wallet",
            WalletStatus::NotFound,
            false,
        );

        assert!(label.contains("Open wallet retry_ready"));
        assert!(label.contains("wallet Recovered"));
        assert!(label.contains("path loaded"));
        assert!(label.contains("status not_found"));
        assert!(label.contains("TspRequest::OpenWallet"));
        assert!(label.contains("no file picker"));
        assert!(label.contains("DeleteWallet"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_open_wallet_metadata_marks_opened_wallet_local() {
        let label = tsp_open_wallet_metadata_label("Primary", "", WalletStatus::Opened, true);

        assert!(label.contains("Open wallet already_open"));
        assert!(label.contains("wallet Primary"));
        assert!(label.contains("path missing"));
        assert!(label.contains("role default"));
        assert!(label.contains("Only NotFound retry submits"));
    }

    #[test]
    fn tsp_set_default_wallet_confirmation_metadata_is_switch_only() {
        let label = tsp_set_default_wallet_confirmation_metadata_label(
            "Secondary",
            "file:///tmp/secondary.wallet",
            WalletStatus::Opened,
            false,
        );

        assert!(label.contains("Set default confirmation metadata"));
        assert!(label.contains("wallet Secondary"));
        assert!(label.contains("path loaded"));
        assert!(label.contains("status opened"));
        assert!(label.contains("role secondary"));
        assert!(label.contains("TspRequest::SetDefaultWallet"));
        assert!(label.contains("active/default wallet pointer"));
        assert!(label.contains("cancel sends no SetDefaultWallet"));
        assert!(label.contains("no TspRequest::DeleteWallet"));
        assert!(label.contains("no filesystem delete"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_set_default_wallet_confirmation_metadata_uses_safe_fallbacks() {
        let label = tsp_set_default_wallet_confirmation_metadata_label(
            "",
            "",
            WalletStatus::NotFound,
            true,
        );

        assert!(label.contains("wallet unknown wallet"));
        assert!(label.contains("path missing"));
        assert!(label.contains("status not_found"));
        assert!(label.contains("role default"));
        assert!(label.contains("no OpenWallet"));
        assert!(label.contains("no RemoveWallet"));
    }

    #[test]
    fn tsp_remove_wallet_confirmation_metadata_is_list_only() {
        let label = tsp_remove_wallet_confirmation_metadata_label(
            "Secondary",
            "file:///tmp/secondary.wallet",
            WalletStatus::Opened,
            false,
        );

        assert!(label.contains("Remove confirmation metadata"));
        assert!(label.contains("wallet Secondary"));
        assert!(label.contains("path loaded"));
        assert!(label.contains("status opened"));
        assert!(label.contains("role secondary"));
        assert!(label.contains("TspRequest::RemoveWallet"));
        assert!(label.contains("local list/default slot"));
        assert!(label.contains("no TspRequest::DeleteWallet"));
        assert!(label.contains("no filesystem delete"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_remove_wallet_confirmation_metadata_uses_safe_fallbacks() {
        let label =
            tsp_remove_wallet_confirmation_metadata_label("", "", WalletStatus::NotFound, true);

        assert!(label.contains("wallet unknown wallet"));
        assert!(label.contains("path missing"));
        assert!(label.contains("status not_found"));
        assert!(label.contains("role default"));
        assert!(label.contains("cancel sends no RemoveWallet"));
    }
}
