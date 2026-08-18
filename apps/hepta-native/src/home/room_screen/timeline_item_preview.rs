use super::*;

/// Draws a ReplyPreview above a message if it was in-reply to another message.
///
/// ## Arguments
/// * `replied_to_message_view`: the destination `RepliedToMessage` view that will be populated.
/// * `timeline_kind`: the [`TimelineKind`] of the timeline that is being drawn.
/// * `in_reply_to`: if `Some`, the details that will be used to populate the `replied_to_message_view`.
///   If `None`, this function will mark it as non-visible and consider it fully drawn.
/// * `message_event_id`: the [`EventId`] of the message that is the reply itself (the response).
///   This is needed to fetch the details of the replied-to message (if not yet available).
///
/// Returns whether the in-reply-to information was available and fully drawn,
/// i.e., whether it can be considered cached and not needing to be redrawn later.
pub(super) fn draw_replied_to_message(
    cx: &mut Cx2d,
    replied_to_message_view: &WidgetRef,
    timeline_kind: &TimelineKind,
    in_reply_to: Option<&InReplyToDetails>,
    message_event_id: Option<&EventId>,
) -> bool {
    let fully_drawn: bool;
    let show_reply: bool;

    if let Some(in_reply_to_details) = in_reply_to {
        show_reply = true;
        match &in_reply_to_details.event {
            TimelineDetails::Ready(replied_to_event) => {
                let (in_reply_to_username, is_avatar_fully_drawn) =
                    replied_to_message_view
                        .avatar(cx, ids!(preview_content.reply_preview_avatar))
                        .set_avatar_and_get_username(
                            cx,
                            timeline_kind,
                            &replied_to_event.sender,
                            Some(&replied_to_event.sender_profile),
                            Some(in_reply_to_details.event_id.as_ref()),
                            true,
                        );

                fully_drawn = is_avatar_fully_drawn;

                replied_to_message_view
                    .label(cx, ids!(preview_content.reply_preview_username))
                    .set_text(cx, in_reply_to_username.as_str());
                let msg_body = replied_to_message_view.html_or_plaintext(cx, ids!(reply_preview_body));
                populate_preview_of_timeline_item(
                    cx,
                    &msg_body,
                    &replied_to_event.content,
                    &replied_to_event.sender,
                    &in_reply_to_username,
                );
            }
            TimelineDetails::Error(_e) => {
                fully_drawn = true;
                replied_to_message_view
                    .label(cx, ids!(preview_content.reply_preview_username))
                    .set_text(cx, "[Error fetching username]");
                replied_to_message_view
                    .avatar(cx, ids!(preview_content.reply_preview_avatar))
                    .show_text(cx, None, None, "?");
                replied_to_message_view
                    .html_or_plaintext(cx, ids!(preview_content.reply_preview_body))
                    .show_plaintext(cx, "[Error fetching replied-to event]");
            }
            td @ TimelineDetails::Pending | td @ TimelineDetails::Unavailable => {
                // We don't have the replied-to message yet, so we can't fully draw the preview.
                fully_drawn = false;
                replied_to_message_view
                    .label(cx, ids!(preview_content.reply_preview_username))
                    .set_text(cx, "[Loading username...]");
                replied_to_message_view
                    .avatar(cx, ids!(preview_content.reply_preview_avatar))
                    .show_text(cx, None, None, "?");
                replied_to_message_view
                    .html_or_plaintext(cx, ids!(preview_content.reply_preview_body))
                    .show_plaintext(cx, "[Loading replied-to message...]");

                // Confusingly, we need to fetch the details of the `message` (the event that is the reply),
                // not the details of the original event that this `message` is replying to.
                if matches!(td, TimelineDetails::Unavailable) {
                    if let Some(event_id) = message_event_id {
                        submit_async_request(MatrixRequest::FetchDetailsForEvent {
                            timeline_kind: timeline_kind.clone(),
                            event_id: event_id.to_owned(),
                        });
                    }
                }
            }
        }
    } else {
        // This message was not in reply to another message, so we don't need to show a reply.
        show_reply = false;
        fully_drawn = true;
    }

    replied_to_message_view.set_visible(cx, show_reply);
    // After we changed a reply preview's content, we need to clear its cached view and measured height.
    replied_to_message_view.view(cx, ids!(preview_content)).redraw_texture_cache();
    replied_to_message_view.as_collapsible_preview().reset_measured_height();
    fully_drawn
}

