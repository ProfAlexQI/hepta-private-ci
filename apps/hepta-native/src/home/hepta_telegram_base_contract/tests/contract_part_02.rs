#[test]
fn hepta_telegram_base_room_settings_name_id_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_NAME_ID_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_name_id_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_name_id_clipboard_action")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_ID_CLIPBOARD_EVIDENCE.contains("RoomNameId")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_ID_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_ID_CLIPBOARD_EVIDENCE.contains("m.room.name")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_ID_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_NAME_ID_CLIPBOARD_LABEL
            .contains("loaded room label/id")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings name/id clipboard"
            && capability.base_module.contains("RoomNameId")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("Missing room id")
            && capability.notes.contains("m.room.name")
            && capability.notes.contains("m.room.topic")
            && capability.notes.contains("m.room.power_levels")
            && capability.notes.contains("membership list write")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway
                .current_path
                .contains("Name/id, identity, permissions, and members clipboard")
            && runway
                .remaining_gap
                .contains("editable power-level/member state")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_permissions_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_permissions_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_permissions_clipboard_action")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE
            .contains("tl_state.user_power")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE
            .contains("GetRoomPowerLevels")
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
        crate::home::room_screen::ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_LABEL
            .contains("loaded power-level summary")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings permissions clipboard"
            && capability.base_module.contains("tl_state.user_power")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("Missing power-level state")
            && capability.notes.contains("GetRoomPowerLevels")
            && capability.notes.contains("m.room.power_levels mutation")
            && capability.notes.contains("m.room.name")
            && capability.notes.contains("m.room.topic")
            && capability.notes.contains("membership list write")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway
                .current_path
                .contains("Name/id, identity, permissions, and members clipboard")
            && runway
                .remaining_gap
                .contains("editable power-level/member state")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_members_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_MEMBERS_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_members_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_members_clipboard_action")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE
            .contains("room_members cache")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE
            .contains("GetRoomMembers(server-backed")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE
            .contains("m.room.member")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MEMBERS_CLIPBOARD_LABEL
            .contains("local member-cache summary")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings members clipboard"
            && capability.base_module.contains("room_members local cache")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("Missing member cache")
            && capability.notes.contains("GetRoomMembers(server-backed")
            && capability.notes.contains("membership list write")
            && capability.notes.contains("m.room.member")
            && capability.notes.contains("m.room.power_levels mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway
                .current_path
                .contains("Name/id, identity, permissions, and members clipboard")
            && runway
                .remaining_gap
                .contains("editable power-level/member state")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_identity_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_IDENTITY_CLIPBOARD_MARKER,
        "hepta_telegram_room_settings_identity_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_identity_clipboard_action")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE
            .contains("RoomContextMenuDetails")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE
            .contains("canonical alias")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE
            .contains("m.room.canonical_alias")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_IDENTITY_CLIPBOARD_LABEL
            .contains("loaded room-list identity metadata")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings identity clipboard"
            && capability.base_module.contains("RoomContextMenuDetails")
            && capability.base_module.contains("RoomsList")
            && capability.notes.contains("local clipboard")
            && capability
                .notes
                .contains("Missing room-list identity metadata")
            && capability.notes.contains("canonical alias")
            && capability.notes.contains("alternative alias count")
            && capability.notes.contains("avatar cache state")
            && capability.notes.contains("m.room.canonical_alias")
            && capability.notes.contains("m.room.tombstone")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway
                .current_path
                .contains("Name/id, identity, permissions, and members clipboard")
            && runway
                .remaining_gap
                .contains("editable power-level/member state")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_edit_controls_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_MARKER,
        "hepta_telegram_room_settings_edit_controls_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE
            .contains("partial-live settings strip")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE
            .contains("loaded RoomsList identity readiness")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE
            .contains("cached member count")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE
            .contains("power-level display readiness")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE
            .contains("History visibility")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE
            .contains("Join rule")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE
            .contains("Member moderation")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_LABEL.contains(
            "Name/Topic, alias, avatar upload/remove, history, join-rule, and tombstone writes confirm first"
        )
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_edit_controls_boundary"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings edit controls boundary"
            && capability
                .base_module
                .contains("settings_edit_controls_boundary")
            && capability.notes.contains("partial-live settings strip")
            && capability
                .notes
                .contains("loaded RoomsList identity readiness")
            && capability.notes.contains("cached member count")
            && capability.notes.contains("power-level display readiness")
            && capability.notes.contains("MatrixRequest::SetRoomName")
            && capability.notes.contains("MatrixRequest::UploadRoomAvatar")
            && capability.notes.contains("MatrixRequest::RemoveRoomAvatar")
            && capability.notes.contains("Room::upload_avatar")
            && capability.notes.contains("RoomSettingsMutationResult")
            && capability.notes.contains("History visibility")
            && capability.notes.contains("Join rule")
            && capability.notes.contains("Tombstone")
            && capability.notes.contains("MatrixRequest::SetRoomTombstone")
            && capability.notes.contains("Power levels")
            && capability.notes.contains("Member moderation")
            && capability.notes.contains("notification-rule handoff")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("unrelated live mutation")
            && capability.notes.contains("room_settings")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_edit_intent_staging_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_EDIT_INTENT_STAGING_MARKER,
        "hepta_telegram_room_settings_edit_intent_staging_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE
            .contains("Power and Moderation")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE
            .contains("loaded room identity")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE
            .contains("cached member count")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE
            .contains("power-level display readiness")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE
            .contains("Avatar opens the confirmed room-avatar upload path")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE
            .contains("MatrixRequest::SetRoomCanonicalAlias")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE
            .contains("m.room.power_levels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_EVIDENCE
            .contains("member moderation")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_EDIT_INTENT_STAGING_LABEL
            .contains("Edit intent staged locally")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_edit_intent_staging_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings edit intent staging"
            && capability
                .base_module
                .contains("avatar/history/join/tombstone live controls")
            && capability
                .notes
                .contains("Avatar opens the confirmed room-avatar upload path")
            && capability
                .notes
                .contains("History, Join rule, and Tombstone open confirmed room-state writes")
            && capability.notes.contains("loaded room identity")
            && capability.notes.contains("cached member count")
            && capability.notes.contains("power-level display readiness")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomCanonicalAlias")
            && capability.notes.contains("m.room.power_levels")
            && capability.notes.contains("notification-rule handoff")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_field_edit_intent_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_MARKER,
        "hepta_telegram_room_settings_field_edit_intent_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("Name, Topic, Alias, Avatar, Remove avatar, Permissions, and Members")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("local field edit-intent metadata")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("loaded room identity")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("cached member count")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("power-level display readiness")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("separate confirmed Save path")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("draft/metadata")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("MatrixRequest::UploadRoomAvatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_LABEL
            .contains("Avatar upload and Remove avatar confirm live")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_field_edit_intent_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings field edit intent controls"
            && capability
                .base_module
                .contains("settings_field_edit_intents")
            && capability.notes.contains("Name edit, Topic edit")
            && capability.notes.contains("Avatar edit")
            && capability.notes.contains("Remove avatar")
            && capability.notes.contains("Perms edit")
            && capability.notes.contains("Members edit")
            && capability.notes.contains("loaded room identity")
            && capability.notes.contains("cached member count")
            && capability.notes.contains("power-level display readiness")
            && capability.notes.contains("separate confirmed Save path")
            && capability.notes.contains("MatrixRequest::SetRoomName")
            && capability.notes.contains("MatrixRequest::SetRoomTopic")
            && capability.notes.contains("MatrixRequest::UploadRoomAvatar")
            && capability.notes.contains("MatrixRequest::RemoveRoomAvatar")
            && capability.notes.contains("m.room.power_levels")
            && capability.notes.contains("membership list writes")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("unrelated live mutation")
            && capability.notes.contains("room_settings")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_refresh_result_detail_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_RESULT_DETAIL_MARKER,
        "hepta_telegram_room_settings_refresh_result_detail_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE
            .contains("Result, Members, Power, Failure, and Source")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE
            .contains("local refresh result metadata")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE
            .contains("MatrixRequest::GetRoomPowerLevels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE
            .contains("MatrixRequest::GetRoomMembers(server-backed)")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE
            .contains("m.room.name")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE
            .contains("m.room.topic")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE
            .contains("m.room.avatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_EVIDENCE
            .contains("m.room.power_levels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_RESULT_DETAIL_LABEL
            .contains("Refresh result detail stays local")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_refresh_result_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings refresh result detail controls"
            && capability
                .base_module
                .contains("settings_refresh_result_controls")
            && capability.notes.contains("Result, Members, Power")
            && capability.notes.contains("Failure")
            && capability.notes.contains("Source")
            && capability.notes.contains("timeline availability")
            && capability.notes.contains("cached member count")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomPowerLevels")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomMembers(server-backed)")
            && capability.notes.contains("no extra reads")
            && capability
                .notes
                .contains("no extra room-state write outside the confirmed settings write paths")
            && capability.notes.contains("m.room.power_levels")
            && capability.notes.contains("membership list writes")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_refresh_live_read_wiring_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_MARKER,
        "hepta_telegram_room_settings_refresh_live_read_wiring_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_EVIDENCE
            .contains("MatrixRequest::GetRoomPowerLevels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_EVIDENCE
            .contains("MatrixRequest::GetRoomMembers")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_EVIDENCE
            .contains("TimelineUpdate::UserPowerLevels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_EVIDENCE
            .contains("TimelineUpdate::RoomMembersListFetched")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_EVIDENCE
            .contains("Editable m.room.*")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_settings_refresh_live_read_wiring")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway.current_path.contains("live Refresh read wiring")
            && runway.current_path.contains("GetRoomPowerLevels")
            && runway
                .current_path
                .contains("GetRoomMembers(server-backed)")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings refresh live read wiring"
            && capability
                .base_module
                .contains("refresh_telegram_room_settings_read_paths")
            && capability.notes.contains("partial-live for reads only")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomPowerLevels")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomMembers(local_only=false, JOIN)")
            && capability.notes.contains("TimelineUpdate::UserPowerLevels")
            && capability
                .notes
                .contains("TimelineUpdate::RoomMembersListFetched")
            && capability.notes.contains("Power-level writes")
            && capability
                .notes
                .contains("backend room-state mutation/result contracts")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_mutation_preflight_detail_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_MARKER,
        "hepta_telegram_room_settings_mutation_preflight_detail_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Request, Packet, Contract, Taxonomy, Result, Error, Retry, and Source")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("local room-state mutation packet snapshot")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("local mutation-preflight metadata")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("power/member permission-denial and result taxonomy slots")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("m.room.name")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("m.room.topic")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("m.room.avatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("m.room.canonical_alias")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("m.room.power_levels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("retry automation")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("Room-state mutation preflight stays local")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_mutation_preflight_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings mutation preflight detail controls"
            && capability
                .base_module
                .contains("settings_mutation_preflight_controls")
            && capability
                .base_module
                .contains("room_settings_mutation_request_packet_snapshot_label")
            && capability
                .notes
                .contains("Request, Packet, Contract, Taxonomy")
            && capability
                .notes
                .contains("local room-state mutation packet snapshot")
            && capability
                .notes
                .contains("permission-denial and result taxonomy")
            && capability.notes.contains("Retry")
            && capability.notes.contains("Source")
            && capability.notes.contains("timeline availability")
            && capability.notes.contains("cached member count")
            && capability.notes.contains("m.room.name")
            && capability.notes.contains("m.room.topic")
            && capability.notes.contains("m.room.avatar")
            && capability.notes.contains("m.room.canonical_alias")
            && capability.notes.contains("m.room.power_levels")
            && capability.notes.contains("retry automation")
            && capability
                .notes
                .contains("room-state mutation contract call")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway.current_path.contains("mutation preflight controls")
            && runway
                .next_ui_safe_step
                .contains("room-state mutation/result contracts")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_field_mutation_packet_drilldown_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_MARKER,
        "hepta_telegram_room_settings_field_mutation_packet_drilldown_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("field-by-field room-state mutation packet")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("confirmation, request, result, error, retry, and source")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("m.room.name")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("m.room.topic")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("m.room.avatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("m.room.canonical_alias")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("m.room.history_visibility")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("m.room.join_rules")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("m.room.power_levels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("m.room.member moderation")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_LABEL
            .contains("Packet copies field-by-field")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_field_mutation_packet_drilldown_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings field mutation packet drilldown"
            && capability
                .base_module
                .contains("copy_telegram_room_settings_field_mutation_packet")
            && capability
                .base_module
                .contains("room_settings_field_mutation_packet_payload")
            && capability.notes.contains("field-by-field mutation packet")
            && capability.notes.contains("confirmation, request, result")
            && capability.notes.contains("m.room.name")
            && capability.notes.contains("m.room.topic")
            && capability.notes.contains("m.room.avatar")
            && capability.notes.contains("canonical aliases")
            && capability.notes.contains("history visibility")
            && capability.notes.contains("join-rule")
            && capability.notes.contains("power levels")
            && capability.notes.contains("member moderation")
            && capability.notes.contains("tombstone")
            && capability.notes.contains("notification handoff")
            && capability.notes.contains("retry automation")
            && capability
                .notes
                .contains("room-state mutation contract call")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway
                .current_path
                .contains("field mutation packet drilldown")
            && runway
                .next_ui_safe_step
                .contains("room-state mutation/result contracts")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_field_mutation_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_MARKER,
        "hepta_telegram_room_settings_field_mutation_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("typed room-state mutation/result contract packet")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("m.room.name")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("m.room.topic")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("m.room.avatar")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("canonical alias")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("history visibility")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("join-rule")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("power levels")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_EVIDENCE
            .contains("member moderation")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_LABEL
            .contains("typed room-state mutation/result contracts")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_field_mutation_contract_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings field mutation contract packet"
            && capability
                .base_module
                .contains("copy_telegram_room_settings_field_mutation_contract_packet")
            && capability
                .base_module
                .contains("room_settings_field_mutation_contract_packet_payload")
            && capability
                .notes
                .contains("typed room-state mutation/result contract packet")
            && capability.notes.contains("baseline identity")
            && capability.notes.contains("m.room.name")
            && capability.notes.contains("m.room.topic")
            && capability.notes.contains("m.room.avatar")
            && capability.notes.contains("canonical aliases")
            && capability.notes.contains("history visibility")
            && capability.notes.contains("join-rule")
            && capability.notes.contains("power levels")
            && capability.notes.contains("member moderation")
            && capability.notes.contains("tombstone")
            && capability.notes.contains("notification handoff")
            && capability.notes.contains("retry automation")
            && capability
                .notes
                .contains("room-state mutation contract call")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("room_settings")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway
                .current_path
                .contains("typed room-state mutation/result contract packet")
            && runway
                .next_ui_safe_step
                .contains("room-state mutation/result contracts")
    }));
}

#[test]
fn hepta_telegram_base_room_settings_power_member_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_room_settings_power_member_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("permission-denial and result taxonomy packet")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("operation_id_slot not_assigned")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("permission_denied/forbidden/stale-baseline")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("invite, kick, ban, knock")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("no m.room.power_levels write")
    );
    assert!(
        crate::home::room_screen::ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_LABEL
            .contains("power/member permission denial")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"room_settings_power_member_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room settings power member result taxonomy packet"
            && capability
                .base_module
                .contains("copy_telegram_room_settings_power_member_result_taxonomy_packet")
            && capability
                .base_module
                .contains("room_settings_power_member_result_taxonomy_packet_payload")
            && capability
                .notes
                .contains("permission-denial and result taxonomy")
            && capability
                .notes
                .contains("power_levels_operation_id not_assigned")
            && capability.notes.contains("permission_denied")
            && capability
                .notes
                .contains("member_moderation_operation_id not_assigned")
            && capability.notes.contains("invite/kick/ban/knock")
            && capability.notes.contains("retry/source-hash")
            && capability.notes.contains("m.room.power_levels write")
            && capability.notes.contains("m.room.member mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "room_settings"
            && runway
                .current_path
                .contains("power/member permission-denial result taxonomy packet")
            && runway
                .next_ui_safe_step
                .contains("room-state mutation/result contracts")
    }));
}

