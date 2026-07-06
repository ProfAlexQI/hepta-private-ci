//! The `RoomsList` widget displays a filterable list of rooms
//! that can be clicked on to open the room timeline (`RoomScreen`).
//!
//! It is responsible for receiving room-related updates from the background task
//! that runs the room list service.
//! It also receives space-related updates from the background task that runs
//! the space sync service(s).
//!
//! Generally, it maintains several key states:
//! * The set of all joined rooms, which is displayed separately as "direct" rooms
//!   and non-direct regular rooms.
//! * The set of invited rooms, which have not yet been joined.
//! * The map of spaces and their child rooms and nested subspaces.
//!
//! This widget is a global singleton and is thus accessible via `Cx::get_global()`,
//! so you can use it from other widgets or functions on the main UI thread
//! that need to query basic info about a particular room or space.

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque, hash_map::Entry},
    rc::Rc,
    sync::Arc,
};
use crossbeam_queue::SegQueue;
use makepad_widgets::*;
use matrix_sdk_ui::spaces::room_list::SpaceRoomListPaginationState;
use ruma::events::tag::TagName;
use tokio::sync::mpsc::UnboundedSender;
use matrix_sdk::{
    RoomState,
    ruma::{
        events::tag::Tags, MilliSecondsSinceUnixEpoch, OwnedRoomAliasId, OwnedRoomId, OwnedUserId,
    },
};
use crate::{
    app::{AppState, AppStateAction, SelectedRoom},
    home::{
        navigation_tab_bar::{NavigationBarAction, SelectedTab},
        room_context_menu::RoomContextMenuDetails,
        rooms_list_entry::RoomsListEntryAction,
        space_lobby::{SpaceLobbyAction, SpaceLobbyEntryWidgetExt},
    },
    logout::logout_confirm_modal::LogoutAction,
    room::{
        FetchedRoomAvatar,
        room_display_filter::{
            RoomDisplayFilter, RoomDisplayFilterBuilder, RoomFilterCriteria,
            SPACE_UNREAD_MENTION_FILTER_LOCAL_ZERO_EVIDENCE, SortFn,
        },
    },
    shared::{
        collapsible_header::{
            CollapsibleHeaderAction, CollapsibleHeaderWidgetRefExt, HeaderCategory,
        },
        jump_to_bottom_button::UnreadMessageCount,
        popup_list::{PopupKind, enqueue_popup_notification},
        room_filter_input_bar::MainFilterAction,
    },
    sliding_sync::{
        MatrixLinkAction, MatrixRequest, PaginationDirection, TimelineKind, submit_async_request,
    },
    space_service_sync::{ParentChain, SpaceRequest, SpaceRoomListAction},
    utils::{RoomNameId, VecDiff},
};

/// Whether to pre-paginate visible rooms at least once in order to
/// be able to display the latest message in a room's RoomsListEntry,
/// and to have something to immediately show when a user first opens a room.
const PREPAGINATE_VISIBLE_ROOMS: bool = true;

#[allow(dead_code)]
const ROOMS_LIST_SECTION_UNREAD_AGGREGATE_LOCAL_ZERO_EVIDENCE: &str = "People/Rooms section unread/mention aggregate packet keeps header badges on local zero placeholders until a running aggregate is maintained; it may summarize loaded row unread state locally, but header rendering sends no aggregate scan, read receipt, message, room-state, or membership request.";
#[allow(dead_code)]
const ROOMS_LIST_LOAD_MORE_PAGINATION_PACKET_EVIDENCE: &str = "Room-list Load More pagination packet records current RoomListService loaded counts, server max hint, selected-space child pagination status, and missing explicit load-more cursor/result/retry slots locally. Existing service-driven SpaceService pagination and visible-row latest-message prefetch stay read-only; no user-triggered room-list pagination, message, room-state, or membership request is emitted.";
const ROOMS_LIST_SPACE_PARENT_CACHE_LOCAL_EVIDENCE: &str = "Selected-space filtering uses cached SpaceService child/subspace maps recursively; room rows do not store every parent chain yet, and filter rendering sends no Matrix search, message, room-state, or membership request.";
const ROOMS_LIST_NAME_UPDATE_SELECTED_STATE_LOCAL_EVIDENCE: &str = "Room-name updates refresh loaded list rows and filters locally; Dock tabs and StackNav headers still need SelectedRoom broadcast plumbing, and the evidence sends no Matrix room-state, message, or membership request.";
const ROOMS_LIST_REMOVED_ROOM_SELECTED_STATE_LOCAL_EVIDENCE: &str = "Selected-room removed/rejoin packet waits for RoomListService RemoveRoom, then records removed room id, membership state, active selection match, focus clear, replacement UI slot, rejoin request slot, stale-event policy, and selected-space scope locally. Until a backend contract provides an exact replacement/rejoin request and result taxonomy, it sends no JoinRoom, LeaveRoom, Knock, message, room-state, or membership request.";
#[allow(dead_code)]
const ROOMS_LIST_UNSUPPORTED_HEADER_TOGGLE_EVIDENCE: &str = "RoomsList ignores unsupported CollapsibleHeader categories that are not rendered in the current Telegram shell instead of panicking; only Invites, People, and Rooms toggle local expansion state, and no Matrix search, room-list pagination, message, room-state, membership, or live mutation request is sent.";

thread_local! {
    /// The list of all invited rooms, which is only tracked here
    /// because the backend doesn't need to track any info about them.
    ///
    /// This must only be accessed by the main UI thread.
    static ALL_INVITED_ROOMS: Rc<RefCell<HashMap<OwnedRoomId, InvitedRoomInfo>>> = Rc::new(RefCell::new(HashMap::new()));
}

fn toggle_supported_header_category(
    category: HeaderCategory,
    invites_expanded: &mut bool,
    direct_expanded: &mut bool,
    regular_expanded: &mut bool,
) -> bool {
    match category {
        HeaderCategory::Invites => {
            *invites_expanded = !*invites_expanded;
            true
        }
        HeaderCategory::DirectRooms => {
            *direct_expanded = !*direct_expanded;
            true
        }
        HeaderCategory::RegularRooms => {
            *regular_expanded = !*regular_expanded;
            true
        }
        HeaderCategory::Favorites
        | HeaderCategory::LowPriority
        | HeaderCategory::LeftRooms
        | HeaderCategory::None => false,
    }
}

fn removed_room_was_active_selection(
    current_active_room: Option<&SelectedRoom>,
    room_id: &OwnedRoomId,
) -> bool {
    current_active_room.is_some_and(|selected_room| selected_room.room_id() == room_id)
}

fn selected_room_removed_rejoin_packet_label(
    room_id: &OwnedRoomId,
    new_state: &RoomState,
    was_active: bool,
    selected_space: Option<&RoomNameId>,
) -> String {
    let active_state = if was_active {
        "active_selection matched; FocusNone emitted"
    } else {
        "active_selection not_matched; focus unchanged"
    };
    let selected_space_state = selected_space
        .map(|space| space.room_id().to_string())
        .unwrap_or_else(|| "none".to_string());
    format!(
        "Selected-room removed/rejoin packet: removed_room {room_id}; membership_state {new_state:?}; {active_state}; selected_space {selected_space_state}; replacement_ui_slot not_wired; rejoin_request_slot not_built; stale_event_policy do_not_resurrect_removed_room_without_new_selection; no JoinRoom, LeaveRoom, Knock, message, room-state, or membership request was sent."
    )
}

fn room_list_load_more_pagination_packet_label(
    loaded_joined_rooms: usize,
    displayed_invited_rooms: usize,
    displayed_direct_rooms: usize,
    displayed_regular_rooms: usize,
    max_known_rooms: Option<u32>,
    selected_space_status: &str,
) -> String {
    let max_hint = max_known_rooms
        .map(|count| format!("server_max_hint {count}"))
        .unwrap_or_else(|| "server_max_hint unknown".to_string());
    format!(
        "Room-list Load More pagination packet: adapter_source RoomListService entries_with_dynamic_adapters(usize::MAX); loaded_joined_rooms {loaded_joined_rooms}; displayed_invited {displayed_invited_rooms}; displayed_people {displayed_direct_rooms}; displayed_rooms {displayed_regular_rooms}; {max_hint}; {selected_space_status}; load_more_button_slot not_rendered; explicit_cursor_slot not_exposed; inflight_slot not_tracked_by_rooms_list; error_slot SpaceRoomListAction_PaginationError_popup_only; retry_slot not_built; exhausted_policy unknown_until_room_list_contract; latest_preview_pagination_source Matrix_PaginateTimeline_read_only; no user-triggered room-list pagination, message, room-state, or membership request was sent."
    )
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SectionUnreadAggregateSnapshot {
    room_count: usize,
    unread_messages: u64,
    unread_mentions: u64,
    marked_unread_count: usize,
}

impl SectionUnreadAggregateSnapshot {
    fn from_displayed_rooms(
        displayed_rooms: &[OwnedRoomId],
        all_joined_rooms: &HashMap<OwnedRoomId, JoinedRoomInfo>,
    ) -> Self {
        displayed_rooms
            .iter()
            .filter_map(|room_id| all_joined_rooms.get(room_id))
            .fold(Self::default(), |mut snapshot, room| {
                snapshot.room_count += 1;
                snapshot.unread_messages += room.num_unread_messages;
                snapshot.unread_mentions += room.num_unread_mentions;
                if room.is_marked_unread {
                    snapshot.marked_unread_count += 1;
                }
                snapshot
            })
    }
}

fn section_unread_aggregate_packet_label(
    direct: SectionUnreadAggregateSnapshot,
    regular: SectionUnreadAggregateSnapshot,
    selected_space: Option<&RoomNameId>,
    filter_active: bool,
) -> String {
    let selected_space_state = selected_space
        .map(|space| space.room_id().to_string())
        .unwrap_or_else(|| "none".to_string());
    format!(
        "People/Rooms unread/mention aggregate packet: people_loaded_rooms {} unread {} mentions {} manual_unread {}; rooms_loaded_rooms {} unread {} mentions {} manual_unread {}; selected_space {}; filter_active {}; header_badge_source local_zero_placeholder; aggregate_refresh_slot not_built; muted_low_priority_policy not_defined; parent_chain_attribution partial_cache_only; no aggregate scan, read receipt, message, room-state, or membership request was sent.",
        direct.room_count,
        direct.unread_messages,
        direct.unread_mentions,
        direct.marked_unread_count,
        regular.room_count,
        regular.unread_messages,
        regular.unread_mentions,
        regular.marked_unread_count,
        selected_space_state,
        filter_active,
    )
}

/// Returns a reference to the list of all invited rooms.
///
/// This function requires passing in a reference to `Cx`,
/// which isn't used, but acts as a guarantee that this function
/// must only be called by the main UI thread.
pub fn get_invited_rooms(_cx: &mut Cx) -> Rc<RefCell<HashMap<OwnedRoomId, InvitedRoomInfo>>> {
    ALL_INVITED_ROOMS.with(Rc::clone)
}

/// Clears all invited rooms known to the global rooms list.
///
/// This function requires passing in a reference to `Cx`,
/// which isn't used, but acts as a guarantee that this function
/// must only be called by the main UI thread.
pub fn clear_all_invited_rooms(_cx: &mut Cx) {
    ALL_INVITED_ROOMS.with(|rooms| {
        rooms.borrow_mut().clear();
    });
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.RoomsListStatusLabel = View {
        width: Fill, height: Fit,
        flow: Down,
        align: Align{ x: 0.5, y: 0.5 }
        padding: 15.0,
        spacing: 6.0,

        status_row := View {
            width: Fill,
            height: Fit,
            flow: Right,
            align: Align{ x: 0.5, y: 0.5 }
            spacing: 10.0,

            loading_spinner := LoadingSpinner {
                visible: false,
                width: 20,
                height: 20,
                draw_bg +: {
                    color: (COLOR_ACTIVE_PRIMARY)
                    border_size: 3.0
                }
            }

            label := Label {
                padding: 0
                width: Fit,
                flow: Flow.Right{wrap: true},
                align: Align{ x: 0.5, y: 0.5 }
                draw_text +: {
                    color: (COLOR_TELEGRAM_TEXT),
                    text_style: REGULAR_TEXT {}
                }
                text: "Loading workspaces..."
            }
        }

        evidence_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "Dialog list uses loaded RoomListService state and local filters; no Matrix search or mutation."
        }

        membership_edge_evidence_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "Membership edge: SlidingSync removes Banned rooms and skips Knocked/Left rooms locally; re-knock/cancel-prior-knock UI remains unwired, and no JoinRoom, LeaveRoom, Knock, message, room-state, or membership request is sent from list rendering."
        }

        room_list_pagination_evidence_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "Room-list adapter loads the current RoomListService result locally; there is no Load more rooms UI, while visible room rows only prefetch latest-message previews through the existing PaginateTimeline read path."
        }

        room_list_load_more_packet_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "Room-list Load More pagination packet keeps explicit cursor/result/retry slots local until the adapter exposes a user-triggered pagination contract."
        }

        space_unread_filter_evidence_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "Space unread/mention aggregate packet keeps JoinedSpaceInfo filters on the local-zero source; no aggregate unread fetch, read receipt, message, room-state, or membership request is sent."
        }

        section_unread_aggregate_evidence_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "People/Rooms unread/mention aggregate packet keeps header badges local-zero until aggregate refresh exists; no aggregate scan, read receipt, message, room-state, or membership request is sent."
        }

        space_parent_cache_evidence_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "Selected-space filtering uses cached SpaceService parent/child maps; row filtering sends no Matrix search, message, room-state, or membership request."
        }

        room_name_update_evidence_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "Room name updates refresh loaded rows locally; selected tabs/header rename broadcast remains unwired and sends no Matrix room-state, message, or membership request."
        }

        removed_room_selected_state_evidence_label := Label {
            width: Fill,
            height: Fit,
            flow: Flow.Right{wrap: true}
            align: Align{ x: 0.5, y: 0.5 }
            draw_text +: {
                color: (COLOR_TELEGRAM_MUTED),
                text_style: REGULAR_TEXT { font_size: 9.0 }
            }
            text: "Selected-room removed/rejoin packet waits for RoomListService RemoveRoom and records local replacement/rejoin slots without JoinRoom, LeaveRoom, Knock, message, room-state, or membership request."
        }
    }

    mod.widgets.RoomsList = #(RoomsList::register_widget(vm)) {
        width: Fill, height: Fill
        flow: Down
        cursor: MouseCursor.Default,
        show_bg: true
        draw_bg.color: (COLOR_TELEGRAM_PANEL)

        space_lobby_entry := SpaceLobbyEntry {}

        list := PortalList {
            keep_invisible: false,
            auto_tail: false,
            width: Fill, height: Fill
            flow: Down,
            padding: Inset{top: 4, bottom: 8}
            spacing: 0.0

            collapsible_header := CollapsibleHeader {}
            rooms_list_entry := RoomsListEntry {}
            empty := View {}
            status_label := mod.widgets.RoomsListStatusLabel {}
            bottom_filler := View {
                width: Fill,
                height: 100.0,
            }
        }
    }
}