/// Draws a one-line thread summary at the bottom of a message if it is the root of a thread.
///
/// Returns whether the thread summary information was available and fully drawn,
/// i.e., whether it can be considered cached and not needing to be redrawn later.
pub(super) fn populate_thread_root_summary(
    cx: &mut Cx2d,
    item: &WidgetRef,
    timeline_item_index: usize,
    timeline_kind: &TimelineKind,
    msg_like_content: &MsgLikeContent,
    event_tl_item: &EventTimelineItem,
    fetched_thread_summaries: &HashMap<OwnedEventId, FetchedThreadSummary>,
    pending_thread_summary_fetches: &mut HashSet<OwnedEventId>,
) -> bool {
    let thread_summary_view = item.view(cx, ids!(thread_root_summary));
    thread_summary_view.set_visible(cx, false); // hide by default
    let fully_drawn: bool;

    if matches!(timeline_kind, TimelineKind::Thread { .. }) {
        // If we're already drawing a message in a thread-focused timeline,
        // it doesn't make sense to show a redundant thread summary.
        fully_drawn = true;
        return fully_drawn;
    }

    let Some(thread_summary) = msg_like_content.thread_summary.as_ref() else {
        // consider this as fully drawn since there's no thread summary to show.
        fully_drawn = true;
        return fully_drawn;
    };

    // Here, we actually need to show the thread summary.
    thread_summary_view.set_visible(cx, true);
    let local_num_replies = thread_summary.num_replies;
    let thread_root_event_id = event_tl_item.event_id().map(|id| id.to_owned());
    let fetched_summary = thread_root_event_id
        .as_ref()
        .and_then(|root_id| fetched_thread_summaries.get(root_id));
    let replies_count = fetched_summary
        .map(|f| f.num_replies)
        .unwrap_or(local_num_replies);

    let latest_preview: Cow<str> = match &thread_summary.latest_event {
        TimelineDetails::Ready(embedded_event) => {
            fully_drawn = true;
            let sender_username = match &embedded_event.sender_profile {
                TimelineDetails::Ready(profile) => profile
                    .display_name
                    .as_deref()
                    .unwrap_or(embedded_event.sender.as_str()),
                _ => embedded_event.sender.as_str(),
            };
            let preview = text_preview_of_timeline_item(
                &embedded_event.content,
                &embedded_event.sender,
                sender_username,
            ).format_with(sender_username, true);
            match utils::replace_linebreaks_separators(&preview, true) {
                Cow::Borrowed(_) => Cow::Owned(preview),
                Cow::Owned(replaced) => Cow::Owned(replaced),
            }
        }
        td @ TimelineDetails::Pending | td @ TimelineDetails::Unavailable => {
            fully_drawn = true;
            if td.is_unavailable()
                && let Some(thread_root_event_id) = thread_root_event_id.clone()
            {
                let needs_refresh = fetched_summary
                    .is_none_or(|fs| fs.latest_reply_preview_text.is_none());
                if needs_refresh && pending_thread_summary_fetches.insert(thread_root_event_id.clone()) {
                    let accepted = submit_async_request(MatrixRequest::FetchThreadSummaryDetails {
                        timeline_kind: timeline_kind.clone(),
                        thread_root_event_id: thread_root_event_id.clone(),
                        timeline_item_index,
                    }).was_accepted();
                    if !accepted {
                        pending_thread_summary_fetches.remove(&thread_root_event_id);
                    }
                }
            }
            fetched_summary.and_then(|fs| fs.latest_reply_preview_text.as_deref())
                .unwrap_or("<i>Loading latest reply...</i>")
                .into()
        }
        TimelineDetails::Error(_) => {
            fully_drawn = true; // consider this fully drawn since there's no point retrying.
            "<i>Unable to load latest reply</i>".into()
        }
    };

    let replies_count_text = match replies_count {
        1 => Cow::Borrowed("1 reply"),
        n => Cow::Owned(format!("{n} replies"))
    };
    item.label(cx, ids!(thread_summary_count))
        .set_text(cx, &replies_count_text);
    item.html(cx, ids!(thread_summary_latest))
        .set_text(cx, &latest_preview);
    fully_drawn
}

