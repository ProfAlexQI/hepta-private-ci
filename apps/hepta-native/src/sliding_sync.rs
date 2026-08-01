use anyhow::{anyhow, bail, Result};
use bitflags::bitflags;
use clap::Parser;
use eyeball::Subscriber;
use eyeball_im::VectorDiff;
use futures_util::{future::join_all, pin_mut, StreamExt};
use imbl::Vector;
use makepad_widgets::{error, log, warning, Cx, SignalToUI};
use matrix_sdk_base::crypto::{DecryptionSettings, TrustRequirement};
use matrix_sdk::{
    config::RequestConfig,
    encryption::{identities::Device, EncryptionSettings},
    event_handler::EventHandlerDropGuard,
    media::MediaRequestParameters,
    notification_settings::{IsEncrypted, IsOneToOne, RoomNotificationMode},
    room::{edit::EditedContent, reply::Reply, IncludeRelations, RelationsOptions, RoomMember},
    ruma::{
        api::{
            Direction,
            client::{
                filter::{RoomEventFilter, UrlFilter},
                profile::{AvatarUrl, DisplayName},
                receipt::create_receipt::v3::ReceiptType,
                search::search_events,
            },
        },
        events::{
            Mentions,
            relation::RelationType,
            room::{
                canonical_alias::RoomCanonicalAliasEventContent,
                history_visibility::{HistoryVisibility, RoomHistoryVisibilityEventContent},
                join_rules::{JoinRule, RoomJoinRulesEventContent},
                message::{RoomMessageEventContent, TextMessageEventContent},
                power_levels::RoomPowerLevels,
                tombstone::RoomTombstoneEventContent,
                MediaSource,
            },
            MessageLikeEventType, StateEventType,
        },
        EventId, MatrixToUri, MatrixUri, MilliSecondsSinceUnixEpoch, OwnedDeviceId, OwnedEventId,
        OwnedMxcUri, OwnedRoomAliasId, OwnedRoomId, OwnedUserId, RoomOrAliasId, UserId, uint,
    },
    send_queue::SendHandle,
    sliding_sync::VersionBuilder,
    Client, ClientBuildError, OwnedServerName, Room, RoomDisplayName, RoomMemberships, RoomState,
    SessionChange, SuccessorRoom,
};
#[cfg(not(target_os = "ios"))]
use matrix_sdk::Error;
use matrix_sdk_ui::{
    RoomListService, Timeline, encryption_sync_service,
    room_list_service::{RoomListItem, RoomListLoadingState, SyncIndicator, filters},
    sync_service::{self, SyncService},
    timeline::{
        AttachmentConfig, LatestEventValue, RoomExt, TimelineDetails, TimelineEventItemId,
        TimelineFocus, TimelineItem, TimelineReadReceiptTracking,
    },
};
#[cfg(not(target_os = "ios"))]
use robius_open::Uri;
use ruma::{OwnedRoomOrAliasId, RoomId, events::tag::Tags};
use tokio::{
    runtime::Handle,
    sync::{
        broadcast,
        mpsc::{Sender, UnboundedReceiver, UnboundedSender},
        watch, Notify,
    },
    task::JoinHandle,
    time::error::Elapsed,
};
use url::Url;
use std::{
    borrow::Cow,
    cmp::{max, min},
    future::Future,
    hash::{BuildHasherDefault, DefaultHasher},
    iter::Peekable,
    ops::{Deref, DerefMut, Not},
    path::{Path, PathBuf},
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use std::io;
use hashbrown::{HashMap, HashSet};
use crate::{
    app::AppStateAction,
    app_data_dir,
    avatar_cache::AvatarUpdate,
    event_preview::{
        BeforeText, TextPreview, text_preview_of_raw_timeline_event, text_preview_of_timeline_item,
    },
    home::{
        add_room::KnockResultAction,
        invite_screen::{JoinRoomResultAction, LeaveRoomResultAction},
        link_preview::{LinkPreviewData, LinkPreviewDataNonNumeric, LinkPreviewRateLimitResponse},
        room_screen::{
            matrix_link_preview_failure_metadata_label, matrix_link_preview_result_metadata_label,
            EditHistorySummary, InviteResultAction, MatrixLinkJoinResultAction,
            MessageSearchServerFilter, MessageSearchServerHit, MessageSearchServerResponse,
            RoomSettingsMutationField, TimelineUpdate,
        },
        rooms_list::{
            self, InvitedRoomInfo, InviterInfo, JoinedRoomInfo, RoomsListUpdate,
            enqueue_rooms_list_update,
        },
        rooms_list_header::RoomsListHeaderAction,
        tombstone_footer::SuccessorRoomDetails,
    },
    login::login_screen::LoginAction,
    logout::{
        logout_confirm_modal::LogoutAction,
        logout_state_machine::{LogoutConfig, is_logout_in_progress, logout_with_state_machine},
    },
    room_preview_cache::{enqueue_room_preview_update, RoomPreviewUpdate},
    media_cache::{MediaCacheEntry, MediaCacheEntryRef},
    persistence::{self, ClientSessionPersisted, load_app_state},
    profile::{
        user_profile::UserProfile,
        user_profile_cache::{UserProfileUpdate, enqueue_user_profile_update},
    },
    room::{FetchedRoomAvatar, FetchedRoomPreview, RoomPreviewAction},
    shared::{
        avatar::AvatarState,
        jump_to_bottom_button::UnreadMessageCount,
        popup_list::{PopupKind, enqueue_popup_notification},
    },
    space_service_sync::space_service_loop,
    utils::{self, AVATAR_THUMBNAIL_FORMAT, RoomNameId, VecDiff, avatar_from_room_name},
    verification::add_verification_event_handlers_and_sync_client,
};

#[derive(Parser, Default)]
struct Cli {
    /// The user ID to login with.
    #[clap(value_parser)]
    user_id: String,

    /// The password that should be used for the login.
    #[clap(value_parser)]
    password: String,

    /// The homeserver to connect to.
    #[clap(value_parser)]
    homeserver: Option<String>,

    /// Set the proxy that should be used for the connection.
    #[clap(short, long)]
    proxy: Option<String>,

    /// Force login screen.
    #[clap(short, long, action)]
    login_screen: bool,

    /// Enable verbose logging output.
    #[clap(short, long, action)]
    verbose: bool,
}

impl std::fmt::Debug for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cli")
            .field("user_id", &self.user_id)
            .field("password", &"<REDACTED>")
            .field("homeserver", &self.homeserver)
            .field("proxy", &self.proxy)
            .field("login_screen", &self.login_screen)
            .field("verbose", &self.verbose)
            .finish()
    }
}

impl From<LoginByPassword> for Cli {
    fn from(login: LoginByPassword) -> Self {
        Self {
            user_id: login.user_id,
            password: login.password,
            homeserver: login.homeserver,
            proxy: None,
            login_screen: false,
            verbose: false,
        }
    }
}

/// Shared SQLite store config for both `build_client` and `restore_session`,
/// so both code paths can't drift.
pub fn build_sqlite_store_config(
    db_path: &Path,
    passphrase: &str,
) -> matrix_sdk::SqliteStoreConfig {
    matrix_sdk::SqliteStoreConfig::with_low_memory_config(db_path).passphrase(Some(passphrase))
}

/// Build a new client.
async fn build_client(
    cli: &Cli,
    data_dir: &Path,
) -> Result<(Client, ClientSessionPersisted), ClientBuildError> {
    // Generate a unique subfolder name for the client database,
    // which allows multiple clients to run simultaneously.
    let now = chrono::Local::now();
    let db_subfolder_name: String = format!("db_{}", now.format("%F_%H_%M_%S_%f"));
    let db_path = data_dir.join(&db_subfolder_name);
    log!("Building new client with db at: {}", db_path.display());

    // Eagerly creat the db dir to avoid any issues within the matrix SDK.
    if let Err(e) = tokio::fs::create_dir_all(&db_path).await {
        error!(
            "Failed to pre-create db directory at {}: {e}. Continuing anyway; \
             matrix-sdk-sqlite will retry the create internally.",
            db_path.display(),
        );
    }

    // Generate a random passphrase.
    let passphrase: String = {
        use rand::{Rng, thread_rng};
        thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    };

    let homeserver_url = cli
        .homeserver
        .as_deref()
        .unwrap_or("https://matrix-client.matrix.org/");
    // .unwrap_or("https://matrix.org/");

    let store_config = build_sqlite_store_config(&db_path, &passphrase);

    let mut builder = Client::builder()
        .server_name_or_homeserver_url(homeserver_url)
        // Use a sqlite database to persist the client's encryption setup.
        .sqlite_store_with_config_and_cache_path(store_config, None::<&std::path::Path>)
        .with_threading_support(matrix_sdk::ThreadingSupport::Enabled {
            with_subscriptions: true,
        })
        // The sliding sync proxy has now been deprecated in favor of native sliding sync.
        .sliding_sync_version_builder(VersionBuilder::DiscoverNative)
        .with_decryption_settings(DecryptionSettings {
            sender_device_trust_requirement: TrustRequirement::Untrusted,
        })
        .with_encryption_settings(EncryptionSettings {
            auto_enable_cross_signing: true,
            backup_download_strategy: matrix_sdk::encryption::BackupDownloadStrategy::OneShot,
            auto_enable_backups: true,
        })
        .with_enable_share_history_on_invite(true)
        .handle_refresh_tokens();

    if let Some(proxy) = cli.proxy.as_ref() {
        builder = builder.proxy(proxy.clone());
    }

    // Use a 60 second timeout for all requests to the homeserver.
    // Yes, this is a long timeout, but the standard matrix homeserver is often very slow.
    builder =
        builder.request_config(RequestConfig::new().timeout(std::time::Duration::from_secs(60)));
    let client = builder.build().await?;
    let homeserver_url = client.homeserver().to_string();
    Ok((
        client,
        ClientSessionPersisted {
            homeserver: homeserver_url,
            // Store the relative subfolder name only. The absolute path is
            // rebuilt on restore. Avoids baking in a sandbox path that goes
            // stale on iOS (container UUID changes across reinstalls).
            db_path: PathBuf::from(db_subfolder_name),
            passphrase,
        },
    ))
}

/// Logs in to the given Matrix homeserver using the given username and password.
///
/// This function is used by the login screen to log in to the Matrix server.
///
/// Upon success, this function returns the logged-in client and an optional sync token.
async fn login(cli: &Cli, login_request: LoginRequest) -> Result<(Client, Option<String>)> {
    match login_request {
        LoginRequest::LoginByCli | LoginRequest::LoginByPassword(_) => {
            let cli = if let LoginRequest::LoginByPassword(login_by_password) = login_request {
                &Cli::from(login_by_password)
            } else {
                cli
            };
            let (client, client_session) = build_client(cli, app_data_dir()).await?;
            Cx::post_action(LoginAction::Status {
                title: "Authenticating".into(),
                status: format!("Logging in as {}...", cli.user_id),
            });
            // Attempt to login using the CLI-provided username & password.
            let login_result = client
                .matrix_auth()
                .login_username(&cli.user_id, &cli.password)
                .initial_device_display_name("hepta-native-un-pw")
                .send()
                .await?;
            if client.matrix_auth().logged_in() {
                log!("Logged in successfully.");
                let status = format!("Logged in as {}.\n → Loading rooms...", cli.user_id);
                // enqueue_popup_notification(status.clone());
                enqueue_rooms_list_update(RoomsListUpdate::Status { status });
                if let Err(e) = persistence::save_session(&client, client_session).await {
                    let err_msg = format!("Failed to save session state to storage: {e}");
                    error!("{err_msg}");
                    enqueue_popup_notification(err_msg, PopupKind::Error, None);
                }
                Ok((client, None))
            } else {
                let err_msg = format!("Failed to login as {}: {:?}", cli.user_id, login_result);
                enqueue_popup_notification(err_msg.clone(), PopupKind::Error, None);
                enqueue_rooms_list_update(RoomsListUpdate::Status {
                    status: err_msg.clone(),
                });
                bail!(err_msg);
            }
        }

        LoginRequest::LoginBySSOSuccess(client, client_session) => {
            if let Err(e) = persistence::save_session(&client, client_session).await {
                error!("Failed to save session state to storage: {e:?}");
            }
            Ok((client, None))
        }
        LoginRequest::HomeserverLoginTypesQuery(_) => {
            bail!("LoginRequest::HomeserverLoginTypesQuery not handled earlier");
        }
    }
}

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
    Option<crossbeam_channel::Sender<TimelineUpdate>>,
);

enum SaveMediaOpenOutcome {
    NotRequested,
    Opened,
    Failed(String),
    InvalidPath,
}

fn save_media_success_popup_message(
    destination_path: &Path,
    open_outcome: SaveMediaOpenOutcome,
) -> String {
    let mut message = format!("Media saved to {}", destination_path.display());
    match open_outcome {
        SaveMediaOpenOutcome::NotRequested => {}
        SaveMediaOpenOutcome::Opened => {
            message.push_str(" and opened.");
        }
        SaveMediaOpenOutcome::Failed(error) => {
            message.push_str(&format!(", but opening it failed: {error}"));
        }
        SaveMediaOpenOutcome::InvalidPath => {
            message.push_str(", but the saved path could not be opened.");
        }
    }
    message
}

#[cfg(test)]
mod save_media_tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn save_media_success_popup_message_reports_download_only_save() {
        let label = save_media_success_popup_message(
            Path::new("/tmp/hepta-media.bin"),
            SaveMediaOpenOutcome::NotRequested,
        );

        assert!(label.contains("Media saved to /tmp/hepta-media.bin"));
        assert!(!label.contains("opened"));
    }

    #[test]
    fn save_media_success_popup_message_reports_opened_play_result() {
        let label = save_media_success_popup_message(
            Path::new("/tmp/hepta-media.bin"),
            SaveMediaOpenOutcome::Opened,
        );

        assert!(label.contains("Media saved to /tmp/hepta-media.bin and opened."));
    }

    #[test]
    fn save_media_success_popup_message_reports_opener_failure() {
        let label = save_media_success_popup_message(
            Path::new("/tmp/hepta-media.bin"),
            SaveMediaOpenOutcome::Failed("permission denied".to_string()),
        );

        assert!(label.contains("Media saved to /tmp/hepta-media.bin"));
        assert!(label.contains("opening it failed: permission denied"));
    }
}

/// Error types for URL preview operations.
#[derive(Debug)]
pub enum UrlPreviewError {
    /// HTTP request failed.
    Request(matrix_sdk::reqwest::Error),
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Client not available.
    ClientNotAvailable,
    /// Access token not available.
    AccessTokenNotAvailable,
    /// HTTP error status.
    HttpStatus(u16),
    /// URL parsing error.
    UrlParse(url::ParseError),
}

impl std::fmt::Display for UrlPreviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlPreviewError::Request(e) => write!(f, "HTTP request failed: {}", e),
            UrlPreviewError::Json(e) => write!(f, "JSON parsing failed: {}", e),
            UrlPreviewError::ClientNotAvailable => write!(f, "Matrix client not available"),
            UrlPreviewError::AccessTokenNotAvailable => write!(f, "Access token not available"),
            UrlPreviewError::HttpStatus(status) => write!(f, "HTTP {} error", status),
            UrlPreviewError::UrlParse(e) => write!(f, "URL parsing failed: {}", e),
        }
    }
}

impl std::error::Error for UrlPreviewError {}

/// The function signature for the callback that gets invoked when link preview data is fetched.
pub type OnLinkPreviewFetchedFn = fn(
    String,
    Arc<Mutex<crate::home::link_preview::TimestampedCacheEntry>>,
    Result<LinkPreviewData, UrlPreviewError>,
    Option<crossbeam_channel::Sender<TimelineUpdate>>,
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
    /// Result of [`MatrixRequest::GetDevices`] for the current user's device directory.
    OwnDevicesFetched(Result<Vec<AccountDeviceDirectoryEntry>, String>),
    /// Result of [`MatrixRequest::RenameDevice`] for the current user's device.
    DeviceRenamed(Result<AccountDeviceRenameResult, String>),
}

/// Actions emitted in response to a [`MatrixRequest::SearchUserDirectory`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserDirectorySearchAction {
    /// Result of a live Matrix user-directory search.
    Searched(Result<UserDirectorySearchResult, String>),
}

/// Lightweight account device-directory data for the Native UI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountDeviceDirectoryEntry {
    pub device_id: String,
    pub display_name: Option<String>,
    pub last_seen_ip: Option<String>,
    pub last_seen_ts_ms: Option<u64>,
}

/// Lightweight result for a confirmed current-device display-name rename.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountDeviceRenameResult {
    pub device_id: OwnedDeviceId,
    pub display_name: String,
}

/// Lightweight user-directory search result for mention-picker discovery.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDirectorySearchResult {
    pub query: String,
    pub limited: bool,
    pub results: Vec<UserDirectorySearchEntry>,
}

/// Lightweight Matrix user-directory row for the Native UI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDirectorySearchEntry {
    pub user_id: OwnedUserId,
    pub display_name: Option<String>,
    pub avatar_url: Option<OwnedMxcUri>,
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
    /// Posts a [`RoomPreviewAction::Fetched`] action with the result.
    Action,
    /// Enqueues the result to be inserted into the [`crate::room_preview_cache`],
    /// if successful.
    RoomPreviewCache,
}

/// The enabled Matrix notification keyword-rule state read through [`NotificationSettings`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotificationKeywordRulesSummary {
    /// Whether the SDK reports at least one enabled custom keyword rule.
    pub has_enabled_keywords: bool,
    /// Enabled custom keyword patterns, sorted for stable UI rendering.
    pub enabled_keywords: Vec<String>,
}

/// The supported custom notification keyword mutation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NotificationKeywordMutation {
    /// Add or enable an enabled content push rule for a keyword.
    Add,
    /// Remove all custom content push rules for a keyword.
    Remove,
}

/// Read-only Matrix push capability status for notification device/pusher surfaces.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotificationPusherStatusSummary {
    /// Whether the homeserver advertises encrypted event-to-device push support.
    pub encrypted_event_to_device_push: Result<bool, String>,
}