/// The possible updates that should be displayed by the single list of all rooms.
///
/// These updates are enqueued by the `enqueue_rooms_list_update` function
/// (which is called from background async tasks that receive updates from the matrix server),
/// and then dequeued by the `RoomsList` widget's `handle_event` function.
pub enum RoomsListUpdate {
    /// No rooms have been loaded yet.
    NotLoaded,
    /// Some rooms were loaded, and the server optionally told us
    /// the max number of rooms that will ever be loaded.
    LoadedRooms { max_rooms: Option<u32> },
    /// Add a new room to the list of rooms the user has been invited to.
    /// This will be maintained and displayed separately from joined rooms.
    AddInvitedRoom(InvitedRoomInfo),
    /// Add a new room to the list of all rooms that the user has joined.
    AddJoinedRoom(JoinedRoomInfo),
    /// Clear all rooms in the list of all rooms.
    ClearRooms,
    /// Update the latest event content and timestamp for the given room.
    UpdateLatestEvent {
        room_id: OwnedRoomId,
        timestamp: MilliSecondsSinceUnixEpoch,
        /// The Html-formatted text preview of the latest message.
        latest_message_text: String,
    },
    /// Update the number of unread messages and mentions for the given room.
    UpdateNumUnreadMessages {
        room_id: OwnedRoomId,
        is_marked_unread: bool,
        unread_messages: UnreadMessageCount,
        unread_mentions: u64,
    },
    /// Update the displayable name for the given room.
    UpdateRoomName { new_room_name: RoomNameId },
    /// Update the avatar (image) for the given room.
    UpdateRoomAvatar {
        room_id: OwnedRoomId,
        room_avatar: FetchedRoomAvatar,
    },
    /// Update whether the given room is a direct room.
    UpdateIsDirect {
        room_id: OwnedRoomId,
        is_direct: bool,
    },
    /// Remove the given room from the rooms list
    RemoveRoom {
        room_id: OwnedRoomId,
        /// The new state of the room (which caused its removal).
        new_state: RoomState,
    },
    /// Update the tags for the given room.
    Tags {
        room_id: OwnedRoomId,
        new_tags: Tags,
    },
    /// Update the status label at the bottom of the list of all rooms.
    Status { status: String },
    /// Mark the given room as tombstoned.
    TombstonedRoom { room_id: OwnedRoomId },
    /// Hide the given room from being displayed.
    ///
    /// This is useful for temporarily preventing a room from being shown,
    /// e.g., after a room has been left but before the homeserver has registered
    /// that we left it and removed it via the RoomListService.
    HideRoom { room_id: OwnedRoomId },
    /// Scroll to the given room.
    ScrollToRoom(OwnedRoomId),
    /// The background space service is now listening for requests,
    /// and the sender-side channel endpoint is included.
    SpaceRequestSender(UnboundedSender<SpaceRequest>),
    /// Update the ordering of rooms based on the given diff.
    RoomOrderUpdate(VecDiff<OwnedRoomId>),
}

static PENDING_ROOM_UPDATES: SegQueue<RoomsListUpdate> = SegQueue::new();

/// Enqueue a new room update for the list of all rooms
/// and signals the UI that a new update is available to be handled.
pub fn enqueue_rooms_list_update(update: RoomsListUpdate) {
    PENDING_ROOM_UPDATES.push(update);
    SignalToUI::set_ui_signal();
}

/// Actions related to a single room in the RoomsList widget.
#[derive(Debug, Clone, Default)]
pub enum RoomsListAction {
    /// A new room or space was selected.
    Selected(SelectedRoom),
    /// A new room was joined from an accepted invite,
    /// meaning that the existing `InviteScreen` should be converted
    /// to a `RoomScreen` to display the now-joined room.
    InviteAccepted { room_name_id: RoomNameId },
    /// Instructs the top-level app to show the context menu for the given room.
    ///
    /// Emitted by the RoomsList when the user right-clicks or long-presses
    /// on a room in the rooms list.
    OpenRoomContextMenu {
        details: RoomContextMenuDetails,
        pos: DVec2,
    },
    #[default]
    None,
}

impl ActionDefaultRef for RoomsListAction {
    fn default_ref() -> &'static Self {
        static DEFAULT: RoomsListAction = RoomsListAction::None;
        &DEFAULT
    }
}

/// UI-related info about a joined room.
///
/// This includes info needed display a preview of that room in the RoomsList
/// and to filter the list of rooms based on the current search filter.
#[derive(Debug)]
pub struct JoinedRoomInfo {
    /// The displayable name of this room (includes room ID for fallback).
    pub room_name_id: RoomNameId,
    /// The number of unread messages in this room.
    pub num_unread_messages: u64,
    /// The number of unread mentions in this room.
    pub num_unread_mentions: u64,
    /// Whether the room is manually marked as unread.
    pub is_marked_unread: bool,
    /// The canonical alias for this room, if any.
    pub canonical_alias: Option<OwnedRoomAliasId>,
    /// The alternative aliases for this room, if any.
    pub alt_aliases: Vec<OwnedRoomAliasId>,
    /// The tags associated with this room, if any.
    /// This includes things like is_favourite, is_low_priority,
    /// whether the room is a server notice room, etc.
    pub tags: Tags,
    /// The timestamp and Html text content of the latest message in this room.
    pub latest: Option<(MilliSecondsSinceUnixEpoch, String)>,
    /// The avatar for this room: either an array of bytes holding the avatar image
    /// or a string holding the first Unicode character of the room name.
    pub room_avatar: FetchedRoomAvatar,
    /// Whether this room has been paginated at least once.
    /// We pre-paginate visible rooms at least once in order to
    /// be able to display the latest message in the RoomsListEntry
    /// and to have something to immediately show when a user first opens a room.
    pub has_been_paginated: bool,
    /// Whether this room is currently selected in the UI.
    pub is_selected: bool,
    /// Whether this a direct room.
    pub is_direct: bool,
    /// Whether this room is tombstoned (shut down and replaced with a successor room).
    pub is_tombstoned: bool,
    // TODO: we could store the parent chain(s) of this room, i.e., which spaces
    //       they are children of. One room can be in multiple spaces.
    // Today selected-space filtering uses the cached SpaceService map instead
    // of a per-row parent-chain store.
}

/// UI-related info about a room that the user has been invited to.
///
/// This includes info needed display a preview of that room in the RoomsList
/// and to filter the list of rooms based on the current search filter.
pub struct InvitedRoomInfo {
    /// The displayable name of this room (includes room ID for fallback).
    pub room_name_id: RoomNameId,
    /// The canonical alias for this room, if any.
    pub canonical_alias: Option<OwnedRoomAliasId>,
    /// The alternative aliases for this room, if any.
    pub alt_aliases: Vec<OwnedRoomAliasId>,
    /// The avatar for this room: either an array of bytes holding the avatar image
    /// or a string holding the first Unicode character of the room name.
    pub room_avatar: FetchedRoomAvatar,
    /// Info about the user who invited us to this room, if available.
    pub inviter_info: Option<InviterInfo>,
    /// The timestamp and Html text content of the latest message in this room.
    pub latest: Option<(MilliSecondsSinceUnixEpoch, String)>,
    /// The state of how this invite is being handled by the client backend
    /// and what should be shown in the UI.
    ///
    /// We maintain this state here instead of in the `InviteScreen`
    /// because we need the state to persist even if the `InviteScreen` is closed.
    pub invite_state: InviteState,
    /// Whether this room is currently selected in the UI.
    pub is_selected: bool,
    /// Whether this is an invite to a direct room.
    pub is_direct: bool,
}

fn room_context_menu_details_from_joined_room(jr: &JoinedRoomInfo) -> RoomContextMenuDetails {
    RoomContextMenuDetails {
        room_name_id: jr.room_name_id.clone(),
        is_favorite: jr.tags.contains_key(&TagName::Favorite),
        is_low_priority: jr.tags.contains_key(&TagName::LowPriority),
        is_marked_unread: jr.is_marked_unread,
        num_unread_messages: jr.num_unread_messages,
        num_unread_mentions: jr.num_unread_mentions,
        canonical_alias: jr.canonical_alias.clone(),
        alt_aliases: jr.alt_aliases.clone(),
        alt_alias_count: jr.alt_aliases.len(),
        room_avatar_loaded: matches!(&jr.room_avatar, FetchedRoomAvatar::Image(_)),
        is_tombstoned: jr.is_tombstoned,
    }
}

