impl RoomScreenRef {
    /// See [`RoomScreen::set_displayed_room()`].
    pub fn set_displayed_room(
        &self,
        cx: &mut Cx,
        room_name_id: &RoomNameId,
        thread_root_event_id: Option<OwnedEventId>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.set_displayed_room(cx, room_name_id, thread_root_event_id);
    }

    pub fn hide_displayed_room(&self, cx: &mut Cx) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.hide_displayed_room(cx);
    }
}

/// Immutable RoomScreen states passed via Scope props
/// from a RoomScreen widget to its child widgets for event/draw handlers.
pub struct RoomScreenProps {
    pub room_screen_widget_uid: WidgetUid,
    pub room_name_id: RoomNameId,
    pub timeline_kind: TimelineKind,
    pub room_members: Option<Arc<Vec<RoomMember>>>,
    pub room_avatar_url: Option<OwnedMxcUri>,
}

#[derive(Clone, Debug, Default)]
pub struct MessageSearchServerHit {
    pub event_id: Option<String>,
    pub sender: Option<String>,
    pub origin_server_ts: Option<String>,
    pub body: String,
    pub source_json: Option<String>,
    pub rank: Option<f64>,
    pub context_before_count: usize,
    pub context_after_count: usize,
    pub context_before_previews: Vec<String>,
    pub context_after_previews: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MessageSearchServerFilter {
    pub sender: Option<String>,
    pub media_only: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum MessageSearchLoadedScope {
    #[default]
    AllLoaded,
    LatestLoadedDay,
    PinnedLoaded,
}

#[derive(Clone, Debug, Default)]
pub struct MessageSearchServerResponse {
    pub query: String,
    pub room_id: String,
    pub filter: MessageSearchServerFilter,
    pub count: Option<String>,
    pub next_batch: Option<String>,
    pub highlights: Vec<String>,
    pub hits: Vec<MessageSearchServerHit>,
}

/// Actions for the room screen's tooltip.
#[derive(Clone, Debug, Default)]
pub enum RoomScreenTooltipActions {
    /// Mouse over event when the mouse is over the read receipt.
    HoverInReadReceipt {
        /// The rect of the moused over widget
        widget_rect: Rect,
        /// Includes the list of users who have seen this event
        read_receipts: indexmap::IndexMap<matrix_sdk::ruma::OwnedUserId, Receipt>,
    },
    /// Mouse over event when the mouse is over the reaction button.
    HoverInReactionButton {
        /// The rectangle (bounds) of the hovered-over widget.
        widget_rect: Rect,
        /// Includes the list of users who have reacted to the emoji.
        reaction_data: ReactionData,
    },
    /// Mouse out event and clear tooltip.
    HoverOut,
    #[default]
    None,
}

/// The narrow set of room settings state fields currently wired to live Matrix writes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoomSettingsMutationField {
    Name,
    Topic,
    Avatar,
    CanonicalAlias,
    HistoryVisibility,
    JoinRule,
    Tombstone,
}

impl RoomSettingsMutationField {
    fn label(self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::Topic => "Topic",
            Self::Avatar => "Avatar",
            Self::CanonicalAlias => "Canonical alias",
            Self::HistoryVisibility => "History visibility",
            Self::JoinRule => "Join rule",
            Self::Tombstone => "Tombstone",
        }
    }

    fn matrix_event_type(self) -> &'static str {
        match self {
            Self::Name => "m.room.name",
            Self::Topic => "m.room.topic",
            Self::Avatar => "m.room.avatar",
            Self::CanonicalAlias => "m.room.canonical_alias",
            Self::HistoryVisibility => "m.room.history_visibility",
            Self::JoinRule => "m.room.join_rules",
            Self::Tombstone => "m.room.tombstone",
        }
    }
}

#[allow(dead_code)]
enum RoomAvatarUploadPickResult {
    Picked(PathBuf),
    Canceled,
    Unsupported,
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn pick_room_avatar_upload_file() -> RoomAvatarUploadPickResult {
    rfd::FileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp", "webp"])
        .pick_file()
        .map(RoomAvatarUploadPickResult::Picked)
        .unwrap_or(RoomAvatarUploadPickResult::Canceled)
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn pick_room_avatar_upload_file() -> RoomAvatarUploadPickResult {
    RoomAvatarUploadPickResult::Unsupported
}

fn room_avatar_upload_mime_type(path: &Path) -> mime::Mime {
    mime_guess::from_path(path).first_or_octet_stream()
}

fn validate_room_avatar_upload_file(
    path: &Path,
    mime_type: &mime::Mime,
) -> Result<(), &'static str> {
    let metadata = fs::metadata(path).map_err(|_| "selected image is unreadable")?;
    if !metadata.is_file() {
        return Err("selected path is not a regular file");
    }
    if metadata.len() == 0 {
        return Err("selected image is empty");
    }
    if mime_type.type_() != mime::IMAGE {
        return Err("selected file is not an image");
    }
    Ok(())
}

fn room_avatar_upload_selection_summary(path: &Path, mime_type: &mime::Mime) -> String {
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("selected image");
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .filter(|extension| !extension.trim().is_empty())
        .unwrap_or_else(|| "no extension".to_string());
    let size_label = fs::metadata(path)
        .map(|metadata| ByteSize::b(metadata.len()).to_string())
        .unwrap_or_else(|_| "size unavailable".to_string());
    format!("{filename} · {} · {size_label} · {extension}", mime_type)
}

fn room_settings_avatar_upload_value(value: &str) -> bool {
    value.starts_with("upload avatar:")
}

fn room_avatar_upload_lifecycle_label(
    room_label: &str,
    lifecycle_state: &str,
    selected_summary: Option<&str>,
) -> String {
    let selected_state = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .map(|summary| format!(" Selected image: {summary}."))
        .unwrap_or_default();
    format!(
        "Room Avatar upload {lifecycle_state} for {room_label}.{selected_state} {ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL} No power-level, membership, gateway/runtime/auth, Telegram delivery, or unrelated live mutation was requested."
    )
}

/// A message that is sent from a background async task to a room's timeline view
/// for the purpose of update the Timeline UI contents or metadata.
pub enum TimelineUpdate {
    /// The very first update a given room's timeline receives.
    FirstUpdate {
        /// The initial list of timeline items (events) for a room.
        initial_items: Vector<Arc<TimelineItem>>,
    },
    /// The content of a room's timeline was updated in the background.
    NewItems {
        /// The entire list of timeline items (events) for a room.
        new_items: Vector<Arc<TimelineItem>>,
        /// The range of indices in the `items` list that have been changed in this update
        /// and thus must be removed from any caches of drawn items in the timeline.
        /// Any items outside of this range are assumed to be unchanged and need not be redrawn.
        changed_indices: Range<usize>,
        /// An optimization that informs the UI whether the changes to the timeline
        /// resulted in new items being *appended to the end* of the timeline.
        is_append: bool,
        /// Whether to clear the entire cache of drawn items in the timeline.
        /// This supersedes `index_of_first_change` and is used when the entire timeline is being redrawn.
        clear_cache: bool,
    },
    /// The updated number of unread messages in the room.
    NewUnreadMessagesCount(UnreadMessageCount),
    /// The target event ID was found at the given `index` in the timeline items vector.
    ///
    /// This means that the RoomScreen widget can scroll the timeline up to this event,
    /// and the background `timeline_subscriber_handler` async task can stop looking for this event.
    TargetEventFound {
        target_event_id: OwnedEventId,
        index: usize,
    },
    /// A notice that the background task doing pagination for this room is currently running
    /// a pagination request in the given direction, and is waiting for that request to complete.
    PaginationRunning(PaginationDirection),
    /// An error occurred while paginating the timeline for this room.
    PaginationError {
        error: timeline::Error,
        direction: PaginationDirection,
    },
    /// A notice that the background task doing pagination for this room has become idle,
    /// meaning that it has completed its recent pagination request(s).
    PaginationIdle {
        /// If `true`, the start of the timeline has been reached, meaning that
        /// there is no need to send further pagination requests.
        fully_paginated: bool,
        direction: PaginationDirection,
    },
    /// A notice that event details have been fetched from the server,
    /// including a `result` that indicates whether the request was successful.
    EventDetailsFetched {
        event_id: OwnedEventId,
        result: Result<(), matrix_sdk_ui::timeline::Error>,
    },
    /// A notice that fresh thread-summary details were fetched for a thread root.
    ThreadSummaryDetailsFetched {
        thread_root_event_id: OwnedEventId,
        timeline_item_index: usize,
        num_replies: u32,
        latest_reply_preview_text: Option<String>,
    },
    /// A compact summary of Matrix m.replace relations for an edited event.
    EditHistoryFetched {
        event_id: OwnedEventId,
        result: Result<EditHistorySummary, String>,
    },
    /// Raw JSON source fetched for a specific event in this room.
    EventSourceFetched {
        event_id: OwnedEventId,
        result: Result<String, String>,
    },
    /// The result of a request to edit a message in this timeline.
    MessageEdited {
        timeline_event_item_id: TimelineEventItemId,
        result: Result<(), matrix_sdk_ui::timeline::Error>,
    },
    /// A notice that the room's members have been fetched from the server,
    /// though the success or failure of the request is not yet known until the client
    /// requests the member info via a timeline event's `sender_profile()` method.
    RoomMembersSynced,
    /// A notice that the room's full member list has been fetched from the server,
    /// includes a complete list of room members that can be shared across components.
    /// This is different from RoomMembersSynced which only indicates members were fetched
    /// but doesn't provide the actual data.
    RoomMembersListFetched { members: Vec<RoomMember> },
    /// A notice with an option of Media Request Parameters that one or more requested media items (images, videos, etc.)
    /// that should be displayed in this timeline have now been fetched and are available.
    MediaFetched(MediaRequestParameters),
    /// The result of saving a plain Matrix media item to a user-selected local destination.
    MediaSaveResult {
        source_key: String,
        destination_path: PathBuf,
        result: Result<(), String>,
    },
    /// A notice that one or more members of a this room are currently typing.
    TypingUsers {
        /// The list of users (their displayable name) who are currently typing in this room.
        users: Vec<String>,
    },
    /// The result of a pin/unpin request ([`MatrixRequest::PinEvent`]).
    PinResult {
        event_id: OwnedEventId,
        result: Result<bool, matrix_sdk::Error>,
        pin: bool,
    },
    /// The result of a confirmed Matrix report_content request.
    MessageReportResult {
        event_id: OwnedEventId,
        result: Result<(), String>,
    },
    /// The result of a compact Matrix link room-preview request.
    MatrixLinkPreviewResult {
        room_or_alias_id: OwnedRoomOrAliasId,
        via: Vec<OwnedServerName>,
        event_id: Option<OwnedEventId>,
        event_source_room_id: Option<OwnedRoomId>,
        event_source_json: Option<String>,
        result: Result<String, String>,
    },
    /// The result of a live Matrix server-side message search request.
    MessageSearchServerResult {
        result: Result<MessageSearchServerResponse, String>,
    },
    /// The worker result from handing an attachment to the SDK send queue.
    AttachmentSendResult {
        filename: String,
        result: Result<(), String>,
    },
    /// The worker result from canceling a timeline local echo send queue item.
    LocalSendAbortResult { result: Result<bool, String> },
    /// An update containing the set of pinned events in this room.
    PinnedEvents(Vec<OwnedEventId>),
    /// An update containing the currently logged-in user's power levels for this room.
    UserPowerLevels(UserPowerLevels),
    /// An update containing the effective Matrix notification mode for this room.
    RoomNotificationMode(Option<RoomNotificationMode>),
    /// The result of a confirmed Matrix room notification mode write.
    RoomNotificationModeSet {
        mode: RoomNotificationMode,
        result: Result<(), String>,
    },
    /// Enabled Matrix notification keyword-rule state read from the SDK.
    NotificationKeywordRulesFetched(NotificationKeywordRulesSummary),
    /// The result of a confirmed Matrix notification keyword add/remove write.
    NotificationKeywordRulesMutated {
        keyword: String,
        mutation: NotificationKeywordMutation,
        result: Result<(), String>,
    },
    /// Read-only Matrix pusher/device capability state read from the SDK.
    NotificationPusherStatusFetched(NotificationPusherStatusSummary),
    /// The Matrix default notification mode for this room's current room class.
    NotificationDefaultRoomModeFetched(Result<NotificationDefaultRoomModeSummary, String>),
    /// The result of a confirmed Matrix default room-mode write for this room's class.
    NotificationDefaultRoomModeMutated {
        mode: RoomNotificationMode,
        result: Result<NotificationDefaultRoomModeSummary, String>,
    },
    /// The result of a confirmed Matrix room settings name/topic state write.
    RoomSettingsMutationResult {
        field: RoomSettingsMutationField,
        value: String,
        result: Result<(), String>,
    },
    /// An update to the currently logged-in user's own read receipt for this room.
    OwnUserReadReceipt(Receipt),
    /// A notice that the given room has been tombstoned (closed)
    /// and replaced by the given successor room.
    Tombstoned(SuccessorRoomDetails),
    /// A notice that link preview data for a URL has been fetched and is now available.
    LinkPreviewFetched,
    /// A file upload has been accepted by the background worker for this timeline.
    FileUploadStarted {
        upload_id: FileUploadAttemptId,
        file_name: String,
        in_reply_to: Option<OwnedEventId>,
        abort_handle: futures_util::future::AbortHandle,
    },
    /// Progress update for a specific file-upload attempt.
    FileUploadUpdate {
        upload_id: FileUploadAttemptId,
        current: u64,
        total: u64,
    },
    /// A specific file-upload attempt failed.
    FileUploadError {
        upload_id: FileUploadAttemptId,
        error: String,
        upload: AttachmentUpload,
        retryable: bool,
    },
    /// A specific file-upload attempt completed successfully.
    FileUploadComplete { upload_id: FileUploadAttemptId },
    /// An attachment download finished and should briefly expose its result state.
    AttachmentDownloadFinished(OwnedMxcUri, Result<(), String>),
    /// Clear a completed or cancelled attachment download from local UI state.
    AttachmentDownloadReset(OwnedMxcUri),
}

thread_local! {
    /// The global set of all timeline states, one entry per room.
    ///
    /// This is only useful when accessed from the main UI thread.
    static TIMELINE_STATES: RefCell<HashMap<TimelineKind, TimelineUiState>> =
        RefCell::new(HashMap::new());
}

/// The UI-side state of a single room's timeline, which is only accessed/updated by the UI thread.
///
/// This struct should only include states that need to be persisted for a given room
/// across multiple `Hide`/`Show` cycles of that room's timeline within a RoomScreen.
/// If a state is more temporary and shouldn't be persisted when the timeline is hidden,
/// then it should be stored in the RoomScreen widget itself, not in this struct.
struct TimelineUiState {
    /// Info determining whether this is a main room timeline is a thread-focused timeline.
    kind: TimelineKind,

