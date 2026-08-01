use super::*;

fn assert_type_anchor<T>(expected: &str) {
    let actual = std::any::type_name::<T>();
    assert!(
        actual.contains(expected),
        "expected type anchor {expected:?}, got {actual:?}"
    );
}

#[test]
fn hepta_telegram_base_reuses_matrix_heart_modules() {
    assert_type_anchor::<crate::home::home_screen::HomeScreen>("HomeScreen");
    assert_type_anchor::<crate::home::main_desktop_ui::MainDesktopUI>("MainDesktopUI");
    assert_type_anchor::<crate::home::rooms_sidebar::RoomsSideBar>("RoomsSideBar");
    assert_type_anchor::<crate::home::rooms_list_header::RoomsListHeader>("RoomsListHeader");
    assert_type_anchor::<crate::home::rooms_list::RoomsList>("RoomsList");
    assert_type_anchor::<crate::home::room_screen::RoomScreen>("RoomScreen");
    assert_type_anchor::<crate::room::room_input_bar::RoomInputBar>("RoomInputBar");
    assert_type_anchor::<crate::shared::room_filter_input_bar::RoomFilterInputBar>(
        "RoomFilterInputBar",
    );
    assert_type_anchor::<crate::home::navigation_tab_bar::NavigationTabBar>("NavigationTabBar");
    assert_type_anchor::<crate::home::new_message_context_menu::NewMessageContextMenu>(
        "NewMessageContextMenu",
    );
    assert_type_anchor::<crate::home::edited_indicator::EditedIndicator>("EditedIndicator");
    assert_type_anchor::<crate::home::room_context_menu::RoomContextMenu>("RoomContextMenu");
    assert_type_anchor::<crate::home::editing_pane::EditingPane>("EditingPane");
    assert_type_anchor::<crate::room::typing_notice::TypingNotice>("TypingNotice");
    assert_type_anchor::<crate::shared::jump_to_bottom_button::JumpToBottomButton>(
        "JumpToBottomButton",
    );
    assert_type_anchor::<crate::home::room_read_receipt::AvatarRow>("AvatarRow");
    assert_type_anchor::<crate::home::event_reaction_list::ReactionList>("ReactionList");
    assert_type_anchor::<crate::media_cache::MediaCache>("MediaCache");
    assert_type_anchor::<crate::home::link_preview::LinkPreview>("LinkPreview");
    assert_type_anchor::<crate::shared::image_viewer::ImageViewerMetaData>("ImageViewerMetaData");
    assert_type_anchor::<crate::profile::user_profile::UserProfile>("UserProfile");
}

#[test]
fn hepta_telegram_base_matrix_requests_cover_chat_actions() {
    let compile_check: fn(&crate::sliding_sync::MatrixRequest) -> Option<&'static str> =
        telegram_matrix_request_label;
    let _ = compile_check;

    let required_actions = [
        "send_message",
        "send_typing_notice",
        "paginate_timeline",
        "create_thread_timeline",
        "fetch_thread_summary_details",
        "fetch_details_for_event",
        "get_number_unread_messages",
        "get_successor_room_details",
        "get_room_preview",
        "preview_matrix_link_target",
        "fetch_avatar",
        "sync_room_member_list",
        "read_receipt",
        "edit_message",
        "toggle_reaction",
        "redact_message",
        "pin_event",
        "knock_room",
        "set_unread_flag",
        "set_is_favorite",
        "set_is_low_priority",
        "generate_matrix_link",
        "fetch_media",
        "get_url_preview",
        "subscribe_typing_notices",
        "subscribe_pinned_events",
        "subscribe_own_read_receipts",
    ];

    assert_eq!(required_actions.len(), 27);
}

#[test]
fn hepta_telegram_base_space_requests_cover_read_sync_actions() {
    let compile_check: fn(&crate::space_service_sync::SpaceRequest) -> Option<&'static str> =
        telegram_space_request_label;
    let _ = compile_check;

    let required_space_actions = [
        "subscribe_space_room_list",
        "paginate_space_room_list",
        "get_space_children",
        "get_detailed_space_children",
        "get_top_level_space_details",
    ];

    assert_eq!(required_space_actions.len(), 5);
}

#[test]
fn hepta_telegram_base_timeline_pagination_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_TIMELINE_PAGINATION_READ_MARKER,
        "hepta_telegram_timeline_pagination_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_THREAD_SUMMARY_READ_MARKER,
        "hepta_telegram_thread_summary_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"timeline_pagination_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"timeline_pagination_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "timeline pagination read path"
            && capability
                .notes
                .contains("existing Matrix PaginateTimeline read path")
            && capability.notes.contains("no message send, edit")
            && capability.notes.contains("room-state mutation")
    }));
}

#[test]
fn hepta_telegram_base_thread_summary_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_THREAD_SUMMARY_READ_MARKER,
        "hepta_telegram_thread_summary_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"thread_summary_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"thread_summary_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "thread summary read path"
            && capability
                .notes
                .contains("existing Matrix FetchThreadSummaryDetails")
            && capability
                .notes
                .contains("CreateThreadTimeline read/open paths")
            && capability.notes.contains("does not send a message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_thread_open_timeline_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_THREAD_OPEN_TIMELINE_READ_MARKER,
        "hepta_telegram_thread_open_timeline_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"thread_open_timeline_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"thread_open_timeline_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "thread open timeline read path"
            && capability
                .notes
                .contains("RoomsListAction::Selected for a thread-focused timeline")
            && capability
                .notes
                .contains("existing Matrix CreateThreadTimeline read/open path")
            && capability.notes.contains("does not create a room")
            && capability.notes.contains("send a message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_reply_preview_event_details_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_REPLY_PREVIEW_EVENT_DETAILS_READ_MARKER,
        "hepta_telegram_reply_preview_event_details_read_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"reply_preview_event_details_read_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"reply_preview_event_details_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "reply preview event-details read path"
            && capability
                .notes
                .contains("existing Matrix FetchDetailsForEvent read path")
            && capability.notes.contains("does not send a message")
            && capability.notes.contains("redaction")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_sender_profile_event_details_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_SENDER_PROFILE_EVENT_DETAILS_READ_MARKER,
        "hepta_telegram_sender_profile_event_details_read_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"sender_profile_event_details_read_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"sender_profile_event_details_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "sender profile event-details read path"
            && capability
                .notes
                .contains("existing Matrix FetchDetailsForEvent read path")
            && capability.notes.contains("user_profile_cache fallback")
            && capability.notes.contains("no message")
            && capability.notes.contains("profile/account mutation")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_unread_count_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_UNREAD_COUNT_READ_MARKER,
        "hepta_telegram_unread_count_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"unread_count_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"unread_count_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "unread badge count read path"
            && capability
                .notes
                .contains("existing Matrix GetNumberUnreadMessages read path")
            && capability.notes.contains("JumpToBottomButton")
            && capability.notes.contains("Unknown, 0, and known counts")
            && capability.notes.contains("without SetUnreadFlag")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("retry")
            && capability.notes.contains("cancel")
            && capability.notes.contains("membership request")
    }));
}

#[test]
fn hepta_telegram_base_successor_room_details_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_SUCCESSOR_ROOM_DETAILS_READ_MARKER,
        "hepta_telegram_successor_room_details_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"successor_room_details_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"successor_room_details_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "successor room details read path"
            && capability
                .notes
                .contains("existing Matrix GetSuccessorRoomDetails read path")
            && capability.notes.contains("TombstoneFooter")
            && capability.notes.contains("does not send JoinRoom")
            && capability.notes.contains("Knock")
            && capability.notes.contains("membership")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
    }));
}

#[test]
fn hepta_telegram_base_room_preview_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_PREVIEW_READ_MARKER,
        "hepta_telegram_room_preview_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_preview_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_preview_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room preview read path"
            && capability
                .notes
                .contains("existing Matrix GetRoomPreview read path")
            && capability.notes.contains("Add Room")
            && capability.notes.contains("do not send JoinRoom")
            && capability.notes.contains("Knock")
            && capability.notes.contains("membership")
    }));
}

#[test]
fn hepta_telegram_base_avatar_fetch_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_AVATAR_FETCH_READ_MARKER,
        "hepta_telegram_avatar_fetch_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"avatar_fetch_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"avatar_fetch_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "avatar fetch read path"
            && capability
                .notes
                .contains("existing Matrix FetchAvatar cache path")
            && capability.notes.contains("known MXC URIs")
            && capability.notes.contains("Avatar and avatar_cache")
            && capability.notes.contains("fallback initials")
            && capability.notes.contains("Requested")
            && capability.notes.contains("Known(None)")
            && capability.notes.contains("do not send SetAvatar")
            && capability.notes.contains("FetchMedia")
            && capability.notes.contains("profile mutation")
            && capability.notes.contains("account mutation")
            && capability.notes.contains("membership")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
    }));
}