/// Info about the user who invited us to a room.
#[derive(Clone)]
pub struct InviterInfo {
    pub user_id: OwnedUserId,
    pub display_name: Option<String>,
    pub avatar: Option<Arc<[u8]>>,
}
impl std::fmt::Debug for InviterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InviterInfo")
            .field("user_id", &self.user_id)
            .field("display_name", &self.display_name)
            .field("avatar?", &self.avatar.is_some())
            .finish()
    }
}

/// The state of a pending invite.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InviteState {
    /// Waiting for the user to accept or decline the invite.
    #[default]
    WaitingOnUserInput,
    /// Waiting for the server to respond to the user's "join room" action.
    WaitingForJoinResult,
    /// Waiting for the server to respond to the user's "leave room" action.
    WaitingForLeaveResult,
    /// The invite was accepted and the room was successfully joined.
    /// We're now waiting for our client to receive the joined room from the homeserver.
    WaitingForJoinedRoom,
    /// The invite was declined and the room was successfully left.
    /// This should result in the InviteScreen being closed.
    RoomLeft,
}

/// The value in the RoomsList's `space_map` that contains info about a space.
#[derive(Default)]
struct SpaceMapValue {
    /// Whether this space is fully paginated, meaning that our client has obtained
    /// the full list of direct children within this space.
    ///
    /// Note that it *does not* mean that all nested/subspaces within this space
    /// have been fully paginated themselves.
    is_fully_paginated: bool,
    /// The set of rooms that are direct children of this space, excluding subspaces.
    direct_child_rooms: Arc<HashSet<OwnedRoomId>>,
    /// The nested subspaces (only spaces) that are direct children of this space.
    direct_subspaces: Arc<HashSet<OwnedRoomId>>,
    /// The chain of parents that this space has, ordered from highest to lowest level.
    ///
    /// That is, the first element is this space's top-level ancestor space,
    /// while the last element is this space's immediate parent.
    parent_chain: ParentChain,
}

#[derive(Script, Widget)]
pub struct RoomsList {
    #[deref]
    view: View,

    /// The list of all rooms that the user has been invited to.
    ///
    /// This is a shared reference to the thread-local [`ALL_INVITED_ROOMS`] variable.
    #[rust]
    invited_rooms: Rc<RefCell<HashMap<OwnedRoomId, InvitedRoomInfo>>>,

    /// The set of all joined rooms and their cached info.
    /// This includes both direct rooms and regular rooms, but not invited rooms.
    #[rust]
    all_joined_rooms: HashMap<OwnedRoomId, JoinedRoomInfo>,

    /// The list of all room IDs in display order, matching the order from the room list service.
    #[rust]
    all_known_rooms_order: VecDeque<OwnedRoomId>,

    /// The space that is currently selected as a display filter for the rooms list, if any.
    /// * If `None` (default), no space is selected, and all rooms can be shown.
    /// * If `Some`, the rooms list is in "space" mode. A special "Space Lobby" entry
    ///   is shown at the top, and only child rooms within this space will be displayed.
    #[rust]
    selected_space: Option<RoomNameId>,

    /// The sender used to send Space-related requests to the background service.
    #[rust]
    space_request_sender: Option<UnboundedSender<SpaceRequest>>,

    /// A flattened map of all spaces known to the client.
    ///
    /// The key is a Space ID, and the value contains a list of all regular rooms
    /// and nested subspaces *directly* within that space.
    ///
    /// This can include both joined and non-joined spaces.
    #[rust]
    space_map: HashMap<OwnedRoomId, SpaceMapValue>,

    /// Rooms that are explicitly hidden and should never be shown in the rooms list.
    #[rust]
    hidden_rooms: HashSet<OwnedRoomId>,

    /// The currently-active filter function for the list of rooms.
    ///
    /// ## Important Notes
    /// 1. Do not use this directly. Instead, use the `should_display_room!()` macro.
    /// 2. This does *not* get auto-applied when it changes, for performance reasons.
    #[rust]
    display_filter: RoomDisplayFilter,

    /// The currently-active sort function for the list of rooms.
    #[rust]
    sort_fn: Option<Box<SortFn>>,

    /// The list of invited rooms currently displayed in the UI.
    #[rust]
    displayed_invited_rooms: Vec<OwnedRoomId>,
    #[rust(false)]
    is_invited_rooms_header_expanded: bool,
    #[rust]
    invited_rooms_indexes: RoomCategoryIndexes,

    /// The list of direct rooms currently displayed in the UI.
    #[rust]
    displayed_direct_rooms: Vec<OwnedRoomId>,
    #[rust(false)]
    is_direct_rooms_header_expanded: bool,
    #[rust]
    direct_rooms_indexes: RoomCategoryIndexes,

    /// The list of regular (non-direct) joined rooms currently displayed in the UI.
    ///
    /// **Direct rooms are excluded** from this; they are in `displayed_direct_rooms`.
    #[rust]
    displayed_regular_rooms: Vec<OwnedRoomId>,
    #[rust(true)]
    is_regular_rooms_header_expanded: bool,
    #[rust]
    regular_rooms_indexes: RoomCategoryIndexes,

    /// The latest status message that should be displayed in the bottom status label.
    #[rust]
    status: String,

    /// The currently-selected room.
    #[rust]
    current_active_room: Option<SelectedRoom>,

    /// The maximum number of rooms that will ever be loaded.
    ///
    /// This should not be used to determine whether all requested rooms have been loaded,
    /// because we will likely never receive this many rooms due to the room list service
    /// excluding rooms that we have filtered out (e.g., left or tombstoned rooms, spaces, etc).
    #[rust]
    max_known_rooms: Option<u32>,
    /// The latest local removed-room packet shown in the status evidence row.
    #[rust]
    last_removed_room_rejoin_packet: Option<String>,
    // /// Whether the room list service has loaded all requested rooms from the homeserver.
    // #[rust] all_rooms_loaded: bool,
}

impl ScriptHook for RoomsList {
    fn on_after_new(&mut self, _vm: &mut ScriptVm) {
        self.invited_rooms = ALL_INVITED_ROOMS.with(Rc::clone);
    }
}

/// A macro that returns whether a given Room should be displayed in the RoomsList.
/// This is only intended for usage within RoomsList methods.
///
/// ## Arguments
/// 1. `self: &RoomsList`: an immutable reference to the `RoomsList` widget struct.
/// 2. `room_id: &OwnedRoomId`: an immutable reference to the room's ID.
/// 3. `room: &dyn impl FilterableRoom`: an immutable reference to the room info,
///     which must implement the [`FilterableRoom`] trait.
macro_rules! should_display_room {
    ($self:expr, $room_id:expr, $room:expr) => {
        !$self.hidden_rooms.contains($room_id)
            && ($self.display_filter)($room)
            && $self
                .selected_space
                .as_ref()
                .is_none_or(|space| $self.is_room_indirectly_in_space(space.room_id(), $room_id))
    };
}

impl RoomsList {
    /// Returns whether the homeserver has finished syncing all of the rooms
    /// that should be synced to our client based on the currently-specified room list filter.
    pub fn all_rooms_loaded(&self) -> bool {
        // TODO: fix this: figure out a way to determine if
        //       all requested rooms have been received from the homeserver.
        // Until then this stays a local unknown/false restore signal and emits
        // no room-list pagination, Matrix search, message, room-state, or
        // membership request by itself.
        false
        // self.all_rooms_loaded
    }

    /// Returns the state of the room if it is loaded and known to our client.
    pub fn get_room_state(&self, room_id: &OwnedRoomId) -> Option<RoomState> {
        if self.all_joined_rooms.contains_key(room_id) {
            return Some(RoomState::Joined);
        }
        if self.invited_rooms.borrow().contains_key(room_id) {
            return Some(RoomState::Invited);
        }
        None
    }

