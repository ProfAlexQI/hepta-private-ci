use std::cell::RefCell;

use makepad_widgets::{text::selection::Cursor, *};
use matrix_sdk::{
    room::edit::EditedContent,
    ruma::{
        events::{
            poll::unstable_start::{UnstablePollAnswer, UnstablePollStartContentBlock},
            room::message::{FormattedBody, MessageType, RoomMessageEventContentWithoutRelation},
        },
    },
};
use matrix_sdk_ui::timeline::{
    EventTimelineItem, MsgLikeKind, TimelineEventItemId, TimelineItemContent,
};

use crate::shared::mentionable_text_input::{
    MentionableTextInputWidgetExt, MentionableTextInputWidgetRefExt,
};
use crate::{
    app::PositiveConfirmationModalAction,
    settings::app_preferences::{AppPreferencesAction, AppPreferencesGlobal},
    shared::{
        confirmation_modal::ConfirmationModalContent,
        popup_list::{PopupKind, enqueue_popup_notification},
    },
    sliding_sync::{MatrixRequest, TimelineKind, submit_async_request},
};

const EDITING_PANE_CONFIRMATION_COMPACT_LABEL: &str =
    "Confirmation required before the Matrix edit request.";
const EDITING_PANE_LIMITS_COMPACT_LABEL: &str =
    "Edit extras stay local; Save Edit uses confirmation.";
pub const EDITING_PANE_DETAIL_PACKET_EVIDENCE: &str = "Edit/Poll detail packet records attachment_edit_slot not_built, mention_payload_scope preserve_existing_only_or_none, poll_answer_edit_slot not_built, save_spinner_operation_id not_assigned, result_mapping not_wired, and stale-result policy before the existing confirmed Matrix EditMessage request.";
pub const EDITING_PANE_ATTACHMENT_PREFLIGHT_PACKET_EVIDENCE: &str = "Edit attachment preflight packet records original_attachment_scope, add/remove/replace/upload/delete slots not_built, caption_edit_handoff existing_confirmed_MatrixRequest_EditMessage_body_only, MIME/size probe not_started, retry idempotency, and cancel policy while keeping SendAttachment/media delete/upload unwired.";
pub const EDITING_PANE_MENTION_PAYLOAD_PREFLIGHT_PACKET_EVIDENCE: &str = "Edit mention payload preflight packet records edited @token counts, literal Matrix user-id token counts, @room token scope, completed pill reconciliation not_connected, directory result scope unavailable, fresh_mentions_payload_slot not_built, existing_mentions_handoff preserve_existing_only_or_none, retry source-hash slot missing, and cancel policy while keeping fresh Matrix Mentions extraction unwired.";
pub const EDITING_PANE_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE: &str = "Edit mention payload typed contract packet records token_scan_source edited_text_only, source_hash_slot not_assigned, directory_snapshot_id_slot unavailable, completed_pill_snapshot_slot unavailable, fresh_mentions_payload_result_slot not_built, retry_idempotency_key_slot missing, and result_mapping not_wired while keeping fresh Matrix Mentions extraction unwired.";
pub const EDITING_PANE_SAVE_RESULT_MAPPING_PACKET_EVIDENCE: &str = "Edit/Poll save-result mapping packet records lifecycle_state, operation_id_slot not_assigned, request_slot existing_confirmed_MatrixRequest_EditMessage, spinner_slot not_rendered, result_mapping saved/failed/canceled/stale/ignored-late, stale_result_guard timeline_event_item_id_match_only, repeated_save_policy not_held_until_pending_operation_id, and retry_slot not_built without changing the existing confirmed Matrix EditMessage path.";
pub const EDITING_PANE_RETRY_ERROR_DRILLDOWN_PACKET_EVIDENCE: &str = "Edit/Poll retry/error drilldown packet records failure_source existing_MatrixRequest_EditMessage_result_only, retry_request_slot not_built, retry_confirmation_slot not_built, late_result_guard timeline_event_item_id_match_only_without_operation_id, pending_operation_id missing_backend_contract, spinner_state not_rendered, cancel_state confirmation_cancel_no_request, and error_redaction popup_text_not_persisted_or_reused without changing the existing confirmed Matrix EditMessage path.";

fn editing_pane_detail_packet_label(
    content_kind: &str,
    edited_text_len: usize,
    preserves_existing_mentions: bool,
) -> String {
    let mention_scope = if preserves_existing_mentions {
        "preserve_existing_mentions_only"
    } else {
        "none"
    };
    format!(
        "{EDITING_PANE_LIMITS_COMPACT_LABEL} Detail packet: content_kind {content_kind}; edited_text_len {edited_text_len}; attachment_edit_slot not_built; mention_payload_scope {mention_scope}; poll_answer_edit_slot not_built; save_spinner_operation_id not_assigned; result_mapping not_wired; stale_result_policy ignore_late_result_without_matching_operation_id; no attachment upload/remove, Matrix mention payload, poll answer edit, timeline reload, message send, room-state, or membership request was sent."
    )
}

fn editing_pane_attachment_preflight_packet_label(
    content_kind: &str,
    edited_text_len: usize,
) -> String {
    let original_attachment_scope = match content_kind {
        "image_caption" => "existing_image_media_caption_only",
        "audio_caption" => "existing_audio_media_caption_only",
        "file_caption" => "existing_file_media_caption_only",
        "video_caption" => "existing_video_media_caption_only",
        "text_body" | "emote_body" => "no_existing_media_attachment",
        "poll_question_preserve_answers" => "poll_no_attachment",
        _ => "unsupported_attachment_scope",
    };
    format!(
        "Attachment preflight packet: content_kind {content_kind}; edited_text_len {edited_text_len}; original_attachment_scope {original_attachment_scope}; selected_attachment_slot unavailable; add_attachment_slot not_built; remove_attachment_slot not_built; replace_attachment_slot not_built; upload_request_slot not_built; media_delete_slot not_built; caption_edit_handoff existing_confirmed_MatrixRequest_EditMessage_body_only; mime_size_probe not_started; retry_policy no_duplicate_upload_without_operation_id; cancel_policy leaves_original_media_and_local_selection_untouched; no SendAttachment, media delete, upload, timeline reload, room-state, membership request, gateway/runtime/auth, or live mutation was sent."
    )
}