/// The Matrix default notification mode for the loaded room's current room class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotificationDefaultRoomModeSummary {
    /// The default notification mode the SDK resolves for this room class.
    pub mode: RoomNotificationMode,
    /// Whether the loaded room is encrypted for notification-default matching.
    pub is_encrypted: bool,
    /// Whether the loaded room currently has the one-to-one notification class.
    pub is_one_to_one: bool,
    /// Active member count used by the SDK to classify one-to-one defaults.
    pub active_members_count: u64,
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
    /// Request to fetch a compact edit-history summary for the given event.
    FetchEditHistory {
        timeline_kind: TimelineKind,
        event_id: OwnedEventId,
    },
    /// Request to fetch raw JSON source for an event in the current room.
    FetchEventSource {
        timeline_kind: TimelineKind,
        event_id: OwnedEventId,
    },
    /// Request a live Matrix server-side message search for the current room.
    SearchMessagesServer {
        timeline_kind: TimelineKind,
        query: String,
        filter: MessageSearchServerFilter,
        limit: u16,
        next_batch: Option<String>,
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
    /// Request to join the given room id or alias through a Matrix link target.
    JoinRoomByIdOrAlias {
        room_or_alias_id: OwnedRoomOrAliasId,
        #[doc(alias("via"))]
        server_names: Vec<OwnedServerName>,
    },
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
    /// either as a [`RoomPreviewAction::Fetched`] action, or by enqueueing
    /// a cache update into the [`crate::room_preview_cache`].
    GetRoomPreview {
        room_or_alias_id: OwnedRoomOrAliasId,
        via: Vec<OwnedServerName>,
        response_mode: RoomPreviewResponseMode,
    },
    /// Request a compact room preview for an unknown Matrix link target.
    PreviewMatrixLinkTarget {
        timeline_kind: TimelineKind,
        room_or_alias_id: OwnedRoomOrAliasId,
        via: Vec<OwnedServerName>,
        event_id: Option<OwnedEventId>,
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
    /// Request a live Matrix user-directory search.
    ///
    /// The response is delivered via [`UserDirectorySearchAction::Searched`].
    SearchUserDirectory { query: String, limit: u64 },
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
    /// Request to set the notification mode for the given room.
    SetRoomNotificationMode {
        room_id: OwnedRoomId,
        mode: RoomNotificationMode,
    },
    /// Request to set the current room's display name state event.
    SetRoomName {
        timeline_kind: TimelineKind,
        name: String,
    },
    /// Request to set the current room's topic state event.
    SetRoomTopic {
        timeline_kind: TimelineKind,
        topic: String,
    },
    /// Request to remove the current room's avatar state event.
    RemoveRoomAvatar { timeline_kind: TimelineKind },
    /// Request to upload image bytes and set them as the current room's avatar.
    UploadRoomAvatar {
        timeline_kind: TimelineKind,
        file_path: PathBuf,
        mime_type: mime::Mime,
    },
    /// Request to set the current room's history visibility state event.
    SetRoomHistoryVisibility {
        timeline_kind: TimelineKind,
        visibility: String,
    },
    /// Request to set the current room's join rule state event.
    SetRoomJoinRule {
        timeline_kind: TimelineKind,
        join_rule: String,
    },
    /// Request to set the current room's canonical alias state event.
    SetRoomCanonicalAlias {
        timeline_kind: TimelineKind,
        alias: String,
        alt_aliases: Vec<OwnedRoomAliasId>,
    },
    /// Request to set the current room's tombstone replacement state event.
    SetRoomTombstone {
        timeline_kind: TimelineKind,
        replacement_room_id: OwnedRoomId,
        reason: String,
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
    /// Request to upload image bytes and set them as the current user's account avatar.
    UploadAvatar {
        file_path: PathBuf,
        mime_type: mime::Mime,
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
    /// Request to fetch all devices for the current account.
    /// The response is delivered via [`AccountDataAction::OwnDevicesFetched`].
    GetDevices,
    /// Request to rename a Matrix device display name.
    /// The response is delivered via [`AccountDataAction::DeviceRenamed`].
    RenameDevice {
        device_id: OwnedDeviceId,
        display_name: String,
    },
    /// Request to fetch an Avatar image from the server.
    /// Upon completion of the async media request, the `on_fetched` function
    /// will be invoked with the content of an `AvatarUpdate`.
    FetchAvatar {
        mxc_uri: OwnedMxcUri,
        on_fetched: fn(AvatarUpdate),
    },
    /// Request to fetch media from the server.
    /// Upon completion of the async media request, the `on_fetched` function
    /// will be invoked with four arguments: the `destination`, the `media_request`,
    /// the result of the media fetch, and the `update_sender`.
    FetchMedia {
        media_request: MediaRequestParameters,
        on_fetched: OnMediaFetchedFn,
        destination: MediaCacheEntryRef,
        update_sender: Option<crossbeam_channel::Sender<TimelineUpdate>>,
    },
    /// Request to fetch media from the server and save it to a user-selected path.
    SaveMedia {
        media_request: MediaRequestParameters,
        destination_path: PathBuf,
        open_after_save: bool,
        update_sender: Option<crossbeam_channel::Sender<TimelineUpdate>>,
    },
    /// Request to send a message to the given room.
    SendMessage {
        timeline_kind: TimelineKind,
        message: RoomMessageEventContent,
        replied_to: Option<Reply>,
        #[cfg(feature = "tsp")]
        sign_with_tsp: bool,
    },
    /// Request to send a file attachment to the given room timeline.
    SendAttachment {
        timeline_kind: TimelineKind,
        file_path: PathBuf,
        mime_type: mime::Mime,
        caption: Option<TextMessageEventContent>,
        mentions: Option<Mentions>,
        in_reply_to: Option<OwnedEventId>,
    },
    /// Request to abort a pending local echo send queue item through its SDK handle.
    AbortLocalSend {
        timeline_kind: TimelineKind,
        send_handle: SendHandle,
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
    /// Reads the effective notification mode for this room without changing push rules.
    ///
    /// The response is delivered back to the main UI thread via [`TimelineUpdate::RoomNotificationMode`].
    GetRoomNotificationMode { timeline_kind: TimelineKind },
    /// Reads enabled custom notification keyword rules from Matrix notification settings.
    ///
    /// The response is delivered back to the main UI thread via
    /// [`TimelineUpdate::NotificationKeywordRulesFetched`].
    GetNotificationKeywordRules { timeline_kind: TimelineKind },
    /// Adds/enables or removes custom notification keyword rules through SDK notification settings.
    ///
    /// The response is delivered back to the UI thread via
    /// [`TimelineUpdate::NotificationKeywordRulesMutated`].
    SetNotificationKeywordRule {
        timeline_kind: TimelineKind,
        keyword: String,
        mutation: NotificationKeywordMutation,
    },
    /// Reads homeserver/device push capability without changing pusher configuration.
    ///
    /// The response is delivered back to the main UI thread via
    /// [`TimelineUpdate::NotificationPusherStatusFetched`].
    GetNotificationPusherStatus { timeline_kind: TimelineKind },
    /// Reads the Matrix default notification mode for this room's current room class.
    ///
    /// The response is delivered back to the main UI thread via
    /// [`TimelineUpdate::NotificationDefaultRoomModeFetched`].
    GetDefaultRoomNotificationMode { timeline_kind: TimelineKind },
    /// Sets the Matrix default notification mode for this room's current room class.
    ///
    /// The response is delivered back to the main UI thread via
    /// [`TimelineUpdate::NotificationDefaultRoomModeMutated`].
    SetDefaultRoomNotificationMode {
        timeline_kind: TimelineKind,
        mode: RoomNotificationMode,
    },
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
    /// Reports the given event content in the given room.
    ReportContent {
        timeline_kind: TimelineKind,
        event_id: OwnedEventId,
        reason: String,
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
        update_sender: Option<crossbeam_channel::Sender<TimelineUpdate>>,
    },
}

/// Submits a request to the worker thread to be executed asynchronously.
pub fn submit_async_request(req: MatrixRequest) {
    if let Some(sender) = REQUEST_SENDER.lock().unwrap().as_ref() {
        sender
            .send(req)
            .expect("BUG: matrix worker task receiver has died!");
    }
}

pub(crate) fn sanitize_user_directory_search_query(query: &str) -> String {
    query.trim().trim_start_matches('@').trim().to_string()
}

fn parse_room_history_visibility(value: &str) -> Result<HistoryVisibility, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "invited" => Ok(HistoryVisibility::Invited),
        "joined" => Ok(HistoryVisibility::Joined),
        "shared" => Ok(HistoryVisibility::Shared),
        "world_readable" | "world-readable" | "world readable" => {
            Ok(HistoryVisibility::WorldReadable)
        }
        other => Err(format!("Unsupported history visibility `{other}`")),
    }
}

fn parse_room_join_rule(value: &str) -> Result<JoinRule, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "invite" => Ok(JoinRule::Invite),
        "knock" => Ok(JoinRule::Knock),
        "private" => Ok(JoinRule::Private),
        "public" => Ok(JoinRule::Public),
        other => Err(format!("Unsupported join rule `{other}`")),
    }
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

mod worker;

use worker::matrix_worker_task;

/// The single global Tokio runtime that is used by all async tasks.
static TOKIO_RUNTIME: Mutex<Option<tokio::runtime::Runtime>> = Mutex::new(None);

/// The sender used by [`submit_async_request`] to send requests to the async worker thread.
/// Currently there is only one, but it can be cloned if we need more concurrent senders.
static REQUEST_SENDER: Mutex<Option<UnboundedSender<MatrixRequest>>> = Mutex::new(None);

/// A client object that is proactively created during initialization
/// in order to speed up the client-building process when the user logs in.
static DEFAULT_SSO_CLIENT: Mutex<Option<(Client, ClientSessionPersisted)>> = Mutex::new(None);

/// Used to notify the SSO login task that the async creation of the `DEFAULT_SSO_CLIENT` has finished.
static DEFAULT_SSO_CLIENT_NOTIFIER: LazyLock<Arc<Notify>> =
    LazyLock::new(|| Arc::new(Notify::new()));

/// Handle to the in-flight `ASWebAuthenticationSession`. Set when the auth
/// sheet is presented, cleared by the completion callback or by
/// [`cancel_active_sso_auth_session`].
#[cfg(target_os = "ios")]
static ACTIVE_SSO_AUTH_SESSION: Mutex<Option<robius_web_auth_session::AuthSessionHandle>> =
    Mutex::new(None);

/// Dismiss the iOS auth sheet. The completion callback fires with
/// `UserCancelled`, which surfaces as a `LoginFailure` and resets all
/// SSO state so the next attempt works. No-op if nothing's running.
#[cfg(target_os = "ios")]
pub fn cancel_active_sso_auth_session() {
    if let Ok(mut slot) = ACTIVE_SSO_AUTH_SESSION.lock() {
        if let Some(handle) = slot.take() {
            handle.cancel();
        }
    }
}

/// Blocks the current thread until the given future completes.
///
/// ## Warning
/// This should be used with caution, especially on the main UI thread,
/// as blocking a thread prevents it from handling other events or running other tasks.
pub fn block_on_async_with_timeout<T>(
    timeout: Option<Duration>,
    async_future: impl Future<Output = T>,
) -> Result<T, Elapsed> {
    let rt = TOKIO_RUNTIME
        .lock()
        .unwrap()
        .get_or_insert_with(|| {
            tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime")
        })
        .handle()
        .clone();

    if let Some(timeout) = timeout {
        rt.block_on(async { tokio::time::timeout(timeout, async_future).await })
    } else {
        Ok(rt.block_on(async_future))
    }
}

/// The primary initialization routine for starting the Matrix client sync
/// and the async tokio runtime.
///
/// Returns a handle to the Tokio runtime that is used to run async background tasks.
pub fn start_matrix_tokio() -> Result<tokio::runtime::Handle> {
    // Create a Tokio runtime, and save it in a static variable to ensure it isn't dropped.
    let rt_handle = TOKIO_RUNTIME
        .lock()
        .unwrap()
        .get_or_insert_with(|| {
            tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime")
        })
        .handle()
        .clone();

    let rt = rt_handle.clone();
    // Spawn the main async task that drives the Matrix client SDK and
    // monitors the related background tasks. (The `DEFAULT_SSO_CLIENT`
    // pre-build is gated inside that task on whether the user actually
    // needs to log in. Otherwise it leaves an orphaned sqlite db on disk
    // every cold start.)
    rt_handle.spawn(start_matrix_client_login_and_sync(rt));

    Ok(rt_handle)
}

/// A tokio::watch channel sender for sending requests from the RoomScreen UI widget
/// to the corresponding background async task for that room (its `timeline_subscriber_handler`).
pub type TimelineRequestSender = watch::Sender<Vec<BackwardsPaginateUntilEventRequest>>;

/// The return type for [`take_timeline_endpoints()`].
///
/// This primarily contains endpoints for channels of communication
/// between the timeline UI (`RoomScreen`] and the background worker tasks.
/// If the relevant room was tombstoned, this also includes info about its successor room.
pub struct TimelineEndpoints {
    pub update_sender: crossbeam_channel::Sender<TimelineUpdate>,
    pub update_receiver: crossbeam_channel::Receiver<TimelineUpdate>,
    pub request_sender: TimelineRequestSender,
    pub successor_room: Option<SuccessorRoom>,
}

/// Info about a timeline for a joined room or a thread in a joined room.
struct PerTimelineDetails {
    /// A shared reference to a room's main timeline or thread's timeline of events.
    timeline: Arc<Timeline>,
    /// A clone-able sender for updates to this timeline.
    timeline_update_sender: crossbeam_channel::Sender<TimelineUpdate>,
    /// A tuple of two separate channel endpoints that can only be taken *once* by the main UI thread.
    ///
    /// 1. The single receiver that can receive updates from this timeline.
    ///    * When a new room is joined (or a thread is opened), an unbounded crossbeam channel will be created
    ///      and its sender given to a background task (the `timeline_subscriber_handler()`)
    ///      that enqueues timeline updates as it receives timeline vector diffs from the server.
    ///    * The UI thread can take ownership of this update receiver in order to receive updates
    ///      to this room or thread timeline, but only one receiver can exist at a time.
    /// 2. The sender that can send requests to the background timeline subscriber handler,
    ///    e.g., to watch for a specific event to be prepended to the timeline (via back pagination).
    timeline_singleton_endpoints: Option<(
        crossbeam_channel::Receiver<TimelineUpdate>,
        TimelineRequestSender,
    )>,
    /// The async task that listens for updates for this timeline.
    timeline_subscriber_handler_task: JoinHandle<()>,
}

struct JoinedRoomDetails {
    /// The room ID of this joined room.
    room_id: OwnedRoomId,
    /// Details about the main timeline for this room.
    main_timeline: PerTimelineDetails,
    /// Thread-focused timelines for this room, keyed by thread root event ID.
    thread_timelines: HashMap<OwnedEventId, PerTimelineDetails>,
    /// The set of thread timelines currently being created, to avoid duplicate in-flight work.
    pending_thread_timelines: HashSet<OwnedEventId>,
    /// A drop guard for the event handler that represents a subscription to typing notices for this room.
    typing_notice_subscriber: Option<EventHandlerDropGuard>,
    /// A drop guard for the event handler that represents a subscription to pinned events for this room.
    pinned_events_subscriber: Option<EventHandlerDropGuard>,
}
impl Drop for JoinedRoomDetails {
    fn drop(&mut self) {
        log!("Dropping JoinedRoomDetails for room {}", self.room_id);
        self.main_timeline.timeline_subscriber_handler_task.abort();
        for thread_timeline in self.thread_timelines.values() {
            thread_timeline.timeline_subscriber_handler_task.abort();
        }
        drop(self.typing_notice_subscriber.take());
        drop(self.pinned_events_subscriber.take());
    }
}

/// A const-compatible hasher, used for `static` items containing `HashMap`s or `HashSet`s.
type ConstHasher = BuildHasherDefault<DefaultHasher>;

/// Information about all joined rooms that our client currently know about.
/// We use a `HashMap` for O(1) lookups, as this is accessed frequently (e.g. every timeline update).
static ALL_JOINED_ROOMS: Mutex<HashMap<OwnedRoomId, JoinedRoomDetails, ConstHasher>> =
    Mutex::new(HashMap::with_hasher(BuildHasherDefault::new()));

/// Returns the timeline and timeline update sender for the given joined room/thread timeline.
fn get_per_timeline_details<'a>(
    all_joined_rooms: &'a mut HashMap<OwnedRoomId, JoinedRoomDetails, ConstHasher>,
    kind: &TimelineKind,
) -> Option<&'a mut PerTimelineDetails> {
    let room_info = all_joined_rooms.get_mut(kind.room_id())?;
    match kind {
        TimelineKind::MainRoom { .. } => Some(&mut room_info.main_timeline),
        TimelineKind::Thread {
            thread_root_event_id,
            ..
        } => room_info.thread_timelines.get_mut(thread_root_event_id),
    }
}

/// Obtains the lock on `ALL_JOINED_ROOMS` and returns the timeline for the given timeline kind.
fn get_timeline(kind: &TimelineKind) -> Option<Arc<Timeline>> {
    get_per_timeline_details(ALL_JOINED_ROOMS.lock().unwrap().deref_mut(), kind)
        .map(|details| details.timeline.clone())
}

/// Obtains the lock on `ALL_JOINED_ROOMS` and returns the timeline and timeline update sender for the given timeline kind.
fn get_timeline_and_sender(
    kind: &TimelineKind,
) -> Option<(Arc<Timeline>, crossbeam_channel::Sender<TimelineUpdate>)> {
    get_per_timeline_details(ALL_JOINED_ROOMS.lock().unwrap().deref_mut(), kind).map(|details| {
        (
            details.timeline.clone(),
            details.timeline_update_sender.clone(),
        )
    })
}

