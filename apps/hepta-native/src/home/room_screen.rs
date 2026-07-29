//! The `RoomScreen` widget is the UI view that displays a single room or thread's timeline
//! of events (messages，state changes, etc.), along with an input bar at the bottom.

use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    fs,
    ops::{DerefMut, Range},
    path::{Path, PathBuf},
    sync::Arc,
};

use bytesize::ByteSize;
use hashbrown::{HashMap, HashSet};
use imbl::Vector;
use makepad_widgets::{image_cache::ImageBuffer, *};
use matrix_sdk::{
    OwnedServerName, RoomDisplayName, RoomState,
    media::{MediaFormat, MediaRequestParameters},
    notification_settings::RoomNotificationMode,
    room::RoomMember,
    ruma::{
        EventId, MatrixToUri, MatrixUri, MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedMxcUri,
        OwnedRoomAliasId, OwnedRoomId, OwnedRoomOrAliasId, UserId,
        events::{
            receipt::Receipt,
            room::{
                ImageInfo, MediaSource,
                message::{
                    AudioMessageEventContent, EmoteMessageEventContent, FileMessageEventContent,
                    FormattedBody, ImageMessageEventContent, KeyVerificationRequestEventContent,
                    LocationMessageEventContent, MessageFormat, MessageType,
                    NoticeMessageEventContent, TextMessageEventContent, VideoMessageEventContent,
                },
            },
            sticker::{StickerEventContent, StickerMediaSource},
        },
        matrix_uri::MatrixId,
        uint,
    },
};
use matrix_sdk_ui::timeline::{
    self, EmbeddedEvent, EncryptedMessage, EventSendState, EventTimelineItem, InReplyToDetails,
    LiveLocationState, MemberProfileChange, MembershipChange, MsgLikeContent, MsgLikeKind,
    OtherMessageLike, PollState, RoomMembershipChange, TimelineDetails, TimelineEventItemId,
    TimelineItem, TimelineItemContent, TimelineItemKind, VirtualTimelineItem,
};
use ruma::{
    OwnedUserId,
    api::client::receipt::create_receipt::v3::ReceiptType,
    events::{AnySyncMessageLikeEvent, AnySyncTimelineEvent, SyncMessageLikeEvent},
    owned_room_id,
    room::{JoinRuleSummary, RoomType},
};

use matrix_sdk_ui::sync_service::State;
use crate::{
    app::{AppStateAction, ConfirmDeleteAction, PositiveConfirmationModalAction, SelectedRoom},
    avatar_cache,
    event_preview::{
        plaintext_body_of_timeline_item, text_preview_of_encrypted_message,
        text_preview_of_member_profile_change, text_preview_of_other_message_like,
        text_preview_of_other_state, text_preview_of_room_membership_change,
        text_preview_of_timeline_item,
    },
    hepta_action_bridge::{
        decide_hepta_action, HeptaActionBridgeRequest, MUTATION_APPROVE_TOOL_EXEC,
    },
    hepta_event::{card_text_for_event, is_hepta_event_type, HeptaEventEnvelope},
    home::{
        add_room::KnockResultAction,
        edited_indicator::{
            EditedIndicatorAction, EditedIndicatorWidgetRefExt, MESSAGE_EDIT_HISTORY_COMPACT_LABEL,
            loaded_edit_history_target_metadata_label,
        },
        link_preview::{LinkPreviewCache, LinkPreviewRef, LinkPreviewWidgetRefExt},
        loading_pane::{LoadingPaneState, LoadingPaneWidgetExt},
        room_image_viewer::{get_image_name_and_filesize, populate_matrix_image_modal},
        rooms_list::{RoomsListAction, RoomsListRef},
        rooms_list_header::RoomsListHeaderAction,
        search_messages::{SearchMessagesAction, SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_LABEL},
        tombstone_footer::SuccessorRoomDetails,
    },
    media_cache::{MediaCache, MediaCacheEntry},
    profile::{
        user_profile::{
            ShowUserProfileAction, UserProfile, UserProfileAndRoomId, UserProfilePaneInfo,
            UserProfileSlidingPaneRef, UserProfileSlidingPaneWidgetExt,
        },
        user_profile_cache,
    },
    room::{
        BasicRoomDetails, FetchedRoomAvatar, FetchedRoomPreview,
        room_input_bar::{RoomInputBarState, RoomInputBarWidgetRefExt},
        typing_notice::TypingNoticeWidgetExt,
    },
    shared::{
        avatar::{AvatarState, AvatarWidgetRefExt},
        confirmation_modal::ConfirmationModalContent,
        html_or_plaintext::{
            HtmlOrPlaintextRef, HtmlOrPlaintextWidgetRefExt, RobrixHtmlLinkAction,
        },
        image_viewer::{ImageViewerAction, ImageViewerMetaData, LoadState},
        jump_to_bottom_button::{JumpToBottomButtonWidgetExt, UnreadMessageCount},
        popup_list::{PopupKind, enqueue_popup_notification},
        restore_status_view::RestoreStatusViewWidgetExt,
        styles::*,
        text_or_image::{TextOrImageAction, TextOrImageRef, TextOrImageWidgetRefExt},
        timestamp::TimestampWidgetRefExt,
    },
    sliding_sync::{
        BackwardsPaginateUntilEventRequest, MatrixRequest, NotificationDefaultRoomModeSummary,
        NotificationKeywordMutation, NotificationKeywordRulesSummary,
        NotificationPusherStatusSummary, PaginationDirection, TimelineEndpoints, TimelineKind,
        TimelineRequestSender, UserPowerLevels, get_client, submit_async_request,
        take_timeline_endpoints,
    },
    utils::{self, ImageFormat, MEDIA_THUMBNAIL_FORMAT, RoomNameId, unix_time_millis_to_datetime},
};
use crate::home::event_reaction_list::ReactionListWidgetRefExt;
use crate::home::invite_modal::InviteModalAction;
use crate::home::room_context_menu::RoomContextMenuDetails;
use crate::home::room_read_receipt::AvatarRowWidgetRefExt;
use crate::join_leave_room_modal::{JoinLeaveModalKind, JoinLeaveRoomModalAction};
use crate::room::room_input_bar::RoomInputBarWidgetExt;
use crate::shared::mentionable_text_input::MentionableTextInputAction;

use rangemap::RangeSet;

use super::{
    event_reaction_list::ReactionData,
    loading_pane::LoadingPaneRef,
    new_message_context_menu::{MessageAbilities, MessageDetails, MessageReportTargetMetadata},
    room_read_receipt::{self, populate_read_receipts, MAX_VISIBLE_AVATARS_IN_READ_RECEIPT},
};

pub const MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_EVIDENCE: &str = "RoomScreen message search is a loaded-timeline-only local helper while message_search remains a base gap. The search input, result count, active-match preview snippet, Prev/Next jumps, empty state, Close, Escape, and sidebar Messages button only scan timeline items already present in RoomScreen tl_state with plaintext_body_of_timeline_item. They do not submit Matrix-backed search, server-side history query, event context fetch, timeline pagination, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests.";
pub const MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_LABEL: &str =
    "Loaded timeline only: no Matrix-backed history search, event fetch, pagination, or mutation.";
pub const MESSAGE_SEARCH_COMPACT_LABEL: &str = "Search scans loaded local messages only.";
pub const MESSAGE_SEARCH_LOADED_METADATA_EVIDENCE: &str = "RoomScreen message search metadata summary is derived from already loaded RoomScreen tl_state items and local search state: query length, loaded timeline item count, match count, active match ordinal, active loaded index, and active loaded event-id availability. It sends no Matrix-backed search, server-side history query, event context fetch, timeline pagination or reload, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_LOADED_METADATA_LABEL: &str =
    "Loaded search metadata only; no server-side search.";
pub const MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_EVIDENCE: &str = "RoomScreen message search active-result detail is derived only from the currently loaded timeline match in tl_state. It shows the active ordinal, loaded item index, loaded event-id availability, query character count, local occurrence count inside the loaded plaintext body, and a compact snippet from plaintext_body_of_timeline_item. It sends no Matrix-backed search, server-side history query, event context fetch, timeline pagination or reload, event source open, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_LABEL: &str = "Active result detail is loaded-timeline local; no server-side search, event context, pagination, or mutation.";
pub const MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE: &str = "RoomScreen message search result-action controls expose Jump, Copy, Source, Thread, and Sender as visible controls and real result-action handoffs while message_search remains a base gap. Jump, Copy, Thread, and Sender remain real loaded result-action handoffs that still derive from the current loaded timeline match. Jump scrolls/highlights the active loaded match locally. Copy writes the current loaded match plaintext to the local clipboard. Source opens the existing local EventSourceModal from the active loaded timeline row's room id, event id, and loaded latest_json data when available; if latest_json is missing but the last Matrix /search response returned a raw server hit source for the current room, Source opens that cached server-result source instead; if the current-room event id is known but no raw JSON is cached, Source submits only MatrixRequest::FetchEventSource so Room::load_or_fetch_event can return source JSON to the same EventSourceModal. Thread opens the existing thread-focused timeline path only when the active loaded row already carries a thread root id. Sender opens the existing UserProfileSlidingPane from the active loaded timeline row's sender id, loaded sender_profile data, and local room_members cache when available; if member details are missing, the pane may reuse its existing GetUserProfile/profile-member read path. The controls do not submit a new Matrix search, fetch extra event context, call MatrixRequest::PaginateTimeline outside the existing context action, reload timeline, send/edit/redact a message, mutate room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_LABEL: &str = "Result actions are loaded handoffs: Jump scrolls/highlights the loaded match, Copy writes loaded plaintext, Source opens loaded/cached source or fetches source-only JSON, Thread opens a loaded thread root, and Sender opens the existing profile pane.";
pub const MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_EVIDENCE: &str = "RoomScreen message search Jump is a real loaded scroll/highlight handoff for the active loaded timeline match. The action uses the already computed `telegram_message_search_matches` index, refreshes loaded matches from RoomScreen tl_state, smooth-scrolls the PortalList to the active loaded index, and stages the existing message highlight animation. Its metadata is derived from query, loaded item count, local match count, active ordinal, loaded index, event-id availability, and compact plaintext snippet. It sends no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, thread timeline open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_LABEL: &str =
    "Search result Jump scrolls/highlights loaded local match only.";
pub const MESSAGE_SEARCH_RESULT_THREAD_OPEN_EVIDENCE: &str = "RoomScreen message search Thread is a real loaded thread timeline handoff from the active loaded timeline match. The action derives the thread root id from the loaded `MsgLikeContent.thread_root` or loaded thread summary root event id, then dispatches `RoomsListAction::Selected(SelectedRoom::Thread)` for the current room. If that thread timeline is not already loaded, the existing `CreateThreadTimeline` read/open path may run exactly as it does when clicking a timeline thread summary. It sends no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_RESULT_THREAD_OPEN_LABEL: &str =
    "Thread opens only an already loaded thread root via the existing thread timeline path.";
pub const MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_EVIDENCE: &str = "RoomScreen message search Sender is a real loaded profile-pane handoff that opens the existing UserProfileSlidingPane only from the active loaded timeline match. The action builds UserProfilePaneInfo from the loaded sender id, loaded TimelineDetails::Ready(sender_profile) display name/avatar when available, current TimelineKind room id, and a matching local room_members cache row when present. If the room member row is missing, the pane may reuse its existing user_profile_cache GetUserProfile/profile-member read path, exactly like clicking a timeline avatar. It sends no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, message send/edit/redact, profile mutation, direct-message start, room-state, membership mutation, account/profile mutation, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_LABEL: &str =
    "Sender opens the existing profile pane from loaded timeline sender data.";
pub const MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_EVIDENCE: &str = "RoomScreen message search Copy is a real loaded plaintext clipboard handoff that writes the active loaded timeline match plaintext to the local clipboard from plaintext_body_of_timeline_item. The copied payload is derived from already loaded RoomScreen tl_state and local search state: query, loaded item count, match count, active ordinal, active loaded index, event-id availability, plaintext character count, and byte count. It sends no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source open, thread timeline open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_LABEL: &str =
    "Search result Copy uses loaded plaintext and local clipboard only.";
pub const MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE: &str = "RoomScreen message search Source is a real source modal handoff that opens the existing local EventSourceModal from either the active loaded timeline match, the cached Matrix /search server result, or a source-only MatrixRequest::FetchEventSource fallback for a current-room hit with a known event id. The loaded path remains a real loaded source modal handoff and uses the current TimelineKind room id, loaded event id when available, and latest_json from the loaded EventTimelineItem. The server-result path uses the current room id plus the raw event JSON cached from MatrixRequest::SearchMessagesServer when a server hit event id is available. The fallback path submits MatrixRequest::FetchEventSource for the current TimelineKind only, whose worker calls Room::load_or_fetch_event and returns TimelineUpdate::EventSourceFetched for the same EventSourceModal. Missing active match, missing event id, missing latest_json, missing cached server source, and failed source refetch leave Source as local metadata. The source metadata is derived from already loaded RoomScreen tl_state, local search state, and the last server search response: query, loaded item count, match count, active ordinal, loaded index, event-id availability, source origin, JSON character count, JSON line count, and source-only fetch state. It sends no new Matrix-backed search, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, thread timeline open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_RESULT_SOURCE_MODAL_LABEL: &str =
    "Search result Source opens loaded/cached EventSourceModal or requests source-only JSON.";
pub const MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE: &str = "RoomScreen message search query lifecycle metadata stays local to the loaded timeline helper. Opening the strip shows local surface state, every query edit normalizes/trims the query, resets active_match to 0, rescans only already loaded RoomScreen tl_state items, reports loaded item count, match count, active index state, and timeline-loaded state, while Close/Escape clears the query and match vector locally. It sends no Matrix-backed search, server-side history query, event context fetch, timeline pagination or reload, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_LABEL: &str = "Search query lifecycle is local: query edits reset active match and rescan loaded tl_state only.";
pub const MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE: &str = "RoomScreen message search server/context boundary metadata now separates live read paths from remaining context work: Server submits MatrixRequest::SearchMessagesServer, Media can add RoomEventFilter::url_filter=EventsWithUrl, Load older submits the returned next_batch cursor when available, failed Retry resubmits the current query from the first page, Context can use the first cached current-room server hit event id with the existing BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline read path, server result labels parse and surface Matrix /search event_context before/after preview snippets, and Source can open raw event JSON cached from the last server result or submit a source-only MatrixRequest::FetchEventSource fallback for a known current-room event id. Cross-room context, remote date/pins scope adapters, room preview fetch, and full result cursor adapters remain blocked. The row reports query state, loaded item count, local match count, timeline availability, source availability, context-window preview availability, and cursor availability without message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_LABEL: &str = "Server/Older use live Matrix search reads; server context-window previews are parsed; Context paginates cached current-room hits; Source can fetch source-only JSON.";
pub const MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_EVIDENCE: &str = "RoomScreen exposes Server, Context, Older, and Source-backed result actions for message search. Server submits the first live MatrixRequest::SearchMessagesServer page for the current room/query; Media can add RoomEventFilter::url_filter=EventsWithUrl; Older submits the returned next_batch cursor through the same /_matrix/client/v3/search path when a cursor exists; Matrix /search event_context before/after windows are parsed into compact server-result previews; Context parses the first cached server hit event id and reuses the existing BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline current-room timeline read path; Source can open cached raw event JSON returned by Matrix /search without a second request, or submit MatrixRequest::FetchEventSource through Room::load_or_fetch_event when only the current-room event id is known. Cross-room context, room preview fetch, remote date/pins/scope adapters, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain blocked.";
pub const MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_LABEL: &str = "Message search controls: Server/Older are live Matrix search reads; server context-window previews are parsed; Context paginates cached current-room hits; Source opens or source-fetches current-room JSON.";
pub const MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE: &str = "RoomScreen exposes Filter, From, Date, Media, and Pins as visible message-search advanced filter controls while message_search remains a base gap. From is the live sender filter: the sender input stays local until From or Return submits MatrixRequest::SearchMessagesServer with a RoomEventFilter::senders value. Media is the live URL/media filter: Media submits MatrixRequest::SearchMessagesServer with RoomEventFilter::url_filter=EventsWithUrl so the Matrix /search request asks for message events with a URL-backed media source. Older/Retry reuse the last sender/media filter with the same Matrix /search path. Filter, Date, and Pins are live loaded-scope filters over already loaded RoomScreen tl_state: Filter restores all loaded message matches, Date limits local matches to the latest loaded-day timestamp window, and Pins limits local matches to event ids already received from the SubscribeToPinnedEvents read subscription. They submit no remote date index query, pinned event fetch, PinEvent, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, room preview fetch, event source open, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_LABEL: &str = "Advanced search filters: From sender and Media URL filters are live server reads; Filter, Date, and Pins are live loaded-scope filters.";
pub const MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE: &str = "RoomScreen exposes Server query, Packet, Contract, Result, Error, Retry, Scope, and Taxonomy beside the live message-search server controls. Server query submits the first MatrixRequest::SearchMessagesServer page, Retry resubmits the current query from the first page after an error through the same /_matrix/client/v3/search worker path, Older owns next_batch pagination, and Context owns cached current-room hit pagination. Packet/Contract/Result/Error/Scope/Taxonomy remain local metadata views. Taxonomy records remote date/pins/scope/full-result slots before any remote adapter can be promoted. The live controls do not submit remote date index query, remote pinned event fetch, cross-room scope search, full remote result adapter work, remote event context fetch, timeline reload outside BackwardsPaginateUntilEvent, search scope fetch, room preview fetch, event source open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL: &str = "Server search preflight: Server query and Retry use live Matrix search reads; Packet, Contract, Result, Error, Scope, and Taxonomy stay local.";
pub const MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_EVIDENCE: &str = "RoomScreen message search Packet copies only a local server query/result packet snapshot to the local clipboard. The payload is built from already loaded RoomScreen tl_state and local search state: query, loaded item count, local match count, active match, timeline availability, pinned-event count, server/context metadata, and server preflight metadata. It creates no Matrix search request body, allocates no result cursor, submits no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, retry automation, result pagination, search scope fetch, room preview fetch, event source open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_LABEL: &str =
    "Search Packet copies loaded query/result preflight state to local clipboard only.";
pub const MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE: &str = "RoomScreen message search Contract renders only a local typed Matrix search acceptance contract from the loaded query/result packet and cached server preflight metadata. The contract names request slots for room scope, query term, keys, order, limit, filters, next_batch cursor, and event-context window; result slots for event id, sender, timestamp, snippet, highlights, context, source availability, and pagination; error slots for forbidden, rate-limited, offline, timeout, malformed query, and empty result; retry slots for confirmation, idempotency, and stale cursor; and scope/cursor promotion blockers before any real Matrix search adapter can be wired. It builds no Matrix search request body, allocates no result cursor, submits no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, retry automation, result pagination, search scope fetch, room preview fetch, event source open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_LABEL: &str = "Search Contract maps Packet to typed request/result/error/retry/scope/cursor acceptance locally.";
pub const MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomScreen message search Taxonomy renders only a local remote date/pins/scope/full-result taxonomy packet from the loaded query/result packet, current server/context metadata, pinned-event subscription count, and cached Matrix /search state. The packet names current live references as MatrixRequest::SearchMessagesServer first page, next_batch Older pagination, failed Retry first-page resubmit, From sender filter, Media url filter, parsed Matrix /search event_context preview snippets, current-room BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline Context pagination, cached/raw-or-refetched EventSourceModal Source, loaded Jump/Copy/Thread/Sender handoffs, and loaded-scope Filter/Date/Pins over existing timeline rows and SubscribeToPinnedEvents ids. It records remote_date_index_operation_id, remote_pinned_fetch_operation_id, cross_room_scope_request_id, full_result_cursor_id, full_result_page_id, sort_order_result, room_preview_result, non_current_room_context_result, full_result_render_result, stale_query_result, retry_cancel_result, and audit redaction slots as not_assigned or not_wired before remote search adapters can be promoted. It submits no extra Matrix search beyond the existing explicit Server/Older/Retry/From/Media controls, no remote date index query, no remote pinned event fetch, no PinEvent, no cross-room scope search, no room preview fetch, no non-current-room event context fetch, no full result adapter rendering, no retry automation, no message send/edit/redact, no room-state or membership mutation, no account/profile mutation, no gateway/runtime/auth/provider call, and no live mutation.";
pub const MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_LABEL: &str =
    "Search Taxonomy records remote date/pins/scope/full-result slots locally.";
pub const ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL: &str =
    "Timeline send state: SDK queue progress/error/sent from local echo.";

/// The maximum number of timeline items to search through
/// when looking for a particular event.
///
/// This is a safety measure to prevent the main UI thread
/// from getting into a long-running loop if an event cannot be found quickly.
const MAX_ITEMS_TO_SEARCH_THROUGH: usize = 100;
#[allow(dead_code)]
const ROOM_MEMBERS_READ_EVIDENCE: &str = "Member count uses existing SyncRoomMemberList plus GetRoomMembers local_only local cache; this info strip sends no JoinRoom, LeaveRoom, InviteUser, Knock, message, room-state, or membership mutation request.";
#[allow(dead_code)]
const ROOM_MEMBER_SYNC_READ_EVIDENCE: &str = "Member sync uses existing SyncRoomMemberList before local GetRoomMembers; it only refreshes local member profiles and sends no JoinRoom, LeaveRoom, InviteUser, Knock, message, room-state, or membership mutation request from Room info.";
#[allow(dead_code)]
const ROOM_POWER_LEVELS_READ_EVIDENCE: &str = "Permissions use existing GetRoomPowerLevels to display local UserPowerLevels for send, react, and @room permission state; this settings strip sends no power-level, room-state, message, or membership mutation request.";
const ROOM_MEMBERS_COMPACT_LABEL: &str = "members from local SDK cache";
const ROOM_POWER_LEVELS_COMPACT_LABEL: &str = "Permissions read-only from loaded power levels";
pub const ROOM_SETTINGS_LOCAL_BOUNDARY_EVIDENCE: &str = "RoomScreen room settings now shows a compact partial-live summary from already loaded room name/id, RoomsList canonical alias/avatar/tombstone state, local room_members cache, the existing GetRoomPowerLevels/GetRoomMembers read path, confirmed m.room.name/m.room.topic/m.room.canonical_alias write controls, confirmed room avatar upload/removal, confirmed m.room.history_visibility/m.room.join_rules preset writes, and confirmed m.room.tombstone replacement writes. Opening settings, copying Name, Identity, Permissions, and Members, viewing Topic, and Close only update local labels, clipboard text, and preview copy. Save name and Save topic open PositiveConfirmationModal before MatrixRequest::SetRoomName or MatrixRequest::SetRoomTopic submits a Matrix SDK room-state write; Save alias opens PositiveConfirmationModal before MatrixRequest::SetRoomCanonicalAlias submits Room::send_state_event(RoomCanonicalAliasEventContent) while preserving loaded alt aliases; Avatar edit validates a desktop image selection and confirms before MatrixRequest::UploadRoomAvatar submits Room::upload_avatar; Remove avatar requires a loaded avatar and opens PositiveConfirmationModal before MatrixRequest::RemoveRoomAvatar submits Room::remove_avatar; History confirms before MatrixRequest::SetRoomHistoryVisibility submits Room::send_state_event(RoomHistoryVisibilityEventContent); Join rule confirms before MatrixRequest::SetRoomJoinRule submits Room::send_state_event(RoomJoinRulesEventContent); Tombstone validates a replacement Matrix room id and confirms before MatrixRequest::SetRoomTombstone submits Room::send_state_event(RoomTombstoneEventContent). power levels, membership moderation, invite, kick, ban, knock, notification rule, message send/edit/redact, account/profile, gateway/runtime/auth, Telegram delivery, and broader live mutation requests remain blocked.";
pub const ROOM_SETTINGS_LOCAL_BOUNDARY_LABEL: &str = "Room settings partial-live: name/topic/avatar/alias/history/join-rule/tombstone writes require confirmation; power/member edits stay blocked.";
pub const ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE: &str = "RoomScreen room settings Save name, Save topic, Save alias, Avatar edit, Remove avatar, History, Join rule, and Tombstone are wired to live Matrix room-state writes. The text inputs keep only local drafts until Save or Return opens PositiveConfirmationModal; accepting submits MatrixRequest::SetRoomName, MatrixRequest::SetRoomTopic, MatrixRequest::SetRoomCanonicalAlias, or MatrixRequest::SetRoomTombstone for the loaded TimelineKind. Canonical alias writes parse the draft room alias, preserve the loaded alternative alias list, and send m.room.canonical_alias through Room::send_state_event(RoomCanonicalAliasEventContent). Tombstone writes validate the draft replacement room id, build a local replacement body, and send m.room.tombstone through Room::send_state_event(RoomTombstoneEventContent). Avatar edit opens a desktop image picker, validates the selected local image, previews filename/MIME/size/extension metadata, then opens PositiveConfirmationModal before MatrixRequest::UploadRoomAvatar submits Room::upload_avatar for the loaded TimelineKind; the SDK uploads media and sets m.room.avatar through Room::set_avatar_url. Remove avatar first requires loaded room-list avatar identity, then opens PositiveConfirmationModal and submits MatrixRequest::RemoveRoomAvatar for the loaded TimelineKind. History opens PositiveConfirmationModal before MatrixRequest::SetRoomHistoryVisibility submits m.room.history_visibility through Room::send_state_event(RoomHistoryVisibilityEventContent). Join rule opens PositiveConfirmationModal before MatrixRequest::SetRoomJoinRule submits m.room.join_rules through Room::send_state_event(RoomJoinRulesEventContent). SlidingSync calls Room::set_name, Room::set_room_topic, Room::send_state_event(RoomCanonicalAliasEventContent), Room::upload_avatar, Room::remove_avatar, or Room::send_state_event for these fields and returns TimelineUpdate::RoomSettingsMutationResult with success/error metadata. Failed writes cache the field/value; failed-state Retry opens PositiveConfirmationModal and then resubmits the cached MatrixRequest::SetRoomName, MatrixRequest::SetRoomTopic, MatrixRequest::SetRoomCanonicalAlias, MatrixRequest::UploadRoomAvatar, MatrixRequest::RemoveRoomAvatar, MatrixRequest::SetRoomHistoryVisibility, MatrixRequest::SetRoomJoinRule, or MatrixRequest::SetRoomTombstone through the same submit path. The path sends no power levels, membership moderation, invite, kick, ban, knock, notification-rule handoff, message mutation, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or unrelated live mutation.";
pub const ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL: &str = "Name/topic/avatar/alias/history/join-rule/tombstone live writes use PositiveConfirmationModal and Matrix SDK room state only.";
pub const ROOM_SETTINGS_NAME_ID_CLIPBOARD_EVIDENCE: &str = "RoomScreen room settings Name copies only the already loaded current room display label and Matrix room id from RoomNameId to the local clipboard. The payload is derived from the active RoomScreen selection and partial-live settings strip metadata; missing room id stays local-unavailable and writes no clipboard payload. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.power_levels, membership list write, invite, kick, ban, knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ROOM_SETTINGS_NAME_ID_CLIPBOARD_LABEL: &str =
    "Room settings Name copies loaded room label/id to local clipboard only.";
pub const ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE: &str = "RoomScreen room settings Permissions copies only the already loaded current-user permission summary from RoomScreen tl_state.user_power to the local clipboard: send message, send reaction, and @room notification allowance. Missing power-level state stays local-unavailable and writes no clipboard payload. It reuses the existing GetRoomPowerLevels read result and sends no m.room.power_levels mutation, m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, membership list write, invite, kick, ban, knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_LABEL: &str =
    "Room settings Permissions copies loaded power-level summary to local clipboard only.";
pub const ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE: &str = "RoomScreen room settings Members copies only the already loaded room_members cache summary to the local clipboard: loaded member count and a compact display-name/user-id sample. Missing member cache stays local-unavailable and writes no clipboard payload. It reuses the existing GetRoomMembers(server-backed refresh) / SyncRoomMemberList read result and sends no membership list write, invite, kick, ban, knock, m.room.member mutation, m.room.power_levels mutation, m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ROOM_SETTINGS_MEMBERS_CLIPBOARD_LABEL: &str =
    "Room settings Members copies loaded local member-cache summary to local clipboard only.";
pub const ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE: &str = "RoomScreen room settings Identity copies only already loaded RoomsList RoomContextMenuDetails identity metadata plus RoomScreen member-cache availability to the local clipboard: current room label/id, canonical alias presence/value, alternative alias count, avatar cache state, tombstone state, and loaded member count. Missing RoomsList identity metadata stays local-unavailable and writes no clipboard payload. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.tombstone, m.room.power_levels, membership list write, invite, kick, ban, knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ROOM_SETTINGS_IDENTITY_CLIPBOARD_LABEL: &str =
    "Room settings Identity copies loaded room-list identity metadata to local clipboard only.";
pub const ROOM_SETTINGS_LOADED_IDENTITY_EVIDENCE: &str = "RoomScreen room settings identity preview reuses loaded RoomsList metadata for canonical alias presence, alternative alias count, avatar cache state, tombstone state, and room name/id, plus loaded member count from RoomScreen tl_state. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.tombstone, m.room.power_levels, membership, notification rule, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ROOM_SETTINGS_LOADED_IDENTITY_LABEL: &str =
    "Identity preview uses loaded room-list state only; no room-state fetch or mutation.";
pub const ROOM_SETTINGS_CLOSE_METADATA_EVIDENCE: &str = "RoomScreen room settings close metadata is derived only from the current local option-staging state, loaded RoomsList identity availability, local room_members cache count, and current power-level display readiness before hiding the local strip. Close does not submit m.room.name, m.room.topic, m.room.power_levels, membership, invite, kick, ban, knock, room avatar, canonical alias, notification rule, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation requests.";
pub const ROOM_SETTINGS_CLOSE_METADATA_LABEL: &str =
    "Close metadata is local only; no Matrix room-state request.";