    /// The power levels of the currently logged-in user in this room.
    user_power: UserPowerLevels,

    /// The effective Matrix notification mode for this room, if it has been read.
    room_notification_mode: Option<RoomNotificationMode>,

    /// The list of room members for this room.
    room_members: Option<Arc<Vec<RoomMember>>>,

    /// Whether this room's timeline has been fully paginated, which means
    /// that the oldest (first) event in the timeline is locally synced and available.
    /// When `true`, further backwards pagination requests will not be sent.
    ///
    /// This must be reset to `false` whenever the timeline is fully cleared.
    fully_paginated: bool,

    /// The list of items (events) in this room's timeline that our client currently knows about.
    items: Vector<Arc<TimelineItem>>,

    /// The range of items (indices in the above `items` list) whose event **contents** have been drawn
    /// since the last update and thus do not need to be re-populated on future draw events.
    ///
    /// This range is partially cleared on each background update (see below) to ensure that
    /// items modified during the update are properly redrawn. Thus, it is a conservative
    /// "cache tracker" that may not include all items that have already been drawn,
    /// but that's okay because big updates that clear out large parts of the rangeset
    /// only occur during back pagination, which is both rare and slow in and of itself.
    /// During typical usage, new events are appended to the end of the timeline,
    /// meaning that the range of already-drawn items doesn't need to be cleared.
    ///
    /// Upon a background update, only item indices greater than or equal to the
    /// `index_of_first_change` are removed from this set.
    content_drawn_since_last_update: RangeSet<usize>,

    /// Same as `content_drawn_since_last_update`, but for the event **profiles** (avatar, username).
    profile_drawn_since_last_update: RangeSet<usize>,

    /// The channel receiver for timeline updates for this room.
    ///
    /// Here we use a synchronous (non-async) channel because the receiver runs
    /// in a sync context and the sender runs in an async context,
    /// which is okay because a sender on an unbounded channel never needs to block.
    update_receiver: crossbeam_channel::Receiver<TimelineUpdate>,
    /// The channel sender for worker tasks that need to report timeline-scoped UI metadata.
    update_sender: crossbeam_channel::Sender<TimelineUpdate>,

    /// The sender for timeline requests from a RoomScreen showing this room
    /// to the background async task that handles this room's timeline updates.
    request_sender: TimelineRequestSender,

    /// The cache of media items (images, videos, etc.) that appear in this timeline.
    ///
    /// Currently this excludes avatars, as those are shared across multiple rooms.
    media_cache: MediaCache,
    /// Successful plain SaveMedia destinations keyed by MXC URI, used by the recovery Open folder control.
    saved_media_destinations: HashMap<String, PathBuf>,
    /// Attachment downloads that are active or briefly showing a terminal result.
    pending_downloads: Vec<PendingDownload>,

    /// Cache for link preview data indexed by URL to avoid redundant network requests.
    link_preview_cache: LinkPreviewCache,
    /// Cached fetched thread-summary details, keyed by thread-root event ID.
    fetched_thread_summaries: HashMap<OwnedEventId, FetchedThreadSummary>,
    /// Set of thread roots currently being fetched to avoid duplicate in-flight requests.
    pending_thread_summary_fetches: HashSet<OwnedEventId>,

    /// The states relevant to the UI display of this timeline that are saved upon
    /// a `Hide` action and restored upon a `Show` action.
    saved_state: SavedState,

    /// The state of the message highlight animation.
    ///
    /// We need to run the animation once the scrolling, triggered by the click of of a
    /// a reply preview, ends. so we keep a small state for it.
    /// By default, it starts in Off.
    /// Once the scrolling is started, the state becomes Pending.
    /// If the animation was triggered, the state goes back to Off.
    message_highlight_animation_state: MessageHighlightAnimationState,

    /// The index of the timeline item that was most recently scrolled up past it.
    /// This is used to detect when the user has scrolled up past the second visible item (index 1)
    /// upwards to the first visible item (index 0), which is the top of the timeline,
    /// at which point we submit a backwards pagination request to fetch more events.
    last_scrolled_index: usize,

    /// The index of the first item shown in the timeline's PortalList from *before* the last "jump".
    ///
    /// This index is saved before the timeline undergoes any jumps, e.g.,
    /// receiving new items, major scroll changes, or other timeline view jumps.
    prev_first_index: Option<usize>,

    /// Whether the user has scrolled past their latest read marker.
    ///
    /// This is used to determine whether we should send a fully-read receipt
    /// after the user scrolls past their "read marker", i.e., their latest fully-read receipt.
    /// Its value is determined by comparing the fully-read event's timestamp with the
    /// first and last timestamp of displayed events in the timeline.
    /// When scrolling down, if the value is true, we send a fully-read receipt
    /// for the last visible event in the timeline.
    ///
    /// When new message come in, this value is reset to `false`.
    scrolled_past_read_marker: bool,
    latest_own_user_receipt: Option<Receipt>,

