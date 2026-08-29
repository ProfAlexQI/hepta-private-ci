//! Timeline message widget and its widget actions.

use super::*;

/// Actions related to a specific message within a room timeline.
#[derive(Clone, Default, Debug)]
pub enum MessageAction {
    /// The user clicked the "react" button on a message
    /// and wants to send the given `reaction` to that message.
    React {
        details: MessageDetails,
        reaction: String,
    },
    /// The user clicked the "reply" button on a message.
    Reply(MessageDetails),
    /// The user clicked the "reply in thread" button on a message, indicating
    /// they want to open (or start) that message's thread and reply within it.
    ReplyInThread(MessageDetails),
    /// The user clicked the "edit" button on a message.
    Edit(MessageDetails),
    /// The user requested to edit their latest message in this room.
    EditLatest,
    /// The user clicked the "pin" button on a message.
    Pin(MessageDetails),
    /// The user clicked the "unpin" button on a message.
    Unpin(MessageDetails),
    /// The user clicked the "copy text" button on a message.
    CopyText(MessageDetails),
    /// The user clicked the "copy HTML" button on a message.
    CopyHtml(MessageDetails),
    /// The user clicked the "copy link" button on a message.
    CopyLink(MessageDetails),
    /// The user clicked the "view source" button on a message.
    ViewSource(MessageDetails),
    /// The user clicked the "jump to related" button on a message,
    /// indicating that they want to auto-scroll back to the related message,
    /// e.g., a replied-to message.
    JumpToRelated(MessageDetails),
    /// The user clicked the "Show more" or "Show less" button on a tall reply preview.
    ToggleReplyPreviewExpanded(TimelineEventItemId),
    /// The user clicked the thread summary on a thread-root message.
    OpenThread(OwnedEventId),
    /// The user requested to jump to a specific event in this room.
    JumpToEvent(OwnedEventId),
    /// The user clicked the "delete" button on a message.
    #[doc(alias("delete"))]
    Redact {
        details: MessageDetails,
        reason: Option<String>,
    },

    // /// The user clicked the "report" button on a message.
    // Report(MessageDetails),
    /// The user clicked the "Download" button on a media/file message.
    DownloadAttachment(DownloadableAttachment),
    /// The user clicked the "Share" button on a media/file message.
    ShareAttachment(DownloadableAttachment),
    /// User clicked the cancel × next to the in-progress spinner.
    CancelDownload(OwnedMxcUri),
    /// The message at the given item index in the timeline should be highlighted.
    HighlightMessage(usize),
    /// The user requested that we show a context menu with actions
    /// that can be performed on a given message.
    OpenMessageContextMenu {
        details: MessageDetails,
        /// The absolute position where we should show the context menu,
        /// in which the (0,0) origin coordinate is the top left corner of the app window.
        abs_pos: DVec2,
    },
    /// The user requested opening the message action bar
    ActionBarOpen {
        /// At the given timeline item index
        item_id: usize,
        /// The message rect, so the action bar can be positioned relative to it
        message_rect: Rect,
    },
    /// The user requested closing the message action bar
    ActionBarClose,
    #[default]
    None,
}
impl ActionDefaultRef for MessageAction {
    fn default_ref() -> &'static Self {
        static DEFAULT: MessageAction = MessageAction::None;
        &DEFAULT
    }
}

/// A widget representing a single message of any kind within a room timeline.
#[derive(Script, Widget, Animator)]
pub struct Message {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[apply_default]
    animator: Animator,

    #[rust]
    details: Option<MessageDetails>,
    /// Set on file/image/audio/video messages so the download button knows
    /// what to save when the user clicks it. `None` for plain text messages,
    /// which hide the download button entirely.
    #[rust]
    download_info: Option<DownloadableAttachment>,
    /// Cached so `set_data` can reset_hover only on the button that just
    /// transitioned into visibility, not on every redraw.
    #[rust]
    download_state: DownloadDisplayState,

    // Belowhere: cached references to child widgets, for efficiency.
    #[rust]
    replied_to_message_view: Option<CollapsiblePreviewRef>,
    #[rust]
    thread_root_summary_view: Option<ViewRef>,
}

