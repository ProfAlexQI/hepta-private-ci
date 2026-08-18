use std::sync::{Arc, Mutex};

use makepad_widgets::WidgetUid;
use matrix_sdk::{
    Client, OwnedServerName, RoomMemberships,
    encryption::identities::Device,
    media::MediaRequestParameters,
    room::{RoomMember, edit::EditedContent, reply::Reply},
    ruma::{
        MatrixToUri, MatrixUri, OwnedEventId, OwnedMxcUri, OwnedRoomId, OwnedUserId,
        api::client::receipt::create_receipt::v3::ReceiptType,
        events::room::{MediaSource, message::RoomMessageEventContent},
    },
};
use matrix_sdk_ui::timeline::TimelineEventItemId;
use ruma::OwnedRoomOrAliasId;

use crate::{
    avatar_cache::AvatarUpdate,
    home::{link_preview::LinkPreviewData, timeline_update_queue::TimelineUpdateSender},
    media_cache::{MediaCacheEntry, MediaCacheEntryRef},
    persistence::ClientSessionPersisted,
    profile::user_profile::UserProfile,
    shared::{
        attachment_download::MediaDownloadResult,
        file_upload_modal::{AttachmentUpload, FileUploadAttemptId},
    },
    utils::RoomNameId,
};

/// Which direction to paginate in.
///
/// * `Forwards` will retrieve later events (towards the end of the timeline),
///   which only works if the timeline is *focused* on a specific event.
/// * `Backwards`: the more typical choice, in which earlier events are retrieved
///   (towards the start of the timeline), which works in  both live mode and focused mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaginationDirection {
    Forwards,
    Backwards,
}
impl std::fmt::Display for PaginationDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forwards => write!(f, "forwards"),
            Self::Backwards => write!(f, "backwards"),
        }
    }
}

/// The function signature for the callback that gets invoked when media is fetched.
pub type OnMediaFetchedFn = fn(
    &Mutex<MediaCacheEntry>,
    MediaRequestParameters,
    matrix_sdk::Result<Vec<u8>>,
    Option<TimelineUpdateSender>,
);

/// Error types for URL preview operations.
#[derive(Debug)]
pub enum UrlPreviewError {
    /// The Matrix client was not available.
    ClientNotAvailable,
    /// The request to the homeserver failed.
    Request(matrix_sdk::HttpError),
    /// Parsing the preview JSON failed.
    Json(serde_json::Error),
}

impl std::fmt::Display for UrlPreviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlPreviewError::ClientNotAvailable => write!(f, "Matrix client not available"),
            UrlPreviewError::Request(e) => write!(f, "HTTP request failed: {e}"),
            UrlPreviewError::Json(e) => write!(f, "JSON parsing failed: {e}"),
        }
    }
}

impl std::error::Error for UrlPreviewError {}

/// The function signature for the callback that gets invoked when link preview data is fetched.
pub type OnLinkPreviewFetchedFn = fn(
    Arc<Mutex<crate::home::link_preview::TimestampedCacheEntry>>,
    Result<LinkPreviewData, UrlPreviewError>,
    Option<TimelineUpdateSender>,
);

/// Actions emitted in response to a [`MatrixRequest::GenerateMatrixLink`].
#[derive(Clone, Debug)]
pub enum MatrixLinkAction {
    MatrixToUri(MatrixToUri),
    MatrixUri(MatrixUri),
    Error(String),
}

/// Actions emitted when account data (e.g., avatar, display name) changes.
#[derive(Clone, Debug)]
pub enum AccountDataAction {
    /// The user's avatar was successfully updated or removed.
    AvatarChanged(Option<OwnedMxcUri>),
    /// Failed to update or remove the user's avatar.
    AvatarChangeFailed(String),
    /// The user's display name was successfully updated or removed.
    DisplayNameChanged(Option<String>),
    /// Failed to update the user's display name.
    DisplayNameChangeFailed(String),
    /// Result of [`MatrixRequest::GetOwnDevice`], in a `Box` because `Device` is large.
    /// * `None` if not logged in or the crypto store isn't ready yet.
    OwnDeviceFetched(Option<Box<Device>>),
}

