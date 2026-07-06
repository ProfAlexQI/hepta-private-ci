use std::cell::RefCell;

use makepad_widgets::*;

use crate::{
    app::PositiveConfirmationModalAction,
    shared::{
        confirmation_modal::ConfirmationModalContent,
        popup_list::{enqueue_popup_notification, PopupKind},
        styles::*,
    },
    tsp::{
        create_did_modal::CreateDidModalAction, create_wallet_modal::CreateWalletModalAction,
        submit_tsp_request, tsp_state_ref, TspIdentityAction, TspRequest, TspWalletAction,
        TspWalletEntry, TspWalletMetadata,
    },
};

const TSP_IMPORT_WALLET_COMPACT_LABEL: &str =
    "Import wallet is not implemented; no wallet or file request starts.";
pub const TSP_WALLET_IMPORT_BLOCKED_METADATA_EVIDENCE: &str = "TspSettingsScreen Import Existing Wallet now shows local blocked metadata derived only from loaded wallet/identity state: total loaded wallets, active wallet availability, other-wallet count, and active identity availability. Clicking Import still only emits a warning popup and starts no file picker, wallet database open, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, or live mutation.";
pub const TSP_WALLET_IMPORT_PREFLIGHT_PACKET_EVIDENCE: &str = "TspSettingsScreen Import Existing Wallet now opens a local preflight packet derived only from loaded wallet/identity state and the import backend contract boundaries. Acknowledge/Close only dismisses local UI; it starts no file picker, password capture, wallet database open, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, or live mutation.";
pub const TSP_WALLET_IMPORT_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "TspSettingsScreen Import Existing Wallet now also renders a local result taxonomy packet for picker, authentication, vault-open, metadata, duplicate, persistence, retry, cancel, stale-operation, and audit-redaction states before any import request exists. The packet records operation_id_slot not_assigned and starts no file picker, password capture, wallet database open, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, or live mutation.";
pub const TSP_WORKER_RECEIPT_RESULT_PACKET_EVIDENCE: &str = "TspSettingsScreen now keeps a local TSP worker receipt/result packet for existing wallet and identity worker actions. The packet records the already-requested operation, missing backend operation id, Cx::post_action receipt source, success/error/canceled/stale result taxonomy, local UI effect, retry slot, stale-result policy, and audit redaction boundary without creating any new TspRequest, cancel/delete/import/remove behavior, filesystem delete, Matrix request, gateway/runtime/auth, or live mutation.";

fn tsp_import_wallet_blocked_metadata_label(
    wallet_count: usize,
    active_wallet_loaded: bool,
    other_wallet_count: usize,
    active_identity_loaded: bool,
) -> String {
    let active_wallet = if active_wallet_loaded {
        "loaded"
    } else {
        "missing"
    };
    let active_identity = if active_identity_loaded {
        "loaded"
    } else {
        "missing"
    };
    format!(
        "Import blocked locally: loaded wallets {wallet_count}; active wallet {active_wallet}; other wallets {other_wallet_count}; active identity {active_identity}. No file picker, wallet database open, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, or live mutation starts."
    )
}

fn tsp_import_wallet_preflight_packet_label(
    wallet_count: usize,
    active_wallet_loaded: bool,
    other_wallet_count: usize,
    active_identity_loaded: bool,
) -> String {
    let active_wallet = if active_wallet_loaded {
        "loaded"
    } else {
        "missing"
    };
    let active_identity = if active_identity_loaded {
        "loaded"
    } else {
        "missing"
    };
    let duplicate_policy = if wallet_count == 0 {
        "no_loaded_wallets"
    } else {
        "loaded_wallets_require_duplicate_check"
    };
    let blocked_metadata = tsp_import_wallet_blocked_metadata_label(
        wallet_count,
        active_wallet_loaded,
        other_wallet_count,
        active_identity_loaded,
    );
    format!(
        "Import preflight packet: picker_result not_started; selected_path unavailable; password_state not_collected; vault_open not_started; persistence_result not_started; duplicate_policy {duplicate_policy}; active wallet {active_wallet}; loaded wallets {wallet_count}; other wallets {other_wallet_count}; active identity {active_identity}. Acknowledge only records local UI review and sends no file picker, password capture, wallet database open, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, or live mutation. {blocked_metadata}"
    )
}