    /// Handle all pending updates to the list of all rooms.
    fn handle_rooms_list_updates(&mut self, cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        let mut num_updates: usize = 0;
        let mut needs_sort = false;
        while let Some(update) = PENDING_ROOM_UPDATES.pop() {
            num_updates += 1;
            match update {
                RoomsListUpdate::AddInvitedRoom(invited_room) => {
                    let room_id = invited_room.room_name_id.room_id().clone();
                    let should_display = should_display_room!(self, &room_id, &invited_room);
                    let _replaced = self
                        .invited_rooms
                        .borrow_mut()
                        .insert(room_id.clone(), invited_room);
                    if should_display {
                        self.displayed_invited_rooms.push(room_id);
                    }
                    self.update_status();
                    SignalToUI::set_ui_signal(); // signal the InviteScreen to update itself
                }
                RoomsListUpdate::AddJoinedRoom(joined_room) => {
                    let room_id = joined_room.room_name_id.room_id().clone();
                    let is_direct = joined_room.is_direct;
                    let should_display = should_display_room!(self, &room_id, &joined_room);
                    let _replaced = self.all_joined_rooms.insert(room_id.clone(), joined_room);
                    if should_display {
                        if is_direct {
                            self.displayed_direct_rooms.push(room_id.clone());
                        } else {
                            self.displayed_regular_rooms.push(room_id.clone());
                        }
                    }

                    // If this room was added as a result of accepting an invite, we must:
                    // 1. Remove the room from the list of invited rooms.
                    // 2. Update the displayed invited rooms list to remove this room.
                    // 3. Emit an action to inform other widgets that the InviteScreen
                    //    displaying the invite to this room should be converted to a
                    //    RoomScreen displaying the now-joined room.
                    if let Some(_accepted_invite) = self.invited_rooms.borrow_mut().remove(&room_id)
                    {
                        log!("Removed room {room_id} from the list of invited rooms");
                        self.displayed_invited_rooms
                            .iter()
                            .position(|r| r == &room_id)
                            .map(|index| self.displayed_invited_rooms.remove(index));
                        if let Some(room) = self.all_joined_rooms.get(&room_id) {
                            cx.widget_action(
                                self.widget_uid(),
                                RoomsListAction::InviteAccepted {
                                    room_name_id: room.room_name_id.clone(),
                                },
                            );
                        }
                    }
                    self.update_status();
                    SignalToUI::set_ui_signal(); // signal the RoomScreen to update itself
                }
                RoomsListUpdate::UpdateRoomAvatar {
                    room_id,
                    room_avatar,
                } => {
                    if let Some(room) = self.all_joined_rooms.get_mut(&room_id) {
                        room.room_avatar = room_avatar;
                    } else if let Some(room) = self.invited_rooms.borrow_mut().get_mut(&room_id) {
                        room.room_avatar = room_avatar;
                    } else {
                        error!("Error: couldn't find room {room_id} to update avatar");
                    }
                }
                RoomsListUpdate::UpdateLatestEvent {
                    room_id,
                    timestamp,
                    latest_message_text,
                } => {
                    if let Some(room) = self.all_joined_rooms.get_mut(&room_id) {
                        room.latest = Some((timestamp, latest_message_text));
                    } else {
                        error!("Error: couldn't find room {room_id} to update latest event");
                    }
                }
                RoomsListUpdate::UpdateNumUnreadMessages {
                    room_id,
                    is_marked_unread,
                    unread_messages,
                    unread_mentions,
                } => {
                    if let Some(room) = self.all_joined_rooms.get_mut(&room_id) {
                        room.num_unread_messages = match unread_messages {
                            UnreadMessageCount::Unknown => 0,
                            UnreadMessageCount::Known(count) => count,
                        };
                        room.num_unread_mentions = unread_mentions;
                        room.is_marked_unread = is_marked_unread;
                    } else {
                        warning!(
                            "Warning: couldn't find room {} to update unread messages count",
                            room_id
                        );
                    }
                }
                RoomsListUpdate::UpdateRoomName { new_room_name } => {
                    // TODO: broadcast a new AppState action to ensure that this room's or space's new name
                    //       gets updated in all of the `SelectedRoom` instances throughout Robrix,
                    //       e.g., the name of the room in the Dock Tab or the StackNav header.
                    // The list-row update below is local UI cache maintenance; it does not send
                    // room-state, message, or membership requests.

                    let room_id = new_room_name.room_id().clone();
                    // Try to update joined room first
                    if let Some(room) = self.all_joined_rooms.get_mut(&room_id) {
                        room.room_name_id = new_room_name;
                        let is_direct = room.is_direct;
                        let should_display = should_display_room!(self, &room_id, room);
                        let (pos_in_list, displayed_list) = if is_direct {
                            (
                                self.displayed_direct_rooms
                                    .iter()
                                    .position(|r| r == &room_id),
                                &mut self.displayed_direct_rooms,
                            )
                        } else {
                            (
                                self.displayed_regular_rooms
                                    .iter()
                                    .position(|r| r == &room_id),
                                &mut self.displayed_regular_rooms,
                            )
                        };
                        if should_display {
                            if pos_in_list.is_none() {
                                displayed_list.push(room_id);
                            }
                        } else {
                            pos_in_list.map(|i| displayed_list.remove(i));
                        }
                    }
                    // If not a joined room, try to update invited room
                    else {
                        let mut invited_rooms = self.invited_rooms.borrow_mut();
                        if let Some(invited_room) = invited_rooms.get_mut(&room_id) {
                            invited_room.room_name_id = new_room_name;
                            let should_display = should_display_room!(self, &room_id, invited_room);
                            let pos_in_list = self
                                .displayed_invited_rooms
                                .iter()
                                .position(|r| r == &room_id);
                            if should_display {
                                if pos_in_list.is_none() {
                                    self.displayed_invited_rooms.push(room_id);
                                }
                            } else {
                                pos_in_list.map(|i| self.displayed_invited_rooms.remove(i));
                            }
                        } else {
                            warning!(
                                "Warning: couldn't find room {new_room_name} to update its name."
                            );
                        }
                    }
                }
                RoomsListUpdate::UpdateIsDirect { room_id, is_direct } => {
                    if let Some(room) = self.all_joined_rooms.get_mut(&room_id) {
                        let was_direct = room.is_direct;
                        if was_direct == is_direct {
                            continue;
                        }

                        // Remove the room from the previous list (direct or regular).
                        let list_to_remove_from = if was_direct {
                            &mut self.displayed_direct_rooms
                        } else {
                            &mut self.displayed_regular_rooms
                        };
                        list_to_remove_from
                            .iter()
                            .position(|r| r == &room_id)
                            .map(|index| list_to_remove_from.remove(index));

                        // Update the room. If it should be displayed, add it to the proper list.
                        room.is_direct = is_direct;
                        if should_display_room!(self, &room_id, room) {
                            if is_direct {
                                self.displayed_direct_rooms.push(room_id);
                            } else {
                                self.displayed_regular_rooms.push(room_id);
                            }
                        }
                    } else {
                        error!("Error: couldn't find room {room_id} to update is_direct");
                    }
                }
                RoomsListUpdate::RemoveRoom { room_id, new_state } => {
                    // TODO: once we have a dedicated LoadingScreen widget, we should emit an action
                    // to replace this room (if it's currently open) with the LoadingScreen widget,
                    // which should show whether it has been left, kicked, or banned,
                    // and then options/buttons for the user to re-join it if desired.
                    // The list removal and stale-focus clear below are local UI cache maintenance;
                    // they do not send
                    // join/leave/knock, message, room-state, or membership requests.
                    let should_clear_active_room = removed_room_was_active_selection(
                        self.current_active_room.as_ref(),
                        &room_id,
                    );
                    self.last_removed_room_rejoin_packet =
                        Some(selected_room_removed_rejoin_packet_label(
                            &room_id,
                            &new_state,
                            should_clear_active_room,
                            self.selected_space.as_ref(),
                        ));

                    if let Some(removed) = self.all_joined_rooms.remove(&room_id) {
                        log!(
                            "Removed room {room_id} from the list of all joined rooms, now has state {new_state:?}"
                        );
                        let list_to_remove_from = if removed.is_direct {
                            &mut self.displayed_direct_rooms
                        } else {
                            &mut self.displayed_regular_rooms
                        };
                        list_to_remove_from
                            .iter()
                            .position(|r| r == &room_id)
                            .map(|index| list_to_remove_from.remove(index));
                    } else if let Some(_removed) = self.invited_rooms.borrow_mut().remove(&room_id)
                    {
                        log!("Removed room {room_id} from the list of all invited rooms");
                        self.displayed_invited_rooms
                            .iter()
                            .position(|r| r == &room_id)
                            .map(|index| self.displayed_invited_rooms.remove(index));
                    }

                    self.hidden_rooms.remove(&room_id);
                    if should_clear_active_room {
                        self.current_active_room = None;
                        cx.action(AppStateAction::FocusNone);
                    }
                    self.update_status();
                }
                RoomsListUpdate::ClearRooms => {
                    self.all_joined_rooms.clear();
                    self.displayed_direct_rooms.clear();
                    self.displayed_regular_rooms.clear();
                    self.invited_rooms.borrow_mut().clear();
                    self.displayed_invited_rooms.clear();
                    self.update_status();
                }
                RoomsListUpdate::NotLoaded => {
                    self.status =
                        "Loading workspaces (waiting for Matrix transport)...".to_string();
                }
                RoomsListUpdate::LoadedRooms { max_rooms } => {
                    self.max_known_rooms = max_rooms;
                }
                RoomsListUpdate::Tags { room_id, new_tags } => {
                    if let Some(room) = self.all_joined_rooms.get_mut(&room_id) {
                        room.tags = new_tags;
                    } else if let Some(_room) = self.invited_rooms.borrow().get(&room_id) {
                        log!("Ignoring updated tags update for invited room {room_id}");
                    } else {
                        warning!("Warning: skipping updated Tags for unknown room {room_id}.");
                    }
                }
                RoomsListUpdate::Status { status } => {
                    self.status = status;
                }
                RoomsListUpdate::TombstonedRoom { room_id } => {
                    if let Some(room) = self.all_joined_rooms.get_mut(&room_id) {
                        room.is_tombstoned = true;
                        let is_direct = room.is_direct;
                        let should_display = should_display_room!(self, &room_id, room);
                        let (pos_in_list, displayed_list) = if is_direct {
                            (
                                self.displayed_direct_rooms
                                    .iter()
                                    .position(|r| r == &room_id),
                                &mut self.displayed_direct_rooms,
                            )
                        } else {
                            (
                                self.displayed_regular_rooms
                                    .iter()
                                    .position(|r| r == &room_id),
                                &mut self.displayed_regular_rooms,
                            )
                        };
                        if should_display {
                            if pos_in_list.is_none() {
                                displayed_list.push(room_id);
                            }
                        } else {
                            pos_in_list.map(|i| displayed_list.remove(i));
                        }
                    } else {
                        warning!(
                            "Warning: couldn't find room {room_id} to update the tombstone status"
                        );
                    }
                }
                RoomsListUpdate::HideRoom { room_id } => {
                    self.hidden_rooms.insert(room_id.clone());
                    // Hiding a regular room is the most common case (e.g., after its successor is joined),
                    // so we check that list first.
                    if let Some(i) = self
                        .displayed_regular_rooms
                        .iter()
                        .position(|r| r == &room_id)
                    {
                        self.displayed_regular_rooms.remove(i);
                    } else if let Some(i) = self
                        .displayed_direct_rooms
                        .iter()
                        .position(|r| r == &room_id)
                    {
                        self.displayed_direct_rooms.remove(i);
                    } else if let Some(i) = self
                        .displayed_invited_rooms
                        .iter()
                        .position(|r| r == &room_id)
                    {
                        self.displayed_invited_rooms.remove(i);
                    }
                }
                RoomsListUpdate::ScrollToRoom(room_id) => {
                    // Ensure indexes are fresh in case rooms were added/removed in this batch of updates.
                    self.recalculate_indexes();
                    let portal_list = self.view.portal_list(cx, ids!(list));
                    let speed = 50.0;
                    let portal_list_index = if let Some(regular_index) = self
                        .displayed_regular_rooms
                        .iter()
                        .position(|r| r == &room_id)
                    {
                        self.regular_rooms_indexes.first_room_index + regular_index
                    } else if let Some(direct_index) = self
                        .displayed_direct_rooms
                        .iter()
                        .position(|r| r == &room_id)
                    {
                        self.direct_rooms_indexes.first_room_index + direct_index
                    } else if let Some(invited_index) = self
                        .displayed_invited_rooms
                        .iter()
                        .position(|r| r == &room_id)
                    {
                        self.invited_rooms_indexes.first_room_index + invited_index
                    } else {
                        continue;
                    };
                    portal_list.smooth_scroll_to(cx, portal_list_index, speed, Some(15), 10.0);
                }
                RoomsListUpdate::SpaceRequestSender(sender) => {
                    self.space_request_sender = Some(sender);
                    num_updates -= 1; // this does not require a redraw.
                }
                RoomsListUpdate::RoomOrderUpdate(diff) => match diff {
                    VecDiff::Append { values } => {
                        self.all_known_rooms_order.extend(values);
                        needs_sort = true;
                    }
                    VecDiff::Clear => {
                        self.all_known_rooms_order.clear();
                        needs_sort = true;
                    }
                    VecDiff::PushFront { value } => {
                        self.all_known_rooms_order.push_front(value);
                        needs_sort = true;
                    }
                    VecDiff::PushBack { value } => {
                        self.all_known_rooms_order.push_back(value);
                        needs_sort = true;
                    }
                    VecDiff::PopFront => {
                        self.all_known_rooms_order.pop_front();
                        needs_sort = true;
                    }
                    VecDiff::PopBack => {
                        self.all_known_rooms_order.pop_back();
                        needs_sort = true;
                    }
                    VecDiff::Insert { index, value } => {
                        if index <= self.all_known_rooms_order.len() {
                            self.all_known_rooms_order.insert(index, value);
                            needs_sort = true;
                        }
                    }
                    VecDiff::Set { index, value } => {
                        if let Some(existing) = self.all_known_rooms_order.get_mut(index) {
                            if *existing != value {
                                *existing = value;
                                needs_sort = true;
                            }
                        }
                    }
                    VecDiff::Remove { index } => {
                        if index < self.all_known_rooms_order.len() {
                            self.all_known_rooms_order.remove(index);
                            needs_sort = true;
                        }
                    }
                    VecDiff::Truncate { length } => {
                        self.all_known_rooms_order.truncate(length);
                        needs_sort = true;
                    }
                },
            }
        }
        if needs_sort {
            // Only re-sort if there's no active sort function
            if self.sort_fn.is_none() {
                self.update_displayed_rooms(cx, false);
            }
        }
        if num_updates > 0 {
            self.redraw(cx);
        }
    }

    /// Updates the status message to show how many rooms have been loaded
    /// or how many rooms match the current room filter keywords.
    ///
    /// Note: this *does not* actually redraw the status message or rooms list;
    ///       that must be done separately.
    fn update_status(&mut self) {
        let num_rooms = self.visible_room_count();

        let mut text = match (self.display_filter.is_none(), num_rooms) {
            (true, 0) => "No joined or invited rooms found".to_string(),
            (true, 1) => "Loaded 1 room".to_string(),
            (true, n) => format!("Loaded {n} rooms"),
            (false, 0) => "No matching rooms found".to_string(),
            (false, 1) => "Found 1 matching room".to_string(),
            (false, n) => format!("Found {n} matching rooms"),
        };
        match self.selected_space.is_some() {
            true => text.push_str(" in this space."),
            false => text.push('.'),
        };
        self.status = text;
    }

    fn visible_room_count(&self) -> usize {
        self.displayed_invited_rooms.len()
            + self.displayed_direct_rooms.len()
            + self.displayed_regular_rooms.len()
    }