    /// If `Some`, this room has been tombstoned and the details of its successor room
    /// are contained within. If `None`, the room has not been tombstoned.
    tombstone_info: Option<SuccessorRoomDetails>,
}

#[derive(Default, Debug)]
enum MessageHighlightAnimationState {
    Pending {
        item_id: usize,
    },
    #[default]
    Off,
}

/// States that are necessary to save in order to maintain a consistent UI display for a timeline.
///
/// These are saved when navigating away from a timeline (upon `Hide`)
/// and restored when navigating back to a timeline (upon `Show`).
#[derive(Default)]
struct SavedState {
    /// The index of the first item in the timeline's PortalList that is currently visible,
    /// and the scroll offset from the top of the list's viewport to the beginning of that item.
    /// If this is `None`, then the timeline has not yet been scrolled by the user
    /// and the portal list will be set to "tail" (track) the bottom of the list.
    first_index_and_scroll: Option<(usize, f64)>,
    /// The state of all UI elements in the `RoomInputBar`.
    room_input_bar_state: RoomInputBarState,
}

/// Returns info about the item in the list of `new_items` that matches the event ID
/// of a visible item in the given `curr_items` list.
///
/// This info includes a tuple of:
/// 1. the index of the item in the current items list,
/// 2. the index of the item in the new items list,
/// 3. the positional "scroll" offset of the corresponding current item in the portal list,
/// 4. the unique event ID of the item.
fn find_new_item_matching_current_item(
    cx: &mut Cx,
    portal_list: &PortalListRef,
    starting_at_curr_idx: usize,
    curr_items: &Vector<Arc<TimelineItem>>,
    new_items: &Vector<Arc<TimelineItem>>,
) -> Option<(usize, usize, f64, OwnedEventId)> {
    let mut curr_item_focus = curr_items.focus();
    let mut idx_curr = starting_at_curr_idx;
    let mut curr_items_with_ids: Vec<(usize, OwnedEventId)> =
        Vec::with_capacity(portal_list.visible_items());

    // Find all items with real event IDs that are currently visible in the portal list.
    // TODO: if this is slow, we could limit it to 3-5 events at the most.
    if curr_items_with_ids.len() <= portal_list.visible_items() {
        while let Some(curr_item) = curr_item_focus.get(idx_curr) {
            if let Some(event_id) = curr_item.as_event().and_then(|ev| ev.event_id()) {
                curr_items_with_ids.push((idx_curr, event_id.to_owned()));
            }
            if curr_items_with_ids.len() >= portal_list.visible_items() {
                break;
            }
            idx_curr += 1;
        }
    }

    // Find a new item that has the same real event ID as any of the current items.
    for (idx_new, new_item) in new_items.iter().enumerate() {
        let Some(event_id) = new_item.as_event().and_then(|ev| ev.event_id()) else {
            continue;
        };
        if let Some((idx_curr, _)) = curr_items_with_ids
            .iter()
            .find(|(_, ev_id)| ev_id == event_id)
        {
            // Not all items in the portal list are guaranteed to have a position offset,
            // some may be zeroed-out, so we need to account for that possibility by only
            // using events that have a real non-zero area
            if let Some(pos_offset) = portal_list.position_of_item(cx, *idx_curr) {
                log!(
                    "Found matching event ID {event_id} at index {idx_new} in new items list, corresponding to current item index {idx_curr} at pos offset {pos_offset}"
                );
                return Some((*idx_curr, idx_new, pos_offset, event_id.to_owned()));
            }
        }
    }

    None
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct ItemDrawnStatus {
    /// Whether the profile info (avatar and displayable username) were drawn for this item.
    profile_drawn: bool,
    /// Whether the content of the item was drawn (e.g., the message text, image, video, sticker, etc).
    content_drawn: bool,
}

#[derive(Clone, Debug)]
struct FetchedThreadSummary {
    num_replies: u32,
    latest_reply_preview_text: Option<String>,
}
impl ItemDrawnStatus {
    /// Returns a new `ItemDrawnStatus` with both `profile_drawn` and `content_drawn` set to `false`.
    const fn new() -> Self {
        Self {
            profile_drawn: false,
            content_drawn: false,
        }
    }
    /// Returns a new `ItemDrawnStatus` with both `profile_drawn` and `content_drawn` set to `true`.
    const fn both_drawn() -> Self {
        Self {
            profile_drawn: true,
            content_drawn: true,
        }
    }
}

fn update_local_echo_send_state_status(
    cx: &mut Cx2d,
    item: &WidgetRef,
    event_tl_item: &EventTimelineItem,
) {
    let status_text = attachment_timeline_send_state_status_text(event_tl_item.send_state());

    let visible = !status_text.is_empty();
    item.view(cx, ids!(content.local_send_status))
        .set_visible(cx, visible);
    if visible {
        item.label(cx, ids!(content.local_send_status.status))
            .set_text(cx, &status_text);
    }
}

fn attachment_timeline_send_state_status_text(send_state: Option<&EventSendState>) -> String {
    match send_state {
        Some(EventSendState::NotSentYet {
            progress: Some(progress),
        }) => {
            let percent = if progress.progress.total > 0 {
                (progress.progress.current.saturating_mul(100) / progress.progress.total).min(100)
            } else {
                0
            };
            format!(
                "Uploading via SDK send queue: {percent}% · {}",
                ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL
            )
        }
        Some(EventSendState::NotSentYet { progress: None }) => format!(
            "Queued in SDK send queue · {}",
            ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL
        ),
        Some(EventSendState::SendingFailed {
            error,
            is_recoverable,
        }) => {
            let recovery = if *is_recoverable {
                "recoverable"
            } else {
                "parked"
            };
            format!(
                "SDK send queue failed ({recovery}): {error} · {}",
                ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL
            )
        }
        Some(EventSendState::Sent { event_id }) => format!(
            "Sent by SDK send queue: {event_id} · {}",
            ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL
        ),
        None => String::new(),
    }
}

#[cfg(test)]
mod attachment_timeline_send_state_tests {
    use matrix_sdk::send_queue::AbstractProgress;
    use matrix_sdk_ui::timeline::MediaUploadProgress;

    use super::*;

    #[test]
    fn attachment_timeline_send_state_reports_sdk_upload_progress() {
        let send_state = EventSendState::NotSentYet {
            progress: Some(MediaUploadProgress {
                index: 0,
                progress: AbstractProgress {
                    current: 42,
                    total: 100,
                },
            }),
        };

        let label = attachment_timeline_send_state_status_text(Some(&send_state));

        assert!(label.contains("Uploading via SDK send queue: 42%"));
        assert!(label.contains(ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL));
        assert!(label.contains("SDK queue progress/error/sent"));
    }

    #[test]
    fn attachment_timeline_send_state_reports_queued_without_progress() {
        let send_state = EventSendState::NotSentYet { progress: None };

        let label = attachment_timeline_send_state_status_text(Some(&send_state));

        assert!(label.contains("Queued in SDK send queue"));
        assert!(label.contains(ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL));
    }
}

/// Creates, populates, and adds a Message liveview widget to the given `PortalList`
/// with the given `item_id`.
///
/// The content of the returned `Message` widget is populated with data from a message
/// or sticker and its containing `EventTimelineItem`.
fn populate_message_view(
    cx: &mut Cx2d,
    list: &mut PortalList,
    item_id: usize,
    timeline_kind: &TimelineKind,
    event_tl_item: &EventTimelineItem,
    msg_like_content: &MsgLikeContent,
    prev_event: Option<&Arc<TimelineItem>>,
    media_cache: &mut MediaCache,
    link_preview_cache: &mut LinkPreviewCache,
    fetched_thread_summaries: &HashMap<OwnedEventId, FetchedThreadSummary>,
    pending_thread_summary_fetches: &mut HashSet<OwnedEventId>,
    user_power_levels: &UserPowerLevels,
    pinned_events: &[OwnedEventId],
    item_drawn_status: ItemDrawnStatus,
    room_screen_widget_uid: WidgetUid,
) -> (WidgetRef, ItemDrawnStatus) {
    let mut new_drawn_status = item_drawn_status;
    let ts_millis = event_tl_item.timestamp();

    let mut is_notice = false; // whether this message is a Notice (automated bot message)
    let mut is_server_notice = false; // whether this message is a Server Notice

    // Determine whether we can use a more compact UI view that hides the user's profile info
    // if the previous message (including stickers) was sent by the same user within 10 minutes.
    let use_compact_view = match prev_event.map(|p| p.kind()) {
        Some(TimelineItemKind::Event(prev_event_tl_item)) => match prev_event_tl_item.content() {
            TimelineItemContent::MsgLike(_msg_like_content) => {
                let prev_msg_sender = prev_event_tl_item.sender();
                prev_msg_sender == event_tl_item.sender()
                    && ts_millis
                        .0
                        .checked_sub(prev_event_tl_item.timestamp().0)
                        .is_some_and(|d| d < uint!(600000)) // 10 mins in millis
            }
            _ => false,
        },
        _ => false,
    };

    let has_html_body: bool;

    // Sometimes we need to call this up-front, so we save the result in this variable
    // to avoid having to call it twice.
    let mut set_username_and_get_avatar_retval = None;
    let mut has_room_mention = false;
    let (item, used_cached_item) = match &msg_like_content.kind {
        MsgLikeKind::Message(msg) => {
            let room_mention_room_id = if msg.mentions().is_some_and(|m| m.room) {
                has_room_mention = true;
                Some(timeline_kind.room_id())
            } else {
                None
            };
            match msg.msgtype() {
                MessageType::Text(TextMessageEventContent {
                    body, formatted, ..
                }) => {
                    has_html_body = formatted
                        .as_ref()
                        .is_some_and(|f| f.format == MessageFormat::Html);
                    let template = if use_compact_view {
                        id!(CondensedMessage)
                    } else {
                        id!(Message)
                    };
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        let mut link_preview_ref =
                            item.link_preview(cx, ids!(content.link_preview_view));
                        new_drawn_status.content_drawn = populate_text_message_content(
                            cx,
                            &html_or_plaintext_ref,
                            body,
                            formatted.as_ref(),
                            room_mention_room_id,
                            Some(&mut link_preview_ref),
                            Some(media_cache),
                            Some(link_preview_cache),
                        );
                        (item, false)
                    }
                }
                // A notice message is just a message sent by an automated bot,
                // so we treat it just like a message but use a different font color.
                MessageType::Notice(NoticeMessageEventContent {
                    body, formatted, ..
                }) => {
                    is_notice = true;
                    has_html_body = formatted
                        .as_ref()
                        .is_some_and(|f| f.format == MessageFormat::Html);
                    let template = if use_compact_view {
                        id!(CondensedMessage)
                    } else {
                        id!(Message)
                    };
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        // Apply gray color to all text styles for notice messages.
                        // This covers both rendering paths in HtmlOrPlaintext: the rich
                        // `html_view.html` widget (used when the message has an HTML body)
                        // and the `plaintext_view.pt_label` (used for plain-text notices).
                        let mut html_widget = html_or_plaintext_ref.html(cx, ids!(html_view.html));
                        script_apply_eval!(cx, html_widget, {
                            font_color: mod.widgets.COLOR_MESSAGE_NOTICE_TEXT,
                            draw_block +: {
                                quote_fg_color: mod.widgets.COLOR_MESSAGE_NOTICE_TEXT,
                            }
                        });
                        let mut pt_label =
                            html_or_plaintext_ref.label(cx, ids!(plaintext_view.pt_label));
                        script_apply_eval!(cx, pt_label, {
                            draw_text +: {
                                color: mod.widgets.COLOR_MESSAGE_NOTICE_TEXT
                            }
                        });
                        let mut link_preview_ref =
                            item.link_preview(cx, ids!(content.link_preview_view));
                        new_drawn_status.content_drawn = populate_text_message_content(
                            cx,
                            &html_or_plaintext_ref,
                            body,
                            formatted.as_ref(),
                            room_mention_room_id,
                            Some(&mut link_preview_ref),
                            Some(media_cache),
                            Some(link_preview_cache),
                        );
                        (item, false)
                    }
                }
                MessageType::ServerNotice(sn) => {
                    is_server_notice = true;
                    has_html_body = false;
                    let (item, existed) = list.item_with_existed(cx, item_id, id!(Message));
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        // Apply red color to all text styles for server notices.
                        let mut html_widget = html_or_plaintext_ref.html(cx, ids!(html_view.html));
                        script_apply_eval!(cx, html_widget, {
                            font_color: mod.widgets.COLOR_FG_DANGER_RED
                            draw_text +: { color: mod.widgets.COLOR_FG_DANGER_RED }
                            draw_block +: {
                                line_color: mod.widgets.COLOR_FG_DANGER_RED
                                quote_fg_color: mod.widgets.COLOR_FG_DANGER_RED
                            }
                        });
                        let formatted = format!(
                            "<b>Server notice:</b> {}\n\n<i>Notice type:</i>: {}{}{}",
                            sn.body,
                            sn.server_notice_type.as_str(),
                            sn.limit_type
                                .as_ref()
                                .map(|l| format!("\n<i>Limit type:</i> {}", l.as_str()))
                                .unwrap_or_default(),
                            sn.admin_contact
                                .as_ref()
                                .map(|c| format!("\n<i>Admin contact:</i> {}", c))
                                .unwrap_or_default(),
                        );
                        let mut link_preview_ref =
                            item.link_preview(cx, ids!(content.link_preview_view));
                        new_drawn_status.content_drawn = populate_text_message_content(
                            cx,
                            &html_or_plaintext_ref,
                            &sn.body,
                            Some(&FormattedBody {
                                format: MessageFormat::Html,
                                body: formatted,
                            }),
                            room_mention_room_id,
                            Some(&mut link_preview_ref),
                            Some(media_cache),
                            Some(link_preview_cache),
                        );
                        (item, false)
                    }
                }
                // An emote is just like a message but is prepended with the user's name
                // to indicate that it's an "action" that the user is performing.
                MessageType::Emote(EmoteMessageEventContent {
                    body, formatted, ..
                }) => {
                    has_html_body = formatted
                        .as_ref()
                        .is_some_and(|f| f.format == MessageFormat::Html);
                    let template = if use_compact_view {
                        id!(CondensedMessage)
                    } else {
                        id!(Message)
                    };
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        // Draw the profile up front here because we need the username for the emote body.
                        let (username, profile_drawn) = item
                            .avatar(cx, ids!(profile.avatar))
                            .set_avatar_and_get_username(
                                cx,
                                timeline_kind,
                                event_tl_item.sender(),
                                Some(event_tl_item.sender_profile()),
                                event_tl_item.event_id(),
                                true,
                            );

                        // Prepend a "* <username> " to the emote body, as suggested by the Matrix spec.
                        let (body, formatted) = if let Some(fb) = formatted.as_ref() {
                            (
                                Cow::from(&fb.body),
                                Some(FormattedBody {
                                    format: fb.format.clone(),
                                    body: format!("* {} {}", &username, &fb.body),
                                }),
                            )
                        } else {
                            (Cow::from(format!("* {} {}", &username, body)), None)
                        };
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        let mut link_preview_ref =
                            item.link_preview(cx, ids!(content.link_preview_view));
                        let link_previews_drawn = populate_text_message_content(
                            cx,
                            &html_or_plaintext_ref,
                            &body,
                            formatted.as_ref(),
                            room_mention_room_id,
                            Some(&mut link_preview_ref),
                            Some(media_cache),
                            Some(link_preview_cache),
                        );
                        set_username_and_get_avatar_retval = Some((username, profile_drawn));
                        new_drawn_status.content_drawn = link_previews_drawn;
                        (item, false)
                    }
                }
                MessageType::Image(image) => {
                    has_html_body = image
                        .formatted
                        .as_ref()
                        .is_some_and(|f| f.format == MessageFormat::Html);
                    let template = if use_compact_view {
                        id!(CondensedImageMessage)
                    } else {
                        id!(ImageMessage)
                    };
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        let image_info = image.info.as_deref();
                        let text_or_image_ref = item.text_or_image(cx, ids!(content.message));
                        let is_image_fully_drawn = populate_image_message_content(
                            cx,
                            &text_or_image_ref,
                            image_info,
                            image.source.clone(),
                            msg.body(),
                            media_cache,
                        );
                        new_drawn_status.content_drawn = is_image_fully_drawn;
                        (item, false)
                    }
                }
                MessageType::Location(location) => {
                    has_html_body = false;
                    let template = if use_compact_view {
                        id!(CondensedMessage)
                    } else {
                        id!(Message)
                    };
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        let is_location_fully_drawn =
                            populate_location_message_content(cx, &html_or_plaintext_ref, location);
                        new_drawn_status.content_drawn = is_location_fully_drawn;
                        (item, false)
                    }
                }
                MessageType::File(file_content) => {
                    has_html_body = file_content
                        .formatted
                        .as_ref()
                        .is_some_and(|f| f.format == MessageFormat::Html);
                    let template = if use_compact_view {
                        id!(CondensedMessage)
                    } else {
                        id!(Message)
                    };
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        new_drawn_status.content_drawn =
                            populate_file_message_content(cx, &html_or_plaintext_ref, file_content);
                        (item, false)
                    }
                }
                MessageType::Audio(audio) => {
                    has_html_body = audio
                        .formatted
                        .as_ref()
                        .is_some_and(|f| f.format == MessageFormat::Html);
                    let template = if use_compact_view {
                        id!(CondensedMessage)
                    } else {
                        id!(Message)
                    };
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        new_drawn_status.content_drawn =
                            populate_audio_message_content(cx, &html_or_plaintext_ref, audio);
                        (item, false)
                    }
                }
                MessageType::Video(video) => {
                    has_html_body = video
                        .formatted
                        .as_ref()
                        .is_some_and(|f| f.format == MessageFormat::Html);
                    let template = if use_compact_view {
                        id!(CondensedMessage)
                    } else {
                        id!(Message)
                    };
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        new_drawn_status.content_drawn =
                            populate_video_message_content(cx, &html_or_plaintext_ref, video);
                        (item, false)
                    }
                }
                MessageType::VerificationRequest(verification) => {
                    has_html_body = verification
                        .formatted
                        .as_ref()
                        .is_some_and(|f| f.format == MessageFormat::Html);
                    let template = id!(Message);
                    let (item, existed) = list.item_with_existed(cx, item_id, template);
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        // Use `FormattedBody` to hold our custom summary of this verification request.
                        let formatted = FormattedBody {
                            format: MessageFormat::Html,
                            body: format!(
                                "<i>Sent a <b>verification request</b> to {}.<br>(Supported methods: {})</i>",
                                verification.to,
                                verification
                                    .methods
                                    .iter()
                                    .map(|m| m.as_str())
                                    .collect::<Vec<_>>()
                                    .join(", "),
                            ),
                        };
                        let html_or_plaintext_ref =
                            item.html_or_plaintext(cx, ids!(content.message));
                        let mut link_preview_ref =
                            item.link_preview(cx, ids!(content.link_preview_view));

                        new_drawn_status.content_drawn = populate_text_message_content(
                            cx,
                            &html_or_plaintext_ref,
                            &verification.body,
                            Some(&formatted),
                            room_mention_room_id,
                            Some(&mut link_preview_ref),
                            Some(media_cache),
                            Some(link_preview_cache),
                        );
                        (item, false)
                    }
                }
                _ => {
                    has_html_body = false;
                    let (item, existed) = list.item_with_existed(cx, item_id, id!(Message));
                    if existed && item_drawn_status.content_drawn {
                        (item, true)
                    } else {
                        item.label(cx, ids!(content.message))
                            .set_text(cx, &format!("[Unsupported {:?}]", msg_like_content.kind));
                        new_drawn_status.content_drawn = true;
                        (item, false)
                    }
                }
            }
        }
        // Handle sticker messages that are static images.
        MsgLikeKind::Sticker(sticker) => {
            has_html_body = false;
            let StickerEventContent {
                body, info, source, ..
            } = sticker.content();

            let template = if use_compact_view {
                id!(CondensedImageMessage)
            } else {
                id!(ImageMessage)
            };
            let (item, existed) = list.item_with_existed(cx, item_id, template);

            if existed && item_drawn_status.content_drawn {
                (item, true)
            } else {
                if let StickerMediaSource::Plain(owned_mxc_url) = source {
                    let image_info = info;
                    let text_or_image_ref = item.text_or_image(cx, ids!(content.message));
                    let is_image_fully_drawn = populate_image_message_content(
                        cx,
                        &text_or_image_ref,
                        Some(image_info),
                        MediaSource::Plain(owned_mxc_url.clone()),
                        body,
                        media_cache,
                    );
                    new_drawn_status.content_drawn = is_image_fully_drawn;
                    (item, false)
                } else {
                    (item, true)
                }
            }
        }
        // Handle messages that have been redacted (deleted).
        MsgLikeKind::Poll(poll_state) => {
            has_html_body = true;
            let template = if use_compact_view {
                id!(CondensedMessage)
            } else {
                id!(Message)
            };
            let (item, existed) = list.item_with_existed(cx, item_id, template);
            if existed && item_drawn_status.content_drawn {
                (item, true)
            } else {
                let html_or_plaintext_ref = item.html_or_plaintext(cx, ids!(content.message));
                new_drawn_status.content_drawn =
                    populate_poll_message_content(cx, &html_or_plaintext_ref, poll_state);
                (item, false)
            }
        }
        MsgLikeKind::Redacted => {
            has_html_body = false;
            let template = if use_compact_view {
                id!(CondensedMessage)
            } else {
                id!(Message)
            };
            let (item, existed) = list.item_with_existed(cx, item_id, template);
            if existed && item_drawn_status.content_drawn {
                (item, true)
            } else {
                let html_or_plaintext_ref = item.html_or_plaintext(cx, ids!(content.message));
                // Apply a smaller font size for redacted messages.
                let mut html_widget = html_or_plaintext_ref.html(cx, ids!(html_view.html));
                script_apply_eval!(cx, html_widget, {
                    font_size: mod.widgets.REDACTED_MESSAGE_FONT_SIZE
                    text_style_normal +: { font_size: mod.widgets.REDACTED_MESSAGE_FONT_SIZE }
                    text_style_italic +: { font_size: mod.widgets.REDACTED_MESSAGE_FONT_SIZE }
                    text_style_bold +: { font_size: mod.widgets.REDACTED_MESSAGE_FONT_SIZE }
                    text_style_bold_italic +: { font_size: mod.widgets.REDACTED_MESSAGE_FONT_SIZE }
                    text_style_fixed +: { font_size: mod.widgets.REDACTED_MESSAGE_FONT_SIZE }
                });
                new_drawn_status.content_drawn = populate_redacted_message_content(
                    cx,
                    &html_or_plaintext_ref,
                    event_tl_item,
                    timeline_kind.room_id(),
                );
                (item, false)
            }
        }
        other => {
            has_html_body = false;
            let (item, existed) = list.item_with_existed(cx, item_id, id!(Message));
            if existed && item_drawn_status.content_drawn {
                (item, true)
            } else {
                item.label(cx, ids!(content.message))
                    .set_text(cx, &format!("[Unsupported {:?}] ", other));
                new_drawn_status.content_drawn = true;
                (item, false)
            }
        }
    };

    let timeline_event_id = event_tl_item.identifier();

    // If we didn't use a cached item, we need to draw all other message content:
    // the reactions, the read receipts avatar row, the reply preview.
    if !used_cached_item {
        let reactions = (!matches!(msg_like_content.kind, MsgLikeKind::Redacted))
            .then(|| event_tl_item.content().reactions())
            .flatten();
        item.reaction_list(cx, ids!(content.reaction_list)).set_list(
            cx,
            reactions,
            timeline_kind,
            &timeline_event_id,
            item_id,
        );
        populate_read_receipts(&item, cx, timeline_kind, event_tl_item);
        let is_reply_fully_drawn = draw_replied_to_message(
            cx,
            &item.view(cx, ids!(replied_to_message)),
            timeline_kind,
            msg_like_content.in_reply_to.as_ref(),
            event_tl_item.event_id(),
        );
        let is_thread_summary_fully_drawn = populate_thread_root_summary(
            cx,
            &item,
            item_id,
            timeline_kind,
            msg_like_content,
            event_tl_item,
            fetched_thread_summaries,
            pending_thread_summary_fetches,
        );

        // The content is only considered to be fully drawn if the logic above marked it as such
        // *and* if the reply preview was also fully drawn
        // *and* if the thread root summary (if applicable) was also fully drawn.
        new_drawn_status.content_drawn &= is_reply_fully_drawn;
        new_drawn_status.content_drawn &= is_thread_summary_fully_drawn;
    }

    let thread_root_event_id = msg_like_content.thread_root.clone().or_else(|| {
        msg_like_content
            .thread_summary
            .as_ref()
            .and_then(|_| event_tl_item.event_id().map(|id| id.to_owned()))
    });
    let related_event_id = msg_like_content
        .in_reply_to
        .as_ref()
        .map(|r| r.event_id.clone());
    let should_be_highlighted = event_tl_item.is_highlighted() || has_room_mention;
    let local_send_handle = event_tl_item.local_echo_send_handle();
    let loaded_report_target_metadata = MessageReportTargetMetadata::from_loaded_body(
        item_id,
        &plaintext_body_of_timeline_item(event_tl_item),
        event_tl_item.event_id().is_some(),
        related_event_id.is_some(),
        thread_root_event_id.is_some(),
        local_send_handle.is_some(),
        should_be_highlighted,
    );

    // We must always re-set the message details, even when re-using a cached portallist item,
    // because the item type might be the same but for a different message entirely.
    let message_details = MessageDetails {
        thread_root_event_id,
        timeline_event_id,
        item_id,
        related_event_id,
        room_screen_widget_uid,
        abilities: MessageAbilities::from_user_power_and_event(
            user_power_levels,
            event_tl_item,
            msg_like_content,
            pinned_events,
            has_html_body,
            matches!(timeline_kind, TimelineKind::Thread { .. }),
        ),
        should_be_highlighted,
        local_send_handle,
        loaded_report_target_metadata,
    };
    item.as_message().set_data(message_details);
    update_local_echo_send_state_status(cx, &item, event_tl_item);

    // If `used_cached_item` is false, we should always redraw the profile, even if profile_drawn is true.
    let skip_draw_profile =
        use_compact_view || (used_cached_item && item_drawn_status.profile_drawn);
    if skip_draw_profile {
        // log!("\t --> populate_message_view(): SKIPPING profile draw for item_id: {item_id}");
        item.view(cx, ids!(content.sender_profile_read_evidence))
            .set_visible(cx, false);
        new_drawn_status.profile_drawn = true;
    } else {
        // log!("\t --> populate_message_view(): DRAWING  profile draw for item_id: {item_id}");
        let mut username_label = item.label(cx, ids!(content.username));
        let sender_profile_event_details_read =
            matches!(event_tl_item.sender_profile(), TimelineDetails::Unavailable);

        if !is_server_notice {
            // the normal case
            let (username, profile_drawn) =
                set_username_and_get_avatar_retval.unwrap_or_else(|| {
                    item.avatar(cx, ids!(profile.avatar))
                        .set_avatar_and_get_username(
                            cx,
                            timeline_kind,
                            event_tl_item.sender(),
                            Some(event_tl_item.sender_profile()),
                            event_tl_item.event_id(),
                            true,
                        )
                });
            if is_notice {
                script_apply_eval!(cx, username_label, {
                    draw_text +: {
                        color: mod.widgets.COLOR_MESSAGE_NOTICE_TEXT
                    }
                });
            }
            username_label.set_text(cx, &username);
            item.view(cx, ids!(content.sender_profile_read_evidence))
                .set_visible(cx, sender_profile_event_details_read);
            if sender_profile_event_details_read {
                item.label(cx, ids!(content.sender_profile_read_evidence.status))
                    .set_text(
                        cx,
                        "Sender profile uses FetchDetailsForEvent read path; no message, profile, room-state, or membership mutation was sent.",
                    );
            }
            new_drawn_status.profile_drawn = profile_drawn;
        } else {
            // Server notices are drawn with a red color avatar background and username.
            let avatar = item.avatar(cx, ids!(profile.avatar));
            avatar.show_text(cx, Some(COLOR_FG_DANGER_RED), None, "⚠");
            username_label.set_text(cx, "Server notice");
            item.view(cx, ids!(content.sender_profile_read_evidence))
                .set_visible(cx, false);
            script_apply_eval!(cx, username_label, {
                draw_text +: {
                    color: (mod.widgets.COLOR_FG_DANGER_RED)
                }
            });
            new_drawn_status.profile_drawn = true;
        }
    }

    // If we've previously drawn the item content, skip all other steps.
    if used_cached_item && item_drawn_status.content_drawn && item_drawn_status.profile_drawn {
        return (item, new_drawn_status);
    }

    // Set the timestamp.
    if let Some(dt) = unix_time_millis_to_datetime(ts_millis) {
        item.timestamp(cx, ids!(profile.timestamp))
            .set_date_time(cx, dt);
    }

    // Set the "edited" indicator if this message was edited.
    if msg_like_content.as_message().is_some_and(|m| m.is_edited()) {
        item.edited_indicator(cx, ids!(profile.edited_indicator))
            .set_latest_edit(cx, event_tl_item);
    }

    #[cfg(feature = "tsp")]
    {
        use matrix_sdk::ruma::serde::Base64;
        use crate::tsp::{
            self,
            tsp_sign_indicator::{TspSignState, TspSignIndicatorWidgetRefExt},
        };

        if let Some(mut tsp_sig) = event_tl_item
            .latest_json()
            .and_then(|raw| raw.get_field::<serde_json::Value>("content").ok())
            .flatten()
            .and_then(|content_obj| content_obj.get("org.robius.tsp_signature").cloned())
            .and_then(|tsp_sig_value| serde_json::from_value::<Base64>(tsp_sig_value).ok())
            .map(|b64| b64.into_inner())
        {
            log!(
                "Found event {:?} with TSP signature.",
                event_tl_item.event_id()
            );
            let tsp_sign_state = if let Some(sender_vid) = tsp::tsp_state_ref()
                .lock()
                .unwrap()
                .get_verified_vid_for(event_tl_item.sender())
            {
                log!(
                    "Found verified VID for sender {}: \"{}\"",
                    event_tl_item.sender(),
                    sender_vid.identifier()
                );
                tsp_sdk::crypto::verify(&*sender_vid, &mut tsp_sig).map_or(
                    TspSignState::WrongSignature,
                    |(msg, msg_type)| {
                        log!("TSP signature verified successfully!\n    Msg type: {msg_type:?}\n    Message: {:?} ({msg:X?})", std::str::from_utf8(msg));
                        TspSignState::Verified
                    }
                )
            } else {
                TspSignState::Unknown
            };

            log!(
                "TSP signature state for event {:?} is {:?}",
                event_tl_item.event_id(),
                tsp_sign_state
            );
            item.tsp_sign_indicator(cx, ids!(profile.tsp_sign_indicator))
                .show_with_state(cx, tsp_sign_state);
        }
    }

    (item, new_drawn_status)
}