#[test]
fn hepta_telegram_base_message_search_mode_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_MODE_MARKER,
        "hepta_telegram_message_search_mode_local_only"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_LOCAL_SURFACE_MARKER,
        "hepta_telegram_message_search_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_EMPTY_CLOSE_LOCAL_MARKER,
        "hepta_telegram_message_search_empty_close_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SIDEBAR_MESSAGE_SEARCH_LOCAL_BUTTON_MARKER,
        "hepta_telegram_sidebar_message_search_local_button_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_MARKER,
        "hepta_telegram_sidebar_message_search_open_handoff_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_header"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"sidebar_message_search_button"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"sidebar_message_search_open_handoff"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_mode_strip"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_input_results_jumps"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_empty_close_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_loaded_timeline_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search"
            && capability.notes.contains("loaded-timeline local search")
            && capability
                .notes
                .contains("partial-live Matrix server search")
            && capability.notes.contains("Server query")
            && capability.notes.contains("next_batch cursor")
            && capability.notes.contains("failed Retry")
    }));
}

#[test]
fn hepta_telegram_base_message_search_empty_close_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_EMPTY_CLOSE_LOCAL_MARKER,
        "hepta_telegram_message_search_empty_close_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_empty_close_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search empty/close local evidence"
            && capability.notes.contains("loaded-timeline-only search")
            && capability
                .notes
                .contains("empty results, Close, and Escape")
            && capability
                .notes
                .contains("send no live SearchMessagesServer request")
            && capability.notes.contains("failed Retry")
    }));
}