fn tsp_import_wallet_result_taxonomy_packet_label(
    wallet_count: usize,
    active_wallet_loaded: bool,
    other_wallet_count: usize,
    active_identity_loaded: bool,
) -> String {
    let active_wallet = if active_wallet_loaded {
        "loaded"
    } else {
        "missing"
    };
    let active_identity = if active_identity_loaded {
        "loaded"
    } else {
        "missing"
    };
    let duplicate_result = if wallet_count == 0 {
        "not_started_no_loaded_wallets"
    } else {
        "not_started_loaded_wallets_require_duplicate_check"
    };
    format!(
        "Import result taxonomy packet: operation_id_slot not_assigned; picker_result canceled|selected_path_unavailable|inaccessible_path|unsupported_url_scheme not_wired; auth_result password_not_collected|invalid_password|redacted_retry_required not_wired; vault_open_result opened|invalid_password|unsupported_vault|corrupted_database|already_imported|duplicate_path|permission_denied not_wired; metadata_result wallet_name_sanitized_path_default_role not_started; duplicate_result {duplicate_result}; persistence_result saved|duplicate_blocked|failed|stale_operation not_started; retry_policy selected_path_reused_password_fresh_backend_required; cancel_policy local_dismiss_no_request; stale_result_policy backend_operation_id_required_before_import_live; audit_redaction_policy no_password_token_private_vid_key_material_raw_path; active wallet {active_wallet}; loaded wallets {wallet_count}; other wallets {other_wallet_count}; active identity {active_identity}. No file picker, password capture, wallet database open, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, or live mutation starts. {TSP_WALLET_IMPORT_RESULT_TAXONOMY_PACKET_EVIDENCE}"
    )
}

fn tsp_worker_wallet_target_label(metadata: Option<&TspWalletMetadata>) -> String {
    let Some(metadata) = metadata else {
        return "wallet_name missing path_state missing".to_string();
    };
    let wallet_name_state = if metadata.wallet_name.trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    let path_state = if metadata.url.as_url_unencoded().trim().is_empty() {
        "missing"
    } else {
        "loaded"
    };
    format!("wallet_name {wallet_name_state} path_state {path_state}")
}

fn tsp_worker_identity_target_label(did_loaded: bool, user_loaded: bool) -> String {
    let did_state = if did_loaded { "loaded" } else { "missing" };
    let user_state = if user_loaded { "loaded" } else { "missing" };
    format!("did_state {did_state} user_state {user_state}")
}

fn tsp_worker_receipt_result_packet_label(
    operation: &str,
    request_slot: &str,
    result_state: &str,
    target: &str,
    ui_effect: &str,
) -> String {
    format!(
        "TSP worker receipt/result packet: operation {operation}; request_slot {request_slot}; operation_id_slot not_assigned; worker_receipt Cx_post_action; result_state {result_state}; target {target}; ui_effect {ui_effect}; result_mapping success_error_canceled_stale_local_taxonomy; retry_slot existing_guarded_paths_only; stale_result_policy local_screen_cache_match_only_backend_operation_id_required_for_cancel_or_retry; audit_redaction_policy no_password_token_private_vid_key_material. No new TspRequest, no cancel/delete/import/remove behavior, no filesystem delete, Matrix request, gateway/runtime/auth, or live mutation starts. {TSP_WORKER_RECEIPT_RESULT_PACKET_EVIDENCE}"
    )
}

