impl ActionDefaultRef for MessageAction {
    fn default_ref() -> &'static Self {
        static DEFAULT: MessageAction = MessageAction::None;
        &DEFAULT
    }
}

/// A widget representing a single message of any kind within a room timeline.
#[derive(Script, ScriptHook, Widget, Animator)]
pub struct Message {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[apply_default]
    animator: Animator,

    #[rust]
    details: Option<MessageDetails>,
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

        let Some(details) = self.details.clone() else {
            return;
        };

        // We first handle a click on the replied-to message preview, if present,
        // because we don't want any widgets within the replied-to message to be
        // clickable or otherwise interactive.
        match event.hits(cx, self.view(cx, ids!(replied_to_message)).area()) {
            Hit::FingerDown(fe) if fe.device.mouse_button().is_some_and(|b| b.is_secondary()) => {
                cx.widget_action(
                    details.room_screen_widget_uid,
                    MessageAction::OpenMessageContextMenu {
                        details: details.clone(),
                        abs_pos: fe.abs,
                    },
                );
            }
            Hit::FingerLongPress(lp) => {
                cx.widget_action(
                    details.room_screen_widget_uid,
                    MessageAction::OpenMessageContextMenu {
                        details: details.clone(),
                        abs_pos: lp.abs,
                    },
                );
            }
            // If the hit occurred on the replied-to message preview, jump to it.
            Hit::FingerUp(fe) if fe.is_over && fe.is_primary_hit() && fe.was_tap() => {
                cx.widget_action(
                    details.room_screen_widget_uid,
                    MessageAction::JumpToRelated(details.clone()),
                );
            }
            _ => {}
        }

