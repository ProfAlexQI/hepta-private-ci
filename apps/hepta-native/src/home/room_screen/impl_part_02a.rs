impl RoomScreen {
    fn show_telegram_matrix_link_preview_retry_confirmation(&mut self, cx: &mut Cx) {
        let Some(timeline_kind) = self
            .telegram_matrix_link_preview_retry_timeline_kind
            .clone()
        else {
            enqueue_popup_notification(
                "Matrix link retry unavailable: no cached timeline kind for the failed preview.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let Some(room_or_alias_id) = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .clone()
        else {
            enqueue_popup_notification(
                "Matrix link retry unavailable: no cached Matrix room or alias target.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let via = self.telegram_matrix_link_preview_retry_via.clone();
        let event_id = self.telegram_matrix_link_preview_retry_event_id.clone();
        let via_count = via.len();
        let event_id_for_boundary = event_id.clone();
        let target = room_or_alias_id.to_string();
        let target_for_accept = target.clone();
        let target_for_cancel = target.clone();
        let room_or_alias_id_for_accept = room_or_alias_id.clone();
        let body_text =
            matrix_link_preview_retry_confirmation_label(&target, via.len(), event_id.as_ref());
        let content = ConfirmationModalContent {
            title_text: "Retry Matrix Link".into(),
            body_text: body_text.into(),
            accept_button_text: Some("Retry".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                submit_async_request(MatrixRequest::PreviewMatrixLinkTarget {
                    timeline_kind,
                    room_or_alias_id: room_or_alias_id_for_accept.clone(),
                    via,
                    event_id,
                });
                enqueue_popup_notification(
                    format!("Matrix link retry requested for {target_for_accept}."),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Matrix link retry canceled for {target_for_cancel}. {MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        self.telegram_matrix_link_preview_status = "retry".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Retry confirmation open for {target}. {MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_LABEL}"
        );
        self.telegram_matrix_link_preview_metadata =
        "Retry uses only cached TimelineKind, room-or-alias id, via list, and optional event id; no event context, join, knock, browser handoff, or mutation."
            .to_string();
        self.telegram_matrix_link_preview_target_label = target.clone();
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_via_count = via_count;
        self.telegram_matrix_link_preview_event_id_label = event_id_for_boundary
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Retry cache"));
        self.telegram_matrix_link_route_scope_metadata = matrix_link_route_scope_controls_label(
            Some("Retry cache"),
            "retry",
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            true,
        );
        self.telegram_matrix_link_context_actions_metadata = matrix_link_context_actions_row_label(
            Some("Retry cache"),
            "retry",
            via_count,
            event_id_for_boundary.as_ref(),
            true,
        );
        self.telegram_matrix_link_server_context_boundary =
            matrix_link_server_context_boundary_label(
                "retry confirmation",
                via_count,
                event_id_for_boundary.as_ref(),
                true,
            );
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            format!(
                "Matrix link retry confirmation opened for {target}. {MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_LABEL}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn copy_telegram_matrix_link_room_target(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some();
        let payload = matrix_link_room_target_clipboard_payload(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            retry_cache_ready,
        );
        let copied = payload.is_some();
        if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
        }
        let label = matrix_link_room_target_clipboard_label(
            copied,
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            retry_cache_ready,
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Room clipboard"));
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_preview_summary = if copied {
            format!(
                "Matrix link Room copied cached target metadata. {MATRIX_LINK_ROOM_TARGET_CLIPBOARD_LABEL}"
            )
        } else {
            format!(
                "Matrix link Room clipboard unavailable: cached target label is empty. {MATRIX_LINK_ROOM_TARGET_CLIPBOARD_LABEL}"
            )
        };
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        let popup_kind = if copied {
            PopupKind::Success
        } else {
            PopupKind::Warning
        };
        enqueue_popup_notification(label, popup_kind, Some(5.0));
    }

    fn copy_telegram_matrix_link_via_servers(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some();
        let payload = matrix_link_via_servers_clipboard_payload(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_via_label,
            &self.telegram_matrix_link_preview_event_id_label,
            retry_cache_ready,
        );
        let copied = payload.is_some();
        if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
        }
        let label = matrix_link_via_servers_clipboard_label(
            copied,
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_via_label,
            &self.telegram_matrix_link_preview_event_id_label,
            retry_cache_ready,
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Via clipboard"));
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_preview_summary = if copied {
            format!(
                "Matrix link Via copied cached server list. {MATRIX_LINK_VIA_SERVERS_CLIPBOARD_LABEL}"
            )
        } else {
            format!(
                "Matrix link Via clipboard unavailable: cached via server list is empty. {MATRIX_LINK_VIA_SERVERS_CLIPBOARD_LABEL}"
            )
        };
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        let popup_kind = if copied {
            PopupKind::Success
        } else {
            PopupKind::Warning
        };
        enqueue_popup_notification(label, popup_kind, Some(5.0));
    }

    fn copy_telegram_matrix_link_event_id(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some();
        let payload = matrix_link_event_id_clipboard_payload(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            retry_cache_ready,
        );
        let copied = payload.is_some();
        if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
        }
        let label = matrix_link_event_id_clipboard_label(
            copied,
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            retry_cache_ready,
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Event clipboard"));
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_preview_summary = if copied {
            format!(
                "Matrix link Event copied cached event id. {MATRIX_LINK_EVENT_ID_CLIPBOARD_LABEL}"
            )
        } else {
            format!(
                "Matrix link Event clipboard unavailable: cached event id is empty. {MATRIX_LINK_EVENT_ID_CLIPBOARD_LABEL}"
            )
        };
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        let popup_kind = if copied {
            PopupKind::Success
        } else {
            PopupKind::Warning
        };
        enqueue_popup_notification(label, popup_kind, Some(5.0));
    }

    fn copy_telegram_matrix_link_preview_metadata(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some();
        let payload = matrix_link_preview_metadata_clipboard_payload(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            &self.telegram_matrix_link_preview_metadata,
            retry_cache_ready,
        );
        let copied = payload.is_some();
        if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
        }
        let label = matrix_link_preview_metadata_clipboard_label(
            copied,
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_event_id_label,
            &self.telegram_matrix_link_preview_metadata,
            retry_cache_ready,
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Preview clipboard"));
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_preview_summary = if copied {
            format!(
                "Matrix link Preview copied cached local metadata. {MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_LABEL}"
            )
        } else {
            format!(
                "Matrix link Preview clipboard unavailable: cached preview metadata is empty. {MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_LABEL}"
            )
        };
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        let popup_kind = if copied {
            PopupKind::Success
        } else {
            PopupKind::Warning
        };
        enqueue_popup_notification(label, popup_kind, Some(5.0));
    }

    fn copy_telegram_matrix_link_route_drilldown_packet(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some();
        let event_id_label = self.telegram_matrix_link_preview_event_id_label.clone();
        let loaded_source_available = !event_id_label.trim().is_empty()
            && self.tl_state.as_ref().is_some_and(|tl| {
                tl.items.iter().any(|item| {
                    item.as_event()
                        .and_then(|event| event.event_id())
                        .is_some_and(|loaded_event_id| {
                            loaded_event_id.as_str() == event_id_label.trim()
                        })
                })
            });
        let payload = matrix_link_route_drilldown_packet_payload(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_via_label,
            &event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            self.telegram_matrix_link_preview_error_chars,
            retry_cache_ready,
            loaded_source_available,
        );
        cx.copy_to_clipboard(&payload);
        let label = matrix_link_route_drilldown_packet_label(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            retry_cache_ready,
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Route packet"));
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_context_actions_metadata = label.clone();
        self.telegram_matrix_link_server_context_boundary = label.clone();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Packet copied per-target route drilldown. {MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_LABEL}"
        );
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(label, PopupKind::Success, Some(5.0));
    }

    fn copy_telegram_matrix_link_route_result_contract_packet(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some();
        let event_id_label = self.telegram_matrix_link_preview_event_id_label.clone();
        let loaded_source_available = !event_id_label.trim().is_empty()
            && self.tl_state.as_ref().is_some_and(|tl| {
                tl.items.iter().any(|item| {
                    item.as_event()
                        .and_then(|event| event.event_id())
                        .is_some_and(|loaded_event_id| {
                            loaded_event_id.as_str() == event_id_label.trim()
                        })
                })
            });
        let payload = matrix_link_route_result_contract_packet_payload(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_via_label,
            &event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            self.telegram_matrix_link_preview_error_chars,
            retry_cache_ready,
            loaded_source_available,
        );
        cx.copy_to_clipboard(&payload);
        let label = matrix_link_route_result_contract_packet_label(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            retry_cache_ready,
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Route contract"));
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_context_actions_metadata = label.clone();
        self.telegram_matrix_link_server_context_boundary = label.clone();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Contract copied typed route/result contracts. {MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_LABEL}"
        );
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(label, PopupKind::Success, Some(5.0));
    }

    fn copy_telegram_matrix_link_route_result_taxonomy_packet(&mut self, cx: &mut Cx) {
        let retry_cache_ready = self
            .telegram_matrix_link_preview_retry_room_or_alias_id
            .is_some()
            && self
                .telegram_matrix_link_preview_retry_timeline_kind
                .is_some();
        let event_id_label = self.telegram_matrix_link_preview_event_id_label.clone();
        let loaded_source_available = !event_id_label.trim().is_empty()
            && self.tl_state.as_ref().is_some_and(|tl| {
                tl.items.iter().any(|item| {
                    item.as_event()
                        .and_then(|event| event.event_id())
                        .is_some_and(|loaded_event_id| {
                            loaded_event_id.as_str() == event_id_label.trim()
                        })
                })
            });
        let payload = matrix_link_route_result_taxonomy_packet_payload(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &self.telegram_matrix_link_preview_via_label,
            &event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            self.telegram_matrix_link_preview_error_chars,
            retry_cache_ready,
            loaded_source_available,
        );
        cx.copy_to_clipboard(&payload);
        let label = matrix_link_route_result_taxonomy_packet_label(
            &self.telegram_matrix_link_preview_status,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &event_id_label,
            self.telegram_matrix_link_preview_metadata.chars().count(),
            retry_cache_ready,
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Route taxonomy"));
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_context_actions_metadata = label.clone();
        self.telegram_matrix_link_server_context_boundary = label.clone();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Taxonomy copied route/event-context result slots. {MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_LABEL}"
        );
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(label, PopupKind::Success, Some(5.0));
    }

    fn open_telegram_matrix_link_loaded_event_source(&mut self, cx: &mut Cx, action: &str) {
        let action = action.trim();
        let action = if action.is_empty() {
            "Event source"
        } else {
            action
        };
        let event_id_label = self
            .telegram_matrix_link_preview_event_id_label
            .trim()
            .to_string();
        let loaded_source_detail = self.tl_state.as_ref().and_then(|tl| {
            if event_id_label.is_empty() {
                return None;
            }
            tl.items.iter().enumerate().find_map(|(index, item)| {
                let event_tl_item = item.as_event()?;
                let loaded_event_id = event_tl_item.event_id()?;
                (loaded_event_id.as_str() == event_id_label).then(|| {
                    let latest_json = event_tl_item
                        .latest_json()
                        .and_then(|raw_event| serde_json::to_value(raw_event).ok())
                        .and_then(|value| serde_json::to_string_pretty(&value).ok());
                    (
                        index,
                        tl.kind.room_id().clone(),
                        loaded_event_id.to_owned(),
                        latest_json,
                    )
                })
            })
        });
        let fetched_source_detail = if self
            .telegram_matrix_link_preview_source_json
            .trim()
            .is_empty()
        {
            None
        } else {
            self.telegram_matrix_link_preview_source_room_id
                .clone()
                .zip(self.telegram_matrix_link_preview_source_event_id.clone())
                .map(|(room_id, event_id)| {
                    (
                        usize::MAX,
                        room_id,
                        event_id,
                        Some(self.telegram_matrix_link_preview_source_json.clone()),
                    )
                })
        };
        let source_detail = loaded_source_detail.or(fetched_source_detail);
        let loaded_index = source_detail
            .as_ref()
            .and_then(|(index, _, _, _)| (*index != usize::MAX).then_some(*index));
        let loaded_json = source_detail
            .as_ref()
            .and_then(|(_, _, _, latest_json)| latest_json.as_deref());
        if let Some((_, room_id, event_id, latest_json)) = source_detail.clone() {
            cx.action(super::event_source_modal::EventSourceModalAction::Open {
                room_id,
                event_id: Some(event_id),
                latest_json,
            });
        }
        let opened = source_detail.is_some();
        let label = matrix_link_loaded_event_source_modal_label(
            action,
            &self.telegram_matrix_link_preview_target_label,
            self.telegram_matrix_link_preview_via_count,
            &event_id_label,
            loaded_index,
            loaded_json,
            opened,
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some(action));
        self.telegram_matrix_link_context_actions_metadata = label.clone();
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_server_context_boundary = label.clone();
        self.telegram_matrix_link_preview_summary = if opened {
            format!(
                "Matrix link Source opened EventSourceModal from loaded or preview-fetched event JSON. {MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_LABEL}"
            )
        } else {
            format!(
                "Matrix link Source stayed local: loaded or preview-fetched event source unavailable. {MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_LABEL}"
            )
        };
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(label, PopupKind::Info, Some(5.0));
    }

    fn show_telegram_matrix_link_join_confirmation(&mut self, cx: &mut Cx) {
        let (room_or_alias_id, server_names) = match self
            .telegram_matrix_link_join_retry_room_or_alias_id
            .clone()
        {
            Some(room_or_alias_id) => (
                room_or_alias_id,
                self.telegram_matrix_link_join_retry_via_servers.clone(),
            ),
            None => {
                let room_or_alias_id = match matrix_link_room_or_alias_join_target(
                    &self.telegram_matrix_link_preview_target_label,
                    &self.telegram_matrix_link_preview_event_id_label,
                ) {
                    Ok(room_or_alias_id) => room_or_alias_id,
                    Err(reason) => {
                        let label = matrix_link_join_room_unavailable_label(
                            &self.telegram_matrix_link_preview_target_label,
                            &self.telegram_matrix_link_preview_event_id_label,
                            &reason,
                        );
                        self.refresh_telegram_matrix_link_unresolved_detail(Some(
                            "Join unavailable",
                        ));
                        self.telegram_matrix_link_context_actions_metadata = label.clone();
                        self.telegram_matrix_link_server_context_boundary = label.clone();
                        self.telegram_matrix_link_preview_summary = label.clone();
                        self.update_telegram_matrix_link_preview_strip(cx);
                        self.set_telegram_matrix_link_preview_visible(cx, true);
                        enqueue_popup_notification(label, PopupKind::Warning, Some(5.0));
                        return;
                    }
                };
                (
                    room_or_alias_id,
                    self.telegram_matrix_link_join_via_servers.clone(),
                )
            }
        };

        let target = room_or_alias_id.to_string();
        let status = if self.telegram_matrix_link_preview_status.trim().is_empty() {
            "preview"
        } else {
            self.telegram_matrix_link_preview_status.trim()
        };
        let via_count = server_names.len();
        let event_id_label = self.telegram_matrix_link_preview_event_id_label.clone();
        let body_text = matrix_link_join_room_confirmation_label(
            &room_or_alias_id,
            status,
            via_count,
            &event_id_label,
        );
        let submitted_room_or_alias_id = room_or_alias_id.clone();
        let submitted_server_names = server_names.clone();
        let submitted_target = target.clone();
        let submitted_event_id_label = event_id_label.clone();
        let canceled_target = target.clone();
        let content = ConfirmationModalContent {
            title_text: "Join Matrix Room".into(),
            body_text: body_text.clone().into(),
            accept_button_text: Some("Join".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                cx.action(MatrixLinkJoinAction::Submitted {
                    room_or_alias_id: submitted_room_or_alias_id.clone(),
                    target: submitted_target.clone(),
                    via_count,
                    event_id_label: submitted_event_id_label.clone(),
                });
                submit_async_request(MatrixRequest::JoinRoomByIdOrAlias {
                    room_or_alias_id: submitted_room_or_alias_id.clone(),
                    server_names: submitted_server_names.clone(),
                });
                enqueue_popup_notification(
                    format!(
                        "Matrix link Join requested for {submitted_target}. {MATRIX_LINK_JOIN_ROOM_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |cx| {
                cx.action(MatrixLinkJoinAction::Canceled {
                    target: canceled_target.clone(),
                });
                enqueue_popup_notification(
                    format!(
                        "Matrix link Join canceled for {canceled_target}. {MATRIX_LINK_JOIN_ROOM_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        self.telegram_matrix_link_preview_status = "join-confirm".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Join confirmation open for {target}. {MATRIX_LINK_JOIN_ROOM_CONFIRMATION_LABEL}"
        );
        self.telegram_matrix_link_preview_metadata = body_text.clone();
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Join confirmation"));
        self.telegram_matrix_link_context_actions_metadata = body_text.clone();
        self.telegram_matrix_link_route_scope_metadata = body_text.clone();
        self.telegram_matrix_link_server_context_boundary = body_text.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(body_text, PopupKind::Info, Some(5.0));
    }

    fn show_telegram_matrix_link_join_submitted(
        &mut self,
        cx: &mut Cx,
        room_or_alias_id: OwnedRoomOrAliasId,
        target: String,
        via_count: usize,
        event_id_label: String,
    ) {
        self.telegram_matrix_link_join_pending_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_join_retry_room_or_alias_id = None;
        self.telegram_matrix_link_join_retry_via_servers.clear();
        self.telegram_matrix_link_knock_pending_room_or_alias_id = None;
        self.telegram_matrix_link_knock_retry_room_or_alias_id = None;
        self.telegram_matrix_link_knock_retry_via_servers.clear();
        self.telegram_matrix_link_preview_status = "joining".to_string();
        self.telegram_matrix_link_preview_target_label = target.clone();
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_via_count = via_count;
        self.telegram_matrix_link_preview_event_id_label = event_id_label;
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Join submitted for {room_or_alias_id}. Waiting for MatrixLinkJoinResultAction."
        );
        self.telegram_matrix_link_preview_metadata = format!(
            "MatrixRequest::JoinRoomByIdOrAlias submitted for {room_or_alias_id}. {MATRIX_LINK_JOIN_ROOM_CONFIRMATION_LABEL}"
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Join submitted"));
        self.telegram_matrix_link_context_actions_metadata =
            self.telegram_matrix_link_preview_metadata.clone();
        self.telegram_matrix_link_route_scope_metadata =
            self.telegram_matrix_link_preview_metadata.clone();
        self.telegram_matrix_link_server_context_boundary =
            self.telegram_matrix_link_preview_metadata.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }

    fn show_telegram_matrix_link_join_canceled(&mut self, cx: &mut Cx, target: String) {
        self.telegram_matrix_link_join_pending_room_or_alias_id = None;
        self.telegram_matrix_link_preview_status = "join-canceled".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Join canceled for {target}. {MATRIX_LINK_JOIN_ROOM_CONFIRMATION_LABEL}"
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Join canceled"));
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }

    fn show_telegram_matrix_link_join_result(
        &mut self,
        cx: &mut Cx,
        room_or_alias_id: OwnedRoomOrAliasId,
        server_names: Vec<OwnedServerName>,
        room_id: Option<OwnedRoomId>,
        error: Option<String>,
    ) {
        let succeeded = error.is_none();
        let label = matrix_link_join_room_result_label(
            &room_or_alias_id,
            room_id.as_ref(),
            succeeded,
            error.as_deref(),
        );
        self.telegram_matrix_link_join_pending_room_or_alias_id = None;
        self.telegram_matrix_link_join_retry_room_or_alias_id = if succeeded {
            None
        } else {
            Some(room_or_alias_id.clone())
        };
        self.telegram_matrix_link_join_retry_via_servers = if succeeded {
            Vec::new()
        } else {
            server_names.clone()
        };
        self.telegram_matrix_link_preview_status = if succeeded {
            "joined".to_string()
        } else {
            "failed".to_string()
        };
        self.telegram_matrix_link_preview_target_label = room_or_alias_id.to_string();
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_via_count = server_names.len();
        self.telegram_matrix_link_preview_via_label = matrix_link_via_servers_label(&server_names);
        self.telegram_matrix_link_preview_summary = label.clone();
        self.telegram_matrix_link_preview_metadata = label.clone();
        self.telegram_matrix_link_preview_error_chars = error.as_ref().map(|error| error.len());
        self.refresh_telegram_matrix_link_unresolved_detail(Some(if succeeded {
            "Join result"
        } else {
            "Join failed"
        }));
        self.telegram_matrix_link_context_actions_metadata = label.clone();
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_server_context_boundary = label.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(
            label,
            if succeeded {
                PopupKind::Success
            } else {
                PopupKind::Error
            },
            Some(5.0),
        );
    }

    fn show_telegram_matrix_link_knock_confirmation(&mut self, cx: &mut Cx) {
        let (room_or_alias_id, server_names) = match self
            .telegram_matrix_link_knock_retry_room_or_alias_id
            .clone()
        {
            Some(room_or_alias_id) => (
                room_or_alias_id,
                self.telegram_matrix_link_knock_retry_via_servers.clone(),
            ),
            None => {
                let room_or_alias_id = match matrix_link_room_or_alias_join_target(
                    &self.telegram_matrix_link_preview_target_label,
                    &self.telegram_matrix_link_preview_event_id_label,
                ) {
                    Ok(room_or_alias_id) => room_or_alias_id,
                    Err(reason) => {
                        let label = matrix_link_knock_room_unavailable_label(
                            &self.telegram_matrix_link_preview_target_label,
                            &self.telegram_matrix_link_preview_event_id_label,
                            &reason,
                        );
                        self.refresh_telegram_matrix_link_unresolved_detail(Some(
                            "Knock unavailable",
                        ));
                        self.telegram_matrix_link_context_actions_metadata = label.clone();
                        self.telegram_matrix_link_server_context_boundary = label.clone();
                        self.telegram_matrix_link_preview_summary = label.clone();
                        self.update_telegram_matrix_link_preview_strip(cx);
                        self.set_telegram_matrix_link_preview_visible(cx, true);
                        enqueue_popup_notification(label, PopupKind::Warning, Some(5.0));
                        return;
                    }
                };
                (
                    room_or_alias_id,
                    self.telegram_matrix_link_knock_via_servers.clone(),
                )
            }
        };

        let target = room_or_alias_id.to_string();
        let status = if self.telegram_matrix_link_preview_status.trim().is_empty() {
            "preview"
        } else {
            self.telegram_matrix_link_preview_status.trim()
        };
        let via_count = server_names.len();
        let event_id_label = self.telegram_matrix_link_preview_event_id_label.clone();
        let body_text = matrix_link_knock_room_confirmation_label(
            &room_or_alias_id,
            status,
            via_count,
            &event_id_label,
        );
        let submitted_room_or_alias_id = room_or_alias_id.clone();
        let submitted_server_names = server_names.clone();
        let submitted_target = target.clone();
        let submitted_event_id_label = event_id_label.clone();
        let canceled_target = target.clone();
        let content = ConfirmationModalContent {
            title_text: "Knock On Matrix Room".into(),
            body_text: body_text.clone().into(),
            accept_button_text: Some("Knock".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                cx.action(MatrixLinkKnockAction::Submitted {
                    room_or_alias_id: submitted_room_or_alias_id.clone(),
                    target: submitted_target.clone(),
                    via_count,
                    event_id_label: submitted_event_id_label.clone(),
                });
                submit_async_request(MatrixRequest::Knock {
                    room_or_alias_id: submitted_room_or_alias_id.clone(),
                    reason: None,
                    server_names: submitted_server_names.clone(),
                });
                enqueue_popup_notification(
                    format!(
                        "Matrix link Knock requested for {submitted_target}. {MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |cx| {
                cx.action(MatrixLinkKnockAction::Canceled {
                    target: canceled_target.clone(),
                });
                enqueue_popup_notification(
                    format!(
                        "Matrix link Knock canceled for {canceled_target}. {MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        self.telegram_matrix_link_preview_status = "knock-confirm".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Knock confirmation open for {target}. {MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_LABEL}"
        );
        self.telegram_matrix_link_preview_metadata = body_text.clone();
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Knock confirmation"));
        self.telegram_matrix_link_context_actions_metadata = body_text.clone();
        self.telegram_matrix_link_route_scope_metadata = body_text.clone();
        self.telegram_matrix_link_server_context_boundary = body_text.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(body_text, PopupKind::Info, Some(5.0));
    }

    fn show_telegram_matrix_link_knock_submitted(
        &mut self,
        cx: &mut Cx,
        room_or_alias_id: OwnedRoomOrAliasId,
        target: String,
        via_count: usize,
        event_id_label: String,
    ) {
        self.telegram_matrix_link_knock_pending_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_knock_retry_room_or_alias_id = None;
        self.telegram_matrix_link_knock_retry_via_servers.clear();
        self.telegram_matrix_link_join_pending_room_or_alias_id = None;
        self.telegram_matrix_link_join_retry_room_or_alias_id = None;
        self.telegram_matrix_link_join_retry_via_servers.clear();
        self.telegram_matrix_link_preview_status = "knocking".to_string();
        self.telegram_matrix_link_preview_target_label = target.clone();
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_via_count = via_count;
        self.telegram_matrix_link_preview_event_id_label = event_id_label;
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Knock submitted for {room_or_alias_id}. Waiting for KnockResultAction."
        );
        self.telegram_matrix_link_preview_metadata = format!(
            "MatrixRequest::Knock submitted for {room_or_alias_id}. {MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_LABEL}"
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Knock submitted"));
        self.telegram_matrix_link_context_actions_metadata =
            self.telegram_matrix_link_preview_metadata.clone();
        self.telegram_matrix_link_route_scope_metadata =
            self.telegram_matrix_link_preview_metadata.clone();
        self.telegram_matrix_link_server_context_boundary =
            self.telegram_matrix_link_preview_metadata.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }

    fn show_telegram_matrix_link_knock_canceled(&mut self, cx: &mut Cx, target: String) {
        self.telegram_matrix_link_knock_pending_room_or_alias_id = None;
        self.telegram_matrix_link_preview_status = "knock-canceled".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Knock canceled for {target}. {MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_LABEL}"
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Knock canceled"));
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }

    fn show_telegram_matrix_link_knock_result(
        &mut self,
        cx: &mut Cx,
        room_or_alias_id: OwnedRoomOrAliasId,
        server_names: Vec<OwnedServerName>,
        room_id: Option<OwnedRoomId>,
        error: Option<String>,
    ) {
        let succeeded = error.is_none();
        let label = matrix_link_knock_room_result_label(
            &room_or_alias_id,
            room_id.as_ref(),
            succeeded,
            error.as_deref(),
        );
        self.telegram_matrix_link_knock_pending_room_or_alias_id = None;
        self.telegram_matrix_link_knock_retry_room_or_alias_id = if succeeded {
            None
        } else {
            Some(room_or_alias_id.clone())
        };
        self.telegram_matrix_link_knock_retry_via_servers = if succeeded {
            Vec::new()
        } else {
            server_names.clone()
        };
        self.telegram_matrix_link_knock_via_servers = server_names.clone();
        self.telegram_matrix_link_preview_status = if succeeded {
            "knocked".to_string()
        } else {
            "failed".to_string()
        };
        self.telegram_matrix_link_preview_target_label = room_or_alias_id.to_string();
        self.telegram_matrix_link_preview_room_or_alias_id = Some(room_or_alias_id.clone());
        self.telegram_matrix_link_preview_via_count = server_names.len();
        self.telegram_matrix_link_preview_via_label = matrix_link_via_servers_label(&server_names);
        self.telegram_matrix_link_preview_summary = label.clone();
        self.telegram_matrix_link_preview_metadata = label.clone();
        self.telegram_matrix_link_preview_error_chars = error.as_ref().map(|error| error.len());
        self.refresh_telegram_matrix_link_unresolved_detail(Some(if succeeded {
            "Knock result"
        } else {
            "Knock failed"
        }));
        self.telegram_matrix_link_context_actions_metadata = label.clone();
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_server_context_boundary = label.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(
            label,
            if succeeded {
                PopupKind::Success
            } else {
                PopupKind::Error
            },
            Some(5.0),
        );
    }

    fn show_telegram_matrix_link_invite_confirmation(&mut self, cx: &mut Cx) {
        let (room_id, user_id) = match (
            self.telegram_matrix_link_invite_retry_room_id.clone(),
            self.telegram_matrix_link_invite_retry_user_id.clone(),
        ) {
            (Some(room_id), Some(user_id)) => (room_id, user_id),
            _ => {
                let Some(room_id) = self
                    .room_name_id
                    .as_ref()
                    .map(|room| room.room_id().clone())
                else {
                    let label = matrix_link_invite_user_unavailable_label(
                        &self.telegram_matrix_link_preview_target_label,
                        "no loaded current room is available for InviteUser",
                    );
                    self.refresh_telegram_matrix_link_unresolved_detail(Some("Invite unavailable"));
                    self.telegram_matrix_link_context_actions_metadata = label.clone();
                    self.telegram_matrix_link_server_context_boundary = label.clone();
                    self.telegram_matrix_link_preview_summary = label.clone();
                    self.update_telegram_matrix_link_preview_strip(cx);
                    self.set_telegram_matrix_link_preview_visible(cx, true);
                    enqueue_popup_notification(label, PopupKind::Warning, Some(5.0));
                    return;
                };
                let user_id = match matrix_link_user_invite_target(
                    &self.telegram_matrix_link_preview_target_label,
                ) {
                    Ok(user_id) => user_id,
                    Err(reason) => {
                        let label = matrix_link_invite_user_unavailable_label(
                            &self.telegram_matrix_link_preview_target_label,
                            &reason,
                        );
                        self.refresh_telegram_matrix_link_unresolved_detail(Some(
                            "Invite unavailable",
                        ));
                        self.telegram_matrix_link_context_actions_metadata = label.clone();
                        self.telegram_matrix_link_server_context_boundary = label.clone();
                        self.telegram_matrix_link_preview_summary = label.clone();
                        self.update_telegram_matrix_link_preview_strip(cx);
                        self.set_telegram_matrix_link_preview_visible(cx, true);
                        enqueue_popup_notification(label, PopupKind::Warning, Some(5.0));
                        return;
                    }
                };
                (room_id, user_id)
            }
        };

        let status = if self.telegram_matrix_link_preview_status.trim().is_empty() {
            "preview"
        } else {
            self.telegram_matrix_link_preview_status.trim()
        };
        let via_count = self.telegram_matrix_link_preview_via_count;
        let target = user_id.to_string();
        let body_text =
            matrix_link_invite_user_confirmation_label(&room_id, &user_id, status, via_count);
        let submitted_room_id = room_id.clone();
        let submitted_user_id = user_id.clone();
        let submitted_target = target.clone();
        let canceled_target = target.clone();
        let content = ConfirmationModalContent {
            title_text: "Invite Matrix User".into(),
            body_text: body_text.clone().into(),
            accept_button_text: Some("Invite".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                cx.action(MatrixLinkInviteAction::Submitted {
                    room_id: submitted_room_id.clone(),
                    user_id: submitted_user_id.clone(),
                    target: submitted_target.clone(),
                    via_count,
                });
                submit_async_request(MatrixRequest::InviteUser {
                    room_id: submitted_room_id.clone(),
                    user_id: submitted_user_id.clone(),
                });
                enqueue_popup_notification(
                    format!(
                        "Matrix link Invite requested for {submitted_target}. {MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |cx| {
                cx.action(MatrixLinkInviteAction::Canceled {
                    target: canceled_target.clone(),
                });
                enqueue_popup_notification(
                    format!(
                        "Matrix link Invite canceled for {canceled_target}. {MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        self.telegram_matrix_link_preview_status = "invite-confirm".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Invite confirmation open for {target}. {MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL}"
        );
        self.telegram_matrix_link_preview_metadata = body_text.clone();
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Invite confirmation"));
        self.telegram_matrix_link_context_actions_metadata = body_text.clone();
        self.telegram_matrix_link_route_scope_metadata = body_text.clone();
        self.telegram_matrix_link_server_context_boundary = body_text.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(body_text, PopupKind::Info, Some(5.0));
    }

    fn show_telegram_matrix_link_invite_submitted(
        &mut self,
        cx: &mut Cx,
        room_id: OwnedRoomId,
        user_id: OwnedUserId,
        target: String,
        via_count: usize,
    ) {
        self.telegram_matrix_link_invite_pending_room_id = Some(room_id.clone());
        self.telegram_matrix_link_invite_pending_user_id = Some(user_id.clone());
        self.telegram_matrix_link_invite_retry_room_id = None;
        self.telegram_matrix_link_invite_retry_user_id = None;
        self.telegram_matrix_link_join_pending_room_or_alias_id = None;
        self.telegram_matrix_link_join_retry_room_or_alias_id = None;
        self.telegram_matrix_link_join_retry_via_servers.clear();
        self.telegram_matrix_link_knock_pending_room_or_alias_id = None;
        self.telegram_matrix_link_knock_retry_room_or_alias_id = None;
        self.telegram_matrix_link_knock_retry_via_servers.clear();
        self.telegram_matrix_link_preview_status = "inviting".to_string();
        self.telegram_matrix_link_preview_target_label = target.clone();
        self.telegram_matrix_link_preview_room_or_alias_id = None;
        self.telegram_matrix_link_preview_via_count = via_count;
        self.telegram_matrix_link_preview_event_id_label.clear();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Invite submitted for {user_id} into {room_id}. Waiting for InviteResultAction."
        );
        self.telegram_matrix_link_preview_metadata = format!(
            "MatrixRequest::InviteUser submitted for {user_id} into {room_id}. {MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL}"
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Invite submitted"));
        self.telegram_matrix_link_context_actions_metadata =
            self.telegram_matrix_link_preview_metadata.clone();
        self.telegram_matrix_link_route_scope_metadata =
            self.telegram_matrix_link_preview_metadata.clone();
        self.telegram_matrix_link_server_context_boundary =
            self.telegram_matrix_link_preview_metadata.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }

    fn show_telegram_matrix_link_invite_canceled(&mut self, cx: &mut Cx, target: String) {
        self.telegram_matrix_link_invite_pending_room_id = None;
        self.telegram_matrix_link_invite_pending_user_id = None;
        self.telegram_matrix_link_preview_status = "invite-canceled".to_string();
        self.telegram_matrix_link_preview_summary = format!(
            "Matrix link Invite canceled for {target}. {MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL}"
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Invite canceled"));
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
    }

    fn show_telegram_matrix_link_invite_result(
        &mut self,
        cx: &mut Cx,
        room_id: OwnedRoomId,
        user_id: OwnedUserId,
        error: Option<String>,
    ) {
        let succeeded = error.is_none();
        let label =
            matrix_link_invite_user_result_label(&room_id, &user_id, succeeded, error.as_deref());
        self.telegram_matrix_link_invite_pending_room_id = None;
        self.telegram_matrix_link_invite_pending_user_id = None;
        self.telegram_matrix_link_invite_retry_room_id = if succeeded {
            None
        } else {
            Some(room_id.clone())
        };
        self.telegram_matrix_link_invite_retry_user_id = if succeeded {
            None
        } else {
            Some(user_id.clone())
        };
        self.telegram_matrix_link_preview_status = if succeeded {
            "invited".to_string()
        } else {
            "failed".to_string()
        };
        self.telegram_matrix_link_preview_target_label = user_id.to_string();
        self.telegram_matrix_link_preview_room_or_alias_id = None;
        self.telegram_matrix_link_preview_summary = label.clone();
        self.telegram_matrix_link_preview_metadata = label.clone();
        self.telegram_matrix_link_preview_error_chars = error.as_ref().map(|error| error.len());
        self.refresh_telegram_matrix_link_unresolved_detail(Some(if succeeded {
            "Invite result"
        } else {
            "Invite failed"
        }));
        self.telegram_matrix_link_context_actions_metadata = label.clone();
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_server_context_boundary = label.clone();
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(
            label,
            if succeeded {
                PopupKind::Success
            } else {
                PopupKind::Error
            },
            Some(5.0),
        );
    }

    fn stage_telegram_matrix_link_context_action(&mut self, cx: &mut Cx, action: &str) {
        let action = action.trim();
        let action = if action.is_empty() { "Context" } else { action };
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
                .is_some();
        let via_count = self.telegram_matrix_link_preview_retry_via.len();
        let event_id = self.telegram_matrix_link_preview_retry_event_id.clone();
        let event_id_label = self.telegram_matrix_link_preview_event_id_label.clone();
        let loaded_source_available = !event_id_label.trim().is_empty()
            && self.tl_state.as_ref().is_some_and(|tl| {
                tl.items.iter().any(|item| {
                    item.as_event()
                        .and_then(|event| event.event_id())
                        .is_some_and(|loaded_event_id| {
                            loaded_event_id.as_str() == event_id_label.trim()
                        })
                })
            });
        let metadata = if matches!(action, "Server context" | "Event context") {
            matrix_link_server_context_packet_snapshot_label(
                action,
                status,
                &self.telegram_matrix_link_preview_target_label,
                self.telegram_matrix_link_preview_via_count,
                &self.telegram_matrix_link_preview_via_label,
                &event_id_label,
                self.telegram_matrix_link_preview_metadata.chars().count(),
                self.telegram_matrix_link_preview_error_chars,
                retry_cache_ready,
                loaded_source_available,
            )
        } else {
            matrix_link_context_actions_row_label(
                Some(action),
                status,
                via_count,
                event_id.as_ref(),
                retry_cache_ready,
            )
        };
        self.refresh_telegram_matrix_link_unresolved_detail(Some(action));
        let detail = self.telegram_matrix_link_unresolved_detail.clone();
        self.telegram_matrix_link_context_actions_metadata = metadata.clone();
        self.telegram_matrix_link_preview_summary = if matches!(
            action,
            "Server context" | "Event context"
        ) {
            format!(
                "Matrix link {action} rendered a local server-context packet snapshot. {MATRIX_LINK_CONTEXT_ACTIONS_ROW_LABEL}"
            )
        } else {
            format!(
                "Matrix link {action} context control stayed local. {MATRIX_LINK_CONTEXT_ACTIONS_ROW_LABEL}"
            )
        };
        self.telegram_matrix_link_server_context_boundary =
            if matches!(action, "Server context" | "Event context") {
                metadata.clone()
            } else {
                matrix_link_server_context_boundary_label(
                    &format!("{action} context action stayed local"),
                    via_count,
                    event_id.as_ref(),
                    retry_cache_ready,
                )
            };
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);
        enqueue_popup_notification(format!("{metadata} {detail}"), PopupKind::Info, Some(5.0));
    }

    fn refresh_telegram_matrix_link_server_context(&mut self, cx: &mut Cx) {
        let Some(room_or_alias_id) = self.telegram_matrix_link_preview_room_or_alias_id.clone()
        else {
            self.stage_telegram_matrix_link_context_action(cx, "Server context");
            return;
        };
        let via = if !self.telegram_matrix_link_preview_retry_via.is_empty() {
            self.telegram_matrix_link_preview_retry_via.clone()
        } else if !self.telegram_matrix_link_join_via_servers.is_empty() {
            self.telegram_matrix_link_join_via_servers.clone()
        } else if !self.telegram_matrix_link_knock_via_servers.is_empty() {
            self.telegram_matrix_link_knock_via_servers.clone()
        } else {
            Vec::new()
        };
        let event_id = self
            .telegram_matrix_link_preview_event_id_label
            .trim()
            .to_string();
        let event_id = if event_id.is_empty() {
            None
        } else {
            EventId::parse(event_id).ok()
        };
        let target = room_or_alias_id.to_string();
        let current_status = if self.telegram_matrix_link_preview_status.trim().is_empty() {
            "preview"
        } else {
            self.telegram_matrix_link_preview_status.trim()
        };
        let event_state = event_id
            .as_ref()
            .map(|event_id| format!("event id {event_id} requested"))
            .unwrap_or_else(|| "no event id requested".to_string());
        let metadata = matrix_link_target_metadata_label(
            "server context",
            &target,
            via.len(),
            "cached RoomScreen target",
            current_status,
            &event_state,
            "standalone PreviewMatrixLinkTarget server-context refresh requested",
        );
        self.show_matrix_link_preview_request(
            cx,
            "Server context",
            target,
            room_or_alias_id,
            via,
            event_id,
            metadata,
        );
    }

    fn show_telegram_matrix_link_browser_confirmation(&mut self, cx: &mut Cx) {
        let via_label = if self
            .telegram_matrix_link_preview_via_label
            .trim()
            .is_empty()
            && !self.telegram_matrix_link_preview_retry_via.is_empty()
        {
            matrix_link_via_servers_label(&self.telegram_matrix_link_preview_retry_via)
        } else {
            self.telegram_matrix_link_preview_via_label.clone()
        };
        let url = matrix_link_browser_handoff_url(
            &self.telegram_matrix_link_preview_target_label,
            &self.telegram_matrix_link_preview_event_id_label,
            &via_label,
        );
        let via_count = if self.telegram_matrix_link_preview_via_count > 0 {
            self.telegram_matrix_link_preview_via_count
        } else {
            via_label
                .split(',')
                .map(str::trim)
                .filter(|via| !via.is_empty())
                .count()
        };
        let label = matrix_link_browser_handoff_confirmation_label(
            &self.telegram_matrix_link_preview_target_label,
            &self.telegram_matrix_link_preview_event_id_label,
            via_count,
            url.as_deref(),
        );
        self.refresh_telegram_matrix_link_unresolved_detail(Some("Browser"));
        self.telegram_matrix_link_context_actions_metadata = label.clone();
        self.telegram_matrix_link_route_scope_metadata = label.clone();
        self.telegram_matrix_link_server_context_boundary = label.clone();
        self.telegram_matrix_link_preview_summary = if url.is_some() {
            format!(
                "Matrix link Browser confirmation opened for cached matrix.to URL. {MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_LABEL}"
            )
        } else {
            format!(
                "Matrix link Browser handoff unavailable: cached target missing. {MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_LABEL}"
            )
        };
        self.update_telegram_matrix_link_preview_strip(cx);
        self.set_telegram_matrix_link_preview_visible(cx, true);

        if let Some(url) = url {
            enqueue_popup_notification(label.clone(), PopupKind::Info, Some(5.0));
            Self::show_external_link_confirmation(cx, url);
        } else {
            enqueue_popup_notification(label, PopupKind::Warning, Some(5.0));
        }
    }

    fn update_telegram_notifications_strip(&mut self, cx: &mut Cx, room_label: &str) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let current_status = if let Some(local_status) = local_status {
            format!("{local_status}; {mode_summary}")
        } else {
            mode_summary.clone()
        };
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let timed_global_boundary = notifications_timed_global_boundary_label(
            &mode_summary,
            &loaded_attention,
            local_status,
        );
        let requested_mode = self.telegram_notifications_requested_mode_label();
        let retry_cache_ready = self.telegram_notifications_any_retry_cache_ready();
        let keyword_retry_cache_ready = self.telegram_notifications_keyword_retry_cache_ready();
        let default_retry_cache_ready =
            self.telegram_notifications_default_mode_retry_cache_ready();
        let pusher_keyword_boundary = notifications_pusher_keyword_boundary_label(
            &mode_summary,
            &loaded_attention,
            retry_cache_ready,
            local_status,
        );
        let target_metadata = notifications_mode_target_metadata_label(
            room_label,
            &mode_summary,
            requested_mode,
            self.telegram_room_action_details.is_some(),
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        let result_detail = notifications_result_detail_control_label(
            room_label,
            Some(&self.telegram_notifications_result_detail_action),
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        let preflight_detail = notifications_preflight_detail_control_label(
            room_label,
            Some(&self.telegram_notifications_preflight_detail_action),
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        let retry_visible = self
            .telegram_notifications_local_status
            .starts_with("Update failed:")
            && self.telegram_notifications_room_mode_retry_cache_ready()
            || self
                .telegram_notifications_local_status
                .starts_with("Keyword update failed:")
                && keyword_retry_cache_ready
            || self
                .telegram_notifications_local_status
                .starts_with("Default update failed:")
                && default_retry_cache_ready;
        self.view
            .label(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notifications_header
                        .notifications_title
                ),
            )
            .set_text(cx, "Notifications");
        self.view
            .label(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notifications_header
                        .notifications_status
                ),
            )
            .set_text(cx, self.telegram_room_notification_mode_badge());
        self.view
            .button(
                cx,
                ids!(
                    telegram_notifications_strip
                        .notifications_header
                        .retry_notifications_button
                ),
            )
            .set_visible(cx, retry_visible);
        self.view
            .label(cx, ids!(telegram_notifications_strip.notifications_summary))
            .set_text(
                cx,
                &format!(
                "{room_label}: {current_status}. {loaded_attention}. {NOTIFICATIONS_COMPACT_LABEL}"
            ),
            );
        self.view
            .label(
                cx,
                ids!(telegram_notifications_strip.notifications_option_evidence),
            )
            .set_text(cx, NOTIFICATIONS_LOADED_ATTENTION_LABEL);
        self.view
            .label(
                cx,
                ids!(telegram_notifications_strip.notifications_mode_target_metadata),
            )
            .set_text(cx, &target_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_notifications_strip.notifications_result_detail),
            )
            .set_text(cx, &result_detail);
        self.view
            .label(
                cx,
                ids!(telegram_notifications_strip.notifications_preflight_detail),
            )
            .set_text(cx, &preflight_detail);
        self.view
            .label(
                cx,
                ids!(telegram_notifications_strip.notifications_timed_global_boundary),
            )
            .set_text(cx, &timed_global_boundary);
        self.view
            .label(
                cx,
                ids!(telegram_notifications_strip.notifications_pusher_keyword_boundary),
            )
            .set_text(cx, &pusher_keyword_boundary);
    }

    fn stage_telegram_notifications_advanced_control(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        control: &str,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let status = notifications_advanced_control_boundary_label(
            room_label,
            control,
            &mode_summary,
            &loaded_attention,
        );
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn stage_telegram_notifications_result_detail_control(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        control: &str,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let retry_cache_ready = self.telegram_notifications_any_retry_cache_ready();
        let requested_mode = self.telegram_notifications_requested_mode_label();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let status = notifications_result_detail_control_label(
            room_label,
            Some(control),
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        self.telegram_notifications_result_detail_action = control.trim().to_string();
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn stage_telegram_notifications_preflight_detail_control(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        control: &str,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let retry_cache_ready = self.telegram_notifications_any_retry_cache_ready();
        let requested_mode = self.telegram_notifications_requested_mode_label();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let status = if control.trim().eq_ignore_ascii_case("Schedule") {
            notifications_schedule_local_snapshot_label(
                room_label,
                &mode_summary,
                &loaded_attention,
                requested_mode,
                retry_cache_ready,
                self.tl_state.is_some(),
                local_status,
            )
        } else {
            notifications_preflight_detail_control_label(
                room_label,
                Some(control),
                &mode_summary,
                &loaded_attention,
                requested_mode,
                retry_cache_ready,
                self.tl_state.is_some(),
                local_status,
            )
        };
        self.telegram_notifications_preflight_detail_action = control.trim().to_string();
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn copy_telegram_notifications_rule_packet(&mut self, cx: &mut Cx, room_label: &str) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let retry_cache_ready = self.telegram_notifications_any_retry_cache_ready();
        let requested_mode = self.telegram_notifications_requested_mode_label();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let payload = notifications_rule_packet_payload(
            room_label,
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        cx.copy_to_clipboard(&payload);
        let status = notifications_rule_packet_clipboard_label(
            room_label,
            true,
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        self.telegram_notifications_preflight_detail_action = "Packet".to_string();
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn copy_telegram_notifications_rule_contract_packet(&mut self, cx: &mut Cx, room_label: &str) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let retry_cache_ready = self.telegram_notifications_any_retry_cache_ready();
        let requested_mode = self.telegram_notifications_requested_mode_label();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let payload = notifications_rule_contract_packet_payload(
            room_label,
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        cx.copy_to_clipboard(&payload);
        let status = notifications_rule_contract_packet_clipboard_label(
            room_label,
            true,
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        self.telegram_notifications_preflight_detail_action = "Contract".to_string();
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn copy_telegram_notifications_result_taxonomy_packet(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let retry_cache_ready = self.telegram_notifications_any_retry_cache_ready();
        let requested_mode = self.telegram_notifications_requested_mode_label();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let payload = notifications_result_taxonomy_packet_payload(
            room_label,
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        cx.copy_to_clipboard(&payload);
        let status = notifications_result_taxonomy_packet_clipboard_label(
            room_label,
            true,
            &mode_summary,
            &loaded_attention,
            requested_mode,
            retry_cache_ready,
            self.tl_state.is_some(),
            local_status,
        );
        self.telegram_notifications_preflight_detail_action = "Taxonomy".to_string();
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn stage_telegram_notifications_advanced_detail_control(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        control: &str,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let retry_cache_ready = self.telegram_notifications_any_retry_cache_ready();
        let status = notifications_advanced_detail_control_label(
            room_label,
            control,
            &mode_summary,
            &loaded_attention,
            retry_cache_ready,
        );
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn read_telegram_notification_keyword_rules(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        control: &str,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        if let Some(tl_state) = self.tl_state.as_ref() {
            submit_async_request(MatrixRequest::GetNotificationKeywordRules {
                timeline_kind: tl_state.kind.clone(),
            });
            let status = notifications_keyword_rules_live_read_requested_label(
                room_label,
                control,
                &mode_summary,
                &loaded_attention,
            );
            self.telegram_notifications_preflight_detail_action = control.trim().to_string();
            self.telegram_notifications_local_status = status.clone();
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
        } else {
            let status = notifications_keyword_rules_unavailable_label(
                room_label,
                control,
                &mode_summary,
                &loaded_attention,
            );
            self.telegram_notifications_preflight_detail_action = control.trim().to_string();
            self.telegram_notifications_local_status = status.clone();
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
        }
    }

    fn update_telegram_notification_keyword_rules_result(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        summary: &NotificationKeywordRulesSummary,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let status = notifications_keyword_rules_live_result_label(
            room_label,
            summary,
            &mode_summary,
            &loaded_attention,
            local_status,
        );
        self.telegram_notifications_preflight_detail_action = "Keywords".to_string();
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(5.0));
    }

    fn show_telegram_notification_keyword_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        keyword: &str,
        mutation: NotificationKeywordMutation,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let Some(keyword) = normalized_notification_keyword(keyword) else {
            let status = "Notification keyword write needs a non-empty keyword. No Matrix request was submitted."
            .to_string();
            self.telegram_notifications_local_status = status.clone();
            self.telegram_notifications_preflight_detail_action =
                notification_keyword_mutation_action_label(mutation).to_string();
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
            return;
        };
        let Some(timeline_kind) = self.tl_state.as_ref().map(|tl_state| tl_state.kind.clone())
        else {
            let status = notifications_keyword_mutation_unavailable_label(
                room_label,
                &keyword,
                mutation,
                &mode_summary,
                &loaded_attention,
            );
            self.telegram_notifications_local_status = status.clone();
            self.telegram_notifications_preflight_detail_action =
                notification_keyword_mutation_action_label(mutation).to_string();
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
            return;
        };

        let action = notification_keyword_mutation_action_label(mutation);
        self.telegram_notifications_retry_keyword = keyword.clone();
        self.telegram_notifications_retry_keyword_mutation = Some(mutation);
        self.telegram_notifications_preflight_detail_action = action.to_string();
        let room_label_for_accept = room_label.to_string();
        let room_label_for_cancel = room_label.to_string();
        let keyword_for_accept = keyword.clone();
        let keyword_for_cancel = keyword.clone();
        let action_for_accept = action.to_string();
        let action_for_cancel = action.to_string();
        let content = ConfirmationModalContent {
            title_text: "Confirm Keyword".into(),
            body_text: notifications_keyword_mutation_confirmation_label(
                room_label, &keyword, mutation,
            )
            .into(),
            accept_button_text: Some(action.into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                submit_async_request(MatrixRequest::SetNotificationKeywordRule {
                    timeline_kind,
                    keyword: keyword_for_accept.clone(),
                    mutation,
                });
                enqueue_popup_notification(
                    format!(
                        "Notification keyword {action_for_accept} requested for {room_label_for_accept}: {keyword_for_accept}."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Notification keyword {action_for_cancel} canceled for {room_label_for_cancel}: {keyword_for_cancel}. {NOTIFICATIONS_KEYWORD_MUTATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        let status = notifications_keyword_mutation_requested_label(
            room_label,
            &keyword,
            mutation,
            &mode_summary,
            &loaded_attention,
        );
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn show_telegram_notification_keyword_retry_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
    ) {
        let keyword = self.telegram_notifications_retry_keyword.trim().to_string();
        if keyword.is_empty() {
            enqueue_popup_notification(
                "Notification keyword retry unavailable: no cached keyword for the failed write.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        }
        let Some(mutation) = self.telegram_notifications_retry_keyword_mutation else {
            enqueue_popup_notification(
                "Notification keyword retry unavailable: no cached keyword operation for the failed write.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        self.show_telegram_notification_keyword_confirmation(cx, room_label, &keyword, mutation);
    }

    fn update_telegram_notification_keyword_mutation_result(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        keyword: String,
        mutation: NotificationKeywordMutation,
        result: Result<(), String>,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let status = notifications_keyword_mutation_result_label(
            room_label,
            &keyword,
            mutation,
            &result,
            &mode_summary,
            &loaded_attention,
        );
        let popup = notification_keyword_mutation_result_popup_message(
            room_label, &keyword, mutation, &result,
        );
        match result {
            Ok(()) => {
                self.telegram_notifications_local_status = status.clone();
                self.telegram_notifications_result_detail_action = "Result".to_string();
                self.telegram_notifications_retry_keyword.clear();
                self.telegram_notifications_retry_keyword_mutation = None;
                if let Some(tl_state) = self.tl_state.as_ref() {
                    submit_async_request(MatrixRequest::GetNotificationKeywordRules {
                        timeline_kind: tl_state.kind.clone(),
                    });
                }
                enqueue_popup_notification(popup, PopupKind::Success, Some(4.0));
            }
            Err(_) => {
                self.telegram_notifications_local_status = format!(
                    "Keyword update failed: {}",
                    notification_keyword_mutation_action_label(mutation)
                );
                self.telegram_notifications_result_detail_action = "Failure".to_string();
                self.telegram_notifications_retry_keyword = keyword;
                self.telegram_notifications_retry_keyword_mutation = Some(mutation);
                enqueue_popup_notification(popup, PopupKind::Error, Some(6.0));
            }
        }
        self.update_telegram_notifications_strip(cx, room_label);
    }

    fn read_telegram_notification_pusher_status(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        control: &str,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        if let Some(tl_state) = self.tl_state.as_ref() {
            submit_async_request(MatrixRequest::GetNotificationPusherStatus {
                timeline_kind: tl_state.kind.clone(),
            });
            let status = notifications_pusher_status_live_read_requested_label(
                room_label,
                control,
                &mode_summary,
                &loaded_attention,
            );
            self.telegram_notifications_preflight_detail_action = control.trim().to_string();
            self.telegram_notifications_local_status = status.clone();
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
        } else {
            let status = notifications_pusher_status_unavailable_label(
                room_label,
                control,
                &mode_summary,
                &loaded_attention,
            );
            self.telegram_notifications_preflight_detail_action = control.trim().to_string();
            self.telegram_notifications_local_status = status.clone();
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
        }
    }

    fn update_telegram_notification_pusher_status_result(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        summary: &NotificationPusherStatusSummary,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let status = notifications_pusher_status_live_result_label(
            room_label,
            summary,
            &mode_summary,
            &loaded_attention,
            local_status,
        );
        self.telegram_notifications_preflight_detail_action = "Pushers".to_string();
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(status, PopupKind::Info, Some(5.0));
    }

    fn read_telegram_notification_default_room_mode(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        control: &str,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        if let Some(tl_state) = self.tl_state.as_ref() {
            submit_async_request(MatrixRequest::GetDefaultRoomNotificationMode {
                timeline_kind: tl_state.kind.clone(),
            });
            let status = notifications_default_room_mode_live_read_requested_label(
                room_label,
                control,
                &mode_summary,
                &loaded_attention,
            );
            self.telegram_notifications_preflight_detail_action = control.trim().to_string();
            self.telegram_notifications_local_status = status.clone();
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
        } else {
            let status = notifications_default_room_mode_unavailable_label(
                room_label,
                control,
                &mode_summary,
                &loaded_attention,
            );
            self.telegram_notifications_preflight_detail_action = control.trim().to_string();
            self.telegram_notifications_local_status = status.clone();
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
        }
    }

    fn update_telegram_notification_default_room_mode_result(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        result: &Result<NotificationDefaultRoomModeSummary, String>,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let status = notifications_default_room_mode_live_result_label(
            room_label,
            result,
            &mode_summary,
            &loaded_attention,
            local_status,
        );
        self.telegram_notifications_preflight_detail_action = "Defaults".to_string();
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        let popup_kind = if result.is_ok() {
            PopupKind::Info
        } else {
            PopupKind::Warning
        };
        enqueue_popup_notification(status, popup_kind, Some(5.0));
    }

    fn show_telegram_notification_default_room_mode_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        mode: RoomNotificationMode,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let Some(timeline_kind) = self.tl_state.as_ref().map(|tl_state| tl_state.kind.clone())
        else {
            let status = notifications_default_room_mode_write_unavailable_label(
                room_label,
                mode,
                &mode_summary,
                &loaded_attention,
            );
            self.telegram_notifications_local_status = status.clone();
            self.telegram_notifications_preflight_detail_action =
                format!("Default {}", telegram_notification_mode_action_label(mode));
            self.update_telegram_notifications_strip(cx, room_label);
            self.set_telegram_notifications_visible(cx, true);
            enqueue_popup_notification(status, PopupKind::Warning, Some(4.0));
            return;
        };

        let mode_label = telegram_notification_mode_action_label(mode);
        self.telegram_notifications_retry_default_timeline_kind = Some(timeline_kind.clone());
        self.telegram_notifications_retry_default_mode = Some(mode);
        self.telegram_notifications_preflight_detail_action = format!("Default {mode_label}");
        let room_label_for_accept = room_label.to_string();
        let room_label_for_cancel = room_label.to_string();
        let mode_label_for_accept = mode_label.to_string();
        let mode_label_for_cancel = mode_label.to_string();
        let content = ConfirmationModalContent {
            title_text: "Confirm Defaults".into(),
            body_text: notifications_default_room_mode_write_confirmation_label(room_label, mode)
                .into(),
            accept_button_text: Some(mode_label.into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                submit_async_request(MatrixRequest::SetDefaultRoomNotificationMode {
                    timeline_kind,
                    mode,
                });
                enqueue_popup_notification(
                    format!(
                        "Default notification mode update requested for {room_label_for_accept}: {mode_label_for_accept}."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Default notification mode update canceled for {room_label_for_cancel}: {mode_label_for_cancel}. {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        let status = notifications_default_room_mode_write_requested_label(
            room_label,
            mode,
            &mode_summary,
            &loaded_attention,
        );
        self.telegram_notifications_local_status = status.clone();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(status, PopupKind::Info, Some(4.0));
    }

    fn show_telegram_notification_default_room_mode_retry_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
    ) {
        let Some(timeline_kind) = self
            .telegram_notifications_retry_default_timeline_kind
            .clone()
        else {
            enqueue_popup_notification(
                "Default notification retry unavailable: no cached timeline for the failed default mode write.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let Some(mode) = self.telegram_notifications_retry_default_mode else {
            enqueue_popup_notification(
                "Default notification retry unavailable: no cached default mode for the failed write.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let mode_label = telegram_notification_mode_action_label(mode);
        let room_label_for_accept = room_label.to_string();
        let room_label_for_cancel = room_label.to_string();
        let mode_label_for_accept = mode_label.to_string();
        let mode_label_for_cancel = mode_label.to_string();
        let content = ConfirmationModalContent {
            title_text: "Retry Defaults".into(),
            body_text: notifications_default_room_mode_retry_confirmation_label(room_label, mode)
                .into(),
            accept_button_text: Some("Retry".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                submit_async_request(MatrixRequest::SetDefaultRoomNotificationMode {
                    timeline_kind,
                    mode,
                });
                enqueue_popup_notification(
                    format!(
                        "Default notification mode retry requested for {room_label_for_accept}: {mode_label_for_accept}."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Default notification mode retry canceled for {room_label_for_cancel}: {mode_label_for_cancel}. {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        self.telegram_notifications_local_status =
            format!("Default retry confirmation open: {mode_label}");
        self.telegram_notifications_result_detail_action = "Retry cache".to_string();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            format!(
                "Default notification mode retry confirmation opened for {room_label}: {mode_label}. {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn update_telegram_notification_default_room_mode_mutation_result(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        timeline_kind: TimelineKind,
        mode: RoomNotificationMode,
        result: Result<NotificationDefaultRoomModeSummary, String>,
    ) {
        let mode_summary = self.telegram_room_notification_mode_summary();
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let status = notifications_default_room_mode_write_result_label(
            room_label,
            mode,
            &result,
            &mode_summary,
            &loaded_attention,
        );
        let popup =
            notification_default_room_mode_write_result_popup_message(room_label, mode, &result);
        let mode_label = telegram_notification_mode_action_label(mode);
        match result {
            Ok(_) => {
                self.telegram_notifications_local_status = status.clone();
                self.telegram_notifications_result_detail_action = "Result".to_string();
                self.telegram_notifications_preflight_detail_action =
                    format!("Default {mode_label}");
                self.telegram_notifications_retry_default_timeline_kind = None;
                self.telegram_notifications_retry_default_mode = None;
                enqueue_popup_notification(popup, PopupKind::Success, Some(5.0));
            }
            Err(_) => {
                self.telegram_notifications_local_status =
                    format!("Default update failed: {mode_label}");
                self.telegram_notifications_result_detail_action = "Failure".to_string();
                self.telegram_notifications_preflight_detail_action =
                    format!("Default {mode_label}");
                self.telegram_notifications_retry_default_timeline_kind = Some(timeline_kind);
                self.telegram_notifications_retry_default_mode = Some(mode);
                enqueue_popup_notification(popup, PopupKind::Error, Some(6.0));
            }
        }
        self.update_telegram_notifications_strip(cx, room_label);
    }

    fn copy_telegram_notifications_mode_summary(&mut self, cx: &mut Cx, room_label: &str) {
        let mode_label = self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_notification_mode)
            .map(telegram_notification_mode_action_label);
        let loaded_attention = self.telegram_room_notification_loaded_attention_summary();
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let payload = mode_label.and_then(|mode_label| {
            notifications_mode_clipboard_payload(
                room_label,
                mode_label,
                &loaded_attention,
                local_status,
            )
        });
        let copied = if let Some(payload) = payload.as_deref() {
            cx.copy_to_clipboard(payload);
            true
        } else {
            false
        };
        let status = notifications_mode_clipboard_label(
            copied,
            room_label,
            mode_label,
            &loaded_attention,
            self.tl_state.is_some(),
            local_status,
        );
        self.telegram_notifications_local_status = status.clone();
        self.telegram_notifications_result_detail_action = "Copy mode".to_string();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        enqueue_popup_notification(
            if copied {
                status
            } else {
                format!(
                    "Notification mode clipboard unavailable: loaded mode is missing. {NOTIFICATIONS_MODE_CLIPBOARD_LABEL}"
                )
            },
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn set_telegram_notifications_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_notifications_visible == visible {
            return;
        }
        self.telegram_notifications_visible = visible;
        self.view
            .view(cx, ids!(telegram_notifications_strip))
            .set_visible(cx, visible);
    }

    fn show_telegram_notifications_surface(&mut self, cx: &mut Cx, room_label: &str) {
        self.refresh_telegram_room_action_details(cx);
        // Notifications evidence: opening the strip reads mode before any confirmed write.
        if let Some(tl_state) = self.tl_state.as_ref() {
            submit_async_request(MatrixRequest::GetRoomNotificationMode {
                timeline_kind: tl_state.kind.clone(),
            });
        }
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_search_mode_visible(cx, false);
        self.set_telegram_message_edit_history_visible(cx, false);
        self.set_telegram_message_report_status_visible(cx, false);
        self.set_telegram_room_actions_visible(cx, false, None);
        self.set_telegram_room_info_visible(cx, false);
        self.set_telegram_room_settings_visible(cx, false);
        self.set_telegram_matrix_link_preview_visible(cx, false);
        self.set_telegram_notifications_visible(cx, true);
    }

    fn show_telegram_notification_mode_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        room_id: OwnedRoomId,
        mode: RoomNotificationMode,
    ) {
        let mode_label = telegram_notification_mode_action_label(mode);
        self.telegram_notifications_retry_room_id = Some(room_id.clone());
        self.telegram_notifications_retry_mode = Some(mode);
        let room_label_for_accept = room_label.to_string();
        let room_label_for_cancel = room_label.to_string();
        let mode_label_for_accept = mode_label.to_string();
        let mode_label_for_cancel = mode_label.to_string();
        let content = ConfirmationModalContent {
        title_text: "Confirm Notifications".into(),
        body_text: format!(
            "Set notification mode for {room_label} to {mode_label}? {NOTIFICATIONS_LOCAL_BOUNDARY_LABEL}"
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
                    "Notification mode update canceled for {room_label_for_cancel}: {mode_label_for_cancel}. {NOTIFICATIONS_LOCAL_BOUNDARY_LABEL}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
        })),
        ..Default::default()
    };
        self.telegram_notifications_local_status = format!("Confirmation open: {mode_label}");
        self.telegram_notifications_result_detail_action = "Requested".to_string();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            format!(
                "Notification mode confirmation opened for {room_label}: {mode_label}. {NOTIFICATIONS_LOCAL_BOUNDARY_LABEL}"
            ),
            PopupKind::Info,
            Some(3.0),
        );
    }

    fn show_telegram_notification_mode_retry_confirmation(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
    ) {
        let Some(room_id) = self.telegram_notifications_retry_room_id.clone() else {
            enqueue_popup_notification(
                "Notification retry unavailable: no cached room id for the failed mode write.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let Some(mode) = self.telegram_notifications_retry_mode else {
            enqueue_popup_notification(
                "Notification retry unavailable: no cached notification mode for the failed write.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let mode_label = telegram_notification_mode_action_label(mode);
        let room_label_for_accept = room_label.to_string();
        let room_label_for_cancel = room_label.to_string();
        let mode_label_for_accept = mode_label.to_string();
        let mode_label_for_cancel = mode_label.to_string();
        let content = ConfirmationModalContent {
            title_text: "Retry Notifications".into(),
            body_text: notifications_retry_confirmation_label(room_label, mode_label).into(),
            accept_button_text: Some("Retry".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                submit_async_request(MatrixRequest::SetRoomNotificationMode { room_id, mode });
                enqueue_popup_notification(
                    format!(
                        "Notification mode retry requested for {room_label_for_accept}: {mode_label_for_accept}."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Notification mode retry canceled for {room_label_for_cancel}: {mode_label_for_cancel}. {NOTIFICATIONS_RETRY_CONFIRMATION_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
            ..Default::default()
        };
        self.telegram_notifications_local_status = format!("Retry confirmation open: {mode_label}");
        self.telegram_notifications_result_detail_action = "Retry cache".to_string();
        self.update_telegram_notifications_strip(cx, room_label);
        self.set_telegram_notifications_visible(cx, true);
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        enqueue_popup_notification(
            format!(
                "Notification mode retry confirmation opened for {room_label}: {mode_label}. {NOTIFICATIONS_RETRY_CONFIRMATION_LABEL}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn telegram_room_notification_mode_summary(&self) -> String {
        self.tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_notification_mode)
            .map(|mode| match mode {
                RoomNotificationMode::AllMessages => {
                    "Current Matrix mode: all messages".to_string()
                }
                RoomNotificationMode::MentionsAndKeywordsOnly => {
                    "Current Matrix mode: mentions and keywords".to_string()
                }
                RoomNotificationMode::Mute => "Current Matrix mode: muted".to_string(),
            })
            .unwrap_or_else(|| "Current Matrix mode: loading read-only".to_string())
    }

    fn telegram_room_notification_mode_badge(&self) -> &'static str {
        match self
            .tl_state
            .as_ref()
            .and_then(|tl_state| tl_state.room_notification_mode)
        {
            Some(RoomNotificationMode::AllMessages) => "all",
            Some(RoomNotificationMode::MentionsAndKeywordsOnly) => "mentions",
            Some(RoomNotificationMode::Mute) => "muted",
            None => "loading",
        }
    }

    fn telegram_room_notification_loaded_attention_summary(&self) -> String {
        self.telegram_room_action_details
            .as_ref()
            .map(|details| {
                let manual_state = if details.is_marked_unread {
                    "manual unread"
                } else {
                    "not manually unread"
                };
                format!(
                    "Loaded attention: {} unread, {} mentions, {manual_state}",
                    details.num_unread_messages, details.num_unread_mentions
                )
            })
            .unwrap_or_else(|| "Loaded attention: waiting for room-list unread state".to_string())
    }

    fn telegram_notifications_room_mode_retry_cache_ready(&self) -> bool {
        self.telegram_notifications_retry_room_id.is_some()
            && self.telegram_notifications_retry_mode.is_some()
    }

    fn telegram_notifications_keyword_retry_cache_ready(&self) -> bool {
        self.telegram_notifications_retry_keyword_mutation.is_some()
            && !self.telegram_notifications_retry_keyword.trim().is_empty()
    }

    fn telegram_notifications_default_mode_retry_cache_ready(&self) -> bool {
        self.telegram_notifications_retry_default_timeline_kind
            .is_some()
            && self.telegram_notifications_retry_default_mode.is_some()
    }

    fn telegram_notifications_any_retry_cache_ready(&self) -> bool {
        self.telegram_notifications_room_mode_retry_cache_ready()
            || self.telegram_notifications_keyword_retry_cache_ready()
            || self.telegram_notifications_default_mode_retry_cache_ready()
    }

    fn telegram_notifications_requested_mode_label(&self) -> Option<&'static str> {
        self.telegram_notifications_retry_mode
            .or(self.telegram_notifications_retry_default_mode)
            .map(telegram_notification_mode_action_label)
    }

    fn telegram_notifications_close_refresh_metadata_summary(
        &self,
        room_label: &str,
        action: &str,
    ) -> String {
        let local_status = self.telegram_notifications_local_status.trim();
        let local_status = (!local_status.is_empty()).then_some(local_status);
        let attention_loaded = self.telegram_room_action_details.is_some();
        notifications_close_refresh_metadata_label(
            room_label,
            action,
            &self.telegram_room_notification_mode_summary(),
            local_status,
            attention_loaded,
            self.tl_state.is_some(),
        )
    }

    fn update_telegram_notification_mode_result(
        &mut self,
        cx: &mut Cx,
        room_label: &str,
        mode: RoomNotificationMode,
        result: Result<(), String>,
    ) {
        let mode_label = telegram_notification_mode_action_label(mode);
        match result {
            Ok(()) => {
                self.telegram_notifications_local_status = format!("Updated: {mode_label}");
                self.telegram_notifications_result_detail_action = "Result".to_string();
                self.telegram_notifications_retry_room_id = None;
                self.telegram_notifications_retry_mode = None;
                enqueue_popup_notification(
                    notification_mode_write_result_popup_message(room_label, mode, &Ok(())),
                    PopupKind::Success,
                    Some(4.0),
                );
            }
            Err(error) => {
                self.telegram_notifications_local_status = format!("Update failed: {mode_label}");
                self.telegram_notifications_result_detail_action = "Failure".to_string();
                if self.telegram_notifications_retry_room_id.is_none() {
                    self.telegram_notifications_retry_room_id = self
                        .room_name_id
                        .as_ref()
                        .map(|room_name_id| room_name_id.room_id().clone());
                }
                self.telegram_notifications_retry_mode = Some(mode);
                enqueue_popup_notification(
                    notification_mode_write_result_popup_message(room_label, mode, &Err(error)),
                    PopupKind::Error,
                    Some(6.0),
                );
            }
        }
        self.update_telegram_notifications_strip(cx, room_label);
    }

    fn reset_telegram_message_search_state(&mut self, cx: &mut Cx) {
        self.telegram_message_search_query.clear();
        self.telegram_message_search_matches.clear();
        self.telegram_message_search_active_match = 0;
        self.telegram_message_search_loaded_scope = MessageSearchLoadedScope::AllLoaded;
        self.telegram_message_search_server_pending = false;
        self.telegram_message_search_server_last_query.clear();
        self.telegram_message_search_server_next_batch = None;
        self.telegram_message_search_server_room_id.clear();
        self.telegram_message_search_server_hits.clear();
        self.telegram_message_search_server_context_target_event_id = None;
        self.telegram_message_search_server_last_error.clear();
        self.view
            .text_input(
                cx,
                ids!(telegram_message_search_strip.search_row.search_input),
            )
            .set_text(cx, "");
        self.update_telegram_message_search_labels(cx);
    }

    fn telegram_message_search_latest_loaded_day_floor(&self) -> Option<ruma::UInt> {
        let newest = self
            .tl_state
            .as_ref()?
            .items
            .iter()
            .filter_map(|item| {
                item.as_event()
                    .map(|event_tl_item| event_tl_item.timestamp().0)
            })
            .max()?;
        Some(newest.checked_sub(uint!(86_400_000)).unwrap_or(uint!(0)))
    }

    fn telegram_message_search_event_matches_loaded_scope(
        &self,
        event_tl_item: &EventTimelineItem,
        latest_loaded_day_floor: Option<ruma::UInt>,
    ) -> bool {
        match self.telegram_message_search_loaded_scope {
            MessageSearchLoadedScope::AllLoaded => true,
            MessageSearchLoadedScope::LatestLoadedDay => latest_loaded_day_floor
                .map(|floor| event_tl_item.timestamp().0 >= floor)
                .unwrap_or(false),
            MessageSearchLoadedScope::PinnedLoaded => {
                event_tl_item.event_id().is_some_and(|event_id| {
                    self.pinned_events
                        .iter()
                        .any(|pinned_event_id| pinned_event_id == event_id)
                })
            }
        }
    }

    fn telegram_message_search_matches_for_query(&self, query: &str) -> Vec<usize> {
        let needle = query.trim().to_lowercase();
        if needle.is_empty() {
            return Vec::new();
        }

        let Some(tl_state) = self.tl_state.as_ref() else {
            return Vec::new();
        };

        let latest_loaded_day_floor = self.telegram_message_search_latest_loaded_day_floor();
        tl_state
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                item.as_event().and_then(|event_tl_item| {
                    if !self.telegram_message_search_event_matches_loaded_scope(
                        event_tl_item,
                        latest_loaded_day_floor,
                    ) {
                        return None;
                    }
                    plaintext_body_of_timeline_item(event_tl_item)
                        .to_lowercase()
                        .contains(&needle)
                        .then_some(index)
                })
            })
            .collect()
    }

    fn refresh_telegram_message_search_matches(&mut self, cx: &mut Cx) {
        let query = self.telegram_message_search_query.clone();
        self.telegram_message_search_matches =
            self.telegram_message_search_matches_for_query(&query);
        if self.telegram_message_search_active_match >= self.telegram_message_search_matches.len() {
            self.telegram_message_search_active_match = 0;
        }
        self.update_telegram_message_search_labels(cx);
    }

    fn set_telegram_message_search_query(&mut self, cx: &mut Cx, query: String) {
        self.telegram_message_search_query = query;
        self.telegram_message_search_active_match = 0;
        self.telegram_message_search_server_next_batch = None;
        self.telegram_message_search_server_room_id.clear();
        self.telegram_message_search_server_hits.clear();
        self.telegram_message_search_server_context_target_event_id = None;
        self.telegram_message_search_server_last_error.clear();
        self.refresh_telegram_message_search_matches(cx);
    }

    fn update_telegram_message_search_labels(&mut self, cx: &mut Cx) {
        let query = self.telegram_message_search_query.trim();
        let match_count = self.telegram_message_search_matches.len();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let status_text = if query.is_empty() {
            "local only".to_string()
        } else if self.tl_state.is_none() {
            "timeline loading".to_string()
        } else if match_count == 0 {
            "0 local matches".to_string()
        } else {
            format!(
                "{} / {} local",
                self.telegram_message_search_active_match + 1,
                match_count
            )
        };
        self.view
            .label(cx, ids!(telegram_message_search_strip.search_row.status))
            .set_text(cx, &status_text);

        let result_text = if query.is_empty() {
            MESSAGE_SEARCH_COMPACT_LABEL.to_string()
        } else if self.tl_state.is_none() {
            "Timeline is still loading; search stays local.".to_string()
        } else if match_count == 0 {
            format!("No loaded local messages match \"{query}\".")
        } else {
            let preview = self
                .telegram_message_search_active_match_preview()
                .unwrap_or_else(|| "preview unavailable".to_string());
            format!(
                "Local match {} of {} for \"{}\". Preview: {}",
                self.telegram_message_search_active_match + 1,
                match_count,
                query,
                preview
            )
        };
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.results_row.result_summary),
            )
            .set_text(cx, &result_text);
        self.view
            .label(cx, ids!(telegram_message_search_strip.search_evidence))
            .set_text(
                cx,
                &loaded_message_search_query_lifecycle_label(
                    query,
                    loaded_item_count,
                    match_count,
                    self.telegram_message_search_active_match,
                    self.telegram_message_search_visible,
                    self.tl_state.is_some(),
                ),
            );
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
        let metadata_text = loaded_message_search_metadata_label(
            &query,
            loaded_item_count,
            match_count,
            self.telegram_message_search_active_match,
            active_loaded_index,
            active_event_id_loaded,
        );
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.loaded_search_metadata),
            )
            .set_text(cx, &metadata_text);
        let active_detail_text = loaded_message_search_active_result_label(
            query,
            match_count,
            self.telegram_message_search_active_match,
            active_loaded_index,
            active_match_detail
                .as_ref()
                .and_then(|(event_id, _)| event_id.as_deref()),
            active_match_detail.as_ref().map(|(_, body)| body.as_str()),
        );
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.active_search_result_detail),
            )
            .set_text(cx, &active_detail_text);
        self.telegram_message_search_result_action_controls_metadata =
            loaded_message_search_result_action_controls_label(
                None,
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                active_loaded_index,
                active_event_id_loaded,
                active_match_detail.as_ref().map(|(_, body)| body.as_str()),
            );
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_result_action_controls_metadata),
            )
            .set_text(
                cx,
                &self.telegram_message_search_result_action_controls_metadata,
            );
        self.telegram_message_search_server_context_controls_metadata =
            loaded_message_search_server_context_controls_label(
                None,
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                self.tl_state.is_some(),
                self.telegram_message_search_server_next_batch.is_some(),
            );
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_server_context_controls_metadata),
            )
            .set_text(
                cx,
                &self.telegram_message_search_server_context_controls_metadata,
            );
        self.telegram_message_search_advanced_filter_controls_metadata =
            loaded_message_search_advanced_filter_controls_label(
                None,
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                self.tl_state.is_some(),
                self.pinned_events.len(),
                self.telegram_message_search_loaded_scope,
            );
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_advanced_filter_controls_metadata),
            )
            .set_text(
                cx,
                &self.telegram_message_search_advanced_filter_controls_metadata,
            );
        self.telegram_message_search_server_preflight_controls_metadata =
            loaded_message_search_server_preflight_controls_label(
                None,
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                self.tl_state.is_some(),
                self.pinned_events.len(),
                &self.telegram_message_search_server_context_controls_metadata,
            );
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_server_preflight_controls_metadata),
            )
            .set_text(
                cx,
                &self.telegram_message_search_server_preflight_controls_metadata,
            );
        let server_context_boundary_text = loaded_message_search_server_context_boundary_label(
            query,
            loaded_item_count,
            match_count,
            self.tl_state.is_some(),
            self.telegram_message_search_server_next_batch.is_some(),
        );
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.server_context_boundary),
            )
            .set_text(cx, &server_context_boundary_text);
    }

    fn cached_telegram_message_search_server_context_event_id(
        &self,
    ) -> Result<OwnedEventId, String> {
        if self.telegram_message_search_server_pending {
            return Err("server search still pending".to_string());
        }
        if self.telegram_message_search_server_hits.is_empty() {
            return Err("no cached server search hits".to_string());
        }
        let Some(current_room_id) = self.timeline_kind.as_ref().map(TimelineKind::room_id) else {
            return Err("timeline waiting".to_string());
        };
        if self
            .telegram_message_search_server_room_id
            .trim()
            .is_empty()
        {
            return Err("server result room id missing".to_string());
        }
        if current_room_id.as_str() != self.telegram_message_search_server_room_id.trim() {
            return Err("server result belongs to a different room".to_string());
        }

        self.telegram_message_search_server_hits
            .iter()
            .filter_map(|hit| hit.event_id.as_deref())
            .find_map(|event_id| EventId::parse(event_id).ok())
            .ok_or_else(|| "cached server hits have no valid event id".to_string())
    }

    fn loaded_event_index_for_event_id(&self, event_id: &EventId) -> Option<usize> {
        self.tl_state.as_ref()?.items.iter().position(|item| {
            item.as_event()
                .and_then(|event| event.event_id())
                .is_some_and(|loaded_event_id| loaded_event_id == event_id)
        })
    }

    fn submit_telegram_message_search_server_context_event(
        &mut self,
        cx: &mut Cx,
        portal_list: &PortalListRef,
    ) {
        self.set_telegram_search_mode_visible(cx, true);
        let query = if self
            .telegram_message_search_server_last_query
            .trim()
            .is_empty()
        {
            self.telegram_message_search_query.trim()
        } else {
            self.telegram_message_search_server_last_query.trim()
        };
        let hit_count = self.telegram_message_search_server_hits.len();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let target_event_id = match self.cached_telegram_message_search_server_context_event_id() {
            Ok(event_id) => event_id,
            Err(reason) => {
                let metadata = message_search_server_context_event_unavailable_label(
                    query,
                    self.telegram_message_search_server_pending,
                    hit_count,
                    &reason,
                );
                self.telegram_message_search_server_context_target_event_id = None;
                self.telegram_message_search_server_context_controls_metadata = metadata.clone();
                self.view
                    .label(
                        cx,
                        ids!(telegram_message_search_strip.search_server_context_controls_metadata),
                    )
                    .set_text(cx, &metadata);
                self.view
                    .label(
                        cx,
                        ids!(telegram_message_search_strip.server_context_boundary),
                    )
                    .set_text(cx, &metadata);
                enqueue_popup_notification(metadata, PopupKind::Warning, Some(4.0));
                return;
            }
        };

        let loaded_index = self.loaded_event_index_for_event_id(&target_event_id);
        let metadata = message_search_server_context_event_request_label(
            query,
            &target_event_id,
            loaded_item_count,
            hit_count,
            loaded_index.is_some(),
        );
        self.telegram_message_search_server_context_target_event_id = Some(target_event_id.clone());
        self.telegram_message_search_server_context_controls_metadata = metadata.clone();
        self.view
            .label(cx, ids!(telegram_message_search_strip.search_row.status))
            .set_text(cx, "context");
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_server_context_controls_metadata),
            )
            .set_text(cx, &metadata);
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.server_context_boundary),
            )
            .set_text(cx, &metadata);

        let loading_pane = self.view.loading_pane(cx, ids!(loading_pane));
        self.jump_to_event(cx, &target_event_id, None, portal_list, &loading_pane);
        if let Some(loaded_index) = loaded_index {
            let snippet = self
                .tl_state
                .as_ref()
                .and_then(|tl_state| {
                    loaded_event_plaintext_preview_for_event_id(&tl_state.items, &target_event_id)
                })
                .unwrap_or_else(|| "loaded event preview unavailable".to_string());
            self.show_telegram_message_search_server_context_event_found(
                cx,
                &target_event_id,
                loaded_index,
                &snippet,
                false,
            );
        } else {
            enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        }
    }

    fn show_telegram_message_search_server_context_event_found(
        &mut self,
        cx: &mut Cx,
        event_id: &OwnedEventId,
        loaded_index: usize,
        snippet: &str,
        after_pagination: bool,
    ) {
        self.set_telegram_search_mode_visible(cx, true);
        let query = if self
            .telegram_message_search_server_last_query
            .trim()
            .is_empty()
        {
            self.telegram_message_search_query.trim()
        } else {
            self.telegram_message_search_server_last_query.trim()
        };
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let metadata = message_search_server_context_event_found_label(
            query,
            event_id,
            loaded_index,
            loaded_item_count,
            self.telegram_message_search_server_hits.len(),
            snippet,
            after_pagination,
        );
        self.telegram_message_search_server_context_target_event_id = None;
        self.telegram_message_search_server_context_controls_metadata = metadata.clone();
        self.view
            .label(cx, ids!(telegram_message_search_strip.search_row.status))
            .set_text(cx, "context found");
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_server_context_controls_metadata),
            )
            .set_text(cx, &metadata);
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.server_context_boundary),
            )
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Success, Some(4.0));
    }

    fn submit_telegram_message_search_server_request(
        &mut self,
        cx: &mut Cx,
        next_batch: Option<String>,
        filter: MessageSearchServerFilter,
    ) {
        self.set_telegram_search_mode_visible(cx, true);
        let query = self.telegram_message_search_query.trim().to_string();
        let Some(timeline_kind) = self.timeline_kind.clone() else {
            let metadata = "Server search needs a loaded timeline before it can query Matrix.";
            self.telegram_message_search_server_last_error = metadata.to_string();
            self.view
                .label(
                    cx,
                    ids!(telegram_message_search_strip.search_server_preflight_controls_metadata),
                )
                .set_text(cx, metadata);
            enqueue_popup_notification(metadata, PopupKind::Error, Some(4.0));
            return;
        };
        if query.is_empty() {
            let metadata = "Server search needs a non-empty query.";
            self.telegram_message_search_server_last_error = metadata.to_string();
            self.view
                .label(
                    cx,
                    ids!(telegram_message_search_strip.search_server_preflight_controls_metadata),
                )
                .set_text(cx, metadata);
            enqueue_popup_notification(metadata, PopupKind::Info, Some(3.0));
            return;
        }

        let cursor_state = if next_batch.is_some() {
            "next cursor"
        } else {
            "first page"
        };
        let filter_state = message_search_server_filter_label(&filter);
        let metadata = format!(
            "Submitting live Matrix server search: query {} chars, {cursor_state}, room {}, {filter_state}.",
            query.chars().count(),
            timeline_kind.room_id(),
        );
        self.telegram_message_search_server_pending = true;
        self.telegram_message_search_server_last_query = query.clone();
        self.telegram_message_search_server_last_filter = filter.clone();
        self.telegram_message_search_server_hits.clear();
        self.telegram_message_search_server_context_target_event_id = None;
        self.telegram_message_search_server_last_error.clear();
        self.telegram_message_search_server_preflight_controls_metadata = metadata.clone();
        self.view
            .label(cx, ids!(telegram_message_search_strip.search_row.status))
            .set_text(cx, "server search");
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_server_preflight_controls_metadata),
            )
            .set_text(cx, &metadata);
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.results_row.result_summary),
            )
            .set_text(cx, "Searching Matrix server history...");
        submit_async_request(MatrixRequest::SearchMessagesServer {
            timeline_kind,
            query,
            filter,
            limit: 20,
            next_batch,
        });
    }

    fn submit_telegram_message_search_sender_filter(&mut self, cx: &mut Cx) {
        self.set_telegram_search_mode_visible(cx, true);
        let sender = self
            .telegram_message_search_sender_filter_draft
            .trim()
            .to_string();
        if sender.is_empty() {
            let metadata = "From filter needs a Matrix sender id such as @alice:example.org before live search.";
            self.telegram_message_search_advanced_filter_controls_metadata = metadata.to_string();
            self.view
                .label(
                    cx,
                    ids!(telegram_message_search_strip.search_advanced_filter_controls_metadata),
                )
                .set_text(cx, metadata);
            enqueue_popup_notification(metadata, PopupKind::Info, Some(3.0));
            return;
        }
        self.submit_telegram_message_search_server_request(
            cx,
            None,
            MessageSearchServerFilter {
                sender: Some(sender),
                media_only: false,
            },
        );
    }

    fn submit_telegram_message_search_media_filter(&mut self, cx: &mut Cx) {
        self.set_telegram_search_mode_visible(cx, true);
        self.submit_telegram_message_search_server_request(
            cx,
            None,
            MessageSearchServerFilter {
                sender: self
                    .telegram_message_search_server_last_filter
                    .sender
                    .clone(),
                media_only: true,
            },
        );
    }

    fn submit_telegram_message_search_server_next_page(&mut self, cx: &mut Cx) {
        self.set_telegram_search_mode_visible(cx, true);
        if self.telegram_message_search_server_pending {
            let metadata = message_search_server_next_page_unavailable_label(
                &self.telegram_message_search_query,
                true,
                &self.telegram_message_search_server_last_error,
            );
            self.telegram_message_search_server_preflight_controls_metadata = metadata.clone();
            self.view
                .label(
                    cx,
                    ids!(telegram_message_search_strip.search_server_preflight_controls_metadata),
                )
                .set_text(cx, &metadata);
            enqueue_popup_notification(metadata, PopupKind::Info, Some(3.0));
            return;
        }

        let Some(next_batch) = self.telegram_message_search_server_next_batch.clone() else {
            let metadata = message_search_server_next_page_unavailable_label(
                &self.telegram_message_search_query,
                false,
                &self.telegram_message_search_server_last_error,
            );
            self.telegram_message_search_server_preflight_controls_metadata = metadata.clone();
            self.view
                .label(
                    cx,
                    ids!(telegram_message_search_strip.search_server_preflight_controls_metadata),
                )
                .set_text(cx, &metadata);
            self.view
                .label(
                    cx,
                    ids!(telegram_message_search_strip.server_context_boundary),
                )
                .set_text(cx, &metadata);
            enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
            return;
        };

        self.submit_telegram_message_search_server_request(
            cx,
            Some(next_batch),
            self.telegram_message_search_server_last_filter.clone(),
        );
    }

    fn apply_telegram_message_search_server_result(
        &mut self,
        cx: &mut Cx,
        result: Result<MessageSearchServerResponse, String>,
    ) {
        self.set_telegram_search_mode_visible(cx, true);
        self.telegram_message_search_server_pending = false;
        match result {
            Ok(response) => {
                let result_text = message_search_server_result_label(&response);
                let preflight_text = message_search_server_live_preflight_label(&response);
                self.telegram_message_search_server_last_query = response.query.clone();
                self.telegram_message_search_server_last_filter = response.filter.clone();
                self.telegram_message_search_server_next_batch = response.next_batch.clone();
                self.telegram_message_search_server_room_id = response.room_id.clone();
                self.telegram_message_search_server_hits = response.hits.clone();
                self.telegram_message_search_server_context_target_event_id = None;
                self.telegram_message_search_server_last_error.clear();
                self.telegram_message_search_server_preflight_controls_metadata =
                    preflight_text.clone();
                self.view
                    .label(cx, ids!(telegram_message_search_strip.search_row.status))
                    .set_text(cx, "server result");
                self.view
                    .label(
                        cx,
                        ids!(telegram_message_search_strip.results_row.result_summary),
                    )
                    .set_text(cx, &result_text);
                self.view
                    .label(
                        cx,
                        ids!(
                            telegram_message_search_strip.search_server_preflight_controls_metadata
                        ),
                    )
                    .set_text(cx, &preflight_text);
                self.view
                    .label(
                        cx,
                        ids!(telegram_message_search_strip.server_context_boundary),
                    )
                    .set_text(cx, &message_search_server_live_boundary_label(&response));
                enqueue_popup_notification(result_text, PopupKind::Success, Some(5.0));
            }
            Err(error) => {
                let query = self.telegram_message_search_server_last_query.clone();
                let metadata = message_search_server_error_label(&query, &error);
                self.telegram_message_search_server_last_error = error;
                self.telegram_message_search_server_next_batch = None;
                self.telegram_message_search_server_hits.clear();
                self.telegram_message_search_server_context_target_event_id = None;
                self.telegram_message_search_server_preflight_controls_metadata = metadata.clone();
                self.view
                    .label(cx, ids!(telegram_message_search_strip.search_row.status))
                    .set_text(cx, "server error");
                self.view
                    .label(
                        cx,
                        ids!(telegram_message_search_strip.results_row.result_summary),
                    )
                    .set_text(cx, &metadata);
                self.view
                    .label(
                        cx,
                        ids!(
                            telegram_message_search_strip.search_server_preflight_controls_metadata
                        ),
                    )
                    .set_text(cx, &metadata);
                enqueue_popup_notification(metadata, PopupKind::Error, Some(6.0));
            }
        }
    }

    fn stage_telegram_message_search_server_preflight_control(
        &mut self,
        cx: &mut Cx,
        action: &str,
    ) {
        self.set_telegram_search_mode_visible(cx, true);
        let query = self.telegram_message_search_query.trim();
        let loaded_item_count = self
            .tl_state
            .as_ref()
            .map(|tl_state| tl_state.items.len())
            .unwrap_or_default();
        let match_count = self.telegram_message_search_matches.len();
        let timeline_loaded = self.tl_state.is_some();
        let metadata = if action.trim().eq_ignore_ascii_case("Server query") {
            loaded_message_search_server_query_local_snapshot_label(
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                timeline_loaded,
                self.pinned_events.len(),
                &self.telegram_message_search_server_context_controls_metadata,
            )
        } else if action.trim().eq_ignore_ascii_case("Contract") {
            message_search_matrix_contract_acceptance_label(
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                timeline_loaded,
                self.pinned_events.len(),
                &self.telegram_message_search_server_context_controls_metadata,
                &self.telegram_message_search_server_preflight_controls_metadata,
            )
        } else if action.trim().eq_ignore_ascii_case("Taxonomy") {
            message_search_remote_result_taxonomy_packet_label(
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                timeline_loaded,
                self.pinned_events.len(),
                &self.telegram_message_search_server_context_controls_metadata,
                &self.telegram_message_search_server_preflight_controls_metadata,
            )
        } else {
            loaded_message_search_server_preflight_controls_label(
                Some(action),
                query,
                loaded_item_count,
                match_count,
                self.telegram_message_search_active_match,
                timeline_loaded,
                self.pinned_events.len(),
                &self.telegram_message_search_server_context_controls_metadata,
            )
        };
        self.telegram_message_search_server_preflight_controls_metadata = metadata.clone();
        self.view
            .label(
                cx,
                ids!(telegram_message_search_strip.search_server_preflight_controls_metadata),
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
                "{action} preflight stayed local. {MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL}"
            ),
            );
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
    }
}