/// Obtains the lock on `ALL_JOINED_ROOMS` and returns the main timeline for the given room.
fn get_room_timeline(room_id: &RoomId) -> Option<Arc<Timeline>> {
    ALL_JOINED_ROOMS
        .lock()
        .unwrap()
        .get(room_id)
        .map(|jrd| jrd.main_timeline.timeline.clone())
}

/// The logged-in Matrix client, which can be freely and cheaply cloned.
static CLIENT: Mutex<Option<Client>> = Mutex::new(None);

pub fn get_client() -> Option<Client> {
    CLIENT.lock().unwrap().clone()
}

/// Returns the user ID of the currently logged-in user, if any.
pub fn current_user_id() -> Option<OwnedUserId> {
    CLIENT
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|c| c.session_meta().map(|m| m.user_id.clone()))
}

/// The singleton sync service.
static SYNC_SERVICE: Mutex<Option<Arc<SyncService>>> = Mutex::new(None);
static SYNC_SERVICE_DESIRED_RUNNING: AtomicBool = AtomicBool::new(true);
static SYNC_SERVICE_ASSUMED_RUNNING: AtomicBool = AtomicBool::new(false);
static SYNC_SERVICE_LIFECYCLE_LOCK: LazyLock<tokio::sync::Mutex<()>> =
    LazyLock::new(|| tokio::sync::Mutex::new(()));

/// Set to `true` when the access token has been rejected by the homeserver,
/// signaling the main task to tear down the current session and wait for re-login.
static TOKEN_EXPIRED: AtomicBool = AtomicBool::new(false);
/// Notifies the main monitoring loop to wake up and check `TOKEN_EXPIRED`.
static TOKEN_EXPIRED_NOTIFY: LazyLock<Notify> = LazyLock::new(Notify::new);

/// Wakes the monitoring loop on logout, see `is_logout_in_progress()`.
static LOGOUT_NOTIFY: LazyLock<Notify> = LazyLock::new(Notify::new);

/// Get a reference to the current sync service, if available.
pub fn get_sync_service() -> Option<Arc<SyncService>> {
    SYNC_SERVICE.lock().ok()?.as_ref().cloned()
}

pub fn sync_service_desired_running() -> bool {
    SYNC_SERVICE_DESIRED_RUNNING.load(Ordering::Acquire)
}

pub fn set_sync_service_desired_running(running: bool, reason: &'static str) {
    let previous = SYNC_SERVICE_DESIRED_RUNNING.swap(running, Ordering::AcqRel);
    if previous == running && SYNC_SERVICE_ASSUMED_RUNNING.load(Ordering::Acquire) == running {
        log!(
            "Matrix sync service already desired {}; skipping lifecycle request ({reason}).",
            if running { "running" } else { "stopped" }
        );
        return;
    }

    let rt_handle = TOKIO_RUNTIME
        .lock()
        .unwrap()
        .as_ref()
        .map(|rt| rt.handle().clone());
    let Some(rt_handle) = rt_handle else {
        log!(
            "Stored Matrix sync desired state as {}; Tokio runtime is not running yet ({reason}).",
            if running { "running" } else { "stopped" }
        );
        return;
    };

    rt_handle.spawn(apply_sync_service_desired_state(reason));
}

async fn apply_sync_service_desired_state(reason: &'static str) {
    let _guard = SYNC_SERVICE_LIFECYCLE_LOCK.lock().await;
    loop {
        let desired = SYNC_SERVICE_DESIRED_RUNNING.load(Ordering::Acquire);
        if SYNC_SERVICE_ASSUMED_RUNNING.load(Ordering::Acquire) == desired {
            break;
        }

        let Some(sync_service) = get_sync_service() else {
            log!(
                "Matrix sync service is not available while applying lifecycle request ({reason})."
            );
            break;
        };

        if desired {
            log!("Starting Matrix sync service after lifecycle request ({reason}).");
            sync_service.start().await;
        } else {
            log!("Stopping Matrix sync service after lifecycle request ({reason}).");
            sync_service.stop().await;
        }
        SYNC_SERVICE_ASSUMED_RUNNING.store(desired, Ordering::Release);
    }
}

pub fn stop_sync_service_for_shutdown(timeout: Duration) -> Result<(), Elapsed> {
    SYNC_SERVICE_DESIRED_RUNNING.store(false, Ordering::Release);
    let Some(sync_service) = get_sync_service() else {
        SYNC_SERVICE_ASSUMED_RUNNING.store(false, Ordering::Release);
        return Ok(());
    };

    let result = block_on_async_with_timeout(Some(timeout), async move {
        let _guard = SYNC_SERVICE_LIFECYCLE_LOCK.lock().await;
        sync_service.stop().await;
    });
    if result.is_ok() {
        SYNC_SERVICE_ASSUMED_RUNNING.store(false, Ordering::Release);
    }
    result
}

/// The list of users that the current user has chosen to ignore.
/// Ideally we shouldn't have to maintain this list ourselves,
/// but the Matrix SDK doesn't currently properly maintain the list of ignored users.
static IGNORED_USERS: Mutex<HashSet<OwnedUserId, ConstHasher>> =
    Mutex::new(HashSet::with_hasher(BuildHasherDefault::new()));

/// Returns a deep clone of the current list of ignored users.
pub fn get_ignored_users() -> HashSet<OwnedUserId, ConstHasher> {
    IGNORED_USERS.lock().unwrap().clone()
}

/// Returns whether the given user ID is currently being ignored.
pub fn is_user_ignored(user_id: &UserId) -> bool {
    IGNORED_USERS.lock().unwrap().contains(user_id)
}

/// Returns three channel endpoints related to the timeline for the given joined room or thread.
///
/// 1. A timeline update sender.
/// 2. The timeline update receiver, which is a singleton, and can only be taken once.
/// 3. A `tokio::watch` sender that can be used to send requests to the timeline subscriber handler.
///
/// This will only succeed once per room (or once per room thread),
/// as only a single channel receiver can exist.
pub fn take_timeline_endpoints(kind: &TimelineKind) -> Option<TimelineEndpoints> {
    let mut all_joined_rooms = ALL_JOINED_ROOMS.lock().unwrap();
    let jrd = all_joined_rooms.get_mut(kind.room_id())?;
    let details = match kind {
        TimelineKind::MainRoom { .. } => &mut jrd.main_timeline,
        TimelineKind::Thread {
            thread_root_event_id,
            ..
        } => jrd.thread_timelines.get_mut(thread_root_event_id)?,
    };
    let (update_receiver, request_sender) = details.timeline_singleton_endpoints.take()?;
    Some(TimelineEndpoints {
        update_sender: details.timeline_update_sender.clone(),
        update_receiver,
        request_sender,
        successor_room: details.timeline.room().successor_room(),
    })
}

const DEFAULT_HOMESERVER: &str = "matrix.org";

fn username_to_full_user_id(username: &str, homeserver: Option<&str>) -> Option<OwnedUserId> {
    username.try_into().ok().or_else(|| {
        let homeserver_url = homeserver.unwrap_or(DEFAULT_HOMESERVER);
        let user_id_str = if username.starts_with("@") {
            format!("{}:{}", username, homeserver_url)
        } else {
            format!("@{}:{}", username, homeserver_url)
        };
        user_id_str.as_str().try_into().ok()
    })
}

/// Info we store about a room received by the room list service.
///
/// This struct is necessary in order for us to track the previous state
/// of a room received from the room list service, so that we can
/// determine what room data has changed since the last update.
/// We can't just store the `matrix_sdk::Room` object itself,
/// because that is a shallow reference to an inner room object within
/// the room list service.
#[derive(Clone)]
struct RoomListServiceRoomInfo {
    room_id: OwnedRoomId,
    state: RoomState,
    is_direct: bool,
    is_marked_unread: bool,
    is_tombstoned: bool,
    tags: Option<Tags>,
    user_power_levels: Option<UserPowerLevels>,
    latest_event_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    num_unread_messages: u64,
    num_unread_mentions: u64,
    display_name: Option<RoomDisplayName>,
    room_avatar: Option<OwnedMxcUri>,
    room: matrix_sdk::Room,
}
impl RoomListServiceRoomInfo {
    async fn from_room(room: matrix_sdk::Room, current_user_id: &Option<OwnedUserId>) -> Self {
        // Parallelize fetching of independent room data.
        let (is_direct, tags, display_name, user_power_levels) =
            tokio::join!(room.is_direct(), room.tags(), room.display_name(), async {
                if let Some(user_id) = current_user_id {
                    UserPowerLevels::from_room(&room, user_id.deref()).await
                } else {
                    None
                }
            });

        Self {
            room_id: room.room_id().to_owned(),
            state: room.state(),
            is_direct: is_direct.unwrap_or(false),
            is_marked_unread: room.is_marked_unread(),
            is_tombstoned: room.is_tombstoned(),
            tags: tags.ok().flatten(),
            user_power_levels,
            latest_event_timestamp: room.latest_event_timestamp(),
            num_unread_messages: room.num_unread_messages(),
            num_unread_mentions: room.num_unread_mentions(),
            display_name: display_name.ok(),
            room_avatar: room.avatar_url(),
            room,
        }
    }
    async fn from_room_ref(room: &matrix_sdk::Room, current_user_id: &Option<OwnedUserId>) -> Self {
        Self::from_room(room.clone(), current_user_id).await
    }
}

/// Aborts all handles in parallel, then awaits each so their Drop chain
/// (Arcs, channels, etc.) finishes before we move on.
async fn abort_and_await_handles(handles: &mut Vec<JoinHandle<()>>) {
    for h in handles.iter() {
        h.abort();
    }
    for h in handles.drain(..) {
        // Skip handles we've already consumed, as those would block forever.
        if !h.is_finished() {
            let _ = h.await;
        }
    }
}