pub const ROOM_SETTINGS_REFRESH_METADATA_EVIDENCE: &str = "RoomScreen room settings Refresh reuses only existing read paths for the current loaded timeline: MatrixRequest::GetRoomPowerLevels and MatrixRequest::GetRoomMembers(server-backed). The refresh metadata summarizes timeline availability, loaded RoomsList identity metadata, cached member count, and power-level display readiness before the refreshed results arrive. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.tombstone, m.room.power_levels mutation, invite, kick, ban, knock, notification rule write, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_EVIDENCE: &str = "RoomScreen room settings Refresh has live Matrix read wiring: when a timeline is loaded it submits MatrixRequest::GetRoomPowerLevels and MatrixRequest::GetRoomMembers with local_only=false/JOIN membership, then renders TimelineUpdate::UserPowerLevels and TimelineUpdate::RoomMembersListFetched into the settings strip. Editable m.room.* and membership writes remain blocked behind the room-state mutation contract.";
pub const ROOM_SETTINGS_REFRESH_METADATA_LABEL: &str = "Refresh re-reads power levels and server members; Name/Topic/avatar/alias/history/join-rule/tombstone writes use confirmed live room-state path.";
pub const ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE: &str = "RoomScreen room settings edit-controls boundary metadata is derived from the current partial-live settings strip: room label, loaded RoomsList identity readiness, cached member count, power-level display readiness, confirmed Name/Topic/Alias write controls, confirmed avatar upload/removal controls, confirmed History/Join rule preset controls, and confirmed Tombstone replacement controls. Save name and Save topic are live only after PositiveConfirmationModal, then MatrixRequest::SetRoomName or MatrixRequest::SetRoomTopic reaches SlidingSync and RoomSettingsMutationResult. Save alias is live only after PositiveConfirmationModal, then MatrixRequest::SetRoomCanonicalAlias reaches SlidingSync, preserves loaded alt aliases, sends RoomCanonicalAliasEventContent, and returns RoomSettingsMutationResult. Avatar edit is live only after a valid desktop image selection and PositiveConfirmationModal accepts MatrixRequest::UploadRoomAvatar. Remove avatar is live only when loaded room-list avatar identity exists and PositiveConfirmationModal accepts MatrixRequest::RemoveRoomAvatar. History visibility and Join rule are live only after PositiveConfirmationModal accepts MatrixRequest::SetRoomHistoryVisibility or MatrixRequest::SetRoomJoinRule. Tombstone is live only after a valid replacement Matrix room id and PositiveConfirmationModal accepts MatrixRequest::SetRoomTombstone. Power levels, Member moderation, Invite/Kick/Ban/Knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, Telegram delivery, and unrelated live mutation controls remain local blocked.";
pub const ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_LABEL: &str = "Name/Topic, alias, avatar upload/remove, history, join-rule, and tombstone writes confirm first; power levels and member moderation stay blocked.";
pub const ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE: &str = "RoomScreen room settings edit-intent staging keeps Power and Moderation as local buttons that only update the partial-live settings strip status using loaded room identity, cached member count, and power-level display readiness. Alias now uses the confirmed canonical alias write path through MatrixRequest::SetRoomCanonicalAlias. Avatar opens the confirmed room-avatar upload path, Name and Topic use the separate confirmed Save path through MatrixRequest::SetRoomName/SetRoomTopic, History/Join rule use separate confirmed preset writes through MatrixRequest::SetRoomHistoryVisibility/SetRoomJoinRule, and Tombstone uses the confirmed replacement-room write path through MatrixRequest::SetRoomTombstone. Power and Moderation remain product placeholders for the other editable room-state fields and do not submit m.room.power_levels, member moderation, invite, kick, ban, knock, notification-rule, message send/edit/redact, account/profile, gateway/runtime/auth, Telegram delivery, or unrelated live mutation requests.";
pub const ROOM_SETTINGS_EDIT_INTENT_STAGING_LABEL: &str = "Edit intent staged locally for remaining fields; Name/Topic, Alias, Avatar, History, Join rule, and Tombstone use confirmed live writes.";
pub const ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE: &str = "RoomScreen room settings field edit-intent controls add visible Name, Topic, Alias, Avatar, Remove avatar, Permissions, and Members buttons in the settings strip. Name, Topic, and Alias field intents stage draft/metadata before the separate confirmed Save path; Alias confirms before MatrixRequest::SetRoomCanonicalAlias submits RoomCanonicalAliasEventContent while preserving loaded alt aliases. Avatar opens a desktop image picker and confirms before MatrixRequest::UploadRoomAvatar submits Room::upload_avatar; Permissions and Members only update local field edit-intent metadata, settings strip status, and popup copy from loaded room identity, cached member count, and power-level display readiness. Remove avatar requires loaded avatar identity and opens PositiveConfirmationModal before MatrixRequest::RemoveRoomAvatar submits Room::remove_avatar. The settings options/write rows separately expose confirmed History, Join rule, and Tombstone writes. These field-intent controls do not submit power-level or membership writes, invite, kick, ban, knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, Telegram delivery, or unrelated live mutation requests.";
pub const ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_LABEL: &str = "Name/Topic/Alias field intents stage drafts; Avatar upload and Remove avatar confirm live; Permissions and Members stay local.";
pub const ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE: &str = "RoomScreen room settings refresh result detail adds visible Result, Members, Power, Failure, and Source local buttons in the settings strip. Clicking any one only updates local refresh result metadata, settings strip status, and popup copy from current timeline availability, loaded room identity, cached member count, power-level display state, and local status text. Refresh remains the only control that reuses MatrixRequest::GetRoomPowerLevels and MatrixRequest::GetRoomMembers(server-backed); the detail buttons do not submit extra reads, m.room.name, m.room.topic, m.room.avatar, m.room.power_levels mutation, membership list writes, invite, kick, ban, knock, canonical alias, history visibility, join rule, tombstone, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation requests.";
pub const ROOM_SETTINGS_REFRESH_RESULT_DETAIL_LABEL: &str = "Refresh result detail stays local: Result, Members, Power, Failure, and Source do not write room state.";
pub const ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "RoomScreen room settings mutation preflight detail adds visible Request, Packet, Contract, Taxonomy, Result, Error, Retry, and Source local buttons in the settings strip. Request renders a local room-state mutation packet snapshot from current timeline availability, loaded room identity, cached member count, power-level display state, and the last local edit-intent or refresh status. Packet copies field-by-field local acceptance criteria, Contract copies typed room-state mutation/result contracts, Taxonomy copies power/member permission-denial and result taxonomy slots, and Result, Error, Retry, and Source only update local mutation-preflight metadata, settings strip status, and popup copy from the same loaded state. The controls do not submit m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership list writes, invite, kick, ban, knock, tombstone, notification-rule handoff, retry automation, room-state mutation contract calls, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation requests. Request, Result, Error, Retry, and Source remain local-only status controls.";
pub const ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str = "Room-state mutation preflight stays local: Request, Packet, Contract, Taxonomy, Result, Error, Retry, and Source do not write m.room.* state.";
pub const ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE: &str = "RoomScreen room settings Packet copies only a local field-by-field room-state mutation packet to the clipboard. The payload is derived from current timeline availability, loaded RoomsList identity readiness, cached member count, current power-level display state, and local settings/preflight status. It lists confirmation, request, result, error, retry, and source acceptance slots for m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias/aliases, m.room.history_visibility, m.room.join_rules, confirmed m.room.tombstone replacement writes, m.room.power_levels, m.room.member moderation, and notification handoff boundaries. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership write, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, message mutation, account/profile, gateway/runtime/auth, or live mutation.";
pub const ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_LABEL: &str =
    "Room settings Packet copies field-by-field room-state mutation acceptance criteria locally.";
pub const ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE: &str = "RoomScreen room settings Contract copies only a local typed room-state mutation/result contract packet to the clipboard. The payload is derived from current timeline availability, loaded RoomsList identity readiness, cached member count, current power-level display state, the local settings/preflight status, and the existing field mutation packet boundary. It maps m.room.name, m.room.topic, m.room.avatar, current history visibility/join-rule/tombstone preset writes, canonical alias/aliases, power levels, member moderation, and notification handoff to typed request/result/error/retry/source contracts while broader room-state expansion stays contract-first. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership write, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, message mutation, account/profile, gateway/runtime/auth, or live mutation.";
pub const ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_LABEL: &str = "Room settings Contract maps the field packet to typed room-state mutation/result contracts locally.";
pub const ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomScreen room settings Taxonomy copies only a local power/member permission-denial and result taxonomy packet to the clipboard. The payload is derived from current timeline availability, loaded RoomsList identity readiness, cached member count, current power-level display state, local settings/preflight status, and the existing field mutation contract boundary. Existing live references are limited to confirmed Name/Topic/Alias/avatar/history/join-rule/tombstone result paths and Refresh GetRoomPowerLevels/GetRoomMembers read paths; m.room.power_levels and m.room.member moderation slots record operation_id_slot not_assigned, permission_denied/forbidden/stale-baseline/invalid-delta/already-in-state/not-wired result mapping, retry/source-hash/stale-room policy, cancel policy, rollback/audit slots, and redaction requirements. It sends no m.room.power_levels write, m.room.member mutation, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, account/profile, gateway/runtime/auth, Telegram delivery, or live mutation.";
pub const ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_LABEL: &str =
    "Room settings Taxonomy maps power/member permission denial and result slots locally.";
pub const ROOM_SETTINGS_COMPACT_LABEL: &str = "Settings partial-live: Name/Topic/Alias, avatar, history, join-rule, and tombstone writes run after confirmation; power/member edits stay blocked.";
#[allow(dead_code)]
const PINNED_EVENTS_SUBSCRIPTION_READ_EVIDENCE: &str = "Pinned event count uses existing SubscribeToPinnedEvents for local pinned-event updates; this info strip sends no PinEvent, message, room-state, or membership request.";
#[allow(dead_code)]
const TYPING_NOTICES_SUBSCRIPTION_READ_EVIDENCE: &str = "Incoming typing notice display uses existing SubscribeToTypingNotices for local typing-user updates; this info strip sends no typing notice, message, room-state, or membership request.";
#[allow(dead_code)]
const OWN_READ_RECEIPT_SUBSCRIPTION_READ_EVIDENCE: &str = "Own read marker uses existing SubscribeToOwnUserReadReceiptsChanged for local marker updates; this info strip sends no ReadReceipt, message, room-state, or membership request.";
const ROOM_PINNED_COMPACT_LABEL: &str = "Pinned count follows the live timeline subscription.";
const ROOM_TYPING_COMPACT_LABEL: &str = "Typing display follows the live typing subscription.";
const ROOM_READ_RECEIPT_COMPACT_LABEL: &str =
    "Own read marker follows the local receipt subscription.";
const ROOM_UNREAD_COMPACT_LABEL: &str = "Unread badge is read-only local badge state.";
const ROOM_AVATAR_COMPACT_LABEL: &str = "Avatar uses the existing cache read path.";
#[allow(dead_code)]
const ROOM_AVATAR_FETCH_CACHE_READ_EVIDENCE: &str = avatar_cache::AVATAR_FETCH_CACHE_READ_EVIDENCE;
pub const MESSAGE_COPY_CLIPBOARD_EVIDENCE: &str = "Message Copy Text, Copy Text as HTML, and Copy Link use loaded timeline item data or locally constructed matrix.to URIs to write clipboard text locally; they send no Matrix event fetch, message send, edit, redact, room-state, membership, or live mutation request.";
pub const MESSAGE_COPY_CLIPBOARD_COMPACT_LABEL: &str = "Copied locally from the loaded timeline.";
pub const MESSAGE_COPY_LOADED_METADATA_EVIDENCE: &str = "Message copy popups summarize already loaded clipboard payload metadata for Copy Text, Copy Text as HTML, and Copy Link: payload kind, event-id availability, character count, and byte count. The summary is derived from the same loaded timeline body, loaded formatted body, or locally constructed matrix.to URI that is copied to the local clipboard and sends no Matrix event fetch, event source request, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_COPY_LOADED_METADATA_LABEL: &str =
    "Clipboard payload metadata is local; no event fetch.";
pub const EXTERNAL_LINK_CONFIRMATION_EVIDENCE: &str = "External ordinary URL links and unhandled Matrix links open a local confirmation guard before the existing system browser handoff; opening the confirmation, Cancel, and guard display send no browser handoff, Matrix event fetch, room preview fetch, message send, room-state, membership, or live mutation request.";
pub const EXTERNAL_LINK_CONFIRMATION_COMPACT_LABEL: &str =
    "External link opens only after confirmation.";
pub const MATRIX_LINK_LOCAL_PREVIEW_EVIDENCE: &str = "RoomScreen Matrix link handling keeps known room links on local room navigation, loaded room alias links on local RoomsList alias navigation, known user links on the profile pane handoff and its existing profile read path, and current-room event links on a loaded local jump when the event id is already present in RoomScreen tl_state or on the existing BackwardsPaginateUntilEvent/PaginateTimeline read path when the event is missing from loaded rows. Unknown room ids, unknown room aliases, non-current-room event links, and other event links stay on a compact MatrixRequest::PreviewMatrixLinkTarget read path routed back to the originating TimelineKind. Unknown room ids and aliases fetch room preview details through the existing get_room_preview path; event links can additionally fetch source JSON through Room::load_or_fetch_event when the previewed room is known to the current client. Cached room id or alias targets can refresh Server context through the same compact PreviewMatrixLinkTarget read or be promoted through PositiveConfirmationModal to MatrixRequest::JoinRoomByIdOrAlias or MatrixRequest::Knock, with RoomScreen consuming MatrixLinkJoinResultAction or KnockResultAction for the Matrix link strip. Cached Matrix user targets can be promoted through PositiveConfirmationModal to MatrixRequest::InviteUser for the current room, with InviteResultAction rendered back into the same Matrix link strip. Link parsing, preview staging, known-room navigation, loaded-alias navigation, profile-pane handoff, loaded-event local jump, current-room event pagination, compact room preview, source-only event fetch for known previewed rooms, preview result display, cached Server context refresh, failed-state Retry confirmation, confirmed room-or-alias Join, confirmed room-or-alias Knock, and confirmed current-room user Invite send no server-side event context window, external browser handoff before confirmation, message, room-state, account/profile, gateway/runtime/auth, or unrelated live mutation request.";
pub const MATRIX_LINK_UNKNOWN_TARGET_LOCAL_BOUNDARY_EVIDENCE: &str = "RoomScreen keeps unknown Matrix room ids, unknown room aliases, non-current-room event links, and other unresolved Matrix targets on a compact Matrix link preview read while matrix_link_resolution remains a base gap. Unknown room ids and unknown room aliases use MatrixRequest::PreviewMatrixLinkTarget to fetch room preview details; non-current-room event links fetch only the containing room preview. Cached room id or alias targets can be refreshed from the Server context control through the same MatrixRequest::PreviewMatrixLinkTarget read, confirmed into MatrixRequest::JoinRoomByIdOrAlias, or confirmed into MatrixRequest::Knock with via servers, and cached Matrix user targets can be confirmed into MatrixRequest::InviteUser for the current room. Server-side event context remains blocked. Opening an external browser before confirmation, sending messages, mutating room-state, touching account/profile, gateway/runtime/auth, and unrelated live mutation paths remain unwired.";
pub const MATRIX_LINK_UNKNOWN_TARGET_LOCAL_BOUNDARY_LABEL: &str = "Unknown Matrix link target: compact room preview plus confirmed room-or-alias Join/Knock; event context, browser handoff, and mutation stay bounded.";
pub const MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_EVIDENCE: &str = "RoomScreen Matrix link Knock parses a cached room id or alias target from the preview strip, opens PositiveConfirmationModal, and submits MatrixRequest::Knock with no reason only from the accept branch. KnockResultAction success/failure is rendered back into the same Matrix link strip with a failed-state retry cache preserving the room id or alias plus via servers. Invite targets, event context fetch, browser handoff, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation remain blocked; cached Server context refresh stays read-only through PreviewMatrixLinkTarget.";
pub const MATRIX_LINK_KNOCK_ROOM_CONFIRMATION_LABEL: &str =
    "Matrix link Knock confirms before MatrixRequest::Knock for cached room ids or aliases.";
pub const MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_EVIDENCE: &str = "Current-room Matrix event links jump locally when the target event id is already loaded in RoomScreen tl_state; the helper scrolls and highlights the loaded row without server-side event context fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request. Missing current-room event links now use the existing BackwardsPaginateUntilEvent/PaginateTimeline read path; non-current-room event links continue to use the compact MatrixRequest::PreviewMatrixLinkTarget room-preview read and keep event context as a gap.";
pub const MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_LABEL: &str = "Loaded current-room event link jumped locally; missing current-room events paginate read-only.";
pub const MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_EVIDENCE: &str = "Missing current-room Matrix event links stage the Matrix link preview strip, then reuse the existing BackwardsPaginateUntilEvent request sender and MatrixRequest::PaginateTimeline read path to load older timeline items until the event appears. TargetEventFound scrolls and highlights the row and refreshes the Matrix link preview strip to loaded/source-ready state. Non-current-room event links, server-side alias resolution, server-side event context fetch, join/knock/invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain blocked.";
pub const MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_LABEL: &str =
    "Missing current-room event link paginates older timeline items read-only.";
pub const MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_EVIDENCE: &str = "RoomScreen Matrix link loaded-event context metadata is derived only from the already loaded timeline row selected by jump_to_loaded_matrix_link_event. It shows the target event id, loaded item index, current-room relation, loaded event-id availability, loaded plaintext snippet, local scroll/highlight action, and visible Matrix link preview-strip source affordance before the row is highlighted. It sends no MatrixRequest::BackwardsPaginateUntilEvent, PreviewMatrixLinkTarget follow-up, event-context fetch, timeline pagination/reload, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_LABEL: &str = "Loaded event context metadata only; no event-context fetch, pagination, source-only preview event source fetch stays on compact preview; full remote event source fetch, or mutation.";
pub const MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE: &str = "RoomScreen Matrix link Source opens the existing local EventSourceModal for either a current-room Matrix event link already loaded in RoomScreen tl_state or cached source JSON returned by MatrixRequest::PreviewMatrixLinkTarget for a known previewed room event. The loaded action uses the cached Matrix link preview target, current TimelineKind room id, loaded event id, loaded item index, and loaded EventTimelineItem.latest_json from the visible timeline cache when available. The preview-fetched action uses the preview result room id, requested event id, and source JSON fetched through Room::load_or_fetch_event. Missing, failed, unresolved, or source-less links keep Source as local metadata and send no Source-click follow-up request, BackwardsPaginateUntilEvent, event-context window fetch, timeline pagination/reload, join, knock, invite, external browser handoff, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_LABEL: &str =
    "Matrix link Source opens loaded current-room JSON or preview-fetched event JSON.";
pub const MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_EVIDENCE: &str = "Room alias Matrix links navigate locally only when the alias matches a loaded RoomsList canonical_alias or alt_aliases entry. The helper scans already loaded joined-room metadata and emits NavigateToRoom for that room without MatrixRequest::PreviewMatrixLinkTarget, server-side alias resolution, join, knock, invite, event context fetch, timeline pagination/reload, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request; unknown aliases continue to use compact room-preview metadata.";
pub const MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_LABEL: &str =
    "Loaded room alias opened locally; unknown aliases stay preview-only.";
pub const MATRIX_LINK_TARGET_METADATA_EVIDENCE: &str = "RoomScreen Matrix link target metadata is computed from the clicked MatrixId, via server list, current RoomScreen room id, loaded RoomsList room/alias state, and already loaded timeline event ids before any action is taken. The popup metadata labels target kind, via count, current-room relation, loaded target state, event-id loaded state, and whether the path is local navigation, local scroll/highlight, profile-pane handoff, or compact PreviewMatrixLinkTarget room-preview read. It sends no extra Matrix request beyond the existing compact preview read for unknown targets, and no server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MATRIX_LINK_TARGET_METADATA_LABEL: &str =
    "Target metadata is loaded locally; unresolved targets stay compact preview-only.";
pub const MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE: &str = "Matrix link compact preview result metadata is summarized after the existing PreviewMatrixLinkTarget get_room_preview read returns FetchedRoomPreview. The popup adds already fetched canonical alias presence, topic state, joined and active member counts, room type, join rule, world-readable history flag, current-user room state, direct-room flag, hero count, avatar fetch/fallback state, and the source-only Room::load_or_fetch_event status when an event id was requested for a room known to the current client. It sends no server-side alias resolution, event context window fetch, timeline pagination/reload, join, knock, invite, message send/edit/redact, room-state mutation, membership change, account/profile request, gateway/runtime/auth request, or live mutation.";
pub const MATRIX_LINK_PREVIEW_RESULT_METADATA_LABEL: &str = "Preview result metadata uses fetched room preview plus source-only status; event context remains unwired.";
pub const MATRIX_LINK_PREVIEW_FAILURE_METADATA_EVIDENCE: &str = "Matrix link compact preview failure metadata is summarized only after the existing PreviewMatrixLinkTarget get_room_preview read returns an error. The RoomScreen preview strip keeps the target, via server count, requested event-id state, error message length, and boundary note visible. Failed Retry reuses only the cached originating TimelineKind, room-or-alias id, via list, and optional event id, opens PositiveConfirmationModal first, and only submits the same compact PreviewMatrixLinkTarget read after confirmation. It sends no retry without confirmation, no follow-up Matrix request beyond the confirmed compact preview read, no server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, message send/edit/redact, room-state mutation, membership change, account/profile request, gateway/runtime/auth request, or live mutation.";
pub const MATRIX_LINK_PREVIEW_FAILURE_METADATA_LABEL: &str =
    "Preview failure metadata is error-only; Retry confirms before the same compact preview read.";
pub const MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_EVIDENCE: &str = "Matrix link compact preview failed-state Retry reuses only the cached originating TimelineKind, room-or-alias id, via list, and optional event id from the failed PreviewMatrixLinkTarget attempt. Retry opens PositiveConfirmationModal before another compact room-preview read is submitted; unavailable cached target, unavailable TimelineKind, and confirmation cancel stay local. It sends no automatic retry, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, message send/edit/redact, room-state mutation, membership change, account/profile request, gateway/runtime/auth request, or live mutation.";
pub const MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_LABEL: &str = "Failed Matrix link Retry confirms before PreviewMatrixLinkTarget; event context, join, knock, browser handoff, and mutation stay unwired.";
pub const MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE: &str = "RoomScreen Matrix link server/context boundary metadata is derived from clicked target metadata and compact PreviewMatrixLinkTarget status: loading, resolved, failed, retry confirmation, confirmed room-or-alias join, confirmed room-or-alias knock, confirmed current-room user invite, cached Server context refresh, or loaded current-room or preview-fetched event source state; via server count, optional event id, and retry cache readiness remain visible. Cached room id or alias targets can refresh Server context through the same MatrixRequest::PreviewMatrixLinkTarget compact room-preview read. event context fetch, timeline pagination/reload, MatrixRequest::BackwardsPaginateUntilEvent outside current-room missing event pagination, external browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation remain local blocked controls while active unresolved-target paths are MatrixRequest::PreviewMatrixLinkTarget for compact room preview metadata or cached Server context refresh, confirmed MatrixRequest::JoinRoomByIdOrAlias for cached room ids or aliases, confirmed MatrixRequest::Knock for cached room ids or aliases, and confirmed MatrixRequest::InviteUser for cached Matrix user ids in the current room.";
pub const MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_LABEL: &str = "Server context refresh uses cached PreviewMatrixLinkTarget read-only; event context window, extra pagination, browser handoff before confirmation, and full remote event source workflow stay blocked; cached room-or-alias Join/Knock, current-room user Invite, and source-only preview fetch are confirmed.";
pub const MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE: &str = "RoomScreen exposes Server, Event, Alias, Join, Knock, Invite, Browser, and Source as visible Matrix link context controls while matrix_link_resolution remains a base gap. Clicking Server uses the cached room id or alias target, via servers, and optional event id to submit the same MatrixRequest::PreviewMatrixLinkTarget compact read as a standalone Server context refresh when a target is cached, and falls back to the local Matrix link server-context packet snapshot when no cached target is available. Clicking Event renders a local Matrix link server-context packet snapshot from the current preview status, target label, via server list, requested event id, metadata/error length, retry cache state, and loaded current-room or preview-fetched source availability. Clicking Alias only updates local Matrix link preview summary metadata, context-action metadata, server/context boundary text, visible strip state, and popup copy derived from the same cached target/retry state. Clicking Join parses the cached room id or alias target, opens PositiveConfirmationModal, and only the accept branch submits MatrixRequest::JoinRoomByIdOrAlias with cached via servers. Clicking Knock parses the same cached room id or alias target, opens PositiveConfirmationModal, and only the accept branch submits MatrixRequest::Knock with cached via servers and no reason. Clicking Invite parses the cached Matrix user target, requires a loaded current room, opens PositiveConfirmationModal, and only the accept branch submits MatrixRequest::InviteUser for that room/user pair. RoomScreen consumes MatrixLinkJoinResultAction, KnockResultAction, or InviteResultAction to render joined/knocked/invited/failed status and retry cache on the same Matrix link strip. Clicking Browser builds a cached matrix.to URL from the preview target/via/event state, opens PositiveConfirmationModal, and only the accept branch hands that URL to the system opener. Clicking Source may open the existing local EventSourceModal when the cached target is a current-room event already loaded in RoomScreen tl_state or when the compact preview result returned source-only JSON for a known previewed room event; otherwise it stays metadata-only. PreviewMatrixLinkTarget is limited to compact preview, confirmed failed-state Retry, or cached Server context refresh. It does not submit MatrixRequest::BackwardsPaginateUntilEvent outside current-room missing event pagination, event context fetch, timeline pagination/reload, unconfirmed invite, unconfirmed external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, account/profile, gateway/runtime/auth, or unrelated live mutation.";
pub const MATRIX_LINK_CONTEXT_ACTIONS_ROW_LABEL: &str = "Matrix link context actions: Server refreshes cached PreviewMatrixLinkTarget, Event/Alias stay metadata, room-or-alias Join/Knock confirm, user Invite confirms, Browser confirms, Source opens loaded-local or preview-fetched.";
pub const MATRIX_LINK_JOIN_ROOM_CONFIRMATION_EVIDENCE: &str = "RoomScreen Matrix link Join parses a cached room id or alias target from the preview strip, opens PositiveConfirmationModal, and submits MatrixRequest::JoinRoomByIdOrAlias only from the accept branch. MatrixLinkJoinResultAction success/failure is rendered back into the same Matrix link strip with a failed-state retry cache preserving the room id or alias plus via servers. Knock and Invite have their own confirmed MatrixRequest::Knock and MatrixRequest::InviteUser paths; event context fetch, browser handoff, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation remain blocked; cached Server context refresh stays read-only through PreviewMatrixLinkTarget.";
pub const MATRIX_LINK_JOIN_ROOM_CONFIRMATION_LABEL: &str = "Matrix link Join confirms before MatrixRequest::JoinRoomByIdOrAlias for cached room ids or aliases.";
pub const MATRIX_LINK_INVITE_USER_CONFIRMATION_EVIDENCE: &str = "RoomScreen Matrix link Invite parses a cached Matrix user id target from the preview strip, requires a loaded current room id, opens PositiveConfirmationModal, and submits MatrixRequest::InviteUser only from the accept branch. InviteResultAction success/failure is rendered back into the same Matrix link strip with a failed-state retry cache preserving the current room id plus user id. Join and Knock have separate room-or-alias confirmation paths; event context fetch, browser handoff, message mutation, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation remain blocked; cached Server context refresh stays read-only through PreviewMatrixLinkTarget.";
pub const MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL: &str = "Matrix link Invite confirms before MatrixRequest::InviteUser for cached Matrix user ids in the current room.";
pub const MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_EVIDENCE: &str = "RoomScreen Matrix link Browser uses only the cached preview target, via server label, and requested event id to build a matrix.to URL. It opens PositiveConfirmationModal first and hands the URL to robius_open system opener only from the accept branch; missing cached target and cancel stay warning-only/local. It submits no PreviewMatrixLinkTarget, BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, Telegram delivery, or live mutation.";
pub const MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_LABEL: &str =
    "Matrix link Browser confirms before matrix.to system opener handoff.";
pub const MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE: &str = "RoomScreen exposes Room, Event, Via, Preview, and Source as visible local Matrix link route-scope controls while matrix_link_resolution remains a base gap, plus Packet, Contract, and Taxonomy controls for local acceptance evidence. Clicking Room copies only the cached Matrix link target label/status/via/event metadata to the local clipboard when a target label exists. Clicking Via copies only the cached Matrix link via server list to the local clipboard when one exists. Clicking Event copies only the cached requested Matrix event id to the local clipboard when an event id exists. Clicking Preview copies only the already cached local preview metadata/status/target/via/event summary to the local clipboard when metadata exists. Clicking Source may open the existing local EventSourceModal when the cached target is a current-room event already loaded in RoomScreen tl_state or when the compact preview result returned source-only JSON for a known previewed room event; otherwise it stays metadata-only. Packet copies per-target route acceptance criteria; Contract copies typed route/result contracts for alias, room, event, preview, join, source, retry, server refresh, and browser handoff; Taxonomy copies route/event-context result slots locally before any richer route adapter is promoted. PreviewMatrixLinkTarget is limited to compact preview, confirmed failed-state Retry, or cached Server context refresh; it sends no MatrixRequest::BackwardsPaginateUntilEvent outside current-room missing event pagination, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MATRIX_LINK_ROUTE_SCOPE_CONTROLS_LABEL: &str = "Matrix link route scope: Room copies cached target metadata; Via copies cached via list; Event copies cached event id; Preview copies cached metadata; Source opens loaded or preview-fetched source; Contract and Taxonomy stay local.";
pub const MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_EVIDENCE: &str = "RoomScreen Matrix link Packet copies a per-target route drilldown from cached preview-strip state only. The packet records room target, event id, via servers, preview metadata, server-context packet, alias resolution, join/knock/invite, external browser handoff, and loaded and preview-fetched source acceptance slots before any route is promoted. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh. It submits no MatrixRequest::BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_LABEL: &str =
    "Matrix link Packet copies per-target route acceptance criteria only.";