/// Actions emitted in response to a [`MatrixRequest::OpenOrCreateDirectMessage`].
#[derive(Debug)]
pub enum DirectMessageRoomAction {
    /// A direct message room already existed with the given user.
    FoundExisting {
        user_id: OwnedUserId,
        room_name_id: RoomNameId,
    },
    /// A direct message room didn't exist, and we didn't attempt to create a new one.
    DidNotExist { user_profile: UserProfile },
    /// A direct message room didn't exist, but we successfully created a new one.
    NewlyCreated {
        user_profile: UserProfile,
        room_name_id: RoomNameId,
    },
    /// A direct message room didn't exist, and we failed to create a new one.
    FailedToCreate {
        user_profile: UserProfile,
        error: matrix_sdk::Error,
    },
}

/// Either a main room timeline or a thread-focused timeline.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimelineKind {
    MainRoom {
        room_id: OwnedRoomId,
    },
    Thread {
        room_id: OwnedRoomId,
        thread_root_event_id: OwnedEventId,
    },
}
impl TimelineKind {
    pub fn room_id(&self) -> &OwnedRoomId {
        match self {
            TimelineKind::MainRoom { room_id } => room_id,
            TimelineKind::Thread { room_id, .. } => room_id,
        }
    }

    pub fn thread_root_event_id(&self) -> Option<&OwnedEventId> {
        match self {
            TimelineKind::MainRoom { .. } => None,
            TimelineKind::Thread {
                thread_root_event_id,
                ..
            } => Some(thread_root_event_id),
        }
    }
}
impl std::fmt::Display for TimelineKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimelineKind::MainRoom { room_id } => write!(f, "MainRoom({})", room_id),
            TimelineKind::Thread {
                room_id,
                thread_root_event_id,
            } => {
                write!(f, "Thread({}, {})", room_id, thread_root_event_id)
            }
        }
    }
}

/// The desired response for a [`MatrixRequest::GetRoomPreview`].
#[derive(Clone, Debug)]
pub enum RoomPreviewResponseMode {
    /// Posts a [`crate::room::RoomPreviewAction::Fetched`] action with the result.
    Action,
    /// Enqueues the result to be inserted into the [`crate::room_preview_cache`],
    /// if successful.
    RoomPreviewCache,
}