#[test]
fn hepta_telegram_base_message_search_loaded_timeline_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_PREVIEW_MARKER,
        "hepta_telegram_message_search_loaded_preview_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_MARKER,
        "hepta_telegram_message_search_loaded_timeline_boundary_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_EVIDENCE
            .contains("loaded-timeline-only local helper")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_EVIDENCE
            .contains("plaintext_body_of_timeline_item")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_EVIDENCE
            .contains("active-match preview snippet")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_EVIDENCE
            .contains("server-side history query")
    );
    assert!(
        crate::home::search_messages::SIDEBAR_MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_EVIDENCE
            .contains("server-side history request")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_TIMELINE_BOUNDARY_LABEL
            .contains("no Matrix-backed history search")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_loaded_timeline_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_loaded_preview_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search loaded timeline boundary evidence"
            && capability.base_module.contains("SearchMessagesButton")
            && capability.notes.contains("RoomScreen tl_state")
            && capability
                .notes
                .contains("active-match plaintext preview snippet")
            && capability.notes.contains("plaintext_body_of_timeline_item")
            && capability
                .notes
                .contains("live SearchMessagesServer request")
            && capability.notes.contains("failed Retry")
            && capability.notes.contains("timeline pagination")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_sidebar_message_search_opens_active_room_strip_locally() {
    assert_eq!(
        HEPTA_TELEGRAM_SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_MARKER,
        "hepta_telegram_sidebar_message_search_open_handoff_ready"
    );
    assert!(
        crate::home::search_messages::SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_EVIDENCE
            .contains("SearchMessagesAction::LocalPreviewOpened")
    );
    assert!(
        crate::home::search_messages::SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_EVIDENCE
            .contains("active RoomScreen handles that action")
    );
    assert!(
        crate::home::search_messages::SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_EVIDENCE
            .contains("telegram_message_search_strip")
    );
    assert!(
        crate::home::search_messages::SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_EVIDENCE
            .contains("sends no Matrix-backed search")
    );
    assert!(
        crate::home::search_messages::SIDEBAR_MESSAGE_SEARCH_OPEN_HANDOFF_LABEL
            .contains("loaded-timeline search strip")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"sidebar_message_search_open_handoff"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search sidebar open handoff"
            && capability
                .notes
                .contains("SearchMessagesAction::LocalPreviewOpened")
            && capability.notes.contains("active RoomScreen handles")
            && capability.notes.contains("telegram_message_search_strip")
            && capability.notes.contains("loaded-timeline search UI")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("timeline pagination")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_loaded_metadata_summary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_METADATA_MARKER,
        "hepta_telegram_message_search_loaded_metadata_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_METADATA_EVIDENCE.contains("query length")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_METADATA_EVIDENCE
            .contains("loaded timeline item count")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_METADATA_EVIDENCE
            .contains("active loaded event-id availability")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_METADATA_EVIDENCE
            .contains("server-side history query")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_LOADED_METADATA_LABEL
            .contains("no server-side search")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_loaded_metadata_summary")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search loaded metadata summary"
            && capability.notes.contains("loaded RoomScreen tl_state")
            && capability.notes.contains("local search state")
            && capability.notes.contains("query length")
            && capability.notes.contains("active loaded index")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_active_result_detail_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_MARKER,
        "hepta_telegram_message_search_active_result_detail_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_EVIDENCE
            .contains("currently loaded timeline match")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_EVIDENCE
            .contains("loaded item index")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_EVIDENCE
            .contains("loaded event-id availability")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_EVIDENCE
            .contains("local occurrence count")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_EVIDENCE
            .contains("plaintext_body_of_timeline_item")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_EVIDENCE
            .contains("server-side history query")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_LABEL
            .contains("no server-side search")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_active_result_detail"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search active result detail"
            && capability.notes.contains("currently loaded timeline match")
            && capability.notes.contains("active ordinal")
            && capability.notes.contains("loaded item index")
            && capability.notes.contains("local occurrence count")
            && capability.notes.contains("plaintext_body_of_timeline_item")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("timeline pagination")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_query_lifecycle_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_message_search_query_lifecycle_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_query_lifecycle_metadata")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE
            .contains("query lifecycle metadata")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE
            .contains("resets active_match to 0")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE
            .contains("already loaded RoomScreen tl_state")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE
            .contains("Close/Escape clears")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE
            .contains("server-side history query")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_LABEL
            .contains("rescan loaded tl_state only")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search query lifecycle metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("loaded_message_search_query_lifecycle_label")
            && capability.notes.contains("surface visibility")
            && capability.notes.contains("reset active_match to 0")
            && capability
                .notes
                .contains("rescan already loaded RoomScreen tl_state")
            && capability.notes.contains("Close/Escape clears query")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("server-side history query")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_server_context_boundary_has_live_search_pagination() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_MARKER,
        "hepta_telegram_message_search_server_context_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_MARKER,
        "hepta_telegram_message_search_server_context_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_PAGINATION_LIVE_MARKER,
        "hepta_telegram_message_search_server_pagination_live_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_server_context_boundary")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_server_context_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_server_pagination_live_wiring")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("Server submits MatrixRequest::SearchMessagesServer")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("Context can use the first cached current-room server hit")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("Load older submits the returned next_batch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("failed Retry resubmits the current query")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("Source can open raw event JSON cached from the last server result")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::FetchEventSource")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_LABEL
            .contains("live Matrix search reads")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_EVIDENCE
            .contains("Server submits the first live MatrixRequest::SearchMessagesServer page")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_EVIDENCE
            .contains("Older submits the returned next_batch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_LABEL
            .contains("Server/Older are live Matrix search reads")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search server context boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("server_context_boundary")
            && capability
                .notes
                .contains("MatrixRequest::SearchMessagesServer")
            && capability.notes.contains("next_batch cursor")
            && capability.notes.contains("failed Retry")
            && capability
                .notes
                .contains("first cached current-room server hit")
            && capability
                .notes
                .contains("Source can open raw event JSON cached")
            && capability.notes.contains("MatrixRequest::FetchEventSource")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search server context controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("search_server_context_controls")
            && capability
                .notes
                .contains("first live MatrixRequest::SearchMessagesServer page")
            && capability.notes.contains("stored next_batch")
            && capability
                .notes
                .contains("first cached current-room hit event id")
            && capability
                .notes
                .contains("Source can open cached raw event JSON")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search server pagination live wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("submit_telegram_message_search_server_next_page")
            && capability
                .base_module
                .contains("MatrixRequest::SearchMessagesServer(next_batch)")
            && capability
                .base_module
                .contains("submit_telegram_message_search_server_context_event")
            && capability.notes.contains("next_batch cursor")
            && capability.notes.contains("/_matrix/client/v3/search")
            && capability
                .notes
                .contains("read-only timeline context pagination")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_result_action_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_MARKER,
        "hepta_telegram_message_search_result_action_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_result_action_controls_row")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
            .contains("visible controls")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
            .contains("Jump, Copy, Source, Thread, and Sender")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
            .contains("current loaded timeline match")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
            .contains("Jump scrolls/highlights")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
            .contains("Source opens the existing local EventSourceModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
            .contains("Room::load_or_fetch_event")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
            .contains("Thread opens the existing thread-focused timeline path")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
            .contains("Sender opens the existing UserProfileSlidingPane")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_LABEL
            .contains("Sender opens the existing profile pane")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search result action controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("search_result_action_controls")
            && capability
                .notes
                .contains("Jump, Copy, Source, Thread, and Sender")
            && capability.notes.contains("Jump scrolls/highlights")
            && capability.notes.contains("Copy writes")
            && capability.notes.contains("Source opens")
            && capability.notes.contains("MatrixRequest::FetchEventSource")
            && capability.notes.contains("Thread opens")
            && capability.notes.contains("Sender opens")
            && capability.notes.contains("UserProfileSlidingPane")
            && capability.notes.contains("GetUserProfile")
            && capability.notes.contains("EventSourceModal")
            && capability.notes.contains("active loaded match")
            && capability.notes.contains("profile mutation")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    let runway = HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY
        .iter()
        .find(|runway| runway.gap_id == "message_search")
        .expect("message search runway exists");
    assert!(runway.current_path.contains("result-action"));
    assert!(runway.current_path.contains("Jump loaded-match"));
    assert!(runway.current_path.contains("Thread open"));
    assert!(runway.current_path.contains("Sender profile-pane"));
    assert!(runway.current_path.contains("Copy clipboard"));
    assert!(runway.current_path.contains("Source modal"));
    assert!(runway.current_path.contains("server-query packet snapshot"));
    assert!(
        runway
            .current_path
            .contains("live Matrix /search first page")
    );
    assert!(
        runway
            .current_path
            .contains("failed Retry first-page resubmit")
    );
    assert!(runway.remaining_gap.contains("remote event context"));
    assert!(!runway.remaining_gap.contains("source fetch"));
    assert!(
        runway
            .remaining_gap
            .contains("remote result-action adapter")
    );
    assert!(!runway.remaining_gap.contains("Sender result action"));
}

#[test]
fn hepta_telegram_base_message_search_result_jump_loaded_match_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_MARKER,
        "hepta_telegram_message_search_result_jump_loaded_match_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_result_jump_loaded_match_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_EVIDENCE
            .contains("PortalList")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_EVIDENCE
            .contains("message highlight animation")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_EVIDENCE
            .contains("telegram_message_search_matches")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_EVIDENCE
            .contains("MatrixRequest::PaginateTimeline")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_LABEL
            .contains("scrolls/highlights")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search result jump loaded match"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("jump_telegram_message_search_active_match")
            && capability
                .base_module
                .contains("loaded_message_search_result_jump_loaded_match_label")
            && capability.notes.contains("active loaded timeline match")
            && capability.notes.contains("PortalList")
            && capability.notes.contains("message highlight animation")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("server-side history query")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("thread timeline open")
            && capability.notes.contains("sender/profile lookup")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_result_thread_open_uses_existing_read_path() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_THREAD_OPEN_MARKER,
        "hepta_telegram_message_search_result_thread_open_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_result_thread_open_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_THREAD_OPEN_EVIDENCE
            .contains("MsgLikeContent.thread_root")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_THREAD_OPEN_EVIDENCE
            .contains("SelectedRoom::Thread")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_THREAD_OPEN_EVIDENCE
            .contains("CreateThreadTimeline")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_THREAD_OPEN_EVIDENCE
            .contains("MatrixRequest::PaginateTimeline")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_THREAD_OPEN_LABEL
            .contains("existing thread timeline path")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search result thread open"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("open_telegram_message_search_active_thread")
            && capability
                .base_module
                .contains("loaded_message_search_result_thread_open_label")
            && capability.notes.contains("loaded thread root")
            && capability.notes.contains("RoomsListAction::Selected")
            && capability.notes.contains("SelectedRoom::Thread")
            && capability.notes.contains("CreateThreadTimeline")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("server-side history query")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("sender/profile lookup")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_result_sender_profile_pane_uses_existing_read_path() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_MARKER,
        "hepta_telegram_message_search_result_sender_profile_pane_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_result_sender_profile_pane_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_EVIDENCE
            .contains("UserProfileSlidingPane")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_EVIDENCE
            .contains("TimelineDetails::Ready(sender_profile)")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_EVIDENCE
            .contains("GetUserProfile/profile-member read path")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_LABEL
            .contains("existing profile pane")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search result sender profile pane"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("open_telegram_message_search_active_sender_profile")
            && capability
                .base_module
                .contains("loaded_message_search_result_sender_profile_pane_label")
            && capability.base_module.contains("UserProfileSlidingPane")
            && capability.notes.contains("UserProfilePaneInfo")
            && capability.notes.contains("sender_profile")
            && capability.notes.contains("room_members cache")
            && capability.notes.contains("GetUserProfile")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("server-side history query")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("profile mutation")
            && capability.notes.contains("direct-message start")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_result_copy_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_MARKER,
        "hepta_telegram_message_search_result_copy_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_result_copy_clipboard_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_EVIDENCE
            .contains("plaintext_body_of_timeline_item")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_EVIDENCE
            .contains("already loaded RoomScreen tl_state")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_EVIDENCE
            .contains("MatrixRequest::PaginateTimeline")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_LABEL
            .contains("loaded plaintext")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search result copy clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("stage_telegram_message_search_result_action_control")
            && capability
                .base_module
                .contains("loaded_message_search_result_copy_clipboard_label")
            && capability
                .notes
                .contains("active loaded timeline match plaintext")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("RoomScreen tl_state")
            && capability.notes.contains("plaintext char count")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("server-side history query")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("event source open")
            && capability.notes.contains("thread timeline open")
            && capability.notes.contains("sender/profile lookup")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_result_source_modal_opens_loaded_or_server_source() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_RESULT_SOURCE_MODAL_MARKER,
        "hepta_telegram_message_search_result_source_modal_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_search_result_source_modal_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE
            .contains("EventSourceModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE
            .contains("latest_json")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE
            .contains("cached Matrix /search server result")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE
            .contains("raw event JSON cached from MatrixRequest::SearchMessagesServer")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE
            .contains("TimelineUpdate::EventSourceFetched")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE
            .contains("MatrixRequest::PaginateTimeline")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_RESULT_SOURCE_MODAL_LABEL
            .contains("source-only JSON")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search result source modal"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("stage_telegram_message_search_result_action_control")
            && capability
                .base_module
                .contains("loaded_message_search_result_source_modal_label")
            && capability
                .base_module
                .contains("EventSourceModalAction::Open")
            && capability.notes.contains("active loaded timeline match")
            && capability
                .notes
                .contains("cached Matrix /search server-result source")
            && capability.notes.contains("MatrixRequest::FetchEventSource")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability.notes.contains("latest_json")
            && capability.notes.contains("JSON line count")
            && capability.notes.contains("no new Matrix-backed search")
            && capability
                .notes
                .contains("MatrixRequest::SearchMessagesServer")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("thread timeline open")
            && capability.notes.contains("sender/profile lookup")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_advanced_filter_controls_include_live_from_filter() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_MARKER,
        "hepta_telegram_message_search_advanced_filter_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_advanced_filter_controls_row")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
            .contains("visible message-search advanced filter controls")
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_LOADED_SCOPE_FILTERS_LIVE_MARKER,
        "hepta_telegram_message_search_loaded_scope_filters_live_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
            .contains("From is the live sender filter")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
            .contains("RoomEventFilter::senders")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
            .contains("RoomEventFilter::url_filter=EventsWithUrl")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
            .contains("date index query")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
            .contains("latest loaded-day timestamp window")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
            .contains("SubscribeToPinnedEvents")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
            .contains("pinned event fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_LABEL
            .contains("Media URL filters are live server reads")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_LABEL
            .contains("Filter, Date, and Pins are live loaded-scope filters")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search advanced filter controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("search_advanced_filter_controls")
            && capability
                .notes
                .contains("From is live for sender filtering")
            && capability.notes.contains("RoomEventFilter::senders")
            && capability
                .notes
                .contains("RoomEventFilter::url_filter=EventsWithUrl")
            && capability
                .notes
                .contains("Older/Retry reuse the last sender/media filter")
            && capability.notes.contains("live loaded-scope filters")
            && capability
                .notes
                .contains("latest loaded-day timestamp window")
            && capability.notes.contains("SubscribeToPinnedEvents")
            && capability.notes.contains("remote date index query")
            && capability.notes.contains("PinEvent")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search loaded scope filters live wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("MessageSearchLoadedScope")
            && capability
                .base_module
                .contains("apply_telegram_message_search_loaded_scope_control")
            && capability.notes.contains("LatestLoadedDay")
            && capability.notes.contains("PinnedLoaded")
            && capability.notes.contains("SubscribeToPinnedEvents")
            && capability
                .notes
                .contains("no Matrix SearchMessagesServer request")
            && capability.notes.contains("no PinEvent")
            && capability.notes.contains("no timeline reload")
    }));
}