fn populate_poll_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    poll_state: &PollState,
) -> bool {
    // Poll preview read evidence: this function only formats the already-loaded
    // PollState results into the local message row. It intentionally does not
    // submit poll response, edit, redact, timeline reload, room-state, or membership requests.
    let results = poll_state.results();
    let total_votes: usize = results.votes.values().map(Vec::len).sum();
    let status = if results.end_time.is_some() {
        "closed"
    } else {
        "open"
    };
    let edited_note = if results.has_been_edited {
        " / edited"
    } else {
        ""
    };
    let max_selections_text = results.max_selections.to_string();
    let preview_packet = poll_answer_preview_result_packet_label(
        results.answers.len(),
        total_votes,
        &max_selections_text,
        status,
        results.has_been_edited,
    );
    let answer_rows = if results.answers.is_empty() {
        "<br><i>No answer options are available in this loaded timeline state.</i>".to_string()
    } else {
        results
            .answers
            .iter()
            .map(|answer| {
                let votes = results.votes.get(&answer.id).map_or(0, Vec::len);
                let percent = if total_votes == 0 {
                    0
                } else {
                    (votes * 100) / total_votes
                };
                let plural = if votes == 1 { "" } else { "s" };
                format!(
                    "<br>&bull; {} - {} vote{} ({}%)",
                    htmlize::escape_text(&answer.text),
                    votes,
                    plural,
                    percent
                )
            })
            .collect::<String>()
    };
    let fallback_rows = if results.answers.is_empty() {
        "No answer options are available.".to_string()
    } else {
        results
            .answers
            .iter()
            .map(|answer| {
                let votes = results.votes.get(&answer.id).map_or(0, Vec::len);
                format!("{} - {} votes", answer.text, votes)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let fallback = format!(
        "Poll: {}\n{}\n{} total votes; {} poll; max selections {}{}.\n{}",
        results.question,
        fallback_rows,
        total_votes,
        status,
        &max_selections_text,
        edited_note,
        preview_packet
    );
    let formatted = FormattedBody {
        format: MessageFormat::Html,
        body: format!(
            "<b>Poll:</b> {}{}<br><i>{} poll; {} total vote{}; max selections {}</i><br>{}<br><i>{}</i><br><i>{}</i>",
            htmlize::escape_text(&results.question),
            edited_note,
            status,
            total_votes,
            if total_votes == 1 { "" } else { "s" },
            &max_selections_text,
            answer_rows,
            POLL_MESSAGE_PREVIEW_COMPACT_LABEL,
            htmlize::escape_text(&preview_packet),
        ),
    };

    populate_text_message_content(
        cx,
        message_content_widget,
        &fallback,
        Some(&formatted),
        None,
        None,
        None,
        None,
    )
}

fn poll_answer_preview_result_packet_label(
    answer_count: usize,
    total_votes: usize,
    max_selections: &str,
    status: &str,
    has_been_edited: bool,
) -> String {
    let edited_state = if has_been_edited {
        "edited"
    } else {
        "original"
    };
    format!(
        "Poll answer preview/result packet: answer_count {answer_count}; total_votes {total_votes}; max_selections {max_selections}; poll_status {status}; edited_state {edited_state}; answer_edit_slot not_built; vote_response_slot not_sent; result_mapping read_only_loaded_pollstate; stale_poll_policy ignore_without_matching_event_update; unsupported_server_capability_boundary local_disabled; no poll response, poll answer edit, timeline reload, message, room-state, or membership request was sent."
    )
}

#[cfg(test)]
mod poll_answer_preview_result_packet_tests {
    use super::*;

    #[test]
    fn poll_answer_preview_result_packet_keeps_answers_read_only() {
        let label = poll_answer_preview_result_packet_label(3, 11, "2", "open", true);

        assert!(label.contains("Poll answer preview/result packet"));
        assert!(label.contains("answer_count 3"));
        assert!(label.contains("total_votes 11"));
        assert!(label.contains("max_selections 2"));
        assert!(label.contains("poll_status open"));
        assert!(label.contains("edited_state edited"));
        assert!(label.contains("answer_edit_slot not_built"));
        assert!(label.contains("vote_response_slot not_sent"));
        assert!(label.contains("result_mapping read_only_loaded_pollstate"));
        assert!(label.contains("stale_poll_policy"));
        assert!(label.contains("unsupported_server_capability_boundary local_disabled"));
        assert!(label.contains("no poll response, poll answer edit"));
        assert!(
            POLL_ANSWER_PREVIEW_RESULT_PACKET_EVIDENCE.contains("already loaded PollState only")
        );
    }
}

/// Draws the Html or plaintext body of the given Text or Notice message into the `message_content_widget`.
/// Also populates link previews if a link_preview_ref is provided.
///
/// Returns whether the text items were fully drawn.
fn populate_text_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    body: &str,
    formatted_body: Option<&FormattedBody>,
    room_mention_room_id: Option<&OwnedRoomId>,
    link_preview_ref: Option<&mut LinkPreviewRef>,
    media_cache: Option<&mut MediaCache>,
    link_preview_cache: Option<&mut LinkPreviewCache>,
) -> bool {
    /// If this is a room mention, replace `@room` text in `html` with a pill
    /// link to the room so it renders as a red room pill with the room's avatar.
    fn apply_room_mention<'a>(html: Cow<'a, str>, room_id: Option<&OwnedRoomId>) -> Cow<'a, str> {
        if let Some(room_id) = room_id {
            // Only replace @room if it's NOT already inside an <a> tag
            // (some clients pre-link @room in the formatted_body).
            if html.contains("@room") && !html.contains("\">@room</a>") {
                return Cow::Owned(html.replace(
                    "@room",
                    &format!("<a href=\"https://matrix.to/#/{room_id}\">@room</a>"),
                ));
            }
        }
        html
    }

    // The message was HTML-formatted rich text.
    let mut links = Vec::new();
    if let Some(fb) = formatted_body
        .as_ref()
        .and_then(|fb| (fb.format == MessageFormat::Html).then_some(fb))
    {
        let linkified_html = utils::linkify_get_urls(
            utils::trim_start_html_whitespace(&fb.body),
            true,
            Some(&mut links),
        );
        let html = apply_room_mention(linkified_html, room_mention_room_id);
        message_content_widget.show_html(cx, html);
    }
    // The message was non-HTML plaintext.
    else {
        let linkified_html = utils::linkify_get_urls(body, false, Some(&mut links));
        let html = apply_room_mention(linkified_html, room_mention_room_id);
        match html {
            Cow::Owned(linkified_html) => message_content_widget.show_html(cx, &linkified_html),
            Cow::Borrowed(plaintext) => message_content_widget.show_plaintext(cx, plaintext),
        }
    };

    // Populate link previews if all required parameters are provided
    if let (Some(link_preview_ref), Some(media_cache), Some(link_preview_cache)) =
        (link_preview_ref, media_cache, link_preview_cache)
    {
        link_preview_ref.populate_below_message(
            cx,
            &links,
            media_cache,
            link_preview_cache,
            &populate_image_message_content,
        )
    } else {
        true
    }
}