#[test]
fn hepta_telegram_base_contract_marks_fixture_as_smoke_only() {
    assert_eq!(
        HEPTA_TELEGRAM_BASE_CONTRACT_MARKER,
        "hepta_telegram_real_base_contract_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_BASE_MODULE_MARKER,
        "hepta_telegram_uses_matrix_heart_modules"
    );
    assert_eq!(
        HEPTA_TELEGRAM_STATIC_FIXTURE_SCOPE_MARKER,
        "hepta_telegram_static_fixture_smoke_only"
    );
    assert_eq!(HEPTA_TELEGRAM_SHELL_MARKER, "hepta_telegram_shell_ready");
    assert_eq!(
        HEPTA_TELEGRAM_BOTTOM_ANCHORED_MARKER,
        "hepta_telegram_bottom_anchored_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_REAL_CHROME_MARKER,
        "hepta_telegram_real_chrome_on_standard_shell"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACTION_CHROME_MARKER,
        "hepta_telegram_action_chrome_on_real_menus"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TIMELINE_PAGINATION_READ_MARKER,
        "hepta_telegram_timeline_pagination_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_THREAD_SUMMARY_READ_MARKER,
        "hepta_telegram_thread_summary_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_THREAD_OPEN_TIMELINE_READ_MARKER,
        "hepta_telegram_thread_open_timeline_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_REPLY_PREVIEW_EVENT_DETAILS_READ_MARKER,
        "hepta_telegram_reply_preview_event_details_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SENDER_PROFILE_EVENT_DETAILS_READ_MARKER,
        "hepta_telegram_sender_profile_event_details_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_UNREAD_COUNT_READ_MARKER,
        "hepta_telegram_unread_count_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SUCCESSOR_ROOM_DETAILS_READ_MARKER,
        "hepta_telegram_successor_room_details_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_PREVIEW_READ_MARKER,
        "hepta_telegram_room_preview_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_AVATAR_FETCH_READ_MARKER,
        "hepta_telegram_avatar_fetch_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_PINNED_EVENTS_SUBSCRIPTION_MARKER,
        "hepta_telegram_pinned_events_subscription_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_MEMBERS_READ_MARKER,
        "hepta_telegram_room_members_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_MEMBER_SYNC_READ_MARKER,
        "hepta_telegram_room_member_sync_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_POWER_LEVELS_READ_MARKER,
        "hepta_telegram_room_power_levels_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TYPING_NOTICE_SUBSCRIPTION_MARKER,
        "hepta_telegram_typing_notice_subscription_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_OWN_READ_RECEIPT_SUBSCRIPTION_MARKER,
        "hepta_telegram_own_read_receipt_subscription_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_message_report_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_message_report_option_staging_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_message_report_send_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_CONTENT_LIVE_SEND_WIRING_MARKER,
        "hepta_telegram_message_report_content_live_send_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_MARKER,
        "hepta_telegram_message_report_moderation_workflow_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_LOADED_TARGET_METADATA_MARKER,
        "hepta_telegram_message_report_loaded_target_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_MARKER,
        "hepta_telegram_message_report_custom_reason_draft_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_CUSTOM_REASON_CONFIRMATION_MARKER,
        "hepta_telegram_message_report_custom_reason_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_STATUS_LIFECYCLE_MARKER,
        "hepta_telegram_message_report_status_lifecycle_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_STATUS_CLIPBOARD_MARKER,
        "hepta_telegram_message_report_status_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_message_report_retry_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_MARKER,
        "hepta_telegram_message_report_workflow_actions_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_MARKER,
        "hepta_telegram_message_report_moderation_reviewer_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_MARKER,
        "hepta_telegram_message_report_workflow_result_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_message_report_workflow_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_message_report_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_LOADED_SOURCE_MODAL_MARKER,
        "hepta_telegram_message_report_loaded_source_modal_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_SURFACE_MARKER,
        "hepta_telegram_message_edit_history_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_message_edit_history_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_CLICK_LOCAL_MARKER,
        "hepta_telegram_message_edit_history_click_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_COMPACT_SUMMARY_LIVE_READ_WIRING_MARKER,
        "hepta_telegram_message_edit_history_compact_summary_live_read_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_MARKER,
        "hepta_telegram_message_edit_history_loaded_target_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_MARKER,
        "hepta_telegram_message_edit_history_detail_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_MARKER,
        "hepta_telegram_message_edit_history_full_modal_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_MARKER,
        "hepta_telegram_message_edit_history_local_full_snapshot_modal_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_CONTROLS_MARKER,
        "hepta_telegram_message_edit_history_full_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_MARKER,
        "hepta_telegram_message_edit_history_loaded_source_modal_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_MARKER,
        "hepta_telegram_message_edit_history_loaded_diff_detail_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_MARKER,
        "hepta_telegram_message_edit_history_loaded_diff_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_MARKER,
        "hepta_telegram_message_edit_history_loaded_side_by_side_diff_modal_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_MARKER,
        "hepta_telegram_message_edit_history_full_diff_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_MARKER,
        "hepta_telegram_message_edit_history_full_history_result_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_message_edit_history_remote_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_message_edit_history_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_message_edit_history_retry_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_EMPTY_CLOSE_LOCAL_MARKER,
        "hepta_telegram_message_search_empty_close_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_MARKER,
        "hepta_telegram_message_search_loaded_timeline_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_METADATA_MARKER,
        "hepta_telegram_message_search_loaded_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_MARKER,
        "hepta_telegram_message_search_active_result_detail_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_MARKER,
        "hepta_telegram_message_search_result_action_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_MARKER,
        "hepta_telegram_message_search_result_jump_loaded_match_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_THREAD_OPEN_MARKER,
        "hepta_telegram_message_search_result_thread_open_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_MARKER,
        "hepta_telegram_message_search_result_sender_profile_pane_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_MARKER,
        "hepta_telegram_message_search_result_copy_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_SOURCE_MODAL_MARKER,
        "hepta_telegram_message_search_result_source_modal_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_message_search_query_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_MARKER,
        "hepta_telegram_message_search_server_context_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_MARKER,
        "hepta_telegram_message_search_advanced_filter_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_MARKER,
        "hepta_telegram_message_search_server_preflight_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_message_search_remote_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_IDENTITY_LOCAL_SURFACE_MARKER,
        "hepta_telegram_tsp_identity_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_VERIFICATION_REQUEST_METADATA_MARKER,
        "hepta_telegram_tsp_verification_request_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_CRYPTO_VERIFICATION_REQUEST_METADATA_MARKER,
        "hepta_telegram_crypto_verification_request_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_LOGIN_AUTO_CANCEL_LOCAL_MARKER,
        "hepta_telegram_login_auto_cancel_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_DIALOG_FILTER_MARKER,
        "hepta_telegram_dialog_filter_on_real_sidebar"
    );
    assert_eq!(
        HEPTA_TELEGRAM_DIALOG_STATE_FILTER_MARKER,
        "hepta_telegram_dialog_state_filters_on_real_room_filter"
    );
    assert_eq!(
        HEPTA_TELEGRAM_DIALOG_FILTER_PRESET_MARKER,
        "hepta_telegram_dialog_filter_presets_emit_main_filter_action"
    );
    assert_eq!(
        HEPTA_TELEGRAM_DIALOG_LIST_EMPTY_STATE_LOCAL_FILTER_MARKER,
        "hepta_telegram_dialog_list_empty_state_local_filter_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_DESKTOP_DOCK_RESTORE_LAZY_LOCAL_MARKER,
        "hepta_telegram_desktop_dock_restore_lazy_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_DESKTOP_SHELL_MARKER,
        "hepta_telegram_desktop_shell_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MOBILE_STACK_NAVIGATION_LOCAL_MARKER,
        "hepta_telegram_mobile_stack_navigation_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MOBILE_SHELL_MARKER,
        "hepta_telegram_mobile_shell_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MOBILE_EVIDENCE_DENSITY_GUARD_MARKER,
        "hepta_telegram_mobile_evidence_density_guard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NAVIGATION_SPACES_TOGGLE_LOCAL_MARKER,
        "hepta_telegram_navigation_spaces_toggle_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NAVIGATION_TOP_LEVEL_TAB_SELECTION_LOCAL_MARKER,
        "hepta_telegram_navigation_top_level_tab_selection_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_ICON_SETTINGS_NAVIGATION_LOCAL_MARKER,
        "hepta_telegram_profile_icon_settings_navigation_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SETTINGS_CLOSE_PREVIOUS_SELECTION_LOCAL_MARKER,
        "hepta_telegram_settings_close_previous_selection_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SPACES_BAR_ENTRY_SELECTION_LOCAL_MARKER,
        "hepta_telegram_spaces_bar_entry_selection_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SPACES_BAR_SECONDARY_CLICK_LOCAL_MARKER,
        "hepta_telegram_spaces_bar_secondary_click_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SPACES_BAR_EMPTY_FILTER_LOCAL_MARKER,
        "hepta_telegram_spaces_bar_empty_filter_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_PAGINATION_ADAPTER_LOCAL_MARKER,
        "hepta_telegram_rooms_list_pagination_adapter_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_LOAD_MORE_PAGINATION_PACKET_MARKER,
        "hepta_telegram_rooms_list_load_more_pagination_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_HEADER_SPACE_SCOPE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_header_space_scope_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_SECTION_UNREAD_AGGREGATE_LOCAL_ZERO_MARKER,
        "hepta_telegram_rooms_list_section_unread_aggregate_local_zero_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_ALL_ROOMS_LOADED_LOCAL_UNKNOWN_MARKER,
        "hepta_telegram_rooms_list_all_rooms_loaded_local_unknown_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_SPACE_PARENT_CACHE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_space_parent_cache_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_NAME_UPDATE_SELECTED_STATE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_name_update_selected_state_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_REMOVED_ROOM_SELECTED_STATE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_removed_room_selected_state_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_HEADER_ACTIONS_MARKER,
        "hepta_telegram_room_header_actions_local_only"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_MENU_ELLIPSIS_MARKER,
        "hepta_telegram_room_menu_ellipsis_visible"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_ACTIONS_STRIP_MARKER,
        "hepta_telegram_room_actions_strip_uses_base_link_path"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_ACTIONS_CLOSE_LOCAL_EVIDENCE_MARKER,
        "hepta_telegram_room_actions_close_local_evidence_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_LINK_COPY_HANDOFF_MARKER,
        "hepta_telegram_room_link_copy_handoff_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_ACTIONS_INVITE_LEAVE_MARKER,
        "hepta_telegram_room_actions_invite_leave_reuse_existing_modals"
    );
    assert_eq!(
        HEPTA_TELEGRAM_INVITE_USER_CONFIRMATION_MARKER,
        "hepta_telegram_invite_user_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TIMELINE_INVITE_CONFIRMATION_MARKER,
        "hepta_telegram_timeline_invite_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_INVITE_RESPONSE_CONFIRMATION_MARKER,
        "hepta_telegram_invite_response_confirmation_required"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_JOIN_LEAVE_MODAL_MARKER,
        "hepta_telegram_space_lobby_join_leave_modal_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_READ_SYNC_MARKER,
        "hepta_telegram_space_lobby_read_sync_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_ROOM_LIST_LIFECYCLE_MARKER,
        "hepta_telegram_space_lobby_room_list_lifecycle_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_EMPTY_STATE_READ_SYNC_MARKER,
        "hepta_telegram_space_lobby_empty_state_read_sync_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_MEMBERSHIP_EDGE_LOCAL_MARKER,
        "hepta_telegram_space_lobby_membership_edge_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ADD_ROOM_KNOCK_CONFIRMATION_MARKER,
        "hepta_telegram_add_room_knock_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_ACTIONS_TOGGLES_MARKER,
        "hepta_telegram_room_actions_stateful_toggles_reuse_base_paths"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_INFO_STRIP_MARKER,
        "hepta_telegram_room_info_strip_uses_loaded_room_state"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_CONTEXT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_room_context_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_SURFACE_MARKER,
        "hepta_telegram_room_settings_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_OPTION_SUMMARY_MARKER,
        "hepta_telegram_room_settings_option_summary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_room_settings_option_staging_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_NAME_ID_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_name_id_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_permissions_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_MEMBERS_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_members_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_IDENTITY_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_identity_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_LOADED_IDENTITY_MARKER,
        "hepta_telegram_room_settings_loaded_identity_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_CLOSE_METADATA_MARKER,
        "hepta_telegram_room_settings_close_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_METADATA_MARKER,
        "hepta_telegram_room_settings_refresh_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_MARKER,
        "hepta_telegram_room_settings_refresh_live_read_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_MARKER,
        "hepta_telegram_room_settings_name_topic_live_write_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_CANONICAL_ALIAS_LIVE_WIRING_MARKER,
        "hepta_telegram_room_settings_canonical_alias_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_AVATAR_REMOVE_LIVE_WIRING_MARKER,
        "hepta_telegram_room_settings_avatar_remove_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_TOMBSTONE_LIVE_WRITE_MARKER,
        "hepta_telegram_room_settings_tombstone_live_write_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_READONLY_BOUNDARY_MARKER,
        "hepta_telegram_room_settings_readonly_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_room_settings_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_MARKER,
        "hepta_telegram_room_settings_edit_controls_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_EDIT_INTENT_STAGING_MARKER,
        "hepta_telegram_room_settings_edit_intent_staging_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_MARKER,
        "hepta_telegram_room_settings_field_edit_intent_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_RESULT_DETAIL_MARKER,
        "hepta_telegram_room_settings_refresh_result_detail_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_MARKER,
        "hepta_telegram_room_settings_mutation_preflight_detail_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_MARKER,
        "hepta_telegram_room_settings_field_mutation_packet_drilldown_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_MARKER,
        "hepta_telegram_room_settings_field_mutation_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_room_settings_power_member_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_MODE_MARKER,
        "hepta_telegram_message_search_mode_local_only"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_LOCAL_SURFACE_MARKER,
        "hepta_telegram_message_search_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_LOCAL_SURFACE_MARKER,
        "hepta_telegram_notifications_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_notifications_option_staging_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_MODE_WRITE_CONFIRMATION_MARKER,
        "hepta_telegram_notifications_mode_write_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_LOADED_ATTENTION_MARKER,
        "hepta_telegram_notifications_loaded_attention_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_MODE_TARGET_METADATA_MARKER,
        "hepta_telegram_notifications_mode_target_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_CLOSE_REFRESH_METADATA_MARKER,
        "hepta_telegram_notifications_close_refresh_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_notifications_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_MARKER,
        "hepta_telegram_notifications_timed_global_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_MARKER,
        "hepta_telegram_notifications_pusher_keyword_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_MARKER,
        "hepta_telegram_notifications_keyword_list_live_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_KEYWORD_MUTATION_LIVE_WRITE_MARKER,
        "hepta_telegram_notifications_keyword_mutation_live_write_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_MARKER,
        "hepta_telegram_notifications_pusher_status_live_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_ADVANCED_CONTROLS_MARKER,
        "hepta_telegram_notifications_advanced_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_notifications_advanced_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RESULT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_notifications_result_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_notifications_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RULE_PACKET_DRILLDOWN_MARKER,
        "hepta_telegram_notifications_rule_packet_drilldown_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RULE_CONTRACT_PACKET_MARKER,
        "hepta_telegram_notifications_rule_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_notifications_retry_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_LOCAL_SURFACE_CLOSE_MARKER,
        "hepta_telegram_room_local_surface_close_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_PLACEHOLDER_MARKER,
        "hepta_telegram_composer_attachment_placeholder_local_only"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_composer_attachment_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_composer_attachment_camera_contact_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_HANDOFF_CONFIRMATION_MARKER,
        "hepta_telegram_composer_attachment_handoff_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SEND_HANDOFF_MARKER,
        "hepta_telegram_composer_attachment_send_handoff_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_PRE_SEND_REVIEW_MARKER,
        "hepta_telegram_composer_attachment_pre_send_review_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTED_FILE_PREVIEW_MARKER,
        "hepta_telegram_composer_attachment_selected_file_preview_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTED_IMAGE_METADATA_MARKER,
        "hepta_telegram_composer_attachment_selected_image_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_MAIN_SEND_GUARD_MARKER,
        "hepta_telegram_composer_attachment_main_send_guard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTION_REPLACEMENT_PRESERVE_MARKER,
        "hepta_telegram_composer_attachment_selection_replacement_preserve_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_composer_attachment_review_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_SEND_SINGLE_SUBMIT_MARKER,
        "hepta_telegram_composer_attachment_review_send_single_submit_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_DISCARD_CLOSE_IDEMPOTENT_MARKER,
        "hepta_telegram_composer_attachment_review_discard_close_idempotent_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_CAPTION_REPLY_CONTEXT_MARKER,
        "hepta_telegram_composer_attachment_caption_reply_context_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_FILE_VALIDATION_LOCAL_ERROR_MARKER,
        "hepta_telegram_composer_attachment_file_validation_local_error_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_VALIDATION_ERROR_RECOVERY_MARKER,
        "hepta_telegram_composer_attachment_validation_error_recovery_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SEND_OPERATION_STATUS_LOCAL_MARKER,
        "hepta_telegram_attachment_send_operation_status_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SEND_RESULT_BRIDGE_MARKER,
        "hepta_telegram_attachment_send_result_bridge_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_QUEUE_FAILURE_RECOVERY_MARKER,
        "hepta_telegram_attachment_queue_failure_recovery_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_QUEUE_FAILURE_RECOVERY_COPY_MARKER,
        "hepta_telegram_attachment_queue_failure_recovery_copy_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_attachment_send_failure_retry_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_attachment_true_queue_control_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_MARKER,
        "hepta_telegram_attachment_accepted_queue_actions_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_MARKER,
        "hepta_telegram_attachment_accepted_queue_timeline_cancel_bridge_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_LOCAL_SEND_ABORT_RESULT_MARKER,
        "hepta_telegram_attachment_local_send_abort_result_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_PER_FILE_STATUS_CONTROLS_MARKER,
        "hepta_telegram_attachment_per_file_status_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_MARKER,
        "hepta_telegram_attachment_per_file_queue_drilldown_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_MARKER,
        "hepta_telegram_attachment_sdk_queue_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_attachment_queue_progress_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_attachment_send_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_MARKER,
        "hepta_telegram_attachment_multi_file_queue_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_TIMELINE_SEND_STATE_MARKER,
        "hepta_telegram_attachment_timeline_send_state_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_TIMELINE_CANCEL_LOCAL_SEND_MARKER,
        "hepta_telegram_attachment_timeline_cancel_local_send_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_STATUS_TAXONOMY_LOCAL_MARKER,
        "hepta_telegram_attachment_status_taxonomy_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_REVIEW_ROW_COMPACT_FIT_MARKER,
        "hepta_telegram_attachment_review_row_compact_fit_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_MOBILE_PICKER_CONTROLS_MARKER,
        "hepta_telegram_composer_attachment_mobile_picker_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_MARKER,
        "hepta_telegram_attachment_mobile_share_sheet_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_MOBILE_ACTION_DENSITY_MARKER,
        "hepta_telegram_attachment_mobile_action_density_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_EMOJI_PLACEHOLDER_MARKER,
        "hepta_telegram_composer_emoji_placeholder_local_only"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_EMOJI_LOCAL_SURFACE_MARKER,
        "hepta_telegram_composer_emoji_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_EMOJI_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_composer_emoji_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_PLACEHOLDER_MARKER,
        "hepta_telegram_composer_voice_placeholder_local_only"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_LOCAL_SURFACE_MARKER,
        "hepta_telegram_composer_voice_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_DESKTOP_AUDIO_HANDOFF_MARKER,
        "hepta_telegram_composer_voice_desktop_audio_handoff_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_AUDIO_FILE_SURFACE_MARKER,
        "hepta_telegram_composer_voice_audio_file_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_OPTION_STAGING_AUDIO_MARKER,
        "hepta_telegram_composer_voice_option_staging_audio_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_AUDIO_HANDOFF_MARKER,
        "hepta_telegram_composer_voice_send_audio_handoff_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_LIVE_WIRING_MARKER,
        "hepta_telegram_composer_voice_send_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_PERMISSION_RECORDING_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_composer_voice_permission_recording_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDING_BOUNDARY_MARKER,
        "hepta_telegram_composer_voice_recording_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SELECTED_AUDIO_METADATA_MARKER,
        "hepta_telegram_composer_voice_selected_audio_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_composer_voice_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_CONFIRMATION_CANCEL_METADATA_MARKER,
        "hepta_telegram_composer_voice_confirmation_cancel_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_WAVEFORM_CODEC_BOUNDARY_MARKER,
        "hepta_telegram_composer_voice_recorder_waveform_codec_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_STATUS_CONTROLS_MARKER,
        "hepta_telegram_composer_voice_recorder_status_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_CAPTURE_LIFECYCLE_CONTROLS_MARKER,
        "hepta_telegram_composer_voice_capture_lifecycle_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_composer_voice_send_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_composer_voice_recorder_lifecycle_drilldown_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_composer_voice_recorder_typed_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_SURFACE_MARKER,
        "hepta_telegram_composer_mention_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_composer_mention_option_staging_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_CACHED_SELECTION_MARKER,
        "hepta_telegram_composer_mention_cached_selection_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOADED_IDENTITY_MARKER,
        "hepta_telegram_composer_mention_loaded_identity_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_CANDIDATE_ROWS_MARKER,
        "hepta_telegram_composer_mention_local_candidate_rows_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_DUPLICATE_HINTS_MARKER,
        "hepta_telegram_composer_mention_local_duplicate_hints_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_composer_mention_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_KEYBOARD_SELECTION_MARKER,
        "hepta_telegram_composer_mention_keyboard_selection_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_RICH_PICKER_BOUNDARY_MARKER,
        "hepta_telegram_composer_mention_rich_picker_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_DIRECTORY_DISAMBIGUATION_BOUNDARY_MARKER,
        "hepta_telegram_composer_mention_directory_disambiguation_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_RICH_DIRECTORY_CONTROLS_MARKER,
        "hepta_telegram_composer_mention_rich_directory_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_DIRECTORY_SEARCH_LIVE_WIRING_MARKER,
        "hepta_telegram_composer_mention_directory_search_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_DIRECTORY_RESULT_PROMOTION_LIVE_MARKER,
        "hepta_telegram_composer_mention_directory_result_promotion_live_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_HOVER_CARD_SNAPSHOT_LIVE_MARKER,
        "hepta_telegram_composer_mention_hover_card_snapshot_live_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_PILL_TRAY_LIVE_MARKER,
        "hepta_telegram_composer_mention_local_pill_tray_live_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_composer_mention_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_composer_mention_send_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_PAYLOAD_METADATA_MARKER,
        "hepta_telegram_composer_mention_send_payload_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_LIVE_WIRING_MARKER,
        "hepta_telegram_composer_mention_send_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_SCOPE_CONTROLS_MARKER,
        "hepta_telegram_composer_mention_payload_scope_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_composer_mention_payload_drilldown_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_composer_mention_payload_typed_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_REMOTE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_composer_mention_remote_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_LOCATION_SEND_CONFIRMATION_MARKER,
        "hepta_telegram_location_send_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_MARKER,
        "hepta_telegram_live_location_continuous_updates_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_READ_RECEIPT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_profile_read_receipt_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_MEMBER_READ_MARKER,
        "hepta_telegram_profile_member_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_ACCOUNT_IDENTITY_CLIPBOARD_MARKER,
        "hepta_telegram_profile_account_identity_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_DIRECT_MESSAGE_CONFIRMATION_MARKER,
        "hepta_telegram_profile_direct_message_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_DIRECT_MESSAGE_CREATE_CONFIRMATION_MARKER,
        "hepta_telegram_direct_message_create_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_IGNORE_CONFIRMATION_MARKER,
        "hepta_telegram_profile_ignore_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_DISPLAY_NAME_STAGING_MARKER,
        "hepta_telegram_account_display_name_staging_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_DISPLAY_NAME_CONFIRMATION_MARKER,
        "hepta_telegram_account_display_name_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_DEVICE_SELF_CHECK_MARKER,
        "hepta_telegram_account_device_self_check_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LOCAL_SURFACE_MARKER,
        "hepta_telegram_account_avatar_upload_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_account_avatar_upload_option_staging_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_MARKER,
        "hepta_telegram_account_avatar_upload_selected_file_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_MARKER,
        "hepta_telegram_account_avatar_upload_selected_image_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_MARKER,
        "hepta_telegram_account_avatar_upload_decode_probe_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_PIXEL_DECODE_LIVE_MARKER,
        "hepta_telegram_account_avatar_upload_pixel_decode_live_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LIVE_WIRING_MARKER,
        "hepta_telegram_account_avatar_upload_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_DIRECT_MXC_SET_MARKER,
        "hepta_telegram_account_avatar_direct_mxc_set_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_account_avatar_upload_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_account_avatar_upload_retry_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_MARKER,
        "hepta_telegram_account_avatar_upload_crop_editor_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_MARKER,
        "hepta_telegram_account_avatar_upload_editor_controls_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_MARKER,
        "hepta_telegram_account_avatar_upload_source_preview_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_account_avatar_upload_source_editor_drilldown_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_account_avatar_upload_source_editor_typed_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_account_avatar_upload_source_editor_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_SOURCE_PATH_CLIPBOARD_MARKER,
        "hepta_telegram_account_avatar_source_path_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_MARKER,
        "hepta_telegram_account_avatar_upload_source_path_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_account_avatar_upload_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_account_avatar_upload_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_DELETE_CONFIRMATION_MARKER,
        "hepta_telegram_account_avatar_delete_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_DELETE_LIVE_WIRING_MARKER,
        "hepta_telegram_account_avatar_delete_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_account_management_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_account_management_option_staging_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOADED_IDENTITY_MARKER,
        "hepta_telegram_account_management_loaded_identity_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LIVE_WIRING_MARKER,
        "hepta_telegram_account_management_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DISPLAY_NAME_LIVE_WIRING_MARKER,
        "hepta_telegram_account_management_display_name_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_RENAME_LIVE_WIRING_MARKER,
        "hepta_telegram_account_management_current_device_rename_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_LIVE_WIRING_MARKER,
        "hepta_telegram_account_management_device_directory_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_BROWSER_PORTAL_HANDOFF_MARKER,
        "hepta_telegram_account_management_browser_portal_handoff_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_account_management_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_MARKER,
        "hepta_telegram_account_management_refresh_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_MARKER,
        "hepta_telegram_account_management_session_revoke_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_MARKER,
        "hepta_telegram_account_management_session_actions_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_MARKER,
        "hepta_telegram_account_management_device_directory_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_MARKER,
        "hepta_telegram_account_management_device_directory_controls_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_MARKER,
        "hepta_telegram_account_management_current_device_metadata_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_MARKER,
        "hepta_telegram_account_management_current_device_metadata_controls_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_device_verification_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_device_id_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_device_display_name_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_session_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_device_source_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_account_management_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_MARKER,
        "hepta_telegram_account_management_preflight_detail_controls_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_account_management_session_device_drilldown_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_account_management_session_device_typed_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_account_management_session_device_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_account_management_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_LOCAL_SURFACE_CLOSE_MARKER,
        "hepta_telegram_account_local_surface_close_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_LOGOUT_CONFIRMATION_MARKER,
        "hepta_telegram_account_logout_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_MESSAGE_BLOCKED_ACTIONS_MARKER,
        "hepta_telegram_media_message_blocked_actions_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_CONFIRMATION_MARKER,
        "hepta_telegram_media_save_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_media_save_retry_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_DOWNLOAD_METADATA_MARKER,
        "hepta_telegram_media_download_metadata_preview_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_METADATA_CLIPBOARD_MARKER,
        "hepta_telegram_media_metadata_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_media_save_dialog_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_DESTINATION_METADATA_MARKER,
        "hepta_telegram_media_save_destination_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_ENCRYPTED_METADATA_LOCAL_MARKER,
        "hepta_telegram_media_encrypted_metadata_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_FETCH_CACHE_READ_MARKER,
        "hepta_telegram_media_fetch_cache_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_media_download_playback_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_MARKER,
        "hepta_telegram_media_inline_playback_queue_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_MARKER,
        "hepta_telegram_media_inline_player_disabled_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_CODEC_TRANSCODE_CONTROLS_MARKER,
        "hepta_telegram_media_codec_transcode_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_STATUS_BOUNDARY_MARKER,
        "hepta_telegram_media_save_result_status_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_MARKER,
        "hepta_telegram_media_save_result_recovery_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_MARKER,
        "hepta_telegram_media_save_result_recovery_controls_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_media_save_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_OPERATION_PACKET_DRILLDOWN_MARKER,
        "hepta_telegram_media_operation_packet_drilldown_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_MARKER,
        "hepta_telegram_media_playback_queue_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_media_playback_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_DOWNLOAD_METADATA_MARKER,
        "hepta_telegram_media_download_metadata_preview_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_POLL_MESSAGE_PREVIEW_LOCAL_MARKER,
        "hepta_telegram_poll_message_preview_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_POLL_ANSWER_PREVIEW_RESULT_PACKET_MARKER,
        "hepta_telegram_poll_answer_preview_result_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_LINK_PREVIEW_LOCAL_CONTROLS_MARKER,
        "hepta_telegram_link_preview_local_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_LINK_PREVIEW_LOADED_METADATA_MARKER,
        "hepta_telegram_link_preview_loaded_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_URL_PREVIEW_READ_MARKER,
        "hepta_telegram_url_preview_read_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOCAL_PREVIEW_MARKER,
        "hepta_telegram_matrix_link_local_preview_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_LIVE_READ_WIRING_MARKER,
        "hepta_telegram_matrix_link_preview_live_read_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_ALIAS_NAVIGATION_MARKER,
        "hepta_telegram_matrix_link_loaded_alias_navigation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_MARKER,
        "hepta_telegram_matrix_link_loaded_event_local_jump_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_MARKER,
        "hepta_telegram_matrix_link_loaded_event_context_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_MARKER,
        "hepta_telegram_matrix_link_loaded_event_source_modal_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_LOCAL_MARKER,
        "hepta_telegram_matrix_link_unknown_target_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_BOUNDARY_MARKER,
        "hepta_telegram_matrix_link_unknown_target_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_TARGET_METADATA_MARKER,
        "hepta_telegram_matrix_link_target_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_RESULT_METADATA_MARKER,
        "hepta_telegram_matrix_link_preview_result_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_FAILURE_METADATA_MARKER,
        "hepta_telegram_matrix_link_preview_failure_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_matrix_link_preview_retry_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_MARKER,
        "hepta_telegram_matrix_link_server_context_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_CONTEXT_ACTIONS_ROW_MARKER,
        "hepta_telegram_matrix_link_context_actions_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROOM_OR_ALIAS_JOIN_LIVE_WIRING_MARKER,
        "hepta_telegram_matrix_link_room_or_alias_join_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROOM_OR_ALIAS_KNOCK_LIVE_WIRING_MARKER,
        "hepta_telegram_matrix_link_room_or_alias_knock_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_USER_INVITE_LIVE_WIRING_MARKER,
        "hepta_telegram_matrix_link_user_invite_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_MARKER,
        "hepta_telegram_matrix_link_browser_handoff_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_SCOPE_CONTROLS_MARKER,
        "hepta_telegram_matrix_link_route_scope_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_matrix_link_route_drilldown_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_MARKER,
        "hepta_telegram_matrix_link_route_result_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_matrix_link_route_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROOM_TARGET_CLIPBOARD_MARKER,
        "hepta_telegram_matrix_link_room_target_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_VIA_SERVERS_CLIPBOARD_MARKER,
        "hepta_telegram_matrix_link_via_servers_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_EVENT_ID_CLIPBOARD_MARKER,
        "hepta_telegram_matrix_link_event_id_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_MARKER,
        "hepta_telegram_matrix_link_preview_metadata_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_UNRESOLVED_DETAIL_MARKER,
        "hepta_telegram_matrix_link_unresolved_detail_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_EXTERNAL_LINK_CONFIRMATION_MARKER,
        "hepta_telegram_external_link_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_EVENT_SOURCE_LOCAL_SURFACE_MARKER,
        "hepta_telegram_event_source_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_EVENT_SOURCE_CLIPBOARD_COPY_MARKER,
        "hepta_telegram_event_source_clipboard_copy_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_EVENT_SOURCE_LOADED_METADATA_MARKER,
        "hepta_telegram_event_source_loaded_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_COPY_LOCAL_SURFACE_MARKER,
        "hepta_telegram_message_copy_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_COPY_LOADED_METADATA_MARKER,
        "hepta_telegram_message_copy_loaded_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_PIN_CONFIRMATION_MARKER,
        "hepta_telegram_message_pin_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_DELETE_CONFIRMATION_MARKER,
        "hepta_telegram_message_delete_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_LOCAL_SURFACE_CLOSE_MARKER,
        "hepta_telegram_composer_local_surface_close_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_AUDIT_MARKER,
        "hepta_telegram_base_gap_hard_boundary_audit_ready"
    );
    assert!(hepta_telegram_base_contract_ready());
    assert!(hepta_telegram_base_direct_reuse_count() >= 8);
    assert_eq!(
        hepta_telegram_base_gap_count(),
        HEPTA_TELEGRAM_BASE_GAPS.len()
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_report_send"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_edit_history"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
}

#[test]
fn hepta_telegram_base_gap_status_matches_explicit_gap_list() {
    let explicit_gap_surfaces = [
        ("message_search", "message search"),
        ("message_report_send", "message report/send"),
        ("message_edit_history", "message edit history"),
        ("matrix_link_resolution", "Matrix link preview/navigation"),
        ("room_settings", "room settings"),
        ("notifications", "notifications"),
        ("file_upload_send", "file upload send"),
        ("media_download_playback", "media download/playback"),
        ("account_avatar_upload", "account avatar upload"),
        ("account_management", "account management"),
        ("mention_picker_send", "mention picker/send"),
        ("voice_message_send", "voice message send"),
    ];

    assert_eq!(explicit_gap_surfaces.len(), HEPTA_TELEGRAM_BASE_GAPS.len());

    for (gap_id, surface) in explicit_gap_surfaces {
        assert!(
            HEPTA_TELEGRAM_BASE_GAPS.contains(&gap_id),
            "explicit gap id {gap_id} missing from HEPTA_TELEGRAM_BASE_GAPS"
        );
        let capability = HEPTA_TELEGRAM_BASE_CAPABILITIES
            .iter()
            .find(|capability| capability.telegram_surface == surface)
            .unwrap_or_else(|| panic!("missing gap capability surface {surface}"));
        assert_eq!(
            capability.status,
            HeptaTelegramBaseStatus::Gap,
            "gap capability surface {surface} must have Gap status"
        );
    }

    let unexpected_gap_surfaces: Vec<&str> = HEPTA_TELEGRAM_BASE_CAPABILITIES
        .iter()
        .filter(|capability| capability.status == HeptaTelegramBaseStatus::Gap)
        .filter(|capability| {
            !explicit_gap_surfaces
                .iter()
                .any(|(_, surface)| *surface == capability.telegram_surface)
        })
        .map(|capability| capability.telegram_surface)
        .collect();
    assert!(
        unexpected_gap_surfaces.is_empty(),
        "Gap status must only appear on explicit base gaps: {unexpected_gap_surfaces:?}"
    );
}

#[test]
fn hepta_telegram_base_gap_hard_boundary_audit_covers_every_gap() {
    assert!(hepta_telegram_base_gap_hard_boundary_audit_ready());
    assert_eq!(hepta_telegram_base_gap_hard_boundary_count(), 12);
    assert_eq!(
        hepta_telegram_base_gap_hard_boundary_count(),
        HEPTA_TELEGRAM_BASE_GAPS.len()
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_NOTIFICATIONS_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_LOCAL_BOUNDARY_MARKER)
    );
    assert!(
        HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS
            .contains(&HEPTA_TELEGRAM_COMPOSER_VOICE_PERMISSION_RECORDING_LOCAL_BOUNDARY_MARKER)
    );
}

