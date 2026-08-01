impl RoomScreen {
    fn copy_telegram_message_search_server_packet(&mut self, cx: &mut Cx) {
        self.set_telegram_search_mode_visible(cx, true);
        let query = self.telegram_message_search_query.trim();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let match_count = self.telegram_message_search_matches.len();
        let timeline_loaded = self.tl_state.is_some();
        let payload = message_search_server_packet_clipboard_payload(
            query,
            loaded_item_count,
            match_count,
            self.telegram_message_search_active_match,
            timeline_loaded,
            self.pinned_events.len(),
            &self.telegram_message_search_server_context_controls_metadata,
            &self.telegram_message_search_server_preflight_controls_metadata,
        );
        cx.copy_to_clipboard(&payload);
        let label = message_search_server_packet_clipboard_label(
            true,
            query,
            loaded_item_count,
            match_count,
            &payload,
        );
        self.telegram_message_search_server_preflight_controls_metadata = label.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_server_preflight_controls_metadata),
            )
            .set_text(cx, &label);
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.server_context_boundary),
            )
            .set_text(
                cx,
                &format!("Packet copied locally. {MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_LABEL}"),
            );
        enqueue_popup_notification(label, PopupKind::Info, Some(4.0));
    }

    fn apply_telegram_message_search_loaded_scope_control(
        &mut self,
        cx: &mut Cx,
        action: &str,
        loaded_scope: MessageSearchLoadedScope,
    ) {
        self.set_telegram_search_mode_visible(cx, true);
        self.telegram_message_search_loaded_scope = loaded_scope;
        self.telegram_message_search_active_match = 0;
        self.refresh_telegram_message_search_matches(cx);
        let query = self.telegram_message_search_query.trim();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let match_count = self.telegram_message_search_matches.len();
        let timeline_loaded = self.tl_state.is_some();
        let metadata = loaded_message_search_advanced_filter_controls_label(
            Some(action),
            query,
            loaded_item_count,
            match_count,
            self.telegram_message_search_active_match,
            timeline_loaded,
            self.pinned_events.len(),
            self.telegram_message_search_loaded_scope,
        );
        self.telegram_message_search_advanced_filter_controls_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_advanced_filter_controls_metadata),
            )
            .set_text(cx, &metadata);
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.server_context_boundary),
            )
            .set_text(
                cx,
                &message_search_loaded_scope_filter_label(
                    action,
                    loaded_scope,
                    loaded_item_count,
                    match_count,
                    self.pinned_events.len(),
                ),
            );
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn stage_telegram_message_search_result_action_control(&mut self, cx: &mut Cx, action: &str) {
        self.set_telegram_search_mode_visible(cx, true);
        let query = self.telegram_message_search_query.trim();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let match_count = self.telegram_message_search_matches.len();
        let active_loaded_index = self
            .telegram_message_search_matches
            .get(self.telegram_message_search_active_match)
            .copied();
        let active_match_detail = if let (Some(tl_state), Some(item_index)) =
            (self.tl_state.as_ref(), active_loaded_index)
        {
            tl_state
                .items
                .get(item_index)
                .and_then(|item| item.as_event())
                .map(|event_tl_item| {
                    (
                        event_tl_item.event_id().map(ToString::to_string),
                        plaintext_body_of_timeline_item(event_tl_item),
                    )
                })
        } else {
            None
        };
        let active_source_detail = if let (Some(tl_state), Some(item_index)) =
            (self.tl_state.as_ref(), active_loaded_index)
        {
            tl_state
                .items
                .get(item_index)
                .and_then(|item| item.as_event())
                .map(|event_tl_item| {
                    let latest_json = event_tl_item
                        .latest_json()
                        .and_then(|raw_event| serde_json::to_value(raw_event).ok())
                        .and_then(|value| serde_json::to_string_pretty(&value).ok());
                    (
                        tl_state.kind.room_id().clone(),
                        event_tl_item.event_id().map(|event_id| event_id.to_owned()),
                        latest_json,
                    )
                })
        } else {
            None
        };
        let active_event_id_loaded = active_match_detail
            .as_ref()
            .and_then(|(event_id, _)| event_id.as_ref())
            .is_some();
        let loaded_body = active_match_detail.as_ref().map(|(_, body)| body.as_str());
        let server_source_detail = self
            .telegram_message_search_server_source_detail()
            .filter(|(_, latest_json)| !latest_json.trim().is_empty());
        let mut source_opened = false;
        let mut source_fetch_requested = false;
        let metadata = if action.eq_ignore_ascii_case("Copy") {
            if let Some(body) = loaded_body {
                cx.copy_to_clipboard(body);
            }
            loaded_message_search_result_copy_clipboard_label(
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                active_loaded_index,
                active_event_id_loaded,
                loaded_body,
            )
        } else if action.eq_ignore_ascii_case("Source") {
            let source_json = active_source_detail
                .as_ref()
                .and_then(|(_, _, latest_json)| latest_json.as_deref());
            if let Some((room_id, event_id, Some(latest_json))) = active_source_detail.clone() {
                cx.action(super::event_source_modal::EventSourceModalAction::Open {
                    room_id,
                    event_id,
                    latest_json: Some(latest_json),
                });
                source_opened = true;
            } else if let Some((event_id, latest_json)) = server_source_detail.clone()
                && let Some(timeline_kind) = self.timeline_kind.clone()
            {
                cx.action(super::event_source_modal::EventSourceModalAction::Open {
                    room_id: timeline_kind.room_id().clone(),
                    event_id: Some(event_id),
                    latest_json: Some(latest_json),
                });
                source_opened = true;
            } else {
                let loaded_source_fetch_target =
                    active_source_detail
                        .as_ref()
                        .and_then(|(_, event_id, latest_json)| {
                            latest_json.is_none().then(|| event_id.clone()).flatten()
                        });
                let server_source_fetch_target =
                    self.telegram_message_search_server_source_target();
                if let (Some(timeline_kind), Some(event_id)) = (
                    self.timeline_kind.clone(),
                    loaded_source_fetch_target.or(server_source_fetch_target),
                ) {
                    submit_async_request(MatrixRequest::FetchEventSource {
                        timeline_kind,
                        event_id,
                    });
                    source_fetch_requested = true;
                }
            }
            loaded_message_search_result_source_modal_label(
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                active_loaded_index,
                active_event_id_loaded,
                source_json,
                server_source_detail
                    .as_ref()
                    .map(|(_, latest_json)| latest_json.as_str()),
                source_opened,
                source_fetch_requested,
            )
        } else {
            loaded_message_search_result_action_controls_label(
                Some(action),
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                active_loaded_index,
                active_event_id_loaded,
                loaded_body,
            )
        };
        self.telegram_message_search_result_action_controls_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_result_action_controls_metadata),
            )
            .set_text(cx, &metadata);
        self.view
        .label(
            cx,
            ids!(telegram_message_search_strip.server_context_boundary),
        )
        .set_text(cx, &{
            if action.eq_ignore_ascii_case("Copy") {
                format!(
                    "Copy result action wrote loaded plaintext to local clipboard. {MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_LABEL}"
                )
            } else if action.eq_ignore_ascii_case("Source") {
                if source_opened {
                    format!(
                        "Source result action opened loaded or Matrix server-result event source. {MESSAGE_SEARCH_RESULT_SOURCE_MODAL_LABEL}"
                    )
                } else if source_fetch_requested {
                    format!(
                        "Source result action requested current-room MatrixRequest::FetchEventSource. {MESSAGE_SEARCH_RESULT_SOURCE_MODAL_LABEL}"
                    )
                } else {
                    format!(
                        "Source result action needs a loaded latest_json, cached Matrix server-result source, or current-room event id before opening event source. {MESSAGE_SEARCH_RESULT_SOURCE_MODAL_LABEL}"
                    )
                }
            } else {
                format!(
                    "{action} result action stayed metadata-only. {MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_LABEL}"
                )
            }
        });
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn telegram_message_search_server_source_detail(&self) -> Option<(OwnedEventId, String)> {
        let current_room_id = self.timeline_kind.as_ref()?.room_id();
        if !self
            .telegram_message_search_server_room_id
            .trim()
            .is_empty()
            && self.telegram_message_search_server_room_id.trim() != current_room_id.as_str()
        {
            return None;
        }
        self.telegram_message_search_server_hits
            .iter()
            .find_map(|hit| {
                let event_id = hit.event_id.as_deref()?.trim();
                let latest_json = hit
                    .source_json
                    .as_deref()
                    .map(str::trim)
                    .filter(|json| !json.is_empty())?;
                let event_id = OwnedEventId::try_from(event_id).ok()?;
                Some((event_id, latest_json.to_string()))
            })
    }

    fn telegram_message_search_server_source_target(&self) -> Option<OwnedEventId> {
        let current_room_id = self.timeline_kind.as_ref()?.room_id();
        if !self
            .telegram_message_search_server_room_id
            .trim()
            .is_empty()
            && self.telegram_message_search_server_room_id.trim() != current_room_id.as_str()
        {
            return None;
        }
        self.telegram_message_search_server_hits
            .iter()
            .find_map(|hit| {
                let cached_source_present = hit
                    .source_json
                    .as_deref()
                    .map(str::trim)
                    .is_some_and(|json| !json.is_empty());
                if cached_source_present {
                    return None;
                }
                let event_id = hit.event_id.as_deref()?.trim();
                OwnedEventId::try_from(event_id).ok()
            })
    }

    fn jump_telegram_message_search_active_match(
        &mut self,
        cx: &mut Cx,
        portal_list: &PortalListRef,
    ) {
        self.set_telegram_search_mode_visible(cx, true);
        self.refresh_telegram_message_search_matches(cx);
        let query = self.telegram_message_search_query.trim();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let match_count = self.telegram_message_search_matches.len();
        let active_loaded_index = self
            .telegram_message_search_matches
            .get(self.telegram_message_search_active_match)
            .copied();
        let active_match_detail = if let (Some(tl_state), Some(item_index)) =
            (self.tl_state.as_ref(), active_loaded_index)
        {
            tl_state
                .items
                .get(item_index)
                .and_then(|item| item.as_event())
                .map(|event_tl_item| {
                    (
                        event_tl_item.event_id().map(ToString::to_string),
                        plaintext_body_of_timeline_item(event_tl_item),
                    )
                })
        } else {
            None
        };
        let active_event_id_loaded = active_match_detail
            .as_ref()
            .and_then(|(event_id, _)| event_id.as_ref())
            .is_some();
        let loaded_body = active_match_detail.as_ref().map(|(_, body)| body.as_str());
        let jumped = if let (Some(tl_state), Some(item_index)) =
            (self.tl_state.as_mut(), active_loaded_index)
        {
            let speed = 50.0;
            portal_list.smooth_scroll_to(cx, item_index, speed, None, 10.0);
            tl_state.message_highlight_animation_state = MessageHighlightAnimationState::Pending {
                item_id: item_index,
            };
            true
        } else {
            false
        };
        let metadata = loaded_message_search_result_jump_loaded_match_label(
            query,
            loaded_item_count,
            match_count,
            self.telegram_message_search_active_match,
            active_loaded_index,
            active_event_id_loaded,
            loaded_body,
            jumped,
        );
        self.telegram_message_search_result_action_controls_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_result_action_controls_metadata),
            )
            .set_text(cx, &metadata);
        self.view
        .label(
            cx,
            ids!(telegram_message_search_strip.server_context_boundary),
        )
        .set_text(
            cx,
            &format!(
                "Jump result action scrolled to loaded local match. {MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_LABEL}"
            ),
        );
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn open_telegram_message_search_active_thread(&mut self, cx: &mut Cx) {
        self.set_telegram_search_mode_visible(cx, true);
        self.refresh_telegram_message_search_matches(cx);
        let query = self.telegram_message_search_query.trim();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let match_count = self.telegram_message_search_matches.len();
        let active_loaded_index = self
            .telegram_message_search_matches
            .get(self.telegram_message_search_active_match)
            .copied();
        let active_match_detail = if let (Some(tl_state), Some(item_index)) =
            (self.tl_state.as_ref(), active_loaded_index)
        {
            tl_state
                .items
                .get(item_index)
                .and_then(|item| item.as_event())
                .map(|event_tl_item| {
                    (
                        event_tl_item.event_id().map(ToString::to_string),
                        plaintext_body_of_timeline_item(event_tl_item),
                        loaded_message_search_thread_root_event_id(event_tl_item),
                    )
                })
        } else {
            None
        };
        let active_event_id_loaded = active_match_detail
            .as_ref()
            .and_then(|(event_id, _, _)| event_id.as_ref())
            .is_some();
        let loaded_body = active_match_detail
            .as_ref()
            .map(|(_, body, _)| body.as_str());
        let thread_root_event_id = active_match_detail
            .as_ref()
            .and_then(|(_, _, thread_root_event_id)| thread_root_event_id.clone());
        let opened = if let (Some(room_name_id), Some(thread_root_event_id)) = (
            self.room_name_id.as_ref().cloned(),
            thread_root_event_id.clone(),
        ) {
            cx.widget_action(
                self.widget_uid(),
                RoomsListAction::Selected(SelectedRoom::Thread {
                    room_name_id,
                    thread_root_event_id,
                }),
            );
            true
        } else {
            false
        };
        let metadata = loaded_message_search_result_thread_open_label(
            query,
            loaded_item_count,
            match_count,
            self.telegram_message_search_active_match,
            active_loaded_index,
            active_event_id_loaded,
            loaded_body,
            thread_root_event_id
                .as_ref()
                .map(|event_id| event_id.as_str()),
            opened,
        );
        self.telegram_message_search_result_action_controls_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_result_action_controls_metadata),
            )
            .set_text(cx, &metadata);
        self.view
        .label(
            cx,
            ids!(telegram_message_search_strip.server_context_boundary),
        )
        .set_text(
            cx,
            &format!(
                "Thread result action opened loaded thread root when available. {MESSAGE_SEARCH_RESULT_THREAD_OPEN_LABEL}"
            ),
    );
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn open_telegram_message_search_active_sender_profile(
        &mut self,
        cx: &mut Cx,
        pane: &UserProfileSlidingPaneRef,
    ) {
        self.set_telegram_search_mode_visible(cx, true);
        self.refresh_telegram_message_search_matches(cx);
        let query = self.telegram_message_search_query.trim().to_string();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let match_count = self.telegram_message_search_matches.len();
        let active_loaded_index = self
            .telegram_message_search_matches
            .get(self.telegram_message_search_active_match)
            .copied();
        let active_sender_detail = if let (Some(tl_state), Some(item_index)) =
            (self.tl_state.as_ref(), active_loaded_index)
        {
            tl_state
                .items
                .get(item_index)
                .and_then(|item| item.as_event())
                .map(|event_tl_item| {
                    let sender_id = event_tl_item.sender().to_owned();
                    let (display_name, avatar_state, sender_profile_ready) =
                        match event_tl_item.sender_profile() {
                            TimelineDetails::Ready(profile) => (
                                profile.display_name.clone(),
                                AvatarState::Known(profile.avatar_url.clone()),
                                true,
                            ),
                            _ => (None, AvatarState::Unknown, false),
                        };
                    let room_member = tl_state.room_members.as_ref().and_then(|members| {
                        members
                            .iter()
                            .find(|member| member.user_id() == event_tl_item.sender())
                            .cloned()
                    });
                    let room_member_loaded = room_member.is_some();
                    let profile_info = UserProfilePaneInfo {
                        profile_and_room_id: UserProfileAndRoomId {
                            user_profile: UserProfile {
                                user_id: sender_id.clone(),
                                username: display_name.clone(),
                                avatar_state,
                            },
                            room_id: tl_state.kind.room_id().clone(),
                        },
                        room_name: self
                            .room_name_id
                            .as_ref()
                            .map_or_else(|| UNNAMED_ROOM.to_string(), |r| r.to_string()),
                        room_member,
                    };
                    (
                        profile_info,
                        sender_id.to_string(),
                        display_name,
                        sender_profile_ready,
                        room_member_loaded,
                        event_tl_item.event_id().is_some(),
                        plaintext_body_of_timeline_item(event_tl_item),
                    )
                })
        } else {
            None
        };
        let opened = if let Some((profile_info, ..)) = active_sender_detail.clone() {
            self.show_user_profile(cx, pane, profile_info);
            true
        } else {
            false
        };
        let metadata = loaded_message_search_result_sender_profile_pane_label(
            &query,
            loaded_item_count,
            match_count,
            self.telegram_message_search_active_match,
            active_loaded_index,
            active_sender_detail
                .as_ref()
                .map(|(_, _, _, _, _, event_id_loaded, _)| *event_id_loaded)
                .unwrap_or(false),
            active_sender_detail
                .as_ref()
                .map(|(_, _, _, _, _, _, body)| body.as_str()),
            active_sender_detail
                .as_ref()
                .map(|(_, sender_id, _, _, _, _, _)| sender_id.as_str()),
            active_sender_detail
                .as_ref()
                .and_then(|(_, _, display_name, _, _, _, _)| display_name.as_deref()),
            active_sender_detail
                .as_ref()
                .map(|(_, _, _, sender_profile_ready, _, _, _)| *sender_profile_ready)
                .unwrap_or(false),
            active_sender_detail
                .as_ref()
                .map(|(_, _, _, _, room_member_loaded, _, _)| *room_member_loaded)
                .unwrap_or(false),
            opened,
        );
        self.telegram_message_search_result_action_controls_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_result_action_controls_metadata),
            )
            .set_text(cx, &metadata);
        self.view
        .label(
            cx,
            ids!(telegram_message_search_strip.server_context_boundary),
        )
        .set_text(
            cx,
            &format!(
                "Sender result action opened the existing profile pane when available. {MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_LABEL}"
            ),
        );
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }

    fn jump_telegram_message_search_match(
        &mut self,
        cx: &mut Cx,
        portal_list: &PortalListRef,
        direction: isize,
    ) {
        self.refresh_telegram_message_search_matches(cx);
        let match_count = self.telegram_message_search_matches.len();
        let query = self.telegram_message_search_query.trim();
        if query.is_empty() {
            enqueue_popup_notification(
                "Type a local query before jumping between message search matches.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }
        if match_count == 0 {
            enqueue_popup_notification(
                format!(
                    "No loaded local messages matched this query. {MESSAGE_SEARCH_COMPACT_LABEL}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }

        if direction < 0 {
            self.telegram_message_search_active_match =
                (self.telegram_message_search_active_match + match_count - 1) % match_count;
        } else if direction > 0 {
            self.telegram_message_search_active_match =
                (self.telegram_message_search_active_match + 1) % match_count;
        }
        let item_index =
            self.telegram_message_search_matches[self.telegram_message_search_active_match];
        portal_list.set_first_id_and_scroll(item_index, 15.0);
        self.update_telegram_message_search_labels(cx);
        let preview = self
            .telegram_message_search_active_match_preview()
            .unwrap_or_else(|| "preview unavailable".to_string());
        enqueue_popup_notification(
            format!(
                "Moved to local match {} of {}. Preview: {}. {MESSAGE_SEARCH_COMPACT_LABEL}",
                self.telegram_message_search_active_match + 1,
                match_count,
                preview
            ),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn telegram_message_search_active_match_preview(&self) -> Option<String> {
        let tl_state = self.tl_state.as_ref()?;
        let item_index = *self
            .telegram_message_search_matches
            .get(self.telegram_message_search_active_match)?;
        let event_tl_item = tl_state.items.get(item_index)?.as_event()?;
        Some(compact_loaded_message_search_preview(
            &plaintext_body_of_timeline_item(event_tl_item),
        ))
    }

    fn show_telegram_room_info(&mut self, cx: &mut Cx, room_label: &str) {
        self.refresh_telegram_room_action_details(cx);
        self.update_telegram_room_info_strip(cx, room_label);
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_message_edit_history_visible(cx, false);
        self.set_telegram_message_report_status_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, false, None);
        self.set_telegram_notifications_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, false);
        self.set_telegram_matrix_link_preview_visible(cx, false);
        self.set_telegram_room_info_visible(cx, true);
    }

    fn update_telegram_room_header(&mut self, cx: &mut Cx, room_name_id: &RoomNameId) {
        let title = if room_name_id.is_empty() {
            "Chat".to_string()
        } else {
            room_name_id.to_string()
        };
        self.view
            .label(cx, ids!(telegram_room_header.title_stack.title))
            .set_text(cx, &title);
        self.view
            .label(cx, ids!(telegram_room_header.title_stack.status))
            .set_text(cx, "local chat ready");
    }

    fn set_telegram_search_mode_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_message_search_visible == visible {
            return;
        }
        self.telegram_message_search_visible = visible;
        self.view
            .view(cx, ids!(telegram_message_search_strip))
            .set_visible(cx, visible);
        if visible {
            self.set_telegram_message_edit_history_visible(cx, false);
            self.set_telegram_message_report_status_visible(cx, false);
            self.set_telegram_room_info_visible(cx, false);
            self.set_telegram_notifications_visible(cx, false);
            self.set_telegram_room_settings_visible(cx, false);
            self.set_telegram_matrix_link_preview_visible(cx, false);
            self.refresh_telegram_message_search_matches(cx);
            self.view
                .text_input(
                    cx,
                    ids!(telegram_message_search_strip.search_row.search_input),
                )
                .set_key_focus(cx);
        }
    }

    fn set_telegram_room_actions_visible(
        &mut self,
        cx: &mut Cx,
        visible: bool,
        status: Option<&str>,
    ) {
        if let Some(status) = status {
            self.view
                .label(cx, ids!(telegram_room_actions_strip.room_action_status))
                .set_text(cx, status);
        }
        if self.telegram_room_actions_visible == visible {
            return;
        }
        self.telegram_room_actions_visible = visible;
        self.view
            .view(cx, ids!(telegram_room_actions_strip))
            .set_visible(cx, visible);
    }

    fn show_telegram_room_actions(&mut self, cx: &mut Cx, status: &str) {
        self.refresh_telegram_room_action_details(cx);
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_message_edit_history_visible(cx, false);
        self.set_telegram_message_report_status_visible(cx, false);
        self.set_telegram_room_info_visible(cx, false);
        self.set_telegram_notifications_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, false);
        self.set_telegram_matrix_link_preview_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, true, Some(status));
    }

    fn handle_telegram_room_header_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        portal_list: &PortalListRef,
        user_profile_sliding_pane: &UserProfileSlidingPaneRef,
    ) {
        if self
            .view
            .button(cx, ids!(telegram_room_info_strip.close_info_button))
            .clicked(actions)
        {
            self.set_telegram_room_info_visible(cx, false);
            enqueue_popup_notification(
                "Telegram room info preview closed. No room setting was changed.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }

        let search_input = self.view.text_input(
            cx,
            ids!(telegram_message_search_strip.search_row.search_input),
        );
        if let Some(query) = search_input.changed(actions) {
            self.set_telegram_message_search_query(cx, query);
        }
        if search_input.escaped(actions) {
            self.set_telegram_search_mode_visible(cx, false);
            enqueue_popup_notification(
                "Telegram message search preview closed with Escape. No Matrix search query was sent.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }
        if search_input.returned(actions).is_some() {
            self.jump_telegram_message_search_match(cx, portal_list, 0);
            return;
        }

        let search_from_input = self.view.text_input(
            cx,
            ids!(
                telegram_message_search_strip
                    .search_advanced_filter_controls
                    .search_from_input
            ),
        );
        if let Some(sender) = search_from_input.changed(actions) {
            self.telegram_message_search_sender_filter_draft = sender;
        }
        if search_from_input.returned(actions).is_some() {
            self.submit_telegram_message_search_sender_filter(cx);
            return;
        }
        if search_from_input.escaped(actions) {
            self.telegram_message_search_sender_filter_draft.clear();
            search_from_input.set_text(cx, "");
            enqueue_popup_notification(
                "Message search From sender filter cleared. No Matrix search request was sent.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_header
                        .retry_edit_history_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_message_edit_history_retry_confirmation(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_full_controls
                        .full_history_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_message_edit_history_local_full_snapshot_modal(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_full_controls
                        .full_diff_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_edit_history_loaded_diff(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_full_controls
                        .event_context_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_edit_history_full_control(cx, "Event context");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_full_controls
                        .event_source_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_message_edit_history_loaded_source(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_full_controls
                        .edit_history_packet_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_edit_history_full_diff_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_full_controls
                        .edit_history_contract_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_edit_history_full_history_result_contract_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_full_controls
                        .edit_history_taxonomy_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_edit_history_remote_result_taxonomy_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_preflight_controls
                        .edit_history_request_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_edit_history_preflight_detail_control(cx, "Request");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_preflight_controls
                        .edit_history_result_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_edit_history_preflight_detail_control(cx, "Result");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_preflight_controls
                        .edit_history_error_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_edit_history_preflight_detail_control(cx, "Error");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_preflight_controls
                        .edit_history_retry_detail_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_edit_history_preflight_detail_control(cx, "Retry");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_preflight_controls
                        .edit_history_source_detail_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_edit_history_preflight_detail_control(cx, "Source");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_result_action_controls
                        .search_result_jump_button
                ),
            )
            .clicked(actions)
        {
            self.jump_telegram_message_search_active_match(cx, portal_list);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_result_action_controls
                        .search_result_copy_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_search_result_action_control(cx, "Copy");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_result_action_controls
                        .search_result_source_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_search_result_action_control(cx, "Source");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_result_action_controls
                        .search_result_thread_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_message_search_active_thread(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_result_action_controls
                        .search_result_sender_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_message_search_active_sender_profile(cx, user_profile_sliding_pane);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_context_controls
                        .search_server_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_message_search_server_request(
                cx,
                None,
                MessageSearchServerFilter::default(),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_context_controls
                        .search_event_context_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_message_search_server_context_event(cx, portal_list);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_context_controls
                        .search_load_older_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_message_search_server_next_page(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_advanced_filter_controls
                        .search_filter_button
                ),
            )
            .clicked(actions)
        {
            self.apply_telegram_message_search_loaded_scope_control(
                cx,
                "Filter",
                MessageSearchLoadedScope::AllLoaded,
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_advanced_filter_controls
                        .search_from_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_message_search_sender_filter(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_advanced_filter_controls
                        .search_date_button
                ),
            )
            .clicked(actions)
        {
            self.apply_telegram_message_search_loaded_scope_control(
                cx,
                "Date",
                MessageSearchLoadedScope::LatestLoadedDay,
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_advanced_filter_controls
                        .search_media_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_message_search_media_filter(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_advanced_filter_controls
                        .search_pins_button
                ),
            )
            .clicked(actions)
        {
            self.apply_telegram_message_search_loaded_scope_control(
                cx,
                "Pins",
                MessageSearchLoadedScope::PinnedLoaded,
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_preflight_controls
                        .search_server_query_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_message_search_server_request(
                cx,
                None,
                MessageSearchServerFilter::default(),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_preflight_controls
                        .search_server_packet_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_search_server_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_preflight_controls
                        .search_server_contract_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_search_server_preflight_control(cx, "Contract");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_preflight_controls
                        .search_server_result_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_search_server_preflight_control(cx, "Result");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_preflight_controls
                        .search_server_error_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_search_server_preflight_control(cx, "Error");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_preflight_controls
                        .search_server_retry_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_message_search_server_request(
                cx,
                None,
                self.telegram_message_search_server_last_filter.clone(),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_preflight_controls
                        .search_server_scope_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_search_server_preflight_control(cx, "Scope");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_server_preflight_controls
                        .search_server_taxonomy_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_search_server_preflight_control(cx, "Taxonomy");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_row
                        .prev_search_result_button
                ),
            )
            .clicked(actions)
        {
            self.jump_telegram_message_search_match(cx, portal_list, -1);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_search_strip
                        .search_row
                        .next_search_result_button
                ),
            )
            .clicked(actions)
        {
            self.jump_telegram_message_search_match(cx, portal_list, 1);
            return;
        }

        if self
            .view
            .button(
                cx,
                ids!(telegram_message_search_strip.search_row.close_search_button),
            )
            .clicked(actions)
        {
            self.set_telegram_search_mode_visible(cx, false);
            enqueue_popup_notification(
                "Telegram message search preview closed. No Matrix search query was sent.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_edit_history_strip
                        .edit_history_header
                        .close_edit_history_button
                ),
            )
            .clicked(actions)
        {
            self.set_telegram_message_edit_history_visible(cx, false);
            enqueue_popup_notification(
                "Telegram edit history detail closed. No event context fetch, timeline reload, event source open, or mutation was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_status_header
                        .retry_report_status_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_message_report_retry_confirmation(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_workflow_actions
                        .report_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_workflow_action(cx, "Moderation queue");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_workflow_actions
                        .report_policy_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_workflow_action(cx, "Server policy lookup");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_workflow_actions
                        .report_assign_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_workflow_action(cx, "Reviewer assignment");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_workflow_actions
                        .report_appeal_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_workflow_action(cx, "Appeal workflow");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_workflow_actions
                        .report_enforce_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_workflow_action(cx, "Enforcement tools");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_workflow_actions
                        .report_moderation_packet_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_report_moderation_reviewer_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_workflow_actions
                        .report_workflow_contract_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_report_workflow_result_contract_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_workflow_actions
                        .report_workflow_taxonomy_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_report_workflow_result_taxonomy_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_preflight_detail_controls
                        .report_request_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_preflight_detail_control(cx, "Request");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_preflight_detail_controls
                        .report_result_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_preflight_detail_control(cx, "Result");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_preflight_detail_controls
                        .report_copy_status_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_message_report_status_summary(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_preflight_detail_controls
                        .report_error_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_preflight_detail_control(cx, "Error");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_preflight_detail_controls
                        .report_retry_detail_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_message_report_preflight_detail_control(cx, "Retry");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_preflight_detail_controls
                        .report_source_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_message_report_loaded_source(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_route_scope_controls
                        .matrix_link_route_room_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_matrix_link_room_target(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_route_scope_controls
                        .matrix_link_route_event_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_matrix_link_event_id(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_route_scope_controls
                        .matrix_link_route_via_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_matrix_link_via_servers(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_route_scope_controls
                        .matrix_link_route_preview_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_matrix_link_preview_metadata(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_route_scope_controls
                        .matrix_link_route_source_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_matrix_link_loaded_event_source(cx, "Source route");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_route_scope_controls
                        .matrix_link_route_packet_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_matrix_link_route_drilldown_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_route_scope_controls
                        .matrix_link_route_contract_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_matrix_link_route_result_contract_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_route_scope_controls
                        .matrix_link_route_taxonomy_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_matrix_link_route_result_taxonomy_packet(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_context_actions
                        .matrix_link_server_button
                ),
            )
            .clicked(actions)
        {
            self.refresh_telegram_matrix_link_server_context(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_context_actions
                        .matrix_link_event_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_matrix_link_context_action(cx, "Event context");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_context_actions
                        .matrix_link_alias_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_matrix_link_context_action(cx, "Alias retry");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_context_actions
                        .matrix_link_join_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_matrix_link_join_confirmation(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_context_actions
                        .matrix_link_knock_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_matrix_link_knock_confirmation(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_context_actions
                        .matrix_link_invite_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_matrix_link_invite_confirmation(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_context_actions
                        .matrix_link_browser_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_matrix_link_browser_confirmation(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_context_actions
                        .matrix_link_source_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_matrix_link_loaded_event_source(cx, "Event source");
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_preview_header
                        .retry_matrix_link_preview_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_matrix_link_preview_retry_confirmation(cx);
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_matrix_link_preview_strip
                        .matrix_link_preview_header
                        .close_matrix_link_preview_button
                ),
            )
            .clicked(actions)
        {
            self.set_telegram_matrix_link_preview_visible(cx, false);
            enqueue_popup_notification(
                "Matrix link preview closed. No retry, event context fetch, join, knock, browser handoff, room-state, membership, or mutation was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_message_report_status_strip
                        .report_status_header
                        .close_report_status_button
                ),
            )
            .clicked(actions)
        {
            self.set_telegram_message_report_status_visible(cx, false);
            enqueue_popup_notification(
                "Telegram report status closed. No retry, cancel queue, moderation lookup, room-state, membership, or mutation was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(telegram_room_actions_strip.close_room_actions_button),
            )
            .clicked(actions)
        {
            self.set_telegram_room_actions_visible(cx, false, None);
            enqueue_popup_notification(
                "Telegram room actions preview closed. No room setting or notification mode write was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notifications_header
                        .close_notifications_button
                ),
            )
            .clicked(actions)
        {
            // Notifications evidence: Close hides the strip without a mode write.
            let room_label = self
                .room_name_id
                .as_ref()
                .map(|room_name_id| {
                    if room_name_id.is_empty() {
                        "this chat".to_string()
                    } else {
                        room_name_id.to_string()
                    }
                })
                .unwrap_or_else(|| "this chat".to_string());
            let close_metadata =
                self.telegram_notifications_close_refresh_metadata_summary(&room_label, "close");
            self.set_telegram_notifications_visible(cx, false);
            enqueue_popup_notification(close_metadata, PopupKind::Info, Some(4.0));
            return;
        }
        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_header
                        .close_settings_button
                ),
            )
            .clicked(actions)
        {
            let room_label = self
                .room_name_id
                .as_ref()
                .map(|room_name_id| {
                    if room_name_id.is_empty() {
                        "this chat".to_string()
                    } else {
                        room_name_id.to_string()
                    }
                })
                .unwrap_or_else(|| "this chat".to_string());
            let close_metadata = self.telegram_room_settings_close_metadata_summary(&room_label);
            self.set_telegram_room_settings_visible(cx, false);
            enqueue_popup_notification(close_metadata, PopupKind::Info, Some(4.0));
            return;
        }

        let Some(room_name_id) = self.room_name_id.as_ref() else {
            return;
        };
        let room_label = if room_name_id.is_empty() {
            "this chat".to_string()
        } else {
            room_name_id.to_string()
        };
        let room_name_id_for_modal = room_name_id.clone();
        let room_id = room_name_id.room_id().clone();

        let keyword_rule_input = self.view.text_input(
            cx,
            ids!(
                telegram_notifications_strip
                    .notification_keyword_write_row
                    .keyword_rule_input
            ),
        );
        if let Some(value) = keyword_rule_input.changed(actions) {
            self.telegram_notifications_keyword_draft = value;
        }
        if keyword_rule_input.returned(actions).is_some() {
            self.show_telegram_notification_keyword_confirmation(
                cx,
                &room_label,
                &keyword_rule_input.text(),
                NotificationKeywordMutation::Add,
            );
            return;
        }
        if keyword_rule_input.escaped(actions) {
            self.telegram_notifications_keyword_draft.clear();
            keyword_rule_input.set_text(cx, "");
            enqueue_popup_notification(
                "Notification keyword draft cleared. No keyword rule write was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }

        let name_live_input = self.view.text_input(
            cx,
            ids!(
                telegram_room_settings_strip
                    .settings_name_write_row
                    .name_live_input
            ),
        );
        if let Some(value) = name_live_input.changed(actions) {
            self.telegram_room_settings_name_draft = value;
        }
        if name_live_input.returned(actions).is_some() {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::Name,
                name_live_input.text(),
            );
            return;
        }
        if name_live_input.escaped(actions) {
            self.telegram_room_settings_name_draft.clear();
            name_live_input.set_text(cx, "");
            enqueue_popup_notification(
                "Room name draft cleared. No room-state write was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }

        let topic_live_input = self.view.text_input(
            cx,
            ids!(
                telegram_room_settings_strip
                    .settings_topic_write_row
                    .topic_live_input
            ),
        );
        if let Some(value) = topic_live_input.changed(actions) {
            self.telegram_room_settings_topic_draft = value;
        }
        if topic_live_input.returned(actions).is_some() {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::Topic,
                topic_live_input.text(),
            );
            return;
        }
        if topic_live_input.escaped(actions) {
            self.telegram_room_settings_topic_draft.clear();
            topic_live_input.set_text(cx, "");
            enqueue_popup_notification(
                "Room topic draft cleared. No room-state write was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }

        let alias_live_input = self.view.text_input(
            cx,
            ids!(
                telegram_room_settings_strip
                    .settings_alias_write_row
                    .alias_live_input
            ),
        );
        if let Some(value) = alias_live_input.changed(actions) {
            self.telegram_room_settings_alias_draft = value;
        }
        if alias_live_input.returned(actions).is_some() {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::CanonicalAlias,
                alias_live_input.text(),
            );
            return;
        }
        if alias_live_input.escaped(actions) {
            self.telegram_room_settings_alias_draft.clear();
            alias_live_input.set_text(cx, "");
            enqueue_popup_notification(
                "Room canonical alias draft cleared. No room-state write was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }

        let tombstone_replacement_live_input = self.view.text_input(
            cx,
            ids!(
                telegram_room_settings_strip
                    .settings_tombstone_write_row
                    .tombstone_replacement_live_input
            ),
        );
        if let Some(value) = tombstone_replacement_live_input.changed(actions) {
            self.telegram_room_settings_tombstone_replacement_draft = value;
        }
        if tombstone_replacement_live_input.returned(actions).is_some() {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::Tombstone,
                tombstone_replacement_live_input.text(),
            );
            return;
        }
        if tombstone_replacement_live_input.escaped(actions) {
            self.telegram_room_settings_tombstone_replacement_draft
                .clear();
            tombstone_replacement_live_input.set_text(cx, "");
            enqueue_popup_notification(
                "Room replacement draft cleared. No m.room.tombstone write was requested.",
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }

        if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_name_write_row
                        .save_name_live_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::Name,
                name_live_input.text(),
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_topic_write_row
                        .save_topic_live_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::Topic,
                topic_live_input.text(),
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_alias_write_row
                        .save_alias_live_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::CanonicalAlias,
                alias_live_input.text(),
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_tombstone_write_row
                        .save_tombstone_live_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::Tombstone,
                tombstone_replacement_live_input.text(),
            );
            return;
        }

        if self
            .view
            .button(
                cx,
                ids!(telegram_room_settings_strip.settings_options.name_button),
            )
            .clicked(actions)
        {
            self.copy_telegram_room_settings_name_id(cx, &room_label, room_id.as_str());
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(telegram_room_settings_strip.settings_options.topic_button),
            )
            .clicked(actions)
        {
            let status = self.telegram_room_settings_topic_summary();
            self.stage_telegram_room_settings_choice(cx, &room_label, &status);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_options
                        .permissions_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_room_settings_permissions(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(telegram_room_settings_strip.settings_options.members_button),
            )
            .clicked(actions)
        {
            self.copy_telegram_room_settings_members(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_options
                        .identity_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_room_settings_identity(cx, &room_label, room_id.as_str());
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(telegram_room_settings_strip.settings_options.avatar_button),
            )
            .clicked(actions)
        {
            self.open_telegram_room_settings_avatar_upload_picker(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(telegram_room_settings_strip.settings_options.alias_button),
            )
            .clicked(actions)
        {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::CanonicalAlias,
                alias_live_input.text(),
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(telegram_room_settings_strip.settings_options.history_button),
            )
            .clicked(actions)
        {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::HistoryVisibility,
                "shared".to_string(),
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_options
                        .join_rule_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::JoinRule,
                "invite".to_string(),
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(telegram_room_settings_strip.settings_options.power_button),
            )
            .clicked(actions)
        {
            let status = self.telegram_room_settings_edit_intent_summary(&room_label, "Power");
            self.stage_telegram_room_settings_choice(cx, &room_label, &status);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_options
                        .moderation_button
                ),
            )
            .clicked(actions)
        {
            let status = self.telegram_room_settings_edit_intent_summary(&room_label, "Moderation");
            self.stage_telegram_room_settings_choice(cx, &room_label, &status);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_field_edit_intents
                        .name_edit_button
                ),
            )
            .clicked(actions)
        {
            let status =
                self.telegram_room_settings_field_edit_intent_summary(&room_label, "Name edit");
            self.stage_telegram_room_settings_choice(cx, &room_label, &status);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_field_edit_intents
                        .topic_edit_button
                ),
            )
            .clicked(actions)
        {
            let status =
                self.telegram_room_settings_field_edit_intent_summary(&room_label, "Topic edit");
            self.stage_telegram_room_settings_choice(cx, &room_label, &status);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_field_edit_intents
                        .avatar_edit_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_room_settings_avatar_upload_picker(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_field_edit_intents
                        .remove_avatar_live_button
                ),
            )
            .clicked(actions)
        {
            self.submit_telegram_room_settings_live_write(
                cx,
                &room_label,
                RoomSettingsMutationField::Avatar,
                "remove avatar".to_string(),
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_field_edit_intents
                        .permissions_edit_button
                ),
            )
            .clicked(actions)
        {
            let status = self
                .telegram_room_settings_field_edit_intent_summary(&room_label, "Permissions edit");
            self.stage_telegram_room_settings_choice(cx, &room_label, &status);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_field_edit_intents
                        .members_edit_button
                ),
            )
            .clicked(actions)
        {
            let status =
                self.telegram_room_settings_field_edit_intent_summary(&room_label, "Members edit");
            self.stage_telegram_room_settings_choice(cx, &room_label, &status);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_refresh_result_controls
                        .result_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_refresh_result_detail(cx, &room_label, "Result");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_refresh_result_controls
                        .members_result_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_refresh_result_detail(cx, &room_label, "Members");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_refresh_result_controls
                        .power_result_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_refresh_result_detail(cx, &room_label, "Power");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_refresh_result_controls
                        .failure_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_refresh_result_detail(cx, &room_label, "Failure");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_refresh_result_controls
                        .source_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_refresh_result_detail(cx, &room_label, "Source");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_mutation_preflight_controls
                        .request_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_mutation_preflight_detail(cx, &room_label, "Request");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_mutation_preflight_controls
                        .packet_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_room_settings_field_mutation_packet(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_mutation_preflight_controls
                        .contract_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_room_settings_field_mutation_contract_packet(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_mutation_preflight_controls
                        .taxonomy_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_room_settings_power_member_result_taxonomy_packet(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_mutation_preflight_controls
                        .result_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_mutation_preflight_detail(cx, &room_label, "Result");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_mutation_preflight_controls
                        .error_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_mutation_preflight_detail(cx, &room_label, "Error");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_mutation_preflight_controls
                        .retry_button
                ),
            )
            .clicked(actions)
        {
            if self.show_telegram_room_settings_mutation_retry_confirmation(cx, &room_label) {
                return;
            }
            self.stage_telegram_room_settings_mutation_preflight_detail(cx, &room_label, "Retry");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_mutation_preflight_controls
                        .source_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_room_settings_mutation_preflight_detail(cx, &room_label, "Source");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_room_settings_strip
                        .settings_options
                        .refresh_settings_button
                ),
            )
            .clicked(actions)
        {
            self.refresh_telegram_room_settings_read_paths(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notifications_header
                        .retry_notifications_button
                ),
            )
            .clicked(actions)
        {
            if self
                .telegram_notifications_local_status
                .starts_with("Keyword update failed:")
                && self.telegram_notifications_retry_keyword_mutation.is_some()
                && !self.telegram_notifications_retry_keyword.trim().is_empty()
            {
                self.show_telegram_notification_keyword_retry_confirmation(cx, &room_label);
            } else if self
                .telegram_notifications_local_status
                .starts_with("Default update failed:")
                && self.telegram_notifications_default_mode_retry_cache_ready()
            {
                self.show_telegram_notification_default_room_mode_retry_confirmation(
                    cx,
                    &room_label,
                );
            } else {
                self.show_telegram_notification_mode_retry_confirmation(cx, &room_label);
            }
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_controls
                        .timed_mute_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_advanced_control(cx, &room_label, "Timed mute");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_controls
                        .keyword_rules_button
                ),
            )
            .clicked(actions)
        {
            self.read_telegram_notification_keyword_rules(cx, &room_label, "Keyword rules");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_controls
                        .pusher_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_advanced_control(cx, &room_label, "Pusher setup");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_controls
                        .global_preferences_button
                ),
            )
            .clicked(actions)
        {
            self.read_telegram_notification_default_room_mode(cx, &room_label, "Global defaults");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_keyword_write_row
                        .add_keyword_rule_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_notification_keyword_confirmation(
                cx,
                &room_label,
                &keyword_rule_input.text(),
                NotificationKeywordMutation::Add,
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_keyword_write_row
                        .remove_keyword_rule_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_notification_keyword_confirmation(
                cx,
                &room_label,
                &keyword_rule_input.text(),
                NotificationKeywordMutation::Remove,
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_default_mode_write_row
                        .default_all_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_notification_default_room_mode_confirmation(
                cx,
                &room_label,
                RoomNotificationMode::AllMessages,
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_default_mode_write_row
                        .default_mentions_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_notification_default_room_mode_confirmation(
                cx,
                &room_label,
                RoomNotificationMode::MentionsAndKeywordsOnly,
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_default_mode_write_row
                        .default_mute_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_notification_default_room_mode_confirmation(
                cx,
                &room_label,
                RoomNotificationMode::Mute,
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_detail_controls
                        .quiet_hours_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_advanced_detail_control(
                cx,
                &room_label,
                "Quiet hours",
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_detail_controls
                        .keyword_list_button
                ),
            )
            .clicked(actions)
        {
            self.read_telegram_notification_keyword_rules(cx, &room_label, "Keyword list");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_detail_controls
                        .device_push_button
                ),
            )
            .clicked(actions)
        {
            self.read_telegram_notification_pusher_status(cx, &room_label, "Device push");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_detail_controls
                        .defaults_button
                ),
            )
            .clicked(actions)
        {
            self.read_telegram_notification_default_room_mode(cx, &room_label, "Defaults");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_advanced_detail_controls
                        .sound_badge_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_advanced_detail_control(
                cx,
                &room_label,
                "Sound badge",
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_result_detail_controls
                        .result_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_result_detail_control(cx, &room_label, "Result");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_result_detail_controls
                        .requested_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_result_detail_control(cx, &room_label, "Requested");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_result_detail_controls
                        .retry_cache_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_result_detail_control(cx, &room_label, "Retry cache");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_result_detail_controls
                        .failure_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_result_detail_control(cx, &room_label, "Failure");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_result_detail_controls
                        .source_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_result_detail_control(cx, &room_label, "Source");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_preflight_detail_controls
                        .schedule_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_preflight_detail_control(cx, &room_label, "Schedule");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_preflight_detail_controls
                        .packet_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_notifications_rule_packet(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_preflight_detail_controls
                        .contract_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_notifications_rule_contract_packet(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_preflight_detail_controls
                        .taxonomy_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_notifications_result_taxonomy_packet(cx, &room_label);
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_preflight_detail_controls
                        .account_data_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_notifications_preflight_detail_control(
                cx,
                &room_label,
                "Account data",
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_preflight_detail_controls
                        .keyword_source_button
                ),
            )
            .clicked(actions)
        {
            self.read_telegram_notification_keyword_rules(cx, &room_label, "Keywords");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_preflight_detail_controls
                        .pushers_button
                ),
            )
            .clicked(actions)
        {
            self.read_telegram_notification_pusher_status(cx, &room_label, "Pushers");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_preflight_detail_controls
                        .preflight_defaults_button
                ),
            )
            .clicked(actions)
        {
            self.read_telegram_notification_default_room_mode(cx, &room_label, "Defaults");
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_options
                        .mute_1h_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_notification_mode_confirmation(
                cx,
                &room_label,
                room_id.clone(),
                RoomNotificationMode::AllMessages,
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_options
                        .mute_8h_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_notification_mode_confirmation(
                cx,
                &room_label,
                room_id.clone(),
                RoomNotificationMode::MentionsAndKeywordsOnly,
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_options
                        .mute_forever_button
                ),
            )
            .clicked(actions)
        {
            self.show_telegram_notification_mode_confirmation(
                cx,
                &room_label,
                room_id.clone(),
                RoomNotificationMode::Mute,
            );
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_options
                        .unmute_button
                ),
            )
            .clicked(actions)
        {
            if let Some(tl_state) = self.tl_state.as_ref() {
                submit_async_request(MatrixRequest::GetRoomNotificationMode {
                    timeline_kind: tl_state.kind.clone(),
                });
                self.telegram_notifications_local_status = "Refreshing current mode".to_string();
                self.telegram_notifications_result_detail_action = "Source".to_string();
                self.update_telegram_notifications_strip(cx, &room_label);
                let refresh_metadata = self
                    .telegram_notifications_close_refresh_metadata_summary(&room_label, "refresh");
                enqueue_popup_notification(refresh_metadata, PopupKind::Info, Some(4.0));
            } else {
                enqueue_popup_notification(
                    format!(
                        "Notification mode for {room_label} is not available until the room timeline is loaded."
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
            }
            return;
        } else if self
            .view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notification_options
                        .copy_mode_button
                ),
            )
            .clicked(actions)
        {
            self.copy_telegram_notifications_mode_summary(cx, &room_label);
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.copy_link_button))
            .clicked(actions)
        {
            submit_async_request(MatrixRequest::GenerateMatrixLink {
                room_id: room_id.clone(),
                event_id: None,
                use_matrix_scheme: false,
                join_on_click: false,
            });
            self.set_telegram_room_actions_visible(cx, false, None);
            enqueue_popup_notification(
                format!(
                    "Room link requested for {room_label}. {ROOM_LINK_INVITE_LEAVE_COMPACT_LABEL}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.invite_button))
            .clicked(actions)
        {
            cx.action(InviteModalAction::Open(room_name_id_for_modal.clone()));
            self.set_telegram_room_actions_visible(cx, false, None);
            enqueue_popup_notification(
                format!("Invite opened for {room_label}. {ROOM_LINK_INVITE_LEAVE_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(3.0),
            );
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.leave_button))
            .clicked(actions)
        {
            cx.action(JoinLeaveRoomModalAction::Open {
                kind: JoinLeaveModalKind::LeaveRoom(BasicRoomDetails::Name(
                    room_name_id_for_modal.clone(),
                )),
                show_tip: true,
            });
            self.set_telegram_room_actions_visible(cx, false, None);
            enqueue_popup_notification(
                format!(
                    "Leave confirmation opened for {room_label}. {ROOM_LINK_INVITE_LEAVE_COMPACT_LABEL}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.mark_unread_button))
            .clicked(actions)
        {
            self.refresh_telegram_room_action_details(cx);
            let Some(details) = self.telegram_room_action_details.as_ref() else {
                self.warn_missing_telegram_room_action_state(cx, &room_label);
                return;
            };
            let mark_as_unread = !details.is_marked_unread;
            self.show_telegram_room_status_confirmation(
                cx,
                &room_label,
                "Unread Flag",
                if mark_as_unread {
                    "mark as unread"
                } else {
                    "mark as read"
                },
                if mark_as_unread {
                    "Mark Unread"
                } else {
                    "Mark Read"
                },
                MatrixRequest::SetUnreadFlag {
                    room_id: room_id.clone(),
                    mark_as_unread,
                },
            );
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.favorite_button))
            .clicked(actions)
        {
            self.refresh_telegram_room_action_details(cx);
            let Some(details) = self.telegram_room_action_details.as_ref() else {
                self.warn_missing_telegram_room_action_state(cx, &room_label);
                return;
            };
            let is_favorite = !details.is_favorite;
            self.show_telegram_room_status_confirmation(
                cx,
                &room_label,
                "Favorite",
                if is_favorite {
                    "favorite"
                } else {
                    "remove favorite"
                },
                if is_favorite {
                    "Favorite"
                } else {
                    "Remove Favorite"
                },
                MatrixRequest::SetIsFavorite {
                    room_id: room_id.clone(),
                    is_favorite,
                },
            );
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.priority_button))
            .clicked(actions)
        {
            self.refresh_telegram_room_action_details(cx);
            let Some(details) = self.telegram_room_action_details.as_ref() else {
                self.warn_missing_telegram_room_action_state(cx, &room_label);
                return;
            };
            let is_low_priority = !details.is_low_priority;
            self.show_telegram_room_status_confirmation(
                cx,
                &room_label,
                "Priority",
                if is_low_priority {
                    "set low priority"
                } else {
                    "restore normal priority"
                },
                if is_low_priority {
                    "Low Priority"
                } else {
                    "Normal Priority"
                },
                MatrixRequest::SetIsLowPriority {
                    room_id: room_id.clone(),
                    is_low_priority,
                },
            );
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.room_info_button))
            .clicked(actions)
        {
            self.show_telegram_room_info(cx, &room_label);
            enqueue_popup_notification(
                format!(
                    "Room info opened for {room_label} from loaded room state. Room settings Name/Topic/avatar/alias/history/join-rule/tombstone writes are confirmed live; power/member edits stay backend-contract work."
                ),
                PopupKind::Info,
                Some(4.0),
            );
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.room_settings_button))
            .clicked(actions)
        {
            self.show_telegram_room_settings_surface(cx, &room_label);
            enqueue_popup_notification(
                format!("Room settings opened for {room_label}. {ROOM_SETTINGS_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(3.0),
            );
            return;
        } else if self
            .view
            .button(cx, ids!(telegram_room_actions_strip.notifications_button))
            .clicked(actions)
        {
            // Notifications evidence: room action Notifications opens the same local strip only.
            self.show_telegram_notifications_surface(cx, &room_label);
            enqueue_popup_notification(
                format!("Notifications opened for {room_label}. {NOTIFICATIONS_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(3.0),
            );
            return;
        }

        if self
            .view
            .button(cx, ids!(telegram_room_header.search_button))
            .clicked(actions)
        {
            self.set_telegram_room_actions_visible(cx, false, None);
            self.set_telegram_room_info_visible(cx, false);
            self.set_telegram_search_mode_visible(cx, true);
            enqueue_popup_notification(
                format!(
                    "Search messages opens a local-only Telegram search surface for {room_label}. It searches the loaded timeline and does not query Matrix yet."
                ),
                PopupKind::Info,
                Some(4.0),
            );
        } else if self
            .view
            .button(cx, ids!(telegram_room_header.info_button))
            .clicked(actions)
        {
            self.show_telegram_room_info(cx, &room_label);
            enqueue_popup_notification(
                format!(
                    "Room info is shown in the Telegram header for {room_label} using loaded room state. Real editable room settings remain a base gap."
                ),
                PopupKind::Info,
                Some(4.0),
            );
        } else if self
            .view
            .button(cx, ids!(telegram_room_header.mute_button))
            .clicked(actions)
        {
            // Notifications evidence: header Mute opens the mode strip; writes still require confirmation.
            self.show_telegram_notifications_surface(cx, &room_label);
            enqueue_popup_notification(
                format!("Mute opened for {room_label}. {NOTIFICATIONS_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(3.0),
            );
        } else if self
            .view
            .button(cx, ids!(telegram_room_header.menu_button))
            .clicked(actions)
        {
            self.show_telegram_room_actions(cx, "Room actions");
            enqueue_popup_notification(
                "Telegram room actions opened. Link/Invite/Leave reuse existing base paths; settings remains read-only and notification mode writes require confirmation.",
                PopupKind::Info,
                Some(4.0),
            );
        }
    }

    fn apply_hepta_fixture_timeline_visibility_from_cxless_state(&mut self, visible: bool) {
        self.is_loaded = visible;
        self.tl_state = None;
    }

    /// Sends read receipts based on the current scroll position of the timeline.
    fn send_user_read_receipts_based_on_scroll_pos(
        &mut self,
        _cx: &mut Cx,
        actions: &ActionsBuf,
        portal_list: &PortalListRef,
    ) {
        //stopped scrolling
        if portal_list.scrolled(actions) {
            return;
        }
        let first_index = portal_list.first_id();
        let Some(tl_state) = self.tl_state.as_mut() else {
            return;
        };

        if let Some(ref mut index) = tl_state.prev_first_index {
            // to detect change of scroll when scroll ends
            if *index != first_index {
                if first_index >= *index {
                    // Get event_id and timestamp for the last visible event
                    let Some((last_event_id, last_timestamp)) = tl_state
                        .items
                        .get(std::cmp::min(
                            first_index + portal_list.visible_items(),
                            tl_state.items.len().saturating_sub(1),
                        ))
                        .and_then(|f| f.as_event())
                        .and_then(|f| f.event_id().map(|e| (e, f.timestamp())))
                    else {
                        *index = first_index;
                        return;
                    };
                    submit_async_request(MatrixRequest::ReadReceipt {
                        timeline_kind: tl_state.kind.clone(),
                        event_id: last_event_id.to_owned(),
                        receipt_type: ReceiptType::Read,
                    });
                    if tl_state.scrolled_past_read_marker {
                        submit_async_request(MatrixRequest::ReadReceipt {
                            timeline_kind: tl_state.kind.clone(),
                            event_id: last_event_id.to_owned(),
                            receipt_type: ReceiptType::FullyRead,
                        });
                    } else {
                        if let Some(own_user_receipt_timestamp) = &tl_state
                            .latest_own_user_receipt
                            .clone()
                            .and_then(|receipt| receipt.ts)
                        {
                            let Some((_first_event_id, first_timestamp)) = tl_state
                                .items
                                .get(first_index)
                                .and_then(|f| f.as_event())
                                .and_then(|f| f.event_id().map(|e| (e, f.timestamp())))
                            else {
                                *index = first_index;
                                return;
                            };
                            if own_user_receipt_timestamp >= &first_timestamp
                                && own_user_receipt_timestamp <= &last_timestamp
                            {
                                tl_state.scrolled_past_read_marker = true;
                                submit_async_request(MatrixRequest::ReadReceipt {
                                    timeline_kind: tl_state.kind.clone(),
                                    event_id: last_event_id.to_owned(),
                                    receipt_type: ReceiptType::FullyRead,
                                });
                            }
                        }
                    }
                }
                *index = first_index;
            }
        } else {
            tl_state.prev_first_index = Some(first_index);
        }
    }

    /// Sends a backwards pagination request if the user is scrolling up
    /// and is approaching the top of the timeline.
    fn send_pagination_request_based_on_scroll_pos(
        &mut self,
        _cx: &mut Cx,
        actions: &ActionsBuf,
        portal_list: &PortalListRef,
    ) {
        let Some(tl) = self.tl_state.as_mut() else {
            return;
        };
        if tl.fully_paginated {
            return;
        };
        if !portal_list.scrolled(actions) {
            return;
        };

        let first_index = portal_list.first_id();
        if first_index == 0 && tl.last_scrolled_index > 0 {
            log!(
                "Scrolled up from item {} --> 0, sending back pagination request for room {}",
                tl.last_scrolled_index,
                tl.kind,
            );
            submit_async_request(MatrixRequest::PaginateTimeline {
                timeline_kind: tl.kind.clone(),
                num_events: 50,
                direction: PaginationDirection::Backwards,
            });
        }
        tl.last_scrolled_index = first_index;
    }
}