#[test]
fn hepta_telegram_base_message_search_server_preflight_controls_split_live_and_local() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_MARKER,
        "hepta_telegram_message_search_server_preflight_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_server_preflight_controls_row")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
            .contains("beside the live message-search server controls")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
            .contains("Server query, Packet, Contract, Result, Error, Retry, Scope, and Taxonomy")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
            .contains("Retry resubmits the current query")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
            .contains("/_matrix/client/v3/search worker path")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
            .contains("Older owns next_batch pagination")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
            .contains("Context owns cached current-room hit pagination")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
            .contains("search scope fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL
            .contains("live Matrix search reads")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search server preflight controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("search_server_preflight_controls")
            && capability.notes.contains(
                "Server query, Packet, Contract, Result, Error, Retry, Scope, and Taxonomy",
            )
            && capability
                .notes
                .contains("Server query submits the first MatrixRequest::SearchMessagesServer page")
            && capability
                .notes
                .contains("Retry resubmits the current query")
            && capability
                .notes
                .contains("Older owns next_batch pagination")
            && capability
                .notes
                .contains("Context owns cached current-room hit pagination")
            && capability
                .notes
                .contains("timeline reload outside BackwardsPaginateUntilEvent")
            && capability.notes.contains("search scope fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_server_packet_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_MARKER,
        "hepta_telegram_message_search_server_packet_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_server_packet_clipboard_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_EVIDENCE
            .contains("local server query/result packet")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_EVIDENCE
            .contains("already loaded RoomScreen tl_state")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_EVIDENCE
            .contains("result cursor")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_EVIDENCE
            .contains("MatrixRequest::PaginateTimeline")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_LABEL
            .contains("local clipboard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search server packet clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("copy_telegram_message_search_server_packet")
            && capability
                .base_module
                .contains("message_search_server_packet_clipboard_payload")
            && capability
                .notes
                .contains("local server query/result packet")
            && capability
                .notes
                .contains("already loaded RoomScreen tl_state")
            && capability.notes.contains("result cursor")
            && capability.notes.contains("Matrix-backed search")
            && capability.notes.contains("server-side history query")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("search scope fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_matrix_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_MARKER,
        "hepta_telegram_message_search_matrix_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_matrix_contract_packet_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE
            .contains("typed Matrix search acceptance contract")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE
            .contains("request slots for room scope")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE
            .contains("result slots for event id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE
            .contains("error slots for forbidden")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE
            .contains("retry slots for confirmation")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_LABEL
            .contains("typed request/result/error/retry/scope/cursor")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search Matrix contract packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("search_server_contract_button")
            && capability
                .base_module
                .contains("message_search_matrix_contract_acceptance_label")
            && capability
                .notes
                .contains("typed Matrix search acceptance contract")
            && capability.notes.contains("query term")
            && capability.notes.contains("next_batch cursor")
            && capability.notes.contains("event id")
            && capability.notes.contains("rate-limited")
            && capability.notes.contains("idempotency")
            && capability.notes.contains("Matrix search request body")
            && capability.notes.contains("result cursor")
            && capability.notes.contains("MatrixRequest::PaginateTimeline")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_search_remote_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_message_search_remote_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_search_remote_result_taxonomy_packet_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("remote date/pins/scope/full-result taxonomy packet")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("MatrixRequest::SearchMessagesServer first page")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("remote_date_index_operation_id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("remote_pinned_fetch_operation_id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("cross_room_scope_request_id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_LABEL
            .contains("remote date/pins/scope/full-result")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message search remote result taxonomy packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("search_server_taxonomy_button")
            && capability
                .base_module
                .contains("message_search_remote_result_taxonomy_packet_label")
            && capability
                .notes
                .contains("remote date/pins/scope/full-result result taxonomy packet")
            && capability
                .notes
                .contains("MatrixRequest::SearchMessagesServer first page")
            && capability.notes.contains("next_batch Older pagination")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("remote_date_index_operation_id")
            && capability
                .notes
                .contains("remote_pinned_fetch_operation_id")
            && capability.notes.contains("cross_room_scope_request_id")
            && capability.notes.contains("full_result_cursor_id")
            && capability.notes.contains("no extra Matrix search")
            && capability.notes.contains("remote date index query")
            && capability.notes.contains("cross-room scope search")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_surface_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_LOCAL_SURFACE_MARKER,
        "hepta_telegram_notifications_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notification_mute_header"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notification_mute_surface"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_context_notifications_surface"));
    let capability = HEPTA_TELEGRAM_BASE_CAPABILITIES
        .iter()
        .find(|capability| capability.telegram_surface == "notifications")
        .expect("notifications capability remains tracked");
    assert!(capability.notes.contains("All messages"));
    assert!(capability.notes.contains("Mentions"));
    assert!(capability.notes.contains("Mute"));
    assert!(capability.notes.contains("PositiveConfirmationModal"));
    assert!(
        capability
            .notes
            .contains("MatrixRequest::GetRoomNotificationMode")
    );
    assert!(
        capability
            .notes
            .contains("MatrixRequest::SetRoomNotificationMode")
    );
    assert!(capability.notes.contains("Timed mute"));
    assert!(
        capability
            .notes
            .contains("push gateway/device configuration")
    );
}

#[test]
fn hepta_telegram_base_notifications_option_staging_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_notifications_option_staging_local_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_OPTION_STAGING_LOCAL_EVIDENCE.contains(
            "open a confirmation guard before supported All messages, Mentions, and Mute"
        )
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_OPTION_STAGING_LOCAL_EVIDENCE
            .contains("MatrixRequest::SetRoomNotificationMode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_OPTION_STAGING_LOCAL_EVIDENCE
            .contains("Timed mute choices")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_OPTION_STAGING_LOCAL_EVIDENCE
            .contains("live mutation request")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notification_option_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notification_mode_write_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications option staging local evidence"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("Header Mute")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomNotificationMode")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomNotificationMode")
            && capability.notes.contains("All messages")
            && capability.notes.contains("Mentions")
            && capability.notes.contains("Mute")
            && capability.notes.contains("Refresh")
            && capability.notes.contains("timed mute remains unwired")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation requests")
    }));
}