/// Performs the Matrix client login or session restore, and starts the main sync service.
///
/// After starting the sync service, this also starts the main room list service loop
/// and the main space service loop.
async fn start_matrix_client_login_and_sync(rt: Handle) {
    // Run clean up before anything else, like creating new db dirs.
    persistence::cleanup_orphan_db_dirs().await;

    // Create a channel for sending requests from the main UI thread to a background worker task.
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<MatrixRequest>();
    REQUEST_SENDER.lock().unwrap().replace(sender);

    let (login_sender, mut login_receiver) = tokio::sync::mpsc::channel(1);

    // Spawn the async worker task that handles matrix requests.
    // We must do this now such that the matrix worker task can listen for incoming login requests
    // from the UI, and forward them to this task (via the login_sender --> login_receiver).
    let mut matrix_worker_task_handle = rt.spawn(matrix_worker_task(receiver, login_sender));

    let most_recent_user_id = persistence::most_recent_user_id().await;
    log!("Most recent user ID: {most_recent_user_id:?}");
    let cli_parse_result = Cli::try_parse();
    let cli_has_valid_username_password = cli_parse_result
        .as_ref()
        .is_ok_and(|cli| !cli.user_id.is_empty() && !cli.password.is_empty());
    log!(
        "CLI parsing succeeded? {}. CLI has valid UN+PW? {}",
        cli_parse_result.as_ref().is_ok(),
        cli_has_valid_username_password,
    );
    let wait_for_login = !cli_has_valid_username_password
        && (most_recent_user_id.is_none()
            || std::env::args().any(|arg| arg == "--login-screen" || arg == "--force-login"));
    log!("Waiting for login? {}", wait_for_login);

    let new_login_opt = if !wait_for_login {
        let specified_username = cli_parse_result
            .as_ref()
            .ok()
            .and_then(|cli| username_to_full_user_id(&cli.user_id, cli.homeserver.as_deref()));
        log!(
            "Trying to restore session for user: {:?}",
            specified_username.as_ref().or(most_recent_user_id.as_ref())
        );
        match persistence::restore_session(specified_username).await {
            Ok(session) => Some(session),
            Err(e) => {
                let status_err = "Could not restore previous user session.\n\nPlease login again.";
                log!("{status_err} Error: {e:?}");
                Cx::post_action(LoginAction::LoginFailure(status_err.to_string()));

                if let Ok(cli) = &cli_parse_result {
                    log!(
                        "Attempting auto-login from CLI arguments as user '{}'...",
                        cli.user_id
                    );
                    Cx::post_action(LoginAction::CliAutoLogin {
                        user_id: cli.user_id.clone(),
                        homeserver: cli.homeserver.clone(),
                    });
                    match login(cli, LoginRequest::LoginByCli).await {
                        Ok(new_login) => Some(new_login),
                        Err(e) => {
                            error!("CLI-based login failed: {e:?}");
                            Cx::post_action(LoginAction::LoginFailure(format!(
                                "Could not login with CLI-provided arguments.\n\nPlease login manually.\n\nError: {e}"
                            )));
                            enqueue_rooms_list_update(RoomsListUpdate::Status {
                                status: format!("Login failed: {e:?}"),
                            });
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }
    } else {
        None
    };
    let cli: Cli = cli_parse_result.unwrap_or(Cli::default());
    // `initial_client_opt` holds the client obtained from the session restore or CLI auto-login.
    // On subsequent iterations of the login loop (after a post-auth setup failure), it is `None`,
    // which causes the loop to wait for the user to submit a new manual login request.
    let mut initial_client_opt = new_login_opt;

    // Only pre-build `DEFAULT_SSO_CLIENT` if we'll actually show the login
    // screen. Building it eagerly during session restore just leaves an
    // orphaned sqlite db every cold start. If we skip the build, still
    // notify so a later SSO attempt doesn't deadlock on the notifier.
    // The SSO handler builds a fresh client itself if it's still `None`.
    if initial_client_opt.is_none() {
        rt.spawn(async move {
            match build_client(&Cli::default(), app_data_dir()).await {
                Ok(client_and_session) => {
                    DEFAULT_SSO_CLIENT
                        .lock()
                        .unwrap()
                        .get_or_insert(client_and_session);
                }
                Err(e) => error!("Error: could not create DEFAULT_SSO_CLIENT object: {e}"),
            };
            DEFAULT_SSO_CLIENT_NOTIFIER.notify_one();
            Cx::post_action(LoginAction::SsoPending(false));
        });
    } else {
        DEFAULT_SSO_CLIENT_NOTIFIER.notify_one();
    }

    'login_loop: loop {
        let (client, _sync_token) = match initial_client_opt.take() {
            Some(login) => login,
            None => loop {
                log!("Waiting for login request...");
                match login_receiver.recv().await {
                    Some(login_request) => match login(&cli, login_request).await {
                        Ok((client, sync_token)) => break (client, sync_token),
                        Err(e) => {
                            error!("Login failed: {e:?}");
                            Cx::post_action(LoginAction::LoginFailure(format!("{e}")));
                            enqueue_rooms_list_update(RoomsListUpdate::Status {
                                status: format!("Login failed: {e}"),
                            });
                        }
                    },
                    None => {
                        error!("BUG: login_receiver hung up unexpectedly");
                        let err = String::from(
                            "Please restart Hepta Native.\n\nUnable to listen for login requests.",
                        );
                        Cx::post_action(LoginAction::LoginFailure(err.clone()));
                        enqueue_rooms_list_update(RoomsListUpdate::Status { status: err });
                        return;
                    }
                }
            },
        };
        client.send_queue().enable_upload_progress(true);
        log!("Enabled Matrix SDK send queue media upload progress reporting.");

        // Deallocate the default SSO client after a successful login.
        if let Ok(mut client_opt) = DEFAULT_SSO_CLIENT.lock() {
            let _ = client_opt.take();
        }

        let logged_in_user_id: OwnedUserId = client
            .user_id()
            .expect("BUG: Client::user_id() returned None after successful login!")
            .to_owned();
        let status = format!("Logged in as {}.\n → Loading rooms...", logged_in_user_id);
        enqueue_rooms_list_update(RoomsListUpdate::Status { status });

        // Store this active client in our global Client state so that other tasks can access it.
        if let Some(_existing) = CLIENT.lock().unwrap().replace(client.clone()) {
            error!(
                "BUG: unexpectedly replaced an existing client when initializing the matrix client."
            );
        }

        // Track all async tasks so we can nicely clean them up with abort+await.
        // Generally anything that holds a reference to `Client` should be here.
        let mut subscriber_task_handles: Vec<JoinHandle<()>> = Vec::new();

        // Listen for changes to our verification status and incoming verification requests.
        subscriber_task_handles.push(add_verification_event_handlers_and_sync_client(
            client.clone(),
        ));

        // Listen for updates to the ignored user list.
        subscriber_task_handles.push(handle_ignore_user_list_subscriber(client.clone()));

        // Listen for session changes, e.g., when the access token becomes invalid.
        subscriber_task_handles.push(handle_session_changes(client.clone()));

        Cx::post_action(LoginAction::Status {
            title: "Connecting".into(),
            status: "Setting up sync service...".into(),
        });
        let sync_service = match SyncService::builder(client.clone())
            .with_offline_mode()
            .build()
            .await
        {
            Ok(ss) => ss,
            Err(e) => {
                error!("Failed to create SyncService: {e:?}");
                let err_msg = if is_invalid_token_error(&e) {
                    "Your login token is no longer valid.\n\nPlease log in again.".to_string()
                } else {
                    format!(
                        "Please restart Hepta Native.\n\nFailed to create Matrix sync service: {e}."
                    )
                };
                Cx::post_action(LoginAction::LoginFailure(err_msg.clone()));
                enqueue_popup_notification(err_msg.clone(), PopupKind::Error, None);
                enqueue_rooms_list_update(RoomsListUpdate::Status { status: err_msg });
                // Clear the stored client so the next login attempt doesn't trigger the
                // "unexpectedly replaced an existing client" warning.
                let _ = CLIENT.lock().unwrap().take();
                abort_and_await_handles(&mut subscriber_task_handles).await;
                continue 'login_loop;
            }
        };

        // Signal login success now that SyncService::build() has already succeeded,
        // which is the only step that can fail with an invalid/expired token.
        // Doing this before sync_service.start() lets the UI transition to the home screen
        // without waiting for the sync loop to begin.
        TOKEN_EXPIRED.store(false, Ordering::Release);
        Cx::post_action(LoginAction::LoginSuccess);

        // Attempt to load the previously-saved app state.
        // One-shot, drops on its own; not tracked.
        handle_load_app_state(logged_in_user_id.to_owned());
        subscriber_task_handles.push(handle_sync_indicator_subscriber(&sync_service));
        subscriber_task_handles.push(handle_sync_service_state_subscriber(sync_service.state()));

        let room_list_service = sync_service.room_list_service();
        let sync_service = Arc::new(sync_service);

        if let Some(_existing) = SYNC_SERVICE.lock().unwrap().replace(sync_service) {
            error!(
                "BUG: unexpectedly replaced an existing sync service when initializing the matrix client."
            );
        }
        apply_sync_service_desired_state("initial Matrix sync startup").await;

        let mut room_list_service_task = rt.spawn(room_list_service_loop(room_list_service));
        let mut space_service_task = rt.spawn(space_service_loop(client));

        // Now, this task becomes an infinite loop that monitors the state of the
        // three core matrix-related background tasks that we just spawned above.
        #[allow(clippy::never_loop)] // unsure if needed, just following tokio's examples.
        loop {
            tokio::select! {
                // If we were notified but it got canceled, check the TOKEN_EXPIRED bool.
                _ = TOKEN_EXPIRED_NOTIFY.notified() => {
                    if !TOKEN_EXPIRED.load(Ordering::Acquire) {
                        continue;
                    }
                    break;
                }
                _ = LOGOUT_NOTIFY.notified() => {
                    if !is_logout_in_progress() {
                        continue;
                    }
                    log!("Login loop received logout signal");
                    break;
                }
                result = &mut matrix_worker_task_handle => {
                    match result {
                        Ok(Ok(())) => {
                            // Check if this is due to logout
                            if is_logout_in_progress() {
                                log!("matrix worker task ended due to logout");
                            } else {
                                error!("BUG: matrix worker task ended unexpectedly!");
                            }
                        }
                        Ok(Err(e)) => {
                            // Check if this is due to logout
                            if is_logout_in_progress() {
                                log!("matrix worker task ended with error due to logout: {e:?}");
                            } else {
                                error!("Error: matrix worker task ended:\n\t{e:?}");
                                rooms_list::enqueue_rooms_list_update(RoomsListUpdate::Status {
                                    status: e.to_string(),
                                });
                                enqueue_popup_notification(
                                    format!("Rooms list update error: {e}"),
                                    PopupKind::Error,
                                    None,
                                );
                            }
                        },
                        Err(e) => {
                            error!("BUG: failed to join matrix worker task: {e:?}");
                        }
                    }
                    break;
                }
                result = &mut room_list_service_task => {
                    match result {
                        Ok(Ok(())) => {
                            error!("BUG: room list service loop task ended unexpectedly!");
                        }
                        Ok(Err(e)) => {
                            error!("Error: room list service loop task ended:\n\t{e:?}");
                            rooms_list::enqueue_rooms_list_update(RoomsListUpdate::Status {
                                status: e.to_string(),
                            });
                            enqueue_popup_notification(
                                format!("Room list service  error: {e}"),
                                PopupKind::Error,
                                None,
                            );
                        },
                        Err(e) => {
                            error!("BUG: failed to join room list service loop task: {e:?}");
                        }
                    }
                    break;
                }
                result = &mut space_service_task => {
                    match result {
                        Ok(Ok(())) => {
                            error!("BUG: space service loop task ended unexpectedly!");
                        }
                        Ok(Err(e)) => {
                            error!("Error: space service loop task ended:\n\t{e:?}");
                            rooms_list::enqueue_rooms_list_update(RoomsListUpdate::Status {
                                status: e.to_string(),
                            });
                            enqueue_popup_notification(
                                format!("Space service error: {e}"),
                                PopupKind::Error,
                                None,
                            );
                        },
                        Err(e) => {
                            error!("BUG: failed to join space service loop task: {e:?}");
                        }
                    }
                    break;
                }
            }
        }

        let was_token_expired = TOKEN_EXPIRED.load(Ordering::Acquire);
        let was_logout = is_logout_in_progress();
        if was_token_expired || was_logout {
            if was_token_expired {
                log!("Token expired; cleaning up session state and waiting for re-login.");
            } else {
                log!("Logout in progress; cleaning up session state and waiting for re-login.");
            }
            // `is_finished()` skips handles already consumed by the select!
            // above; awaiting them again would block forever.
            room_list_service_task.abort();
            space_service_task.abort();
            for h in &subscriber_task_handles {
                h.abort();
            }
            if !room_list_service_task.is_finished() {
                let _ = room_list_service_task.await;
            }
            if !space_service_task.is_finished() {
                let _ = space_service_task.await;
            }
            for h in subscriber_task_handles.drain(..) {
                if !h.is_finished() {
                    let _ = h.await;
                }
            }
            // No-ops if `clear_app_state` already cleared these.
            let _ = CLIENT.lock().unwrap().take();
            let _ = SYNC_SERVICE.lock().unwrap().take();
            SYNC_SERVICE_ASSUMED_RUNNING.store(false, Ordering::Release);
            continue 'login_loop;
        }
        // Unexpected break (e.g. matrix_worker_task panicked).
        room_list_service_task.abort();
        space_service_task.abort();
        for h in &subscriber_task_handles {
            h.abort();
        }
        if !room_list_service_task.is_finished() {
            let _ = room_list_service_task.await;
        }
        if !space_service_task.is_finished() {
            let _ = space_service_task.await;
        }
        for h in subscriber_task_handles.drain(..) {
            if !h.is_finished() {
                let _ = h.await;
            }
        }
        return;
    }
}

/// The main async task that listens for changes to all rooms.
async fn room_list_service_loop(room_list_service: Arc<RoomListService>) -> Result<()> {
    let all_rooms_list = room_list_service.all_rooms().await?;
    handle_room_list_service_loading_state(all_rooms_list.loading_state());

    let (room_diff_stream, room_list_dynamic_entries_controller) =
        // TODO: paginate room list to avoid loading all rooms at once
        all_rooms_list.entries_with_dynamic_adapters(usize::MAX);

    // By default, our rooms list should only show rooms that are:
    // 1. not spaces (those are handled by the SpaceService),
    // 2. not left (clients don't typically show rooms that the user has already left),
    // 3. not outdated (don't show tombstoned rooms whose successor is already joined).
    room_list_dynamic_entries_controller.set_filter(Box::new(filters::new_filter_all(vec![
        Box::new(filters::new_filter_not(Box::new(
            filters::new_filter_space(),
        ))),
        Box::new(filters::new_filter_non_left()),
        Box::new(filters::new_filter_deduplicate_versions()),
    ])));

    let mut all_known_rooms: Vector<RoomListServiceRoomInfo> = Vector::new();
    let current_user_id = current_user_id();

    pin_mut!(room_diff_stream);
    while let Some(batch) = room_diff_stream.next().await {
        let mut peekable_diffs = batch.into_iter().peekable();
        while let Some(diff) = peekable_diffs.next() {
            let is_reset = matches!(diff, VectorDiff::Reset { .. });
            match diff {
                VectorDiff::Append { values: new_rooms }
                | VectorDiff::Reset { values: new_rooms } => {
                    // Append and Reset are identical, except for Reset first clears all rooms.
                    let _num_new_rooms = new_rooms.len();
                    if is_reset {
                        if LOG_ROOM_LIST_DIFFS {
                            log!(
                                "room_list: diff Reset, old length {}, new length {}",
                                all_known_rooms.len(),
                                new_rooms.len()
                            );
                        }
                        // Iterate manually so we can know which rooms are being removed.
                        while let Some(room) = all_known_rooms.pop_back() {
                            remove_room(&room);
                        }
                        // ALL_JOINED_ROOMS should already be empty due to successive calls to `remove_room()`,
                        // so this is just a sanity check.
                        ALL_JOINED_ROOMS.lock().unwrap().clear();
                        enqueue_rooms_list_update(RoomsListUpdate::ClearRooms);
                        enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(VecDiff::Clear));
                    } else {
                        if LOG_ROOM_LIST_DIFFS {
                            log!(
                                "room_list: diff Append, old length {}, adding {} new items",
                                all_known_rooms.len(),
                                _num_new_rooms
                            );
                        }
                    }

                    // Parallelize creating each room's RoomListServiceRoomInfo and adding that new room.
                    // We combine `from_room` and `add_new_room` into a single async task per room.
                    let new_room_infos: Vec<RoomListServiceRoomInfo> =
                        join_all(new_rooms.into_iter().map(|room| async {
                            let room_info = RoomListServiceRoomInfo::from_room(
                                room.into_inner(),
                                &current_user_id,
                            )
                            .await;
                            if let Err(e) =
                                add_new_room(&room_info, &room_list_service, false).await
                            {
                                error!(
                                    "Failed to add new room: {:?} ({}); error: {:?}",
                                    room_info.display_name, room_info.room_id, e
                                );
                            }
                            room_info
                        }))
                        .await;

                    // Send room order update with the new room IDs
                    let (room_id_refs, room_ids) = {
                        let mut room_id_refs = Vec::with_capacity(new_room_infos.len());
                        let mut room_ids = Vec::with_capacity(new_room_infos.len());
                        for r in &new_room_infos {
                            room_id_refs.push(r.room_id.as_ref());
                            room_ids.push(r.room_id.clone());
                        }
                        (room_id_refs, room_ids)
                    };
                    if !room_ids.is_empty() {
                        enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(
                            VecDiff::Append { values: room_ids },
                        ));
                        room_list_service.subscribe_to_rooms(&room_id_refs).await;
                        all_known_rooms.extend(new_room_infos);
                    }
                }
                VectorDiff::Clear => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff Clear");
                    }
                    all_known_rooms.clear();
                    ALL_JOINED_ROOMS.lock().unwrap().clear();
                    enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(VecDiff::Clear));
                    enqueue_rooms_list_update(RoomsListUpdate::ClearRooms);
                }
                VectorDiff::PushFront { value: new_room } => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff PushFront");
                    }
                    let new_room =
                        RoomListServiceRoomInfo::from_room(new_room.into_inner(), &current_user_id)
                            .await;
                    let room_id = new_room.room_id.clone();
                    add_new_room(&new_room, &room_list_service, true).await?;
                    enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(
                        VecDiff::PushFront { value: room_id },
                    ));
                    all_known_rooms.push_front(new_room);
                }
                VectorDiff::PushBack { value: new_room } => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff PushBack");
                    }
                    let new_room =
                        RoomListServiceRoomInfo::from_room(new_room.into_inner(), &current_user_id)
                            .await;
                    let room_id = new_room.room_id.clone();
                    add_new_room(&new_room, &room_list_service, true).await?;
                    enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(
                        VecDiff::PushBack { value: room_id },
                    ));
                    all_known_rooms.push_back(new_room);
                }
                remove_diff @ VectorDiff::PopFront => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff PopFront");
                    }
                    if let Some(room) = all_known_rooms.pop_front() {
                        enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(
                            VecDiff::PopFront,
                        ));
                        optimize_remove_then_add_into_update(
                            remove_diff,
                            &room,
                            &mut peekable_diffs,
                            &mut all_known_rooms,
                            &room_list_service,
                            &current_user_id,
                        )
                        .await?;
                    }
                }
                remove_diff @ VectorDiff::PopBack => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff PopBack");
                    }
                    if let Some(room) = all_known_rooms.pop_back() {
                        enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(
                            VecDiff::PopBack,
                        ));
                        optimize_remove_then_add_into_update(
                            remove_diff,
                            &room,
                            &mut peekable_diffs,
                            &mut all_known_rooms,
                            &room_list_service,
                            &current_user_id,
                        )
                        .await?;
                    }
                }
                VectorDiff::Insert {
                    index,
                    value: new_room,
                } => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff Insert at {index}");
                    }
                    let new_room =
                        RoomListServiceRoomInfo::from_room(new_room.into_inner(), &current_user_id)
                            .await;
                    let room_id = new_room.room_id.clone();
                    add_new_room(&new_room, &room_list_service, true).await?;
                    enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(VecDiff::Insert {
                        index,
                        value: room_id,
                    }));
                    all_known_rooms.insert(index, new_room);
                }
                VectorDiff::Set {
                    index,
                    value: changed_room,
                } => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff Set at {index}");
                    }
                    let changed_room = RoomListServiceRoomInfo::from_room(
                        changed_room.into_inner(),
                        &current_user_id,
                    )
                    .await;
                    if let Some(old_room) = all_known_rooms.get(index) {
                        update_room(old_room, &changed_room, &room_list_service).await?;
                    } else {
                        error!("BUG: room list diff: Set index {index} was out of bounds.");
                    }
                    // Send order update (room ID at this index may have changed)
                    enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(VecDiff::Set {
                        index,
                        value: changed_room.room_id.clone(),
                    }));
                    all_known_rooms.set(index, changed_room);
                }
                remove_diff @ VectorDiff::Remove { index } => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff Remove at {index}");
                    }
                    if index < all_known_rooms.len() {
                        let room = all_known_rooms.remove(index);
                        enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(
                            VecDiff::Remove { index },
                        ));
                        optimize_remove_then_add_into_update(
                            remove_diff,
                            &room,
                            &mut peekable_diffs,
                            &mut all_known_rooms,
                            &room_list_service,
                            &current_user_id,
                        )
                        .await?;
                    } else {
                        error!(
                            "BUG: room_list: diff Remove index {index} out of bounds, len {}",
                            all_known_rooms.len()
                        );
                    }
                }
                VectorDiff::Truncate { length } => {
                    if LOG_ROOM_LIST_DIFFS {
                        log!("room_list: diff Truncate to {length}");
                    }
                    // Iterate manually so we can know which rooms are being removed.
                    while all_known_rooms.len() > length {
                        if let Some(room) = all_known_rooms.pop_back() {
                            remove_room(&room);
                        }
                    }
                    all_known_rooms.truncate(length); // sanity check
                    enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(
                        VecDiff::Truncate { length },
                    ));
                }
            }
        }
    }

    bail!("room list service sync loop ended unexpectedly")
}

/// Attempts to optimize a common RoomListService operation of remove + add.
///
/// If a `Remove` diff (or `PopBack` or `PopFront`) is immediately followed by
/// an `Insert` diff (or `PushFront` or `PushBack`) for the same room,
/// we can treat it as a simple `Set` operation, in which we call `update_room()`.
/// This is much more efficient than removing the room and then adding it back.
///
/// This tends to happen frequently in order to change the room's state
/// or to "sort" the room list by changing its positional order.
async fn optimize_remove_then_add_into_update(
    remove_diff: VectorDiff<RoomListItem>,
    room: &RoomListServiceRoomInfo,
    peekable_diffs: &mut Peekable<impl Iterator<Item = VectorDiff<RoomListItem>>>,
    all_known_rooms: &mut Vector<RoomListServiceRoomInfo>,
    room_list_service: &RoomListService,
    current_user_id: &Option<OwnedUserId>,
) -> Result<()> {
    let next_diff_was_handled: bool;
    match peekable_diffs.peek() {
        Some(VectorDiff::Insert {
            index: insert_index,
            value: new_room,
        }) if room.room_id == new_room.room_id() => {
            if LOG_ROOM_LIST_DIFFS {
                log!(
                    "Optimizing {remove_diff:?} + Insert({insert_index}) into Update for room {}",
                    room.room_id
                );
            }
            let new_room =
                RoomListServiceRoomInfo::from_room_ref(new_room.deref(), current_user_id).await;
            update_room(room, &new_room, room_list_service).await?;
            // Send order update for the insert
            enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(VecDiff::Insert {
                index: *insert_index,
                value: new_room.room_id.clone(),
            }));
            all_known_rooms.insert(*insert_index, new_room);
            next_diff_was_handled = true;
        }
        Some(VectorDiff::PushFront { value: new_room }) if room.room_id == new_room.room_id() => {
            if LOG_ROOM_LIST_DIFFS {
                log!(
                    "Optimizing {remove_diff:?} + PushFront into Update for room {}",
                    room.room_id
                );
            }
            let new_room =
                RoomListServiceRoomInfo::from_room_ref(new_room.deref(), current_user_id).await;
            update_room(room, &new_room, room_list_service).await?;
            // Send order update for the push front
            enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(VecDiff::PushFront {
                value: new_room.room_id.clone(),
            }));
            all_known_rooms.push_front(new_room);
            next_diff_was_handled = true;
        }
        Some(VectorDiff::PushBack { value: new_room }) if room.room_id == new_room.room_id() => {
            if LOG_ROOM_LIST_DIFFS {
                log!(
                    "Optimizing {remove_diff:?} + PushBack into Update for room {}",
                    room.room_id
                );
            }
            let new_room =
                RoomListServiceRoomInfo::from_room_ref(new_room.deref(), current_user_id).await;
            update_room(room, &new_room, room_list_service).await?;
            // Send order update for the push back
            enqueue_rooms_list_update(RoomsListUpdate::RoomOrderUpdate(VecDiff::PushBack {
                value: new_room.room_id.clone(),
            }));
            all_known_rooms.push_back(new_room);
            next_diff_was_handled = true;
        }
        _ => next_diff_was_handled = false,
    }
    if next_diff_was_handled {
        peekable_diffs.next(); // consume the next diff
    } else {
        remove_room(room);
    }
    Ok(())
}