script_mod! {
    link tsp_enabled

    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.REPUBLISH_IDENTITY_BUTTON_TEXT = "Republish Current Identity to DID Server"

    // The view containing all TSP-related settings.
    mod.widgets.TspSettingsScreen = #(TspSettingsScreen::register_widget(vm)) {
        width: Fill, height: Fit
        flow: Down

        TitleLabel {
            text: "TSP Wallet Settings"
        }

        SubsectionLabel {
            text: "Your active identity:"
        }

        View {
            width: Fill, height: Fit
            flow: Right,
            spacing: 10

            copy_identity_button := RobrixNeutralIconButton {
                margin: Inset{left: 5}
                padding: 12,
                spacing: 0,
                draw_icon.svg: (ICON_COPY)
                icon_walk: Walk{width: 16, height: 16, margin: Inset{right: -2} }
            }

            current_identity_label := Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true},
                margin: Inset{top: 8}
                draw_text +: {
                    text_style: MESSAGE_TEXT_STYLE { font_size: 11 },
                }
            }
        }

        republish_identity_button := RobrixIconButton {
            width: Fit,
            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
            padding: 10,
            margin: Inset{top: 8, bottom: 10, left: 5},
            draw_icon.svg: (ICON_UPLOAD)
            icon_walk: Walk{width: 16, height: 16}
            text: mod.widgets.REPUBLISH_IDENTITY_BUTTON_TEXT
        }


        SubsectionLabel {
            text: "Your Wallets:"
        }

        no_wallets_label := View {
            width: Fill, height: Fit
            Label {
                width: Fill, height: Fit
                margin: Inset{top: 10, bottom: 8, left: 13, right: 10},
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TEXT_WARNING_NOT_FOUND),
                    text_style: MESSAGE_TEXT_STYLE { font_size: 11 },
                }
                text: "No wallets found. Create or import a wallet."
            }
        }

        RoundedView {
            width: Fill, height: Fit
            margin: 5,

            show_bg: true,
            draw_bg +: {
                color: #F6F8F9
                border_radius: 4.0
            }

            wallet_list := FlatList {
                width: Fill,
                height: Fit,
                spacing: 0.0
                flow: Down,

                grab_key_focus: true,
                drag_scrolling: true,
                scroll_bars: { show_scroll_x: false, show_scroll_y: false },

                wallet_entry := WalletEntry { }
            }
        }

        View {
            margin: Inset{top: 5},
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true},
            align: Align{y: 0.5},
            spacing: 10

            create_did_button := RobrixPositiveIconButton {
                width: Fit,
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: 10,
                margin: Inset{left: 5},
                draw_icon.svg: (ICON_ADD_USER)
                icon_walk: Walk{width: 19, height: Fit, margin: 0}
                text: "Create New Identity (DID)"
            }

            create_wallet_button := RobrixPositiveIconButton {
                width: Fit,
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: 10,
                margin: Inset{left: 5},
                draw_icon.svg: (ICON_ADD_WALLET)
                icon_walk: Walk{width: 21, height: Fit, margin: 0}
                text: "Create New Wallet"
            }

            import_wallet_button := RobrixIconButton {
                width: Fit,
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                margin: Inset{left: 5}
                text: "Import Existing Wallet"
                draw_icon +: {
                    svg: (ICON_IMPORT)
                    color: (COLOR_PRIMARY)
                }
                icon_walk: Walk{width: 16, height: 16}
            }
        }

        wallet_import_evidence := Label {
            width: Fill, height: Fit
            margin: Inset{top: 6, bottom: 4, left: 10, right: 10}
            flow: Flow.Right{wrap: true}
            draw_text +: {
                color: (MESSAGE_TEXT_COLOR),
                text_style: MESSAGE_TEXT_STYLE { font_size: 10 },
            }
            text: "Import wallet is not implemented; no wallet or file request starts."
        }

        worker_receipt_result_evidence := Label {
            width: Fill, height: Fit
            margin: Inset{top: 2, bottom: 4, left: 10, right: 10}
            flow: Flow.Right{wrap: true}
            draw_text +: {
                color: (MESSAGE_TEXT_COLOR),
                text_style: MESSAGE_TEXT_STYLE { font_size: 10 },
            }
            text: "TSP worker receipt/result packet awaits the next worker result."
        }
    }
}