#[test]
fn hepta_telegram_base_notifications_mode_write_has_confirmation_guard() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_MODE_WRITE_CONFIRMATION_MARKER,
        "hepta_telegram_notifications_mode_write_confirmation_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_WRITE_CONFIRMATION_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_WRITE_CONFIRMATION_EVIDENCE
            .contains("MatrixRequest::SetRoomNotificationMode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_WRITE_CONFIRMATION_EVIDENCE
            .contains("NotificationSettings::set_room_notification_mode")
    );
    assert!(
        crate::home::room_context_menu::ROOM_CONTEXT_NOTIFICATION_MODE_COMPACT_LABEL
            .contains("confirmation")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications mode write confirmation"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("PositiveConfirmationModal")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomNotificationMode")
            && capability
                .notes
                .contains("TimelineUpdate::RoomNotificationModeSet")
            && capability.notes.contains("context menu writes")
            && capability.notes.contains("timed mute")
    }));
}

#[test]
fn hepta_telegram_base_notifications_loaded_attention_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_LOADED_ATTENTION_MARKER,
        "hepta_telegram_notifications_loaded_attention_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_LOADED_ATTENTION_EVIDENCE
            .contains("RoomsList unread count")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_LOADED_ATTENTION_EVIDENCE.contains("mention count")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_LOADED_ATTENTION_EVIDENCE
            .contains("manual unread state")
    );
    assert!(
        crate::home::room_context_menu::ROOM_CONTEXT_NOTIFICATION_LOADED_ATTENTION_EVIDENCE
            .contains("loaded RoomsList unread count")
    );
    assert!(
        crate::home::room_context_menu::ROOM_CONTEXT_NOTIFICATION_LOADED_ATTENTION_EVIDENCE
            .contains("push gateway/device")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_loaded_attention_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications loaded attention preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("RoomContextMenuDetails")
            && capability.notes.contains("unread count")
            && capability.notes.contains("mention count")
            && capability.notes.contains("manual unread state")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomNotificationMode")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_notifications_mode_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_MODE_CLIPBOARD_MARKER,
        "hepta_telegram_notifications_mode_clipboard_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_CLIPBOARD_EVIDENCE
            .contains("already loaded current room notification mode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_CLIPBOARD_EVIDENCE.contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_CLIPBOARD_EVIDENCE
            .contains("Missing notification mode stays local-unavailable")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_CLIPBOARD_EVIDENCE
            .contains("GetRoomNotificationMode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_CLIPBOARD_EVIDENCE
            .contains("SetRoomNotificationMode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_CLIPBOARD_EVIDENCE
            .contains("push gateway/device")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_CLIPBOARD_LABEL.contains("local clipboard")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_mode_clipboard_action"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications mode clipboard"
            && capability.base_module.contains("Copy mode control")
            && capability
                .notes
                .contains("already loaded current room notification mode")
            && capability
                .notes
                .contains("loaded RoomsList unread/mention/manual-unread")
            && capability.notes.contains("local clipboard")
            && capability
                .notes
                .contains("Missing mode stays local-unavailable")
            && capability.notes.contains("GetRoomNotificationMode")
            && capability.notes.contains("SetRoomNotificationMode")
            && capability.notes.contains("push gateway/device")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability
                .notes
                .contains("notifications remains a base gap")
    }));
}

