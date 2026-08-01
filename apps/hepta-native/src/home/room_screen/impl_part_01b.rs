impl RoomScreen {
    fn is_current_hepta_fixture_workspace(&self) -> bool {
        crate::hepta_fixture::is_fixture_mode_enabled()
            && self.room_name_id.as_ref().is_some_and(|room_name_id| {
                crate::hepta_fixture::is_fixture_room_id(room_name_id.room_id().as_str())
            })
    }

    fn apply_hepta_fixture_timeline_visibility(&mut self, cx: &mut Cx, visible: bool) {
        self.view
            .view(cx, ids!(telegram_room_header))
            .set_visible(cx, !visible);
        self.set_telegram_search_mode_visible(cx, false);
        self.reset_telegram_message_search_state(cx);
        self.reset_telegram_message_edit_history_state(cx);
        self.reset_telegram_message_report_status_state(cx);
        self.telegram_notifications_local_status.clear();
        self.telegram_notifications_result_detail_action.clear();
        self.telegram_notifications_preflight_detail_action.clear();
        self.telegram_notifications_retry_room_id = None;
        self.telegram_notifications_retry_mode = None;
        self.telegram_notifications_retry_default_timeline_kind = None;
        self.telegram_notifications_retry_default_mode = None;
        self.telegram_room_settings_local_status.clear();
        self.telegram_room_settings_refresh_detail_action.clear();
        self.telegram_room_settings_mutation_preflight_action
            .clear();
        self.set_telegram_room_actions_visible(cx, false, None);
        self.set_telegram_notifications_visible(cx, false);
        self.set_telegram_room_info_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, false);
        self.view.view(cx, ids!(timeline)).set_visible(cx, !visible);
        self.view
            .view(cx, ids!(hepta_fixture_timeline))
            .set_visible(cx, visible);
        self.view
            .view(cx, ids!(typing_notice))
            .set_visible(cx, false);
        self.view
            .view(cx, ids!(room_input_bar))
            .set_visible(cx, !visible);
        self.is_loaded = visible;
    }

    fn refresh_telegram_room_action_details(&mut self, cx: &mut Cx) {
        self.telegram_room_action_details = self.room_name_id.as_ref().and_then(|room_name_id| {
            if cx.has_global::<RoomsListRef>() {
                cx.get_global::<RoomsListRef>()
                    .get_room_context_menu_details(room_name_id.room_id())
            } else {
                None
            }
        });
        self.update_telegram_room_action_buttons(cx);
    }

    fn update_telegram_room_action_buttons(&mut self, cx: &mut Cx) {
        let mark_unread_button = self
            .view
            .button(cx, ids!(telegram_room_actions_strip.mark_unread_button));
        let favorite_button = self
            .view
            .button(cx, ids!(telegram_room_actions_strip.favorite_button));
        let priority_button = self
            .view
            .button(cx, ids!(telegram_room_actions_strip.priority_button));

        let Some(details) = self.telegram_room_action_details.as_ref() else {
            mark_unread_button.set_text(cx, "Unread");
            favorite_button.set_text(cx, "Fav");
            priority_button.set_text(cx, "Low");
            return;
        };

        if details.is_marked_unread {
            mark_unread_button.set_text(cx, "Read");
        } else {
            mark_unread_button.set_text(cx, "Unread");
        }

        if details.is_favorite {
            favorite_button.set_text(cx, "Unfav");
        } else {
            favorite_button.set_text(cx, "Fav");
        }

        if details.is_low_priority {
            priority_button.set_text(cx, "Normal");
        } else {
            priority_button.set_text(cx, "Low");
        }
    }

    fn warn_missing_telegram_room_action_state(&mut self, cx: &mut Cx, room_label: &str) {
        self.show_telegram_room_actions(cx, "Room state unavailable");
        enqueue_popup_notification(
            format!(
                "Room state for {room_label} is not available yet, so no favorite, priority, or unread mutation was sent."
            ),
            PopupKind::Warning,
            Some(4.0),
        );
    }

    fn show_telegram_room_status_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        title_label: &'static str,
        action_label: &'static str,
        accept_label: &'static str,
        request: MatrixRequest,
    ) {
        let room_label_for_accept = room_label.to_string();
        let room_label_for_cancel = room_label.to_string();
        let action_label_for_accept = action_label.to_string();
        let action_label_for_cancel = action_label.to_string();
        let content = ConfirmationModalContent {
            title_text: format!("Confirm {title_label}").into(),
            body_text: format!(
                "Update {room_label} to {action_label}? {ROOM_STATUS_CONFIRMATION_COMPACT_LABEL}"
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
                        "Room status update canceled for {room_label_for_cancel}: {action_label_for_cancel}. {ROOM_STATUS_CONFIRMATION_COMPACT_LABEL}"
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
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, true, Some("Confirmation required"));
        enqueue_popup_notification(
            format!(
                "Room status confirmation opened for {room_label}: {action_label}. {ROOM_STATUS_CONFIRMATION_COMPACT_LABEL}"
            ),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn update_telegram_room_info_strip(&mut self, cx: &mut Cx, room_label: &str) {
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let pinned_count = self.pinned_events.len();
        let member_text = member_count
            .map(|count| format!("{count} {ROOM_MEMBERS_COMPACT_LABEL}"))
            .unwrap_or_else(|| format!("{ROOM_MEMBERS_COMPACT_LABEL} loading"));
        let pinned_text = match pinned_count {
            0 => "no pinned messages loaded".to_string(),
            1 => "1 pinned message loaded".to_string(),
            count => format!("{count} pinned messages loaded"),
        };

        self.view
            .label(cx, ids!(telegram_room_info_strip.info_summary))
            .set_text(cx, room_label);
        self.view
            .label(cx, ids!(telegram_room_info_strip.info_meta))
            .set_text(cx, &format!("{member_text} / {pinned_text}"));
        self.view
            .label(cx, ids!(telegram_room_info_strip.info_subscription))
            .set_text(cx, ROOM_PINNED_COMPACT_LABEL);
        self.view
            .label(cx, ids!(telegram_room_info_strip.info_typing_subscription))
            .set_text(cx, ROOM_TYPING_COMPACT_LABEL);
        self.view
            .label(
                cx,
                ids!(telegram_room_info_strip.info_read_receipt_subscription),
            )
            .set_text(cx, ROOM_READ_RECEIPT_COMPACT_LABEL);
        self.view
            .label(cx, ids!(telegram_room_info_strip.info_unread_count_read))
            .set_text(cx, ROOM_UNREAD_COMPACT_LABEL);
        self.view
            .label(cx, ids!(telegram_room_info_strip.info_avatar_fetch))
            .set_text(cx, ROOM_AVATAR_COMPACT_LABEL);

        let state_text = self
            .telegram_room_action_details
            .as_ref()
            .map(|details| {
                format!(
                    "state: {} / {} / {}",
                    if details.is_favorite {
                        "favorite"
                    } else {
                        "not favorite"
                    },
                    if details.is_low_priority {
                        "low priority"
                    } else {
                        "normal priority"
                    },
                    if details.is_marked_unread {
                        "marked unread"
                    } else {
                        "read"
                    },
                )
            })
            .unwrap_or_else(|| "state: room list details not loaded yet".to_string());
        self.view
            .label(cx, ids!(telegram_room_info_strip.info_state))
            .set_text(cx, &state_text);
    }

    fn set_telegram_room_info_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_room_info_visible == visible {
            return;
        }
        self.telegram_room_info_visible = visible;
        self.view
            .view(cx, ids!(telegram_room_info_strip))
            .set_visible(cx, visible);
    }

    fn update_telegram_room_settings_strip(&mut self, cx: &mut Cx, room_label: &str) {
        let current_status = if self.telegram_room_settings_local_status.trim().is_empty() {
            "Name/topic writes are confirmation-gated; identity, permissions, and members are read-only"
        } else {
            self.telegram_room_settings_local_status.as_str()
        };
        let identity_text = self.telegram_room_settings_loaded_identity_summary();
        let power_text = self
            .tl_state
            .as_ref()
            .map(|tl_state| {
                let user_power = tl_state.user_power;
                format!(
                    "{}: send {}, react {}, @room {}.",
                    ROOM_POWER_LEVELS_COMPACT_LABEL,
                    if user_power.can_send_message() {
                        "allowed"
                    } else {
                        "blocked"
                    },
                    if user_power.can_send_reaction() {
                        "allowed"
                    } else {
                        "blocked"
                    },
                    if user_power.can_notify_room() {
                        "allowed"
                    } else {
                        "blocked"
                    },
                )
            })
            .unwrap_or_else(|| format!("{ROOM_POWER_LEVELS_COMPACT_LABEL}: waiting."));
        self.view
            .label(
                cx,
                ids!(telegram_room_settings_strip.settings_header.settings_title),
            )
            .set_text(cx, "Room settings");
        self.view
            .label(
                cx,
                ids!(telegram_room_settings_strip.settings_header.settings_status),
            )
            .set_text(cx, "partial live");
        self.view
            .label(cx, ids!(telegram_room_settings_strip.settings_summary))
            .set_text(
                cx,
                &format!("{room_label}: {current_status}. {ROOM_SETTINGS_COMPACT_LABEL}"),
            );
        self.view
            .label(cx, ids!(telegram_room_settings_strip.settings_power_levels))
            .set_text(cx, &power_text);
        self.view
            .label(cx, ids!(telegram_room_settings_strip.settings_identity))
            .set_text(cx, &identity_text);
        let refresh_result_detail =
            self.telegram_room_settings_refresh_result_detail_summary(room_label, None);
        self.view
            .label(
                cx,
                ids!(telegram_room_settings_strip.settings_refresh_result_detail),
            )
            .set_text(cx, &refresh_result_detail);
        let mutation_preflight_detail =
            self.telegram_room_settings_mutation_preflight_detail_summary(room_label, None);
        self.view
            .label(
                cx,
                ids!(telegram_room_settings_strip.settings_mutation_preflight_detail),
            )
            .set_text(cx, &mutation_preflight_detail);
        self.view
            .label(
                cx,
                ids!(telegram_room_settings_strip.settings_option_evidence),
            )
            .set_text(cx, ROOM_SETTINGS_COMPACT_LABEL);
        let edit_boundary = self.telegram_room_settings_edit_controls_boundary_summary(room_label);
        self.view
            .label(
                cx,
                ids!(telegram_room_settings_strip.settings_edit_controls_boundary),
            )
            .set_text(cx, &edit_boundary);
    }

    fn telegram_room_settings_loaded_identity_summary(&self) -> String {
        let room_state = self
            .room_name_id
            .as_ref()
            .map(|room_name_id| {
                if room_name_id.is_empty() {
                    "id only"
                } else {
                    "name/id loaded"
                }
            })
            .unwrap_or("room id waiting");
        let member_state = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| format!("members {}", members.len()))
            .unwrap_or_else(|| "members waiting".to_string());
        let Some(details) = self.telegram_room_action_details.as_ref() else {
            return format!("Identity: {room_state}; room-list metadata waiting; {member_state}.");
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
            "Identity: {room_state}; {alias_state}; alts {}; {avatar_state}; {tombstone_state}; {member_state}.",
            details.alt_alias_count
        )
    }

    fn copy_telegram_room_settings_name_id(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        room_id: &str,
    ) {
        let payload = room_settings_name_id_clipboard_payload(room_label, room_id);
        let copied = if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
            true
        } else {
            false
        };
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_levels_loaded = self.tl_state.is_some();
        let label = room_settings_name_id_clipboard_label(
            copied,
            room_label,
            room_id,
            identity_loaded,
            member_count,
            power_levels_loaded,
        );
        self.telegram_room_settings_local_status = label.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(
            if copied {
                label
            } else {
                format!(
                    "Room settings Name clipboard unavailable: loaded room id is missing. {ROOM_SETTINGS_NAME_ID_CLIPBOARD_LABEL}"
                )
            },
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn copy_telegram_room_settings_permissions(&mut self, cx: &mut Cx, room_label: &str) {
        let permissions = self.tl_state.as_ref().map(|tl_state| {
            let user_power = tl_state.user_power;
            (
                user_power.can_send_message(),
                user_power.can_send_reaction(),
                user_power.can_notify_room(),
            )
        });
        let payload = permissions.map(|(can_send, can_react, can_notify_room)| {
            room_settings_permissions_clipboard_payload(can_send, can_react, can_notify_room)
        });
        let copied = if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
            true
        } else {
            false
        };
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let label = room_settings_permissions_clipboard_label(
            copied,
            room_label,
            permissions,
            identity_loaded,
            member_count,
        );
        self.telegram_room_settings_local_status = label.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(
            if copied {
                label
            } else {
                format!(
                    "Room settings Permissions clipboard unavailable: loaded power levels are missing. {ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_LABEL}"
                )
            },
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn copy_telegram_room_settings_members(&mut self, cx: &mut Cx, room_label: &str) {
        let members = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref());
        let member_count = members.map(|members| members.len());
        let member_preview = members
            .map(|members| room_settings_members_cache_preview(members.as_slice()))
            .unwrap_or_else(|| "member cache waiting".to_string());
        let payload = member_count
            .map(|count| room_settings_members_clipboard_payload(count, &member_preview));
        let copied = if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
            true
        } else {
            false
        };
        let identity_loaded = self.telegram_room_action_details.is_some();
        let power_levels_loaded = self.tl_state.is_some();
        let label = room_settings_members_clipboard_label(
            copied,
            room_label,
            member_count,
            &member_preview,
            identity_loaded,
            power_levels_loaded,
        );
        self.telegram_room_settings_local_status = label.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(
            if copied {
                label
            } else {
                format!(
                    "Room settings Members clipboard unavailable: local member cache is missing. {ROOM_SETTINGS_MEMBERS_CLIPBOARD_LABEL}"
                )
            },
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn copy_telegram_room_settings_identity(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        room_id: &str,
    ) {
        self.refresh_telegram_room_action_details(cx);
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let identity = self.telegram_room_action_details.as_ref().map(|details| {
            (
                details.canonical_alias.as_ref().map(ToString::to_string),
                details.alt_alias_count,
                details.room_avatar_loaded,
                details.is_tombstoned,
            )
        });
        let payload = identity.as_ref().and_then(
            |(canonical_alias, alt_alias_count, room_avatar_loaded, is_tombstoned)| {
                room_settings_identity_clipboard_payload(
                    room_label,
                    room_id,
                    canonical_alias.as_deref(),
                    *alt_alias_count,
                    *room_avatar_loaded,
                    *is_tombstoned,
                    member_count,
                )
            },
        );
        let copied = if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
            true
        } else {
            false
        };
        let canonical_alias = identity.as_ref().and_then(|identity| identity.0.as_deref());
        let alt_alias_count = identity.as_ref().map(|identity| identity.1);
        let room_avatar_loaded = identity.as_ref().map(|identity| identity.2);
        let is_tombstoned = identity.as_ref().map(|identity| identity.3);
        let label = room_settings_identity_clipboard_label(
            copied,
            room_label,
            room_id,
            canonical_alias,
            alt_alias_count,
            room_avatar_loaded,
            is_tombstoned,
            member_count,
        );
        self.telegram_room_settings_local_status = label.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(
            if copied {
                label
            } else {
                format!(
                    "Room settings Identity clipboard unavailable: loaded room-list identity metadata is missing. {ROOM_SETTINGS_IDENTITY_CLIPBOARD_LABEL}"
                )
            },
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn telegram_room_settings_topic_summary(&self) -> String {
        "Topic read-only: not loaded in this RoomScreen cache yet; no m.room.topic request or mutation"
        .to_string()
    }

    fn telegram_room_settings_edit_controls_boundary_summary(&self, room_label: &str) -> String {
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_levels_loaded = self.tl_state.is_some();
        room_settings_edit_controls_boundary_label(
            room_label,
            identity_loaded,
            member_count,
            power_levels_loaded,
        )
    }

    fn telegram_room_settings_edit_intent_summary(
        &self,
        room_label: &str,
        edit_intent: &str,
    ) -> String {
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_levels_loaded = self.tl_state.is_some();
        room_settings_edit_intent_staging_label(
            room_label,
            edit_intent,
            identity_loaded,
            member_count,
            power_levels_loaded,
        )
    }

    fn telegram_room_settings_field_edit_intent_summary(
        &self,
        room_label: &str,
        field_intent: &str,
    ) -> String {
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_levels_loaded = self.tl_state.is_some();
        room_settings_field_edit_intent_controls_label(
            room_label,
            field_intent,
            identity_loaded,
            member_count,
            power_levels_loaded,
        )
    }

    fn telegram_room_settings_power_result_summary(&self) -> String {
        self.tl_state
            .as_ref()
            .map(|tl_state| {
                let user_power = tl_state.user_power;
                format!(
                    "power result send {}, react {}, @room {}",
                    if user_power.can_send_message() {
                        "allowed"
                    } else {
                        "blocked"
                    },
                    if user_power.can_send_reaction() {
                        "allowed"
                    } else {
                        "blocked"
                    },
                    if user_power.can_notify_room() {
                        "allowed"
                    } else {
                        "blocked"
                    },
                )
            })
            .unwrap_or_else(|| "power result waiting".to_string())
    }

    fn telegram_room_settings_refresh_result_detail_summary(
        &self,
        room_label: &str,
        action: Option<&str>,
    ) -> String {
        let action = action.unwrap_or(self.telegram_room_settings_refresh_detail_action.as_str());
        let timeline_loaded = self.tl_state.is_some();
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_summary = self.telegram_room_settings_power_result_summary();
        let local_status_chars = self
            .telegram_room_settings_local_status
            .trim()
            .chars()
            .count();
        room_settings_refresh_result_detail_label(
            room_label,
            Some(action),
            timeline_loaded,
            identity_loaded,
            member_count,
            &power_summary,
            local_status_chars,
        )
    }

    fn telegram_room_settings_mutation_preflight_detail_summary(
        &self,
        room_label: &str,
        action: Option<&str>,
    ) -> String {
        let action = action.unwrap_or(
            self.telegram_room_settings_mutation_preflight_action
                .as_str(),
        );
        let timeline_loaded = self.tl_state.is_some();
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_summary = self.telegram_room_settings_power_result_summary();
        let local_status_chars = self
            .telegram_room_settings_local_status
            .trim()
            .chars()
            .count();
        if action.trim() == "Request" {
            room_settings_mutation_request_packet_snapshot_label(
                room_label,
                timeline_loaded,
                identity_loaded,
                member_count,
                &power_summary,
                local_status_chars,
            )
        } else if action.trim() == "Packet" {
            room_settings_field_mutation_packet_clipboard_label(
                true,
                room_label,
                timeline_loaded,
                identity_loaded,
                member_count,
                &power_summary,
                local_status_chars,
            )
        } else if action.trim() == "Contract" {
            room_settings_field_mutation_contract_packet_clipboard_label(
                true,
                room_label,
                timeline_loaded,
                identity_loaded,
                member_count,
                &power_summary,
                local_status_chars,
            )
        } else if action.trim() == "Taxonomy" {
            room_settings_power_member_result_taxonomy_packet_clipboard_label(
                true,
                room_label,
                timeline_loaded,
                identity_loaded,
                member_count,
                &power_summary,
                local_status_chars,
            )
        } else {
            room_settings_mutation_preflight_detail_label(
                room_label,
                Some(action),
                timeline_loaded,
                identity_loaded,
                member_count,
                &power_summary,
                local_status_chars,
            )
        }
    }

    fn copy_telegram_room_settings_field_mutation_packet(&mut self, cx: &mut Cx, room_label: &str) {
        let timeline_loaded = self.tl_state.is_some();
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_summary = self.telegram_room_settings_power_result_summary();
        let local_status = self.telegram_room_settings_local_status.clone();
        let last_preflight_action = self
            .telegram_room_settings_mutation_preflight_action
            .clone();
        let payload = room_settings_field_mutation_packet_payload(
            room_label,
            timeline_loaded,
            identity_loaded,
            member_count,
            &power_summary,
            &local_status,
            &last_preflight_action,
        );
        cx.copy_to_clipboard(&payload);
        self.telegram_room_settings_mutation_preflight_action = "Packet".to_string();
        let label = room_settings_field_mutation_packet_clipboard_label(
            true,
            room_label,
            timeline_loaded,
            identity_loaded,
            member_count,
            &power_summary,
            local_status.trim().chars().count(),
        );
        self.telegram_room_settings_local_status = label.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(label, PopupKind::Info, Some(4.0));
    }

    fn copy_telegram_room_settings_field_mutation_contract_packet(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
    ) {
        let timeline_loaded = self.tl_state.is_some();
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_summary = self.telegram_room_settings_power_result_summary();
        let local_status = self.telegram_room_settings_local_status.clone();
        let last_preflight_action = self
            .telegram_room_settings_mutation_preflight_action
            .clone();
        let payload = room_settings_field_mutation_contract_packet_payload(
            room_label,
            timeline_loaded,
            identity_loaded,
            member_count,
            &power_summary,
            &local_status,
            &last_preflight_action,
        );
        cx.copy_to_clipboard(&payload);
        self.telegram_room_settings_mutation_preflight_action = "Contract".to_string();
        let label = room_settings_field_mutation_contract_packet_clipboard_label(
            true,
            room_label,
            timeline_loaded,
            identity_loaded,
            member_count,
            &power_summary,
            local_status.trim().chars().count(),
        );
        self.telegram_room_settings_local_status = label.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(label, PopupKind::Info, Some(4.0));
    }

    fn copy_telegram_room_settings_power_member_result_taxonomy_packet(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
    ) {
        let timeline_loaded = self.tl_state.is_some();
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_summary = self.telegram_room_settings_power_result_summary();
        let local_status = self.telegram_room_settings_local_status.clone();
        let last_preflight_action = self
            .telegram_room_settings_mutation_preflight_action
            .clone();
        let payload = room_settings_power_member_result_taxonomy_packet_payload(
            room_label,
            timeline_loaded,
            identity_loaded,
            member_count,
            &power_summary,
            &local_status,
            &last_preflight_action,
        );
        cx.copy_to_clipboard(&payload);
        self.telegram_room_settings_mutation_preflight_action = "Taxonomy".to_string();
        let label = room_settings_power_member_result_taxonomy_packet_clipboard_label(
            true,
            room_label,
            timeline_loaded,
            identity_loaded,
            member_count,
            &power_summary,
            local_status.trim().chars().count(),
        );
        self.telegram_room_settings_local_status = label.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(label, PopupKind::Info, Some(4.0));
    }

    fn telegram_room_settings_close_metadata_summary(&self, room_label: &str) -> String {
        let option_staged = !self.telegram_room_settings_local_status.trim().is_empty();
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_levels_loaded = self.tl_state.is_some();
        room_settings_close_metadata_label(
            room_label,
            option_staged,
            identity_loaded,
            member_count,
            power_levels_loaded,
        )
    }

    fn telegram_room_settings_refresh_metadata_summary(&self, room_label: &str) -> String {
        let timeline_loaded = self.tl_state.is_some();
        let identity_loaded = self.telegram_room_action_details.is_some();
        let member_count = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_members.as_ref())
            .map(|members| members.len());
        let power_levels_loaded = self.tl_state.is_some();
        room_settings_refresh_metadata_label(
            room_label,
            timeline_loaded,
            identity_loaded,
            member_count,
            power_levels_loaded,
        )
    }

    fn refresh_telegram_room_settings_read_paths(&mut self, cx: &mut Cx, room_label: &str) {
        self.telegram_room_settings_refresh_detail_action = "Refresh".to_string();
        self.refresh_telegram_room_action_details(cx);
        if let Some(tl_state) = self.tl_state.as_ref() {
            let timeline_kind = tl_state.kind.clone();
            submit_async_request(MatrixRequest::GetRoomPowerLevels {
                timeline_kind: timeline_kind.clone(),
            });
            submit_async_request(MatrixRequest::GetRoomMembers {
                timeline_kind,
                memberships: matrix_sdk::RoomMemberships::JOIN,
                local_only: false,
            });
        }
        let metadata = self.telegram_room_settings_refresh_metadata_summary(room_label);
        self.telegram_room_settings_local_status = metadata.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn update_telegram_room_settings_live_inputs(&mut self, cx: &mut Cx) {
        self.view
            .text_input(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_name_write_row
                        .name_live_input
                ),
            )
            .set_text(cx, &self.telegram_room_settings_name_draft);
        self.view
            .text_input(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_topic_write_row
                        .topic_live_input
                ),
            )
            .set_text(cx, &self.telegram_room_settings_topic_draft);
        self.view
            .text_input(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_alias_write_row
                        .alias_live_input
                ),
            )
            .set_text(cx, &self.telegram_room_settings_alias_draft);
        self.view
            .text_input(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_tombstone_write_row
                        .tombstone_replacement_live_input
                ),
            )
            .set_text(cx, &self.telegram_room_settings_tombstone_replacement_draft);
    }

    fn open_telegram_room_settings_avatar_upload_picker(&mut self, cx: &mut Cx, room_label: &str) {
        let Some(timeline_kind) = self.tl_state.as_ref().map(|tl_state| tl_state.kind.clone())
        else {
            let status = room_avatar_upload_lifecycle_label(
                room_label,
                "blocked locally: no loaded timeline is available",
                None,
            );
            self.telegram_room_settings_local_status = status.clone();
            self.update_telegram_room_settings_strip(cx, room_label);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
            return;
        };

        self.telegram_room_settings_mutation_preflight_action = "Avatar upload".to_string();
        let opened = room_avatar_upload_lifecycle_label(room_label, "picker opened", None);
        self.telegram_room_settings_local_status = opened.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        enqueue_popup_notification(opened, PopupKind::Info, Some(3.0));

        match pick_room_avatar_upload_file() {
            RoomAvatarUploadPickResult::Picked(file_path) => {
                let mime_type = room_avatar_upload_mime_type(&file_path);
                if let Err(reason) = validate_room_avatar_upload_file(&file_path, &mime_type) {
                    let summary = room_avatar_upload_selection_summary(&file_path, &mime_type);
                    let status = room_avatar_upload_lifecycle_label(
                        room_label,
                        &format!("invalid selection held locally: {reason}"),
                        Some(&summary),
                    );
                    self.telegram_room_settings_local_status = status.clone();
                    self.update_telegram_room_settings_strip(cx, room_label);
                    enqueue_popup_notification(status, PopupKind::Error, Some(4.0));
                    return;
                }

                let selected_summary = room_avatar_upload_selection_summary(&file_path, &mime_type);
                self.show_telegram_room_settings_avatar_upload_confirmation(
                    cx,
                    room_label,
                    timeline_kind,
                    file_path,
                    mime_type,
                    selected_summary,
                );
            }
            RoomAvatarUploadPickResult::Canceled => {
                let status = room_avatar_upload_lifecycle_label(
                    room_label,
                    "picker canceled; no UploadRoomAvatar was requested",
                    None,
                );
                self.telegram_room_settings_local_status = status.clone();
                self.update_telegram_room_settings_strip(cx, room_label);
                enqueue_popup_notification(status, PopupKind::Info, Some(3.0));
            }
            RoomAvatarUploadPickResult::Unsupported => {
                let status = room_avatar_upload_lifecycle_label(
                    room_label,
                    "picker unsupported on this platform",
                    None,
                );
                self.telegram_room_settings_local_status = status.clone();
                self.update_telegram_room_settings_strip(cx, room_label);
                enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
            }
        }
    }

    fn show_telegram_room_settings_avatar_upload_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        timeline_kind: TimelineKind,
        file_path: PathBuf,
        mime_type: mime::Mime,
        selected_summary: String,
    ) {
        let room_label_for_accept = room_label.to_string();
        let room_label_for_cancel = room_label.to_string();
        let file_path_for_request = file_path.clone();
        let mime_type_for_request = mime_type.clone();
        let selected_summary_for_accept = selected_summary.clone();
        let selected_summary_for_cancel = selected_summary.clone();
        let content = ConfirmationModalContent {
        title_text: "Upload Room Avatar".into(),
        body_text: format!(
            "Upload selected image as the room avatar for {room_label}? Selected image: {selected_summary}. {ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL}"
        )
        .into(),
        accept_button_text: Some("Upload".into()),
        cancel_button_text: Some("Cancel".into()),
        on_accept_clicked: Some(Box::new(move |_cx| {
            submit_async_request(MatrixRequest::UploadRoomAvatar {
                timeline_kind,
                file_path: file_path_for_request.clone(),
                mime_type: mime_type_for_request.clone(),
            });
            enqueue_popup_notification(
                room_avatar_upload_lifecycle_label(
                    &room_label_for_accept,
                    "confirmed; MatrixRequest::UploadRoomAvatar was requested",
                    Some(&selected_summary_for_accept),
                ),
                PopupKind::Info,
                Some(4.0),
            );
        })),
        on_cancel_clicked: Some(Box::new(move |_cx| {
            enqueue_popup_notification(
                room_avatar_upload_lifecycle_label(
                    &room_label_for_cancel,
                    "confirmation canceled; UploadRoomAvatar was not requested",
                    Some(&selected_summary_for_cancel),
                ),
                PopupKind::Info,
                Some(3.0),
            );
        })),
        ..Default::default()
    };
        let preserving_avatar_retry = self.telegram_room_settings_retry_field
            == Some(RoomSettingsMutationField::Avatar)
            && room_settings_avatar_upload_value(&self.telegram_room_settings_retry_value);
        if !preserving_avatar_retry {
            self.telegram_room_settings_retry_field = None;
            self.telegram_room_settings_retry_value.clear();
        }
        self.telegram_room_settings_retry_avatar_file_path = Some(file_path);
        self.telegram_room_settings_retry_avatar_mime_type = Some(mime_type);
        self.telegram_room_settings_mutation_preflight_action =
            "Avatar upload confirmation".to_string();
        self.telegram_room_settings_local_status = room_avatar_upload_lifecycle_label(
            room_label,
            "confirmation opened",
            Some(&selected_summary),
        );
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            room_avatar_upload_lifecycle_label(
                room_label,
                "confirmation opened",
                Some(&selected_summary),
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn submit_telegram_room_settings_live_write(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        field: RoomSettingsMutationField,
        value: String,
    ) {
        let value = value.trim().to_string();
        if value.is_empty() {
            let status = room_settings_live_write_validation_label(field, true);
            self.telegram_room_settings_local_status = status.clone();
            self.update_telegram_room_settings_strip(cx, room_label);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
            return;
        }
        let Some(timeline_kind) = self.tl_state.as_ref().map(|tl_state| tl_state.kind.clone())
        else {
            let status = room_settings_live_write_validation_label(field, false);
            self.telegram_room_settings_local_status = status.clone();
            self.update_telegram_room_settings_strip(cx, room_label);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
            return;
        };

        if field == RoomSettingsMutationField::Avatar {
            self.refresh_telegram_room_action_details(cx);
            let avatar_loaded = self
                .telegram_room_action_details
                .as_ref()
                .map(|details| details.room_avatar_loaded)
                .unwrap_or(false);
            if !avatar_loaded {
                let status = room_settings_live_write_avatar_validation_label(room_label);
                self.telegram_room_settings_local_status = status.clone();
                self.update_telegram_room_settings_strip(cx, room_label);
                enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
                return;
            }
        }

        if field == RoomSettingsMutationField::Tombstone {
            if let Err(error) = OwnedRoomId::try_from(value.as_str()) {
                let status = room_settings_tombstone_replacement_validation_label(
                    room_label,
                    &value,
                    &error.to_string(),
                );
                self.telegram_room_settings_local_status = status.clone();
                self.update_telegram_room_settings_strip(cx, room_label);
                enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
                return;
            }
        }

        let canonical_alias_alt_aliases = if field == RoomSettingsMutationField::CanonicalAlias {
            self.refresh_telegram_room_action_details(cx);
            self.telegram_room_action_details
                .as_ref()
                .map(|details| details.alt_aliases.clone())
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        self.show_telegram_room_settings_mutation_confirmation(
            cx,
            room_label,
            timeline_kind,
            field,
            value,
            canonical_alias_alt_aliases,
        );
    }

    fn show_telegram_room_settings_mutation_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        timeline_kind: TimelineKind,
        field: RoomSettingsMutationField,
        value: String,
        canonical_alias_alt_aliases: Vec<OwnedRoomAliasId>,
    ) {
        let field_label = field.label();
        let event_type = field.matrix_event_type();
        let room_label_for_accept = room_label.to_string();
        let room_label_for_cancel = room_label.to_string();
        let value_for_accept = value.clone();
        let value_for_cancel = value.clone();
        let canonical_alias_alt_aliases_for_accept = canonical_alias_alt_aliases.clone();
        let content = ConfirmationModalContent {
            title_text: format!("Confirm Room {field_label}").into(),
            body_text: room_settings_live_write_confirmation_label(room_label, field, &value)
                .into(),
            accept_button_text: Some(format!("Save {field_label}").into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                match field {
                    RoomSettingsMutationField::Name => {
                        submit_async_request(MatrixRequest::SetRoomName {
                            timeline_kind,
                            name: value_for_accept.clone(),
                        });
                    }
                    RoomSettingsMutationField::Topic => {
                        submit_async_request(MatrixRequest::SetRoomTopic {
                            timeline_kind,
                            topic: value_for_accept.clone(),
                        });
                    }
                    RoomSettingsMutationField::Avatar => {
                        submit_async_request(MatrixRequest::RemoveRoomAvatar { timeline_kind });
                    }
                    RoomSettingsMutationField::CanonicalAlias => {
                        submit_async_request(MatrixRequest::SetRoomCanonicalAlias {
                            timeline_kind,
                            alias: value_for_accept.clone(),
                            alt_aliases: canonical_alias_alt_aliases_for_accept.clone(),
                        });
                    }
                    RoomSettingsMutationField::HistoryVisibility => {
                        submit_async_request(MatrixRequest::SetRoomHistoryVisibility {
                            timeline_kind,
                            visibility: value_for_accept.clone(),
                        });
                    }
                    RoomSettingsMutationField::JoinRule => {
                        submit_async_request(MatrixRequest::SetRoomJoinRule {
                            timeline_kind,
                            join_rule: value_for_accept.clone(),
                        });
                    }
                    RoomSettingsMutationField::Tombstone => {
                        match OwnedRoomId::try_from(value_for_accept.as_str()) {
                            Ok(replacement_room_id) => {
                                submit_async_request(MatrixRequest::SetRoomTombstone {
                                    timeline_kind,
                                    replacement_room_id,
                                    reason: room_settings_tombstone_body(
                                        &room_label_for_accept,
                                        &value_for_accept,
                                    ),
                                });
                            }
                            Err(error) => {
                                enqueue_popup_notification(
                                    room_settings_tombstone_replacement_validation_label(
                                        &room_label_for_accept,
                                        &value_for_accept,
                                        &error.to_string(),
                                    ),
                                    PopupKind::Warning,
                                    Some(4.0),
                                );
                                return;
                            }
                        }
                    }
                }
                enqueue_popup_notification(
                    format!(
                        "Room {field_label} update requested for {room_label_for_accept}: {event_type}."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Room {field_label} update canceled for {room_label_for_cancel}: `{}`.",
                        compact_message_preview(&value_for_cancel, "empty")
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        self.telegram_room_settings_retry_field = None;
        self.telegram_room_settings_retry_value.clear();
        self.telegram_room_settings_retry_avatar_file_path = None;
        self.telegram_room_settings_retry_avatar_mime_type = None;
        self.telegram_room_settings_retry_canonical_alias_alt_aliases =
            if field == RoomSettingsMutationField::CanonicalAlias {
                canonical_alias_alt_aliases
            } else {
                Vec::new()
            };
        self.telegram_room_settings_mutation_preflight_action =
            format!("{field_label} confirmation");
        self.telegram_room_settings_local_status = format!(
            "Confirmation open: {field_label} -> {}",
            compact_message_preview(&value, "")
        );
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            format!(
                "Room {field_label} confirmation opened for {room_label}. {ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL}"
            ),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn show_telegram_room_settings_mutation_retry_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
    ) -> bool {
        let Some(field) = self.telegram_room_settings_retry_field else {
            return false;
        };
        let value = self.telegram_room_settings_retry_value.clone();
        let Some(timeline_kind) = self.tl_state.as_ref().map(|tl_state| tl_state.kind.clone())
        else {
            enqueue_popup_notification(
                "Room settings retry unavailable: no loaded timeline for the failed write.",
                PopupKind::Warning,
                Some(4.0),
            );
            return true;
        };
        if field == RoomSettingsMutationField::Avatar
            && room_settings_avatar_upload_value(&value)
            && let (Some(file_path), Some(mime_type)) = (
                self.telegram_room_settings_retry_avatar_file_path.clone(),
                self.telegram_room_settings_retry_avatar_mime_type.clone(),
            )
        {
            let selected_summary = room_avatar_upload_selection_summary(&file_path, &mime_type);
            self.show_telegram_room_settings_avatar_upload_confirmation(
                cx,
                room_label,
                timeline_kind,
                file_path,
                mime_type,
                selected_summary,
            );
            return true;
        }
        let canonical_alias_alt_aliases = if field == RoomSettingsMutationField::CanonicalAlias {
            self.telegram_room_settings_retry_canonical_alias_alt_aliases
                .clone()
        } else {
            Vec::new()
        };
        self.show_telegram_room_settings_mutation_confirmation(
            cx,
            room_label,
            timeline_kind,
            field,
            value,
            canonical_alias_alt_aliases,
        );
        true
    }

    fn update_telegram_room_settings_mutation_result(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        field: RoomSettingsMutationField,
        value: String,
        result: Result<(), String>,
    ) {
        let status = room_settings_live_write_result_label(room_label, field, &value, &result);
        match result {
            Ok(()) => {
                self.telegram_room_settings_retry_field = None;
                self.telegram_room_settings_retry_value.clear();
                self.telegram_room_settings_retry_avatar_file_path = None;
                self.telegram_room_settings_retry_avatar_mime_type = None;
                self.telegram_room_settings_retry_canonical_alias_alt_aliases
                    .clear();
                enqueue_popup_notification(status.clone(), PopupKind::Success, Some(4.0));
            }
            Err(_) => {
                self.telegram_room_settings_retry_field = Some(field);
                if field != RoomSettingsMutationField::Avatar
                    || !room_settings_avatar_upload_value(&value)
                {
                    self.telegram_room_settings_retry_avatar_file_path = None;
                    self.telegram_room_settings_retry_avatar_mime_type = None;
                }
                if field != RoomSettingsMutationField::CanonicalAlias {
                    self.telegram_room_settings_retry_canonical_alias_alt_aliases
                        .clear();
                }
                self.telegram_room_settings_retry_value = value;
                enqueue_popup_notification(status.clone(), PopupKind::Error, Some(6.0));
            }
        }
        self.telegram_room_settings_mutation_preflight_action = "Result".to_string();
        self.telegram_room_settings_local_status = status;
        self.update_telegram_room_settings_strip(cx, room_label);
        if self.telegram_room_settings_visible {
            self.set_telegram_room_settings_visible(cx, true);
        }
    }

    fn stage_telegram_room_settings_refresh_result_detail(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        action: &str,
    ) {
        self.telegram_room_settings_refresh_detail_action = action.to_string();
        let detail =
            self.telegram_room_settings_refresh_result_detail_summary(room_label, Some(action));
        self.telegram_room_settings_local_status = detail.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(detail, PopupKind::Info, Some(4.0));
    }

    fn stage_telegram_room_settings_mutation_preflight_detail(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        action: &str,
    ) {
        self.telegram_room_settings_mutation_preflight_action = action.to_string();
        let detail =
            self.telegram_room_settings_mutation_preflight_detail_summary(room_label, Some(action));
        self.telegram_room_settings_local_status = detail.clone();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(detail, PopupKind::Info, Some(4.0));
    }

    fn set_telegram_room_settings_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_room_settings_visible == visible {
            return;
        }
        self.telegram_room_settings_visible = visible;
        self.view
            .view(cx, ids!(telegram_room_settings_strip))
            .set_visible(cx, visible);
    }

    fn show_telegram_room_settings_surface(&mut self, cx: &mut Cx, room_label: &str) {
        self.refresh_telegram_room_action_details(cx);
        self.telegram_room_settings_name_draft = self
            .room_name_id
            .as_ref()
            .and_then(|room_name_id| room_name_id.name_for_avatar())
            .unwrap_or_default()
            .to_string();
        self.telegram_room_settings_topic_draft.clear();
        self.telegram_room_settings_alias_draft = self
            .telegram_room_action_details
            .as_ref()
            .and_then(|details| details.canonical_alias.as_ref())
            .map(ToString::to_string)
            .unwrap_or_default();
        self.telegram_room_settings_tombstone_replacement_draft
            .clear();
        self.update_telegram_room_settings_live_inputs(cx);
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_message_edit_history_visible(cx, false);
        self.set_telegram_message_report_status_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, false, None);
        self.set_telegram_notifications_visible(cx, false);
        self.set_telegram_room_info_visible(cx, false);
        self.set_telegram_matrix_link_preview_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, true);
    }

    fn stage_telegram_room_settings_choice(&mut self, cx: &mut Cx, room_label: &str, status: &str) {
        self.telegram_room_settings_local_status = status.to_string();
        self.update_telegram_room_settings_strip(cx, room_label);
        self.set_telegram_room_settings_visible(cx, true);
        enqueue_popup_notification(
            format!("{status} staged for {room_label}. {ROOM_SETTINGS_COMPACT_LABEL}"),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn update_telegram_message_edit_history_strip(&mut self, cx: &mut Cx) {
        self.view
            .label(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_header
                        .edit_history_title
                ),
            )
            .set_text(cx, "Edit history");
        self.view
            .label(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_header
                        .edit_history_status
                ),
            )
            .set_text(cx, "read only");
        let retry_visible = self
            .telegram_message_edit_history_summary
            .starts_with("Edit history lookup failed")
            && self.telegram_message_edit_history_retry_event_id.is_some()
            && self
                .telegram_message_edit_history_retry_timeline_kind
                .is_some();
        self.view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_header
                        .retry_edit_history_button
                ),
            )
            .set_visible(cx, retry_visible);
        self.view
            .label(
                cx,
                ids!(telegram_message_edit_history_strip.edit_history_summary),
            )
            .set_text(cx, &self.telegram_message_edit_history_summary);
        self.view
            .label(
                cx,
                ids!(telegram_message_edit_history_strip.edit_history_diff),
            )
            .set_text(cx, &self.telegram_message_edit_history_diff);
        self.view
            .label(
                cx,
                ids!(telegram_message_edit_history_strip.edit_history_metadata),
            )
            .set_text(cx, &self.telegram_message_edit_history_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_message_edit_history_strip.edit_history_loaded_diff_detail),
            )
            .set_text(cx, &self.telegram_message_edit_history_loaded_diff_detail);
        self.view
            .label(
                cx,
                ids!(telegram_message_edit_history_strip.edit_history_preflight_detail),
            )
            .set_text(cx, &self.telegram_message_edit_history_preflight_detail);
        self.view
            .label(
                cx,
                ids!(telegram_message_edit_history_strip.edit_history_full_modal_boundary),
            )
            .set_text(cx, &self.telegram_message_edit_history_full_boundary);
    }

    fn refresh_telegram_message_edit_history_loaded_diff_detail(
        &mut self,
        control: Option<&str>,
        retry_cache_ready: bool,
    ) {
        let retry_cache_ready = retry_cache_ready
            || (self.telegram_message_edit_history_retry_event_id.is_some()
                && self
                    .telegram_message_edit_history_retry_timeline_kind
                    .is_some());
        self.telegram_message_edit_history_loaded_diff_detail =
            edit_history_loaded_diff_detail_label(
                control,
                &self.telegram_message_edit_history_loaded_event_id,
                self.telegram_message_edit_history_replacement_count,
                &self.telegram_message_edit_history_latest_event,
                self.telegram_message_edit_history_latest_timestamp.clone(),
                &self.telegram_message_edit_history_loaded_original_preview,
                &self.telegram_message_edit_history_latest_preview,
                retry_cache_ready,
            );
    }

    fn refresh_telegram_message_edit_history_preflight_detail(&mut self, control: Option<&str>) {
        let retry_cache_ready = self.telegram_message_edit_history_retry_event_id.is_some()
            && self
                .telegram_message_edit_history_retry_timeline_kind
                .is_some();
        self.telegram_message_edit_history_preflight_detail = edit_history_preflight_detail_label(
            control,
            &self.telegram_message_edit_history_loaded_event_id,
            self.telegram_message_edit_history_replacement_count,
            &self.telegram_message_edit_history_latest_event,
            self.telegram_message_edit_history_latest_timestamp.clone(),
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
            &self.telegram_message_edit_history_result_error,
            retry_cache_ready,
            &self.telegram_message_edit_history_metadata,
            &self.telegram_message_edit_history_full_boundary,
        );
    }

    fn open_telegram_message_edit_history_local_full_snapshot_modal(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self.telegram_message_edit_history_retry_event_id.is_some()
            && self
                .telegram_message_edit_history_retry_timeline_kind
                .is_some();
        let loaded_source_available = self.telegram_message_edit_history_loaded_source_available();
        let snapshot_json = edit_history_local_full_snapshot_modal_json(
            &self.telegram_message_edit_history_loaded_event_id,
            self.telegram_message_edit_history_replacement_count,
            self.telegram_message_edit_history_pages_fetched,
            self.telegram_message_edit_history_pagination_exhausted,
            &self.telegram_message_edit_history_latest_event,
            self.telegram_message_edit_history_latest_timestamp.clone(),
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
            &self.telegram_message_edit_history_latest_source_json,
            &self.telegram_message_edit_history_result_error,
            retry_cache_ready,
            loaded_source_available,
        );
        let room_id = self.tl_state.as_ref().map(|tl| tl.kind.room_id().clone());
        let target_event_id =
            EventId::parse(self.telegram_message_edit_history_loaded_event_id.trim()).ok();
        let modal_opened = if let Some(room_id) = room_id {
            cx.action(super::event_source_modal::EventSourceModalAction::Open {
                room_id,
                event_id: target_event_id,
                latest_json: Some(snapshot_json.clone()),
            });
            true
        } else {
            false
        };
        self.refresh_telegram_message_edit_history_loaded_diff_detail(
            Some("Local full snapshot modal"),
            retry_cache_ready,
        );
        self.telegram_message_edit_history_full_boundary =
            edit_history_local_full_snapshot_modal_label(
                &self.telegram_message_edit_history_loaded_event_id,
                modal_opened,
                snapshot_json.len(),
                self.telegram_message_edit_history_replacement_count,
                self.telegram_message_edit_history_pages_fetched,
                self.telegram_message_edit_history_pagination_exhausted,
                retry_cache_ready,
                loaded_source_available,
            );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        let popup_kind = if modal_opened {
            PopupKind::Success
        } else {
            PopupKind::Warning
        };
        enqueue_popup_notification(
            self.telegram_message_edit_history_full_boundary.clone(),
            popup_kind,
            Some(5.0),
        );
    }

    fn stage_telegram_message_edit_history_full_control(&mut self, cx: &mut Cx, control: &str) {
        let status = if control == "Full history modal" {
            edit_history_local_full_snapshot_label(
                &self.telegram_message_edit_history_loaded_event_id,
                self.telegram_message_edit_history_replacement_count,
                &self.telegram_message_edit_history_latest_event,
                self.telegram_message_edit_history_latest_timestamp.clone(),
                &self.telegram_message_edit_history_loaded_original_preview,
                &self.telegram_message_edit_history_latest_preview,
                &self.telegram_message_edit_history_result_error,
                self.telegram_message_edit_history_retry_event_id.is_some()
                    && self
                        .telegram_message_edit_history_retry_timeline_kind
                        .is_some(),
            )
        } else {
            edit_history_full_control_boundary_label(control)
        };
        let detail_control = if control == "Full history modal" {
            "Local full snapshot"
        } else {
            control
        };
        self.refresh_telegram_message_edit_history_loaded_diff_detail(Some(detail_control), false);
        let detail = self
            .telegram_message_edit_history_loaded_diff_detail
            .clone();
        self.telegram_message_edit_history_full_boundary = status.clone();
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        enqueue_popup_notification(format!("{status} {detail}"), PopupKind::Info, Some(5.0));
    }

    fn telegram_message_edit_history_loaded_original_source_json(&self) -> Option<String> {
        let target_event_id = self.telegram_message_edit_history_loaded_event_id.trim();
        if target_event_id.is_empty() {
            return None;
        }
        self.tl_state.as_ref().and_then(|tl| {
            tl.items.iter().find_map(|item| {
                let event_tl_item = item.as_event()?;
                let loaded_event_id = event_tl_item.event_id()?;
                (loaded_event_id.as_str() == target_event_id).then(|| {
                    event_tl_item
                        .latest_json()
                        .and_then(|raw_event| serde_json::to_value(raw_event).ok())
                        .and_then(|value| serde_json::to_string_pretty(&value).ok())
                })?
            })
        })
    }

    fn copy_telegram_message_edit_history_loaded_diff(&mut self, cx: &mut Cx) {
        let payload = edit_history_loaded_diff_clipboard_payload(
            &self.telegram_message_edit_history_loaded_event_id,
            self.telegram_message_edit_history_replacement_count,
            &self.telegram_message_edit_history_latest_event,
            self.telegram_message_edit_history_latest_timestamp.clone(),
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
        );
        let copied = payload.is_some();
        if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
        }
        let original_source_json = self.telegram_message_edit_history_loaded_original_source_json();
        let side_by_side_snapshot = edit_history_loaded_side_by_side_diff_modal_json_with_sources(
            &self.telegram_message_edit_history_loaded_event_id,
            self.telegram_message_edit_history_replacement_count,
            self.telegram_message_edit_history_pages_fetched,
            self.telegram_message_edit_history_pagination_exhausted,
            &self.telegram_message_edit_history_latest_event,
            self.telegram_message_edit_history_latest_timestamp.clone(),
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
            original_source_json.as_deref(),
            &self.telegram_message_edit_history_latest_source_json,
        );
        let side_by_side_json_bytes = side_by_side_snapshot
            .as_ref()
            .map(|snapshot| snapshot.json.len());
        let loaded_full_body_diff = side_by_side_snapshot
            .as_ref()
            .is_some_and(|snapshot| snapshot.loaded_full_body);
        let modal_opened = if let (Some(room_id), Some(side_by_side_json)) = (
            self.tl_state.as_ref().map(|tl| tl.kind.room_id().clone()),
            side_by_side_snapshot
                .as_ref()
                .map(|snapshot| snapshot.json.as_str()),
        ) {
            let target_event_id =
                EventId::parse(self.telegram_message_edit_history_loaded_event_id.trim()).ok();
            cx.action(super::event_source_modal::EventSourceModalAction::Open {
                room_id,
                event_id: target_event_id,
                latest_json: Some(side_by_side_json.to_string()),
            });
            true
        } else {
            false
        };
        self.refresh_telegram_message_edit_history_loaded_diff_detail(
            Some("Loaded side-by-side preview diff"),
            false,
        );
        self.telegram_message_edit_history_full_boundary = edit_history_loaded_diff_clipboard_label(
            &self.telegram_message_edit_history_loaded_event_id,
            copied,
            modal_opened,
            side_by_side_json_bytes,
            loaded_full_body_diff,
            self.telegram_message_edit_history_replacement_count,
            &self.telegram_message_edit_history_latest_event,
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        let popup_kind = if copied && modal_opened {
            PopupKind::Success
        } else {
            PopupKind::Warning
        };
        enqueue_popup_notification(
            self.telegram_message_edit_history_full_boundary.clone(),
            popup_kind,
            Some(5.0),
        );
    }

    fn telegram_message_edit_history_loaded_source_available(&self) -> bool {
        if !self
            .telegram_message_edit_history_latest_source_json
            .trim()
            .is_empty()
            && !self
                .telegram_message_edit_history_latest_event
                .trim()
                .is_empty()
        {
            return true;
        }

        let target_event_id = self
            .telegram_message_edit_history_loaded_event_id
            .trim()
            .to_string();
        if target_event_id.is_empty() {
            return false;
        }
        let Some(tl) = self.tl_state.as_ref() else {
            return false;
        };
        tl.items
            .iter()
            .filter_map(|item| item.as_event())
            .any(|event| {
                event
                    .event_id()
                    .map(|loaded_event_id| {
                        loaded_event_id.as_str() == target_event_id.as_str()
                            && event.latest_json().is_some()
                    })
                    .unwrap_or(false)
            })
    }

    fn copy_telegram_message_edit_history_full_diff_packet(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self.telegram_message_edit_history_retry_event_id.is_some()
            && self
                .telegram_message_edit_history_retry_timeline_kind
                .is_some();
        let loaded_source_available = self.telegram_message_edit_history_loaded_source_available();
        self.refresh_telegram_message_edit_history_loaded_diff_detail(
            Some("Loaded/full diff Packet"),
            retry_cache_ready,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        let payload = edit_history_full_diff_packet_payload(
            &self.telegram_message_edit_history_loaded_event_id,
            self.telegram_message_edit_history_replacement_count,
            &self.telegram_message_edit_history_latest_event,
            self.telegram_message_edit_history_latest_timestamp.clone(),
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
            &self.telegram_message_edit_history_result_error,
            retry_cache_ready,
            loaded_source_available,
            &self.telegram_message_edit_history_loaded_diff_detail,
            &self.telegram_message_edit_history_preflight_detail,
            &self.telegram_message_edit_history_full_boundary,
        );
        cx.copy_to_clipboard(&payload);
        self.telegram_message_edit_history_full_boundary = edit_history_full_diff_packet_label(
            &self.telegram_message_edit_history_loaded_event_id,
            self.telegram_message_edit_history_replacement_count,
            retry_cache_ready,
            loaded_source_available,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(Some("Packet"));
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        enqueue_popup_notification(
            self.telegram_message_edit_history_full_boundary.clone(),
            PopupKind::Info,
            Some(5.0),
        );
    }

    fn copy_telegram_message_edit_history_full_history_result_contract_packet(
        &mut self,
        cx: &mut Cx,
    ) {
        let retry_cache_ready = self.telegram_message_edit_history_retry_event_id.is_some()
            && self
                .telegram_message_edit_history_retry_timeline_kind
                .is_some();
        let loaded_source_available = self.telegram_message_edit_history_loaded_source_available();
        self.refresh_telegram_message_edit_history_loaded_diff_detail(
            Some("Typed full-history Contract"),
            retry_cache_ready,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        let payload = edit_history_full_history_result_contract_packet_payload(
            &self.telegram_message_edit_history_loaded_event_id,
            self.telegram_message_edit_history_replacement_count,
            &self.telegram_message_edit_history_latest_event,
            self.telegram_message_edit_history_latest_timestamp.clone(),
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
            &self.telegram_message_edit_history_result_error,
            retry_cache_ready,
            loaded_source_available,
            &self.telegram_message_edit_history_loaded_diff_detail,
            &self.telegram_message_edit_history_preflight_detail,
            &self.telegram_message_edit_history_full_boundary,
        );
        cx.copy_to_clipboard(&payload);
        self.telegram_message_edit_history_full_boundary =
            edit_history_full_history_result_contract_packet_label(
                &self.telegram_message_edit_history_loaded_event_id,
                self.telegram_message_edit_history_replacement_count,
                retry_cache_ready,
                loaded_source_available,
            );
        self.refresh_telegram_message_edit_history_preflight_detail(Some("Contract"));
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        enqueue_popup_notification(
            self.telegram_message_edit_history_full_boundary.clone(),
            PopupKind::Info,
            Some(5.0),
        );
    }

    fn copy_telegram_message_edit_history_remote_result_taxonomy_packet(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self.telegram_message_edit_history_retry_event_id.is_some()
            && self
                .telegram_message_edit_history_retry_timeline_kind
                .is_some();
        let loaded_source_available = self.telegram_message_edit_history_loaded_source_available();
        self.refresh_telegram_message_edit_history_loaded_diff_detail(
            Some("Remote full-history Taxonomy"),
            retry_cache_ready,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        let payload = edit_history_remote_result_taxonomy_packet_payload(
            &self.telegram_message_edit_history_loaded_event_id,
            self.telegram_message_edit_history_replacement_count,
            self.telegram_message_edit_history_pages_fetched,
            self.telegram_message_edit_history_pagination_exhausted,
            &self.telegram_message_edit_history_latest_event,
            self.telegram_message_edit_history_latest_timestamp.clone(),
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
            &self.telegram_message_edit_history_result_error,
            retry_cache_ready,
            loaded_source_available,
            &self.telegram_message_edit_history_loaded_diff_detail,
            &self.telegram_message_edit_history_preflight_detail,
            &self.telegram_message_edit_history_full_boundary,
        );
        cx.copy_to_clipboard(&payload);
        self.telegram_message_edit_history_full_boundary =
            edit_history_remote_result_taxonomy_packet_label(
                &self.telegram_message_edit_history_loaded_event_id,
                self.telegram_message_edit_history_replacement_count,
                retry_cache_ready,
                loaded_source_available,
            );
        self.refresh_telegram_message_edit_history_preflight_detail(Some("Taxonomy"));
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        enqueue_popup_notification(
            self.telegram_message_edit_history_full_boundary.clone(),
            PopupKind::Info,
            Some(5.0),
        );
    }

    fn open_telegram_message_edit_history_loaded_source(&mut self, cx: &mut Cx) {
        let replacement_source_json = self
            .telegram_message_edit_history_latest_source_json
            .trim()
            .to_string();
        let replacement_source_detail = self.tl_state.as_ref().and_then(|tl| {
            if replacement_source_json.is_empty() {
                return None;
            }
            let event_id =
                EventId::parse(self.telegram_message_edit_history_latest_event.trim()).ok()?;
            Some((
                "latest replacement",
                tl.kind.room_id().clone(),
                event_id,
                replacement_source_json.clone(),
            ))
        });
        let target_event_id = self.telegram_message_edit_history_loaded_event_id.clone();
        let original_source_detail = self.tl_state.as_ref().and_then(|tl| {
            let target_event_id = target_event_id.trim();
            if target_event_id.is_empty() {
                return None;
            }
            tl.items.iter().find_map(|item| {
                let event_tl_item = item.as_event()?;
                let loaded_event_id = event_tl_item.event_id()?;
                (loaded_event_id.as_str() == target_event_id).then(|| {
                    let latest_json = event_tl_item
                        .latest_json()
                        .and_then(|raw_event| serde_json::to_value(raw_event).ok())
                        .and_then(|value| serde_json::to_string_pretty(&value).ok());
                    (
                        tl.kind.room_id().clone(),
                        loaded_event_id.to_owned(),
                        latest_json,
                    )
                })
            })
        });
        let source_detail = replacement_source_detail.or_else(|| {
            original_source_detail.and_then(|(room_id, event_id, latest_json)| {
                latest_json.map(|latest_json| ("loaded original", room_id, event_id, latest_json))
            })
        });
        let loaded_json = source_detail
            .as_ref()
            .map(|(_, _, _, latest_json)| latest_json.clone());
        let mut source_kind = source_detail
            .as_ref()
            .map(|(source_kind, _, _, _)| *source_kind)
            .unwrap_or("unavailable source")
            .to_string();
        let source_event_id = source_detail
            .as_ref()
            .map(|(_, _, event_id, _)| event_id.to_string())
            .unwrap_or_else(|| self.telegram_message_edit_history_loaded_event_id.clone());
        let source_opened = if let Some((_, room_id, event_id, latest_json)) = source_detail {
            cx.action(super::event_source_modal::EventSourceModalAction::Open {
                room_id,
                event_id: Some(event_id),
                latest_json: Some(latest_json),
            });
            true
        } else {
            false
        };
        let source_fetch_requested = if source_opened {
            false
        } else {
            self.tl_state
                .as_ref()
                .and_then(|tl| {
                    EventId::parse(self.telegram_message_edit_history_latest_event.trim())
                        .ok()
                        .map(|event_id| (tl.kind.clone(), event_id))
                })
                .map(|(timeline_kind, event_id)| {
                    submit_async_request(MatrixRequest::FetchEventSource {
                        timeline_kind,
                        event_id,
                    });
                    true
                })
                .unwrap_or(false)
        };
        if source_fetch_requested {
            source_kind = "remote latest replacement request".to_string();
        }
        self.refresh_telegram_message_edit_history_loaded_diff_detail(
            Some(if source_kind == "latest replacement" {
                "Latest replacement source"
            } else if source_kind == "remote latest replacement request" {
                "Remote latest replacement source"
            } else {
                "Loaded original source"
            }),
            false,
        );
        self.telegram_message_edit_history_full_boundary = edit_history_loaded_source_modal_label(
            &source_kind,
            &source_event_id,
            source_opened,
            loaded_json.as_deref(),
            &self.telegram_message_edit_history_loaded_original_preview,
            &self.telegram_message_edit_history_latest_preview,
            self.telegram_message_edit_history_replacement_count,
            &self.telegram_message_edit_history_latest_event,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        let popup_kind = if source_opened || source_fetch_requested {
            PopupKind::Success
        } else {
            PopupKind::Warning
        };
        enqueue_popup_notification(
            self.telegram_message_edit_history_full_boundary.clone(),
            popup_kind,
            Some(5.0),
        );
    }

    fn stage_telegram_message_edit_history_preflight_detail_control(
        &mut self,
        cx: &mut Cx,
        control: &str,
    ) {
        self.refresh_telegram_message_edit_history_preflight_detail(Some(control));
        let detail = self.telegram_message_edit_history_preflight_detail.clone();
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        enqueue_popup_notification(detail, PopupKind::Info, Some(5.0));
    }

    fn set_telegram_message_edit_history_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_message_edit_history_visible == visible {
            return;
        }
        self.telegram_message_edit_history_visible = visible;
        self.view
            .view(cx, ids!(telegram_message_edit_history_strip))
            .set_visible(cx, visible);
    }

    fn reset_telegram_message_edit_history_state(&mut self, cx: &mut Cx) {
        self.telegram_message_edit_history_summary.clear();
        self.telegram_message_edit_history_diff.clear();
        self.telegram_message_edit_history_metadata.clear();
        self.telegram_message_edit_history_loaded_diff_detail
            .clear();
        self.telegram_message_edit_history_preflight_detail.clear();
        self.telegram_message_edit_history_full_boundary.clear();
        self.telegram_message_edit_history_loaded_event_id.clear();
        self.telegram_message_edit_history_latest_event.clear();
        self.telegram_message_edit_history_replacement_count = None;
        self.telegram_message_edit_history_pages_fetched = None;
        self.telegram_message_edit_history_pagination_exhausted = false;
        self.telegram_message_edit_history_latest_timestamp = None;
        self.telegram_message_edit_history_loaded_original_preview
            .clear();
        self.telegram_message_edit_history_latest_preview.clear();
        self.telegram_message_edit_history_latest_source_json
            .clear();
        self.telegram_message_edit_history_result_error.clear();
        self.telegram_message_edit_history_retry_event_id = None;
        self.telegram_message_edit_history_retry_timeline_kind = None;
        self.set_telegram_message_edit_history_visible(cx, false);
    }

    fn show_telegram_message_edit_history_loading(
        &mut self,
        cx: &mut Cx,
        event_id: &EventId,
        loaded_metadata: &str,
    ) {
        self.telegram_message_edit_history_summary = format!(
            "Reading compact m.replace edit history for {event_id}. {MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_LABEL}"
        );
        self.telegram_message_edit_history_diff =
            "Waiting for replacement count and latest replacement preview.".to_string();
        self.telegram_message_edit_history_metadata = format!(
            "{loaded_metadata} No event-context fetch, timeline reload, event source, room-state, or live mutation."
        );
        self.telegram_message_edit_history_loaded_event_id = event_id.to_string();
        self.telegram_message_edit_history_latest_event.clear();
        self.telegram_message_edit_history_replacement_count = None;
        self.telegram_message_edit_history_pages_fetched = None;
        self.telegram_message_edit_history_pagination_exhausted = false;
        self.telegram_message_edit_history_latest_timestamp = None;
        self.telegram_message_edit_history_loaded_original_preview
            .clear();
        self.telegram_message_edit_history_latest_preview.clear();
        self.telegram_message_edit_history_latest_source_json
            .clear();
        self.telegram_message_edit_history_result_error.clear();
        self.refresh_telegram_message_edit_history_loaded_diff_detail(None, false);
        self.telegram_message_edit_history_full_boundary = edit_history_full_modal_boundary_label(
            "loading compact m.replace summary",
            None,
            None,
            false,
            false,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_message_report_status_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, false, None);
        self.set_telegram_room_info_visible(cx, false);
        self.set_telegram_notifications_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, false);
        self.set_telegram_matrix_link_preview_visible(cx, false);
        self.set_telegram_message_edit_history_visible(cx, true);
    }

    fn show_telegram_message_edit_history_result(
        &mut self,
        cx: &mut Cx,
        event_id: &EventId,
        replacement_count: usize,
        pages_fetched: usize,
        pagination_exhausted: bool,
        latest_event: &str,
        latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
        loaded_original: &str,
        latest_preview: &str,
        latest_source_json: Option<String>,
    ) {
        self.telegram_message_edit_history_summary = edit_history_detail_summary_label(
            event_id,
            replacement_count,
            pages_fetched,
            pagination_exhausted,
            latest_event,
            latest_timestamp,
        );
        self.telegram_message_edit_history_diff =
            edit_history_detail_diff_label(loaded_original, latest_preview);
        self.telegram_message_edit_history_metadata = edit_history_detail_metadata_label(
            replacement_count,
            pages_fetched,
            pagination_exhausted,
            loaded_original,
            latest_preview,
        );
        self.telegram_message_edit_history_loaded_event_id = event_id.to_string();
        self.telegram_message_edit_history_latest_event = latest_event.to_string();
        self.telegram_message_edit_history_replacement_count = Some(replacement_count);
        self.telegram_message_edit_history_pages_fetched = Some(pages_fetched);
        self.telegram_message_edit_history_pagination_exhausted = pagination_exhausted;
        self.telegram_message_edit_history_latest_timestamp = latest_timestamp.clone();
        self.telegram_message_edit_history_loaded_original_preview = loaded_original.to_string();
        self.telegram_message_edit_history_latest_preview = latest_preview.to_string();
        self.telegram_message_edit_history_latest_source_json =
            latest_source_json.unwrap_or_default();
        self.telegram_message_edit_history_result_error.clear();
        self.refresh_telegram_message_edit_history_loaded_diff_detail(None, false);
        self.telegram_message_edit_history_full_boundary = edit_history_full_modal_boundary_label(
            "loaded complete m.replace relation pagination summary",
            Some(replacement_count),
            Some(pages_fetched),
            pagination_exhausted,
            false,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
    }

    fn show_telegram_message_edit_history_error(
        &mut self,
        cx: &mut Cx,
        event_id: &EventId,
        error: &str,
    ) {
        self.telegram_message_edit_history_summary =
            format!("Edit history lookup failed for {event_id}.");
        self.telegram_message_edit_history_diff =
            "No replacement preview was rendered from the failed read.".to_string();
        self.telegram_message_edit_history_metadata = format!(
            "Failure metadata: {}. {}",
            compact_message_preview(error, "empty error"),
            MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_LABEL
        );
        self.telegram_message_edit_history_loaded_event_id = event_id.to_string();
        self.telegram_message_edit_history_latest_event.clear();
        self.telegram_message_edit_history_replacement_count = None;
        self.telegram_message_edit_history_pages_fetched = None;
        self.telegram_message_edit_history_pagination_exhausted = false;
        self.telegram_message_edit_history_latest_timestamp = None;
        self.telegram_message_edit_history_loaded_original_preview
            .clear();
        self.telegram_message_edit_history_latest_preview.clear();
        self.telegram_message_edit_history_latest_source_json
            .clear();
        self.telegram_message_edit_history_result_error = error.to_string();
        self.refresh_telegram_message_edit_history_loaded_diff_detail(None, true);
        self.telegram_message_edit_history_full_boundary = edit_history_full_modal_boundary_label(
            "failed compact m.replace summary",
            None,
            None,
            false,
            true,
        );
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
    }

    fn show_telegram_message_edit_history_retry_confirmation(&mut self, cx: &mut Cx) {
        let Some(event_id) = self.telegram_message_edit_history_retry_event_id.clone() else {
            enqueue_popup_notification(
                "Edit-history retry unavailable: no cached event id for the failed read.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let Some(timeline_kind) = self
            .telegram_message_edit_history_retry_timeline_kind
            .clone()
        else {
            enqueue_popup_notification(
                "Edit-history retry unavailable: no cached timeline kind for the failed read.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let event_id_for_accept = event_id.clone();
        let event_id_for_cancel = event_id.clone();
        let event_id_label = event_id.to_string();
        let body_label = edit_history_retry_confirmation_label(&event_id);
        let content = ConfirmationModalContent {
            title_text: "Retry Edit History".into(),
            body_text: body_label.into(),
            accept_button_text: Some("Retry".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                submit_async_request(MatrixRequest::FetchEditHistory {
                    timeline_kind,
                    event_id,
                });
                enqueue_popup_notification(
                    format!("Edit-history retry requested for {event_id_for_accept}."),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Edit-history retry canceled for {event_id_for_cancel}. {MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        self.telegram_message_edit_history_summary = format!(
            "Retry confirmation open for {event_id_label}. {MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_LABEL}"
        );
        self.telegram_message_edit_history_diff =
            "Retry is waiting for confirmation before another compact m.replace summary read."
                .to_string();
        self.telegram_message_edit_history_metadata =
        "Retry uses only cached event id and timeline kind; no full modal, event context, timeline reload, event source, or mutation."
            .to_string();
        self.telegram_message_edit_history_loaded_event_id = event_id_label.clone();
        self.telegram_message_edit_history_latest_event.clear();
        self.telegram_message_edit_history_replacement_count = None;
        self.telegram_message_edit_history_pages_fetched = None;
        self.telegram_message_edit_history_pagination_exhausted = false;
        self.telegram_message_edit_history_latest_timestamp = None;
        self.telegram_message_edit_history_loaded_original_preview
            .clear();
        self.telegram_message_edit_history_latest_preview.clear();
        self.telegram_message_edit_history_latest_source_json
            .clear();
        self.refresh_telegram_message_edit_history_loaded_diff_detail(None, true);
        self.telegram_message_edit_history_full_boundary =
            edit_history_full_modal_boundary_label("retry confirmation", None, None, false, true);
        self.refresh_telegram_message_edit_history_preflight_detail(None);
        self.update_telegram_message_edit_history_strip(cx);
        self.set_telegram_message_edit_history_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            format!(
                "Edit-history retry confirmation opened for {event_id_label}. {MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_LABEL}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn set_telegram_message_report_status_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.telegram_message_report_status_visible = visible;
        self.view
            .view(cx, ids!(telegram_message_report_status_strip))
            .set_visible(cx, visible);
    }

    fn reset_telegram_message_report_status_state(&mut self, cx: &mut Cx) {
        self.telegram_message_report_status_badge.clear();
        self.telegram_message_report_status_summary.clear();
        self.telegram_message_report_status_metadata.clear();
        self.telegram_message_report_preflight_detail_metadata
            .clear();
        self.telegram_message_report_result_error.clear();
        self.telegram_message_report_retry_event_id = None;
        self.telegram_message_report_retry_reason.clear();
        self.set_telegram_message_report_status_visible(cx, false);
    }

    fn update_telegram_message_report_status_strip(&mut self, cx: &mut Cx) {
        let retry_visible = self.telegram_message_report_status_badge == "failed"
            && self.telegram_message_report_retry_event_id.is_some()
            && !self.telegram_message_report_retry_reason.trim().is_empty();
        self.view
            .label(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_status_header
                        .report_status_badge
                ),
            )
            .set_text(cx, &self.telegram_message_report_status_badge);
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_status_summary),
            )
            .set_text(cx, &self.telegram_message_report_status_summary);
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_status_metadata),
            )
            .set_text(cx, &self.telegram_message_report_status_metadata);
        self.telegram_message_report_preflight_detail_metadata =
            message_report_preflight_detail_label(
                None,
                &self.telegram_message_report_status_badge,
                self.telegram_message_report_retry_event_id.as_deref(),
                &self.telegram_message_report_retry_reason,
                (!self.telegram_message_report_result_error.trim().is_empty())
                    .then_some(self.telegram_message_report_result_error.as_str()),
                retry_visible,
                &self.telegram_message_report_status_metadata,
            );
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_preflight_detail_metadata),
            )
            .set_text(cx, &self.telegram_message_report_preflight_detail_metadata);
        self.view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_status_header
                        .retry_report_status_button
                ),
            )
            .set_visible(cx, retry_visible);
    }

    fn show_telegram_message_report_submitted(
        &mut self,
        cx: &mut Cx,
        event_id: &EventId,
        reason: &str,
    ) {
        self.telegram_message_report_retry_event_id = Some(event_id.to_owned());
        self.telegram_message_report_retry_reason = reason.to_string();
        self.telegram_message_report_result_error.clear();
        self.telegram_message_report_status_badge = "submitted".to_string();
        self.telegram_message_report_status_summary =
            message_report_status_summary_label(event_id, "submitted", reason);
        self.telegram_message_report_status_metadata =
            message_report_status_metadata_label("submitted", reason, None);
        self.update_telegram_message_report_status_strip(cx);
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_message_edit_history_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, false, None);
        self.set_telegram_room_info_visible(cx, false);
        self.set_telegram_notifications_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, false);
        self.set_telegram_matrix_link_preview_visible(cx, false);
        self.set_telegram_message_report_status_visible(cx, true);
    }

    fn show_telegram_message_report_result(
        &mut self,
        cx: &mut Cx,
        event_id: &EventId,
        result: &Result<(), String>,
    ) {
        let reason = if self.telegram_message_report_retry_reason.trim().is_empty() {
            "confirmed reason"
        } else {
            self.telegram_message_report_retry_reason.as_str()
        };
        if self.telegram_message_report_retry_event_id.is_none() {
            self.telegram_message_report_retry_event_id = Some(event_id.to_owned());
        }
        match result {
            Ok(()) => {
                self.telegram_message_report_result_error.clear();
                self.telegram_message_report_status_badge = "sent".to_string();
                self.telegram_message_report_status_summary =
                    message_report_status_summary_label(event_id, "sent", reason);
                self.telegram_message_report_status_metadata =
                    message_report_status_metadata_label("sent", reason, None);
            }
            Err(error) => {
                self.telegram_message_report_result_error = error.clone();
                self.telegram_message_report_status_badge = "failed".to_string();
                self.telegram_message_report_status_summary =
                    message_report_status_summary_label(event_id, "failed", reason);
                self.telegram_message_report_status_metadata =
                    message_report_status_metadata_label("failed", reason, Some(error));
            }
        }
        self.update_telegram_message_report_status_strip(cx);
        self.set_telegram_message_report_status_visible(cx, true);
    }

    fn show_telegram_message_report_retry_confirmation(&mut self, cx: &mut Cx) {
        let Some(event_id) = self.telegram_message_report_retry_event_id.clone() else {
            enqueue_popup_notification(
                "Report retry unavailable: no cached event id for the failed report.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let reason = self.telegram_message_report_retry_reason.trim().to_string();
        if reason.is_empty() {
            enqueue_popup_notification(
                "Report retry unavailable: no cached report reason for the failed report.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        }
        let ui = self.widget_uid();
        let body_text = message_report_retry_confirmation_label(&event_id, &reason);
        let retry_event_id = event_id.clone();
        let retry_reason = reason.clone();
        let content = ConfirmationModalContent {
            title_text: "Retry Report".into(),
            body_text: body_text.into(),
            accept_button_text: Some("Retry".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                cx.widget_action(
                    ui,
                    MessageAction::RetryReport {
                        event_id: retry_event_id.clone(),
                        reason: retry_reason.clone(),
                    },
                );
                enqueue_popup_notification(
                    "Report retry confirmed. Matrix report_content was requested again.",
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(|_cx| {
                enqueue_popup_notification(
                    "Report retry canceled. Matrix report request was not sent again.",
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        enqueue_popup_notification(
            format!("Report retry confirmation opened. {MESSAGE_REPORT_RETRY_CONFIRMATION_LABEL}"),
            PopupKind::Info,
            Some(4.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn stage_telegram_message_report_workflow_action(&mut self, cx: &mut Cx, action: &str) {
        let status = if self.telegram_message_report_status_badge.trim().is_empty() {
            "waiting"
        } else {
            self.telegram_message_report_status_badge.as_str()
        };
        let reason = if self.telegram_message_report_retry_reason.trim().is_empty() {
            "confirmed reason unavailable"
        } else {
            self.telegram_message_report_retry_reason.as_str()
        };
        let metadata = if action == "Moderation queue" {
            message_report_local_moderation_snapshot_label(
                status,
                self.telegram_message_report_retry_event_id.as_deref(),
                reason,
                (!self.telegram_message_report_result_error.trim().is_empty())
                    .then_some(self.telegram_message_report_result_error.as_str()),
                &self.telegram_message_report_status_summary,
                &self.telegram_message_report_status_metadata,
                &self.telegram_message_report_preflight_detail_metadata,
                self.telegram_message_report_status_badge == "failed"
                    && self.telegram_message_report_retry_event_id.is_some()
                    && !self.telegram_message_report_retry_reason.trim().is_empty(),
            )
        } else {
            message_report_workflow_action_boundary_label(
                action,
                status,
                self.telegram_message_report_retry_event_id.as_deref(),
                reason,
            )
        };
        if self.telegram_message_report_status_badge.trim().is_empty() {
            self.telegram_message_report_status_badge = "local".to_string();
        }
        if self
            .telegram_message_report_status_summary
            .trim()
            .is_empty()
        {
            self.telegram_message_report_status_summary = format!(
                "Report workflow action {action} stayed local. {MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_LABEL}"
            );
        }
        self.telegram_message_report_status_metadata = metadata.clone();
        self.update_telegram_message_report_status_strip(cx);
        self.set_telegram_message_report_status_visible(cx, true);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn telegram_message_report_loaded_source_available(&self) -> bool {
        let Some(target_event_id) = self.telegram_message_report_retry_event_id.as_ref() else {
            return false;
        };
        let Some(tl) = self.tl_state.as_ref() else {
            return false;
        };
        tl.items
            .iter()
            .filter_map(|item| item.as_event())
            .any(|event| {
                event
                    .event_id()
                    .map(|loaded_event_id| {
                        loaded_event_id.as_str() == target_event_id.as_str()
                            && event.latest_json().is_some()
                    })
                    .unwrap_or(false)
            })
    }

    fn copy_telegram_message_report_moderation_reviewer_packet(&mut self, cx: &mut Cx) {
        if self.telegram_message_report_status_badge.trim().is_empty() {
            self.telegram_message_report_status_badge = "local".to_string();
        }
        if self
            .telegram_message_report_status_summary
            .trim()
            .is_empty()
        {
            self.telegram_message_report_status_summary = format!(
                "Report moderation reviewer packet stayed local. {MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_LABEL}"
            );
        }
        self.update_telegram_message_report_status_strip(cx);
        let retry_cached = self.telegram_message_report_status_badge == "failed"
            && self.telegram_message_report_retry_event_id.is_some()
            && !self.telegram_message_report_retry_reason.trim().is_empty();
        let loaded_source_available = self.telegram_message_report_loaded_source_available();
        let payload = message_report_moderation_reviewer_packet_payload(
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            &self.telegram_message_report_retry_reason,
            (!self.telegram_message_report_result_error.trim().is_empty())
                .then_some(self.telegram_message_report_result_error.as_str()),
            &self.telegram_message_report_status_summary,
            &self.telegram_message_report_status_metadata,
            &self.telegram_message_report_preflight_detail_metadata,
            retry_cached,
            loaded_source_available,
        );
        cx.copy_to_clipboard(&payload);
        let metadata = message_report_moderation_reviewer_packet_label(
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            retry_cached,
            loaded_source_available,
        );
        self.telegram_message_report_status_metadata = metadata.clone();
        self.telegram_message_report_preflight_detail_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_status_metadata),
            )
            .set_text(cx, &metadata);
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_preflight_detail_metadata),
            )
            .set_text(cx, &metadata);
        self.set_telegram_message_report_status_visible(cx, true);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn copy_telegram_message_report_workflow_result_contract_packet(&mut self, cx: &mut Cx) {
        if self.telegram_message_report_status_badge.trim().is_empty() {
            self.telegram_message_report_status_badge = "local".to_string();
        }
        if self
            .telegram_message_report_status_summary
            .trim()
            .is_empty()
        {
            self.telegram_message_report_status_summary = format!(
                "Report workflow Contract stayed local. {MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_LABEL}"
            );
        }
        self.update_telegram_message_report_status_strip(cx);
        let retry_cached = self.telegram_message_report_status_badge == "failed"
            && self.telegram_message_report_retry_event_id.is_some()
            && !self.telegram_message_report_retry_reason.trim().is_empty();
        let loaded_source_available = self.telegram_message_report_loaded_source_available();
        let payload = message_report_workflow_result_contract_packet_payload(
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            &self.telegram_message_report_retry_reason,
            (!self.telegram_message_report_result_error.trim().is_empty())
                .then_some(self.telegram_message_report_result_error.as_str()),
            &self.telegram_message_report_status_summary,
            &self.telegram_message_report_status_metadata,
            &self.telegram_message_report_preflight_detail_metadata,
            retry_cached,
            loaded_source_available,
        );
        cx.copy_to_clipboard(&payload);
        let metadata = message_report_workflow_result_contract_packet_label(
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            retry_cached,
            loaded_source_available,
        );
        self.telegram_message_report_status_metadata = metadata.clone();
        self.telegram_message_report_preflight_detail_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_status_metadata),
            )
            .set_text(cx, &metadata);
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_preflight_detail_metadata),
            )
            .set_text(cx, &metadata);
        self.set_telegram_message_report_status_visible(cx, true);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn copy_telegram_message_report_workflow_result_taxonomy_packet(&mut self, cx: &mut Cx) {
        if self.telegram_message_report_status_badge.trim().is_empty() {
            self.telegram_message_report_status_badge = "local".to_string();
        }
        if self
            .telegram_message_report_status_summary
            .trim()
            .is_empty()
        {
            self.telegram_message_report_status_summary = format!(
                "Report workflow Taxonomy stayed local. {MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_LABEL}"
            );
        }
        self.update_telegram_message_report_status_strip(cx);
        let retry_cached = self.telegram_message_report_status_badge == "failed"
            && self.telegram_message_report_retry_event_id.is_some()
            && !self.telegram_message_report_retry_reason.trim().is_empty();
        let loaded_source_available = self.telegram_message_report_loaded_source_available();
        let payload = message_report_workflow_result_taxonomy_packet_payload(
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            &self.telegram_message_report_retry_reason,
            (!self.telegram_message_report_result_error.trim().is_empty())
                .then_some(self.telegram_message_report_result_error.as_str()),
            &self.telegram_message_report_status_summary,
            &self.telegram_message_report_status_metadata,
            &self.telegram_message_report_preflight_detail_metadata,
            retry_cached,
            loaded_source_available,
        );
        cx.copy_to_clipboard(&payload);
        let metadata = message_report_workflow_result_taxonomy_packet_label(
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            retry_cached,
            loaded_source_available,
        );
        self.telegram_message_report_status_metadata = metadata.clone();
        self.telegram_message_report_preflight_detail_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_status_metadata),
            )
            .set_text(cx, &metadata);
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_preflight_detail_metadata),
            )
            .set_text(cx, &metadata);
        self.set_telegram_message_report_status_visible(cx, true);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn stage_telegram_message_report_preflight_detail_control(
        &mut self,
        cx: &mut Cx,
        action: &str,
    ) {
        if self.telegram_message_report_status_badge.trim().is_empty() {
            self.telegram_message_report_status_badge = "local".to_string();
        }
        if self
            .telegram_message_report_status_summary
            .trim()
            .is_empty()
        {
            self.telegram_message_report_status_summary = format!(
                "Report preflight detail {action} stayed local. {MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
            );
        }
        self.update_telegram_message_report_status_strip(cx);
        let retry_cached = self.telegram_message_report_status_badge == "failed"
            && self.telegram_message_report_retry_event_id.is_some()
            && !self.telegram_message_report_retry_reason.trim().is_empty();
        let metadata = message_report_preflight_detail_label(
            Some(action),
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            &self.telegram_message_report_retry_reason,
            (!self.telegram_message_report_result_error.trim().is_empty())
                .then_some(self.telegram_message_report_result_error.as_str()),
            retry_cached,
            &self.telegram_message_report_status_metadata,
        );
        self.telegram_message_report_preflight_detail_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_preflight_detail_metadata),
            )
            .set_text(cx, &metadata);
        self.set_telegram_message_report_status_visible(cx, true);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn copy_telegram_message_report_status_summary(&mut self, cx: &mut Cx) {
        self.update_telegram_message_report_status_strip(cx);
        let payload = message_report_status_clipboard_payload(
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            &self.telegram_message_report_retry_reason,
            (!self.telegram_message_report_result_error.trim().is_empty())
                .then_some(self.telegram_message_report_result_error.as_str()),
            &self.telegram_message_report_status_summary,
            &self.telegram_message_report_status_metadata,
            &self.telegram_message_report_preflight_detail_metadata,
        );
        let copied = payload.is_some();
        if let Some(payload) = payload {
            cx.copy_to_clipboard(&payload);
        }
        let metadata = message_report_status_clipboard_label(
            copied,
            &self.telegram_message_report_status_badge,
            self.telegram_message_report_retry_event_id.as_deref(),
            &self.telegram_message_report_retry_reason,
            (!self.telegram_message_report_result_error.trim().is_empty())
                .then_some(self.telegram_message_report_result_error.as_str()),
            &self.telegram_message_report_status_summary,
        );
        if self.telegram_message_report_status_badge.trim().is_empty() {
            self.telegram_message_report_status_badge = "local".to_string();
        }
        if self
            .telegram_message_report_status_summary
            .trim()
            .is_empty()
        {
            self.telegram_message_report_status_summary = metadata.clone();
        }
        self.telegram_message_report_preflight_detail_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_preflight_detail_metadata),
            )
            .set_text(cx, &metadata);
        self.set_telegram_message_report_status_visible(cx, true);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn open_telegram_message_report_loaded_source(&mut self, cx: &mut Cx) {
        if self.telegram_message_report_status_badge.trim().is_empty() {
            self.telegram_message_report_status_badge = "local".to_string();
        }
        if self
            .telegram_message_report_status_summary
            .trim()
            .is_empty()
        {
            self.telegram_message_report_status_summary = format!(
                "Report Source stayed on loaded local event JSON. {MESSAGE_REPORT_LOADED_SOURCE_MODAL_LABEL}"
            );
        }

        let cached_event_id = self.telegram_message_report_retry_event_id.clone();
        let loaded_source_detail = cached_event_id.as_ref().and_then(|target_event_id| {
            self.tl_state.as_ref().and_then(|tl| {
                let room_id = tl.kind.room_id().clone();
                tl.items
                    .iter()
                    .enumerate()
                    .filter_map(|(index, item)| item.as_event().map(|event| (index, event)))
                    .find_map(|(loaded_index, event_tl_item)| {
                        let loaded_event_id = event_tl_item.event_id()?;
                        (loaded_event_id.as_str() == target_event_id.as_str()).then(|| {
                            let latest_json = event_tl_item
                                .latest_json()
                                .and_then(|raw_event| serde_json::to_value(raw_event).ok())
                                .and_then(|value| serde_json::to_string_pretty(&value).ok());
                            (
                                room_id.clone(),
                                loaded_event_id.to_owned(),
                                loaded_index,
                                latest_json,
                            )
                        })
                    })
            })
        });
        let mut source_fetch_requested = false;
        if let Some((room_id, event_id, _, Some(latest_json))) = loaded_source_detail.clone() {
            cx.action(super::event_source_modal::EventSourceModalAction::Open {
                room_id,
                event_id: Some(event_id),
                latest_json: Some(latest_json),
            });
        } else if let (Some(event_id), Some(timeline_kind)) =
            (cached_event_id.clone(), self.timeline_kind.clone())
        {
            submit_async_request(MatrixRequest::FetchEventSource {
                timeline_kind,
                event_id,
            });
            source_fetch_requested = true;
        }
        let loaded_index = loaded_source_detail
            .as_ref()
            .map(|(_, _, loaded_index, _)| *loaded_index);
        let latest_json = loaded_source_detail
            .as_ref()
            .and_then(|(_, _, _, latest_json)| latest_json.as_deref());
        let metadata = message_report_loaded_source_modal_label(
            cached_event_id.as_deref(),
            &self.telegram_message_report_status_badge,
            &self.telegram_message_report_retry_reason,
            loaded_index,
            latest_json,
            source_fetch_requested,
        );
        self.update_telegram_message_report_status_strip(cx);
        self.telegram_message_report_preflight_detail_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_report_status_strip.report_preflight_detail_metadata),
            )
            .set_text(cx, &metadata);
        self.set_telegram_message_report_status_visible(cx, true);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn set_telegram_matrix_link_preview_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.telegram_matrix_link_preview_visible = visible;
        self.view
            .view(cx, ids!(telegram_matrix_link_preview_strip))
            .set_visible(cx, visible);
    }

    fn update_telegram_matrix_link_preview_strip(&mut self, cx: &mut Cx) {
        let retry_visible = self.telegram_matrix_link_preview_status == "failed"
            && self
                .telegram_matrix_link_preview_retry_room_or_alias_id
                .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some();
        self.view
            .label(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_preview_header
                        .matrix_link_preview_status
                ),
            )
            .set_text(cx, &self.telegram_matrix_link_preview_status);
        self.view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_preview_header
                        .retry_matrix_link_preview_button
                ),
            )
            .set_visible(cx, retry_visible);
        self.view
            .label(
                cx,
                ids!(telegram_matrix_link_preview_strip.matrix_link_preview_summary),
            )
            .set_text(cx, &self.telegram_matrix_link_preview_summary);
        self.view
            .label(
                cx,
                ids!(telegram_matrix_link_preview_strip.matrix_link_preview_metadata),
            )
            .set_text(cx, &self.telegram_matrix_link_preview_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_matrix_link_preview_strip.matrix_link_unresolved_detail),
            )
            .set_text(cx, &self.telegram_matrix_link_unresolved_detail);
        self.view
            .label(
                cx,
                ids!(telegram_matrix_link_preview_strip.matrix_link_route_scope_metadata),
            )
            .set_text(cx, &self.telegram_matrix_link_route_scope_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_matrix_link_preview_strip.matrix_link_context_actions_metadata),
            )
            .set_text(cx, &self.telegram_matrix_link_context_actions_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_matrix_link_preview_strip.matrix_link_server_context_boundary),
            )
            .set_text(cx, &self.telegram_matrix_link_server_context_boundary);
    }

    fn refresh_telegram_matrix_link_unresolved_detail(&mut self, action: Option<&str>) {
        let status = if self.telegram_matrix_link_preview_status.trim().is_empty() {
            "preview"
        } else {
            self.telegram_matrix_link_preview_status.trim()
        };
        let retry_cache_ready = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some()
            || self
                .telegram_matrix_link_join_retry_room_or_alias_id
                .is_some()
            || self
                .telegram_matrix_link_knock_retry_room_or_alias_id
                .is_some();
        let retry_cache_ready = retry_cache_ready
            || self.telegram_matrix_link_invite_retry_room_id.is_some()
            || self.telegram_matrix_link_invite_retry_user_id.is_some();
        self.telegram_matrix_link_unresolved_detail = matrix_link_unresolved_detail_label(
            action,
            status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            self.telegram_matrix_link_preview_error_chars,
            retry_cache_ready,
        );
    }

    fn show_telegram_matrix_link_user_target(
        &mut self,
        cx: &mut Cx,
        user_id: &UserId,
        via: &[OwnedServerName],
        current_room_id: OwnedRoomId,
        metadata: String,
    ) {
        self.telegram_matrix_link_preview_retry_room_or_alias_id = None;
        self.telegram_matrix_link_preview_retry_via.clear();
        self.telegram_matrix_link_preview_retry_event_id = None;
        self.telegram_matrix_link_preview_retry_timeline_kind = None;
        self.telegram_matrix_link_join_pending_room_or_alias_id = None;
        self.telegram_matrix_link_join_retry_room_or_alias_id = None;
        self.telegram_matrix_link_join_via_servers = via.to_vec();
        self.telegram_matrix_link_join_retry_via_servers.clear();
        self.telegram_matrix_link_knock_pending_room_or_alias_id = None;
        self.telegram_matrix_link_knock_retry_room_or_alias_id = None;
        self.telegram_matrix_link_knock_via_servers = via.to_vec();
        self.telegram_matrix_link_knock_retry_via_servers.clear();
        self.telegram_matrix_link_invite_pending_room_id = None;
        self.telegram_matrix_link_invite_pending_user_id = None;
        self.telegram_matrix_link_invite_retry_room_id = Some(current_room_id);
        self.telegram_matrix_link_invite_retry_user_id = Some(user_id.to_owned());
        self.telegram_matrix_link_preview_status = "user".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix user link opened for {user_id}. Profile pane uses the existing read path; Invite confirms before MatrixRequest::InviteUser."
        );
        self.telegram_matrix_link_preview_metadata =
            format!("{metadata} {MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL}");
        self.telegram_matrix_link_preview_target_label = user_id.to_string();
        self.telegram_matrix_link_preview_room_or_alias_id = None;
        self.telegram_matrix_link_preview_via_count = via.len();
        self.telegram_matrix_link_preview_via_label = matrix_link_via_servers_label(via);
        self.telegram_matrix_link_preview_event_id_label.clear();
        self.telegram_matrix_link_preview_error_chars = None;
        self.telegram_matrix_link_preview_source_room_id = None;
        self.telegram_matrix_link_preview_source_event_id = None;
        self.telegram_matrix_link_preview_source_json.clear();
        self.refresh_telegram_matrix_link_unresolved_detail(Some("User link"));
        self.telegram_matrix_link_route_scope_metadata = matrix_link_route_scope_controls_label(
            Some("User"),
            "user",
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            true,
        );
        self.telegram_matrix_link_context_actions_metadata =
            matrix_link_context_actions_row_label(Some("Invite"), "user", via.len(), None, true);
        self.telegram_matrix_link_server_context_boundary =
            matrix_link_server_context_boundary_label("user profile target", via.len(), None, true);
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_message_edit_history_visible(cx, false);
        self.set_telegram_message_report_status_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, false, None);
        self.set_telegram_room_info_visible(cx, false);
        self.set_telegram_notifications_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, false);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }
}