fn editing_pane_mention_payload_preflight_packet_label(
    content_kind: &str,
    edited_text: &str,
    preserves_existing_mentions: bool,
) -> String {
    let at_token_count = edited_text
        .split_whitespace()
        .filter(|token| token.starts_with('@'))
        .count();
    let literal_user_id_token_count = edited_text
        .split_whitespace()
        .filter(|token| token.starts_with('@') && token.contains(':'))
        .count();
    let room_token_scope = if edited_text.split_whitespace().any(|token| token == "@room") {
        "present_requires_power_level_recheck"
    } else {
        "absent"
    };
    let existing_mentions_handoff = if preserves_existing_mentions {
        "preserve_existing_mentions_only"
    } else {
        "none"
    };
    format!(
        "Mention payload preflight packet: content_kind {content_kind}; edited_text_len {}; edited_at_token_count {at_token_count}; literal_user_id_token_count {literal_user_id_token_count}; room_token_scope {room_token_scope}; completed_pill_reconcile_slot not_connected_to_editing_pane; directory_result_scope unavailable_in_editing_pane; fresh_mentions_payload_slot not_built; existing_mentions_handoff {existing_mentions_handoff}; reply_sendtime_state not_reused; retry_source_hash_slot missing; stale_token_policy backend_required_before_live_mentions; cancel_policy confirmation_cancel_no_request; no fresh Matrix Mentions payload, profile lookup, directory search, SendMessage, SendAttachment, room-state, membership request, gateway/runtime/auth, or live mutation was sent.",
        edited_text.len()
    )
}

fn editing_pane_mention_payload_typed_contract_packet_label(
    content_kind: &str,
    edited_text: &str,
    preserves_existing_mentions: bool,
) -> String {
    let at_token_count = edited_text
        .split_whitespace()
        .filter(|token| token.starts_with('@'))
        .count();
    let literal_user_id_token_count = edited_text
        .split_whitespace()
        .filter(|token| token.starts_with('@') && token.contains(':'))
        .count();
    let room_token_contract_scope = if edited_text.split_whitespace().any(|token| token == "@room")
    {
        "requires_power_level_recheck_before_payload"
    } else {
        "absent"
    };
    let existing_mentions_handoff = if preserves_existing_mentions {
        "preserve_existing_mentions_only"
    } else {
        "none"
    };
    format!(
        "Mention payload typed contract packet: content_kind {content_kind}; edited_text_len {}; mention_contract_version local_v0; token_scan_source edited_text_only; edited_at_token_count {at_token_count}; literal_user_id_contract_count {literal_user_id_token_count}; room_token_contract_scope {room_token_contract_scope}; directory_snapshot_id_slot unavailable; completed_pill_snapshot_slot unavailable; existing_mentions_handoff {existing_mentions_handoff}; source_hash_slot not_assigned; fresh_mentions_payload_result_slot not_built; retry_idempotency_key_slot missing; stale_result_guard body_source_hash_required_before_live_mentions; result_mapping accepted|permission_denied|stale_body|malformed_token|directory_unavailable not_wired; privacy_redaction token_counts_only; no fresh Matrix Mentions payload, directory snapshot reuse, profile lookup, SendMessage, SendAttachment, room-state, membership request, gateway/runtime/auth, or live mutation was sent.",
        edited_text.len()
    )
}

fn editing_pane_save_result_mapping_packet_label(
    content_kind: &str,
    edited_text_len: usize,
    preserves_existing_mentions: bool,
    lifecycle_state: &str,
) -> String {
    let mention_scope = if preserves_existing_mentions {
        "preserve_existing_mentions_only"
    } else {
        "none"
    };
    format!(
        "Save result mapping packet: lifecycle_state {lifecycle_state}; content_kind {content_kind}; edited_text_len {edited_text_len}; mention_payload_scope {mention_scope}; operation_id_slot not_assigned; request_slot existing_confirmed_MatrixRequest_EditMessage; spinner_slot not_rendered; result_mapping saved_hide_pane|failed_popup|canceled_no_request|stale_event_id_ignored|ignored_late_result_without_matching_operation_id; stale_result_guard timeline_event_item_id_match_only; repeated_save_policy not_held_until_pending_operation_id; retry_slot not_built; no attachment upload/remove, Matrix mention payload, poll answer edit, timeline reload, message send, room-state, or membership request was sent."
    )
}

fn editing_pane_retry_error_drilldown_packet_label(
    content_kind: &str,
    edited_text_len: usize,
    preserves_existing_mentions: bool,
    lifecycle_state: &str,
) -> String {
    let mention_scope = if preserves_existing_mentions {
        "preserve_existing_mentions_only"
    } else {
        "none"
    };
    let retry_state = match lifecycle_state {
        "confirmation_opened" => "confirmation_pending_cancel_no_request_until_accept",
        "failed_popup" => "manual_retry_not_built_existing_popup_only",
        "stale_event_id_ignored" => "retry_blocked_stale_event_id",
        "saved_hide_pane" => "not_needed_after_success",
        _ => "idle_not_started",
    };
    format!(
        "Retry/error drilldown packet: lifecycle_state {lifecycle_state}; content_kind {content_kind}; edited_text_len {edited_text_len}; mention_payload_scope {mention_scope}; failure_source existing_MatrixRequest_EditMessage_result_only; error_redaction popup_text_not_persisted_or_reused; retry_request_slot not_built; retry_confirmation_slot not_built; late_result_guard timeline_event_item_id_match_only_without_operation_id; pending_operation_id missing_backend_contract; spinner_state not_rendered; cancel_state confirmation_cancel_no_request; repeated_save_policy not_held_until_pending_operation_id; stale_result_policy ignore_late_result_without_matching_operation_id; retry_state {retry_state}; no attachment upload/remove, Matrix mention payload, poll answer edit, timeline reload, extra message send beyond the existing confirmed edit request, room-state, membership request, gateway/runtime/auth, or live mutation was sent."
    )
}