#[test]
fn hepta_telegram_base_notifications_mode_target_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_MODE_TARGET_METADATA_MARKER,
        "hepta_telegram_notifications_mode_target_metadata_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_TARGET_METADATA_EVIDENCE
            .contains("current loaded room notification mode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_TARGET_METADATA_EVIDENCE
            .contains("requested All/Mentions/Mute mode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_TARGET_METADATA_EVIDENCE
            .contains("loaded RoomsList attention availability")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_TARGET_METADATA_EVIDENCE
            .contains("retry cache availability")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_TARGET_METADATA_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_TARGET_METADATA_EVIDENCE
            .contains("push gateway/device")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_MODE_TARGET_METADATA_LABEL
            .contains("SetRoomNotificationMode waits for confirmation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_mode_target_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications mode target metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("telegram_notifications_strip")
            && capability
                .notes
                .contains("current loaded room notification mode")
            && capability
                .notes
                .contains("requested All/Mentions/Mute mode")
            && capability
                .notes
                .contains("loaded RoomsList attention availability")
            && capability.notes.contains("retry cache availability")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomNotificationMode unless PositiveConfirmationModal")
            && capability.notes.contains("global notification preference")
            && capability.notes.contains("push gateway/device")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_close_refresh_metadata_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_CLOSE_REFRESH_METADATA_MARKER,
        "hepta_telegram_notifications_close_refresh_metadata_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_CLOSE_REFRESH_METADATA_EVIDENCE
            .contains("current local notification status")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_CLOSE_REFRESH_METADATA_EVIDENCE
            .contains("MatrixRequest::GetRoomNotificationMode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_CLOSE_REFRESH_METADATA_EVIDENCE
            .contains("Close only hides the local strip")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_CLOSE_REFRESH_METADATA_LABEL
            .contains("no notification mode write")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_close_refresh_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications close refresh metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("local notification status")
            && capability.notes.contains("loaded notification mode state")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomNotificationMode")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomNotificationMode")
            && capability.notes.contains("push gateway/device")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_notifications_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_notifications_local_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_LOCAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::GetRoomNotificationMode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_LOCAL_BOUNDARY_EVIDENCE
            .contains("confirmed current-room Matrix notification mode writes")
    );
    assert!(crate::home::room_screen::NOTIFICATIONS_LOCAL_BOUNDARY_EVIDENCE.contains("timed mute"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_LOCAL_BOUNDARY_EVIDENCE
            .contains("live default room-mode reads")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_LOCAL_BOUNDARY_EVIDENCE
            .contains("confirmed default room-mode writes")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_LOCAL_BOUNDARY_EVIDENCE
            .contains("push gateway mutation")
    );
    assert!(crate::home::room_screen::NOTIFICATIONS_LOCAL_BOUNDARY_LABEL.contains("confirmation"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications local boundary evidence"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("telegram_notifications_strip")
            && capability
                .notes
                .contains("confirmed All/Mentions/Mute mode writes")
            && capability.notes.contains("Timed mute")
            && capability.notes.contains("live default room-mode reads")
            && capability
                .notes
                .contains("confirmed default room-mode writes")
            && capability
                .notes
                .contains("global notification preference writes")
            && capability
                .notes
                .contains("push gateway/device configuration")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_timed_global_boundary_is_local_metadata() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_MARKER,
        "hepta_telegram_notifications_timed_global_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE
            .contains("timed mute durations")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE
            .contains("global notification preference writes")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::GetDefaultRoomNotificationMode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE
            .contains("keyword rule")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE
            .contains("push gateway/device or pusher configuration")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE
            .contains("current loaded room notification mode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE
            .contains("loaded RoomsList unread/mention/manual-unread")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_EVIDENCE
            .contains("unless All messages, Mentions, or Mute is explicitly confirmed")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_LABEL
            .contains("boundary metadata")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_timed_global_boundary_metadata")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications timed global boundary metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("telegram_notifications_strip")
            && capability
                .notes
                .contains("loaded current room notification mode")
            && capability
                .notes
                .contains("loaded RoomsList unread/mention/manual-unread")
            && capability.notes.contains("Timed mute durations")
            && capability
                .notes
                .contains("global notification preference writes")
            && capability
                .notes
                .contains("MatrixRequest::GetDefaultRoomNotificationMode")
            && capability
                .notes
                .contains("MatrixRequest::SetNotificationKeywordRule")
            && capability.notes.contains("pusher set/delete mutations")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomNotificationMode")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationKeywordRules")
            && capability
                .notes
                .contains("NotificationSettings::add_keyword/remove_keyword")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationPusherStatus")
            && capability
                .notes
                .contains("unrelated live mutation remain unwired")
    }));
}

#[test]
fn hepta_telegram_base_notifications_pusher_keyword_boundary_splits_live_read_from_writes() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_MARKER,
        "hepta_telegram_notifications_pusher_keyword_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("Keyword rules")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("Global and Defaults")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("NotificationSettings::get_default_room_notification_mode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("Timed mute duration presets")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("Push gateway/device setup")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("Pusher enable/disable")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SetNotificationKeywordRule")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("NotificationSettings::add_keyword")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::GetNotificationPusherStatus")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("Sound/badge tuning")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_EVIDENCE
            .contains("Matrix notification rule account-data edits")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_LABEL
            .contains("pusher status read live Matrix settings")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_pusher_keyword_boundary")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications pusher keyword boundary"
            && capability
                .base_module
                .contains("notifications_pusher_keyword_boundary")
            && capability
                .notes
                .contains("current loaded room notification mode")
            && capability
                .notes
                .contains("loaded RoomsList unread/mention/manual-unread")
            && capability.notes.contains("retry cache readiness")
            && capability.notes.contains("Keyword rules")
            && capability.notes.contains("Keyword list")
            && capability.notes.contains("Add keyword")
            && capability.notes.contains("Remove keyword")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationKeywordRules")
            && capability
                .notes
                .contains("MatrixRequest::SetNotificationKeywordRule")
            && capability.notes.contains("NotificationSettings::")
            && capability
                .notes
                .contains("TimelineUpdate::NotificationKeywordRulesMutated")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationPusherStatus")
            && capability
                .notes
                .contains("Client::can_homeserver_push_encrypted_event_to_device")
            && capability.notes.contains("Global and Defaults")
            && capability
                .notes
                .contains("MatrixRequest::GetDefaultRoomNotificationMode")
            && capability
                .notes
                .contains("TimelineUpdate::NotificationDefaultRoomModeFetched")
            && capability.notes.contains("Timed mute duration presets")
            && capability.notes.contains("Push gateway/device setup")
            && capability.notes.contains("Pusher enable/disable")
            && capability.notes.contains("Sound/badge tuning")
            && capability
                .notes
                .contains("Matrix notification rule account-data edits")
            && capability.notes.contains("pusher mutations")
            && capability.notes.contains("confirmed keyword Add/Remove")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability
                .notes
                .contains("notifications remains a base gap")
    }));
}