/// Generates a rich HTML text preview of the given `timeline_item_content`
/// and populates the given `widget_out` with that content.
pub fn populate_preview_of_timeline_item(
    cx: &mut Cx,
    widget_out: &HtmlOrPlaintextRef,
    timeline_item_content: &TimelineItemContent,
    sender_user_id: &UserId,
    sender_username: &str,
) {
    if let Some(m) = timeline_item_content.as_message() {
        match m.msgtype() {
            MessageType::Text(TextMessageEventContent { body, formatted, .. })
            | MessageType::Notice(NoticeMessageEventContent { body, formatted, .. }) => {
                let _ = populate_text_message_content(cx, widget_out, body, formatted.as_ref(), None, None, None, None);
                return;
            }
            _ => { } // fall through to the general case for all timeline items below.
        }
    }
    let html = text_preview_of_timeline_item(
        timeline_item_content,
        sender_user_id,
        sender_username,
    ).format_with(sender_username, true);
    widget_out.show_html(cx, html);
}


/// A trait for abstracting over the different types of timeline events
/// that can be displayed in a `SmallStateEvent` widget.
pub(super) trait SmallStateEventContent {
    /// Populates the *content* (not the profile) of the given `item` with data from
    /// the given `event_tl_item` and `self` (the specific type of event content).
    ///
    /// ## Arguments
    /// * `item`: a `SmallStateEvent` widget that has already been added to
    ///   the given `PortalList` at the given `item_id`.
    ///   This function may either modify that item or completely replace it
    ///   with a different widget if needed.
    /// * `item_drawn_status`: the old (prior) drawn status of the item.
    /// * `new_drawn_status`: the new drawn status of the item, which may have already
    ///   been updated to reflect the item's profile having been drawn right before this function.
    ///
    /// ## Return
    /// Returns a tuple of the drawn `item` and its `new_drawn_status`.
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        list: &mut PortalList,
        item_id: usize,
        item: WidgetRef,
        event_tl_item: &EventTimelineItem,
        username: &str,
        item_drawn_status: ItemDrawnStatus,
        new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus);
}

// For unable to decrypt messages.
impl SmallStateEventContent for EncryptedMessage {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content)).set_text(
            cx,
            &text_preview_of_encrypted_message(self).format_with(username, false),
        );
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

// For other message-like content (custom message-like events).
impl SmallStateEventContent for LiveLocationState {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content)).set_text(
            cx,
            &format!("{username} shared a live location."),
        );
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

impl SmallStateEventContent for OtherMessageLike {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content)).set_text(
            cx,
            &text_preview_of_other_message_like(self).format_with(username, false),
        );
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

// TODO: once we properly display polls, we should remove this,
//       because Polls shouldn't be displayed using the SmallStateEvent widget.
impl SmallStateEventContent for PollState {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        _username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content)).set_text(
            cx,
            self.fallback_text().unwrap_or_else(|| self.results().question).as_str(),
        );
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