/// Invoked when the room list service has received an update that changes an existing room.
async fn update_room(
    old_room: &RoomListServiceRoomInfo,
    new_room: &RoomListServiceRoomInfo,
    room_list_service: &RoomListService,
) -> Result<()> {
    let new_room_id = new_room.room_id.clone();
    if old_room.room_id == new_room_id {
        // Handle state transitions for a room.
        if LOG_ROOM_LIST_DIFFS {
            log!(
                "Room {:?} ({new_room_id}) state went from {:?} --> {:?}",
                new_room.display_name,
                old_room.state,
                new_room.state
            );
        }
        if old_room.state != new_room.state {
            match new_room.state {
                RoomState::Banned => {
                    // TODO: handle rooms that this user has been banned from.
                    log!(
                        "Removing Banned room: {:?} ({new_room_id})",
                        new_room.display_name
                    );
                    remove_room(new_room);
                    return Ok(());
                }
                RoomState::Left => {
                    log!(
                        "Removing Left room: {:?} ({new_room_id})",
                        new_room.display_name
                    );
                    // TODO: instead of removing this, we could optionally add it to
                    //       a separate list of left rooms, which would be collapsed by default.
                    //       Upon clicking a left room, we could show a splash page
                    //       that prompts the user to rejoin the room or forget it permanently.
                    //       Currently, we just remove it and do not show left rooms at all.
                    remove_room(new_room);
                    return Ok(());
                }
                RoomState::Joined => {
                    log!(
                        "update_room(): adding new Joined room: {:?} ({new_room_id})",
                        new_room.display_name
                    );
                    return add_new_room(new_room, room_list_service, true).await;
                }
                RoomState::Invited => {
                    log!(
                        "update_room(): adding new Invited room: {:?} ({new_room_id})",
                        new_room.display_name
                    );
                    return add_new_room(new_room, room_list_service, true).await;
                }
                RoomState::Knocked => {
                    // TODO: handle Knocked rooms (e.g., can you re-knock? or cancel a prior knock?)
                    return Ok(());
                }
            }
        }

        // First, we check for changes to room data that is relevant to any room,
        // including joined, invited, and other rooms.
        // This includes the room name and room avatar.
        if old_room.room_avatar != new_room.room_avatar {
            log!("Updating room avatar for room {}", new_room_id);
            spawn_fetch_room_avatar(new_room);
        }
        if old_room.display_name != new_room.display_name {
            log!(
                "Updating room {} name: {:?} --> {:?}",
                new_room_id,
                old_room.display_name,
                new_room.display_name
            );

            enqueue_rooms_list_update(RoomsListUpdate::UpdateRoomName {
                new_room_name: (new_room.display_name.clone(), new_room_id.clone()).into(),
            });
        }

        // Then, we check for changes to room data that is only relevant to joined rooms:
        // including the latest event, tags, unread counts, is_direct, tombstoned state, power levels, etc.
        // Invited or left rooms don't care about these details.
        if matches!(new_room.state, RoomState::Joined) {
            // For some reason, the latest event API does not reliably catch *all* changes
            // to the latest event in a given room, such as redactions.
            // Thus, we have to re-obtain the latest event on *every* update, regardless of timestamp.
            //
            let update_latest = match (
                old_room.latest_event_timestamp,
                new_room.room.latest_event_timestamp(),
            ) {
                (Some(old_ts), Some(new_ts)) => new_ts >= old_ts,
                (None, Some(_)) => true,
                _ => false,
            };
            if update_latest {
                update_latest_event(&new_room.room).await;
            }

            if old_room.tags != new_room.tags {
                log!(
                    "Updating room {} tags from {:?} to {:?}",
                    new_room_id,
                    old_room.tags,
                    new_room.tags
                );
                enqueue_rooms_list_update(RoomsListUpdate::Tags {
                    room_id: new_room_id.clone(),
                    new_tags: new_room.tags.clone().unwrap_or_default(),
                });
            }

            if old_room.is_marked_unread != new_room.is_marked_unread
                || old_room.num_unread_messages != new_room.num_unread_messages
                || old_room.num_unread_mentions != new_room.num_unread_mentions
            {
                log!(
                    "Updating room {}, marked unread {} --> {}, unread messages {} --> {}, unread mentions {} --> {}",
                    new_room_id,
                    old_room.is_marked_unread,
                    new_room.is_marked_unread,
                    old_room.num_unread_messages,
                    new_room.num_unread_messages,
                    old_room.num_unread_mentions,
                    new_room.num_unread_mentions,
                );
                enqueue_rooms_list_update(RoomsListUpdate::UpdateNumUnreadMessages {
                    room_id: new_room_id.clone(),
                    is_marked_unread: new_room.is_marked_unread,
                    unread_messages: UnreadMessageCount::Known(new_room.num_unread_messages),
                    unread_mentions: new_room.num_unread_mentions,
                });
            }

            if old_room.is_direct != new_room.is_direct {
                log!(
                    "Updating room {} is_direct from {} to {}",
                    new_room_id,
                    old_room.is_direct,
                    new_room.is_direct,
                );
                enqueue_rooms_list_update(RoomsListUpdate::UpdateIsDirect {
                    room_id: new_room_id.clone(),
                    is_direct: new_room.is_direct,
                });
            }

            let mut __timeline_update_sender_opt = None;
            let mut get_timeline_update_sender = |room_id| {
                if __timeline_update_sender_opt.is_none() {
                    if let Some(jrd) = ALL_JOINED_ROOMS.lock().unwrap().get(room_id) {
                        __timeline_update_sender_opt =
                            Some(jrd.main_timeline.timeline_update_sender.clone());
                    }
                }
                __timeline_update_sender_opt.clone()
            };

            if !old_room.is_tombstoned && new_room.is_tombstoned {
                let successor_room = new_room.room.successor_room();
                log!("Updating room {new_room_id} to be tombstoned, {successor_room:?}");
                enqueue_rooms_list_update(RoomsListUpdate::TombstonedRoom {
                    room_id: new_room_id.clone(),
                });
                if let Some(timeline_update_sender) = get_timeline_update_sender(&new_room_id) {
                    spawn_fetch_successor_room_preview(
                        room_list_service.client().clone(),
                        successor_room,
                        new_room_id.clone(),
                        timeline_update_sender,
                    );
                } else {
                    error!(
                        "BUG: could not find JoinedRoomDetails for newly-tombstoned room {new_room_id}"
                    );
                }
            }

            if let Some(nupl) = new_room.user_power_levels
                && old_room.user_power_levels.is_none_or(|oupl| oupl != nupl)
            {
                if let Some(timeline_update_sender) = get_timeline_update_sender(&new_room_id) {
                    log!("Updating room {new_room_id} user power levels.");
                    match timeline_update_sender.send(TimelineUpdate::UserPowerLevels(nupl)) {
                        Ok(_) => SignalToUI::set_ui_signal(),
                        Err(_) => error!(
                            "Failed to send the UserPowerLevels update to room {new_room_id}"
                        ),
                    }
                } else {
                    error!(
                        "BUG: could not find JoinedRoomDetails for room {new_room_id} where power levels changed."
                    );
                }
            }
        }
        Ok(())
    } else {
        warning!(
            "UNTESTED SCENARIO: update_room(): removing old room {}, replacing with new room {}",
            old_room.room_id,
            new_room_id,
        );
        remove_room(old_room);
        add_new_room(new_room, room_list_service, true).await
    }
}

/// Invoked when the room list service has received an update to remove an existing room.
fn remove_room(room: &RoomListServiceRoomInfo) {
    ALL_JOINED_ROOMS.lock().unwrap().remove(&room.room_id);
    enqueue_rooms_list_update(RoomsListUpdate::RemoveRoom {
        room_id: room.room_id.clone(),
        new_state: room.state,
    });
}

/// Invoked when the room list service has received an update with a brand new room.
async fn add_new_room(
    new_room: &RoomListServiceRoomInfo,
    room_list_service: &RoomListService,
    subscribe: bool,
) -> Result<()> {
    match new_room.state {
        RoomState::Knocked => {
            log!(
                "Got new Knocked room: {:?} ({})",
                new_room.display_name,
                new_room.room_id
            );
            // Note: here we could optionally display Knocked rooms as a separate type of room
            //       in the rooms list, but it's not really necessary at this point.
            return Ok(());
        }
        RoomState::Banned => {
            log!(
                "Got new Banned room: {:?} ({})",
                new_room.display_name,
                new_room.room_id
            );
            // Note: here we could optionally display Banned rooms as a separate type of room
            //       in the rooms list, but it's not really necessary at this point.
            return Ok(());
        }
        RoomState::Left => {
            log!(
                "Got new Left room: {:?} ({:?})",
                new_room.display_name,
                new_room.room_id
            );
            // Note: here we could optionally display Left rooms as a separate type of room
            //       in the rooms list, but it's not really necessary at this point.
            return Ok(());
        }
        RoomState::Invited => {
            let invite_details = new_room.room.invite_details().await.ok();
            let room_name_id =
                RoomNameId::from((new_room.display_name.clone(), new_room.room_id.clone()));
            // Start with a basic text avatar; the avatar image will be fetched asynchronously below.
            let room_avatar = avatar_from_room_name(room_name_id.name_for_avatar());
            let inviter_info = if let Some(inviter) = invite_details.and_then(|d| d.inviter) {
                Some(InviterInfo {
                    user_id: inviter.user_id().to_owned(),
                    display_name: inviter.display_name().map(|n| n.to_string()),
                    avatar: inviter
                        .avatar(AVATAR_THUMBNAIL_FORMAT.into())
                        .await
                        .ok()
                        .flatten()
                        .map(Into::into),
                })
            } else {
                None
            };
            rooms_list::enqueue_rooms_list_update(RoomsListUpdate::AddInvitedRoom(
                InvitedRoomInfo {
                    room_name_id: room_name_id.clone(),
                    inviter_info,
                    room_avatar,
                    canonical_alias: new_room.room.canonical_alias(),
                    alt_aliases: new_room.room.alt_aliases(),
                    // we don't actually display the latest event for Invited rooms, so don't bother.
                    latest: None,
                    invite_state: Default::default(),
                    is_selected: false,
                    is_direct: new_room.is_direct,
                },
            ));
            Cx::post_action(AppStateAction::RoomLoadedSuccessfully {
                room_name_id,
                is_invite: true,
            });
            spawn_fetch_room_avatar(new_room);
            return Ok(());
        }
        RoomState::Joined => {} // Fall through to adding the joined room below.
    }

    // If we didn't already subscribe to this room, do so now.
    // This ensures we will properly receive all of its states and latest event.
    if subscribe {
        room_list_service
            .subscribe_to_rooms(&[&new_room.room_id])
            .await;
    }

    let timeline = Arc::new(
        new_room
            .room
            .timeline_builder()
            .with_focus(TimelineFocus::Live {
                // we show threads as separate timelines in their own RoomScreen
                hide_threaded_events: true,
            })
            .track_read_marker_and_receipts(TimelineReadReceiptTracking::AllEvents)
            .build()
            .await
            .map_err(|e| {
                anyhow::anyhow!(
                    "BUG: Failed to build timeline for room {}: {e}",
                    new_room.room_id
                )
            })?,
    );
    let (timeline_update_sender, timeline_update_receiver) = crossbeam_channel::unbounded();

    let (request_sender, request_receiver) = watch::channel(Vec::new());
    let timeline_subscriber_handler_task = Handle::current().spawn(timeline_subscriber_handler(
        new_room.room.clone(),
        timeline.clone(),
        timeline_update_sender.clone(),
        request_receiver,
        None,
    ));

    // We need to add the room to the `ALL_JOINED_ROOMS` list before we can send
    // an `AddJoinedRoom` update to the RoomsList widget, because that widget might
    // immediately issue a `MatrixRequest` that relies on that room being in `ALL_JOINED_ROOMS`.
    log!(
        "Adding new joined room {}, name: {:?}",
        new_room.room_id,
        new_room.display_name
    );
    ALL_JOINED_ROOMS.lock().unwrap().insert(
        new_room.room_id.clone(),
        JoinedRoomDetails {
            room_id: new_room.room_id.clone(),
            main_timeline: PerTimelineDetails {
                timeline,
                timeline_singleton_endpoints: Some((timeline_update_receiver, request_sender)),
                timeline_update_sender,
                timeline_subscriber_handler_task,
            },
            thread_timelines: HashMap::new(),
            pending_thread_timelines: HashSet::new(),
            typing_notice_subscriber: None,
            pinned_events_subscriber: None,
        },
    );

    let latest = get_latest_event_details(
        &new_room.room.latest_event().await,
        room_list_service.client(),
    )
    .await;
    let room_name_id = RoomNameId::from((new_room.display_name.clone(), new_room.room_id.clone()));
    // Start with a basic text avatar; the avatar image will be fetched asynchronously below.
    let room_avatar = avatar_from_room_name(room_name_id.name_for_avatar());
    rooms_list::enqueue_rooms_list_update(RoomsListUpdate::AddJoinedRoom(JoinedRoomInfo {
        latest,
        tags: new_room.tags.clone().unwrap_or_default(),
        num_unread_messages: new_room.num_unread_messages,
        num_unread_mentions: new_room.num_unread_mentions,
        is_marked_unread: new_room.is_marked_unread,
        room_avatar,
        room_name_id: room_name_id.clone(),
        canonical_alias: new_room.room.canonical_alias(),
        alt_aliases: new_room.room.alt_aliases(),
        has_been_paginated: false,
        is_selected: false,
        is_direct: new_room.is_direct,
        is_tombstoned: new_room.is_tombstoned,
    }));

    Cx::post_action(AppStateAction::RoomLoadedSuccessfully {
        room_name_id,
        is_invite: false,
    });
    spawn_fetch_room_avatar(new_room);
    Ok(())
}

#[allow(unused)]
async fn current_ignore_user_list(client: &Client) -> Option<HashSet<OwnedUserId>> {
    use matrix_sdk::ruma::events::ignored_user_list::IgnoredUserListEventContent;
    let ignored_users = client
        .account()
        .account_data::<IgnoredUserListEventContent>()
        .await
        .ok()??
        .deserialize()
        .ok()?
        .ignored_users
        .into_keys()
        .collect();

    Some(ignored_users)
}

/// This function spawns a task that captures a strong `Client` ref,
/// so the caller should abort+await it upon logout to ensure the Client gets dropped.
fn handle_ignore_user_list_subscriber(client: Client) -> JoinHandle<()> {
    let mut subscriber = client.subscribe_to_ignore_user_list_changes();
    log!("Initial ignored-user list is: {:?}", subscriber.get());
    Handle::current().spawn(async move {
        let mut first_update = true;
        while let Some(ignore_list) = subscriber.next().await {
            log!("Received an updated ignored-user list: {ignore_list:?}");
            let ignored_users_new = ignore_list
                .into_iter()
                .filter_map(|u| OwnedUserId::try_from(u).ok())
                .collect::<HashSet<_, ConstHasher>>();

            // TODO: when we support persistent state, don't forget to update `IGNORED_USERS` upon app boot.
            let mut ignored_users_old = IGNORED_USERS.lock().unwrap();
            let has_changed = *ignored_users_old != ignored_users_new;
            *ignored_users_old = ignored_users_new;

            if has_changed && !first_update {
                // After successfully (un)ignoring a user, all timelines are fully cleared by the Matrix SDK.
                // Therefore, we need to re-fetch all timelines for all rooms,
                // and currently the only way to actually accomplish this is via pagination.
                // See: <https://github.com/matrix-org/matrix-rust-sdk/issues/1703#issuecomment-2250297923>
                for joined_room in client.joined_rooms() {
                    submit_async_request(MatrixRequest::PaginateTimeline {
                        timeline_kind: TimelineKind::MainRoom {
                            room_id: joined_room.room_id().to_owned(),
                        },
                        num_events: 50,
                        direction: PaginationDirection::Backwards,
                    });
                }
            }

            first_update = false;
        }
    })
}

/// Asynchronously loads and restores the app state from persistent storage for the given user.
///
/// When a saved state file is found, this emits a `RestoreAppStateFromPersistentState` action
/// so that the app can restore preferences and the dock layout (on desktop).
/// We emit this action even if the dock state is empty to ensure that prefs always get restored.
fn handle_load_app_state(user_id: OwnedUserId) {
    Handle::current().spawn(async move {
        match load_app_state(&user_id).await {
            Ok(Some(app_state)) => {
                log!("Loaded app state from persistent storage. Restoring now...");
                Cx::post_action(AppStateAction::RestoreAppStateFromPersistentState(
                    app_state,
                ));
            }
            Ok(None) => {
                // No saved file (fresh install) or file was unreadable; nothing to restore.
            }
            Err(_e) => {
                log!("Failed to restore app state from persistent storage: {_e}");
                enqueue_popup_notification(
                    "Could not restore the previous session's app state.",
                    PopupKind::Warning,
                    None,
                );
            }
        }
    });
}

/// Returns `true` if the given sync service error is due to an invalid/expired access token.
fn is_invalid_token_error(e: &sync_service::Error) -> bool {
    use matrix_sdk::ruma::api::client::error::ErrorKind;
    let sdk_error = match e {
        sync_service::Error::RoomList(matrix_sdk_ui::room_list_service::Error::SlidingSync(
            err,
        )) => err,
        sync_service::Error::EncryptionSync(encryption_sync_service::Error::SlidingSync(err)) => {
            err
        }
        _ => return false,
    };
    matches!(
        sdk_error.client_api_error_kind(),
        Some(ErrorKind::UnknownToken { .. } | ErrorKind::MissingToken)
    )
}