impl RoomScreen {
    fn show_matrix_link_preview_request(
        &mut self,
        cx: &mut Cx,
        kind: &str,
        target: String,
        room_or_alias_id: OwnedRoomOrAliasId,
        via: Vec<OwnedServerName>,
        event_id: Option<OwnedEventId>,
        metadata: String,
    ) {
        let Some(timeline_kind) = self.timeline_kind.clone() else {
            enqueue_popup_notification(
                format!(
                    "{kind} Matrix link preview unavailable for {target}: no active timeline. {MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_LABEL}"
                ),
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };

        self.telegram_matrix_link_preview_retry_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_retry_via = via.clone();
        self.telegram_matrix_link_join_via_servers = via.clone();
        self.telegram_matrix_link_knock_via_servers = via.clone();
        self.telegram_matrix_link_preview_retry_event_id = event_id.clone();
        self.telegram_matrix_link_preview_retry_timeline_kind = Some(timeline_kind.clone());
        self.telegram_matrix_link_preview_status = "loading".to_string();
        self.telegram_matrix_link_preview_summary =
            format!("{kind} Matrix link preview requested for {target}.");
        self.telegram_matrix_link_preview_metadata =
            format!("{metadata} {MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_LABEL}");
        self.telegram_matrix_link_preview_target_label = target.clone();
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_via_count = via.len();
        self.telegram_matrix_link_preview_via_label = matrix_link_via_servers_label(&via);
        self.telegram_matrix_link_preview_event_id_label = event_id
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        self.telegram_matrix_link_preview_error_chars = None;
        self.telegram_matrix_link_preview_source_room_id = None;
        self.telegram_matrix_link_preview_source_event_id = None;
        self.telegram_matrix_link_preview_source_json.clear();
        self.refresh_telegram_matrix_link_unresolved_detail(None);
        self.telegram_matrix_link_route_scope_metadata = matrix_link_route_scope_controls_label(
            None,
            "loading",
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            true,
        );
        self.telegram_matrix_link_context_actions_metadata = matrix_link_context_actions_row_label(
            None,
            "loading",
            via.len(),
            event_id.as_ref(),
            true,
        );
        self.telegram_matrix_link_server_context_boundary =
            matrix_link_server_context_boundary_label(
                "compact preview read requested",
                via.len(),
                event_id.as_ref(),
                true,
            );
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_message_edit_history_visible(cx, false);
        self.set_telegram_message_report_status_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, false, None);
        self.set_telegram_room_info_visible(cx, false);
        self.set_telegram_notifications_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, false);
        self.set_telegram_matrix_link_preview_visible(cx, true);