impl SmallStateEventContent for timeline::OtherState {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        list: &mut PortalList,
        item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        let item = if let Some(text_preview) = text_preview_of_other_state(self, false) {
            item.label(cx, ids!(content))
                .set_text(cx, &text_preview.format_with(username, false));
            new_drawn_status.content_drawn = true;
            item
        } else {
            let item = list.item(cx, item_id, id!(Empty));
            new_drawn_status = ItemDrawnStatus::new();
            item
        };
        (item, new_drawn_status)
    }
}

impl SmallStateEventContent for MemberProfileChange {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content)).set_text(
            cx,
            &text_preview_of_member_profile_change(self, username, false)
                .format_with(username, false),
        );
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

impl SmallStateEventContent for RoomMembershipChange {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        list: &mut PortalList,
        item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        let Some(preview) = text_preview_of_room_membership_change(self, false) else {
            // Don't actually display anything for nonexistent/unimportant membership changes.
            return (
                list.item(cx, item_id, id!(Empty)),
                ItemDrawnStatus::new(),
            );
        };

        item.label(cx, ids!(content))
            .set_text(cx, &preview.format_with(username, false));

        // The invite_user_button is only used for "Knocked" membership change events.
        item.button(cx, ids!(invite_user_button)).set_visible(
            cx,
            matches!(self.change(), Some(MembershipChange::Knocked)),
        );

        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

/// Creates, populates, and adds a SmallStateEvent liveview widget to the given `PortalList`
/// with the given `item_id`.
///
/// The content of the returned widget is populated with data from the
/// given room membership change and its parent `EventTimelineItem`.
pub(super) fn populate_small_state_event(
    cx: &mut Cx,
    list: &mut PortalList,
    item_id: usize,
    timeline_kind: &TimelineKind,
    event_tl_item: &EventTimelineItem,
    event_content: &impl SmallStateEventContent,
    item_drawn_status: ItemDrawnStatus,
) -> (WidgetRef, ItemDrawnStatus) {
    let mut new_drawn_status = item_drawn_status;
    let (item, existed) = list.item_with_existed(cx, item_id, id!(SmallStateEvent));
    // The content of a small state event view may depend on the profile info,
    // so we can only mark the content as drawn after the profile has been fully drawn and cached.
    let skip_redrawing_profile = existed && item_drawn_status.profile_drawn;
    let skip_redrawing_content = skip_redrawing_profile && item_drawn_status.content_drawn;
    populate_read_receipts(&item, cx, timeline_kind, event_tl_item);
    if skip_redrawing_content {
        return (item, new_drawn_status);
    }

    // If the profile has been drawn, we can just quickly grab the user's display name
    // instead of having to call `set_avatar_and_get_username` again.
    let username_opt = skip_redrawing_profile
        .then(|| get_profile_display_name(event_tl_item))
        .flatten();

    let username = username_opt.unwrap_or_else(|| {
        // As a fallback, call `set_avatar_and_get_username` to get the user's display name.
        let avatar_ref = item.avatar(cx, ids!(avatar));

        let (username, profile_drawn) = avatar_ref.set_avatar_and_get_username(
            cx,
            timeline_kind,
            event_tl_item.sender(),
            Some(event_tl_item.sender_profile()),
            event_tl_item.event_id(),
            true,
        );
        // Draw the timestamp as part of the profile.
        if let Some(dt) = unix_time_millis_to_datetime(event_tl_item.timestamp()) {
            item.timestamp(cx, ids!(left_container.timestamp)).set_date_time(cx, dt);
        }
        new_drawn_status.profile_drawn = profile_drawn;
        username
    });

    // Proceed to draw the actual event content.
    event_content.populate_item_content(
        cx,
        list,
        item_id,
        item,
        event_tl_item,
        &username,
        item_drawn_status,
        new_drawn_status,
    )
}


/// Returns the display name of the sender of the given `event_tl_item`, if available.
fn get_profile_display_name(event_tl_item: &EventTimelineItem) -> Option<String> {
    if let TimelineDetails::Ready(profile) = event_tl_item.sender_profile() {
        profile.display_name.clone()
    } else {
        None
    }
}
