impl RoomScreen {
    fn room_id(&self) -> Option<&OwnedRoomId> {
        self.room_name_id.as_ref().map(|r| r.room_id())
    }

    fn handle_hepta_approval_decision(&self, cx: &mut Cx, index: usize, approved: bool) {
        let Some(tl) = self.tl_state.as_ref() else {
            return;
        };
        let Some(event_tl_item) = tl.items.get(index).and_then(|item| item.as_event()) else {
            return;
        };
        let envelope = hepta_envelope_from_timeline_item(event_tl_item);
        let envelope_id = envelope
            .as_ref()
            .map(|envelope| envelope.id.clone())
            .or_else(|| {
                event_tl_item
                    .event_id()
                    .map(|event_id| event_id.to_string())
            })
            .unwrap_or_else(|| "unknown approval".to_string());
        let decision = if approved { "approved" } else { "rejected" };
        let bridge_policy = decide_hepta_action(HeptaActionBridgeRequest {
            mutation_class: MUTATION_APPROVE_TOOL_EXEC,
            requires_confirmation: true,
            external_mutation_enabled: false,
            confirmed: false,
        });
        let exact_payload = serde_json::to_string_pretty(&serde_json::json!({
            "decision": decision,
            "matrix_event_id": event_tl_item.event_id().map(ToString::to_string),
            "hepta_envelope": envelope,
            "bridge_policy": bridge_policy.as_payload_value(),
        }))
        .unwrap_or_else(|err| format!("payload preview unavailable: {err}"));
        let body_text = format!(
            "Decision: {decision}\nTarget: {envelope_id}\nPolicy: {}\nReason: {}\n\nExact payload preview:\n{exact_payload}\n\nThis confirmation is local-only in the current Hepta Native phase. No OpenClaw Gateway call, Matrix send, tool approval, or task mutation will be executed; Hepta native execution remains policy-gated.",
            bridge_policy.disposition.label(),
            bridge_policy.reason,
        );
        let popup_target = envelope_id.clone();
        let content = ConfirmationModalContent {
            title_text: format!("Confirm local Hepta {decision} preview").into(),
            body_text: body_text.into(),
            accept_button_text: Some(
                if approved {
                    "Confirm preview"
                } else {
                    "Confirm reject preview"
                }
                .into(),
            ),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Hepta approval {decision}: {popup_target}\n\nLocal confirmation recorded as a preview only; no external mutation was sent."
                    ),
                    PopupKind::Warning,
                    Some(5.0),
                );
            })),
            ..Default::default()
        };

        if approved {
            cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                content,
            ))));
        } else {
            cx.action(ConfirmDeleteAction::Show(RefCell::new(Some(content))));
        }
    }

    fn handle_hepta_inspect_event(&self, cx: &mut Cx, index: usize) {
        let Some(tl) = self.tl_state.as_ref() else {
            return;
        };
        let Some(event_tl_item) = tl.items.get(index).and_then(|item| item.as_event()) else {
            return;
        };
        let latest_json: Option<String> = event_tl_item
            .latest_json()
            .and_then(|raw_event| serde_json::to_value(raw_event).ok())
            .and_then(|value| serde_json::to_string_pretty(&value).ok());
        let event_id = event_tl_item.event_id().map(|event_id| event_id.to_owned());
        cx.action(super::event_source_modal::EventSourceModalAction::Open {
            room_id: tl.kind.room_id().clone(),
            event_id,
            latest_json,
        });
    }

    /// Processes all pending background updates to the currently-shown timeline.
    ///
    /// Redraws this RoomScreen view if any updates were applied.
    fn process_timeline_updates(&mut self, cx: &mut Cx, portal_list: &PortalListRef) {
        let top_space = self.view(cx, ids!(top_space));
        let jump_to_bottom_button = self.jump_to_bottom_button(cx, ids!(jump_to_bottom_button));
        let curr_first_id = portal_list.first_id();
        let ui = self.widget_uid();
        let Some(tl) = self.tl_state.as_mut() else {
            return;
        };

        let mut done_loading = false;
        let mut should_continue_backwards_pagination = false;
        let mut typing_users = None;
        let mut notification_mode_updated = false;
        let mut notification_mode_write_result = None;
        let mut notification_keyword_rules_update = None;
        let mut notification_keyword_mutation_result = None;
        let mut notification_pusher_status_update = None;
        let mut notification_default_mode_update = None;
        let mut notification_default_mode_mutation_result = None;
        let mut room_settings_mutation_result = None;
        enum PendingTelegramEditHistorySurfaceUpdate {
            Result {
                event_id: OwnedEventId,
                replacement_count: usize,
                pages_fetched: usize,
                pagination_exhausted: bool,
                latest_event: String,
                latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
                loaded_original: String,
                latest_preview: String,
                latest_source_json: Option<String>,
            },
            Error {
                event_id: OwnedEventId,
                error: String,
            },
        }
        enum PendingTelegramMatrixLinkPreviewSurfaceUpdate {
            Result {
                room_or_alias_id: OwnedRoomOrAliasId,
                via: Vec<OwnedServerName>,
                event_id: Option<OwnedEventId>,
                event_source_room_id: Option<OwnedRoomId>,
                event_source_json: Option<String>,
                metadata: String,
            },
            Error {
                timeline_kind: TimelineKind,
                room_or_alias_id: OwnedRoomOrAliasId,
                via: Vec<OwnedServerName>,
                event_id: Option<OwnedEventId>,
                error: String,
            },
        }
        let mut edit_history_surface_update = None;
        let mut matrix_link_preview_surface_update = None;
        let mut matrix_link_paginated_event_update = None;
        let mut message_search_context_event_update = None;
        let mut message_search_server_surface_update = None;
        let mut report_status_surface_update = None;
        let mut num_updates = 0;
        while let Ok(update) = tl.update_receiver.try_recv() {
            num_updates += 1;
            match update {
                TimelineUpdate::FirstUpdate { initial_items } => {
                    tl.content_drawn_since_last_update.clear();
                    tl.profile_drawn_since_last_update.clear();
                    tl.fully_paginated = false;
                    // Set the portal list to the very bottom of the timeline.
                    portal_list.set_first_id_and_scroll(initial_items.len().saturating_sub(1), 0.0);
                    portal_list.set_tail_range(true);
                    jump_to_bottom_button.update_visibility(cx, true);

                    tl.items = initial_items;
                    done_loading = true;
                }
                TimelineUpdate::NewItems {
                    new_items,
                    changed_indices,
                    is_append,
                    clear_cache,
                } => {
                    if new_items.is_empty() {
                        if !tl.items.is_empty() {
                            log!(
                                "process_timeline_updates(): timeline (had {} items) was cleared for room {}",
                                tl.items.len(),
                                tl.kind.room_id()
                            );
                            // For now, we paginate a cleared timeline in order to be able to show something at least.
                            // A proper solution would be what's described below, which would be to save a few event IDs
                            // and then either focus on them (if we're not close to the end of the timeline)
                            // or paginate backwards until we find them (only if we are close the end of the timeline).
                            should_continue_backwards_pagination = true;
                        }

                        // If the bottom of the timeline (the last event) is visible, then we should
                        // set the timeline to live mode.
                        // If the bottom of the timeline is *not* visible, then we should
                        // set the timeline to Focused mode.

                        // TODO: Save the event IDs of the top 3 items before we apply this update,
                        //       which indicates this timeline is in the process of being restored,
                        //       such that we can jump back to that position later after applying this update.

                        // TODO: here we need to re-build the timeline via TimelineBuilder
                        //       and set the TimelineFocus to one of the above-saved event IDs.

                        // TODO: the docs for `TimelineBuilder::with_focus()` claim that the timeline's focus mode
                        //       can be changed after creation, but I do not see any methods to actually do that.
                        //       <https://matrix-org.github.io/matrix-rust-sdk/matrix_sdk_ui/timeline/struct.TimelineBuilder.html#method.with_focus>
                        //
                        //       As such, we probably need to create a new async request enum variant
                        //       that tells the background async task to build a new timeline
                        //       (either in live mode or focused mode around one or more events)
                        //       and then replaces the existing timeline in ALL_ROOMS_INFO with the new one.
                    }

                    let prior_items_changed = clear_cache || changed_indices.start <= curr_first_id;

                    if new_items.len() == tl.items.len() {
                        // log!("process_timeline_updates(): no jump necessary for updated timeline of same length: {}", items.len());
                    } else if curr_first_id > new_items.len() {
                        log!(
                            "process_timeline_updates(): jumping to bottom: curr_first_id {} is out of bounds for {} new items",
                            curr_first_id,
                            new_items.len()
                        );
                        portal_list.set_first_id_and_scroll(new_items.len().saturating_sub(1), 0.0);
                        portal_list.set_tail_range(true);
                        jump_to_bottom_button.update_visibility(cx, true);
                    }
                    // If the prior items changed, we need to find the new index of an item that was visible
                    // in the timeline viewport so that we can maintain the scroll position of that item,
                    // which ensures that the timeline doesn't jump around unexpectedly and ruin the user's experience.
                    else if let Some((curr_item_idx, new_item_idx, new_item_scroll, _event_id)) =
                        prior_items_changed
                            .then(|| {
                                find_new_item_matching_current_item(
                                    cx,
                                    portal_list,
                                    curr_first_id,
                                    &tl.items,
                                    &new_items,
                                )
                            })
                            .flatten()
                    {
                        if curr_item_idx != new_item_idx {
                            log!(
                                "process_timeline_updates(): jumping view from event index {curr_item_idx} to new index {new_item_idx}, scroll {new_item_scroll}, event ID {_event_id}"
                            );
                            portal_list.set_first_id_and_scroll(new_item_idx, new_item_scroll);
                            tl.prev_first_index = Some(new_item_idx);
                            // Set scrolled_past_read_marker false when we jump to a new event
                            tl.scrolled_past_read_marker = false;
                            // Hide the tooltip when the timeline jumps, as a hover-out event won't occur.
                            cx.widget_action(ui, RoomScreenTooltipActions::HoverOut);
                        }
                    }
                    //
                    // TODO: after an (un)ignore user event, all timelines are cleared. Handle that here.
                    //
                    else {
                        // warning!("!!! Couldn't find new event with matching ID for ANY event currently visible in the portal list");
                    }

                    // If new items were appended to the end of the timeline, show an unread messages badge on the jump to bottom button.
                    if is_append && !portal_list.is_at_end() {
                        // We only show unread message badges on the jump to bottom button for main room timelines,
                        // because the matrix SDK doesn't currently support querying unread message counts for threads.
                        if matches!(tl.kind, TimelineKind::MainRoom { .. }) {
                            // Immediately show the unread badge with no count while we fetch the actual count in the background.
                            jump_to_bottom_button
                                .show_unread_message_badge(cx, UnreadMessageCount::Unknown);
                            submit_async_request(MatrixRequest::GetNumberUnreadMessages {
                                timeline_kind: tl.kind.clone(),
                            });
                        }
                    }

                    if prior_items_changed {
                        // If this RoomScreen is showing the loading pane and has an ongoing backwards pagination request,
                        // then we should update the status message in that loading pane
                        // and then continue paginating backwards until we find the target event.
                        // Note that we do this here because `clear_cache` will always be true if backwards pagination occurred.
                        let loading_pane = self.view.loading_pane(cx, ids!(loading_pane));
                        let mut loading_pane_state = loading_pane.take_state();
                        if let LoadingPaneState::BackwardsPaginateUntilEvent {
                            events_paginated,
                            target_event_id,
                            ..
                        } = &mut loading_pane_state
                        {
                            *events_paginated += new_items.len().saturating_sub(tl.items.len());
                            log!(
                                "While finding target event {target_event_id}, we have now loaded {events_paginated} messages..."
                            );
                            // Here, we assume that we have not yet found the target event,
                            // so we need to continue paginating backwards.
                            // If the target event has already been found, it will be handled
                            // in the `TargetEventFound` match arm below, which will set
                            // `should_continue_backwards_pagination` to `false`.
                            // So either way, it's okay to set this to `true` here.
                            should_continue_backwards_pagination = true;
                        }
                        loading_pane.set_state(cx, loading_pane_state);
                    }

                    if clear_cache {
                        tl.content_drawn_since_last_update.clear();
                        tl.profile_drawn_since_last_update.clear();
                        tl.fully_paginated = false;
                    } else {
                        tl.content_drawn_since_last_update
                            .remove(changed_indices.clone());
                        tl.profile_drawn_since_last_update
                            .remove(changed_indices.clone());
                        // log!("process_timeline_updates(): changed_indices: {changed_indices:?}, items len: {}\ncontent drawn: {:#?}\nprofile drawn: {:#?}", items.len(), tl.content_drawn_since_last_update, tl.profile_drawn_since_last_update);
                    }
                    tl.items = new_items;
                    done_loading = true;
                }
                TimelineUpdate::NewUnreadMessagesCount(unread_messages_count) => {
                    // We only show unread message badges on the jump to bottom button for main room timelines,
                    // because the matrix SDK doesn't currently support querying unread message counts for threads.
                    if matches!(tl.kind, TimelineKind::MainRoom { .. }) {
                        jump_to_bottom_button.show_unread_message_badge(cx, unread_messages_count);
                    }
                }
                TimelineUpdate::TargetEventFound {
                    target_event_id,
                    index,
                } => {
                    // log!("Target event found in room {}: {target_event_id}, index: {index}", tl.kind.room_id());
                    tl.request_sender.send_if_modified(|requests| {
                        requests.retain(|r| &r.room_id != tl.kind.room_id());
                        // no need to notify/wake-up all receivers for a completed request
                        false
                    });

                    // sanity check: ensure the target event is in the timeline at the given `index`.
                    let item = tl.items.get(index);
                    let is_valid = item.is_some_and(|item| {
                        item.as_event()
                            .is_some_and(|ev| ev.event_id() == Some(&target_event_id))
                    });
                    let loading_pane = self.view.loading_pane(cx, ids!(loading_pane));

                    // log!("TargetEventFound: is_valid? {is_valid}. room {}, event {target_event_id}, index {index} of {}\n  --> item: {item:?}", tl.kind.room_id(), tl.items.len());
                    if is_valid {
                        // We successfully found the target event, so we can close the loading pane,
                        // reset the loading panestate to `None`, and stop issuing backwards pagination requests.
                        let is_matrix_link_paginated_event =
                            self.telegram_matrix_link_preview_status == "paginating"
                                && self.telegram_matrix_link_preview_event_id_label.trim()
                                    == target_event_id.as_str();
                        let is_message_search_context_event = self
                            .telegram_message_search_server_context_target_event_id
                            .as_ref()
                            .is_some_and(|event_id| event_id == &target_event_id);
                        loading_pane.set_status(
                            cx,
                            if is_matrix_link_paginated_event {
                                "Successfully found Matrix event link target!"
                            } else if is_message_search_context_event {
                                "Successfully found server search context event!"
                            } else {
                                "Successfully found replied-to message!"
                            },
                        );
                        loading_pane.set_state(cx, LoadingPaneState::None);

                        // NOTE: this code was copied from the `MessageAction::JumpToRelated` handler;
                        //       we should deduplicate them at some point.
                        let speed = 50.0;
                        portal_list.smooth_scroll_to(cx, index, speed, None, 10.0);
                        // start highlight animation.
                        tl.message_highlight_animation_state =
                            MessageHighlightAnimationState::Pending { item_id: index };
                        if is_matrix_link_paginated_event {
                            let snippet = loaded_event_plaintext_preview_for_event_id(
                                &tl.items,
                                &target_event_id,
                            )
                            .unwrap_or_else(|| "loaded event preview unavailable".to_string());
                            matrix_link_paginated_event_update =
                                Some((target_event_id.clone(), index, snippet));
                        }
                        if is_message_search_context_event {
                            let snippet = loaded_event_plaintext_preview_for_event_id(
                                &tl.items,
                                &target_event_id,
                            )
                            .unwrap_or_else(|| "loaded event preview unavailable".to_string());
                            message_search_context_event_update =
                                Some((target_event_id.clone(), index, snippet));
                        }
                    } else {
                        // Here, the target event was not found in the current timeline,
                        // or we found it previously but it is no longer in the timeline (or has moved),
                        // which means we encountered an error and are unable to jump to the target event.
                        error!(
                            "Target event index {index} of {} is out of bounds for room {}",
                            tl.items.len(),
                            tl.kind.room_id()
                        );
                        // Show this error in the loading pane, which should already be open.
                        loading_pane.set_state(
                            cx,
                            LoadingPaneState::Error(String::from(
                                "Unable to find related message; it may have been deleted.",
                            )),
                        );
                    }

                    should_continue_backwards_pagination = false;

                    // redraw now before any other items get added to the timeline list.
                    self.view.redraw(cx);
                }
                TimelineUpdate::PaginationRunning(direction) => {
                    if direction == PaginationDirection::Backwards {
                        top_space.set_visible(cx, true);
                        done_loading = false;
                    } else {
                        error!("Unexpected PaginationRunning update in the Forwards direction");
                    }
                }
                TimelineUpdate::PaginationError { error, direction } => {
                    error!(
                        "Pagination error ({direction}) in {:?}: {error:?}",
                        self.room_name_id
                    );
                    let room_name = self.room_name_id.as_ref().map(|r| r.to_string());
                    enqueue_popup_notification(
                        utils::stringify_pagination_error(
                            &error,
                            room_name.as_deref().unwrap_or(UNNAMED_ROOM),
                        ),
                        PopupKind::Error,
                        Some(10.0),
                    );
                    done_loading = true;
                }
                TimelineUpdate::PaginationIdle {
                    fully_paginated,
                    direction,
                } => {
                    if direction == PaginationDirection::Backwards {
                        // Don't set `done_loading` to `true` here, because we want to keep the top space visible
                        // (with the "loading" message) until the corresponding `NewItems` update is received.
                        tl.fully_paginated = fully_paginated;
                        if fully_paginated {
                            done_loading = true;
                        }
                    } else {
                        error!("Unexpected PaginationIdle update in the Forwards direction");
                    }
                }
                TimelineUpdate::EventDetailsFetched { event_id, result } => {
                    if let Err(_e) = result {
                        error!(
                            "Failed to fetch details fetched for event {event_id} in room {}. Error: {_e:?}",
                            tl.kind.room_id()
                        );
                    }
                    // Here, to be most efficient, we could redraw only the updated event,
                    // but for now we just fall through and let the final `redraw()` call re-draw the whole timeline view.
                }
                TimelineUpdate::ThreadSummaryDetailsFetched {
                    thread_root_event_id,
                    timeline_item_index,
                    num_replies,
                    latest_reply_preview_text,
                } => {
                    tl.pending_thread_summary_fetches
                        .remove(&thread_root_event_id);
                    tl.fetched_thread_summaries.insert(
                        thread_root_event_id.clone(),
                        FetchedThreadSummary {
                            num_replies,
                            latest_reply_preview_text,
                        },
                    );
                    let event_id_matches_at_index = tl
                        .items
                        .get(timeline_item_index)
                        .and_then(|item| item.as_event())
                        .and_then(|ev| ev.event_id())
                        .is_some_and(|id| id == thread_root_event_id);
                    if event_id_matches_at_index {
                        tl.content_drawn_since_last_update
                            .remove(timeline_item_index..timeline_item_index + 1);
                    } else {
                        tl.content_drawn_since_last_update.clear();
                    }
                }
                TimelineUpdate::EditHistoryFetched { event_id, result } => match result {
                    Ok(summary) => {
                        self.telegram_message_edit_history_retry_event_id = None;
                        self.telegram_message_edit_history_retry_timeline_kind = None;
                        let loaded_original =
                            loaded_event_plaintext_preview_for_event_id(&tl.items, &event_id)
                                .unwrap_or_else(|| {
                                    "loaded original preview unavailable".to_string()
                                });
                        let latest = summary
                            .latest_preview_text
                            .as_deref()
                            .map(compact_edit_history_preview)
                            .unwrap_or_else(|| "No replacement preview text returned.".to_string());
                        let latest_event = summary
                            .latest_event_id
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_else(|| "unknown replacement event".to_string());
                        let diff_hint = edit_history_local_diff_hint(&loaded_original, &latest);
                        let latest_timestamp = summary.latest_timestamp;
                        let timestamp_note = latest_timestamp
                            .map(|ts| format!(" Latest replacement timestamp: {}.", ts.get()))
                            .unwrap_or_default();
                        edit_history_surface_update =
                            Some(PendingTelegramEditHistorySurfaceUpdate::Result {
                                event_id: event_id.clone(),
                                replacement_count: summary.replacement_count,
                                pages_fetched: summary.pages_fetched,
                                pagination_exhausted: summary.pagination_exhausted,
                                latest_event: latest_event.clone(),
                                latest_timestamp,
                                loaded_original: loaded_original.clone(),
                                latest_preview: latest.clone(),
                                latest_source_json: summary.latest_source_json.clone(),
                            });
                        enqueue_popup_notification(
                            format!(
                                "Edit history summary for {event_id}: {} replacement event(s) across {} relation page(s). Loaded original: {loaded_original}. Latest replacement: {latest_event}. {latest}. {diff_hint}{timestamp_note}",
                                summary.replacement_count, summary.pages_fetched
                            ),
                            PopupKind::Info,
                            Some(8.0),
                        );
                    }
                    Err(error) => {
                        self.telegram_message_edit_history_retry_event_id = Some(event_id.clone());
                        self.telegram_message_edit_history_retry_timeline_kind =
                            Some(tl.kind.clone());
                        edit_history_surface_update =
                            Some(PendingTelegramEditHistorySurfaceUpdate::Error {
                                event_id: event_id.clone(),
                                error: error.to_string(),
                            });
                        enqueue_popup_notification(
                            format!("Failed to fetch edit history for {event_id}: {error}"),
                            PopupKind::Error,
                            Some(6.0),
                        );
                    }
                },
                TimelineUpdate::EventSourceFetched { event_id, result } => match result {
                    Ok(latest_json) => {
                        cx.action(super::event_source_modal::EventSourceModalAction::Open {
                            room_id: tl.kind.room_id().clone(),
                            event_id: Some(event_id.clone()),
                            latest_json: Some(latest_json.clone()),
                        });
                        enqueue_popup_notification(
                            format!(
                                "Fetched event source for {event_id} through Matrix room.event/load_or_fetch_event; opened EventSourceModal with {} chars.",
                                latest_json.chars().count()
                            ),
                            PopupKind::Success,
                            Some(5.0),
                        );
                    }
                    Err(error) => {
                        enqueue_popup_notification(
                            format!("Failed to fetch event source for {event_id}: {error}"),
                            PopupKind::Error,
                            Some(6.0),
                        );
                    }
                },
                TimelineUpdate::RoomMembersSynced => {
                    // log!("process_timeline_updates(): room members fetched for room {}", tl.kind.room_id());
                    // Here, to be most efficient, we could redraw only the user avatars and names in the timeline,
                    // but for now we just fall through and let the final `redraw()` call re-draw the whole timeline view.
                }
                TimelineUpdate::RoomMembersListFetched { members } => {
                    // Store room members directly in TimelineUiState
                    tl.room_members = Some(Arc::new(members));
                }
                TimelineUpdate::MediaFetched(request) => {
                    log!(
                        "process_timeline_updates(): media fetched for room {}",
                        tl.kind.room_id()
                    );
                    // Set Image to image viewer modal if the media is not a thumbnail.
                    if let (MediaFormat::File, media_source) = (request.format, request.source) {
                        populate_matrix_image_modal(cx, media_source, &mut tl.media_cache);
                    }
                    // Here, to be most efficient, we could redraw only the media items in the timeline,
                    // but for now we just fall through and let the final `redraw()` call re-draw the whole timeline view.
                }
                TimelineUpdate::MediaSaveResult {
                    source_key,
                    destination_path,
                    result,
                } => match result {
                    Ok(()) => {
                        tl.saved_media_destinations
                            .insert(source_key.clone(), destination_path.clone());
                        enqueue_popup_notification(
                            media_save_result_cache_update_label(
                                &source_key,
                                &destination_path,
                                true,
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    }
                    Err(error) => {
                        tl.saved_media_destinations.remove(&source_key);
                        enqueue_popup_notification(
                            format!(
                                "{} {error}",
                                media_save_result_cache_update_label(
                                    &source_key,
                                    &destination_path,
                                    false,
                                )
                            ),
                            PopupKind::Warning,
                            Some(4.0),
                        );
                    }
                },
                TimelineUpdate::MessageEdited {
                    timeline_event_item_id: timeline_event_id,
                    result,
                } => {
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .handle_edit_result(cx, timeline_event_id, result);
                }
                TimelineUpdate::PinResult { result, pin, .. } => {
                    let (message, auto_dismissal_duration, kind) = match &result {
                        Ok(true) => (
                            format!(
                                "Successfully {} event.",
                                if pin { "pinned" } else { "unpinned" }
                            ),
                            Some(4.0),
                            PopupKind::Success,
                        ),
                        Ok(false) => (
                            format!(
                                "Message was already {}.",
                                if pin { "pinned" } else { "unpinned" }
                            ),
                            Some(4.0),
                            PopupKind::Info,
                        ),
                        Err(e) => (
                            format!(
                                "Failed to {} event. Error: {e}",
                                if pin { "pin" } else { "unpin" }
                            ),
                            None,
                            PopupKind::Error,
                        ),
                    };
                    enqueue_popup_notification(message, kind, auto_dismissal_duration);
                }
                TimelineUpdate::MessageReportResult { event_id, result } => {
                    report_status_surface_update = Some((event_id.clone(), result.clone()));
                    match result {
                        Ok(()) => enqueue_popup_notification(
                            "Report sent to Matrix server.",
                            PopupKind::Success,
                            Some(4.0),
                        ),
                        Err(error) => enqueue_popup_notification(
                            format!("Report failed: {error}"),
                            PopupKind::Error,
                            Some(6.0),
                        ),
                    }
                }
                TimelineUpdate::MatrixLinkPreviewResult {
                    room_or_alias_id,
                    via,
                    event_id,
                    event_source_room_id,
                    event_source_json,
                    result,
                } => match result {
                    Ok(metadata) => {
                        matrix_link_preview_surface_update =
                            Some(PendingTelegramMatrixLinkPreviewSurfaceUpdate::Result {
                                room_or_alias_id,
                                via,
                                event_id,
                                event_source_room_id,
                                event_source_json,
                                metadata,
                            });
                    }
                    Err(error) => {
                        matrix_link_preview_surface_update =
                            Some(PendingTelegramMatrixLinkPreviewSurfaceUpdate::Error {
                                timeline_kind: tl.kind.clone(),
                                room_or_alias_id,
                                via,
                                event_id,
                                error,
                            });
                    }
                },
                TimelineUpdate::MessageSearchServerResult { result } => {
                    message_search_server_surface_update = Some(result);
                }
                TimelineUpdate::AttachmentSendResult { filename, result } => {
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .handle_attachment_send_result(cx, filename, result);
                }
                TimelineUpdate::LocalSendAbortResult { result } => {
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .handle_local_send_abort_result(cx, result);
                }
                TimelineUpdate::TypingUsers { users } => {
                    // This update loop should be kept tight & fast, so all we do here is
                    // save the list of typing users for future use after the loop exits.
                    // Then, we "process" it later (by turning it into a string) after the
                    // update loop has completed, which avoids unnecessary expensive work
                    // if the list of typing users gets updated many times in a row.
                    typing_users = Some(users);
                }
                TimelineUpdate::PinnedEvents(pinned_events) => {
                    self.pinned_events = pinned_events;
                    // We need to redraw any events that might have been pinned or unpinned
                    // in order to have all events properly reflect their pinned state.
                    // However, it's intractable to find exactly which events in the timeline
                    // had a change in their pinned state, so we just clear all draw caches.
                    tl.content_drawn_since_last_update.clear();
                    tl.profile_drawn_since_last_update.clear();
                }
                TimelineUpdate::UserPowerLevels(user_power_levels) => {
                    tl.user_power = user_power_levels;
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .update_user_power_levels(cx, user_power_levels);
                    // Update the @room mention capability based on the user's power level
                    cx.action(MentionableTextInputAction::PowerLevelsUpdated {
                        room_id: tl.kind.room_id().clone(),
                        can_notify_room: user_power_levels.can_notify_room(),
                    });
                    // We need to redraw all events in order to reflect the new power levels,
                    // e.g., for the message context menu to be correctly populated.
                    tl.content_drawn_since_last_update.clear();
                    tl.profile_drawn_since_last_update.clear();
                }
                TimelineUpdate::RoomNotificationMode(mode) => {
                    tl.room_notification_mode = mode;
                    notification_mode_updated = true;
                }
                TimelineUpdate::RoomNotificationModeSet { mode, result } => {
                    if result.is_ok() {
                        tl.room_notification_mode = Some(mode);
                        notification_mode_updated = true;
                    }
                    notification_mode_write_result = Some((mode, result));
                }
                TimelineUpdate::NotificationKeywordRulesFetched(summary) => {
                    notification_keyword_rules_update = Some(summary);
                }
                TimelineUpdate::NotificationKeywordRulesMutated {
                    keyword,
                    mutation,
                    result,
                } => {
                    notification_keyword_mutation_result = Some((keyword, mutation, result));
                }
                TimelineUpdate::NotificationPusherStatusFetched(summary) => {
                    notification_pusher_status_update = Some(summary);
                }
                TimelineUpdate::NotificationDefaultRoomModeFetched(result) => {
                    notification_default_mode_update = Some(result);
                }
                TimelineUpdate::NotificationDefaultRoomModeMutated { mode, result } => {
                    notification_default_mode_mutation_result =
                        Some((tl.kind.clone(), mode, result));
                }
                TimelineUpdate::RoomSettingsMutationResult {
                    field,
                    value,
                    result,
                } => {
                    room_settings_mutation_result = Some((field, value, result));
                }
                TimelineUpdate::OwnUserReadReceipt(receipt) => {
                    tl.latest_own_user_receipt = Some(receipt);
                }
                TimelineUpdate::Tombstoned(successor_room_details) => {
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .update_tombstone_footer(
                            cx,
                            tl.kind.room_id(),
                            Some(&successor_room_details),
                        );
                    tl.tombstone_info = Some(successor_room_details);
                }
                TimelineUpdate::LinkPreviewFetched => {}
                TimelineUpdate::FileUploadStarted {
                    upload_id,
                    file_name,
                    in_reply_to,
                    abort_handle,
                } => {
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .handle_file_upload_started(
                            cx,
                            upload_id,
                            &file_name,
                            in_reply_to.as_ref(),
                            abort_handle,
                        );
                }
                TimelineUpdate::FileUploadUpdate {
                    upload_id,
                    current,
                    total,
                } => {
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .set_upload_progress(cx, upload_id, current, total);
                }
                TimelineUpdate::FileUploadError {
                    upload_id,
                    error,
                    upload,
                    retryable,
                } => {
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .show_upload_error(cx, upload_id, &error, upload, retryable);
                }
                TimelineUpdate::FileUploadComplete { upload_id } => {
                    self.view
                        .room_input_bar(cx, ids!(room_input_bar))
                        .hide_upload_progress(cx, upload_id);
                }
                TimelineUpdate::AttachmentDownloadFinished(mxc, result) => {
                    if let Some(entry) = tl
                        .pending_downloads
                        .iter_mut()
                        .find(|pending| pending.mxc == mxc)
                    {
                        entry.state = match result {
                            Ok(()) => PendingDownloadState::JustSucceeded,
                            Err(_) => PendingDownloadState::JustFailed,
                        };
                    }
                    portal_list.redraw(cx);
                }
                TimelineUpdate::AttachmentDownloadReset(mxc) => {
                    tl.pending_downloads.retain(|pending| pending.mxc != mxc);
                    portal_list.redraw(cx);
                }
            }
        }

        if should_continue_backwards_pagination {
            submit_async_request(MatrixRequest::PaginateTimeline {
                timeline_kind: tl.kind.clone(),
                num_events: 50,
                direction: PaginationDirection::Backwards,
            });
        }

        if done_loading {
            top_space.set_visible(cx, false);
        }

        if let Some((mode, result)) = notification_mode_write_result {
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
            self.update_telegram_notification_mode_result(cx, &room_label, mode, result);
        }

        if let Some((field, value, result)) = room_settings_mutation_result {
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
            self.update_telegram_room_settings_mutation_result(
                cx,
                &room_label,
                field,
                value,
                result,
            );
        }

        if let Some(summary) = notification_keyword_rules_update {
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
            self.update_telegram_notification_keyword_rules_result(cx, &room_label, &summary);
        }

        if let Some((keyword, mutation, result)) = notification_keyword_mutation_result {
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
            self.update_telegram_notification_keyword_mutation_result(
                cx,
                &room_label,
                keyword,
                mutation,
                result,
            );
        }

        if let Some(summary) = notification_pusher_status_update {
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
            self.update_telegram_notification_pusher_status_result(cx, &room_label, &summary);
        }

        if let Some(result) = notification_default_mode_update {
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
            self.update_telegram_notification_default_room_mode_result(cx, &room_label, &result);
        }

        if let Some((timeline_kind, mode, result)) = notification_default_mode_mutation_result {
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
            self.update_telegram_notification_default_room_mode_mutation_result(
                cx,
                &room_label,
                timeline_kind,
                mode,
                result,
            );
        }

        if notification_mode_updated && self.telegram_notifications_visible {
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
            self.update_telegram_notifications_strip(cx, &room_label);
        }

        if let Some(update) = edit_history_surface_update {
            match update {
                PendingTelegramEditHistorySurfaceUpdate::Result {
                    event_id,
                    replacement_count,
                    pages_fetched,
                    pagination_exhausted,
                    latest_event,
                    latest_timestamp,
                    loaded_original,
                    latest_preview,
                    latest_source_json,
                } => self.show_telegram_message_edit_history_result(
                    cx,
                    &event_id,
                    replacement_count,
                    pages_fetched,
                    pagination_exhausted,
                    &latest_event,
                    latest_timestamp,
                    &loaded_original,
                    &latest_preview,
                    latest_source_json,
                ),
                PendingTelegramEditHistorySurfaceUpdate::Error { event_id, error } => {
                    self.show_telegram_message_edit_history_error(cx, &event_id, &error);
                }
            }
        }

        if let Some(update) = matrix_link_preview_surface_update {
            match update {
                PendingTelegramMatrixLinkPreviewSurfaceUpdate::Result {
                    room_or_alias_id,
                    via,
                    event_id,
                    event_source_room_id,
                    event_source_json,
                    metadata,
                } => self.show_telegram_matrix_link_preview_result(
                    cx,
                    &room_or_alias_id,
                    &via,
                    event_id.as_ref(),
                    event_source_room_id,
                    event_source_json,
                    &metadata,
                ),
                PendingTelegramMatrixLinkPreviewSurfaceUpdate::Error {
                    timeline_kind,
                    room_or_alias_id,
                    via,
                    event_id,
                    error,
                } => self.show_telegram_matrix_link_preview_error(
                    cx,
                    timeline_kind,
                    room_or_alias_id,
                    via,
                    event_id,
                    &error,
                ),
            }
        }

        if let Some((event_id, loaded_index, snippet)) = matrix_link_paginated_event_update {
            self.show_telegram_matrix_link_paginated_event_found(
                cx,
                &event_id,
                loaded_index,
                &snippet,
            );
        }

        if let Some((event_id, loaded_index, snippet)) = message_search_context_event_update {
            self.show_telegram_message_search_server_context_event_found(
                cx,
                &event_id,
                loaded_index,
                &snippet,
                true,
            );
        }

        if let Some(result) = message_search_server_surface_update {
            self.apply_telegram_message_search_server_result(cx, result);
        }

        if let Some((event_id, result)) = report_status_surface_update {
            self.show_telegram_message_report_result(cx, &event_id, &result);
        }

        if let Some(users) = typing_users {
            self.view
                .typing_notice(cx, ids!(typing_notice))
                .show_or_hide(cx, &users);
        }

        if num_updates > 0 {
            // log!("Applied {} timeline updates for room {}, redrawing with {} items...", num_updates, tl.kind.room_id(), tl.items.len());
            self.redraw(cx);
        }
    }

    /// Opens a confirmation guard before handing a URL to the system browser.
    fn show_external_link_confirmation(cx: &mut Cx, url: String) {
        let url_for_accept = url.clone();
        let url_for_cancel = url.clone();
        let content = ConfirmationModalContent {
        title_text: "Open External Link".into(),
        body_text: format!(
            "Open this link in the system browser?\n\n{url}\n\n{EXTERNAL_LINK_CONFIRMATION_COMPACT_LABEL}"
        )
        .into(),
        accept_button_text: Some("Open".into()),
        cancel_button_text: Some("Cancel".into()),
        on_accept_clicked: Some(Box::new(move |_cx| {
            // External link confirmation evidence: this is the only branch
            // that hands the URL to the system browser. The confirmation
            // guard itself, Cancel, and display paths send no browser
            // handoff, Matrix event fetch, room preview fetch, message
            // send, room-state, membership, or live mutation request.
            log!("Opening URL \"{}\" after confirmation", url_for_accept);
            if let Err(e) = robius_open::Uri::new(&url_for_accept).open() {
                error!("Failed to open URL {:?}. Error: {:?}", url_for_accept, e);
                enqueue_popup_notification(
                    format!("Could not open URL: {url_for_accept}"),
                    PopupKind::Error,
                    Some(10.0),
                );
            } else {
                enqueue_popup_notification(
                    format!("External link opened after confirmation: {url_for_accept}"),
                    PopupKind::Info,
                    Some(4.0),
                );
            }
        })),
        on_cancel_clicked: Some(Box::new(move |_cx| {
            enqueue_popup_notification(
                format!(
                    "External link canceled for {url_for_cancel}. {EXTERNAL_LINK_CONFIRMATION_COMPACT_LABEL}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
        })),
    };
        enqueue_popup_notification(
            format!(
                "External link confirmation opened for {url}. {EXTERNAL_LINK_CONFIRMATION_COMPACT_LABEL}"
            ),
            PopupKind::Info,
            Some(3.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn handle_media_download_link(&mut self, cx: &mut Cx, url: &str) -> bool {
        let Ok(parsed) = url::Url::parse(url) else {
            return false;
        };
        if parsed.scheme() != MEDIA_DOWNLOAD_URL_SCHEME {
            return false;
        }

        let mut mxc = None;
        let mut filename = None;
        let mut open_after_save = false;
        let mut metadata = MediaDownloadActionMetadata::default();
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "mxc" => mxc = Some(value.into_owned()),
                "name" => filename = Some(value.into_owned()),
                "open" => open_after_save = value == "1",
                "kind" => metadata.kind = value.into_owned(),
                "mime" => metadata.mime_type = Some(value.into_owned()),
                "size" => metadata.size_label = Some(value.into_owned()),
                "duration" => metadata.duration_label = Some(value.into_owned()),
                "dimensions" => metadata.dimensions_label = Some(value.into_owned()),
                _ => {}
            }
        }

        let Some(mxc) = mxc else {
            enqueue_popup_notification(
                "Media link is missing its MXC URI.",
                PopupKind::Error,
                Some(4.0),
            );
            return true;
        };
        let mxc_uri = OwnedMxcUri::from(mxc);
        let suggested_filename = filename
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "hepta-media-download".to_string());

        self.show_media_save_confirmation(
            cx,
            mxc_uri,
            suggested_filename,
            open_after_save,
            metadata,
        );
        true
    }

    fn handle_media_metadata_clipboard_link(cx: &mut Cx, url: &str) -> bool {
        let Ok(parsed) = url::Url::parse(url) else {
            return false;
        };
        if parsed.scheme() != MEDIA_METADATA_CLIPBOARD_URL_SCHEME {
            return false;
        }

        let mut filename = None;
        let mut metadata = MediaDownloadActionMetadata::default();
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "name" => filename = Some(value.into_owned()),
                "kind" => metadata.kind = value.into_owned(),
                "mime" => metadata.mime_type = Some(value.into_owned()),
                "size" => metadata.size_label = Some(value.into_owned()),
                "duration" => metadata.duration_label = Some(value.into_owned()),
                "dimensions" => metadata.dimensions_label = Some(value.into_owned()),
                _ => {}
            }
        }

        let filename = filename
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "hepta-media-download".to_string());
        let metadata_summary = metadata.summary(&filename);
        let payload = media_metadata_clipboard_payload(&filename, &metadata);
        cx.copy_to_clipboard(&payload);
        enqueue_popup_notification(
            media_metadata_clipboard_label(true, &filename, &metadata_summary),
            PopupKind::Info,
            Some(4.0),
        );
        true
    }

    fn handle_media_operation_packet_link(cx: &mut Cx, url: &str) -> bool {
        let Ok(parsed) = url::Url::parse(url) else {
            return false;
        };
        if parsed.scheme() != MEDIA_OPERATION_PACKET_URL_SCHEME {
            return false;
        }

        let mut action_label = None;
        let mut filename = None;
        let mut open_after_save = false;
        let mut metadata = MediaDownloadActionMetadata::default();
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "label" => action_label = Some(value.into_owned()),
                "name" => filename = Some(value.into_owned()),
                "open" => open_after_save = value == "1",
                "kind" => metadata.kind = value.into_owned(),
                "mime" => metadata.mime_type = Some(value.into_owned()),
                "size" => metadata.size_label = Some(value.into_owned()),
                "duration" => metadata.duration_label = Some(value.into_owned()),
                "dimensions" => metadata.dimensions_label = Some(value.into_owned()),
                _ => {}
            }
        }

        let action_label = action_label
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| if open_after_save { "Play" } else { "Download" }.to_string());
        let filename = filename
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "hepta-media-download".to_string());
        let metadata_summary = metadata.summary(&filename);
        let payload =
            media_operation_packet_payload(&action_label, &filename, &metadata, open_after_save);
        cx.copy_to_clipboard(&payload);
        enqueue_popup_notification(
            media_operation_packet_clipboard_label(
                true,
                &action_label,
                &filename,
                &metadata_summary,
                open_after_save,
            ),
            PopupKind::Info,
            Some(4.0),
        );
        true
    }

    fn handle_media_playback_queue_contract_link(cx: &mut Cx, url: &str) -> bool {
        let Ok(parsed) = url::Url::parse(url) else {
            return false;
        };
        if parsed.scheme() != MEDIA_PLAYBACK_QUEUE_CONTRACT_URL_SCHEME {
            return false;
        }

        let mut action_label = None;
        let mut filename = None;
        let mut open_after_save = false;
        let mut metadata = MediaDownloadActionMetadata::default();
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "label" => action_label = Some(value.into_owned()),
                "name" => filename = Some(value.into_owned()),
                "open" => open_after_save = value == "1",
                "kind" => metadata.kind = value.into_owned(),
                "mime" => metadata.mime_type = Some(value.into_owned()),
                "size" => metadata.size_label = Some(value.into_owned()),
                "duration" => metadata.duration_label = Some(value.into_owned()),
                "dimensions" => metadata.dimensions_label = Some(value.into_owned()),
                _ => {}
            }
        }

        let action_label = action_label
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| if open_after_save { "Play" } else { "Download" }.to_string());
        let filename = filename
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "hepta-media-download".to_string());
        let metadata_summary = metadata.summary(&filename);
        let payload = media_playback_queue_contract_payload(
            &action_label,
            &filename,
            &metadata,
            open_after_save,
        );
        cx.copy_to_clipboard(&payload);
        enqueue_popup_notification(
            media_playback_queue_contract_clipboard_label(
                true,
                &action_label,
                &filename,
                &metadata_summary,
                open_after_save,
            ),
            PopupKind::Info,
            Some(4.0),
        );
        true
    }

    fn handle_media_playback_result_taxonomy_link(cx: &mut Cx, url: &str) -> bool {
        let Ok(parsed) = url::Url::parse(url) else {
            return false;
        };
        if parsed.scheme() != MEDIA_PLAYBACK_RESULT_TAXONOMY_URL_SCHEME {
            return false;
        }

        let mut action_label = None;
        let mut filename = None;
        let mut open_after_save = false;
        let mut metadata = MediaDownloadActionMetadata::default();
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "label" => action_label = Some(value.into_owned()),
                "name" => filename = Some(value.into_owned()),
                "open" => open_after_save = value == "1",
                "kind" => metadata.kind = value.into_owned(),
                "mime" => metadata.mime_type = Some(value.into_owned()),
                "size" => metadata.size_label = Some(value.into_owned()),
                "duration" => metadata.duration_label = Some(value.into_owned()),
                "dimensions" => metadata.dimensions_label = Some(value.into_owned()),
                _ => {}
            }
        }

        let action_label = action_label
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| if open_after_save { "Play" } else { "Download" }.to_string());
        let filename = filename
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "hepta-media-download".to_string());
        let metadata_summary = metadata.summary(&filename);
        let payload = media_playback_result_taxonomy_payload(
            &action_label,
            &filename,
            &metadata,
            open_after_save,
        );
        cx.copy_to_clipboard(&payload);
        enqueue_popup_notification(
            media_playback_result_taxonomy_clipboard_label(
                true,
                &action_label,
                &filename,
                &metadata_summary,
                open_after_save,
            ),
            PopupKind::Info,
            Some(4.0),
        );
        true
    }

    fn handle_media_result_control_link(&mut self, cx: &mut Cx, url: &str) -> bool {
        let Ok(parsed) = url::Url::parse(url) else {
            return false;
        };
        if parsed.scheme() != MEDIA_RESULT_CONTROL_URL_SCHEME {
            return false;
        }

        let mut mxc = None;
        let mut action = None;
        let mut action_label = None;
        let mut filename = None;
        let mut open_after_save = false;
        let mut metadata = MediaDownloadActionMetadata::default();
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "mxc" => mxc = Some(value.into_owned()),
                "action" => action = Some(value.into_owned()),
                "label" => action_label = Some(value.into_owned()),
                "name" => filename = Some(value.into_owned()),
                "open" => open_after_save = value == "1",
                "kind" => metadata.kind = value.into_owned(),
                "mime" => metadata.mime_type = Some(value.into_owned()),
                "size" => metadata.size_label = Some(value.into_owned()),
                "duration" => metadata.duration_label = Some(value.into_owned()),
                "dimensions" => metadata.dimensions_label = Some(value.into_owned()),
                _ => {}
            }
        }

        let action = action
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "Open folder".to_string());
        let action_label = action_label
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| if open_after_save { "Play" } else { "Download" }.to_string());
        let filename = filename
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "hepta-media-download".to_string());
        let metadata_summary = metadata.summary(&filename);

        if action == "Retry" {
            let Some(mxc) = mxc else {
                enqueue_popup_notification(
                    media_save_retry_unavailable_label(&action_label, &filename),
                    PopupKind::Warning,
                    Some(4.0),
                );
                return true;
            };
            self.show_media_save_confirmation(
                cx,
                OwnedMxcUri::from(mxc),
                filename,
                open_after_save,
                metadata,
            );
            return true;
        }

        if action == "Open folder" {
            let Some(mxc) = mxc.as_deref().and_then(media_save_destination_cache_key) else {
                enqueue_popup_notification(
                    media_open_folder_unavailable_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        "no plain MXC source was available for this row",
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
                return true;
            };
            let Some(destination_path) = self
                .tl_state
                .as_ref()
                .and_then(|tl_state| tl_state.saved_media_destinations.get(mxc))
                .cloned()
            else {
                enqueue_popup_notification(
                    media_open_folder_unavailable_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        "no successful SaveMedia destination is cached for this row yet",
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
                return true;
            };
            if let Some(reason) = media_cached_saved_file_stale_reason(&destination_path) {
                if let Some(tl_state) = self.tl_state.as_mut() {
                    tl_state.saved_media_destinations.remove(mxc);
                }
                enqueue_popup_notification(
                    media_open_folder_unavailable_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        &format!("{reason}; cleared stale cached destination"),
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
                return true;
            }
            match open_media_saved_folder(&destination_path) {
                Ok(folder_path) => enqueue_popup_notification(
                    media_open_folder_result_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        &destination_path,
                        &folder_path,
                    ),
                    PopupKind::Success,
                    Some(4.0),
                ),
                Err(error) => enqueue_popup_notification(
                    media_open_folder_failed_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        &destination_path,
                        &error,
                    ),
                    PopupKind::Error,
                    Some(6.0),
                ),
            }
            return true;
        }

        if action == "Replay" {
            let Some(mxc) = mxc.as_deref().and_then(media_save_destination_cache_key) else {
                enqueue_popup_notification(
                    media_replay_unavailable_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        "no plain MXC source was available for this row",
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
                return true;
            };
            let Some(destination_path) = self
                .tl_state
                .as_ref()
                .and_then(|tl_state| tl_state.saved_media_destinations.get(mxc))
                .cloned()
            else {
                enqueue_popup_notification(
                    media_replay_unavailable_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        "no successful SaveMedia destination is cached for this row yet",
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
                return true;
            };
            if let Some(reason) = media_cached_saved_file_stale_reason(&destination_path) {
                if let Some(tl_state) = self.tl_state.as_mut() {
                    tl_state.saved_media_destinations.remove(mxc);
                }
                enqueue_popup_notification(
                    media_replay_unavailable_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        &format!("{reason}; cleared stale cached destination"),
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
                return true;
            }
            match open_media_saved_file(&destination_path) {
                Ok(()) => enqueue_popup_notification(
                    media_replay_result_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        &destination_path,
                    ),
                    PopupKind::Success,
                    Some(4.0),
                ),
                Err(error) => enqueue_popup_notification(
                    media_replay_failed_label(
                        &action_label,
                        &filename,
                        &metadata_summary,
                        &destination_path,
                        &error,
                    ),
                    PopupKind::Error,
                    Some(6.0),
                ),
            }
            return true;
        }

        let cached_saved_file_status = if action == "Queue" {
            self.media_cached_saved_file_status_for_mxc(mxc.as_deref())
        } else {
            None
        };

        let label = if action == "Queue" {
            media_playback_download_queue_snapshot_label(
                &action,
                &action_label,
                &filename,
                &metadata_summary,
                open_after_save,
                cached_saved_file_status.as_deref(),
            )
        } else {
            media_save_result_recovery_control_label(
                &action,
                &action_label,
                &filename,
                &metadata_summary,
                open_after_save,
            )
        };

        enqueue_popup_notification(label, PopupKind::Info, Some(4.0));
        true
    }

    fn media_cached_saved_file_status_for_mxc(&mut self, mxc: Option<&str>) -> Option<String> {
        let Some(cache_key) = mxc.and_then(media_save_destination_cache_key) else {
            return Some(
                "No plain MXC source was available, so no cached saved-file status was read."
                    .to_string(),
            );
        };
        let Some(destination_path) = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.saved_media_destinations.get(cache_key))
            .cloned()
        else {
            return Some(
                "No successful SaveMedia destination is cached for this row yet.".to_string(),
            );
        };

        match media_cached_saved_file_status_label(&destination_path) {
            Ok(label) => Some(label),
            Err(reason) => {
                if let Some(tl_state) = self.tl_state.as_mut() {
                    tl_state.saved_media_destinations.remove(cache_key);
                }
                Some(format!(
                    "{reason}; cleared stale cached destination. {MEDIA_CACHED_SAVED_FILE_STATUS_LABEL}"
                ))
            }
        }
    }

    fn handle_media_save_preflight_control_link(&mut self, cx: &mut Cx, url: &str) -> bool {
        let Ok(parsed) = url::Url::parse(url) else {
            return false;
        };
        if parsed.scheme() != MEDIA_SAVE_PREFLIGHT_CONTROL_URL_SCHEME {
            return false;
        }

        let mut mxc = None;
        let mut action = None;
        let mut action_label = None;
        let mut filename = None;
        let mut open_after_save = false;
        let mut metadata = MediaDownloadActionMetadata::default();
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "mxc" => mxc = Some(value.into_owned()),
                "action" => action = Some(value.into_owned()),
                "label" => action_label = Some(value.into_owned()),
                "name" => filename = Some(value.into_owned()),
                "open" => open_after_save = value == "1",
                "kind" => metadata.kind = value.into_owned(),
                "mime" => metadata.mime_type = Some(value.into_owned()),
                "size" => metadata.size_label = Some(value.into_owned()),
                "duration" => metadata.duration_label = Some(value.into_owned()),
                "dimensions" => metadata.dimensions_label = Some(value.into_owned()),
                _ => {}
            }
        }

        let action = action
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "Request".to_string());
        let action_label = action_label
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| if open_after_save { "Play" } else { "Download" }.to_string());
        let filename = filename
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "hepta-media-download".to_string());
        let metadata_summary = metadata.summary(&filename);

        if action == "Retry" {
            let Some(mxc) = mxc else {
                enqueue_popup_notification(
                    media_save_retry_unavailable_label(&action_label, &filename),
                    PopupKind::Warning,
                    Some(4.0),
                );
                return true;
            };
            self.show_media_save_confirmation(
                cx,
                OwnedMxcUri::from(mxc),
                filename,
                open_after_save,
                metadata,
            );
            return true;
        }

        enqueue_popup_notification(
            media_save_preflight_detail_control_label(
                &action,
                &action_label,
                &filename,
                &metadata_summary,
                open_after_save,
            ),
            PopupKind::Info,
            Some(4.0),
        );
        true
    }

    fn handle_media_codec_transcode_control_link(_cx: &mut Cx, url: &str) -> bool {
        let Ok(parsed) = url::Url::parse(url) else {
            return false;
        };
        if parsed.scheme() != MEDIA_CODEC_TRANSCODE_CONTROL_URL_SCHEME {
            return false;
        }

        let mut action = None;
        let mut action_label = None;
        let mut filename = None;
        let mut open_after_save = false;
        let mut metadata = MediaDownloadActionMetadata::default();
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "action" => action = Some(value.into_owned()),
                "label" => action_label = Some(value.into_owned()),
                "name" => filename = Some(value.into_owned()),
                "open" => open_after_save = value == "1",
                "kind" => metadata.kind = value.into_owned(),
                "mime" => metadata.mime_type = Some(value.into_owned()),
                "size" => metadata.size_label = Some(value.into_owned()),
                "duration" => metadata.duration_label = Some(value.into_owned()),
                "dimensions" => metadata.dimensions_label = Some(value.into_owned()),
                _ => {}
            }
        }

        let action = action
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "Codec".to_string());
        let action_label = action_label
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| if open_after_save { "Play" } else { "Download" }.to_string());
        let filename = filename
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "hepta-media-download".to_string());
        let metadata_summary = metadata.summary(&filename);

        enqueue_popup_notification(
            media_codec_transcode_control_label(
                &action,
                &action_label,
                &filename,
                &metadata_summary,
                open_after_save,
            ),
            PopupKind::Info,
            Some(4.0),
        );
        true
    }

    fn show_media_save_confirmation(
        &mut self,
        cx: &mut Cx,
        mxc_uri: OwnedMxcUri,
        suggested_filename: String,
        open_after_save: bool,
        metadata: MediaDownloadActionMetadata,
    ) {
        let action_label = if open_after_save { "Play" } else { "Download" };
        let accept_label = if open_after_save {
            "Save & Open"
        } else {
            "Choose Save Location"
        };
        let metadata_summary = metadata.summary(&suggested_filename);
        let update_sender = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.update_sender.clone());
        let filename_for_body = suggested_filename.clone();
        let filename_for_accept = suggested_filename.clone();
        let filename_for_cancel = suggested_filename.clone();
        let metadata_summary_for_accept = metadata_summary.clone();
        let metadata_summary_for_cancel = metadata_summary.clone();
        let action_label_for_accept = action_label.to_string();
        let action_label_for_cancel = action_label.to_string();
        let inline_boundary = media_inline_playback_queue_boundary_label(
            action_label,
            &suggested_filename,
            &metadata_summary,
            open_after_save,
        );
        let result_boundary = media_save_result_status_boundary_label(
            action_label,
            &suggested_filename,
            &metadata_summary,
            open_after_save,
        );
        let content = ConfirmationModalContent {
            title_text: format!("{action_label} Media").into(),
            body_text: format!(
                "{action_label} {filename_for_body}? {inline_boundary} {result_boundary}"
            )
            .into(),
            accept_button_text: Some(accept_label.into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                Self::submit_media_save_after_path_pick(
                    mxc_uri,
                    filename_for_accept,
                    open_after_save,
                    metadata_summary_for_accept,
                    action_label_for_accept,
                    update_sender,
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    media_save_dialog_lifecycle_metadata_label(
                        &action_label_for_cancel,
                        &filename_for_cancel,
                        &metadata_summary_for_cancel,
                        "confirmation canceled",
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
            media_save_dialog_lifecycle_metadata_label(
                action_label,
                &suggested_filename,
                &metadata_summary,
                "confirmation opened",
            ),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn submit_media_save_after_path_pick(
        mxc_uri: OwnedMxcUri,
        suggested_filename: String,
        open_after_save: bool,
        metadata_summary: String,
        action_label: String,
        update_sender: Option<crossbeam_channel::Sender<TimelineUpdate>>,
    ) {
        match pick_media_save_path(&suggested_filename) {
            MediaSavePathPickResult::Picked(destination_path) => {
                let destination_status = media_save_destination_metadata_label(
                    &action_label,
                    &suggested_filename,
                    &metadata_summary,
                    &destination_path,
                    open_after_save,
                );
                let result_boundary = media_save_result_status_boundary_label(
                    &action_label,
                    &suggested_filename,
                    &metadata_summary,
                    open_after_save,
                );
                submit_async_request(MatrixRequest::SaveMedia {
                    media_request: MediaRequestParameters {
                        source: MediaSource::Plain(mxc_uri),
                        format: MediaFormat::File,
                    },
                    destination_path,
                    open_after_save,
                    update_sender,
                });
                enqueue_popup_notification(
                    format!(
                        "{} {} {}",
                        media_save_dialog_lifecycle_metadata_label(
                            &action_label,
                            &suggested_filename,
                            &metadata_summary,
                            if open_after_save {
                                "requested after save location picked; saving first, then opening the saved file"
                            } else {
                                "requested after save location picked; saving to selected local path"
                            },
                        ),
                        destination_status,
                        result_boundary,
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            }
            MediaSavePathPickResult::Canceled => {
                enqueue_popup_notification(
                    media_save_dialog_lifecycle_metadata_label(
                        &action_label,
                        &suggested_filename,
                        &metadata_summary,
                        "save dialog canceled; no download request was sent",
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            }
            MediaSavePathPickResult::Unsupported => {
                enqueue_popup_notification(
                    media_save_dialog_lifecycle_metadata_label(
                        &action_label,
                        &suggested_filename,
                        &metadata_summary,
                        "save dialog unsupported on this platform",
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
            }
        }
    }

    /// Handles a link being clicked in any child widgets of this RoomScreen.
    ///
    /// Returns `true` if the given `action` was handled as a link click.
    fn handle_link_clicked(
        &mut self,
        cx: &mut Cx,
        action: &Action,
        pane: &UserProfileSlidingPaneRef,
        portal_list: &PortalListRef,
    ) -> bool {
        if let HtmlLinkAction::Clicked { url, .. } = action.as_widget_action().cast() {
            if Self::handle_media_metadata_clipboard_link(cx, &url) {
                return true;
            }
            if Self::handle_media_operation_packet_link(cx, &url) {
                return true;
            }
            if Self::handle_media_playback_queue_contract_link(cx, &url) {
                return true;
            }
            if Self::handle_media_playback_result_taxonomy_link(cx, &url) {
                return true;
            }
            if self.handle_media_download_link(cx, &url) {
                return true;
            }
            if self.handle_media_result_control_link(cx, &url) {
                return true;
            }
            if self.handle_media_save_preflight_control_link(cx, &url) {
                return true;
            }
            if Self::handle_media_codec_transcode_control_link(cx, &url) {
                return true;
            }
        }

        let mut handle_matrix_link = |id: &MatrixId, via: &[OwnedServerName]| -> bool {
            match id {
                MatrixId::User(user_id) => {
                    let Some(room_name_id) = self.room_name_id.as_ref() else {
                        return false;
                    };
                    let current_room_id = room_name_id.room_id().clone();
                    // There is no synchronous way to get the user's full profile info
                    // including the details of their room membership,
                    // so we fill in with the details we *do* know currently,
                    // show the UserProfileSlidingPane, and then after that,
                    // the UserProfileSlidingPane itself will fire off
                    // an async request to get the rest of the details.
                    // Known user links hand off to the local profile pane. Any
                    // later profile read belongs to the existing profile path,
                    // not Matrix link alias/event resolution.
                    self.show_user_profile(
                        cx,
                        pane,
                        UserProfilePaneInfo {
                            profile_and_room_id: UserProfileAndRoomId {
                                user_profile: UserProfile {
                                    user_id: user_id.to_owned(),
                                    username: None,
                                    avatar_state: AvatarState::Unknown,
                                },
                                room_id: room_name_id.room_id().clone(),
                            },
                            room_name: room_name_id.to_string(),
                            // TODO: use the extra `via` parameters
                            room_member: None,
                        },
                    );
                    let metadata = matrix_link_target_metadata_label(
                        "user",
                        user_id.as_str(),
                        via.len(),
                        "current room available",
                        "known user id profile-pane handoff",
                        "no event id",
                        "profile pane existing read path plus confirmed InviteUser action",
                    );
                    self.show_telegram_matrix_link_user_target(
                        cx,
                        user_id,
                        via,
                        current_room_id,
                        metadata.clone(),
                    );
                    enqueue_popup_notification(
                        format!("Opened Matrix user link locally. {metadata}"),
                        PopupKind::Info,
                        Some(4.0),
                    );
                    true
                }
                MatrixId::Room(room_id) => {
                    if self
                        .room_name_id
                        .as_ref()
                        .is_some_and(|r| r.room_id() == room_id)
                    {
                        enqueue_popup_notification(
                            format!(
                                "You are already viewing that room. {}",
                                matrix_link_target_metadata_label(
                                    "room id",
                                    room_id.as_str(),
                                    via.len(),
                                    "target is current room",
                                    "current room already open",
                                    "no event id",
                                    "no preview read requested",
                                )
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                        return true;
                    }
                    if let Some(room_name_id) =
                        cx.get_global::<RoomsListRef>().get_room_name(room_id)
                    {
                        // Known room links navigate using loaded RoomsList state.
                        cx.action(AppStateAction::NavigateToRoom {
                            room_to_close: None,
                            destination_room: BasicRoomDetails::Name(room_name_id),
                        });
                        enqueue_popup_notification(
                            format!(
                                "Opened known Matrix room link locally. {}",
                                matrix_link_target_metadata_label(
                                    "room id",
                                    room_id.as_str(),
                                    via.len(),
                                    "target is a different joined room",
                                    "known room in loaded RoomsList",
                                    "no event id",
                                    "local NavigateToRoom",
                                )
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                        return true;
                    } else {
                        self.show_matrix_link_preview_request(
                            cx,
                            "Room",
                            room_id.to_string(),
                            room_id.to_owned().into(),
                            via.to_vec(),
                            None,
                            matrix_link_target_metadata_label(
                                "room id",
                                room_id.as_str(),
                                via.len(),
                                "target is not the current room",
                                "no loaded joined-room match",
                                "no event id",
                                "compact PreviewMatrixLinkTarget room preview read requested",
                            ),
                        );
                    }
                    true
                }
                MatrixId::RoomAlias(room_alias) => {
                    if let Some(room_name_id) = cx
                        .get_global::<RoomsListRef>()
                        .get_joined_room_name_by_alias(room_alias.as_str())
                    {
                        if self
                            .room_name_id
                            .as_ref()
                            .is_some_and(|current| current.room_id() == room_name_id.room_id())
                        {
                            enqueue_popup_notification(
                                format!(
                                    "You are already viewing loaded alias {room_alias}. {}",
                                    matrix_link_target_metadata_label(
                                        "room alias",
                                        room_alias.as_str(),
                                        via.len(),
                                        "alias resolves to current room",
                                        "loaded RoomsList canonical/alt alias match",
                                        "no event id",
                                        "no preview read requested",
                                    )
                                ),
                                PopupKind::Info,
                                Some(4.0),
                            );
                            return true;
                        }
                        let alias_label = room_alias.to_string();
                        let destination_label = room_name_id.to_string();
                        cx.action(AppStateAction::NavigateToRoom {
                            room_to_close: None,
                            destination_room: BasicRoomDetails::Name(room_name_id),
                        });
                        enqueue_popup_notification(
                            format!(
                                "Opened loaded Matrix alias {alias_label} as {destination_label}. {}",
                                matrix_link_target_metadata_label(
                                    "room alias",
                                    &alias_label,
                                    via.len(),
                                    "alias resolves to a loaded joined room",
                                    "loaded RoomsList canonical/alt alias match",
                                    "no event id",
                                    "local NavigateToRoom",
                                )
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                        return true;
                    }
                    self.show_matrix_link_preview_request(
                        cx,
                        "Room alias",
                        room_alias.to_string(),
                        room_alias.to_owned().into(),
                        via.to_vec(),
                        None,
                        matrix_link_target_metadata_label(
                            "room alias",
                            room_alias.as_str(),
                            via.len(),
                            "alias is not loaded as current room",
                            "no loaded RoomsList alias match",
                            "no event id",
                            "compact PreviewMatrixLinkTarget room preview read requested",
                        ),
                    );
                    true
                }
                MatrixId::Event(room_id, event_id) => {
                    let target = format!("{event_id} in {room_id}");
                    let loaded_event = self
                        .loaded_matrix_link_event_index(room_id, event_id)
                        .is_some();
                    let event_metadata =
                        matrix_link_target_metadata_label(
                            "event",
                            &target,
                            via.len(),
                            if self.room_name_id.as_ref().is_some_and(|current| {
                                current.room_id().as_str() == room_id.as_str()
                            }) {
                                "event room is current room"
                            } else {
                                "event room is not current room"
                            },
                            if loaded_event {
                                "event id found in loaded RoomScreen timeline"
                            } else {
                                "event target not available in loaded timeline"
                            },
                            if loaded_event {
                                "event id loaded"
                            } else {
                                "event id missing from loaded rows"
                            },
                            if loaded_event {
                                "local scroll/highlight"
                            } else {
                                "compact PreviewMatrixLinkTarget room preview read requested"
                            },
                        );
                    if self.jump_to_loaded_matrix_link_event(
                        cx,
                        room_id,
                        event_id,
                        via,
                        portal_list,
                        Some(event_metadata.clone()),
                    ) {
                        return true;
                    }
                    if self.paginate_current_room_matrix_link_event(
                        cx,
                        room_id,
                        event_id,
                        via,
                        portal_list,
                        event_metadata.clone(),
                    ) {
                        return true;
                    }
                    self.show_matrix_link_preview_request(
                        cx,
                        "Event",
                        target,
                        room_id.to_owned().into(),
                        via.to_vec(),
                        Some(event_id.to_owned()),
                        event_metadata,
                    );
                    true
                }
                _ => false,
            }
        };

        if let HtmlLinkAction::Clicked { url, .. } = action.as_widget_action().cast() {
            let mut link_was_handled = false;
            if let Ok(matrix_to_uri) = MatrixToUri::parse(&url) {
                // matrix.to links use the same local Matrix link handler.
                link_was_handled |= handle_matrix_link(matrix_to_uri.id(), matrix_to_uri.via());
            } else if let Ok(matrix_uri) = MatrixUri::parse(&url) {
                // matrix: links use the same local Matrix link handler.
                link_was_handled |= handle_matrix_link(matrix_uri.id(), matrix_uri.via());
            }

            if !link_was_handled {
                Self::show_external_link_confirmation(cx, url);
            }
            true
        } else if let RobrixHtmlLinkAction::ClickedMatrixLink {
            url,
            matrix_id,
            via,
            ..
        } = action.as_widget_action().cast()
        {
            let link_was_handled = handle_matrix_link(&matrix_id, &via);
            if !link_was_handled {
                Self::show_external_link_confirmation(cx, url);
            }
            true
        } else {
            false
        }
    }

    fn paginate_current_room_matrix_link_event(
        &mut self,
        cx: &mut Cx,
        room_or_alias_id: &OwnedRoomOrAliasId,
        event_id: &OwnedEventId,
        via: &[OwnedServerName],
        portal_list: &PortalListRef,
        metadata: String,
    ) -> bool {
        if !self
            .room_name_id
            .as_ref()
            .is_some_and(|current| current.room_id().as_str() == room_or_alias_id.as_str())
        {
            return false;
        }
        if self.tl_state.is_none() {
            return false;
        }

        let target_label = format!("{event_id} in {room_or_alias_id}");
        let via_count = via.len();
        let event_id_label = event_id.to_string();
        self.telegram_matrix_link_preview_retry_room_or_alias_id = None;
        self.telegram_matrix_link_preview_retry_via.clear();
        self.telegram_matrix_link_preview_retry_event_id = None;
        self.telegram_matrix_link_preview_retry_timeline_kind = None;
        self.telegram_matrix_link_preview_status = "paginating".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Current-room Matrix event link is loading older timeline items for {event_id}. {MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_LABEL}"
        );
        self.telegram_matrix_link_preview_metadata =
            format!("{metadata} {MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_LABEL}");
        self.telegram_matrix_link_preview_target_label = target_label;
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.to_owned());
        self.telegram_matrix_link_preview_via_count = via_count;
        self.telegram_matrix_link_preview_via_label = matrix_link_via_servers_label(via);
        self.telegram_matrix_link_preview_event_id_label = event_id_label;
        self.telegram_matrix_link_preview_error_chars = None;
        self.telegram_matrix_link_preview_source_room_id = None;
        self.telegram_matrix_link_preview_source_event_id = None;
        self.telegram_matrix_link_preview_source_json.clear();
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Current-room event pagination"));
        self.telegram_matrix_link_route_scope_metadata = matrix_link_route_scope_controls_label(
            Some("Event pagination"),
            "paginating",
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            false,
        );
        self.telegram_matrix_link_context_actions_metadata = matrix_link_context_actions_row_label(
            Some("Event pagination"),
            "paginating",
            via_count,
            Some(event_id),
            false,
        );
        self.telegram_matrix_link_server_context_boundary =
            matrix_link_server_context_boundary_label(
                "current-room event pagination read requested",
                via_count,
                Some(event_id),
                false,
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

        let loading_pane = self.view.loading_pane(cx, ids!(loading_pane));
        self.jump_to_event(cx, event_id, None, portal_list, &loading_pane);
        enqueue_popup_notification(
            format!(
                "Loading older timeline items for Matrix event {event_id}. {MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_LABEL}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
        true
    }

    fn show_telegram_matrix_link_paginated_event_found(
        &mut self,
        cx: &mut Cx,
        event_id: &OwnedEventId,
        loaded_index: usize,
        snippet: &str,
    ) {
        let context_metadata = matrix_link_loaded_event_context_metadata_label(
            event_id,
            loaded_index,
            "event room is current room",
            "event id found after BackwardsPaginateUntilEvent pagination",
            snippet,
        );
        self.telegram_matrix_link_preview_retry_room_or_alias_id = None;
        self.telegram_matrix_link_preview_retry_via.clear();
        self.telegram_matrix_link_preview_retry_event_id = None;
        self.telegram_matrix_link_preview_retry_timeline_kind = None;
        self.telegram_matrix_link_preview_status = "loaded".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Current-room Matrix event link loaded via timeline pagination for {event_id}. {MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_LABEL}"
        );
        self.telegram_matrix_link_preview_metadata = format!(
            "{context_metadata} {MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_LABEL} {MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_LABEL}"
        );
        self.telegram_matrix_link_preview_error_chars = None;
        self.telegram_matrix_link_preview_source_room_id = None;
        self.telegram_matrix_link_preview_source_event_id = None;
        self.telegram_matrix_link_preview_source_json.clear();
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Current-room event loaded"));
        self.telegram_matrix_link_route_scope_metadata = matrix_link_route_scope_controls_label(
            None,
            "loaded",
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            false,
        );
        self.telegram_matrix_link_context_actions_metadata = matrix_link_context_actions_row_label(
            None,
            "loaded",
            self.telegram_matrix_link_preview_via_count,
            Some(event_id),
            false,
        );
        self.telegram_matrix_link_server_context_boundary =
            matrix_link_server_context_boundary_label(
                "current-room event pagination found target",
                self.telegram_matrix_link_preview_via_count,
                Some(event_id),
                false,
            );
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Matrix event {event_id} found after timeline pagination. {MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_LABEL}"
            ),
            PopupKind::Success,
            Some(4.0),
        );
    }

    fn jump_to_loaded_matrix_link_event(
        &mut self,
        cx: &mut Cx,
        room_or_alias_id: &OwnedRoomOrAliasId,
        event_id: &EventId,
        via: &[OwnedServerName],
        portal_list: &PortalListRef,
        metadata: Option<String>,
    ) -> bool {
        let Some(index) = self.loaded_matrix_link_event_index(room_or_alias_id, event_id) else {
            return false;
        };
        let via_count = via.len();
        let context_metadata =
            self.loaded_matrix_link_event_context_metadata(room_or_alias_id, event_id, index);
        let target_label = format!("{event_id} in {room_or_alias_id}");
        let event_id_label = event_id.to_string();
        let event_id_owned = event_id.to_owned();
        let metadata =
            metadata.unwrap_or_else(|| MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_LABEL.to_string());
        self.telegram_matrix_link_preview_retry_room_or_alias_id = None;
        self.telegram_matrix_link_preview_retry_via.clear();
        self.telegram_matrix_link_preview_retry_event_id = None;
        self.telegram_matrix_link_preview_retry_timeline_kind = None;
        self.telegram_matrix_link_preview_status = "loaded".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Loaded Matrix event link opened locally for {event_id}. {MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_LABEL}"
        );
        self.telegram_matrix_link_preview_metadata = format!("{metadata} {context_metadata}");
        self.telegram_matrix_link_preview_target_label = target_label;
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.to_owned());
        self.telegram_matrix_link_preview_via_count = via_count;
        self.telegram_matrix_link_preview_via_label = matrix_link_via_servers_label(via);
        self.telegram_matrix_link_preview_event_id_label = event_id_label;
        self.telegram_matrix_link_preview_error_chars = None;
        self.telegram_matrix_link_preview_source_room_id = None;
        self.telegram_matrix_link_preview_source_event_id = None;
        self.telegram_matrix_link_preview_source_json.clear();
        self.refresh_telegram_matrix_link_unresolved_detail(None);
        self.telegram_matrix_link_route_scope_metadata = matrix_link_route_scope_controls_label(
            None,
            "loaded",
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            false,
        );
        self.telegram_matrix_link_context_actions_metadata = matrix_link_context_actions_row_label(
            None,
            "loaded",
            via_count,
            Some(&event_id_owned),
            false,
        );
        self.telegram_matrix_link_server_context_boundary =
            matrix_link_server_context_boundary_label(
                "loaded event local jump",
                via_count,
                Some(&event_id_owned),
                false,
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

        portal_list.smooth_scroll_to(cx, index, 50.0, None, 10.0);
        let Some(tl) = self.tl_state.as_mut() else {
            return false;
        };
        tl.message_highlight_animation_state =
            MessageHighlightAnimationState::Pending { item_id: index };
        enqueue_popup_notification(
            format!(
                "Opened loaded Matrix event link locally. {} {}",
                self.telegram_matrix_link_preview_metadata,
                MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_LABEL
            ),
            PopupKind::Info,
            Some(4.0),
        );
        true
    }

    fn loaded_matrix_link_event_index(
        &self,
        room_or_alias_id: &OwnedRoomOrAliasId,
        event_id: &EventId,
    ) -> Option<usize> {
        if !self
            .room_name_id
            .as_ref()
            .is_some_and(|current| current.room_id().as_str() == room_or_alias_id.as_str())
        {
            return None;
        }

        let tl = self.tl_state.as_ref()?;
        tl.items.iter().position(|item| {
            item.as_event()
                .and_then(|event| event.event_id())
                .is_some_and(|loaded_event_id| loaded_event_id == event_id)
        })
    }

    fn loaded_matrix_link_event_context_metadata(
        &self,
        room_or_alias_id: &OwnedRoomOrAliasId,
        event_id: &EventId,
        loaded_index: usize,
    ) -> String {
        let current_room_state = if self
            .room_name_id
            .as_ref()
            .is_some_and(|current| current.room_id().as_str() == room_or_alias_id.as_str())
        {
            "event room is current room"
        } else {
            "event room is not current room"
        };
        let (loaded_event_state, snippet) = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.items.get(loaded_index))
            .and_then(|item| item.as_event())
            .map(|event_tl_item| {
                let loaded_event_state = event_tl_item
                    .event_id()
                    .map(|loaded_event_id| {
                        if loaded_event_id == event_id {
                            "loaded event id matches target"
                        } else {
                            "loaded event id differs from target"
                        }
                    })
                    .unwrap_or("loaded event id missing");
                (
                    loaded_event_state,
                    compact_message_preview(
                        &plaintext_body_of_timeline_item(event_tl_item),
                        "empty loaded event snippet",
                    ),
                )
            })
            .unwrap_or_else(|| {
                (
                    "loaded event row unavailable",
                    "loaded event snippet unavailable".to_string(),
                )
            });
        matrix_link_loaded_event_context_metadata_label(
            event_id,
            loaded_index,
            current_room_state,
            loaded_event_state,
            &snippet,
        )
    }

    /// Handles image clicks in message content by opening the image viewer.
    fn handle_image_click(
        &mut self,
        cx: &mut Cx,
        mxc_uri: Option<MediaSource>,
        texture: Option<Texture>,
        item_id: usize,
    ) {
        let Some(media_source) = mxc_uri else {
            return;
        };
        let Some(tl_state) = self.tl_state.as_mut() else {
            return;
        };
        let Some(event_tl_item) = tl_state.items.get(item_id).and_then(|item| item.as_event())
        else {
            return;
        };

        let timestamp_millis = event_tl_item.timestamp();
        let (image_name, image_file_size) = get_image_name_and_filesize(event_tl_item);
        cx.action(ImageViewerAction::Show(LoadState::Loading(
            texture.clone(),
            Some(ImageViewerMetaData {
                image_name,
                image_file_size,
                timestamp: unix_time_millis_to_datetime(timestamp_millis),
                avatar_parameter: Some((tl_state.kind.clone(), event_tl_item.clone())),
                downloadable: None,
            }),
        )));

        populate_matrix_image_modal(cx, media_source, &mut tl_state.media_cache);
    }

    /// Looks up the event specified by the given message details in the given timeline.
    ///
    /// This will first try an instant index-based lookup via `details.item_id`,
    /// and then fall back to searching the timeline in reverse for the `details.event_id`
    /// if the index is "stale", meaning the timeline items have changed (e.g., due to pagination)
    /// since the message context menu was opened or the `MessageAction` was received by the `RoomScreen`.
    ///
    /// We search in reverse because it is far more likely that the user is interacting
    /// with an event that is close to the end of the timeline.
    fn find_event_in_timeline<'a>(
        items: &'a Vector<Arc<TimelineItem>>,
        details: &MessageDetails,
    ) -> Option<&'a EventTimelineItem> {
        let target_event_id = details.event_id()?;
        if let Some(event) = items
            .get(details.item_id)
            .and_then(|item| item.as_event())
            .filter(|ev| ev.event_id().is_some_and(|id| id == target_event_id))
        {
            return Some(event);
        }
        items
            .iter()
            .rev()
            .take(MAX_ITEMS_TO_SEARCH_THROUGH)
            .filter_map(|item| item.as_event())
            .find(|ev| ev.event_id().is_some_and(|id| id == target_event_id))
    }

    /// Handles any [`MessageAction`]s received by this RoomScreen.
    fn handle_message_actions(
        &mut self,
        cx: &mut Cx,
        actions: &ActionsBuf,
        portal_list: &PortalListRef,
        loading_pane: &LoadingPaneRef,
    ) {
        let room_screen_widget_uid = self.widget_uid();
        for action in actions {
            match action
                .as_widget_action()
                .widget_uid_eq(room_screen_widget_uid)
                .cast_ref()
            {
                MessageAction::React { details, reaction } => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    submit_async_request(MatrixRequest::ToggleReaction {
                        timeline_kind: tl.kind.clone(),
                        timeline_event_id: details.timeline_event_id.clone(),
                        reaction: reaction.clone(),
                    });
                }
                MessageAction::Reply(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    if let Some(event_tl_item) =
                        Self::find_event_in_timeline(&tl.items, details).cloned()
                    {
                        let replied_to_info = EmbeddedEvent::from_timeline_item(&event_tl_item);
                        self.view
                            .room_input_bar(cx, ids!(room_input_bar))
                            .show_replying_to(cx, (event_tl_item, replied_to_info), &tl.kind);
                    } else {
                        enqueue_popup_notification(
                            "Could not find message in timeline to reply to. Please try again.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        error!(
                            "MessageAction::Reply: couldn't find event [{}] {:?} to reply to in room {:?}",
                            details.item_id,
                            details.timeline_event_id,
                            self.room_id(),
                        );
                    }
                }
                MessageAction::ReplyInThread(details) => {
                    let Some(room_name_id) = self.room_name_id.as_ref().cloned() else {
                        error!(
                            "MessageAction::ReplyInThread had no room identity for {:?}",
                            self.room_id()
                        );
                        continue;
                    };
                    let Some(thread_root_event_id) = details
                        .thread_root_event_id
                        .clone()
                        .or_else(|| details.event_id().cloned())
                    else {
                        enqueue_popup_notification(
                            "Cannot reply in thread to an unsent message.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        continue;
                    };
                    cx.widget_action(
                        room_screen_widget_uid,
                        RoomsListAction::Selected(SelectedRoom::Thread {
                            room_name_id,
                            thread_root_event_id,
                        }),
                    );
                }
                MessageAction::Edit(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    if let Some(event_tl_item) = Self::find_event_in_timeline(&tl.items, details) {
                        self.view
                            .room_input_bar(cx, ids!(room_input_bar))
                            .show_editing_pane(cx, event_tl_item.clone(), tl.kind.clone());
                    } else {
                        enqueue_popup_notification(
                            "Could not find message in timeline to edit. Please try again.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        error!(
                            "MessageAction::Edit: couldn't find event [{}] {:?} to edit in room {:?}",
                            details.item_id,
                            details.timeline_event_id,
                            self.room_id(),
                        );
                    }
                }
                MessageAction::EditLatest => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    if let Some(latest_sent_msg) = tl
                        .items
                        .iter()
                        .rev()
                        .take(MAX_ITEMS_TO_SEARCH_THROUGH)
                        .find_map(|item| item.as_event().filter(|ev| ev.is_editable()).cloned())
                    {
                        self.view
                            .room_input_bar(cx, ids!(room_input_bar))
                            .show_editing_pane(cx, latest_sent_msg, tl.kind.clone());
                    } else {
                        enqueue_popup_notification(
                            "No recent message available to edit. Please manually select a message to edit.",
                            PopupKind::Warning,
                            Some(5.0),
                        );
                    }
                }
                MessageAction::Pin(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    if let Some(event_id) = details.event_id() {
                        submit_async_request(MatrixRequest::PinEvent {
                            timeline_kind: tl.kind.clone(),
                            event_id: event_id.clone(),
                            pin: true,
                        });
                    } else {
                        enqueue_popup_notification(
                            "This event cannot be pinned.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                    }
                }
                MessageAction::Unpin(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    if let Some(event_id) = details.event_id() {
                        submit_async_request(MatrixRequest::PinEvent {
                            timeline_kind: tl.kind.clone(),
                            event_id: event_id.clone(),
                            pin: false,
                        });
                    } else {
                        enqueue_popup_notification(
                            "This event cannot be unpinned.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                    }
                }
                MessageAction::CopyText(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    // Message copy clipboard evidence: Copy Text reads the
                    // loaded timeline item and only writes local clipboard
                    // text. It sends no Matrix event fetch, message send, edit,
                    // redact, room-state, membership, or live mutation request.
                    if let Some(event_tl_item) = Self::find_event_in_timeline(&tl.items, details) {
                        let payload = plaintext_body_of_timeline_item(event_tl_item);
                        let metadata = loaded_message_copy_metadata_label(
                            "plain text",
                            &payload,
                            details.event_id().map(|event_id| event_id.as_str()),
                        );
                        cx.copy_to_clipboard(&payload);
                        enqueue_popup_notification(
                            format!("Copied message text. {metadata}"),
                            PopupKind::Info,
                            Some(3.0),
                        );
                    } else {
                        enqueue_popup_notification(
                            "Could not find message in timeline to copy text from. Please try again.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        error!(
                            "MessageAction::CopyText: couldn't find event [{}] {:?} to copy text from in room {}",
                            details.item_id,
                            details.timeline_event_id,
                            tl.kind.room_id(),
                        );
                    }
                }
                MessageAction::CopyHtml(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    // Message copy clipboard evidence: Copy Text as HTML reads
                    // the loaded formatted body, if present, and only writes
                    // local clipboard text. It sends no Matrix event fetch,
                    // message send, edit, redact, room-state, membership, or
                    // live mutation request.
                    // The logic for getting the formatted body of a message is the same
                    // as the logic used in `populate_message_view()`.
                    let mut metadata = None;
                    if let Some(event_tl_item) = Self::find_event_in_timeline(&tl.items, details) {
                        if let Some(message) = event_tl_item.content().as_message() {
                            match message.msgtype() {
                                MessageType::Text(TextMessageEventContent {
                                    formatted: Some(FormattedBody { body, .. }),
                                    ..
                                })
                                | MessageType::Notice(NoticeMessageEventContent {
                                    formatted: Some(FormattedBody { body, .. }),
                                    ..
                                })
                                | MessageType::Emote(EmoteMessageEventContent {
                                    formatted: Some(FormattedBody { body, .. }),
                                    ..
                                })
                                | MessageType::Image(ImageMessageEventContent {
                                    formatted: Some(FormattedBody { body, .. }),
                                    ..
                                })
                                | MessageType::File(FileMessageEventContent {
                                    formatted: Some(FormattedBody { body, .. }),
                                    ..
                                })
                                | MessageType::Audio(AudioMessageEventContent {
                                    formatted: Some(FormattedBody { body, .. }),
                                    ..
                                })
                                | MessageType::Video(VideoMessageEventContent {
                                    formatted: Some(FormattedBody { body, .. }),
                                    ..
                                })
                                | MessageType::VerificationRequest(
                                    KeyVerificationRequestEventContent {
                                        formatted: Some(FormattedBody { body, .. }),
                                        ..
                                    },
                                ) => {
                                    cx.copy_to_clipboard(body);
                                    metadata = Some(loaded_message_copy_metadata_label(
                                        "HTML",
                                        body,
                                        details.event_id().map(|event_id| event_id.as_str()),
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                    if let Some(metadata) = metadata {
                        enqueue_popup_notification(
                            format!("Copied message HTML. {metadata}"),
                            PopupKind::Info,
                            Some(3.0),
                        );
                    } else {
                        enqueue_popup_notification(
                            "Could not find message in timeline to copy HTML from. Please try again.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        error!(
                            "MessageAction::CopyHtml: couldn't find event [{}] {:?} to copy HTML from in room {}",
                            details.item_id,
                            details.timeline_event_id,
                            tl.kind.room_id(),
                        );
                    }
                }
                MessageAction::CopyLink(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    // Message copy clipboard evidence: Copy Link constructs a
                    // matrix.to URI from the loaded room id and event id, then
                    // only writes local clipboard text. It sends no Matrix
                    // event fetch, message send, edit, redact, room-state,
                    // membership, or live mutation request.
                    if let Some(event_id) = details.event_id() {
                        let matrix_to_uri = tl.kind.room_id().matrix_to_event_uri(event_id.clone());
                        let payload = matrix_to_uri.to_string();
                        let metadata = loaded_message_copy_metadata_label(
                            "matrix.to link",
                            &payload,
                            Some(event_id.as_str()),
                        );
                        cx.copy_to_clipboard(&payload);
                        enqueue_popup_notification(
                            format!("Copied message link. {metadata}"),
                            PopupKind::Info,
                            Some(3.0),
                        );
                    } else {
                        enqueue_popup_notification(
                            "Couldn't create permalink to message. Please try again.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        error!(
                            "MessageAction::CopyLink: no `event_id`: [{}] {:?} in room {}",
                            details.item_id,
                            details.timeline_event_id,
                            tl.kind.room_id(),
                        );
                    }
                }
                MessageAction::ViewSource(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        continue;
                    };
                    let Some(event_tl_item) = Self::find_event_in_timeline(&tl.items, details)
                    else {
                        enqueue_popup_notification(
                            "Could not find message in timeline to view source.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        continue;
                    };
                    // Get the latest JSON from the event and pretty-print it
                    let latest_json: Option<String> = event_tl_item
                        .latest_json()
                        .and_then(|raw_event| serde_json::to_value(raw_event).ok())
                        .and_then(|value| serde_json::to_string_pretty(&value).ok());

                    let event_id = event_tl_item.event_id().map(|e| e.to_owned());

                    cx.action(super::event_source_modal::EventSourceModalAction::Open {
                        room_id: tl.kind.room_id().clone(),
                        event_id,
                        latest_json,
                    });
                }
                MessageAction::JumpToRelated(details) => {
                    let Some(related_event_id) = details.related_event_id.as_ref() else {
                        error!(
                            "BUG: MessageAction::JumpToRelated had no related event ID.\n{details:#?}"
                        );
                        enqueue_popup_notification(
                            "Could not find related message or event in timeline.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        continue;
                    };
                    self.jump_to_event(
                        cx,
                        related_event_id,
                        Some(details.item_id),
                        portal_list,
                        loading_pane,
                    );
                }
                MessageAction::JumpToEvent(event_id) => {
                    self.jump_to_event(cx, event_id, None, portal_list, loading_pane);
                }
                MessageAction::OpenThread(thread_root_event_id) => {
                    let Some(room_name_id) = self.room_name_id.as_ref().cloned() else {
                        error!(
                            "### ERROR: MessageAction::OpenThread: thread_root_event_id: {thread_root_event_id}, but room_name_id was None!"
                        );
                        continue;
                    };
                    cx.widget_action(
                        room_screen_widget_uid,
                        RoomsListAction::Selected(SelectedRoom::Thread {
                            room_name_id,
                            thread_root_event_id: thread_root_event_id.clone(),
                        }),
                    );
                }
                MessageAction::Redact { details, reason } => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    let timeline_event_id = details.timeline_event_id.clone();
                    let timeline_kind = tl.kind.clone();
                    let reason = reason.clone();
                    let content = ConfirmationModalContent {
                        title_text: "Delete Message".into(),
                        body_text:
                            "Are you sure you want to delete this message? This cannot be undone."
                                .into(),
                        accept_button_text: Some("Delete".into()),
                        on_accept_clicked: Some(Box::new(move |_cx| {
                            submit_async_request(MatrixRequest::RedactMessage {
                                timeline_kind,
                                timeline_event_id,
                                reason,
                            });
                        })),
                        ..Default::default()
                    };
                    cx.action(ConfirmDeleteAction::Show(RefCell::new(Some(content))));
                }
                MessageAction::Report { details, reason } => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    let timeline_kind = tl.kind.clone();
                    let Some(event_id) = details.event_id() else {
                        enqueue_popup_notification(
                            "This pending local event cannot be reported.",
                            PopupKind::Error,
                            Some(5.0),
                        );
                        continue;
                    };
                    self.show_telegram_message_report_submitted(cx, event_id, reason);
                    submit_async_request(MatrixRequest::ReportContent {
                        timeline_kind,
                        event_id: event_id.clone(),
                        reason: reason.clone(),
                    });
                }
                MessageAction::RetryReport { event_id, reason } => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    let timeline_kind = tl.kind.clone();
                    self.show_telegram_message_report_submitted(cx, event_id, reason);
                    submit_async_request(MatrixRequest::ReportContent {
                        timeline_kind,
                        event_id: event_id.clone(),
                        reason: reason.clone(),
                    });
                }
                MessageAction::CancelLocalSend(details) => {
                    let Some(tl) = self.tl_state.as_ref() else {
                        return;
                    };
                    let Some(send_handle) = details.local_send_handle.clone() else {
                        enqueue_popup_notification(
                            "This timeline row no longer has a cancellable SDK send handle.",
                            PopupKind::Warning,
                            Some(4.0),
                        );
                        continue;
                    };
                    submit_async_request(MatrixRequest::AbortLocalSend {
                        timeline_kind: tl.kind.clone(),
                        send_handle,
                    });
                }

                // This is handled within the Message widget itself.
                MessageAction::HighlightMessage(..) => {}
                // This is handled by the top-level App itself.
                MessageAction::OpenMessageContextMenu { .. } => {}
                // This isn't yet handled, as we need to completely redesign it.
                MessageAction::ActionBarOpen { .. } => {}
                // This isn't yet handled, as we need to completely redesign it.
                MessageAction::ActionBarClose => {}
                MessageAction::None => {}
            }
        }
    }

    /// Jumps to the target event ID in this timeline by smooth scrolling to it.
    ///
    /// This function searches backwards from the given `max_tl_idx` in the timeline
    /// for the given `event_id`. If found, it smooth-scrolls the portal list to that event.
    /// If not found, it displays the loading pane and starts a background search for the event.
    fn jump_to_event(
        &mut self,
        cx: &mut Cx,
        target_event_id: &OwnedEventId,
        max_tl_idx: Option<usize>,
        portal_list: &PortalListRef,
        loading_pane: &LoadingPaneRef,
    ) {
        let Some(tl) = self.tl_state.as_mut() else {
            return;
        };
        let max_tl_idx = max_tl_idx.unwrap_or_else(|| tl.items.len());

        // Attempt to find the index of replied-to message in the timeline.
        // Start from the current item's index (`tl_idx`) and search backwards,
        // since we know the related message must come before the current item.
        let mut num_items_searched = 0;
        let related_msg_tl_index = tl
            .items
            .focus()
            .narrow(..max_tl_idx)
            .into_iter()
            .rev()
            .take(MAX_ITEMS_TO_SEARCH_THROUGH)
            .position(|i| {
                num_items_searched += 1;
                i.as_event()
                    .and_then(|e| e.event_id())
                    .is_some_and(|ev_id| ev_id == target_event_id)
            })
            .map(|position| max_tl_idx.saturating_sub(position).saturating_sub(1));

        if let Some(index) = related_msg_tl_index {
            // log!("The related message {replied_to_event} was immediately found in room {}, scrolling to from index {reply_message_item_id} --> {index} (first ID {}).", tl.kind.room_id(), portal_list.first_id());
            let speed = 50.0;
            portal_list.smooth_scroll_to(cx, index, speed, None, 10.0);
            // start highlight animation.
            tl.message_highlight_animation_state =
                MessageHighlightAnimationState::Pending { item_id: index };
        } else {
            log!(
                "The related event {target_event_id} wasn't immediately available in room {}, searching for it in the background...",
                tl.kind.room_id()
            );
            // Here, we set the state of the loading pane and display it to the user.
            // The main logic will be handled in `process_timeline_updates()`, which is the only
            // place where we can receive updates to the timeline from the background tasks.
            loading_pane.set_state(
                cx,
                LoadingPaneState::BackwardsPaginateUntilEvent {
                    target_event_id: target_event_id.clone(),
                    events_paginated: 0,
                    request_sender: tl.request_sender.clone(),
                },
            );
            loading_pane.show(cx);

            tl.request_sender.send_if_modified(|requests| {
            if let Some(existing) = requests.iter_mut().find(|r| &r.room_id == tl.kind.room_id()) {
                warning!("Unexpected: room {} already had an existing timeline request in progress, event: {:?}", tl.kind.room_id(), existing.target_event_id);
                // We might as well re-use this existing request...
                existing.target_event_id = target_event_id.clone();
            } else {
                requests.push(BackwardsPaginateUntilEventRequest {
                    room_id: tl.kind.room_id().clone(),
                    target_event_id: target_event_id.clone(),
                    // avoid re-searching through items we already searched through.
                    starting_index: max_tl_idx.saturating_sub(num_items_searched),
                    current_tl_len: tl.items.len(),
                });
            }
            true
        });

            // Don't unconditionally start backwards pagination here, because we want to give the
            // background `timeline_subscriber_handler` task a chance to process the request first
            // and search our locally-known timeline history for the replied-to message.
        }
        self.redraw(cx);
    }

    /// Shows the user profile sliding pane with the given avatar info.
    fn show_user_profile(
        &mut self,
        cx: &mut Cx,
        pane: &UserProfileSlidingPaneRef,
        info: UserProfilePaneInfo,
    ) {
        pane.set_info(cx, info);
        pane.show(cx);
        self.redraw(cx);
    }

    /// Invoke this when this timeline is being shown,
    /// e.g., when the user navigates to this timeline.
    fn show_timeline(&mut self, cx: &mut Cx) {
        let kind = self
            .timeline_kind
            .clone()
            .expect("BUG: Timeline::show_timeline(): no timeline_kind was set.");
        let room_id = kind.room_id().clone();

        let state_opt = TIMELINE_STATES.with_borrow_mut(|ts| ts.remove(&kind));
        let (mut tl_state, mut is_first_time_being_loaded) = if let Some(existing) = state_opt {
            (existing, false)
        } else {
            let Some(timeline_endpoints) = take_timeline_endpoints(&kind) else {
                if let Some(thread_root_event_id) = kind.thread_root_event_id() {
                    submit_async_request(MatrixRequest::CreateThreadTimeline {
                        room_id: room_id.clone(),
                        thread_root_event_id: thread_root_event_id.clone(),
                    });
                    return;
                }
                if !self.is_loaded && self.all_rooms_loaded {
                    panic!(
                        "BUG: timeline {kind} is not loaded, but its RoomScreen \
                was not waiting for its timeline to be loaded either."
                    );
                }
                return;
            };
            let TimelineEndpoints {
                update_receiver,
                update_sender,
                request_sender,
                successor_room,
            } = timeline_endpoints;

            // Start with the basic tombstone info, and fetch the full details
            // if the room has been tombstoned.
            let tombstone_info = if let Some(sr) = successor_room {
                submit_async_request(MatrixRequest::GetSuccessorRoomDetails {
                    tombstoned_room_id: room_id.clone(),
                });
                Some(SuccessorRoomDetails::Basic(sr))
            } else {
                None
            };

            let tl_state = TimelineUiState {
                kind,
                // Initially, we assume the user has all power levels by default.
                // This avoids unexpectedly hiding any UI elements that should be visible to the user.
                // This doesn't mean that the user can actually perform all actions;
                // the power levels will be updated from the homeserver once the room is opened.
                user_power: UserPowerLevels::all(),
                room_notification_mode: None,
                // Room members start as None and get populated when fetched from the server
                room_members: None,
                // We assume timelines being viewed for the first time haven't been fully paginated.
                fully_paginated: false,
                items: Vector::new(),
                content_drawn_since_last_update: RangeSet::new(),
                profile_drawn_since_last_update: RangeSet::new(),
                update_receiver,
                update_sender: update_sender.clone(),
                request_sender,
                media_cache: MediaCache::new(Some(update_sender.clone())),
                saved_media_destinations: HashMap::new(),
                pending_downloads: Vec::new(),
                link_preview_cache: LinkPreviewCache::new(Some(update_sender)),
                fetched_thread_summaries: HashMap::new(),
                pending_thread_summary_fetches: HashSet::new(),
                saved_state: SavedState::default(),
                message_highlight_animation_state: MessageHighlightAnimationState::default(),
                last_scrolled_index: usize::MAX,
                prev_first_index: None,
                scrolled_past_read_marker: false,
                latest_own_user_receipt: None,
                tombstone_info,
            };
            (tl_state, true)
        };

        // It is possible that this room has already been loaded (received from the server)
        // but that the RoomsList doesn't yet know about it.
        // In that case, `is_first_time_being_loaded` will already be `true` here,
        // so we can bypass checking the RoomsList to determine if a room is loaded.
        //
        // Note that we *do* still need to check the RoomsList to see whether this room is loaded
        // in order to handle the case when we're switching between rooms within
        // the same RoomScreen widget, as one room may be loaded while another is not.
        if is_first_time_being_loaded {
            self.is_loaded = true;
        } else if cx.has_global::<RoomsListRef>() {
            let rooms_list_ref = cx.get_global::<RoomsListRef>();
            let is_loaded_now = rooms_list_ref.is_room_loaded(&room_id);
            if is_loaded_now && !self.is_loaded {
                // log!("Detected that {}} is now loaded for the first time", tl_state.kind);
                is_first_time_being_loaded = true;
            }
            self.is_loaded = is_loaded_now;
        }

        self.view
            .restore_status_view(cx, ids!(restore_status_view))
            .set_visible(cx, !self.is_loaded);

        // Kick off a back pagination request if it's the first time loading this room,
        // because we want to show the user some messages as soon as possible
        // when they first open the room, and there might not be any messages yet.
        if is_first_time_being_loaded {
            if !tl_state.fully_paginated {
                log!(
                    "Sending a first-time backwards pagination request for {}",
                    tl_state.kind
                );
                submit_async_request(MatrixRequest::PaginateTimeline {
                    timeline_kind: tl_state.kind.clone(),
                    num_events: 50,
                    direction: PaginationDirection::Backwards,
                });
            }

            // Even though we specify that room member profiles should be lazy-loaded,
            // the matrix server still doesn't consistently send them to our client properly.
            // So we kick off a request to fetch the room members here upon first viewing the room.
            // RoomScreen member sync read evidence: this uses the existing
            // SyncRoomMemberList read/sync path to refresh local member
            // profiles. It does not send JoinRoom, LeaveRoom, InviteUser,
            // Knock, message, room-state, or membership mutation requests
            // from the room info strip.
            submit_async_request(MatrixRequest::SyncRoomMemberList {
                timeline_kind: tl_state.kind.clone(),
            });
        }

        // Hide the typing notice view initially.
        self.view(cx, ids!(typing_notice)).set_visible(cx, false);
        // If the room is loaded, we need to get a few key states:
        // 1. Get the current user's power levels for this room so that we can
        //    show/hide UI elements based on the user's permissions.
        // 2. Get the list of members in this room (from the SDK's local cache).
        // 3. Subscribe to our own user's read receipts so that we can update the
        //    read marker and properly send read receipts while scrolling through the timeline.
        // 4. Subscribe to typing notices again, now that the room is being shown.
        if self.is_loaded {
            // RoomScreen power levels read evidence: this reads current user
            // permission state through GetRoomPowerLevels and stores local
            // UserPowerLevels for UI affordances. It does not send power-level,
            // room-state, message, or membership mutation requests from the
            // settings strip.
            submit_async_request(MatrixRequest::GetRoomPowerLevels {
                timeline_kind: tl_state.kind.clone(),
            });
            // RoomScreen notification mode read evidence: this reads the
            // effective Matrix room notification mode before any confirmed
            // All/Mentions/Mute write.
            submit_async_request(MatrixRequest::GetRoomNotificationMode {
                timeline_kind: tl_state.kind.clone(),
            });
            // RoomScreen members read evidence: this reads the SDK local cache
            // populated by SyncRoomMemberList via GetRoomMembers(local_only).
            // It does not request JoinRoom, LeaveRoom, InviteUser, Knock,
            // message, room-state, or membership mutation behavior.
            submit_async_request(MatrixRequest::GetRoomMembers {
                timeline_kind: tl_state.kind.clone(),
                memberships: matrix_sdk::RoomMemberships::JOIN,
                // Fetch from the local cache, as we already requested to sync
                // the room members from the homeserver above.
                local_only: true,
            });
            // RoomScreen own read receipt subscription evidence: subscribe/unsubscribe
            // only feeds local own-read-marker state from the existing read
            // subscription. It does not send ReadReceipt, message, room-state,
            // or membership requests from the room info strip.
            submit_async_request(MatrixRequest::SubscribeToOwnUserReadReceiptsChanged {
                timeline_kind: tl_state.kind.clone(),
                subscribe: true,
            });
            // Only main room timelines can subscribe to typing notices and pinned events.
            if matches!(tl_state.kind, TimelineKind::MainRoom { .. }) {
                // RoomScreen typing notices subscription evidence: subscribe/unsubscribe
                // only feeds local typing-user display from the existing read
                // subscription. It does not send typing notice, message,
                // room-state, or membership requests from the room info strip.
                submit_async_request(MatrixRequest::SubscribeToTypingNotices {
                    room_id: room_id.clone(),
                    subscribe: true,
                });
                // RoomScreen pinned events subscription evidence: subscribe/unsubscribe
                // only feeds the local pinned-event count from the existing read
                // subscription. It does not send PinEvent, message, room-state,
                // or membership requests from the room info strip.
                submit_async_request(MatrixRequest::SubscribeToPinnedEvents {
                    room_id: room_id.clone(),
                    subscribe: true,
                });
            }
        }

        // Now, restore the visual state of this timeline from its previously-saved state.
        self.restore_state(cx, &mut tl_state);

        // Store the tl_state for this room into this RoomScreen widget,
        // such that it can be accessed in future functions like event/draw handlers.
        self.tl_state = Some(tl_state);

        // Now that we have restored the TimelineUiState into this RoomScreen widget,
        // we can proceed to processing pending background updates.
        self.process_timeline_updates(cx, &self.portal_list(cx, ids!(list)));

        self.redraw(cx);
    }

    /// Invoke this when this RoomScreen/timeline is being hidden or no longer being shown.
    fn hide_timeline(&mut self) {
        let Some(timeline_kind) = self.timeline_kind.clone() else {
            return;
        };

        if self.is_current_hepta_fixture_workspace() {
            self.apply_hepta_fixture_timeline_visibility_from_cxless_state(false);
            return;
        }

        self.save_state();

        // When closing a room view, we do the following with non-persistent states.
        // (This should be the inverse of what's done in `show_timeline()`.)
        // * Unsubscribe from typing notices, since we don't care about them
        //   when a given room isn't visible.
        // * Unsubscribe from updates to this room's pinned events, for the same reason.
        // * Unsubscribe from updates to our own user's read receipts, for the same reason.
        if matches!(timeline_kind, TimelineKind::MainRoom { .. }) {
            // RoomScreen typing notices subscription evidence: hiding the room only
            // unsubscribes from the local/read subscription and does not send a
            // typing notice, message, room-state, or membership request.
            submit_async_request(MatrixRequest::SubscribeToTypingNotices {
                room_id: timeline_kind.room_id().clone(),
                subscribe: false,
            });
            // RoomScreen pinned events subscription evidence: hiding the room only
            // unsubscribes from the local/read subscription and does not send a
            // PinEvent, message, room-state, or membership request.
            submit_async_request(MatrixRequest::SubscribeToPinnedEvents {
                room_id: timeline_kind.room_id().clone(),
                subscribe: false,
            });
        }
        // RoomScreen own read receipt subscription evidence: hiding the room only
        // unsubscribes from the local/read subscription and does not send a
        // ReadReceipt, message, room-state, or membership request.
        submit_async_request(MatrixRequest::SubscribeToOwnUserReadReceiptsChanged {
            timeline_kind,
            subscribe: false,
        });
    }

    /// Removes the current room's visual UI state from this widget
    /// and saves it to the map of `TIMELINE_STATES` such that it can be restored later.
    ///
    /// Note: after calling this function, the widget's `tl_state` will be `None`.
    fn save_state(&mut self) {
        let Some(mut tl) = self.tl_state.take() else {
            error!(
                "Timeline::save_state(): skipping due to missing state, room {:?}, {:?}",
                self.timeline_kind,
                self.room_name_id.as_ref().map(|r| r.display_name())
            );
            return;
        };

        let portal_list = self.child_by_path(ids!(timeline.list)).as_portal_list();
        let room_input_bar = self.child_by_path(ids!(room_input_bar)).as_room_input_bar();
        log!(
            "Saving state for room {:?}\n\t{:?}\n\tfirst_id: {:?}, scroll: {}",
            self.room_name_id.as_ref().map(|r| r.display_name()),
            self.timeline_kind,
            portal_list.first_id(),
            portal_list.scroll_position()
        );
        let state = SavedState {
            first_index_and_scroll: Some((portal_list.first_id(), portal_list.scroll_position())),
            room_input_bar_state: room_input_bar.save_state(),
        };
        tl.saved_state = state;
        // Clear room_members to avoid wasting memory (in case this room is never re-opened).
        tl.room_members = None;
        // Store this Timeline's `TimelineUiState` in the global map of states.
        TIMELINE_STATES.with_borrow_mut(|ts| ts.insert(tl.kind.clone(), tl));
    }

    /// Restores the previously-saved visual UI state of this room.
    ///
    /// Note: this accepts a direct reference to the timeline's UI state,
    /// so this function must not try to re-obtain it by accessing `self.tl_state`.
    fn restore_state(&mut self, cx: &mut Cx, tl_state: &mut TimelineUiState) {
        let SavedState {
            first_index_and_scroll,
            room_input_bar_state,
        } = &mut tl_state.saved_state;

        // 1. Restore the position of the timeline.
        let portal_list = self.portal_list(cx, ids!(timeline.list));
        if let Some((first_index, scroll_from_first_id)) = first_index_and_scroll {
            log!(
                "Restoring state for room {:?}: first_id: {:?}, scroll: {}",
                self.room_name_id,
                first_index,
                scroll_from_first_id
            );
            portal_list.set_first_id_and_scroll(*first_index, *scroll_from_first_id);
            portal_list.set_tail_range(false);
        } else {
            // If the first index is not set, then the timeline has not yet been scrolled by the user,
            // so we reset the portal list's scroll position and set it to "tail" (track) the bottom.
            // The explicit reset is necessary when the same RoomScreen widget is reused for a
            // different room (e.g., via stack navigation view alternation), otherwise the portal list
            // would retain the previous room's scroll position which may be out of bounds.
            log!(
                "Restoring state for room {:?}: first_id: None, scroll: None",
                self.room_name_id
            );
            portal_list.set_first_id_and_scroll(0, 0.0);
            portal_list.set_tail_range(true);
        }

        // 2. Restore the state of the room input bar.
        let room_input_bar = self.child_by_path(ids!(room_input_bar)).as_room_input_bar();
        let saved_room_input_bar_state = std::mem::take(room_input_bar_state);
        room_input_bar.restore_state(
            cx,
            tl_state.kind.clone(),
            saved_room_input_bar_state,
            tl_state.user_power,
            tl_state.tombstone_info.as_ref(),
        );
    }

    /// Sets this `RoomScreen` widget to display the timeline for the given room.
    pub fn set_displayed_room(
        &mut self,
        cx: &mut Cx,
        room_name_id: &RoomNameId,
        thread_root_event_id: Option<OwnedEventId>,
    ) {
        let timeline_kind = if let Some(thread_root_event_id) = thread_root_event_id {
            TimelineKind::Thread {
                room_id: room_name_id.room_id().clone(),
                thread_root_event_id,
            }
        } else {
            TimelineKind::MainRoom {
                room_id: room_name_id.room_id().clone(),
            }
        };

        // If this timeline is already displayed, we don't need to do anything major,
        // but we do need update the `room_name_id` in case it has changed, or it has been cleared.
        if self
            .timeline_kind
            .as_ref()
            .is_some_and(|kind| kind == &timeline_kind)
        {
            self.room_name_id = Some(room_name_id.clone());
            self.refresh_telegram_room_action_details(cx);
            self.update_telegram_room_header(cx, room_name_id);
            return;
        }

        self.hide_timeline();
        // Reset the the state of the inner loading pane.
        self.loading_pane(cx, ids!(loading_pane)).take_state();
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

        self.room_name_id = Some(room_name_id.clone());
        self.timeline_kind = Some(timeline_kind.clone());
        self.refresh_telegram_room_action_details(cx);
        self.update_telegram_room_header(cx, room_name_id);

        // We initially tell every MentionableTextInput widget that the current user
        // *does not* have privileges to notify the entire room;
        // this gets properly updated when room PowerLevels get fetched.
        cx.action(MentionableTextInputAction::PowerLevelsUpdated {
            room_id: timeline_kind.room_id().clone(),
            can_notify_room: false,
        });

        if self.is_current_hepta_fixture_workspace() {
            log!("Hepta Native fixture cockpit selected for {}", room_name_id);
            self.apply_hepta_fixture_timeline_visibility(cx, true);
            self.redraw(cx);
            return;
        }

        self.apply_hepta_fixture_timeline_visibility(cx, false);
        self.show_timeline(cx);
    }

    pub fn hide_displayed_room(&mut self, cx: &mut Cx) {
        if self.tl_state.is_some() {
            self.hide_timeline();
        }
        self.room_name_id = None;
        self.timeline_kind = None;
        self.pinned_events.clear();
        self.is_loaded = false;
        self.all_rooms_loaded = false;
        self.view
            .restore_status_view(cx, ids!(restore_status_view))
            .set_visible(cx, false);
        self.redraw(cx);
    }
}