impl ScriptHook for Message {
    fn on_after_reload(&mut self, _vm: &mut ScriptVm) {
        // A script reload changes the Message's children; invalidate the ones we cached.
        self.replied_to_message_view = None;
        self.thread_root_summary_view = None;
    }
}

impl Widget for Message {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if !self.animator.is_track_animating(id!(highlight))
            && self.animator_in_state(cx, ids!(highlight.on))
        {
            self.animator_play(cx, ids!(highlight.off));
        }

        let Some(d) = self.details.as_ref() else {
            return;
        };
        let room_screen_widget_uid = d.room_screen_widget_uid;
        let thread_root_event_id = d.thread_root_event_id.clone();

        // We first handle a click on the replied-to message preview, if present,
        // because we don't want any widgets within the replied-to message to be
        // clickable or otherwise interactive.
        let reply = self.replied_to_message_view(cx);
        let reply_content_area = reply.content_area(cx);
        match event.hits(cx, reply_content_area) {
            Hit::FingerHoverIn(..) => {
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_fho) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerDown(fe) if fe.device.mouse_button().is_some_and(|b| b.is_secondary()) => {
                cx.widget_action(
                    room_screen_widget_uid,
                    MessageAction::OpenMessageContextMenu {
                        details: self.details.clone().unwrap(), // guaranteed to be Some()
                        abs_pos: fe.abs,
                    },
                );
            }
            Hit::FingerLongPress(lp) => {
                cx.widget_action(
                    room_screen_widget_uid,
                    MessageAction::OpenMessageContextMenu {
                        details: self.details.clone().unwrap(), // guaranteed to be Some()
                        abs_pos: lp.abs,
                    },
                );
            }
            Hit::FingerUp(fe) if fe.is_over && fe.is_primary_hit() && fe.was_tap() => {
                // Tapping on a collapsed reply preview expands it.
                // Tapping on an expanded reply preview jumps to the replied-to message.
                let action = if reply.is_collapsed() {
                    MessageAction::ToggleReplyPreviewExpanded(
                        self.details.as_ref().unwrap().timeline_event_id.clone(), // guaranteed to be Some()
                    )
                } else {
                    MessageAction::JumpToRelated(self.details.clone().unwrap()) // guaranteed to be Some()
                };
                cx.widget_action(room_screen_widget_uid, action);
            }
            _ => {}
        }

        // Handle clicks on the thread summary shown beneath a thread-root message.
        if let Some(thread_root_event_id) = thread_root_event_id.as_ref() {
            let thread_root_summary = self.thread_root_summary_view(cx);
            let apply_hover = |cx: &mut Cx, bg_color: Vec4| {
                let mut thread_root_summary_ref = thread_root_summary.clone();
                script_apply_eval!(cx, thread_root_summary_ref, {
                    draw_bg.color: #(bg_color)
                });
            };
            match event.hits(cx, thread_root_summary.area()) {
                Hit::FingerDown(fe) => {
                    apply_hover(cx, COLOR_THREAD_SUMMARY_BG_HOVER);
                    if fe.device.mouse_button().is_some_and(|b| b.is_secondary()) {
                        cx.widget_action(
                            room_screen_widget_uid,
                            MessageAction::OpenMessageContextMenu {
                                details: self.details.clone().unwrap(), // guaranteed to be Some()
                                abs_pos: fe.abs,
                            },
                        );
                    }
                }
                Hit::FingerHoverIn(_) => {
                    apply_hover(cx, COLOR_THREAD_SUMMARY_BG_HOVER);
                }
                Hit::FingerHoverOut(_) => {
                    apply_hover(cx, COLOR_THREAD_SUMMARY_BG);
                }
                Hit::FingerLongPress(lp) => {
                    cx.widget_action(
                        room_screen_widget_uid,
                        MessageAction::OpenMessageContextMenu {
                            details: self.details.clone().unwrap(), // guaranteed to be Some()
                            abs_pos: lp.abs,
                        },
                    );
                }
                Hit::FingerUp(fe) => {
                    apply_hover(cx, COLOR_THREAD_SUMMARY_BG);
                    if fe.is_over && fe.is_primary_hit() && fe.was_tap() {
                        cx.widget_action(
                            room_screen_widget_uid,
                            MessageAction::OpenThread(thread_root_event_id.clone()),
                        );
                    }
                }
                _ => {}
            }
        }