pub const MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_EVIDENCE: &str = "RoomScreen Matrix link Contract copies a typed route/result contract packet from cached preview-strip state only. The packet maps room target identity, event id, via servers, preview request/result/error, server-context packet, alias resolution, event context, pagination cursor, join/knock/invite, external browser handoff, loaded source, preview-fetched source, full remote source, retry, and source-hash acceptance slots before any server-context work is promoted. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh. It submits no MatrixRequest::BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_LABEL: &str =
    "Matrix link Contract maps the route drilldown to typed route/result contracts locally.";
pub const MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomScreen Matrix link Taxonomy copies a route/event-context result taxonomy packet from cached preview-strip state only. The packet names existing live references as loaded alias navigation, loaded current-room event jump, current-room missing-event MatrixRequest::BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline read wiring, compact MatrixRequest::PreviewMatrixLinkTarget room-preview read, cached Server context refresh, confirmed failed-state Retry, source-only Room::load_or_fetch_event for known previewed room events, loaded or preview-fetched EventSourceModal Source, confirmed matrix.to Browser system-opener handoff, confirmed JoinRoomByIdOrAlias, confirmed MatrixRequest::Knock, and confirmed current-room MatrixRequest::InviteUser result/retry. It records route_adapter_request_id, alias_resolution_operation_id, non_current_room_event_context_operation_id, via_route_request_id, full_remote_source_request_id, room_preview_route_result, event_context_window_result, alias_resolution_result, via_resolution_result, full_remote_source_result, access_denied_result, stale_target_result, retry_cancel_result, and audit redaction slots as not_assigned or not_wired before a richer route adapter can be promoted. It submits no PreviewMatrixLinkTarget beyond explicit compact preview, Server refresh, or confirmed Retry controls, no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, no event-context fetch, no non-current-room timeline pagination/reload, no full remote source fetch, no unconfirmed browser handoff, no unconfirmed join/knock/invite, no message send/edit/redact, no room-state mutation, no membership mutation outside confirmed join/knock/invite paths, no account/profile mutation, no gateway/runtime/auth/provider call, and no live mutation.";
pub const MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_LABEL: &str =
    "Matrix link Taxonomy maps route/event-context result slots locally.";
pub const MATRIX_LINK_ROOM_TARGET_CLIPBOARD_EVIDENCE: &str = "RoomScreen Matrix link Room copies only the cached Matrix link target label from the preview strip to the local clipboard when one exists, together with preview status, via server count, requested event id, and retry-cache readiness. Missing target label stays local-unavailable and writes no clipboard payload. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; it sends no BackwardsPaginateUntilEvent outside current-room missing event pagination, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MATRIX_LINK_ROOM_TARGET_CLIPBOARD_LABEL: &str =
    "Matrix link Room copies cached target metadata only.";
pub const MATRIX_LINK_VIA_SERVERS_CLIPBOARD_EVIDENCE: &str = "RoomScreen Matrix link Via copies only the cached Matrix link via server list from the preview strip to the local clipboard when one exists, together with preview status, target label, requested event id, and retry-cache readiness. Missing via server list stays local-unavailable and writes no clipboard payload. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; it sends no BackwardsPaginateUntilEvent outside current-room missing event pagination, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MATRIX_LINK_VIA_SERVERS_CLIPBOARD_LABEL: &str =
    "Matrix link Via copies cached via server list only.";
pub const MATRIX_LINK_EVENT_ID_CLIPBOARD_EVIDENCE: &str = "RoomScreen Matrix link Event copies only the cached requested Matrix event id from the preview strip to the local clipboard when one exists. The popup metadata includes preview status, target label, via server count, requested event id, and retry-cache readiness. Missing event id stays local-unavailable and writes no clipboard payload. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; it sends no BackwardsPaginateUntilEvent outside current-room missing event pagination, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MATRIX_LINK_EVENT_ID_CLIPBOARD_LABEL: &str =
    "Matrix link Event copies cached requested event id only.";
pub const MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_EVIDENCE: &str = "RoomScreen Matrix link Preview copies only the already cached preview strip metadata to the local clipboard: preview status, target label, via server count, requested event id, retry-cache readiness, and the current local preview metadata text. Missing metadata stays local-unavailable and writes no clipboard payload. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh; it sends no BackwardsPaginateUntilEvent outside current-room missing event pagination, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_LABEL: &str =
    "Matrix link Preview copies cached local preview metadata only.";
pub const MATRIX_LINK_UNRESOLVED_DETAIL_EVIDENCE: &str = "RoomScreen Matrix link unresolved detail state is derived only from the clicked target metadata, the compact MatrixRequest::PreviewMatrixLinkTarget status, cached retry context, cached room-or-alias join/knock state, cached Server context refresh state, and loaded current-room event source availability. The detail label reports selected Server/Event/Alias/Join/Knock/Source action, preview status, unresolved target, via server count, requested event-id state, metadata character count, optional error character count, and retry cache readiness. Clicking Server can submit a cached MatrixRequest::PreviewMatrixLinkTarget read-only refresh; clicking Join can submit only a confirmed MatrixRequest::JoinRoomByIdOrAlias for cached room id or alias targets; clicking Knock can submit only a confirmed MatrixRequest::Knock for cached room id or alias targets. Other context controls update only this local detail state, the summary, the server/context boundary label, and popup copy. Source may open only the already loaded current-room EventSourceModal. It submits no MatrixRequest::BackwardsPaginateUntilEvent outside current-room missing event pagination, event context fetch, timeline pagination/reload, invite, external browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, account/profile, gateway/runtime/auth, or unrelated live mutation.";
pub const MATRIX_LINK_UNRESOLVED_DETAIL_LABEL: &str = "Unresolved Matrix link detail is local: target, via, event id, metadata chars, error chars, and retry cache only.";
pub const MATRIX_LINK_UNKNOWN_TARGET_COMPACT_LABEL: &str =
    "Matrix link preview reads room metadata only.";
pub const ROOM_STATUS_CONFIRMATION_COMPACT_LABEL: &str =
    "Room status changes run only after confirmation.";
pub const ROOM_LINK_INVITE_LEAVE_COMPACT_LABEL: &str =
    "Room action uses the existing guarded path.";
pub const TIMELINE_INVITE_CONFIRMATION_COMPACT_LABEL: &str =
    "Invite is sent only after confirmation.";
pub const NOTIFICATIONS_OPTION_STAGING_LOCAL_EVIDENCE: &str = "RoomScreen notification surfaces now read the effective Matrix room notification mode through MatrixRequest::GetRoomNotificationMode and open a confirmation guard before supported All messages, Mentions, and Mute writes submit MatrixRequest::SetRoomNotificationMode. Refresh and Close remain read/local UI only. Timed mute choices such as 1h or 8h remain unwired and are not presented as real writes; no message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request is sent.";
pub const NOTIFICATIONS_MODE_WRITE_CONFIRMATION_EVIDENCE: &str = "RoomScreen notifications All messages, Mentions, and Mute options use PositiveConfirmationModal before MatrixRequest::SetRoomNotificationMode writes the current room's Matrix notification mode through NotificationSettings::set_room_notification_mode. The write returns TimelineUpdate::RoomNotificationModeSet, updates only the displayed room notification mode on success, and caches room id plus mode for a failed-state Retry that also requires PositiveConfirmationModal before resubmitting SetRoomNotificationMode. Keyword Add/Remove also uses PositiveConfirmationModal before MatrixRequest::SetNotificationKeywordRule reaches NotificationSettings::add_keyword/remove_keyword. This still sends no timed mute, global preference, pusher, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request beyond confirmed room-mode and confirmed keyword writes.";
pub const NOTIFICATIONS_LOADED_ATTENTION_EVIDENCE: &str = "RoomScreen notifications strip reflects already loaded RoomsList unread count, mention count, and manual unread state next to the current-room notification mode. This preview sends no notification rule read beyond MatrixRequest::GetRoomNotificationMode, timed mute, global notification preference, keyword, push gateway/device, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const NOTIFICATIONS_MODE_CLIPBOARD_EVIDENCE: &str = "RoomScreen notifications Copy mode writes only the already loaded current room notification mode plus the loaded RoomsList unread/mention/manual-unread summary to the local clipboard. Missing notification mode stays local-unavailable with no clipboard payload. The action reuses the existing GetRoomNotificationMode read result and sends no SetRoomNotificationMode, timed mute, global notification preference, keyword rule, push gateway/device, pusher, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const NOTIFICATIONS_MODE_CLIPBOARD_LABEL: &str =
    "Copy mode uses loaded notification state and local clipboard only.";
pub const NOTIFICATIONS_MODE_TARGET_METADATA_EVIDENCE: &str = "RoomScreen notifications mode target metadata is derived only from local strip state before any confirmed write: current loaded room notification mode, requested All/Mentions/Mute mode when a confirmation or failed retry is staged, loaded RoomsList attention availability, retry cache availability, timeline availability, and current local status. Opening Notifications, staging All/Mentions/Mute, failed Retry visibility, Refresh, and Close send no MatrixRequest::SetRoomNotificationMode unless the user accepts PositiveConfirmationModal, and send no timed mute, global notification preference, keyword rule, push gateway/device, pusher, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const NOTIFICATIONS_MODE_TARGET_METADATA_LABEL: &str =
    "Mode target metadata is local; SetRoomNotificationMode waits for confirmation.";
pub const NOTIFICATIONS_CLOSE_REFRESH_METADATA_EVIDENCE: &str = "RoomScreen notifications Close and Refresh metadata is derived only from the current local notification status, current loaded notification mode state, loaded RoomsList unread/mention/manual-unread availability, and timeline availability. Refresh reuses the existing MatrixRequest::GetRoomNotificationMode read path; Close only hides the local strip. Neither action submits MatrixRequest::SetRoomNotificationMode, timed mute, global notification preference, keyword, push gateway/device, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests.";
pub const NOTIFICATIONS_CLOSE_REFRESH_METADATA_LABEL: &str =
    "Close/Refresh metadata is local/read-only; no notification mode write.";
pub const NOTIFICATIONS_LOCAL_BOUNDARY_EVIDENCE: &str = "RoomScreen notifications now supports confirmed current-room Matrix notification mode writes for All messages, Mentions, and Mute, confirmed keyword Add/Remove writes through MatrixRequest::SetNotificationKeywordRule, confirmed default room-mode writes through MatrixRequest::SetDefaultRoomNotificationMode, confirmed failed-state Retry for cached SetRoomNotificationMode, keyword mutation, or default room-mode mutation state, loaded unread/mention/manual-unread reflection, live enabled-keyword reads, live pusher capability reads, and live default room-mode reads through MatrixRequest::GetDefaultRoomNotificationMode, while notifications remains a base gap for timed mute, raw global notification preference writes beyond SDK keyword/default APIs, push gateway/device configuration, and broader room-list notification indication. Refresh and Close only update local labels after MatrixRequest::GetRoomNotificationMode. Timed mute writes, account-data editing beyond SDK notification rules, push gateway mutation, raw default preference writes outside the SDK default-room-mode API, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and unrelated live mutation paths remain unwired.";
pub const NOTIFICATIONS_LOCAL_BOUNDARY_LABEL: &str =
    "Notifications mode and keyword writes only after confirmation; timed mute stays unwired.";
pub const NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_EVIDENCE: &str = "RoomScreen notifications default All/Mentions/Mute controls use PositiveConfirmationModal before MatrixRequest::SetDefaultRoomNotificationMode writes the Matrix SDK default room notification mode for the loaded room's encrypted and one-to-one class through NotificationSettings::set_default_room_notification_mode. The worker reads NotificationSettings::get_default_room_notification_mode after success and returns TimelineUpdate::NotificationDefaultRoomModeMutated with a NotificationDefaultRoomModeSummary. Failure caches TimelineKind plus RoomNotificationMode for a failed-state Retry that also requires PositiveConfirmationModal. It sends no timed mute write, pusher set/delete mutation, push gateway/device configuration write, sound/badge tuning, raw account-data edit outside the SDK default-room-mode API, room-state, membership, account/profile, gateway/runtime/auth, or unrelated live mutation.";
pub const NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL: &str =
    "Default All/Mentions/Mute writes confirm before SDK default room-mode mutation.";
pub const NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE: &str = "RoomScreen notifications timed/global boundary metadata keeps the unsupported notification controls visible in the local strip: timed mute durations, raw global notification preference writes beyond live default-mode SDK APIs and SDK keyword rules, push gateway/device or pusher configuration, broader room-list notification indication, and retry/failure automation remain unwired. The metadata is computed from the current loaded room notification mode, loaded RoomsList unread/mention/manual-unread attention state, keyword-list read/write status, MatrixRequest::GetDefaultRoomNotificationMode read status, live default room-mode read/write status, and current local status only; it sends no MatrixRequest::SetRoomNotificationMode unless All messages, Mentions, or Mute is explicitly confirmed, sends no MatrixRequest::SetNotificationKeywordRule unless Add keyword or Remove keyword is explicitly confirmed, and sends no MatrixRequest::SetDefaultRoomNotificationMode unless Default All/Mentions/Mute or failed-state Retry is explicitly confirmed. It sends no timed mute, raw default preference write outside the SDK default-room-mode API, unconfirmed keyword write, push gateway/device, pusher, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or unrelated live mutation request.";
pub const NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_LABEL: &str =
    "Timed/global notification controls are boundary metadata only.";
pub const NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE: &str = "RoomScreen notifications pusher/keyword boundary metadata now separates live keyword-list reads, confirmed keyword mutations, pusher-status reads, default room-mode reads, and confirmed default room-mode writes from blocked pusher/global/timed writes. Keyword rules, Keyword list, and Keywords submit MatrixRequest::GetNotificationKeywordRules, which reads Matrix SDK NotificationSettings::contains_keyword_rules and enabled_keywords, then returns TimelineUpdate::NotificationKeywordRulesFetched to the strip. Add keyword and Remove keyword validate a typed keyword, open PositiveConfirmationModal, then submit MatrixRequest::SetNotificationKeywordRule so SlidingSync calls NotificationSettings::add_keyword or remove_keyword and returns TimelineUpdate::NotificationKeywordRulesMutated. Device push and Pushers submit MatrixRequest::GetNotificationPusherStatus, which reads Matrix SDK Client::can_homeserver_push_encrypted_event_to_device and returns TimelineUpdate::NotificationPusherStatusFetched to the strip. Global and Defaults submit MatrixRequest::GetDefaultRoomNotificationMode, which reads NotificationSettings::get_default_room_notification_mode for the loaded room class and returns TimelineUpdate::NotificationDefaultRoomModeFetched. Default All/Mentions/Mute validate the loaded timeline class, open PositiveConfirmationModal, then submit MatrixRequest::SetDefaultRoomNotificationMode so SlidingSync calls NotificationSettings::set_default_room_notification_mode and returns TimelineUpdate::NotificationDefaultRoomModeMutated. Raw global preference writes beyond SDK default APIs, Timed mute duration presets, Push gateway/device setup writes, Pusher enable/disable mutations, Sound/badge tuning, and broader room-list notification indication remain local blocked controls. The boundary is computed from the current loaded room notification mode, loaded RoomsList unread/mention/manual-unread attention state, retry cache readiness, keyword read/write status, pusher read status, default-mode read/write status, and local status. It does not submit Matrix notification rule account-data edits outside the SDK keyword/default APIs, pusher mutations, push gateway/device configuration write, timed mute writes, raw default preference writes outside the SDK default-room-mode API, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or unrelated live mutation while notifications stays a base gap beyond confirmed All/Mentions/Mute SetRoomNotificationMode, confirmed keyword writes, confirmed default writes, and live keyword-rule, default-mode, or pusher-status reads.";
pub const NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_LABEL: &str = "Keyword list, default mode, and pusher status read live Matrix settings; keyword Add/Remove and default mode writes confirm first; timed mute, pusher writes, sound, and badge stay blocked.";
pub const NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_EVIDENCE: &str = "RoomScreen notification Keyword rules, Keyword list, and Keywords controls are wired to a real Matrix SDK read. Each click submits MatrixRequest::GetNotificationKeywordRules for the loaded timeline, SlidingSync reads NotificationSettings::contains_keyword_rules() and NotificationSettings::enabled_keywords(), sorts the enabled custom keyword patterns, and returns TimelineUpdate::NotificationKeywordRulesFetched. The result updates the notification strip and popup with the enabled keyword count/list or an empty-state message. Add keyword and Remove keyword are separate confirmed live writes through MatrixRequest::SetNotificationKeywordRule. The read path itself submits no unconfirmed add/remove keyword rule write, no account-data mutation outside the SDK notification settings API, no pusher mutation, no push gateway/device configuration, no timed mute write, no global preference write, no sound/badge tuning, no SetRoomNotificationMode outside the existing confirmed room-mode path, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_LABEL: &str = "Keyword list reads live Matrix notification settings; Add/Remove keyword writes confirm first.";
pub const NOTIFICATIONS_KEYWORD_MUTATION_EVIDENCE: &str = "RoomScreen notifications keyword Add/Remove is a confirmed Matrix SDK notification-rule mutation. The keyword input stays local until Add keyword, Remove keyword, or Return stages PositiveConfirmationModal. Accept submits MatrixRequest::SetNotificationKeywordRule for the loaded TimelineKind; SlidingSync calls NotificationSettings::add_keyword or NotificationSettings::remove_keyword and returns TimelineUpdate::NotificationKeywordRulesMutated. Success clears the failed keyword retry cache and refreshes MatrixRequest::GetNotificationKeywordRules; failure caches keyword plus operation and exposes Retry, which reopens PositiveConfirmationModal before resubmitting the same MatrixRequest. This path sends no unconfirmed keyword write, no raw Matrix notification account-data edit outside SDK notification settings, no pusher set/delete mutation, no push gateway/device configuration write, no timed mute write, no global preference write, no sound/badge tuning, no message send/edit/redact, no room-state, membership, account/profile, gateway/runtime/auth, or unrelated live mutation.";
pub const NOTIFICATIONS_KEYWORD_MUTATION_LABEL: &str =
    "Keyword Add/Remove writes use PositiveConfirmationModal and Matrix SDK notification settings.";
pub const NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_EVIDENCE: &str = "RoomScreen notification Device push and Pushers controls are wired to a real Matrix SDK read-only pusher capability check. Each click submits MatrixRequest::GetNotificationPusherStatus for the loaded timeline, SlidingSync calls Client::can_homeserver_push_encrypted_event_to_device(), and TimelineUpdate::NotificationPusherStatusFetched updates the notification strip and popup with supported, unsupported, or error status. This is read-only: it submits no pusher set/delete mutation, no push gateway/device configuration write, no account-data mutation, no push-rule write, no keyword mutation, no timed mute write, no global preference write, no sound/badge tuning, no SetRoomNotificationMode outside the existing confirmed room-mode path, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth expansion, or live mutation.";
pub const NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_LABEL: &str =
    "Device push and Pushers read live homeserver push capability; pusher writes stay blocked.";
pub const NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE: &str = "RoomScreen notifications advanced controls row exposes Timed, Keywords, Pusher, and Global on telegram_notifications_strip. Keywords is a live read-only Matrix SDK handoff through MatrixRequest::GetNotificationKeywordRules and TimelineUpdate::NotificationKeywordRulesFetched; Global is a live read-only Matrix SDK handoff through MatrixRequest::GetDefaultRoomNotificationMode and TimelineUpdate::NotificationDefaultRoomModeFetched; the adjacent keyword input row performs confirmed Add/Remove writes through MatrixRequest::SetNotificationKeywordRule. Timed and Pusher setup only update local notification status, boundary metadata, and popup from the current loaded room notification mode plus loaded RoomsList unread/mention/manual-unread state. It does not submit raw Matrix notification rule account-data edits, default preference writes, pusher mutations, push gateway/device configuration, timed mute writes, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or unrelated live mutation while confirmed All/Mentions/Mute SetRoomNotificationMode and confirmed keyword Add/Remove remain the notification write paths.";
pub const NOTIFICATIONS_ADVANCED_CONTROLS_LABEL: &str = "Keywords and Global defaults read live; keyword Add/Remove confirms first; Timed and Pusher setup stay local blocked controls.";
pub const NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_EVIDENCE: &str = "RoomScreen notifications advanced detail controls row exposes Quiet hours, Keyword list, Device push, Defaults, and Sound badge on telegram_notifications_strip. Keyword list is a live read-only Matrix SDK handoff through MatrixRequest::GetNotificationKeywordRules and TimelineUpdate::NotificationKeywordRulesFetched; Device push is a live read-only Matrix SDK handoff through MatrixRequest::GetNotificationPusherStatus and TimelineUpdate::NotificationPusherStatusFetched; Defaults is a live read-only Matrix SDK handoff through MatrixRequest::GetDefaultRoomNotificationMode and TimelineUpdate::NotificationDefaultRoomModeFetched. Quiet hours and Sound badge only update local advanced notification detail metadata, boundary labels, and popup copy from the current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, and retry cache readiness. It does not submit Matrix notification rule account-data edits, push-rule writes beyond the separate confirmed keyword Add/Remove row, pusher mutations, push gateway/device configuration writes, timed mute writes, default preference writes, sound/badge tuning, retry automation, MatrixRequest::SetRoomNotificationMode outside the existing confirmed All/Mentions/Mute or failed-state Retry paths, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_LABEL: &str = "Keyword list, Device push, and Defaults read live; Quiet hours and Sound badge stay local status controls.";
pub const NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE: &str = "RoomScreen notifications result detail controls row exposes Result, Requested, Retry cache, Failure, and Source as visible local status buttons on telegram_notifications_strip. Clicking any one only updates local notification result detail metadata, boundary labels, and popup copy from the current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, retry cache readiness, timeline availability, and local status text. It does not submit MatrixRequest::GetRoomNotificationMode outside the existing Refresh/read-open path, MatrixRequest::SetRoomNotificationMode outside the existing confirmed All/Mentions/Mute or failed-state Retry paths, timed mute writes, global notification preference writes, unconfirmed keyword writes, push-rule writes beyond the confirmed keyword Add/Remove row, pusher mutations, push gateway/device configuration, sound/badge tuning, retry automation, cancel queue, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const NOTIFICATIONS_RESULT_DETAIL_CONTROLS_LABEL: &str =
    "Result, Requested, Retry cache, Failure, and Source stay local result detail controls.";
pub const NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "RoomScreen notifications timed/global/pusher preflight controls row exposes Schedule, Packet, Contract, Account data, Keywords, Pushers, and Defaults on telegram_notifications_strip. Keywords is a live read-only Matrix SDK handoff through MatrixRequest::GetNotificationKeywordRules and TimelineUpdate::NotificationKeywordRulesFetched; Pushers is a live read-only Matrix SDK handoff through MatrixRequest::GetNotificationPusherStatus and TimelineUpdate::NotificationPusherStatusFetched; Defaults is a live read-only Matrix SDK handoff through MatrixRequest::GetDefaultRoomNotificationMode and TimelineUpdate::NotificationDefaultRoomModeFetched. Schedule renders a local notification schedule packet snapshot from the current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, requested mode, retry cache readiness, timeline availability, and local status text; Contract copies a typed account-data/push-rule/pusher/result contract packet; Account data updates local preflight metadata, boundary labels, and popup copy from the same loaded state. It submits no Matrix notification rule account-data write, no push-rule write beyond SDK keyword/default reads, no unconfirmed keyword mutation outside the separate Add/Remove confirmation row, no pusher mutation, no push gateway/device configuration write, no timed mute write, no default preference write, no sound/badge tuning, no MatrixRequest::GetRoomNotificationMode outside the existing Refresh/read-open path, no MatrixRequest::SetRoomNotificationMode outside the existing confirmed All/Mentions/Mute or failed-state Retry paths, no retry automation, cancel queue, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str = "Schedule, Packet, Contract, Account data, Keywords, Pushers, and Defaults: Keywords, Pushers, and Defaults read live; other preflight controls stay local.";
pub const NOTIFICATIONS_RULE_PACKET_DRILLDOWN_EVIDENCE: &str = "RoomScreen notifications Packet copies a local notification rule packet to the local clipboard from the current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, requested mode, retry cache readiness, timeline availability, and local status text. The packet persists request/result/error/retry acceptance criteria for room mode, timed mute, global preferences, keyword rules, pusher/device config, defaults, and sound/badge tuning while notifications remains a base gap beyond confirmed All/Mentions/Mute SetRoomNotificationMode, and before typed account-data, push-rule, pusher, or notification-result contracts exist. It submits no Matrix notification rule account-data read or write, no push-rule write, no pusher mutation, no push gateway/device configuration, no timed mute write, no global notification preference write, no sound/badge tuning, no extra GetRoomNotificationMode, no unconfirmed SetRoomNotificationMode, no retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation.";
pub const NOTIFICATIONS_RULE_PACKET_DRILLDOWN_LABEL: &str =
    "Notification Packet copies local rule/result/retry acceptance criteria only.";
pub const NOTIFICATIONS_RULE_CONTRACT_PACKET_EVIDENCE: &str = "RoomScreen notifications Contract copies a typed notification account-data/pusher contract packet to the local clipboard from the current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, requested mode, retry cache readiness, timeline availability, and local status text. The contract persists typed request/result/error/retry/source slots for room notification mode, account-data notification rules, push-rule keyword rules, pusher/device configuration, global preference defaults, timed mute scheduling, sound/badge tuning, stale requested-mode retries, and result reconciliation before timed/global/keyword/pusher writes can be promoted. It submits no Matrix notification rule account-data read or write, no push-rule write, no pusher mutation, no push gateway/device configuration, no timed mute write, no global notification preference write, no sound/badge tuning, no extra GetRoomNotificationMode, no unconfirmed SetRoomNotificationMode, no retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation.";
pub const NOTIFICATIONS_RULE_CONTRACT_PACKET_LABEL: &str =
    "Notification Contract copies typed account-data/push-rule/pusher/result criteria only.";
pub const NOTIFICATIONS_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomScreen notifications Taxonomy copies a local notification timed/global/pusher result taxonomy packet to the local clipboard from the current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, requested mode, retry cache readiness, timeline availability, and local status text. The packet records operation_id_slot not_assigned for timed mute, raw account-data, pusher/device, and sound/badge writes; live-result references for the existing confirmed room mode, keyword, and default-mode SDK paths; not-wired result mappings for timed mute scheduled/applied/expired/failed/stale, raw account-data applied/failed/stale, pusher enabled/disabled/failed/stale, sound/badge applied/failed/stale, retry/cancel/source-hash policy, and audit redaction. It submits no Matrix notification rule account-data read or write outside SDK keyword/default APIs, no pusher mutation, no push gateway/device configuration, no timed mute write, no sound/badge tuning, no extra GetRoomNotificationMode, no unconfirmed SetRoomNotificationMode or SetDefaultRoomNotificationMode, no retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation.";
pub const NOTIFICATIONS_RESULT_TAXONOMY_PACKET_LABEL: &str =
    "Notification Taxonomy copies timed/global/pusher result slots locally.";
pub const NOTIFICATIONS_RETRY_CONFIRMATION_EVIDENCE: &str = "RoomScreen notification mode failed-state Retry reuses only the cached room id and RoomNotificationMode from the last confirmation-gated MatrixRequest::SetRoomNotificationMode attempt. Retry opens PositiveConfirmationModal before another SetRoomNotificationMode request is submitted; unavailable cached mode, unavailable room id, and confirmation cancel stay local. It sends no timed mute, global notification preference, keyword rule, push gateway/device, pusher, retry automation, cancel queue, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const NOTIFICATIONS_RETRY_CONFIRMATION_LABEL: &str = "Failed notification Retry confirms before SetRoomNotificationMode; timed/global/pusher work stays unwired.";
pub const NOTIFICATIONS_LOADED_ATTENTION_LABEL: &str =
    "Loaded unread/mention state is read-only; push and timed mute stay unwired.";
pub const NOTIFICATIONS_COMPACT_LABEL: &str =
    "All, Mentions, Mute, and default modes write after confirmation.";

fn telegram_notification_mode_action_label(mode: RoomNotificationMode) -> &'static str {
    match mode {
        RoomNotificationMode::AllMessages => "All messages",
        RoomNotificationMode::MentionsAndKeywordsOnly => "Mentions",
        RoomNotificationMode::Mute => "Mute",
    }
}

fn notification_mode_write_result_popup_message(
    room_label: &str,
    mode: RoomNotificationMode,
    result: &Result<(), String>,
) -> String {
    let mode_label = telegram_notification_mode_action_label(mode);
    match result {
        Ok(()) => format!("Notification mode updated for {room_label}: {mode_label}."),
        Err(error) => {
            format!("Notification mode update failed for {room_label}: {mode_label}. {error}")
        }
    }
}

fn notification_default_room_mode_write_result_popup_message(
    room_label: &str,
    mode: RoomNotificationMode,
    result: &Result<NotificationDefaultRoomModeSummary, String>,
) -> String {
    let mode_label = telegram_notification_mode_action_label(mode);
    match result {
        Ok(summary) => {
            let default_summary = notification_default_room_mode_summary_label(summary);
            format!(
                "Default notification mode updated for {room_label}: {mode_label}; {default_summary}."
            )
        }
        Err(error) => {
            format!(
                "Default notification mode update failed for {room_label}: {mode_label}. {error}"
            )
        }
    }
}