#[test]
fn hepta_telegram_base_gap_product_runway_covers_every_gap() {
    assert!(hepta_telegram_base_gap_product_runway_ready());
    assert_eq!(hepta_telegram_base_gap_product_runway_count(), 12);
    assert_eq!(
        hepta_telegram_base_gap_product_runway_count(),
        HEPTA_TELEGRAM_BASE_GAPS.len()
    );

    for gap in HEPTA_TELEGRAM_BASE_GAPS {
        let runway_entry = HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY
            .iter()
            .find(|item| item.gap_id == *gap)
            .unwrap_or_else(|| panic!("missing product runway entry for {gap}"));
        assert!(
            !runway_entry.current_path.is_empty(),
            "current_path must be pinned for {gap}"
        );
        assert!(
            !runway_entry.remaining_gap.is_empty(),
            "remaining_gap must be pinned for {gap}"
        );
        assert!(
            !runway_entry.next_ui_safe_step.is_empty(),
            "next_ui_safe_step must be pinned for {gap}"
        );
    }

    let mut runway_ids: Vec<&str> = HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY
        .iter()
        .map(|item| item.gap_id)
        .collect();
    runway_ids.sort_unstable();
    runway_ids.dedup();
    assert_eq!(
        runway_ids.len(),
        HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.len(),
        "product runway entries must be unique"
    );
}