/// Draws the given image message's content into the `message_content_widget`.
///
/// Returns whether the image message content was fully drawn.
fn populate_image_message_content(
    cx: &mut Cx,
    text_or_image_ref: &TextOrImageRef,
    image_info_source: Option<&ImageInfo>,
    original_source: MediaSource,
    body: &str,
    media_cache: &mut MediaCache,
) -> bool {
    // We don't use thumbnails, as their resolution is too low to be visually useful.
    // We also don't trust the provided mimetype, as it can be incorrect.
    let (mimetype, _width, _height) = image_info_source
        .map(|info| (info.mimetype.as_deref(), info.width, info.height))
        .unwrap_or_default();

    // If we have a known mimetype and it's not a static image,
    // then show a message about it being unsupported (e.g., for animated gifs).
    if let Some(mime) = mimetype.as_ref() {
        if ImageFormat::from_mimetype(mime).is_none() {
            text_or_image_ref.show_text(cx, format!("{body}\n\nUnsupported type {mime:?}"));
            return true; // consider this as fully drawn
        }
    }

    let fully_drawn = Cell::new(false);

    // A closure that fetches and shows the image from the given `mxc_uri`,
    // marking it as fully drawn if the image was available.
    let mut fetch_and_show_image_uri =
        |cx: &mut Cx, mxc_uri: OwnedMxcUri, image_info: &ImageInfo| {
            let media_source = MediaSource::Plain(mxc_uri.clone());
            match media_cache
                .try_get_media_or_fetch(&media_source, MEDIA_THUMBNAIL_FORMAT.into())
            {
                (MediaCacheEntry::Loaded(data), _media_format) => {
                    let show_image_result = text_or_image_ref.show_image(
                        cx,
                        Some(media_source),
                        |cx, img| {
                            utils::load_png_or_jpg(&img, cx, &data)
                                .map(|()| img.size_in_pixels(cx).unwrap_or_default())
                        },
                    );
                    if let Err(e) = show_image_result {
                        let err_str = format!("{body}\n\nFailed to display image: {e:?}");
                        error!("{err_str}");
                        text_or_image_ref.show_text(cx, &err_str);
                    }

                    // We're done drawing the image, so mark it as fully drawn.
                    fully_drawn.set(true);
                }
                (MediaCacheEntry::Requested, _media_format) => {
                    // If the image is being fetched, we try to show its blurhash.
                    if let (Some(ref blurhash), Some(width), Some(height)) = (
                        image_info.blurhash.clone(),
                        image_info.width,
                        image_info.height,
                    ) {
                        let show_image_result = text_or_image_ref.show_image(
                            cx,
                            Some(MediaSource::Plain(mxc_uri)),
                            |cx, img| {
                                let (Ok(width), Ok(height)) = (width.try_into(), height.try_into())
                                else {
                                    return Err(image_cache::ImageError::EmptyData);
                                };
                                let (width, height): (u32, u32) = (width, height);
                                if width == 0 || height == 0 {
                                    warning!(
                                        "Image had an invalid aspect ratio (width or height of 0)."
                                    );
                                    return Err(image_cache::ImageError::EmptyData);
                                }
                                let aspect_ratio: f32 = width as f32 / height as f32;
                                // Cap the blurhash to a max size of 500 pixels in each dimension
                                // because the `blurhash::decode()` function can be rather expensive.
                                let (mut capped_width, mut capped_height) = (width, height);
                                if capped_height > BLURHASH_IMAGE_MAX_SIZE {
                                    capped_height = BLURHASH_IMAGE_MAX_SIZE;
                                    capped_width =
                                        (capped_height as f32 * aspect_ratio).floor() as u32;
                                }
                                if capped_width > BLURHASH_IMAGE_MAX_SIZE {
                                    capped_width = BLURHASH_IMAGE_MAX_SIZE;
                                    capped_height =
                                        (capped_width as f32 / aspect_ratio).floor() as u32;
                                }

                                match blurhash::decode(blurhash, capped_width, capped_height, 1.0) {
                                    Ok(data) => ImageBuffer::new(
                                        &data,
                                        capped_width as usize,
                                        capped_height as usize,
                                    )
                                    .map(|img_buff| {
                                        let texture = Some(img_buff.into_new_texture(cx));
                                        img.set_texture(cx, texture);
                                        img.size_in_pixels(cx).unwrap_or_default()
                                    }),
                                    Err(e) => {
                                        error!("Failed to decode blurhash {e:?}");
                                        Err(image_cache::ImageError::EmptyData)
                                    }
                                }
                            },
                        );
                        if let Err(e) = show_image_result {
                            let err_str = format!("{body}\n\nFailed to display image: {e:?}");
                            error!("{err_str}");
                            text_or_image_ref.show_text(cx, &err_str);
                        }
                    } else {
                        text_or_image_ref.show_text(
                            cx,
                            format!("{body}\n\n{MEDIA_IMAGE_FETCH_CACHE_COMPACT_LABEL}"),
                        );
                    }
                    fully_drawn.set(false);
                }
                (MediaCacheEntry::Failed(_status_code), _media_format) => {
                    if text_or_image_ref
                        .view(cx, ids!(default_image_view))
                        .visible()
                    {
                        fully_drawn.set(true);
                        return;
                    }
                    text_or_image_ref.show_text(
                        cx,
                        format!("{body}\n\n{MEDIA_IMAGE_FETCH_FAILED_COMPACT_LABEL}"),
                    );
                    // For now, we consider this as being "complete". In the future, we could support
                    // retrying to fetch thumbnail of the image on a user click/tap.
                    fully_drawn.set(true);
                }
            }
        };

    let mut fetch_and_show_media_source =
        |cx: &mut Cx, media_source: MediaSource, image_info: &ImageInfo| {
            match media_source {
                MediaSource::Encrypted(_encrypted) => {
                    // We consider this as "fully drawn" since we don't yet support encryption.
                    text_or_image_ref.show_text(
                        cx,
                        encrypted_image_local_metadata_preview(body, &image_info),
                    );
                    fully_drawn.set(true);
                }
                MediaSource::Plain(mxc_uri) => fetch_and_show_image_uri(cx, mxc_uri, image_info),
            }
        };

    match image_info_source {
        Some(image_info) => {
            // Use the provided thumbnail URI if it exists; otherwise use the original URI.
            let media_source = image_info
                .thumbnail_source
                .clone()
                .unwrap_or(original_source);
            fetch_and_show_media_source(cx, media_source, image_info);
        }
        None => {
            text_or_image_ref.show_text(cx, "{body}\n\nImage message had no source URL.");
            fully_drawn.set(true);
        }
    }

    fully_drawn.get()
}