fn notification_keyword_mutation_action_label(
    mutation: NotificationKeywordMutation,
) -> &'static str {
    match mutation {
        NotificationKeywordMutation::Add => "Add keyword",
        NotificationKeywordMutation::Remove => "Remove keyword",
    }
}

fn notification_keyword_mutation_verb(mutation: NotificationKeywordMutation) -> &'static str {
    match mutation {
        NotificationKeywordMutation::Add => "added",
        NotificationKeywordMutation::Remove => "removed",
    }
}

fn notification_keyword_mutation_result_popup_message(
    room_label: &str,
    keyword: &str,
    mutation: NotificationKeywordMutation,
    result: &Result<(), String>,
) -> String {
    let action = notification_keyword_mutation_action_label(mutation);
    match result {
        Ok(()) => format!("Notification keyword {action} succeeded for {room_label}: {keyword}."),
        Err(error) => {
            format!("Notification keyword {action} failed for {room_label}: {keyword}. {error}")
        }
    }
}

pub const MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_EVIDENCE: &str = "RoomScreen media download/playback now gives plain File, Audio, and Video messages Download and Play links that open PositiveConfirmationModal before the local save dialog. Confirmations preview already loaded timeline metadata such as type, filename, MIME type, size, duration, and dimensions when present. Confirmed Save/Play then submits MatrixRequest::SaveMedia, which fetches full-file media through the SDK media cache path and writes it to the selected path. Play saves first and then asks the system opener to open that saved file. Row-scoped Retry on recovery/preflight controls is the guarded exception: when the row carries a plain MXC, Retry opens PositiveConfirmationModal and reuses the same SaveMedia path after the user picks a save path. Encrypted file/audio/video rows show loaded timeline metadata, and encrypted image rows show loaded ImageInfo metadata, in local-disabled previews while Decrypt, codec/transcode work, image decode, thumbnail fetch, inline audio/video player controls, accepted queue cancel/resume controls, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain unwired local evidence while media_download_playback remains a base gap.";
pub const MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_LABEL: &str = "Media Save/Play requires confirmation and local save; decrypt, codec, inline player, retry/cancel, and live mutation stay local.";
pub const MEDIA_DOWNLOAD_PLAYBACK_METADATA_EVIDENCE: &str = "RoomScreen media Download/Play confirmations carry loaded file/audio/video metadata from the existing timeline content: media type, filename, MIME type, size, duration, and dimensions when already present. This metadata preview updates only the confirmation body and popup copy before the local save dialog; it sends no extra media fetch, decrypt, codec/transcode, inline playback, retry/cancel queue control, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation before the confirmed MatrixRequest::SaveMedia branch.";
pub const MEDIA_DOWNLOAD_PLAYBACK_METADATA_LABEL: &str =
    "Loaded media metadata preview only; SaveMedia still waits for confirmation.";
pub const MEDIA_METADATA_CLIPBOARD_EVIDENCE: &str = "RoomScreen media Copy metadata writes only already loaded File, Audio, and Video timeline metadata to the local clipboard: media kind, filename, MIME type, size, duration, dimensions, and a compact summary when present. It is available for plain and encrypted file/audio/video rows because it does not need media bytes. Missing fields stay labeled unavailable. It sends no FetchMedia, SaveMedia, decrypt, codec/transcode, inline playback, system opener, retry/cancel queue control, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while media_download_playback remains a base gap.";
pub const MEDIA_METADATA_CLIPBOARD_LABEL: &str =
    "Copy metadata uses loaded timeline media fields and local clipboard only.";
pub const MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE: &str = "RoomScreen media save dialog lifecycle metadata reuses only the already loaded media action metadata summary for confirmation opened, confirmation canceled, save dialog accepted, save dialog canceled, and unsupported save-dialog popup states. SaveMedia is still submitted only after confirmation accepts and the local save dialog returns a selected path; confirmation cancel, save dialog cancel, unsupported platforms, and popup metadata send no extra media fetch, decrypt, codec/transcode, inline playback, retry/cancel queue control, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_LABEL: &str =
    "Media save-dialog lifecycle metadata is local; SaveMedia waits for a picked path.";
pub const MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE: &str = "RoomScreen media save destination metadata shows the selected local destination path, loaded filename/type metadata, and whether Play will open the saved file before MatrixRequest::SaveMedia is submitted. The destination label is emitted only after PositiveConfirmationModal accept plus local save dialog picked; confirmation cancel, save dialog cancel, unsupported save dialog, retry/cancel queue controls, inline audio/video controls, decrypt, codec/transcode, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain unwired.";
pub const MEDIA_SAVE_DESTINATION_METADATA_LABEL: &str = "Selected save destination is local metadata; SaveMedia still waits for confirmation + picked path.";
pub const MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_EVIDENCE: &str = "RoomScreen media inline playback and queue boundary metadata is computed from already loaded file/audio/video action metadata before the save dialog opens. Download states that it only writes a picked local file after confirmation; Play states that it saves first through MatrixRequest::SaveMedia and then asks the system opener to open the saved file. inline audio/video controls, decrypt, codec/transcode, retry/cancel queue controls, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain unwired; no extra media fetch or SaveMedia request is submitted before confirmation accept plus a picked local path.";
pub const MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_LABEL: &str =
    "Inline playback and queue controls stay boundary metadata only.";
pub const MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_EVIDENCE: &str = "RoomScreen audio and video timeline rows now render a visible disabled inline-player control strip from already loaded timeline metadata: media kind, loaded filename, duration availability, MIME type, local size, and dimensions when present. The strip makes Playhead, Seek, Queue, Decrypt, and Codec controls visibly disabled while Download/Play remain the only active links. It sends no FetchMedia, SaveMedia, media cache read beyond the existing row render, decrypt, codec/transcode, inline player startup, playback progress subscription, retry/cancel queue control, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_LABEL: &str = "Inline player disabled: Playhead/Seek/Queue/Decrypt/Codec stay local; Download/Play still confirm before SaveMedia.";
pub const MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE: &str = "RoomScreen audio and video timeline rows expose Codec, Transcode, Captions, Quality, and Decrypt as visible local codec/transcode controls next to the disabled inline-player strip. Clicking any control only rebuilds already loaded media metadata from the local link query and shows popup copy; it does not submit FetchMedia, submit SaveMedia, start a decoder, start a transcoder, inspect codec support beyond loaded MIME/duration/dimensions labels, fetch captions, change playback quality, decrypt media, start inline playback, invoke the system opener, mutate retry/cancel queue state, attach/send media, mutate room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MEDIA_CODEC_TRANSCODE_CONTROLS_LABEL: &str =
    "Codec, Transcode, Captions, Quality, and Decrypt are local codec/transcode controls.";
pub const MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE: &str = "RoomScreen media save result/open boundary metadata makes the remaining media_download_playback result gap explicit while counting the existing opener outcome mapping as live popup evidence. MatrixRequest::SaveMedia completion reports saved, download failed, save failed, system opener opened, opener failed, and invalid saved-path states through popup status via SaveMediaOpenOutcome, and sends a TimelineUpdate::MediaSaveResult so RoomScreen caches the successful plain-MXC destination for the result-row Open folder and Replay handoffs. Inline audio/video player state, seek controls, retry/cancel queue controls, decrypt retry, codec/transcode fallback, background download list, delivery/read receipts, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain local blocked controls. The metadata is derived only from the already loaded media action summary and the requested Download or Play mode before confirmation accept; it submits no extra FetchMedia, SaveMedia, retry, queue cancel, decrypt, codec, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const MEDIA_SAVE_RESULT_STATUS_BOUNDARY_LABEL: &str = "Save/open result popup maps saved, failed, opened, opener-failed, and invalid-path states; successful plain saves cache Open folder and Replay targets while inline player, retry queue, decrypt, codec, and live mutation remain blocked.";
pub const MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE: &str = "RoomScreen media save/open recovery controls row exposes Open folder, Replay, Retry, Queue, and Decrypt as visible result-recovery links on plain File, Audio, and Video timeline rows. Open folder is a live local OS handoff only after a successful MatrixRequest::SaveMedia result cached the selected destination for the same plain MXC; it first validates that the cached saved path is still a regular local file, clears stale cached destinations, then opens the saved file's parent folder through the system opener and sends no Matrix request. Replay is also a live local OS handoff from the same cached successful SaveMedia destination: it validates that the cached saved path is still a regular local file, clears stale cached destinations, converts it to a file URL, and opens it through the system opener without fetching media again. Queue renders a local media playback/download queue snapshot from requested action mode, loaded metadata, save-result boundary, opener state, and any cached saved-file status; if the cached destination is stale, Queue clears it locally. Retry is a guarded live resubmit: when the row carries a plain MXC, it opens PositiveConfirmationModal and then reuses the same MatrixRequest::SaveMedia Download/Play path after the user picks a save path. Decrypt only rebuilds loaded media metadata from the link query and shows local popup boundary labels. The row sends no unconfirmed FetchMedia, no unconfirmed SaveMedia, no automatic retry, no queue cancel/resume, no queue retry/resume, no decrypt retry, no codec/transcode, no background download list mutation, no delivery/read receipt, no attachment send, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request outside confirmed SaveMedia retry and cached-destination Open folder/Replay.";
pub const MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL: &str = "Open folder/Replay use cached SaveMedia destination; Queue/Decrypt remain local recovery controls.";
pub const MEDIA_CACHED_SAVED_FILE_STATUS_EVIDENCE: &str = "RoomScreen media Queue can read only local filesystem metadata for an already cached successful SaveMedia destination: regular-file state, size, readonly bit, and modified timestamp seconds. Missing, inaccessible, or non-file destinations clear the cached MXC destination locally before any Open folder or Replay handoff. This submits no FetchMedia, no SaveMedia, no inline player, no decoder, no queue retry/resume/cancel, no system opener, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MEDIA_CACHED_SAVED_FILE_STATUS_LABEL: &str =
    "Cached saved-file status is local metadata only; stale destinations clear the cache.";
pub const MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "RoomScreen media SaveMedia preflight detail controls row exposes Request, Result, Error, Retry, and Source as visible links on plain File, Audio, and Video timeline rows. Request, Result, Error, and Source only rebuild loaded media metadata from the link query and show local popup copy for the requested SaveMedia phase, cached result/error shape, retry availability, and source metadata. Retry is a guarded live resubmit: when the row carries a plain MXC, it opens PositiveConfirmationModal and then reuses the same MatrixRequest::SaveMedia Download/Play path after the user picks a save path. It sends no unconfirmed FetchMedia, no unconfirmed extra SaveMedia, no cached-destination Open folder/Replay handoff from preflight details, no automatic retry, no queue cancel/resume, no decrypt retry, no codec/transcode, no background download mutation, no delivery/read receipt, no attachment send, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request outside the confirmed SaveMedia retry.";
pub const MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str =
    "Request, Result, Error, Retry, and Source are local SaveMedia preflight details.";
pub const MEDIA_OPERATION_PACKET_DRILLDOWN_EVIDENCE: &str = "RoomScreen media Packet copies a local media operation acceptance matrix to the clipboard from already loaded File, Audio, and Video timeline metadata. The packet records requested action, media metadata, SaveMedia request/result shape, cached Open folder/Replay destination slots, inline playback slot, decrypt/decode slot, codec/transcode slot, captions slot, queue retry/resume/cancel slot, system opener result slot, and promotion criteria for media_download_playback. It is available for plain and encrypted file/audio/video rows because it needs no media bytes. It submits no FetchMedia, SaveMedia, system opener request outside cached Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt retry, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MEDIA_OPERATION_PACKET_DRILLDOWN_LABEL: &str = "Media Packet copies local playback/decrypt/codec/queue acceptance criteria; no FetchMedia, SaveMedia, decrypt, codec, queue, opener, or live mutation runs.";
pub const MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE: &str = "RoomScreen media Contract copies a local typed playback/media queue contract to the clipboard from the same already loaded File, Audio, and Video timeline metadata used by Packet. The contract names typed slots for media identity, SaveMedia request/result/error, cached Open folder/Replay destination result with stale cache validation and eviction, inline playback request/result/error/progress, decrypt/decode request/result/error, codec/transcode/captions/quality fallback, system opener result, queue retry/resume/cancel/background persistence, delivery/read receipt mapping, source metadata hashing, broader stale local file handling beyond cached Open folder/Replay validation, and adapter promotion blockers before true inline/decrypt/queue controls can be wired. It is available for plain and encrypted file/audio/video rows because it needs no media bytes. It submits no FetchMedia, SaveMedia, system opener request outside cached-destination Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt retry, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_LABEL: &str = "Media Contract maps Packet to typed playback/decrypt/codec/opener/queue result contracts locally.";
pub const MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomScreen media Taxonomy copies a local decrypt/decode/opener/queue result taxonomy packet to the clipboard from already loaded File, Audio, and Video timeline metadata. The packet names the only live references as existing MatrixRequest::FetchMedia image/cache reads, confirmed MatrixRequest::SaveMedia Download/Play result mapping, cached Open folder/Replay stale validation and local OS opener handoff, and guarded SaveMedia Retry. Inline playback session/progress, encrypted-media decrypt/decode, codec/transcode/captions/quality fallback, background queue retry/resume/cancel, delivery/read receipt mapping, and stale inline/decrypt local-file handling remain not-assigned/not-wired result slots. It submits no FetchMedia, SaveMedia, system opener request outside cached Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt retry, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_LABEL: &str =
    "Media Taxonomy maps decrypt/decode/opener/queue result slots locally.";
pub const MEDIA_ENCRYPTED_METADATA_LOCAL_EVIDENCE: &str = "RoomScreen encrypted File, Audio, and Video message rows show already loaded timeline metadata such as filename, MIME type, size, duration, and dimensions inside the local disabled media preview. The preview does not start decrypt, SaveMedia, FetchMedia, codec/transcode, inline playback, retry/cancel queue control, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation work.";
pub const MEDIA_ENCRYPTED_METADATA_LOCAL_LABEL: &str =
    "Encrypted media metadata stays visible; decrypt and Save/Play stay disabled.";
pub const MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_EVIDENCE: &str = "RoomScreen encrypted Image message rows show already loaded ImageInfo metadata such as filename/body, MIME type, size, dimensions, blurhash availability, and thumbnail-source availability inside the local disabled image preview. The preview does not start decrypt, SaveMedia, FetchMedia, image decode, thumbnail fetch, media cache mutation, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation work.";
pub const MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_LABEL: &str =
    "Encrypted image metadata stays visible; decrypt and image decode stay disabled.";
pub const MEDIA_IMAGE_FETCH_CACHE_COMPACT_LABEL: &str =
    "Image preview uses cache; controls stay local.";
pub const MEDIA_IMAGE_FETCH_FAILED_COMPACT_LABEL: &str =
    "Image preview failed; cache state stays local.";
pub const MEDIA_ENCRYPTED_IMAGE_COMPACT_LABEL: &str =
    "Encrypted image preview only; decrypt stays local-disabled.";
pub const MEDIA_FILE_COMPACT_LABEL: &str =
    "File download confirms before local save; playback stays local.";
pub const MEDIA_AUDIO_COMPACT_LABEL: &str =
    "Audio download/play confirms before local save; codec controls stay local.";
pub const MEDIA_VIDEO_COMPACT_LABEL: &str =
    "Video download/play confirms before local save; codec controls stay local.";
const MEDIA_DOWNLOAD_URL_SCHEME: &str = "hepta-media-download";
const MEDIA_METADATA_CLIPBOARD_URL_SCHEME: &str = "hepta-media-metadata-copy";
const MEDIA_RESULT_CONTROL_URL_SCHEME: &str = "hepta-media-result-control";
const MEDIA_SAVE_PREFLIGHT_CONTROL_URL_SCHEME: &str = "hepta-media-save-preflight-control";
const MEDIA_CODEC_TRANSCODE_CONTROL_URL_SCHEME: &str = "hepta-media-codec-transcode-control";
const MEDIA_OPERATION_PACKET_URL_SCHEME: &str = "hepta-media-operation-packet";
const MEDIA_PLAYBACK_QUEUE_CONTRACT_URL_SCHEME: &str = "hepta-media-playback-queue-contract";
const MEDIA_PLAYBACK_RESULT_TAXONOMY_URL_SCHEME: &str = "hepta-media-playback-result-taxonomy";
pub const POLL_MESSAGE_PREVIEW_READ_EVIDENCE: &str = "Poll timeline items render from already loaded matrix-sdk-ui PollState results inside populate_poll_message_content. Question, answers, vote counts, total votes, open/closed status, edited note, and max selections are formatted into local message-row HTML/plaintext only and send no poll response, edit, redact, message, room-state, membership, timeline reload, or live mutation request.";
pub const POLL_MESSAGE_PREVIEW_COMPACT_LABEL: &str =
    "Read-only poll preview from loaded timeline state.";
pub const POLL_ANSWER_PREVIEW_RESULT_PACKET_EVIDENCE: &str = "Poll answer preview/result packet records loaded answer count, total votes, max selections, open/closed status, edited state, answer edit slot, vote response slot, result mapping, stale poll policy, and unsupported server capability boundary from already loaded PollState only. It sends no poll response, poll answer edit, timeline reload, message, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE: &str = "RoomScreen renders a local Telegram edit-history detail strip after MatrixRequest::FetchEditHistory completes paginated m.replace relation reads. The strip shows target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached latest replacement raw JSON availability, and a local preview-diff hint, then can be closed locally. Full can open a local synthetic EventSourceModal snapshot from this cached state. It reuses only the already loaded timeline row plus the complete paginated m.replace relation summary; it sends no remote full-history modal request, event-context fetch, timeline pagination/reload, replacement event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_LABEL: &str = "Edit history detail is local read-only: complete m.replace pagination summary plus loaded original preview.";
pub const MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_EVIDENCE: &str = "RoomScreen edit-history failed-state Retry reuses only the cached event id and TimelineKind from the last MatrixRequest::FetchEditHistory attempt. Retry opens PositiveConfirmationModal before another compact MatrixRequest::FetchEditHistory m.replace summary read is submitted; unavailable cached event id, unavailable TimelineKind, and confirmation cancel stay local. It sends no remote full-history modal request, full diff rendering, event-context fetch, timeline pagination/reload, event source open, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_LABEL: &str = "Failed edit-history Retry confirms before FetchEditHistory; full modal, event context, timeline reload, and event source stay unwired.";
pub const MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE: &str = "RoomScreen edit-history full modal boundary metadata is derived from the current paginated m.replace read state: loading, loaded, failed, or retry confirmation, replacement count when available, relation pages fetched/exhausted state, cached latest replacement raw JSON availability, and retry cache readiness. The Full control opens the existing local EventSourceModal with a synthetic full snapshot JSON built from the loaded target, complete paginated replacement summary, original preview, latest replacement preview, cached latest replacement raw JSON availability, cached error, and retry cache state. Remote/server-backed Full history modal UI, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and write-side live mutation remain local blocked controls while the real edit-history read is MatrixRequest::FetchEditHistory paginating Room::relations to next_batch exhaustion plus EventSourceModal handoff for cached latest replacement raw JSON, MatrixRequest::FetchEventSource room.event/load_or_fetch_event fallback for missing latest replacement source JSON, or loaded original latest_json.";
pub const MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_LABEL: &str = "Full opens a local snapshot modal; full diff, event context, remote modal, and reload stay blocked; complete m.replace pagination is live.";
pub const MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_EVIDENCE: &str = "RoomScreen edit-history Full opens a local synthetic EventSourceModal snapshot from cached MatrixRequest::FetchEditHistory state only: target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached latest replacement raw JSON availability, cached error, retry cache readiness, and read-only side-effect metadata. It sends no extra MatrixRequest::FetchEditHistory, no remote full-history modal request, no side-by-side full diff rendering, no event-context fetch, no timeline pagination/reload, no message send/edit/redact, no room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation.";
pub const MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_LABEL: &str =
    "Full opens a local synthetic EventSourceModal snapshot from cached edit-history state only.";
pub const MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE: &str = "RoomScreen edit-history full controls row exposes Full, Diff, Context, Source, Packet, Contract, and Taxonomy on telegram_message_edit_history_strip. Full opens the existing local EventSourceModal with a synthetic full snapshot JSON from the current complete paginated m.replace summary surface, while Context updates only the local boundary label and popup status. Diff is a real loaded side-by-side preview/full-body diff handoff that opens the existing local EventSourceModal with a read-only JSON snapshot, using cached latest replacement raw JSON and the loaded original latest_json to render full-body rows when both sources are available, then falling back to compact original/latest preview rows as a real loaded side-by-side preview diff handoff; it also copies the compact preview summary to the local clipboard. Source remains a real loaded edit-source modal handoff that opens the existing local EventSourceModal for latest replacement raw JSON returned by MatrixRequest::FetchEditHistory when available, requests MatrixRequest::FetchEventSource through Room::load_or_fetch_event for the latest replacement when cached JSON is missing, and otherwise falls back to the already loaded original edited event row when loaded latest_json is available. Packet copies a loaded/full diff and remote modal acceptance contract to the local clipboard from the same cached state, including relation pages fetched/exhausted metadata. Contract maps that Packet to typed full-history modal/result, diff, source, context, and retry contracts locally. Taxonomy maps remote full-history, source reconciliation, server-backed diff, event-context, stale, retry/cancel, and source-hash result slots locally. The row does not request a remote full-history modal, server-backed side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation; MatrixRequest::FetchEditHistory is the complete paginated edit-history read path and FetchEventSource is a source-only read fallback.";
pub const MESSAGE_EDIT_HISTORY_FULL_CONTROLS_LABEL: &str = "Full opens a local snapshot modal; Context stays local; Diff opens a loaded side-by-side preview diff modal and, when cached source JSON is available, a loaded full-body diff modal plus clipboard handoff; Source opens cached latest replacement JSON or loaded original JSON locally; Packet copies acceptance contract; Contract maps typed full-history result contracts; Taxonomy copies remote full-history/source result slots.";
pub const MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE: &str = "RoomScreen edit-history loaded diff detail state is derived only from the paginated MatrixRequest::FetchEditHistory m.replace summary and the already loaded original timeline row. The detail label reports selected Full/Diff/Context/Source/Packet/Contract/Taxonomy control, target event, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp availability, original/latest preview character counts, latest replacement raw JSON availability, local delta state, and retry cache readiness. Clicking Full opens a local synthetic EventSourceModal snapshot and updates this local detail state, boundary label, and popup copy; clicking Context updates only this local detail state, the boundary label, and popup copy; clicking Diff opens a loaded side-by-side preview/full-body diff snapshot in EventSourceModal from cached source JSON when available and falls back to compact previews, then copies only the compact loaded original/latest preview diff summary to the local clipboard; clicking Source opens the cached latest replacement raw JSON in the existing EventSourceModal when available, otherwise requests a source-only MatrixRequest::FetchEventSource for the latest replacement before falling back to the already loaded original event source; clicking Packet copies only the loaded/full diff remote modal acceptance contract locally; clicking Contract copies only typed full-history modal/result contracts locally; clicking Taxonomy copies only blocked remote full-history/source reconciliation result taxonomy locally. It submits no remote full-history modal request, server-backed side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation.";
pub const MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_LABEL: &str =
    "Loaded diff detail is local: original/latest previews, counts, delta, and timestamp only.";
pub const MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE: &str = "RoomScreen edit-history Packet copies a loaded/full diff remote modal acceptance contract to the local clipboard only from cached paginated m.replace state: target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached error, retry-cache readiness, loaded diff detail, preflight detail, and full-modal boundary metadata. The packet defines acceptance criteria for remote full-history modal request/result/error, server-backed side-by-side full diff rendering, event context, replacement event source, loaded original source, retry/cancel state, and source hashing before any backend/full-modal work can be promoted. It sends no extra MatrixRequest::FetchEditHistory, no retry without PositiveConfirmationModal, no remote full-history modal request, no server-backed side-by-side full diff rendering, no event-context fetch, no timeline pagination/reload, no replacement event source fetch, no message send/edit/redact, no room-state mutation, no membership mutation, no account/profile mutation, no gateway/runtime/auth/provider call, no Telegram delivery, and no live mutation.";
pub const MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_LABEL: &str =
    "Edit-history Packet copies loaded/full diff remote modal acceptance locally.";
pub const MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_EVIDENCE: &str = "RoomScreen edit-history Contract copies a typed full-history modal/result contract packet to the local clipboard only from cached paginated m.replace state and the loaded/full diff Packet boundary: target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached error, retry-cache readiness, loaded diff detail, preflight detail, and full-modal boundary metadata. The contract maps typed request/result/error/retry/source slots for full-history modal rendering, side-by-side diff rendering, event context, replacement event source, loaded original source fallback, source-hash, retry/cancel, stale target handling, and promotion blockers before backend edit-history work can be promoted. It sends no extra MatrixRequest::FetchEditHistory, no retry without PositiveConfirmationModal, no remote full-history modal request, no side-by-side full diff rendering, no event-context fetch, no timeline pagination/reload, no replacement event source fetch, no message send/edit/redact, no room-state mutation, no membership mutation, no account/profile mutation, no gateway/runtime/auth/provider call, no Telegram delivery, and no live mutation.";
pub const MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_LABEL: &str =
    "Edit-history Contract maps Packet to typed full-history modal/result contracts locally.";
pub const MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomScreen edit-history Taxonomy copies a remote full-history/source reconciliation result taxonomy packet to the local clipboard only from cached paginated m.replace state and loaded source/diff metadata. The packet names existing live references as paginated MatrixRequest::FetchEditHistory through Room::relations next_batch exhaustion, confirmed failed-state Retry, local synthetic Full EventSourceModal snapshot, loaded side-by-side preview/full-body diff EventSourceModal snapshot, compact diff clipboard handoff, cached latest replacement raw JSON EventSourceModal handoff, source-only MatrixRequest::FetchEventSource / Room::load_or_fetch_event fallback, and loaded original EventSourceModal fallback. The packet keeps remote full-history request ids, full-history cursor/page ids, server-backed full diff operation ids, replacement source reconciliation ids, event-context operation ids, per-replacement source result, stale-target result, retry/cancel result, and source-hash policy not-assigned/not-wired until backend edit-history contracts exist. It sends no extra MatrixRequest::FetchEditHistory, no retry without PositiveConfirmationModal, no remote full-history modal request, no server-backed side-by-side full diff rendering, no event-context fetch, no timeline pagination/reload, no replacement event source fetch beyond the existing Source control fallback, no message send/edit/redact, no room-state mutation, no membership mutation, no account/profile mutation, no gateway/runtime/auth/provider call, no Telegram delivery, and no live mutation.";
pub const MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_LABEL: &str =
    "Edit-history Taxonomy maps remote full-history/source reconciliation result slots locally.";
pub const MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_EVIDENCE: &str = "RoomScreen edit-history Diff control is a real loaded side-by-side preview/full-body diff modal plus compact diff clipboard handoff. It opens an EventSourceModal snapshot and copies a compact loaded original/latest preview diff summary to the local clipboard only from the current edit-history strip state: target event id, paginated m.replace replacement count, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached latest replacement raw JSON, loaded original latest_json when available, side-by-side preview/full-body rows, and local delta hint. When source JSON is missing, the fallback remains a real loaded side-by-side preview diff modal. It does not request a full history modal, server-backed side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch beyond the Source control fallback, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_LABEL: &str = "Edit-history Diff opens a loaded side-by-side preview/full-body diff modal and copies compact preview to local clipboard.";
pub const MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_EVIDENCE: &str = "RoomScreen edit-history Diff opens an existing local EventSourceModal with a read-only loaded side-by-side preview/full-body diff snapshot. The snapshot is built only from the current MatrixRequest::FetchEditHistory cached result, cached latest replacement raw JSON, and loaded original timeline latest_json when available: target event id, replacement count, relation pages fetched/exhausted state, latest replacement id/timestamp, original rows, latest rows, changed row flags, body-source labels, and side-effect booleans. When source JSON is missing, the fallback remains a read-only loaded side-by-side preview diff snapshot. It sends no extra MatrixRequest::FetchEditHistory, no remote full-history modal request, no server-backed full-body side-by-side diff rendering, no event-context fetch, no timeline pagination/reload, no replacement event source fetch, no message send/edit/redact, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_LABEL: &str =
    "Edit-history Diff opens a loaded side-by-side preview/full-body snapshot modal only.";
pub const MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "RoomScreen edit-history preflight detail controls row exposes Request, Result, Error, Retry, and Source as visible local buttons on telegram_message_edit_history_strip. Clicking any control only updates cached paginated edit-history preflight metadata and popup copy from the current MatrixRequest::FetchEditHistory local state: target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, original/latest preview counts, cached latest replacement raw JSON availability, cached error text, retry cache readiness, and local source/boundary metadata. It submits no extra MatrixRequest::FetchEditHistory, no retry without PositiveConfirmationModal, no remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str =
    "Request, Result, Error, Retry, and Source edit-history preflight controls stay local.";