#[test]
fn hepta_telegram_base_notifications_keyword_list_live_read_is_sdk_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_MARKER,
        "hepta_telegram_notifications_keyword_list_live_read_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_EVIDENCE
            .contains("MatrixRequest::GetNotificationKeywordRules")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_EVIDENCE
            .contains("NotificationSettings::")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_EVIDENCE
            .contains("TimelineUpdate::NotificationKeywordRulesFetched")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_EVIDENCE
            .contains("no unconfirmed add/remove keyword rule write")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_keyword_list_live_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications keyword list live read"
            && capability
                .base_module
                .contains("MatrixRequest::GetNotificationKeywordRules")
            && capability.notes.contains("NotificationSettings::")
            && capability
                .notes
                .contains("TimelineUpdate::NotificationKeywordRulesFetched")
            && capability.notes.contains("read path")
            && capability
                .notes
                .contains("no unconfirmed keyword add/remove write")
            && capability.notes.contains("pusher mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_keyword_mutation_is_confirmed_live_write() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_KEYWORD_MUTATION_LIVE_WRITE_MARKER,
        "hepta_telegram_notifications_keyword_mutation_live_write_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_KEYWORD_MUTATION_EVIDENCE
            .contains("MatrixRequest::SetNotificationKeywordRule")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_KEYWORD_MUTATION_EVIDENCE
            .contains("NotificationSettings::add_keyword")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_KEYWORD_MUTATION_EVIDENCE
            .contains("TimelineUpdate::NotificationKeywordRulesMutated")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_keyword_mutation_live_write")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications keyword mutation live write"
            && capability
                .base_module
                .contains("MatrixRequest::SetNotificationKeywordRule")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability
                .notes
                .contains("NotificationSettings::add_keyword")
            && capability
                .notes
                .contains("TimelineUpdate::NotificationKeywordRulesMutated")
            && capability.notes.contains("Retry")
            && capability.notes.contains("no unconfirmed keyword write")
            && capability.notes.contains("gateway/runtime/auth")
    }));
}

#[test]
fn hepta_telegram_base_notifications_pusher_status_live_read_is_sdk_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_MARKER,
        "hepta_telegram_notifications_pusher_status_live_read_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_EVIDENCE
            .contains("MatrixRequest::GetNotificationPusherStatus")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_EVIDENCE
            .contains("Client::can_homeserver_push_encrypted_event_to_device")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_EVIDENCE
            .contains("TimelineUpdate::NotificationPusherStatusFetched")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_EVIDENCE
            .contains("no pusher set/delete mutation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_pusher_status_live_read")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications pusher status live read"
            && capability
                .base_module
                .contains("MatrixRequest::GetNotificationPusherStatus")
            && capability
                .notes
                .contains("Client::can_homeserver_push_encrypted_event_to_device")
            && capability
                .notes
                .contains("TimelineUpdate::NotificationPusherStatusFetched")
            && capability.notes.contains("read-only")
            && capability.notes.contains("no pusher set/delete mutation")
            && capability
                .notes
                .contains("push gateway/device configuration write")
            && capability.notes.contains("account-data mutation")
            && capability.notes.contains("push-rule write")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_advanced_controls_include_keyword_live_read() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_ADVANCED_CONTROLS_MARKER,
        "hepta_telegram_notifications_advanced_controls_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE
            .contains("Timed, Keywords, Pusher, and Global")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE
            .contains("live read-only Matrix SDK handoff")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE
            .contains("keyword input row")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE
            .contains("current loaded room notification mode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE
            .contains("default preference writes")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE
            .contains("confirmed keyword Add/Remove")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_CONTROLS_LABEL
            .contains("Keywords and Global defaults read live")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_advanced_controls_row"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications advanced controls row"
            && capability
                .base_module
                .contains("notification_advanced_controls")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationKeywordRules")
            && capability
                .notes
                .contains("MatrixRequest::GetDefaultRoomNotificationMode")
            && capability
                .notes
                .contains("MatrixRequest::SetNotificationKeywordRule")
            && capability
                .notes
                .contains("Timed, Keywords, Pusher, and Global")
            && capability.notes.contains("keyword input row")
            && capability
                .notes
                .contains("current loaded room notification mode")
            && capability
                .notes
                .contains("loaded RoomsList unread/mention/manual-unread")
            && capability
                .notes
                .contains("Matrix notification rule account-data edits")
            && capability.notes.contains("pusher mutations")
            && capability.notes.contains("confirmed keyword Add/Remove")
            && capability.notes.contains("default preference writes")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability
                .notes
                .contains("confirmed All/Mentions/Mute SetRoomNotificationMode")
            && capability.notes.contains("confirmed keyword Add/Remove")
    }));
}