/// Subscribes to session change notifications from the Matrix client.
///
/// When the homeserver rejects the access token with a 401 `M_UNKNOWN_TOKEN` error
/// (e.g., the token was revoked or expired), this emits a [`LoginAction::LoginFailure`]
/// so the user is prompted to log in again.
fn handle_session_changes(client: Client) -> JoinHandle<()> {
    let mut receiver = client.subscribe_to_session_changes();
    Handle::current().spawn(async move {
        loop {
            match receiver.recv().await {
                Ok(SessionChange::UnknownToken(data)) => {
                    let soft_logout = data.soft_logout;
                    let msg = if soft_logout {
                        "Your login session has expired.\n\nPlease log in again."
                    } else {
                        "Your login token is no longer valid.\n\nPlease log in again."
                    };
                    error!("Session token is no longer valid (soft_logout: {soft_logout}). Prompting re-login.");
                    TOKEN_EXPIRED.store(true, Ordering::Release);
                    TOKEN_EXPIRED_NOTIFY.notify_one();
                    Cx::post_action(LoginAction::LoginFailure(msg.to_string()));
                    // Only prompt once — the SDK will keep emitting UnknownToken
                    // for every rejected request, but one re-login prompt suffices.
                    break;
                }
                Ok(SessionChange::TokensRefreshed) => {}
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    warning!("Session change receiver lagged, missed {n} messages.");
                }
                Err(broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    })
}

fn handle_sync_service_state_subscriber(
    mut subscriber: Subscriber<sync_service::State>,
) -> JoinHandle<()> {
    log!("Initial sync service state is {:?}", subscriber.get());
    Handle::current().spawn(async move {
        while let Some(state) = subscriber.next().await {
            log!("Received a sync service state update: {state:?}");
            match state {
                sync_service::State::Error(e) => {
                    SYNC_SERVICE_ASSUMED_RUNNING.store(false, Ordering::Release);
                    if is_invalid_token_error(&e) {
                        // The access token is invalid; `handle_session_changes` will have
                        // already posted a LoginAction::LoginFailure, so just log here.
                        // Stop the sync service and exit this loop to prevent further
                        // state transitions (e.g., Offline) from triggering misleading
                        // "cannot reach homeserver" notifications.
                        // Setting TOKEN_EXPIRED signals the main monitoring loop to
                        // tear down the current session and wait for re-login.
                        error!("Sync service stopped due to invalid/expired access token: {e}.");
                        TOKEN_EXPIRED.store(true, Ordering::Release);
                        TOKEN_EXPIRED_NOTIFY.notify_one();
                        if let Some(ss) = get_sync_service() {
                            ss.stop().await;
                            SYNC_SERVICE_ASSUMED_RUNNING.store(false, Ordering::Release);
                        }
                        break;
                    } else {
                        if !sync_service_desired_running() {
                            log!("Not restarting sync service after error because lifecycle currently wants it stopped: {e}.");
                            continue;
                        }
                        log!("Restarting sync service due to error: {e}.");
                        if get_sync_service().is_some() {
                            apply_sync_service_desired_state("sync service error restart").await;
                        } else {
                            enqueue_popup_notification(
                                "Unable to restart the Matrix sync service.\n\nPlease quit and restart Hepta Native.",
                                PopupKind::Error,
                                None,
                            );
                        }
                    }
                }
                _other if TOKEN_EXPIRED.load(Ordering::Acquire) => {
                    log!("Ignoring sync service state update after token expiration.");
                    break;
                }
                other => Cx::post_action(RoomsListHeaderAction::StateUpdate(other)),
            }
        }
    })
}

fn handle_sync_indicator_subscriber(sync_service: &SyncService) -> JoinHandle<()> {
    /// Duration for sync indicator delay before showing
    const SYNC_INDICATOR_DELAY: Duration = Duration::from_millis(100);
    /// Duration for sync indicator delay before hiding
    const SYNC_INDICATOR_HIDE_DELAY: Duration = Duration::from_millis(200);
    let sync_indicator_stream = sync_service
        .room_list_service()
        .sync_indicator(SYNC_INDICATOR_DELAY, SYNC_INDICATOR_HIDE_DELAY);

    Handle::current().spawn(async move {
        let mut sync_indicator_stream = std::pin::pin!(sync_indicator_stream);

        while let Some(indicator) = sync_indicator_stream.next().await {
            let is_syncing = match indicator {
                SyncIndicator::Show => true,
                SyncIndicator::Hide => false,
            };
            Cx::post_action(RoomsListHeaderAction::SetSyncStatus(is_syncing));
        }
    })
}

fn handle_room_list_service_loading_state(mut loading_state: Subscriber<RoomListLoadingState>) {
    log!(
        "Initial room list loading state is {:?}",
        loading_state.get()
    );
    Handle::current().spawn(async move {
        while let Some(state) = loading_state.next().await {
            log!("Received a room list loading state update: {state:?}");
            match state {
                RoomListLoadingState::NotLoaded => {
                    enqueue_rooms_list_update(RoomsListUpdate::NotLoaded);
                }
                RoomListLoadingState::Loaded {
                    maximum_number_of_rooms,
                } => {
                    enqueue_rooms_list_update(RoomsListUpdate::LoadedRooms {
                        max_rooms: maximum_number_of_rooms,
                    });
                    // The SDK docs state that we cannot move from the `Loaded` state
                    // back to the `NotLoaded` state, so we can safely exit this task here.
                    return;
                }
            }
        }
    });
}

/// Spawns an async task to fetch the RoomPreview for the given successor room.
///
/// After the fetch completes, this emites a [`RoomPreviewAction`]
/// containing the fetched room preview or an error if it failed.
fn spawn_fetch_successor_room_preview(
    client: Client,
    successor_room: Option<SuccessorRoom>,
    tombstoned_room_id: OwnedRoomId,
    timeline_update_sender: crossbeam_channel::Sender<TimelineUpdate>,
) {
    Handle::current().spawn(async move {
        log!("Updating room {tombstoned_room_id} to be tombstoned, {successor_room:?}");
        let srd = if let Some(SuccessorRoom { room_id, reason }) = successor_room {
            match fetch_room_preview_with_avatar(&client, room_id.deref().into(), Vec::new()).await
            {
                Ok(room_preview) => SuccessorRoomDetails::Full {
                    room_preview,
                    reason,
                },
                Err(e) => {
                    log!("Failed to fetch preview of successor room {room_id}, error: {e:?}");
                    SuccessorRoomDetails::Basic(SuccessorRoom { room_id, reason })
                }
            }
        } else {
            log!("BUG: room {tombstoned_room_id} was tombstoned but had no successor room!");
            SuccessorRoomDetails::None
        };

        match timeline_update_sender.send(TimelineUpdate::Tombstoned(srd)) {
            Ok(_) => SignalToUI::set_ui_signal(),
            Err(_) => error!("Failed to send the Tombstoned update to room {tombstoned_room_id}"),
        }
    });
}

/// Fetches the full preview information for the given `room`.
/// Also fetches that room preview's avatar, if it had an avatar URL.
async fn fetch_room_preview_with_avatar(
    client: &Client,
    room: &RoomOrAliasId,
    via: Vec<OwnedServerName>,
) -> Result<FetchedRoomPreview, matrix_sdk::Error> {
    let room_preview = client.get_room_preview(room, via).await?;
    // If this room has an avatar URL, fetch it.
    let room_avatar = if let Some(avatar_url) = room_preview.avatar_url.clone() {
        let media_request = MediaRequestParameters {
            source: MediaSource::Plain(avatar_url),
            format: AVATAR_THUMBNAIL_FORMAT.into(),
        };
        match client.media().get_media_content(&media_request, true).await {
            Ok(avatar_content) => {
                log!(
                    "Fetched avatar for room preview {:?} ({})",
                    room_preview.name,
                    room_preview.room_id
                );
                FetchedRoomAvatar::Image(avatar_content.into())
            }
            Err(e) => {
                log!(
                    "Failed to fetch avatar for room preview {:?} ({}), error: {e:?}",
                    room_preview.name,
                    room_preview.room_id
                );
                avatar_from_room_name(room_preview.name.as_deref())
            }
        }
    } else {
        // The successor room did not have an avatar URL
        avatar_from_room_name(room_preview.name.as_deref())
    };
    Ok(FetchedRoomPreview::from(room_preview, room_avatar))
}

/// Fetches key details about the given thread root event.
///
/// Returns a tuple of:
/// 1. the number of replies in the thread (excluding the root event itself),
/// 2. the latest reply event, if it could be fetched.
async fn fetch_thread_summary_details(
    room: &Room,
    thread_root_event_id: &EventId,
) -> (
    u32,
    Option<matrix_sdk::deserialized_responses::TimelineEvent>,
) {
    let mut num_replies = 0;
    let mut latest_reply_event = None;

    if let Ok(thread_root_event) = room.load_or_fetch_event(thread_root_event_id, None).await
        && let Some(thread_summary) = thread_root_event.thread_summary.summary()
    {
        num_replies = thread_summary.num_replies;
        if let Some(latest_reply_event_id) = thread_summary.latest_reply.as_ref()
            && let Ok(latest_reply) = room.load_or_fetch_event(latest_reply_event_id, None).await
        {
            latest_reply_event = Some(latest_reply);
        }
    }

    // Always compute the reply count directly from the fetched thread relations,
    // for some reason we can't rely on the SDK-provided thread_summary to be accurate
    // (it's almost always totally wrong or out-of-date...).
    let count_replies_future = count_thread_replies(room, thread_root_event_id);

    // Fetch the latest reply event and count the thread replies in parallel.
    let (fetched_latest_reply_opt, reply_count_opt) = if latest_reply_event.is_none() {
        tokio::join!(
            fetch_latest_thread_reply_event(room, thread_root_event_id),
            count_replies_future,
        )
    } else {
        (None, count_replies_future.await)
    };

    if let Some(event) = fetched_latest_reply_opt {
        latest_reply_event = Some(event);
    }
    if let Some(count) = reply_count_opt {
        num_replies = count;
    }
    (num_replies, latest_reply_event)
}

/// Fetches the latest reply event in the thread rooted at `thread_root_event_id`.
async fn fetch_latest_thread_reply_event(
    room: &Room,
    thread_root_event_id: &EventId,
) -> Option<matrix_sdk::deserialized_responses::TimelineEvent> {
    let options = RelationsOptions {
        dir: Direction::Backward,
        limit: Some(uint!(1)),
        include_relations: IncludeRelations::RelationsOfType(RelationType::Thread),
        ..Default::default()
    };

    room.relations(thread_root_event_id.to_owned(), options)
        .await
        .ok()
        .and_then(|relations| relations.chunk.into_iter().next())
}

/// Counts all replies in the given thread by paginating `/relations` in batches.
async fn count_thread_replies(room: &Room, thread_root_event_id: &EventId) -> Option<u32> {
    let mut total_replies: u32 = 0;
    let mut next_batch_token = None;

    loop {
        let options = RelationsOptions {
            from: next_batch_token.clone(),
            dir: Direction::Backward,
            limit: Some(uint!(100)),
            include_relations: IncludeRelations::RelationsOfType(RelationType::Thread),
            ..Default::default()
        };

        let relations = room
            .relations(thread_root_event_id.to_owned(), options)
            .await
            .ok()?;
        if relations.chunk.is_empty() {
            break;
        }
        total_replies = total_replies.saturating_add(relations.chunk.len() as u32);

        next_batch_token = relations.next_batch_token;
        if next_batch_token.is_none() {
            break;
        }
    }

    Some(total_replies)
}

async fn search_room_messages_server(
    client: Client,
    room_id: OwnedRoomId,
    query: String,
    search_filter: MessageSearchServerFilter,
    limit: u16,
    next_batch: Option<String>,
) -> Result<MessageSearchServerResponse, String> {
    let mut categories = search_events::v3::Categories::new();
    let mut criteria = search_events::v3::Criteria::new(query.clone());
    criteria.keys = Some(vec![search_events::v3::SearchKeys::ContentBody]);
    criteria.order_by = Some(search_events::v3::OrderBy::Recent);
    let mut event_context = search_events::v3::EventContext::new();
    event_context.before_limit = uint!(1);
    event_context.after_limit = uint!(1);
    event_context.include_profile = true;
    criteria.event_context = event_context;
    let mut filter = RoomEventFilter::default();
    filter.limit = Some(if limit >= 50 {
        uint!(50)
    } else if limit >= 20 {
        uint!(20)
    } else if limit >= 10 {
        uint!(10)
    } else {
        uint!(5)
    });
    filter.rooms = Some(vec![room_id.clone()]);
    filter.types = Some(vec!["m.room.message".to_string()]);
    if let Some(sender) = search_filter
        .sender
        .as_deref()
        .map(str::trim)
        .filter(|sender| !sender.is_empty())
    {
        let sender_id = OwnedUserId::try_from(sender)
            .map_err(|error| format!("Invalid Matrix sender filter `{sender}`: {error}"))?;
        filter.senders = Some(vec![sender_id]);
    }
    if search_filter.media_only {
        filter.url_filter = Some(UrlFilter::EventsWithUrl);
    }
    criteria.filter = filter;
    categories.room_events = Some(criteria);

    let mut request = search_events::v3::Request::new(categories);
    request.next_batch = next_batch;
    let response = client
        .send(request)
        .await
        .map_err(|error| format!("{error:?}"))?;
    let room_events = response.search_categories.room_events;
    let hits = room_events
        .results
        .iter()
        .map(message_search_server_hit_from_result)
        .collect();

    Ok(MessageSearchServerResponse {
        query,
        room_id: room_id.to_string(),
        filter: search_filter,
        count: room_events.count.map(|count| count.to_string()),
        next_batch: room_events.next_batch,
        highlights: room_events.highlights,
        hits,
    })
}

fn message_search_server_hit_from_result(
    result: &search_events::v3::SearchResult,
) -> MessageSearchServerHit {
    let raw_event = result.result.as_ref();
    MessageSearchServerHit {
        event_id: raw_event
            .and_then(|raw| raw.get_field::<OwnedEventId>("event_id").ok().flatten())
            .map(|event_id| event_id.to_string()),
        sender: raw_event
            .and_then(|raw| raw.get_field::<OwnedUserId>("sender").ok().flatten())
            .map(|sender| sender.to_string()),
        origin_server_ts: raw_event
            .and_then(|raw| {
                raw.get_field::<MilliSecondsSinceUnixEpoch>("origin_server_ts")
                    .ok()
                    .flatten()
            })
            .map(|timestamp| timestamp.get().to_string()),
        body: raw_event
            .map(message_search_server_body_from_raw)
            .unwrap_or_else(|| "body unavailable".to_string()),
        source_json: raw_event.and_then(message_search_server_source_json_from_raw),
        rank: result.rank,
        context_before_count: result.context.events_before.len(),
        context_after_count: result.context.events_after.len(),
        context_before_previews: message_search_server_context_previews(
            &result.context.events_before,
        ),
        context_after_previews: message_search_server_context_previews(
            &result.context.events_after,
        ),
    }
}

fn message_search_server_context_previews<T>(
    events: &[matrix_sdk::ruma::serde::Raw<T>],
) -> Vec<String> {
    events
        .iter()
        .take(2)
        .map(message_search_server_context_preview_from_raw)
        .collect()
}

fn message_search_server_context_preview_from_raw<T>(
    raw: &matrix_sdk::ruma::serde::Raw<T>,
) -> String {
    let event_id = raw
        .get_field::<OwnedEventId>("event_id")
        .ok()
        .flatten()
        .map(|event_id| event_id.to_string())
        .unwrap_or_else(|| "event id unavailable".to_string());
    let sender = raw
        .get_field::<OwnedUserId>("sender")
        .ok()
        .flatten()
        .map(|sender| sender.to_string())
        .unwrap_or_else(|| "sender unavailable".to_string());
    let body = message_search_server_body_from_raw(raw);
    format!("{event_id} from {sender}: {body}")
}

fn message_search_server_source_json_from_raw<T>(
    raw: &matrix_sdk::ruma::serde::Raw<T>,
) -> Option<String> {
    serde_json::to_value(raw)
        .ok()
        .and_then(|value| serde_json::to_string_pretty(&value).ok())
}

fn message_search_server_body_from_raw<T>(raw: &matrix_sdk::ruma::serde::Raw<T>) -> String {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(raw.json().get()) else {
        return "body unavailable".to_string();
    };
    value
        .get("content")
        .and_then(|content| {
            content
                .get("body")
                .or_else(|| content.get("formatted_body"))
        })
        .and_then(|body| body.as_str())
        .filter(|body| !body.trim().is_empty())
        .map(|body| body.trim().to_string())
        .or_else(|| {
            value
                .get("type")
                .and_then(|event_type| event_type.as_str())
                .map(|event_type| format!("{event_type} event"))
        })
        .unwrap_or_else(|| "body unavailable".to_string())
}

/// Fetches a compact summary of `m.replace` relations for an edited event.
async fn fetch_message_edit_history(
    room: &Room,
    edited_event_id: &EventId,
) -> Result<EditHistorySummary, String> {
    let mut replacement_count = 0usize;
    let mut pages_fetched = 0usize;
    let mut next_batch_token = None;
    let mut latest_event_id = None;
    let mut latest_timestamp = None;
    let mut latest_preview_text = None;
    let mut latest_source_json = None;

    let pagination_exhausted = loop {
        let options = RelationsOptions {
            from: next_batch_token.clone(),
            dir: Direction::Backward,
            limit: Some(uint!(50)),
            include_relations: IncludeRelations::RelationsOfType(RelationType::Replacement),
            ..Default::default()
        };

        let relations = room
            .relations(edited_event_id.to_owned(), options)
            .await
            .map_err(|error| format!("{error:?}"))?;
        pages_fetched = pages_fetched.saturating_add(1);
        if relations.chunk.is_empty() {
            break relations.next_batch_token.is_none();
        }

        for replacement in &relations.chunk {
            replacement_count = replacement_count.saturating_add(1);
            if latest_event_id.is_none() {
                let raw = replacement.raw();
                latest_event_id = raw.get_field::<OwnedEventId>("event_id").ok().flatten();
                latest_timestamp = raw
                    .get_field::<MilliSecondsSinceUnixEpoch>("origin_server_ts")
                    .ok()
                    .flatten();
                latest_source_json = serde_json::to_value(raw)
                    .ok()
                    .and_then(|value| serde_json::to_string_pretty(&value).ok());
                latest_preview_text = text_preview_of_latest_thread_reply(room, replacement).await;
            }
        }

        next_batch_token = relations.next_batch_token;
        if next_batch_token.is_none() {
            break true;
        }
    };

    Ok(EditHistorySummary {
        replacement_count,
        pages_fetched,
        pagination_exhausted,
        latest_event_id,
        latest_timestamp,
        latest_preview_text,
        latest_source_json,
    })
}

async fn fetch_event_source_json(room: &Room, event_id: &EventId) -> Result<String, String> {
    let event = room
        .load_or_fetch_event(event_id, None)
        .await
        .map_err(|error| format!("Failed to fetch event source for {event_id}: {error}"))?;

    serde_json::to_value(event.raw())
        .and_then(|value| serde_json::to_string_pretty(&value))
        .map_err(|error| format!("Failed to serialize event source for {event_id}: {error}"))
}

/// Returns an HTML-formatted text preview of the given latest thread reply event.
async fn text_preview_of_latest_thread_reply(
    room: &Room,
    latest_reply_event: &matrix_sdk::deserialized_responses::TimelineEvent,
) -> Option<String> {
    let raw = latest_reply_event.raw();
    let sender_id = raw.get_field::<OwnedUserId>("sender").ok().flatten()?;
    let sender_room_member = match room.get_member_no_sync(&sender_id).await {
        Ok(Some(rm)) => Some(rm),
        _ => room.get_member(&sender_id).await.ok().flatten(),
    };
    let sender_name = sender_room_member
        .as_ref()
        .and_then(|rm| rm.display_name())
        .unwrap_or(sender_id.as_str());
    let text_preview = text_preview_of_raw_timeline_event(raw, sender_name).unwrap_or_else(|| {
        let event_type = raw.get_field::<String>("type").ok().flatten();
        TextPreview::from((
            event_type.unwrap_or_else(|| "unknown event type".to_string()),
            BeforeText::UsernameWithColon,
        ))
    });
    let preview_str = text_preview.format_with(sender_name, true);
    match utils::replace_linebreaks_separators(&preview_str, true) {
        Cow::Borrowed(_) => Some(preview_str),
        Cow::Owned(replaced) => Some(replaced),
    }
}

/// Returns the timestamp and an HTML-formatted text preview of the given `latest_event`.
///
/// If the sender profile of the event is not yet available, this function will
/// generate a preview using the sender's user ID instead of their display name.
async fn get_latest_event_details(
    latest_event_value: &LatestEventValue,
    client: &Client,
) -> Option<(MilliSecondsSinceUnixEpoch, String)> {
    macro_rules! get_sender_username {
        ($profile:expr, $sender:expr, $is_own:expr) => {{
            let sender_username_opt = if let TimelineDetails::Ready(profile) = $profile {
                profile.display_name.clone()
            } else if $is_own {
                client.account().get_display_name().await.ok().flatten()
            } else {
                None
            };
            sender_username_opt.unwrap_or_else(|| $sender.to_string())
        }};
    }

    match latest_event_value {
        LatestEventValue::None => None,
        LatestEventValue::Remote {
            timestamp,
            sender,
            is_own,
            profile,
            content,
        } => {
            let sender_username = get_sender_username!(profile, sender, *is_own);
            let latest_message_text =
                text_preview_of_timeline_item(content, sender, &sender_username)
                    .format_with(&sender_username, true);
            Some((*timestamp, latest_message_text))
        }
        LatestEventValue::Local {
            timestamp,
            sender,
            profile,
            content,
            state: _,
        } => {
            // TODO: use the `state` enum to augment the preview text with more details.
            //       Example: "<span color="blue">Sending... {msg}</span>" or
            //                "<span color="red">Failed to send {msg}</span>"
            let is_own = current_user_id().is_some_and(|id| &id == sender);
            let sender_username = get_sender_username!(profile, sender, is_own);
            let latest_message_text =
                text_preview_of_timeline_item(content, sender, &sender_username)
                    .format_with(&sender_username, true);
            Some((*timestamp, latest_message_text))
        }
        LatestEventValue::RemoteInvite { timestamp, .. } => {
            Some((*timestamp, String::from("You were invited to this room.")))
        }
    }
}

/// Handles the given updated latest event for the given room.
///
/// This function sends a `RoomsListUpdate::UpdateLatestEvent`
/// to update the latest event in the RoomsListEntry for the given room.
async fn update_latest_event(room: &Room) {
    if let Some((timestamp, latest_message_text)) =
        get_latest_event_details(&room.latest_event().await, &room.client()).await
    {
        enqueue_rooms_list_update(RoomsListUpdate::UpdateLatestEvent {
            room_id: room.room_id().to_owned(),
            timestamp,
            latest_message_text,
        });
    }
}

/// A request to search backwards for a specific event in a room's timeline.
pub struct BackwardsPaginateUntilEventRequest {
    pub room_id: OwnedRoomId,
    pub target_event_id: OwnedEventId,
    /// The index in the timeline where a backwards search should begin.
    pub starting_index: usize,
    /// The number of items in the timeline at the time of the request,
    /// which is used to detect if the timeline has changed since the request was made,
    /// meaning that the `starting_index` can no longer be relied upon.
    pub current_tl_len: usize,
}

/// Whether to enable verbose logging of all timeline diff updates.
const LOG_TIMELINE_DIFFS: bool = cfg!(feature = "log_timeline_diffs");
/// Whether to enable verbose logging of all room list service diff updates.
const LOG_ROOM_LIST_DIFFS: bool = cfg!(feature = "log_room_list_diffs");

/// A per-timeline async task that listens for timeline updates and sends them to the UI thread.
///
/// One instance of this async task is spawned for each room the client knows about,
/// and also one for each thread that the user opens in a thread view.
async fn timeline_subscriber_handler(
    room: Room,
    timeline: Arc<Timeline>,
    timeline_update_sender: crossbeam_channel::Sender<TimelineUpdate>,
    mut request_receiver: watch::Receiver<Vec<BackwardsPaginateUntilEventRequest>>,
    thread_root_event_id: Option<OwnedEventId>,
) {
    /// An inner function that searches the given new timeline items for a target event.
    ///
    /// If the target event is found, it is removed from the `target_event_id_opt` and returned,
    /// along with the index/position of that event in the given iterator of new items.
    fn find_target_event<'a>(
        target_event_id_opt: &mut Option<OwnedEventId>,
        mut new_items_iter: impl Iterator<Item = &'a Arc<TimelineItem>>,
    ) -> Option<(usize, OwnedEventId)> {
        let found_index = target_event_id_opt.as_ref().and_then(|target_event_id| {
            new_items_iter.position(|new_item| {
                new_item
                    .as_event()
                    .is_some_and(|new_ev| new_ev.event_id() == Some(target_event_id))
            })
        });

        if let Some(index) = found_index {
            target_event_id_opt.take().map(|ev| (index, ev))
        } else {
            None
        }
    }

    let room_id = room.room_id().to_owned();
    log!("Starting timeline subscriber for room {room_id}, thread {thread_root_event_id:?}...");
    let (mut timeline_items, mut subscriber) = timeline.subscribe().await;
    log!(
        "Received initial timeline update of {} items for room {room_id}, thread {thread_root_event_id:?}.",
        timeline_items.len()
    );

    timeline_update_sender.send(TimelineUpdate::FirstUpdate {
        initial_items: timeline_items.clone(),
    }).unwrap_or_else(
        |_e| panic!("Error: timeline update sender couldn't send first update ({} items) to room {room_id}, thread {thread_root_event_id:?}...!", timeline_items.len())
    );

    // the event ID to search for while loading previous items into the timeline.
    let mut target_event_id = None;
    // the timeline index and event ID of the target event, if it has been found.
    let mut found_target_event_id: Option<(usize, OwnedEventId)> = None;

    loop {
        tokio::select! {
            // we should check for new requests before handling new timeline updates,
            // because the request might influence how we handle a timeline update.
            biased;

            // Handle updates to the current backwards pagination requests.
            Ok(()) = request_receiver.changed() => {
                let prev_target_event_id = target_event_id.clone();
                let new_request_details = request_receiver
                    .borrow_and_update()
                    .iter()
                    .find_map(|req| req.room_id
                        .eq(&room_id)
                        .then(|| (req.target_event_id.clone(), req.starting_index, req.current_tl_len))
                    );

                target_event_id = new_request_details.as_ref().map(|(ev, ..)| ev.clone());

                // If we received a new request, start searching backwards for the target event.
                if let Some((new_target_event_id, starting_index, current_tl_len)) = new_request_details {
                    if prev_target_event_id.as_ref() != Some(&new_target_event_id) {
                        let starting_index = if current_tl_len == timeline_items.len() {
                            starting_index
                        } else {
                            // The timeline has changed since the request was made, so we can't rely on the `starting_index`.
                            // Instead, we have no choice but to start from the end of the timeline.
                            timeline_items.len()
                        };
                        // log!("Received new request to search for event {new_target_event_id} in room {room_id}, thread {thread_root_event_id:?} starting from index {starting_index} (tl len {}).", timeline_items.len());
                        // Search backwards for the target event in the timeline, starting from the given index.
                        if let Some(target_event_tl_index) = timeline_items
                            .focus()
                            .narrow(..starting_index)
                            .into_iter()
                            .rev()
                            .position(|i| i.as_event()
                                .and_then(|e| e.event_id())
                                .is_some_and(|ev_id| ev_id == new_target_event_id)
                            )
                            .map(|i| starting_index.saturating_sub(i).saturating_sub(1))
                        {
                            // log!("Found existing target event {new_target_event_id} in room {room_id}, thread {thread_root_event_id:?} at index {target_event_tl_index}.");

                            // Nice! We found the target event in the current timeline items,
                            // so there's no need to actually proceed with backwards pagination;
                            // thus, we can clear the locally-tracked target event ID.
                            target_event_id = None;
                            found_target_event_id = None;
                            timeline_update_sender.send(
                                TimelineUpdate::TargetEventFound {
                                    target_event_id: new_target_event_id.clone(),
                                    index: target_event_tl_index,
                                }
                            ).unwrap_or_else(
                                |_e| panic!("Error: timeline update sender couldn't send TargetEventFound({new_target_event_id}, {target_event_tl_index}) to room {room_id}, thread {thread_root_event_id:?}!")
                            );
                            // Send a Makepad-level signal to update this room's timeline UI view.
                            SignalToUI::set_ui_signal();
                        }
                        else {
                            log!("Target event not in timeline. Starting backwards pagination \
                                in room {room_id}, thread {thread_root_event_id:?} to find target event \
                                {new_target_event_id} starting from index {starting_index}.",
                            );
                            // If we didn't find the target event in the current timeline items,
                            // we need to start loading previous items into the timeline.
                            submit_async_request(MatrixRequest::PaginateTimeline {
                                timeline_kind: if let Some(thread_root_event_id) = thread_root_event_id.clone() {
                                    TimelineKind::Thread {
                                        room_id: room_id.clone(),
                                        thread_root_event_id,
                                    }
                                } else {
                                    TimelineKind::MainRoom {
                                        room_id: room_id.clone(),
                                    }
                                },
                                num_events: 50,
                                direction: PaginationDirection::Backwards,
                            });
                        }
                    }
                }
            }

            // Handle updates to the actual timeline content.
            batch_opt = subscriber.next() => {
                let Some(batch) = batch_opt else { break };
                let mut num_updates = 0;
                let mut index_of_first_change = usize::MAX;
                let mut index_of_last_change = usize::MIN;
                // whether to clear the entire cache of drawn items
                let mut clear_cache = false;
                // whether the changes include items being appended to the end of the timeline
                let mut is_append = false;
                for diff in batch {
                    num_updates += 1;
                    match diff {
                        VectorDiff::Append { values } => {
                            let _values_len = values.len();
                            index_of_first_change = min(index_of_first_change, timeline_items.len());
                            timeline_items.extend(values);
                            index_of_last_change = max(index_of_last_change, timeline_items.len());
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff Append {_values_len}. Changes: {index_of_first_change}..{index_of_last_change}"); }
                            is_append = true;
                        }
                        VectorDiff::Clear => {
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff Clear"); }
                            clear_cache = true;
                            timeline_items.clear();
                        }
                        VectorDiff::PushFront { value } => {
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff PushFront"); }
                            if let Some((index, _ev)) = found_target_event_id.as_mut() {
                                *index += 1; // account for this new `value` being prepended.
                            } else {
                                found_target_event_id = find_target_event(&mut target_event_id, std::iter::once(&value));
                            }

                            clear_cache = true;
                            timeline_items.push_front(value);
                        }
                        VectorDiff::PushBack { value } => {
                            index_of_first_change = min(index_of_first_change, timeline_items.len());
                            timeline_items.push_back(value);
                            index_of_last_change = max(index_of_last_change, timeline_items.len());
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff PushBack. Changes: {index_of_first_change}..{index_of_last_change}"); }
                            is_append = true;
                        }
                        VectorDiff::PopFront => {
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff PopFront"); }
                            clear_cache = true;
                            timeline_items.pop_front();
                            if let Some((i, _ev)) = found_target_event_id.as_mut() {
                                *i = i.saturating_sub(1); // account for the first item being removed.
                            }
                            // This doesn't affect whether we should reobtain the latest event.
                        }
                        VectorDiff::PopBack => {
                            timeline_items.pop_back();
                            index_of_first_change = min(index_of_first_change, timeline_items.len());
                            index_of_last_change = usize::MAX;
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff PopBack. Changes: {index_of_first_change}..{index_of_last_change}"); }
                        }
                        VectorDiff::Insert { index, value } => {
                            if index == 0 {
                                clear_cache = true;
                            } else {
                                index_of_first_change = min(index_of_first_change, index);
                                index_of_last_change = usize::MAX;
                            }
                            if index >= timeline_items.len() {
                                is_append = true;
                            }

                            if let Some((i, _ev)) = found_target_event_id.as_mut() {
                                // account for this new `value` being inserted before the previously-found target event's index.
                                if index <= *i {
                                    *i += 1;
                                }
                            } else {
                                found_target_event_id = find_target_event(&mut target_event_id, std::iter::once(&value))
                                    .map(|(i, ev)| (i + index, ev));
                            }

                            timeline_items.insert(index, value);
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff Insert at {index}. Changes: {index_of_first_change}..{index_of_last_change}"); }
                        }
                        VectorDiff::Set { index, value } => {
                            index_of_first_change = min(index_of_first_change, index);
                            index_of_last_change  = max(index_of_last_change, index.saturating_add(1));
                            timeline_items.set(index, value);
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff Set at {index}. Changes: {index_of_first_change}..{index_of_last_change}"); }
                        }
                        VectorDiff::Remove { index } => {
                            if index == 0 {
                                clear_cache = true;
                            } else {
                                index_of_first_change = min(index_of_first_change, index.saturating_sub(1));
                                index_of_last_change = usize::MAX;
                            }
                            if let Some((i, _ev)) = found_target_event_id.as_mut() {
                                // account for an item being removed before the previously-found target event's index.
                                if index <= *i {
                                    *i = i.saturating_sub(1);
                                }
                            }
                            timeline_items.remove(index);
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff Remove at {index}. Changes: {index_of_first_change}..{index_of_last_change}"); }
                        }
                        VectorDiff::Truncate { length } => {
                            if length == 0 {
                                clear_cache = true;
                            } else {
                                index_of_first_change = min(index_of_first_change, length.saturating_sub(1));
                                index_of_last_change = usize::MAX;
                            }
                            timeline_items.truncate(length);
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff Truncate to length {length}. Changes: {index_of_first_change}..{index_of_last_change}"); }
                        }
                        VectorDiff::Reset { values } => {
                            if LOG_TIMELINE_DIFFS { log!("timeline_subscriber: room {room_id}, thread {thread_root_event_id:?} diff Reset, new length {}", values.len()); }
                            clear_cache = true; // we must assume all items have changed.
                            timeline_items = values;
                        }
                    }
                }


                if num_updates > 0 {
                    // Handle the case where back pagination inserts items at the beginning of the timeline
                    // (meaning the entire timeline needs to be re-drawn),
                    // but there is a virtual event at index 0 (e.g., a day divider).
                    // When that happens, we want the RoomScreen to treat this as if *all* events changed.
                    if index_of_first_change == 1 && timeline_items.front().and_then(|item| item.as_virtual()).is_some() {
                        index_of_first_change = 0;
                        clear_cache = true;
                    }

                    let changed_indices = index_of_first_change..index_of_last_change;

                    if LOG_TIMELINE_DIFFS {
                        log!("timeline_subscriber: applied {num_updates} updates for room {room_id}, thread {thread_root_event_id:?}, timeline now has {} items. is_append? {is_append}, clear_cache? {clear_cache}. Changes: {changed_indices:?}.", timeline_items.len());
                    }
                    timeline_update_sender.send(TimelineUpdate::NewItems {
                        new_items: timeline_items.clone(),
                        changed_indices,
                        clear_cache,
                        is_append,
                    }).expect("Error: timeline update sender couldn't send update with new items!");

                    // We must send this update *after* the actual NewItems update,
                    // otherwise the UI thread (RoomScreen) won't be able to correctly locate the target event.
                    if let Some((index, found_event_id)) = found_target_event_id.take() {
                        target_event_id = None;
                        timeline_update_sender.send(
                            TimelineUpdate::TargetEventFound {
                                target_event_id: found_event_id.clone(),
                                index,
                            }
                        ).unwrap_or_else(
                            |_e| panic!("Error: timeline update sender couldn't send TargetEventFound({found_event_id}, {index}) to room {room_id}, thread {thread_root_event_id:?}!")
                        );
                    }

                    // Send a Makepad-level signal to update this room's timeline UI view.
                    SignalToUI::set_ui_signal();
                }
            }

            else => {
                break;
            }
        }
    }

    error!(
        "Error: unexpectedly ended timeline subscriber for room {room_id}, thread {thread_root_event_id:?}."
    );
}

/// Spawn a new async task to fetch the room's new avatar.
fn spawn_fetch_room_avatar(room: &RoomListServiceRoomInfo) {
    let room_id = room.room_id.clone();
    let room_name_id = RoomNameId::from((room.display_name.clone(), room.room_id.clone()));
    let inner_room = room.room.clone();
    Handle::current().spawn(async move {
        let room_avatar = room_avatar(&inner_room, &room_name_id).await;
        rooms_list::enqueue_rooms_list_update(RoomsListUpdate::UpdateRoomAvatar {
            room_id,
            room_avatar,
        });
    });
}

/// Fetches and returns the avatar image for the given room (if one exists),
/// otherwise returns a text avatar string of the first character of the room name.
async fn room_avatar(room: &Room, room_name_id: &RoomNameId) -> FetchedRoomAvatar {
    match room.avatar(AVATAR_THUMBNAIL_FORMAT.into()).await {
        Ok(Some(avatar)) => FetchedRoomAvatar::Image(avatar.into()),
        _ => {
            if let Ok(room_members) = room.members(RoomMemberships::ACTIVE).await {
                if room_members.len() == 2 {
                    if let Some(non_account_member) =
                        room_members.iter().find(|m| !m.is_account_user())
                    {
                        if let Ok(Some(avatar)) = non_account_member
                            .avatar(AVATAR_THUMBNAIL_FORMAT.into())
                            .await
                        {
                            return FetchedRoomAvatar::Image(avatar.into());
                        }
                    }
                }
            }
            utils::avatar_from_room_name(room_name_id.name_for_avatar())
        }
    }
}

/// Spawn an async task to login to the given Matrix homeserver using the given SSO identity provider ID.
///
/// This function will post a `LoginAction::SsoPending(true)` to the main thread, and another
/// `LoginAction::SsoPending(false)` once the async task has either successfully logged in or
/// failed to do so.
///
/// If the login attempt is successful, the resulting `Client` and `ClientSession` will be sent
/// to the login screen using the `login_sender`.
async fn spawn_sso_server(
    brand: String,
    homeserver_url: String,
    identity_provider_id: String,
    login_sender: Sender<LoginRequest>,
) {
    Cx::post_action(LoginAction::SsoPending(true));
    // Post a status update to inform the user that we're waiting for the client to be built.
    Cx::post_action(LoginAction::Status {
        title: "Initializing client...".into(),
        status: "Please wait while Matrix builds and configures the client object for login."
            .into(),
    });

    // Wait for the notification that the client has been built
    DEFAULT_SSO_CLIENT_NOTIFIER.notified().await;

    // Try to use the DEFAULT_SSO_CLIENT, if it was successfully built.
    // We do not clone it because a Client cannot be re-used again
    // once it has been used for a login attempt, so this forces us to create a new one
    // if that occurs.
    let client_and_session_opt = DEFAULT_SSO_CLIENT.lock().unwrap().take();

    Handle::current().spawn(async move {
        // Try to use the DEFAULT_SSO_CLIENT that we proactively created
        // during initialization (to speed up opening the SSO browser window).
        let mut client_and_session = client_and_session_opt;

        // If the DEFAULT_SSO_CLIENT is none (meaning it failed to build),
        // or if the homeserver_url is *not* empty and isn't the default,
        // we cannot use the DEFAULT_SSO_CLIENT, so we must build a new one.
        let mut build_client_error = None;
        if client_and_session.is_none() || (
            !homeserver_url.is_empty()
                && homeserver_url != "matrix.org"
                && Url::parse(&homeserver_url) != Url::parse("https://matrix-client.matrix.org/")
                && Url::parse(&homeserver_url) != Url::parse("https://matrix.org/")
        ) {
            match build_client(
                &Cli {
                    homeserver: homeserver_url.is_empty().not().then_some(homeserver_url),
                    ..Default::default()
                },
                app_data_dir(),
            ).await {
                Ok(success) => client_and_session = Some(success),
                Err(e) => build_client_error = Some(e),
            }
        }

        let Some((client, client_session)) = client_and_session else {
            Cx::post_action(LoginAction::LoginFailure(
                if let Some(err) = build_client_error {
                    format!("Could not create client object. Please try to login again.\n\nError: {err}")
                } else {
                    String::from("Could not create client object. Please try to login again.")
                }
            ));
            // This ensures that the called to `DEFAULT_SSO_CLIENT_NOTIFIER.notified()`
            // at the top of this function will not block upon the next login attempt.
            DEFAULT_SSO_CLIENT_NOTIFIER.notify_one();
            Cx::post_action(LoginAction::SsoPending(false));
            return;
        };

        // The proactively-built client may have a stale TCP connection by
        // now. Retry once here so it surfaces before we open the browser.
        if let Err(e) = warmup_homeserver_connection(&client).await {
            error!("SSO warmup failed twice: {e:?}");
            Cx::post_action(LoginAction::LoginFailure(format!(
                "Could not reach homeserver: {e}"
            )));
            DEFAULT_SSO_CLIENT_NOTIFIER.notify_one();
            Cx::post_action(LoginAction::SsoPending(false));
            return;
        }

        let mut is_logged_in = false;

        // Desktop's `login_sso` uses a local HTTP server for the OAuth
        // redirect, which iOS suspends when Robrix backgrounds for Safari.
        // iOS uses ASWebAuthenticationSession to keep the app foregrounded.
        #[cfg(not(target_os = "ios"))]
        let login_result = {
            Cx::post_action(LoginAction::Status {
                title: "Opening your browser...".into(),
                status: "Please finish logging in using your browser, and then come back to Hepta Native.".into(),
            });
            client
                .matrix_auth()
                .login_sso(|sso_url: String| async move {
                    let url = Url::parse(&sso_url)?;
                    for (key, value) in url.query_pairs() {
                        if key == "redirectUrl" {
                            let redirect_url = Url::parse(&value)?;
                            Cx::post_action(LoginAction::SsoSetRedirectUrl(redirect_url));
                            break
                        }
                    }
                    Uri::new(&sso_url).open().map_err(|err|
                        Error::Io(io::Error::other(format!("Unable to open SSO login url. Error: {:?}", err)))
                    )
                })
                .identity_provider_id(&identity_provider_id)
                .initial_device_display_name(&format!("hepta-native-sso-{brand}"))
                .await
        };
        #[cfg(target_os = "ios")]
        let login_result = {
            Cx::post_action(LoginAction::Status {
                title: "Opening in-app authentication...".into(),
                status: "Please complete login in the authentication sheet that appeared.".into(),
            });
            run_ios_sso_flow(&client, &identity_provider_id, &brand).await
        };

        match login_result.inspect(|_| {
            if let Some(client) = get_client() {
                if client.matrix_auth().logged_in() {
                    is_logged_in = true;
                    log!("Already logged in, ignore login with sso");
                }
            }
        }) {
            Ok(identity_provider_res) => {
                if !is_logged_in {
                    if let Err(e) = login_sender.send(LoginRequest::LoginBySSOSuccess(client, client_session)).await {
                        error!("Error sending login request to login_sender: {e:?}");
                        Cx::post_action(LoginAction::LoginFailure(String::from(
                            "BUG: failed to send login request to matrix worker thread."
                        )));
                    }
                    enqueue_rooms_list_update(RoomsListUpdate::Status {
                        status: format!(
                            "Logged in as {:?}.\n → Loading rooms...",
                            &identity_provider_res.user_id
                        ),
                    });
                }
            }
            Err(e) => {
                if !is_logged_in {
                    error!("SSO Login failed: {e:?}");
                    Cx::post_action(LoginAction::LoginFailure(format!("SSO login failed: {e}")));
                }
            }
        }

        // This ensures that the called to `DEFAULT_SSO_CLIENT_NOTIFIER.notified()`
        // at the top of this function will not block upon the next login attempt.
        DEFAULT_SSO_CLIENT_NOTIFIER.notify_one();
        Cx::post_action(LoginAction::SsoPending(false));
    });
}

/// Pings the homeserver before SSO opens a browser or sheet, retrying once.
/// Recovers from stale pooled connections so the first SSO click doesn't
/// fail visibly.
async fn warmup_homeserver_connection(client: &Client) -> matrix_sdk::HttpResult<()> {
    // `supported_versions()` doesn't cache for unauthenticated clients, so
    // it always hits the network, which is what we want for warmup.
    match client.supported_versions().await {
        Ok(_) => Ok(()),
        Err(e) => {
            warning!("Homeserver warmup failed (likely stale connection): {e:?}. Retrying once.");
            client.supported_versions().await.map(|_| ())
        }
    }
}

/// Drives iOS SSO via `ASWebAuthenticationSession`. Gets the SSO URL with a
/// `hepta-native://` redirect, opens it in the auth sheet, and feeds the callback
/// URL through `login_with_sso_callback` to finish.
#[cfg(target_os = "ios")]
async fn run_ios_sso_flow(
    client: &Client,
    identity_provider_id: &str,
    brand: &str,
) -> std::result::Result<
    matrix_sdk::ruma::api::client::session::login::v3::Response,
    matrix_sdk::Error,
> {
    use tokio::sync::oneshot;

    // Session-scoped scheme, so no Info.plist registration needed. Synapse
    // doesn't validate redirectUrl, so the URL shape is up to us.
    const REDIRECT_URL: &str = "hepta-native://login";
    const CALLBACK_SCHEME: &str = "hepta-native";

    let auth = client.matrix_auth();
    let sso_url = auth
        .get_sso_login_url(REDIRECT_URL, Some(identity_provider_id))
        .await?;

    // Bridge the OS completion callback into a Rust oneshot. Mutex<Option>
    // guards against the OS double-firing, and the same callback clears
    // ACTIVE_SSO_AUTH_SESSION so cancel becomes a no-op once auth is done.
    let (tx, rx) = oneshot::channel::<robius_web_auth_session::Result<String>>();
    let tx = std::sync::Mutex::new(Some(tx));

    let handle = robius_web_auth_session::AuthSession::new(&sso_url, CALLBACK_SCHEME)
        .start(move |result| {
            if let Ok(mut slot) = ACTIVE_SSO_AUTH_SESSION.lock() {
                *slot = None;
            }
            if let Some(tx) = tx.lock().unwrap().take() {
                let _ = tx.send(result);
            }
        })
        .map_err(|e| {
            matrix_sdk::Error::Io(io::Error::other(format!(
                "Failed to start ASWebAuthenticationSession: {e}"
            )))
        })?;

    // Publish the cancel handle so the modal's Cancel button can reach it.
    // Cleared by the completion callback above.
    if let Ok(mut slot) = ACTIVE_SSO_AUTH_SESSION.lock() {
        *slot = Some(handle);
    }

    let callback_url_str = rx
        .await
        .map_err(|_| {
            matrix_sdk::Error::Io(io::Error::other(
                "ASWebAuthenticationSession completion handler dropped without firing",
            ))
        })?
        .map_err(|e| {
            matrix_sdk::Error::Io(io::Error::other(format!(
                "ASWebAuthenticationSession failed: {e}"
            )))
        })?;

    let callback_url = Url::parse(&callback_url_str).map_err(|e| {
        matrix_sdk::Error::Io(io::Error::other(format!(
            "Invalid SSO callback URL ({callback_url_str:?}): {e}"
        )))
    })?;

    auth.login_with_sso_callback(callback_url.into())
        .map_err(|e| {
            matrix_sdk::Error::Io(io::Error::other(format!(
                "Failed to parse SSO callback for loginToken: {e}"
            )))
        })?
        .initial_device_display_name(&format!("hepta-native-sso-{brand}"))
        .await
}

bitflags! {
    /// The powers that a user has in a given room.
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct UserPowerLevels: u64 {
        const Ban = 1 << 0;
        const Invite = 1 << 1;
        const Kick = 1 << 2;
        const Redact = 1 << 3;
        const NotifyRoom = 1 << 4;
        // -------------------------------------
        // -- Copied from TimelineEventType ----
        // -- Unused powers are commented out --
        // -------------------------------------
        // const CallAnswer = 1 << 5;
        // const CallInvite = 1 << 6;
        // const CallHangup = 1 << 7;
        // const CallCandidates = 1 << 8;
        // const CallNegotiate = 1 << 9;
        // const CallReject = 1 << 10;
        // const CallSdpStreamMetadataChanged = 1 << 11;
        // const CallSelectAnswer = 1 << 12;
        // const KeyVerificationReady = 1 << 13;
        // const KeyVerificationStart = 1 << 14;
        // const KeyVerificationCancel = 1 << 15;
        // const KeyVerificationAccept = 1 << 16;
        // const KeyVerificationKey = 1 << 17;
        // const KeyVerificationMac = 1 << 18;
        // const KeyVerificationDone = 1 << 19;
        const Location = 1 << 20;
        const Message = 1 << 21;
        // const PollStart = 1 << 22;
        // const UnstablePollStart = 1 << 23;
        // const PollResponse = 1 << 24;
        // const UnstablePollResponse = 1 << 25;
        // const PollEnd = 1 << 26;
        // const UnstablePollEnd = 1 << 27;
        // const Beacon = 1 << 28;
        const Reaction = 1 << 29;
        // const RoomEncrypted = 1 << 30;
        const RoomMessage = 1 << 31;
        const RoomRedaction = 1 << 32;
        const Sticker = 1 << 33;
        // const CallNotify = 1 << 34;
        // const PolicyRuleRoom = 1 << 35;
        // const PolicyRuleServer = 1 << 36;
        // const PolicyRuleUser = 1 << 37;
        // const RoomAliases = 1 << 38;
        // const RoomAvatar = 1 << 39;
        // const RoomCanonicalAlias = 1 << 40;
        // const RoomCreate = 1 << 41;
        // const RoomEncryption = 1 << 42;
        // const RoomGuestAccess = 1 << 43;
        // const RoomHistoryVisibility = 1 << 44;
        // const RoomJoinRules = 1 << 45;
        // const RoomMember = 1 << 46;
        // const RoomName = 1 << 47;
        const RoomPinnedEvents = 1 << 48;
        // const RoomPowerLevels = 1 << 49;
        // const RoomServerAcl = 1 << 50;
        // const RoomThirdPartyInvite = 1 << 51;
        // const RoomTombstone = 1 << 52;
        // const RoomTopic = 1 << 53;
        // const SpaceChild = 1 << 54;
        // const SpaceParent = 1 << 55;
        // const BeaconInfo = 1 << 56;
        // const CallMember = 1 << 57;
        // const MemberHints = 1 << 58;
    }
}
impl UserPowerLevels {
    pub fn from(power_levels: &RoomPowerLevels, user_id: &UserId) -> Self {
        let mut retval = UserPowerLevels::empty();
        let user_power = power_levels.for_user(user_id);
        retval.set(UserPowerLevels::Ban, user_power >= power_levels.ban);
        retval.set(UserPowerLevels::Invite, user_power >= power_levels.invite);
        retval.set(UserPowerLevels::Kick, user_power >= power_levels.kick);
        retval.set(UserPowerLevels::Redact, user_power >= power_levels.redact);
        retval.set(
            UserPowerLevels::NotifyRoom,
            user_power >= power_levels.notifications.room,
        );
        retval.set(
            UserPowerLevels::Location,
            user_power >= power_levels.for_message(MessageLikeEventType::Location),
        );
        retval.set(
            UserPowerLevels::Message,
            user_power >= power_levels.for_message(MessageLikeEventType::Message),
        );
        retval.set(
            UserPowerLevels::Reaction,
            user_power >= power_levels.for_message(MessageLikeEventType::Reaction),
        );
        retval.set(
            UserPowerLevels::RoomMessage,
            user_power >= power_levels.for_message(MessageLikeEventType::RoomMessage),
        );
        retval.set(
            UserPowerLevels::RoomRedaction,
            user_power >= power_levels.for_message(MessageLikeEventType::RoomRedaction),
        );
        retval.set(
            UserPowerLevels::Sticker,
            user_power >= power_levels.for_message(MessageLikeEventType::Sticker),
        );
        retval.set(
            UserPowerLevels::RoomPinnedEvents,
            user_power >= power_levels.for_state(StateEventType::RoomPinnedEvents),
        );
        retval
    }

    pub async fn from_room(room: &Room, user_id: &UserId) -> Option<Self> {
        let room_power_levels = room.power_levels().await.ok()?;
        Some(UserPowerLevels::from(&room_power_levels, user_id))
    }

    pub fn can_ban(self) -> bool {
        self.contains(UserPowerLevels::Ban)
    }

    pub fn can_unban(self) -> bool {
        self.can_ban() && self.can_kick()
    }

    pub fn can_invite(self) -> bool {
        self.contains(UserPowerLevels::Invite)
    }

    pub fn can_kick(self) -> bool {
        self.contains(UserPowerLevels::Kick)
    }

    pub fn can_redact(self) -> bool {
        self.contains(UserPowerLevels::Redact)
    }

    pub fn can_notify_room(self) -> bool {
        self.contains(UserPowerLevels::NotifyRoom)
    }

    pub fn can_redact_own(self) -> bool {
        self.contains(UserPowerLevels::RoomRedaction)
    }

    pub fn can_redact_others(self) -> bool {
        self.can_redact_own() && self.contains(UserPowerLevels::Redact)
    }

    pub fn can_send_location(self) -> bool {
        self.contains(UserPowerLevels::Location)
    }

    pub fn can_send_message(self) -> bool {
        self.contains(UserPowerLevels::RoomMessage) || self.contains(UserPowerLevels::Message)
    }

    pub fn can_send_reaction(self) -> bool {
        self.contains(UserPowerLevels::Reaction)
    }

    pub fn can_send_sticker(self) -> bool {
        self.contains(UserPowerLevels::Sticker)
    }

    #[doc(alias("unpin"))]
    pub fn can_pin(self) -> bool {
        self.contains(UserPowerLevels::RoomPinnedEvents)
    }
}

/// Drops session state and signals the login loop to wait for re-login.
/// Keeps `REQUEST_SENDER` alive, and also the `matrix_worker_task
/// which needs to keep running to receive the next login request.
pub async fn clear_app_state(config: &LogoutConfig) -> Result<()> {
    CLIENT.lock().unwrap().take();
    SYNC_SERVICE.lock().unwrap().take();
    SYNC_SERVICE_ASSUMED_RUNNING.store(false, Ordering::Release);
    IGNORED_USERS.lock().unwrap().clear();
    ALL_JOINED_ROOMS.lock().unwrap().clear();
    LOGOUT_NOTIFY.notify_one();

    let on_clear_appstate = Arc::new(Notify::new());
    Cx::post_action(LogoutAction::ClearAppState {
        on_clear_appstate: on_clear_appstate.clone(),
    });

    match tokio::time::timeout(
        config.app_state_cleanup_timeout,
        on_clear_appstate.notified(),
    )
    .await
    {
        Ok(_) => {
            log!("Received signal that UI-side app state was cleaned successfully");
            Ok(())
        }
        Err(_) => Err(anyhow!("Timed out waiting for UI-side app state cleanup")),
    }
}