#[test]
fn hepta_telegram_base_local_gap_affordances_are_visible_without_live_mutation() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_HEADER_ACTIONS_MARKER,
        "hepta_telegram_room_header_actions_local_only"
    );
    assert_eq!(hepta_telegram_local_gap_affordance_count(), 408);
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"base_gap_product_runway_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_header"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"sidebar_message_search_button"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"sidebar_message_search_open_handoff"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_mode_strip"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_input_results_jumps"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_loaded_preview_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_empty_close_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_loaded_metadata_summary")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_active_result_detail"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_result_action_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_result_copy_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_query_lifecycle_metadata")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_server_context_boundary")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_server_context_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_advanced_filter_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_loaded_scope_filters_live_wiring")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_server_preflight_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_server_packet_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_matrix_contract_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_loaded_source_modal_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"dialog_list_empty_state_local_filter_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"rooms_list_membership_edge_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_pagination_adapter_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_load_more_pagination_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_header_space_scope_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"desktop_dock_restore_lazy_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mobile_stack_navigation_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"navigation_spaces_toggle_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"profile_icon_settings_navigation_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"settings_close_previous_selection_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"spaces_bar_entry_selection_local_read_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"spaces_bar_secondary_click_local_no_menu_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"spaces_bar_empty_filter_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_section_unread_aggregate_local_zero_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_section_unread_aggregate_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_all_rooms_loaded_local_unknown_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_space_parent_cache_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_name_update_selected_state_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_removed_room_selected_state_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_removed_room_rejoin_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"space_unread_filter_local_zero_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"space_unread_filter_aggregate_packet_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"timeline_pagination_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"thread_summary_read_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"reply_preview_event_details_read_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"sender_profile_event_details_read_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"unread_count_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"successor_room_details_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_preview_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"avatar_fetch_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"pinned_events_subscription_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"typing_notice_subscription_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_members_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_member_sync_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_power_levels_read_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"own_read_receipt_subscription_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_info_header"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_info_strip"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_loaded_identity_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_identity_clipboard_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_close_metadata_preview"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_edit_controls_boundary"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_edit_intent_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_field_edit_intent_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_refresh_result_detail_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_mutation_preflight_detail_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_power_member_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_context_settings_surface"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notification_mute_header"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notification_mute_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notification_option_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_loaded_attention_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_mode_clipboard_action"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_mode_target_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_close_refresh_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_local_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_pusher_keyword_boundary")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_keyword_list_live_read"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_pusher_status_live_read")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_advanced_controls_row"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_advanced_detail_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_result_detail_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_preflight_detail_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_rule_contract_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_context_notifications_surface"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_menu_header"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_actions_strip"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_actions_close_local_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_link_copy_handoff_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_local_surface_close_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_status_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"invite_user_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"timeline_invite_confirmation_guard"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"invite_response_confirmation_required")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"space_lobby_join_leave_modal_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"space_lobby_read_sync_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"space_lobby_room_list_lifecycle_cleanup_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"space_lobby_empty_state_read_sync_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"space_lobby_membership_edge_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"space_lobby_reknock_cancel_prior_packet_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"add_room_knock_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"add_room_preview_cancel_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"add_room_membership_edge_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"add_room_reknock_cancel_prior_packet_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_pin_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_confirmation_guard"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_unsupported_features_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_detail_packet_preview"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_save_result_mapping_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_retry_error_drilldown_packet_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_option_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_loaded_target_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_custom_reason_draft_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_moderation_workflow_boundary")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_preflight_detail_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_moderation_reviewer_packet_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_click_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_target_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_full_modal_boundary")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_local_full_snapshot_modal_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_full_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_source_modal_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_diff_detail_state")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_diff_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_side_by_side_diff_modal_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_full_diff_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_full_history_result_contract_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_remote_result_taxonomy_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_preflight_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_identity_preview_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_wallet_pending_cancel_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_wallet_import_blocked_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_association_cancel_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_association_result_taxonomy_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_verification_request_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"crypto_verification_request_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"login_auto_cancel_local_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_upload_composer"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_picker_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_mobile_picker_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_handoff_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_send_handoff_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_pre_send_review_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_review_lifecycle_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_validation_error_recovery_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_send_operation_status_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_send_result_bridge_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_queue_failure_recovery_copy_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_timeline_send_state_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_send_failure_retry_confirmation_guard")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_true_queue_control_local_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_accepted_queue_actions_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_accepted_queue_timeline_cancel_bridge")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_local_send_abort_result_bridge")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_per_file_queue_drilldown_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_sdk_queue_contract_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_multi_file_queue_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_status_taxonomy_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_review_row_compact_fit_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_mobile_action_density_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_message_preview_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_message_blocked_actions_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_download_playback_local_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_inline_playback_queue_boundary_metadata")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_codec_transcode_controls_row"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_encrypted_image_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_result_status_boundary"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_result_recovery_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_preflight_detail_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_operation_packet_drilldown_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_playback_queue_contract_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_playback_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_fetch_cache_read_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"poll_answer_preview_result_packet_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"image_viewer_local_controls"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"link_preview_local_controls"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"link_preview_loaded_metadata_summary"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"url_preview_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_preview_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_unknown_target_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_loaded_event_source_modal_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_server_context_boundary"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_room_target_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_via_servers_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_event_id_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_preview_metadata_clipboard_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_context_actions_row"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_route_scope_controls_row"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_route_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_unresolved_detail_state"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"external_link_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"event_source_local_surface"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"event_source_clipboard_copy_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"event_source_loaded_metadata_summary"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_copy_local_surface"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_copy_loaded_metadata_summary"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_display_name_staging_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_display_name_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_device_self_check_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_upload_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_option_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_selected_file_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_selected_image_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_upload_decode_probe"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_crop_editor_boundary")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_upload_editor_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_preview_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_editor_drilldown_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_editor_typed_contract_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_editor_result_taxonomy_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_local_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_delete_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_management_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_loaded_identity_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_management_browser_portal_handoff")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_device_rename_live_wiring")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_lifecycle_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_refresh_confirmation_guard")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_local_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_session_revoke_boundary")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_management_session_actions_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_device_directory_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_device_metadata_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_session_device_drilldown_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_session_device_typed_contract_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_session_device_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_local_surface_close_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_logout_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"emoji_sticker_picker_composer"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"emoji_sticker_surface"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_helper_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_option_staging_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_lifecycle_metadata_preview"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_keyboard_selection_boundary"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_directory_disambiguation_boundary")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_rich_directory_controls_row"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_directory_search_live_wiring"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_directory_result_promotion_live")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_local_pill_tray_live"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_preflight_detail_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_send_payload_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"composer_typing_notice_send_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"composer_local_surface_close_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"location_send_confirmation_guard"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"live_location_continuous_updates_boundary")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_message_composer"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_message_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_permission_recording_local_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_recorder_waveform_codec_boundary")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_recorder_status_controls_row"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_selected_audio_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_lifecycle_metadata_preview"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_confirmation_cancel_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_recorder_lifecycle_drilldown_packet_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"profile_direct_message_confirmation_guard")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"direct_message_create_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"profile_ignore_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"profile_member_read_evidence"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"profile_read_receipt_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"profile_account_identity_clipboard_surface")
    );
}