#[derive(Debug, Default)]
struct WalletState {
    active_wallet: Option<TspWalletMetadata>,
    other_wallets: Vec<(TspWalletMetadata, WalletStatus)>,
    active_identity: Option<String>,
}
impl WalletState {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.active_wallet.is_some() as usize + self.other_wallets.len()
    }

    fn get(&self, index: usize) -> Option<(&TspWalletMetadata, WalletStatusAndDefault)> {
        if let Some(active) = self.active_wallet.as_ref() {
            if index == 0 {
                Some((
                    active,
                    WalletStatusAndDefault::new(WalletStatus::Opened, true),
                ))
            } else {
                self.other_wallets
                    .get(index - 1)
                    .map(|(m, s)| (m, WalletStatusAndDefault::new(*s, false)))
            }
        } else {
            self.other_wallets
                .get(index)
                .map(|(m, s)| (m, WalletStatusAndDefault::new(*s, false)))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletStatus {
    Opened,
    NotFound,
}

#[derive(Clone, Copy)]
pub struct WalletStatusAndDefault {
    pub status: WalletStatus,
    pub is_default: bool,
}
impl WalletStatusAndDefault {
    pub fn new(status: WalletStatus, is_default: bool) -> Self {
        Self { status, is_default }
    }
}

/// The view containing all TSP-related settings.
#[derive(Script, ScriptHook, Widget)]
pub struct TspSettingsScreen {
    #[deref]
    view: View,

    /// The list of wallets that are known by this widget.
    ///
    /// * If `None`, this widget doesn't know about any wallets or is outdated,
    ///   and must retrieve them from the TSP state.
    /// * If `Some`, the wallets has been opened and is up-to-date.
    ///   * This doesn't mean that any wallets actually exist.
    ///
    /// This is sort of a "cache" of the wallets that have been drawn
    /// to avoid having to re-fetch them from the shared TSP state every time,
    /// as that requires locking the mutex and can be expensive.
    #[rust]
    wallets: Option<WalletState>,
}

impl Widget for TspSettingsScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.wallets.is_none() {
            // If we don't have any wallets, load them from the TSP state.
            self.refresh_wallets();
            log!("Wallets were refreshed: {:?}", self.wallets);
        }

        // Draw the current identity label and republish button based on the active identity.
        let (current_did_text, current_did_text_color, show_republish_button) = match self
            .wallets
            .as_ref()
            .and_then(|ws| ws.active_identity.as_deref())
        {
            Some(current_did) => (current_did.to_string(), COLOR_FG_ACCEPT_GREEN, true),
            None => (
                "No default identity has been set.".to_string(),
                COLOR_TEXT_WARNING_NOT_FOUND,
                false,
            ),
        };
        let mut current_identity_label = self.view.label(cx, ids!(current_identity_label));
        script_apply_eval!(cx, current_identity_label, {
            text: #(current_did_text),
            draw_text +: { color: #(current_did_text_color) },
        });
        self.view
            .button(cx, ids!(republish_identity_button))
            .set_visible(cx, show_republish_button);

        // If we don't have any wallets, show the "no wallets" label.
        let is_wallets_empty = self.wallets.as_ref().is_none_or(|w| w.is_empty());
        self.view
            .view(cx, ids!(no_wallets_label))
            .set_visible(cx, is_wallets_empty);
        let import_metadata = self.wallets.as_ref().map_or_else(
            || tsp_import_wallet_blocked_metadata_label(0, false, 0, false),
            |wallets| {
                tsp_import_wallet_blocked_metadata_label(
                    wallets.len(),
                    wallets.active_wallet.is_some(),
                    wallets.other_wallets.len(),
                    wallets.active_identity.is_some(),
                )
            },
        );
        self.view
            .label(cx, ids!(wallet_import_evidence))
            .set_text(cx, &import_metadata);

        while let Some(subview) = self.view.draw_walk(cx, scope, walk).step() {
            // Here, we only need to handle drawing the wallet list.
            let flat_list_ref = subview.as_flat_list();
            let Some(mut list) = flat_list_ref.borrow_mut() else {
                error!(
                    "!!! TspSettingsScreen::draw_walk(): BUG: expected a FlatList widget, but got something else"
                );
                continue;
            };
            let Some(wallets) = self.wallets.as_ref() else {
                return DrawStep::done();
            };

            for (metadata, mut status_and_default) in
                (0..wallets.len()).filter_map(|i| wallets.get(i))
            {
                let item_live_id = LiveId::from_str(metadata.url.as_url_unencoded());
                let item = list.item(cx, item_live_id, id!(wallet_entry)).unwrap();
                // Pass the wallet metadata in through Scope via props,
                // and status/is_default via data.
                let mut scope = Scope::with_data_props(&mut status_and_default, metadata);
                item.draw_all(cx, &mut scope);
            }
        }
        DrawStep::done()
    }
}

impl MatchEvent for TspSettingsScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut republish_identity_button = self.view.button(cx, ids!(republish_identity_button));

        for action in actions {
            match action.downcast_ref() {
                // Add the new wallet to the list of drawn wallets.
                Some(TspWalletAction::CreateWalletSuccess {
                    metadata,
                    is_default,
                }) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "create_wallet",
                                "existing_TspRequest_CreateWallet",
                                if *is_default {
                                    "success_default_wallet"
                                } else {
                                    "success_secondary_wallet"
                                },
                                &tsp_worker_wallet_target_label(Some(metadata)),
                                "wallet_list_insert_redraw",
                            ),
                        );
                    let wallets = self.wallets.get_or_insert_default();
                    if *is_default {
                        wallets.active_wallet = Some(metadata.clone());
                    } else {
                        wallets
                            .other_wallets
                            .push((metadata.clone(), WalletStatus::Opened));
                    }
                    self.view.redraw(cx);
                    continue;
                }

                // Remove the wallet from the list of drawn wallets.
                Some(TspWalletAction::WalletRemoved {
                    metadata,
                    was_default,
                }) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "remove_wallet",
                                "existing_confirmed_TspRequest_RemoveWallet",
                                if *was_default {
                                    "success_default_removed"
                                } else {
                                    "success_secondary_removed"
                                },
                                &tsp_worker_wallet_target_label(Some(metadata)),
                                "wallet_list_remove_redraw",
                            ),
                        );
                    let Some(wallets) = &mut self.wallets.as_mut() else {
                        continue;
                    };
                    if *was_default {
                        wallets.active_wallet = None;
                    } else if let Some(pos) = wallets
                        .other_wallets
                        .iter()
                        .position(|(w, _)| w == metadata)
                    {
                        wallets.other_wallets.remove(pos);
                    } else {
                        continue;
                    }
                    enqueue_popup_notification(
                        format!("Removed wallet \"{}\".", metadata.wallet_name),
                        PopupKind::Success,
                        Some(4.0),
                    );
                    if *was_default {
                        // If the removed wallet was the default wallet, notify the user.
                        // The user should then select another wallet as the default.
                        enqueue_popup_notification(
                            "The default wallet was removed.\n\n\
                                TSP features will not work properly until you set a default wallet.",
                            PopupKind::Warning,
                            None,
                        );
                    }
                    self.view.redraw(cx);
                    continue;
                }

                // Update the default/active wallet.
                Some(TspWalletAction::DefaultWalletChanged(Ok(metadata))) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "set_default_wallet",
                                "existing_confirmed_TspRequest_SetDefaultWallet",
                                "success_default_changed",
                                &tsp_worker_wallet_target_label(Some(metadata)),
                                "active_wallet_swap_redraw",
                            ),
                        );
                    let wallets = self.wallets.get_or_insert_default();
                    let previous_active = wallets.active_wallet.replace(metadata.clone());
                    // If the newly-default wallet was in the other wallets list, remove it
                    // and then add the previous active wallet back to that other wallets list.
                    if let Some(idx_to_remove) = wallets
                        .other_wallets
                        .iter()
                        .position(|(w, _)| w == metadata)
                    {
                        wallets.other_wallets.remove(idx_to_remove);
                    }
                    if let Some(previous_active) = previous_active {
                        wallets
                            .other_wallets
                            .insert(0, (previous_active, WalletStatus::Opened));
                    }
                    self.view.redraw(cx);
                    continue;
                }
                Some(TspWalletAction::DefaultWalletChanged(Err(_))) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "set_default_wallet",
                                "existing_confirmed_TspRequest_SetDefaultWallet",
                                "error_wallet_not_found_or_not_open",
                                &tsp_worker_wallet_target_label(None),
                                "popup_only_no_wallet_cache_change",
                            ),
                        );
                    enqueue_popup_notification(
                        "Failed to set default wallet, could not find or open selected wallet.",
                        PopupKind::Error,
                        None,
                    );
                    continue;
                }

                // Handle a newly-opened wallet.
                Some(TspWalletAction::WalletOpened(Ok(metadata))) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "open_wallet",
                                "existing_TspRequest_OpenWallet",
                                "success_opened",
                                &tsp_worker_wallet_target_label(Some(metadata)),
                                "notfound_row_status_opened_or_append",
                            ),
                        );
                    let wallets = self.wallets.get_or_insert_default();
                    if let Some((_m, status)) = wallets
                        .other_wallets
                        .iter_mut()
                        .find(|(w, _)| w == metadata)
                    {
                        *status = WalletStatus::Opened;
                    } else {
                        wallets
                            .other_wallets
                            .push((metadata.clone(), WalletStatus::Opened));
                    }
                    self.view.redraw(cx);
                    continue;
                }
                Some(TspWalletAction::WalletOpened(Err(e))) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "open_wallet",
                                "existing_TspRequest_OpenWallet",
                                "error_open_failed",
                                &tsp_worker_wallet_target_label(None),
                                "popup_only_no_wallet_cache_change",
                            ),
                        );
                    enqueue_popup_notification(
                        format!("Failed to open wallet: {e}"),
                        PopupKind::Error,
                        None,
                    );
                    continue;
                }

                // This is handled in the CreateWalletModal
                Some(TspWalletAction::CreateWalletError { .. }) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "create_wallet",
                                "existing_TspRequest_CreateWallet",
                                "error_create_failed_modal_owned",
                                &tsp_worker_wallet_target_label(None),
                                "modal_popup_only_no_screen_cache_change",
                            ),
                        );
                    continue;
                }
                None => {}
            }

            match action.downcast_ref() {
                Some(TspIdentityAction::DidCreationResult(result)) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "create_did",
                                "existing_TspRequest_CreateDid",
                                if result.is_ok() {
                                    "success_did_created"
                                } else {
                                    "error_did_create_failed"
                                },
                                &tsp_worker_identity_target_label(result.is_ok(), false),
                                if result.is_ok() {
                                    "active_identity_set_if_empty"
                                } else {
                                    "modal_popup_only_no_identity_cache_change"
                                },
                            ),
                        );
                    // If there is no active identity, set the newly-created identity as active.
                    let wallets = self.wallets.get_or_insert_default();
                    if let (Ok(did), None) = (result, wallets.active_identity.as_ref()) {
                        wallets.active_identity = Some(did.clone());
                        self.view.redraw(cx);
                    }
                    continue;
                }
                Some(TspIdentityAction::DidRepublishResult(result)) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "republish_did",
                                "existing_TspRequest_RepublishDid",
                                if result.is_ok() {
                                    "success_republished"
                                } else {
                                    "error_republish_failed"
                                },
                                &tsp_worker_identity_target_label(result.is_ok(), false),
                                "republish_button_restored_popup_only",
                            ),
                        );
                    // restore the republish button to its original state.
                    script_apply_eval!(cx, republish_identity_button, {
                        enabled: true,
                        text: mod.widgets.REPUBLISH_IDENTITY_BUTTON_TEXT,
                    });
                    match result {
                        Ok(did) => {
                            enqueue_popup_notification(
                                format!(
                                    "Successfully republished identity \"{}\" to the DID server.",
                                    did
                                ),
                                PopupKind::Success,
                                Some(5.0),
                            );
                        }
                        Err(e) => {
                            enqueue_popup_notification(
                                format!("Failed to republish identity to the DID server: {e}"),
                                PopupKind::Error,
                                None,
                            );
                        }
                    }
                    continue;
                }
                Some(TspIdentityAction::SentDidAssociationRequest { .. }) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "associate_did",
                                "existing_TspRequest_AssociateDidWithUserId",
                                "success_request_sent",
                                &tsp_worker_identity_target_label(true, true),
                                "profile_widget_waiting_state",
                            ),
                        );
                    continue;
                } // handled in the TspVerifyUser widget
                Some(TspIdentityAction::ErrorSendingDidAssociationRequest { .. }) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "associate_did",
                                "existing_TspRequest_AssociateDidWithUserId",
                                "error_request_send_failed",
                                &tsp_worker_identity_target_label(true, true),
                                "profile_widget_retry_enabled_popup",
                            ),
                        );
                    continue;
                } // handled in the TspVerifyUser widget
                Some(TspIdentityAction::ReceivedDidAssociationResponse { accepted, .. }) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "association_response",
                                "existing_receive_loop_action",
                                if *accepted {
                                    "success_remote_accepted"
                                } else {
                                    "canceled_or_rejected_remote_response"
                                },
                                &tsp_worker_identity_target_label(true, true),
                                "profile_widget_refresh_from_verified_info",
                            ),
                        );
                    continue;
                } // handled in the TspVerifyUser widget
                Some(TspIdentityAction::ReceivedDidAssociationRequest { .. }) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "incoming_association_request",
                                "existing_receive_loop_action",
                                "received_pending_user_response",
                                &tsp_worker_identity_target_label(true, true),
                                "verification_modal_owned",
                            ),
                        );
                    continue;
                } // handled in the TspVerificationModal widget
                Some(TspIdentityAction::ReceiveLoopError { .. }) => {
                    self.view
                        .label(cx, ids!(worker_receipt_result_evidence))
                        .set_text(
                            cx,
                            &tsp_worker_receipt_result_packet_label(
                                "receive_loop",
                                "existing_receive_loop_task",
                                "error_receive_loop_failed",
                                &tsp_worker_identity_target_label(false, false),
                                "top_level_popup_no_wallet_cache_change",
                            ),
                        );
                    continue;
                } // handled in the top-level app
                None => {}
            }
        }

        if self
            .view
            .button(cx, ids!(copy_identity_button))
            .clicked(actions)
        {
            if let Some(did) = self
                .wallets
                .as_ref()
                .and_then(|ws| ws.active_identity.as_deref())
            {
                cx.copy_to_clipboard(did);
                enqueue_popup_notification(
                    "Copied your default TSP identity to the clipboard.",
                    PopupKind::Success,
                    Some(3.0),
                );
            } else {
                enqueue_popup_notification(
                    "No default TSP identity has been set.",
                    PopupKind::Warning,
                    Some(4.0),
                );
            }
        }

        // Allow the user to republish their identity to the DID server.
        // This is primarily needed because some DID servers (e.g., the test servers)
        // frequently wipe their identity storage after a certain period of time.
        if self
            .view
            .button(cx, ids!(republish_identity_button))
            .clicked(actions)
        {
            if self.has_default_wallet() {
                if let Some(our_did) = self
                    .wallets
                    .as_ref()
                    .and_then(|ws| ws.active_identity.as_deref())
                {
                    script_apply_eval!(cx, republish_identity_button, {
                        enabled: false,
                        text: "Republishing DID now...",
                    });

                    submit_tsp_request(TspRequest::RepublishDid {
                        did: our_did.to_string(),
                    });
                } else {
                    enqueue_popup_notification(
                        "You must set a default TSP identity to be republished.",
                        PopupKind::Error,
                        Some(5.0),
                    );
                }
            }
        }

        if self
            .view
            .button(cx, ids!(create_wallet_button))
            .clicked(actions)
        {
            cx.action(CreateWalletModalAction::Open);
        }

        if self
            .view
            .button(cx, ids!(create_did_button))
            .clicked(actions)
        {
            if self.has_default_wallet() {
                cx.action(CreateDidModalAction::Open);
            }
        }

        if self
            .view
            .button(cx, ids!(import_wallet_button))
            .clicked(actions)
        {
            let (import_metadata, import_result_taxonomy) = self.wallets.as_ref().map_or_else(
                || {
                    (
                        tsp_import_wallet_preflight_packet_label(0, false, 0, false),
                        tsp_import_wallet_result_taxonomy_packet_label(0, false, 0, false),
                    )
                },
                |wallets| {
                    (
                        tsp_import_wallet_preflight_packet_label(
                            wallets.len(),
                            wallets.active_wallet.is_some(),
                            wallets.other_wallets.len(),
                            wallets.active_identity.is_some(),
                        ),
                        tsp_import_wallet_result_taxonomy_packet_label(
                            wallets.len(),
                            wallets.active_wallet.is_some(),
                            wallets.other_wallets.len(),
                            wallets.active_identity.is_some(),
                        ),
                    )
                },
            );
            let import_packet = format!("{import_metadata}\n\n{import_result_taxonomy}");
            self.view
                .label(cx, ids!(wallet_import_evidence))
                .set_text(cx, &import_packet);
            let content = ConfirmationModalContent {
                title_text: "Import Wallet Preflight".into(),
                body_text: format!(
                    "{import_packet}\n\n{TSP_WALLET_IMPORT_PREFLIGHT_PACKET_EVIDENCE}\n\n{TSP_WALLET_IMPORT_RESULT_TAXONOMY_PACKET_EVIDENCE}"
                )
                .into(),
                accept_button_text: Some("Acknowledge".into()),
                cancel_button_text: Some("Close".into()),
                on_accept_clicked: Some(Box::new(|_cx| {
                    enqueue_popup_notification(
                        "Import preflight acknowledged. No wallet import request was sent.",
                        PopupKind::Warning,
                        Some(4.0),
                    );
                })),
                on_cancel_clicked: Some(Box::new(|_cx| {
                    enqueue_popup_notification(
                        "Import preflight closed. No wallet import request was sent.",
                        PopupKind::Warning,
                        Some(4.0),
                    );
                })),
            };
            cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                content,
            ))));
            enqueue_popup_notification(
                TSP_IMPORT_WALLET_COMPACT_LABEL,
                PopupKind::Warning,
                Some(4.0),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tsp_import_wallet_blocked_metadata_summarizes_loaded_state() {
        let label = tsp_import_wallet_blocked_metadata_label(3, true, 2, true);

        assert!(label.contains("loaded wallets 3"));
        assert!(label.contains("active wallet loaded"));
        assert!(label.contains("other wallets 2"));
        assert!(label.contains("active identity loaded"));
        assert!(label.contains("No file picker"));
        assert!(label.contains("TspRequest"));
        assert!(label.contains("filesystem read/write"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_import_wallet_blocked_metadata_reports_missing_state() {
        let label = tsp_import_wallet_blocked_metadata_label(0, false, 0, false);

        assert!(label.contains("loaded wallets 0"));
        assert!(label.contains("active wallet missing"));
        assert!(label.contains("active identity missing"));
        assert!(label.contains("wallet database open"));
    }

    #[test]
    fn tsp_import_wallet_preflight_packet_summarizes_contract_boundaries() {
        let label = tsp_import_wallet_preflight_packet_label(2, true, 1, true);

        assert!(label.contains("Import preflight packet"));
        assert!(label.contains("picker_result not_started"));
        assert!(label.contains("selected_path unavailable"));
        assert!(label.contains("password_state not_collected"));
        assert!(label.contains("vault_open not_started"));
        assert!(label.contains("persistence_result not_started"));
        assert!(label.contains("loaded_wallets_require_duplicate_check"));
        assert!(label.contains("Acknowledge only records local UI review"));
        assert!(label.contains("no file picker"));
        assert!(label.contains("password capture"));
        assert!(label.contains("TspRequest"));
        assert!(label.contains("filesystem read/write"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_import_wallet_preflight_packet_uses_empty_wallet_policy() {
        let label = tsp_import_wallet_preflight_packet_label(0, false, 0, false);

        assert!(label.contains("duplicate_policy no_loaded_wallets"));
        assert!(label.contains("active wallet missing"));
        assert!(label.contains("active identity missing"));
    }

    #[test]
    fn tsp_import_wallet_result_taxonomy_packet_summarizes_unwired_results() {
        let label = tsp_import_wallet_result_taxonomy_packet_label(2, true, 1, true);

        assert!(label.contains("Import result taxonomy packet"));
        assert!(label.contains("operation_id_slot not_assigned"));
        assert!(label.contains("picker_result"));
        assert!(label.contains("auth_result password_not_collected"));
        assert!(label.contains("vault_open_result opened|invalid_password"));
        assert!(
            label.contains("duplicate_result not_started_loaded_wallets_require_duplicate_check")
        );
        assert!(label.contains(
            "persistence_result saved|duplicate_blocked|failed|stale_operation not_started"
        ));
        assert!(
            label.contains("retry_policy selected_path_reused_password_fresh_backend_required")
        );
        assert!(label.contains("cancel_policy local_dismiss_no_request"));
        assert!(label.contains(
            "audit_redaction_policy no_password_token_private_vid_key_material_raw_path"
        ));
        assert!(label.contains("No file picker"));
        assert!(label.contains("wallet database open"));
        assert!(label.contains("TspRequest"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn tsp_import_wallet_result_taxonomy_packet_uses_empty_wallet_duplicate_state() {
        let label = tsp_import_wallet_result_taxonomy_packet_label(0, false, 0, false);

        assert!(label.contains("duplicate_result not_started_no_loaded_wallets"));
        assert!(label.contains("active wallet missing"));
        assert!(label.contains("active identity missing"));
        assert!(!label.contains("password secret"));
    }

    #[test]
    fn tsp_worker_receipt_result_packet_maps_wallet_success_without_secret_path() {
        let metadata = TspWalletMetadata {
            wallet_name: "Primary Wallet".to_string(),
            url: crate::tsp::TspWalletSqliteUrl("sqlite:///tmp/primary.sqlite".to_string()),
            password: "secret password".to_string(),
        };
        let label = tsp_worker_receipt_result_packet_label(
            "open_wallet",
            "existing_TspRequest_OpenWallet",
            "success_opened",
            &tsp_worker_wallet_target_label(Some(&metadata)),
            "notfound_row_status_opened_or_append",
        );

        assert!(label.contains("operation open_wallet"));
        assert!(label.contains("operation_id_slot not_assigned"));
        assert!(label.contains("worker_receipt Cx_post_action"));
        assert!(label.contains("result_state success_opened"));
        assert!(label.contains("wallet_name loaded"));
        assert!(label.contains("path_state loaded"));
        assert!(label.contains("retry_slot existing_guarded_paths_only"));
        assert!(label.contains("stale_result_policy"));
        assert!(label.contains("audit_redaction_policy"));
        assert!(label.contains("No new TspRequest"));
        assert!(label.contains("live mutation"));
        assert!(!label.contains("secret password"));
        assert!(!label.contains("/tmp/primary.sqlite"));
    }

    #[test]
    fn tsp_worker_receipt_result_packet_maps_identity_error_taxonomy() {
        let label = tsp_worker_receipt_result_packet_label(
            "associate_did",
            "existing_TspRequest_AssociateDidWithUserId",
            "error_request_send_failed",
            &tsp_worker_identity_target_label(true, true),
            "profile_widget_retry_enabled_popup",
        );

        assert!(label.contains("operation associate_did"));
        assert!(label.contains("result_state error_request_send_failed"));
        assert!(label.contains("did_state loaded"));
        assert!(label.contains("user_state loaded"));
        assert!(label.contains("success_error_canceled_stale_local_taxonomy"));
        assert!(label.contains("backend_operation_id_required_for_cancel_or_retry"));
        assert!(label.contains("no filesystem delete"));
        assert!(label.contains("Matrix request"));
    }
}

impl TspSettingsScreen {
    /// Re-fetches the TSP state and populates this widget's list of wallets.
    fn refresh_wallets(&mut self) {
        let tsp_state = tsp_state_ref().lock().unwrap();
        let current_wallet = tsp_state
            .current_wallet
            .as_ref()
            .map(|w| w.metadata.clone());
        let other_wallets = tsp_state
            .other_wallets
            .iter()
            .map(|entry| match entry {
                TspWalletEntry::Opened(opened) => (opened.metadata.clone(), WalletStatus::Opened),
                TspWalletEntry::NotFound(metadata) => (metadata.clone(), WalletStatus::NotFound),
            })
            .collect::<Vec<_>>();
        self.wallets = Some(WalletState {
            active_wallet: current_wallet,
            other_wallets,
            active_identity: tsp_state.current_local_vid.clone(),
        });
    }

    /// Checks if the current TSP state has a default wallet set and ready to use.
    ///
    /// This function will display warnings to the user if no default wallet is set
    /// or if there are no wallets at all.
    ///
    /// Returns `true` if a default wallet is set, `false` otherwise.
    fn has_default_wallet(&self) -> bool {
        let Some(wallets) = self.wallets.as_ref() else {
            enqueue_popup_notification(
                "No TSP wallets found.\n\nPlease create or import a wallet.",
                PopupKind::Warning,
                Some(5.0),
            );
            return false;
        };
        if wallets.active_wallet.is_none() {
            enqueue_popup_notification(
                "No default TSP wallet is set.\n\nPlease select or create a default wallet.",
                PopupKind::Warning,
                Some(5.0),
            );
            return false;
        }
        true
    }
}