pub const MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE: &str = "RoomScreen edit-history Source is a real loaded edit-source modal handoff that opens the existing local EventSourceModal for the latest replacement source when MatrixRequest::FetchEditHistory returned cached raw JSON for the m.replace relation; if no replacement raw JSON is cached and a latest replacement event id is known, it submits MatrixRequest::FetchEventSource, whose worker calls Room::load_or_fetch_event and opens EventSourceModal with the fetched raw JSON. If no replacement source is available, Source falls back to the already loaded original edited event row when loaded EventTimelineItem.latest_json is available. The action uses the paginated edit-history latest replacement event id, current TimelineKind room id, and raw JSON returned by Room::relations or room.event/load_or_fetch_event, or the original target event id plus loaded latest_json from the visible timeline cache. Missing event id, missing cached/fetched raw JSON, missing timeline row, or missing latest_json leaves Source as local metadata. It sends no event-context fetch, timeline pagination/reload, remote full-history modal request, side-by-side full diff rendering, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation.";
pub const MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_LABEL: &str = "Edit-history Source opens cached latest replacement JSON or loaded original event source locally.";
pub const MESSAGE_REPORT_STATUS_LIFECYCLE_EVIDENCE: &str = "RoomScreen message report status lifecycle shows the event-scoped submitted, succeeded, or failed state from the existing MatrixRequest::ReportContent and TimelineUpdate::MessageReportResult path. The strip records only the selected event id, compact reason metadata, and worker result text; failed status can open a local Retry confirmation while Close hides the local status. It sends no retry without confirmation, cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_REPORT_STATUS_LIFECYCLE_LABEL: &str = "Report status follows ReportContent result; failed Retry confirms first, cancel queue and moderation tools stay unwired.";
pub const MESSAGE_REPORT_STATUS_CLIPBOARD_EVIDENCE: &str = "RoomScreen message report Copy status writes only the current local report status cache to the local clipboard: status badge, cached event id, compact reason, result/error text, summary, lifecycle metadata, and preflight metadata when present. Missing status stays local-unavailable and writes no clipboard payload. It sends no extra MatrixRequest::ReportContent, retry without confirmation, cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_REPORT_STATUS_CLIPBOARD_LABEL: &str =
    "Report Copy status uses cached report status and local clipboard only.";
pub const MESSAGE_REPORT_RETRY_CONFIRMATION_EVIDENCE: &str = "RoomScreen message report failed-state Retry reuses only the cached event id and compact reason from the last confirmed MatrixRequest::ReportContent. Retry opens PositiveConfirmationModal before another MatrixRequest::ReportContent is submitted; confirmation cancel and unavailable cached reason stay local. It sends no retry queue automation, cancel queue, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_REPORT_RETRY_CONFIRMATION_LABEL: &str = "Failed report Retry confirms before reusing ReportContent; cancel queue and moderation stay unwired.";
pub const MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE: &str = "RoomScreen message report workflow actions row exposes Queue, Policy, Assign, Appeal, Enforce, Packet, Contract, and Taxonomy as visible local blocked buttons on telegram_message_report_status_strip. Queue now renders a local moderation packet snapshot from the cached MatrixRequest::ReportContent status path: status badge, event id, reason, cached error, summary, workflow metadata, preflight detail, and retry-cache state. Packet copies a moderation reviewer acceptance matrix to the local clipboard from the same cached status path. Contract maps that packet to typed moderation workflow/result contracts locally. Taxonomy copies blocked queue/policy/reviewer/appeal/enforcement result taxonomy locally. Queue does not cancel a moderation queue. Policy, Assign, Appeal, and Enforce only update local report status metadata and popup copy. These controls do not fetch a server policy, assign a reviewer, open an appeal workflow, redact/delete content, ban, kick, ignore/block, mutate room-state, mutate membership, send or edit messages, touch account/profile, call gateway/runtime/auth, or perform live mutation.";
pub const MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_LABEL: &str = "Queue, Policy, Assign, Appeal, Enforce, Packet, Contract, and Taxonomy stay local blocked report workflow controls.";
pub const MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE: &str = "RoomScreen message report Packet copies a moderation reviewer acceptance matrix to the local clipboard only from the cached ReportContent status strip: status badge, cached event id, compact reason, cached error, summary, workflow metadata, preflight metadata, retry-cache state, and loaded-source availability. The packet defines acceptance criteria for moderation queue persistence, policy lookup, reviewer assignment, evidence/source retention, reporter and target audit trails, appeal workflow, enforcement actions, retry/cancel handling, and result/error mapping before any backend moderation workflow can be promoted. It sends no extra MatrixRequest::ReportContent, no retry without PositiveConfirmationModal, no queue persist/cancel/reorder, no policy lookup, no reviewer assignment, no evidence upload or event-context fetch, no appeal/enforcement workflow, no redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation.";
pub const MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_LABEL: &str =
    "Report Packet copies moderation/reviewer acceptance criteria locally.";
pub const MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_EVIDENCE: &str = "RoomScreen message report Contract copies a typed moderation workflow/result contract packet to the local clipboard only from the cached ReportContent status strip and moderation reviewer packet boundary: status badge, cached event id, compact reason, cached error, summary, workflow metadata, preflight metadata, retry-cache state, and loaded-source availability. The contract names typed queue, policy, reviewer assignment, evidence/source, reporter/target audit, appeal, enforcement, result/error, retry/cancel, and source-hash slots before any backend moderation workflow can be promoted. It sends no extra MatrixRequest::ReportContent, no retry without PositiveConfirmationModal, no queue persist/cancel/reorder, no policy lookup, no reviewer assignment, no evidence upload or event-context fetch, no appeal/enforcement workflow, no redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation.";
pub const MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_LABEL: &str =
    "Report Contract maps Packet to typed moderation workflow/result contracts locally.";
pub const MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomScreen message report Taxonomy copies a blocked moderation workflow result taxonomy packet to the local clipboard only from the cached ReportContent status strip, moderation reviewer packet boundary, and loaded-source availability. The packet names existing live references as confirmed MatrixRequest::ReportContent send/result/retry and the loaded-or-source-fetch EventSourceModal handoff, then records queue, policy, reviewer, evidence, appeal, enforcement, retry, cancel, source-hash, and audit result slots as not_wired before backend moderation workflow promotion. It sends no extra MatrixRequest::ReportContent, no retry without PositiveConfirmationModal, no queue persist/cancel/reorder, no policy lookup, no reviewer assignment, no evidence upload or event-context fetch, no appeal/enforcement workflow, no redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation.";
pub const MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_LABEL: &str =
    "Report Taxonomy records blocked moderation workflow result slots locally.";
pub const MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "RoomScreen message report preflight detail controls expose Request, Result, Error, Retry, and Source as visible local buttons on telegram_message_report_status_strip. Request, Result, Error, and Retry only update local report preflight detail metadata and popup copy from the existing MatrixRequest::ReportContent status cache: status badge, cached event id, compact reason, cached error text, retry availability, and status metadata source. Source is a real loaded-or-source-fetch modal handoff: it opens the existing local EventSourceModal when the cached reported event id is still present in the loaded timeline with latest_json available, or submits only MatrixRequest::FetchEventSource for current-room event JSON when the cached report event id is known but loaded latest_json is unavailable. Missing event id or timeline state stays local metadata. It sends no extra MatrixRequest::ReportContent, no retry without PositiveConfirmationModal, no event context fetch, no cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str = "Report preflight details stay local; Source opens loaded event JSON or source-fetches current-room JSON.";
pub const MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE: &str = "RoomScreen message report Source is a real loaded-or-source-fetch modal handoff. It opens the existing local EventSourceModal from the cached reported event id and already loaded RoomScreen timeline row when EventTimelineItem.latest_json is available. If loaded JSON is unavailable but the cached reported event id and current timeline are known, it submits MatrixRequest::FetchEventSource; the existing worker calls Room::load_or_fetch_event and returns TimelineUpdate::EventSourceFetched for the same EventSourceModal path. Missing cache, missing timeline, invalid state, fetch failure, or missing source leaves Source as local metadata. It sends no event-context fetch, report retry, extra MatrixRequest::ReportContent, moderation workflow, redact/delete, ban, kick, ignore/block, room-state, membership, gateway/runtime/auth, or write-side live mutation request.";
pub const MESSAGE_REPORT_LOADED_SOURCE_MODAL_LABEL: &str = "Report Source opens loaded event JSON or requests source-only current-room JSON; no event-context fetch or moderation workflow.";

#[derive(Clone, Debug)]
pub struct EditHistorySummary {
    pub replacement_count: usize,
    pub pages_fetched: usize,
    pub pagination_exhausted: bool,
    pub latest_event_id: Option<OwnedEventId>,
    pub latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    pub latest_preview_text: Option<String>,
    pub latest_source_json: Option<String>,
}

/// The max size (width or height) of a blurhash image to decode.
/// Blurhash is a blurred placeholder — it is designed to be decoded at a small
/// size and then stretched by the GPU. Decoding at large sizes is extremely
/// expensive (CPU-bound, O(width*height)) and completely unnecessary since the
/// result is inherently blurry. A 32×32 decode is ~240x faster than 500×500
/// while being visually indistinguishable when scaled up.
const BLURHASH_IMAGE_MAX_SIZE: u32 = 32;

static UNNAMED_ROOM: &str = "Unnamed Room";