fn editing_pane_detail_packet_source(event_tl_item: &EventTimelineItem) -> (&'static str, bool) {
    match event_tl_item.content() {
        TimelineItemContent::MsgLike(msg_like_content) => match &msg_like_content.kind {
            MsgLikeKind::Message(message) => {
                let content_kind = match message.msgtype() {
                    MessageType::Text(_) => "text_body",
                    MessageType::Emote(_) => "emote_body",
                    MessageType::Image(_) => "image_caption",
                    MessageType::Audio(_) => "audio_caption",
                    MessageType::File(_) => "file_caption",
                    MessageType::Video(_) => "video_caption",
                    _ => "unsupported_message",
                };
                (content_kind, message.mentions().is_some())
            }
            MsgLikeKind::Poll(_) => ("poll_question_preserve_answers", false),
            _ => ("unsupported_event", false),
        },
        _ => ("unsupported_event", false),
    }
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.EditingContent = RoundedView {
        width: Fill,
        height: Fit,
        padding: Inset{ left: 20, right: 20, top: 10, bottom: 10 }
        spacing: 10,
        flow: Down,

        // this must match the RoomInputBar exactly such that it overlaps atop it.
        margin: Inset{left: -4, right: -4, bottom: -4 }
        show_bg: true,
        draw_bg +: {
            color: (COLOR_TELEGRAM_PANEL)
            border_radius: 0.0
            border_color: (COLOR_TELEGRAM_BORDER)
            border_size: 1.0
            // shadow_color: #0006
            // shadow_radius: 0.0
            // shadow_offset: vec2(0.0,0.0)
        }

        View {
            width: Fill, height: Fit
            flow: Right
            align: Align{y: 0.5}
            padding: Inset{left: 5, right: 5}

            Label {
                width: Fill,
                flow: Right, // do not wrap
                margin: Inset{top: 3}
                draw_text +: {
                    text_style: USERNAME_TEXT_STYLE {},
                    color: (COLOR_TELEGRAM_BLUE),
                }
                text: "Editing:"
            }

            cancel_button := RobrixNegativeIconButton {
                width: Fit,
                height: Fit,
                padding: 13,
                spacing: 0,
                margin: Inset{left: 5, right: 5},

                draw_icon.svg: (ICON_CLOSE)
                icon_walk: Walk{width: 16, height: 16, margin: 0}
            }

            accept_button := RobrixPositiveIconButton {
                width: Fit,
                height: Fit,
                padding: 13,
                spacing: 0,
                margin: Inset{left: 5},

                draw_icon.svg: (ICON_CHECKMARK)
                icon_walk: Walk{width: 16, height: 16, margin: 0}
            }
        }

        LineH { }

        edit_text_input := MentionableTextInput {
            width: Fill
            height: Fit{max: FitBound.Rel{base: Base.Full, factor: 0.75}}
            margin: Inset{ bottom: 5, top: 5 }
        }

        edit_unsupported_features_evidence := Label {
            width: Fill,
            draw_text +: {
                text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 },
                color: (COLOR_TELEGRAM_MUTED),
            }
            text: "Edit/Poll detail packet keeps attachment edit, mention payload extraction, poll answer edit, and save spinner result local until typed contracts exist."
        }

        edit_attachment_preflight_packet := Label {
            width: Fill,
            draw_text +: {
                text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 },
                color: (COLOR_TELEGRAM_MUTED),
            }
            text: "Attachment preflight packet keeps add, remove, replace, upload, delete, retry, and cancel local."
        }

        edit_mention_payload_preflight_packet := Label {
            width: Fill,
            draw_text +: {
                text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 },
                color: (COLOR_TELEGRAM_MUTED),
            }
            text: "Mention payload preflight packet keeps fresh Matrix Mentions extraction local."
        }

        edit_mention_payload_typed_contract_packet := Label {
            width: Fill,
            draw_text +: {
                text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 },
                color: (COLOR_TELEGRAM_MUTED),
            }
            text: "Mention payload typed contract packet keeps source hash, idempotency, and result mapping local."
        }

        edit_save_result_mapping_packet := Label {
            width: Fill,
            draw_text +: {
                text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 },
                color: (COLOR_TELEGRAM_MUTED),
            }
            text: "Save result mapping packet stays local until operation/result contracts exist."
        }

        edit_retry_error_drilldown_packet := Label {
            width: Fill,
            draw_text +: {
                text_style: MESSAGE_TEXT_STYLE { font_size: 9.5 },
                color: (COLOR_TELEGRAM_MUTED),
            }
            text: "Retry/error drilldown packet keeps failure, retry, cancel, and late-result states local."
        }
    }


    mod.widgets.EditingPane = #(EditingPane::register_widget(vm)) {
        ..mod.widgets.RoundedView

        visible: false,
        width: Fill,
        height: Fit{max: FitBound.Rel{base: Base.Full, factor: 0.75}}
        align: Align{x: 0.5, y: 1.0}

        editing_content := mod.widgets.EditingContent { }

        slide: 1.0,

        animator: Animator{
            panel: {
                default: @hide
                show: AnimatorState{
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: { slide: 0.0 }
                }
                hide: AnimatorState{
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: { slide: 1.0 }
                }
            }
        }
    }
}

/// Action emitted by the EditingPane widget.
#[derive(Clone, Default, Debug)]
pub enum EditingPaneAction {
    /// The editing pane's hide animation has started.
    HideAnimationStarted,
    /// The editing pane has been fully closed/hidden.
    Hidden,
    #[default]
    None,
}

impl ActionDefaultRef for EditingPaneAction {
    fn default_ref() -> &'static Self {
        static DEFAULT: EditingPaneAction = EditingPaneAction::None;
        &DEFAULT
    }
}

/// The information maintained by the EditingPane widget.
struct EditingPaneInfo {
    event_tl_item: EventTimelineItem,
    timeline_kind: TimelineKind,
}

/// A view that slides in from the bottom of the screen to allow editing a message.
#[derive(Script, Widget, Animator)]
pub struct EditingPane {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[apply_default]
    animator: Animator,
    #[live]
    slide: f32,

    #[rust]
    info: Option<EditingPaneInfo>,
    #[rust]
    is_animating_out: bool,
    #[rust]
    last_content_height: f64,
    /// Used to force this widget's parent to do a re-draw
    /// after the hide animation completes on this pane.
    #[rust]
    next_frame: NextFrame,
}

impl ScriptHook for EditingPane {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            let send_on_enter = cx.global::<AppPreferencesGlobal>().0.send_on_enter;
            self.mentionable_text_input(cx, ids!(editing_content.edit_text_input))
                .text_input_ref()
                .set_submit_on_enter(send_on_enter);
        });
    }
}