#[allow(dead_code)]
enum MediaSavePathPickResult {
    Picked(std::path::PathBuf),
    Canceled,
    Unsupported,
}

#[derive(Clone, Debug, Default)]
struct MediaDownloadActionMetadata {
    kind: String,
    mime_type: Option<String>,
    size_label: Option<String>,
    duration_label: Option<String>,
    dimensions_label: Option<String>,
}

impl MediaDownloadActionMetadata {
    fn new(kind: &str) -> Self {
        Self {
            kind: kind.to_string(),
            ..Default::default()
        }
    }

    fn summary(&self, filename: &str) -> String {
        let kind = if self.kind.trim().is_empty() {
            "Media"
        } else {
            self.kind.trim()
        };
        let mut parts = vec![kind.to_string(), filename.to_string()];
        if let Some(mime_type) = self.mime_type.as_deref().filter(|value| !value.is_empty()) {
            parts.push(mime_type.to_string());
        }
        if let Some(size_label) = self.size_label.as_deref().filter(|value| !value.is_empty()) {
            parts.push(size_label.to_string());
        }
        if let Some(duration_label) = self
            .duration_label
            .as_deref()
            .filter(|value| !value.is_empty())
        {
            parts.push(duration_label.to_string());
        }
        if let Some(dimensions_label) = self
            .dimensions_label
            .as_deref()
            .filter(|value| !value.is_empty())
        {
            parts.push(dimensions_label.to_string());
        }
        parts.join(" · ")
    }
}

fn media_save_dialog_lifecycle_metadata_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    lifecycle_state: &str,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let lifecycle_state = lifecycle_state.trim();
    let lifecycle_state = if lifecycle_state.is_empty() {
        "status updated"
    } else {
        lifecycle_state
    };

    format!(
        "Media {action_label} {lifecycle_state} for {filename}. Loaded metadata: {metadata_summary}. {MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_LABEL}"
    )
}

fn media_save_destination_metadata_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    destination_path: &std::path::Path,
    open_after_save: bool,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let destination = destination_path.display();
    let open_state = if open_after_save {
        "will ask the system opener after SaveMedia writes the file"
    } else {
        "will only save to the selected local path"
    };

    format!(
        "Media {action_label} destination selected for {filename}: {destination}. Loaded metadata: {metadata_summary}. {open_state}. {MEDIA_SAVE_DESTINATION_METADATA_LABEL}"
    )
}

fn media_inline_playback_queue_boundary_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let playback_state = if open_after_save {
        "Play saves first with MatrixRequest::SaveMedia, then asks the system opener to open the saved file"
    } else {
        "Download writes the picked local file only after confirmation"
    };

    format!(
        "Media {action_label} boundary for {filename}. Loaded metadata: {metadata_summary}. {playback_state}; inline audio/video controls, decrypt, codec/transcode, retry/cancel queue controls, attachment send, message mutation, room-state, membership, gateway/runtime/auth, and live mutation remain unwired. {MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_LABEL}"
    )
}

fn media_inline_player_disabled_controls_preview(
    media_kind: &str,
    filename: &str,
    metadata_summary: &str,
) -> String {
    let media_kind = media_kind.trim();
    let media_kind = if media_kind.is_empty() {
        "Media"
    } else {
        media_kind
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };

    format!(
        "<br><i>{media_kind} inline controls disabled for {}: Playhead 00:00, Seek, Queue, Decrypt, and Codec are local boundary controls. Loaded metadata: {}. {MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_LABEL} No FetchMedia, SaveMedia, decrypt, codec/transcode, inline player startup, retry/cancel queue control, gateway/runtime/auth, or live mutation.</i>",
        htmlize::escape_text(filename),
        htmlize::escape_text(metadata_summary),
    )
}

fn media_save_result_status_boundary_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let opener_state = if open_after_save {
        "system opener success or failure is reported in the same popup after SaveMedia writes the file"
    } else {
        "no system opener request is attempted for Download"
    };

    format!(
        "Media {action_label} result boundary for {filename}. Loaded metadata: {metadata_summary}. SaveMedia completion reports saved, download failed, save failed, optional system opener failed states, and a successful plain-MXC destination cache for Open folder and Replay; {opener_state}. Inline audio/video player state, seek controls, retry/cancel queue controls, decrypt retry, codec/transcode fallback, background download list, delivery/read receipts, message mutation, gateway/runtime/auth, and live mutation remain local blocked controls. {MEDIA_SAVE_RESULT_STATUS_BOUNDARY_LABEL}"
    )
}

fn media_save_destination_cache_key(mxc: &str) -> Option<&str> {
    let mxc = mxc.trim();
    (!mxc.is_empty()).then_some(mxc)
}

fn open_media_saved_folder(destination_path: &Path) -> Result<PathBuf, String> {
    let folder_path = destination_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| "saved path has no parent folder".to_string())?;
    let folder_url = url::Url::from_directory_path(folder_path)
        .map_err(|()| format!("could not convert {} to a file URL", folder_path.display()))?;
    robius_open::Uri::new(folder_url.as_str())
        .open()
        .map_err(|error| format!("{error:?}"))?;
    Ok(folder_path.to_path_buf())
}

fn open_media_saved_file(destination_path: &Path) -> Result<(), String> {
    if let Some(reason) = media_cached_saved_file_stale_reason(destination_path) {
        return Err(reason);
    }
    let file_url = url::Url::from_file_path(destination_path).map_err(|()| {
        format!(
            "could not convert {} to a file URL",
            destination_path.display()
        )
    })?;
    robius_open::Uri::new(file_url.as_str())
        .open()
        .map_err(|error| format!("{error:?}"))
}

fn media_cached_saved_file_stale_reason(destination_path: &Path) -> Option<String> {
    match fs::metadata(destination_path) {
        Ok(metadata) if metadata.is_file() => None,
        Ok(_) => Some(format!(
            "cached saved file is not a regular file: {}",
            destination_path.display()
        )),
        Err(error) => Some(format!(
            "cached saved file is stale ({error}): {}",
            destination_path.display()
        )),
    }
}

fn media_cached_saved_file_status_label(destination_path: &Path) -> Result<String, String> {
    let metadata = fs::metadata(destination_path).map_err(|error| {
        format!(
            "Cached saved-file status unavailable ({error}): {}",
            destination_path.display()
        )
    })?;
    if !metadata.is_file() {
        return Err(format!(
            "Cached saved-file status unavailable: not a regular file: {}",
            destination_path.display()
        ));
    }
    let size_label = ByteSize::b(metadata.len()).to_string();
    let readonly_state = if metadata.permissions().readonly() {
        "readonly"
    } else {
        "writable"
    };
    let modified_state = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|duration| format!("modified {}s since epoch", duration.as_secs()))
        .unwrap_or_else(|| "modified time unavailable".to_string());

    Ok(format!(
        "Cached saved-file status: regular file, {size_label}, {readonly_state}, {modified_state}, path {}. {MEDIA_CACHED_SAVED_FILE_STATUS_LABEL} No FetchMedia, no SaveMedia, no inline player startup, no queue retry/resume/cancel, no system opener, gateway/runtime/auth, or live mutation.",
        destination_path.display()
    ))
}