/// #FFF4E5
const COLOR_THREAD_SUMMARY_BG: Vec4 = vec4(1.0, 0.957, 0.898, 1.0);
/// #FFEACC
const COLOR_THREAD_SUMMARY_BG_HOVER: Vec4 = vec4(1.0, 0.918, 0.8, 1.0);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.COLOR_BG = (COLOR_TELEGRAM_BG)
    mod.widgets.COLOR_OVERLAY_BG = #x000000d8
    mod.widgets.COLOR_READ_MARKER = (COLOR_TELEGRAM_BLUE)

    mod.widgets.REACTION_TEXT_COLOR = (COLOR_TELEGRAM_BLUE)

    mod.widgets.COLOR_THREAD_SUMMARY_BG = #xEAF7FCD8
    mod.widgets.COLOR_THREAD_SUMMARY_BG_HOVER = #xF4FCFFE8
    mod.widgets.COLOR_THREAD_SUMMARY_BORDER = (COLOR_TELEGRAM_BORDER)
    mod.widgets.COLOR_THREAD_SUMMARY_REPLY_COUNT = (COLOR_TELEGRAM_BLUE)

    mod.widgets.TelegramRoomHeaderButton = RobrixNeutralIconButton {
        width: Fit,
        height: 34,
        margin: 0,
        padding: Inset{top: 7, bottom: 7, left: 7, right: 7},
        spacing: 4,
        align: Align{x: 0.5, y: 0.5}

        draw_bg +: {
            color: #x00000000
            color_hover: (COLOR_TELEGRAM_INPUT)
            color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
            border_radius: 17.0
            border_size: 1.0
            border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
        }
        draw_icon.color: (COLOR_TELEGRAM_MUTED)
        draw_text +: {
            color: (COLOR_TELEGRAM_MUTED)
            color_hover: (COLOR_TELEGRAM_TEXT)
            color_down: (COLOR_TELEGRAM_TEXT)
            text_style: theme.font_bold { font_size: 10.0 }
        }
        icon_walk: Walk{width: 14, height: 14, margin: Inset{right: -1}}
    }

    mod.widgets.TelegramRoomHeaderIconButton = RobrixNeutralIconButton {
        width: 34,
        height: 34,
        margin: 0,
        padding: Inset{top: 8, bottom: 8, left: 8, right: 8},
        spacing: 0,
        align: Align{x: 0.5, y: 0.5}

        draw_bg +: {
            color: #x00000000
            color_hover: (COLOR_TELEGRAM_INPUT)
            color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
            border_radius: 17.0
            border_size: 1.0
            border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
        }
        draw_icon.color: (COLOR_TELEGRAM_MUTED)
        draw_text +: {
            color: #x00000000
            color_hover: #x00000000
            color_down: #x00000000
            text_style: theme.font_bold { font_size: 0.1 }
        }
        icon_walk: Walk{width: 16, height: 16, margin: 0}
    }

    // An empty view that takes up no space in the portal list.
    mod.widgets.Empty = View { }

    // A summary at the bottom of a message that is the root of a thread.
    mod.widgets.ThreadRootSummary = RoundedView {
        visible: false
        width: Fill,
        height: Fit
        flow: Down,
        align: Align{x: 0.0, y: 0.5}
        spacing: 4.0
        margin: Inset{ top: 5.0 }
        padding: Inset{top: 10, right: 12, bottom: 10, left: 12},
        cursor: MouseCursor.Hand

        show_bg: true
        draw_bg +: {
            color: (mod.widgets.COLOR_THREAD_SUMMARY_BG)
            border_radius: 4.0
            border_size: 1.5
            border_color: (mod.widgets.COLOR_THREAD_SUMMARY_BORDER)
        }

        thread_summary_row := View {
            width: Fill,
            height: Fit,
            flow: Right,
            align: Align{x: 0.0, y: 0.5}
            spacing: 5.0

            thread_summary_count := Label {
                width: Fit,
                draw_text +: {
                    text_style: USERNAME_TEXT_STYLE { font_size: 11 }
                    color: (mod.widgets.COLOR_THREAD_SUMMARY_REPLY_COUNT)
                }
                text: ""
            }

            Icon {
                width: Fit, height: Fit,
                align: Align{x: 0.5, y: 0.5}
                draw_icon +: {
                    svg: crate_resource("self://resources/icons/double_chat.svg")
                    color: (mod.widgets.COLOR_THREAD_SUMMARY_REPLY_COUNT)
                }
                icon_walk: Walk{ width: 25, height: 25, margin: Inset{top: 3, right: 7} }
            }

            thread_summary_latest := MessageHtml {
                width: Fill,
                flow: Right,
                max_lines: 2
                text_overflow: Ellipsis
            }
        }

        thread_summary_evidence := Label {
            width: Fill,
            max_lines: 1
            text_overflow: Ellipsis
            draw_text +: {
                text_style: theme.font_regular { font_size: 9.0 },
                color: (COLOR_TELEGRAM_MUTED)
            }
            text: "Thread open uses existing FetchThreadSummaryDetails + CreateThreadTimeline read/open paths; no message, room-state, or membership mutation."
        }
    }

    // The view used for each text-based message event in a room's timeline.
    mod.widgets.Message = set_type_default() do #(Message::register_widget(vm)) {

        width: Fill,
        height: Fit,
        margin: 0.0
        flow: Down,
        cursor: MouseCursor.Default,
        padding: 0.0,
        spacing: 0.0

        show_bg: true
        draw_bg +: {
            highlight: instance(0.0)
            hover: instance(0.0)
            color: instance((COLOR_TELEGRAM_BG))

            mentions_bar_color: instance((COLOR_TELEGRAM_BG))
            mentions_bar_width: instance(4.0)

            pixel: fn() {
                let base_color = mix(
                    self.color,
                    #xEAF7FCD8,
                    self.hover
                );

                let with_highlight = mix(
                    base_color,
                    #xBDEFFF88,
                    self.highlight
                );

                let sdf = Sdf2d.viewport(self.pos * self.rect_size);

                // draw bg
                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill(with_highlight);

                // draw the left vertical line
                sdf.rect(0., 0., self.mentions_bar_width, self.rect_size.y);
                sdf.fill(self.mentions_bar_color);

                return sdf.result;
            }
        }

        animator: Animator{
            highlight: {
                default: @off
                off: AnimatorState{
                    redraw: true,
                    from: { all: Forward {duration: 2.0} }
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: { draw_bg: {highlight: 0.0} }
                }
                on: AnimatorState{
                    redraw: true,
                    from: { all: Forward {duration: 0.5} }
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: { draw_bg: {highlight: 1.0} }
                }
            }
            hover: {
                default: @off
                off: AnimatorState{
                    redraw: true,
                    from: { all: Snap }
                    apply: { draw_bg: {hover: 0.0} }
                }
                on: AnimatorState{
                    redraw: true,
                    from: { all: Snap }
                    apply: { draw_bg: {hover: 1.0} }
                }
            }
        }

        // A preview of the earlier message that this message was in reply to.
        replied_to_message := mod.widgets.RepliedToMessage {
            flow: Right
            margin: Inset{ bottom: 3, top: 10 }
            replied_to_message_content +: {
                margin +: { left: 29 }
                padding +: { bottom: 10 }
            }
        }

        body := View {
            width: Fill,
            height: Fit
            flow: Right,
            padding: Inset{top: 0, bottom: 10, left: 10, right: 10},

            profile := View {
                align: Align{x: 0.5, y: 0.0} // centered horizontally, top aligned
                width: 65.0,
                height: Fit,
                margin: Inset{top: 4.5, right: 10}
                flow: Down,
                avatar := Avatar {
                    width: 48,
                    height: 48,
                }
                timestamp := Timestamp {
                    margin: Inset{ top: 5.9 }
                }
                edited_indicator := EditedIndicator { }
                tsp_sign_indicator := TspSignIndicator { }
            }

            content := View {
                width: Fill,
                height: Fit
                flow: Down,
                padding: 0.0

                username_view := View {
                    flow: Right,
                    width: Fill,
                    height: Fit,
                    username := Label {
                        width: Fill,
                        flow: Right, // do not wrap
                        padding: 0,
                        margin: Inset{bottom: 9.0, top: 20.0, right: 10.0,}
                        max_lines: 1
                        text_overflow: Ellipsis
                        draw_text +: {
                            text_style: USERNAME_TEXT_STYLE {},
                            color: (COLOR_TELEGRAM_BLUE)
                        }
                        text: "<Username not available>"
                    }
                }
                sender_profile_read_evidence := View {
                    visible: false
                    width: Fill,
                    height: Fit
                    margin: Inset{top: -6.0, bottom: 7.0, right: 10.0}
                    status := Label {
                        width: Fill,
                        height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            text_style: MESSAGE_TEXT_STYLE { font_size: 10.0 },
                            color: (COLOR_TELEGRAM_DIM)
                        }
                        text: ""
                    }
                }

                message := HtmlOrPlaintext {
                    plaintext_view +: {
                        pt_label +: {
                            draw_text +: { color: (COLOR_TELEGRAM_TEXT) }
                        }
                    }
                    html_view +: {
                        html +: {
                            font_color: (COLOR_TELEGRAM_TEXT)
                            draw_text +: { color: (COLOR_TELEGRAM_TEXT) }
                        }
                    }
                }
                local_send_status := View {
                    visible: false
                    width: Fill,
                    height: Fit
                    margin: Inset{top: 4.0, bottom: 5.0, right: 10.0}
                    status := Label {
                        width: Fill,
                        height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            text_style: MESSAGE_TEXT_STYLE { font_size: 10.0 },
                            color: (COLOR_TELEGRAM_DIM)
                        }
                        text: ""
                    }
                }
                link_preview_view := mod.widgets.LinkPreview {}
                View {
                    width: Fill,
                    height: Fit
                    flow: Right,
                    reaction_list := mod.widgets.ReactionList { }
                    avatar_row := mod.widgets.AvatarRow {}
                }
                thread_root_summary := mod.widgets.ThreadRootSummary {}
            }
        }
    }

    // The view used for a condensed message that came right after another message
    // from the same sender, and thus doesn't need to display the sender's profile again.
    mod.widgets.CondensedMessage = mod.widgets.Message {
        padding: Inset{ top: 2.0, bottom: 2.0 }
        replied_to_message +: {
            replied_to_message_content +: {
                margin: Inset{ left: 74, bottom: 5.0 }
            }
        }
        body := View {
            width: Fill,
            height: Fit
            flow: Right,
            padding: Inset{ top: 0, bottom: 2.5, left: 10.0, right: 10.0 },
            profile := View {
                align: Align{x: 0.5, y: 0.0} // centered horizontally, top aligned
                width: 65.0,
                height: Fit,
                flow: Down,
                timestamp := Timestamp {
                    margin: Inset{top: 2.5}
                }
                edited_indicator := EditedIndicator { }
                tsp_sign_indicator := TspSignIndicator { }
            }
            content := View {
                width: Fill,
                height: Fit,
                flow: Down,
                padding: Inset{ left: 10.0 }

                message := HtmlOrPlaintext {
                    plaintext_view +: {
                        pt_label +: {
                            draw_text +: { color: (COLOR_TELEGRAM_TEXT) }
                        }
                    }
                    html_view +: {
                        html +: {
                            font_color: (COLOR_TELEGRAM_TEXT)
                            draw_text +: { color: (COLOR_TELEGRAM_TEXT) }
                        }
                    }
                }
                local_send_status := View {
                    visible: false
                    width: Fill,
                    height: Fit
                    margin: Inset{top: 4.0, bottom: 5.0, right: 10.0}
                    status := Label {
                        width: Fill,
                        height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            text_style: MESSAGE_TEXT_STYLE { font_size: 10.0 },
                            color: (COLOR_TELEGRAM_DIM)
                        }
                        text: ""
                    }
                }
                link_preview_view := mod.widgets.LinkPreview {}
                View {
                    width: Fill,
                    height: Fit
                    flow: Right,
                    reaction_list := mod.widgets.ReactionList { }
                    avatar_row := mod.widgets.AvatarRow {}
                }
                thread_root_summary := mod.widgets.ThreadRootSummary {}
            }
        }
    }

    // A single, shared `Size::Fit{max: ...}` object on the script heap,
    // referenced by every `Image` widget inside an `ImageMessage` /
    // `CondensedImageMessage`. Having one heap object instead of many
    // lets the "Maximum Image Thumbnail Height" App Setting mutate just
    // this object's `max` field at runtime (see
    // `AppPreferences::on_thumbnail_max_height_changed`) — every widget
    // whose `walk.height` referenced this object observes the change
    // through the same heap slot on the next `Event::ScriptReapply`.
    //
    // This sidesteps the override-chain divergence that would otherwise
    // make the mutation invisible to derived templates (e.g., the
    // `ImageMessage := mod.widgets.ImageMessage {}` local alias inside a
    // PortalList's `list`).
    mod.widgets.IMG_MSG_FIT = Fit{max: FitBound.Abs(200.0)}

    // The view used for each static image-based message event in a room's timeline.
    // This excludes stickers and other animated GIFs, video clips, audio clips, etc.
    mod.widgets.ImageMessage = mod.widgets.Message {
        body +: {
            content +: {
                message := TextOrImage {
                    // Cap the height on the `Image` itself (not the outer view) so
                    // that `ImageFit.Smallest` scales the texture proportionally
                    // instead of the outer view just clipping the drawn pixels.
                    image_view +: { image +: {
                        height: (mod.widgets.IMG_MSG_FIT)
                    } }
                    default_image_view +: { image +: {
                        height: (mod.widgets.IMG_MSG_FIT)
                    } }
                }
                View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    reaction_list := mod.widgets.ReactionList { }
                    avatar_row := mod.widgets.AvatarRow {}
                }
                thread_root_summary := mod.widgets.ThreadRootSummary {}
            }

        }
    }

    // The view used for a condensed image message that came right after another message
    // from the same sender, and thus doesn't need to display the sender's profile again.
    // This excludes stickers and other animated GIFs, video clips, audio clips, etc.
    mod.widgets.CondensedImageMessage = mod.widgets.CondensedMessage {
        body +: {
            content +: {
                message := TextOrImage {
                    image_view +: { image +: {
                        height: (mod.widgets.IMG_MSG_FIT)
                    } }
                    default_image_view +: { image +: {
                        height: (mod.widgets.IMG_MSG_FIT)
                    } }
                }
                View {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    reaction_list := mod.widgets.ReactionList { }
                    avatar_row := mod.widgets.AvatarRow {}
                }
                thread_root_summary := mod.widgets.ThreadRootSummary {}
            }
        }
    }


    // The view used for each state event (non-messages) in a room's timeline.
    // The timestamp, profile picture, and text are all very small.
    mod.widgets.SmallStateEvent = View {
        width: Fill,
        height: Fit,
        flow: Right,
        margin: Inset{ top: 4.0, bottom: 4.0}
        padding: Inset{ top: 1.0, bottom: 1.0, right: 10.0 }
        spacing: 0.0
        cursor: MouseCursor.Default

        body := View {
            width: Fill,
            height: Fit
            flow: Right,
            padding: Inset{ left: 7.0, top: 2.0, bottom: 2.0 }
            spacing: 5.0

            left_container := View {
                align: Align{x: 0.5, y: 0}
                width: 70.0,
                height: Fit

                timestamp := Timestamp {
                    margin: Inset{top: 3}
                }
            }

            avatar := Avatar {
                width: 19.,
                height: 19.,
                margin: 0

                text_view +: {
                    text +: {
                        draw_text +: {
                            text_style: TITLE_TEXT { font_size: 7.0 }
                        }
                    }
                }
            }

            // Show an invite button only for a `Knocked` room membership change.
            // All other small state events will not show this button.
            invite_user_button := RobrixPositiveIconButton {
                visible: false
                margin: Inset{ top: -1.5, left: 2, right: 2}
                padding: Inset{top: 4, bottom: 4, left: 9, right: 9}
                draw_bg +: {
                    border_size: 0.75
                }
                draw_icon.svg: (ICON_ADD_USER)
                draw_text.text_style: SMALL_STATE_TEXT_STYLE {}
                icon_walk: Walk{width: 15, height: Fit, margin: Inset{right: -4}}
                text: "Invite to Room"
            }

            content := Label {
                width: Fill,
                height: Fit
                flow: Flow.Right{wrap: true},
                margin: Inset{top: 2.5}
                padding: Inset{ top: 0.0, bottom: 0.0, left: 0.0, right: 0.0 }
                draw_text +: {
                    text_style: SMALL_STATE_TEXT_STYLE {},
                    color: (SMALL_STATE_TEXT_COLOR)
                }
                text: ""
            }

            avatar_row := mod.widgets.AvatarRow {}
        }
    }

    // First-class card renderer for Hepta custom Matrix-style message events.
    // This keeps Robrix's Matrix timeline substrate intact while making
    // runtime/task/tool/approval events visibly Hepta-native in the UI.
    mod.widgets.HeptaEventCard = RoundedView {
        width: Fill,
        height: Fit,
        flow: Down,
        margin: Inset{ top: 6.0, bottom: 6.0, left: 74.0, right: 14.0 }
        padding: Inset{ top: 12.0, bottom: 12.0, left: 14.0, right: 14.0 }
        spacing: 7.0
        show_bg: true
        draw_bg +: {
            color: #xF4F7FF
            border_color: #x8EA7FF
            border_size: 1.0
            border_radius: 7.0
        }

        header := View {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8.0,
            align: Align{ y: 0.5 }

            eyebrow := Label {
                width: Fill,
                height: Fit,
                draw_text +: {
                    color: #x324A7A,
                    text_style: theme.font_bold { font_size: 11.5 }
                }
                text: "Hepta event"
            }

            status := Label {
                width: Fit,
                height: Fit,
                padding: Inset{ left: 7.0, right: 7.0, top: 3.0, bottom: 3.0 }
                draw_text +: {
                    color: #x20345F,
                    text_style: theme.font_bold { font_size: 10.5 }
                }
                text: "running"
            }
        }

        title := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{ wrap: true }
            draw_text +: {
                color: #x16213D,
                text_style: theme.font_bold { font_size: 14.0 }
            }
            text: "Runtime event"
        }

        body := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{ wrap: true }
            draw_text +: {
                color: #x263654,
                text_style: theme.font_regular { font_size: 12.0 }
            }
            text: ""
        }

        meta := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{ wrap: true }
            draw_text +: {
                color: #x657292,
                text_style: theme.font_regular { font_size: 10.5 }
            }
            text: ""
        }

        policy := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{ wrap: true }
            draw_text +: {
                color: #x43547C,
                text_style: theme.font_bold { font_size: 10.5 }
            }
            text: "policy: local preview"
        }

        hepta_actions := View {
            visible: true,
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8.0,
            margin: Inset{ top: 3.0 }

            inspect_button := RobrixNeutralIconButton {
                width: Fit,
                height: Fit,
                padding: Inset{ top: 5.0, bottom: 5.0, left: 10.0, right: 10.0 }
                text: "Inspect payload"
            }

            approve_button := RobrixPositiveIconButton {
                visible: false,
                width: Fit,
                height: Fit,
                padding: Inset{ top: 5.0, bottom: 5.0, left: 10.0, right: 10.0 }
                text: "Approve"
            }

            reject_button := RobrixNegativeIconButton {
                visible: false,
                width: Fit,
                height: Fit,
                padding: Inset{ top: 5.0, bottom: 5.0, left: 10.0, right: 10.0 }
                text: "Reject"
            }
        }
    }


    // The view used for each day divider in a room's timeline.
    // The date text is centered between two horizontal lines.
    mod.widgets.DateDivider = View {
        width: Fill,
        height: Fit,
        margin: Inset{top: 7.0, bottom: 7.0}
        flow: Right,
        padding: Inset{left: 7.0, right: 7.0},
        spacing: 0.0,
        align: Align{x: 0.5, y: 0.5} // center horizontally and vertically

        left_line := LineH { }

        date := Label {
            padding: Inset{left: 7.0, right: 7.0}
            draw_text +: {
                text_style: TEXT_SUB {},
                color: (COLOR_TELEGRAM_DIM)
            }
            text: "<date>"
        }

        right_line := LineH { }
    }

    // The view used for the divider indicating where the user's last-viewed message is.
    // This is implemented as a DateDivider with a different color and a fixed text label.
    mod.widgets.ReadMarker = mod.widgets.DateDivider {
        left_line := LineH {
            draw_bg.color: (mod.widgets.COLOR_READ_MARKER)
        }

        date := Label {
            draw_text.color: (mod.widgets.COLOR_READ_MARKER)
            text: "New Messages"
        }

        right_line := LineH {
            draw_bg.color: (mod.widgets.COLOR_READ_MARKER)
        }
    }


    // The top space is used to display a loading message while the room is being paginated.
    mod.widgets.TopSpace = SolidView {
        visible: false,
        width: Fill,
        height: Fit,
        align: Align{x: 0.5, y: 0}
        flow: Right,
        show_bg: true,
        draw_bg.color: (COLOR_TELEGRAM_PANEL)

        label := Label {
            width: Fill,
            height: Fit,
            align: Align{x: 0.5, y: 0.5},
            flow: Right,
            padding: Inset{ top: 10.0, bottom: 7.0, left: 15.0, right: 15.0 }
            draw_text +: {
                text_style: MESSAGE_TEXT_STYLE { font_size: 10 },
                color: (COLOR_TELEGRAM_MUTED)
            }
            text: "Loading earlier messages..."
        }
    }

    mod.widgets.Timeline = View {
        width: Fill,
        height: Fill,
        align: Align{x: 0.5, y: 0.0} // center horizontally, align to top vertically
        flow: Overlay,

        list := PortalList {
            height: Fill,
            width: Fill
            flow: Down

            auto_tail: true, // set to `true` to lock the view to the last item.
            max_pull_down: 0.0, // set to `0.0` to disable the pulldown bounce animation.
            // TODO: enable `reuse_items: true` once Makepad's Html/TextFlow widget
            //   properly resets all internal state during `script_apply(Reload)`.
            //   Currently, stale TextFlow layout state (particularly related to
            //   list items) leaks through when a widget is recycled, causing
            //   excessive whitespace in HTML messages with `<ul>`/`<ol>` lists.

            // Below, we must place all of the possible templates (views) that can be used in the portal list.
            Message := mod.widgets.Message {}
            CondensedMessage := mod.widgets.CondensedMessage {}
            ImageMessage := mod.widgets.ImageMessage {}
            CondensedImageMessage := mod.widgets.CondensedImageMessage {}
            SmallStateEvent := mod.widgets.SmallStateEvent {}
            Empty := mod.widgets.Empty {}
            DateDivider := mod.widgets.DateDivider {}
            ReadMarker := mod.widgets.ReadMarker {}
        }

        // A jump to bottom button (with an unread message badge) that is shown
        // when the timeline is not at the bottom.
        jump_to_bottom_button := JumpToBottomButton { }
    }


    mod.widgets.RoomScreen = #(RoomScreen::register_widget(vm)) {
        width: Fill, height: Fill,
        cursor: MouseCursor.Default,
        flow: Down,
        spacing: 0.0

        room_screen_wrapper := SolidView {
            width: Fill, height: Fill,
            flow: Overlay,

            show_bg: true
            draw_bg.color: (COLOR_TELEGRAM_BG)

            restore_status_view := RestoreStatusView {}

            // This used to be a KeyboardView wrapper, but now the on-screen keyboard shift
            // is handled by the top-level Window.
            timeline_and_input_bar := View {
                width: Fill, height: Fill,
                flow: Down,

                telegram_room_header := RoundedView {
                    width: Fill,
                    height: 58,
                    flow: Right,
                    spacing: 10.0,
                    align: Align{y: 0.5},
                    padding: Inset{top: 8.0, bottom: 8.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_PANEL)
                        border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    title_stack := View {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: 2.0

                        title := Label {
                            width: Fill,
                            height: Fit,
                            flow: Flow.Right{wrap: false},
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                text_style: theme.font_bold { font_size: 13.5 }
                            }
                            text: "Chat"
                        }

                        status := Label {
                            width: Fill,
                            height: Fit,
                            flow: Flow.Right{wrap: false},
                            draw_text +: {
                                color: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 10.5 }
                            }
                            text: "local chat ready"
                        }
                    }

                    search_button := mod.widgets.TelegramRoomHeaderIconButton {
                        draw_icon.svg: (ICON_SEARCH)
                        text: ""
                    }

                    info_button := mod.widgets.TelegramRoomHeaderIconButton {
                        draw_icon.svg: (ICON_INFO)
                        text: ""
                    }

                    mute_button := mod.widgets.TelegramRoomHeaderIconButton {
                        draw_icon.svg: (ICON_FORBIDDEN)
                        text: ""
                    }

                    menu_button := mod.widgets.TelegramRoomHeaderIconButton {
                        draw_icon.svg: (ICON_MENU)
                        text: ""
                    }
                }

                telegram_message_search_strip := RoundedView {
                    visible: false
                    width: Fill,
                    height: 328,
                    flow: Down,
                    spacing: 6.0,
                    padding: Inset{top: 7.0, bottom: 7.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        border_color: (COLOR_TELEGRAM_BORDER)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    search_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0,
                        align: Align{y: 0.5}

                        search_icon := Icon {
                            width: Fit,
                            height: Fit,
                            draw_icon.svg: (ICON_SEARCH)
                            draw_icon.color: (COLOR_TELEGRAM_MUTED)
                            icon_walk: Walk{width: 15, height: 15}
                        }

                        search_input := RobrixTextInput {
                            width: Fill,
                            height: 30,
                            flow: Right,
                            padding: Inset{top: 5, bottom: 5, left: 8, right: 8}
                            empty_text: "Search in this chat"

                            draw_bg +: {
                                border_radius: 15.0
                                border_size: 1.0
                                color: (COLOR_TELEGRAM_PANEL)
                                color_hover: (COLOR_TELEGRAM_PANEL)
                                color_focus: (COLOR_TELEGRAM_PANEL)
                                color_down: (COLOR_TELEGRAM_PANEL)
                                color_empty: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_BORDER)
                                border_color_hover: (COLOR_TELEGRAM_BLUE)
                                border_color_focus: (COLOR_TELEGRAM_BLUE)
                                border_color_down: (COLOR_TELEGRAM_BLUE)
                                border_color_empty: (COLOR_TELEGRAM_BORDER)
                            }
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                color_hover: (COLOR_TELEGRAM_TEXT)
                                color_focus: (COLOR_TELEGRAM_TEXT)
                                color_down: (COLOR_TELEGRAM_TEXT)
                                color_empty: (COLOR_TELEGRAM_DIM)
                                color_empty_hover: (COLOR_TELEGRAM_DIM)
                                color_empty_focus: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 11.5 }
                            }
                            draw_cursor +: {
                                color: (COLOR_TELEGRAM_TEXT)
                            }
                        }

                        status := Label {
                            width: Fit,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_MUTED)
                                text_style: theme.font_bold { font_size: 10.0 }
                            }
                            text: "local only"
                        }

                        prev_search_result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_UP)
                            text: "Prev"
                        }

                        next_search_result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_DOWN)
                            text: "Next"
                        }

                        close_search_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CLOSE)
                            text: "Close"
                        }
                    }

                    results_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0,
                        align: Align{y: 0.5}

                        result_summary := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 10.5 }
                            }
                            text: "Loaded timeline only: no Matrix-backed history search, event fetch, pagination, or mutation."
                        }
                    }

                    search_evidence := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Loaded timeline only; Close/Escape and 0-result states send no Matrix-backed search query."
                    }

                    loaded_search_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Loaded search metadata: query empty, 0 loaded items, 0 matches, no active match. Loaded search metadata only; no server-side search."
                    }

                    active_search_result_detail := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Active result detail waits for a loaded local match; no server-side search, event context, pagination, or mutation."
                    }

                    search_result_action_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        search_result_jump_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_DOWN)
                            text: "Jump"
                        }

                        search_result_copy_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Copy"
                        }

                        search_result_source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Source"
                        }

                        search_result_thread_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Thread"
                        }

                        search_result_sender_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Sender"
                        }
                    }

                    search_result_action_controls_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Jump, Copy, Source, Thread, and Sender stay local result-action controls."
                    }

                    search_server_context_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        search_server_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Server"
                        }

                        search_event_context_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Context"
                        }

                        search_load_older_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_UP)
                            text: "Older"
                        }
                    }

                    search_advanced_filter_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        search_filter_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Filter"
                        }

                        search_from_input := RobrixTextInput {
                            width: 220,
                            height: 28,
                            flow: Right,
                            padding: Inset{top: 4, bottom: 4, left: 8, right: 8}
                            empty_text: "@sender:server"

                            draw_bg +: {
                                border_radius: 14.0
                                border_size: 1.0
                                color: (COLOR_TELEGRAM_PANEL)
                                color_hover: (COLOR_TELEGRAM_PANEL)
                                color_focus: (COLOR_TELEGRAM_PANEL)
                                color_down: (COLOR_TELEGRAM_PANEL)
                                color_empty: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_BORDER)
                                border_color_hover: (COLOR_TELEGRAM_BLUE)
                                border_color_focus: (COLOR_TELEGRAM_BLUE)
                                border_color_down: (COLOR_TELEGRAM_BLUE)
                                border_color_empty: (COLOR_TELEGRAM_BORDER)
                            }
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                color_hover: (COLOR_TELEGRAM_TEXT)
                                color_focus: (COLOR_TELEGRAM_TEXT)
                                color_down: (COLOR_TELEGRAM_TEXT)
                                color_empty: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 10.0 }
                            }
                        }

                        search_from_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "From"
                        }

                        search_date_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_DOWN)
                            text: "Date"
                        }

                        search_media_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Media"
                        }

                        search_pins_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_PIN)
                            text: "Pins"
                        }
                    }

                    search_server_preflight_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        search_server_query_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Server query"
                        }

                        search_server_packet_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Packet"
                        }

                        search_server_contract_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Contract"
                        }

                        search_server_result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Result"
                        }

                        search_server_error_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Error"
                        }

                        search_server_retry_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_UP)
                            text: "Retry"
                        }

                        search_server_scope_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Scope"
                        }

                        search_server_taxonomy_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Taxonomy"
                        }
                    }

                    search_server_preflight_controls_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                            text: "Server query and Retry use live Matrix search; Packet, Contract, Result, Error, Scope, and Taxonomy stay local."
                    }

                    search_advanced_filter_controls_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "From/Media can run server filters; Filter/Date/Pins rescan loaded scope."
                    }

                    search_server_context_controls_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Server/Older use live Matrix search; context previews parse server windows."
                    }

                    server_context_boundary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Server/Older use live Matrix search; Context jumps current-room hits; Date/Pins stay loaded-scope."
                    }
                }

                telegram_message_edit_history_strip := RoundedView {
                    visible: false
                    width: Fill,
                    height: 282,
                    flow: Down,
                    spacing: 6.0,
                    padding: Inset{top: 8.0, bottom: 8.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        border_color: (COLOR_TELEGRAM_BORDER)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    edit_history_header := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0,
                        align: Align{y: 0.5}

                        edit_history_title := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                text_style: theme.font_bold { font_size: 12.0 }
                            }
                            text: "Edit history"
                        }

                        edit_history_status := Label {
                            width: Fit,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_MUTED)
                                text_style: theme.font_bold { font_size: 10.0 }
                            }
                            text: "read only"
                        }

                        retry_edit_history_button := mod.widgets.TelegramRoomHeaderButton {
                            visible: false
                            draw_icon.svg: (ICON_SEND)
                            text: "Retry"
                        }

                        close_edit_history_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CLOSE)
                            text: "Close"
                        }
                    }

                    edit_history_full_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        full_history_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Full"
                        }

                        full_diff_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Diff"
                        }

                        event_context_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Context"
                        }

                        event_source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_LINK)
                            text: "Source"
                        }

                        edit_history_packet_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Packet"
                        }

                        edit_history_contract_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Contract"
                        }

                        edit_history_taxonomy_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Taxonomy"
                        }
                    }

                    edit_history_preflight_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        edit_history_request_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEND)
                            text: "Request"
                        }

                        edit_history_result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Result"
                        }

                        edit_history_error_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Error"
                        }

                        edit_history_retry_detail_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_UP)
                            text: "Retry"
                        }

                        edit_history_source_detail_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_LINK)
                            text: "Source"
                        }
                    }

                    edit_history_summary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_TEXT)
                            text_style: theme.font_regular { font_size: 11.0 }
                        }
                        text: "Click an edited badge to read a compact m.replace summary."
                    }

                    edit_history_diff := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Loaded original and latest replacement previews stay local."
                    }

                    edit_history_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "No event context, timeline reload, event source, message mutation, room-state, or live mutation."
                    }

                    edit_history_loaded_diff_detail := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Loaded diff detail waits for compact m.replace summary; no full modal, event context, event source, or mutation."
                    }

                    edit_history_preflight_detail := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Preflight detail waits for Request/Result/Error/Retry/Source; no extra FetchEditHistory."
                    }

                    edit_history_full_modal_boundary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Full history modal, full diff, event context, reload, and event source stay local blocked controls."
                    }
                }

                telegram_message_report_status_strip := RoundedView {
                    visible: false
                    width: Fill,
                    height: 202,
                    flow: Down,
                    spacing: 6.0,
                    padding: Inset{top: 8.0, bottom: 8.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        border_color: (COLOR_TELEGRAM_BORDER)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    report_status_header := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0,
                        align: Align{y: 0.5}

                        report_status_title := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                text_style: theme.font_bold { font_size: 12.0 }
                            }
                            text: "Report status"
                        }

                        report_status_badge := Label {
                            width: Fit,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_MUTED)
                                text_style: theme.font_bold { font_size: 10.0 }
                            }
                            text: "waiting"
                        }

                        retry_report_status_button := mod.widgets.TelegramRoomHeaderButton {
                            visible: false
                            draw_icon.svg: (ICON_SEND)
                            text: "Retry"
                        }

                        close_report_status_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CLOSE)
                            text: "Close"
                        }
                    }

                    report_workflow_actions := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        report_queue_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Queue"
                        }

                        report_policy_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Policy"
                        }

                        report_assign_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_ADD_USER)
                            text: "Assign"
                        }

                        report_appeal_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Appeal"
                        }

                        report_enforce_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TOMBSTONE)
                            text: "Enforce"
                        }

                        report_moderation_packet_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Packet"
                        }

                        report_workflow_contract_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Contract"
                        }

                        report_workflow_taxonomy_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Taxonomy"
                        }
                    }

                    report_preflight_detail_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        report_request_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEND)
                            text: "Request"
                        }

                        report_result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Result"
                        }

                        report_copy_status_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Copy"
                        }

                        report_error_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Error"
                        }

                        report_retry_detail_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_UP)
                            text: "Retry"
                        }

                        report_source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Source"
                        }
                    }

                    report_status_summary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_TEXT)
                            text_style: theme.font_regular { font_size: 11.0 }
                        }
                        text: "Report status appears after a confirmed Matrix report_content request."
                    }

                    report_preflight_detail_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Request, Result, Error, Retry, and Source stay local ReportContent preflight details."
                    }

                    report_status_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Retry confirms before ReportContent; cancel queue, policy lookup, ban, kick, room-state, membership, and live mutation stay unwired."
                    }
                }

                telegram_matrix_link_preview_strip := RoundedView {
                    visible: false
                    width: Fill,
                    height: 304,
                    flow: Down,
                    spacing: 6.0,
                    padding: Inset{top: 8.0, bottom: 8.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        border_color: (COLOR_TELEGRAM_BORDER)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    matrix_link_preview_header := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0,
                        align: Align{y: 0.5}

                        matrix_link_preview_title := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                text_style: theme.font_bold { font_size: 12.0 }
                            }
                            text: "Matrix link"
                        }

                        matrix_link_preview_status := Label {
                            width: Fit,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_MUTED)
                                text_style: theme.font_bold { font_size: 10.0 }
                            }
                            text: "preview"
                        }

                        retry_matrix_link_preview_button := mod.widgets.TelegramRoomHeaderButton {
                            visible: false
                            draw_icon.svg: (ICON_SEND)
                            text: "Retry"
                        }

                        close_matrix_link_preview_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CLOSE)
                            text: "Close"
                        }
                    }

                    matrix_link_preview_summary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_TEXT)
                            text_style: theme.font_regular { font_size: 11.0 }
                        }
                        text: "Unknown Matrix links use compact room preview only."
                    }

                    matrix_link_preview_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Retry confirms before PreviewMatrixLinkTarget; event context, join, knock, browser handoff, and live mutation stay unwired."
                    }

                    matrix_link_unresolved_detail := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Unresolved detail waits for a compact Matrix link preview target; no alias resolution, event context, join, event source, or mutation."
                    }

                    matrix_link_route_scope_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        matrix_link_route_room_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Room"
                        }

                        matrix_link_route_event_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Event"
                        }

                        matrix_link_route_via_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Via"
                        }

                        matrix_link_route_preview_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Preview"
                        }

                        matrix_link_route_source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Source"
                        }

                        matrix_link_route_packet_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Packet"
                        }

                        matrix_link_route_contract_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Contract"
                        }

                        matrix_link_route_taxonomy_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Taxonomy"
                        }
                    }

                    matrix_link_route_scope_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Room, Event, Via, Preview, Source, Packet, Contract, and Taxonomy route-scope controls stay local."
                    }

                    matrix_link_context_actions := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        matrix_link_server_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Server"
                        }

                        matrix_link_event_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Event"
                        }

                        matrix_link_alias_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Alias"
                        }

                        matrix_link_join_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_ADD_USER)
                            text: "Join"
                        }

                        matrix_link_knock_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_ADD_USER)
                            text: "Knock"
                        }

                        matrix_link_invite_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_ADD_USER)
                            text: "Invite"
                        }

                        matrix_link_browser_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_EXTERNAL_LINK)
                            text: "Browser"
                        }

                        matrix_link_source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Source"
                        }
                    }

                    matrix_link_context_actions_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Server, Event, Alias, Join, Knock, Invite, and Source controls stay guarded; Browser confirms before matrix.to system opener."
                    }

                    matrix_link_server_context_boundary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Server context, event context, extra pagination, browser handoff before confirmation, and full event source stay bounded while Join/Knock/Invite confirm before membership handoff."
                    }
                }

                telegram_room_actions_strip := RoundedView {
                    visible: false
                    width: Fill,
                    height: 114,
                    flow: Flow.Right{wrap: true},
                    spacing: 6.0,
                    align: Align{y: 0.5},
                    padding: Inset{top: 7.0, bottom: 7.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        border_color: (COLOR_TELEGRAM_BORDER)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    room_action_status := Label {
                        width: Fit,
                        height: Fit,
                        flow: Flow.Right{wrap: false},
                        margin: Inset{right: 4.0}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_TEXT)
                            text_style: theme.font_bold { font_size: 11.5 }
                        }
                        text: "Room actions"
                    }

                    copy_link_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_LINK)
                        text: "Link"
                    }

                    invite_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_ADD_USER)
                        text: "Invite"
                    }

                    leave_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_LOGOUT)
                        text: "Leave"
                    }

                    mark_unread_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_CHECKMARK)
                        text: "Unread"
                    }

                    favorite_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_PIN)
                        text: "Fav"
                    }

                    priority_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_TOMBSTONE)
                        text: "Low"
                    }

                    room_settings_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_SETTINGS)
                        text: "Settings"
                    }

                    room_info_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_INFO)
                        text: "Info"
                    }

                    notifications_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_SETTINGS)
                        text: "Mute"
                    }

                    close_room_actions_button := mod.widgets.TelegramRoomHeaderButton {
                        draw_icon.svg: (ICON_CLOSE)
                        text: "Close"
                    }

                    room_actions_close_evidence := Label {
                        width: Fill,
                        height: Fit,
                        margin: Inset{top: 1.0}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Close only dismisses this local action preview; no Matrix search, room-state, notification, message, or membership request is sent."
                    }
                }

                telegram_notifications_strip := RoundedView {
                    visible: false
                    width: Fill,
                    height: 468,
                    flow: Down,
                    spacing: 6.0,
                    padding: Inset{top: 8.0, bottom: 8.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        border_color: (COLOR_TELEGRAM_BORDER)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    notifications_header := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0,
                        align: Align{y: 0.5}

                        notifications_title := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                text_style: theme.font_bold { font_size: 12.0 }
                            }
                            text: "Notifications"
                        }

                        notifications_status := Label {
                            width: Fit,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_MUTED)
                                text_style: theme.font_bold { font_size: 10.0 }
                            }
                            text: "local only"
                        }

                        retry_notifications_button := mod.widgets.TelegramRoomHeaderButton {
                            visible: false
                            draw_icon.svg: (ICON_SEND)
                            text: "Retry"
                        }

                        close_notifications_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CLOSE)
                            text: "Close"
                        }
                    }

                    notification_options := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        mute_1h_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "All"
                        }

                        mute_8h_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Mentions"
                        }

                        mute_forever_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Mute"
                        }

                        unmute_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Refresh"
                        }

                        copy_mode_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Copy mode"
                        }
                    }

                    notification_advanced_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        timed_mute_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Timed"
                        }

                        keyword_rules_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Keywords"
                        }

                        pusher_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SETTINGS)
                            text: "Pusher"
                        }

                        global_preferences_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SETTINGS)
                            text: "Global"
                        }
                    }

                    notification_keyword_write_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        keyword_rule_input := RobrixTextInput {
                            width: Fill,
                            height: 28,
                            flow: Right,
                            padding: Inset{top: 4, bottom: 4, left: 8, right: 8}
                            empty_text: "Keyword rule"

                            draw_bg +: {
                                border_radius: 8.0
                                border_size: 1.0
                                color: (COLOR_TELEGRAM_PANEL)
                                color_hover: (COLOR_TELEGRAM_PANEL)
                                color_focus: (COLOR_TELEGRAM_PANEL)
                                color_down: (COLOR_TELEGRAM_PANEL)
                                color_empty: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_BORDER)
                                border_color_hover: (COLOR_TELEGRAM_BLUE)
                                border_color_focus: (COLOR_TELEGRAM_BLUE)
                                border_color_down: (COLOR_TELEGRAM_BLUE)
                                border_color_empty: (COLOR_TELEGRAM_BORDER)
                            }
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                color_hover: (COLOR_TELEGRAM_TEXT)
                                color_focus: (COLOR_TELEGRAM_TEXT)
                                color_down: (COLOR_TELEGRAM_TEXT)
                                color_empty: (COLOR_TELEGRAM_DIM)
                                color_empty_hover: (COLOR_TELEGRAM_DIM)
                                color_empty_focus: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 11.0 }
                            }
                            draw_cursor +: {
                                color: (COLOR_TELEGRAM_TEXT)
                            }
                        }

                        add_keyword_rule_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Add keyword"
                        }

                        remove_keyword_rule_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRASH)
                            text: "Remove keyword"
                        }
                    }

                    notification_default_mode_write_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        default_all_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Default all"
                        }

                        default_mentions_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Default mentions"
                        }

                        default_mute_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TOMBSTONE)
                            text: "Default mute"
                        }
                    }

                    notification_advanced_detail_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        quiet_hours_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Quiet hours"
                        }

                        keyword_list_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Keyword list"
                        }

                        device_push_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SETTINGS)
                            text: "Device push"
                        }

                        defaults_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Defaults"
                        }

                        sound_badge_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SETTINGS)
                            text: "Sound badge"
                        }
                    }

                    notification_result_detail_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Result"
                        }

                        requested_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Requested"
                        }

                        retry_cache_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEND)
                            text: "Retry cache"
                        }

                        failure_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TOMBSTONE)
                            text: "Failure"
                        }

                        source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Source"
                        }
                    }

                    notification_preflight_detail_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        schedule_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Schedule"
                        }

                        packet_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Packet"
                        }

                        contract_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Contract"
                        }

                        taxonomy_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Taxonomy"
                        }

                        account_data_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SETTINGS)
                            text: "Account data"
                        }

                        keyword_source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEARCH)
                            text: "Keywords"
                        }

                        pushers_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SETTINGS)
                            text: "Pushers"
                        }

                        preflight_defaults_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Defaults"
                        }
                    }

                    notifications_summary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.5 }
                        }
                        text: "Notification mode is read from Matrix; All, Mentions, and Mute write only after confirmation."
                    }

                    notifications_option_evidence := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Timed mute remains unwired; no message, room-state, membership, gateway/runtime/auth, or live mutation request is sent."
                    }

                    notifications_mode_target_metadata := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Mode target metadata stays local until confirmation."
                    }

                    notifications_result_detail := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Notification result detail stays local."
                    }

                    notifications_preflight_detail := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Notification rule and pusher preflight stays local."
                    }

                    notifications_timed_global_boundary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Failed Retry confirms before SetRoomNotificationMode; timed/global notification controls, keywords, and push gateway/device setup stay local boundary metadata."
                    }

                    notifications_pusher_keyword_boundary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Keyword rules, global preferences, timed mute, pusher setup, sound, and badge controls stay local blocked."
                    }
                }

                telegram_room_info_strip := RoundedView {
                    visible: false
                    width: Fill,
                    height: 190,
                    flow: Down,
                    spacing: 6.0,
                    padding: Inset{top: 8.0, bottom: 8.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        border_color: (COLOR_TELEGRAM_BORDER)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    info_header := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0,
                        align: Align{y: 0.5}

                        info_title := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                text_style: theme.font_bold { font_size: 12.0 }
                            }
                            text: "Room info"
                        }

                        close_info_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CLOSE)
                            text: "Close"
                        }
                    }

                    info_summary := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_TEXT)
                            text_style: theme.font_regular { font_size: 11.5 }
                        }
                        text: "Current room"
                    }

                    info_meta := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.5 }
                        }
                        text: "members: local cache / pinned: live timeline"
                    }

                    info_subscription := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "pinned events: existing subscription; no PinEvent or room-state mutation"
                    }

                    info_typing_subscription := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "incoming typing: existing subscription; no typing notice send"
                    }

                    info_read_receipt_subscription := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "own read marker: existing subscription; no ReadReceipt send"
                    }

                    info_unread_count_read := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "unread badge count: existing read path; no unread status mutation"
                    }

                    info_avatar_fetch := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "avatars: existing FetchAvatar cache; no SetAvatar or FetchMedia"
                    }

                    info_state := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_MUTED)
                            text_style: theme.font_bold { font_size: 10.0 }
                        }
                        text: "state: local room list"
                    }
                }

                telegram_room_settings_strip := RoundedView {
                    visible: false
                    width: Fill,
                    height: 480,
                    flow: Down,
                    spacing: 6.0,
                    padding: Inset{top: 8.0, bottom: 8.0, left: 14.0, right: 12.0}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        border_color: (COLOR_TELEGRAM_BORDER)
                        border_size: 1.0
                        border_radius: 0.0
                    }

                    settings_header := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 8.0,
                        align: Align{y: 0.5}

                        settings_title := Label {
                            width: Fill,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                text_style: theme.font_bold { font_size: 12.0 }
                            }
                            text: "Room settings"
                        }

                        settings_status := Label {
                            width: Fit,
                            height: Fit,
                            draw_text +: {
                                color: (COLOR_TELEGRAM_MUTED)
                                text_style: theme.font_bold { font_size: 10.0 }
                            }
                            text: "local only"
                        }

                        close_settings_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CLOSE)
                            text: "Close"
                        }
                    }

                    settings_options := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        name_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SETTINGS)
                            text: "Name"
                        }

                        topic_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Topic"
                        }

                        permissions_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Perms"
                        }

                        members_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_ADD_USER)
                            text: "Members"
                        }

                        identity_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Identity"
                        }

                        avatar_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_UPLOAD)
                            text: "Avatar"
                        }

                        alias_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_LINK)
                            text: "Alias"
                        }

                        history_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "History"
                        }

                        join_rule_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_ADD_USER)
                            text: "Join rule"
                        }

                        power_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Power"
                        }

                        moderation_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRASH)
                            text: "Moderation"
                        }

                        refresh_settings_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Refresh"
                        }
                    }

                    settings_field_edit_intents := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        name_edit_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SETTINGS)
                            text: "Name edit"
                        }

                        topic_edit_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Topic edit"
                        }

                        avatar_edit_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_UPLOAD)
                            text: "Avatar edit"
                        }

                        remove_avatar_live_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRASH)
                            text: "Remove avatar"
                        }

                        permissions_edit_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Perms edit"
                        }

                        members_edit_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_ADD_USER)
                            text: "Members edit"
                        }
                    }

                    settings_name_write_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        name_live_input := RobrixTextInput {
                            width: Fill,
                            height: 28,
                            flow: Right,
                            padding: Inset{top: 4, bottom: 4, left: 8, right: 8}
                            empty_text: "Room name"

                            draw_bg +: {
                                border_radius: 8.0
                                border_size: 1.0
                                color: (COLOR_TELEGRAM_PANEL)
                                color_hover: (COLOR_TELEGRAM_PANEL)
                                color_focus: (COLOR_TELEGRAM_PANEL)
                                color_down: (COLOR_TELEGRAM_PANEL)
                                color_empty: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_BORDER)
                                border_color_hover: (COLOR_TELEGRAM_BLUE)
                                border_color_focus: (COLOR_TELEGRAM_BLUE)
                                border_color_down: (COLOR_TELEGRAM_BLUE)
                                border_color_empty: (COLOR_TELEGRAM_BORDER)
                            }
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                color_hover: (COLOR_TELEGRAM_TEXT)
                                color_focus: (COLOR_TELEGRAM_TEXT)
                                color_down: (COLOR_TELEGRAM_TEXT)
                                color_empty: (COLOR_TELEGRAM_DIM)
                                color_empty_hover: (COLOR_TELEGRAM_DIM)
                                color_empty_focus: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 11.0 }
                            }
                            draw_cursor +: {
                                color: (COLOR_TELEGRAM_TEXT)
                            }
                        }

                        save_name_live_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Save name"
                        }
                    }

                    settings_topic_write_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        topic_live_input := RobrixTextInput {
                            width: Fill,
                            height: 28,
                            flow: Right,
                            padding: Inset{top: 4, bottom: 4, left: 8, right: 8}
                            empty_text: "Room topic"

                            draw_bg +: {
                                border_radius: 8.0
                                border_size: 1.0
                                color: (COLOR_TELEGRAM_PANEL)
                                color_hover: (COLOR_TELEGRAM_PANEL)
                                color_focus: (COLOR_TELEGRAM_PANEL)
                                color_down: (COLOR_TELEGRAM_PANEL)
                                color_empty: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_BORDER)
                                border_color_hover: (COLOR_TELEGRAM_BLUE)
                                border_color_focus: (COLOR_TELEGRAM_BLUE)
                                border_color_down: (COLOR_TELEGRAM_BLUE)
                                border_color_empty: (COLOR_TELEGRAM_BORDER)
                            }
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                color_hover: (COLOR_TELEGRAM_TEXT)
                                color_focus: (COLOR_TELEGRAM_TEXT)
                                color_down: (COLOR_TELEGRAM_TEXT)
                                color_empty: (COLOR_TELEGRAM_DIM)
                                color_empty_hover: (COLOR_TELEGRAM_DIM)
                                color_empty_focus: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 11.0 }
                            }
                            draw_cursor +: {
                                color: (COLOR_TELEGRAM_TEXT)
                            }
                        }

                        save_topic_live_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Save topic"
                        }
                    }

                    settings_alias_write_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        alias_live_input := RobrixTextInput {
                            width: Fill,
                            height: 28,
                            flow: Right,
                            padding: Inset{top: 4, bottom: 4, left: 8, right: 8}
                            empty_text: "Canonical alias"

                            draw_bg +: {
                                border_radius: 8.0
                                border_size: 1.0
                                color: (COLOR_TELEGRAM_PANEL)
                                color_hover: (COLOR_TELEGRAM_PANEL)
                                color_focus: (COLOR_TELEGRAM_PANEL)
                                color_down: (COLOR_TELEGRAM_PANEL)
                                color_empty: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_BORDER)
                                border_color_hover: (COLOR_TELEGRAM_BLUE)
                                border_color_focus: (COLOR_TELEGRAM_BLUE)
                                border_color_down: (COLOR_TELEGRAM_BLUE)
                                border_color_empty: (COLOR_TELEGRAM_BORDER)
                            }
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                color_hover: (COLOR_TELEGRAM_TEXT)
                                color_focus: (COLOR_TELEGRAM_TEXT)
                                color_down: (COLOR_TELEGRAM_TEXT)
                                color_empty: (COLOR_TELEGRAM_DIM)
                                color_empty_hover: (COLOR_TELEGRAM_DIM)
                                color_empty_focus: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 11.0 }
                            }
                            draw_cursor +: {
                                color: (COLOR_TELEGRAM_TEXT)
                            }
                        }

                        save_alias_live_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Save alias"
                        }
                    }

                    settings_tombstone_write_row := View {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        tombstone_replacement_live_input := RobrixTextInput {
                            width: Fill,
                            height: 28,
                            flow: Right,
                            padding: Inset{top: 4, bottom: 4, left: 8, right: 8}
                            empty_text: "Replacement room ID"

                            draw_bg +: {
                                border_radius: 8.0
                                border_size: 1.0
                                color: (COLOR_TELEGRAM_PANEL)
                                color_hover: (COLOR_TELEGRAM_PANEL)
                                color_focus: (COLOR_TELEGRAM_PANEL)
                                color_down: (COLOR_TELEGRAM_PANEL)
                                color_empty: (COLOR_TELEGRAM_PANEL)
                                border_color: (COLOR_TELEGRAM_BORDER)
                                border_color_hover: (COLOR_TELEGRAM_BLUE)
                                border_color_focus: (COLOR_TELEGRAM_BLUE)
                                border_color_down: (COLOR_TELEGRAM_BLUE)
                                border_color_empty: (COLOR_TELEGRAM_BORDER)
                            }
                            draw_text +: {
                                color: (COLOR_TELEGRAM_TEXT)
                                color_hover: (COLOR_TELEGRAM_TEXT)
                                color_focus: (COLOR_TELEGRAM_TEXT)
                                color_down: (COLOR_TELEGRAM_TEXT)
                                color_empty: (COLOR_TELEGRAM_DIM)
                                color_empty_hover: (COLOR_TELEGRAM_DIM)
                                color_empty_focus: (COLOR_TELEGRAM_DIM)
                                text_style: theme.font_regular { font_size: 11.0 }
                            }
                            draw_cursor +: {
                                color: (COLOR_TELEGRAM_TEXT)
                            }
                        }

                        save_tombstone_live_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Replace"
                        }
                    }

                    settings_refresh_result_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Result"
                        }

                        members_result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_ADD_USER)
                            text: "Members"
                        }

                        power_result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Power"
                        }

                        failure_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRASH)
                            text: "Failure"
                        }

                        source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Source"
                        }
                    }

                    settings_mutation_preflight_controls := View {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        align: Align{y: 0.5}

                        request_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_SEND)
                            text: "Request"
                        }

                        packet_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_COPY)
                            text: "Packet"
                        }

                        contract_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Contract"
                        }

                        taxonomy_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Taxonomy"
                        }

                        result_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_CHECKMARK)
                            text: "Result"
                        }

                        error_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_INFO)
                            text: "Error"
                        }

                        retry_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_TRIANGLE_UP)
                            text: "Retry"
                        }

                        source_button := mod.widgets.TelegramRoomHeaderButton {
                            draw_icon.svg: (ICON_LINK)
                            text: "Source"
                        }
                    }

                    settings_summary := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.5 }
                        }
                        text: "Read-only room settings summary. No Matrix room state event was sent."
                    }

                    settings_power_levels := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Permissions use existing GetRoomPowerLevels; no power-level, room-state, message, or membership mutation"
                    }

                    settings_identity := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Identity preview uses loaded room-list alias/avatar/tombstone state only; no room-state fetch or mutation."
                    }

                    settings_refresh_result_detail := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Refresh result detail stays local: Result, Members, Power, Failure, and Source do not write room state."
                    }

                    settings_mutation_preflight_detail := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Room-state mutation preflight stays local: Request, Packet, Contract, Result, Error, Retry, and Source do not write m.room.* state."
                    }

                    settings_option_evidence := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Room settings partial-live: loaded name/id, cached members, power levels, confirmed Name/Topic/avatar/alias/history/join-rule/tombstone writes; power/member edits stay blocked."
                    }

                    settings_edit_controls_boundary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_DIM)
                            text_style: theme.font_regular { font_size: 10.0 }
                        }
                        text: "Editable room-state controls: name, topic, avatar, canonical alias, history, and join rule confirm live; power levels and member moderation stay blocked."
                    }
                }

                // First, display the timeline of all messages/events.
                timeline := mod.widgets.Timeline { }

                hepta_fixture_timeline := ScrollYView {
                    visible: false
                    width: Fill,
                    height: Fill,
                    flow: Down,
                    padding: Inset{top: 14.0, bottom: 18.0, left: 14.0, right: 14.0}

                    fixture_cockpit := mod.widgets.HeptaFixtureCockpit {}
                }

                // Below that, display a typing notice when other users in the room are typing.
                typing_notice := TypingNotice { }

                room_input_bar := RoomInputBar { }
            }

            // Note: here, we're within a View that has an Overlay flow,
            // so the order that we define the below views determines which one is on top.

            // The top space should be displayed as an overlay at the top of the timeline.
            top_space := mod.widgets.TopSpace { }

            // The user profile sliding pane should be displayed on top of other "static" subviews
            // (on top of all other views that are always visible).
            user_profile_sliding_pane := mod.widgets.UserProfileSlidingPane { }

            // The loading pane appears while the user is waiting for something in the room screen
            // to finish loading, e.g., when loading an older replied-to message.
            loading_pane := LoadingPane { }


            /*
             * TODO: add the action bar back in as a series of floating buttons.
             *
            message_action_bar_popup := PopupNotification {
                align: Align{x: 0.0, y: 0.0}
                content: {
                    height: Fit,
                    width: Fit,
                    show_bg: false,
                    align: Align{
                        x: 0.5,
                        y: 0.5
                    }

                    message_action_bar := MessageActionBar {}
                }
            }
            */
        }
    }
}