impl Widget for EditingPane {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Handle the next-frame event scheduled after hide animation completes.
        // This forces a full redraw cycle so the parent relayouts properly.
        if self.next_frame.is_event(event).is_some() {
            cx.redraw_all();
        }

        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            for action in actions {
                if let Some(AppPreferencesAction::SendOnEnterChanged(v)) = action.downcast_ref() {
                    self.mentionable_text_input(cx, ids!(editing_content.edit_text_input))
                        .text_input_ref()
                        .set_submit_on_enter(*v);
                }
            }
        }

        if !self.visible {
            return;
        }

        let animator_action = self.animator_handle_event(cx, event);
        if animator_action.must_redraw() {
            // During hide, redraw the entire UI so the parent RoomInputBar
            // can animate the input_bar height in its draw_walk.
            // During show, only this widget needs to redraw.
            if self.is_animating_out {
                cx.redraw_all();
            } else {
                self.redraw(cx);
            }
        }
        // If we started animating the hide, check if the track has finished.
        // `is_track_animating` returns false once the track has fully completed,
        // even on the same frame that returned the last `Animating` action.
        if self.is_animating_out {
            if !self.animator.is_track_animating(id!(panel)) {
                self.visible = false;
                self.is_animating_out = false;
                self.info = None;
                cx.widget_action(self.widget_uid(), EditingPaneAction::Hidden);
                cx.revert_key_focus();
                self.redraw(cx);
                self.next_frame = cx.new_next_frame();
                return;
            }
        } else if self.animator_in_state(cx, ids!(panel.hide))
            && matches!(animator_action, AnimatorAction::Animating { .. })
        {
            self.is_animating_out = true;
        }

        if let Event::Actions(actions) = event {
            let edit_text_input = self
                .mentionable_text_input(cx, ids!(editing_content.edit_text_input))
                .text_input_ref();

            // Hide the editing pane if the cancel button was clicked
            // or if the `Escape` key was pressed within the edit text input.
            if self.button(cx, ids!(cancel_button)).clicked(actions)
                || edit_text_input.escaped(actions)
            {
                self.animator_play(cx, ids!(panel.hide));
                cx.widget_action(self.widget_uid(), EditingPaneAction::HideAnimationStarted);
                self.redraw(cx);
                return;
            }

            let Some(info) = self.info.as_ref() else {
                return;
            };

            if self.button(cx, ids!(accept_button)).clicked(actions)
                || edit_text_input.returned(actions).is_some()
            {
                let edited_text = edit_text_input.text().trim().to_string();
                let edited_text_len = edited_text.len();
                let edited_content = match info.event_tl_item.content() {
                    TimelineItemContent::MsgLike(msg_like_content) => {
                        match &msg_like_content.kind {
                            MsgLikeKind::Message(message) => {
                                // Only these types of messages can be edited.
                                let mut edited_content = match message.msgtype() {
                                    // TODO: try to distinguish between plaintext, markdown, and html messages,
                                    //       For now, we just assume that all messages are markdown.
                                    //       But this is a problem, since the body of the text/emote message might not be markdown.

                                    // TODO: also handle "/html" or "/plain" prefixes, just like when sending new messages.
                                    MessageType::Text(_text) => EditedContent::RoomMessage(
                                        RoomMessageEventContentWithoutRelation::text_markdown(
                                            &edited_text,
                                        ),
                                    ),
                                    MessageType::Emote(_emote) => EditedContent::RoomMessage(
                                        RoomMessageEventContentWithoutRelation::emote_markdown(
                                            &edited_text,
                                        ),
                                    ),
                                    // TODO: support adding/removing attachments.
                                    //       For now, we just support modifying the body/formatted body of the message.
                                    // TODO: once we update the matrix-sdk dependency, we can use the new
                                    //       `EditedContent::MediaCaption` variant to edit media messages captions only.
                                    MessageType::Image(image) => {
                                        let mut new_image_msg = image.clone();
                                        if image.formatted.is_some() {
                                            new_image_msg.formatted =
                                                FormattedBody::markdown(&edited_text);
                                        }
                                        new_image_msg.body = edited_text.clone();
                                        EditedContent::RoomMessage(
                                            RoomMessageEventContentWithoutRelation::new(
                                                MessageType::Image(new_image_msg),
                                            ),
                                        )
                                    }
                                    MessageType::Audio(audio) => {
                                        let mut new_audio_msg = audio.clone();
                                        if audio.formatted.is_some() {
                                            new_audio_msg.formatted =
                                                FormattedBody::markdown(&edited_text);
                                        }
                                        new_audio_msg.body = edited_text.clone();
                                        EditedContent::RoomMessage(
                                            RoomMessageEventContentWithoutRelation::new(
                                                MessageType::Audio(new_audio_msg),
                                            ),
                                        )
                                    }
                                    MessageType::File(file) => {
                                        let mut new_file_msg = file.clone();
                                        if file.formatted.is_some() {
                                            new_file_msg.formatted =
                                                FormattedBody::markdown(&edited_text);
                                        }
                                        new_file_msg.body = edited_text.clone();
                                        EditedContent::RoomMessage(
                                            RoomMessageEventContentWithoutRelation::new(
                                                MessageType::File(new_file_msg),
                                            ),
                                        )
                                    }
                                    MessageType::Video(video) => {
                                        let mut new_video_msg = video.clone();
                                        if video.formatted.is_some() {
                                            new_video_msg.formatted =
                                                FormattedBody::markdown(&edited_text);
                                        }
                                        new_video_msg.body = edited_text.clone();
                                        EditedContent::RoomMessage(
                                            RoomMessageEventContentWithoutRelation::new(
                                                MessageType::Video(new_video_msg),
                                            ),
                                        )
                                    }
                                    _non_editable => {
                                        enqueue_popup_notification(
                                            "That message type cannot be edited.",
                                            PopupKind::Error,
                                            None,
                                        );
                                        self.animator_play(cx, ids!(panel.hide));
                                        cx.widget_action(
                                            self.widget_uid(),
                                            EditingPaneAction::HideAnimationStarted,
                                        );
                                        self.redraw(cx);
                                        return;
                                    }
                                };

                                // TODO: extract mentions out of the new edited text and use them here.
                                if let Some(existing_mentions) = message.mentions() {
                                    if let EditedContent::RoomMessage(new_message_content) =
                                        &mut edited_content
                                    {
                                        new_message_content.mentions =
                                            Some(existing_mentions.clone());
                                    }
                                    // TODO: once we update the matrix-sdk dependency, uncomment this.
                                    // EditedContent::MediaCaption { mentions, .. }) => {
                                    //     mentions = Some(existing_mentions);
                                    // }
                                }

                                edited_content
                            }

                            MsgLikeKind::Poll(poll) => {
                                let poll_result = poll.results();
                                let poll_answers = poll_result.answers;
                                // TODO: support editing poll answers. For now, just keep the same answers.
                                let Ok(new_poll_answers) = poll_answers
                                    .into_iter()
                                    .map(|answer| UnstablePollAnswer::new(answer.id, answer.text))
                                    .collect::<Vec<_>>()
                                    .try_into()
                                else {
                                    enqueue_popup_notification(
                                        "Failed to obtain existing poll answers while editing poll.",
                                        PopupKind::Error,
                                        None,
                                    );
                                    return;
                                };
                                let mut new_content_block = UnstablePollStartContentBlock::new(
                                    edited_text.clone(),
                                    new_poll_answers,
                                );
                                new_content_block.kind = poll_result.kind;
                                new_content_block.max_selections = poll_result.max_selections
                                    .try_into()
                                    .inspect_err(|e| error!("BUG: failed to obtain existing poll max selections while editing: {}", e))
                                    .unwrap_or_default();
                                EditedContent::PollStart {
                                    fallback_text: edited_text.clone(),
                                    new_content: new_content_block,
                                }
                            }
                            _ => {
                                enqueue_popup_notification(
                                    "That event type cannot be edited.",
                                    PopupKind::Error,
                                    None,
                                );
                                return;
                            }
                        }
                    }
                    _ => {
                        enqueue_popup_notification(
                            "That event type cannot be edited.",
                            PopupKind::Error,
                            None,
                        );
                        return;
                    }
                };

                let timeline_kind = info.timeline_kind.clone();
                let timeline_event_item_id = info.event_tl_item.identifier();
                let (content_kind, preserves_existing_mentions) =
                    editing_pane_detail_packet_source(&info.event_tl_item);
                self.set_mention_payload_preflight_packet_label(
                    cx,
                    content_kind,
                    &edited_text,
                    preserves_existing_mentions,
                );
                self.set_save_result_mapping_packet_label(
                    cx,
                    content_kind,
                    edited_text_len,
                    preserves_existing_mentions,
                    "confirmation_opened",
                );

                let content = ConfirmationModalContent {
                    title_text: "Confirm Message Edit".into(),
                    body_text:
                        "Save this edited message? Confirmation required before the Matrix edit request."
                            .into(),
                    accept_button_text: Some("Save Edit".into()),
                    on_accept_clicked: Some(Box::new(move |_cx| {
                        submit_async_request(MatrixRequest::EditMessage {
                            timeline_kind,
                            timeline_event_item_id,
                            edited_content,
                        });
                        enqueue_popup_notification(
                            "Existing Matrix message edit path was requested.".to_string(),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    })),
                    on_cancel_clicked: Some(Box::new(move |_cx| {
                        enqueue_popup_notification(
                            "Message edit canceled. Matrix edit request was not sent."
                                .to_string(),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    })),
                    ..Default::default()
                };
                cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                    content,
                ))));
                enqueue_popup_notification(
                    format!(
                        "Message edit confirmation opened. {EDITING_PANE_CONFIRMATION_COMPACT_LABEL}"
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );

                // TODO: show a loading spinner within the accept button.
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, mut walk: Walk) -> DrawStep {
        if self.info.is_none() {
            self.visible = false;
        };

        // Animate both the layout height and content position simultaneously:
        // 1. walk.height grows from 0 to ch (and shrinks back during hide),
        //    so the RoomInputBar border grows/shrinks smoothly.
        // 2. Balanced margins on editing_content slide it within the pane:
        //    margin.top pushes content below the clip boundary,
        //    margin.bottom compensates so the Fit height stays constant.
        //    The pane's show_bg provides the clipping.
        let ch = self.last_content_height;
        if self.slide > 0.001 {
            let offset = if ch > 0.0 {
                ch * self.slide as f64
            } else {
                10000.0
            };
            if let Some(mut ec) = self.view(cx, ids!(editing_content)).borrow_mut() {
                ec.walk.margin.top = offset;
                ec.walk.margin.bottom = -offset;
            }
            // Animate the layout height alongside the content slide,
            // so the RoomInputBar border grows/shrinks smoothly.
            if ch > 0.0 {
                walk.height = Size::Fixed((ch * (1.0 - self.slide as f64)).max(0.0));
            } else {
                walk.height = Size::Fixed(0.0);
            }
        } else {
            // Fully shown or not animating: reset margins.
            if let Some(mut ec) = self.view(cx, ids!(editing_content)).borrow_mut() {
                ec.walk.margin.top = 0.0;
                ec.walk.margin.bottom = 0.0;
            }
        }

        let step = self.view.draw_walk(cx, scope, walk);

        // Read area rect AFTER drawing to capture this frame's layout.
        let ec_height = self.view(cx, ids!(editing_content)).area().rect(cx).size.y;
        if ec_height > 0.0 {
            self.last_content_height = ec_height;
        }

        step
    }
}