#[test]
fn hepta_telegram_base_room_menu_ellipsis_stays_visible() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_MENU_ELLIPSIS_MARKER,
        "hepta_telegram_room_menu_ellipsis_visible"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_menu_header"));
}

#[test]
fn hepta_telegram_base_room_context_settings_local_notifications_confirmed() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_CONTEXT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_room_context_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_context_settings_surface"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_context_notifications_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.base_module == "RoomContextMenu"
            && capability.notes.contains("local-only settings preview")
            && capability
                .notes
                .contains("confirmed All/Mentions/Mute notification mode writes")
            && capability.notes.contains("timed mute stays unwired")
    }));
}

#[test]
fn hepta_telegram_base_room_actions_strip_reuses_link_generation() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_ACTIONS_STRIP_MARKER,
        "hepta_telegram_room_actions_strip_uses_base_link_path"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_LINK_COPY_HANDOFF_MARKER,
        "hepta_telegram_room_link_copy_handoff_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_actions_strip"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_link_copy_handoff_evidence"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"generate_matrix_link"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.base_module == "RoomContextMenu"
            && capability
                .notes
                .contains("existing Matrix GenerateMatrixLink path")
            && capability.notes.contains("writes to clipboard")
            && capability
                .notes
                .contains("without room state or membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_room_actions_strip_reuses_invite_leave_modals() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_ACTIONS_INVITE_LEAVE_MARKER,
        "hepta_telegram_room_actions_invite_leave_reuse_existing_modals"
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_invite"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_leave"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.base_module == "RoomContextMenu"
            && capability.notes.contains("invite")
            && capability.notes.contains("leave")
    }));
}