        submit_async_request(MatrixRequest::PreviewMatrixLinkTarget {
            timeline_kind,
            room_or_alias_id,
            via,
            event_id,
        });
        enqueue_popup_notification(
            format!("{kind} Matrix link preview requested for {target}. {metadata}"),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn show_telegram_matrix_link_preview_result(
        &mut self,
        cx: &mut Cx,
        room_or_alias_id: &OwnedRoomOrAliasId,
        via: &[OwnedServerName],
        event_id: Option<&OwnedEventId>,
        event_source_room_id: Option<OwnedRoomId>,
        event_source_json: Option<String>,
        metadata: &str,
    ) {
        let event_source_ready = event_id.is_some()
            && event_source_room_id.is_some()
            && event_source_json
                .as_deref()
                .is_some_and(|json| !json.trim().is_empty());
        self.telegram_matrix_link_preview_retry_room_or_alias_id = None;
        self.telegram_matrix_link_preview_retry_via.clear();
        self.telegram_matrix_link_join_via_servers = via.to_vec();
        self.telegram_matrix_link_knock_via_servers = via.to_vec();
        self.telegram_matrix_link_preview_retry_event_id = None;
        self.telegram_matrix_link_preview_retry_timeline_kind = None;
        self.telegram_matrix_link_preview_status = "resolved".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link compact preview resolved for {room_or_alias_id}. {MATRIX_LINK_PREVIEW_RESULT_METADATA_LABEL}"
        );
        self.telegram_matrix_link_preview_metadata = metadata.to_string();
        self.telegram_matrix_link_preview_target_label = room_or_alias_id.to_string();
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_via_count = via.len();
        self.telegram_matrix_link_preview_via_label = matrix_link_via_servers_label(via);
        self.telegram_matrix_link_preview_event_id_label =
            event_id.map(ToString::to_string).unwrap_or_default();
        self.telegram_matrix_link_preview_error_chars = None;
        self.telegram_matrix_link_preview_source_room_id = if event_source_ready {
            event_source_room_id
        } else {
            None
        };
        self.telegram_matrix_link_preview_source_event_id = if event_source_ready {
            event_id.cloned()
        } else {
            None
        };
        self.telegram_matrix_link_preview_source_json = if event_source_ready {
            event_source_json.unwrap_or_default()
        } else {
            String::new()
        };
        self.refresh_telegram_matrix_link_unresolved_detail(None);
        self.telegram_matrix_link_route_scope_metadata = matrix_link_route_scope_controls_label(
            None,
            "resolved",
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            false,
        );
        self.telegram_matrix_link_context_actions_metadata =
            matrix_link_context_actions_row_label(None, "resolved", via.len(), event_id, false);
        self.telegram_matrix_link_server_context_boundary =
            matrix_link_server_context_boundary_label(
                "compact preview resolved",
                via.len(),
                event_id,
                false,
            );
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }

    fn show_telegram_matrix_link_preview_error(
        &mut self,
        cx: &mut Cx,
        timeline_kind: TimelineKind,
        room_or_alias_id: OwnedRoomOrAliasId,
        via: Vec<OwnedServerName>,
        event_id: Option<OwnedEventId>,
        error: &str,
    ) {
        let target = room_or_alias_id.to_string();
        let metadata = matrix_link_preview_failure_metadata_label(
            &target,
            via.len(),
            event_id.as_ref(),
            error,
        );
        let boundary = matrix_link_server_context_boundary_label(
            "compact preview failed",
            via.len(),
            event_id.as_ref(),
            true,
        );
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_retry_room_or_alias_id = Some(room_or_alias_id);
        self.telegram_matrix_link_preview_retry_via = via.clone();
        self.telegram_matrix_link_join_via_servers = via.clone();
        self.telegram_matrix_link_knock_via_servers = via;
        self.telegram_matrix_link_preview_retry_event_id = event_id;
        self.telegram_matrix_link_preview_retry_timeline_kind = Some(timeline_kind);
        self.telegram_matrix_link_preview_status = "failed".to_string();
        self.telegram_matrix_link_preview_summary =
            format!("Matrix link compact preview failed for {target}.");
        self.telegram_matrix_link_preview_metadata =
            format!("{metadata} {MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_LABEL}");
        self.telegram_matrix_link_preview_target_label = target;
        self.telegram_matrix_link_preview_via_count =
            self.telegram_matrix_link_preview_retry_via.len();
        self.telegram_matrix_link_preview_via_label =
            matrix_link_via_servers_label(&self.telegram_matrix_link_preview_retry_via);
        self.telegram_matrix_link_preview_event_id_label = self
            .telegram_matrix_link_preview_retry_event_id
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        self.telegram_matrix_link_preview_error_chars = Some(error.trim().chars().count());
        self.telegram_matrix_link_preview_source_room_id = None;
        self.telegram_matrix_link_preview_source_event_id = None;
        self.telegram_matrix_link_preview_source_json.clear();
        self.refresh_telegram_matrix_link_unresolved_detail(None);
        self.telegram_matrix_link_route_scope_metadata = matrix_link_route_scope_controls_label(
            None,
            "failed",
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            true,
        );
        self.telegram_matrix_link_context_actions_metadata = matrix_link_context_actions_row_label(
            None,
            "failed",
            self.telegram_matrix_link_preview_retry_via.len(),
            self.telegram_matrix_link_preview_retry_event_id.as_ref(),
            true,
        );
        self.telegram_matrix_link_server_context_boundary = boundary;
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }
}