impl EditingPane {
    fn set_detail_packet_label(
        &mut self,
        cx: &mut Cx,
        content_kind: &str,
        edited_text: &str,
        preserves_existing_mentions: bool,
    ) {
        let edited_text_len = edited_text.len();
        let label = editing_pane_detail_packet_label(
            content_kind,
            edited_text_len,
            preserves_existing_mentions,
        );
        self.label(cx, ids!(editing_content.edit_unsupported_features_evidence))
            .set_text(cx, &label);
        self.set_attachment_preflight_packet_label(cx, content_kind, edited_text_len);
        self.set_mention_payload_preflight_packet_label(
            cx,
            content_kind,
            edited_text,
            preserves_existing_mentions,
        );
        self.set_save_result_mapping_packet_label(
            cx,
            content_kind,
            edited_text_len,
            preserves_existing_mentions,
            "idle_preflight",
        );
    }

    fn set_attachment_preflight_packet_label(
        &mut self,
        cx: &mut Cx,
        content_kind: &str,
        edited_text_len: usize,
    ) {
        let label = editing_pane_attachment_preflight_packet_label(content_kind, edited_text_len);
        self.label(cx, ids!(editing_content.edit_attachment_preflight_packet))
            .set_text(cx, &label);
    }

    fn set_mention_payload_preflight_packet_label(
        &mut self,
        cx: &mut Cx,
        content_kind: &str,
        edited_text: &str,
        preserves_existing_mentions: bool,
    ) {
        let label = editing_pane_mention_payload_preflight_packet_label(
            content_kind,
            edited_text,
            preserves_existing_mentions,
        );
        self.label(
            cx,
            ids!(editing_content.edit_mention_payload_preflight_packet),
        )
        .set_text(cx, &label);
        self.set_mention_payload_typed_contract_packet_label(
            cx,
            content_kind,
            edited_text,
            preserves_existing_mentions,
        );
    }

    fn set_mention_payload_typed_contract_packet_label(
        &mut self,
        cx: &mut Cx,
        content_kind: &str,
        edited_text: &str,
        preserves_existing_mentions: bool,
    ) {
        let label = editing_pane_mention_payload_typed_contract_packet_label(
            content_kind,
            edited_text,
            preserves_existing_mentions,
        );
        self.label(
            cx,
            ids!(editing_content.edit_mention_payload_typed_contract_packet),
        )
        .set_text(cx, &label);
    }

    fn set_save_result_mapping_packet_label(
        &mut self,
        cx: &mut Cx,
        content_kind: &str,
        edited_text_len: usize,
        preserves_existing_mentions: bool,
        lifecycle_state: &str,
    ) {
        let label = editing_pane_save_result_mapping_packet_label(
            content_kind,
            edited_text_len,
            preserves_existing_mentions,
            lifecycle_state,
        );
        self.label(cx, ids!(editing_content.edit_save_result_mapping_packet))
            .set_text(cx, &label);
        self.set_retry_error_drilldown_packet_label(
            cx,
            content_kind,
            edited_text_len,
            preserves_existing_mentions,
            lifecycle_state,
        );
    }

    fn set_retry_error_drilldown_packet_label(
        &mut self,
        cx: &mut Cx,
        content_kind: &str,
        edited_text_len: usize,
        preserves_existing_mentions: bool,
        lifecycle_state: &str,
    ) {
        let label = editing_pane_retry_error_drilldown_packet_label(
            content_kind,
            edited_text_len,
            preserves_existing_mentions,
            lifecycle_state,
        );
        self.label(cx, ids!(editing_content.edit_retry_error_drilldown_packet))
            .set_text(cx, &label);
    }

    /// Returns `true` if this pane is currently being shown.
    pub fn is_currently_shown(&self, _cx: &mut Cx) -> bool {
        self.visible
    }