    fn status_evidence_text(&self) -> &'static str {
        if self.status.starts_with("Loading") {
            "Dialog list loading waits for existing RoomListService/SlidingSync updates; no message, room-state, or membership mutation was sent."
        } else if self.visible_room_count() == 0 {
            match (self.display_filter.is_some(), self.selected_space.is_some()) {
                (true, true) => {
                    "Dialog filter empty state is local RoomDisplayFilter matching over loaded room fields plus cached SpaceService children; no Matrix search query, JoinRoom, LeaveSpace, membership, or room-state mutation was sent."
                }
                (true, false) => {
                    "Dialog filter empty state is local RoomDisplayFilter matching over loaded room fields and tags; no Matrix search query, message send, room-state, or membership mutation was sent."
                }
                (false, true) => {
                    "Empty dialog list in this space is local rendering of RoomsList state from cached SpaceService children; no JoinRoom, LeaveSpace, membership, or room-state mutation was sent."
                }
                (false, false) => {
                    "Empty dialog list is local rendering of loaded joined/invited room state; no Matrix search query, JoinRoom, LeaveRoom, message send, room-state, or membership mutation was sent."
                }
            }
        } else if self.display_filter.is_some() {
            "Dialog filter results are local RoomDisplayFilter matches over loaded room fields and tags; no Matrix search query, message send, room-state, or membership mutation was sent."
        } else {
            "Dialog list status comes from existing RoomListService and cached room state; count rendering sends no message, room-state, or membership mutation."
        }
    }

    fn room_list_pagination_evidence_text(&self) -> String {
        let adapter_limit = "RoomListService entries_with_dynamic_adapters(usize::MAX)";
        let max_hint = self
            .max_known_rooms
            .map(|count| format!(" server hint max {count}"))
            .unwrap_or_else(|| " no server max hint yet".to_string());
        format!(
            "Room-list adapter: {adapter_limit} feeds the loaded list locally ({max_hint}); no Load more rooms UI or room-list pagination request is emitted. Visible room rows may prefetch latest-message previews through the existing Matrix PaginateTimeline read path only."
        )
    }

    fn selected_space_pagination_status_text(&self) -> String {
        let Some(selected_space) = self.selected_space.as_ref() else {
            return "selected_space none; selected_space_child_pagination not_applicable"
                .to_string();
        };
        let Some(space_state) = self.space_map.get(selected_space.room_id()) else {
            return format!(
                "selected_space {}; selected_space_child_pagination map_missing_read_sync_pending",
                selected_space.room_id()
            );
        };
        let pagination_state = if space_state.is_fully_paginated {
            "fully_paginated"
        } else {
            "service_read_sync_in_progress_or_pending"
        };
        format!(
            "selected_space {}; selected_space_child_pagination {}; direct_child_rooms {}; direct_subspaces {}",
            selected_space.room_id(),
            pagination_state,
            space_state.direct_child_rooms.len(),
            space_state.direct_subspaces.len()
        )
    }

    fn room_list_load_more_pagination_packet_text(&self) -> String {
        room_list_load_more_pagination_packet_label(
            self.all_joined_rooms.len(),
            self.displayed_invited_rooms.len(),
            self.displayed_direct_rooms.len(),
            self.displayed_regular_rooms.len(),
            self.max_known_rooms,
            &self.selected_space_pagination_status_text(),
        )
    }

    fn space_unread_filter_evidence_text(&self) -> &'static str {
        SPACE_UNREAD_MENTION_FILTER_LOCAL_ZERO_EVIDENCE
    }

    fn section_unread_aggregate_snapshots(
        &self,
    ) -> (
        SectionUnreadAggregateSnapshot,
        SectionUnreadAggregateSnapshot,
    ) {
        (
            SectionUnreadAggregateSnapshot::from_displayed_rooms(
                &self.displayed_direct_rooms,
                &self.all_joined_rooms,
            ),
            SectionUnreadAggregateSnapshot::from_displayed_rooms(
                &self.displayed_regular_rooms,
                &self.all_joined_rooms,
            ),
        )
    }

    fn section_unread_aggregate_evidence_text(&self) -> String {
        let (direct, regular) = self.section_unread_aggregate_snapshots();
        section_unread_aggregate_packet_label(
            direct,
            regular,
            self.selected_space.as_ref(),
            self.display_filter.is_some(),
        )
    }

    fn space_parent_cache_evidence_text(&self) -> &'static str {
        ROOMS_LIST_SPACE_PARENT_CACHE_LOCAL_EVIDENCE
    }

    fn room_name_update_evidence_text(&self) -> &'static str {
        ROOMS_LIST_NAME_UPDATE_SELECTED_STATE_LOCAL_EVIDENCE
    }

    fn removed_room_selected_state_evidence_text(&self) -> String {
        self.last_removed_room_rejoin_packet
            .clone()
            .unwrap_or_else(|| ROOMS_LIST_REMOVED_ROOM_SELECTED_STATE_LOCAL_EVIDENCE.to_string())
    }

    /// Updates the display filter and sort function based on the
    /// current filter keywords and the currently-selected space (if any).
    fn regenerate_display_filter_and_sort_fn(&mut self, filter_keywords: &str) {
        // Determine and set the filter function and sort function.
        let (display_fn, sort_fn) = if filter_keywords.is_empty() {
            (RoomDisplayFilter::default(), None)
        } else {
            // Create a new filter function based on the given keywords.
            RoomDisplayFilterBuilder::new()
                .set_keywords(filter_keywords.into())
                .set_filter_criteria(RoomFilterCriteria::All)
                .build()
        };
        self.display_filter = display_fn;
        self.sort_fn = sort_fn;
    }

    /// Updates and redraws the lists of displayed rooms in the RoomsList.
    ///
    /// If `reset_scroll` is true, the portal list will scroll to the top.
    /// If `false`, the scroll position is preserved, unless it exceeds the new list length,
    /// in which case the logic in `draw_walk()` will limit it to the max valid index.
    fn update_displayed_rooms(&mut self, cx: &mut Cx, reset_scroll: bool) {
        let (invited, regular, direct) = self.generate_displayed_rooms();
        self.displayed_invited_rooms = invited;
        self.displayed_regular_rooms = regular;
        self.displayed_direct_rooms = direct;

        self.update_status();

        let portal_list = self.view.portal_list(cx, ids!(list));
        if reset_scroll {
            portal_list.set_first_id_and_scroll(0, 0.0);
        }
        self.redraw(cx);
    }

    /// Generates a tuple of three kinds of displayed rooms (accounting for the current `display_filter`):
    /// 1. displayed_invited_rooms
    /// 2. displayed_regular_rooms
    /// 3. displayed_direct_rooms
    ///
    /// If `self.sort_fn` is `Some`, the rooms are ordered based on that function.
    /// Otherwise, the rooms are ordered based on `self.all_known_rooms_order` (the default).
    fn generate_displayed_rooms(&self) -> (Vec<OwnedRoomId>, Vec<OwnedRoomId>, Vec<OwnedRoomId>) {
        let mut new_displayed_invited_rooms = Vec::new();
        let mut new_displayed_regular_rooms = Vec::new();
        let mut new_displayed_direct_rooms = Vec::new();

        let mut push_joined_room = |room_id: &OwnedRoomId, jr: &JoinedRoomInfo| {
            let room_id = room_id.clone();
            if jr.is_direct {
                new_displayed_direct_rooms.push(room_id);
            } else {
                new_displayed_regular_rooms.push(room_id);
            }
        };

        let invited_rooms_ref = self.invited_rooms.borrow();

        // If a sort function was provided, use it.
        if let Some(sort_fn) = self.sort_fn.as_deref() {
            let mut filtered_joined_rooms = self
                .all_joined_rooms
                .iter()
                .filter(|&(room_id, room)| should_display_room!(self, room_id, room))
                .collect::<Vec<_>>();
            filtered_joined_rooms.sort_by(|(_, room_a), (_, room_b)| sort_fn(*room_a, *room_b));
            for (room_id, jr) in filtered_joined_rooms.into_iter() {
                push_joined_room(room_id, jr)
            }

            let mut filtered_invited_rooms = invited_rooms_ref
                .iter()
                .filter(|&(room_id, room)| should_display_room!(self, room_id, room))
                .collect::<Vec<_>>();
            filtered_invited_rooms.sort_by(|(_, room_a), (_, room_b)| sort_fn(*room_a, *room_b));
            for (room_id, _ir) in filtered_invited_rooms.into_iter() {
                new_displayed_invited_rooms.push(room_id.clone());
            }
        }
        // Otherwise, if no sort function was provided (default), use the `all_known_rooms_order`.
        else {
            for room_id in &self.all_known_rooms_order {
                if let Some(jr) = self.all_joined_rooms.get(room_id) {
                    if should_display_room!(self, room_id, jr) {
                        push_joined_room(room_id, jr);
                    }
                } else if let Some(ir) = invited_rooms_ref.get(room_id) {
                    if should_display_room!(self, room_id, ir) {
                        new_displayed_invited_rooms.push(room_id.clone());
                    }
                }
            }
        }

        (
            new_displayed_invited_rooms,
            new_displayed_regular_rooms,
            new_displayed_direct_rooms,
        )
    }

    /// Calculates the indexes in the PortalList where the headers and rooms should be drawn.
    ///
    /// Updates the following three fields:
    /// 1. `invited_rooms_indexes`: the indexes for the invited rooms,
    /// 2. `direct_rooms_indexes`: the indexes for the direct rooms (DMs / People),
    /// 3. `regular_rooms_indexes`: the indexes for the regular non-direct joined rooms.
    fn recalculate_indexes(&mut self) {
        // Based on the various displayed room lists and is_expanded state of each room header,
        // calculate the indexes in the PortalList where the headers and rooms should be drawn.
        let should_show_invited_rooms_header = !self.displayed_invited_rooms.is_empty();
        let should_show_direct_rooms_header = !self.displayed_direct_rooms.is_empty();
        let should_show_regular_rooms_header = !self.displayed_regular_rooms.is_empty();

        let index_of_invited_rooms_header = should_show_invited_rooms_header.then_some(0);
        let index_of_first_invited_room = should_show_invited_rooms_header as usize;
        let index_after_invited_rooms = index_of_first_invited_room
            + if self.is_invited_rooms_header_expanded {
                self.displayed_invited_rooms.len()
            } else {
                0
            };

        let index_of_direct_rooms_header =
            should_show_direct_rooms_header.then_some(index_after_invited_rooms);
        let index_of_first_direct_room =
            index_after_invited_rooms + should_show_direct_rooms_header as usize;
        let index_after_direct_rooms = index_of_first_direct_room
            + if self.is_direct_rooms_header_expanded {
                self.displayed_direct_rooms.len()
            } else {
                0
            };

        let index_of_regular_rooms_header =
            should_show_regular_rooms_header.then_some(index_after_direct_rooms);
        let index_of_first_regular_room =
            index_after_direct_rooms + should_show_regular_rooms_header as usize;
        let index_after_regular_rooms = index_of_first_regular_room
            + if self.is_regular_rooms_header_expanded {
                self.displayed_regular_rooms.len()
            } else {
                0
            };

        self.invited_rooms_indexes = RoomCategoryIndexes {
            header_index: index_of_invited_rooms_header,
            first_room_index: index_of_first_invited_room,
            after_rooms_index: index_after_invited_rooms,
        };
        self.direct_rooms_indexes = RoomCategoryIndexes {
            header_index: index_of_direct_rooms_header,
            first_room_index: index_of_first_direct_room,
            after_rooms_index: index_after_direct_rooms,
        };
        self.regular_rooms_indexes = RoomCategoryIndexes {
            header_index: index_of_regular_rooms_header,
            first_room_index: index_of_first_regular_room,
            after_rooms_index: index_after_regular_rooms,
        };
    }

    /// Handle any incoming updates to spaces' room lists and pagination state.
    fn handle_space_room_list_action(&mut self, cx: &mut Cx, action: &SpaceRoomListAction) {
        match action {
            SpaceRoomListAction::UpdatedChildren {
                space_id,
                parent_chain,
                direct_child_rooms,
                direct_subspaces,
            } => {
                match self.space_map.entry(space_id.clone()) {
                    Entry::Occupied(mut occ) => {
                        let occ_mut = occ.get_mut();
                        occ_mut.parent_chain = parent_chain.clone();
                        occ_mut.direct_child_rooms = Arc::clone(direct_child_rooms);
                        occ_mut.direct_subspaces = Arc::clone(direct_subspaces);
                    }
                    Entry::Vacant(vac) => {
                        vac.insert_entry(SpaceMapValue {
                            is_fully_paginated: false,
                            parent_chain: parent_chain.clone(),
                            direct_child_rooms: Arc::clone(direct_child_rooms),
                            direct_subspaces: Arc::clone(direct_subspaces),
                        });
                    }
                }
                if self.selected_space.as_ref().is_some_and(|sel_space| {
                    sel_space.room_id() == space_id || parent_chain.contains(sel_space.room_id())
                }) {
                    self.update_displayed_rooms(cx, false);
                }
            }
            SpaceRoomListAction::PaginationState {
                space_id,
                parent_chain,
                state,
            } => {
                let is_fully_paginated = matches!(
                    state,
                    SpaceRoomListPaginationState::Idle { end_reached: true }
                );
                // Only re-fetch the list of rooms in this space if it was not already fully paginated.
                let should_fetch_rooms: bool;
                match self.space_map.entry(space_id.clone()) {
                    Entry::Occupied(mut occ) => {
                        let value_mut = occ.get_mut();
                        should_fetch_rooms = !value_mut.is_fully_paginated;
                        value_mut.is_fully_paginated = is_fully_paginated;
                    }
                    Entry::Vacant(vac) => {
                        vac.insert_entry(SpaceMapValue {
                            is_fully_paginated,
                            parent_chain: parent_chain.clone(),
                            ..Default::default()
                        });
                        should_fetch_rooms = true;
                    }
                }
                let Some(sender) = self.space_request_sender.as_ref() else {
                    error!(
                        "BUG: RoomsList: no space request sender was available after pagination state update."
                    );
                    return;
                };
                if should_fetch_rooms {
                    if sender
                        .send(SpaceRequest::GetChildren {
                            space_id: space_id.clone(),
                            parent_chain: parent_chain.clone(),
                        })
                        .is_err()
                    {
                        error!(
                            "BUG: RoomsList: failed to send GetRooms request for space {space_id}."
                        );
                    }
                }

                // In order to determine which rooms are in a given top-level space,
                // we also must know all of the rooms within that space's subspaces.
                // Thus, we must continue paginating this space until we fully fetch
                // all of its children, such that we can see if any of them are subspaces,
                // and then we'll paginate those as well.
                if !is_fully_paginated {
                    if sender
                        .send(SpaceRequest::PaginateSpaceRoomList {
                            space_id: space_id.clone(),
                            parent_chain: parent_chain.clone(),
                        })
                        .is_err()
                    {
                        error!(
                            "BUG: RoomsList: failed to send pagination request for space {space_id}."
                        );
                    }
                }
            }
            SpaceRoomListAction::PaginationError { space_id, error } => {
                error!("RoomsList: failed to paginate rooms in space {space_id}: {error:?}");
                enqueue_popup_notification(
                    "Failed to fetch more rooms in this space. Try again later.",
                    PopupKind::Error,
                    None,
                );
            }
            SpaceRoomListAction::LeaveSpaceResult {
                space_name_id,
                result,
            } => match result {
                Ok(()) => {
                    enqueue_popup_notification(
                        format!("Successfully left space \"{}\".", space_name_id),
                        PopupKind::Success,
                        Some(4.0),
                    );
                    // If the space we left was the currently-selected one, go back to the main Home view.
                    if self
                        .selected_space
                        .as_ref()
                        .is_some_and(|s| s.room_id() == space_name_id.room_id())
                    {
                        cx.action(NavigationBarAction::GoToHome);
                    }
                }
                Err(e) => {
                    error!("Failed to leave space {space_name_id:?}: {e:?}");
                    enqueue_popup_notification(
                        format!("Failed to leave space \"{space_name_id}\".\n\nError: {e}"),
                        PopupKind::Error,
                        None,
                    );
                }
            },
            // Details-related space actions are handled by SpaceLobbyScreen, not RoomsList.
            SpaceRoomListAction::DetailedChildren { .. }
            | SpaceRoomListAction::TopLevelSpaceDetails(_) => {}
        }
    }

    /// Returns whether the given target room or space is indirectly within the given parent space.
    ///
    /// This will recursively search all nested spaces within the given `parent_space`.
    fn is_room_indirectly_in_space(
        &self,
        parent_space: &OwnedRoomId,
        target: &OwnedRoomId,
    ) -> bool {
        if let Some(smv) = self.space_map.get(parent_space) {
            if smv.direct_child_rooms.contains(target) {
                return true;
            }
            for subspace in smv.direct_subspaces.iter() {
                if self.is_room_indirectly_in_space(subspace, target) {
                    return true;
                }
            }
        }
        false
    }
}

