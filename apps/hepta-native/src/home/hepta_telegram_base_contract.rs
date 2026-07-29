//! UI-facing contract for the Telegram skin on top of the existing Matrix chat base.
//!
//! This module does not create new runtime behavior. It pins the product direction:
//! Telegram is the visual shell, while Robrix/Matrix remains the chat engine.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeptaTelegramBaseStatus {
    DirectReuse,
    ReskinNeeded,
    Gap,
}

impl HeptaTelegramBaseStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectReuse => "direct_reuse",
            Self::ReskinNeeded => "reskin_needed",
            Self::Gap => "gap",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeptaTelegramBaseCapability {
    pub telegram_surface: &'static str,
    pub base_module: &'static str,
    pub status: HeptaTelegramBaseStatus,
    pub notes: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeptaTelegramBaseGapRunway {
    pub gap_id: &'static str,
    pub current_path: &'static str,
    pub remaining_gap: &'static str,
    pub next_ui_safe_step: &'static str,
}

pub const HEPTA_TELEGRAM_BASE_CONTRACT_MARKER: &str = "hepta_telegram_real_base_contract_ready";
pub const HEPTA_TELEGRAM_BASE_MODULE_MARKER: &str = "hepta_telegram_uses_matrix_heart_modules";
pub const HEPTA_TELEGRAM_STATIC_FIXTURE_SCOPE_MARKER: &str =
    "hepta_telegram_static_fixture_smoke_only";
pub const HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_AUDIT_MARKER: &str =
    "hepta_telegram_base_gap_hard_boundary_audit_ready";
pub const HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY_MARKER: &str =
    "hepta_telegram_base_gap_product_runway_ready";
pub const HEPTA_TELEGRAM_SHELL_MARKER: &str = "hepta_telegram_shell_ready";
pub const HEPTA_TELEGRAM_BOTTOM_ANCHORED_MARKER: &str = "hepta_telegram_bottom_anchored_ready";
pub const HEPTA_TELEGRAM_REAL_CHROME_MARKER: &str = "hepta_telegram_real_chrome_on_standard_shell";
pub const HEPTA_TELEGRAM_ACTION_CHROME_MARKER: &str = "hepta_telegram_action_chrome_on_real_menus";
pub const HEPTA_TELEGRAM_TIMELINE_PAGINATION_READ_MARKER: &str =
    "hepta_telegram_timeline_pagination_read_ready";
pub const HEPTA_TELEGRAM_THREAD_SUMMARY_READ_MARKER: &str =
    "hepta_telegram_thread_summary_read_ready";
pub const HEPTA_TELEGRAM_THREAD_OPEN_TIMELINE_READ_MARKER: &str =
    "hepta_telegram_thread_open_timeline_read_ready";
pub const HEPTA_TELEGRAM_REPLY_PREVIEW_EVENT_DETAILS_READ_MARKER: &str =
    "hepta_telegram_reply_preview_event_details_read_ready";
pub const HEPTA_TELEGRAM_SENDER_PROFILE_EVENT_DETAILS_READ_MARKER: &str =
    "hepta_telegram_sender_profile_event_details_read_ready";
pub const HEPTA_TELEGRAM_UNREAD_COUNT_READ_MARKER: &str = "hepta_telegram_unread_count_read_ready";
pub const HEPTA_TELEGRAM_SUCCESSOR_ROOM_DETAILS_READ_MARKER: &str =
    "hepta_telegram_successor_room_details_read_ready";
pub const HEPTA_TELEGRAM_ROOM_PREVIEW_READ_MARKER: &str = "hepta_telegram_room_preview_read_ready";
pub const HEPTA_TELEGRAM_AVATAR_FETCH_READ_MARKER: &str = "hepta_telegram_avatar_fetch_read_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_message_report_local_surface_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_OPTION_STAGING_LOCAL_MARKER: &str =
    "hepta_telegram_message_report_option_staging_local_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_message_report_send_local_boundary_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_CONTENT_LIVE_SEND_WIRING_MARKER: &str =
    "hepta_telegram_message_report_content_live_send_wiring_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_MARKER: &str =
    "hepta_telegram_message_report_moderation_workflow_boundary_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_LOADED_TARGET_METADATA_MARKER: &str =
    "hepta_telegram_message_report_loaded_target_metadata_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_CUSTOM_REASON_CONFIRMATION_MARKER: &str =
    "hepta_telegram_message_report_custom_reason_confirmation_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_MARKER: &str =
    "hepta_telegram_message_report_custom_reason_draft_metadata_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_CANCEL_LOCAL_MARKER: &str =
    "hepta_telegram_message_report_cancel_local_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_STATUS_LIFECYCLE_MARKER: &str =
    "hepta_telegram_message_report_status_lifecycle_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_STATUS_CLIPBOARD_MARKER: &str =
    "hepta_telegram_message_report_status_clipboard_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_RETRY_CONFIRMATION_MARKER: &str =
    "hepta_telegram_message_report_retry_confirmation_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_MARKER: &str =
    "hepta_telegram_message_report_workflow_actions_row_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_MARKER: &str =
    "hepta_telegram_message_report_moderation_reviewer_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_message_report_workflow_result_contract_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_message_report_workflow_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_message_report_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_MESSAGE_REPORT_LOADED_SOURCE_MODAL_MARKER: &str =
    "hepta_telegram_message_report_loaded_source_modal_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_message_edit_history_local_surface_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_CLICK_LOCAL_MARKER: &str =
    "hepta_telegram_message_edit_history_click_local_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_COMPACT_SUMMARY_LIVE_READ_WIRING_MARKER: &str =
    "hepta_telegram_message_edit_history_compact_summary_live_read_wiring_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_message_edit_history_local_boundary_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_PREVIEW_MARKER: &str =
    "hepta_telegram_message_edit_history_loaded_preview_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_MARKER: &str =
    "hepta_telegram_message_edit_history_loaded_target_metadata_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_MARKER: &str =
    "hepta_telegram_message_edit_history_detail_surface_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_message_edit_history_full_modal_boundary_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_MARKER: &str =
    "hepta_telegram_message_edit_history_local_full_snapshot_modal_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_CONTROLS_MARKER: &str =
    "hepta_telegram_message_edit_history_full_controls_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_MARKER: &str =
    "hepta_telegram_message_edit_history_loaded_source_modal_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_MARKER: &str =
    "hepta_telegram_message_edit_history_loaded_diff_detail_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_MARKER: &str =
    "hepta_telegram_message_edit_history_loaded_diff_clipboard_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_MARKER: &str =
    "hepta_telegram_message_edit_history_loaded_side_by_side_diff_modal_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_MARKER: &str =
    "hepta_telegram_message_edit_history_full_diff_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_message_edit_history_full_history_result_contract_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_message_edit_history_remote_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_message_edit_history_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_MARKER: &str =
    "hepta_telegram_message_edit_history_retry_confirmation_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_CONFIRMATION_MARKER: &str =
    "hepta_telegram_message_edit_confirmation_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_UNSUPPORTED_FEATURES_LOCAL_MARKER: &str =
    "hepta_telegram_message_edit_unsupported_features_local_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_ATTACHMENT_PREFLIGHT_PACKET_MARKER: &str =
    "hepta_telegram_message_edit_attachment_preflight_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_MENTION_PAYLOAD_PREFLIGHT_PACKET_MARKER: &str =
    "hepta_telegram_message_edit_mention_payload_preflight_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_message_edit_mention_payload_typed_contract_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_SAVE_RESULT_MAPPING_PACKET_MARKER: &str =
    "hepta_telegram_message_edit_save_result_mapping_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_EDIT_RETRY_ERROR_DRILLDOWN_PACKET_MARKER: &str =
    "hepta_telegram_message_edit_retry_error_drilldown_packet_ready";
pub const HEPTA_TELEGRAM_TSP_IDENTITY_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_tsp_identity_local_surface_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_PENDING_CANCEL_LOCAL_MARKER: &str =
    "hepta_telegram_tsp_wallet_pending_cancel_local_ready";
pub const HEPTA_TELEGRAM_TSP_PENDING_CANCEL_OPERATION_PACKET_MARKER: &str =
    "hepta_telegram_tsp_pending_cancel_operation_packet_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_OPEN_RETRY_MARKER: &str =
    "hepta_telegram_tsp_wallet_open_retry_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_SET_DEFAULT_CONFIRMATION_METADATA_MARKER: &str =
    "hepta_telegram_tsp_wallet_set_default_confirmation_metadata_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_REMOVE_CONFIRMATION_METADATA_MARKER: &str =
    "hepta_telegram_tsp_wallet_remove_confirmation_metadata_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_DELETE_BLOCKED_METADATA_MARKER: &str =
    "hepta_telegram_tsp_wallet_delete_blocked_metadata_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_DELETE_PREFLIGHT_RESULT_PACKET_MARKER: &str =
    "hepta_telegram_tsp_wallet_delete_preflight_result_packet_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_IMPORT_LOCAL_MARKER: &str =
    "hepta_telegram_tsp_wallet_import_local_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_IMPORT_BLOCKED_METADATA_MARKER: &str =
    "hepta_telegram_tsp_wallet_import_blocked_metadata_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_IMPORT_PREFLIGHT_PACKET_MARKER: &str =
    "hepta_telegram_tsp_wallet_import_preflight_packet_ready";
pub const HEPTA_TELEGRAM_TSP_WALLET_IMPORT_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_tsp_wallet_import_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_TSP_WORKER_RECEIPT_RESULT_PACKET_MARKER: &str =
    "hepta_telegram_tsp_worker_receipt_result_packet_ready";
pub const HEPTA_TELEGRAM_TSP_ASSOCIATION_CANCEL_LOCAL_MARKER: &str =
    "hepta_telegram_tsp_association_cancel_local_ready";
pub const HEPTA_TELEGRAM_TSP_ASSOCIATION_BLOCKED_METADATA_MARKER: &str =
    "hepta_telegram_tsp_association_blocked_metadata_ready";
pub const HEPTA_TELEGRAM_TSP_ASSOCIATION_CANCEL_REMOVE_PACKET_MARKER: &str =
    "hepta_telegram_tsp_association_cancel_remove_packet_ready";
pub const HEPTA_TELEGRAM_TSP_ASSOCIATION_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_tsp_association_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_TSP_VERIFICATION_REQUEST_METADATA_MARKER: &str =
    "hepta_telegram_tsp_verification_request_metadata_ready";
pub const HEPTA_TELEGRAM_CRYPTO_VERIFICATION_REQUEST_METADATA_MARKER: &str =
    "hepta_telegram_crypto_verification_request_metadata_ready";
pub const HEPTA_TELEGRAM_LOGIN_AUTO_CANCEL_LOCAL_MARKER: &str =
    "hepta_telegram_login_auto_cancel_local_ready";
pub const HEPTA_TELEGRAM_DIALOG_FILTER_MARKER: &str =
    "hepta_telegram_dialog_filter_on_real_sidebar";
pub const HEPTA_TELEGRAM_DIALOG_STATE_FILTER_MARKER: &str =
    "hepta_telegram_dialog_state_filters_on_real_room_filter";
pub const HEPTA_TELEGRAM_DIALOG_FILTER_PRESET_MARKER: &str =
    "hepta_telegram_dialog_filter_presets_emit_main_filter_action";
pub const HEPTA_TELEGRAM_DIALOG_LIST_EMPTY_STATE_LOCAL_FILTER_MARKER: &str =
    "hepta_telegram_dialog_list_empty_state_local_filter_ready";
pub const HEPTA_TELEGRAM_DESKTOP_DOCK_RESTORE_LAZY_LOCAL_MARKER: &str =
    "hepta_telegram_desktop_dock_restore_lazy_local_ready";
pub const HEPTA_TELEGRAM_DESKTOP_SHELL_MARKER: &str = "hepta_telegram_desktop_shell_ready";
pub const HEPTA_TELEGRAM_MOBILE_STACK_NAVIGATION_LOCAL_MARKER: &str =
    "hepta_telegram_mobile_stack_navigation_local_ready";
pub const HEPTA_TELEGRAM_MOBILE_SHELL_MARKER: &str = "hepta_telegram_mobile_shell_ready";
pub const HEPTA_TELEGRAM_MOBILE_EVIDENCE_DENSITY_GUARD_MARKER: &str =
    "hepta_telegram_mobile_evidence_density_guard_ready";
pub const HEPTA_TELEGRAM_NAVIGATION_SPACES_TOGGLE_LOCAL_MARKER: &str =
    "hepta_telegram_navigation_spaces_toggle_local_ready";
pub const HEPTA_TELEGRAM_NAVIGATION_TOP_LEVEL_TAB_SELECTION_LOCAL_MARKER: &str =
    "hepta_telegram_navigation_top_level_tab_selection_local_ready";
pub const HEPTA_TELEGRAM_PROFILE_ICON_SETTINGS_NAVIGATION_LOCAL_MARKER: &str =
    "hepta_telegram_profile_icon_settings_navigation_local_ready";
pub const HEPTA_TELEGRAM_SETTINGS_CLOSE_PREVIOUS_SELECTION_LOCAL_MARKER: &str =
    "hepta_telegram_settings_close_previous_selection_local_ready";
pub const HEPTA_TELEGRAM_SPACES_BAR_ENTRY_SELECTION_LOCAL_MARKER: &str =
    "hepta_telegram_spaces_bar_entry_selection_local_ready";
pub const HEPTA_TELEGRAM_SPACES_BAR_SECONDARY_CLICK_LOCAL_MARKER: &str =
    "hepta_telegram_spaces_bar_secondary_click_local_ready";
pub const HEPTA_TELEGRAM_SPACES_BAR_EMPTY_FILTER_LOCAL_MARKER: &str =
    "hepta_telegram_spaces_bar_empty_filter_local_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_MEMBERSHIP_EDGE_LOCAL_MARKER: &str =
    "hepta_telegram_rooms_list_membership_edge_local_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_PAGINATION_ADAPTER_LOCAL_MARKER: &str =
    "hepta_telegram_rooms_list_pagination_adapter_local_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_LOAD_MORE_PAGINATION_PACKET_MARKER: &str =
    "hepta_telegram_rooms_list_load_more_pagination_packet_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_HEADER_SPACE_SCOPE_LOCAL_MARKER: &str =
    "hepta_telegram_rooms_list_header_space_scope_local_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_SECTION_UNREAD_AGGREGATE_LOCAL_ZERO_MARKER: &str =
    "hepta_telegram_rooms_list_section_unread_aggregate_local_zero_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_ALL_ROOMS_LOADED_LOCAL_UNKNOWN_MARKER: &str =
    "hepta_telegram_rooms_list_all_rooms_loaded_local_unknown_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_SPACE_PARENT_CACHE_LOCAL_MARKER: &str =
    "hepta_telegram_rooms_list_space_parent_cache_local_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_NAME_UPDATE_SELECTED_STATE_LOCAL_MARKER: &str =
    "hepta_telegram_rooms_list_name_update_selected_state_local_ready";
pub const HEPTA_TELEGRAM_ROOMS_LIST_REMOVED_ROOM_SELECTED_STATE_LOCAL_MARKER: &str =
    "hepta_telegram_rooms_list_removed_room_selected_state_local_ready";
pub const HEPTA_TELEGRAM_SPACE_UNREAD_FILTER_LOCAL_ZERO_MARKER: &str =
    "hepta_telegram_space_unread_filter_local_zero_ready";
pub const HEPTA_TELEGRAM_ROOM_HEADER_ACTIONS_MARKER: &str =
    "hepta_telegram_room_header_actions_local_only";
pub const HEPTA_TELEGRAM_ROOM_MENU_ELLIPSIS_MARKER: &str =
    "hepta_telegram_room_menu_ellipsis_visible";
pub const HEPTA_TELEGRAM_ROOM_ACTIONS_STRIP_MARKER: &str =
    "hepta_telegram_room_actions_strip_uses_base_link_path";
pub const HEPTA_TELEGRAM_ROOM_ACTIONS_CLOSE_LOCAL_EVIDENCE_MARKER: &str =
    "hepta_telegram_room_actions_close_local_evidence_ready";
pub const HEPTA_TELEGRAM_ROOM_LINK_COPY_HANDOFF_MARKER: &str =
    "hepta_telegram_room_link_copy_handoff_ready";
pub const HEPTA_TELEGRAM_ROOM_ACTIONS_INVITE_LEAVE_MARKER: &str =
    "hepta_telegram_room_actions_invite_leave_reuse_existing_modals";
pub const HEPTA_TELEGRAM_INVITE_USER_CONFIRMATION_MARKER: &str =
    "hepta_telegram_invite_user_confirmation_ready";
pub const HEPTA_TELEGRAM_TIMELINE_INVITE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_timeline_invite_confirmation_ready";
pub const HEPTA_TELEGRAM_INVITE_RESPONSE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_invite_response_confirmation_required";
pub const HEPTA_TELEGRAM_SPACE_LOBBY_JOIN_LEAVE_MODAL_MARKER: &str =
    "hepta_telegram_space_lobby_join_leave_modal_ready";
pub const HEPTA_TELEGRAM_SPACE_LOBBY_READ_SYNC_MARKER: &str =
    "hepta_telegram_space_lobby_read_sync_ready";
pub const HEPTA_TELEGRAM_SPACE_LOBBY_ROOM_LIST_LIFECYCLE_MARKER: &str =
    "hepta_telegram_space_lobby_room_list_lifecycle_ready";
pub const HEPTA_TELEGRAM_SPACE_LOBBY_EMPTY_STATE_READ_SYNC_MARKER: &str =
    "hepta_telegram_space_lobby_empty_state_read_sync_ready";
pub const HEPTA_TELEGRAM_SPACE_LOBBY_MEMBERSHIP_EDGE_LOCAL_MARKER: &str =
    "hepta_telegram_space_lobby_membership_edge_local_ready";
pub const HEPTA_TELEGRAM_ADD_ROOM_KNOCK_CONFIRMATION_MARKER: &str =
    "hepta_telegram_add_room_knock_confirmation_ready";
pub const HEPTA_TELEGRAM_ADD_ROOM_PREVIEW_CANCEL_MARKER: &str =
    "hepta_telegram_add_room_preview_cancel_ready";
pub const HEPTA_TELEGRAM_ADD_ROOM_MEMBERSHIP_EDGE_LOCAL_MARKER: &str =
    "hepta_telegram_add_room_membership_edge_local_ready";
pub const HEPTA_TELEGRAM_ADD_ROOM_RESTRICTED_JOIN_RULE_LOCAL_MARKER: &str =
    "hepta_telegram_add_room_restricted_join_rule_local_ready";
pub const HEPTA_TELEGRAM_ROOM_ACTIONS_TOGGLES_MARKER: &str =
    "hepta_telegram_room_actions_stateful_toggles_reuse_base_paths";
pub const HEPTA_TELEGRAM_ROOM_STATUS_CONFIRMATION_MARKER: &str =
    "hepta_telegram_room_status_confirmation_ready";
pub const HEPTA_TELEGRAM_ROOM_INFO_STRIP_MARKER: &str =
    "hepta_telegram_room_info_strip_uses_loaded_room_state";
pub const HEPTA_TELEGRAM_ROOM_MEMBERS_READ_MARKER: &str = "hepta_telegram_room_members_read_ready";
pub const HEPTA_TELEGRAM_ROOM_MEMBER_SYNC_READ_MARKER: &str =
    "hepta_telegram_room_member_sync_read_ready";
pub const HEPTA_TELEGRAM_ROOM_POWER_LEVELS_READ_MARKER: &str =
    "hepta_telegram_room_power_levels_read_ready";
pub const HEPTA_TELEGRAM_TYPING_NOTICE_SUBSCRIPTION_MARKER: &str =
    "hepta_telegram_typing_notice_subscription_ready";
pub const HEPTA_TELEGRAM_OWN_READ_RECEIPT_SUBSCRIPTION_MARKER: &str =
    "hepta_telegram_own_read_receipt_subscription_ready";
pub const HEPTA_TELEGRAM_PINNED_EVENTS_SUBSCRIPTION_MARKER: &str =
    "hepta_telegram_pinned_events_subscription_ready";
pub const HEPTA_TELEGRAM_ROOM_CONTEXT_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_room_context_local_surface_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_room_settings_local_surface_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_OPTION_SUMMARY_MARKER: &str =
    "hepta_telegram_room_settings_option_summary_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_OPTION_STAGING_LOCAL_MARKER: &str =
    "hepta_telegram_room_settings_option_staging_local_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_NAME_ID_CLIPBOARD_MARKER: &str =
    "hepta_telegram_room_settings_name_id_clipboard_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_MARKER: &str =
    "hepta_telegram_room_settings_permissions_clipboard_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_MEMBERS_CLIPBOARD_MARKER: &str =
    "hepta_telegram_room_settings_members_clipboard_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_IDENTITY_CLIPBOARD_MARKER: &str =
    "hepta_telegram_room_settings_identity_clipboard_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_LOADED_IDENTITY_MARKER: &str =
    "hepta_telegram_room_settings_loaded_identity_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_CLOSE_METADATA_MARKER: &str =
    "hepta_telegram_room_settings_close_metadata_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_METADATA_MARKER: &str =
    "hepta_telegram_room_settings_refresh_metadata_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_MARKER: &str =
    "hepta_telegram_room_settings_refresh_live_read_wiring_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_MARKER: &str =
    "hepta_telegram_room_settings_name_topic_live_write_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_CANONICAL_ALIAS_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_room_settings_canonical_alias_live_wiring_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_AVATAR_REMOVE_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_room_settings_avatar_remove_live_wiring_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_TOMBSTONE_LIVE_WRITE_MARKER: &str =
    "hepta_telegram_room_settings_tombstone_live_write_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_READONLY_BOUNDARY_MARKER: &str =
    "hepta_telegram_room_settings_readonly_boundary_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_room_settings_local_boundary_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_MARKER: &str =
    "hepta_telegram_room_settings_edit_controls_boundary_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_EDIT_INTENT_STAGING_MARKER: &str =
    "hepta_telegram_room_settings_edit_intent_staging_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_MARKER: &str =
    "hepta_telegram_room_settings_field_edit_intent_controls_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_RESULT_DETAIL_MARKER: &str =
    "hepta_telegram_room_settings_refresh_result_detail_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_MARKER: &str =
    "hepta_telegram_room_settings_mutation_preflight_detail_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_MARKER: &str =
    "hepta_telegram_room_settings_field_mutation_packet_drilldown_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_room_settings_field_mutation_contract_packet_ready";
pub const HEPTA_TELEGRAM_ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_room_settings_power_member_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_MODE_MARKER: &str =
    "hepta_telegram_message_search_mode_local_only";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_message_search_local_surface_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_EMPTY_CLOSE_LOCAL_MARKER: &str =
    "hepta_telegram_message_search_empty_close_local_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_PREVIEW_MARKER: &str =
    "hepta_telegram_message_search_loaded_preview_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_MARKER: &str =
    "hepta_telegram_message_search_loaded_timeline_boundary_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_METADATA_MARKER: &str =
    "hepta_telegram_message_search_loaded_metadata_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_MARKER: &str =
    "hepta_telegram_message_search_active_result_detail_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_MARKER: &str =
    "hepta_telegram_message_search_result_action_controls_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_MARKER: &str =
    "hepta_telegram_message_search_result_jump_loaded_match_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_THREAD_OPEN_MARKER: &str =
    "hepta_telegram_message_search_result_thread_open_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_MARKER: &str =
    "hepta_telegram_message_search_result_sender_profile_pane_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_MARKER: &str =
    "hepta_telegram_message_search_result_copy_clipboard_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_SOURCE_MODAL_MARKER: &str =
    "hepta_telegram_message_search_result_source_modal_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_MARKER: &str =
    "hepta_telegram_message_search_query_lifecycle_metadata_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_MARKER: &str =
    "hepta_telegram_message_search_server_context_boundary_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_MARKER: &str =
    "hepta_telegram_message_search_server_context_controls_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_MARKER: &str =
    "hepta_telegram_message_search_advanced_filter_controls_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_SCOPE_FILTERS_LIVE_MARKER: &str =
    "hepta_telegram_message_search_loaded_scope_filters_live_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_MARKER: &str =
    "hepta_telegram_message_search_server_preflight_controls_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_MARKER: &str =
    "hepta_telegram_message_search_server_packet_clipboard_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_message_search_matrix_contract_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_message_search_remote_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_PAGINATION_LIVE_MARKER: &str =
    "hepta_telegram_message_search_server_pagination_live_ready";
pub const HEPTA_TELEGRAM_SIDEBAR_MESSAGE_SEARCH_LOCAL_BUTTON_MARKER: &str =
    "hepta_telegram_sidebar_message_search_local_button_ready";
pub const HEPTA_TELEGRAM_SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_MARKER: &str =
    "hepta_telegram_sidebar_message_search_open_handoff_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_notifications_local_surface_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_OPTION_STAGING_LOCAL_MARKER: &str =
    "hepta_telegram_notifications_option_staging_local_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_MODE_WRITE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_notifications_mode_write_confirmation_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_LOADED_ATTENTION_MARKER: &str =
    "hepta_telegram_notifications_loaded_attention_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_MODE_CLIPBOARD_MARKER: &str =
    "hepta_telegram_notifications_mode_clipboard_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_MODE_TARGET_METADATA_MARKER: &str =
    "hepta_telegram_notifications_mode_target_metadata_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_CLOSE_REFRESH_METADATA_MARKER: &str =
    "hepta_telegram_notifications_close_refresh_metadata_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_notifications_local_boundary_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_notifications_timed_global_boundary_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_MARKER: &str =
    "hepta_telegram_notifications_pusher_keyword_boundary_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_MARKER: &str =
    "hepta_telegram_notifications_keyword_list_live_read_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_KEYWORD_MUTATION_LIVE_WRITE_MARKER: &str =
    "hepta_telegram_notifications_keyword_mutation_live_write_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_MARKER: &str =
    "hepta_telegram_notifications_pusher_status_live_read_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_ADVANCED_CONTROLS_MARKER: &str =
    "hepta_telegram_notifications_advanced_controls_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_notifications_advanced_detail_controls_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_RESULT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_notifications_result_detail_controls_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_notifications_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_RULE_PACKET_DRILLDOWN_MARKER: &str =
    "hepta_telegram_notifications_rule_packet_drilldown_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_RULE_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_notifications_rule_contract_packet_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_notifications_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_NOTIFICATIONS_RETRY_CONFIRMATION_MARKER: &str =
    "hepta_telegram_notifications_retry_confirmation_ready";
pub const HEPTA_TELEGRAM_ROOM_LOCAL_SURFACE_CLOSE_MARKER: &str =
    "hepta_telegram_room_local_surface_close_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_PLACEHOLDER_MARKER: &str =
    "hepta_telegram_composer_attachment_placeholder_local_only";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_composer_attachment_local_surface_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_OPTION_STAGING_LOCAL_MARKER: &str =
    "hepta_telegram_composer_attachment_option_staging_local_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_composer_attachment_camera_contact_local_boundary_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_HANDOFF_CONFIRMATION_MARKER: &str =
    "hepta_telegram_composer_attachment_handoff_confirmation_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SEND_HANDOFF_MARKER: &str =
    "hepta_telegram_composer_attachment_send_handoff_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_PRE_SEND_REVIEW_MARKER: &str =
    "hepta_telegram_composer_attachment_pre_send_review_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTED_FILE_PREVIEW_MARKER: &str =
    "hepta_telegram_composer_attachment_selected_file_preview_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTED_IMAGE_METADATA_MARKER: &str =
    "hepta_telegram_composer_attachment_selected_image_metadata_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_MAIN_SEND_GUARD_MARKER: &str =
    "hepta_telegram_composer_attachment_main_send_guard_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTION_REPLACEMENT_PRESERVE_MARKER: &str =
    "hepta_telegram_composer_attachment_selection_replacement_preserve_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_LIFECYCLE_METADATA_MARKER: &str =
    "hepta_telegram_composer_attachment_review_lifecycle_metadata_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_SEND_SINGLE_SUBMIT_MARKER: &str =
    "hepta_telegram_composer_attachment_review_send_single_submit_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_DISCARD_CLOSE_IDEMPOTENT_MARKER: &str =
    "hepta_telegram_composer_attachment_review_discard_close_idempotent_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_CAPTION_REPLY_CONTEXT_MARKER: &str =
    "hepta_telegram_composer_attachment_caption_reply_context_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_FILE_VALIDATION_LOCAL_ERROR_MARKER: &str =
    "hepta_telegram_composer_attachment_file_validation_local_error_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_VALIDATION_ERROR_RECOVERY_MARKER: &str =
    "hepta_telegram_composer_attachment_validation_error_recovery_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_SEND_OPERATION_STATUS_LOCAL_MARKER: &str =
    "hepta_telegram_attachment_send_operation_status_local_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_SEND_RESULT_BRIDGE_MARKER: &str =
    "hepta_telegram_attachment_send_result_bridge_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_QUEUE_FAILURE_RECOVERY_MARKER: &str =
    "hepta_telegram_attachment_queue_failure_recovery_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_QUEUE_FAILURE_RECOVERY_COPY_MARKER: &str =
    "hepta_telegram_attachment_queue_failure_recovery_copy_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_MARKER: &str =
    "hepta_telegram_attachment_send_failure_retry_confirmation_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_attachment_true_queue_control_local_boundary_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_MARKER: &str =
    "hepta_telegram_attachment_accepted_queue_actions_row_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_MARKER: &str =
    "hepta_telegram_attachment_accepted_queue_timeline_cancel_bridge_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_LOCAL_SEND_ABORT_RESULT_MARKER: &str =
    "hepta_telegram_attachment_local_send_abort_result_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_PER_FILE_STATUS_CONTROLS_MARKER: &str =
    "hepta_telegram_attachment_per_file_status_controls_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_MARKER: &str =
    "hepta_telegram_attachment_per_file_queue_drilldown_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_attachment_sdk_queue_contract_packet_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_attachment_queue_progress_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_attachment_send_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_MARKER: &str =
    "hepta_telegram_attachment_multi_file_queue_boundary_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_TIMELINE_SEND_STATE_MARKER: &str =
    "hepta_telegram_attachment_timeline_send_state_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_TIMELINE_CANCEL_LOCAL_SEND_MARKER: &str =
    "hepta_telegram_attachment_timeline_cancel_local_send_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_STATUS_TAXONOMY_LOCAL_MARKER: &str =
    "hepta_telegram_attachment_status_taxonomy_local_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_REVIEW_ROW_COMPACT_FIT_MARKER: &str =
    "hepta_telegram_attachment_review_row_compact_fit_ready";
pub const HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_MOBILE_PICKER_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_attachment_mobile_picker_controls_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_MARKER: &str =
    "hepta_telegram_attachment_mobile_share_sheet_boundary_ready";
pub const HEPTA_TELEGRAM_ATTACHMENT_MOBILE_ACTION_DENSITY_MARKER: &str =
    "hepta_telegram_attachment_mobile_action_density_ready";
pub const HEPTA_TELEGRAM_COMPOSER_EMOJI_PLACEHOLDER_MARKER: &str =
    "hepta_telegram_composer_emoji_placeholder_local_only";
pub const HEPTA_TELEGRAM_COMPOSER_EMOJI_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_composer_emoji_local_surface_ready";
pub const HEPTA_TELEGRAM_COMPOSER_EMOJI_SEND_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_composer_emoji_send_local_boundary_ready";
pub const HEPTA_TELEGRAM_COMPOSER_EMOJI_LIFECYCLE_METADATA_MARKER: &str =
    "hepta_telegram_composer_emoji_lifecycle_metadata_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_PLACEHOLDER_MARKER: &str =
    "hepta_telegram_composer_voice_placeholder_local_only";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_composer_voice_local_surface_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_DESKTOP_AUDIO_HANDOFF_MARKER: &str =
    "hepta_telegram_composer_voice_desktop_audio_handoff_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_AUDIO_FILE_SURFACE_MARKER: &str =
    "hepta_telegram_composer_voice_audio_file_surface_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_OPTION_STAGING_LOCAL_MARKER: &str =
    "hepta_telegram_composer_voice_option_staging_local_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_OPTION_STAGING_AUDIO_MARKER: &str =
    "hepta_telegram_composer_voice_option_staging_audio_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_LOCAL_BLOCKED_MARKER: &str =
    "hepta_telegram_composer_voice_send_local_blocked_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_AUDIO_HANDOFF_MARKER: &str =
    "hepta_telegram_composer_voice_send_audio_handoff_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_composer_voice_send_live_wiring_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_PERMISSION_RECORDING_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_composer_voice_permission_recording_local_boundary_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDING_BOUNDARY_MARKER: &str =
    "hepta_telegram_composer_voice_recording_boundary_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_SELECTED_AUDIO_METADATA_MARKER: &str =
    "hepta_telegram_composer_voice_selected_audio_metadata_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_SELECTED_AUDIO_WAVEFORM_CODEC_MARKER: &str =
    "hepta_telegram_composer_voice_selected_audio_waveform_codec_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_LIFECYCLE_METADATA_MARKER: &str =
    "hepta_telegram_composer_voice_lifecycle_metadata_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_CONFIRMATION_CANCEL_METADATA_MARKER: &str =
    "hepta_telegram_composer_voice_confirmation_cancel_metadata_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_WAVEFORM_CODEC_BOUNDARY_MARKER: &str =
    "hepta_telegram_composer_voice_recorder_waveform_codec_boundary_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_STATUS_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_voice_recorder_status_controls_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_CAPTURE_LIFECYCLE_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_voice_capture_lifecycle_controls_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_MOBILE_PICKER_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_voice_mobile_picker_controls_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_REVIEW_PLAYBACK_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_voice_review_playback_controls_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_REVIEW_PLAYBACK_OPENER_MARKER: &str =
    "hepta_telegram_composer_voice_review_playback_opener_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_REVIEW_DROP_PENDING_AUDIO_MARKER: &str =
    "hepta_telegram_composer_voice_review_drop_pending_audio_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_voice_send_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_MARKER: &str =
    "hepta_telegram_composer_voice_recorder_lifecycle_drilldown_packet_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_TYPED_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_composer_voice_recorder_typed_contract_packet_ready";
pub const HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_composer_voice_recorder_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_COMPOSER_LOCAL_SURFACE_CLOSE_MARKER: &str =
    "hepta_telegram_composer_local_surface_close_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_composer_mention_local_surface_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_OPTION_STAGING_LOCAL_MARKER: &str =
    "hepta_telegram_composer_mention_option_staging_local_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_CACHED_SELECTION_MARKER: &str =
    "hepta_telegram_composer_mention_cached_selection_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_LOADED_IDENTITY_MARKER: &str =
    "hepta_telegram_composer_mention_loaded_identity_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_CANDIDATE_ROWS_MARKER: &str =
    "hepta_telegram_composer_mention_local_candidate_rows_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_DUPLICATE_HINTS_MARKER: &str =
    "hepta_telegram_composer_mention_local_duplicate_hints_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_LIFECYCLE_METADATA_MARKER: &str =
    "hepta_telegram_composer_mention_lifecycle_metadata_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_KEYBOARD_SELECTION_MARKER: &str =
    "hepta_telegram_composer_mention_keyboard_selection_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_RICH_PICKER_BOUNDARY_MARKER: &str =
    "hepta_telegram_composer_mention_rich_picker_boundary_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_DIRECTORY_DISAMBIGUATION_BOUNDARY_MARKER: &str =
    "hepta_telegram_composer_mention_directory_disambiguation_boundary_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_RICH_DIRECTORY_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_mention_rich_directory_controls_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_DIRECTORY_SEARCH_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_composer_mention_directory_search_live_wiring_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_DIRECTORY_RESULT_PROMOTION_LIVE_MARKER: &str =
    "hepta_telegram_composer_mention_directory_result_promotion_live_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_HOVER_CARD_SNAPSHOT_LIVE_MARKER: &str =
    "hepta_telegram_composer_mention_hover_card_snapshot_live_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_PILL_TRAY_LIVE_MARKER: &str =
    "hepta_telegram_composer_mention_local_pill_tray_live_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_mention_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_composer_mention_send_local_boundary_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_PAYLOAD_METADATA_MARKER: &str =
    "hepta_telegram_composer_mention_send_payload_metadata_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_composer_mention_send_live_wiring_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_SCOPE_CONTROLS_MARKER: &str =
    "hepta_telegram_composer_mention_payload_scope_controls_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_DRILLDOWN_PACKET_MARKER: &str =
    "hepta_telegram_composer_mention_payload_drilldown_packet_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_composer_mention_payload_typed_contract_packet_ready";
pub const HEPTA_TELEGRAM_COMPOSER_MENTION_REMOTE_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_composer_mention_remote_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_COMPOSER_SEND_SHORTCUT_LOCAL_PREFERENCE_MARKER: &str =
    "hepta_telegram_composer_send_shortcut_local_preference_ready";
pub const HEPTA_TELEGRAM_MESSAGE_SEND_OPERATION_STATUS_LOCAL_MARKER: &str =
    "hepta_telegram_message_send_operation_status_local_ready";
pub const HEPTA_TELEGRAM_COMPOSER_TYPING_NOTICE_SEND_MARKER: &str =
    "hepta_telegram_composer_typing_notice_send_ready";
pub const HEPTA_TELEGRAM_LOCATION_SEND_CONFIRMATION_MARKER: &str =
    "hepta_telegram_location_send_confirmation_ready";
pub const HEPTA_TELEGRAM_LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_MARKER: &str =
    "hepta_telegram_live_location_continuous_updates_boundary_ready";
pub const HEPTA_TELEGRAM_PROFILE_READ_RECEIPT_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_profile_read_receipt_local_surface_ready";
pub const HEPTA_TELEGRAM_PROFILE_MEMBER_READ_MARKER: &str =
    "hepta_telegram_profile_member_read_ready";
pub const HEPTA_TELEGRAM_PROFILE_ACCOUNT_IDENTITY_CLIPBOARD_MARKER: &str =
    "hepta_telegram_profile_account_identity_clipboard_ready";
pub const HEPTA_TELEGRAM_PROFILE_DIRECT_MESSAGE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_profile_direct_message_confirmation_ready";
pub const HEPTA_TELEGRAM_DIRECT_MESSAGE_CREATE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_direct_message_create_confirmation_ready";
pub const HEPTA_TELEGRAM_PROFILE_IGNORE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_profile_ignore_confirmation_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_DISPLAY_NAME_STAGING_MARKER: &str =
    "hepta_telegram_account_display_name_staging_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_DISPLAY_NAME_CONFIRMATION_MARKER: &str =
    "hepta_telegram_account_display_name_confirmation_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_DEVICE_SELF_CHECK_MARKER: &str =
    "hepta_telegram_account_device_self_check_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_account_avatar_upload_local_surface_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_OPTION_STAGING_LOCAL_MARKER: &str =
    "hepta_telegram_account_avatar_upload_option_staging_local_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_MARKER: &str =
    "hepta_telegram_account_avatar_upload_selected_file_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_MARKER: &str =
    "hepta_telegram_account_avatar_upload_selected_image_metadata_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_MARKER: &str =
    "hepta_telegram_account_avatar_upload_decode_probe_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_PIXEL_DECODE_LIVE_MARKER: &str =
    "hepta_telegram_account_avatar_upload_pixel_decode_live_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_account_avatar_upload_live_wiring_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_DIRECT_MXC_SET_MARKER: &str =
    "hepta_telegram_account_avatar_direct_mxc_set_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_MARKER: &str =
    "hepta_telegram_account_avatar_upload_lifecycle_metadata_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_MARKER: &str =
    "hepta_telegram_account_avatar_upload_retry_confirmation_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_MARKER: &str =
    "hepta_telegram_account_avatar_upload_crop_editor_boundary_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_MARKER: &str =
    "hepta_telegram_account_avatar_upload_editor_controls_row_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_MARKER: &str =
    "hepta_telegram_account_avatar_upload_source_preview_controls_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_MARKER: &str =
    "hepta_telegram_account_avatar_upload_source_editor_drilldown_packet_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_account_avatar_upload_source_editor_typed_contract_packet_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_account_avatar_upload_source_editor_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_SOURCE_PATH_CLIPBOARD_MARKER: &str =
    "hepta_telegram_account_avatar_source_path_clipboard_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_MARKER: &str =
    "hepta_telegram_account_avatar_upload_source_path_clipboard_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_account_avatar_upload_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_account_avatar_upload_local_boundary_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_DELETE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_account_avatar_delete_confirmation_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_AVATAR_DELETE_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_account_avatar_delete_live_wiring_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_account_management_local_surface_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_OPTION_STAGING_LOCAL_MARKER: &str =
    "hepta_telegram_account_management_option_staging_local_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOADED_IDENTITY_MARKER: &str =
    "hepta_telegram_account_management_loaded_identity_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_account_management_live_wiring_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DISPLAY_NAME_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_account_management_display_name_live_wiring_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_account_management_device_directory_live_wiring_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_RENAME_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_account_management_current_device_rename_live_wiring_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_BROWSER_PORTAL_HANDOFF_MARKER: &str =
    "hepta_telegram_account_management_browser_portal_handoff_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_MARKER: &str =
    "hepta_telegram_account_management_lifecycle_metadata_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_MARKER: &str =
    "hepta_telegram_account_management_refresh_confirmation_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_MARKER: &str =
    "hepta_telegram_account_management_session_revoke_boundary_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_MARKER: &str =
    "hepta_telegram_account_management_session_actions_row_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_MARKER: &str =
    "hepta_telegram_account_management_device_directory_controls_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_MARKER: &str =
    "hepta_telegram_account_management_device_directory_controls_row_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_MARKER: &str =
    "hepta_telegram_account_management_current_device_metadata_controls_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_MARKER: &str =
    "hepta_telegram_account_management_current_device_metadata_controls_row_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_MARKER: &str =
    "hepta_telegram_account_management_current_device_verification_clipboard_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_MARKER: &str =
    "hepta_telegram_account_management_current_device_id_clipboard_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_MARKER: &str =
    "hepta_telegram_account_management_current_device_display_name_clipboard_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_MARKER: &str =
    "hepta_telegram_account_management_current_session_clipboard_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_MARKER: &str =
    "hepta_telegram_account_management_current_device_source_clipboard_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_account_management_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_MARKER: &str =
    "hepta_telegram_account_management_preflight_detail_controls_row_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_MARKER: &str =
    "hepta_telegram_account_management_session_device_drilldown_packet_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_account_management_session_device_typed_contract_packet_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_account_management_session_device_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_account_management_local_boundary_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_LOCAL_SURFACE_CLOSE_MARKER: &str =
    "hepta_telegram_account_local_surface_close_ready";
pub const HEPTA_TELEGRAM_ACCOUNT_LOGOUT_CONFIRMATION_MARKER: &str =
    "hepta_telegram_account_logout_confirmation_ready";
pub const HEPTA_TELEGRAM_MEDIA_MESSAGE_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_media_message_local_surface_ready";
pub const HEPTA_TELEGRAM_MEDIA_MESSAGE_BLOCKED_ACTIONS_MARKER: &str =
    "hepta_telegram_media_message_blocked_actions_ready";
pub const HEPTA_TELEGRAM_MEDIA_SAVE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_media_save_confirmation_ready";
pub const HEPTA_TELEGRAM_MEDIA_SAVE_RETRY_CONFIRMATION_MARKER: &str =
    "hepta_telegram_media_save_retry_confirmation_ready";
pub const HEPTA_TELEGRAM_MEDIA_DOWNLOAD_METADATA_MARKER: &str =
    "hepta_telegram_media_download_metadata_preview_ready";
pub const HEPTA_TELEGRAM_MEDIA_METADATA_CLIPBOARD_MARKER: &str =
    "hepta_telegram_media_metadata_clipboard_ready";
pub const HEPTA_TELEGRAM_MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_MARKER: &str =
    "hepta_telegram_media_save_dialog_lifecycle_metadata_ready";
pub const HEPTA_TELEGRAM_MEDIA_SAVE_DESTINATION_METADATA_MARKER: &str =
    "hepta_telegram_media_save_destination_metadata_ready";
pub const HEPTA_TELEGRAM_MEDIA_ENCRYPTED_METADATA_LOCAL_MARKER: &str =
    "hepta_telegram_media_encrypted_metadata_local_ready";
pub const HEPTA_TELEGRAM_MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_MARKER: &str =
    "hepta_telegram_media_encrypted_image_metadata_local_ready";
pub const HEPTA_TELEGRAM_MEDIA_FETCH_CACHE_READ_MARKER: &str =
    "hepta_telegram_media_fetch_cache_read_ready";
pub const HEPTA_TELEGRAM_MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_MARKER: &str =
    "hepta_telegram_media_download_playback_local_boundary_ready";
pub const HEPTA_TELEGRAM_MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_MARKER: &str =
    "hepta_telegram_media_inline_playback_queue_boundary_ready";
pub const HEPTA_TELEGRAM_MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_MARKER: &str =
    "hepta_telegram_media_inline_player_disabled_controls_ready";
pub const HEPTA_TELEGRAM_MEDIA_CODEC_TRANSCODE_CONTROLS_MARKER: &str =
    "hepta_telegram_media_codec_transcode_controls_ready";
pub const HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_STATUS_BOUNDARY_MARKER: &str =
    "hepta_telegram_media_save_result_status_boundary_ready";
pub const HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_MARKER: &str =
    "hepta_telegram_media_save_result_recovery_controls_ready";
pub const HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_MARKER: &str =
    "hepta_telegram_media_save_result_recovery_controls_row_ready";
pub const HEPTA_TELEGRAM_MEDIA_CACHED_SAVED_FILE_STATUS_MARKER: &str =
    "hepta_telegram_media_cached_saved_file_status_ready";
pub const HEPTA_TELEGRAM_MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_MARKER: &str =
    "hepta_telegram_media_save_preflight_detail_controls_ready";
pub const HEPTA_TELEGRAM_MEDIA_OPERATION_PACKET_DRILLDOWN_MARKER: &str =
    "hepta_telegram_media_operation_packet_drilldown_ready";
pub const HEPTA_TELEGRAM_MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_media_playback_queue_contract_packet_ready";
pub const HEPTA_TELEGRAM_MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_media_playback_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_POLL_MESSAGE_PREVIEW_LOCAL_MARKER: &str =
    "hepta_telegram_poll_message_preview_local_ready";
pub const HEPTA_TELEGRAM_POLL_ANSWER_PREVIEW_RESULT_PACKET_MARKER: &str =
    "hepta_telegram_poll_answer_preview_result_packet_ready";
pub const HEPTA_TELEGRAM_IMAGE_VIEWER_LOCAL_CONTROLS_MARKER: &str =
    "hepta_telegram_image_viewer_local_controls_ready";
pub const HEPTA_TELEGRAM_LINK_PREVIEW_LOCAL_CONTROLS_MARKER: &str =
    "hepta_telegram_link_preview_local_controls_ready";
pub const HEPTA_TELEGRAM_LINK_PREVIEW_LOADED_METADATA_MARKER: &str =
    "hepta_telegram_link_preview_loaded_metadata_ready";
pub const HEPTA_TELEGRAM_URL_PREVIEW_READ_MARKER: &str = "hepta_telegram_url_preview_read_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_LOCAL_PREVIEW_MARKER: &str =
    "hepta_telegram_matrix_link_local_preview_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_LIVE_READ_WIRING_MARKER: &str =
    "hepta_telegram_matrix_link_preview_live_read_wiring_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_LOADED_ALIAS_NAVIGATION_MARKER: &str =
    "hepta_telegram_matrix_link_loaded_alias_navigation_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_MARKER: &str =
    "hepta_telegram_matrix_link_loaded_event_local_jump_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_MARKER: &str =
    "hepta_telegram_matrix_link_current_room_event_pagination_live_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_MARKER: &str =
    "hepta_telegram_matrix_link_loaded_event_context_metadata_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_MARKER: &str =
    "hepta_telegram_matrix_link_loaded_event_source_modal_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_LOCAL_MARKER: &str =
    "hepta_telegram_matrix_link_unknown_target_local_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_BOUNDARY_MARKER: &str =
    "hepta_telegram_matrix_link_unknown_target_boundary_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_TARGET_METADATA_MARKER: &str =
    "hepta_telegram_matrix_link_target_metadata_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_RESULT_METADATA_MARKER: &str =
    "hepta_telegram_matrix_link_preview_result_metadata_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_FAILURE_METADATA_MARKER: &str =
    "hepta_telegram_matrix_link_preview_failure_metadata_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_MARKER: &str =
    "hepta_telegram_matrix_link_preview_retry_confirmation_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_MARKER: &str =
    "hepta_telegram_matrix_link_server_context_boundary_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_SERVER_CONTEXT_REFRESH_LIVE_MARKER: &str =
    "hepta_telegram_matrix_link_server_context_refresh_live_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_CONTEXT_ACTIONS_ROW_MARKER: &str =
    "hepta_telegram_matrix_link_context_actions_row_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_ROOM_OR_ALIAS_JOIN_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_matrix_link_room_or_alias_join_live_wiring_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_ROOM_OR_ALIAS_KNOCK_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_matrix_link_room_or_alias_knock_live_wiring_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_USER_INVITE_LIVE_WIRING_MARKER: &str =
    "hepta_telegram_matrix_link_user_invite_live_wiring_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_MARKER: &str =
    "hepta_telegram_matrix_link_browser_handoff_confirmation_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_SCOPE_CONTROLS_MARKER: &str =
    "hepta_telegram_matrix_link_route_scope_controls_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_MARKER: &str =
    "hepta_telegram_matrix_link_route_drilldown_packet_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_MARKER: &str =
    "hepta_telegram_matrix_link_route_result_contract_packet_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_MARKER: &str =
    "hepta_telegram_matrix_link_route_result_taxonomy_packet_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_ROOM_TARGET_CLIPBOARD_MARKER: &str =
    "hepta_telegram_matrix_link_room_target_clipboard_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_VIA_SERVERS_CLIPBOARD_MARKER: &str =
    "hepta_telegram_matrix_link_via_servers_clipboard_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_EVENT_ID_CLIPBOARD_MARKER: &str =
    "hepta_telegram_matrix_link_event_id_clipboard_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_MARKER: &str =
    "hepta_telegram_matrix_link_preview_metadata_clipboard_ready";
pub const HEPTA_TELEGRAM_MATRIX_LINK_UNRESOLVED_DETAIL_MARKER: &str =
    "hepta_telegram_matrix_link_unresolved_detail_ready";
pub const HEPTA_TELEGRAM_EXTERNAL_LINK_CONFIRMATION_MARKER: &str =
    "hepta_telegram_external_link_confirmation_ready";
pub const HEPTA_TELEGRAM_EVENT_SOURCE_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_event_source_local_surface_ready";
pub const HEPTA_TELEGRAM_EVENT_SOURCE_CLIPBOARD_COPY_MARKER: &str =
    "hepta_telegram_event_source_clipboard_copy_ready";
pub const HEPTA_TELEGRAM_EVENT_SOURCE_LOADED_METADATA_MARKER: &str =
    "hepta_telegram_event_source_loaded_metadata_ready";
pub const HEPTA_TELEGRAM_MESSAGE_COPY_LOCAL_SURFACE_MARKER: &str =
    "hepta_telegram_message_copy_local_surface_ready";
pub const HEPTA_TELEGRAM_MESSAGE_COPY_LOADED_METADATA_MARKER: &str =
    "hepta_telegram_message_copy_loaded_metadata_ready";
pub const HEPTA_TELEGRAM_MESSAGE_PIN_CONFIRMATION_MARKER: &str =
    "hepta_telegram_message_pin_confirmation_ready";
pub const HEPTA_TELEGRAM_MESSAGE_DELETE_CONFIRMATION_MARKER: &str =
    "hepta_telegram_message_delete_confirmation_ready";

pub const HEPTA_TELEGRAM_BASE_CAPABILITIES: &[HeptaTelegramBaseCapability] = &[
    HeptaTelegramBaseCapability {
        telegram_surface: "base gap product runway",
        base_module: "HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "All 12 explicit base gaps have a machine-checkable product runway entry that records the current UI-safe path, remaining product gap, and next UI-safe step. The runway is planning/evidence metadata only and sends no Matrix request, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth request, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "dialog list",
        base_module: "RoomsList",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "room filtering, latest message preview, unread/mention, marked unread, favorite, low priority, tags, and empty/filter edge states reuse the existing RoomsList, RoomDisplayFilter, RoomListService, SlidingSync, and guarded room-status request paths; product-completeness edges such as all_rooms_loaded, section aggregate unread, full parent-chain filtering, selected-room rename/remove propagation, and re-knock/cancel-prior-knock remain tracked as local evidence rather than an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message thread",
        base_module: "RoomScreen",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "timeline, pagination, threads, typing notices, read receipts, pinned events, media and previews",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "thread summary read path",
        base_module: "RoomScreen + ThreadRootSummary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Thread root summaries use existing Matrix FetchThreadSummaryDetails and CreateThreadTimeline read/open paths for reply count, latest reply, and opening the thread; the summary does not send a message, edit, room-state mutation, or membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "thread open timeline read path",
        base_module: "RoomScreen + ThreadRootSummary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Opening a thread dispatches RoomsListAction::Selected for a thread-focused timeline and, when needed, reuses the existing Matrix CreateThreadTimeline read/open path; this does not create a room, send a message, edit, redact, change room-state, or perform membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "reply preview event-details read path",
        base_module: "RoomScreen + RepliedToMessage",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Inline reply previews use the existing Matrix FetchDetailsForEvent read path when the replied-to event details are unavailable; loading/error copy makes that read-only path explicit and does not send a message, edit, redaction, room-state mutation, or membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "sender profile event-details read path",
        base_module: "Avatar + RoomScreen + SmallStateEvent",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Timeline sender avatars and display names use loaded sender_profile data, local user_profile_cache fallback, and when unavailable the existing Matrix FetchDetailsForEvent read path; fallback UI makes this read-only path explicit and sends no message, edit, redaction, profile/account mutation, room-state mutation, or membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "space lobby empty/filter read-sync state",
        base_module: "SpaceLobbyScreen + SpaceService",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Space lobby loading, empty-tree, and filter-empty states reuse existing SpaceService GetDetailedChildren and GetTopLevelSpaceDetails read/sync results plus cached children; they do not send Matrix search, JoinRoom, LeaveSpace, InviteUser, membership mutation, or room-state mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "rooms list pagination adapter local state",
        base_module: "RoomsList + RoomListService + PaginateTimeline",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomsList status evidence explains that RoomListService entries_with_dynamic_adapters(usize::MAX) feeds the loaded list locally and that there is no Load more rooms UI, no room-list pagination request, and no room-list pagination mutation. The status row now renders a Room-list Load More pagination packet with loaded row counts, server_max_hint, selected-space child pagination status, load_more_button_slot not_rendered, explicit_cursor_slot not_exposed, inflight_slot not_tracked_by_rooms_list, error_slot popup-only, retry_slot not_built, and latest_preview_pagination_source Matrix_PaginateTimeline_read_only. Existing SpaceService child pagination remains service-driven read-sync, visible room rows may prefetch latest-message previews through the existing Matrix PaginateTimeline read path only, and rendering sends no user-triggered room-list pagination, message, edit, room-state, or membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "rooms list header selected-space local evidence",
        base_module: "RoomsListHeader + NavigationBarAction::TabSelected",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomsListHeader copies TabSelected(SelectedTab::Space) into the local header title, resets the title to Chats for non-space tabs, and updates only the local evidence label. Header title/reset sends no SpaceService fetch, Matrix search, room-list pagination, message, room-state mutation, membership request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "desktop dock restore lazy tab local evidence",
        base_module: "MainDesktopUI + SavedDockState + RobrixDock",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MainDesktopUI restores SavedDockState from AppState locally, initializes only visible restored Dock tabs, and defers hidden tab content until tab press, drop, or close exposes it; dock restore sends no Matrix search, room-list pagination, message, room-state mutation, membership request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mobile stack navigation local evidence",
        base_module: "HomeScreen + StackNavigation + RoomScreen pool",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "HomeScreen mobile navigation pushes selected rooms into StackNavigation using 16 dedicated RoomScreen-backed room views, preserves previous selected_room values in a local mobile_room_nav_stack, restores them on StackNavigation pop, and sends no Matrix search, message, room-state mutation, membership request, or live mutation request from push/pop rendering",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "navigation spaces toggle local evidence",
        base_module: "NavigationTabBar + SpacesBarWrapper",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "NavigationTabBar ToggleSpacesBarButton only flips the local SpacesBarWrapper show/hide state and emits ToggleSpacesBar; it does not select a space, does not fetch SpaceService children, and sends no Matrix search, message, room-state mutation, membership request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "navigation top-level tab selection local evidence",
        base_module: "NavigationTabBar + HomeScreen PageFlip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "NavigationTabBar Home and Add Room buttons only update local selected-tab visuals and emit GoToHome or GoToAddRoom; HomeScreen records previous_selection, updates selected_tab, emits TabSelected, and switches the local PageFlip. AddRoom Join/Knock requests remain behind AddRoomScreen confirmation guards, and top-level tab selection sends no Matrix search, room-list pagination, message, room-state mutation, membership request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "spaces bar entry selection local/read evidence",
        base_module: "SpacesBar + HomeScreen + RoomsList",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "SpacesBarAction::ButtonClicked records selected_space locally, redraws selected styling, and emits NavigationBarAction::GoToSpace; HomeScreen converts that to TabSelected(SelectedTab::Space), while RoomsList filtering reuses cached SpaceService child/subspace maps or existing read-sync state. Entry selection does not directly fetch SpaceService children and sends no Matrix search, message, room-state mutation, membership request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "spaces bar secondary-click local no-menu evidence",
        base_module: "SpacesBarEntry + SpacesBar",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "SpacesBarEntry right-click / long-press emits SpacesBarAction::ButtonSecondaryClicked with the space identity, and SpacesBar consumes it as a local no-op with no context menu, SpaceService fetch, Matrix search, message, room-state mutation, membership request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "spaces bar empty/filter local evidence",
        base_module: "SpacesBar + RoomDisplayFilter",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "SpacesBarStatus empty and filter-empty states render the local all_joined_spaces map and RoomDisplayFilter keyword matching only; update_displayed_spaces rebuilds displayed_spaces locally and sends no Matrix search, direct SpaceService child fetch, message, room-state mutation, membership request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "rooms list section unread aggregate local zero state",
        base_module: "RoomsList + CollapsibleHeader + loaded room state",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomsList People and Rooms section headers pass local zero placeholders for aggregate unread mention badges because a running section aggregate is not maintained yet; the status row renders a People/Rooms unread/mention aggregate packet with loaded row totals, manual-unread counts, selected-space scope, header_badge_source local_zero_placeholder, aggregate_refresh_slot not_built, muted/low-priority policy not_defined, and parent-chain attribution partial-cache-only. individual room rows still use loaded per-room unread state, and header rendering sends no aggregate scan, read receipt, message, room-state, or membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "rooms list all rooms loaded local unknown state",
        base_module: "RoomsList + RestoreStatusView + RoomScreen + InviteScreen",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomsList all_rooms_loaded currently stays a local unknown/false restore signal until room-list completeness detection is implemented; RoomScreen and InviteScreen show RestoreStatusView waiting evidence from that flag, and the status sends no room-list pagination request, Matrix search, message, room-state, or membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "rooms list space parent cache local state",
        base_module: "RoomsList + SpaceService cached child maps",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Selected-space filtering uses cached SpaceService direct child room and subspace maps recursively while JoinedRoomInfo does not store every parent chain yet; row filtering itself sends no Matrix search, message, room-state, or membership mutation, and cache misses rely on the existing SpaceService read-sync path instead of ad hoc list mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "rooms list name update selected state local evidence",
        base_module: "RoomsList + SelectedRoom",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomsList UpdateRoomName refreshes loaded joined/invited room rows and local filter membership from synced room-name state while the SelectedRoom broadcast needed to rename every Dock tab and StackNav header remains unwired; the evidence sends no Matrix room-state mutation, message, membership request, or live rename request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "rooms list removed room selected state local evidence",
        base_module: "RoomsList + SelectedRoom stale-focus clear + LoadingScreen TODO",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomsList RemoveRoom removes left, kicked, or banned rooms from the loaded local list, clears AppState selected-room focus when the removed room was active, and renders a selected-room removed/rejoin packet with removed room id, membership state, FocusNone, selected-space scope, replacement_ui_slot not_wired, rejoin_request_slot not_built, and stale-event policy; list rendering and focus clearing send no Matrix JoinRoom, LeaveRoom, Knock, message, room-state mutation, or membership request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "space unread filter local zero state",
        base_module: "RoomDisplayFilter + RoomsList + JoinedSpaceInfo",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomsList status evidence exposes a space unread/mention aggregate packet: JoinedSpaceInfo unread_mentions and unread_messages remain local zero placeholders, so is:unread and is:mention filters over spaces use the room-display-filter zero source with aggregate_refresh_slot not_built. The packet and filters do not fetch aggregate unread counts, send read receipts, send messages, mutate room-state, or perform membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "unread badge count read path",
        base_module: "RoomScreen + JumpToBottomButton",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Unread badge count refresh uses the existing Matrix GetNumberUnreadMessages read path after appended timeline events; JumpToBottomButton only reflects Unknown, 0, and known counts in local badge state, and updates the jump-to-bottom badge and room list count without SetUnreadFlag, message, room-state mutation, retry, cancel, or membership request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "successor room details read path",
        base_module: "RoomScreen + TombstoneFooter",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Tombstoned rooms use the existing Matrix GetSuccessorRoomDetails read path to fetch replacement room preview details for TombstoneFooter; the footer does not send JoinRoom, Knock, membership, message, or room-state mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room preview read path",
        base_module: "AddRoomScreen + RoomPreviewCache",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Add Room search uses the existing Matrix GetRoomPreview read path to fetch room/space preview details; preview search and cancel do not send JoinRoom, Knock, membership, message, or room-state mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "avatar fetch read path",
        base_module: "Avatar + avatar_cache",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Timeline, room, profile, and space avatars use the existing Matrix FetchAvatar cache path for known MXC URIs; Avatar and avatar_cache keep fallback initials, cache hits, failures, Requested, and Known(None) states local and do not send SetAvatar, FetchMedia, profile mutation, account mutation, membership, message, or room-state mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "pinned events subscription read path",
        base_module: "RoomScreen + room info strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room info displays the current pinned event count from the existing Matrix SubscribeToPinnedEvents timeline subscription; RoomScreen subscribe/unsubscribe only updates local pinned-event state and the info strip does not send PinEvent, message, room-state mutation, or membership requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "typing notice subscription read path",
        base_module: "RoomScreen + TypingNotice + room info strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room info and TypingNotice consume incoming users from the existing Matrix SubscribeToTypingNotices timeline subscription; RoomScreen subscribe/unsubscribe only updates local typing-user state and the info strip does not send a typing notice, message, room-state mutation, or membership requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room members read path",
        base_module: "RoomScreen + room info strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room info displays member count from existing SyncRoomMemberList plus loaded RoomMembersListFetched/GetRoomMembers local-only state; RoomScreen reads the SDK local cache via GetRoomMembers(local_only=true) and does not send JoinRoom, LeaveRoom, InviteUser, Knock, message, room-state, or membership mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room member sync read path",
        base_module: "RoomScreen + room info strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room show uses the existing Matrix SyncRoomMemberList read/sync path to refresh local member profiles before GetRoomMembers local_only is read for the info strip; the RoomScreen member sync/read evidence does not send JoinRoom, LeaveRoom, InviteUser, Knock, message, room-state, or membership mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room power levels read path",
        base_module: "RoomScreen + room settings strip + permission-aware widgets",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings explains current user send, reaction, and @room permission display state from the existing Matrix GetRoomPowerLevels read path; RoomScreen only stores local UserPowerLevels for UI affordances and does not send power-level, room-state, message, or membership mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "own read receipt subscription read path",
        base_module: "RoomScreen + room info strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room info explains the own read marker from the existing Matrix SubscribeToOwnUserReadReceiptsChanged subscription; RoomScreen subscribe/unsubscribe only updates local own-read-marker state and the info strip does not send ReadReceipt, message, room-state mutation, or membership requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "timeline pagination read path",
        base_module: "RoomScreen + LoadingPane",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Backwards pagination uses the existing Matrix PaginateTimeline read path to fetch older loaded timeline events; LoadingPane explains that no message send, edit, or room-state mutation is started",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "composer",
        base_module: "RoomInputBar",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "text send, reply, edit pane, local @mention helper, typing notice, send-on-enter, permission states, location send behind a confirmation guard, local attachment, local emoji/sticker surface (Smile, Thumbs, Heart, Sticker), local voice affordances, and local Close evidence for composer preview panels",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "emoji/sticker send local boundary evidence",
        base_module: "RoomInputBar telegram_emoji_sticker_panel",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Smile, Thumbs, Heart, Sticker, repeated selection, and Close only stage local emoji/sticker preview status; they do not insert composer text, attach sticker media, create sticker payloads, submit SendMessage, submit SendAttachment, upload media, start SDK send-queue work, send typing notices, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "emoji/sticker lifecycle metadata",
        base_module: "RoomInputBar telegram_emoji_sticker_panel emoji_lifecycle_metadata",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Opening, repeated Smile/Thumbs/Heart/Sticker staging, Close, and reopen update only local panel visibility, last staged choice, staged choice count, and close/reopen state; the lifecycle metadata sends no composer insertion, sticker payload, SendMessage, SendAttachment, upload, SDK send-queue work, typing notice, remote picker/search, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "location send confirmation",
        base_module: "RoomInputBar + LocationPreview + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Location preview can fetch current coordinates, but opening the local confirmation guard, Cancel, and guard display keep the location message unsent; only the confirmed accept handler emits LocationSendConfirmed and requests the existing Matrix location SendMessage path, with no location SendMessage before confirmation, retry, cancel-location, extra message, room-state, membership, account/profile, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "live location continuous updates boundary",
        base_module: "LocationPreview + location worker",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "LocationPreview exposes explicit local Start and Stop device-update controls while keeping the existing one-shot UpdateOnce path and guarded Matrix location send. Start submits only LocationRequest::StartUpdates; Stop and Cancel submit only LocationRequest::StopUpdates when continuous updates are active. The controls do not create a live-location Matrix event, submit SendMessage, or send room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message context menu",
        base_module: "NewMessageContextMenu",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "react, reply, edit through EditingPane confirmation, pin/unpin behind a confirmation guard, copy, copy link, jump related, view source, delete/redact behind a confirmation guard, and report preview with Spam/Abuse confirmation before Matrix report_content",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "event source local surface",
        base_module: "EventSourceModal + NewMessageContextMenu",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "View Source opens a local event JSON modal from already loaded timeline event data; Copy Room ID, Copy Event ID, Copy Source, open, and Close only write local clipboard text or close local UI, sending no Matrix event source request, event fetch, message send, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "event source loaded metadata summary",
        base_module: "EventSourceModal loaded metadata label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "View Source summarizes already loaded room id, event id, latest JSON source availability, and local source byte/line counts from data passed into the modal. This sends no Matrix event source request, event fetch, event context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message copy local surface",
        base_module: "RoomScreen + NewMessageContextMenu clipboard actions",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Copy Text, Copy Text as HTML, and Copy Link use loaded timeline item data, loaded formatted bodies, or locally constructed matrix.to URIs and only write local clipboard text, sending no Matrix event fetch, message send, edit, redact, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message copy loaded metadata summary",
        base_module: "RoomScreen clipboard popup metadata",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Copy Text, Copy Text as HTML, and Copy Link popups summarize already loaded clipboard payload metadata: payload kind, event-id availability, character count, and byte count. The summary is derived from the same loaded timeline body, loaded formatted body, or locally constructed matrix.to URI that is copied to the local clipboard and sends no Matrix event fetch, event source request, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message pin confirmation",
        base_module: "NewMessageContextMenu + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Pin and Unpin open a confirmation modal before the existing Matrix PinEvent path is requested, and Cancel keeps the request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message delete confirmation",
        base_module: "NewMessageContextMenu + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Delete opens a confirmation modal before the existing Matrix redaction path is requested, and cancel keeps the request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report/send",
        base_module: "NewMessageContextMenu + MatrixRequest::ReportContent",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "message context menu exposes a Report surface where opening Report and Cancel remain local, while Spam, Abuse, and Custom reason open a confirmation guard before MatrixRequest::ReportContent calls Room::report_content. Custom reason draft metadata tracks raw/trimmed characters, bytes, cap state, empty/ready state, and target event-id state locally; empty custom reason stays unsent locally. The status strip shows submitted/sent/failed and failed Retry opens a second confirmation before reusing ReportContent. Cancel queue, moderation policy lookup, server-side moderation workflow, and abuse-management UI remain TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report option staging local evidence",
        base_module: "NewMessageContextMenu report preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Report opening and Cancel only stage local Telegram report preview evidence, while Spam, Abuse, and Custom reason require a confirmation modal before MatrixRequest::ReportContent is submitted; empty custom reason stays local and does not submit",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report send local boundary evidence",
        base_module: "NewMessageContextMenu report preview buttons + Room::report_content",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Report opening, repeated preview, empty custom reason, and Cancel remain local moderation preview evidence while message_report_send remains a base gap. Spam/Abuse/Custom confirmation sends only MatrixRequest::ReportContent through Room::report_content with the selected reason. It sends no moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report content live send wiring",
        base_module: "RoomScreen MessageAction::Report/RetryReport + MatrixRequest::ReportContent + TimelineUpdate::MessageReportResult",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Spam, Abuse, and Custom report actions already send the live MatrixRequest::ReportContent path only after confirmation. RoomScreen records submitted status before submit, SlidingSync calls Room::report_content, and TimelineUpdate::MessageReportResult returns sent or failed result metadata to the report status strip. Failed Retry reuses only cached event id and reason and opens PositiveConfirmationModal before resubmitting ReportContent. This partial-live path does not perform moderation queue persistence/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation outside the Matrix report_content request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report moderation workflow boundary",
        base_module: "NewMessageContextMenu report_moderation_boundary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The Report preview now shows moderation workflow boundary metadata from only the loaded target row and local custom-reason readiness. Moderation queue controls, server policy lookup, redact/delete, ban, kick, ignore/block, evidence queue, reviewer assignment, appeal flow, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth, and live mutation remain local blocked controls; the only real send path remains confirmed MatrixRequest::ReportContent for the selected event and reason",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report loaded target metadata preview",
        base_module: "NewMessageContextMenu report preview loaded metadata",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Report preview shows selected loaded row index, event-id availability, loaded body preview, character count, byte count, related-event availability, thread-root availability, local echo send-handle availability, and highlight state before Spam/Abuse/Custom confirmation. The metadata is derived only from the loaded timeline row and sends no Matrix report_content before confirmation, moderation policy lookup, relations fetch, event-context fetch, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report custom reason draft metadata",
        base_module: "NewMessageContextMenu report preview custom reason input",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Custom reason draft metadata is derived only from the local report reason text input and selected loaded target row before confirmation: raw character count, raw byte count, whitespace-compacted character count, whitespace-compacted byte count, 240-character cap state, empty-versus-ready state, target row index, and target event-id availability. Updating the draft, pressing empty Send Custom, viewing metadata, and canceling send no Matrix report_content before confirmation, moderation policy lookup, relations fetch, event-context fetch, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report cancel local evidence",
        base_module: "NewMessageContextMenu report preview Cancel/Escape",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Report Cancel and Escape only hide the local report preview, restore the Report button, reset focus/menu state, and show local popup evidence. They do not submit MatrixRequest::ReportContent, reuse draft reasons, retry or cancel a moderation queue, fetch moderation policy, relations, or event context, redact/delete, ban, kick, ignore/block, mutate room-state, mutate membership, send or edit messages, touch account/profile, call gateway/runtime/auth, or perform live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report status lifecycle",
        base_module: "RoomScreen + MatrixRequest::ReportContent + TimelineUpdate::MessageReportResult",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After Spam/Abuse/Custom confirmation submits MatrixRequest::ReportContent, RoomScreen shows an event-scoped report status strip for submitted, succeeded, or failed worker results from TimelineUpdate::MessageReportResult. The strip records only selected event id, compact reason metadata, and result/error text; failed status exposes Retry but it opens confirmation first. Close hides locally and sends no retry without confirmation, cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report status clipboard action",
        base_module: "RoomScreen copy_telegram_message_report_status_summary + local clipboard",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The report status strip Copy button writes only the current local ReportContent status cache to the local clipboard: status badge, cached event id, compact reason, result/error text, summary, lifecycle metadata, and preflight metadata. Missing status stays local-unavailable and writes no clipboard payload. It sends no extra MatrixRequest::ReportContent, retry without PositiveConfirmationModal, cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request while message_report_send remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report retry confirmation",
        base_module: "RoomScreen report status strip + MatrixRequest::ReportContent",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Failed report status can retry only from cached event id and compact reason metadata, and Retry opens PositiveConfirmationModal before another MatrixRequest::ReportContent is submitted. Confirmation cancel and unavailable cached reason stay local. This sends no retry queue automation, cancel queue, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report workflow actions row",
        base_module: "RoomScreen telegram_message_report_status_strip report_workflow_actions",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The report status strip exposes Queue, Policy, Assign, Appeal, Enforce, Packet, Contract, and Taxonomy as visible local blocked buttons after the existing MatrixRequest::ReportContent status path. Queue renders a local moderation packet snapshot from cached ReportContent state: status badge, event id, reason, cached error, summary, workflow metadata, preflight detail, and retry-cache state. Packet copies moderation reviewer acceptance criteria to the local clipboard. Contract maps that packet to typed moderation workflow/result contracts locally. Taxonomy records blocked queue/policy/reviewer/evidence/appeal/enforcement result slots locally before any workflow promotion. Policy, Assign, Appeal, and Enforce only update local report status metadata and popup copy. It does not cancel or reorder a moderation queue, fetch a server policy, assign a reviewer, open an appeal workflow, redact/delete content, ban, kick, ignore/block, mutate room-state, mutate membership, send or edit messages, touch account/profile, call gateway/runtime/auth, or perform live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report moderation reviewer packet",
        base_module: "RoomScreen copy_telegram_message_report_moderation_reviewer_packet + message_report_moderation_reviewer_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The report Packet action copies only a local moderation reviewer acceptance matrix from the cached ReportContent status strip: status badge, cached event id, compact reason, cached error, summary, workflow metadata, preflight metadata, retry-cache state, and loaded-source availability. It names acceptance criteria for moderation queue persistence, policy lookup, reviewer assignment, evidence/source retention, reporter and target audit, appeal workflow, enforcement, retry/cancel handling, and result/error mapping. It sends no extra MatrixRequest::ReportContent, retry without PositiveConfirmationModal, queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation while message_report_send remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report workflow result contract packet",
        base_module: "RoomScreen copy_telegram_message_report_workflow_result_contract_packet + message_report_workflow_result_contract_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The report Contract action copies only a local typed moderation workflow/result contract packet from the cached ReportContent status strip and moderation reviewer packet boundary: status badge, cached event id, compact reason, cached error, summary, workflow metadata, preflight metadata, retry-cache state, and loaded-source availability. It names typed queue, policy, reviewer assignment, evidence/source, reporter/target audit, appeal, enforcement, result/error, retry/cancel, and source-hash slots before any backend moderation workflow can be promoted. It sends no extra MatrixRequest::ReportContent, retry without PositiveConfirmationModal, queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation while message_report_send remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report workflow result taxonomy packet",
        base_module: "RoomScreen copy_telegram_message_report_workflow_result_taxonomy_packet + message_report_workflow_result_taxonomy_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The report Taxonomy action copies only a local blocked moderation workflow result taxonomy packet from the cached ReportContent status strip, moderation reviewer packet boundary, and loaded-source availability. It names the existing confirmed MatrixRequest::ReportContent send/result/retry and loaded-or-source-fetch EventSourceModal paths as live references, then records queue, policy, reviewer, evidence, appeal, enforcement, retry, cancel, source-hash, and audit result slots as not_wired before backend moderation workflow promotion. It sends no extra MatrixRequest::ReportContent, retry without PositiveConfirmationModal, queue persist/cancel/reorder, policy lookup, reviewer assignment, evidence upload, event-context fetch, appeal/enforcement workflow, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation while message_report_send remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report preflight detail controls row",
        base_module: "RoomScreen telegram_message_report_status_strip report_preflight_detail_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The report status strip exposes Request, Result, Error, Retry, and Source as visible ReportContent preflight detail controls. Request, Result, Error, and Retry update only local report preflight detail metadata and popup copy from the existing status cache: status badge, cached event id, compact reason, cached error text, retry availability, and status metadata source. Source is a real loaded-or-source-fetch modal handoff that may open already loaded EventSourceModal JSON for the cached reported event id, or request MatrixRequest::FetchEventSource for current-room event JSON when loaded latest_json is unavailable. It sends no extra MatrixRequest::ReportContent, no retry without PositiveConfirmationModal, no event-context fetch, no cancel queue, duplicate report automation, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, message send/edit, account/profile, gateway/runtime/auth, or write-side live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message report loaded source modal action",
        base_module: "RoomScreen open_telegram_message_report_loaded_source + EventSourceModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The report Source action is a real loaded-or-source-fetch modal handoff. It opens the existing EventSourceModal from the cached reported event id and the already loaded RoomScreen timeline row when EventTimelineItem.latest_json is available. If loaded JSON is unavailable but the cached reported event id and current timeline are known, it submits MatrixRequest::FetchEventSource; the existing worker calls Room::load_or_fetch_event and returns TimelineUpdate::EventSourceFetched for the same EventSourceModal path. Missing cache, missing timeline, invalid state, fetch failure, or missing source leaves Source as local metadata. It sends no event-context fetch, extra ReportContent, retry automation, moderation workflow request, moderation policy lookup, redact/delete, ban, kick, ignore/block, room-state mutation, membership mutation, account/profile, gateway/runtime/auth, or write-side live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history",
        base_module: "EditedIndicator + Matrix m.replace relations read",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "edited message indicator starts MatrixRequest::FetchEditHistory for paginated m.replace relations and returns count/pages/exhausted metadata, latest replacement preview, cached latest replacement raw JSON, and an already loaded original plaintext preview; RoomScreen Full opens a local synthetic EventSourceModal snapshot from that cached state, Diff opens a loaded side-by-side preview/full-body diff EventSourceModal snapshot plus clipboard handoff by using cached latest replacement raw JSON and loaded original latest_json when both source bodies are available, falling back to the loaded side-by-side preview diff snapshot when source JSON is missing, and Source can open the existing EventSourceModal for the cached latest replacement raw JSON returned by FetchEditHistory, request MatrixRequest::FetchEventSource through Room::load_or_fetch_event when that latest replacement raw JSON is missing, or fall back to the already loaded original event source when latest_json is still in the visible timeline cache; message_edit_history remains an explicit base gap until remote/server-backed full history modal UI, server-authored full-body diff payloads, timeline reload UI, event-context fetch, and server-backed source reconciliation are implemented",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history click summary read",
        base_module: "EditedIndicator + MatrixRequest::FetchEditHistory",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "clicking the edited indicator sends only MatrixRequest::FetchEditHistory for m.replace relations, then shows a compact count/latest replacement preview and compares it with the loaded original plaintext preview as a local diff hint; it sends no timeline reload, event-context fetch, message send, edit, redact, room-state request, membership mutation, account/profile mutation, gateway/runtime/auth, or live mutation while full edit history UI remains TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history compact summary live read wiring",
        base_module: "EditedIndicator + MatrixRequest::FetchEditHistory + TimelineUpdate::EditHistoryFetched",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Clicking the edited indicator already submits the live MatrixRequest::FetchEditHistory path for m.replace relations. SlidingSync calls Room::relations with RelationType::Replacement, follows next_batch until exhaustion, returns replacement count, relation pages fetched/exhausted state, latest replacement event id/timestamp, latest preview text, and cached latest replacement raw JSON through TimelineUpdate::EditHistoryFetched, and RoomScreen renders result or error metadata into the edit-history strip. Failed Retry reuses only the cached event id and TimelineKind and opens PositiveConfirmationModal before another paginated FetchEditHistory read. Source can additionally submit MatrixRequest::FetchEventSource for the latest replacement when cached raw JSON is missing. This partial-live path does not request a remote full history modal, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or write-side live mutation outside the paginated m.replace summary/source reads",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history loaded target metadata preview",
        base_module: "EditedIndicator loaded target metadata",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "EditedIndicator caches loaded edit-history target metadata from the already loaded timeline row: loaded event-id availability, loaded original plaintext preview, character count, byte count, and latest edit timestamp availability. Hover and click-start popup show this cached metadata before the existing MatrixRequest::FetchEditHistory read and send no event-context fetch, timeline pagination/reload, event source open, remote full-history modal request, full diff rendering, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history detail surface",
        base_module: "RoomScreen telegram_message_edit_history_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen renders a local Telegram edit-history detail strip after MatrixRequest::FetchEditHistory completes paginated m.replace relation reads. The strip shows target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached latest replacement raw JSON availability, and a local preview-diff hint, then Close hides the strip locally. Full can open a local synthetic EventSourceModal snapshot from this cached state. It reuses the already loaded timeline row plus the complete paginated m.replace relation summary; Source may submit MatrixRequest::FetchEventSource for the latest replacement raw JSON only when the cached relation source is missing. It sends no remote full-history modal request, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history full modal boundary",
        base_module: "RoomScreen edit_history_full_modal_boundary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen shows edit-history full-modal boundary metadata from the current paginated m.replace read state: loading, loaded, failed, or retry confirmation, replacement count when available, relation pages fetched/exhausted state, cached latest replacement raw JSON availability, and retry cache readiness. Full opens the existing local EventSourceModal with a synthetic full snapshot JSON from the loaded target, complete paginated replacement summary, original preview, latest replacement preview, cached latest replacement raw JSON availability, cached error, and retry cache state. Remote/server-backed Full history modal UI, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and write-side live mutation remain local blocked controls while the real edit-history read is MatrixRequest::FetchEditHistory paginating Room::relations to next_batch exhaustion plus EventSourceModal handoff for cached latest replacement raw JSON, MatrixRequest::FetchEventSource fallback for missing latest replacement source JSON, or loaded original latest_json",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history local full snapshot modal",
        base_module: "RoomScreen open_telegram_message_edit_history_local_full_snapshot_modal + EventSourceModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen edit-history Full opens the existing local EventSourceModal with a synthetic JSON snapshot built only from cached MatrixRequest::FetchEditHistory state and loaded RoomScreen timeline state: target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached latest replacement raw JSON availability, cached error, retry cache readiness, and read-only side-effect metadata. It sends no extra MatrixRequest::FetchEditHistory, no remote full-history modal request, no side-by-side full diff rendering, no event-context fetch, no timeline pagination/reload, no message send/edit/redact, no room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history full controls row",
        base_module: "RoomScreen edit_history_full_controls buttons",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen exposes Full, Diff, Context, Source, Packet, Contract, and Taxonomy on telegram_message_edit_history_strip. Full opens the existing local EventSourceModal with a synthetic full snapshot JSON from the current complete paginated m.replace summary surface, while Context updates only the local boundary label and popup status. Diff opens a loaded side-by-side preview/full-body diff EventSourceModal snapshot and copies the compact original/latest preview summary when loaded preview data exists; full-body rows come only from cached latest replacement raw JSON plus loaded original latest_json when both bodies are available, and the fallback remains the loaded side-by-side preview diff modal when source JSON is missing. Source is a real loaded edit-source modal handoff that opens the existing EventSourceModal for cached latest replacement raw JSON returned by MatrixRequest::FetchEditHistory when available, requests MatrixRequest::FetchEventSource through Room::load_or_fetch_event when cached latest replacement raw JSON is missing, and otherwise falls back to the already loaded original edited event row when loaded EventTimelineItem.latest_json is available. Packet copies a loaded/full diff remote modal acceptance contract to the local clipboard from the same cached state, including relation pages fetched/exhausted metadata. Contract maps that packet to typed full-history modal/result, diff, source, context, and retry contracts locally. Taxonomy copies blocked remote full-history, source reconciliation, server-backed diff, event-context, stale-target, retry/cancel, and source-hash result slots locally. The controls send no remote full-history modal request, server-backed side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation request; MatrixRequest::FetchEditHistory is the complete paginated edit-history read path and MatrixRequest::FetchEventSource is a source-only read fallback",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history loaded edit source modal",
        base_module: "RoomScreen open_telegram_message_edit_history_loaded_source + EventSourceModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen edit-history Source is a real loaded edit-source modal handoff that reuses the existing EventSourceModal for cached latest replacement raw JSON when MatrixRequest::FetchEditHistory returned a relation source; if no replacement raw JSON is cached and a latest replacement event id is known, it submits MatrixRequest::FetchEventSource, whose worker calls Room::load_or_fetch_event and opens EventSourceModal with the fetched raw JSON. If no replacement source is available, it falls back to the already loaded original edited event row when loaded EventTimelineItem.latest_json is available. It derives the target from the paginated edit-history latest replacement event id, current TimelineKind room id, and raw JSON returned by Room::relations or Room::load_or_fetch_event, or the original target event id plus loaded latest_json from the visible timeline cache. Missing event id, missing cached/fetched raw JSON, missing row, or missing latest_json leaves Source as local metadata. It sends no event-context fetch, timeline pagination/reload, remote full-history modal request, side-by-side full diff rendering, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history loaded diff detail state",
        base_module: "RoomScreen edit_history_loaded_diff_detail",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen derives loaded edit-history diff detail only from the paginated MatrixRequest::FetchEditHistory m.replace summary plus the already loaded original timeline row. The detail label reports selected Full/Diff/Context/Source/Packet/Contract/Taxonomy control, target event, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp availability, original/latest preview character counts, latest replacement raw JSON availability, local delta state, and retry cache readiness. Clicking Full opens a local synthetic EventSourceModal snapshot and updates this local detail state, boundary label, and popup copy; clicking Context updates only this local detail state, the boundary label, and popup copy; clicking Diff opens a loaded side-by-side preview/full-body diff EventSourceModal snapshot, uses cached latest replacement raw JSON plus loaded original latest_json for full-body rows when available, falls back to the loaded side-by-side preview diff snapshot when source JSON is missing, and copies only the loaded compact original/latest preview diff summary to the local clipboard when loaded preview data exists; clicking Source also opens the cached latest replacement raw JSON in EventSourceModal when available, otherwise requests MatrixRequest::FetchEventSource through Room::load_or_fetch_event before falling back to the already loaded original EventSourceModal when latest_json is available; clicking Packet copies only the loaded/full diff remote modal acceptance contract locally; clicking Contract copies only typed full-history modal/result contracts locally; clicking Taxonomy copies only blocked remote full-history/source reconciliation result taxonomy locally. It sends no remote full-history modal request, server-backed side-by-side full diff rendering, event-context fetch, timeline pagination/reload, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or write-side live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history loaded side-by-side diff modal action",
        base_module: "RoomScreen copy_telegram_message_edit_history_loaded_diff + EventSourceModal + local clipboard",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen Diff is a real loaded side-by-side preview/full-body diff modal plus compact diff clipboard handoff, with the fallback still a real loaded side-by-side preview diff modal. It opens EventSourceModal with a read-only JSON snapshot and copies a compact loaded original/latest preview diff summary to the local clipboard only from current edit-history strip state when loaded preview data exists: target event id, paginated m.replace replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview rows, latest replacement preview rows, full-body rows from cached latest replacement raw JSON plus loaded original latest_json when both bodies exist, changed-row flags, body-source labels, and local delta hint. Missing target, missing previews, or missing current timeline state leaves the action local-unavailable. It sends no extra MatrixRequest::FetchEditHistory, no remote full-history modal request, no server-backed full-body side-by-side diff rendering, no event-context fetch, no timeline pagination/reload, no replacement event source fetch, no message send/edit/redact, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history loaded/full diff packet",
        base_module: "RoomScreen copy_telegram_message_edit_history_full_diff_packet + edit_history_full_diff_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen Packet copies only a loaded/full diff remote modal acceptance contract to the local clipboard from cached paginated m.replace state: target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached error, retry-cache readiness, loaded diff detail, preflight detail, and full-modal boundary metadata. It names acceptance criteria for remote full-history modal request/result/error, side-by-side full diff rendering, event context, cached latest replacement source, MatrixRequest::FetchEventSource fallback source, loaded original source fallback, retry/cancel state, and source hashing. It sends no extra MatrixRequest::FetchEditHistory, retry without PositiveConfirmationModal, remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, source fetch beyond the Source control's source-only MatrixRequest::FetchEventSource fallback, message send/edit/redact, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or write-side live mutation while message_edit_history remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history full-history result contract packet",
        base_module: "RoomScreen copy_telegram_message_edit_history_full_history_result_contract_packet + edit_history_full_history_result_contract_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen Contract copies only a typed full-history modal/result contract packet to the local clipboard from cached paginated m.replace state and the loaded/full diff Packet boundary: target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, loaded original preview, latest replacement preview, cached error, retry-cache readiness, loaded diff detail, preflight detail, and full-modal boundary metadata. It maps typed request/result/error/retry/source slots for full-history modal rendering, side-by-side diff rendering, event context, cached latest replacement source, MatrixRequest::FetchEventSource fallback source, loaded original source fallback, source-hash, retry/cancel, stale target handling, and promotion blockers before backend edit-history work can be promoted. It sends no extra MatrixRequest::FetchEditHistory, retry without PositiveConfirmationModal, remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, source fetch beyond the Source control's source-only MatrixRequest::FetchEventSource fallback, message send/edit/redact, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or write-side live mutation while message_edit_history remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history remote result taxonomy packet",
        base_module: "RoomScreen copy_telegram_message_edit_history_remote_result_taxonomy_packet + edit_history_remote_result_taxonomy_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen Taxonomy copies only a remote full-history/source reconciliation result taxonomy packet to the local clipboard from cached paginated m.replace state and loaded source/diff metadata. It names existing live references as paginated MatrixRequest::FetchEditHistory through Room::relations next_batch exhaustion, confirmed failed-state Retry, local synthetic Full EventSourceModal snapshot, loaded side-by-side preview/full-body diff EventSourceModal snapshot, compact diff clipboard handoff, cached latest replacement raw JSON EventSourceModal handoff, source-only MatrixRequest::FetchEventSource and Room::load_or_fetch_event fallback, and loaded original EventSourceModal fallback. It keeps remote_full_history_request_id, full_history_cursor_id, server_backed_full_diff_operation_id, replacement_source_reconciliation_operation_id, event_context_operation_id, stale target result, retry/cancel result, and source-hash policy not-assigned/not-wired. It sends no extra MatrixRequest::FetchEditHistory, retry without PositiveConfirmationModal, remote full-history modal request, server-backed side-by-side full diff rendering, event-context fetch, timeline pagination/reload, source fetch beyond the Source control's source-only MatrixRequest::FetchEventSource fallback, message send/edit/redact, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or write-side live mutation while message_edit_history remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history preflight detail controls row",
        base_module: "RoomScreen edit_history_preflight_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen exposes Request, Result, Error, Retry, and Source as visible local edit-history preflight detail controls on telegram_message_edit_history_strip. Each button only updates cached paginated edit-history preflight metadata and popup copy from the current MatrixRequest::FetchEditHistory local state: target event id, replacement count, relation pages fetched/exhausted state, latest replacement event/timestamp, original/latest preview counts, cached error text, retry cache readiness, and local source/boundary metadata. It sends no extra MatrixRequest::FetchEditHistory, no retry without PositiveConfirmationModal, no remote full-history modal request, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, event source open, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history retry confirmation",
        base_module: "RoomScreen telegram_message_edit_history_strip + MatrixRequest::FetchEditHistory",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After TimelineUpdate::EditHistoryFetched returns a failed compact m.replace read, RoomScreen exposes Retry only when it has cached the event id and TimelineKind from the previous MatrixRequest::FetchEditHistory attempt. Retry opens PositiveConfirmationModal before another MatrixRequest::FetchEditHistory request is submitted; unavailable cached event id, unavailable TimelineKind, and confirmation cancel stay local. It sends no remote full-history modal request, full diff rendering, event-context fetch, timeline pagination/reload, event source open, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit history local boundary evidence",
        base_module: "EditedIndicator hover + compact m.replace summary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "EditedIndicator hover reads only the already loaded latest edit timestamp, and click uses MatrixRequest::FetchEditHistory for complete paginated m.replace relations while RoomScreen adds cached latest replacement raw JSON plus the already loaded original plaintext preview as local diff/source evidence. The Full action opens a local synthetic EventSourceModal snapshot, and Source can open the cached latest replacement raw JSON locally through EventSourceModal or fall back to the already loaded original event source when latest_json is available, but message_edit_history remains a base gap. Remote/server-backed full history modal UI, side-by-side full diff rendering, event-context fetch, timeline pagination/reload, replacement source reconciliation, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain unwired",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit confirmation",
        base_module: "EditingPane + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Save Edit opens a confirmation modal before the existing Matrix EditMessage path is requested, and Cancel keeps the edit request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit unsupported feature local evidence",
        base_module: "EditingPane",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "EditingPane visibly states that HTML/plain prefixes, attachment add/remove, mention extraction, poll answer edits, and the Save spinner remain local/unwired; it renders an Edit/Poll detail packet with content_kind, edited_text_len, attachment_edit_slot not_built, mention_payload_scope preserve_existing_mentions_only or none, poll_answer_edit_slot not_built, save_spinner_operation_id not_assigned, result_mapping not_wired, and stale_result_policy. It also renders attachment preflight, mention payload preflight, mention payload typed contract, save-result mapping, and retry/error drilldown packets. Save Edit still uses the existing Matrix EditMessage confirmation path and sends no attachment upload/remove, Matrix mention payload, poll answer edit, timeline reload, message send, room-state, or membership request for those unsupported edit features",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit attachment preflight packet",
        base_module: "EditingPane attachment preflight label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "EditingPane attachment preflight packet records content_kind, edited_text_len, original_attachment_scope for image/audio/file/video caption edits or non-media/poll edits, selected_attachment_slot unavailable, add_attachment_slot not_built, remove_attachment_slot not_built, replace_attachment_slot not_built, upload_request_slot not_built, media_delete_slot not_built, caption_edit_handoff existing_confirmed_MatrixRequest_EditMessage_body_only, mime_size_probe not_started, retry_policy no_duplicate_upload_without_operation_id, and cancel_policy leaves_original_media_and_local_selection_untouched. It sends no SendAttachment, media delete, upload, timeline reload, room-state, membership request, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit mention payload preflight packet",
        base_module: "EditingPane mention payload preflight label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "EditingPane mention payload preflight packet records content_kind, edited_text_len, edited_at_token_count, literal_user_id_token_count, room_token_scope, completed_pill_reconcile_slot not_connected_to_editing_pane, directory_result_scope unavailable_in_editing_pane, fresh_mentions_payload_slot not_built, existing_mentions_handoff preserve_existing_mentions_only or none, reply_sendtime_state not_reused, retry_source_hash_slot missing, stale_token_policy backend_required_before_live_mentions, and cancel_policy confirmation_cancel_no_request. It sends no fresh Matrix Mentions payload, profile lookup, directory search, SendMessage, SendAttachment, room-state, membership request, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit mention payload typed contract packet",
        base_module: "EditingPane mention payload typed contract label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "EditingPane mention payload typed contract packet records content_kind, edited_text_len, mention_contract_version local_v0, token_scan_source edited_text_only, edited_at_token_count, literal_user_id_contract_count, room_token_contract_scope, directory_snapshot_id_slot unavailable, completed_pill_snapshot_slot unavailable, existing_mentions_handoff preserve_existing_mentions_only or none, source_hash_slot not_assigned, fresh_mentions_payload_result_slot not_built, retry_idempotency_key_slot missing, stale_result_guard body_source_hash_required_before_live_mentions, result_mapping accepted permission_denied stale_body malformed_token directory_unavailable not_wired, and privacy_redaction token_counts_only. It sends no fresh Matrix Mentions payload, directory snapshot reuse, profile lookup, SendMessage, SendAttachment, room-state, membership request, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit save-result mapping packet",
        base_module: "EditingPane + MatrixRequest::EditMessage result handler",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "EditingPane save-result mapping packet records lifecycle_state idle_preflight/confirmation_opened/saved_hide_pane/failed_popup/stale_event_id_ignored, operation_id_slot not_assigned, request_slot existing_confirmed_MatrixRequest_EditMessage, spinner_slot not_rendered, result_mapping saved_hide_pane failed_popup canceled_no_request stale_event_id_ignored ignored_late_result_without_matching_operation_id, stale_result_guard timeline_event_item_id_match_only, repeated_save_policy not_held_until_pending_operation_id, and retry_slot not_built. It only updates local labels around the existing confirmation-gated Matrix EditMessage path and sends no attachment upload/remove, Matrix mention payload, poll answer edit, timeline reload, message send, room-state, membership request, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message edit retry/error drilldown packet",
        base_module: "EditingPane retry/error drilldown label + MatrixRequest::EditMessage result handler",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "EditingPane retry/error drilldown packet records failure_source existing_MatrixRequest_EditMessage_result_only, error_redaction popup_text_not_persisted_or_reused, retry_request_slot not_built, retry_confirmation_slot not_built, late_result_guard timeline_event_item_id_match_only_without_operation_id, pending_operation_id missing_backend_contract, spinner_state not_rendered, cancel_state confirmation_cancel_no_request, repeated_save_policy not_held_until_pending_operation_id, stale_result_policy ignore_late_result_without_matching_operation_id, and retry_state idle/confirmation/failed/stale/saved. It only updates local labels around the existing confirmation-gated Matrix EditMessage path and sends no attachment upload/remove, Matrix mention payload, poll answer edit, timeline reload, extra message send beyond the existing confirmed edit request, room-state, membership request, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP identity indicator",
        base_module: "TspSignIndicator local preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "signed-message TSP indicator exposes a local-only identity preview for unknown, verified, and warning states without TSP profile lookup, DID resolution, or Matrix request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP wallet pending cancel local evidence",
        base_module: "WalletEntry + CreateWalletModal + CreateDidModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "WalletEntry Delete Wallet is blocked local evidence with loaded wallet name, URL/path availability, opened/not-found status, and default-wallet metadata, and it sends no TspRequest::DeleteWallet, filesystem delete, wallet database write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation; WalletEntry also renders a local delete preflight/result taxonomy packet; Create Wallet and Create DID modals show local pending-cancel operation packets with operation_id missing_backend_contract, local_operation_key metadata, cancel_state disabled_no_request, stale_result_policy backend_operation_id_required, and password/secret redaction while TSP async creation is pending because TspRequest cancellation is not wired and no cancel request is sent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP wallet delete preflight/result packet",
        base_module: "WalletEntry",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "WalletEntry Delete Wallet renders a local preflight/result taxonomy packet with wallet_identity, path_validation_slot backend_required_exists_regular_app_owned_single_scope, ownership_scope backend_required, open_wallet_closure_slot, default_fallback_slot, persistence_result_slot not_started, filesystem_result_taxonomy deleted/already_missing/permission_denied/busy/not_app_owned/partial_failure, retry_cancel_policy confirmation_gated_idempotent_retry_cancel_sends_no_request, and audit_redaction_policy. It sends no TspRequest::DeleteWallet, filesystem delete, wallet database write, TSP state mutation, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP pending cancel operation packet",
        base_module: "CreateWalletModal + CreateDidModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "CreateWalletModal and CreateDidModal pending states now render local operation packets for wallet creation and DID creation/publication. The packets expose operation_id missing_backend_contract, non-secret local_operation_key fields, cancel_state disabled_no_request, stale_result_policy backend_operation_id_required, password_redacted or secret_redacted true, server/default state, and alias availability. They send no TspRequest cancel, wallet rollback, DID rollback, filesystem delete/write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP wallet open retry",
        base_module: "WalletEntry + TspRequest::OpenWallet",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "WalletEntry exposes Open Wallet only for loaded NotFound wallet rows and reuses the existing TspRequest::OpenWallet worker path with the known wallet name/path metadata. Opened rows show local already-open metadata and do not submit a retry. The path starts no file picker, Import Existing Wallet flow, wallet creation, SetDefaultWallet, TspRequest::DeleteWallet, filesystem delete, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP wallet set default confirmation metadata",
        base_module: "WalletEntry + PositiveConfirmationModal + TspRequest::SetDefaultWallet",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "WalletEntry Set As Default shows loaded wallet metadata in PositiveConfirmationModal before confirmation: wallet name, URL/path availability, opened/not-found status, and default-wallet state. Confirmed Set Default uses only the existing TspRequest::SetDefaultWallet active/default wallet switch path; confirmation cancel sends no SetDefaultWallet, OpenWallet, RemoveWallet, TspRequest::DeleteWallet, filesystem delete, wallet database delete, Matrix request, gateway/runtime/auth path, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP wallet remove confirmation metadata",
        base_module: "WalletEntry + ConfirmationModal + TspRequest::RemoveWallet",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "WalletEntry Remove From List shows loaded wallet metadata before confirmation: wallet name, URL/path availability, opened/not-found status, and default-wallet state. Confirmed Remove uses only the existing TspRequest::RemoveWallet list/default-slot path; confirmation cancel sends no RemoveWallet, TspRequest::DeleteWallet, filesystem delete, wallet database delete, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP wallet import blocked local evidence",
        base_module: "TspSettingsScreen",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Import Existing Wallet is blocked local evidence in the real TSP settings screen with loaded wallet count, active-wallet availability, other-wallet count, and active identity metadata; it opens no file picker, wallet database, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation while wallet import remains TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP wallet import preflight packet",
        base_module: "TspSettingsScreen + PositiveConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Import Existing Wallet now opens a PositiveConfirmationModal local preflight packet with picker_result not_started, selected_path unavailable, password_state not_collected, vault_open not_started, persistence_result not_started, duplicate-policy metadata, loaded wallet count, active-wallet availability, other-wallet count, and active identity metadata. Acknowledge and Close only dismiss local UI; they send no file picker, password capture, wallet database open, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP wallet import result taxonomy packet",
        base_module: "TspSettingsScreen + PositiveConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Import Existing Wallet now renders a local result taxonomy packet with operation_id_slot not_assigned, picker_result canceled selected_path_unavailable inaccessible_path unsupported_url_scheme not_wired, auth_result password_not_collected invalid_password redacted_retry_required not_wired, vault_open_result opened invalid_password unsupported_vault corrupted_database already_imported duplicate_path permission_denied not_wired, metadata_result wallet_name_sanitized_path_default_role not_started, duplicate_result, persistence_result saved duplicate_blocked failed stale_operation not_started, retry_policy selected_path_reused_password_fresh_backend_required, cancel_policy local_dismiss_no_request, stale_result_policy backend_operation_id_required_before_import_live, and audit_redaction_policy no_password_token_private_vid_key_material_raw_path. It sends no file picker, password capture, wallet database open, TspRequest, filesystem read/write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP worker receipt/result packet",
        base_module: "TspSettingsScreen + TspWalletAction + TspIdentityAction",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "TspSettingsScreen renders a local TSP worker receipt/result packet for existing wallet and identity worker actions. It records operation, request_slot, operation_id_slot not_assigned, worker_receipt Cx_post_action, result_state success/error/canceled/stale taxonomy, target wallet/identity loaded state, ui_effect, retry_slot existing_guarded_paths_only, stale_result_policy local_screen_cache_match_only_backend_operation_id_required_for_cancel_or_retry, and audit_redaction_policy. It creates no new TspRequest and adds no cancel, delete, import, remove, filesystem delete, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation behavior beyond the already requested worker paths",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP association cancel local evidence",
        base_module: "TspVerifyUser",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "TspVerifyUser still sends AssociateDidWithUserId through the existing TSP worker path, but initiator-side cancel and Remove TSP Association are visible local evidence with loaded target user id, DID availability, local association state metadata, and a local association cancel/remove packet; they send no CancelAssociateDidRequest, VerificationCancel, TspRequest cancel, TSP state update, wallet database write, filesystem write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP association cancel/remove packet",
        base_module: "TspVerifyUser",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "TspVerifyUser initiator cancel and Remove TSP Association now render a local association cancel/remove packet with request_id missing_backend_contract, target/DID availability, cancel_state disabled_no_request, persistence_scope backend_required, receive_loop_scope backend_required, and stale_result_policy backend_request_id_required. It sends no CancelAssociateDidRequest, VerificationCancel, TspRequest cancel, TSP state update, wallet database write, filesystem write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP association result taxonomy packet",
        base_module: "TspVerifyUser",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "TspVerifyUser association cancel/remove packet now includes a local result taxonomy for local_only_cancel_not_sent, remote_cancel_not_sent, already_answered_local_state, failed_cancel_not_started, stale_request_blocked, and remove_not_started, plus persistence_result_slot not_started, receive_loop_result_slot not_started, responder_notification_slot not_sent, retry_policy blocked_until_backend_request_id, and audit_redaction target_did_presence_only. It sends no CancelAssociateDidRequest, VerificationCancel, TspRequest cancel, TSP state update, wallet database write, filesystem write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "TSP verification request loaded metadata",
        base_module: "TspVerificationModal + tsp_verification_request_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "TspVerificationModal now shows loaded request metadata from TspVerificationDetails, current Matrix identity, and wallet VID cache: initiating user/VID availability, responding user/VID availability, current-user match, and wallet responding VID availability. Rendering metadata starts no extra TspRequest, wallet database write, filesystem write, Matrix request, gateway/runtime/auth, runtime mutation, or live mutation; Accept and Ignore keep the existing RespondToDidAssociationRequest path unchanged",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix crypto verification request metadata",
        base_module: "VerificationModal + verification_request_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "VerificationModal now shows loaded Matrix verification metadata from VerificationRequest plus local modal stage: own user, other user, room or to-device scope, flow id availability, self-verification state, local or remote origin, passive/ready state, and supported-method count. Rendering metadata starts no Matrix verification accept, cancel, SAS confirm, device trust write, account/profile, gateway/runtime/auth, runtime mutation, or live mutation; Accept, Cancel, and SAS confirmation keep the existing response_sender and async verification handler paths unchanged",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "login auto cancel local evidence",
        base_module: "LoginScreen + LoginStatusModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "CLI auto-login shows a disabled Cancel evidence row because login cancellation is not wired; password login Cancel only closes the local status modal, while SSO Cancel keeps the existing local redirect-server shutdown path. No Matrix login cancel request, message, room-state, membership, or runtime mutation is sent from the disabled/local evidence paths",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room context menu",
        base_module: "RoomContextMenu",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "mark read/unread, favorite, and low priority open a confirmation guard before the existing Matrix room status paths; copy room link reuses the existing Matrix GenerateMatrixLink path and then writes to clipboard without room state or membership mutation; invite modal behind a confirmation guard, leave, local-only settings preview, plus confirmed All/Mentions/Mute notification mode writes while timed mute stays unwired",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room status confirmation",
        base_module: "RoomContextMenu + RoomScreen + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room list context menu and room action strip require confirmation before Matrix SetUnreadFlag, SetIsFavorite, or SetIsLowPriority paths are requested, and Cancel keeps the room status request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room invite confirmation",
        base_module: "InviteModal + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "InviteModal opens a confirmation modal before the existing Matrix InviteUser path is requested, and Cancel keeps the invite request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "timeline invite confirmation",
        base_module: "RoomScreen SmallStateEvent + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Knocked-member timeline Invite buttons open a confirmation modal before the existing Matrix InviteUser path is requested, and Cancel keeps the timeline invite request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "invite response confirmation",
        base_module: "InviteScreen + JoinLeaveRoomModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Accept Invite and Reject Invite always open the existing join/leave confirmation modal before Matrix JoinRoom or LeaveRoom is requested; Shift-click no longer bypasses confirmation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "space lobby join/leave confirmation",
        base_module: "SpaceLobbyScreen + JoinLeaveRoomModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Space lobby Join and Leave actions open the existing JoinLeaveRoomModal before Matrix JoinRoom or LeaveRoom is requested, preserving the same membership mutation path behind confirmation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "space lobby hierarchy read/sync path",
        base_module: "SpaceLobbyScreen + SpaceService",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Space Lobby hierarchy uses existing SpaceRequest GetDetailedChildren, GetTopLevelSpaceDetails, GetChildren, PaginateSpaceRoomList, and SubscribeToSpaceRoomList read/sync paths for child rooms, subspaces, top-level details, and pagination; these paths do not send LeaveSpace, JoinRoom, InviteUser, Knock, membership, message, or room-state mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "space lobby room-list lifecycle cleanup",
        base_module: "SpaceLobbyScreen + SpaceRequest::UnsubscribeFromSpaceRoomList",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Space Lobby labels SubscribeToSpaceRoomList and PaginateSpaceRoomList as read-sync paths, while UnsubscribeFromSpaceRoomList is classified as service lifecycle cleanup for the local space-room-list task, not a user-facing stop-sync control. Rendering this evidence sends no UnsubscribeFromSpaceRoomList, LeaveSpace, JoinRoom, InviteUser, Knock, message, room-state, membership, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "space lobby banned/knocked membership edge evidence",
        base_module: "SpaceLobbyScreen + SpaceService",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "SpaceLobby tree entries expose local evidence for Banned and Knocked child room/space states; banned entries disable Join/Knock locally, knocked entries render a re-knock/cancel-prior packet with tree_reknock_action_slot not_exposed, previous_knock_request_id missing, cancel_prior_request_slot not_built, cancel_prior_result_slot not_wired, and neither path sends Matrix JoinRoom, Knock, cancel-prior-knock, membership, message, or room-state requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "add room knock confirmation",
        base_module: "AddRoomScreen + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Knock-only room previews open a confirmation modal before the existing Matrix Knock path is requested, and Cancel keeps the knock request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "add room preview cancel",
        base_module: "AddRoomScreen fetched room preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Cancel closes the fetched room preview locally after the existing room preview lookup and sends no Matrix JoinRoom, Knock, or membership mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "add room banned/knocked membership edge evidence",
        base_module: "AddRoomScreen fetched room preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Banned previews disable Join/Knock and send no Matrix JoinRoom, Knock, cancel-prior-knock, membership, message, or room-state request; already-knocked previews render a re-knock/cancel-prior packet where re-knock can only re-open the existing Knock confirmation path, previous_knock_request_id is missing, cancel_prior_request_slot not_built, cancel_prior_result_slot not_wired, and no cancel request is sent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "add room restricted join-rule local evidence",
        base_module: "AddRoomScreen fetched room preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Restricted or unknown join-rule previews keep membership actions disabled locally until newer Matrix join-rule handling is wired; the preview sends no Matrix JoinRoom, Knock, cancel-prior-knock, message, room-state mutation, or membership request, while public and knock previews still wait for the existing confirmation guard before their Matrix paths are requested",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "transient chat states",
        base_module: "ReplyPreview + EditingPane + TypingNotice + JumpToBottomButton",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "reply preview, edit pane, typing notice, and jump-to-bottom affordance remain on the real room path",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message status",
        base_module: "AvatarRow",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "read receipt row and tooltip data already live in the room timeline path",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "reactions",
        base_module: "ReactionList",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "timeline reactions already render and are backed by ToggleReaction",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media and link previews",
        base_module: "MediaCache + LinkPreview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "image/sticker/location display, media fetch cache, link preview cache, and local-only file/audio/video metadata previews",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media fetch/cache read path",
        base_module: "MediaCache + RoomScreen image preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Image and sticker previews use the existing Matrix FetchMedia read/cache path through MediaCache::try_get_media_or_fetch only for missing MXC thumbnail/full-file cache entries; cache hits, Requested, Loaded, Failed, clear pending/failed, insert_into_cache, TimelineUpdate::MediaFetched, and SignalToUI only update local media cache/redraw state and send no manual Download, Play, Decrypt, message, room-state, membership, account, profile, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "poll message preview",
        base_module: "RoomScreen Message + matrix-sdk-ui PollState",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Poll timeline items now render as first-class message rows from already loaded matrix-sdk-ui PollState results inside populate_poll_message_content with question, answers, vote counts, total votes, open/closed status, edited note, and max selections. The row includes a Poll answer preview/result packet with answer_count, total_votes, max_selections, poll_status, edited_state, answer_edit_slot not_built, vote_response_slot not_sent, result_mapping read_only_loaded_pollstate, stale_poll_policy, and unsupported_server_capability_boundary local_disabled. This is read-only preview evidence and sends no poll response, poll answer edit, redact, message, room-state, membership, timeline reload, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "link preview local controls",
        base_module: "LinkPreview + LinkPreviewCache",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "URL preview data uses the existing Matrix GetUrlPreview cache/fetch path, while Show more, Show fewer, hover, title tap dispatch, dedup, matrix.to filtering, cache-hit reuse, pending, failed, and loaded display controls are local LinkPreview widget/cache state and send no extra GetUrlPreview beyond the first missing cache entry, no Matrix alias resolution, room preview fetch, event context fetch, external browser handoff, media download, message, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "link preview loaded metadata summary",
        base_module: "LinkPreview populate_view + LinkPreviewData",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "LoadedLinkPreview rows summarize already loaded GetUrlPreview metadata for title, site name, description, image presence, image MIME type, image dimensions, and image size in the local read-path label, and pass loaded og:image width/height into ImageInfo before the existing image cache renderer runs. This sends no extra GetUrlPreview beyond the first missing accepted URL, no Matrix alias resolution, room preview fetch, event context fetch, external browser handoff, media download, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "URL preview read/cache path",
        base_module: "LinkPreview + LinkPreviewCache",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "LinkPreviewCache may submit the existing Matrix GetUrlPreview read request only for a missing accepted URL cache entry; LoadedLinkPreview, Requested, Failed, cleanup, rate-limit retry scheduling, insert_into_cache, TimelineUpdate::LinkPreviewFetched, and SignalToUI only update local URL preview cache/redraw state and send no Matrix alias resolution, room preview fetch, event context fetch, media download, browser handoff, message, room-state, membership, account/profile, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link preview/navigation",
        base_module: "RoomScreen link handler + MatrixRequest::PreviewMatrixLinkTarget",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "RoomScreen Matrix link handling keeps known room links on local room navigation, loaded room alias links on local RoomsList alias navigation, known user links on the profile pane handoff and its existing profile read path, and current-room event links on loaded local jump when the event id is already present in RoomScreen tl_state or on the existing BackwardsPaginateUntilEvent/PaginateTimeline read path when the event is missing from loaded rows. Unknown room ids, unknown room aliases, non-current-room event links, and other event links stay on MatrixRequest::PreviewMatrixLinkTarget. Unknown room ids and unknown aliases fetch compact room preview details through get_room_preview; event links can additionally fetch source-only JSON through Room::load_or_fetch_event when the previewed room is known to the current client, while full event context window fetch remains a gap. link parsing, preview staging, known-room navigation, loaded-alias navigation, profile-pane handoff, loaded-event local jump, current-room event pagination, compact room preview, and source-only preview event fetch send no join, knock, invite, server-side event context window fetch, external browser handoff before confirmation, message, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link compact preview live read wiring",
        base_module: "RoomScreen show_matrix_link_preview_request + MatrixRequest::PreviewMatrixLinkTarget + TimelineUpdate::MatrixLinkPreviewResult",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Unknown Matrix room ids, unknown room aliases, non-current-room event links, and other unresolved targets already submit the live compact MatrixRequest::PreviewMatrixLinkTarget read after RoomScreen stages the preview strip. SlidingSync fetches room preview details through fetch_room_preview_with_avatar, adds source-only Room::load_or_fetch_event JSON when an event id targets a room known to the current client, and returns TimelineUpdate::MatrixLinkPreviewResult for result or error metadata; failed Retry stays gated by PositiveConfirmationModal before resubmitting the same compact read. This partial-live path does not perform server-side alias route promotion, server-side event context window fetch, join, knock, invite, browser handoff, message mutation, room-state, membership, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation outside the compact preview/source read",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link loaded alias navigation",
        base_module: "RoomsList + RoomScreen handle_link_clicked",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Loaded room alias Matrix links scan already loaded RoomsList joined-room canonical_alias and alt_aliases metadata. A match emits NavigateToRoom for that loaded room and skips MatrixRequest::PreviewMatrixLinkTarget; unknown aliases continue to use compact room preview metadata. This adds no server-side alias resolution, join, knock, invite, event context fetch, timeline pagination/reload, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link loaded event context metadata",
        base_module: "RoomScreen jump_to_loaded_matrix_link_event + matrix_link_loaded_event_context_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Loaded current-room Matrix event links add context metadata from the already loaded RoomScreen tl_state row before local scroll/highlight: target event id, loaded item index, current-room relation, loaded event-id availability, compact plaintext snippet, local scroll/highlight action, and visible preview-strip Source affordance. This sends no MatrixRequest::BackwardsPaginateUntilEvent, PreviewMatrixLinkTarget follow-up, event context fetch, timeline pagination/reload, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link current-room event pagination",
        base_module: "RoomScreen paginate_current_room_matrix_link_event + BackwardsPaginateUntilEventRequest + MatrixRequest::PaginateTimeline",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Missing current-room Matrix event links stage the Matrix link preview strip, then reuse the existing BackwardsPaginateUntilEvent request sender and MatrixRequest::PaginateTimeline read path to load older timeline items until the event appears. TargetEventFound scrolls and highlights the row and refreshes the Matrix link preview strip to loaded/source-ready state. This remains current-room-only and sends no PreviewMatrixLinkTarget for that path, no server-side alias resolution, server-side event context fetch, non-current-room timeline pagination/reload, join, knock, invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link loaded event source modal",
        base_module: "RoomScreen open_telegram_matrix_link_loaded_event_source + EventSourceModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Source reuses the existing EventSourceModal when the cached Matrix link target is a current-room event already loaded in RoomScreen tl_state or when MatrixRequest::PreviewMatrixLinkTarget returned cached event source JSON from Room::load_or_fetch_event for a known previewed room. The loaded path derives the room id from current TimelineKind, event id and item index from the loaded row, and source text from loaded EventTimelineItem.latest_json when available. The preview-fetched path derives room id, event id, and JSON from the compact preview result. Missing, failed, unresolved, and source-less links keep Source as metadata only. Source click sends no follow-up Matrix request, BackwardsPaginateUntilEvent, event-context window fetch, timeline pagination/reload, join, knock, invite, external browser handoff, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link unknown target boundary evidence",
        base_module: "RoomScreen handle_link_clicked unknown Matrix ids/aliases/events",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Unknown Matrix room ids, unknown room aliases, non-current-room event links, and other unresolved Matrix targets use a compact MatrixRequest::PreviewMatrixLinkTarget room-preview read while matrix_link_resolution remains a base gap. Unknown room ids and unknown room aliases fetch room preview details; non-current-room event links fetch only the containing room preview. Cached room id or alias targets can refresh cached Server context through the same MatrixRequest::PreviewMatrixLinkTarget read, be confirmed into MatrixRequest::JoinRoomByIdOrAlias, or be confirmed into MatrixRequest::Knock with via servers, and cached Matrix user targets can be confirmed into MatrixRequest::InviteUser for the current room. Server-side event context remains blocked. Opening an external browser before confirmation, sending messages, mutating room-state, touching account/profile, gateway/runtime/auth, and unrelated live mutation paths remain unwired",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link target metadata preview",
        base_module: "RoomScreen handle_link_clicked + matrix_link_target_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link target popups summarize clicked MatrixId kind, target, via server count, current-room relation, loaded RoomsList room/alias state, loaded RoomScreen timeline event-id state, and whether the action is profile-pane handoff, local NavigateToRoom, local scroll/highlight, or compact MatrixRequest::PreviewMatrixLinkTarget room-preview read. The metadata is computed before action from already loaded UI state and sends no extra Matrix request beyond the existing compact preview read for unknown targets, no server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link preview result metadata",
        base_module: "PreviewMatrixLinkTarget result + matrix_link_preview_result_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link compact preview result popups summarize the FetchedRoomPreview returned by the existing PreviewMatrixLinkTarget get_room_preview read: canonical alias presence, topic state, joined and active member counts, room type, join rule, world-readable history flag, current-user room state, direct-room flag, hero count, avatar fetch/fallback state, and source-only Room::load_or_fetch_event status when an event id was requested for a known client room. This adds no server-side alias resolution, event context window fetch, timeline pagination/reload, join, knock, invite, message send/edit/redact, room-state mutation, membership change, account/profile request, gateway/runtime/auth request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link preview failure metadata",
        base_module: "PreviewMatrixLinkTarget error + matrix_link_preview_failure_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link compact preview failure popups and the RoomScreen preview strip summarize only the failed PreviewMatrixLinkTarget get_room_preview result: target, via server count, requested event-id state, error message length, and boundary note. Failed Retry is visible only from cached target metadata and still requires confirmation before the same compact preview read. This adds no retry without confirmation, no follow-up Matrix request beyond the confirmed compact preview read, no server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, message send/edit/redact, room-state mutation, membership change, account/profile request, gateway/runtime/auth request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link preview retry confirmation",
        base_module: "RoomScreen Matrix link preview strip + PositiveConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link compact preview failed-state Retry reuses only the cached originating TimelineKind, room-or-alias id, via list, and optional event id from the failed PreviewMatrixLinkTarget attempt. Retry opens PositiveConfirmationModal before another compact room-preview read is submitted; unavailable cached target, unavailable TimelineKind, and confirmation cancel stay local. This sends no automatic retry, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, external browser handoff, message send/edit/redact, room-state mutation, membership change, account/profile request, gateway/runtime/auth request, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link server context boundary",
        base_module: "RoomScreen matrix_link_server_context_boundary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen Matrix link preview strip shows server/context boundary metadata from clicked target metadata and compact PreviewMatrixLinkTarget status: loading, resolved, failed, retry confirmation, cached Server context refresh, confirmed MatrixRequest::JoinRoomByIdOrAlias, confirmed MatrixRequest::Knock, confirmed MatrixRequest::InviteUser, or loaded current-room or preview-fetched event source state; via server count, optional event id, and retry cache readiness remain visible. Cached Server context refresh reuses MatrixRequest::PreviewMatrixLinkTarget read-only. event context fetch, timeline pagination/reload, MatrixRequest::BackwardsPaginateUntilEvent, external browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message send/edit/redact, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation remain blocked controls while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link context actions row",
        base_module: "RoomScreen matrix_link_context_actions + matrix_link_server_context_packet_snapshot_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen Matrix link preview strip exposes Server, Event, Alias, Join, Knock, Invite, Browser, and Source controls as visible context actions, with Browser confirmation-guarded before external opener handoff. Server uses a cached room id or alias target, via servers, and optional event id to submit the same MatrixRequest::PreviewMatrixLinkTarget compact read as a standalone Server context refresh when a target is cached, and falls back to the local packet snapshot when no cached target is available. Event renders a local Matrix link server-context packet snapshot from preview status, target label, via server list, requested event id, metadata/error length, retry cache state, and loaded current-room or preview-fetched source availability. Alias updates only the preview summary, context-action metadata, server/context boundary text, visible strip state, and popup copy from the same cached target/retry state. Clicking Join parses the cached room id or alias target, opens PositiveConfirmationModal, and only the accept branch submits MatrixRequest::JoinRoomByIdOrAlias with cached via servers; RoomScreen consumes MatrixLinkJoinResultAction for joined/failed strip state and failed-state retry cache. Clicking Knock parses the same cached room id or alias target, opens PositiveConfirmationModal, and only the accept branch submits MatrixRequest::Knock with cached via servers and no reason; RoomScreen consumes KnockResultAction for knocked/failed strip state and failed-state retry cache. Clicking Invite parses the cached Matrix user target, requires a loaded current room, opens PositiveConfirmationModal, and only the accept branch submits MatrixRequest::InviteUser for that room/user pair; RoomScreen consumes InviteResultAction for invited/failed strip state and failed-state retry cache. Browser builds a cached matrix.to URL from preview target/via/event state, opens PositiveConfirmationModal, and only the accept branch hands that URL to the system opener. Source may open the existing EventSourceModal for a cached current-room event already loaded in RoomScreen tl_state or a preview-fetched source JSON result from compact PreviewMatrixLinkTarget; otherwise it stays metadata-only. PreviewMatrixLinkTarget is limited to compact preview, confirmed failed-state Retry, or cached Server context refresh; BackwardsPaginateUntilEvent stays limited to current-room event pagination; event context fetch, timeline pagination/reload, unconfirmed invite, unconfirmed browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, account/profile, gateway/runtime/auth, or unrelated live mutation stay blocked while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link room-or-alias Join live wiring",
        base_module: "RoomScreen show_telegram_matrix_link_join_confirmation + MatrixRequest::JoinRoomByIdOrAlias + MatrixLinkJoinResultAction",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Join is live for cached room id or alias targets. RoomScreen parses the cached preview-strip target, opens PositiveConfirmationModal, submits MatrixRequest::JoinRoomByIdOrAlias with cached via servers only after accept, records submitted state in the Matrix link strip, consumes MatrixLinkJoinResultAction::Joined/Failed for result metadata, and keeps failed-state room-or-alias retry confirmation. Knock and Invite have their own confirmed MatrixRequest::Knock and MatrixRequest::InviteUser handoffs; cached Server context refresh is a separate read-only PreviewMatrixLinkTarget path, while event context fetch, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation remain blocked while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link room-or-alias Knock live wiring",
        base_module: "RoomScreen show_telegram_matrix_link_knock_confirmation + MatrixRequest::Knock + KnockResultAction",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Knock is live for cached room id or alias targets. RoomScreen parses the cached preview-strip target, opens PositiveConfirmationModal, submits MatrixRequest::Knock with cached via servers and no reason only after accept, records submitted state in the Matrix link strip, consumes KnockResultAction::Knocked/Failed for result metadata, and keeps failed-state room-or-alias retry confirmation. Invite has its own confirmed MatrixRequest::InviteUser handoff; cached Server context refresh is a separate read-only PreviewMatrixLinkTarget path, while event context fetch, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation remain blocked while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link user Invite live wiring",
        base_module: "RoomScreen show_telegram_matrix_link_invite_confirmation + MatrixRequest::InviteUser + InviteResultAction",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Invite is live for cached Matrix user id targets in the currently loaded room. RoomScreen opens the existing profile pane for the user link, stages the Matrix link strip with the user id and current room id, opens PositiveConfirmationModal, submits MatrixRequest::InviteUser only after accept, records submitted state in the Matrix link strip, consumes InviteResultAction::Sent/Failed for result metadata, and keeps failed-state room/user retry confirmation. Join and Knock keep their own room-or-alias confirmation paths; cached Server context refresh is a separate read-only PreviewMatrixLinkTarget path, while event context fetch, message mutation, room-state, account/profile, gateway/runtime/auth, and unrelated live mutation remain blocked while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link browser handoff confirmation",
        base_module: "RoomScreen show_telegram_matrix_link_browser_confirmation + PositiveConfirmationModal + matrix.to system opener",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Browser uses only cached RoomScreen preview-strip target, via server label, and requested event id to build a matrix.to URL, then opens PositiveConfirmationModal before any external handoff. Accept reuses the existing show_external_link_confirmation/robius_open system opener path; cancel and missing cached target stay local warning-only. It sends no PreviewMatrixLinkTarget, BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, Telegram delivery, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link route-scope controls row",
        base_module: "RoomScreen matrix_link_route_scope_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen Matrix link preview strip exposes Room, Event, Via, Preview, Source, Packet, Contract, and Taxonomy controls as visible local route-scope actions; the Room, Event, Via, Preview, and Source scope remains unchanged while Packet adds the acceptance packet, Contract adds typed route/result contracts, and Taxonomy adds route/event-context result slots. Room copies only the cached Matrix link target label/status/via/event metadata to the local clipboard when a target label exists. Via copies only the cached Matrix link via server list to the local clipboard when a via list exists. Event copies only the cached requested Matrix event id to the local clipboard when an event id exists. Preview copies only the already cached preview metadata/status/target/via/event summary to the local clipboard when metadata exists. Source may open the existing EventSourceModal for a cached current-room event already loaded in RoomScreen tl_state or a preview-fetched source JSON result from compact PreviewMatrixLinkTarget; otherwise it stays metadata-only. Packet copies a local per-target route acceptance matrix; Contract and Taxonomy remain local packet copy controls. It does not submit PreviewMatrixLinkTarget outside the existing confirmed failed-state Retry, BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link route drilldown packet",
        base_module: "RoomScreen copy_telegram_matrix_link_route_drilldown_packet + matrix_link_route_drilldown_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Packet copies a local per-target route drilldown from cached RoomScreen preview-strip state only. The packet records room target, event id, via servers, preview metadata, server-context packet, alias resolution, join/knock/invite, external browser handoff, and loaded and preview-fetched source acceptance slots before any route is promoted. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh. It submits no BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link route result contract packet",
        base_module: "RoomScreen copy_telegram_matrix_link_route_result_contract_packet + matrix_link_route_result_contract_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Contract copies a local typed route/result contract packet from cached RoomScreen preview-strip state only. The packet maps target identity, preview request/result/error, loaded alias and room routes, event route, via route, join/knock/invite, external browser handoff, loaded source, preview-fetched source, full remote source, retry, and source-hash acceptance slots before any route is promoted. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh. It submits no BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link route result taxonomy packet",
        base_module: "RoomScreen copy_telegram_matrix_link_route_result_taxonomy_packet + matrix_link_route_result_taxonomy_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Taxonomy copies a local route/event-context result taxonomy packet from cached RoomScreen preview-strip state only. It names the existing live references as loaded alias navigation, loaded current-room event jump, current-room missing-event BackwardsPaginateUntilEvent/PaginateTimeline read wiring, compact PreviewMatrixLinkTarget room-preview read, cached Server context refresh, confirmed failed-state Retry, source-only Room::load_or_fetch_event for known previewed room events, loaded or preview-fetched EventSourceModal Source, confirmed matrix.to Browser opener, confirmed JoinRoomByIdOrAlias, confirmed MatrixRequest::Knock, and confirmed current-room MatrixRequest::InviteUser result/retry. It records route_adapter_request_id, alias_resolution_operation_id, non_current_room_event_context_operation_id, via_route_request_id, full_remote_source_request_id, event_context_window_result, alias_resolution_result, via_resolution_result, full_remote_source_result, access_denied_result, stale_target_result, retry_cancel_result, and audit redaction slots as not_assigned or not_wired before a richer route adapter can be promoted. It submits no PreviewMatrixLinkTarget beyond explicit compact preview, Server refresh, or confirmed Retry controls, no BackwardsPaginateUntilEvent outside current-room missing event link pagination, no server-side alias resolution, no event-context fetch, no non-current-room timeline pagination/reload, no full remote source fetch, no unconfirmed browser handoff, no unconfirmed join/knock/invite, no message mutation, room-state, membership outside confirmed join/knock/invite paths, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link room target clipboard",
        base_module: "RoomScreen copy_telegram_matrix_link_room_target + local clipboard",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Room route-scope action copies only the cached Matrix link target label/status/via/event metadata from the RoomScreen preview strip to the local clipboard when a target label exists. Missing target label stays local-unavailable and writes no clipboard payload. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh. It submits no BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link via servers clipboard",
        base_module: "RoomScreen copy_telegram_matrix_link_via_servers + local clipboard",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Via route-scope action copies only the cached Matrix link via server list from the RoomScreen preview strip to the local clipboard when a via list exists. Missing via server list stays local-unavailable and writes no clipboard payload. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh. It submits no BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link event id clipboard",
        base_module: "RoomScreen copy_telegram_matrix_link_event_id + local clipboard",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Event route-scope action copies only the cached requested Matrix event id from the RoomScreen preview strip to the local clipboard when one exists. Missing event id stays local-unavailable and writes no clipboard payload. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh. It submits no BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link preview metadata clipboard",
        base_module: "RoomScreen copy_telegram_matrix_link_preview_metadata + local clipboard",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Matrix link Preview route-scope action copies only already cached RoomScreen preview-strip metadata to the local clipboard: preview status, target label, via server count, requested event id, retry-cache readiness, and the current local preview metadata text. Missing metadata stays local-unavailable and writes no clipboard payload. PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh. It submits no BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, join, knock, invite, browser handoff, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Matrix link unresolved detail state",
        base_module: "RoomScreen matrix_link_unresolved_detail",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen Matrix link preview strip derives an unresolved detail label only from clicked target metadata, compact PreviewMatrixLinkTarget status, preview metadata length, optional error length, cached retry context, cached Join/Knock result state, and loaded current-room event source availability. The label reports selected Server/Event/Alias/Join/Knock/Source action, preview status, unresolved target, via server count, requested event-id state, metadata character count, optional error character count, and retry cache readiness. Clicking context controls updates only this local detail state, the summary, the server/context boundary label, and popup copy; Join and Knock can confirm before room-or-alias membership handoff, and Source may open only the already loaded current-room EventSourceModal. It submits no PreviewMatrixLinkTarget outside confirmed failed-state Retry, BackwardsPaginateUntilEvent, server-side alias resolution, event context fetch, timeline pagination/reload, invite, browser handoff before confirmation, source-only preview event source fetch stays on compact preview; full remote event source fetch, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while matrix_link_resolution remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "external link confirmation",
        base_module: "RoomScreen link handler + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "ordinary URL links and unhandled Matrix links open a local confirmation modal before the existing external browser handoff is requested; opening the confirmation, Cancel, and guard display send no browser handoff, Matrix event fetch, room preview fetch, message send, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media download/playback",
        base_module: "RoomScreen media message preview",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "plain file, audio, and video messages expose Download and Play links guarded by PositiveConfirmationModal before the local save dialog, with loaded filename, MIME type, size, duration, and dimensions previewed in confirmation when present; confirmed accept then submits MatrixRequest::SaveMedia, which fetches full-file media through the SDK media cache path and writes it to the selected path. Play then asks the system opener to open that saved file. Row-scoped Retry on plain File/Audio/Video recovery and preflight controls reopens the same confirmation and resubmits MatrixRequest::SaveMedia after a picked save path; encrypted or missing-source Retry stays local. Successful plain-MXC SaveMedia results cache destinations for Open folder and Replay local OS handoffs. Encrypted image rows expose loaded ImageInfo metadata only; Decrypt, inline audio/video player controls, codec/transcode work, queue retry/resume/cancel controls, and richer playback remain TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media download/playback local boundary evidence",
        base_module: "RoomScreen image/file/audio/video/encrypted media preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "File, Audio, and Video Download/Play affordances use PositiveConfirmationModal before opening the local save dialog and preview already loaded type, filename, MIME type, size, duration, and dimensions when present; confirmed accept submits MatrixRequest::SaveMedia, fetches full-file media through the SDK media cache path, saves to the selected path, and optionally hands the saved file to the system opener. Row-scoped Retry with a plain MXC source reuses the same confirmation plus MatrixRequest::SaveMedia path, while encrypted or missing-source Retry stays local. Encrypted image rows preview already loaded ImageInfo metadata only; Decrypt, image decode, thumbnail fetch, codec/transcode controls, inline audio/video player controls, queue retry/resume/cancel controls, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain unwired local evidence while media_download_playback remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media download/playback metadata preview",
        base_module: "RoomScreen media action confirmation",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Download/Play action URLs carry loaded timeline media metadata for file/audio/video type, filename, MIME type, size, duration, and dimensions into the confirmation body and popup before the local save dialog. The preview sends no extra media fetch, decrypt, codec/transcode, inline playback, retry/cancel queue control, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation before confirmed MatrixRequest::SaveMedia",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media metadata clipboard",
        base_module: "RoomScreen Copy metadata link + loaded file/audio/video metadata",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "File, Audio, and Video media rows expose Copy metadata as a true local clipboard action. It writes only already loaded timeline media metadata: kind, filename, MIME type, size, duration, dimensions, and compact summary, including encrypted file/audio/video rows where media bytes are not needed. It submits no FetchMedia, SaveMedia, decrypt, codec/transcode, inline playback, system opener, retry/cancel queue control, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while media_download_playback remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media save dialog lifecycle metadata",
        base_module: "RoomScreen media action confirmation + save dialog popup states",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Media Download/Play confirmation opened, confirmation canceled, save dialog accepted, save dialog canceled, and unsupported save-dialog popup states reuse only the loaded media action metadata summary for type, filename, MIME type, size, duration, and dimensions. MatrixRequest::SaveMedia remains gated behind confirmation accept plus a selected local save path, while canceled/unsupported lifecycle states submit no SaveMedia, extra media fetch, decrypt, codec/transcode, inline playback, retry/cancel queue control, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media save destination metadata",
        base_module: "RoomScreen submit_media_save_after_path_pick + media_save_destination_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After PositiveConfirmationModal accept and a local save dialog picked path, Download/Play popup metadata includes selected destination path, loaded filename/type metadata, and whether Play will ask the system opener after MatrixRequest::SaveMedia writes the file. Confirmation cancel, save dialog cancel, unsupported save dialog, retry/cancel queue controls, inline audio/video controls, decrypt, codec/transcode, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain unwired",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media inline playback queue boundary metadata",
        base_module: "RoomScreen media action confirmation",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Media Download/Play confirmation body carries loaded file/audio/video action metadata plus an explicit inline playback and queue boundary. Download only writes a picked local file after confirmation; Play first submits MatrixRequest::SaveMedia after confirmation and a picked path, then asks the system opener to open the saved file. Inline audio/video controls, decrypt, codec/transcode, retry/cancel queue controls, attachment send, message mutation, room-state, membership, gateway/runtime/auth, and live mutation remain unwired",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media inline player disabled controls",
        base_module: "RoomScreen audio/video message rows + media_inline_player_disabled_controls_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Audio and Video timeline rows render a visible disabled inline-player control strip from already loaded timeline metadata: media kind, filename, duration, MIME type, size, and dimensions when present. Playhead, Seek, Queue, Decrypt, and Codec controls are visibly disabled while Download/Play stay the only active links; the strip sends no FetchMedia, SaveMedia, decrypt, codec/transcode, inline player startup, playback progress subscription, retry/cancel queue control, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media codec transcode controls row",
        base_module: "RoomScreen media_codec_transcode_controls_preview + handle_media_codec_transcode_control_link",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Audio and Video timeline rows expose Codec, Transcode, Captions, Quality, and Decrypt as visible local codec/transcode controls next to the disabled inline-player strip. Clicking a control only routes through handle_media_codec_transcode_control_link, rebuilds already loaded media metadata from the local link query, and shows popup copy. It does not submit FetchMedia, submit SaveMedia, start a decoder, start a transcoder, inspect codec support beyond loaded MIME/duration/dimensions labels, fetch captions, change playback quality, decrypt media, start inline playback, invoke the system opener, mutate retry/cancel queue state, attach/send media, mutate room-state, membership, account/profile, gateway/runtime/auth, or live mutation while media_download_playback remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media save result/open status boundary",
        base_module: "RoomScreen media_save_result_status_boundary_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Media Download/Play result boundary metadata states that MatrixRequest::SaveMedia completion reports saved, download failed, save failed, system opener opened, opener failed, and invalid saved-path states through local popup status via SaveMediaOpenOutcome, then sends TimelineUpdate::MediaSaveResult so RoomScreen caches successful plain-MXC destinations for the result-row Open folder and Replay handoffs. Inline audio/video player state, seek controls, retry/cancel queue controls, decrypt retry, codec/transcode fallback, background download list, delivery/read receipts, message mutation, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain local blocked controls; the metadata submits no extra FetchMedia, SaveMedia, retry, queue cancel, decrypt, codec, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media save retry confirmation",
        base_module: "RoomScreen handle_media_result_control_link + handle_media_save_preflight_control_link + show_media_save_confirmation",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "File, Audio, and Video recovery/preflight Retry links carry the row plain MXC source when available. Clicking Retry confirms through PositiveConfirmationModal, then reuses the same MatrixRequest::SaveMedia Download/Play handoff after the user picks a save path. Encrypted rows or links without a plain MXC source show a local unavailable label. This adds no unconfirmed FetchMedia, no unconfirmed SaveMedia, no cached-destination Open folder/Replay handoff from Retry, no retry automation, no queue resume/cancel, no decrypt retry, no codec/transcode, no background download mutation, no delivery/read receipt, no attachment send, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation outside the confirmed SaveMedia retry",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media save result recovery controls row",
        base_module: "RoomScreen media_save_result_recovery_controls_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "File, Audio, and Video media rows expose Open folder, Replay, Retry, Queue, and Decrypt as Save/Open recovery controls. Open folder is a live local OS handoff only after a successful MatrixRequest::SaveMedia result cached the selected destination for the same plain MXC; it validates the cached saved file is still a regular local file, clears stale cached destinations, opens the saved file's parent folder through the system opener, and sends no Matrix request. Replay is a live local OS handoff from the same cached successful SaveMedia destination: it validates the saved file is still a regular local file, clears stale cached destinations, converts it to a file URL, and opens it through the system opener without FetchMedia or SaveMedia. Retry routes through handle_media_result_control_link; with a plain MXC source it confirms through PositiveConfirmationModal and reuses MatrixRequest::SaveMedia after a picked save path, while missing-source Retry shows a local unavailable label. Queue renders a local media playback/download queue snapshot from requested action mode, loaded metadata, save-result boundary, opener state, and cached saved-file status; stale Queue snapshots clear the cached destination locally. Decrypt only rebuilds loaded metadata from the local link query and shows popup copy. It sends no unconfirmed FetchMedia, no unconfirmed SaveMedia, no automatic retry, no queue control, no queue retry/resume/cancel, no decrypt retry, no codec/transcode, no background download mutation, no delivery/read receipt, no attachment send, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation outside confirmed SaveMedia retry and cached-destination Open folder/Replay while media_download_playback remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media cached saved-file status snapshot",
        base_module: "RoomScreen media_cached_saved_file_status_label + Queue control",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Media Queue reads only local filesystem metadata for an already cached successful SaveMedia destination: regular-file state, size, readonly bit, and modified timestamp seconds. Missing, inaccessible, or non-file cached destinations clear the cached MXC destination locally before any Open folder or Replay handoff. This submits no FetchMedia, no SaveMedia, no inline player, no decoder, no queue retry/resume/cancel, no system opener, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation while media_download_playback remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media save preflight detail controls row",
        base_module: "RoomScreen media_save_preflight_detail_controls_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "File, Audio, and Video media rows expose Request, Result, Error, Retry, and Source as SaveMedia preflight detail controls. Retry routes through handle_media_save_preflight_control_link; with a plain MXC source it confirms through PositiveConfirmationModal and reuses MatrixRequest::SaveMedia after a picked save path, while missing-source Retry shows a local unavailable label. Request, Result, Error, and Source only rebuild loaded metadata from the local link query and show popup copy for the requested SaveMedia phase, cached result/error shape, retry availability, and source metadata. It sends no unconfirmed FetchMedia, no unconfirmed extra SaveMedia, no cached-destination Open folder/Replay handoff from preflight details, no automatic retry, no queue control, no decrypt retry, no codec/transcode, no background download mutation, no delivery/read receipt, no attachment send, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation outside the confirmed SaveMedia retry while media_download_playback remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media operation packet drilldown",
        base_module: "RoomScreen media_operation_packet_payload + Packet link",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "File, Audio, and Video media rows expose Packet as a visible local clipboard drilldown for plain and encrypted media. Packet copies a local media operation acceptance matrix from already loaded timeline metadata: requested action, SaveMedia request/result shape, cached Open folder/Replay destination slots, inline playback slot, decrypt/decode slot, codec/transcode slot, captions slot, queue retry/resume/cancel slot, system opener result slot, and promotion criteria for the media_download_playback base gap. It submits no FetchMedia, SaveMedia, system opener request outside cached Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt retry, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media playback queue contract packet",
        base_module: "RoomScreen media_playback_queue_contract_payload + Contract link",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "File, Audio, and Video media rows expose Contract as a visible local typed playback/media queue contract for plain and encrypted media. Contract copies typed slots from already loaded timeline metadata for media identity, SaveMedia request/result/error, cached Open folder/Replay destination result with stale cache validation and eviction, inline playback request/result/error/progress, decrypt/decode request/result/error, codec/transcode/captions/quality fallback, system opener result, queue retry/resume/cancel/background persistence, delivery/read receipt mapping, source metadata hashing, broader stale local file handling beyond cached Open folder/Replay validation, and adapter promotion blockers before true inline/decrypt/queue controls can be wired. It submits no FetchMedia, SaveMedia, system opener request outside cached-destination Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt retry, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while media_download_playback remains a base gap beyond confirmed SaveMedia",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media playback result taxonomy packet",
        base_module: "RoomScreen media_playback_result_taxonomy_payload + Taxonomy link",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "File, Audio, and Video media rows expose Taxonomy as a visible local decrypt/decode/opener/queue result packet for plain and encrypted media. Taxonomy copies only already loaded timeline metadata and names the live references as MatrixRequest::FetchMedia image/cache reads, confirmed MatrixRequest::SaveMedia Download/Play result mapping, cached Open folder/Replay stale validation and local OS opener handoff, and PositiveConfirmationModal-gated SaveMedia Retry. Inline playback session/progress, encrypted-media decrypt/decode, codec/transcode/captions/quality fallback, background queue retry/resume/cancel, delivery/read receipt mapping, and stale inline/decrypt local-file handling remain not-assigned/not-wired result slots. It submits no FetchMedia, SaveMedia, system opener request outside cached Open folder/Replay, inline player startup, playback progress subscription, key lookup, decrypt retry, decoder, transcoder, captions fetch, quality switch, retry/cancel queue mutation, background download mutation, delivery/read receipt, attachment send, message mutation, room-state, membership, account/profile, gateway/runtime/auth, Telegram delivery, or live mutation while media_download_playback remains a base gap beyond confirmed SaveMedia",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media encrypted metadata local preview",
        base_module: "RoomScreen encrypted file/audio/video local disabled preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Encrypted File, Audio, and Video timeline rows display already loaded filename, MIME type, size, duration, and dimensions metadata in the disabled media preview while keeping Download, Play, Decrypt, SaveMedia, FetchMedia, codec/transcode, inline playback, retry/cancel queue control, attachment send, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation unwired",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "media encrypted image metadata local preview",
        base_module: "RoomScreen encrypted image local disabled preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Encrypted Image timeline rows display already loaded ImageInfo filename/body, MIME type, size, dimensions, blurhash availability, and thumbnail-source availability in the disabled image preview while keeping Decrypt, SaveMedia, FetchMedia, image decode, thumbnail fetch, media cache mutation, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation unwired",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "image viewer local controls",
        base_module: "ImageViewer + RoomScreen image click",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "image clicks use the existing media cache/fetch path to load content, while Close, Escape, background tap, Zoom, Rotate, Reset, pan, pinch, and overlay auto-hide only update local viewer state and send no additional FetchMedia, download, playback, decrypt, message, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "ProfileIcon settings navigation local/cache evidence",
        base_module: "NavigationTabBar ProfileIcon + HomeScreen SettingsScreen",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "ProfileIcon displays own avatar/display name from get_own_profile, user_profile_cache, and avatar_cache read/cache state; clicking it only emits OpenSettings, selects the local Settings tab, populates SettingsScreen from current AppState/cache, and sends no account mutation, profile mutation, message, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Settings close previous-selection local evidence",
        base_module: "SettingsScreen + HomeScreen CloseSettings",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "SettingsScreen close button, Escape, back gesture, and mouse back only emit CloseSettings; HomeScreen restores previous_selection in local UI state, broadcasts the restored tab, and sends no logout, account mutation, profile mutation, message, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "profile/avatar",
        base_module: "UserProfile + Avatar",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "user profile, avatar cache, room member profile lookup, account settings avatar controls, display-name staging before the existing SetDisplayName path, profile/account identity clipboard copy, direct-message confirmation before the existing OpenOrCreateDirectMessage lookup path, and local-only read receipt jump preview",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "profile/member read path",
        base_module: "UserProfileSlidingPane + user_profile_cache",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "User Profile populates display name, avatar, membership, and role from loaded user_profile_cache state or the existing Matrix GetUserProfile/profile-member read path; Loaded entries are reused locally, Requested entries suppress duplicates, fetch_if_missing only submits GetUserProfile read requests, and opening or refreshing the pane starts no profile mutation, ignore/block, direct-message, message, room-state, or membership mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "profile/account identity clipboard",
        base_module: "UserProfileSlidingPane + AccountSettings clipboard actions",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Copy Link to User builds a matrix.to URI from the loaded user id, and Copy User ID uses the loaded own profile id; both only write clipboard text locally without Matrix profile lookup, account request, event fetch, message send, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "profile direct message confirmation",
        base_module: "UserProfileSlidingPane + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Direct Message opens a confirmation modal before the existing Matrix OpenOrCreateDirectMessage lookup path is requested with allow_create=false, and Cancel keeps the request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "direct message create confirmation",
        base_module: "App DirectMessageRoomAction + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "When no existing DM room is found, Create New Direct Message keeps the existing Matrix OpenOrCreateDirectMessage create path behind a second confirmation with allow_create=true, and Cancel keeps the create request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "profile ignore/block confirmation",
        base_module: "UserProfileSlidingPane + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Ignore and Unignore open a confirmation modal before the existing Matrix IgnoreUser path is requested, and Cancel keeps the request unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account display name",
        base_module: "AccountSettings display name staging preview + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account Settings stages display name drafts locally until Save Name is confirmed; Matrix SetDisplayName is requested only from the confirmed accept handler, DisplayNameChanged repaints cached profile/input state locally, and Cancel/reset keeps SetDisplayName, avatar, account, device/session-management, message, room-state, and membership requests unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account device self-check",
        base_module: "AccountSettings verification banner + Matrix GetOwnDevice",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account Settings uses the existing Matrix GetOwnDevice read path only while own_device is missing; fetched Device data populates verified/unverified device evidence, session name, and Device ID locally while account mutation, device-list lookup, session-management, profile mutation, message, room-state, and membership requests remain unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar upload",
        base_module: "AccountSettings avatar upload picker + ConfirmationModal",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "Account Settings now uses a desktop image picker, selected-file metadata preview with header-only dimensions status, local file validation, confirmation before MatrixRequest::UploadAvatar, and bounded local Thumbnail/Full-size pixel decode for selected images; the confirmed handler calls client.account().upload_avatar, whose SDK path uploads media and calls Account::set_avatar_url(Some(mxc)) before AvatarChanged(Some(mxc)) repaints. The direct MXC editor separately validates mxc:// input, confirms, and submits MatrixRequest::SetAvatar(Some) through client.account().set_avatar_url(Some), while Crop, Cancel, picker cancel, invalid files, mobile camera/photo-library capture, image editing beyond local preview decode, persistent thumbnail file handoff, browser handoff, message, room-state, membership, gateway/runtime/auth, and unconfirmed live mutation remain TODO/local",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar upload live wiring",
        base_module: "AccountSettings avatar upload picker + MatrixRequest::UploadAvatar + client.account().upload_avatar",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account avatar upload is partial-live for the confirmed desktop selected avatar image path: PositiveConfirmationModal accept submits MatrixRequest::UploadAvatar, the Matrix worker calls client.account().upload_avatar, the SDK uploads media and calls Account::set_avatar_url(Some(mxc)), and AvatarChanged(Some(mxc)) repaints profile/avatar state locally. Failed Retry reuses the cached selected local file and MIME type behind the same confirmation path, while cropper/editor, camera/photo-library capture, transformed thumbnail/full image handoff, richer mobile UX, gateway/runtime/auth expansion, and unconfirmed live mutation remain blocked/local",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar direct MXC SetAvatar(Some)",
        base_module: "AccountSettings avatar_direct_mxc_input + MatrixRequest::SetAvatar(Some) + client.account().set_avatar_url(Some)",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Direct avatar MXC editor validates an existing mxc:// URI locally, opens PositiveConfirmationModal, submits MatrixRequest::SetAvatar(Some) only from the confirmed accept handler, and SlidingSync reuses client.account().set_avatar_url(Some) before AvatarChanged(Some(mxc)) repaints profile/avatar state. AvatarChangeFailed keeps the cached mxc:// URI for a confirmed Retry through the same SetAvatar(Some) path; invalid input, cancel, UploadAvatar, cropper/editor, camera/photo-library, transformed thumbnail handoff, browser handoff, message, room-state, membership, gateway/runtime/auth, and unconfirmed live mutation stay unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar selected file preview",
        base_module: "AccountSettings avatar_upload_selection_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After local avatar file validation, AccountSettings previews selected filename, MIME type, local file size, extension, and dimensions status before confirmation. The preview, confirmation open/cancel, Crop, preview Cancel, picker cancel, invalid files, camera/photo-library capture, image editing, thumbnail generation, browser handoff, account-data mutation beyond confirmed upload_avatar set_avatar_url(Some), message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, and live mutation remain local until the confirmed MatrixRequest::UploadAvatar branch runs",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar selected image metadata preview",
        base_module: "AccountSettings avatar_upload_selection_preview header metadata",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Selected avatar image metadata reads only the already selected local file header to surface dimensions status for PNG, JPEG, GIF, BMP, or WebP alongside filename, MIME type, local file size, and extension before confirmation. It performs no thumbnail decode, full image decode, cropper/editor work, camera/photo-library capture, browser handoff, upload, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation before the confirmed MatrixRequest::UploadAvatar branch runs",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar thumbnail/full-size pixel decode",
        base_module: "AccountSettings avatar_source_preview_thumbnail_button + avatar_source_preview_full_size_button + account_avatar_upload_decode_probe_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload Thumbnail and Full-size controls run a bounded local pixel decode against only the already selected image file. The decode reuses the PNG/JPEG/GIF/BMP/WebP header parser for dimensions, byte budget, and pixel budget, then Thumbnail generates an in-memory 128px RGBA thumbnail buffer and Full-size decodes the original RGBA pixel buffer. It creates no thumbnail file, runs no cropper/editor transform, opens no camera/photo-library/browser handoff, and submits no UploadAvatar, SetAvatar(Some), account/profile mutation, message mutation, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation while account_avatar_upload remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar upload lifecycle metadata",
        base_module: "AccountSettings avatar upload picker + preview + confirmation callbacks",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload picker opened, picker canceled, picker unsupported, invalid selection, confirmation opened, confirmation canceled, confirmed upload handoff, Crop, and preview Cancel popup states reuse only the selected-file metadata summary when available: filename, MIME type, local file size, extension, dimensions status, and validation reason for invalid files. MatrixRequest::UploadAvatar remains gated behind valid desktop image selection and the confirmed accept handler; direct MXC SetAvatar(Some) uses its own confirmation path. Picker cancel, invalid files, Crop, preview Cancel, unsupported platforms, camera/photo-library capture, image editor, thumbnail generation, browser handoff, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, and unconfirmed live mutation stay local/unwired",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar upload retry confirmation",
        base_module: "AccountSettings show_avatar_upload_retry_confirmation",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Failed avatar upload Retry reuses only the cached local file path and MIME type from the last validated selected image, opens PositiveConfirmationModal, and requests MatrixRequest::UploadAvatar only from the confirmed accept handler. Direct MXC failed-state Retry separately reuses only the cached mxc:// URI and confirms before MatrixRequest::SetAvatar(Some). Missing cached selection and confirmation cancel remain local; Retry sends no new picker, cropper/editor, thumbnail decode, camera/photo-library capture, browser handoff, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or unconfirmed live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar crop editor boundary",
        base_module: "AccountSettings account_avatar_upload_crop_editor_boundary_label + account_avatar_upload_cropper_snapshot_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload crop/editor boundary metadata is derived only from AvatarUploadPreviewState and selected local image metadata while the desktop picker plus confirmation-gated MatrixRequest::UploadAvatar path and direct MXC confirmation-gated MatrixRequest::SetAvatar(Some) path remain unchanged. Crop renders a local avatar cropper packet snapshot for crop box, aspect preset, rotate/zoom state, thumbnail target, camera/library source, browser handoff, and UploadAvatar handoff metadata. Crop, aspect-ratio presets, rotate/zoom, image editor controls, thumbnail generation, mobile camera capture, mobile photo-library capture, browser handoff, account-data mutation beyond confirmed UploadAvatar/direct SetAvatar, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, and live mutation remain local blocked controls",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar editor controls row",
        base_module: "AccountSettings avatar_editor_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload preview exposes Aspect, Rotate, Zoom, Camera, and Library as visible local controls. Aspect renders a local avatar cropper packet snapshot; other controls only update AccountSettings local preview metadata, crop/editor boundary copy, and popup text from AvatarUploadPreviewState plus selected image metadata when available. It starts no cropper/editor, aspect-ratio transform, rotate/zoom image decode, thumbnail generation, camera capture, photo-library picker, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, message mutation, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation while account_avatar_upload remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar source preview controls row",
        base_module: "AccountSettings avatar_upload_source_preview_controls + stage_avatar_upload_source_preview_control",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload preview exposes Source, Camera, Library, Thumbnail, Full-size, Packet, Contract, and Taxonomy as visible local source/preview controls. Source can copy the already selected local avatar file path to the clipboard; Thumbnail and Full-size run bounded local pixel decode with in-memory RGBA buffers; Camera, Library, Packet, Contract, and Taxonomy only update AccountSettings local source/preview metadata, selected-image copy, and popup text from AvatarUploadPreviewState plus selected image metadata when available. Packet persists the source/editor acceptance matrix; Contract maps that matrix to typed cropper, camera, image-edit, thumbnail/full-size decode, UploadAvatar, and SetAvatar contracts; Taxonomy records source/cropper/camera/library/thumbnail artifact result slots. It opens no file picker, camera capture, photo-library picker, persistent thumbnail file, cropper/editor, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, message mutation, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation while account_avatar_upload remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar source editor drilldown packet",
        base_module: "AccountSettings avatar_source_preview_packet_button + account_avatar_upload_source_editor_drilldown_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload Packet persists a local source/editor drilldown acceptance matrix from AvatarUploadPreviewState plus selected image metadata. It records source type, desktop file path handoff, MIME/extension/size/dimensions, crop box/aspect/rotate/zoom, thumbnail/full-size decode targets, camera/photo-library permission and picker states, image editor handoff, UploadAvatar request/result/error/retry/source slots, and SetAvatar handoff while account_avatar_upload remains a base gap. Packet starts no file picker, camera/photo-library permission, capture, source mutation, cropper/editor, thumbnail decode/generation, full image decode, UploadAvatar, SetAvatar(Some), account/profile mutation, message mutation, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar source editor typed contract packet",
        base_module: "AccountSettings avatar_source_preview_contract_button + account_avatar_upload_source_editor_typed_contract_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload Contract maps the local source/editor Packet acceptance matrix to a typed cropper-camera contract packet plus the now-live direct MXC SetAvatar(Some) result path from AvatarUploadPreviewState plus selected image metadata. It records typed source identity, desktop file handoff, camera/photo-library permission and picker request/result/error slots, cropper crop-box/aspect/rotate/zoom request/result/error slots, thumbnail/full-size decode request/result/error slots, image editor transform result slots, UploadAvatar request/result/error/retry/source slots, direct SetAvatar(Some) request/result/retry mapping, stale local file handling, source-hash, idempotency, and promotion blockers while account_avatar_upload remains a base gap. Contract starts no file picker, camera/photo-library permission, capture, source mutation, cropper/editor, thumbnail decode/generation, full image decode, UploadAvatar, account/profile mutation beyond separately confirmed direct SetAvatar(Some), message mutation, room-state, membership, account/device/session-management, gateway/runtime/auth, or unconfirmed live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar source editor result taxonomy packet",
        base_module: "AccountSettings avatar_source_preview_taxonomy_button + account_avatar_upload_source_editor_result_taxonomy_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload Taxonomy records a local source/editor result taxonomy packet from AvatarUploadPreviewState plus selected image metadata while account_avatar_upload remains a base gap. It names only confirmed desktop UploadAvatar plus SDK Account::set_avatar_url(Some), confirmed failed-state UploadAvatar Retry, direct MXC SetAvatar(Some) plus confirmed failed-state Retry, SetAvatar(None) delete, selected-file metadata, source-path clipboard, and bounded in-memory Thumbnail/Full-size pixel decode as live references. Source identity operation id, camera/photo-library permission results, camera capture result, photo-library selection result, crop box/aspect/rotate/zoom result, editor transform result, persistent thumbnail artifact id, transformed image hash, transformed upload result, transformed SetAvatar result, mobile capture result, stale source result, retry/cancel result, and audit redaction remain not_assigned or not_wired. Taxonomy starts no file picker, camera/photo-library permission, capture, source mutation, cropper/editor, persistent thumbnail generation, transformed image write, transformed UploadAvatar, transformed SetAvatar(Some), account/profile mutation beyond existing confirmed avatar upload/direct set/delete paths, message mutation, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar source path clipboard",
        base_module: "AccountSettings copy_avatar_upload_source_path",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload Source copies only the already selected local avatar file path from AvatarUploadSelectionPreview to the local clipboard. If no image has been selected, Source stays a local prompt. It opens no file picker, camera capture, photo-library picker, thumbnail decode/generation, full image decode, cropper/editor, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, message mutation, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation while account_avatar_upload remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar upload preflight detail controls row",
        base_module: "AccountSettings avatar_upload_preflight_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Avatar upload preview exposes Request, Result, Error, Retry, and Source as visible local UploadAvatar preflight detail controls. Clicking a control only updates AccountSettings local preflight metadata, preview copy, and popup text from AvatarUploadPreviewState plus selected image metadata when available. It opens no file picker, cropper/editor, image decode, thumbnail generation, camera capture, photo-library picker, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, message mutation, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation while account_avatar_upload remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar upload local boundary evidence",
        base_module: "AccountSettings avatar_upload_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Upload Avatar and Choose Photo open a desktop image picker and confirmation before MatrixRequest::UploadAvatar; confirmed UploadAvatar calls client.account().upload_avatar, whose SDK path uploads media and calls Account::set_avatar_url(Some(mxc)). Direct MXC SetAvatar(Some) validates mxc:// input, confirms, and calls client.account().set_avatar_url(Some). Crop, preview Cancel, picker cancel, invalid files, mobile camera/photo-library capture, image editing, thumbnail generation, browser handoff, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or unconfirmed live mutation remain unwired local evidence",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar delete confirmation",
        base_module: "AccountSettings + ConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Delete Avatar opens a confirmation modal before Matrix SetAvatar(None) is requested from the confirmed accept handler; AvatarChanged result data repaints cached profile/avatar widgets locally, and Cancel keeps SetAvatar(None), upload, display-name, account, device/session-management, message, room-state, and membership requests unsent",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account avatar delete live wiring",
        base_module: "AccountSettings delete_avatar_button + MatrixRequest::SetAvatar(None) + client.account().set_avatar_url(None)",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account avatar delete is partial-live: Delete Avatar opens PositiveConfirmationModal, the confirmed accept handler submits MatrixRequest::SetAvatar { avatar_url: None }, the Matrix worker calls client.account().set_avatar_url(None), and AvatarChanged(None) repaints cached profile/avatar widgets locally. Cancel keeps SetAvatar(None) unsent; confirmed UploadAvatar remains the avatar upload path; cropper/editor, camera/photo-library capture, direct SetAvatar(Some) handoff from editor, gateway/runtime/auth expansion, and unconfirmed live mutation remain blocked/local",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management",
        base_module: "AccountSettings manage account preview",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "Account Settings account management previews now combine loaded own_profile account identity with Matrix GetOwnDevice current-session/device details, the confirmed All devices MatrixRequest::GetDevices read-only directory when requested, and confirmed Browser/Portal homeserver system-browser handoff. Failed GetDevices results cache an error so Retry can confirm and resubmit the same read-only directory request. Manage Account, Security, Sessions, and Close update AccountManagementPreviewState and labels locally without password/SSO change, session-management mutation, cross-session revoke, Matrix account mutation, message, room-state, membership, or live mutation request, while dedicated account-management portal routes, password/SSO, session/device management writes, and Matrix account mutations beyond display name remain TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management live wiring",
        base_module: "AccountSettings own_profile identity + MatrixRequest::GetOwnDevice current-session refresh + MatrixRequest::GetDevices all-device directory + MatrixRequest::SetDisplayName + MatrixRequest::RenameDevice + show_account_management_device_rename_confirmation + show_account_management_browser_portal_confirmation",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management is partial-live for current-session reads, all-device directory reads/retry, profile display-name writes, current-device rename writes, and Browser/Portal homeserver opener: AccountSettings combines loaded own_profile identity with MatrixRequest::GetOwnDevice when current device data is missing, Refresh reuses PositiveConfirmationModal before the same GetOwnDevice request repaints Device ID, display name, verification, and session summary, All devices submits MatrixRequest::GetDevices and stores OwnDevicesFetched AccountDeviceDirectoryEntry rows for read-only directory summaries, failed OwnDevicesFetched errors are cached for a PositiveConfirmationModal Retry that resubmits MatrixRequest::GetDevices through the same read-only path, Save Name confirms MatrixRequest::SetDisplayName before client.account().set_display_name returns DisplayNameChanged or DisplayNameChangeFailed, Rename confirms MatrixRequest::RenameDevice for the loaded current Device ID before SlidingSync calls client.rename_device and posts AccountDataAction::DeviceRenamed, and Browser/Portal build get_client().homeserver(), strip query/fragment, require http/https, then confirm before handing the active Matrix homeserver URL to robius_open. DisplayNameChangeFailed keeps the draft editable so Save Name can open confirmation again and resubmit MatrixRequest::SetDisplayName through the same live path. Dedicated account-management portal routes, password/SSO, cross-session revoke, device delete/trust mutation, profile/account mutations beyond display name and current-device rename, gateway/runtime/auth expansion, and unconfirmed write-side live mutation remain blocked/local",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management browser portal handoff",
        base_module: "AccountSettings show_account_management_browser_portal_confirmation + get_client().homeserver + PositiveConfirmationModal + robius_open system opener",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Browser and Portal now build the active Matrix client's homeserver URL from get_client().homeserver(), validate http/https, strip query and fragment, show PositiveConfirmationModal, and only the accept branch hands that URL to robius_open. Missing client, invalid URL, cancel, and opener failure stay local popup states. This submits no MatrixRequest, starts no password/SSO flow, opens no dedicated account-management portal route beyond the homeserver root, performs no session-management lookup, cross-session revoke, device trust/rename/delete mutation, Matrix account/profile mutation, gateway/runtime/auth, Telegram delivery, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management display name live wiring",
        base_module: "AccountSettings Save Name confirmation + MatrixRequest::SetDisplayName + client.account().set_display_name",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account display-name editing is a live account-management slice: drafts stay local until Save Name opens PositiveConfirmationModal, the confirmed accept handler submits MatrixRequest::SetDisplayName, SlidingSync calls client.account().set_display_name(new_display_name.as_deref()), and DisplayNameChanged or DisplayNameChangeFailed repaints the cached profile/input state. DisplayNameChangeFailed re-enables the staged draft and Save Name can open confirmation again for a confirmed SetDisplayName resubmit; Cancel/reset keeps SetDisplayName unsent. Avatar, dedicated account portal routes, password/SSO, session/device mutation, gateway/runtime/auth expansion, and unconfirmed live mutation remain blocked/local",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management device directory live wiring",
        base_module: "AccountSettings All devices + MatrixRequest::GetDevices + client.devices + AccountDataAction::OwnDevicesFetched",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management All devices is a live read-only directory slice: clicking All devices switches the preview to Sessions, submits MatrixRequest::GetDevices, SlidingSync calls client.devices(), and AccountDataAction::OwnDevicesFetched stores AccountDeviceDirectoryEntry rows with device id, display name, last-seen IP, and last-seen timestamp summary or records an error. A failed result caches own_devices_last_error so Retry opens PositiveConfirmationModal before another MatrixRequest::GetDevices request is submitted through the same read-only path. Current-device Rename and Browser/Portal homeserver opener are handled separately; the directory result only repaints AccountManagementPreviewState labels and popup evidence. Password/SSO, dedicated account portal routes, cross-session revoke/trust, device delete/trust, account/profile mutation beyond current-device rename, gateway/runtime/auth expansion, and write-side live mutation remain blocked/local",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management current device rename live wiring",
        base_module: "AccountSettings show_account_management_device_rename_confirmation + MatrixRequest::RenameDevice + client.rename_device + AccountDataAction::DeviceRenamed",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management current-device Rename is a live write slice: clicking Rename requires loaded GetOwnDevice metadata, derives a bounded Hepta Native display name from own_profile, opens PositiveConfirmationModal, and only the accept branch submits MatrixRequest::RenameDevice for the current Device ID. SlidingSync calls client.rename_device, posts AccountDataAction::DeviceRenamed, and success refreshes MatrixRequest::GetOwnDevice plus MatrixRequest::GetDevices so current-session and directory metadata are reread. Missing current-device metadata, cancel, empty target name, failed rename_device, and result repaint stay explicit. Password/SSO, dedicated account portal routes, session-management lookup, cross-session revoke, device delete/trust mutation, account/profile mutation beyond current-device rename, gateway/runtime/auth expansion, Telegram delivery, and unconfirmed write-side live mutation remain blocked/local",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management loaded identity preview",
        base_module: "AccountSettings account_management_loaded_identity_text",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management loaded identity preview combines already loaded own_profile display name, Matrix user id, avatar state, existing Matrix GetOwnDevice current-session details, and the latest read-only GetDevices directory summary when loaded. Current-device Rename has a separate confirmation-gated MatrixRequest::RenameDevice path, and Browser/Portal can confirm before opening the active Matrix homeserver URL in the system browser; Manage Account, Security, Sessions, and Close only update local preview labels. This sends no Matrix profile lookup, avatar fetch, password/SSO change, session-management mutation, cross-session revoke, account-data mutation, Matrix account mutation beyond current-device rename, profile/account mutation beyond current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, or unconfirmed live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management lifecycle metadata",
        base_module: "AccountSettings account_management_lifecycle_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Manage Account, Security, Sessions, All devices, Browser/Portal, Rename, and Close popup states now reuse AccountManagementPreviewState plus loaded own_profile identity, current Matrix device/session metadata, and read-only GetDevices directory summaries from account_management_loaded_identity_text. Manage Account, Security, and Sessions request MatrixRequest::GetOwnDevice only when own_device is missing; All devices submits MatrixRequest::GetDevices as a read-only directory fetch; Rename opens PositiveConfirmationModal before MatrixRequest::RenameDevice for the current Device ID; Browser/Portal open PositiveConfirmationModal before the homeserver system opener handoff; Close only hides the local preview and reports the previous preview state. This sends no password/SSO change, session-management mutation beyond current-device rename, cross-session revoke, account-data mutation, Matrix account mutation beyond current-device rename, profile/account mutation beyond current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, or unconfirmed live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management refresh confirmation",
        base_module: "AccountSettings account_management_refresh_confirmation_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Refresh in the account management preview opens PositiveConfirmationModal before reusing MatrixRequest::GetOwnDevice for current-session metadata. The confirmed branch only refreshes current Device display name, Device ID, and verification labels already shown by AccountSettings; cancel, missing-device results, and repaint stay local without Matrix profile lookup, avatar fetch, external account page, browser handoff, password/SSO change, session-management mutation, cross-session revoke, account-data mutation, Matrix account mutation, profile/account mutation, message send/edit/redact, room-state, membership, account/device/session-management mutation, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management local boundary evidence",
        base_module: "AccountSettings account_management_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Manage Account, Security, Sessions, Close, All devices, Browser/Portal, Rename, and preview feedback can display loaded own_profile identity plus current Matrix device/session details from GetOwnDevice and read-only all-device directory details from GetDevices while account_management remains a base gap. Rename confirms before MatrixRequest::RenameDevice for the current Device ID, and Browser/Portal can confirm before opening the active Matrix homeserver URL through robius_open. Dedicated account-management portal routes, password change, SSO change, session-management mutation beyond current-device rename, cross-session revoke, account-data mutation, Matrix account mutation beyond display name/current-device rename, profile/account mutation beyond display name/current-device rename, message send/edit/redact, room-state, membership, account/device/session-management mutation beyond current-device rename, gateway/runtime/auth, and unconfirmed live mutation remain unwired local evidence",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management session revoke boundary",
        base_module: "AccountSettings account_management_session_revoke_boundary_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management session/revoke boundary metadata is derived only from AccountManagementPreviewState plus loaded own_profile identity, current-device GetOwnDevice text, read-only GetDevices directory summaries, the separate confirmed current-device Rename path, and the separate confirmed homeserver opener boundary. Dedicated external account page routes, password change, SSO change, all-device management beyond the read-only directory, session-management lookup, cross-session revoke, device delete/trust changes, account-data mutation, Matrix account/profile mutation beyond display name and current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, and unconfirmed live mutation remain local blocked controls; only confirmed current-device Rename submits MatrixRequest::RenameDevice",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management session actions row",
        base_module: "AccountSettings account_management_session_actions_row_label + show_account_management_device_rename_confirmation + show_account_management_browser_portal_confirmation",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management preview now exposes Revoke, Rename, Trust, and Browser. Rename opens PositiveConfirmationModal and only the accept branch submits MatrixRequest::RenameDevice for the loaded current Device ID through client.rename_device. Browser opens PositiveConfirmationModal and only the accept branch hands the active Matrix homeserver URL to robius_open. Revoke and Trust only update AccountManagementPreviewState, local labels, and popup copy from loaded own_profile identity plus current-device GetOwnDevice text; the row submits no all-device list lookup, session-management lookup, cross-session revoke, device delete/trust change, Matrix account/profile mutation beyond current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, or unconfirmed live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management device directory controls row",
        base_module: "AccountSettings account_management_device_directory_controls_row_label + show_account_management_browser_portal_confirmation",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management preview now exposes All devices, Password, SSO, Portal, and Activity as account/device-directory controls. All devices submits MatrixRequest::GetDevices and renders read-only OwnDevicesFetched summaries locally; failed GetDevices records a cached error for confirmed Retry through the same read-only path. Portal opens PositiveConfirmationModal and only the accept branch hands the active Matrix homeserver URL to robius_open. Password, SSO, and Activity only update AccountManagementPreviewState, local labels, and popup copy from loaded own_profile identity plus current-device GetOwnDevice text. The row performs no session-management mutation, password change, SSO start, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or write-side live mutation while account_management remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management current device metadata controls row",
        base_module: "AccountSettings account_management_device_metadata_controls + copy_account_management_current_device_id + copy_account_management_current_device_verification + copy_account_management_current_device_display_name + copy_account_management_current_session + copy_account_management_current_device_source_metadata + stage_account_management_current_device_metadata_control",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management preview now exposes Device, Verified, Display, Session, and Source as visible local current-device metadata controls. Device copies the loaded current Device ID from GetOwnDevice to the local clipboard when available; Verified copies the loaded current-device verification status from local Matrix verification state plus GetOwnDevice current device ID to the local clipboard when available; Display copies the loaded current device display name from GetOwnDevice to the local clipboard when available; Session copies the loaded current-session summary from GetOwnDevice to the local clipboard when available; Source copies loaded own_profile plus current-device summary text to the local clipboard. It requests no extra GetOwnDevice, opens no external account portal or browser, fetches no all-device list, performs no session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation while account_management remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management current device verification clipboard",
        base_module: "AccountSettings copy_account_management_current_device_verification + account_management_current_device_verification_clipboard_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Verified copies only the already loaded current-device verification status from local Matrix verification state plus the existing GetOwnDevice current device ID to the local clipboard. Missing current-device metadata stays a local prompt; the action sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management current device id clipboard",
        base_module: "AccountSettings copy_account_management_current_device_id + account_management_current_device_id_clipboard_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Device copies only the already loaded current Matrix Device ID from the existing GetOwnDevice result to the local clipboard. Missing current-device metadata stays a local prompt; the action sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management current device display name clipboard",
        base_module: "AccountSettings copy_account_management_current_device_display_name + account_management_current_device_display_name_clipboard_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Display copies only the already loaded current device display name from the existing GetOwnDevice result to the local clipboard. Missing current-device metadata or a missing device display name stays a local prompt; the action sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management current session clipboard",
        base_module: "AccountSettings copy_account_management_current_session + account_management_current_session_clipboard_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Session copies only the already loaded current-session summary from the existing GetOwnDevice result to the local clipboard. Missing current-device metadata stays a local prompt; the action sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management current device source clipboard",
        base_module: "AccountSettings copy_account_management_current_device_source_metadata + account_management_current_device_source_clipboard_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Source copies only the loaded local account/current-device summary from own_profile plus the existing GetOwnDevice text to the local clipboard. The action sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management preflight detail controls row",
        base_module: "AccountSettings account_management_preflight_controls + account_management_request_snapshot_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management preview now exposes Request, Result, Error, Retry, Source, Packet, Contract, and Taxonomy as visible account/session preflight detail controls. Request renders a local account/session request snapshot from AccountManagementPreviewState, loaded own_profile identity, and current-device GetOwnDevice text. Retry uses only a cached GetDevices failure and opens PositiveConfirmationModal before resubmitting MatrixRequest::GetDevices through the read-only directory path. Packet routes to a local session/device drilldown acceptance matrix, Contract maps that matrix to typed dedicated account portal, Browser/Portal homeserver opener, all-device, password/SSO, current-device RenameDevice result, cross-session action, device delete/trust, account/profile mutation guard, and result/error contracts, and Taxonomy records blocked dedicated-portal/password/SSO/revoke/trust/delete result slots while Result, Error, Source, Packet, Contract, and Taxonomy only update AccountManagementPreviewState, local labels, and popup copy from the same loaded metadata; it requests no extra GetOwnDevice, opens no dedicated account portal route, performs no session-management lookup, password change, SSO start, automatic retry, session revoke, extra current-device RenameDevice, cross-session device delete/trust change, Matrix account/profile mutation beyond current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation while account_management remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management session device drilldown packet",
        base_module: "AccountSettings account_preview_preflight_packet_button + account_management_session_device_drilldown_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Packet persists a local session/device drilldown acceptance matrix from AccountManagementPreviewState plus already loaded own_profile and GetOwnDevice text. It records loaded own_profile identity, current GetOwnDevice session/device metadata, verification state, device id/display/session/source clipboard payloads, Refresh/GetOwnDevice request/result/error/retry/source slots, current-device RenameDevice request/result/error/retry/source slots, dedicated account portal route targets, Browser/Portal homeserver opener outcome, all-device directory scope, password/SSO scope, cross-session revoke/trust scope, device delete/trust scope, account/profile mutation guard, and live-mutation boundary while account_management remains a base gap. Packet starts no extra GetOwnDevice, dedicated portal route open, extra homeserver opener, all-device list fetch, session-management lookup, password/SSO change, automatic retry, session revoke, extra current-device RenameDevice, cross-session device delete/trust mutation, Matrix account/profile mutation beyond current-device rename, message mutation, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management session device typed contract packet",
        base_module: "AccountSettings account_preview_preflight_contract_button + account_management_session_device_typed_contract_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Contract maps the local session/device drilldown Packet to typed dedicated account portal route, Browser/Portal homeserver opener outcome, all-device directory, password/SSO, current-device RenameDevice, cross-session revoke/trust, device delete/trust, account/profile mutation guard, GetOwnDevice refresh, result/error/retry/source, source-hash, idempotency, stale-session, and promotion-blocker contracts from AccountManagementPreviewState plus already loaded own_profile and GetOwnDevice text. Contract starts no extra GetOwnDevice, dedicated portal route open, extra homeserver opener, all-device list fetch, session-management lookup, password/SSO change, automatic retry, session revoke, extra current-device RenameDevice, cross-session device delete/trust mutation, Matrix account/profile mutation beyond current-device rename, message mutation, room-state, membership, gateway/runtime/auth, or live mutation while account_management remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account management session device result taxonomy packet",
        base_module: "AccountSettings account_preview_preflight_taxonomy_button + account_management_session_device_result_taxonomy_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Account management Taxonomy records only the live GetOwnDevice, GetDevices, SetDisplayName, current-device RenameDevice, and confirmed Browser/Portal homeserver opener result references, while dedicated account portal routes, password/SSO actions, cross-session revoke/trust, device delete/trust, and account/profile mutations beyond display-name/current-device rename stay blocked. It lists operation_id not_assigned, request slots not_built, applied/permission_denied/failed/stale/cancelled result states not_wired, stale-session and directory source-hash requirements, confirmation-gated retry policy, local-dismiss cancel policy, and audit redaction for passwords, tokens, SSO codes, refresh tokens, raw last-seen IP, and device secrets. Taxonomy starts no extra GetOwnDevice, dedicated portal route open, password/SSO flow, session revoke, cross-session trust, device delete/trust mutation, Matrix account/profile mutation beyond existing live paths, message mutation, room-state, membership, gateway/runtime/auth, or live mutation while account_management remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "account logout confirmation",
        base_module: "AccountSettings + LogoutConfirmModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Logout opens a confirmation modal before the existing Matrix Logout path is requested; Matrix Logout is requested only from the confirmed LogoutConfirmModal handler, and open, Cancel, dismiss, reset, progress, or final-result repaint sends no extra logout, account/profile, message, room-state, membership, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "Telegram visual chrome",
        base_module: "RoomsList + RoomScreen + RoomInputBar skin",
        status: HeptaTelegramBaseStatus::ReskinNeeded,
        notes: "apply Telegram layout and density over existing widgets instead of replacing them with a static fixture",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "desktop shell",
        base_module: "MainDesktopUI + RoomsSideBar",
        status: HeptaTelegramBaseStatus::ReskinNeeded,
        notes: "Telegram two-pane desktop chrome should hide dock tabs and keep room switching in the real dialog list",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mobile chat navigation",
        base_module: "HomeScreen StackNavigation + RoomsSideBar",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "mobile already supports list-to-chat stack pushes and back navigation on top of real room screens",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "chat search field",
        base_module: "RoomFilterInputBar + RoomDisplayFilter",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "existing room filter supports room name, room id, alias, and tag prefixes for Telegram-like dialog filtering",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "desktop dialog filter placement",
        base_module: "RoomsSideBar + MainFilterAction",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "desktop search lives in the real left dialog rail and feeds MainFilterAction into RoomsList",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "dialog state filters",
        base_module: "RoomDisplayFilterBuilder",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "dialog filter supports is:direct, is:unread, is:mention, is:favorite, and is:low_priority on real room fields",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "dialog filter presets",
        base_module: "RoomsSideBar filter tabs + MainFilterAction",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "All, Unread, Direct, and Fav shortcuts write the real filter input and emit MainFilterAction",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "dialog list empty/filter local state",
        base_module: "RoomsList + RoomDisplayFilter + RoomListService",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomsList loading, empty-list, filter-empty, and space-filter empty states render existing RoomListService/SlidingSync updates, cached room state, RoomDisplayFilter matches, and cached SpaceService children; they do not send Matrix search queries, messages, JoinRoom, LeaveRoom, LeaveSpace, membership mutation, or room-state mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "rooms list membership edge local state",
        base_module: "SlidingSync + RoomsList",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "SlidingSync removes Banned rooms, skips Knocked and Left rooms locally, and RoomsList renders that local membership edge evidence; re-knock and cancel-prior-knock UI remain unwired and the list rendering sends no Matrix JoinRoom, LeaveRoom, Knock, message, room-state, or membership mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room header actions",
        base_module: "RoomScreen header + local search strip + room actions strip + room info strip + room settings strip + notifications strip",
        status: HeptaTelegramBaseStatus::ReskinNeeded,
        notes: "Search opens a local-only search surface with input, loaded-timeline result counts, and Prev/Next local jumps; Info opens a read-only room info strip from loaded RoomScreen/RoomsList state; Settings opens a partial-live room settings strip for name/id, loaded alias/avatar/tombstone identity, topic availability, permissions, cached members, confirmed Name/Topic writes through MatrixRequest::SetRoomName/SetRoomTopic, confirmed canonical alias writes through MatrixRequest::SetRoomCanonicalAlias, confirmed avatar upload through MatrixRequest::UploadRoomAvatar, confirmed avatar removal through MatrixRequest::RemoveRoomAvatar, confirmed History/Join rule preset writes through MatrixRequest::SetRoomHistoryVisibility/SetRoomJoinRule, and confirmed Tombstone replacement writes through MatrixRequest::SetRoomTombstone; Mute opens a notifications strip that reads current mode and writes All/Mentions/Mute only after confirmation; Close/Escape on Search, Info, Room actions, Settings, and Notifications emits local unsent evidence without Matrix search or extra room-state/notification writes; the Room menu opens a local action strip whose Link action reuses MatrixRequest::GenerateMatrixLink, Invite/Leave open existing base modals, Favorite/Low/Unread reuse MatrixRequest::SetIsFavorite, SetIsLowPriority, and SetUnreadFlag after confirmation when room state is available, and notification mode writes reuse MatrixRequest::SetRoomNotificationMode after confirmation; editable room settings beyond Name/Topic/Alias/avatar/history/join-rule/tombstone, timed mute, and full notification preferences remain gap affordances",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room actions close local evidence",
        base_module: "RoomScreen telegram_room_actions_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The room actions strip exposes a visible Close evidence row; Close only dismisses the local action preview and sends no Matrix search, room-state, notification, message, or membership request while Link/Invite/Leave and state toggles continue to use their existing guarded base paths",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search",
        base_module: "RoomScreen local search strip + SearchMessages sidebar button",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "message search now combines loaded-timeline local search with partial-live Matrix server search: Server query submits MatrixRequest::SearchMessagesServer for the first room-scoped /_matrix/client/v3/search page, From submits the same live search with RoomEventFilter::senders, Media submits the same live search with RoomEventFilter::url_filter=EventsWithUrl for URL-backed media messages, Older submits the returned next_batch cursor with the last sender/media filter, failed Retry resubmits the current query from the first page with the last sender/media filter, Context can jump/paginate to the first cached current-room server hit through BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline, Source can open cached raw event JSON from the last server result, Filter/Date/Pins rescan loaded timeline matches through all-loaded/latest-loaded-day/pinned-loaded local scopes using existing timeline timestamps and SubscribeToPinnedEvents ids, and Taxonomy records remote date/pins/scope/full-result result slots locally. Standalone source refetch, cross-room event context, remote date index, pinned-event fetch integration, search scope fetch, and richer remote result rendering remain backend-contract blocked",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search empty/close local evidence",
        base_module: "RoomScreen telegram_message_search_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "the real search strip has a visible evidence row for loaded-timeline-only search; empty results, Close, and Escape send no live SearchMessagesServer request while Server, Older, and failed Retry remain the explicit partial-live Matrix search controls",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search loaded timeline boundary evidence",
        base_module: "RoomScreen telegram_message_search_strip + SearchMessagesButton",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search input, loaded timeline result count, active-match plaintext preview snippet, Prev/Next jumps, empty state, Close, Escape, and sidebar Messages button only scan timeline items already present in RoomScreen tl_state via plaintext_body_of_timeline_item. They submit no live SearchMessagesServer request, event context fetch, timeline pagination, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while Server, Older, and failed Retry remain the explicit partial-live Matrix search controls",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search sidebar open handoff",
        base_module: "SearchMessagesButton + RoomScreen telegram_message_search_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The sidebar Messages button emits SearchMessagesAction::LocalPreviewOpened, and the active RoomScreen handles that action by opening the existing telegram_message_search_strip for the selected room. The handoff only reveals the loaded-timeline search UI and sends no Matrix-backed search, server-side history query, event context fetch, timeline pagination/reload, room preview fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search loaded metadata summary",
        base_module: "RoomScreen telegram_message_search_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search metadata summary is derived only from loaded RoomScreen tl_state items and local search state: query length, loaded timeline item count, match count, active match ordinal, active loaded index, and active loaded event-id availability. It submits no Matrix-backed search, server-side history query, event context fetch, timeline pagination or reload, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search active result detail",
        base_module: "RoomScreen telegram_message_search_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search active-result detail is derived only from the currently loaded timeline match in RoomScreen tl_state: active ordinal, loaded item index, loaded event-id availability, query character count, local occurrence count inside plaintext_body_of_timeline_item, and a compact loaded snippet. It submits no Matrix-backed search, server-side history query, event context fetch, timeline pagination or reload, event source open, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search result action controls row",
        base_module: "RoomScreen telegram_message_search_strip search_result_action_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search strip exposes Jump, Copy, Source, Thread, and Sender as visible result-action controls and real result-action handoffs. Jump scrolls/highlights the active loaded match locally. Copy writes the current loaded match plaintext to the local clipboard. Source opens the existing local EventSourceModal from the active loaded timeline row's room id, event id, and loaded latest_json data when available; if latest_json is missing but the last Matrix /search response returned raw event JSON for a current-room server hit, Source opens that cached server-result source; if only the current-room event id is known, Source submits source-only MatrixRequest::FetchEventSource through Room::load_or_fetch_event before opening the same EventSourceModal. Thread opens the existing thread-focused timeline path only when the active loaded row already carries a thread root id. Sender opens the existing UserProfileSlidingPane from the active loaded timeline row's sender id, loaded sender_profile data, and local room_members cache when available; if member details are missing, the pane may reuse its existing GetUserProfile/profile-member read path. The controls submit no new Matrix-backed search, extra event context fetch, MatrixRequest::PaginateTimeline outside the existing context action, timeline reload, message send/edit/redact, room-state, membership mutation, profile mutation, account/profile mutation, gateway/runtime/auth, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search result jump loaded match",
        base_module: "RoomScreen jump_telegram_message_search_active_match + loaded_message_search_result_jump_loaded_match_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search Jump action is a real loaded scroll/highlight handoff for the active loaded timeline match. It refreshes local loaded matches from RoomScreen tl_state, uses the already computed telegram_message_search_matches index, smooth-scrolls the PortalList to the active loaded index, and stages the existing message highlight animation. Its metadata is derived from query, loaded item count, local match count, active ordinal, loaded index, event-id availability, and compact plaintext snippet. It submits no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, thread timeline open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search result thread open",
        base_module: "RoomScreen open_telegram_message_search_active_thread + loaded_message_search_result_thread_open_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search Thread action is a real loaded thread timeline handoff that opens only a loaded thread root from the active loaded timeline match. It derives the root from MsgLikeContent.thread_root or the loaded thread-summary root event id, then dispatches RoomsListAction::Selected(SelectedRoom::Thread) for the current room. If the thread timeline is not already loaded, it reuses the same existing CreateThreadTimeline read/open path as a normal timeline thread-summary click. It submits no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search result sender profile pane",
        base_module: "RoomScreen open_telegram_message_search_active_sender_profile + loaded_message_search_result_sender_profile_pane_label + UserProfileSlidingPane",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search Sender action is a real loaded profile-pane handoff that opens the existing UserProfileSlidingPane only from the active loaded timeline match. It builds UserProfilePaneInfo from the loaded sender id, loaded TimelineDetails::Ready(sender_profile) display name/avatar when available, current TimelineKind room id, and a matching local room_members cache row when present. If the member row is missing, it reuses the same existing user_profile_cache/GetUserProfile profile-member read path as a normal timeline avatar click. It submits no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source fetch, message send/edit/redact, profile mutation, direct-message start, room-state, membership mutation, account/profile mutation, gateway/runtime/auth, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search result copy clipboard",
        base_module: "RoomScreen stage_telegram_message_search_result_action_control + loaded_message_search_result_copy_clipboard_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search Copy action is a real loaded plaintext clipboard handoff that writes only the active loaded timeline match plaintext from plaintext_body_of_timeline_item to the local clipboard. The copied payload metadata is derived from RoomScreen tl_state and local search state: query, loaded item count, match count, active ordinal, active loaded index, event-id availability, plaintext char count, and byte count. It submits no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, event source open, thread timeline open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search result source modal",
        base_module: "RoomScreen stage_telegram_message_search_result_action_control + loaded_message_search_result_source_modal_label + EventSourceModalAction::Open",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search Source action is a real source modal handoff that opens the existing local EventSourceModal from the active loaded timeline match, from the cached Matrix /search server-result source, or from a source-only MatrixRequest::FetchEventSource fallback for a current-room hit. The loaded path uses TimelineKind room id, loaded event id when available, and latest_json from the loaded EventTimelineItem. The server-result path uses the current room id, parsed server hit event id, and raw event JSON cached from MatrixRequest::SearchMessagesServer. The fallback path submits MatrixRequest::FetchEventSource for the current TimelineKind only; SlidingSync calls Room::load_or_fetch_event and returns TimelineUpdate::EventSourceFetched for the same modal. Missing active match, missing loaded event row, missing latest_json, missing server hit event id, missing cached source, or failed source fetch leaves Source as local metadata. The source metadata is derived from RoomScreen tl_state, local search state, and the last server search response: query, loaded item count, match count, active ordinal, active loaded index, event-id availability, source origin, JSON char count, JSON line count, and source-only fetch state. It submits no new Matrix-backed search, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, thread timeline open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search query lifecycle metadata",
        base_module: "RoomScreen loaded_message_search_query_lifecycle_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search query lifecycle metadata is local to the loaded timeline helper: opening the strip reports surface visibility, query edits trim/normalize the query, reset active_match to 0, rescan already loaded RoomScreen tl_state only, and report loaded item count, match count, active index, and timeline-loaded state. Close/Escape clears query and match vectors locally. It submits no Matrix-backed search, server-side history query, event context fetch, timeline pagination or reload, room preview fetch, message send, edit, redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search server context boundary",
        base_module: "RoomScreen telegram_message_search_strip server_context_boundary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search strip now separates live server-search reads from remaining context work: Server submits MatrixRequest::SearchMessagesServer for the current room/query, Media can add RoomEventFilter::url_filter=EventsWithUrl, Older submits the returned next_batch cursor, failed Retry resubmits the current query from the first page, Context can use the first cached current-room server hit with BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline, and Source can open raw event JSON cached from the last Matrix /search response or submit source-only MatrixRequest::FetchEventSource through Room::load_or_fetch_event when only the current-room event id is known. Richer context windows, cross-room context, room preview fetch, date/pins scope adapters, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain blocked",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search server context controls row",
        base_module: "RoomScreen telegram_message_search_strip search_server_context_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search strip exposes Server, Context, Older, and Source-backed result actions. Server submits the first live MatrixRequest::SearchMessagesServer page; Media can add RoomEventFilter::url_filter=EventsWithUrl; Older submits the stored next_batch through the same Matrix /search adapter when available; Context parses the first cached current-room hit event id and reuses BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline to scroll/highlight the row; Source can open cached raw event JSON returned by Matrix /search without a second request, or submit MatrixRequest::FetchEventSource through Room::load_or_fetch_event when only the current-room event id is known. Richer context windows, cross-room context, room preview fetch, remote date/pins/scope adapters, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain blocked",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search advanced filter controls row",
        base_module: "RoomScreen telegram_message_search_strip search_advanced_filter_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search strip exposes Filter, From, Date, Media, and Pins as visible advanced filter controls. From is live for sender filtering: the sender input stays local until From or Return submits MatrixRequest::SearchMessagesServer with RoomEventFilter::senders. Media is live for URL-backed media filtering: Media submits MatrixRequest::SearchMessagesServer with RoomEventFilter::url_filter=EventsWithUrl, and Older/Retry reuse the last sender/media filter. Filter, Date, and Pins are live loaded-scope filters over already loaded RoomScreen tl_state: Filter restores all loaded message matches, Date limits local matches to the latest loaded-day timestamp window, and Pins limits local matches to event ids already received from SubscribeToPinnedEvents. They submit no remote date index query, pinned event fetch, PinEvent, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, room preview fetch, event source open, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while message_search remains partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search loaded scope filters live wiring",
        base_module: "RoomScreen MessageSearchLoadedScope + apply_telegram_message_search_loaded_scope_control",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Filter, Date, and Pins now mutate only RoomScreen's local MessageSearchLoadedScope and immediately refresh telegram_message_search_matches from already loaded timeline rows. AllLoaded restores the default local query scan, LatestLoadedDay compares loaded event timestamps against the latest loaded 24-hour floor, and PinnedLoaded intersects loaded event ids with the current SubscribeToPinnedEvents vector. This path updates visible metadata and popup copy only; it sends no Matrix SearchMessagesServer request, no remote date index query, no pinned event fetch, no PinEvent, no event-context fetch, no timeline reload, no room-state/membership/account mutation, no gateway/runtime/auth/provider call, and no Telegram delivery",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search server preflight controls row",
        base_module: "RoomScreen telegram_message_search_strip search_server_preflight_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search strip exposes Server query, Packet, Contract, Result, Error, Retry, Scope, and Taxonomy as visible server-search controls. Server query submits the first MatrixRequest::SearchMessagesServer page, Retry resubmits the current query without next_batch after an error, Older owns next_batch pagination, and Context owns cached current-room hit pagination. Packet, Contract, Result, Error, Scope, and Taxonomy remain local metadata views. It submits no remote date index query, pinned event fetch, cross-room scope search, full remote result adapter work, remote event context fetch, timeline reload outside BackwardsPaginateUntilEvent, search scope fetch, room preview fetch, event source open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search server pagination live wiring",
        base_module: "RoomScreen submit_telegram_message_search_server_next_page + submit_telegram_message_search_server_context_event + MatrixRequest::SearchMessagesServer(next_batch) + BackwardsPaginateUntilEventRequest",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Older now submits the next_batch cursor returned by the live Matrix /_matrix/client/v3/search response through the existing SearchMessagesServer adapter, and Context now submits the first cached current-room hit event id through BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline for read-only timeline context pagination. Source can also submit MatrixRequest::FetchEventSource for source-only current-room JSON when cached source is missing. Missing cursor, pending request, missing hit, invalid event id, and cross-room states stay local with explicit unavailable labels. This sends no full remote context window fetch, timeline reload, message mutation, room-state, membership, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, external mutation, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search server packet clipboard",
        base_module: "RoomScreen copy_telegram_message_search_server_packet + message_search_server_packet_clipboard_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search Packet action copies only a local server query/result packet snapshot from already loaded RoomScreen tl_state and local search state: query, loaded item count, local match count, active match, timeline availability, pinned-event count, server/context metadata, and server preflight metadata. It creates no Matrix search request body, allocates no result cursor, submits no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, retry automation, result pagination, search scope fetch, room preview fetch, event source open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search Matrix contract packet",
        base_module: "RoomScreen search_server_contract_button + message_search_matrix_contract_acceptance_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search Contract action renders only a local typed Matrix search acceptance contract from the loaded query/result packet and cached server preflight metadata. The contract names request slots for room scope, query term, keys, order, limit, filters, next_batch cursor, and event-context window; result slots for event id, sender, timestamp, snippet, highlights, context, source availability, and pagination; error slots for forbidden, rate-limited, offline, timeout, malformed query, and empty result; retry slots for confirmation, idempotency, and stale cursor; and scope/cursor promotion blockers. It builds no Matrix search request body, allocates no result cursor, submits no Matrix-backed search, server-side history query, event context fetch, MatrixRequest::PaginateTimeline, timeline reload, retry automation, result pagination, search scope fetch, room preview fetch, event source open, sender/profile lookup, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message search remote result taxonomy packet",
        base_module: "RoomScreen search_server_taxonomy_button + message_search_remote_result_taxonomy_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The message search Taxonomy action renders only a local remote date/pins/scope/full-result result taxonomy packet from the loaded query/result packet, current server/context metadata, pinned-event subscription count, and cached Matrix /search state. It names the existing live references as MatrixRequest::SearchMessagesServer first page, next_batch Older pagination, failed Retry first-page resubmit, From sender filter, Media url filter, parsed Matrix /search event_context previews, current-room BackwardsPaginateUntilEvent/MatrixRequest::PaginateTimeline Context pagination, cached/raw-or-refetched EventSourceModal Source, loaded Jump/Copy/Thread/Sender handoffs, and loaded-scope Filter/Date/Pins over existing timeline rows and SubscribeToPinnedEvents ids. It records remote_date_index_operation_id, remote_pinned_fetch_operation_id, cross_room_scope_request_id, full_result_cursor_id, full_result_page_id, sort order, room preview, non-current-room context, full result rendering, stale query, retry/cancel, and audit redaction slots as not-assigned/not-wired. It submits no extra Matrix search beyond explicit Server/Older/Retry/From/Media controls, remote date index query, remote pinned event fetch, PinEvent, cross-room scope search, room preview fetch, non-current-room event context fetch, full result adapter rendering, retry automation, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth/provider, or live mutation request while message_search remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings",
        base_module: "RoomContextMenu + RoomScreen room settings strip",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "header menu and room list context menu open partial-live room settings surfaces for room name/id, loaded canonical alias/avatar/tombstone identity, topic availability, permissions from GetRoomPowerLevels, cached members, server-backed member refresh, confirmed Name/Topic writes through MatrixRequest::SetRoomName and MatrixRequest::SetRoomTopic, confirmed canonical alias writes through MatrixRequest::SetRoomCanonicalAlias and RoomCanonicalAliasEventContent, confirmed avatar upload through MatrixRequest::UploadRoomAvatar, confirmed avatar removal through MatrixRequest::RemoveRoomAvatar, confirmed History/Join rule preset writes through MatrixRequest::SetRoomHistoryVisibility and MatrixRequest::SetRoomJoinRule, and confirmed Tombstone replacement writes through MatrixRequest::SetRoomTombstone and RoomTombstoneEventContent with TimelineUpdate::RoomSettingsMutationResult plus cached failed-state Retry, cached avatar file/MIME, and cached canonical alias alt aliases; power levels, member moderation, gateway/runtime/auth, and Telegram delivery remain TODO while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings option staging local evidence",
        base_module: "RoomScreen telegram_room_settings_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Name copies loaded room label/id to the local clipboard; Identity copies loaded RoomsList alias/avatar/tombstone metadata to the local clipboard; Perms copies the loaded GetRoomPowerLevels tl_state.user_power summary to the local clipboard; Members copies the loaded local room_members cache summary to the local clipboard; Topic shows a local summary while the separate Save name, Save topic, Save alias, Avatar edit, Remove avatar, History, Join rule, and Tombstone controls confirm before MatrixRequest::SetRoomName/SetRoomTopic/SetRoomCanonicalAlias/UploadRoomAvatar/RemoveRoomAvatar/SetRoomHistoryVisibility/SetRoomJoinRule/SetRoomTombstone. The evidence row and popup do not send power-level, membership, message, notification-rule, or unrelated room-state mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings name/id clipboard",
        base_module: "RoomScreen Name control + loaded RoomNameId",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Name copies only the already loaded current room display label and Matrix room id from RoomScreen RoomNameId to the local clipboard. Missing room id stays local-unavailable with no clipboard payload. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.power_levels, membership list write, invite, kick, ban, knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request while room_settings stays a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings permissions clipboard",
        base_module: "RoomScreen Perms control + loaded GetRoomPowerLevels tl_state.user_power",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Permissions copies only the already loaded current-user permission summary from RoomScreen tl_state.user_power to the local clipboard: send message, send reaction, and @room notification allowance. Missing power-level state stays local-unavailable with no clipboard payload. It reuses the existing GetRoomPowerLevels read result and sends no m.room.power_levels mutation, m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, membership list write, invite, kick, ban, knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request while room_settings stays a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings members clipboard",
        base_module: "RoomScreen Members control + loaded room_members local cache",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Members copies only the already loaded room_members cache summary to the local clipboard: loaded member count and compact display-name/user-id sample. Missing member cache stays local-unavailable with no clipboard payload. It reuses the existing GetRoomMembers(server-backed refresh) / SyncRoomMemberList read result and sends no membership list write, invite, kick, ban, knock, m.room.member mutation, m.room.power_levels mutation, m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request while room_settings stays a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings identity clipboard",
        base_module: "RoomScreen Identity control + RoomContextMenuDetails + RoomsList",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Identity copies only already loaded RoomsList identity metadata plus RoomScreen member-cache availability to the local clipboard: current room label/id, canonical alias presence/value, alternative alias count, avatar cache state, tombstone state, and loaded member count. Missing room-list identity metadata stays local-unavailable with no clipboard payload. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.tombstone, m.room.power_levels, membership list write, invite, kick, ban, knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request while room_settings stays a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings loaded identity preview",
        base_module: "RoomContextMenuDetails + RoomsList + RoomScreen telegram_room_settings_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings identity preview reads already loaded RoomsList metadata for canonical alias presence, alternative alias count, avatar cache state, tombstone state, and RoomScreen loaded member count. It does not fetch or mutate m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.tombstone, m.room.power_levels, membership, notification rules, account/profile, gateway/runtime/auth, or live mutation paths",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings close metadata",
        base_module: "RoomScreen telegram_room_settings_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Close summarizes current local option-staging state, loaded RoomsList identity availability, RoomScreen member count, and power-level display readiness before hiding the local strip. It sends no m.room.name, m.room.topic, m.room.power_levels, membership, invite, kick, ban, knock, room avatar, canonical alias, notification rule, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings refresh metadata",
        base_module: "RoomScreen telegram_room_settings_strip + MatrixRequest::GetRoomPowerLevels + MatrixRequest::GetRoomMembers",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Refresh reuses existing live read paths for the loaded timeline: MatrixRequest::GetRoomPowerLevels and MatrixRequest::GetRoomMembers(server-backed). It stages metadata for timeline availability, loaded RoomsList identity state, cached member count, and power-level display readiness before refreshed results arrive. It sends no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.tombstone, m.room.power_levels mutation, invite, kick, ban, knock, notification rule write, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings refresh live read wiring",
        base_module: "RoomScreen refresh_telegram_room_settings_read_paths + TimelineUpdate::UserPowerLevels + TimelineUpdate::RoomMembersListFetched",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Refresh is partial-live for reads only: when a timeline is loaded it submits MatrixRequest::GetRoomPowerLevels plus MatrixRequest::GetRoomMembers(local_only=false, JOIN), then reflects TimelineUpdate::UserPowerLevels and TimelineUpdate::RoomMembersListFetched in the settings strip. Power-level writes, membership moderation, notification-rule handoff, gateway/runtime/auth, and unrelated live mutation remain blocked behind backend room-state mutation/result contracts",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings name topic avatar tombstone live write",
        base_module: "RoomScreen Save name/topic/alias/avatar/history/join rule/tombstone + MatrixRequest::SetRoomName/SetRoomTopic/SetRoomCanonicalAlias/UploadRoomAvatar/RemoveRoomAvatar/SetRoomHistoryVisibility/SetRoomJoinRule/SetRoomTombstone + TimelineUpdate::RoomSettingsMutationResult",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Save name, Save topic, Save alias, Avatar edit, Remove avatar, History, Join rule, and Tombstone are partial-live Matrix room-state writes: RobrixTextInput drafts stay local until Save/Return opens PositiveConfirmationModal, Alias preserves loaded alternative aliases before PositiveConfirmationModal, Avatar edit validates a desktop image selection before PositiveConfirmationModal, Remove avatar requires loaded room-list avatar identity before opening PositiveConfirmationModal, History and Join rule use confirmed preset writes, Tombstone validates a replacement Matrix room id before confirmation, accepted confirmations submit MatrixRequest::SetRoomName, MatrixRequest::SetRoomTopic, MatrixRequest::SetRoomCanonicalAlias, MatrixRequest::UploadRoomAvatar, MatrixRequest::RemoveRoomAvatar, MatrixRequest::SetRoomHistoryVisibility, MatrixRequest::SetRoomJoinRule, or MatrixRequest::SetRoomTombstone for the loaded TimelineKind, SlidingSync calls Room::set_name, Room::set_room_topic, Room::send_state_event(RoomCanonicalAliasEventContent), Room::upload_avatar, Room::remove_avatar, or Room::send_state_event(RoomHistoryVisibilityEventContent/RoomJoinRulesEventContent/RoomTombstoneEventContent), and TimelineUpdate::RoomSettingsMutationResult returns success/error with a cached failed-state Retry. The Retry control reopens PositiveConfirmationModal and resubmits the cached field/value, cached canonical alias plus loaded alt aliases, or cached avatar file/MIME through the same MatrixRequest path. Power levels, membership moderation, notification-rule handoff, gateway/runtime/auth, Telegram delivery, and unrelated live mutation remain blocked while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings local boundary evidence",
        base_module: "RoomScreen telegram_room_settings_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Name, Identity, Permissions, and Members copy loaded read-only values to local clipboard while Topic and Close only update local labels and preview copy. Save name, Save topic, Save alias, Avatar edit, Remove avatar, History, Join rule, and Tombstone are confirmed room-state writes and route through MatrixRequest::SetRoomName/SetRoomTopic/SetRoomCanonicalAlias/UploadRoomAvatar/RemoveRoomAvatar/SetRoomHistoryVisibility/SetRoomJoinRule/SetRoomTombstone. They do not submit m.room.power_levels, membership, invite, kick, ban, knock, notification rule, message send/edit/redact, account/profile, gateway/runtime/auth, Telegram delivery, or unrelated live mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings edit controls boundary",
        base_module: "RoomScreen settings_edit_controls_boundary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings edit-controls boundary metadata is derived from the partial-live settings strip: room label, loaded RoomsList identity readiness, cached member count, power-level display readiness, confirmed Name/Topic/Alias write controls, confirmed avatar upload/removal controls, confirmed History/Join rule controls, and confirmed Tombstone replacement controls. Name and Topic save only after PositiveConfirmationModal, then MatrixRequest::SetRoomName/SetRoomTopic reaches Room::set_name/Room::set_room_topic and RoomSettingsMutationResult. Alias saves only after PositiveConfirmationModal, then MatrixRequest::SetRoomCanonicalAlias reaches Room::send_state_event(RoomCanonicalAliasEventContent) and RoomSettingsMutationResult while preserving loaded alt aliases. Avatar edit validates a local image and PositiveConfirmationModal accepts MatrixRequest::UploadRoomAvatar, then Room::upload_avatar returns RoomSettingsMutationResult. Remove avatar requires loaded room-list avatar identity, then PositiveConfirmationModal accepts MatrixRequest::RemoveRoomAvatar and Room::remove_avatar returns RoomSettingsMutationResult. History visibility and Join rule confirm before MatrixRequest::SetRoomHistoryVisibility/SetRoomJoinRule, then Room::send_state_event returns RoomSettingsMutationResult. Tombstone validates a replacement Matrix room id and confirms before MatrixRequest::SetRoomTombstone, then Room::send_state_event(RoomTombstoneEventContent) returns RoomSettingsMutationResult. Power levels, Member moderation, Invite/Kick/Ban/Knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, Telegram delivery, and unrelated live mutation controls remain local blocked while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings edit intent staging",
        base_module: "RoomScreen alias/avatar/history/join/tombstone live controls plus power/moderation local edit buttons",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Alias, Avatar, History, Join rule, and Tombstone open confirmed room-state writes. Alias uses MatrixRequest::SetRoomCanonicalAlias and RoomCanonicalAliasEventContent while preserving loaded alt aliases; Avatar opens the confirmed room-avatar upload path; History and Join rule open confirmed room-state preset writes; Tombstone validates a replacement room id and opens the confirmed m.room.tombstone path. Power and Moderation buttons stage local edit intent metadata only, using loaded room identity, cached member count, and power-level display readiness in the partial-live settings strip. Name and Topic use the separate confirmed Save path through MatrixRequest::SetRoomName/SetRoomTopic. These remaining intent buttons submit no m.room.power_levels, member moderation, invite, kick, ban, knock, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, Telegram delivery, or unrelated live mutation request while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings field edit intent controls",
        base_module: "RoomScreen settings_field_edit_intents local buttons",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Name edit, Topic edit, Avatar edit, Remove avatar, Perms edit, and Members edit buttons use loaded room identity, cached member count, and power-level display readiness in the settings strip. Name and Topic field intents stage drafts for the separate confirmed Save path through MatrixRequest::SetRoomName and MatrixRequest::SetRoomTopic; Avatar edit validates a local image and confirms before MatrixRequest::UploadRoomAvatar; Remove avatar requires loaded avatar identity and confirmation before MatrixRequest::RemoveRoomAvatar; the settings options/write rows separately expose confirmed History, Join rule, and Tombstone writes. Perms and Members submit no m.room.power_levels, membership list writes, invite, kick, ban, knock, canonical alias, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, Telegram delivery, or unrelated live mutation request while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings refresh result detail controls",
        base_module: "RoomScreen settings_refresh_result_controls + settings_refresh_result_detail",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Result, Members, Power, Failure, and Source buttons stage local refresh result detail metadata only, using timeline availability, loaded room identity, cached member count, current power-level display state, and local status text. Refresh remains the only control that reuses MatrixRequest::GetRoomPowerLevels and MatrixRequest::GetRoomMembers(server-backed). The detail buttons submit no extra reads and no extra room-state write outside the confirmed settings write paths; they send no m.room.power_levels mutation, membership list writes, invite, kick, ban, knock, canonical alias, tombstone, notification-rule handoff, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings mutation preflight detail controls",
        base_module: "RoomScreen settings_mutation_preflight_controls + settings_mutation_preflight_detail + room_settings_mutation_request_packet_snapshot_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Request, Packet, Contract, Taxonomy, Result, Error, Retry, and Source remain visible mutation-preflight controls. Request renders a local room-state mutation packet snapshot using timeline availability, loaded room identity, cached member count, current power-level display state, and the last local edit-intent or refresh status. Packet copies field acceptance locally, Contract copies typed room-state mutation/result contracts, Taxonomy copies power/member permission-denial and result taxonomy slots, and Result, Error, Retry, and Source stage local room-state mutation preflight metadata only from the same loaded state. They submit no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership list writes, invite, kick, ban, knock, tombstone, notification-rule handoff, retry automation, room-state mutation contract call, message send/edit/redact, account/profile, gateway/runtime/auth, or live mutation request while room_settings stays a base gap for editable room state",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings field mutation packet drilldown",
        base_module: "RoomScreen copy_telegram_room_settings_field_mutation_packet + room_settings_field_mutation_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Packet copies a local field-by-field mutation packet from current timeline availability, loaded room identity readiness, cached member count, current power-level display state, and local settings/preflight status. The packet lists confirmation, request, result, error, retry, and source acceptance slots for m.room.name, m.room.topic, m.room.avatar, current history visibility/join-rule/tombstone preset writes, canonical aliases, power levels, member moderation, and notification handoff. It submits no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership write, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, gateway/runtime/auth, or live mutation while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings field mutation contract packet",
        base_module: "RoomScreen copy_telegram_room_settings_field_mutation_contract_packet + room_settings_field_mutation_contract_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Contract copies a local typed room-state mutation/result contract packet from current timeline availability, loaded room identity readiness, cached member count, current power-level display state, local settings/preflight status, and the existing field mutation packet boundary. The packet maps baseline identity plus m.room.name, m.room.topic, m.room.avatar, current history visibility/join-rule/tombstone preset writes, canonical aliases, power levels, member moderation, and notification handoff to typed request/result/error/retry/source contracts. It submits no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership write, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, gateway/runtime/auth, or live mutation while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "room settings power member result taxonomy packet",
        base_module: "RoomScreen copy_telegram_room_settings_power_member_result_taxonomy_packet + room_settings_power_member_result_taxonomy_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Room settings Taxonomy copies a local power/member permission-denial and result taxonomy packet from current timeline availability, loaded room identity readiness, cached member count, current power-level display state, local settings/preflight status, and the existing field mutation contract boundary. It names existing live result references for confirmed Name/Topic/Alias/avatar/history/join-rule/tombstone writes plus Refresh GetRoomPowerLevels/GetRoomMembers reads, then records power_levels_operation_id not_assigned, power_levels_result permission_denied/forbidden/stale_baseline/invalid_delta/failed not_wired, member_moderation_operation_id not_assigned, invite/kick/ban/knock result mapping, retry/source-hash/stale-room policy, cancel policy, rollback/audit slots, and redaction requirements. It submits no m.room.power_levels write, m.room.member mutation, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, gateway/runtime/auth, Telegram delivery, or live mutation while room_settings stays partial-live",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications",
        base_module: "RoomContextMenu + RoomScreen notifications strip",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "header Mute and room actions Notifications read current-room notification mode through MatrixRequest::GetRoomNotificationMode and reflect already loaded RoomsList unread/mention/manual-unread state; room list context menu Notifications can choose modes by room id; All messages, Mentions, and Mute write Matrix notification rules only after PositiveConfirmationModal through MatrixRequest::SetRoomNotificationMode; failed mode writes can Retry only after another confirmation; Keyword rules, Keyword list, and Keywords read enabled custom keyword rules through MatrixRequest::GetNotificationKeywordRules and NotificationSettings::enabled_keywords; Add keyword and Remove keyword write only after PositiveConfirmationModal through MatrixRequest::SetNotificationKeywordRule and NotificationSettings::add_keyword/remove_keyword, with failed keyword writes retried only after another confirmation; Global/Defaults read MatrixRequest::GetDefaultRoomNotificationMode and Default All/Mentions/Mute writes only after PositiveConfirmationModal through MatrixRequest::SetDefaultRoomNotificationMode and NotificationSettings::set_default_room_notification_mode. Timed mute, raw global notification preference writes beyond SDK keyword/default APIs, push gateway/device configuration, pusher mutation, sound/badge tuning, and broader room-list notification indication remain TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications option staging local evidence",
        base_module: "RoomScreen telegram_notifications_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Header Mute and room actions Notifications read the effective notification mode through MatrixRequest::GetRoomNotificationMode, then All messages, Mentions, and Mute open confirmation before MatrixRequest::SetRoomNotificationMode; Refresh and Close remain local/read-only UI actions, timed mute remains unwired, and none of these paths send message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications mode write confirmation",
        base_module: "RoomContextMenu + RoomScreen telegram_notifications_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "All messages, Mentions, and Mute room notification modes use PositiveConfirmationModal before MatrixRequest::SetRoomNotificationMode calls NotificationSettings::set_room_notification_mode. The worker returns TimelineUpdate::RoomNotificationModeSet so RoomScreen updates displayed mode only after success; context menu writes use OwnedRoomId and keep timed mute as an explicit boundary",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications loaded attention preview",
        base_module: "RoomContextMenuDetails + RoomsList + RoomScreen telegram_notifications_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications previews reflect already loaded RoomsList unread count, mention count, and manual unread state in both the room context notification preview and current-room notification strip. This sends no extra notification rule read beyond MatrixRequest::GetRoomNotificationMode, timed mute, global notification preference, keyword, push gateway/device, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications mode clipboard",
        base_module: "RoomScreen Copy mode control + loaded room_notification_mode",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Copy mode writes only the already loaded current room notification mode plus loaded RoomsList unread/mention/manual-unread attention summary to the local clipboard. Missing mode stays local-unavailable with no clipboard payload. It reuses the existing GetRoomNotificationMode read result and sends no SetRoomNotificationMode, timed mute, global notification preference, keyword rule, push gateway/device, pusher, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while notifications remains a base gap beyond confirmed All/Mentions/Mute mode writes",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications mode target metadata",
        base_module: "RoomScreen telegram_notifications_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications mode target metadata is derived only from local strip state before any confirmed write: current loaded room notification mode, requested All/Mentions/Mute mode when a confirmation or failed retry is staged, loaded RoomsList attention availability, retry cache availability, timeline availability, and current local status. Opening Notifications, staging All/Mentions/Mute, failed Retry visibility, Refresh, and Close send no MatrixRequest::SetRoomNotificationMode unless PositiveConfirmationModal is accepted, and send no timed mute, global notification preference, keyword rule, push gateway/device, pusher, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications close refresh metadata",
        base_module: "RoomScreen telegram_notifications_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Close and Refresh metadata summarizes local notification status, loaded notification mode state, loaded RoomsList unread/mention/manual-unread availability, and timeline availability. Refresh only reuses MatrixRequest::GetRoomNotificationMode; Close only hides the local strip. Neither action submits MatrixRequest::SetRoomNotificationMode, timed mute, global notification preference, keyword, push gateway/device, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications local boundary evidence",
        base_module: "RoomScreen telegram_notifications_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Header Mute and room actions Notifications support confirmed All/Mentions/Mute mode writes, confirmed keyword Add/Remove writes, loaded unread/mention/manual-unread reflection, live enabled-keyword-rule reads through MatrixRequest::GetNotificationKeywordRules, live default room-mode reads through MatrixRequest::GetDefaultRoomNotificationMode, confirmed default room-mode writes through MatrixRequest::SetDefaultRoomNotificationMode and NotificationSettings::set_default_room_notification_mode, and live read-only homeserver pusher capability reads through MatrixRequest::GetNotificationPusherStatus. Timed mute, raw/global notification preference writes beyond SDK reads/keyword/default APIs, push gateway/device configuration writes, pusher mutation, broader room-list notification indication, Close, Refresh, and failure feedback remain boundary evidence while notifications remains a base gap. They do not submit message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests outside confirmed room-mode, confirmed keyword writes, and confirmed default room-mode writes",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications timed global boundary metadata",
        base_module: "RoomScreen telegram_notifications_strip",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications strip shows timed/global boundary metadata from the loaded current room notification mode, loaded RoomsList unread/mention/manual-unread attention state, current keyword-list read/write status, default room-mode read/write status, pusher status read status, and current local status. Timed mute durations, raw global notification preference writes beyond live default-mode SDK APIs and SDK keyword rules, push gateway/device configuration writes, pusher set/delete mutations, broader room-list notification indication, cancel queue, and unrelated live mutation remain unwired while confirmed All/Mentions/Mute plus failed-state confirmed Retry remain the MatrixRequest::SetRoomNotificationMode write paths; MatrixRequest::GetNotificationKeywordRules reads enabled keywords, MatrixRequest::SetNotificationKeywordRule writes Add/Remove keyword rules after confirmation through NotificationSettings::add_keyword/remove_keyword, MatrixRequest::GetDefaultRoomNotificationMode reads NotificationSettings::get_default_room_notification_mode, MatrixRequest::SetDefaultRoomNotificationMode writes NotificationSettings::set_default_room_notification_mode after confirmation and failed-state Retry, and MatrixRequest::GetNotificationPusherStatus remains read-only",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications pusher keyword boundary",
        base_module: "RoomScreen telegram_notifications_strip notifications_pusher_keyword_boundary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications pusher/keyword boundary metadata is derived from current loaded room notification mode, loaded RoomsList unread/mention/manual-unread attention state, retry cache readiness, keyword-list read/write status, default room-mode read/write status, pusher status read status, and local status. Keyword rules and Keyword list can submit MatrixRequest::GetNotificationKeywordRules to read NotificationSettings::contains_keyword_rules and enabled_keywords. Add keyword and Remove keyword submit MatrixRequest::SetNotificationKeywordRule only after PositiveConfirmationModal, then SlidingSync calls NotificationSettings::add_keyword/remove_keyword and returns TimelineUpdate::NotificationKeywordRulesMutated with confirmed failed-state Retry. Global and Defaults can submit MatrixRequest::GetDefaultRoomNotificationMode to read NotificationSettings::get_default_room_notification_mode for the loaded room class and return TimelineUpdate::NotificationDefaultRoomModeFetched; Default All/Mentions/Mute submit MatrixRequest::SetDefaultRoomNotificationMode only after PositiveConfirmationModal, then SlidingSync calls NotificationSettings::set_default_room_notification_mode and returns TimelineUpdate::NotificationDefaultRoomModeMutated with confirmed failed-state Retry. Device push and Pushers can submit MatrixRequest::GetNotificationPusherStatus to read Client::can_homeserver_push_encrypted_event_to_device. Raw global preference writes beyond SDK default APIs, Timed mute duration presets, Push gateway/device setup writes, Pusher enable/disable mutations, Sound/badge tuning, and broader room-list notification indication stay local blocked controls. It sends no Matrix notification rule account-data edits outside SDK notification settings/default APIs, pusher mutations, push gateway/device configuration write, timed mute writes, raw default preference writes, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or unrelated live mutation while notifications remains a base gap beyond confirmed All/Mentions/Mute SetRoomNotificationMode, confirmed keyword Add/Remove, keyword-list read, default-mode read/write, and pusher-status read",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications keyword list live read",
        base_module: "RoomScreen notification keyword controls + MatrixRequest::GetNotificationKeywordRules",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Keyword rules, Keyword list, and Keywords controls submit MatrixRequest::GetNotificationKeywordRules for the loaded timeline. SlidingSync reads NotificationSettings::contains_keyword_rules() and NotificationSettings::enabled_keywords(), sorts the enabled custom keyword patterns, and returns TimelineUpdate::NotificationKeywordRulesFetched so RoomScreen updates the strip and popup with the live enabled keyword count/list or empty state. This read path sends no unconfirmed keyword add/remove write, account-data mutation outside SDK notification settings, pusher mutation, push gateway/device configuration, timed mute write, global preference write, sound/badge tuning, SetRoomNotificationMode outside the existing confirmed room-mode path, message mutation, room-state, membership, account/profile, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications keyword mutation live write",
        base_module: "RoomScreen notification keyword input + MatrixRequest::SetNotificationKeywordRule",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Add keyword and Remove keyword controls keep the keyword draft local until PositiveConfirmationModal is accepted. Accept submits MatrixRequest::SetNotificationKeywordRule for the loaded timeline, SlidingSync calls NotificationSettings::add_keyword or NotificationSettings::remove_keyword, and TimelineUpdate::NotificationKeywordRulesMutated updates the notification strip. Success refreshes MatrixRequest::GetNotificationKeywordRules; failure caches keyword plus Add/Remove operation and Retry reopens PositiveConfirmationModal before resubmitting. This sends no unconfirmed keyword write, no raw Matrix notification account-data edit outside SDK notification settings, no pusher mutation, no push gateway/device configuration write, no timed mute write, no global preference write, no sound/badge tuning, no SetRoomNotificationMode outside the existing confirmed room-mode path, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or unrelated live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications pusher status live read",
        base_module: "RoomScreen notification pusher controls + MatrixRequest::GetNotificationPusherStatus",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Device push and Pushers controls submit MatrixRequest::GetNotificationPusherStatus for the loaded timeline. SlidingSync calls Client::can_homeserver_push_encrypted_event_to_device() and returns TimelineUpdate::NotificationPusherStatusFetched so RoomScreen updates the strip and popup with supported, unsupported, or error status. This is read-only and sends no pusher set/delete mutation, no push gateway/device configuration write, no account-data mutation, no push-rule write, no keyword add/remove write, no timed mute write, no global preference write, no sound/badge tuning, no SetRoomNotificationMode outside the existing confirmed room-mode path, no message mutation, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications advanced controls row",
        base_module: "RoomScreen notification_advanced_controls buttons",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications strip exposes Timed, Keywords, Pusher, and Global buttons plus keyword input row and default mode write rows. Keywords submits MatrixRequest::GetNotificationKeywordRules as a live SDK read; Global submits MatrixRequest::GetDefaultRoomNotificationMode as a live SDK default-mode read; Add keyword and Remove keyword confirm before MatrixRequest::SetNotificationKeywordRule writes through NotificationSettings::add_keyword/remove_keyword; Default All/Mentions/Mute confirm before MatrixRequest::SetDefaultRoomNotificationMode writes through NotificationSettings::set_default_room_notification_mode; Timed and Pusher setup only update local notification status, boundary labels, and popup copy from current loaded room notification mode plus loaded RoomsList unread/mention/manual-unread attention state. It sends no raw Matrix notification rule account-data edits, pusher mutations, push gateway/device configuration, timed mute writes, unconfirmed default preference writes, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or unrelated live mutation while confirmed All/Mentions/Mute SetRoomNotificationMode, confirmed default mode writes, and confirmed keyword Add/Remove remain the notification write paths",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications advanced detail controls row",
        base_module: "RoomScreen notification_advanced_detail_controls buttons",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications strip exposes Quiet hours, Keyword list, Device push, Defaults, Sound badge, and Default All/Mentions/Mute controls. Keyword list submits MatrixRequest::GetNotificationKeywordRules as a live read-only SDK handoff; Device push submits MatrixRequest::GetNotificationPusherStatus as a live read-only SDK handoff; Defaults submits MatrixRequest::GetDefaultRoomNotificationMode as a live read-only SDK handoff; Default All/Mentions/Mute submits MatrixRequest::SetDefaultRoomNotificationMode only after PositiveConfirmationModal. Quiet hours and Sound badge only update local advanced notification detail metadata, boundary labels, and popup copy from current loaded room notification mode, loaded RoomsList unread/mention/manual-unread attention state, and retry cache readiness. It sends no Matrix notification rule account-data edits, push-rule writes beyond the separate confirmed keyword/default rows, pusher mutations, push gateway/device configuration writes, timed mute writes, unconfirmed default preference writes, sound/badge tuning, retry automation, MatrixRequest::SetRoomNotificationMode outside existing confirmed All/Mentions/Mute or failed-state Retry paths, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while notifications remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications result detail controls row",
        base_module: "RoomScreen notification_result_detail_controls buttons",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications strip exposes Result, Requested, Retry cache, Failure, and Source as visible local result detail controls. Clicking any of these buttons only updates local notification result detail metadata, boundary labels, and popup copy from current loaded room notification mode, loaded RoomsList unread/mention/manual-unread attention state, retry cache readiness, timeline availability, and local status text. It sends no MatrixRequest::GetRoomNotificationMode outside the existing Refresh/read-open path, no MatrixRequest::SetRoomNotificationMode outside existing confirmed All/Mentions/Mute or failed-state Retry paths, no timed mute writes, global notification preference writes, unconfirmed keyword writes, push-rule writes beyond the confirmed keyword Add/Remove row, pusher mutations, push gateway/device configuration, sound/badge tuning, retry automation, cancel queue, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while notifications remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications timed global pusher preflight controls row",
        base_module: "RoomScreen notification_preflight_detail_controls buttons",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications strip exposes Schedule, Packet, Contract, Account data, Keywords, Pushers, and Defaults as timed/global/pusher preflight controls. Keywords submits MatrixRequest::GetNotificationKeywordRules as a live read-only SDK handoff. Pushers submits MatrixRequest::GetNotificationPusherStatus as a live read-only SDK handoff. Defaults submits MatrixRequest::GetDefaultRoomNotificationMode as a live read-only SDK handoff. Schedule renders a local notification schedule packet snapshot from current loaded room notification mode, loaded RoomsList unread/mention/manual-unread attention state, requested mode, retry cache readiness, timeline availability, and local status text; Contract copies a typed account-data/push-rule/pusher/result contract packet; Account data updates local metadata, boundary labels, and popup copy from the same loaded state. It sends no Matrix notification rule account-data write, push-rule write beyond SDK reads, keyword add/remove write, pusher mutation, push gateway/device configuration write, timed mute write, default preference write, sound/badge tuning, MatrixRequest::GetRoomNotificationMode outside the existing Refresh/read-open path, MatrixRequest::SetRoomNotificationMode outside existing confirmed All/Mentions/Mute or failed-state Retry paths, retry automation, cancel queue, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while notifications remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications rule packet drilldown",
        base_module: "RoomScreen copy_telegram_notifications_rule_packet + notifications_rule_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Packet copies a local notification rule packet from current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, requested mode, retry cache readiness, timeline availability, and local status text. The packet persists request/result/error/retry acceptance criteria for room mode, timed mute, global preferences, keyword rules, pusher/device config, defaults, and sound/badge tuning before account-data, push-rule, pusher, or notification-result contracts exist. It submits no Matrix notification rule account-data read or write, push-rule write, pusher mutation, push gateway/device configuration, timed mute write, global notification preference write, sound/badge tuning, extra GetRoomNotificationMode, unconfirmed SetRoomNotificationMode, retry automation, cancel queue, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while notifications remains a base gap beyond confirmed All/Mentions/Mute SetRoomNotificationMode",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications rule contract packet",
        base_module: "RoomScreen copy_telegram_notifications_rule_contract_packet + notifications_rule_contract_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Contract copies a typed notification account-data/pusher contract packet from current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, requested mode, retry cache readiness, timeline availability, and local status text. The contract persists typed request/result/error/retry/source slots for room mode, account-data rules, push-rule keywords, pusher/device configuration, global defaults, timed mute scheduling, sound/badge tuning, stale requested-mode retries, and result reconciliation before timed/global/keyword/pusher writes can be promoted. It submits no Matrix notification rule account-data read or write, push-rule write, pusher mutation, push gateway/device configuration, timed mute write, global notification preference write, sound/badge tuning, extra GetRoomNotificationMode, unconfirmed SetRoomNotificationMode, retry automation, cancel queue, room-state, membership, account/profile, gateway/runtime/auth, or live mutation while notifications remains a base gap beyond confirmed All/Mentions/Mute SetRoomNotificationMode",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications result taxonomy packet",
        base_module: "RoomScreen copy_telegram_notifications_result_taxonomy_packet + notifications_result_taxonomy_packet_payload",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Notifications Taxonomy copies a local timed/global/pusher result taxonomy packet from current loaded room notification mode, loaded RoomsList unread/mention/manual-unread state, requested mode, retry cache readiness, timeline availability, and local status text. The packet references the existing confirmed SetRoomNotificationMode, SetNotificationKeywordRule, and SetDefaultRoomNotificationMode result paths, then records operation_id_slot not_assigned and not-wired result slots for timed mute, raw account-data, pusher/device, and sound/badge writes plus retry, cancel, source-hash, and audit-redaction policy. It submits no Matrix notification rule account-data read or write outside SDK keyword/default APIs, pusher mutation, push gateway/device configuration, timed mute write, sound/badge tuning, extra GetRoomNotificationMode, unconfirmed SetRoomNotificationMode or SetDefaultRoomNotificationMode, retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation while notifications remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "notifications retry confirmation",
        base_module: "RoomScreen telegram_notifications_strip + MatrixRequest::SetRoomNotificationMode",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After TimelineUpdate::RoomNotificationModeSet reports a failed SetRoomNotificationMode write, RoomScreen exposes Retry only when it has cached the room id and RoomNotificationMode from the last confirmation. Retry opens PositiveConfirmationModal before resubmitting MatrixRequest::SetRoomNotificationMode; unavailable cached mode, unavailable room id, and confirmation cancel stay local. It sends no timed mute, global notification preference, keyword rule, push gateway/device, pusher, retry automation, cancel queue, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "file upload send",
        base_module: "RoomInputBar attachment affordance / media sender",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "composer exposes Photo and File send handoff through confirmation, desktop rfd picker, local selected-file preview, local pre-send review, and MatrixRequest::SendAttachment from the review Send action; full cross-platform attachment UX, Camera, Contact, thumbnail decode, and actual progress/retry/cancel queue control remain TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment option staging local evidence",
        base_module: "RoomInputBar telegram_attachment_picker",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Photo and File only open the confirmation guard before any picker or upload, while Camera, Contact, and Close stage local Telegram attachment preview; the evidence row and popup send no native picker, upload, Matrix media send, message, room-state, or membership request before confirmation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment camera/contact local boundary evidence",
        base_module: "RoomInputBar telegram_attachment_picker Camera/Contact buttons",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Camera and Contact are explicit local placeholders while file_upload_send still lacks cross-platform capture and contact-share implementation. Camera does not request camera or photo-library permission, capture media, create image/video payloads, write files, generate thumbnails, upload media, or submit SendAttachment. Contact does not request contacts permission, read an address book, create vCard/contact payloads, attach contact media, send a text fallback, or submit SendMessage. Camera, Contact, repeated selection, Close, unsupported mobile picker states, evidence rows, and popup feedback only update local preview/status copy and emit no SDK send queue work, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment mobile picker controls row",
        base_module: "RoomInputBar telegram_attachment_picker attachment_mobile_picker_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The attachment picker exposes Gallery, Camera, Files, Contact, Thumbnail, and Share as visible local mobile picker controls while file_upload_send still lacks cross-platform picker/capture/share implementation. Clicking any mobile picker control only calls the local mobile-picker boundary handler and updates status/popup copy from pending review and latest local status; it requests no camera, photo-library, files, or contacts permission, opens no mobile picker or system share sheet, captures no media, reads no contacts or shared media, generates or decodes no thumbnails, creates no image/video/vCard/share payload, uploads nothing, submits no SendAttachment, submits no SendMessage, mutates no SDK send queue work, and emits no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment mobile share sheet boundary",
        base_module: "RoomInputBar telegram_attachment_picker share_attachment_mobile_button",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The attachment mobile Share control is a visible local share-sheet boundary while file_upload_send still lacks real mobile share handoff. Clicking Share only calls the local attachment mobile-picker handler and updates status/popup copy from pending review and latest local status; it opens no system share sheet, invokes no platform share extension, reads no shared media item, creates no share payload, attaches no shared file, uploads nothing, submits no MatrixRequest::SendAttachment, submits no MatrixRequest::SendMessage, mutates no SDK send-queue work, touches no account/profile, room-state, membership, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment handoff confirmation evidence",
        base_module: "RoomInputBar telegram_attachment_picker + PositiveConfirmationModal",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Photo and File open a confirmation guard before the native desktop picker; opening and canceling the guard, picker cancel, unsupported mobile picker state, Camera, Contact, Close, evidence rows, and popup feedback send no upload, Matrix media send, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request, and selecting a file only stages local pre-send review",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment send handoff",
        base_module: "RoomInputBar + MatrixRequest::SendAttachment + Timeline::send_attachment",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After Photo/File confirmation, selecting a desktop rfd file stages local review; only the review row Send action submits MatrixRequest::SendAttachment with inferred MIME type, optional composer caption, and reply/thread event id. The async worker uses the existing matrix-sdk-ui Timeline::send_attachment().use_send_queue() path for upload/media send, while Discard, Close, picker cancel, unsupported platforms, Camera, Contact, account/profile, gateway/runtime/auth, room-state, membership, and live mutation paths remain untouched",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment pre-send review local evidence",
        base_module: "RoomInputBar telegram_pending_attachment_send + attachment_review_actions",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Selected desktop Photo/File attachments are staged in local pending review state with filename, MIME type, file extension, local file size when available, caption preview, and reply context before any Matrix media send. Only the review row Send button submits MatrixRequest::SendAttachment; Discard and Close clear pending attachment state locally and send no Matrix media upload, SendAttachment, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment selected-file preview local evidence",
        base_module: "RoomInputBar attachment_review_preview + telegram_pending_attachment_send metadata",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The selected-file preview shows filename, MIME type, extension, local size, caption preview, and reply-context status from the already selected path and composer state. It performs no upload, media decode, thumbnail generation, SendAttachment, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request; the final caption is still read from composer text only when review Send is clicked",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment selected image metadata preview",
        base_module: "RoomInputBar Photo picker + attachment_review_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The selected Photo image preview shows filename, MIME type, extension, local size, and dimensions status from the already selected local file. Lightweight PNG, JPEG, GIF, BMP, or WebP header dimensions can be displayed when available, and unavailable dimensions stay explicit. It performs no thumbnail decode, full image decode, upload, SendAttachment before review Send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment main send guard local evidence",
        base_module: "RoomInputBar send_message_button + text_input.returned guard",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "When an attachment is pending review, the main composer Send button and Enter submit path only bring the local review surface forward and tell the user to use the review-row Send. They do not send caption text as SendMessage, submit SendAttachment, clear the pending attachment, upload media, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment selection replacement preserve evidence",
        base_module: "RoomInputBar telegram_pending_attachment_send + desktop picker cancel",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Repeated Photo/File selection while an attachment is pending review replaces only local pending review state; canceling a new picker preserves the existing pending attachment. Replacement and picker cancel do not upload or send the previous file, submit a cancel for pending review, clear caption/reply context before review Send, emit SendAttachment or SendMessage, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live state",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment review lifecycle metadata preview",
        base_module: "RoomInputBar telegram_pending_attachment_send metadata popup/status",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Select, Replace, picker cancel, Close, and Discard now show local attachment review lifecycle metadata derived from telegram_pending_attachment_send: kind, filename, MIME type, local size, caption preview, reply context, validation warning, and previous pending filename when replacing. The metadata only updates popup/status/review copy; it does not open a picker beyond confirmed handoff, upload media, submit MatrixRequest::SendAttachment, send caption-only SendMessage, retry or cancel SDK send-queue work, mutate room-state or membership, touch account/profile, call gateway/runtime/auth, or perform live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment review send single-submit evidence",
        base_module: "RoomInputBar telegram_pending_attachment_send.take + send_selected_attachment_button",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Review-row Send consumes telegram_pending_attachment_send with Option::take() before MatrixRequest::SendAttachment is submitted, so duplicate clicks and empty review Send have no pending attachment to submit. Empty or duplicate review Send only updates local status/popup/operation strip and does not submit duplicate SendAttachment, send the caption as SendMessage, upload media, cancel SDK send-queue work, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live state",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment review discard close idempotent evidence",
        base_module: "RoomInputBar telegram_pending_attachment_send.take + discard_selected_attachment_button + close_attachment_picker_button",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Attachment review Discard and picker Close consume telegram_pending_attachment_send with Option::take() as local pending-review cleanup only. Repeated Discard, empty Discard, empty Close, and review-row Send after Discard/Close only update local status/popup/operation strip and do not submit SendAttachment, send the caption as SendMessage, upload media, cancel SDK send-queue work, clear composer caption/reply text, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live state",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment caption reply context boundary evidence",
        base_module: "RoomInputBar MentionableTextInput caption preview + replying_to + SendAttachment",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "While an attachment waits in pending review, caption preview live-updates from composer text and reply context stays local. Main Send/Enter, picker cancel, Discard, Close, empty Discard/Close, and empty or duplicate review-row Send preserve composer caption/reply text and do not submit caption-only SendMessage, SendAttachment, upload, SDK queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests. Review-row Send with pending attachment is the only attachment path that consumes the current composer caption into SendAttachment, carries the captured reply/thread event id, and then clears composer text plus reply preview after submit",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment file validation local error evidence",
        base_module: "RoomInputBar validate_telegram_attachment_file_for_review_send + send_selected_attachment_button",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Review-row Send revalidates the selected attachment path before MatrixRequest::SendAttachment. Unreadable paths, non-file paths, and empty files stay in local pending review with Attachment validation held locally status; composer caption/reply text is preserved and no SendAttachment, caption-only SendMessage, upload, SDK queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request is emitted. MIME fallback to application/octet-stream and size unavailable states remain visible local metadata before Send",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment validation error recovery evidence",
        base_module: "RoomInputBar validation_error + Photo/File replacement + Discard/Close",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Attachment validation errors remain recoverable local review state. After unreadable, non-file, or empty-file validation failure, the pending attachment stays visible with validation_error evidence; choosing Photo/File again replaces only local pending review and clears the validation warning, while Discard and Close clear the pending attachment plus warning locally. Retry/Cancel controls remain local evidence only and do not revalidate, resubmit SendAttachment, send a caption-only SendMessage, upload media, cancel SDK queue work, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live mutation state",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment send operation status local evidence",
        base_module: "RoomInputBar send_operation_status + MatrixRequest::SendAttachment",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After attachment review Send submits MatrixRequest::SendAttachment, the visible operation strip now receives the worker queued/failure handoff result, while timeline local echo rows show SDK queue progress/error/sent state. Worker failure Retry confirms before reusing the cached last SendAttachment handoff; Retry never auto-runs, Cancel does not abort SDK send-queue work or remove queued media, and neither emits room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment send result bridge evidence",
        base_module: "RoomInputBar send_operation_status + MatrixRequest::SendAttachment + sliding_sync popup result path",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After review Send hands the attachment to Timeline::send_attachment().use_send_queue(), the composer result bridge receives a queued-only or immediate failure result from the worker, clears cached retry on queued acceptance, states that SDK send queue owns upload/delivery, and still uses the existing popup error path for worker failure. The strip does not claim delivery success, swallow failure, auto-resubmit SendAttachment, cancel SDK send-queue work, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment queue failure recovery copy evidence",
        base_module: "RoomInputBar send_operation_status + result_bridge + Retry/Cancel copy",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After review-row Send submits SendAttachment, RoomInputBar receives queued-only or immediate failure handoff results and RoomScreen renders SDK queue progress/error/sent state from the timeline local echo. Worker failure Retry reuses the cached filename, MIME, local file path, caption, compact caption mentions, reply id, and TimelineKind only after PositiveConfirmationModal, sends no caption-only SendMessage, and never auto-runs; Cancel does not abort, remove, or cancel SDK send-queue work. Reopening Photo/File after a queued submit starts a new local review without inferring delivery success or failure for the previous queue item and emits no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment send failure retry confirmation",
        base_module: "RoomInputBar telegram_attachment_send_retry_attempt + PositiveConfirmationModal + MatrixRequest::SendAttachment",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar caches only the last validated attachment handoff after review-row Send submits MatrixRequest::SendAttachment. If the worker returns an immediate failure before SDK queue ownership, Retry opens PositiveConfirmationModal and confirmed accept resubmits the same cached TimelineKind, local file path, MIME type, caption, compact caption mentions, and reply event id through MatrixRequest::SendAttachment. Successful queued handoff clears the cache; missing cache and confirmation cancel stay local. This does not implement SDK queue retry/resume, upload abort, queue removal, delivery receipt mapping, caption-only SendMessage fallback, room-state, membership, account/profile, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment true queue control local boundary evidence",
        base_module: "RoomInputBar send_operation_status + MatrixRequest::SendAttachment + Timeline::send_attachment().use_send_queue",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The desktop attachment review-row Send handoff is real and uses MatrixRequest::SendAttachment plus Timeline::send_attachment().use_send_queue(), the worker now reports queued or immediate failure to the composer, and RoomScreen renders SDK queue progress/error/sent state from the timeline local echo. true file_upload_send queue control remains a base gap: the composer recovery strip can confirm and resubmit only an immediate worker handoff failure, but it does not retry or resume accepted SDK queue uploads, abort uploads, remove queued media, or map SDK delivery receipts back into Retry/Cancel state. Cancel emits no SDK queue abort/remove/cancel, caption-only SendMessage, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment accepted queue actions row",
        base_module: "RoomInputBar send_operation_status accepted_queue_actions",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The send operation strip now exposes Pause, Resume, Reorder, Background, and Clear as visible accepted-SDK-queue controls after the existing SendAttachment/use_send_queue handoff. Background renders a local accepted attachment queue snapshot from pending review, immediate handoff retry cache, and current local attachment status; Pause, Resume, Reorder, and Clear only call the local accepted-queue boundary handler and update status/popup copy. These actions do not retry or resume accepted SDK queue uploads, pause uploads, abort uploads, remove queued media, reorder SDK queue items, open a background queue manager, clear delivery receipts, resubmit SendAttachment, send caption-only SendMessage, mutate room-state or membership, touch account/profile, call gateway/runtime/auth, or perform live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment accepted queue timeline cancel bridge",
        base_module: "RoomInputBar send_operation_status accepted_queue_timeline_cancel_bridge + NewMessageContextMenu + MatrixRequest::AbortLocalSend + TimelineUpdate::LocalSendAbortResult",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The send operation strip exposes Status, Handle, Timeline, Cancel, and Source as visible accepted-queue timeline-cancel bridge controls after the existing SendAttachment/use_send_queue handoff. Clicking any bridge control only updates local status/popup copy from pending review, cached handoff retry state, and latest local operation status. The bridge explicitly points to the real cancel surface: pending timeline local echoes expose Cancel Send only when matrix-sdk-ui provides local_echo_send_handle, and RoomScreen then submits MatrixRequest::AbortLocalSend for that exact SendHandle. SlidingSync returns TimelineUpdate::LocalSendAbortResult so the operation strip can report canceled, already-sent/no-longer-cancellable, or failed abort outcomes. The composer bridge holds no SendHandle, aborts no upload from the composer, removes no queued media, retries/resumes no accepted queue item, resubmits no SendAttachment, and performs no gateway/runtime/auth or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment local send abort result bridge",
        base_module: "MatrixRequest::AbortLocalSend + TimelineUpdate::LocalSendAbortResult + RoomInputBar handle_local_send_abort_result",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The confirmed timeline local echo Cancel Send path now reports the SDK SendHandle::abort result back through TimelineUpdate::LocalSendAbortResult. RoomInputBar reflects canceled, already-sent/no-longer-cancellable, or failed outcomes in the operation strip without holding a SendHandle in composer controls, retrying or resuming accepted SDK queue items, removing queued media, resubmitting SendAttachment, sending caption-only SendMessage, mutating room-state or membership, touching account/profile, gateway/runtime/auth, or live mutation outside the exact timeline SendHandle abort path",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment per-file status controls row",
        base_module: "RoomInputBar send_operation_status per_file_status_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The send operation strip exposes Status, Progress, Pause, Resume, Cancel, Retry, Drilldown, Contract, and Taxonomy as visible per-file controls around the existing SendAttachment/use_send_queue handoff. Clicking any control only calls the local per-file status boundary handler and updates status/popup copy from pending review, cached immediate handoff retry readiness, latest operation status, and accepted-send queue acceptance fields. Contract renders a typed SDK queue control/progress/result/error/delivery receipt/background/multi-file acceptance contract from the same local drilldown state. Taxonomy records accepted queue/progress/result slots locally before real queue controls can be promoted. It does not inspect SDK queue entries, subscribe to upload progress, pause or resume accepted uploads, abort uploads, remove queued media, retry accepted SDK queue items, map delivery receipts, resubmit SendAttachment, send caption-only SendMessage, mutate room-state or membership, touch account/profile, call gateway/runtime/auth, or perform live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment per-file queue drilldown",
        base_module: "RoomInputBar attachment_per_file_queue_drilldown_label + drilldown_attachment_file_button",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The per-file Drilldown control renders a local accepted-send queue acceptance matrix from pending review metadata, cached immediate handoff retry readiness, and the latest local operation status. The matrix names queue item identity, stable file metadata, progress slot, pause/resume/cancel eligibility, retry eligibility, timeline local-echo cancel handle, result/error slots, delivery receipt mapping, background persistence, and reorder/grouping slots as local acceptance fields only. It performs no SDK queue lookup, progress subscription, upload pause/resume/cancel, accepted queue retry, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth call, or live mutation while file_upload_send remains a base gap beyond the current single-file review handoff",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment SDK queue contract packet",
        base_module: "RoomInputBar attachment_sdk_queue_contract_packet_label + contract_attachment_file_button",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The per-file Contract control renders a local typed SDK queue contract from the per-file drilldown, pending review metadata, cached immediate handoff retry readiness, and latest local operation status. The contract names queue item/local echo identity, file metadata, upload progress bytes/percent/speed/ETA slots, pause/resume/cancel/retry/reorder/remove eligibility, SendHandle and AbortLocalSend boundary, result states, error taxonomy, delivery receipt mapping, background persistence, multi-file album grouping, idempotency, stale-handle handling, and promotion blockers before real accepted SDK queue controls can be wired. It performs no SDK queue lookup, progress subscription, upload pause/resume/cancel, accepted queue retry, queue reorder/remove, delivery receipt read, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth call, or live mutation while file_upload_send remains a base gap beyond the current single-file review handoff",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment queue progress result taxonomy packet",
        base_module: "RoomInputBar attachment_queue_progress_result_taxonomy_packet_label + taxonomy_attachment_file_button",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The per-file Taxonomy control renders a local accepted queue/progress/result taxonomy packet from pending review metadata, cached immediate handoff retry readiness, latest local operation status, and the existing timeline local-echo cancel boundary. It names the current live references as review-row MatrixRequest::SendAttachment, Timeline::send_attachment().use_send_queue(), timeline local echo progress/error/sent rendering, MatrixRequest::AbortLocalSend plus TimelineUpdate::LocalSendAbortResult, and confirmed failed-handoff Retry only. It records accepted queue operation id, queue item/local echo identity, progress subscription, delivery receipt, pause/resume, accepted queue retry, cancel ownership, reorder/remove, background persistence, stale SendHandle, and audit redaction slots as not_wired before backend accepted queue promotion. It performs no SDK queue lookup, progress subscription, upload pause/resume/cancel, accepted queue retry, queue reorder/remove, delivery receipt read, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth call, or live mutation while file_upload_send remains a base gap beyond the current single-file review handoff",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment send preflight detail controls row",
        base_module: "RoomInputBar send_operation_status send_preflight_detail_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The send operation strip exposes Request, Result, Error, Retry, and Source as visible local preflight/result detail controls around the existing review-row SendAttachment and confirmed failed-handoff Retry paths. Clicking any control only calls the local preflight-detail handler and updates status/popup copy from pending review, latest local operation status, cached immediate handoff failure text, retry cache readiness, and result-bridge/source evidence; it does not submit SendAttachment, retry accepted SDK queue items, subscribe to upload progress, inspect queue entries, abort uploads, remove queued media, cancel SDK send-queue work, send caption-only SendMessage, duplicate upload, map delivery receipts, mutate room-state or membership, touch account/profile, call gateway/runtime/auth, or perform live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment multi file queue boundary",
        base_module: "RoomInputBar send_operation_status multi_file_queue_boundary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Attachment multi-file/album queue boundary metadata is derived only from local pending review state and cached immediate handoff retry state while the existing single selected-file review SendAttachment path remains unchanged. Multiple-file selection, album grouping, per-file progress rows, background upload list, reorder/remove queued items, bulk retry, accepted SDK queue retry/resume/cancel, delivery receipt fan-in, and queue persistence across room switches stay local blocked controls. It sends no extra file picker request, additional SendAttachment, caption-only SendMessage, SDK queue abort/remove/cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while file_upload_send remains a base gap beyond the current single-file review handoff",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment timeline send state",
        base_module: "RoomScreen attachment timeline local echo send state + MatrixRequest::AbortLocalSend",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomScreen timeline rows expose SDK queue progress, error, and sent state only from the existing local echo/send queue lifecycle after MatrixRequest::SendAttachment hands off to Timeline::send_attachment().use_send_queue(). Pending local echoes may show Cancel Send only when matrix-sdk-ui provides local_echo_send_handle, and the confirmed cancel path submits MatrixRequest::AbortLocalSend for that exact SendHandle then routes TimelineUpdate::LocalSendAbortResult to the composer operation strip. Rendering timeline send state submits no SendAttachment, retries no accepted queue item, creates no caption-only or extra SendMessage fallback, performs no upload abort outside the confirmed timeline SendHandle path, and sends no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while true file_upload_send queue controls remain a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment timeline local send cancel",
        base_module: "NewMessageContextMenu + MessageAction::CancelLocalSend + MatrixRequest::AbortLocalSend",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Pending timeline local echoes expose Cancel Send only when matrix-sdk-ui provides a local_echo_send_handle. The context menu opens a confirmation guard, then MatrixRequest::AbortLocalSend calls SendHandle::abort for that exact local echo and returns TimelineUpdate::LocalSendAbortResult to RoomInputBar. Success removes the local echo through the SDK CancelledLocalEvent update and reports canceled; already-sent/no-longer-cancellable or failed aborts show bounded operation-strip copy. This does not resubmit SendAttachment, duplicate upload, cancel another row, mutate room-state, change membership, touch account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment status taxonomy local evidence",
        base_module: "RoomInputBar send_operation_status taxonomy label + attachment local status",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Attachment status labels are pinned to review-pending, review-replaced, review-preserved, validation-held, handoff-submitted, queued-only, failure-copy, retry-confirmation-open, retry-confirmed, empty-held, discarded-local, closed-local, retry-local, and cancel-local. Only handoff-submitted and retry-confirmed can emit MatrixRequest::SendAttachment; queued-only never claims delivery success, Retry never auto-runs, and all other taxonomy states are local review/status/popup evidence without caption-only SendMessage, duplicate upload, SDK queue abort/remove/cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment review row compact fit evidence",
        base_module: "RoomInputBar attachment_review_actions + send_operation_status compact_fit label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Attachment review filename, metadata, validation warning, caption/reply context, taxonomy, result bridge copy, and retry confirmation label use wrapping Fill/Fit labels, while Send, Discard, Retry, and Cancel stay explicit wrapped button affordances on desktop and narrow mobile layouts. Compact fit evidence prevents overlap and does not change behavior: only review-row Send with pending state or confirmed failed-handoff Retry can emit MatrixRequest::SendAttachment, and wrapping or overflow prevention sends no caption-only SendMessage, duplicate upload, SDK queue abort/remove/cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "attachment mobile action density evidence",
        base_module: "RoomInputBar TelegramAttachmentOptionButton + attachment_review_actions + send_operation_status action_density label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Send, Discard, Retry, Cancel, Close, Photo, File, Camera, and Contact attachment controls share TelegramAttachmentOptionButton density with 36px touch-height, 8/12 padding, wrapped action rows, and icon+label affordances on narrow mobile layouts. Action density evidence prevents hidden overflow send affordances and is visual/local only: it does not change the send boundary, emit MatrixRequest::SendAttachment outside pending review-row Send or confirmed failed-handoff Retry, send caption-only SendMessage, duplicate upload automatically, abort/remove/cancel SDK send-queue work, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live state",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention picker/send",
        base_module: "MentionableTextInput / composer mention sender",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "composer exposes a compact @mention payload helper while the active token is an unfinished @mention and a completed-token pill tray after insertion; @room, literal Matrix user ids, and cached room member display/localpart matches can attach Matrix Mentions on the existing RoomInputBar SendMessage path. A minimal local suggestion row can insert @room or up to three cached member tokens from already loaded room_members, show cached suggestion count/selected token/no-match state, use ArrowUp/ArrowDown to change the local selected candidate, and Tab/Enter inserts the selected visible suggestion before trailing space releases Enter back to the normal send path. Completed @room, literal user-id, loaded-member, and unmatched @tokens become removable local pills; clicking a pill rewrites only composer text before the existing send-time Mentions payload scan. Full mention_picker_send remains a base gap for popup search, rich popup highlight styling, server directory, disambiguation UI, remote member lookup, attachment/edit mention payloads, and membership mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention option payload local-cache evidence",
        base_module: "MentionableTextInput / RoomInputBar composer send path",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "the visible @mention helper states that @room and cached @user mentions attach Matrix Mentions on Send using already loaded RoomScreen room_members cache. The local suggestion row inserts @room or up to three cached @user tokens into the composer, shows cached suggestion count, selected token, or no-match state for the active @query, ArrowUp/ArrowDown chooses a local selected candidate, and Tab/Enter inserts the selected visible suggestion only while an unfinished @token is active. Completed mention tokens remain visible as removable local pills, and removing one only edits composer text before normal text send uses the existing composer path. The helper works without remote member lookup, full popup search, rich popup highlight styling, server directory, disambiguation UI, attachment/edit mention payload, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention cached selection preview",
        base_module: "MentionableTextInput local suggestion status",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput displays cached suggestion count, currently selected token, and no-match state for the active @query from already loaded room_members and @room power-level state only. The preview sends no remote member lookup, popup search, pill editor, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention loaded identity preview",
        base_module: "MentionableTextInput local suggestion identity preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput shows the selected cached @user suggestion's loaded member identity from RoomScreen room_members only: display name availability, Matrix user id, localpart, and avatar MXC presence. @room selection shows loaded power-level allowance. This sends no remote member lookup, profile fetch, avatar fetch, popup search, pill editor, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention local candidate rows",
        base_module: "MentionableTextInput mention_candidate_rows_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput exposes local candidate rows for the active @query from @room power-level state plus up to three cached RoomMember matches. The row preview records rank, selected state, token, display name availability, Matrix user id, localpart, avatar MXC status, and cache source from already loaded room_members only. It starts no remote member lookup, server-side member directory search, profile/avatar fetch, duplicate-name disambiguation, rich popup search, pill editor, attachment/edit mention payload, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request while mention_picker_send remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention local duplicate hints",
        base_module: "MentionableTextInput mention_duplicate_hints_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput exposes local duplicate-name hints for the active @query by counting display-name collisions only inside already loaded room_members and the cached suggestion pass. The hint reports cached candidate count, duplicate display-name group count, selected token, selected display collision count, and localpart/Matrix user-id clues while keeping rich disambiguation UI, server-side member directory search, profile/avatar fetch, hover cards, pill editor, attachment/edit mention payloads, extra SendMessage, SendAttachment, room-state, membership, account/profile, gateway/runtime/auth, retry automation, and live mutation unwired while mention_picker_send remains a base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention lifecycle metadata",
        base_module: "MentionableTextInput mention_picker_lifecycle_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput lifecycle metadata summarizes only the local active @query, cached suggestion count, selected token, @room power-level allowance, keyboard selection, Tab/Enter insertion, click insertion, and trailing-space send release state from already loaded room_members. ArrowUp/ArrowDown only changes selected_suggestion_index, Tab/Enter/click only replaces the active @token and appends trailing space, and completed mentions release Enter back to the existing RoomInputBar SendMessage path. This sends no remote member lookup, profile fetch, avatar fetch, popup search, pill editor, disambiguation UI, attachment/edit mention payload request, extra SendMessage, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention keyboard selection boundary",
        base_module: "MentionableTextInput keyboard active-token selection",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput ArrowUp/ArrowDown selection and Tab/Enter insertion are visible local active-token controls. Arrow keys only update selected_suggestion_index over cached @room or already loaded RoomMember suggestions, and Tab/Enter only replace the unfinished @token plus trailing space before returning Enter to the existing RoomInputBar SendMessage path. The keyboard path starts no remote member lookup, server-side member directory search, profile/avatar fetch, rich popup search, pill editor, attachment/edit mention payload, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request while mention_picker_send remains an explicit base gap",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention rich picker boundary metadata",
        base_module: "MentionableTextInput mention_picker_rich_popup_boundary_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput pins rich picker scope as explicit local boundary metadata: the compact cached row is not a floating popup search, rich highlighted result list, pill editor, disambiguation UI, remote member lookup, profile/avatar fetch, attachment/edit mention payload editor, membership mutation, gateway/runtime/auth, or live mutation path. It only reports active @query, cached suggestion count, selected token/no-match state, and loaded identity from RoomScreen room_members before the existing SendMessage mention payload path",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention directory disambiguation boundary",
        base_module: "MentionableTextInput mention_picker_directory_disambiguation_boundary_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput pins directory/disambiguation as boundary metadata derived from the active @query, cached suggestion count, selected token, loaded RoomScreen room_members cache size, and optional live user-directory result before the existing SendMessage add_mentions path. Directory can submit read-only MatrixRequest::SearchUserDirectory, SlidingSync calls client.search_users, UserDirectorySearchAction::Searched repaints result/error metadata, directory result promotion buttons can insert literal Matrix user-id tokens locally, and Hover can render a local hover-card snapshot from already cached directory rows or the selected loaded member/@room suggestion. duplicate display-name disambiguation UI, remote profile hover cards, avatar/profile fetch beyond directory response fields, rich highlighted popup results beyond bounded buttons, multi-select mention tray, pill editor, attachment/edit mention payload editor, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain local blocked controls with no SendAttachment, extra SendMessage, room-state, membership, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention rich directory controls row",
        base_module: "MentionableTextInput mention_rich_directory_controls_row + mention_directory_result_promotion_row + MatrixRequest::SearchUserDirectory + UserDirectorySearchAction::Searched + mention_picker_rich_mention_packet_snapshot_label + mention_picker_hover_card_snapshot_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput exposes Rich, Directory, Hover, Tray, and Pills as visible controls while the compact @mention preview is open. Rich and Pills render a local rich mention packet snapshot from active @query, cached suggestion count, selected token, @room power-level allowance, loaded room_members cache size, and the existing SendMessage/add_mentions handoff metadata. Directory submits MatrixRequest::SearchUserDirectory for a non-empty active @query, SlidingSync calls client.search_users, UserDirectorySearchAction::Searched repaints read-only result/error metadata, and the bounded directory result promotion row can insert literal Matrix user-id tokens through the same insert_mention_token path as cached suggestions. Hover renders a local hover-card snapshot from cached directory result rows or the selected cached RoomMember/@room suggestion; Tray only updates local rich/directory boundary labels and popup copy. The controls start no floating popup search, duplicate-name disambiguation, remote profile hover card fetch, avatar/profile fetch beyond directory response fields, highlighted result list beyond bounded buttons, multi-select tray, pill editor mutation, attachment/edit mention payload, SendAttachment, extra SendMessage, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention hover-card snapshot live",
        base_module: "MentionableTextInput Hover control + mention_picker_hover_card_snapshot_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput Hover is a live local hover-card snapshot action while the compact @mention preview is open. It reads only already available @mention metadata: cached Matrix user-directory result rows when Directory has returned, selected cached RoomMember identity/candidate labels from loaded room_members, or @room power-level state. The snapshot reports active query, suggestion count, selected token, loaded member cache rows, directory result count/limited flag, display-name availability, Matrix user id, avatar MXC presence, and local source labels. It submits no MatrixRequest::SearchUserDirectory, no profile/avatar fetch, no remote hover-card request, no duplicate-name disambiguation workflow, no rich popup search, no multi-select tray mutation, no pill editor mutation, no SendMessage, no SendAttachment, no room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention directory result promotion live",
        base_module: "MentionableTextInput mention_directory_result_promotion_row + handle_directory_result_promotion_click + insert_mention_token",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput promotes live Matrix user-directory rows only after UserDirectorySearchAction::Searched has populated read-only client.search_users results. Up to three visible buttons render bounded result labels; clicking one replaces the active @token with that literal Matrix user id, appends trailing space through insert_mention_token, refreshes the local completed mention pill tray, and later relies on the existing SendMessage/add_mentions or attachment-caption AttachmentConfig.mentions scan. Search completion does not auto-insert, and the promotion path emits no extra SendMessage, SendAttachment, profile/avatar fetch beyond directory response fields, duplicate-name disambiguation workflow, multi-select tray mutation, pill editor mutation, attachment/edit mention payload rewrite, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention local pill tray live",
        base_module: "MentionableTextInput mention_pill_tray + completed_mention_pills_for_text + remove_completed_mention_token",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput keeps completed mention tokens live as a local pill tray after insertion. Completed @room, literal Matrix user-id, loaded-member display/localpart matches, and unmatched local @tokens are summarized from composer text plus already loaded room_members; up to three visible pills can remove that completed token from composer text. Removal only updates TextInput state, recomputes cached suggestion/tray metadata, and changes the existing send-time Mentions payload preview. It submits no remote member lookup, server-side member directory search, profile/avatar fetch, duplicate-name disambiguation, SendMessage, SendAttachment, edit/reply mention payload rewrite, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention preflight detail controls row",
        base_module: "MentionableTextInput mention_preflight_detail_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput exposes Request, Result, Error, Retry, and Source as visible local preflight detail controls while the compact @mention preview is open. Clicking them only updates local preflight metadata and popup copy from active @query, cached suggestion count, selected token, @room power-level allowance, loaded room_members cache size, and existing SendMessage/add_mentions source. The controls start no remote member lookup, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, rich popup search, pill editor, attachment/edit mention payload, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention payload local-cache boundary evidence",
        base_module: "MentionableTextInput / RoomInputBar SendMessage boundary",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Typing an unfinished @token opens compact local payload preview and up to three cached suggestion buttons while full mention_picker_send remains a base gap; ArrowUp/ArrowDown only changes local selected_suggestion_index, Tab/Enter only replaces the active @token and appends trailing space, and completed mentions no longer intercept Enter before send. create_message_with_mentions preserves the existing markdown, /html, and /plain message creation path, then add_mentions attaches Matrix Mentions only for @room, literal Matrix user ids, and cached RoomMember display/localpart matches from RoomScreen room_members. The attachment review Send path also reuses mentions_for_text and passes compact caption mentions through MatrixRequest::SendAttachment into AttachmentConfig.mentions. It sends no remote member lookup, popup search, rich popup highlight styling, pill editor, disambiguation UI, rich attachment payload editor, edit mention payload rewrite, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention send payload metadata",
        base_module: "MentionableTextInput mention_send_payload_metadata_label + RoomInputBar SendMessage/SendAttachment status",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar records send-time mention payload metadata on the existing MatrixRequest::SendMessage status strip and the attachment review-row SendAttachment path: message format, scanned @token count, deduped Matrix mention user count, literal Matrix user-id tokens, cached RoomMember display/localpart matches, unmatched local tokens, @room flag state, and loaded room_members cache size. The label is computed locally from composer text, loaded room_members, and @room power-level allowance before add_mentions attaches Matrix Mentions once for text sends, while mentions_for_text provides AttachmentConfig.mentions for captioned media sends; it does not start remote member lookup, profile/avatar fetch, popup search, rich highlight styling, rich pill editor, disambiguation UI, rich attachment payload editors, edit mention payloads, extra SendMessage, extra SendAttachment, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation. The completed-token pill tray remains a local composer-text edit surface before send",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention send live payload wiring",
        base_module: "MentionableTextInput + RoomInputBar create_message_with_mentions/mentions_for_text + MatrixRequest::SendMessage + MatrixRequest::SendAttachment + AttachmentConfig.mentions",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The existing RoomInputBar SendMessage path is partial-live for cached mention payloads: completed @room, literal Matrix user-id tokens, and cached RoomMember display/localpart matches from already loaded room_members flow through create_message_with_mentions, then add_mentions attaches Matrix Mentions before MatrixRequest::SendMessage is submitted. Attachment review-row Send is also partial-live for caption mentions: it calls mentions_for_text from the same loaded member cache, passes the result through MatrixRequest::SendAttachment, and sliding_sync fills AttachmentConfig.mentions before Timeline::send_attachment().use_send_queue(). MentionableTextInput only inserts cached active-token suggestions before those normal user-initiated send paths. This partial-live wiring starts no remote member lookup, server-side directory search, duplicate-name disambiguation, profile/avatar fetch, rich popup search, pill editor, edit mention payload rewrite, extra SendMessage, extra SendAttachment, typing notice beyond the existing composer behavior, room-state mutation, membership mutation, account/profile mutation, gateway/runtime/auth/provider call, Telegram delivery, or live mutation outside the existing SendMessage/SendAttachment payloads",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention payload scope controls row",
        base_module: "MentionableTextInput mention_payload_scope_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput exposes Send, Attach, Edit, Reply, Source, Packet, Contract, and Taxonomy as visible local payload-scope controls while the compact @mention preview is open. Clicking them only updates local payload-scope metadata and popup copy from active @query, cached suggestion count, selected token, loaded room_members cache size, @room allowance, existing SendMessage/add_mentions source, and attachment-caption mentions_for_text sources. Send and attachment captions are the current live compact mention payload paths; Packet persists drilldown acceptance metadata, Contract maps it to typed rich-picker, directory, and richer payload contract slots, and Taxonomy records remote hover/profile/disambiguation/edit-reply result slots as local blocked metadata. Edit, Reply, rich attachment payload editors, and Source do not create edit-message mention payload rewrites, reply mention rewriting, remote member lookup, server-side member directory search beyond explicit Directory, profile/avatar fetch, remote hover-card fetch, rich popup search, pill editor, extra SendMessage, extra SendAttachment, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention payload drilldown packet",
        base_module: "MentionableTextInput mention_payload_drilldown_button + mention_picker_payload_drilldown_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput exposes Packet as a visible local payload-scope control that persists a mention payload drilldown acceptance matrix while mention_picker_send remains a base gap. The packet combines rich picker, server directory, duplicate-name disambiguation, hover-card, tray, pills, SendMessage/add_mentions, attachment-caption AttachmentConfig.mentions, rich attachment/edit/reply payload scope, and Request/Result/Error/Retry/Source preflight acceptance criteria from active @query, cached suggestion count, selected token, loaded room_members cache size, and @room allowance. It starts no remote member lookup, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, rich popup search, hover-card fetch, pill editor mutation, rich attachment/edit/reply mention payload rewrite, extra SendAttachment beyond the review-row handoff, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention payload typed contract packet",
        base_module: "MentionableTextInput mention_payload_contract_button + mention_picker_payload_typed_contract_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput exposes Contract as a visible local payload-scope control that maps the mention payload drilldown Packet to typed mention contracts while mention_picker_send remains a base gap. The contract covers rich picker request/result/error/retry/source, server directory lookup, duplicate-name disambiguation, hover-card source, tray state, pill draft, SendMessage/add_mentions handoff, attachment-caption AttachmentConfig.mentions handoff, rich attachment/edit/reply payload scopes, source-hash, stale-token handling, idempotency, and promotion blockers from active @query, cached suggestion count, selected token, loaded room_members cache size, and @room allowance. It starts no remote member lookup, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, hover-card fetch, rich popup search, pill editor mutation, rich attachment/edit/reply mention payload rewrite, extra SendAttachment beyond the review-row handoff, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "mention remote result taxonomy packet",
        base_module: "MentionableTextInput mention_payload_taxonomy_button + mention_picker_remote_result_taxonomy_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "MentionableTextInput exposes Taxonomy as a visible local payload-scope control that records remote hover/profile/disambiguation/edit-reply result slots while mention_picker_send remains a base gap. The packet names the current live references as SendMessage/add_mentions, attachment-caption AttachmentConfig.mentions, read-only MatrixRequest::SearchUserDirectory/client.search_users, UserDirectorySearchAction::Searched result/error metadata, bounded directory result promotion into literal Matrix user-id tokens, local hover-card snapshots, local completed-token pill removal, local rich packet snapshots, and local Packet/Contract copy. It records rich picker operation id, richer directory result UI, duplicate-name disambiguation result, remote hover-card/profile result, avatar/profile fetch result, rich attachment editor result, edit/reply mention payload rewrite result, multi-select tray, pill editor mutation, retry/cancel automation, source-hash reconciliation, stale-token handling, idempotency, and audit redaction as not-assigned or not-wired. It starts no remote member lookup beyond explicit Directory, profile/avatar fetch, remote hover-card request, duplicate-name disambiguation workflow, rich popup search, rich attachment/edit/reply mention payload rewrite, extra SendMessage, extra SendAttachment, typing notice, room-state, membership, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "composer send shortcut local preference evidence",
        base_module: "AppSettings + AppPreferencesAction::SendOnEnterChanged + RoomInputBar + EditingPane",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "The Send Message Keyboard Shortcut setting is local AppPreferences state: toggling it only broadcasts AppPreferencesAction::SendOnEnterChanged, updates RoomInputBar and EditingPane submit_on_enter, and sends no message, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "message send operation status local evidence",
        base_module: "RoomInputBar + MatrixRequest::SendMessage + MatrixRequest::SendAttachment",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Text, reply, thread, location, and confirmed desktop attachment sends submit through existing MatrixRequest paths; the visible operation strip, set_message_send_operation_status, and queued/progress/failure labels update local labels. The only guarded recovery submit is failed attachment handoff Retry after PositiveConfirmationModal reuses the cached SendAttachment attempt; Retry never auto-runs, Cancel emits no SDK queue cancel, and the strip sends no caption-only SendMessage, room-state, membership, gateway/runtime/auth, account/profile, or live mutation Matrix request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "composer typing notice send evidence",
        base_module: "RoomInputBar + MatrixRequest::SendTypingNotice",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Plain composer text changes submit through the existing MatrixRequest::SendTypingNotice path; reserved Hepta command previews suppress typing notices and stay local preview state, while the visible evidence strip and set_typing_notice_status only update local labels and sends no message, room-state, retry, cancel, membership, account/profile, or extra Matrix request beyond the intended typing notice",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice message send",
        base_module: "RoomInputBar voice affordance / audio sender",
        status: HeptaTelegramBaseStatus::Gap,
        notes: "composer exposes a guarded voice surface where Send can choose an existing desktop audio file, stage local pending review, and reuse MatrixRequest::SendAttachment; microphone permission, recording, audio encoding, captured payload upload, mobile picker, and true voice-message recorder UX remain TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice option staging local evidence",
        base_module: "RoomInputBar telegram_voice_message_panel",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Record, Lock, Cancel, and Close only stage a local Telegram voice preview; Send opens a confirmation before the desktop audio-file picker and then reuses attachment pending review. The evidence row and popup send no microphone permission, recording, audio encoding, captured-payload upload, message, room-state, or membership request while true recorder-based voice message send remains TODO",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice message send local blocked evidence",
        base_module: "RoomInputBar telegram_voice_message_panel",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Voice Send can choose an existing desktop audio file after confirmation and reuse the pending review plus MatrixRequest::SendAttachment handoff, while voice_message_send remains a base gap for recording. Record, Lock, Cancel, repeated Close, and reopen update only local status, labels, and popup copy with no microphone permission, captured audio payload storage, encoding, caption/text fallback, hidden SDK send-queue work before review Send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice send live attachment wiring",
        base_module: "RoomInputBar open_telegram_attachment_handoff_confirmation + MatrixRequest::SendAttachment + Timeline::send_attachment().use_send_queue()",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Voice Send is partial-live for existing desktop audio files: after PositiveConfirmationModal, the desktop picker stages a Voice attachment review, and review-row Send reuses the same MatrixRequest::SendAttachment path that hands the file to Timeline::send_attachment().use_send_queue(). The failed-handoff Retry path can resubmit the cached Voice SendAttachment only after confirmation. Microphone permission, recorder/audio-session capture, waveform sampling, codec/transcription work, captured upload, mobile picker/share-sheet handoff, accepted SDK queue controls, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, and live mutation remain blocked contract work",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice selected audio metadata preview",
        base_module: "RoomInputBar voice Send + attachment_review_preview",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "After confirmed Voice Send chooses an existing desktop audio file, the local attachment review surface shows filename, MIME type, extension, local file size, duration status, codec/container status, and bounded WAV PCM waveform peaks before any SendAttachment. Simple WAV header duration and PCM peaks are shown when available; otherwise duration, codec, or waveform stays visibly unavailable. This preview uses only the already selected local file path and sends no microphone permission request, privacy entitlement change, audio session activation, platform recorder, captured local audio file creation, temporary recording write, recorder waveform capture, media decode, player startup, opus/aac encoding, upload, hidden SDK send-queue work before review Send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice selected audio waveform codec preview",
        base_module: "RoomInputBar voice_audio_waveform_codec_label + voice_message_recorder_status_controls_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Waveform and Codec controls can summarize the already selected desktop Voice attachment with capped local WAV RIFF/fmt/data parsing: codec name, sample rate, channels, bit depth, data bytes, duration, and coarse PCM peak buckets. Non-WAV and unsupported WAV codecs remain local unavailable states. This starts no microphone permission, recorder/audio-session capture, captured file write, compressed media decode, playback, transcoding, upload, extra SendAttachment before review Send, SendMessage fallback, SDK queue mutation, account/profile, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice lifecycle metadata",
        base_module: "RoomInputBar voice_message_lifecycle_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Voice lifecycle metadata summarizes only local voice panel visibility, Record/Lock/Cancel/Close status, confirmation and picker state copy, pending selected desktop audio filename, duration status, and reply-context state. Record, Lock, Cancel, Close, reopen, confirmation cancel, picker cancel, unsupported picker, selected audio review, and repeated status repaint update only local labels/popup copy. Send opens the existing confirmation before the desktop audio picker, and selected audio still enters the existing attachment review row before MatrixRequest::SendAttachment. This sends no microphone permission request, privacy entitlement change, audio session activation, platform recorder, captured local audio file creation, temporary recording write, waveform sampling, duration capture from a recorder, media decode, player startup, opus/aac encoding, captured media upload, SendMessage text fallback, hidden SDK send-queue work before review Send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice confirmation cancel metadata",
        base_module: "RoomInputBar AttachmentHandoffCanceled + voice_confirmation_cancel_metadata_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Voice Send confirmation Cancel now emits only a local RoomInputBarAction that repaints the voice panel and attachment review preview state. If an existing pending attachment review is loaded it is preserved; otherwise the waiting picker preview is hidden. Cancel sends no desktop picker request, microphone permission request, recorder/audio-session work, local recording file creation, waveform sampling, encoder work, upload, SendAttachment, SendMessage fallback, SDK queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice permission recording local boundary evidence",
        base_module: "RoomInputBar telegram_voice_message_panel",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Voice Record, Lock, Cancel, Close, waveform, and timer are local-only preview evidence while voice_message_send remains a base gap. They submit no microphone permission, privacy entitlement, audio session activation, platform recorder, local audio file creation, temporary file write, waveform sampling, duration capture, opus/aac encoding, captured media upload, SendMessage text fallback, SDK send-queue work, room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests. Voice Send is limited to confirmed desktop audio-file selection and pending review before SendAttachment",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice recorder waveform codec boundary",
        base_module: "RoomInputBar voice_message_recorder_waveform_codec_boundary_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "Voice recorder, waveform capture/render, encoder/codec selection, opus/ogg/amr conversion, silence trimming, transcription, playback scrubber, upload progress, background recording, attachment/edit voice payload, and hidden SDK queue controls remain local blocked while voice_message_send remains a base gap. Record, Lock, Cancel, Close, confirmation cancel, and status repaint send no microphone permission prompt, privacy entitlement change, audio session activation, platform recorder, captured file write, media decode, codec/transcription service request, upload progress subscription, SendAttachment, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation request. The existing positive Voice Send path still only chooses an already selected desktop audio file, stages attachment review, and can submit MatrixRequest::SendAttachment from that review row",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice recorder status controls row",
        base_module: "RoomInputBar voice_recorder_status_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar exposes Timer, Waveform, Transcript, Progress, and Codec as visible local voice recorder status controls. Clicking them only updates local voice panel status, static meter/timer copy, recorder status metadata, and popup text from panel visibility plus pending desktop audio review state. Waveform and Codec can summarize already selected desktop WAV files with capped local RIFF/fmt/data parsing and coarse PCM peak buckets. The controls request no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, recorder waveform sampling, transcription service, codec conversion, upload progress subscription, SDK queue control, SendAttachment, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation while the confirmed positive path remains desktop audio-file review SendAttachment",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice capture lifecycle controls row",
        base_module: "RoomInputBar voice_capture_lifecycle_controls + voice_message_capture_request_packet_snapshot_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar exposes Permission, Capture, Encode, Review, Upload, Packet, Contract, and Taxonomy as visible local voice capture lifecycle controls. Permission and Upload render a local voice capture/request packet snapshot from voice panel visibility, pending desktop audio review state, retry state, local status, and source copy; Packet renders a recorder lifecycle drilldown packet with microphone/recorder acceptance criteria; Contract maps that drilldown to typed recorder/upload contracts; Taxonomy records permission/capture/upload result slots before recorder or captured-upload work can be promoted; Capture, Encode, and Review only update local voice panel status, capture lifecycle metadata, and popup text from the same local state. Permission requests no microphone permission or privacy entitlement; Capture starts no platform recorder, audio session, captured local audio file, temporary recording write, waveform sampling, or duration capture; Encode performs no codec conversion, media decode, silence trimming, or transcription; Review creates no captured voice payload or edit/attachment voice payload; Upload submits no SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, account/profile, room-state, membership, gateway/runtime/auth, or live mutation while the confirmed positive path remains desktop audio-file review SendAttachment",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice recorder lifecycle drilldown packet",
        base_module: "RoomInputBar packet_voice_capture_button + voice_message_recorder_lifecycle_drilldown_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar exposes Packet in the voice capture lifecycle row to render a recorder lifecycle drilldown packet while voice_message_send remains a base gap. Packet uses only voice panel visibility, pending desktop audio review filename and duration status, local voice status, retry-cache readiness, cached immediate handoff error text, and source/evidence copy. It persists microphone permission, privacy entitlement, audio session, recorder start/lock/cancel, temporary capture file lifecycle, waveform sampling/rendering, timer/duration capture, codec/encoding/transcription, review playback/drop cleanup, mobile picker/share sheet, upload queue, result/error/retry/source, and confirmed desktop audio review SendAttachment acceptance criteria as local metadata only. It requests no microphone permission, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice recorder typed contract packet",
        base_module: "RoomInputBar contract_voice_capture_button + voice_message_recorder_typed_contract_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar exposes Contract in the voice capture lifecycle row to map the recorder lifecycle drilldown packet to typed microphone permission, privacy entitlement, audio session, recorder session, capture file, waveform/timer, codec/encoding/transcription, review playback/drop, mobile picker/share sheet, upload queue, SendAttachment result, stale capture, idempotency, and adapter promotion blocker contracts while voice_message_send remains a base gap. Contract uses only voice panel visibility, pending desktop audio review filename and duration status, local voice status, retry-cache readiness, cached immediate handoff error text, and source/evidence copy. It requests no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice recorder result taxonomy packet",
        base_module: "RoomInputBar taxonomy_voice_capture_button + voice_message_recorder_result_taxonomy_packet_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar exposes Taxonomy in the voice capture lifecycle row to render a local recorder permission/capture/upload result taxonomy packet while voice_message_send remains a base gap. Taxonomy uses only voice panel visibility, pending desktop audio review filename and duration status, local voice status, retry-cache readiness, cached immediate handoff error text, and source/evidence copy. It names only confirmed desktop audio review MatrixRequest::SendAttachment, Timeline::send_attachment().use_send_queue(), failed-handoff Retry, selected-audio bounded WAV metadata/waveform analysis, review Play local system-opener handoff, and Drop pending-audio local cleanup as live references. Microphone permission operation id, privacy entitlement result, audio session id, recorder session id, capture file identity, waveform/timer result, codec/transcription result, review player result, mobile picker/share result, captured upload queue item, delivery result, stale capture result, retry/cancel result, and audit redaction remain not_assigned or not_wired. It requests no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, inline audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice mobile picker controls row",
        base_module: "RoomInputBar voice_mobile_picker_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar exposes Mic, Files, Library, Retake, and Share as visible local voice mobile picker controls. Clicking them only updates local voice panel status, mobile picker metadata, and popup text from panel visibility plus pending desktop audio review state. Mic requests no mobile microphone permission or privacy entitlement; Files opens no mobile document picker; Library opens no photo/audio library picker; Retake deletes no captured clip and starts no capture session; Share opens no system share sheet or external handoff. The row creates no captured voice payload, reads no mobile media, submits no SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, account/profile, room-state, membership, gateway/runtime/auth, or live mutation while the confirmed positive path remains desktop audio-file review SendAttachment",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice review playback controls row",
        base_module: "RoomInputBar voice_review_playback_controls",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar exposes Play, Pause, Scrub, Speed, and Drop as visible voice review playback controls. Play is now a narrow local OS handoff: it opens only the pending desktop Voice attachment's readable regular local file through the system opener, and stale or missing pending audio stays warning-only. Pause, Scrub, and Speed only update local voice panel status, review playback metadata, and popup text from panel visibility plus pending desktop audio review filename, duration status, and latest voice status. Drop is a real pending-audio cleanup handoff: it consumes only pending Voice review state with Option::take(), clears voice retry metadata, preserves composer caption/reply text, and leaves Photo/File pending attachments untouched. The controls start no inline audio player, media decode, waveform sampling, playback position subscription, speed transform, scrubber timeline, local file deletion, SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, account/profile, room-state, membership, gateway/runtime/auth, or live mutation while the confirmed positive network path remains desktop audio-file review SendAttachment",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice review drop pending audio action",
        base_module: "RoomInputBar drop_telegram_voice_review_audio",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar voice review Drop is a real pending-audio cleanup handoff. It consumes only a pending desktop Voice attachment review with Option::take(), clears voice failed-handoff retry metadata, preserves composer caption/reply text, and leaves Photo/File pending attachments untouched. It deletes no local file, opens no audio player, decodes no media, submits no MatrixRequest::SendAttachment, sends no caption-only SendMessage, aborts or removes no SDK send-queue work, and emits no account/profile, room-state, membership, gateway/runtime/auth, or live mutation while the confirmed positive path remains desktop audio-file review SendAttachment",
    },
    HeptaTelegramBaseCapability {
        telegram_surface: "voice send preflight detail controls row",
        base_module: "RoomInputBar voice_send_preflight_detail_controls + voice_message_capture_request_packet_snapshot_label",
        status: HeptaTelegramBaseStatus::DirectReuse,
        notes: "RoomInputBar exposes Request, Result, Error, Retry, and Source as visible local Voice SendAttachment preflight detail controls. Request renders a local voice capture/request packet snapshot from voice panel visibility, pending desktop audio review filename and duration status, latest local voice status, cached immediate attachment handoff failure text, retry-cache readiness, and source evidence from the existing attachment send bridge. Result, Error, Retry, and Source only summarize the same local state. The controls request no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, transcription service, codec conversion, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation; the only real Voice SendAttachment paths remain confirmed desktop audio picker to attachment review-row Send and failed-handoff Retry after PositiveConfirmationModal",
    },
];

pub const HEPTA_TELEGRAM_BASE_GAPS: &[&str] = &[
    "message_search",
    "message_report_send",
    "message_edit_history",
    "matrix_link_resolution",
    "room_settings",
    "notifications",
    "file_upload_send",
    "media_download_playback",
    "account_avatar_upload",
    "account_management",
    "mention_picker_send",
    "voice_message_send",
];

pub const HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY: &[HeptaTelegramBaseGapRunway] = &[
    HeptaTelegramBaseGapRunway {
        gap_id: "message_search",
        current_path: "loaded timeline search strip plus live Matrix /search first page, live From sender filter, live Media URL filter, live next_batch Older pagination, failed Retry first-page resubmit, server result/error labels, cached current-room Context pagination, visible result-action/server/context controls, real loaded result-action handoffs, local Jump loaded-match action, local Thread open action, local Sender profile-pane action, local Copy clipboard action, local Source modal action with latest_json or cached server raw JSON, source-only MatrixRequest::FetchEventSource fallback through Room::load_or_fetch_event, live loaded-scope Filter/Date/Pins over all-loaded/latest-loaded-day/pinned-loaded timeline rows, advanced filter controls, server-query packet snapshot, server query/result packet clipboard, typed Matrix search acceptance contract, remote date/pins/scope/full-result result taxonomy packet, and server preflight detail controls",
        remaining_gap: "remote event context window/rendering, remote result-action adapter, remote date index, pinned-event fetch integration beyond the existing subscription, cross-room search scope fetch, richer remote result cursor rendering, and full remote result rendering",
        next_ui_safe_step: "coordinate event-context window, remote date/pins/scope, and remote result-action contracts outside the UI lane before promoting server results beyond read-only search/next_batch/retry/source/media-url-filter/local-loaded-scope filters",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "message_report_send",
        current_path: "live confirmed Spam/Abuse/Custom ReportContent send/result wiring plus confirmed failed-state Retry, status clipboard, local moderation packet snapshot, reviewer packet, typed moderation workflow/result contract packet, workflow result taxonomy packet, local workflow actions, loaded report Source EventSourceModal handoff, and request/result/error/retry/source preflight detail",
        remaining_gap: "moderation workflow, policy lookup, report queue cancel, abuse tooling, reviewer assignment, appeal/enforcement flow, and richer server report lifecycle contract",
        next_ui_safe_step: "coordinate backend moderation workflow/result contracts before wiring moderation service actions",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "message_edit_history",
        current_path: "live paginated FetchEditHistory m.replace read/result wiring to Room::relations next_batch exhaustion plus confirmed failed-state Retry with loaded original/latest preview, relation pages/exhausted metadata, local synthetic full snapshot EventSourceModal, cached latest replacement raw JSON EventSourceModal handoff, MatrixRequest::FetchEventSource Room::load_or_fetch_event fallback for missing latest replacement source JSON, loaded original EventSourceModal fallback, loaded side-by-side preview/full-body diff modal plus compact diff clipboard handoff, loaded/full diff packet action, typed full-history result contract packet, remote full-history/source result taxonomy packet, visible full controls, and request/result/error/source preflight detail",
        remaining_gap: "remote/server-backed full history result adapter, event context, server-backed source reconciliation, and server-authored full-body diff payloads",
        next_ui_safe_step: "coordinate backend edit-history modal/result contracts before wiring remote full-history result adapter, event context, source reconciliation, or server-authored full-body diff payloads",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "matrix_link_resolution",
        current_path: "loaded alias navigation, loaded-event jump/source modal, current-room missing-event BackwardsPaginateUntilEvent/PaginateTimeline read wiring, live compact PreviewMatrixLinkTarget read/result wiring plus cached Server context refresh and confirmed failed-state Retry, confirmed cached room-or-alias MatrixRequest::JoinRoomByIdOrAlias with MatrixLinkJoinResultAction status/retry, confirmed cached room-or-alias MatrixRequest::Knock with KnockResultAction status/retry, confirmed current-room user InviteUser, Room target clipboard, Via servers clipboard, Event id clipboard, Preview metadata clipboard, visible local route-scope/context actions, local server-context packet snapshot, route drilldown packet, typed route/result contract packet, route/event-context result taxonomy packet, and unresolved detail state",
        remaining_gap: "full non-current-room event-context window and richer route/result adapter",
        next_ui_safe_step: "coordinate backend Matrix route/result contracts before wiring richer event context or route-result adapter",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "room_settings",
        current_path: "partial-live settings strip with Name/id, identity, permissions, and members clipboard, live Refresh read wiring for GetRoomPowerLevels and GetRoomMembers(server-backed), live confirmed SetRoomName/SetRoomTopic/SetRoomCanonicalAlias/UploadRoomAvatar/RemoveRoomAvatar/SetRoomHistoryVisibility/SetRoomJoinRule/SetRoomTombstone writes with result/error plus confirmed failed-state Retry resubmit, visible local field edit-intent, refresh result detail, room-state mutation request snapshot, field mutation packet drilldown, typed room-state mutation/result contract packet, power/member permission-denial result taxonomy packet, and room-state mutation preflight controls",
        remaining_gap: "editable power-level/member state and richer source/result reconciliation",
        next_ui_safe_step: "coordinate backend room-state mutation/result contracts for power levels and membership moderation before wiring those writes",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "notifications",
        current_path: "confirmed All/Mentions/Mute writes, confirmed failed-state SetRoomNotificationMode Retry, confirmed keyword Add/Remove writes through MatrixRequest::SetNotificationKeywordRule, confirmed default room-mode writes through MatrixRequest::SetDefaultRoomNotificationMode, loaded mode clipboard, live enabled keyword-list read through MatrixRequest::GetNotificationKeywordRules, live default room-mode read through MatrixRequest::GetDefaultRoomNotificationMode, live pusher/device capability read, local schedule snapshot, notification rule packet drilldown, typed account-data/pusher contract packet, timed/global/pusher result taxonomy packet, and visible advanced/detail/result/preflight controls",
        remaining_gap: "timed mute, raw/global preference writes beyond SDK keyword/default APIs, push gateway and pusher config, sound/badge tuning",
        next_ui_safe_step: "connect remaining timed/global/pusher/sound writes only after backend notification account-data, pusher, sound/badge, timed mute, and result contracts are coordinated outside the UI lane",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "file_upload_send",
        current_path: "live single-file SendAttachment/use_send_queue handoff, live confirmed failed-handoff Retry resubmit for cached SendAttachment metadata, live timeline SDK progress/error/sent local-echo state, live timeline AbortLocalSend for rows with local_echo_send_handle plus LocalSendAbortResult operation-strip bridge, local accepted queue snapshot, per-file accepted-send queue drilldown, typed SDK queue contract packet, accepted queue/progress/result taxonomy packet, visible local queue/per-file/preflight detail controls, accepted queue timeline-cancel bridge controls, and local mobile picker/thumbnail/share-sheet controls",
        remaining_gap: "accepted SDK queue retry/resume/abort/remove/reorder, upload progress subscription, delivery receipt mapping, real camera/contact capture/share, thumbnail decode/generation, and real mobile picker",
        next_ui_safe_step: "connect real accepted-queue controls only after the backend/SDK adapter exposes progress, result, delivery, and cancel contracts outside the UI lane",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "media_download_playback",
        current_path: "live FetchMedia media-cache read path, confirmed Download/Play SaveMedia path, guarded row-scoped SaveMedia Retry resubmit, live system opener outcome popup mapping, cached successful SaveMedia destination Open folder/Replay handoffs with stale cache validation/eviction, Queue cached saved-file metadata snapshot with stale cache cleanup, loaded metadata clipboard, media operation packet drilldown, typed playback/media queue contract packet, decrypt/decode/opener/queue result taxonomy packet, disabled inline-player controls, visible codec/transcode controls, local playback/download queue snapshot, and visible save/open recovery/preflight detail controls",
        remaining_gap: "true inline playback, decrypt/decode, real codec/transcode work, captions, playback progress subscription, and queue retry/resume/cancel controls",
        next_ui_safe_step: "connect true inline/decrypt/queue controls only after the backend/media adapter exposes typed playback progress, decrypt/decode result, codec fallback, and queue control contracts outside the UI lane",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "account_avatar_upload",
        current_path: "live confirmed UploadAvatar/client.account().upload_avatar wiring with SDK Account::set_avatar_url(Some(mxc)), live confirmed failed-state UploadAvatar Retry resubmit, live confirmed direct MXC MatrixRequest::SetAvatar(Some) plus failed-state Retry, live confirmed SetAvatar(None) delete wiring, confirmed local avatar file upload, image metadata, source path clipboard, bounded in-memory thumbnail/full-size pixel decode, local cropper packet snapshot, visible local editor/source/preflight detail controls, source/editor drilldown packet, typed cropper-camera contract packet, and source/cropper/camera/editor artifact result taxonomy packet",
        remaining_gap: "source identity, cropper/editor, camera/photo-library, transformed-image handoff beyond direct MXC SetAvatar(Some), persistent thumbnail artifact mapping, richer mobile UX, and editor result mapping",
        next_ui_safe_step: "coordinate backend avatar source/cropper/camera/editor/thumbnail contracts before wiring cropper/editor, camera/photo-library capture, transformed-image SetAvatar handoff result mapping, persistent thumbnail artifacts, or richer mobile UX",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "account_management",
        current_path: "live GetOwnDevice current-session read/refresh wiring, live GetDevices read-only all-device directory wiring plus confirmed failed-state GetDevices Retry resubmit, live confirmed SetDisplayName profile display-name mutation plus failed-state confirmed Save Name resubmit, live confirmed current-device Rename MatrixRequest::RenameDevice/client.rename_device mutation, live confirmed Browser/Portal active homeserver system-opener handoff, loaded own profile, visible local session/device-directory/preflight controls, local account/session request snapshot, current-device id/verified/display/session/source clipboard metadata controls, session/device drilldown packet, typed account-session contract packet, and password/SSO/revoke/trust/delete result taxonomy packet",
        remaining_gap: "dedicated account-management portal route, password/SSO, session revoke/trust, cross-session device management, device delete/trust mutation, account/profile mutations beyond display name and current-device rename",
        next_ui_safe_step: "coordinate backend dedicated account portal, password/SSO, session action, cross-session device management, device delete/trust mutation, and account/profile result contracts beyond display name/current-device rename before writes",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "mention_picker_send",
        current_path: "cached @room/@user suggestions, local candidate rows, local duplicate-name hints, keyboard selection, live completed-token pill tray removal, live SendMessage/add_mentions payload wiring, live attachment caption AttachmentConfig.mentions wiring, live read-only Matrix user-directory search metadata through MatrixRequest::SearchUserDirectory/client.search_users, live bounded directory result promotion into literal Matrix user-id tokens, local hover-card snapshots from cached directory or loaded member metadata, send payload metadata, local rich mention packet snapshot, visible local rich/directory/preflight/payload-scope controls, mention payload drilldown packet, typed mention contract packet, and remote hover/profile/disambiguation/edit-reply result taxonomy packet",
        remaining_gap: "rich popup search, remote disambiguation UI, remote hover-card/profile adapter, rich attachment payload editors, edit payload rewrites, typed rich/edit/reply payload scopes, and richer directory result UI beyond bounded user-id token promotion",
        next_ui_safe_step: "coordinate backend mention contracts plus richer UI promotion contracts before wiring rich picker, remote disambiguation, remote hover-card/profile adapters, rich attachment payload editors, or edit/reply payload rewrites",
    },
    HeptaTelegramBaseGapRunway {
        gap_id: "voice_message_send",
        current_path: "live desktop audio SendAttachment/use_send_queue wiring plus confirmed failed-handoff Retry for cached Voice SendAttachment, confirmed desktop audio-file review handoff, selected-audio duration/codec/bounded WAV PCM waveform analysis, review Play local system-opener handoff, real Drop pending-audio cleanup handoff, local recorder/waveform/codec boundary, recorder status, capture lifecycle, local voice capture/request packet snapshot, recorder lifecycle drilldown packet, typed recorder/upload contract packet, permission/capture/upload result taxonomy packet, mobile picker boundary controls, review playback/drop cleanup, and SendAttachment preflight detail controls",
        remaining_gap: "microphone permission, real recorder/audio session, recorder waveform capture, encoding/transcription, inline review player/scrubber, captured upload queue, mobile picker",
        next_ui_safe_step: "coordinate backend recorder/upload contracts before wiring microphone permission, recorder session, capture, recorder waveform, codec/transcription, inline review player, mobile picker, or captured upload work",
    },
];

pub const HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS: &[&str] = &[
    HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_ROOM_SETTINGS_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_NOTIFICATIONS_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_LOCAL_BOUNDARY_MARKER,
    HEPTA_TELEGRAM_COMPOSER_VOICE_PERMISSION_RECORDING_LOCAL_BOUNDARY_MARKER,
];

pub const HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES: &[&str] = &[
    "base_gap_product_runway_evidence",
    "message_search_header",
    "sidebar_message_search_button",
    "sidebar_message_search_open_handoff",
    "message_search_mode_strip",
    "message_search_input_results_jumps",
    "message_search_loaded_preview_evidence",
    "message_search_empty_close_local_evidence",
    "message_search_loaded_timeline_boundary_evidence",
    "message_search_loaded_metadata_summary",
    "message_search_active_result_detail",
    "message_search_result_action_controls_row",
    "message_search_result_jump_loaded_match_action",
    "message_search_result_thread_open_action",
    "message_search_result_sender_profile_pane_action",
    "message_search_result_copy_clipboard_action",
    "message_search_result_source_modal_action",
    "message_search_query_lifecycle_metadata",
    "message_search_server_context_boundary",
    "message_search_server_context_controls_row",
    "message_search_advanced_filter_controls_row",
    "message_search_loaded_scope_filters_live_wiring",
    "message_search_server_preflight_controls_row",
    "message_search_server_packet_clipboard_action",
    "message_search_matrix_contract_packet_action",
    "message_search_remote_result_taxonomy_packet_action",
    "message_search_server_pagination_live_wiring",
    "dialog_list_empty_state_local_filter_evidence",
    "rooms_list_membership_edge_local_evidence",
    "rooms_list_pagination_adapter_local_evidence",
    "rooms_list_load_more_pagination_packet_preview",
    "rooms_list_header_space_scope_local_evidence",
    "desktop_dock_restore_lazy_local_evidence",
    "mobile_stack_navigation_local_evidence",
    "navigation_spaces_toggle_local_evidence",
    "navigation_top_level_tab_selection_local_evidence",
    "profile_icon_settings_navigation_local_evidence",
    "settings_close_previous_selection_local_evidence",
    "spaces_bar_entry_selection_local_read_evidence",
    "spaces_bar_secondary_click_local_no_menu_evidence",
    "spaces_bar_empty_filter_local_evidence",
    "rooms_list_section_unread_aggregate_local_zero_evidence",
    "rooms_list_section_unread_aggregate_packet_preview",
    "rooms_list_all_rooms_loaded_local_unknown_evidence",
    "rooms_list_space_parent_cache_local_evidence",
    "rooms_list_name_update_selected_state_local_evidence",
    "rooms_list_removed_room_selected_state_local_evidence",
    "rooms_list_removed_room_rejoin_packet_preview",
    "space_unread_filter_local_zero_evidence",
    "space_unread_filter_aggregate_packet_preview",
    "timeline_pagination_read_evidence",
    "thread_summary_read_evidence",
    "thread_open_timeline_read_evidence",
    "reply_preview_event_details_read_evidence",
    "sender_profile_event_details_read_evidence",
    "unread_count_read_evidence",
    "successor_room_details_read_evidence",
    "room_preview_read_evidence",
    "avatar_fetch_read_evidence",
    "pinned_events_subscription_evidence",
    "typing_notice_subscription_evidence",
    "room_members_read_evidence",
    "room_member_sync_read_evidence",
    "room_power_levels_read_evidence",
    "own_read_receipt_subscription_evidence",
    "room_info_header",
    "room_info_strip",
    "room_settings_surface",
    "room_settings_option_staging_local_evidence",
    "room_settings_name_id_clipboard_action",
    "room_settings_permissions_clipboard_action",
    "room_settings_members_clipboard_action",
    "room_settings_identity_clipboard_action",
    "room_settings_loaded_identity_preview",
    "room_settings_close_metadata_preview",
    "room_settings_refresh_metadata_preview",
    "room_settings_refresh_live_read_wiring",
    "room_settings_name_topic_live_write",
    "room_settings_canonical_alias_live_wiring",
    "room_settings_avatar_remove_live_wiring",
    "room_settings_tombstone_live_write",
    "room_settings_local_boundary_evidence",
    "room_settings_edit_controls_boundary",
    "room_settings_edit_intent_staging_local_evidence",
    "room_settings_field_edit_intent_controls_row",
    "room_settings_refresh_result_detail_controls_row",
    "room_settings_mutation_preflight_detail_controls_row",
    "room_settings_field_mutation_packet_drilldown_action",
    "room_settings_field_mutation_contract_packet_action",
    "room_settings_power_member_result_taxonomy_packet_action",
    "room_context_settings_surface",
    "notification_mute_header",
    "notification_mute_surface",
    "notification_option_staging_local_evidence",
    "notification_mode_write_confirmation_guard",
    "notifications_loaded_attention_preview",
    "notifications_mode_clipboard_action",
    "notifications_mode_target_metadata_preview",
    "notifications_close_refresh_metadata_preview",
    "notifications_local_boundary_evidence",
    "notifications_timed_global_boundary_metadata",
    "notifications_pusher_keyword_boundary",
    "notifications_keyword_list_live_read",
    "notifications_keyword_mutation_live_write",
    "notifications_pusher_status_live_read",
    "notifications_advanced_controls_row",
    "notifications_advanced_detail_controls_row",
    "notifications_result_detail_controls_row",
    "notifications_preflight_detail_controls_row",
    "notifications_rule_packet_drilldown_action",
    "notifications_rule_contract_packet_action",
    "notifications_result_taxonomy_packet_action",
    "notifications_retry_confirmation_guard",
    "room_context_notifications_surface",
    "room_menu_header",
    "room_actions_strip",
    "room_actions_close_local_evidence",
    "room_link_copy_handoff_evidence",
    "room_local_surface_close_evidence",
    "room_status_confirmation_guard",
    "invite_user_confirmation_guard",
    "timeline_invite_confirmation_guard",
    "invite_response_confirmation_required",
    "space_lobby_join_leave_modal_guard",
    "space_lobby_read_sync_evidence",
    "space_lobby_room_list_lifecycle_cleanup_evidence",
    "space_lobby_empty_state_read_sync_evidence",
    "space_lobby_membership_edge_evidence",
    "space_lobby_reknock_cancel_prior_packet_preview",
    "add_room_knock_confirmation_guard",
    "add_room_preview_cancel_evidence",
    "add_room_membership_edge_evidence",
    "add_room_reknock_cancel_prior_packet_preview",
    "add_room_restricted_join_rule_local_evidence",
    "message_pin_confirmation_guard",
    "message_edit_confirmation_guard",
    "message_edit_unsupported_features_local_evidence",
    "message_edit_detail_packet_preview",
    "message_edit_attachment_preflight_packet_preview",
    "message_edit_mention_payload_preflight_packet_preview",
    "message_edit_mention_payload_typed_contract_packet_preview",
    "message_edit_save_result_mapping_packet_preview",
    "message_edit_retry_error_drilldown_packet_preview",
    "message_report_surface",
    "message_report_option_staging_local_evidence",
    "message_report_send_local_boundary_evidence",
    "message_report_content_live_send_wiring",
    "message_report_moderation_workflow_boundary",
    "message_report_loaded_target_metadata_preview",
    "message_report_custom_reason_draft_metadata_preview",
    "message_report_cancel_local_evidence",
    "message_report_custom_reason_confirmation_guard",
    "message_report_status_lifecycle_surface",
    "message_report_status_clipboard_action",
    "message_report_retry_confirmation_guard",
    "message_report_workflow_actions_row",
    "message_report_moderation_reviewer_packet_action",
    "message_report_workflow_result_contract_packet_action",
    "message_report_workflow_result_taxonomy_packet_action",
    "message_report_preflight_detail_controls_row",
    "message_report_loaded_source_modal_action",
    "message_edit_history_surface",
    "message_edit_history_click_local_evidence",
    "message_edit_history_compact_summary_live_read_wiring",
    "message_edit_history_loaded_original_preview",
    "message_edit_history_loaded_target_metadata_preview",
    "message_edit_history_detail_surface",
    "message_edit_history_full_modal_boundary",
    "message_edit_history_local_full_snapshot_modal_action",
    "message_edit_history_full_controls_row",
    "message_edit_history_loaded_source_modal_action",
    "message_edit_history_loaded_diff_detail_state",
    "message_edit_history_loaded_diff_clipboard_action",
    "message_edit_history_loaded_side_by_side_diff_modal_action",
    "message_edit_history_full_diff_packet_action",
    "message_edit_history_full_history_result_contract_packet_action",
    "message_edit_history_remote_result_taxonomy_packet_action",
    "message_edit_history_preflight_detail_controls_row",
    "message_edit_history_retry_confirmation_guard",
    "message_edit_history_local_boundary_evidence",
    "tsp_identity_preview_surface",
    "tsp_wallet_pending_cancel_local_evidence",
    "tsp_pending_cancel_operation_packet_preview",
    "tsp_wallet_open_retry_evidence",
    "tsp_wallet_set_default_confirmation_metadata_preview",
    "tsp_wallet_remove_confirmation_metadata_preview",
    "tsp_wallet_delete_blocked_metadata_preview",
    "tsp_wallet_delete_preflight_result_packet_preview",
    "tsp_wallet_import_blocked_local_evidence",
    "tsp_wallet_import_blocked_metadata_preview",
    "tsp_wallet_import_preflight_packet_preview",
    "tsp_wallet_import_result_taxonomy_packet_preview",
    "tsp_worker_receipt_result_packet_preview",
    "tsp_association_cancel_local_evidence",
    "tsp_association_blocked_metadata_preview",
    "tsp_association_cancel_remove_packet_preview",
    "tsp_association_result_taxonomy_packet_preview",
    "tsp_verification_request_metadata_preview",
    "crypto_verification_request_metadata_preview",
    "login_auto_cancel_local_evidence",
    "attachment_upload_composer",
    "attachment_picker_surface",
    "attachment_option_staging_local_evidence",
    "attachment_camera_contact_local_boundary_evidence",
    "attachment_mobile_picker_controls_row",
    "attachment_mobile_share_sheet_boundary",
    "attachment_handoff_confirmation_guard",
    "attachment_send_handoff_evidence",
    "attachment_pre_send_review_local_evidence",
    "attachment_selected_file_preview_local_evidence",
    "attachment_selected_image_metadata_preview",
    "attachment_main_send_guard_local_evidence",
    "attachment_selection_replacement_preserve_evidence",
    "attachment_review_lifecycle_metadata_preview",
    "attachment_review_send_single_submit_evidence",
    "attachment_review_discard_close_idempotent_evidence",
    "attachment_caption_reply_context_boundary_evidence",
    "attachment_file_validation_local_error_evidence",
    "attachment_validation_error_recovery_evidence",
    "attachment_send_operation_status_local_evidence",
    "attachment_send_result_bridge_evidence",
    "attachment_queue_failure_recovery_copy_evidence",
    "attachment_send_failure_retry_confirmation_guard",
    "attachment_true_queue_control_local_boundary_evidence",
    "attachment_accepted_queue_actions_row",
    "attachment_accepted_queue_timeline_cancel_bridge",
    "attachment_local_send_abort_result_bridge",
    "attachment_per_file_status_controls_row",
    "attachment_per_file_queue_drilldown_action",
    "attachment_sdk_queue_contract_packet_action",
    "attachment_queue_progress_result_taxonomy_packet_action",
    "attachment_send_preflight_detail_controls_row",
    "attachment_multi_file_queue_boundary_evidence",
    "attachment_timeline_send_state_evidence",
    "attachment_timeline_cancel_local_send_evidence",
    "attachment_status_taxonomy_local_evidence",
    "attachment_review_row_compact_fit_evidence",
    "attachment_mobile_action_density_evidence",
    "media_message_preview_surface",
    "media_message_blocked_actions_evidence",
    "media_save_play_confirmation_guard",
    "media_download_metadata_preview",
    "media_metadata_clipboard_action",
    "media_save_dialog_lifecycle_metadata_preview",
    "media_save_destination_metadata_preview",
    "media_save_retry_confirmation_guard",
    "media_inline_playback_queue_boundary_metadata",
    "media_inline_player_disabled_controls",
    "media_codec_transcode_controls_row",
    "media_save_result_status_boundary",
    "media_save_result_recovery_controls_row",
    "media_cached_saved_file_status_snapshot",
    "media_save_preflight_detail_controls_row",
    "media_operation_packet_drilldown_action",
    "media_playback_queue_contract_packet_action",
    "media_playback_result_taxonomy_packet_action",
    "media_encrypted_metadata_preview",
    "media_encrypted_image_metadata_preview",
    "media_download_playback_local_boundary_evidence",
    "media_fetch_cache_read_evidence",
    "poll_answer_preview_result_packet_preview",
    "image_viewer_local_controls",
    "link_preview_local_controls",
    "link_preview_loaded_metadata_summary",
    "url_preview_read_evidence",
    "matrix_link_preview_surface",
    "matrix_link_preview_live_read_wiring",
    "matrix_link_loaded_alias_navigation",
    "matrix_link_loaded_event_local_jump",
    "matrix_link_current_room_event_pagination_live",
    "matrix_link_loaded_event_context_metadata",
    "matrix_link_loaded_event_source_modal_action",
    "matrix_link_target_metadata_preview",
    "matrix_link_preview_result_metadata",
    "matrix_link_preview_failure_metadata",
    "matrix_link_preview_retry_confirmation_guard",
    "matrix_link_server_context_boundary",
    "matrix_link_context_actions_row",
    "matrix_link_room_or_alias_join_live_wiring",
    "matrix_link_room_or_alias_knock_live_wiring",
    "matrix_link_user_invite_live_wiring",
    "matrix_link_browser_handoff_confirmation",
    "matrix_link_route_scope_controls_row",
    "matrix_link_route_drilldown_packet_action",
    "matrix_link_route_result_contract_packet_action",
    "matrix_link_route_result_taxonomy_packet_action",
    "matrix_link_room_target_clipboard_action",
    "matrix_link_via_servers_clipboard_action",
    "matrix_link_event_id_clipboard_action",
    "matrix_link_preview_metadata_clipboard_action",
    "matrix_link_unresolved_detail_state",
    "matrix_link_unknown_target_local_evidence",
    "matrix_link_unknown_target_boundary_evidence",
    "external_link_confirmation_guard",
    "event_source_local_surface",
    "event_source_clipboard_copy_evidence",
    "event_source_loaded_metadata_summary",
    "message_copy_local_surface",
    "message_copy_loaded_metadata_summary",
    "account_display_name_staging_surface",
    "account_display_name_confirmation_guard",
    "account_device_self_check_evidence",
    "account_avatar_upload_surface",
    "account_avatar_upload_option_staging_local_evidence",
    "account_avatar_upload_selected_file_preview",
    "account_avatar_upload_selected_image_metadata_preview",
    "account_avatar_upload_decode_probe",
    "account_avatar_upload_pixel_decode_live",
    "account_avatar_upload_live_wiring",
    "account_avatar_direct_mxc_setavatar_live_wiring",
    "account_avatar_upload_lifecycle_metadata_preview",
    "account_avatar_upload_retry_confirmation_guard",
    "account_avatar_upload_crop_editor_boundary",
    "account_avatar_upload_editor_controls_row",
    "account_avatar_upload_source_preview_controls_row",
    "account_avatar_upload_source_editor_drilldown_packet_action",
    "account_avatar_upload_source_editor_typed_contract_packet_action",
    "account_avatar_upload_source_editor_result_taxonomy_packet_action",
    "account_avatar_upload_source_path_clipboard_action",
    "account_avatar_upload_preflight_detail_controls_row",
    "account_avatar_upload_local_boundary_evidence",
    "account_avatar_delete_confirmation_guard",
    "account_avatar_delete_live_wiring",
    "account_management_surface",
    "account_management_option_staging_local_evidence",
    "account_management_loaded_identity_preview",
    "account_management_live_wiring",
    "account_management_display_name_live_wiring",
    "account_management_device_directory_live_wiring",
    "account_management_current_device_rename_live_wiring",
    "account_management_browser_portal_handoff",
    "account_management_lifecycle_metadata_preview",
    "account_management_refresh_confirmation_guard",
    "account_management_local_boundary_evidence",
    "account_management_session_revoke_boundary",
    "account_management_session_actions_row",
    "account_management_device_directory_controls_row",
    "account_management_current_device_metadata_controls_row",
    "account_management_current_device_id_clipboard_action",
    "account_management_current_device_verification_clipboard_action",
    "account_management_current_device_display_name_clipboard_action",
    "account_management_current_session_clipboard_action",
    "account_management_current_device_source_clipboard_action",
    "account_management_preflight_detail_controls_row",
    "account_management_session_device_drilldown_packet_action",
    "account_management_session_device_typed_contract_packet_action",
    "account_management_session_device_result_taxonomy_packet_action",
    "account_local_surface_close_evidence",
    "account_logout_confirmation_guard",
    "emoji_sticker_picker_composer",
    "emoji_sticker_surface",
    "emoji_sticker_send_local_boundary_evidence",
    "emoji_sticker_lifecycle_metadata_preview",
    "mention_helper_surface",
    "mention_option_staging_local_evidence",
    "mention_cached_selection_preview",
    "mention_loaded_identity_preview",
    "mention_local_candidate_rows_preview",
    "mention_local_duplicate_hints_preview",
    "mention_lifecycle_metadata_preview",
    "mention_keyboard_selection_boundary",
    "mention_rich_picker_boundary_evidence",
    "mention_directory_disambiguation_boundary",
    "mention_rich_directory_controls_row",
    "mention_directory_search_live_wiring",
    "mention_directory_result_promotion_live",
    "mention_hover_card_snapshot_live",
    "mention_local_pill_tray_live",
    "mention_preflight_detail_controls_row",
    "mention_send_local_boundary_evidence",
    "mention_send_payload_metadata_preview",
    "mention_send_live_payload_wiring",
    "mention_payload_scope_controls_row",
    "mention_payload_drilldown_packet_action",
    "mention_payload_typed_contract_packet_action",
    "mention_remote_result_taxonomy_packet_action",
    "composer_send_shortcut_local_preference_evidence",
    "message_send_operation_status_local_evidence",
    "composer_typing_notice_send_evidence",
    "composer_local_surface_close_evidence",
    "location_send_confirmation_guard",
    "live_location_continuous_updates_boundary",
    "voice_message_composer",
    "voice_message_surface",
    "voice_option_staging_local_evidence",
    "voice_message_send_local_blocked_evidence",
    "voice_send_live_attachment_wiring",
    "voice_selected_audio_metadata_preview",
    "voice_selected_audio_waveform_codec_preview",
    "voice_lifecycle_metadata_preview",
    "voice_confirmation_cancel_metadata_preview",
    "voice_permission_recording_local_boundary_evidence",
    "voice_recorder_waveform_codec_boundary",
    "voice_recorder_status_controls_row",
    "voice_capture_lifecycle_controls_row",
    "voice_mobile_picker_controls_row",
    "voice_review_playback_controls_row",
    "voice_review_drop_pending_audio_action",
    "voice_send_preflight_detail_controls_row",
    "voice_recorder_lifecycle_drilldown_packet_action",
    "voice_recorder_typed_contract_packet_action",
    "voice_recorder_result_taxonomy_packet_action",
    "profile_direct_message_confirmation_guard",
    "direct_message_create_confirmation_guard",
    "profile_ignore_confirmation_guard",
    "profile_member_read_evidence",
    "profile_read_receipt_surface",
    "profile_account_identity_clipboard_surface",
];

pub fn hepta_telegram_base_direct_reuse_count() -> usize {
    HEPTA_TELEGRAM_BASE_CAPABILITIES
        .iter()
        .filter(|capability| capability.status == HeptaTelegramBaseStatus::DirectReuse)
        .count()
}

pub fn hepta_telegram_base_gap_count() -> usize {
    HEPTA_TELEGRAM_BASE_CAPABILITIES
        .iter()
        .filter(|capability| capability.status == HeptaTelegramBaseStatus::Gap)
        .count()
}

pub fn hepta_telegram_base_contract_ready() -> bool {
    hepta_telegram_base_direct_reuse_count() >= 8
        && hepta_telegram_base_gap_count() == HEPTA_TELEGRAM_BASE_GAPS.len()
}

pub fn hepta_telegram_local_gap_affordance_count() -> usize {
    HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.len()
}

pub fn hepta_telegram_base_gap_hard_boundary_count() -> usize {
    HEPTA_TELEGRAM_BASE_GAP_HARD_BOUNDARY_MARKERS.len()
}

pub fn hepta_telegram_base_gap_product_runway_count() -> usize {
    HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.len()
}

pub fn hepta_telegram_base_gap_hard_boundary_audit_ready() -> bool {
    hepta_telegram_base_gap_count() == HEPTA_TELEGRAM_BASE_GAPS.len()
        && hepta_telegram_base_gap_hard_boundary_count() == HEPTA_TELEGRAM_BASE_GAPS.len()
}

pub fn hepta_telegram_base_gap_product_runway_ready() -> bool {
    hepta_telegram_base_gap_product_runway_count() == HEPTA_TELEGRAM_BASE_GAPS.len()
        && HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY
            .iter()
            .all(|item| HEPTA_TELEGRAM_BASE_GAPS.contains(&item.gap_id))
}

#[cfg(test)]
fn telegram_matrix_request_label(
    request: &crate::sliding_sync::MatrixRequest,
) -> Option<&'static str> {
    use crate::sliding_sync::MatrixRequest;

    match request {
        MatrixRequest::PaginateTimeline { .. } => Some("paginate_timeline"),
        MatrixRequest::CreateThreadTimeline { .. } => Some("create_thread_timeline"),
        MatrixRequest::FetchThreadSummaryDetails { .. } => Some("fetch_thread_summary_details"),
        MatrixRequest::FetchEditHistory { .. } => Some("fetch_edit_history"),
        MatrixRequest::FetchEventSource { .. } => Some("fetch_event_source"),
        MatrixRequest::FetchDetailsForEvent { .. } => Some("fetch_details_for_event"),
        MatrixRequest::GetNumberUnreadMessages { .. } => Some("get_number_unread_messages"),
        MatrixRequest::GetSuccessorRoomDetails { .. } => Some("get_successor_room_details"),
        MatrixRequest::GetRoomPreview { .. } => Some("get_room_preview"),
        MatrixRequest::PreviewMatrixLinkTarget { .. } => Some("preview_matrix_link_target"),
        MatrixRequest::FetchAvatar { .. } => Some("fetch_avatar"),
        MatrixRequest::SyncRoomMemberList { .. } => Some("sync_room_member_list"),
        MatrixRequest::EditMessage { .. } => Some("edit_message"),
        MatrixRequest::SetUnreadFlag { .. } => Some("set_unread_flag"),
        MatrixRequest::SetIsFavorite { .. } => Some("set_is_favorite"),
        MatrixRequest::SetIsLowPriority { .. } => Some("set_is_low_priority"),
        MatrixRequest::SetRoomNotificationMode { .. } => Some("set_room_notification_mode"),
        MatrixRequest::GenerateMatrixLink { .. } => Some("generate_matrix_link"),
        MatrixRequest::FetchMedia { .. } => Some("fetch_media"),
        MatrixRequest::SaveMedia { .. } => Some("save_media"),
        MatrixRequest::SendMessage { .. } => Some("send_message"),
        MatrixRequest::SendAttachment { .. } => Some("send_attachment"),
        MatrixRequest::SendTypingNotice { .. } => Some("send_typing_notice"),
        MatrixRequest::SubscribeToTypingNotices { .. } => Some("subscribe_typing_notices"),
        MatrixRequest::SubscribeToOwnUserReadReceiptsChanged { .. } => {
            Some("subscribe_own_read_receipts")
        }
        MatrixRequest::SubscribeToPinnedEvents { .. } => Some("subscribe_pinned_events"),
        MatrixRequest::GetRoomNotificationMode { .. } => Some("get_room_notification_mode"),
        MatrixRequest::GetNotificationKeywordRules { .. } => Some("get_notification_keyword_rules"),
        MatrixRequest::GetNotificationPusherStatus { .. } => Some("get_notification_pusher_status"),
        MatrixRequest::ReadReceipt { .. } => Some("read_receipt"),
        MatrixRequest::ToggleReaction { .. } => Some("toggle_reaction"),
        MatrixRequest::RedactMessage { .. } => Some("redact_message"),
        MatrixRequest::ReportContent { .. } => Some("report_content"),
        MatrixRequest::PinEvent { .. } => Some("pin_event"),
        MatrixRequest::Knock { .. } => Some("knock_room"),
        MatrixRequest::GetUrlPreview { .. } => Some("get_url_preview"),
        _ => None,
    }
}

#[cfg(test)]
fn telegram_space_request_label(
    request: &crate::space_service_sync::SpaceRequest,
) -> Option<&'static str> {
    use crate::space_service_sync::SpaceRequest;

    match request {
        SpaceRequest::SubscribeToSpaceRoomList { .. } => Some("subscribe_space_room_list"),
        SpaceRequest::PaginateSpaceRoomList { .. } => Some("paginate_space_room_list"),
        SpaceRequest::GetChildren { .. } => Some("get_space_children"),
        SpaceRequest::GetDetailedChildren { .. } => Some("get_detailed_space_children"),
        SpaceRequest::GetTopLevelSpaceDetails { .. } => Some("get_top_level_space_details"),
        SpaceRequest::UnsubscribeFromSpaceRoomList { .. } | SpaceRequest::LeaveSpace { .. } => None,
    }
}

#[cfg(test)]
mod tests;