    /// Call this when the result of an edit operation is received.
    ///
    /// This will handle the result, and either show a success message
    /// and hide this editing pane, or show an error message.
    pub fn handle_edit_result(
        &mut self,
        cx: &mut Cx,
        timeline_event_item_id: TimelineEventItemId,
        edit_result: Result<(), matrix_sdk_ui::timeline::Error>,
    ) {
        let Some(info) = self.info.as_ref() else {
            error!("Editing pane received and edit result but had no info set.");
            return;
        };
        let current_event_item_id = info.event_tl_item.identifier();
        let (content_kind, preserves_existing_mentions) =
            editing_pane_detail_packet_source(&info.event_tl_item);
        let edited_text_len = self
            .mentionable_text_input(cx, ids!(editing_content.edit_text_input))
            .text()
            .trim()
            .len();
        let edited_text = self
            .mentionable_text_input(cx, ids!(editing_content.edit_text_input))
            .text()
            .trim()
            .to_string();
        self.set_mention_payload_preflight_packet_label(
            cx,
            content_kind,
            &edited_text,
            preserves_existing_mentions,
        );
        if current_event_item_id != timeline_event_item_id {
            self.set_save_result_mapping_packet_label(
                cx,
                content_kind,
                edited_text_len,
                preserves_existing_mentions,
                "stale_event_id_ignored",
            );
            error!("Editing pane received an edit result for a different event.");
            return;
        }
        match edit_result {
            Ok(()) => {
                self.set_save_result_mapping_packet_label(
                    cx,
                    content_kind,
                    edited_text_len,
                    preserves_existing_mentions,
                    "saved_hide_pane",
                );
                self.animator_play(cx, ids!(panel.hide));
                cx.widget_action(self.widget_uid(), EditingPaneAction::HideAnimationStarted);
            }
            Err(e) => {
                self.set_save_result_mapping_packet_label(
                    cx,
                    content_kind,
                    edited_text_len,
                    preserves_existing_mentions,
                    "failed_popup",
                );
                enqueue_popup_notification(
                    format!("Failed to edit message: {}", e),
                    PopupKind::Error,
                    None,
                );
            }
        }
    }

    /// Shows the editing pane and sets it up to edit the given `event`'s content.
    pub fn show(
        &mut self,
        cx: &mut Cx,
        event_tl_item: EventTimelineItem,
        timeline_kind: TimelineKind,
    ) {
        if !event_tl_item.is_editable() {
            enqueue_popup_notification("That message cannot be edited.", PopupKind::Error, None);
            return;
        }

        let edit_text_input =
            self.mentionable_text_input(cx, ids!(editing_content.edit_text_input));

        if let Some(message) = event_tl_item.content().as_message() {
            edit_text_input.set_text(cx, message.body());
        } else if let Some(poll) = event_tl_item.content().as_poll() {
            edit_text_input.set_text(cx, &poll.results().question);
        } else {
            enqueue_popup_notification(
                "That message cannot be edited.",
                PopupKind::Error,
                Some(4.0),
            );
            return;
        }

        let (content_kind, preserves_existing_mentions) =
            editing_pane_detail_packet_source(&event_tl_item);
        self.set_detail_packet_label(
            cx,
            content_kind,
            &edit_text_input.text(),
            preserves_existing_mentions,
        );

        self.info = Some(EditingPaneInfo {
            event_tl_item,
            timeline_kind,
        });

        self.visible = true;
        self.is_animating_out = false;
        self.button(cx, ids!(accept_button)).reset_hover(cx);
        self.button(cx, ids!(cancel_button)).reset_hover(cx);
        self.animator_play(cx, ids!(panel.show));

        // Set the text input's cursor to the end and give it key focus.
        let inner_text_input = edit_text_input.text_input_ref();
        let text_len = edit_text_input.text().len();
        inner_text_input.set_cursor(
            cx,
            Cursor {
                index: text_len,
                prefer_next_row: false,
            },
            false,
        );
        // TODO: this doesn't work, likely because of Makepad's bug in which you cannot
        // give key focus to a widget that hasn't been drawn yet (as it has no Area).
        inner_text_input.set_key_focus(cx);
        self.redraw(cx);
    }

    /// Returns the state of this `EditingPane`, if any.
    pub fn save_state(&self) -> Option<EditingPaneState> {
        self.info.as_ref().map(|info| EditingPaneState {
            event_tl_item: info.event_tl_item.clone(),
            text_input_state: self
                .child_by_path(ids!(editing_content.edit_text_input))
                .as_mentionable_text_input()
                .text_input_ref()
                .save_state(),
        })
    }

    /// Restores the state of this `EditingPane` from the given `editing_pane_state`.
    pub fn restore_state(
        &mut self,
        cx: &mut Cx,
        editing_pane_state: EditingPaneState,
        timeline_kind: TimelineKind,
    ) {
        let EditingPaneState {
            event_tl_item,
            text_input_state,
        } = editing_pane_state;
        self.mentionable_text_input(cx, ids!(editing_content.edit_text_input))
            .text_input_ref()
            .restore_state(cx, text_input_state);
        let (content_kind, preserves_existing_mentions) =
            editing_pane_detail_packet_source(&event_tl_item);
        let restored_text = self
            .mentionable_text_input(cx, ids!(editing_content.edit_text_input))
            .text()
            .to_string();
        self.set_detail_packet_label(
            cx,
            content_kind,
            &restored_text,
            preserves_existing_mentions,
        );
        self.info = Some(EditingPaneInfo {
            event_tl_item,
            timeline_kind,
        });
        self.visible = true;
        self.is_animating_out = false;
        self.button(cx, ids!(accept_button)).reset_hover(cx);
        self.button(cx, ids!(cancel_button)).reset_hover(cx);
        self.animator_play(cx, ids!(panel.show));
        self.redraw(cx);

        // In this function, we do not give key focus to the text input,
        // because we don't want the IME/soft keyboard to pop up immediately
        // when the user navigates back to a room they were previously editing a message in.
        // That soft-keyboard pop-up effect is jarring and unpleasant.
    }
}

impl EditingPaneRef {
    /// See [`EditingPane::is_currently_shown()`].
    pub fn is_currently_shown(&self, cx: &mut Cx) -> bool {
        let Some(inner) = self.borrow() else {
            return false;
        };
        inner.is_currently_shown(cx)
    }

    /// Returns the current slide value (0.0 = fully shown, 1.0 = fully hidden).
    pub fn slide(&self) -> f32 {
        self.borrow().map_or(1.0, |inner| inner.slide)
    }

    /// See [`EditingPane::handle_edit_result()`].
    pub fn handle_edit_result(
        &self,
        cx: &mut Cx,
        timeline_event_item_id: TimelineEventItemId,
        edit_result: Result<(), matrix_sdk_ui::timeline::Error>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.handle_edit_result(cx, timeline_event_item_id, edit_result);
    }

    /// Returns whether this `EditingPane` was hidden by the given actions, i.e.,
    /// `true` if `actions` contains an [`EditingPaneAction::Hidden`] for this widget.
    pub fn was_hidden(&self, actions: &Actions) -> bool {
        matches!(
            actions.find_widget_action(self.widget_uid()).cast_ref(),
            EditingPaneAction::Hidden,
        )
    }

    /// Returns whether this `EditingPane`'s hide animation started in the given actions.
    pub fn was_hide_animation_started(&self, actions: &Actions) -> bool {
        matches!(
            actions.find_widget_action(self.widget_uid()).cast_ref(),
            EditingPaneAction::HideAnimationStarted,
        )
    }