/// The main widget that displays a single Matrix room.
#[derive(Script, Widget)]
pub struct RoomScreen {
    #[deref]
    view: View,

    /// The name and ID of the currently-shown room, if any.
    #[rust]
    room_name_id: Option<RoomNameId>,
    /// The timeline currently displayed by this RoomScreen, if any.
    #[rust]
    timeline_kind: Option<TimelineKind>,
    /// The persistent UI-relevant states for the room that this widget is currently displaying.
    #[rust]
    tl_state: Option<TimelineUiState>,
    /// The set of pinned events in this room.
    #[rust]
    pinned_events: Vec<OwnedEventId>,
    /// Whether this room has been successfully loaded (received from the homeserver).
    #[rust]
    is_loaded: bool,
    /// Whether or not all rooms have been loaded (received from the homeserver).
    #[rust]
    all_rooms_loaded: bool,
    /// Whether the local-only Telegram message search preview strip is visible.
    #[rust]
    telegram_message_search_visible: bool,
    /// Current query for the local-only Telegram message search preview.
    #[rust]
    telegram_message_search_query: String,
    /// Local timeline item indices that match the current local-only search query.
    #[rust]
    telegram_message_search_matches: Vec<usize>,
    /// Current selected index inside `telegram_message_search_matches`.
    #[rust]
    telegram_message_search_active_match: usize,
    /// Active loaded-timeline scope used by Filter/Date/Pins local search controls.
    #[rust]
    telegram_message_search_loaded_scope: MessageSearchLoadedScope,
    /// Current local server/context controls metadata for message search.
    #[rust]
    telegram_message_search_server_context_controls_metadata: String,
    /// Current local advanced filter controls metadata for message search.
    #[rust]
    telegram_message_search_advanced_filter_controls_metadata: String,
    /// Draft sender ID used by the live Matrix server-side From filter.
    #[rust]
    telegram_message_search_sender_filter_draft: String,
    /// Current local result-action controls metadata for message search.
    #[rust]
    telegram_message_search_result_action_controls_metadata: String,
    /// Current local server preflight controls metadata for message search.
    #[rust]
    telegram_message_search_server_preflight_controls_metadata: String,
    /// Whether a live Matrix server-side message search request is currently in flight.
    #[rust]
    telegram_message_search_server_pending: bool,
    /// Last live Matrix message search query submitted from this room screen.
    #[rust]
    telegram_message_search_server_last_query: String,
    /// Last live Matrix message search filter submitted from this room screen.
    #[rust]
    telegram_message_search_server_last_filter: MessageSearchServerFilter,
    /// Next server-side search cursor returned by the last successful Matrix search.
    #[rust]
    telegram_message_search_server_next_batch: Option<String>,
    /// Room id returned by the last successful Matrix search result.
    #[rust]
    telegram_message_search_server_room_id: String,
    /// Hits returned by the last successful Matrix search result.
    #[rust]
    telegram_message_search_server_hits: Vec<MessageSearchServerHit>,
    /// Current cached server hit event id being opened through timeline pagination.
    #[rust]
    telegram_message_search_server_context_target_event_id: Option<OwnedEventId>,
    /// Last server-side search error shown in the message-search strip.
    #[rust]
    telegram_message_search_server_last_error: String,
    /// Whether the local Telegram edit-history detail strip is visible.
    #[rust]
    telegram_message_edit_history_visible: bool,
    /// Current edit-history detail summary text.
    #[rust]
    telegram_message_edit_history_summary: String,
    /// Current edit-history local diff text.
    #[rust]
    telegram_message_edit_history_diff: String,
    /// Current edit-history metadata/boundary text.
    #[rust]
    telegram_message_edit_history_metadata: String,
    /// Current edit-history loaded diff detail text.
    #[rust]
    telegram_message_edit_history_loaded_diff_detail: String,
    /// Current edit-history request/result/error/retry/source preflight detail text.
    #[rust]
    telegram_message_edit_history_preflight_detail: String,
    /// Current edit-history full-modal/full-diff boundary text.
    #[rust]
    telegram_message_edit_history_full_boundary: String,
    /// Cached target event id for the latest compact edit-history summary.
    #[rust]
    telegram_message_edit_history_loaded_event_id: String,
    /// Cached latest replacement event id label from the latest compact edit-history summary.
    #[rust]
    telegram_message_edit_history_latest_event: String,
    /// Cached replacement count from the latest compact edit-history summary.
    #[rust]
    telegram_message_edit_history_replacement_count: Option<usize>,
    /// Cached relation pages fetched by the latest edit-history read.
    #[rust]
    telegram_message_edit_history_pages_fetched: Option<usize>,
    /// Whether the latest edit-history read reached the end of the relations cursor.
    #[rust]
    telegram_message_edit_history_pagination_exhausted: bool,
    /// Cached latest replacement timestamp from the latest compact edit-history summary.
    #[rust]
    telegram_message_edit_history_latest_timestamp: Option<MilliSecondsSinceUnixEpoch>,
    /// Cached loaded original preview for the latest compact edit-history summary.
    #[rust]
    telegram_message_edit_history_loaded_original_preview: String,
    /// Cached latest replacement preview for the latest compact edit-history summary.
    #[rust]
    telegram_message_edit_history_latest_preview: String,
    /// Cached latest replacement event JSON returned by the compact m.replace read.
    #[rust]
    telegram_message_edit_history_latest_source_json: String,
    /// Cached error text from the latest failed compact edit-history read.
    #[rust]
    telegram_message_edit_history_result_error: String,
    /// Cached edit-history retry event id after a failed compact history read.
    #[rust]
    telegram_message_edit_history_retry_event_id: Option<OwnedEventId>,
    /// Cached edit-history retry timeline kind after a failed compact history read.
    #[rust]
    telegram_message_edit_history_retry_timeline_kind: Option<TimelineKind>,
    /// Whether the local Telegram message report status strip is visible.
    #[rust]
    telegram_message_report_status_visible: bool,
    /// Current report status badge text.
    #[rust]
    telegram_message_report_status_badge: String,
    /// Current report status summary text.
    #[rust]
    telegram_message_report_status_summary: String,
    /// Current report status lifecycle/boundary metadata.
    #[rust]
    telegram_message_report_status_metadata: String,
    /// Current local report request/result/error/retry/source preflight detail metadata.
    #[rust]
    telegram_message_report_preflight_detail_metadata: String,
    /// Cached report result error text for local preflight details.
    #[rust]
    telegram_message_report_result_error: String,
    /// Last confirmed report event id that can be retried after a failed result.
    #[rust]
    telegram_message_report_retry_event_id: Option<OwnedEventId>,
    /// Last confirmed report reason that can be retried after a failed result.
    #[rust]
    telegram_message_report_retry_reason: String,
    /// Whether the local Matrix link preview result strip is visible.
    #[rust]
    telegram_matrix_link_preview_visible: bool,
    /// Current Matrix link preview status badge text.
    #[rust]
    telegram_matrix_link_preview_status: String,
    /// Current Matrix link preview summary text.
    #[rust]
    telegram_matrix_link_preview_summary: String,
    /// Current Matrix link preview metadata/boundary text.
    #[rust]
    telegram_matrix_link_preview_metadata: String,
    /// Current Matrix link unresolved-detail text.
    #[rust]
    telegram_matrix_link_unresolved_detail: String,
    /// Cached Matrix link target label for the local unresolved-detail row.
    #[rust]
    telegram_matrix_link_preview_target_label: String,
    /// Cached Matrix link room id or alias target for read-only Server context refresh.
    #[rust]
    telegram_matrix_link_preview_room_or_alias_id: Option<OwnedRoomOrAliasId>,
    /// Cached Matrix link via count for the local unresolved-detail row.
    #[rust]
    telegram_matrix_link_preview_via_count: usize,
    /// Cached Matrix link via server list for local route-scope clipboard controls.
    #[rust]
    telegram_matrix_link_preview_via_label: String,
    /// Cached Matrix link requested event id label for the local unresolved-detail row.
    #[rust]
    telegram_matrix_link_preview_event_id_label: String,
    /// Cached Matrix link preview error character count for the local unresolved-detail row.
    #[rust]
    telegram_matrix_link_preview_error_chars: Option<usize>,
    /// Cached event source room id fetched during a compact Matrix link event preview.
    #[rust]
    telegram_matrix_link_preview_source_room_id: Option<OwnedRoomId>,
    /// Cached event source event id fetched during a compact Matrix link event preview.
    #[rust]
    telegram_matrix_link_preview_source_event_id: Option<OwnedEventId>,
    /// Cached event source JSON fetched during a compact Matrix link event preview.
    #[rust]
    telegram_matrix_link_preview_source_json: String,
    /// Current Matrix link local route-scope controls metadata.
    #[rust]
    telegram_matrix_link_route_scope_metadata: String,
    /// Current Matrix link local context-actions row metadata.
    #[rust]
    telegram_matrix_link_context_actions_metadata: String,
    /// Current Matrix link server/context boundary text.
    #[rust]
    telegram_matrix_link_server_context_boundary: String,
    /// Cached Matrix link preview retry target after a failed compact preview read.
    #[rust]
    telegram_matrix_link_preview_retry_room_or_alias_id: Option<OwnedRoomOrAliasId>,
    /// Cached Matrix link preview retry via server list after a failed compact preview read.
    #[rust]
    telegram_matrix_link_preview_retry_via: Vec<OwnedServerName>,
    /// Cached Matrix link preview retry event id after a failed compact preview read.
    #[rust]
    telegram_matrix_link_preview_retry_event_id: Option<OwnedEventId>,
    /// Cached Matrix link preview retry timeline kind after a failed compact preview read.
    #[rust]
    telegram_matrix_link_preview_retry_timeline_kind: Option<TimelineKind>,
    /// Cached Matrix link room id or alias join currently waiting for a result action.
    #[rust]
    telegram_matrix_link_join_pending_room_or_alias_id: Option<OwnedRoomOrAliasId>,
    /// Cached Matrix link room id or alias join target after a failed result action.
    #[rust]
    telegram_matrix_link_join_retry_room_or_alias_id: Option<OwnedRoomOrAliasId>,
    /// Cached Matrix link via server list for the current join-capable target.
    #[rust]
    telegram_matrix_link_join_via_servers: Vec<OwnedServerName>,
    /// Cached Matrix link via server list after a failed join result.
    #[rust]
    telegram_matrix_link_join_retry_via_servers: Vec<OwnedServerName>,
    /// Cached Matrix link room id or alias knock currently waiting for a result action.
    #[rust]
    telegram_matrix_link_knock_pending_room_or_alias_id: Option<OwnedRoomOrAliasId>,
    /// Cached Matrix link room id or alias knock target after a failed result action.
    #[rust]
    telegram_matrix_link_knock_retry_room_or_alias_id: Option<OwnedRoomOrAliasId>,
    /// Cached Matrix link via server list for the current knock-capable target.
    #[rust]
    telegram_matrix_link_knock_via_servers: Vec<OwnedServerName>,
    /// Cached Matrix link via server list after a failed knock result.
    #[rust]
    telegram_matrix_link_knock_retry_via_servers: Vec<OwnedServerName>,
    /// Cached Matrix link invite currently waiting for a result action.
    #[rust]
    telegram_matrix_link_invite_pending_room_id: Option<OwnedRoomId>,
    /// Cached Matrix link invite user currently waiting for a result action.
    #[rust]
    telegram_matrix_link_invite_pending_user_id: Option<OwnedUserId>,
    /// Cached Matrix link invite room after a failed result action.
    #[rust]
    telegram_matrix_link_invite_retry_room_id: Option<OwnedRoomId>,
    /// Cached Matrix link invite user after a failed result action.
    #[rust]
    telegram_matrix_link_invite_retry_user_id: Option<OwnedUserId>,
    /// Whether the Telegram-style room action strip is visible.
    #[rust]
    telegram_room_actions_visible: bool,
    /// Current room management state used by the Telegram room actions strip.
    #[rust]
    telegram_room_action_details: Option<RoomContextMenuDetails>,
    /// Whether the Telegram-style notifications mode strip is visible.
    #[rust]
    telegram_notifications_visible: bool,
    /// Current notifications mode read/write status.
    #[rust]
    telegram_notifications_local_status: String,
    /// Current local result-detail action staged from the notifications strip.
    #[rust]
    telegram_notifications_result_detail_action: String,
    /// Current local timed/global/pusher preflight action staged from the notifications strip.
    #[rust]
    telegram_notifications_preflight_detail_action: String,
    /// Last notification-mode room id that can be retried after a failed write.
    #[rust]
    telegram_notifications_retry_room_id: Option<OwnedRoomId>,
    /// Last notification mode that can be retried after a failed write.
    #[rust]
    telegram_notifications_retry_mode: Option<RoomNotificationMode>,
    /// Last default notification-mode timeline that can be retried after a failed write.
    #[rust]
    telegram_notifications_retry_default_timeline_kind: Option<TimelineKind>,
    /// Last default notification mode that can be retried after a failed write.
    #[rust]
    telegram_notifications_retry_default_mode: Option<RoomNotificationMode>,
    /// Draft keyword used by the notification keyword Add/Remove controls.
    #[rust]
    telegram_notifications_keyword_draft: String,
    /// Last failed notification keyword mutation keyword that can be retried.
    #[rust]
    telegram_notifications_retry_keyword: String,
    /// Last failed notification keyword mutation operation that can be retried.
    #[rust]
    telegram_notifications_retry_keyword_mutation: Option<NotificationKeywordMutation>,
    /// Whether the Telegram-style read-only room info strip is visible.
    #[rust]
    telegram_room_info_visible: bool,
    /// Whether the Telegram-style read-only room settings strip is visible.
    #[rust]
    telegram_room_settings_visible: bool,
    /// Current read-only room settings preview status.
    #[rust]
    telegram_room_settings_local_status: String,
    /// Last local refresh/result detail control selected in room settings.
    #[rust]
    telegram_room_settings_refresh_detail_action: String,
    /// Last local room-state mutation preflight control selected in room settings.
    #[rust]
    telegram_room_settings_mutation_preflight_action: String,
    /// Draft value for the live room name write field.
    #[rust]
    telegram_room_settings_name_draft: String,
    /// Draft value for the live room topic write field.
    #[rust]
    telegram_room_settings_topic_draft: String,
    /// Draft value for the live canonical alias write field.
    #[rust]
    telegram_room_settings_alias_draft: String,
    /// Draft replacement room id for the live tombstone write field.
    #[rust]
    telegram_room_settings_tombstone_replacement_draft: String,
    /// Last failed room settings mutation field that can be retried.
    #[rust]
    telegram_room_settings_retry_field: Option<RoomSettingsMutationField>,
    /// Last failed room settings mutation value that can be retried.
    #[rust]
    telegram_room_settings_retry_value: String,
    /// Last failed room avatar upload path that can be retried.
    #[rust]
    telegram_room_settings_retry_avatar_file_path: Option<PathBuf>,
    /// Last failed room avatar upload MIME type that can be retried.
    #[rust]
    telegram_room_settings_retry_avatar_mime_type: Option<mime::Mime>,
    /// Last failed room canonical alias write alt aliases that can be retried.
    #[rust]
    telegram_room_settings_retry_canonical_alias_alt_aliases: Vec<OwnedRoomAliasId>,
}

impl Drop for RoomScreen {
    fn drop(&mut self) {
        // This ensures that the `TimelineUiState` instance owned by this room is *always* returned
        // back to to `TIMELINE_STATES`, which ensures that its UI state(s) are not lost
        // and that other RoomScreen instances can show this room in the future.
        // RoomScreen will be dropped whenever its widget instance is destroyed, e.g.,
        // when a Tab is closed or the app is resized to a different AdaptiveView layout.
        self.hide_timeline();
    }
}

impl ScriptHook for RoomScreen {
    fn on_after_reload(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            if let Some(tl_state) = &mut self.tl_state.as_mut() {
                // Clear the timeline's drawn items caches and redraw it.
                tl_state.content_drawn_since_last_update.clear();
                tl_state.profile_drawn_since_last_update.clear();
                self.view.redraw(cx);
            }
        });
    }
}

impl Widget for RoomScreen {
    // Handle events and actions for the RoomScreen widget and its inner Timeline view.
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let room_screen_widget_uid = self.widget_uid();
        let portal_list = self.portal_list(cx, ids!(timeline.list));
        let user_profile_sliding_pane =
            self.user_profile_sliding_pane(cx, ids!(user_profile_sliding_pane));
        let loading_pane = self.loading_pane(cx, ids!(loading_pane));

