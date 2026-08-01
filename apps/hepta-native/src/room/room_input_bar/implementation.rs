use super::*;

impl ScriptHook for RoomInputBar {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            let send_on_enter = cx.global::<AppPreferencesGlobal>().0.send_on_enter;
            self.mentionable_text_input(cx, ids!(mentionable_text_input))
                .text_input_ref()
                .set_submit_on_enter(send_on_enter);
        });
    }
}

impl Widget for RoomInputBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let room_screen_props = scope
            .props
            .get::<RoomScreenProps>()
            .expect("BUG: RoomScreenProps should be available in Scope::props for RoomInputBar");

        match event.hits(
            cx,
            self.view
                .view(cx, ids!(replying_preview.reply_preview_content))
                .area(),
        ) {
            // If the hit occurred on the replying message preview, jump to it.
            Hit::FingerUp(fe) if fe.is_over && fe.is_primary_hit() && fe.was_tap() => {
                if let Some(event_id) = self
                    .replying_to
                    .as_ref()
                    .and_then(|(event_tl_item, _)| event_tl_item.event_id().map(ToOwned::to_owned))
                {
                    cx.widget_action(
                        room_screen_props.room_screen_widget_uid,
                        MessageAction::JumpToEvent(event_id),
                    );
                } else {
                    enqueue_popup_notification(
                        "BUG: couldn't find the message you're replying to.",
                        PopupKind::Error,
                        None,
                    );
                }
            }
            _ => {}
        }

        if let Event::Actions(actions) = event {
            // Handle changes to the `send_on_enter` preference.
            for action in actions {
                if let Some(AppPreferencesAction::SendOnEnterChanged(v)) = action.downcast_ref() {
                    self.mentionable_text_input(cx, ids!(mentionable_text_input))
                        .text_input_ref()
                        .set_submit_on_enter(*v);
                }
            }

            self.handle_actions(cx, actions, room_screen_props);
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Shrink the input_bar's height as the editing pane slides in,
        // and grow it back as the editing pane slides out.
        // slide=1.0 → editing pane hidden → input_bar at full Fit height.
        // slide=0.0 → editing pane shown → input_bar at zero height.
        let slide = self.editing_pane(cx, ids!(editing_pane)).slide();
        let input_bar = self.view.view(cx, ids!(input_bar));

        // Remap slide through a steeper curve so the input_bar reaches
        // its full target height before the ExpDecay tail.
        let remapped = (slide as f64 * 1.25).min(1.0);
        if remapped >= 1.0 {
            // Input_bar has reached its full natural height: switch to Fit
            // so it can respond to content changes normally.
            // Update the cached height for future animations.
            let h = input_bar.area().rect(cx).size.y;
            if h > 0.0 {
                self.input_bar_natural_height = h;
            }
            if let Some(mut inner) = input_bar.borrow_mut() {
                inner.walk.height = Size::fit();
            }
        } else {
            let target = self.input_bar_natural_height;
            if let Some(mut inner) = input_bar.borrow_mut() {
                inner.walk.height = Size::Fixed((target * remapped).max(0.0));
            }
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl RoomInputBar {
    fn handle_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        room_screen_props: &RoomScreenProps,
    ) {
        let mentionable_text_input = self.mentionable_text_input(cx, ids!(mentionable_text_input));
        let text_input = mentionable_text_input.text_input_ref();
        mentionable_text_input.update_cached_member_suggestions(
            cx,
            room_screen_props
                .room_members
                .as_ref()
                .map(|members| members.as_slice()),
        );

        for action in actions {
            if let Some(RoomInputBarAction::AttachmentHandoffConfirmed {
                kind,
                timeline_kind,
                in_reply_to,
            }) = action.downcast_ref()
            {
                let kind = *kind;
                let label = kind.label();
                match pick_telegram_attachment_file(kind) {
                    AttachmentFilePickResult::Picked(file_path) => {
                        self.telegram_attachment_send_retry_attempt = None;
                        let replaced_attachment = self
                            .telegram_pending_attachment_send
                            .as_ref()
                            .map(|pending| {
                                (pending.filename.clone(), pending.validation_error.clone())
                            });
                        let filename = display_attachment_filename(&file_path);
                        let file_extension = display_attachment_extension(&file_path);
                        let file_size_bytes = telegram_attachment_file_size(&file_path);
                        let mime_type = telegram_attachment_mime_type(&file_path);
                        let mime_label = mime_type.to_string();
                        let image_dimensions_label = (kind == AttachmentHandoffKind::Photo)
                            .then(|| selected_image_dimensions_label(&file_path, &mime_type));
                        let audio_duration_label = (kind == AttachmentHandoffKind::Voice)
                            .then(|| voice_audio_duration_label(&file_path, &mime_type));
                        let audio_waveform_codec_label = (kind == AttachmentHandoffKind::Voice)
                            .then(|| voice_audio_waveform_codec_label(&file_path, &mime_type));
                        let caption_preview =
                            summarize_attachment_caption(&mentionable_text_input.text());
                        self.telegram_pending_attachment_send = Some(PendingAttachmentSend {
                            kind,
                            timeline_kind: timeline_kind.clone(),
                            file_path,
                            mime_type: mime_type.clone(),
                            filename: filename.clone(),
                            file_extension,
                            file_size_bytes,
                            image_dimensions_label: image_dimensions_label.clone(),
                            audio_duration_label: audio_duration_label.clone(),
                            audio_waveform_codec_label: audio_waveform_codec_label.clone(),
                            caption_preview: caption_preview.clone(),
                            in_reply_to: in_reply_to.clone(),
                            validation_error: None,
                        });
                        let image_note = image_dimensions_label
                            .as_deref()
                            .map(|label| format!("; {label}"))
                            .unwrap_or_default();
                        let audio_note = audio_duration_label
                            .as_deref()
                            .map(|label| format!("; {label}"))
                            .unwrap_or_default();
                        let audio_waveform_note = audio_waveform_codec_label
                            .as_deref()
                            .map(|label| format!("; {label}"))
                            .unwrap_or_default();
                        let metadata_note =
                            format!("{image_note}{audio_note}{audio_waveform_note}");
                        let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                            "selected",
                            label,
                            Some(&filename),
                            Some(&mime_label),
                            file_size_bytes,
                            Some(&caption_preview),
                            in_reply_to.is_some(),
                            None,
                            replaced_attachment
                                .as_ref()
                                .map(|(previous_filename, _)| previous_filename.as_str()),
                        );
                        self.telegram_attachment_local_status = if let Some((
                            previous_filename,
                            previous_validation_error,
                        )) = &replaced_attachment
                        {
                            let recovery_note = previous_validation_error
                                .as_ref()
                                .map(|reason| {
                                    format!("; cleared previous local validation warning: {reason}")
                                })
                                .unwrap_or_default();
                            format!(
                                "{label} selected for review: {filename} ({mime_label}{metadata_note}); replaced previous pending attachment locally: {previous_filename}{recovery_note}"
                            )
                        } else {
                            format!(
                                "{label} selected for review: {filename} ({mime_label}{metadata_note})"
                            )
                        };
                        if kind == AttachmentHandoffKind::Voice {
                            self.telegram_voice_local_status = format!(
                                "Selected audio file staged locally: {filename}; {}; {}",
                                audio_duration_label.as_deref().unwrap_or(
                                    "duration: unavailable before recorder/player metadata"
                                ),
                                audio_waveform_codec_label.as_deref().unwrap_or(
                                    "codec/waveform: unavailable before selected audio analysis"
                                )
                            );
                            self.update_telegram_voice_message_panel(cx);
                            self.set_telegram_voice_message_panel_visible(cx, false);
                        }
                        let voice_lifecycle_metadata =
                            (kind == AttachmentHandoffKind::Voice).then(|| {
                                self.current_voice_lifecycle_metadata_label(
                                    "audio file selected",
                                    "desktop audio picker accepted; pending review loaded",
                                )
                            });
                        let voice_lifecycle_note = voice_lifecycle_metadata
                            .as_deref()
                            .map(|metadata| format!(" {metadata}"))
                            .unwrap_or_default();
                        self.update_telegram_attachment_picker(cx);
                        self.set_telegram_attachment_picker_visible(cx, true);
                        if replaced_attachment.is_some() {
                            self.set_message_send_operation_status(
                                cx,
                                "review-replaced",
                                "Attachment selection replaced locally",
                                &format!(
                                    "A newly selected desktop attachment replaced only the local pending review state and clears any previous local validation warning. The previous selected file was not uploaded, sent, canceled through SDK send queue, or mutated on Matrix; composer caption/reply context stays local and review-row Send is still the only SendAttachment submit path. {lifecycle_metadata}{voice_lifecycle_note}"
                                ),
                            );
                        } else {
                            self.set_message_send_operation_status(
                                cx,
                                "review-pending",
                                "Attachment selected locally",
                                &format!(
                                    "Selected desktop attachment is staged in local review state. Caption preview live-updates from composer text and reply context remains local. No MatrixRequest::SendAttachment, upload, or media send is submitted until Send is clicked; review-row Send is the only attachment path that consumes caption/reply context. Discard and Close clear only the pending attachment locally. {lifecycle_metadata}{voice_lifecycle_note}"
                                ),
                            );
                        }
                        let replacement_note = replaced_attachment
                            .as_ref()
                            .map(|(previous_filename, previous_validation_error)| {
                                let recovery_note = previous_validation_error
                                    .as_ref()
                                    .map(|reason| {
                                        format!(" Cleared previous local validation warning: {reason}.")
                                    })
                                    .unwrap_or_default();
                                format!(" Replaced previous pending attachment locally: {previous_filename}.{recovery_note}")
                            })
                            .unwrap_or_default();
                        enqueue_popup_notification(
                            format!(
                                "{label} attachment selected for local review: {filename} ({mime_label}{metadata_note}).{replacement_note} {lifecycle_metadata}{voice_lifecycle_note} Click Send to submit or Discard to clear it."
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    }
                    AttachmentFilePickResult::Canceled => {
                        let preserved_metadata = self
                            .telegram_pending_attachment_send
                            .as_ref()
                            .map(|pending| {
                                let mime_label = pending.mime_type.to_string();
                                attachment_review_lifecycle_metadata_label(
                                    "picker cancel preserved",
                                    pending.kind.label(),
                                    Some(&pending.filename),
                                    Some(&mime_label),
                                    pending.file_size_bytes,
                                    Some(&pending.caption_preview),
                                    pending.in_reply_to.is_some(),
                                    pending.validation_error.as_deref(),
                                    None,
                                )
                            })
                            .unwrap_or_else(|| {
                                attachment_review_lifecycle_metadata_label(
                                    "picker canceled",
                                    label,
                                    None,
                                    None,
                                    None,
                                    None,
                                    false,
                                    None,
                                    None,
                                )
                            });
                        let preserved_attachment = self
                            .telegram_pending_attachment_send
                            .as_ref()
                            .map(|pending| pending.filename.clone());
                        self.telegram_attachment_local_status = if let Some(filename) =
                            &preserved_attachment
                        {
                            format!(
                                "{label} attachment picker canceled locally; still reviewing existing pending attachment: {filename}"
                            )
                        } else {
                            format!("{label} attachment picker canceled locally")
                        };
                        if kind == AttachmentHandoffKind::Voice {
                            self.telegram_voice_local_status =
                                "Voice audio picker canceled locally".to_string();
                            self.update_telegram_voice_message_panel(cx);
                        }
                        let voice_lifecycle_note = (kind == AttachmentHandoffKind::Voice)
                            .then(|| {
                                format!(
                                    " {}",
                                    self.current_voice_lifecycle_metadata_label(
                                        "picker canceled",
                                        "desktop audio picker canceled"
                                    )
                                )
                            })
                            .unwrap_or_default();
                        self.update_telegram_attachment_picker(cx);
                        self.set_telegram_attachment_picker_visible(cx, true);
                        if preserved_attachment.is_some() {
                            self.set_message_send_operation_status(
                                cx,
                                "review-preserved",
                                "Picker cancel preserved pending attachment",
                                &format!(
                                    "Canceling a new desktop picker leaves the existing pending attachment review, composer caption, and reply preview intact. No pending attachment was cleared, uploaded, sent, canceled through SDK send queue, or mutated on Matrix. {preserved_metadata}{voice_lifecycle_note}"
                                ),
                            );
                        }
                        enqueue_popup_notification(
                            format!(
                                "{label} attachment picker canceled. Existing pending review is preserved if one was already selected; no upload or Matrix media send was started. {preserved_metadata}{voice_lifecycle_note}"
                            ),
                            PopupKind::Info,
                            Some(3.0),
                        );
                    }
                    AttachmentFilePickResult::Unsupported => {
                        self.telegram_attachment_local_status =
                            format!("{label} attachment picker is not available on this platform");
                        if kind == AttachmentHandoffKind::Voice {
                            self.telegram_voice_local_status =
                                "Voice audio picker unsupported on this platform".to_string();
                            self.update_telegram_voice_message_panel(cx);
                        }
                        let voice_lifecycle_note = (kind == AttachmentHandoffKind::Voice)
                            .then(|| {
                                format!(
                                    " {}",
                                    self.current_voice_lifecycle_metadata_label(
                                        "picker unsupported",
                                        "desktop audio picker unavailable"
                                    )
                                )
                            })
                            .unwrap_or_default();
                        self.update_telegram_attachment_picker(cx);
                        self.set_telegram_attachment_picker_visible(cx, true);
                        enqueue_popup_notification(
                            format!(
                                "{label} attachment picker is not available on this platform yet. No upload or Matrix media send was started.{voice_lifecycle_note}"
                            ),
                            PopupKind::Warning,
                            Some(4.0),
                        );
                    }
                }
                continue;
            }

            if let Some(RoomInputBarAction::AttachmentHandoffCanceled { kind }) =
                action.downcast_ref()
            {
                let kind = *kind;
                let label = kind.label();
                let pending_attachment = self.telegram_pending_attachment_send.as_ref();
                let pending_filename = pending_attachment.map(|pending| pending.filename.clone());
                let pending_voice_filename = pending_attachment
                    .filter(|pending| pending.kind == AttachmentHandoffKind::Voice)
                    .map(|pending| pending.filename.clone());
                let reply_context_loaded = pending_attachment
                    .map(|pending| pending.in_reply_to.is_some())
                    .unwrap_or_else(|| self.replying_to.is_some());
                let has_pending_attachment = pending_filename.is_some();
                self.telegram_attachment_local_status = if let Some(filename) = &pending_filename {
                    format!(
                        "{label} attachment send confirmation canceled locally; still reviewing existing pending attachment: {filename}"
                    )
                } else {
                    format!("{label} attachment send confirmation canceled before picker")
                };
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, has_pending_attachment);
                if kind == AttachmentHandoffKind::Voice {
                    self.telegram_voice_local_status =
                        "Voice Send confirmation canceled locally".to_string();
                    self.update_telegram_voice_message_panel(cx);
                    self.set_telegram_voice_message_panel_visible(cx, true);
                    let cancel_metadata = voice_confirmation_cancel_metadata_label(
                        pending_voice_filename.as_deref(),
                        reply_context_loaded,
                    );
                    self.set_message_send_operation_status(
                            cx,
                            "voice-confirmation-canceled-local",
                            "Voice confirmation canceled locally",
                        &format!(
                            "{cancel_metadata} No desktop picker, microphone permission, recorder, upload, SendAttachment, SDK queue cancel, room-state, membership, gateway/runtime/auth, or live mutation request was emitted."
                        ),
                    );
                    enqueue_popup_notification(
                        format!(
                            "Voice Send confirmation canceled before the desktop audio picker. {cancel_metadata}"
                        ),
                        PopupKind::Info,
                        Some(3.0),
                    );
                }
                continue;
            }

            if let Some(RoomInputBarAction::AttachmentSendRetryConfirmed { attempt }) =
                action.downcast_ref()
            {
                let attempt = attempt.clone();
                let label = attempt.kind.label();
                let filename = attempt.filename.clone();
                let mime_label = attempt.mime_type.to_string();
                let retry_metadata = attachment_send_failure_retry_confirmation_label(
                    &filename,
                    label,
                    attempt.caption.is_some(),
                    attempt.in_reply_to.is_some(),
                );
                self.telegram_attachment_send_retry_attempt = Some(attempt.clone());
                self.telegram_attachment_send_cached_error = None;
                self.telegram_attachment_local_status = format!(
                    "{label} attachment retry submitted after confirmation: {filename} ({mime_label})"
                );
                submit_async_request(MatrixRequest::SendAttachment {
                    timeline_kind: attempt.timeline_kind,
                    file_path: attempt.file_path,
                    mime_type: attempt.mime_type,
                    caption: attempt.caption,
                    mentions: attempt.mentions,
                    in_reply_to: attempt.in_reply_to,
                });
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "retry-confirmed",
                    "Attachment Retry confirmed",
                    &format!(
                        "PositiveConfirmationModal accepted the failed attachment handoff Retry. The cached MatrixRequest::SendAttachment was resubmitted with the same TimelineKind, local file path, MIME type, caption, compact caption mentions, and reply id. This does not retry or resume accepted SDK queue uploads, abort uploads, remove queued media, send a caption-only SendMessage, mutate room-state or membership, touch account/profile, call gateway/runtime/auth, or perform live mutation. {retry_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "{label} attachment retry confirmed and resubmitted to the existing SendAttachment handoff: {filename} ({mime_label})."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                continue;
            }

            if let Some(RoomInputBarAction::LocationSendConfirmed {
                timeline_kind,
                coords,
                replied_to,
                #[cfg(feature = "tsp")]
                sign_with_tsp,
            }) = action.downcast_ref()
            {
                let geo_uri = format!(
                    "{}{},{}",
                    utils::GEO_URI_SCHEME,
                    coords.latitude,
                    coords.longitude
                );
                let message = RoomMessageEventContent::new(MessageType::Location(
                    LocationMessageEventContent::new(geo_uri.clone(), geo_uri),
                ));
                // Location confirmation evidence: only this confirmed action submits the
                // existing Matrix location SendMessage path.
                submit_async_request(MatrixRequest::SendMessage {
                    timeline_kind: timeline_kind.clone(),
                    message,
                    replied_to: replied_to.clone().map(RoomInputBarReplyTarget::into_reply),
                    #[cfg(feature = "tsp")]
                    sign_with_tsp: *sign_with_tsp,
                });

                self.clear_replying_to(cx);
                self.telegram_pending_attachment_send = None;
                self.telegram_attachment_send_retry_attempt = None;
                let location_preview = self.location_preview(cx, ids!(location_preview));
                location_preview.clear();
                location_preview.redraw(cx);
                enqueue_popup_notification(
                    "Location send confirmed. Existing Matrix location message path was requested.",
                    PopupKind::Info,
                    Some(4.0),
                );
                self.set_message_send_operation_status(
                    cx,
                    "location submitted",
                    "Location SendMessage submitted",
                    "Existing MatrixRequest::SendMessage was submitted for this location. Queued/progress/failure labels plus Retry/Cancel controls are local evidence only; no retry or cancel request was emitted from the evidence strip.",
                );
                return;
            }
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .pause_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Pause");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .resume_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Resume");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .reorder_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Reorder");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .background_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Background");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .clear_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Clear");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .status_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Status");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .handle_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Handle");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .timeline_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Timeline");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .cancel_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Cancel");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .source_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Source");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .status_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Status");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .progress_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Progress");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .pause_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Pause");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .resume_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Resume");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .cancel_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Cancel");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .retry_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Retry");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .drilldown_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Drilldown");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .contract_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Contract");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .taxonomy_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Taxonomy");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .request_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Request");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .result_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Result");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .error_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Error");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .retry_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Retry");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .source_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Source");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .status_actions
                        .retry_send_operation_button
                ),
            )
            .clicked(actions)
        {
            if let Some(attempt) = self.telegram_attachment_send_retry_attempt.clone() {
                let label = attempt.kind.label();
                let filename = attempt.filename.clone();
                let retry_metadata = attachment_send_failure_retry_confirmation_label(
                    &filename,
                    label,
                    attempt.caption.is_some(),
                    attempt.in_reply_to.is_some(),
                );
                self.set_message_send_operation_status(
                    cx,
                    "retry-confirmation-open",
                    "Confirm attachment Retry",
                    &format!(
                        "Attachment Retry is available only for the cached immediate SendAttachment handoff failure. Confirming will resubmit the same local file path, MIME type, caption, reply id, and TimelineKind; canceling the confirmation stays local. This is not SDK queue retry/resume, upload abort, queue removal, delivery receipt mapping, caption-only SendMessage fallback, room-state, membership, gateway/runtime/auth, or live mutation. {retry_metadata}"
                    ),
                );
                let attempt_for_accept = attempt.clone();
                let content = ConfirmationModalContent {
                    title_text: "Retry attachment handoff".into(),
                    body_text: format!(
                        "Retry sending {label} attachment {filename}? This reuses the cached local file, MIME type, caption, reply id, and timeline after the worker failed before SDK queue ownership. It does not retry or cancel accepted SDK queue uploads."
                    )
                    .into(),
                    accept_button_text: Some("Retry".into()),
                    cancel_button_text: Some("Keep Failed".into()),
                    on_accept_clicked: Some(Box::new(move |cx| {
                        cx.action(RoomInputBarAction::AttachmentSendRetryConfirmed {
                            attempt: attempt_for_accept.clone(),
                        });
                    })),
                    on_cancel_clicked: Some(Box::new(move |_cx| {
                        enqueue_popup_notification(
                            format!(
                                "{label} attachment Retry confirmation canceled locally. No SendAttachment resubmit, SDK queue retry, upload abort, or live mutation was emitted."
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    })),
                };
                enqueue_popup_notification(
                    format!(
                        "Attachment Retry confirmation opened for {filename}. No SendAttachment resubmit occurs before confirmation."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                    content,
                ))));
            } else {
                self.set_message_send_operation_status(
                    cx,
                    "retry-local",
                    "Retry staged locally",
                    "Retry had no cached failed SendAttachment handoff to reuse, so it only updates this local recovery copy. It does not submit SendMessage or SendAttachment, does not send the caption as a plain message, does not duplicate media upload, does not replace the SDK send queue item, and sends no room-state, membership, gateway/runtime/auth, account/profile, or live mutation request.",
                );
                enqueue_popup_notification(
                    "Retry has no cached failed attachment handoff. No Matrix retry, duplicate SendAttachment, caption-only SendMessage, or upload was requested.",
                    PopupKind::Info,
                    Some(4.0),
                );
            }
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .status_actions
                        .cancel_send_operation_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_attachment_send_retry_attempt = None;
            self.set_message_send_operation_status(
                cx,
                "cancel-local",
                "Cancel staged locally",
                "Cancel clears only the local cached failed-handoff retry attempt and updates this recovery copy after a queued attachment handoff or popup failure. It does not abort SDK send-queue work, remove queued attachments, cancel upload, clear Matrix queue state, or send a Matrix cancel, room-state, membership, gateway/runtime/auth, account/profile, or live mutation request.",
            );
            enqueue_popup_notification(
                "Cancel cleared local attachment retry cache only. No SDK send-queue cancel, Matrix cancel, upload abort, or queue removal was emitted.",
                PopupKind::Info,
                Some(4.0),
            );
        }

        // Clear the replying-to preview pane if the "cancel reply" button was clicked
        // or if the `Escape` key was pressed within the message input box.
        if self.button(cx, ids!(cancel_reply_button)).clicked(actions)
            || text_input.escaped(actions)
        {
            self.clear_replying_to(cx);
            self.redraw(cx);
        }

        if self.button(cx, ids!(attachment_button)).clicked(actions) {
            self.show_telegram_attachment_picker(cx);
            enqueue_popup_notification(
                "File attachments require confirmation first. On desktop, choosing Photo or File opens the native picker, then selected files enter local review before Matrix attachment send.",
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_header
                        .close_attachment_picker_button
                ),
            )
            .clicked(actions)
        {
            let discarded = self.telegram_pending_attachment_send.take();
            self.telegram_attachment_send_retry_attempt = None;
            self.set_telegram_attachment_picker_visible(cx, false);
            if let Some(pending) = discarded {
                let mime_label = pending.mime_type.to_string();
                let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                    "closed",
                    pending.kind.label(),
                    Some(&pending.filename),
                    Some(&mime_label),
                    pending.file_size_bytes,
                    Some(&pending.caption_preview),
                    pending.in_reply_to.is_some(),
                    pending.validation_error.as_deref(),
                    None,
                );
                let validation_note = pending
                    .validation_error
                    .as_deref()
                    .map(|reason| format!("; cleared local validation warning: {reason}"))
                    .unwrap_or_default();
                self.telegram_attachment_local_status = format!(
                    "{} attachment closed and discarded locally: {}{}",
                    pending.kind.label(),
                    pending.filename,
                    validation_note
                );
                self.set_message_send_operation_status(
                    cx,
                    "closed-local",
                    "Attachment review closed locally",
                    &format!(
                        "Close consumed and cleared the pending selected attachment plus any validation warning locally while preserving composer caption/reply text. Repeated Close or review-row Send after Close has no pending attachment to submit. No MatrixRequest::SendAttachment, caption-only SendMessage, upload, SDK send-queue cancel, room-state, membership, gateway/runtime/auth, account/profile, or live mutation request was emitted. {lifecycle_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "{} attachment review closed and discarded locally. No upload or Matrix media send was started. {lifecycle_metadata}",
                        pending.kind.label()
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            } else {
                let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                    "empty close",
                    "Attachment",
                    None,
                    None,
                    None,
                    None,
                    false,
                    None,
                    None,
                );
                self.telegram_attachment_local_status =
                    "Attachment picker closed locally with no pending attachment".to_string();
                self.set_message_send_operation_status(
                    cx,
                    "empty-held",
                    "Close held locally",
                    &format!(
                        "Close found no pending attachment review state. This empty Close stays local, preserves composer caption/reply text, and does not submit SendAttachment, send a caption-only SendMessage, upload media, cancel SDK send-queue work, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live state. {lifecycle_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "Attachment picker closed locally. No native picker, upload, or Matrix media send was started. {lifecycle_metadata}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            }
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_review_actions
                        .send_selected_attachment_button
                ),
            )
            .clicked(actions)
        {
            if let Some(mut pending) = self.telegram_pending_attachment_send.take() {
                if let Err(validation_reason) =
                    validate_telegram_attachment_file_for_review_send(&pending.file_path)
                {
                    let label = pending.kind.label();
                    let filename = pending.filename.clone();
                    let mime_label = pending.mime_type.to_string();
                    pending.file_size_bytes = telegram_attachment_file_size(&pending.file_path);
                    if pending.kind == AttachmentHandoffKind::Photo {
                        pending.image_dimensions_label = Some(selected_image_dimensions_label(
                            &pending.file_path,
                            &pending.mime_type,
                        ));
                    }
                    if pending.kind == AttachmentHandoffKind::Voice {
                        pending.audio_duration_label = Some(voice_audio_duration_label(
                            &pending.file_path,
                            &pending.mime_type,
                        ));
                    }
                    pending.validation_error = Some(validation_reason.to_string());
                    self.telegram_pending_attachment_send = Some(pending);
                    self.telegram_attachment_local_status = format!(
                        "{label} attachment validation held locally: {filename} ({mime_label}); {validation_reason}"
                    );
                    self.update_telegram_attachment_picker(cx);
                    self.set_telegram_attachment_picker_visible(cx, true);
                    self.set_message_send_operation_status(
                        cx,
                        "validation-held",
                        "Attachment validation held locally",
                        "Review-row Send revalidated the selected file before MatrixRequest::SendAttachment. The selected path was unreadable, not a regular file, or an empty file, so pending review stayed local with a visible validation warning, composer caption/reply text was preserved, and no SendAttachment, caption-only SendMessage, upload, SDK send-queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request was emitted. Replace, Discard, and Close are the local recovery paths; Retry/Cancel controls remain local evidence only.",
                    );
                    enqueue_popup_notification(
                        format!(
                            "{label} attachment stayed in local review because {validation_reason}. Choose another file or discard it."
                        ),
                        PopupKind::Warning,
                        Some(4.0),
                    );
                    return;
                }
                let PendingAttachmentSend {
                    kind,
                    timeline_kind,
                    file_path,
                    mime_type,
                    filename,
                    in_reply_to,
                    ..
                } = pending;
                let label = kind.label();
                let mime_label = mime_type.to_string();
                let caption_text = mentionable_text_input.text().trim().to_string();
                let mentions = (!caption_text.is_empty())
                    .then(|| {
                        mentionable_text_input.mentions_for_text(
                            &caption_text,
                            room_screen_props
                                .room_members
                                .as_ref()
                                .map(|members| members.as_slice()),
                        )
                    })
                    .flatten();
                let caption = (!caption_text.is_empty())
                    .then(|| TextMessageEventContent::plain(caption_text));
                let retry_attempt = AttachmentSendRetryAttempt {
                    kind,
                    timeline_kind: timeline_kind.clone(),
                    file_path: file_path.clone(),
                    mime_type: mime_type.clone(),
                    filename: filename.clone(),
                    caption: caption.clone(),
                    mentions: mentions.clone(),
                    in_reply_to: in_reply_to.clone(),
                };
                self.telegram_attachment_send_retry_attempt = Some(retry_attempt);
                self.telegram_attachment_send_cached_error = None;

                // Consume pending state before submit so duplicate/second clicks fall
                // into the local empty review guard instead of resubmitting.
                submit_async_request(MatrixRequest::SendAttachment {
                    timeline_kind,
                    file_path,
                    mime_type,
                    caption,
                    mentions,
                    in_reply_to,
                });

                self.clear_replying_to(cx);
                mentionable_text_input.set_text(cx, "");
                self.update_hepta_command_preview(cx, "");
                self.telegram_attachment_local_status =
                    format!("{label} attachment send queued: {filename} ({mime_label})");
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "queued-only",
                    "Attachment SendAttachment submitted",
                    "MatrixRequest::SendAttachment was submitted only after local attachment review Send consumed pending state before submit; this is the handoff-submitted taxonomy boundary and caches the last validated handoff for a possible confirmed worker-failure Retry. The UI labels this as queued-only until the worker returns a queued-confirmed or failure-copy handoff result: the existing matrix-sdk-ui Timeline::send_attachment().use_send_queue() path owns upload/media send. Review-row Send is the only attachment path that consumes the current composer caption into SendAttachment, carries compact caption mentions through AttachmentConfig.mentions, carries the captured reply/thread event id, then clears composer text and reply preview after submit. Retry never auto-runs; Cancel does not abort, remove, or cancel SDK send-queue work.",
                );
                enqueue_popup_notification(
                    format!(
                        "{label} attachment queued for Matrix media send after review: {filename} ({mime_label})."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            } else {
                self.telegram_attachment_local_status =
                    "No selected attachment is waiting for review".to_string();
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "empty-held",
                    "Review Send held locally",
                    "No pending attachment was available for review-row Send. This empty or duplicate Send stays local and preserves composer caption/reply text: no duplicate MatrixRequest::SendAttachment, no caption-only SendMessage, no upload, no SDK send-queue cancel, and no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.",
                );
                enqueue_popup_notification(
                    "No selected attachment is waiting for review. Choose Photo or File first.",
                    PopupKind::Info,
                    Some(3.0),
                );
            }
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_review_actions
                        .discard_selected_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_attachment_send_retry_attempt = None;
            if let Some(pending) = self.telegram_pending_attachment_send.take() {
                let mime_label = pending.mime_type.to_string();
                let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                    "discarded",
                    pending.kind.label(),
                    Some(&pending.filename),
                    Some(&mime_label),
                    pending.file_size_bytes,
                    Some(&pending.caption_preview),
                    pending.in_reply_to.is_some(),
                    pending.validation_error.as_deref(),
                    None,
                );
                let validation_note = pending
                    .validation_error
                    .as_deref()
                    .map(|reason| format!("; cleared local validation warning: {reason}"))
                    .unwrap_or_default();
                self.telegram_attachment_local_status = format!(
                    "{} attachment discarded locally: {}{}",
                    pending.kind.label(),
                    pending.filename,
                    validation_note
                );
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "discarded-local",
                    "Attachment discarded locally",
                    &format!(
                        "Discard cleared the pending selected attachment plus any validation warning locally after consuming it with Option::take() while preserving composer caption/reply text. Repeated Discard or review-row Send after Discard has no pending attachment to submit. No MatrixRequest::SendAttachment, caption-only SendMessage, upload, SDK send-queue cancel, room-state, membership, gateway/runtime/auth, account/profile, or live mutation request was emitted. {lifecycle_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "{} attachment discarded locally. No upload or Matrix media send was started. {lifecycle_metadata}",
                        pending.kind.label()
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            } else {
                let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                    "empty discard",
                    "Attachment",
                    None,
                    None,
                    None,
                    None,
                    false,
                    None,
                    None,
                );
                self.telegram_attachment_local_status =
                    "No selected attachment to discard".to_string();
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "empty-held",
                    "Discard held locally",
                    &format!(
                        "Discard found no pending attachment review state. This empty or repeated Discard stays local, preserves composer caption/reply text, and does not submit SendAttachment, send a caption-only SendMessage, upload media, cancel SDK send-queue work, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live state. {lifecycle_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "No selected attachment to discard. No Matrix media request was emitted. {lifecycle_metadata}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            }
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_options
                        .photo_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_attachment_handoff_confirmation(
                cx,
                AttachmentHandoffKind::Photo,
                room_screen_props.timeline_kind.clone(),
                self.replied_to_for_send(&room_screen_props.timeline_kind)
                    .map(|target| target.event_id),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_options
                        .file_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_attachment_handoff_confirmation(
                cx,
                AttachmentHandoffKind::File,
                room_screen_props.timeline_kind.clone(),
                self.replied_to_for_send(&room_screen_props.timeline_kind)
                    .map(|target| target.event_id),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_options
                        .camera_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_choice(cx, "Camera");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_options
                        .contact_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_choice(cx, "Contact");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .gallery_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Gallery");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .camera_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Camera");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .files_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Files");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .contact_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Contact");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .thumbnail_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Thumbnail");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .share_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Share");
        }

        if self.button(cx, ids!(emoji_button)).clicked(actions) {
            self.show_telegram_emoji_sticker_panel(cx);
            let lifecycle_metadata = self.current_emoji_sticker_lifecycle_metadata_label("opened");
            enqueue_popup_notification(
                format!(
                    "Emoji and sticker picking is staged in the Telegram composer emoji/sticker surface. This local preview does not open a picker, upload stickers, or send Matrix content. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_header
                        .close_emoji_sticker_panel_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_emoji_sticker_local_status =
                "Emoji/sticker picker closed locally".to_string();
            self.telegram_emoji_sticker_last_lifecycle_action = "closed".to_string();
            self.set_telegram_emoji_sticker_panel_visible(cx, false);
            self.update_telegram_emoji_sticker_panel(cx);
            let lifecycle_metadata = self.current_emoji_sticker_lifecycle_metadata_label("closed");
            enqueue_popup_notification(
                format!(
                    "Emoji/sticker picker closed locally. No picker, sticker upload, or Matrix media send was started. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_options
                        .smile_emoji_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_emoji_sticker_choice(cx, "Smile");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_options
                        .thumbs_emoji_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_emoji_sticker_choice(cx, "Thumbs");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_options
                        .heart_emoji_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_emoji_sticker_choice(cx, "Heart");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_options
                        .sticker_emoji_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_emoji_sticker_choice(cx, "Sticker");
        }

        if self.button(cx, ids!(voice_message_button)).clicked(actions) {
            self.show_telegram_voice_message_panel(cx);
            let lifecycle_metadata =
                self.current_voice_lifecycle_metadata_label("opened", "voice panel opened");
            enqueue_popup_notification(
                format!(
                    "Voice messages open a guarded composer surface. Record and Lock stay local; Send can choose an existing desktop audio file for review before Matrix attachment send. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_header
                        .close_voice_message_panel_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_voice_local_status = "Voice preview closed locally".to_string();
            self.update_telegram_voice_message_panel(cx);
            self.set_telegram_voice_message_panel_visible(cx, false);
            let lifecycle_metadata =
                self.current_voice_lifecycle_metadata_label("closed", "close control staged");
            enqueue_popup_notification(
                format!(
                    "Voice preview closed locally. No microphone permission, recording, upload, or Matrix media send was started. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_controls
                        .record_voice_preview_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_choice(cx, "Record");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_controls
                        .lock_voice_preview_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_choice(cx, "Lock");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_controls
                        .cancel_voice_preview_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_voice_local_status = "Voice preview cancelled locally".to_string();
            self.update_telegram_voice_message_panel(cx);
            self.set_telegram_voice_message_panel_visible(cx, false);
            let lifecycle_metadata =
                self.current_voice_lifecycle_metadata_label("cancelled", "cancel control staged");
            enqueue_popup_notification(
                format!(
                    "Voice preview was cancelled locally. No microphone permission, recording, upload, or Matrix media send was started. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_controls
                        .send_voice_preview_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_voice_local_status = "Voice Send confirmation opened locally".to_string();
            self.update_telegram_voice_message_panel(cx);
            let in_reply_to = self.replied_to_event_id();
            self.open_telegram_attachment_handoff_confirmation(
                cx,
                AttachmentHandoffKind::Voice,
                room_screen_props.timeline_kind.clone(),
                in_reply_to,
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .timer_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Timer");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .waveform_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Waveform");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .transcript_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Transcript");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .progress_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Progress");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .codec_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Codec");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .permission_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Permission");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .capture_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Capture");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .encode_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Encode");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .review_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Review");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .upload_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Upload");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .packet_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Packet");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .contract_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Contract");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .taxonomy_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Taxonomy");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .mic_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Mic");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .files_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Files");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .library_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Library");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .retake_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Retake");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .share_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Share");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .play_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Play");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .pause_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Pause");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .scrub_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Scrub");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .speed_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Speed");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .drop_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Drop");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .request_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Request");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .result_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Result");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .error_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Error");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .retry_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Retry");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .source_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Source");
        }

        // Handle the add location button being clicked.
        if self.button(cx, ids!(location_button)).clicked(actions) {
            self.set_telegram_attachment_picker_visible(cx, false);
            self.set_telegram_emoji_sticker_panel_visible(cx, false);
            self.set_telegram_voice_message_panel_visible(cx, false);
            log!("Add location button clicked; requesting current location...");
            if let Err(_e) = init_location_subscriber(cx) {
                error!("Failed to initialize location subscriber");
                enqueue_popup_notification(
                    "Failed to initialize location services.",
                    PopupKind::Error,
                    None,
                );
            }
            self.view
                .location_preview(cx, ids!(location_preview))
                .show(cx);
            self.redraw(cx);
        }

        // Handle the send location button being clicked.
        if self
            .button(cx, ids!(location_preview.send_location_button))
            .clicked(actions)
        {
            let location_preview = self.location_preview(cx, ids!(location_preview));
            if let Some((coords, _system_time_opt)) = location_preview.get_current_data() {
                let timeline_kind = room_screen_props.timeline_kind.clone();
                let replied_to = self.replied_to_for_send(&room_screen_props.timeline_kind);
                #[cfg(feature = "tsp")]
                let sign_with_tsp = self.is_tsp_signing_enabled(cx);
                // Location confirmation evidence: opening/canceling this guard keeps the
                // location message unsent until the accept handler emits LocationSendConfirmed.
                let content = ConfirmationModalContent {
                    title_text: "Send Location".into(),
                    body_text: "Send your current location to this room? The existing Matrix location message path will only be requested after this confirmation.".into(),
                    accept_button_text: Some("Send Location".into()),
                    cancel_button_text: Some("Cancel".into()),
                    on_accept_clicked: Some(Box::new(move |cx| {
                        cx.action(RoomInputBarAction::LocationSendConfirmed {
                            timeline_kind,
                            coords,
                            replied_to,
                            #[cfg(feature = "tsp")]
                            sign_with_tsp,
                        });
                    })),
                    on_cancel_clicked: Some(Box::new(|_cx| {
                        enqueue_popup_notification(
                            "Location send canceled. No Matrix location message was sent.",
                            PopupKind::Info,
                            Some(3.0),
                        );
                    })),
                };
                enqueue_popup_notification(
                    "Location send confirmation opened. No Matrix location message was sent before confirmation.",
                    PopupKind::Info,
                    Some(4.0),
                );
                cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                    content,
                ))));
            }
        }

        // Handle the send message button being clicked, or a `Returned` action
        // from the message text input. The text input only emits `Returned`
        // for the key combination chosen by the user in App Settings (plus
        // Cmd/Ctrl+Enter, which always submits).
        if self.button(cx, ids!(send_message_button)).clicked(actions)
            || text_input.returned(actions).is_some()
        {
            let pending_attachment_summary = self
                .telegram_pending_attachment_send
                .as_ref()
                .map(|pending| (pending.kind.label(), pending.filename.clone()));
            if let Some((label, filename)) = pending_attachment_summary {
                self.telegram_attachment_local_status = format!(
                    "{label} attachment still waiting for review: {filename}. Use attachment review Send."
                );
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "attachment review required",
                    "Main Send held locally",
                    "A selected attachment is pending review. Main composer Send/Enter preserved the pending attachment plus composer caption/reply preview, did not send the caption as plain text, did not submit SendAttachment, and did not clear the pending attachment. Use the attachment review row Send to submit it.",
                );
                enqueue_popup_notification(
                    format!(
                        "{label} attachment is waiting for review: {filename}. Use the attachment review Send button to submit it."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                return;
            }

            let entered_text = mentionable_text_input.text().trim().to_string();
            if !entered_text.is_empty() {
                if let Some(plan) = plan_hepta_composer_command(&entered_text, current_time_ms()) {
                    let preview = plan.to_bridge_input();
                    enqueue_popup_notification(
                        format!(
                            "Hepta dry-run staged locally: {}\nPreview event: m.hepta.{} / {}\n\nHepta native execution adapters are pending; no external mutation was sent.",
                            plan.operator_summary(),
                            preview.event_kind,
                            preview.id,
                        ),
                        PopupKind::Warning,
                        Some(6.0),
                    );
                    mentionable_text_input.set_text(cx, "");
                    self.update_hepta_command_preview(cx, "");
                    self.set_telegram_attachment_picker_visible(cx, false);
                    self.set_telegram_emoji_sticker_panel_visible(cx, false);
                    self.set_telegram_voice_message_panel_visible(cx, false);
                    self.enable_send_message_button(cx, false);
                    return;
                }

                let message = mentionable_text_input.create_message_with_mentions(&entered_text);
                let mention_payload_metadata = mentionable_text_input.send_payload_metadata_label(
                    &entered_text,
                    room_screen_props
                        .room_members
                        .as_ref()
                        .map(|members| members.as_slice()),
                );
                let replied_to = self
                    .replying_to
                    .take()
                    .and_then(|(event_tl_item, _emb)| {
                        event_tl_item.event_id().map(|event_id| {
                            let enforce_thread = if room_screen_props
                                .timeline_kind
                                .thread_root_event_id()
                                .is_some()
                            {
                                EnforceThread::Threaded(ReplyWithinThread::Yes)
                            } else {
                                EnforceThread::MaybeThreaded
                            };
                            Reply {
                                event_id: event_id.to_owned(),
                                enforce_thread,
                                add_mentions: AddMentions::Yes,
                            }
                        })
                    })
                    .or_else(|| {
                        room_screen_props.timeline_kind.thread_root_event_id().map(
                            |thread_root_event_id| Reply {
                                event_id: thread_root_event_id.clone(),
                                enforce_thread: EnforceThread::Threaded(ReplyWithinThread::No),
                                add_mentions: AddMentions::No,
                            },
                        )
                    });
                submit_async_request(MatrixRequest::SendMessage {
                    timeline_kind: room_screen_props.timeline_kind.clone(),
                    message,
                    replied_to,
                    #[cfg(feature = "tsp")]
                    sign_with_tsp: self.is_tsp_signing_enabled(cx),
                });

                self.clear_replying_to(cx);
                mentionable_text_input.set_text(cx, "");
                self.update_hepta_command_preview(cx, "");
                self.set_telegram_attachment_picker_visible(cx, false);
                self.set_telegram_emoji_sticker_panel_visible(cx, false);
                self.set_telegram_voice_message_panel_visible(cx, false);
                self.telegram_pending_attachment_send = None;
                self.telegram_attachment_send_retry_attempt = None;
                self.enable_send_message_button(cx, false);
                self.set_message_send_operation_status(
                    cx,
                    "text submitted",
                    "Text SendMessage submitted",
                    &format!(
                        "Existing MatrixRequest::SendMessage was submitted for this text/reply/thread send. Queued/progress/failure labels plus Retry/Cancel controls are local evidence only; no retry or cancel request was emitted from the evidence strip. {mention_payload_metadata}"
                    ),
                );
            }
        }

        // If the user starts/stops typing in the message input box,
        // send a typing notice to the room and update the send_message_button state.
        let is_text_input_empty = if let Some(new_text) = text_input.changed(actions) {
            self.update_hepta_command_preview(cx, &new_text);
            mentionable_text_input.update_cached_member_suggestions(
                cx,
                room_screen_props
                    .room_members
                    .as_ref()
                    .map(|members| members.as_slice()),
            );
            if let Some(pending) = self.telegram_pending_attachment_send.as_mut() {
                pending.caption_preview = summarize_attachment_caption(&new_text);
                self.update_telegram_attachment_picker(cx);
            }
            let is_empty = new_text.is_empty();
            if !looks_like_hepta_composer_command(&new_text) {
                submit_async_request(MatrixRequest::SendTypingNotice {
                    room_id: room_screen_props.timeline_kind.room_id().clone(),
                    typing: !is_empty,
                });
                self.set_typing_notice_status(
                    cx,
                    if is_empty {
                        "Typing notice cleared"
                    } else {
                        "Typing notice submitted"
                    },
                    "Existing MatrixRequest::SendTypingNotice was submitted for plain composer text. No message send, room-state, retry, or cancel request was emitted from the typing evidence strip.",
                );
            } else {
                self.set_typing_notice_status(
                    cx,
                    "Hepta command preview suppressed Matrix typing notice",
                    "Reserved Hepta command previews stay local and do not submit MatrixRequest::SendTypingNotice. No message send, room-state, retry, or cancel request was emitted from the typing evidence strip.",
                );
            }
            is_empty
        } else {
            text_input.text().is_empty()
        };
        self.enable_send_message_button(cx, !is_text_input_empty);

        // Handle the user pressing the up arrow in an empty message input box
        // to edit their latest sent message.
        if is_text_input_empty {
            if let Some(KeyEvent {
                key_code: KeyCode::ArrowUp,
                modifiers:
                    KeyModifiers {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
                ..
            }) = text_input.key_down_unhandled(actions)
            {
                cx.widget_action(
                    room_screen_props.room_screen_widget_uid,
                    MessageAction::EditLatest,
                );
            }
        }

        // When the hide animation fully completes, restore the replying preview.
        if self
            .view
            .editing_pane(cx, ids!(editing_pane))
            .was_hidden(actions)
        {
            self.on_editing_pane_hidden(cx);
        }
    }

    fn replied_to_for_send(&self, timeline_kind: &TimelineKind) -> Option<RoomInputBarReplyTarget> {
        self.replying_to
            .as_ref()
            .and_then(|(event_tl_item, _emb)| {
                event_tl_item.event_id().map(|event_id| {
                    let enforce_thread = if timeline_kind.thread_root_event_id().is_some() {
                        RoomInputBarReplyThread::ThreadedYes
                    } else {
                        RoomInputBarReplyThread::MaybeThreaded
                    };
                    RoomInputBarReplyTarget {
                        event_id: event_id.to_owned(),
                        enforce_thread,
                        add_mentions: true,
                    }
                })
            })
            .or_else(|| {
                timeline_kind
                    .thread_root_event_id()
                    .map(|thread_root_event_id| RoomInputBarReplyTarget {
                        event_id: thread_root_event_id.clone(),
                        enforce_thread: RoomInputBarReplyThread::ThreadedNo,
                        add_mentions: false,
                    })
            })
    }

    /// Shows a preview of the given event that the user is currently replying to
    /// above the message input bar.
    ///
    /// If `grab_key_focus` is true, this will also automatically focus the keyboard
    /// on the message input box so that the user can immediately start typing their reply.
    fn show_replying_to(
        &mut self,
        cx: &mut Cx,
        replying_to: (EventTimelineItem, EmbeddedEvent),
        timeline_kind: &TimelineKind,
        grab_key_focus: bool,
    ) {
        // When the user clicks the reply button next to a message, we need to:
        // 1. Populate and show the ReplyingPreview, of course.
        let replying_preview = self.view(cx, ids!(replying_preview));
        let (replying_preview_username, _) = replying_preview
            .avatar(cx, ids!(reply_preview_content.reply_preview_avatar))
            .set_avatar_and_get_username(
                cx,
                timeline_kind,
                replying_to.0.sender(),
                Some(replying_to.0.sender_profile()),
                replying_to.0.event_id(),
                true,
            );

        replying_preview
            .label(cx, ids!(reply_preview_content.reply_preview_username))
            .set_text(cx, replying_preview_username.as_str());

        populate_preview_of_timeline_item(
            cx,
            &replying_preview.html_or_plaintext(cx, ids!(reply_preview_content.reply_preview_body)),
            replying_to.0.content(),
            replying_to.0.sender(),
            &replying_preview_username,
        );

        replying_preview.set_visible(cx, true);
        self.replying_to = Some(replying_to);

        // 2. Hide other views that are irrelevant to a reply, e.g.,
        //    the `EditingPane` would improperly cover up the ReplyPreview.
        self.editing_pane(cx, ids!(editing_pane))
            .force_reset_hide(cx);
        self.on_editing_pane_hidden(cx);
        // 3. Automatically focus the keyboard on the message input box
        //    so that the user can immediately start typing their reply
        //    without having to manually click on the message input box.
        if grab_key_focus {
            self.text_input(cx, ids!(input_bar.mentionable_text_input.text_input))
                .set_key_focus(cx);
        }
        self.button(cx, ids!(cancel_reply_button)).reset_hover(cx);
        self.redraw(cx);
    }

    /// Clears (and makes invisible) the preview of the message
    /// that the user is currently replying to.
    fn clear_replying_to(&mut self, cx: &mut Cx) {
        self.view(cx, ids!(replying_preview)).set_visible(cx, false);
        self.replying_to = None;
    }

    /// Shows the editing pane to allow the user to edit the given event.
    fn show_editing_pane(
        &mut self,
        cx: &mut Cx,
        behavior: ShowEditingPaneBehavior,
        timeline_kind: TimelineKind,
    ) {
        // Cache the input_bar's natural height before the animation shrinks it.
        let input_bar_height = self.view.view(cx, ids!(input_bar)).area().rect(cx).size.y;
        if input_bar_height > 0.0 {
            self.input_bar_natural_height = input_bar_height;
        }

        // Hide the replying preview and location preview while the editing
        // pane is shown. The input_bar is not hidden; instead it is slid out
        // of view in draw_walk using the EditingPane's slide value.
        self.set_telegram_attachment_picker_visible(cx, false);
        self.telegram_pending_attachment_send = None;
        self.set_telegram_emoji_sticker_panel_visible(cx, false);
        self.set_telegram_voice_message_panel_visible(cx, false);
        let replying_preview = self.view.view(cx, ids!(replying_preview));
        self.was_replying_preview_visible = replying_preview.visible();
        replying_preview.set_visible(cx, false);
        self.view
            .location_preview(cx, ids!(location_preview))
            .clear();

        let editing_pane = self.view.editing_pane(cx, ids!(editing_pane));
        match behavior {
            ShowEditingPaneBehavior::ShowNew { event_tl_item } => {
                editing_pane.show(cx, event_tl_item, timeline_kind);
            }
            ShowEditingPaneBehavior::RestoreExisting { editing_pane_state } => {
                editing_pane.restore_state(cx, editing_pane_state, timeline_kind);
            }
        };

        self.redraw(cx);
    }

    /// This should be invoked after the EditingPane has been fully hidden.
    fn on_editing_pane_hidden(&mut self, cx: &mut Cx) {
        // Restore the replying_preview.
        if self.was_replying_preview_visible && self.replying_to.is_some() {
            self.view
                .view(cx, ids!(replying_preview))
                .set_visible(cx, true);
        }
        self.redraw(cx);
        // We don't need to do anything with the editing pane itself here,
        // because it has already been hidden by the time this function gets called.
    }

    fn update_telegram_attachment_picker(&mut self, cx: &mut Cx) {
        let status = if let Some(pending) = &self.telegram_pending_attachment_send {
            let validation_context = pending
                .validation_error
                .as_deref()
                .map(|reason| format!("; validation warning: {reason}"))
                .unwrap_or_default();
            let taxonomy_status = if pending.validation_error.is_some() {
                "validation-held"
            } else {
                "review-pending"
            };
            format!(
                "{taxonomy_status}: {} selected for review: {} ({}, {}){}",
                pending.kind.label(),
                pending.filename,
                pending.mime_type,
                format_attachment_file_size(pending.file_size_bytes),
                validation_context
            )
        } else if self.telegram_attachment_local_status.trim().is_empty() {
            "Choose Photo or File to confirm desktop picker + local review before Matrix send; Camera and Contact stage local previews"
                .to_string()
        } else {
            self.telegram_attachment_local_status.clone()
        };
        let header_status = if self.telegram_pending_attachment_send.is_some() {
            "review"
        } else {
            "confirm + review"
        };
        self.view
            .label(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_header
                        .attachment_status
                ),
            )
            .set_text(cx, header_status);
        self.view
            .label(cx, ids!(telegram_attachment_picker.attachment_summary))
            .set_text(
                cx,
                &format!(
                    "{status}. Send submits selected attachments; Discard, Close, picker cancel, and unsupported picker states send no upload or Matrix media request."
                ),
            );
        self.view
            .label(cx, ids!(telegram_attachment_picker.attachment_option_evidence))
            .set_text(
                cx,
                    "Photo and File confirm before desktop rfd picker; Voice Send confirms before a desktop audio picker. Selected files stage local review with filename, MIME, extension, size, caption preview, and reply context; selected Photo image files also show dimensions status from lightweight PNG/JPEG/GIF/BMP/WebP headers when available, and selected audio also shows duration status, codec/container status, and bounded WAV PCM waveform peaks when available. Attachment status taxonomy stays stable: review-pending, review-replaced, review-preserved, validation-held, handoff-submitted, queued-only, failure-copy, retry-confirmation-open, retry-confirmed, empty-held, discarded-local, closed-local, retry-local, and cancel-local. Caption preview live-updates from composer text. Main Send, picker cancel, Discard, Close, and empty review Send preserve composer caption/reply text. Choosing another file replaces only local pending review state, clears any local validation warning, and review Send consumes pending once before MatrixRequest::SendAttachment via Timeline::send_attachment().use_send_queue(). Review-row Send revalidates the selected path before submit; unreadable, non-file, or empty-file paths stay local with validation evidence. Replace, Discard, and Close recover from validation warnings locally; worker-failure Retry confirms before resubmitting only the cached last handoff, while Cancel clears local retry cache only and never cancels SDK queue work. MIME fallback to application/octet-stream, size unavailable, image dimensions unavailable, audio duration/codec/waveform unavailable states stay visible metadata before Send. Review-row Send is the only attachment path that consumes caption/reply context; empty or duplicate review Send stays local. Discard and Close are idempotent local cleanup; repeated Discard/Close and review Send after cleanup stay local. Camera, Contact, and Share stay local with no permissions, capture, share sheet, thumbnail decode, full image decode, contacts or shared-media read, payload, upload, or send.",
            );
        self.view
            .label(
                cx,
                ids!(telegram_attachment_picker.attachment_review_compact_fit),
            )
            .set_text(cx, ATTACHMENT_REVIEW_ROW_COMPACT_FIT_LABEL);
        self.view
            .label(
                cx,
                ids!(telegram_attachment_picker.attachment_review_action_density),
            )
            .set_text(cx, ATTACHMENT_MOBILE_ACTION_DENSITY_LABEL);
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let mobile_picker_controls_label = attachment_mobile_picker_controls_label(
            "Status",
            pending_review.as_deref(),
            self.telegram_attachment_local_status.as_str(),
        );
        self.view
            .label(
                cx,
                ids!(telegram_attachment_picker.attachment_mobile_picker_controls_label),
            )
            .set_text(cx, &mobile_picker_controls_label);
        if let Some(pending) = &self.telegram_pending_attachment_send {
            let reply_context = if pending.in_reply_to.is_some() {
                "reply: included"
            } else {
                "reply: none"
            };
            let validation_context = pending
                .validation_error
                .as_deref()
                .map(|reason| {
                    format!("validation: {reason}; recover with Replace, Discard, or Close")
                })
                .unwrap_or_else(|| "validation: ready".to_string());
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_title
                    ),
                )
                .set_text(cx, &format!("Review {} before Send", pending.kind.label()));
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_filename
                    ),
                )
                .set_text(cx, &pending.filename);
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_metadata
                    ),
                )
                .set_text(
                    cx,
                    &pending_attachment_image_metadata_label(pending)
                        .or_else(|| pending_attachment_audio_metadata_label(pending))
                        .unwrap_or_else(|| {
                            format!(
                                "MIME: {} | ext: {} | size: {}",
                                pending.mime_type,
                                pending.file_extension,
                                format_attachment_file_size(pending.file_size_bytes)
                            )
                        }),
                );
            let local_metadata_label = match pending.kind {
                AttachmentHandoffKind::Photo => ATTACHMENT_SELECTED_IMAGE_METADATA_LABEL,
                AttachmentHandoffKind::Voice => VOICE_MESSAGE_SELECTED_AUDIO_METADATA_LABEL,
                AttachmentHandoffKind::File => {
                    "Selected-file metadata stays local until review Send."
                }
            };
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_context
                    ),
                )
                .set_text(
                    cx,
                    &format!(
                        "{} | {} | {} | local preview only until Send. {}",
                        pending.caption_preview,
                        reply_context,
                        validation_context,
                        local_metadata_label
                    ),
                );
        } else {
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_title
                    ),
                )
                .set_text(cx, "No selected attachment");
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_filename
                    ),
                )
                .set_text(
                    cx,
                    "Pick Photo or File to review filename and MIME before send.",
                );
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_metadata
                    ),
                )
                .set_text(
                    cx,
                    "Local metadata appears after picker selection; no upload or media decode.",
                );
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_context
                    ),
                )
                .set_text(
                    cx,
                    "Caption/reply stays in the composer until a pending review Send consumes it.",
                );
        }
    }

    fn set_telegram_attachment_picker_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_attachment_picker_visible == visible {
            return;
        }
        self.telegram_attachment_picker_visible = visible;
        self.view
            .view(cx, ids!(telegram_attachment_picker))
            .set_visible(cx, visible);
        self.redraw(cx);
    }

    fn show_telegram_attachment_picker(&mut self, cx: &mut Cx) {
        self.set_telegram_emoji_sticker_panel_visible(cx, false);
        self.set_telegram_voice_message_panel_visible(cx, false);
        self.update_telegram_attachment_picker(cx);
        self.set_telegram_attachment_picker_visible(cx, true);
    }

    fn stage_telegram_attachment_choice(&mut self, cx: &mut Cx, label: &str) {
        self.telegram_pending_attachment_send = None;
        self.telegram_attachment_send_retry_attempt = None;
        self.telegram_attachment_local_status = if matches!(label, "Camera" | "Contact") {
            format!("{label} attachment placeholder staged locally")
        } else {
            format!("{label} attachment preview staged locally")
        };
        self.update_telegram_attachment_picker(cx);
        self.set_telegram_attachment_picker_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "{label} attachment was staged in the local Telegram composer preview. No permission prompt, native picker, capture/contact read, payload, upload, or Matrix send was started."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_attachment_mobile_picker_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let label = attachment_mobile_picker_controls_label(
            control,
            pending_review.as_deref(),
            self.telegram_attachment_local_status.as_str(),
        );
        self.telegram_attachment_local_status =
            format!("Mobile attachment {control} control stayed local");
        self.update_telegram_attachment_picker(cx);
        self.set_telegram_attachment_picker_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Attachment mobile {control} stayed local. No camera/photos/files/contacts permission, picker, capture, contact or shared-media read, thumbnail decode, system share sheet, share extension, payload, upload, SendAttachment, SendMessage, SDK queue mutation, gateway/runtime/auth, or live mutation was emitted. {label}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn open_telegram_attachment_handoff_confirmation(
        &mut self,
        cx: &mut Cx,
        kind: AttachmentHandoffKind,
        timeline_kind: TimelineKind,
        in_reply_to: Option<OwnedEventId>,
    ) {
        let label = kind.label();
        self.telegram_attachment_local_status =
            format!("{label} attachment send waiting for confirmation");
        self.update_telegram_attachment_picker(cx);
        self.set_telegram_attachment_picker_visible(cx, true);
        let voice_lifecycle_note = (kind == AttachmentHandoffKind::Voice)
            .then(|| {
                format!(
                    " {}",
                    self.current_voice_lifecycle_metadata_label(
                        "send confirmation opened",
                        "confirmation opened before desktop audio picker"
                    )
                )
            })
            .unwrap_or_default();
        let content = ConfirmationModalContent {
            title_text: format!("Send {label} attachment").into(),
            body_text: format!("{label} attachments open the native desktop picker after this confirmation. Choosing a file stages local review first; only the review row Send button submits MatrixRequest::SendAttachment through the Matrix attachment send queue. Cancel sends no upload or Matrix media request.{voice_lifecycle_note}").into(),
            accept_button_text: Some("Choose File".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                cx.action(RoomInputBarAction::AttachmentHandoffConfirmed {
                    kind,
                    timeline_kind: timeline_kind.clone(),
                    in_reply_to: in_reply_to.clone(),
                });
            })),
            on_cancel_clicked: Some(Box::new(move |cx| {
                cx.action(RoomInputBarAction::AttachmentHandoffCanceled { kind });
                enqueue_popup_notification(
                    format!(
                        "{label} attachment send canceled before picker. No upload or Matrix media send was started."
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        enqueue_popup_notification(
            format!(
                "{label} attachment send confirmation opened. No native picker, upload, or Matrix media send was started before confirmation.{voice_lifecycle_note}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn current_emoji_sticker_lifecycle_metadata_label(&self, action: &str) -> String {
        let resolved_action = if action.trim().is_empty() {
            self.telegram_emoji_sticker_last_lifecycle_action.as_str()
        } else {
            action
        };
        emoji_sticker_lifecycle_metadata_label(
            resolved_action,
            self.telegram_emoji_sticker_panel_visible,
            self.telegram_emoji_sticker_last_choice.as_deref(),
            self.telegram_emoji_sticker_stage_count,
            Some(self.telegram_emoji_sticker_local_status.as_str()),
        )
    }

    fn update_telegram_emoji_sticker_panel(&mut self, cx: &mut Cx) {
        let status = if self.telegram_emoji_sticker_local_status.trim().is_empty() {
            "Choose Smile, Thumbs, Heart, or Sticker to stage a local-only emoji/sticker preview"
        } else {
            self.telegram_emoji_sticker_local_status.as_str()
        };
        self.view
            .label(cx, ids!(telegram_emoji_sticker_panel.emoji_summary))
            .set_text(
                cx,
                &format!("{status}. {EMOJI_STICKER_SEND_LOCAL_BOUNDARY_LABEL}"),
            );
        let lifecycle_metadata =
            self.current_emoji_sticker_lifecycle_metadata_label("panel update");
        self.view
            .label(
                cx,
                ids!(telegram_emoji_sticker_panel.emoji_lifecycle_metadata),
            )
            .set_text(cx, &lifecycle_metadata);
    }

    fn set_telegram_emoji_sticker_panel_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_emoji_sticker_panel_visible == visible {
            return;
        }
        self.telegram_emoji_sticker_panel_visible = visible;
        self.view
            .view(cx, ids!(telegram_emoji_sticker_panel))
            .set_visible(cx, visible);
        self.redraw(cx);
    }

    fn show_telegram_emoji_sticker_panel(&mut self, cx: &mut Cx) {
        self.set_telegram_attachment_picker_visible(cx, false);
        self.set_telegram_voice_message_panel_visible(cx, false);
        self.telegram_emoji_sticker_last_lifecycle_action =
            if self.telegram_emoji_sticker_stage_count == 0 {
                "opened"
            } else {
                "reopened"
            }
            .to_string();
        self.set_telegram_emoji_sticker_panel_visible(cx, true);
        self.update_telegram_emoji_sticker_panel(cx);
    }

    fn stage_telegram_emoji_sticker_choice(&mut self, cx: &mut Cx, label: &str) {
        self.telegram_emoji_sticker_last_choice = Some(label.to_string());
        self.telegram_emoji_sticker_stage_count =
            self.telegram_emoji_sticker_stage_count.saturating_add(1);
        self.telegram_emoji_sticker_last_lifecycle_action = format!("staged {label}");
        self.telegram_emoji_sticker_local_status =
            format!("{label} emoji/sticker preview staged locally");
        self.set_telegram_emoji_sticker_panel_visible(cx, true);
        self.update_telegram_emoji_sticker_panel(cx);
        let lifecycle_metadata = self.current_emoji_sticker_lifecycle_metadata_label("");
        enqueue_popup_notification(
            format!(
                "{label} emoji/sticker preview was staged in the local Telegram composer preview. {EMOJI_STICKER_SEND_LOCAL_BOUNDARY_LABEL} {lifecycle_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn current_voice_lifecycle_metadata_label(&self, action: &str, picker_state: &str) -> String {
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        voice_message_lifecycle_metadata_label(
            action,
            self.telegram_voice_message_panel_visible,
            Some(self.telegram_voice_local_status.as_str()),
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            pending_voice
                .map(|pending| pending.in_reply_to.is_some())
                .unwrap_or_else(|| self.replying_to.is_some()),
            picker_state,
        )
    }

    fn update_telegram_voice_message_panel(&mut self, cx: &mut Cx) {
        let status = if self.telegram_voice_local_status.trim().is_empty() {
            "Use Send to choose a desktop audio file for review; Record and Lock stay local"
        } else {
            self.telegram_voice_local_status.as_str()
        };
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let lifecycle_metadata =
            self.current_voice_lifecycle_metadata_label("panel update", "status repaint only");
        let recorder_status_metadata = voice_message_recorder_status_controls_label(
            self.telegram_voice_recorder_last_control.as_deref(),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            pending_voice.and_then(|pending| pending.audio_waveform_codec_label.as_deref()),
        );
        let capture_lifecycle_metadata = voice_message_capture_lifecycle_controls_label(
            self.telegram_voice_capture_lifecycle_last_control
                .as_deref(),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
        );
        let mobile_picker_metadata = voice_message_mobile_picker_controls_label(
            self.telegram_voice_mobile_picker_last_control.as_deref(),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            status,
        );
        let review_playback_metadata = voice_message_review_playback_controls_label(
            self.telegram_voice_review_playback_last_control.as_deref(),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            status,
        );
        let voice_preflight_control = voice_message_send_preflight_control_from_status(
            self.telegram_voice_local_status.as_str(),
        );
        let voice_preflight_source = if self.telegram_voice_send_preflight_detail.trim().is_empty()
        {
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
        } else {
            self.telegram_voice_send_preflight_detail.as_str()
        };
        let voice_preflight_detail = voice_message_send_preflight_detail_controls_label(
            voice_preflight_control,
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            self.telegram_attachment_send_retry_attempt
                .as_ref()
                .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                .unwrap_or(false),
            status,
            self.telegram_attachment_send_cached_error.as_deref(),
            voice_preflight_source,
        );
        self.telegram_voice_send_preflight_detail = voice_preflight_detail.clone();
        self.view
            .label(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_preview
                        .voice_preview_mode
                ),
            )
            .set_text(cx, &recorder_status_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_recorder_status_metadata),
            )
            .set_text(cx, &recorder_status_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_capture_lifecycle_metadata),
            )
            .set_text(cx, &capture_lifecycle_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_mobile_picker_metadata),
            )
            .set_text(cx, &mobile_picker_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_review_playback_metadata),
            )
            .set_text(cx, &review_playback_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_send_preflight_detail_metadata),
            )
            .set_text(cx, &voice_preflight_detail);
        self.view
            .label(cx, ids!(telegram_voice_message_panel.voice_summary))
            .set_text(
                cx,
                &format!(
                    "{status}. {VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_LABEL} {} {capture_lifecycle_metadata} {mobile_picker_metadata} {review_playback_metadata} {lifecycle_metadata}",
                    voice_message_recorder_waveform_codec_boundary_label(
                        "panel update",
                        self.telegram_voice_message_panel_visible,
                        None,
                    )
                ),
            );
        self.view
            .label(cx, ids!(telegram_voice_message_panel.voice_option_evidence))
            .set_text(
                cx,
                &format!(
                    "Voice Send reuses the confirmed desktop file picker and attachment review row for local audio files. Selected audio review shows filename, MIME, extension, size, duration, codec/container status, and bounded WAV waveform peaks before SendAttachment. Confirmation cancel repaints only local voice/picker status. Play can open the pending desktop audio review through the system opener; Record, Lock, Cancel, Permission, Capture, Encode, Review, Upload, Packet, Contract, Taxonomy, Mic, Files, Library, Retake, Share, Pause, Scrub, Speed, and Close stay local; Packet records recorder lifecycle acceptance criteria, Contract maps typed recorder/upload contracts, Taxonomy records recorder result slots, and Drop only clears pending voice review state. No microphone permission, mobile picker, share sheet, recorder, inline player, recorder waveform capture, encoder, codec conversion, transcription, upload progress, text fallback, room-state, membership, or live mutation is requested. {VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_LABEL} {VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL}"
                ),
            );
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_send_blocked_evidence),
            )
            .set_text(
                cx,
                &format!(
                    "{VOICE_MESSAGE_SEND_LOCAL_BLOCKED_LABEL} {VOICE_MESSAGE_SELECTED_AUDIO_METADATA_LABEL} {VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_LABEL} {VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL} {VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_LABEL} {VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_LABEL} {VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_LABEL} {VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL} {VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL} {VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL} {VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_LABEL} {VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_LABEL} {VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL} {VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_LABEL} {VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
                ),
            );
    }

    fn set_telegram_voice_message_panel_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_voice_message_panel_visible == visible {
            return;
        }
        self.telegram_voice_message_panel_visible = visible;
        self.view
            .view(cx, ids!(telegram_voice_message_panel))
            .set_visible(cx, visible);
        self.redraw(cx);
    }

    fn show_telegram_voice_message_panel(&mut self, cx: &mut Cx) {
        self.set_telegram_attachment_picker_visible(cx, false);
        self.set_telegram_emoji_sticker_panel_visible(cx, false);
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
    }

    fn stage_telegram_voice_choice(&mut self, cx: &mut Cx, label: &str) {
        self.telegram_voice_local_status = match label {
            "Record" => "Record control stayed local without microphone permission".to_string(),
            "Lock" => "Lock control stayed local without starting hands-free recording".to_string(),
            "Send" => "Send opens a confirmed desktop audio-file picker; mic capture stays local"
                .to_string(),
            _ => format!("{label} control staged a local voice preview"),
        };
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        let lifecycle_metadata =
            self.current_voice_lifecycle_metadata_label(label, "local control staged");
        enqueue_popup_notification(
            format!(
                "{label} voice control was staged in the local Telegram composer preview. {VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_LABEL} {} {lifecycle_metadata}",
                voice_message_recorder_waveform_codec_boundary_label(
                    label,
                    self.telegram_voice_message_panel_visible,
                    None,
                )
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_recorder_status_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        self.telegram_voice_recorder_last_control = Some(control.to_string());
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let pending_voice_filename = pending_voice.map(|pending| pending.filename.clone());
        let pending_voice_duration =
            pending_voice.and_then(|pending| pending.audio_duration_label.clone());
        let pending_voice_waveform_codec =
            pending_voice.and_then(|pending| pending.audio_waveform_codec_label.clone());
        self.telegram_voice_local_status = if matches!(control, "Waveform" | "Codec") {
            if let Some(filename) = pending_voice_filename.as_deref() {
                let analysis = pending_voice_waveform_codec
                    .as_deref()
                    .unwrap_or("selected-audio waveform/codec unavailable");
                format!("{control} selected-audio analysis stayed local: {filename}; {analysis}")
            } else {
                format!(
                    "{control} recorder status stayed local; no pending selected audio analysis"
                )
            }
        } else {
            format!(
                "{control} recorder status stayed local without mic permission, recording, transcription, upload progress, or codec work"
            )
        };
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        let recorder_status_metadata = voice_message_recorder_status_controls_label(
            Some(control),
            self.telegram_voice_message_panel_visible,
            pending_voice_filename.as_deref(),
            pending_voice_duration.as_deref(),
            pending_voice_waveform_codec.as_deref(),
        );
        enqueue_popup_notification(
            format!(
                "{control} recorder status control stayed local. Waveform/Codec can read only capped bytes from the already selected desktop audio review when present; no microphone permission, audio session, recorder, recorder waveform sampling, transcription service, upload progress subscription, SendAttachment, gateway/runtime/auth, or live mutation was emitted. {recorder_status_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_capture_lifecycle_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        self.telegram_voice_capture_lifecycle_last_control = Some(control.to_string());
        self.telegram_voice_local_status = format!(
            "{control} capture lifecycle stayed local without mic permission, recording, encoding, upload, or SendAttachment"
        );
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let capture_lifecycle_metadata = if control.eq_ignore_ascii_case("Packet") {
            let source_copy = if self.telegram_voice_send_preflight_detail.trim().is_empty() {
                VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            } else {
                self.telegram_voice_send_preflight_detail.as_str()
            };
            voice_message_recorder_lifecycle_drilldown_packet_label(
                self.telegram_voice_message_panel_visible,
                pending_voice.map(|pending| pending.filename.as_str()),
                pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
                self.telegram_attachment_send_retry_attempt
                    .as_ref()
                    .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                    .unwrap_or(false),
                self.telegram_voice_local_status.as_str(),
                self.telegram_attachment_send_cached_error.as_deref(),
                source_copy,
            )
        } else if control.eq_ignore_ascii_case("Contract") {
            let source_copy = if self.telegram_voice_send_preflight_detail.trim().is_empty() {
                VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            } else {
                self.telegram_voice_send_preflight_detail.as_str()
            };
            voice_message_recorder_typed_contract_packet_label(
                self.telegram_voice_message_panel_visible,
                pending_voice.map(|pending| pending.filename.as_str()),
                pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
                self.telegram_attachment_send_retry_attempt
                    .as_ref()
                    .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                    .unwrap_or(false),
                self.telegram_voice_local_status.as_str(),
                self.telegram_attachment_send_cached_error.as_deref(),
                source_copy,
            )
        } else if control.eq_ignore_ascii_case("Taxonomy") {
            let source_copy = if self.telegram_voice_send_preflight_detail.trim().is_empty() {
                VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            } else {
                self.telegram_voice_send_preflight_detail.as_str()
            };
            voice_message_recorder_result_taxonomy_packet_label(
                self.telegram_voice_message_panel_visible,
                pending_voice.map(|pending| pending.filename.as_str()),
                pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
                self.telegram_attachment_send_retry_attempt
                    .as_ref()
                    .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                    .unwrap_or(false),
                self.telegram_voice_local_status.as_str(),
                self.telegram_attachment_send_cached_error.as_deref(),
                source_copy,
            )
        } else {
            voice_message_capture_lifecycle_controls_label(
                Some(control),
                self.telegram_voice_message_panel_visible,
                pending_voice.map(|pending| pending.filename.as_str()),
                pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            )
        };
        enqueue_popup_notification(
            format!(
                "Voice {control} capture lifecycle stayed local. No microphone permission, audio session, platform recorder, captured file, waveform sampling, codec conversion, upload progress subscription, SendAttachment, SendMessage fallback, gateway/runtime/auth, or live mutation was emitted. {capture_lifecycle_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_mobile_picker_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let latest_status = if self.telegram_voice_local_status.trim().is_empty() {
            "voice mobile picker local"
        } else {
            self.telegram_voice_local_status.as_str()
        };
        let mobile_picker_metadata = voice_message_mobile_picker_controls_label(
            Some(control),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            latest_status,
        );
        self.telegram_voice_mobile_picker_last_control = Some(control.to_string());
        self.telegram_voice_local_status = format!(
            "{control} mobile voice picker stayed local without mobile permission, picker, capture, share sheet, upload, or SendAttachment"
        );
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Voice mobile {control} picker control stayed local. No mobile microphone permission, document picker, library picker, capture session, retake deletion, share sheet, SendAttachment, SendMessage fallback, gateway/runtime/auth, or live mutation was emitted. {mobile_picker_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_review_playback_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        if control.eq_ignore_ascii_case("Drop") {
            self.drop_telegram_voice_review_audio(cx);
            return;
        }
        if control.eq_ignore_ascii_case("Play") {
            self.play_telegram_voice_review_audio(cx);
            return;
        }
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let latest_status = if self.telegram_voice_local_status.trim().is_empty() {
            "voice review playback local"
        } else {
            self.telegram_voice_local_status.as_str()
        };
        let review_playback_metadata = voice_message_review_playback_controls_label(
            Some(control),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            latest_status,
        );
        self.telegram_voice_review_playback_last_control = Some(control.to_string());
        self.telegram_voice_local_status = format!(
            "{control} review playback stayed local without player, decode, scrubber, deletion, upload, or SendAttachment"
        );
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Voice review {control} stayed local. No inline audio player, media decode, waveform sampling, playback subscription, speed transform, scrubber timeline, local file deletion, SendAttachment, SendMessage fallback, gateway/runtime/auth, or live mutation was emitted. Play is the only review control that opens the pending local audio file with the system opener; Drop clears pending voice review state. {review_playback_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn play_telegram_voice_review_audio(&mut self, cx: &mut Cx) {
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice)
            .cloned();
        let Some(pending) = pending_voice else {
            let metadata = voice_message_review_playback_open_result_label(
                "",
                None,
                "unavailable; no pending Voice attachment review is loaded",
            );
            self.telegram_voice_review_playback_last_control = Some("Play".to_string());
            self.telegram_voice_local_status =
                "Play needs a pending desktop audio review before opener handoff".to_string();
            self.update_telegram_voice_message_panel(cx);
            self.set_telegram_voice_message_panel_visible(cx, true);
            self.set_message_send_operation_status(
                cx,
                "voice-review-play-empty",
                "Voice Play held locally",
                &metadata,
            );
            enqueue_popup_notification(metadata, PopupKind::Warning, Some(4.0));
            return;
        };

        let filename = pending.filename.clone();
        let duration_label = pending.audio_duration_label.clone();
        let open_result = open_voice_review_audio_file(&pending.file_path);
        self.telegram_voice_review_playback_last_control = Some("Play".to_string());
        let result_state = match &open_result {
            Ok(()) => {
                self.telegram_voice_local_status = format!(
                    "Play opened pending voice review audio with system opener: {filename}"
                );
                "opened with system opener".to_string()
            }
            Err(error) => {
                self.telegram_voice_local_status =
                    format!("Play could not open pending voice review audio: {filename}; {error}");
                format!("failed: {error}")
            }
        };
        let metadata = voice_message_review_playback_open_result_label(
            &filename,
            duration_label.as_deref(),
            &result_state,
        );
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        self.set_message_send_operation_status(
            cx,
            if open_result.is_ok() {
                "voice-review-opened-local"
            } else {
                "voice-review-open-failed"
            },
            if open_result.is_ok() {
                "Voice review Play opened locally"
            } else {
                "Voice review Play failed locally"
            },
            &metadata,
        );
        enqueue_popup_notification(
            metadata,
            if open_result.is_ok() {
                PopupKind::Info
            } else {
                PopupKind::Warning
            },
            Some(4.0),
        );
    }

    fn drop_telegram_voice_review_audio(&mut self, cx: &mut Cx) {
        let has_pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| pending.kind == AttachmentHandoffKind::Voice)
            .unwrap_or(false);
        let dropped_voice = if has_pending_voice {
            self.telegram_pending_attachment_send.take()
        } else {
            None
        };
        let retry_cache_cleared = self
            .telegram_attachment_send_retry_attempt
            .as_ref()
            .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
            .unwrap_or(false);
        if retry_cache_cleared {
            self.telegram_attachment_send_retry_attempt = None;
            self.telegram_attachment_send_cached_error = None;
        }

        let drop_label = voice_message_review_drop_pending_audio_label(
            dropped_voice
                .as_ref()
                .map(|pending| pending.filename.as_str()),
            dropped_voice
                .as_ref()
                .and_then(|pending| pending.audio_duration_label.as_deref()),
            retry_cache_cleared,
        );
        self.telegram_voice_review_playback_last_control = Some("Drop".to_string());
        if let Some(pending) = dropped_voice.as_ref() {
            let duration_note = pending
                .audio_duration_label
                .as_deref()
                .unwrap_or("duration unavailable");
            self.telegram_voice_local_status = format!(
                "Drop cleared pending voice review locally: {} ({duration_note})",
                pending.filename
            );
            self.telegram_attachment_local_status = format!(
                "Voice attachment review dropped locally: {}; no local file deletion or upload",
                pending.filename
            );
            self.set_message_send_operation_status(
                cx,
                "discarded-local",
                "Voice review dropped locally",
                &format!(
                    "Voice review Drop consumed the pending selected audio review with Option::take() while preserving composer caption/reply text. Repeated review-row Send now has no pending voice attachment to submit. No local file was deleted, no MatrixRequest::SendAttachment or caption-only SendMessage was emitted, and no SDK send-queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request was sent. {drop_label}"
                ),
            );
        } else {
            self.telegram_voice_local_status =
                "Drop found no pending voice review audio and stayed local".to_string();
            self.set_message_send_operation_status(
                cx,
                "empty-held",
                "Voice Drop held locally",
                &format!(
                    "Voice review Drop found no pending Voice attachment review to clear. Existing Photo/File pending review state, if any, was left untouched. No local file deletion, SendAttachment, SendMessage fallback, SDK queue cancel, gateway/runtime/auth, or live mutation request was emitted. {drop_label}"
                ),
            );
        }

        self.update_telegram_attachment_picker(cx);
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        enqueue_popup_notification(
            format!("Voice review Drop completed as local cleanup. {drop_label}"),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_send_preflight_detail_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let status = format!("voice-send-preflight-{control}-local").to_ascii_lowercase();
        let latest_status = if self.telegram_voice_local_status.trim().is_empty() {
            status.as_str()
        } else {
            self.telegram_voice_local_status.as_str()
        };
        let source_copy = if self.telegram_voice_send_preflight_detail.trim().is_empty() {
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
        } else {
            self.telegram_voice_send_preflight_detail.as_str()
        };
        let label = voice_message_send_preflight_detail_controls_label(
            control,
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            self.telegram_attachment_send_retry_attempt
                .as_ref()
                .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                .unwrap_or(false),
            latest_status,
            self.telegram_attachment_send_cached_error.as_deref(),
            source_copy,
        );
        self.telegram_voice_send_preflight_detail = label.clone();
        self.telegram_voice_local_status =
            format!("Voice Send preflight {control} detail stayed local");
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Voice Send {control} detail stayed local. No microphone permission, recorder, captured audio file, extra SendAttachment, unconfirmed retry, SendMessage fallback, gateway/runtime/auth, or live mutation was emitted. {label}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn replied_to_event_id(&self) -> Option<OwnedEventId> {
        self.replying_to
            .as_ref()
            .and_then(|(event_tl_item, _)| event_tl_item.event_id().map(ToOwned::to_owned))
    }

    /// Updates (populates and shows or hides) this room's tombstone footer
    /// based on the given successor room details.
    fn update_tombstone_footer(
        &mut self,
        cx: &mut Cx,
        tombstoned_room_id: &OwnedRoomId,
        successor_room_details: Option<&SuccessorRoomDetails>,
    ) {
        let tombstone_footer = self.tombstone_footer(cx, ids!(tombstone_footer));
        let input_bar = self.view(cx, ids!(input_bar));

        if let Some(srd) = successor_room_details {
            tombstone_footer.show(cx, tombstoned_room_id, srd);
            input_bar.set_visible(cx, false);
            self.set_telegram_attachment_picker_visible(cx, false);
            self.set_telegram_emoji_sticker_panel_visible(cx, false);
            self.set_telegram_voice_message_panel_visible(cx, false);
        } else {
            tombstone_footer.hide(cx);
            input_bar.set_visible(cx, true);
        }
    }

    fn set_message_send_operation_status(
        &mut self,
        cx: &mut Cx,
        status: &str,
        title: &str,
        evidence: &str,
    ) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let multi_file_queue_boundary = attachment_multi_file_queue_boundary_label(
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
        );
        let per_file_status_controls = attachment_per_file_status_controls_label(
            "Status",
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            status,
        );
        let timeline_cancel_bridge = attachment_accepted_queue_timeline_cancel_bridge_label(
            "Status",
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            status,
        );
        let send_preflight_control = attachment_send_preflight_control_from_status(status);
        let send_preflight_detail = attachment_send_preflight_detail_controls_label(
            send_preflight_control,
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            status,
            self.telegram_attachment_send_cached_error.as_deref(),
            evidence,
        );
        self.telegram_attachment_send_preflight_detail = send_preflight_detail.clone();
        // Message send operation evidence: this only updates local status labels after
        // existing SendMessage/SendAttachment submit paths or local Retry/Cancel clicks.
        self.view
            .label(
                cx,
                ids!(send_operation_status.status_actions.queue_status_label),
            )
            .set_text(cx, status);
        self.view
            .label(cx, ids!(send_operation_status.title))
            .set_text(cx, title);
        self.view
            .label(cx, ids!(send_operation_status.evidence))
            .set_text(cx, evidence);
        self.view
            .label(cx, ids!(send_operation_status.result_bridge))
            .set_text(cx, ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_LABEL);
        self.view
            .label(cx, ids!(send_operation_status.taxonomy))
            .set_text(cx, ATTACHMENT_STATUS_TAXONOMY_LABEL);
        self.view
            .label(cx, ids!(send_operation_status.compact_fit))
            .set_text(cx, ATTACHMENT_REVIEW_ROW_COMPACT_FIT_LABEL);
        self.view
            .label(cx, ids!(send_operation_status.action_density))
            .set_text(cx, ATTACHMENT_MOBILE_ACTION_DENSITY_LABEL);
        self.view
            .label(cx, ids!(send_operation_status.multi_file_queue_boundary))
            .set_text(cx, &multi_file_queue_boundary);
        self.view
            .label(cx, ids!(send_operation_status.accepted_queue_actions_label))
            .set_text(cx, ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_LABEL);
        self.view
            .label(
                cx,
                ids!(send_operation_status.accepted_queue_timeline_cancel_bridge_label),
            )
            .set_text(cx, &timeline_cancel_bridge);
        self.view
            .label(
                cx,
                ids!(send_operation_status.per_file_status_controls_label),
            )
            .set_text(cx, &per_file_status_controls);
        self.view
            .label(
                cx,
                ids!(send_operation_status.attachment_send_preflight_detail_controls_label),
            )
            .set_text(cx, &send_preflight_detail);
        self.redraw(cx);
    }

    fn stage_telegram_attachment_timeline_cancel_bridge_control(
        &mut self,
        cx: &mut Cx,
        control: &str,
    ) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let latest_status = if self.telegram_attachment_local_status.trim().is_empty() {
            "local evidence"
        } else {
            self.telegram_attachment_local_status.as_str()
        };
        let label = attachment_accepted_queue_timeline_cancel_bridge_label(
            control,
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            latest_status,
        );
        self.telegram_attachment_local_status =
            format!("Accepted queue timeline-cancel {control} bridge stayed local");
        self.set_message_send_operation_status(
            cx,
            &format!("timeline-cancel-{control}-local").to_ascii_lowercase(),
            &format!("Timeline cancel {control} stayed local"),
            &label,
        );
        enqueue_popup_notification(
            format!(
                "Attachment timeline-cancel {control} bridge stayed local. Use the timeline local echo context menu's Cancel Send when a SendHandle exists; no composer SDK queue abort, SendAttachment resubmit, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_attachment_accepted_queue_action(&mut self, cx: &mut Cx, action: &str) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let retry_cache_ready = self.telegram_attachment_send_retry_attempt.is_some();
        let label = if action.trim().eq_ignore_ascii_case("Background") {
            attachment_accepted_queue_background_snapshot_label(
                pending_review.as_deref(),
                retry_cache_ready,
                &self.telegram_attachment_local_status,
            )
        } else {
            attachment_accepted_queue_actions_row_label(
                action,
                pending_review.as_deref(),
                retry_cache_ready,
            )
        };
        self.telegram_attachment_local_status =
            format!("Accepted SDK queue {action} control stayed local");
        self.set_message_send_operation_status(
            cx,
            &format!("queue-{action}-local").to_ascii_lowercase(),
            &format!("Queue {action} stayed local"),
            &label,
        );
        enqueue_popup_notification(
            format!(
                "Attachment queue {action} stayed local. No SDK queue retry/resume/abort/remove/reorder, SendAttachment resubmit, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_attachment_per_file_status_control(&mut self, cx: &mut Cx, control: &str) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let status = format!("per-file-{control}-local").to_ascii_lowercase();
        let label = attachment_per_file_status_controls_label(
            control,
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            &status,
        );
        self.telegram_attachment_local_status =
            format!("Per-file attachment {control} control stayed local");
        self.set_message_send_operation_status(
            cx,
            &status,
            &format!("Per-file {control} stayed local"),
            &label,
        );
        enqueue_popup_notification(
            format!(
                "Attachment per-file {control} stayed local. No SDK progress subscription, queue pause/resume/cancel/retry, SendAttachment resubmit, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_attachment_send_preflight_detail_control(
        &mut self,
        cx: &mut Cx,
        control: &str,
    ) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let status = format!("send-preflight-{control}-local").to_ascii_lowercase();
        let latest_status = if self.telegram_attachment_local_status.trim().is_empty() {
            status.as_str()
        } else {
            self.telegram_attachment_local_status.as_str()
        };
        let source_copy = if self
            .telegram_attachment_send_preflight_detail
            .trim()
            .is_empty()
        {
            ATTACHMENT_SEND_RESULT_BRIDGE_EVIDENCE
        } else {
            self.telegram_attachment_send_preflight_detail.as_str()
        };
        let label = attachment_send_preflight_detail_controls_label(
            control,
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            latest_status,
            self.telegram_attachment_send_cached_error.as_deref(),
            source_copy,
        );
        self.telegram_attachment_send_preflight_detail = label.clone();
        self.telegram_attachment_local_status =
            format!("Attachment send preflight {control} detail stayed local");
        self.set_message_send_operation_status(
            cx,
            &status,
            &format!("Attachment {control} detail stayed local"),
            &label,
        );
        enqueue_popup_notification(
            format!(
                "Attachment send {control} detail stayed local. No SendAttachment, SDK queue retry/cancel, upload abort, caption-only SendMessage, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn handle_attachment_send_result_inner(
        &mut self,
        cx: &mut Cx,
        filename: String,
        result: Result<(), String>,
    ) {
        match result {
            Ok(()) => {
                if self
                    .telegram_attachment_send_retry_attempt
                    .as_ref()
                    .map(|attempt| attempt.filename == filename)
                    .unwrap_or(false)
                {
                    self.telegram_attachment_send_retry_attempt = None;
                }
                self.telegram_attachment_send_cached_error = None;
                self.telegram_attachment_local_status =
                    format!("Attachment queued in Matrix send queue: {filename}");
                self.set_message_send_operation_status(
                    cx,
                    "queued-confirmed",
                    "Attachment queued by SDK",
                    "The SendAttachment worker confirmed that Timeline::send_attachment().use_send_queue() accepted this attachment into the SDK send queue and the cached failed-handoff Retry attempt for this filename was cleared. This is still not delivery success: RoomScreen renders SDK queue progress/error/sent state on the timeline local echo, while composer Cancel still does not abort or remove SDK queue work and Retry has no cached failure to reuse.",
                );
                enqueue_popup_notification(
                    format!("Attachment queued by Matrix send queue: {filename}."),
                    PopupKind::Success,
                    Some(4.0),
                );
            }
            Err(error) => {
                self.telegram_attachment_send_cached_error = Some(error.clone());
                self.telegram_attachment_local_status =
                    format!("Attachment handoff failed before SDK queue: {filename}");
                self.set_message_send_operation_status(
                    cx,
                    "failure-copy",
                    "Attachment handoff failed",
                    &format!(
                        "The SendAttachment worker returned an immediate handoff failure before SDK queue ownership for {filename}: {error}. Retry now requires PositiveConfirmationModal before reusing the cached last validated SendAttachment handoff; Cancel clears only local retry cache and does not abort or remove SDK queue work. No automatic retry, caption-only SendMessage, SDK queue retry/resume, room-state, membership, gateway/runtime/auth, or live mutation request is emitted."
                    ),
                );
            }
        }
        self.update_telegram_attachment_picker(cx);
    }

    fn handle_local_send_abort_result_inner(&mut self, cx: &mut Cx, result: Result<bool, String>) {
        let status = match &result {
            Ok(true) => "timeline-cancel-canceled",
            Ok(false) => "timeline-cancel-not-cancellable",
            Err(_) => "timeline-cancel-failed",
        };
        let title = match &result {
            Ok(true) => "Timeline local send canceled",
            Ok(false) => "Timeline local send already sent",
            Err(_) => "Timeline local send cancel failed",
        };
        let label = attachment_local_send_abort_result_label(&result);
        self.telegram_attachment_local_status = title.to_string();
        self.set_message_send_operation_status(cx, status, title, &label);
        self.update_telegram_attachment_picker(cx);
    }

    fn set_typing_notice_status(&mut self, cx: &mut Cx, title: &str, evidence: &str) {
        // Typing notice evidence: this only updates local labels around the existing
        // SendTypingNotice path or the local Hepta command preview suppression path.
        self.view
            .label(cx, ids!(typing_notice_status.title))
            .set_text(cx, title);
        self.view
            .label(cx, ids!(typing_notice_status.evidence))
            .set_text(cx, evidence);
        self.redraw(cx);
    }

    /// Sets the send_message_button to be enabled and green, or disabled and gray.
    ///
    /// This should be called to update the button state when the message TextInput content changes.
    fn enable_send_message_button(&mut self, cx: &mut Cx, enable: bool) {
        let mut send_message_button = self.view.button(cx, ids!(send_message_button));
        let (fg_color, bg_color) = if enable {
            (COLOR_FG_ACCEPT_GREEN, COLOR_BG_ACCEPT_GREEN)
        } else {
            (COLOR_FG_DISABLED, COLOR_BG_DISABLED)
        };
        script_apply_eval!(cx, send_message_button, {
            enabled: #(enable),
            draw_icon.color: #(fg_color),
            draw_bg.color: #(bg_color),
        });
    }

    fn update_hepta_command_preview(&mut self, cx: &mut Cx, input: &str) {
        let preview_view = self.view.view(cx, ids!(hepta_command_preview));
        let Some(plan) = plan_hepta_composer_command(input, 0) else {
            preview_view.set_visible(cx, false);
            return;
        };
        let preview = plan.to_bridge_input();
        preview_view.set_visible(cx, true);
        self.view
            .label(cx, ids!(hepta_command_preview.title))
            .set_text(
                cx,
                &format!("Hepta dry-run · m.hepta.{}", preview.event_kind),
            );
        self.view
            .label(cx, ids!(hepta_command_preview.body))
            .set_text(cx, &plan.operator_summary());
        self.view.label(cx, ids!(hepta_command_preview.meta)).set_text(
            cx,
            &format!(
                "preview={} · confirmation={} · external_mutation_enabled=false · Matrix typing notice suppressed",
                preview.id,
                plan.requires_confirmation(),
            ),
        );
    }

    /// Updates the visibility of select views based on the user's new power levels.
    ///
    /// This will show/hide the `input_bar` and the `can_not_send_message_notice` views.
    fn update_user_power_levels(&mut self, cx: &mut Cx, user_power_levels: UserPowerLevels) {
        let can_send = user_power_levels.can_send_message();
        self.view
            .view(cx, ids!(input_bar))
            .set_visible(cx, can_send);
        self.view
            .view(cx, ids!(can_not_send_message_notice))
            .set_visible(cx, !can_send);
        if !can_send {
            self.set_telegram_attachment_picker_visible(cx, false);
            self.set_telegram_emoji_sticker_panel_visible(cx, false);
            self.set_telegram_voice_message_panel_visible(cx, false);
        }
    }

    /// Returns true if the TSP signing checkbox is checked, false otherwise.
    ///
    /// If TSP is not enabled, this will always return false.
    #[cfg(feature = "tsp")]
    fn is_tsp_signing_enabled(&self, cx: &mut Cx) -> bool {
        self.view.check_box(cx, ids!(tsp_sign_checkbox)).active(cx)
    }
}

impl RoomInputBarRef {
    /// Shows a preview of the given event that the user is currently replying to
    /// above the message input bar.
    pub fn show_replying_to(
        &self,
        cx: &mut Cx,
        replying_to: (EventTimelineItem, EmbeddedEvent),
        timeline_kind: &TimelineKind,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.show_replying_to(cx, replying_to, timeline_kind, true);
    }

    /// Hides the upload progress view for the given upload attempt.
    pub fn hide_upload_progress(&self, cx: &mut Cx, upload_id: FileUploadAttemptId) {
        let Some(inner) = self.borrow() else {
            return;
        };
        inner
            .child_by_path(ids!(upload_progress_view))
            .as_upload_progress_view()
            .hide(cx, upload_id);
    }

    /// Updates progress for the matching upload attempt.
    pub fn set_upload_progress(
        &self,
        cx: &mut Cx,
        upload_id: FileUploadAttemptId,
        current: u64,
        total: u64,
    ) {
        let Some(inner) = self.borrow() else {
            return;
        };
        inner
            .child_by_path(ids!(upload_progress_view))
            .as_upload_progress_view()
            .set_progress(cx, upload_id, current, total);
    }

    /// Shows an upload error while preserving the retry payload.
    pub fn show_upload_error(
        &self,
        cx: &mut Cx,
        upload_id: FileUploadAttemptId,
        error: &str,
        upload: AttachmentUpload,
        retryable: bool,
    ) {
        let Some(inner) = self.borrow() else {
            return;
        };
        inner
            .child_by_path(ids!(upload_progress_view))
            .as_upload_progress_view()
            .show_error(cx, upload_id, error, upload, retryable);
    }

    /// Starts the matching upload row and clears only the captured reply target.
    pub fn handle_file_upload_started(
        &self,
        cx: &mut Cx,
        upload_id: FileUploadAttemptId,
        file_name: &str,
        in_reply_to: Option<&OwnedEventId>,
        abort_handle: futures_util::future::AbortHandle,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner
            .child_by_path(ids!(upload_progress_view))
            .as_upload_progress_view()
            .show(cx, upload_id, file_name, abort_handle);

        if let Some(in_reply_to) = in_reply_to {
            let should_clear_reply = inner
                .replying_to
                .as_ref()
                .and_then(|(event_tl_item, _)| event_tl_item.event_id())
                .is_some_and(|current| current == in_reply_to);
            if should_clear_reply {
                inner.clear_replying_to(cx);
            }
        }
    }

    /// Shows the editing pane to allow the user to edit the given event.
    pub fn show_editing_pane(
        &self,
        cx: &mut Cx,
        event_tl_item: EventTimelineItem,
        timeline_kind: TimelineKind,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.show_editing_pane(
            cx,
            ShowEditingPaneBehavior::ShowNew { event_tl_item },
            timeline_kind,
        );
    }

    /// Updates the visibility of select views based on the user's new power levels.
    ///
    /// This will show/hide the `input_bar` and the `can_not_send_message_notice` views.
    pub fn update_user_power_levels(&self, cx: &mut Cx, user_power_levels: UserPowerLevels) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.update_user_power_levels(cx, user_power_levels);
    }

    /// Updates this room's tombstone footer based on the given `tombstone_state`.
    pub fn update_tombstone_footer(
        &self,
        cx: &mut Cx,
        tombstoned_room_id: &OwnedRoomId,
        successor_room_details: Option<&SuccessorRoomDetails>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.update_tombstone_footer(cx, tombstoned_room_id, successor_room_details);
    }

    /// Forwards the result of an edit request to the `EditingPane` widget
    /// within this `RoomInputBar`.
    pub fn handle_edit_result(
        &self,
        cx: &mut Cx,
        timeline_event_item_id: TimelineEventItemId,
        edit_result: Result<(), matrix_sdk_ui::timeline::Error>,
    ) {
        let Some(inner) = self.borrow_mut() else {
            return;
        };
        inner
            .editing_pane(cx, ids!(editing_pane))
            .handle_edit_result(cx, timeline_event_item_id, edit_result);
    }

    /// Forwards the worker result of an attachment send handoff to the Telegram
    /// operation strip.
    pub fn handle_attachment_send_result(
        &self,
        cx: &mut Cx,
        filename: String,
        result: Result<(), String>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.handle_attachment_send_result_inner(cx, filename, result);
    }

    /// Forwards the worker result of a timeline local echo cancel request to the
    /// Telegram operation strip.
    pub fn handle_local_send_abort_result(&self, cx: &mut Cx, result: Result<bool, String>) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.handle_local_send_abort_result_inner(cx, result);
    }

    /// Save a snapshot of the UI state of this `RoomInputBar`.
    pub fn save_state(&self) -> RoomInputBarState {
        let Some(inner) = self.borrow() else {
            return Default::default();
        };
        // Clear the location preview. We don't save this state because the
        // current location might change by the next time the user opens this same room.
        inner
            .child_by_path(ids!(location_preview))
            .as_location_preview()
            .clear();
        RoomInputBarState {
            was_replying_preview_visible: inner.was_replying_preview_visible,
            replying_to: inner.replying_to.clone(),
            editing_pane_state: inner
                .child_by_path(ids!(editing_pane))
                .as_editing_pane()
                .save_state(),
            text_input_state: inner
                .child_by_path(ids!(input_bar.mentionable_text_input.text_input))
                .as_text_input()
                .save_state(),
        }
    }

    /// Restore the UI state of this `RoomInputBar` from the given state snapshot.
    pub fn restore_state(
        &self,
        cx: &mut Cx,
        timeline_kind: TimelineKind,
        saved_state: RoomInputBarState,
        user_power_levels: UserPowerLevels,
        tombstone_info: Option<&SuccessorRoomDetails>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        let RoomInputBarState {
            was_replying_preview_visible,
            text_input_state,
            replying_to,
            editing_pane_state,
        } = saved_state;

        // Note: we do *not* restore the location preview state here; see `save_state()`.
        inner.set_telegram_attachment_picker_visible(cx, false);
        inner.set_telegram_emoji_sticker_panel_visible(cx, false);
        inner.set_telegram_voice_message_panel_visible(cx, false);

        // 0. Update select views based on user power levels from the RoomScreen (the `TimelineUiState`).
        //    This must happen before we restore the state of the `EditingPane`,
        //    because the call to `show_editing_pane()` might re-update the `input_bar`'s visibility.
        inner.update_user_power_levels(cx, user_power_levels);

        // 1. Restore the state of the TextInput within the MentionableTextInput.
        inner
            .text_input(cx, ids!(input_bar.mentionable_text_input.text_input))
            .restore_state(cx, text_input_state);

        // 2. Restore the state of the replying-to preview.
        if let Some(replying_to) = replying_to {
            inner.show_replying_to(cx, replying_to, &timeline_kind, false);
        } else {
            inner.clear_replying_to(cx);
        }
        inner.was_replying_preview_visible = was_replying_preview_visible;

        // 3. Restore the state of the editing pane.
        if let Some(editing_pane_state) = editing_pane_state {
            inner.show_editing_pane(
                cx,
                ShowEditingPaneBehavior::RestoreExisting { editing_pane_state },
                timeline_kind.clone(),
            );
        } else {
            inner
                .editing_pane(cx, ids!(editing_pane))
                .force_reset_hide(cx);
            inner.on_editing_pane_hidden(cx);
        }

        // 4. Restore the state of the tombstone footer.
        //    This depends on the `EditingPane` state, so it must be done after Step 3.
        inner.update_tombstone_footer(cx, timeline_kind.room_id(), tombstone_info);
    }
}