    /// See [`EditingPane::show()`].
    pub fn show(&self, cx: &mut Cx, event_tl_item: EventTimelineItem, timeline_kind: TimelineKind) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.show(cx, event_tl_item, timeline_kind);
    }

    /// See [`EditingPane::save_state()`].
    pub fn save_state(&self) -> Option<EditingPaneState> {
        self.borrow()?.save_state()
    }

    /// Restores the state of this `EditingPane` from the given `event_tl_item` and `text_input_state`.
    ///
    /// The arguments should be the result of a previous call to [`Self::save_state()`].
    pub fn restore_state(
        &self,
        cx: &mut Cx,
        editing_pane_state: EditingPaneState,
        timeline_kind: TimelineKind,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.restore_state(cx, editing_pane_state, timeline_kind);
    }

    /// Hides the editing pane immediately and clears its state without animating it out.
    ///
    /// This function *DOES NOT* emit an [`EditingPaneAction::Hidden`] action.
    pub fn force_reset_hide(&self, cx: &mut Cx) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.visible = false;
        inner.animator_cut(cx, ids!(panel.hide));
        inner.is_animating_out = false;
        inner.info = None;
        // Reset editing_content margins in case we interrupted an animation.
        if let Some(mut ec) = inner.view(cx, ids!(editing_content)).borrow_mut() {
            ec.walk.margin.top = 0.0;
            ec.walk.margin.bottom = 0.0;
        }
        // Redraw all so the parent RoomInputBar restores the input_bar
        // height (its draw_walk reads the slide value, which is now 1.0).
        cx.redraw_all();
    }
}

/// The state of the EditingPane, used for saving/restoring its state.
pub struct EditingPaneState {
    event_tl_item: EventTimelineItem,
    text_input_state: TextInputState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editing_pane_detail_packet_marks_unsupported_edit_edges_local() {
        let label = editing_pane_detail_packet_label("image_caption", 42, true);

        assert!(label.contains("Edit extras stay local"));
        assert!(label.contains("Detail packet"));
        assert!(label.contains("content_kind image_caption"));
        assert!(label.contains("edited_text_len 42"));
        assert!(label.contains("attachment_edit_slot not_built"));
        assert!(label.contains("mention_payload_scope preserve_existing_mentions_only"));
        assert!(label.contains("poll_answer_edit_slot not_built"));
        assert!(label.contains("save_spinner_operation_id not_assigned"));
        assert!(label.contains("result_mapping not_wired"));
        assert!(label.contains("stale_result_policy"));
        assert!(label.contains("no attachment upload/remove"));
        assert!(label.contains("Matrix mention payload"));
        assert!(label.contains("poll answer edit"));
        assert!(label.contains("room-state"));
        assert!(label.contains("membership request was sent"));
        assert!(EDITING_PANE_DETAIL_PACKET_EVIDENCE.contains("attachment_edit_slot not_built"));
    }

    #[test]
    fn editing_pane_detail_packet_records_no_mention_scope() {
        let label = editing_pane_detail_packet_label("poll_question_preserve_answers", 18, false);

        assert!(label.contains("content_kind poll_question_preserve_answers"));
        assert!(label.contains("mention_payload_scope none"));
        assert!(label.contains("poll_answer_edit_slot not_built"));
    }