impl Widget for RoomsList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Process all pending updates to the list of all rooms, and then redraw it.
        if matches!(event, Event::Signal) {
            self.handle_rooms_list_updates(cx, event, scope);
        }

        // First, we handle any actions that came from widgets within the room list,
        // e.g., the user clicking on a RoomsListEntry to select a room.
        // We use Scope `props` to pass down the current scrolling state of the PortalList.
        let props = RoomsListScopeProps {
            was_scrolling: self.view.portal_list(cx, ids!(list)).was_scrolling(),
        };
        let rooms_list_actions = cx.capture_actions(|cx| {
            self.view
                .handle_event(cx, event, &mut Scope::with_props(&props))
        });
        for action in rooms_list_actions {
            // Handle a regular room (joined or invited) being clicked.
            if let RoomsListEntryAction::PrimaryClicked(room_id) = action.as_widget_action().cast()
            {
                let new_selected_room = if let Some(jr) = self.all_joined_rooms.get(&room_id) {
                    SelectedRoom::JoinedRoom {
                        room_name_id: jr.room_name_id.clone(),
                    }
                } else if let Some(ir) = self.invited_rooms.borrow().get(&room_id) {
                    SelectedRoom::InvitedRoom {
                        room_name_id: ir.room_name_id.clone(),
                    }
                } else {
                    error!("BUG: couldn't find clicked room details for room {room_id}");
                    continue;
                };

                self.current_active_room = Some(new_selected_room.clone());
                cx.widget_action(
                    self.widget_uid(),
                    RoomsListAction::Selected(new_selected_room),
                );
                self.redraw(cx);
            }
            // Handle a room being right-clicked or long-pressed by opening the room context menu.
            else if let RoomsListEntryAction::SecondaryClicked(room_id, pos) =
                action.as_widget_action().cast()
            {
                // Determine details for the context menu
                let Some(jr) = self.all_joined_rooms.get(&room_id) else {
                    error!("BUG: couldn't find right-clicked room details for room {room_id}");
                    continue;
                };
                let details = room_context_menu_details_from_joined_room(jr);
                cx.widget_action(
                    self.widget_uid(),
                    RoomsListAction::OpenRoomContextMenu { details, pos },
                );
            }
            // Handle the space lobby being clicked.
            else if let Some(SpaceLobbyAction::SpaceLobbyEntryClicked) = action.downcast_ref() {
                let Some(space_name_id) = self.selected_space.clone() else {
                    continue;
                };
                let new_selected_space = SelectedRoom::Space { space_name_id };
                self.current_active_room = Some(new_selected_space.clone());
                cx.widget_action(
                    self.widget_uid(),
                    RoomsListAction::Selected(new_selected_space),
                );
                self.redraw(cx);
            }
            // Handle a collapsible header being clicked.
            else if let CollapsibleHeaderAction::Toggled { category } =
                action.as_widget_action().cast()
            {
                if toggle_supported_header_category(
                    category,
                    &mut self.is_invited_rooms_header_expanded,
                    &mut self.is_direct_rooms_header_expanded,
                    &mut self.is_regular_rooms_header_expanded,
                ) {
                    self.redraw(cx);
                }
            }
        }

        // Second, handle any other actions that came from other widgets/components.
        if let Event::Actions(actions) = event {
            for action in actions {
                // Clear widget state upon logout.
                if let Some(LogoutAction::ClearAppState { .. }) = action.downcast_ref() {
                    self.invited_rooms.borrow_mut().clear();
                    self.all_joined_rooms.clear();
                    self.all_known_rooms_order.clear();
                    self.selected_space = None;
                    self.space_request_sender = None;
                    self.space_map.clear();
                    self.hidden_rooms.clear();
                    self.displayed_invited_rooms.clear();
                    self.invited_rooms_indexes = Default::default();
                    self.displayed_direct_rooms.clear();
                    self.direct_rooms_indexes = Default::default();
                    self.displayed_regular_rooms.clear();
                    self.regular_rooms_indexes = Default::default();
                    self.current_active_room = None;
                    self.max_known_rooms = None;
                    self.last_removed_room_rejoin_packet = None;
                    self.status = String::new();
                    self.update_status();
                    self.redraw(cx);
                    continue;
                }

                // Only handle filter changes from the home screen's filter bar,
                // not from any other RoomFilterInputBar instance (e.g., SpaceLobbyScreen's).
                if let Some(MainFilterAction::Changed(keywords)) = action.downcast_ref() {
                    self.regenerate_display_filter_and_sort_fn(keywords);
                    self.update_displayed_rooms(cx, true);
                    continue;
                }

                // Handle a space navigation tab being selected or de-selected.
                if let Some(NavigationBarAction::TabSelected(tab)) = action.downcast_ref() {
                    match tab {
                        SelectedTab::Space { space_name_id } => {
                            if self
                                .selected_space
                                .as_ref()
                                .is_some_and(|s| s.room_id() == space_name_id.room_id())
                            {
                                continue;
                            }

                            self.selected_space = Some(space_name_id.clone());
                            self.view
                                .space_lobby_entry(cx, ids!(space_lobby_entry))
                                .set_visible(cx, true);

                            // If we don't have the full list of children in this newly-selected space, then fetch it.
                            let (is_fully_paginated, parent_chain) = self
                                .space_map
                                .get(space_name_id.room_id())
                                .map(|smv| (smv.is_fully_paginated, smv.parent_chain.clone()))
                                .unwrap_or_default();
                            if !is_fully_paginated {
                                let Some(sender) = self.space_request_sender.as_ref() else {
                                    error!(
                                        "BUG: RoomsList: no space request sender was available."
                                    );
                                    continue;
                                };

                                if sender
                                    .send(SpaceRequest::SubscribeToSpaceRoomList {
                                        space_id: space_name_id.room_id().clone(),
                                        parent_chain: parent_chain.clone(),
                                    })
                                    .is_err()
                                {
                                    error!(
                                        "BUG: RoomsList: failed to send SubscribeToSpaceRoomList request for space {space_name_id}."
                                    );
                                }
                                if sender
                                    .send(SpaceRequest::PaginateSpaceRoomList {
                                        space_id: space_name_id.room_id().clone(),
                                        parent_chain: parent_chain.clone(),
                                    })
                                    .is_err()
                                {
                                    error!(
                                        "BUG: RoomsList: failed to send PaginateSpaceRoomList request for space {space_name_id}."
                                    );
                                }
                                if sender
                                    .send(SpaceRequest::GetChildren {
                                        space_id: space_name_id.room_id().clone(),
                                        parent_chain,
                                    })
                                    .is_err()
                                {
                                    error!(
                                        "BUG: RoomsList: failed to send GetRooms request for space {space_name_id}."
                                    );
                                }
                            }
                        }
                        _ => {
                            self.selected_space = None;
                            self.view
                                .space_lobby_entry(cx, ids!(space_lobby_entry))
                                .set_visible(cx, false);
                        }
                    }

                    self.update_displayed_rooms(cx, true);
                    continue;
                }

                // Handle a matrix link being generated.
                fn on_link_generated(cx: &mut Cx, link: &str) {
                    cx.copy_to_clipboard(link);
                    enqueue_popup_notification(
                        "Room link copied after existing Matrix link generation. No Matrix room state event or membership mutation was sent.",
                        PopupKind::Success,
                        Some(3.0),
                    );
                }
                match action.downcast_ref() {
                    Some(MatrixLinkAction::MatrixToUri(link)) => {
                        on_link_generated(cx, &link.to_string());
                        continue;
                    }
                    Some(MatrixLinkAction::MatrixUri(link)) => {
                        on_link_generated(cx, &link.to_string());
                        continue;
                    }
                    Some(MatrixLinkAction::Error(err)) => {
                        enqueue_popup_notification(
                            format!("Failed to generate link: {}", err),
                            PopupKind::Error,
                            Some(5.0),
                        );
                        continue;
                    }
                    _ => {}
                }

                if let Some(space_room_list_action) = action.downcast_ref() {
                    self.handle_space_room_list_action(cx, space_room_list_action);
                    continue;
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let app_state = scope.data.get_mut::<AppState>().unwrap();
        // Update the currently-selected room from the AppState data.
        self.current_active_room = app_state.selected_room.clone();

        // Based on the various displayed room lists and is_expanded state of each room header,
        // calculate the indexes in the PortalList where the headers and rooms should be drawn.
        self.recalculate_indexes();

        let status_label_id = self.regular_rooms_indexes.after_rooms_index;
        // Add one for the status label
        let total_count = status_label_id + 1;

        let get_invited_room_id = |portal_list_index: usize| {
            portal_list_index
                .checked_sub(self.invited_rooms_indexes.first_room_index)
                .and_then(|index| {
                    self.is_invited_rooms_header_expanded
                        .then(|| self.displayed_invited_rooms.get(index))
                })
                .flatten()
        };

        let get_direct_room_id = |portal_list_index: usize| {
            portal_list_index
                .checked_sub(self.direct_rooms_indexes.first_room_index)
                .and_then(|index| {
                    self.is_direct_rooms_header_expanded
                        .then(|| self.displayed_direct_rooms.get(index))
                })
                .flatten()
        };
        let get_regular_room_id = |portal_list_index: usize| {
            portal_list_index
                .checked_sub(self.regular_rooms_indexes.first_room_index)
                .and_then(|index| {
                    self.is_regular_rooms_header_expanded
                        .then(|| self.displayed_regular_rooms.get(index))
                })
                .flatten()
        };

        // Start the actual drawing procedure.
        while let Some(widget_to_draw) = self.view.draw_walk(cx, scope, walk).step() {
            // Ensure that the portal list is not scrolled past the end of the list.
            let portal_list_ref = widget_to_draw.as_portal_list();
            if portal_list_ref.first_id() > status_label_id {
                portal_list_ref.set_first_id_and_scroll(status_label_id, 0.0);
            }
            // We only care about drawing the portal list.
            let Some(mut list) = portal_list_ref.borrow_mut() else {
                continue;
            };

            list.set_item_range(cx, 0, total_count);

            while let Some(portal_list_index) = list.next_visible_item(cx) {
                let mut scope = Scope::empty();

                if self.invited_rooms_indexes.header_index == Some(portal_list_index) {
                    let item = list.item(cx, portal_list_index, id!(collapsible_header));
                    item.as_collapsible_header().set_details(
                        cx,
                        self.is_invited_rooms_header_expanded,
                        HeaderCategory::Invites,
                        self.displayed_invited_rooms.len() as u64,
                    );
                    item.draw_all(cx, &mut scope);
                } else if let Some(invited_room_id) = get_invited_room_id(portal_list_index) {
                    let mut invited_rooms_mut = self.invited_rooms.borrow_mut();
                    if let Some(invited_room) = invited_rooms_mut.get_mut(invited_room_id) {
                        let item = list.item(cx, portal_list_index, id!(rooms_list_entry));
                        invited_room.is_selected = self
                            .current_active_room
                            .as_ref()
                            .is_some_and(|sel_room| sel_room.room_id() == invited_room_id);
                        // Pass the room info down to the RoomsListEntry widget via Scope.
                        scope = Scope::with_props(&*invited_room);
                        item.draw_all(cx, &mut scope);
                    } else {
                        list.item(cx, portal_list_index, id!(empty))
                            .draw_all(cx, &mut scope);
                    }
                } else if self.direct_rooms_indexes.header_index == Some(portal_list_index) {
                    let item = list.item(cx, portal_list_index, id!(collapsible_header));
                    item.as_collapsible_header().set_details(
                        cx,
                        self.is_direct_rooms_header_expanded,
                        HeaderCategory::DirectRooms,
                        // Section-level mention totals are not maintained yet.
                        0,
                        // TODO: sum up all the unread mentions in rooms.
                        // NOTE: this might be really slow, so we should maintain a running total of mentions in this struct
                    );
                    item.draw_all(cx, &mut scope);
                } else if let Some(direct_room_id) = get_direct_room_id(portal_list_index) {
                    if let Some(direct_room) = self.all_joined_rooms.get_mut(direct_room_id) {
                        let item = list.item(cx, portal_list_index, id!(rooms_list_entry));
                        direct_room.is_selected = self
                            .current_active_room
                            .as_ref()
                            .is_some_and(|sel_room| sel_room.room_id() == direct_room_id);

                        // Paginate the room if it hasn't been paginated yet.
                        if PREPAGINATE_VISIBLE_ROOMS && !direct_room.has_been_paginated {
                            direct_room.has_been_paginated = true;
                            submit_async_request(MatrixRequest::PaginateTimeline {
                                timeline_kind: TimelineKind::MainRoom {
                                    room_id: direct_room.room_name_id.room_id().clone(),
                                },
                                num_events: 50,
                                direction: PaginationDirection::Backwards,
                            });
                        }
                        // Pass the room info down to the RoomsListEntry widget via Scope.
                        scope = Scope::with_props(&*direct_room);
                        item.draw_all(cx, &mut scope);
                    } else {
                        list.item(cx, portal_list_index, id!(empty))
                            .draw_all(cx, &mut scope);
                    }
                } else if self.regular_rooms_indexes.header_index == Some(portal_list_index) {
                    let item = list.item(cx, portal_list_index, id!(collapsible_header));
                    item.as_collapsible_header().set_details(
                        cx,
                        self.is_regular_rooms_header_expanded,
                        HeaderCategory::RegularRooms,
                        // Section-level mention totals are not maintained yet.
                        0,
                        // TODO: sum up all the unread mentions in rooms.
                        // NOTE: this might be really slow, so we should maintain a running total of mentions in this struct
                    );
                    item.draw_all(cx, &mut scope);
                } else if let Some(regular_room_id) = get_regular_room_id(portal_list_index) {
                    if let Some(regular_room) = self.all_joined_rooms.get_mut(regular_room_id) {
                        let item = list.item(cx, portal_list_index, id!(rooms_list_entry));
                        regular_room.is_selected = self
                            .current_active_room
                            .as_ref()
                            .is_some_and(|sel_room| sel_room.room_id() == regular_room_id);

                        // Paginate the room if it hasn't been paginated yet.
                        if PREPAGINATE_VISIBLE_ROOMS && !regular_room.has_been_paginated {
                            regular_room.has_been_paginated = true;
                            submit_async_request(MatrixRequest::PaginateTimeline {
                                timeline_kind: TimelineKind::MainRoom {
                                    room_id: regular_room.room_name_id.room_id().clone(),
                                },
                                num_events: 50,
                                direction: PaginationDirection::Backwards,
                            });
                        }
                        // Pass the room info down to the RoomsListEntry widget via Scope.
                        scope = Scope::with_props(&*regular_room);
                        item.draw_all(cx, &mut scope);
                    } else {
                        list.item(cx, portal_list_index, id!(empty))
                            .draw_all(cx, &mut scope);
                    }
                }
                // Draw the status label as the bottom entry.
                else if portal_list_index == status_label_id {
                    let item = list.item(cx, portal_list_index, id!(status_label));
                    let is_loading = self.status.starts_with("Loading");
                    item.child_by_path(ids!(status_row.loading_spinner))
                        .set_visible(cx, is_loading);
                    item.label(cx, ids!(status_row.label))
                        .set_text(cx, &self.status);
                    item.label(cx, ids!(evidence_label))
                        .set_text(cx, self.status_evidence_text());
                    item.label(cx, ids!(room_list_pagination_evidence_label))
                        .set_text(cx, &self.room_list_pagination_evidence_text());
                    item.label(cx, ids!(room_list_load_more_packet_label))
                        .set_text(cx, &self.room_list_load_more_pagination_packet_text());
                    item.label(cx, ids!(space_unread_filter_evidence_label))
                        .set_text(cx, self.space_unread_filter_evidence_text());
                    item.label(cx, ids!(section_unread_aggregate_evidence_label))
                        .set_text(cx, &self.section_unread_aggregate_evidence_text());
                    item.label(cx, ids!(space_parent_cache_evidence_label))
                        .set_text(cx, self.space_parent_cache_evidence_text());
                    item.label(cx, ids!(room_name_update_evidence_label))
                        .set_text(cx, self.room_name_update_evidence_text());
                    item.label(cx, ids!(removed_room_selected_state_evidence_label))
                        .set_text(cx, &self.removed_room_selected_state_evidence_text());
                    item.draw_all(cx, &mut scope);
                }
                // Draw a filler entry to take up space at the bottom of the portal list.
                else {
                    list.item(cx, portal_list_index, id!(bottom_filler))
                        .draw_all(cx, &mut scope);
                }
            }
        }

        DrawStep::done()
    }
}

impl RoomsListRef {
    /// See [`RoomsList::all_rooms_loaded()`].
    pub fn all_rooms_loaded(&self) -> bool {
        let Some(inner) = self.borrow() else {
            return false;
        };
        inner.all_rooms_loaded()
    }

    /// Returns `true` if this room is loaded and known to our client.
    pub fn is_room_loaded(&self, room_id: &OwnedRoomId) -> bool {
        self.get_room_state(room_id).is_some()
    }

    /// See [`RoomsList::get_room_state()`].
    pub fn get_room_state(&self, room_id: &OwnedRoomId) -> Option<RoomState> {
        self.borrow()?.get_room_state(room_id)
    }

    /// Returns the name of the given room, if it is known and loaded.
    pub fn get_room_name(&self, room_id: &OwnedRoomId) -> Option<RoomNameId> {
        let inner = self.borrow()?;
        inner
            .all_joined_rooms
            .get(room_id)
            .map(|jr| jr.room_name_id.clone())
            .or_else(|| {
                inner
                    .invited_rooms
                    .borrow()
                    .get(room_id)
                    .map(|ir| ir.room_name_id.clone())
            })
    }

    /// Returns room-management details for the given joined room, if loaded.
    pub fn get_room_context_menu_details(
        &self,
        room_id: &OwnedRoomId,
    ) -> Option<RoomContextMenuDetails> {
        let inner = self.borrow()?;
        let jr = inner.all_joined_rooms.get(room_id)?;
        Some(room_context_menu_details_from_joined_room(jr))
    }

    /// Returns the joined room matching the given loaded alias, if any.
    pub fn get_joined_room_name_by_alias(&self, room_alias: &str) -> Option<RoomNameId> {
        let inner = self.borrow()?;
        inner
            .all_joined_rooms
            .values()
            .find(|jr| {
                jr.canonical_alias
                    .as_ref()
                    .is_some_and(|alias| alias.as_str() == room_alias)
                    || jr
                        .alt_aliases
                        .iter()
                        .any(|alias| alias.as_str() == room_alias)
            })
            .map(|jr| jr.room_name_id.clone())
    }

    /// Returns the currently-selected space (the one selected in the SpacesBar).
    pub fn get_selected_space(&self) -> Option<RoomNameId> {
        self.borrow()?.selected_space.clone()
    }

    /// Same as [`Self::get_selected_space()`], but only returns the space ID.
    pub fn get_selected_space_id(&self) -> Option<OwnedRoomId> {
        self.borrow()?
            .selected_space
            .as_ref()
            .map(|ss| ss.room_id().clone())
    }

    /// Returns a clone of the space request sender channel, if available.
    ///
    /// This allows other widgets to submit space-related requests directly
    /// to the background space service.
    pub fn get_space_request_sender(&self) -> Option<UnboundedSender<SpaceRequest>> {
        self.borrow()?.space_request_sender.clone()
    }

    /// Returns the parent chain of the given space, if known.
    pub fn get_space_parent_chain(&self, space_id: &OwnedRoomId) -> Option<ParentChain> {
        self.borrow()?
            .space_map
            .get(space_id)
            .map(|smv| smv.parent_chain.clone())
    }
}

pub struct RoomsListScopeProps {
    /// Whether the RoomsList's inner PortalList was scrolling
    /// when the latest finger down event occurred.
    pub was_scrolling: bool,
}

/// The set of indexes for each room category in the the RoomsList's PortalList.
///
/// Each category's room count should be `after_rooms_index - first_room_index`.
#[derive(Clone, Copy, Debug, Default)]
struct RoomCategoryIndexes {
    /// The index of this room category's header, at which a `<CollapsibleHeader>` widget is displayed.
    ///
    /// This is an `Option` because the header is only shown if there are some rooms in this category.
    /// This is `Some` if the header should be shown (meaning there *are* rooms in this category),
    /// and `None` if the header should *not* be shown (meaning there are no rooms in this category).
    header_index: Option<usize>,
    /// The index of the first room in this category that appears immediately after the header.
    first_room_index: usize,
    /// The index after the last room in this category, which is where the next category should start.
    after_rooms_index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::room::FetchedRoomAvatar;
    use matrix_sdk::RoomDisplayName;

    fn test_room_name_id(room_id: &str) -> RoomNameId {
        RoomNameId::new(
            RoomDisplayName::Named("Test room".into()),
            OwnedRoomId::try_from(room_id).unwrap(),
        )
    }

    fn test_joined_room(
        room_id: &str,
        is_direct: bool,
        unread_messages: u64,
        unread_mentions: u64,
        is_marked_unread: bool,
    ) -> (OwnedRoomId, JoinedRoomInfo) {
        let room_name_id = test_room_name_id(room_id);
        let room_id = room_name_id.room_id().clone();
        (
            room_id,
            JoinedRoomInfo {
                room_name_id,
                num_unread_messages: unread_messages,
                num_unread_mentions: unread_mentions,
                is_marked_unread,
                canonical_alias: None,
                alt_aliases: Vec::new(),
                tags: Tags::default(),
                latest: None,
                room_avatar: FetchedRoomAvatar::default(),
                has_been_paginated: false,
                is_selected: false,
                is_direct,
                is_tombstoned: false,
            },
        )
    }

    #[test]
    fn rooms_list_header_toggle_updates_rendered_sections() {
        let mut invites = false;
        let mut direct = false;
        let mut regular = true;

        assert!(toggle_supported_header_category(
            HeaderCategory::Invites,
            &mut invites,
            &mut direct,
            &mut regular,
        ));
        assert!(invites);
        assert!(!direct);
        assert!(regular);

        assert!(toggle_supported_header_category(
            HeaderCategory::DirectRooms,
            &mut invites,
            &mut direct,
            &mut regular,
        ));
        assert!(invites);
        assert!(direct);
        assert!(regular);

        assert!(toggle_supported_header_category(
            HeaderCategory::RegularRooms,
            &mut invites,
            &mut direct,
            &mut regular,
        ));
        assert!(invites);
        assert!(direct);
        assert!(!regular);
    }

    #[test]
    fn rooms_list_header_toggle_ignores_unrendered_sections() {
        for category in [
            HeaderCategory::Favorites,
            HeaderCategory::LowPriority,
            HeaderCategory::LeftRooms,
            HeaderCategory::None,
        ] {
            let mut invites = true;
            let mut direct = false;
            let mut regular = true;

            assert!(!toggle_supported_header_category(
                category,
                &mut invites,
                &mut direct,
                &mut regular,
            ));
            assert!(invites, "{ROOMS_LIST_UNSUPPORTED_HEADER_TOGGLE_EVIDENCE}");
            assert!(!direct, "{ROOMS_LIST_UNSUPPORTED_HEADER_TOGGLE_EVIDENCE}");
            assert!(regular, "{ROOMS_LIST_UNSUPPORTED_HEADER_TOGGLE_EVIDENCE}");
        }
    }

    #[test]
    fn removed_room_active_selection_detection_matches_same_room() {
        let selected = SelectedRoom::JoinedRoom {
            room_name_id: test_room_name_id("!active:example.org"),
        };
        let removed_room_id = OwnedRoomId::try_from("!active:example.org").unwrap();

        assert!(removed_room_was_active_selection(
            Some(&selected),
            &removed_room_id,
        ));
    }

    #[test]
    fn removed_room_active_selection_detection_ignores_other_rooms() {
        let selected = SelectedRoom::InvitedRoom {
            room_name_id: test_room_name_id("!active:example.org"),
        };
        let removed_room_id = OwnedRoomId::try_from("!other:example.org").unwrap();

        assert!(!removed_room_was_active_selection(
            Some(&selected),
            &removed_room_id,
        ));
        assert!(!removed_room_was_active_selection(None, &removed_room_id,));
    }

    #[test]
    fn selected_room_removed_rejoin_packet_records_active_clear_without_membership_write() {
        let room_id = OwnedRoomId::try_from("!active:example.org").unwrap();
        let selected_space = test_room_name_id("!space:example.org");
        let label = selected_room_removed_rejoin_packet_label(
            &room_id,
            &RoomState::Left,
            true,
            Some(&selected_space),
        );

        assert!(label.contains("Selected-room removed/rejoin packet"));
        assert!(label.contains("membership_state Left"));
        assert!(label.contains("active_selection matched"));
        assert!(label.contains("FocusNone emitted"));
        assert!(label.contains("selected_space !space:example.org"));
        assert!(label.contains("replacement_ui_slot not_wired"));
        assert!(label.contains("rejoin_request_slot not_built"));
        assert!(label.contains("stale_event_policy"));
        assert!(label.contains("no JoinRoom, LeaveRoom, Knock"));
    }

    #[test]
    fn selected_room_removed_rejoin_packet_records_non_active_noop() {
        let room_id = OwnedRoomId::try_from("!other:example.org").unwrap();
        let label =
            selected_room_removed_rejoin_packet_label(&room_id, &RoomState::Banned, false, None);

        assert!(label.contains("membership_state Banned"));
        assert!(label.contains("active_selection not_matched"));
        assert!(label.contains("focus unchanged"));
        assert!(label.contains("selected_space none"));
        assert!(label.contains("membership request was sent"));
    }

    #[test]
    fn room_list_load_more_pagination_packet_records_missing_user_cursor_contract() {
        let label = room_list_load_more_pagination_packet_label(
            42,
            2,
            11,
            29,
            Some(256),
            "selected_space !space:example.org; selected_space_child_pagination service_read_sync_in_progress_or_pending; direct_child_rooms 7; direct_subspaces 3",
        );

        assert!(label.contains("Room-list Load More pagination packet"));
        assert!(label.contains("entries_with_dynamic_adapters(usize::MAX)"));
        assert!(label.contains("loaded_joined_rooms 42"));
        assert!(label.contains("displayed_invited 2"));
        assert!(label.contains("displayed_people 11"));
        assert!(label.contains("displayed_rooms 29"));
        assert!(label.contains("server_max_hint 256"));
        assert!(label.contains("selected_space !space:example.org"));
        assert!(
            label.contains(
                "selected_space_child_pagination service_read_sync_in_progress_or_pending"
            )
        );
        assert!(label.contains("load_more_button_slot not_rendered"));
        assert!(label.contains("explicit_cursor_slot not_exposed"));
        assert!(label.contains("error_slot SpaceRoomListAction_PaginationError_popup_only"));
        assert!(
            label.contains("latest_preview_pagination_source Matrix_PaginateTimeline_read_only")
        );
        assert!(label.contains("no user-triggered room-list pagination"));
        assert!(
            ROOMS_LIST_LOAD_MORE_PAGINATION_PACKET_EVIDENCE
                .contains("missing explicit load-more cursor/result/retry slots")
        );
    }

    #[test]
    fn section_unread_aggregate_packet_summarizes_loaded_rows_locally() {
        let mut rooms = HashMap::new();
        let (direct_id, direct_room) = test_joined_room("!direct:example.org", true, 4, 1, true);
        let (regular_id, regular_room) =
            test_joined_room("!regular:example.org", false, 7, 2, false);
        rooms.insert(direct_id.clone(), direct_room);
        rooms.insert(regular_id.clone(), regular_room);

        let missing_id = OwnedRoomId::try_from("!missing:example.org").unwrap();
        let direct = SectionUnreadAggregateSnapshot::from_displayed_rooms(
            &[direct_id.clone(), missing_id],
            &rooms,
        );
        let regular =
            SectionUnreadAggregateSnapshot::from_displayed_rooms(&[regular_id.clone()], &rooms);

        assert_eq!(
            direct,
            SectionUnreadAggregateSnapshot {
                room_count: 1,
                unread_messages: 4,
                unread_mentions: 1,
                marked_unread_count: 1,
            }
        );
        assert_eq!(
            regular,
            SectionUnreadAggregateSnapshot {
                room_count: 1,
                unread_messages: 7,
                unread_mentions: 2,
                marked_unread_count: 0,
            }
        );
    }

    #[test]
    fn section_unread_aggregate_packet_keeps_header_badges_local_zero() {
        let selected_space = test_room_name_id("!space:example.org");
        let label = section_unread_aggregate_packet_label(
            SectionUnreadAggregateSnapshot {
                room_count: 2,
                unread_messages: 5,
                unread_mentions: 1,
                marked_unread_count: 1,
            },
            SectionUnreadAggregateSnapshot {
                room_count: 3,
                unread_messages: 8,
                unread_mentions: 4,
                marked_unread_count: 2,
            },
            Some(&selected_space),
            true,
        );

        assert!(label.contains("People/Rooms unread/mention aggregate packet"));
        assert!(label.contains("people_loaded_rooms 2 unread 5 mentions 1 manual_unread 1"));
        assert!(label.contains("rooms_loaded_rooms 3 unread 8 mentions 4 manual_unread 2"));
        assert!(label.contains("selected_space !space:example.org"));
        assert!(label.contains("filter_active true"));
        assert!(label.contains("header_badge_source local_zero_placeholder"));
        assert!(label.contains("aggregate_refresh_slot not_built"));
        assert!(label.contains("parent_chain_attribution partial_cache_only"));
        assert!(label.contains("no aggregate scan, read receipt, message"));
        assert!(
            ROOMS_LIST_SECTION_UNREAD_AGGREGATE_LOCAL_ZERO_EVIDENCE
                .contains("header badges on local zero placeholders")
        );
    }
}