        // Next, we forward the event to the child view such that it has the chance
        // to handle it before the Message widget handles it.
        // This ensures that events like right-clicking/long-pressing a reaction button
        // or a link within a message will be treated as an action upon that child view
        // rather than an action upon the message itself.
        self.view.handle_event(cx, event, scope);

        // Finally, handle any hits on the rest of the message body itself.
        let message_view_area = self.view.area();
        match event.hits(cx, message_view_area) {
            Hit::FingerDown(fe) => {
                cx.set_key_focus(message_view_area);
                // A right click means we should display the context menu.
                if fe.device.mouse_button().is_some_and(|b| b.is_secondary()) {
                    cx.widget_action(
                        room_screen_widget_uid,
                        MessageAction::OpenMessageContextMenu {
                            details: self.details.clone().unwrap(), // guaranteed to be Some()
                            abs_pos: fe.abs,
                        },
                    );
                }
            }
            Hit::FingerLongPress(lp) => {
                cx.widget_action(
                    room_screen_widget_uid,
                    MessageAction::OpenMessageContextMenu {
                        details: self.details.clone().unwrap(), // guaranteed to be Some()
                        abs_pos: lp.abs,
                    },
                );
            }
            Hit::FingerHoverIn(..) => {
                self.animator_play(cx, ids!(hover.on));
                // TODO: here, show the "action bar" buttons upon hover-in
            }
            Hit::FingerHoverOut(_fho) => {
                self.animator_play(cx, ids!(hover.off));
                // TODO: here, hide the "action bar" buttons upon hover-out
            }
            _ => {}
        }