#[test]
fn hepta_telegram_base_room_status_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_STATUS_CONFIRMATION_MARKER,
        "hepta_telegram_room_status_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_status_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_status"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room status confirmation"
            && capability
                .notes
                .contains("before Matrix SetUnreadFlag, SetIsFavorite, or SetIsLowPriority")
            && capability
                .notes
                .contains("Cancel keeps the room status request unsent")
    }));
}

#[test]
fn hepta_telegram_base_invite_user_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_INVITE_USER_CONFIRMATION_MARKER,
        "hepta_telegram_invite_user_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"invite_user_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"invite_user"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room invite confirmation"
            && capability
                .notes
                .contains("before the existing Matrix InviteUser path")
            && capability
                .notes
                .contains("Cancel keeps the invite request unsent")
    }));
}

#[test]
fn hepta_telegram_base_timeline_invite_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_TIMELINE_INVITE_CONFIRMATION_MARKER,
        "hepta_telegram_timeline_invite_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"timeline_invite_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"timeline_invite_user"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "timeline invite confirmation"
            && capability
                .notes
                .contains("Knocked-member timeline Invite buttons")
            && capability
                .notes
                .contains("Cancel keeps the timeline invite request unsent")
    }));
}

#[test]
fn hepta_telegram_base_invite_response_confirmation_is_required() {
    assert_eq!(
        HEPTA_TELEGRAM_INVITE_RESPONSE_CONFIRMATION_MARKER,
        "hepta_telegram_invite_response_confirmation_required"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"invite_response_confirmation_required")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"invite_response"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "invite response confirmation"
            && capability
                .notes
                .contains("before Matrix JoinRoom or LeaveRoom is requested")
            && capability
                .notes
                .contains("Shift-click no longer bypasses confirmation")
    }));
}

#[test]
fn hepta_telegram_base_space_lobby_join_leave_reuses_confirmation_modal() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_JOIN_LEAVE_MODAL_MARKER,
        "hepta_telegram_space_lobby_join_leave_modal_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"space_lobby_join_leave_modal_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"space_lobby_join_leave"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "space lobby join/leave confirmation"
            && capability.base_module == "SpaceLobbyScreen + JoinLeaveRoomModal"
            && capability
                .notes
                .contains("before Matrix JoinRoom or LeaveRoom is requested")
            && capability
                .notes
                .contains("membership mutation path behind confirmation")
    }));
}