fn media_save_result_cache_update_label(
    source_key: &str,
    destination_path: &Path,
    saved: bool,
) -> String {
    let source_key = source_key.trim();
    let source_key = if source_key.is_empty() {
        "unknown MXC"
    } else {
        source_key
    };
    let state = if saved {
        "cached successful SaveMedia destination"
    } else {
        "cleared cached SaveMedia destination after failure"
    };
    format!(
        "Media save result {state} for {source_key}: {}. Open folder and Replay can use this cached destination after a successful SaveMedia result only; inline playback, seek, decrypt, queue control, gateway/runtime/auth, and live mutation remain blocked.",
        destination_path.display()
    )
}

fn media_replay_result_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    destination_path: &Path,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Play"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    format!(
        "Media Replay opened cached saved file for {action_label} {filename} through the system opener. Loaded metadata: {metadata_summary}. Saved file: {}. No FetchMedia, no SaveMedia, no retry automation, no queue control, no decrypt retry, no codec/transcode, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL}",
        destination_path.display()
    )
}

fn media_replay_failed_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    destination_path: &Path,
    error: &str,
) -> String {
    let base = media_replay_unavailable_label(
        action_label,
        filename,
        metadata_summary,
        &format!(
            "system opener failed for cached saved file {}: {error}",
            destination_path.display()
        ),
    );
    format!("{base} Replay request was attempted and failed; no Matrix request was submitted.")
}

fn media_replay_unavailable_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    reason: &str,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Play"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let reason = reason.trim();
    let reason = if reason.is_empty() {
        "no successful SaveMedia destination is cached for this row yet"
    } else {
        reason
    };
    format!(
        "Media Replay unavailable for {action_label} {filename}: {reason}. Loaded metadata: {metadata_summary}. Replay only uses a cached successful SaveMedia destination for the same plain MXC row, validates the saved file before system opener handoff, and clears stale cached destinations; it sends no FetchMedia, SaveMedia, retry automation, queue control, decrypt retry, codec/transcode, gateway/runtime/auth, or live mutation. {MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL}"
    )
}

fn media_open_folder_result_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    destination_path: &Path,
    folder_path: &Path,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    format!(
        "Media Open folder handed {action_label} {filename} to the system opener using the cached successful SaveMedia destination. Loaded metadata: {metadata_summary}. Saved file: {}. Opened folder: {}. No FetchMedia, no SaveMedia, no cached-file Replay request, no retry automation, no queue control, no decrypt retry, no codec/transcode, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL}",
        destination_path.display(),
        folder_path.display()
    )
}

fn media_open_folder_failed_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    destination_path: &Path,
    error: &str,
) -> String {
    let base = media_open_folder_unavailable_label(
        action_label,
        filename,
        metadata_summary,
        &format!(
            "system opener failed for cached destination {}: {error}",
            destination_path.display()
        ),
    );
    format!("{base} Open folder request was attempted and failed; no Matrix request was submitted.")
}

fn media_open_folder_unavailable_label(
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    reason: &str,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let reason = reason.trim();
    let reason = if reason.is_empty() {
        "no successful SaveMedia destination is cached for this row yet"
    } else {
        reason
    };
    format!(
        "Media Open folder unavailable for {action_label} {filename}: {reason}. Loaded metadata: {metadata_summary}. Open folder only uses a cached successful SaveMedia destination for the same plain MXC row after validating the saved file and clearing stale cached destinations; it sends no FetchMedia, SaveMedia, cached-file Replay request, retry automation, queue control, decrypt retry, codec/transcode, gateway/runtime/auth, or live mutation. {MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL}"
    )
}

fn media_save_result_recovery_control_label(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
) -> String {
    let action = action.trim();
    let action = if action.is_empty() {
        "Open folder"
    } else {
        action
    };
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let opener_state = if open_after_save {
        "requested Play mode remains SaveMedia plus system opener after a picked local path"
    } else {
        "requested Download mode remains SaveMedia to the picked local path only"
    };

    if action == "Open folder" {
        return media_open_folder_unavailable_label(
            action_label,
            filename,
            metadata_summary,
            "no successful SaveMedia destination is cached for this row yet",
        );
    }

    format!(
        "Media recovery control {action} stayed local for {action_label} {filename}. Loaded metadata: {metadata_summary}. {opener_state}. SaveMedia completion still reports saved, download failed, save failed, optional opener failure, and successful Open folder/Replay cache updates through popup/status only. No cached Open folder/Replay handoff for this action, SaveMedia retry, FetchMedia, queue control, decrypt retry, codec/transcode, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL}"
    )
}

fn media_save_retry_unavailable_label(action_label: &str, filename: &str) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    format!(
        "Media {action_label} retry stayed local for {filename}: no plain MXC source was available in this row. Retry confirms before SaveMedia only for plain File/Audio/Video rows; encrypted/decrypt, queue resume/cancel, cached Open folder/Replay, codec/transcode, gateway/runtime/auth, and live mutation remain blocked."
    )
}

fn media_playback_download_queue_snapshot_label(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
    cached_saved_file_status: Option<&str>,
) -> String {
    let action = action.trim();
    let action = if action.is_empty() { "Queue" } else { action };
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let opener_state = if open_after_save {
        "Play mode would save first, then ask the system opener after a picked local path"
    } else {
        "Download mode would only write to the picked local path"
    };
    let cached_saved_file_status = cached_saved_file_status
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("No cached saved-file status is available for this row.");

    format!(
        "Local media playback/download queue snapshot: {action} selected for {action_label} {filename}; loaded metadata {metadata_summary}; {opener_state}; cached file: {cached_saved_file_status}; SaveMedia result states remain saved/download-failed/save-failed/opener-failed popup copy only. Queue renders this local snapshot only; no FetchMedia, SaveMedia, inline player startup, playback progress subscription, queue retry/resume/cancel, background download mutation, decrypt retry, codec/transcode, cached-destination Open folder/Replay handoff, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL}"
    )
}

fn media_save_preflight_detail_control_label(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
) -> String {
    let action = action.trim();
    let action = if action.is_empty() { "Request" } else { action };
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let opener_state = if open_after_save {
        "Play will only ask the system opener after confirmed SaveMedia writes the picked file"
    } else {
        "Download has no system opener phase"
    };
    let phase_state = match action {
        "Request" => "request metadata is the confirmation/picked-path state before SaveMedia",
        "Result" => {
            "result metadata is the local saved/download-failed/save-failed/opener-failed popup shape"
        }
        "Error" => "error metadata is cached popup copy only; no retry is submitted",
        "Retry" => {
            "retry metadata uses the separate guarded confirmation path when a plain MXC source is available"
        }
        "Source" => {
            "source metadata is loaded timeline media metadata plus local destination state"
        }
        _ => "preflight metadata stays local",
    };

    format!(
        "Media SaveMedia preflight {action} for {action_label} {filename}. Loaded metadata: {metadata_summary}. {phase_state}; {opener_state}. No FetchMedia, no extra SaveMedia, no cached-destination Open folder/Replay handoff, no retry automation, no queue control, no decrypt retry, no codec/transcode, no background download mutation, no delivery/read receipt, no attachment send, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
    )
}

fn media_codec_transcode_control_label(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
) -> String {
    let action = action.trim();
    let action = if action.is_empty() { "Codec" } else { action };
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Play"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let opener_state = if open_after_save {
        "Play still saves first with confirmed SaveMedia before the system opener phase"
    } else {
        "Download still writes only a picked local file after confirmation"
    };
    let phase_state = match action {
        "Codec" => "codec metadata is loaded MIME/duration/dimensions copy only; no decoder starts",
        "Transcode" => "transcode metadata is local gap copy only; no transcoder starts",
        "Captions" => {
            "caption metadata is unavailable local gap copy only; no subtitle fetch starts"
        }
        "Quality" => {
            "quality metadata is loaded timeline copy only; no adaptive stream or quality switch starts"
        }
        "Decrypt" => {
            "decrypt metadata is local blocked copy only; no key lookup or decrypt retry starts"
        }
        _ => "codec/transcode metadata stays local",
    };

    format!(
        "Media codec/transcode {action} stayed local for {action_label} {filename}. Loaded metadata: {metadata_summary}. {phase_state}; {opener_state}. No FetchMedia, SaveMedia, decoder, transcoder, captions fetch, quality switch, decrypt, inline player startup, system opener request, retry/cancel queue mutation, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {MEDIA_CODEC_TRANSCODE_CONTROLS_LABEL}"
    )
}