        // Handle actions here before processing timeline updates.
        // Normally (in most other widgets), the order of event handling doesn't matter much.
        // However, since actions may refer to a specific timeline item's index,
        // we want to handle those before processing any updates that might change
        // the set of timeline indices (which would invalidate the index values in any actions).
        if let Event::Actions(actions) = event {
            for action in actions {
                if let Some(SearchMessagesAction::LocalPreviewOpened) = action.downcast_ref() {
                    if self.room_name_id.is_none() && self.tl_state.is_none() {
                        continue;
                    }
                    let room_label = self
                        .room_name_id
                        .as_ref()
                        .map_or_else(|| "this chat".to_string(), ToString::to_string);
                    self.set_telegram_room_actions_visible(cx, false, None);
                    self.set_telegram_search_mode_visible(cx, true);
                    enqueue_popup_notification(
                        format!(
                            "Messages sidebar opened local search for {room_label}. {SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_LABEL} No Matrix search query was sent."
                        ),
                        PopupKind::Info,
                        Some(4.0),
                    );
                    return;
                }
            }

            self.handle_telegram_room_header_actions(
                cx,
                actions,
                &portal_list,
                &user_profile_sliding_pane,
            );

            for (index, wr) in portal_list.items_with_actions(actions) {
                // Handle a hover-in action on the reaction list: show a reaction summary.
                let reaction_list = wr.reaction_list(cx, ids!(reaction_list));
                if let RoomScreenTooltipActions::HoverInReactionButton {
                    widget_rect,
                    reaction_data,
                } = reaction_list.hovered_in(actions)
                {
                    let Some(_tl_state) = self.tl_state.as_ref() else {
                        continue;
                    };
                    let tooltip_text_arr: Vec<String> = reaction_data
                        .reaction_senders
                        .iter()
                        .map(|(sender, _react_info)| {
                            user_profile_cache::get_user_display_name_for_room(
                                cx,
                                sender.clone(),
                                Some(&reaction_data.room_id),
                                true,
                            )
                            .into_option()
                            .unwrap_or_else(|| sender.to_string())
                        })
                        .collect();

                    let mut tooltip_text = utils::human_readable_list(
                        &tooltip_text_arr,
                        MAX_VISIBLE_AVATARS_IN_READ_RECEIPT,
                    );
                    tooltip_text.push_str(&format!(" reacted with: {}", reaction_data.reaction));
                    cx.widget_action(
                        room_screen_widget_uid,
                        TooltipAction::HoverIn {
                            text: tooltip_text,
                            widget_rect,
                            options: CalloutTooltipOptions {
                                position: TooltipPosition::Bottom,
                                ..Default::default()
                            },
                        },
                    );
                }

                // Handle a hover-out action on the reaction list or avatar row.
                let avatar_row_ref = wr.avatar_row(cx, ids!(avatar_row));
                if reaction_list.hovered_out(actions) || avatar_row_ref.hover_out(actions) {
                    cx.widget_action(room_screen_widget_uid, TooltipAction::HoverOut);
                }

                // Handle a hover-in action on the avatar row: show a read receipts summary.
                if let RoomScreenTooltipActions::HoverInReadReceipt {
                    widget_rect,
                    read_receipts,
                } = avatar_row_ref.hover_in(actions)
                {
                    let Some(room_id) = self.room_id() else {
                        return;
                    };
                    let tooltip_text =
                        room_read_receipt::populate_tooltip(cx, read_receipts, room_id);
                    cx.widget_action(
                        room_screen_widget_uid,
                        TooltipAction::HoverIn {
                            text: tooltip_text,
                            widget_rect,
                            options: CalloutTooltipOptions {
                                position: TooltipPosition::Left,
                                ..Default::default()
                            },
                        },
                    );
                }

                // Handle an image within the message being clicked.
                let content_message = wr.text_or_image(cx, ids!(content.message));
                if let TextOrImageAction::Clicked(mxc_uri) = actions
                    .find_widget_action(content_message.widget_uid())
                    .cast()
                {
                    let texture = content_message.get_texture(cx);
                    self.handle_image_click(cx, mxc_uri, texture, index);
                    continue;
                }

                let edited_indicator = wr.edited_indicator(cx, ids!(profile.edited_indicator));
                if let EditedIndicatorAction::ShowEditHistory {
                    event_id,
                    loaded_target_metadata,
                } = actions
                    .find_widget_action(edited_indicator.widget_uid())
                    .cast()
                {
                    if let Some(tl) = self.tl_state.as_ref() {
                        let timeline_kind = tl.kind.clone();
                        self.telegram_message_edit_history_retry_event_id = Some(event_id.clone());
                        self.telegram_message_edit_history_retry_timeline_kind =
                            Some(timeline_kind.clone());
                        submit_async_request(MatrixRequest::FetchEditHistory {
                            timeline_kind,
                            event_id: event_id.clone(),
                        });
                        let metadata_note = loaded_target_metadata
                            .as_ref()
                            .map(loaded_edit_history_target_metadata_label)
                            .unwrap_or_else(|| {
                                "Loaded edit target metadata unavailable.".to_string()
                            });
                        self.show_telegram_message_edit_history_loading(
                            cx,
                            &event_id,
                            &metadata_note,
                        );
                        enqueue_popup_notification(
                            format!(
                                "Edit history lookup started for {event_id}. {metadata_note} {MESSAGE_EDIT_HISTORY_COMPACT_LABEL}"
                            ),
                            PopupKind::Info,
                            Some(3.0),
                        );
                    }
                }

                // Handle the invite_user_button (in a SmallStateEvent) being clicked.
                if wr.button(cx, ids!(invite_user_button)).clicked(actions) {
                    let Some(tl) = self.tl_state.as_ref() else {
                        continue;
                    };
                    if let Some(event_tl_item) =
                        tl.items.get(index).and_then(|item| item.as_event())
                    {
                        let user_id = event_tl_item.sender().to_owned();
                        let username = if let TimelineDetails::Ready(profile) =
                            event_tl_item.sender_profile()
                        {
                            profile.display_name.as_deref().unwrap_or(user_id.as_str())
                        } else {
                            user_id.as_str()
                        };
                        let room_id = tl.kind.room_id().clone();
                        let username_for_cancel = username.to_string();
                        let username_for_status = username.to_string();
                        let content = ConfirmationModalContent {
                            title_text: "Send Invitation".into(),
                            body_text: format!(
                                "Invite {username} to this room? {TIMELINE_INVITE_CONFIRMATION_COMPACT_LABEL}"
                            )
                            .into(),
                            accept_button_text: Some("Invite".into()),
                            cancel_button_text: Some("Cancel".into()),
                            on_accept_clicked: Some(Box::new(move |_cx| {
                                submit_async_request(MatrixRequest::InviteUser { room_id, user_id });
                            })),
                            on_cancel_clicked: Some(Box::new(move |_cx| {
                                enqueue_popup_notification(
                                    format!(
                                        "Timeline invite canceled for {username_for_cancel}. {TIMELINE_INVITE_CONFIRMATION_COMPACT_LABEL}"
                                    ),
                                    PopupKind::Info,
                                    Some(3.0),
                                );
                            })),
                        };
                        enqueue_popup_notification(
                            format!(
                                "Timeline invite confirmation opened for {username_for_status}. {TIMELINE_INVITE_CONFIRMATION_COMPACT_LABEL}"
                            ),
                            PopupKind::Info,
                            Some(3.0),
                        );
                        cx.action(InviteAction::ShowInviteConfirmationModal(RefCell::new(
                            Some(content),
                        )));
                    }
                }

                if wr
                    .button(cx, ids!(hepta_actions.inspect_button))
                    .clicked(actions)
                {
                    self.handle_hepta_inspect_event(cx, index);
                }
                if wr
                    .button(cx, ids!(hepta_actions.approve_button))
                    .clicked(actions)
                {
                    self.handle_hepta_approval_decision(cx, index, true);
                }
                if wr
                    .button(cx, ids!(hepta_actions.reject_button))
                    .clicked(actions)
                {
                    self.handle_hepta_approval_decision(cx, index, false);
                }
            }

            self.handle_message_actions(cx, actions, &portal_list, &loading_pane);

            for action in actions {
                // Handle actions related to restoring the previously-saved state of rooms.
                if let Some(AppStateAction::RoomLoadedSuccessfully { room_name_id, .. }) =
                    action.downcast_ref()
                {
                    if self
                        .room_name_id
                        .as_ref()
                        .is_some_and(|rn| rn.room_id() == room_name_id.room_id())
                    {
                        // `set_displayed_room()` does nothing if the room_name_id is unchanged, so we clear it first.
                        self.room_name_id = None;
                        let thread_root_event_id = self
                            .timeline_kind
                            .as_ref()
                            .and_then(|k| k.thread_root_event_id().cloned());
                        self.set_displayed_room(cx, room_name_id, thread_root_event_id);
                        return;
                    }
                }

                if let Some(matrix_link_join_action) = action.downcast_ref::<MatrixLinkJoinAction>()
                {
                    match matrix_link_join_action {
                        MatrixLinkJoinAction::Submitted {
                            room_or_alias_id,
                            target,
                            via_count,
                            event_id_label,
                        } => {
                            self.show_telegram_matrix_link_join_submitted(
                                cx,
                                room_or_alias_id.clone(),
                                target.clone(),
                                *via_count,
                                event_id_label.clone(),
                            );
                        }
                        MatrixLinkJoinAction::Canceled { target } => {
                            self.show_telegram_matrix_link_join_canceled(cx, target.clone());
                        }
                    }
                    continue;
                }

                if let Some(matrix_link_knock_action) =
                    action.downcast_ref::<MatrixLinkKnockAction>()
                {
                    match matrix_link_knock_action {
                        MatrixLinkKnockAction::Submitted {
                            room_or_alias_id,
                            target,
                            via_count,
                            event_id_label,
                        } => {
                            self.show_telegram_matrix_link_knock_submitted(
                                cx,
                                room_or_alias_id.clone(),
                                target.clone(),
                                *via_count,
                                event_id_label.clone(),
                            );
                        }
                        MatrixLinkKnockAction::Canceled { target } => {
                            self.show_telegram_matrix_link_knock_canceled(cx, target.clone());
                        }
                    }
                    continue;
                }

                if let Some(matrix_link_invite_action) =
                    action.downcast_ref::<MatrixLinkInviteAction>()
                {
                    match matrix_link_invite_action {
                        MatrixLinkInviteAction::Submitted {
                            room_id,
                            user_id,
                            target,
                            via_count,
                        } => {
                            self.show_telegram_matrix_link_invite_submitted(
                                cx,
                                room_id.clone(),
                                user_id.clone(),
                                target.clone(),
                                *via_count,
                            );
                        }
                        MatrixLinkInviteAction::Canceled { target } => {
                            self.show_telegram_matrix_link_invite_canceled(cx, target.clone());
                        }
                    }
                    continue;
                }

                if let Some(MatrixLinkJoinResultAction::Joined {
                    room_or_alias_id,
                    server_names,
                    room_id,
                }) = action.downcast_ref()
                {
                    let should_handle = self
                        .telegram_matrix_link_join_pending_room_or_alias_id
                        .as_ref()
                        .is_some_and(|pending| pending == room_or_alias_id)
                        || self
                            .telegram_matrix_link_join_retry_room_or_alias_id
                            .as_ref()
                            .is_some_and(|retry| retry == room_or_alias_id);
                    if should_handle {
                        self.show_telegram_matrix_link_join_result(
                            cx,
                            room_or_alias_id.clone(),
                            server_names.clone(),
                            Some(room_id.clone()),
                            None,
                        );
                        continue;
                    }
                }
                if let Some(MatrixLinkJoinResultAction::Failed {
                    room_or_alias_id,
                    server_names,
                    error,
                }) = action.downcast_ref()
                {
                    let should_handle = self
                        .telegram_matrix_link_join_pending_room_or_alias_id
                        .as_ref()
                        .is_some_and(|pending| pending == room_or_alias_id)
                        || self
                            .telegram_matrix_link_join_retry_room_or_alias_id
                            .as_ref()
                            .is_some_and(|retry| retry == room_or_alias_id);
                    if should_handle {
                        self.show_telegram_matrix_link_join_result(
                            cx,
                            room_or_alias_id.clone(),
                            server_names.clone(),
                            None,
                            Some(error.to_string()),
                        );
                        continue;
                    }
                }
                if let Some(KnockResultAction::Knocked {
                    room_or_alias_id,
                    room,
                }) = action.downcast_ref()
                {
                    let retry_matches = self
                        .telegram_matrix_link_knock_retry_room_or_alias_id
                        .as_ref()
                        .is_some_and(|retry| retry == room_or_alias_id);
                    let should_handle = retry_matches
                        || self
                            .telegram_matrix_link_knock_pending_room_or_alias_id
                            .as_ref()
                            .is_some_and(|pending| pending == room_or_alias_id);
                    if should_handle {
                        let server_names = if retry_matches {
                            self.telegram_matrix_link_knock_retry_via_servers.clone()
                        } else {
                            self.telegram_matrix_link_knock_via_servers.clone()
                        };
                        self.show_telegram_matrix_link_knock_result(
                            cx,
                            room_or_alias_id.clone(),
                            server_names,
                            Some(room.room_id().to_owned()),
                            None,
                        );
                        continue;
                    }
                }
                if let Some(KnockResultAction::Failed {
                    room_or_alias_id,
                    error,
                }) = action.downcast_ref()
                {
                    let retry_matches = self
                        .telegram_matrix_link_knock_retry_room_or_alias_id
                        .as_ref()
                        .is_some_and(|retry| retry == room_or_alias_id);
                    let should_handle = retry_matches
                        || self
                            .telegram_matrix_link_knock_pending_room_or_alias_id
                            .as_ref()
                            .is_some_and(|pending| pending == room_or_alias_id);
                    if should_handle {
                        let server_names = if retry_matches {
                            self.telegram_matrix_link_knock_retry_via_servers.clone()
                        } else {
                            self.telegram_matrix_link_knock_via_servers.clone()
                        };
                        self.show_telegram_matrix_link_knock_result(
                            cx,
                            room_or_alias_id.clone(),
                            server_names,
                            None,
                            Some(error.to_string()),
                        );
                        continue;
                    }
                }

                if let Some(InviteResultAction::Sent { room_id, user_id }) = action.downcast_ref() {
                    let retry_matches = self
                        .telegram_matrix_link_invite_retry_room_id
                        .as_ref()
                        .zip(self.telegram_matrix_link_invite_retry_user_id.as_ref())
                        .is_some_and(|(retry_room, retry_user)| {
                            retry_room == room_id && retry_user == user_id
                        });
                    let should_handle = retry_matches
                        || self
                            .telegram_matrix_link_invite_pending_room_id
                            .as_ref()
                            .zip(self.telegram_matrix_link_invite_pending_user_id.as_ref())
                            .is_some_and(|(pending_room, pending_user)| {
                                pending_room == room_id && pending_user == user_id
                            });
                    if should_handle {
                        self.show_telegram_matrix_link_invite_result(
                            cx,
                            room_id.clone(),
                            user_id.clone(),
                            None,
                        );
                        continue;
                    }
                }
                if let Some(InviteResultAction::Failed {
                    room_id,
                    user_id,
                    error,
                }) = action.downcast_ref()
                {
                    let retry_matches = self
                        .telegram_matrix_link_invite_retry_room_id
                        .as_ref()
                        .zip(self.telegram_matrix_link_invite_retry_user_id.as_ref())
                        .is_some_and(|(retry_room, retry_user)| {
                            retry_room == room_id && retry_user == user_id
                        });
                    let should_handle = retry_matches
                        || self
                            .telegram_matrix_link_invite_pending_room_id
                            .as_ref()
                            .zip(self.telegram_matrix_link_invite_pending_user_id.as_ref())
                            .is_some_and(|(pending_room, pending_user)| {
                                pending_room == room_id && pending_user == user_id
                            });
                    if should_handle {
                        self.show_telegram_matrix_link_invite_result(
                            cx,
                            room_id.clone(),
                            user_id.clone(),
                            Some(error.to_string()),
                        );
                        continue;
                    }
                }

                // Handle InviteResultAction to show popup notifications.
                if let Some(InviteResultAction::Sent { room_id, .. }) = action.downcast_ref() {
                    // Only handle if this is for the current room.
                    if self
                        .room_name_id
                        .as_ref()
                        .is_some_and(|rn| rn.room_id() == room_id)
                    {
                        enqueue_popup_notification(
                            "Sent invite successfully.",
                            PopupKind::Success,
                            Some(4.0),
                        );
                    }
                }
                if let Some(InviteResultAction::Failed { room_id, error, .. }) =
                    action.downcast_ref()
                {
                    // Only handle if this is for the current room.
                    if self
                        .room_name_id
                        .as_ref()
                        .is_some_and(|rn| rn.room_id() == room_id)
                    {
                        enqueue_popup_notification(
                            format!("Failed to send invite.\n\nError: {error}"),
                            PopupKind::Error,
                            None,
                        );
                    }
                }

                // When transitioning from offline to online, clear stale `Requested`/`Failed`
                // entries from per-room caches so they can be re-fetched.
                if let Some(RoomsListHeaderAction::StateUpdate(new_state)) = action.downcast_ref() {
                    if !matches!(new_state, State::Offline) {
                        if let Some(tl) = self.tl_state.as_mut() {
                            tl.media_cache.clear_all_pending_and_failed_requests();
                            tl.link_preview_cache
                                .clear_all_pending_and_failed_requests();
                        }
                    }
                    continue;
                }

                // Handle the highlight animation for a message.
                let Some(tl) = self.tl_state.as_mut() else {
                    continue;
                };
                if let MessageHighlightAnimationState::Pending { item_id } =
                    tl.message_highlight_animation_state
                {
                    if portal_list.smooth_scroll_reached(actions) {
                        cx.widget_action(
                            room_screen_widget_uid,
                            MessageAction::HighlightMessage(item_id),
                        );
                        tl.message_highlight_animation_state = MessageHighlightAnimationState::Off;
                        // Adjust the scrolled-to item's position to be slightly beneath the top of the viewport.
                        // portal_list.set_first_id_and_scroll(portal_list.first_id(), 15.0);
                    }
                }
            }

            /*
            // close message action bar if scrolled.
            if portal_list.scrolled(actions) {
                let message_action_bar_popup = self.popup_notification(cx, ids!(message_action_bar_popup));
                message_action_bar_popup.close(cx);
            }
            */

            // Set visibility of loading message banner based of pagination logic
            self.send_pagination_request_based_on_scroll_pos(cx, actions, &portal_list);
            // Handle sending any read receipts for the current logged-in user.
            self.send_user_read_receipts_based_on_scroll_pos(cx, actions, &portal_list);

            // Handle the jump to bottom button: update its visibility, and handle clicks.
            self.jump_to_bottom_button(cx, ids!(jump_to_bottom_button))
                .update_from_actions(cx, &portal_list, actions);
        }

        // Currently, a Signal event is only used to tell this widget:
        // 1. to check if the room has been loaded from the homeserver yet, or
        // 2. that its timeline events have been updated in the background.
        if let Event::Signal = event {
            if self.room_name_id.is_some() {
                self.refresh_telegram_room_action_details(cx);
            }

            if let (false, Some(room_name_id), true) = (
                self.is_loaded,
                self.room_name_id.as_ref(),
                cx.has_global::<RoomsListRef>(),
            ) {
                let rooms_list_ref = cx.get_global::<RoomsListRef>();
                if rooms_list_ref.is_room_loaded(room_name_id.room_id()) {
                    let room_name_clone = room_name_id.clone();
                    let thread_root_event_id = self
                        .timeline_kind
                        .as_ref()
                        .and_then(|k| k.thread_root_event_id().cloned());
                    // This room has been loaded now, so we call `set_displayed_room()`.
                    // We first clear the `room_name_id`, otherwise that function will do nothing.
                    self.room_name_id = None;
                    self.set_displayed_room(cx, &room_name_clone, thread_root_event_id);
                } else {
                    self.all_rooms_loaded = rooms_list_ref.all_rooms_loaded();
                    return;
                }
            }

            // If this RoomScreen is waiting to show a thread timeline (not the main room timeline),
            // then we need to retry showing the timeline now (upon a Signal),
            // because the thread timeline may have been successfully created.
            if self.tl_state.is_none() && self.timeline_kind.is_some() {
                self.show_timeline(cx);
            }

            self.process_timeline_updates(cx, &portal_list);
            if self.telegram_message_search_visible {
                self.refresh_telegram_message_search_matches(cx);
            }

            // Ideally we would do this elsewhere on the main thread, because it's not room-specific,
            // but it doesn't hurt to do it here.
            // TODO: move this up a layer to something higher in the UI tree,
            //       and wrap it in a `if let Event::Signal` conditional.
            user_profile_cache::process_user_profile_updates(cx);
            avatar_cache::process_avatar_updates(cx);
        }

        // We only forward "interactive hit" events to the inner timeline view
        // if none of the various overlay views are visible.
        // We always forward "non-interactive hit" events to the inner timeline view.
        // We check which overlay views are visible in the order of those views' z-ordering,
        // such that the top-most views get a chance to handle the event first.
        //
        let is_interactive_hit = utils::is_interactive_hit_event(event);
        let is_pane_shown: bool;
        if loading_pane.is_currently_shown(cx) {
            is_pane_shown = true;
            if is_interactive_hit {
                loading_pane.handle_event(cx, event, scope);
            }
        } else if user_profile_sliding_pane.is_currently_shown(cx) {
            is_pane_shown = true;
            if is_interactive_hit {
                user_profile_sliding_pane.handle_event(cx, event, scope);
            }
        } else {
            is_pane_shown = false;
        }

        // TODO: once we use the `hits()` API, should be able to remove the above conditionals
        //       about whether the loading pane or user profile pane are shown, because
        //       Makepad already delivers most events to all views regardless of visibility,
        //       so the only thing we'd need here is the conditional below.

        if !is_pane_shown || !is_interactive_hit {
            // Create a Scope with RoomScreenProps containing the room members.
            // This scope is needed by child widgets like MentionableTextInput during event handling.
            let room_props = if let Some(tl) = self.tl_state.as_ref() {
                let room_id = tl.kind.room_id().clone();
                let room_members = tl.room_members.clone();

                // Fetch room data once to avoid duplicate expensive lookups
                let (room_display_name, room_avatar_url) = get_client()
                    .and_then(|client| client.get_room(&room_id))
                    .map(|room| {
                        (
                            room.cached_display_name().unwrap_or(RoomDisplayName::Empty),
                            room.avatar_url(),
                        )
                    })
                    .unwrap_or((RoomDisplayName::Empty, None));

                RoomScreenProps {
                    room_screen_widget_uid,
                    room_name_id: RoomNameId::new(room_display_name, room_id),
                    timeline_kind: tl.kind.clone(),
                    room_members,
                    room_avatar_url,
                }
            } else if let Some(room_name) = &self.room_name_id {
                // Fallback case: we have a room_name but no tl_state yet
                RoomScreenProps {
                    room_screen_widget_uid,
                    room_name_id: room_name.clone(),
                    timeline_kind: self
                        .timeline_kind
                        .clone()
                        .expect("BUG: room_name_id was set but timeline_kind was missing"),
                    room_members: None,
                    room_avatar_url: None,
                }
            } else {
                // No room selected yet, skip event handling that requires room context
                if !is_pane_shown || !is_interactive_hit {
                    return;
                }
                log!(
                    "RoomScreen handling event with no room_name_id and no tl_state, skipping room-dependent event handling"
                );
                // Use a dummy room props for non-room-specific events
                let room_id = owned_room_id!("!dummy:matrix.org");
                RoomScreenProps {
                    room_screen_widget_uid,
                    room_name_id: RoomNameId::empty(room_id.clone()),
                    timeline_kind: TimelineKind::MainRoom { room_id },
                    room_members: None,
                    room_avatar_url: None,
                }
            };
            let mut room_scope = Scope::with_props(&room_props);

            // Forward the event to the inner timeline view, but capture any actions it produces
            // such that we can handle the ones relevant to only THIS RoomScreen widget right here and now,
            // ensuring they are not mistakenly handled by other RoomScreen widget instances.
            let mut actions_generated_within_this_room_screen =
                cx.capture_actions(|cx| self.view.handle_event(cx, event, &mut room_scope));
            // Here, we handle and remove any general actions that are relevant to only this RoomScreen.
            // Removing the handled actions ensures they are not mistakenly handled by other RoomScreen widget instances.
            actions_generated_within_this_room_screen.retain(|action| {
                if self.handle_link_clicked(cx, action, &user_profile_sliding_pane, &portal_list) {
                    return false;
                }

                // Handle the action that requests to show the user profile sliding pane.
                if let ShowUserProfileAction::ShowUserProfile(profile_and_room_id) =
                    action.as_widget_action().cast()
                {
                    self.show_user_profile(
                        cx,
                        &user_profile_sliding_pane,
                        UserProfilePaneInfo {
                            profile_and_room_id,
                            room_name: self
                                .room_name_id
                                .as_ref()
                                .map_or_else(|| UNNAMED_ROOM.to_string(), |r| r.to_string()),
                            room_member: None,
                        },
                    );
                }

                /*
                match action.as_widget_action().widget_uid_eq(room_screen_widget_uid).cast() {
                    MessageAction::ActionBarClose => {
                        let message_action_bar_popup = self.popup_notification(cx, ids!(message_action_bar_popup));
                        let message_action_bar = message_action_bar_popup.message_action_bar(cx, ids!(message_action_bar));

                        // close only if the active message is requesting it to avoid double closes.
                        if let Some(message_widget_uid) = message_action_bar.message_widget_uid() {
                            if action.as_widget_action().widget_uid_eq(message_widget_uid).is_some() {
                                message_action_bar_popup.close(cx);
                            }
                        }
                    }
                    MessageAction::ActionBarOpen { item_id, message_rect } => {
                        let message_action_bar_popup = self.popup_notification(cx, ids!(message_action_bar_popup));
                        let message_action_bar = message_action_bar_popup.message_action_bar(cx, ids!(message_action_bar));

                        let margin_x = 50.;

                        let coords = dvec2(
                            (message_rect.pos.x + message_rect.size.x) - margin_x,
                            message_rect.pos.y,
                        );

                        script_apply_eval!(cx, message_action_bar_popup, {
                            content +: { margin +: { left: #(coords.x), top: #(coords.y) } }
                        });

                        if let Some(message_widget_uid) = action.as_widget_action().map(|a| a.widget_uid) {
                            message_action_bar_popup.open(cx);
                            message_action_bar.initialize_with_data(cx, widget_uid, message_widget_uid, item_id);
                        }
                    }
                    _ => {}
                }
                */

                // Keep all unhandled actions so we can add them back to the global action list below.
                true
            });
            // Add back any unhandled actions to the global action list.
            cx.extend_actions(actions_generated_within_this_room_screen);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.is_current_hepta_fixture_workspace() {
            self.apply_hepta_fixture_timeline_visibility(cx, true);
            return self.view.draw_walk(cx, scope, walk);
        }

        // If the room isn't loaded yet, we show the restore status label only.
        if !self.is_loaded {
            let Some(room_name) = &self.room_name_id else {
                // No room selected yet, nothing to show.
                return DrawStep::done();
            };
            let mut restore_status_view =
                self.view.restore_status_view(cx, ids!(restore_status_view));
            restore_status_view.set_content(cx, self.all_rooms_loaded, room_name);
            return restore_status_view.draw(cx, scope);
        }
        if self.tl_state.is_none() {
            // Tl_state may not be ready after dock loading.
            // If return DrawStep::done() inside self.view.draw_walk, turtle will misalign and panic.
            return DrawStep::done();
        }

        let room_screen_widget_uid = self.widget_uid();
        while let Some(subview) = self.view.draw_walk(cx, scope, walk).step() {
            // Here, we only need to handle drawing the portal list.
            let portal_list_ref = subview.as_portal_list();
            let Some(mut list_ref) = portal_list_ref.borrow_mut() else {
                error!(
                    "!!! RoomScreen::draw_walk(): BUG: expected a PortalList widget, but got something else"
                );
                continue;
            };
            let Some(tl_state) = self.tl_state.as_mut() else {
                return DrawStep::done();
            };

            // Set the portal list's range based on the number of timeline items.
            let tl_items = &tl_state.items;
            let last_item_id = tl_items.len();

            let list = list_ref.deref_mut();
            list.set_item_range(cx, 0, last_item_id);

            while let Some(item_id) = list.next_visible_item(cx) {
                let item = {
                    let tl_idx = item_id;
                    let Some(timeline_item) = tl_items.get(tl_idx) else {
                        // This shouldn't happen (unless the timeline gets corrupted or some other weird error),
                        // but we can always safely fill the item with an empty widget that takes up no space.
                        list.item(cx, item_id, id!(Empty));
                        continue;
                    };

                    // Determine whether this item's content and profile have been drawn since the last update.
                    // Pass this state to each of the `populate_*` functions so they can attempt to re-use
                    // an item in the timeline's portallist that was previously populated, if one exists.
                    let item_drawn_status = ItemDrawnStatus {
                        content_drawn: tl_state.content_drawn_since_last_update.contains(&tl_idx),
                        profile_drawn: tl_state.profile_drawn_since_last_update.contains(&tl_idx),
                    };
                    let (item, item_new_draw_status) = match timeline_item.kind() {
                        TimelineItemKind::Event(event_tl_item) => match event_tl_item.content() {
                            TimelineItemContent::MsgLike(msg_like_content) => {
                                if tl_state.kind.thread_root_event_id().is_none()
                                    && msg_like_content.thread_root.is_some()
                                {
                                    // Hide threaded replies from the main room timeline UI.
                                    (
                                        list.item(cx, item_id, id!(Empty)),
                                        ItemDrawnStatus::both_drawn(),
                                    )
                                } else {
                                    match &msg_like_content.kind {
                                        MsgLikeKind::Message(_)
                                        | MsgLikeKind::Sticker(_)
                                        | MsgLikeKind::Redacted
                                        | MsgLikeKind::Poll(_) => {
                                            let prev_event =
                                                tl_idx.checked_sub(1).and_then(|i| tl_items.get(i));
                                            populate_message_view(
                                                cx,
                                                list,
                                                item_id,
                                                &tl_state.kind,
                                                event_tl_item,
                                                msg_like_content,
                                                prev_event,
                                                &mut tl_state.media_cache,
                                                &mut tl_state.link_preview_cache,
                                                &tl_state.fetched_thread_summaries,
                                                &mut tl_state.pending_thread_summary_fetches,
                                                &tl_state.user_power,
                                                &self.pinned_events,
                                                item_drawn_status,
                                                room_screen_widget_uid,
                                            )
                                        }
                                        MsgLikeKind::UnableToDecrypt(utd) => {
                                            populate_small_state_event(
                                                cx,
                                                list,
                                                item_id,
                                                &tl_state.kind,
                                                event_tl_item,
                                                utd,
                                                item_drawn_status,
                                            )
                                        }
                                        MsgLikeKind::LiveLocation(live_loc) => {
                                            populate_small_state_event(
                                                cx,
                                                list,
                                                item_id,
                                                &tl_state.kind,
                                                event_tl_item,
                                                live_loc,
                                                item_drawn_status,
                                            )
                                        }
                                        MsgLikeKind::Other(other)
                                            if is_hepta_event_type(
                                                &other.event_type().to_string(),
                                            ) =>
                                        {
                                            populate_hepta_event_card(
                                                cx,
                                                list,
                                                item_id,
                                                event_tl_item,
                                                other,
                                                item_drawn_status,
                                            )
                                        }
                                        MsgLikeKind::Other(other) => populate_small_state_event(
                                            cx,
                                            list,
                                            item_id,
                                            &tl_state.kind,
                                            event_tl_item,
                                            other,
                                            item_drawn_status,
                                        ),
                                    }
                                }
                            }
                            TimelineItemContent::MembershipChange(membership_change) => {
                                populate_small_state_event(
                                    cx,
                                    list,
                                    item_id,
                                    &tl_state.kind,
                                    event_tl_item,
                                    membership_change,
                                    item_drawn_status,
                                )
                            }
                            TimelineItemContent::ProfileChange(profile_change) => {
                                populate_small_state_event(
                                    cx,
                                    list,
                                    item_id,
                                    &tl_state.kind,
                                    event_tl_item,
                                    profile_change,
                                    item_drawn_status,
                                )
                            }
                            TimelineItemContent::OtherState(other) => populate_small_state_event(
                                cx,
                                list,
                                item_id,
                                &tl_state.kind,
                                event_tl_item,
                                other,
                                item_drawn_status,
                            ),
                            unhandled => {
                                let item = list.item(cx, item_id, id!(SmallStateEvent));
                                item.label(cx, ids!(content))
                                    .set_text(cx, &format!("[Unsupported] {:?}", unhandled));
                                (item, ItemDrawnStatus::both_drawn())
                            }
                        },
                        TimelineItemKind::Virtual(VirtualTimelineItem::DateDivider(millis)) => {
                            let item = list.item(cx, item_id, id!(DateDivider));
                            let text = unix_time_millis_to_datetime(*millis)
                                // format the time as a shortened date (Sat, Sept 5, 2021)
                                .map(|dt| format!("{}", dt.date_naive().format("%a %b %-d, %Y")))
                                .unwrap_or_else(|| format!("{:?}", millis));
                            item.label(cx, ids!(date)).set_text(cx, &text);
                            (item, ItemDrawnStatus::both_drawn())
                        }
                        TimelineItemKind::Virtual(VirtualTimelineItem::ReadMarker) => {
                            let item = list.item(cx, item_id, id!(ReadMarker));
                            (item, ItemDrawnStatus::both_drawn())
                        }
                        TimelineItemKind::Virtual(VirtualTimelineItem::TimelineStart) => {
                            let item = list.item(cx, item_id, id!(Empty));
                            (item, ItemDrawnStatus::both_drawn())
                        }
                    };

                    // Now that we've drawn the item, add its index to the set of drawn items.
                    if item_new_draw_status.content_drawn {
                        tl_state
                            .content_drawn_since_last_update
                            .insert(tl_idx..tl_idx + 1);
                    }
                    if item_new_draw_status.profile_drawn {
                        tl_state
                            .profile_drawn_since_last_update
                            .insert(tl_idx..tl_idx + 1);
                    }
                    item
                };
                item.draw_all(cx, scope);
            }

            // If the list is not filling the viewport, we need to back paginate the timeline
            // until we have enough events items to fill the viewport.
            if !tl_state.fully_paginated && !list.is_filling_viewport() {
                log!(
                    "Automatically paginating timeline to fill viewport for room {:?}",
                    self.room_name_id
                );
                submit_async_request(MatrixRequest::PaginateTimeline {
                    timeline_kind: tl_state.kind.clone(),
                    num_events: 50,
                    direction: PaginationDirection::Backwards,
                });
            }
        }
        DrawStep::done()
    }
}

include!("room_screen/impl_part_01a.rs");
include!("room_screen/impl_part_01b.rs");
include!("room_screen/impl_part_02a.rs");
include!("room_screen/impl_part_02b.rs");
include!("room_screen/after_impl_part_01a.rs");
include!("room_screen/after_impl_part_01b.rs");
include!("room_screen/after_impl_part_02.rs");
include!("room_screen/after_impl_part_03a.rs");
include!("room_screen/after_impl_part_03b.rs");