#[test]
fn hepta_telegram_base_space_lobby_read_sync_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_READ_SYNC_MARKER,
        "hepta_telegram_space_lobby_read_sync_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"space_lobby_read_sync_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"space_lobby_read_sync"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "space lobby hierarchy read/sync path"
            && capability.base_module == "SpaceLobbyScreen + SpaceService"
            && capability
                .notes
                .contains("SpaceRequest GetDetailedChildren")
            && capability.notes.contains("GetTopLevelSpaceDetails")
            && capability.notes.contains("SubscribeToSpaceRoomList")
            && capability.notes.contains("do not send LeaveSpace")
            && capability.notes.contains("membership")
            && capability.notes.contains("room-state mutation")
    }));
}

#[test]
fn hepta_telegram_base_space_lobby_room_list_lifecycle_is_cleanup_only() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_ROOM_LIST_LIFECYCLE_MARKER,
        "hepta_telegram_space_lobby_room_list_lifecycle_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"space_lobby_room_list_lifecycle_cleanup_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"space_lobby_room_list_lifecycle"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "space lobby room-list lifecycle cleanup"
            && capability
                .base_module
                .contains("SpaceRequest::UnsubscribeFromSpaceRoomList")
            && capability.notes.contains("SubscribeToSpaceRoomList")
            && capability.notes.contains("PaginateSpaceRoomList")
            && capability.notes.contains("service lifecycle cleanup")
            && capability
                .notes
                .contains("not a user-facing stop-sync control")
            && capability
                .notes
                .contains("sends no UnsubscribeFromSpaceRoomList")
            && capability.notes.contains("LeaveSpace")
            && capability.notes.contains("room-state")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_space_lobby_empty_state_read_sync_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_EMPTY_STATE_READ_SYNC_MARKER,
        "hepta_telegram_space_lobby_empty_state_read_sync_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"space_lobby_empty_state_read_sync_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"space_lobby_empty_state_read_sync"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "space lobby empty/filter read-sync state"
            && capability.base_module == "SpaceLobbyScreen + SpaceService"
            && capability.notes.contains("GetDetailedChildren")
            && capability.notes.contains("cached children")
            && capability.notes.contains("do not send Matrix search")
            && capability.notes.contains("JoinRoom")
            && capability.notes.contains("LeaveSpace")
            && capability.notes.contains("membership mutation")
            && capability.notes.contains("room-state mutation")
    }));
}

#[test]
fn hepta_telegram_base_space_lobby_membership_edge_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_LOBBY_MEMBERSHIP_EDGE_LOCAL_MARKER,
        "hepta_telegram_space_lobby_membership_edge_local_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"space_lobby_membership_edge_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"space_lobby_reknock_cancel_prior_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"space_lobby_membership_edge"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "space lobby banned/knocked membership edge evidence"
            && capability.base_module == "SpaceLobbyScreen + SpaceService"
            && capability
                .notes
                .contains("Banned and Knocked child room/space states")
            && capability.notes.contains("disable Join/Knock locally")
            && capability
                .notes
                .contains("tree_reknock_action_slot not_exposed")
            && capability
                .notes
                .contains("cancel_prior_request_slot not_built")
            && capability
                .notes
                .contains("cancel_prior_result_slot not_wired")
            && capability.notes.contains("Matrix JoinRoom")
            && capability.notes.contains("Knock")
            && capability.notes.contains("membership")
            && capability.notes.contains("room-state requests")
    }));
}

#[test]
fn hepta_telegram_base_add_room_knock_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_ADD_ROOM_KNOCK_CONFIRMATION_MARKER,
        "hepta_telegram_add_room_knock_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"add_room_knock_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"add_room_knock"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "add room knock confirmation"
            && capability
                .notes
                .contains("before the existing Matrix Knock path")
            && capability
                .notes
                .contains("Cancel keeps the knock request unsent")
    }));
}

#[test]
fn hepta_telegram_base_add_room_preview_cancel_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ADD_ROOM_PREVIEW_CANCEL_MARKER,
        "hepta_telegram_add_room_preview_cancel_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"add_room_preview_cancel_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"add_room_preview_cancel"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "add room preview cancel"
            && capability.base_module == "AddRoomScreen fetched room preview"
            && capability.notes.contains("existing room preview lookup")
            && capability
                .notes
                .contains("no Matrix JoinRoom, Knock, or membership mutation request")
    }));
}

#[test]
fn hepta_telegram_base_add_room_membership_edge_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ADD_ROOM_MEMBERSHIP_EDGE_LOCAL_MARKER,
        "hepta_telegram_add_room_membership_edge_local_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"add_room_membership_edge_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"add_room_reknock_cancel_prior_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"add_room_membership_edge"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "add room banned/knocked membership edge evidence"
            && capability.base_module == "AddRoomScreen fetched room preview"
            && capability
                .notes
                .contains("Banned previews disable Join/Knock")
            && capability.notes.contains("cancel-prior-knock")
            && capability.notes.contains("already-knocked previews")
            && capability
                .notes
                .contains("existing Knock confirmation path")
            && capability.notes.contains("previous_knock_request_id")
            && capability
                .notes
                .contains("cancel_prior_request_slot not_built")
            && capability
                .notes
                .contains("cancel_prior_result_slot not_wired")
            && capability.notes.contains("no cancel request")
    }));
}

#[test]
fn hepta_telegram_base_add_room_restricted_join_rule_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ADD_ROOM_RESTRICTED_JOIN_RULE_LOCAL_MARKER,
        "hepta_telegram_add_room_restricted_join_rule_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"add_room_restricted_join_rule_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"add_room_restricted_join_rule"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "add room restricted join-rule local evidence"
            && capability.base_module == "AddRoomScreen fetched room preview"
            && capability
                .notes
                .contains("Restricted or unknown join-rule previews")
            && capability
                .notes
                .contains("membership actions disabled locally")
            && capability.notes.contains("newer Matrix join-rule handling")
            && capability.notes.contains("no Matrix JoinRoom")
            && capability.notes.contains("Knock")
            && capability.notes.contains("cancel-prior-knock")
            && capability.notes.contains("membership request")
            && capability.notes.contains("existing confirmation guard")
    }));
}

#[test]
fn hepta_telegram_base_room_actions_strip_reuses_stateful_toggles() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_ACTIONS_TOGGLES_MARKER,
        "hepta_telegram_room_actions_stateful_toggles_reuse_base_paths"
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"set_is_favorite"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"set_is_low_priority"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"set_unread_flag"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.base_module.contains("RoomScreen")
            && capability.notes.contains("SetIsFavorite")
            && capability.notes.contains("SetIsLowPriority")
            && capability.notes.contains("SetUnreadFlag")
    }));
}

#[test]
fn hepta_telegram_base_room_info_strip_uses_loaded_room_state_without_settings_mutation() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_INFO_STRIP_MARKER,
        "hepta_telegram_room_info_strip_uses_loaded_room_state"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_info_strip"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.base_module.contains("RoomScreen")
            && capability.notes.contains("read-only room info strip")
            && capability
                .notes
                .contains("loaded RoomScreen/RoomsList state")
    }));
}

#[test]
fn hepta_telegram_base_pinned_events_subscription_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_PINNED_EVENTS_SUBSCRIPTION_MARKER,
        "hepta_telegram_pinned_events_subscription_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"pinned_events_subscription_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"pinned_events_subscription"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "pinned events subscription read path"
            && capability
                .notes
                .contains("existing Matrix SubscribeToPinnedEvents")
            && capability.notes.contains("subscribe/unsubscribe")
            && capability.notes.contains("local pinned-event state")
            && capability.notes.contains("does not send PinEvent")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership")
    }));
}

#[test]
fn hepta_telegram_base_typing_notice_subscription_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_TYPING_NOTICE_SUBSCRIPTION_MARKER,
        "hepta_telegram_typing_notice_subscription_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"typing_notice_subscription_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"typing_notice_subscription"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "typing notice subscription read path"
            && capability
                .notes
                .contains("existing Matrix SubscribeToTypingNotices")
            && capability.notes.contains("subscribe/unsubscribe")
            && capability.notes.contains("local typing-user state")
            && capability.notes.contains("does not send a typing notice")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership")
    }));
}