/// The set of requests for async work that can be made to the worker thread.
#[allow(clippy::large_enum_variant)]
pub enum MatrixRequest {
    /// Request from the login screen to log in with the given credentials.
    Login(LoginRequest),
    /// Request to logout.
    Logout { is_desktop: bool },
    /// Request to paginate the older (or newer) events of a room or thread timeline.
    PaginateTimeline {
        timeline_kind: TimelineKind,
        /// The maximum number of timeline events to fetch in each pagination batch.
        num_events: u16,
        direction: PaginationDirection,
    },
    /// Request to edit the content of an event in the given room's timeline.
    EditMessage {
        timeline_kind: TimelineKind,
        timeline_event_item_id: TimelineEventItemId,
        edited_content: EditedContent,
    },
    /// Request to fetch the full details of the given event in the given room's timeline.
    FetchDetailsForEvent {
        timeline_kind: TimelineKind,
        event_id: OwnedEventId,
    },
    /// Request to fetch the latest thread-reply preview and latest reply count
    /// for the given thread root.
    FetchThreadSummaryDetails {
        timeline_kind: TimelineKind,
        thread_root_event_id: OwnedEventId,
        timeline_item_index: usize,
    },
    /// Request to fetch profile information for all members of a room.
    ///
    /// This can be *very* slow depending on the number of members in the room.
    ///
    /// Even though it operates on a room itself, this accepts a `TimelineKind`
    /// in order to be able to send the fetched room member list to a specific timeline UI.
    SyncRoomMemberList { timeline_kind: TimelineKind },
    /// Request to create a thread timeline focused on the given thread root event in the given room.
    CreateThreadTimeline {
        room_id: OwnedRoomId,
        thread_root_event_id: OwnedEventId,
    },
    /// Request to stop a thread timeline's backend sync loop (e.g. for when its tab was closed).
    CloseThreadTimeline {
        room_id: OwnedRoomId,
        thread_root_event_id: OwnedEventId,
    },
    /// Request to knock on (request an invite to) the given room.
    Knock {
        room_or_alias_id: OwnedRoomOrAliasId,
        reason: Option<String>,
        #[doc(alias("via"))]
        server_names: Vec<OwnedServerName>,
    },
    /// Request to invite the given user to the given room.
    InviteUser {
        room_id: OwnedRoomId,
        user_id: OwnedUserId,
    },
    /// Request to join the given room.
    JoinRoom { room_id: OwnedRoomId },
    /// Request to leave the given room.
    LeaveRoom { room_id: OwnedRoomId },
    /// Request to get the actual list of members in a room.
    ///
    /// This returns the list of members that can be displayed in the UI.
    ///
    /// Even though it operates on a room itself, this accepts a `TimelineKind`
    /// in order to be able to send the fetched room member list to a specific timeline UI.
    GetRoomMembers {
        timeline_kind: TimelineKind,
        memberships: RoomMemberships,
        /// * If `true` (not recommended), only the local cache will be accessed.
        /// * If `false` (recommended), details will be fetched from the server.
        local_only: bool,
    },
    /// Request to fetch the preview (basic info) for the given room,
    /// either one that is joined locally or one that is unknown.
    ///
    /// On completion, the result is dispatched according to `response_mode`:
    /// either as a [`crate::room::RoomPreviewAction::Fetched`] action, or by enqueueing
    /// a cache update into the [`crate::room_preview_cache`].
    GetRoomPreview {
        room_or_alias_id: OwnedRoomOrAliasId,
        via: Vec<OwnedServerName>,
        response_mode: RoomPreviewResponseMode,
    },
    /// Request to fetch the full details (the room preview) of a tombstoned room.
    GetSuccessorRoomDetails { tombstoned_room_id: OwnedRoomId },
    /// Request to create or open a direct message room with the given user.
    ///
    /// If there is no existing DM room with the given user, this will create a new DM room
    /// if `allow_create` is `true`; otherwise it will emit an action indicating that
    /// no DM room existed, upon which the UI will prompt the user to confirm that they want
    /// to proceed with creating a new DM room.
    #[doc(alias("dm"))]
    OpenOrCreateDirectMessage {
        user_profile: UserProfile,
        allow_create: bool,
    },
    /// Request to fetch profile information for the given user ID.
    GetUserProfile {
        user_id: OwnedUserId,
        /// * If `Some`, the user is known to be a member of a room, so this will
        ///   fetch the user's profile from that room's membership info.
        /// * If `None`, the user's profile info will be fetched from the server
        ///   in a room-agnostic manner, and no room membership info will be returned.
        room_id: Option<OwnedRoomId>,
        /// * If `true` (not recommended), only the local cache will be accessed.
        /// * If `false` (recommended), details will be fetched from the server.
        local_only: bool,
    },
    /// Request to fetch the number of unread messages in the given room.
    GetNumberUnreadMessages { timeline_kind: TimelineKind },
    /// Request to set the unread flag for the given room.
    SetUnreadFlag {
        room_id: OwnedRoomId,
        /// If `true`, marks the room as unread.
        /// If `false`, marks the room as read.
        mark_as_unread: bool,
    },
    /// Request to set the favorite flag for the given room.
    SetIsFavorite {
        room_id: OwnedRoomId,
        is_favorite: bool,
    },
    /// Request to set the low priority flag for the given room.
    SetIsLowPriority {
        room_id: OwnedRoomId,
        is_low_priority: bool,
    },
    /// Request to generate a Matrix link (permalink) for a room or event.
    GenerateMatrixLink {
        /// The ID of the room to generate a link for.
        room_id: OwnedRoomId,
        /// * If `Some`, the link will point to this specific event within the room.
        /// * If `None`, the link will point to the room itself.
        event_id: Option<OwnedEventId>,
        /// * If `true`, the `matrix:` URI scheme will be used to create a [`MatrixUri`].
        /// * If `false` (default), the `https://matrix.to` scheme will be used to create a [`MatrixToUri`].
        use_matrix_scheme: bool,
        /// * If `true` (default is false), the link will include an action hint to join the room.
        join_on_click: bool,
    },
    /// Request to ignore/block or unignore/unblock a user.
    IgnoreUser {
        /// Whether to ignore (`true`) or unignore (`false`) the user.
        ignore: bool,
        /// The room membership info of the user to (un)ignore.
        room_member: RoomMember,
        /// The room ID of the room where the user is a member,
        /// which is only needed because it isn't present in the `RoomMember` object.
        room_id: OwnedRoomId,
    },
    /// Request to set or remove the avatar of the current user's account.
    SetAvatar {
        /// * If `Some`, the avatar will be set to the given MXC URI.
        /// * If `None`, the avatar will be removed.
        avatar_url: Option<OwnedMxcUri>,
    },
    /// Request to set or remove the display name of the current user's account.
    SetDisplayName {
        /// * If `Some`, the display name will be set to the given value.
        /// * If `None`, the display name will be removed.
        new_display_name: Option<String>,
    },
    /// Request to fetch our own [`Device`].
    /// The response is delivered via [`AccountDataAction::OwnDeviceFetched`].
    GetOwnDevice,
    /// Request to verify this device by sending an outgoing verification request
    /// to the user's other logged-in devices, which'll open the verification modal.
    RequestSelfVerification,
    /// Request to fetch an Avatar image from the server.
    /// Upon completion of the async media request, the `on_fetched` function
    /// will be invoked with the content of an `AvatarUpdate`.
    FetchAvatar {
        mxc_uri: OwnedMxcUri,
        on_fetched: fn(AvatarUpdate),
    },
    /// Request to fetch or compute a room's avatar.
    /// Returns the result via [`crate::home::rooms_list::RoomsListUpdate::UpdateRoomAvatar`].
    FetchRoomAvatar { room_name_id: RoomNameId },
    /// Request to fetch media from the server.
    /// Upon completion of the async media request, the `on_fetched` function
    /// will be invoked with four arguments: the `destination`, the `media_request`,
    /// the result of the media fetch, and the `update_sender`.
    FetchMedia {
        media_request: MediaRequestParameters,
        on_fetched: OnMediaFetchedFn,
        destination: MediaCacheEntryRef,
        update_sender: Option<TimelineUpdateSender>,
    },
    /// Request to send a message to the given room.
    SendMessage {
        timeline_kind: TimelineKind,
        message: RoomMessageEventContent,
        replied_to: Option<Reply>,
        #[cfg(feature = "tsp")]
        sign_with_tsp: bool,
    },
    /// Request to send a file attachment to the given room.
    SendAttachment {
        upload_id: FileUploadAttemptId,
        upload: AttachmentUpload,
    },
    /// Sends a notice to the given room that the current user is or is not typing.
    ///
    /// This request does not return a response or notify the UI thread, and
    /// furthermore, there is no need to send a follow-up request to stop typing
    /// (though you certainly can do so).
    SendTypingNotice { room_id: OwnedRoomId, typing: bool },
    /// Spawn an async task to login to the given Matrix homeserver using the given SSO identity provider ID.
    ///
    /// While an SSO request is in flight, the login screen will temporarily prevent the user
    /// from submitting another redundant request, until this request has succeeded or failed.
    SpawnSSOServer {
        brand: String,
        homeserver_url: String,
        identity_provider_id: String,
    },
    /// Subscribe to typing notices for the given room.
    ///
    /// This is only valid for the main room timeline, not for thread-focused timelines.
    ///
    /// This request does not immediately return a response or notify the UI thread,
    /// but it will send updates to the UI via the timeline's update sender.
    SubscribeToTypingNotices {
        room_id: OwnedRoomId,
        /// Whether to subscribe or unsubscribe.
        subscribe: bool,
    },
    /// Subscribe to changes in the read receipts of our own user.
    ///
    /// This request does not immediately return a response or notify the UI thread,
    /// but it will send updates to the UI via the timeline's update sender.
    SubscribeToOwnUserReadReceiptsChanged {
        timeline_kind: TimelineKind,
        /// Whether to subscribe or unsubscribe.
        subscribe: bool,
    },
    /// Subscribe to changes in the set of pinned events for the given room.
    ///
    /// This is only valid for the main room timeline, not for thread-focused timelines.
    SubscribeToPinnedEvents {
        room_id: OwnedRoomId,
        /// Whether to subscribe or unsubscribe.
        subscribe: bool,
    },
    /// Sends a read receipt for the given event to the given room or thread timeline.
    ReadReceipt {
        timeline_kind: TimelineKind,
        event_id: OwnedEventId,
        receipt_type: ReceiptType,
    },
    /// Sends a request to obtain the power levels for this room.
    ///
    /// The response is delivered back to the main UI thread via [`TimelineUpdate::UserPowerLevels`].
    ///
    /// Even though it operates on a room itself, this accepts a `TimelineKind`
    /// in order to be able to send the fetched room member list to a specific timeline UI.
    GetRoomPowerLevels { timeline_kind: TimelineKind },
    /// Toggles the given reaction to the given event in the given room.
    ToggleReaction {
        timeline_kind: TimelineKind,
        timeline_event_id: TimelineEventItemId,
        reaction: String,
    },
    /// Redacts (deletes) the given event in the given room.
    #[doc(alias("delete"))]
    RedactMessage {
        timeline_kind: TimelineKind,
        timeline_event_id: TimelineEventItemId,
        reason: Option<String>,
    },
    /// Pin or unpin the given event in the given room.
    #[doc(alias("unpin"))]
    PinEvent {
        timeline_kind: TimelineKind,
        event_id: OwnedEventId,
        pin: bool,
    },
    /// Request to fetch URL preview from the Matrix homeserver.
    GetUrlPreview {
        url: String,
        on_fetched: OnLinkPreviewFetchedFn,
        destination: Arc<Mutex<crate::home::link_preview::TimestampedCacheEntry>>,
        update_sender: Option<TimelineUpdateSender>,
    },
    /// Request to download a media attachment/file.
    ///
    /// The given callback `on_download_result` is called from the backend
    /// matrix worker tokio task.
    /// If the given McxUri was already downloading, the request is rejected
    /// and `on_download_result` is called with `Cancelled`.
    DownloadMedia {
        media_source: MediaSource,
        filename: String,
        on_download_result: Box<dyn FnOnce(MediaDownloadResult) + Send + 'static>,
    },
    /// Request to cancel an in-progress download.
    CancelDownload(OwnedMxcUri),
    /// Request to find all known rooms and spaces that match the `query` string.
    ///
    /// Returns a list of matching rooms/spaces via
    /// [`crate::shared::mentionable_text_input::MentionMatches`].
    GetMatchingRooms {
        query: String,
        request_id: u64,
        owner: WidgetUid,
    },
}

/// Details of a login request that get submitted within [`MatrixRequest::Login`].
pub enum LoginRequest {
    LoginByPassword(LoginByPassword),
    LoginBySSOSuccess(Client, ClientSessionPersisted),
    LoginByCli,
    HomeserverLoginTypesQuery(String),
}
/// Information needed to log in to a Matrix homeserver.
pub struct LoginByPassword {
    pub user_id: String,
    pub password: String,
    pub homeserver: Option<String>,
}