#[test]
fn hepta_telegram_base_notifications_advanced_detail_controls_include_keyword_live_read() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_notifications_advanced_detail_controls_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_EVIDENCE
            .contains("Quiet hours, Keyword list, Device push, Defaults, and Sound badge")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_EVIDENCE
            .contains("live read-only Matrix SDK handoff")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_EVIDENCE
            .contains("retry cache readiness")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_EVIDENCE
            .contains("Matrix notification rule account-data edits")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_EVIDENCE
            .contains("MatrixRequest::SetRoomNotificationMode outside")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_LABEL
            .contains("Keyword list, Device push, and Defaults read live")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_advanced_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications advanced detail controls row"
            && capability
                .base_module
                .contains("notification_advanced_detail_controls")
            && capability.notes.contains("Quiet hours")
            && capability.notes.contains("Keyword list")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationKeywordRules")
            && capability.notes.contains("Device push")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationPusherStatus")
            && capability.notes.contains("Defaults")
            && capability
                .notes
                .contains("MatrixRequest::GetDefaultRoomNotificationMode")
            && capability.notes.contains("Sound badge")
            && capability
                .notes
                .contains("current loaded room notification mode")
            && capability.notes.contains("retry cache readiness")
            && capability
                .notes
                .contains("Matrix notification rule account-data edits")
            && capability.notes.contains("push-rule writes")
            && capability.notes.contains("pusher mutations")
            && capability.notes.contains("sound/badge tuning")
            && capability.notes.contains("retry automation")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomNotificationMode outside")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_result_detail_controls_are_local_status_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RESULT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_notifications_result_detail_controls_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE
            .contains("Result, Requested, Retry cache, Failure, and Source")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE
            .contains("local notification result detail metadata")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE
            .contains("retry cache readiness")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE
            .contains("MatrixRequest::GetRoomNotificationMode outside")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE
            .contains("MatrixRequest::SetRoomNotificationMode outside")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_DETAIL_CONTROLS_LABEL
            .contains("stay local result detail controls")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_result_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications result detail controls row"
            && capability
                .base_module
                .contains("notification_result_detail_controls")
            && capability.notes.contains("Result")
            && capability.notes.contains("Requested")
            && capability.notes.contains("Retry cache")
            && capability.notes.contains("Failure")
            && capability.notes.contains("Source")
            && capability
                .notes
                .contains("current loaded room notification mode")
            && capability.notes.contains("retry cache readiness")
            && capability.notes.contains("local status text")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomNotificationMode outside")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomNotificationMode outside")
            && capability.notes.contains("push-rule writes")
            && capability.notes.contains("pusher mutations")
            && capability.notes.contains("cancel queue")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_preflight_detail_controls_include_keyword_live_read() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_notifications_preflight_detail_controls_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Schedule, Packet, Contract, Account data, Keywords, Pushers, and Defaults")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("typed account-data/push-rule/pusher/result contract packet")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("live read-only Matrix SDK handoff")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Defaults is a live read-only Matrix SDK handoff")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("local notification schedule packet snapshot")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("no Matrix notification rule account-data write")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("MatrixRequest::GetRoomNotificationMode outside")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("Keywords, Pushers, and Defaults read live")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_preflight_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications timed global pusher preflight controls row"
            && capability
                .base_module
                .contains("notification_preflight_detail_controls")
            && capability.notes.contains("Schedule")
            && capability.notes.contains("Account data")
            && capability.notes.contains("Keywords")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationKeywordRules")
            && capability.notes.contains("Pushers")
            && capability
                .notes
                .contains("MatrixRequest::GetNotificationPusherStatus")
            && capability.notes.contains("Defaults")
            && capability
                .notes
                .contains("MatrixRequest::GetDefaultRoomNotificationMode")
            && capability
                .notes
                .contains("local notification schedule packet snapshot")
            && capability.notes.contains("Contract")
            && capability
                .notes
                .contains("typed account-data/push-rule/pusher/result contract")
            && capability
                .notes
                .contains("Matrix notification rule account-data write")
            && capability.notes.contains("push-rule write")
            && capability.notes.contains("pusher mutation")
            && capability
                .notes
                .contains("push gateway/device configuration")
            && capability.notes.contains("timed mute write")
            && capability.notes.contains("default preference write")
            && capability
                .notes
                .contains("MatrixRequest::GetRoomNotificationMode outside")
            && capability
                .notes
                .contains("MatrixRequest::SetRoomNotificationMode outside")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_rule_packet_drilldown_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RULE_PACKET_DRILLDOWN_MARKER,
        "hepta_telegram_notifications_rule_packet_drilldown_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_PACKET_DRILLDOWN_EVIDENCE
            .contains("local notification rule packet")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_PACKET_DRILLDOWN_EVIDENCE
            .contains("request/result/error/retry acceptance criteria")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_PACKET_DRILLDOWN_EVIDENCE
            .contains("typed account-data, push-rule, pusher")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_PACKET_DRILLDOWN_EVIDENCE
            .contains("no Matrix notification rule account-data read or write")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_PACKET_DRILLDOWN_LABEL
            .contains("local rule/result/retry acceptance criteria")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_rule_packet_drilldown_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications rule packet drilldown"
            && capability
                .base_module
                .contains("copy_telegram_notifications_rule_packet")
            && capability
                .base_module
                .contains("notifications_rule_packet_payload")
            && capability
                .notes
                .contains("current loaded room notification mode")
            && capability
                .notes
                .contains("loaded RoomsList unread/mention/manual-unread")
            && capability
                .notes
                .contains("request/result/error/retry acceptance criteria")
            && capability.notes.contains("timed mute")
            && capability.notes.contains("global preferences")
            && capability.notes.contains("keyword rules")
            && capability.notes.contains("pusher/device config")
            && capability.notes.contains("sound/badge tuning")
            && capability
                .notes
                .contains("Matrix notification rule account-data read or write")
            && capability.notes.contains("push-rule write")
            && capability.notes.contains("pusher mutation")
            && capability
                .notes
                .contains("unconfirmed SetRoomNotificationMode")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_notifications_rule_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RULE_CONTRACT_PACKET_MARKER,
        "hepta_telegram_notifications_rule_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_CONTRACT_PACKET_EVIDENCE
            .contains("typed notification account-data/pusher contract packet")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_CONTRACT_PACKET_EVIDENCE
            .contains("typed request/result/error/retry/source slots")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_CONTRACT_PACKET_EVIDENCE
            .contains("push-rule keyword rules")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_CONTRACT_PACKET_EVIDENCE
            .contains("pusher/device configuration")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_CONTRACT_PACKET_EVIDENCE
            .contains("result reconciliation")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_CONTRACT_PACKET_EVIDENCE
            .contains("no Matrix notification rule account-data read or write")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RULE_CONTRACT_PACKET_LABEL
            .contains("typed account-data/push-rule/pusher/result")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_rule_contract_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications rule contract packet"
            && capability
                .base_module
                .contains("copy_telegram_notifications_rule_contract_packet")
            && capability
                .base_module
                .contains("notifications_rule_contract_packet_payload")
            && capability.notes.contains("Contract")
            && capability
                .notes
                .contains("current loaded room notification mode")
            && capability
                .notes
                .contains("typed request/result/error/retry/source slots")
            && capability.notes.contains("account-data rules")
            && capability.notes.contains("push-rule keywords")
            && capability.notes.contains("pusher/device configuration")
            && capability.notes.contains("result reconciliation")
            && capability
                .notes
                .contains("Matrix notification rule account-data read or write")
            && capability.notes.contains("push-rule write")
            && capability.notes.contains("pusher mutation")
            && capability
                .notes
                .contains("unconfirmed SetRoomNotificationMode")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "notifications"
            && runway
                .current_path
                .contains("typed account-data/pusher contract packet")
            && runway.remaining_gap.contains("timed mute")
            && runway
                .next_ui_safe_step
                .contains("backend notification account-data")
    }));
}

#[test]
fn hepta_telegram_base_notifications_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_notifications_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("local notification timed/global/pusher result taxonomy packet")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("operation_id_slot not_assigned")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("timed mute scheduled/applied/expired/failed/stale")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("raw account-data applied/failed/stale")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("pusher enabled/disabled/failed/stale")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("no Matrix notification rule account-data read or write")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RESULT_TAXONOMY_PACKET_LABEL
            .contains("timed/global/pusher result slots")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"notifications_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications result taxonomy packet"
            && capability
                .base_module
                .contains("copy_telegram_notifications_result_taxonomy_packet")
            && capability
                .base_module
                .contains("notifications_result_taxonomy_packet_payload")
            && capability.notes.contains("operation_id_slot not_assigned")
            && capability.notes.contains("timed mute")
            && capability.notes.contains("raw account-data")
            && capability.notes.contains("pusher/device")
            && capability.notes.contains("sound/badge")
            && capability
                .notes
                .contains("Matrix notification rule account-data read or write")
            && capability.notes.contains("pusher mutation")
            && capability.notes.contains("timed mute write")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "notifications"
            && runway.current_path.contains("notification rule packet")
            && runway.remaining_gap.contains("timed mute")
            && runway
                .next_ui_safe_step
                .contains("backend notification account-data")
    }));
}

#[test]
fn hepta_telegram_base_notifications_retry_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_NOTIFICATIONS_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_notifications_retry_confirmation_ready"
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RETRY_CONFIRMATION_EVIDENCE
            .contains("cached room id")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RETRY_CONFIRMATION_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RETRY_CONFIRMATION_EVIDENCE
            .contains("SetRoomNotificationMode")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RETRY_CONFIRMATION_EVIDENCE
            .contains("push gateway/device")
    );
    assert!(
        crate::home::room_screen::NOTIFICATIONS_RETRY_CONFIRMATION_LABEL
            .contains("SetRoomNotificationMode")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"notifications_retry_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "notifications retry confirmation"
            && capability
                .base_module
                .contains("MatrixRequest::SetRoomNotificationMode")
            && capability
                .notes
                .contains("TimelineUpdate::RoomNotificationModeSet")
            && capability.notes.contains("cached the room id")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("push gateway/device")
            && capability.notes.contains("cancel queue")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_room_local_surface_close_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_LOCAL_SURFACE_CLOSE_MARKER,
        "hepta_telegram_room_local_surface_close_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_search"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"room_settings"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"notifications"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_local_surface_close_evidence"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room header actions"
            && capability.notes.contains("Close/Escape on Search")
            && capability
                .notes
                .contains("Info, Room actions, Settings, and Notifications")
            && capability
                .notes
                .contains("without Matrix search or extra room-state/notification writes")
    }));
}

#[test]
fn hepta_telegram_base_room_actions_close_evidence_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOM_ACTIONS_CLOSE_LOCAL_EVIDENCE_MARKER,
        "hepta_telegram_room_actions_close_local_evidence_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"room_actions_close_local_evidence"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "room actions close local evidence"
            && capability.base_module == "RoomScreen telegram_room_actions_strip"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .notes
                .contains("Close only dismisses the local action preview")
            && capability.notes.contains(
                "no Matrix search, room-state, notification, message, or membership request",
            )
            && capability.notes.contains("existing guarded base paths")
    }));
}