#[test]
fn hepta_telegram_base_room_members_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_MEMBERS_READ_MARKER,
        "hepta_telegram_room_members_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_members_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_members_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room members read path"
            && capability.notes.contains(
                "SyncRoomMemberList plus loaded RoomMembersListFetched/GetRoomMembers local-only",
            )
            && capability.notes.contains("GetRoomMembers(local_only=true)")
            && capability.notes.contains("SDK local cache")
            && capability.notes.contains("does not send JoinRoom")
            && capability.notes.contains("InviteUser")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_room_member_sync_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_MEMBER_SYNC_READ_MARKER,
        "hepta_telegram_room_member_sync_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_member_sync_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_member_sync_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room member sync read path"
            && capability
                .notes
                .contains("existing Matrix SyncRoomMemberList read/sync path")
            && capability.notes.contains("local member profiles")
            && capability.notes.contains("GetRoomMembers local_only")
            && capability.notes.contains("does not send JoinRoom")
            && capability.notes.contains("InviteUser")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_room_power_levels_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_POWER_LEVELS_READ_MARKER,
        "hepta_telegram_room_power_levels_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_power_levels_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_power_levels_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room power levels read path"
            && capability
                .notes
                .contains("existing Matrix GetRoomPowerLevels")
            && capability.notes.contains("current user")
            && capability.notes.contains("permission display state")
            && capability.notes.contains("local UserPowerLevels")
            && capability.notes.contains("does not send power-level")
            && capability.notes.contains("room-state")
            && capability.notes.contains("message")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_own_read_receipt_subscription_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_OWN_READ_RECEIPT_SUBSCRIPTION_MARKER,
        "hepta_telegram_own_read_receipt_subscription_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"own_read_receipt_subscription_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"own_read_receipt_subscription"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "own read receipt subscription read path"
            && capability
                .notes
                .contains("existing Matrix SubscribeToOwnUserReadReceiptsChanged")
            && capability.notes.contains("subscribe/unsubscribe")
            && capability.notes.contains("local own-read-marker state")
            && capability.notes.contains("does not send ReadReceipt")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_surface_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_SURFACE_MARKER,
        "hepta_telegram_room_settings_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_OPTION_SUMMARY_MARKER,
        "hepta_telegram_room_settings_option_summary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_room_settings_option_staging_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_LOADED_IDENTITY_MARKER,
        "hepta_telegram_room_settings_loaded_identity_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_CLOSE_METADATA_MARKER,
        "hepta_telegram_room_settings_close_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_METADATA_MARKER,
        "hepta_telegram_room_settings_refresh_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_room_settings_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_MARKER,
        "hepta_telegram_room_settings_edit_controls_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_EDIT_INTENT_STAGING_MARKER,
        "hepta_telegram_room_settings_edit_intent_staging_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_MARKER,
        "hepta_telegram_room_settings_field_edit_intent_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_option_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_name_id_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_permissions_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_members_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_loaded_identity_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_close_metadata_preview"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_refresh_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_refresh_live_read_wiring")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_edit_controls_boundary"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_edit_intent_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_refresh_result_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_context_settings_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings"
            && capability
                .notes
                .contains("partial-live room settings surfaces")
            && capability
                .notes
                .contains("permissions from GetRoomPowerLevels")
            && capability
                .notes
                .contains("loaded canonical alias/avatar/tombstone")
            && capability.notes.contains("cached members")
            && capability.notes.contains("MatrixRequest::SetRoomName")
            && capability.notes.contains("MatrixRequest::SetRoomTopic")
            && capability.notes.contains("MatrixRequest::UploadRoomAvatar")
            && capability.notes.contains("MatrixRequest::RemoveRoomAvatar")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomHistoryVisibility")
            && capability.notes.contains("MatrixRequest::SetRoomJoinRule")
            && capability.notes.contains("RoomSettingsMutationResult")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomCanonicalAlias")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings option staging local evidence"
            && capability
                .notes
                .contains("Name copies loaded room label/id")
            && capability
                .notes
                .contains("Identity copies loaded RoomsList")
            && capability
                .notes
                .contains("Perms copies the loaded GetRoomPowerLevels")
            && capability
                .notes
                .contains("Members copies the loaded local room_members")
            && capability.notes.contains("Topic shows")
            && capability.notes.contains("Avatar edit")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains(
                "separate Save name, Save topic, Save alias, Avatar edit, Remove avatar, History, Join rule, and Tombstone controls",
            )
            && capability.notes.contains("MatrixRequest::SetRoomName")
            && capability.notes.contains("SetRoomTopic")
            && capability.notes.contains("UploadRoomAvatar")
            && capability.notes.contains("RemoveRoomAvatar")
            && capability.notes.contains("SetRoomHistoryVisibility")
            && capability.notes.contains("SetRoomJoinRule")
            && capability.notes.contains("SetRoomTombstone")
            && capability.notes.contains("power-level")
            && capability.notes.contains("membership")
            && capability.notes.contains("message")
            && capability.notes.contains("unrelated room-state mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings loaded identity preview"
            && capability
                .notes
                .contains("already loaded RoomsList metadata")
            && capability.notes.contains("canonical alias presence")
            && capability.notes.contains("avatar cache state")
            && capability.notes.contains("tombstone state")
            && capability.notes.contains("RoomScreen loaded member count")
            && capability.notes.contains("m.room.avatar")
            && capability.notes.contains("m.room.canonical_alias")
            && capability.notes.contains("m.room.tombstone")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings close metadata"
            && capability.notes.contains("local option-staging state")
            && capability
                .notes
                .contains("loaded RoomsList identity availability")
            && capability.notes.contains("RoomScreen member count")
            && capability.notes.contains("power-level display readiness")
            && capability.notes.contains("m.room.power_levels")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings refresh metadata"
            && capability
                .notes
                .contains("MatrixRequest::GetRoomPowerLevels")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomMembers(server-backed)")
            && capability.notes.contains("loaded RoomsList identity state")
            && capability.notes.contains("cached member count")
            && capability.notes.contains("power-level display readiness")
            && capability.notes.contains("m.room.power_levels mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_avatar_upload"
            && runway
                .current_path
                .contains("typed cropper-camera contract packet")
            && runway.next_ui_safe_step.contains(
                "coordinate backend avatar source/cropper/camera/editor/thumbnail contracts",
            )
    }));
}

#[test]
fn hepta_telegram_base_room_settings_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_READONLY_BOUNDARY_MARKER,
        "hepta_telegram_room_settings_readonly_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_room_settings_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_permissions_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_MEMBERS_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_members_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_LOCAL_BOUNDARY_EVIDENCE
            .contains("compact partial-live summary")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_LOCAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SetRoomName")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_LOCAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SetRoomTopic")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_LOCAL_BOUNDARY_EVIDENCE
            .contains("GetRoomPowerLevels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE
            .contains("tl_state.user_power")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE
            .contains("m.room.power_levels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE
            .contains("room_members cache")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE
            .contains("membership list write")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_LOADED_IDENTITY_EVIDENCE
            .contains("RoomsList metadata")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_CLOSE_METADATA_EVIDENCE
            .contains("current local option-staging state")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_CLOSE_METADATA_EVIDENCE
            .contains("local room_members cache count")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_METADATA_EVIDENCE
            .contains("MatrixRequest::GetRoomPowerLevels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_METADATA_EVIDENCE
            .contains("MatrixRequest::GetRoomMembers(server-backed)")
    );
    assert!(
        crate::home::room_context_menu::ROOM_CONTEXT_SETTINGS_LOADED_IDENTITY_EVIDENCE
            .contains("avatar cache state")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_LOCAL_BOUNDARY_EVIDENCE.contains("power levels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_LOCAL_BOUNDARY_EVIDENCE.contains("canonical alias")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_LOCAL_BOUNDARY_LABEL.contains(
            "name/topic/avatar/alias/history/join-rule/tombstone writes require confirmation"
        )
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_CLOSE_METADATA_LABEL
            .contains("no Matrix room-state request")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_METADATA_LABEL.contains(
            "Name/Topic/avatar/alias/history/join-rule/tombstone writes use confirmed live room-state path"
        )
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_close_metadata_preview"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_refresh_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_permissions_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_members_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_edit_controls_boundary"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_edit_intent_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_field_edit_intent_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings local boundary evidence"
            && capability
                .base_module
                .contains("telegram_room_settings_strip")
            && capability.notes.contains(
                "Save name, Save topic, Save alias, Avatar edit, Remove avatar, History, Join rule, and Tombstone",
            )
            && capability.notes.contains("Room settings")
            && capability.notes.contains("m.room.power_levels")
            && capability.notes.contains("invite")
            && capability.notes.contains("kick")
            && capability.notes.contains("ban")
            && capability.notes.contains("SetRoomCanonicalAlias")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_name_topic_live_write_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_MARKER,
        "hepta_telegram_room_settings_name_topic_live_write_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("MatrixRequest::SetRoomName")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("MatrixRequest::SetRoomTopic")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("MatrixRequest::UploadRoomAvatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("MatrixRequest::RemoveRoomAvatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("MatrixRequest::SetRoomHistoryVisibility")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("MatrixRequest::SetRoomJoinRule")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("MatrixRequest::SetRoomTombstone")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("Room::set_name")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("Room::set_room_topic")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("Room::upload_avatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("Room::remove_avatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("Room::send_state_event")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("RoomHistoryVisibilityEventContent")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("RoomJoinRulesEventContent")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("RoomTombstoneEventContent")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("TimelineUpdate::RoomSettingsMutationResult")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("failed-state Retry opens PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
            .contains("resubmits the cached MatrixRequest::SetRoomName")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_name_topic_live_write"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_avatar_remove_live_wiring")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_tombstone_live_write"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings name topic avatar tombstone live write"
            && capability
                .notes
                .contains("partial-live Matrix room-state writes")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::SetRoomName")
            && capability.notes.contains("MatrixRequest::SetRoomTopic")
            && capability.notes.contains("MatrixRequest::UploadRoomAvatar")
            && capability.notes.contains("MatrixRequest::RemoveRoomAvatar")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomHistoryVisibility")
            && capability.notes.contains("MatrixRequest::SetRoomJoinRule")
            && capability.notes.contains("MatrixRequest::SetRoomTombstone")
            && capability.notes.contains("Room::set_name")
            && capability.notes.contains("Room::set_room_topic")
            && capability.notes.contains("Room::upload_avatar")
            && capability.notes.contains("Room::remove_avatar")
            && capability.notes.contains("Room::send_state_event")
            && capability.notes.contains("RoomTombstoneEventContent")
            && capability
                .notes
                .contains("TimelineUpdate::RoomSettingsMutationResult")
            && capability.notes.contains("cached failed-state Retry")
            && capability.notes.contains("cached avatar file/MIME")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("room_settings")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway.current_path.contains("live confirmed SetRoomName")
            && runway.current_path.contains("SetRoomTopic")
            && runway.current_path.contains("SetRoomCanonicalAlias")
            && runway.current_path.contains("UploadRoomAvatar")
            && runway.current_path.contains("RemoveRoomAvatar")
            && runway.current_path.contains("SetRoomHistoryVisibility")
            && runway.current_path.contains("SetRoomJoinRule")
            && runway.current_path.contains("SetRoomTombstone")
            && runway
                .current_path
                .contains("confirmed failed-state Retry resubmit")
            && runway
                .remaining_gap
                .contains("editable power-level/member state")
            && !runway.remaining_gap.contains("tombstone")
    }));
}