        if let Event::Actions(actions) = event {
            for action in actions {
                match action
                    .as_widget_action()
                    .widget_uid_eq(room_screen_widget_uid)
                    .cast_ref()
                {
                    MessageAction::HighlightMessage(id)
                        if id == &self.details.as_ref().unwrap().item_id =>
                    {
                        // guaranteed to be Some()
                        self.animator_play(cx, ids!(highlight.on));
                        self.redraw(cx);
                    }
                    _ => {}
                }
            }

            // Handle clicks on the reply preview's "show more" or "show less" buttons.
            let reply_expand_button = self.button(cx, ids!(replied_to_message.reply_expand_button));
            let reply_collapse_button =
                self.button(cx, ids!(replied_to_message.reply_collapse_button));
            if reply_expand_button.clicked(actions) || reply_collapse_button.clicked(actions) {
                cx.widget_action(
                    room_screen_widget_uid,
                    MessageAction::ToggleReplyPreviewExpanded(
                        self.details.as_ref().unwrap().timeline_event_id.clone(), // guaranteed to be Some()
                    ),
                );
                reply_expand_button.reset_hover(cx);
                reply_collapse_button.reset_hover(cx);
            }

            // Handle clicks on the media-related buttons (download, share, cancel) beneath media messages.
            if let Some(info) = self.download_info.as_ref() {
                if self
                    .view
                    .button(cx, ids!(content.download_section.download_button))
                    .clicked(actions)
                {
                    cx.widget_action(
                        room_screen_widget_uid,
                        MessageAction::DownloadAttachment(info.clone()),
                    );
                }
                if self
                    .view
                    .button(cx, ids!(content.download_section.share_button))
                    .clicked(actions)
                {
                    cx.widget_action(
                        room_screen_widget_uid,
                        MessageAction::ShareAttachment(info.clone()),
                    );
                }
                if self
                    .view
                    .button(
                        cx,
                        ids!(content.download_section.downloading_view.cancel_button),
                    )
                    .clicked(actions)
                {
                    cx.widget_action(
                        room_screen_widget_uid,
                        MessageAction::CancelDownload(media_source_mxc(&info.media_source).clone()),
                    );
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self
            .details
            .as_ref()
            .is_some_and(|d| d.should_be_highlighted)
        {
            script_apply_eval!(cx, self, {
                draw_bg +: {
                    color: #ffffd1,
                    mentions_bar_color: #ffd54f
                }
            });
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl Message {
    fn replied_to_message_view(&mut self, cx: &mut Cx) -> CollapsiblePreviewRef {
        if let Some(reply) = &self.replied_to_message_view {
            return reply.clone();
        }
        let reply = self
            .view
            .widget(cx, ids!(replied_to_message))
            .as_collapsible_preview();
        self.replied_to_message_view = Some(reply.clone());
        reply
    }

    fn thread_root_summary_view(&mut self, cx: &mut Cx) -> ViewRef {
        if let Some(view) = &self.thread_root_summary_view {
            return view.clone();
        }
        let view = self.view(cx, ids!(thread_root_summary));
        self.thread_root_summary_view = Some(view.clone());
        view
    }

    /// Called every time `populate_message_view` runs, including on cached
    /// items, so all states must be re-set unconditionally.
    fn set_data(
        &mut self,
        cx: &mut Cx,
        details: MessageDetails,
        download_info: Option<DownloadableAttachment>,
        download_state: DownloadDisplayState,
        is_reply_expanded: bool,
    ) {
        let prev_section_visible = self.download_info.is_some();
        let prev_state = self.download_state;

        self.details = Some(details);
        self.download_info = download_info;

        // Re-apply this every time to ensure a re-used portallist item is still correctly expanded.
        self.view
            .widget(cx, ids!(replied_to_message))
            .as_collapsible_preview()
            .set_expanded(is_reply_expanded);

        let section_visible = self.download_info.is_some();
        self.view
            .view(cx, ids!(content.download_section))
            .set_visible(cx, section_visible);
        if section_visible {
            let download_button = self
                .view
                .button(cx, ids!(content.download_section.download_button));
            let share_button = self
                .view
                .button(cx, ids!(content.download_section.share_button));
            let downloading_view = self
                .view
                .view(cx, ids!(content.download_section.downloading_view));
            let cancel_button = self.view.button(
                cx,
                ids!(content.download_section.downloading_view.cancel_button),
            );
            let success_button = self
                .view
                .button(cx, ids!(content.download_section.success_button));
            let failure_button = self
                .view
                .button(cx, ids!(content.download_section.failure_button));
            let is_idle = matches!(download_state, DownloadDisplayState::Idle);
            download_button.set_visible(cx, is_idle);
            share_button.set_visible(cx, is_idle);
            downloading_view.set_visible(
                cx,
                matches!(download_state, DownloadDisplayState::InProgress),
            );
            success_button.set_visible(
                cx,
                matches!(download_state, DownloadDisplayState::Succeeded(_)),
            );
            failure_button.set_visible(cx, matches!(download_state, DownloadDisplayState::Failed));
            if let DownloadDisplayState::Succeeded(kind) = download_state {
                success_button.set_text(
                    cx,
                    match kind {
                        TransferKind::Download => "Downloaded",
                        TransferKind::Share => "Shared",
                    },
                );
            }
            // Only reset hover for the button(s) just now becoming visible.
            let newly_visible = !prev_section_visible || prev_state != download_state;
            if newly_visible {
                match download_state {
                    DownloadDisplayState::Idle => {
                        download_button.reset_hover(cx);
                        share_button.reset_hover(cx);
                    }
                    DownloadDisplayState::InProgress => cancel_button.reset_hover(cx),
                    DownloadDisplayState::Succeeded(_) => success_button.reset_hover(cx),
                    DownloadDisplayState::Failed => failure_button.reset_hover(cx),
                }
            }
        }
        self.download_state = download_state;
    }
}

impl MessageRef {
    pub(super) fn set_data(
        &self,
        cx: &mut Cx,
        details: MessageDetails,
        download_info: Option<DownloadableAttachment>,
        download_state: DownloadDisplayState,
        is_reply_expanded: bool,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.set_data(
            cx,
            details,
            download_info,
            download_state,
            is_reply_expanded,
        );
    }
}