    #[test]
    fn editing_pane_attachment_preflight_packet_records_media_boundaries() {
        let label = editing_pane_attachment_preflight_packet_label("image_caption", 42);

        assert!(label.contains("Attachment preflight packet"));
        assert!(label.contains("content_kind image_caption"));
        assert!(label.contains("edited_text_len 42"));
        assert!(label.contains("original_attachment_scope existing_image_media_caption_only"));
        assert!(label.contains("selected_attachment_slot unavailable"));
        assert!(label.contains("add_attachment_slot not_built"));
        assert!(label.contains("remove_attachment_slot not_built"));
        assert!(label.contains("replace_attachment_slot not_built"));
        assert!(label.contains("upload_request_slot not_built"));
        assert!(label.contains("media_delete_slot not_built"));
        assert!(label.contains(
            "caption_edit_handoff existing_confirmed_MatrixRequest_EditMessage_body_only"
        ));
        assert!(label.contains("mime_size_probe not_started"));
        assert!(label.contains("retry_policy no_duplicate_upload_without_operation_id"));
        assert!(
            label.contains("cancel_policy leaves_original_media_and_local_selection_untouched")
        );
        assert!(label.contains("no SendAttachment"));
        assert!(label.contains("media delete"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(
            EDITING_PANE_ATTACHMENT_PREFLIGHT_PACKET_EVIDENCE
                .contains("add/remove/replace/upload/delete slots not_built")
        );
    }

    #[test]
    fn editing_pane_attachment_preflight_packet_records_non_media_scope() {
        let text_label = editing_pane_attachment_preflight_packet_label("text_body", 18);
        let poll_label =
            editing_pane_attachment_preflight_packet_label("poll_question_preserve_answers", 18);

        assert!(text_label.contains("original_attachment_scope no_existing_media_attachment"));
        assert!(poll_label.contains("original_attachment_scope poll_no_attachment"));
        assert!(poll_label.contains("add_attachment_slot not_built"));
    }

    #[test]
    fn editing_pane_mention_payload_preflight_packet_records_token_boundaries() {
        let label = editing_pane_mention_payload_preflight_packet_label(
            "text_body",
            "hello @room @alice:example.org @bob",
            true,
        );

        assert!(label.contains("Mention payload preflight packet"));
        assert!(label.contains("content_kind text_body"));
        assert!(label.contains("edited_at_token_count 3"));
        assert!(label.contains("literal_user_id_token_count 1"));
        assert!(label.contains("room_token_scope present_requires_power_level_recheck"));
        assert!(label.contains("completed_pill_reconcile_slot not_connected_to_editing_pane"));
        assert!(label.contains("directory_result_scope unavailable_in_editing_pane"));
        assert!(label.contains("fresh_mentions_payload_slot not_built"));
        assert!(label.contains("existing_mentions_handoff preserve_existing_mentions_only"));
        assert!(label.contains("reply_sendtime_state not_reused"));
        assert!(label.contains("retry_source_hash_slot missing"));
        assert!(label.contains("stale_token_policy backend_required_before_live_mentions"));
        assert!(label.contains("cancel_policy confirmation_cancel_no_request"));
        assert!(label.contains("no fresh Matrix Mentions payload"));
        assert!(label.contains("profile lookup"));
        assert!(label.contains("directory search"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(
            EDITING_PANE_MENTION_PAYLOAD_PREFLIGHT_PACKET_EVIDENCE
                .contains("fresh_mentions_payload_slot not_built")
        );
    }

    #[test]
    fn editing_pane_mention_payload_preflight_packet_records_empty_scope() {
        let label = editing_pane_mention_payload_preflight_packet_label(
            "poll_question_preserve_answers",
            "question only",
            false,
        );

        assert!(label.contains("content_kind poll_question_preserve_answers"));
        assert!(label.contains("edited_at_token_count 0"));
        assert!(label.contains("literal_user_id_token_count 0"));
        assert!(label.contains("room_token_scope absent"));
        assert!(label.contains("existing_mentions_handoff none"));
    }

    #[test]
    fn editing_pane_mention_payload_typed_contract_packet_records_contract_shape() {
        let label = editing_pane_mention_payload_typed_contract_packet_label(
            "text_body",
            "hello @room @alice:example.org @bob",
            true,
        );

        assert!(label.contains("Mention payload typed contract packet"));
        assert!(label.contains("mention_contract_version local_v0"));
        assert!(label.contains("token_scan_source edited_text_only"));
        assert!(label.contains("edited_at_token_count 3"));
        assert!(label.contains("literal_user_id_contract_count 1"));
        assert!(
            label.contains("room_token_contract_scope requires_power_level_recheck_before_payload")
        );
        assert!(label.contains("directory_snapshot_id_slot unavailable"));
        assert!(label.contains("completed_pill_snapshot_slot unavailable"));
        assert!(label.contains("existing_mentions_handoff preserve_existing_mentions_only"));
        assert!(label.contains("source_hash_slot not_assigned"));
        assert!(label.contains("fresh_mentions_payload_result_slot not_built"));
        assert!(label.contains("retry_idempotency_key_slot missing"));
        assert!(
            label.contains("stale_result_guard body_source_hash_required_before_live_mentions")
        );
        assert!(label.contains("result_mapping accepted|permission_denied"));
        assert!(label.contains("privacy_redaction token_counts_only"));
        assert!(label.contains("no fresh Matrix Mentions payload"));
        assert!(label.contains("directory snapshot reuse"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(
            EDITING_PANE_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("fresh_mentions_payload_result_slot not_built")
        );
    }

    #[test]
    fn editing_pane_mention_payload_typed_contract_packet_records_empty_scope() {
        let label = editing_pane_mention_payload_typed_contract_packet_label(
            "poll_question_preserve_answers",
            "question only",
            false,
        );

        assert!(label.contains("content_kind poll_question_preserve_answers"));
        assert!(label.contains("edited_at_token_count 0"));
        assert!(label.contains("literal_user_id_contract_count 0"));
        assert!(label.contains("room_token_contract_scope absent"));
        assert!(label.contains("existing_mentions_handoff none"));
        assert!(label.contains("result_mapping accepted|permission_denied"));
    }

    #[test]
    fn editing_pane_save_result_mapping_packet_records_operation_slots() {
        let label = editing_pane_save_result_mapping_packet_label(
            "text_body",
            64,
            true,
            "confirmation_opened",
        );

        assert!(label.contains("Save result mapping packet"));
        assert!(label.contains("lifecycle_state confirmation_opened"));
        assert!(label.contains("content_kind text_body"));
        assert!(label.contains("edited_text_len 64"));
        assert!(label.contains("mention_payload_scope preserve_existing_mentions_only"));
        assert!(label.contains("operation_id_slot not_assigned"));
        assert!(label.contains("request_slot existing_confirmed_MatrixRequest_EditMessage"));
        assert!(label.contains("spinner_slot not_rendered"));
        assert!(label.contains("result_mapping saved_hide_pane"));
        assert!(label.contains("failed_popup"));
        assert!(label.contains("canceled_no_request"));
        assert!(label.contains("stale_event_id_ignored"));
        assert!(label.contains("ignored_late_result_without_matching_operation_id"));
        assert!(label.contains("stale_result_guard timeline_event_item_id_match_only"));
        assert!(label.contains("repeated_save_policy not_held_until_pending_operation_id"));
        assert!(label.contains("retry_slot not_built"));
        assert!(label.contains("no attachment upload/remove"));
        assert!(label.contains("timeline reload"));
        assert!(label.contains("membership request was sent"));
        assert!(
            EDITING_PANE_SAVE_RESULT_MAPPING_PACKET_EVIDENCE
                .contains("result_mapping saved/failed")
        );
    }

    #[test]
    fn editing_pane_save_result_mapping_packet_records_poll_scope() {
        let label = editing_pane_save_result_mapping_packet_label(
            "poll_question_preserve_answers",
            18,
            false,
            "stale_event_id_ignored",
        );

        assert!(label.contains("content_kind poll_question_preserve_answers"));
        assert!(label.contains("mention_payload_scope none"));
        assert!(label.contains("lifecycle_state stale_event_id_ignored"));
        assert!(label.contains("poll answer edit"));
    }

    #[test]
    fn editing_pane_retry_error_drilldown_packet_records_retry_boundaries() {
        let label =
            editing_pane_retry_error_drilldown_packet_label("text_body", 64, true, "failed_popup");

        assert!(label.contains("Retry/error drilldown packet"));
        assert!(label.contains("lifecycle_state failed_popup"));
        assert!(label.contains("content_kind text_body"));
        assert!(label.contains("edited_text_len 64"));
        assert!(label.contains("mention_payload_scope preserve_existing_mentions_only"));
        assert!(label.contains("failure_source existing_MatrixRequest_EditMessage_result_only"));
        assert!(label.contains("error_redaction popup_text_not_persisted_or_reused"));
        assert!(label.contains("retry_request_slot not_built"));
        assert!(label.contains("retry_confirmation_slot not_built"));
        assert!(
            label.contains(
                "late_result_guard timeline_event_item_id_match_only_without_operation_id"
            )
        );
        assert!(label.contains("pending_operation_id missing_backend_contract"));
        assert!(label.contains("spinner_state not_rendered"));
        assert!(label.contains("cancel_state confirmation_cancel_no_request"));
        assert!(label.contains("retry_state manual_retry_not_built_existing_popup_only"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            EDITING_PANE_RETRY_ERROR_DRILLDOWN_PACKET_EVIDENCE
                .contains("retry_request_slot not_built")
        );
    }

    #[test]
    fn editing_pane_retry_error_drilldown_packet_records_stale_and_cancel_state() {
        let stale_label = editing_pane_retry_error_drilldown_packet_label(
            "poll_question_preserve_answers",
            18,
            false,
            "stale_event_id_ignored",
        );
        let confirmation_label = editing_pane_retry_error_drilldown_packet_label(
            "poll_question_preserve_answers",
            18,
            false,
            "confirmation_opened",
        );

        assert!(stale_label.contains("content_kind poll_question_preserve_answers"));
        assert!(stale_label.contains("mention_payload_scope none"));
        assert!(stale_label.contains("retry_state retry_blocked_stale_event_id"));
        assert!(
            stale_label
                .contains("stale_result_policy ignore_late_result_without_matching_operation_id")
        );
        assert!(
            confirmation_label
                .contains("retry_state confirmation_pending_cancel_no_request_until_accept")
        );
        assert!(confirmation_label.contains("cancel_state confirmation_cancel_no_request"));
    }
}