        // Handle clicks on the thread summary shown beneath a thread-root message.
        if let Some(thread_root_event_id) = details.thread_root_event_id.as_ref() {
            let thread_root_summary = self.view(cx, ids!(thread_root_summary));
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
                            details.room_screen_widget_uid,
                            MessageAction::OpenMessageContextMenu {
                                details: details.clone(),
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
                        details.room_screen_widget_uid,
                        MessageAction::OpenMessageContextMenu {
                            details: details.clone(),
                            abs_pos: lp.abs,
                        },
                    );
                }
                Hit::FingerUp(fe) => {
                    apply_hover(cx, COLOR_THREAD_SUMMARY_BG);
                    if fe.is_over && fe.is_primary_hit() && fe.was_tap() {
                        cx.widget_action(
                            details.room_screen_widget_uid,
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
                        details.room_screen_widget_uid,
                        MessageAction::OpenMessageContextMenu {
                            details: details.clone(),
                            abs_pos: fe.abs,
                        },
                    );
                }
            }
            Hit::FingerLongPress(lp) => {
                cx.widget_action(
                    details.room_screen_widget_uid,
                    MessageAction::OpenMessageContextMenu {
                        details: details.clone(),
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
                    .widget_uid_eq(details.room_screen_widget_uid)
                    .cast_ref()
                {
                    MessageAction::HighlightMessage(id) if id == &details.item_id => {
                        self.animator_play(cx, ids!(highlight.on));
                        self.redraw(cx);
                    }
                    _ => {}
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
    fn set_data(&mut self, details: MessageDetails) {
        self.details = Some(details);
    }
}

impl MessageRef {
    fn set_data(&self, details: MessageDetails) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.set_data(details);
    }
}

fn compact_loaded_message_search_preview(body: &str) -> String {
    compact_message_preview(body, "empty message preview")
}

fn loaded_message_search_query_lifecycle_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    surface_visible: bool,
    timeline_loaded: bool,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!(
            "query {} chars, normalized local token \"{}\"",
            query.chars().count(),
            query.to_lowercase()
        )
    };
    let surface_state = if surface_visible {
        "surface visible"
    } else {
        "surface hidden"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let active_state = if match_count == 0 {
        "active index reset to 0 with no local match".to_string()
    } else {
        format!(
            "active index {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };

    format!(
        "Search query lifecycle metadata: {surface_state}; {query_state}; {timeline_state}; {loaded_item_count} loaded items; {match_count} local matches; {active_state}. Query edits reset active_match to 0 and rescan loaded tl_state only; Close/Escape clears query and matches locally. {MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_LABEL} {MESSAGE_SEARCH_COMPACT_LABEL} No Matrix-backed search, server-side history query, event context, pagination/reload, room preview fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_metadata_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    active_loaded_index: Option<usize>,
    active_event_id_loaded: bool,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "no active match".to_string()
    } else if let Some(loaded_index) = active_loaded_index {
        let active_ordinal = active_match.saturating_add(1).min(match_count);
        let event_id_state = if active_event_id_loaded {
            "event id loaded"
        } else {
            "event id missing"
        };
        format!(
            "active {active_ordinal}/{match_count} at loaded index {loaded_index}, {event_id_state}"
        )
    } else {
        let active_ordinal = active_match.saturating_add(1).min(match_count);
        format!("active {active_ordinal}/{match_count} missing loaded item, event id missing")
    };

    format!(
        "Loaded search metadata: {query_state}, {loaded_item_count} loaded items, {match_count} matches, {active_state}. {MESSAGE_SEARCH_LOADED_METADATA_LABEL}"
    )
}

fn loaded_message_search_occurrence_count(body: &str, query: &str) -> usize {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return 0;
    }
    body.to_lowercase().match_indices(&needle).count()
}

fn loaded_message_search_active_result_label(
    query: &str,
    match_count: usize,
    active_match: usize,
    active_loaded_index: Option<usize>,
    active_event_id: Option<&str>,
    loaded_body: Option<&str>,
) -> String {
    let query = query.trim();
    if query.is_empty() {
        return format!(
            "Active result detail: query empty; no selected loaded message. {MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_LABEL}"
        );
    }
    if match_count == 0 {
        return format!(
            "Active result detail: no loaded local match for \"{query}\"; Close/Escape stays local. {MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_LABEL}"
        );
    }

    let active_ordinal = active_match.saturating_add(1).min(match_count);
    let loaded_index_state = active_loaded_index
        .map(|index| format!("loaded index {index}"))
        .unwrap_or_else(|| "loaded index missing".to_string());
    let event_id_state = active_event_id
        .filter(|event_id| !event_id.trim().is_empty())
        .map(|event_id| format!("event id {event_id}"))
        .unwrap_or_else(|| "event id missing".to_string());
    let query_chars = query.chars().count();
    let (occurrence_count, snippet) = loaded_body
        .map(|body| {
            (
                loaded_message_search_occurrence_count(body, query),
                compact_loaded_message_search_preview(body),
            )
        })
        .unwrap_or_else(|| (0, "loaded snippet unavailable".to_string()));

    format!(
        "Active result detail: active {active_ordinal}/{match_count}, {loaded_index_state}, {event_id_state}, query {query_chars} chars, {occurrence_count} local occurrence(s), snippet: {snippet}. {MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_LABEL}"
    )
}

fn loaded_message_search_thread_root_event_id(
    event_tl_item: &EventTimelineItem,
) -> Option<OwnedEventId> {
    let TimelineItemContent::MsgLike(msg_like_content) = event_tl_item.content() else {
        return None;
    };
    msg_like_content.thread_root.clone().or_else(|| {
        msg_like_content
            .thread_summary
            .as_ref()
            .and_then(|_| event_tl_item.event_id().map(|id| id.to_owned()))
    })
}

fn loaded_message_search_result_action_controls_label(
    action: Option<&str>,
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    active_loaded_index: Option<usize>,
    active_event_id_loaded: bool,
    loaded_body: Option<&str>,
) -> String {
    let action_state = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} selected"))
        .unwrap_or_else(|| "no result action selected".to_string());
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "no active loaded match".to_string()
    } else {
        format!(
            "active {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let loaded_index_state = active_loaded_index
        .map(|index| format!("loaded index {index}"))
        .unwrap_or_else(|| "loaded index missing".to_string());
    let event_id_state = if active_event_id_loaded {
        "event id loaded"
    } else {
        "event id missing"
    };
    let snippet = loaded_body
        .filter(|body| !body.trim().is_empty())
        .map(compact_loaded_message_search_preview)
        .unwrap_or_else(|| "loaded snippet unavailable".to_string());

    format!(
        "Search result-action controls: {action_state}; loaded handoffs for controls Jump, Copy, Source, Thread, Sender; {query_state}; {loaded_item_count} loaded items; {active_state}; {loaded_index_state}; {event_id_state}; snippet: {snippet}. {MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_LABEL} No remote event context window fetch, new Matrix-backed search, pagination/reload beyond current-room context, message mutation, profile mutation, room-state, membership mutation, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_result_jump_loaded_match_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    active_loaded_index: Option<usize>,
    event_id_loaded: bool,
    loaded_body: Option<&str>,
    jumped: bool,
) -> String {
    let query_state = if query.trim().is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "no active loaded match".to_string()
    } else {
        format!("active {} of {}", active_match + 1, match_count)
    };
    let loaded_index_state = active_loaded_index
        .map(|index| format!("loaded index {index}"))
        .unwrap_or_else(|| "loaded index missing".to_string());
    let event_id_state = if event_id_loaded {
        "event id loaded"
    } else {
        "event id missing"
    };
    let jump_state = if jumped {
        "scrolled and queued highlight"
    } else {
        "jump unavailable"
    };
    let snippet = loaded_body
        .filter(|body| !body.trim().is_empty())
        .map(compact_loaded_message_search_preview)
        .unwrap_or_else(|| "loaded snippet unavailable".to_string());
    format!(
        "Search result Jump handoff: {jump_state}; {query_state}; {loaded_item_count} loaded items; {active_state}; {loaded_index_state}; {event_id_state}; snippet: {snippet}. {MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_LABEL} No Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, thread timeline open, sender/profile lookup, message mutation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_result_thread_open_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    active_loaded_index: Option<usize>,
    event_id_loaded: bool,
    loaded_body: Option<&str>,
    thread_root_event_id: Option<&str>,
    opened: bool,
) -> String {
    let query_state = if query.trim().is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "no active loaded match".to_string()
    } else {
        format!("active {} of {}", active_match + 1, match_count)
    };
    let loaded_index_state = active_loaded_index
        .map(|index| format!("loaded index {index}"))
        .unwrap_or_else(|| "loaded index missing".to_string());
    let event_id_state = if event_id_loaded {
        "event id loaded"
    } else {
        "event id missing"
    };
    let thread_state = thread_root_event_id
        .filter(|event_id| !event_id.trim().is_empty())
        .map(|event_id| format!("thread root {event_id}"))
        .unwrap_or_else(|| "thread root unavailable".to_string());
    let open_state = if opened {
        "thread timeline selected"
    } else {
        "thread open unavailable"
    };
    let snippet = loaded_body
        .filter(|body| !body.trim().is_empty())
        .map(compact_loaded_message_search_preview)
        .unwrap_or_else(|| "loaded snippet unavailable".to_string());
    format!(
        "Search result Thread handoff: {open_state}; {query_state}; {loaded_item_count} loaded items; {active_state}; {loaded_index_state}; {event_id_state}; {thread_state}; snippet: {snippet}. {MESSAGE_SEARCH_RESULT_THREAD_OPEN_LABEL} Uses RoomsListAction::Selected(SelectedRoom::Thread) and the existing CreateThreadTimeline read/open path when needed. No Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, sender/profile lookup, message mutation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_result_sender_profile_pane_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    active_loaded_index: Option<usize>,
    event_id_loaded: bool,
    loaded_body: Option<&str>,
    sender_id: Option<&str>,
    sender_display_name: Option<&str>,
    sender_profile_ready: bool,
    room_member_loaded: bool,
    opened: bool,
) -> String {
    let query_state = if query.trim().is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "no active loaded match".to_string()
    } else {
        format!("active {} of {}", active_match + 1, match_count)
    };
    let loaded_index_state = active_loaded_index
        .map(|index| format!("loaded index {index}"))
        .unwrap_or_else(|| "loaded index missing".to_string());
    let event_id_state = if event_id_loaded {
        "event id loaded"
    } else {
        "event id missing"
    };
    let sender_state = sender_id
        .filter(|sender_id| !sender_id.trim().is_empty())
        .map(|sender_id| format!("sender {sender_id}"))
        .unwrap_or_else(|| "sender unavailable".to_string());
    let display_state = sender_display_name
        .filter(|display_name| !display_name.trim().is_empty())
        .map(|display_name| format!("display \"{display_name}\""))
        .unwrap_or_else(|| "display unavailable".to_string());
    let profile_state = if sender_profile_ready {
        "loaded sender_profile ready"
    } else {
        "sender_profile not ready"
    };
    let member_state = if room_member_loaded {
        "local room member loaded"
    } else {
        "local room member missing; profile pane may use existing GetUserProfile read path"
    };
    let open_state = if opened {
        "profile pane opened"
    } else {
        "profile pane unavailable"
    };
    let snippet = loaded_body
        .filter(|body| !body.trim().is_empty())
        .map(compact_loaded_message_search_preview)
        .unwrap_or_else(|| "loaded snippet unavailable".to_string());
    format!(
        "Search result Sender handoff: {open_state}; {query_state}; {loaded_item_count} loaded items; {active_state}; {loaded_index_state}; {event_id_state}; {sender_state}; {display_state}; {profile_state}; {member_state}; snippet: {snippet}. {MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_LABEL} Reuses UserProfileSlidingPane and the existing user_profile_cache/GetUserProfile profile-member read path when needed. No Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, message mutation, profile mutation, direct-message start, room-state, membership mutation, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_result_copy_clipboard_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    active_loaded_index: Option<usize>,
    active_event_id_loaded: bool,
    loaded_body: Option<&str>,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "no active loaded match".to_string()
    } else {
        format!(
            "active {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let loaded_index_state = active_loaded_index
        .map(|index| format!("loaded index {index}"))
        .unwrap_or_else(|| "loaded index missing".to_string());
    let event_id_state = if active_event_id_loaded {
        "event id loaded"
    } else {
        "event id missing"
    };
    let (payload_state, char_count, byte_count, snippet) = loaded_body
        .map(|body| {
            (
                "copied loaded plaintext",
                body.chars().count(),
                body.len(),
                compact_loaded_message_search_preview(body),
            )
        })
        .unwrap_or((
            "copy unavailable",
            0,
            0,
            "loaded snippet unavailable".to_string(),
        ));

    format!(
        "Search result Copy clipboard handoff: {payload_state}; {query_state}; {loaded_item_count} loaded items; {active_state}; {loaded_index_state}; {event_id_state}; {char_count} chars; {byte_count} bytes; snippet: {snippet}. {MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_LABEL} No Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source open, thread timeline open, sender/profile lookup, message mutation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_result_source_modal_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    active_loaded_index: Option<usize>,
    event_id_loaded: bool,
    latest_json: Option<&str>,
    server_source_json: Option<&str>,
    source_opened: bool,
    source_fetch_requested: bool,
) -> String {
    let query_state = if query.trim().is_empty() {
        "query empty".to_string()
    } else {
        format!("query {query:?}")
    };
    let active_state = if match_count == 0 {
        "no active match".to_string()
    } else {
        format!("active match {} of {}", active_match + 1, match_count)
    };
    let loaded_index_state = active_loaded_index
        .map(|index| format!("loaded index {index}"))
        .unwrap_or_else(|| "no loaded index".to_string());
    let event_id_state = if event_id_loaded {
        "event id loaded"
    } else {
        "event id unavailable"
    };
    let loaded_json_available = latest_json.is_some_and(|json| !json.trim().is_empty());
    let server_json_available = server_source_json.is_some_and(|json| !json.trim().is_empty());
    let source_state = if source_opened && loaded_json_available {
        "loaded source modal opened"
    } else if source_opened && server_json_available {
        "server-result source modal opened"
    } else if source_fetch_requested {
        "source-only MatrixRequest::FetchEventSource requested"
    } else {
        "source unavailable; latest_json, cached server source, and current-room event id missing"
    };
    let json_state = latest_json
        .filter(|json| !json.trim().is_empty())
        .map(|json| {
            format!(
                "latest JSON {} chars across {} lines",
                json.chars().count(),
                json.lines().count().max(1)
            )
        })
        .or_else(|| {
            server_source_json
                .filter(|json| !json.trim().is_empty())
                .map(|json| {
                    format!(
                        "server result JSON {} chars across {} lines",
                        json.chars().count(),
                        json.lines().count().max(1)
                    )
                })
        })
        .unwrap_or_else(|| {
            if source_fetch_requested {
                "source fetch pending".to_string()
            } else {
                "latest JSON unavailable".to_string()
            }
        });
    format!(
        "Search result Source modal handoff: {source_state}; {query_state}; {loaded_item_count} loaded items; {active_state}; {loaded_index_state}; {event_id_state}; {json_state}. {MESSAGE_SEARCH_RESULT_SOURCE_MODAL_LABEL} Source may request only MatrixRequest::FetchEventSource for current-room event JSON; no new Matrix-backed search, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, thread timeline open, sender/profile lookup, message mutation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_server_context_boundary_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    timeline_loaded: bool,
    has_next_batch: bool,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let cursor_state = if has_next_batch {
        "next_batch cursor available"
    } else {
        "next_batch cursor unavailable"
    };
    format!(
        "Server/context boundary: {query_state}, {loaded_item_count} loaded items, {match_count} local matches, {timeline_state}, {cursor_state}. {MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_LABEL} Context uses cached current-room server hit events with BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline; Source can open cached Matrix /search raw event JSON or request source-only MatrixRequest::FetchEventSource; cross-room context, room preview fetch, remote date/pins/scope adapters, full result rendering, message mutation, gateway/runtime/auth, and live mutation remain blocked."
    )
}

fn loaded_message_search_server_context_controls_label(
    action: Option<&str>,
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    timeline_loaded: bool,
    has_next_batch: bool,
) -> String {
    let action_state = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} selected"))
        .unwrap_or_else(|| "no server/context control selected".to_string());
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "active index reset to 0 with no local match".to_string()
    } else {
        format!(
            "active index {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let cursor_state = if has_next_batch {
        "Older can submit next_batch"
    } else {
        "Older waits for next_batch"
    };

    format!(
        "Search server/context controls: {action_state}; {query_state}; {loaded_item_count} loaded items; {match_count} local matches; {active_state}; {timeline_state}; {cursor_state}. {MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_LABEL} Context reuses BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline for cached current-room hits; Source can open cached Matrix /search raw event JSON or request source-only MatrixRequest::FetchEventSource; cross-room context, room preview fetch, remote date/pins/scope adapters, full result rendering, message mutation, gateway/runtime/auth, and live mutation remain blocked."
    )
}

fn message_search_server_context_event_unavailable_label(
    query: &str,
    pending: bool,
    hit_count: usize,
    reason: &str,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let pending_state = if pending {
        "server search pending"
    } else {
        "server search idle"
    };
    let reason = compact_message_preview(reason, "reason unavailable");

    format!(
        "Search server Context unavailable: {query_state}; {pending_state}; {hit_count} cached server hit(s); reason {reason}. Run Server first and wait for a current-room hit with an event id. Server result rows can surface parsed Matrix /search context-window previews when available; Source can open cached Matrix /search raw event JSON or request source-only MatrixRequest::FetchEventSource when a hit is available; cross-room context, remote date/pins/scope adapters, gateway/runtime/auth, and live mutation remain blocked."
    )
}

fn message_search_server_context_event_request_label(
    query: &str,
    event_id: &EventId,
    loaded_item_count: usize,
    hit_count: usize,
    already_loaded: bool,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let load_state = if already_loaded {
        "event already loaded"
    } else {
        "event missing from loaded rows; BackwardsPaginateUntilEvent requested"
    };
    format!(
        "Search server Context requested: {query_state}; first cached server hit {event_id}; {loaded_item_count} loaded items; {hit_count} cached server hit(s); {load_state}. Reuses the existing BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline read path for current-room context only; server result rows can surface parsed Matrix /search context-window previews when available; Source can open cached Matrix /search raw event JSON or request source-only MatrixRequest::FetchEventSource; cross-room context, remote date/pins/scope adapters, gateway/runtime/auth, and live mutation remain blocked."
    )
}

fn message_search_server_context_event_found_label(
    query: &str,
    event_id: &EventId,
    loaded_index: usize,
    loaded_item_count: usize,
    hit_count: usize,
    snippet: &str,
    after_pagination: bool,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let path_state = if after_pagination {
        "found after BackwardsPaginateUntilEvent pagination"
    } else {
        "found in loaded timeline before pagination"
    };
    let snippet = compact_message_preview(snippet, "loaded event preview unavailable");
    format!(
        "Search server Context found: {query_state}; event {event_id}; loaded index {loaded_index} of {loaded_item_count}; {hit_count} cached server hit(s); {path_state}; snippet {snippet}. Current-room context jump is live through BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline; server result rows can surface parsed Matrix /search context-window previews when available; Source can open cached Matrix /search raw event JSON or request source-only MatrixRequest::FetchEventSource; cross-room context, remote date/pins/scope adapters, gateway/runtime/auth, and live mutation remain blocked."
    )
}

fn message_search_server_next_page_unavailable_label(
    query: &str,
    pending: bool,
    last_error: &str,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let pending_state = if pending {
        "server search already pending"
    } else {
        "server search idle"
    };
    let error_state = if last_error.trim().is_empty() {
        "no cached server error".to_string()
    } else {
        format!("cached error {} chars", last_error.trim().chars().count())
    };

    format!(
        "Older server search page unavailable: {query_state}; {pending_state}; next_batch cursor missing; {error_state}. Run Server first or wait for a result with next_batch before search pagination. Context pagination uses cached current-room hits only; Source can open cached Matrix /search raw event JSON when a hit is available; gateway/runtime/auth and live mutation remain blocked."
    )
}

fn message_search_loaded_scope_name(scope: MessageSearchLoadedScope) -> &'static str {
    match scope {
        MessageSearchLoadedScope::AllLoaded => "all loaded messages",
        MessageSearchLoadedScope::LatestLoadedDay => "latest loaded-day window",
        MessageSearchLoadedScope::PinnedLoaded => "loaded pinned events",
    }
}

fn message_search_loaded_scope_filter_label(
    action: &str,
    scope: MessageSearchLoadedScope,
    loaded_item_count: usize,
    match_count: usize,
    pinned_event_count: usize,
) -> String {
    let scope_name = message_search_loaded_scope_name(scope);
    let action = action.trim();
    format!(
        "{action} loaded-scope filter applied: {scope_name}; {loaded_item_count} loaded items; {match_count} local match(es); {pinned_event_count} subscribed pinned event(s). Filter/Date/Pins only rescan already loaded timeline rows and the existing pinned-event subscription; no remote date index query, pinned event fetch, PinEvent, timeline reload, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_advanced_filter_controls_label(
    action: Option<&str>,
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    timeline_loaded: bool,
    pinned_event_count: usize,
    loaded_scope: MessageSearchLoadedScope,
) -> String {
    let action_state = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} selected"))
        .unwrap_or_else(|| "no advanced filter selected".to_string());
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "active index reset to 0 with no local match".to_string()
    } else {
        format!(
            "active index {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let scope_state = message_search_loaded_scope_name(loaded_scope);

    format!(
        "Search advanced filter controls: {action_state}; controls Filter, From, Date, Media, Pins; {query_state}; {loaded_item_count} loaded items; {match_count} local matches; {active_state}; {timeline_state}; {pinned_event_count} subscribed pinned event(s); loaded scope {scope_state}. {MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_LABEL} From can submit MatrixRequest::SearchMessagesServer with RoomEventFilter::senders; Media can submit MatrixRequest::SearchMessagesServer with RoomEventFilter::url_filter=EventsWithUrl; Filter restores all loaded matches, Date applies the latest loaded-day window, and Pins applies the existing pinned-event subscription to loaded event ids. They send no extra Matrix-backed search, remote date index query, pinned event fetch, PinEvent, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, room preview fetch, event source open, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_message_search_server_preflight_controls_label(
    action: Option<&str>,
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    timeline_loaded: bool,
    pinned_event_count: usize,
    server_context_metadata: &str,
) -> String {
    let action_state = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} selected"))
        .unwrap_or_else(|| "no server preflight control selected".to_string());
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "active index reset to 0 with no local match".to_string()
    } else {
        format!(
            "active index {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let server_context_state = if server_context_metadata.trim().is_empty() {
        "server/context metadata empty".to_string()
    } else {
        format!(
            "server/context metadata {} chars cached",
            server_context_metadata.chars().count()
        )
    };

    format!(
        "Search server preflight controls: {action_state}; controls Server query, Packet, Contract, Result, Error, Retry, Scope, Taxonomy; {query_state}; {loaded_item_count} loaded items; {match_count} local matches; {active_state}; {timeline_state}; {pinned_event_count} subscribed pinned event(s); {server_context_state}. {MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL} Packet/Contract/Result/Error/Scope/Taxonomy stay local; Context pagination is owned by the server/context controls; remote date index query, remote pinned event fetch, cross-room scope search, full result adapter rendering, remote event context fetch, timeline reload outside BackwardsPaginateUntilEvent, search scope fetch, room preview fetch, event source open, sender/profile lookup, message mutation, gateway/runtime/auth, and live mutation remain blocked."
    )
}

fn loaded_message_search_server_query_local_snapshot_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    timeline_loaded: bool,
    pinned_event_count: usize,
    server_context_metadata: &str,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "active index reset to 0 with no local match".to_string()
    } else {
        format!(
            "active index {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for local preview"
    } else {
        "timeline waiting for local preview"
    };
    let server_context_state = if server_context_metadata.trim().is_empty() {
        "server/context metadata empty".to_string()
    } else {
        format!(
            "server/context metadata {} chars cached",
            server_context_metadata.chars().count()
        )
    };

    format!(
        "Local message search server-query packet: {query_state}; {loaded_item_count} loaded items; {match_count} local matches; {active_state}; {timeline_state}; {pinned_event_count} subscribed pinned event(s); {server_context_state}; server request body not built; result cursor not allocated. {MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL} Server query renders this loaded local request snapshot only; no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, retry automation, result pagination, search scope fetch, room preview fetch, event source open, sender/profile lookup, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn message_search_server_packet_clipboard_payload(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    timeline_loaded: bool,
    pinned_event_count: usize,
    server_context_metadata: &str,
    server_preflight_metadata: &str,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "empty".to_string()
    } else {
        query.to_string()
    };
    let active_match_state = if match_count == 0 {
        "none".to_string()
    } else {
        format!(
            "{} of {}",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let timeline_state = if timeline_loaded { "loaded" } else { "waiting" };
    let context = server_context_metadata.trim();
    let preflight = server_preflight_metadata.trim();

    format!(
        "Message search server packet\nQuery: {query_state}\nLoaded items: {loaded_item_count}\nLocal matches: {match_count}\nActive match: {active_match_state}\nTimeline: {timeline_state}\nPinned events: {pinned_event_count}\nServer/context metadata: {context}\nServer preflight metadata: {preflight}\nRequest body: not built\nResult cursor: not allocated\nBoundary: no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, retry automation, result pagination, search scope fetch, gateway/runtime/auth, or live mutation"
    )
}

fn message_search_server_packet_clipboard_label(
    copied: bool,
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    payload: &str,
) -> String {
    let action_state = if copied {
        "copied local query/result packet to clipboard"
    } else {
        "server packet clipboard unavailable"
    };
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };

    format!(
        "Search server packet clipboard: {action_state}; {query_state}; {loaded_item_count} loaded items; {match_count} local matches; payload {} chars. {MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_LABEL} No Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, retry automation, result pagination, search scope fetch, gateway/runtime/auth, or live mutation.",
        payload.chars().count()
    )
}

fn message_search_matrix_contract_acceptance_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    timeline_loaded: bool,
    pinned_event_count: usize,
    server_context_metadata: &str,
    server_preflight_metadata: &str,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "active index reset to 0 with no local match".to_string()
    } else {
        format!(
            "active index {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for local contract"
    } else {
        "timeline waiting for local contract"
    };
    let context_state = if server_context_metadata.trim().is_empty() {
        "server/context metadata empty".to_string()
    } else {
        format!(
            "server/context metadata {} chars cached",
            server_context_metadata.chars().count()
        )
    };
    let preflight_state = if server_preflight_metadata.trim().is_empty() {
        "packet/preflight metadata empty".to_string()
    } else {
        format!(
            "packet/preflight metadata {} chars cached",
            server_preflight_metadata.chars().count()
        )
    };

    format!(
        "Matrix search acceptance contract stayed local: {query_state}; {loaded_item_count} loaded items; {match_count} local matches; {active_state}; {timeline_state}; {pinned_event_count} subscribed pinned event(s); {context_state}; {preflight_state}. Request slots: room scope, query term, keys, order, limit, filters, next_batch cursor, event-context window. Result slots: event id, sender, timestamp, snippet, highlights, context, source availability, pagination token. Error slots: forbidden, rate-limited, offline, timeout, malformed query, empty result. Retry slots: confirmation, idempotency, stale cursor. Scope/cursor blockers remain explicit before adapter promotion. {MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_LABEL} No Matrix search request body, result cursor allocation, Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, retry automation, result pagination, search scope fetch, room preview fetch, event source open, sender/profile lookup, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn message_search_remote_result_taxonomy_packet_label(
    query: &str,
    loaded_item_count: usize,
    match_count: usize,
    active_match: usize,
    timeline_loaded: bool,
    pinned_event_count: usize,
    server_context_metadata: &str,
    server_preflight_metadata: &str,
) -> String {
    let query = query.trim();
    let query_state = if query.is_empty() {
        "query empty".to_string()
    } else {
        format!("query {} chars", query.chars().count())
    };
    let active_state = if match_count == 0 {
        "active index reset to 0 with no local match".to_string()
    } else {
        format!(
            "active index {} of {} local match(es)",
            active_match.saturating_add(1).min(match_count),
            match_count
        )
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for taxonomy"
    } else {
        "timeline waiting for taxonomy"
    };
    let context_state = if server_context_metadata.trim().is_empty() {
        "server/context metadata empty".to_string()
    } else {
        format!(
            "server/context metadata {} chars cached",
            server_context_metadata.chars().count()
        )
    };
    let preflight_state = if server_preflight_metadata.trim().is_empty() {
        "packet/preflight metadata empty".to_string()
    } else {
        format!(
            "packet/preflight metadata {} chars cached",
            server_preflight_metadata.chars().count()
        )
    };

    format!(
        "Message search remote-result taxonomy packet stayed local: {query_state}; {loaded_item_count} loaded items; {match_count} local matches; {active_state}; {timeline_state}; {pinned_event_count} subscribed pinned event(s); {context_state}; {preflight_state}. Live references remain MatrixRequest::SearchMessagesServer first page, next_batch Older pagination, failed Retry first-page resubmit, From sender filter, Media url filter, Matrix /search event_context preview parsing, current-room BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline Context pagination, cached/raw-or-refetched EventSourceModal Source, loaded Jump/Copy/Thread/Sender handoffs, and loaded-scope Filter/Date/Pins over existing timeline rows and SubscribeToPinnedEvents ids. remote_date_index_operation_id not_assigned; remote_pinned_fetch_operation_id not_assigned; cross_room_scope_request_id not_assigned; full_result_cursor_id not_assigned; full_result_page_id not_assigned; sort_order_result not_wired; room_preview_result not_wired; non_current_room_context_result not_wired; full_result_render_result not_wired; stale_query_result not_wired; retry_cancel_result not_wired; audit_redaction query_hash_event_ids_room_ids_only_no_raw_access_tokens_or_full_sources. No extra Matrix search beyond explicit Server/Older/Retry/From/Media controls, remote date index query, remote pinned event fetch, PinEvent, cross-room scope search, room preview fetch, non-current-room event context fetch, full result adapter rendering, retry automation, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth/provider, or live mutation was submitted. {MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_LABEL} {MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_LABEL} {MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL}"
    )
}

fn message_search_server_result_label(response: &MessageSearchServerResponse) -> String {
    let count_state = response
        .count
        .as_deref()
        .map(|count| format!("server count {count}"))
        .unwrap_or_else(|| "server count unavailable".to_string());
    let cursor_state = if response.next_batch.is_some() {
        "next cursor available"
    } else {
        "no next cursor"
    };
    let first_hit = response.hits.first().map(|hit| {
        let event = hit.event_id.as_deref().unwrap_or("event id unavailable");
        let sender = hit.sender.as_deref().unwrap_or("sender unavailable");
        let snippet = compact_message_search_server_body(&hit.body);
        let context = message_search_server_hit_context_preview(hit)
            .map(|preview| format!(" Context window: {preview}."))
            .unwrap_or_else(|| " Context window unavailable.".to_string());
        format!(" First hit {event} from {sender}: {snippet}.{context}")
    });
    let filter_state = message_search_server_filter_label(&response.filter);

    format!(
        "Live Matrix search returned {} hit(s) for \"{}\" in {}; {filter_state}; {count_state}; {cursor_state}; {} highlight term(s).{}",
        response.hits.len(),
        response.query,
        response.room_id,
        response.highlights.len(),
        first_hit.unwrap_or_default()
    )
}

fn message_search_server_live_preflight_label(response: &MessageSearchServerResponse) -> String {
    let context_hits = response
        .hits
        .iter()
        .filter(|hit| hit.context_before_count > 0 || hit.context_after_count > 0)
        .count();
    let context_preview_hits = response
        .hits
        .iter()
        .filter(|hit| {
            !hit.context_before_previews.is_empty() || !hit.context_after_previews.is_empty()
        })
        .count();
    let cursor_state = response
        .next_batch
        .as_ref()
        .map(|cursor| format!("next_batch {} chars", cursor.chars().count()))
        .unwrap_or_else(|| "next_batch absent".to_string());
    let filter_state = message_search_server_filter_label(&response.filter);
    format!(
        "Search server live result: query {} chars; room {}; {}; {} hit(s); {} hit(s) with server event context; {} hit(s) with parsed context-window previews; {}; highlights {}. Uses MatrixRequest::SearchMessagesServer over /_matrix/client/v3/search; no gateway/runtime/auth/provider mutation.",
        response.query.chars().count(),
        response.room_id,
        filter_state,
        response.hits.len(),
        context_hits,
        context_preview_hits,
        cursor_state,
        response.highlights.len(),
    )
}

fn message_search_server_live_boundary_label(response: &MessageSearchServerResponse) -> String {
    let filter_state = message_search_server_filter_label(&response.filter);
    let source_hits = response
        .hits
        .iter()
        .filter(|hit| {
            hit.source_json
                .as_deref()
                .is_some_and(|source| !source.trim().is_empty())
        })
        .count();
    let context_preview_hits = response
        .hits
        .iter()
        .filter(|hit| {
            !hit.context_before_previews.is_empty() || !hit.context_after_previews.is_empty()
        })
        .count();
    format!(
        "Server search is live for room-scoped Matrix history search, optional From sender filtering, result snippets, highlights, context counts, parsed context-window previews for {} hit(s), next_batch cursor metadata, and cached raw source JSON for {} hit(s). Context can jump/paginate to the first cached current-room hit; Source can open a cached server-result EventSourceModal while cross-room context, remote date/pins/scope adapters, and full result rendering remain blocked. Query \"{}\" returned {} hit(s); {filter_state}.",
        context_preview_hits,
        source_hits,
        response.query,
        response.hits.len()
    )
}

fn message_search_server_filter_label(filter: &MessageSearchServerFilter) -> String {
    let sender_state = filter
        .sender
        .as_deref()
        .map(str::trim)
        .filter(|sender| !sender.is_empty())
        .map(|sender| format!("sender filter {sender}"))
        .unwrap_or_else(|| "no sender filter".to_string());
    if filter.media_only {
        format!("{sender_state}; media/url filter")
    } else {
        sender_state
    }
}

fn message_search_server_error_label(query: &str, error: &str) -> String {
    let query_state = if query.trim().is_empty() {
        "query unavailable".to_string()
    } else {
        format!("query {} chars", query.trim().chars().count())
    };
    format!(
        "Live Matrix search failed: {query_state}; {error}. Retry resubmits MatrixRequest::SearchMessagesServer with the current query."
    )
}

fn compact_message_search_server_body(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return "body unavailable".to_string();
    }
    const MAX_CHARS: usize = 120;
    let mut out = trimmed.chars().take(MAX_CHARS).collect::<String>();
    if trimmed.chars().count() > MAX_CHARS {
        out.push_str("...");
    }
    out
}

fn message_search_server_hit_context_preview(hit: &MessageSearchServerHit) -> Option<String> {
    let before = hit
        .context_before_previews
        .first()
        .map(|preview| compact_message_search_server_body(preview));
    let after = hit
        .context_after_previews
        .first()
        .map(|preview| compact_message_search_server_body(preview));
    match (before, after) {
        (Some(before), Some(after)) => Some(format!("before {before}; after {after}")),
        (Some(before), None) => Some(format!("before {before}; after unavailable")),
        (None, Some(after)) => Some(format!("before unavailable; after {after}")),
        (None, None) => None,
    }
}

fn loaded_message_copy_metadata_label(
    payload_kind: &str,
    payload: &str,
    event_id: Option<&str>,
) -> String {
    let event_state = if event_id.is_some_and(|value| !value.trim().is_empty()) {
        "event id loaded"
    } else {
        "event id missing"
    };
    format!(
        "Loaded clipboard payload: {payload_kind}, {event_state}, {} chars, {} bytes. {MESSAGE_COPY_LOADED_METADATA_LABEL}",
        payload.chars().count(),
        payload.len()
    )
}

fn matrix_link_target_metadata_label(
    kind: &str,
    target: &str,
    via_count: usize,
    current_room_state: &str,
    loaded_target_state: &str,
    event_state: &str,
    action_state: &str,
) -> String {
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    format!(
        "Matrix link target metadata: kind {kind}, target {target}, {via_state}, {current_room_state}, {loaded_target_state}, {event_state}, action {action_state}. {MATRIX_LINK_TARGET_METADATA_LABEL}"
    )
}

fn matrix_link_loaded_event_context_metadata_label(
    event_id: &EventId,
    loaded_index: usize,
    current_room_state: &str,
    loaded_event_state: &str,
    snippet: &str,
) -> String {
    format!(
        "Loaded Matrix event context: target {event_id}, loaded index {loaded_index}, {current_room_state}, {loaded_event_state}, action local scroll/highlight, snippet: {snippet}. {MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_LABEL}"
    )
}

pub(crate) fn matrix_link_preview_result_metadata_label(
    fetched: &FetchedRoomPreview,
    event_id: Option<&OwnedEventId>,
) -> String {
    let alias_state = fetched
        .canonical_alias
        .as_ref()
        .map(|alias| format!("canonical alias loaded: {alias}"))
        .unwrap_or_else(|| "canonical alias missing".to_string());
    let topic_state = fetched
        .topic
        .as_deref()
        .filter(|topic| !topic.trim().is_empty())
        .map(|topic| format!("topic loaded: {} chars", topic.chars().count()))
        .unwrap_or_else(|| "topic missing".to_string());
    let active_member_state = fetched
        .num_active_members
        .map(|count| format!("{count} active members"))
        .unwrap_or_else(|| "active member count unknown".to_string());
    let room_type_state = match fetched.room_type.as_ref() {
        Some(RoomType::Space) => "room type space",
        Some(_) => "room type custom",
        None => "room type regular",
    };
    let join_rule_state = match fetched.join_rule.as_ref() {
        Some(JoinRuleSummary::Public) => "join rule public",
        Some(JoinRuleSummary::Invite) => "join rule invite",
        Some(JoinRuleSummary::Knock) | Some(JoinRuleSummary::KnockRestricted(_)) => {
            "join rule knock"
        }
        Some(JoinRuleSummary::Restricted(_)) => "join rule restricted",
        Some(_) => "join rule other",
        None => "join rule unknown",
    };
    let world_readable_state = match fetched.is_world_readable {
        Some(true) => "world-readable history",
        Some(false) => "non-world-readable history",
        None => "history visibility unknown",
    };
    let room_state = match fetched.state.as_ref() {
        Some(RoomState::Joined) => "current-user state joined",
        Some(RoomState::Invited) => "current-user state invited",
        Some(RoomState::Left) => "current-user state left",
        Some(RoomState::Knocked) => "current-user state knocked",
        Some(RoomState::Banned) => "current-user state banned",
        None => "current-user state unknown",
    };
    let direct_state = match fetched.is_direct {
        Some(true) => "direct room",
        Some(false) => "not direct",
        None => "direct flag unknown",
    };
    let heroes_state = fetched
        .heroes
        .as_ref()
        .map(|heroes| format!("{} heroes loaded", heroes.len()))
        .unwrap_or_else(|| "heroes unknown".to_string());
    let avatar_state = match &fetched.room_avatar {
        FetchedRoomAvatar::Image(bytes) => format!("avatar image loaded: {} bytes", bytes.len()),
        FetchedRoomAvatar::Text(text) if text.trim().is_empty() => {
            "avatar text fallback empty".to_string()
        }
        FetchedRoomAvatar::Text(text) => {
            format!("avatar text fallback: {} chars", text.chars().count())
        }
    };
    let event_state = event_id
        .map(|event_id| format!("event {event_id} context fetch still not wired"))
        .unwrap_or_else(|| "no event id requested".to_string());

    format!(
        "Preview result metadata: room {}, {} joined members, {active_member_state}, {alias_state}, {topic_state}, {room_type_state}, {join_rule_state}, {world_readable_state}, {room_state}, {direct_state}, {heroes_state}, {avatar_state}, {event_state}. {MATRIX_LINK_PREVIEW_RESULT_METADATA_LABEL}",
        fetched.room_name_id, fetched.num_joined_members
    )
}

pub(crate) fn matrix_link_preview_failure_metadata_label(
    target: &str,
    via_count: usize,
    event_id: Option<&OwnedEventId>,
    error: &str,
) -> String {
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = event_id
        .map(|event_id| {
            format!("event {event_id} was requested; event context fetch still not wired")
        })
        .unwrap_or_else(|| "no event id requested".to_string());
    let error = error.trim();
    let error_state = if error.is_empty() {
        "error message empty".to_string()
    } else {
        format!("error message {} chars", error.chars().count())
    };

    format!(
        "Preview failure metadata: target {target}, {via_state}, {event_state}, {error_state}. {MATRIX_LINK_PREVIEW_FAILURE_METADATA_LABEL}"
    )
}

fn matrix_link_preview_retry_confirmation_label(
    target: &str,
    via_count: usize,
    event_id: Option<&OwnedEventId>,
) -> String {
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = event_id
        .map(|event_id| format!("event {event_id} requested"))
        .unwrap_or_else(|| "no event id requested".to_string());
    format!(
        "Retry Matrix link preview for {target}? Cached target only: {via_state}, {event_state}. {MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_LABEL}"
    )
}

fn matrix_link_server_context_boundary_label(
    phase: &str,
    via_count: usize,
    event_id: Option<&OwnedEventId>,
    retry_cache_ready: bool,
) -> String {
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = event_id
        .map(|event_id| format!("event id {event_id} requested"))
        .unwrap_or_else(|| "no event id requested".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link server/context boundary: {phase}; {via_state}; {event_state}; {retry_state}. {MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_LABEL} Current-room missing event links may use BackwardsPaginateUntilEvent/PaginateTimeline read-only pagination from the link click; cached Server context refresh may use PreviewMatrixLinkTarget read-only; Join, Knock, and Invite confirm before membership handoff; no server-side event context fetch, non-current-room timeline pagination/reload, unconfirmed invite, external browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_context_actions_row_label(
    action: Option<&str>,
    status: &str,
    via_count: usize,
    event_id: Option<&OwnedEventId>,
    retry_cache_ready: bool,
) -> String {
    let action_state = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} selected"))
        .unwrap_or_else(|| "no context action selected".to_string());
    let status = status.trim();
    let status_state = if status.is_empty() {
        "status preview"
    } else {
        status
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = event_id
        .map(|event_id| format!("event id {event_id} requested"))
        .unwrap_or_else(|| "no event id requested".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link context actions row: {action_state}; status {status_state}; {via_state}; {event_state}; {retry_state}; controls Server, Event, Alias, Join, Knock, Invite, Browser, Source. {MATRIX_LINK_CONTEXT_ACTIONS_ROW_LABEL} Server refreshes cached room-or-alias targets through PreviewMatrixLinkTarget read-only; Join and Knock confirm before room-or-alias membership mutation; Invite confirms before current-room MatrixRequest::InviteUser. Browser confirms before matrix.to system opener handoff. Source opens loaded current-room or preview-fetched EventSourceModal when available. BackwardsPaginateUntilEvent is limited to current-room missing event link clicks; no server-side event context fetch, non-current-room timeline pagination/reload, unconfirmed invite, unconfirmed external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_server_context_packet_snapshot_label(
    action: &str,
    status: &str,
    target: &str,
    via_count: usize,
    via_label: &str,
    event_id: &str,
    metadata_chars: usize,
    error_chars: Option<usize>,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let action = action.trim();
    let action_state = if action.is_empty() {
        "Server context selected".to_string()
    } else {
        format!("{action} selected")
    };
    let status = status.trim();
    let status_state = if status.is_empty() {
        "status preview"
    } else {
        status
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let via_list_state = if via_label.trim().is_empty() {
        "via list waiting".to_string()
    } else {
        format!("via list {}", via_label.trim())
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let error_state = error_chars
        .map(|chars| format!("error metadata {chars} chars"))
        .unwrap_or_else(|| "error metadata unavailable".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    let loaded_source_state = if loaded_source_available {
        "loaded current-room source available"
    } else {
        "loaded current-room source unavailable"
    };

    format!(
        "Local Matrix link server-context packet snapshot: {action_state}; status {status_state}; {target_state}; {via_state}; {via_list_state}; {event_state}; preview metadata {metadata_chars} chars; {error_state}; {retry_state}; {loaded_source_state}. Event-context request body, result slot, error slot, retry envelope, alias lookup, event context route, pagination cursor, join result, knock result, invite result, external browser handoff, and source handoff are represented as local metadata only. {MATRIX_LINK_CONTEXT_ACTIONS_ROW_LABEL} PreviewMatrixLinkTarget is limited to compact preview, confirmed failed-state Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side event context fetch, non-current-room timeline pagination/reload, unconfirmed invite, browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_route_scope_controls_label(
    action: Option<&str>,
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    metadata_chars: usize,
    retry_cache_ready: bool,
) -> String {
    let action_state = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} route scope selected"))
        .unwrap_or_else(|| "no route scope selected".to_string());
    let status = status.trim();
    let status_state = if status.is_empty() {
        "status preview"
    } else {
        status
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link route scope controls: {action_state}; status {status_state}; {target_state}; {via_state}; {event_state}; preview metadata {metadata_chars} chars; {retry_state}; controls Room, Event, Via, Preview, Source, Packet, Contract, Taxonomy. {MATRIX_LINK_ROUTE_SCOPE_CONTROLS_LABEL} Room copies cached target metadata when available. Event copies cached requested event id when available. Preview copies cached local preview metadata when available. Source opens loaded current-room or preview-fetched EventSourceModal when available. Packet copies per-target route acceptance criteria. Contract copies typed route/result contracts. Taxonomy copies route/event-context result slots. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side event context fetch, non-current-room timeline pagination/reload, join from route-scope controls, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_route_drilldown_packet_payload(
    status: &str,
    target: &str,
    via_count: usize,
    via_label: &str,
    event_id: &str,
    metadata_chars: usize,
    error_chars: Option<usize>,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let via_list_state = if via_label.trim().is_empty() {
        "via list waiting".to_string()
    } else {
        format!("via list {}", via_label.trim())
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let error_state = error_chars
        .map(|chars| format!("error metadata {chars} chars"))
        .unwrap_or_else(|| "error metadata unavailable".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    let loaded_source_state = if loaded_source_available {
        "loaded current-room source available"
    } else {
        "loaded current-room source unavailable"
    };

    format!(
        "Matrix link route drilldown packet\n\
Status: {status_state}\n\
Target: {target_state}\n\
Via: {via_state}; {via_list_state}\n\
Event: {event_state}\n\
Preview: metadata {metadata_chars} chars; {error_state}\n\
Retry: {retry_state}\n\
Loaded source: {loaded_source_state}\n\
Room route acceptance: cached target label/status/via/event metadata, loaded-room or preview-result source, result/error/retry/source slots.\n\
Event route acceptance: cached event id, loaded current-room scroll/highlight, current-room missing-event BackwardsPaginateUntilEvent/PaginateTimeline read, and server-side event context result/error/retry/source slots.\n\
Via route acceptance: cached via server list, route provenance, alias/event preview retry envelope, result/error/source slots.\n\
Preview route acceptance: compact PreviewMatrixLinkTarget request/result/error metadata only, no follow-up request outside confirmed Retry.\n\
Server context acceptance: server-context packet, server-side alias resolution, event context, pagination/reload, and retry cursor remain typed contract slots.\n\
Alias route acceptance: loaded alias local navigation vs server-side alias resolution contract, result/error/retry/source slots.\n\
Join route acceptance: room-or-alias join uses the confirmed JoinRoomByIdOrAlias path.\n\
Knock route acceptance: room-or-alias knock uses the confirmed MatrixRequest::Knock path and KnockResultAction result slot.\n\
Invite/browser handoff remains confirmation-gated contract work.\n\
Source route acceptance: loaded current-room EventSourceModal only; remote event source handoff remains a typed result/error contract.\n\
{MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_LABEL}\n\
PreviewMatrixLinkTarget limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side event context fetch, non-current-room timeline pagination/reload, unconfirmed invite, external browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_route_drilldown_packet_label(
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    metadata_chars: usize,
    retry_cache_ready: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link route Packet copied per-target route drilldown to local clipboard: status {status_state}; {target_state}; {via_state}; {event_state}; preview metadata {metadata_chars} chars; {retry_state}. {MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_LABEL} Join, Knock, and Invite confirm before membership handoff. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side event context fetch, non-current-room timeline pagination/reload, unconfirmed invite, external browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_route_result_contract_packet_payload(
    status: &str,
    target: &str,
    via_count: usize,
    via_label: &str,
    event_id: &str,
    metadata_chars: usize,
    error_chars: Option<usize>,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let via_list_state = if via_label.trim().is_empty() {
        "via list waiting".to_string()
    } else {
        format!("via list {}", via_label.trim())
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let error_state = error_chars
        .map(|chars| format!("error metadata {chars} chars"))
        .unwrap_or_else(|| "error metadata unavailable".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    let loaded_source_state = if loaded_source_available {
        "loaded current-room source available"
    } else {
        "loaded current-room source unavailable"
    };

    format!(
        "Matrix link typed route/result contract\n\
Status: {status_state}\n\
Target: {target_state}\n\
Via: {via_state}; {via_list_state}\n\
Event: {event_state}\n\
Preview: metadata {metadata_chars} chars; {error_state}\n\
Retry: {retry_state}\n\
Loaded source: {loaded_source_state}\n\
Contract matrix:\n\
- Target identity: typed room id, alias, user id, event id, via server list, current-room relation, and target-kind classification must be stable before routing.\n\
- Preview contract: compact PreviewMatrixLinkTarget request/result/error/retry/source slots, stale target detection, and source hash are required before richer preview handling.\n\
- Alias route contract: local loaded alias navigation vs server-side alias lookup result, canonical/alt alias provenance, error taxonomy, retry/source slots, and no join before confirmation.\n\
- Room route contract: room preview result, membership/join rule/world-readable/direct flags, avatar/topic/member counts, open-room action, error/retry/source slots, and browser fallback before promotion.\n\
- Event route contract: loaded current-room jump result, current-room missing-event BackwardsPaginateUntilEvent/PaginateTimeline result, server-side event context result/error/retry/source slots, and pagination cursor before non-current-room event-context work.\n\
- Via route contract: via server provenance, routed alias/event preview envelope, server result/error/source slots, and signed retry envelope before server-context work.\n\
- Join, Knock, and Invite contract: room-or-alias JoinRoomByIdOrAlias, MatrixRequest::Knock, and current-room MatrixRequest::InviteUser results are live after confirmation; server-context promotion still requires typed route/result slots before richer routing.\n\
- Source contract: loaded current-room EventSourceModal result, remote event source request/result/error/retry/source slots, source hash, and stale event detection before remote source handoff.\n\
- Browser handoff contract: typed external URL/open confirmation, result/error/cancel slots, and no browser handoff before confirmation.\n\
Promotion blocker: map the route drilldown packet to typed route/result contracts before server-context work.\n\
Boundary: {MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_LABEL} PreviewMatrixLinkTarget limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side event context fetch, non-current-room timeline pagination/reload, unconfirmed invite, external browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_route_result_contract_packet_label(
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    metadata_chars: usize,
    retry_cache_ready: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link route Contract copied typed route/result packet to local clipboard: status {status_state}; {target_state}; {via_state}; {event_state}; preview metadata {metadata_chars} chars; {retry_state}. {MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_LABEL} Join, Knock, and Invite result slots are live after confirmation. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side event context fetch, non-current-room timeline pagination/reload, unconfirmed invite, external browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_route_result_taxonomy_packet_payload(
    status: &str,
    target: &str,
    via_count: usize,
    via_label: &str,
    event_id: &str,
    metadata_chars: usize,
    error_chars: Option<usize>,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let via_list_state = if via_label.trim().is_empty() {
        "via list waiting".to_string()
    } else {
        format!("via list {}", via_label.trim())
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let error_state = error_chars
        .map(|chars| format!("error metadata {chars} chars"))
        .unwrap_or_else(|| "error metadata unavailable".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    let loaded_source_state = if loaded_source_available {
        "loaded current-room source available"
    } else {
        "loaded current-room source unavailable"
    };

    format!(
        "Matrix link route/event-context result taxonomy\n\
Status: {status_state}\n\
Target: {target_state}\n\
Via: {via_state}; {via_list_state}\n\
Event: {event_state}\n\
Preview: metadata {metadata_chars} chars; {error_state}\n\
Retry: {retry_state}\n\
Loaded source: {loaded_source_state}\n\
Live references:\n\
- loaded_alias_navigation_result: loaded RoomsList canonical/alt alias NavigateToRoom\n\
- loaded_event_jump_result: current-room loaded event scroll/highlight\n\
- current_room_missing_event_pagination_result: MatrixRequest::BackwardsPaginateUntilEvent / MatrixRequest::PaginateTimeline\n\
- compact_preview_result: MatrixRequest::PreviewMatrixLinkTarget room-preview read, cached Server refresh, and confirmed failed-state Retry only\n\
- source_only_preview_fetch_result: Room::load_or_fetch_event for known previewed room event source JSON\n\
- source_modal_result: loaded or preview-fetched EventSourceModal Source\n\
- browser_handoff_result: PositiveConfirmationModal-gated matrix.to system opener\n\
- join_knock_invite_result: confirmed JoinRoomByIdOrAlias, MatrixRequest::Knock, and current-room MatrixRequest::InviteUser status/retry\n\
Blocked taxonomy:\n\
- route_adapter_request_id: not_assigned\n\
- alias_resolution_operation_id: not_assigned\n\
- non_current_room_event_context_operation_id: not_assigned\n\
- via_route_request_id: not_assigned\n\
- full_remote_source_request_id: not_assigned\n\
- room_preview_route_result: compact_preview_only; richer_route_result not_wired\n\
- event_context_window_result: not_wired\n\
- alias_resolution_result: not_wired\n\
- via_resolution_result: not_wired\n\
- full_remote_source_result: not_wired\n\
- access_denied_result: not_wired\n\
- stale_target_result: not_wired\n\
- retry_cancel_result: confirmed_retry_only; cancel_local_only\n\
- audit_redaction: target_hash_room_or_alias_event_id_via_count_status_only_no_access_tokens_raw_source_or_homeserver_credentials\n\
Boundary: {MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_LABEL} No PreviewMatrixLinkTarget beyond explicit compact preview, Server refresh, or confirmed Retry controls, no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, no event-context fetch, no non-current-room timeline pagination/reload, no full remote source fetch, no unconfirmed browser handoff, no unconfirmed join/knock/invite, no message send/edit/redact, no room-state mutation, no membership mutation outside confirmed join/knock/invite paths, no account/profile mutation, no gateway/runtime/auth/provider, Telegram delivery, or live mutation."
    )
}

fn matrix_link_route_result_taxonomy_packet_label(
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    metadata_chars: usize,
    retry_cache_ready: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link Taxonomy copied route/event-context result slots to local clipboard: status {status_state}; {target_state}; {via_state}; {event_state}; preview metadata {metadata_chars} chars; {retry_state}. {MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_LABEL} Live references stay limited to loaded alias navigation, loaded/current-room pagination, compact PreviewMatrixLinkTarget, confirmed Retry, source-only preview fetch, EventSourceModal, confirmed Browser opener, and confirmed Join/Knock/Invite results. route_adapter_request_id not_assigned; alias_resolution_operation_id not_assigned; non_current_room_event_context_operation_id not_assigned; via_route_request_id not_assigned; full_remote_source_request_id not_assigned; event_context_window_result not_wired; alias_resolution_result not_wired; via_resolution_result not_wired; full_remote_source_result not_wired; stale_target_result not_wired; audit_redaction target_hash_room_or_alias_event_id_via_count_status_only_no_access_tokens_raw_source_or_homeserver_credentials. No server-side alias resolution, event-context fetch, non-current-room timeline pagination/reload, full remote source fetch, unconfirmed browser handoff, unconfirmed join/knock/invite, message mutation, room-state, membership outside confirmed join/knock/invite paths, gateway/runtime/auth/provider, or live mutation."
    )
}

fn matrix_link_via_servers_label(via: &[OwnedServerName]) -> String {
    via.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn matrix_link_fragment_segment(value: &str) -> String {
    url::form_urlencoded::byte_serialize(value.trim().as_bytes())
        .collect::<String>()
        .replace('+', "%20")
}

fn matrix_link_cached_target_room_label<'a>(target: &'a str, event_id: &str) -> Option<&'a str> {
    let target = target.trim();
    if target.is_empty() {
        return None;
    }
    if event_id.trim().is_empty() {
        return Some(target);
    }
    target
        .rsplit_once(" in ")
        .map(|(_, room)| room.trim())
        .filter(|room| !room.is_empty())
        .or(Some(target))
}

fn matrix_link_room_or_alias_join_target(
    target: &str,
    event_id: &str,
) -> Result<OwnedRoomOrAliasId, String> {
    let Some(room_label) = matrix_link_cached_target_room_label(target, event_id) else {
        return Err("cached target missing".to_string());
    };
    let room_label = room_label.trim();
    if !room_label.starts_with('!') && !room_label.starts_with('#') {
        return Err(format!(
            "cached target {room_label} is not a Matrix room id or alias; join/knock routes require a cached room id or alias"
        ));
    }
    OwnedRoomOrAliasId::try_from(room_label.to_string()).map_err(|_| {
        format!(
            "cached target {room_label} is not a valid Matrix room id or alias; join/knock routes require a cached room id or alias"
        )
    })
}

fn matrix_link_user_invite_target(target: &str) -> Result<OwnedUserId, String> {
    let target = target.trim();
    if target.is_empty() {
        return Err("cached target missing".to_string());
    }
    if !target.starts_with('@') {
        return Err(format!(
            "cached target {target} is not a Matrix user id; invite routes require @user:server"
        ));
    }
    OwnedUserId::try_from(target.to_string()).map_err(|_| {
        format!(
            "cached target {target} is not a valid Matrix user id; invite routes require @user:server"
        )
    })
}

fn matrix_link_join_room_confirmation_label(
    room_or_alias_id: &OwnedRoomOrAliasId,
    status: &str,
    via_count: usize,
    event_id: &str,
) -> String {
    let status = status.trim();
    let status_state = if status.is_empty() {
        "status preview"
    } else {
        status
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!(
            "event id {} cached; join applies to containing room only",
            event_id.trim()
        )
    };
    format!(
        "Join Matrix room {room_or_alias_id}? Cached room id or alias only: status {status_state}; {via_state}; {event_state}. {MATRIX_LINK_JOIN_ROOM_CONFIRMATION_LABEL} Knock and Invite have separate confirmed routes; event context, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation stay blocked."
    )
}

fn matrix_link_join_room_unavailable_label(target: &str, event_id: &str, reason: &str) -> String {
    let target_state = if target.trim().is_empty() {
        "target missing".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} cached", event_id.trim())
    };
    format!(
        "Matrix link Join unavailable: {target_state}; {event_state}; {reason}. {MATRIX_LINK_JOIN_ROOM_CONFIRMATION_LABEL}"
    )
}

fn matrix_link_join_room_result_label(
    room_or_alias_id: &OwnedRoomOrAliasId,
    room_id: Option<&OwnedRoomId>,
    succeeded: bool,
    error: Option<&str>,
) -> String {
    if succeeded {
        let resolved_room = room_id
            .map(ToString::to_string)
            .unwrap_or_else(|| "resolved room id unavailable".to_string());
        return format!(
            "Matrix link Join succeeded for {room_or_alias_id} as {resolved_room}. MatrixLinkJoinResultAction::Joined was consumed by RoomScreen; server context, invite, event context, and room-state remain blocked."
        );
    }
    let error_state = error
        .map(str::trim)
        .filter(|error| !error.is_empty())
        .map(|error| format!("error {error}"))
        .unwrap_or_else(|| "error unavailable".to_string());
    format!(
        "Matrix link Join failed for {room_or_alias_id}: {error_state}. Failed-state retry keeps the cached room id or alias and confirms before MatrixRequest::JoinRoomByIdOrAlias."
    )
}

fn matrix_link_knock_room_confirmation_label(
    room_or_alias_id: &OwnedRoomOrAliasId,
    status: &str,
    via_count: usize,
    event_id: &str,
) -> String {
    let status = status.trim();
    let status_state = if status.is_empty() {
        "status preview"
    } else {
        status
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!(
            "event id {} cached; knock applies to containing room only",
            event_id.trim()
        )
    };
    format!(
        "Knock on Matrix room {room_or_alias_id}? Cached room id or alias only: status {status_state}; {via_state}; {event_state}. {MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_LABEL} Invite, event context, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation stay blocked."
    )
}

fn matrix_link_knock_room_unavailable_label(target: &str, event_id: &str, reason: &str) -> String {
    let target_state = if target.trim().is_empty() {
        "target missing".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} cached", event_id.trim())
    };
    format!(
        "Matrix link Knock unavailable: {target_state}; {event_state}; {reason}. {MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_LABEL}"
    )
}

fn matrix_link_knock_room_result_label(
    room_or_alias_id: &OwnedRoomOrAliasId,
    room_id: Option<&OwnedRoomId>,
    succeeded: bool,
    error: Option<&str>,
) -> String {
    if succeeded {
        let resolved_room = room_id
            .map(ToString::to_string)
            .unwrap_or_else(|| "resolved room id unavailable".to_string());
        return format!(
            "Matrix link Knock succeeded for {room_or_alias_id} as {resolved_room}. KnockResultAction::Knocked was consumed by RoomScreen; server context, invite, event context, and room-state remain blocked."
        );
    }
    let error_state = error
        .map(str::trim)
        .filter(|error| !error.is_empty())
        .map(|error| format!("error {error}"))
        .unwrap_or_else(|| "error unavailable".to_string());
    format!(
        "Matrix link Knock failed for {room_or_alias_id}: {error_state}. Failed-state retry keeps the cached room id or alias and confirms before MatrixRequest::Knock."
    )
}

fn matrix_link_invite_user_confirmation_label(
    room_id: &OwnedRoomId,
    user_id: &OwnedUserId,
    status: &str,
    via_count: usize,
) -> String {
    let status = status.trim();
    let status_state = if status.is_empty() {
        "status preview"
    } else {
        status
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    format!(
        "Invite Matrix user {user_id} to current room {room_id}? Cached Matrix user id only: status {status_state}; {via_state}. PositiveConfirmationModal gates MatrixRequest::InviteUser; accept sends the room/user pair and cancel stays local. {MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL} Join, Knock, event context, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation stay separate."
    )
}

fn matrix_link_invite_user_unavailable_label(target: &str, reason: &str) -> String {
    let target_state = if target.trim().is_empty() {
        "target missing".to_string()
    } else {
        format!("target {}", target.trim())
    };
    format!(
        "Matrix link Invite unavailable: {target_state}; {reason}. {MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL}"
    )
}

fn matrix_link_invite_user_result_label(
    room_id: &OwnedRoomId,
    user_id: &OwnedUserId,
    succeeded: bool,
    error: Option<&str>,
) -> String {
    if succeeded {
        return format!(
            "Matrix link Invite succeeded for {user_id} into {room_id}. InviteResultAction::Sent was consumed by RoomScreen; server context, event context, and room-state remain blocked."
        );
    }
    let error_state = error
        .map(str::trim)
        .filter(|error| !error.is_empty())
        .map(|error| format!("error {error}"))
        .unwrap_or_else(|| "error unavailable".to_string());
    format!(
        "Matrix link Invite failed for {user_id} into {room_id}: {error_state}. Failed-state retry keeps the cached room id and user id and confirms before MatrixRequest::InviteUser."
    )
}

fn matrix_link_browser_handoff_url(
    target: &str,
    event_id: &str,
    via_label: &str,
) -> Option<String> {
    let room = matrix_link_cached_target_room_label(target, event_id)?;
    let mut fragment = format!("/{}", matrix_link_fragment_segment(room));
    let event_id = event_id.trim();
    if !event_id.is_empty() {
        fragment.push('/');
        fragment.push_str(&matrix_link_fragment_segment(event_id));
    }
    let via_values = via_label
        .split(',')
        .map(str::trim)
        .filter(|via| !via.is_empty())
        .collect::<Vec<_>>();
    if !via_values.is_empty() {
        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        for via in via_values {
            serializer.append_pair("via", via);
        }
        fragment.push('?');
        fragment.push_str(&serializer.finish());
    }
    Some(format!("https://matrix.to/#{fragment}"))
}

fn matrix_link_browser_handoff_confirmation_label(
    target: &str,
    event_id: &str,
    via_count: usize,
    url: Option<&str>,
) -> String {
    let target = target.trim();
    let target_state = if target.is_empty() {
        "target missing".to_string()
    } else {
        format!("target {target}")
    };
    let event_id = event_id.trim();
    let event_state = if event_id.is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {event_id} cached")
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let url_state = url
        .map(|url| format!("matrix.to URL {url}"))
        .unwrap_or_else(|| "matrix.to URL unavailable".to_string());
    format!(
        "Matrix link Browser handoff: {target_state}; {via_state}; {event_state}; {url_state}. PositiveConfirmationModal gates the system opener; accept opens matrix.to through robius_open and cancel stays local. {MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_LABEL} No PreviewMatrixLinkTarget, no BackwardsPaginateUntilEvent, no server-side alias resolution, no event context fetch, no timeline pagination/reload, no join/knock/invite, no message mutation, no room-state, no membership, no gateway/runtime/auth, no Telegram delivery, or live mutation."
    )
}

fn matrix_link_room_target_clipboard_payload(
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    retry_cache_ready: bool,
) -> Option<String> {
    let target = target.trim();
    if target.is_empty() {
        return None;
    }
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    Some(format!(
        "Matrix link room target\nStatus: {status_state}\nTarget: {target}\nVia: {via_state}\nEvent: {event_state}\nRetry: {retry_state}\n{MATRIX_LINK_ROOM_TARGET_CLIPBOARD_LABEL}\nPreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    ))
}

fn matrix_link_room_target_clipboard_label(
    copied: bool,
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    retry_cache_ready: bool,
) -> String {
    let action_state = if copied {
        "copied cached target metadata to local clipboard"
    } else {
        "target clipboard unavailable"
    };
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link room target clipboard: {action_state}; status {status_state}; {target_state}; {via_state}; {event_state}; {retry_state}. {MATRIX_LINK_ROOM_TARGET_CLIPBOARD_LABEL} PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_via_servers_clipboard_payload(
    status: &str,
    target: &str,
    via_count: usize,
    via_label: &str,
    event_id: &str,
    retry_cache_ready: bool,
) -> Option<String> {
    let via_label = via_label.trim();
    if via_label.is_empty() {
        return None;
    }
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "via count unknown".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    Some(format!(
        "Matrix link via servers\nStatus: {status_state}\nTarget: {target_state}\nVia: {via_state}\nServers: {via_label}\nEvent: {event_state}\nRetry: {retry_state}\n{MATRIX_LINK_VIA_SERVERS_CLIPBOARD_LABEL}\nPreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    ))
}

fn matrix_link_via_servers_clipboard_label(
    copied: bool,
    status: &str,
    target: &str,
    via_count: usize,
    via_label: &str,
    event_id: &str,
    retry_cache_ready: bool,
) -> String {
    let action_state = if copied {
        "copied cached via server list to local clipboard"
    } else {
        "via server clipboard unavailable"
    };
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "via count unknown".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let via_list_state = if via_label.trim().is_empty() {
        "via list waiting".to_string()
    } else {
        format!("via list {}", via_label.trim())
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link via servers clipboard: {action_state}; status {status_state}; {target_state}; {via_state}; {via_list_state}; {event_state}; {retry_state}. {MATRIX_LINK_VIA_SERVERS_CLIPBOARD_LABEL} PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_event_id_clipboard_payload(
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    retry_cache_ready: bool,
) -> Option<String> {
    let event_id = event_id.trim();
    if event_id.is_empty() {
        return None;
    }
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    Some(format!(
        "Matrix link event id\nStatus: {status_state}\nTarget: {target_state}\nVia: {via_state}\nEvent: {event_id}\nRetry: {retry_state}\n{MATRIX_LINK_EVENT_ID_CLIPBOARD_LABEL}\nPreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    ))
}

fn matrix_link_event_id_clipboard_label(
    copied: bool,
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    retry_cache_ready: bool,
) -> String {
    let action_state = if copied {
        "copied cached event id to local clipboard"
    } else {
        "event id clipboard unavailable"
    };
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "event id waiting".to_string()
    } else {
        format!("event id {}", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link event id clipboard: {action_state}; status {status_state}; {target_state}; {via_state}; {event_state}; {retry_state}. {MATRIX_LINK_EVENT_ID_CLIPBOARD_LABEL} PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_preview_metadata_clipboard_payload(
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    metadata: &str,
    retry_cache_ready: bool,
) -> Option<String> {
    let metadata = metadata.trim();
    if metadata.is_empty() {
        return None;
    }
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    Some(format!(
        "Matrix link preview metadata\nStatus: {status_state}\nTarget: {target_state}\nVia: {via_state}\nEvent: {event_state}\nRetry: {retry_state}\nMetadata ({} chars, {} bytes): {metadata}\n{MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_LABEL}\nPreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.",
        metadata.chars().count(),
        metadata.len()
    ))
}

fn matrix_link_preview_metadata_clipboard_label(
    copied: bool,
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    metadata: &str,
    retry_cache_ready: bool,
) -> String {
    let action_state = if copied {
        "copied cached preview metadata to local clipboard"
    } else {
        "preview metadata clipboard unavailable"
    };
    let status_state = if status.trim().is_empty() {
        "preview status waiting"
    } else {
        status.trim()
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let metadata_state = if metadata.trim().is_empty() {
        "metadata waiting".to_string()
    } else {
        format!(
            "metadata {} chars/{} bytes",
            metadata.trim().chars().count(),
            metadata.trim().len()
        )
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link preview metadata clipboard: {action_state}; status {status_state}; {target_state}; {via_state}; {event_state}; {metadata_state}; {retry_state}. {MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_LABEL} PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_loaded_event_source_modal_label(
    action: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    loaded_index: Option<usize>,
    latest_json: Option<&str>,
    opened: bool,
) -> String {
    let action = action.trim();
    let action_state = if action.is_empty() {
        "Source selected".to_string()
    } else {
        format!("{action} selected")
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "event id missing".to_string()
    } else {
        format!("event id {} cached", event_id.trim())
    };
    let fetched_source_available = latest_json.is_some() && loaded_index.is_none();
    let loaded_state = loaded_index
        .map(|index| format!("loaded index {index}"))
        .unwrap_or_else(|| {
            if fetched_source_available {
                "preview-fetched source cached".to_string()
            } else {
                "loaded row unavailable".to_string()
            }
        });
    let json_state = latest_json
        .map(|json| {
            format!(
                "latest_json loaded: {} chars, {} lines",
                json.chars().count(),
                json.lines().count()
            )
        })
        .unwrap_or_else(|| "latest_json unavailable".to_string());
    let open_state = if opened && fetched_source_available {
        "opened preview-fetched EventSourceModal"
    } else if opened {
        "opened loaded local EventSourceModal"
    } else {
        "Source stayed local; no loaded or preview-fetched event source"
    };
    format!(
        "Matrix link event Source: {action_state}; {open_state}; {target_state}; {via_state}; {event_state}; {loaded_state}; {json_state}. {MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_LABEL} Source click sends no follow-up Matrix request; preview-fetched source is cached from the compact PreviewMatrixLinkTarget worker's source-only Room::load_or_fetch_event fallback. No BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side event-context window fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn matrix_link_unresolved_detail_label(
    action: Option<&str>,
    status: &str,
    target: &str,
    via_count: usize,
    event_id: &str,
    metadata_chars: usize,
    error_chars: Option<usize>,
    retry_cache_ready: bool,
) -> String {
    let action_state = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} detail selected"))
        .unwrap_or_else(|| "no Server/Event/Alias/Join/Knock/Source detail selected".to_string());
    let status = status.trim();
    let status_state = if status.is_empty() {
        "status preview"
    } else {
        status
    };
    let target_state = if target.trim().is_empty() {
        "target waiting".to_string()
    } else {
        format!("target {}", target.trim())
    };
    let via_state = match via_count {
        0 => "no via servers".to_string(),
        1 => "1 via server".to_string(),
        count => format!("{count} via servers"),
    };
    let event_state = if event_id.trim().is_empty() {
        "no event id requested".to_string()
    } else {
        format!("event id {} requested", event_id.trim())
    };
    let error_state = error_chars
        .map(|chars| format!("error metadata {chars} chars"))
        .unwrap_or_else(|| "error metadata unavailable".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache cleared"
    };
    format!(
        "Matrix link unresolved detail: {action_state}; status {status_state}; {target_state}; {via_state}; {event_state}; preview metadata {metadata_chars} chars; {error_state}; {retry_state}. {MATRIX_LINK_UNRESOLVED_DETAIL_LABEL} Source opens loaded current-room or preview-fetched EventSourceModal when available. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn loaded_event_plaintext_preview_for_event_id(
    items: &Vector<Arc<TimelineItem>>,
    event_id: &EventId,
) -> Option<String> {
    items.iter().find_map(|item| {
        let event_tl_item = item.as_event()?;
        let loaded_event_id = event_tl_item.event_id()?;
        (loaded_event_id == event_id)
            .then(|| compact_edit_history_preview(&plaintext_body_of_timeline_item(event_tl_item)))
    })
}

fn compact_edit_history_preview(body: &str) -> String {
    compact_message_preview(body, "empty edit preview")
}

fn compact_message_preview(body: &str, empty_label: &str) -> String {
    let compact = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.is_empty() {
        return empty_label.to_string();
    }
    let preview: String = compact.chars().take(96).collect();
    if compact.chars().count() > 96 {
        format!("{preview}...")
    } else {
        preview
    }
}

fn message_report_status_summary_label(event_id: &EventId, status: &str, reason: &str) -> String {
    let reason = compact_message_preview(reason, "empty reason");
    format!(
        "Report {status} for {event_id}: reason {reason}. {MESSAGE_REPORT_STATUS_LIFECYCLE_LABEL}"
    )
}

fn message_report_status_metadata_label(status: &str, reason: &str, error: Option<&str>) -> String {
    let reason_chars = reason.chars().count();
    let error_state = error
        .map(|error| {
            format!(
                "error {} chars: {}",
                error.chars().count(),
                compact_message_preview(error, "empty error")
            )
        })
        .unwrap_or_else(|| "no error result".to_string());
    format!(
        "Report status metadata: state {status}, reason {reason_chars} chars, {error_state}. {MESSAGE_REPORT_STATUS_LIFECYCLE_LABEL} No retry without confirmation, cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_status_clipboard_payload(
    status: &str,
    event_id: Option<&EventId>,
    reason: &str,
    error: Option<&str>,
    summary: &str,
    metadata: &str,
    preflight_metadata: &str,
) -> Option<String> {
    let status = status.trim();
    if status.is_empty() || status == "waiting" || status == "local" {
        return None;
    }
    let event_state = event_id
        .map(ToString::to_string)
        .unwrap_or_else(|| "event unavailable".to_string());
    let reason = compact_message_preview(reason, "empty reason");
    let error_state = error
        .map(|error| compact_message_preview(error, "empty error"))
        .unwrap_or_else(|| "no error result".to_string());
    let summary = compact_message_preview(summary, "summary unavailable");
    let metadata = compact_message_preview(metadata, "metadata unavailable");
    let preflight = compact_message_preview(preflight_metadata, "preflight metadata unavailable");
    Some(format!(
        "Message report status\nStatus: {status}\nEvent: {event_state}\nReason: {reason}\nError: {error_state}\nSummary: {summary}\nMetadata: {metadata}\nPreflight: {preflight}\nScope: cached RoomScreen report status\nBoundary: no extra ReportContent, retry, moderation queue, policy lookup, redact/delete, ban, kick, room-state, membership, gateway/runtime/auth, or live mutation"
    ))
}

fn message_report_status_clipboard_label(
    copied: bool,
    status: &str,
    event_id: Option<&EventId>,
    reason: &str,
    error: Option<&str>,
    summary: &str,
) -> String {
    let action_state = if copied {
        "copied cached report status to local clipboard"
    } else {
        "report status clipboard unavailable"
    };
    let status_state = if status.trim().is_empty() {
        "status waiting".to_string()
    } else {
        format!("status {}", status.trim())
    };
    let event_state = event_id
        .map(|event_id| format!("event {event_id}"))
        .unwrap_or_else(|| "event unavailable".to_string());
    let reason_state = format!("reason {} chars", reason.trim().chars().count());
    let error_state = error
        .map(|error| format!("error {} chars", error.trim().chars().count()))
        .unwrap_or_else(|| "error cache empty".to_string());
    let summary_state = if summary.trim().is_empty() {
        "summary waiting".to_string()
    } else {
        format!("summary {} chars", summary.trim().chars().count())
    };
    format!(
        "Report status clipboard: {action_state}; {status_state}; {event_state}; {reason_state}; {error_state}; {summary_state}. {MESSAGE_REPORT_STATUS_CLIPBOARD_LABEL} No extra MatrixRequest::ReportContent, retry without confirmation, cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_retry_confirmation_label(event_id: &EventId, reason: &str) -> String {
    let reason = compact_message_preview(reason, "empty reason");
    format!(
        "Retry report_content for {event_id} with reason {reason}? {MESSAGE_REPORT_RETRY_CONFIRMATION_LABEL} No retry queue automation, cancel queue, moderation policy lookup, ban, kick, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_workflow_action_boundary_label(
    action: &str,
    status: &str,
    event_id: Option<&EventId>,
    reason: &str,
) -> String {
    let event_state = event_id
        .map(|event_id| format!("event {event_id}"))
        .unwrap_or_else(|| "event unavailable".to_string());
    let reason = compact_message_preview(reason, "empty reason");
    format!(
        "{action} stayed local for report status {status}, {event_state}, reason {reason}. {MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_LABEL} No moderation queue cancel/reorder, server policy fetch, reviewer assignment, appeal workflow, redact/delete, ban, kick, ignore/block, room-state, membership, message send/edit, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_local_moderation_snapshot_label(
    status: &str,
    event_id: Option<&EventId>,
    reason: &str,
    error: Option<&str>,
    summary: &str,
    workflow_metadata: &str,
    preflight_detail: &str,
    retry_cached: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "status waiting"
    } else {
        status.trim()
    };
    let event_state = event_id
        .map(|event_id| format!("event {event_id}"))
        .unwrap_or_else(|| "event unavailable".to_string());
    let reason_state = compact_message_preview(reason, "empty reason");
    let error_state = error
        .map(str::trim)
        .filter(|error| !error.is_empty())
        .map(|error| format!("error {} chars: {}", error.chars().count(), error))
        .unwrap_or_else(|| "error cache empty".to_string());
    let summary_state = if summary.trim().is_empty() {
        "summary waiting".to_string()
    } else {
        format!(
            "summary {} chars: {}",
            summary.trim().chars().count(),
            summary.trim()
        )
    };
    let workflow_state = if workflow_metadata.trim().is_empty() {
        "workflow metadata waiting".to_string()
    } else {
        format!(
            "workflow metadata {} chars",
            workflow_metadata.trim().chars().count()
        )
    };
    let preflight_state = if preflight_detail.trim().is_empty() {
        "preflight detail waiting".to_string()
    } else {
        format!(
            "preflight detail {} chars",
            preflight_detail.trim().chars().count()
        )
    };
    let retry_state = if retry_cached {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    format!(
        "Local moderation packet snapshot: {status_state}; {event_state}; reason {reason_state}; {error_state}; {summary_state}; {workflow_state}; {preflight_state}; {retry_state}. Queue renders this cached local ReportContent packet only. No moderation queue cancel/reorder, server policy fetch, reviewer assignment, appeal workflow, redact/delete, ban, kick, ignore/block, room-state, membership, message send/edit, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_moderation_reviewer_packet_payload(
    status: &str,
    event_id: Option<&EventId>,
    reason: &str,
    error: Option<&str>,
    summary: &str,
    workflow_metadata: &str,
    preflight_metadata: &str,
    retry_cached: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "waiting"
    } else {
        status.trim()
    };
    let event_state = event_id
        .map(ToString::to_string)
        .unwrap_or_else(|| "event unavailable".to_string());
    let reason_state = compact_message_preview(reason, "empty reason");
    let error_state = error
        .map(|error| compact_message_preview(error, "empty error"))
        .unwrap_or_else(|| "no error result".to_string());
    let summary_state = compact_message_preview(summary, "summary unavailable");
    let workflow_state =
        compact_message_preview(workflow_metadata, "workflow metadata unavailable");
    let preflight_state =
        compact_message_preview(preflight_metadata, "preflight metadata unavailable");
    let retry_state = if retry_cached {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded event source available"
    } else {
        "loaded event source unavailable"
    };

    format!(
        "Message report moderation reviewer packet\n\
Status: {status_state}\n\
Event: {event_state}\n\
Reason: {reason_state}\n\
Error: {error_state}\n\
Summary: {summary_state}\n\
Workflow metadata: {workflow_state}\n\
Preflight metadata: {preflight_state}\n\
Retry: {retry_state}\n\
Loaded source: {source_state}\n\
Acceptance: moderation queue persistence requires a stable report id, event id, reason, result/error slots, dedupe key, retry/cancel state, and audit timestamp before promotion.\n\
Acceptance: policy lookup requires policy id, version, matched category, severity, reviewer-visible rationale, and missing-policy fallback before promotion.\n\
Acceptance: reviewer assignment requires reviewer id, role, conflict check, SLA state, escalation path, and assignment result/error slots before promotion.\n\
Acceptance: evidence/source retention requires loaded event JSON availability, source hash, redaction-safe body preview, relation/thread context slots, and evidence upload result/error slots before promotion.\n\
Acceptance: reporter and target audit requires reporter id scope, target sender id scope, room id scope, privacy redaction state, and immutable audit trail slots before promotion.\n\
Acceptance: appeal workflow requires appeal eligibility, deadline, status, reviewer response, reversal/enforcement handoff, and user-visible result/error slots before promotion.\n\
Acceptance: enforcement requires explicit action type, redact/delete, ban, kick, ignore/block eligibility, confirmation state, result/error slots, and rollback handoff before promotion.\n\
Boundary: no extra MatrixRequest::ReportContent, retry without PositiveConfirmationModal, queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state, membership, message send/edit, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation."
    )
}

fn message_report_moderation_reviewer_packet_label(
    status: &str,
    event_id: Option<&EventId>,
    retry_cached: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "status waiting".to_string()
    } else {
        format!("status {}", status.trim())
    };
    let event_state = event_id
        .map(|event_id| format!("event {event_id}"))
        .unwrap_or_else(|| "event unavailable".to_string());
    let retry_state = if retry_cached {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded source available"
    } else {
        "loaded source unavailable"
    };

    format!(
        "Report moderation reviewer packet copied: {status_state}; {event_state}; {retry_state}; {source_state}. {MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_LABEL} No moderation queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_workflow_result_contract_packet_payload(
    status: &str,
    event_id: Option<&EventId>,
    reason: &str,
    error: Option<&str>,
    summary: &str,
    workflow_metadata: &str,
    preflight_metadata: &str,
    retry_cached: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "waiting"
    } else {
        status.trim()
    };
    let event_state = event_id
        .map(ToString::to_string)
        .unwrap_or_else(|| "event unavailable".to_string());
    let reason_state = compact_message_preview(reason, "empty reason");
    let error_state = error
        .map(|error| compact_message_preview(error, "empty error"))
        .unwrap_or_else(|| "no error result".to_string());
    let summary_state = compact_message_preview(summary, "summary unavailable");
    let workflow_state =
        compact_message_preview(workflow_metadata, "workflow metadata unavailable");
    let preflight_state =
        compact_message_preview(preflight_metadata, "preflight metadata unavailable");
    let retry_state = if retry_cached {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded event source available"
    } else {
        "loaded event source unavailable"
    };

    format!(
        "Message report typed workflow/result contract\n\
Status: {status_state}\n\
Event: {event_state}\n\
Reason: {reason_state}\n\
Error: {error_state}\n\
Summary: {summary_state}\n\
Workflow metadata: {workflow_state}\n\
Preflight metadata: {preflight_state}\n\
Retry: {retry_state}\n\
Loaded source: {source_state}\n\
Contract: queue item requires report id, stable event id, room id, reason, dedupe key, submit timestamp, queue state, cancel state, result id, and result/error slots.\n\
Contract: policy lookup requires policy id, version, matched rule, category, severity, reviewer-visible rationale, source hash, and missing-policy fallback slots.\n\
Contract: reviewer assignment requires reviewer id, role, conflict state, SLA state, escalation path, reassignment history, and assignment result/error slots.\n\
Contract: evidence/source requires loaded event JSON availability, event-context window, redaction-safe body preview, relation/thread context, source hash, evidence upload result/error, and retention policy slots.\n\
Contract: reporter/target audit requires reporter id scope, target sender id scope, privacy redaction state, immutable audit trail id, audit timestamp, and visibility result slots.\n\
Contract: appeal flow requires appeal eligibility, deadline, appeal status, reviewer response, reversal handoff, user-visible result, and error slots.\n\
Contract: enforcement result requires explicit action type, redact/delete, ban, kick, ignore/block eligibility, confirmation state, result event/action id, rollback handoff, and user-visible error slots.\n\
Contract: retry/cancel/source requires PositiveConfirmationModal retry gate, idempotency key, stale status guard, cancellation result, loaded source/source-hash handoff, and final result taxonomy.\n\
Promotion blocker: map moderation reviewer packet to typed workflow/result contracts before backend moderation work.\n\
Boundary: {MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_LABEL} No extra MatrixRequest::ReportContent, retry without PositiveConfirmationModal, queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state, membership, message send/edit, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation."
    )
}

fn message_report_workflow_result_contract_packet_label(
    status: &str,
    event_id: Option<&EventId>,
    retry_cached: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "status waiting".to_string()
    } else {
        format!("status {}", status.trim())
    };
    let event_state = event_id
        .map(|event_id| format!("event {event_id}"))
        .unwrap_or_else(|| "event unavailable".to_string());
    let retry_state = if retry_cached {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded source available"
    } else {
        "loaded source unavailable"
    };

    format!(
        "Report workflow Contract copied typed workflow/result packet to local clipboard: {status_state}; {event_state}; {retry_state}; {source_state}. {MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_LABEL} No queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_workflow_result_taxonomy_packet_payload(
    status: &str,
    event_id: Option<&EventId>,
    reason: &str,
    error: Option<&str>,
    summary: &str,
    workflow_metadata: &str,
    preflight_metadata: &str,
    retry_cached: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "waiting"
    } else {
        status.trim()
    };
    let event_state = event_id
        .map(ToString::to_string)
        .unwrap_or_else(|| "event unavailable".to_string());
    let reason_state = compact_message_preview(reason, "empty reason");
    let error_state = error
        .map(|error| compact_message_preview(error, "empty error"))
        .unwrap_or_else(|| "no error result".to_string());
    let summary_state = compact_message_preview(summary, "summary unavailable");
    let workflow_state =
        compact_message_preview(workflow_metadata, "workflow metadata unavailable");
    let preflight_state =
        compact_message_preview(preflight_metadata, "preflight metadata unavailable");
    let retry_state = if retry_cached {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded event source available"
    } else {
        "loaded event source unavailable"
    };

    format!(
        "Message report workflow result taxonomy packet\n\
Status: {status_state}\n\
Event: {event_state}\n\
Reason: {reason_state}\n\
Error: {error_state}\n\
Summary: {summary_state}\n\
Workflow metadata: {workflow_state}\n\
Preflight metadata: {preflight_state}\n\
Retry: {retry_state}\n\
Loaded source: {source_state}\n\
Live result references: confirmed MatrixRequest::ReportContent send/result/retry and loaded-or-source-fetch EventSourceModal only.\n\
Blocked queue_operation_id: not_assigned\n\
Blocked queue_result: queued, duplicate, cancelled, failed, stale not_wired\n\
Blocked policy_lookup_operation_id: not_assigned\n\
Blocked policy_result: matched, not_matched, failed, stale not_wired\n\
Blocked reviewer_assignment_operation_id: not_assigned\n\
Blocked reviewer_result: assigned, unassigned, conflict, failed, stale not_wired\n\
Blocked evidence_retention_operation_id: not_assigned\n\
Blocked evidence_result: retained, unavailable, failed, stale not_wired\n\
Blocked appeal_operation_id: not_assigned\n\
Blocked appeal_result: opened, updated, closed, failed, stale not_wired\n\
Blocked enforcement_operation_id: not_assigned\n\
Blocked enforcement_result: none, redacted, deleted, kicked, banned, ignored, blocked, permission_denied, failed, stale not_wired\n\
Retry policy: PositiveConfirmationModal, backend request id, source hash, and queue id required before any workflow retry.\n\
Cancel policy: local dismiss only; no queue cancel request and no ReportContent cancel.\n\
Stale policy: event id, room id, reason hash, source hash, and queue generation required before workflow promotion.\n\
Audit redaction: no access token, raw event JSON, raw reporter id, raw target user id, policy secret, reviewer identity, or full moderation reason in local packet.\n\
Boundary: {MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_LABEL} No extra MatrixRequest::ReportContent, retry without PositiveConfirmationModal, queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state, membership, message send/edit, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation."
    )
}

fn message_report_workflow_result_taxonomy_packet_label(
    status: &str,
    event_id: Option<&EventId>,
    retry_cached: bool,
    loaded_source_available: bool,
) -> String {
    let status_state = if status.trim().is_empty() {
        "status waiting".to_string()
    } else {
        format!("status {}", status.trim())
    };
    let event_state = event_id
        .map(|event_id| format!("event {event_id}"))
        .unwrap_or_else(|| "event unavailable".to_string());
    let retry_state = if retry_cached {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded source available"
    } else {
        "loaded source unavailable"
    };

    format!(
        "Report workflow Taxonomy copied blocked workflow result taxonomy to local clipboard: {status_state}; {event_state}; {retry_state}; {source_state}. {MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_LABEL} No queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_preflight_detail_label(
    action: Option<&str>,
    status: &str,
    event_id: Option<&EventId>,
    reason: &str,
    error: Option<&str>,
    retry_cached: bool,
    status_metadata: &str,
) -> String {
    let action_state = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} selected"))
        .unwrap_or_else(|| "no preflight detail selected".to_string());
    let status = if status.trim().is_empty() {
        "waiting"
    } else {
        status.trim()
    };
    let event_state = event_id
        .map(|event_id| format!("event {event_id}"))
        .unwrap_or_else(|| "event unavailable".to_string());
    let reason_chars = reason.trim().chars().count();
    let error_state = error
        .map(str::trim)
        .filter(|error| !error.is_empty())
        .map(|error| {
            format!(
                "error {} chars: {}",
                error.chars().count(),
                compact_message_preview(error, "empty error")
            )
        })
        .unwrap_or_else(|| "error cache empty".to_string());
    let retry_state = if retry_cached {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let source_state = if status_metadata.trim().is_empty() {
        "status metadata source empty".to_string()
    } else {
        format!(
            "status metadata source {} chars cached",
            status_metadata.chars().count()
        )
    };

    format!(
        "Report preflight detail: {action_state}; status {status}; {event_state}; reason {reason_chars} chars; {error_state}; {retry_state}; {source_state}. {MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_LABEL} No extra MatrixRequest::ReportContent, no retry without PositiveConfirmationModal, no cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn message_report_loaded_source_modal_label(
    event_id: Option<&EventId>,
    status: &str,
    reason: &str,
    loaded_index: Option<usize>,
    latest_json: Option<&str>,
    source_fetch_requested: bool,
) -> String {
    let event_state = event_id
        .map(|event_id| format!("event {event_id}"))
        .unwrap_or_else(|| "event unavailable".to_string());
    let status = if status.trim().is_empty() {
        "waiting"
    } else {
        status.trim()
    };
    let reason_chars = reason.trim().chars().count();
    let loaded_state = loaded_index
        .map(|index| format!("loaded row {index}"))
        .unwrap_or_else(|| "loaded row unavailable".to_string());
    let source_state = if let Some(json) = latest_json {
        format!(
            "loaded latest_json {} chars across {} line(s)",
            json.chars().count(),
            json.lines().count().max(1)
        )
    } else if source_fetch_requested {
        "source-only MatrixRequest::FetchEventSource requested".to_string()
    } else {
        "latest_json unavailable".to_string()
    };

    format!(
        "Report Source modal handoff: status {status}; {event_state}; reason {reason_chars} chars; {loaded_state}; {source_state}. {MESSAGE_REPORT_LOADED_SOURCE_MODAL_LABEL} FetchEventSource uses only current-room event source JSON when loaded latest_json is unavailable; no event-context fetch, extra ReportContent, retry automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state, membership, gateway/runtime/auth, or write-side live mutation."
    )
}

fn edit_history_local_diff_hint(original: &str, latest: &str) -> &'static str {
    if original == latest {
        "Local diff hint: latest preview matches the loaded original preview."
    } else {
        "Local diff hint: latest preview differs from the loaded original preview."
    }
}

fn edit_history_detail_summary_label(
    event_id: &EventId,
    replacement_count: usize,
    pages_fetched: usize,
    pagination_exhausted: bool,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
) -> String {
    let timestamp = latest_timestamp
        .map(|ts| format!("latest timestamp {}", ts.get()))
        .unwrap_or_else(|| "latest timestamp unavailable".to_string());
    let pagination_state =
        edit_history_relation_pagination_state_label(Some(pages_fetched), pagination_exhausted);
    format!(
        "Edit history for {event_id}: {replacement_count} replacement event(s), {pagination_state}, latest replacement {latest_event}, {timestamp}. {MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_LABEL}"
    )
}

fn edit_history_detail_diff_label(loaded_original: &str, latest_preview: &str) -> String {
    format!(
        "{} Loaded original: {}. Latest replacement: {}.",
        edit_history_local_diff_hint(loaded_original, latest_preview),
        loaded_original,
        latest_preview
    )
}

fn edit_history_detail_metadata_label(
    replacement_count: usize,
    pages_fetched: usize,
    pagination_exhausted: bool,
    loaded_original: &str,
    latest_preview: &str,
) -> String {
    let original_chars = loaded_original.chars().count();
    let latest_chars = latest_preview.chars().count();
    let pagination_state =
        edit_history_relation_pagination_state_label(Some(pages_fetched), pagination_exhausted);
    format!(
        "Result metadata: {replacement_count} m.replace relation(s), {pagination_state}, loaded original preview {original_chars} chars, latest replacement preview {latest_chars} chars. No full modal, event-context fetch, timeline reload, event source open, message mutation, room-state, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_relation_pagination_state_label(
    pages_fetched: Option<usize>,
    pagination_exhausted: bool,
) -> String {
    match pages_fetched {
        Some(pages) if pagination_exhausted => {
            format!("complete m.replace pagination exhausted after {pages} relation page(s)")
        }
        Some(pages) => {
            format!("m.replace pagination paused after {pages} relation page(s)")
        }
        None => "m.replace pagination waiting".to_string(),
    }
}

fn edit_history_local_full_snapshot_label(
    event_id: &str,
    replacement_count: Option<usize>,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
    error: &str,
    retry_cache_ready: bool,
) -> String {
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} compact replacement relation(s) loaded"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let latest_event_state = if latest_event.trim().is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {}", latest_event.trim())
    };
    let timestamp_state = latest_timestamp
        .map(|timestamp| format!("latest timestamp {}", timestamp.get()))
        .unwrap_or_else(|| "latest timestamp unavailable".to_string());
    let original_state = if loaded_original.trim().is_empty() {
        "original preview waiting".to_string()
    } else {
        format!(
            "original preview {} chars/{} bytes: {}",
            loaded_original.chars().count(),
            loaded_original.len(),
            loaded_original
        )
    };
    let latest_state = if latest_preview.trim().is_empty() {
        "latest preview waiting".to_string()
    } else {
        format!(
            "latest preview {} chars/{} bytes: {}",
            latest_preview.chars().count(),
            latest_preview.len(),
            latest_preview
        )
    };
    let delta_state = if loaded_original.trim().is_empty() && latest_preview.trim().is_empty() {
        "local delta waiting".to_string()
    } else if loaded_original == latest_preview {
        format!(
            "local delta unchanged; {}",
            edit_history_local_diff_hint(loaded_original, latest_preview)
        )
    } else {
        format!(
            "local delta differs; {}",
            edit_history_local_diff_hint(loaded_original, latest_preview)
        )
    };
    let error_state = if error.trim().is_empty() {
        "error cache empty".to_string()
    } else {
        format!("cached error {} chars: {}", error.chars().count(), error)
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not used"
    };
    format!(
        "Local full snapshot: {event_state}; {replacement_state}; {latest_event_state}; {timestamp_state}; {original_state}; {latest_state}; {delta_state}; {error_state}; {retry_state}. Full opens this loaded local snapshot from the paginated m.replace summary in EventSourceModal only. Remote full history modal UI, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message mutation, room-state, membership, gateway/runtime/auth, and live mutation remain unwired."
    )
}

fn edit_history_local_full_snapshot_modal_json(
    event_id: &str,
    replacement_count: Option<usize>,
    pages_fetched: Option<usize>,
    pagination_exhausted: bool,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
    latest_source_json: &str,
    error: &str,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let timestamp_ms = latest_timestamp.map(|timestamp| timestamp.get().to_string());
    let latest_source_json_available = !latest_source_json.trim().is_empty();
    let value = serde_json::json!({
        "kind": "hepta.telegram.edit_history.local_full_snapshot",
        "source": "cached MatrixRequest::FetchEditHistory + loaded RoomScreen timeline state",
        "target_event_id": event_id.trim(),
        "replacement_count": replacement_count,
        "relation_pages_fetched": pages_fetched,
        "pagination_exhausted": pagination_exhausted,
        "latest_replacement_event_id": latest_event.trim(),
        "latest_replacement_timestamp_ms": timestamp_ms,
        "loaded_original_preview": loaded_original,
        "loaded_original_chars": loaded_original.chars().count(),
        "latest_replacement_preview": latest_preview,
        "latest_replacement_chars": latest_preview.chars().count(),
        "latest_replacement_raw_json_available": latest_source_json_available,
        "latest_replacement_raw_json_chars": latest_source_json.chars().count(),
        "loaded_source_available": loaded_source_available,
        "cached_error": error,
        "retry_cache_ready": retry_cache_ready,
        "boundary": MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_LABEL,
        "side_effects": {
            "extra_fetch_edit_history": false,
            "remote_full_history_modal_request": false,
            "side_by_side_full_diff_rendering": false,
            "event_context_fetch": false,
            "timeline_pagination_reload": false,
            "message_send_edit_redact": false,
            "room_state_mutation": false,
            "membership_mutation": false,
            "account_profile_mutation": false,
            "gateway_runtime_auth": false,
            "write_side_live_mutation": false
        }
    });
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| {
        format!(
            "{{\"kind\":\"hepta.telegram.edit_history.local_full_snapshot\",\"target_event_id\":\"{}\",\"boundary\":\"{}\"}}",
            event_id.trim(),
            MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_LABEL
        )
    })
}

fn edit_history_local_full_snapshot_modal_label(
    event_id: &str,
    modal_opened: bool,
    snapshot_bytes: usize,
    replacement_count: Option<usize>,
    pages_fetched: Option<usize>,
    pagination_exhausted: bool,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let modal_state = if modal_opened {
        "opened local full snapshot EventSourceModal"
    } else {
        "local full snapshot modal unavailable because current timeline state is missing"
    };
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} compact replacement relation(s)"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let pagination_state =
        edit_history_relation_pagination_state_label(pages_fetched, pagination_exhausted);
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not used"
    };
    let source_state = if loaded_source_available {
        "loaded source available"
    } else {
        "loaded source unavailable"
    };
    format!(
        "Local full snapshot modal: {modal_state}; {event_state}; {replacement_state}; {pagination_state}; {source_state}; {retry_state}; snapshot JSON {snapshot_bytes} bytes. {MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_LABEL} No extra MatrixRequest::FetchEditHistory, no remote full-history modal request, no side-by-side full diff rendering, no event-context fetch, no timeline pagination/reload, no message mutation, no room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_full_modal_boundary_label(
    phase: &str,
    replacement_count: Option<usize>,
    pages_fetched: Option<usize>,
    pagination_exhausted: bool,
    retry_cache_ready: bool,
) -> String {
    let replacement_state = replacement_count
        .map(|count| format!("{count} compact replacement relation(s)"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let pagination_state =
        edit_history_relation_pagination_state_label(pages_fetched, pagination_exhausted);
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not used"
    };
    format!(
        "Full history boundary: {phase}; {replacement_state}; {pagination_state}; {retry_state}. {MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_LABEL} No remote full-history modal request, side-by-side full diff, event-context fetch, timeline pagination/reload, replacement event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_full_control_boundary_label(control: &str) -> String {
    format!(
        "{control} is local only. {MESSAGE_EDIT_HISTORY_FULL_CONTROLS_LABEL} No remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message mutation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_loaded_source_modal_label(
    source_kind: &str,
    event_id: &str,
    source_loaded: bool,
    latest_json: Option<&str>,
    loaded_original: &str,
    latest_preview: &str,
    replacement_count: Option<usize>,
    latest_event: &str,
) -> String {
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("{} event {}", source_kind.trim(), event_id.trim())
    };
    let open_state = if source_loaded {
        match source_kind.trim() {
            "latest replacement" => "opened latest replacement EventSourceModal",
            _ => "opened loaded original EventSourceModal",
        }
    } else if source_kind.trim() == "remote latest replacement request" {
        "requested remote latest replacement EventSourceModal source fetch"
    } else {
        "loaded edit source unavailable"
    };
    let json_state = latest_json
        .map(|json| {
            format!(
                "source JSON {} chars, {} line(s)",
                json.chars().count(),
                json.lines().count().max(1)
            )
        })
        .unwrap_or_else(|| "source JSON unavailable".to_string());
    let replacement_state = replacement_count
        .map(|count| format!("{count} replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let latest_event_state = if latest_event.trim().is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {}", latest_event.trim())
    };
    format!(
        "Loaded edit source: {open_state}; {event_state}; {json_state}; {replacement_state}; {latest_event_state}; original {} chars; latest {} chars. {MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_LABEL} Latest replacement source uses cached raw JSON returned by FetchEditHistory when available, can request Matrix room.event/load_or_fetch_event for the latest replacement source when the cache lacks JSON, and falls back to the loaded original source. No remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message mutation, room-state, membership, gateway/runtime/auth, or write-side live mutation.",
        loaded_original.chars().count(),
        latest_preview.chars().count(),
    )
}

fn edit_history_loaded_diff_detail_label(
    control: Option<&str>,
    event_id: &str,
    replacement_count: Option<usize>,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
    retry_cache_ready: bool,
) -> String {
    let control_state = control
        .map(|control| format!("{control} detail selected"))
        .unwrap_or_else(|| {
            "No Full/Diff/Context/Source/Packet/Contract detail selected".to_string()
        });
    let event_state = if event_id.is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {event_id}")
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} replacement relation(s) loaded"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let latest_event_state = if latest_event.is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {latest_event}")
    };
    let timestamp_state = latest_timestamp
        .map(|timestamp| format!("latest timestamp {}", timestamp.get()))
        .unwrap_or_else(|| "latest timestamp unavailable".to_string());
    let original_chars = loaded_original.chars().count();
    let latest_chars = latest_preview.chars().count();
    let preview_state = if loaded_original.is_empty() && latest_preview.is_empty() {
        "preview chars waiting".to_string()
    } else {
        format!("original {original_chars} chars, latest {latest_chars} chars")
    };
    let delta_state = if loaded_original.is_empty() && latest_preview.is_empty() {
        "delta waiting".to_string()
    } else if loaded_original == latest_preview {
        "local delta matches loaded original".to_string()
    } else {
        "local delta differs from loaded original".to_string()
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not used"
    };
    format!(
        "Loaded diff detail: {control_state}; {event_state}; {replacement_state}; {latest_event_state}; {timestamp_state}; {preview_state}; {delta_state}; {retry_state}. {MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_LABEL} No remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message mutation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

struct EditHistoryLoadedDiffModalSnapshot {
    json: String,
    loaded_full_body: bool,
}

#[allow(dead_code)]
fn edit_history_loaded_side_by_side_diff_modal_json(
    event_id: &str,
    replacement_count: Option<usize>,
    pages_fetched: Option<usize>,
    pagination_exhausted: bool,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
) -> Option<String> {
    edit_history_loaded_side_by_side_diff_modal_json_with_sources(
        event_id,
        replacement_count,
        pages_fetched,
        pagination_exhausted,
        latest_event,
        latest_timestamp,
        loaded_original,
        latest_preview,
        None,
        "",
    )
    .map(|snapshot| snapshot.json)
}

fn edit_history_loaded_side_by_side_diff_modal_json_with_sources(
    event_id: &str,
    replacement_count: Option<usize>,
    pages_fetched: Option<usize>,
    pagination_exhausted: bool,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
    original_source_json: Option<&str>,
    latest_source_json: &str,
) -> Option<EditHistoryLoadedDiffModalSnapshot> {
    let event_id = event_id.trim();
    if event_id.is_empty()
        || (loaded_original.trim().is_empty() && latest_preview.trim().is_empty())
    {
        return None;
    }

    let original_body =
        original_source_json.and_then(|json| edit_history_body_from_event_source_json(json, false));
    let latest_body = edit_history_body_from_event_source_json(latest_source_json, true);
    let loaded_full_body = original_body.is_some() && latest_body.is_some();
    let original_body_source = if original_body.is_some() {
        "loaded original latest_json body"
    } else {
        "loaded original preview fallback"
    };
    let latest_body_source = if latest_body.is_some() {
        "cached latest replacement raw JSON body"
    } else {
        "latest replacement preview fallback"
    };
    let original_text = original_body.as_deref().unwrap_or(loaded_original);
    let latest_text = latest_body.as_deref().unwrap_or(latest_preview);
    let original_lines = preview_lines_for_side_by_side(original_text);
    let latest_lines = preview_lines_for_side_by_side(latest_text);
    let row_count = original_lines.len().max(latest_lines.len());
    let rows: Vec<serde_json::Value> = (0..row_count)
        .map(|index| {
            let original = original_lines.get(index).cloned().unwrap_or_default();
            let latest = latest_lines.get(index).cloned().unwrap_or_default();
            let changed = original != latest;
            serde_json::json!({
                "line": index + 1,
                "original": original,
                "latest": latest,
                "changed": changed,
            })
        })
        .collect();
    let timestamp_ms = latest_timestamp.map(|timestamp| timestamp.get().to_string());
    let value = serde_json::json!({
        "kind": "hepta.telegram.edit_history.loaded_side_by_side_preview_full_body_diff",
        "source": "cached MatrixRequest::FetchEditHistory + loaded original timeline preview",
        "target_event_id": event_id,
        "replacement_count": replacement_count,
        "relation_pages_fetched": pages_fetched,
        "pagination_exhausted": pagination_exhausted,
        "latest_replacement_event_id": latest_event.trim(),
        "latest_replacement_timestamp_ms": timestamp_ms,
        "rendering_scope": if loaded_full_body {
            "loaded full body rows from cached source JSON; remote full-history body is not requested"
        } else {
            "loaded preview rows only; remote full-history body is not requested"
        },
        "loaded_full_body_side_by_side_snapshot": loaded_full_body,
        "original_body_source": original_body_source,
        "latest_body_source": latest_body_source,
        "delta_hint": edit_history_local_diff_hint(original_text, latest_text),
        "loaded_original_chars": original_text.chars().count(),
        "latest_replacement_chars": latest_text.chars().count(),
        "side_by_side_rows": rows,
        "boundary": MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_LABEL,
        "side_effects": {
            "extra_fetch_edit_history": false,
            "remote_full_history_modal_request": false,
            "loaded_preview_side_by_side_snapshot": true,
            "loaded_full_body_side_by_side_snapshot": loaded_full_body,
            "server_backed_full_body_side_by_side_diff_rendering": false,
            "event_context_fetch": false,
            "timeline_pagination_reload": false,
            "replacement_event_source_fetch": false,
            "message_send_edit_redact": false,
            "room_state_mutation": false,
            "membership_mutation": false,
            "account_profile_mutation": false,
            "gateway_runtime_auth": false,
            "write_side_live_mutation": false
        }
    });

    serde_json::to_string_pretty(&value)
        .ok()
        .map(|json| EditHistoryLoadedDiffModalSnapshot {
            json,
            loaded_full_body,
        })
}

fn edit_history_body_from_event_source_json(
    raw_json: &str,
    prefer_new_content: bool,
) -> Option<String> {
    let value = serde_json::from_str::<serde_json::Value>(raw_json).ok()?;
    let content = value.get("content")?;
    if prefer_new_content {
        if let Some(body) = content
            .get("m.new_content")
            .and_then(edit_history_body_from_event_content_value)
        {
            return Some(body);
        }
    }
    edit_history_body_from_event_content_value(content)
}

fn edit_history_body_from_event_content_value(content: &serde_json::Value) -> Option<String> {
    content
        .get("body")
        .or_else(|| content.get("formatted_body"))
        .and_then(|value| value.as_str())
        .map(str::to_string)
}

fn preview_lines_for_side_by_side(text: &str) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    if text.ends_with('\n') {
        lines.push(String::new());
    }
    if lines.is_empty() {
        lines.push(text.to_string());
    }
    lines
}

fn edit_history_loaded_diff_clipboard_payload(
    event_id: &str,
    replacement_count: Option<usize>,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
) -> Option<String> {
    let event_id = event_id.trim();
    if event_id.is_empty()
        || (loaded_original.trim().is_empty() && latest_preview.trim().is_empty())
    {
        return None;
    }
    let replacement_state = replacement_count
        .map(|count| format!("{count} compact replacement relation(s)"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let latest_event_state = if latest_event.trim().is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {}", latest_event.trim())
    };
    let timestamp_state = latest_timestamp
        .map(|timestamp| format!("latest timestamp {}", timestamp.get()))
        .unwrap_or_else(|| "latest timestamp unavailable".to_string());
    let delta_state = edit_history_local_diff_hint(loaded_original, latest_preview);
    Some(format!(
        "Edit history compact diff\nTarget: {event_id}\n{replacement_state}\n{latest_event_state}\n{timestamp_state}\n{delta_state}\nOriginal preview ({} chars, {} bytes): {}\nLatest preview ({} chars, {} bytes): {}\n{MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_LABEL}\nNo remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message mutation, room-state, membership, gateway/runtime/auth, or live mutation.",
        loaded_original.chars().count(),
        loaded_original.len(),
        loaded_original,
        latest_preview.chars().count(),
        latest_preview.len(),
        latest_preview,
    ))
}

fn edit_history_loaded_diff_clipboard_label(
    event_id: &str,
    copied: bool,
    modal_opened: bool,
    modal_json_bytes: Option<usize>,
    loaded_full_body_diff: bool,
    replacement_count: Option<usize>,
    latest_event: &str,
    loaded_original: &str,
    latest_preview: &str,
) -> String {
    let action_state = if copied {
        "copied loaded compact diff to local clipboard"
    } else {
        "diff clipboard unavailable"
    };
    let modal_state = if modal_opened {
        let modal_kind = if loaded_full_body_diff {
            "loaded full-body side-by-side diff modal"
        } else {
            "loaded side-by-side preview diff modal"
        };
        match modal_json_bytes {
            Some(bytes) => format!("opened {modal_kind} {bytes} bytes"),
            None => format!("opened {modal_kind}"),
        }
    } else if modal_json_bytes.is_some() {
        "side-by-side preview diff modal unavailable because current timeline state is missing"
            .to_string()
    } else {
        "side-by-side preview diff modal unavailable because target or preview data is missing"
            .to_string()
    };
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let latest_event_state = if latest_event.trim().is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {}", latest_event.trim())
    };
    let preview_state = if loaded_original.trim().is_empty() && latest_preview.trim().is_empty() {
        "preview data waiting".to_string()
    } else {
        format!(
            "original {} chars/{} bytes, latest {} chars/{} bytes",
            loaded_original.chars().count(),
            loaded_original.len(),
            latest_preview.chars().count(),
            latest_preview.len()
        )
    };
    format!(
        "Loaded diff clipboard: {action_state}; {modal_state}; {event_state}; {replacement_state}; {latest_event_state}; {preview_state}. {MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_LABEL} {MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_LABEL} No remote full-history modal request, server-backed side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_full_diff_packet_payload(
    event_id: &str,
    replacement_count: Option<usize>,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
    error: &str,
    retry_cache_ready: bool,
    loaded_source_available: bool,
    loaded_diff_detail: &str,
    preflight_detail: &str,
    full_boundary: &str,
) -> String {
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} compact replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let latest_event_state = if latest_event.trim().is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {}", latest_event.trim())
    };
    let timestamp_state = latest_timestamp
        .map(|timestamp| format!("latest timestamp {}", timestamp.get()))
        .unwrap_or_else(|| "latest timestamp unavailable".to_string());
    let preview_state = if loaded_original.trim().is_empty() && latest_preview.trim().is_empty() {
        "preview data waiting".to_string()
    } else {
        format!(
            "original {} chars/{} bytes, latest {} chars/{} bytes, {}",
            loaded_original.chars().count(),
            loaded_original.len(),
            latest_preview.chars().count(),
            latest_preview.len(),
            edit_history_local_diff_hint(loaded_original, latest_preview)
        )
    };
    let error_state = if error.trim().is_empty() {
        "error cache empty".to_string()
    } else {
        format!(
            "cached error {} chars: {}",
            error.trim().chars().count(),
            compact_message_preview(error, "empty error")
        )
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded original source available"
    } else {
        "loaded original source unavailable"
    };
    let loaded_diff_state =
        compact_message_preview(loaded_diff_detail, "loaded diff detail unavailable");
    let preflight_state = compact_message_preview(preflight_detail, "preflight detail unavailable");
    let boundary_state = compact_message_preview(full_boundary, "full boundary unavailable");

    format!(
        "Edit history loaded/full diff packet\n\
Target: {event_state}\n\
Replacement: {replacement_state}\n\
Latest: {latest_event_state}; {timestamp_state}\n\
Previews: {preview_state}\n\
Error: {error_state}\n\
Retry: {retry_state}\n\
Loaded source: {source_state}\n\
Loaded diff detail: {loaded_diff_state}\n\
Preflight detail: {preflight_state}\n\
Full boundary: {boundary_state}\n\
Acceptance: remote full-history modal request/result/error requires target event id, replacement count cursor, latest replacement event, loaded original preview, retry/cancel state, and source provenance before promotion.\n\
Evidence: complete replacement pagination is live through FetchEditHistory following Room::relations next_batch to exhaustion and caching relation page metadata before any full modal work.\n\
Acceptance: side-by-side full diff rendering requires original body, each replacement body, timestamp/author metadata, redaction-safe HTML/plaintext normalization, and overflow/empty-state rules before promotion.\n\
Acceptance: event context requires thread/reply/relation context slots, context result/error/retry state, and no BackwardsPaginateUntilEvent or timeline pagination until a typed contract exists.\n\
Acceptance: replacement event source requires source request/result/error slots, source hash, loaded original fallback, and unavailable-source copy before promotion.\n\
Acceptance: loaded original source requires existing EventSourceModal-only fallback, source hash, and local latest_json availability before promotion.\n\
Acceptance: retry/cancel requires PositiveConfirmationModal for retry, local cancel copy, cached TimelineKind, and no automatic refetch before promotion.\n\
Boundary: no extra MatrixRequest::FetchEditHistory, retry without PositiveConfirmationModal, remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation."
    )
}

fn edit_history_full_diff_packet_label(
    event_id: &str,
    replacement_count: Option<usize>,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded source available"
    } else {
        "loaded source unavailable"
    };

    format!(
        "Edit-history Packet copied loaded/full diff remote modal contract locally: {event_state}; {replacement_state}; {retry_state}; {source_state}. {MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_LABEL} No extra FetchEditHistory, remote full modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_full_history_result_contract_packet_payload(
    event_id: &str,
    replacement_count: Option<usize>,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
    error: &str,
    retry_cache_ready: bool,
    loaded_source_available: bool,
    loaded_diff_detail: &str,
    preflight_detail: &str,
    full_boundary: &str,
) -> String {
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} compact replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let latest_event_state = if latest_event.trim().is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {}", latest_event.trim())
    };
    let timestamp_state = latest_timestamp
        .map(|timestamp| format!("latest timestamp {}", timestamp.get()))
        .unwrap_or_else(|| "latest timestamp unavailable".to_string());
    let preview_state = if loaded_original.trim().is_empty() && latest_preview.trim().is_empty() {
        "preview data waiting".to_string()
    } else {
        format!(
            "original {} chars/{} bytes, latest {} chars/{} bytes, {}",
            loaded_original.chars().count(),
            loaded_original.len(),
            latest_preview.chars().count(),
            latest_preview.len(),
            edit_history_local_diff_hint(loaded_original, latest_preview)
        )
    };
    let error_state = if error.trim().is_empty() {
        "error cache empty".to_string()
    } else {
        format!(
            "cached error {} chars: {}",
            error.trim().chars().count(),
            compact_message_preview(error, "empty error")
        )
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded original source available"
    } else {
        "loaded original source unavailable"
    };
    let loaded_diff_state =
        compact_message_preview(loaded_diff_detail, "loaded diff detail unavailable");
    let preflight_state = compact_message_preview(preflight_detail, "preflight detail unavailable");
    let boundary_state = compact_message_preview(full_boundary, "full boundary unavailable");

    format!(
        "Edit history typed full-history modal/result contract packet\n\
Target: {event_state}\n\
Replacement: {replacement_state}\n\
Latest: {latest_event_state}; {timestamp_state}\n\
Previews: {preview_state}\n\
Error: {error_state}\n\
Retry: {retry_state}\n\
Loaded source: {source_state}\n\
Loaded diff detail: {loaded_diff_state}\n\
Preflight detail: {preflight_state}\n\
Full boundary: {boundary_state}\n\
Contract: full-history modal request slots require target event id, timeline kind, room id, relation type, relation pages/exhausted metadata, source-hash seed, retry idempotency key, and stale-target guard.\n\
Contract: full-history modal result slots require original event identity, ordered replacement list, latest replacement event, author/timestamp metadata, replacement count cursor, loaded original preview, and unavailable-source copy.\n\
Contract: side-by-side diff slots require original body, replacement body list, normalized HTML/plaintext, diff segment ranges, redaction-safe fallback, overflow rules, empty-state copy, and accessibility summary.\n\
Contract: event context slots require thread/reply/relation context, context result/error/retry/source state, and no BackwardsPaginateUntilEvent or timeline pagination until backend contracts exist.\n\
Contract: replacement source slots require per-replacement source request/result/error, source hash, loaded original EventSourceModal fallback, source unavailable copy, and source provenance before promotion.\n\
Contract: retry/cancel slots require PositiveConfirmationModal retry, local cancel state, cached TimelineKind, stale cursor handling, and no automatic refetch.\n\
Promotion blocker: {MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_LABEL} Backend edit-history modal/result contracts must be coordinated before full modal, event context, replacement source, or diff rendering work is wired.\n\
Boundary: no extra MatrixRequest::FetchEditHistory, retry without PositiveConfirmationModal, remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation."
    )
}

fn edit_history_full_history_result_contract_packet_label(
    event_id: &str,
    replacement_count: Option<usize>,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded source available"
    } else {
        "loaded source unavailable"
    };

    format!(
        "Edit-history Contract copied typed full-history modal/result contract locally: {event_state}; {replacement_state}; {retry_state}; {source_state}. {MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_LABEL} No extra FetchEditHistory, remote full modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_remote_result_taxonomy_packet_payload(
    event_id: &str,
    replacement_count: Option<usize>,
    pages_fetched: Option<usize>,
    pagination_exhausted: bool,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
    error: &str,
    retry_cache_ready: bool,
    loaded_source_available: bool,
    loaded_diff_detail: &str,
    preflight_detail: &str,
    full_boundary: &str,
) -> String {
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} compact replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let pagination_state =
        edit_history_relation_pagination_state_label(pages_fetched, pagination_exhausted);
    let latest_event_state = if latest_event.trim().is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {}", latest_event.trim())
    };
    let timestamp_state = latest_timestamp
        .map(|timestamp| format!("latest timestamp {}", timestamp.get()))
        .unwrap_or_else(|| "latest timestamp unavailable".to_string());
    let preview_state = if loaded_original.trim().is_empty() && latest_preview.trim().is_empty() {
        "preview data waiting".to_string()
    } else {
        format!(
            "original {} chars/{} bytes, latest {} chars/{} bytes, {}",
            loaded_original.chars().count(),
            loaded_original.len(),
            latest_preview.chars().count(),
            latest_preview.len(),
            edit_history_local_diff_hint(loaded_original, latest_preview)
        )
    };
    let error_state = if error.trim().is_empty() {
        "error cache empty".to_string()
    } else {
        format!(
            "cached error {} chars: {}",
            error.trim().chars().count(),
            compact_message_preview(error, "empty error")
        )
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded source available"
    } else {
        "loaded source unavailable"
    };
    let loaded_diff_state =
        compact_message_preview(loaded_diff_detail, "loaded diff detail unavailable");
    let preflight_state = compact_message_preview(preflight_detail, "preflight detail unavailable");
    let boundary_state = compact_message_preview(full_boundary, "full boundary unavailable");

    format!(
        "Edit history remote full-history/source result taxonomy packet\n\
Target: {event_state}\n\
Replacement: {replacement_state}; {pagination_state}\n\
Latest: {latest_event_state}; {timestamp_state}\n\
Previews: {preview_state}\n\
Error: {error_state}\n\
Retry: {retry_state}\n\
Loaded source: {source_state}\n\
Loaded diff detail: {loaded_diff_state}\n\
Preflight detail: {preflight_state}\n\
Full boundary: {boundary_state}\n\
Live result references: paginated MatrixRequest::FetchEditHistory through Room::relations next_batch exhaustion, confirmed failed-state Retry, local synthetic Full EventSourceModal snapshot, loaded side-by-side preview/full-body diff EventSourceModal snapshot, compact diff clipboard handoff, cached latest replacement raw JSON EventSourceModal handoff, source-only MatrixRequest::FetchEventSource / Room::load_or_fetch_event fallback, and loaded original EventSourceModal fallback only.\n\
Blocked remote_full_history_request_id: not_assigned\n\
Blocked full_history_cursor_id: not_assigned\n\
Blocked full_history_page_result: ready, empty, exhausted, failed, stale not_wired\n\
Blocked server_backed_full_diff_operation_id: not_assigned\n\
Blocked server_backed_full_diff_result: rendered, partial_source, missing_original, missing_replacement, unsupported, failed, stale not_wired\n\
Blocked replacement_source_reconciliation_operation_id: not_assigned\n\
Blocked replacement_source_result: loaded, unavailable, forbidden, redacted, failed, stale not_wired\n\
Blocked event_context_operation_id: not_assigned\n\
Blocked event_context_result: before_after_window, thread_context, relation_context, failed, stale not_wired\n\
Blocked stale_target_result: target_hash_mismatch, cursor_generation_mismatch, source_hash_mismatch not_wired\n\
Retry policy: PositiveConfirmationModal, cached TimelineKind, request id, cursor id, and source hash required before any remote full-history retry; current retry is compact FetchEditHistory only.\n\
Cancel policy: local dismiss only; no full-history cancel request and no relation fetch cancellation.\n\
Source-hash policy: original event hash, latest replacement source hash, relation page hash, body-normalization hash, and target hash required before backend promotion.\n\
Audit redaction: no access token, raw homeserver credential, raw source JSON in gate reports, unredacted replacement body, sender profile secret, or request cursor secret in local taxonomy.\n\
Boundary: {MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_LABEL} No extra MatrixRequest::FetchEditHistory, retry without PositiveConfirmationModal, remote full-history modal request, server-backed side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch beyond the existing Source fallback, message send/edit/redact, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation."
    )
}

fn edit_history_remote_result_taxonomy_packet_label(
    event_id: &str,
    replacement_count: Option<usize>,
    retry_cache_ready: bool,
    loaded_source_available: bool,
) -> String {
    let event_state = if event_id.trim().is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {}", event_id.trim())
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement count waiting".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if loaded_source_available {
        "loaded source available"
    } else {
        "loaded source unavailable"
    };

    format!(
        "Edit-history Taxonomy copied remote full-history/source reconciliation result slots locally: {event_state}; {replacement_state}; {retry_state}; {source_state}. {MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_LABEL} No extra FetchEditHistory, remote full modal request, server-backed full diff rendering, event-context fetch, timeline pagination/reload, replacement source fetch, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_preflight_detail_label(
    control: Option<&str>,
    event_id: &str,
    replacement_count: Option<usize>,
    latest_event: &str,
    latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    loaded_original: &str,
    latest_preview: &str,
    error: &str,
    retry_cache_ready: bool,
    metadata: &str,
    boundary: &str,
) -> String {
    let control_state = control
        .map(|control| format!("{control} selected"))
        .unwrap_or_else(|| "No Request/Result/Error/Retry/Source detail selected".to_string());
    let phase_state = if !error.is_empty() {
        "failed compact read cached".to_string()
    } else if replacement_count.is_some() {
        "result cached from compact m.replace summary".to_string()
    } else if !event_id.is_empty() {
        "request metadata cached for compact m.replace summary".to_string()
    } else {
        "waiting for edited badge request".to_string()
    };
    let event_state = if event_id.is_empty() {
        "target event waiting".to_string()
    } else {
        format!("target event {event_id}")
    };
    let replacement_state = replacement_count
        .map(|count| format!("{count} replacement relation(s) cached"))
        .unwrap_or_else(|| "replacement result waiting".to_string());
    let latest_event_state = if latest_event.is_empty() {
        "latest replacement event waiting".to_string()
    } else {
        format!("latest replacement event {latest_event}")
    };
    let timestamp_state = latest_timestamp
        .map(|timestamp| format!("latest timestamp {}", timestamp.get()))
        .unwrap_or_else(|| "latest timestamp unavailable".to_string());
    let original_chars = loaded_original.chars().count();
    let latest_chars = latest_preview.chars().count();
    let preview_state = if loaded_original.is_empty() && latest_preview.is_empty() {
        "preview counts waiting".to_string()
    } else {
        format!("original {original_chars} chars, latest {latest_chars} chars")
    };
    let error_state = if error.is_empty() {
        "error cache empty".to_string()
    } else {
        format!("error cache {} chars", error.chars().count())
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache not ready"
    };
    let source_state = if metadata.is_empty() && boundary.is_empty() {
        "source metadata waiting".to_string()
    } else {
        format!(
            "source metadata {} chars, boundary {} chars",
            metadata.chars().count(),
            boundary.chars().count()
        )
    };
    format!(
        "Edit history preflight detail: {control_state}; controls Request, Result, Error, Retry, Source; {phase_state}; {event_state}; {replacement_state}; {latest_event_state}; {timestamp_state}; {preview_state}; {error_state}; {retry_state}; {source_state}. {MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_LABEL} No extra MatrixRequest::FetchEditHistory, no retry without PositiveConfirmationModal, no remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement source fetch, message mutation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn edit_history_retry_confirmation_label(event_id: &EventId) -> String {
    format!(
        "Retry compact edit history read for {event_id}? {MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_LABEL} No remote full-history modal request, full diff rendering, event-context fetch, timeline pagination/reload, event source open, message mutation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

#[cfg(test)]
mod message_report_status_lifecycle_tests {
    use super::*;

    #[test]
    fn message_report_status_summary_is_event_scoped() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label = message_report_status_summary_label(&event_id, "submitted", "spam reason");

        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("submitted"));
        assert!(label.contains("spam reason"));
        assert!(label.contains("ReportContent result"));
    }

    #[test]
    fn message_report_status_metadata_keeps_moderation_gaps_explicit() {
        let label = message_report_status_metadata_label(
            "failed",
            "custom moderation reason",
            Some("permission denied"),
        );

        assert!(label.contains("state failed"));
        assert!(label.contains("reason 24 chars"));
        assert!(label.contains("permission denied"));
        assert!(label.contains("No retry"));
        assert!(label.contains("moderation policy lookup"));
        assert!(label.contains("ban"));
        assert!(label.contains("kick"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_report_status_clipboard_payload_uses_cached_status_only() {
        let event_id = EventId::parse("$reported:example.org").unwrap();
        let payload = message_report_status_clipboard_payload(
            "failed",
            Some(&event_id),
            "spam reason",
            Some("permission denied"),
            "Report failed for loaded event",
            "Report status metadata cached",
            "Report preflight detail cached",
        )
        .unwrap();

        assert!(payload.contains("Message report status"));
        assert!(payload.contains("Status: failed"));
        assert!(payload.contains("$reported:example.org"));
        assert!(payload.contains("Reason: spam reason"));
        assert!(payload.contains("Error: permission denied"));
        assert!(payload.contains("Scope: cached RoomScreen report status"));
        assert!(payload.contains("no extra ReportContent"));
        assert!(payload.contains("moderation queue"));
        assert!(payload.contains("policy lookup"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
        assert!(MESSAGE_REPORT_STATUS_CLIPBOARD_EVIDENCE.contains("local clipboard"));
    }

    #[test]
    fn message_report_status_clipboard_payload_requires_real_status() {
        assert!(message_report_status_clipboard_payload("", None, "", None, "", "", "").is_none());
        assert!(
            message_report_status_clipboard_payload("waiting", None, "", None, "", "", "")
                .is_none()
        );
        assert!(
            message_report_status_clipboard_payload("local", None, "", None, "", "", "").is_none()
        );
    }

    #[test]
    fn message_report_status_clipboard_label_reports_copied_and_unavailable_states() {
        let event_id = EventId::parse("$reported:example.org").unwrap();
        let copied = message_report_status_clipboard_label(
            true,
            "sent",
            Some(&event_id),
            "spam reason",
            None,
            "Report sent for loaded event",
        );
        let unavailable = message_report_status_clipboard_label(false, "", None, "", None, "");

        assert!(copied.contains("copied cached report status to local clipboard"));
        assert!(copied.contains("status sent"));
        assert!(copied.contains("$reported:example.org"));
        assert!(copied.contains("reason 11 chars"));
        assert!(copied.contains("error cache empty"));
        assert!(copied.contains(MESSAGE_REPORT_STATUS_CLIPBOARD_LABEL));
        assert!(copied.contains("No extra MatrixRequest::ReportContent"));
        assert!(copied.contains("moderation policy lookup"));
        assert!(unavailable.contains("report status clipboard unavailable"));
        assert!(unavailable.contains("status waiting"));
        assert!(unavailable.contains("event unavailable"));
    }

    #[test]
    fn message_report_retry_confirmation_is_confirmed_and_narrow() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label = message_report_retry_confirmation_label(&event_id, "spam reason");

        assert!(label.contains("Retry report_content"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("spam reason"));
        assert!(label.contains("Retry confirms before reusing ReportContent"));
        assert!(label.contains("No retry queue automation"));
        assert!(label.contains("moderation policy lookup"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_report_workflow_action_boundary_is_local_only() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label = message_report_workflow_action_boundary_label(
            "Reviewer assignment",
            "failed",
            Some(&event_id),
            "spam reason",
        );

        assert!(label.contains("Reviewer assignment stayed local"));
        assert!(label.contains("report status failed"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("spam reason"));
        assert!(
            label
                .contains("Queue, Policy, Assign, Appeal, Enforce, Packet, Contract, and Taxonomy")
        );
        assert!(label.contains("server policy fetch"));
        assert!(label.contains("appeal workflow"));
        assert!(label.contains("redact/delete"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_report_local_moderation_snapshot_renders_cached_packet() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label = message_report_local_moderation_snapshot_label(
            "failed",
            Some(&event_id),
            "spam reason",
            Some("permission denied"),
            "Report failed after server response",
            "Workflow metadata cached",
            "Preflight detail cached",
            true,
        );

        assert!(label.contains("Local moderation packet snapshot"));
        assert!(label.contains("failed"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("spam reason"));
        assert!(label.contains("error 17 chars: permission denied"));
        assert!(label.contains("summary 35 chars"));
        assert!(label.contains("workflow metadata 24 chars"));
        assert!(label.contains("preflight detail 23 chars"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("Queue renders this cached local ReportContent packet"));
        assert!(label.contains("server policy fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE
                .contains("local moderation packet snapshot")
        );
    }

    #[test]
    fn message_report_moderation_reviewer_packet_payload_lists_acceptance_matrix() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let payload = message_report_moderation_reviewer_packet_payload(
            "failed",
            Some(&event_id),
            "spam reason",
            Some("permission denied"),
            "Report failed after server response",
            "Workflow metadata cached",
            "Preflight detail cached",
            true,
            true,
        );

        assert!(payload.contains("Message report moderation reviewer packet"));
        assert!(payload.contains("Status: failed"));
        assert!(payload.contains("$reported:example.org"));
        assert!(payload.contains("Reason: spam reason"));
        assert!(payload.contains("Error: permission denied"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains("Loaded source: loaded event source available"));
        assert!(payload.contains("moderation queue persistence"));
        assert!(payload.contains("policy id"));
        assert!(payload.contains("reviewer assignment"));
        assert!(payload.contains("evidence/source retention"));
        assert!(payload.contains("reporter and target audit"));
        assert!(payload.contains("appeal workflow"));
        assert!(payload.contains("enforcement"));
        assert!(payload.contains("no extra MatrixRequest::ReportContent"));
        assert!(payload.contains("PositiveConfirmationModal"));
        assert!(payload.contains("gateway/runtime/auth/provider"));
        assert!(payload.contains("Telegram delivery"));
        assert!(payload.contains("live mutation"));
        assert!(
            MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE
                .contains("moderation reviewer acceptance matrix")
        );
    }

    #[test]
    fn message_report_moderation_reviewer_packet_label_reports_copy_state() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label =
            message_report_moderation_reviewer_packet_label("failed", Some(&event_id), true, false);

        assert!(label.contains("Report moderation reviewer packet copied"));
        assert!(label.contains("status failed"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("loaded source unavailable"));
        assert!(label.contains(MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_LABEL));
        assert!(label.contains("policy lookup"));
        assert!(label.contains("reviewer assignment"));
        assert!(label.contains("appeal/enforcement workflow"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_report_workflow_result_contract_packet_payload_lists_typed_contracts() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let payload = message_report_workflow_result_contract_packet_payload(
            "failed",
            Some(&event_id),
            "spam reason",
            Some("permission denied"),
            "Report failed after server response",
            "Workflow metadata cached",
            "Preflight detail cached",
            true,
            true,
        );

        assert!(payload.contains("Message report typed workflow/result contract"));
        assert!(payload.contains("Status: failed"));
        assert!(payload.contains("$reported:example.org"));
        assert!(payload.contains("Reason: spam reason"));
        assert!(payload.contains("Error: permission denied"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains("Loaded source: loaded event source available"));
        assert!(payload.contains("queue item requires report id"));
        assert!(payload.contains("policy lookup requires policy id"));
        assert!(payload.contains("reviewer assignment requires reviewer id"));
        assert!(payload.contains("evidence/source requires loaded event JSON"));
        assert!(payload.contains("reporter/target audit"));
        assert!(payload.contains("appeal flow"));
        assert!(payload.contains("enforcement result"));
        assert!(payload.contains("retry/cancel/source"));
        assert!(payload.contains("source hash"));
        assert!(payload.contains(MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_LABEL));
        assert!(payload.contains("No extra MatrixRequest::ReportContent"));
        assert!(payload.contains("PositiveConfirmationModal"));
        assert!(payload.contains("gateway/runtime/auth/provider"));
        assert!(payload.contains("Telegram delivery"));
        assert!(payload.contains("live mutation"));
        assert!(
            MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_EVIDENCE
                .contains("typed moderation workflow/result contract packet")
        );
    }

    #[test]
    fn message_report_workflow_result_contract_packet_label_reports_copy_state() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label = message_report_workflow_result_contract_packet_label(
            "failed",
            Some(&event_id),
            true,
            false,
        );

        assert!(label.contains("Report workflow Contract copied"));
        assert!(label.contains("status failed"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("loaded source unavailable"));
        assert!(label.contains(MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_LABEL));
        assert!(label.contains("policy lookup"));
        assert!(label.contains("reviewer assignment"));
        assert!(label.contains("appeal/enforcement workflow"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_report_workflow_result_taxonomy_packet_payload_lists_blocked_results() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let payload = message_report_workflow_result_taxonomy_packet_payload(
            "failed",
            Some(&event_id),
            "spam reason",
            Some("permission denied"),
            "Report failed after server response",
            "Workflow metadata cached",
            "Preflight detail cached",
            true,
            true,
        );

        assert!(payload.contains("Message report workflow result taxonomy packet"));
        assert!(payload.contains("Status: failed"));
        assert!(payload.contains("$reported:example.org"));
        assert!(payload.contains("Reason: spam reason"));
        assert!(payload.contains("Error: permission denied"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains("Loaded source: loaded event source available"));
        assert!(payload.contains("confirmed MatrixRequest::ReportContent"));
        assert!(payload.contains("loaded-or-source-fetch EventSourceModal"));
        assert!(payload.contains("queue_operation_id: not_assigned"));
        assert!(
            payload.contains("queue_result: queued, duplicate, cancelled, failed, stale not_wired")
        );
        assert!(payload.contains("policy_lookup_operation_id: not_assigned"));
        assert!(payload.contains("reviewer_assignment_operation_id: not_assigned"));
        assert!(payload.contains("evidence_retention_operation_id: not_assigned"));
        assert!(payload.contains("appeal_operation_id: not_assigned"));
        assert!(payload.contains("enforcement_operation_id: not_assigned"));
        assert!(payload.contains("permission_denied"));
        assert!(payload.contains("PositiveConfirmationModal"));
        assert!(payload.contains("queue id required"));
        assert!(payload.contains("no queue cancel request"));
        assert!(payload.contains("source hash"));
        assert!(payload.contains("Audit redaction"));
        assert!(payload.contains("No extra MatrixRequest::ReportContent"));
        assert!(payload.contains("gateway/runtime/auth/provider"));
        assert!(payload.contains("Telegram delivery"));
        assert!(payload.contains("live mutation"));
        assert!(
            MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("blocked moderation workflow result taxonomy packet")
        );
    }

    #[test]
    fn message_report_workflow_result_taxonomy_packet_label_reports_copy_state() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label = message_report_workflow_result_taxonomy_packet_label(
            "failed",
            Some(&event_id),
            true,
            false,
        );

        assert!(label.contains("Report workflow Taxonomy copied"));
        assert!(label.contains("status failed"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("loaded source unavailable"));
        assert!(label.contains(MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(label.contains("policy lookup"));
        assert!(label.contains("reviewer assignment"));
        assert!(label.contains("appeal/enforcement workflow"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_report_preflight_detail_label_summarizes_cached_status() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label = message_report_preflight_detail_label(
            Some("Error"),
            "failed",
            Some(&event_id),
            "spam reason",
            Some("permission denied"),
            true,
            "Report status metadata cached",
        );

        assert!(label.contains("Error selected"));
        assert!(label.contains("status failed"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("reason 11 chars"));
        assert!(label.contains("permission denied"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("status metadata source"));
        assert!(label.contains(MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(label.contains("No extra MatrixRequest::ReportContent"));
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains("moderation policy lookup"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_report_loaded_source_modal_label_uses_loaded_event_json_only() {
        let event_id = EventId::parse("$reported:example.org").unwrap();

        let label = message_report_loaded_source_modal_label(
            Some(&event_id),
            "sent",
            "spam reason",
            Some(8),
            Some("{\"type\":\"m.room.message\"}\n{\"body\":\"bad\"}"),
            false,
        );

        assert!(label.contains("Report Source modal handoff"));
        assert!(label.contains("status sent"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("reason 11 chars"));
        assert!(label.contains("loaded row 8"));
        assert!(label.contains("loaded latest_json"));
        assert!(label.contains("2 line"));
        assert!(label.contains(MESSAGE_REPORT_LOADED_SOURCE_MODAL_LABEL));
        assert!(label.contains("FetchEventSource uses only current-room event source JSON"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("extra ReportContent"));
        assert!(label.contains("moderation policy lookup"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("write-side live mutation"));
    }

    #[test]
    fn message_report_loaded_source_modal_label_reports_source_fetch_request() {
        let event_id = EventId::parse("$reported:example.org").unwrap();
        let label = message_report_loaded_source_modal_label(
            Some(&event_id),
            "failed",
            "abuse",
            None,
            None,
            true,
        );

        assert!(label.contains("Report Source modal handoff"));
        assert!(label.contains("status failed"));
        assert!(label.contains("$reported:example.org"));
        assert!(label.contains("loaded row unavailable"));
        assert!(label.contains("source-only MatrixRequest::FetchEventSource requested"));
        assert!(label.contains("current-room event source JSON"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("write-side live mutation"));
    }

    #[test]
    fn message_report_loaded_source_modal_label_reports_missing_loaded_source() {
        let label = message_report_loaded_source_modal_label(None, "", "   ", None, None, false);

        assert!(label.contains("status waiting"));
        assert!(label.contains("event unavailable"));
        assert!(label.contains("reason 0 chars"));
        assert!(label.contains("loaded row unavailable"));
        assert!(label.contains("latest_json unavailable"));
    }

    #[test]
    fn message_report_preflight_detail_label_reports_waiting_state() {
        let label = message_report_preflight_detail_label(None, "", None, "   ", None, false, "");

        assert!(label.contains("no preflight detail selected"));
        assert!(label.contains("status waiting"));
        assert!(label.contains("event unavailable"));
        assert!(label.contains("reason 0 chars"));
        assert!(label.contains("error cache empty"));
        assert!(label.contains("retry cache empty"));
        assert!(label.contains("status metadata source empty"));
    }

    #[test]
    fn message_report_preflight_detail_evidence_names_local_boundaries() {
        assert!(
            MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("Request, Result, Error, Retry, and Source")
        );
        assert!(
            MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("Source is a real loaded-or-source-fetch modal handoff")
        );
        assert!(MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("EventSourceModal"));
        assert!(
            MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("MatrixRequest::ReportContent")
        );
        assert!(MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE.contains("cached reported event id"));
        assert!(
            MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE
                .contains("real loaded-or-source-fetch modal handoff")
        );
        assert!(
            MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE.contains("EventTimelineItem.latest_json")
        );
        assert!(
            MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE.contains("MatrixRequest::FetchEventSource")
        );
        assert!(MESSAGE_REPORT_LOADED_SOURCE_MODAL_LABEL.contains("source-only current-room JSON"));
        assert!(
            MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("PositiveConfirmationModal")
        );
        assert!(MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("cancel queue"));
        assert!(
            MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("moderation policy lookup")
        );
        assert!(MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_LABEL.contains("stay local"));
    }
}