fn media_metadata_clipboard_payload(
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
) -> String {
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let kind = metadata.kind.trim();
    let kind = if kind.is_empty() { "Media" } else { kind };
    let mime = metadata
        .mime_type
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("unavailable");
    let size = metadata
        .size_label
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("unavailable");
    let duration = metadata
        .duration_label
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("unavailable");
    let dimensions = metadata
        .dimensions_label
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("unavailable");
    let summary = metadata.summary(filename);

    format!(
        "Media metadata\nType: {kind}\nFilename: {filename}\nMIME: {mime}\nSize: {size}\nDuration: {duration}\nDimensions: {dimensions}\nSummary: {summary}\n{MEDIA_METADATA_CLIPBOARD_LABEL}\nNo FetchMedia, SaveMedia, decrypt, codec/transcode, inline playback, system opener, retry/cancel queue control, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn media_metadata_clipboard_label(copied: bool, filename: &str, metadata_summary: &str) -> String {
    let action_state = if copied {
        "copied loaded media metadata to local clipboard"
    } else {
        "media metadata clipboard unavailable"
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    format!(
        "Media metadata clipboard: {action_state} for {filename}; loaded summary {metadata_summary}. {MEDIA_METADATA_CLIPBOARD_LABEL} No FetchMedia, SaveMedia, decrypt, codec/transcode, inline playback, system opener, retry/cancel queue control, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn media_metadata_field_or_unavailable(value: Option<&str>) -> &str {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("unavailable")
}

fn media_operation_packet_payload(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let kind = if metadata.kind.trim().is_empty() {
        "Media"
    } else {
        metadata.kind.trim()
    };
    let mime = media_metadata_field_or_unavailable(metadata.mime_type.as_deref());
    let size = media_metadata_field_or_unavailable(metadata.size_label.as_deref());
    let duration = media_metadata_field_or_unavailable(metadata.duration_label.as_deref());
    let dimensions = media_metadata_field_or_unavailable(metadata.dimensions_label.as_deref());
    let summary = metadata.summary(filename);
    let opener_state = if open_after_save {
        "system opener result slot: pending until confirmed SaveMedia writes a picked file; success/failure remains popup-only"
    } else {
        "system opener result slot: not requested for Download"
    };

    format!(
        "Media operation packet\nRequested action: {action_label}\nMedia kind: {kind}\nFilename: {filename}\nMIME: {mime}\nSize: {size}\nDuration: {duration}\nDimensions: {dimensions}\nLoaded summary: {summary}\nSaveMedia request slot: not built by Packet; only the existing confirmed Download/Play path may submit after confirmation and a picked local path\nSaveMedia result slot: saved, download failed, save failed, opener failed, and cached Open folder/Replay destinations remain popup/status mapping only\nInline playback acceptance: player shell, playhead, seek, queue, progress subscription, and inline restart are local blocked slots\nOpen folder/Replay acceptance: only a successful plain-MXC SaveMedia result may cache a destination for the OS folder or saved-file handoff\nDecrypt/decode acceptance: encrypted media key lookup, decrypt retry, image/audio/video decode, and decrypted preview are local blocked slots\nCodec/transcode acceptance: codec support, transcode fallback, captions, and quality selection are local blocked slots\nQueue acceptance: retry, resume, cancel, background download, delivery/read receipt mapping, and persistence are local blocked slots\n{opener_state}\nPromotion criteria: typed media operation contract, progress/result/error model, decrypt/decode path, codec/caption fallback path, and queue control mapping must exist before replacing this packet with live controls\nBoundary: no FetchMedia, SaveMedia, system opener request outside cached Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, gateway/runtime/auth, or live mutation"
    )
}

fn media_operation_packet_clipboard_label(
    copied: bool,
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
) -> String {
    let action_state = if copied {
        "copied local media operation packet to clipboard"
    } else {
        "media operation packet clipboard unavailable"
    };
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let opener_state = if open_after_save {
        "Play opener result slot included"
    } else {
        "Download-only opener slot omitted"
    };

    format!(
        "Media operation packet clipboard: {action_state} for {action_label} {filename}; loaded summary {metadata_summary}; {opener_state}. {MEDIA_OPERATION_PACKET_DRILLDOWN_LABEL} No FetchMedia, SaveMedia, decrypt, codec/transcode, inline playback, queue mutation, opener request, gateway/runtime/auth, or live mutation."
    )
}

fn media_playback_queue_contract_payload(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let kind = if metadata.kind.trim().is_empty() {
        "Media"
    } else {
        metadata.kind.trim()
    };
    let mime = media_metadata_field_or_unavailable(metadata.mime_type.as_deref());
    let size = media_metadata_field_or_unavailable(metadata.size_label.as_deref());
    let duration = media_metadata_field_or_unavailable(metadata.duration_label.as_deref());
    let dimensions = media_metadata_field_or_unavailable(metadata.dimensions_label.as_deref());
    let summary = metadata.summary(filename);
    let opener_state = if open_after_save {
        "system opener/open-folder result contract is represented as saved-file handoff status plus opener success/error slots"
    } else {
        "system opener/open-folder result contract is disabled for Download until a saved local file exists"
    };

    format!(
        "Media playback/queue typed contract\nRequested action: {action_label}\nMedia identity: kind={kind}; filename={filename}; MIME={mime}; size={size}; duration={duration}; dimensions={dimensions}; loaded summary={summary}\nSaveMedia contract: request metadata, picked destination, saved/download-failed/save-failed result, cached error, retry source, cached Open folder/Replay destination, and idempotency slots are local acceptance fields\nInline playback contract: player session id, play/pause, playhead, seek, duration, buffer/progress, inline restart, and teardown request/result/error slots stay local blocked fields\nOpen folder/Replay contract: only a successful plain-MXC SaveMedia result may cache a destination for local OS folder or saved-file handoff; cached Open folder and Replay validate that the saved path is still a regular file and clear stale cached destinations before any opener request; broader stale inline/decrypt/queue handling remains blocked before richer playback controls\nDecrypt/decode contract: encrypted source, key lookup, decrypt retry, decoded image/audio/video preview, thumbnail/full-file cache, stale local file, and error taxonomy slots stay local blocked fields\nCodec/transcode/captions contract: codec support, transcode fallback, caption fetch, quality switch, waveform/thumbnail fallback, and error taxonomy slots stay local blocked fields\nQueue contract: retry, resume, cancel, background download persistence, progress subscription, delivery/read receipt mapping, and stale queue item handling stay local blocked fields\n{opener_state}\nPromotion blockers: typed playback/media queue adapter, progress/result/error model, decrypt/decode path, codec/caption fallback, stale file handling beyond cached Open folder/Replay, and queue control mapping must exist before live controls replace this packet\nBoundary: no FetchMedia, SaveMedia, system opener request outside cached Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation\n{MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_LABEL}"
    )
}

fn media_playback_queue_contract_clipboard_label(
    copied: bool,
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
) -> String {
    let action_state = if copied {
        "copied local media playback/queue contract to clipboard"
    } else {
        "media playback/queue contract clipboard unavailable"
    };
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let opener_state = if open_after_save {
        "Play opener/result contract included"
    } else {
        "Download-only opener/result contract disabled"
    };

    format!(
        "Media playback/queue contract clipboard: {action_state} for {action_label} {filename}; loaded summary {metadata_summary}; {opener_state}. {MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_LABEL} No FetchMedia, SaveMedia, decrypt, codec/transcode, inline playback, queue mutation, opener request, gateway/runtime/auth, or live mutation."
    )
}

fn media_playback_result_taxonomy_payload(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let kind = if metadata.kind.trim().is_empty() {
        "Media"
    } else {
        metadata.kind.trim()
    };
    let mime = media_metadata_field_or_unavailable(metadata.mime_type.as_deref());
    let size = media_metadata_field_or_unavailable(metadata.size_label.as_deref());
    let duration = media_metadata_field_or_unavailable(metadata.duration_label.as_deref());
    let dimensions = media_metadata_field_or_unavailable(metadata.dimensions_label.as_deref());
    let summary = metadata.summary(filename);
    let opener_state = if open_after_save {
        "requested_play_opener_result: opened, failed, invalid_path via SaveMediaOpenOutcome only"
    } else {
        "requested_play_opener_result: not_requested_for_download"
    };

    format!(
        "Media playback decrypt/decode result taxonomy packet\nRequested action: {action_label}\nMedia identity: kind={kind}; filename={filename}; MIME={mime}; size={size}; duration={duration}; dimensions={dimensions}; loaded summary={summary}\nLive result references:\n- FetchMedia/image cache read: existing MatrixRequest::FetchMedia and MediaCache::try_get_media_or_fetch read paths only.\n- SaveMedia Download/Play: existing confirmed MatrixRequest::SaveMedia result path only.\n- Open folder/Replay: cached successful SaveMedia destination validation, stale cache eviction, and local OS opener handoff only.\n- Retry: existing PositiveConfirmationModal-gated SaveMedia retry for plain MXC rows only.\nBlocked decrypt/decode/opener/queue slots:\n- playback_session_id: not_assigned\n- playback_progress_result: playhead, buffered, completed, failed, stale not_wired\n- inline_player_result: opened, paused, seeked, failed, stale not_wired\n- decrypt_operation_id: not_assigned\n- decrypt_result: decrypted, missing_key, unsupported, failed, stale not_wired\n- decode_result: decoded_image, decoded_audio, decoded_video, unsupported_codec, failed, stale not_wired\n- codec_fallback_result: transcoded, captions_loaded, quality_switched, failed, stale not_wired\n- background_queue_result: queued, resumed, cancelled, failed, stale not_wired\n- delivery_receipt_result: not_wired\n- cached_file_stale_result: cached_open_folder_replay_only; broader_inline_decrypt_queue_stale_policy not_built\n- {opener_state}\n- retry_policy: PositiveConfirmationModal plain_mxc_source_required backend_request_id_and_source_hash_required_before_automatic_retry\n- cancel_policy: local_popup_no_background_queue_cancel\n- audit_redaction: no raw MXC access token, local absolute path beyond user-picked popup, encryption key, codec probe dump, caption body, or delivery receipt secret in local packet\nBoundary: {MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_LABEL} No FetchMedia, SaveMedia, system opener request outside cached Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt retry, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, Telegram delivery, or live mutation."
    )
}

fn media_playback_result_taxonomy_clipboard_label(
    copied: bool,
    action_label: &str,
    filename: &str,
    metadata_summary: &str,
    open_after_save: bool,
) -> String {
    let action_state = if copied {
        "copied local media result taxonomy to clipboard"
    } else {
        "media result taxonomy clipboard unavailable"
    };
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata_summary.trim();
    let metadata_summary = if metadata_summary.is_empty() {
        "metadata unavailable"
    } else {
        metadata_summary
    };
    let opener_state = if open_after_save {
        "Play opener taxonomy included"
    } else {
        "Download-only opener taxonomy disabled"
    };

    format!(
        "Media result taxonomy clipboard: {action_state} for {action_label} {filename}; loaded summary {metadata_summary}; {opener_state}. {MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_LABEL} No FetchMedia, SaveMedia, decrypt, codec/transcode, inline playback, queue mutation, opener request, gateway/runtime/auth, or live mutation."
    )
}

fn media_metadata_clipboard_url(filename: &str, metadata: &MediaDownloadActionMetadata) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer
        .append_pair("name", filename)
        .append_pair("kind", metadata.kind.as_str());
    if let Some(mime_type) = metadata.mime_type.as_deref() {
        serializer.append_pair("mime", mime_type);
    }
    if let Some(size_label) = metadata.size_label.as_deref() {
        serializer.append_pair("size", size_label);
    }
    if let Some(duration_label) = metadata.duration_label.as_deref() {
        serializer.append_pair("duration", duration_label);
    }
    if let Some(dimensions_label) = metadata.dimensions_label.as_deref() {
        serializer.append_pair("dimensions", dimensions_label);
    }
    let query = serializer.finish();
    format!("{MEDIA_METADATA_CLIPBOARD_URL_SCHEME}://media?{query}")
}

fn media_metadata_clipboard_link(filename: &str, metadata: &MediaDownloadActionMetadata) -> String {
    let url = media_metadata_clipboard_url(filename, metadata);
    format!(
        "<a href=\"{}\">Copy metadata</a>",
        htmlize::escape_attribute(&url)
    )
}

fn media_operation_packet_url(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer
        .append_pair("label", action_label)
        .append_pair("name", filename)
        .append_pair("open", if open_after_save { "1" } else { "0" })
        .append_pair("kind", metadata.kind.as_str());
    if let Some(mime_type) = metadata.mime_type.as_deref() {
        serializer.append_pair("mime", mime_type);
    }
    if let Some(size_label) = metadata.size_label.as_deref() {
        serializer.append_pair("size", size_label);
    }
    if let Some(duration_label) = metadata.duration_label.as_deref() {
        serializer.append_pair("duration", duration_label);
    }
    if let Some(dimensions_label) = metadata.dimensions_label.as_deref() {
        serializer.append_pair("dimensions", dimensions_label);
    }
    let query = serializer.finish();
    format!("{MEDIA_OPERATION_PACKET_URL_SCHEME}://media?{query}")
}

fn media_operation_packet_link(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let url = media_operation_packet_url(action_label, filename, metadata, open_after_save);
    format!("<a href=\"{}\">Packet</a>", htmlize::escape_attribute(&url))
}

fn media_playback_queue_contract_url(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer
        .append_pair("label", action_label)
        .append_pair("name", filename)
        .append_pair("open", if open_after_save { "1" } else { "0" })
        .append_pair("kind", metadata.kind.as_str());
    if let Some(mime_type) = metadata.mime_type.as_deref() {
        serializer.append_pair("mime", mime_type);
    }
    if let Some(size_label) = metadata.size_label.as_deref() {
        serializer.append_pair("size", size_label);
    }
    if let Some(duration_label) = metadata.duration_label.as_deref() {
        serializer.append_pair("duration", duration_label);
    }
    if let Some(dimensions_label) = metadata.dimensions_label.as_deref() {
        serializer.append_pair("dimensions", dimensions_label);
    }
    let query = serializer.finish();
    format!("{MEDIA_PLAYBACK_QUEUE_CONTRACT_URL_SCHEME}://media?{query}")
}

fn media_playback_queue_contract_link(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let url = media_playback_queue_contract_url(action_label, filename, metadata, open_after_save);
    format!(
        "<a href=\"{}\">Contract</a>",
        htmlize::escape_attribute(&url)
    )
}

fn media_playback_result_taxonomy_url(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer
        .append_pair("label", action_label)
        .append_pair("name", filename)
        .append_pair("open", if open_after_save { "1" } else { "0" })
        .append_pair("kind", metadata.kind.as_str());
    if let Some(mime_type) = metadata.mime_type.as_deref() {
        serializer.append_pair("mime", mime_type);
    }
    if let Some(size_label) = metadata.size_label.as_deref() {
        serializer.append_pair("size", size_label);
    }
    if let Some(duration_label) = metadata.duration_label.as_deref() {
        serializer.append_pair("duration", duration_label);
    }
    if let Some(dimensions_label) = metadata.dimensions_label.as_deref() {
        serializer.append_pair("dimensions", dimensions_label);
    }
    let query = serializer.finish();
    format!("{MEDIA_PLAYBACK_RESULT_TAXONOMY_URL_SCHEME}://media?{query}")
}

fn media_playback_result_taxonomy_link(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let url = media_playback_result_taxonomy_url(action_label, filename, metadata, open_after_save);
    format!(
        "<a href=\"{}\">Taxonomy</a>",
        htmlize::escape_attribute(&url)
    )
}
